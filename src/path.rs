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
        self.points.push(format!("M {} {}", x, y));
        self
    }

    pub fn m(mut self, dx: f64, dy: f64) -> Self {
        self.points.push(format!("m {} {}", dx, dy));
        self
    }

    pub fn L(mut self, x: f64, y: f64) -> Self {
        self.points.push(format!("L {} {}", x, y));
        self
    }

    pub fn l(mut self, dx: f64, dy: f64) -> Self {
        self.points.push(format!("l {} {}", dx, dy));
        self
    }

    pub fn H(mut self, x: f64) -> Self {
        self.points.push(format!("H {}", x));
        self
    }

    pub fn h(mut self, dx: f64) -> Self {
        self.points.push(format!("h {}", dx));
        self
    }

    pub fn V(mut self, y: f64) -> Self {
        self.points.push(format!("V {}", y));
        self
    }

    pub fn v(mut self, dy: f64) -> Self {
        self.points.push(format!("v {}", dy));
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

    pub fn C(mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> Self {
        self.points
            .push(format!("C {} {}, {} {}, {} {}", x1, y1, x2, y2, x, y));
        self
    }

    pub fn c(mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx: f64, dy: f64) -> Self {
        self.points
            .push(format!("c {} {}, {} {}, {} {}", dx1, dy1, dx2, dy2, dx, dy));
        self
    }

    pub fn S(mut self, x2: f64, y2: f64, x: f64, y: f64) -> Self {
        self.points.push(format!("S {} {}, {} {}", x2, y2, x, y));
        self
    }
    pub fn s(mut self, dx2: f64, dy2: f64, dx: f64, dy: f64) -> Self {
        self.points
            .push(format!("s {} {}, {} {}", dx2, dy2, dx, dy));
        self
    }

    pub fn Q(mut self, x1: f64, y1: f64, x: f64, y: f64) -> Self {
        self.points.push(format!("Q  {} {}, {} {}", x1, y1, x, y));
        self
    }
    pub fn q(mut self, dx1: f64, dy1: f64, dx: f64, dy: f64) -> Self {
        self.points
            .push(format!("q  {} {}, {} {}", dx1, dy1, dx, dy));
        self
    }

    pub fn T(mut self, x: f64, y: f64) -> Self {
        self.points.push(format!("T {} {}", x, y));
        self
    }
    pub fn t(mut self, dx: f64, dy: f64) -> Self {
        self.points.push(format!("t {} {}", dx, dy));
        self
    }

    pub fn A(
        mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
        x: f64,
        y: f64,
    ) -> Self {
        self.points.push(format!(
            "A {} {} {} {} {} {} {}",
            rx, ry, x_axis_rotation, large_arc_flag as i8, sweep_flag as i8, x, y
        ));
        self
    }

    pub fn a(
        mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
        dx: f64,
        dy: f64,
    ) -> Self {
        self.points.push(format!(
            "a {} {} {} {} {} {} {}",
            rx, ry, x_axis_rotation, large_arc_flag as i8, sweep_flag as i8, dx, dy
        ));
        self
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.join(" "))
    }
}
