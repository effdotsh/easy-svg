use crate::color::Color;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Shape::Rect(rect) => rect.to_string(),
            Shape::Circle(circle) => circle.to_string(),
        };
        write!(f, "{}", display_str)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl Rect {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            fill: None,
            stroke: None,
            x: None,
            y: None,
        }
    }
    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
}
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "rect", "",);
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(" {}=\"{}\"", "height", height));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(" {}=\"{}\"", "x", x));
        }
        if let Some(y) = &self.y {
            svg.push_str(&format!(" {}=\"{}\"", "y", y));
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub r: Option<f64>,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
}
impl Circle {
    pub fn new() -> Self {
        Self {
            fill: None,
            stroke: None,
            r: None,
            cx: None,
            cy: None,
        }
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }
    pub fn cx(mut self, value: f64) -> Self {
        self.cx = Some(value);
        self
    }
    pub fn cy(mut self, value: f64) -> Self {
        self.cy = Some(value);
        self
    }
}
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "circle", "",);
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(r) = &self.r {
            svg.push_str(&format!(" {}=\"{}\"", "r", r));
        }
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(" {}=\"{}\"", "cx", cx));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(" {}=\"{}\"", "cy", cy));
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
