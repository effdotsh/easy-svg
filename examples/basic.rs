use easy_svg::elements::{Circle, Rect, Svg, Text};
use easy_svg::types::color::Color;

fn main() {
    let svg = Svg::new()
        .width(500.)
        .height(500.)
        .add_child_shape_element(
            Rect::new()
                .width(200.)
                .height(400.)
                .x(20.)
                .fill(Color::DarkOliveGreen),
        )
        .add_child_text(
            Text::new()
                .x(30.)
                .y(70.)
                .fill(Color::DarkMagenta)
                .add_child_string("Hello World".to_string())
                .font_family("Arial".to_string()),
        )
        .add_child_shape_element(Circle::new().fill(Color::DarkBlue).r(20.).cx(80.).cy(85.));

    println!("{}", svg);
}
