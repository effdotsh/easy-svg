use easy_svg::color::Color;
use easy_svg::{Circle, Rect};

#[test]
fn test_basic_circle() {
    let svg = Circle::new(500f64)
        .fill(Color::Red)
        .position(30f64, 5f64)
        .to_string();
    println!("SVG output:\n{}", svg);
    assert_eq!(svg, r#"<circle cx="30" cy="5" r="500" fill="red" />"#);
}

#[test]
fn test_basic_rect() {
    let svg = Rect::new(400f64, 500f64).corner_radius(20f64).to_string();
    println!("SVG output:\n{}", svg);
    assert_eq!(
        svg,
        r#"<rect x="0" y="0" width="400" height="500" rx="20" ry="20" />"#
    );
}
