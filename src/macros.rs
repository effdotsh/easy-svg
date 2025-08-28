#[macro_export]
macro_rules! impl_shape_builder {
    ($T:ty) => {
        impl $T {
            pub fn fill(mut self, color: $crate::color::Color) -> Self {
                self.base.fill = Some(color.into());
                self
            }

            pub fn stroke(mut self, color: $crate::color::Color) -> Self {
                self.base.stroke = Some(color.into());
                self
            }

            pub fn stroke_width(mut self, width: f64) -> Self {
                self.base.stroke_width = Some(width);
                self
            }

            pub fn position(mut self, x: f64, y: f64) -> Self {
                self.base.transform.x = Some(x);
                self.base.transform.y = Some(y);
                self
            }
        }
    };
}
