use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Path {
    points: Vec<String>,
}

#[allow(non_snake_case)]
impl Path {
    pub fn new() -> Self {
        Path { points: Vec::new() }
    }

    pub fn M(mut self, x: f64, y: f64) -> Self {
        self.points.push(format!("M{} {}", x, y));
        self
    }

    pub fn m(mut self, dx: f64, dy: f64) -> Self {
        self.points.push(format!("m{} {}", dx, dy));
        self
    }

    pub fn L(mut self, x: f64, y: f64) -> Self {
        self.points.push(format!("L{} {}", x, y));
        self
    }

    pub fn l(mut self, dx: f64, dy: f64) -> Self {
        self.points.push(format!("l{} {}", dx, dy));
        self
    }

    pub fn H(mut self, x: f64) -> Self {
        self.points.push(format!("H{}", x));
        self
    }

    pub fn h(mut self, dx: f64) -> Self {
        self.points.push(format!("h{}", dx));
        self
    }

    pub fn V(mut self, y: f64) -> Self {
        self.points.push(format!("V{}", y));
        self
    }

    pub fn v(mut self, dy: f64) -> Self {
        self.points.push(format!("v{}", dy));
        self
    }

    pub fn Z(mut self) -> Self {
        self.points.push("Z".to_string());
        self
    }

    pub fn z(mut self) -> Self {
        self.points.push("z".to_string());
        self
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.join(" "))
    }
}
