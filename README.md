# easy-svg

`easy-svg` builds SVG documents with a statically typed Rust builder API.

The element model is generated at compile time from the offline MDN SVG reference
under `SVG/developer.mozilla.org/en-US/docs/Web/SVG/`. MDN element pages define
elements, attributes, categories, and permitted children; MDN attribute pages
define global applicability and value syntax.

```rust
use easy_svg::elements::{Circle, Svg};
use easy_svg::types::Length;

let svg = Svg::new()
    .view_box((0., 0., 100., 100.))
    .width(Length::percent(100.))
    .add_child(
        Circle::new()
            .cx(50.)
            .cy(50.)
            .r(40.)
            .fill("darkolivegreen"),
    );

assert_eq!(
    svg.to_string(),
    r#"<svg viewBox="0 0 100 100" width="100%"><circle cx="50" cy="50" fill="darkolivegreen" r="40"/></svg>"#
);
```

## Type system

Closed SVG-specific grammars are generated from MDN where its pages provide
structured syntax:

- `PathData` methods are generated from the command tables on MDN's `d` page.
- `PreserveAspectRatioAlign` is generated from MDN's `preserveAspectRatio` syntax.
- `TransformList` methods and parameter groups are generated from MDN's
  `transform` function sections.

Value helper families such as `Length`, `Paint`, `Color`, `Points`, `ViewBox`,
`TransformList`, and `Iri` are emitted into the generated `types` module.
Open-ended grammars accept raw values. In particular, `Color` is an open wrapper
rather than a named-color enum, because SVG color syntax comes from CSS; new
color syntax can be passed immediately without regenerating the crate. Length
units and paint keywords are open for the same reason.

Less structured MDN value grammars use `TextValue`.

Every generated element also provides `.attr(name, value)` and
`.add_child_unchecked(child)` escape hatches.

Parents with permitted element children expose `.add_child(...)` and
`.add_children(...)`. Each parent has its own generated child trait, so invalid
child elements are rejected at compile time. `add_children` accepts an iterator
whose items all have the same permitted element type; chain `add_child` calls
for heterogeneous children.

## Updating the generated API

Replace or update the offline MDN mirror in `SVG/`, then run:

```bash
cargo test --all-targets
```

Cargo reruns `build.rs`, which reads the MDN hydration data and generates the
element API and public SVG value types in `OUT_DIR`. The checked-in `src/`
directory contains only generic XML storage and rendering support.
