use easy_svg::elements::{Circle, Defs, LinearGradient, Rect, Stop, Svg, Text};
use easy_svg::types::{Iri, Length, Paint, PreserveAspectRatioAlign, TransformList};

fn main() {
    let gradient = LinearGradient::new()
        .id("accent")
        .x1(Length::percent(0.))
        .x2(Length::percent(100.))
        .add_children([
            Stop::new().offset(Length::percent(0.)).stop_color("gold"),
            Stop::new()
                .offset(Length::percent(100.))
                .stop_color("darkorange"),
        ]);

    let svg = Svg::new()
        .view_box((0., 0., 500., 500.))
        .preserve_aspect_ratio(PreserveAspectRatioAlign::XMidYMid)
        .width(Length::with_unit(600., "px"))
        .height(Length::with_unit(600., "px"))
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
                .fill("darkmagenta")
                .transform(TransformList::new().rotate(-5., None))
                .add_text("Hello World"),
        )
        .add_child(Circle::new().fill("darkblue").r(20.).cx(80.).cy(85.));

    println!("{svg}");
}
