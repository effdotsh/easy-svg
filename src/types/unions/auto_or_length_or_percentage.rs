use crate::types::{Length, Percentage};
use std::fmt::{Display, Formatter};

pub enum AutoOrLengthOrPercentage {
    Length(Length),
    Percentage(Percentage),
    Auto,
}

impl Display for AutoOrLengthOrPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AutoOrLengthOrPercentage::Length(l) => l.to_string(),
                AutoOrLengthOrPercentage::Percentage(p) => p.to_string(),
                AutoOrLengthOrPercentage::Auto => "auto".to_string(),
            }
        )
    }
}

impl From<Length> for AutoOrLengthOrPercentage {
    fn from(length: Length) -> Self {
        AutoOrLengthOrPercentage::Length(length)
    }
}

impl From<Percentage> for AutoOrLengthOrPercentage {
    fn from(percentage: Percentage) -> Self {
        AutoOrLengthOrPercentage::Percentage(percentage)
    }
}

impl From<f64> for AutoOrLengthOrPercentage {
    fn from(value: f64) -> Self {
        //use length when a float is passed in
        AutoOrLengthOrPercentage::Length(value.into())
    }
}
