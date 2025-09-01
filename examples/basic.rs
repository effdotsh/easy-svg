use easy_svg::color::Color;
use easy_svg::svg::Svg;
use easy_svg::{Circle, Rect, Text};

fn main() {
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
}
