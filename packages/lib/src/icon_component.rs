use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> String;
    fn xmlns(&self) -> String;
    fn child_elements(&self) -> LazyNodes;
}

/// Icon component Props
#[derive(PartialEq, Props)]
pub struct IconProps<'a, T: IconShape> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to 20.
    #[props(default = 20)]
    pub height: u32,
    /// The width of the `<svg>` element. Defaults to 20.
    #[props(default = 20)]
    pub width: u32,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor")]
    pub fill: &'a str,
    /// An class for the `<svg>` element.
    #[props(default = "")]
    pub class: &'a str,
    /// An accessible, short-text description for the icon.
    #[props(default = "")]
    pub title: &'a str,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<'a, T: IconShape>(cx: Scope<'a, IconProps<'a, T>>) -> Element<'a> {
    cx.render(rsx! {
        svg {
            stroke: "currentColor",
            stroke_width: "0",
            class: format_args!("{}", cx.props.class),
            height: format_args!("{}", cx.props.height),
            width: format_args!("{}", cx.props.width),
            view_box: format_args!("{}", cx.props.icon.view_box()),
            xmlns: format_args!("{}", cx.props.icon.xmlns()),
            fill: format_args!("{}", cx.props.fill),
            title {
                "{cx.props.title}"
            }
            cx.props.icon.child_elements()
        }
    })
}
