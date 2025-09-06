use std::fmt::Formatter;

pub struct Percentage {
    percentage: f64,
}
impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.percentage)
    }
}
