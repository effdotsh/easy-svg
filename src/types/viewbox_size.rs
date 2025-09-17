use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBoxSize {
    min_x: f64,
    min_y: f64,
    width: f64,
    height: f64,
}
impl Display for ViewBoxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{}, {}, {}, {}",
            self.min_x, self.min_y, self.width, self.height
        );
        write!(f, "{}", str)
    }
}

impl From<(f64, f64, f64, f64)> for ViewBoxSize {
    fn from(tuple: (f64, f64, f64, f64)) -> Self {
        Self {
            min_x: tuple.0,
            min_y: tuple.1,
            width: tuple.2,
            height: tuple.3,
        }
    }
}
