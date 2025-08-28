use easy_svg::color::Color;
use easy_svg::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rect = Rect::new(100.0, 50.0)
        .fill(Color::Red)
        .stroke(Color::Black)
        .stroke_width(2.0)
        .position(50.0, 50.0);

    let circle = Circle::new(30.0)
        .fill(Color::Blue)
        .stroke(Color::Green)
        .stroke_width(3.0)
        .position(200.0, 100.0);

    let text = Text::new("Hello World!")
        .font_size(24.0)
        .font_family("Arial")
        .fill(Color::Purple)
        .position(50.0, 200.0);

    let line = Line::new(vec![0.0, 0.0, 100.0, 100.0, 200.0, 50.0])
        .stroke(Color::Orange)
        .stroke_width(5.0)
        .line_cap("round")
        .position(300.0, 150.0);

    let svg = Svg::new(800.0, 600.0)
        .add_element(rect)
        .add_element(circle)
        .add_element(text)
        .add_element(line);

    println!("\nSVG output:\n{}", svg.to_string());

    Ok(())
}
