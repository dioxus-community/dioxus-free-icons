use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoMode;
impl IconShape for MdAutoMode {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.03,3.56c-1.67-1.39-3.74-2.3-6.03-2.51v2.01c1.73,0.19,3.31,0.88,4.61,1.92L19.03,3.56z",
            }
            path {
                d: "M11,3.06V1.05C8.71,1.25,6.64,2.17,4.97,3.56l1.42,1.42C7.69,3.94,9.27,3.25,11,3.06z",
            }
            path {
                d: "M4.98,6.39L3.56,4.97C2.17,6.64,1.26,8.71,1.05,11h2.01C3.25,9.27,3.94,7.69,4.98,6.39z",
            }
            path {
                d: "M20.94,11h2.01c-0.21-2.29-1.12-4.36-2.51-6.03l-1.42,1.42C20.06,7.69,20.75,9.27,20.94,11z",
            }
            polygon {
                points: "7,12 10.44,13.56 12,17 13.56,13.56 17,12 13.56,10.44 12,7 10.44,10.44",
            }
            path {
                d: "M12,21c-3.11,0-5.85-1.59-7.46-4H7v-2H1v6h2v-2.7c1.99,2.84,5.27,4.7,9,4.7c4.87,0,9-3.17,10.44-7.56l-1.96-0.45 C19.25,18.48,15.92,21,12,21z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlinds;
impl IconShape for MdBlinds {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M16,9h2v2h-2V9z M14,11H6V9h8V11z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-6h8v1.82 c-0.45,0.32-0.75,0.84-0.75,1.43c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h2v6H6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlindsClosed;
impl IconShape for MdBlindsClosed {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h11.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M18,11h-2V9h2V11z M14,11H6V9h8V11z M14,13v2H6v-2H14z M16,13h2v2h-2V13z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-2h8v2H6z M16,19v-2h2v2H16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBroadcastOnHome;
impl IconShape for MdBroadcastOnHome {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,6c0-1.1-0.9-2-2-2H4v2h16v2.59c0.73,0.29,1.4,0.69,2,1.17V6z",
            }
            path {
                d: "M8,9H3c-0.5,0-1,0.5-1,1v9c0,0.5,0.5,1,1,1h5c0.5,0,1-0.5,1-1v-9C9,9.5,8.5,9,8,9z M7,18H4v-7h3V18z",
            }
            path {
                d: "M17.75,16.97c0.3-0.23,0.5-0.57,0.5-0.97c0-0.69-0.56-1.25-1.25-1.25s-1.25,0.56-1.25,1.25c0,0.4,0.2,0.75,0.5,0.97V22 h1.5V16.97z",
            }
            path {
                d: "M17,13.5c1.38,0,2.5,1.12,2.5,2.5c0,0.69-0.28,1.31-0.73,1.76l1.06,1.06C20.55,18.1,21,17.1,21,16c0-2.21-1.79-4-4-4 c-2.21,0-4,1.79-4,4c0,1.1,0.45,2.1,1.17,2.83l1.06-1.06c-0.45-0.45-0.73-1.08-0.73-1.77C14.5,14.62,15.62,13.5,17,13.5z",
            }
            path {
                d: "M17,9.5c-3.59,0-6.5,2.91-6.5,6.5c0,1.79,0.73,3.42,1.9,4.6l1.06-1.06C12.56,18.63,12,17.38,12,16c0-2.76,2.24-5,5-5 s5,2.24,5,5c0,1.37-0.56,2.62-1.46,3.52l1.07,1.06c1.17-1.18,1.89-2.8,1.89-4.58C23.5,12.41,20.59,9.5,17,9.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBroadcastOnPersonal;
impl IconShape for MdBroadcastOnPersonal {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,8c0.7,0,1.38,0.1,2.02,0.27L12,3L4,9v12h6.76C9.66,19.63,9,17.89,9,16C9,11.58,12.58,8,17,8z",
            }
            path {
                d: "M17,14.75c-0.69,0-1.25,0.56-1.25,1.25c0,0.4,0.2,0.75,0.5,0.97V22h1.5v-5.03c0.3-0.23,0.5-0.57,0.5-0.97 C18.25,15.31,17.69,14.75,17,14.75z",
            }
            path {
                d: "M17,12c-2.21,0-4,1.79-4,4c0,1.1,0.45,2.1,1.17,2.83l1.06-1.06c-0.45-0.45-0.73-1.08-0.73-1.77c0-1.38,1.12-2.5,2.5-2.5 s2.5,1.12,2.5,2.5c0,0.69-0.28,1.31-0.73,1.76l1.06,1.06C20.55,18.1,21,17.1,21,16C21,13.79,19.21,12,17,12z",
            }
            path {
                d: "M17,9.5c-3.59,0-6.5,2.91-6.5,6.5c0,1.79,0.73,3.42,1.9,4.6l1.06-1.06C12.56,18.63,12,17.38,12,16c0-2.76,2.24-5,5-5 s5,2.24,5,5c0,1.37-0.56,2.62-1.46,3.52l1.07,1.06c1.17-1.18,1.89-2.8,1.89-4.58C23.5,12.41,20.59,9.5,17,9.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudySnowing;
impl IconShape for MdCloudySnowing {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S5,17.45,5,18z M17,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1 S17,17.45,17,18z M8,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S8,21.45,8,22z M11,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1 S11,17.45,11,18z M14,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S14,21.45,14,22z M17.5,16h-10C4.47,16,2,13.53,2,10.5 c0-2.76,2.09-5.09,4.78-5.44C7.83,3.18,9.82,2,12,2c2.97,0,5.45,2.18,5.92,5.02C20.21,7.23,22,9.16,22,11.5 C22,13.98,19.98,16,17.5,16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCurtains;
impl IconShape for MdCurtains {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M8.19,12c2.04-1.35,3.5-3.94,3.76-7h0.09c0.26,3.06,1.72,5.65,3.76,7 c-2.04,1.35-3.5,3.94-3.76,7h-0.09C11.69,15.94,10.23,13.35,8.19,12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCurtainsClosed;
impl IconShape for MdCurtainsClosed {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M11,5h2v14h-2V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricBolt;
impl IconShape for MdElectricBolt {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.69,2.21L4.33,11.49c-0.64,0.58-0.28,1.65,0.58,1.73L13,14l-4.85,6.76c-0.22,0.31-0.19,0.74,0.08,1.01h0 c0.3,0.3,0.77,0.31,1.08,0.02l10.36-9.28c0.64-0.58,0.28-1.65-0.58-1.73L11,10l4.85-6.76c0.22-0.31,0.19-0.74-0.08-1.01l0,0 C15.47,1.93,15,1.92,14.69,2.21z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricMeter;
impl IconShape for MdElectricMeter {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,2c-4.96,0-9,4.04-9,9c0,3.91,2.51,7.24,6,8.47V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V22h2v-2.53 c3.49-1.24,6-4.57,6-8.47C21,6.04,16.96,2,12,2z M14.25,14l-3,3l-1.5-1.5L11,14.25L9.75,13l3-3l1.5,1.5L13,12.75L14.25,14z M16,9H8 V7h8V9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEnergySavingsLeaf;
impl IconShape for MdEnergySavingsLeaf {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12l0-9L12,3z M15.83,12.26l-5.16,4.63c-0.16,0.15-0.41,0.14-0.56-0.01 c-0.14-0.14-0.16-0.36-0.04-0.52l2.44-3.33l-4.05-0.4c-0.44-0.04-0.63-0.59-0.3-0.89l5.16-4.63c0.16-0.15,0.41-0.14,0.56,0.01 c0.14,0.14,0.16,0.36,0.04,0.52l-2.44,3.33l4.05,0.4C15.98,11.41,16.16,11.96,15.83,12.26z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFoggy;
impl IconShape for MdFoggy {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.92,7.02C17.45,4.18,14.97,2,12,2C9.82,2,7.83,3.18,6.78,5.06C4.09,5.41,2,7.74,2,10.5C2,13.53,4.47,16,7.5,16h10 c2.48,0,4.5-2.02,4.5-4.5C22,9.16,20.21,7.23,17.92,7.02z M18,17.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S17.45,17.01,18,17.01z M7,20.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S6.45,20.01,7,20.01z M6,17.01h9c0.55,0,1,0.45,1,1l0,0 c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1l0,0C5,17.46,5.45,17.01,6,17.01z M10,20.01h7c0.55,0,1,0.45,1,1l0,0c0,0.55-0.45,1-1,1h-7 c-0.55,0-1-0.45-1-1l0,0C9,20.46,9.45,20.01,10,20.01z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGasMeter;
impl IconShape for MdGasMeter {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16,4h-1V2h-2v2h-2V2H9v2H8C5.79,4,4,5.79,4,8v10c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4V8C20,5.79,18.21,4,16,4z M12,18 c-1.38,0-2.5-1.1-2.5-2.46c0-1.09,0.43-1.39,2.5-3.79c2.05,2.38,2.5,2.7,2.5,3.79C14.5,16.9,13.38,18,12,18z M16,10H8V8h8V10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHeatPump;
impl IconShape for MdHeatPump {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12.75,7.08 c0.82,0.12,1.57,0.44,2.2,0.91l-2.2,2.2V7.08z M11.25,7.08v3.11l-2.2-2.2C9.68,7.52,10.43,7.2,11.25,7.08z M7.99,9.05l2.2,2.2H7.08 C7.2,10.43,7.52,9.68,7.99,9.05z M7.08,12.75h3.11l-2.2,2.2C7.52,14.32,7.2,13.57,7.08,12.75z M11.25,16.92 c-0.82-0.12-1.57-0.44-2.2-0.91l2.2-2.2V16.92z M12,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C13,12.55,12.55,13,12,13z M12.75,16.92v-3.11l2.2,2.2C14.32,16.48,13.57,16.8,12.75,16.92z M16.01,14.95l-2.2-2.2h3.11C16.8,13.57,16.48,14.32,16.01,14.95z M13.81,11.25l2.2-2.2c0.47,0.64,0.79,1.39,0.91,2.2H13.81z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdModeFanOff;
impl IconShape for MdModeFanOff {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.34,8.36l-2.29,0.82c-0.18-0.13-0.38-0.25-0.58-0.34c0.17-0.83,0.63-1.58,1.36-2.06C16.85,5.44,16.18,2,13.39,2 c-3.08,0-4.9,1.47-5.3,3.26L18.73,15.9c1.5,0.39,3.27-0.51,3.27-2.51C22,9,18.99,7.16,16.34,8.36z",
            }
            path {
                d: "M2.81,2.81L1.39,4.22L5.27,8.1C3.77,7.7,2,8.61,2,10.61c0,4.4,3.01,6.24,5.66,5.03l2.29-0.82 c0.18,0.13,0.38,0.25,0.58,0.34c-0.17,0.83-0.63,1.58-1.36,2.06C7.15,18.56,7.82,22,10.61,22c3.08,0,4.9-1.47,5.3-3.26l3.87,3.87 l1.41-1.41L2.81,2.81z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNestCamWiredStand;
impl IconShape for MdNestCamWiredStand {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.83,1.01l-4.11,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85C12.71,15.04,12.36,15,12,15 c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45 C17.02,14.56,18,13.66,18,12.5V3C18,1.83,17,0.91,15.83,1.01z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOilBarrel;
impl IconShape for MdOilBarrel {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M12,16 c-1.66,0-3-1.32-3-2.95c0-1.3,0.52-1.67,3-4.55c2.47,2.86,3,3.24,3,4.55C15,14.68,13.66,16,12,16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPropane;
impl IconShape for MdPropane {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,6h-1V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1H7c-3.31,0-6,2.69-6,6s2.69,6,6,6v3h2v-3h6v3h2v-3c3.31,0,6-2.69,6-6 S20.31,6,17,6z M10,5h4v1h-4V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPropaneTank;
impl IconShape for MdPropaneTank {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4,15v3c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-3H4z",
            }
            path {
                d: "M20,13v-3c0-1.86-1.28-3.41-3-3.86V4c0-1.1-0.9-2-2-2H9C7.9,2,7,2.9,7,4v2.14c-1.72,0.45-3,2-3,3.86v3H20z M9,4h6v2h-2 c0-0.55-0.45-1-1-1s-1,0.45-1,1H9V4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRollerShades;
impl IconShape for MdRollerShades {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M6,19v-6h5v1.8c-0.4,0.3-0.8,0.8-0.8,1.4c0,1,0.8,1.8,1.8,1.8s1.8-0.8,1.8-1.8 c0-0.6-0.3-1.1-0.8-1.4V13h5v6H6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRollerShadesClosed;
impl IconShape for MdRollerShadesClosed {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M6,19v-2h5v2H6z M13,19v-2h5v2H13z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorDoor;
impl IconShape for MdSensorDoor {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 S17,11.17,17,12S16.33,13.5,15.5,13.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorOccupied;
impl IconShape for MdSensorOccupied {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,11c1.66,0,3-1.34,3-3s-1.34-3-3-3S9,6.34,9,8S10.34,11,12,11z",
            }
            path {
                d: "M12,12c-1.84,0-3.56,0.5-5.03,1.37C6.36,13.72,6,14.39,6,15.09V17h12v-1.91c0-0.7-0.36-1.36-0.97-1.72 C15.56,12.5,13.84,12,12,12z",
            }
            path {
                d: "M21.23,8.15l1.85-0.77c-1.22-2.91-3.55-5.25-6.46-6.46l-0.77,1.85C18.27,3.79,20.21,5.73,21.23,8.15z",
            }
            path {
                d: "M8.15,2.77L7.38,0.92C4.47,2.14,2.14,4.47,0.92,7.38l1.85,0.77C3.79,5.73,5.73,3.79,8.15,2.77z",
            }
            path {
                d: "M2.77,15.85l-1.85,0.77c1.22,2.91,3.55,5.25,6.46,6.46l0.77-1.85C5.73,20.21,3.79,18.27,2.77,15.85z",
            }
            path {
                d: "M15.85,21.23l0.77,1.85c2.91-1.22,5.25-3.55,6.46-6.46l-1.85-0.77C20.21,18.27,18.27,20.21,15.85,21.23z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorWindow;
impl IconShape for MdSensorWindow {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2L18,2z M7,19h10v-6H7 V19z M10,10h4v1h3V5H7v6h3V10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShelves;
impl IconShape for MdShelves {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 1v2H5V1H3v22h2v-2h14v2h2V1h-2zm0 4v6h-6V7H7v4H5V5h14zm-2 14v-4h-6v4H5v-6h14v6h-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShieldMoon;
impl IconShape for MdShieldMoon {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M15.97,14.41c-1.84,2.17-5.21,2.1-6.96-0.07 c-2.19-2.72-0.65-6.72,2.69-7.33c0.34-0.06,0.63,0.27,0.51,0.6c-0.46,1.23-0.39,2.64,0.32,3.86c0.71,1.22,1.89,1.99,3.18,2.2 C16.05,13.72,16.2,14.14,15.97,14.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSnowing;
impl IconShape for MdSnowing {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 12.75a1.25 1.25 0 110 2.5 1.25 1.25 0 010-2.5zM4.75 6a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm12 8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm-9 12a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 12a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSolarPower;
impl IconShape for MdSolarPower {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "3.33,16 11,16 11,13 4,13",
            }
            polygon {
                points: "13,16 20.67,16 20,13 13,13",
            }
            polygon {
                points: "21.11,18 13,18 13,22 22,22",
            }
            polygon {
                points: "2,22 11,22 11,18 2.89,18",
            }
            rect {
                height: "3",
                width: "2",
                x: "11",
                y: "8",
            }
            rect {
                height: "3",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.2089 14.6085)",
                width: "2",
                x: "16.53",
                y: "6.06",
            }
            rect {
                height: "2",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.448 6.7885)",
                width: "3",
                x: "4.97",
                y: "6.56",
            }
            rect {
                height: "2",
                width: "3",
                x: "3",
                y: "2",
            }
            rect {
                height: "2",
                width: "3",
                x: "18",
                y: "2",
            }
            path {
                d: "M12,7c2.76,0,5-2.24,5-5H7C7,4.76,9.24,7,12,7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSunny;
impl IconShape for MdSunny {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11,4V2c0-0.55,0.45-1,1-1s1,0.45,1,1v2c0,0.55-0.45,1-1,1S11,4.55,11,4z M18.36,7.05l1.41-1.42c0.39-0.39,0.39-1.02,0-1.41 c-0.39-0.39-1.02-0.39-1.41,0l-1.41,1.42c-0.39,0.39-0.39,1.02,0,1.41C17.34,7.44,17.97,7.44,18.36,7.05z M22,11h-2 c-0.55,0-1,0.45-1,1s0.45,1,1,1h2c0.55,0,1-0.45,1-1S22.55,11,22,11z M12,19c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1s1-0.45,1-1v-2 C13,19.45,12.55,19,12,19z M5.64,7.05L4.22,5.64c-0.39-0.39-0.39-1.03,0-1.41s1.03-0.39,1.41,0l1.41,1.41 c0.39,0.39,0.39,1.03,0,1.41S6.02,7.44,5.64,7.05z M16.95,16.95c-0.39,0.39-0.39,1.03,0,1.41l1.41,1.41c0.39,0.39,1.03,0.39,1.41,0 c0.39-0.39,0.39-1.03,0-1.41l-1.41-1.41C17.98,16.56,17.34,16.56,16.95,16.95z M2,13h2c0.55,0,1-0.45,1-1s-0.45-1-1-1H2 c-0.55,0-1,0.45-1,1S1.45,13,2,13z M5.64,19.78l1.41-1.41c0.39-0.39,0.39-1.03,0-1.41s-1.03-0.39-1.41,0l-1.41,1.41 c-0.39,0.39-0.39,1.03,0,1.41C4.61,20.17,5.25,20.17,5.64,19.78z M12,6c-3.31,0-6,2.69-6,6s2.69,6,6,6s6-2.69,6-6S15.31,6,12,6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSunnySnowing;
impl IconShape for MdSunnySnowing {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 5h-2V1h2v4zM1 11h4v2H1v-2zm18 2v-2h4v2h-4zm-1.34-5.24-1.41-1.41 2.83-2.83 1.41 1.41-2.83 2.83zM3.51 4.93l1.41-1.41 2.83 2.83-1.41 1.41-2.83-2.83zM4.75 17a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm12 0a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm-9 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3-4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zM17 13v-1c0-2.76-2.24-5-5-5s-5 2.24-5 5v1h10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalShades;
impl IconShape for MdVerticalShades {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M10,19V5h4v14H10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalShadesClosed;
impl IconShape for MdVerticalShadesClosed {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,19V3H4v16H2v2h20v-2H20z M13,5h1.5v14H13V5z M11,19H9.5V5H11V19z M6,5h1.5v14H6V5z M16.5,19V5H18v14H16.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWindPower;
impl IconShape for MdWindPower {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            rect {
                height: "2",
                width: "6",
                x: "4",
                y: "3",
            }
            rect {
                height: "2",
                width: "5",
                x: "1",
                y: "7",
            }
            rect {
                height: "2",
                width: "5",
                x: "3",
                y: "19",
            }
            path {
                d: "M13.73,10.61c0.75,0.23,1.3,0.78,1.57,1.46l4.27-7.11c0.65-1.08,0.3-2.48-0.78-3.13c-0.87-0.52-1.99-0.41-2.73,0.29 l-3.43,3.21C12.23,5.7,12,6.23,12,6.78v3.93C12.36,10.56,12.98,10.38,13.73,10.61z",
            }
            path {
                d: "M10.61,12.27c0.16-0.52,0.48-0.96,0.89-1.27H3.28C2.02,11,1,12.02,1,13.28c0,1.02,0.67,1.91,1.65,2.19l4.51,1.29 c0.53,0.15,1.1,0.08,1.58-0.21l2.69-1.61C10.66,14.32,10.3,13.27,10.61,12.27z",
            }
            path {
                d: "M22.21,18.61l-2.28-4.1c-0.27-0.48-0.73-0.83-1.26-0.97l-3.18-0.8c0.03,0.32,0,0.66-0.1,0.99 c-0.32,1.06-1.28,1.77-2.39,1.77c-0.61,0-0.99-0.22-1-0.22V21c-1.1,0-2,0.9-2,2h6c0-1.1-0.9-2-2-2v-4.28l4.61,4.61 c0.89,0.89,2.33,0.89,3.22,0C22.55,20.61,22.71,19.5,22.21,18.61z",
            }
            path {
                d: "M12.56,14.43c0.79,0.24,1.63-0.2,1.87-1c0.24-0.79-0.2-1.63-1-1.87c-0.79-0.24-1.63,0.2-1.87,1 C11.32,13.35,11.77,14.19,12.56,14.43z",
            }
        }
    }
}
