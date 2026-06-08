use proc_macro2::{Ident, TokenStream};
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::__private::quote::{format_ident, quote};

const ELEMENT_DIR: &str = "SVG/developer.mozilla.org/en-US/docs/Web/SVG/Element";
const ATTRIBUTE_DIR: &str = "SVG/developer.mozilla.org/en-US/docs/Web/SVG/Attribute";

#[derive(Debug, Deserialize)]
struct Hydration {
    doc: Document,
}

#[derive(Debug, Deserialize)]
struct Document {
    body: Vec<serde_json::Value>,
    #[serde(default, rename = "browserCompat")]
    browser_compat: Vec<String>,
    #[serde(default)]
    summary: String,
    title: String,
}

#[derive(Debug, Default)]
struct AttributeInfo {
    syntax: String,
    summary: String,
    element_docs: BTreeMap<String, String>,
    global: bool,
    elements: BTreeSet<String>,
}

#[derive(Debug, Default)]
struct AttributeUse {
    syntax: String,
    docs: String,
}

#[derive(Debug, Default)]
struct ElementInfo {
    name: String,
    summary: String,
    attributes: BTreeMap<String, AttributeUse>,
    categories: BTreeSet<String>,
    child_categories: BTreeSet<String>,
    child_elements: BTreeSet<String>,
    permits_text: bool,
}

#[derive(Debug)]
struct PathCommand {
    command: String,
    parameters: Vec<String>,
    docs: String,
}

#[derive(Debug)]
struct TransformFunction {
    name: String,
    required: Vec<String>,
    optional: Vec<String>,
    docs: String,
}

fn main() {
    println!("cargo:rerun-if-changed={ELEMENT_DIR}");
    println!("cargo:rerun-if-changed={ATTRIBUTE_DIR}");

    let element_paths = html_files(ELEMENT_DIR);
    let element_names: BTreeSet<String> = element_paths
        .iter()
        .filter_map(|path| path.file_stem()?.to_str().map(ToOwned::to_owned))
        .collect();
    let attributes = parse_attribute_docs(&element_names);
    let mut elements = BTreeMap::new();

    for path in element_paths {
        let mut element = parse_element_doc(&path, &attributes);
        for (attribute_name, attribute) in &attributes {
            if element.attributes.contains_key(attribute_name)
                || attribute.global
                || attribute.elements.contains(&element.name)
            {
                let docs = attribute
                    .element_docs
                    .get(&element.name)
                    .unwrap_or(&attribute.summary)
                    .clone();
                let docs = with_mdn_attribute_reference(&docs, attribute_name);
                element
                    .attributes
                    .entry(attribute_name.clone())
                    .and_modify(|usage| {
                        usage.docs.clone_from(&docs);
                        if usage.syntax.is_empty() {
                            usage.syntax.clone_from(&attribute.syntax);
                        }
                    })
                    .or_insert_with(|| AttributeUse {
                        syntax: attribute.syntax.clone(),
                        docs,
                    });
            }
        }
        elements.insert(element.name.clone(), element);
    }

    let preserve_aspect_ratio = parse_preserve_aspect_ratio();
    let path_commands = parse_path_commands();
    let transform_functions = parse_transform_functions();
    let generated = generate(
        &elements,
        &preserve_aspect_ratio,
        &path_commands,
        &transform_functions,
    );
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    fs::write(
        out.join("generated.rs"),
        format_rust_code(&generated.to_string()),
    )
    .expect("write generated Rust");

    println!(
        "cargo:warning=generated {} SVG elements, {} documented attributes, {} path commands, and {} transform functions from MDN",
        elements.len(),
        attributes.len(),
        path_commands.len(),
        transform_functions.len(),
    );
}

fn parse_preserve_aspect_ratio() -> Vec<String> {
    let path = Path::new(ATTRIBUTE_DIR).join("preserveAspectRatio.html");
    let hydration = hydration(&path);
    let content = body_content(&hydration.doc, "syntax")
        .unwrap_or_else(|| panic!("no preserveAspectRatio syntax in {}", path.display()));
    let fragment = Html::parse_fragment(content);
    let dt = selector("dt code");
    fragment
        .select(&dt)
        .map(text)
        .filter(|value| value != "meet" && value != "slice")
        .collect()
}

