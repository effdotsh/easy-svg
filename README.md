# easy-svg

Typed Rust SVG builders generated at compile time from the offline MDN reference
in `SVG/`. Elements, attributes, permitted children, value helpers, and hover
documentation update when the reference is replaced.

```rust
use easy_svg::elements::{Circle, Svg};
use easy_svg::types::Color;

let svg = Svg::new()
    .view_box((0., 0., 100., 100.))
    .add_child(Circle::new().cx(50.).cy(50.).r(40.).fill(Color::GOLD));

assert_eq!(
    svg.to_string(),
    r#"<svg viewBox="0 0 100 100"><circle cx="50" cy="50" fill="gold" r="40"/></svg>"#
);
```

Colors are typed: pass a named constant (`Color::GOLD`, `Color::DARK_ORANGE`,
`Color::CURRENT_COLOR`), build one with `Color::rgb(...)` / `Color::rgba(...)` /
`Color::hex(...)`, or use `Paint::NONE` / `Paint::url(...)`. A misspelled color
such as `"fakecolor"` is a compile error rather than an invalid SVG.

Lengths are typed too: pick the unit by constructor (`Length::pixels(600)`,
`Length::percent(50)`, `Length::em(1.5)`, `Length::unitless(10)`) instead of
hand-writing a unit suffix. A bare `"600px"` no longer compiles; use
`Length::raw(...)` for genuinely dynamic values like `calc(...)`.

Parents expose typed `.add_child(...)` and `.add_children(...)` methods, so an
element can only contain children the spec permits. Genuinely open-ended values
accept strings; `Paint::raw(...)`, `.attr(...)` and `.add_child_unchecked(...)`
are explicit escape hatches for deliberately building invalid SVGs.
Use `.to_string_pretty()` for copy-friendly output.

To regenerate after replacing `SVG/`:

```bash
cargo test --all-targets
```
