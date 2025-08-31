use crate::types::color::Color;
use crate::types::target::Target;
use serde::{Deserialize, Serialize};
pub trait AnimationElement: Into<Shape> + Clone {}
pub trait BasicShape: Into<Shape> + Clone {}
pub trait ContainerElement: Into<Shape> + Clone {}
pub trait DescriptiveElement: Into<Shape> + Clone {}
pub trait FilterPrimitiveElement: Into<Shape> + Clone {}
pub trait GradientElement: Into<Shape> + Clone {}
pub trait GraphicsElement: Into<Shape> + Clone {}
pub trait GraphicsReferencingElement: Into<Shape> + Clone {}
pub trait LightSourceElement: Into<Shape> + Clone {}
pub trait NeverRenderedElement: Into<Shape> + Clone {}
pub trait PaintServerElement: Into<Shape> + Clone {}
pub trait RenderableElement: Into<Shape> + Clone {}
pub trait ShapeElement: Into<Shape> + Clone {}
pub trait StructuralElement: Into<Shape> + Clone {}
pub trait TextContentChildElement: Into<Shape> + Clone {}
pub trait TextContentElement: Into<Shape> + Clone {}
pub trait UncategorizedElement: Into<Shape> + Clone {}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    A(A),
    Circle(Circle),
    Defs(Defs),
    Desc(Desc),
    Ellipse(Ellipse),
    Line(Line),
    Rect(Rect),
    Text(Text),
    String(String),
}
impl From<String> for Shape {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Shape::A(a) => a.to_string(),
            Shape::Circle(circle) => circle.to_string(),
            Shape::Defs(defs) => defs.to_string(),
            Shape::Desc(desc) => desc.to_string(),
            Shape::Ellipse(ellipse) => ellipse.to_string(),
            Shape::Line(line) => line.to_string(),
            Shape::Rect(rect) => rect.to_string(),
            Shape::Text(text) => text.to_string(),
            Shape::String(string) => string.to_string(),
        };
        write!(f, "{}", display_str)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A {
    pub download: Option<String>,
    pub href: Option<String>,
    pub hreflang: Option<String>,
    pub referrerpolicy: Option<String>,
    pub target: Option<Target>,
    children: Vec<Shape>,
}
impl ContainerElement for A {}
impl Default for A {
    fn default() -> Self {
        Self::new()
    }
}
impl A {
    pub fn new() -> Self {
        Self {
            download: None,
            href: None,
            hreflang: None,
            referrerpolicy: None,
            target: None,
            children: Vec::new(),
        }
    }
    pub fn download(mut self, value: String) -> Self {
        self.download = Some(value);
        self
    }
    pub fn href(mut self, value: String) -> Self {
        self.href = Some(value);
        self
    }
    pub fn hreflang(mut self, value: String) -> Self {
        self.hreflang = Some(value);
        self
    }
    pub fn referrerpolicy(mut self, value: String) -> Self {
        self.referrerpolicy = Some(value);
        self
    }
    pub fn target(mut self, value: Target) -> Self {
        self.target = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_shape_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + ShapeElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_structural_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + StructuralElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_gradient_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + GradientElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_rect(mut self, child: Rect) -> Self {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "a", "",);
        if let Some(download) = &self.download {
            svg.push_str(&format!(" {}=\"{}\"", "download", download));
        }
        if let Some(href) = &self.href {
            svg.push_str(&format!(" {}=\"{}\"", "href", href));
        }
        if let Some(hreflang) = &self.hreflang {
            svg.push_str(&format!(" {}=\"{}\"", "hreflang", hreflang));
        }
        if let Some(referrerpolicy) = &self.referrerpolicy {
            svg.push_str(&format!(" {}=\"{}\"", "referrerpolicy", referrerpolicy));
        }
        if let Some(target) = &self.target {
            svg.push_str(&format!(" {}=\"{}\"", "target", target));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "a"));
        write!(f, "{}", svg)
    }
}
impl From<A> for Shape {
    fn from(a: A) -> Self {
        Self::A(a)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub element_timing: Option<String>,
    pub fill: Option<Color>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub r: Option<f64>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub stroke: Option<Color>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    children: Vec<Shape>,
}
impl BasicShape for Circle {}
impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}
impl Circle {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            cx: None,
            cy: None,
            element_timing: None,
            fill: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            r: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            stroke: None,
            style: None,
            tab_index: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn cx(mut self, value: f64) -> Self {
        self.cx = Some(value);
        self
    }
    pub fn cy(mut self, value: f64) -> Self {
        self.cy = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "circle", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(" {}=\"{}\"", "cx", cx));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(" {}=\"{}\"", "cy", cy));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(r) = &self.r {
            svg.push_str(&format!(" {}=\"{}\"", "r", r));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "circle"));
        write!(f, "{}", svg)
    }
}
impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defs {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub element_timing: Option<String>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    pub width: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Defs {}
impl Default for Defs {
    fn default() -> Self {
        Self::new()
    }
}
impl Defs {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            element_timing: None,
            height: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            style: None,
            tab_index: None,
            width: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_shape_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + ShapeElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_structural_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + StructuralElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_a(mut self, child: A) -> Self {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Defs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "defs", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(" {}=\"{}\"", "height", height));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "defs"));
        write!(f, "{}", svg)
    }
}
impl From<Defs> for Shape {
    fn from(defs: Defs) -> Self {
        Self::Defs(defs)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desc {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub element_timing: Option<String>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    children: Vec<Shape>,
}
impl DescriptiveElement for Desc {}
impl Default for Desc {
    fn default() -> Self {
        Self::new()
    }
}
impl Desc {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            element_timing: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            style: None,
            tab_index: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_basic_shape<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + BasicShape,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_container_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + ContainerElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_filter_primitive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + FilterPrimitiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_gradient_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + GradientElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_graphics_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + GraphicsElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_graphics_referencing_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + GraphicsReferencingElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_light_source_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + LightSourceElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_never_rendered_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + NeverRenderedElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_paint_server_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + PaintServerElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_renderable_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + RenderableElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_shape_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + ShapeElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_structural_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + StructuralElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_text_content_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + TextContentElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_text_content_child_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + TextContentChildElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_uncategorized_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + UncategorizedElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Desc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "desc", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "desc"));
        write!(f, "{}", svg)
    }
}
impl From<Desc> for Shape {
    fn from(desc: Desc) -> Self {
        Self::Desc(desc)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ellipse {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub element_timing: Option<String>,
    pub fill: Option<Color>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub path_length: Option<f64>,
    pub rx: Option<f64>,
    pub ry: Option<f64>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub stroke: Option<Color>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    children: Vec<Shape>,
}
impl BasicShape for Ellipse {}
impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}
impl Ellipse {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            cx: None,
            cy: None,
            element_timing: None,
            fill: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            path_length: None,
            rx: None,
            ry: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            stroke: None,
            style: None,
            tab_index: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn cx(mut self, value: f64) -> Self {
        self.cx = Some(value);
        self
    }
    pub fn cy(mut self, value: f64) -> Self {
        self.cy = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn path_length(mut self, value: f64) -> Self {
        self.path_length = Some(value);
        self
    }
    pub fn rx(mut self, value: f64) -> Self {
        self.rx = Some(value);
        self
    }
    pub fn ry(mut self, value: f64) -> Self {
        self.ry = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "ellipse", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(" {}=\"{}\"", "cx", cx));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(" {}=\"{}\"", "cy", cy));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(path_length) = &self.path_length {
            svg.push_str(&format!(" {}=\"{}\"", "pathLength", path_length));
        }
        if let Some(rx) = &self.rx {
            svg.push_str(&format!(" {}=\"{}\"", "rx", rx));
        }
        if let Some(ry) = &self.ry {
            svg.push_str(&format!(" {}=\"{}\"", "ry", ry));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "ellipse"));
        write!(f, "{}", svg)
    }
}
impl From<Ellipse> for Shape {
    fn from(ellipse: Ellipse) -> Self {
        Self::Ellipse(ellipse)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub element_timing: Option<String>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub path_length: Option<f64>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub stroke: Option<Color>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    pub x1: Option<f64>,
    pub x2: Option<f64>,
    pub y1: Option<f64>,
    pub y2: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Line {}
impl GraphicsElement for Line {}
impl RenderableElement for Line {}
impl ShapeElement for Line {}
impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}
impl Line {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            element_timing: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            path_length: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            stroke: None,
            style: None,
            tab_index: None,
            x1: None,
            x2: None,
            y1: None,
            y2: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn path_length(mut self, value: f64) -> Self {
        self.path_length = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn x1(mut self, value: f64) -> Self {
        self.x1 = Some(value);
        self
    }
    pub fn x2(mut self, value: f64) -> Self {
        self.x2 = Some(value);
        self
    }
    pub fn y1(mut self, value: f64) -> Self {
        self.y1 = Some(value);
        self
    }
    pub fn y2(mut self, value: f64) -> Self {
        self.y2 = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_text_content_child_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + TextContentChildElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_a(mut self, child: A) -> Self {
        self.children.push(child.into());
        self
    }
    pub fn add_child_string(mut self, child: String) -> Self {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "line", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(path_length) = &self.path_length {
            svg.push_str(&format!(" {}=\"{}\"", "pathLength", path_length));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if let Some(x1) = &self.x1 {
            svg.push_str(&format!(" {}=\"{}\"", "x1", x1));
        }
        if let Some(x2) = &self.x2 {
            svg.push_str(&format!(" {}=\"{}\"", "x2", x2));
        }
        if let Some(y1) = &self.y1 {
            svg.push_str(&format!(" {}=\"{}\"", "y1", y1));
        }
        if let Some(y2) = &self.y2 {
            svg.push_str(&format!(" {}=\"{}\"", "y2", y2));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "line"));
        write!(f, "{}", svg)
    }
}
impl From<Line> for Shape {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub element_timing: Option<String>,
    pub fill: Option<Color>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub path_length: Option<f64>,
    pub rx: Option<f64>,
    pub ry: Option<f64>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub stroke: Option<Color>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    pub width: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Rect {}
impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}
impl Rect {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            element_timing: None,
            fill: None,
            height: None,
            id: None,
            inner_html: None,
            nonce: None,
            outer_html: None,
            path_length: None,
            rx: None,
            ry: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            stroke: None,
            style: None,
            tab_index: None,
            width: None,
            x: None,
            y: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn height(mut self, value: f64) -> Self {
        self.height = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn path_length(mut self, value: f64) -> Self {
        self.path_length = Some(value);
        self
    }
    pub fn rx(mut self, value: f64) -> Self {
        self.rx = Some(value);
        self
    }
    pub fn ry(mut self, value: f64) -> Self {
        self.ry = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn width(mut self, value: f64) -> Self {
        self.width = Some(value);
        self
    }
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "rect", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(" {}=\"{}\"", "height", height));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(path_length) = &self.path_length {
            svg.push_str(&format!(" {}=\"{}\"", "pathLength", path_length));
        }
        if let Some(rx) = &self.rx {
            svg.push_str(&format!(" {}=\"{}\"", "rx", rx));
        }
        if let Some(ry) = &self.ry {
            svg.push_str(&format!(" {}=\"{}\"", "ry", ry));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(" {}=\"{}\"", "x", x));
        }
        if let Some(y) = &self.y {
            svg.push_str(&format!(" {}=\"{}\"", "y", y));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "rect"));
        write!(f, "{}", svg)
    }
}
impl From<Rect> for Shape {
    fn from(rect: Rect) -> Self {
        Self::Rect(rect)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub autofocus: Option<bool>,
    pub class_name: Option<String>,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub element_timing: Option<String>,
    pub fill: Option<Color>,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
    pub font_size_adjust: Option<f64>,
    pub id: Option<String>,
    pub inner_html: Option<String>,
    pub length_adjust: Option<String>,
    pub nonce: Option<String>,
    pub outer_html: Option<String>,
    pub rotate: Option<f64>,
    pub scroll_left: Option<f64>,
    pub scroll_top: Option<f64>,
    pub slot: Option<String>,
    pub stroke: Option<Color>,
    pub style: Option<String>,
    pub tab_index: Option<i32>,
    pub text_length: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    children: Vec<Shape>,
}
impl TextContentElement for Text {}
impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Text {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            class_name: None,
            dx: None,
            dy: None,
            element_timing: None,
            fill: None,
            font_family: None,
            font_size: None,
            font_size_adjust: None,
            id: None,
            inner_html: None,
            length_adjust: None,
            nonce: None,
            outer_html: None,
            rotate: None,
            scroll_left: None,
            scroll_top: None,
            slot: None,
            stroke: None,
            style: None,
            tab_index: None,
            text_length: None,
            x: None,
            y: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn class_name(mut self, value: String) -> Self {
        self.class_name = Some(value);
        self
    }
    pub fn dx(mut self, value: f64) -> Self {
        self.dx = Some(value);
        self
    }
    pub fn dy(mut self, value: f64) -> Self {
        self.dy = Some(value);
        self
    }
    pub fn element_timing(mut self, value: String) -> Self {
        self.element_timing = Some(value);
        self
    }
    pub fn fill(mut self, value: Color) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn font_family(mut self, value: String) -> Self {
        self.font_family = Some(value);
        self
    }
    pub fn font_size(mut self, value: String) -> Self {
        self.font_size = Some(value);
        self
    }
    pub fn font_size_adjust(mut self, value: f64) -> Self {
        self.font_size_adjust = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn inner_html(mut self, value: String) -> Self {
        self.inner_html = Some(value);
        self
    }
    pub fn length_adjust(mut self, value: String) -> Self {
        self.length_adjust = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outer_html(mut self, value: String) -> Self {
        self.outer_html = Some(value);
        self
    }
    pub fn rotate(mut self, value: f64) -> Self {
        self.rotate = Some(value);
        self
    }
    pub fn scroll_left(mut self, value: f64) -> Self {
        self.scroll_left = Some(value);
        self
    }
    pub fn scroll_top(mut self, value: f64) -> Self {
        self.scroll_top = Some(value);
        self
    }
    pub fn slot(mut self, value: String) -> Self {
        self.slot = Some(value);
        self
    }
    pub fn stroke(mut self, value: Color) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn style(mut self, value: String) -> Self {
        self.style = Some(value);
        self
    }
    pub fn tab_index(mut self, value: i32) -> Self {
        self.tab_index = Some(value);
        self
    }
    pub fn text_length(mut self, value: f64) -> Self {
        self.text_length = Some(value);
        self
    }
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }
    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }
    pub fn add_child_animation_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + AnimationElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_descriptive_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + DescriptiveElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_text_content_child_element<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + TextContentChildElement,
    {
        self.children.push(child.into());
        self
    }
    pub fn add_child_a(mut self, child: A) -> Self {
        self.children.push(child.into());
        self
    }
    pub fn add_child_string(mut self, child: String) -> Self {
        self.children.push(child.into());
        self
    }
}
impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "text", "",);
        if let Some(autofocus) = &self.autofocus {
            svg.push_str(&format!(" {}=\"{}\"", "autofocus", autofocus));
        }
        if let Some(class_name) = &self.class_name {
            svg.push_str(&format!(" {}=\"{}\"", "className", class_name));
        }
        if let Some(dx) = &self.dx {
            svg.push_str(&format!(" {}=\"{}\"", "dx", dx));
        }
        if let Some(dy) = &self.dy {
            svg.push_str(&format!(" {}=\"{}\"", "dy", dy));
        }
        if let Some(element_timing) = &self.element_timing {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", element_timing));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(font_family) = &self.font_family {
            svg.push_str(&format!(" {}=\"{}\"", "font-family", font_family));
        }
        if let Some(font_size) = &self.font_size {
            svg.push_str(&format!(" {}=\"{}\"", "font-size", font_size));
        }
        if let Some(font_size_adjust) = &self.font_size_adjust {
            svg.push_str(&format!(" {}=\"{}\"", "font-size-adjust", font_size_adjust));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(inner_html) = &self.inner_html {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", inner_html));
        }
        if let Some(length_adjust) = &self.length_adjust {
            svg.push_str(&format!(" {}=\"{}\"", "lengthAdjust", length_adjust));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outer_html) = &self.outer_html {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outer_html));
        }
        if let Some(rotate) = &self.rotate {
            svg.push_str(&format!(" {}=\"{}\"", "rotate", rotate));
        }
        if let Some(scroll_left) = &self.scroll_left {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scroll_left));
        }
        if let Some(scroll_top) = &self.scroll_top {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scroll_top));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(stroke) = &self.stroke {
            svg.push_str(&format!(" {}=\"{}\"", "stroke", stroke));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tab_index) = &self.tab_index {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tab_index));
        }
        if let Some(text_length) = &self.text_length {
            svg.push_str(&format!(" {}=\"{}\"", "textLength", text_length));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(" {}=\"{}\"", "x", x));
        }
        if let Some(y) = &self.y {
            svg.push_str(&format!(" {}=\"{}\"", "y", y));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push('>');
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "text"));
        write!(f, "{}", svg)
    }
}
impl From<Text> for Shape {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}
