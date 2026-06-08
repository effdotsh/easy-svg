use easy_svg::elements::{Circle, Defs, LinearGradient, Rect, Stop, Svg, Text};
use easy_svg::types::{Color, Iri, Length, Paint, PreserveAspectRatioAlign, TransformList};

fn main() {
    let gradient = LinearGradient::new()
        .id("accent")
        .x1(Length::percent(0.))
        .x2(Length::percent(100.))
        .add_children([
            Stop::new()
                .offset(Length::percent(0.))
                .stop_color(Color::GOLD),
            Stop::new()
                .offset(Length::percent(100.))
                .stop_color(Color::DARK_ORANGE),
        ]);

    let svg = Svg::new()
        .view_box((0., 0., 500., 500.))
        .preserve_aspect_ratio(PreserveAspectRatioAlign::XMidYMid)
        .width(Length::pixels(600))
        .height(Length::pixels(600))
        .add_child(Defs::new().add_child(gradient))
        .add_child(
            Rect::new()
                .width(200.)
                .height(400.)
                .x(20.)
                .fill(Paint::url(Iri::local("accent"))),
        )
        .add_child(
            Text::new()
                .x(30.)
                .y(70.)
                .fill(Color::DARK_MAGENTA)
                .transform(TransformList::new().rotate(-5., None))
                .add_text("Hello World"),
        )
        .add_child(Circle::new().fill(Color::DARK_BLUE).r(20.).cx(80.).cy(85.));

    println!("{svg}");
}
