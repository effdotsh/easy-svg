use crate::Shape;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SvgElement {
    Shape(Shape),
    Svg(Svg),
}

impl From<Shape> for SvgElement {
    fn from(shape: Shape) -> Self {
        SvgElement::Shape(shape)
    }
}

impl From<Svg> for SvgElement {
    fn from(svg: Svg) -> Self {
        SvgElement::Svg(svg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Svg {
    #[serde(flatten)]
    x: Option<f64>,
    y: Option<f64>,
    width: f64,
    height: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<SvgElement>,
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            x: None,
            y: None,
            width,
            height,
            children: Vec::new(),
        }
    }

    pub fn add_element(mut self, element: impl Into<Shape>) -> Self {
        self.children.push(element.into().into());
        self
    }

    pub fn add_layer(mut self, element: impl Into<Svg>) -> Self {
        self.children.push(element.into().into());
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }
}

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children_svg = self
            .children
            .iter()
            .map(|child| match child {
                SvgElement::Svg(element) => element.to_string(),
                SvgElement::Shape(shape) => shape.to_string(),
            })
            .collect::<Vec<String>>()
            .join("");

        let mut str = format!(r#"<svg width="{}" height="{}""#, self.width, self.height,);
        if let Some(x) = &self.x {
            str.push_str(&format!(r#" x="{}""#, x));
        }
        if let Some(y) = &self.y {
            str.push_str(&format!(r#" y="{}""#, y));
        }
        str.push_str(">");
        str.push_str(&children_svg);
        str.push_str("</svg>");
        write!(f, "{}", str)
    }
}
