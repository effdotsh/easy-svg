use easy_svg::elements::{Circle, G, Path, Polygon, Rect, Svg, Text};
use easy_svg::types::{Length, PathData, Points, TransformList};

#[test]
fn builds_typed_mdn_elements() {
    let svg = Svg::new().view_box((0., 0., 100., 100.)).add_child(
        G::new()
            .id("shapes")
            .transform(TransformList::new().translate(5., Some(10.)))
            .add_child(
                Polygon::new()
                    .points(Points::from([(0., 0.), (20., 0.), (10., 20.)]))
                    .fill("gold"),
            )
            .add_child(
                Path::new()
                    .d(PathData::new().M(0., 0.).L(10., 10.).Z())
                    .stroke("black")
                    .fill("none"),
            ),
    );

    assert_eq!(
        svg.to_string(),
        r##"<svg viewBox="0 0 100 100"><g id="shapes" transform="translate(5 10)"><polygon points="0,0 20,0 10,20" fill="gold"/><path d="M 0 0 L 10 10 Z" stroke="black" fill="none"/></g></svg>"##
    );
}

#[test]
fn supports_units_raw_attributes_and_xml_escaping() {
    let svg = Svg::new()
        .width(Length::percent(100.))
        .attr("data-label", "A&B")
        .add_child(
            Rect::new()
                .width(Length::with_unit(20., "px"))
                .height("calc(100% - 2px)"),
        )
        .add_child(Text::new().add_text("<typed & escaped>"))
        .add_child(Circle::new().r(5.));

    assert_eq!(
        svg.to_string(),
        r#"<svg width="100%" data-label="A&amp;B"><rect width="20px" height="calc(100% - 2px)"/><text>&lt;typed &amp; escaped></text><circle r="5"/></svg>"#
    );
}

#[test]
fn adds_multiple_permitted_children() {
    let svg = Svg::new().add_children([
        Circle::new().cx(10.).cy(10.).r(5.),
        Circle::new().cx(30.).cy(10.).r(5.),
    ]);

    assert_eq!(
        svg.to_string(),
        r#"<svg><circle cx="10" cy="10" r="5"/><circle cx="30" cy="10" r="5"/></svg>"#
    );
}

#[test]
fn builds_transform_functions_from_mdn_signatures() {
    let transforms = TransformList::new()
        .matrix(1., 0., 0., 1., 5., 10.)
        .translate(5., None)
        .scale(2., Some(3.))
        .rotate(45., Some((10., 20.)))
        .skew_x(15.)
        .skew_y(20.);

    assert_eq!(
        transforms.to_string(),
        "matrix(1 0 0 1 5 10) translate(5) scale(2 3) rotate(45 10 20) skewX(15) skewY(20)"
    );
}
