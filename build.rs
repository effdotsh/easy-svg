use proc_macro2::TokenStream;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use syn::__private::quote::{format_ident, quote};

#[derive(Deserialize, Debug)]
struct Config {
    elements: HashMap<String, Element>,
    derives: Option<HashMap<String, Derivable>>,
    element_types: HashMap<String, ElementType>,
}

#[derive(Deserialize, Clone, Debug)]
struct Derivable {
    fields: HashMap<String, Field>,
}

#[derive(Deserialize, Clone, Debug)]
struct ElementType {}

#[derive(Deserialize, Debug)]
struct Element {
    derives: Option<Vec<String>>,
    fields: HashMap<String, Field>,
    element_type: String,
    valid_child_types: Vec<String>,
    #[serde(default)]
    constructor_params: Vec<Param>,
}

#[derive(Deserialize, Clone, Debug)]
struct Param {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Field {
    #[serde(rename = "type")]
    field_type: String,
    from_constructor: Option<bool>,
}

fn main() {
    println!("cargo:rerun-if-chancategory_traitsged=svg_elements.yml");
    let yaml_content =
        fs::read_to_string("svg_elements.yml").expect("Failed to read svg_elements.yml");
    let mut config: Config = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");
    if let Some(derives) = &config.derives {
        for element in config.elements.values_mut() {
            if let Some(element_derives) = &element.derives {
                for derive_name in element_derives {
                    if let Some(derivable) = derives.get(derive_name) {
                        for (field_name, field) in &derivable.fields {
                            element.fields.insert(field_name.clone(), field.clone());
                        }
                    }
                }
            }
        }
    }

    let category_traits = generate_category_traits(&config);

    let shape_enum = generate_shape_enum(&config);

    let element_code = config.elements.iter().map(|(element_name, element)| {
        let struct_code = generate_struct(element_name, element);
        let impl_code = generate_impl(element_name, element);
        let to_string_code = generate_to_string(element_name, element);
        let shape_from_code = generate_shape_from(element_name);

        quote! {
            #struct_code
            #impl_code
            #to_string_code
            #shape_from_code
        }
    });

    let generated_code = quote! {
        use crate::color::Color;
        use serde::{Deserialize, Serialize};
        #category_traits
        #shape_enum
        #( #element_code )*
    };

    fs::write(
        "src/generated.rs",
        format_rust_code(&generated_code.to_string()),
    )
    .unwrap();
}
fn generate_category_traits(config: &Config) -> TokenStream {
    let category_traits = config.element_types.keys().map(|category| {
        let trait_name = format_ident!("{}", category);
        quote! {
            pub trait #trait_name: Into<Shape> + Clone {}
        }
    });

    quote! {
        #( #category_traits )*
    }
}

fn generate_shape_from(element_name: &String) -> TokenStream {
    let struct_name = capitalize(element_name);

    let struct_name_ident = format_ident!("{}", &struct_name);
    let element_name_ident = format_ident!("{}", &element_name);

    quote! {
        impl From<#struct_name_ident> for Shape {
            fn from(#element_name_ident: #struct_name_ident) -> Self {
                Self::#struct_name_ident(#element_name_ident)
            }
        }
    }
}

fn generate_shape_enum(config: &Config) -> TokenStream {
    let enum_variants = config.elements.iter().map(|(element_name, _)| {
        let struct_name_str = capitalize(element_name);
        let struct_name_ident = format_ident!("{}", struct_name_str);
        quote! {
            #struct_name_ident(#struct_name_ident)
        }
    });

    let display_match_arms = config.elements.iter().map(|(element_name, _)| {
        let struct_name_str = capitalize(element_name);
        let struct_name_ident = format_ident!("{}", struct_name_str);
        let element_name_ident = format_ident!("{}", element_name);

        quote! {
            Shape::#struct_name_ident(#element_name_ident) => #element_name_ident.to_string()
        }
    });

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(tag = "type")]
        pub enum Shape {
            #( #enum_variants ),*
        }

        impl std::fmt::Display for Shape {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let display_str = match self {
                    #( #display_match_arms ),*
                };
                write!(f, "{}", display_str)
            }
        }
    }
}
fn generate_struct(name: &str, element: &Element) -> TokenStream {
    let struct_name = capitalize(name);
    let mut fields = Vec::new();

    for (field_name, field) in &element.fields {
        let field_name_ident = format_ident!("{}", &field_name);
        let field_type_tokens: TokenStream = field
            .field_type
            .parse()
            .expect("Failed to parse field type");
        fields.push(quote! {
            pub #field_name_ident: #field_type_tokens
        });
    }

    let struct_name_ident = format_ident!("{}", struct_name);

    quote! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
     pub struct #struct_name_ident {
           #( #fields ),*,
            pub children: Vec<Shape>
         }
     }
}

