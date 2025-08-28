use crate::{BaseProperties, Shape};
use serde::{Deserialize, Serialize};
use std::fmt;

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
    base: BaseProperties,
    width: f64,
    height: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<SvgElement>,
}

impl Svg {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            base: BaseProperties::default(),
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

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.base.transform.x = Some(x);
        self.base.transform.y = Some(y);
        self
    }
}

impl fmt::Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let children_svg = self
            .children
            .iter()
            .map(|child| match child {
                SvgElement::Svg(element) => element.to_string(),
                SvgElement::Shape(shape) => shape.to_string(),
            })
            .collect::<Vec<String>>()
            .join("");

        write!(
            f,
            r#"<svg width="{}" height="{}" x="{}" y="{}" xmlns="http://www.w3.org/2000/svg">{}</svg>"#,
            self.width,
            self.height,
            self.base.transform.x.unwrap_or(0.0),
            self.base.transform.y.unwrap_or(0.0),
            children_svg,
        )
    }
}
