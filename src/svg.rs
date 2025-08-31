use crate::Shape;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SvgElement {
    Shape(Box<Shape>),
    Svg(Svg),
}

impl From<Shape> for SvgElement {
    fn from(shape: Shape) -> Self {
        SvgElement::Shape(Box::new(shape))
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

#[allow(dead_code)]
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

        let mut str = format!(r#"<svg width="{}" height="{}""#, self.width, self.height);
        if let Some(x) = &self.x {
            str.push_str(&format!(r#" x="{}""#, x));
        }
        if let Some(y) = &self.y {
            str.push_str(&format!(r#" y="{}""#, y));
        }
        str.push('>');
        str.push_str(&children_svg);
        str.push_str("</svg>");
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::Rect;

    #[test]
    fn test_new_svg() {
        let svg = Svg::new(100.0, 200.0);
        assert_eq!(svg.width, 100.0);
        assert_eq!(svg.height, 200.0);
        assert!(svg.children.is_empty());
    }

    #[test]
    fn test_add_element() {
        let rect = Rect::new().width(10.0).height(20.0);
        let svg = Svg::new(100.0, 200.0).add_element(rect);
        assert_eq!(svg.children.len(), 1);
    }

    #[test]
    fn test_add_layer() {
        let inner_svg = Svg::new(50.0, 50.0);
        let svg = Svg::new(100.0, 200.0).add_layer(inner_svg);
        assert_eq!(svg.children.len(), 1);
    }

    #[test]
    fn test_x_and_y() {
        let svg = Svg::new(100.0, 200.0).x(10.0).y(20.0);
        assert_eq!(svg.x, Some(10.0));
        assert_eq!(svg.y, Some(20.0));
    }

    #[test]
    fn test_display() {
        let rect = Rect::new().width(10.0).height(20.0);
        let svg = Svg::new(100.0, 200.0).add_element(rect);
        let svg_string = svg.to_string();
        assert!(svg_string.starts_with(r#"<svg width="100" height="200">"#));
        assert!(svg_string.contains(r#"height="20""#));
        assert!(svg_string.contains(r#"width="10""#));
        assert!(svg_string.ends_with("</svg>"));
    }
}
