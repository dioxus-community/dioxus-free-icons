use dioxus::prelude::*;

/// Icon data structure
#[derive(Clone, Debug, PartialEq)]
pub struct Icon<'a> {
    pub view_box: &'a str,
    pub xmlns: &'a str,
    pub d: &'a str,
}

/// Icon component Props
#[derive(PartialEq, Props)]
pub struct IconProps<'a> {
    /// An optional class for the `<svg>` element.
    #[props(default)]
    pub class: Option<&'static str>,
    /// The size of the `<svg>` element. All the heroicons are square, so this
    /// will be turned into the `height` and `width` attributes for the
    /// `<svg>`. Defaults to 20.
    #[props(default = 20)]
    pub size: u32,
    /// The color to use for filling the icon. This is only relevant for solid
    /// icons. Defaults to "currentColor".
    #[props(default = "currentColor")]
    pub fill: &'a str,
    /// The icon shape to use.
    pub icon: Icon<'a>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        svg {
            class: format_args!("{}", cx.props.class.unwrap_or("")),
            height: format_args!("{}", cx.props.size),
            width: format_args!("{}", cx.props.size),
            view_box: format_args!("{}", cx.props.icon.view_box),
            fill: format_args!("{}", cx.props.fill),
            path {
                d: format_args!("{}", cx.props.icon.d),
            },
        }
    })
}
