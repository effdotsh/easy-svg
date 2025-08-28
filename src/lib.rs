mod color;
mod generated;
mod svg;

pub use generated::*;
#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::svg::Svg;
    use crate::{Circle, Rect};

    #[test]
    fn test_rect() {
        let rect = Rect::new()
            .width(100.0)
            .height(100.0)
            .fill(Color::Aqua)
            .stroke(Color::Black);

        assert_eq!(rect.width, Some(100.0));
        assert_eq!(rect.height, Some(50.0));
        assert_eq!(rect.fill, Some(Color::Aqua));
        println!("{}", rect);
    }

    #[test]
    fn test_rect_in_svg() {
        let svg = Svg::new(500., 500.)
            .add_element(
                Rect::new()
                    .width(200.)
                    .height(400.)
                    .x(20.)
                    .fill(Color::DarkOliveGreen),
            )
            .add_element(Circle::new().fill(Color::Aqua).r(20.).cx(15.).cy(30.));
        println!("{}", svg);
    }
}
