use core::str;

use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> &str;
    fn width(&self) -> &str;
    fn height(&self) -> &str;
    fn xmlns(&self) -> &str;
    fn child_elements(&self) -> Element;
    fn fill(&self) -> &str;
    fn stroke(&self) -> &str;
    fn stroke_width(&self) -> &str;
    fn stroke_linecap(&self) -> &str;
    fn stroke_linejoin(&self) -> &str;
}

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + Clone + PartialEq + 'static> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to the icon's default height.
    #[props(default = 0)]
    pub height: u32,
    /// The width of the `<svg>` element. Defaults to the icon's default width.
    #[props(default = 0)]
    pub width: u32,
    /// The color to use for filling the icon. Defaults to the icon's default fill color.
    #[props(default = None)]
    pub fill: Option<String>,
    /// The color to use for strokeing the icon. Defaults to the icon's default stroke color.
    #[props(default = None)]
    pub stroke: Option<String>,
    /// The width of the stroke. Defaults to the icon's default stroke width.
    #[props(default = None)]
    pub stroke_width: Option<u32>,
    /// The linecap style of the stroke. Defaults to the icon's default stroke linecap.
    #[props(default = None)]
    pub stroke_linecap: Option<String>,
    /// The linejoin style of the stroke. Defaults to the icon's default stroke linejoin.
    #[props(default = None)]
    pub stroke_linejoin: Option<String>,
    /// An class for the `<svg>` element.
    #[props(default = "".to_string())]
    pub class: String,
    /// An accessible, short-text description for the icon.
    pub title: Option<String>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {

    let width = if props.width == 0 { props.icon.width() } else { &props.width.to_string() };
    let height = if props.height == 0 { props.icon.height() } else { &props.height.to_string() };
    let fill = props.fill.unwrap_or(props.icon.fill().to_string());
    let stroke = props.stroke.unwrap_or(props.icon.stroke().to_string());
    let stroke_width = props.stroke_width.map(|v| v.to_string()).unwrap_or(props.icon.stroke_width().to_string());
    let stroke_linecap = props.stroke_linecap.unwrap_or(props.icon.stroke_linecap().to_string());
    let stroke_linejoin = props.stroke_linejoin.unwrap_or(props.icon.stroke_linejoin().to_string());

    rsx!(
        svg {
            class: "{props.class}",
            height,
            width,
            view_box: "{props.icon.view_box()}",
            xmlns: "{props.icon.xmlns()}",
            fill,
            stroke,
            stroke_width,
            stroke_linecap,
            stroke_linejoin,
            if let Some(title_text) = props.title {
                title {
                    "{title_text}"
                }
            },
            {props.icon.child_elements()},
        }
    )
}
