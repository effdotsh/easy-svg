use crate::BaseProperties;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    #[serde(flatten)]
    base: BaseProperties,
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self {
            base: BaseProperties::default(),
            radius,
        }
    }
}

impl_shape_builder!(Circle);

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut svg = format!(
            r#"<circle cx="{}" cy="{}" r="{}""#,
            self.base.transform.x.unwrap_or(0.0),
            self.base.transform.y.unwrap_or(0.0),
            self.radius
        );
        if let Some(fill) = &self.base.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        if let Some(stroke) = &self.base.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        if let Some(stroke_width) = self.base.stroke_width {
            svg.push_str(&format!(r#" stroke-width="{}""#, stroke_width));
        }
        svg.push_str(" />");
        write!(f, "{}", svg)
    }
}
