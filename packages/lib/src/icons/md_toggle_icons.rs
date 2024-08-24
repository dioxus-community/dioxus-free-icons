use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4GuAv+ZfhnjRvDfwXfzv8j7Ad/PCIf5lLw38Ff87PQS4lRcM8S/7buC9+N/pe4D35gVD/MsuAsf532kXOMELhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/5n+BngwcIwXTrxgiH+Z+Z/nb4DXBh4M/DZwjBdMvGCIf5n5n+VvgNcGdrnipYHfBo7x/IkXDPEvM/9z/A3w2sAuz+mlgd8GjvG8xAuG+JeZ/xn+BnhtYJfn76eAt+Z5iRcM8S8z//3+BnhtYJfn77uA9+b5Ey8Y4l9m/nv9DfDawC7P33cB780LJl4wxL/M/Pf5G+C1gV2ev+8C3psXTrxgiH+Z+e/xN8BrA7s8f98FvDf/MvGCIf5l5r/e3wCvDezy/H0X8N68aMQLhviXmf9afwO8NrDL8/ddwHvzohMvGOJfZl64vwH+Gngv/v3+BnhtYJfn77uA9+ZfR7xgiH+ZecH+BnhtYBf4buC9+Lf7G+C1gV2ev+8C3pt/PfGCIf5l5vn7G+C1gV2e7buB9+Jf72+A1wZ2ef6+C3hv/m3EC4b4l5nn9TfAawO7PK/vBt6LF93fAK8N7PL8fRfw3vzbiRcM8S8zz+uvgdcBdnn+vht4L/5lfwO8NrDL8/ddwHvz7yNeMMS/zDx/fw28DrDL8/fdwHvxgv0N8NrALs/fdwHvzb+feMEQ/zLzgv018DrALs/fdwPvxfP6G+C1gV2ev+8C3pv/GOIFQ/zLzAv318DrALs8f98NvBfP9jfAawO7PH/fBbw3/3HEC4b4l5l/2V8DrwPs8vx9N/BewN8Arw3s8vx9F/De/McSLxjiX2ZeNH8NvA6wy/P32cBXA7s8f98FvDf/8cQLhviXmRfdXwOvA+zyr/NdwHvzn0O8YIh/mfnX+WvgdYBdXjTfBbw3/3nEC4b4l5l/vb8GXgfY5YX7LuC9+c8lXjDEv8z82/w18DrALs/fdwHvzX8+8YIh/mXm3+6vgdcBdnlO3wW8N/81xAuG+JeZf5+/Bl4H2OWK7wLem/864gVD/MvMv99fA68DfBXw3vzXEi8Y4l9m/mPsAsf5rydeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv2wXOMb/TpeA47xgiH/ZdwPvxf9O3wO8Ny8Y4l/20sBf8b/TQ4BbecEQL5r3Br6L/13eB/huXjjEi+7BwGcDbw0c43+mS8BPA58N3Mq/DPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BBsQq0ESihrWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheckBox;
impl IconShape for MdCheckBox {
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
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a/5neWngq3le4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lDwaezv9ODwFu5QVDvGjeG/gu/nd5H+C7eeEQL7oHA58NvDVwjP+ZLgE/DXw2cCv/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EbfLikGVIMc5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheckBoxOutlineBlank;
impl IconShape for MdCheckBoxOutlineBlank {
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
                d: "M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8y8aP4G+Gj+a3w18FK8aMQLhviXmRfN7wCvzX+N3wZeixeNeMEQ/zLzovkd4LX5r/HbwGvxohEvGOJfZl40vwO8Nv81fht4LV404gVD/MvMi+Z3gNfmv8ZvA6/Fi0a8YIh/mXnR/DXw0fzX+GrgpXnRiBcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHTZpKQZ9kzcoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdIndeterminateCheckBox;
impl IconShape for MdIndeterminateCheckBox {
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
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M17,13H7v-2h10V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRadioButtonChecked;
impl IconShape for MdRadioButtonChecked {
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
                d: "M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0-5C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRadioButtonUnchecked;
impl IconShape for MdRadioButtonUnchecked {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR4M7AK7/PdA/Pf6bK74bP57IP57PZ0rHsJ/D8R/n/cGvosr3gf4bv7rIf77/Bbw2lzx28Dr8F8P8d/jpYG/4jm9DPDX/NdC/Pf4buC9eE7fA7w3/7UQ//WOAxd5/k4Au/zXQfzX+2jgq3j+Pgb4av7rIP7rPR14MM/frcBD+K+D+K/11sBP8cK9DfDT/NdA/Nf6LeC1eeF+G3gd/msg/us8GHg6L5qHALfynw/xX+e7gffiRfM9wHvznw/xX+M48HTgOC+aXeAhwC7/uRD/NT4a+Cr+dd4H+G7+cyH+azwdeDD/OrcCD+E/F+I/32sDv8W/zesAv81/HsS/3UsDx7jipYHjXPHSwHGueGngOP8+u8Bfc8Uu8NdcsQv8NVdcAv6afz3E8/fawGtxxYOBB3PFSwPH+Z9tF/hrrrgVuJUrfgf4bZ4T4gX7buC9+L/he4D35nkhXrjPBj6L/90+B/hsnj/Ev+y9ge/if6f3Ab6bFwzxonlt4KeBY/zvcAl4a+C3eeEQL7qXBn4aeBD/sz0DeGvgr/mXIf51jgO/DbwU/zP9DfDawC4vGsS/3nHgu4G34n+WnwHeG9jlRYf4t/tu4L34n+F7gPfmXw/x7/PewHfx3+t9gO/m3wbx7/fewFcDx/ivdQn4aOC7+bdD/Md4aeC3gWP817gEvDbw1/z7IP7jPBj4aeCl+M/1N8BbA7fy74f4j/XawG/xn+t1gN/mPwbiP9Z7A9/Ff673Ab6b/xiI/1jfDbwX/7m+B3hv/mMg/mM9HXgw/7luBR7CfwzEf5wHA0/nv8YJYJd/P8R/nLcGfor/Gm8D/DT/foj/OF8NfBT/Nb4G+Gj+/RD/cf4KeGn+a/w18DL8+yH+45j/WuLfD/Ef47WB3+Jf5xLw0Vzx1cAx/nVeB/ht/n0Q/zE+G/gsXnRfA3w2sMsVx4HPBj6KF93nAJ/Nvw/iP8ZvA6/Fv+x3gI8G/prn76WBrwZei3/Z7wCvzb8P4j+GeeEuAR8NfDcvmvcGvho4xgu2C5zg3wfx7/fSwF/xgn0N8NnALv86x4HPBj6KF+xlgL/m3w7x7/fRwFfxvH4H+Gjgr/n3eWngq4HX4nl9DPDV/Nsh/v1+Gngrnu0S8NHAd/Mf672BrwaO8Ww/A7w1/3aIf7+nAw/miq8BPhvY5T/HceCzgY/iiluBh/Bvh/j3eTDwdOBvgPcG/pr/Gq8NfDXwUsBDgFv5t0H8+7w18GDgq/nv8dHArcBP82+D+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CffGQQa2YPe0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStar;
impl IconShape for MdStar {
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
                d: "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStarBorder;
impl IconShape for MdStarBorder {
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
                d: "M22 9.24l-7.19-.62L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.63-7.03L22 9.24zM12 15.4l-3.76 2.27 1-4.28-3.32-2.88 4.38-.38L12 6.1l1.71 4.04 4.38.38-3.32 2.88 1 4.28L12 15.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR4M7AK7/PdA/Pf6bK74bP57IP57PZ0rHsJ/D8R/n/cGvosr3gf4bv7rIf77/Bbw2lzx28Dr8F8P8d/jpYG/4jm9DPDX/NdC/Pf4buC9eE7fA7w3/7UQ//WOAxd5/k4Au/zXQfzX+2jgq3j+Pgb4av7rIP7rPR14MM/frcBD+K+D+K/11sBP8cK9DfDT/NdA/Nf6LeC1eU7PAB7Es/028Dr810D813kw8HSe1+cAn8VzeghwK//5EP91vht4L57XCeAiz+l7gPfmPx/iv8Zx4OnAcZ6XgO8G3otn2wUeAuzynwvxX+Ojga/i+RPw0sBf8Zw+Bvhq/nMh/ms8HXgwz5+44q+Bl+LZbgUewn8uxH++1wZ+ixdMXPHewHfxnF4H+G3+8yD+7V4aOMYVLw0c54qXBo5zxUsDx3nhxLPtAsd4Xr/Ns/02z/bbXHEJ+Gv+9RDP32sDr8UVDwYezBUvDRznP5Z4ts8GPot/n13gr7niVuBWrvgd4Ld5TogX7LuB9+I/n3i2BwNP5z/e9wDvzfNCvHCfDXwW/7nEc/pu4L34j/M5wGfz/CH+Ze8NfBf/ecTzejDwYK54MPBgrnhp4DhXvDRwjBfufYDv5gVDvGheG/hp4Bj/8cS/3XsDXwUc5zldAt4a+G1eOMSL7qWBnwYexH8s8W/z3sB38bwuAa8N/DX/MsS/znHgt4GX4j+O+Nf7LuC9eV5/A7w2sMuLBvGvdxz4buCt+I8hXnTHge8C3prn9TPAewO7vOgQ/3bfDbwX/37iRXMc+C3gpXle3wO8N/96iH+f9wa+i38f8S97aeCngAfzvD4G+Gr+bRD/fu8NfDVwjH8b8cK9NvBTwHGe0yXgo4Hv5t8O8R/jpYHfBo7xrydesLcGforndQl4beCv+fdB/Md5MPDTwEvxryNesI8Gvorn9DfAewN/zb8f4j/WawO/xb+OeMFeGvgrntPrAL/NfwzEf6z3Br6Lfx3xwpnn9DHAV/MfA/Ef67uB9+JfR7xwvw28Fs/2PcB78x8D8R/r6cCD+dcRL9xnA5/Fs90KPIT/GIj/OA8Gns6/3ncDHwPs8vy9NvBbPKeHALfy74f4j/PWwE/xb7MLfDbwNTyv48BFntP7AN/Nvx/iP85XAx/Fv89fAx8D/DbP6a+Bl+LZvgb4aP79EP9x/gp4af5jfDfwMcAuV3w18FE8218DL8O/H+I/jvmPtQt8NPA9wFsDP8VzOgHs8u+D+I/x2sBv8a9zCfhorvhq4BjP328DXwP8FM/pdYDf5t8H8R/js4HP4kX3NcBnA7tccRz4bOCjeNF9DvDZ/Psg/mP8NvBa/Mt+B/ho4K95/l4a+GrgtfiX/Q7w2vz7IP5jmBfuEvDRwHfzonlv4KuBY7xw4t8H8e/30sBf8YJ9DfDZwC7/OseBzwY+ihfsZYC/5t8O8e/30cBX8bx+B/ho4K/593lp4KuB1+J5fQzw1fzbIf79fhp4K57tEvDRwHfzH+u9ga8GjvFsPwO8Nf92iH+/pwMP5oqvAT4b2OU/x3Hgs4GP4opbgYfwb4f493kw8HTgb4D3Bv6a/xqvDXw18FLAQ4Bb+bdB/Pu8NfBg4Kv57/HRwK3AT/Nvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Alaot0FnNuWzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStarHalf;
impl IconShape for MdStarHalf {
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
                d: "M22,9.24l-7.19-0.62L12,2L9.19,8.63L2,9.24l5.46,4.73L5.82,21L12,17.27L18.18,21l-1.63-7.03L22,9.24z M12,15.4V6.1 l1.71,4.04l4.38,0.38l-3.32,2.88l1,4.28L12,15.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStarOutline;
impl IconShape for MdStarOutline {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/vwcCD+O/xDOBW/u0Q/zavDbwX8NbAcf577QI/DXwP8Nv86yD+dV4b+Czgtfmf6beBzwF+mxcN4kX3UcBX87/DRwNfw78M8aL5LuC9+d/lu4H34YVD/Mu+Gvgo/nf6HOCzecEQL9xbAz/FC/Y9wE8Dfw3cyhWvDbw18N7AMf77vQ3w0zx/iBfu6cCDeV5/A7w38Ne8YMeBrwbei/9etwIP4flDvGCfDXwWz+tvgNcGdnnRfDfwXvz3+hzgs3leiBfs6cCDeU6XgJcGbuVf57eB1+K/z18DL8PzQjx/DwaezvP6HOCz+dd7aeCv+O/1EOBWnhPi+fto4Kt4Xg8BbuXf5q+Bl+K/z8cAX81zQjx/nw18Fs/pb4CX5t/us4HP4r/P5wCfzXNCPH/fDbwXz+l3gNfm3+6jga/iv8/XAB/Nc0I8f98NvBfP6XeA1+bf7qOBr+K/z9cAH81zQjx/nw18Fs/pr4GX4d/us4HP4r/P5wCfzXNCPH8fDXwVz+shwK382/wV8NL89/kY4Kt5Tojn78HA03lenwN8Nv96Lw38Ff+9HgLcynNCvGB/DbwUz2kXeBngVv51fgt4bf77/A3w0jwvxAv22cBn8bz+GngdYJcXzXcB781/r48BvprnhXjhbgUexPP6a+BjgN/mBTsOfBXw3vz3egbwYJ4/xAv31sBP8YJ9N/DTwN8At3LFawFvDbw3cJz/fm8D/DTPH+Jf9tXAR/G/0+cAn80LhnjRfDfwXvzv8j3Ae/PCIV50Hw18Ff87fAzw1fzLEP86rw18NvBa/M/0O8BnA7/Niwbxb/PawHsDrw08iP9ezwB+G/hu4Lf510H8+x0HXpr/Hn8N7PJvh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wiuYWhBDmSBwQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToggleOff;
impl IconShape for MdToggleOff {
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
                d: "M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zM7 15c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/vwcCD+O/xDOBW/u0Q/zavDbwX8NbAcf577QI/DXwP8Nv86yD+dV4b+Czgtfmf6beBzwF+mxcN4kX3UcBX87/DRwNfw78M8aL5LuC9+d/lu4H34YVD/Mu+Gvgo/nf6HOCzecEQL9xbAz/Ff79LwHcDPw38Nlc8GHhp4K2B9+IFexvgp3n+EC/c04EH89/re4CPBnZ5wV4b+G7gQTyvW4GH8PwhXrDPBj6L/17fA7w3L5rjwG8DL8Xz+hzgs3leiBfs6cCD+e/zO8Br86/zYOCvgWM8p78GXobnhXj+Hgw8nf9eDwFu5V/vs4HP4nk9BLiV54R4/j4a+Cr++/wN8NL82zwYeDrP62OAr+Y5IZ6/zwY+i/8+nwN8Nv92fw28FM/pc4DP5jkhnr/vBt6L/z7vA3w3/3a/DbwWz+lrgI/mOSGev+8G3ov/Pu8DfDf/dr8NvBbP6WuAj+Y5IZ6/zwY+i/8+nwN8Nv92fwW8NM/pc4DP5jkhnr+PBr6K/z5/DbwM/zYPBp7O8/oY4Kt5Tojn78HA0/nv9RDgVv71Phv4LJ7XQ4BbeU6IF+yvgZfiv89vA6/Dv86Dgb8CjvOc/gZ4aZ4X4gX7bOCz+O/13cD78KI5DvwW8NI8r48BvprnhXjhbgUexH+v7wY+BtjlBXtt4LuAB/O8ngE8mOcP8cK9NfBT/PfbBb4b+Gngd7jiwcBrAa8NvDcv2NsAP83zh/iXfTXwUfzv9DnAZ/OCIV403w28F/+7fA/w3rxwiBfdRwNfxf8OHwN8Nf8yxL/OawOfDbwW/zP9DvDZwG/zokH827w28N7AawMP4r/XM4DfBr4b+G3+dRD/fseBl+a/x18Du/zbIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIz4mRB8OJYSgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToggleOn;
impl IconShape for MdToggleOn {
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
                d: "M17 7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h10c2.76 0 5-2.24 5-5s-2.24-5-5-5zm0 8c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}
