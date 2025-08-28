use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Rect {
    pub width: f64,
    pub fill: Option<String>,
    pub height: f64,
    pub stroke: Option<String>,
}
impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            fill: None,
            height,
            stroke: None,
        }
    }
    pub fn fill(mut self, value: String) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn stroke(mut self, value: String) -> Self {
        self.stroke = Some(value);
        self
    }
}
impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(
            r#"<rect width="{}" height="{}" "#, self.width, self.height,
        );
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(r#" fill="{}""#, fill));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(r#" stroke="{}""#, stroke));
        }
        svg.push_str(" />");
        write!(f, "{}", svg)
    }
}
