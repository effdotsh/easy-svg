use crate::BaseProperties;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    #[serde(flatten)]
    base: BaseProperties,
    width: f64,
    height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<f64>,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            base: BaseProperties::default(),
            width,
            height,
            corner_radius: None,
        }
    }

    pub fn corner_radius(mut self, radius: f64) -> Self {
        self.corner_radius = Some(radius);
        self
    }
}

impl_shape_builder!(Rect);

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut svg = format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}""#,
            self.base.transform.x.unwrap_or(0.0),
            self.base.transform.y.unwrap_or(0.0),
            self.width,
            self.height
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
        if let Some(corner_radius) = self.corner_radius {
            svg.push_str(&format!(
                r#" rx="{}" ry="{}""#,
                corner_radius, corner_radius
            ));
        }
        svg.push_str(" />");
        write!(f, "{}", svg)
    }
}
