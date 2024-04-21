use dioxus::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn view_box(&self) -> &str;
    fn xmlns(&self) -> &str;
    fn child_elements(&self) -> Element;
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "0")
    }
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
    pub title: Option<String>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let (fill, stroke, stroke_width) = props.icon.fill_and_stroke(&props.fill);
    rsx!(
        svg {
            class: "{props.class}",
            height: "{props.height}",
            width: "{props.width}",
            view_box: "{props.icon.view_box()}",
            xmlns: "{props.icon.xmlns()}",
            fill,
            stroke,
            stroke_width,
            if let Some(title_text) = props.title {
                title {
                    "{title_text}"
                }
            },
            {props.icon.child_elements()}
        }
    )
}
