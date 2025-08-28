use crate::BaseProperties;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    base: BaseProperties,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            base: BaseProperties::default(),
            text: text.into(),
            font_size: None,
            font_family: None,
            align: None,
        }
    }

    pub fn font_size(mut self, size: f64) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    pub fn align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }
}

impl_shape_builder!(Text);

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut svg = format!(
            r#"<text x="{}" y="{}""#,
            self.base.transform.x.unwrap_or(0.0),
            self.base.transform.y.unwrap_or(0.0),
        );
        if let Some(font_size) = self.font_size {
            svg.push_str(&format!(r#" font-size="{}""#, font_size));
        }
        if let Some(font_family) = &self.font_family {
            svg.push_str(&format!(r#" font-family="{}""#, font_family));
        }
        if let Some(fill) = &self.base.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        if let Some(align) = &self.align {
            svg.push_str(&format!(r#" text-anchor="{}""#, align));
        }
        svg.push_str(&format!(">{}</text>", self.text));
        write!(f, "{}", svg)
    }
}
