use serde::{Deserialize, Serialize};
use std::fmt;

#[macro_use]
pub mod macros;

pub mod circle;
pub mod color;
pub mod line;
mod linecaps;
pub mod rect;
pub mod svg;
pub mod text;

pub use circle::*;
pub use line::*;
pub use rect::*;
pub use svg::*;
pub use text::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseProperties {
    opacity: Option<f64>,
    #[serde(flatten)]
    transform: Transform,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Line(Line),
    Text(Text),
}

impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}
impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

impl From<Line> for Shape {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}
impl From<Text> for Shape {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Rect(rect) => write!(f, "{}", rect),
            Shape::Circle(circle) => write!(f, "{}", circle),
            Shape::Line(line) => write!(f, "{}", line),
            Shape::Text(text) => write!(f, "{}", text),
        }
    }
}
