use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MdIndeterminateCheckBox;
impl IconShape for MdIndeterminateCheckBox {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            rect {
                height: "24",
                width: "24",
            }
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M17,13H7v-2h10V13z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStarBorder;
impl IconShape for MdStarBorder {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M22 9.24l-7.19-.62L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.63-7.03L22 9.24zM12 15.4l-3.76 2.27 1-4.28-3.32-2.88 4.38-.38L12 6.1l1.71 4.04 4.38.38-3.32 2.88 1 4.28L12 15.4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRadioButtonUnchecked;
impl IconShape for MdRadioButtonUnchecked {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCheckBoxOutlineBlank;
impl IconShape for MdCheckBoxOutlineBlank {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStarHalf;
impl IconShape for MdStarHalf {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            rect {
                height: "24",
                width: "24",
                x: "0",
            }
            path {
                d: "M22,9.24l-7.19-0.62L12,2L9.19,8.63L2,9.24l5.46,4.73L5.82,21L12,17.27L18.18,21l-1.63-7.03L22,9.24z M12,15.4V6.1 l1.71,4.04l4.38,0.38l-3.32,2.88l1,4.28L12,15.4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRadioButtonChecked;
impl IconShape for MdRadioButtonChecked {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0-5C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdToggleOn;
impl IconShape for MdToggleOn {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zm0 8c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdToggleOff;
impl IconShape for MdToggleOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zM7 15c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStar;
impl IconShape for MdStar {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStarOutline;
impl IconShape for MdStarOutline {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M22 9.24l-7.19-.62L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.63-7.03L22 9.24zM12 15.4l-3.76 2.27 1-4.28-3.32-2.88 4.38-.38L12 6.1l1.71 4.04 4.38.38-3.32 2.88 1 4.28L12 15.4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCheckBox;
impl IconShape for MdCheckBox {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            }
        }
    }
}
