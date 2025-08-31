use crate::color::Color;
use serde::{Deserialize, Serialize};
pub trait AnimationElement: Into<Shape> + Clone {}
pub trait BasicShape: Into<Shape> + Clone {}
pub trait ContainerElement: Into<Shape> + Clone {}
pub trait DescriptiveElement: Into<Shape> + Clone {}
pub trait FilterPrimitiveElement: Into<Shape> + Clone {}
pub trait GradientElement: Into<Shape> + Clone {}
pub trait GraphicsElement: Into<Shape> + Clone {}
pub trait GraphicsReferencingElement: Into<Shape> + Clone {}
pub trait LightSourceElement: Into<Shape> + Clone {}
pub trait NeverRenderedElement: Into<Shape> + Clone {}
pub trait PaintServerElement: Into<Shape> + Clone {}
pub trait RenderableElement: Into<Shape> + Clone {}
pub trait ShapeElement: Into<Shape> + Clone {}
pub trait StructuralElement: Into<Shape> + Clone {}
pub trait TextContentChildElement: Into<Shape> + Clone {}
pub trait TextContentElement: Into<Shape> + Clone {}
pub trait UncategorizedElement: Into<Shape> + Clone {}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    A(A),
    Circle(Circle),
    Rect(Rect),
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Shape::A(a) => a.to_string(),
            Shape::Circle(circle) => circle.to_string(),
            Shape::Rect(rect) => rect.to_string(),
        };
        write!(f, "{}", display_str)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A {
    pub download: Option<String>,
    children: Vec<Shape>,
}
impl ContainerElement for A {}
impl A {
    pub fn new() -> Self {
        Self {
            download: None,
            children: Vec::new(),
        }
    }
    pub fn download(mut self, value: String) -> Self {
        self.download = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_shape_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + ShapeElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_structural_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + StructuralElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_gradient_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + GradientElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_rect<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + Into<Rect>,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "a", "",);
        if let Some(download) = &self.download {
            svg.push_str(&format!(" {}=\"{}\"", "download", download));
        }
        if (self.children.is_empty()) {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "a"));
        write!(f, "{}", svg)
    }
}
impl From<A> for Shape {
    fn from(a: A) -> Self {
        Self::A(a)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub fill: Option<Color>,
    pub r: Option<f64>,
    pub stroke: Option<Color>,
    children: Vec<Shape>,
}
impl BasicShape for Circle {}
impl Circle {
    pub fn new() -> Self {
        Self {
            cx: None,
            cy: None,
            fill: None,
            r: None,
            stroke: None,
            children: Vec::new(),
        }
    }
    pub fn cx(mut self, value: f64) -> Self {
        self.cx = Some(value);
        self
    }
    pub fn cy(mut self, value: f64) -> Self {
        self.cy = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "circle", "",);
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(" {}=\"{}\"", "cx", cx));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(" {}=\"{}\"", "cy", cy));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(r) = &self.r {
            svg.push_str(&format!(" {}=\"{}\"", "r", r));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if (self.children.is_empty()) {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "circle"));
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
    pub fill: Option<Color>,
    pub height: Option<f64>,
    pub stroke: Option<Color>,
    pub width: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Rect {}
impl Rect {
    pub fn new() -> Self {
        Self {
            fill: None,
            height: None,
            stroke: None,
            width: None,
            x: None,
            y: None,
            children: Vec::new(),
        }
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
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
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "rect", "",);
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(" {}=\"{}\"", "height", height));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(" {}=\"{}\"", "x", x));
        }
        if let Some(y) = &self.y {
            svg.push_str(&format!(" {}=\"{}\"", "y", y));
        }
        if (self.children.is_empty()) {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "rect"));
        write!(f, "{}", svg)
    }
}
impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}
