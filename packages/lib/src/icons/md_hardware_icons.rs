use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MdDeviceUnknown;
impl IconShape for MdDeviceUnknown {
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
                d: "M17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14zM12 6.72c-1.96 0-3.5 1.52-3.5 3.47h1.75c0-.93.82-1.75 1.75-1.75s1.75.82 1.75 1.75c0 1.75-2.63 1.57-2.63 4.45h1.76c0-1.96 2.62-2.19 2.62-4.45 0-1.96-1.54-3.47-3.5-3.47zm-.88 8.8h1.76v1.76h-1.76z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardCapslock;
impl IconShape for MdKeyboardCapslock {
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
                d: "M12 8.41L16.59 13 18 11.59l-6-6-6 6L7.41 13 12 8.41zM6 18h12v-2H6v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTablet;
impl IconShape for MdTablet {
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
                d: "M21 4H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h18c1.1 0 1.99-.9 1.99-2L23 6c0-1.1-.9-2-2-2zm-2 14H5V6h14v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHeadset;
impl IconShape for MdHeadset {
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
                opacity: ".1",
            }
            path {
                d: "M12 1c-4.97 0-9 4.03-9 9v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-3.87 3.13-7 7-7s7 3.13 7 7v2h-4v8h3c1.66 0 3-1.34 3-3v-7c0-4.97-4.03-9-9-9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardHide;
impl IconShape for MdKeyboardHide {
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
                d: "M20 3H4c-1.1 0-1.99.9-1.99 2L2 15c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 3h2v2h-2V6zm0 3h2v2h-2V9zM8 6h2v2H8V6zm0 3h2v2H8V9zm-1 2H5V9h2v2zm0-3H5V6h2v2zm9 7H8v-2h8v2zm0-4h-2V9h2v2zm0-3h-2V6h2v2zm3 3h-2V9h2v2zm0-3h-2V6h2v2zm-7 15l4-4H8l4 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMonitor;
impl IconShape for MdMonitor {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M20 3H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h3l-1 1v2h12v-2l-1-1h3c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 13H4V5h16v11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDesktopMac;
impl IconShape for MdDesktopMac {
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
                d: "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7l-2 3v1h8v-1l-2-3h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 12H3V4h18v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMemory;
impl IconShape for MdMemory {
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
                d: "M15 9H9v6h6V9zm-2 4h-2v-2h2v2zm8-2V9h-2V7c0-1.1-.9-2-2-2h-2V3h-2v2h-2V3H9v2H7c-1.1 0-2 .9-2 2v2H3v2h2v2H3v2h2v2c0 1.1.9 2 2 2h2v2h2v-2h2v2h2v-2h2c1.1 0 2-.9 2-2v-2h2v-2h-2v-2h2zm-4 6H7V7h10v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardArrowLeft;
impl IconShape for MdKeyboardArrowLeft {
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
                d: "M15.41 16.59L10.83 12l4.58-4.59L14 6l-6 6 6 6 1.41-1.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHeadsetOff;
impl IconShape for MdHeadsetOff {
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
                d: "M12 4c3.87 0 7 3.13 7 7v2h-2.92L21 17.92V11c0-4.97-4.03-9-9-9-1.95 0-3.76.62-5.23 1.68l1.44 1.44C9.3 4.41 10.6 4 12 4zM2.27 1.72L1 3l3.33 3.32C3.49 7.68 3 9.29 3 11v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-1.17.29-2.26.79-3.22L15 17v4h3c.3 0 .59-.06.86-.14L21 23l1.27-1.27-20-20.01z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSimCard;
impl IconShape for MdSimCard {
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
                d: "M19.99 4c0-1.1-.89-2-1.99-2h-8L4 8v12c0 1.1.9 2 2 2h12.01c1.1 0 1.99-.9 1.99-2l-.01-16zM9 19H7v-2h2v2zm8 0h-2v-2h2v2zm-8-4H7v-4h2v4zm4 4h-2v-4h2v4zm0-6h-2v-2h2v2zm4 2h-2v-4h2v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdToys;
impl IconShape for MdToys {
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
                d: "M12 12c0-3 2.5-5.5 5.5-5.5S23 9 23 12H12zm0 0c0 3-2.5 5.5-5.5 5.5S1 15 1 12h11zm0 0c-3 0-5.5-2.5-5.5-5.5S9 1 12 1v11zm0 0c3 0 5.5 2.5 5.5 5.5S15 23 12 23V12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSecurity;
impl IconShape for MdSecurity {
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
                d: "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSpeaker;
impl IconShape for MdSpeaker {
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
                d: "M17 2H7c-1.1 0-2 .9-2 2v16c0 1.1.9 1.99 2 1.99L17 22c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5 2c1.1 0 2 .9 2 2s-.9 2-2 2c-1.11 0-2-.9-2-2s.89-2 2-2zm0 16c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSmartphone;
impl IconShape for MdSmartphone {
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
                d: "M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardVoice;
impl IconShape for MdKeyboardVoice {
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
                d: "M12 15c1.66 0 2.99-1.34 2.99-3L15 6c0-1.66-1.34-3-3-3S9 4.34 9 6v6c0 1.66 1.34 3 3 3zm5.3-3c0 3-2.54 5.1-5.3 5.1S6.7 15 6.7 12H5c0 3.42 2.72 6.23 6 6.72V22h2v-3.28c3.28-.48 6-3.3 6-6.72h-1.7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCast;
impl IconShape for MdCast {
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
                opacity: ".1",
            }
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v3h2V5h18v14h-7v2h7c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM1 18v3h3c0-1.66-1.34-3-3-3zm0-4v2c2.76 0 5 2.24 5 5h2c0-3.87-3.13-7-7-7zm0-4v2c4.97 0 9 4.03 9 9h2c0-6.08-4.93-11-11-11z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhonelinkOff;
impl IconShape for MdPhonelinkOff {
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
                d: "M22 6V4H6.82l2 2H22zM1.92 1.65L.65 2.92l1.82 1.82C2.18 5.08 2 5.52 2 6v11H0v3h17.73l2.35 2.35 1.27-1.27L3.89 3.62 1.92 1.65zM4 6.27L14.73 17H4V6.27zM23 8h-6c-.55 0-1 .45-1 1v4.18l2 2V10h4v7h-2.18l3 3H23c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhoneAndroid;
impl IconShape for MdPhoneAndroid {
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
                d: "M16 1H8C6.34 1 5 2.34 5 4v16c0 1.66 1.34 3 3 3h8c1.66 0 3-1.34 3-3V4c0-1.66-1.34-3-3-3zm-2 20h-4v-1h4v1zm3.25-3H6.75V4h10.5v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTabletAndroid;
impl IconShape for MdTabletAndroid {
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
                d: "M18,0H6C4.34,0,3,1.34,3,3v18c0,1.66,1.34,3,3,3h12c1.66,0,3-1.34,3-3V3C21,1.34,19.66,0,18,0z M14,22h-4v-1h4V22z M19.25,19H4.75V3h14.5V19z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGamepad;
impl IconShape for MdGamepad {
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
                d: "M15 7.5V2H9v5.5l3 3 3-3zM7.5 9H2v6h5.5l3-3-3-3zM9 16.5V22h6v-5.5l-3-3-3 3zM16.5 9l-3 3 3 3H22V9h-5.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPowerInput;
impl IconShape for MdPowerInput {
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
                d: "M2 9v2h19V9H2zm0 6h5v-2H2v2zm7 0h5v-2H9v2zm7 0h5v-2h-5v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLaptopWindows;
impl IconShape for MdLaptopWindows {
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
                d: "M20 18v-1c1.1 0 1.99-.9 1.99-2L22 5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2v1H0v2h24v-2h-4zM4 5h16v10H4V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSpeakerGroup;
impl IconShape for MdSpeakerGroup {
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
                d: "M18.2 1H9.8C8.81 1 8 1.81 8 2.8v14.4c0 .99.81 1.79 1.8 1.79l8.4.01c.99 0 1.8-.81 1.8-1.8V2.8c0-.99-.81-1.8-1.8-1.8zM14 3c1.1 0 2 .89 2 2s-.9 2-2 2-2-.89-2-2 .9-2 2-2zm0 13.5c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4z",
            }
            circle {
                cx: "14",
                cy: "12.5",
                r: "2.5",
            }
            path {
                d: "M6 5H4v16c0 1.1.89 2 2 2h10v-2H6V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardReturn;
impl IconShape for MdKeyboardReturn {
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
                d: "M19 7v4H5.83l3.58-3.59L8 6l-6 6 6 6 1.41-1.41L5.83 13H21V7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhonelink;
impl IconShape for MdPhonelink {
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
                d: "M4 6h18V4H4c-1.1 0-2 .9-2 2v11H0v3h14v-3H4V6zm19 2h-6c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zm-1 9h-4v-7h4v7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdVideogameAsset;
impl IconShape for MdVideogameAsset {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M0 0v24h24V0H0zm23 16c0 1.1-.9 2-2 2H3c-1.1 0-2-.9-2-2V8c0-1.1.9-2 2-2h18c1.1 0 2 .9 2 2v8z",
            }
            path {
                d: "M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTv;
impl IconShape for MdTv {
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMouse;
impl IconShape for MdMouse {
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
                d: "M13 1.07V9h7c0-4.08-3.05-7.44-7-7.93zM4 15c0 4.42 3.58 8 8 8s8-3.58 8-8v-4H4v4zm7-13.93C7.05 1.56 4 4.92 4 9h7V1.07z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrowserNotSupported;
impl IconShape for MdBrowserNotSupported {
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
                d: "M19,6v10.5l1.95,1.95C20.98,18.3,21,18.15,21,18V6c0-1.1-0.9-2-2-2H6.5l2,2H19z",
            }
            path {
                d: "M3.22,3.32L1.95,4.59L3,5.64L3,18c0,1.1,0.9,2,2,2h12.36l2.06,2.06l1.27-1.27L3.22,3.32z M15,18H5V7.64L15.36,18H15z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardArrowDown;
impl IconShape for MdKeyboardArrowDown {
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
                d: "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdTabletMac;
impl IconShape for MdTabletMac {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M18.5 0h-14C3.12 0 2 1.12 2 2.5v19C2 22.88 3.12 24 4.5 24h14c1.38 0 2.5-1.12 2.5-2.5v-19C21 1.12 19.88 0 18.5 0zm-7 23c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm7.5-4H4V3h15v16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDeveloperBoard;
impl IconShape for MdDeveloperBoard {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M22 9V7h-2V5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-2h2v-2h-2v-2h2v-2h-2V9h2zm-4 10H4V5h14v14zM6 13h5v4H6zm6-6h4v3h-4zM6 7h5v5H6zm6 4h4v6h-4z",
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardArrowUp;
impl IconShape for MdKeyboardArrowUp {
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
                d: "M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLaptopMac;
impl IconShape for MdLaptopMac {
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
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2H0c0 1.1.9 2 2 2h20c1.1 0 2-.9 2-2h-4zM4 5h16v11H4V5zm8 14c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPointOfSale;
impl IconShape for MdPointOfSale {
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
                d: "M17,2H7C5.9,2,5,2.9,5,4v2c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V4C19,2.9,18.1,2,17,2z M17,6H7V4h10V6z M20,22H4 c-1.1,0-2-0.9-2-2v-1h20v1C22,21.1,21.1,22,20,22z M18.53,10.19C18.21,9.47,17.49,9,16.7,9H7.3c-0.79,0-1.51,0.47-1.83,1.19L2,18 h20L18.53,10.19z M9.5,16h-1C8.22,16,8,15.78,8,15.5C8,15.22,8.22,15,8.5,15h1c0.28,0,0.5,0.22,0.5,0.5C10,15.78,9.78,16,9.5,16z M9.5,14h-1C8.22,14,8,13.78,8,13.5C8,13.22,8.22,13,8.5,13h1c0.28,0,0.5,0.22,0.5,0.5C10,13.78,9.78,14,9.5,14z M9.5,12h-1 C8.22,12,8,11.78,8,11.5C8,11.22,8.22,11,8.5,11h1c0.28,0,0.5,0.22,0.5,0.5C10,11.78,9.78,12,9.5,12z M12.5,16h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,15.78,12.78,16,12.5,16z M12.5,14h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,13.78,12.78,14,12.5,14z M12.5,12h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,11.78,12.78,12,12.5,12z M15.5,16h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,15.78,15.78,16,15.5,16z M15.5,14h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,13.78,15.78,14,15.5,14z M15.5,12h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,11.78,15.78,12,15.5,12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdHeadsetMic;
impl IconShape for MdHeadsetMic {
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
                opacity: ".1",
            }
            path {
                d: "M12 1c-4.97 0-9 4.03-9 9v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-3.87 3.13-7 7-7s7 3.13 7 7v2h-4v8h4v1h-7v2h6c1.66 0 3-1.34 3-3V10c0-4.97-4.03-9-9-9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCastConnected;
impl IconShape for MdCastConnected {
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
                opacity: ".1",
            }
            path {
                d: "M1 18v3h3c0-1.66-1.34-3-3-3zm0-4v2c2.76 0 5 2.24 5 5h2c0-3.87-3.13-7-7-7zm18-7H5v1.63c3.96 1.28 7.09 4.41 8.37 8.37H19V7zM1 10v2c4.97 0 9 4.03 9 9h2c0-6.08-4.93-11-11-11zm20-7H3c-1.1 0-2 .9-2 2v3h2V5h18v14h-7v2h7c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWatch;
impl IconShape for MdWatch {
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
                opacity: ".1",
            }
            path {
                d: "M20 12c0-2.54-1.19-4.81-3.04-6.27L16 0H8l-.95 5.73C5.19 7.19 4 9.45 4 12s1.19 4.81 3.05 6.27L8 24h8l.96-5.73C18.81 16.81 20 14.54 20 12zM6 12c0-3.31 2.69-6 6-6s6 2.69 6 6-2.69 6-6 6-6-2.69-6-6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLaptopChromebook;
impl IconShape for MdLaptopChromebook {
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
                d: "M22 18V3H2v15H0v2h24v-2h-2zm-8 0h-4v-1h4v1zm6-3H4V5h16v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdCastForEducation;
impl IconShape for MdCastForEducation {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M21,3H3C1.9,3,1,3.9,1,5v3h2V5h18v14h-7v2h7c1.1,0,2-0.9,2-2V5C23,3.9,22.1,3,21,3z M1,18v3h3C4,19.34,2.66,18,1,18z M1,14 v2c2.76,0,5,2.24,5,5h2C8,17.13,4.87,14,1,14z M1,10v2c4.97,0,9,4.03,9,9h2C12,14.92,7.07,10,1,10z M11,11.09v2L14.5,15l3.5-1.91 v-2L14.5,13L11,11.09z M14.5,6L9,9l5.5,3L20,9L14.5,6z",
            }
            path {
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDesktopWindows;
impl IconShape for MdDesktopWindows {
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
                d: "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7v2H8v2h8v-2h-2v-2h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H3V4h18v12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdPhoneIphone;
impl IconShape for MdPhoneIphone {
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
                d: "M15.5 1h-8C6.12 1 5 2.12 5 3.5v17C5 21.88 6.12 23 7.5 23h8c1.38 0 2.5-1.12 2.5-2.5v-17C18 2.12 16.88 1 15.5 1zm-4 21c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4.5-4H7V4h9v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardTab;
impl IconShape for MdKeyboardTab {
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
                d: "M11.59 7.41L15.17 11H1v2h14.17l-3.59 3.59L13 18l6-6-6-6-1.41 1.41zM20 6v12h2V6h-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLaptop;
impl IconShape for MdLaptop {
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
                d: "M20,18c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v10c0,1.1,0.9,2,2,2H0v2h24v-2H20z M4,6h16v10H4V6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDeviceHub;
impl IconShape for MdDeviceHub {
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
                d: "M17 16l-4-4V8.82C14.16 8.4 15 7.3 15 6c0-1.66-1.34-3-3-3S9 4.34 9 6c0 1.3.84 2.4 2 2.82V12l-4 4H3v5h5v-3.05l4-4.2 4 4.2V21h5v-5h-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardBackspace;
impl IconShape for MdKeyboardBackspace {
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
                d: "M21 11H6.83l3.58-3.59L9 6l-6 6 6 6 1.41-1.41L6.83 13H21z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdComputer;
impl IconShape for MdComputer {
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
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScanner;
impl IconShape for MdScanner {
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
                d: "M19.8 10.7L4.2 5l-.7 1.9L17.6 12H5c-1.1 0-2 .9-2 2v4c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-5.5c0-.8-.5-1.6-1.2-1.8zM7 17H5v-2h2v2zm12 0H9v-2h10v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDevicesOther;
impl IconShape for MdDevicesOther {
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
                d: "M3 6h18V4H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4v-2H3V6zm10 6H9v1.78c-.61.55-1 1.33-1 2.22s.39 1.67 1 2.22V20h4v-1.78c.61-.55 1-1.34 1-2.22s-.39-1.67-1-2.22V12zm-2 5.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM22 8h-6c-.5 0-1 .5-1 1v10c0 .5.5 1 1 1h6c.5 0 1-.5 1-1V9c0-.5-.5-1-1-1zm-1 10h-4v-8h4v8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdConnectedTv;
impl IconShape for MdConnectedTv {
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12zM4 14v2h2c0-1.11-.89-2-2-2zm0-3v1.43c1.97 0 3.57 1.6 3.57 3.57H9c0-2.76-2.24-5-5-5zm0-3v1.45c3.61 0 6.55 2.93 6.55 6.55H12c0-4.42-3.59-8-8-8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboard;
impl IconShape for MdKeyboard {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M20 5H4c-1.1 0-1.99.9-1.99 2L2 17c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm-9 3h2v2h-2V8zm0 3h2v2h-2v-2zM8 8h2v2H8V8zm0 3h2v2H8v-2zm-1 2H5v-2h2v2zm0-3H5V8h2v2zm9 7H8v-2h8v2zm0-4h-2v-2h2v2zm0-3h-2V8h2v2zm3 3h-2v-2h2v2zm0-3h-2V8h2v2z",
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDock;
impl IconShape for MdDock {
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
                d: "M8 23h8v-2H8v2zm8-21.99L8 1c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM16 15H8V5h8v10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdKeyboardArrowRight;
impl IconShape for MdKeyboardArrowRight {
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
                d: "M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdRouter;
impl IconShape for MdRouter {
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
                d: "M20.2 5.9l.8-.8C19.6 3.7 17.8 3 16 3s-3.6.7-5 2.1l.8.8C13 4.8 14.5 4.2 16 4.2s3 .6 4.2 1.7zm-.9.8c-.9-.9-2.1-1.4-3.3-1.4s-2.4.5-3.3 1.4l.8.8c.7-.7 1.6-1 2.5-1 .9 0 1.8.3 2.5 1l.8-.8zM19 13h-2V9h-2v4H5c-1.1 0-2 .9-2 2v4c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-4c0-1.1-.9-2-2-2zM8 18H6v-2h2v2zm3.5 0h-2v-2h2v2zm3.5 0h-2v-2h2v2z",
            }
        }
    }
}
