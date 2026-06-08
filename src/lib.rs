use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Controls how an SVG element is rendered to a string.
pub struct FormatOptions {
    indent: Option<String>,
}

impl FormatOptions {
    /// Renders without insignificant whitespace.
    #[must_use]
    pub const fn compact() -> Self {
        Self { indent: None }
    }

    /// Renders element children on separate lines with two-space indentation.
    #[must_use]
    pub fn pretty() -> Self {
        Self::pretty_with_indent("  ")
    }

    /// Renders element children on separate lines with the given indentation.
    #[must_use]
    pub fn pretty_with_indent(indent: impl Into<String>) -> Self {
        Self {
            indent: Some(indent.into()),
        }
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self::compact()
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ElementData {
    attributes: Vec<(String, String)>,
    children: Vec<Node>,
}

impl ElementData {
    pub fn set_attribute(&mut self, name: impl Into<String>, value: impl Display) {
        let name = name.into();
        let value = value.to_string();
        if let Some((_, current)) = self.attributes.iter_mut().find(|(key, _)| key == &name) {
            *current = value;
        } else {
            self.attributes.push((name, value));
        }
    }

    pub fn push_child(&mut self, child: impl Into<Node>) {
        self.children.push(child.into());
    }
}

#[doc(hidden)]
pub fn render_element(name: &str, data: &ElementData, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "<{name}")?;
    for (attribute, value) in &data.attributes {
        write!(f, " {attribute}=\"{}\"", escape_attribute(value))?;
    }
    if data.children.is_empty() {
        return f.write_str("/>");
    }
    f.write_str(">")?;
    for child in &data.children {
        write!(f, "{child}")?;
    }
    write!(f, "</{name}>")
}

#[doc(hidden)]
pub fn render_element_with_options(
    output: &mut String,
    name: &str,
    data: &ElementData,
    options: &FormatOptions,
    depth: usize,
) {
    if let Some(indent) = &options.indent {
        output.push_str(&indent.repeat(depth));
    }
    write!(output, "<{name}").expect("writing to String cannot fail");
    for (attribute, value) in &data.attributes {
        write!(output, " {attribute}=\"{}\"", escape_attribute(value))
            .expect("writing to String cannot fail");
    }
    if data.children.is_empty() {
        output.push_str("/>");
        return;
    }

    output.push('>');
    let pretty_children =
        options.indent.is_some() && data.children.iter().all(Node::supports_pretty_indentation);
    if pretty_children {
        output.push('\n');
        for (index, child) in data.children.iter().enumerate() {
            child.render_with_options(output, options, depth + 1);
            if index + 1 != data.children.len() {
                output.push('\n');
            }
        }
        output.push('\n');
        output.push_str(&options.indent.as_deref().unwrap_or_default().repeat(depth));
    } else {
        let compact = FormatOptions::compact();
        for child in &data.children {
            child.render_with_options(output, &compact, 0);
        }
    }
    write!(output, "</{name}>").expect("writing to String cannot fail");
}

fn escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

fn escape_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raw(String);

impl Raw {
    pub fn new(svg: impl Into<String>) -> Self {
        Self(svg.into())
    }
}

impl Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[allow(clippy::nursery, clippy::pedantic)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

pub use generated::types;
pub use generated::*;
