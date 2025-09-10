use crate::types::{Length, Percentage};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl Display for LengthOrPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthOrPercentage::Length(l) => l.to_string(),
                LengthOrPercentage::Percentage(p) => p.to_string(),
            }
        )
    }
}

impl From<Length> for LengthOrPercentage {
    fn from(length: Length) -> Self {
        LengthOrPercentage::Length(length)
    }
}

impl From<Percentage> for LengthOrPercentage {
    fn from(percentage: Percentage) -> Self {
        LengthOrPercentage::Percentage(percentage)
    }
}

impl From<f64> for LengthOrPercentage {
    fn from(value: f64) -> Self {
        //use length when a float is passed in
        LengthOrPercentage::Length(value.into())
    }
}
