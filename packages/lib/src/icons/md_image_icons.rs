use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MdAssistantPhoto;
impl IconShape for MdAssistantPhoto {
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
                d: "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness4;
impl IconShape for MdBrightness4 {
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
                d: "M20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM12 18c-.89 0-1.74-.2-2.5-.55C11.56 16.5 13 14.42 13 12s-1.44-4.5-3.5-5.45C10.26 6.2 11.11 6 12 6c3.31 0 6 2.69 6 6s-2.69 6-6 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoFixHigh;
impl IconShape for MdAutoFixHigh {
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
                d: "M7.5 5.6L10 7 8.6 4.5 10 2 7.5 3.4 5 2l1.4 2.5L5 7zm12 9.8L17 14l1.4 2.5L17 19l2.5-1.4L22 19l-1.4-2.5L22 14zM22 2l-2.5 1.4L17 2l1.4 2.5L17 7l2.5-1.4L22 7l-1.4-2.5zm-7.63 5.29c-.39-.39-1.02-.39-1.41 0L1.29 18.96c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0L16.7 11.05c.39-.39.39-1.02 0-1.41l-2.33-2.35zm-1.03 5.49l-2.12-2.12 2.44-2.44 2.12 2.12-2.44 2.44z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrokenImage;
impl IconShape for MdBrokenImage {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0zm21 19c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2V5c0-1.1.9-2 2-2h14c1.1 0 2 .9 2 2",
            }
            path {
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M21 5v6.59l-3-3.01-4 4.01-4-4-4 4-3-3.01V5c0-1.1.9-2 2-2h14c1.1 0 2 .9 2 2zm-3 6.42l3 3.01V19c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2v-6.58l3 2.99 4-4 4 4 4-3.99z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness3;
impl IconShape for MdBrightness3 {
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
                d: "M9 2c-1.05 0-2.05.16-3 .46 4.06 1.27 7 5.06 7 9.54 0 4.48-2.94 8.27-7 9.54.95.3 1.95.46 3 .46 5.52 0 10-4.48 10-10S14.52 2 9 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAssistant;
impl IconShape for MdAssistant {
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
                d: "M19 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5.12 10.88L12 17l-1.88-4.12L6 11l4.12-1.88L12 5l1.88 4.12L18 11l-4.12 1.88z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGridOff;
impl IconShape for MdGridOff {
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
                d: "M8 4v1.45l2 2V4h4v4h-3.45l2 2H14v1.45l2 2V10h4v4h-3.45l2 2H20v1.45l2 2V4c0-1.1-.9-2-2-2H4.55l2 2H8zm8 0h4v4h-4V4zM1.27 1.27L0 2.55l2 2V20c0 1.1.9 2 2 2h15.46l2 2 1.27-1.27L1.27 1.27zM10 12.55L11.45 14H10v-1.45zm-6-6L5.45 8H4V6.55zM8 20H4v-4h4v4zm0-6H4v-4h3.45l.55.55V14zm6 6h-4v-4h3.45l.55.54V20zm2 0v-1.46L17.46 20H16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMonochromePhotos;
impl IconShape for MdMonochromePhotos {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M-74 29h48v48h-48V29z",
            }
            path {
                d: "M20 5h-3.2L15 3H9L7.2 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 14h-8v-1c-2.8 0-5-2.2-5-5s2.2-5 5-5V7h8v12zm-3-6c0-2.8-2.2-5-5-5v1.8c1.8 0 3.2 1.4 3.2 3.2s-1.4 3.2-3.2 3.2V18c2.8 0 5-2.2 5-5zm-8.2 0c0 1.8 1.4 3.2 3.2 3.2V9.8c-1.8 0-3.2 1.4-3.2 3.2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlipCameraIos;
impl IconShape for MdFlipCameraIos {
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
                d: "M20,5h-3.17L15,3H9L7.17,5H4C2.9,5,2,5.9,2,7v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M12,18 c-2.76,0-5-2.24-5-5H5l2.5-2.5L10,13H8c0,2.21,1.79,4,4,4c0.58,0,1.13-0.13,1.62-0.35l0.74,0.74C13.65,17.76,12.86,18,12,18z M16.5,15.5L14,13h2c0-2.21-1.79-4-4-4c-0.58,0-1.13,0.13-1.62,0.35L9.64,8.62C10.35,8.24,11.14,8,12,8c2.76,0,5,2.24,5,5h2 L16.5,15.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGradient;
impl IconShape for MdGradient {
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
                d: "M11 9h2v2h-2zm-2 2h2v2H9zm4 0h2v2h-2zm2-2h2v2h-2zM7 9h2v2H7zm12-6H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 18H7v-2h2v2zm4 0h-2v-2h2v2zm4 0h-2v-2h2v2zm2-7h-2v2h2v2h-2v-2h-2v2h-2v-2h-2v2H9v-2H7v2H5v-2h2v-2H5V5h14v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSwitchCamera;
impl IconShape for MdSwitchCamera {
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
                d: "M20 4h-3.17L15 2H9L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-5 11.5V13H9v2.5L5.5 12 9 8.5V11h6V8.5l3.5 3.5-3.5 3.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRemoveRedEye;
impl IconShape for MdRemoveRedEye {
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
                d: "M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoFixOff;
impl IconShape for MdAutoFixOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M23 1l-2.5 1.4L18 1l1.4 2.5L18 6l2.5-1.4L23 6l-1.4-2.5L23 1zm-8.34 6.22l2.12 2.12-2.44 2.44.81.81 2.55-2.55c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0L11.4 8.84l.81.81 2.45-2.43zm-.78 6.65l-3.75-3.75-6.86-6.86L2 4.53l6.86 6.86-6.57 6.57c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0l6.57-6.57L19.47 22l1.27-1.27-6.86-6.86z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md23mp;
impl IconShape for Md23mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm7.5 1.5c0 .55-.45 1-1 1H13V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm-2 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdViewComfy;
impl IconShape for MdViewComfy {
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
                d: "M3 9h4V5H3v4zm0 5h4v-4H3v4zm5 0h4v-4H8v4zm5 0h4v-4h-4v4zM8 9h4V5H8v4zm5-4v4h4V5h-4zm5 9h4v-4h-4v4zM3 19h4v-4H3v4zm5 0h4v-4H8v4zm5 0h4v-4h-4v4zm5 0h4v-4h-4v4zm0-14v4h4V5h-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness2;
impl IconShape for MdBrightness2 {
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
                d: "M10 2c-1.82 0-3.53.5-5 1.35C7.99 5.08 10 8.3 10 12s-2.01 6.92-5 8.65C6.47 21.5 8.18 22 10 22c5.52 0 10-4.48 10-10S15.52 2 10 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbCloudy;
impl IconShape for MdWbCloudy {
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
                d: "M19.36 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.64-4.96z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness5;
impl IconShape for MdBrightness5 {
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
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLeakRemove;
impl IconShape for MdLeakRemove {
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
                d: "M10 3H8c0 .37-.04.72-.12 1.06l1.59 1.59C9.81 4.84 10 3.94 10 3zM3 4.27l2.84 2.84C5.03 7.67 4.06 8 3 8v2c1.61 0 3.09-.55 4.27-1.46L8.7 9.97C7.14 11.24 5.16 12 3 12v2c2.71 0 5.19-.99 7.11-2.62l2.5 2.5C10.99 15.81 10 18.29 10 21h2c0-2.16.76-4.14 2.03-5.69l1.43 1.43C14.55 17.91 14 19.39 14 21h2c0-1.06.33-2.03.89-2.84L19.73 21 21 19.73 4.27 3 3 4.27zM14 3h-2c0 1.5-.37 2.91-1.02 4.16l1.46 1.46C13.42 6.98 14 5.06 14 3zm5.94 13.12c.34-.08.69-.12 1.06-.12v-2c-.94 0-1.84.19-2.66.52l1.6 1.6zm-4.56-4.56l1.46 1.46C18.09 12.37 19.5 12 21 12v-2c-2.06 0-3.98.58-5.62 1.56z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBurstMode;
impl IconShape for MdBurstMode {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M1 5h2v14H1zm4 0h2v14H5zm17 0H10c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1zM11 17l2.5-3.15L15.29 16l2.5-3.22L21 17H11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPictureAsPdf;
impl IconShape for MdPictureAsPdf {
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
                d: "M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8.5 7.5c0 .83-.67 1.5-1.5 1.5H9v2H7.5V7H10c.83 0 1.5.67 1.5 1.5v1zm5 2c0 .83-.67 1.5-1.5 1.5h-2.5V7H15c.83 0 1.5.67 1.5 1.5v3zm4-3H19v1h1.5V11H19v2h-1.5V7h3v1.5zM9 9.5h1v-1H9v1zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm10 5.5h1v-3h-1v3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposure;
impl IconShape for MdExposure {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM6 7h5v1.5H6V7zm13 12H5L19 5v14zm-4.5-3v2H16v-2h2v-1.5h-2v-2h-1.5v2h-2V16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAudiotrack;
impl IconShape for MdAudiotrack {
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
                d: "M12 3v9.28c-.47-.17-.97-.28-1.5-.28C8.01 12 6 14.01 6 16.5S8.01 21 10.5 21c2.31 0 4.2-1.75 4.45-4H15V6h4V3h-7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterTiltShift;
impl IconShape for MdFilterTiltShift {
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
                d: "M11 4.07V2.05c-2.01.2-3.84 1-5.32 2.21L7.1 5.69c1.11-.86 2.44-1.44 3.9-1.62zm7.32.19C16.84 3.05 15.01 2.25 13 2.05v2.02c1.46.18 2.79.76 3.9 1.62l1.42-1.43zM19.93 11h2.02c-.2-2.01-1-3.84-2.21-5.32L18.31 7.1c.86 1.11 1.44 2.44 1.62 3.9zM5.69 7.1L4.26 5.68C3.05 7.16 2.25 8.99 2.05 11h2.02c.18-1.46.76-2.79 1.62-3.9zM4.07 13H2.05c.2 2.01 1 3.84 2.21 5.32l1.43-1.43c-.86-1.1-1.44-2.43-1.62-3.89zM15 12c0-1.66-1.34-3-3-3s-3 1.34-3 3 1.34 3 3 3 3-1.34 3-3zm3.31 4.9l1.43 1.43c1.21-1.48 2.01-3.32 2.21-5.32h-2.02c-.18 1.45-.76 2.78-1.62 3.89zM13 19.93v2.02c2.01-.2 3.84-1 5.32-2.21l-1.43-1.43c-1.1.86-2.43 1.44-3.89 1.62zm-7.32-.19C7.16 20.95 9 21.75 11 21.95v-2.02c-1.46-.18-2.79-.76-3.9-1.62l-1.42 1.43z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAdjust;
impl IconShape for MdAdjust {
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
                d: "M12 2C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3-8c0 1.66-1.34 3-3 3s-3-1.34-3-3 1.34-3 3-3 3 1.34 3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCameraRear;
impl IconShape for MdCameraRear {
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
                d: "M10 20H5v2h5v2l3-3-3-3v2zm4 0v2h5v-2h-5zm3-20H7C5.9 0 5 .9 5 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V2c0-1.1-.9-2-2-2zm-5 6c-1.11 0-2-.9-2-2s.89-2 1.99-2 2 .9 2 2C14 5.1 13.1 6 12 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoAwesome;
impl IconShape for MdAutoAwesome {
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
                d: "M19 9l1.25-2.75L23 5l-2.75-1.25L19 1l-1.25 2.75L15 5l2.75 1.25L19 9zm-7.5.5L9 4 6.5 9.5 1 12l5.5 2.5L9 20l2.5-5.5L17 12l-5.5-2.5zM19 15l-1.25 2.75L15 19l2.75 1.25L19 23l1.25-2.75L23 19l-2.75-1.25L19 15z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTagFaces;
impl IconShape for MdTagFaces {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMusicOff;
impl IconShape for MdMusicOff {
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
                d: "M4.27 3L3 4.27l9 9v.28c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4v-1.73L19.73 21 21 19.73 4.27 3zM14 7h4V3h-6v5.18l2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoLibrary;
impl IconShape for MdPhotoLibrary {
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
                d: "M22 16V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2zm-11-4l2.03 2.71L16 11l4 5H8l3-4zM2 6v14c0 1.1.9 2 2 2h14v-2H4V6H2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbSunny;
impl IconShape for MdWbSunny {
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
                d: "M6.76 4.84l-1.8-1.79-1.41 1.41 1.79 1.79 1.42-1.41zM4 10.5H1v2h3v-2zm9-9.95h-2V3.5h2V.55zm7.45 3.91l-1.41-1.41-1.79 1.79 1.41 1.41 1.79-1.79zm-3.21 13.7l1.79 1.8 1.41-1.41-1.8-1.79-1.4 1.4zM20 10.5v2h3v-2h-3zm-8-5c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm-1 16.95h2V19.5h-2v2.95zm-7.45-3.91l1.41 1.41 1.79-1.8-1.41-1.41-1.79 1.8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md16mp;
impl IconShape for Md16mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13.5 9H15v1.5h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm3 6c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5V7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H13zm2.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlashAuto;
impl IconShape for MdFlashAuto {
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
                d: "M3 2v12h3v9l7-12H9l4-9H3zm16 0h-2l-3.2 9h1.9l.7-2h3.2l.7 2h1.9L19 2zm-2.15 5.65L18 4l1.15 3.65h-2.3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md3mp;
impl IconShape for Md3mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H10V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCrop169;
impl IconShape for MdCrop169 {
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
                d: "M19 6H5c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 10H5V8h14v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md9mp;
impl IconShape for Md9mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11.5 6.5H13V8h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H10V10h3V9h-2c-.55 0-1-.45-1-1V6.5c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropRotate;
impl IconShape for MdCropRotate {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0zm0 0h24v24H0V0z",
            }
            path {
                d: "M7.47 21.49C4.2 19.93 1.86 16.76 1.5 13H0c.51 6.16 5.66 11 11.95 11 .23 0 .44-.02.66-.03L8.8 20.15l-1.33 1.34zM12.05 0c-.23 0-.44.02-.66.04l3.81 3.81 1.33-1.33C19.8 4.07 22.14 7.24 22.5 11H24c-.51-6.16-5.66-11-11.95-11zM16 14h2V8c0-1.11-.9-2-2-2h-6v2h6v6zm-8 2V4H6v2H4v2h2v8c0 1.1.89 2 2 2h8v2h2v-2h2v-2H8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMotionPhotosPaused;
impl IconShape for MdMotionPhotosPaused {
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
                d: "M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8 s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2C17.52,2,22,6.48,22,12z M5.5,4 C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z M11,16V8H9v8H11z M15,16V8h-2v8H15z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBlurOff;
impl IconShape for MdBlurOff {
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
                d: "M14 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm-.2 4.48l.2.02c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5l.02.2c.09.67.61 1.19 1.28 1.28zM14 3.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm-4 0c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm11 7c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM10 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm8 8c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm-4 13.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM2.5 5.27l3.78 3.78L6 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1c0-.1-.03-.19-.06-.28l2.81 2.81c-.71.11-1.25.73-1.25 1.47 0 .83.67 1.5 1.5 1.5.74 0 1.36-.54 1.47-1.25l2.81 2.81c-.09-.03-.18-.06-.28-.06-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1c0-.1-.03-.19-.06-.28l3.78 3.78L20 20.23 3.77 4 2.5 5.27zM10 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm11-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 13c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zM3 9.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm7 11c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm-3-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md18mp;
impl IconShape for Md18mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H13c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm-3 0H15V9h-1.5v1.5zm0-2.5H15V6.5h-1.5V8zm2 6H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCameraFront;
impl IconShape for MdCameraFront {
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
                d: "M10 20H5v2h5v2l3-3-3-3v2zm4 0v2h5v-2h-5zM12 8c1.1 0 2-.9 2-2s-.9-2-2-2-1.99.9-1.99 2S10.9 8 12 8zm5-8H7C5.9 0 5 .9 5 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V2c0-1.1-.9-2-2-2zM7 2h10v10.5c0-1.67-3.33-2.5-5-2.5s-5 .83-5 2.5V2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterNone;
impl IconShape for MdFilterNone {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHdrWeak;
impl IconShape for MdHdrWeak {
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
                d: "M5 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm12-2c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zm0 10c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterVintage;
impl IconShape for MdFilterVintage {
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
                d: "M18.7 12.4c-.28-.16-.57-.29-.86-.4.29-.11.58-.24.86-.4 1.92-1.11 2.99-3.12 3-5.19-1.79-1.03-4.07-1.11-6 0-.28.16-.54.35-.78.54.05-.31.08-.63.08-.95 0-2.22-1.21-4.15-3-5.19C10.21 1.85 9 3.78 9 6c0 .32.03.64.08.95-.24-.2-.5-.39-.78-.55-1.92-1.11-4.2-1.03-6 0 0 2.07 1.07 4.08 3 5.19.28.16.57.29.86.4-.29.11-.58.24-.86.4-1.92 1.11-2.99 3.12-3 5.19 1.79 1.03 4.07 1.11 6 0 .28-.16.54-.35.78-.54-.05.32-.08.64-.08.96 0 2.22 1.21 4.15 3 5.19 1.79-1.04 3-2.97 3-5.19 0-.32-.03-.64-.08-.95.24.2.5.38.78.54 1.92 1.11 4.2 1.03 6 0-.01-2.07-1.08-4.08-3-5.19zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooks6;
impl IconShape for MdLooks6 {
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
                d: "M11 15h2v-2h-2v2zm8-12H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 6h-4v2h2c1.1 0 2 .89 2 2v2c0 1.11-.9 2-2 2h-2c-1.1 0-2-.89-2-2V9c0-1.11.9-2 2-2h4v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdIso;
impl IconShape for MdIso {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM5.5 7.5h2v-2H9v2h2V9H9v2H7.5V9h-2V7.5zM19 19H5L19 5v14zm-2-2v-1.5h-5V17h5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLeakAdd;
impl IconShape for MdLeakAdd {
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
                d: "M6 3H3v3c1.66 0 3-1.34 3-3zm8 0h-2c0 4.97-4.03 9-9 9v2c6.08 0 11-4.93 11-11zm-4 0H8c0 2.76-2.24 5-5 5v2c3.87 0 7-3.13 7-7zm0 18h2c0-4.97 4.03-9 9-9v-2c-6.07 0-11 4.93-11 11zm8 0h3v-3c-1.66 0-3 1.34-3 3zm-4 0h2c0-2.76 2.24-5 5-5v-2c-3.87 0-7 3.13-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md12mp;
impl IconShape for Md12mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zM15.5 9h-2v1h3v1.5H12V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm0 5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdShutterSpeed;
impl IconShape for MdShutterSpeed {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M15 1H9v2h6V1zm4.03 6.39l1.42-1.42c-.43-.51-.9-.99-1.41-1.41l-1.42 1.42C16.07 4.74 14.12 4 12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9 9-4.03 9-9c0-2.12-.74-4.07-1.97-5.61zM12 20c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm-.32-5H6.35c.57 1.62 1.82 2.92 3.41 3.56l-.11-.06 2.03-3.5zm5.97-4c-.57-1.6-1.78-2.89-3.34-3.54L12.26 11h5.39zm-7.04 7.83c.45.11.91.17 1.39.17 1.34 0 2.57-.45 3.57-1.19l-2.11-3.9-2.85 4.92zM7.55 8.99C6.59 10.05 6 11.46 6 13c0 .34.04.67.09 1h4.72L7.55 8.99zm8.79 8.14C17.37 16.06 18 14.6 18 13c0-.34-.04-.67-.09-1h-4.34l2.77 5.13zm-3.01-9.98C12.9 7.06 12.46 7 12 7c-1.4 0-2.69.49-3.71 1.29l2.32 3.56 2.72-4.7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md7mp;
impl IconShape for Md7mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-2.5-7h-1.75L12.62 7H10V5.5h3.5c.67 0 1.15.65.96 1.29L13 11.5zm2.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterCenterFocus;
impl IconShape for MdFilterCenterFocus {
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
                d: "M5 15H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zM12 9c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposureNeg1;
impl IconShape for MdExposureNeg1 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M4 11v2h8v-2H4zm15 7h-2V7.38L14 8.4V6.7L18.7 5h.3v13z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposurePlus1;
impl IconShape for MdExposurePlus1 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M10 7H8v4H4v2h4v4h2v-4h4v-2h-4V7zm10 11h-2V7.38L15 8.4V6.7L19.7 5h.3v13z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoFilter;
impl IconShape for MdPhotoFilter {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19.02 10v9H5V5h9V3H5.02c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-9h-2zM17 10l.94-2.06L20 7l-2.06-.94L17 4l-.94 2.06L14 7l2.06.94zm-3.75.75L12 8l-1.25 2.75L8 12l2.75 1.25L12 16l1.25-2.75L16 12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTimer3;
impl IconShape for MdTimer3 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M11.61 12.97c-.16-.24-.36-.46-.62-.65-.25-.19-.56-.35-.93-.48.3-.14.57-.3.8-.5.23-.2.42-.41.57-.64.15-.23.27-.46.34-.71.08-.24.11-.49.11-.73 0-.55-.09-1.04-.28-1.46-.18-.42-.44-.77-.78-1.06-.33-.28-.73-.5-1.2-.64-.45-.13-.97-.2-1.53-.2-.55 0-1.06.08-1.52.24-.47.17-.87.4-1.2.69-.33.29-.6.63-.78 1.03-.2.39-.29.83-.29 1.29h1.98c0-.26.05-.49.14-.69.09-.2.22-.38.38-.52.17-.14.36-.25.58-.33.22-.08.46-.12.73-.12.61 0 1.06.16 1.36.47.3.31.44.75.44 1.32 0 .27-.04.52-.12.74-.08.22-.21.41-.38.57-.17.16-.38.28-.63.37-.25.09-.55.13-.89.13H6.72v1.57H7.9c.34 0 .64.04.91.11.27.08.5.19.69.35.19.16.34.36.44.61.1.24.16.54.16.87 0 .62-.18 1.09-.53 1.42-.35.33-.84.49-1.45.49-.29 0-.56-.04-.8-.13-.24-.08-.44-.2-.61-.36-.17-.16-.3-.34-.39-.56-.09-.22-.14-.46-.14-.72H4.19c0 .55.11 1.03.32 1.45.21.42.5.77.86 1.05s.77.49 1.24.63.96.21 1.48.21c.57 0 1.09-.08 1.58-.23.49-.15.91-.38 1.26-.68.36-.3.64-.66.84-1.1.2-.43.3-.93.3-1.48 0-.29-.04-.58-.11-.86-.08-.25-.19-.51-.35-.76zm9.26 1.4c-.14-.28-.35-.53-.63-.74-.28-.21-.61-.39-1.01-.53s-.85-.27-1.35-.38c-.35-.07-.64-.15-.87-.23-.23-.08-.41-.16-.55-.25-.14-.09-.23-.19-.28-.3-.05-.11-.08-.24-.08-.39s.03-.28.09-.41c.06-.13.15-.25.27-.34.12-.1.27-.18.45-.24s.4-.09.64-.09c.25 0 .47.04.66.11.19.07.35.17.48.29.13.12.22.26.29.42.06.16.1.32.1.49h1.95c0-.39-.08-.75-.24-1.09-.16-.34-.39-.63-.69-.88-.3-.25-.66-.44-1.09-.59-.43-.15-.92-.22-1.46-.22-.51 0-.98.07-1.39.21-.41.14-.77.33-1.06.57-.29.24-.51.52-.67.84-.16.32-.23.65-.23 1.01s.08.68.23.96c.15.28.37.52.64.73.27.21.6.38.98.53.38.14.81.26 1.27.36.39.08.71.17.95.26s.43.19.57.29c.13.1.22.22.27.34.05.12.07.25.07.39 0 .32-.13.57-.4.77-.27.2-.66.29-1.17.29-.22 0-.43-.02-.64-.08-.21-.05-.4-.13-.56-.24-.17-.11-.3-.26-.41-.44-.11-.18-.17-.41-.18-.67h-1.89c0 .36.08.71.24 1.05.16.34.39.65.7.93.31.27.69.49 1.15.66.46.17.98.25 1.58.25.53 0 1.01-.06 1.44-.19.43-.13.8-.31 1.11-.54.31-.23.54-.51.71-.83.17-.32.25-.67.25-1.06-.02-.4-.09-.74-.24-1.02z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdImageNotSupported;
impl IconShape for MdImageNotSupported {
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
                d: "M21.9,21.9l-8.49-8.49l0,0L3.59,3.59l0,0L2.1,2.1L0.69,3.51L3,5.83V19c0,1.1,0.9,2,2,2h13.17l2.31,2.31L21.9,21.9z M5,18 l3.5-4.5l2.5,3.01L12.17,15l3,3H5z M21,18.17L5.83,3H19c1.1,0,2,0.9,2,2V18.17z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoCameraBack;
impl IconShape for MdPhotoCameraBack {
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
                d: "M20 5c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V7c0-1.1.9-2 2-2h3.17L9 3h6l1.83 2H20zm0 14V7H4v12h16zm-6-7l-3 3.72L9 13l-3 4h12l-4-5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterFrames;
impl IconShape for MdFilterFrames {
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
                d: "M20 4h-4l-4-4-4 4H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H4V6h4.52l3.52-3.5L15.52 6H20v14zM18 8H6v10h12",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDehaze;
impl IconShape for MdDehaze {
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
                d: "M2 15.5v2h20v-2H2zm0-5v2h20v-2H2zm0-5v2h20v-2H2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCrop54;
impl IconShape for MdCrop54 {
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
                d: "M19 5H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 12H5V7h14v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMovieCreation;
impl IconShape for MdMovieCreation {
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
                d: "M18 4l2 4h-3l-2-4h-2l2 4h-3l-2-4H8l2 4H7L5 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4h-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdControlPoint;
impl IconShape for MdControlPoint {
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
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10 10-4.49 10-10S17.51 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterDrama;
impl IconShape for MdFilterDrama {
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.61 5.64 5.36 8.04 2.35 8.36 0 10.9 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4h2c0-2.76-1.86-5.08-4.4-5.78C8.61 6.88 10.2 6 12 6c3.03 0 5.5 2.47 5.5 5.5v.5H19c1.65 0 3 1.35 3 3s-1.35 3-3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md24mp;
impl IconShape for Md24mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm8.5 1h-1v1.5H16V10h-3V5.5h1.5v3H16v-3h1.5v3h1V10zm-3 4H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaPhotosphere;
impl IconShape for MdPanoramaPhotosphere {
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
                d: "M21.4 11.32v2.93c-.1.05-2.17.85-3.33 1.17-.94.26-3.84.73-6.07.73-3.7 0-7-.7-9.16-1.8-.08-.04-.16-.06-.24-.1V9.76c6.02-2.84 12.6-2.92 18.8 0v1.56zm-9.39 8.88c-2.5 0-4.87-1.15-6.41-3.12 4.19 1.22 8.57 1.23 12.82-.01-1.54 1.97-3.9 3.13-6.41 3.13zM12 3.8c2.6 0 4.91 1.23 6.41 3.12-4.1-1.19-8.48-1.26-12.83.01C7.08 5.03 9.4 3.8 12 3.8zm10.49 4.71c-.47-.23-.93-.44-1.4-.64C19.52 4.41 16.05 2 12 2S4.47 4.41 2.9 7.88c-.47.2-.93.41-1.4.63-.31.15-.5.48-.5.83v5.32c0 .35.19.68.51.83.47.23.93.44 1.39.64 3.55 7.83 14.65 7.82 18.2 0 .47-.2.93-.41 1.39-.63.31-.17.51-.49.51-.84V9.34c0-.35-.19-.68-.51-.83z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCamera;
impl IconShape for MdCamera {
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
                d: "M9.4 10.5l4.77-8.26C13.47 2.09 12.75 2 12 2c-2.4 0-4.6.85-6.32 2.25l3.66 6.35.06-.1zM21.54 9c-.92-2.92-3.15-5.26-6-6.34L11.88 9h9.66zm.26 1h-7.49l.29.5 4.76 8.25C21 16.97 22 14.61 22 12c0-.69-.07-1.35-.2-2zM8.54 12l-3.9-6.75C3.01 7.03 2 9.39 2 12c0 .69.07 1.35.2 2h7.49l-1.15-2zm-6.08 3c.92 2.92 3.15 5.26 6 6.34L12.12 15H2.46zm11.27 0l-3.9 6.76c.7.15 1.42.24 2.17.24 2.4 0 4.6-.85 6.32-2.25l-3.66-6.35-.93 1.6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbAuto;
impl IconShape for MdWbAuto {
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
                d: "M6.85 12.65h2.3L8 9l-1.15 3.65zM22 7l-1.2 6.29L19.3 7h-1.6l-1.49 6.29L15 7h-.76C12.77 5.17 10.53 4 8 4c-4.42 0-8 3.58-8 8s3.58 8 8 8c3.13 0 5.84-1.81 7.15-4.43l.1.43H17l1.5-6.1L20 16h1.75l2.05-9H22zm-11.7 9l-.7-2H6.4l-.7 2H3.8L7 7h2l3.2 9h-1.9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter5;
impl IconShape for MdFilter5 {
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
                d: "M21 1H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm14 8v-2c0-1.11-.9-2-2-2h-2V7h4V5h-6v6h4v2h-4v2h4c1.1 0 2-.89 2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter2;
impl IconShape for MdFilter2 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-4-4h-4v-2h2c1.1 0 2-.89 2-2V7c0-1.11-.9-2-2-2h-4v2h4v2h-2c-1.1 0-2 .89-2 2v4h6v-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFaceRetouchingNatural;
impl IconShape for MdFaceRetouchingNatural {
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

#[derive(Copy, Clone, Debug)]
pub struct MdFilterBAndW;
impl IconShape for MdFilterBAndW {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16l-7-8v8H5l7-8V5h7v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaHorizontalSelect;
impl IconShape for MdPanoramaHorizontalSelect {
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
                d: "M21.43 4c-.1 0-.2.02-.31.06C18.18 5.16 15.09 5.7 12 5.7s-6.18-.55-9.12-1.64C2.77 4.02 2.66 4 2.57 4c-.34 0-.57.23-.57.63v14.75c0 .39.23.62.57.62.1 0 .2-.02.31-.06 2.94-1.1 6.03-1.64 9.12-1.64s6.18.55 9.12 1.64c.11.04.21.06.31.06.33 0 .57-.23.57-.63V4.63c0-.4-.24-.63-.57-.63z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHdrEnhancedSelect;
impl IconShape for MdHdrEnhancedSelect {
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
                d: "M12 2c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6 2.69-6 6-6zm0 2C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm1 7h-2V9H9V7h2V5h2v2h2v2h-2v2zm11 9h-2v2h-1.5v-2h-2v-1.5h2v-2H22v2h2V20zm-6-1.5c0 .6-.4 1.1-.9 1.4L18 22h-1.5l-.9-2h-1.1v2H13v-6h3.5c.8 0 1.5.7 1.5 1.5v1zm-1.5 0v-1h-2v1h2zm-13-.5v-2H5v6H3.5v-2.5h-2V22H0v-6h1.5v2h2zm6.5-2c.8 0 1.5.7 1.5 1.5v3c0 .8-.7 1.5-1.5 1.5H6.5v-6H10zm0 4.5v-3H8v3h2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdControlPointDuplicate;
impl IconShape for MdControlPointDuplicate {
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
                d: "M16 8h-2v3h-3v2h3v3h2v-3h3v-2h-3zM2 12c0-2.79 1.64-5.2 4.01-6.32V3.52C2.52 4.76 0 8.09 0 12s2.52 7.24 6.01 8.48v-2.16C3.64 17.2 2 14.79 2 12zm13-9c-4.96 0-9 4.04-9 9s4.04 9 9 9 9-4.04 9-9-4.04-9-9-9zm0 16c-3.86 0-7-3.14-7-7s3.14-7 7-7 7 3.14 7 7-3.14 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter3;
impl IconShape for MdFilter3 {
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
                d: "M21 1H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm14 8v-1.5c0-.83-.67-1.5-1.5-1.5.83 0 1.5-.67 1.5-1.5V7c0-1.11-.9-2-2-2h-4v2h4v2h-2v2h2v2h-4v2h4c1.1 0 2-.89 2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTimer;
impl IconShape for MdTimer {
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
                d: "M15 1H9v2h6V1zm-4 13h2V8h-2v6zm8.03-6.61l1.42-1.42c-.43-.51-.9-.99-1.41-1.41l-1.42 1.42C16.07 4.74 14.12 4 12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9 9-4.03 9-9c0-2.12-.74-4.07-1.97-5.61zM12 20c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbIncandescent;
impl IconShape for MdWbIncandescent {
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
                d: "M3.55 18.54l1.41 1.41 1.79-1.8-1.41-1.41-1.79 1.8zM11 22.45h2V19.5h-2v2.95zM4 10.5H1v2h3v-2zm11-4.19V1.5H9v4.81C7.21 7.35 6 9.28 6 11.5c0 3.31 2.69 6 6 6s6-2.69 6-6c0-2.22-1.21-4.15-3-5.19zm5 4.19v2h3v-2h-3zm-2.76 7.66l1.79 1.8 1.41-1.41-1.8-1.79-1.4 1.4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStraighten;
impl IconShape for MdStraighten {
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
                d: "M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 10H3V8h2v4h2V8h2v4h2V8h2v4h2V8h2v4h2V8h2v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md20mp;
impl IconShape for Md20mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14.5 7H16v3h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm2-8c0 .55-.45 1-1 1H14c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm5.5 5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter4;
impl IconShape for MdFilter4 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm12 10h2V5h-2v4h-2V5h-2v6h4v4zm6-14H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaFishEye;
impl IconShape for MdPanoramaFishEye {
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
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCrop;
impl IconShape for MdCrop {
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
                d: "M17 15h2V7c0-1.1-.9-2-2-2H9v2h8v8zM7 17V1H5v4H1v2h4v10c0 1.1.9 2 2 2h10v4h2v-4h4v-2H7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTimelapse;
impl IconShape for MdTimelapse {
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
                d: "M16.24 7.76C15.07 6.59 13.54 6 12 6v6l-4.24 4.24c2.34 2.34 6.14 2.34 8.49 0 2.34-2.34 2.34-6.14-.01-8.48zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md15mp;
impl IconShape for Md15mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zM16.5 7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H12V10h3V9h-3V5.5h4.5V7zm-1 7H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdColorLens;
impl IconShape for MdColorLens {
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
                d: "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPortrait;
impl IconShape for MdPortrait {
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
                d: "M12 12.25c1.24 0 2.25-1.01 2.25-2.25S13.24 7.75 12 7.75 9.75 8.76 9.75 10s1.01 2.25 2.25 2.25zm4.5 4c0-1.5-3-2.25-4.5-2.25s-4.5.75-4.5 2.25V17h9v-.75zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlashOff;
impl IconShape for MdFlashOff {
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
                d: "M3.27 3L2 4.27l5 5V13h3v9l3.58-6.14L17.73 20 19 18.73 3.27 3zM17 10h-4l4-8H7v2.18l8.46 8.46L17 10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoStories;
impl IconShape for MdAutoStories {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 1l-5 5v11l5-4.5V1zM1 6v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05C3.1 20.45 5.05 20 6.5 20c1.95 0 4.05.4 5.5 1.5V6c-1.45-1.1-3.55-1.5-5.5-1.5S2.45 4.9 1 6zm22 13.5V6c-.6-.45-1.25-.75-2-1v13.5c-1.1-.35-2.3-.5-3.5-.5-1.7 0-4.15.65-5.5 1.5v2c1.35-.85 3.8-1.5 5.5-1.5 1.65 0 3.35.3 4.75 1.05.1.05.15.05.25.05.25 0 .5-.25.5-.5v-1.1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCases;
impl IconShape for MdCases {
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
                d: "M18 6V4l-2-2h-5L9 4v2H5v11s1 2 2 2h13s2-.98 2-2V6h-4zM4 9H2v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2H4V9zm7-4c0-.55.53-1 1-1h3c.46 0 1 .54 1 1v1h-5V5zM5 6h17v11c0 1.1-.9 2-2 2H7c-1.1 0-2-.9-2-2V6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGridOn;
impl IconShape for MdGridOn {
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM8 20H4v-4h4v4zm0-6H4v-4h4v4zm0-6H4V4h4v4zm6 12h-4v-4h4v4zm0-6h-4v-4h4v4zm0-6h-4V4h4v4zm6 12h-4v-4h4v4zm0-6h-4v-4h4v4zm0-6h-4V4h4v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAnimation;
impl IconShape for MdAnimation {
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
                d: "M15 2c-2.71 0-5.05 1.54-6.22 3.78-1.28.67-2.34 1.72-3 3C3.54 9.95 2 12.29 2 15c0 3.87 3.13 7 7 7 2.71 0 5.05-1.54 6.22-3.78 1.28-.67 2.34-1.72 3-3C20.46 14.05 22 11.71 22 9c0-3.87-3.13-7-7-7zM9 20c-2.76 0-5-2.24-5-5 0-1.12.37-2.16 1-3 0 3.87 3.13 7 7 7-.84.63-1.88 1-3 1zm3-3c-2.76 0-5-2.24-5-5 0-1.12.37-2.16 1-3 0 3.86 3.13 6.99 7 7-.84.63-1.88 1-3 1zm4.7-3.3c-.53.19-1.1.3-1.7.3-2.76 0-5-2.24-5-5 0-.6.11-1.17.3-1.7.53-.19 1.1-.3 1.7-.3 2.76 0 5 2.24 5 5 0 .6-.11 1.17-.3 1.7zM19 12c0-3.86-3.13-6.99-7-7 .84-.63 1.87-1 3-1 2.76 0 5 2.24 5 5 0 1.12-.37 2.16-1 3z",
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbTwighlight;
impl IconShape for MdWbTwighlight {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M16.954 8.66l2.12-2.12 1.415 1.414-2.13 2.12zM17.9 14c-.5-2.85-2.95-5-5.9-5s-5.45 2.15-5.9 5h11.8zM2 16h20v4H2zm9-12h2v3h-2zM3.54 7.925L4.954 6.51l2.122 2.122-1.415 1.415z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlip;
impl IconShape for MdFlip {
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
                d: "M15 21h2v-2h-2v2zm4-12h2V7h-2v2zM3 5v14c0 1.1.9 2 2 2h4v-2H5V5h4V3H5c-1.1 0-2 .9-2 2zm16-2v2h2c0-1.1-.9-2-2-2zm-8 20h2V1h-2v22zm8-6h2v-2h-2v2zM15 5h2V3h-2v2zm4 8h2v-2h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDetails;
impl IconShape for MdDetails {
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
                d: "M12,3L2,21h20L12,3z M13,8.92L18.6,19H13V8.92z M11,8.92V19H5.4L11,8.92z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddAPhoto;
impl IconShape for MdAddAPhoto {
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
                d: "M3,4V1h2v3h3v2H5v3H3V6H0V4H3z M6,10V7h3V4h7l1.83,2H21c1.1,0,2,0.9,2,2v12c0,1.1-0.9,2-2,2H5c-1.1,0-2-0.9-2-2V10H6z M13,19c2.76,0,5-2.24,5-5s-2.24-5-5-5s-5,2.24-5,5S10.24,19,13,19z M9.8,14c0,1.77,1.43,3.2,3.2,3.2s3.2-1.43,3.2-3.2 s-1.43-3.2-3.2-3.2S9.8,12.23,9.8,14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStyle;
impl IconShape for MdStyle {
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
                d: "M2.53 19.65l1.34.56v-9.03l-2.43 5.86c-.41 1.02.08 2.19 1.09 2.61zm19.5-3.7L17.07 3.98c-.31-.75-1.04-1.21-1.81-1.23-.26 0-.53.04-.79.15L7.1 5.95c-.75.31-1.21 1.03-1.23 1.8-.01.27.04.54.15.8l4.96 11.97c.31.76 1.05 1.22 1.83 1.23.26 0 .52-.05.77-.15l7.36-3.05c1.02-.42 1.51-1.59 1.09-2.6zM7.88 8.75c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-2 11c0 1.1.9 2 2 2h1.45l-3.45-8.34v6.34z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaVerticalSelect;
impl IconShape for MdPanoramaVerticalSelect {
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
                d: "M19.93 21.12c-1.1-2.94-1.64-6.03-1.64-9.12s.55-6.18 1.64-9.12c.05-.11.07-.22.07-.31 0-.34-.24-.57-.64-.57H4.62c-.4 0-.63.23-.63.57 0 .1.02.2.06.31C5.16 5.82 5.7 8.91 5.7 12s-.55 6.18-1.64 9.12c-.05.11-.07.22-.07.31 0 .33.23.57.63.57h14.75c.39 0 .63-.24.63-.57 0-.1-.02-.2-.07-.31z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropDin;
impl IconShape for MdCropDin {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropSquare;
impl IconShape for MdCropSquare {
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
                d: "M18 4H6c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H6V6h12v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNature;
impl IconShape for MdNature {
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
                d: "M13 16.12c3.47-.41 6.17-3.36 6.17-6.95 0-3.87-3.13-7-7-7s-7 3.13-7 7c0 3.47 2.52 6.34 5.83 6.89V20H5v2h14v-2h-6v-3.88z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoFixNormal;
impl IconShape for MdAutoFixNormal {
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
                d: "M22 2l-2.5 1.4L17 2l1.4 2.5L17 7l2.5-1.4L22 7l-1.4-2.5zm-7.63 5.29c-.39-.39-1.02-.39-1.41 0L1.29 18.96c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0L16.7 11.05c.39-.39.39-1.02 0-1.41l-2.33-2.35zm-1.03 5.49l-2.12-2.12 2.44-2.44 2.12 2.12-2.44 2.44z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md11mp;
impl IconShape for Md11mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM11 5.5v6H9.5V7H8V5.5h3zm5 0v6h-1.5V7H13V5.5h3zm-.5 8.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilterHdr;
impl IconShape for MdFilterHdr {
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
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdImage;
impl IconShape for MdImage {
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
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md4mp;
impl IconShape for Md4mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3-8.5h-1v1.5h-1.5V10h-3V5.5H11v3h1.5v-3H14v3h1V10zm.5 8.5H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm0-4.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropLandscape;
impl IconShape for MdCropLandscape {
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
                d: "M19 5H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 12H5V7h14v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCameraAlt;
impl IconShape for MdCameraAlt {
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

#[derive(Copy, Clone, Debug)]
pub struct MdDirtyLens;
impl IconShape for MdDirtyLens {
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
                d: "M12.95 19H20V7H4v12h7.24c.14-.98.42-2.05-.16-2.43-.89-.59-1.27 2.06-2.8 1.35-1.39-1.12 1.05-1.29.5-3.27-.22-.79-2.28.36-2.4-1.24-.08-1 1.49-.74 1.51-1.49.03-.75-1.03-1.05-.25-1.91.22-.24.71-.26.91-.19.79.27 1.55 1.82 2.51 1.19 1.03-.66-1.88-2.35 0-2.86 1.64-.44 1.31 2.08 2.65 2.44 1.94.52 2.65-4.55 4.41-2.33 1.85 2.33-3.43 2.27-2.85 4.01.34 1.01 2.15-1.2 2.76.53.64 1.83-3.09.82-3.04 1.66.06.83 2.41.55 1.64 2.12-1.14 1.86-3-1.03-3.81.09-.39.57-.09 1.49.13 2.33zM20 5c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V7c0-1.1.9-2 2-2h3.17L9 3h6l1.83 2H20zm-1.86 13.01c-.47 0-.86-.38-.86-.86s.38-.86.86-.86c.47 0 .86.38.86.86s-.38.86-.86.86z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBlurLinear;
impl IconShape for MdBlurLinear {
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
                d: "M5 17.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5.67 1.5 1.5 1.5zM9 13c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zM3 21h18v-2H3v2zM5 9.5c.83 0 1.5-.67 1.5-1.5S5.83 6.5 5 6.5 3.5 7.17 3.5 8 4.17 9.5 5 9.5zm0 4c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5.67 1.5 1.5 1.5zM9 17c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm8-.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM3 3v2h18V3H3zm14 5.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm0 4c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM13 9c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 4c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoCameraFront;
impl IconShape for MdPhotoCameraFront {
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
                d: "M18 10.48l4-3.98v11l-4-3.98V18c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2h12c1.1 0 2 .9 2 2v4.48zm-2-.79V6H4v12h12V9.69zM10 12c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm0 1c1.34 0 4 .67 4 2v1H6v-1c0-1.33 2.66-2 4-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSwitchVideo;
impl IconShape for MdSwitchVideo {
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
                d: "M18 9.5V6c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-3.5l4 4v-13l-4 4zm-5 6V13H7v2.5L3.5 12 7 8.5V11h6V8.5l3.5 3.5-3.5 3.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBlurCircular;
impl IconShape for MdBlurCircular {
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
                d: "M10 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zM7 9.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm3 7c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm-3-3c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm3-6c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM14 9c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-1.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm3 6c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-4c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm2-3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-3.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdEdit;
impl IconShape for MdEdit {
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
                d: "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness7;
impl IconShape for MdBrightness7 {
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
                d: "M20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6zm0-10c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGrain;
impl IconShape for MdGrain {
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
                d: "M10 12c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12-8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-4 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-4-4c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdReceiptLong;
impl IconShape for MdReceiptLong {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0,0h24v24H0V0z",
            }
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

#[derive(Copy, Clone, Debug)]
pub struct MdImageAspectRatio;
impl IconShape for MdImageAspectRatio {
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
                d: "M16 10h-2v2h2v-2zm0 4h-2v2h2v-2zm-8-4H6v2h2v-2zm4 0h-2v2h2v-2zm8-6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h16v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLens;
impl IconShape for MdLens {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoAwesomeMotion;
impl IconShape for MdAutoAwesomeMotion {
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
                d: "M14 2H4c-1.11 0-2 .9-2 2v10h2V4h10V2zm4 4H8c-1.11 0-2 .9-2 2v10h2V8h10V6zm2 4h-8c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMovieFilter;
impl IconShape for MdMovieFilter {
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
                d: "M18 4l2 3h-3l-2-3h-2l2 3h-3l-2-3H8l2 3H7L5 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4h-4zm-6.75 11.25L10 18l-1.25-2.75L6 14l2.75-1.25L10 10l1.25 2.75L14 14l-2.75 1.25zm5.69-3.31L16 14l-.94-2.06L13 11l2.06-.94L16 8l.94 2.06L19 11l-2.06.94z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTimer10;
impl IconShape for MdTimer10 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M0 7.72V9.4l3-1V18h2V6h-.25L0 7.72zm23.78 6.65c-.14-.28-.35-.53-.63-.74-.28-.21-.61-.39-1.01-.53s-.85-.27-1.35-.38c-.35-.07-.64-.15-.87-.23-.23-.08-.41-.16-.55-.25-.14-.09-.23-.19-.28-.3-.05-.11-.08-.24-.08-.39 0-.14.03-.28.09-.41.06-.13.15-.25.27-.34.12-.1.27-.18.45-.24s.4-.09.64-.09c.25 0 .47.04.66.11.19.07.35.17.48.29.13.12.22.26.29.42.06.16.1.32.1.49h1.95c0-.39-.08-.75-.24-1.09-.16-.34-.39-.63-.69-.88-.3-.25-.66-.44-1.09-.59C21.49 9.07 21 9 20.46 9c-.51 0-.98.07-1.39.21-.41.14-.77.33-1.06.57-.29.24-.51.52-.67.84-.16.32-.23.65-.23 1.01s.08.69.23.96c.15.28.36.52.64.73.27.21.6.38.98.53.38.14.81.26 1.27.36.39.08.71.17.95.26s.43.19.57.29c.13.1.22.22.27.34.05.12.07.25.07.39 0 .32-.13.57-.4.77-.27.2-.66.29-1.17.29-.22 0-.43-.02-.64-.08-.21-.05-.4-.13-.56-.24-.17-.11-.3-.26-.41-.44-.11-.18-.17-.41-.18-.67h-1.89c0 .36.08.71.24 1.05.16.34.39.65.7.93.31.27.69.49 1.15.66.46.17.98.25 1.58.25.53 0 1.01-.06 1.44-.19.43-.13.8-.31 1.11-.54.31-.23.54-.51.71-.83.17-.32.25-.67.25-1.06-.02-.4-.09-.74-.24-1.02zm-9.96-7.32c-.34-.4-.75-.7-1.23-.88-.47-.18-1.01-.27-1.59-.27-.58 0-1.11.09-1.59.27-.48.18-.89.47-1.23.88-.34.41-.6.93-.79 1.59-.18.65-.28 1.45-.28 2.39v1.92c0 .94.09 1.74.28 2.39.19.66.45 1.19.8 1.6.34.41.75.71 1.23.89.48.18 1.01.28 1.59.28.59 0 1.12-.09 1.59-.28.48-.18.88-.48 1.22-.89.34-.41.6-.94.78-1.6.18-.65.28-1.45.28-2.39v-1.92c0-.94-.09-1.74-.28-2.39-.18-.66-.44-1.19-.78-1.59zm-.92 6.17c0 .6-.04 1.11-.12 1.53-.08.42-.2.76-.36 1.02-.16.26-.36.45-.59.57-.23.12-.51.18-.82.18-.3 0-.58-.06-.82-.18s-.44-.31-.6-.57c-.16-.26-.29-.6-.38-1.02-.09-.42-.13-.93-.13-1.53v-2.5c0-.6.04-1.11.13-1.52.09-.41.21-.74.38-1 .16-.25.36-.43.6-.55.24-.11.51-.17.81-.17.31 0 .58.06.81.17.24.11.44.29.6.55.16.25.29.58.37.99.08.41.13.92.13 1.52v2.51z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaHorizontal;
impl IconShape for MdPanoramaHorizontal {
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
                d: "M20 6.54v10.91c-2.6-.77-5.28-1.16-8-1.16-2.72 0-5.4.39-8 1.16V6.54c2.6.77 5.28 1.16 8 1.16 2.72.01 5.4-.38 8-1.16M21.43 4c-.1 0-.2.02-.31.06C18.18 5.16 15.09 5.7 12 5.7c-3.09 0-6.18-.55-9.12-1.64-.11-.04-.22-.06-.31-.06-.34 0-.57.23-.57.63v14.75c0 .39.23.62.57.62.1 0 .2-.02.31-.06 2.94-1.1 6.03-1.64 9.12-1.64 3.09 0 6.18.55 9.12 1.64.11.04.21.06.31.06.33 0 .57-.23.57-.63V4.63c0-.4-.24-.63-.57-.63z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoAlbum;
impl IconShape for MdPhotoAlbum {
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
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4zm0 15l3-3.86 2.14 2.58 3-3.86L18 19H6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaPhotosphereSelect;
impl IconShape for MdPanoramaPhotosphereSelect {
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
                d: "M22.49 8.51c-.47-.23-.93-.44-1.4-.64C19.52 4.41 16.05 2 12 2S4.47 4.41 2.9 7.88c-.47.2-.93.41-1.4.63-.31.15-.5.48-.5.83v5.32c0 .35.19.68.51.83.47.23.93.44 1.39.64 3.55 7.83 14.65 7.82 18.2 0 .47-.2.93-.41 1.39-.63.31-.17.51-.49.51-.84V9.34c0-.35-.19-.68-.51-.83zM12 3.8c2.6 0 4.91 1.23 6.41 3.12-4.1-1.19-8.48-1.26-12.83.01C7.08 5.03 9.4 3.8 12 3.8zM5.6 17.08c4.19 1.22 8.57 1.23 12.82-.01-1.54 1.97-3.9 3.13-6.41 3.13-2.5 0-4.87-1.15-6.41-3.12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLoupe;
impl IconShape for MdLoupe {
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
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.49 2 2 6.49 2 12s4.49 10 10 10h8c1.1 0 2-.9 2-2v-8c0-5.51-4.49-10-10-10zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooks;
impl IconShape for MdLooks {
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
                d: "M12 10c-3.86 0-7 3.14-7 7h2c0-2.76 2.24-5 5-5s5 2.24 5 5h2c0-3.86-3.14-7-7-7zm0-4C5.93 6 1 10.93 1 17h2c0-4.96 4.04-9 9-9s9 4.04 9 9h2c0-6.07-4.93-11-11-11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoSizeSelectSmall;
impl IconShape for MdPhotoSizeSelectSmall {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0zm24 24H0V0h24v24z",
            }
            path {
                d: "M23 15h-2v2h2v-2zm0-4h-2v2h2v-2zm0 8h-2v2c1 0 2-1 2-2zM15 3h-2v2h2V3zm8 4h-2v2h2V7zm-2-4v2h2c0-1-1-2-2-2zM3 21h8v-6H1v4c0 1.1.9 2 2 2zM3 7H1v2h2V7zm12 12h-2v2h2v-2zm4-16h-2v2h2V3zm0 16h-2v2h2v-2zM3 3C2 3 1 4 1 5h2V3zm0 8H1v2h2v-2zm8-8H9v2h2V3zM7 3H5v2h2V3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness6;
impl IconShape for MdBrightness6 {
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
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18V6c3.31 0 6 2.69 6 6s-2.69 6-6 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md21mp;
impl IconShape for Md21mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM11 9H9v1h3v1.5H7.5V9c0-.55.45-1 1-1h2V7h-3V5.5H11c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm3-3.5h3v6h-1.5V7H14V5.5zm1.5 8.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNavigateNext;
impl IconShape for MdNavigateNext {
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
                d: "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightness1;
impl IconShape for MdBrightness1 {
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
            circle {
                cx: "12",
                cy: "12",
                r: "10",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTune;
impl IconShape for MdTune {
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
                d: "M3 17v2h6v-2H3zM3 5v2h10V5H3zm10 16v-2h8v-2h-8v-2h-2v6h2zM7 9v2H3v2h4v2h2V9H7zm14 4v-2H11v2h10zm-6-4h2V7h4V5h-4V3h-2v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoCamera;
impl IconShape for MdPhotoCamera {
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

#[derive(Copy, Clone, Debug)]
pub struct MdCenterFocusWeak;
impl IconShape for MdCenterFocusWeak {
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
                d: "M5 15H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zM12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md14mp;
impl IconShape for Md14mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm7.5 4.5h-1v1.5H15V10h-3V5.5h1.5v3H15v-3h1.5v3h1V10zm-2 4H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooks4;
impl IconShape for MdLooks4 {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 14h-2v-4H9V7h2v4h2V7h2v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooks3;
impl IconShape for MdLooks3 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M.01 0h24v24h-24z",
            }
            path {
                d: "M19.01 3h-14c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 7.5c0 .83-.67 1.5-1.5 1.5.83 0 1.5.67 1.5 1.5V15c0 1.11-.9 2-2 2h-4v-2h4v-2h-2v-2h2V9h-4V7h4c1.1 0 2 .89 2 2v1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMp;
impl IconShape for MdMp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM6.5 9H11c.55 0 1 .45 1 1v5h-1.5v-4.5h-1v3H8v-3H7V15H5.5v-5c0-.55.45-1 1-1zm9 6H14V9h3.5c.55 0 1 .45 1 1v2.5c0 .55-.45 1-1 1h-2V15zm0-3H17v-1.5h-1.5V12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropPortrait;
impl IconShape for MdCropPortrait {
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
                d: "M17 3H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H7V5h10v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBedtime;
impl IconShape for MdBedtime {
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
                d: "M12.34,2.02C6.59,1.82,2,6.42,2,12c0,5.52,4.48,10,10,10c3.71,0,6.93-2.02,8.66-5.02C13.15,16.73,8.57,8.55,12.34,2.02z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRotateRight;
impl IconShape for MdRotateRight {
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
                d: "M15.55 5.55L11 1v3.07C7.06 4.56 4 7.92 4 12s3.05 7.44 7 7.93v-2.02c-2.84-.48-5-2.94-5-5.91s2.16-5.43 5-5.91V10l4.55-4.45zM19.93 11c-.17-1.39-.72-2.73-1.62-3.89l-1.42 1.42c.54.75.88 1.6 1.02 2.47h2.02zM13 17.9v2.02c1.39-.17 2.74-.71 3.9-1.61l-1.44-1.44c-.75.54-1.59.89-2.46 1.03zm3.89-2.42l1.42 1.41c.9-1.16 1.45-2.5 1.62-3.89h-2.02c-.14.87-.48 1.72-1.02 2.48z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHdrStrong;
impl IconShape for MdHdrStrong {
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
                d: "M17 6c-3.31 0-6 2.69-6 6s2.69 6 6 6 6-2.69 6-6-2.69-6-6-6zM5 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNaturePeople;
impl IconShape for MdNaturePeople {
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
                d: "M22.17 9.17c0-3.87-3.13-7-7-7s-7 3.13-7 7c0 3.47 2.52 6.34 5.83 6.89V20H6v-3h1v-4c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v4h1v5h16v-2h-3v-3.88c3.47-.41 6.17-3.36 6.17-6.95zM4.5 11c.83 0 1.5-.67 1.5-1.5S5.33 8 4.5 8 3 8.67 3 9.5 3.67 11 4.5 11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposureNeg2;
impl IconShape for MdExposureNeg2 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M15.05 16.29l2.86-3.07c.38-.39.72-.79 1.04-1.18.32-.39.59-.78.82-1.17.23-.39.41-.78.54-1.17s.19-.79.19-1.18c0-.53-.09-1.02-.27-1.46-.18-.44-.44-.81-.78-1.11-.34-.31-.77-.54-1.26-.71-.51-.16-1.08-.24-1.72-.24-.69 0-1.31.11-1.85.32-.54.21-1 .51-1.36.88-.37.37-.65.8-.84 1.3-.18.47-.27.97-.28 1.5h2.14c.01-.31.05-.6.13-.87.09-.29.23-.54.4-.75.18-.21.41-.37.68-.49.27-.12.6-.18.96-.18.31 0 .58.05.81.15.23.1.43.25.59.43.16.18.28.4.37.65.08.25.13.52.13.81 0 .22-.03.43-.08.65-.06.22-.15.45-.29.7-.14.25-.32.53-.56.83-.23.3-.52.65-.88 1.03l-4.17 4.55V18H21v-1.71h-5.95zM2 11v2h8v-2H2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCompare;
impl IconShape for MdCompare {
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
                d: "M10 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h5v2h2V1h-2v2zm0 15H5l5-6v6zm9-15h-5v2h5v13l-5-6v9h5c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposurePlus2;
impl IconShape for MdExposurePlus2 {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M16.05 16.29l2.86-3.07c.38-.39.72-.79 1.04-1.18.32-.39.59-.78.82-1.17.23-.39.41-.78.54-1.17.13-.39.19-.79.19-1.18 0-.53-.09-1.02-.27-1.46-.18-.44-.44-.81-.78-1.11-.34-.31-.77-.54-1.26-.71-.51-.16-1.08-.24-1.72-.24-.69 0-1.31.11-1.85.32-.54.21-1 .51-1.36.88-.37.37-.65.8-.84 1.3-.18.47-.27.97-.28 1.5h2.14c.01-.31.05-.6.13-.87.09-.29.23-.54.4-.75.18-.21.41-.37.68-.49.27-.12.6-.18.96-.18.31 0 .58.05.81.15.23.1.43.25.59.43.16.18.28.4.37.65.08.25.13.52.13.81 0 .22-.03.43-.08.65-.06.22-.15.45-.29.7-.14.25-.32.53-.56.83-.23.3-.52.65-.88 1.03l-4.17 4.55V18H22v-1.71h-5.95zM8 7H6v4H2v2h4v4h2v-4h4v-2H8V7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanorama;
impl IconShape for MdPanorama {
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
                d: "M23 18V6c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2zM8.5 12.5l2.5 3.01L14.5 11l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCollectionsBookmark;
impl IconShape for MdCollectionsBookmark {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
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

#[derive(Copy, Clone, Debug)]
pub struct MdHdrOn;
impl IconShape for MdHdrOn {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M21 11.5v-1c0-.8-.7-1.5-1.5-1.5H16v6h1.5v-2h1.1l.9 2H21l-.9-2.1c.5-.3.9-.8.9-1.4zm-1.5 0h-2v-1h2v1zm-13-.5h-2V9H3v6h1.5v-2.5h2V15H8V9H6.5v2zM13 9H9.5v6H13c.8 0 1.5-.7 1.5-1.5v-3c0-.8-.7-1.5-1.5-1.5zm0 4.5h-2v-3h2v3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter9Plus;
impl IconShape for MdFilter9Plus {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm11 7V8c0-1.11-.9-2-2-2h-1c-1.1 0-2 .89-2 2v1c0 1.11.9 2 2 2h1v1H9v2h3c1.1 0 2-.89 2-2zm-3-3V8h1v1h-1zm10-8H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 8h-2V7h-2v2h-2v2h2v2h2v-2h2v6H7V3h14v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md5mp;
impl IconShape for Md5mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM14.5 7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H10V10h3V9h-3V5.5h4.5V7zm1 7H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMicExternalOn;
impl IconShape for MdMicExternalOn {
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
                d: "M9.22 7H4.78C4.3 6.47 4 5.77 4 5c0-1.66 1.34-3 3-3s3 1.34 3 3c0 .77-.3 1.47-.78 2zM16 2c2.21 0 4 1.79 4 4v16h-2V6c0-1.1-.9-2-2-2s-2 .9-2 2v12c0 2.21-1.79 4-4 4s-4-1.79-4-4H5L4 8h6L9 18H8c0 1.1.9 2 2 2s2-.9 2-2V6c0-2.21 1.79-4 4-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTexture;
impl IconShape for MdTexture {
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
                d: "M19.51 3.08L3.08 19.51c.09.34.27.65.51.9.25.24.56.42.9.51L20.93 4.49c-.19-.69-.73-1.23-1.42-1.41zM11.88 3L3 11.88v2.83L14.71 3h-2.83zM5 3c-1.1 0-2 .9-2 2v2l4-4H5zm14 18c.55 0 1.05-.22 1.41-.59.37-.36.59-.86.59-1.41v-2l-4 4h2zm-9.71 0h2.83L21 12.12V9.29L9.29 21z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddPhotoAlternate;
impl IconShape for MdAddPhotoAlternate {
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
                d: "M19 7v2.99s-1.99.01-2 0V7h-3s.01-1.99 0-2h3V2h2v3h3v2h-3zm-3 4V8h-3V5H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-8h-3zM5 19l3-4 2 3 3-4 4 5H5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md10mp;
impl IconShape for Md10mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13.5 7H15v3h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H13c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm-1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooks5;
impl IconShape for MdLooks5 {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 6h-4v2h2c1.1 0 2 .89 2 2v2c0 1.11-.9 2-2 2H9v-2h4v-2H9V7h6v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCircle;
impl IconShape for MdCircle {
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
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlashOn;
impl IconShape for MdFlashOn {
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
                d: "M7 2v11h3v9l7-12h-4l4-8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoSizeSelectLarge;
impl IconShape for MdPhotoSizeSelectLarge {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M24 24H0V0h24v24z",
            }
            path {
                d: "M21 15h2v2h-2v-2zm0-4h2v2h-2v-2zm2 8h-2v2c1 0 2-1 2-2zM13 3h2v2h-2V3zm8 4h2v2h-2V7zm0-4v2h2c0-1-1-2-2-2zM1 7h2v2H1V7zm16-4h2v2h-2V3zm0 16h2v2h-2v-2zM3 3C2 3 1 4 1 5h2V3zm6 0h2v2H9V3zM5 3h2v2H5V3zm-4 8v8c0 1.1.9 2 2 2h12V11H1zm2 8l2.5-3.21 1.79 2.15 2.5-3.22L13 19H3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter8;
impl IconShape for MdFilter8 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2h2c1.1 0 2-.89 2-2v-1.5c0-.83-.67-1.5-1.5-1.5.83 0 1.5-.67 1.5-1.5V7c0-1.11-.9-2-2-2h-2c-1.1 0-2 .89-2 2v1.5c0 .83.67 1.5 1.5 1.5-.83 0-1.5.67-1.5 1.5V13c0 1.11.9 2 2 2zm0-8h2v2h-2V7zm0 4h2v2h-2v-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCrop32;
impl IconShape for MdCrop32 {
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
                d: "M19 4H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H5V6h14v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrush;
impl IconShape for MdBrush {
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
                d: "M7 14c-1.66 0-3 1.34-3 3 0 1.31-1.16 2-2 2 .92 1.22 2.49 2 4 2 2.21 0 4-1.79 4-4 0-1.66-1.34-3-3-3zm13.71-9.37l-1.34-1.34c-.39-.39-1.02-.39-1.41 0L9 12.25 11.75 15l8.96-8.96c.39-.39.39-1.02 0-1.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter1;
impl IconShape for MdFilter1 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm11 10h2V5h-4v2h2v8zm7-14H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHealing;
impl IconShape for MdHealing {
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
                d: "M17.73 12.02l3.98-3.98c.39-.39.39-1.02 0-1.41l-4.34-4.34c-.39-.39-1.02-.39-1.41 0l-3.98 3.98L8 2.29C7.8 2.1 7.55 2 7.29 2c-.25 0-.51.1-.7.29L2.25 6.63c-.39.39-.39 1.02 0 1.41l3.98 3.98L2.25 16c-.39.39-.39 1.02 0 1.41l4.34 4.34c.39.39 1.02.39 1.41 0l3.98-3.98 3.98 3.98c.2.2.45.29.71.29.26 0 .51-.1.71-.29l4.34-4.34c.39-.39.39-1.02 0-1.41l-3.99-3.98zM12 9c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-4.71 1.96L3.66 7.34l3.63-3.63 3.62 3.62-3.62 3.63zM10 13c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm2 2c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm2-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2.66 9.34l-3.63-3.62 3.63-3.63 3.62 3.62-3.62 3.63z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdEuro;
impl IconShape for MdEuro {
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
                d: "M15,18.5c-2.51,0-4.68-1.42-5.76-3.5H15l1-2H8.58c-0.05-0.33-0.08-0.66-0.08-1s0.03-0.67,0.08-1H15l1-2H9.24 C10.32,6.92,12.5,5.5,15,5.5c1.61,0,3.09,0.59,4.23,1.57L21,5.3C19.41,3.87,17.3,3,15,3c-3.92,0-7.24,2.51-8.48,6H3l-1,2h4.06 C6.02,11.33,6,11.66,6,12s0.02,0.67,0.06,1H3l-1,2h4.52c1.24,3.49,4.56,6,8.48,6c2.31,0,4.41-0.87,6-2.3l-1.78-1.77 C18.09,17.91,16.62,18.5,15,18.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter6;
impl IconShape for MdFilter6 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2h2c1.1 0 2-.89 2-2v-2c0-1.11-.9-2-2-2h-2V7h4V5h-4c-1.1 0-2 .89-2 2v6c0 1.11.9 2 2 2zm0-4h2v2h-2v-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLandscape;
impl IconShape for MdLandscape {
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
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNavigateBefore;
impl IconShape for MdNavigateBefore {
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
                d: "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdVignette;
impl IconShape for MdVignette {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 15c-4.42 0-8-2.69-8-6s3.58-6 8-6 8 2.69 8 6-3.58 6-8 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMicExternalOff;
impl IconShape for MdMicExternalOff {
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
                d: "M21.19 21.19L2.81 2.81 1.39 4.22 5.17 8H4l1 10h1c0 2.21 1.79 4 4 4s4-1.79 4-4v-1.17l5.78 5.78 1.41-1.42zM12 18c0 1.1-.9 2-2 2s-2-.9-2-2h1l.56-5.61L12 14.83V18zm2-12v5.17l-2-2V6c0-2.21 1.79-4 4-4s4 1.79 4 4v11.17l-2-2V6c0-1.1-.9-2-2-2s-2 .9-2 2zm-4-1c0 .62-.2 1.18-.52 1.66L5.33 2.51C5.81 2.19 6.38 2 7 2c1.66 0 3 1.34 3 3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdImageSearch;
impl IconShape for MdImageSearch {
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
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M18 13v7H4V6h5.02c.05-.71.22-1.38.48-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-5l-2-2zm-1.5 5h-11l2.75-3.53 1.96 2.36 2.75-3.54zm2.8-9.11c.44-.7.7-1.51.7-2.39C20 4.01 17.99 2 15.5 2S11 4.01 11 6.5s2.01 4.5 4.49 4.5c.88 0 1.7-.26 2.39-.7L21 13.42 22.42 12 19.3 8.89zM15.5 9C14.12 9 13 7.88 13 6.5S14.12 4 15.5 4 18 5.12 18 6.5 16.88 9 15.5 9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter;
impl IconShape for MdFilter {
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
                d: "M15.96 10.29l-2.75 3.54-1.96-2.36L8.5 15h11l-3.54-4.71zM3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter7;
impl IconShape for MdFilter7 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zm-8-2l4-8V5h-6v2h4l-4 8h2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbShade;
impl IconShape for MdWbShade {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14 12v2.5l5.5 5.5H22zm0 8h3l-3-3zM8 4l-6 6h2v10h8V10h2L8 4zm1 10H7v-4h2v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md22mp;
impl IconShape for Md22mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm6.5 0h-2v1h3v1.5H13V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm-1 5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFilter9;
impl IconShape for MdFilter9 {
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
                d: "M3 5H1v16c0 1.1.9 2 2 2h16v-2H3V5zm18-4H7c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 16H7V3h14v14zM15 5h-2c-1.1 0-2 .89-2 2v2c0 1.11.9 2 2 2h2v2h-4v2h4c1.1 0 2-.89 2-2V7c0-1.11-.9-2-2-2zm0 4h-2V7h2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTonality;
impl IconShape for MdTonality {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.94-.49-7-3.85-7-7.93s3.05-7.44 7-7.93v15.86zm2-15.86c1.03.13 2 .45 2.87.93H13v-.93zM13 7h5.24c.25.31.48.65.68 1H13V7zm0 3h6.74c.08.33.15.66.19 1H13v-1zm0 9.93V19h2.87c-.87.48-1.84.8-2.87.93zM18.24 17H13v-1h5.92c-.2.35-.43.69-.68 1zm1.5-3H13v-1h6.93c-.04.34-.11.67-.19 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooksOne;
impl IconShape for MdLooksOne {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14h-2V9h-2V7h4v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCenterFocusStrong;
impl IconShape for MdCenterFocusStrong {
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
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm-7 7H3v4c0 1.1.9 2 2 2h4v-2H5v-4zM5 5h4V3H5c-1.1 0-2 .9-2 2v4h2V5zm14-2h-4v2h4v4h2V5c0-1.1-.9-2-2-2zm0 16h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhotoSizeSelectActual;
impl IconShape for MdPhotoSizeSelectActual {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M24 24H0V0h24v24z",
            }
            path {
                d: "M21 3H3C2 3 1 4 1 5v14c0 1.1.9 2 2 2h18c1 0 2-1 2-2V5c0-1-1-2-2-2zM5 17l3.5-4.5 2.5 3.01L14.5 11l4.5 6H5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCrop75;
impl IconShape for MdCrop75 {
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
                d: "M19 7H5c-1.1 0-2 .9-2 2v6c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zm0 8H5V9h14v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCollections;
impl IconShape for MdCollections {
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
                d: "M22 16V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2zm-11-4l2.03 2.71L16 11l4 5H8l3-4zM2 6v14c0 1.1.9 2 2 2h14v-2H4V6H2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlipCameraAndroid;
impl IconShape for MdFlipCameraAndroid {
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

#[derive(Copy, Clone, Debug)]
pub struct MdCropFree;
impl IconShape for MdCropFree {
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
                d: "M3 5v4h2V5h4V3H5c-1.1 0-2 .9-2 2zm2 10H3v4c0 1.1.9 2 2 2h4v-2H5v-4zm14 4h-4v2h4c1.1 0 2-.9 2-2v-4h-2v4zm0-16h-4v2h4v4h2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMotionPhotosPause;
impl IconShape for MdMotionPhotosPause {
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
                d: "M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8 s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2C17.52,2,22,6.48,22,12z M5.5,4 C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z M18,12c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6S18,8.69,18,12z M11,9H9v6h2V9z M15,9h-2v6h2V9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdExposureZero;
impl IconShape for MdExposureZero {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M16.14 12.5c0 1-.1 1.85-.3 2.55-.2.7-.48 1.27-.83 1.7-.36.44-.79.75-1.3.95-.51.2-1.07.3-1.7.3-.62 0-1.18-.1-1.69-.3-.51-.2-.95-.51-1.31-.95-.36-.44-.65-1.01-.85-1.7-.2-.7-.3-1.55-.3-2.55v-2.04c0-1 .1-1.85.3-2.55.2-.7.48-1.26.84-1.69.36-.43.8-.74 1.31-.93C10.81 5.1 11.38 5 12 5c.63 0 1.19.1 1.7.29.51.19.95.5 1.31.93.36.43.64.99.84 1.69.2.7.3 1.54.3 2.55v2.04zm-2.11-2.36c0-.64-.05-1.18-.13-1.62-.09-.44-.22-.79-.4-1.06-.17-.27-.39-.46-.64-.58-.25-.13-.54-.19-.86-.19-.32 0-.61.06-.86.18s-.47.31-.64.58c-.17.27-.31.62-.4 1.06s-.13.98-.13 1.62v2.67c0 .64.05 1.18.14 1.62.09.45.23.81.4 1.09s.39.48.64.61.54.19.87.19c.33 0 .62-.06.87-.19s.46-.33.63-.61c.17-.28.3-.64.39-1.09.09-.45.13-.99.13-1.62v-2.66z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHdrOff;
impl IconShape for MdHdrOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M17.5 15v-2h1.1l.9 2H21l-.9-2.1c.5-.2.9-.8.9-1.4v-1c0-.8-.7-1.5-1.5-1.5H16v4.9l1.1 1.1h.4zm0-4.5h2v1h-2v-1zm-4.5 0v.4l1.5 1.5v-1.9c0-.8-.7-1.5-1.5-1.5h-1.9l1.5 1.5h.4zm-3.5-1l-7-7-1.1 1L6.9 9h-.4v2h-2V9H3v6h1.5v-2.5h2V15H8v-4.9l1.5 1.5V15h3.4l7.6 7.6 1.1-1.1-12.1-12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaWideAngle;
impl IconShape for MdPanoramaWideAngle {
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
                d: "M12 6c2.45 0 4.71.2 7.29.64.47 1.78.71 3.58.71 5.36 0 1.78-.24 3.58-.71 5.36-2.58.44-4.84.64-7.29.64s-4.71-.2-7.29-.64C4.24 15.58 4 13.78 4 12c0-1.78.24-3.58.71-5.36C7.29 6.2 9.55 6 12 6m0-2c-2.73 0-5.22.24-7.95.72l-.93.16-.25.9C2.29 7.85 2 9.93 2 12s.29 4.15.87 6.22l.25.89.93.16c2.73.49 5.22.73 7.95.73s5.22-.24 7.95-.72l.93-.16.25-.89c.58-2.08.87-4.16.87-6.23s-.29-4.15-.87-6.22l-.25-.89-.93-.16C17.22 4.24 14.73 4 12 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTransform;
impl IconShape for MdTransform {
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
                d: "M22 18v-2H8V4h2L7 1 4 4h2v2H2v2h4v8c0 1.1.9 2 2 2h8v2h-2l3 3 3-3h-2v-2h4zM10 8h6v6h2V8c0-1.1-.9-2-2-2h-6v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCameraRoll;
impl IconShape for MdCameraRoll {
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
                d: "M14 5c0-1.1-.9-2-2-2h-1V2c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1v1H4c-1.1 0-2 .9-2 2v15c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2h8V5h-8zm-2 13h-2v-2h2v2zm0-9h-2V7h2v2zm4 9h-2v-2h2v2zm0-9h-2V7h2v2zm4 9h-2v-2h2v2zm0-9h-2V7h2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSlideshow;
impl IconShape for MdSlideshow {
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
                d: "M10 8v8l5-4-5-4zm9-5H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMusicNote;
impl IconShape for MdMusicNote {
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
                d: "M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md8mp;
impl IconShape for Md8mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11.5 9H13v1.5h-1.5zm0-2.5H13V8h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-1-8c0 .55-.45 1-1 1H11c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaVertical;
impl IconShape for MdPanoramaVertical {
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
                d: "M19.94 21.12c-1.1-2.94-1.64-6.03-1.64-9.12 0-3.09.55-6.18 1.64-9.12.04-.11.06-.22.06-.31 0-.34-.23-.57-.63-.57H4.63c-.4 0-.63.23-.63.57 0 .1.02.2.06.31C5.16 5.82 5.71 8.91 5.71 12c0 3.09-.55 6.18-1.64 9.12-.05.11-.07.22-.07.31 0 .33.23.57.63.57h14.75c.39 0 .63-.24.63-.57-.01-.1-.03-.2-.07-.31zM6.54 20c.77-2.6 1.16-5.28 1.16-8 0-2.72-.39-5.4-1.16-8h10.91c-.77 2.6-1.16 5.28-1.16 8 0 2.72.39 5.4 1.16 8H6.54z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md2mp;
impl IconShape for Md2mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm-2-9.5h-2v1h3v1.5H10V9c0-.55.45-1 1-1h2V7h-3V5.5h3.5c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm2 5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhoto;
impl IconShape for MdPhoto {
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
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWbIridescent;
impl IconShape for MdWbIridescent {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M5 14.5h14v-6H5v6zM11 .55V3.5h2V.55h-2zm8.04 2.5l-1.79 1.79 1.41 1.41 1.8-1.79-1.42-1.41zM13 22.45V19.5h-2v2.95h2zm7.45-3.91l-1.8-1.79-1.41 1.41 1.79 1.8 1.42-1.42zM3.55 4.46l1.79 1.79 1.41-1.41-1.79-1.79-1.41 1.41zm1.41 15.49l1.79-1.8-1.41-1.41-1.79 1.79 1.41 1.42z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoAwesomeMosaic;
impl IconShape for MdAutoAwesomeMosaic {
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
                d: "M3 5v14c0 1.1.89 2 2 2h6V3H5c-1.11 0-2 .9-2 2zm16-2h-6v8h8V5c0-1.1-.9-2-2-2zm-6 18h6c1.1 0 2-.9 2-2v-6h-8v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdColorize;
impl IconShape for MdColorize {
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
                d: "M20.71 5.63l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-3.12 3.12-1.93-1.91-1.41 1.41 1.42 1.42L3 16.25V21h4.75l8.92-8.92 1.42 1.42 1.41-1.41-1.92-1.92 3.12-3.12c.4-.4.4-1.03.01-1.42zM6.92 19L5 17.08l8.06-8.06 1.92 1.92L6.92 19z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md17mp;
impl IconShape for Md17mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm5 6h-1.75L14.62 7H12V5.5h3.5c.67 0 1.15.65.96 1.29L15 11.5zm.5 2.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPanoramaWideAngleSelect;
impl IconShape for MdPanoramaWideAngleSelect {
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
                d: "M12 4c-2.73 0-5.22.24-7.95.72l-.93.16-.25.9C2.29 7.85 2 9.93 2 12s.29 4.15.87 6.22l.25.89.93.16c2.73.49 5.22.73 7.95.73s5.22-.24 7.95-.72l.93-.16.25-.89c.58-2.08.87-4.16.87-6.23s-.29-4.15-.87-6.22l-.25-.89-.93-.16C17.22 4.24 14.73 4 12 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRotateLeft;
impl IconShape for MdRotateLeft {
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
                d: "M7.11 8.53L5.7 7.11C4.8 8.27 4.24 9.61 4.07 11h2.02c.14-.87.49-1.72 1.02-2.47zM6.09 13H4.07c.17 1.39.72 2.73 1.62 3.89l1.41-1.42c-.52-.75-.87-1.59-1.01-2.47zm1.01 5.32c1.16.9 2.51 1.44 3.9 1.61V17.9c-.87-.15-1.71-.49-2.46-1.03L7.1 18.32zM13 4.07V1L8.45 5.55 13 10V6.09c2.84.48 5 2.94 5 5.91s-2.16 5.43-5 5.91v2.02c3.95-.49 7-3.85 7-7.93s-3.05-7.44-7-7.93z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdFlare;
impl IconShape for MdFlare {
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
                d: "M7 11H1v2h6v-2zm2.17-3.24L7.05 5.64 5.64 7.05l2.12 2.12 1.41-1.41zM13 1h-2v6h2V1zm5.36 6.05l-1.41-1.41-2.12 2.12 1.41 1.41 2.12-2.12zM17 11v2h6v-2h-6zm-5-2c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3zm2.83 7.24l2.12 2.12 1.41-1.41-2.12-2.12-1.41 1.41zm-9.19.71l1.41 1.41 2.12-2.12-1.41-1.41-2.12 2.12zM11 23h2v-6h-2v6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md6mp;
impl IconShape for Md6mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11.5 9H13v1.5h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm-1-7c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5V7h-3v1h2c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1H11zm4.5 7H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm0-4.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md13mp;
impl IconShape for Md13mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zM10 5.5v6H8.5V7H7V5.5h3zm6.5 5c0 .55-.45 1-1 1H12V10h3V9h-2V8h2V7h-3V5.5h3.5c.55 0 1 .45 1 1v4zm-1 3.5H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMotionPhotosOff;
impl IconShape for MdMotionPhotosOff {
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
                d: "M20.84 20.84L3.16 3.16 1.89 4.43l1.89 1.89C2.66 7.93 2 9.89 2 12c0 5.52 4.48 10 10 10 2.11 0 4.07-.66 5.68-1.77l1.89 1.89 1.27-1.28zM12 20c-4.41 0-8-3.59-8-8 0-1.55.45-3 1.22-4.23l1.46 1.46C6.25 10.06 6 11 6 12c0 3.31 2.69 6 6 6 1 0 1.94-.25 2.77-.68l1.46 1.46C15 19.55 13.55 20 12 20zM6.32 3.77C7.93 2.66 9.89 2 12 2c5.52 0 10 4.48 10 10 0 2.11-.66 4.07-1.77 5.68l-1.45-1.45C19.55 15 20 13.55 20 12c0-4.41-3.59-8-8-8-1.55 0-3 .45-4.23 1.22L6.32 3.77zM18 12c0 1-.25 1.94-.68 2.77L9.23 6.68C10.06 6.25 11 6 12 6c3.31 0 6 2.69 6 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCropOriginal;
impl IconShape for MdCropOriginal {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-5.04-6.71l-2.75 3.54-1.96-2.36L6.5 17h11l-3.54-4.71z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTimerOff;
impl IconShape for MdTimerOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M19.04 4.55l-1.42 1.42C16.07 4.74 14.12 4 12 4c-1.83 0-3.53.55-4.95 1.48l1.46 1.46C9.53 6.35 10.73 6 12 6c3.87 0 7 3.13 7 7 0 1.27-.35 2.47-.94 3.49l1.45 1.45C20.45 16.53 21 14.83 21 13c0-2.12-.74-4.07-1.97-5.61l1.42-1.42-1.41-1.42zM15 1H9v2h6V1zm-4 8.44l2 2V8h-2v1.44zM3.02 4L1.75 5.27 4.5 8.03C3.55 9.45 3 11.16 3 13c0 4.97 4.02 9 9 9 1.84 0 3.55-.55 4.98-1.5l2.5 2.5 1.27-1.27-7.71-7.71L3.02 4zM12 20c-3.87 0-7-3.13-7-7 0-1.28.35-2.48.95-3.52l9.56 9.56c-1.03.61-2.23.96-3.51.96z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPalette;
impl IconShape for MdPalette {
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
                d: "M12 3c-4.97 0-9 4.03-9 9s4.03 9 9 9c.83 0 1.5-.67 1.5-1.5 0-.39-.15-.74-.39-1.01-.23-.26-.38-.61-.38-.99 0-.83.67-1.5 1.5-1.5H16c2.76 0 5-2.24 5-5 0-4.42-4.03-8-9-8zm-5.5 9c-.83 0-1.5-.67-1.5-1.5S5.67 9 6.5 9 8 9.67 8 10.5 7.33 12 6.5 12zm3-4C8.67 8 8 7.33 8 6.5S8.67 5 9.5 5s1.5.67 1.5 1.5S10.33 8 9.5 8zm5 0c-.83 0-1.5-.67-1.5-1.5S13.67 5 14.5 5s1.5.67 1.5 1.5S15.33 8 14.5 8zm3 4c-.83 0-1.5-.67-1.5-1.5S16.67 9 17.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLooksTwo;
impl IconShape for MdLooksTwo {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 8c0 1.11-.9 2-2 2h-2v2h4v2H9v-4c0-1.11.9-2 2-2h2V9H9V7h4c1.1 0 2 .89 2 2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddToPhotos;
impl IconShape for MdAddToPhotos {
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
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-1 9h-4v4h-2v-4H9V9h4V5h2v4h4v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Md19mp;
impl IconShape for Md19mp {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 7h3V9h-2c-.55 0-1-.45-1-1V6.5c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4c0 .55-.45 1-1 1H12V10zm1.5-2H15V6.5h-1.5V8zM7 5.5h3v6H8.5V7H7V5.5zm5 13h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm6.5-2.5c0 .55-.45 1-1 1h-2v1.5H14v-6h3.5c.55 0 1 .45 1 1V16zm-3-2H17v1.5h-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRotate90DegreesCcw;
impl IconShape for MdRotate90DegreesCcw {
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
                d: "M7.34 6.41L.86 12.9l6.49 6.48 6.49-6.48-6.5-6.49zM3.69 12.9l3.66-3.66L11 12.9l-3.66 3.66-3.65-3.66zm15.67-6.26C17.61 4.88 15.3 4 13 4V.76L8.76 5 13 9.24V6c1.79 0 3.58.68 4.95 2.05 2.73 2.73 2.73 7.17 0 9.9C16.58 19.32 14.79 20 13 20c-.97 0-1.94-.21-2.84-.61l-1.49 1.49C10.02 21.62 11.51 22 13 22c2.3 0 4.61-.88 6.36-2.64 3.52-3.51 3.52-9.21 0-12.72z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLinkedCamera;
impl IconShape for MdLinkedCamera {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            circle {
                cx: "12",
                cy: "14",
                r: "3.2",
            }
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

#[derive(Copy, Clone, Debug)]
pub struct MdMotionPhotosOn;
impl IconShape for MdMotionPhotosOn {
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
                y: "0",
            }
            path {
                d: "M10,16.5v-9l6,4.5L10,16.5z M22,12c0,5.52-4.48,10-10,10S2,17.52,2,12c0-1.19,0.22-2.32,0.6-3.38L4.48,9.3 C4.17,10.14,4,11.05,4,12c0,4.41,3.59,8,8,8s8-3.59,8-8s-3.59-8-8-8c-0.95,0-1.85,0.17-2.69,0.48L8.63,2.59C9.69,2.22,10.82,2,12,2 C17.52,2,22,6.48,22,12z M5.5,4C4.67,4,4,4.67,4,5.5S4.67,7,5.5,7S7,6.33,7,5.5S6.33,4,5.5,4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBlurOn;
impl IconShape for MdBlurOn {
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
                d: "M6 13c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-8c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm-3 .5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM6 5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm15 5.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM14 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0-3.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zm-11 10c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm7 7c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm0-17c.28 0 .5-.22.5-.5s-.22-.5-.5-.5-.5.22-.5.5.22.5.5.5zM10 7c.55 0 1-.45 1-1s-.45-1-1-1-1 .45-1 1 .45 1 1 1zm0 5.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm8 .5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-8c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0-4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm3 8.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zM14 17c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm0 3.5c-.28 0-.5.22-.5.5s.22.5.5.5.5-.22.5-.5-.22-.5-.5-.5zm-4-12c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0 8.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1zm4-4.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0-4c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdViewCompact;
impl IconShape for MdViewCompact {
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
                d: "M3 19h6v-7H3v7zm7 0h12v-7H10v7zM3 5v6h19V5H3z",
            }
        }
    }
}