fn parse_path_commands() -> Vec<PathCommand> {
    let path = Path::new(ATTRIBUTE_DIR).join("d.html");
    let hydration = hydration(&path);
    let row = selector("tr");
    let heading = selector("th");
    let cell = selector("td");
    let variable = selector("var");
    let code = selector("code");
    let mut commands = BTreeMap::new();

    for body in &hydration.doc.body {
        let Some(value) = body.get("value") else {
            continue;
        };
        let Some(id) = value.get("id").and_then(serde_json::Value::as_str) else {
            continue;
        };
        if id == "path_commands" {
            continue;
        }
        let Some(content) = value.get("content").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let docs = with_mdn_section_reference(
            &first_paragraph_docs(content),
            &format!("Attribute/d#{id}"),
        );
        let fragment = Html::parse_fragment(content);
        for row in fragment.select(&row) {
            let command_heading = row.select(&heading).next().map(text).unwrap_or_default();
            let command_names = command_heading
                .split(|character: char| character == ',' || character.is_whitespace())
                .filter(|command| {
                    command.len() == 1
                        && command
                            .chars()
                            .all(|character| character.is_ascii_alphabetic())
                })
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if command_names.is_empty() {
                continue;
            }

            let parameters = row
                .select(&cell)
                .next()
                .map(|parameters_cell| {
                    let mut parameters = parameters_cell
                        .select(&variable)
                        .map(text)
                        .filter(|parameter| !parameter.is_empty())
                        .collect::<Vec<_>>();
                    if parameters.is_empty() {
                        parameters = parameters_cell
                            .select(&code)
                            .map(text)
                            .filter(|parameter| {
                                !parameter.is_empty()
                                    && parameter.chars().all(|character| {
                                        character.is_ascii_alphanumeric() || character == '-'
                                    })
                            })
                            .collect();
                    }
                    parameters.dedup();
                    parameters
                })
                .unwrap_or_default();

            for command in command_names {
                commands.entry(command.clone()).or_insert(PathCommand {
                    command,
                    parameters: parameters.clone(),
                    docs: docs.clone(),
                });
            }
        }
    }
    commands.into_values().collect()
}

fn parse_transform_functions() -> Vec<TransformFunction> {
    let path = Path::new(ATTRIBUTE_DIR).join("transform.html");
    let hydration = hydration(&path);
    let code = selector("p code");
    let mut functions = Vec::new();

    for body in &hydration.doc.body {
        let Some(value) = body.get("value") else {
            continue;
        };
        let Some(id) = value.get("id").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let Some(content) = value.get("content").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let docs = with_mdn_section_reference(
            &first_paragraph_docs(content),
            &format!("Attribute/transform#{id}"),
        );
        let fragment = Html::parse_fragment(content);
        let Some(signature) = fragment.select(&code).next().map(text) else {
            continue;
        };
        let Some((name, parameters)) = signature.split_once('(') else {
            continue;
        };
        let Some(parameters) = parameters.strip_suffix(')') else {
            continue;
        };
        if !name.eq_ignore_ascii_case(id) {
            continue;
        }

        let mut required = Vec::new();
        let mut optional = Vec::new();
        let mut optional_depth = 0_usize;
        let mut remaining = parameters;
        while let Some(start) = remaining.find(['[', ']', '<']) {
            remaining = &remaining[start..];
            match remaining.as_bytes()[0] {
                b'[' => {
                    optional_depth += 1;
                    remaining = &remaining[1..];
                }
                b']' => {
                    optional_depth = optional_depth.saturating_sub(1);
                    remaining = &remaining[1..];
                }
                b'<' => {
                    let Some(end) = remaining.find('>') else {
                        break;
                    };
                    let parameter = remaining[1..end].to_owned();
                    if optional_depth == 0 {
                        required.push(parameter);
                    } else {
                        optional.push(parameter);
                    }
                    remaining = &remaining[end + 1..];
                }
                _ => unreachable!(),
            }
        }
        functions.push(TransformFunction {
            name: name.to_owned(),
            required,
            optional,
            docs,
        });
    }
    functions
}

fn html_files(directory: &str) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {directory}: {error}"))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "html")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn hydration(path: &Path) -> Hydration {
    let html = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let marker = r#"<script type="application/json" id="hydration">"#;
    let start = html
        .find(marker)
        .unwrap_or_else(|| panic!("no MDN hydration data in {}", path.display()))
        + marker.len();
    let end = html[start..]
        .find("</script>")
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("unterminated hydration data in {}", path.display()));
    serde_json::from_str(&html[start..end])
        .unwrap_or_else(|error| panic!("invalid hydration data in {}: {error}", path.display()))
}

fn body_content<'a>(doc: &'a Document, id: &str) -> Option<&'a str> {
    doc.body.iter().find_map(|body| {
        let value = body.get("value")?;
        if value.get("id")?.as_str()? == id {
            value.get("content")?.as_str()
        } else {
            None
        }
    })
}

