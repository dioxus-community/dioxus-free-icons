use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw/71uBf4a+B5gl38dxL/eewNfBRznf5Zd4GOA7+ZFh/jXeW3gt/if7XWA3+ZFg/jXeTrwYP5nuxV4CC8axIvupYG/4n+HlwH+mn8Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zX+hzgp4G/Bl4b+GzgtfiXvQ7w2/zLEC+61wZ+i/86rwP8Ns/ru4H34oV7HeC3+ZchXnSvDfwW/zW+B3hvnr/jwEVeuNcBfpt/GeJF99rAb/Ff42OAr+YF+23gtXjBXgf4bf5liBfdawO/xX+NjwG+mhfst4HX4gV7HeC3+ZchXnSvDfwW/zW+B3hvnr/jwEVeuNcBfpt/GeJF99rAb/Ff53WA3+Z5fRfw3rxwrwP8Nv8yxIvutYHf4r/WZwM/A/w18FrAZwOvzb/sdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX30sBX87/DRwN/zb8M8f8b4v83xP9viP/fEP+/8Y9GPoFB0WCjKwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorDoor;
impl IconShape for MdSensorDoor {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 S17,11.17,17,12S16.33,13.5,15.5,13.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSensorWindow;
impl IconShape for MdSensorWindow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2L18,2z M7,19h10v-6H7 V19z M10,10h4v1h3V5H7v6h3V10z",
            }
        }
    }
}
