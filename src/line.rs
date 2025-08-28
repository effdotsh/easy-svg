use crate::BaseProperties;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    #[serde(flatten)]
    base: BaseProperties,
    points: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_join: Option<String>,
}

impl Line {
    pub fn new(points: Vec<f64>) -> Self {
        Self {
            base: BaseProperties::default(),
            points,
            line_cap: None,
            line_join: None,
        }
    }

    pub fn line_cap(mut self, cap: impl Into<String>) -> Self {
        self.line_cap = Some(cap.into());
        self
    }
}

impl_shape_builder!(Line);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let points_str = self
            .points
            .chunks(2)
            .map(|p| format!("{} {}", p[0], p[1]))
            .collect::<Vec<String>>()
            .join(" ");

        let mut svg = format!(r#"<polyline points="{}""#, points_str);

        if let Some(stroke) = &self.base.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        if let Some(stroke_width) = &self.base.stroke_width {
            svg.push_str(&format!(r#" stroke-width="{}""#, stroke_width));
        }
        if let Some(line_cap) = &self.line_cap {
            svg.push_str(&format!(r#" stroke-linecap="{}""#, line_cap));
        }
        if let Some(line_join) = &self.line_join {
            svg.push_str(&format!(r#" stroke-linejoin="{}""#, line_join));
        }
        svg.push_str(&format!(
            r#" fill="{}""#,
            self.base.fill.as_ref().unwrap_or(&"none".to_string())
        ));
        svg.push_str(" />");
        write!(f, "{}", svg)
    }
}
