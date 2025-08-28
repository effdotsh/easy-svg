use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Rect(Rect),
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Shape::Rect(rect) => rect.to_string(),
        };
        write!(f, "{}", str)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub stroke: Option<Color>,
    pub height: f64,
    pub fill: Option<Color>,
    pub width: f64,
}
impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            stroke: None,
            height,
            fill: None,
            width,
        }
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
}
impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(
            r#"<rect width="{}" height="{}" "#, self.width, self.height,
        );
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        svg.push_str("/>");
        write!(f, "{}", svg)
    }
}
impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}
