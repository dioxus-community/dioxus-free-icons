use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MdBluetoothSearching;
impl IconShape for MdBluetoothSearching {
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
                d: "M14.24 12.01l2.32 2.32c.28-.72.44-1.51.44-2.33 0-.82-.16-1.59-.43-2.31l-2.33 2.32zm5.29-5.3l-1.26 1.26c.63 1.21.98 2.57.98 4.02s-.36 2.82-.98 4.02l1.2 1.2c.97-1.54 1.54-3.36 1.54-5.31-.01-1.89-.55-3.67-1.48-5.19zm-3.82 1L10 2H9v7.59L4.41 5 3 6.41 8.59 12 3 17.59 4.41 19 9 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM11 5.83l1.88 1.88L11 9.59V5.83zm1.88 10.46L11 18.17v-3.76l1.88 1.88z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalWifiOff;
impl IconShape for MdSignalWifiOff {
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
                d: "M23.64 7c-.45-.34-4.93-4-11.64-4-1.5 0-2.89.19-4.15.48L18.18 13.8 23.64 7zm-6.6 8.22L3.27 1.44 2 2.72l2.05 2.06C1.91 5.76.59 6.82.36 7l11.63 14.49.01.01.01-.01 3.9-4.86 3.32 3.32 1.27-1.27-3.46-3.46z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAccessAlarm;
impl IconShape for MdAccessAlarm {
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
                d: "M22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM12.5 8H11v6l4.75 2.85.75-1.23-4-2.37V8zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLocationDisabled;
impl IconShape for MdLocationDisabled {
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
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06c-1.13.12-2.19.46-3.16.97l1.5 1.5C10.16 5.19 11.06 5 12 5c3.87 0 7 3.13 7 7 0 .94-.19 1.84-.52 2.65l1.5 1.5c.5-.96.84-2.02.97-3.15H23v-2h-2.06zM3 4.27l2.04 2.04C3.97 7.62 3.25 9.23 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c1.77-.2 3.38-.91 4.69-1.98L19.73 21 21 19.73 4.27 3 3 4.27zm13.27 13.27C15.09 18.45 13.61 19 12 19c-3.87 0-7-3.13-7-7 0-1.61.55-3.09 1.46-4.27l9.81 9.81z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightnessHigh;
impl IconShape for MdBrightnessHigh {
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
pub struct MdBatteryFull;
impl IconShape for MdBatteryFull {
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
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdLocationSearching;
impl IconShape for MdLocationSearching {
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
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAccessAlarms;
impl IconShape for MdAccessAlarms {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M-618-568H782v3600H-618zM0 0h24v24H0z",
            }
            path {
                d: "M22 5.7l-4.6-3.9-1.3 1.5 4.6 3.9L22 5.7zM7.9 3.4L6.6 1.9 2 5.7l1.3 1.5 4.6-3.8zM12.5 8H11v6l4.7 2.9.8-1.2-4-2.4V8zM12 4c-5 0-9 4-9 9s4 9 9 9 9-4 9-9-4-9-9-9zm0 16c-3.9 0-7-3.1-7-7s3.1-7 7-7 7 3.1 7 7-3.1 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightnessLow;
impl IconShape for MdBrightnessLow {
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
pub struct MdSdStorage;
impl IconShape for MdSdStorage {
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
                d: "M18 2h-8L4.02 8 4 20c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 6h-2V4h2v4zm3 0h-2V4h2v4zm3 0h-2V4h2v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightnessAuto;
impl IconShape for MdBrightnessAuto {
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
                d: "M10.85 12.65h2.3L12 9l-1.15 3.65zM20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM14.3 16l-.7-2h-3.2l-.7 2H7.8L11 7h2l3.2 9h-1.9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAirplanemodeActive;
impl IconShape for MdAirplanemodeActive {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M22,16v-2l-8.5-5V3.5C13.5,2.67,12.83,2,12,2s-1.5,0.67-1.5,1.5V9L2,14v2l8.5-2.5V19L8,20.5L8,22l4-1l4,1l0-1.5L13.5,19 v-5.5L22,16z",
            }
            path {
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNetworkWifi;
impl IconShape for MdNetworkWifi {
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
                d: "M24,8.98C20.93,5.9,16.69,4,12,4C7.31,4,3.07,5.9,0,8.98L12,21v0l0,0L24,8.98z M2.92,9.07C5.51,7.08,8.67,6,12,6 s6.49,1.08,9.08,3.07l-1.43,1.43C17.5,8.94,14.86,8,12,8s-5.5,0.94-7.65,2.51L2.92,9.07z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDataUsage;
impl IconShape for MdDataUsage {
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
                d: "M13 2.05v3.03c3.39.49 6 3.39 6 6.92 0 .9-.18 1.75-.48 2.54l2.6 1.53c.56-1.24.88-2.62.88-4.07 0-5.18-3.95-9.45-9-9.95zM12 19c-3.87 0-7-3.13-7-7 0-3.53 2.61-6.43 6-6.92V2.05c-5.06.5-9 4.76-9 9.95 0 5.52 4.47 10 9.99 10 3.31 0 6.24-1.61 8.06-4.09l-2.6-1.53C16.17 17.98 14.21 19 12 19z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellular4Bar;
impl IconShape for MdSignalCellular4Bar {
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
                d: "M2 22h20V2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGpsNotFixed;
impl IconShape for MdGpsNotFixed {
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
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellularNull;
impl IconShape for MdSignalCellularNull {
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
                d: "M20 6.83V20H6.83L20 6.83M22 2L2 22h20V2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSettingsSystemDaydream;
impl IconShape for MdSettingsSystemDaydream {
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
                d: "M9 16h6.5c1.38 0 2.5-1.12 2.5-2.5S16.88 11 15.5 11h-.05c-.24-1.69-1.69-3-3.45-3-1.4 0-2.6.83-3.16 2.02h-.16C7.17 10.18 6 11.45 6 13c0 1.66 1.34 3 3 3zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScreenLockPortrait;
impl IconShape for MdScreenLockPortrait {
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
                d: "M10 16h4c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1v-1c0-1.11-.9-2-2-2-1.11 0-2 .9-2 2v1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1zm.8-6c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2v1h-2.4v-1zM17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGraphicEq;
impl IconShape for MdGraphicEq {
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
                d: "M7 18h2V6H7v12zm4 4h2V2h-2v20zm-8-8h2v-4H3v4zm12 4h2V6h-2v12zm4-8v4h2v-4h-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScreenRotation;
impl IconShape for MdScreenRotation {
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
                d: "M16.48 2.52c3.27 1.55 5.61 4.72 5.97 8.48h1.5C23.44 4.84 18.29 0 12 0l-.66.03 3.81 3.81 1.33-1.32zm-6.25-.77c-.59-.59-1.54-.59-2.12 0L1.75 8.11c-.59.59-.59 1.54 0 2.12l12.02 12.02c.59.59 1.54.59 2.12 0l6.36-6.36c.59-.59.59-1.54 0-2.12L10.23 1.75zm4.6 19.44L2.81 9.17l6.36-6.36 12.02 12.02-6.36 6.36zm-7.31.29C4.25 19.94 1.91 16.76 1.55 13H.05C.56 19.16 5.71 24 12 24l.66-.03-3.81-3.81-1.33 1.32z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellular0Bar;
impl IconShape for MdSignalCellular0Bar {
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
                d: "M20,6.83V20H6.83L20,6.83 M22,2L2,22h20V2L22,2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMobileFriendly;
impl IconShape for MdMobileFriendly {
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
                d: "M19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zM7.01 13.47l-2.55-2.55-1.27 1.27L7 16l7.19-7.19-1.27-1.27z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGpsOff;
impl IconShape for MdGpsOff {
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
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06c-1.13.12-2.19.46-3.16.97l1.5 1.5C10.16 5.19 11.06 5 12 5c3.87 0 7 3.13 7 7 0 .94-.19 1.84-.52 2.65l1.5 1.5c.5-.96.84-2.02.97-3.15H23v-2h-2.06zM3 4.27l2.04 2.04C3.97 7.62 3.25 9.23 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c1.77-.2 3.38-.91 4.69-1.98L19.73 21 21 19.73 4.27 3 3 4.27zm13.27 13.27C15.09 18.45 13.61 19 12 19c-3.87 0-7-3.13-7-7 0-1.61.55-3.09 1.46-4.27l9.81 9.81z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBatteryChargingFull;
impl IconShape for MdBatteryChargingFull {
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
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zM11 20v-5.5H9L13 7v5.5h2L11 20z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBluetoothConnected;
impl IconShape for MdBluetoothConnected {
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
                d: "M7 12l-2-2-2 2 2 2 2-2zm10.71-4.29L12 2h-1v7.59L6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 5.83l1.88 1.88L13 9.59V5.83zm1.88 10.46L13 18.17v-3.76l1.88 1.88zM19 10l-2 2 2 2 2-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWifiTethering;
impl IconShape for MdWifiTethering {
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
                d: "M12 11c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 2c0-3.31-2.69-6-6-6s-6 2.69-6 6c0 2.22 1.21 4.15 3 5.19l1-1.74c-1.19-.7-2-1.97-2-3.45 0-2.21 1.79-4 4-4s4 1.79 4 4c0 1.48-.81 2.75-2 3.45l1 1.74c1.79-1.04 3-2.97 3-5.19zM12 3C6.48 3 2 7.48 2 13c0 3.7 2.01 6.92 4.99 8.65l1-1.73C5.61 18.53 4 15.96 4 13c0-4.42 3.58-8 8-8s8 3.58 8 8c0 2.96-1.61 5.53-4 6.92l1 1.73c2.99-1.73 5-4.95 5-8.65 0-5.52-4.48-10-10-10z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdStorage;
impl IconShape for MdStorage {
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
                d: "M2 20h20v-4H2v4zm2-3h2v2H4v-2zM2 4v4h20V4H2zm4 3H4V5h2v2zm-4 7h20v-4H2v4zm2-3h2v2H4v-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddToHomeScreen;
impl IconShape for MdAddToHomeScreen {
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
                d: "M18 1.01L8 1c-1.1 0-2 .9-2 2v3h2V5h10v14H8v-1H6v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM10 15h2V8H5v2h3.59L3 15.59 4.41 17 10 11.41z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdUsb;
impl IconShape for MdUsb {
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
                d: "M15 7v4h1v2h-3V5h2l-3-4-3 4h2v8H8v-2.07c.7-.37 1.2-1.08 1.2-1.93 0-1.21-.99-2.2-2.2-2.2-1.21 0-2.2.99-2.2 2.2 0 .85.5 1.56 1.2 1.93V13c0 1.11.89 2 2 2h3v3.05c-.71.37-1.2 1.1-1.2 1.95 0 1.22.99 2.2 2.2 2.2 1.21 0 2.2-.98 2.2-2.2 0-.85-.49-1.58-1.2-1.95V15h3c1.11 0 2-.89 2-2v-2h1V7h-4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdResetTv;
impl IconShape for MdResetTv {
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
                d: "M21 10h-8.01V7L9 11l3.99 4v-3H21v5H3V5h18v3h2V5c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2v-5H23c0-1.1-.9-2-2-2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdGpsFixed;
impl IconShape for MdGpsFixed {
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
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddAlarm;
impl IconShape for MdAddAlarm {
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
                d: "M7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm1-11h-2v3H8v2h3v3h2v-3h3v-2h-3V9z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNfc;
impl IconShape for MdNfc {
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
                d: "M4 20h16V4H4v16z",
            }
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 18H4V4h16v16zM18 6h-5c-1.1 0-2 .9-2 2v2.28c-.6.35-1 .98-1 1.72 0 1.1.9 2 2 2s2-.9 2-2c0-.74-.4-1.38-1-1.72V8h3v8H8V8h2V6H6v12h12V6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBluetooth;
impl IconShape for MdBluetooth {
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
                d: "M17.71 7.71L12 2h-1v7.59L6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 5.83l1.88 1.88L13 9.59V5.83zm1.88 10.46L13 18.17v-3.76l1.88 1.88z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBatteryUnknown;
impl IconShape for MdBatteryUnknown {
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
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zm-2.72 13.95h-1.9v-1.9h1.9v1.9zm1.35-5.26s-.38.42-.67.71c-.48.48-.83 1.15-.83 1.6h-1.6c0-.83.46-1.52.93-2l.93-.94c.27-.27.44-.65.44-1.06 0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5H9c0-1.66 1.34-3 3-3s3 1.34 3 3c0 .66-.27 1.26-.7 1.69z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScreenLockLandscape;
impl IconShape for MdScreenLockLandscape {
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
                d: "M21 5H3c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm-2 12H5V7h14v10zm-9-1h4c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1v-1c0-1.11-.9-2-2-2-1.11 0-2 .9-2 2v1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1zm.8-6c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2v1h-2.4v-1z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellularConnectedNoInternet4Bar;
impl IconShape for MdSignalCellularConnectedNoInternet4Bar {
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
                d: "M20 18h2v-8h-2v8zm0 4h2v-2h-2v2zM2 22h16V8h4V2L2 22z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScreenSearchDesktop;
impl IconShape for MdScreenSearchDesktop {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            rect {
                height: "1.8",
                style: "fill:none",
                width: "4.8",
                x: "9.6",
                y: "16.8",
            }
            path {
                d: "M20,18 C21.1,18 21.99,17.1 21.99,16 L22,6 C22,4.89 21.1,4 20,4 L4,4 C2.89,4 2,4.89 2,6 L2,16 C2,17.1 2.89,18 4,18 L0,18 L0,20 L24,20 L24,18 L20,18 Z M4,16 L4,6 L20,6 L20,16 L20,16.01 L4,16 Z M9.0967,9.9531 C9.0967,8.9261 9.9327,8.0891 10.9607,8.0891 C11.9877,8.0891 12.8247,8.9261 12.8247,9.9531 C12.8247,10.9801 11.9877,11.8171 10.9607,11.8171 C9.9327,11.8171 9.0967,10.9801 9.0967,9.9531 Z M16.1287,14.1891 L13.6467,11.7071 C13.9777,11.2021 14.1737,10.6001 14.1737,9.9531 C14.1737,8.1811 12.7327,6.7401 10.9607,6.7401 C9.1887,6.7401 7.7467,8.1811 7.7467,9.9531 C7.7467,11.7251 9.1887,13.1671 10.9607,13.1671 C11.5967,13.1671 12.1857,12.9751 12.6847,12.6561 L15.1737,15.1441 L16.1287,14.1891 Z",
            }
            rect {
                height: "24",
                style: "fill:none",
                width: "24",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBrightnessMedium;
impl IconShape for MdBrightnessMedium {
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
pub struct MdWallpaper;
impl IconShape for MdWallpaper {
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
                d: "M4 4h7V2H4c-1.1 0-2 .9-2 2v7h2V4zm6 9l-4 5h12l-3-4-2.03 2.71L10 13zm7-4.5c0-.83-.67-1.5-1.5-1.5S14 7.67 14 8.5s.67 1.5 1.5 1.5S17 9.33 17 8.5zM20 2h-7v2h7v7h2V4c0-1.1-.9-2-2-2zm0 18h-7v2h7c1.1 0 2-.9 2-2v-7h-2v7zM4 13H2v7c0 1.1.9 2 2 2h7v-2H4v-7z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellularNoSim;
impl IconShape for MdSignalCellularNoSim {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M-618-2872H782V728H-618zM-1 0h26v24H-1zm1 0h24v24H0z",
            }
            path {
                d: "M18.99 5c0-1.1-.89-2-1.99-2h-7L7.66 5.34 19 16.68 18.99 5zM3.65 3.88L2.38 5.15 5 7.77V19c0 1.1.9 2 2 2h10.01c.35 0 .67-.1.96-.26l1.88 1.88 1.27-1.27L3.65 3.88z",
            }
            path {
                d: "M.01 0h24v24h-24z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAccessTime;
impl IconShape for MdAccessTime {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
            path {
                d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDeveloperMode;
impl IconShape for MdDeveloperMode {
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
                d: "M7 5h10v2h2V3c0-1.1-.9-1.99-2-1.99L7 1c-1.1 0-2 .9-2 2v4h2V5zm8.41 11.59L20 12l-4.59-4.59L14 8.83 17.17 12 14 15.17l1.41 1.42zM10 15.17L6.83 12 10 8.83 8.59 7.41 4 12l4.59 4.59L10 15.17zM17 19H7v-2H5v4c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-4h-2v2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalWifi0Bar;
impl IconShape for MdSignalWifi0Bar {
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
                d: "M12,6L12,6c3.33,0,6.49,1.08,9.08,3.07L12,18.17l-9.08-9.1C5.51,7.08,8.67,6,12,6 M12,4C7.31,4,3.07,5.9,0,8.98L12,21 L24,8.98C20.93,5.9,16.69,4,12,4L12,4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdMobileOff;
impl IconShape for MdMobileOff {
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
                d: "M2.76 2.49L1.49 3.76 5 7.27V21c0 1.1.9 2 2 2h10c1.02 0 1.85-.77 1.98-1.75l1.72 1.72 1.27-1.27L2.76 2.49zM7 19V9.27L16.73 19H7zM17 5v9.17l2 2V3c0-1.1-.9-2-2-2H7c-.85 0-1.58.54-1.87 1.3L7.83 5H17z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalWifi4BarLock;
impl IconShape for MdSignalWifi4BarLock {
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
                d: "M23 16v-1.5c0-1.4-1.1-2.5-2.5-2.5S18 13.1 18 14.5V16c-.5 0-1 .5-1 1v4c0 .5.5 1 1 1h5c.5 0 1-.5 1-1v-4c0-.5-.5-1-1-1zm-1 0h-3v-1.5c0-.8.7-1.5 1.5-1.5s1.5.7 1.5 1.5V16zm-6.5-1.5c0-2.8 2.2-5 5-5 .4 0 .7 0 1 .1L23.6 7c-.4-.3-4.9-4-11.6-4C5.3 3 .8 6.7.4 7L12 21.5l3.5-4.4v-2.6z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWifiLock;
impl IconShape for MdWifiLock {
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
                d: "M20.5 9.5c.28 0 .55.04.81.08L24 6c-3.34-2.51-7.5-4-12-4S3.34 3.49 0 6l12 16 3.5-4.67V14.5c0-2.76 2.24-5 5-5zM23 16v-1.5c0-1.38-1.12-2.5-2.5-2.5S18 13.12 18 14.5V16c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 0h-3v-1.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5V16z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNetworkCell;
impl IconShape for MdNetworkCell {
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
                d: "M2,22h20V2L2,22z M20,20h-3V9.83l3-3V20z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAirplanemodeInactive;
impl IconShape for MdAirplanemodeInactive {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx! {
            path {
                d: "M10.5,7.67V3.5C10.5,2.67,11.17,2,12,2c0.83,0,1.5,0.67,1.5,1.5V9l8.5,5v2l-4.49-1.32L10.5,7.67z M19.78,22.61l1.41-1.41 L13.5,13.5L9.56,9.56L2.81,2.81L1.39,4.22l6.38,6.38L2,14v2l8.5-2.5V19L8,20.5L8,22l4-1l4,1l0-1.5L13.5,19v-2.67L19.78,22.61z",
            }
            path {
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBluetoothDisabled;
impl IconShape for MdBluetoothDisabled {
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
                d: "M13 5.83l1.88 1.88-1.6 1.6 1.41 1.41 3.02-3.02L12 2h-1v5.03l2 2v-3.2zM5.41 4L4 5.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l4.29-4.29 2.3 2.29L20 18.59 5.41 4zM13 18.17v-3.76l1.88 1.88L13 18.17z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdScreenLockRotation;
impl IconShape for MdScreenLockRotation {
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
                d: "M23.25 12.77l-2.57-2.57-1.41 1.41 2.22 2.22-5.66 5.66L4.51 8.17l5.66-5.66 2.1 2.1 1.41-1.41L11.23.75c-.59-.59-1.54-.59-2.12 0L2.75 7.11c-.59.59-.59 1.54 0 2.12l12.02 12.02c.59.59 1.54.59 2.12 0l6.36-6.36c.59-.59.59-1.54 0-2.12zM8.47 20.48C5.2 18.94 2.86 15.76 2.5 12H1c.51 6.16 5.66 11 11.95 11l.66-.03-3.81-3.82-1.33 1.33zM16 9h5c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1v-.5C21 1.12 19.88 0 18.5 0S16 1.12 16 2.5V3c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1zm.8-6.5c0-.94.76-1.7 1.7-1.7s1.7.76 1.7 1.7V3h-3.4v-.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellularAlt;
impl IconShape for MdSignalCellularAlt {
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
                d: "M17 4h3v16h-3zM5 14h3v6H5zm6-5h3v11h-3z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWidgets;
impl IconShape for MdWidgets {
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
                d: "M13 13v8h8v-8h-8zM3 21h8v-8H3v8zM3 3v8h8V3H3zm13.66-1.31L11 7.34 16.66 13l5.66-5.66-5.66-5.65z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalWifi4Bar;
impl IconShape for MdSignalWifi4Bar {
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
                d: "M12.01 21.49L23.64 7c-.45-.34-4.93-4-11.64-4C5.28 3 .81 6.66.36 7l11.63 14.49.01.01.01-.01z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSignalCellularOff;
impl IconShape for MdSignalCellularOff {
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
                d: "M21 1l-8.59 8.59L21 18.18V1zM4.77 4.5L3.5 5.77l6.36 6.36L1 21h17.73l2 2L22 21.73 4.77 4.5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdBatteryStd;
impl IconShape for MdBatteryStd {
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
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDevices;
impl IconShape for MdDevices {
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
pub struct MdBatteryAlert;
impl IconShape for MdBatteryAlert {
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
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zM13 18h-2v-2h2v2zm0-4h-2V9h2v5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdSendToMobile;
impl IconShape for MdSendToMobile {
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
                d: "M17,17h2v4c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V3c0-1.1,0.9-1.99,2-1.99L17,1c1.1,0,2,0.9,2,2v4h-2V6H7v12h10V17z M22,12 l-4-4v3h-5v2h5v3L22,12z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDeviceThermostat;
impl IconShape for MdDeviceThermostat {
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
                d: "M15 13V5c0-1.66-1.34-3-3-3S9 3.34 9 5v8c-1.21.91-2 2.37-2 4 0 2.76 2.24 5 5 5s5-2.24 5-5c0-1.63-.79-3.09-2-4zm-4-8c0-.55.45-1 1-1s1 .45 1 1h-1v1h1v2h-1v1h1v2h-2V5z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAdUnits;
impl IconShape for MdAdUnits {
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
                d: "M17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14zM8 6h8v2H8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdDvr;
impl IconShape for MdDvr {
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12zm-2-9H8v2h11V8zm0 4H8v2h11v-2zM7 8H5v2h2V8zm0 4H5v2h2v-2z",
            }
        }
    }
}
