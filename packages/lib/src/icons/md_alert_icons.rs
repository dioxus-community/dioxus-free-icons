use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MdErrorOutline;
impl IconShape for MdErrorOutline {
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
                d: "M11 15h2v2h-2zm0-8h2v6h-2zm.99-5C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAddAlert;
impl IconShape for MdAddAlert {
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
                d: "M10.01 21.01c0 1.1.89 1.99 1.99 1.99s1.99-.89 1.99-1.99h-3.98zm8.87-4.19V11c0-3.25-2.25-5.97-5.29-6.69v-.72C13.59 2.71 12.88 2 12 2s-1.59.71-1.59 1.59v.72C7.37 5.03 5.12 7.75 5.12 11v5.82L3 18.94V20h18v-1.06l-2.12-2.12zM16 13.01h-3v3h-2v-3H8V11h3V8h2v3h3v2.01z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdNotificationImportant;
impl IconShape for MdNotificationImportant {
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
                d: "M18 16v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2zm-5 0h-2v-2h2v2zm0-4h-2V8h2v4zm-1 10c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdWarning;
impl IconShape for MdWarning {
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
                d: "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdAutoDelete;
impl IconShape for MdAutoDelete {
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
            polygon {
                points: "15,2 11.5,2 10.5,1 5.5,1 4.5,2 1,2 1,4 15,4",
            }
            path {
                d: "M16,9c-0.7,0-1.37,0.1-2,0.29V5H2v12c0,1.1,0.9,2,2,2h5.68c1.12,2.36,3.53,4,6.32,4c3.87,0,7-3.13,7-7 C23,12.13,19.87,9,16,9z M16,21c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5S18.76,21,16,21z",
            }
            polygon {
                points: "16.5,12 15,12 15,17 18.6,19.1 19.4,17.9 16.5,16.2",
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MdError;
impl IconShape for MdError {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            }
        }
    }
}
