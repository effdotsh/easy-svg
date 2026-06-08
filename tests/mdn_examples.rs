use easy_svg::FormatOptions;
use easy_svg::elements::{Path, Polygon, Polyline, Svg, Text, Tspan};
use easy_svg::types::{PathData, Points, TransformList};

// SVG/developer.mozilla.org/en-US/docs/Web/SVG/Attribute/points.html
#[test]
fn builds_mdn_points_example() {
    let points = Points::from([(50., 0.), (21., 90.), (98., 35.), (2., 35.), (79., 90.)]);
    let svg = Svg::new()
        .view_box((-10., -10., 220., 120.))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .add_child(
            Polyline::new()
                .stroke("black")
                .fill("none")
                .points(points.clone()),
        )
        .add_child(
            Polygon::new()
                .stroke("black")
                .fill("none")
                .transform(TransformList::new().translate(100., Some(0.)))
                .points(points),
        );

    assert_eq!(
        svg.to_string_pretty(),
        r#"<svg viewBox="-10 -10 220 120" xmlns="http://www.w3.org/2000/svg">
  <polyline stroke="black" fill="none" points="50,0 21,90 98,35 2,35 79,90"/>
  <polygon stroke="black" fill="none" transform="translate(100 0)" points="50,0 21,90 98,35 2,35 79,90"/>
</svg>"#
    );
}

// SVG/developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule.html
#[test]
fn builds_mdn_fill_rule_example() {
    let svg = Svg::new().add_children([
        Polygon::new()
            .points(Points::from([
                (50., 0.),
                (21., 90.),
                (98., 35.),
                (2., 35.),
                (79., 90.),
            ]))
            .fill_rule("nonzero")
            .stroke("red"),
        Polygon::new()
            .points(Points::from([
                (150., 0.),
                (121., 90.),
                (198., 35.),
                (102., 35.),
                (179., 90.),
            ]))
            .fill_rule("evenodd")
            .stroke("red"),
    ]);

    assert_eq!(
        svg.to_string_pretty(),
        r#"<svg>
  <polygon points="50,0 21,90 98,35 2,35 79,90" fill-rule="nonzero" stroke="red"/>
  <polygon points="150,0 121,90 198,35 102,35 179,90" fill-rule="evenodd" stroke="red"/>
</svg>"#
    );
}

// SVG/developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d.html
#[test]
fn builds_mdn_heart_path_example() {
    let heart = PathData::new()
        .M(10., 30.)
        .A(20., 20., 0., false, true, 50., 30.)
        .A(20., 20., 0., false, true, 90., 30.)
        .Q(90., 60., 50., 90.)
        .Q(10., 60., 10., 30.)
        .z();
    let svg = Svg::new().add_child(Path::new().d(heart));

    assert_eq!(
        svg.to_string_pretty(),
        r#"<svg>
  <path d="M 10 30 A 20 20 0 0 1 50 30 A 20 20 0 0 1 90 30 Q 90 60 50 90 Q 10 60 10 30 z"/>
</svg>"#
    );
}

#[test]
fn supports_custom_pretty_indentation() {
    let svg =
        Svg::new().add_child(Polygon::new().points(Points::from([(0., 0.), (10., 0.), (5., 10.)])));

    assert_eq!(
        svg.to_string_with_options(&FormatOptions::pretty_with_indent("    ")),
        "<svg>\n    <polygon points=\"0,0 10,0 5,10\"/>\n</svg>"
    );
}

#[test]
fn pretty_formatting_does_not_change_text_content() {
    let svg = Svg::new().add_child(
        Text::new()
            .add_text("Hello ")
            .add_child(Tspan::new().add_text("<SVG>"))
            .add_text("!"),
    );

    assert_eq!(
        svg.to_string_pretty(),
        "<svg>\n  <text>Hello <tspan>&lt;SVG></tspan>!</text>\n</svg>"
    );
}
