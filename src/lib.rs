use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

pub use generated::types;
pub use generated::*;