fn generate_to_string(name: &str, element: &Element) -> TokenStream {
    let struct_name = capitalize(name);
    let struct_name_ident = format_ident!("{}", struct_name);

    let required_parameters = element
        .constructor_params
        .iter()
        .map(|param| format!(" {}=\"{{}}\"", param.name))
        .collect::<Vec<_>>()
        .join("");

    let required_arguments = element.constructor_params.iter().map(|param| {
        let param_name_ident = format_ident!("{}", &param.name);
        quote! {
            self.#param_name_ident
        }
    });

    let optional_field_handling = element.fields.iter().filter_map(|(field_name, field)| {
        if field.from_constructor.unwrap_or(false) {
            return None;
        }

        let field_name_ident = format_ident!("{}", field_name);
        Some(quote! {
            if let Some(#field_name_ident) = &self.#field_name_ident {
                svg.push_str(&format!(" {}=\"{}\"", #field_name, #field_name_ident));
            }
        })
    });

    quote! {
        impl std::fmt::Display for #struct_name_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut svg = format!(
                    r#"<{}{}"#,
                    #name,
                    #required_parameters,
                    #( #required_arguments ),*
                );

                #( #optional_field_handling )*

                 if (self.children.is_empty()) {
                    svg.push_str("/>");
                    return write!(f, "{}", svg);
                }

                svg.push_str(">");
                for child in self.children.iter() {
                    svg.push_str(&child.to_string());
                }
                svg.push_str(&format!("</{}>", #name));
                write!(f, "{}", svg)
                    }
                }
    }
}
fn generate_impl(name: &str, element: &Element) -> TokenStream {
    let struct_name = capitalize(name);
    let constructor_tokens = generate_constructor(element);

    let builder_methods = element
        .fields
        .iter()
        .filter_map(|(field_name, field)| {
            if !field.from_constructor.unwrap_or(false) {
                return Some(generate_builder_method(field_name, field));
            }
            return None;
        })
        .collect::<Vec<_>>();
    let struct_name_ident = format_ident!("{}", struct_name);

    let element_type_ident = format_ident!("{}", element.element_type);

    let children_methods = generate_children_methods(element);

    //todo(effdotsh) add check to verify valid element type. Not super critical because will fail to generate proper code but harder to debug without it
    quote! {
        impl #element_type_ident for #struct_name_ident {}
        impl #struct_name_ident {
            #constructor_tokens
            #( #builder_methods )*
          #( #children_methods )*
        }
    }
}

fn generate_children_methods(element: &Element) -> Vec<TokenStream> {
    let mut methods = Vec::new();
    for child_type in element.valid_child_types.iter() {
        let child_type_ident = format_ident!("{}", child_type);
        let method_name_ident = format_ident!("add_child_{}", camel_to_snake(child_type));
        methods.push(quote! {
            fn #method_name_ident <T>(mut self, child: T) -> Self
            where
                T: Into<Shape> + #child_type_ident,
            {
                self.children.push(child.into());
                self
            }
        });
    }
    methods
}

fn generate_constructor(element: &Element) -> TokenStream {
    let constructor_params = element.constructor_params.iter().map(|p| {
        let param_name_ident = format_ident!("{}", &p.name);
        let param_type_tokens: TokenStream = p
            .param_type
            .parse()
            .expect(&format!("Invalid parameter type: {}", p.param_type));

        quote! {
            #param_name_ident: #param_type_tokens
        }
    });

    let field_assignments = element.fields.iter().map(|(field_name, field)| {
        let field_name_ident = format_ident!("{}", field_name);

        if field.from_constructor.unwrap_or(false) {
            quote! {
                #field_name_ident
            }
        } else {
            quote! {
                #field_name_ident: None
            }
        }
    });

    quote! {
        pub fn new(#( #constructor_params ),*) -> Self {
            Self {
                #( #field_assignments ),*,
                 children: Vec::new()
            }
        }
    }
}
fn generate_builder_method(field_name: &str, field: &Field) -> TokenStream {
    let param_type = if field.field_type.starts_with("Option<") {
        &field.field_type[7..field.field_type.len() - 1]
    } else {
        &field.field_type
    };

    let field_name_ident = format_ident!("{}", field_name);
    let param_type_tokens: TokenStream = param_type.parse().expect("Failed to parse field type");
    quote! {
        pub fn #field_name_ident(mut self, value: #param_type_tokens) -> Self {
            self.#field_name_ident = Some(value);
            self
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
fn format_rust_code(code: &str) -> String {
    match syn::parse_file(code) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(e) => {
            println!("cargo:warning=Failed to parse generated code: {}", e);
            println!("cargo:warning=Using unformatted code");
            code.to_string()
        }
    }
}

fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lower = false;

    for ch in s.chars() {
        if ch.is_uppercase() {
            if prev_was_lower {
                result.push('_');
            }
            result.extend(ch.to_lowercase());
            prev_was_lower = false;
        } else {
            result.push(ch);
            prev_was_lower = ch.is_lowercase();
        }
    }

    result
}
