pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
mod shape {
    include!(concat!(env!("OUT_DIR"), "/shape.rs"));
}
pub mod color;
pub mod path_data;
pub mod svg;
pub use svg::*;
mod svg_element;
pub mod target;

pub use generated::*;
#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::elements::{A, Circle, Line, Path, Rect, Text};
    use crate::path_data::PathData;
    use crate::svg::Svg;

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

    #[test]
    fn test_rect_and_text() {
        let svg = Svg::new(500., 500.)
            .add_element(
                Rect::new()
                    .width(200.)
                    .height(400.)
                    .x(20.)
                    .fill(Color::DarkOliveGreen),
            )
            .add_element(
                Text::new()
                    .x(30.)
                    .y(70.)
                    .fill(Color::DarkMagenta)
                    .add_child_string("Hello World".to_string())
                    .font_family("Arial".to_string()),
            )
            .add_element(Circle::new().fill(Color::DarkBlue).r(20.).cx(80.).cy(85.));

        println!("{}", svg);

        assert_eq!(
            svg.to_string(),
            r#"<svg width="500" height="500"><rect fill="darkolivegreen" height="400" width="200" x="20"/><text fill="darkmagenta" font-family="Arial" x="30" y="70">Hello World</text><circle cx="80" cy="85" fill="darkblue" r="20"/></svg>"#
        );
    }
    #[test]
    fn test_line() {
        let svg = Svg::new(500., 500.).add_element(
            Line::new()
                .x1(10.)
                .y1(10.)
                .x2(100.)
                .y2(100.)
                .stroke(Color::Red),
        );

        println!("{}", svg);

        assert_eq!(
            svg.to_string(),
            r#"<svg width="500" height="500"><line stroke="red" x1="10" x2="100" y1="10" y2="100"/></svg>"#
        );
    }

    #[test]
    fn test_path_arc() {
        // from https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorials/SVG_from_scratch/Paths#arcs
        let path_data = PathData::new()
            .M(10., 315.)
            .L(110., 215.)
            .A(30., 50., 0., false, true, 162.55, 162.45)
            .L(172.55, 152.45)
            .A(30., 50., -45., false, true, 215.1, 109.9)
            .L(315., 10.);

        let svg = Svg::new(500., 500.).add_element(Path::new().d(path_data).fill(Color::Green));
        assert_eq!(
            svg.to_string(),
            r#""<svg width="500" height="500">M 10 315 L 110 215 A 30 50 0 0 1 162.55 162.45 L 172.55 152.45 A 30 50 -45 0 1 215.1 109.9 L 315 10"#
        )
    }
}
