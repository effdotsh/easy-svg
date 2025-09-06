use std::fmt::Display;

pub struct Length {
    length: f64,
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.length)
    }
}

impl From<f64> for Length {
    fn from(length: f64) -> Self {
        Self { length }
    }
}
