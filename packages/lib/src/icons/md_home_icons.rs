use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorDoor;
impl IconShape for MdSensorDoor {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn width(&self) -> &str {
        "24"
    }
    fn height(&self) -> &str {
        "24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "black"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn title(&self) -> &str {
        ""
    }
    fn child_elements(&self) -> Element {
        rsx! {
            g {
                rect {
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 S17,11.17,17,12S16.33,13.5,15.5,13.5z",
                }
            }
            
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorWindow;
impl IconShape for MdSensorWindow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn width(&self) -> &str {
        "24"
    }
    fn height(&self) -> &str {
        "24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "black"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn title(&self) -> &str {
        ""
    }
    fn child_elements(&self) -> Element {
        rsx! {
            g {
                rect {
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2L18,2z M7,19h10v-6H7 V19z M10,10h4v1h3V5H7v6h3V10z",
                }
            }
            
        }
    }
}
