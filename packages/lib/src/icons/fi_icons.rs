use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFXklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/eV4a+Ciu+Brgr/mfB/Gf46WBv+LZdoGXAW7lfxbEf47vBt6L5/Q9wHvzPwviP8dF4DjPaRc4wf8siP94bw38FM/f2wA/zf8ciP943w28F8/f9wDvzf8ciP94F4HjPH+7wAn+50D8x3pr4Kd44d4G+Gn+Z0D8x/pu4L144b4HeG/+Z0D8x7oIHOeF2wVO8D8D4j/OWwM/xYvmbYCf5r8f4j/OdwPvxXO6xBXHeE7fA7w3//0Q/3EuAsd5Tt/DFe/Fc9oFTvDfD/Ef462Bn+J5vQ1X/BTP622An+a/F+I/xncD78VzugQc54pd4BjP6XuA9+a/F+I/xkXgOM/pe4D35orvBt6L57QLnOC/F+Lf762Bn+J5vQ3w01zx1sBP8bzeBvhp/vsg/v2+G3gvntMl4DjPaRc4xnP6HuC9+e+D+Pe7CBznOX0P8N48p+8G3ovntAuc4L8P4t/nrYGf4nm9DfDTPKe3Bn6K5/U2wE/z3wPx7/PdwHvxnC4Bx3n+doFjPKfvAd6b/x6If5+LwHGe0/cA783z993Ae/GcdoET/PdA/Nu9NfBTPK+3AX6a5++tgZ/ieb0N8NP810P823038F48p0vAcV64XeAYz+l7gPfmvx7iOR0H3gt4a/5lLw0c5zl9D/DevHDfDbwXz2kXOMG/zksD7wW8NC+6nwa+B9jlCsRz+ivgpfm3exvgp3nh3hr4KZ7X2wA/zYvmpYHfAo7zr/fXwMtwBeLZXhv4Lf7tLgHHedHsAsd4Tt8DvDcvmu8G3ot/u9cBfhtAPNtrA7/Fv933AO/Ni+a7gffiOe0CJ3jRXASO82/3OsBvA4jn9NfAS/Gvdwl4beCvedG8NfBTPK+3AX6aF+6tgZ/i3+5vgJfmCsRzOg68N/DawHFeNH8NfDfw1/zr7ALHeE7fA7w3L9x3A+/Fc7oE/DUv3C7w28B3A7tcgfjv893Ae/GcdoETvHAXgeM8p+8B3pt/PcR/n7cGforn9TbAT/P8vTXwUzyvtwF+mn89xH+vXeAYz+l7gPfm+ftu4L14TpeA4/zbIP57fTfwXjynXeAEz99F4DjP6XuA9+bfBvHf662Bn+J5vQ3w0zyntwZ+iuf1NsBP82+D+O+3CxzjOX0P8N48p+8G3ovndAk4zr8d4r/fdwPvxXPaBU7wnC4Cx3lO3wO8N/92iP9+bw38FM/rbYCf5oq3Bn6K5/U2wE/zb4f4n2EXOMZz+h7gvbniu4H34jldAo7z74P4n+G7gffiOe0CJ7jiInCc5/Q9wHvz74P4n+GtgZ/ieb0NV/wUz+ttgJ/m3wfxP8cucIzn9D1c8V48p0vAcf79EP9zfDfwXjynXa44znP6HuC9+fdD/M/x1sBP8aJ5G+Cn+fdD/M+yCxzjhbsEHOc/BuJ/lu8G3osX7nuA9+Y/BuJ/lrcGfooX7m2An+Y/BuJ/nl3gGM/fJeA4/3EQ//N8N/BePH/fA7w3/3EQ//O8NfBTPH9vA/w0/3EQ/zPtAsd4TpeA4/zHQvzP9N3Ae/Gcvgd4b/5jIf5nemngr3i2S8BLA7fyHwvxP9dLAx/NFV8N/DX/8RD/vyH+f0P8/4b4/w3x/xv/CLBLu0EommNSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "22 12 18 12 15 21 9 3 6 12 2 12",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/3mvxn+t3+I+D+I9zHPgt4KX5z/XXwOsAu/z7If5jHAd+C3hp/mv8NfA6wC7/Poh/v+PAbwEvzX+tvwZeB9jl3w7x73Mc+C3gpfnv8dfA6wC7/Nsg/u2OA78FvDT/vf4aeB1gl389xL/NceC3gJfm+fse4L35j/XdwHvx/P018DrALv86iH+948BvAS/N8/c9wHvzn+O7gffi+ftr4HWAXV50iH+d48BvAS/N8/c9wHvzn+u7gffi+ftr4HWAXV40iBfdceC3gJfm+fse4L35r/HdwHvx/P018DrALv8yxIvmOPBbwEvz/H0P8N781/pu4L14/v4aeB1glxcO8S87DvwW8NI8f98DvDf/Pb4beC+ev78GXgfY5QVDvHDHgd8CXprn73uA9+a/13cD78Xz99fA6wC7PH+IF+w48FvAS/P8fQ/w3vzP8N3Ae/H8/TXwOsAuzwvx/B0Hfgt4aZ6/7wHem/9Zvht4L56/vwZeB9jlOSGe13Hgt4CX5vn7HuC9+Z/pu4H34vn7a+B1gF2eDfGcjgO/Bbw0z9/3AO/Nf5zX4orf4T/OdwPvxfP318DrALtcgXhOfwW8NM/f9wDvzX+M48BvAS/NFX8NvA6wy3+M7wbei+fvr4GX4QrEs7028Fs8f98DvDf/cT4a+Cqe08cAX81/nO8G3ovn73WA3wYQz/bawG/xvL4HeG/+Y/028Fo8p58B3pr/WN8NvBfP63WA3wYQz/bawG/xvMR/vN8GXovn9DvAa/Mfzzyv1wF+G0A822sDv8XzEv/xfht4LZ7T7wCvzX8887xeB/htAPFsrw38Fs9L/Mf7beC1eE6/A7w2//HM83od4LcBxLO9NvBbPC/xH++3gdfiOf0O8Nr8xzPP63WA3wYQz/bawG/xvMR/vN8GXovn9DvAa/Mfzzyv1wF+G0A822sDv8XzEv/xfht4LZ7T7wCvzX8887xeB/htAPFsrw38Fs9L/Mf7beC1eE6/A7w2//HM83od4LcBxLO9NvBbPC/xH++3gdfiOf0O8Nr8xzPP63WA3wYQz/bawG/xvMR/vN8GXovn9DvAa/Mfzzyv1wF+G0A822sDv8XzEv/xfht4LZ7T7wCvzX8887xeB/htAPFsrw38Fs9L/Mf7auCjeE5fA3w0//HM83od4LcBxLO9NvBbPC/xH++lgb/iOb0M8Nf8xzPP63WA3wYQz/bawG/xvMR/jpcG3psrvhv4a/5zmOf1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMR/js8CPporvhr4HP5zmOf1OsBvA4hne23gt3he4j/eewPfxXN6H+C7+Y9nntfrAL8NIJ7ttYHf4nl9N/A+/Mf6buC9eE4/A7w1/7G+C3hvntfrAL8NIJ7ttYHf4vn7buB9+I/z0cBX8Zw+Bvhq/uN8F/DePH+vA/w2gHhOfw28FM/fdwPvw3+M48BvAy/FFX8DvDawy3+M7wLem+fvb4CX5grEczoO/DbwUjx/3w28D/9xXpsrfpv/ON8FvDfP398Arw3scgXieR0Hfht4KZ6/7wbeh/+Zvgt4b56/vwFeG9jl2RDP33Hgt4GX4vn7buB9+J/lu4D35vn7G+C1gV2eE+IFOw78NvBSPH/fDbwP/zN8F/DePH9/A7w2sMvzQrxwx4HfBl6K5++7gffhv9d3Ae/N8/c3wGsDuzx/iH/ZceC3gZfi+ftu4H347/FdwHvz/P0N8NrALi8Y4kVzHPht4KV4/r4beB/+a30X8N48f38DvDawywuHeNEdB34beCmev+8G3of/Gt8FvDfP398Arw3s8i9D/OscB34beCmev+8G3of/XN8FvDfP398Arw3s8qJB/OsdB34beCmev+8G3of/HN8FvDfP398Arw3s8qJD/NscB34beCmev+8G3of/WN8FvDfP398Arw3s8q+D+Lc7Dvw28FL89/ob4LWBXf71EP8+x4HfBl6K/x5/A7w2sMu/DeLf7zjw28BL8V/rb4DXBnb5t0P8xzgO/DbwUvzX+BvgtYFd/n0Q/3GOA78NvBT/uf4GeG1gl38/xH+81+Y/12/zHwfx/xvi/zf+EbIbEFCFZMiTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACmUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/u/018D3AX/O8EM/fSwN/xf8tLwP8Nc8J8fx9NfBR/N/yNcBH85wQz99XAx/F/y1fA3w0zwnx/L008Ff83/IywF/znBAv2EsD7w28NP+7/TXw3cBf87wQ/78h/n9D/P+G+P8N8f8b4t/us4CPBo7z32sX+Grgc/jXQ/zbvDfwXfzP8jbAT/Ovg/i3+W7gvfif5WuAj+ZfB/Fv89HAV/E/y8cAX82/DuLf5jjw28BL8T/D3wCvDezyr4P493lt/mf4bf5tEP+/If5/Q/z/hvj/DfHCvRb/N/wOzx/i+TsO/Bbw0vzf8NfA6wC7PCfE8/fRwFfxf8vHAF/Nc0I8f98NvBf/t3wN8NE8J8Tz99bAT/F/y9sAP81zQrxgnw18NHCM/90uAV8NfDbPC/H/G+L/N8T/b4j/3xD/vyH+7V4aeC/gpfnv9dfA9wB/zb8e4t/mpYG/4n+WlwH+mn8dxL/NVwMfxf8sXwN8NP86iH+brwY+iv9Zvgb4aP51EP82Lw38Ff+zvAzw1/zrIP7tXhp4b+Cl+e/118B3A3/Nvx7i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BGaU2QVfq8ZgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/u/018D3AX/O8EM/fSwN/xf8tLwP8Nc8J8fx9NfBR/N/yNcBH85wQz99XAx/F/y1fA3w0zwnx/L008Ff83/IywF/znBAv2EsD7w28NP+7/TXw3cBf87wQ/78h/n9D/P+G+P8N8f8b4gX7LOCjgeP877YLfDXwOTwvxPP33sB38X/L2wA/zXNCPH/fDbwX/7d8DfDRPCfE8/fRwFfxf8vHAF/Nc0I8f8eB3wZeiv8b/gZ4bWCX54R44V6b/xt+m+cP8f8b4v83xP9viP/fEC/ca/F/w+/w/CGev+PAbwEvzf8Nfw28DrDLc0I8fx8NfBX/t3wM8NU8J8Tz993Ae/F/y9cAH81zQjx/bw38FP+3vA3w0zwnxAv22cBHA8f43+0S8NXAZ/O8EP+/If5/Q/z/hvj/DfH/G+IFe2ngvYCX5n+3vwa+B/hrnhfi+Xtp4K/4v+VlgL/mOSGev68GPor/W74G+GieE+L5+2rgo/i/5WuAj+Y5IZ6/lwb+iv9bXgb4a54T4gV7aeC9gZfmf7e/Br4b+GueF+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFnFjZB73kmagAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACk0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/u/018D3AX/O8EM/fSwN/xf8tLwP8Nc8J8fx9NfBR/N/yNcBH85wQz99XAx/F/y1fA3w0zwnx/L008Ff83/IywF/znBAv2EsD7w28NP+7/TXw3cBf87wQ/78h/n9D/P+G+P8N8f8b4gX7LOCjgeP8z/TTwPsAu/zbIZ6/9wa+i//5vgd4b/7tEM/fdwPvxf8O4t8O8fx9NPBV/M/3N8BL82+HeP6OA78NvBT/c10CXhv4a/7tEC/ca/M/118Du/z7IP5/Q/z/hvj/DfH/G+KFey3+b/gdnj/E83cc+C3gpfm/4a+B1wF2eU6I5++jga/i/5aPAb6a54R4/r4beC/+b/ka4KN5Tojn762Bn+L/lrcBfprnhHjBPhv4aOAY/7tdAr4a+GyeF+L/N8T/b4j/3xD/vyH+f0O8YC8NvBfw0vzPcyvwM8BP8++DeP5eGvgr/uf7HOCz+bdDPH9fDXwU//PtAif4t0M8f18NfBT/8z0DeDD/dojn76WBv+J/vs8BPpt/O8QL9tLAewMvzf88twI/Dfw0/z6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EMoY2QQzlyrkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/u/018D3AX/O8EM/fSwN/xf8tLwP8Nc8J8fx9NfBR/N/yNcBH85wQz99XAx/F/y1fA3w0zwnx/L008Ff83/IywF/znBAv2EsD7w28NP+7/TXw3cBf87wQ/78h/n9D/P+G+P8N8f8b4t/vOPBdwFvzP9Mu8NXA5/C8EP9+Pw28Ff/zvQ3w0zwnxL+f+d/ha4CP5jkh/v3+Gngp/uf7GOCreU6If7+XBn4bOMb/XH8DvDawy3NC/Md4MPBg/uf6bZ4/xP9viP/fEP+/If5/Q7xwr8X/Db/D84d4/o4DvwW8NP83/DXwOsAuzwnx/H008FX83/IxwFfznBDP33cD78X/LV8DfDTPCfH8vTXwU/zf8jbAT/OcEC/YZwMfDRzjf7dLwFcDn83zQvz/hvj/DfH/G+L/N8T/b4j/GG8NvBXwYP7n+Wvge4C/5nkh/v0+G/gs/ud7GeCveU6If7+LwHH+5/sa4KN5Toh/v13gGP/zfQ3w0TwnxL/fZwOfxf98LwP8Nc8J8R/jrYG3Bh7M/zx/DXw38Nc8L8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKdqDZBuMrmLwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xBXvDXwVsCD+f/hVuBngJ8W8NnAZ/H/0+cIuAgc5/+nXQHm/y8EmP+/EGCe198Au/zfchx4KZ4TAszzeh3gt/m/5bWB3+I5IcA8r9cBfpv/W14b+C2eEwLM83od4Lf5v+W1gd/iOSHAPK/XAX6b/1teG/gtnhMCzPN6HeC3+b/ltYHf4jkhwDyv1wF+m/9bXhv4LZ4TAszzeh3gt/m/5bWB3+I5IcA8r48G/hp4aa74a/53emmu+GvgpYGv5jkhwPz/hQDz/xcCzP9fCPhp4K34/+l7BBwHvhp4L/5/+R7go8Xz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem3+btwbeCngwL5pbgZ8Bfpp/m98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+df7bOCz+Lf5HOCz+df7beC1eE6/A7w2zwnx/P028Fo8p98BXpt/vYvAcf5tdoET/Ov9NvBaPKffAV6b54R4/n4beC2e0+8Ar82/3i5wjH+bS8Bx/vV+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Zf77OBz+Lf5nOAz+Zf77eB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/m3eGnhr4MG8aG4Ffhr4af5tfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/GcdoG/5n+HlwaO85x+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+2rgo/i/5WuAj+Y5IZ6/lwb+iv9bXgb4a54T4gV7aeC9gZfmf7e/Br4b+GueF+L/N8T/b4j/3xD/vyH+f+MfAdvevVJZj2UWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pc7DnwX8NbAXwPvA/w1/zleG/gq4KWBnwbeB9jl3w7x7/fTwFvxbLvAywC38h/rvYHv4jn9DPDW/Nsh/v3M83od4Lf5j/PewHfx/Il/O8S/n3lerwP8Nv8x3hv4Lp6/vwFemn87xL+feV6vA/w2/37vDXwXL9jrAL/Nvx3i3888r9cBfpt/n/cGvosX7H2A7+bfB/HvZ57X6wC/zb/dewPfxQv2PsB38++H+Pczz+t1gN/m3+a9ge/iBXsf4Lv5j4H49zPP63WA3+Zf772B7+IFex/gu/mPg/j3M8/rdYDf5l/nvYHv4gV7H+C7+Y+F+Pczz+t1gN/mRffewHfxgr0P8N38x0P8+5nn9TrAb/OieW/gu3jB3gf4bv5zIP79zPN6HeC3+Ze9N/BdvGDvA3w3/3kQ/37meb0O8Nu8cO8NfBcv2PsA381/LsS/n3lerwP8Ni/YewPfxQv2PsB3858P8e9nntfrAL/N8/fewHfxgr0P8N3810D8+5nn9TrAb/O83hv4Ll6w9wG+m/86iH8/87xeB/htntN7A9/FC/Y+wHfzXwvx72ee1+sAv82zvTfwXbxg7wN8N//1EP9+5nm9DvDbXPHewHfxgr0P8N3890D8+5nn9TrAbwPvDXwXL9j7AN/Nfx/Ev595Xq8DPBj4Ll6w9wG+m/9eiH8/87y+BPgkXrD3Ab6b/36Ifz/zr/M+wHfzPwPi38+86N4H+G7+50D8+5kXzfsA383/LIh/P/Mv+2Lgl/mP9QzgVv59EP9+5r/HLvA6wF/zb4f49zP/ff4aeBn+7RD/fua/l/i3Q/z73QVcz3+P7wHem387xL/fg4DfBW7hv9b3AB8N7PJvh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJIwnFBhar34wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pc7DnwX8NbAXwMfA/w2/zleGvgu4KWBnwbeB9jl3w7x7/fTwFvxnN4H+G7+Yz0Y+CvgOM/2PcB782+H+Pczz9/7AN/Nf5zXBn6L5yX+7RD/fn8NvBTP3/sA381/jNcGfovnJf7tEP9+rw38Fi/Y+wDfzb/fawO/xfMS/3aI/xjvDXwXL9j7AN/Nv89rA7/F8xL/doj/OO8NfBcv2PsA382/3WsDv8XzEv92iP9Y7w18Fy/Y+wDfzb/NawO/xfMS/3aI/3jvDXwXL9j7AN/Nv95rA7/F8xL/doj/HO8NfBcv2PsA382/zmsDv8XzEv92iP887w18Fy/Y+wDfzYvutYHf4nmJfzvEf673Br6LF+x9gO/mRfPawG/xvMS/HeI/33sD38UL9j7Ad/Mve23gt3he4t8O8V/jvYHv4gV7H+C7eeFeG/gtnpf4t0P813lv4Lt4wd4H+G5esNcGfovnJf7tEP+13hv4Ll6w9wG+m+fvtYHf4nmJfzvEf733Br6LF+x9gO/meb028Fs8L/Fvh/jv8d7Ad/GCvQ/w3Tyn1wZ+i+cl/u0Q/33eG/guXrD3Ab6bZ3tt4Ld4XuLfDvHf672B7+IFex/gu7nitYHf4nmJfzvEf7/3Br6LF+x9gO8GXhv4LZ6X+LdD/M/w3sB38YK9D3Ar8Fs8L/Fvh/if472B7+IF+xLgk3he4t8O8T/LewPfxb+O+LdD/M/z3sB38aIT/3aI/5neG/guXjTi3w7xH+PBwIP4j/XGwCfzLxP/doh/v5cGfgs4zn8P8W+H+Pf7K+Cl+e8j/u0Q/37mv5f4t0P8+/008Fb893gK8Aj+7RD/fseBrwbei/9aTwFeH3gG/3aI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjVC1UQeyJP+YAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9drccXv8N8D8d/jOPBbwEtzxV8DrwPs8l8L8d/jo4Gv4jl9DPDV/NdC/Pf4beC1eE4/A7w1/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L86xwH3gt4aeBngJ/m3+a3gdfiOf0O8Nr827w18FbAXwPfA+zyokH86/wV8NI823cD78O/3m8Dr8Vz+h3gtfnX+y7gvXm2vwZehhcN4kX32sBv8by+G3gf/nV+G3gtntPvAK/Nv853Ae/N83od4Lf5lyFedK8N/BbP33cD78OL7reB1+I5/Q7w2rzovgt4b56/1wF+m38Z4l/nr4GX4vn7buB9eNH8NvBaPKffAV6bF813Ae/N8/c3wEvzokH86xwHfht4KZ6/7wbeh3/ZbwOvxXP6HeC1+Zd9F/DePH9/A7w2sMuLBvGvdxz4beCleP6+G3gfXrjfBl6L5/Q7wGvzwn0X8N48f38DvDawy4sO8W9zHPht4KV4/r4beB9esN8GXovn9DvAa/OCfRfw3jx/fwO8NrDLvw7i3+448NvAS/H8fTfwPjx/vw28Fs/pd4DX5vn7LuC9ef7+BnhtYJd/PcS/z3Hgt4GX4vn7buB9eF6/DbwWz+l3gNfmeX0X8N48f38DvDawy78N4t/vOPDbwEvx/H038D48p98GXovn9DvAa/Ocvgt4b56/vwFeG9jl3w7xH+M48NvAS/H8fTfwPjzbbwOvxXP6HeC1ebbvAt6b5+9vgNcGdvn3QfzHOQ78NvBSPH/fDbwPV/w28Fo8p98BXpsrvgt4b56/vwFeG9jl3w/xH+s48NvAS/H8fTfwPsBvA6/Fc/od4LWB7wLem+fvb4DXBnb5j4H4j3cc+G3gpXj+vht4CPBaPKffAZ4OvDfP398Arw3s8h8H8Z/jOPDbwEvx/N0DXMdzuge4jufvb4DXBnb5j4X4z3Mc+G3gpfj3+RvgtYFd/uMh/nMdB34beCn+bf4GeG1gl/8ciP98x4HfBl6Kf52/AV4b2OU/D+K/xnHgt4GX4kXzN8BrA7v850L81zkO/DbwUrxwfwO8NrDLfz7Ef63jwG8DL8Xz9zfAawO7/NdA/Nc7Dvw9cCPP6U7gxYFd/usg/ns8CHg8sOCKJfAY4Bn810L89/porvhq/nsg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C4dmoQeHYaGkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9drccXv8N8D8d/jOPBbwEtzxV8DrwPs8l8L8V/vOPBbwEvznD4G+Gr+ayH+ax0Hfgt4aZ7X1wAfzX8txH+d48BvAS/N8/c2wE/zXwvxX+M48FvAS/P8fQ/w3vzXQ/znOw78FvDSPH/fA7w3/z0Q/7mOA78FvDTP3/cA781/H8R/nuPAbwEvzfP3PcB7898L8Z/jOPBbwEvz/H0P8N7890P8xzsO/Bbw0jx/3wO8N/8zIP5jHQd+C3hpnr/vAd6b/zkQ/3GOA78FvDTP3/cA783/LIj/GMeB3wJemufve4D35n8exL/fceC3gJfm+fse4L35nwnx73Mc+C3gpXn+vgd4b/7nQvzbHQd+C3hpnr/vAd6b/9kQ/zbHgd8CXprn73uA9+Z/PsS/3nHgt4CX5vn7HuC9+d8B8a/zIOAPgBt5/u4BnsB/n58GvgfY5UWD+Nc5Ahb8z/bXwMvwokG86D4a+Cr+d3gd4Lf5lyFedB8NfBX/O7wO8Nv8yxD/OkfAgv/Z/gZ4aV40iH+dBwF/ANzI83cP8ET+e+wCvw18N7DLiwbxr3cc+G3gpXj+vht4H/53QPzbHAd+G3gpnr/vBt6H//kQ/3bHgd8GXorn77uB9+F/NsS/z3Hgt4GX4vn7buB9+J8L8e93HPht4KV4/r4beB/+Z0L8xzgO/DbwUjx/3w28D//zIP7jHAd+G3gpnr/vBt6H/1kQ/7GOA78NvBTP33cD78P/HIj/eMeB3wZeiufvu4H34X8GxH+O48BvAy/F8/fdwPvw3w/xn+c48NvAS/H8fTfwPvz3QvznOg78NvBSPH/fDbwP/30Q//mOA78NvBTP33cD78N/D8R/jePAbwMvxfP33cD78F8P8V/nOPDbwEvx/L0N8NP810L81zoO/DbwUjyvrwE+mv9aiP96x4HfBl6K5/QxwFfzXwvx3+M48NvAS3HF3wCvDezyXwvx3+u1ueK3+e+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IeDBfQeyVOrYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9drccXv8N8D8d/jOPBbwEtzxV8DrwPs8l8L8d/jo4Gv4jn9NfA6wC7/dRD/Pb4beC+e118DrwPs8l8D8d/jrYGf4vn7a+B1gF3+8yH++3w38F48f38NvA6wy38uxH+v7wbei+fvr4HXAXb5z4P47/fdwHvx/P018DrALv85EP8zfDfwXjx/fw28DrDLfzzE/xzfDbwXz99fA68D7PIfC/E/y3cD78Xz99fA6wC7/MdB/M/z3cB78fz9NfA6wC7/MRD/M3038F48f38NvA6wy78f4n+u7wbei+fvr4HXAXb590H8z/bdwHvx/P018DrALv92iP/5vht4L56/vwZeB9jl3wbxv8N3A+/F8/fXwOsAu/zrIf73+G7gvXj+/hp4HWCXfx3E/y7fDbwXz99fA68D7PKiQ/zrHAfeC3hr/vs8GriO5+9O4NWAZ/CiQfzr/BXw0vzPtgQ2eNEgXnSvDfwW/zt8DPDV/MsQL7rXBn6L/x0+Bvhq/mWIf52/Bl6K/9mWwAYvGsS/znHgvYHXBo7z3+NRwHU8f3cCrwY8gxcN4n+X7wLem+fvb4DXBnZ50SH+9/gu4L15/v4GeG1gl38dxP8O3wW8N8/f3wCvDezyr4f4n++7gPfm+fsb4LWBXf5tEP+zfRfw3jx/fwO8NrDLvx3if67vAt6b5+9vgNcGdvn3QfzP9F3Ae/P8/Q3w2sAu/36I/3m+C3hvnr+/AV4b2OU/BuJ/lu8C3pvn72+A1wZ2+Y+D+J/ju4D35vn7G+C1gV3+YyH+Z/gu4L15/v4GeG1gl/94iP9+3wW8N8/f3wCvDezynwPx3+u7gPfm+fsb4LWBXf7zIP77fBfw3jx/fwO8NrDLfy7Ef4/3Br6L5+9vgNcGdvnPh/jv8d3Ae/G8/gZ4bWCX/xqI/x4fDXwVz+lvgNcGdvmvg/jvcRz4beCluOJvgNcGdvmvhfjv9dpc8dv890D8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwR7YHxBBd+NCgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADh0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pd7EPBHwPX81/pp4H2AXf7tEP9+dwHX89/je4D35t8O8e9n/nuJfzvEv5/57/M3wEvzb4f49zP/PS4Brw38Nf92iH8/8y/7EuCX+Y/118Au/z6Ifz/zonkf4Lv5nwXx72dedO8DfDf/cyD+/cy/zvsA383/DIh/P/O8vgT4JF6w9wG+m/9+iH8/87xeB3gw8F28YO8DfDf/vRD/fuZ5vQ7w28B7A9/FC/Y+wHfz3wfx72ee1+sAv80V7w18Fy/Y+wDfzX8PxL+feV6vA/w2z/bewHfxgr0P8N3810P8+5nn9TrAb/Oc3hv4Ll6w9wG+m/9aiH8/87xeB/htntd7A9/FC/Y+wHfzXwfx72ee1+sAv83z997Ad/GCvQ/w3fzXQPz7mef1OsBv84K9N/BdvGDvA3w3//kQ/37meb0O8Nu8cO8NfBcv2PsA381/LsS/n3lerwP8Nv+y9wa+ixfsfYDv5j8P4t/PPK/XAX6bF817A9/FC/Y+wHfznwPx72ee1+sAv82L7r2B7+IFex/gu/mPh/j3M8/rdYDf5l/nvYHv4gV7H+C7+Y+F+Pczz+t1gN/mX++9ge/iBXsf4Lv5j4P49zPP63WA3+bf5r2B7+IFex/gu/mPgfj3M8/rdYDf5t/uvYHv4gV7H+C7+fdD/PuZ5/U6wG/z7/PewHfxgr0P8N38+yD+/czzeh3gt/n3e2/gu3jBXgf4bf7tEP9+5nm9DvDb/Md4b+C7eP7+GngZ/u0Q/37meb0O8Nv8x3lv4Lt4/sS/HeLf76eBt+LZLgEPBnb5j/XewHfxnL4HeG/+7RD/fseBrwbeC/gb4L2Bv+Y/x2sDXw28FPA9wEcDu/zbIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj33BcUGhRSM5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pc7DnwX8Nb817obeBXgGfzbIf79fhp4K/573AY8iH87xL+f+e8l/u0Q/35/DbwU/33Evx3i3++lgd8GjvHfQ/zbIf5jPBh4MP+x3hj4JP5l4t8O8T/TewPfxYtG/Nsh/ud5b+C7eNGJfzvE/yzvDXwX/zri3w7xP8d7A9/FC/YlwCfxvMS/HeJ/hvcGvosX7H2AW4Hf4nmJfzvEf7/3Br6LF+x9gO8GXhv4LZ6X+LdD/Pd6b+C7eMHeB/hurnht4Ld4XuLfDvHf572B7+IFex/gu3m21wZ+i+cl/u0Q/z3eG/guXrD3Ab6b5/TawG/xvMS/HeK/3nsD38UL9j7Ad/O8Xhv4LZ6X+LdD/Nd6b+C7eMHeB/hunr/XBn6L5yX+7RD/dd4b+C5esPcBvpsX7LWB3+J5iX87xH+N9wa+ixfsfYDv5oV7beC3eF7i3w7xn++9ge/iBXsf4Lv5l7028Fs8L/Fvh/jP9d7Ad/GCvQ/w3bxoXhv4LZ6X+LdD/Od5b+C7eMHeB/huXnSvDfwWz0v82yH+c7w38F28YO8DfDf/Oq8N/BbPS/zbIf7jvTfwXbxg7wN8N/96rw38Fs9L/Nsh/mO9N/BdvGDvA3w3/zavDfwWz0v82yH+47w38F28YO8DfDf/dq8N/BbPS/zbIf5jvDfwXbxg7wN8N/8+rw38Fs9L/Nsh/v1eG/gtXrD3Ab6bf7/XBn6L5yX+7RD/fn8FvDTP3/sA381/jNcGfovnJf7tEP9+5vl7H+C7+Y/z2sBv8bzEvx3i3++ngbfiOb0P8N38x3ow8NfAMZ7te4D35t8O8e93HPhu4K2AvwE+Gvht/nO8NPDdwEsB3wN8NLDLvx3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8Ib+VUQZjZiLUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dHccXX8N8D8d/jQcDjgQVXLIHHAM/gvxbiv95x4O+BG3lOdwIvDuzyXwfxX+s48FvAS/P8/TXwOsAu/zUQ/3WOA78FvDQv3F8DrwPs8p8P8V/jOPBbwEvzovlr4HWAXf5zIf7zHQd+C3hp/nX+GngdYJf/PIj/XMeB3wJemn+bvwZeB9jlPwfiP89x4LeAl+bf56+B1wF2+Y+H+M9xHPgt4KV5/u4BruM53QNcx/P318DrALv8x0L8xzsO/Bbw0jx/3wM8GHgtntPvALcC78Xz99fA6wC7/MdB/Mc6DvwW8NI8f98DvDfw28Br8Zx+B3ht4LuB9+L5+2vgdYBd/mMg/uMcB34LeGmev+8B3psrfht4LZ7T7wCvzRXfDbwXz99fA68D7PLvh/iPcRz4LeClef6+B3hvnu23gdfiOf0O8No823cD78Xz99fA6wC7/Psg/v2OA78FvDTP3/cA781z+m3gtXhOvwO8Ns/pu4H34vn7a+B1gF3+7RD/PseB3wJemufve4D35nn9NvBaPKffAV6b5/XdwHvx/P018DrALv82iH+748BvAS/N8/c9wHvz/P028Fo8p98BXpvn77uB9+L5+2vgdYBd/vUQ/zbHgd8CXprn73uA9+YF+23gtXhOvwO8Ni/YdwPvxfP318DrALv86yD+9Y4DvwW8NM/f9wDvzQv328Br8Zx+B3htXrjvBt6L5++vgdcBdnnRIf51jgO/Bbw0z9/3AO/Nv+y3gdfiOf0O8Nr8y74beC+ev78GXgfY5UWD+Nf5K+Clef6+B3hvXjS/DbwWz+l3gNfmRfPdwHvx/P018DK8aBAvutcGfovn73uA9+ZF99vAa/Gcfgd4bV503w28F8/f6wC/zb8M8aJ7beC3eF7fA7w3/zq/DbwWz+l3gNfmX+e7gffieb0O8Nv8yxD/On8NvBTP9j3Ae/Ov99vAa/Gcfgd4bf71vht4L57tb4CX5kWD+Nc5Drw38GDgt4Gf5t/mt4HX4jn9DvDa/Nu8NfDawK3AdwO7vGgQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tj8DvDX/tRD/PT4a+Cqe08cAX81/LcR/j+PAbwMvxRV/A7w2sMt/LcR/r9fmit/mvwfi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPXt6hBH0TlPAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz99LAy8FvDTw0vzPcyvw18BfA38D7PJvg3hODwZ+Cnhp/vfYBd4H+Gn+9RDP9t7AVwHH+d/pp4G34V8HccVrA7/F/35fA3w0LzoEHAf+Cngw/ze8DvDbvGgQ8NHAV/G8LgF/zf9cDwYexPP6HeC1edEg4KeBt+I5PQN4aWCX/9m+G3gvnpd40SDgr4CX5jl9DvDZ/M/3YODpPK+XAf6afxkCzPN6HeC3+d/BPK/XAX6bfxkCzPN6HeC3+d/BPK/XAX6bfxkCzPN6HeC3+d/BPK/XAX6bfxkCzPN6HeC3+d/BPK/XAX6bfxkCzPN6HeC3+d/BPK/XAX6bfxkCzPN6HeC3+d/BPK/XAX6b5++lgWPAM4BbBZjn9TrAb/O/g3lerwP8Ns/rs4HP4opd4HUEmOf1OsBv87+DeV6vA/w2z+mlgb/iOf21APO8Xgf4bf53MM/rdYDf5jm9NvBbPCcEmOf1OsBv87+DeV6vA/w2z+m1gd/iOSHAPK/XAX6b/x3M83od4Ld5Tq8N/BbPCQHmeb0O8Nv872Ce1+sAv81zem3gt3hOCDDP63WA3+Z/B/O8Xgf4bZ7TawO/xXNCgHlerwP8Nv87mOf1OsBv85xeG/gtnhMCzPN6HeC3+d/BPK/XAX6b5/TSwF/xnP5GgHlerwP8Nv87mOf1OsBv87w+G/gsrrgEvLYA87xeB/ht/ncwz+t1gN/m+Xtp4Djw18CuAPO8Xgf4bf53MM/rdYDf5l+GAPO8Xgf4bf53MM/rdYDf5l+GAPO8Xgf4bf53MM/rdYDf5l+GAPO8Xgf4bf53MM/rdYDf5l+GAPO8Xgf4bf53MM/rdYDf5l+GgL8GXorn9DnAZ/M/34OBp/O8Xgb4a/5lCPhp4K14TrcCLwPs8j/bdwHvzfMSLxoEfDTwVTyvXeCv+Z/rwcCDeV6/A7w2LxoEHAf+GngQ/ze8DvDbvGgQV7w28Fv87/c1wEfzokM823sDXw0c43+nnwHemn8dxHN6MPDTwEvxv8cl4L2Bn+ZfD/H8vTTw0sBLAy/N/zy3An8N/DXw18Au/zaI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EG/ieOnpo+lAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t/ss4KOB4/zn2AX+GrgV+GngZ/iPh/i3eW/gu/ivtQt8NfA5/MdB/Nt8N/Be/Pe4FXgb4K/590P82/w28Fr899kF3gf4af59EP82vw28Fv/9Xgf4bf7tEP82vw28Fs/pb4CP5j/Gg4GXBl4aeGngGM/fLvA6wF/zb4P4t/lt4LV4Tr8DvDb/8Y4D3w28Fc/fbwOvw78N4t/mt4HX4jn9DvDa/Od5a+CneP5eB/ht/vUQ/za/DbwWz+l3gNfmP9dHA1/F8/pr4GX410P82/w28Fo8p98BXpv/fL8NvBbP6wSwy78O4t/mt4HX4jn9DvDa/Od7beC3eF7vA3w3/zqIf5vfBl6L5/Q7wGvzX8M8r88BPpt/HcS/zW8Dr8Vz+h3gtfmv8dfAS/GcPgf4bP51EP82vw28Fs/pd4DX5r/GbwOvxXP6GeCt+ddB/Nv8NvBaPKffAV6b/xpPBx7Mc/oc4LP510H82/w28Fo8p98BXpv/fMeBizyvzwE+m38dxL/NbwOvxXP6HeC1+c/33sB38bzeBvhp/nUQ/za/DbwWz+l3gNfmP9dx4OnAcZ7TJeA4/3qIf5vfBl6L5/Q7wGvzn+ungLfmeX0P8N786yH+bX4beC2e0+8Ar81/juPAdwFvzfP3EOBW/vUQ/za/DbwWz+l3gNfmP9Zx4K2ArwaO8/x9DfDR/Nsg/m1+G3gtntMEHPAfZw7MeeH+Bnhp/u0Q/zZ/Drwc/73+HngNYJd/O8S/ze3ATfz3ug34KOCn+bdD/NvcDtzE/wxfDXwM/zaIf5s/B16O/zm+G3gf/vUQ/za/DbwWz2kCDviPMQEHXHEAvDj/so8Bvpp/HcS/zW8Dr8Vz+h3gtfnP82Dgo4GP4gV7HeC3edEh/m1+G3gtntPvAK/Nf77XBr4beBDP67eB1+FFh/i3+W3gtXhOvwO8Nv81jgO3Asd4Xu8DfDcvGsS/zW8Dr8Vz+h3gtfmv89bAT/G8vgd4b140iH+b3wZei+f0O8Br81/rp4G34jntAid40SD+bX4beC2e0+8Ar81/rY8Gvorn9RDgVv5liH+b3wZei+f0O8Br81/rtYHf4nm9DvDb/MsQ/za/DbwWz+l3gNfmv555Xq8D/Db/MsS/zW8Dr8Vz+h3gtfmv9dLAX/G8Xgf4bf5liH+b3wZei+f0O8Br81/rvYHv4nm9DvDb/MsQ/za/DbwWz+l3gNfmv9ZXAx/F8xIvGsS/zW8Dr8Vz+h3gtfmv82Dgr4DjPKefAd6aFw3i3+a3gdfiOf0O8Nr81/kt4LV5Xh8DfDUvGsS/zW8Dr8Vz+h3gtfnPdxz4KuC9eV6XgAcDu7xoEP82vw28Fs/pd4DX5j/XawPfBTyY5+9jgK/mRYf4t/lt4LV4Tn8NfDT/8V4aeGngpYGX5gX7HeC1+ddB/Nv8NvBa/M/xN8BrA7v86yD+bX4beC3+Z/gb4K2BW/nXQ/zbfDfwXvz3+xngvYFd/m0Q/zZvDfwU/33+Bvho4Lf590H823028NHAMf7z/Q2wC/w28NPAX/MfA/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BD/lt0EZPrYVAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP6TjwXsBb8++zC/w1V/w18DP827wV8NJc8dLAcf59fhr4HmCXKxDP6a+Al+Y/3i7w08DnALfywj0Y+CzgrYHj/Mf7a+BluALxbK8N/Bb/+b4a+Bie13Hgs4CP5j/f6wC/DSCe7bWB3+K/xl8DrwPscsVx4LeAl+a/xusAvw0gnu21gd/iv85fA6/DFU8HjvNf53WA3wYQz/bawG/xvH6Hf5sHAw/ihfs7oAEvzQv3DOBW/m1ei+f1OsBvA4hne23gt3he4t/uOPDWwFcDx/jXuQR8NPDTwC7/duZ5vQ7w2wDi2V4b+C2el/j3Ow58N/BWvGh+BnhvYJd/P/O8Xgf4bQDxbK8N/BbPS/zH+WngrXjhfgZ4a/7jmOf1OsBvA4hne23gt3he4j/OceBW4BjP3yXgwcAu/3HM83od4LcBxLO9NvBbPC/xH+u9ge/i+Xsf4Lv5j2We1+sAvw0gnu21gd/ieYn/WMeBizx/J4Bd/mOZ5/U6wG8DiGd7beC3eF7iP95F4DjPaRc4wX8887xeB/htAPFsrw38Fs9L/Mc7C5zmOZ0DzvAfzzyv1wF+G0A822sDv8XzEv/xLgLHeU67wAn+45nn9TrAbwOIZ3tt4Ld4XuI/3kXgOM9pFzjBfzzzvF4H+G0A8WyvDfwWz0v8x7sIHOc57QIn+I9nntfrAL8NIJ7ttYHf4nmJ/3gXgeM8p13gBP/xzPN6HeC3AcSzvTbwWzwv8R/vInCc57QLnOA/nnlerwP8NoB4ttcGfovnJf7jXQSO85x2gRP8xzPP63WA3wYQz/bawG/xvMR/vIvAcZ7TLnCC/3jmeb0O8NsA4tleG/gtnpf4j3cROM5z2gVO8B/PPK/XAX4bQDzbawO/xfMS//EuAsd5TrvACf7jmef1OsBvA4hne23gt3he4j/eReA4z2kXOMF/PPO8Xgf4bQDxbK8N/BbPS/zHuwgc5zntAif4j2ee1+sAvw0gnu21gd/ieYn/eBeB4zynXeAE//HM83od4LcBxLO9NvBbPC/xH+8icJzntAuc4D+eeV6vA/w2gHi21wZ+i+cl/uNdBI7znHaBE/zHM8/rdYDfBhDP9trAb/G8xH+8i8BxntMucIL/eOZ5vQ7w2wDi2V4b+C2el/iPdxE4znPaBU7wH888r9cBfhtAPNtrA7/F8xL/8S4Cx3lOu8AJ/uOZ5/U6wG8DiGd7beC3eF7iP95F4DjPaRc4wX8887xeB/htAPFsrw38Fs9L/Me7CBznOe0CJ/iPZ57X6wC/DSCe7bWB3+J5if94F4HjPKdd4AT/8czzeh3gtwHEs7028Fs8L/Ef7yJwnOe0C5zgP555Xq8D/DaAeLbXBn6L5yX+410EjvOcdoET/Mczz+t1gN8GEM/22sBv8bzEf7yLwHGe0y5wgv945nm9DvDbAOLZXhv4LZ6X+I93ETjOc9oFTvAfzzyv1wF+G0A822sDv8XzEv/xLgLHeU67wAn+45nn9TrAbwOIZ3tt4Ld4XuI/3kXgOM9pFzjBfzzzvF4H+G0A8WyvDfwWz+u3+fe5FbgVuBX4GWAXuAgc5zntAieA48BbAQ8GHgw8mH+f1+Z5vQ7w2wDi2V4b+C3+83038PbAFs/pAPhx4L35z/c6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gntNfAy/F/21/A7w0VyCe03HgvYHXBo7zH+e1+Lf5Hf7j7AK/DXw3sMsViP86bw18N3CMF+4S8N7AT/OfD/Ff66WB3waO8fxdAl4auJX/Goj/em8N/BTP39sAP81/HcR/D/P8if9aiP8eF4HjPKdd4AT/tRD/Pf4SeBme018BL8t/LcR/j3cCfpjn9M7Aj/BfC/Hf552AT+KKLwF+hP96iP/fEP+/If5/Q/z/hvj/jX8ESLn6Qcb+kmoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf76WBY/zP8wzgVl44xL/NZwGvDbw2/3PtAq8D/DUvGOJf56WB7wJemv8d/hp4GV4wxIvurYHvAo7zv4t4wRAvmuPA04Hj/O8jXjDEi+angLfmfyfxgiH+ZW8N/BTP6xnARwN/DdzKf7/XBn6L5yVeMMS/7LOBz+I5XQIeDOzyP8drA7/F8xIvGOJf9tvAa/Gcvgb4aP5neW3gt3he4gVD/MsuAsd5Th8DfDX/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv80VLw0c47/fSwNfzfMSLxjiX2ae13cDDwZem//5xAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5nn7xnARwN/DdzKf7/XBn6L5yVeMMS/zDyvA+BmYJf/OV4b+C2el3jBEP8y87x+Anh7/md5beC3eF7iBUP8y8zz+gbgw/mf5bWB3+J5iRcM8S9LQDynbwA+nP9ZXhv4LZ6XeMEQ/7KnAw/mOX0N8NH8z/LawG/xvMQLhviXfTbwWTynXeAhwC7/c7w28Fs8L/GCIf5lrw38Fs/rVuCjgb8BbuW/32sDv8XzEi8Y4kXz08Bb8b+TeMEQL5rjwK3AMf73ES8Y4kX31sB3A8f430W8YIh/nZcGvht4Kf53+BvgpXnBEP82nw28NvBa/M91CXht4K95wRD/fi8NHOd/nr8GdnnhEP+/If5/4x8BdJekQVESp6IAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD0ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7ccB94LeGngZ4Cf5oVD/N9xHPgt4KV5ts8BPpsXDPF/w3Hgt4CX5jntAid4wRD/+x0Hfgt4aZ7XJeA4Lxjif7fjwG8BL83z9znAZ/OCIf73Og78FvDSPH/fA7w3Lxzif6fjwG8BL83z9z3Ae/MvQ/zvcxz4LeClef6+B3hvXjSI/12OA78FvDTP3/cA782LDvG/x3Hgt4CX5vn7HuC9+ddB/O9wHPgt4KV5/r4HeG/+9RD/8x0Hfgt4aZ6/7wHem38bxP9sx4HfAl6a5+97gPfm3w7xP9dx4LeAl+b5+x7gvfn3QfzPdBz4LeClef6+B3hv/v0Q//McB34LeGmev+8B3pv/GIgXzVsDbwX8NfA9wC7/OY4DvwW8NM/f9wDvzX8cxL/ss4HP4tn+GngdYJf/WMeB3wJemufve4D35j8W4l92ETjOc/pr4HWAXf5jHAd+C3hpnr/vAd6b/3iIf9kucIzn9dfA6wC7/PscB34LeGmev+8B3pv/HIh/2WcDn8Xz99fA6wC7/NscB34LeGmev+8B3pv/PIgXzXcD78Xz99fA6wC7/OscB34LeGmev+8B3pv/XIgX3XcD78Xz99fA6wC7vGiOA78FvDTP3/cA781/PsS/zncD78Xz99fA6wC7vHDHgd8CXprn73uA9+a/BuJf77uB9+L5+2vgdYBdnr/jwG8BL83z9z3Ae/NfB/Fv893Ae/H8/TXwOsAuz+k48FvAS/P8fQ/w3vzXQvzbfTfwXjx/fw28DrDLFceB3wJemufve4D35r8e4t/nu4H34vn7a+B1uOK3gJfm+fse4L3574H49/tu4L14/v6aK16a5+97gPfmvw/iP8Z3A+/Fv873AO/Nfy/Ef5zvBt6LF833AO/Nfz/Ef6zvBt6LF+57gPfmfwbEf7zvBt6L5+97gPfmfw7Ef47vBt6L5/Q9wHvzPwviP89nAx/NFV8NfDb/8yD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9O1WpBQXWXMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "20 6 9 17 4 12",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP92Lw28F1d8D/DX/Nd6aeC9uOJ7gL/mXw/xb/PSwF/xnN4H+G7+a7w38F08p5cB/pp/HcS/zVcDH8Xzeh/gu/nP9d7Ad/G8vgb4aP51EP82Xw18FM/f+wDfzX+O9wa+i+fva4CP5l8H8W/z0sBf8YK9D/Dd/Md6b+C7eMFeBvhr/nUQ/3bvDXwXL9j7AN/Nf4z3Br6LF+x9gO/mXw/x7/PewHfxgr0P8N38+7w38F28YO8DfDf/Noh/v/cGvosX7H2A7+bf5r2B7+IFex/gu/m3Q/zHeG/gu3jB3gf4bv513hv4Ll6w9wG+m38fxH+c9wa+ixfsfYDv5kXz3sB38YK9D/Dd/Psh/mO9N/BdvGDvA3w3L9x7A9/FC/Y+wHfzHwPxH++9ge/iBXsf4Lt5/t4b+C5esPcBvpv/OIj/HO8NfBcv2PsA381zem/gu3jB3gf4bv5jIf7zvDfwXbxg7wN8N1e8N/BdvGDvA3w3//EQ/7neG/guXrD34Yrv4gV7H+C7+c+B+M/33sB38W/zPsB3858H8V/jvYHv4l/nfYDv5j8X4r/OewPfxYvmfYDv5j8f4r/WewPfxQv3PsB3818D8V/vvYHv4vl7H+C7+a+D+O/x3sB38ZzeB/hu/msh/vu8NPDeXPHdwF/zXw/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPzctJQUeZzeQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "6 9 12 15 18 9",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3O9NPBeXPE9wF/zHw/xP9N7A9/Fc3oZ4K/5j4X4n+e9ge/ieX0N8NH8x0L8z/LewHfx/H0N8NH8x0L8z/HewHfxgr0M8Nf8x0L8z/DewHfxgr0P8N38x0P893tv4Lt4wd4H+G7+cyD+e7038F28YO8DfDf/eRD/fd4b+C5esPcBvpv/XIj/Hu8NfBcv2PsA381/PsR/vfcGvosX7H2A7+a/BuK/1nsD38UL9j7Ad/NfB/Ff572B7+IFex/gu/mvhfiv8d7Ad/GCvQ/w3fzXQ/zne2/gu3jB3gf4bv57IP5zvTfwXbxg7wN8N/99EP953hv4Ll6w9wG+m/9eiP8cLw38FS/Y+wDfzX8/xH+OrwY+iufvfYDv5n8GxH+OrwY+iufvfYDv5n8GxH+Olwb+ihfsfYDv5r8f4j/PewPfxQv2PsB3898L8Z/rvYHv4gV7H+C7+e+D+M/33sB38YK9D/Dd/PdA/Nd4b+C7eMHeB/hu/ush/uu8N/BdvGDvA3w3/7UQ/7XeG/guXrD3Ab6b/zqI/3rvDXwXL9j7AN/Nfw3Ef4/3Br6LF+x9gO/mPx/iv897A9/FC/Y+wHfznwvx3+u9ge/iBXsf4Lv5z4P47/fewHfxgr0P8N3850D8z/DewHfxgr0P8N38x0P8z/HewHfxgr0M8Nf8x0L8z/LewHfx/H0N8NH8x0L8z/PewHfxvL4G+Gj+YyH+Z3pv4Lt4Ti8D/DX/sRD/c7008N5c8d3AX/MfD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj8IBTEG5Qx4SAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "15 18 9 12 15 6",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3leGngvrvge4K/5nwfxn+Olgb/iOb0P8N38z4L4z/HVwEfxvN4H+G7+50D85/hq4KN4/t4H+G7+Z0D853hp4K94wd4H+G7++yH+87w38F28YO8DfDf/vRD/ud4b+C5esPcBvpv/Poj/fO8NfBcv2PsA381/D8R/jfcGvosX7H2A7+a/HuK/znsD38UL9j7Ad/NfC/Ff672B7+IFex/gu/mvg/iv997Ad/GCvQ/w3fzXQPz3eG/gu3jB3gf4bv7zIf77vDfwXbxg7wN8N/+5EP+93hv4Ll6w9wG+m/88iP9+7w18Fy/Y+wDfzX8OxP8M7w18Fy/Y+wDfzX88xP8c7w18Fy/YywB/zX8sxP8s7w18F8/f1wAfzX8sxP8s7w18F8/f1wAfzX8sxP8c7w18Fy/YywB/zX8sxP8M7w18Fy/Y+wDfzX88xH+/9wa+ixfsfYDv5j8H4r/XewPfxQv2PsB3858H8d/nvYHv4gV7H+C7+c+F+O/x3sB38YK9D/Dd/OdD/Nd7b+C7eMHeB/hu/msg/mu9N/BdvGDvA3w3/3UQ/3XeG/guXrD3Ab6b/1qI/xrvDXwXL9j7AN/Nfz3Ef773Br6LF+x9gO/mvwfiP9d7A9/FC/Y+wHfz3wfxn+e9ge/iBXsf4Lv574X4z/HSwF/xgr0P8N3890P85/hq4KN4/t4H+G7+Z0D85/hq4KN4Xu8DfDf/cyD+c7w08Fc8p/cBvpv/WRD/eV4aeG+u+G7gr/mfB/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj7IuTEHB82tnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "9 18 15 12 9 6",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP99Xhp4L674HuCv+a+H+O/x3sB38ZzeB/hu/msh/uu9N/BdPH/vA3w3/3UQ/7XeG/guXrj3Ab6b/xqI/zrvDXwXL5r3Ab6b/3yI/xrvDXwX/zrvA3w3/7kQ//neG/gu/m3eB/hu/vMg/nO9N/BdvGDvwxXfxQv2PsB3858D8Z/nvYHv4gV7H+C7ueK9ge/iBXsf4Lv5j4f4z/HewHfxgr0P8N08p/cGvosX7H2A7+Y/FuI/3nsD38UL9j7Ad/P8vTfwXbxg7wN8N/9xEP+x3hv4Ll6w9wG+mxfuvYHv4gV7H+C7+Y+B+I/z3sB38YK9D/DdvGjeG/guXrD3Ab6bfz/Ef4z3Br6LF+x9gO/mX+e9ge/iBXsf4Lv590H8+7038F28YO8DfDf/Nu8NfBcv2PsA382/HeLf572B7+IFex/gu/n3eW/gu3jB3gf4bv5tEP927w18Fy/Y+wDfzX+M9wa+ixfsfYDv5l8P8W/z0sBf8YK9D/Dd/Md6b+C7eMFeBvhr/nUQ/zZfDXwUz9/7AN/Nf473Br6L5+9rgI/mXwfxb/PVwEfxvN4H+G7+c7038F08r68BPpp/HcS/zUsDf8Vzeh/gu/mv8d7Ad/GcXgb4a/51EP92Lw28N1d8N/DX/Nd6aeC9ueK7gb/mXw/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP6A1JQRktpBUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polyline {
                points: "18 15 12 9 6 15",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zHeGngr4K+B7wF2+c9xHHgv4KWBnwF+mn8fxL/fZwOfxbP9NfA6wC7/sY4DvwW8NM/2OcBn82+H+Pe7CBznOf018DrALv8xjgO/Bbw0z2kXOMG/HeLfbxc4xvP6a+B1gF3+fY4DvwW8NM/rEnCcfzvEv99nA5/F8/fXwOsAu/zbHAd+C3hpnr/PAT6bfzvEf4zvBt6L5++vgdcBdvnXOQ78FvDSPH/fA7w3/z6I/zjfDbwXz99fA68D7PKiOQ78FvDSPH/fA7w3/36I/1jfDbwXz99fA68D7PLCHQd+C3hpnr/vAd6b/xiI/3jfDbwXz99fA68D7PL8HQd+C3hpnr/vAd6b/ziI/xzfDbwXz99fA68D7PKcjgO/Bbw0z9/3AO/NfyzEf57vBt6L5++vgdcBdrniOPBbwEvz/H0P8N78x0P85/pu4L14/v4aeB2u+C3gpXn+vgd4b/5zIP7zfTfwXjx/f80VL83z9z3Ae/OfB/Ff47uB9+Jf53uA9+Y/F+K/zncD78WL5nuA9+Y/H+K/1ncD78UL9z3Ae/NfA/Ff77uB9+L5+x7gvfmvg/jv8d3Ae/Gcvgd4b/5rIf77fDbw0Vzx1cBn818P8e93HPgu4K2BvwY+Bvht/nO8NvBVwEsDPw28D7DLvx3i3++ngbfiOb0P8N38x3pv4Lt4Tj8DvDX/doh/P/P8vQ/w3fzHeG/gu3j+xL8d4t/vr4GX4vl7H+C7+fd5b+C7eP7+Bnhp/u0Q/36vDfwWL9j7AN/Nv817A9/FC/Y6wG/zb4f4j/HewHfxgr0P8N3867w38F28YO8DfDf/Poj/OO8NfBcv2PsA382L5r2B7+IFex/gu/n3Q/zHem/gu3jB3gf4bl649wa+ixfsfYDv5j8G4j/eewPfxQv2PsB38/y9N/BdvGDvA3w3/3EQ/zneG/guXrD3Ab6b5/TewHfxgr0P8N38x0L853lv4Lt4wd4H+G6ueG/gu3jB3gf4bv7jIf5zvTfwXbxg78MV38UL9j7Ad/OfA/Gf772B7+Lf5n2A7+Y/D+K/xnsD38W/zvsA381/LsR/nfcGvosXzfsA381/PsR/rfcGvosX7n2A7+a/BuK/3nsD38Xz9z7Ad/NfB/Hf472B7+I5vQ/w3fzXQvz3eWngvbniu4G/5r8e4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfATtNiEGVrtmMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K/12sBXAS8N/DTwPsAu/zafBXw0V3w18Dn86yH+67w38F08p+8B3pt/ve8C3pvn9DbAT/Ovg/iv8d7Ad/H8iX+d7wLem+f1NcBH86+D+M/33sB38fz9DfDSvOi+C3hvnr+PAb6afx3Ef673Br6LF+x1gN/mRfNdwHvz/P0N8NrALv86iP887w18Fy/Y+wDfzYvmu4D35vn7G+C1gV3+9RD/Od4b+C5esPcBvpsXzXcB783z9zfAawO7/Nsg/uO9N/BdvGDvA3w3L5rvAt6b5+9vgNcGdvm3Q/zHem/gu3jB3gf4bl403wW8N8/f3wCvDezy74P4j/PewHfxgr0P8N28aL4LeG+ev78BXhvY5d8P8R/jvYHv4gV7H+C7edF8F/DePH9/A7w2sMt/DMS/33sD38UL9j7Ad/Oi+S7gvXn+/gZ4bWCX/ziIf5/3Br6LF+x9gO/mRfNdwHvz/P0N8NrALv+xEP927w18Fy/Y+wDfzYvmu4D35vn7G+C1gV3+4yH+bV4a+CtesPcBvpsXzWcDn8Xz9zfAawO7/OdA/Nt8NfBRPH/vA3w3L7qLwHGe198Arw3s8p8H8W/z1cBH8fy9D/DdvOh2gWM8r78GXgfY5T8P4t/mpYG/4gV7H+C7edF8NvBZPH9/DbwOsMt/DsS/3XsD38UL9j7Ad/Oi+W7gvXj+/hp4HWCX/3iIf5/3Br6LF+x9gO/mRfPdwHvx/P018DrALv+xEP9+7w18Fy/Y+wDfzYvmu4H34vn7a+B1gF3+4yD+Y7w38F28YO8DfDcvmu8G3ovn76+B1wF2+Y+B+I/z3sB38YK9D/DdvGi+G3gvnr+/Bl4H2OXfD/Ef672B7+IFex/gu3nRfDfwXjx/fw28DrDLvw/iP957A9/FC/Y+wHfzovlu4L14/v4aeB1gl387xH+O9wa+ixfsfYDv5kXz3cB78fz9NfA6wC7/Noj/PO8NfBcv2PsA382L5ruB9+L5+2vgdYBd/vUQ/7neG/guXrDXAX6bF813A+/F8/fXwOsAu/zrIP7zvTfwXTx/fw28DC+67wbei+fvY4Cv5l8H8V/jvYHv4vkT/zrfDbwXz+trgI/mXwfxX+e9ge/iOX0P8N7863038F48p7cBfpp/HcR/rdcGvhp4KeB7gI8Gdvm3+Wzgo7niq4HP5l8P8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BG/pmEHmbtH/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Lf7LOCjueKrgc/h3+Y48F3AWwN/DXwM8Nv810D827w38F08p+8G3od/vZ8G3orn9D7Ad/OfD/Fv893Ae/G8vht4H/51zPP3PsB3858L8W/z0cBX8fx9N/A+vOj+Gngpnr/3Ab6b/zyIf5vjwG8DL8Xz993A+/CieW3gt3jB3gf4bv5zIP7tjgO/DbwUz993A+/Di+a9ge/iBXsf4Lv5j4f49zkO/DbwUjx/3w28Dy+a9wa+ixfsfYDv5j8W4t/vOPDbwEvx/H038D68aN4b+C5esPcBvpv/OIj/GMeB3wZeiufvu4H34UXz3sB38YK9D/Dd/MdA/Mc5Dvw28FI8f98NvA8vmvcGvosX7H2A7+bfD/Ef6zjw28BL8fx9N/A+vGjeG/guXrD3Ab6bfx/Ef7zjwG8DL8Xz993A+/CieW/gu3jB3gf4bv7tEP85jgO/DbwUz993A+/Di+a9ge/iBXsf4Lv5t0H85zkO/DbwUjx/3w28Dy+a9wa+ixfsfYDv5l8P8Z/rOPDbwEvx/H0O8Nm8aN4b+C5esJcB/pp/HcR/vuPAbwMvxfPaBU7wontv4Lt4/r4G+Gj+dRD/+Y4DvwW8NM/rGcCDedG9N/BdPH9fA3w0/zqI/1zHgd8CXprn73OAz+ZF897Ad/GCvQzw1/zrIP7zHAd+C3hpnr/vAd6bF817A9/FC/Y+wHfzr4f4z3Ec+C3gpXn+vgd4b1407w18Fy/Y+wDfzb8N4j/eceC3gJfm+fse4L150bw38F28YO8DfDf/doj/WMeB3wJemufve4D35kXz3sB38YK9D/Dd/Psg/uMcB34LeGmev+8B3psXzXsD38UL9j7Ad/Pvh/iPcRz4LeClef6+B3hvXjTvDXwXL9j7AN/NfwzEv99x4LeAl+b5+x7gvXnRvDfwXbxg7wN8N/9xEP8+x4HfAl6a5+97gPfmRfPewHfxgr0P8N38x0L82x0Hfgt4aZ6/7wHemxfNewPfxQv2PsB38x8P8W9zHPgt4KV5/r4HeG9eNK8N/BYv2PsA381/DsS/zUcDX8Xz9z3Ae/Oi+yvgpXn+3gf4bv7zIP5tvht4L57X9wDvzb+Oef7eB/hu/nMh/m3eGvgpntP3AO/Nv95PA2/Fc3of4Lv5z4f4t/ts4KO54quBz+bf5jjw3cBbAX8DfDTw2/zXQPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHVQ4JBco/gfAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEW0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/31eGngvrvge4K/5r4f47/HewHfxnN4H+G7+ayH+67038F08f+8DfDf/dRD/td4b+C5euPcBvpv/Goj/Ou8NfBcvmvcBvpv/fIj/Gu8NfBf/Ou8DfDf/uRD/+d4b+C7+bd4H+G7+8yD+c7038F28YO/DFd/FC/Y+wHfznwPxn+e9ge/iBXsf4Lu54r2B7+IFex/gu/mPh/jP8d7Ad/GCvQ/w3Tyn9wa+ixfsfYDv5j8W4j/eewPfxQv2PsB38/y9N/BdvGDvA3w3/3EQ/7HeG/guXrD3Ab6bF+69ge/iBXsf4Lv5j4H4j/PewHfxgr0P8N28aN4b+C5esPcBvpt/P8R/jPcGvosX7H2A7+Zf572B7+IFex/gu/n3Qfz7vTbwW7xg7wN8N/827w18Fy/Y6wC/zb8d4t/vr4CX5vl7H+C7+fd5b+C7eP7+GngZ/u0Q/37m+Xsf4Lv5j/HewHfx/Il/O8S/308Db8Vzeh/gu/mP9d7Ad/Gcvgd4b/7tEP9+x4GvBt4L+Bvgo4Hf5j/HawNfDbwU8D3ARwO7/Nsh/vt8FvDRXPHVwOfwXw/x3+O7gPfmOX038D7810L81/su4L15/r4beB/+6yD+a30X8N68cN8NvA//NRD/db4LeG9eNN8NvA//+RD/Nb4LeG/+db4beB/+cyH+830X8N48f3/DFS/F8/fdwPvwnwfxn+u7gPfm+fsb4LW54reBl+L5+27gffjPgfjP813Ae/P8/Q3w2sAuVxwHfht4KZ6/7wbeh/94iP8c3wW8N8/f3wCvDezynI4Dvw28FM/fdwPvw38sxH+87wLem+fvb4DXBnZ5/o4Dvw28FM/fdwPvw38cxH+s7wLem+fvb4DXBnZ54Y4Dvw28FM/fdwPvw38MxH+c7wLem+fvb4DXBnZ50RwHfht4KZ6/7wbeh38/xH+M7wLem+fvb4DXBnb51zkO/DbwUjx/3w28D/8+iH+/zwY+i+fvb4DXBnb5tzkO/DbwUjx/nwN8Nv92iH+/i8BxntffAK8N7PLvcxz4beCleF67wAn+7RD/frvAMZ7T3wCvDezyH+M48NvAS/GcLgHH+bdD/Pt9NvBZPNvfAK8N7PIf6zjw28BL8WyfA3w2/3aI/xhvDbw2cCvw3cAu/zmOA+8NPBj4beCn+fdB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjcniIQd3f6RMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFkUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/12vxH+N3+M+B+I93HPgu4K35j/XTwPsAu/zHQfzHOg78FfBg/nP8NfA6wC7/MRD/sX4KeGv+c/008Db8x0D8xzL/NcR/DMR/nNcGfov/Gq8D/Db/foh/vZcG3gp4beClgeO8cOLfx7xwu8BfA78N/Azw17zoEP86HwV8Nf864t/HvOh2gc8GvoYXDeJF91PAW/OvJ/59zL/eTwNvw78M8aL5aOCr+LcR/z7m3+ZjgK/mhUP8y14a+Cv+7cS/j/m32QVeB/hrXjDEv+yzgc/ief0M8NvAX3PFSwNfxfP6bf59Xpvn9THAX3PFSwOvDbwVz+tzgM/mBUP8y34beC2e088Ab81zejDwdP5rPAS4lef028Br8Zx+B3htXjDEv+wicJzn9DHAV/O8/hp4Kf5z/Q3w0jyvjwa+iue0C5zgBUP8y8zzeh3gt3leLw38Ff+5Xgb4a57XawO/xfMSLxjiX2ae1+sAv83z99rATwPH+I91CXhr4Ld5/l4b+C2el3jBEP8y87xeB/htXrDjwFsDLw28NP8+fw38NfDTwC4v2GsDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DfDT/M/y3sB38bzEC4b4l/028Fo8p68BPpr/Wb4a+Cie0+8Ar80LhviXfTXwUTynXeCjgZ8BdvnvdRx4K+C7eV5fA3w0LxjiX/bWwE/xv9PbAD/NC4Z40fw08Fb87/IzwFvzwiFeNMeBW4Fj/O9wCXgwsMsLh3jRPRj4buC1+J/td4D3Bm7lX4b41/to4KOBB/E/yzOArwa+mhcd4t/ntfmf4bf5t0H8/4b4/41/BI5yx0F1Rx7pAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/NWwGsDLw08GHgwz+lW4Fbgr4G/Bn4HuJX/Xoh/u+PAewFvDbw2/zZ/DXw18DPALv/1EP96x4GPAj4aOM5/nK8GPgfY5b8O4l/nq4CP5j/PLvDZwNfwXwPxonkw8FPAS/Nf47eBtwF2+c+F+Je9NfBdwHH+ZZeA3wb+GrgVuBU4Drw08NLAceC1eNHsAq8D/DX/eRAv3HsD38UL9wzgu4GfBv6aF82Dgc8G3ot/2fsA381/DsQL9t7Ad/GCPQP4bOC7+bd7MPDRwHsDx3jB3gf4bv7jIZ6/1wZ+ixfsa4CP5j/OceCrgffiBXsd4Lf5j4V4Xi8N/BZwnOd1CXhv4Kf5z/HewHfx/O0CrwP8Nf9xEM/rr4CX5nldAl4b+Gv+c7008NvAMZ7XXwMvw38cxHP6bOCzeF6XgNcG/pr/Gi8N/BXP38cAX81/DMSzPRh4Os/f+wDfzX+t9wa+i+e1CzwE2OXfD/Fs3w28F8/ra4CP5r/HVwMfxfP6GuCj+fdDXHEceDpwnOf0DOClgV3+exwHbgWO8Zx2gRP8+yGu+Gzgs3he7wN8N/+9Phr4Kp7X+wDfzb8P4oq/Al6a5/QM4MH8z7ALHOMFuxW4FbgV+Gngd4Bd/mUIeDDwdJ7X5wCfzf8M3w28F/863w18DnArLxgC3hv4Lp7XQ4Bb+Z/hpYG/4t/ms4GvAXZ5Xgj4buC9eE7PAB7M/yy3Ag/i3+avgdcBdnlOCPht4LV4Tt8DvDf/s7w28NPAMf5tdoHXAf6aZ0PA04EH85w+B/hs/uc5Drw0z+nBwIOBBwNvDRzjBdsFHgLscgUCzPP6HOCz+d/pvYHPBh7E8/fXwOsAuwACzPN6HeC3+d/tu4H34vn7HOCzAQSY5/U6wG/zv997A9/F8/cQ4FYB5nm9D/Dd/N/w3cB78by+B3hvAX8NvBTP6XOAz+b/jluBB/G8Tgj4aeCteE6/A7w2/3e8N/BdPK+3EfDZwGfxnG4FHsL/LbvAMZ7T9wh4a+CneF4PAW7l/47vBt6L5/Q7Ah4MPJ3n9TXAR/N/x2cDn8Vz+mtxxV8DL8VzuhV4CP93vDfwXTwnxBXvDXwXz+t9gO/m/4b3Br6L54S44jhwkee1CzwE2OV/v88GPovn9Dfi2b4beC+e19cAH83/ft8NvBfP6XfEsz0Y+GvgGM/rfYDv5n+3i8BxntP3iOf00cBX8bx2gdcB/pr/nd4b+C6e19uI5/XXwEvxvHaB1wH+mv99ng48mOd1QjyvlwZ+GzjG89oFPgb4bv73+C7gvXle3wO8t3j+Xhv4LV6w7wbeh//53hv4Lp6/hwC3ihfsvYHv4gW7Ffhs4Hv4n+m7gPfm+fsc4LMBxAv33sB38cLdCnw18DvAX/Pf772AzwYezPP3N8BrA7sA4l/22sBPA8f4l+0Cvw38NVfcCtzKf64HAw8GHgy8NXCcF+wS8GBglysQL5rjwG8DL8X/XpeA1wb+mmdD/Ot8NvDRwDH+d/kb4LWBXZ4T4l/vOPDRwEcDx/if73OArwZ2eV6If7vjwHsD7w28FP/zfA/w2cCtvGCI/xjHgdcGXht4aa54MPAg/mv8DbAL3Ar8NPDbwC7/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EbSj2V6kq4rqAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7GOA+8FvDTwM8BP8x/jrYG3Av4a+B5gl/8YiP84x4HfAl6aZ/sc4LP59/ls4LN4tr8GXgfY5d8P8R/jOPBbwEvznHaBE/z7XASO85z+GngdYJd/H8S/33Hgt4CX5nk9A3gw/z67wDGe118DrwPs8m+H+Pc5DvwW8NI8f58DfDb/Pp8NfBbP318DrwPs8m+D+Lc7DvwW8NI8f98DvDf/Mb4beC+ev78GXgfY5V8P8W9zHPgt4KV5/r4HeG/+Y3038F48f38NvA6wy78O4l/vOPBbwEvz/H0P8N785/hu4L14/v4aeB1glxcd4l/nOPBbwEvz/H0P8N785/pu4L14/v4aeB1glxcN4kV3HPgt4KV5/r4HeG/+a3w38F48f38NvA6wy78M8aI5DvwW8NI8f98DvDf/tb4beC+ev78GXgfY5YVD/MuOA78FvDTP3/cA781/j+8G3ovn76+B1wF2ecEQL9xx4LeAl+b5+x7gvfnv9d3Ae/H8/TXwOsAuzx/iBTsO/Bbw0jx/3wO8N/8zfDfwXjx/fw28DrDL80I8f8eB3wJemufve4D35n+W7wbei+fvr4HXAXZ5TojndRz4LeClef6+B3hv/mf6buC9eP7+GngdYJdnQzyn48BvAS/N8/c9wHvzP9t3A+/F8/fXwOsAu1yBeE5/Bbw0z9/3AO/N/w7fDbwXz99fAy/DFYhne23gt3j+vgd4b/53+W7gvXj+Xgf4bQDxbK8N/BbP33cD78P/Lt8FvDfP3+sAvw0gntNfAy/F8/fdwPvwv8N3Ae/N8/c3wEtzBeI5HQd+G3gpnr/vBt6H/9m+C3hvnr+/AV4b2OUKxPM6Dvw28FI8f98NvA//M30X8N48f38DvDawy7Mhnr/jwG8DL8Xz993A+/A/y3cB783z9zfAawO7PCfEC3Yc+G3gpXj+vht4H/5n+C7gvXn+/gZ4bWCX54V44Y4Dvw28FM/fdwPvw3+v7wLem+fvb4DXBnZ5/hD/suPAbwMvxfP33cD78N/ju4D35vn7G+C1gV1eMMSL5jjw28BL8fx9N/A+/Nf6LuC9ef7+BnhtYJcXDvGiOw78NvBSPH/fDbwP/zW+C3hvnr+/AV4b2OVfhvjXOQ78NvBSPH/fDbwP/7m+C3hvnr+/AV4b2OVFg/jXOw78NvBSPH/fDbwP/zm+C3hvnr+/AV4b2OVFh/i3OQ78NvBSPH/fDbwP/7G+C3hvnr+/AV4b2OVfB/Fvdxz4beCleP6+G3gf/mN8F/DePH9/A7w2sMu/HuLf5zjw28BL8fx9DvDZ/Pt8NvBZPH9/A7w2sMu/DeLf7zjw28BL8bx2gRP8+1wEjvO8/gZ4bWCXfzvEf4zjwG8DL8VzegbwYP59doFjPKe/AV4b2OXfB/Ef5zjw28BL8WyfA3w2/z6fDXwWz/Y3wGsDu/z7If5jHQfeG3gw8NvAT/Mf462B1wZuBb4b2OU/BuL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHMxbpBZXZjPgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4v+Wlge8CXhr4aeB9gF1eMMT/HQ8G/go4zrP9DPDWvGCI/zteG/gtnpd4wRD/d7w28Fs8L/GCIf7veG3gt3he4gVD/N/x2sBv8bzEC4b4v+O1gd/ieYkXDPF/x2sDv8XzEi8Y4v+O1wZ+i+clXjDE/x2vDfwWz0u8YIj/O14b+C2el3jBEP93vDbwWzwv8YIh/u94beC3eF7iBUP83/HawG/xvMQLhvi/47WB3+J5iRcM8X/HawO/xfMSLxjiP89nAR/NFV8NfA7/uV4b+C2el3jBEP85vgt4b57T2wA/zX+e1wZ+i+clXjDEf7zvAt6b5/U1wEfzn+e1gd/ieYkXDPEf67uA9+b5+xjgq/nP89rAb/G8xAuG+I/zXcB78/z9DfDawC7/eV4b+C2el3jBEP8xvgt4b56/vwFeG9jlP9d7A9/F8xIvGOLf77uA9+b5+xvgtYFd/vN9NfBRPKdLwHFeMMS/z3cB783z9zfAawO7/Nf4beC1eE6/A7w2Lxji3+67gPfm+fsb4LWBXf5rHAeeDhznOX0P8N68YIh/m+8C3pvn72+A1wZ2+a/zU8Bb87zeBvhpXjDEv953Ae/N8/c3wGsDu/zXeWvgp3hel4DjvHCIf53vAt6b5+9vgNcGdvmv81HAV/P8fQ7w2bxwiBfddwHvzfP3N8BrA7v813gt4LOB1+b5ewbwYP5liBfNbwCvy/N3APw1MPGf7zjw0rxwl4DXBv6afxniX/ZbwGvzv8fbAD/NiwbxL0tA/M93CXhr4Ld50SH+ZQmI/9l+B/ho4K/510H8y34LeG3+Z/ob4KOB3+bfBvGi+U3gdXj+DoC/ASb+8/02V9wK/DSwy78P4kX33cB78fz9NfA6wC7/uyD+db4beC+ev78GXgfY5X8PxL/edwPvxfP318DrALv874D4t/lu4L14/v4aeB1gl//5EP923w28F8/fXwOvA+zyPxvi3+e7gffi+ftr4HWAXf7nQvz7fTfwXjx/fw28DrDL/0yI/xjfDbwXz99fA68D7PI/D+I/zncD78Xz99fA6wC7/M+C+I/13cB78fx9DPDV/M+C+I/33cB78by+Bvho/mdB/Of4buC9eE5vA/w0/7Mg/vN8NvDRXPHVwGfzPw/i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjQE4VBFjTkygAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEKklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgt4a+CvgfcB/pr/GxD/sp8G3opn2wVeBriV//0Q/zLzvF4H+G3+90P8y8zzeh3gt/nfD/EvM8/rdYDf5n8/xL/MPK/XAX6b//0Q/zLzvF4H+G3+90P8y8zzeh3gt/nfD/EvM8/rdYDf5n8/xL/MPK/XAX6b//0Q/zLzvF4H+G3+90P8y8zzeh3gt/nfD/EvM8/rdYDf5n8/xL/MPK/XAX6b//0Q/zLzvF4H+G3+90P8y8zzeh3gt/nfD/EvM8/rdYDf5j/XZwEfzRVfDXwO//EQ/zLzvF4H+G3+87w38F08p+8G3of/WIh/mXlerwP8Nv95vht4L57XdwPvw38cxL/MPK/XAX6b/zwfDXwVz993A+/DfwzEv8w8r9cBfpv/PMeB3wZeiufvu4H34d8P8S8zz+t9gO/mP9dx4LeBl+L5+27gffj3QfzLdoFjPKevAT6a/3zHgd8GXorn77uB9+HfDvEv+23gtXhOvwO8Nv81jgO/DbwUz993A+/Dvw3iX/bdwHvxnHaBhwC7/Nc4Dvw28FI8f98NvA//eoh/2VsDP8Xz+mngbfivcxz4beCleP6+G3gf/nUQL5pd4BjP622An+a/znHgt4GX4vn7buB9eNEhXjSfDXwWz99HA1/Df53jwG8DL8Xz993A+/CiQbzobgUexPP328BnA7/Df43jwG8DL8Xz993A+/AvQ7zoXhr4beAYL9xfA7v856vASwNbPH+/CbweLxziX+e9ge/if4/fBl6HFwzxr/fawE8Dx/ifz0DwgiH+bV4a+GrgtfifzUDwgiH+fV4b+Grgpfif6beB1+EFQ/zHOA68NfBgrnht/vNV4KWALZ6/3wJelxcO8b/TceC3gJfm+fse4L35lyH+9zkO/Bbw0jx/3wO8Ny8axP8ux4HfAl6a5+97gPfmRYf43+M48FvAS/P8fQ/w3vzrIP53OA78FvDSPH/fA7w3/3qI//mOA78FvDTP3/cA782/DeJ/tuPAbwEvzfP3PcB782+H+J/rOPBbwEvz/H0P8N78+yD+ZzoO/Bbw0jx/3wO8N/9+iP+ZPhr4Kp6/7wHem/8YiP+Zvht4L57X9wDvzX8cxP9Mbw38FM/pe4D35j8W4n+uzwY+miu+Gvhs/uMh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/BDoBBctcYuQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b473EceC3grYEHAy8NHOc/1k8D7wPs8oIh/msdBz4K+GjgOP/5fgZ4a14wxH+dlwZ+Cngw/7XEC4b4r/HewFcBx/mv9TfAS/OCIf7zvTbwW/zXuwS8NvDXvGCI/1wvDfwWcJzn7xLw18BfA38N3Mp/nL8GdnnhEP+5fgt4bZ6/nwHeG9jlvw/iP89bAz/F87oEvDfw0/z3Q/zn+SvgpXleHwN8Nf8zIP5zHAcu8rx+B3ht/udA/Od4b+C7eF6vA/w2/3Mg/nN8NvBZPC/xPwviP8dXAx/Fc/ob4KX5nwXxn+O3gdfiOf0O8Nr8z4L4z/HbwGvxnH4HeG3+Z0H85/ht4LV4Tr8DvDb/syD+c/w28Fo8p98BXpv/WRD/OX4beC2e0+8Ar83/LIj/HL8NvBbP6XeA1+Z/FsR/jt8GXovn9DvAa/M/C+I/x28Dr8Vz+h3gtfmfBfGf47eB1+I5/Q7w2vzPgvjP8dvAa/Gcfgd4bf5nQfzn+G3gtXhOvwO8Nv+zIP5z/DbwWjyn3wFem/9ZEP85fht4LZ7T7wCvzf8siP8cvw28Fs/pd4DX5n8WxH+O3wZei+f0O8Br8z8L4j/HbwOvxXP6HeC1+Z8F8aJ5a+CtgL8GvgfY5YX7beC1eE6/A7w2L9xx4L2AlwZ+Bvhp/nMh/mWfDXwWz/bXwOsAu7xgvw28Fs/pd4DX5gU7DvwW8NI82+cAn81/HsS/7CJwnOf018DrALs8f78NvBbP6XeA1+b5Ow78FvDSPKdd4AT/eRD/sl3gGM/rr4HXAXZ5Xr8NvBbP6XeA1+Z5HQd+C3hpntcl4Dj/eRD/ss8GPovn76+B1wF2eU6/DbwWz+l3gNfmOR0Hfgt4aZ6/zwE+m/88iBfNdwPvxfP318DrALs8228Dr8Vz+h3gtXm248BvAS/N8/c9wHvznwvxovtu4L14/v4aeB1glyt+G3gtntPvAK/NFceB3wJemufve4D35j8f4l/nu4H34vn7a+B1gF3gt4HX4jn9DvDawHHgt4CX5vn7HuC9+a+B+Nf7buC9eP7+Gngd4KeB1+I5/Q7w1sBvAS/N8/c9wHvzXwfxb/PdwHvx/P01cAi8Gs/pD4BN4KV5/r4HeG/+ayH+7b4beC+evwNgi+d0AGzx/H0P8N7810P8+3w38F78+3wP8N7890D8+3038F7823wP8N7890H8x/hu4L341/ke4L3574X4j/PdwHvxovke4L3574f4j/XdwHvxwn0P8N78z4D4j/fdwHvx/H0P8N78z4H4z/GbwOvwnH4LeF3+Z0H85/kt4LW44neA1+F/HsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EVhlmEFy9R/VAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/NbwGtxxe8Ar8P/PIj/HL8HvDrP6TeB1+N/FsR/vO8C3pvn77uB9+F/DsR/rO8C3psX7ruB9+F/BsR/nO8C3psXzXcD78N/P8R/jO8C3pt/ne8G3of/Xoh/v+8C3pt/m+8G3of/Poh/n+8C3pt/n+8G3of/Hoh/u+8C3pvn7wDY4jkdAFs8f98NvA//9RD/Nt8FvDfP398AB8Cr8Zz+ANgCXorn77uB9+G/FuJf77uA9+b5+xvgtYGfBl6L5/Q7wFsDvw28FM/fdwPvw38dxL/OdwHvzfP3N8BrA7vAbwOvxXP6HeC1gePAbwMvxfP33cD78F8D8aL7LuC9ef7+BnhtYJcrfht4LZ7T7wCvzRXHgd8GXorn77uB9+E/H+JF813Ae/P8/Q3w2sAuz/bbwGvxnH4HeG2e7Tjw28BL8fx9N/A+/OdC/Ms+G/gsnr+/AV4b2OU5/TbwWjyn3wFem+d0HPht4KV4/j4H+Gz+8yD+ZReB4zyvvwFeG9jlef028Fo8p98BXpvndRz4beCleF67wAn+8yD+ZbvAMZ7T3wCvDezy/P028Fo8p98BXpvn7zjw28BL8ZwuAcf5z4P4l3028Fk8298Arw3s8oL9NvBaPKffAV6bF+w48NvAS/FsnwN8Nv95EC+atwZeG7gV+G5glxfut4HX4jn9DvDavHDHgfcGHgz8NvDT/OdC/Of4beC1eE6/A7w2/7Mg/nP8NvBaPKffAV6b/1kQ/zl+G3gtntPvAK/N/yyI/xy/DbwWz+l3gNfmfxbEf47fBl6L5/Q7wGvzPwviP8dvA6/Fc/od4LX5nwXxn+O3gdfiOf0O8Nr8z4L4z/HbwGvxnH4HeG3+Z0H85/ht4LV4Tr8DvDb/syD+c/w28Fo8p98BXpv/WRD/OX4beC2e0+8Ar83/LIj/HL8NvBbP6XeA1+Z/FsR/jt8GXovn9DvAa/M/C+I/x28Dr8Vz+h3gtfmfBfGf47eB1+I5/Q7w2vzPgvjP8dvAa/Gcfgd4bf5nQfzn+Grgo3hOfw28DP+zIP5zfDbwWTwv8T8L4j/HewPfxfN6HeC3+Z8D8Z/jOHCR5/XbwOvwPwfiP89fAy/F8/oY4Kv5nwHxn+e1gd/iee0C7wP8NP/9EP+5fht4LZ6/nwbeB9jlvw/iP9dLA78NHOP52wX+Gvhr4K+BW/mP8wzgVl44xH++1wZ+i/96u8DrAH/NC4b4r/HewFcDx/iv9dfAy/CCIf7rvDTw08CD+K8lXjDEf73PBj4aOMZ/vu8B3psXDPHf562BtwYeDLw0cIz/WN8DfDSwywuG+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CQjSYQZ/KEHsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgt4a/5j7QJ/DdwK/DTwM/zXQ/zLfhp4K/7z7QJfDXwO/3UQ/zLzX+tW4G2Av+Y/H+Jf9tfAS/Ffaxd4H+Cn+c+F+Je9NPDbwDH+670O8Nv850G8aB4MPJj/OA8GXhp4aeClgWM8f7vA6wB/zX8OxH+/48B3A2/F8/fbwOvwnwPxP8dbAz/F8/c6wG/zHw/xP8tHA1/F8/pr4GX4j4f4n+e3gdfieZ0AdvmPhfif57WB3+J5vQ/w3fzHQvzPZJ7X5wCfzX8sxP9Mfw28FM/pc4DP5j8W4n+m3wZei+f0O8Br8x8L8T/TbwOvxXP6HeC1+Y+F+J/pt4HX4jn9DvDa/MdC/M/028Br8Zx+B3ht/mMh/mf6beC1eE6/A7w2/7EQ/zP9NvBaPKffAV6b/1iI/5l+G3gtntPvAK/NfyzE/0y/DbwWz+l3gNfmPxbif6bfBl6L5/Q7wGvzHwvxP9NvA6/Fc/od4LX5j4X4n+m3gdfiOf0O8Nr8x0L8z/TbwGvxnH4HeG3+YyH+Z/pt4LV4Tr8DvDb/sRD/M/028Fo8p98BXpv/WIj/mX4beC2e0+8Ar81/LMT/TL8NvBbP6XeA1+Y/FuK/xlsDbwX8NfA9wC4v3G8Dr8Vz+h3gtXnhjgPvBbw08DPAT/PCIf7zfTbwWTzbXwOvA+zygv028Fo8p98BXpsX7DjwW8BL82yfA3w2LxjiP99F4DjP6a+B1wF2ef5+G3gtntPvAK/N83cc+C3gpXlOu8AJXjDEf75d4BjP66+B1wF2eV6/DbwWz+l3gNfmeR0Hfgt4aZ7XJeA4LxjiP99nA5/F8/fXwOsAuzyn3wZei+f0O8Br85yOA78FvDTP3+cAn80Lhviv8d3Ae/H8/TXwOsAuz/bbwGvxnH4HeG2e7TjwW8BL8/x9D/DevHCI/zrfDbwXz99fA68D7HLFbwOvxXP6HeC1ueI48FvAS/P8fQ/w3vzLEP+1vht4L56/vwZeB9gFfht4LZ7T7wCvDRwHfgt4aZ6/7wHemxcN4r/edwPvxfP318DrAD8NvBbP6XeAtwZ+C3hpnr/vAd6bFx3iv8d3A+/F8/fXwCHwajynPwA2gZfm+fse4L3510H89/lu4L14/g6ALZ7TAbDF8/c9wHvzr4f47/XdwHvx7/M9wHvzb4P47/fdwHvxb/M9wHvzb4f4n+G7gffiX+d7gPfm3wfxP8d3A+/Fi+Z7gPfm3w/xP8t3A+/FC/c9wHvzHwPxP893A+/F8/c9wHvzHwfxP9NvAq/Dc/ot4HX5j4X4n+u3gNfiit8BXof/eIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8Iyz8mEGZalyOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+u3gNfiit8BXof/eIj/mX4PeHWe028Cr8d/LMT/PN8FvDfP33cD78N/HMT/LN8FvDcv3HcD78N/DMT/HN8FvDcvmu8G3od/P8T/DN8FvDf/Ot8NvA//Poj/ft8FvDf/Nt8NvA//doj/Xt8FvDf/Pt8NvA//Noj/Pt8FvDfP3wGwxXM6ALZ4/r4beB/+9RD/Pb4LeG+ev78BDoBX4zn9AbAFvBTP33cD78O/DuK/3ncB783z9zfAawM/DbwWz+l3gLcGfht4KZ6/7wbehxcd4r/WdwHvzfP3N8BrA7vAbwOvxXP6HeC1gePAbwMvxfP33cD78KJB/Nf5LuC9ef7+BnhtYJcrfht4LZ7T7wCvzRXHgd8GXorn77uB9+Ffhviv8V3Ae/P8/Q3w2sAuz/bbwGvxnH4HeG2e7Tjw28BL8fx9N/A+vHCI/3yfDXwWz9/fAK8N7PKcfht4LZ7T7wCvzXM6Dvw28FI8f58DfDYvGOI/30XgOM/rb4DXBnZ5Xr8NvBbP6XeA1+Z5HQd+G3gpntcucIIXDPGfbxc4xnP6G+C1gV2ev98GXovn9DvAa/P8HQd+G3gpntMl4DgvGOI/32cDn8Wz/Q3w2sAuL9hvA6/Fc/od4LV5wY4Dvw28FM/2OcBn84Ih/mu8NfDawK3AdwO7vHC/DbwWz+l3gNfmhTsOvDfwYOC3gZ/mhUP8z/TbwGvxnH4HeG3+YyH+Z/pt4LV4Tr8DvDb/sRD/M/028Fo8p98BXpv/WIj/mX4beC2e0+8Ar81/LMT/TL8NvBbP6XeA1+Y/FuJ/pt8GXovn9DvAa/MfC/E/028Dr8Vz+h3gtfmPhfif6beB1+I5/Q7w2vzHQvzP9NvAa/Gcfgd4bf5jIf5n+m3gtXhOvwO8Nv+xEP8z/TbwWjyn3wFem/9YiP+Zfht4LZ7T7wCvzX8sxP9Mvw28Fs/pd4DX5j8W4n+m3wZei+f0O8Br8x8L8T/TbwOvxXP6HeC1+Y+F+J/pt4HX4jn9DvDa/MdC/M/0V8BL85w+B/hs/mMh/mcyz+tzgM/mPxbif57XBn6L5/U+wHfzHwvxP89vAa/N8zoB7PIfC/E/y0cDX8Xz+hvgpfmPh/if462Bn+L5ex3gt/mPh/jvdxz4LuCtef5+B3ht/nMgXjQPBh7Ef5wHAy8NvDTw0sBxnr9LwGsDf81/DsS/7KWB3wKO81/vdYDf5j8P4l/2V8BL81/rEvDewE/znwvxLzP/tZ4BvDXw1/znQ/zLfhp4K/7zXQK+Gvhs/usg/mXHge8G3or/WJeAvwZuBX4a+Gn+6yH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj/GkmEEoiOkCAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/NZwEdzxVcDn8P/PIj/HN8FvDfP6W2An+Z/FsR/vO8C3pvn9TXAR/M/C+I/1ncB783z9zHAV/M/C+I/zncB783z9zfAawO7/M+C+I/xXcB78/z9DfDawC7/8yD+/b4LeG+ev78BXhvY5X8mxL/PdwHvzfP3N8BrA7v8z4X4t/su4L15/v4GeG1gl//ZEP823wW8N8/f3wCvDezyPx/iX++7gPfm+fsb4LWBXf53QPzrfBfw3jx/fwO8NrDL/x6IF913Ae/N8/c3wGsDu/zvgnjR/Abwujx/B8BfAxP/+f4a2AVuBX4G2OXfB/Ev+y3gtfmf6a+BjwF+m38bxL8sAfE/228DHwP8Nf86iH9ZAuJ/vl3gbYDf5kWH+Jf9FvDa/O/xNsBP86JBvGh+E3gdnr8D4G+Aif98x4GX4oXbBV4H+Gv+ZYgX3XcD78Xz99fA6wC7/Nd4beCzgdfi+bsVeAj/MsS/zncD78Xz99fA6wC7/Nf5aOCreP4+B/hsXjjEv953A+/F8/fXwOsAu/zXeWvgp3heu8AJXjjEv813A+/F8/fXwOsAu/zX+WngrXhebwP8NC8Y4t/uu4H34vn7a+B1gF3+axwHbgWO8Zy+B3hvXjDEv893A+/F8/fXwOsAu/zX+G3gtXhOvwO8Ni8Y4t/vu4H34vn7a+B1gF3+83018FE8p13gBC8Y4j/GdwPvxfP318DrALv853pv4Lt4XuIFQ/zH+W7gvXj+/hp4HWCX/zyvDfwWz0u8YIj/WN8NvBfP38cAX81/ntcGfovnJV4wxH+87wbei+f1NcBH85/ntYHf4nmJFwzxn+O7gffiOb0N8NP853lt4Ld4XuIFQ/zn+Wzgo7niq4HP5j/XawO/xfMSLxji/47XBn6L5yVeMMT/Ha8N/BbPS7xgiP87Xhv4LZ6XeMEQ/3e8NvBbPC/xgiH+73ht4Ld4XuIFQ/zf8drAb/G8xAuG+L/jtYHf4nmJFwzxf8drA7/F8xIvGOL/jtcGfovnJV4wxP8drw38Fs9LvGCI/zteG/gtnpd4wRD/d7w28Fs8L/GCIf7veG3gt3he4gVD/N/x2sBv8bzEC4b4v+PBwF8Dx3i27wHemxcM8X/LSwPfDbwU8D3ARwO7vGCI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPF5oZBYXezggAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+uzgI/miq8GPof/eIj/md4b+C6e03cD78N/LMT/TN8NvBfP67uB9+E/DuJ/po8Gvorn77uB9+E/BuJ/puPAbwMvxfP33cD78O+H+J/rOPDbwEvx/H038D78+yD+ZzsO/DbwUjx/3w28D/92iP/5jgO/DbwUz993A+/Dvw3if4fjwG8DL8Xz993A+/Cvh/jf4zjw28BL8fx9N/A+/Osg/nc5Dvw28FI8f98NvA8vOsT/PseB3wZeiufvu4H34UWD+N/pOPDbwEvx/H038D78yxD/MY4DbwU8GDgOvDT/+Srw0sAWz99vAq/HC4f493lr4LOAl+Z/pt8GXocXDPFv89LAVwGvzf9sBoIXDPGv99rATwHH+Z/PQPCCIf513hv4Lv73+G3gdXjBEC+6lwZ+CzjOC/c3wC7/+SrwUsAWz99vAa/LC4d40RwH/gp4MM/f7wCfDfw2/zWOA78FvDTP3/cA782/DPGi+Wzgs3hel4DPBr6a/zrHgd8CXprn73uA9+ZFg/iXHQeeDhzneb0N8NP81zkO/Bbw0jx/3wO8Ny86xL/srYGf4nn9DPDW/Nc5DvwW8NI8f98DvDf/Ooh/2XcD78VzugQ8GNjlv8Zx4LeAl+b5+x7gvfnXQ/zLfht4LZ7T7wCvzX+N48BvAS/N8/c9wHvzb4P4l10EjvOcPgf4bP7zHQd+C3hpnr/vAd6bfzvEv8w8r/cBvpv/XMeB3wJemufve4D35t8H8S8zz+t1gN/mP89x4LeAl+b5+x7gvfn3Q/zLzPN6HeC3+c/z0cBX8fx9D/De/MdA/MvM83od4Lf5z/PdwHvxvL4HeG/+4yD+ZeZ5vQ7w2/zneWvgp3hO3wO8N/+xEP8y87xeB/ht/nN9NvDRXPHVwGfzHw/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP+ynwbeime7BDwY2OV/P8S/7Djw3cBbAX8DvDfw1/zfgPj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AhVUgUE/ActaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADqklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4Xi8NvBXw2sBLA8f5320X+Gvgt4GfAf6aZ0M8p48Cvpr/u3aBzwa+hisQz/ZTwFvz/8NPA28DIK74aOCr+P/lY4CvFvDSwF/x/88u8DoCPhv4LJ7XzwC/Dfw1/7u9NPDawFvxvD5HwG8Dr8Vz+hngrfm/5beB1+I5/Y6Ai8BxntPHAF/N/y0fDXwVz2lXgHlerwP8Nv+3vDbwWzwnBJjn9TrAb/N/y2sDv8VzQoB5Xq8D/Db/t7w28Fs8JwSY5/U6wG/zf8trA7/Fc0KAeV6vA/w2/7e8NvBbPCcEmOf1OsBv83/LawO/xXNCgHlerwP8Nv+3vDbwWzwnBJjn9TrAb/N/y2sDv8VzQoD5/wsB5v8vBJj/vxBg/v9CgPn/CwHmeX0M8Nf83/LSwFfxnBBgntfrAL/N/y2vDfwWzwkB5nm9DvDb/N/y2sBv8ZwQYJ7X6wC/zf8trw38Fs8JAeZ5vQ7w2/zf8trAb/GcEGCe1+sAv83/La8N/BbPCQHmeb0O8Nv83/LawG/xnBBgntfrAL/N/y2vDfwWzwkB5nm9DvDb/N/y2sBv8ZwQYJ7X6wC/zf8trw38Fs8JAeZ5vQ7w2/zf8trAb/GcEGCe1+sAv83/La8N/BbPCQHmeb0O8Nv83/LawG/xnBBgntfrAL/N/y2vDfwWzwkB5nm9DvDb/N/y2sBv8ZwQYJ7X6wC/zf8trw38Fs8JAeZ5vQ7w2/zf8trAb/GcEGCe1+sAv83/La8N/BbPCQHmeb0O8Nv83/LawG/xnBCwCxzjOX0M8NX83/LRwFfxnC4J+G3gtXhOPw28Df+3/Bbw2jyn3xHw2cBn8bx+G/hp4K/53+2lgbcGXpvn9TkCXhr4beAY/79cAl5bXPHRwFfx/8vHAF8tnu2ngbfi/4efAd4aQDynjwY+GzjG/02XgM8GvporEM/rpYG3Bl4beGngGP+7XQL+Gvht4KeBv+bZEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHDx+K4EGSmM8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3e2ngu4CXBn4aeB9gl/9dEP82x4GnA8d5tp8B3pr/XRD/Nq8N/BbPS/zvgvi3eW3gt3he4n8XxL/NawO/xfMS/7sg/m1eG/gtnpf43wXxb/PawG/xvMT/Loh/m9cGfovnJf53QfzbvDXwUzwv8b8L4t/ms4HP4nmJ/10Q/zafDXwWz0v874L4t/ls4LN4XuJ/F8S/zWcDn8XzEv+7IK44DrwUL9hr85xeG3htntdn8x/jOPDSPKdd4K+Bvwb+BriVfz8EvDTwW8Bx/nf5auBzgF3+7RDwV8BL87/TrcDbAH/Nvw0CzP9ufw28DrDLvx4Cfhp4K/53+xrgo/nXQ8Bx4LuBt+LfZwD+iP98r8Xz9xDgVv51EP82nw18Fs/pd4DX5j/fWwM/xfN6G+Cn+ddB/Nt8NvBZPKffAV6b/xq/DbwWz+lzgM/mXwfxb/PZwGfxnH4HeG3+a/w28Fo8p58B3pp/HcS/zWcDn8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82nw18Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nt8NvBZPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PZwGfxnH4HeG3+a/w28Fo8p98BXpt/HcS/zWcDn8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82nw18Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nt8NvBZPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PZwGfxnH4HeG3+a/w28Fo8p98BXpt/HcS/zWcDn8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82nw18Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nt8NvBZPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PZwGfxnH4HeG3+a/w28Fo8p98BXpt/HcS/zWcDn8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82nw18Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nt8NvBZPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PdwHvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zU8Db8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82nw18Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nu8NfBTPK+3AX6a/3y/DbwWz+l3gNfmXwfxb/Ng4Ok8f7/Nf76XBo7znHaBv+ZFcyvwM8BPi3+7rwY+iv/dPkf82x0H/hp4EP977Yp/n5cGfhp4EP87PUP8+x0HPhv4KP73+RzxH+fBwEsDLw28Nv+5Xgo4znPaBf6GF82twE8DPy3+d/pt4LV4Tr8DvDb/Ooj/nX4beC2e0+8Ar82/DuJ/p98GXovn9DvAa/Ovg/jf6beB1+I5/Q7w2vzrIP53+m3gtXhOvwO8Nv86iP+dfht4LZ7T7wCvzb8O4n+n3wZei+f0O8Br86+D+N/pt4HX4jn9DvDa/Osg/nf6beC1eE6/A7w2/zqI/51+G3gtntPvAK/Nvw7if6fvBt6L5/Q9wHvzr4P43+mtgZ/iOb0N8NP86yD+9/ps4KO54quBz+Zfj38EGHK5ENM/a3YAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGRElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/XawHcBD+Z/jl3gr4HfBn4G+GueDfEf56WB3wKO8z/XLvDZwNdwBeI/xksDvwUc53+HnwbeBkD8+x0Hfgt4af53+Rjgq8W/z3Hgt4CX5n+fXeB1xL/PdwHvzfP3McBf89/vpYHXBt6K5/U54t/uu4D35vn7GuCj+Z/lt4HX4jn9jvi3eW/gu3j+vgd4b/7n+Wjgq3hOu+Jf772B7+L5+xvgpfmf6bWB3+I5If51Xhr4LeA4z+tvgNcGdvmf6bWB3+I5IV50Lw38FnCc53UJeGngVv7nem3gt3hOiBfNceC3gJfmeV0CXhv4a/5ne23gt3hOiH/ZceC3gJfm+Xsb4Kf5n++1gd/iOSH+Zd8FvDfP3/sA382/zUsD78UV3wP8Nf86Lw28F1d8D/DXvHCvDfwWzwnxwn0X8N48f18DfDT/Ni8N/BXP6X2A7+ZF897Ad/GcXgb4a16w1wZ+i+eEeMHeG/gunr/vAd6bf7uvBj6K5/U+wHfzwr038F08r68BPpoX7LWB3+I5IZ6/9wa+i+fvb4CX5t/nq4GP4vl7H+C7ef7eG/gunr+vAT6aF+y1gd/iOSGe10sDvwUc53n9DfDawC7/Pi8N/BUv2PsA381zem/gu3jBXgb4a16w1wZ+i+eEeE4vDfwWcJzndQl4aeBW/mO8N/BdvGDvA3w3V7w38F28YO8DfDcv3GsDv8VzQjzbceC3gJfmeV0CXhv4a/5jvTfwXbxg78MV38UL9j7Ad/Mve23gt3hOiCuOA78FvDTP39sAP81/jvcGvot/m/cBvpsXzWsDv8VzQlzxXcB78/y9D/Dd/Od6b+C7+Nd5H+C7edG9NvBbPCcEfDbwWTx/XwN8NP813hv4Ll407wN8N/86rw38Fs8JAReB4zyv7wHem/9a7w18Fy/c+wDfzb/eawO/xXNCwC5wjOf13cD78F/rvYHv4oV7H+C7+dd7beC3eE4I+Gzgs3j+vgb4aP5rvDfwXbxo3gf4bv51Xhv4LZ4T4orvBt6L5+99gO/mP9d7A9/Fv877AN/Ni+61gd/iOSGuOA78NvBSPH9vA/w0/zneG/gu/m3eB/huXjSvDfwWzwnxbMeB3wZeiue1C7wO8Nf8x3pv4Lt4wd6HK76LF+x9gO/mX/bawG/xnBDP6aWB3waO8bx2gZcBbuU/xnsD38UL9j7Ad3PFewPfxQv2PsB388K9NvBbPCfE83pp4LeBYzyvvwZeB9jl3+elgb/iBXsf4Lt5Tu8NfBcv2MsAf80L9trAb/GcEM/fewPfxfP318DL8O/z1cBH8fy9D/DdPH/vDXwXz9/XAB/NC/bawG/xnBAv2HsD38Xz993A+/Bv99XAR/G83gf4bl649wa+i+f1NcBH84K9NvBbPCfEC/fdwHvx/H0N8NH827w08Fc8p/cBvpsXzXsD38Vzehngr3nBXhv4LZ4T4l/23cB78fy9D/Dd/Nu8NPDeXPHdwF/zr/PSwHtzxXcDf80L99rAb/GcEP+y48BvAy/F8/c2wE/zP99rA7/Fc0K8aI4Dvw28FM9rF3gd4K/5n+21gd/iOSFedC8N/DZwjOe1C7wMcCv/c7028Fs8J8S/zksDvw0c43n9NfA6wC7/M7028Fs8J8S/3nsD38Xz99fAy/A/02sDv8VzQvzbvDfwXTx/3w28D//zfDTwVTynS+Lf7ruB9+L5+xrgo/mf5beA1+Y5/Y749/lu4L14/j4a+Gv++7008NbAa/O8Pkf8+xwHfht4Kf73uQS8tvj3Ow78NvBS/O/yMcBXi/8YLw38NnCM/x1+BnhrAPEf56WB3waO8T/XJeCzga/mCsR/rNcGvht4EP9zXAL+Gvht4KeBv+bZEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHMbzk4Gu2r3AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/fSwFsBrw28NFf8NfDbwM8Af81/HcR/rY8CvpoXbBf4bOBr+K+B+K/zU8Bb86L5aeBt+M+H+K/x0cBX8a/zMcBX858L8Z/vpYG/4l9vF3gd4K/5z4P4z/fZwGfxvH4G+G2ueG3grXhenwN8Nv95EP/5fht4LZ7TzwBvzXP6beC1eE6/A7w2/3kQ//kuAsd5Th8DfDXP6aOBr+I57QIn+M+D+M+3CxzjOX0M8NU8p48GvorndAk4zn8exH++3wZei+f008Db8Jx+C3htntPvAK/Nfx7Ef77PBj6L5/XbwE9zxVsDr83z+hzgs/nPg/jP99LAbwPH+Ne5BLw28Nf850H81/ho4Kv41/kY4Kv5z4X4r/PTwFvxovkZ4K35z4f4r/XRwGcDx3j+LgGfDXw1/zUQ//VeGnhr4LWBl+aKvwZ+G/hp4K/5r4P4/w3x/xvi/zfE/2+I/98Q/zrHgfcC3pr/mX4a+B5glxcN4l/nr4CX5n+2vwZehhcN4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+G+Nf5a+Cl+J/tb4CX5kWD+Nc5Drw38NrAcf5n2QV+G/huYJcXDeL/N8T/b4j/3xD/vyH+f0P813tp4K2A1wZemiv+Gvht4GeAv+a/DuK/1kcBX80Ltgt8NvA1/NdA/Nf5KeCtedH8NPA2/OdD/Nf4aOCr+Nf5GOCr+c+F+M/30sBf8a+3C7wO8Nf850H85/ts4LN4Xj8D/DZXvDbwVjyvzwE+m/88iP98vw28Fs/pZ4C35jn9NvBaPKffAV6b/zyI/3wXgeM8p48Bvprn9NHAV/GcdoET/OdB/OfbBY7xnD4G+Gqe00cDX8VzugQc5z8P4j/fbwOvxXP6aeBteE6/Bbw2z+l3gNfmPw/iP99nA5/F8/pt4Ke54q2B1+Z5fQ7w2fznQfzne2ngt4Fj/OtcAl4b+Gv+8yD+a3w08FX863wM8NX850L81/lp4K140fwM8Nb850P81/po4LOBYzx/l4DPBr6a/xqI/3ovDbw18NrAS3PFXwO/Dfw08Nf810H8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEDBndBNRKb9QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFuUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fV4aeC+u+B7gr/mvh/jv8dLAX/GcXgb4a/5rIf57fDXwUTynrwE+mv9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qIf7/jwHcBbw38NfAxwG/zwv028Fo8p98BXpsX7rWBrwJeGvhp4H2AXf7tEP9+Pw28Fc/pfYDv5gX7beC1eE6/A7w2L9h7A9/Fc/oZ4K35t0P8+5nn732A7+b5+23gtXhOvwO8Ns/fewPfxfMn/u0Q/35/DbwUz9/7AN/N8/pt4LV4Tr8DvDbP672B7+L5+xvgpfm3Q/z7vTbwW7xg7wN8N8/pt4HX4jn9DvDaPKf3Br6LF+x1gN/m3w7xH+O9ge/iBXsf4Lt5tt8GXovn9DvAa/Ns7w18Fy/Y+wDfzb8P4j/OewPfxQv2PsB3c8VvA6/Fc/od4LW54r2B7+IFex/gu/n3Q/zHem/gu3jB3gf4buC3gdfiOf0O8NrAewPfxQv2PsB38x8D8R/vvYHv4gV7H+C9gdfiOf0O8N3Ad/GCvQ/w3fzHQfzneG/gu3jBngA8muf0BODRvGDvA3w3/7EQ/3neG/gu/mO8D/Dd/MdD/Od6b+C7+Pd5H+C7+c+B+M/33sB38W/zPsB3858H8YK9NPBeXPE9wF/zb/fewHfxr/M+wHfzb/fSwHtxxfcAf83zQjx/Lw38Fc/pZYC/5t/uvYHv4kXzPsB382/30sBf8ZxeBvhrnhPi+ftq4KN4Tl8DfDT/Pu8NfBcv3PsA382/z1cDH8Vz+hrgo3lOiOfvt4HX4jn9DvDa/Pu9N/BdPH/vA3w3/36/DbwWz+l3gNfmOSGev98GXovn9DvAa/Mf4zOBz+E5fRbwufzH+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/zjvBHwSV3wJ8CP8x/lt4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOPwO8Nc8J8fx9NvBZPKffAV6b/x1+G3gtntPnAJ/Nc0I8f28N/BTP622An+Z/trcGforn9TbAT/OcEM/fg4Gn8/z9Nv+zvTbP30OAW3lOiBfsq4GP4v+GrwE+mueFeMGOA38NPIj/3f4GeG1gl+eFeOFeGvhp4EH87/Q3wHsDf83zh/iXHQc+G/go/nf5GuCzgV1eMMSL7sHASwMvDbw2//PsAn8N/DXw18Ct/MsQ/78h/n9D/P+G+P8N8f8b/wg5dsVBCAnkiwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFwElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/puPAS/Gi+x3+bRD/MxwH3gp4beCtgeP86/w18DrALv86iP9+HwV8NnCcf5+PAb6afx3Ef5+XBn4KeDD/MX4GeGv+dRD/Pd4b+CrgOP9xfgd4bf51EP/13hv4Lv7j/Q7w2vzrIP5rvTXwU7xgl4CfBn4a+GvgVp6/3wZei+f0O8Br86+D+K/z0sBvAcd5/j4H+Gpgl3/ZbwOvxXP6HeC1+ddB/Nf5LeC1eV6XgLcGfpsX3W8Dr8Vz+h3gtfnXQfzXeG3gt3hel4DXBv6af53fBl6L5/Q7wGvzr4P4r/FbwGvzvN4G+Gn+9X4beC2e0+8Ar82/DuI/34OBp/O8fgZ4a56/48BbAS8NvDTP66WB4zyn3wFem38dxH++jwa+iuf1EOBWntdbA98FHOdf53eA1+ZfB/Gf76eBt+I5/Q3w0jyvlwb+in+b3wFem38dxH++3wZei+f0NcBH87z+Cnhp/m2+Bvho/nUQ//kuAsd5Tp8DfDbP6cHA0/m3exngr/nXQfznM8/rbYCf5jm9NvBbPK/f4YX7a+C7gb/mXw/xn888r9cBfpvn9NrAb/G8xH8exH8+87xeB/htntNrA7/F8xL/eRD/+czzeh3gt3lOrw38Fs9L/OdB/Oczz+t1gN/mOb028Fs8L/GfB/Gfzzyv1wF+m+f02sBv8bzEfx7Ev99x4LuAt+ZF9zrAb/OcXhv4Lf71doGvBj6Hfz3Ev99PA2/Fv87rAL/Nc3pt4Lf4t3sf4Lv510H8+5l/vdcBfpvn9NrAb/Fv9zXAR/Ovg/j3M/96LwP8Nc/ppYG/4t/uc4DP5l8H8e9n/nU+B/hsnr/PBj6Lf5u3AX6afx3Ev595Xh8D/DXPaxf4a164lwaO84K9N/BePK8TwC7/Ooh/P/O8Xgf4bf5z/DTwVjynvwFemn89xL+feV6vA/w2/zkuAsd5Tl8DfDT/eoh/P/O8Xgf4bf7jvTTwVzyvtwF+mn89xL+feV6vA/w2//E+GvgqntcJYJd/PcS/n3leHw38Nc/rEvDXvHAvDRzj+fts4LV5Tn8DvDT/Noh/P/Ov8znAZ/P8fTbwWfzrfA3w0fzbIP79zL/eywB/zXN6aeCv+Nd7G+Cn+bdB/PuZf73XAX6b5/TawG/xr3cC2OXfBvHv99PAW/Gv8zrAb/OcXhv4Lf51fgZ4a/7tEP9+x4HvBt6KF93rAL/Nc3pt4Ld40X0P8NHALv92iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5/hp4Kf5t/gZ4af7zIP7zvTTw28Ax/nUuAa8N/DX/eRD/NR4MPJh/nb8GdvnPhfj/DfH/G/8ID1nPQYdbK8sAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP6TjwXsBb83/TbwNfA+xyBeI5/RXw0vzf9tfAy3AF4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOI5/TXwEvxf9vfAC/NFYjn9GDgr4FjPH9/A+zyP9tx4KV4/i4BLw3cyhWI5/XSwF/x/P018DL8z/ZXwEvz/L0M8Nc8G+L5e2/gu3j+vht4H/5n+i7gvXn+3gf4bp4T4gX7bOCzeP4+Bvhq/mf5bOCzeP4+B/hsnhfihftu4L14/t4H+G7+Z3hv4Lt4/r4HeG+eP8QLdxz4beCleF67wOsAf81/r5cGfgs4zvP6G+C1gV2eP8S/7Djw18CDeF67wEOAXf57HAeeDhzneT0DeGlglxcM8aJ5aeC3gWM8r78GXgfY5b/WceC3gJfmeV0CXhv4a144xIvurYGf4vn7aeBt+K/1U8Bb8/y9DfDT/MsQ/zrvDXwXz9/XAB/Nf42vBj6K5+99gO/mRYP41/tu4L14/t4H+G7+c7038F08f98DvDcvOsS/zW8Dr8Xz9zrAb/Of47WB3+L5+x3gtfnXQfzbHAd+G3gpntcu8DrAX/Mf66WB3wKO87z+BnhtYJd/HcS/3YOBvwaO8bz+GngdYJf/GMeB3wJemud1CXhp4Fb+9RD/Pi8N/BXP318DL8N/jL8CXprn72WAv+bfBvHv997Ad/H8fTfwPvz7fBfw3jx/7wN8N/92iP8Ynw18Fs/fxwBfzb/NRwNfxfP3OcBn8++D+I/z3cB78fy9D/Dd/Ou8N/BdPH/fA7w3/36I/zjHgd8GXorntQu8DvDXvGheGvgt4DjP62+A1wZ2+fdD/Mc6Dvw18CCe1y7wEGCXF+448HTgOM/rGcBLA7v8x0D8x3tp4LeBYzyvvwZeB9jl+TsO/Bbw0jyvS8BrA3/NfxzEf463Bn6K5++ngbfh+fsp4K15/t4G+Gn+YyH+83w08FU8fy8D/DXP6aWBv+L5ex/gu/mPh/jP9d3Ae/G8Xgf4bZ7TawO/xfP6HuC9+c+B+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmed1K/DZwPfwbK8N/BbPS/znQfznMy/YrcBnA98DvDbwWzwv8Z8H8Z/vVuBBvHC3Aj8NfDTPS/znQfzne23gp4Fj/NuI/zyI/xoPBj4beC/+9cR/HsR/rQcDnw28Fy868Z8H8d/jwcBnA+/Fv0z850H893ow8NnAe/H8XQKO858H8T/Dg4HPBt6L5/Q5wGfznwfxP8uDgfcGXhr4aeC7+c+F+P8N8f8b4v83xP9viP/f+Ed4Ib5BY4KzKwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xksDx3j+fof/uRD/eseBtwJeG3hp4KX5l/018NfATwO/A+zyPwPiRffSwEcB782/308DPw38DLDLfx/Ei+ajgM8GjvMf77uBnwZ+hn+/48B7AS8N/Azw07xwiH/ZZwOfxX++W4GfBr4H+Gv+9d4L+GzgwTzb5wCfzQuG+JddBI7zX2sX+Gngr4G/Bv4G2OWKBwMP4orXBl4beG2ev13gBC8Y4l9mXrDfAX4b+G2uuBW4lWd7beClgZcG3ho4xn898YIh/mXmeX0O8Nn867008N7AWwMP4r+GeMEQ/zLzvF4H+G3+fd4aeGvgvfjPJV4wxL/MPK/XAX6b/xjHgbcG3hp4K/7jiRcM8S8zz+t1gN/mP95x4LWBtwZeGngp/mV/A/w1sAt8FM9LvGCIf5l5Xq8D/Db/NV6b528X+Gue7bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5tleGjjG8/c7/Nd4beC3eF7iBUP8y8zz+mrgOPDSwEvzL/tr4K+BnwZ+B9jlP95rA7/F8xIvGOJfZv7j/TTw08DPALv8x3ht4Ld4XuIFQ/zLzH+u7wZ+GvgZ/n1eG/gtnpd4wRD/MvNf41bgp4HvAf6af73PAj6b5yVeMMS/zPzX2wV+Gvhr4K+BvwF2ueLBwIO44rWB1wZemxdMvGCIf5l5wX4H+G3gt7niVuBWnu21gZcGXhp4a+AY/7UuAcd5wRD/MvO8vgT4ZP71Xhp4b+CtgQfxn+9zgM/mBUP8y8zzeh3gt/n3eWvgrYH34j/eJeCzga/mhUP8y8zzeh3gt/mPcRx4a+Ctgbfi3+97gK8G/pp/GeJfZp7X6wC/zX+848BrA28NvDTwUvzL/gb4a+C3gZ8GdnnRIf5l5nm9DvDb/Nd4bZ6/XeCv+fdB/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+m//9EP8y87xeB/ht/vdD/MvM83od4Lf53w/xL/tr4KV4Ti8D/DX/+yH+ZS8N/DZwjCs+B/hs/m9AvGgeDDwY2AX+mv87EP+/If5/4x8BrvzTQSlc1uYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv3HHgpYCXBl4aeDD/en8NfA/w1/zPg3jB3hr4LuA4/zFeBvhr/mdBPK/jwHcBb81/rL8GXob/WRDP66uBj+I/x+cAn83/HIjn9NrAb/Gf62WAv+Z/BsRz+m3gtXhezwBu5V/nOPBSPK+/Bl6G/xkQz8k8r+8B3pt/m78GXorn9TnAZ/PfD/FsLw38Fc/rIcCt/Nu8NPBXPH8vA/w1/70Qz/bawG/xvMS/z2cDn8XzuhW4lf8ctwJ/Dfw18DfALs8f4tleG/gtnpf49/tr4KX477ELvA/w0zwvxLO9NvBbPC/x7/fSwF/x3+ungbfhOSGe7bWB3+J5if8Ynw18Fv+9vgb4aJ4N8WyvDfwWz0v8x/lr4KX47/U6wG9zBeLZXhv4LZ6X+I9zHHhv4K35z/dg4EE8r98BXpsrEM/22sBv8bzE/17fDbwXz0tcgXi21wZ+i+cl/vd6MPB0ntfLAH8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv92r8V/jN/h3848r9cBfhtAPNtrA7/F8xL/eseB3wJemv8Yfw28DrDLv555Xq8D/DaAeLbXBn6L5yX+9T4a+Cr+Y30M8NX865nn9TrAbwOIZ3tt4Ld4XuJf77uB9+I/1tcAH82/nnlerwP8NoB4ttcGfovnJf713hr4Kf5jvQ3w0/zrmef1OsBvA4hne23gt3he4t/ms4GPBo7x73MJ+Grgs/m3Mc/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7aeCveF4PAW7lf6cHA0/neb0M8NcA4jmZ5/XdwPvwv9N3Ae/N8xJXIJ7TbwOvxfO6FbiV/10eDDyY5/U7wGtzBeI5vTbwW/zf9jrAb3MF4nl9NfBR/N/0NcBH82yI53Uc+G7grfi/5WeAt+Y5IV6wtwa+GzjG/26XgPcGfprnhXjhjgMvDbw08NLAg/nf4Vbgr4G/Bv4a2OX5Q/z/hvj/DfH/G+L/N8T/b/wjkni0QRsfr2oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv3HHgpYCXBl4aeDD/en8NfA/w1/zPg3jB3hr4LuA4/zFeBvhr/mdBPK/jwHcBb81/rL8GXob/WRDP66uBj+I/x+cAn83/HIjn9NrAb/Gf62WAv+Z/BsRz+m3gtXhezwBu5V/nOPBSPK+/Bl6G/xkQz8k8r+8B3pt/m78GXorn9TnAZ/PfD/FsLw38Fc/rIcCt/Nu8NPBXPH8vA/w1/70Qz/bawG/xvMS/z2cDn8XzuhW4lf8ctwJ/Dfw18DfALs8f4tleG/gtnpf49/tr4KX477ELvA/w0zwvxLO9NvBbPC/x7/fSwF/x3+ungbfhOSGe7bWB3+J5if8Ynw18Fv+9vgb4aJ4N8WyvDfwWz0v8x/lr4KX47/U6wG9zBeLZXhv4LZ6X+I9zHHhv4K35z/dg4EE8r98BXpsrEM/22sBv8bzE/17fDbwXz0tcgXi21wZ+i+cl/vd6MPB0ntfLAH8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne2ngr3heDwFu5X+nBwNP53m9DPDXAOI5mef13cD78L/TdwHvzfMSVyCe028Dr8XzuhW4lf9dHgw8mOf1O8BrcwXiOb028Fv83/Y6wG9zBeJ5fTXwUfzf9DXAR/NsiOd1HPhu4K34v+VngLfmOSFesLcGvhs4xv9ul4D3Bn6a54V44Y4DLw28NPDSwIP53+FW4K+Bvwb+Gtjl+UP8/4b4/w3x/xvi/zfE/2/8Iy6cp0FfPz88AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4r/HceCleE5/A+zyXwvxX+fBwEcBbw08mOfvVuCnga8BbuU/H+I/34OBzwLem3+d7wY+B7iV/zyI/1wvDfwWcJx/m13gdYC/5j8H4j/PewPfxX+M9wG+m/94iP8cLw38Ff+xXgb4a/5jIf7jPRj4K+A4z9/fAD8N/Dbw11zx0sBrA28NvBTP3y7wMsCt/MdB/Mf7buC9eP6+BvhoXrDjwGcDH8Xz9z3Ae/MfB/Ef68HA03n+3gb4aV40bw38FM/fQ4Bb+Y+B+I/11cBH8by+Bvho/nW+GvgontfXAB/NfwzEf6ynAw/mOf0N8NL86x0Hfht4KZ7TXwMvw38MxBWvDbwX8GD+7Srw6jyvzwE+m3+bzwY+i+f1C8Am/3a3Aj8D/LSAtwZ+iv88rwP8Nv82rw38Fv953kfAbwOvxX+eE8Au/zbHgYv85/kdAb8NvBb/eU4Au/zbHAcu8p/ndwS8NfBT/Od5HeC3+bd5beC3+M/zNuKK1wbeG3gw/3aHwJvyvD4H+Gz+bT4b+Cye1y8Cm/zb3Qr8NPDT4j/WrcCDeE5/DbwOsMu/znHgt4CX5jn9DfDS/MdA/Mf6auCjeF5fA3w0/zpfDXwUz+trgI/mPwbiP9aDgafz/L0N8NO8aN4a+Cmev4cAt/IfA/Ef77uB9+L5+2rgc4Bdnr/jwGcBH83z9z3Ae/MfB/Ef78HAXwPHeP7+Gvhp4LeBv+GKlwJeG3hr4KV5/i4BLw3cyn8cxH+Olwb+iv9YLwP8Nf+xEP953hv4Lv5jvA/w3fzHQ/znemngt4Fj/NtcAl4b+Gv+cyD+8z0Y+GzgvfjX+R7gs4Fb+c+D+K/zYOCjgbcGHsTz9zfAbwNfDdzKfz7Ef4/jwEvznP4a2OW/FuL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CM4P2txqEMiAgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b493kw8F7AawMvDRznv8Yu8NfAbwPfA9zKvw3i3+69ge/if4b3Ab6bfz3Ev81XAx/F/yxfA3w0/zqIf73XBn6L/5leB/htXnSIf72nAw/mf6ZbgYfwokP867w38F08r0vAdwO/Dezyn+s48NrAewPHeF7vA3w3LxrEv87TgQfzvF4G+Gv+a7008Fc8r1uBh/CiQbzo3hv4Lp7X9wDvzX+P7wbei+f1PsB38y9DvOieDjyY5/UQ4Fb+ezwYeDrP61bgIfzLEC+a9wa+i+f1PcB789/ru4H34nm9D/DdvHCIF83TgQfzvB4C3Mp/rwcDT+d53Qo8hBcO8S97b+C7eF7fA7w3/zN8N/BePK/3Ab6bFwzxL3s68GCe10OAW/mf4cHA03letwIP4QVDvHDvDXwXz+t7gPfmf5bvBt6L5/U+wHfz/CFeuKcDD+Z5PQS4lf9ZHgw8ned1K/AQnj/EC/bewHfxvL4HeG/+Z/pu4L14Xu8DfDfPC/GCPR14MM/rIcCt/M/0YODpPK9bgYfwvBDP33sD38Xz+hrgo/mf7buB9+J5vQ/w3TwnxPP3dODBPKdLwIOBXf5nezDwdJ7XrcBDeE6I5/XewHfxvD4H+Gz+d/hu4L14Xu8DfDfPhnheTwcezHO6BDwY2OV/hwcDT+d53Qo8hGdDPKcPBb6B53UrcCv/uzwauI7n9WHAN3IF4jmtgZ7/2wZgxhWIZ/to4Kv4/+FjgK8GEM/20cBX8f/DxwBfDSCe7bWB3+L/h9cBfhtAPNtrA7/F8/od/nd7LZ7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xLP32cBH80VXw18Ds/fceC7gLcG/hr4GOC3+a9jntfrAL8NIJ7ttYHf4nmJ5/XewHfxnN4H+G6e108Db8Wz7QIvA9zKfw3zvF4H+G0A8WyvDfwWz0s8r+8G3ovn9DPAW/O8zPN6HeC3+a9hntfrAL8NIJ7ttYHf4nmJ5/XRwFfxnD4G+Gqe118DL8Vzehngr/mvYZ7X6wC/DSCe7bWB3+J5ied1HPht4KW44m+A1wZ2eV4vDfw2cIwrPgf4bP7rmOf1OsBvA4hne23gt3he4gV7ba74bV64BwMPBnaBv+a/lnlerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/rOHCR5/U6wG8DiOdkntf7AN/N/07vDXwXz0tcgXhOtwIP4nl9NfDXwK387/Bg4KWBj+Z5PQN4MFcgntNHA1/F/20fA3w1VyCe128Dr8X/Tb8DvDbPhnheDwb+GjjG/y2XgJcGbuXZEM/fceC7gbfi/4afAd4b2OU5IV64twZeG3hp4LX43+V3gL8Gfhv4aZ4/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RQq26QbcaLGoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Nd4LeGvgt4HvAXb513kv4K2B3wa+B9jlPwbiP99nA5/Fs/018DK86D4b+Cye7a+Bl+E/BuI/30XgOM/pdYDf5kVzETjOc3od4Lf590P85zPP63WA3+ZFY57X6wC/zb8f4j/XawO/xfN6HeC3+Ze9NvBbPK/XAX6bfz/Ef67XBn6L5yVeNK8N/BbPS/zHQPzn+mjgq3he4kXz0cBX8bzEfwzEf67PBj6L5/Q3wEvzovls4LN4Tn8DvDT/MRD/ub4beC+e0+8Ar82L5ruB9+I5/Q7w2vzHQPzn+m3gtXhO3wO8Ny+a3wZei+f0PcB78x8D8Z/rr4CX5jl9DvDZvGj+CnhpntPnAJ/NfwzEfy7zvD4G+GpeNOZ5fQzw1fzHQPznMs/rdYDf5kVjntfrAL/NfwzEf57XBn6L5/U6wG/zL3tt4Ld4Xq8D/Db/MRD/eV4b+C2el3jRvDbwWzwv8R8H8Z/no4Gv4nmJF81HA1/F8xL/cRAvmrcG3gp4MC+6BwMP5jkdAH/Oi+bBwIN5TgfAn/MvuxX4GeCneeEQ/7LPBj6L/50+B/hsXjDEv+wicJz/nXaBE7xgiH/ZLnCM/50uAcd5wRD/ss8GPov/nT4H+GxeMMSL5q2BtwYezIvutXhefw1c4kXzWjyvvwYu8S+7Ffhp4Kd54RD/OV4b+C2e1+sAv82/7LWB3+J5vQ7w2/zHQfzneG3gt3he4kXz2sBv8bzEfyzEf46PBr6K5yVeNB8NfBXPS/zHQvzn+Gzgs3hOfwO8NC+azwY+i+f0N8BL8x8L8Z/ju4H34jn9DvDavGi+G3gvntPvAK/NfyzEf47fBl6L5/Q9wHvzovlt4LV4Tt8DvDf/sRD/Of4KeGme0+cAn82L5q+Al+Y5fQ7w2fzHQvznMM/rY4Cv5kVjntfHAF/NfyzEfw7zvF4H+G1eNOZ5vQ7w2/zHQvzHe23gt3herwP8Nv+y1wZ+i+f1OsBv8x8L8aJ5a+CtgAfzLzsOvDTP67d50RwHXprn9du86G4Ffgb4aV44xL/ss4HP4n+nzwE+mxcM8S+7CBznf6dd4AQvGOJftgsc43+nS8BxXjDEv+yzgc/if6fPAT6bFwzxonlr4K2BB/Mvezlgi+d0K/AMXjQvB2zxnG4FnsGL7lbgp4Gf5oVD/Mczz+tjgK/mRWOe18cAX81/PMR/PPO8Xgf4bV405nm9DvDb/MdD/Md6beC3eF6vA/w2/7LXBn6L5/U6wG/zHw/xH+u1gd/ieYkXzWsDv8XzEv85EP+xPhr4Kp6XeNF8NPBVPC/xnwPxH+uzgc/iOf0N8NK8aD4b+Cye098AL81/DsR/rO8G3ovn9DvAa/Oi+W7gvXhOvwO8Nv85EP+xfht4LZ7T9wDvzYvmt4HX4jl9D/De/OdA/Mf6K+CleU6fA3w2L5q/Al6a5/Q5wGfznwPxH8s8r48BvpoXjXleHwN8Nf85EP+xzPN6HeC3edGY5/U6wG/znwPxH+e1gd/ieb0O8Nv8y14b+C2e1+sAv81/DsR/nNcGfovnJV40rw38Fs9L/OdB/Md5beC3eE6XgOO8aF4b+C2e0yXgOP95EP+x/hp4KZ7tc4DP5kX318BL8WyfA3w2/3kQ/7GOA+8NvDTw28B3869zHHhv4KWB3wa+m/9ciP/fEP+/If5/Q/z/hvj/jX8ErajCQYDP5dsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dg8GXgt4MFe8NHArsMsVvw38Dv8xXgt4ba44DjwY+GuuuBX4HeBW/vUQ/zrHgY8C3hp4af5lu8BvAz8NfA//Ou8FvDXw1rxo/hr4aeBrgF1eNIgXzXHgo4CPBo7zb3Mr8D7Ab/PCvTbwXcCD+bfZBb4a+BpglxcO8S97aeC3gOP8x/hp4H2AXZ7TceC7gLfmP8Yu8DrAX/OCIV649wa+i/94fw28DXArV7w08F3AS/Mf732A7+b5Q7xgnw18Fv+yZwC38pxeGjjGC7cLvA5X/BZwnBfuEvDXPKcHAw/iX/Y5wGfzvBDP33sD38UL9gzgq4GfBm7l+Xtr4K2B9+IF2+WK47xg3wP8NPDTPH8PBt4a+GjgQbxg7wN8N88J8bxeGvgrnr9LwFcDn82L7sHAVwNvxb/OzwAfDdzKi+6zgc/iBXsZ4K95NsTzejrwYJ7XJeC1gb/m3+argY/iRfM5wGfzb/PSwG8Dx3hetwIP4dkQz+mzgc/ieV0CXhq4lX+f9wa+ixfufYDv5t/npYHfBo7xvD4H+GyuQDzbceDpwHGe18sAf81/jPcGvovn732A7+Y/xksDf8Xz2gUeAuwCiGf7aOCreF6fA3w2/7HeG/guntP7AN/Nf6zPBj6L5/U+wHcDiGf7aeCteE6XgAcDu/zHe23go7niq4Hf5j/eceBW4BjP6WeAtwYQz2ae19cAH83/bl8NfBTPSwDiitcGfovn9TrAb/O/22sDv8Xzeh3gt8UVbw38FM9L/N9gntfrAL8trvhs4LN4TpeA4/zfsAsc4zl9DvDZ4orPBj6L5/Q7wGvzf8NvA6/Fc/oc4LPFFZ8NfBbP6VbgIfzf8HTgwTynzwE+W1zx3sB38bzE/w3meb0N8NPiitcGfovn9RDgVv53ezDwdJ7X6wC/La54MPB0ntfHAF/N/24fDXwVz+shwK3i2f4aeCme018DL8P/bn8FvDTP6W+AlwYQz/bVwEfxvN4H+G7+d3pv4Lt4Xp8DfDaAeLYHA0/ned0KvAywy/8ux4G/Ah7M83oIcCuAeE7fDbwXz+u3gdfhf5e/Al6a5/U9wHtzBeI5PRh4Os/fdwPvw/8O3wW8N8/fQ4BbuQLxvD4a+Cqev+8G3of/2b4LeG+ev48BvppnQzx/3w28F8/fdwPvw/9M3wW8N8/f9wDvzXNCPH/Hgd8GXorn77uB9+F/lu8C3pvn72+Al+Z5IV6w48BvAy/F8/fdwPvwP8N3Ae/N8/c3wGsDuzwvxAt3HPht4KV4/r4beB/+e30X8N48f38DvDawy/OH+JcdB34beCmev+8G3of/Ht8FvDfP398Arw3s8oIhXjTHgd8GXorn77uB9+G/1ncB783z9zfAawO7vHCIF91x4LeBl+L5+27gffiv8V3Ae/P8/Q3w2sAu/zLEv85x4LeBl+L5+27gffjP9V3Ae/P8/Q3w2sAuLxrEv95x4LeBl+L5+27gffjP8V3Ae/P8/Q3w2sAuLzrEv81x4LeBl+L5+27gffiP9V3Ae/P8/Q3w2sAu/zqIf7vjwG8DL8Xz993A+/Af47uA9+b5+xvgtYFd/vUQ/z7Hgd8GXorn77uB9+Hf57uA9+b5+xvgtYFd/m0Q/37Hgd8GXorn77uB9+Hf5ruA9+b5+xvgtYFd/u0Q/zGOA78NvBTP33cD78O/zncB783z9zfAawO7/Psg/uMcB34beCmev+8G3ocXzXcB783z9zfAawO7/Psh/mMdB34beCmev+8G3ocX7ruA9+b5+xvgtYFd/mMg/uMdB34beCmev+8G3ofn77uA9+b5+xvgtYFd/uMg/nMcB34beCmev+8G3ofn9F3Ae/P8/Q3w2sAu/7EQ/3mOA78NvBTP33cD78MV3wW8N8/f3wCvDezyHw/xn+s48NvAS/H8fTdXvDfP398Arw3s8p8D8Z/vOPDbwEvxr/M3wGsDu/znQfzXOA78NvBSvGj+BnhtYJf/XIj/OseB3wZeihfub4DXBnb5z4f4r3Uc+G3gpXj+/gZ4bWCX/xqI/3rHgd8GXorn9DfAawO7/NdB/Pc4Dvw08Fpc8TvAWwO7/NdC/Pd6aa74a/57IP5/Q/z/hvj/DfH/G/8IJz0MUDmw3Q8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b493kw8F7AawMvDRznv8Yu8NfAbwPfA9zKvw3i3+69ge/if4b3Ab6bfz3Ev81XAx/F/yxfA3w0/zqIf73XBn6L/5leB/htXnSIf72nAw/mf6ZbgYfwokP867w38F08r0vAdwO/Dezyn+s48NrAewPHeF7vA3w3LxrEv87TgQfzvF4G+Gv+a7008Fc8r1uBh/CiQbzo3hv4Lp7X9wDvzX+P7wbei+f1PsB38y9DvOieDjyY5/UQ4Fb+ezwYeDrP61bgIfzLEC+a9wa+i+f1PcB789/ru4H34nm9D/DdvHCIF83TgQfzvB4C3Mp/rwcDT+d53Qo8hBcO8S97b+C7eF7fA7w3/zN8N/BePK/3Ab6bFwzxL3s68GCe10OAW/mf4cHA03letwIP4QVDvHDvDXwXz+t7gPfmf5bvBt6L5/U+wHfz/CFeuKcDD+Z5PQS4lf9ZHgw8ned1K/AQnj/EC/bewHfxvL4HeG/+Z/pu4L14Xu8DfDfPC/GCPR14MM/rIcCt/M/0YODpPK9bgYfwvBDP33sD38Xz+hrgo/mf7buB9+J5vQ/w3TwnxPP3dODBPKdLwIOBXf5nezDwdJ7XrcBDeE6I5/XewHfxvD4H+Gz+d/hu4L14Xu8DfDfPhnheTwcezHO6BDwY2OV/hwcDT+d53Qo8hGdDPKcPBb6B53UrcCsvul3gp4Hv4T/GewFvDRznRfdo4Dqe14cB38gViOe0Bnr+43wO8Nn8+3w28Fn8xxmAGVcgnu2jga/iP97LAH/Nv81LA3/Ff7yPAb4aQDzbRwNfxX+8vwZehn+bvwJemv94HwN8NYB4ttcGfov/HJ8DfDb/Op8NfBb/OV4H+G0A8WyvDfwWz+t3+Nd5LZ6/lwH+mhfNSwN/xfP3O/zrvBbP63WA3wYQz/bawG/xvMS/zmcDn8Xz+mvgZXjR/BXw0jyvzwE+m38d87xeB/htAPFsrw38Fs9L/Ov9NfBSPK/PAT6bF+6zgc/ief0N8NL865nn9TrAbwOIZ3tt4Ld4XuJf76WBv+L5+21euNfm+XsZ4K/51zPP63WA3wYQz/bawG/xvMS/zWcDn8V/jM8BPpt/G/O8Xgf4bQDxbK8N/BbPS/zb/TXwUvz7/A3w0vzbmef1OsBvA4hne23gt3he4t/upYG/4t/nZYC/5t/OPK/XAX4bQDzbawO/xfMS/z6fDXwW/zafA3w2/z7meb0O8NsA4tleG/gtnpf493tv4K2B47xodoGfBr6bfz/zvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/rOHCR5/U6wG8DiOdkntf7AN/N/07vDXwXz0tcgXhOtwIP4nl9NfDXwK387/Bg4KWBj+Z5PQN4MFcgntNHA1/F/20fA3w1VyCe128Dr8X/Tb8DvDbPhnheDwb+GjjG/y2XgJcGbuXZEM/fceC7gbfi/4afAd4b2OU5IV64twZeG3hp4LX43+V3gL8Gfhv4aZ4/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RCA7EQfADU38AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3cc+C7grfnP99fA+wB/zYsG8Z/vp4G34r/OLvAywK38yxD/+cx/vdcBfpt/GeI/318DL8V/rZcB/pp/GeI/30sDvw0c47/G5wCfzYsG8V/jwcCDedH8EHAdz+kpwAfwL9sF/poXHeJ/lrcGforn9T7Ad/MfD/E/y3cD78XzOgHs8h8P8T/HceAiz+t7gPfmPwfif473Br6L5/U2wE/znwPxP8dfAS/Nc3oG8GD+8yD+Z3gw8HSe19cAH81/HsT/DF8NfBTP6yHArfznQfzP8HTgwTynvwFemv9ciP9+bw38FM/rfYDv5j8X4r/fdwPvxfM6Aezynwvx3+s4cJHn9T3Ae/OfD/Hf672B7+J5vQ3w0/znQ/z3+ivgpXlOzwAezH8NxH+fBwNP53l9DfDR/NdA/Pf5auCjeF4PAW7lvwbiv8/TgQfznP4GeGn+6yD+e7w18FM8r/cBvpv/Ooj/Ht8NvBfP6wSwy38dxH+948BFntf3AO/Nfy3Ef733Br6L5/U2wE/zXwvxX++vgJfmOT0DeDD/9RD/tR4MPJ3n9TXAR/NfD/Ff66uBj+J5PQS4lf96iP9aTwcezHP6G+Cl+e+B+K/z1sBP8bzeB/hu/nsg/ut8N/BePK8TwC7/PRD/NY4DF3le3wO8N/99EP813hv4Lp7X2wA/zX8fxH+NvwJemuf0DODB/PdC/Od7MPB0ntfXAB/Nfy/Ef76vBj6K5/UQ4Fb+eyH+8z0deDDP6W+Al+a/H+I/11sDP8Xzeh/gu/nvh/jP9d3Ae/G8TgC7/PdD/Oc5DlzkeX0P8N78z4D4z/PewHfxvN4G+Gn+Z0D86700cIx/2bcDD+c53QO8My+aZwC38p8L8a/z2cBn8V9jF3gd4K/5z4N40b008Ff81/pr4GX4z4N40b028Fv81xP/eRAvugcDfw0c47/O9wDvzX8exL/OSwPfDbwU//m+B/hoYJf/PIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I8iNZ0FlLX+CAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeNEdB14LeGvgwcCDgQfzL9sFvhr4HP51Pgv4aOA4/7JbgVuBW4GfBn4H2OVfhviXPRj4LOC9+fd5G+CnedG8N/Bd/Pt8N/A5wK28YIgX7DjwWcBH8x/ja4CP5kXz3cB78R/js4GvAXZ5Xojn7zjwW8BL8x/nY4Cv5kXz0cBX8R/nr4HXAXZ5Tojn9dLAbwHH+Y/zN8BrA7u8aI4Dvw28FP9xdoHXAf6aZ0M8p+PA04HjvGCXgJ8GbgVuBW7lX/bb/Nu8Nv+yBwMPBh4MvDVwjBdsF3gIsMsViGc7DvwW8NI8f88APhv4bv5ne2/gs4EH8fz9NfA6wC6AeLavBj6K5+97gPfmf5fvBt6L5+9zgM8GEFc8GHg6z9/7AN/N/07vDXwXz99DgFvFFd8NvBfP63uA9+Z/t+8G3ovn9T3Aews4DlzkeT0DeDD/N9wKPIjndULAWwM/xfN6H+C7+b/hvYHv4nm9jYDvBt6L53QJOM7/LbvAMZ7T9wj4beC1eE7fA7w3/3rHgfcC3pr/HD8NfA+wy7/edwPvxXP6HQFPBx7Mc/oc4LP51/sr4KX5z/XXwMvwr/fZwGfxnP5agHle7wN8N/86rw38Fv81Xgf4bf513hv4Lp4TAszzeh/gu/nXeW3gt/iv8TrAb/Ov897Ad/GcEHAr8CCe0+cAn82/3l8DL8V/rr8BXpp/vc8GPovn9DcCfht4LZ7T9wDvzb/eceC9gdcGjvMfaxf4beC7gV3+9b4beC+e0+8I+G7gvXhOu8AJ/m+5CBznOX2PgLcGforn9T7Ad/N/w3sD38XzehsBx4GLPK9bgYfwf8PTgQfzvE6IK74beC+e13cD78P/bt8FvDfP63uA9xZXPBh4Os/f+wDfzf9O7w18F8/fQ4BbxbN9NvBZPH/fDbwP/7t8F/DePH+fA3w2gHi248BvAy/F83cr8NnA9/A/23sBnw08mOfvb4DXBnYBxHM6DtwKHOMF2wV+GrgVuBW4lX/Z7/Bv81r8yx4MPBh4MPDWwHFesEvAg4FdrkA8r5cGfhs4xn+cvwZeB9jlRXMc+C3gpfmPcwl4beCveTbE83cc+G3gpfiP8zHAV/Oi+Wjgq/iP8zfAawO7PCfEC3Yc+Gjgs/iP8TXAR/Oi+W7gvfiP8TnAVwO7PC/Ev+zBwGcD78W/z9sAP82L5q2Bn+Lf53uAzwZu5QVDvOiOA68NvDXwYOA48FL8yy4BXw18Nv86nw18NHCMf9nfALvArcBPA78N7PIvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEonapeXNIeYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+/FFd8D/DX/suPAewFvzf8cfw18D/DXPC/E8/fSwF/xnF4G+GteuL8CXpr/mV4G+GueE+L5+2rgo3hOXwN8NC/YawO/xf9cXwN8NM8J8fx9NfBRPKevAT6aF+y1gd/if66vAT6a54R4/l4a+Cue08sAf80L99fAS/E/08sAf81zQrxgLw28N1d8N/DX/MuOA+8NvDZwnP8Z/hr4buCveV6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/IV6wlwbeiyu+B/hr/mXHgfcC3pr/Of4a+B7gr3leiOfvpYG/4jm9DPDXvHB/Bbw0/zO9DPDXPCfE8/fVwEfxnL4G+GhesNcGfov/ub4G+GieE+L5+2rgo3hOXwN8NC/YawO/xf9cXwN8NM8J8fy9NPBXPKeXAf6aF+6vgZfif6aXAf6a54R4wV4aeG+u+G7gr/mXHQfeG3ht4Dj/M/w18N3AX/O8EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0O8YC8NvBdXfA/w1/zLjgPvBbw1/3P8NfA9wF/zvBDP30sDf8Vzehngr3nh/gp4af5nehngr3lOiOfvq4GP4jl9DfDRvGCvDfwW/3N9DfDRPCfE8/fVwEfxnL4G+GhesNcGfov/ub4G+GieE+L5e2ngr3hOLwP8NS/cXwMvxf9MLwP8Nc8J8YK9NPDeXPHdwF/zLzsOvDfw2sBx/mf4a+C7gb/meSH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHv4U5BVT7yVgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/c7008F7AS/Oi+23gr4G/AW7lX4b4n+mlgb/i3+ergc8BdnnBEP8zfTXwUfz7/TXwPsBf8/wh/mf6auCj+I/x18DrALs8L8T/TC8N/BX/cb4G+GieF+J/rpcG3ht4af51Xovn7yHArTwnxP89bw38FM/rbYCf5jkh/m/6beC1eE6fA3w2zwnxf9NvA6/Fc/oZ4K15Toj/m34beC2e0+8Ar81zQvzf9NvAa/Gcfgd4bZ4T4j/fceC7gLcG/hr4GOC3+c/128Br8Zx+B3htnhPiP99PA2/Fc3of4Lv5z/PbwGvxnH4HeG2eE+I/n3n+3gf4bv5z/DbwWjyn3wFem+eE+M/318BL8fy9D/Dd/Mf7beC1eE6/A7w2zwnxn++1gd/iBXsf4Lv5j/XbwGvxnH4HeG2eE+K/xnsD38UL9j7Ad/Mf57eB1+I5/Q7w2jwnxH+d9wa+ixfsfYDv5j/GbwOvxXP6HeC1eU6I/1rvDXwXL9j7AN/Nv99vA6/Fc/od4LV5Toj/eu8NfBcv2PsA382/z28Dr8Vz+h3gtXlOiP8e7w18Fy/Y+wDfzb/dbwOvxXP6HeC1eU6I/z7vDXwXL9j7AN/Nv81vA6/Fc/od4LV5Toj/Xu8NfBcv2PsA382/3m8Dr8Vz+h3gtXlOiP9+7w18Fy/Y+wDfzb/ObwOvxXP6HeC1eU6IF+ylgfcCXpr/fNcBj+YF+yzgc3nR/TbwWjyn3wFem+eEeP5eGvgr/md5Z+BHeNH8NvBaPKffAV6b54R4/r4a+Cj+Z/kr4GV50fw28Fo8p98BXpvnhHj+vhr4KP5n+SvgZXnR/DbwWjyn3wFem+eEeP5eGvgr/md5Z+BHeNH8NvBaPKffAV6b54R4wV4aeG/gpfnPdy3waF6wzwI+lxfdbwOvxXP6HeC1eU6I/37vDXwXL9j7AN/Nv85vA6/Fc/od4LV5Toj/Xu8NfBcv2PsA382/3m8Dr8Vz+h3gtXlOiP8+7w18Fy/Y+wDfzb/NbwOvxXP6HeC1eU6I/x7vDXwXL9j7AN/Nv91vA6/Fc/od4LV5Toj/eu8NfBcv2PsA382/z28Dr8Vz+h3gtXlOiP9a7w18Fy/Y+wDfzb/fbwOvxXP6HeC1eU6I/zrvDXwXL9j7AN/Nf4zfBl6L5/Q7wGvznBD/Nd4b+C5esPcBvpv/OL8NvBbP6XeA1+Y5If7zvTbwW7xg7wN8N/+xfht4LZ7T7wCvzXNC/Of7K+Clef7eB/hu/uP9NvBaPKffAV6b54T4z2eev/cBvpv/HL8NvBbP6XeA1+Y5If7z/TTwVjyn9wG+m/88vw28Fs/pd4DX5jkh/vMdB74beCvgb4CPBn6b/1y/DbwWz+l3gNfmOSH+b/pt4LV4Tr8DvDbPCfF/028Dr8Vz+h3gtXlOiP+bfht4LZ7TzwBvzXNC/N/028Br8Zw+B/hsnhPi/563Bn6K5/U2wE/znBD/c7008F7AS/Ov89o8fw8BbuU5If5nemngr/iP8zXAR/O8EP8zfTXwUfzH+BvgtYFdnhfif6avBj6Kf7+/Ad4b+GueP8T/TC8N/BX/Pl8DfDawywuG+J/rpYH3Bl6aF80u8NfAXwN/DdzKvwzx/xvi/zfE/2+I/98Q/7/xjw/XuUFOict/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAvugcDLwW8NPDavOj+Gvge4K/5nwfxLzsOfBbw0fz7vAzw1/zPgnjhXhr4KeDB/Pt9DfDR/M+CeMGOA38FPJj/GF8DfDT/syBesK8GPor/OC8D/DX/syCevwcDT+f5+x3+df4a+G7gr/mfB/H8vTXwUzyvtwF+mv87EM/fZwOfxXP6HeC1+b8F8fz9NvBaPKffAV6b/1sQz99vA6/Fc/od4LX5vwXx/P028Fo8p98BXpv/WxDP328Dr8Vz+h3gtfmvdxz4LuCtgb8GPgb4bf5jIJ6/3wZei+f0O8Br81/vp4G34jm9D/Dd/Pshnr/fBl6L5/Q7wGvzX888f+8DfDf/Pojn77eB1+I5/Q7w2vzX+2vgpXj+3gf4bv7tEM/fbwOvxXP6HeC1+a/32sBv8YK9D/Dd/Nsgnr/fBl6L5/Q7wGvz3+O9ge/iBXsf4Lv510M8f78NvBbP6XeA1+a/z3sD38UL9j7Ad/Ovg3j+fht4LZ7T7wCvzX+v9wa+ixfsfYDv5kWHeP5+G3gtntPvAK/Nf7/3Br6LF+x9gO/mRYN4/n4beC2e0+8Ar83/DO8NfBcv2PsA382/DPH8/TbwWjyn3wFem/853hv4Ll6w9wG+mxcO8fz9NvBaPKffAV6b/1neG/guXrD3Ab6bFwzx/P028Fo8p98BXpv/ed4b+C5esPcBvpvnD/H8/TbwWjyn3wFemxfdSwPvBbw0//muAx7NC/ZZwOfyvBDP328Dr8Vz+h3gtXnRvDTwV/zP8s7Aj/CcEM/fbwOvxXP6HeC1edF8NfBR/M/yV8DL8pwQz99vA6/Fc/od4LV50Xw18FH8z/JXwMvynBDP328Dr8Vz+h3gtXnRvDTwV/zP8s7Aj/CcEM/fbwOvxXP6HeC1edG9NPDewEvzn+9a4NG8YJ8FfC7PC/H8/TbwWjyn3wFem/953hv4Ll6w9wG+m+cP8fz9NvBaPKffAV6b/1neG/guXrD3Ab6bFwzx/P028Fo8p98BXpv/Od4b+C5esPcBvpsXDvH8/TbwWjyn3wFem/8Z3hv4Ll6w9wG+m38Z4vn7beC1eE6/A7w2//3eG/guXrD3Ab6bFw3i+ftt4LV4Tr8DvDb/vd4b+C5esPcBvpsXHeL5+23gtXhOvwO8Nv993hv4Ll6w9wG+m38dxPP328Br8Zx+B3ht/nu8N/BdvGDvA3w3/3qI5++3gdfiOf0O8Nr813tt4Ld4wd4H+G7+bRDP328Dr8Vz+h3gtfmv91fAS/P8vQ/w3fzbIZ6/3wZei+f0O8Br81/PPH/vA3w3/z6I5++3gdfiOf0O8Nr81/tp4K14Tu8DfDf/fojn77eB1+I5/Q7w2vzXOw58N/BWwN8AHw38Nv8xEM/fbwOvxXP6HeC1+b8F8fz9NvBaPKffAV6b/1sQz99vA6/Fc/od4LX5vwXx/H028Fk8p98BXpv/WxDP31sDP8Xzehvgp/m/A/H8PRh4Os/fb/Ov89fA9wB/zf88iBfsq4GP4j/OywB/zf8siBfsOPDXwIP4j/E1wEfzPwvihXtp4KeBB/Hv9zXAR/M/C+Jfdhz4bOCj+Pd5GeCv+Z8F8aJ7MPDSwEsDr82L7q+B7wb+mv95EP+/If5/Q/z/hvj/DfH/G/8IsV+0QdRMwMMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44d4aeC3gpYHX5n+X3wb+Gvgd4Kd5/hDP33Hgu4C35v+GnwbeB9jlOSGe14OBvwKO83/LLvAywK08G+J5/Rbw2vzf9NvA6/BsiOf00cBX8X/bxwBfzRWI5/QM4Bae0yXgu4HfBnb53+E48NrAewPHeE5/B7wkVyCek3leHwZ8I/87fTzwZTwvcQXi2T4a+Cqe1y7wPsBP87/LWwPfDRzjeX0M8NUA4tk+GvgqXrDvBj4G2OV/tuPAdwFvzQv2McBXA4hne23gt3jhbgXeB/ht/md6beC7gAfzwr0O8NsA4tleG/gtXjSfDXwO/7N8FfDRvGheB/htAPFsrw38Fi+6vwbeB/hr/nu9NPBdwEvzonsd4LcBxLO9NvBb/OvsAp8NfA3/PT4K+Gr+9V4H+G0A8WyvDfwWz+tzgM/ihftt4G2AXf5rPBj4LuC1eeE+B/gsntfrAL8NIJ7ttYHf4nkJeGngu4GX4gXbBd4H+Gn+c7018F3AcV6wvwHeG/hrwDyv1wF+G0A822sDv8XzElccBz4b+CheuO8GPgbY5T/WceC7gLfmhfsa4LOBXa4wz+t1gN8GEM/22sBv8bzEc3pt4KeBY7xgtwLvA/w2/zFeG/gu4MG8YJeAtwZ+m+dkntfrAL8NIJ7ttYHf4nmJ53Uc+G7grXjhPhv4HP59vgr4aF64nwHeG9jleZnn9TrAbwOIZ3tt4Ld4XuIFe2/gq4FjvGB/DbwP8Nf867w08F3AS/OCXQI+GvhuXjDzvF4H+G0A8WyvDfwWz0u8cA8Gvht4LV6wXeCzga/hRfNRwFfzwv0O8N7Arbxw5nm9DvDbAOLZXhv4LZ6XeNF8NvBZvHC/DbwNsMvz92Dgu4DX5oX7HOCzedGY5/U6wG8DiGd7beC3eF7iRffSwHcDL8ULtgu8D/DTPKe3Br4LOM4L9jfAewN/zYvOPK/XAX4bQDzbawO/xfMS/zrHgc8GPooX7ruBj+GK7wLemhfua4DPBnb51zHP63WA3wYQz/bawG/xvMS/zWsDPw0c4wW7lSsezAt2CXhr4Lf5tzHP63WA3wYQz/bawG/xvMS/3XHgu4G34t/mZ4D3Bnb5tzPP63WA3wYQz/bawG/xvMS/33sDXw0c40VzCfho4Lv59zPP63WA3wYQz/bawG/xvMR/jAcD3w28Fi/c7wDvDdzKfwzzvF4H+G0A8WyvDfwWz0v8x/ps4LN4/j4H+Gz+Y5nn9TrAbwOIZ3tt4Ld4XuI/3ksD3w28FFf8DfDewF/zH888r9cBfhtAPNtrA7/F8xL/eV6bK36b/zzmeb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/r+PARZ7X6wC/DSCek3le7wN8N/87vTfwXTwvcQXiOd0KPIjn9dXAXwO38r/Dg4GXBj6a5/U3wEtzBeI5fTTwVfzf9jHAV3MF4nn9NvBa/N/0O8Br82yI5/Vg4K+BY/zfcgl4aeBWng3x/B0Hvht4K/5v+BngvYFdnhPihXtr4LWBlwZei/9dfgf4a+C3gZ/m+UP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGhx+hBhxk7VQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE20lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/c7008F7AS/NvswVcB5wFvgT4EZ4X4n+mlwb+iv9Y7wz8CM8J8T/TVwMfxX+svwJelueE+J/pq4GP4j/WHcDNPCfE/0wvDfwV/7H+Anh5nhPif66XBt4beGn+ZdcCj+aF+x3gtXlOiP/93hv4Lv5lvwO8Ns8J8b/bewPfxYvmd4DX5jkh/vd6b+C7eNH9DvDaPCfE/07vDXwXL9gTgEfznH4HeG2eE+J/n/cGvosX7H2A9wZei+f0O8Br85wQ/7u8N/BdvGDvA3w38NvAa/Gcfgd4bZ4T4n+P9wa+ixfsfYDv5orfBl6L5/Q7wGvznBD/O7w38F28YO8DfDfP9tvAa/Gcfgd4bZ4T4n++9wa+ixfsfYDv5jn9NvBaPKffAV6b54T4n+29ge/iBXsf4Lt5Xr8NvBbP6XeA1+Y5If7nem/gu3jB3gf4bp6/3wZei+f0O8Br85wQ/zO9N/BdvGDvA3w3L9hvA6/Fc/od4LV5Toj/ed4b+C5esPcBvpsX7reB1+I5/Q7w2jwnxP8sbw38FC/Y+wDfzb/sq4GP4jl9DfDRPCfE/xwPBv4KOM7z9z7Ad/OieWngr3hOLwP8Nc8J8T/HawO/xfP3PsB386/z0sB7c8V3A3/N80L8z/HSwF/xvN4H+G7+cyD+Z/ls4LO44hLw3sBP858H8T/PSwPHgb8GdvnPhfj/DfH/G+L/N8T/b4j/Gg8GHgRcAv6a/zkQ//neGvgu4DhXfA7w2fzPgPjP9d7Ad/G8Xgb4a/77IV6wlwbeiyu+B/hr/nXeG/gunr/XAX6b/36I5++lgb/iOb0M8Ne8aN4b+C6ev0vAg4Fd/vshnr+vBj6K5/Q1wEfzL3tv4Lt4wd4G+Gn+Z0A8f78NvBbP6XeA1+aFe2/gu3jB3gf4bv7nQDx/vw28Fs/pd4DX5gV7b+C7eMHeB/hu/mdBPH+/DbwWz+l3gNfm+Xtv4Lt4wd4H+G7+50E8f78NvBbP6XeA1+Z5vTfwXbxg7wN8N/8zIZ6/3wZei+f0O8Br85zeG/guXrD3Ab6b/7kQz99vA6/Fc/od4LV5tvcGvosX7H2A7+Z/NsTz99vAa/Gcfgd4ba54b+C7eMHeB/hu/udDPH+/DbwWz+l3gNcG3hv4Ll6w9wG+m/8dEM/fbwOvxXP6HeC7ge/iBXsf4Lv53wPx/P028Fo8pycAj+YFex/gu/nfBfH8/TbwWrzo3gf4bv73QTx/vw28Fi+a9wG+m/+dEM/fbwOvxb/sfYDv5n8vxPP328Br8cI9AbiHf9lfA98D/DX/8yCevz8HXo7/WC8D/DX/syCev9uBm/iP9TXAR/M/C+L5+0vgZfiP9TXAR/M/C+L5eyfgh/mP9TLAX/M/C+IFeyfgk4AzwL3AAf82fw18N/DX/M+D+P8N8f8b4v83xP9viP/f+Eessp5BLS2VwgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFC0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAvugcDLwW8NPDavHC3Aj8D/DT/Md4aeCvgwbxwvw38NfA3wK38yxD/suPAZwEfzb/e5wCfzb/PZwOfxb/eVwOfA+zygiFeuJcGfgp4MP82u8AJ/n0uAsf5t/lr4H2Av+b5Q7xgx4G/Ah7Mv90zgAfz77MLHOPf7q+B1wF2eV6IF+yrgY/i3+dzgM/m3+ezgc/i3+drgI/meSGevwcDT+f5+x3+ZbcCPw38NP8x3hp4a+DB/Mtei+fvIcCtPCfE8/fWwE/xvN4G+Gn+Z3tr4Kd4Xm8D/DTPCfH8fTbwWTyn3wFem/8dfht4LZ7T5wCfzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/QzwFvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f98NvBfP6XuA9+Z/h+8G3ovn9D3Ae/OcEM/fWwM/xXN6G+Cn+d/hrYGf4jm9DfDTPCfEC/bZwEdzxVcDn83/Lp8NfDRXfDXw2TwvxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEC/ZZwEdzxVcDn8P/Lp8FfDRXfDXwOTwvxPP33sB38ZzeBvhp/nd4b+C7eE5vA/w0zwnx/H038F48p+8B3pv/Hb4beC+e0/cA781zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7TzwBvzXNCPH+fDXwWz+l3gNfmf4ffBl6L5/Q5wGfznBDP31sDP8Xzehvgp/mf7a2Bn+J5vQ3w0zwnxPP3YODpPH+/zb/sVuBngJ/mP8ZbA28FPJh/2Wvz/D0EuJXnhHjBvhr4KP59Pgf4bP59Phv4LP59vgb4aJ4X4gU7Dvw18CD+7XaBE/z7XASO82/3N8BrA7s8L8QL99LATwMP4t/mGcCD+ffZBY7xb/M3wHsDf83zh/iXHQc+G/go/vU+B/hs/n0+G/gs/vW+BvhsYJcXDPGiezDw0sBLA6/NC3cr8NPAT/Mf462BtwYezAu2C/w18NfAXwO38i9D/P+G+P8N8f8b4v83xP9v/CMvcalBK1w3xgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/u/018D3AX/O8EM/fSwN/xf8tLwP8Nc8J8fx9NfBR/N/yNcBH85wQz99XAx/F/y1fA3w0zwnx/L008Ff83/IywF/znBAv2EsD7w28NP+7/TXw3cBf87wQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q7xgLw28F/DS/O/218D3AH/N80I8fy8N/BX/t7wM8Nc8J8Tz99XAR/F/y9cAH81zQjx/Xw18FP+3fA3w0TwnxPP30sBf8X/LywB/zXNCvGAvDbw38NL87/bXwHcDf83zQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xgr008F7AS/O/218D3wP8Nc8L8fy9NPBX/N/yMsBf85wQz99XAx/F/y1fA3w0zwnx/H018FH83/I1wEfznBDP30sDf8X/LS8D/DXPCfGCvTTw3sBL87/bXwPfDfw1zwvx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/IjypBvBB7UwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/d7008F5c8T3AX/O8EP83vTfwXTynlwH+mueE+L/nvYHv4nl9DfDRPCfE/ywvDXwX8NLATwPvA+zyontv4Lt4/r4G+GieE+J/jgcDfwUc59l+BnhrXjTvDXwXL9jLAH/Nc0L8z/HawG/xvMS/7L2B7+IFex/gu3leiP85Xhv4LZ6XeOHeG/guXrD3Ab6b5w/xP8drA7/F8xIv2HsD38UL9j7Ad/OCIf7neG3gt3he4vl7b+C7eMHeB/huXjjE/xyvDfwWz0s8r/cGvosX7H2A7+Zfhvif47WB3+J5ief03sB38YK9D/DdvGgQ/3O8NvBbPC/xbO8NfBcv2PsA382LDvE/x2sDv8XzEle8N/BdvGDvA3w3/zqI/zleG/gtnpeA9wa+ixfsfYDv5l8P8T/HawO/xfN6H+C7eMHeB/hu/m0Q/3O8NvBb/Ou8D/Dd/Nsh/ud4beC3eNG9D/Dd/Psg/ud4beC3eNG8D/Dd/Psh/ud4beC3+Je9D/Dd/MdA/M/x2sBv8R/np4H3AXZ5wRD/c7wz8EP8x/oZ4K15wRD/c3w08FX8xxMvGOJ/jncCfpj/WH8DvDQvGOJ/lt8CXpv/GJeA1wb+mhcM8T/POwHXA08BDvi3+2tglxcO8f8b4v83xP9viP/fEC+aBwMP4t/uOuA64B7gR/ifA/Eve2ngt4Dj/Mf4beB1+J8B8S/7K+Cl+Y/1zsCP8N8P8S8z//E+Bvhq/vsh/mU/DbwV/7HeAvh5/vsh/mXHge8G3or/OK8D/Db//RD/sd4b+C7+Za8D/Db//RD/cd4b+C5eNK8D/Db//RD/Md4b+C5edK8D/Db//RD/fu8NfBf/Oq8D/Db//RD/Pu8NfBcv2PsA38Xzeh3gt/nvh/i3e2/gu3jB3gf4bsA8r9cBfpv/foh/m/cGvosX7H2A7+YK87xeB/ht/vsh/vXeG/guXrD3Ab6bZzPP63WA3+a/H+Jf572B7+IFex/gu3lO5nm9DvDb/PdDvOjeG/guXrD3Ab6b52We1+sAv81/P8SL5r2B7+IFex/gu3n+zPN6HeC3+e+H+Je9N/BdvGDvA3w3L5h5Xq8D/Db//RAv3HsD38UL9j7Ad/PCmef1OsBv898P8YK9N/BdvGDvA3w3/zLzvF4H+G3++yGev5cG/ooX7H2A7+ZF89PAW/Fsl4AHA7v890M8f18NfBTP3/sA382L7jjw3cBbAX8DvDfw1/zPgHj+vhr4KJ7X+wDfzf8diOfvpYG/4jm9D/Dd/N+CeMFeGnhvrvhu4K/5vwfx/xvi/zfE/2+I/98Q/7/xjwe1j0Fu2P05AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEsUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sY4D7wW8NPAzwE/zH+OtgbcC/hr4HmCX/xiI/1h/Bbw0z/Y5wGfz7/PZwGfxbH8NvAz/MRD/cV4b+C2e0y5wgn+fi8BxntPrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85HA1/Fc7oEHOffZxc4xnP6GOCr+fdD/Mf5LeC1eU6/A7w2/z6/DbwWz+mngbfh3w/xwr0W/7KXBt4aeG2e1+cAn82/z2cDn8Xz+m3gp4G/5l/2Ozx/iOfvOPBbwEvzb3cJeG3gr/n3eWngt4Fj/Nv9NfA6wC7PCfH8fTTwVfz7fAzw1fzH+Gjgq/j3+Rjgq3lOiOfvu4H34t/uZ4C35j/WTwNvxb/d1wAfzXNCPH9vDfwU/3qXgM8Gvpr/HB8NfDZwjH+9twF+mueEeME+G/ho4Bgv3CXgr4HfBn4a+Gv+c7008NbAawMvDRzjhbsEfDXw2TwvxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEC/ZZwEcDx3nhdoG/Bn4b+Bngr/nP9dLAWwGvDbw0cJwXbhf4auBzeF6I5++9ge/iX28X+Gzga/jP8VHAV/Nv8zbAT/OcEM/fdwPvxb/dTwNvw3+snwLemn+7rwE+mueEeP4+Gvgq/n0+Bvhq/mN8NPBV/Pt8DPDVPCfE83cc+G3gpfi32wVeB/hr/n1eGvgr/n3+BnhtYJfnhHjhXpt/2UsDrw28Fc/rc4DP5t/ns4HP4nn9DPDbwF/zL/ttnj/Ef5zfBl6L5/Q7wGvz7/PbwGvxnH4GeGv+/RD/cT4a+Cqe0y5wgn+fi8BxntPHAF/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/F8xL/PuZ5vQ7w2/z7If7jvDbwWzwv8e9jntfrAL/Nvx/iP85rA7/Fc3oG8GD+fXaBYzyn1wF+m38/xH+svwZeimf7HOCz+ff5bOCzeLa/AV6a/xiI/1jHgfcGHgz8NvDT/Md4a+C1gVuB7wZ2+Y+B+P8N8f8b4v83xP9viP/f+EfA8ZxBWMMUPwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABu0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/OseB9wLemv+Zfhr4HmCXFw3iX+evgJfmf7a/Bl6GFw3iRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4b41/lr4KX4n+1vgJfmRYP41zkOvDfw2sBx/mfZBX4b+G5glxcN4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BbyIPQd7ZrxwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv3HHgpYCXBl4aeDD/O9wK/DXw18DfALs8f4gX7K2B7wKO87/bLvA+wE/zvBDP6zjwXcBb83/LTwNvw3NCPK+vBj6K/5u+Bvhong3xnF4b+C3+b3sd4Le5AvGcfht4LZ7XM4Bb+d/lwcCDeF6/A7w2VyCek3le3wO8N/87fTfwXjwvcQXi2V4a+Cue10OAW/nf6cHA03leLwP8NYB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/3sdBy7yvF4H+G0A8ZzM83of4Lv53+m9ge/ieYkrEM/pVuBBPK+vBv4auJX/WC/NFX/Nf6wHAy8NfDTP6xnAg7kC8Zw+Gvgq/m/7GOCruQLxvH4beC3+b/od4LV5NsTzejDw18Ax/m+5BLw0cCvPhnj+jgPfDbwV/zf8DPDewC7PCfHCvTXw2sBLA6/F/y6/A/w18NvAT/P8If57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/1jHgfcC3poX7qWB4zynXeCveeF+GvgeYJf/GIj/WH8FvDT/uf4aeBn+YyD+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/XXwEvxn+tvgJfmPwbiP9Zx4L2B1waO88JtAddyxb3AAS/cLvDbwHcDu/zHQPz/hvj/DfH/G+L/N8T/b/wjcTehQXh5YQcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADBklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+Jf76WBY8AzgFv593ktrvgd/n0eDDwIuAT8NS86xIvuo4DPBo7zbH8NvA/w17zojgPfBbw1z+mngfcBdnnRvTTwXcBL82y7wGcDX8O/DPGi+S7gvXnBXgf4bf5lx4G/Ah7M8/fXwOsAu/zL3hr4KV6w7wbehxcO8S97a+CneOF2gYcAu7xwPwW8NS/cTwNvwwt3HHg6cJwX7m2An+YFQ/zLfht4Lf5l7wN8Ny+cedGIF+69ge/iX/Y7wGvzgiH+ZReB4/zLvgb4aF6w1wZ+ixfN6wC/zQv21cBH8S/bBU7wgiH+ZbvAMf5lXwN8NC/YawO/xYvmdYDf5gX7auCj+JddAo7zgiH+Zb8NvBb/svcBvpsXzrxoxAv33sB38S/7HeC1ecEQ/7K3Bn6KF+4S8GBglxfup4G34oX7HuC9eeGOA7cCx3jh3gb4aV4wxIvmu4H34gV7HeC3+ZcdB34beCmev78BXhvY5V/22sBv8YJ9DfDRvHCIF91HA58NHOPZ/gZ4b+CvedEdB74beCue088A7w3s8qJ7aeC7gZfi2S4Bnw18Nf8yxL/eSwPHgVuBW/n3eW2u+G3+fR4MPBjYBf6aFx3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I8fXTkGtlzPjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADAUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HWwEcBL80Vfw18DfDT/NdC/Nf7LuC9ef6+G3gf/usg/mt9NPBVvHAfA3w1/zUQ/7UuAsd54XaBE/zXQPzXeWngr3jRvAzw1/znQ/zXeW3gt3jRvA7w2/znQ/zXeTDwdF40DwFu5T8f4r/WXwMvxQv3N8BL818D8V/rpYG/4oV7GeCv+a+B+K/32sBPA8d4TpeAtwZ+m/86iP8ex4G3Bl6aK/4a+Glgl/9aiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8R/r9fiit/hvwfiv95x4LuAt+Y5/TTwPsAu/3UQ/7WOA38FPJjn76+B1wF2+a+B+K/1U8Bb88L9NPA2/NdA/NcyLxrxXwPxX+e1gd/iRfM6wG/znw/xX+e1gd/iRfM6wG/znw/xX8u8aMR/DcR/rZ8G3ooX7nuA9+a/BuK/1nHgt4GX4vn7G+C1gV3+ayD+6x0Hvht4K57TzwDvDezyXwfx3+u1ueK3+e+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z3OA68FfDSXPHXwM8Au/zXQvzXe2vgu4DjPKdd4G2A3+a/DuK/1ksDf8UL9zLAX/NfA/Ff66+Al+aF+2vgZfivgfiv82Dg6bxoHgLcyn8+xH+d1wZ+ixfN6wC/zX8+xH+dlwb+ihfNywB/zX8+xH+tXeAYL9wl4Dj/NRD/tT4a+CpeuI8Bvpr/Goj/et8NvBfP39cAH81/HcR/j7cGPhp4aa74a+CrgZ/mvxbi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjGQEpBkRxkqgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 19 21 12 17 5 21 12 2",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "3 11 22 2 13 21 11 13 3 11",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIOklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x3HgpXhOfwPs8l8L8Z/vrYC3Bh4MvDYvmt8GbgV+GvgZ/vMg/uM9GHgr4LWBt+Y/xk8Dvw18D7DLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jfBTw2cBx/mvsAp8NfA3/Poh/n7cGvgp4MP89bgXeB/ht/m0Q/3YfBXw1L5pLwE8DtwK/zRW7wF9zxUsDx7nitYEHA28NHONF8z7Ad/Ovh/i3+S7gvXnhngH8NPDTwG/zb/PawFsDbw08iBfuu4H34V8H8a9zHPgt4KV5wS4BXw18Nv+xPhv4aOAYL9hvA28D7PKiQbzojgO/Bbw0z98l4KuBrwZ2+c9xHPho4KOBYzx/fw28DrDLvwzxovsr4KV5/p4BvDXw1/zXeGngp4EH8fz9NvA6/MsQL5rvAt6b5+9vgNcGdvmvdRz4beCleP6+G3gfXjjEv+yjga/i+fse4L357/XdwHvx/L0P8N28YIgX7q2Bn+L5+x7gvfmf4buB9+L5ex3gt3n+EC/c04EH87z+Bnhp/mf5a+CleF63Ag/h+UO8YB8NfBXP6xnASwO7/M9yHPhr4EE8r48BvprnhXj+jgNPB47znC4Brw38Nf8zvTTw28AxntMu8BBgl+eEeP4+G/gsntfnAJ/N/2yfDXwWz+tzgM/mOSGe14OBvwKO85wuAQ8Gdvmf7ThwK3CM57QLPATY5dkQz+ujga/ieX0O8Nn827w18FbAXwPfA+zy/B0H3gt4aeBngJ/m3+azgc/ieb0P8N08G+J5/TTwVjynZwAP5t/ms4HP4tn+GngZnr+/Al6aZ/sc4LP5t7kVeBDP6WeAt+bZEM/pOHCR5/U1wEfzb3MROM5zeh3gt3lOrw38Fs9pFzjBv81XAx/F8xLPhnhObw38FM/rdYDf5t/GPK/XAX6b5/TawG/xvMS/zWsDv8Xzehvgp7kC8Zy+G3gvntMl4Dj/duZ5vQ7w2zyn1wZ+i+cl/u12gWM8p+8B3psrEM/pt4HX4jl9D/De/NuZ5/U6wG/znF4b+C2el/i3+27gvXhOvwO8NlcgnpN5Xp8DfDb/duZ5vQ7w2zyn1wZ+i+cl/u0+G/gsnpe4AvGczPN6HeC3+bczz+t1gN/mOb028Fs8L/Fv99rAb/G8xBWIZzsOXOR5vQ7w2/zbmef1OsBv85xeG/gtnpf4t3tt4Ld4XieAXQDxbK8N/BbP62WAv+bfzjyv1wF+m+f02sBv8bzEv91LA3/F83od4LcBxLO9NvBbPC/x72Oe1+sAv81zem3gt3he4t/HPK/XAX4bQDzbawO/xfMS/z7meb0O8Ns8p9cGfovnJf59zPN6HeC3AcSzvTbwWzyvhwC38m9nntfrAL/Nc3pt4Ld4XuLf7qWBv+J5vQ7w2wDi2Y4DF3lerwP8Nv925nm9DvDbPKfXBn6L5yX+7V4b+C2e1wlgF0A8J/O8Xgf4bf7tzPN6HeC3eU6vDfwWz0v827028Fs8L3EF4jmZ5/UxwFfzb2ee1+sAv81zem3gt3he4t/us4HP4nmJKxDP6beB1+I5fQ/w3vzbmef1OsBv85xeG/gtnpf4t/tu4L14Tr8DvDZXIJ7TdwPvxXPaBU7wb2ee1+sAv81zem3gt3he4t/uInCc5/Q9wHtzBeI5vTXwUzyv1wF+m38b87xeB/htntNrA7/F8xL/Nq8N/BbP622An+YKxHM6DlzkeX0N8NH82+wCx3hOrwP8Ns/ptYHf4jldAo7zb/PVwEfxvMSzIZ7XTwNvxXO6FXgI/zafDXwWz/Y3wEvz/P018FI82+cAn82/zdOBB/OcfgZ4a54N8bw+GvgqntfnAJ/Nv81bA68N3Ap8N7DL83cceG/gwcBvAz/Nv81nA5/F83of4Lt5NsTzOg7cChzjOe0CDwF2+Z/tOPB04DjP6RLwYGCXZ0M8f58NfBbP63OAz+Z/ts8GPovn9TnAZ/OcEM/fceBW4BjPaRd4HeCv+Z/ppYG/4nldAh4M7PKcEC/YRwNfxfO6FXgZYJf/WY4DTweO87w+BvhqnhfihbsVeBDP66+Bl+F/lr8CXprn9QzgwTx/iBfutYHf4vn7buB9+J/hu4D35vl7HeC3ef4Q/7KPBr6K5++7gffhv89x4KuA9+b5ex/gu3nBEC+a7wbei+fvr4HXAXb5r3Uc+C3gpXn+vgd4b144xIvur4GX4vm7FXgb4K/5r/HSwG8Bx3n+fgd4bf5liBfdceC3gZfi+dsFvhr4GmCX/xzHgY8CPpsX7G+A1wZ2+Zch/nWOA78NvBQv2C7w1cDn8B/rs4CPBo7zgv0O8NbALi8axL/NdwPvxQt3K/DTwE8Dv8O/zWsBbw28NfBgXrjvAd6bfx3Ev917A9/Fi2YX+GngVuC3ueIS8Ndc8dLAMa54beDBwFsDx3nRvA/w3fzrIf59Xhv4buBB/Pd4BvDewG/zb4P4j/HRwGcDx/ivcQn4bOCr+fdB/Mc5Dnw08NHAMf5zXAK+GvhqYJd/P8R/vOPAWwNvDbwV/zF+Bvhp4KeBXf7jIP7zvTXw1sCDgdfiRfM7wK3ATwM/zX8exH+P48BL85z+Gtjlvxbi/zfE/2/8IxFJTlAplAlZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjf5aWB7wJemn+/u4FXEf97PBj4K+A4/3FuEy+61+JF8zfALv/xXhv4Lf5jIV64zwLeGnhp/nV2gc8Gvob/OC8N/BX/sRDP34OBnwJemn+fvwbeB/hr/mN8NvBZ/MdBPK8HA38FHOc/xq3AywC7/Md4aeA4/7I3Bj6JFw7xvH4KeGv+Y/018DL813lv4Lv4lyGe02sDv8V/jrcBfpr/fO8NfBcvGsRz+m3gtXhenwN8NbDLC/dg4L2Bz+J5fQ/w3vznem/gu3jRIZ7TbwOvxXP6HuC9+df5beC1eE67wAn+87w38F28YF8CfBLPCfGcLgLHeU5vA/w0/zpvDfwUz+shwK38x3tv4Lt4wd4HuBX4LZ4T4jmZ5/UxwFfzr/Ng4Ok8r9cBfpv/WO8NfBcv2PsA3w28NvBbPCfEc/pr4KV4Tl8DfDT/ervAMZ7T5wCfzX+c9wa+ixfsfYDv5orXBn6L54R4Tj8NvBXP6XeA1+Zf77eB1+I5/Qzw1vzHeG/gu3jB3gf4bp7ttYHf4jkhntNnA5/Fc9oFTvCv99nAZ/G8TgC7/Pu8N/BdvGDvA3w3z+m1gd/iOSGe01sDP8XzEv96Lw38Fc/rfYDv5t/uvYHv4gV7H+C7eV6vDfwWzwnxnF4a+Cue1+sAv82/3i5wjOf0PcB782/z3sB38YK9D/DdPH+vDfwWzwnxvMzz+hzgs/nX+27gvXhOtwIP4V/vvYHv4gV7H+C7ecFeG/gtnhPief028Fo8p78GXoZ/vbcGforn9RDgVl507w18Fy/Y+wDfzQv32sBv8ZwQz+ujga/ieZ0AdvnXeW3gt3herwP8Ni+a1wZ+ixfsfYDv5l/20sBf8Zz+Rjyvlwb+iuf1PsB386/z1cBH8bweAtzKi+avgJfm+Xsf4Lt50X028FlccQl4bfH83Qo8iOf0PcB786J7b+C7eF7PAB7Mi848f+8DfDf/ei8NHAf+GtgVz993A+/Fc9oFTvCieW/gu3j+Pgf4bF50Pw28Fc/pfYDv5t8P8fy9NfBTPK/3Ab6bF+69ge/i+XsG8NLALi+648BXA+8F/A3w0cBv8x8D8fwdB24FjvGc/hp4GV6w1wZ+ixfsZYC/5n8OxAv21cBH8bxeB/htnr+nAw/m+Xsf4Lv5nwXxgj0YeDrP63uA9+b5M8/f+wDfzf88iBfup4G34nk9BLiV53Ur8CCe1+cAn83/PIgX7q2Bn+J5fQ/w3jyv1wZ+i+fvIcCt/M+C+JfdCjyI5/UywF/zvL4a+Cie128Dr8P/LIh/2XsD38Xz+m3gdXhex4G/Bh7E8/pu4H34nwPxovlr4KV4Xm8D/DTP67WB3+L5+xzgs/mfAfGieW3gt3hetwIP4fn7auCjeP7eB/hu/vshXnQ/DbwVz+trgI/meR0Hfht4KZ6/twF+mv9eiBfdg4Gn8/y9DvDbPK/jwF8DD+L5ex/gu3nRHQdeCrgE/DX/foh/nc8GPovntQs8BNjleb008NvAMZ6/zwY+hxfuq4DXBl6aZ9sFfhr4beBngF3+9RD/en8NvBTP66eBt+H5e2ngr3jBvht4H57XSwPfBbw0/7KfBv4a+G3gGcCt/MsQ/3ovDfw2cIzn9THAV/P8vTfwXbxgvw28D3ArV7w38F38290K/Dbw08DP8Pwh/m0+Gvgqnr/3Ab6b5++lgd8GjvGCfTbwDOC7+I/z18DrALs8J8S/3U8Db8Xz9zrAb/P8vTTw08CD+K91K/A2wF/zbIh/u+PArcAxntcu8DrAX/P8HQd+G3gp/nU+Bvhr4Djw1sBrAw/iRffXwMvwbIh/n5cGfhs4xvPaBV4H+Guev+PAZwMfxYvmfYDv5nm9NPDWwGsDLw0c44X7GOCruQLx7/fSwF/x/O0CbwP8Ni/YawPfDTyIF+x9gO/mRfNg4KWBjwZei+e1C5zgCsR/jPcGvosX7H2A7+YFOw58NvBRPK/3Ab6bf5vPBj6L53UC2AUQ/3HeG/guXrD3Ab6bF+61ge8GHgQ8A3hv4Lf5t3tp4K94Xq8D/DaA+I/13sB38YL9NPA+wC7/dczzeh3gtwHEf7z3Br6LF+xW4H2A3+a/hnlerwP8NoD4z/HSwG8Dx3jBPhv4HP7zmef1OsBvA4j/PC8N/DZwjBfsVuB9gN/mP495Xq8D/DaA+M91HPhp4LV44X4beB/gVv7jmef1OsBvA4j/Gh8NfBX/sq8GPgfY5T+OeV6vA/w2gPiv89LAdwMvxQu3C/w08DnArfz7mef1OsBvA4j/WseBzwY+ihfNbwNfDfwM/3bmeb0O8NsA4r/Hg4HvBl6LF82twE8Dvw38DC+648BFntfrAL8NIP57vTbw3cCD+Nf5aeCngd8BbuUF+2jgq3heJ4BdAPE/w0cDHw08iH+b3wZuBW4FfpsrXhv4bJ7XM4AHcwXif5b3Bt4beC3+83wN8NFcgfif6aWBjwbei/9YfwO8NrDLFYj/2Y4Dbw28NfDawDH+7S4Brw38Nc+G+N/ltYG3Bt4aeBAvur8B3hv4a54T4n+v48BLAy8NPBh4aeClgWNccQn4a+C3gc/m+UP83/Rg4Fb+ZYj/3xD/v/GPJhRLP1+kgYwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "5 3 19 12 5 21 5 3",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x3HgpXhOfwPs8l8L8Z/vrYC3Bh4MvDYvmt8GbgV+GvgZ/vMg/uM9GHgr4LWBt+Y/xk8Dvw18D7DLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jfBTw2cBx/mvsAp8NfA3/Poh/n7cGvgp4MP89bgXeB/ht/m0Q/3YfBXw1L5pLwE8DtwK/zRW7wF9zxUsDx7nitYEHA28NHONF8z7Ad/Ovh/i3+S7gvXnhngH8NPDTwG/zb/PawFsDbw08iBfuu4H34V8H8a9zHPgt4KV5wS4BXw18Nv+xPhv4aOAYL9hvA28D7PKiQbzojgO/Bbw0z98l4KuBrwZ2+c9xHPho4KOBYzx/fw28DrDLvwzxovsr4KV5/p4BvDXw1/zXeGngp4EH8fz9NvA6/MsQL5rvAt6b5+9vgNcGdvmvdRz4beCleP6+G3gfXjjEv+yjga/i+fse4L357/XdwHvx/L0P8N28YIgX7q2Bn+L5+x7gvfmf4buB9+L5ex3gt3n+EC/c04EH87z+Bnhp/mf5a+CleF63Ag/h+UO8YB8NfBXP6xnASwO7/M9yHPhr4EE8r48BvprnhXj+jgNPB47znC4Brw38Nf8xXosrfof/GC8N/DZwjOe0CzwE2OU5IZ6/zwY+i+f1OcBn8+93HPgt4KW54q+B1wF2+ff7bOCzeF6fA3w2zwnxvB4M/BVwnOd0CXgwsMu/30cDX8Vz+hjgq/n3Ow7cChzjOe0CDwF2eTbE8/po4Kt4Xp8DfDb/MX4beC2e088Ab81/jM8GPovn9T7Ad/NsiOf108Bb8ZyeATyY/zi/DbwWz+l3gNfmP86twIN4Tj8DvDXPhnhOx4GLPK+vAT6a/zi/DbwWz+l3gNfmP85XAx/F8xLPhnhObw38FM/rdYDf5j/ObwOvxXP6HeC1+Y/z2sBv8bzeBvhprkA8p+8G3ovndAk4zn+s3wZei+f0O8Br8x9rFzjGc/oe4L25AvGcfht4LZ7T9wDvzX+s3wZei+f0O8Br8x/ru4H34jn9DvDaXIF4TuZ5fQ7w2fzH+m3gtXhOvwO8Nv+xPhv4LJ6XuALxnMzzeh3gt/mP9dvAa/Gcfgd4bf5jvTbwWzwvcQXi2Y4DF3lerwP8Ni+a48B7AW/NC/fSwHGe0y7w17xwPw18D7DLi+a1gd/ieZ0AdgHEs7028Fs8r5cB/poXzV8BL81/rr8GXoYXzUsDf8Xzeh3gtwHEs7028Fs8L/GieW3gt/iv8TrAb/OiMc/rdYDfBhDP9trAb/G8xIvmtYHf4r/G6wC/zYvGPK/XAX4bQDzbawO/xfN6CHArL5q/Bl6K/1x/A7w0L5qXBv6K5/U6wG8DiGc7Dlzkeb0O8Nu8aI4D7w28NnCcF+ylgOM8p13gb3jBdoHfBr4b2OVF89rAb/G8TgC7AOI5mef1OsBv8x/rt4HX4jn9DvDa/Md6beC3eF7iCsRzMs/rY4Cv5j/WbwOvxXP6HeC1+Y/12cBn8bzEFYjn9NvAa/Gcvgd4b/5j/TbwWjyn3wFem/9Y3w28F8/pd4DX5grEc/pu4L14TrvACf5j/TbwWjyn3wFem/9YF4HjPKfvAd6bKxDP6a2Bn+J5vQ7w2/zH+W3gtXhOvwO8Nv9xXhv4LZ7X2wA/zRWI53QcuMjz+hrgo/mP89vAa/Gcfgd4bf7jfDXwUTwv8WyI5/XTwFvxnG4FHsJ/nN8GXovn9DvAa/Mf5+nAg3lOPwO8Nc+GeF4fDXwVz+tzgM/mP8ZvA6/Fc/oZ4K35j/HZwGfxvN4H+G6eDfG8jgO3Asd4TrvAQ4Bd/v0+GvgqntPHAF/Nv99x4OnAcZ7TJeDBwC7Phnj+Phv4LJ7X5wCfzb/fceC3gZfiir8BXhvY5d/vs4HP4nl9DvDZPCfE83ccuBU4xnPaBV4H+Gv+Y7w2V/w2/zFeGvgrntcl4MHALs8J8YJ9NPBVPK9bgZcBdvmf5TjwdOA4z+tjgK/meSFeuFuBB/G8/hp4Gf5n+SvgpXlezwAezPOHeOFeG/gtnr/vBt6H/xm+C3hvnr/XAX6b5w/xL/to4Kt4/r4beB/++xwHvgp4b56/9wG+mxcM8aL5buC9eP7+GngdYJf/WseB3wJemufve4D35oVDvOj+Gngpnr9bgbcB/pr/Gi8N/BZwnOfvd4DX5l+GeNEdB34beCmev13gq4GvAXb5z3Ec+Cjgs3nB/gZ4bWCXfxniX+c48NvAS/GC7QJfDXwO/7E+C/ho4Dgv2O8Abw3s8qJB/Nt8N/BevHC3Aj8N/DTwO/zbvBbw1sBbAw/mhfse4L3510H827038F28aHaBnwZuBX6bKy4Bf80VLw0c44rXBh4MvDVwnBfN+wDfzb8e4t/ntYHvBh7Ef49nAO8N/Db/Noj/GB8NfDZwjP8al4DPBr6afx/Ef5zjwEcDHw0c4z/HJeCrga8Gdvn3Q/zHOw68NfDWwFvxH+NngJ8GfhrY5T8O4j/fWwNvDTwYeC1eNL8D3Ar8NPDT/OdB/Pc4Drw0z+mvgV3+ayH+f0P8/8Y/AnmAQ1C6GKReAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9drccXv8N8D8d/jOPBbwEtzxV8DrwPs8l8L8d/jo4Gv4jl9DPDV/NdC/Pf4beC1eE4/A7w1/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L86xwH3gt4a/59Xho4znPaBf6af5+fBr4H2OVFg/jX+Svgpfmf7a+Bl+FFg3jRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIf51/hp4Kf5n+xvgpXnRIP51jgPvDbw2cJx/u5cCjvOcdoG/4d9uF/ht4LuBXV40iP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKefAd6a/1qI/x4fDXwVz+ljgK/mvxbiv8dx4LeBl+KKvwFeG9jlvxbiv9drc8Vv898D8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4R9tFfQav9bL8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv3HHgpYCXBl4aeDD/O9wK/DXw18DfALs8f4gX7K2B7wKO87/bLvA+wE/zvBDP6zjwXcBb83/LTwNvw3NCPK+vBj6K/5u+Bvhong3xnF4b+C3+b3sd4Le5AvGcfht4LZ7XM4Bb+d/lwcCDeF6/A7w2VyCek3le3wO8N/87fTfwXjwvcQXi2V4a+Cue10OAW/nf6cHA03leLwP8NYB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMSL7qXBr4LeGngp4H3AXb59zkOfBfw1sBfA+8D/DUvOvO8Xgf4bQDxbK8N/BbPS7xoHgz8FXCcZ/tr4HWAXf5tjgO/Bbw0z7YLvAxwKy8a87xeB/htAPFsrw38Fs9LvGheG/gtntdfA68D7PKvcxz4LeCleV6vA/w2LxrzvF4H+G0A8WyvDfwWz0u8aF4a+Cuev78GXgfY5UVzHPgt4KV5/l4G+GteNOZ5vQ7w2wDi2V4b+C2el3jRfTXwUTx/fw28DrDLC3cc+C3gpXn+vgb4aF505nm9DvDbAOLZXhv4LZ6X+Nf5buC9eP7+GngdYJfn7zjwW8BL8/x9D/De/OuY5/U6wG8DiGd7beC3eF4ngF3+db4beC+ev78GXgfY5TkdB34LeGmev+8B3pt/nePARZ7X6wC/DSCe7aWBv+J5vQ7w2/zrfTfwXjx/fw28DrDLFceB3wJemufve4D35l/vtYHf4nk9BLgVQDwn87xeB/ht/m2+G3gvnr+/Bl6HK34LeGmev+8B3pt/m9cGfovnJa5APCfzvN4H+G7+7b4beC+ev7/mipfm+fse4L35t/to4Kt4XuIKxHP6a+CleE5fA3w0/z7fDbwX/zrfA7w3/z5fDXwUz+lvgJfmCsRz+mngrXhOtwIP4d/vu4H34kXzPcB78+/3dODBPKefAd6aKxDP6b2B7+J5PQS4lX+/7wbeixfue4D35t/vwcDTeV7vA3w3VyCe04OBp/O8Pgb4av5jfDfwXjx/3wO8N/8xPhr4Kp7XQ4BbuQLxvP4aeCme018DL8N/nO8G3ovn9D3Ae/Mf56+Al+Y5/Q3w0jwb4nl9NfBRPK/XAX6b/zifDXw0V3w18Nn8x3lt4Ld4Xp8DfDbPhnheLw38Fc/rr4GX4X+HvwJemuf1MsBf82yI5++3gdfieb0P8N38z/bewHfxvH4GeGueE+L5ezDwdJ7XrcDLALv8z3Qc+CvgwTyvhwC38pwQL9h3A+/F8/pp4G34n+mngLfmeX0P8N48L8QL9mDgr4FjPK+vAT6a/1m+Gvgontcl4KWBW3leiBfus4HP4vl7H+C7+Z/hvYHv4vn7HOCzef4Q/7KfBt6K5+9tgJ/mv9dbAz/F8/czwFvzgiH+ZceB3wZeiufvo4Gv4b/HZwGfzfP3N8BrA7u8YIgXzYOBvwaO8fx9N/A+/Nc5DnwV8N48f5eAlwZu5YVDvOheGvgrXrC/Bj4G+G3+c7028FXAS/OCvQzw1/zLEP86bw18N3CMF+y7gc8BbuU/1oOBzwLemxfsEvDewE/zokH867008NPAg3jhvhv4GuCv+fd5aeCjgPfmhfsb4L2Bv+ZFh/i3OQ78NvBS/Mt2gZ8Gfhr4HWCXF+448FrAWwNvDRznX/Y7wFsDu/zrIP59vht4L/71doG/5jm9NHCcf73vAd6bfxvEv99rA58NvBb/tX4H+Gzgt/m3Q/zHeW3gu4EH8Z/rGcB7A7/Nvx/iP957A+8NvBb/sX4H+G7gu/mPg/jPcxx4a+C1gbcGjvGvcwn4aeCngd8GdvmPh/iv89LAceA48NJc8WCuuJUr/hrYBXaBv+Y/H+L/N8T/b4j/3xD/vyH+f+MfATJcB1BRm5XjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tV4LeGngOFe8NLAL3ArcCtwK/A2wy38NxH+u1wbeCnhp4LX51/lr4KeB7wFu5T8H4j/eceC9gI8GHsx/jJ8Gvhv4Gf5jIeA48F3AWwN/DbwP8Nf86x0HPgr4bP7z3Ap8N/A1wC7/fgj4aeCteLZd4GWAW3nRvTXwVcCD+a9xK/A+wG/z74MA87xeB/ht/mXHge8C3pp/nb8Bdnm248BL8a/31cDnALv82yDAPK/XAX6bF+448FvAS/Mv+xngt4G/Bn6bF+w48NLAg4GPBl6Kf9lfA+8D/DX/eggwz+t1gN/mBXtp4LeA47xgzwC+G/hu4Fb+bV4a+GjgrYFjvGC7wOsAf82/DgLM83od4Ld5/l4a+C3gOM/fJeCrgc/mP85x4KOBz+IF2wVeB/hrXnQIMM/rdYDf5nm9N/BdvGA/A3w0cCv/OV4a+GrgtXj+doGXAW7lRYMA87xeB/htntN7A9/FC/Y9wHvzX+OrgY/i+ftr4HWAXf5lCDDP63WA3+bZ3hv4Ll6w9wG+m/9a7w18F8/fdwPvw78MAeZ5vQ7w21zxUcBX84K9D/Dd/Pd4b+C7eP7eBvhpXjgEmOf1OsBvA98FvDfP3yXgvYGf5r/XVwMfxfO6FXgILxwCzPN6HeC9gPfm+bsEvDbw1/zP8NfAS/G8Pgf4bF4wBJjn9cvAG/P8XQJeG/hr/ud4MPDXwDGe0y7wEGCX5w8B5kV3CXht4K/5n+ezgc/ieX0M8NU8fwgwL5q/Ad4auJX/mY4DtwLHeE5/DbwMzx8CzL/sb4DXBnb5n+2zgc/ieT0EuJXnhQDzH+evgfcB/pr/Hg8Gns7z+hjgq3leCDD/sXaBlwFu5b/HXwMvxXP6a+BleF4IMP/xXgf4bf57fDTwVTyvE8AuzwkBR8CC/1gvA/w1/z0eDDyd5/U6wG/znBDwTsAPAeI/xucAn81/L/O8Xgf4bZ4T4oqXBF4XOACewr/dLvDX/PfbBY7xnD4H+GyeE+L/pt8GXovn9DnAZ/OcEP83/TbwWjynnwHemueE+L/pt4HX4jn9DvDaPCfE/02/DbwWz+lzgM/mOSH+b/or4KV5Tp8DfDbPCfF/k3leHwN8Nc8J8X/PSwN/xfN6HeC3eU6IF+448F7AawM/DXwP//N9NPBVPK8TwC7PCfHC/RXw0jzb5wCfzf9svw28Fs/pb4CX5nkhXrDXBn6L53UC2OV/puPARZ7X1wAfzfNCvGCvDfwWz+tzgM/mf6bPBj6L5/UywF/zvBAv3K3Ag3hOtwIP4X+e48DTgeM8p2cAD+b5Q7xwHw18Fc/rY4Cv5n+WzwY+i+f1OcBn8/whXrjjwK3AMZ7TLvAywK38z3AceDpwnOd0CXgwsMvzh/iXfTXwUTyv3wZeh/8Zfgt4bZ7X5wCfzQuG+JcdB24FjvG8Pgb4av57fTbwWTyvS8CDgV1eMMSL5q2Bn+L5ex/gu/nv8d7Ad/H8vQ/w3bxwiBfdTwNvxfP3PsB381/rvYHv4vn7HuC9+ZchXnTHgVuBYzx/Xw18DP81Pgv4bJ6/ZwAvDezyL0P867w08NvAMZ6/3wbeBtjlP8eDge8CXpvn7xLw2sBf86JB/Ou9NPDbwDGev13gq4HP4T/WewFfDRzn+bsEvDbw17zoEP82Lw38NnCMF+xW4KuB7wF2+bc5DrwV8NnAg3nBLgGvDfw1/zqIf7uXBr4beCn+Zd8N/DbwO8CtvHAPBl4LeGvgrfmXPQN4a+Cv+ddD/PscB74aeC9edLvAX/O8jgMvzb/O9wAfDezyb4P4j/HewFcDx/ivcQl4b+Cn+fdB/Mc5Dnw08Fn857kEfDXw1cAu/36I/3gPBt4beG/gQfzHuAR8NfDVwC7/cRD/ud4aeGvgtYEH8a/zDOCngZ8Gfpv/HIj/OseB1wZemmd7ba7YBf4auBW4FfhrYJf/fIj/3xD/vyH+f0P8/4b4/41/BGUgC1BxgD1WAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF5UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgt4a57tVuBWrrgVuBX4a2AX+B3+90D8y34aeCv+dX4b+G3gt4Hf4X8uxL/M/PvsAj8NfA3w1/zPgviX/TXwUvzHuBX4buBrgF3++yH+ZS8N/DZwjP84u8BXA18D7PLfB/GieTDwYOClgePAceClueLBwIP4t9kFvhr4HP57IP7jHAdeG3hr4K2BY7zobgXeB/ht/msh/vO8NPDWwFsDL8WL5qeB9wF2+a+B+K/xYOC9gY8GjvHC3Qq8DfDX/OdD/Nd6MPDZwHvxL/to4Gv4z4X47/Fg4KuBt+KF+27gffjPg/jv9drAZwOvxQv228DbALv8x0P8z/DZwGfxgv018DrALv+xEP9zvDfw1cAxnr+/Bl4H2OU/DuJ/lpcGfhs4xvP318DrALv8x0D8z3Mc+G3gpXj+fht4Hf5jIP5nOg58NfBePH/fDbwP/36I/9m+G3gvnr+PAb6afx/Ei+bBwIN4tt/hv8Zx4LeBl+L5exngr/m3Q/zLXhr4LeA4z2sX+G3gp4HfAW7lP95x4FbgGM/rVuBlgF3+bRD/sr8CXpoXzV8DPw18DbDLf5yXBn4bOMbz+mngbfi3QfzLzL/eLvDVwNcAu/zHeG/gu3j+Xgf4bf71EP+ynwbein+bXeCzga/hP8ZnA5/F87oVeAj/eoh/2XHgq4H34t/ur4G3AW7l3++3gdfieX0O8Nn86yD+9V6bK44Drw28NfAg/mW7wOsAf82/z2sDv8Xz2gUeAuzyokP8x3gw8NbARwMP4oV7H+C7+ff5aeCteF6fA3w2LzrEf7zPBj4aOMYL9jnAZ/Nv92Dg6TyvXeAhwC4vGsR/juPAZwMfxQv2PsB382/33cB78bw+B/hsXjSI/1zvDXwXL9jLAH/Nv82Dgb8GjvGcbgUewosG8Z/vpYHfBo7xvHaBlwFu5d/ms4HP4nm9DPDX/MsQ/zVeGvht4BjP66+Bl+Hf5sHA03le3wO8N/8yxH+dlwb+iufvY4Cv5t/mr4GX4jntAif4lyH+a7038F08r13gIcAu/3pfDXwUz+t1gN/mhUP81/tq4KN4Xp8DfDb/ei8N/BXP63OAz+aFQ/zXOw7cChzjOe0CDwF2+dfbBY7xnH4HeG1eOMR/j88GPovn9TnAZ/Ov993Ae/G8xAuH+O9zK/AgntNfAy/Dv95bAz/F83od4Ld5wRD/Om8FvDTwYOCvgZ8BbuXf5qOBr+J5PQS4lX+d48BFntfbAD/NC4Z40TwY+C7gtXlenw18Dv96DwaezvN6H+C7+dczz+tzgM/mBUO8aP4KeGlesM8BPpt/vVuBB/GcfgZ4a/71bgUexHP6HuC9ecEQ/7LPBj6Lf9lDgFv51/lq4KN4TrcCD+Ff77eB1+I5/Q7w2rxgiH/ZbwOvxb/sY4Cv5l/nrYGf4nmJf73fBl6L5/Q7wGvzgiH+ZeZF8zXAR/Ov89rAb/G8xL/eVwMfxXP6a+BleMEQ/7K/Bl6Kf9nnAJ/Nv85rA7/F83od4Lf51/ls4LN4XuIFQ/zLfhp4K/5lbwP8NP965nm9DvDb/Ot8NvBZPC/xgiH+ZQ8G/ho4xgv2O8Br829jntfrAL/Nv85rA7/F8xIvGOJF897AVwPHeF5/A7w1cCv/Nj8NvBXPdgl4MLDLv86Dgb8GjvFs3wO8Ny8Y4kX3YOCrgQcDLwX8DvDbwGfz73Mc+G7grYC/Ad4b+Gv+bV4a+G7gpYDvAT4a2OUFQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAdU31kFugq/gAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGNElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+V4aOAa8NlccBx4M/DVX3ArcCjwDuJX/Woj/eMeB1wLeGnhr4Dgvur8Gfhv4HuCv+c+H+I9zHPgq4L35j3Er8NnA9/CfB/Hvdxz4KOCjgeP8x7sV+Bjgp/mPh/j3eW3gp4Dj/Of7beBtgF3+4yD+7d4b+C7+dX6H53QceCledLcCbwP8Nf8xEP823wW8Ny/cJeCngZ8GfhvY5QV7MPDWwGsDb8ULtwu8D/DT/Psh/vV+CnhrXrBnAJ8NfDf/NseBzwY+ihfufYDv5t8H8a/zXcB78/xdAr4a+Gpgl3+/BwNfDbwVL9jrAL/Nvx3iRffewHfx/F0CXhv4a/7jfTTwVTx/u8DrAH/Nvw3iRfPawG/x/P0N8NrALv953hr4buAYz+uvgdcBdvnXQ/zLjgNPB47zvP4GeG1gl/98Lw38NnCM5/U9wHvzr4f4l3028Fk8r0vAg4Fd/uu8N/BdPH8PAW7lXwfxwh0Hng4c5zldAl4b+Gv+63028Fk8r98GXod/HcQL993Ae/G8Pgf4bP77/DXwUjyv1wF+mxcd4gU7DlzkeT0DeGlgl/8+rw38Fs/rZ4C35kWHeMHeGvgpntf7AN/Nf7/fBl6L57QLnOBFh3jBvht4L57TJeA4/zO8NfBTPK+3AX6aFw3iBbsIHOc5fQ/w3vzPsQsc4zl9DfDRvGgQz99LA3/F83ob4Kf5n+OngbfiOd0KPIQXDeL5e23gt3heJ4Bd/ud4b+C7eF7iRYN4/j4b+Cyel/if5bWB3+J5iRcN4vn7bOCzeE6/A7w2/7O8NvBbPK+XAf6afxni+ftq4KN4Tr8DvDb/85jn9TrAb/MvQzx/Pw28Fc/pd4DX5n8e87xeB/ht/mWI5++zgc/iOf0O8Nr8z3IcuMjzeh3gt/mXIZ6/zwY+i+f018DL8D/LawO/xfN6CHAr/zLE8/fewHfxvMT/LK8N/BbPS7xoEM/fawO/xfN6CHAr/3N8NPBVPC/xokE8fw8Gns7z+hjgq/mf46+Al+Y5/Q3w0rxoEC/YXwMvxXP6GeCt+Z/hOHCR5/UxwFfzokG8YF8NfBTP6wSwy3+/9wa+i+f1MsBf86JBvGAvDfwVz+trgI/mv9/TgQfznJ4BPJgXHeKFuxV4EM/rIcCt/Pf5bOCzeF5fA3w0LzrEC/fewHfxvH4aeBv+exwHng4c5zldAh4M7PKiQ/zLbgUexPP6GOCr+a/3V8BL87w+B/hs/nUQ/7K3Bn6K5+9tgJ/mv84PAO/K87oEPBjY5V8H8aL5beC1eF67wOsAf81/vp8C3prn72OAr+ZfD/GiOQ78NfAgntcu8DHAd/Of58eAt+f5+x7gvfm3QbzoXhr4beAYz99nA5/Df6zjwG8BL80L9j7Ad/Nvg/jXeWvgp3jB/hr4GOC3+ff7LOCjgeP8y94H+G7+9RD/eu8NfBcv3G8DXw38DP86x4G3Aj4beDD/Ou8DfDf/Ooh/m9cGfho4xgu3C/w28NPArVzxO1xxHHgprnhp4L2Bl+bf532A7+ZFh/i3e2ngu4GX4r/O9wC/DXwXL9j7AN/Niwbx73Mc+GrgvfjPdQn4bOCrueK9ge/iBXsf4Lv5lyH+YzwY+G7gtfiPdQn4auCrgV2e03sD38UL9j7Ad/PCIf5jvTbw0cBrA8f4t3sG8NPAZwO7vGDvDXwXL9j7AN/NC4b4z/PWwGsDbw08iH/Z3wDfDfw28Ne86N4b+C5esPcBvpvnD/Ff67V5TrcCt/Lv997Ad/GCvQ/w3TwvxP8d7w18Fy/YywB/zXNC/N/y3sB38fx9DfDRPCfE/z3vDXwXz+trgI/mOSH+b3pv4Lt4Ti8D/DXPCfF/10sD780V3w38Nc8L8f8b4v83xP9viP/fEP+/8Y8EA+VBcfLfHwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+JF82DgQfzv8gzgVl44xAt2HPgq4K2B4/zvtAv8NPAxwC7PC/H8vTXwXcBx/m/YBd4H+GmeE+J5vTXwU/zf9DbAT/NsiOd0HHg6cJz/m3aBhwC7XIF4Tt8NvBf/t30P8N5cgXhOF4HjPKe/AT4a+G3+d3lt4KuBl+I57QInuALxbA8Gns7zeh3gt/nf6bWB3+J5PQS4FUA822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ5++lgffiiu8B/pr/mczzeh3gtwHEs7028Fs8L/G8Xhr4K57TywB/zf885nm9DvDbAOLZXhv4LZ6XeF5fDXwUz+lrgI/mfx7zvF4H+G0A8WyvDfwWz0s8r68GPorn9DXAR/M/j3lerwP8NoB4ttcGfovnJZ7XSwN/xXN6GeCv+Z/HPK/XAX4bQDzbawO/xfMSz99LA+/NFd8N/DX/M5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzPRh4Os/rdYDf5n+n1wZ+i+f1EOBWAPGcdoFjPKe/Bj4G+G3+d3lt4KuAl+Y5XQKOcwXiOX038F783/Y9wHtzBeI5HQduBY7xf9Ml4MHALlcgntdbAz/F/01vA/w0z4Z4/t4a+G7gGP83XALeG/hpnhPiBTsOfDXw1sAx/ne6BPw08NHALs8L8aJ5MPBg/ne5FbiVFw7x/xvi/zfE/2+I/98QL5oHAw/if5dnALfywiFesOPAVwFvDRznf6dd4KeBjwF2eV6I5++tge8CjvN/wy7wPsBP85wQz+utgZ/i/6a3AX6aZ0M8p+PA04Hj/N+0CzwE2OUKxHP6buC9+L/te4D35grEc7oIHOc5/Q3w0cBv87/LawNfDbwUz2kXOMEViGd7MPB0ntfrAL/N/06vDfwWz+shwK0A4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPSzx/Lw28F1d8D/DX/M9kntfrAL8NIJ7ttYHf4nmJ5/XSwF/xnF4G+Gv+5zHP63WA3wYQz/bawG/xvMTz+mrgo3hOXwN8NP/zmOf1OsBvA4hne23gt3he4nl9NfBRPKevAT6a/3nM83od4LcBxLO9NvBbPC/xvF4a+Cue08sAf83/POZ5vQ7w2wDi2V4b+C2el3j+Xhp4b674buCv+Z/JPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu3BwNN5Xq8D/Db/O7028Fs8r4cAtwKI57QLHOM5/TXwMcBv87/LawNfBbw0z+kScJwrEM/pu4H34v+27wHemysQz+k4cCtwjP+bLgEPBna5AvG83hr4Kf5vehvgp3k2xPP31sB3A8f4v+ES8N7AT/OcEC/YceCrgbcGjvG/0yXgp4GPBnZ5XogXzYOBB/O/y63ArbxwiP/fEP+/8Y8tq8RBBi9QkAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMooDjwHcBb82/zk8D7wPs8r8XAn4aeCv+bX4GeGv+90KA+fcR/3sh4K+Bl+Lf7nWA3+Z/JwS8NPDbwDH+bT4G+Gr+d0Jc8WDgwfzLvhp4KZ7T9wDvzf9OiH+d7wbei+f018DL8L8T4l/no4Gv4nmJ/50Q/zqvDfwWz+t1gN/mfx/Ev555Xh8DfDX/+yD+9f4aeCme0/cA783/Poh/ve8G3ovn9NfAy/C/D+Jf76OBr+J5if99EP96rw38Fs/rr4Fd/nfYBX4a+B7xb2P+b/gc8W/z18BL8b/frvi3+W7gvfjf75L4t/lo4Kv43+93xL/NawO/xfP6G2CX/5lei+f1NeLfzjyvjwG+mv95jgMXeV7vI/7t/hp4KZ7T9wDvzf88rw38Fs/rZcS/3XcD78Vz+mvgZfif56OBr+J5SfzbfTTwVTwv8T/PdwPvxXP6HeC1xb/dawO/xfN6HeC3+Z/lr4CX5jl9D/De4t/HPK+PAb6a/1nM8/oY4KvFv89fAy/Fc/oe4L35n+O1gd/ieb0O8Nvi3+e7gffiOf018DL8z/HewHfxvAQg/n0+Gvgqnpf4n+OrgY/iOf0N8NIA4t/ntYHf4nm9DvDb/M/w28Br8Zx+BnhrAPHvZ57XxwBfzf8M5nl9DvDZAOLf76+Bl+I5fQ/w3vz3e2ngr3herwP8NoD49/tu4L14Tn8NvAz//d4a+Cme1wlgF0D8+3008FU8L/Hf77OBz+I5PQN4MFcg/v1eG/gtntfrAL/Nf6/fBl6L5/Q7wGtzBeI/hnleHwN8Nf+9LgLHeU6fA3w2VyD+Y/w18FI8p+8B3pv/Pg8Gns7zehvgp7kC8R/ju4H34jn9NfAy/Pd5beC3eF4PAW7lCsR/jI8GvornJf77fDbwWTynS8Bxng3xH+O1gd/ieX0M8NX893g68GCe0+8Ar82zIf7j3Ao8iOe0C3w08DPALv81Xhr4LOCteV4fA3w1z4b4j/PVwEfxP9sJYJdnQ/zHeTDw18Ax/mf6GuCjeU6I/1ivDfwW//P8DfDSPC/Ef7z3Br4aOMb/DL8DvDWwy/NC/Oc4Dnw28FH893kG8NHAT/OCIf7zPRh4MP+1fpsXDeL/N8T/b4j/3xD/vyH+c70WV/wO/3VeGjgGPAO4lRcO8R/vOPBdwFvznH4aeB9gl/8cHwV8NnCcZ/tr4H2Av+b5Q/zHOg78FfBgnr+/Bl4H2OU/1ncB780L9jrAb/O8EP+xfgp4a164nwbehv84bw38FC/cLvAQYJfnhPiPZV404j/ObwOvxb/sfYDv5jkh/uO8NvBbvGheB/ht/mNcBI7zL/sa4KN5Toj/OK8N/BYvmtcBfpv/GLvAMf5lXwN8NM8J8R/LvGjEf5zfBl6Lf9n7AN/Nc0L8x/pp4K144b4HeG/+47w18FO8cJeABwO7PCfEf6zjwG8DL8Xz9zfAawO7/Mf6buC9eMFeB/htnhfiP95x4LuBt+I5/Qzw3sAu/zk+Gvhs4BjP9jfAewN/zfOH+M/12lzx2/zXeWngOHArcCsvHOL/N8T/b/wj3dq3gCTAr7UAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH+0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x3HgpXhOfwPs8l8L8Z/vrYC3Bh4MvDYvmt8GbgV+GvgZ/vMg/uM9GHgr4LWBt+Y/xk8Dvw18D7DLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jfBTw2cBx/mvsAp8NfA3/Poh/n7cGvgp4MP89bgXeB/ht/m0Q/3YfBXw1L5pLwE8DtwK/zRW7wF9zxUsDx7nitYEHA28NHONF8z7Ad/Ovh/i3+S7gvXnhngH8NPDTwG/zb/PawFsDbw08iBfuu4H34V8H8a/zIOAPgBt5wS4BXw18Nv+xPhv4aOAYL9hvA28D7PKiQbzoHgQ8Hljw/F0Cvhr4amCX/xzHgY8GPho4xvP318DrALv8yxAvujuAG3n+ngG8NfDX/Nd4aeCngQfx/P028Dr8yxAvmu8C3psX7MOAb+S/1nHgt4GX4vn7buB9eOEQ/7KPBr6Kf9n7AN/Nf73vBt6L5+99gO/mBUO8cG8N/BQvuvcBvpv/et8NvBfP3+sAv83zh3jhng48mH+d9wG+m/96fw28FM/rVuAhPH+IF+yjga/ieT0D+FLgG3jB3gf4bv5rHQf+GngQz+tjgK/meSGev+PA04HjPKdLwGsDfw28N/BdvGDvA3w3/7VeGvht4BjPaRd4CLDLc0I8f58NfBbP63OAz+bZ3hv4Ll6w9wG+m/9anw18Fs/rc4DP5jkhnteDgb8CjvOcLgEPBnZ5Tu8NfBcv2PsA381/nePArcAxntMu8BBgl2dDPK+PBr6K5/U5wGfz/L038F28YO8DfDf/dT4b+Cye1/sA382zIZ7XTwNvxXN6BvBgXrj3Br6LF+x9gO/mv86twIN4Tj8DvDXPhnhOx4GLPK+vAT6af9l7A9/FC/Y+wHfzX+OrgY/ieYlnQzyntwZ+iuf1OsBv86J5b+C7eMHeB/hu/vO9NvBbPK+3AX6aKxDP6buB9+I5XQKO86/z3sB38YK9D/Dd/OfbBY7xnL4HeG+uQDyn3wZei+f0PcB786/33sB38YK9D/Dd/Of6buC9eE6/A7w2VyCek3lenwN8Nv827w18Fy/Y+wDfzX+ezwY+i+clrkA8J/O8Xgf4bf7t3hv4Ll6w9wG+m/8crw38Fs9LXIF4tuPARZ7X6wC/zb/PewPfxQv2PsB38x/vtYHf4nmdAHYBxLO9NvBbPK+XAf6af7/3Br6LF+x9gO/mP9ZLA3/F83od4LcBxLO9NvBbPC/xH+e9ge/iBXsf4Lv5j2We1+sAvw0gnu21gd/ieYn/WO8NfBcv2PsA381/HPO8Xgf4bQDxbK8N/BbP6yHArfzHem/gu3jB3gf4bv79Xhr4K57X6wC/DSCe7Thwkef1OsBv8x/vvYHv4gV7H+C7+fd5beC3eF4ngF0A8ZzM83od4Lf5z/HewHfxgr0P8N3827028Fs8L3EF4jmZ5/UxwFfzn+e9ge/iBXsf4Lv5t/ls4LN4XuIKxHP6beC1eE7fA7w3/7neG/guXrD3Ab6bf73vBt6L5/Q7wGtzBeI5fTfwXjynXeAE//neG/guXrD3Ab6bf52LwHGe0/cA780ViOf01sBP8bxeB/ht/vO9N/BdvGDvA3w3L5rXBn6L5/U2wE9zBeI5HQcu8ry+Bvho/mu8N/BdvGDvA3w3/7KvBj6K5yWeDfG8fhp4K57TrcBD+K/z3sB38YK9D/DdvHBPBx7Mc/oZ4K15NsTz+mjgq3henwN8Nv913hv4Ll6w9wG+m+fvs4HP4nm9D/DdPBvieR0HbgWO8Zx2gYcAu/zXeW/gu3jB3gf4bp7TceDpwHGe0yXgwcAuz4Z4/j4b+Cye1+cAn81/rfcGvosX7H2A7+bZPhv4LJ7X5wCfzXNCPH/HgVuBYzynXeB1gL/mv9Z7A9/FC/Y+wHcDLw38Fc/rEvBgYJfnhHjBPhr4Kp7XrcDLALv813pv4Lt4wT4M+ALgOM/rY4Cv5nkhXrhbgQfxvP4aeBn+67038F386zwDeDDPH+KFe23gt3j+vht4H/7rvTfwXbzoXgf4bZ4/xL/so4Gv4vn7buB9+K/33sB38S97H+C7ecEQL5rvBt6L5++vgdcBdvmv9aHAN/CCfQ/w3rxwiBfdXwMvxfN3K/A2wF/zX+Olgd8CjvP8PQF4DP8yxIvuOPDbwEvx/O0CXw18DbDLf47jwEcBn80LtgQeAzyDfxniX+c48NvAS/GC7QJfDXwO/7E+C/ho4Dgv2BOANwaewYsG8W/z3cB78cLdCvw08NPA7/Bv81rAWwNvDTyYF+57gPfmXwfxb/fewHfxotkFfhq4FfhtrrgE/DVXvDRwjCteG3gw8NbAcV407wN8N/96iH+f1wa+G3gQ/z2eAbw38Nv82yD+Y3w08NnAMf5rXAI+G/hq/n0Q/3GOAx8NfDRwjP8cl4CvBr4a2OXfD/Ef7zjw1sBbA2/Ff4yfAX4a+Glgl/84iP98bw28NfBg4LV40fwOcCvw08BP858H8d/jOPDSPKe/Bnb5r4X4/w3x/xv/CCfMPVACAl4iAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xoOBB/Ff6xnArfz7IP7tjgNfBbw1cJz/HrvATwMfA+zyr4f4t3lr4LuA4/zPsAu8D/DT/Osg/vXeGvgp/md6G+CnedEh/nWOA08HjvM/0y7wEGCXFw3iX+e7gffif7bvAd6bFw3iX+cicJzn9DfARwO/zX+t1wa+GngpntMucIIXDeJF92Dg6Tyv1wF+m/8erw38Fs/rIcCt/MsQL7rXBn6L5yX+e5nn9TrAb/MvQ7zoXhv4LZ6X+O9lntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2el/jvZZ7X6wC/zb8M8aJ7beC3eF7iv5d5Xq8D/Db/MsSL7rWB3+J5if9e5nm9DvDb/MsQL7rXBn6L5yX+e5nn9TrAb/MvQ7zoXhv4LZ6X+O9lntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2el/jvZZ7X6wC/zb8M8aJ7beC3eF7iv5d5Xq8D/Db/MsSL7rWB3+J5if9e5nm9DvDb/MsQL7rXBn6L5yX+e5nn9TrAb/MvQ7zoXhv4LZ6X+O9lntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2el/jvZZ7X6wC/zb8M8aJ7beC3eF7iv5d5Xq8D/Db/MsSL7rWB3+J5iX+blwbeiyu+B/hr/m3M83od4Lf5lyFedK8N/BbPS/zrvTTwVzynlwH+mn8987xeB/ht/mWIF91rA7/F8xL/el8NfBTP6WuAj+Zfzzyv1wF+m38Z4kX32sBv8bzEv95XAx/Fc/oa4KP51zPP63WA3+ZfhnjRvTbwWzwv8a/30sBf8ZxeBvhr/vXM83od4Lf5lyFedK8N/BbPS/zbvDTw3lzx3cBf829jntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOgeDDyd5/U6wG/z3+O1gd/ieT0EuJV/GeJfZxc4xnP6a+BjgN/mv9ZrA18FvDTP6RJwnBcN4l/nu4H34n+27wHemxcN4l/nOHArcIz/mS4BDwZ2edEg/vXeGvgp/md6G+CnedEh/m3eGvhu4Bj/M1wC3hv4af51EP92x4GvBt4aOMZ/j0vATwMfDezyr4f4j/Fg4MH817oVuJV/H8T/b4j/3/hHwdjCQXRZhpcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIRElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x3HgpXhOfwPs8l8L8Z/vrYC3Bh4MvDYvmt8GbgV+GvgZ/vMg/uM9GHgr4LWBt+Y/xk8Dvw18D7DLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jfBTw2cBx/mvsAp8NfA3/Poh/n7cGvgp4MP89bgXeB/ht/m0Q/3YfBXw1L5pLwE8DtwK/zRW7wF9zxUsDx7nitYEHA28NHONF8z7Ad/Ovh/i3+S7gvXnhngH8NPDTwG/zb/PawFsDbw08iBfuu4H34V8H8a9zHPgt4KV5wS4BXw18Nv+xPhv4aOAYL9hvA28D7PKiQbzojgO/Bbw0z98l4KuBrwZ2+c9xHPho4KOBYzx/fw28DrDLvwzxovsr4KV5/p4BvDXw1/zXeGngp4EH8fz9NvA6/MsQL5rvAt6b5+9vgNcGdvmvdRz4beCleP6+G3gfXjjEv+yjga/i+fse4L357/XdwHvx/L0P8N28YIgX7q2Bn+L5+x7gvfmf4buB9+L5ex3gt3n+EC/c04EH87z+Bnhp/mf5a+CleF63Ag/h+UO8YB8NfBXP6xnASwO7/M9yHPhr4EE8r48BvprnhXj+jgNPB47znC4Brw38Nf8zvTTw28AxntMu8BBgl+eEeP4+G/gsntfnAJ/N/2yfDXwWz+tzgM/mOSGe14OBvwKO85wuAQ8Gdvmf7ThwK3CM57QLPATY5dkQz+ujga/ieX0O8Nm86F4aeC/gpfn3+Wvge4C/5kX32cBn8bzeB/hung3xvH4aeCue0zOAB/Oie2ngr/iP9TLAX/OiuxV4EM/pZ4C35tkQz+k4cJHn9TXAR/Oi+2rgo/iP9TXAR/Oi+2rgo3he4tkQz+mtgZ/ieb0O8Nu86H4beC3+Y/0O8Nq86F4b+C2e19sAP80ViOf03cB78ZwuAcf51/lt4LX4j/U7wGvzr7MLHOM5fQ/w3lyBeE6/DbwWz+l7gPfmX+e3gdfiOV0C/poXzUsDx3hOvwO8Nv863w28F8/pd4DX5grEczLP63OAz+Zf57eB1+I5/Q7w2rxofht4LZ7T7wCvzb/OZwOfxfMSVyCek3lerwP8Nv86vw28Fs/pd4DX5kXz28Br8Zx+B3ht/nVeG/gtnpe4AvFsx4GLPK/XAX6bf53fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/Oq8N/BbP6wSwCyCe7bWB3+J5vQzw1/zr/DbwWjyn3wFemxfNbwOvxXP6HeC1+dd5aeCveF6vA/w2gHi21wZ+i+cl/vV+G3gtntPvAK/Ni+a3gdfiOf0O8Nr865nn9TrAbwOIZ3tt4Ld4XuJf77eB1+I5/Q7w2rxofht4LZ7T7wCvzb+eeV6vA/w2gHi21wZ+i+f1EOBW/nV+G3gtntPvAK/Ni+a3gdfiOf0O8Nr867w08Fc8r9cBfhtAPNtx4CLP63WA3+Zf57eB1+I5/Q7w2rxofht4LZ7T7wCvzb/OawO/xfM6AewCiOdkntfrAL/Nv85vA6/Fc/od4LV50fw28Fo8p98BXpt/ndcGfovnJa5APCfzvD4G+Gr+dX4beC2e0+8Ar82L5reB1+I5/Q7w2vzrfDbwWTwvcQXiOf028Fo8p+8B3pt/nd8GXovntAv8NS+alwaO85x+B3ht/nW+G3gvntPvAK/NFYjn9N3Ae/GcdoET/Ov8NvBa/Mf6HeC1+de5CBznOX0P8N5cgXhObw38FM/rdYDf5kX328Br8R/rd4DX5kX32sBv8bzeBvhprkA8p+PARZ7X1wAfzYvuq4GP4j/W1wAfzYvuq4GP4nmJZ0M8r58G3orndCvwEF50Lw38Ff+xXgb4a150TwcezHP6GeCteTbE8/po4Kt4Xp8DfDYvupcG3ht4af59/hr4buCvedF9NvBZPK/3Ab6bZ0M8r+PArcAxntMu8BBgl//ZjgNPB47znC4BDwZ2eTbE8/fZwGfxvD4H+Gz+Z/ts4LN4Xp8DfDbPCfH8HQduBY7xnHaB1wH+mv+ZXhr4K57XJeDBwC7PCfGCfTTwVTyvW4GXAXb5n+U48HTgOM/rY4Cv5nkhXrhbgQfxvP4aeBn+Z/kr4KV5Xs8AHszzh3jhXhv4LZ6/7wbeh/8Zvgt4b56/1wF+m+cP8S/7aOCreP6+G3gf/vscB74KeG+ev/cBvpsXDPGi+W7gvXj+/hp4HWCX/1rHgd8CXprn73uA9+aFQ7zo/hp4KZ6/W4G3Af6a/xovDfwWcJzn73eA1+ZfhnjRHQd+G3gpnr9d4KuBrwF2+c9xHPgo4LN5wf4GeG1gl38Z4l/nOPDbwEvxgu0CXw18Dv+xPgv4aOA4L9jvAG8N7PKiQfzbfDfwXrxwtwI/Dfw08Dv827wW8NbAWwMP5oX7HuC9+ddB/Nu9N/BdvGh2gZ8GbgV+mysuAX/NFS8NHOOK1wYeDLw1cJwXzfsA382/HuLf57WB7wYexH+PZwDvDfw2/zaI/xgfDXw2cIz/GpeAzwa+mn8fxH+c48BHAx8NHOM/xyXgq4GvBnb590P8xzsOvDXw1sBb8R/jZ4CfBn4a2OU/DuI/31sDbw08GHgtXjS/A9wK/DTw0/znQfz3OA68NM/pr4Fd/msh/n9D/P/GPwKmtEpQPXoWQgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHbUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjv81nAR3PFVwOfw389xH+P9wa+i+f0NsBP818L8d/ju4H34jl9D/De/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRAvutcCHgz8DnAr/z5fDXwUz+lrgI/m3+fBwGsBtwK/w78M8aL5KeCtuWIX+Bjgu/m3e2ngr3hOLwP8Nf927w18FXCcK34aeBteOMS/7LWB3+J5vQ/w3fzbvTTw3lzx3cBf82/33sB38bxeB/htXjDEv+y9ge/i+Xsf4Lv57/XewHfx/L0P8N28YIh/2YOBvwaO8fy9D/Dd/Pd4b+C7eP4uAS8N3MoLhnjRvDfwXbxgLwP8Nf+1Xhr4K16w9wG+mxcO8aJ7b+C7eP6+Bvho/mt9N/BePH/vA3w3/zLEv857A9/F8/oe4L35r/XdwHvxvN4H+G5eNIh/vfcGvovn9DLAX/PCHQdeC3hr4MHAg4EHc8WtwK3ArcBPA78D7PLCvTbwWzyn9wG+mxcd4t/mpYH3Bo4DXw38NS/Yg4HPAt6bf53vBj4HuJUX7LWB9wZ2ge8G/pp/HcR/nuPAZwEfzb/PZwNfA+zyHw/xn+M48FvAS/Mf46+B1wF2+Y+F+I/30sBvAcf5j7ULvA7w1/zHQfzHOg48HTjOC3YJ+GngVuBWrngw8GDgrYFjvGC7wEOAXf5jIP7jHAd+C3hpnr9nAJ8NfDcv3HsDnw08iOfvr4HXAXb590P8x/lq4KN4/r4HeG/+db4beC+ev88BPpt/P8R/jAcDT+f5ex/gu/m3eW/gu3j+HgLcyr8P4j/GdwPvxfP6HuC9+ff5buC9eF7fA7w3/z6If7/jwEWe1zOAB/Mf41bgQTyvE8Au/3aIf7+3Bn6K5/U+wHfzH+O9ge/ieb0N8NP82yH+/b4beC+e0yXgOP+xdoFjPKfvAd6bfzvEFW8NvBXwYJ7XXwPfA/w1z99vA6/Fc/oe4L35j/XdwHvxnH4HeG2ev5cG3gt4aZ7XrcDPAD8t4LOBz+Jf9jLAX/O8ng48mOf0OcBn8x/rs4HP4jn9NfAyPK+XBv6Kf9nnCLgIHOdf9jXAR/O8zPN6H+C7+Y/13sB38bzE8/pq4KP4l+0K2AWO8S/7GuCjeV7meb0P8N38x3pv4Lt4XuJ5fTXwUfzLniHgs4HP4l/2MsBf87xuBR7Ec/oc4LP5j/XZwGfxnP4GeGme10sDf8W/7HPEFW8NvDXwYJ7XXwPfDfw1z99vA6/Fc/oe4L35j/XdwHvxnH4HeG2ev5cG3ht4aZ7XrcBPAz8t/v2+G3gvntMucIL/WBeB4zyn7wHem387xL/fWwM/xfN6H+C7+Y/x3sB38bzeBvhp/u0Q/37HgYs8r1uBh/Af4+nAg3leJ4Bd/u0Q/zG+G3gvntd3A+/Dv893Ae/N8/oe4L3590H8x3gw8HSev/cBvpt/m/cGvovn7yHArfz7IP7jfDbwWTx/3w28D/863wW8N8/f5wCfzb8f4j/OceC3gZfi+bsV+Gzge3jh3gv4bODBPH9/A7w2sMu/H+I/1nHgVuAYL9gu8NPArcCtXPFg4MHAWwPHecEuAQ8GdvmPgfiP99LAbwPH+I91CXht4K/5j4P4z3Ec+G3gpfiP8TfAawO7/MdC/Oc5Dnw08Fn8+3wO8NXALv/xEP82Lw18FFd8D/DbvGAPBj4beC/+db4H+GzgVl6w1wbeiyu+Bvhr/nUQ/3rvDXwXz+llgL/mhTsOvDbw1sCDgePAS3HF3wC7wK3ATwO/Dezywr028Fs8p/cBvpsXHeJf572B7+J5fQ/w3vzX+m7gvXhe7wN8Ny8axIvuvYHv4vn7GuCj+a/13cB78fy9D/Dd/MsQL5r3Br6LF+xlgL/mv9ZLA3/FC/Y+wHfzwiH+ZQ8G/go4zvP3PsB389/jvYHv4vnbBV4GuJUXDPEve2/gu3j+3gf4bv57vTfwXTx/7wN8Ny8Y4l/22sBv8bzeB/hu/u1eGngvrvge4K/5t3tv4Lt4Xq8D/DYvGOJF89PAW3HFJeCjge/m3+6lgb/iOb0M8Nf827038NXAMa74GeCteeEQL7rXBh4M/DZwK/8+Xw18FM/pa4CP5t/nwcBrA7cCv82/DPHf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4buC9eE7fA7w3/7UQ/z3eGvgpntPbAD/Nfy3Ef5/PBj6aK74a+Gz+6/GPLFogHxkAjfAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf57X4n+F3+LdB/Ot9FvDewIP5n+VW4KuBr+FFh3jRPRj4LuC1+Z/tt4H3AW7lX4Z40RwHng4c53+HXeAhwC4vHOJF81PAW/O/y08Db8MLh/iXvTXwU/zv9DbAT/OCIf5lXw18FM/pEvDRwHfzP8N7A9/F8/oa4KN5wRD/st8GXovn9DXAR/M/y1cDH8Vz+h3gtXnBEP8y87zeBvhp/md5b+C7eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpt/n5cG3osrvgf4a/59Xhv4LZ6XeMEQ/zLzvF4H+G3+7V4a+Cue08sAf82/3WsDv8XzEi8Y4l9mntfrAL/Nv91XAx/Fc/oa4KP5t3tt4Ld4XuIFQ/zLzPN6HeC3+bf7auCjeE5fA3w0/3avDfwWz0u8YIh/mXlerwP8Nv92Lw38Fc/pZYC/5t/utYHf4nmJFwzxLzPP63WA3+bf56WB9+aK7wb+mn+f1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzehvgp/mf5b2B7+J5iRcM8S/7beC1eE5fA3w0/7N8NfBRPKffAV6bFwzxL/tq4KN4TrvARwM/A+zy3+s48FbAd/O8vgb4aF4wxL/srYGf4n+ntwF+mhcM8aL5aeCt+N/lZ4C35oVDvGiOA7cCx/jf4RLwYGCXFw7xonsw8N3Aa/E/2+8A7w3cyr8M8a/30cBHAw/if5ZnAF8NfDUvOsS/z2vzP8Nv82+D+P8N8f8b/whmT79BYbkO6wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfNg4EHAJeCv+b8D8S97aeC3gONc8dXAx/B/A+Jf9lfAS/Ocvht4H/73Q/zLzPP33cD78L8b4l/208Bb8fx9N/A+/O+F+JcdB34beCmev+8G3of/nRAvmuPAbwMvxfP33cD78L8P4kV3HPht4KV4/r4beB/+d0H86xwHfht4KZ6/7wbeh/89EP96x4HfBl6K5++7gffhfwfEv81x4LeBl+L5+27gffifD/Fvdxz4beCleP6+G3gf/mdD/PscB34beCmev+8G3of/uRD/fseB3wZeiufvu4H34X8mxH+M48BvAy/F8/fdwPvwPw/iP85x4LeBl+L5+27gffifBfEf6zjw28BL8fx9N/A+/M+B+I93HPht4KV4/r4G+Gj+Z0D85zgO/DbwUjx/LwP8Nf/9EP95Phr4Kp6/1wF+m/9+iP8c7w18F8/fJeDBwC7//RD/8d4b+C5esLcBfpr/GRD/sd4b+C5esPcBvpv/ORD/cd4b+C5esPcBvpv/WRD/Md4b+C5esPcBvpv/eRD/fu8NfBcv2PsA383/TIh/n/cGvosX7H2A7+Z/LsS/3XsD38UL9j7Ad/M/G+Lf5r2B7+IFex/gu/mfD/Gv997Ad/GCvQ/w3fzvgPjXeW/gu3jB3gf4bv73QLzo3hv4Ll6w9wG+m/9dEC+a9wa+ixfsfYDv5n8fxL/stYHf4gV7H+C7+d8J8S/7K+Clef7eB/hu/vdC/MvM8/c+wHfzvxviX/bTwFvxnN4H+G7+90P8y44D3w28FfA3wEcDv83/DYj/Pp8FfDRwnP8cPw28D7DLC4b47/HewHfxn+9ngLfmBUP89/hu4L34ryFeMMR/j48Gvor/fH8DvDQvGOK/x3Hgt4GX4j/PJeC1gb/mBUP893pt/vP8NbDLC4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wiO11tBhPrMNAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG7UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/xoOBB/Gv8wzgVv5zIf7zHAe+Cnhr4Dj/NrvATwMfA+zyHw/xn+Otge8CjvMfYxd4H+Cn+Y+F+I/31sBP8Z/jbYCf5j8O4j/WceDpwHH+c+wCDwF2+Y+B+I/13cB78Z/re4D35j8G4j/WReA4z+lvgI8Gfpt/ndcGvhp4KZ7TLnCC/xiI/zgPBp7O83od4Lf5t3lt4Ld4Xg8BbuXfD/Ef57WB3+J5iX8f87xeB/ht/v0Q/3FeG/gtnpf49zHP63WA3+bfD/Ef57WB3+J5iX8f87xeB/ht/v0QL9hLA+8FvDTPVoHrgC3gHHCOZzsOvDTP67f593ltntdfA7s822ngNHAA3ANMPNtfA98D/DXPC/H8vTTwV/zf8jLAX/OcEM/fVwMfxf8tXwN8NM8J8fx9NfBR/N/yNcBH85wQz99LA3/F/y0vA/w1zwnxgr008N7AS3PFMeCleV6/w7NV4FpgDuwC5/mPcQo4DqyAe4GJZ3stntdfA5e44q+B7wb+mueFeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v8yx4MPAi4BPw1L9xLA8eAZwC38i8zz+t1gN/mX4Z40b028Fs8L/HCvTTwW8Bxrvgc4LN5/j4b+Cyu2AVeB/hrXjjzvF4H+G3+ZYgX3WsDv8XzEi/cXwEvzXN6GeCveU4vDfwVz+mvgZfhhTPP63WA3+ZfhnjRvTbwWzwv8cKZ5/U6wG/znF4b+C2el3jhzPN6HeC3+ZchXnSvDfwWz0u8cOZ5vQ7w2zyn1wZ+i+clXjjzvF4H+G3+ZYgX3WsDv8XzEi+ceV6vA/w2z+m1gd/ieYkXzjyv1wF+m38Z4kX32sBv8bzEC2ee1+sAv81zem3gt3he4oUzz+t1gN/mX4Z40b028Fs8L/HCmef1OsBv85xeG/gtnpd44czzeh3gt/mXIV50rw38Fs9LvHDmeb0O8Ns8p9cGfovnJV4487xeB/ht/mWIF91rA7/F8xIvnHlerwP8Ns/ptYHf4nmJF848r9cBfpt/GeJF99rAb/G8xAtnntfrAL/Nc3pt4Ld4XuKFM8/rdYDf5l+GeNG9NvBbPC/xwpnn9TrAb/OcXhv4LZ6XeOHM83od4Lf5lyFedK8N/BbPS7xw5nm9DvDbPKfXBn6L5yVeOPO8Xgf4bf5liBfdawO/xfMSL5x5Xq8D/DbP6bWB3+J5iRfOPK/XAX6bfxniRffawG/xvMQLZ57X6wC/zXN6beC3eF7ihTPP63WA3+ZfhnjRvTbwWzwv8cKZ5/U6wG/znF4b+C2el3jhzPN6HeC3+ZchXnSvDfwWz0u8cOZ5vQ7w2zyn1wZ+i+clXjjzvF4H+G3+ZYgX3WsDv8XzEi+ceV6vA/w2z+m1gd/ieYkXzjyv1wF+m38Z4kX32sBv8bzEC2ee1+sAv81zem3gt3he4oUzz+t1gN/mX4Z40b028Fs8L/HCmef1OsBv85xeG/gtnpd44czzeh3gt/mXIV50rw38Fs9LvHA/DbwVz3YJeDCwy3N6MPDXwDGe7XuA9+aFM8/rdYDf5l+GeNG9NvBbPC/xwh0Hvhp4L+BvgPcG/prn76WB7wZeCvge4KOBXV4487xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRPRh4Os/rdYDf5r/HawO/xfN6CHAr/zLEv84ucIzn9NfAxwC/zX+t1wa+CnhpntMl4DgvGsS/zncD78X/bN8DvDcvGsS/znHgVuAY/zNdAh4M7PKiQfzrvTXwU/zP9DbAT/OiQ/zbvDXw3cAx/me4BLw38NP86yD+7Y4DXw28NXCM/x6XgJ8GPhrY5V8P8R/jwcCD+a91K3Ar/z6I/98Q/7/xjw1oDVAbluiAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/xoOBB/Gv8wzgVv5zIf7zHAe+Cnhr4Dj/NrvATwMfA+zyHw/xn+Otge8CjvMfYxd4H+Cn+Y+F+I/31sBP8Z/jbYCf5j8O4j/WceDpwHH+c+wCDwF2+Y+B+I/13cB78Z/re4D35j8G4j/WReA4z+lvgI8Gfpt/ndcGvhp4KZ7TLnCC/xiI/zgPBp7O83od4Lf5t3lt4Ld4Xg8BbuXfD/Ef57WB3+J5iX8f87xeB/ht/v0Q/3FeG/gtnpf49zHP63WA3+bfD/Ef57WB3+J5iX8f87xeB/ht/v0QL9hLA+8FvDTPVoHrgC3gHHCOZzsOvDTP67f593ltntdfA7s822ngNHAA3ANMPNtfA98D/DXPC/H8vTTwV/zf8jLAX/OcEM/fVwMfxf8tXwN8NM8J8fx9NfBR/N/yNcBH85wQz99LA3/F/y0vA/w1zwnxgr008N7AS3PFMeCleV6/w7NV4FpgDuwC5/mPcQo4DqyAe4GJZ3stntdfA5e44q+B7wb+mueFeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedA8Gns7zeh3gt/nv8drAb/G8HgLcyr8M8a+zCxzjOf018DHAb/Nf67WBrwJemud0CTjOiwbxr/PdwHvxP9v3AO/Niwbxr3McuBU4xv9Ml4AHA7u8aBD/em8N/BT/M70N8NO86BD/Nm8NfDdwjP8ZLgHvDfw0/zqIf7vjwFcDbw0c47/HJeCngY8GdvnXQ/zHeDDwYP5r3Qrcyr8P4v83xP9v/CNtfsJBncHBUwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QV7w18FbAXwPfA+zy/wMCPhv4LJ7tr4HXAXb5vw8BF4HjPKe/Bl4H2OX/NgTsAsd4Xn8NvA6wy/9dCPhs4LN4/v4aeB1gl/+bEFd8N/BePH9/DbwOsMv/PYhn+27gvXj+/hp4HWCX/1sQz+m7gffi+ftr4HWAXf5zfBbw0Vzx1cDn8J8P8by+G3gvnr+/Bl4H2OU/1ncB781z+m7gffjPhXj+vht4L56/vwZeB9jlP8Z3Ae/N8/fdwPvwnwfxgn038F48f38NvA6wy7/PdwHvzQv33cD78J8D8cJ9N/BePH9/DbwOsMu/zXcB782L5ruB9+E/HuJf9t3Ae/H8/TXwOsAu/zrfBbw3/zrfDbwP/7EQL5rvBt6L5++vgdcBdnnRfBfw3jx/f8MVL8Xz993A+/AfB/Gi+27gvXj+/hp4HWCXF+67gPfm+fsb4LW54reBl+L5+27gffiPgfjX+W7gvXj+/hp4HWCX5++7gPfm+fsb4LWBXa44Dvw28FI8f98NvA//foh/ve8G3ovn76+B1wF2eU7fBbw3z9/fAK8N7PKcjgO/DbwUz993A+/Dv85x4L2AlwZ+Bvhp8W/z3cB78fz9NfA6wC5XfBfw3jx/fwO8NrDL83cc+G3gpXj+vht4H150fwW8NM/2OeLf7ruB9+L5+2vgdYCvAt6b5+9vgNcGdnnhjgO/DbwUz993A+/Dv+y1gd/iOe2Kf5/vBt6L528XOM7z9zfAawO7vGiOA78NvBTP33cD78ML99rAb/GcEP9+3w28Fy+6vwFeG9jlX+c48NvAS/H8fTfwPrxgrw38Fs8J8R/ju4H34l/2N8BrA7v82xwHfht4KZ6/7wbeh+fvtYHf4jkh/uN8N/BevGB/A7w2sMu/z3Hgt4GX4vn7buB9eF6vDfwWzwnxH+u7gffief0N8NrALv8xjgO/DbwUz993A+/Dc3pt4Ld4Toj/eN8NvBfP9jfAawO7/Mc6Dvw28FI8f98NvA/P9trAb/GcEP853hp4beBW4LuBXf5zHAd+G3gpnr/vBt6HK14b+C2eE+J/v+PAbwMvxfP33cD7AK8N/BbPCfF/w3Hgt4GX4vn7buB7gN/iOSH+7zgO/DbwUjx/vwy8Mc8J8X/LceC3gZfiRYP4v+c48NvAS/EvQ/zfdBz4beCleOEQ/7scB94LeGv+ZRV4aWCLFwzxv8tfAS/NfxzE/x6vDfwW/7EQ/3u8NvBb/MdK8b/LXwMvxX+c3xb/uxwH3ht4beA4/3oVuBYYgJ8FPkX8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPtyKRccfsxlQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7scB94LeGv+bSpwHVf8OPAp4n+XvwJemv84vy3+93ht4Lf4j2Xxv8drA7/FfyzE/y5/DbwU/3EQ/7scB94beG3gOC9cBV4K2OIFQ/zfdBz4LeCleeEQ//ccB34LeGn+ZYj/W44DvwW8NC8axP8dx4HfAl6a5+9XgDfiOSH+bzgO/Bbw0jx/3wN8N/BbPCfE/37Hgd8CXprn73uA9wZeG/gtnhPiP8dbA28F/DXwPcAu/zmOA78FvDTP3/cA780Vrw38Fs8J8R/vu4D35tn+GngdYJf/WMeB3wJemufve4D35tleG/gtnhPiP9Z3Ae/N8/pr4HWAXf5jHAd+C3hpnr/vAd6b5/TawG/xnBD/cb4LeG9esL8GXgfY5d/nOPBbwEvz/H0P8N48r9cGfovnhPiP8V3Ae/Mv+2vgdYBd/m2OA78FvDTP3/cA783z99rAb/GcEP9+3wW8Ny+6vwZeB9jlX+c48FvAS/P8fQ/w3rxgrw38Fs8J8e/zXcB78/xdAo7x/P018DrALi+a48BvAS/N8/c9wHvzwr028Fs8J8S/3XcB783z9zfAawNfDbwXz99fA68D7PLCHQd+C3hpnr/vAd6bf9lrA7/Fc3qG+Lf5LuC9ef7+BnhtYJcrvht4L56/vwZeB9jl+TsO/Bbw0jx/3wO8Ny+6vwZeimf7HPGv913Ae/P8/Q3w2sAuz+m7gffi+ftr4HWAXZ7TceC3gJfm+fse4L351zkOvDfwYOC3gZ8W/zrfBbw3z9/fAK8N7PL8fTfwXjx/fw28DrDLFceB3wJemufve4D35t8P8aL7LuC9ef7+BnhtYJcX7ruB9+L5+2vgdbjit4CX5vn7HuC9+Y+BeNF8F/DePH9/A7w2sMuL5ruB9+L5+2uueGmev+8B3pv/OIh/2XcB783z9zfAawO7/Ot8N/Be/Ot8D/De/MdCvHDfBbw3z9/fAK8N7PJv893Ae/Gi+R7gvfmPh3jBvgt4b56/vwFeG9jl3+e7gffihfse4L35z4F4/r4LeG+ev78BXhvY5T/GdwPvxfP3PcB7858H8by+C3hvnr+/AV4b2OU/1ncD78Vz+h7gvfnPhXhO3wW8N8/f3wCvDezyn+OzgY/miq8GPpv/fIhn+y7gvXn+/gZ4bWCX/1sQV3wX8N48f38DvDawy/89CPhs4LN4/v4GeG1gl/+bEHAROM7z+hvgtYFd/u9CwC5wjOf0N8BrA7v834aAzwY+i2f7G+C1gV3+70Nc8dbAawO3At8N7PL/A+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGwfKNOrmG3yAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xoOBBwGXgL/mP9dLA8eAZwC38u+D+Pd7aeC3gONc8dXAx/Cf46uAj+aKXeB1gL/m3w7x7/dXwEvznL4beB/+Y30X8N48p78GXoZ/O8S/n3n+vht4H/5jfBfw3jx/4t8O8e/308Bb8fx9N/A+/Pt8F/DePH/fA7w3/3aIf7/jwG8DL8Xz993A+/Bv813Ae/P8/Q3w2sAu/3aI/xjHgd8GXorn77uB9+Ff57uA9+b5+xvgtYFd/n0Q/3GOA78NvBTP33cD78OL5ruA9+b5+xvgtYFd/v0Q/7GOA78NvBTP33cD78ML913Ae/P8/Q3w2sAu/zEQ//GOA78NvBTP33cD78Pz913Ae/P8/Q3w2sAu/3EQ/zmOA78NvBTP33cD78Nz+i7gvXn+/gZ4bWCX/1iI/zzHgd8GXorn77uB9+GK7wLem+fvb4DXBnb5j4f4z3Uc+G3gpXj+vpsr3pvn72+A1wZ2+c+B+M93HPht4KX41/kb4LWBXf7zIF64twZeC3hp4LX53+W3gb8Gfgf4aZ4/xPN3HPgu4K35v+GngfcBdnlOiOf1YOCvgOP837ILvAxwK8+GeF6/Bbw2/zf9NvA6PBviOX028Fn83/YxwFdzBeI5PR14MM/pEvDdwG8Du/zHemmu+Gv+Yx0HXht4b+AYz+lW4CFcgXhO5nm9D/Dd/O/03sB38bzEFYhne23gt3he4n+v48BFntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hnezDwdJ7X6wC/zf9Orw38Fs/rIcCtAOI57QLHeE5/DXwM8Nv87/LawFcBL81zugQc5wrEc/pu4L34v+17gPfmCsRzOg7cChzj/6ZLwIOBXa5APK+3Bn6K/5veBvhpng3x/L018N3AMf5vuAS8N/DTPCfEC3Yc+GrgrYFj/O90Cfhp4KOBXZ4X4kXzYODB/O9yK3ArLxzi/zfE/2/8IxkbzEGI+TeCAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHTklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+KFey3+b/gdnj/E83cc+C3gpfm/4a+B1wF2eU6I5++jga/i/5aPAb6a54R4/n4beC3+b/kZ4K15Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr8+7w08F3ASwM/DbwPsMsLdxz4LuCtgb8G3gf4a1643wZei+f0O8Br85wQz99vA6/Fc/od4LX5t3sw8FfAcZ7te4D35oX7aeCteLZd4GWAW3nBfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm3+61gd/ieYkXzjyv1wF+mxfst4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+7V4b+C2el3jhzPN6HeC3ecF+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+bf7rWB3+J5iRfOPK/XAX6bF+y3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf7tXhv4LZ6XeOHM83od4Ld5wX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5t/utYHf4nmJF848r9cBfpsX7LeB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/u1eG/gtnpd44czzeh3gt3nBfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm3+61gd/ieYkXzjyv1wF+mxfst4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+7V4b+C2el3jhzPN6HeC3ecF+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+bf7rWB3+J5iRfOPK/XAX6bF+y3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf7tXhv4LZ6XeOHM83od4Ld5wX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5t/utYHf4nmJF848r9cBfpsX7LeB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/u1eG/gtnpd44czzeh3gt3nBfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm3+7BwF8Dx3i27wHemxfup4G34tkuAQ8GdnnBfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm3+elge8GXgr4HuCjgV1euOPAVwPvBfwN8N7AX/PC/TbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf98x4G/Ah7Mc/od4LV5Tojn77eB1+I5/Q7w2vzPdhz4LeCleV4/A7w1zwnx/P028Fo8p98BXpv/uY4DvwW8NM/f+wDfzXNCPH+/DbwWz+l3gNfmf6bjwG8BL83z9z3Ae/O8EM/fbwOvxXP6HeC1+Z/nOPBbwEvz/H0P8N48f4jn77eB1+I5/Q7w2vzPchz4LeClef6+B3hvXjDE8/fbwGvxnH4HeG3+5zgO/Bbw0jx/3wO8Ny8c4vn7beC1eE6/A7w2/zMcB34LeGmev+8B3pt/GeL5+23gtXhOvwO8Nv/9jgO/Bbw0z9/3AO/Niwbx/P028Fo8p13gr3nR3Ar8DPDTvGAvDXwU8GD+dR4MPJjn73uA9+ZFh3j+fht4Lf79Pgf4bJ7XewPfxX+s7wHem38dxPP328Br8e+3C5zgOb038F38x/oe4L3510M8f18NfBT/McSzvTfwXfzH+h7gvfm3QTx/Lw38Ff8xxBXvDXwX/7G+B3hv/u0QL9hLA+8NvDQvmuPAS/G8BLw38F28YL/Dv84u8NPAd/Pvg/iP89rAb/G83gf4Ll6w9wG+m/8eiP84rw38Fv867wN8N/99EP9xXhv4LV507wN8N/+9EP9xXhv4LV407wN8N//9EP9xXhv4Lf5l7wN8N/8zIP7jvDbwW7xw7wN8N/9zIP7jvDbwW7xg7wN8N/+zIP7jvDbwWzx/7wN8N//zIP7jvDTwVzyv9wG+m/+ZEP+xPhv4LK64BHw08N38z4X4j/fSwHHgr4Fd/mdD/P+G+P+NfwQD8wxQRhbaXAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgt4a/53+WngfYBdXjDEv+yngbfif6efAd6aFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+bf7quAl+Y5/TXwMfzbvTTwVTwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/Mv+GngpntPLAH/Nv91vA6/Fc/od4LX5t3tp4K94Tn8DvDQvGOJf9tLAbwPHuOJzgM/m3+e3gdfiOf0O8Nr8+3w28FlccQl4beCvecEQL5oHAw8GdoG/5t/vt4HX4jn9DvDa/Pu9NHAc+GtglxcO8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvxn+u1eP6+GnhpntNfAx/N8/c7/OdA/Oc4DvwW8NL8x/hr4HWAXf5jIf5zfDTwVfzH+hjgq/mPhfjP8d3Ae/Ef62uAj+Y/FuI/x1sDP8V/rLcBfpr/WIj/PJ8NfDRwjH+fS8BXA5/NfzzE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGRU4BBfOT1rAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/di8NvBdXfA/w1/zXemngvbjie4C/5l8P8W/z0sBf8ZxeBvhr/mu8NPBXPKeXAf6afx3Ev81XAx/Fc/oa4KP5r/HVwEfxnL4G+Gj+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81XAx/Fc/od4LX5r/HbwGvxnD4H+Gz+dRD/Np8NfBbP6XeA1+a/xm8Dr8Vz+hzgs/nXQfzbfDbwWTwv8V/DPK+PAb6afx3Ev81rA7/F83od4Lf5z/XWwE/xvF4H+G3+dRD/duZ5fQ3w0fzn+mrgo3hOl4Dj/Osh/u1+GngrntMu8BBgl/8cx4GnA8d5Tj8DvDX/eoh/u/cGvovn9TXAR/Of46uBj+J5vQ/w3fzrIf7tjgO3Asd4Xq8D/Db/sV4b+C2e1zOAlwZ2+ddD/Pu8N/BdPK9d4HWAv+Y/xksDvwUc53m9D/Dd/Nsg/v1uBR7E87oVeBvgr/n3eWngp4AH87yeATyYfzvEv99LA78NHON57QLvA/w0/zZvDXwXcJzndQl4beCv+bdD/Md4b+C7eMF+G/gY4K950bw08FXAa/OCvQ/w3fz7IP7jfDbwWbxwfw38NPDbwCXgr7nipYFjwGsDbw28NC/c5wCfzb8f4j/WRwNfxX+ujwG+mv8YiP94bw18N3CM/1iXgPcGfpr/OIj/HA8Gvht4Lf5j/A7w3sCt/MdC/Od6beCzgdfi3+Z3gM8Gfpv/HIj/Gi8NvDbw1sBr8cL9DvDTwG8Df81/LsR/j+PAS/Oc/hrY5b8W4v83xP9viP/fEP+/If5/Q/z/hvj/DfGieWvgrYAH87/DrcDPAD/NC4f4l3028Fn87/Q5wGfzgiH+ZReB4/zvtAuc4AVD/Mt2gWP873QJOM4LhviXfTbwWfzv9DnAZ/OCIV40bw28NfBg/ne4Ffhp4Kd54RD/vyH+f0P8/4b4/w3x/xv/CJAVtEHCmlI2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sd4KeGmueGngVmAXuBX4GWCX/1kQ/34PBj4LeGvgOC/cXwMfA/w2/zMg/u2OA58FfDT/er8NfAzw1/z3QvzbHAd+C3hp/u12gbcBfpv/Poh/vePA04Hj/Md4G+Cn+e+B+Nc5DvwW8NK8cM8AbgWOAy/FC7cLvA7w1/zXQ/zrfDXwUTx/l4CPBn4a2OU5vTbw2cBr8fzdCjyE/3qIF92Dgafz/P0M8N7ALi/cRwNfxfP3OcBn818L8aL7buC9eF4/A7w1L7q3Bn6K57ULnOC/FuJFdxE4znO6BDwY2OVf56eBt+J5vQ3w0/zXQbxo3hr4KZ7X+wDfzb/eceBW4BjP6XuA9+a/DuJF89nAZ/G8TgC7/Nv8NvBaPKffAV6b/zqIF81nA5/Fc3oG8GD+7b4a+Cie0y5wgv86iBfNTwNvxXP6HeC1+bf7aOCreF7ivw7iRfPVwEfxnP4aeBn+7T4b+Cyel/ivg3jRfDbwWTwv8W/308Bb8Zx+B3ht/usgXjTvDXwXz+t1gN/m3+bpwIN5Tj8DvDX/dRAvmuPARZ7XbwOvw7/eRwNfxfN6H+C7+a+DeNH9NfBSPK+PAb6aF91LA78FHOd5PQS4lf86iBfdawO/xfPaBd4H+Gn+ZS8NfBfw0jyv7wHem/9aiH+d3wZei+fvp4H3AXZ5/j4K+GzgOM/fQ4Bb+a+F+Nd5aeC3gWM8f7vAXwN/DdwKHAdeGnhp4MG8YO8DfDcvmpcGjvHv9wzgVvGv99rAb/Ef53uA9+ZF89nAZ/EfYxd4HfFv897AVwPH+Pf5HuC9edG8NPBX/Mf6a/Fv99LATwMP4t/mfYDv5kX32sBv8R8L8e/32cBHA8d40XwP8NnArfzrPBj4a+AY/3G+R/zHeWvgrYEHAy8NHOOK3wF2gZ8Gfhu4lX+7lwa+G3gp/v2+B/ho8f8b4v83xP9viP/fEP+/IV40x4G3Ah4MHAdemv84twK3ArvA7wB/zX8dxAv31sBnAS/Nf51bge8GvgbY5T8X4vl7aeCrgNfmv8+twMcAP81/HsTzem3gp4Dj/M/w1cDH8J8D8ZzeG/gu/uf5buB9+I+HeLaXBn4LOM4L9zfALv+xXot/2ccAX81/LMQVx4G/Ah7M8/c7wGcDv81/ngcDHw18FC/Y6wC/zX8cxBWfDXwWz+sS8NnAV/Nf57WB7wYexPP6beB1+I+DgOPA04HjPK+3AX6a/3rHgVuBYzyv9wG+m/8YCHhr4Kd4Xj8DvDX/fd4a+Cme1/cA781/DAR8N/BePKdLwIOBXf57/TTwVjynXeAE/zEQ8NvAa/Gcfgd4bf77fTTwVTyvhwC38u+HgIvAcZ7T5wCfzX+/1wZ+i+f1OsBv8++HAPO83gf4bv5nMM/rdYDf5t8PAeZ5vQ7w2/zPYJ7X6wC/zb8fAszzeh3gt/mfwTyv1wF+m38/BJjn9TrAb/M/g3lerwP8Nv9+CDDP63WA3+Z/BvO8Xgf4bf79EGCe1+sAv83/DOZ5vQ7w2/z7IWAXOMZzeh3gt/mfYRc4xnN6HeC3+fdDwGcDn8Wz/Q3w0vzP8dnAZ/FsfwO8NP8xEFe8NfDawK3AdwO7/M/y1sBrA7cC3w3s8h8D8f8b4v83xP9viP/fEP+/8Y8Kqcvv6AI9nQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9V4aeC3gOPBgrrgV2AV+B/hr/usg/mscBz4KeG/gwbxwtwLfDXwNsMt/LsR/vrcGvgs4zr/OrcDHAD/Nfx7Ef66vAj6af5+vBj6G/xyI/zzfBbw3/zG+G3gf/uMh/nN8NvBZ/Mt+hytei3/ZxwBfzX8sxH+81wZ+ixfsc4DvBm7lOT0Y+Gjgo3jBXgf4bf7jIP7j/Rbw2jyvZwDvDfw2L9xrA98NPIjn9dvA6/AfB/Ef672B7+J5XQIeDOzyojkO3Aoc43m9D/Dd/MdA/Mf6buC9eF5vA/w0/zpvDfwUz+t7gPfmPwbiP9ZF4DjP6WeAt+bf5qeBt+I57QIn+I+B+I/z0sBf8bw+Bvhq/m0+GvgqntdDgFv590P8x3lt4Ld4Xq8D/Db/Nq8N/BbP63WA3+bfD/Ef57WB3+J5iX8f87xeB/ht/v0Q/3FeG/gtntfLAH/Nv81LA3/F83od4Lf590P8x3lt4Ld4Xm8D/DT/Nu8NfBfP63WA3+bfD/EfyzyvrwE+mn+brwY+iucl/mMg/mP9NvBaPKdd4GWAW/nXeTDwV8BxntPPAG/NfwzEf6yPBr6K5/XbwOvwr/NbwGvzvD4G+Gr+YyD+Yx0HbgWO8by+G/gYYJcX7jjwVcB787wuAQ8GdvmPgfiP99HAV/H83Qq8DfDXPH+vDXwX8GCev48Bvpr/OIj/HL8NvBYv2F8Dfw38NVe8NPDSwEvzgv0O8Nr8x0L85zgO/DbwUvzH+BvgtYFd/mMh/vM8GPhp4KX49/kb4K2BW/mPh/jPdRz4buCt+Lf5GeC9gV3+cyD+a7w28NXAS/Gi+Rvgo4Hf5j8X4r/WSwNvDbw2cBx4Ka74G2AX+G3gp4G/5r8G4v83xP9viP/fEP+/If5/Q/zHOA68FfBg4Djw0vzn+GtgF7gV+Blgl38fxL/PWwOfBbw0/z3+GvgY4Lf5t0H827w08FXAa/M/w28DHwP8Nf86iH+91wZ+CjjO/yy7wNsAv82LDvGv897Ad/E/29sAP82LBvGie2ngt4DjvHB/A+zyn+M48FK8cLvA6wB/zb8M8aI5DvwV8GCev98BPhv4bf5rvDbw2cBr8fzdCjyEfxniRfPZwGfxvC4Bnw18Nf89Phr4Kp6/zwE+mxcO8S87DjwdOM7zehvgp/nv9dbAT/G8doETvHCIf9lbAz/F8/oZ4K35n+Gngbfieb0N8NO8YIh/2XcD78VzugQ8GNjlf4bjwK3AMZ7T9wDvzQuG+Jf9NvBaPKffAV6b/1l+G3gtntPvAK/NC4b4l10EjvOcPgf4bP5n+Wrgo3hOu8AJXjDEv8w8r/cBvpv/Wd4b+C6el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLdoFjPKfXAX6b/1leG/gtntMl4DgvGOJf9tnAZ/FsfwO8NP8z/TXwUjzb5wCfzQuGeNG8NfDawK3AdwO7/M90HHhv4MHAbwM/zQuH+P8N8f8b4v83xP9viP/f+Ef9TLxBJIkb4AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHQklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8V4aeC3gOPBgrrgV2AV+B/hr/vWOAy8FPAO4lf84iP8Yx4GPAt4beDAv3K3AdwNfA+zyL3tp4LeA41zx28BXAz/Dvx/i3++tge8CjvOvcyvwMcBP88I9HXgwz+tW4GOAn+bfDvHv81XAR/Pv89XAx/CCmRfufYDv5t8G8W/3XcB78x/ju4H34fn7aeCteOE+Gvga/vUQ/zafDXwW/7Lf4YrX4l/2McBX87yOA98NvBUv3OcAn82/DuJf77WB3+IF+xzgu4FbeU4PBj4a+ChesNcBfpvn78HARwPvDRzj+XsIcCsvOsS/3m8Br83zegbw3sBv88K9NvDdwIN4Xr8NvA4v3HHgp4HX4nl9D/DevOgQ/zrvDXwXz+sS8GBglxfNceBW4BjP632A7+Zf9t3Ae/G8Xgb4a140iH+d7wbei+f1NsBP86/z1sBP8by+B3hv/mXHgVuBYzynnwHemhcN4l/nInCc5/QzwFvzb/PTwFvxnHaBE7xoPhv4LJ7TXwMvw4sG8aJ7aeCveF4fA3w1/zYfDXwVz+shwK38y14b+C2el3jRIF50rw38Fs/rdYDf5t/mtYHf4nm9DvDb/MseDDyd5/UywF/zL0O86F4b+C2el/j3Mc/rdYDf5kVjntfrAL/Nvwzxontt4Ld4Xi8D/DX/Ni8N/BXP63WA3+Zfdhy4yPN6HeC3+ZchXnSvDfwWz+ttgJ/m3+a9ge/ieb0O8Nv8y14b+C2el3jRIP51zPP6GuCj+bf5auCjeF7iRfPewHfxnJ4BPJgXDeJf57eB1+I57QIvA9zKv86Dgb8CjvOcfgZ4a140TwcezHP6HeC1edEg/nU+GvgqntdvA6/Dv85vAa/N8/oY4Kv5l3008FU8r/cBvpsXDeJf5zhwK3CM5/XdwMcAu7xwx4GvAt6b53UJeDCwywv33sB38byeATyYFx3iX++jga/i+bsVeBvgr3n+Xhv4LuDBPH8fA3w1L9hLAx8FvDfP3+sAv82LDvFv89vAa/GC/TXw18Bfc8VLAy8NvDQv2O8Ar83z917AewOvzQv2O8Br86+D+Lc5Dvw28FL8x/gb4LWBXZ7XZwOfxQv3N8BrA7v86yD+7R4M/DTwUvz7/A3w1sCtPH8XgeO8YH8DvDawy78e4t/nOPDdwFvxb/MzwHsDu7xgu8Axnr/vAd6bfzvEf4zXBr4aeCleNH8DfDTw2/zLPhv4LJ7T9wBfDfw1/z6I/1gvDbw18NrAceCluOJvgF3gt4GfBv6af533Bt4a+Gvgu4Fb+Y+B+P8N8f8b4v83xP9viP/fEC+a48BbAQ8GjgMvzX+OvwZ2gVuBnwF2+dd7aeAY8DfALi8c4oV7a+CzgJfmv8dfAx8D/DYvms8GPosrdoGfBj4HuJXnD/H8vTTwVcBr8z/DbwMfA/w1L9iDgafz/P008DHArTwnxPN6beCngOP8z7ILvA3w2zx/rw38Fi/YLvA6wF/zbIjn9N7Ad/E/29sAP83zOg7cChzjBdsFXgf4a65APNtLA78FHOeF+xtgl/8cx4GX4oXbBV4H+Gue10sDPw08iBdsF3gd4K8BxBXHgb8CHszz9zvAZwO/zX+N1wY+G3gtnr9bgYfwgr028N7Ae/H8/TbwOgDiis8GPovndQn4bOCr+e/x0cBX8fx9DvDZvHAvDfw2cIzn9TrAbws4DjwdOM7zehvgp/nv9dbAT/G8doET/MteGvhp4EE8p1uBhwh4a+CneF4/A7w1/zP8NPBWPK+3AX6af9l7A9/F83obAd8NvBfP6RLwYGCX/xmOA7cCx3hO3wO8Ny+aW4EH8Zw+R8BvA6/Fc/od4LX5n+W3gdfiOf0O8Nq8aL4beC+e088IuAgc5zl9DvDZ/M/y1cBH8Zx2gRO8aD4b+Cye0+8IMM/rfYDv5n+W9wa+i+clXjTvDXwXzwkB5nm9DvDb/M/y2sBv8bzEi+a1gd/iOSHAPK/XAX6b/1leG/gtnpd40bw28Fs8JwSY5/U6wG/zP8trA7/F8xIvmtcGfovnhADzvF4H+G3+Z3lt4Ld4XuJF89rAb/GcEGCe1+sAv83/LK8N/BbPS7xoXhv4LZ4TAnaBYzyn1wF+m/9ZXhv4LZ7TM4AH86J5beC3eE7PEPDZwGfxbH8DvDT/M/018FI82+cAn82L7q+Bl+LZPkdc8dbAawO3At8N7PI/03HgvYEHA78N/DT/OseB9wYeDPw28NPi/zfE/2+I/98Q/78h/n/jHwFb0x/qbnxSsgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvihXst/vs8A7iV/1yI5/VZwHsDD+a/3y7w08DHALv8x0M824OB7wJem/95doH3AX6a/1iIK44DTweO8z/b2wA/zX8cxBU/Bbw1//PtAg8BdvmPgYC3Bn6K/z2+B3hv/mMg4KuBj+I5XQI+Gvhu/vu8NvDVwEvxnHaBE/zbvDXwVcCDgZ8G3kfAbwOvxXP6GuCj+e/32sBv8bweAtzKi+448FnAR/OcvkeAeV5vA/w0/zOY5/U6wG/zonlp4LuAl+Z5IcA8r9cBfpv/Gczzeh3gt/mXfRTw1bxgCDDP63WA3+Z/BvO8Xgf4bV6w48BPAa/NC4cA87xeB/ht/mcwz+t1gN/m+Xtr4LuA4/zLEGCe1+sAv83/DOZ5vQ7w2zyn48BnAR/Niw4B5nm9DvDb/M9gntfrAL/Ns7008F3AS/OCXQKO8ZwQYJ7X6wC/zf8M5nm9DvDbXPFRwFfzwv0M8N3AT/GcEGCe1+sAv83/DOZ5vQ7w18BPAa/NC3YJ+Gjgu4HXBn6L54QA87xeB/ht/mcwz+szgI8DjvOC/Q3w1sCtXPHawG/xnBBgntfrAL/N/wzmX+9zgM/mOb028Fs8JwSY5/U6wG/zP4N50T0DeG/gt3lerw38Fs8JAeZ5vQ7w2/zPYF40PwO8N7DL8/fawG/xnBBgntfrAL/N/wzmhbsEfDTw3bxwrw38Fs8JAeZ5vQ7w2/zPYF6wvwHeGriVf9lrA7/Fc0KAeV6vA/w2/zOYF+yvgfcB/pp/2WsDv8VzQoB5Xq8D/Db/M5gXbhf4bOBreOFeG/gtnhMCzPN6HeC3+Z/BvGh+GngfYJfn77WB3+I5IcA8r9cBfpv/GcyLbhd4G+C3eV6vDfwWzwkB5nm9DvDb/M9g/vW+GvgYntNrA7/Fc0KAeV6vA/w2/zOY5/UZwMcDx3jB/hp4H+CvueK1gd/iOSHAPK/XAX6b/xnM83od4K+BnwZeixdsF/hs4GuA1wZ+i+eEAPO8Xgf4bf5nMM/rdYDf5oqPBr6KF+6nge8BfornhADzvF4H+G3+ZzDP63WA3+bZXhr4buCl+NdBgHlerwP8Nv8zmOf1OsBv85yOA58NfBQvOgSY5/U6wG/zP4N5Xq8D/DbP31sD3w0c41+GAPO8Xgf4bf5nMM/rdYDf5gU7Dvw08Fq8cAgwz+t1gN/mfwbzvF4H+G3+ZR8NfBUvGALM83ob4Kf5n8E8r9cBfpsXzUsD3w28FM8LAb8NvBbP6WuAj+a/32sDv8XzeghwKy+648BnAx/Fc/odAV8NfBTPaRf4aOBngF3+e7w28FXAS/OcLgHH+bd5a+CrgQcBPwO8t4C3Bn6K/z2+B3hv/mMgrvhp4K34n+8S8GBgl/8YiCuOA7cCx/if7W2An+Y/DuLZHgx8N/Ba/M9zCXhv4Kf5j4V4Xh8NfDTwIP77XQJ+GvhoYJf/eIgX7rX573MrcCv/uRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEyOd2lXVuGrQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv95LA8eAZwC38t8L8V/ru4D35opd4HWAv+a/D+K/xnHgt4CX5jn9NfAy/PdB/Od7aeC3gOM8f+K/D+I/13sD38ULJ/77IP7zfBfw3vzLxH8fxH+848BvAS/Ni0b890H8x3pp4LeA47zoxH8fxH+c9wa+i3898d8H8R/ju4D35gX7G+CzgZ/ieYn/Poh/n+PAbwEvzQv2PcB7A68N/BbPS7xgrw38FHAcuBV4G+Cv+Y+DeE7HgfcC3poXzUsDx3nB3gf4bq54beC3eF7iBbsIHOfZdoGHALv8x0A8p78CXpp/v0vAawN/zbO9NvBbPC/x/L028Fs8r78GXgfY5d8P8WyvDfwW/35/A7w2sMtzem3gt3he4gXbBY7xvL4G+Gj+/RDP9trAb/Hv8z3Ae/P8vTbwWzwv8YK9N/BdPH8PAW7l3wfxbK8N/Bb/du8DfDcv2GsDv8XzEi/cdwPvxfP6HuC9+fdBPNtrA7/F8/odXrhbga8G/poX7rWB3+J5iX/ZrcCDeF4ngF3+7RDP9trAb/G8xH+M1wZ+i+cl/mVvDfwUz+ttgJ/m3w7xbK8N/BbPS/zHeG3gt3he4kWzCxzjOX0P8N782yGe7bWB3+J5if8Yrw38Fs9LvGi+G3gvntPvAK/Nvx3i2V4b+C2el/iP8drAb/G8xIvms4HP4jndCjyEfzvEs7028Fs8L/Ef47WB3+J5iRfNewPfxfMS/3aIZ3tt4Ld4XuI/xmsDv8XzEi+a9wa+i+cl/u0Qz/bawG/xvMR/jNcGfovnJV40nw18Fs/pGcCD+bdDPNtrA7/F8xL/MV4b+C2el3jRfDfwXjyn3wFem387xLO9NvBbPC/xH+O1gd/ieYkXzUXgOM/pe4D35t8O8WyvDfwWz0v8x3ht4Ld4XuJf9t7Ad/G83gb4af7tEM/22sBv8bx+mxfuVuBrgL/mhXtt4Ld4XuJf9nTgwTyvE8Au/3aIZ3tt4Lf4t3sf4Lt5wV4b+C2el3jhvgt4b57X9wDvzb8P4tleG/gt/n2+G3gfnr/XBn6L5yVesPcGvovn7yHArfz7IJ7ttYHf4t/vr4HXAXZ5Tq8N/BbPSzx/x4GnA8d5Xp8DfDb/fojn9NfAS/Hvtwu8DvDXPNtrA7/F8xLP32sDv8Xz+hvgpfmPgXhOx4H3Bl4bOM6/7KWBY7xg7wN8N1e8NvBbPC/xgu0Cx3i2S8CDgV3+YyD+fY4Dvw28FC/YdwPvA7w28Fs8L/GCvTTw08CDgL8B3hv4a/7jIP5jfDfwXrxgfw18DvBTPC/x3wfxH+e9ge/iX0/890H8x3pp4LeBY7zoxH8fxH+848BvAy/Fi0b890H85/lu4L34l4n/Poj/XO8NfBcvnPjvg/jP99LAbwPHeP7Efx/Ef43jwG8DL8VzegbwYP77IP5rfTXwUVxxCXht4K/574P4r/fSwHHgr4Fd/nsh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C5rKZQVPQ8lMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv95LA8eAZwC38t8L8V/ru4D35opd4HWAv+a/D+K/xnHgt4CX5jn9NfAy/PdB/Od7aeC3gOM8f+K/D+I/13sD38ULJ/77IP7zfBfw3vzLxH8fxH+848BvAS/Ni0b890H8x3pp4LeA47zoxH8fxH+c9wa+i3898d8H8R/ju4D35gX7G+CzgZ/ieYn/Poh/n+PAbwEvzQv2PcB7A68N/BbPS/z3QTyn48B7AW/Ni+algeO8YO8DfDdXvDbwWzwv8d8H8Zz+Cnhp/v0uAa8N/DXP9trAb/G8xH8fxLO9NvBb/Pv9DfDawC7P6bWB3+J5if8+iGd7beC3+Pf5HuC9ef5eG/gtnpf474N4ttcGfot/u/cBvpsX7LWB3+J5if8+iGd7beC3eF6/wwt3K/DVwF/zwr028Fs8L/HfB/Fsrw38Fs9L/Md4beC3eF7ivw/i2V4b+C2el/iP8drAb/G8xH8fxLO9NvBbPC/xH+O1gd/ieYn/Pohne23gt3he4j/GawO/xfMS/30Qz/bawG/xvMR/jNcGfovnJf77IJ7ttYHf4nmJ/xivDfwWz0v890E822sDv8XzEv8xXhv4LZ6X+O+DeLbXBn6L5yX+Y7w28Fs8L/HfB/Fsrw38Fs9L/Md4beC3eF7ivw/i2V4b+C2el/iP8drAb/G8xH8fxLO9NvBbPK/f5oW7Ffga4K954V4b+C2el/jvg3i21wZ+i3+79wG+mxfstYHf4nmJ/z6IZ3tt4Lf49/lu4H14/l4b+C2el/jvg3i21wZ+i3+/vwZeB9jlOb028Fs8L/HfB/Gc/hp4Kf79doHXAf6aZ3tt4Ld4XuK/D+I5HQfeG3ht4Dj/spcGjvGCvQ/w3Vzx2sBv8bzEfx/Ev89x4LeBl+IF+27gfYDXBn6L5yX++yD+Y3w38F68YH8NfA7wUzwv8d8H8R/nvYHv4l9P/PdB/Md6aeC3gWO86MR/H8R/vOPAbwMvxYtG/PdB/Of5buC9+JeJ/z6I/1zvDXwXL5z474P4z/fSwG8Dx3j+xH8fxH+N48BvAy/Fc3oG8GD++yD+a3018FFccQl4beCv+e+D+K/30sBx4K+BXf57If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AmPAZkE/8/Y5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "11 5 6 9 2 9 2 15 6 15 11 19 11 5",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG/UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMooDjwHcBbw38NfAxwG/z/wMCfhp4K57T+wDfzf99CDDP3/sA383/bQj4a+CleP7eB/hu/u9CwGsDv8UL9j7Ad/N/E+KK9wa+ixfsfYDv5v8exLO9N/BdvGDvA3w3/7cgntN7A9/FC/Y+wHfzfwfieb038F28YO8DfDf/NyCev/cGvosX7H2A7+Z/P8QL9t7Ad/GCvQ/w3fzrvRYv2N8Au/zXQbxw7w18Fy/Y+wDfzYvmpYHfAo7zovltrtgFfhr4HeBW/mMh/mXvDXwXL9j7AN/Nv+zpwIP59/lr4LeBnwZ+h38/xIvmvYHv4gX7BODLeeHMf7yfBn4a+B7+bRAvuvcGvosX7LOAz+UFuxV4EP85bgU+G/ge/nUQ/zpfDHwSL9j7AN/N8/fSwE8DD+I/z63AZwPfw4sG8aJ5beCzgNfmX/Y+wHfzgh0Hdnler82zvTZXPBh4beBB/OvcCnw28D28cIgX7sHAdwGvzb/O+wDfzX+cBwNvDbw28Fa86P4aeBvgVp4/xAv21sB3Acf5t3kf4Lv5z/HWwFsD78W/bBd4H+CneV6I5++rgI/mX/YE4NG8YO8DfDf/eR4MfDbwXvzLvhr4GJ4T4jm9NPBdwEvzwn0P8NXAXwPvDXwXL9j7AN/Nf64HA58NvBcv3F8D7wP8NVcgnu29ga8CjvOCfQ/w2cCtPKf3Br6LF+x9gO/mP9+Dgc8G3osXbBf4GOC7AcQV7w18Fy/YJeC9gZ/mBXtv4Lt4wd4H+G7+a7w18N3AMV6w9wG+W1zx28Br8fz9DfDWwK38y94b+C5esPcBvpv/Gg8Gfhp4KZ6/3wFeW1zx28Br8by+Bvho/nXeG/guXrBPAL6c/zpfDXwUz+t3gNcWV7w18FM82yXgvYGf5t/m44Ev4wX7LOBz+a/z1sB3A8d4trcBflo821sDbw3cCnw1sMu/znHgrYCPBl6af9n7AN/Nv95rAb/Dv95x4KOBBwM/Dfw0gPiP8VHAZwPH+dd5H+C7edG8NPBTwIOBXeCrga8Bdvm3Q/z7vBfw2cCD+bd7H+C7+Zc9HXgwz+lW4LOB7+HfBvFvcxz4LuCt+Y/xPsB388KZF+y3gbcBdvnXQfzrvTbwXcCD+Zf9DfDdwG8DLw18Fy/Y+wDfzQt2K/AgXrBd4G2A3+ZFh/jXeW/gu3jhLgHfDXw1cCvP6b2B7+IFex/gu3n+Xhr4aeBBvHDvA3w3LxrEi+7BwNN54T4H+GpglxfsvYHv4gV7H+C7ef6OAx8NfDRwjBfsIcCt/MsQL7rXBn6L5+9ngI8GbuVF897Ad/GCvQ/w3bxgDwa+Gngrnr/XAX6bfxniRfdg4Ok8r48Bvpp/vfcGvosX7H2A7+aF+2jgq3heDwFu5V+G+Nd5b+CrgWPA3wDvDfw1/3bvDXwXL9j7AN/NC/fSwHcDLwVcAj4a+G5eNIh/m+PALv8x3hv4Ll6w9wG+m3/ZcWCXfx3Ef63jwGsBL80VD+aK48Bb84K9D/Dd/MdD/Oc7DrwX8N7AS/Nv9z7Ad/MfC/Gf5zjwVcB78x/nfYDv5j8O4j/HewFfDRznP977AN/NfwzEf7yPBr6K/1zvA3w3/36I/3hPBx7Mv+wS8Ndc8WDgQfzrvA/w3fz7IP7j/TbwWjyvS8BXA78N/DbP32sDrw18NHCMf9n7AN/Nvx3iP95bAz/Fs10Cvhr4amCXF81x4KOBjwaO8cK9D/Dd/Nsg/nO8NvDawC7w3cAu/zbHgfcGXht4K16w9wG+m389xP8e7w18Fy/Y+wDfzb8O4n+X9wa+ixfsfYDv5kWH+N/nvYHv4gV7H+C7edEg/nd6b+C7eMHeB/hu/mWI/16vxRW/w7/eewPfxQv2PsB388Ih/nscB34LeGmu+GvgdYBd/nXeG/guXrD3Ab6bFwzx3+Ojga/iOX0M8NX867038F28YO8DfDfPH+K/x3cD78Vz+hngrfm3eW/gu3jB3gf4bp4X4r/HWwM/xXN6H+C7+bd7b+C7eMHeB/hunhPiv89nAx/NFV8NfDb/fu8NfBcv2PsA382zIf7veW/gu3jB3gf4bq5A/N/03sB38YK9D/DdAOL/rvcGvosX7H2A7xb/t7038F28YK8j/u97b+C7eP7+Wvz/8N7Ad/G8EP9/vDfwXTyn7xH/v7w28NXASwHfA3z0PwKe5woOabNusgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8eDgQfxnJ4B3Mp/LcR/nrcCXht4aa44Drw0L5q/Bna54q+B3wZ+hv94iP84Lw28FvDWwGvzn+O3gZ8Gfgf4a/79EP8+rw28F/DWwHH+a+0CPw18D/Db/Nsg/m1eG/gs4LX5n+G3gc8Bfpt/HcS/znsBnw08mP+Z/hr4auB7eNEgXjTvBXw28GD+dS4Bvw38NVfcCtzKFbcCt3LFg4EHc8VLA8e54qWB1waO8a9zK/DZwPfwwiFeuAcDPwW8NC+63wF+G/hp4K/5j/HSwFsDrw28Fi+6vwbeBriV5w/xgr018F3Acf5l3wP8NPDbwC7/uY4Drw28NfBe/Mt2gfcBfprnhXj+vgr4aP5l3wN8NnAr/z0eDHw28F78y74a+BieE+I5vTTwXcBL88J9D/DZwK38z/Bg4LOB9+KF+2vgbYBbuQLxbO8NfBVwnBfse4DPBm7lf6YHA58NvBcv2C7wMcB3A4gr3hv4Ll6wS8B7Az/N/w5vDXw3cIwX7H2A7xZX/DbwWjx/fwO8NXAr/7s8GPhp4KV4/n4HeG1xxW8Dr8Xz+hrgo/n3eyvgpYEHAw/miuPAS3PFrcCtXLEL/DXw18DvALv8+3w18FE8r98BXltc8dbAT/Fsl4D3Bn6af5uXBt4KeG3gtfn3+Wvgt4HfBn6Gf5u3Br4bOMazvQ3w0+LZ3hp4a+BW4KuBXf713gv4bODB/Oe4Ffhq4HuAXf51jgMfDTwY+GngpwHEv99x4K2AzwYezH+NXeCrga8Bdvm3Q/z7vDXwXcBx/nvsAp8NfA3/Noh/m+PAZwEfzYvuEvDXwK3ArVyxC+wCD+aK48BLAw8GHsSL7reB9wFu5V8H8a/30sBPAQ/mX/Y7wG8DPw38Nf86Lw28NvDawGsDx3jhdoH3AX6aFx3iX+e9ge/iX/Y9wGcDt/If4zjw0cBHA8d44b4beB9eNIgX3YOBp/OCXQK+GvhqYJf/HMeBtwY+G3gQL9jbAD/Nvwzxontt4Ld4/n4HeG/gVv5rHAe+Gngvnr/PAT6bfxniRfdg4Ok8r88BPpv/Hm8NfDdwjOf0OsBv8y9D/Ou8N/DVwDHgb4D3Bv6a/14PBr4beC2u+Bzgs3nRIP5tjgO7/M9yHNjlXwfxX+s48FrAS3PFg7niVq74a+B3gF3+ayD+8x0H3gt4b+CledHcCnw38DXALv95EP95jgNfBbw3/z7fDXwMsMt/PMR/jvcCvho4zn+MXeCzga/hPxbiP95HA1/Ff47PAT6b/ziI/3hPBx7Mv+wS8Ndc8WDgQfzLbgUewn8cxH+83wZei+d1Cfhq4LeB3+b5e23gtYGPBo7xvJ4BPJj/OIj/eG8N/BTPdgn4auCrgV1eNMeBjwY+GjjGs70N8NP8x0H853ht4LWBXeC7gV3+bY4D7w0cB34b+G3+YyH+f0P8/4b4/w3x/xviv9drccXv8N8D8d/jOPBbwEtzxV8DrwPs8l8L8d/jo4Gv4jl9DPDV/NdC/Pf4buC9eE4/A7w1/7UQ/z3eGvgpntP7AN/Nfy3Ef5/PBj6aK74a+Gz+6yH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjzGuyEHr1McsAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P81zsOPBj4a/77If5rHAe+Cnhp4KV5tt8Gvhr4Gf57IP7zvTbwU8BxXrC/Bt4GuJX/Woj/XO8NfBcvml3gfYCf5r8O4j/XReA4L7pd4GWAW/mvgfjPZf71fht4Hf5rIP5z/TbwWjzbJeCrgd8GPhp4K56/E8Au//kQ/7mOA18NvDRwK/DRwK0823cD78Xzeh3gt/nPh/jvZ57XxwBfzX8+xH+v48BFntfrAL/Ni+a1gLcG3hp4MFf8NnAr8D3Ab/OCIf57fTXwUTyvlwH+mhfuwcBPAS/NC/fbwPsAt/K8EM/rpYFj/Od6MPDWwFvzvJ4BPJgX7r2BrwKO86LZBd4G+G2eE+I5fTbwWfz3+hzgs3nBHgw8nX+b1wF+m2dDPNtLA3/Ff69LwIOBXV6wtwZ+in+bvwZehmdDPNtrA7/Ff6/XAX6bF+6lgb/ieX0N8Ntc8dbAe/H8fQzw1VyBeLYHA38NHOO/3iXgo4Hv5kXz2cBnccUzgPcGfpvn9NrATwPHeE6/A7w2VyCe00sD3w28FP/5/gbYBX4b+Gpgl3+dBwMPBn6bF+yjga/ieYkrEP+3HQcu8rxeB/htAPF/n3lerwP8NoD4v+2lgb/ieT0EuBVA/N/2U8Bb87zEFYj/uz4K+Gqe1/cA780ViOd0HHgv4K353+3BwIN5/l4H+G2uQDynvwJemv+7fgd4bZ4N8WyvDfwW/3f9DfDawC7Phni21wZ+i/+b/gZ4bWCX54R4Tn8NvBT/dzwD+Gzgu3n+EM/pOPDewGsDx/nf6a+BW4G/Bn6bFw7x/xvi/zfE/2+I/98Q/78hntNLA98FvDT//f4a2AV+G/gaYJf/eIhnezDwV8Bx/ufZBT4G+G7+YyGe7bWB3+J/ttcBfpv/OIhne2ngr/ifbRd4CLDLfwzEc/ps4LP4n+1zgM/mPwbieb00cJz/Xg8G3hp4K57XrcBD+I+B+J/tq4GP4nm9DPDX/Psh/mc7Dlzkeb0O8Nv8+yH+5zPP62OAr+bfD/Hf6zjwVcBLA7cCHwPcyrN9F/DePK/XAX6bfz/Ef6/fAl6bZ9sFvhr4beCjgbfm+TsB7PLvh/jvZf71fgZ4a/5jIP577QLHeNFdAl4auJX/GIj/Xu8NfBcvmkvAWwO/zX8cxH+/1wZ+GjjGC/Y3wFsDt/IfC/E/w3Hgq4GXBl6KZ/sd4KuBn+Y/B+J/nuPAceBW/vMh/n9D/P+G+P8N8f8b4v83xP9viP/f+EePq5NBV0HXDgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3YvDbwXV3wP8Nf813pp4L244nuAv+ZfD/Fv89LAX/Gc3gf4bv5rvDfwXTynlwH+mn8dxL/NVwMfxfN6H+C7+c/13sB38by+Bvho/nUQ/zZfDXwUz9/7AN/Nf473Br6L5+9rgI/mXwfxb/PSwF/xgr0P8N38x3pv4Lt4wV4G+Gv+dRD/du8NfBcv2PsA381/jPcGvosX7H2A7+ZfD/Hv897Ad/GCvQ/w3fz7vDfwXbxg7wN8N/82iH+/9wa+ixfsfYDv5t/mvYHv4gV7H+C7+bdD/Md4b+C7eMHeB/hu/nXeG/guXrD3Ab6bfx/Ef5z3Br6LF+x9gO/mRfPewHfxgr0P8N38+yH+Y7038F28YO8DfDcv3HsD38UL9j7Ad/MfA/Ef772B7+IFex/gu3n+3hv4Ll6w9wG+m/84iP8c7w18Fy/Y+wDfzXN6b+C7eMHeB/hu/mMh/vO8N/BdvGDvA3w3V7w38F28YO8DfDf/8RD/ud4b+C5esPfhiu/iBXsf4Lv5z4H4z/fewHfxb/M+wHfznwfxX+O9ge/iX+d9gO/mPxfiv857A9/Fi+Z9gO/mPx/iv9Z7A9/FC/c+wHfzXwPxX+u9ge/ihXsf4Lv5r4H4r/PewHfxonkf4Lv5z4f4r/HewHfxr/M+wHfznwvxn++9ge/i3+Z9gO/mPw/iP9d7A9/FC/Y+XPFdvGDvA3w3/zkQ/3neG/guXrD3Ab6bK94b+C5esPcBvpv/eIj/HO8NfBcv2PsA381zem/gu3jB3gf4bv5jIf7jvTfwXbxg7wN8N8/fewPfxQv2PsB38x8H8R/rvYHv4gV7H+C7eeHeG/guXrD3Ab6b/xiI/zjvDXwXL9j7AN/Ni+a9ge/iBXsf4Lv590P8x3hv4Lt4wd4H+G7+dd4b+C5esPcBvpt/H8S/33sD38UL9j7Ad/Nv897Ad/GCvQ/w3fzbIf593hv4Ll6w9wG+m3+f9wa+ixfsfYDv5t8G8W/33sB38YK9D/Dd/Md4b+C7eMHeB/hu/vUQ/zYvDfwVL9j7AN/Nf6z3Br6LF+xlgL/mXwfxb/PVwEfx/L0P8N3853hv4Lt4/r4G+Gj+dRD/Nl8NfBTP632A7+Y/13sD38Xz+hrgo/nXQfzbvDTwVzyn9wG+m/8a7w18F8/pZYC/5l8H8W/30sB7c8V3A3/Nf62XBt6bK74b+Gv+9RD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjwyYZBzEnCjwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn child_elements(&self) -> Element {
        rsx! {
            polygon {
                points: "13 2 3 14 12 14 11 22 21 10 12 10 13 2",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
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
