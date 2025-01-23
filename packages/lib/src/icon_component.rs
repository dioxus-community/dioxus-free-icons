use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> &str;
    fn xmlns(&self) -> &str;
    fn child_elements(&self) -> Element;
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
}

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + Clone + PartialEq + 'static> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to 20. Pass None to omit.
    #[props(default = Some(20))]
    pub height: Option<u32>,
    /// The width of the `<svg>` element. Defaults to 20. Pass None to omit.
    #[props(default = Some(20))]
    pub width: Option<u32>,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// An class for the `<svg>` element.
    #[props(default = "".to_string())]
    pub class: String,
    /// An accessible, short-text description for the icon.
    pub title: Option<String>,
    /// The style of the `<svg>` element.
    pub style: Option<String>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let (fill, stroke, stroke_width) = props.icon.fill_and_stroke(&props.fill);
    rsx!(
        svg {
            class: "{props.class}",
            style: props.style,
            height: props.height.map(|height| height.to_string()),
            width: props.width.map(|width| width.to_string()),
            view_box: "{props.icon.view_box()}",
            xmlns: "{props.icon.xmlns()}",
            fill,
            stroke,
            stroke_width,
            stroke_linecap: "{props.icon.stroke_linecap()}",
            stroke_linejoin: "{props.icon.stroke_linejoin()}",
            if let Some(title_text) = props.title {
                title { "{title_text}" }
            }
            {props.icon.child_elements()}
        }
    )
}
