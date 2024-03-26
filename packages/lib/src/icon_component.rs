use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> String;
    fn xmlns(&self) -> String;
    fn child_elements(&self) -> Element;
}

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + Clone + PartialEq + 'static> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to 20.
    #[props(default = 20)]
    pub height: u32,
    /// The width of the `<svg>` element. Defaults to 20.
    #[props(default = 20)]
    pub width: u32,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// An class for the `<svg>` element.
    #[props(default = "".to_string())]
    pub class: String,
    /// An accessible, short-text description for the icon.
    #[props(default = "".to_string())]
    pub title: String,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    rsx!(
        svg {
            stroke_width: "0",
            class: "{props.class}",
            height: "{props.height}",
            width: "{props.width}",
            view_box: "{props.icon.view_box()}",
            xmlns: "{props.icon.xmlns()}",
            fill: "{props.fill}",
            stroke: "{props.fill}",
            title {
                "{props.title}"
            }
            {props.icon.child_elements()}
        }
    )
}
