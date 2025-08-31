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
    AnimateMotion(AnimateMotion),
    Circle(Circle),
    Defs(Defs),
    Desc(Desc),
    Rect(Rect),
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Shape::A(a) => a.to_string(),
            Shape::AnimateMotion(animateMotion) => animateMotion.to_string(),
            Shape::Circle(circle) => circle.to_string(),
            Shape::Defs(defs) => defs.to_string(),
            Shape::Desc(desc) => desc.to_string(),
            Shape::Rect(rect) => rect.to_string(),
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
    pub fn add_child_rect<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + Into<Rect>,
    {
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
        svg.push_str(">");
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
pub struct AnimateMotion {
    pub keyPoints: Option<f64>,
    pub path: Option<String>,
    children: Vec<Shape>,
}
impl AnimationElement for AnimateMotion {}
impl AnimateMotion {
    pub fn new() -> Self {
        Self {
            keyPoints: None,
            path: None,
            children: Vec::new(),
        }
    }
    pub fn keyPoints(mut self, value: f64) -> Self {
        self.keyPoints = Some(value);
        self
    }
    pub fn path(mut self, value: String) -> Self {
        self.path = Some(value);
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
impl std::fmt::Display for AnimateMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut svg = format!(r#"<{}{}"#, "animateMotion", "",);
        if let Some(keyPoints) = &self.keyPoints {
            svg.push_str(&format!(" {}=\"{}\"", "keyPoints", keyPoints));
        }
        if let Some(path) = &self.path {
            svg.push_str(&format!(" {}=\"{}\"", "path", path));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
        for child in self.children.iter() {
            svg.push_str(&child.to_string());
        }
        svg.push_str(&format!("</{}>", "animateMotion"));
        write!(f, "{}", svg)
    }
}
impl From<AnimateMotion> for Shape {
    fn from(animateMotion: AnimateMotion) -> Self {
        Self::AnimateMotion(animateMotion)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub autofocus: Option<bool>,
    pub className: Option<String>,
    pub cx: Option<f64>,
    pub cy: Option<f64>,
    pub elementTiming: Option<String>,
    pub fill: Option<Color>,
    pub id: Option<String>,
    pub innerHtml: Option<String>,
    pub nonce: Option<String>,
    pub outerHtml: Option<String>,
    pub r: Option<f64>,
    pub scrollLeft: Option<f64>,
    pub scrollTop: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tabIndex: Option<i32>,
    pub textContent: Option<String>,
    children: Vec<Shape>,
}
impl BasicShape for Circle {}
impl Circle {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            className: None,
            cx: None,
            cy: None,
            elementTiming: None,
            fill: None,
            id: None,
            innerHtml: None,
            nonce: None,
            outerHtml: None,
            r: None,
            scrollLeft: None,
            scrollTop: None,
            slot: None,
            style: None,
            tabIndex: None,
            textContent: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn className(mut self, value: String) -> Self {
        self.className = Some(value);
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
    pub fn elementTiming(mut self, value: String) -> Self {
        self.elementTiming = Some(value);
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
    pub fn innerHtml(mut self, value: String) -> Self {
        self.innerHtml = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outerHtml(mut self, value: String) -> Self {
        self.outerHtml = Some(value);
        self
    }
    pub fn r(mut self, value: f64) -> Self {
        self.r = Some(value);
        self
    }
    pub fn scrollLeft(mut self, value: f64) -> Self {
        self.scrollLeft = Some(value);
        self
    }
    pub fn scrollTop(mut self, value: f64) -> Self {
        self.scrollTop = Some(value);
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
    pub fn tabIndex(mut self, value: i32) -> Self {
        self.tabIndex = Some(value);
        self
    }
    pub fn textContent(mut self, value: String) -> Self {
        self.textContent = Some(value);
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
        if let Some(className) = &self.className {
            svg.push_str(&format!(" {}=\"{}\"", "className", className));
        }
        if let Some(cx) = &self.cx {
            svg.push_str(&format!(" {}=\"{}\"", "cx", cx));
        }
        if let Some(cy) = &self.cy {
            svg.push_str(&format!(" {}=\"{}\"", "cy", cy));
        }
        if let Some(elementTiming) = &self.elementTiming {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", elementTiming));
        }
        if let Some(fill) = &self.fill {
            svg.push_str(&format!(" {}=\"{}\"", "fill", fill));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(innerHtml) = &self.innerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", innerHtml));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outerHtml) = &self.outerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outerHtml));
        }
        if let Some(r) = &self.r {
            svg.push_str(&format!(" {}=\"{}\"", "r", r));
        }
        if let Some(scrollLeft) = &self.scrollLeft {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scrollLeft));
        }
        if let Some(scrollTop) = &self.scrollTop {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scrollTop));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tabIndex) = &self.tabIndex {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tabIndex));
        }
        if let Some(textContent) = &self.textContent {
            svg.push_str(&format!(" {}=\"{}\"", "textContent", textContent));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
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
    pub className: Option<String>,
    pub elementTiming: Option<String>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub innerHtml: Option<String>,
    pub nonce: Option<String>,
    pub outerHtml: Option<String>,
    pub scrollLeft: Option<f64>,
    pub scrollTop: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tabIndex: Option<i32>,
    pub textContent: Option<String>,
    pub width: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Defs {}
impl Defs {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            className: None,
            elementTiming: None,
            height: None,
            id: None,
            innerHtml: None,
            nonce: None,
            outerHtml: None,
            scrollLeft: None,
            scrollTop: None,
            slot: None,
            style: None,
            tabIndex: None,
            textContent: None,
            width: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn className(mut self, value: String) -> Self {
        self.className = Some(value);
        self
    }
    pub fn elementTiming(mut self, value: String) -> Self {
        self.elementTiming = Some(value);
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
    pub fn innerHtml(mut self, value: String) -> Self {
        self.innerHtml = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outerHtml(mut self, value: String) -> Self {
        self.outerHtml = Some(value);
        self
    }
    pub fn scrollLeft(mut self, value: f64) -> Self {
        self.scrollLeft = Some(value);
        self
    }
    pub fn scrollTop(mut self, value: f64) -> Self {
        self.scrollTop = Some(value);
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
    pub fn tabIndex(mut self, value: i32) -> Self {
        self.tabIndex = Some(value);
        self
    }
    pub fn textContent(mut self, value: String) -> Self {
        self.textContent = Some(value);
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
    pub fn add_child_a<T>(mut self, child: T) -> Self
    where
        T: Into<Shape> + Into<A>,
    {
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
        if let Some(className) = &self.className {
            svg.push_str(&format!(" {}=\"{}\"", "className", className));
        }
        if let Some(elementTiming) = &self.elementTiming {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", elementTiming));
        }
        if let Some(height) = &self.height {
            svg.push_str(&format!(" {}=\"{}\"", "height", height));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(innerHtml) = &self.innerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", innerHtml));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outerHtml) = &self.outerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outerHtml));
        }
        if let Some(scrollLeft) = &self.scrollLeft {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scrollLeft));
        }
        if let Some(scrollTop) = &self.scrollTop {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scrollTop));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tabIndex) = &self.tabIndex {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tabIndex));
        }
        if let Some(textContent) = &self.textContent {
            svg.push_str(&format!(" {}=\"{}\"", "textContent", textContent));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
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
    pub className: Option<String>,
    pub elementTiming: Option<String>,
    pub id: Option<String>,
    pub innerHtml: Option<String>,
    pub nonce: Option<String>,
    pub outerHtml: Option<String>,
    pub scrollLeft: Option<f64>,
    pub scrollTop: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tabIndex: Option<i32>,
    pub textContent: Option<String>,
    children: Vec<Shape>,
}
impl DescriptiveElement for Desc {}
impl Desc {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            className: None,
            elementTiming: None,
            id: None,
            innerHtml: None,
            nonce: None,
            outerHtml: None,
            scrollLeft: None,
            scrollTop: None,
            slot: None,
            style: None,
            tabIndex: None,
            textContent: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn className(mut self, value: String) -> Self {
        self.className = Some(value);
        self
    }
    pub fn elementTiming(mut self, value: String) -> Self {
        self.elementTiming = Some(value);
        self
    }
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn innerHtml(mut self, value: String) -> Self {
        self.innerHtml = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outerHtml(mut self, value: String) -> Self {
        self.outerHtml = Some(value);
        self
    }
    pub fn scrollLeft(mut self, value: f64) -> Self {
        self.scrollLeft = Some(value);
        self
    }
    pub fn scrollTop(mut self, value: f64) -> Self {
        self.scrollTop = Some(value);
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
    pub fn tabIndex(mut self, value: i32) -> Self {
        self.tabIndex = Some(value);
        self
    }
    pub fn textContent(mut self, value: String) -> Self {
        self.textContent = Some(value);
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
        if let Some(className) = &self.className {
            svg.push_str(&format!(" {}=\"{}\"", "className", className));
        }
        if let Some(elementTiming) = &self.elementTiming {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", elementTiming));
        }
        if let Some(id) = &self.id {
            svg.push_str(&format!(" {}=\"{}\"", "id", id));
        }
        if let Some(innerHtml) = &self.innerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", innerHtml));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outerHtml) = &self.outerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outerHtml));
        }
        if let Some(scrollLeft) = &self.scrollLeft {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scrollLeft));
        }
        if let Some(scrollTop) = &self.scrollTop {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scrollTop));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tabIndex) = &self.tabIndex {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tabIndex));
        }
        if let Some(textContent) = &self.textContent {
            svg.push_str(&format!(" {}=\"{}\"", "textContent", textContent));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
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
pub struct Rect {
    pub autofocus: Option<bool>,
    pub className: Option<String>,
    pub elementTiming: Option<String>,
    pub fill: Option<Color>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub innerHtml: Option<String>,
    pub nonce: Option<String>,
    pub outerHtml: Option<String>,
    pub scrollLeft: Option<f64>,
    pub scrollTop: Option<f64>,
    pub slot: Option<String>,
    pub style: Option<String>,
    pub tabIndex: Option<i32>,
    pub textContent: Option<String>,
    pub width: Option<f64>,
    pub x: Option<f64>,
    children: Vec<Shape>,
}
impl BasicShape for Rect {}
impl Rect {
    pub fn new() -> Self {
        Self {
            autofocus: None,
            className: None,
            elementTiming: None,
            fill: None,
            height: None,
            id: None,
            innerHtml: None,
            nonce: None,
            outerHtml: None,
            scrollLeft: None,
            scrollTop: None,
            slot: None,
            style: None,
            tabIndex: None,
            textContent: None,
            width: None,
            x: None,
            children: Vec::new(),
        }
    }
    pub fn autofocus(mut self, value: bool) -> Self {
        self.autofocus = Some(value);
        self
    }
    pub fn className(mut self, value: String) -> Self {
        self.className = Some(value);
        self
    }
    pub fn elementTiming(mut self, value: String) -> Self {
        self.elementTiming = Some(value);
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
    pub fn innerHtml(mut self, value: String) -> Self {
        self.innerHtml = Some(value);
        self
    }
    pub fn nonce(mut self, value: String) -> Self {
        self.nonce = Some(value);
        self
    }
    pub fn outerHtml(mut self, value: String) -> Self {
        self.outerHtml = Some(value);
        self
    }
    pub fn scrollLeft(mut self, value: f64) -> Self {
        self.scrollLeft = Some(value);
        self
    }
    pub fn scrollTop(mut self, value: f64) -> Self {
        self.scrollTop = Some(value);
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
    pub fn tabIndex(mut self, value: i32) -> Self {
        self.tabIndex = Some(value);
        self
    }
    pub fn textContent(mut self, value: String) -> Self {
        self.textContent = Some(value);
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
        if let Some(className) = &self.className {
            svg.push_str(&format!(" {}=\"{}\"", "className", className));
        }
        if let Some(elementTiming) = &self.elementTiming {
            svg.push_str(&format!(" {}=\"{}\"", "elementTiming", elementTiming));
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
        if let Some(innerHtml) = &self.innerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "innerHtml", innerHtml));
        }
        if let Some(nonce) = &self.nonce {
            svg.push_str(&format!(" {}=\"{}\"", "nonce", nonce));
        }
        if let Some(outerHtml) = &self.outerHtml {
            svg.push_str(&format!(" {}=\"{}\"", "outerHtml", outerHtml));
        }
        if let Some(scrollLeft) = &self.scrollLeft {
            svg.push_str(&format!(" {}=\"{}\"", "scrollLeft", scrollLeft));
        }
        if let Some(scrollTop) = &self.scrollTop {
            svg.push_str(&format!(" {}=\"{}\"", "scrollTop", scrollTop));
        }
        if let Some(slot) = &self.slot {
            svg.push_str(&format!(" {}=\"{}\"", "slot", slot));
        }
        if let Some(style) = &self.style {
            svg.push_str(&format!(" {}=\"{}\"", "style", style));
        }
        if let Some(tabIndex) = &self.tabIndex {
            svg.push_str(&format!(" {}=\"{}\"", "tabIndex", tabIndex));
        }
        if let Some(textContent) = &self.textContent {
            svg.push_str(&format!(" {}=\"{}\"", "textContent", textContent));
        }
        if let Some(width) = &self.width {
            svg.push_str(&format!(" {}=\"{}\"", "width", width));
        }
        if let Some(x) = &self.x {
            svg.push_str(&format!(" {}=\"{}\"", "x", x));
        }
        if self.children.is_empty() {
            svg.push_str("/>");
            return write!(f, "{}", svg);
        }
        svg.push_str(">");
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