fn all_content(doc: &Document) -> String {
    doc.body
        .iter()
        .filter_map(|body| body.get("value")?.get("content")?.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_attribute_docs(element_names: &BTreeSet<String>) -> BTreeMap<String, AttributeInfo> {
    let element_ids = element_names
        .iter()
        .map(|name| (name.to_ascii_lowercase(), name.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut attributes = BTreeMap::new();
    for path in html_files(ATTRIBUTE_DIR) {
        let hydration = hydration(&path);
        let name = attribute_name(&hydration.doc.title, &path);
        if name.is_empty() || name.contains('*') {
            continue;
        }
        let content = all_content(&hydration.doc);
        let mut info = AttributeInfo {
            syntax: extract_value_syntax(&content),
            summary: normalize_doc_text(&hydration.doc.summary),
            element_docs: BTreeMap::new(),
            global: content.contains("with any SVG element"),
            elements: BTreeSet::new(),
        };
        for body in &hydration.doc.body {
            let Some(value) = body.get("value") else {
                continue;
            };
            let Some(id) = value.get("id").and_then(serde_json::Value::as_str) else {
                continue;
            };
            let Some(element) = element_ids.get(id) else {
                continue;
            };
            let Some(content) = value.get("content").and_then(serde_json::Value::as_str) else {
                continue;
            };
            let docs = attribute_section_docs(content);
            if !docs.is_empty() {
                info.element_docs.insert(element.clone(), docs);
            }
        }
        for key in &hydration.doc.browser_compat {
            if key.starts_with("svg.global_attributes.") {
                info.global = true;
            }
            if let Some(rest) = key.strip_prefix("svg.elements.")
                && let Some((element, _)) = rest.split_once('.')
                && element_names.contains(element)
            {
                info.elements.insert(element.to_owned());
            }
        }
        if info.syntax.is_empty() {
            info.syntax = name.clone();
        }
        attributes.insert(name, info);
    }
    attributes
}

fn attribute_name(title: &str, path: &Path) -> String {
    let title = title
        .strip_prefix("SVG attribute: ")
        .unwrap_or(title)
        .trim();
    if title.contains(' ') || title.contains('<') {
        path.file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .replace('_', ":")
    } else {
        title.to_owned()
    }
}

fn extract_value_syntax(content: &str) -> String {
    let fragment = Html::parse_fragment(content);
    let row = selector("tr");
    let th = selector("th");
    let td = selector("td");
    let mut values = BTreeSet::new();
    for row in fragment.select(&row) {
        let heading = row
            .select(&th)
            .next()
            .map(text)
            .unwrap_or_default()
            .to_ascii_lowercase();
        if heading == "value"
            && let Some(value) = row.select(&td).next().map(text)
        {
            values.insert(value);
        }
    }
    values.into_iter().collect::<Vec<_>>().join(" | ")
}

fn attribute_section_docs(content: &str) -> String {
    let fragment = Html::parse_fragment(content);
    let paragraph = selector("p");
    let row = selector("tr");
    let th = selector("th");
    let td = selector("td");
    let mut lines = Vec::new();

    if let Some(description) = fragment.select(&paragraph).next().map(text) {
        let description = normalize_doc_text(&description);
        if !description.is_empty() {
            lines.push(description);
        }
    }

    let mut properties = Vec::new();
    for row in fragment.select(&row) {
        let heading = row.select(&th).next().map(text).unwrap_or_default();
        if !matches!(heading.as_str(), "Value" | "Default value" | "Animatable") {
            continue;
        }
        let Some(value) = row.select(&td).next().map(text) else {
            continue;
        };
        let value = normalize_doc_text(&value);
        if !value.is_empty() {
            properties.push(format!("{heading}: {value}"));
        }
    }

    if !properties.is_empty() {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(properties);
    }

    lines.join("\n")
}

fn first_paragraph_docs(content: &str) -> String {
    let fragment = Html::parse_fragment(content);
    fragment
        .select(&selector("p"))
        .next()
        .map(text)
        .map(|value| normalize_doc_text(&value))
        .unwrap_or_default()
}

fn normalize_doc_text(value: &str) -> String {
    code_format_angle_terms(value)
        .replace('[', r"\[")
        .replace(']', r"\]")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(" ,", ",")
        .replace(" .", ".")
        .replace("( ", "(")
        .replace(" )", ")")
}

fn code_format_angle_terms(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars().peekable();
    while let Some(character) = chars.next() {
        if character != '<' {
            output.push(character);
            continue;
        }

        let mut term = String::from("<");
        for next in chars.by_ref() {
            term.push(next);
            if next == '>' {
                break;
            }
        }

        if term.ends_with('>') {
            output.push('`');
            output.push_str(&term);
            output.push('`');
        } else {
            output.push_str(&term);
        }
    }
    output
}

fn with_mdn_attribute_reference(docs: &str, attribute: &str) -> String {
    with_mdn_section_reference(docs, &format!("Attribute/{attribute}"))
}

fn with_mdn_section_reference(docs: &str, path: &str) -> String {
    let reference =
        format!("[MDN reference](https://developer.mozilla.org/en-US/docs/Web/SVG/{path})");
    if docs.is_empty() {
        reference
    } else {
        format!("{docs}\n\n{reference}")
    }
}

fn parse_element_doc(path: &Path, attributes: &BTreeMap<String, AttributeInfo>) -> ElementInfo {
    let hydration = hydration(path);
    let name = path
        .file_stem()
        .and_then(|value| value.to_str())
        .expect("element page has a file name")
        .to_owned();
    let mut element = ElementInfo {
        name,
        summary: normalize_doc_text(&hydration.doc.summary),
        ..ElementInfo::default()
    };

    if let Some(content) = body_content(&hydration.doc, "attributes") {
        parse_element_attributes(content, attributes, &mut element.attributes);
    }
    if let Some(content) = body_content(&hydration.doc, "usage_context") {
        parse_usage_context(content, &mut element);
    }
    element
}

fn parse_element_attributes(
    content: &str,
    registry: &BTreeMap<String, AttributeInfo>,
    output: &mut BTreeMap<String, AttributeUse>,
) {
    let fragment = Html::parse_fragment(content);
    let dl_selector = selector("dl");
    let code_selector = selector("code");
    let link_selector = selector("a");

    for dl in fragment.select(&dl_selector) {
        let mut current = None;
        for child in dl.children() {
            let Some(child) = ElementRef::wrap(child) else {
                continue;
            };
            match child.value().name() {
                "dt" => {
                    current = child.select(&code_selector).next().map(text);
                }
                "dd" => {
                    if let Some(name) = current.take() {
                        let description = text(child);
                        output.insert(
                            name,
                            AttributeUse {
                                syntax: inline_value_syntax(&description),
                                docs: normalize_doc_text(&description),
                            },
                        );
                    }
                }
                _ => {}
            }
        }
    }

    for link in fragment.select(&link_selector) {
        let Some(href) = link.value().attr("href") else {
            continue;
        };
        if href.contains("/Attribute/") || href.contains("../Attribute/") {
            let name = link
                .select(&code_selector)
                .next()
                .map(text)
                .unwrap_or_default();
            if !name.is_empty() {
                let usage = registry
                    .get(&name)
                    .map(|info| AttributeUse {
                        syntax: info.syntax.clone(),
                        docs: info.summary.clone(),
                    })
                    .unwrap_or_else(|| AttributeUse {
                        syntax: name.clone(),
                        docs: String::new(),
                    });
                output.entry(name).or_insert(usage);
            }
        }
    }
}

fn inline_value_syntax(description: &str) -> String {
    let Some((_, after)) = description.split_once("Value type") else {
        return description.to_owned();
    };
    after
        .split_once("Default value")
        .map(|(value, _)| value)
        .unwrap_or(after)
        .trim_matches(|character: char| {
            character.is_whitespace() || character == ':' || character == ';'
        })
        .to_owned()
}

fn parse_usage_context(content: &str, element: &mut ElementInfo) {
    let fragment = Html::parse_fragment(content);
    let row_selector = selector("tr");
    let th_selector = selector("th");
    let td_selector = selector("td");
    let link_selector = selector("a");

    for row in fragment.select(&row_selector) {
        let heading = row
            .select(&th_selector)
            .next()
            .map(text)
            .unwrap_or_default();
        let Some(value) = row.select(&td_selector).next() else {
            continue;
        };
        if heading == "Categories" {
            for category in text(value).split(',') {
                let category = category.trim();
                if !category.eq_ignore_ascii_case("none") && !category.is_empty() {
                    element.categories.insert(category_ident(category));
                }
            }
        } else if heading == "Permitted content" {
            element.permits_text = text(value).contains("Character data");
            for link in value.select(&link_selector) {
                let href = link.value().attr("href").unwrap_or_default();
                let label = text(link);
                if href.contains("Element.html#") || href.contains("/SVG/Element#") {
                    element.child_categories.insert(category_ident(&label));
                } else if let Some(child) = linked_element_name(href) {
                    element.child_elements.insert(child);
                }
            }
        }
    }
}

fn linked_element_name(href: &str) -> Option<String> {
    let marker = if href.contains("/Element/") {
        "/Element/"
    } else if href.contains("../Element/") {
        "../Element/"
    } else {
        return None;
    };
    let name = href.split(marker).nth(1)?.split(['.', '#', '?']).next()?;
    (!name.is_empty()).then(|| name.to_owned())
}

fn generate(
    elements: &BTreeMap<String, ElementInfo>,
    preserve_aspect_ratio: &[String],
    path_commands: &[PathCommand],
    transform_functions: &[TransformFunction],
) -> TokenStream {
    let generated_types =
        generate_svg_types(preserve_aspect_ratio, path_commands, transform_functions);
    let structs = elements
        .values()
        .map(|element| generate_element(element, elements));
    let node_variants = elements.values().map(|element| {
        let name = type_ident(&element.name);
        quote! { #name(elements::#name) }
    });
    let node_display_arms = elements.values().map(|element| {
        let name = type_ident(&element.name);
        quote! { Self::#name(element) => std::fmt::Display::fmt(element, f) }
    });
    let node_from_impls = elements.values().map(|element| {
        let name = type_ident(&element.name);
        quote! {
            impl From<elements::#name> for Node {
                fn from(element: elements::#name) -> Self {
                    Self::#name(element)
                }
            }
        }
    });

    quote! {
        pub mod types {
            #generated_types
        }

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub enum Node {
            #( #node_variants, )*
            CharacterData(String),
            Raw(crate::Raw),
        }

        impl From<String> for Node {
            fn from(value: String) -> Self {
                Self::CharacterData(value)
            }
        }

        impl From<&str> for Node {
            fn from(value: &str) -> Self {
                Self::CharacterData(value.to_owned())
            }
        }

        impl From<crate::Raw> for Node {
            fn from(value: crate::Raw) -> Self {
                Self::Raw(value)
            }
        }

        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( #node_display_arms, )*
                    Self::CharacterData(value) => f.write_str(&crate::escape_text(value)),
                    Self::Raw(value) => std::fmt::Display::fmt(value, f),
                }
            }
        }

        pub mod elements {
            use crate::types::*;
            use crate::{ElementData, Node};

            pub trait SvgElement: Into<Node> + Clone {}
            #( #structs )*
        }

        #( #node_from_impls )*
    }
}

fn generate_svg_types(
    preserve_aspect_ratio: &[String],
    path_commands: &[PathCommand],
    transform_functions: &[TransformFunction],
) -> TokenStream {
    let value_helpers = generate_value_helpers();
    let align_variants = preserve_aspect_ratio.iter().map(|keyword| {
        let variant = type_ident(keyword);
        quote! { #variant }
    });
    let align_display_arms = preserve_aspect_ratio.iter().map(|keyword| {
        let variant = type_ident(keyword);
        quote! { Self::#variant => #keyword }
    });
    let path_methods = path_commands.iter().map(|command| {
        let method = format_ident!("{}", command.command);
        let command_name = &command.command;
        let docs = &command.docs;
        if command.parameters.is_empty() {
            return quote! {
                #[doc = #docs]
                pub fn #method(mut self) -> Self {
                    self.commands.push(#command_name.to_owned());
                    self
                }
            };
        }
        let parameters = command.parameters.iter().map(|parameter| {
            let ident = method_ident(parameter);
            let ty = if parameter.contains("flag") {
                quote! { bool }
            } else {
                quote! { f64 }
            };
            quote! { #ident: #ty }
        });
        let arguments = command.parameters.iter().map(|parameter| {
            let ident = method_ident(parameter);
            if parameter.contains("flag") {
                quote! { (#ident as u8).to_string() }
            } else {
                quote! { #ident.to_string() }
            }
        });
        quote! {
            #[doc = #docs]
            pub fn #method(mut self, #( #parameters ),*) -> Self {
                let arguments = [#( #arguments ),*].join(" ");
                self.commands.push(
                    if arguments.is_empty() {
                        #command_name.to_owned()
                    } else {
                        format!("{} {}", #command_name, arguments)
                    }
                );
                self
            }
        }
    });
    let transform_methods = transform_functions.iter().map(|function| {
        let method = method_ident(&function.name);
        let function_name = &function.name;
        let docs = &function.docs;
        let required_parameters = function.required.iter().map(|parameter| {
            let ident = method_ident(parameter);
            quote! { #ident: f64 }
        });
        let required_arguments = function.required.iter().map(|parameter| {
            let ident = method_ident(parameter);
            quote! { #ident.to_string() }
        });
        let arguments_mutability = (!function.optional.is_empty()).then(|| quote! { mut });
        let (optional_parameter, optional_arguments) = if function.optional.len() == 1 {
            let ident = method_ident(&function.optional[0]);
            (
                quote! { , #ident: Option<f64> },
                quote! {
                    if let Some(#ident) = #ident {
                        arguments.push(#ident.to_string());
                    }
                },
            )
        } else if function.optional.is_empty() {
            (quote! {}, quote! {})
        } else {
            let idents = function
                .optional
                .iter()
                .map(|parameter| method_ident(parameter))
                .collect::<Vec<_>>();
            let types = function.optional.iter().map(|_| quote! { f64 });
            let pushes = idents
                .iter()
                .map(|ident| quote! { arguments.push(#ident.to_string()); });
            (
                quote! { , optional: Option<(#( #types ),*)> },
                quote! {
                    if let Some((#( #idents ),*)) = optional {
                        #( #pushes )*
                    }
                },
            )
        };

        quote! {
            #[doc = #docs]
            pub fn #method(mut self, #( #required_parameters ),* #optional_parameter) -> Self {
                let #arguments_mutability arguments: Vec<String> = vec![#( #required_arguments ),*];
                #optional_arguments
                self.values.push(format!("{}({})", #function_name, arguments.join(" ")));
                self
            }
        }
    });

    quote! {
        #value_helpers

        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum PreserveAspectRatioAlign {
            #( #align_variants ),*
        }

        impl std::fmt::Display for PreserveAspectRatioAlign {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    #( #align_display_arms ),*
                })
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum PreserveAspectRatio {
            Align(PreserveAspectRatioAlign),
            Meet(PreserveAspectRatioAlign),
            Slice(PreserveAspectRatioAlign),
            Raw(String),
        }

        impl PreserveAspectRatio {
            pub fn meet(align: PreserveAspectRatioAlign) -> Self {
                Self::Meet(align)
            }

            pub fn slice(align: PreserveAspectRatioAlign) -> Self {
                Self::Slice(align)
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self::Raw(value.into())
            }
        }

        impl From<PreserveAspectRatioAlign> for PreserveAspectRatio {
            fn from(value: PreserveAspectRatioAlign) -> Self {
                Self::Align(value)
            }
        }

        impl From<String> for PreserveAspectRatio {
            fn from(value: String) -> Self {
                Self::Raw(value)
            }
        }

        impl From<&str> for PreserveAspectRatio {
            fn from(value: &str) -> Self {
                Self::Raw(value.to_owned())
            }
        }

        impl std::fmt::Display for PreserveAspectRatio {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Align(align) => write!(f, "{align}"),
                    Self::Meet(align) => write!(f, "{align} meet"),
                    Self::Slice(align) => write!(f, "{align} slice"),
                    Self::Raw(value) => f.write_str(value),
                }
            }
        }

        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct PathData {
            commands: Vec<String>,
        }

        #[allow(clippy::too_many_arguments, non_snake_case)]
        impl PathData {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn raw(mut self, value: impl Into<String>) -> Self {
                self.commands.push(value.into());
                self
            }

            #( #path_methods )*
        }

        impl From<String> for PathData {
            fn from(value: String) -> Self {
                Self { commands: vec![value] }
            }
        }

        impl From<&str> for PathData {
            fn from(value: &str) -> Self {
                Self { commands: vec![value.to_owned()] }
            }
        }

        impl std::fmt::Display for PathData {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.commands.join(" "))
            }
        }

        impl TransformList {
            #( #transform_methods )*
        }
    }
}

fn generate_value_helpers() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct TextValue(String);

        impl TextValue {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl std::fmt::Display for TextValue {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for TextValue {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for TextValue {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Color(String);

        impl Color {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn hex(value: impl AsRef<str>) -> Self {
                let value = value.as_ref();
                Self(if value.starts_with('#') {
                    value.to_owned()
                } else {
                    format!("#{value}")
                })
            }

            pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
                Self(format!("rgb({red} {green} {blue})"))
            }

            pub fn rgba(red: u8, green: u8, blue: u8, alpha: f64) -> Self {
                Self(format!("rgb({red} {green} {blue} / {alpha})"))
            }
        }

        impl std::fmt::Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for Color {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for Color {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Length(String);

        impl Length {
            pub fn with_unit(value: f64, unit: impl std::fmt::Display) -> Self {
                Self(format!("{value}{unit}"))
            }

            pub fn unitless(value: f64) -> Self {
                Self(value.to_string())
            }

            pub fn percent(value: f64) -> Self {
                Self(format!("{value}%"))
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl std::fmt::Display for Length {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<f64> for Length {
            fn from(value: f64) -> Self {
                Self::unitless(value)
            }
        }

        impl From<i32> for Length {
            fn from(value: i32) -> Self {
                Self::unitless(f64::from(value))
            }
        }

        impl From<String> for Length {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for Length {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum Number {
            Value(f64),
            Raw(String),
        }

        impl Number {
            pub fn raw(value: impl Into<String>) -> Self {
                Self::Raw(value.into())
            }
        }

        impl std::fmt::Display for Number {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Value(value) => write!(f, "{value}"),
                    Self::Raw(value) => f.write_str(value),
                }
            }
        }

        impl From<f64> for Number {
            fn from(value: f64) -> Self {
                Self::Value(value)
            }
        }

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self::Value(f64::from(value))
            }
        }

        impl From<String> for Number {
            fn from(value: String) -> Self {
                Self::Raw(value)
            }
        }

        impl From<&str> for Number {
            fn from(value: &str) -> Self {
                Self::Raw(value.to_owned())
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Integer(pub i64);

        impl std::fmt::Display for Integer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i32> for Integer {
            fn from(value: i32) -> Self {
                Self(i64::from(value))
            }
        }

        impl From<i64> for Integer {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum Angle {
            Degrees(f64),
            Raw(String),
        }

        impl Angle {
            pub fn degrees(value: f64) -> Self {
                Self::Degrees(value)
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self::Raw(value.into())
            }
        }

        impl std::fmt::Display for Angle {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Degrees(value) => write!(f, "{value}"),
                    Self::Raw(value) => f.write_str(value),
                }
            }
        }

        impl From<f64> for Angle {
            fn from(value: f64) -> Self {
                Self::Degrees(value)
            }
        }

        impl From<String> for Angle {
            fn from(value: String) -> Self {
                Self::Raw(value)
            }
        }

        impl From<&str> for Angle {
            fn from(value: &str) -> Self {
                Self::Raw(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum NumberList {
            Values(Vec<f64>),
            Raw(String),
        }

        impl NumberList {
            pub fn new(values: impl IntoIterator<Item = f64>) -> Self {
                Self::Values(values.into_iter().collect())
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self::Raw(value.into())
            }
        }

        impl std::fmt::Display for NumberList {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Values(values) => {
                        for (index, value) in values.iter().enumerate() {
                            if index != 0 {
                                f.write_str(" ")?;
                            }
                            write!(f, "{value}")?;
                        }
                        Ok(())
                    }
                    Self::Raw(value) => f.write_str(value),
                }
            }
        }

        impl<const N: usize> From<[f64; N]> for NumberList {
            fn from(values: [f64; N]) -> Self {
                Self::Values(values.into())
            }
        }

        impl From<Vec<f64>> for NumberList {
            fn from(values: Vec<f64>) -> Self {
                Self::Values(values)
            }
        }

        impl From<String> for NumberList {
            fn from(value: String) -> Self {
                Self::Raw(value)
            }
        }

        impl From<&str> for NumberList {
            fn from(value: &str) -> Self {
                Self::Raw(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum Points {
            Values(Vec<(f64, f64)>),
            Raw(String),
        }

        impl Points {
            pub fn new(points: impl IntoIterator<Item = (f64, f64)>) -> Self {
                Self::Values(points.into_iter().collect())
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self::Raw(value.into())
            }
        }

        impl std::fmt::Display for Points {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Values(points) => {
                        for (index, (x, y)) in points.iter().enumerate() {
                            if index != 0 {
                                f.write_str(" ")?;
                            }
                            write!(f, "{x},{y}")?;
                        }
                        Ok(())
                    }
                    Self::Raw(value) => f.write_str(value),
                }
            }
        }

        impl<const N: usize> From<[(f64, f64); N]> for Points {
            fn from(points: [(f64, f64); N]) -> Self {
                Self::Values(points.into())
            }
        }

        impl From<Vec<(f64, f64)>> for Points {
            fn from(points: Vec<(f64, f64)>) -> Self {
                Self::Values(points)
            }
        }

        impl From<String> for Points {
            fn from(value: String) -> Self {
                Self::Raw(value)
            }
        }

        impl From<&str> for Points {
            fn from(value: &str) -> Self {
                Self::Raw(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Iri(String);

        impl Iri {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn local(id: impl AsRef<str>) -> Self {
                Self(format!("#{}", id.as_ref()))
            }
        }

        impl std::fmt::Display for Iri {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for Iri {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for Iri {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Paint(String);

        impl Paint {
            pub fn url(value: impl Into<Iri>) -> Self {
                Self(format!("url({})", value.into()))
            }

            pub fn raw(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl std::fmt::Display for Paint {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<Color> for Paint {
            fn from(value: Color) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for Paint {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for Paint {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct TransformList {
            values: Vec<String>,
        }

        impl TransformList {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn raw(mut self, value: impl Into<String>) -> Self {
                self.values.push(value.into());
                self
            }
        }

        impl std::fmt::Display for TransformList {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.values.join(" "))
            }
        }

        impl From<String> for TransformList {
            fn from(value: String) -> Self {
                Self { values: vec![value] }
            }
        }

        impl From<&str> for TransformList {
            fn from(value: &str) -> Self {
                Self { values: vec![value.to_owned()] }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct ViewBox {
            pub min_x: f64,
            pub min_y: f64,
            pub width: f64,
            pub height: f64,
        }

        impl std::fmt::Display for ViewBox {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {} {} {}", self.min_x, self.min_y, self.width, self.height)
            }
        }

        impl From<(f64, f64, f64, f64)> for ViewBox {
            fn from(value: (f64, f64, f64, f64)) -> Self {
                Self {
                    min_x: value.0,
                    min_y: value.1,
                    width: value.2,
                    height: value.3,
                }
            }
        }
    }
}

fn generate_element(
    element: &ElementInfo,
    elements: &BTreeMap<String, ElementInfo>,
) -> TokenStream {
    let type_name = type_ident(&element.name);
    let tag_name = &element.name;
    let doc = with_mdn_section_reference(&element.summary, &format!("Element/{}", element.name));
    let setters = element.attributes.iter().map(|(name, usage)| {
        let method = method_ident(name);
        let value_type = value_type(name, &usage.syntax);
        let docs = &usage.docs;
        quote! {
            #[doc = #docs]
            pub fn #method<T>(mut self, value: T) -> Self
            where
                T: Into<#value_type>,
            {
                self.data.set_attribute(#name, value.into());
                self
            }
        }
    });
    let child_trait = format_ident!("{}Child", type_name);
    let child_sealed_module = format_ident!("{}_child_sealed", snake_case(&element.name));
    let allowed_children = elements
        .values()
        .filter(|candidate| {
            element.child_elements.contains(&candidate.name)
                || candidate
                    .categories
                    .iter()
                    .any(|category| element.child_categories.contains(category))
        })
        .map(|child| type_ident(&child.name))
        .collect::<Vec<_>>();
    let sealed_impls = allowed_children
        .iter()
        .map(|child| quote! { impl Sealed for super::#child {} });
    let child_impls = allowed_children
        .iter()
        .map(|child| quote! { impl #child_trait for #child {} });
    let child_api = (!element.child_categories.is_empty() || !element.child_elements.is_empty())
        .then(|| {
            quote! {
                mod #child_sealed_module {
                    pub trait Sealed {}
                    #( #sealed_impls )*
                }

                pub trait #child_trait: SvgElement + #child_sealed_module::Sealed {}
                #( #child_impls )*
            }
        });
    let child_methods = child_api.is_some().then(|| {
        quote! {
            pub fn add_child<T>(mut self, child: T) -> Self
            where
                T: #child_trait,
            {
                self.data.push_child(child);
                self
            }

            pub fn add_children<I, T>(mut self, children: I) -> Self
            where
                I: IntoIterator<Item = T>,
                T: #child_trait,
            {
                for child in children {
                    self.data.push_child(child);
                }
                self
            }
        }
    });
    let add_text = element.permits_text.then(|| {
        quote! {
            pub fn add_text(mut self, text: impl Into<String>) -> Self {
                self.data.push_child(text.into());
                self
            }
        }
    });

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct #type_name {
            data: ElementData,
        }

        impl SvgElement for #type_name {}
        #child_api

        impl #type_name {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn attr(mut self, name: impl Into<String>, value: impl std::fmt::Display) -> Self {
                self.data.set_attribute(name, value);
                self
            }

            pub fn add_child_unchecked(mut self, child: impl Into<Node>) -> Self {
                self.data.push_child(child);
                self
            }

            #( #setters )*
            #child_methods
            #add_text
        }

        impl std::fmt::Display for #type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                crate::render_element(#tag_name, &self.data, f)
            }
        }
    }
}

fn value_type(name: &str, syntax: &str) -> TokenStream {
    let lower_name = name.to_ascii_lowercase();
    let lower = syntax.to_ascii_lowercase();
    let rust_type = if name == "d" || lower.contains("path_data") {
        "PathData"
    } else if name == "points" {
        "Points"
    } else if name == "viewBox" {
        "ViewBox"
    } else if name == "preserveAspectRatio" {
        "PreserveAspectRatio"
    } else if lower_name == "fill"
        || lower_name == "stroke"
        || lower_name.ends_with("-color")
        || lower.contains("<paint>")
    {
        "Paint"
    } else if lower.contains("transform-list") || lower_name.ends_with("transform") {
        "TransformList"
    } else if lower.contains("<url>") || lower.contains("<iri>") || lower_name == "href" {
        "Iri"
    } else if lower.contains("list-of-numbers")
        || lower.contains("<number> +")
        || lower.contains("<number>+")
        || lower.contains("<number> ,?")
    {
        "NumberList"
    } else if lower.contains("<length")
        || lower.contains("<percentage>")
        || lower.contains("<coordinate>")
        || lower.contains("auto |")
    {
        "Length"
    } else if lower.contains("<integer>") {
        "Integer"
    } else if lower.contains("<angle>") {
        "Angle"
    } else if lower.contains("<number>") || lower_name.contains("opacity") {
        "Number"
    } else {
        "TextValue"
    };
    rust_type.parse().expect("valid generated type")
}

fn selector(value: &str) -> Selector {
    Selector::parse(value).unwrap_or_else(|_| panic!("valid selector: {value}"))
}

fn text(element: ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn category_ident(value: &str) -> String {
    let singular = value
        .trim()
        .trim_end_matches(" elements")
        .trim_end_matches(" element");
    format!("{}Element", pascal_case(singular))
}

fn type_ident(value: &str) -> Ident {
    format_ident!("{}", pascal_case(value))
}

fn method_ident(value: &str) -> Ident {
    let mut method = snake_case(value);
    if matches!(
        method.as_str(),
        "as" | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    ) {
        method.push('_');
    }
    format_ident!("{method}")
}

fn pascal_case(value: &str) -> String {
    let mut output = String::new();
    let mut uppercase = true;
    for character in value.chars() {
        if character == '-' || character == ':' || character == '_' || character.is_whitespace() {
            uppercase = true;
        } else if uppercase {
            output.extend(character.to_uppercase());
            uppercase = false;
        } else {
            output.push(character);
        }
    }
    output
}

fn snake_case(value: &str) -> String {
    let mut output = String::new();
    let mut previous_lowercase = false;
    for character in value.chars() {
        if character == '-' || character == ':' || character == ' ' || !character.is_alphanumeric()
        {
            if !output.ends_with('_') {
                output.push('_');
            }
            previous_lowercase = false;
        } else if character.is_uppercase() {
            if previous_lowercase && !output.ends_with('_') {
                output.push('_');
            }
            output.extend(character.to_lowercase());
            previous_lowercase = false;
        } else {
            output.push(character);
            previous_lowercase = character.is_lowercase();
        }
    }
    output.trim_matches('_').to_owned()
}

fn format_rust_code(code: &str) -> String {
    match syn::parse_file(code) {
        Ok(file) => prettyplease::unparse(&file),
        Err(error) => {
            println!("cargo:warning=failed to format generated code: {error}");
            code.to_owned()
        }
    }
}
