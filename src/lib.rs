mod color;
mod generated;
mod svg;

pub use generated::*;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_rect() {
        let rect = Rect::new(100.0, 50.0)
            .fill(Color::Aqua)
            .stroke(Color::Black);

        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
        assert_eq!(rect.fill, Some(Color::Aqua));
        println!("{}", rect);
    }
}
