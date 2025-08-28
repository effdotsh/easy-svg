use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Circle(Circle),
    Rect(Rect),
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Shape::Circle(circle) => circle.to_string(),
            Shape::Rect(rect) => rect.to_string(),
        };
        write!(f, "{}", display_str)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub stroke: Option<Color>,
    pub cy: Option<f64>,
    pub cx: Option<f64>,
    pub r: Option<f64>,
    pub fill: Option<Color>,
}
impl Circle {
    pub fn new() -> Self {
        Self {
            stroke: None,
            cy: None,
            cx: None,
            r: None,
            fill: None,
        }
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn cy(mut self, value: f64) -> Self {
        self.cy = Some(value);
        self
    }
    pub fn cx(mut self, value: f64) -> Self {
        self.cx = Some(value);
        self
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
}
impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<circle"#,);
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(r#" cy="{}""#, cy));
        }
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(r#" cx="{}""#, cx));
        }
        if let Some(r) = &self.r {
            svg.push_str(&format!(r#" r="{}""#, r));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        svg.push_str("/>");
        write!(f, "{}", svg)
    }
}
impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub stroke: Option<Color>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub x: Option<f64>,
    pub fill: Option<Color>,
    pub y: Option<f64>,
}
impl Rect {
    pub fn new() -> Self {
        Self {
            stroke: None,
            height: None,
            width: None,
            x: None,
            fill: None,
            y: None,
        }
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }
    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
}
impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<rect"#,);
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(r#" height="{}""#, height));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(r#" width="{}""#, width));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(r#" x="{}""#, x));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        if let Some(y) = &self.y {
            svg.push_str(&format!(r#" y="{}""#, y));
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
