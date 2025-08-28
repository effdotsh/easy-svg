mod color;
mod generated;
mod svg;

pub use generated::*;
#[cfg(test)]
mod tests {
    use crate::Rect;
    use crate::color::Color;
    use crate::svg::Svg;

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

    #[test]
    fn test_rect_in_svg() {
        let svg = Svg::new(500.0, 500.0).add_element(Rect::new(20.0, 20.0));
        println!("{}", svg);
    }
}
