use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiActivity;
impl IconShape for FiActivity {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "22 12 18 12 15 21 9 3 6 12 2 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAirplay;
impl IconShape for FiAirplay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1",
            }
            polygon {
                points: "12 15 17 21 7 21 12 15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlertCircle;
impl IconShape for FiAlertCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "16",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlertOctagon;
impl IconShape for FiAlertOctagon {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "16",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlertTriangle;
impl IconShape for FiAlertTriangle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "9",
                y2: "13",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "17",
                y2: "17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlignCenter;
impl IconShape for FiAlignCenter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "18",
                x2: "6",
                y1: "10",
                y2: "10",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "18",
                x2: "6",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlignJustify;
impl IconShape for FiAlignJustify {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "21",
                x2: "3",
                y1: "10",
                y2: "10",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlignLeft;
impl IconShape for FiAlignLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "17",
                x2: "3",
                y1: "10",
                y2: "10",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "17",
                x2: "3",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAlignRight;
impl IconShape for FiAlignRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "21",
                x2: "7",
                y1: "10",
                y2: "10",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "21",
                x2: "3",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "21",
                x2: "7",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAnchor;
impl IconShape for FiAnchor {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "5",
                r: "3",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22",
                y2: "8",
            }
            path {
                d: "M5 12H2a10 10 0 0 0 20 0h-3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAperture;
impl IconShape for FiAperture {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "14.31",
                x2: "20.05",
                y1: "8",
                y2: "17.94",
            }
            line {
                x1: "9.69",
                x2: "21.17",
                y1: "8",
                y2: "8",
            }
            line {
                x1: "7.38",
                x2: "13.12",
                y1: "12",
                y2: "2.06",
            }
            line {
                x1: "9.69",
                x2: "3.95",
                y1: "16",
                y2: "6.06",
            }
            line {
                x1: "14.31",
                x2: "2.83",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "16.62",
                x2: "10.88",
                y1: "12",
                y2: "21.94",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArchive;
impl IconShape for FiArchive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "21 8 21 21 3 21 3 8",
            }
            rect {
                height: "5",
                width: "22",
                x: "1",
                y: "3",
            }
            line {
                x1: "10",
                x2: "14",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowDownCircle;
impl IconShape for FiArrowDownCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polyline {
                points: "8 12 12 16 16 12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowDownLeft;
impl IconShape for FiArrowDownLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "17",
                x2: "7",
                y1: "7",
                y2: "17",
            }
            polyline {
                points: "17 17 7 17 7 7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowDownRight;
impl IconShape for FiArrowDownRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "7",
                x2: "17",
                y1: "7",
                y2: "17",
            }
            polyline {
                points: "17 7 17 17 7 17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowDown;
impl IconShape for FiArrowDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "5",
                y2: "19",
            }
            polyline {
                points: "19 12 12 19 5 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowLeftCircle;
impl IconShape for FiArrowLeftCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polyline {
                points: "12 8 8 12 12 16",
            }
            line {
                x1: "16",
                x2: "8",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowLeft;
impl IconShape for FiArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "19",
                x2: "5",
                y1: "12",
                y2: "12",
            }
            polyline {
                points: "12 19 5 12 12 5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowRightCircle;
impl IconShape for FiArrowRightCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polyline {
                points: "12 16 16 12 12 8",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowRight;
impl IconShape for FiArrowRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "5",
                x2: "19",
                y1: "12",
                y2: "12",
            }
            polyline {
                points: "12 5 19 12 12 19",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowUpCircle;
impl IconShape for FiArrowUpCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polyline {
                points: "16 12 12 8 8 12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "16",
                y2: "8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowUpLeft;
impl IconShape for FiArrowUpLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "17",
                x2: "7",
                y1: "17",
                y2: "7",
            }
            polyline {
                points: "7 17 7 7 17 7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowUpRight;
impl IconShape for FiArrowUpRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "7",
                x2: "17",
                y1: "17",
                y2: "7",
            }
            polyline {
                points: "7 7 17 7 17 17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiArrowUp;
impl IconShape for FiArrowUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "19",
                y2: "5",
            }
            polyline {
                points: "5 12 12 5 19 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAtSign;
impl IconShape for FiAtSign {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "4",
            }
            path {
                d: "M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiAward;
impl IconShape for FiAward {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "8",
                r: "7",
            }
            polyline {
                points: "8.21 13.89 7 23 12 20 17 23 15.79 13.88",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBarChart2;
impl IconShape for FiBarChart2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "18",
                x2: "18",
                y1: "20",
                y2: "10",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "20",
                y2: "4",
            }
            line {
                x1: "6",
                x2: "6",
                y1: "20",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBarChart;
impl IconShape for FiBarChart {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "20",
                y2: "10",
            }
            line {
                x1: "18",
                x2: "18",
                y1: "20",
                y2: "4",
            }
            line {
                x1: "6",
                x2: "6",
                y1: "20",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBatteryCharging;
impl IconShape for FiBatteryCharging {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19",
            }
            line {
                x1: "23",
                x2: "23",
                y1: "13",
                y2: "11",
            }
            polyline {
                points: "11 6 7 12 13 12 9 18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBattery;
impl IconShape for FiBattery {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "12",
                rx: "2",
                ry: "2",
                width: "18",
                x: "1",
                y: "6",
            }
            line {
                x1: "23",
                x2: "23",
                y1: "13",
                y2: "11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBellOff;
impl IconShape for FiBellOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.73 21a2 2 0 0 1-3.46 0",
            }
            path {
                d: "M18.63 13A17.89 17.89 0 0 1 18 8",
            }
            path {
                d: "M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14",
            }
            path {
                d: "M18 8a6 6 0 0 0-9.33-5",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBell;
impl IconShape for FiBell {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9",
            }
            path {
                d: "M13.73 21a2 2 0 0 1-3.46 0",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBluetooth;
impl IconShape for FiBluetooth {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBold;
impl IconShape for FiBold {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z",
            }
            path {
                d: "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBookOpen;
impl IconShape for FiBookOpen {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z",
            }
            path {
                d: "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBook;
impl IconShape for FiBook {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20",
            }
            path {
                d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBookmark;
impl IconShape for FiBookmark {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBox;
impl IconShape for FiBox {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z",
            }
            polyline {
                points: "3.27 6.96 12 12.01 20.73 6.96",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22.08",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiBriefcase;
impl IconShape for FiBriefcase {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "14",
                rx: "2",
                ry: "2",
                width: "20",
                x: "2",
                y: "7",
            }
            path {
                d: "M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCalendar;
impl IconShape for FiCalendar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "4",
            }
            line {
                x1: "16",
                x2: "16",
                y1: "2",
                y2: "6",
            }
            line {
                x1: "8",
                x2: "8",
                y1: "2",
                y2: "6",
            }
            line {
                x1: "3",
                x2: "21",
                y1: "10",
                y2: "10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCameraOff;
impl IconShape for FiCameraOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
            path {
                d: "M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCamera;
impl IconShape for FiCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z",
            }
            circle {
                cx: "12",
                cy: "13",
                r: "4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCast;
impl IconShape for FiCast {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6",
            }
            line {
                x1: "2",
                x2: "2.01",
                y1: "20",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCheckCircle;
impl IconShape for FiCheckCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 11.08V12a10 10 0 1 1-5.93-9.14",
            }
            polyline {
                points: "22 4 12 14.01 9 11.01",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCheckSquare;
impl IconShape for FiCheckSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "9 11 12 14 22 4",
            }
            path {
                d: "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCheck;
impl IconShape for FiCheck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "20 6 9 17 4 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronDown;
impl IconShape for FiChevronDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "6 9 12 15 18 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronLeft;
impl IconShape for FiChevronLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "15 18 9 12 15 6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronRight;
impl IconShape for FiChevronRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "9 18 15 12 9 6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronUp;
impl IconShape for FiChevronUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "18 15 12 9 6 15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronsDown;
impl IconShape for FiChevronsDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "7 13 12 18 17 13",
            }
            polyline {
                points: "7 6 12 11 17 6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronsLeft;
impl IconShape for FiChevronsLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "11 17 6 12 11 7",
            }
            polyline {
                points: "18 17 13 12 18 7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronsRight;
impl IconShape for FiChevronsRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "13 17 18 12 13 7",
            }
            polyline {
                points: "6 17 11 12 6 7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChevronsUp;
impl IconShape for FiChevronsUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "17 11 12 6 7 11",
            }
            polyline {
                points: "17 18 12 13 7 18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiChrome;
impl IconShape for FiChrome {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "4",
            }
            line {
                x1: "21.17",
                x2: "12",
                y1: "8",
                y2: "8",
            }
            line {
                x1: "3.95",
                x2: "8.54",
                y1: "6.06",
                y2: "14",
            }
            line {
                x1: "10.88",
                x2: "15.46",
                y1: "21.94",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCircle;
impl IconShape for FiCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
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

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiClipboard;
impl IconShape for FiClipboard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2",
            }
            rect {
                height: "4",
                rx: "1",
                ry: "1",
                width: "8",
                x: "8",
                y: "2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiClock;
impl IconShape for FiClock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polyline {
                points: "12 6 12 12 16 14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloudDrizzle;
impl IconShape for FiCloudDrizzle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "8",
                x2: "8",
                y1: "19",
                y2: "21",
            }
            line {
                x1: "8",
                x2: "8",
                y1: "13",
                y2: "15",
            }
            line {
                x1: "16",
                x2: "16",
                y1: "19",
                y2: "21",
            }
            line {
                x1: "16",
                x2: "16",
                y1: "13",
                y2: "15",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "21",
                y2: "23",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "15",
                y2: "17",
            }
            path {
                d: "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloudLightning;
impl IconShape for FiCloudLightning {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9",
            }
            polyline {
                points: "13 11 9 17 15 17 11 23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloudOff;
impl IconShape for FiCloudOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloudRain;
impl IconShape for FiCloudRain {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "16",
                x2: "16",
                y1: "13",
                y2: "21",
            }
            line {
                x1: "8",
                x2: "8",
                y1: "13",
                y2: "21",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "15",
                y2: "23",
            }
            path {
                d: "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloudSnow;
impl IconShape for FiCloudSnow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25",
            }
            line {
                x1: "8",
                x2: "8.01",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "8",
                x2: "8.01",
                y1: "20",
                y2: "20",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "22",
                y2: "22",
            }
            line {
                x1: "16",
                x2: "16.01",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "16",
                x2: "16.01",
                y1: "20",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCloud;
impl IconShape for FiCloud {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCode;
impl IconShape for FiCode {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "16 18 22 12 16 6",
            }
            polyline {
                points: "8 6 2 12 8 18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCodepen;
impl IconShape for FiCodepen {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22",
                y2: "15.5",
            }
            polyline {
                points: "22 8.5 12 15.5 2 8.5",
            }
            polyline {
                points: "2 15.5 12 8.5 22 15.5",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "8.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCodesandbox;
impl IconShape for FiCodesandbox {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z",
            }
            polyline {
                points: "7.5 4.21 12 6.81 16.5 4.21",
            }
            polyline {
                points: "7.5 19.79 7.5 14.6 3 12",
            }
            polyline {
                points: "21 12 16.5 14.6 16.5 19.79",
            }
            polyline {
                points: "3.27 6.96 12 12.01 20.73 6.96",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22.08",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCoffee;
impl IconShape for FiCoffee {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 8h1a4 4 0 0 1 0 8h-1",
            }
            path {
                d: "M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z",
            }
            line {
                x1: "6",
                x2: "6",
                y1: "1",
                y2: "4",
            }
            line {
                x1: "10",
                x2: "10",
                y1: "1",
                y2: "4",
            }
            line {
                x1: "14",
                x2: "14",
                y1: "1",
                y2: "4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiColumns;
impl IconShape for FiColumns {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCommand;
impl IconShape for FiCommand {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCompass;
impl IconShape for FiCompass {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polygon {
                points: "16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCopy;
impl IconShape for FiCopy {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "13",
                rx: "2",
                ry: "2",
                width: "13",
                x: "9",
                y: "9",
            }
            path {
                d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerDownLeft;
impl IconShape for FiCornerDownLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "9 10 4 15 9 20",
            }
            path {
                d: "M20 4v7a4 4 0 0 1-4 4H4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerDownRight;
impl IconShape for FiCornerDownRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "15 10 20 15 15 20",
            }
            path {
                d: "M4 4v7a4 4 0 0 0 4 4h12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerLeftDown;
impl IconShape for FiCornerLeftDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "14 15 9 20 4 15",
            }
            path {
                d: "M20 4h-7a4 4 0 0 0-4 4v12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerLeftUp;
impl IconShape for FiCornerLeftUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "14 9 9 4 4 9",
            }
            path {
                d: "M20 20h-7a4 4 0 0 1-4-4V4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerRightDown;
impl IconShape for FiCornerRightDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "10 15 15 20 20 15",
            }
            path {
                d: "M4 4h7a4 4 0 0 1 4 4v12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerRightUp;
impl IconShape for FiCornerRightUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "10 9 15 4 20 9",
            }
            path {
                d: "M4 20h7a4 4 0 0 0 4-4V4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerUpLeft;
impl IconShape for FiCornerUpLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "9 14 4 9 9 4",
            }
            path {
                d: "M20 20v-7a4 4 0 0 0-4-4H4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCornerUpRight;
impl IconShape for FiCornerUpRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "15 14 20 9 15 4",
            }
            path {
                d: "M4 20v-7a4 4 0 0 1 4-4h12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCpu;
impl IconShape for FiCpu {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "16",
                rx: "2",
                ry: "2",
                width: "16",
                x: "4",
                y: "4",
            }
            rect {
                height: "6",
                width: "6",
                x: "9",
                y: "9",
            }
            line {
                x1: "9",
                x2: "9",
                y1: "1",
                y2: "4",
            }
            line {
                x1: "15",
                x2: "15",
                y1: "1",
                y2: "4",
            }
            line {
                x1: "9",
                x2: "9",
                y1: "20",
                y2: "23",
            }
            line {
                x1: "15",
                x2: "15",
                y1: "20",
                y2: "23",
            }
            line {
                x1: "20",
                x2: "23",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "20",
                x2: "23",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "1",
                x2: "4",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "1",
                x2: "4",
                y1: "14",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCreditCard;
impl IconShape for FiCreditCard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "16",
                rx: "2",
                ry: "2",
                width: "22",
                x: "1",
                y: "4",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "10",
                y2: "10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCrop;
impl IconShape for FiCrop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.13 1L6 16a2 2 0 0 0 2 2h15",
            }
            path {
                d: "M1 6.13L16 6a2 2 0 0 1 2 2v15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiCrosshair;
impl IconShape for FiCrosshair {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "22",
                x2: "18",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "6",
                x2: "2",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "6",
                y2: "2",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDatabase;
impl IconShape for FiDatabase {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            ellipse {
                cx: "12",
                cy: "5",
                rx: "9",
                ry: "3",
            }
            path {
                d: "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3",
            }
            path {
                d: "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDelete;
impl IconShape for FiDelete {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z",
            }
            line {
                x1: "18",
                x2: "12",
                y1: "9",
                y2: "15",
            }
            line {
                x1: "12",
                x2: "18",
                y1: "9",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDisc;
impl IconShape for FiDisc {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDivideCircle;
impl IconShape for FiDivideCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "8",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDivideSquare;
impl IconShape for FiDivideSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDivide;
impl IconShape for FiDivide {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "6",
                r: "2",
            }
            line {
                x1: "5",
                x2: "19",
                y1: "12",
                y2: "12",
            }
            circle {
                cx: "12",
                cy: "18",
                r: "2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDollarSign;
impl IconShape for FiDollarSign {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "1",
                y2: "23",
            }
            path {
                d: "M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDownloadCloud;
impl IconShape for FiDownloadCloud {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "8 17 12 21 16 17",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "12",
                y2: "21",
            }
            path {
                d: "M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDownload;
impl IconShape for FiDownload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4",
            }
            polyline {
                points: "7 10 12 15 17 10",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "15",
                y2: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDribbble;
impl IconShape for FiDribbble {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            path {
                d: "M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiDroplet;
impl IconShape for FiDroplet {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiEdit2;
impl IconShape for FiEdit2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiEdit3;
impl IconShape for FiEdit3 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 20h9",
            }
            path {
                d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiEdit;
impl IconShape for FiEdit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7",
            }
            path {
                d: "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiExternalLink;
impl IconShape for FiExternalLink {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6",
            }
            polyline {
                points: "15 3 21 3 21 9",
            }
            line {
                x1: "10",
                x2: "21",
                y1: "14",
                y2: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiEyeOff;
impl IconShape for FiEyeOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiEye;
impl IconShape for FiEye {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFacebook;
impl IconShape for FiFacebook {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFastForward;
impl IconShape for FiFastForward {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "13 19 22 12 13 5 13 19",
            }
            polygon {
                points: "2 19 11 12 2 5 2 19",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFeather;
impl IconShape for FiFeather {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z",
            }
            line {
                x1: "16",
                x2: "2",
                y1: "8",
                y2: "22",
            }
            line {
                x1: "17.5",
                x2: "9",
                y1: "15",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFigma;
impl IconShape for FiFigma {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z",
            }
            path {
                d: "M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z",
            }
            path {
                d: "M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z",
            }
            path {
                d: "M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z",
            }
            path {
                d: "M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFileMinus;
impl IconShape for FiFileMinus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z",
            }
            polyline {
                points: "14 2 14 8 20 8",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "15",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFilePlus;
impl IconShape for FiFilePlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z",
            }
            polyline {
                points: "14 2 14 8 20 8",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "18",
                y2: "12",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "15",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFileText;
impl IconShape for FiFileText {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z",
            }
            polyline {
                points: "14 2 14 8 20 8",
            }
            line {
                x1: "16",
                x2: "8",
                y1: "13",
                y2: "13",
            }
            line {
                x1: "16",
                x2: "8",
                y1: "17",
                y2: "17",
            }
            polyline {
                points: "10 9 9 9 8 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFile;
impl IconShape for FiFile {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z",
            }
            polyline {
                points: "13 2 13 9 20 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFilm;
impl IconShape for FiFilm {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "20",
                rx: "2.18",
                ry: "2.18",
                width: "20",
                x: "2",
                y: "2",
            }
            line {
                x1: "7",
                x2: "7",
                y1: "2",
                y2: "22",
            }
            line {
                x1: "17",
                x2: "17",
                y1: "2",
                y2: "22",
            }
            line {
                x1: "2",
                x2: "22",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "2",
                x2: "7",
                y1: "7",
                y2: "7",
            }
            line {
                x1: "2",
                x2: "7",
                y1: "17",
                y2: "17",
            }
            line {
                x1: "17",
                x2: "22",
                y1: "17",
                y2: "17",
            }
            line {
                x1: "17",
                x2: "22",
                y1: "7",
                y2: "7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFilter;
impl IconShape for FiFilter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFlag;
impl IconShape for FiFlag {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z",
            }
            line {
                x1: "4",
                x2: "4",
                y1: "22",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFolderMinus;
impl IconShape for FiFolderMinus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "14",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFolderPlus;
impl IconShape for FiFolderPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "11",
                y2: "17",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "14",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFolder;
impl IconShape for FiFolder {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFramer;
impl IconShape for FiFramer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiFrown;
impl IconShape for FiFrown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            path {
                d: "M16 16s-1.5-2-4-2-4 2-4 2",
            }
            line {
                x1: "9",
                x2: "9.01",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "15",
                x2: "15.01",
                y1: "9",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGift;
impl IconShape for FiGift {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "20 12 20 22 4 22 4 12",
            }
            rect {
                height: "5",
                width: "20",
                x: "2",
                y: "7",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22",
                y2: "7",
            }
            path {
                d: "M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z",
            }
            path {
                d: "M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGitBranch;
impl IconShape for FiGitBranch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "6",
                x2: "6",
                y1: "3",
                y2: "15",
            }
            circle {
                cx: "18",
                cy: "6",
                r: "3",
            }
            circle {
                cx: "6",
                cy: "18",
                r: "3",
            }
            path {
                d: "M18 9a9 9 0 0 1-9 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGitCommit;
impl IconShape for FiGitCommit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "4",
            }
            line {
                x1: "1.05",
                x2: "7",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "17.01",
                x2: "22.96",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGitMerge;
impl IconShape for FiGitMerge {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "18",
                cy: "18",
                r: "3",
            }
            circle {
                cx: "6",
                cy: "6",
                r: "3",
            }
            path {
                d: "M6 21V9a9 9 0 0 0 9 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGitPullRequest;
impl IconShape for FiGitPullRequest {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "18",
                cy: "18",
                r: "3",
            }
            circle {
                cx: "6",
                cy: "6",
                r: "3",
            }
            path {
                d: "M13 6h3a2 2 0 0 1 2 2v7",
            }
            line {
                x1: "6",
                x2: "6",
                y1: "9",
                y2: "21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGithub;
impl IconShape for FiGithub {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGitlab;
impl IconShape for FiGitlab {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGlobe;
impl IconShape for FiGlobe {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "2",
                x2: "22",
                y1: "12",
                y2: "12",
            }
            path {
                d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiGrid;
impl IconShape for FiGrid {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "7",
                width: "7",
                x: "3",
                y: "3",
            }
            rect {
                height: "7",
                width: "7",
                x: "14",
                y: "3",
            }
            rect {
                height: "7",
                width: "7",
                x: "14",
                y: "14",
            }
            rect {
                height: "7",
                width: "7",
                x: "3",
                y: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHardDrive;
impl IconShape for FiHardDrive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "22",
                x2: "2",
                y1: "12",
                y2: "12",
            }
            path {
                d: "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z",
            }
            line {
                x1: "6",
                x2: "6.01",
                y1: "16",
                y2: "16",
            }
            line {
                x1: "10",
                x2: "10.01",
                y1: "16",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHash;
impl IconShape for FiHash {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "4",
                x2: "20",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "4",
                x2: "20",
                y1: "15",
                y2: "15",
            }
            line {
                x1: "10",
                x2: "8",
                y1: "3",
                y2: "21",
            }
            line {
                x1: "16",
                x2: "14",
                y1: "3",
                y2: "21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHeadphones;
impl IconShape for FiHeadphones {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 18v-6a9 9 0 0 1 18 0v6",
            }
            path {
                d: "M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHeart;
impl IconShape for FiHeart {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHelpCircle;
impl IconShape for FiHelpCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            path {
                d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "17",
                y2: "17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHexagon;
impl IconShape for FiHexagon {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiHome;
impl IconShape for FiHome {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
            }
            polyline {
                points: "9 22 9 12 15 12 15 22",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiImage;
impl IconShape for FiImage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            circle {
                cx: "8.5",
                cy: "8.5",
                r: "1.5",
            }
            polyline {
                points: "21 15 16 10 5 21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiInbox;
impl IconShape for FiInbox {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "22 12 16 12 14 15 10 15 8 12 2 12",
            }
            path {
                d: "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiInfo;
impl IconShape for FiInfo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "16",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "8",
                y2: "8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiInstagram;
impl IconShape for FiInstagram {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "20",
                rx: "5",
                ry: "5",
                width: "20",
                x: "2",
                y: "2",
            }
            path {
                d: "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z",
            }
            line {
                x1: "17.5",
                x2: "17.51",
                y1: "6.5",
                y2: "6.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiItalic;
impl IconShape for FiItalic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "19",
                x2: "10",
                y1: "4",
                y2: "4",
            }
            line {
                x1: "14",
                x2: "5",
                y1: "20",
                y2: "20",
            }
            line {
                x1: "15",
                x2: "9",
                y1: "4",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiKey;
impl IconShape for FiKey {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLayers;
impl IconShape for FiLayers {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 2 7 12 12 22 7 12 2",
            }
            polyline {
                points: "2 17 12 22 22 17",
            }
            polyline {
                points: "2 12 12 17 22 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLayout;
impl IconShape for FiLayout {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "3",
                x2: "21",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "9",
                x2: "9",
                y1: "21",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLifeBuoy;
impl IconShape for FiLifeBuoy {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "4",
            }
            line {
                x1: "4.93",
                x2: "9.17",
                y1: "4.93",
                y2: "9.17",
            }
            line {
                x1: "14.83",
                x2: "19.07",
                y1: "14.83",
                y2: "19.07",
            }
            line {
                x1: "14.83",
                x2: "19.07",
                y1: "9.17",
                y2: "4.93",
            }
            line {
                x1: "14.83",
                x2: "18.36",
                y1: "9.17",
                y2: "5.64",
            }
            line {
                x1: "4.93",
                x2: "9.17",
                y1: "19.07",
                y2: "14.83",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLink2;
impl IconShape for FiLink2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLink;
impl IconShape for FiLink {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71",
            }
            path {
                d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLinkedin;
impl IconShape for FiLinkedin {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z",
            }
            rect {
                height: "12",
                width: "4",
                x: "2",
                y: "9",
            }
            circle {
                cx: "4",
                cy: "4",
                r: "2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiList;
impl IconShape for FiList {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "8",
                x2: "21",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "8",
                x2: "21",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "8",
                x2: "21",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "3",
                x2: "3.01",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "3",
                x2: "3.01",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "3",
                x2: "3.01",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLoader;
impl IconShape for FiLoader {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "6",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "18",
                y2: "22",
            }
            line {
                x1: "4.93",
                x2: "7.76",
                y1: "4.93",
                y2: "7.76",
            }
            line {
                x1: "16.24",
                x2: "19.07",
                y1: "16.24",
                y2: "19.07",
            }
            line {
                x1: "2",
                x2: "6",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "18",
                x2: "22",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "4.93",
                x2: "7.76",
                y1: "19.07",
                y2: "16.24",
            }
            line {
                x1: "16.24",
                x2: "19.07",
                y1: "7.76",
                y2: "4.93",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLock;
impl IconShape for FiLock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "11",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "11",
            }
            path {
                d: "M7 11V7a5 5 0 0 1 10 0v4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLogIn;
impl IconShape for FiLogIn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4",
            }
            polyline {
                points: "10 17 15 12 10 7",
            }
            line {
                x1: "15",
                x2: "3",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiLogOut;
impl IconShape for FiLogOut {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4",
            }
            polyline {
                points: "16 17 21 12 16 7",
            }
            line {
                x1: "21",
                x2: "9",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMail;
impl IconShape for FiMail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z",
            }
            polyline {
                points: "22,6 12,13 2,6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMapPin;
impl IconShape for FiMapPin {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z",
            }
            circle {
                cx: "12",
                cy: "10",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMap;
impl IconShape for FiMap {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6",
            }
            line {
                x1: "8",
                x2: "8",
                y1: "2",
                y2: "18",
            }
            line {
                x1: "16",
                x2: "16",
                y1: "6",
                y2: "22",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMaximize2;
impl IconShape for FiMaximize2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "15 3 21 3 21 9",
            }
            polyline {
                points: "9 21 3 21 3 15",
            }
            line {
                x1: "21",
                x2: "14",
                y1: "3",
                y2: "10",
            }
            line {
                x1: "3",
                x2: "10",
                y1: "21",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMaximize;
impl IconShape for FiMaximize {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMeh;
impl IconShape for FiMeh {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "15",
                y2: "15",
            }
            line {
                x1: "9",
                x2: "9.01",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "15",
                x2: "15.01",
                y1: "9",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMenu;
impl IconShape for FiMenu {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "3",
                x2: "21",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "3",
                x2: "21",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "3",
                x2: "21",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMessageCircle;
impl IconShape for FiMessageCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMessageSquare;
impl IconShape for FiMessageSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMicOff;
impl IconShape for FiMicOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
            path {
                d: "M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6",
            }
            path {
                d: "M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "19",
                y2: "23",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "23",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMic;
impl IconShape for FiMic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z",
            }
            path {
                d: "M19 10v2a7 7 0 0 1-14 0v-2",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "19",
                y2: "23",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "23",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMinimize2;
impl IconShape for FiMinimize2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "4 14 10 14 10 20",
            }
            polyline {
                points: "20 10 14 10 14 4",
            }
            line {
                x1: "14",
                x2: "21",
                y1: "10",
                y2: "3",
            }
            line {
                x1: "3",
                x2: "10",
                y1: "21",
                y2: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMinimize;
impl IconShape for FiMinimize {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMinusCircle;
impl IconShape for FiMinusCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMinusSquare;
impl IconShape for FiMinusSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMinus;
impl IconShape for FiMinus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "5",
                x2: "19",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMonitor;
impl IconShape for FiMonitor {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "14",
                rx: "2",
                ry: "2",
                width: "20",
                x: "2",
                y: "3",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "21",
                y2: "21",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "17",
                y2: "21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMoon;
impl IconShape for FiMoon {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMoreHorizontal;
impl IconShape for FiMoreHorizontal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "1",
            }
            circle {
                cx: "19",
                cy: "12",
                r: "1",
            }
            circle {
                cx: "5",
                cy: "12",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMoreVertical;
impl IconShape for FiMoreVertical {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "1",
            }
            circle {
                cx: "12",
                cy: "5",
                r: "1",
            }
            circle {
                cx: "12",
                cy: "19",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMousePointer;
impl IconShape for FiMousePointer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z",
            }
            path {
                d: "M13 13l6 6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMove;
impl IconShape for FiMove {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "5 9 2 12 5 15",
            }
            polyline {
                points: "9 5 12 2 15 5",
            }
            polyline {
                points: "15 19 12 22 9 19",
            }
            polyline {
                points: "19 9 22 12 19 15",
            }
            line {
                x1: "2",
                x2: "22",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "22",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiMusic;
impl IconShape for FiMusic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 18V5l12-2v13",
            }
            circle {
                cx: "6",
                cy: "18",
                r: "3",
            }
            circle {
                cx: "18",
                cy: "16",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiNavigation2;
impl IconShape for FiNavigation2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 19 21 12 17 5 21 12 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiNavigation;
impl IconShape for FiNavigation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "3 11 22 2 13 21 11 13 3 11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiOctagon;
impl IconShape for FiOctagon {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPackage;
impl IconShape for FiPackage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "16.5",
                x2: "7.5",
                y1: "9.4",
                y2: "4.21",
            }
            path {
                d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z",
            }
            polyline {
                points: "3.27 6.96 12 12.01 20.73 6.96",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "22.08",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPaperclip;
impl IconShape for FiPaperclip {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPauseCircle;
impl IconShape for FiPauseCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "10",
                x2: "10",
                y1: "15",
                y2: "9",
            }
            line {
                x1: "14",
                x2: "14",
                y1: "15",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPause;
impl IconShape for FiPause {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "16",
                width: "4",
                x: "6",
                y: "4",
            }
            rect {
                height: "16",
                width: "4",
                x: "14",
                y: "4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPenTool;
impl IconShape for FiPenTool {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 19l7-7 3 3-7 7-3-3z",
            }
            path {
                d: "M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z",
            }
            path {
                d: "M2 2l7.586 7.586",
            }
            circle {
                cx: "11",
                cy: "11",
                r: "2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPercent;
impl IconShape for FiPercent {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "19",
                x2: "5",
                y1: "5",
                y2: "19",
            }
            circle {
                cx: "6.5",
                cy: "6.5",
                r: "2.5",
            }
            circle {
                cx: "17.5",
                cy: "17.5",
                r: "2.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneCall;
impl IconShape for FiPhoneCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneForwarded;
impl IconShape for FiPhoneForwarded {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "19 1 23 5 19 9",
            }
            line {
                x1: "15",
                x2: "23",
                y1: "5",
                y2: "5",
            }
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneIncoming;
impl IconShape for FiPhoneIncoming {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "16 2 16 8 22 8",
            }
            line {
                x1: "23",
                x2: "16",
                y1: "1",
                y2: "8",
            }
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneMissed;
impl IconShape for FiPhoneMissed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "23",
                x2: "17",
                y1: "1",
                y2: "7",
            }
            line {
                x1: "17",
                x2: "23",
                y1: "1",
                y2: "7",
            }
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneOff;
impl IconShape for FiPhoneOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91",
            }
            line {
                x1: "23",
                x2: "1",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhoneOutgoing;
impl IconShape for FiPhoneOutgoing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "23 7 23 1 17 1",
            }
            line {
                x1: "16",
                x2: "23",
                y1: "8",
                y2: "1",
            }
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPhone;
impl IconShape for FiPhone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPieChart;
impl IconShape for FiPieChart {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.21 15.89A10 10 0 1 1 8 2.83",
            }
            path {
                d: "M22 12A10 10 0 0 0 12 2v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPlayCircle;
impl IconShape for FiPlayCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            polygon {
                points: "10 8 16 12 10 16 10 8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPlay;
impl IconShape for FiPlay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "5 3 19 12 5 21 5 3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPlusCircle;
impl IconShape for FiPlusCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "16",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPlusSquare;
impl IconShape for FiPlusSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "16",
            }
            line {
                x1: "8",
                x2: "16",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPlus;
impl IconShape for FiPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "12",
                x2: "12",
                y1: "5",
                y2: "19",
            }
            line {
                x1: "5",
                x2: "19",
                y1: "12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPocket;
impl IconShape for FiPocket {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z",
            }
            polyline {
                points: "8 10 12 14 16 10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPower;
impl IconShape for FiPower {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.36 6.64a9 9 0 1 1-12.73 0",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiPrinter;
impl IconShape for FiPrinter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "6 9 6 2 18 2 18 9",
            }
            path {
                d: "M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2",
            }
            rect {
                height: "8",
                width: "12",
                x: "6",
                y: "14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRadio;
impl IconShape for FiRadio {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "2",
            }
            path {
                d: "M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRefreshCcw;
impl IconShape for FiRefreshCcw {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "1 4 1 10 7 10",
            }
            polyline {
                points: "23 20 23 14 17 14",
            }
            path {
                d: "M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRefreshCw;
impl IconShape for FiRefreshCw {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "23 4 23 10 17 10",
            }
            polyline {
                points: "1 20 1 14 7 14",
            }
            path {
                d: "M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRepeat;
impl IconShape for FiRepeat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "17 1 21 5 17 9",
            }
            path {
                d: "M3 11V9a4 4 0 0 1 4-4h14",
            }
            polyline {
                points: "7 23 3 19 7 15",
            }
            path {
                d: "M21 13v2a4 4 0 0 1-4 4H3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRewind;
impl IconShape for FiRewind {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 19 2 12 11 5 11 19",
            }
            polygon {
                points: "22 19 13 12 22 5 22 19",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRotateCcw;
impl IconShape for FiRotateCcw {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "1 4 1 10 7 10",
            }
            path {
                d: "M3.51 15a9 9 0 1 0 2.13-9.36L1 10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRotateCw;
impl IconShape for FiRotateCw {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "23 4 23 10 17 10",
            }
            path {
                d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiRss;
impl IconShape for FiRss {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 11a9 9 0 0 1 9 9",
            }
            path {
                d: "M4 4a16 16 0 0 1 16 16",
            }
            circle {
                cx: "5",
                cy: "19",
                r: "1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSave;
impl IconShape for FiSave {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z",
            }
            polyline {
                points: "17 21 17 13 7 13 7 21",
            }
            polyline {
                points: "7 3 7 8 15 8",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiScissors;
impl IconShape for FiScissors {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "6",
                cy: "6",
                r: "3",
            }
            circle {
                cx: "6",
                cy: "18",
                r: "3",
            }
            line {
                x1: "20",
                x2: "8.12",
                y1: "4",
                y2: "15.88",
            }
            line {
                x1: "14.47",
                x2: "20",
                y1: "14.48",
                y2: "20",
            }
            line {
                x1: "8.12",
                x2: "12",
                y1: "8.12",
                y2: "12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSearch;
impl IconShape for FiSearch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "11",
                cy: "11",
                r: "8",
            }
            line {
                x1: "21",
                x2: "16.65",
                y1: "21",
                y2: "16.65",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSend;
impl IconShape for FiSend {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "22",
                x2: "11",
                y1: "2",
                y2: "13",
            }
            polygon {
                points: "22 2 15 22 11 13 2 9 22 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiServer;
impl IconShape for FiServer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "8",
                rx: "2",
                ry: "2",
                width: "20",
                x: "2",
                y: "2",
            }
            rect {
                height: "8",
                rx: "2",
                ry: "2",
                width: "20",
                x: "2",
                y: "14",
            }
            line {
                x1: "6",
                x2: "6.01",
                y1: "6",
                y2: "6",
            }
            line {
                x1: "6",
                x2: "6.01",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSettings;
impl IconShape for FiSettings {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "3",
            }
            path {
                d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShare2;
impl IconShape for FiShare2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "18",
                cy: "5",
                r: "3",
            }
            circle {
                cx: "6",
                cy: "12",
                r: "3",
            }
            circle {
                cx: "18",
                cy: "19",
                r: "3",
            }
            line {
                x1: "8.59",
                x2: "15.42",
                y1: "13.51",
                y2: "17.49",
            }
            line {
                x1: "15.41",
                x2: "8.59",
                y1: "6.51",
                y2: "10.49",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShare;
impl IconShape for FiShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8",
            }
            polyline {
                points: "16 6 12 2 8 6",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShieldOff;
impl IconShape for FiShieldOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18",
            }
            path {
                d: "M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShield;
impl IconShape for FiShield {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShoppingBag;
impl IconShape for FiShoppingBag {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z",
            }
            line {
                x1: "3",
                x2: "21",
                y1: "6",
                y2: "6",
            }
            path {
                d: "M16 10a4 4 0 0 1-8 0",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShoppingCart;
impl IconShape for FiShoppingCart {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "9",
                cy: "21",
                r: "1",
            }
            circle {
                cx: "20",
                cy: "21",
                r: "1",
            }
            path {
                d: "M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiShuffle;
impl IconShape for FiShuffle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "16 3 21 3 21 8",
            }
            line {
                x1: "4",
                x2: "21",
                y1: "20",
                y2: "3",
            }
            polyline {
                points: "21 16 21 21 16 21",
            }
            line {
                x1: "15",
                x2: "21",
                y1: "15",
                y2: "21",
            }
            line {
                x1: "4",
                x2: "9",
                y1: "4",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSidebar;
impl IconShape for FiSidebar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "9",
                x2: "9",
                y1: "3",
                y2: "21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSkipBack;
impl IconShape for FiSkipBack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "19 20 9 12 19 4 19 20",
            }
            line {
                x1: "5",
                x2: "5",
                y1: "19",
                y2: "5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSkipForward;
impl IconShape for FiSkipForward {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "5 4 15 12 5 20 5 4",
            }
            line {
                x1: "19",
                x2: "19",
                y1: "5",
                y2: "19",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSlack;
impl IconShape for FiSlack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z",
            }
            path {
                d: "M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
            path {
                d: "M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z",
            }
            path {
                d: "M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z",
            }
            path {
                d: "M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z",
            }
            path {
                d: "M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z",
            }
            path {
                d: "M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z",
            }
            path {
                d: "M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSlash;
impl IconShape for FiSlash {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "4.93",
                x2: "19.07",
                y1: "4.93",
                y2: "19.07",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSliders;
impl IconShape for FiSliders {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "4",
                x2: "4",
                y1: "21",
                y2: "14",
            }
            line {
                x1: "4",
                x2: "4",
                y1: "10",
                y2: "3",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "21",
                y2: "12",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "8",
                y2: "3",
            }
            line {
                x1: "20",
                x2: "20",
                y1: "21",
                y2: "16",
            }
            line {
                x1: "20",
                x2: "20",
                y1: "12",
                y2: "3",
            }
            line {
                x1: "1",
                x2: "7",
                y1: "14",
                y2: "14",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "8",
                y2: "8",
            }
            line {
                x1: "17",
                x2: "23",
                y1: "16",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSmartphone;
impl IconShape for FiSmartphone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "20",
                rx: "2",
                ry: "2",
                width: "14",
                x: "5",
                y: "2",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSmile;
impl IconShape for FiSmile {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            path {
                d: "M8 14s1.5 2 4 2 4-2 4-2",
            }
            line {
                x1: "9",
                x2: "9.01",
                y1: "9",
                y2: "9",
            }
            line {
                x1: "15",
                x2: "15.01",
                y1: "9",
                y2: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSpeaker;
impl IconShape for FiSpeaker {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "20",
                rx: "2",
                ry: "2",
                width: "16",
                x: "4",
                y: "2",
            }
            circle {
                cx: "12",
                cy: "14",
                r: "4",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "6",
                y2: "6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSquare;
impl IconShape for FiSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiStar;
impl IconShape for FiStar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiStopCircle;
impl IconShape for FiStopCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            rect {
                height: "6",
                width: "6",
                x: "9",
                y: "9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSun;
impl IconShape for FiSun {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "5",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "1",
                y2: "3",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "21",
                y2: "23",
            }
            line {
                x1: "4.22",
                x2: "5.64",
                y1: "4.22",
                y2: "5.64",
            }
            line {
                x1: "18.36",
                x2: "19.78",
                y1: "18.36",
                y2: "19.78",
            }
            line {
                x1: "1",
                x2: "3",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "21",
                x2: "23",
                y1: "12",
                y2: "12",
            }
            line {
                x1: "4.22",
                x2: "5.64",
                y1: "19.78",
                y2: "18.36",
            }
            line {
                x1: "18.36",
                x2: "19.78",
                y1: "5.64",
                y2: "4.22",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSunrise;
impl IconShape for FiSunrise {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 18a5 5 0 0 0-10 0",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "2",
                y2: "9",
            }
            line {
                x1: "4.22",
                x2: "5.64",
                y1: "10.22",
                y2: "11.64",
            }
            line {
                x1: "1",
                x2: "3",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "21",
                x2: "23",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "18.36",
                x2: "19.78",
                y1: "11.64",
                y2: "10.22",
            }
            line {
                x1: "23",
                x2: "1",
                y1: "22",
                y2: "22",
            }
            polyline {
                points: "8 6 12 2 16 6",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiSunset;
impl IconShape for FiSunset {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 18a5 5 0 0 0-10 0",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "9",
                y2: "2",
            }
            line {
                x1: "4.22",
                x2: "5.64",
                y1: "10.22",
                y2: "11.64",
            }
            line {
                x1: "1",
                x2: "3",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "21",
                x2: "23",
                y1: "18",
                y2: "18",
            }
            line {
                x1: "18.36",
                x2: "19.78",
                y1: "11.64",
                y2: "10.22",
            }
            line {
                x1: "23",
                x2: "1",
                y1: "22",
                y2: "22",
            }
            polyline {
                points: "16 5 12 9 8 5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTable;
impl IconShape for FiTable {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTablet;
impl IconShape for FiTablet {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "20",
                rx: "2",
                ry: "2",
                width: "16",
                x: "4",
                y: "2",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "18",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTag;
impl IconShape for FiTag {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z",
            }
            line {
                x1: "7",
                x2: "7.01",
                y1: "7",
                y2: "7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTarget;
impl IconShape for FiTarget {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "6",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTerminal;
impl IconShape for FiTerminal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "4 17 10 11 4 5",
            }
            line {
                x1: "12",
                x2: "20",
                y1: "19",
                y2: "19",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiThermometer;
impl IconShape for FiThermometer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiThumbsDown;
impl IconShape for FiThumbsDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiThumbsUp;
impl IconShape for FiThumbsUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiToggleLeft;
impl IconShape for FiToggleLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "14",
                rx: "7",
                ry: "7",
                width: "22",
                x: "1",
                y: "5",
            }
            circle {
                cx: "8",
                cy: "12",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiToggleRight;
impl IconShape for FiToggleRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "14",
                rx: "7",
                ry: "7",
                width: "22",
                x: "1",
                y: "5",
            }
            circle {
                cx: "16",
                cy: "12",
                r: "3",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTool;
impl IconShape for FiTool {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTrash2;
impl IconShape for FiTrash2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "3 6 5 6 21 6",
            }
            path {
                d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2",
            }
            line {
                x1: "10",
                x2: "10",
                y1: "11",
                y2: "17",
            }
            line {
                x1: "14",
                x2: "14",
                y1: "11",
                y2: "17",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTrash;
impl IconShape for FiTrash {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "3 6 5 6 21 6",
            }
            path {
                d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTrello;
impl IconShape for FiTrello {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            rect {
                height: "9",
                width: "3",
                x: "7",
                y: "7",
            }
            rect {
                height: "5",
                width: "3",
                x: "14",
                y: "7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTrendingDown;
impl IconShape for FiTrendingDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "23 18 13.5 8.5 8.5 13.5 1 6",
            }
            polyline {
                points: "17 18 23 18 23 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTrendingUp;
impl IconShape for FiTrendingUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "23 6 13.5 15.5 8.5 10.5 1 18",
            }
            polyline {
                points: "17 6 23 6 23 12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTriangle;
impl IconShape for FiTriangle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTruck;
impl IconShape for FiTruck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "13",
                width: "15",
                x: "1",
                y: "3",
            }
            polygon {
                points: "16 8 20 8 23 11 23 16 16 16 16 8",
            }
            circle {
                cx: "5.5",
                cy: "18.5",
                r: "2.5",
            }
            circle {
                cx: "18.5",
                cy: "18.5",
                r: "2.5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTv;
impl IconShape for FiTv {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "15",
                rx: "2",
                ry: "2",
                width: "20",
                x: "2",
                y: "7",
            }
            polyline {
                points: "17 2 12 7 7 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTwitch;
impl IconShape for FiTwitch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiTwitter;
impl IconShape for FiTwitter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiType;
impl IconShape for FiType {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "4 7 4 4 20 4 20 7",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "20",
                y2: "20",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "4",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUmbrella;
impl IconShape for FiUmbrella {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUnderline;
impl IconShape for FiUnderline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3",
            }
            line {
                x1: "4",
                x2: "20",
                y1: "21",
                y2: "21",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUnlock;
impl IconShape for FiUnlock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "11",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "11",
            }
            path {
                d: "M7 11V7a5 5 0 0 1 9.9-1",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUploadCloud;
impl IconShape for FiUploadCloud {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "16 16 12 12 8 16",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "12",
                y2: "21",
            }
            path {
                d: "M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3",
            }
            polyline {
                points: "16 16 12 12 8 16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUpload;
impl IconShape for FiUpload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4",
            }
            polyline {
                points: "17 8 12 3 7 8",
            }
            line {
                x1: "12",
                x2: "12",
                y1: "3",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUserCheck;
impl IconShape for FiUserCheck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "8.5",
                cy: "7",
                r: "4",
            }
            polyline {
                points: "17 11 19 13 23 9",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUserMinus;
impl IconShape for FiUserMinus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "8.5",
                cy: "7",
                r: "4",
            }
            line {
                x1: "23",
                x2: "17",
                y1: "11",
                y2: "11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUserPlus;
impl IconShape for FiUserPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "8.5",
                cy: "7",
                r: "4",
            }
            line {
                x1: "20",
                x2: "20",
                y1: "8",
                y2: "14",
            }
            line {
                x1: "23",
                x2: "17",
                y1: "11",
                y2: "11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUserX;
impl IconShape for FiUserX {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "8.5",
                cy: "7",
                r: "4",
            }
            line {
                x1: "18",
                x2: "23",
                y1: "8",
                y2: "13",
            }
            line {
                x1: "23",
                x2: "18",
                y1: "8",
                y2: "13",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUser;
impl IconShape for FiUser {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "12",
                cy: "7",
                r: "4",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiUsers;
impl IconShape for FiUsers {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2",
            }
            circle {
                cx: "9",
                cy: "7",
                r: "4",
            }
            path {
                d: "M23 21v-2a4 4 0 0 0-3-3.87",
            }
            path {
                d: "M16 3.13a4 4 0 0 1 0 7.75",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVideoOff;
impl IconShape for FiVideoOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVideo;
impl IconShape for FiVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "23 7 16 12 23 17 23 7",
            }
            rect {
                height: "14",
                rx: "2",
                ry: "2",
                width: "15",
                x: "1",
                y: "5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVoicemail;
impl IconShape for FiVoicemail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "5.5",
                cy: "11.5",
                r: "4.5",
            }
            circle {
                cx: "18.5",
                cy: "11.5",
                r: "4.5",
            }
            line {
                x1: "5.5",
                x2: "18.5",
                y1: "16",
                y2: "16",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVolume1;
impl IconShape for FiVolume1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5",
            }
            path {
                d: "M15.54 8.46a5 5 0 0 1 0 7.07",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVolume2;
impl IconShape for FiVolume2 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5",
            }
            path {
                d: "M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVolumeX;
impl IconShape for FiVolumeX {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5",
            }
            line {
                x1: "23",
                x2: "17",
                y1: "9",
                y2: "15",
            }
            line {
                x1: "17",
                x2: "23",
                y1: "9",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiVolume;
impl IconShape for FiVolume {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiWatch;
impl IconShape for FiWatch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "7",
            }
            polyline {
                points: "12 9 12 12 13.5 13.5",
            }
            path {
                d: "M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiWifiOff;
impl IconShape for FiWifiOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
            path {
                d: "M16.72 11.06A10.94 10.94 0 0 1 19 12.55",
            }
            path {
                d: "M5 12.55a10.94 10.94 0 0 1 5.17-2.39",
            }
            path {
                d: "M10.71 5.05A16 16 0 0 1 22.58 9",
            }
            path {
                d: "M1.42 9a15.91 15.91 0 0 1 4.7-2.88",
            }
            path {
                d: "M8.53 16.11a6 6 0 0 1 6.95 0",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "20",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiWifi;
impl IconShape for FiWifi {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 12.55a11 11 0 0 1 14.08 0",
            }
            path {
                d: "M1.42 9a16 16 0 0 1 21.16 0",
            }
            path {
                d: "M8.53 16.11a6 6 0 0 1 6.95 0",
            }
            line {
                x1: "12",
                x2: "12.01",
                y1: "20",
                y2: "20",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiWind;
impl IconShape for FiWind {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiXCircle;
impl IconShape for FiXCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
            line {
                x1: "15",
                x2: "9",
                y1: "9",
                y2: "15",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "9",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiXOctagon;
impl IconShape for FiXOctagon {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2",
            }
            line {
                x1: "15",
                x2: "9",
                y1: "9",
                y2: "15",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "9",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiXSquare;
impl IconShape for FiXSquare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "18",
                rx: "2",
                ry: "2",
                width: "18",
                x: "3",
                y: "3",
            }
            line {
                x1: "9",
                x2: "15",
                y1: "9",
                y2: "15",
            }
            line {
                x1: "15",
                x2: "9",
                y1: "9",
                y2: "15",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiX;
impl IconShape for FiX {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            line {
                x1: "18",
                x2: "6",
                y1: "6",
                y2: "18",
            }
            line {
                x1: "6",
                x2: "18",
                y1: "6",
                y2: "18",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiYoutube;
impl IconShape for FiYoutube {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z",
            }
            polygon {
                points: "9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiZapOff;
impl IconShape for FiZapOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "12.41 6.75 13 2 10.57 4.92",
            }
            polyline {
                points: "18.57 12.91 21 10 15.66 10",
            }
            polyline {
                points: "8 8 3 14 12 14 11 22 16 16",
            }
            line {
                x1: "1",
                x2: "23",
                y1: "1",
                y2: "23",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiZap;
impl IconShape for FiZap {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "13 2 3 14 12 14 11 22 21 10 12 10 13 2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiZoomIn;
impl IconShape for FiZoomIn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "11",
                cy: "11",
                r: "8",
            }
            line {
                x1: "21",
                x2: "16.65",
                y1: "21",
                y2: "16.65",
            }
            line {
                x1: "11",
                x2: "11",
                y1: "8",
                y2: "14",
            }
            line {
                x1: "8",
                x2: "14",
                y1: "11",
                y2: "11",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FiZoomOut;
impl IconShape for FiZoomOut {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        ("none", user_color, "2")
    }
    fn stroke_linecap(&self) -> &str {
        "round"
    }
    fn stroke_linejoin(&self) -> &str {
        "round"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            circle {
                cx: "11",
                cy: "11",
                r: "8",
            }
            line {
                x1: "21",
                x2: "16.65",
                y1: "21",
                y2: "16.65",
            }
            line {
                x1: "8",
                x2: "14",
                y1: "11",
                y2: "11",
            }
        }
    }
}
