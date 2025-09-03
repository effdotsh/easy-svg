use crate::Svg;
use crate::shape::Shape;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SvgElement {
    Shape(Box<Shape>),
    Svg(Svg),
}
