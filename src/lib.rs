mod generated;
mod svg;
pub mod types;

pub use generated::*;
#[cfg(test)]
mod tests {
    use crate::svg::Svg;
    use crate::types::color::Color;
    use crate::{A, Circle, Rect};

    #[test]
    fn test_rect_and_circle() {
        let svg = Svg::new(500., 500.)
            .add_element(Circle::new().fill(Color::Aqua).r(20.).cx(15.).cy(30.))
            .add_element(
                Rect::new()
                    .width(200.)
                    .height(400.)
                    .x(20.)
                    .fill(Color::DarkOliveGreen),
            );

        println!("{}", svg);

        assert_eq!(
            svg.to_string(),
            r#"<svg width="500" height="500"><circle cx="15" cy="30" fill="aqua" r="20"/><rect fill="darkolivegreen" height="400" width="200" x="20"/></svg>"#
        );
    }

    #[test]
    fn test_a() {
        let svg = Svg::new(500., 500.).add_element(
            A::new().href("https://google.com".into()).add_child_rect(
                Rect::new()
                    .width(200.)
                    .height(400.)
                    .x(20.)
                    .fill(Color::DarkOliveGreen),
            ),
        );
        println!("{}", svg);
        assert_eq!(
            svg.to_string(),
            r#"<svg width="500" height="500"><a href="https://google.com"><rect fill="darkolivegreen" height="400" width="200" x="20"/></a></svg>"#
        );
    }
}
