use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md10mp;
impl IconShape for Md10mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 7H15v3h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H13c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm-1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md11mp;
impl IconShape for Md11mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM11 5.5v6H9.5V7H8V5.5h3zm5 0v6h-1.5V7H13V5.5h3zm-.5 8.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md12mp;
impl IconShape for Md12mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zM15.5 9h-2v1h3v1.5H12V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm0 5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md13mp;
impl IconShape for Md13mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H12V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm-1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md14mp;
impl IconShape for Md14mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm7.5 4.5h-1v1.5H15V10h-3V5.5h1.5v3H15v-3h1.5v3h1V10zm-2 4H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md15mp;
impl IconShape for Md15mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zM16.5 7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H12V10h3V9h-3V5.5h4.5V7zm-1 7H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md16mp;
impl IconShape for Md16mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5 9H15v1.5h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm3 6c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5V7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H13zm2.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md17mp;
impl IconShape for Md17mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm5 6h-1.75L14.62 7H12V5.5h3.5c.67 0 1.15.65.96 1.29L15 11.5zm.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md18mp;
impl IconShape for Md18mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H13c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm-3 0H15V9h-1.5v1.5zm0-2.5H15V6.5h-1.5V8zm2 6H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8y8aC4Bf80Ldhx4KZ6/ZwC38sK9NHCMfz3xgiH+ZeZF8zvAa/OCvTbwWzyvvwFeG9jlhTsO/DbwUvzriBcM8S8zL5rfAV6bF+y1gd/ieb0M8Ne8aF4a+Cv+dcQLhviXmRfN7wCvzQv22sBv8bzEc/otntPr8JzMv454wRD/MvOi+Wvgo3m2S8Bf82yvDfwWz0s8J/OcxHMy/zriBUP8y8y/ze8Ar82zvTbwWzwv8ZzMcxLPyfzriBcM8S8z/za/A7w2z/bawG/xvMRzMs9JPCfzryNeMMS/zPzb/A7w2jzbawO/xfMSz+m1eU6/zXMy/zriBUP8y8y/ze8Ar82zvTbwWzyv9wG+mxed+dcRLxjiX2b+bX4HeG2e7bWB3+J57QIfDXwPLxrzryNeMMS/zPzb/A7w2jzbawO/xb+eeE7mX0e8YIh/mfm3+R3gtXm21wZ+i3898ZzMv454wRD/MvNv8zvAa/Nsrw38Fs/rd3jhXpvnZP51xAuG+JeZf5vfAV6bZ3tt4Ld4XuJfx/zriBcM8S8z/za/A7w2z/bawG/xvMSL7sHA0/nXES8Y4l9m/m1+B3htnu21gd/ieYkX3WcDn8W/jnjBEP8y82/z18BH82wvDXw1z+u1+Zc9GHhr4K351xMvGOJfZv53Ey8Y4l9m/ncTLxjiX2ZesEvAdwM/zb/Og4G3Bt6K5+9jgL/mX/bWwHsDx3jBxAuG+JeZF+xlgL/m3+6zgc/ieb0O8Nu8aF4a+CteMPGCIf5l5vn7HeC1+fd5MPB0ntfrAL/Ni+63gdfi+RMvGOJfZp6/3wFem38/87xeB/htXnS/DbwWz594wRD/MvP8/Q7w2jzbSwNfxRUfA/w1z/bVwEtxxevwnMzzeh3gt3m2rwZeimf7GOCvebbfBl6L50+8YIh/mXn+fgd4bZ7ttYHf4orXAX6bZ/tt4LW4Qjwn87xeB/htnu23gdfi2V4H+G2e7beB1+L5Ey8Y4l9mnr/fAV6bZ3tt4Le44nWA3+bZfht4La4Qz8k8r9cBfptn+23gtXi21wF+m2f7beC1eP7EC4b4l5nn73eA1+bZXhv4La54HeC3ebbfBl6LK8RzMs/rdYDf5tl+G3gtnu11gN/m2X4beC2eP/GCIf5l5vn7HeC1ebbXBn6LK14H+G2e7beB1+IK8ZzM83od4Ld5tpcGjvNsfw3s8my/DbwWz594wRD/MvP8/Q7w2jzbawO/xRWvA/w2z/bbwGtxhXhO5nm9DvDbvOh+G3gtnj/xgiH+Zeb5+x3gtXm21wZ+iyteB/htnu23gdfiCvGczPN6HeC3edH9FfDSPH/iBUP8y8zz9zvAa/Nsrw38Fle8DvDbPNtvA6/FFeI5mef1OsBv86L5LOCzecHEC4b4l5nn73eA1+bZXhv4La74a2CXZ3tp4DhXiOdkntfrAL/Ns3018FI8r+PAS/PCiRcM8S8zz9/vAK/Ns7028Fv8y8RzMs/rdYDf5tl+G3gt/m3EC4b4l5nn73eA1+bZXhv4Lf5l4jmZ5/U6wG/zbL8NvBb/NuIFQ/zLzPP3O8Br82yvDfwW/zLxnMzzeh3gt3m23wZei38b8YIh/mXm+dsF/ppnOw68NP+y3+Y5vTbP66+BXZ7tpYHj/NuIFwzxLzP/u4kXDPEv2wWO8b/TJeA4LxjiX/bdwHvxv9P3AO/NC4b4lz0YeDr/Oz0EuJUXDPGieW/gu/jf5X2A7+aFQ7zoHgx8NvDWwDH+Z7oE/DTw2cCt/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ec6B+5BORw6WAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md19mp;
impl IconShape for Md19mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 7h3V9h-2c-.55 0-1-.45-1-1V6.5c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4c0 .55-.45 1-1 1H12V10zm1.5-2H15V6.5h-1.5V8zM7 5.5h3v6H8.5V7H7V5.5zm5 13h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm6.5-2.5c0 .55-.45 1-1 1h-2v1.5H14v-6h3.5c.55 0 1 .45 1 1V16zm-3-2H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md20mp;
impl IconShape for Md20mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 7H16v3h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm2-8c0 .55-.45 1-1 1H14c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm5.5 5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md21mp;
impl IconShape for Md21mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM11 9H9v1h3v1.5H7.5V9c0-.55.45-1 1-1h2V7h-3V5.5H11c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm3-3.5h3v6h-1.5V7H14V5.5zm1.5 8.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md22mp;
impl IconShape for Md22mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm6.5 0h-2v1h3v1.5H13V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm-1 5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md23mp;
impl IconShape for Md23mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm7.5 1.5c0 .55-.45 1-1 1H13V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm-2 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md24mp;
impl IconShape for Md24mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm8.5 1h-1v1.5H16V10h-3V5.5h1.5v3H16v-3h1.5v3h1V10zm-3 4H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFy0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z8x7gE/DUvmtfiP454wRD/MvMf43eA1+ZF89LAbwPH+PcTLxjiX2b+Y/wO8Nq86L4a+Cj+/cQLhviXmf8YvwO8Ni+6twZ+in8/8YIh/mXmP8ZfAx/NC3YJ+Gue7bWB3+LfT7xgiH+Z+a/xO8Br82yvDfwW/37iBUP8y8x/jd8BXptne23gt/j3Ey8Y4l9m/uNcAr4b+Gme1y7w1zzbewPfxb+feMEQ/zLzH+dlgL/mRfNTwFvz7ydeMMS/zPzH+B3gtfmXPRh4L+Cz+Y8hXjDEv8z8x/gd4LV5ttcGfov/fOIFQ/zLzH+M3wFem2d7beC3+M8nXjDEv8z8x/gd4LV5tpcGvpoX7DjwUvz7iRcM8S8z/zF+B3htXnSvDfwW/37iBUP8y8x/jN8BXpsX3WsDv8W/n3jBEP8y8x/jd4DX5kX32sBv8e8nXjDEv8z8x/hr4KN50b008NX8+4kXDPEvM/+7iRcM8S8z/7uJFwzxLzMv2CXgu4Gf5l/nwcBbA2/F8/cxwF/zL3tr4L2BY7xg4gVD/MvMC/YywF/zb/fZwGfxvF4H+G1eNC8N/BUvmHjBEP8y8/z9DvDa/Ps8GHg6z+t1gN/mRffbwGvx/IkXDPEvM8/f7wCvzb+feV6vA/w2L7rfBl6L50+8YIh/mXn+fgd4bZ7tpYGv4oqPAf6aZ/tq4KW44nV4TuZ5vQ7w2zzbVwMvxbN9DPDXPNtvA6/F8ydeMMS/zDx/vwO8Ns/22sBvccXrAL/Ns/028FpcIZ6TeV6vA/w2z/bbwGvxbK8D/DbP9tvAa/H8iRcM8S8zz9/vAK/Ns7028Ftc8TrAb/Nsvw28FleI52Se1+sAv82z/TbwWjzb6wC/zbP9NvBaPH/iBUP8y8zz9zvAa/Nsrw38Fle8DvDbPNtvA6/FFeI5mef1OsBv82y/DbwWz/Y6wG/zbL8NvBbPn3jBEP8y8/z9DvDaPNtrA7/FFa8D/DbP9tvAa3GFeE7meb0O8Ns820sDx3m2vwZ2ebbfBl6L50+8YIh/mXn+fgd4bZ7ttYHf4orXAX6bZ/tt4LW4Qjwn87xeB/htXnS/DbwWz594wRD/MvP8/Q7w2jzbawO/xRWvA/w2z/bbwGtxhXhO5nm9DvDbvOj+Cnhpnj/xgiH+Zeb5+x3gtXm21wZ+iyteB/htnu23gdfiCvGczPN6HeC3edF8FvDZvGDiBUP8y8zz9zvAa/Nsrw38Flf8NbDLs700cJwrxHMyz+t1gN/m2b4aeCme13HgpXnhxAuG+JeZ5+93gNfm2V4b+C3+ZeI5mef1OsBv82y/DbwW/zbiBUP8y8zz9zvAa/Nsrw38Fv8y8ZzM83od4Ld5tt8GXot/G/GCIf5l5vn7HeC1ebbXBn6Lf5l4TuZ5vQ7w2zzbbwOvxb+NeMEQ/zLz/O0Cf82zHQdemn/Zb/OcXpvn9dfALs/20sBx/m3EC4b4l5n/3cQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8yx4MPJ3/nR4C3MoLhnjRvDfwXfzv8j7Ad/PCIV50DwY+G3hr4Bj/M10Cfhr4bOBW/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMPY9NBEW1tmgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md2mp;
impl IconShape for Md2mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-2-9.5h-2v1h3v1.5H10V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm2 5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md3mp;
impl IconShape for Md3mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H10V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md4mp;
impl IconShape for Md4mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3-8.5h-1v1.5h-1.5V10h-3V5.5H11v3h1.5v-3H14v3h1V10zm.5 8.5H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm0-4.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md5mp;
impl IconShape for Md5mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM14.5 7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H10V10h3V9h-3V5.5h4.5V7zm1 7H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md6mp;
impl IconShape for Md6mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.5 9H13v1.5h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm-1-7c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5V7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H11zm4.5 7H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm0-4.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md7mp;
impl IconShape for Md7mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-2.5-7h-1.75L12.62 7H10V5.5h3.5c.67 0 1.15.65.96 1.29L13 11.5zm2.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGHklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z8x/obYJcX7rX4jyNeMMS/zPzH+Rzgs/mXvTTw28Ax/v3EC4b4l5n/GH8DvDQvuq8GPop/P/GCIf5l5j/G7wCvzbO9NPBVPNvfAB/Ns7018FP8+4kXDPEvM/8xfgd4bZ7ttYHf4tl+B3htnu21gd/i30+8YIh/mfmP8TvAa/Nsrw38Fs/2O8Br82yvDfwW/37iBUP8y8x/jN8BXptne23gt3i23wFem2d7beC3+PcTLxjiX2b+Y/wO8No823HgpXm2XeCvebbXBn6Lfz/xgiH+ZeY/xu8Ar82L7rWB3+LfT7xgiH+Z+Y/xO8Br86J7beC3+PcTLxjiX2b+Y/wO8No822sDv8Wz/Q7w2jzbawO/xb+feMEQ/zLzH+N3gNfm2V4b+C2e7XeA1+bZXhv4Lf79xAuG+JeZ/xi/A7w2z/bawG/xbL8DvDbP9trAb/HvJ14wxL/M/Mf4HeC1ebbXBn6LZ/sd4LV5ttcGfot/P/GCIf5l5j/G7wCvzYvuo4Gv4t9PvGCIf5n5j/MywF/zovkr4KX59xMvGOJfZv7j7ALfDfw2sMvz99LAewMvzX8M8YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5gW7BHw38NP86zwYeGvgrXj+Pgb4a/5lbw28N3CMF0y8YIh/mXnBXgb4a/7tPhv4LJ7X6wC/zYvmpYG/4gUTLxjiX2aev98BXpt/nwcDT+d5vQ7w27zofht4LZ4/8YIh/mXm+fsd4LX59zPP63WA3+ZF99vAa/H8iRcM8S8zz9/vAK/Ns7008FVc8THAX/NsXw28FFe8Ds/JPK/XAX6bZ/tq4KV4to8B/ppn+23gtXj+xAuG+JeZ5+93gNfm2V4b+C2ueB3gt3m23wZeiyvEczLP63WA3+bZfht4LZ7tdYDf5tl+G3gtnj/xgiH+Zeb5+x3gtXm21wZ+iyteB/htnu23gdfiCvGczPN6HeC3ebbfBl6LZ3sd4Ld5tt8GXovnT7xgiH+Zef5+B3htnu21gd/iitcBfptn+23gtbhCPCfzvF4H+G2e7beB1+LZXgf4bZ7tt4HX4vkTLxjiX2aev98BXptne23gt7jidYDf5tl+G3gtrhDPyTyv1wF+m2d7aeA4z/bXwC7P9tvAa/H8iRcM8S8zz9/vAK/Ns7028Ftc8TrAb/Nsvw28FleI52Se1+sAv82L7reB1+L5Ey8Y4l9mnr/fAV6bZ3tt4Le44nWA3+bZfht4La4Qz8k8r9cBfpsX3V8BL83zJ14wxL/MPH+/A7w2z/bawG9xxesAv82z/TbwWlwhnpN5Xq8D/DYvms8CPpsXTLxgiH+Zef5+B3htnu21gd/iir8Gdnm2lwaOc4V4TuZ5vQ7w2zzbVwMvxfM6Drw0L5x4wRD/MvP8/Q7w2jzbawO/xb9MPCfzvF4H+G2e7beB1+LfRrxgiH+Zef5+B3htnu21gd/iXyaek3lerwP8Ns/228Br8W8jXjDEv8w8f78DvDbP9trAb/EvE8/JPK/XAX6bZ/tt4LX4txEvGOJfZp6/XeCvebbjwEvzL/ttntNr87z+Gtjl2V4aOM6/jXjBEP8y87+beMEQ/7Jd4Bj/O10CjvOCIf5l3w28F/87fQ/w3rxgiH/Zg4Gn87/TQ4BbecEQL5r3Br6L/13eB/huXjjEi+7BwGcDbw0c43+mS8BPA58N3Mq/DPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BJU84kHWEhkGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md8mp;
impl IconShape for Md8mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.5 9H13v1.5h-1.5zm0-2.5H13V8h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H11c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md9mp;
impl IconShape for Md9mp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.5 6.5H13V8h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H10V10h3V9h-2c-.55 0-1-.45-1-1V6.5c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddAPhoto;
impl IconShape for MdAddAPhoto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3,4V1h2v3h3v2H5v3H3V6H0V4H3z M6,10V7h3V4h7l1.83,2H21c1.1,0,2,0.9,2,2v12c0,1.1-0.9,2-2,2H5c-1.1,0-2-0.9-2-2V10H6z M13,19c2.76,0,5-2.24,5-5s-2.24-5-5-5s-5,2.24-5,5S10.24,19,13,19z M9.8,14c0,1.77,1.43,3.2,3.2,3.2s3.2-1.43,3.2-3.2 s-1.43-3.2-3.2-3.2S9.8,12.23,9.8,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFD0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O3018FI8p78BPpp/HcT/Tr8NvBbP6XeA1+ZfB/G/028Dr8Vz+h3gtfnXQfzv9NvAa/Gcfgd4bf51EP87/TbwWjyn3wFem38dxP9Ovw28Fs/pd4DX5l8H8b/TbwOvxXP6HeC1+ddB/O/028Br8Zx+B3ht/nUQ/zpvBbw08Nr8x/sb4KO54quBl+IFe2ngOM9pF/hrXrC/AT6a54R40TwY+CngpfnP8zvAa3PFbwOvxX+s3wFem+eE+JcdB/4KeDD/uX4HeG2u+G3gtfiP9TvAa/OcEP+yzwY+i/98vwO8Nlf8NvBa/Mf6HeC1eU6If9nTgQfzn+93gNfmit8GXov/WL8DvDbPCfEvM/81fgd4ba74auClecFeGjjGc7oE/DUv2F8DH81zQvzLzH+N3wFemxfNbwOvxXP6HeC1+ddB/MvMf43fAV6bF81vA6/Fc/od4LX510H8y8x/jV3gr3nB/gb4aK74beC1eE6/A7w2/zqIf5n5n+F3gNfmit8GXovn9DvAa/Ovg/iXmf8Zfgd4ba74beC1eE6/A7w2/zqIf5n5n+F3gNfmit8GXovn9DvAa/Ovg/iXmf8Zfgd4ba74beC1eE6/A7w2/zqIf5n5n+F3gNfmiq8GXprn9NfAR/Ovg/iXmf8Zfgd4bf5jIf5l5n+G3wFem/9YiH+Z+Z9hF/hrXrC/AT6afx3Ev8z87/A7wGvzr4P4l5n/HX4HeG3+dRD/MvO/w+8Ar82/DuJfZv53+B3gtfnXQfzLzP8OvwO8Nv86iH+Z+d/hd4DX5l8H8S8z/zv8DvDa/Osg/mXmv8bPAG/Fv93vAK/Nvw7iX2b+8/0N8NLAewPfxb/N7wCvzb8O4l9m/nNdAl4auJUrvht4L/71fgd4bf51EP8y86/zDOCzge/iRfM2wE/znP4aeCn+dX4HeG3+dRD/MvOv8zLAXwM/DbwVL9zXAB/N8zoO3Aoc40X3O8Br86+D+JeZF93HAF/NFceBW4FjPH9/A7w0L9hLA3/Fi+53gNfmXwfxLzMvmp8B3prn9NbAT/G8LgEvDdzKC/fewHfxovkd4LX510H8y8y/7BnASwO7PK+fBt6K5/Q2wE/zovlu4L34l/0O8Nr86yD+ZeZf9jLAX/P8HQduBY5xxdcAH82/zl8DL8UL9zvAa/Ovg/iXmRfuY4Cv5oV7a+CngL8BXpp/vePArcAxXrDfAV6bfx3Ev8y8YD8DvDUvmu8GPhu4lX+blwb+ihfsd4DX5l8H8S8zz98zgJcGdvmv897Ad/H8/Q7w2vzrIP5l5vl7GeCv+a/33cB78bx+B3ht/nUQ/zLzvL4b+G7++/w0cJzn9DvAa/Ovg/iX7QLH+J/vc4DP5l8H8S/7buC9+J/vbYCf5l8H8S97MPB0/mf7G+Cl+ddDvGjeG/gu/mf6G+CtgVv510O86B4MfDbw1sAx/vs9A/hu4KuBXf5tEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHTE+8QTdgMWQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddPhotoAlternate;
impl IconShape for MdAddPhotoAlternate {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 7v2.99s-1.99.01-2 0V7h-3s.01-1.99 0-2h3V2h2v3h3v2h-3zm-3 4V8h-3V5H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-8h-3zM5 19l3-4 2 3 3-4 4 5H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddToPhotos;
impl IconShape for MdAddToPhotos {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-1 9h-4v4h-2v-4H9V9h4V5h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAdjust;
impl IconShape for MdAdjust {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3-8c0 1.66-1.34 3-3 3s-3-1.34-3-3 1.34-3 3-3 3 1.34 3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAnimation;
impl IconShape for MdAnimation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 2c-2.71 0-5.05 1.54-6.22 3.78-1.28.67-2.34 1.72-3 3C3.54 9.95 2 12.29 2 15c0 3.87 3.13 7 7 7 2.71 0 5.05-1.54 6.22-3.78 1.28-.67 2.34-1.72 3-3C20.46 14.05 22 11.71 22 9c0-3.87-3.13-7-7-7zM9 20c-2.76 0-5-2.24-5-5 0-1.12.37-2.16 1-3 0 3.87 3.13 7 7 7-.84.63-1.88 1-3 1zm3-3c-2.76 0-5-2.24-5-5 0-1.12.37-2.16 1-3 0 3.86 3.13 6.99 7 7-.84.63-1.88 1-3 1zm4.7-3.3c-.53.19-1.1.3-1.7.3-2.76 0-5-2.24-5-5 0-.6.11-1.17.3-1.7.53-.19 1.1-.3 1.7-.3 2.76 0 5 2.24 5 5 0 .6-.11 1.17-.3 1.7zM19 12c0-3.86-3.13-6.99-7-7 .84-.63 1.87-1 3-1 2.76 0 5 2.24 5 5 0 1.12-.37 2.16-1 3z",
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEs0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Om8FvDTw2vzP9NvAXwM/w4sG8aJ5MPBTwEvzv8NfA28D3MoLh/iXHQf+Cngw/7v8NfA6wC4vGOJf9tnAZ/G/0+cAn80LhviXPR14MP873Qo8hBcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/3mewRUP4j+PeMEQ/zLzn+dzuOKz+M8jXjDEv8z857gEPJgrbgWO8Z9DvGCIf5n5z/E1wEdzxVcDH8V/DvGCIf5l5j/HQ4BbueLBwNP5zyFeMMS/zPzH+x7gvXlO3w28F//xxAuG+JeZ/3gPAW7lOb008Ff8xxMvGOJfZv5j/Q7w2jx/vw28Fv+xxAuG+JeZ/1ivA/w2z99rA7/FfyzxgiH+ZeY/zt8AL80L99fAS/EfR7xgiH+Z+Y/zPsB388K9N/Bd/McRLxjiX2b+/S4BXw18Ni+azwY+GjjGv594wRD/MvNv9wzgq4HvBnb51zkOvDfw0cCD+LcTLxjiX2b+9Z4BfDbw3fzHeG/gs4EH8a8nXjDEv8y86H4H+Gzgt/nP8drAZwOvxYtOvGCIf5n5l30P8NXAX/Nf46WBjwbei3+ZeMEQ/zLzwn018DnALv+1jgOfBXw0L5x4wRD/MvMv2wW+Gvge4Fb+cz0YeC/go4Hj/MvEC4b4l5l/ne8GPge4lf9YDwY+C3hv/nXEC4b4l5l/m58Gvgb4bf59Xhv4LOC1+bcRLxjiX2b+fX4b+G7ge/jXeS/gvYHX5t9HvGCIf5n5j3Er8DLALi/cceCvgAfzH0O8YIh/mfmP8znAZ/PCfTbwWfzHES8Y4l9m/uPsAg8Bdnn+jgNPB47zH0e8YIh/mfmP9T7Ad/P8vTfwXfzHEi8Y4l9m/mPdCjyE5+/pwIP5jyVeMMS/zPzHex/gu3lO7w18F//xxAuG+JeZ/3i/DbwOz+mvgJfmP554wRD/MvOf43WA3+aK1wZ+i/8c4gVD/MvMf47fBl6HK34LeG3+c4gXDPEvM/95HsIVT+c/j3jBEP8y85/ne7jivfjPI14wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv+yvgZfif6ffAV6bFwzxL3tv4Lv43+l9gO/mBUO8aL4beC/+d/ke4L154RAvus8GPho4xv9sl4CvBj6bfxniX++1+df5auCl+Lf5G+Cj+df5bV50iP98x4HfBl6Kf52/AV4b2OU/D+K/xnHgt4GX4kXzN8BrA7v850L81zkO/DbwUrxwfwO8NrDLfz7Ef63jwG8DL8Xz9zfAawO7/NdA/Nc7Dvw28FI8p78BXhvY5b8O4r/HceC3gZfiir8BXhvY5b8W4r/PceC3ueK1gV3+6yH+ex3nil3+eyD+f0P8/8Y/Aqmvn0Hn5SAvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssistant;
impl IconShape for MdAssistant {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5.12 10.88L12 17l-1.88-4.12L6 11l4.12-1.88L12 5l1.88 4.12L18 11l-4.12 1.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADyElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP85/kbYJf/fIh/m98GXov/PJ8DfDb/+RD/Nr8NvBb/eX4GeGv+8yH+bX4beC3+89wKPIT/fIh/m98GXov/XCeAXf5zIf5tfht4Lf5zvQ7w2/znQvzb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWjynvwE+mn+b1wY+i/84rwP8Nv8yxL/NbwOvxXP6HeC1+bd5aeCv+I/zOsBv8y9D/Nv8NvBaPKffAV6bfzvzH+d1gN/mX4b4t/lt4LV4Tr8DvDb/dr8NvBb/MV4H+G3+ZYh/m98GXovn9DvAa/Nv99XAR/Ef43WA3+Zfhvi3+W3gtXhOvwO8Nv92bw18NP8xPhr4a/5liH+b3wZei+f0O8Br878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE6/A7w2/7sg/m1+G3gtntPvAK/N/y6If5vfBl6L5/Q7wGvzvwvi3+a3gdfiOf0O8Nr874L4t/lt4LV4Tr8DvDb/uyD+bX4beC2e0+8Ar83/Loh/m98GXovn9DvAa/O/C+Lf5reB1+I5/Q7w2vzvgvi3+W3gtXhOvwO8Nv+7IP5tfht4LZ7T7wCvzf8uiH+b3wZei+f0O8Br878L4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IPg1oQSr5DioAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssistantPhoto;
impl IconShape for MdAssistantPhoto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b472f+Y/wO8Nr86yD++5n/GL8DvDb/Ooj/fuY/xu8Ar82/DuK/n/mP8TvAa/Ovg/jvZ/5j/A7w2vzrIP77mf8YvwO8Nv86iP9+5j/G7wCvzb8O4r+f+Y/xO8Br86+D+O9nXjjxnwfx38+8cOI/D+K/n3nhxH8exH8/88KJ/zyI/37mhRP/eRD//cwLJ/7zIP77mRdO/OdB/PczL5z4z4P472deOPGfB/Hfz7xw4j8P4r+feeHEfx7Efz/zwon/PIj/fuaFE/95EP/9zAsn/vMg/vuZF07850H89zMvnPjPg/jP9WDgrYDXBh4MvDRX/DVwK/DTwHfzwon/PIj/HA8GPgt4b/79xH8exH+8twa+CzjOfwzxnwfxH+u9ge/iP5b4z4P4j/PWwE/xH0/850H8x3gw8FfAcf7jif88iP8Y3w28F/85xH8exL/fg4Gn859H/OdB/Pt9NPBV/OcR/3kQ/34/DbwV/3nEfx7Ev99fAS/Nfx7xnwfx72f+c4n/PIh/P/OfS/znQfz7/TXwUvzn+BvgpfnPg/j3+2ngrfjP8T3Ae/OfB/Hv99HAV/Gf432A7+Y/D+Lf78HA0/mPdwl4MLDLfx7Ef4zvBt6L/1hfA3w0/7kQ/zEeDPw1cIz/GJeABwO7/OdC/Md5a+Cn+I/xNsBP858P8R/rvYHv4t/nfYDv5r8G4j/eWwPfDRzjX+cS8N7AT/NfB/Gf48HAZwPvxYvme4DPBm7lvxbiP9eDgbcGXht4MPBSXPE3wK3AbwM/DdzKfw/E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHm6U1Bsx8wUAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAudiotrack;
impl IconShape for MdAudiotrack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3v9.28c-.47-.17-.97-.28-1.5-.28C8.01 12 6 14.01 6 16.5S8.01 21 10.5 21c2.31 0 4.2-1.75 4.45-4H15V6h4V3h-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFm0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/juPALv86iP87fgp4H2CXFx3i/4YHA08HPgf4bF50iP8bvht4L2AXOMGLDvG/34OBp/Ns7wN8Ny8axP9+nw18Fs92K/AQXjSI/92OA08HjvOc3gb4af5liP/dPhr4Kp7XbwOvw78M8b/Tg4HPAt6bF+y3ga8GfoYXDPGf67254rv5j/HawGcBr82L7lbgs4Hv4Xkh/nM9nSsewr/PewEfDbw0/3a7wFcDXwPscgXiP89rA7/FFa8D/Db/Nh8NfDZwjP8YPwO8N7ALIP7z/Bbw2lzx28Dr8G93HPho4L2BB/Fv8z3AZwO38myI/xwvDfwVz+llgL/m3++9gc8GHsS/7BLw1cB3A7fyvBD/Ob4beC+e0/cA781/nNcGvhp4KZ6/jwG+G9jlBUP8x3sw8HSev4cAt/If57WB3+J5fQ/w3vzLEP/xvhr4KJ6/rwE+mv9YtwIP4jk9BLiVfxniP9Zx4OnAcZ6/XeAhwC7/cd4b+C6e7XeA1+ZFg/iP9dnAZ/HCfQ7w2fzHuhV4EFe8DvDbvGgQ/7GeDjyYF+5W4CH8x/ps4LOAvwFemhcd4j/OewPfxYvmfYDv5j/OceBW4KOB7+ZFh/iP83TgwbxobgUewn+sjwa+mn8dxH+M1wZ+i3+d1wF+m/9eiH+/1wa+Cnhp/nX+GvgY4Lf574P4t3sv4LOBB/Pvcyvw2cD38F8P8a9zHPgo4L2BB/Mf61bgu4GvAXb5r4F40TwY+CjgvYHj/OfaBb4b+BrgVv5zIV64lwY+Cnhv/nt8N/A1wF/znwPx/L028FnAa/M/w28DnwP8Nv+xEM/pvYDPBh7M/0y3Ap8NfA//MRDP6b2BzwYexP9MzwA+G/hu/mMgnr/XBj4beC3+Z/gd4LOB3+Y/FuKFe2ngo4H34r/H9wBfDfw1/zkQL5oHAx8NvDdwjP9cl4DvBr4auJX/XIh/nePARwPvDTyI/1jPAL4b+Gpgl/8aiH+79wY+G3gQ/z7PAD4b+G7+6yH+/V4b+GrgpfjX+Rvgo4Hf5r8P4j/GawO/xb/O6wC/zX8vxH+cW4EH8aJ5BvBg/mN9NPDV/Osg/uO8N/BdvGjeB/hu/uMcB54OfAzw3bzoEP+xbgUexAv3DODB/Mf6bOCzgL8GXoYXHeI/1mcDn8UL9znAZ/Mf6+nAg7nidYDf5kWD+I91HLgVOMbzdwl4MLDLf5z3Br6LZ/tt4HV40SD+43018FE8f18DfDT/sZ4OPJjn9BDgVv5liP94DwaezvP3EOBW/uO8NvBbPK/vAd6bfxniP8d3A+/Fc/oe4L35j/PawFcBL83z99HA9wC7vGCI/xwvDfwVz+llgL/m3++9gM8GHsy/bBf4auB7gFt5Xoj/PL8NvBZX/A7w2vzbHQc+Cnhv4MH823w38DnArTwb4j/PawO/xRWvA/w2/zYfDXwWcJz/GD8DvDewCyD+c93KFQ/m3+e9gY8GXop/u0vAVwNfDexyBeI/13tzxXfzH+O1gc8GXosX3TOAzwa+m+eF+N/pwcBnA+/FC/Y7wFcDP80Lhvjf7aOBr+J5/Q7w2vzLEP+7HQduBY7xnN4G+Gn+ZYj//T4b+Cye7RnAg3nRIP73ezDwdJ7tfYDv5kWD+L/hu4H3Ai4Bx3nRIf5veDDwdOBzgM/mRYf4v+OngfcGdnnRIf7vOA7s8q+D+P8N8f8b/wg9z8pB8GZHggAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoAwesome;
impl IconShape for MdAutoAwesome {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25L19 9zm-7.5.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12l-5.5-2.5zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25L19 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEsUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+fd5HeC3eU6vDfwW/z67wE8DnwPcyr8M8aJ5b+C7+I/zOsBv85xeG/gt/uO8D/DdvHCIf9lLA3/Ff6zXAX6b5/TawG/xH+shwK28YIh/2XcD78V/rNcBfpvn9NrAb/Ef63uA9+YFQ/zLLgLH+Y/1OsBv85xeG/gt/mPtAid4wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Md7HeC3eU6vDfwW//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jkdB16a/3i/zQuG+JeZ/3ivA/w2//0Q/zLzH+91gN/mvx/iX2b+470O8Nv890P8y8x/vNcBfpv/foh/mfmP9zrAb/OcjgMvxX+83+EFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/Mt2gWP8x3od4Ld5Tq8N/Bb/sS4Bx3nBEP+y7wbei/9YrwP8Ns/ptYHf4j/W9wDvzQuG+Je9NPBX/Md6HeC3eU6vDfwW/7EeAtzKC4Z40bw38F38x3kd4Ld5Tq8N/Bb/cd4H+G5eOMSL7sHAZwNvDRzj3+d1gN/mOb028Fv8+1wCfhr4bOBW/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPcRppBZrHY0gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoAwesomeMosaic;
impl IconShape for MdAutoAwesomeMosaic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5v14c0 1.1.89 2 2 2h6V3H5c-1.11 0-2 .9-2 2zm16-2h-6v8h8V5c0-1.1-.9-2-2-2zm-6 18h6c1.1 0 2-.9 2-2v-6h-8v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zU+Bvhr/vMhXrivAj6a/3qvA/w2//kQL9hXAx/Ff4/XAX6b/3yI5++lgb/iv8/rAL/Nfz7E8/fZwGfx3+d1gN/mPx/i+ftp4K14TpeAv+a/xkcDf81/PsTz99vAa/Gcfgd4bf5vQTx/vw28Fs/pd4DX5v8WxPP328Br8Zx+B3ht/m9BPH+/DbwWz+l3gNfm/xbE8/fbwGvxnH4HeG3+b0E8f78NvBbP6XeA1+YFe2ngo4C3Bo7zX0f82yGev98GXovn9DvAa/P8vTfwXfz3EP92iOfvt4HX4jn9DvDaPK+XBv6K/z7i3w7x/P028Fo8p98BXpvn9d3Ae/HfR/zbIZ6/3wZei+f0O8Br87wuAsf57yP+7RDP328Dr8Vz+h3gtXle5nl9DPDX/Nf4bf7tEM/fbwOvxXP6HeC1eV7meb0O8Nv8z4d4/n4beC2e0+8Ar83zMs/rdYDf5n8+xPP328Br8Zx+B3htnpd5Xq8D/Db/8yGev98GXovn9DvAa/O8zPN6HeC3+Z8P8fz9NvBaPKffAV6b52We1+sAv80L92DgQfznuQT8NS8c4vn7beC1eE6/A7w2z8s8r9cBfpvn7zjwXcBb85/vVuB9gN/m+UM8f78NvBbP6XeA1+Z5mef1OsBv87yOA08HjvNf632A7+Z5IZ6/3wZei+f0O8Br87zM83od4Ld5Xj8FvDX/9XaBhwC7PCfE8/fbwGvxnH4HeG2el3lerwP8Ns/pwcDT+e/zMcBX85wQz99vA6/Fc/od4LV5XuZ5vQ7w2zyn1wZ+i/8+3wO8N88J8fz9NvBaPKffAV6b52We1+sAv81zem3gt/jv8zvAa/OcEM/fbwOvxXP6HeC1eV7meb0O8Ns8p9cGfov/Pr8DvDbPCfH8/TbwWjyn3wFem+dlntfrAL/Nc3pt4Lf47/M7wGvznBDP328Dr8Vz+h3gtXle5nm9DvDbPKfXBn6L/z6/A7w2zwnx/P028Fo8p98BXpvnZZ7X6wC/zXN6beC3+O/zO8Br85wQz99LA8d5TrvAX/O8zPN6HeC3eU6vDfwW/31+B3htnhPi3888r9cBfpvn9NrAb/Hf53eA1+Y5If79zPN6HeC3eU6vDfwW/31+B3htnhPi3888r9cBfpvn9NrAb/Hf53eA1+Y5If79zPN6HeC3eU6vDfwW/31+B3htnhPi3888r9cBfpvn9NrAb/Hf53eA1+Y5If79zPN6HeC3eU6vDfwW/31+B3htnhPi3888r9cBfpvn9NrAb/Hf53eA1+Y5If79zPN6HeC3eU6vDfwW/31+B3htnhPi3888r9cBfpvn9NrAb/Hf53eA1+Y5If79zPN6HeC3eU6vDfwW/31+B3htnhPiv85rA7/Ff5/fAV6b54T4r/PawG/x3+d3gNfmOSH+67w28Fv89/kd4LV5Toj/Oq8N/Bb/fX4HeG2eE+K/zmsDv8V/n98BXpvnhPiv89rAb/Hf52eAt+Y5If7rvDTwV/z3+Rzgs3lOiP9atwIP4r/HQ4BbeU6I/1qvDfwW//W+Bvhonhfiv957A18NHOO/xtcAH83zh/jvcRx4b+ClgQfzH28X+Gvgu4FbecEQ/78h/n9D/P+G+P8N8f8b/widx8ZBpktWGwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoAwesomeMotion;
impl IconShape for MdAutoAwesomeMotion {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 2H4c-1.11 0-2 .9-2 2v10h2V4h10V2zm4 4H8c-1.11 0-2 .9-2 2v10h2V8h10V6zm2 4h-8c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoFixHigh;
impl IconShape for MdAutoFixHigh {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.5 5.6L10 7 8.6 4.5 10 2 7.5 3.4 5 2l1.4 2.5L5 7zm12 9.8L17 14l1.4 2.5L17 19l2.5-1.4L22 19l-1.4-2.5L22 14zM22 2l-2.5 1.4L17 2l1.4 2.5L17 7l2.5-1.4L22 7l-1.4-2.5zm-7.63 5.29c-.39-.39-1.02-.39-1.41 0L1.29 18.96c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0L16.7 11.05c.39-.39.39-1.02 0-1.41l-2.33-2.35zm-1.03 5.49l-2.12-2.12 2.44-2.44 2.12 2.12-2.44 2.44z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O700cCuwy4vmOPBg4K95Toj/nY4Dfw18F/A5vHCfBbwP8NLALs8J8b/XZwOfBdwKfAzw0zyn9wI+G3gw8DnAZ/O8EP+73Qo8iCt+G/gcrvgs4LW54hnAg3n+EP+7vTXwU7xwbwP8NM8f4n+/3wZei+fvd4DX5gVD/O/32sBv8fy9DvDbvGCI//1eG/gtnr/XAX6bFwzxv99vAa/N8/fbwOvwgiH+d3tr4Kd44d4G+GmeP8T/bk8HHswVvwN8Nld8NvBaXHEr8BCeP8T/Xp8NfBbwDOCjgZ/mOb018NXAg4DPAT6b54X43+k48NfAdwOfzQv32cB7Ay8N7PKcEP+zPBh4EPAM4FZesJcGbgV2edEcBx4M/DXPCfE/x2cBn80Vu8DHAN/Nfy7E/wzfBbw3z+t9gO/mPw/iv993Ae/NC/Y+wHfznwPx3+u7gPfm2f4GeGvgp4GX4tneB/hu/uMh/vt8F/DePNvfAK8N7ALHgd8GXopnex/gu/mPhfjv8V3Ae/NsfwO8NrDLsx0Hfht4KZ7tfYDv5j8O4r/edwHvzbP9DfDawC7P6zjw28BL8WzvA3w3/zEQ/7W+C3hvnu1vgNcGdnnBjgO/DbwUV+wCDwF2+fdD/Nf5LuC9eba/AV4b2OVf9mDg6Tzb6wC/zb8f4r/GdwHvzXN6CHAr/7LjwG8BL82zPQS4lX8/xH++7wLem+f118DrALu8YMeB3wJemmd7H+C7+Y+B+M/1XcB784L9NfA6wC7P6zjwW8BL82zvA3w3/3EQ/3m+C3hv/mV/DbwOsMuzHQd+C3hpnu19gO/mPxbiP8d3Ae/Ni+6vgdcBdoHjwG8BL82zvQ/w3fzHQ/zH+y7gvfnX+2vgbYCfAl6aZ3sf4Lv5z4H4j/VdwHvzH+d9gO/mPw/iP853Ae/Nf5z3Ab6b/1yI/xjfBbw3/3HeB/hu/vMh/v2+C3hv/uO8D/Dd/NdA/Pt8F/De/Md5H+C7+a+D+Lf7LuC9+Y/zPsB3818L8W/zXcB78x/nfYDv5r8e4l/vu4D35j/O+wDfzX8PxL/OdwHvzX+c9wG+m/8+iBfddwHvzX+c9wG+m/9eiBfNdwHvzX+c9wG+m/9+iH/ZdwHvzX+c9wG+m/8ZEC/cdwHvzX+c9wG+m/85EC/YdwHvzX+c9wG+m/9ZEM/fdwHvzX+c9wG+m/95EM/ru4D35j/O+wDfzf9MiOf02cBn8R/nfYDv5n8uxLM9GHg6/3HeB/hu/mdDPNtrA7/Ff4z3Ab6b//kQz/Zg4K+BY/z7vA/w3fzvgHhO7w18F/927wN8N/97IJ7XewPfxb/e+wDfzf8uiOfvvYHv4kX3PsB3878P4gV7b+C7+Je9D/Dd/O+EeOHeG/guXrD3Ab6b/70Q/7L3Br6L5/U+wHfzvxviRfPewFcDx7jifYDv5n8/xIvuwcCDgVuBW/m/AfH/G+L/N/4Rh7mnQe5EaAAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoFixNormal;
impl IconShape for MdAutoFixNormal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 2l-2.5 1.4L17 2l1.4 2.5L17 7l2.5-1.4L22 7l-1.4-2.5zm-7.63 5.29c-.39-.39-1.02-.39-1.41 0L1.29 18.96c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0L16.7 11.05c.39-.39.39-1.02 0-1.41l-2.33-2.35zm-1.03 5.49l-2.12-2.12 2.44-2.44 2.12 2.12-2.44 2.44z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoFixOff;
impl IconShape for MdAutoFixOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 1l-2.5 1.4L18 1l1.4 2.5L18 6l2.5-1.4L23 6l-1.4-2.5L23 1zm-8.34 6.22l2.12 2.12-2.44 2.44.81.81 2.55-2.55c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0L11.4 8.84l.81.81 2.45-2.43zm-.78 6.65l-3.75-3.75-6.86-6.86L2 4.53l6.86 6.86-6.57 6.57c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0l6.57-6.57L19.47 22l1.27-1.27-6.86-6.86z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoStories;
impl IconShape for MdAutoStories {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 1l-5 5v11l5-4.5V1zM1 6v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05C3.1 20.45 5.05 20 6.5 20c1.95 0 4.05.4 5.5 1.5V6c-1.45-1.1-3.55-1.5-5.5-1.5S2.45 4.9 1 6zm22 13.5V6c-.6-.45-1.25-.75-2-1v13.5c-1.1-.35-2.3-.5-3.5-.5-1.7 0-4.15.65-5.5 1.5v2c1.35-.85 3.8-1.5 5.5-1.5 1.65 0 3.35.3 4.75 1.05.1.05.15.05.25.05.25 0 .5-.25.5-.5v-1.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEV0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dY4DLwU8GHgwVxwHLgGfzX8PxH+u1wbeCnht4KV5/t4H+G7+eyD+4x0HPgp4b+DBvHDPAB7Mfx/Ef5zjwEcBHw0c50XzPsB3898H8R/jvYCvBo7zr3MC2OW/D+Lf5zjwXcBb86/3M8Bb898L8W/30sBPAQ/m3+Z9gO/mvxfi3+algd8CjvNvdwLY5b8X4l/vpYHfAo7zb/cM4MH890P867w08FvAcf59fgZ4a/77IV50x4G/Ah7Mv9/nAJ/Nfz/Ei+6ngLfmP8bnAJ/Nfz/Ei+a9ge/iP87HAF/Nfz/Ev+w48HTgOP9xXgf4bf77If5lnw18Fv+x3gb4af77IV6448DTgeP8x/oc4LP574d44T4b+Cz+430O8Nn890O8cE8HHsx/vO8B3pv/fogX7K2Bn+I/x18DL8N/P8QL9tXAR/GfR/z3Q7xgfwW8NP95Xgf4bf57IZ6/BwNP5z/X1wAfzX8vxPP32sBv8Z/rVuAh/PdCPH8fDXwV//leB/ht/vsgnr/PBj6L/3y/DbwO/30Qz99nA5/Ff42HALfy3wPx/P028Fr81/hp4G3474F4/n4beC3+67wO8Nv810M8f58NfBb/dW4FXgbY5b8W4vn7bOCz+K/13cD78F8L8fx9NPBV/Nd7H+C7+a+DeP5eG/gt/nu8DvDb/NdAPH8PBp7Of49d4HWAv+Y/H+IF+2vgpfjvsQt8DPDd/OdCvGBfDXwU/70+Gvga/vMgXrDXBn6L/34/DbwPsMt/PMQLdyvwIP777QIfDXwP/7EQL9xnA5/F/xy3Au8D/Db/MRAv3HHgVuAY/7PcCnw38D3ArfzbIf5lnw18Fv9z/TXw28BvA78D7PKiQ/zLjgO3Asf43+NW4HeA9+aFQ7xo3hv4Lv53eR3gt3nhEC+6nwbeiv8dvgb4aP5liBfdceCvgQfxP9vfAK8N7PIvQ/zrvDTw28Ax/me6BDwY2OVFg/jXe2ngt4Fj/M9yCXht4K950SH+bV4a+G3gGP8zXAJeG/hr/nUQ/3YvDfw08CD+e/0N8NrALv96iH+f48B3A2/Ff4+vAT4b2OXfBvEf472BrwaO8V/jEvDewE/z74P4j3Mc+Gjgo4Fj/Oe4BHw18NXALv9+iP94x4GPBt4beBD/MZ4BfDfw1cAu/3EQ/7leG3hr4LWBl+Jf52+A3wa+G/hr/nMg/uscB14aeGngOFe8NLAL3MoVu8BfA7/Nfw3E/2+I/98Q/78h/n9D/P/GPwJ43YdB+bORSQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBedtime;
impl IconShape for MdBedtime {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.34,2.02C6.59,1.82,2,6.42,2,12c0,5.52,4.48,10,10,10c3.71,0,6.93-2.02,8.66-5.02C13.15,16.73,8.57,8.55,12.34,2.02z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlurCircular;
impl IconShape for MdBlurCircular {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zM7 9.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm3 7c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm-3-3c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm3-6c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM14 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-1.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm3 6c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-4c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm2-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-3.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/Oi8NvBXwYOCvge8BdvnfC/Gie2/gu3hOtwJvA/w1z99rccXv8MI9GHgQcAn4a/7rIF40Lw38Fc/fXwMvw3M6DvwW8NJc8dfA6wC7PK+XBn4LOM4VnwN8Nv81EC+azwY+ixfsZYC/5tk+GvgqntPHAF/N8/or4KV5Ti8D/DXP33cB7w28DfDT/PsgXjTfDbwXL9jrAL/Ns3038F48p68BPprnZZ7X6wC/zfNnrvga4KP590G8aD4a+CpesBPALs/21sBP8ZzeBvhpntd3A+/Fs10CHgzs8vy9NfDawHcDf82/D+JFcxz4a+BBPK+vAT6a5/XZwHtzxXcDn83zdxz4auC9gL8B3hv4a/5rIF50Lw18N/BSPNvXAJ8N7PK/E+Jf76WB48BfA7v874b4/w3x/xviX+ezgLcGXhr4beBzgN/mfy/Ei+63gNfmeb0P8N08r5cG3osrvgf4a16wtwbeCvhr4HuAXf5rIF407w18F8/fLnCC5/TSwF/xnF4G+Gue12cDn8Wz/TXwMrxgx4GXAn6Hfz/Ei+argY/iBXsd4Ld5tq8GPorn9DXAR/O8LgLHeU6vA/w2z9/TgQcDnwN8Nv8+iBfNVwMfxQv2OsBv82xfDXwUz+lrgI/mee0Cx3hOrwP8Ns/frcCDgM8BPpt/H8SL5r2B7+L5uwQc5zm9NPBXPKeXAf6a5/XZwGfxbH8DvDQv2HHgpYHf5t8P8aL7beC1eF7vA3w3z+ulgffmiu8G/poX7K2B1wZuBb4b2OW/BuJf57OBtwZeCvgd4LOB3+Z/L8T/b4j/3xD/ei8NHAOeAdzK/26IF91LA98FvDTP9tXA5wC7/O+EeNEcB/4KeDDP62uAj+Z5fRbw0cAu8N3A5/D8HQe+Cnhv4K+B9wH+mv8aiBfNRwNfxQt2Atjl2d4a+Cme09sAP83z+m7gvXi2XeAhwC7P31sDbwV8DfDX/PsgXjTfDbwXL9jrAL/Ns3038F48p68BPprnZZ7X6wC/zfNnrvga4KP590G8aD4b+CxesJcB/ppn+2jgq3hOHwN8Nc/rr4GX4jm9DPDXPH/fDbwX8DbAT/Pvg3jRvDTwVzx/fwO8NM/pOPDbwEtxxd8Arw3s8rxeGvht4BhXfA7w2fzXQLzo3hv4Lp7TM4C3Bv6a5++1ueK3eeGOAy8N7AJ/zX8dxL/Og4H3Bo4DtwLfDezyvxfi/zfE/2+I/98Q/7LX4n+33+EFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwH8EqlB1N0htQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlurLinear;
impl IconShape for MdBlurLinear {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 17.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5.67 1.5 1.5 1.5zM9 13c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zM3 21h18v-2H3v2zM5 9.5c.83 0 1.5-.67 1.5-1.5S5.83 6.5 5 6.5 3.5 7.17 3.5 8 4.17 9.5 5 9.5zm0 4c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5.67 1.5 1.5 1.5zM9 17c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm8-.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM3 3v2h18V3H3zm14 5.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm0 4c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM13 9c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+67wW8DfALi/cceClgN/hPx/iv8ZnA58F3Ao8hBfu6cCDgc8BPpv/XIj/Gp8NfBbwDODBvHC3Ag8CPgf4bP5zIf7rvDbw18AuL9xx4KWB3+Y/H+JFc5wrdvm/BfEvOw78Fle8DrDL/x2IF+448FvAS3PFXwOvA+zyfwPiBTsO/Bbw0jynvwZeB9jlRffWwFsBfw18D7DL83cceC/gpYGfAX6aF+ylgffiiu8B/pp/PcQLdhz4beCleF5/DbwOsMu/7LOBz+LZ/hp4GZ6/vwJemmf7HOCzeV4vDfwVz+llgL/mXwfxwh0Hfht4KZ7XXwOvA+zywl0EjvOcXgf4bZ7TawO/xXO6FXgIz+urgY/iOX0N8NH86yD+ZceB3wZeiuf118DrALu8YLvAMZ7T6wC/zXN6beC3eE7PAB7M8/pq4KN4Tl8DfDT/OogXzXHgt4GX4nn9NfA6wC7P32cDn8Wz/Q3w0jx/fw28FM/2OcBn87xeGvgrntPLAH/Nvw7iRXcc+G3gpXhefw28DrDL8/fWwGsDtwLfDezy/B0H3ht4MPDbwE/zgr008N5c8d3AX/Ovh/jXOQ78NvBSPK+/Bl4H2OV/D8S/3nHgt4GX4nn9NfA6wC7/OyD+bY4Dvw28FM/rr4HXAXb5nw/xb3cc+G3gpXhefw28DrDLv95LA8eAZwC38p8L8e9zHPht4KV4Xn8NvA6wy4vmpYHvAl6aZ/tq4HOAXf5zIP7tPgv4aOA4L9hfA68D7PLCHQf+Cngwz+trgI/meX0W8NHALvDdwOfwr4d44T6LKz6H5/TWwE/xovlr4HWAXV6wjwa+ihfsBLDLs7018FM8p7cBfprn9Flc8Tk8f4gX7LWB3+KK1wF+m2f7buC9eE4/AzwYeCme118DrwPs8vx9N/BevGCvA/w2z/bdwHvxnL4G+Gie7bWB3+KK1wF+m+eFeMGOA3/NFS8N7PJsHw18Fc/pY4DvBn4beCme118DrwPs8rw+G/gsXrCXAf6aZ/to4Kt4Th8DfDXPdhz4a654aWCX54X4tzkO/DbwUlzxN8BrA7vAceC3gZfief018DrALs/pwcDTef7+BnhpntNx4LeBl+KKvwFeG9jlXwfx7/PaXPHbPKfjwG8DL8Xz+mvgdYBdntN7A9/Fc3oG8NbAX/P8vTZX/Db/Noj/PMeB3wZeiuf118DrALs8pwcD7w08GPhr4LuBXf7zIP5zHQd+G3gpntdfA68D7PLfB/Gf7zjw28BL8bz+GngdYJf/Hoj/fMeB9wK+mufvr4HXAXb5r4f493ktrvgdnr+XBn4LOM4L99fA6wC7/Ou8Flf8Dv82iH+b48BvAS/NFX8NvA6wy3P6K+CledH8NfA6wC7/suPAbwEvzRV/DbwOsMu/DuIFOw78FVe8DLDLs3008FU8p48Bvppne2ngr/jX+WvgdYBdXriPBr6K5/QxwFfzbMeBv+KKlwF2eV6IF+y1gd/iitcBfptn+27gvXhOXwN8NM/22sBv8a/318DrALu8YN8NvBfP6WuAj+bZXhv4La54HeC3eV6IF+6zueKzeU5vDfwUz+ltgJ/m2R4MPJ0X7GOA9wZeiuf118DrALs8f28N/BTP6W2An+Y5fTZXfDbPH+Lf7rOB9+aK7wY+m+f11cBH8byeAbw0V/w28FI8r78GXgfY5fn7bOC9ueK7gc/mXw/xn+s48NnAR/FsfwO8N/DXXHEc+G3gpXhefw28DrDLfw7Ef43jwEsDu8Bf87yOA78NvBTP66+B1wF2+Y+H+J/jOPDbwEvxvP4aeB1gl/9YiP9ZjgO/DbwUz+uvgdcBdvmPg/if5zjw28BL8bz+GngdYJf/GIh/u5cG3osrvgf4a16wtwbeCvhr4HuAXZ6/48B7AW8NvDbP318DrwPs8u+H+Ld5aeCveE4vA/w1z+uzgc/i2f4aeBmev78CXpp/2V8DrwPs8u+D+Lf5auCjeE5fA3w0z+sicJzn9DrAb/OcXhv4LZ7X3wAvxfP6a+B1gF3+7RD/Nl8NfBTP6WuAj+Z57QLHeE6vA/w2z+m1gd/iOT0DeGngt4GX4nn9NfA6wC7/Noh/m5cG/orn9DLAX/O8Phv4LJ7tb4CX5vn7a+CleLbPAT4bOA78NvBSPK+/Bl4H2OVfD/Fv99LAe3PFdwN/zQv21sBrA7cC3w3s8vwdB94beDDw28BP82zHgd8GXorn9dfA6wC7/Osg/nc5Dvw28FI8r78GXgfY5UWH+N/nOPDbwEvxvP4aeB1glxcN4n+n48BvAy/F8/pr4HWAXf5liP86rwX8DbDLC3cceCngd3jhjgO/DbwUz+lvgNcGdvmXIf5rfDbwWcCtwEN44Z4OPBj4HOCzeeGOA78NvBRX/A3w2sAuLxrEf43PBj4LeAbwYF64W4EHAZ8DfDb/suPAb3PFawO7vOgQ/3VeG/hrYJcX7jjw0sBv86I7Duzyr4f4/w3x/xvi/zfE/2+I/98Q/7/xj9eDSlCc/bJMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlurOff;
impl IconShape for MdBlurOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm-.2 4.48l.2.02c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5l.02.2c.09.67.61 1.19 1.28 1.28zM14 3.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm-4 0c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm11 7c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM10 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm8 8c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm-4 13.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM2.5 5.27l3.78 3.78L6 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1c0-.1-.03-.19-.06-.28l2.81 2.81c-.71.11-1.25.73-1.25 1.47 0 .83.67 1.5 1.5 1.5.74 0 1.36-.54 1.47-1.25l2.81 2.81c-.09-.03-.18-.06-.28-.06-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1c0-.1-.03-.19-.06-.28l3.78 3.78L20 20.23 3.77 4 2.5 5.27zM10 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm11-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 13c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zM3 9.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm7 11c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm-3-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlurOn;
impl IconShape for MdBlurOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 13c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-8c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm-3 .5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm15 5.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM14 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-3.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm-11 10c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm7 7c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-17c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM10 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 5.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm8 .5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-8c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm3 8.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM14 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm-4-12c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0 8.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm4-4.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0-4c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1xvBbw28NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5v+UEsMtzQjx/rw38Fv+3vA7w2zwnxPP30cBX8X/LxwBfzXNCPH+fDXwW/7d8DvDZPCfE8/fZwGfxf8vnAJ/Nc0I8f78NvBb/t/wM8NY8J8Tz99vAa/F/y88Ab81zQjx/nw18Fv+3fA7w2TwnxPP32cBn8X/L5wCfzXNCPH8fDXwV/7d8DPDVPCfE8/fawG/xf8vrAL/Nc0I8fw8Gns7/LSeAXZ4T4gX7a+Cl+L/hb4CX5nkhXrCvBj6K/xu+BvhonhfiBXtr4Kf4v+F1gN/meSFeuFuBB/G/2zOAB/P8IV64zwY+i//dPgf4bJ4/xAt3HLgVOMb/TpeABwO7PH+If9lnA5/F/06fA3w2LxjiX3YcuBU4xv8ul4AHA7u8YIgXzXsD38X/Lm8D/DQvHOJF99PAW/G/w88Ab82/DPGiOw78NfAg/md7BvDSwC7/MsS/zksDvw0c43+mS8BrA3/Niwbxr/fSwG8Dx/if5RLw2sBf86JD/Nu8NPDbwDH+Z7gEvDbw1/zrIP7tXhr4aeBB/Pd6BvDWwF/zr4f49zkOfDfwVvz3+BngvYFd/m0Q/zHeG/hq4Bj/NS4BHw18N/8+iP84x4GPBj4aOMZ/jkvAVwNfDezy74f4j3cc+GjgvYEH8R/jGcB3A18N7PIfB/Gf67WBtwZeG3gp/nX+Bvht4KeB3+Y/B+K/znHgpYGXBo5zxUtzxV9zxS7w18BfA7v850P8/4b4/w3x/xvi/zfE/2/8I647kEEdFaF9AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness1;
impl IconShape for MdBrightness1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/OR4MvBTw0jzbawO3Ardyxa3ArcDv8N8H8R/nrYD3Bl4aeDD/Or8N/DTwM8Ct/NdB/Ps8GPgs4K2B4/zH+Gvgq4Hv4T8f4t/mwcBnAe/Nf55bgc8Gvof/PIh/mwcD3w28Fv/5fhv4GOCv+Y+H+Pf5aOCzgWP85/to4Gv4j4X493tp4KeBB/Gf77uB9+E/DuI/xnHgt4GX4j/fXwOvA+zy74f4j3Mc+G3gpfjP99fA6wC7/Psg/mMdB/4aeBD/+X4beB3+fRD/8V4a+G3gGP/5vgb4aP7tEP853hv4Lv5rvA3w0/zbIP7z/DTwVvzn2wUeAuzyr4f4z3McuBU4xn++7wHem389xH+uzwY+i/8arwP8Nv86iP9cx4FbgWP85/sZ4K3510H85/ts4LP4r/EQ4FZedIj/fA8Gns5/ja8BPpoXHeK/xl8DL8V/vluBh/CiQ/zX+Gjgq/iv8TrAb/OiQfzXeGngr/iv8TnAZ/OiQfzXMf81fgd4bV40iP86vw28Fv/5doETvGgQ/3W+Gvgo/mucAHb5lyH+63w28Fn813gd4Lf5lyH+63w28Fn813gd4Lf5lyH+67w38F3813gd4Lf5lyH+67w38F3813gd4Lf5lyH+63w28Fn813gd4Lf5lyH+63w28Fn813gd4Lf5lyH+63w18FH81zgB7PIvQ/zX+W3gtfjPdwk4zosG8V/H/Nf4HeC1edEg/mu8NPBX/Nf4HOCzedEg/mt8NPBV/Nd4HeC3edEg/mv8FfDS/Od7BvBgXnSI/3wPBp7Of42vAT6aFx3iP99nA5/Ff42HALfyokP85zoOPB04zn++nwHemn8dxH+uzwY+i/8arwP8Nv86iP88x4GnA8f5z/c9wHvzr4f4z/NTwFvzn+8S8GBgl389xH+O9wa+i/8abwP8NP82iP94Lw38FnCc/3xfA3w0/3aI/1jHgb8CHsx/vt8BXpt/H8R/nOPAbwEvzX++vwFeG9jl3wfxH+M48FvAS/Of72+A1wZ2+fdD/Pu9NPBbwHH+830P8N78x0H8+3wU8NnAcf5zXQI+G/hq/mMh/m0eDHwX8Nr85/sd4L2BW/mPh/i3eTDw2cB78Z/nGcBnA9/Nfx7Ev8+Dgc8G3ho4xn+MvwG+Gvhu/vMh/uO8NfDWwGsDD+Jf53eAnwZ+GriV/zqI/xzHgdcGXppne23gVuBWrvhrYBf4bf77IP5/Q/z/hvj/DfH/G+L/N/4RhZWDQRhpWSIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness2;
impl IconShape for MdBrightness2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 2c-1.82 0-3.53.5-5 1.35C7.99 5.08 10 8.3 10 12s-2.01 6.92-5 8.65C6.47 21.5 8.18 22 10 22c5.52 0 10-4.48 10-10S15.52 2 10 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sV4LeGngOFc8GDgO/DVX7AJ/DTwDuJX/foh/v7cGPgt4af51/hr4beBngN/mvwfiP8aDgY8G3hs4xr/ercB3A18D7PJfB/Ef77OBjwaO8a+3C3w18DXALv/5EP85Hgx8NfBW/NvsAh8NfA//uRD/ud4a+G7gGP82Pw28D7DLfw7Ef76XBn4aeBD/NrcCbwP8Nf/xEP81jgO/DbwU/za7wOsAf81/LMR/nePAbwMvxb/NLvA6wF/zHwfxX+s48NfAg/i32QVeB/hr/mMg/uu9NPDbwDH+bW4FXgbY5d8P8d/jrYGf4t/up4G34d8P8d/nt4HX4t/ufYDv5t8H8d/nwcBfA8f4t9kFHgLs8m+H+O/12cBn8W/3OcBn82+H+O91HLgVOMa/zS7wEGCXfxvEf7/vBt6Lf7vPAT6bfxvEf78HA0/n3+5W4CH82yD+Z/hr4KX4t3sd4Lf510P8z/DZwGfxb/c1wEfzr4f4n+G1gd/i3+6vgZfhXw/xP4f59zkB7PKvg/if47eB1+Lf7nWA3+ZfB/E/x3cD78W/3ccAX82/DuJ/js8GPot/u88BPpt/HcT/HJ8NfBb/dp8DfDb/Ooj/Od4b+C7+7X4GeGv+dRD/c7w38F382/0M8Nb86yD+5/hs4LP4t/sc4LP510H8z/HZwGfxb/c5wGfzr4P4n+O7gffi3+5jgK/mXwfxP8dvA6/Fv93rAL/Nvw7ifw7z73MC2OVfB/E/w2sDv8W/3d8AL82/HuJ/hs8GPot/u68BPpp/PcT/DH8FvDT/dq8D/Db/eoj/fg8Gns6/3TOAB/Nvg/jv993Ae/Fv9znAZ/Nvg/jvdRx4OnCcf5tLwIOBXf5tEP+9Phv4LP7tPgf4bP7tEP99Hgz8FXCcf5tLwIOBXf7tEP99fgt4bf7t3gb4af59EP893hr4Kf7tfgZ4a/79EP/1Xhr4LeA4/zbPAF4a2OXfD/Ff6zjwdOA4/zaXgNcG/pr/GIj/OseB3wJemn+bS8BrA3/NfxzEf43jwG8BL82/zSXgtYG/5j8W4j/fSwO/BRzn3+YZwFsDf81/PMR/rrcGvgs4zr/NzwDvDezynwPxn+PBwFcBb82/zSXgvYGf5j8X4j/WceCjgI8GjvOvdwn4auCrgV3+8yH+YzwY+CjgvYHj/Os9A/hu4KuBXf7rIP793hr4bOCl+Nf5G+C3gZ8Gfpv/Hoj/WK8NvDRwnCseDBwH/pordoG/Bv4a2OW/H+L/N8T/b4j/3xD/vyH+f+MfAQ/GkEE+haH1AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness3;
impl IconShape for MdBrightness3 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2c-1.05 0-2.05.16-3 .46 4.06 1.27 7 5.06 7 9.54 0 4.48-2.94 8.27-7 9.54.95.3 1.95.46 3 .46 5.52 0 10-4.48 10-10S14.52 2 9 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness4;
impl IconShape for MdBrightness4 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM12 18c-.89 0-1.74-.2-2.5-.55C11.56 16.5 13 14.42 13 12s-1.44-4.5-3.5-5.45C10.26 6.2 11.11 6 12 6c3.31 0 6 2.69 6 6s-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF1klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13dxxfvw3wPx3+e7gPfmiu8G3of/eoj/Ht8FvDfP6buB9+G/FuK/3ncB783z993A+/BfB/Ff67uA9+aF+27gffivgfiv813Ae/Oi+W7gffjPh/iv8V3Ae/Ov893A+/CfC/Gf77uA9+bf5ruB9+E/D+JFdxx4Kf513ht4b/59vhv4bv51/gbY5V+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/i3+dngJ8GbgV2gb/mipcGjgMPBt4aeCv+fV4H+G3+ZYgX3WsDv8W/3iXgo4GfBnZ50RwH3hr4auAY/3qvA/w2/zLEi+61gd/iX+dzgK8Gdvm3OQ58NPBZ/Ou8DvDb/MsQL7rXBn6LF80l4LWBv+Y/xksDvw0c40XzOsBv8y9DvOheG/gt/mV/A7w2sMt/rAcDPw28FP+y1wF+m38Z4kX32sBv8cJdAh4M7PKf48HAXwPHeOFeB/ht/mWIF91rA7/FC/cywF/zn+ulgb/ihXsd4Lf5lyFedN8FvDcv2OcAn81/jc8GPosX7LuB9+FfhnjRfBfw3rxgl4AHA7v81zgO3Aoc4wX7buB9eOEQ/7LvAt6bF+59gO/mv9Z7A9/FC/fdwPvwgiFeuO8C3pt/2Qlgl/9ax4GL/Mu+G3gfnj/EC/ZdwHvzL/sZ4K357/HTwFvxL/tu4H14Xojn77uA9+ZF8z7Ad/Pf46OBr+JF893A+/CcEM/ru4D35kX3OsBv89/jtYHf4kX33cD78GyI5/RdwHvzr/MywF/z3+Olgb/iX+e7gffhCsSzfRfw3vzrif9e5l/vu4H3ARDP9t3Ae/GvJ/57mX+97wHeG0A8p+8G3ot/nZcB/pr/Hi8N/BX/Ot8DvDdXIJ7XdwPvxYvudYDf5r/HawO/xYvue4D35tkQz993A+/Fi+Z9gO/mv8dHA1/Fi+Z7gPfmOSFesO8G3ot/2c8Ab81/j58G3op/2fcA783zQrxw3w28F/+yE8Au/7WOAxf5l30P8N48f4h/2XcD78UL9z7Ad/Nf672B7+KF+x7gvXnBEC+a7wbeixdsF3gIsMt/jePA04HjvGDfA7w3LxziRffdwHvxgn0O8Nn81/hs4LN4wb4HeG/+ZYgX3WsDv8UL9zLAX/Of66WBv+KFex3gt/mXIV50rw38Fi/cLvAQYJf/HA8G/go4zgv3OsBv8y9DvOheG/gt/mV/DbwOsMt/rAcDPwW8NP+y1wF+m38Z4kX32sBv8aLZBV4H+Gv+Y7w08FvAcV40rwP8Nv8yxIvutYHf4l/ns4GvAXb5tzkOfBTw2fzrvA7w2/zLEC+61wZ+i3+9XeCjgZ8BdnnRHAfeCvhq4Dj/eq8D/Db/MsSL7rWB3+Lf56eBnwZuBS4Bf80VLw0cA14aeG3grfn3eR3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aI7Drw0/zrvDbwX/z7fA3w3/zp/DezyL0P85/tu4L34t/ke4L35z4P4r/HdwHvxr/M9wHvznwvxX+e7gffiRfM9wHvznw/xX+u7gffihfse4L35r4H4r/fdwHvx/H0P8N7810H89/hu4L14Tt8DvDf/tRD/fb4beC+u+B7gvfmvh/jv9d1c8d7890D8/4b4/41/BBpfzEGo1QmsAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness5;
impl IconShape for MdBrightness5 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13dxxfvw3wPx3+e7gPfmiu8G3of/eoj/Ht8FvDfP6buB9+G/FuK/3ncB783z993A+/BfB/Ff67uA9+aF+27gffivgfiv813Ae/Oi+W7gffjPh/iv8V3Ae/Ov893A+/CfC/Gf77uA9+bf5ruB9+E/D+JFdxx4Kf513ht4b/59vhv4bv51/gbY5V+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/iP56AlwaOAw8G3hp4K/59Xgf4bf5liBfdawO/xX888byOA28NfDVwjH+91wF+m38Z4kX32sBv8R9PvGDHgY8GPot/ndcBfpt/GeJF99rAb/EfT/zLXhr4beAYL5rXAX6bfxniRffawG/xH0+8aB4M/DTwUvzLXgf4bf5liBfdawO/xX888aJ7MPDXwDFeuNcBfpt/GeJF99rAb/EfT/zrvDTwV7xwrwP8Nv8yxIvuu4D35j+e+Nf7bOCzeMG+G3gf/mWIF813Ae/Nfw7xr3ccuBU4xgv23cD78MIh/mXfBbw3/3nEv817A9/FC/fdwPvwgiFeuO8C3pv/XOLf5jhwkX/ZdwPvw/OHeMG+C3hv/vOJf7ufBt6Kf9l3A+/D80I8f98FvDf/NcS/3UcDX8WL5ruB9+E5IZ7XdwHvzX8d8W/32sBv8aL7buB9eDbEc/ou4L35ryX+7V4a+Cv+db4beB+uQDzbdwHvzX898e9j/vW+G3gfAPFs3w28F//1xL+P+df7HuC9AcRz+m7gvfivJf7tXhr4K/51vgd4b65APK/vBt6L/zri3+61gd/iRfc9wHvzbIjn77uB9+K/hvi3+2jgq3jRfA/w3jwnxAv23cB78Z9P/Nv9NPBW/Mu+B3hvnhfihftu4L34zyX+bY4DF/mXfQ/w3jx/iH/ZdwPvxX8e8W/z3sB38cJ9D/DevGCIF813A+/Ffw7xr3cceDpwnBfse4D35oVDvOi+G3gv/uOJf73PBj6LF+x7gPfmX4Z40b028Fv8xxP/Oi8N/BUv3OsAv82/DPGie23gt/iPJ150Dwb+CjjOC/c6wG/zL0O86F4b+C3+44kXzYOBnwJemn/Z6wC/zb8M8aJ7beC3+I8n/mUvDfwWcJwXzesAv82/DPGie23gt/iPJ16w48BHAZ/Nv87rAL/Nvwzxontt4Lf4jyee13HgrYCvBo7zr/c6wG/zL0O86F4b+C3+4wl4aeAY8NLAawNvzb/P6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91x4KX513lv4L349/ke4Lv51/lrYJd/GeI/33cD78W/zfcA781/HsR/je8G3ot/ne8B3pv/XIj/Ot8NvBcvmu8B3pv/fIj/Wt8NvBcv3PcA781/DcR/ve8G3ovn73uA9+a/DuK/x3cD78Vz+h7gvfmvhfjv893Ae3HF9wDvzX89xH+v7+aK9+a/B+L/N8T/b/wjI+WaQRuKGYsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness6;
impl IconShape for MdBrightness6 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18V6c3.31 0 6 2.69 6 6s-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightness7;
impl IconShape for MdBrightness7 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6zm0-10c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z8+10CXhv4aOC9+I/zPlzxXbxg4gVD/MvMv88l4LWBv+aK7wbei3+/9wG+myveG/gunj/xgiH+ZbvAMf5tLgGvDfw1z+m7gffi3+59gO/mOb038F08L/GCIf5lLw38NnCMf51LwGsDf83z993Ae/Gv9z7Ad/P8vTfwXTwn8YIhXjQvDfw2cIwXzSXgtYG/5oX7buC9eNG9D/DdvHDvDXwXzyZeMMSL7qWB3waO8cJdAl4b+GteNN8NvBf/svcBvpsXzXsD38UV4gVD/Ou8NPDbwDGev0vAawN/zb/OdwPvxQv2PsB386/z3sB3AeIFQ/zrvTTw28AxntMl4LWBv+b5e23gt3nBvht4L57X+wDfzfP30lzx1zx/7w18Ny8Y4t/mpYHfBo5xxSXgtYG/5vl7b+C7gO8G3ocX7LuB9+LZ3gf4bp6/lwZ+iyteB/hr/vUQ/3YvDfw2V7w28Nc8f+8NfBfP9t3A+/CCfTfwXsD7AN/N8/fSwG8Bx7liF3gd4K/510H8y94b+G6ev5fmir/m+Xtv4Lt4Xt8NvA8v2GsDv83z99LAbwHHeU67wOsAf82LDvEvM/A+wHfzr/PewHfxgn038D7867w08FvAcZ6/XeB1gL/mRYP4l5kr3gf4bl407w18F/+y7wbehxfNSwO/BRznhdsFXgf4a/5liH+Zebb3Ab6bF+69ge/iRffdwPvwwr008FvAcV40u8DrAH/NC4f4l5nn9D7Ad/P8vTfwXfzrfTfwPjx/Lw38FnCcf51d4HWAv+YFQ/zLzPN6H+C7eU7vDXwX/3bfDbwPz+mlgd8CjvNvswuc4AVD/MvM8/c+wHdzxXsD38W/33cD78MVLw38FnCcfx/xgiH+ZeYFex+u+C7+43w38DXAbwHH+fcTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJftgsc43+nS8BxXjDEv+y7gffif6fvAd6bFwzxL3sw8HT+d3oIcCsvGOJF897Ad/G/y/sA380Lh3jRPRj4bOCtgWP8z3QJ+Gngs4Fb+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/gJaFBIMJqvgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrokenImage;
impl IconShape for MdBrokenImage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M21 5v6.59l-3-3.01-4 4.01-4-4-4 4-3-3.01V5c0-1.1.9-2 2-2h14c1.1 0 2 .9 2 2zm-3 6.42l3 3.01V19c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2v-6.58l3 2.99 4-4 4 4 4-3.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+048FXAewO3Ah8D/DQvOsT/XseB3wJemuf0PsB386JB/O90HPgt4KV5/t4H+G7+ZYj/fY4DvwW8NC/c+wDfzQuH+N/lOPBbwEvzonkb4Kd5wRD/exwHfgt4aV50twIP4QVD/O9wHPgt4KX51xMvGOJ/vuPAbwEvzb/e9wDvzQuG+J/tOPBbwEvzr/c3wGsDu7xgiP+5jgO/Bbw0/3p/A7w2sMsLh/if6TjwW8BL86/3N8BrA7v8yxD/8xwHfgt4af71/gZ4bWCXFw3if5bjwG8BL82/3t8Arw3s8qJD/M9xHPgt4KX51/sb4LWBXf51EP8zHAd+C3hp/vX+BnhtYJd/PcR/v+PAbwEvzb/e3wCvDezyb4P473Uc+C3gpfnX+xvgtYFd/u0Q/32OA78FvDT/en8DvDawy78P4r/HceC3gJfmX+9vgNcGdvn3Q/zXOw78FvDS/Ov9DfDawC7/MRD/tY4DvwW8NP96fwO8NrDLfxzEf53jwG8BL82/3t8Arw3s8h8L8V/jOPBbwEvzr/c3wGsDu/zHQ/znOw78FvDS/Ov9DfDawC7/ORD/+X4beC3+9f4GeG1gl/88iP98Lw38NnCMF93fAK8N7PKfC/Ff46WB3waO8S/7G+C1gV3+8yH+67w08NvAMV6wvwFeG9jlvwbiv9ZLA78NHON5/Q3w2sAu/3UQ/z7HgbcCHgy8NPDXwC7wM8CtPH8vDfw2cIxn+xvgtYFd/msh/u0+C/ho4DjP33cDHwPs8rxeGvht4BjwN8BrA7v810P86x0Hfgp4bf5lu8DrAH/N83pp4KuBtwZ2+e+B+Nf7LuC9edHtAg8BdvmfB/Gv89bAT/Gv9z3Ae/M/D+Jf56+Al+bf5iHArfzPgnjRPRh4Ov92HwN8Nf+zIF50rw38Fv92nwN8Nv+zIF50bw38FP92PwO8Nf+zIF50nw18Fv92nwN8Nv+zIF50fwW8NP92HwN8Nf+zIF40Hw18Ff8+DwFu5X8WxL/so4Cv5t/nd4DX5n8exPP30sBbAe8NPJh/v9cBfpv/eRDP33Hgt4GX4t/ve4D35n8mxAt2HPht4KX4t/sb4LWBXf5nQrxwx4GPBj6Lf73fAd4a2OV/LsSL5qWBzwbein/ZJeCzga/mfz7Ev85x4L2Bt+aK1+KKvwFuBX4a+Glgl/8dEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hH3oGIQdCx6ooAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrush;
impl IconShape for MdBrush {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 14c-1.66 0-3 1.34-3 3 0 1.31-1.16 2-2 2 .92 1.22 2.49 2 4 2 2.21 0 4-1.79 4-4 0-1.66-1.34-3-3-3zm13.71-9.37l-1.34-1.34c-.39-.39-1.02-.39-1.41 0L9 12.25 11.75 15l8.96-8.96c.39-.39.39-1.02 0-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGJUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz/bSwFfxvF6H5/XVwEvxnP4G+Giev7cG3gp4MP+9fhr4HmCXKxDP9trAb/G8xPP6beC1eE6/A7w2z+uzgc/if46/Bl6GKxDP9trAb/G8xPP6beC1eE6/A7w2z+sicJz/WV4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/O8zP88rwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1eV7mf57XAX4bQDzbawO/xfMSz+u3gdfiOf0O8No8L/M/z+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bZ6X+Z/ndYDfBhDP9trAb/G8xPP6beC1eE6/A7w2z8v8z/M6wG8DiGd7beC3eF7ief028Fo8p98BXpvnZf7neR3gtwHEs7028Fs8L/G8fht4LZ7T7wCvzfMy//O8DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmeZn/eV4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/O8zP88rwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1eV7mf57XAX4bQDzbawO/xfMSz+u3gdfiOf0O8No8L/M/z+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bZ6X+Z/ndYDfBhDP9trAb/G8xPP6beC1eE6/A7w2z8v8z/M6wG8DiGd7beC3eF7ief028Fo8p98BXpvnZf7neR3gtwHEs7028Fs8L/G8fht4LZ7T7wCvzfMy//O8DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmeZn/eV4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/O8zP88rwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1eV7mf57XAX4bQDzbawO/xfMSz+u3gdfiOf0O8No8L/Of4xLw2cBX8a/3OsBvA4hne23gt3he4nn9NvBaPKffAV6b52X+c7wN8NPAZwOfxb/O6wC/DSCe7bWB3+J5ief128Br8Zx+B3htnpf5j/c1wEfzbL8NvBYvutcBfhtAPNtrA7/F8xLP67eB1+I5/Q7w2jwv86L7Gq74KF6w3wFem+d0HPhr4EG8aF4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/O8zIvmd4DX5oqfBt6K53UJeDCwy/N6aeCveNG8DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmeZl/2TOAlwZ2ueI48NvAS/GcXgf4bV6wjwa+in/Z6wC/DSCe7bWB3+J5ief128Br8Zx+B3htnpd54S4Brw38Nc/pwcBfA8e44mOAr+Zf9tPAW/HCvQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5nmZF+59gO/m+Xtp4K+AnwHemhfNceCvgQfxgr0O8NsA4tleG/gtnpd4Xr8NvBbP6XeA1+Z5mRfse4D35oV7a+C3gV1edC8N/DZwjOfvdYDfBhDP9trAb/G8xPP6beC1eE6/A7w2z8s8f38DvDT/ed4b+C6ev9cBfhtAPNtrA7/F8xLP67eB1+I5/Q7w2jwv87wuAQ8GdvnP9d3Ae/G8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Br87zM87oVuJX/fMeBl+Z5vQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5nmZ/3leB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzvMz/PK8D/DaAeLbXBn6L5yWe128Dr8Vz+h3gtXleu8Ax/md5HeC3AcSzvTbwWzwv8bx+G3gtntPvAK/N8/ps4LP4n+NvgJfmCsSzvTTw1Tyv1+Z5fTXw0jynvwY+mufvrYG3Bh7Mf59d4LeB7wZ2uQLx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+zquhB2vcHaAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBurstMode;
impl IconShape for MdBurstMode {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 5h2v14H1zm4 0h2v14H5zm17 0H10c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1zM11 17l2.5-3.15L15.29 16l2.5-3.22L21 17H11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCamera;
impl IconShape for MdCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.4 10.5l4.77-8.26C13.47 2.09 12.75 2 12 2c-2.4 0-4.6.85-6.32 2.25l3.66 6.35.06-.1zM21.54 9c-.92-2.92-3.15-5.26-6-6.34L11.88 9h9.66zm.26 1h-7.49l.29.5 4.76 8.25C21 16.97 22 14.61 22 12c0-.69-.07-1.35-.2-2zM8.54 12l-3.9-6.75C3.01 7.03 2 9.39 2 12c0 .69.07 1.35.2 2h7.49l-1.15-2zm-6.08 3c.92 2.92 3.15 5.26 6 6.34L12.12 15H2.46zm11.27 0l-3.9 6.76c.7.15 1.42.24 2.17.24 2.4 0 4.6-.85 6.32-2.25l-3.66-6.35-.93 1.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCameraAlt;
impl IconShape for MdCameraAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "3.2",
            }
            path {
                d: "M9 2L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2h-3.17L15 2H9zm3 15c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCameraFront;
impl IconShape for MdCameraFront {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 20H5v2h5v2l3-3-3-3v2zm4 0v2h5v-2h-5zM12 8c1.1 0 2-.9 2-2s-.9-2-2-2-1.99.9-1.99 2S10.9 8 12 8zm5-8H7C5.9 0 5 .9 5 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V2c0-1.1-.9-2-2-2zM7 2h10v10.5c0-1.67-3.33-2.5-5-2.5s-5 .83-5 2.5V2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj3OQ58FPDewIP5r3Ur8N3A1wC7/Nsg/u0eDPwW8GD+e/018DbArfzrIf7t/gp4af5n+GvgZfjXQ/zbvDXwU/zP8jbAT/Ovg/i3+Wzgs/if5XOAz+ZfB/Fv89vAa/Fvdwn4buCvueKlgfcGjvFv9zvAa/Ovg/i3+W3gtfi3+R7go4FdntNx4LuBt+Lf5neA1+ZfB/Fv89vAa/Gv9zPAW/PC/TbwWvzr/Q7w2vzrIP5tfht4Lf71HgLcygv3YODp/Ov9DvDa/Osg/m1+G3gt/nX+BnhpXjR/DbwU/zq/A7w2/zqIf5vfBl6Lf53fAV6bF81vA6/Fv87vAK/Nvw7i3+a3gdfiX+d3gNfmRfPbwGvxr/M7wGvzr4P4t/lt4LX41zsB7PLCHQcu8q/3O8Br86+D+Lf5beC1+Nf7HOCzeeE+G/gs/vV+B3ht/nUQ/za/DbwW/zbvA3w3z997A9/Fv83vAK/Nvw7i3+a3gdfi3+67ga8B/porXgt4b+C9+bf7HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/ms4HP4n+WzwE+m38dxL/NWwM/xf8sbwP8NP86iH+7vwZeiv8Z/gZ4af71EP92DwZ+Gngp/nv9DfDWwK386yH+fY4DHw28N/Ag/ms9A/hu4KuBXf5tEP91Xpor/pr/ORD/dV4b+CngdYC/5n8GxH+d1wZ+C9gFXgf4a/77If7rvDbwW1yxC7wO8Nf890L813lt4Ld4tl3gdYC/5r8P4t/mq4GX4l/nOPDSPKdd4HWAv+Y5fTXwUvzr/A3w0fzrIP5tfht4Lf5j7AKvA/w1z/bbwGvxr/M7wGvzr4P4t/lt4LX4j7MLvA7w11zx28Br8a/zO8Br86+D+Lf5beC1+I9zCXht4K+54reB1+Jf53eA1+ZfB/Fv89vAa/Ef4xLw2sBf82y/DbwW/zq/A7w2/zqIf5uvBl6af53jwEvxnC4Brw38Nc/pq4GX5l/nr4GP5l8H8V/ntYHf4tkuAa8N/DX/fRD/dV4b+C2uuAS8NvDX/PdC/Nd5beC3gEvAawN/zX8/xH+d1wZ+Gnht4K/5nwHxX+elueKv+Z+DfwTnsrVBJe1mHQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCameraRear;
impl IconShape for MdCameraRear {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 20H5v2h5v2l3-3-3-3v2zm4 0v2h5v-2h-5zm3-20H7C5.9 0 5 .9 5 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V2c0-1.1-.9-2-2-2zm-5 6c-1.11 0-2-.9-2-2s.89-2 1.99-2 2 .9 2 2C14 5.1 13.1 6 12 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFV0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf57X417kE/DX/cyD+bY4DvwW8NP96nwN8Nv8zIP5tPhr4Kv7tXgb4a/77If5tfht4Lf7tXgf4bf77If5tfht4Lf7tXgf4bf77If5tfht4Lf7tXgf4bf77IV6448BHAe8NPJj/PH8NfDXwPfzXQrxgDwZ+C3gw/3W+G3gf/usgXrC/Al6a/3qfA3w2/zUQz99bAz/Ff49d4AT/NRDP32cDn8V/n9cBfpv/fIjn77eB1+K/z18Du/zH+hvgo3lOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+I/1iXgr3ler8Xz+htgl+d0HHgpntfv8LxeGjjGc/od4LV5Tojn77eB1+I/1u8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar81zQjx/vw28Fv+xfgd4bZ6XeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ4T4vn7beC1+I/1O8Br87zM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br85wQz99vA6/Ff6zfAV6b52We1+sAv81zem3gt3he4nn9NvBaPKffAV6b54R4/n4beC3+Y/0O8No8L/O8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/Ef6xLw1zyv1+J5/Q2wy3M6DrwUz+t3eF4vDRzjOf0O8No8J8Tz99vAa/Ef63eA1+Z5mef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Y5IZ6/3wZei/9YvwO8Ns/LPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns8J8fz9NvBa/Mf6HeC1eV7meb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eU6I5++3gdfiP9bvAK/N8zLP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/Nc0I8f78NvBb/sX4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev58G3or/Ph8D/DX/sXaBv+Y5IZ6/zwY+i/8+LwP8Nf/5EM/fSwN/xX+PZwAP5r8G4gX7auCj+K/3OsBv818D8cJ9NfBR/Ne4BLw38NP810H8y14aeGvgpYHj/Me7Ffhr4LuBXf5rIf5/Q/z/hvj/DfH/G+L/N/4RDW22QfXUSaMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCameraRoll;
impl IconShape for MdCameraRoll {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 5c0-1.1-.9-2-2-2h-1V2c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1v1H4c-1.1 0-2 .9-2 2v15c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2h8V5h-8zm-2 13h-2v-2h2v2zm0-9h-2V7h2v2zm4 9h-2v-2h2v2zm0-9h-2V7h2v2zm4 9h-2v-2h2v2zm0-9h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9Y4D3wUc519nF3gfYJf/OIj/WseB3wJemn+bvwZeB9jlPwbiv85x4LeAl+bf56+B1wF2+fdD/Nc4DvwW8NL8x/hr4HWAXf59EP/5jgO/Bbw0/7H+GngdYJd/O8R/vr8CXprn73f4lx0HXorn76+Bl+HfDvGfzzx/rwP8Ni+a9wa+i+dP/Nsh/vOZ5/U1wEfzr/PVwEfxvMS/HeI/n3lebwP8NP86bw38FM9L/Nsh/vOZ5/U6wG/zr/PawG/xvMS/HeI/n3lerwP8Nv86rw38Fs9L/Nsh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/IZ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDb/tyCev98GXovn9DvAa/N/C+L5+23gtXhOvwO8Nv+3IJ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDb/tyCev98GXovn9DvAa/N/C+L5+23gtXhOvwO8Nv+3IJ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDb/tyCev98GXovn9DvAa/N/C+L5+23gtXhOvwO8Nv+3IJ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDZXHAd2+d/lOLDLc0I8f78NvBbP6XeA1+aKjwa+G9jlf4+XBv6a54R4/n4beC2e0+8Ar80VLw18FvA2/O/w3sBvA7fynBDP328Dr8Vz+h3gtXm2rwb+Gvhu/md7b+ClgY/meSGev98GXovn9DvAa/Ocvhr4a+C7+Z/nOPBdwDOAj+b5Qzx/vw28Fs/pd4DX5nm9NFfcCuzyP8ODgbcGfhq4lRcM8fz9NvBaPKffAV6b/1sQz99vA6/Fc/od4LX5vwXx/P008FY8p13gr/nf4WOAv+Zfhnj+Phv4LP73ehngr/mXIZ6/lwb+iv+dngE8mBcN4gX7auCj+N/nbYCf5kWDeOG+Gvgo/ne4BHw08N286BD/spcG3hp4aeA4//PcCvw18N3ALv86iP/fEP+/If5/Q/z/hvj/jX8Egpd/QZh5r8EAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCases;
impl IconShape for MdCases {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 6V4l-2-2h-5L9 4v2H5v11s1 2 2 2h13s2-.98 2-2V6h-4zM4 9H2v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2H4V9zm7-4c0-.55.53-1 1-1h3c.46 0 1 .54 1 1v1h-5V5zM5 6h17v11c0 1.1-.9 2-2 2H7c-1.1 0-2-.9-2-2V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHjvOjEv4950e0CPw18DnAr/zLEi+a9ge/i30b8+5h/m/cBvpsXDvEvezDwdP7txL+P+bd7CHArLxjiX/bdwHvxbyf+fcy/3fcA780LhviXXQSO828n/n3Mv90ucIIXDPEvM8/rY4C/5kXz2/z7vDYvmpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/m3+Y48FbAg4EHc8WtwK3AzwC7/Nu8NvBbPC/xgiH+ZeZ5vQ7w2/zrPBj4LOC9eeG+G/gc4Fb+dV4b+C2el3jBEP8y87xeB/htXnTvDXwX/zrvA3w3L7rXBn6L5yVeMMR/vu8C3pt/m+8G3of/PIj/XO8NfBf/Pu8DfDf/ORD/eR4MPJ3/GA8BbuU/HuI/z3cD78V/jO8B3pv/eIj/HMeBi/zHOgHs8h8L8Z/jvYHv4j/W2wA/zX8sxH+OzwY+i/9YnwN8Nv+xEP85vht4L/5jfQ3w0fzHQvzn+G7gvfiP9TXAR/MfC/Gf47OBz+I/1ucAn81/LMR/jvcGvov/WG8D/DT/sRD/OY4DF/mPdQLY5T8W4j/PdwPvxX+M7wHem/94iP88Dwaezn+MhwC38h8P8Z/rvYHv4t/nfYDv5j8H4j/fdwPvxb/N9wDvzX8exL/MPK/XAX6bF917A9/Fv877AN/Ni+61gd/ieYkXDPEvM8/rdYDf5l/nwcBnA+/FC/c9wGcDt/Kv89rAb/G8xAuG+JeZ5/U6wG/zb3MceGvgwcCDueJW4K+B3wZ2+bd5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a140v8O/z2vxonlp4Kt5XuIFQ/zLdoFj/NuJfx/zb3cJOM4LhviXfTfwXvzbiX8f82/3PcB784Ih/mUPBp7Ov5349zH/dg8BbuUFQ7xo3hv4Lv5txL+P+bd5H+C7eeEQL7oHA58NvDVwjBed+PcxL7pLwE8Dnw3cyr8M8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E71mYQR7LI/4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCenterFocusStrong;
impl IconShape for MdCenterFocusStrong {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm-7 7H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHjvOjEv4950e0CPw18DnAr/zLEi+a9ge/i30b8+5h/m/cBvpsXDvEvezDwdP7txL+P+bd7CHArLxjiX/bdwHvxbyf+fcy/3fcA780LhviXXQSO828n/n3Mv90ucIIXDPEvM8/rY4C/5kXz2/z7vDYvmpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/m3+Y48FbAg4EHc8WtwK3AzwC7/Nu8NvBbPC/xgiH+ZeZ5vQ7w2/zrPBj4LOC9eeG+G/gc4Fb+dV4b+C2el3jBEP8y87xeB/htXnTvDXwX/zrvA3w3L7rXBn6L5yVeMMR/vu8C3pt/m+8G3of/PIj/XO8NfBf/Pu8DfDf/ORD/eR4MPJ0X7BnArVzxYOBBvGAPAW7lPx7iP893A+/F87oEvDfw0zyntwa+GzjG8/oe4L35j4f4z3EcuMjzugQ8GNjl+TsO3Aoc43mdAHb5j4X4z/HewHfxvN4G+GleuLcGforn9TbAT/MfC/Gf47OBz+I5PQN4MC+aW4EH8Zw+B/hs/mMh/nN8N/BePKffAV6bF81vA6/Fc/oa4KP5j4X4z/HdwHvxnH4HeG1eNL8NvBbP6WuAj+Y/FuI/x2cDn8VzuhV4CC+apwMP5jl9DvDZ/MdC/Od4b+C7eF5vA/w0L9xbAz/F83ob4Kf5j4X4z3EcuMjz2gUeAuzy/L008FvAcZ7XCWCX/1iI/zzfDbwXz2sXeB/gp3lObw18F3Cc5/U9wHvzHw/xn+fBwNN5wW4FbuWKBwMP5gV7CHAr//EQ/7neG/gu/n3eB/hu/nMg/vN9N/Be/Nt8D/De/OdB/MvM83od4Ld50b038F3867wP8N286F4b+C2el3jBEP8y87xeB/ht/nUeDHw28F68cN8DfDZwK/86rw38Fs9LvGCIf5l5Xq8D/Db/NseBtwYeDDyYK24F/hr4bWCXf5vXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/ro4G/5kXzO/z7vBYvmpcGvprnJV4wxL9sFzjGv5349zH/dpeA47xgiH/ZdwPvxb+d+Pcx/3bfA7w3LxjiX/Zg4On824l/H/Nv9xDgVl4wxIvmvYHv4t9G/PuYf5v3Ab6bFw7xonsw8NnAWwPHeNGJfx/zorsE/DTw2cCt/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EeZLbRB0E88/QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCenterFocusWeak;
impl IconShape for MdCenterFocusWeak {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 15H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zM12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1xvBbw28NrAS/Ov89fAbwO/DfwM/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DnwW8N3Cc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZVwMfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMGeDjyY/xv+GngZnhfi+Xsw8HT+bzkB7PKcEM/fawO/xf8trwP8Ns8J8fx9NPBV/N/yMcBX85wQz99nA5/F/y2fA3w2zwnx/H028Fn83/I5wGfznBDP328Dr8X/LT8DvDXPCfH8/TbwWvzf8jPAW/OcEM/fZwOfxf8tnwN8Ns8J8fx9NvBZ/N/yOcBn85wQz99HA1/F/y0fA3w1zwnx/L028Fv83/I6wG/znBDP34OBp/N/ywlgl+eEeMH+Gngp/m/4G+CleV6IF+yrgY/i/4avAT6a54V4wd4a+Cn+b3gd4Ld5XogX7lbgQfzv9gzgwTx/iBfus4HP4n+3zwE+m+cP8cIdB24FjvG/0yXgwcAuzx/iX/bZwGfxv9PnAJ/NC4b4lx0HbgWO8b/LJeDBwC4vGOJF897Ad/G/y/sA380Lh3jR/TTwVvzv8DPAW/MvQ7zojgN/DTyI/9meAbw0sMu/DPGv89LAbwPH+J/pEvDawF/zokH867008NvAMf5nuQS8NvDXvOgQ/zYvDfw2cIz/GS4Brw38Nf86iH+7lwZ+GngQ/72eAbw18Nf86yH+fY4D3w28Ff89fgZ4b2CXfxvEf4z3Br4aOMZ/jUvARwPfzb8P4j/OceCjgY8GjvGf4xLw1cBXA7v8+yH+4x0HPhp4b+BB/Md4BvDdwFcDu/zHQfznem3grYHXBl6Kf52/AX4b+Gngt/nPgfivcxx4aeClgeNc8dJc8ddcsQv8NfDXwC7/+RD/vyH+f0P8/4b4/w3x/xv/CK93kEE/1H0HAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCircle;
impl IconShape for MdCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCollections;
impl IconShape for MdCollections {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 16V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2zm-11-4l2.03 2.71L16 11l4 5H8l3-4zM2 6v14c0 1.1.9 2 2 2h14v-2H4V6H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCollectionsBookmark;
impl IconShape for MdCollectionsBookmark {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z",
            }
            path {
                d: "M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 10l-2.5-1.5L15 12V4h5v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3EceCvgwcBrAw8GHsxzuhW4FbgV+Gvgd4C/5j8X4j/PceC9gPcGXpp/m13gp4HvAX6b/3iI/3gPBj4KeG/gOP9xbgU+G/ge/uMg/mN9FvDRwHH+89wKfAzw0/z7If5jvDTwXcBL81/np4H3AXb5t0P8+7038FXAcZ7X3wA/DdwKvDTw3sAxXrhLwHcDfw08GHhv4EE8f7cCbwP8Nf82iH+f9wa+i+fve4D35jk9GPhp4KV4/v4GeGvgVp7TdwPvxfO3C7wO8Nf86yH+7d4b+C6ev78BXprn76WBv+L5exngr3n+bgUexPO3C7wN8Nv86yD+bd4b+C5esM8BPpsX7K+Bl+I5/Q3w0rxgnw18Fi/YLvA6wF/zokP867008FvAcV6w9wG+mxfst4HX4jn9DvDavGAfDXwVL9wu8BBglxcN4l/vr4CX5oX7GuCjecEuAsd5TrcCD+EF+2rgo/iX/TTwNrxoEP86nw18Fv+yXeBlgFt5Xh8NfBXP38cAX83zejDwV8BxXjSvA/w2/zLEi+7BwF8Bx3nR/DXwPsBf82wfBXw2cJznbxf4bOBreLaXBr4LeGledH8NvAz/MsSL7quBj+Jf76+BXeClgeO8aHaBvwaOAy/Nv83bAD/NC4d40RwHng4c53+PnwHemhcO8aL5aOCr+N/nBLDLC4Z40fwV8NI8r88Bfhr4a+C1gc8GXov/XL8DfDbw28BLA28NfBbP3/sA380LhviXHQcu8rxeB/htntd3A+/Ff47vAd6b5/XawG/xvL4G+GheMMS/7L2B7+I5fQ/w3jx/x4GL/Oc4Aezy/H038F48p98BXpsXDPEv+2rgo3hOHwN8NS/YbwOvxX+s3wFemxfso4Gv4jntAid4wRD/st8GXovn9DHAV/OC/TbwWvzH+h3gtXnBPhr4Kp6XeMEQ/7KnAw/mOX0P8N48f8eBi/znOAHs8vx9N/BePC/xgiH+Zeb5ex3gt3le3wW8N/85vht4H57XawO/xfN6BvBgXjDEv8y8YJ8N/Azw18BrAZ8NvDb/uX4b+Gzgd4CXBt4K+Gyev98BXpsXDPEvM/97fQ3w0bxgiH+Z+d/rbYCf5gVD/MtuBR7E/z6XgOO8cIh/2W8Dr8X/Pu8DfDcvHOJf9tXAR/G/yzOAB/MvQ/zL3hv4Lv73uAS8NvDX/MsQ/7LjwEX+93gb4Kd50SBeNH8NvBT/s10C3hr4bV50iBfNRwNfxf9cvwO8N3Ar/zqIF81x4FbgGP+z/Azw1cBv82+DeNF9NfBR/Nf4HZ6/3+aKW4GfBnb590G86B4M/DVwjP88XwN8NrDLfw3Ev85nA5/Ff46fAd6a/1qIf72/Bl6K/3gvA/w1/7UQ/3oPBv4aOMZ/LPFfD/Fv89bAT/Ef6yHArfzXQvzbvTfwXfzH+Rzgs/mvhfj3eW/gu/iPsQu8DHAr/3UQ/35vDXw3cIx/v+8B3pv/Ooj/GA8Gfhp4Kf59doET/NdB/Mf6bOCjgWP82zwDeDD/dRD/8R4MfDTw3sAx/nV+Bnhr/usg/vMcB94beG/gpfiXXQJeGriV/zqI/xrHgdcGXhp4beDBwIO44hnAXwMfDdzKfy3E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGEb8VBxatTAgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdColorLens;
impl IconShape for MdColorLens {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+mjgI8GjgPfDXwM/zaI/32+C3hvntN3A+/Dvx7if5fvAt6b5++7gffhXwfxv8d3Ae/NC/fdwPvwokP89/ou4HeA7+aF+y7gvXnRfDfwPrxoEP99vgt4b654H+C7ef6+C3hv/nW+G3gf/mWI/x7fBbw3z+l9gO/mOX0X8N7823w38D68cIj/et8FvDfP3/sA380V3wW8N/8+XwN8NC8Y4r/eewPfxQv2PsBrAe/Nv98ucIIXDPHf472B7+I/3zOAB/OCIf77vDfwXfzneh/gu3nBEP+9Phv4LP5zvA/w3bxwiP8+Lw38FnCc/3jvA3w3/zLEf4+XBn4LOM5/vPcBvpsXDeK/3ksDvwUc5z/e+wDfzYsO8V/rpYHfAo7zH+99gO/mXwfxX+elgd8CjvP8vQ/w2sB78a/3PsB386+H+K/x0sBvAcd5/t4H+G6u+G7gvXjRvQ/w3fzbIP7zvTTwW8Bxnr/3Ab6b5/TdwHvxL3sf4Lv5t0P853pp4LeA4zx/7wN8N8/fdwPvxQv3PsB382+H+M/z0sBvAcd5/t4H+G5euO8G3osX7n2A7+bfBvGf46WB3wKO8/y9D/Dd/MveG/gu/mXvA3w3/3qI/3gvDfwWcJzn732A7+Zf9t7Ad/Giex/gu/nXQfzHemngt4DjPH/vA3w3/7L3Br6Lf733Ab6bFx3iP85LA78FHOf5ex/gu/mXvTfwXTx/l4CvBj6L5+97gPfmRYf4j/HSwG8Bx3n+3gf4bv5l7w18F8/fJeC1gb8G3hv4Lp7T9wDvzb8O4t/vpYHfAo7z/L0P8N38y94b+C6ev0vAawN/zbO9N/BdXPE9wHvzr4f493lp4LeA4zx/7wN8N/+y9wa+i+fvEvDawF/zvN4beG3gvfm3QfzbvTTwW8Bxnr/3Ab6bf9l7A9/F83cJeG3gr/nPgfi3eWngt4DjPH/vA3w3/7L3Br6L5+8S8NrAX/OfB/Gv99LAbwHHef7eB/hu/mXvDXwXz98l4LWBv+Y/F+Jf56WB3wKO8/y9D/Dd/MveG/gunr9LwGsDf81/PsSL7qWB3wKO8/y9D/Dd/MveG/gunr9LwGsDf81/DcSL5qWB3wKO8/y9D/Dd/MveG/gunr9LwGsDf81/HcS/7KWB3wKO8/y9D/Dd/MveG/gunr9LwGsDf81/LcS/7CJwnOfvfYDv5l/23sB38fxdAl4b+Gv+6yH+Zeb5ex/gu/mXvTfwXTx/l4DXBv6a/x6If5l5Xt8DvDf/svcGvovn7xLw2sBf898H8S8zz9/7AN/NC/bewHfx/F0CXhv4a/57If5l5gV7H+C7eV7vDXwXz98l4LWBv+a/H+JfZl649wG+m2d7b+C7eP4uAa8N/DX/MyD+ZeZf9j7AdwPvDXwXz98l4LWBv+Z/DsS/zLxovht4b56/S8BrA3/N/yyIf5n597kEvDbw1/zPg/iXmX+7S8BrA3/N/0yIf5n5t7kEvDbw1/zPhfiXmX+9S8BrA3/N/2yIf5n517kEvDbw1/zPh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Am1Du0HhQzrlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdColorize;
impl IconShape for MdColorize {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.71 5.63l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-3.12 3.12-1.93-1.91-1.41 1.41 1.42 1.42L3 16.25V21h4.75l8.92-8.92 1.42 1.42 1.41-1.41-1.92-1.92 3.12-3.12c.4-.4.4-1.03.01-1.42zM6.92 19L5 17.08l8.06-8.06 1.92 1.92L6.92 19z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCompare;
impl IconShape for MdCompare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h5v2h2V1h-2v2zm0 15H5l5-6v6zm9-15h-5v2h5v13l-5-6v9h5c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdControlPoint;
impl IconShape for MdControlPoint {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdControlPointDuplicate;
impl IconShape for MdControlPointDuplicate {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 8h-2v3h-3v2h3v3h2v-3h3v-2h-3zM2 12c0-2.79 1.64-5.2 4.01-6.32V3.52C2.52 4.76 0 8.09 0 12s2.52 7.24 6.01 8.48v-2.16C3.64 17.2 2 14.79 2 12zm13-9c-4.96 0-9 4.04-9 9s4.04 9 9 9 9-4.04 9-9-4.04-9-9-9zm0 16c-3.86 0-7-3.14-7-7s3.14-7 7-7 7 3.14 7 7-3.14 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf5qWBYzynS8Bf878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE6/A7w2/7sg/m1+G3gtntPvAK/N/y6If5vfBl6L5/Q7wGvzvwvi3+a3gdfiOf0O8Nr874L4t/lt4LV4Tr8DvDb/uyD+bX4beC2e0+8Ar83/Loh/m98GXovn9DvAa/O/C+Lf5reB1+I5/Q7w2vzvgvi3+W3gtXhOvwO8Nv+7IJ7tpYGv4kXz0sBxntPvAK/N8/ot/uPtAn8N/Azw1/zbIZ7ttYHf4t/ud4DX5nmZ/1xfDXwM/zaIZ3tt4Lf4t/sd4LV5XuY/39cAH82/HuLZXhv4Lf7tfgd4bZ6X+a/xEOBW/nUQz/bawG/xb/c7wGvzvMx/jc8BPpt/HcSzvTTw1bxoXho4xnP6HeC1eV6/zX+8lwaO8Zx+Bnhr/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/XRD/Nr8NvBbP6XeA1+Z/F8S/zW8Dr8Vz+h3gtfnfBfFv89vAa/Gcfgd4bf53Qfzb/DbwWjyn3wFem/9dEP82Pw28Fc9pF/hr/nf4GOCvAcS/zWcDn8X/Xq8D/DaA+Ld5aeCv+N/rdYDfBhD/dl8NfBT/O70O8NsA4t/nq4GP4n+f1wF+G0D8+7008NbASwPH+a/x0sAxntMl4K950Xw08NcA4n+n3wZei+f0O8Br86+D+N/pt4HX4jn9DvDa/Osg/nf6beC1eE6/A7w2/zqI/51+G3gtntPvAK/Nvw7if6ffBl6L5/Q7wGvzr4P43+m3gdfiOf0O8Nr86yD+d/pt4LV4Tr8DvDb/Ooj/nX4beC2e0+8Ar82/DuJ/p98GXovn9DvAa/Ovg/jf6beB1+I5/Q7w2vzrIP53emngOM9pF/hr/nUQ/78h/n/jHwGTwLRB43QZpgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCrop;
impl IconShape for MdCrop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 15h2V7c0-1.1-.9-2-2-2H9v2h8v8zM7 17V1H5v4H1v2h4v10c0 1.1.9 2 2 2h10v4h2v-4h4v-2H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADS0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeNE9GPgs4K2B4/zPtAv8NPA5wK38yxAvmvcGvov/Xd4H+G5eOMS/7MHA0/nf6SHArbxgiH/ZdwPvxf9O3wO8Ny8Y4l92ETjO/067wAleMMS/zDyvjwH+mv9ZXhr4Kp6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87w+Gvhr/md5aeCreV7iBUP8y3aBY/zvdAk4zguG+Jd9N/Be/O/0PcB784Ih/mUPBp7O/04PAW7lBUO8aN4b+C7+d3kf4Lt54RAvugcDnw28NXCM/5kuAT8NfDZwK/8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ESemWkE/XewnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCrop169;
impl IconShape for MdCrop169 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 6H5c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 10H5V8h14v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+Z+d9NvGCIf9lfAy/F/06/A7w2LxjiX/bewHfxv9P7AN/NC4Z40Xw38F787/I9wHvzwiFedJ8NfDRwjP/ZLgFfDXw2/zLEv95r8z/bb/OiQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BPFTdkH5Sk8EAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCrop32;
impl IconShape for MdCrop32 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 4H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H5V6h14v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zpvBbw08Nr8z/TbwF8DP8OLBvGieTDwU8BL87/DXwNvA9zKC4f4lx0H/gp4MP+7/DXwOsAuLxjiX/bZwGfxv9PnAJ/NC4b4lz0deDD/O90KPIQXDPEvM8/rb4Bd/mc5DrwUz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef118Au/7McB16a5yVeMMS/7FbgQfzv9AzgwbxgiH/ZZwOfxf9OnwN8Ni8Y4l92HPht4KX43+VvgNcGdnnBEC+aBwM/DbwU/zv8DfDWwK28cIh/nbcGXhp4bf5n+m3gr4Gf5kWD+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHQVpuQcPi6OcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCrop54;
impl IconShape for MdCrop54 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 5H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 12H5V7h14v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/vtfif7Xd40SFedJ8FfDRwnP/ZdoGvBj6HfxniRfNdwHvzv8t3A+/DC4f4l7038F387/Q+wHfzgiH+ZX8FvDT/O/0O8Nq8YIh/mfnfTbxgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5n53028YIh/2V8DL8X/Tr8DvDYvGOJf9t7Ad/G/0/sA380LhnjRfDfwXvzv8j3Ae/PCIV50nw18NHCM/9kuAV8NfDb/MsS/3mvzP9tv86JD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BGEuRkGQ2VrEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCrop75;
impl IconShape for MdCrop75 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 7H5c-1.1 0-2 .9-2 2v6c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zm0 8H5V9h14v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a/5neWngq3le4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lDwaezv9ODwFu5QVDvGjeG/gu/nd5H+C7eeEQL7oHA58NvDVwjP+ZLgE/DXw2cCv/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EbfLikGVIMc5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropDin;
impl IconShape for MdCropDin {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHjvOjEv4950e0CPw18DnAr/zLEi+a9ge/i30b8+5h/m/cBvpsXDvEvezDwdP7txL+P+bd7CHArLxjiX/bdwHvxbyf+fcy/3fcA780LhviXXQSO828n/n3Mv90ucIIXDPEvM8/rY4C/5kXz2/z7vDYvmpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK+PBv6aF83v8O/zWrxoXhr4ap6XeMEQ/7Jd4Bj/duLfx/zbXQKO84Ih/mXfDbwX/3bi38f8230P8N68YIh/2YOBp/NvJ/59zL/dQ4BbecEQL5r3Br6Lfxvx72P+bd4H+G5eOMSL7sHAZwNvDRzjRSf+fcyL7hLw08BnA7fyL0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwF25mZBC78g+gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropFree;
impl IconShape for MdCropFree {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5v4h2V5h4V3H5c-1.1 0-2 .9-2 2zm2 10H3v4c0 1.1.9 2 2 2h4v-2H5v-4zm14 4h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zm0-16h-4v2h4v4h2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zpvBbw08Nr8z/TbwF8DP8OLBvGieTDwU8BL87/DXwNvA9zKC4f4lx0H/gp4MP+7/DXwOsAuLxjiX/bZwGfxv9PnAJ/NC4b4lz0deDD/O90KPIQXDPEvM8/rb4Bd/mc5DrwUz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef118Au/7McB16a5yVeMMS/7FbgQfzv9AzgwbxgiH/ZZwOfxf9OnwN8Ni8Y4l92HPht4KX43+VvgNcGdnnBEC+aBwM/DbwU/zv8DfDWwK28cIh/nbcGXhp4bf5n+m3gr4Gf5kWD+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHQVpuQcPi6OcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropLandscape;
impl IconShape for MdCropLandscape {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 5H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 12H5V7h14v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/8c7w28F/A6/Ou8NvBbPC/xgiH+ZeZ5vQ7w2/zHe2ngt4DjwOcAn82L7rWB3+J5iRcM8S8zz+t1gN/mP9Zx4LeAl+bZXgf4bV40rw38Fs9LvGCIf5l5Xq8D/DYvmt8CPgb4a164nwLemue0CzwE2OVf9trAb/G8xAuG+JeZ5/U6wG/zL/tq4KOAW4GXAXZ5/j4a+Cqev98GXod/2WsDv8XzEi8Y4l9mntfrAL/NC/fewHfxbH8NvAzP67WB3+KF+xzgs3nhXhv4LZ6XeMEQ/zLzvF4H+G1esJcGfgs4znP6buB9eLbjwNOB4/zL3gb4aV6w1wZ+i+clXjDEv8w8r9cBfpvn7zjwW8BL8/x9DPDVXPFXwEvzotkFXga4lefvtYHf4nmJFwzxLzPP63WA3+b5+yngrXnh3gZ4beCj+Nf5a+BleP5eG/gtnpd4wRD/MvO8Xgf4bZ7XRwNfxb9sFzjOv83XAB/N83pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwW/zXeBvhpntNrA7/F8xIvGOJfZp7X6wC/zbMdB54OHOe/xi7wMsCtPNtrA7/F8xIvGOJfZp7X6wC/zbM9GHgw/7VuBW7l2V4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvD4a+Gv+Z3lp4Kt5XuIFQ/zLdoFj/O90CTjOC4b4l3038F787/Q9wHvzgiH+ZQ8Gns7/Tg8BbuUFQ7xo3hv4Lv53eR/gu3nhEC+6BwOfDbw1cIz/mS4BPw18NnAr/zLE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hEJVbNB77m8jwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropOriginal;
impl IconShape for MdCropOriginal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-5.04-6.71l-2.75 3.54-1.96-2.36L6.5 17h11l-3.54-4.71z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zkOfBTw3sCD+a91K/DdwNcAu/zbIP7tHgz8FvBg/nv9NfA2wK386yH+7f4KeGn+Z/hr4GX410P827w18FP8z/I2wE/zr4P4t/ls4LP4n+VzgM/mXwfxb/PbwGvxnP4G+Gj+a3w18FI8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+mvgo/mv8dXAS/Ocfgd4bf51EP82nw18Fv+zfA7w2fzrIP5t3hr4Kf5neRvgp/nXQfzb/TXwUvzP8DfAS/Ovh/i3ezDw08BL8d/rb4C3Bm7lXw/x73Mc+GjgvYEH8V/rGcB3A18N7PJvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AsuEskENv0xLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropPortrait;
impl IconShape for MdCropPortrait {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 3H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropRotate;
impl IconShape for MdCropRotate {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.47 21.49C4.2 19.93 1.86 16.76 1.5 13H0c.51 6.16 5.66 11 11.95 11 .23 0 .44-.02.66-.03L8.8 20.15l-1.33 1.34zM12.05 0c-.23 0-.44.02-.66.04l3.81 3.81 1.33-1.33C19.8 4.07 22.14 7.24 22.5 11H24c-.51-6.16-5.66-11-11.95-11zM16 14h2V8c0-1.11-.9-2-2-2h-6v2h6v6zm-8 2V4H6v2H4v2h2v8c0 1.1.89 2 2 2h8v2h2v-2h2v-2H8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+7BwIP4n+EZwK386yH+9Y4D3wW8Nf+z/DTwPsAuLzrEv85x4OnAcf5n2gUeAuzyokH86/wU8Nb8z/bTwNvwokG86B4MPJ3/HR4C3Mq/DPGie23gt/jf4XWA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fv87/A6wG/zL0O86B4MPJ3/HR4C3Mq/DPGv89PAW/E/288Ab82LBvGvcxy4FTjG/0x/A7w2sMuLBvGvdxz4buCt+J/lZ4D3BnZ50SH+7R4MPJj/GW4FbuVfD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+Eb5EfEEBdlYaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCropSquare;
impl IconShape for MdCropSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 4H6c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H6V6h12v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACNElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78hnr+XBo7xf8sl4K95Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54T4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvpYHj/N+yC/w1zwnx/xvi/zfE/2+I/98Q/78h/n9D/P+GeP6+Gngp/m/5G+CjeU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvq4GX5v+WvwY+mueE+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BExVJEF7YeOvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDehaze;
impl IconShape for MdDehaze {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 15.5v2h20v-2H2zm0-5v2h20v-2H2zm0-5v2h20v-2H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDetails;
impl IconShape for MdDetails {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3L2,21h20L12,3z M13,8.92L18.6,19H13V8.92z M11,8.92V19H5.4L11,8.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirtyLens;
impl IconShape for MdDirtyLens {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.95 19H20V7H4v12h7.24c.14-.98.42-2.05-.16-2.43-.89-.59-1.27 2.06-2.8 1.35-1.39-1.12 1.05-1.29.5-3.27-.22-.79-2.28.36-2.4-1.24-.08-1 1.49-.74 1.51-1.49.03-.75-1.03-1.05-.25-1.91.22-.24.71-.26.91-.19.79.27 1.55 1.82 2.51 1.19 1.03-.66-1.88-2.35 0-2.86 1.64-.44 1.31 2.08 2.65 2.44 1.94.52 2.65-4.55 4.41-2.33 1.85 2.33-3.43 2.27-2.85 4.01.34 1.01 2.15-1.2 2.76.53.64 1.83-3.09.82-3.04 1.66.06.83 2.41.55 1.64 2.12-1.14 1.86-3-1.03-3.81.09-.39.57-.09 1.49.13 2.33zM20 5c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V7c0-1.1.9-2 2-2h3.17L9 3h6l1.83 2H20zm-1.86 13.01c-.47 0-.86-.38-.86-.86s.38-.86.86-.86c.47 0 .86.38.86.86s-.38.86-.86.86z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+mjgI8GjgPfDXwM/zaI/32+C3hvntN3A+/Dvx7if5fvAt6b5++7gffhXwfxv8d3Ae/NC/fdwPvwokP87/BdwHvzovlu4H140SD+5/ts4LP41/lu4H34lyH+5zsO/DbwUvzrfDfwPrxwiP8djgO/DbwU/zpfA3w0Lxjif6bXBn6b53Qc+G3gpXjR7QIneMEQ//N8F/DewPsA381zOg78NvBSvGieATyYFwzxP8t3Ae/Ns70P8N08p+PAbwMvxb/sfYDv5gVD/M/xXcB787zeB/huntNx4LeBl+IFex/gu3nhEP8zfBfw3rxg7wN8N8/pOPDbwEvxvN4H+G7+ZYj/ft8FvDf/svcBvpvndBz4beCleLb3Ab6bFw3iv9d3Ae/Ni+59gO/mOR0Hfht4KeB9gO/mRYf47/NdwHvzr/c+wHfznI4Drw38NP86iP8e3wW8N/927wN8N/9+iP963wW8N/9+7wN8N/8+iP9a3wW8N/9xHgLcyr8d4r/OdwHvzX+c9wG+m38fxH+N7wLem/847wN8N/9+iP983wW8N/9x3gf4bv5jIP5zfRfw3vzHeR/gu/mPg/jP813Ae/Mf532A7+Y/FuI/x3cB781/nPcBvpv/eIj/eN8FvDf/cd4H+G7+cyD+Y30X8N78x3kf4Lv5z4P4j/NdwHvzH+d9gO/mPxfiP8Z3Ae/Nf5z3Ab6b/3yIf7/vAt6b/zjvA3w3/zUQ/z7fBbw3/3HeB/hu/usg/u2+C3hv/uO8D/Dd/NdC/Nt8F/De/Md5H+C7+a+H+Nf7LuC9+Y/zPsB3898D8a/zXcB78x/nfYDv5r8P4kX3XcB78x/nfYDv5r8X4kXzXcB78x/nfYDv5r8f4l/23sB38R/nfYDv5n8GxL/M/Md5H+C7+Z8D8S8z/zHeB/hu/mdB/MvMv9/7AN/N/zyIf5n593kf4Lv5nwnxLzP/du8DfDf/cyH+Zebf5n2A7+Z/NsS/zPzrvQ/w3fzPh/iXmX+d9wG+m/8dEP8y86J7H+C7+d8D8S8zL5rvAd6b/10Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcMCG5BedOXfwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEdit;
impl IconShape for MdEdit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+E48FrAawMvzRUPBh7Ms90K3Ar8NfDbwN8At/Lvg/jvcxx4K+CjgZfm3+avge8GvgfY5V8P8V/vOPBZwHsDx/mPsQv8NPAxwC4vOsR/rY8CPhs4zn+OXeCrgc/hRYP4r/HSwFcBr81/jb8G3ga4lRcO8Z/vpYHfAo7zX2sXeB/gp3nBEP+53hv4Lv71/gbY5Tm9NHCMf53vAd6bFwzxn+e9ge/iRXMJ+Gngu4G/BnZ5/o4Drw28NfDWwDFesO8B3psXDvGf46WBv+Jf9gzgs4Hv5l/vOPDRwHsDD+I5fQ/w3vzLEP/xHgz8FXCcF+5zgK8Gdvn3+2rgo7jie4D35kWD+I/3W8Br84JdAt4a+G3+Y7018NrAR/OiQ/zH+mjgq3jBLgGvDfw1/zMg/uMcB54OHOf5uwS8NvDX/M+B+I/z2cBn8YK9DvDb/M+C+I9zETjO8/c5wGfzPw/iP8Z7A9/F8/cM4KWBXf7nQbxwrw38Fv8zvA/w3fzHQrxwvwW8Nv/9ngE8mP94iBfstYHf4n+G9wG+m/94iBfst4DX5r/fM4AH858D8fy9NvBb/M/wPsB3858D8fw9GHgwL5rvAh7M8/ob4KP59/tt/vMg/v3M8/c7wGvzPxvi3888f78DvDb/syH+/czz9zvAa/M/G+Lfzzx/vwO8Nv+zIf5lr8UL993Ag3lefw18NP9xngHcyn8sxAv33sB38T/D6wC/zX8sxAv3dODB/Pf7HeC1+Y+HeMHeG/gu/md4HeC3+Y+HeMGeDjyY/36/A7w2/zkQz997A9/F/wyvA/w2/zkQ/zHeG/gunr9bgZcBdvmfB/EfZxc4xvP3OcBn8z8P4j/OZwOfxQv2OsBv8z8L4j/OceBW4BjP3y7wOsBf8z8H4j/WRwNfxQu2C7wO8Nf8z4D4j/fbwGvxgu0CbwP8Nv+x3hp4LeBjeNEh/uM9GPhr4Bgv3GcDXwPs8u9zHPgs4KO54ruB9+FFg/jP8dLAX/EvuxX4bOB7+Nc7DnwU8NHAcZ7TdwPvw78M8Z/nvYHv4kWzC/w08N3A3wC7PH/HgdcC3hp4a+A4L9h3A+/DC4f4z/XewHfxr3crcCvP6aWB4/zrfDfwPrxgiP98Lw38NnCM/1qXgPcGfpoXDPFf46WBrwZei/8afwO8NXArLxziv9ZHA58NHOM/xyXgq4HP5kWD+K93HPhs4L2BY/zHuAR8N/DZwC4vOsR/r/cGPhp4Kf5t/gb4buC7gV3+9RD/MxwHXht4aeC1ueLBwIN4tmcAtwJ/Dfw28NfArfz7IP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPLISdQZi5/jcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEuro;
impl IconShape for MdEuro {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15,18.5c-2.51,0-4.68-1.42-5.76-3.5H15l1-2H8.58c-0.05-0.33-0.08-0.66-0.08-1s0.03-0.67,0.08-1H15l1-2H9.24 C10.32,6.92,12.5,5.5,15,5.5c1.61,0,3.09,0.59,4.23,1.57L21,5.3C19.41,3.87,17.3,3,15,3c-3.92,0-7.24,2.51-8.48,6H3l-1,2h4.06 C6.02,11.33,6,11.66,6,12s0.02,0.67,0.06,1H3l-1,2h4.52c1.24,3.49,4.56,6,8.48,6c2.31,0,4.41-0.87,6-2.3l-1.78-1.77 C18.09,17.91,16.62,18.5,15,18.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O/wPcB78bzEC4b4l5n/+d4HuBX4LZ6XeMEQ/zLzP9v7AN8NvDbwWzwv8YIh/mXmf673Ab6bK14b+C2el3jBEP8y8z/T+wDfzbO9NvBbPC/xgiH+ZeaF+x3+9V4aOMa/3fsA381zem3gt3he4gVD/MvMCyf+9X4beC3+bd4H+G6e12sDv8XzEi8Y4l9mXjjxr/fbwGvxr/c+wHfz/L028Fs8L/GCIf5l5oUT/3q/DbwW/zrvA3w3L9hrA7/F8xIvGOJfZl641+Zf76uBl+ZF9z7Ad/PCvTbwWzwv8YIh/mXmv9f7AN/Nv+y1gd/ieYkXDPEvM/993gf4bl40rw38Fs9LvGCIf5n57/E+wHfzontt4Ld4XuIFQ/zLzH+99wG+m3+d1wZ+i+clXjDEv8z813of4Lv513tt4Ld4XuIFQ/zLzH+d9wG+m3+b1wZ+i+clXjDEv8z813gf4Lv5t3tt4Ld4XuIFQ/zLzH++9wG+m3+f1wZ+i+clXjDEv8z853of4Lv593tt4Ld4XuIFQ/zLzH+e9wG+m/8Yrw38Fs9LvGCIf5n5z/E+wHfzgr008FU8298AH80L9trAb/G8xAuG+JeZ/3jvA3w3L9xrA7/Fs/0O8Nq8YK8N/BbPS7xgiH+Z+Y/1PsB38y97beC3eLbfAV6bF+y1gd/ieYkXDPEvM/9x3gf4bl40rw38Fs/2O8Br84K9NvBbPC/xgiH+ZeY/xvsA382L7rWB3+LZfgd4bV6w1wZ+i+clXjDEv8z8+70P8N28YC8NHOM5vTTw1TzbXwMfzfP6Ha54beC3eF7iBUP8y8y/z/sA380L99vAa/FvI654beC3eF7iBUP8y8y/3fsA382/7LeB1+LfRlzx2sBv8bzEC4b4l5l/m/cBvpsXzW8Dr8W/jbjitYHf4nmJFwzxLzP/eu8DfDcvuq8GXprndBx4KZ7tEvDXPK/X5orXBn6L5yVeMMS/zPzrvA/w3fz7vTbwWzzb7wCvzQv22sBv8bzEC4b4l5kX3fsA381/jNcGfotn+x3gtXnBXhv4LZ6XeMEQ/zLzonkf4Lv5j/PawG/xbL8DvDYv2GsDv8XzEi8Y4l9m/mXvA3w3/7FeG/gtnu13gNfmBXtt4Ld4XuIFQ/zLzAv3PsB38x/vtYHf4tl+B3htXrDXBn6L5yVeMMS/zLxg7wN8N/8zvDbwWzwv8YIh/mXm+Xsf4Lv5n+O1gd/ieYkXDPEvM8/ru4Hv5n+Wlwa+muclXjDEv2wXOMb/TpeA47xgiH/ZdwPvxf9O3wO8Ny8Y4l/2YODp/O/0EOBWXjDEi+a9ge/if5f3Ab6bFw7xonsw8NnAWwPH+J/pEvDTwGcDt/IvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAUwlq0HafkjfAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposure;
impl IconShape for MdExposure {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM6 7h5v1.5H6V7zm13 12H5L19 5v14zm-4.5-3v2H16v-2h2v-1.5h-2v-2h-1.5v2h-2V16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADa0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//u9NPBegICP5l8H8b/Tg4G3At4beGmu+B3gtfnXQfzvcRx4K+Ctgbfmef0O8Nr86yD+53sr4K2BtwaO84L9DvDa/Osg/md6aeC9gPcGjvOi+R3gtfnXQfzP9NvAa/Gv8zvAa/Ovg/if6beB1+L5u8QVx3hOvwO8Nv86iP+Zfht4LZ7tEvDTwE8DPw38NvBaPKffAV6bfx3E/0y/Dbw08NPATwM/zXP6beC1eE6/A7w2/zqI/5leG/hrYJfn77eB1+I5/Q7w2vzrIP53+m3gtXhOvwO8Nv86iP+dfht4LZ7T7wCvzb8O4n+n3wZei+f0O8Br86+D+N/pt4HX4jn9DvDa/Osg/nf6beC1eE6/A7w2/zqI/51+G3gtntPvAK/Nvw7iRffSwFfxH+91+Nf7beC1eE6/A7w2/zqIF91rA7/Ffzzxr/fbwGvxnH4HeG3+dRAvutcGfov/eOJf77eB1+I5/Q7w2vzrIF50rw38Fv/xxL/ebwOvxXP6HeC1+ddBvOheG/gt/uOJf73fBl6L5/Q7wGvzr4N40b008NX8x3tt/vV+G3gtntPvAK/Nvw7if6ffBl6L5/Q7wGvzr4P43+m3gdfiOf0O8Nr86yD+d/pt4LV4Tr8DvDb/Ooj/nX4beC2e0+8Ar82/DuJ/p98GXovn9DvAa/Ovg/jf6beB1+I5/Q7w2vzrIP53+m3gtXhOvwO8Nv86iP+dfht4LZ7T7wCvzb8O4n+n3wZei+f0O8Br86+D+N/pt4HX4jn9DvDa/Osg/nf6beC1eE6/A7w2/zqI/51+G3gtntPvAK/Nvw7if6ffBl6L5/Q7wGvzr4P4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8Cu8pVQaFTzVEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposureNeg1;
impl IconShape for MdExposureNeg1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 11v2h8v-2H4zm15 7h-2V7.38L14 8.4V6.7L18.7 5h.3v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//O9FvDaXPFgrriVK34b+B3+7RD/Mz0Y+CzgrYHjvHC7wE8DnwPcyr8O4n+e9wa+CjjOv84u8DHAd/OiQ/zP8l3Ae/Pv8z7Ad/OiQfzP8dHAV/Ef43WA3+Zfhvif4cHA03n+/gb4bOCnebYHAx8NfBTP328Dr8O/DPE/w3cD78Xz+h7gvXnB3hv4Lp6/hwC38sIh/vsdBy7yvJ4BvDSwywv328Br8bw+BvhqXjjEf7+3Bn6K5/U1wEfzL3tr4Kd4Xp8DfDYvHOK/32cDn8Xzeh3gt/mXvTTwVzyvnwHemhcO8d/vs4HP4nk9BLiVF415Xl8DfDQvHOJ/v9cGfovn9TnAZ/PCIf73+2jgq3herwP8Ni8c4n+/pwMP5jk9A3gw/zLE/24fDXwVz+tzgM/mX4b43+vBwF8Bx3lOl4AHA7v8yxDP31cDL8V/rL8BPpr/GMeB3wJemuf1McBX86JBPH+/DbwW/7F+B3ht/mN8F/DePK/fAV6bFx3i+ftt4LX4j/U7wGvz7/ddwHvzvC4BDwZ2edEhnr/fBl6L/1i/A7w2/z7vDXwXz9/LAH/Nvw7i+ftt4LX4j/U7wGvzb/fewHfx/L0P8N386yGev68GXpr/WH8NfDT/Nu8NfBfP3/sA382/DeJ/vpcGfgs4zvP6GuCj+bdD/M/20sBvAcd5Xt8DvDf/Poj/uV4a+C3gOM/re4D35t8P8T/TceC3gJfmef0M8Nb8x0D8z3Mc+C3gpXlefwO8NrDLfwzE/yzHgd8CXprn9TfAawO7/MdB/M/yXcB787z+BnhtYJf/WIj/Ob4LeG+e1yXgwcAu//EQ/zN8F/DePH8/Dfw1/3afwwuG+J/B/OcRLxjifwbzn0e8YIj/Gcx/HvGCIf5nMP95xAuG+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AgU8Z0FSxFbSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposureNeg2;
impl IconShape for MdExposureNeg2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.05 16.29l2.86-3.07c.38-.39.72-.79 1.04-1.18.32-.39.59-.78.82-1.17.23-.39.41-.78.54-1.17s.19-.79.19-1.18c0-.53-.09-1.02-.27-1.46-.18-.44-.44-.81-.78-1.11-.34-.31-.77-.54-1.26-.71-.51-.16-1.08-.24-1.72-.24-.69 0-1.31.11-1.85.32-.54.21-1 .51-1.36.88-.37.37-.65.8-.84 1.3-.18.47-.27.97-.28 1.5h2.14c.01-.31.05-.6.13-.87.09-.29.23-.54.4-.75.18-.21.41-.37.68-.49.27-.12.6-.18.96-.18.31 0 .58.05.81.15.23.1.43.25.59.43.16.18.28.4.37.65.08.25.13.52.13.81 0 .22-.03.43-.08.65-.06.22-.15.45-.29.7-.14.25-.32.53-.56.83-.23.3-.52.65-.88 1.03l-4.17 4.55V18H21v-1.71h-5.95zM2 11v2h8v-2H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEHklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7e8FfDWwNcAf82/DPG/30sD7wW8N3CcK14H+G3+ZYj/nR4MvBXw0cCDeV6vA/w2/zLE/x7HgbcCPhp4aV641wF+m38Z4n++twLeG3hrXnSvA/w2/zLEf46XBo7xnC4Bf82/nvnXex3gt/mXIf5z/DbwWjyn3wFem38984I9A3gQz+t1gN/mX4b4z/HbwGvxnH4HeG3+9cxzegbw08B3A38NmOf1OsBv8y9D/Of4beC1eE6/A7w2/3oGngH8NPDdwF/znMzzeh3gt/mXIf5z/DbwWjyn3wFem3+9lwb+mhfMPK/XAX6bfxniP8dvA6/Fc/od4LX5j2ee1+sAv82/DPGf47eB1+I5/Q7w2vzHM8/rdYDf5l+G+M/x28Br8Zx+B3ht/uOZ5/U6wG/zL0P85/ht4LV4Tr8DvDb/8czzeh3gt/mXIf5z/DbwWjyn3wFem/945nm9DvDb/MsQ/zl+G3gtntPvAK/NfzzzvF4H+G3+ZYgX3UsDX8WL5qWB4zynXeCvedF8DPDXvGjM83od4Lf5lyFedK8N/Bb/NV4H+G1eNOZ5vQ7w2/zLEC+61wZ+i/8arwP8Ni8a87xeB/ht/mWIF91rA7/Ff43XAX6bF415Xq8D/Db/MsSL7rWB3+K/xusAv82Lxjyv1wF+m38Z4kX30sBX86J5aeAYz+kS8Ne8aD4a+GteNOZ5vQ7w2/zLEP85fht4LZ7T7wCvzX8887xeB/ht/mWI/xy/DbwWz+l3gNfmP555Xq8D/Db/MsR/jt8GXovn9DvAa/Mfzzyv1wF+m38Z4j/HbwOvxXP6HeC1+Y9nntfrAL/Nvwzxn+O3gdfiOf0O8Nr8xzPP63WA3+ZfhvjP8dvAa/Gcfgd4bf7jmef1OsBv8y9D/Of4beC1eE6/A7w2//HM83od4Lf5lyH+c/w28Fo8p98BXpv/eOZ5vQ7w2/zLEP85fht4LZ7T7wCvzX8887xeB/ht/mWI/xy/DbwWz+l3gNfmP555Xq8D/Db/MsR/jpcGjvOcdoG/5j+eeV6vA/w2/zLE/37meb0O8Nv8yxD/+5nn9TrAb/MvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEIvoxBLlfj9AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposurePlus1;
impl IconShape for MdExposurePlus1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 7H8v4H4v2h4v4h2v-4h4v-2h-4V7zm10 11h-2V7.38L15 8.4V6.7L19.7 5h.3v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3scB14LeGmueG3gr4Fd4Fbgb4C/5l8H8T/fawOfBbw2/7Jbga8GvgfY5V+G+J/rOPBVwHvzr/fXwPsAf80Lh/if6TjwW8BL82+3C7wO8Ne8YIj/mX4LeG3+/W4FHsILhvj3ey2e198Au/zbfDbwWTx/3wN8NfDXPNtrA58NvBbP38cAX83zh/j3M8/rdYDf5l/vOPB04DjP632A7+YF+2ngrXhefw28DM8f4t/PPK/XAX6bf733Br6L5/U9wHvzwh0HLvL8PQS4leeF+Pczz+t1gN/mX++7gffieT0EuJV/2U8Db8Xzeh3gt3leiH8/87xeB/ht/vV+G3gtntMl4Dgvms8GPovn9TrAb/O8EP9+5nm9DvDb/OuZ5/U7wGvzovls4LN4Xq8D/DbPC/HvZ57X6wC/zX+9zwY+i+f1EOBWnhfi3888r9cBfpv/en8FvDTP6RJwnOcP8e9nntfrAL/Nf60HA0/neX0N8NE8f4h/P/O8Xgf4bf5r/Rbw2jyvhwC38vwh/v3M83od4Lf5r/PRwFfxvL4HeG9eMMTz99XAS/GieW2e118Du7xoXod/n5cGfgs4znO6BLw0cCsvGOL5+23gtfivIf7tjgNPB47zvD4G+GpeOMTz99vAa/FfQ/zbHAd+C3hpntf3AO/Nvwzx/P028Fr81xD/eseB3wJemuf1N8BrA7v8yxDP328Dr8V/DfGv913Ae/O8LgEPBnZ50SCev68GXpoXzWvxvP4G2OVF89r863wX8N48r0vAawN/zYsO8e9nntfrAL/Nf7zvAt6b53UJeG3gr/nXQfz7mef1OsBv8x/rvYHv4vl7G+Cn+ddD/PuZ5/U6wG/zH+e9ge/i+Xsf4Lv5t0H8+5nn9TrAb/Mf472B7+L5ex/gu/m3Q/z7mef1OsBv8+/30sBvAcd5Xu8DfDf/Poh/P/O8Xgf4bf59Xhr4LeA4z+t7gPfm3w/x72ee1+sAv82/3UsDvwUc53l9D/De/MdA/PuZ5/U6wG/zb3Mc+C3gpXle3wO8N/9xEP9+5nm9DvDb/OsdB34LeGme1y7w1fzbPQP4bp4T4t/PPK/XAX6bf73XBn6L/xy/A7w2zwnx7/faPK+/Bnb513tt4Lf4z/E7wGvznBD/s7w28Fv85/gd4LV5Toj/WV4b+C3+c/wO8No8J8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hEZ1JdBroKF3QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposurePlus2;
impl IconShape for MdExposurePlus2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.05 16.29l2.86-3.07c.38-.39.72-.79 1.04-1.18.32-.39.59-.78.82-1.17.23-.39.41-.78.54-1.17.13-.39.19-.79.19-1.18 0-.53-.09-1.02-.27-1.46-.18-.44-.44-.81-.78-1.11-.34-.31-.77-.54-1.26-.71-.51-.16-1.08-.24-1.72-.24-.69 0-1.31.11-1.85.32-.54.21-1 .51-1.36.88-.37.37-.65.8-.84 1.3-.18.47-.27.97-.28 1.5h2.14c.01-.31.05-.6.13-.87.09-.29.23-.54.4-.75.18-.21.41-.37.68-.49.27-.12.6-.18.96-.18.31 0 .58.05.81.15.23.1.43.25.59.43.16.18.28.4.37.65.08.25.13.52.13.81 0 .22-.03.43-.08.65-.06.22-.15.45-.29.7-.14.25-.32.53-.56.83-.23.3-.52.65-.88 1.03l-4.17 4.55V18H22v-1.71h-5.95zM8 7H6v4H2v2h4v4h2v-4h4v-2H8V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//VeGngt4DjwYK64FdgFfgf4a/7rIP7rvBfw2cCDeeFuBT4b+B7+8yH+8x0Hfgt4af51/hp4HWCX/zyI/1zHgd8CXpp/m78GXgfY5T8H4j/XVwMfxQt2iSuO8YJ9DfDR/OdA/Od5beC3eF6XgM8GvhvY5YrjwEcDn8Xz9zrAb/MfD/Gf57uB9+J5vQ3w0zx/7w18F8/re4D35j8e4j/HceAiz+t7gPfmhftt4LV4XieAXf5jIf5zvDXwUzyvtwF+mhfurYGf4nm9D/Dd/MdC/Of4bOCzeF7iRWOe19cAH81/LMR/jt8GXovn9DvAa/Oi+WvgpXhOvwO8Nv+xEP85/gp4aZ7T7wCvzYvmt4HX4jntAif4j4X4z2Ge188Ab82L5reB1+J5if9YiP8c5nl9DvDZvGi+Gvgonpf4j4X4z2Ge1+cAn82L5rOBz+J5if9YiP8c5nl9DvDZvGg+G/gsnpf4j4X4z2Ge1+cAn82L5rOBz+J5if9YiP8c5nl9DvDZvGg+G/gsnpf4j4X4z2Ge1+cAn82L5rOBz+J5if9YiP8c5nl9DvDZvGg+G/gsnpf4j4X4z2Ge1+cAn82L5rOBz+J5if9YiP8c5nl9DvDZvGg+G/gsnpf4j4X4z2Ge188Ab82L5quBj+J5if9YiP8ctwIP4jn9DvDavGh+G3gtntPfAC/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/Ob4a+Ciel3jRXASO85w+B/hs/mMh/nO8N/BdPK/XAX6bF+6lgb/ieb0P8N38x0L853gw8HSe1/cA780L99XAR/G8HgLcyn8sxH+enwbeiuf1MsBf8/w9GPgr4DjP6WeAt+Y/HuI/z1sDP8XzuhV4G+CveU4vDXwX8NI8r/cBvpv/eIj/XL8NvBbP318DP80VLw28Nc/f7wCvzX8OxH+uBwN/DRzj3+YS8NLArfznQPzne2ngt4Fj/OtcAl4b+Gv+8yD+azwY+GzgvXjR/Azw0cCt/OdC/Nd6MPDWwFsDDwYexBXPAG4Ffhv4aeCv+a+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Atp0fEHTtN/fAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExposureZero;
impl IconShape for MdExposureZero {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.14 12.5c0 1-.1 1.85-.3 2.55-.2.7-.48 1.27-.83 1.7-.36.44-.79.75-1.3.95-.51.2-1.07.3-1.7.3-.62 0-1.18-.1-1.69-.3-.51-.2-.95-.51-1.31-.95-.36-.44-.65-1.01-.85-1.7-.2-.7-.3-1.55-.3-2.55v-2.04c0-1 .1-1.85.3-2.55.2-.7.48-1.26.84-1.69.36-.43.8-.74 1.31-.93C10.81 5.1 11.38 5 12 5c.63 0 1.19.1 1.7.29.51.19.95.5 1.31.93.36.43.64.99.84 1.69.2.7.3 1.54.3 2.55v2.04zm-2.11-2.36c0-.64-.05-1.18-.13-1.62-.09-.44-.22-.79-.4-1.06-.17-.27-.39-.46-.64-.58-.25-.13-.54-.19-.86-.19-.32 0-.61.06-.86.18s-.47.31-.64.58c-.17.27-.31.62-.4 1.06s-.13.98-.13 1.62v2.67c0 .64.05 1.18.14 1.62.09.45.23.81.4 1.09s.39.48.64.61.54.19.87.19c.33 0 .62-.06.87-.19s.46-.33.63-.61c.17-.28.3-.64.39-1.09.09-.45.13-.99.13-1.62v-2.66z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/Le/NFd/Niwbxf8vTueIhvGgQ/3UeDDwIeGngOFe8NFf8NVfsAn8N/A2wy7/OWwM/xRWvA/w2/zLEf67XBt4KeG3gpfnX+Wvgt4HvAf6af9lvAa/NFb8NvA7/MsR/vOPARwHvDTyY/xi3Ap8DfDfP32sDv8Vzeh3gt3nhEP9xjgMfBXw0cJz/OM8Avhr4bmCX5++7gffiOX0P8N68cIj/GO8FfDVwnP84fwN8NfDdvGCvDbwX8N48f98NfA/w2zx/iH+f48B3AW/Nf5zfAT4b+G1esLcCPhp4bV40vw18DvDbPCfEv91LAz8FPJj/GN8DfDZwKy/YewGfDTyYf5tbgc8GvocrEP82Lw38FnCc/zjfDXwOcCsv2HsDnw08iH+bZwCfDXw3VyD+9V4a+C3gOP85fhv4HOC3ecHeGvho4LV40fwO8NnAb/OcEP86Lw38FnCc/3x/DXw18D28YK8NvDfwXjx/3wN8N/DbPH+IF91x4K+AB/Nf61bgq4HvAXZ5/r4beC+e0/cA780Lh3jR/RTw1vz32QW+Gvge4Fae02sDv8Vzeh3gt3nhEC+a9wa+i3+77wF+G/gu/mN8N/AxwC7P9tvAa3HF7wCvzb8M8S87DjwdOM6/3UOAW4HPBj6L/xjfA7w3z/bWwE9xxesAv82/DPEv+2zgs/i3+xngrXm2jwa+iv8YDwFu5dlu5YoH86JBvHDHgacDx/m3uQS8NHArz+m9ge/i3+9zgM/m2d6bK76bFw3ihfts4LP4t/sc4LN5/l4a+G7gpfi32wUeAuzy/B0H3hr4bp4/xAv3dODB/Nv8DvDavHDHgc8GPop/u48Bvprn76OBjwIewvOHeMHeGvgp/m2eAbw0sMuL5sHAdwOvxb/ercBDeP6eDjwYeBngr3leiBfsq4GP4l/vEvDawF/zr/fSwEcDbw0c40X3PsB385zeG/gurvga4KN5XogX7K+Al+Zf5xLw2sBf8+9zHHhr4LWBtwaO8cL9NfAyPKe/Al6aK/4aeBmeF+L5ezDwdP51LgGvDfw1//EeDDwYeG2e7bWBW4FbueKrgV2ueG3gt3hOJ4BdnhPi+Xtt4Ld40f0N8NrALv8z/Bbw2jyn1wF+m+eEeP4+GvgqXjSfA3w2/7OY5/UxwFfznBDP32cDn8UL9zvAZwO/zf88twIP4jl9DvDZPCfE8/fZwGfx/P0N8NXAd/M/128Dr8Vz+hzgs3lOiOfvt4HX4jk9A3hv4Ld50bw28FZc8TPAb/Ov89rAW3HFzwC/zYvut4HX4jn9DPDWPCfE8/fbwGvxnH4HeG1eNB8NfBXP6WOAr+ZF89HAV/GcPgb4al40vw28Fs/pZ4C35jkhnr/PBj6L5/Q7wGvzL3sw8HSevxPALi/cg4Gn8/ydAHb5l/028Fo8p88BPpvnhHj+Phv4LJ7TXwMvw7/stYHf4vl7HeC3eeFeG/gtnr/XAX6bf9lfAS/Nc/oc4LN5Tojn76OBr+J5iX/ZSwN/xfP3MsBf88K9NPBXPH8vA/w1/zLzvD4G+GqeE+L5e23gt3heDwFu5V/228Br8Zx+BnhrXjS/DbwWz+lngLfmX/bSwF/xvF4H+G2eE+L5ezDwdJ7XxwBfzb/sOPDZwFtzxU8Dnw3s8qI5Dnw28NZc8dPAZwO7/Ms+GvgqntcJYJfnhHjB/hp4KZ7TzwBvzf9sPw28Fc/pb4CX5nkhXrCvBj6K53UC2OV/puPARZ7X1wAfzfNCvGCvDfwWz+tjgK/mf6bPBj6L5/U6wG/zvBAv3K3Ag3hOtwIP4X+mpwMP5jk9A3gwzx/ihfts4LN4Xp8DfDb/s3w28Fk8r88BPpvnD/HCHQduBY7xnHaBlwFu5X+GBwN/BRznOV0CHgzs8vwh/mWfDXwWz+u3gdfhf4bfAl6b5/U5wGfzgiH+ZceBW4FjPK/PAT6b/16fDXwWz+sS8GBglxcM8aJ5b+C7eP7eB/hu/nu8N/BdPH9vA/w0LxziRffTwFvx/L0P8N3813pv4Lt4/n4GeGv+ZYgX3XHgr4EH8fx9NvA5/Nf4LOCzef6eAbw0sMu/DPGv89LAbwPHeP5+G3gf4Fb+czwY+C7gtXn+LgGvDfw1LxrEv95LA78NHOP52wW+Gvgc/mN9FvDRwHGev0vAawN/zYsO8W/z0sBvA8d4wW4Fvhv4GmCXf5vjwEcB7w08mBfsEvDawF/zr4P4t3tp4KeBB/Ev+2ngt4GfAW7lhXtp4LWA1wbemn/ZM4C3Bv6afz3Ev89x4LuBt+Jf56+BXZ7TceCl+df5GeC9gV3+bRD/Md4b+GrgGP81LgHvDfw0/z6I/zjHgY8GPho4xn+OS8BXA18N7PLvh/iPdxz4aOC9gQfxH+MZwHcDXw3s8h8H8Z/rtYG3Bl4beCn+df4G+G3gp4Hf5j8H4r/OceClgZcGjnPFS3PFX3PFLvDXwF8Du/znQ/z/hvj/DfH/G+L/N8T/b/wjgJc7UAHkoTwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFaceRetouchingNatural;
impl IconShape for MdFaceRetouchingNatural {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "9",
                cy: "13",
                r: "1.25",
            }
            path {
                d: "M20.77 8.58l-.92 2.01c.09.46.15.93.15 1.41 0 4.41-3.59 8-8 8s-8-3.59-8-8c0-.05.01-.1 0-.14 2.6-.98 4.69-2.99 5.74-5.55C11.58 8.56 14.37 10 17.5 10c.45 0 .89-.04 1.33-.1l-.6-1.32-.88-1.93-1.93-.88-2.79-1.27 2.79-1.27.71-.32C14.87 2.33 13.47 2 12 2 6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10c0-1.47-.33-2.87-.9-4.13l-.33.71z",
            }
            circle {
                cx: "15",
                cy: "13",
                r: "1.25",
            }
            path {
                d: "M20.6 5.6L19.5 8l-1.1-2.4L16 4.5l2.4-1.1L19.5 1l1.1 2.4L23 4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xksDx/if6xnArTwvxL/PawPfBTyY//l+GngfYJdnQ/zbvTfwXfzvsgs8BNjlCsS/zXHg6cBx/vf5aeBtuALxb/PRwFfxv9dDgFsBxL/NdwPvxf9erwP8NoD4t/lt4LV4Tr8DvDb/85jn9TrAbwOIf5vfBl6L5/Q7wGvzP495Xq8D/DaA+Lf5beC1eE6/A7w2//OY5/U6wG8DiH+b3wZei+f0O8Br8z+PeV6vA/w2gPi3+W3gtXhOvwO8Nv/zmOf1OsBvA4hne2ngq3her8Pz+m3gtXhOvwO8Nv/zmOf1OsBvA4hne23gt3he4nn9NvBaPKffAV6b/3nM83od4LcBxLO9NvBbPC/xvH4beC2e0+8Ar83/POZ5vQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5n8e87xeB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzP495Xq8D/DaAeLbXBn6L5yWe128Dr8Vz+h3gtfmfxzyv1wF+G0A822sDv8XzEs/rt4HX4jn9DvDa/M9jntfrAL8NIJ7ttYHf4nmJ5/XbwGvxnH4HeG3+5zHP63WA3wYQz/bawG/xvMTz+m3gtXhOvwO8Nv/zmOf1OsBvA4hne23gt3he4nn9NvBaPKffAV6b/3nM83od4LcBxLO9NvBbPC/xvH4beC2e0+8Ar83/POZ5vQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5n8e87xeB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzP495Xq8D/DaAeLbXBn6L5yWe128Dr8Vz+h3gtfmfxzyv1wF+G0A822sDv8XzEs/rt4HX4jn9DvDa/Od5beC3+dczz+t1gN8GEM/22sBv8bzE8/pt4LV4Tr8DvDb/OV4b+C3gfYDv5l/HPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Nr8x3sw8FfAcWAXeB3gr3nRmef1OsBvA4hne23gt3he4nn9NvBaPKffAV6bF81xrtjlX/ZXwEvzbH8NvA6wy4vGPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Nq8aH4KOA68Di/cdwHvzfP6buB9eNGY5/U6wG8DiGd7beC3eF7ief028Fo8p98BXpt/2WcDn8UV3w28D8/fewPfxQv2PsB38y8zz+t1gN8GEM/22sBv8bzE8/pt4LV4Tr8DvDYv3GsDv8Vzeh/gu3lOLw38FnCcF2wXeB3gr3nhzPN6HeC3AcSzvTbwWzwv8bx+G3gtntPvAK/NC/Zg4K+A4zyv1wF+myuOA38FPJh/2V8DrwPs8oKZ5/U6wG8DiGd7beC3eF7ief028Fo8p98BXpsX7K+Al+b52wVeB/hr4KeAt+ZF993A+/CCmef1OsBvA4hne23gt3he4nn9NvBaPKffAV6b5++7gPfmhftr4HeAj+Jf732A7+b5M8/rdYDfBhDP9trAb/G8xPP6beC1eE6/A7w2z+u9ge/iP9cu8DrAX/O8zPN6HeC3AcSzvTbwWzwv8bx+G3gtntPvAK/Nc3pp4K/4r/HXwOsAuzwn87xeB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzbMeBvwIezH+d7wbeh+dkntfrAL8NIJ7ttYHf4nmJ5/XbwGvxnH4HeG3+5zHP63WA3wYQz/bawG/xvMTz+m3gtXhOvwO8Nv/zmOf1OsBvA4hne23gt3he4nn9NvBaPKffAV6b/3nM83od4LcBxLO9NvBbPC/xvH4beC2e0+8Ar83/POZ5vQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5n8e87xeB/htAPFsrw38Fs9LPK+fBt6K57QL/DX/87w2z+t1gN8GEM/22sBv8bzE8/ps4LP43+tlgL8GEM/22sBv8bzE83pp4K/43+kZwIO5AvFsrw38Fs9LPH9fDXwU//u8DvDbXIF4ttcGfovnJV6wrwY+iv8dLgHvDfw0z4Z4ttcGfovnJV64lwbeGnhp4Dj/89wK/DXw3cAuzwnxbK8N/BbPS/zfhXi21wZ+i+cl/u9CPNtrA7/F8xL/dyGe7bWB3+J5if+7EM/22sBv8bzE/12IZ3tt4Lf43+t3gNfmXwfxbA8Gns7/Xr8DvDb/Oojn9NPAW/G/0+8Ar82/DuI5HQduBY7xv8/vAK/Nvw7ieR0Hvht4K/53+R3gtfnXQbxgDwYezP8eu8Bf86+D+P8N8f8b/wiF5BZQSwMpzwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter;
impl IconShape for MdFilter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.96 10.29l-2.75 3.54-1.96-2.36L8.5 15h11l-3.54-4.71zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter1;
impl IconShape for MdFilter1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm11 10h2V5h-4v2h2v8zm7-14H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter2;
impl IconShape for MdFilter2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-4-4h-4v-2h2c1.1 0 2-.89 2-2V7c0-1.11-.9-2-2-2h-4v2h4v2h-2c-1.1 0-2 .89-2 2v4h6v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter3;
impl IconShape for MdFilter3 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm14 8v-1.5c0-.83-.67-1.5-1.5-1.5.83 0 1.5-.67 1.5-1.5V7c0-1.11-.9-2-2-2h-4v2h4v2h-2v2h2v2h-4v2h4c1.1 0 2-.89 2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter4;
impl IconShape for MdFilter4 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm12 10h2V5h-2v4h-2V5h-2v6h4v4zm6-14H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter5;
impl IconShape for MdFilter5 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm14 8v-2c0-1.11-.9-2-2-2h-2V7h4V5h-6v6h4v2h-4v2h4c1.1 0 2-.89 2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter6;
impl IconShape for MdFilter6 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2h2c1.1 0 2-.89 2-2v-2c0-1.11-.9-2-2-2h-2V7h4V5h-4c-1.1 0-2 .89-2 2v6c0 1.11.9 2 2 2zm0-4h2v2h-2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter7;
impl IconShape for MdFilter7 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2l4-8V5h-6v2h4l-4 8h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xksDx/if6xnArTwvxL/PawPfBTyY//l+GngfYJdnQ/zbvTfwXfzvsgs8BNjlCsS/zXHg6cBx/vf5aeBtuALxb/PRwFfxv9dDgFsBxL/NdwPvxf9erwP8NoD4t/lt4LV4Tr8DvDb/85jn9TrAbwOIf5vfBl6L5/Q7wGvzP495Xq8D/DaA+Lf5beC1eE6/A7w2//OY5/U6wG8DiH+b3wZei+f0O8Br8z+PeV6vA/w2gPi3+W3gtXhOvwO8Nv/zmOf1OsBvA4hne2ngq3her8Pz+m3gtXhOvwO8Nv96Lw28FfDSwHGe163AXwPfA+zyr2ee1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bf51vgr4aF40u8DHAN/Nv455Xq8D/DaAeLbXBn6L5yWe128Dr8Vz+h3gtXnRfTXwUfzrvQ7w27zozPN6HeC3AcSzvTbwWzwv8bx+G3gtntPvAK/Ni+algb/i3+ZW4CG86Mzzeh3gtwHEs7028Fs8L/G8fht4LZ7T7wCvzYvms4HP4t/uZYC/5kVjntfrAL8NIJ7ttYHf4nmJ5/XbwGvxnH4HeG1eND8NvBXP6RLw1zyv1+J5vQ7w27xozPN6HeC3AcSzvTbwWzwv8bx+G3gtntPvAK/Ni+a3gdfiOf0O8No8L/O8Xgf4bV405nm9DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmRfPbwGvxnH4HeG2el3lerwP8Ni8a87xeB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzovlt4LV4Tr8DvDbPyzyv1wF+mxeNeV6vA/w2gHi21wZ+i+clntdvA6/Fc/od4LV50fw28Fo8p98BXpvnZZ7X6wC/zYvGPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Nq8aD4b+Cye0+8Ar83zMs/rZYC/5kVjntfrAL8NIJ7ttYHf4nmJ5/XbwGvxnH4HeG1eNC8N/BX/Nn8DvDQvOvO8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Br86J7b+C7+Nd5BvDWwF/zojPP63WA3wYQz/bawG/xvMTz+m3gtXhOvwO8Nv86Dwa+Gngt4Dgv2K3AzwCfDezyr2Oe1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bV50Lw18F/DSvOi+GvgcYJcXnXlerwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1edEcB/4KeDD/el8DfDQvOvO8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Br86L5aOCreE5/A3w0z+u3eF4ngF1eNOZ5vQ7w2wDi2V4b+C2el3hevw28Fs/pd4DX5kXz3cB78Zx+B3htnpd5Xq8D/DYvGvO8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Br86L5beC1eE6/A7w2z8s8r9cBfpsXjXlerwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1edH8NvBaPKffAV6b52We1+sAv82Lxjyv1wF+G0A822sDv8XzEs/rt4HX4jn9DvDavGh+G3gtntPvAK/N8zLP63WA3+ZFY57X6wC/DSCe7bWB3+J5ief128Br8Zx+B3htXjS/DbwWz+mvgY/mef02z+t1gN/mRWOe1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bV40nw18Fv92rwP8Ni8a87xeB/htAPFsrw38Fs9LPK/fBl6L5/Q7wGvzonlr4Kf4t7kEHOdFZ57X6wC/DSCe7bWB3+J5ief128Br8Zx+B3htXnR/DbwU/3qfA3w2LzrzvF4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/OiezDw28CDeNF9D/De/OuY5/U6wG8DiGd7beC3eF7ief028Fo8p98BXpt/nePARwPvDTyIF+x3gO8Gvpt/PfO8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Br8z+PeV6vA/w2gHi21wZ+i+clntdvA6/Fc/od4LX5n8c8r9cBfhtAPNtrA7/F8xLP67eB1+I5/Q7w2vzPY57X6wC/DSCe7bWB3+J5ief128Br8Zx+B3ht/ucxz+t1gN8GEM/22sBv8bzE8/pt4LV4Tr8DvDb/85jn9TrAbwOIZ3tt4Ld4XuJ5/TTwVjynXeCv+Z/ntXlerwP8NoB4ttcGfovnJZ7XZwOfxf9eLwP8NYB4ttcGfovnJZ7XSwN/xf9OzwAezBWIZ3tt4Ld4XuL5+2rgo/jf53WA3+YKxLO9NvBbPC/xgn018FH873AJeG/gp3k2xLO9NvBbPC/xwr008NbASwPH+Z/nVuCvge8GdnlOiGd7beC3eF7i/y7Es7028Fs8L/F/F+LZXhv4LZ6X+L8L8WyvDfwWz0v834V4ttcGfovnJf7vQjzbawO/xf9evwO8Nv86iGd7MPB0/vf6HeC1+ddBPKefBt6K/51+B3ht/nUQz+k4cCtwjP99fgd4bf51EM/rOPDdwFvxv8vvAK/Nvw7iBXsw8GD+99gF/pp/HcT/b4j/3/hH54hAUGlqSeAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter8;
impl IconShape for MdFilter8 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2h2c1.1 0 2-.89 2-2v-1.5c0-.83-.67-1.5-1.5-1.5.83 0 1.5-.67 1.5-1.5V7c0-1.11-.9-2-2-2h-2c-1.1 0-2 .89-2 2v1.5c0 .83.67 1.5 1.5 1.5-.83 0-1.5.67-1.5 1.5V13c0 1.11.9 2 2 2zm0-8h2v2h-2V7zm0 4h2v2h-2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter9;
impl IconShape for MdFilter9 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM15 5h-2c-1.1 0-2 .89-2 2v2c0 1.11.9 2 2 2h2v2h-4v2h4c1.1 0 2-.89 2-2V7c0-1.11-.9-2-2-2zm0 4h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilter9Plus;
impl IconShape for MdFilter9Plus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm11 7V8c0-1.11-.9-2-2-2h-1c-1.1 0-2 .89-2 2v1c0 1.11.9 2 2 2h1v1H9v2h3c1.1 0 2-.89 2-2zm-3-3V8h1v1h-1zm10-8H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 8h-2V7h-2v2h-2v2h2v2h2v-2h2v6H7V3h14v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/Od6Hf5jvDTwVTwv8YIh/mXmP5f4j/HawG/xvMQLhviXmf9c4j/GawO/xfMSLxjiX2b+c4n/GK8N/BbPS7xgiH+Z+c8l/mO8NvBbPC/xgiH+ZeY/l/iP8drAb/G8xAuG+JeZ/1ziP8ZrA7/F8xIvGOJfZv5zif8Yrw38Fs9LvGCIf5n5zyX+Y7w28Fs8L/GCIf5l5j+X+I/x2sBv8bzEC4b4l5n/XOI/xmsDv8XzEi8Y4l9m/nOJ/xivDfwWz0u8YIh/mfnPJf5jvDbwWzwv8YIh/mXmP5f4j/HawG/xvMQLhviXmf9c4j/GawO/xfMSLxjiX2b+c4n/GK8N/BbPS7xgiH+Z+c/zN8B7A3/Nv99rA7/F8xIvGOJfZv5zPAN4aeDpwMsAt/Lv89rAb/G8xAuG+JeZ/3iXgNcG/how8NfA6wC7/Nu9NvBbPC/xgiH+ZeY/3ssAf80V5oq/Bl6Gf7vXBn6L5yVeMMS/zPzHeh/gu3k282zfDbwP/zavDfwWz0u8YIh/mfmP8zHAV/OczHP6GuCj+dd7beC3eF7iBUP8y8x/jO8B3pvnZZ7X+wDfzb/OawO/xfMSLxjiX2b+/X4GeGueP/P8vQ3w07zoXhv4LZ6XeMEQ/zLz7/M3wGsDuzx/5vnbBV4H+GteNK8N/BbPS7xgiH+Z+bd7BvDSwC4vmHnBdoGXAW7lX/bawG/xvMQLhviXmX+bS8BrA3/NC2deuL8GXgfY5YV7beC3eF7iBUP8y8y/zcsAf82/zPzL/hp4GV641wZ+i+clXjDEv8z8670P8N28aMyL5ruB9+EFe23gt3he4gVD/MvMv87HAF/Ni8686L4G+Giev9cGfovnJV4wxL/MvOi+B3hv/nXMv877AN/N83pt4Ld4XuIFQ/zLzIvmZ4C35l/P/Ou9DfDTPKfXBn6L5yVeMMS/zPzL/gZ4bWCXfz3zr7cLvA7w1zzbawO/xfMSLxjiX2ZeuGcALw3s8m9j/m12gZcBbuWK1wZ+i+clXjDEv8y8YJeA1wb+mn8782/318DrALvAawO/xfMSLxjiX2ZesJcB/pp/H/Pv89fAywCvDfwWz0u8YIh/mXn+3gf4bv79zL/fdwPfA/wWz0u8YIh/mXle3w18N/8xfpv/GH8NvDTPS7xgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIZwLJBQZLkwQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterBAndW;
impl IconShape for MdFilterBAndW {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16l-7-8v8H5l7-8V5h7v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAES0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHjvOjEv4950e0CPw18DnAr/zLEi+a9ge/i30b8+5h/m/cBvpsXDvEvezDwdP7txL+P+bd7CHArLxjiX/bdwHvxbyf+fcy/3fcA780LhviXXQSO828n/n3Mv90ucIIXDPEvM8/rY4C/5kXz2/z7vDYvmpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zXezDwUcBrAy/NFX8N/DbwNcCt/NdB/Nf6KuCjeeG+GvgY/msg/uv8FfDSvGj+GngZ/vMh/mt8NfBR/Ot8DfDR/OdC/Od7MPB0/m0eAtzKfx7Ef76vBj6Kf5uvAT6a/zyI/3x/Bbw0/zZ/DbwM/3kQ//nMv4/4z4P4z2f+fcR/HsR/vr8GXop/m78BXpr/PIj/fF8NfBT/Nl8DfDT/eRD/+R4MPJ1/m4cAt/KfB/Ff46uBj+Jf52uAj+Y/F+K/zl8DL8WL5m+Al+Y/H+K/1lcDH8UL9zXAR/NfA/Ff78HARwOvDbwUV/wN8NvAVwO38l8H8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87w+GvhrXjS/w7/Pa/GieWngq3le4gVD/Mt2gWP824l/H/Nvdwk4zguG+Jd9N/Be/NuJfx/zb/c9wHvzgiH+ZQ8Gns6/nfj3Mf92DwFu5QVDvGjeG/gu/m3Ev4/5t3kf4Lt54RAvugcDnw28NXCMF5349zEvukvATwOfDdzKvwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSb6ohB6TRubAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterCenterFocus;
impl IconShape for MdFilterCenterFocus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 15H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zM12 9c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFhklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uM8GPgp4KX5z/XXwOsAu/z7If5jvDTwW8BxXrifAX4b+Gvgt3m2lwaOAy8NvDfwUrxwu8DrAH/Nvw/i3+848HTgOC/Y1wBfDdzKi+bBwGcD78ULtgu8DHAr/3aIf7+/Al6a5+93gPcGbuXf5sHAdwOvxfP328Dr8G+H+Pf5bOCzeP6+B3hv/mN8N/BePH+fA3w2/zaIf7sHA0/n+fsc4LP5j/XZwGfx/L0O8Nv86yH+9V4LeGngvYGX5nl9D/De/Of4buC9eF7fA7w3/3qIF81LAx8FvDf/sl3gp4HvAX6b/1jHgb8GHsTzeghwK/86iBfupYGvAl6bf5vfBj4G+Gv+47w28Fs8r+8B3pt/HcQL9lnAZ/Mf47OBz+E/zm8Dr8Vz2gVO8K+DeP6+C3hv/mP9NPA+wC7/fm8N/BTP62WAv+ZFh3hePwW8Nf+y3+E5vRb/su8G3od/v+PARZ7XdwMfA+zyokE8p68GPooX7GeA7wZ+mufvrYH3Bt6KF+xrgI/m3++ngbfi+dsFfhv4buBneMEQz/bWwE/x/P0N8NHAb/OieW3gq4GX4vl7HeC3+ff5bOCz+JfdCnwM8NM8L8SzPR14MM/rb4DXBnb51zkO/DbwUjyvW4GXAXb5t/ts4LN40f028DbALs+GuOK9ge/ief0N8NrALv82x4HfBl6K5/U+wHfzb/fWwE/xr/PXwOsAu1yBuOLpwIN5TpeAlwZu5d/nwcDTeV63Ag/h3+61gd/iX++vgdcBdgEEvDTwVzyvzwE+m/8Ynw18Fs/rdYDf5j/WawNvDbw3cIzn77uB9wEQ8NnAZ/G8TgC7/Mc4DlzkeX0O8Nn85zgOfDXwXjx/rwP8toDfBl6L5/Q7wGvzH+uvgZfiOf0O8Nr85/pu4L14Xr8NvI6A3wZei+f0OcBn8x/rs4HP4jn9NfAy/Of7beC1eF4vI8A8r88BPpv/WJ8NfBbPS/zne2ngr3heHyPAPK+3AX6a/1jHgZfmef02/zX+GngpntPvCLgVeBDP6XOAz+b/ls8GPovn9NcCfht4LZ7T1wAfzf8t7w18F88JAT8NvBXP6VbgIfzf8tHAV/GcEPDewHfxvF4H+G3+7/hs4LN4Tn8j4Dhwkef128Dr8H/HXwEvzXP6HXHFbwOvxfN6H+C7+d/vpYG/4nl9jLjitYHf4nntAq8D/DX/u/0W8No8r4eIZ/tp4K14XrvA2wC/zf9O3wW8N8/rd4DXFs/2YOCvgWM8f18NfA6wy/8Ox4GvAt6b5+91gN8Wz+m1gd/iBdsFfhr4aeAZwF/zP89rAW8NvDdwnOfve4D3BhDP672B7+L/rr8BXhvYBRDP31sD3w0c4/+WvwHeGriVKxAv2EsDPw08iP8bfgd4a2CXZ0P8y94b+GzgQfzv9Azgs4Hv5nkhXnRvDbw18NbAMf7n+xngp4GfBnZ5/hD/dq/N/0y7wF/zokH8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwT9rNCOjB1ClwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterDrama;
impl IconShape for MdFilterDrama {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.61 5.64 5.36 8.04 2.35 8.36 0 10.9 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4h2c0-2.76-1.86-5.08-4.4-5.78C8.61 6.88 10.2 6 12 6c3.03 0 5.5 2.47 5.5 5.5v.5H19c1.65 0 3 1.35 3 3s-1.35 3-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterFrames;
impl IconShape for MdFilterFrames {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4h-4l-4-4-4 4H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H4V6h4.52l3.52-3.5L15.52 6H20v14zM18 8H6v10h12",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/nwcBnAe/Dfz7E/zx/Bbw08DHAV/OfC/E/y1cDH8WzvQzw1/znQfzP8dbAT/GcbgVeBtjlPwfif4YHA38FHOd5/TTwNvznQPzP8FfAS/OCfQzw1fzHQ/z3+2rgo/iXvQzw1/zHQvz3emvgp3jR3Aq8DLDLfxzEf58HA38FHOdF99PA2/AfB/Hf56+Al+Zf72OAr+Y/BuI/znFglxfNVwMfxb/dywB/zb8f4j/GceCvgI8BfpoX7q2Bn+Lf51bgZYBd/n0Q/zF+CnhrYBd4GeBWnr8HA38FHOff76eBt+HfB/Hv99HAV/Fsfw28DM/fZwOfxX+cjwG+mn87xL/PSwN/xfP6GuCjef7+Gngp/uO8DPDX/Nsg/u2OA38FPJjn722An+Z5PRj4a+AY/zFuBV4G2OVfD/Fv91PAW/OC7QIvA9zK83pr4Kf4j/PTwNvwr4f4t/lo4Kv4l/018DI8f18NfBT/cT4G+Gr+dRD/ei8N/BUvuq8BPprn76+Bl+I/zssAf82LDvGvcxz4K+DB/Ou8DfDTPK8HA38NHOM/xq3AywC7vGgQ/zo/Bbw1/3q7wMsAt/K83hr4Kf7j/DTwNrxoEC+6jwa+in+7vwZehufvq4GP4j/OxwBfzb8M8aJ5aeCv+Pf7GuCjef7+Gngp/uO8DPDXvHCIf9lx4K+AB/Mf422An+Z5vTXwU/zHuRV4GWCXFwzxL/sp4K35j7MLvAxwK8/2UcBX8x/vp4G34QVDvHAfDXwV//H+GngZ4DjwXcBb85/nY4Cv5vlDvGAvDfwV/3l+Gnhp4MH853sZ4K95Xojn7zjwV8CD+b/hVuBlgF2eE+L5+yngrfm/5aeBt+E5IZ7XRwNfxf9NHwN8Nc+GeE4vDfwV/7e9DPDXXIF4tuPAXwEP5v+2W4GXAXYBxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EaTza0HigazgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterHdr;
impl IconShape for MdFilterHdr {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterNone;
impl IconShape for MdFilterNone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterTiltShift;
impl IconShape for MdFilterTiltShift {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 4.07V2.05c-2.01.2-3.84 1-5.32 2.21L7.1 5.69c1.11-.86 2.44-1.44 3.9-1.62zm7.32.19C16.84 3.05 15.01 2.25 13 2.05v2.02c1.46.18 2.79.76 3.9 1.62l1.42-1.43zM19.93 11h2.02c-.2-2.01-1-3.84-2.21-5.32L18.31 7.1c.86 1.11 1.44 2.44 1.62 3.9zM5.69 7.1L4.26 5.68C3.05 7.16 2.25 8.99 2.05 11h2.02c.18-1.46.76-2.79 1.62-3.9zM4.07 13H2.05c.2 2.01 1 3.84 2.21 5.32l1.43-1.43c-.86-1.1-1.44-2.43-1.62-3.89zM15 12c0-1.66-1.34-3-3-3s-3 1.34-3 3 1.34 3 3 3 3-1.34 3-3zm3.31 4.9l1.43 1.43c1.21-1.48 2.01-3.32 2.21-5.32h-2.02c-.18 1.45-.76 2.78-1.62 3.89zM13 19.93v2.02c2.01-.2 3.84-1 5.32-2.21l-1.43-1.43c-1.1.86-2.43 1.44-3.89 1.62zm-7.32-.19C7.16 20.95 9 21.75 11 21.95v-2.02c-1.46-.18-2.79-.76-3.9-1.62l-1.42 1.43z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterVintage;
impl IconShape for MdFilterVintage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.7 12.4c-.28-.16-.57-.29-.86-.4.29-.11.58-.24.86-.4 1.92-1.11 2.99-3.12 3-5.19-1.79-1.03-4.07-1.11-6 0-.28.16-.54.35-.78.54.05-.31.08-.63.08-.95 0-2.22-1.21-4.15-3-5.19C10.21 1.85 9 3.78 9 6c0 .32.03.64.08.95-.24-.2-.5-.39-.78-.55-1.92-1.11-4.2-1.03-6 0 0 2.07 1.07 4.08 3 5.19.28.16.57.29.86.4-.29.11-.58.24-.86.4-1.92 1.11-2.99 3.12-3 5.19 1.79 1.03 4.07 1.11 6 0 .28-.16.54-.35.78-.54-.05.32-.08.64-.08.96 0 2.22 1.21 4.15 3 5.19 1.79-1.04 3-2.97 3-5.19 0-.32-.03-.64-.08-.95.24.2.5.38.78.54 1.92 1.11 4.2 1.03 6 0-.01-2.07-1.08-4.08-3-5.19zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlare;
impl IconShape for MdFlare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 11H1v2h6v-2zm2.17-3.24L7.05 5.64 5.64 7.05l2.12 2.12 1.41-1.41zM13 1h-2v6h2V1zm5.36 6.05l-1.41-1.41-2.12 2.12 1.41 1.41 2.12-2.12zM17 11v2h6v-2h-6zm-5-2c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3zm2.83 7.24l2.12 2.12 1.41-1.41-2.12-2.12-1.41 1.41zm-9.19.71l1.41 1.41 2.12-2.12-1.41-1.41-2.12 2.12zM11 23h2v-6h-2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE80lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/st/iv8bPAF/Ni+argZfiOf0N8NH86yD+Zea/xkOAW/mXPRh4Os/fCWCXFx3iX2b+8/0M8Na8aD4a+Cqev48BvpoXHeJfZv7zvQ7w27xo/gp4aZ6/vwZehhcd4l9m/nP9DfDSvGgeDDydF+4hwK28aBD/MvOf632A7+ZF89nAZ/HCfQ7w2bxoEP8y85/nGcCDedE9HXgwz/YzXPFWPNutwEN40SD+ZeY/z+cAn82L5qWBv+I5fQxXfBXP6WWAv+ZfhviXmf8cl4AHA7u8aL4a+Cie00O44uk8p68BPpp/GeJfZv5zfA/w3rzoLgLHeba/AV6aK/4aeCmebRc4wb8M8S8z/zkeAtzKi+atgZ/iOX0M8NVc8dHAV/Gc3gb4aV44xL/M/Mf7GeCtedF9N/BePKeHALdyxYOBp/Ocvgd4b144xL/M/Md7HeC3edEcB54OHOfZngE8mOd0K/Agnm0XOMELh/iXmf9YfwO8NC+69wa+i+f0NcBH85y+GvgontP7AN/NC4b4l5n/WO8DfDcvup8G3orn9DLAX/OcXhr4K57TzwBvzQuG+JeZ/zjPAB7Mi+44cJHn9dk8f5/N8zoB7PL8If5l5j/O5wCfzYvuvYHv4t/nfYDv5vlD/MvMf4xLwIOBXV50fwW8NP8+fw28DM8f4l9m/mN8D/DevOgeDDyd/xgPAW7leSH+ZeY/xkOAW3nRfTTwVTynrwE+mhfuq4GP4jl9DPDVPC/Ev8z8+/0M8Nb86zwdeDDP6WWAv+aFe2ngr3hOtwIP4Xkh/mXm3+91gN/mRffSwF/xnJ4BPJgXza3Ag3hOLwP8Nc8J8S8z/z5/A7w0/zpfDXwUz+lzgM/mRfPVwEfxnL4G+GieE+JfZv593gf4bv51ng48mOf0EOBWXjQPBp7Oc7oVeAjPCfEvM/8+v82L5nuA7waOAy/N8/pt/nVem+f12zwnxL/M/Oe7BDwY2OW/FuJfZv7zvQ/w3fzXQ/zLzH+uvwFemv8eiH+Z+c/1OsBv898D8S8z/3m+B3hv/vsg/mXmP8cl4KWBW/nvg/iXmf8cnwN8Nv+9EP+y1+aF+y3+9Z4BPJj/foh/P/Ov9zbAT/PfD/HvZ/51fgd4bf5nQPz7mX+dhwC38j8D4t/PvOi+Bvho/udA/PuZF80l4MHALv9zIP79zIvmfYDv5n8WxL+f+Zf9DfDS/M+D+Pcz/7LXAX6b/3kQ/37mhfse4L35nwnx72desEvASwO38j8T4t/PvGCfA3w2/3Mh/v3M8/cM4MH8z4b49zPP39sAP83/bIh/P/O8fgd4bf7nQ/z7mef1EOBW/udD/PuZ5/Q1wEfzvwPi38882yXgwcAu/zsg/v3Ms70P8N3874H49zNX/A3w0vzvgvj3M1e8DvDb/O+C+Pcz8D3Ae/O/D+Lfbxd4aeBW/vdB/Pt9NvDZ/O+E+P8N8f8b4v83/hEx/8NBf4Ko1AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlashAuto;
impl IconShape for MdFlashAuto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 2v12h3v9l7-12H9l4-9H3zm16 0h-2l-3.2 9h1.9l.7-2h3.2l.7 2h1.9L19 2zm-2.15 5.65L18 4l1.15 3.65h-2.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlashOff;
impl IconShape for MdFlashOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.27 3L2 4.27l5 5V13h3v9l3.58-6.14L17.73 20 19 18.73 3.27 3zM17 10h-4l4-8H7v2.18l8.46 8.46L17 10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/cV4a+Cr+a7wPcCv/foj/OK8N/Bb/+X4HeG3+YyD+47w28Fv853sd4Lf5j4H4j/PawG/xn+t3gNfmPw7iP85rA7/Ff67XAX6b/ziI/zivDfwW/3l+B3ht/mMh/uO8NvBb/Od5HeC3+Y+F+I/z2sBv8Z/jd4DX5j8e4j/OawO/xX+O1wF+m/94iP84rw38Fv/xfgd4bf5zIP7jvDbwW/zHex3gt/nPgfiP89rAb/Ef63eA1+Y/D+I/zmsDv8V/rNcBfpv/PIj/OK8N/Bb/cX4HeG3+cyH+47w28Fv8x3kd4Lf5z4X4j/PawG/xH+N3gNfmPx/iP85rA7/Ff4zXAX6b/3yI/zivDfwW/36/A7w2/zUQ/3FeG/gt/v1eB/ht/msg/uO8NvBb/Pv8DvDa/NdB/Md5beC3+Pd5HeC3+a+D+I/z2sBv8W93K/DevGj+Btjl3w/xH+e1gd/iP9/fAC/NfwzEf5zXBn6L/3yvA/w2/zEQ/3FeG/gt/nP9DPDW/MdB/Md5beC3+M9zCXhp4Fb+4yD+47w28Fv85/kc4LP5j4X4j/PawG/xn+MZwEsDu/zHQvzHeW3gt/jP8TbAT/MfD/Ef56WBr+YFOw68FP96vwO8Nv85EP91Xhv4Lf71Xgb4a/5zIP7rvDbwW/zrfA3w0fznQfzXeW3gt3jRXQIeDOzynwfxX+e1gd/iRfcxwFfznwvxX+e1gd/iRfM3wEvznw/xX+e1gd/iRfM6wG/znw/xX+e1gd/iX/YzwFvzXwPxX+e1gd/ihbsEvDRwK/81EP91Xhv4LV64zwE+m/86iP86rw38Fi/YM4CXBnb5r4P4r/PawG/xgr0N8NP810L813lt4Ld4/n4HeG3+6yH+67w28Fs8fy8D/DX/9RD/dV4b+C2e19cAH81/D8R/ndcGfovndAl4MLDLfw/Ef53XBn6L5/QxwFfz3wfxX+e1gd/i2f4GeGn+eyH+67w28Fs82+sAv81/L8R/ndcGfosrfgZ4a/77If7rvDbwW8Al4KWBW/nvh/iv89rAbwGfA3w2/zMg/uu8NvDdwEsDu/zPgPiv89rAceCn+Z8D8V/nOLDL/yyI/98Q/78h/n9D/P+G+P+NfwTVbXNBw0vhnQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlashOn;
impl IconShape for MdFlashOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 2v11h3v9l7-12h-4l4-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlip;
impl IconShape for MdFlip {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 21h2v-2h-2v2zm4-12h2V7h-2v2zM3 5v14c0 1.1.9 2 2 2h4v-2H5V5h4V3H5c-1.1 0-2 .9-2 2zm16-2v2h2c0-1.1-.9-2-2-2zm-8 20h2V1h-2v22zm8-6h2v-2h-2v2zM15 5h2V3h-2v2zm4 8h2v-2h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlipCameraAndroid;
impl IconShape for MdFlipCameraAndroid {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9,12c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S9,10.34,9,12z",
            }
            path {
                d: "M8,10V8H5.09C6.47,5.61,9.05,4,12,4c3.72,0,6.85,2.56,7.74,6h2.06c-0.93-4.56-4.96-8-9.8-8C8.73,2,5.82,3.58,4,6.01V4H2v6 H8z",
            }
            path {
                d: "M16,14v2h2.91c-1.38,2.39-3.96,4-6.91,4c-3.72,0-6.85-2.56-7.74-6H2.2c0.93,4.56,4.96,8,9.8,8c3.27,0,6.18-1.58,8-4.01V20 h2v-6H16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlipCameraIos;
impl IconShape for MdFlipCameraIos {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,5h-3.17L15,3H9L7.17,5H4C2.9,5,2,5.9,2,7v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M12,18 c-2.76,0-5-2.24-5-5H5l2.5-2.5L10,13H8c0,2.21,1.79,4,4,4c0.58,0,1.13-0.13,1.62-0.35l0.74,0.74C13.65,17.76,12.86,18,12,18z M16.5,15.5L14,13h2c0-2.21-1.79-4-4-4c-0.58,0-1.13,0.13-1.62,0.35L9.64,8.62C10.35,8.24,11.14,8,12,8c2.76,0,5,2.24,5,5h2 L16.5,15.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGradient;
impl IconShape for MdGradient {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 9h2v2h-2zm-2 2h2v2H9zm4 0h2v2h-2zm2-2h2v2h-2zM7 9h2v2H7zm12-6H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 18H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2zm2-7h-2v2h2v2h-2v-2h-2v2h-2v-2h-2v2H9v-2H7v2H5v-2h2v-2H5V5h14v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8ZLA8e44nf4j/Fg4EFc8QzgVv71EP+5Xhv4LuDBPNsu8NXA5/Bvcxz4LuCteU4/DbwPsMuLDvGf572B7+IF+27gffjXOQ48HTjO87cLPATY5UWD+M9xHHg6cJwX7n2A7+ZF91PAW/PC/TTwNrxoEP85Phr4Kv5lvwO8Ni+aBwNP50XzEOBW/mWI/xzfDbwXLxrxonlt4Ld40bwO8Nv8yxD/Ob4beC9eNOJF89rAb/GieR3gt/mXIf5zfDTwVfzLfgd4bV40DwaezovmIcCt/MsQ/zmOA7cCx3jh3gf4bl50Pw28FS/czwBvzYsG8Z/nvYHv4gX7HuC9+dc5DtwKHOP5+xvgtYFdXjSI/1yvDXw38CCe7RLw1cBn829zHPhu4K14Tj8DvDewy4sO8W9zHHgv4KW54q+B7wF2ef5eGjjOFb/NC/ZWwEsDLw38NfAzwF/z/D0YeDBX3Arcyr8e4l/vvYGvAo7znHaBjwG+m3+9BwM/Bbw0z+urgY/hPwfiX+e1gd/ihXsd4Ld50R0H/gp4MC/Y1wAfzX88xL/O04EH88LdCjyEF91nA5/Fv+whwK38x0K86F4a+CteNC8D/DUvmqcDD+Zf9jnAZ/MfC/Gie23gt3jRvA7w27xozIvmZ4C35j8W4kX32sBv8aJ5HeC3edHsAsf4l30O8Nn8x0L86+wCx3jhLgHHedF9N/Be/MveBvhp/mMh/nU+G/gsXrjPAT6bF92Dgafzwv0N8NL8x0P863038F48f98DvDf/eu8NfBfP398Abw3cyn88xL/NewMfDbwUV/wO8N3Ad/Nv92Dgs4G3Bo4BzwC+G/hqYJf/HIj/XMeBjwLeG3gwsAv8NPA5wK38270X8NHAS3PFXwNfDXwP/zqI/zwPBn4LeDDP3/sA382/3ncB783z993A+/CiQ/zn+SvgpXnhHgLcyovus4HP4oX7HOCzedEg/nO8NfBT/Mu+B3hvXnQXgeO8cLvACV40iP8cnw18Fv+yXeAEL5rXBn6LF83rAL/Nvwzxn+OngbfiRSNeNK8N/BYvmtcBfpt/GeI/x2cDn8W/7BnAg3nRvDTwV7xoXgb4a/5liP8cLw38Ff+yzwE+mxfdrcCDeOGeATyYFw3iP89XAx/FC/Y3wGsDu7zoXhv4LV641wF+mxcN4j/XVwMfxfP6G+CtgVv513tv4KuBYzynS8B7Az/Niw7xb/dg4EFc8QzgVp6/lwbeGnhp4K+BvwZ+mhfstbjiEvDXPH/HgfcGXpor/hr4bmCXfx3Ev95x4LuAt+Y5/TTwPsAu/zafBXw0cJxnuxV4H+C3+c+B+Nc5DjwdOM7ztws8BNjlX+e7gPfmBXsf4Lv5j4f41/kp4K154X4aeBtedO8NfBcv3C7wEGCX/1iIF92DgafzonkIcCsvmr8CXpp/2ccAX81/LMSL7rWB3+JF8zrAb/OiMS+a7wHem/9YiBfdawO/xYvmdYDf5kVjXjRfA3w0/7EQL7oHA0/nRfMQ4FZeNH8NvBT/so8Bvpr/WIh/nZ8G3ooX7meAt+ZF997Ad/HCXQIeDOzyHwvxr3McuBU4xvP3N8BrA7v863w38F68YG8D/DT/8RD/eseB7wbeiuf0M8B7A7v823w28NHAMZ7tGcB7A7/Nfw7Ev92DgQdzxa3ArfzHeG2u2AX+mv9ciP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPwV3QQYCkgggAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGrain;
impl IconShape for MdGrain {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 12c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12-8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-4 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGridOff;
impl IconShape for MdGridOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 4v1.45l2 2V4h4v4h-3.45l2 2H14v1.45l2 2V10h4v4h-3.45l2 2H20v1.45l2 2V4c0-1.1-.9-2-2-2H4.55l2 2H8zm8 0h4v4h-4V4zM1.27 1.27L0 2.55l2 2V20c0 1.1.9 2 2 2h15.46l2 2 1.27-1.27L1.27 1.27zM10 12.55L11.45 14H10v-1.45zm-6-6L5.45 8H4V6.55zM8 20H4v-4h4v4zm0-6H4v-4h3.45l.55.55V14zm6 6h-4v-4h3.45l.55.54V20zm2 0v-1.46L17.46 20H16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGridOn;
impl IconShape for MdGridOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM8 20H4v-4h4v4zm0-6H4v-4h4v4zm0-6H4V4h4v4zm6 12h-4v-4h4v4zm0-6h-4v-4h4v4zm0-6h-4V4h4v4zm6 12h-4v-4h4v4zm0-6h-4v-4h4v4zm0-6h-4V4h4v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHdrEnhancedSelect;
impl IconShape for MdHdrEnhancedSelect {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6 2.69-6 6-6zm0 2C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm1 7h-2V9H9V7h2V5h2v2h2v2h-2v2zm11 9h-2v2h-1.5v-2h-2v-1.5h2v-2H22v2h2V20zm-6-1.5c0 .6-.4 1.1-.9 1.4L18 22h-1.5l-.9-2h-1.1v2H13v-6h3.5c.8 0 1.5.7 1.5 1.5v1zm-1.5 0v-1h-2v1h2zm-13-.5v-2H5v6H3.5v-2.5h-2V22H0v-6h1.5v2h2zm6.5-2c.8 0 1.5.7 1.5 1.5v3c0 .8-.7 1.5-1.5 1.5H6.5v-6H10zm0 4.5v-3H8v3h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHdrOff;
impl IconShape for MdHdrOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {

        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hviXmeckntdvA6/Fs70O8Ns8228Dr8W/7K+BnwY+h+f12sBv8S+7Ffht4GOAXV44xL/MPCfxvH4beC2e7XWA3+bZfht4LV503w28D8/ptYHf4kW3CzwE2OUFQ/zLzHMSz+u3gdfi2V4H+G2e7beB1+Jf53WA3+bZXhv4Lf51vgd4b14wxL/MPCfxvH4beC2e7XWA3+bZfht4Lf51vgb4aJ7ttYHf4l/nVuAhvGCIf5l5TuJ5/TbwWjzb6wC/zbP9NvBaPNvrAL/Nc3pt4Ld4tt8BXptne23gt3i23wFem+dlnpN4wRD/MvOcfpvn9dLAcZ7tdYDf5tl+G3gtnu11gN/mOb028Fs82+8Ar82zvTbwWzzb7wCvzfMyz0m8YIh/mfnXex3gt3m23wZei2d7HeC3eU6vDfwWz/Y7wGvzbK8N/BbP9tfAR/O8fpvnJF4wxL/M/Ou9DvDbPNtvA6/Fs70O8Ns8p9cGfotn+x3gtXm21wZ+i3+dZwAP5gVD/MvMv97rAL/Ns/028Fo82+sAv81zem3gt3i23wFem2d7beC3+Nf5HOCzecEQ/zLznF6H5/XVwEvxbK8D/DbP9tvAa/FsrwP8Ns/ptYHf4tl+B3htnu21gd/iRfc9wHvzwiH+ZeY5ief128Br8WyvA/w2z/bbwGvxbK8D/DbP6bWB3+LZfgd4bZ7ttYHf4tkuAX/NFceBl+LZ/gZ4af5liH+ZeU7ief028Fo82+sAv82z/TbwWjzb6wC/zXN6beC3eLbfAV6bZ3tt4Ld4tt8BXpsrjgO3Asd4thPALi8c4l9mnpN4Xr8NvBbP9jrAb/Nsvw28Fv86XwN8NM/22sBv8Wy/A7w2z/bdwHvxbJ8DfDYvHOJfZp6TeF6/DbwWz/Y6wG/zbL8NvBb/Oq8D/DbP9trAb/FsvwO8Ns/2YODpPNutwEN44RD/MvOcxPP6beC1eLbXAX6bZ/tt4LV40X0P8N48p9cGfotn+x3gtXlO3w28F8/2PsB384Ih/mXmOYnn9dvAa/FsrwP8Ns/228Br8S/7G+C7ga/meb028Fs82+8Ar81zem3gt3i23wZehxcM8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSlXZFBPNtPXAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHdrOn;
impl IconShape for MdHdrOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 11.5v-1c0-.8-.7-1.5-1.5-1.5H16v6h1.5v-2h1.1l.9 2H21l-.9-2.1c.5-.3.9-.8.9-1.4zm-1.5 0h-2v-1h2v1zm-13-.5h-2V9H3v6h1.5v-2.5h2V15H8V9H6.5v2zM13 9H9.5v6H13c.8 0 1.5-.7 1.5-1.5v-3c0-.8-.7-1.5-1.5-1.5zm0 4.5h-2v-3h2v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/vpYHXAt6aK16bK24FbgV2gZ8GfgbY5V8H8T/XewGfDTyYF91PA18D/DYvGsT/PG8NfBXwYP7tfhr4GOBWXjjE/yxfBXw0/zF2gbcBfpsXDPE/w3Hgu4C35j/e+wDfzfOHeNG9FvDaXPHSwF8Du8DPALfy7/NdwHvzn+dtgJ/meSFeuOPARwEfDRznBftt4HOA3+Zf76OBr+I/1y7wOsBf85wQL9hLAz8FPJgX3XcDHwPs8qJ5aeCv+K9xK/AQnhPi+Xtp4LeA4/zr/TXwOsAu/7LfAl6b/zrvA3w3z4Z4Xi8N/BZwnH+73wZehxfutYHf4r/WrcBDeDbE8/ot4LV5wX6HK44DL8UL9jHAV/OCfTfwXvzXexvgp7kC8ZzeGvgpnr/PAb4a2OXZHgx8N/BaPK9d4CHALs/fReA4//W+B3hvrkA8p58G3orn9T7Ad/OCfTfwXjyv9wG+m+f12sBv8d9jFzjBFYjnZJ7X9wDvzQt3HLgVOMZz+hngrXlenw18Fv99xBWIZ3tt4Ld4Xi8D/DX/sq8GPorn9NfAy/C8Phv4LP77vA7w2wDi2V4b+C2el3jRvDfwXTwv8by+G3gv/vu8DvDbAOLZXhv4LZ6XeNF8NPBVPC/xvL4beC/++7wO8NsA4tleG/gtntfLAH/Nv+yrgY/iOf0N8NI8r88GPov/Pq8D/DaAeE7meX0P8N68cMeBpwPHeU4/A7w1z+uzgc/iv4+4AvGcfhp4K57X+wDfzQv2U8Bb87zeB/huntdrA7/Ff49LwHGuQDyntwZ+iufvs4GvAXZ5tgcD3wW8Ns/rEvBgYJfnbxc4xn+97wHemysQz+u3gdfiBfttrjgOvDQv2McAX80L9t3Ae/Ff722An+YKxPN6aeC3gWP82/0O8Nq8cK8N/Bb/tZ4BPJhnQzx/Lw38NnCMf72/AV4b2OVf9tvAa/Ff532A7+bZEC/YSwM/DTyIF933AB8N7PKieWngr/iv8TfAS/OcEP+yzwY+GjjGC/Y7wGcDv82/3kcDX8V/rkvAawN/zXNCvOheGnhrrnhp4K+BXeCngVv59/lu4L34z/M2wE/zvBD/MxwHvht4K/7jvQ/w3Tx/iP9Zvhr4KP5jXALeGvhtXjDE/zxvDXw18CD+7X4G+GjgVl44xP9c7w18NvAgXnQ/A3w18Nu8aBD/87008NrAW3PFa3HFM4BbgV3gp4GfBnb510H8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8Bpb6aQeqNd4MAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHdrStrong;
impl IconShape for MdHdrStrong {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 6c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zM5 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/vpYHXAt6aK16bK24FbgV2gZ8GfgbY5V8H8T/XewGfDTyYF91PA18D/DYvGsT/PG8NfBXwYP7tfhr4GOBWXjjE/yxfBXw0/zF2gbcBfpsXDPE/w3Hgu4C35j/e+wDfzfOHeNG9FvDaXPHSwF8Du8DPALfy7/NdwHvzwv0O8Ntc8dfASwPHgbcGHsQL9zbAT/O8EC/cceCjgI8GjvOC/TbwOcBv86/30cBX8fxdAr4a+GpglxfstYHPBl6L528XeB3gr3lOiBfspYGfAh7Mi+67gY8BdnnRvDTwVzx/fwO8NXArL7r3Br4aOMbzuhV4CM8J8fy9NPBbwHH+9f4aeB1gl3/ZbwGvzfP6G+C1gV3+9V4a+G3gGM/rfYDv5tkQz+ulgd8CjvNv99vA6/DCvTbwWzyvvwFeG9jl3+61gd/ied0KPIRnQzyv3wJem3+/jwG+mhfsu4H34nm9DvDb/Pt9NfBRPK+3AX6aKxDP6a2Bn+I/xi7wEGCX5+8icJzn9DPAW/Mf4zhwK3CM5/Q9wHtzBeI5/TTwVvzHeR/gu3lerw38Fs/rbYCf5j/OdwPvxXPaBU5wBeI5mf9YPwO8Nc/rs4HP4nmJ/1hvDfwUz0tcgXi21wZ+i/9Yfw28DM/rs4HP4jn9DvDa/Md6aeCveF6vA/w2gHi21wZ+i/944nl9N/BePKffAV6b/3jmeb0O8NsA4tleG/gt/uOJ5/XdwHvxnH4HeG3+45nn9TrAbwOIZ3tt4Lf4j/U3wEvzvD4b+Cye0+8Ar81/rJcG/orn9TrAbwOI52T+Y/0M8NY8r88GPovnJf5jvTXwUzwvcQXiOf008Fb8x3kf4Lt5Xq8N/BbP622An+Y/zncD78VzugQc5wrEc3pr4Kf4j3EJeDCwy/O3CxzjOf008Db8xzgOPB04znP6HuC9uQLxvH4beC3+/T4G+GpesO8G3ovn9TrAb/Pv99XAR/G83gb4aa5APK+XBn4bOMa/3e8Ar80L99rAb/G8/hp4HWCXf7vXBn6L5/UM4ME8G+L5e2ngt4Fj/Ov9DfDawC7/st8GXovn9dfA6wC7/Ou9NPBbwHGe1/sA382zIV6wlwZ+GngQL7rvAT4a2OVF89LAX/H8/TXwNsCtvOjeG/gq4DjP62+Al+Y5If5lnw18NHCMF+x3gM8Gfpt/vY8GvooX7LOBrwF2ecFeG/gs4LV5/i4Brw38Nc8J8aJ7aeCtueKlgb8GdoGfBm7l3+e7gffihftr4Ke54q+BlwaOA28NPJgX7m2An+Z5If5nOA58N/BW/Md7H+C7ef4Q/7N8NfBR/Me4BLw18Nu8YIj/ed4a+GrgQfzb/Qzw0cCtvHCI/7neG/hs4EG86H4G+Grgt3nRIP7ne2ngtYG35orX4opnALcCu8BPAz8N7PKvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIzgblB1dvsfgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHdrWeak;
impl IconShape for MdHdrWeak {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm12-2c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm0 10c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHealing;
impl IconShape for MdHealing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.73 12.02l3.98-3.98c.39-.39.39-1.02 0-1.41l-4.34-4.34c-.39-.39-1.02-.39-1.41 0l-3.98 3.98L8 2.29C7.8 2.1 7.55 2 7.29 2c-.25 0-.51.1-.7.29L2.25 6.63c-.39.39-.39 1.02 0 1.41l3.98 3.98L2.25 16c-.39.39-.39 1.02 0 1.41l4.34 4.34c.39.39 1.02.39 1.41 0l3.98-3.98 3.98 3.98c.2.2.45.29.71.29.26 0 .51-.1.71-.29l4.34-4.34c.39-.39.39-1.02 0-1.41l-3.99-3.98zM12 9c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-4.71 1.96L3.66 7.34l3.63-3.63 3.62 3.62-3.62 3.63zM10 13c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm2 2c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm2-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2.66 9.34l-3.63-3.62 3.63-3.63 3.62 3.62-3.62 3.63z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z893gb4KuBB/HvI14wxL/M/Nf7HOCzgZcG/op/H/GCIf5l5r/W7wCvzbN9NPBV/NuJFwzxLzP/dZ4BvDSwy3P6aeCt+LcRLxjiX2b+bd4G+GzgpXjRvQzw1zyv48BfAw/iX0+8YIh/mfnX+xzgs4GXBn4bOMa/7H2A7+YFe2ngr/jXEy8Y4l9m/nV+Bnhrnu21gd/ihfse4L35l3008FX864gXDPEvMy+6ZwAvDezynN4b+C6ev78BXhvY5UXz08Bb8aITLxjiX2ZeNJeA1wb+mufvu4H34jldAl4auJUX3XHgr4EH8aIRLxjiX2ZeNO8DfDcv3G8Dr8WzvQ3w0/zrvTTwV7xoxAuG+JeZf9n3AO/Nv+w48NvASwGfA3w2/3YfDXwV/zLxgiH+ZeaF+xvgpXnRvTTw2cBb8+/308Bb8cKJFwzxLzMv2CXgpYFb+e9xHPhr4EG8YOIFQ/zLzAv2OsBv89/rpYG/4gUTLxjiX2aev88BPpv/GT4a+CqeP/GCIf5l5nn9DPDW/M/y08Bb8bzEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjyDdxQRvlagYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImage;
impl IconShape for MdImage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImageAspectRatio;
impl IconShape for MdImageAspectRatio {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 10h-2v2h2v-2zm0 4h-2v2h2v-2zm-8-4H6v2h2v-2zm4 0h-2v2h2v-2zm8-6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h16v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImageNotSupported;
impl IconShape for MdImageNotSupported {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.9,21.9l-8.49-8.49l0,0L3.59,3.59l0,0L2.1,2.1L0.69,3.51L3,5.83V19c0,1.1,0.9,2,2,2h13.17l2.31,2.31L21.9,21.9z M5,18 l3.5-4.5l2.5,3.01L12.17,15l3,3H5z M21,18.17L5.83,3H19c1.1,0,2,0.9,2,2V18.17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImageSearch;
impl IconShape for MdImageSearch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M18 13v7H4V6h5.02c.05-.71.22-1.38.48-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-5l-2-2zm-1.5 5h-11l2.75-3.53 1.96 2.36 2.75-3.54zm2.8-9.11c.44-.7.7-1.51.7-2.39C20 4.01 17.99 2 15.5 2S11 4.01 11 6.5s2.01 4.5 4.49 4.5c.88 0 1.7-.26 2.39-.7L21 13.42 22.42 12 19.3 8.89zM15.5 9C14.12 9 13 7.88 13 6.5S14.12 4 15.5 4 18 5.12 18 6.5 16.88 9 15.5 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O/wPcB78bzEC4b4l5l/nd/hOb0W//neB7gV+C2el3jBEP8y868jnpP5z/U+wHcDrw38Fs9LvGCIf5n51xHPyfzneR/gu7nitYHf4nmJFwzxLzP/OuI5mf8c7wN8N8/22sBv8bzEC4b4l5l/HfGczH+89wG+m+f02sBv8bzEC4b4l5l/HfGczH+s9wG+m+f12sBv8bzEC4b4l5nn73eA1+bf5reB1+Jf732A7+b5e23gt3he4gVD/MvM8/c7wGvzb/PbwGvxr/M+wHfzgr028Fs8L/GCIf5l5vn7HeC1+bf5beC1eNG9D/DdvHCvDfwWz0u8YIh/mXn+fgd4bf5tfht4LV407wN8N/+y1wZ+i+clXjDEv8z864jnZP7t3gf4bl40rw38Fs9LvGCIf5n51xHPyfzbvA/w3bzoXhv4LZ6XeMEQ/zLzryOek/nXex/gu/nXeW3gt3he4gVD/MvMv454TuZf532A7+Zf77WB3+J5iRcM8S8z/zriOZkX3fsA382/zWsDv8XzEi8Y4l9m/nV+m+f02rxo3gf4bv7tXhv4LZ6XeMEQ/zLzn+99gO/m3+e1gd/ieYkXDPEvM/+53gf4bv79Xhv4LZ6XeMEQ/zLzn+d9gO/mP8ZrA7/F8xIvGOJfZv5zvA/w3fzHeW3gt3he4gVD/MvMf7z3Ab6b/1ivDfwWz0u8YIh/mfmP9T7Ad/Mf77WB3+J5iRcM8S8z/3HeB/hu/nO8NvBbPC/xgiH+ZeY/xvsA381/ntcGfovnJV4wxL/M/Pu9D/Dd/Od6beC3eF7iBUP8y8y/z/sA381/vtcGfovnJV4wxL/M/Nu9D/Dd/Nd4beC3eF7iBUP8y8y/zfsA382/7Lf41/sY4K95Tq8N/BbPS7xgiH+Z+dd7H+C7edGYf73XAX6b5/TawG/xvMQLhviXmX+d9wG+mxed+dd7HeC3eU6vDfwWz0u8YIh/mXnRvQ/w3fzrmH+91wF+m+f02sBv8bzEC4b4l5kXzfsA382/3mvzr/fXwC7P6bWB3+J5iRcM8S8z/7L3Ab6b/16vDfwWz0u8YIh/mXnh3gf4bv77vTbwWzwv8YIh/mXmBXsf4Lv5n+G1gd/ieYkXDPEvM8/f+wDfzf8crw38Fs9LvGCIf5l5Xt8NfDf/s7w08NU8L/GCIf5lu8Ax/ne6BBznBUP8y74beC/+d/oe4L15wRD/sgcDT+d/p4cAt/KCIV407w18F/+7vA/w3bxwiBfdg4HPBt4aOMb/TJeAnwY+G7iVfxni/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wi6YqdBY9CPMAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdIso;
impl IconShape for MdIso {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM5.5 7.5h2v-2H9v2h2V9H9v2H7.5V9h-2V7.5zM19 19H5L19 5v14zm-2-2v-1.5h-5V17h5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/nwcBnAe/Dfz7E/zx/Bbw08DHAV/OfC/E/y1cDH8WzvQzw1/znQfzP8dbAT/GcbgVeBtjlPwfif4YHA38FHOd5/TTwNvznQPzP8FfAS/OCfQzw1fzHQ/z3+2rgo/iXvQzw1/zHQvz3emvgp3jR3Aq8DLDLfxzEf58HA38FHOdF99PA2/AfB/Hf56+Al+Zf72OAr+Y/BuI/znFglxfNVwMfxb/dywB/zb8f4j/GceCvgI8BfpoX7q2Bn+Lf51bgZYBd/n0Q/zF+CnhrYBd4GeBWnr8HA38FHOff76eBt+HfB/Hv99HAV/Fsfw28DM/fZwOfxX+cjwG+mn87xL/PSwN/xfP6GuCjef7+Gngp/uO8DPDX/Nsg/u2OA38FPJjn722An+Z5PRj4a+AY/zFuBV4G2OVfD/Fv91PAW/OC7QIvA9zK83pr4Kf4j/PTwNvwr4f4t/lo4Kv4l/018DI8f18NfBT/cT4G+Gr+dRD/ei8N/BUvuq8BPprn76+Bl+I/zssAf82LDvGvcxz4K+DB/Ou8DfDTPK8HA38NHOM/xq3AywC7vGgQ/zo/Bbw1/3q7wMsAt/K83hr4Kf7j/DTwNrxoEC+6jwa+in+7vwZehufvq4GP4j/OxwBfzb8M8aJ5aeCv+Pf7GuCjef7+Gngp/uO8DPDXvHCIf9lx4K+AB/Mf422An+Z5vTXwU/zHuRV4GWCXFwzxL/sp4K35j7MLvAxwK8/2UcBX8x/vp4G34QVDvHAfDXwV//H+GngZ4DjwXcBb85/nY4Cv5vlDvGAvDfwV/3l+Gnhp4MH853sZ4K95Xojn7zjwV8CD+b/hVuBlgF2eE+L5+yngrfm/5aeBt+E5IZ7XRwNfxf9NHwN8Nc+GeE4vDfwV/7e9DPDXXIF4tuPAXwEP5v+2W4GXAXYBxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EaTza0HigazgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLandscape;
impl IconShape for MdLandscape {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLeakAdd;
impl IconShape for MdLeakAdd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 3H3v3c1.66 0 3-1.34 3-3zm8 0h-2c0 4.97-4.03 9-9 9v2c6.08 0 11-4.93 11-11zm-4 0H8c0 2.76-2.24 5-5 5v2c3.87 0 7-3.13 7-7zm0 18h2c0-4.97 4.03-9 9-9v-2c-6.07 0-11 4.93-11 11zm8 0h3v-3c-1.66 0-3 1.34-3 3zm-4 0h2c0-2.76 2.24-5 5-5v-2c-3.87 0-7 3.13-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLeakRemove;
impl IconShape for MdLeakRemove {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 3H8c0 .37-.04.72-.12 1.06l1.59 1.59C9.81 4.84 10 3.94 10 3zM3 4.27l2.84 2.84C5.03 7.67 4.06 8 3 8v2c1.61 0 3.09-.55 4.27-1.46L8.7 9.97C7.14 11.24 5.16 12 3 12v2c2.71 0 5.19-.99 7.11-2.62l2.5 2.5C10.99 15.81 10 18.29 10 21h2c0-2.16.76-4.14 2.03-5.69l1.43 1.43C14.55 17.91 14 19.39 14 21h2c0-1.06.33-2.03.89-2.84L19.73 21 21 19.73 4.27 3 3 4.27zM14 3h-2c0 1.5-.37 2.91-1.02 4.16l1.46 1.46C13.42 6.98 14 5.06 14 3zm5.94 13.12c.34-.08.69-.12 1.06-.12v-2c-.94 0-1.84.19-2.66.52l1.6 1.6zm-4.56-4.56l1.46 1.46C18.09 12.37 19.5 12 21 12v-2c-2.06 0-3.98.58-5.62 1.56z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5v+UEsMtzQjx/rw38Fv+3vA7w2zwnxPP30cBX8X/LxwBfzXNCPH+fDXwW/7d8DvDZPCfE8/fZwGfxf8vnAJ/Nc0I8f78NvBb/t/wM8NY8J8Tz99vAa/F/y88Ab81zQjx/nw18Fv+3fA7w2TwnxPP32cBn8X/L5wCfzXNCPH8fDXwV/7d8DPDVPCfE8/fawG/xf8vrAL/Nc0I8fw8Gns7/LSeAXZ4T4gX7a+Cl+L/hb4CX5nkhXrCvBj6K/xu+BvhonhfiBXtt4Lf4v+F1gN/meSFeuFuBB/G/2zOAB/P8IV64zwY+i//dPgf4bJ4/xAt3HLgVOMb/TpeABwO7PH+If9lnA5/F/06fA3w2LxjiX3YcuBU4xv8ul4AHA7u8YIgXzXsD38X/Lm8D/DQvHOJF99PAW/G/w88Ab82/DPGiOw78NfAg/md7BvDSwC7/MsS/zksDvw0c43+mS8BrA3/Niwbxr/fSwG8Dx/if5RLw2sBf86JD/Nu8NPDbwDH+Z7gEvDbw1/zrIP7tXhr4aeBB/Pd6BvDWwF/zr4f49zkOfDfwVvz3+BngvYFd/m0Q/zHeG/hq4Bj/NS4B7w38NP8+iP84x4GPBj4aOMZ/jkvAVwNfDezy74f4j3cc+GjgvYEH8R/jGcB3A18N7PIfB/Gf67WBtwZeG3gp/nX+Bvht4KeB3+Y/B+K/znHgpYGXBo5zxUtzxV9zxS7w18BfA7v850P8/4b4/w3x/xvi/zfE/2/8I61/kEGQU631AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLens;
impl IconShape for MdLens {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLinkedCamera;
impl IconShape for MdLinkedCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "14",
                r: "5",
            }
            path {
                d: "M16 3.33c2.58 0 4.67 2.09 4.67 4.67H22c0-3.31-2.69-6-6-6v1.33M16 6c1.11 0 2 .89 2 2h1.33c0-1.84-1.49-3.33-3.33-3.33V6",
            }
            path {
                d: "M24 0H0v24h24V0z",
            }
            path {
                d: "M17 9c0-1.11-.89-2-2-2V4H9L7.17 6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9h-5zm-5 10c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooks;
impl IconShape for MdLooks {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 10c-3.86 0-7 3.14-7 7h2c0-2.76 2.24-5 5-5s5 2.24 5 5h2c0-3.86-3.14-7-7-7zm0-4C5.93 6 1 10.93 1 17h2c0-4.96 4.04-9 9-9s9 4.04 9 9h2c0-6.07-4.93-11-11-11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DXAX/MvQ7xo3hv4Lv53eR/gu3nhEP+yBwNP53+nlwH+mhcM8S/7buC9+N/pZ4C35gVD/MsuAsf532kXOMELhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5l/n9/hX+e1+I8lXjDEv8z8+4h/nePARwOfxX8M8YIh/mXm30f827w38F38+4kXDPEvM/8+4t/uu4H34t9HvGCIf5n59xH/di8N/BX/PuIFQ/zLzL+P+Pcx/z7iBUP8y8x/jd8BXpvnZf59xAuG+JeZ/xq/A7w2z8v8+4gXDPEvM/81fgd4bZ6X+fcRLxjiX2b+a/wO8No8L/PvI14wxL/M/Nf4HeC1eU7HgYv8+4gXDPEvM//xPgb4a57TLvDXPKe3Bn6Kfx/xgiH+ZeY/3usAv80Ldxz4K+DB/PuIFwzxLzP/8V4H+G2ev+PAawFfDTyYfz/xgiH+ZeY/3usAv83zdxx4beCrgQfx7ydeMMS/zPzHex3gt3nhjgN/DTyIfx/xgiH+ZeY/3kcDf81zugT8Nc/prYGf4t9HvGCIf5n5r/E7wGvznI4DF/n3ES8Y4l9m/mv8DvDaPC/z7yNeMMS/zPzX+B3gtXle5t9HvGCIf5n5r/E7wGvzvMy/j3jBEP8y81/jd4DX5nmZfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f827008Ff8+4gXDPEvM/8+4t/uu4H34t9HvGCIf5n59xH/Nu8NfBf/fuIFQ/zLzL+P+Nc5DnwU8Nn8xxAvGOJfZv59fpt/ndfmP5Z4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/7Jd4Bj/O10CjvOCIf5l3w28F/87/Qzw1rxgiH/Zg4G/Bo7xv8/LAH/NC4Z40bw38NXAMf73eB/gu3nhEC+6BwOfDbw1cIz/mS4BPw18NfDX/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ec+5W5BUWruxwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooks3;
impl IconShape for MdLooks3 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.01 3h-14c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 7.5c0 .83-.67 1.5-1.5 1.5.83 0 1.5.67 1.5 1.5V15c0 1.11-.9 2-2 2h-4v-2h4v-2h-2v-2h2V9h-4V7h4c1.1 0 2 .89 2 2v1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD2klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz7/A7P66WBYzynS8Bf87xei38f8YIh/mXm30c8r98GXovn9DvAa/O8zL+PeMEQ/zLz7yOe128Dr8Vz+h3gtXle5t9HvGCIf5n59xHP67eB1+I5/Q7w2jwv8+8jXjDEv8z8+4jn9dvAa/Gcfgd4bZ6X+fcRLxjiX2b+fcTz+m3gtXhOvwO8Ns/L/PuIFwzxLzP/PuJ5/TbwWjyn3wFem+dl/n3EC4b4l5l/H/G8fht4LZ7T7wCvzfMy/z7iBUP8y8y/j3hevw28Fs/pd4DX5nmZfx/xgiH+ZebfRzyv3wZei+f0O8Br87zMv494wRD/MvPvI57XbwOvxXP6HeC1eV7m30e8YIh/mfn3Ec/rpYHjPKdd4K95XubfR7xgiH+Z+fcR/z7m30e8YIh/mfn3Ef8+5t9HvGCIf5n59xH/PubfR7xgiH+Z+fcR/z7m30e8YIh/mfn3eW1eNJeAv+Z5mX8f8YIh/mXmv8bvAK/N8zL/PuIFQ/zLzH+N3wFem+dl/n3EC4b4l5n/Gr8DvDbPy/z7iBcM8S8z/zV+B3htnpf59xEvGOJfZv5r/A7w2jwv8+8jXjDEv8z81/gd4LV5XubfR7xgiH+Z+a/xO8Br87zMv494wRD/MvNf43eA1+Z5mX8f8YIh/mXmv8bvAK/N8zL/PuIFQ/zLzH+N3wFem+dl/n3EC4b4l5n/GrvAX/O8Xpt/H/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IhkJ/QfXplfcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooks4;
impl IconShape for MdLooks4 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 14h-2v-4H9V7h2v4h2V7h2v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz7/A7/Pq/Fv494wRD/MvPvI/59zL+PeMEQ/zLz7yP+fcy/j3jBEP8y8+8j/n3Mv494wRD/MvPvI/59zL+PeMEQ/zLz7yP+fcy/j3jBEP8y8+8jntdvA6/Ffw3xgiH+ZebfRzyv3wZei/8a4gVD/MvMv494Xr8NvBb/NcQLhviXmX8f8bx+G3gt/muIFwzxLzP/PuJ5/TbwWvzXEC8Y4l9m/n3E83pp4DjP67WBlwbeiv844gVD/MvMv4/413tp4KeBB/HvJ14wxL/M/PuIf5sHA38NHOPfR7xgiH+Z+fcR/3afDXwW/z7iBUP8y8y/j/i3ezDwdP59xAuG+JeZf5/X5kVzCfhrnpf59xEvGOJfZv5r/A7w2jwv8+8jXjDEv8z81/gd4LV5XubfR7xgiH+Z+a/xO8Br87zMv494wRD/MvNf43eA1+Z5mX8f8YIh/mXmv8bvAK/N8zL/PuIFQ/zLzL+P+Pcx/z7iBUP8y8y/j/i3e23gt/j3ES8Y4l9m/n3Ev913A+/Fv494wRD/MvPvI/5t3hv4Lv79xAuG+JeZfx/xr3Mc+Cjgs/mPIV4wxL/M/Pv8Nv86r81/LPGCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8I5QNhQYIdZ7cAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooks5;
impl IconShape for MdLooks5 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 6h-4v2h2c1.1 0 2 .89 2 2v2c0 1.11-.9 2-2 2H9v-2h4v-2H9V7h6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADzElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+h3+dV6Lfx/xgiH+ZeY/xucAXw3s8q9j/n3EC4b4l5l/v/cBvpt/G/PvI14wxL/M/Pt8D/De/NuZfx/xgiH+Zebf52WAv+bfzvz7iBcM8S8z/z7i38f8+4gXDPEvM/8+4nn9NvBa/NcQLxjiX2b+fcTz+m3gtfivIV4wxL/M/PuI5/XbwGvxX0O8YIh/mfn3Ec/rt4HX4r+GeMEQ/zLz7yOe128Dr8V/DfGCIf5l5t9HPK+XBo7zvF4beGngrfiPI14wxL/M/PuIf72XBn4aeBD/fuIFQ/zLzL+P+Ld5MPDXwDH+fcQLhviXmX8f8W/32cBn8e8jXjDEv8z8+4h/uwcDT+ffR7xgiH+Z+fcRz+ulgWM8p0vAX/O8zL+PeMEQ/zLz7yOe128Dr8Vz+h3gtXle5t9HvGCIf5n59xHP67eB1+I5/Q7w2jwv8+8jXjDEv8z8+4jn9dvAa/Gcfgd4bZ6X+fcRLxjiX2b+fcTz+m3gtXhOvwO8Ns/L/PuIFwzxLzP/PuJ5/TbwWjyn3wFem+dl/n3EC4b4l5l/H/HvY/59xAuG+JeZf5+XAf6af5vXBn6Lfx/xgiH+Zebf53uA9+bf5ruB9+LfR7xgiH+Z+fd7H+C7+dd5b+C7+PcTLxjiX2b+Y3w28DXALi/cceCjgM/mP4Z4wRD/MvMf67d54V6b/1jiBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lDwaezv9ODwFu5QVDvGjeG/gu/nd5H+C7eeEQL7oHA58NvDVwjP+ZLgE/DXw2cCv/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ER/fcEEZ7TsOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooks6;
impl IconShape for MdLooks6 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 15h2v-2h-2v2zm8-12H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 6h-4v2h2c1.1 0 2 .89 2 2v2c0 1.11-.9 2-2 2h-2c-1.1 0-2-.89-2-2V9c0-1.11.9-2 2-2h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADnklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzHuAT8NS+alwaO8R9DvGCIf5n5j/E7wGvzovlt4LX4jyFeMMS/zPzH+B3gtXnR/DbwWvzHEC8Y4l9m/mP8DvDavGh+G3gt/mOIFwzxLzP/MX4HeG1eNL8NvBb/McQLhviXmf8YvwO8Ni+a3wZei/8Y4gVD/MvMfy7xvH4beC3+Y4gXDPEvM/+5xPP6beC1+I8hXjDEv8z85xLP67eB1+I/hnjBEP8y859LPK/fBl6L/xjiBUP8y8x/LvG8fht4Lf5jiBcM8S8z/7nE8/pt4LX4jyFeMMS/zPznEs/rt4HX4j+GeMEQ/zLzn0s8r98GXov/GOIFQ/zLzH8u8bx+G3gt/mOIFwzxLzP/ucTz+m3gtfiPIV4wxL/M/OcSz+u3gdfiP4Z4wRD/MvOfSzyv3wZei/8Y4gVD/MvMfy7xvH4beC3+Y4gXDPEvM/+5xPP6beC1+I8hXjDEv8z85xLP67eB1+I/hnjBEP8y859LPK/fBl6L/xjiBUP8y8x/LvG8fht4Lf5jiBcM8S8z/7nE8/pt4LX4jyFeMMS/zPznEs/rt4HX4j+GeMEQ/zLzn0s8r98GXov/GOIFQ/zLzH8u8bx+G3gt/mOIFwzxLzP/uX6b5/XSwHH+Y4gXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHXO9gQX/IYIEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooksOne;
impl IconShape for MdLooksOne {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14h-2V9h-2V7h4v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz7/A7/Oq/FfyzxgiH+ZebfR/zrHAc+Gvgs/mOIFwzxLzP/PuLf5r2B7+LfT7xgiH+Z+fcR/3bfDbwX/z7iBUP8y8y/j/i3e23gt/j3ES8Y4l9m/n3Ev4/59xEvGOJfZv5r/A7w2jwv8+8jXjDEv8z81/gd4LV5XubfR7xgiH+Z+a/xO8Br87zMv494wRD/MvNf43eA1+Z5mX8f8YIh/mXmv8bvAK/N8zL/PuIFQ/zLzH+cnwH+Gvhtntcu8Nc8L/PvI14wxL/M/Ps9A3hr4K/513kw8HT+fcQLhviXmX+fS8BLA7fyr/fZwGfx7yNeMMS/zPz7fA7w2fzrvTTwW8Bx/n3EC4b4l5l/n4cAt/Kv89LATwEP5t9PvGCIf5n59xHP66WBYzyv1wZeGnhr/uOIFwzxLzP/PuJ5/TbwWvzXEC8Y4l9m/n3E8/pt4LX4ryFeMMS/zPz7iOf128Br8V9DvGCIf5n59xHP67eB1+K/hnjBEP8y8+8jntdvA6/Ffw3xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t/nt/n3eW3+fcQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj40FpQfmToFcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLooksTwo;
impl IconShape for MdLooksTwo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 8c0 1.11-.9 2-2 2h-2v2h4v2H9v-4c0-1.11.9-2 2-2h2V9H9V7h4c1.1 0 2 .89 2 2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLoupe;
impl IconShape for MdLoupe {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10h8c1.1 0 2-.9 2-2v-8c0-5.51-4.49-10-10-10zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMicExternalOff;
impl IconShape for MdMicExternalOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.19 21.19L2.81 2.81 1.39 4.22 5.17 8H4l1 10h1c0 2.21 1.79 4 4 4s4-1.79 4-4v-1.17l5.78 5.78 1.41-1.42zM12 18c0 1.1-.9 2-2 2s-2-.9-2-2h1l.56-5.61L12 14.83V18zm2-12v5.17l-2-2V6c0-2.21 1.79-4 4-4s4 1.79 4 4v11.17l-2-2V6c0-1.1-.9-2-2-2s-2 .9-2 2zm-4-1c0 .62-.2 1.18-.52 1.66L5.33 2.51C5.81 2.19 6.38 2 7 2c1.66 0 3 1.34 3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMicExternalOn;
impl IconShape for MdMicExternalOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.22 7H4.78C4.3 6.47 4 5.77 4 5c0-1.66 1.34-3 3-3s3 1.34 3 3c0 .77-.3 1.47-.78 2zM16 2c2.21 0 4 1.79 4 4v16h-2V6c0-1.1-.9-2-2-2s-2 .9-2 2v12c0 2.21-1.79 4-4 4s-4-1.79-4-4H5L4 8h6L9 18H8c0 1.1.9 2 2 2s2-.9 2-2V6c0-2.21 1.79-4 4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMonochromePhotos;
impl IconShape for MdMonochromePhotos {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M-74 29h48v48h-48V29z",
            }
            path {
                d: "M20 5h-3.2L15 3H9L7.2 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 14h-8v-1c-2.8 0-5-2.2-5-5s2.2-5 5-5V7h8v12zm-3-6c0-2.8-2.2-5-5-5v1.8c1.8 0 3.2 1.4 3.2 3.2s-1.4 3.2-3.2 3.2V18c2.8 0 5-2.2 5-5zm-8.2 0c0 1.8 1.4 3.2 3.2 3.2V9.8c-1.8 0-3.2 1.4-3.2 3.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMotionPhotosOff;
impl IconShape for MdMotionPhotosOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.84 20.84L3.16 3.16 1.89 4.43l1.89 1.89C2.66 7.93 2 9.89 2 12c0 5.52 4.48 10 10 10 2.11 0 4.07-.66 5.68-1.77l1.89 1.89 1.27-1.28zM12 20c-4.41 0-8-3.59-8-8 0-1.55.45-3 1.22-4.23l1.46 1.46C6.25 10.06 6 11 6 12c0 3.31 2.69 6 6 6 1 0 1.94-.25 2.77-.68l1.46 1.46C15 19.55 13.55 20 12 20zM6.32 3.77C7.93 2.66 9.89 2 12 2c5.52 0 10 4.48 10 10 0 2.11-.66 4.07-1.77 5.68l-1.45-1.45C19.55 15 20 13.55 20 12c0-4.41-3.59-8-8-8-1.55 0-3 .45-4.23 1.22L6.32 3.77zM18 12c0 1-.25 1.94-.68 2.77L9.23 6.68C10.06 6.25 11 6 12 6c3.31 0 6 2.69 6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMotionPhotosOn;
impl IconShape for MdMotionPhotosOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10,16.5v-9l6,4.5L10,16.5z M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3 C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2 C17.52,2,22,6.48,22,12z M5.5,4C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMotionPhotosPause;
impl IconShape for MdMotionPhotosPause {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8 s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2C17.52,2,22,6.48,22,12z M5.5,4 C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z M18,12c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6S18,8.69,18,12z M11,9H9v6h2V9z M15,9h-2v6h2V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMotionPhotosPaused;
impl IconShape for MdMotionPhotosPaused {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8 s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2C17.52,2,22,6.48,22,12z M5.5,4 C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z M11,16V8H9v8H11z M15,16V8h-2v8H15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE6UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDGe7RnArbxgDwYexHO6BPw1L9hx4KV4Xr/DC/davGguAX/Nc0K8cK8NfBfwYJ7TbwOvwwv22sBv8bweAtzKC3Yr8CCe0/sA380L9t3Ae/Ev+x3gtXlOiBfsrYGf4gV7HeC3ecF+G3gtntP3AO/NC/bewHfxnG4FHsIL9mDg6fzLfgd4bZ4T4vk7DjwdOM4L9tvA6/CCvTbwWzyvhwC38oLdCjyI5/Q+wHfzgn038F68cL8DvDbPCfH8vTfwXfzLXgf4bV6w3wZei+f0PcB784K9N/BdPKdbgYfwgj0YeDov3O8Ar81zQjx/Xw18FP+y3wZehxfstYHf4nk9BLiVF+xW4EE8p/cBvpsX7LuB9+IF+x3gtXlOiOfvt4HX4kXzOsBv84L9NvBaPKfvAd6bF+y9ge/iOd0KPIQX7MHA03nBfgd4bZ4T4vn7beC1eNH8NvA6vGCvDfwWz+shwK28YLcCD+I5vQ/w3bxg3w28F8/f7wCvzXNCPH+/DbwWL7rXAX6bF+y3gdfiOX0P8N68YO8NfBfP6VbgIbxgDwaezvP3O8Br85wQz99vA6/Fi+63gdfhBXtt4Ld4Xg8BbuUFuxV4EM/pfYDv5gX7buC9eF6/A7w2zwnx/P028Fr867wO8Nu8YL8NvBbP6XuA9+YFe2/gu3hOtwIP4QV7MPB0ntfvAK/Nc0I8f78NvBb/OrcCt/KCPRh4MM/rt3nhXpvn9dfALi/YSwPHeU6/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8fTfwXvzf8jXAR/OcEM/fRwNfxf8tHwN8Nc8J8fwdB24FjvF/wyXgwcAuzwnxgr038F383/A2wE/zvBAv3GsD3w08iP+dngG8N/DbPH+IF81LA8f532UX+GteOMT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzO2oEEI6DBaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMovieCreation;
impl IconShape for MdMovieCreation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 4l2 4h-3l-2-4h-2l2 4h-3l-2-4H8l2 4H7L5 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMovieFilter;
impl IconShape for MdMovieFilter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 4l2 3h-3l-2-3h-2l2 3h-3l-2-3H8l2 3H7L5 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4h-4zm-6.75 11.25L10 18l-1.25-2.75L6 14l2.75-1.25L10 10l1.25 2.75L14 14l-2.75 1.25zm5.69-3.31L16 14l-.94-2.06L13 11l2.06-.94L16 8l.94 2.06L19 11l-2.06.94z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxgfwO8N/DX/Ou9N/DVwDGe1+sAv82/7KWB7wZeihdMvGCIf5l5/p4BvDSwy7/dewPfxfN6HeC3edEcB24FjvH8iRcM8S8zz9/vAK/Nv595Xq8D/DYvut8GXovnT7xgiH+Zef5+B3ht/v3M83od4Ld50f028Fo8f+IFQ/zLzPP3O8Br82yvDfwWV7wO8Ns8228Dr8UV4jmZ5/U6wG/zbL8NvBbP9jrAb/Nsvw28Fs+feMEQ/zLz/P0O8No822sDv8UVrwP8Ns/228BrcYV4TuZ5vQ7w2zzbbwOvxbO9DvDbPNtvA6/F8ydeMMS/zDx/vwO8Ns/22sBvccXrAL/Ns/028FpcIZ6TeV6vA/w2z/bbwGvxbK8D/DbP9tvAa/H8iRcM8S8zz9/vAK/Ns7028Ftc8TrAb/Nsvw28FleI52Se1+sAv82z/TbwWjzb6wC/zbP9NvBaPH/iBUP8y8zz9zvAa/Nsrw38Fle8DvDbPNtvA6/FFeI5mef1OsBv86L7beC1eP7EC4b4l5nn73eA1+bZXhv4La54HeC3ebbfBl6LK8RzMs/rdYDf5kX328Br8fyJFwzxLzPP3+8Ar82zvTbwW1zxOsBv82y/DbwWV4jnZJ7X6wC/zYvmOPB04DjPn3jBEP8y8/z9DvDaPNtrA7/FFa8D/DbP9tvAa3GFeE7meb0O8Nv8y14a+C7gpXnBxAuG+JeZ5+93gNfm2V4b+C3+ZeI5mef1OsBv82y/DbwW/zbiBUP8y8zz9zvAa/Nsrw38Fv8y8ZzM83od4Ld5tt8GXot/G/GCIf5l5vn7HeC1ebbXBn6Lf5l4TuZ5vQ7w2zzbbwOvxb+NeMEQ/zLz/P0O8No822sDv8W/TDwn87xeB/htnu23gdfi30a8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/swcDT+d/pIcCtvGCIF817A9/F/y7vA3w3LxziRfdg4LOBtwaO8T/TJeCngc8GbuVfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Aq2zpEGyoJbXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMp;
impl IconShape for MdMp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM6.5 9H11c.55 0 1 .45 1 1v5h-1.5v-4.5h-1v3H8v-3H7V15H5.5v-5c0-.55.45-1 1-1zm9 6H14V9h3.5c.55 0 1 .45 1 1v2.5c0 .55-.45 1-1 1h-2V15zm0-3H17v-1.5h-1.5V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADTElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b472f+fcS/HeK/n/n3Ef92iP9+5t9H/Nsh/vuZfx/xb4f472f+fcS/HeK/n/n3Ef92iP9+5t9H/Nsh/vuZfx/xb4f472f+fcS/HeK/n/n3Ef92iP9+5nn9DbDLi+a1+bdD/Pczz+t1gN/mPx/iv595Xq8D/Db/+RD//czzeh3gt/nPh/jvZ57X6wC/zX8+xH8/87xeB/ht/vMh/vuZ5/U6wG/znw/x3888r9cBfpv/fIj/fuZ5vQ7w2/znQ/z3M8/rdYDf5j8f4r+feV6vA/w2//kQ//3M83od4Lf5z4f472ee1+sAv81/PsR/P/O8Xgf4bf7zIf77mef1OsBv858P8d/PPK/XAX6b/3yI/3wPBh7Esz0DuJVnM8/rdYDf5j8f4j/HceC9gPcGXprntQt8N/A1wNN5Xq8D/Db/+RD/8d4a+C7gOP92rwP8Nv/5EP+xvgr4aP79Xgf4bf7zIf7jfDTwVfzHeB3gt/nPh/iP8drAb/Ef53WA3+Y/H+I/xm8Br81/nNcBfpv/fIh/v7cGfor/WK8D/Db/+RD/fl8NfBT/sV4H+G3+8yH+/X4beC3+Y70O8Nv850P8+5n/eK8D/Db/+RD/fuY/3usAv81/PsS/363Ag/iP9TrAb/OfD/Hv99vAa/Ef6wSwy38+xL/fRwNfxX+cvwFemv8aiH+/BwNP5z/OxwBfzX8NxH+M7wbei3+/ZwAvDezyXwPxH+M4cCtwjH+ftwF+mv86iP84Lw38Ff92nwN8Nv+1EP+xXhr4beAY/zofA3w1//UQ//GOA18NvBf/st8BPhv4bf57IP7zPBh4a+CtgQcDD+KK3wH+Gvhp4Lf574X4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwI8dGBBwRujxwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMusicNote;
impl IconShape for MdMusicNote {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FXA2wC7/Mcx/z7i3w7xonlp4LeA48BfA68D7PIfw/z7iH87xL/spYHfAo7zbH8NvA6wy7+f+fcR/3aIf9lvA6/F8/pr4HWAXf59zL+P+LdD/MuOA78NvBTP66+B1wF2+bcz/z7i3w7xojkO/DbwUjyvvwZeB9jl38b8+4h/O8SL7jjw28BL8bz+GngdYJd/PfPvI/7tEP86x4HfBl6K5/XXwOsAu/zrmH8f8W+H+Nc7Dvw28FI8r78GXgfY5UVn/n3Evx3i3+Y48NvAS/G8/hp4HWCXF4359xH/doh/u+PAbwMvxfP6a+B1gF3+ZeZ5/Q2wy4vmtfm3Q/z7HAd+G3gpntdfA68D7PLCmef1OsBv858P8e93HPht4KV4Xn8NvA6wywtmntfrAL/Nfz7Ef4zjwG8DL8Xz+mvgdYBdnj/zvF4H+G3+8yH+4xwHfht4KZ7XXwOvA+zyvN4b+C6e0+sAv81/PsR/rOPAbwMvxfP6a+B1gF2e13sD38WzvQ7w2/znQ/zHOw78NvBSPK+/Bl4H2OV5vTfwXVzxOsBv858P8Z/jOPDbwEvxvP4aeB1gl+f13sB3Aa8D/Db/+RD/eY4Dvw28FM/rr4HXAXZ5Xu8N3Ar8Nv/5EP+5jgO/DbwUz+uvgdcBdvnvg/jPdxz4beCleF5/DbwOsMt/D8R/jePAbwMvxfP6a+B1gF3+6yH+6xwHfht4KZ7XXwOvA+zyXwvxX+s48NvAS/G8/hp4HWCX/zqI/3rHgd8GXorn9dfA6wC7/NdA/Pc4Dvw28FI8r78GXgfY5T8f4j/fg4EH8WzPAG4FjgO/DbwUz+uvgdcBdvnPhfjPcRx4L+C9gZfmee0C3w18D/BXPH9/DbwOsMt/HsR/vLcGvgs4zr/fXwOvA+zynwPxH+urgI/mP9ZfA68D7PIfD/Ef56OBr+I/x18DrwPs8h8L8R/jtYHf4j/XXwOvA+zyHwfxH+O3gNfmP84l4BjP66+B1wF2+Y+B+Pd7a+Cn+I/1NsBnAy/F8/pr4HWAXf79EP9+Xw18FP+xXgf4a+C3gZfief018DrALv8+iH+/3wZei/9YrwP8NnAc+G3gpXhefw28DrDLvx3i38/8x3sd4Le54jjw28BL8bz+GngdYJd/G8S/n/mP9zrAb/Nsx4HfBl6K5/XXwOsAu/zrIf79bgUexH+s1wF+m+d0HPht4KV4Xn8NvA6wy78O4t/vt4HX4j/WCWCX53Uc+G3gpXhefw28DrDLiw7x7/fRwFfxH+dvgJfmBTsO/DbwUjyvvwZeB9jlRYP493sw8HT+43wM8NW8cMeB3wZeiuf118DrALv8yxD/Mb4beC/+/Z4BvDSwy7/sOPDbwEvxvP4aeB1glxcO8R/jOHArcIx/n7cBfpoX3XHgt4GX4nn9NfA6wC4vGOI/zksDf8W/3ecAn82/3nHgt4GX4nn9DvDavGCI/1gvDfw2cIx/nY8Bvpp/u+PAbwMvxbNdAl4b+GteMMR/vOPAVwPvxb/sd4DPBn6bf7/jwG8DLwVcAl4b+GteOMR/ngcDbw28NfBg4EFc8TvAXwM/Dfw2/7GOAz8NfDTw1/zLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHif7RQbxlNU4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMusicOff;
impl IconShape for MdMusicOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.27 3L3 4.27l9 9v.28c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4v-1.73L19.73 21 21 19.73 4.27 3zM14 7h4V3h-6v5.18l2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR4MPAj4G2CX/3qI/zoPBt4KeGvgtXn+doG/Bn4a+BngVv5zIf7zvRfw2cCD+df7a+BzgJ/mPwfiP89rA98FPJh/v98GPgf4bf5jIf5zfBfw3vzH+2rgY/iPg/iPdRz4LeCl+c/z08D7ALv8+yH+4xwHfgt4af7z/TXwOsAu/z6I/zh/Bbw0/3V+Gngb/n0Q/zG+C3hv/ut9DvDZ/Nsh/v1eG/gt/vu8DvDb/Nsg/v2eDjyY/z6/DbwO/zaIf5/3Br6L/36vA/w2/3qIf5+nAw/mv9/PAG/Nvx7i3+7BwNP5n+MEsMu/DuLf7qOBr+J/jvcBvpt/HcS/3W8Dr8X/HD8DvDX/Ooh/O/M/y+8Ar82/DuLfzvzPI/51EP82Dwaezv884l8H8W/z2sBv8T+P+NdB/Ns8GHg6//OIfx3Ev535n0f86yD+7cz/LL8DvDb/Ooh/u98GXov/Ob4HeG/+dRD/dh8NfBX/c7wP8N386yD+7V4a+Cv+5zgB7PKvg/j3+W3gtfjv9zvAa/Ovh/j3eW/gu/jv9zrAb/Ovh/j3uxV4EP99fgd4bf5tEP9+rw38Fv89LgGvDfw1/zaI/xifDXwW//XeB/hu/u0Q/3H+Gngp/ut8D/De/Psg/uMcB34beCn+830P8N78+yH+Yx0Hfhp4Lf7zfA/w3vzHQPzn+Gzgs/iPdQn4bOCr+Y+D+M/z0sBXA6/Fv9/vAO8N3Mp/LMR/vtcGPht4Lf71vgf4buC3+c+B+K9zHHhr4LWBtwaO8bz+Bvht4K+BnwZ2+c+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiH+brwZein+7lwaO85x2gb/m3+5vgI/mXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/zZfDbw0/7P8NfDR/Osg/n9D/P+G+P8N8f8b4v83/hHpLXdBjDFbvAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNature;
impl IconShape for MdNature {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 16.12c3.47-.41 6.17-3.36 6.17-6.95 0-3.87-3.13-7-7-7s-7 3.13-7 7c0 3.47 2.52 6.34 5.83 6.89V20H5v2h14v-2h-6v-3.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sz0YeBDwN8Au//EQ/3M8GHgr4K2B1+b52wX+Gvhp4GeAW/n3Qfz3ey/gs4EH86/318DnAD/Nvw3iv89rA98FPJh/v98GPgf4bf51EP89vgt4b/7jfTXwMbzoEP+1jgO/Bbw0/3l+GngfYJd/GeK/znHgt4CX5j/fXwOvA+zywiH+6/wV8NL81/lp4G144RD/Nb4LeG/+630O8Nm8YIj/fK8N/Bb/fV4H+G2eP8R/vqcDD+a/z28Dr8Pzh/jP9d7Ad/Hf73WA3+Z5If5zPR14MP/9fgZ4a54X4j/Pg4Gn8z/HCWCX54T4z/PRwFfxP8f7AN/Nc0L85/lt4LX4n+NngLfmOSFedO8FvDRX/DXwPbxw5n+W3wFem+eEeNH8FvDaPKffBl6HF8z8zyOeE+Jf9tnAZ/H8fQ7w2TyvBwNP538e8ZwQ/7K/Al6a5++vgZfheb028Fv8zyOeE+JfZl448bweDDyd/3nEc0L8y34beC2ev98BXpvnz/zPI54T4l/22sBv8fy9DvDbPH/mf5bfAV6b54R40bw38NXAMa64BHw08N28YL8NvBb/c3wP8N48J8S/zmtzxW/zL/to4Kv4n+N9gO/mOSH+87w08Ff8z3EC2OU5If5z/TbwWvz3+x3gtXleiBfss4CPBo7zwt0KfDfwOTyv9wa+i/9+rwP8Ns8L8fy9NfBT/Ou8DfDTPK9bgQfx3+d3gNfm+UM8f98NvBf/Ot8DvDfP67WB3+K/xyXgtYG/5vlDPH+/DbwW/zq/A7w2z99nA5/Ff733Ab6bFwzx/P028Fr86/wO8Nq8YH8NvBT/db4HeG9eOMTz99vAa/Gv8zvAa/OCHQd+G3gp/vN9D/De/MsQz99vA6/Fv87vAK/NC3cc+GngtfjP8z3Ae/OiQTx/vw28Fv86vwO8Ni+azwY+i/9Yl4DPBr6aFx3i+ftt4LX41/kd4LV50b008NXAa/Hv9zvAewO38q+DeP5+G3gt/nV+B3ht/vVeG/hs4LX41/se4LuB3+bfBvH8/TbwWvzr/A7w2vzbHQfeGnht4K2BYzyvvwF+G/hr4KeBXf59EM/fbwOvxb/O7wCvzX+M3wZei+f0O8Br8x8L8fz9NvBa/Ov8DvDa/Mf4beC1eE6/A7w2/7EQz99LA8e54rd44V6HK3aBv+Y/xm8Dr8Vz+h3gtfmPhfiXmRdO/Mf7beC1eE6/A7w2/7EQ/zLzwon/eL8NvBbP6XeA1+Y/FuJfZl448R/vt4HX4jn9DvDa/MdC/MvMCyf+4/028Fo8p98BXpv/WIh/mXnhxH+83wZei+f0O8Br8x8L8S8zL5z4j/fbwGvxnH4HeG3+YyH+ZeaFE//xfht4LZ7T7wCvzX8sxL/MvHC/zX+8lwaO85x2gb/mBfsb4KP510H8y8z/Dr8DvDb/Ooh/mfnf4XeA1+ZfB/EvM/87/A7w2vzrIP5l5n+H3wFem38dxL/st/nf4a+Bj+ZfB/H/G+L/N8T/b4j/3xD/v/GPg1WuQc5/QMsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNaturePeople;
impl IconShape for MdNaturePeople {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.17 9.17c0-3.87-3.13-7-7-7s-7 3.13-7 7c0 3.47 2.52 6.34 5.83 6.89V20H6v-3h1v-4c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v4h1v5h16v-2h-3v-3.88c3.47-.41 6.17-3.36 6.17-6.95zM4.5 11c.83 0 1.5-.67 1.5-1.5S5.33 8 4.5 8 3 8.67 3 9.5 3.67 11 4.5 11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/npYGvAt4G2OU/F+J/lpcGfgs4Dvw18DrALv95EP9zvDTwW8Bxnu2vgdcBdvnPgfif4aWB3wKO87z+GngZ/nMg/vu9NPBbwHGev/cBvpv/HIj/Xi8N/BZwnOfvfYDv5j8P4r/PSwO/BRzn+Xsf4Lv5z4X47/HSwG8Bx3n+3gf4bv7zIf7rvTTwW8Bxnr/3Ab6b/xqI/1ovDfwWcJzn732A7+a/DuK/zksDvwUc5/l7H+C7+a+F+K/x0sBvAcd5/t4H+G7+6yH+87008FvAcZ6/9wG+m/8eiP9cLw38FnCc5+99gO/mvw/iP89LA78FHOf5ex/gu/nvhfjP8dLAbwHHef7eB/hu/vsh/nO8NPDbwDGev/cBvpv/foj/PC8N/DZwjOfvfYDv5r8X4j/XSwO/DRzj+Xsf4Lv574P4z/fSwG8Dx3j+3gf4bv57IP5rvDTw28Axnr/3Ab6b/3qI/zovDfw2cIzn732A7+a/FuK/1ksDvw0c4/l7H+C7+a+D+K/30sBvA8d4/t4H+G7+ayD+e7w08NvAMZ6/9wG+m/98iP8+Lw38NnCM5+99gO/mPxfiv9dLA78NHOP5ex/gu/nPg/jv99LAbwPHeP7eB/hu/nMg/md4aeC3gWM8r78BXpr/HIj/OV4a+G3gGM/2N8BrA7v850D8z/LSwG8Dx4C/AV4b2OU/D+J/npcGvhp4a2CX/1yI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjL21EQf4E52gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNavigateBefore;
impl IconShape for MdNavigateBefore {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K9xHPgp4GOAv+Z/DsR/vuPAbwEvDewCrwP8Nf8zIP5zHQd+C3hpnm0XeB3gr/nvh/jP9VfAS/O8doHXAf6a/16I/1zvDXwXz98u8DrAX/PfB/Gf772B7+L52wVeB/hr/nsg/mu8N/BdPH+7wOsAf81/PcR/nfcGvovnbxd4HeCv+a+F+K/13sB38fztAq8D/DX/dRD/9d4b+C6ev13gdYC/5r8G4r/HewPfxfO3C7wO8Nf850P893lv4Lt4/naB1wH+mv9ciP9e7w18F8/fLvA6wF/znwfx3++9ge/i+dsFXgf4a/5zIP5neG/gu3j+doHXAf6a/3iI/zneG/gunr9d4HWAv+Y/FuJ/jvcGvovn7xLw2sBf8x8L8T/DewPfxfN3CXht4K/5j4f47/fewHfx/F0CXhv4a/5zIP57vTfwXTx/l4DXBv6a/zyI/z7vDXwXz98l4LWBv+Y/F+K/x3sD38Xzdwl4beCv+c+H+K/33sB38fxdAl4b+Gv+ayD+a7038F08f5eA1wb+mv86iP867w18F8/fJeC1gb/mvxbiv8Z7A9/F83cJeG3gr/mvh/jP997Ad/H8XQJeG/hr/nsg/nO9N/BdPH+XgNcG/pr/Poj/XH8NvBTP6xLw2sBf898L8Z/rOPDbwEvxbJeA1wb+mv9+iP98x4HfBl4KuAS8NvDX/M+A+K9xHPhp4KOBv+Z/DsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHtWl5Boy4ybgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNavigateNext;
impl IconShape for MdNavigateNext {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3EceCvgwcBrAw8GHsxzuhW4FbgV+Gvgd4C/5j8X4j/PceC9gPcGXpp/m13gp4HvAX6b/3iI/3gPBj4KeG/gOP9xbgU+G/ge/uMg/mN9FvDRwHH+89wKfAzw0/z7If5jvDTwXcBL81/np4H3AXb5t0P8+7038FXAcZ7X3wA/DdwKvDTw3sAxXrhLwHcDfw08GHhv4EE8f7cCbwP8Nf82iH+f9wa+i+fve4D35jk9GPhp4KV4/v4GeGvgVp7TdwPvxfO3C7wO8Nf86yH+7d4b+C6ev78BXprn76WBv+L5exngr3n+bgUexPO3C7wN8Nv86yD+bd4b+C5esM8BPpsX7K+Bl+I5/Q3w0rxgnw18Fi/YLvA6wF/zokP867008FvAcV6w9wG+mxfst4HX4jn9DvDavGAfDXwVL9wu8BBglxcN4l/vr4CX5oX7GuCjecEuAsd5TrcCD+EF+2rgo/iX/TTwNrxoEP86nw18Fv+yXeBlgFt5Xh8NfBXP38cAX83zejDwV8BxXjSvA/w2/zLEi+7BwF8Bx3nR/DXwPsBf82wfBXw2cJznbxf4bOBreLaXBr4LeGledH8NvAz/MsSL7quBj+Jf76+BXeClgeO8aHaBvwaOAy/Nv83bAD/NC4d40RwHng4c53+PnwHemhcO8aL5aOCr+N/nBLDLC4Z40fwV8NI8r88Bfhr4a+C1gc8GXov/XL8DfDbw28BLA28NfBbP3/sA380LhviXHQcu8rxeB/htntd3A+/Ff47vAd6b5/XawG/xvL4G+GheMMS/7L2B7+I5fQ/w3jx/x4GL/Oc4Aezy/H038F48p98BXpsXDPEv+2rgo3hOHwN8NS/YbwOvxX+s3wFemxfso4Gv4jntAid4wRD/st8GXovn9DHAV/OC/TbwWvzH+h3gtXnBPhr4Kp6XeMEQ/7KnAw/mOX0P8N48f8eBi/znOAHs8vx9N/BePC/xgiH+Zeb5ex3gt3le3wW8N/85vht4H57XawO/xfN6BvBgXjDEv8y8YJ8N/Azw18BrAZ8NvDb/uX4b+Gzgd4CXBt4K+Gyev98BXpsXDPEvM/97fQ3w0bxgiH+Z+d/rbYCf5gVD/MtuBR7E/z6XgOO8cIh/2W8Dr8X/Pu8DfDcvHOJf9tXAR/G/yzOAB/MvQ/zL3hv4Lv73uAS8NvDX/MsQ/7LjwEX+93gb4Kd50SBeNH8NvBT/s10C3hr4bV50iBfNRwNfxf9cvwO8N3Ar/zqIF81x4FbgGP+z/Azw1cBv82+DeNF9NfBR/Nf4HZ6/3+aKW4GfBnb590G86B4M/DVwjP88XwN8NrDLfw3Ev85nA5/Ff46fAd6a/1qIf72/Bl6K/3gvA/w1/7UQ/3oPBv4aOMZ/LPFfD/Fv89bAT/Ef6yHArfzXQvzbvTfwXfzH+Rzgs/mvhfj3eW/gu/iPsQu8DHAr/3UQ/35vDXw3cIx/v+8B3pv/Ooj/GA8Gfhp4Kf59doET/NdB/Mf6bOCjgWP82zwDeDD/dRD/8R4MfDTw3sAx/nV+Bnhr/usg/vMcB94beG/gpfiXXQJeGriV/zqI/xrHgdcGXhp4beDBwIO44hnAXwMfDdzKfy3E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGEb8VBxatTAgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPalette;
impl IconShape for MdPalette {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXsw8CD+b3gGcCvPC/G8jgPfBbw1/7f8NPA+wC7PhnhOx4GnA8f5v2kXeAiwyxWI5/RTwFvzf9tPA2/DFYhnezDwdP5/eAhwK4B4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/if46fAd6K/xyvA/w2gHi21wZ+i/8Zfgd4beCjga/iP97rAL8NIJ7ttYHf4r/fJeDBwC5X/DTwVvzHeh3gtwHEs7028Fv893sZ4K95tuPAXwMP4j/O6wC/DSCe7bWB3+I/zs8Ab8W/zscAX83zemngr/iP8zrAbwOIZ3tt4Lf4j/E7wGsD7w18Fy+a7wHemxfso4Gv4j/G6wC/DSCe7bWB3+Lf7xLwYGCXK74a+CheuL8BXhvY5YX7aeCt+Pd7HeC3AcSzvTbwW/z7vQzw1zynnwbeiufvEvDawF/zLzsO/DXwIP59Xgf4bQDxbK8N/Bb/Ph8DfDXP6zjw28BL8bzeBvhpXnQvDfwV/z6vA/w2gHi21wZ+i3+77wHemxfswcBfA8d4tq8BPpp/vY8Gvop/u9cBfhtAPNtrA7/Fv83fAK8N7PLCvTTwV1zxO8Br82/308Bb8W/zOsBvA4hne23gt/jXuwS8NvDXvGjeG/hq4MHALv92x4G/Bh7Ev97rAL8NIJ7ttYHf4l/vbYCf5l/nOLDLv99LA3/Fv97rAL8NIJ7ttYHf4l/na4CP5r/XRwNfxb/O6wC/DSCe7bWB3+JF9zvAa/M/w08Db8WL7nWA3wYQz/bawG/xorkEPBjY5X+G48BfAw/iRfM6wG8DiGd7beC3eNHcCtzK/ywPBh7Mi+Z1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi2BwNP5/+HhwC3Aojn9NPAW/F/288Ab80ViOd0HLgVOMb/TX8DvDawyxWI53Uc+G7grfi/5WeA9wZ2eTbEC/Zg4MH833ArcCvPC/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EVv3lEGDYu6VAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanorama;
impl IconShape for MdPanorama {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 18V6c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2zM8.5 12.5l2.5 3.01L14.5 11l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaFishEye;
impl IconShape for MdPanoramaFishEye {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaHorizontal;
impl IconShape for MdPanoramaHorizontal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6.54v10.91c-2.6-.77-5.28-1.16-8-1.16-2.72 0-5.4.39-8 1.16V6.54c2.6.77 5.28 1.16 8 1.16 2.72.01 5.4-.38 8-1.16M21.43 4c-.1 0-.2.02-.31.06C18.18 5.16 15.09 5.7 12 5.7c-3.09 0-6.18-.55-9.12-1.64-.11-.04-.22-.06-.31-.06-.34 0-.57.23-.57.63v14.75c0 .39.23.62.57.62.1 0 .2-.02.31-.06 2.94-1.1 6.03-1.64 9.12-1.64 3.09 0 6.18.55 9.12 1.64.11.04.21.06.31.06.33 0 .57-.23.57-.63V4.63c0-.4-.24-.63-.57-.63z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAER0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXst4BLw1/zv9tLAMeB3eF6I5++jga/iiluB3wZ+GvgZ/nd4K+CtgdcGHswVHwN8Nc8J8fz9NvBaPK9d4KeBnwZ+B9jlf4bjwGsBbw28NXCc5/U7wGvznBDP328Dr8W/7K+BvwZ+G/gb4K/5r/HSwEsBrw28NPDS/Mt+B3htnhPi+ftt4LX419sF/hrYBf4auBW4FXgGcCv/Og8GHgQ8GHgw8NLAceClgeP86/0O8No8J8Tz99vAa/Gf66+BXa44Drw0/7l+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfjP9TfALlccB16K/1y/A7w2zwnx/P028Fr8610C/hq4FbgVuBW4FbgVuJV/nQcDDwYeDDwYeGngOPDSwDH+9X4HeG2eE+L5+23gtfiX/Q3w18BvA78N3Mp/jZcGXhp4beClgZfiX/Y7wGvznBDP328Dr8XzugT8NPDTwG8Du/zPcBx4beCtgbcGjvG8fgd4bZ4T4vn7aOCruOIZwE8Dvw38NP87vDXw1sBrAw/iio8BvprnhHjBXhvYBf6a/91eGjgO/DbPC/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ETBgikFEfSPYAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaHorizontalSelect;
impl IconShape for MdPanoramaHorizontalSelect {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.43 4c-.1 0-.2.02-.31.06C18.18 5.16 15.09 5.7 12 5.7s-6.18-.55-9.12-1.64C2.77 4.02 2.66 4 2.57 4c-.34 0-.57.23-.57.63v14.75c0 .39.23.62.57.62.1 0 .2-.02.31-.06 2.94-1.1 6.03-1.64 9.12-1.64s6.18.55 9.12 1.64c.11.04.21.06.31.06.33 0 .57-.23.57-.63V4.63c0-.4-.24-.63-.57-.63z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaPhotosphere;
impl IconShape for MdPanoramaPhotosphere {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.4 11.32v2.93c-.1.05-2.17.85-3.33 1.17-.94.26-3.84.73-6.07.73-3.7 0-7-.7-9.16-1.8-.08-.04-.16-.06-.24-.1V9.76c6.02-2.84 12.6-2.92 18.8 0v1.56zm-9.39 8.88c-2.5 0-4.87-1.15-6.41-3.12 4.19 1.22 8.57 1.23 12.82-.01-1.54 1.97-3.9 3.13-6.41 3.13zM12 3.8c2.6 0 4.91 1.23 6.41 3.12-4.1-1.19-8.48-1.26-12.83.01C7.08 5.03 9.4 3.8 12 3.8zm10.49 4.71c-.47-.23-.93-.44-1.4-.64C19.52 4.41 16.05 2 12 2S4.47 4.41 2.9 7.88c-.47.2-.93.41-1.4.63-.31.15-.5.48-.5.83v5.32c0 .35.19.68.51.83.47.23.93.44 1.39.64 3.55 7.83 14.65 7.82 18.2 0 .47-.2.93-.41 1.39-.63.31-.17.51-.49.51-.84V9.34c0-.35-.19-.68-.51-.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaPhotosphereSelect;
impl IconShape for MdPanoramaPhotosphereSelect {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.49 8.51c-.47-.23-.93-.44-1.4-.64C19.52 4.41 16.05 2 12 2S4.47 4.41 2.9 7.88c-.47.2-.93.41-1.4.63-.31.15-.5.48-.5.83v5.32c0 .35.19.68.51.83.47.23.93.44 1.39.64 3.55 7.83 14.65 7.82 18.2 0 .47-.2.93-.41 1.39-.63.31-.17.51-.49.51-.84V9.34c0-.35-.19-.68-.51-.83zM12 3.8c2.6 0 4.91 1.23 6.41 3.12-4.1-1.19-8.48-1.26-12.83.01C7.08 5.03 9.4 3.8 12 3.8zM5.6 17.08c4.19 1.22 8.57 1.23 12.82-.01-1.54 1.97-3.9 3.13-6.41 3.13-2.5 0-4.87-1.15-6.41-3.12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Oh8FvDX/s/008DW8aBAvutcGfov/HV4H+G3+ZYgX3UsDf8X/Di8D/DX/MsS/zq3Ag/if7RnAg3nRIP51vht4L/5n+xrgo3nRIP513hr4KZ7XM4Bb+a/1YOBBPK+3AX6aFw3iX28XOMZz+h7gvfmv9dPAW/GcLgHHedEh/vW+G3gvntOtwEP4r3UROM5z+h7gvXnRIf713hr4KZ7XywB/zX+Nlwb+iuf1NsBP86JD/OsdBy7yvD4G+Gr+a3w08FU8rxPALi86xL/NXwMvxXP6GeCt+a/x08Bb8Zz+Bnhp/nUQ/zbfDbwXz+lW4CH817gIHOc5fQ/w3vzrIP5t3hv4Lp7XCWCX/1wPBp7O83of4Lv510H82zwYeDrP63WA3+Y/12sDv8Xzehngr/nXQfzb7QLHeE4fA3w1/7k+G/gsntMl4Dj/eoh/u98GXovn9D3Ae/Of66eBt+I5/Q7w2vzrIf7tvht4L57T7wCvzX+u3wZei+f0M8Bb86+H+Lf7bOCzeE67wAn+c5nn9TnAZ/Ovh/i3e2/gu3he4j+XeV7vA3w3/3qIf7vXBn6L53UC2OU/x0sDf8Xzeh3gt/nXQ/zbHQcu8rxeB/ht/nO8NvBbPK+HALfyr4f49zHP63WA3+Y/x1sDP8XzEv82iH8f87zeBvhp/nN8NvBZPC/xb4P49zHP63OAz+Y/x2cDn8XzEv82iH+fXeAYz+lzgM/mP8dnA5/F8xL/Noh/n98GXovn9DnAZ/Of47OBz+I5/Q3w0vzbIP59fht4LZ7T5wCfzX+OzwY+i+f0O8Br82+D+Pf5beC1eE6fA3w2/zk+G/gsntPvAK/Nvw3i3+e3gdfiOX0O8Nn85/hs4LN4Tn8NvAz/Noh/n4vAcZ7T5wCfzX+OzwY+i+cl/m0Q/z7meX0O8Nn85/hs4LN4XuLfBvHvY57X2wA/zX+OzwY+i+cl/m0Q/z7meb0O8Nv853hr4Kd4XuLfBvFvdxy4yPN6HeC3+c/x2sBv8bxOALv86yH+7V4b+C2e1wlgl/8cLw38Fc/rdYDf5l8P8W/33sB38bzEfy7zvN4H+G7+9RD/dp8NfBbP6RJwnP9c5nl9DvDZ/Osh/u2+G3gvntPvAK/Nf67fBl6L5/Q9wHvzr4f4t/tt4LV4Tt8DvDf/uX4aeCue0+8Ar82/HuLf7iJwnOf0McBX85/rs4HP4jntAif410P82zwYeDrP63WA3+Y/12sDv8XzeghwK/86iH+b9wa+i+d1AtjlP9eDgafzvN4H+G7+dRD/Nt8NvBfP6RnAg/mvsQsc4zl9D/De/Osg/m3+CnhpntPPAG/Nf42fBt6K5/TXwMvwr4P41zsOXOR5fQzw1fzX+Gjgq3heJ4BdXnSIf723Bn6K5/UywF/zX+Olgb/ieb0N8NO86BD/et8NvBfP6RnAg/mvtQsc4zl9D/DevOgQ/3oXgeM8p+8B3pv/Wj8NvBXPaRc4wYsO8a/z2sBv8bxuBW7lv9aDgQfzvN4G+GleNIh/na8GPor/2b4G+GheNIh/nacDD+Z/tluBh/CiQbzoXhr4K/53eBngr/mXIV50rw38Fv87vA7w2/zLEP86Hw28Nf+z/TTw1bxoEP+/If5/Q/z/hvj/DfH/G/8IjoS6Qd/IOFIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaVertical;
impl IconShape for MdPanoramaVertical {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.94 21.12c-1.1-2.94-1.64-6.03-1.64-9.12 0-3.09.55-6.18 1.64-9.12.04-.11.06-.22.06-.31 0-.34-.23-.57-.63-.57H4.63c-.4 0-.63.23-.63.57 0 .1.02.2.06.31C5.16 5.82 5.71 8.91 5.71 12c0 3.09-.55 6.18-1.64 9.12-.05.11-.07.22-.07.31 0 .33.23.57.63.57h14.75c.39 0 .63-.24.63-.57-.01-.1-.03-.2-.07-.31zM6.54 20c.77-2.6 1.16-5.28 1.16-8 0-2.72-.39-5.4-1.16-8h10.91c-.77 2.6-1.16 5.28-1.16 8 0 2.72.39 5.4 1.16 8H6.54z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADqklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Oh8FvDX/s/008DW8aBAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Di8D/DX/MsS/zi5wjP/ZngE8mBcN4l/nu4H34n+2rwE+mhcN4l/nrYGf4n+2lwH+mhcN4l9vFzjG/0zPAB7Miw7xr/fdwHvxP9P3AO/Niw7xr/fWwE/xP9PbAD/Niw7xr3ccuMj/TOJfB/Fv89fAS/E/y98AL82/DuLf5ruB9+J/lq8BPpp/HcS/zXsD38X/LO8DfDf/Ooh/m5cG/or/WR4C3Mq/DuLfbhc4xv8Ml4Dj/Osh/u1+G3gt/mf4HeC1+ddD/Nv9NPBW/M/wPcB786+H+Lf7bOCz+J/hc4DP5l8P8W/33sB38T/D2wA/zb8e4t/utYHf4n+G1wF+m389xL/dg4Gn8z/DCWCXfz3Ev4/5n0H82yD+fcz/DOLfBvHvY/5nEP82iH8f89/vGcCD+bdB/PvcCjyI/16/A7w2/zaIf5/fBl6L/16/A7w2/zaIf5/fBl6L/16/A7w2/zaIf5+nAw/mv9fvAK/Nvw3i38f897sVeAj/Noh/H/M/g/i3Qfz7mP8ZxL8N4t/H/M8g/m0Q/3YPBp7O/wwngF3+9RD/dq8N/Bb/M7wO8Nv86yH+7d4b+C7+Z3gb4Kf510P823028Fn8z/A5wGfzr4f4t/tp4K34n+F7gPfmXw/xb/fbwGvxP8PvAK/Nvx7i3+4icJz/GW4FHsK/HuLf5qWBv+J/locAt/Kvg/i3eW/gu/if5X2A7+ZfB/Fv893Ae/E/y9cAH82/DuLf5unAg/mf5a+Bl+FfB/Gvdxy4yP9M4l8H8a/31sBP8T/T2wA/zYsO8a/33cB78T/T9wDvzYsO8a93ETjO/0y3Ag/hRYf413lr4Kf4n+1lgL/mRYP41/lu4L34n+1rgI/mRYP417kIHOd/tluBh/CiQbzoXhr4K/53eBngr/mXIV50rw38Fv87vA7w2/zLEP86Hw28Nf+z/TTw1bxoEP+/If5/Q/z/hvj/DfH/G/8IGNJkQUqGsXYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaVerticalSelect;
impl IconShape for MdPanoramaVerticalSelect {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.93 21.12c-1.1-2.94-1.64-6.03-1.64-9.12s.55-6.18 1.64-9.12c.05-.11.07-.22.07-.31 0-.34-.24-.57-.64-.57H4.62c-.4 0-.63.23-.63.57 0 .1.02.2.06.31C5.16 5.82 5.7 8.91 5.7 12s-.55 6.18-1.64 9.12c-.05.11-.07.22-.07.31 0 .33.23.57.63.57h14.75c.39 0 .63-.24.63-.57 0-.1-.02-.2-.07-.31z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFNElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8ZLA8f417kE/DX/uRD/Pi8NHANeGjgOPBh4MHAceGn+Y/01sAvcCtwK7AJ/DTwDuJV/G8SL5rWABwMPBl4aeDDw0vzP8tfArcBfA38NPAP4a144xL/sr4CX5n+nvwZehhcM8cI9GHg6/7udAHZ5/hAv3HsD38X/bu8DfDfPH+KF+27gvXjR/A2wC9wK3MoVv82z7QJ/zYvmpYHjPNtrc8WDgQcDx4GX4kXzPcB78/whXrinAw/mOT0D+G5gF/hr4K+BXf57PBh4MPDSwHHgvYEH8ZxuBR7C84d4wR4MPJ3n9T7Ad/M/03sD38XzeghwK88L8YK9N/BdPK+HALfyP9ODgafzvN4G+GmeF+IF+2zgs3hOl4Dj/M+2CxzjOX0O8Nk8L8QL9tvAa/Gcfgd4bf5n+23gtXhOvwO8Ns8L8YI9HXgwz+lzgM/mf7bPBj6L53Qr8BCeF+IFM8/rfYDv5n+29wa+i+clnhfi+Xtt4Ld4Xq8D/Db/s7028Fs8r5cB/prnhHj+Xhv4LZ6X+N/BPK/XAX6b54R4/t4b+C6el/jfwTyv9wG+m+eEeP4+G/gsntMzgAfzv8OtwIN4Tp8DfDbPCfH8fTbwWTyn3wFem/8dfht4LZ7T5wCfzXNCPH+fDXwWz+l3gNfmf4ffBl6L5/Q5wGfznBDP31cDH8Vz+h3gtfnf4beB1+I5fQ7w2TwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/Xw18FM/pd4DX5n+H3wZei+f0OcBn85wQz99nA5/Fc/od4LX53+G3gdfiOX0O8Nk8J8Tz99nAZ/Gcfgd4bf53+G3gtXhOnwN8Ns8J8fx9NvBZPKdbgYfwv8PTgQfznD4H+GyeE+L5e2/gu3he4n8H87zeB/hunhPi+Xtt4Ld4XuJ/B/O8Xgf4bZ4T4vl7beC3eF6vA/w2/7O9NvBbPK+XAf6a54R4wczzeh/gu/mf7b2B7+J5ieeFeMFuBR7Ec/oc4LP5n+2zgc/iOT0DeDDPC/GC/TbwWjyn3wFem//Zfht4LZ7T7wCvzfNCvGCfDXwWz2kXOMH/bBeB4zynzwE+m+eFeMHeG/guntdDgFv5n+nBwNN5Xm8D/DTPC/GCPRh4Os/rfYDv5n+m9wa+i+f1EOBWnhfihbsVeBDP6Vbgu4Fd4K+BZwC38t/jwcCDgJcGjgPvDTyY5/QM4ME8f4gX7ruB9+JF89fALnArcCtX/DbPdgn4a140r8Vzem2ueDDwYOA48NK8aL4HeG+eP8QL997Ad/G/2/sA383zh3jhHgw8nf/dHgLcyvOH+Jf9NfBS/O/0N8BL84IhXjQvDTwYeGngpYHjwGvxP8vfALcCfw3cCvw18Ne8cIh/nwcDDwZeGjgOPBh4MHAceCn+Y/0NsAvcCtwK7AJ/DewCf82/DeK/xoOBB/OvcytwK/+5EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAR7EuEHCYQYqAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaWideAngle;
impl IconShape for MdPanoramaWideAngle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 6c2.45 0 4.71.2 7.29.64.47 1.78.71 3.58.71 5.36 0 1.78-.24 3.58-.71 5.36-2.58.44-4.84.64-7.29.64s-4.71-.2-7.29-.64C4.24 15.58 4 13.78 4 12c0-1.78.24-3.58.71-5.36C7.29 6.2 9.55 6 12 6m0-2c-2.73 0-5.22.24-7.95.72l-.93.16-.25.9C2.29 7.85 2 9.93 2 12s.29 4.15.87 6.22l.25.89.93.16c2.73.49 5.22.73 7.95.73s5.22-.24 7.95-.72l.93-.16.25-.89c.58-2.08.87-4.16.87-6.23s-.29-4.15-.87-6.22l-.25-.89-.93-.16C17.22 4.24 14.73 4 12 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8ZLA8f417kE/DX/uRD/Pi8NHANeGjgOPBh4MHAceGn+Y/01sAvcCtwK7AJ/DTwDuJV/G8SL5rWABwMPBl4aeDDw0vzP8tfArcBfA38NPAP4a144xL/sr4CX5n+nvwZehhcM8cI9GHg6/7udAHZ5/hAv3HsD38X/bu8DfDfPH+KF+27gvfjf7XuA9+b5Q7xwTwcezP9utwIP4flDvGAPBp7O/w0PAW7leSFesPcGvov/G94G+GmeF+IF+2zgs/i/4XOAz+Z5IV6w3wZei/8bfgd4bZ4X4gV7OvBg/m+4FXgIzwvxgpn/W8TzQjx/rw38Fv+3vAzw1zwnxPP32sBv8X/L6wC/zXNCPH/vDXwX/7e8D/DdPCfE8/fZwGfxf8vnAJ/Nc0I8f58NfBb/t3wO8Nk8J8Tz99nAZ/F/y+cAn81zQjx/Xw18FP+3fA7w2TwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fVwEfxf8vnAJ/Nc0I8f58NfBb/t3wO8Nk8J8Tz99nAZ/F/y+cAn81zQjx/nw18Fv+3fA7w2TwnxPP33sB38X/L+wDfzXNCPH+vDfwW/7e8DvDbPCfE8/fawG/xf8vLAH/Nc0K8YOb/FvG8EC/YrcCD+L/hGcCDeV6IF+y3gdfi/4bfAV6b54V4wT4b+Cz+b/gc4LN5XogX7L2B7+L/hrcBfprnhXjBHgw8nf8bHgLcyvNCvHC3Ag/if7dnAA/m+UO8cN8NvBf/u30P8N48f4gX7r2B7+J/t/cBvpvnD/HCPRh4Ov+7PQS4lecP8S/7a+Cl+N/pb4CX5gVDvGheGngw8NLASwPHgdfif5a/AW4F/hq4Ffhr4K954RD/Pg8GHgy8NHAceDDwYOA48FL8x/obYBe4FbgV2AX+GtgF/pp/G8R/jQcDD+Zf51bgVv5zIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AkiUakGazMmnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanoramaWideAngleSelect;
impl IconShape for MdPanoramaWideAngleSelect {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 4c-2.73 0-5.22.24-7.95.72l-.93.16-.25.9C2.29 7.85 2 9.93 2 12s.29 4.15.87 6.22l.25.89.93.16c2.73.49 5.22.73 7.95.73s5.22-.24 7.95-.72l.93-.16.25-.89c.58-2.08.87-4.16.87-6.23s-.29-4.15-.87-6.22l-.25-.89-.93-.16C17.22 4.24 14.73 4 12 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z893gb4KuBB/HvI14wxL/M/Nf7HOCzgZcG/op/H/GCIf5l5r/W7wCvzbN9NPBV/NuJFwzxLzP/dZ4BvDSwy3P6aeCt+LcRLxjiX2b+bd4G+GzgpXjRvQzw1zyv48BfAw/iX0+8YIh/mfnX+xzgs4GXBn4bOMa/7H2A7+YFe2ngr/jXEy8Y4l9m/nV+Bnhrnu21gd/ihfse4L35l3008FX864gXDPEvMy+6ZwAvDezynN4b+C6ev78BXhvY5UXz08Bb8aITLxjiX2ZeNJeA1wb+mufvu4H34jldAl4auJUX3XHgr4EH8aIRLxjiX2ZeNO8DfDcv3G8Dr8WzvQ3w0/zrvTTwV7xoxAuG+JeZf9n3AO/Nv+w48NvASwGfA3w2/3YfDXwV/zLxgiH+ZeaF+xvgpXnRvTTw2cBb8+/308Bb8cKJFwzxLzMv2CXgpYFb+e9xHPhr4EG8YOIFQ/zLzAv2OsBv89/rpYG/4gUTLxjiX2aev88BPpv/GT4a+CqeP/GCIf5l5nn9DPDW/M/y08Bb8bzEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjyDdxQRvlagYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoto;
impl IconShape for MdPhoto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoAlbum;
impl IconShape for MdPhotoAlbum {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4zm0 15l3-3.86 2.14 2.58 3-3.86L18 19H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoCamera;
impl IconShape for MdPhotoCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "3.2",
            }
            path {
                d: "M9 2L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2h-3.17L15 2H9zm3 15c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoCameraBack;
impl IconShape for MdPhotoCameraBack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 5c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V7c0-1.1.9-2 2-2h3.17L9 3h6l1.83 2H20zm0 14V7H4v12h16zm-6-7l-3 3.72L9 13l-3 4h12l-4-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoCameraFront;
impl IconShape for MdPhotoCameraFront {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 10.48l4-3.98v11l-4-3.98V18c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2h12c1.1 0 2 .9 2 2v4.48zm-2-.79V6H4v12h12V9.69zM10 12c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm0 1c1.34 0 4 .67 4 2v1H6v-1c0-1.33 2.66-2 4-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoFilter;
impl IconShape for MdPhotoFilter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.02 10v9H5V5h9V3H5.02c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-9h-2zM17 10l.94-2.06L20 7l-2.06-.94L17 4l-.94 2.06L14 7l2.06.94zm-3.75.75L12 8l-1.25 2.75L8 12l2.75 1.25L12 16l1.25-2.75L16 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoLibrary;
impl IconShape for MdPhotoLibrary {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 16V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2zm-11-4l2.03 2.71L16 11l4 5H8l3-4zM2 6v14c0 1.1.9 2 2 2h14v-2H4V6H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEhElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/l4b+Cjgrfm/4aeBrwF+m+eEeF4fDXwV/ze9D/DdPBviOb018FP83/Y2wE9zBeI5XQSO83/brcBDuALxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi21wZ+i/8fXgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi21wZ+i/8fXgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/uf4GeCt+M/xOsBvA4hne23gt/if4XeA1wY+Gvgq/uO9DvDbAOLZXhv4Lf77XQIeDOxyxU8Db8V/rNcBfhtAPNtrA7/Ff7+XAf6aZzsO/DXwIP7jvA7w2wDi2V4b+C3+4/wM8Fb863wM8NU8r5cG/or/OK8D/DaAeLbXBn6L/xi/A7w28N7Ad/Gi+R7gvXnBPhr4Kv5jvA7w2wDi2V4b+C3+/S4BDwZ2ueKrgY/ihfsb4LWBXV64nwbein+/1wF+G0A822sDv8W/38sAf81z+mngrXj+LgGvDfw1/7LjwF8DD+Lf53WA3wYQz/bawG/x7/MxwFfzvI4Dvw28FM/rbYCf5kX30sBf8e/zOsBvA4hne23gt/i3+x7gvXnBHgz8NXCMZ/sa4KP51/to4Kv4t3sd4LcBxLO9NvBb/Nv8DfDawC4v3EsDf8UVvwO8Nv92Pw28Ff82rwP8NoB4ttcGfot/vUvAawN/zYvmvYGvBh4M7PJvdxz4a+BB/Ou9DvDbAOLZXhv4Lf713gb4af51jgO7/Pu9NPBX/Ou9DvDbAOLZXhv4Lf51vgb4aP57fTTwVfzrvA7w2wDi2V4b+C1edL8DvDb/M/w08Fa86F4H+G0A8WyvDfwWL5pLwIOBXf5nOA78NfAgXjSvA/w2gHi21wZ+ixfNrcCt/M/yYODBvGheB/htAPFsrw38Fv8/vA7w2wDi2V4b+C3+f3gd4LcBxLO9NvBb/P/wOsBvA4hne23gt/j/4XWA3wYQz/bawG/x/8PrAL8NIJ7ttYHf4v+H1wF+G0A8p13gGP+3PQN4MFcgntNnA5/F/21vA/w0VyCe13cD78X/Te8DfDfPhnj+3hv4aOCl+L/hZ4CvBn6b54T4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIm5JpB295hawAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoSizeSelectActual;
impl IconShape for MdPhotoSizeSelectActual {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3C2 3 1 4 1 5v14c0 1.1.9 2 2 2h18c1 0 2-1 2-2V5c0-1-1-2-2-2zM5 17l3.5-4.5 2.5 3.01L14.5 11l4.5 6H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/l4b+CjgrQHxvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMvDTwNcAv81zQjyvjwa+imcTz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/Ns7wN8N8+GeE5vDfwUz0k8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zHN6G+CnuQLxnC4Cx3lO4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We063AQ7gC8WyvDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3AcSzHQdemuf12zyvlwaO85x2gb/meb02z+uvgV2e03HgpXlev83zemngOM9pF/hrntdr87z+GtgFEP+/If5/Q/z/hvj/DfFsx4GX4nn9Dv+7vRbP62+AXQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4v83xP9viP/fEP+/If5/QzzbSwNfxf88r8O/z2/xvD4G+GsA8WyvDfwW//OIfx/zvF4H+G0A8WyvDfwW//OIfx/zvF4H+G0A8WyvDfwW//OIfx/zvF4H+G0A8WyvDfwW//OIfx/zvF4H+G0A8WyvDfwW//OIf5/f5nl9NPDXAOLZXhv4Lf7nEf95EM/22sBv8T+P+M+DeLbXBn6L/3nEfx7Es7028Fv8zyP+8yCe7bWB3+I/x98AtwJvxb+e+M+DeLbXBn6L/3iXgNcGbgX+GngQ/zri38c8r9cBfhtAPNtrA7/Ff7z3Ab6bK14a+Cv+dcS/j3lerwP8NoB4ttcGfosX3ccAnw0c4wX7GuCjeU4fDXwVLzrx72Oe1+sAvw0gnu21gd/iRfM1wEcDLw38NnCM5/U3wEvz/P008Fa8aMS/j3lerwP8NoB4ttcGfot/2d8AL82zvTfwXTynS8BLA7fy/B0H/hp4EP8y8e9jntfrAL8NIJ7ttYHf4oW7BDwY2OU5fTbwWTzb6wC/zQv30sBf8S8T/z6vzfP6a2AXQDzbawO/xQv3OsBv8/x9N/BewOcAn82L5qOBr+KFE/95EM/22sBv8YJ9DPDVvGDHgY8GPpt/nZ8G3ooXTPznQTzbawO/xfP3M8Bb85/jOPDXwIN4/sR/HsSzvTbwWzyvvwFeG9jlP89LA3/F8yf+8yCe7bWB3+J5fTTw1/zne2/gvXle4nm9NHCM53QJ+Gue12vxvP4G2AUQz/bawG/xP494Xr8NvBbP6XeA1+Z5mef1OsBvA4jntAsc438W8bx+G3gtntPvAK/N8zLP6RnAg7kC8Zw+G/gs/mcRz+u3gdfiOf0O8No8L/Oc3gb4aa5APK/vBt6L/znE8/pt4LV4Tr8DvDbPyzzb+wDfzbMhnr/3Bj4aeCn++4nn9dvAa/Gcfgd4bZ6XgZ8Bvhr4bZ4T4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8ISG3BQda6k6gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoSizeSelectLarge;
impl IconShape for MdPhotoSizeSelectLarge {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 15h2v2h-2v-2zm0-4h2v2h-2v-2zm2 8h-2v2c1 0 2-1 2-2zM13 3h2v2h-2V3zm8 4h2v2h-2V7zm0-4v2h2c0-1-1-2-2-2zM1 7h2v2H1V7zm16-4h2v2h-2V3zm0 16h2v2h-2v-2zM3 3C2 3 1 4 1 5h2V3zm6 0h2v2H9V3zM5 3h2v2H5V3zm-4 8v8c0 1.1.9 2 2 2h12V11H1zm2 8l2.5-3.21 1.79 2.15 2.5-3.22L13 19H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/l4b+CjgrQHxvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMvDTwNcAv81zQjyvjwa+imcTz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/Ns7wN8N8+GeE5vDfwUz0k8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zHN6G+CnuQLxnC4Cx3lO4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We063AQ7gC8WyvDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3AcSzHQdemuf12zyvlwaO85x2gb/meb02z+uvgV2e03HgpXlev83zemngOM9pF/hrntdr87z+GtgFEP+/If5/Q/z/hvj/DfFsx4GX4nn9Dv+7vRbP62+AXQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4v83xP9viP/fEP+/If5/QzzbSwNfxfN6Hf53+y2e18cAfw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhr4ap7Xa/O/22/zvD4a+GsA8f8b4v83xP9viP/fEP+/IZ7ttYHf4r/G6wC/zX8N87xeB/htAPFsrw38Fv81Xgf4bf5rmOf1OsBvA4hne23gt/iv8TrAb/Nfwzyv1wF+G0A822sDv8V/jdcBfpv/GuZ5vQ7w2wDi2V4b+C3+a7wO8Nv81zDP63WA3wYQz/bawG/xX+N1gN/mv8Zr87z+GtgFEM/22sBv8V/jdYDf5r8f4tleG/gt/mu8DvDb/PdDPNtrA7/Ff43XAX6b/36IZ3tt4Lf4r/E6wG/z3w/xbK8N/Bb/NV4H+G2e03HgpXhev8PzemngGM/pEvDXPK/X4nn9DbALIJ7ttYHf4r/G6wC/zXN6beC3eF7ief028Fo8p98BXpvnZZ7X6wC/DSCe0y5wjP98rwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lOzwAezBWI5/TZwGfxn+91gN/mOb028Fs8L/G8fht4LZ7T7wCvzfMyz+ltgJ/mCsTz+m7gvfjP9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZlnex/gu3k2xPP33sBHAy/Ff47XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LwM8AXw38Ns8J8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EdiysQZErfukAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhotoSizeSelectSmall;
impl IconShape for MdPhotoSizeSelectSmall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 15h-2v2h2v-2zm0-4h-2v2h2v-2zm0 8h-2v2c1 0 2-1 2-2zM15 3h-2v2h2V3zm8 4h-2v2h2V7zm-2-4v2h2c0-1-1-2-2-2zM3 21h8v-6H1v4c0 1.1.9 2 2 2zM3 7H1v2h2V7zm12 12h-2v2h2v-2zm4-16h-2v2h2V3zm0 16h-2v2h2v-2zM3 3C2 3 1 4 1 5h2V3zm0 8H1v2h2v-2zm8-8H9v2h2V3zM7 3H5v2h2V3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPictureAsPdf;
impl IconShape for MdPictureAsPdf {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8.5 7.5c0 .83-.67 1.5-1.5 1.5H9v2H7.5V7H10c.83 0 1.5.67 1.5 1.5v1zm5 2c0 .83-.67 1.5-1.5 1.5h-2.5V7H15c.83 0 1.5.67 1.5 1.5v3zm4-3H19v1h1.5V11H19v2h-1.5V7h3v1.5zM9 9.5h1v-1H9v1zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm10 5.5h1v-3h-1v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFTUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/m3+fBwIO44hnArfz7vDbwWzwv8YIh/mXmeb0O8Nv827w08FXAa/Ocfhv4GOCv+bd5beC3eF7iBUP8y8zzeh3gt/nXe2vgp3jh3gb4af71Xhv4LZ6XeMEQ/zLzvF4H+G3+dR4M/BVwnBduF3gZ4Fb+dV4b+C2el3jBEP8y87xeB/ht/nW+GvgoXjRfA3w0/zqvDfwWz0u8YIh/mXlerwP8Nv86TwcezIvmVuAh/Ou8NvBbPC/xgiH+ZeZ5vQ7w2/zrmH8d8a/z2sBv8bzEC4b4l5nn9TrAb/OvY150l4Dj/Ou8NvBbPC/xgiH+ZeZ5vQ7w2/zr/DTwVrxofgZ4a/51Xhv4LZ6XeMEQ/zLzvF4H+G3+dV4b+C1eNK8D/Db/Oq8N/BbPS7xgiH+ZeV6vA/w2/3rfDbwXL9z3AO/Nv95rA7/F8xIvGOJfZp7X6wC/zb/NZwOfxfP3OcBn82/z2sBv8bzEC4b4l5nn9TrAb/Nv92DgrYGX5oq/Bn4auJV/u9cGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpt/vePASwGvzRUvzRV/zRW/DfwNsMu/3msDv8XzEi8Y4l9mntfrAL/Ni+bBwEcBrw28NC+avwZ+G/ga4FZeNK8N/BbPS7xgiH+ZeV6vA/w2L9xLA58FvDX/Pj8NfA7w17xwrw38Fs9LvGCIf5l5Xq8D/DYv2HsD38V/rPcBvpsX7LWB3+J5iRcM8S8zz+t1gN/m+Xtr4Kf4z/E2wE/z/L028Fs8L/GCIf5l5nm9DvDbPH9/Bbw0/zn+GngZnr/XBn6L5yVeMMS/zDyv1wF+m+fP/OcSz99rA7/F8xIvGOJfZp7X6wC/zfNn/nOJ5++1gd/ieYkXDPEvM8/rdYDf5vl7bf5z/TbP32sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae10cDf83/LC8NfDXPS7xgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIOUrpBRbJ26AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPortrait;
impl IconShape for MdPortrait {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 12.25c1.24 0 2.25-1.01 2.25-2.25S13.24 7.75 12 7.75 9.75 8.76 9.75 10s1.01 2.25 2.25 2.25zm4.5 4c0-1.5-3-2.25-4.5-2.25s-4.5.75-4.5 2.25V17h9v-.75zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReceiptLong;
impl IconShape for MdReceiptLong {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.5,3.5L18,2l-1.5,1.5L15,2l-1.5,1.5L12,2l-1.5,1.5L9,2L7.5,3.5L6,2v14H3v3c0,1.66,1.34,3,3,3h12c1.66,0,3-1.34,3-3V2 L19.5,3.5z M19,19c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3H8V5h11V19z",
            }
            rect {
                height: "2",
                width: "6",
                x: "9",
                y: "7",
            }
            rect {
                height: "2",
                width: "2",
                x: "16",
                y: "7",
            }
            rect {
                height: "2",
                width: "6",
                x: "9",
                y: "10",
            }
            rect {
                height: "2",
                width: "2",
                x: "16",
                y: "10",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveRedEye;
impl IconShape for MdRemoveRedEye {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRotate90DegreesCcw;
impl IconShape for MdRotate90DegreesCcw {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.34 6.41L.86 12.9l6.49 6.48 6.49-6.48-6.5-6.49zM3.69 12.9l3.66-3.66L11 12.9l-3.66 3.66-3.65-3.66zm15.67-6.26C17.61 4.88 15.3 4 13 4V.76L8.76 5 13 9.24V6c1.79 0 3.58.68 4.95 2.05 2.73 2.73 2.73 7.17 0 9.9C16.58 19.32 14.79 20 13 20c-.97 0-1.94-.21-2.84-.61l-1.49 1.49C10.02 21.62 11.51 22 13 22c2.3 0 4.61-.88 6.36-2.64 3.52-3.51 3.52-9.21 0-12.72z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRotateLeft;
impl IconShape for MdRotateLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.11 8.53L5.7 7.11C4.8 8.27 4.24 9.61 4.07 11h2.02c.14-.87.49-1.72 1.02-2.47zM6.09 13H4.07c.17 1.39.72 2.73 1.62 3.89l1.41-1.42c-.52-.75-.87-1.59-1.01-2.47zm1.01 5.32c1.16.9 2.51 1.44 3.9 1.61V17.9c-.87-.15-1.71-.49-2.46-1.03L7.1 18.32zM13 4.07V1L8.45 5.55 13 10V6.09c2.84.48 5 2.94 5 5.91s-2.16 5.43-5 5.91v2.02c3.95-.49 7-3.85 7-7.93s-3.05-7.44-7-7.93z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRotateRight;
impl IconShape for MdRotateRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.55 5.55L11 1v3.07C7.06 4.56 4 7.92 4 12s3.05 7.44 7 7.93v-2.02c-2.84-.48-5-2.94-5-5.91s2.16-5.43 5-5.91V10l4.55-4.45zM19.93 11c-.17-1.39-.72-2.73-1.62-3.89l-1.42 1.42c.54.75.88 1.6 1.02 2.47h2.02zM13 17.9v2.02c1.39-.17 2.74-.71 3.9-1.61l-1.44-1.44c-.75.54-1.59.89-2.46 1.03zm3.89-2.42l1.42 1.41c.9-1.16 1.45-2.5 1.62-3.89h-2.02c-.14.87-.48 1.72-1.02 2.48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShutterSpeed;
impl IconShape for MdShutterSpeed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 1H9v2h6V1zm4.03 6.39l1.42-1.42c-.43-.51-.9-.99-1.41-1.41l-1.42 1.42C16.07 4.74 14.12 4 12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9 9-4.03 9-9c0-2.12-.74-4.07-1.97-5.61zM12 20c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm-.32-5H6.35c.57 1.62 1.82 2.92 3.41 3.56l-.11-.06 2.03-3.5zm5.97-4c-.57-1.6-1.78-2.89-3.34-3.54L12.26 11h5.39zm-7.04 7.83c.45.11.91.17 1.39.17 1.34 0 2.57-.45 3.57-1.19l-2.11-3.9-2.85 4.92zM7.55 8.99C6.59 10.05 6 11.46 6 13c0 .34.04.67.09 1h4.72L7.55 8.99zm8.79 8.14C17.37 16.06 18 14.6 18 13c0-.34-.04-.67-.09-1h-4.34l2.77 5.13zm-3.01-9.98C12.9 7.06 12.46 7 12 7c-1.4 0-2.69.49-3.71 1.29l2.32 3.56 2.72-4.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFTElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m3+dBwO7wC7/OV4b+C2el3jBEP8y87xeB/ht/nVeG/gp4HWAv+Y/3msDv8XzEi8Y4l9mntfrAL/Nv85rA7/FFe8DfDf/sV4b+C2el3jBEP8y87xeB/ht/nVeG/gtnu27gffhP85rA7/F8xIvGOJfZp7X6wC/zb/OawO/xXP6a+B1gF3+/V4b+C2el3jBEP8y87xeB/ht/nVeG/gtntcu8DrAX/Pv89rAb/G8xAuG+JeZ5/U6wG/zr/PawG/xgr0P8N3827028Fs8L/GCIf5l5nm9DvDb/Ou8NvBbvHDfDbwP/zavDfwWz0u8YIh/mXlerwP8Nv86rw38Fv+yvwZeB9jlX+e1gd/ieYkXDPEvM8/rdYDf5l/ntYHf4kWzC7wO8Ne86F4b+C2el3jBEP8y87xeB/ht/nVeG/gt/nXeB/huXjSvDfwWz0u8YIh/mXlerwP8Nv86rw38Fv867wN8Ny+a1wZ+i+clXjDEv8w8r9cBfpt/ndcGfosXzSXgtYG/5kX32sBv8bzEC4b4l5nn9TrAb/Ov89rAb/Ev+xvgtYFd/nVeG/gtnpd4wRD/MvO8Xgf4bf51Xhv4LV647wHem3+b1wZ+i+clXjDEv8w8r9cBfpt/ndcGfosX7H2A7+bf7rWB3+J5iRcM8S8zz+t1gN/mX+e1gd/ieV0CXhv4a/59Xhv4LZ6XeMEQ/zLzvF4H+G3+dV4b+C2e098Arw3s8u/32sBv8bzEC4b4l5nn9TrAb/Ov89rAb/Fs3wO8N/9xXhv4LZ6XeMEQ/zLzvF4H+G3+dV4b+C2ueB/gu/mP9drAb/G8xAuG+JeZ5/U6wG/zr/PawE8Drw38Nf/xXhv4LZ6XeMEQ/zLzvF4H+G3+dR4M7AK7/Od4beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a/5neWngq3le4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lDwaezv9ODwFu5QVDvGjeG/gu/nd5H+C7eeEQL7oHA58NvDVwjP+ZLgE/DXw2cCv/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+Eds4vkELzxBOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSlideshow;
impl IconShape for MdSlideshow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 8v8l5-4-5-4zm9-5H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEv0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP7eC/ho4KX5v+Gvga8GvofnhHhe3wW8N/83fTfwPjwb4jl9NvBZ/N/2OcBncwXiOV0EjvN/2y5wgisQz/bawG/x/8PrAL8NIJ7ttYHf4nm9Ds/rq4GX4jn9DfDRPK/f4nl9DPDXPKeXBr6K5/U6PK+vBl6K5/Q3wEfzvH6L5/U6wG8DiGd7beC3eF7ief028Fo8p98BXpvnZZ7X6wC/zXN6beC3eF7ief028Fo8p98BXpvnZZ7X6wC/DSCe7bWB3+J5ief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5ief128Br8Zx+B3htnpd5Xq8D/DaAeLbXBn6L5yWe128Dr8Vz+h3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8Vz+h3gtXle5nm9DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAbwOIZ3tt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL8NIJ7ttYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8NsA4tleG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBvA4hne23gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtntdr87/bb/O8Xgf4bQDxbK8N/Bb/P7wO8NsA4jntAsf4v+0ScJwrEM/ps4HP4v+2zwE+mysQz+u7gffi/6bvAd6bZ0M8f+8NfDTwUvzf8DvAdwPfzXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAbobpUFXVI46AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStraighten;
impl IconShape for MdStraighten {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 10H3V8h2v4h2V8h2v4h2V8h2v4h2V8h2v4h2V8h2v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStyle;
impl IconShape for MdStyle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.53 19.65l1.34.56v-9.03l-2.43 5.86c-.41 1.02.08 2.19 1.09 2.61zm19.5-3.7L17.07 3.98c-.31-.75-1.04-1.21-1.81-1.23-.26 0-.53.04-.79.15L7.1 5.95c-.75.31-1.21 1.03-1.23 1.8-.01.27.04.54.15.8l4.96 11.97c.31.76 1.05 1.22 1.83 1.23.26 0 .52-.05.77-.15l7.36-3.05c1.02-.42 1.51-1.59 1.09-2.6zM7.88 8.75c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-2 11c0 1.1.9 2 2 2h1.45l-3.45-8.34v6.34z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/uV4a+Cr+fT4G+Gv+cyD+87w08FvAcf59doHXAf6a/3iI/xzHgb8CHsx/jFuBlwF2+Y+F+I93HPgt4KX5j/XXwOsAu/zHQfzHOg78FvDS/Of4a+B1gF3+YyBeNC8NHONf9tHAW/Of66eBr+Zfdgn4a144xAv32sB3AQ/mf6dbgfcBfpvnD/GCvTfwXfzf8D7Ad/O8EM/fceDpwHH+b9gFHgLs8pwQz99HA1/F/y0fA3w1zwnx/H038F783/I9wHvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8W/3yXgGP8+l4Bj/Pv9DvDaPCfE8/fbwGvx73MJeG3gr/j3eRngt4Fj/Pv8DvDaPCfE8/fbwGvxb3cJeG3grwHz7yPgpYHfBo7xb/c7wGvznBDP328Dr8W/zSXgtYG/5grz7yOueGngt4Fj/Nv8DvDaPCfE8/fbwGvxr3cJeG3gr3k28+8jnu2lgd8GjvGv9zvAa/OcEM/fbwOvxb/OJeC1gb/mOZl/H/GcXhr4beAY/zq/A7w2zwnx/P028Fq86C4Brw38Nc/L/PuI5/XSwG8Dx3jR/Q7w2jwnxPP328Br8aK5BLw28Nc8f6/Nv89v8/y9NPDbwDFeNL8DvDbPCfH8/TbwWvzLLgGvDfw1/z1eGvht4Bj/st8BXpvnhHj+fht4LV64S8BrA3/Nf6+XBn4bOMYL9zvAa/OcEM/fbwOvxQu3C7wO8Nf893pp4LeA47xwvwO8Ns8J8fz9NvBa/Mt2gdcB/pr/Hi8N/BZwnH/Z7wCvzXNCPH+/DbwWL5pd4HWAv+b5ey3+fX6H5++lgd8CjvOi+R3gtXlOiOfvt4HX4kW3C7wO8Nc8L/PvI57XSwO/BRznRfc7wGvznBDP328Dr8W/zi7wOsBf85zMv494Ti8N/BZwnH+d3wFem+eEeP5+G3gt/vV2gdcB/ppnM/8+4tleGvgt4Dj/er8DvDbPCfH8/TbwWvzb7AKvA/w1V5h/H3HFSwO/BRzn3+Z3gNfmOSGev98GXot/u13gdYC/Bsy/j4CXBn4LOM6/3e8Ar81zQjx/vw28Fv8+u8DrAH/Fv8/LAL8FHOff53eA1+Y5IZ6/3wZei3+/XeA4/z67wHH+/X4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+27gvfi/5WuAj+Y5IZ6/jwa+iv9bPgb4ap4T4vk7DtwKHOP/hkvAg4FdnhPiBXtv4Lv4v+FtgJ/meSFeuNcGvht4EP87PQN4b+C3ef4QL5qXBo7zv8su8Ne8cIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj6/AskHESCtPAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwitchCamera;
impl IconShape for MdSwitchCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4h-3.17L15 2H9L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-5 11.5V13H9v2.5L5.5 12 9 8.5V11h6V8.5l3.5 3.5-3.5 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/89wK/Azw0/z7IJ6/lwb+iv/5Pgf4bF40x4FdnhPi+ftq4KP4n+9W4CH8y14a+GzgrXlOiOfvt4HX4n8H8cK9NPBbwN8Ar81zQjx/vw28Fv87iBfspYHfAo4DvwO8Ns8J8fz9NvBa/O8gnr+XBn4LOM4VvwO8Ns8J8fz9NvBa/O8gntdLA78FHOfZfgd4bZ4T4vn7beC1+N9BPKeXBn4LOM5z+h3gtXlOiOfvt4HX4n8H8WwvDfwWcJzn9TvAa/OcEM/fbwOvxb/OJeAY/3aXgGP864krXhr4LeA4z9/vAK/Nc0I8f78NvBYvur8BPhv4Kf7tfgf4buC7+NcR8NLAbwHHecF+B3htnhPi+ftt4LV40fwN8NrASwO/xb/d7wCvDbw38F286F4G+C3gOC/c7wCvzXNCPH+/DbwW/7K/AV4b2AVeG/gt/u1+B3htrnhv4Lt40ewCx/mX/Q7w2jwnxPP328Br8cL9DfDawC5XvDbwW/zb/Q7w2jzbewPfxX+c3wFem+eEeP5+G3gtXrC/AV4b2OXZXhv4Lf7tfgd4bZ7TewPfxX+M3wFem+eEeP5+G3gtnr+/AV4b2OU5vTbwW/zb/Q7w2jyv9wa+i3+/3wFem+eEeP5+G3gtntffAK8N7PK8jgMvzb/dLvDXPH/vDXwX/z6/A7w2zwnx/P028Fo8p78BXhvY5b/HewPfxb/d7wCvzXNCPH+/DbwWz/Y3wGsDu/z3em/gu/i3+R3gtXlOiOfvt4HX4tn+GngdYJf/Xt8FvDf/Nr8DvDbPCfH8/TbwWjynvwZeB9jlv8d3Ae/Nv93vAK/Nc0I8f78NvBbP66+B1wF2eV7HgZfi3+4S8Nc8f98FvDf/Pr8DvDbPCfH8/TbwWjx/fw28DrDLc3pt4Lf4t/sd4LV5Xt8FvDf/fr8DvDbPCfH8/TbwWrxgfw28DrDLs7028Fv82/0O8No8p+8C3pv/GL8DvDbPCfH8/TbwWrxwfw28DrDLFa8N/Bb/dr8DvDbP9l3Ae/Mf53eA1+Y5IZ6/3wZei3/ZXwOvA+wCrw38Fv92vwO8Nld8F/DevGguAcf4l/0O8No8J8Tz99vAa/Gi+WvgdYCXBn6Lf7vfAV4b+C7gvXnRvQzw28AxXrjfAV6b54R4/n4beC1edH8NfA7wU/zb/Q7wdOC9+dcR8NLAbwPHeMF+B3htnhPi+ftt4LX419kFjvNfT1zx0sBvA8d4/n4HeG2eE+L5+23gtfjfQTzbSwO/DRzjef0O8No8J8Tz99vAa/G/g3hOLw38NnCM5/Q7wGvznBDP328Dr8X/DuJ5vTTw28Axnu13gNfmOSGev98GXov/HcTz99LAbwPHuOJ3gNfmOSGev98GXov/HcQL9tLAbwPHgN8BXpvnhHj+fht4Lf53EC/cSwO/Dfw18No8J8Tz99XAR/E/3zOAB/Mve2ngs4G35jkhnr+XBv6K//k+B/hsXjTHgV2eE+IFe2ngvYGX5n+eW4GfBn6afx/E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CqlO3QcR6flUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwitchVideo;
impl IconShape for MdSwitchVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 9.5V6c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-3.5l4 4v-13l-4 4zm-5 6V13H7v2.5L3.5 12 7 8.5V11h6V8.5l3.5 3.5-3.5 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTagFaces;
impl IconShape for MdTagFaces {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTexture;
impl IconShape for MdTexture {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.51 3.08L3.08 19.51c.09.34.27.65.51.9.25.24.56.42.9.51L20.93 4.49c-.19-.69-.73-1.23-1.42-1.41zM11.88 3L3 11.88v2.83L14.71 3h-2.83zM5 3c-1.1 0-2 .9-2 2v2l4-4H5zm14 18c.55 0 1.05-.22 1.41-.59.37-.36.59-.86.59-1.41v-2l-4 4h2zm-9.71 0h2.83L21 12.12V9.29L9.29 21z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimelapse;
impl IconShape for MdTimelapse {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.24 7.76C15.07 6.59 13.54 6 12 6v6l-4.24 4.24c2.34 2.34 6.14 2.34 8.49 0 2.34-2.34 2.34-6.14-.01-8.48zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimer;
impl IconShape for MdTimer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 1H9v2h6V1zm-4 13h2V8h-2v6zm8.03-6.61l1.42-1.42c-.43-.51-.9-.99-1.41-1.41l-1.42 1.42C16.07 4.74 14.12 4 12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9 9-4.03 9-9c0-2.12-.74-4.07-1.97-5.61zM12 20c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimer10;
impl IconShape for MdTimer10 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 7.72V9.4l3-1V18h2V6h-.25L0 7.72zm23.78 6.65c-.14-.28-.35-.53-.63-.74-.28-.21-.61-.39-1.01-.53s-.85-.27-1.35-.38c-.35-.07-.64-.15-.87-.23-.23-.08-.41-.16-.55-.25-.14-.09-.23-.19-.28-.3-.05-.11-.08-.24-.08-.39 0-.14.03-.28.09-.41.06-.13.15-.25.27-.34.12-.1.27-.18.45-.24s.4-.09.64-.09c.25 0 .47.04.66.11.19.07.35.17.48.29.13.12.22.26.29.42.06.16.1.32.1.49h1.95c0-.39-.08-.75-.24-1.09-.16-.34-.39-.63-.69-.88-.3-.25-.66-.44-1.09-.59C21.49 9.07 21 9 20.46 9c-.51 0-.98.07-1.39.21-.41.14-.77.33-1.06.57-.29.24-.51.52-.67.84-.16.32-.23.65-.23 1.01s.08.69.23.96c.15.28.36.52.64.73.27.21.6.38.98.53.38.14.81.26 1.27.36.39.08.71.17.95.26s.43.19.57.29c.13.1.22.22.27.34.05.12.07.25.07.39 0 .32-.13.57-.4.77-.27.2-.66.29-1.17.29-.22 0-.43-.02-.64-.08-.21-.05-.4-.13-.56-.24-.17-.11-.3-.26-.41-.44-.11-.18-.17-.41-.18-.67h-1.89c0 .36.08.71.24 1.05.16.34.39.65.7.93.31.27.69.49 1.15.66.46.17.98.25 1.58.25.53 0 1.01-.06 1.44-.19.43-.13.8-.31 1.11-.54.31-.23.54-.51.71-.83.17-.32.25-.67.25-1.06-.02-.4-.09-.74-.24-1.02zm-9.96-7.32c-.34-.4-.75-.7-1.23-.88-.47-.18-1.01-.27-1.59-.27-.58 0-1.11.09-1.59.27-.48.18-.89.47-1.23.88-.34.41-.6.93-.79 1.59-.18.65-.28 1.45-.28 2.39v1.92c0 .94.09 1.74.28 2.39.19.66.45 1.19.8 1.6.34.41.75.71 1.23.89.48.18 1.01.28 1.59.28.59 0 1.12-.09 1.59-.28.48-.18.88-.48 1.22-.89.34-.41.6-.94.78-1.6.18-.65.28-1.45.28-2.39v-1.92c0-.94-.09-1.74-.28-2.39-.18-.66-.44-1.19-.78-1.59zm-.92 6.17c0 .6-.04 1.11-.12 1.53-.08.42-.2.76-.36 1.02-.16.26-.36.45-.59.57-.23.12-.51.18-.82.18-.3 0-.58-.06-.82-.18s-.44-.31-.6-.57c-.16-.26-.29-.6-.38-1.02-.09-.42-.13-.93-.13-1.53v-2.5c0-.6.04-1.11.13-1.52.09-.41.21-.74.38-1 .16-.25.36-.43.6-.55.24-.11.51-.17.81-.17.31 0 .58.06.81.17.24.11.44.29.6.55.16.25.29.58.37.99.08.41.13.92.13 1.52v2.51z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/Mc6DrwUz3YJ+Gv+50L8+z0Y+CzgtYEH8/z9NvDdwM8Au/zPgfi3Ow58FfDevOhuBd4H+G3+Z0D82xwHfgt4af5t3gf4bv77If5t/gp4af59Xgb4a/57If71Phv4LJ7XM4DPBn4a2OWK1wY+GngrntdvA6/Dfy/Ev95F4DjP6W+A1wZ2ef6+G3gvntdDgFv574P413lr4Kd4Xg8BbuUFOw7cChzjOX0M8NX890H863w28Fk8p78BXpp/2W8Dr8Vz+hzgs/nvg/jXeW3gtXlOtwLfzb/ss4HP4jn9DvDavGheGngv4KWB1+bZbgX+Gvht4HuAXV50iP86Pw28Fc/pe4D35oV7MPBdwGvzovlq4HOAXf5liP86fwW8NM/pY4Cv5gV7aeC3gOP86/w18DrALi8c4r/GWwM/xXO6BDwY2OX5Ow48HTjOv81fAy/DC4f4z/fWwHcBx3lOnwN8Ni/YdwPvxXO6BHw28N3ALle8NPDVwGvxvN4H+G5eMMR/nM/iOR0HXht4aZ7X9wDvzQt3ETjOc3od4Ld5/n4beC2e018DL8MLhviPY140XwN8NC/cceAiz0u8YG8N/BTPS7xgiP845l/2PsB386Ixz+sEsMvz92DgvXlen80LhviPY1403w18DLDLC7cLHOM5fTfwMcAu/zEQ/3F+m+f0YOBBPH+3Am8D/DUv2GcDn8Xz993AbwO/A9zKvx3iP9dx4KOBjwaO8Zz+GngZXrDjwG8DL8ULdyvw28BPAz/Dvw7iv8ZLA3/F8/oY4Kt5wY4DPw28Fi+aXeCrga8BdvmXIf7rfDfwXjynW4GH8C97a+CjgdfiRXMr8DbAX/PCIf7rvDbwWzwv8aJ7MPDWwFsDr8ULtwu8DHArLxjiv85x4CLP63WA3+bf5qWB9wbeGngQz+tngLfmBUP863w18FI8p+8Bvpt/2XHgIs/rIcCt/PscB74aeC+e1wlgl+cP8a/z28Br8Zy+B3hv/mWvDfwWz0s8r68GXorn9DHAX/OCHQcu8rxeB/htnj/Ev85XAx/Fc9oFHgLs8sJ9N/BePKe/AV6a5/XbwGvxnN4H+G5euKcDD+Y5vQ7w2zx/iH+dlwb+iuf13cD78IK9N/BdPK+PAb6a5/XRwFfxnG4FXgbY5fl7aeCveF4PAW7l+UP86/028Fo8r78Gvhr4GWCXK14LeG/gvXlel4AHA7s8r+PArcAxntOtwGcD38Nzei/gq4HjPKffAV6bFwzxr/dg4K+BY/z7vA3w07xgbw38FC/YLnCcF+5lgL/mBUP827w08NvAMf5t3gf4bv5l7w18F/827wN8Ny8c4t/uwcB3A6/Fi+4ZwHsDv82L7qWBrwZeixfN3wAfDfw2/zLEv99LAx8NvDTwUjyvZwB/DXw38NP827028NbASwOvxXP6HeCvgZ8GfpsXHeI/3ksDx4Fd4K/5nw3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFYu5QdIoJ9AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimer3;
impl IconShape for MdTimer3 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.61 12.97c-.16-.24-.36-.46-.62-.65-.25-.19-.56-.35-.93-.48.3-.14.57-.3.8-.5.23-.2.42-.41.57-.64.15-.23.27-.46.34-.71.08-.24.11-.49.11-.73 0-.55-.09-1.04-.28-1.46-.18-.42-.44-.77-.78-1.06-.33-.28-.73-.5-1.2-.64-.45-.13-.97-.2-1.53-.2-.55 0-1.06.08-1.52.24-.47.17-.87.4-1.2.69-.33.29-.6.63-.78 1.03-.2.39-.29.83-.29 1.29h1.98c0-.26.05-.49.14-.69.09-.2.22-.38.38-.52.17-.14.36-.25.58-.33.22-.08.46-.12.73-.12.61 0 1.06.16 1.36.47.3.31.44.75.44 1.32 0 .27-.04.52-.12.74-.08.22-.21.41-.38.57-.17.16-.38.28-.63.37-.25.09-.55.13-.89.13H6.72v1.57H7.9c.34 0 .64.04.91.11.27.08.5.19.69.35.19.16.34.36.44.61.1.24.16.54.16.87 0 .62-.18 1.09-.53 1.42-.35.33-.84.49-1.45.49-.29 0-.56-.04-.8-.13-.24-.08-.44-.2-.61-.36-.17-.16-.3-.34-.39-.56-.09-.22-.14-.46-.14-.72H4.19c0 .55.11 1.03.32 1.45.21.42.5.77.86 1.05s.77.49 1.24.63.96.21 1.48.21c.57 0 1.09-.08 1.58-.23.49-.15.91-.38 1.26-.68.36-.3.64-.66.84-1.1.2-.43.3-.93.3-1.48 0-.29-.04-.58-.11-.86-.08-.25-.19-.51-.35-.76zm9.26 1.4c-.14-.28-.35-.53-.63-.74-.28-.21-.61-.39-1.01-.53s-.85-.27-1.35-.38c-.35-.07-.64-.15-.87-.23-.23-.08-.41-.16-.55-.25-.14-.09-.23-.19-.28-.3-.05-.11-.08-.24-.08-.39s.03-.28.09-.41c.06-.13.15-.25.27-.34.12-.1.27-.18.45-.24s.4-.09.64-.09c.25 0 .47.04.66.11.19.07.35.17.48.29.13.12.22.26.29.42.06.16.1.32.1.49h1.95c0-.39-.08-.75-.24-1.09-.16-.34-.39-.63-.69-.88-.3-.25-.66-.44-1.09-.59-.43-.15-.92-.22-1.46-.22-.51 0-.98.07-1.39.21-.41.14-.77.33-1.06.57-.29.24-.51.52-.67.84-.16.32-.23.65-.23 1.01s.08.68.23.96c.15.28.37.52.64.73.27.21.6.38.98.53.38.14.81.26 1.27.36.39.08.71.17.95.26s.43.19.57.29c.13.1.22.22.27.34.05.12.07.25.07.39 0 .32-.13.57-.4.77-.27.2-.66.29-1.17.29-.22 0-.43-.02-.64-.08-.21-.05-.4-.13-.56-.24-.17-.11-.3-.26-.41-.44-.11-.18-.17-.41-.18-.67h-1.89c0 .36.08.71.24 1.05.16.34.39.65.7.93.31.27.69.49 1.15.66.46.17.98.25 1.58.25.53 0 1.01-.06 1.44-.19.43-.13.8-.31 1.11-.54.31-.23.54-.51.71-.83.17-.32.25-.67.25-1.06-.02-.4-.09-.74-.24-1.02z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimerOff;
impl IconShape for MdTimerOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.04 4.55l-1.42 1.42C16.07 4.74 14.12 4 12 4c-1.83 0-3.53.55-4.95 1.48l1.46 1.46C9.53 6.35 10.73 6 12 6c3.87 0 7 3.13 7 7 0 1.27-.35 2.47-.94 3.49l1.45 1.45C20.45 16.53 21 14.83 21 13c0-2.12-.74-4.07-1.97-5.61l1.42-1.42-1.41-1.42zM15 1H9v2h6V1zm-4 8.44l2 2V8h-2v1.44zM3.02 4L1.75 5.27 4.5 8.03C3.55 9.45 3 11.16 3 13c0 4.97 4.02 9 9 9 1.84 0 3.55-.55 4.98-1.5l2.5 2.5 1.27-1.27-7.71-7.71L3.02 4zM12 20c-3.87 0-7-3.13-7-7 0-1.28.35-2.48.95-3.52l9.56 9.56c-1.03.61-2.23.96-3.51.96z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTonality;
impl IconShape for MdTonality {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.94-.49-7-3.85-7-7.93s3.05-7.44 7-7.93v15.86zm2-15.86c1.03.13 2 .45 2.87.93H13v-.93zM13 7h5.24c.25.31.48.65.68 1H13V7zm0 3h6.74c.08.33.15.66.19 1H13v-1zm0 9.93V19h2.87c-.87.48-1.84.8-2.87.93zM18.24 17H13v-1h5.92c-.2.35-.43.69-.68 1zm1.5-3H13v-1h6.93c-.04.34-.11.67-.19 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/zoOBW/nfBfEf46WB3wJ+Gngf/vdA/Pu9NPBbwHGu+G7gffjfAfHv89LAbwHHeU7fDbwP//Mh/u1eGvgt4DjP33cD78P/bIh/m5cGfgs4zgv33cD78D8X4l/vpYHfAo7zovlu4H34nwnxr/PSwG8Bx/nX+W7gffifB/GiOw58N3Cc5/RaPK+/AXZ5Tt8NfDf/syD+/czzeh3gt/mfD/HvZ57X6wC/zf98iH8/87xeB/ht/udD/PuZ5/U6wG/zPx/i3888r9cBfpv/+RDP328Dr8W/3esAv81zem3gt/j32QV+Gvgc4Fb+/RDP328Dr8W/3esAv81zem3gt/iP8z7Ad/Pvg3j+fht4Lf7tXgf4bZ7TawO/xX+shwC38m+HeP5+G3gt/u1eB/htntNrA7/Ff6zvAd6bfzvE8/fbwGvxb/c6wG/znF4b+C3+Y+0CJ/i3Qzx/Lw0c50XzWzyv1wF+m+d0HHhp/u1eGvgqnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7FeG/gtnpf4t0P8+5nn9TrAb/Mf67WB3+J5iX87xL+feV6vA/w2/7GOAy/N8/pt/u0Q/37meb0O8Nv8z4f49zPP63WA3+Z/PsS/n3lerwP8Nv/zIf79zPN6HeC3+Z8P8e9nntdHA3/N/yyXgL/mOSH+/XaBY/zP9zvAa/OcEP9+3w28F//z/Q7w2jwnxL/fg4Gn8z/f7wCvzXNC/Md4b+C7+J/td4DX5jkh/uM8GPhs4K2BY/zP8zvAa/OcEP97vDbwWzwv8W+H+N/jtYHf4nmJfzvE/x6vDfwWz0v82yH+93ht4Ld4XuLfDvG/x2sDv8XzEv92iP+ZXhr4Kp7TceCleV6/zXO6BLw3sMu/DPE/13sD38W/ziXgtYG/5kWD+J/tvYHv4kVzCXht4K950SH+53tv4Lt44S4Brw38Nf86iP8d3hv4Lp6/S8BrA3/Nvx7if4/3Br6L53QJeG3gr/m3Qfzv8t7Ad3HFJeC1gb/m3w7xv897A18NvDbw1/z7IP53Og7s8u+H+P8N8f8b/wjPoppBmospQQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTransform;
impl IconShape for MdTransform {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 18v-2H8V4h2L7 1 4 4h2v2H2v2h4v8c0 1.1.9 2 2 2h8v2h-2l3 3 3-3h-2v-2h4zM10 8h6v6h2V8c0-1.1-.9-2-2-2h-6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n888r9cBfpt/P8T/fOZ5vQ7w2/z7If7nM8/rdYDf5t8P8T+feV6vA/w2/36I//nM83od4Lf590P8y36L/xp/A3w0z8s8r78GdnnRvA4vGOJfZv5r/A7w2jwv8+8jXjDEv8z81/gd4LV5XubfR7xgiH+Z+a/xO8Br87zMv494wRD/MvNf43eA1+Z5mX8f8YIh/mW/zX+NvwY+mudlntffALu8aF6bFwzxP595Xq8D/Db/foj/+czzeh3gt/n3Q/zPZ57X6wC/zb8f4n8+87xeB/ht/v0Q//OZ5/U6wG/z74f4j/PawG/xvMT/XIj/OK8N/BbPS/zPhfiP89rAb/G8xP9ciP84rw38Fs9L/M+F+I/z2sBv8bzE/1yIf9lv8aI5Drw0z0s8r68GXor/Gq/DC4b4l5l/H/G8fht4Lf5riBcM8S8z/z7ief028Fr81xAvGOJfZv59xPP6beC1+K8hXjDEv8z8+4jn9dvAa/FfQ7xgiH/Zb/OiOQ68FM9LPK+vBl6a/xqvzQuG+I/z2sBv8bzE/1yI/zivDfwWz0v8z4X4j/PawG/xvMT/XIj/OK8N/BbPS/zPhfiP89rAb/G8xP9ciP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiX/Zb/Pu8Ds/rt4HX4jntAn/Nf7zX4QVD/MvMv494Xr8NvBb/NcQLhviXmX8f8bx+G3gt/muIFwzxLzP/PuJ5/TbwWvzXEC8Y4l9m/n3E8/pt4LX4ryFeMMS/7Lf593ltntdvA6/Fc7oE/DX/8V6bFwzx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAc4ZZEGPTjG/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTune;
impl IconShape for MdTune {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 17v2h6v-2H3zM3 5v2h10V5H3zm10 16v-2h8v-2h-8v-2h-2v6h2zM7 9v2H3v2h4v2h2V9H7zm14 4v-2H11v2h10zm-6-4h2V7h4V5h-4V3h-2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGA0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7Lf4kXzN8BHc8VXAy/Fi+Z1uOKlga/iRfMxwF9zxW/xovkb4KN5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toj/3xD/vyH+Za/Fi+YS8Ndc8dLAMV40v8MVx4GX4kXzN8AuV7wWL5pLwF/znBD/MvOi+R3gtbnit4HX4kUjrnht4Ld40bwO8NtcYV40vwO8Ns8J8S8zL5rfAV6bK34beC1eNOKK1wZ+ixfN6wC/zRXmRfM7wGvznBD/MvOi+R3gtbnit4HX4kUjrnht4Ld40bwO8NtcYV40vwO8Ns8J8S8zL5rfAV6bK34beC1eNOKK1wZ+ixfN6wC/zRXmRfM7wGvznBD/MvOi+R3gtbnit4HX4kUjrnht4Ld40bwO8NtcYV40vwO8Ns8J8S8zL5rfAV6bK34beC1eNOKK1wZ+ixfN6wC/zRXmRfM7wGvznBD/MvOi+R3gtbnit4HX4kUjrnht4Ld40bwO8NtcYV40vwO8Ns8J8S8zL5rfAV6bK34beC1eNOKK1wZ+ixfN6wC/zRXmRfM7wGvznBD/MvOi+R3gtbnit4HX4kUjrnht4Ld40bwO8NtcYV40vwO8Ns8J8S8zL5rfAV6bK34beC1eNOKK1wZ+ixfN6wC/zRXmRfM7wGvznBD/stfmRbML/DVXvDRwnBfNb3PFceCledH8NbDLFa/Ni2YX+GueE+L/N8T/b4h/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/mXnR/A7w2lzx28Br8aIRV7w28Fu8aF4H+G2uMC+a3wFem+eE+JeZF83vAK/NFb8NvBYvGnHFawO/xYvmdYDf5grzovkd4LV5Toh/2W/zovlr4KO54quBl+ZF89pc8dLAV/Oi+Wjgr7nit3nR/DXw0TwnxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AstT7kHyPz0XAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewComfy;
impl IconShape for MdViewComfy {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 9h4V5H3v4zm0 5h4v-4H3v4zm5 0h4v-4H8v4zm5 0h4v-4h-4v4zM8 9h4V5H8v4zm5-4v4h4V5h-4zm5 9h4v-4h-4v4zM3 19h4v-4H3v4zm5 0h4v-4H8v4zm5 0h4v-4h-4v4zm5 0h4v-4h-4v4zm0-14v4h4V5h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADJElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7Lf4v+GvwE+mueE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5l5v+G3wFem+eE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5l5v+G3wFem+eE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5l5v+G3wFem+eE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5lr83/DbvAX/OcEP+/If5/Q/zLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLzL+PuOK1gd/iv8/vAK/Nc0L8y8y/j7jitYHf4r/P7wCvzXNC/MvMv4+44rWB3+K/z+8Ar81zQvzLfpt/n9fmipcGvpr/Pn8NfDTPCfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjyh/W0FWX1koAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewCompact;
impl IconShape for MdViewCompact {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 19h6v-7H3v7zm7 0h12v-7H10v7zM3 5v6h19V5H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/zl+B/hr4K+BW4FbgVt5Tg8GHgw8GHhp4KWB1+I/x+sAvw0gnu21gd/iP87PAN8N/Dawy7/NceC1gfcG3or/OK8D/DaAeLbXBn6Lf59LwFcD3w3cyn+sBwPvDXw0cIx/n9cBfhtAPNtrA7/Fv933AB8N7PKf68HAZwPvxb/d6wC/DSCe7bWB3+Jf7xLw1sBv81/rrYHvBo7xr/c6wG8DiGd7beC3+Nf5G+CtgVv57/Fg4KeBl+Jf53WA3wYQz/bawG/xovsb4LWBXf57HQd+G3gpXnSvA/w2gHi21wZ+ixfNM4CXBnb5n+E48NfAg3jRvA7w2wDi2V4b+C1eNC8D/DX/s7w08Fe8aF4H+G0A8WyvDfwW/7LPAT6b/5k+G/gs/mWvA/w2gHi21wZ+ixfuEvBgYJf/mY4DtwLHeOFeB/htAPFsrw38Fi/c1wAfzf9sXw18FC/c6wC/DSCe7bWB3+KFexngr/mf7aWBv+KFex3gtwHEs7028Fu8YJeA4/zvYF641wF+G0A822sDv8UL9jvAa/O/w28Dr8UL9jrAbwOIZ3tt4Ld4wX4HeG3+d/ht4LV4wV4H+G0A8WyvDfwWL9jvAK/N/w6/DbwWL9jrAL8NIJ7ttYHf4gX7HeC1+d/ht4HX4gV7HeC3AcSzvTbwW7xgu8AJ/ncwL9zrAL8NIJ7ttYHf4oV7GeCv+Z/tpYG/4oV7HeC3AcSzvTbwW7xwXwN8NP+zfTXwUbxwrwP8NoB4ttcGfosXbhd4CLDL/0zHgacDx3nhXgf4bQDxbK8N/Bb/ss8BPpv/mT4b+Cz+Za8D/DaAeLbXBn6LF83LAH/N/ywvDfwVL5rXAX4bQDzbawO/xYvmVuBlgF3+ZzgO/BXwYF40rwP8NoB4ttcGfosX3V8DrwPs8t/rOPBbwEvzonsd4LcBxLO9NvBb/Ov8NfA2wK3893gw8FPAS/Ov8zrAbwOIZ3tt4Lf419sF3gb4bf5rvTXwXcBx/vVeB/htAPFsrw38Fv923w18DnAr/7keDHwW8N78270O8NsA4tleG/gt/n12ga8Gvge4lf9YDwbeC/ho4Dj/Pq8D/DaAeLbXBn6L/zg/Dfw08DPALv82x4G3At4aeGv+47wO8NsA4tleG/gt/nP8NvDXwF8DtwLPAG7lOT0YeBDwYOClgZcGXpv/HK8D/DaAeLbXBn6L/x9eB/htAPFsrw38Fv8/vA7w2wDi2V4b+C3+f3gd4LcBxLO9NvBb/P/wOsBvA4jntAsc4/+2S8BxrkA8p88GPov/2z4H+GyuQDyv7wbei/+bvgd4b54N8fy9N/DRwEvxf8PvAN8NfDfPCfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BKSivkGN7y3hAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVignette;
impl IconShape for MdVignette {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 15c-4.42 0-8-2.69-8-6s3.58-6 8-6 8 2.69 8 6-3.58 6-8 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbAuto;
impl IconShape for MdWbAuto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.85 12.65h2.3L8 9l-1.15 3.65zM22 7l-1.2 6.29L19.3 7h-1.6l-1.49 6.29L15 7h-.76C12.77 5.17 10.53 4 8 4c-4.42 0-8 3.58-8 8s3.58 8 8 8c3.13 0 5.84-1.81 7.15-4.43l.1.43H17l1.5-6.1L20 16h1.75l2.05-9H22zm-11.7 9l-.7-2H6.4l-.7 2H3.8L7 7h2l3.2 9h-1.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgbcBbuXfD/Ef46WB3wKO819jF3gd4K/590H8+x0Hng4c57/WLvAywK382yH+/f4KeGn+e/w18DL82yH+fb4a+Cj+e30O8Nn82yD+7V4a+Cv+Z3gd4Lf510P8670W8NLAewMvzf8Mvw28Dv96iBfNawPvBbw3/3O9DPDX/OsgXriXBr4KeG3+5/se4L3510G8YJ8FfDb/e+wCJ/jXQTx/3wW8N//7vA7w27zoEM/rp4C35n+n7wY+BtjlRYN4Tl8NfBT/++0Cvw18N/AzvGCIZ3tr4Kf4v+dW4GOAn+Z5IZ7t6cCD+b/rt4G3AXZ5NsQV7w18F//3/TXwOsAuVyCueDrwYP5/+GvgdYBdAAEvDfwV/798N/A+AAI+G/gs/v95HeC3Bfw28Fr8//PbwOsI+G3gtfj/6WUEmP+/PkaA+f/rdwTcCjyI/5/+WsBvA6/F/08I+Gngrfj/CQHvDXwX/z/9jYDjwEX+f/odccVvA6/F/z8fI654beC3+P/nIeLZfhp4K/7/+B3gtcWzPRj4a+AY/z+8DvDb4jm9NvBb/N/3PcB7A4jn9d7Ad/F/198Arw3sAojn762B7waO8X/L3wBvDdzKFYgX7KWBnwYexP8NvwO8NbDLsyH+Ze8NfDbwIP53egbw2cB387wQL7q3Bt4aeGvgGP/z/Qzw08BPA7s8f4h/u9fmf6Zd4K950SD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJoHGeOWBDr7AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbCloudy;
impl IconShape for MdWbCloudy {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.36 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.64-4.96z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Ef56XBr4LeGn+ff4aeB/gr/mPh/jP8d7Ad/Ef632A7+Y/FuI/3nsD38V/jvcBvpv/OIj/WO8NfBf/ud4H+G7+YyD+4zwY+CvgOP+5doGXAW7l3w/xH+e3gNfmv8ZvA6/Dvx/iP8ZrA7/Ff63XAX6bfx/Ef4zfAl6b/1q/DbwO/z6If78HA0/nv8dDgFv5t0P8+3008FX89/gY4Kv5t0P8+/008Fb89/ge4L35t0M822sDv8UL9jvAa/O8/gp4af57/DXwMjyv3wZeixfsdYDfBhDP9trAb/GC/Q7w2jwv899LPK/fBl6LF+x1gN8GEM/22sBv8YL9DvDaPC/z30s8r98GXosX7HWA3wYQz/bawG/xgv0O8No8L/Pf5xJwnOf128Br8YK9DvDbAOLZXhv4LV6w3wFem+f128Br8d/jd4DX5nn9NvBavGCvA/w2gHi248BL84LtAn/N8/pt4LX47/E7wGvzvF4aOM4L9tfALoD49/to4Kv47/ExwFfzb4f493tp4K/47/EywF/zb4f4j3Er8CD+az0DeDD/Poj/GO8NfBf/td4H+G7+fRD/cW4FHsR/jWcAD+bfD/Ef562Bn+K/xtsAP82/H+I/1lcDH8V/rs8BPpv/GIj/eD8NvBX/OX4GeGv+4yD+4x0Hvhp4L/5jfQ/w3vzHQvzn+Wjgq/iP8THAV/MfD/Fv92DgVl64BwPfDbwW/za/A7w3cCsv3IOBW/nXQ/zbfBfw1sDrAH/Nv+y1gfcG3ho4xgt3Cfhp4LuB3+Zf9tLAbwE/DbwP/zqIf73vAt6bK3aB1wH+mhfdawMvDRwHHswVtwK7wF8Dv82L7qWB3wKOc8V3A+/Diw7xr/NdwHvznHaB1wH+mv9aLw38FnCc5/TdwPvwokG86L4LeG+ev98BXpv/Wr8NvBbP33cD78O/DPGieW3gt3j+/gZ4bWCX/1rHgd8GXorn73WA3+aFQ7zo3hv4Lp7T3wCvDezy3+M48NvAS/Gc3gf4bv5liH+d9wa+iyv+BnhtYJf/XseB3wZeiiveB/huXjSIf733Bj4aeG1gl3+b3wZei+f0O8Br829zHPht4KuB7+ZFh/jv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x5fDbw0z+mvgY/mvxbi/zfE/2+I/98Q/7/xj7b5nUF8le5VAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbIncandescent;
impl IconShape for MdWbIncandescent {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.55 18.54l1.41 1.41 1.79-1.8-1.41-1.41-1.79 1.8zM11 22.45h2V19.5h-2v2.95zM4 10.5H1v2h3v-2zm11-4.19V1.5H9v4.81C7.21 7.35 6 9.28 6 11.5c0 3.31 2.69 6 6 6s6-2.69 6-6c0-2.22-1.21-4.15-3-5.19zm5 4.19v2h3v-2h-3zm-2.76 7.66l1.79 1.8 1.41-1.41-1.8-1.79-1.4 1.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEm0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfHf472BB/GcngF8N/+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvxr/fSwHsBH8O/3W8Dr8Vz+h3gtfm3+yrge4C/5kWH+Nd5aeC3gOPAdwPvw7/NSwPHeU67wF/zb/NdwHsDu8DrAH/Niwbxontp4LeA4zzbdwPvw3+v7wLem2fbBV4H+Gv+ZYgXzUsDvwUc53l9N/A+/Pf4LuC9eV67wOsAf80Lh3jRHAd+G3gpnr/vBt6H/1rfBbw3z9/fAK8N7PLCIV50x4HfBl6K5/U+wHfzX+u9ge/ief0N8NrALv8yxL/OceC3gZfi2d4H+G7+e7w38F08298Arw3s8qJB/OsdB34beCngfYDv5r/XewPfBfwN8NrALi86xL/NceC1gZ/mf4a3Bn4b2OVfB/H/G+L/N8T/b4j/3xD/vyH+bV4aOMb/LJeAv+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5auCl+Z/lr4GP5l8H8f8b4v83xP9viP/fEP+/If7tHgzcyv8MDwZu5V8P8W/zXcBbA68D/DX/vV4a+C3gp4H34V8H8a/3XcB7c8Uu8DrAX/Pf46WB3wKOc8V3A+/Diw7xr/NdwHvznHaB1wH+mv9aLw38FnCc5/TdwPvwokG86L4LeG+ev98BXpv/Wr8NvBbP33cD78O/DPGieW3gt3j+/gZ4bWCX/1rHgd8GXorn73WA3+aFQ7zo3hv4Lp7T3wCvDezy3+M48NvAS/Gc3gf4bv5liH+d9wa+iyv+BnhtYJf/XseB3wZeiiveB/huXjSIf733Bj4aeG1gl3+b3wZei+f0O8Br829zHPht4KuB7+ZFh/jv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x5fDbw0z+mvgY/mvxbi/zfE/2+I/98Q/7/xj/5PlEEJCBXUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbIridescent;
impl IconShape for MdWbIridescent {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 14.5h14v-6H5v6zM11 .55V3.5h2V.55h-2zm8.04 2.5l-1.79 1.79 1.41 1.41 1.8-1.79-1.42-1.41zM13 22.45V19.5h-2v2.95h2zm7.45-3.91l-1.8-1.79-1.41 1.41 1.79 1.8 1.42-1.42zM3.55 4.46l1.79 1.79 1.41-1.41-1.79-1.79-1.41 1.41zm1.41 15.49l1.79-1.8-1.41-1.41-1.79 1.79 1.41 1.42z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP8eDgVv5nw/xH++7gLcGXgf4a/5nQ/zH+i7gvbliF3gd4K/5nwvxH+e7gPfmOe0CrwP8Nf8zIf5jfBfw3jx/u8DrAH/N/zyIf7/vAt6bF24XeB3gr/mfBfHv813Ae/Oi2QVeB/hr/udA/Nt9F/De/OvsAq8D/DX/MyD+bb4LeG/+bXaB1wH+mv9+iH+97wLem3+fXeB1gL/mvxfiX+e7gPfmP8Yu8DrAX/PfB/Gi+y7gvfmPtQu8DvDX/PdAvGi+C3hv/nPsAq8D/DX/9RD/su8C3pv/XLvA6wB/zX8txAv3XcB7819jF3gd4K/5r4N4wb4LeG/+a+0CrwP8Nf81EM/fewPvzXM6DrwUL9gl4K95Xq/FC/c7PKdd4L2BXf7zIV50rw38Fi/Y7wCvzfMyL5z474N40b028Fu8YL8DvDbPy7xw4r8P4kX32sBv8YL9DvDaPC/zwon/PogX3WsDv8UL9jvAa/O8zAsn/vsgXnSvDfwWL9jvAK/N8zIvnPjvg3jRvTbwW7xgvwO8Ns/LvHDieX018NnALv+5EC+61wZ+ixfsd4DX5nmZF048r98GjgGvA+zynwfxontt4Ld4wX4HeG2el3nhxPP6beC1gL8GXgfY5T8H4kX32sBv8YL9DvDaPC/zwonn9dvAa3HFXwOvA+zyHw/xontt4Ld4wX4HeG2el3nhxPP6beC1eLa/Bl4H2OU/FuJF99rAb/GC7QJ/zfN6bV448bx+G3gtntNfA68D7PIfB/Gie23gt/iPJ57XTwNvxfP6a+B1gF3+YyBedK8N/Bb/8cTzOg78NvBSPK+/Bl4H2OXfD/Gie23gt/iPJ56/48BvAy/F8/pr4HWAXf59EC+61wZ+i/944gU7Dvw28FI8r78GXgfY5d8O8aJ7beC3+I8nXrjjwG8DL8Xz+mvgdYBd/m0QL7rXBn6L/3jiX3Yc+G3gpXhefw28DrDLvx7iRffawG/xH0+8aI4Dvw28FM/rr4HXAXb510G86F4b+C3+44nn9d7Ad/O8jgO/DbwUz+uvgdcBdnnRIV50rw38Fv/xxPP6beDpwPvwvI4Dvw28FM/rr4HXAXZ50SBedK8N/Bb/8cTz+m3gtYDvBt6H53Uc+G3gpXhefw28DrDLvwzxontt4Lf4jyee128Dr8UV3w28D8/rOPDbwEvxvP4aeB1glxcO8aJ7beC3+I8nntdvA6/Fs3038D48r+PAbwMvxfP6a+B1gF1eMMSL7rWB3+I/nnhevw28Fs/pu4H34XkdB34beCme118DrwPs8vwhXnSvDfwW//HE8/pt4LV4Xt8NvA/P6zjw28BL8bz+GngdYJfnhXjRvTbwW/zHE8/rt4HX4vn7buB9eF7Hgd8GXorn9dfA6wC7PCfEi+448NL8x/ttntdLA8d5wf4a2OV5HQdemufvVuBWnhPi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COPlrRBJu2u5gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbShade;
impl IconShape for MdWbShade {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 12v2.5l5.5 5.5H22zm0 8h3l-3-3zM8 4l-6 6h2v10h8V10h2L8 4zm1 10H7v-4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbSunny;
impl IconShape for MdWbSunny {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.76 4.84l-1.8-1.79-1.41 1.41 1.79 1.79 1.42-1.41zM4 10.5H1v2h3v-2zm9-9.95h-2V3.5h2V.55zm7.45 3.91l-1.41-1.41-1.79 1.79 1.41 1.41 1.79-1.79zm-3.21 13.7l1.79 1.8 1.41-1.41-1.8-1.79-1.4 1.4zM20 10.5v2h3v-2h-3zm-8-5c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm-1 16.95h2V19.5h-2v2.95zm-7.45-3.91l1.41 1.41 1.79-1.8-1.41-1.41-1.79 1.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dLA8d4TpeAv+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/vWOA68N/DT/dr8NvBbP6XeA1+bf7qWBW4FdXnSIf53jwG8BLw28D/Dd/Nt8NfDSPKe/Bj6af5uXBn4LuBV4HWCXFw3iRXcc+C3gpXm29wG+m/9eLw38FnCcK/4aeB1gl38Z4kVzHPgt4KV5Xu8DfDf/PV4a+C3gOM/pr4HXAXZ54RAvmpcGfhs4xvP3PsB381/rpYHfAo7z/L0N8NO8cIgX3UsDvw0c4/l7H+C7+a/x0sBvAcd5/t4H+G7+ZYh/nZcGfhs4xvN6H+C7+a/x3sB38fy9D/DdvGgQ/3ovDfw2cIxnex/gu/mXvRXw2sBLAw8GHswVtwK3An8N/DbwM/zL3hv4Lp7T+wDfzYsO8W/z0sBvA8eA9wG+mxfswcBHAe8NHOdFswt8NfA9wK28YO8NfBdXvA/w3fzrIP7tXhp4aeC7ecE+C/hs/n0+G/gcXrD35orv5l8P8Z/jwcBPAS/Nf4zfBt4G2OU/FuI/3ksDvwUc5z/WXwOvA+zyHwfxH+ulgd8CjvOf46+B1wF2+Y+B+I/1V8BL85/rt4HX4T8G4j/OZwOfxX+NjwG+mn8/xH+MBwNP57/OLvAQYJd/H8R/jK8GPor/Wp8DfDb/Poj/GBeB4/zX2gVO8O+D+Pd7beC3+O/xOsBv82+H+Pf7bOCz+O/xOcBn82+H+Pd7aeA4/z12gb/m3w7x/xvi/zfE/2+I/98Qz99LA8f4v+US8Nc8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fSwHH+b9kF/prnhPj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CLG4d0HKPyW6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWbTwighlight;
impl IconShape for MdWbTwighlight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.954 8.66l2.12-2.12 1.415 1.414-2.13 2.12zM17.9 14c-.5-2.85-2.95-5-5.9-5s-5.45 2.15-5.9 5h11.8zM2 16h20v4H2zm9-12h2v3h-2zM3.54 7.925L4.954 6.51l2.122 2.122-1.415 1.415z",
            }
        }
    }
}
