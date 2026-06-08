# easy-svg

Typed Rust SVG builders generated at compile time from the offline MDN reference
in `SVG/`. Elements, attributes, permitted children, value helpers, and hover
documentation update when the reference is replaced.

```rust
use easy_svg::elements::{Circle, Svg};

let svg = Svg::new()
    .view_box((0., 0., 100., 100.))
    .add_child(Circle::new().cx(50.).cy(50.).r(40.).fill("gold"));

assert_eq!(
    svg.to_string(),
    r#"<svg viewBox="0 0 100 100"><circle cx="50" cy="50" fill="gold" r="40"/></svg>"#
);
```

Parents expose typed `.add_child(...)` and `.add_children(...)` methods.
Open-ended values accept strings; `.attr(...)` and `.add_child_unchecked(...)`
are escape hatches to build invalid SVGs.
Use `.to_string_pretty()` for copy-friendly output.

To regenerate after replacing `SVG/`:

```bash
cargo test --all-targets
```
