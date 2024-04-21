use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAppSettingsAlt;
impl IconShape for MdAppSettingsAlt {
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
                d: "M21.81 12.74l-.82-.63v-.22l.8-.63c.16-.12.2-.34.1-.51l-.85-1.48c-.07-.13-.21-.2-.35-.2-.05 0-.1.01-.15.03l-.95.38c-.08-.05-.11-.07-.19-.11l-.15-1.01c-.03-.21-.2-.36-.4-.36h-1.71c-.2 0-.37.15-.4.34l-.14 1.01c-.03.02-.07.03-.1.05l-.09.06-.95-.38c-.05-.02-.1-.03-.15-.03-.14 0-.27.07-.35.2l-.85 1.48c-.1.17-.06.39.1.51l.8.63v.23l-.8.63c-.16.12-.2.34-.1.51l.85 1.48c.07.13.21.2.35.2.05 0 .1-.01.15-.03l.95-.37c.08.05.12.07.2.11l.15 1.01c.03.2.2.34.4.34h1.71c.2 0 .37-.15.4-.34l.15-1.01c.03-.02.07-.03.1-.05l.09-.06.95.38c.05.02.1.03.15.03.14 0 .27-.07.35-.2l.85-1.48c.1-.17.06-.39-.1-.51zM18 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM17 17h2v4c0 1.1-.9 2-2 2H7c-1.1 0-2-.9-2-2V3c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2v4h-2V6H7v12h10v-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdApps;
impl IconShape for MdApps {
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
                d: "M4 8h4V4H4v4zm6 12h4v-4h-4v4zm-6 0h4v-4H4v4zm0-6h4v-4H4v4zm6 0h4v-4h-4v4zm6-10v4h4V4h-4zm-6 4h4V4h-4v4zm6 6h4v-4h-4v4zm0 6h4v-4h-4v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dxrtjlvwfiv89x4Le44nWAXf7rIf57HAd+C3hprvhr4HWAXf5rIf7rHQd+C3hpntNfA68D7PJfB/Ff6zjwW8BL8/x9DvDZ/NdB/Nc5DvwW8NI8f98DvDf/tRD/NY4DvwW8NM/f9wDvzX89xH++48BvAS/N8/c9wHvz3wPxn+s48FvAS/P8fQ/w3vz3QfznOQ78FvDSPH/fA7w3/70Q/zmOA78FvDTP3/cA781/P8R/vOPAbwEvzfP3PcB78z8D4j/WceC3gJfm+fse4L35nwPxH+c48FvAS/P8fQ/w3vzPgviPcRz4LeClef6+B3hv/udB/PsdB34LeGmev+8B3pv/mRD/PseB3wJemufve4D35n8uxL/dceC3gJfm+fse4L35nw3xb3Mc+C3gpXn+vgd4b/7nQ/zrHQd+C3hpXrDf5r/XxwB/zb8M8a9zHPgt4KX5n+11gN/mX4Z40R0Hfgt4af7nex3gt/mXIV50x4HfBl6K//leB/ht/mWIf53jwG8DL8X/bK8D/Db/MsS/3nHgt4GX4gX7Hf57fTTw1/zLEP82x4HfBl6K5++7gffhfz7Ev91x4LeBl+L5+27gffifDfHvcxz4beCleP6+G3gf/udC/PsdB34beCmev+8G3of/mRD/MY4Dvw28FM/fdwPvw/88iP84x4HfBl6K5++7gffhfxbEf6zjwG8DL8Xz993A+/A/B+I/3nHgt4GX4vn7buB9+J8B8Z/jOPDbwEvx/H038D7890P85zkO/DbwUjx/3w28D/+9EP+5jgO/DbwUz993A+/Dfx/Ef77jwG8DL8Xz993A+/DfA/Ff4zjw28BL8fx9N/A+/NdD/Nc5Dvw28FI8f98NvA//tRD/tY4Dvw28FM/f5wCfzX8dxH+948BvAy/Fc/ob4LWBXf7rIP57HAd+G3gprvgb4LWBXf5rIf77HAd+myteG9jlvx7iv9dxrtjlvwfi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPF/1xBEVlZqwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowBack;
impl IconShape for MdArrowBack {
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
                d: "M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4b+G3+Z0H81/gu4L2B9wG+m/85EP/5vgt4b57tfYDv5n8GxH+u7wLem+f1PsB3898P8Z/nu4D35gV7H+C7+e+F+M/xXcB788L9DfDS/PdC/Mf7LuC9eeH+BnhtYJf/Xoj/WN8FvDcv3N8Arw3s8t8P8R/nu4D35oX7G+C1gV3+Z0D8x/gu4L154f4GeG1gl/85EP9+3wW8Ny/c3wCvDezyPwvi3+e7gPfmhfsb4LWBXf7nQfzbfRfw3rxwfwO8NrDL/0yIf5vvAt6bF+5vgNcGdvmfC/Gv913Ae/PC/Q3w2sAu/7Mh/nW+C3hvXri/AV4b2OV/PsSL7ruA9+aF+xvgtYFd/ndAvGi+C3hvXri/AV4b2OV/D8S/7LuA9+aF+xvgtYFd/ndBvHDfBbw3L9zfAK8N7PK/D+IF+y7gvXnh/gZ4bWCX/50Qz993Ae/NC/c3wGsDu/zvhXhe3wW8Ny/c3wCvDezyvxviOX0X8N68cH8DvDawy/9+iGf7LuC9eeH+BnhtYJf/GxBXfBfw3rxwfwO8NrDL/x0I+Gjgq3jh/gZ4bWCX/1sQ8N7Ad/HC/TXwOsAu/7cgrnhv4Lt44f4aeB1gl/87EM/23sB38cL9NfA6wC7/NyCe03sD38UL99fA6wC7/O+HeF7vDXwXL9xfA68D7PK/G+L5e2/gu3jh/hp4HWCX/70QL9h7A9/FC/fXwOsAu/zvhHjh3hv4Ll64vwZeB9jlfx/Ev+y9ge/ihftr4HWAXf53Qbxo3hv4Ll64vwZeB9jlfw/Ei+69ge/ihftr4HWAXf53QPzrvDfwXbxwfw28DrDL/3yIf733Br6LF+6vgdcBdvmfDfFv897Ad/HC/TXwOsAu/3Mh/u3eG/guXri/Bl4H2OV/JsS/z3sD38UL99fA6wC7/M+D+Pd7b+C7eOH+GngdYJf/WRD/Md4b+C5euL8GXgfY5X8OxH+c9wa+ixfur4HXAXb5nwHxH+u9ge/ihftr4HWAXf77If7jvTfwXbxwfw28DrDLfy/Ef473Br6LF+6vgZfhvxfiP897A9/FC/Y+wHfz3wvxn+u9ge/ieb0P8N3890P853tv4Lt4tvcBvpv/GRD/Nd4b+C7gfYDv5n8OxH+dBwO38j8L4v83xP9viP/fEP+/If5/4x8BoruUQUeP3CUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowBackIos;
impl IconShape for MdArrowBackIos {
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
                d: "M11.67 3.87L9.9 2.1 0 12l9.9 9.9 1.77-1.77L3.54 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dLA8d4TpeAv+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+Lc5DnwU8Dn82/w28Fo8p98BXpt/m88CvgbY5V8H8a93HPgt4KWB7wbeh3+93wZei+f0O8Br86/3XcB7A38NvA6wy4sO8a9zHPgt4KV5tu8G3od/nd8GXovn9DvAa/Ov813Ae/Nsfw28DrDLiwbxojsO/Bbw0jyv7wbehxfdbwOvxXP6HeC1edF9F/DePK+/Bl4H2OVfhnjRHQd+G3gpnr/vBt6HF81vA6/Fc/od4LV50XwX8N48f38DvDawy78M8a9zHPht4KV4/r4beB/+Zb8NvBbP6XeA1+Zf9l3Ae/P8/Q3w2sAuLxrEv95x4LeBl+L5+27gfXjhfht4LZ7T7wCvzQv3XcB78/z9DfDawC4vOsS/zXHgt4GX4vn7buB9eMF+G3gtntPvAK/NC/ZdwHvz/P0N8NrALv86iH+748BvAy/F8/fdwPvw/P028Fo8p98BXpvn77uA9+b5+xvgtYFd/vUQ/z7Hgd8GXorn77uB9+F5/TbwWjyn3wFem+f1XcB78/z9DfDawC7/Noh/v+PAbwMvxfP33cD78Jx+G3gtntPvAK/Nc/ou4L15/v4GeG1gl387xH+M48BvAy/F8/fdwPvwbL8NvBbP6XeA1+bZvgt4b56/vwFeG9jl3wfxH+c48NvAS/H8fTfwPlzx28Br8Zx+B3htrvgu4L15/v4GeG1gl38/xH+s48BvAy/F8/fdwPsAvw28Fs/pd4DXBr4LeG+ev78BXhvY5T8G4j/eceC3gZfi+ftu4CHAa/Gcfgd4OvDePH9/A7w2sMt/HMR/juPAbwMvxX+MvwFeG9jlPxbiP89x4LeBl+Lf52+A1wZ2+Y+H+M91HPht4KX4t/kb4LWBXf5zIP7zHQd+G3gp/nX+BnhtYJf/PIj/GseB3wZeihfN3wCvDezynwvxX+c48NvAS/HC/Q3w2sAu//kQ/7WOA78NvBTP398Arw3s8l8D8V/vOPDbwEvxnP4GeG1gl/86iP8ex4HfBl6KK/4GeG1gl/9aiP8+x4Hf5orXBnb5r4f473WcK3b574H4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wgDs6hBGFcD9AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowDownward;
impl IconShape for MdArrowDownward {
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
                d: "M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+I/1mvxX+N3+I+B+I/13sB38Z/rfYDv5j8G4j/eewPfxX+O9wG+m/84iP8c7w18F/+x3gf4bv5jIf7zvDfwXfzHeB/gu/mPh/jP9d7Ad/Hv8z7Ad/OfA/Gf772B7+Lf5n2A7+Y/D+K/xnsD38W/zvsA381/LsR/nfcGvosXzfsA381/PsR/rfcGvosX7n2A7+a/BuK/3nsD38Xz9z7Ad/NfB/Hf472B7+I5vQ/w3fzXQvz3eW/gu7jifYDv5r8e4r/XewPHga/mvwfi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CDmnGkFRjpbvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowDropDown;
impl IconShape for MdArrowDropDown {
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
                d: "M7 10l5 5 5-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5t3sGcCv/sR4MPIh/uxPALs8J8fy9NvBb/NvtAq8D/DX/MV4a+C3gOP92rwP8Ns8J8fx9NPBV/PvsAq8D/DX/Pi8N/BZwnH+fjwG+mueEeP4+G/gs/v12gdcB/pp/m5cGfgs4zr/f5wCfzXNCPH+fDXwW/zF2gdcB/pp/nZcGfgs4zn+MzwE+m+eEeP5+G3gt/uPsAq8D/DUvmpcGfgs4zn+cnwHemueEeP5+G3gt/mPtAq8D/DUv3EsDvwUc5z/WzwBvzXNCPH+fDXwW//F2gdcB/prn76WB3wKO8x/vc4DP5jkhnr/PBj6L/xy7wOsAf81zemngt4Dj/Of4HOCzeU6I5++jga/iP88u8DrAX3PFSwO/BRznP8/HAF/Nc0I8f68N/Bb/uXaB1+GK3wKO85/rdYDf5jkhnr8HA0/nP98uVxznP98JYJfnhHjB/hp4Kf5v+BvgpXleiBfsq4GP4v+GrwE+mueFeMFeG/gt/m94HeC3eV6IF+5W4EH87/YM4ME8f4gX7rOBz+J/t88BPpvnD/HCHQduBY7xv9Ml4MHALs8f4l/22cBn8b/T5wCfzQuG+JcdB24FjvG/yyXgwcAuLxjiRfPewHfxv8vbAD/NC4d40f008Fb87/AzwFvzL0O86I4Dfw08iP/ZngG8NLDLvwzxr/PSwG8Dx/if6RLw2sBf86JB/Ou9NPDbwDH+Z7kEvDbw17zoEP82Lw38NnCM/xkuAa8N/DX/Ooh/u5cGfhp4EP+9ngG8NfDX/Osh/n2OA98NvBX/PX4GeG9gl38bxH+M9wa+GjjGf41LwHsDP82/D+I/znHgo4GPBo7xn+MS8NXAVwO7/Psh/uMdBz4aeG/gQfzHeAbw3cBXA7v8x0H853pt4K2B1wZein+dvwF+G/hp4Lf5z4H4r3MceGngpYHjXPHSXPHXXLEL/DXw18Au//kQ/78h/n9D/P+G+P8N8f8b/wiu8LBBIN3m7gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowDropDownCircle;
impl IconShape for MdArrowDropDownCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 12l-4-4h8l-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvjv9V1c8T7890D89/ku4L254ruB9+G/HuK/x3cB781z+m7gffivhfiv913Ae/P8fTfwPvzXQfzX+i7gvXnhvht4H/5rIP7rfBfw3rxovht4H/7zIf5rfBfw3vzrfDfwPvznQvzn+y7gvfm3+W7gffjPg/jP9V3Ae/Pv893A+/CfA/Gf57uA9+Y/xncD78N/PMR/ju8C3pv/WN8NvA//sRD/8b4LeG/+c3w38D78x0H8x/ou4L35z/XdwPvwHwPxH+c48NL81/hrYJd/P8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hH63kbQdsX6JkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowDropUp;
impl IconShape for MdArrowDropUp {
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
                d: "M7 14l5-5 5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADs0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dxrtjlvwfiv89x4Le44nWAXf7rIf57HAd+C3hprvhr4HWAXf5rIf7rHQd+C3hpntNfA68D7PJfB/Ff77OBz+L5+2vgdYBd/msg/nt8N/BePH9/DbwOsMt/PsR/n+8G3ovn76+B1wF2+c+F+O/13cB78fz9NfA6wC7/eRD//b4beC+ev78GXgfY5T8H4n+G7wbei+fvr4HXAXb5j4f4n+O7gffi+ftr4HWAXf5jIf5n+W7gvXj+/hp4HWCX/ziI/3m+G3gvnr+/Bl4H2OU/BuJ/pu8G3ovn76+B1wF2+fdD/M/13cB78fz9NfA6wC7/Poj/2b4beC+ev78GXgfY5d8O8T/fdwPvxfP318DrALv82yD+d/hu4L14/v4aeB1gl389xP8e3w28F8/fXwOvA+zyr4N40b008FX893ptXrC/Bl4H2OVFh3jRvTbwW/zP9tfA6wC7vGgQL7rXBn6L//n+GngdYJd/GeJF99rAb/E/398Arw3s8i9DvOheG/gt/mf7G+C1gV1eNIgX3UsDX81/r9fiBfsb4LWBXV50iP89vgt4b56/vwFeG9jlXwfxv8N3Ae/N8/c3wGsDu/zrIf7n+y7gvXn+/gZ4bWCXfxvE/2zfBbw3z9/fAK8N7PJvh/if67uA9+b5+xvgtYFd/n0Q/zN9F/DePH9/A7w2sMu/H+J/nu8C3pvn72+A1wZ2+Y+B+J/lu4D35vn7G+C1gV3+4yD+5/gu4L15/v4GeG1gl/9YiP8Zvgt4b56/vwFeG9jlPx7iv993Ae/N8/c3wGsDu/znQPz3+i7gvXn+/gZ4bWCX/zyI/z7fBbw3z9/fAK8N7PKfC/Hf47uA9+b5+xvgtYFd/vMh/ut9NvBZPH9/A7w2sMt/DcR/vePAbwMvxXP6G+C1gV3+6yD+exwHfht4Ka74G+C1gV3+ayH++xwHfpsrXhvY5b8e4r/Xca7Y5b8H4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjHc9+QcXIO0oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowForward;
impl IconShape for MdArrowForward {
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
                d: "M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEQElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8Y4DHwV8Dv/zIf5jHQd+C3hp4LuB9+F/NsR/nOPAbwEvzbN9N/A+/M+F+I9xHPgt4KV5Xt8NvA//MyH+/Y4DvwW8NC/YdwPvw/88iH+f48BvAS/Nv+y7gffhfxbEv89x4LeBl+JF893A+/A/B+Lf7zjw28BL8aL5buB9+J8B8R/jOPDbwEvxovlu4H3474f4j3Mc+G3gpXjRfDfwPvz3QvzHOg78NvBSvGi+G3gf/vsg/uMdB34beCleNN8NvA//PRD/OY4Dvw28FC+a7wbeh/96iP88x4HfBl6KF813A+/Dfy3Ef67jwG8DL8WL5ruB9+G/DuI/33Hgt4GX4kXz3cD78F8D8V/jOPDbwEvxovlu4H34z4f4r3Mc+G3gpXjRfDfwPvznQvzXOg78NvBSvGi+G3gf/vMg/usdB34beCleNN8NvA//ORD/PY4Dvw28FC+a7wbeh/94iP8+x4HfBl6KF813A+/DfyzEf6/jwG8DL8WL5ruB9+E/DuK/33Hgt4GX4kXz3cD78B8D8T/DceC3gZfiRfPdwPvw74f4n+M48NvAS/Gi+W7gffj3QfzPchz4beCleNF8N/A+/Nsh/mc5DvwW8NK8aN4H+G7+7RD/cxwHfgt4aV407wN8N/8+iP8ZjgO/Bbw0L5r3Ab6bfz/Ef7/jwG8BL82L5n2A7+Y/BuK/13Hgt4CX5kXzPsB38x8H8d/nOPBbwEvzonkf4Lv5j4X473Ec+C3gpXnRvA/w3fzHQ/zXOw78FvDSvGjeB/hu/nMg/msdB34LeGleNO8DfDf/eRD/dY4DvwW8NC+a9wG+m/9ciP8ax4HfAl6aF837AN/Nfz7Ef77jwG8BL82L5n2A7+a/BuI/13Hgt4CX5kXzPsB3818H8Z/nOPBbwEvzonkf4Lv5r4X4z3Ec+C3gpXnRvA/w3fzXQ/zHOw78FvDSvGjeB/hu/nsg/mMdB34LeGleNO8DfDf/fRD/cY4DvwW8NC+a9wG+m/9eiP8Yx4HfAl6aF837AN/Nfz/Ev99x4LeAl+ZF8z7Ad/M/A+Lf5zjwW8BL86J5H+C7+Z8D8e9zHPht4KX4l70P8N38z4L49zsO/DbwUrxg7wN8N//zIP5jHAd+G3gpntf7AN/N/0yI/zjHgd8GXopnex/gu/mfC/Ef6zjw28BLAe8DfDf/syH+4x0HXhv4af7nQ/z/hvj/DfH/G+L/N8T/b/wj3xxqQRgW+qoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowForwardIos;
impl IconShape for MdArrowForwardIos {
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
                d: "M5.88 4.12L13.76 12l-7.88 7.88L8 22l10-10L8 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAClklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4n+m48Au//kQ//N8F/A9wG/znw/xP8t3Ae8NvA7w2/znQ/zP8V3Ae3PF6wC/zX8+xP8M3wW8N8/2OsBv858P8d/vu4D35jm9DvDb/OdD/Pf6LuC9eV6vA/w2//kQ/32+C3hvnr/XAX6b/3yI/x7fBbw3L9jrAL/Nfz7Ef73vAt6bF+51gN/mPx/iv9Z3Ae/Nv+x1gN/mPx/iv853Ae/Ni+Z1gN/mPx/iv8Z3Ae/Ni+51gN/mPx/iP99XAx/Fv87rAL/Nfz7Ef773Br6Lf53XAX6b/3yI/xrvDXwXL7rXAX6b/3yI/zrvDXwXL5rXAX6b/3yI/1rvDXwX/7LXAX6b/3yI/3rvDXwXL9zrAL/Nfz7Ef4/3Br6LF+x1gN/mPx/iv897A9/F8/c6wG/znw/x3+u9ge/ieb0O8Nv850P893tv4Lt4Tq8D/Db/+RD/M7w38F082+sAv81/PsT/HO8NfBdXvA7w2/znQ/zP8t7AdwGvA/w2//kQ//O8N3Ar8Nv850P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IbKY1QVcbKroAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowLeft;
impl IconShape for MdArrowLeft {
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
                d: "M24 0v24H0V0h24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAClklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4r/OcWCX/1kQ/3VeG3gv4H34nwPxX+e1gd8Cvht4H/5nQPzXeW3gt7jiu4H34b8f4r/OawO/xbN9N/A+/PdC/Nd5beC3eE7fDbwP/30Q/3VeG/gtntd3A+/Dfw/Ef53XBn6L5++7gffhvx7iv85rA7/FC/bdwPvwXwvxX+e1gd/ihftu4H34r4P4r/PawG/xL/tu4H34r4H4r/PawG/xovlu4H34z4f4r/PawG/xovtu4H34z4X4r/PawG/xr/PdwPvwnwfxX+e1gd/iX+d9gO/mPw/iv85rA7/Fi+59gO/mPxfiv85rA7/Fi+Z9gO/mPx/iv85rA7/Fv+x9gO/mvwbiv85rA7/FC/c+wHfzXwfxX+e1gd/iBXsf4Lv5r4X4r/PawG/x/L0P8N3810P813lt4Ld4Xu8DfDf/PRD/dV4b+C2e0/sA381/H8R/ndcGfotnex/gu/nvhfiv89rAb3HF+wDfzX8/xH+d1wZ+C3gf4Lv5nwHxX+e1gQcD383/HIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BdE8zQWzDAb0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowRight;
impl IconShape for MdArrowRight {
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
                d: "M0 24V0h24v24H0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dxrtjlvwfiv89x4Le44nWAXf7rIf57HAd+C3hprvhr4HWAXf5rIf7rHQd+C3hpntNfA68D7PJfB/Ff6zjwW8BL8/z9NfA6wC7/NRD/dY4DvwW8NC/cXwOvA+zynw/xX+M48FvAS/Oi+WvgdYBd/nMh/vMdB34LeGn+df4aeB1gl/88iP9cx4HfAl6af5u/Bl4H2OU/B+I/z3Hgt4CX5t/nr4HXAXb5j4f4z3Ec+C3gpfmP8dfA6wC7/MdC/Mc7DvwW8NI8f98DPBh4LZ7T7wC3Au/F8/fXwOsAu/zHQfzHOg78FvDSPH/fA7w38NvAa/Gcfgd4beC7gffi+ftr4HWAXf5jIP7jHAd+C3hpnr/vAd6bK34beC2e0+8Ar80V3w28F8/fXwOvA+zy74f4j3Ec+C3gpXn+vgd4b57tt4HX4jn9DvDaPNt3A+/F8/fXwOsAu/z7IP79jgO/Bbw0z9/3AO/Nc/pt4LV4Tr8DvDbP6buB9+L5+2vgdYBd/u0Q/z7Hgd8CXprn73uA9+Z5/TbwWjyn3wFem+f13cB78fz9NfA6wC7/Noh/u+PAbwEvzfP3PcB78/z9NvBaPKffAV6b5++7gffi+ftr4HWAXf71EP82x4HfAl6a5+97gPfmBftt4LV4Tr8DvDYv2HcD78Xz99fA6wC7/Osg/vWOA78FvDTP3/cA780L99vAa/Gcfgd4bV647wbei+fvr4HXAXZ50SH+dY4DvwW8NM/f9wDvzb/st4HX4jn9DvDa/Mu+G3gvnr+/Bl4H2OVFg3jRHQd+C3hpnr/vAd6bF81vA6/Fc/od4LV50Xw38F48f38NvA6wy78M8aI7Dvw28FI8r+8B3psX3W8Dr8Vz+h3gtXnRfTfwXjyvvwFeG9jlX4b41zkO/DbwUjzb9wDvzb/ObwOvxXP6HeC1+df5buC9eLa/AV4b2OVFg/jXOw78NvBSwPcA782/3m8Dr8Vz+h3gtfnX+27gvYC/AV4b2OVFh/i3OQ58NPDZ/Nv8NvBaPKffAV6bf5vPBr4a2OVfB/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pd4aeA4z2kX+Gv+ayH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIiBqhBFMc5lwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowUpward;
impl IconShape for MdArrowUpward {
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
                d: "M4 12l1.41 1.41L11 7.83V20h2V7.83l5.58 5.59L20 12l-8-8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssistantDirection;
impl IconShape for MdAssistantDirection {
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
                d: "M14 10H9c-.6 0-1 .4-1 1v4h2v-3h4v2.5l3.5-3.5L14 7.5V10zm-2-9C5.9 1 1 5.9 1 12s4.9 11 11 11 11-4.9 11-11S18.1 1 12 1zm7.73 11.58l-7.19 7.22c-.35.27-.79.27-1.15 0L4.2 12.58c-.27-.36-.27-.8 0-1.16l7.19-7.22c.35-.27.79-.27 1.15 0l7.19 7.22c.36.27.36.8 0 1.16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssistantNavigation;
impl IconShape for MdAssistantNavigation {
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
                d: "M12 1C5.93 1 1 5.93 1 12s4.93 11 11 11 11-4.93 11-11S18.07 1 12 1zm3.57 16L12 15.42 8.43 17l-.37-.37L12 7l3.95 9.63-.38.37z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/5bjwC4vOsT/DQ8GfoorXoYXHeJ/v7cGvgs4zhVfA3w0LxrE/25fBXw0z+ttgJ/mX4b43+nBwE8BL83ztwu8DHArLxzif5+3Br4LOM4L9j3ARwO7vHCI/xneGnhr4L154b4K+GhesEvARwPfzYsG8d/rwcB3Aa8N/A7w2rxgx4FbgWM8f38DvDfw17zoEP89jgMfBXw2z/Y7wGvzwr028Fs8r+8BPhrY5V8H8V/vrYGvAh7Mc/od4LX5l3028FlccQn4aOC7+bdB/Nd5MPBdwGvz/P0O8Nq8aH4bOA68N/DX/Nsh/vMdBz4K+GxeuN8BXpsXzXGu2OXfB/Gf662BrwIezL/sd4DX5r8W4j/Hg4HvAl6bF93vAK/NFR8NfDX/+RAv3HHgo4D3Bh7Mf67fAV6bK34beDrwPvznQrxgDwZ+C3gw/zV+B3htrvht4LWA9wG+m/88iBfsr4CX5r/O7wCvzRW/DbwWcCvwMsAu/zkQz99bAz/Ff63fAV6bK34beC2ueB/gu/nPgXj+Phv4LP5r/Q7w2lzx28BrccXPAG/Nfw7E8/fbwGvxX+t3gNfmit8GXotnEy/Yb/Gi+Rvgo3lOiOfvt4HX4r/W7wCvzRW/DbwWz/YywF/z/JkXze8Ar81zQjx/vw28Fv+1fgd4ba74beC1eLbXAX6b58+8aH4HeG2eE+L5+23gtfiv9TvAa3PFbwOvxbO9DvDbPH/mRfM7wGvznBDP328Dr8V/rd8BXpsrfht4LZ7tdYDf5vkzL5rfAV6b54R4/n4beC3+a/0O8Npc8dvAa/Fs4gX7bV40fw18NM8J8fx9NvBZ/Nf6HeC1ueK3gdfiip8B3pr/HIjn762Bn+K/1u8Ar80Vvw28Fle8D/Dd/OdAvGB/DbwU/3V+B3htrvht4LWAZwAvDezynwPxgj0Y+Gngpfiv8TvAa3PFbwOvBbwN8NP850G8cMeBjwbeG3gQ/7l+B3htrvht4FbgvfnPhfi3+W3gtXhOvwO8Nlc8GPhu4LV40f0O8Npc8d7Ad/OfD/Fv89vAa/Gcfgd4bZ7TWwNfDTyIf9nvAK/Nfy3Ev81vA6/Fc/od4LV5XseBjwY+ixfud4DX5kVznCt2+fdB/Nv8NvBaPKffAV6bF+zBwHcDr8Xz9zvAa/Oi+S3gOPA+wF/zb4f4t/lt4LV4Tr8DvDb/srcGvhp4EM/pd4DX5l/22cBnccUu8DHAd/Nvg/i3+W3gtXhOvwO8Ni+a48BHA5/Fs/0O8Nq8cK8N/BbP67uBjwF2+ddB/Nv8NvBaPKffAV6bf50HA98NvBbwO8Br84IdB54OHOf5+2vgfYC/5kWH+Lf5beC1eE6/A7w2/zZvDbw18N68cF8NfBQv2C7wMcB386JB/Nv8NvBaPKffAV6b/3xvDXw3cIwX7LuBjwF2eeEQ/za/DbwWz+l3gNfmv8aDgZ8GXorn7xLw0sCtvHCIf5uvBl6a5/TXwEfzX+urgY/ieb0N8NP8yxD/+7018N3AMa74GuCjedEg/m94MPDTXPHSvOgQ/7ccB3Z50SH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLYW7hBQeu4EwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCampaign;
impl IconShape for MdCampaign {
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
                d: "M18 11v2h4v-2h-4zm-2 6.61c.96.71 2.21 1.65 3.2 2.39.4-.53.8-1.07 1.2-1.6-.99-.74-2.24-1.68-3.2-2.4-.4.54-.8 1.08-1.2 1.61zM20.4 5.6c-.4-.53-.8-1.07-1.2-1.6-.99.74-2.24 1.68-3.2 2.4.4.53.8 1.07 1.2 1.6.96-.72 2.21-1.65 3.2-2.4zM4 9c-1.1 0-2 .9-2 2v2c0 1.1.9 2 2 2h1v4h2v-4h1l5 3V6L8 9H4zm11.5 3c0-1.33-.58-2.53-1.5-3.35v6.69c.92-.81 1.5-2.01 1.5-3.34z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCancel;
impl IconShape for MdCancel {
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
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/N/y3sBHAa8D7PIvQ/zf8d7Ad3HFXwOvA+zywiH+b3hv4Lt4Tn8NvA6wywuG+N/vvYHv4vn7HeC1ecEQ/7u9N/BdPH+XgNcG/poXDPG/13sD38Xzdwl4beCveeEQ/zu9N/BdPH+XgNcG/pp/GeJ/n/cGvovn7xLw2sBf86JB/O/y3sB38fxdAl4b+GtedIj/Pd4b+C6ev0vAawN/zb8O4n+H9wa+i+fvEvDawF/zr4f4n++9ge/i+bsEvDbw1/zbIP5ne2/gu3j+LgGvDfw1/3aI/7neG/gunr9LwGsDf82/D+J/pvcGvovn7xLw2sBf8++H+J/nvYHv4vm7BLw28Nf8x0D8z/LewHfx/F0CXhv4a/7jIP71vgv4HeC7+Y/13sB38fxdAl4b+Gv+YyH+db4LeG+ueB/gu/mP8d7Ad/H8XQJeG/hr/uMhXnTfBbw3z+l9gO/m3+e9ge/i+bsEvDbw1/znQLxovhr4KJ6/9wG+m3+b9wa+i+fvEvDawF/znwfxonlp4LeBYzx/7wN8N/867w18F8/fJeC1gb/mPxfiRffSwG8Dx3j+3gf4bl407w18F8/fJeC1gb/mPx/iX+elgd8GjvH8vQ/w3bxw7w18F8/fJeC1gb/mvwbiX++lgd8GjvH8vQ/w3Tx/7w18F8/fJeC1gb/mvw7i3+algd8GjvH8vQ/w3Tyn9wa+i+fvEvDawF/zXwvxb/fSwG8Dx3j+3gf4bq54b+C7eP4uAa8N/DX/9RD/Pi8N/DZwjOfvfbjiu3j+LgGvDfw1/z0Q/34vDfw2cIx/nUvAawN/zX8fxH+MlwZ+GzjGi+YS8NrAX/PfC/Ef56WB3waO8cJdAl4b+Gv++yH+Y7008NvAMZ6/S8BrA3/N/wyI/3gvDfw2cIzndAl4beCv+Z8D8Z/jpYHfBo5xxSXgtYG/5n8WxH+elwZ+myteG/hr/udB/Od6aa74a/5nQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I7gqfEFHeQXEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheck;
impl IconShape for MdCheck {
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
                d: "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/npYGvAt4G2OU/F+J/lpcGfgs4Dvw18DrALv95EP9zvDTwW8Bxnu2vgdcBdvnPgfif4aWB3wKO87z+GngZ/nMg/vu9NPBbwHGev/cBvpv/HIj/Xi8N/BZwnOfvfYDv5j8P4r/PSwO/BRzn+Xsf4Lv5z4X47/HSwG8Bx3n+3gf4bv7zIf7rvTTwW8Bxnr/3Ab6b/xqI/1ovDfwWcJzn732A7+a/DuK/zksDvwUc5/l7H+C7+a+F+K/x0sBvAcd5/t4H+G7+6yH+87008FvAcZ6/9wG+m/8eiP9cLw38FnCc5+99gO/mvw/iP89LA78FHOf5ex/gu/nvhfjP8dLAbwHHef7eB/hu/vsh/nO8NPDbwDGev/cBvpv/foj/PC8N/DZwjOfvfYDv5r8X4j/XSwO/DRzj+Xsf4Lv574P4z/fSwG8Dx3j+3gf4bv57IP5rvDTw28Axnr/3Ab6b/3qI/zovDfw2cIzn732A7+a/FuK/1ksDvw0c4/l7H+C7+a+D+K/30sBvA8d4/t4H+G7+ayD+e7w08NvAMZ6/9wG+m/98iP8+Lw38NnCM5+99gO/mPxfiv9dLA78NHOP5ex/gu/nPg/jv99LAbwPHeP7eB/hu/nMg/md4aeC3gWM8r78BXpr/HIj/OV4a+G3gGM/2N8BrA7v850D8z/LSwG8Dx4C/AV4b2OU/D+J/npcGvhp4a2CX/1yI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjL21EQf4E52gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChevronLeft;
impl IconShape for MdChevronLeft {
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
                d: "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K9xHPgp4GOAv+Z/DsR/vuPAbwEvDewCrwP8Nf8zIP5zHQd+C3hpnm0XeB3gr/nvh/jP9VfAS/O8doHXAf6a/16I/1zvDXwXz98u8DrAX/PfB/Gf772B7+L52wVeB/hr/nsg/mu8N/BdPH+7wOsAf81/PcR/nfcGvovnbxd4HeCv+a+F+K/13sB38fztAq8D/DX/dRD/9d4b+C6ev13gdYC/5r8G4r/HewPfxfO3C7wO8Nf850P893lv4Lt4/naB1wH+mv9ciP9e7w18F8/fLvA6wF/znwfx3++9ge/i+dsFXgf4a/5zIP5neG/gu3j+doHXAf6a/3iI/zneG/gunr9d4HWAv+Y/FuJ/jvcGvovn7xLw2sBf8x8L8T/DewPfxfN3CXht4K/5j4f47/fewHfx/F0CXhv4a/5zIP57vTfwXTx/l4DXBv6a/zyI/z7vDXwXz98l4LWBv+Y/F+K/x3sD38Xzdwl4beCv+c+H+K/33sB38fxdAl4b+Gv+ayD+a7038F08f5eA1wb+mv86iP867w18F8/fJeC1gb/mvxbiv8Z7A9/F83cJeG3gr/mvh/jP997Ad/H8XQJeG/hr/nsg/nO9N/BdPH+XgNcG/pr/Poj/XH8NvBTP6xLw2sBf898L8Z/rOPDbwEvxbJeA1wb+mv9+iP98x4HfBl4KuAS8NvDX/M+A+K9xHPhp4KOBv+Z/DsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHtWl5Boy4ybgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChevronRight;
impl IconShape for MdChevronRight {
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
                d: "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/34vDbw08N3813pv4K+Bv+bfDvHv89LAbwHHgfcBvpv/Gu8NfBewC7wO8Nf82yD+7V4a+C3gOM/2PsB385/rvYHv4tl2gdcB/pp/PcS/zUsDvwUc53m9D/Dd/Od4b+C7eF67wOsAf82/DuLf5r2B7+IFex/gu/mP9d7Ad/GCvQ/w3fzrIP7t3hv4Ll6w9wG+m/8Y7w18Fy/Y+wDfzb8e4t/nvYHv4gV7H+C7+fd5b+C7eMHeB/hu/m0Q/37vDXwXL9j7AN/Nv817A9/FC/Y+wHfzb4f4j/HewHfxgr0P8N3867w38F28YO8DfDf/Poj/OO8NfBcv2PsA382L5r2B7+IFex/gu/n3Q/zHem/gu3jB3gf4bl649wa+ixfsfYDv5j8G4j/eewPfxQv2PsB38/y9N/BdvGDvA3w3/3EQ/zneG/guXrD3Ab6b5/TewHfxgr0P8N38x0L853lv4Lt4wd4H+G6ueG/gu3jB3gf4bv7jIf5zvTfwXbxg78MV38UL9j7Ad/OfA/Gf772B7+Lf5n2A7+Y/D+K/xnsD38W/zvsA381/LsR/nfcGvosXzfsA381/PsR/rfcGvosX7n2A7+a/BuK/1nsD38UL9z7Ad/NfA/Ff572B7+JF8z7Ad/OfD/Ff472B7+Jf532A7+Y/F+I/33sD38W/zfsA381/HsR/rvcGvosX7H244rt4wd4H+G7+cyD+87w38F28YO8DfDdXvDfwXbxg7wN8N//xEP853hv4Ll6w9wG+m+f03sB38YK9D/Dd/MdC/Md7b+C7eMHeB/hunr/3Br6LF+x9gO/mPw7iP9Z7A9/FC/Y+wHfzwr038F28YO8DfDf/MRD/cd4b+C5esPcBvpsXzXsD38UL9j7Ad/Pvh/iP8d7Ad/GCvQ/w3fzrvDfwXbxg7wN8N/8+iH+/9wa+ixfsfYDv5t/mvYHv4gV7H+C7+bdD/Pu8N/BdvGDvA3w3/z7vDXwXL9j7AN/Nvw3i3+69ge/iBXsf4Lv5j/HewHfxgr0P8N386yH+bd4b+C5esPcBvpv/WO8NfBcv2PsA382/DuLf5qWB3waO8bzeB/hu/nO8N/BdPK9LwGsDf82/DuLf7qWB3waO8WzvA3w3/7neG/gunu0S8NrAX/Ovh/j3eWngt4FjwPsA381/jfcGvgu4BLw28Nf82yD+/V4aeGngu/mv9d7AXwN/zb8d4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAXeSikEEZ8WbAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClose;
impl IconShape for MdClose {
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
                d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7oHA9/Fi+5W4H14/t4beC9edN8DfDf/8RD/Op8NfBYvus8BPpvn76+Bl+JF9zLAX/MfC/Gv99fAS/Giexngr3leLw38NnCMF81fA68D7PIfB/Gv92Dgr4FjvGhuBV4G2OV5fTTwVbzovgb4aP7jIP5t3hv4Ll503w28D8/fTwNvxYvubYCf5j8G4t/up4G34kX3NsBP87yOA7cCx3jR7AIPAXb590P82x0H/hp4EC+aXeBlgFt5Xq8N/BYvut8GXod/P8S/z2sDv8WL7reB1+H5+2rgo3jRfQzw1fz7IP79Phv4LF50nwN8Ns/fXwMvxYvuZYC/5t8O8R/jr4GX4kX3MsBf87xeGvht4Bgvmr8GXgfY5d8G8R/jwcBfA8d40dwKvAywy/P6aOCreNF9DfDR/Nsg/uO8N/BdvOi+G3gfnr+fBt6KF93bAD/Nvx7iP9ZPA2/Fi+5tgJ/meR0HbgWO8aLZBR4C7PKvg/iPdRz4a+BBvGh2gZcBbuV5vTbwW7zofht4Hf51EP/xXhv4LV50vw28Ds/fVwMfxYvuY4Cv5kWH+M/x2cBn8aL7HOCzef7+GngpXnQvA/w1LxrEf56/Bl6KF93LAH/N83pp4LeBY7xo/hp4HWCXfxniP8+Dgb8GjvGiuRV4GWCX5/XRwFfxovsa4KP5lyH+c7038F286L4beB+ev58G3ooX3dsAP80Lh/jP9d7Ad/Gi+x7gvXn+fgp4a150bwP8NC8c4j/Pg4G/Ao7zonkG8NLALs/ro4Gv4kX3NcBH8y9D/Of5K+CledG9DPDXPK+XBn4LOM6L5m+A1wZ2+Zch/nN8NvBZvOg+B/hsnr+/Al6aF93LAH/NiwbxH++1gd/iRfc7wGvz/H018FG86D4G+GpedIj/WMeBvwIezIvmEvDSwK08r9cGfosX3e8Ar82/DuI/1k8Bb82L7m2An+Z5HQeeDhznRXMJeDCwy78O4j/OewPfxYvue4D35vn7KeCtedG9DfDT/Osh/mM8GPgr4DgvmmcALw3s8rw+GvgqXnRfA3w0/zaI/xh/Bbw0L7qXAf6a5/XSwG8Bx3nR/A3w2sAu/zaIf7/PBj6LF93nAJ/N8/dXwEvzonsZ4K/5t0P8+7w28Fu86H4HeG2ev68GPooX3ccAX82/D+Lf7jjwV8CDedFcAl4auJXn9drAb/Gi+x3gtfn3Q/zb/RTw1rzo3gb4aZ7XceDpwHFeNJeABwO7/Psh/m3eG/guXnTfA7w3z99PAW/Ni+5tgJ/mPwbiX+/BwF8Bx3nRPAN4aWCX5/XRwFfxovsa4KP5j4P41/sr4KV50b0M8Nc8r5cGfgs4zovmb4DXBnb5j4P41/ls4LN40X0O8Nk8f38FvDQvupcB/pr/WIgX3YOB7+ZFdyvw3jx/7w28Ny+67wa+m/94iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BBVTrEGIhGf1AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoubleArrow;
impl IconShape for MdDoubleArrow {
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
            polygon {
                points: "15.5,5 11,5 16,12 11,19 15.5,19 20.5,12",
            }
            polygon {
                points: "8.5,5 4,5 9,12 4,19 8.5,19 13.5,12",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7O9NFf8Nf85EP9zvTTwW1zxOsBf8x8P8T/TSwO/BRznil3gdYC/5j8W4n+elwZ+CzjOc9oFXgf4a/7jIP7neW/gu3j+doHXAf6a/xiI/5neG/gunr9d4HWAv+bfD/E/13sD38Xztwu8DvDX/Psg/md7b+C7eP52gdcB/pp/O8T/fO8NfBfP3y7wOsBf82+D+N/hvYHv4vnbBV4H+Gv+9RD/e7w38F08f7vA6wB/zb8O4n+X9wa+i+dvF3gd4K950SH+93lv4Lt4/naB1wH+mhcN4n+n9wa+i+dvF3gd4K/5lyH+93pv4Lt4/naB1wH+mhcO8b/bewPfxfO3C7wO8Ne8YIjn76uBl+J/h5cGjvP87QKvA/w1zx/i+ftt4LX4v2EXeB3gr3leiOfvt4HX4v+OXeB1gL/mOSGev98GXov/Oy4Brw38Nc8J8fz9NvBa/N9wCXht4K95Xojn76uBl+Z/h5cGjvH8XQJeG/hrnj/E/27vDXwXz98l4LWBv+YFQ/zv9d7Ad/H8XQJeG/hrXjjE/07vDXwXz98l4LWBv+Zfhvjf572B7+L5uwS8NvDXvGgQ/7u8N/BdPH+XgNcG/poXHeJ/j/cGvovn7xLw2sBf86+D+N/hvYHv4vm7BLw28Nf86yH+53tv4Lt4/i4Brw38Nf82iP/Z3hv4Lp6/S8BrA3/Nvx3if673Br6L5+8S8NrAX/Pvg/if6b2B7+L5uwS8NvDX/Psh/ud5b+C7eP4uAa8N/DX/MRD/87w08NvAMZ7TJeC1gb/mPw7if6aXBn4bOMYVl4DXBv6a/1iI/7leGvhtrnht4K/5j4f4n+2lueKv+c+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHYglsQYimcY0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEast;
impl IconShape for MdEast {
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
                d: "M15,5l-1.41,1.41L18.17,11H2V13h16.17l-4.59,4.59L15,19l7-7L15,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+9Xpor/pr/Hoj/Pi8N/BZXvA7w1/zXQ/z3eGngt4DjXLELvA7w1/zXQvzXe2ngt4DjPKdd4HWAv+a/DuK/1ksDvwUc5/nbBV4H+Gv+ayD+67w08FvAcV64XeB1gL/mPx/iv8ZLA78FHOdFswu8DvDX/OdC/Od7aeC3gOP86+wCrwP8Nf95EP+5Xhr4LeA4z9/7cMV38fztAq8D/DX/ORD/eV4a+C3gOM/f+wDfzRXvDXwXz98u8DrAX/MfD/Gf46WB3wKO8/y9D/DdPKf3Br6L528XeB3gr/mPhfiP99LAbwHHef7eB/hunr/3Br6L528XeB3gr/mPg/iP9dLAbwHHef7eB/huXrj3Br6L528XeB3gr/mPgfiP89LAbwHHef7eB/huXjTvDXwXz98u8DrAX/Pvh/iP8dLAbwHHef7eB/hu/nXeG/gunr9d4HWAv+bfB/Hv99LAbwHHef7eB/hu/m3eG/gunr9d4HWAv+bfDvHv99vAa/H8vQ/w3fz7vDfwXTx/vwO8Nv92iH+/48BvAy/Fc3of4Lv5j/HewHfxnP4GeG1gl387xH+M48BvAy/FFe8DfDf/sd4b+C6u+BvgtYFd/n0Q/3GOA78NfDXw3fzneG/go4HXBnb590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPLjBOQTCgKTcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExpandLess;
impl IconShape for MdExpandLess {
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
                d: "M12 8l-6 6 1.41 1.41L12 10.83l4.59 4.58L18 14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/zHOQ78FvA1wHfzn+O9gY8CXgfY5d8P8R/jOPBbwEtzxfsA381/rPcGvosr/hp4HWCXfx/Ev99x4LeAl+Y5vQ/w3fzHeG/gu3hOfw28DrDLvx3i3++3gdfi+Xsf4Lv593lv4Lt4/n4HeG3+7RD/fi8N/DZwjOfvfYDv5t/mvYHv4vm7BLw28Nf82yH+Y7w08NvAMZ6/9wG+m3+d9wa+i+fvEvDawF/z74P4j/PSwG8Dx3j+3gf4bl407w18F8/fJeC1gb/m3w/xH+ulgd8GjvH8vQ/w3bxw7w18F8/fJeC1gb/mPwbiP95LA78NHOP5ex/gu3n+3hv4Lp6/S8BrA3/NfxzEf46XBn4bOMbz9z7Ad/Oc3hv4Lp6/S8BrA3/NfyzEf56XBn4bOMbz9z7Ad3PFewPfxfN3CXht4K/5j4f4z/XSwG8Dx3j+3ocrvovn7xLw2sBf858D8Z/vpYHfBo7xr3MJeG3gr/nPg/iv8dLAbwPHeNFcAl4b+Gv+cyH+67w08NvAMV64S8BrA3/Nfz7Ef62XBn4bOMbzdwl4beCv+a+B+K/30sBvA8d4TpeA1wb+mv86iP8eLw38NnCMKy4Brw38Nf+1EP99Xhr4ba54beCv+a+H+O/10lzx1/z3QPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNwMk5BgXw0hgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExpandMore;
impl IconShape for MdExpandMore {
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
                d: "M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEG0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pczz+t1gN/mP8dLA18FvA2wy78P4t/PPK/XAX6b/3gvDfwWcBz4a+B1gF3+7RD/fuZ5vQ7w2/zHemngt4DjPNtfA68D7PJvg/j3M8/rdYDf5j/OSwO/BRznef018DL82yD+/czzeh3gt/mP8dLAbwHHef7eB/hu/m0Q/37meb0O8Nv8+7008FvAcZ6/9wG+m387xL+feV6vA/w2/z4vDfwWcJzn732A7+bfB/HvZ57X6wC/zb/dSwO/BRzn+Xsf4Lv590P8+5nn9TrAb/Nv89LAbwHHef7eB/hu/mMg/v3M83od4Lf513tp4LeA4zx/7wN8N/9xEP9+5nm9DvDb/Ou8NPBbwHGev/cBvpv/WIh/P/O8Xgf4bV50Lw38FnCc5+99gO/mPx7i3888r9cBfpsXzUsDvwUc5/l7H+C7+c+B+Pczz+t1gN/mX/bSwG8Bx3n+3gf4bv7zIP79zPN6HeC3eeFeGvgt4DjP3/sA381/LsS/n3lerwP8Ni/YSwO/BRzn+Xsf4Lv5z4f49zPP63WA3+YFe2ngt4FjPH/vA3w3//kQ/37meb0O8Nu8cC8N/DZwjOfvfYDv5j8X4t/PPK/XAX6bf9lLA78NHOP5ex/gu/nPg/j3M8/rdYDf5kXz0sBvA8d4/t4H+G7+cyD+/czzeh3gt3nRvTTw28Axnr/3Ab6b/3iIfz/zvF4H+G3+dV4a+G3gGM/f+wDfzX8sxL+feV6vA/w2/3ovDfw2cIzn732A7+Y/DuLfzzyv1wF+m3+blwZ+GzjG8/c+wHfzHwPx72ee1+sAv82/3UsDvw0c4/l7H+C7+fdD/PuZ5/U6wG/z7/PSwG8Dx3j+3gf4bv59EP9+5nm9DvDb/Pu9NPDbwDGev/cBvpt/O8S/n3lerwP8Nv8xXhr4beAYz9/7AN/Nvw3i3888r9cBfpv/OC8N/DZwjOf1N8BL82+D+Pczz+t1gN/mP9ZLA78NHOPZ/gZ4bWCXfxvEv595Xq8D/Db/8V4a+G3gGPA3wGsDu/zbIf79zPN6HeC3+c/x0sBXA28N7PLvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJloYRBkFzUkgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFirstPage;
impl IconShape for MdFirstPage {
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
                d: "M24 24H0V0h24v24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zZfDbwU/zp/A3w0L5qvBl6Kf52/AT6afx3Ev81vA6/Fv87vAK/Ni+a3gdfiX+d3gNfmXwfxb/PbwGvxr/M7wGvzovlt4LX41/kd4LX510H82/w28Fr86/wO8Nq8aH4beC3+dX4HeG3+dRD/Nr8NvBb/Or8DvDYvmt8GXot/nd8BXpt/HcS/zW8Dr8VzugT8NS/YXwMfzYvmq4GX5gV7aeAYz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nt8NfDSPKe/Bj6a/xpfDbw0z+mvgY/mXwfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xL/NVwMvxXP6G+Cj+a/x1cBL8Zz+Bvho/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXP6HeC1+a/x28Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmv8ZvA6/Fc/od4LX510H82/w28Fo8p98BXpv/Gr8NvBbP6XeA1+ZfB/Fv89vAa/Gcfgd4bf5r/DbwWjyn3wFem38dxL/NbwOvxXPaBf6aF+xvgI/mRfPVwEvxgr00cJzn9DvAa/Ovg/i3+W3gtfjX+R3gtXnR/DbwWvzr/A7w2vzrIP5tfht4Lf51fgd4bV40vw28Fv86vwO8Nv86iH+b3wZei3+d3wFemxfNbwOvxb/O7wCvzb8O4t/mt4HX4l/nd4DX5kXz28Br8a/zO8Br86+D+Lf5auCl+df5a+CjedF8NfDS/Ov8NfDR/Osg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RA3l0QcQ72loAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFullscreen;
impl IconShape for MdFullscreen {
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
                d: "M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zm+GngpntPfAB/Ni+argZfiOf0N8NH8x0L85/ht4LV4Tr8DvDYvmt8GXovn9DvAa/MfC/Gf47eB1+I5/Q7w2rxofht4LZ7T7wCvzX8sxH+O3wZei+f0O8Br86L5beC1eE6/A7w2/7EQ/zl+G3gtntPvAK/Ni+a3gdfiOf0O8Nr8x0L85/ht4LV4Tr8DvDYvmt8GXovn9DvAa/MfC/Gf47eB1+I5/Q7w2rxofht4LZ7T7wCvzX8sxH+O3wZei+f0O8Br86L5beC1eE6/A7w2/7EQ/zZfDbwUL9hLA8d5Tr8DvDYvmt8GXovntAv8NS/Y3wAfzb8O4t/mt4HX4l/nd4DX5kXz28Br8a/zO8Br86+D+Lf5beC1+Nf5HeC1edH8NvBa/Ov8DvDa/Osg/m1+G3gt/nV+B3htXjS/DbwW/zq/A7w2/zqIf5vfBl6Lf53fAV6bF81vA6/Fv87vAK/Nvw7i3+argZfmX+evgY/mRfPVwEvzr/PXwEfzr4P4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/mq4GX4l/nb4CP5kXz1cBL8a/zN8BH86+D+Lf5beC1+Nf5HeC1edH8NvBa/Ov8DvDa/Osg/m1+G3gt/nV+B3htXjS/DbwW/zq/A7w2/zqIf5vfBl6Lf53fAV6bF81vA6/Fv87vAK/Nvw7i3+a3gdfiX+d3gNfmRfPbwGvxr/M7wGvzr4P4t/lq4KV5wV4aOMZz+h3gtXnR/DbwWjynS8Bf84L9NfDR/Osg/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cXw28NM/pr4GP5kXz1cBL85z+Gvho/mMh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4Rg1t0Qep0FqcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFullscreenExit;
impl IconShape for MdFullscreenExit {
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
                d: "M5 16h3v3h2v-5H5v2zm3-8H5v2h5V5H8v3zm6 11h2v-3h3v-2h-5v5zm2-11V5h-2v5h5V8h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHomeWork;
impl IconShape for MdHomeWork {
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
                d: "M8.17 5.7L1 10.48V21h5v-8h4v8h5V10.25z",
            }
            path {
                d: "M17 7h2v2h-2z",
            }
            path {
                d: "M10 3v1.51l2 1.33L13.73 7H15v.85l2 1.34V11h2v2h-2v2h2v2h-2v4h6V3H10zm9 6h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+I9xHPgp4GOAv+Y/x2sDv8XzEv92iH+/48BvAS8N7AKvA/w1//FeG/gtnpf4t0P8+xwHfgt4aZ5tF3gd4K/5j/XawG/xvMS/HeLf56+Al+Z57QKvA/w1/3FeG/gtnpf4t0P8+7w38F08f7vA6wB/zX+M1wZ+i+cl/u0Q/37vDXwXz98u8DrAX/Pv99rAb/G8xL8d4j/GewPfxfO3C7wO8Nf8+7w28Fs8L/Fvh/iP897Ad/H87QKvA/w1/3avDfwWz0v82yH+Y7038F08f7vA6wB/zb/NawO/xfMS/3aI/3jvDXwXz98u8DrAX/Ov99rAb/G8xL8d4j/HewPfxfO3C7wO8Nf867w28Fs8L/Fvh/jP897Ad/H87QKvA/w1L7rXBn6L5yX+7RD/ud4b+C6ev13gdYC/5kXz2sBv8bzEvx3iP997A9/F87cLvA7w1/zLXhv4LZ6X+LdD/Nd4b+C7eP52gdcB/poX7rWB3+J5iX87xH+d9wa+i+dvF3gd4K95wV4b+C2el/i3Q/zXeW/gu3j+LgGvDfw1L9hrA7/F8xL/doj/Gu8NfBfP3yXgtYG/5oV7beC3eF7i3w7xn++9ge/i+bsEvDbw1/zLXhv4LZ6X+LdD/Od6b+C7eP4uAa8N/DUvmtcGfovnJf7tEP953hv4Lp6/S8BrA3/Ni+61gd/ieYl/O8R/jvcGvovn7xLw2sBf86/z2sBv8bzEvx3iP957A9/F83cJeG3gr/nXe23gt3he4t8O8R/rvYHv4vm7BLw28Nf827w28Fs8L/Fvh/iP897Ad/H8XQJeG/hr/u1eG/gtnpf4t0P8x3hv4Lt4/i4Brw38Nf8+rw38Fs9L/Nsh/v3eG/gunr9LwGsDf82/32sDv8XzEv92iH+f9wa+i+fvEvDawF/zH+O1gd/ieYl/O8S/z18DL8XzugS8NvDX/Md5beC3eF7i3w7x73Mc+G3gpXi2S8BrA3/Nf6zXBn6L5yX+7RD/fseB3wZeCrgEvDbw1/zHe23gt3he4t8O8R/jOPDTwEcDf81/jtcGfovnJf7tEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4Rxd/fkHOiBRXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLastPage;
impl IconShape for MdLastPage {
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
                d: "M5.59 7.41L10.18 12l-4.59 4.59L7 18l6-6-6-6zM16 6h2v12h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zWOAx8FfA9wK/+5HswVt/IvQ/zne2vgu4DjwC7wMcB385/jvYGvAt4G+G3+ZYj/PA8Gvgt4bZ7XTwPvA+zyH+M48F3AW3PF6wC/zb8M8R/vOPBRwGfzwt0KvA/w2/z7vDbwXcCDebbXAX6bfxniP9ZbA18FPJgX3WcDn8O/zWcBn83zeh3gt/mXIf5jPBj4KuCt+bf5a+BtgFt50bw08F3AS/P8vQ7w2/zLEP9+nwV8NHCcF+4ZwIN4wXaBzwa+hhfuo4DPBo7zgr0O8Nv8yxD/dq8NfBfwYF64ZwDvDfw28NHAV/HC/TTwPsAuz+k48F3AW/PCfQzw1bxoEP96x4GvAt6bf9nnAJ/Nc3pp4LuBl+IF2wXeBvhtrnht4KeA47xgfwO8N/DXvOgQ/zofBXw2cJwX7neA9wZu5fk7Dnw28FG8cF/NFR/NC/c1wGcDu/zrIF50rw38Fi/cM4CPBn6aF81bA98NHOPf5hLw3sBP82+DeNG9NvBbvGBfA3w2sMu/znHgp4HX4l/nZ4D3Bnb5t0O86F4b+C2e1+8AHw38Nf8+Hw18NnCMF+4S8NnAV/Pvh3jRvTbwWzzbJeCjge/mP85LA98NvBTP398A7w38Nf8xEC+61wZ+iyu+BvhsYJf/HF8NfBTP6WuAj+Y/FuJF99rAVwMfDfw2//leG/hprnhr4Lf5j4d40R0HdvmvdZwrdvnPgfj/DfH/G+L/N8T/b4gX3XHgpfjf4W+AXf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9D/P+G+P8N8f8b4v83xP9viBfdSwNfxf8OHwP8Nf8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheGvhq/nf4aOCv+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4R1C5zQcu+Qq8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLegendToggle;
impl IconShape for MdLegendToggle {
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
                d: "M20,15H4v-2h16V15z M20,17H4v2h16V17z M15,11l5-3.55L20,5l-5,3.55L10,5L4,8.66L4,11l5.92-3.61L15,11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAByUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8S/7Lf53ex1eMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/st/nf7bV5wRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If9lr8b/b7/CCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/AARJBumSyaQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMenu;
impl IconShape for MdMenu {
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
                d: "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADXUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/xl+B3ht/mMh/mXmf4bfAV6b/1iIf5n5n+F3gNfmX/Zg4FZeNIh/mfmf4XeA1+aF+y7grYHXAf6afxniX2b+Z/gd4LV5wb4LeG+u2AVeB/hrXjjEv+y1+Z9hF/hrnr/vAt6b57QLvA7w17xgiP/9vgt4b56/3wFemxcM8b/bdwHvzfP3N8BrA7u8YIj/vb4LeG+ev78BXhvY5YVD/O/0XcB78/z9DfDawC7/MsT/Pt8FvDfP398Arw3s8qJB/O/yXcB78/z9DfDawC4vOsT/Ht8FvDfP398Arw3s8q+D+Jf9Fv81/gb4aJ6/7wLem+fvb4DXBnb510P8y8x/jd8BXpvn9V3Ae/P8/Q3w2sAu/zaIf5n5r/E7wGvznL4LeG+ev78BXhvY5d8O8S8z/zV+B3htntN3A+/F8/fXwOsAu/zbIf5l5r/G7wCvzfP6buC9eP7+GngdYJd/G8S/7Lf5r/HXwEfz/H038F48f38NvA6wy78e4n+P7wbei+fvr4HXAXb510H87/LdwHvx/P018DrALi86xP8+3w28F8/fXwOvA+zyokH87/TdwHvx/P018DrALv8yxP9e3w28F8/fXwOvA+zywiH+d/tu4L14/v4aeB1glxcM8b/fdwPvxfP3O8Br84Ih/mWvxf8Ml4C/5vn7buC9eE6XgNcG/poXDPEvM/8z/A7w2rxg3w28F1dcAl4b+GteOMS/zPzP8DvAa/PCfTfw1sBrA3/NvwzxLzP/M/wO8Nr8yx4M3MqLBvEvM/8z/A7w2vzHQvzLzP8MvwO8Nv+xEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzpIcEFeclzkAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMenuOpen;
impl IconShape for MdMenuOpen {
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
                d: "M3 18h13v-2H3v2zm0-5h10v-2H3v2zm0-7v2h13V6H3zm18 9.59L17.42 12 21 8.41 19.59 7l-5 5 5 5L21 15.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+Lf7sHAg7jiGcCt/Os8GHgQVzwDuJV/nQcDD+KKZwC38q+H+Nc7DnwX8NY8p58G3gfY5YU7DnwX8NY8p58G3gfY5YU7DnwX8NY8p58G3gfY5UWH+Nc5DjwdOM7ztws8BNjl+TsOPB04zvO3CzwE2OX5Ow48HTjO87cLPATY5UWD+Nf5KeCteeF+Gngbnr+fAt6aF+6ngbfh+fsp4K154X4aeBteNIgX3YOBp/OieQhwK8/pwcDTedE8BLiV5/Rg4Om8aB4C3Mq/DPGie23gt3jRvA7w2zyn1wZ+ixfN6wC/zXN6beC3eNG8DvDb/MsQL7rXBn6LF83rAL/Nc3pt4Ld40bwO8Ns8p9cGfosXzesAv82/DPGiezDwdF40DwFu5Tk9GHg6L5qHALfynB4MPJ0XzUOAW/mXIf51fhp4K164nwHemufvp4G34oX7GeCtef5+GngrXrifAd6aFw3iX+c4cCtwjOfvb4DXBnZ5/o4DtwLHeP7+BnhtYJfn7zhwK3CM5+9vgNcGdnnRIP71jgPfDbwVz+lngPcGdnnhjgPfDbwVz+lngPcGdnnhjgPfDbwVz+lngPcGdnnRIf7tHgw8mCtuBW7lX+fBwIO54lbgVv51Hgw8mCtuBW7lXw/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EVxvVEEEANHIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoreHoriz;
impl IconShape for MdMoreHoriz {
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
                d: "M6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADLElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8+DgQdxxTOAW/mvh/ivdxz4LuCteU4/DbwPsMt/HcR/rePA04HjPH+7wEOAXf5rIP5r/RTw1rxwPw28Df81EP91Hgw8nRfNQ4Bb+c+H+K/z2sBv8aJ5HeC3+c+H+K/z2sBv8aJ5HeC3+c+H+K/zYODpvGgeAtzKfz7Ef62fBt6KF+5ngLfmvwbiv9Zx4FbgGM/f3wCvDezyXwPxX+848N3AW/GcfgZ4b2CX/zqI/z4PBh7MFbcCt/JfD/H/G+L/N8T/b4j/3xD/fR4MPIgrngHcyn89xH+948B3AW/Nc/pp4H2AXf7rIP5rHQeeDhzn+dsFHgLs8l8D8V/rp4C35oX7aeBt+K+B+K/zYODpvGgeAtzKfz7Ef53XBn6LF83rAL/Nfz7Ef53XBn6LF83rAL/Nfz7Ef50HA0/nRfMQ4Fb+8yH+a/008Fa8cD8DvDX/NRD/tY4DtwLHeP7+BnhtYJf/Goj/eseB7wbeiuf0M8B7A7v810H893kw8GCuuBW4lf96iP/fEP+/If5/Q/z/hvjv82DgQVzxDOBW/ush/usdB74LeGue008D7wPs8l8H8V/rOPB04DjP3y7wEGCX/xqI/1o/Bbw1L9xPA2/Dfw3Ef50HA0/nRfMQ4Fb+8yH+67w28Fu8aF4H+G3+8yH+67w28Fu8aF4H+G3+8yH+6zwYeDovmocAt/KfD/Ff66eBt+KF+xngrfmvgfivdRy4FTjG8/c3wGsDu/zXQPzXOw58N/BWPKefAd4b2OW/DuK/z4OBB3PFrcCt/NdD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EXG9UQV/jYwEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoreVert;
impl IconShape for MdMoreVert {
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
                d: "M12 8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vV6aK/6a/x6I/z4vDfwWV7wO8Nf810P893hp4LeA41yxC7wO8Nf810L813tp4LeA4zynXeB1gL/mvw7iv9ZLA78FHOf52wVeB/hr/msg/uu8NPBbwHFeuF3gdYC/5j8f4r/GSwO/BRznRbMLvA7w1/znQvzne2ngt4Dj/OvsAq8D/DX/eRD/uV4a+C3gOP82u8DrAH/Nfw7Ef56XBn4LOM6/zy7wOsBf8x8P8Z/jpYHfAo7z/F0CjvGcLgHHeP52gdcB/pr/WIj/eC8N/BZwnOfvfYD3Bl6L5/Q7wHcD38Xztwu8DvDX/MdB/Md6aeC3gOM8f+8DfDfw28Br8Zx+B3ht4L2B7+L52wVeB/hr/mMg/uO8NPBbwHGev/cBvpsrfht4LZ7T7wCvzRXvDXwXz98u8DrAX/Pvh/iP8dLAbwHHef7eB/hunu23gdfiOf0O8No823sD38Xztwu8DvDX/Psg/v1eGvgt4DjP3/sA381z+m3gtXhOvwO8Ns/pvYHv4vnbBV4H+Gv+7RD/Pi8N/BZwnOfvfYDv5nn9NvBaPKffAV6b5/XewHfx/O0CrwP8Nf82iH+7lwZ+CzjO8/c+wHfz/P028Fo8p98BXpvn772B7+L52wVeB/hr/vUQ/zYvDfwWcJzn732A7+YF+23gtXhOvwO8Ni/YewPfxfO3C7wO8Nf86yD+bV4a+G3gGM/rfYDv5oX7beC1eE6/A7w2L9x7A9/F87oEvDbw1/zrIP7tXhr4beAYz/Y+wHfzL/tt4LV4Tr8DvDb/svcGvotnuwS8NvDX/Osh/n1eGvht4BjwPsB386L5beC1eE6/A7w2L5r3Br4LuAS8NvDX/Nsg/v1eGnhp4Lt50f028Fo8p98BXpsX3XsDfw38Nf92iP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HVwEvznP4a+Gj+ayH+f0P8/4b4/w3x/xvi/zf+EWq9r0FDL1cpAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNorth;
impl IconShape for MdNorth {
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
                d: "M5,9l1.41,1.41L11,5.83V22H13V5.83l4.59,4.59L19,9l-7-7L5,9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//l+i/8afwN8NP86iP985r/G7wCvzb8O4j+f+a/xO8Br86+D+M9n/mv8DvDa/Osg/vOZ/xq/A7w2/zqI/3y/zb/fSwPHeOF+B3ht/nUQ//O9N/Bd/Mt+B3ht/nUQ/7O9N/BdvGh+B3ht/nUQ/3O9N/BdvOh+B3ht/nUQ/zO9N/BdvGCXgGM8p98BXpt/HcT/PO8NfBcv2PsA7w28Fs/pd4DX5l8H8T/LewPfxQv2PsB3A78NvBbP6XeA1+ZfB/E/x3sD38UL9j7Ad3PFbwOvxXP6HeC1+ddB/M/w3sB38YK9D/DdPNtvA6/Fc/od4LX510H893tv4Lt4wd4H+G6e028Dr8Vz+h3gtfnXQfz3em/gu3jB3gf4bp7XbwOvxXP6HeC1+ddB/Pd5b+C7eMHeB/hunr/fBl6L5/Q7wGvzr4P47/HewHfxgr0P8N28YL8NvBbP6XeA1+ZfB/Ff772B7+IFex/gu3nhfht4LZ7T7wCvzb8O4r/WewPfxQv2PsB38y/7beC1eE6/A7w2/zqI/zrvDXwXL9j7AN/Ni+a3gdfiOf0O8Nr86yD+a7w38F28YO8DfDcvut8GXovn9DvAa/Ovg/jP997Ad/GCvQ/w3fzr/DbwWjyn3wFem38dxH+u9wa+ixfsfYDv5l/vt4HX4jn9DvDa/Osg/vO8N/BdvGDvA3w3/za/DbwWz+l3gNfmXwfxn+O9ge/iBXsf4Lv5t/tt4LV4Tr8DvDb/Ooj/eO8NfBcv2PsA382/z28Dr8Vz+h3gtfnXQfzHem/gu3jB3gf4bv7nQPzHeW/gu3jB3gf4bv5nQfzHeG/gu3jB3gf4bv7nQfz7vTfwXbxg7wN8N/8zIf593hv4Ll6w9wG+m/+5EP927w18Fy/Y+wDfzf9siH+b9wa+ixfsfYDv5n8+xL/eewPfxQv2PsB3878D4l/nvYHv4gV7H+C7+d8D8aJ7MPB0XrD3Ab6b/10Q/zrvDXwXz+t9gO/mfx/Ev957A9/Fs70P8N3874T4t3lv4LuA9wG+m/+9EP92DwZu5X83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHrlBzQb3SJZ4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNorthEast;
impl IconShape for MdNorthEast {
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
                d: "M9,5v2h6.59L4,18.59L5.41,20L17,8.41V15h2V5H9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD90lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zZfDbwU/zVeh/88iH+b3wZei/8a4j8P4t/mt4HX4r+G+M+D+Lf5beC1+K8h/vMg/m1+G3gt/muI/zyIf5vfBl6LF+4S8Nf8+702/3kQ/za/DbwW/7L3Ab6b/7kQ/za/DbwWL5r3Ab6b/5kQ/za/DbwWL7r3Ab6b/3kQ/za/DbwWz+kScIwX7H2A7+Z/FsS/zW8Dr8Vz+h3gu4Hv4gV7H+C7+Z8D8W/z28Br8Zx+B3ht4L2B7+IFex/gu/mfAfFv89vAa/Gcfgd4ba54b+C7eMHeB/hu/vsh/m1+G3gtntPvAK/Ns7038F28YO8DfDf/vRD/Nr8NvBbP6XeA1+Y5vTfwXbxg7wN8N/99EP82vw28Fs/pd4DX5nm9N/BdvGDvA3w3/z0Q/za/DbwWz+l3gNfm+Xtv4Lt4wd4H+G7+6yH+bX4beC2e0+8Ar80L9t7Ad/GCvQ/w3fzXQvzb/DbwWjyn3wFemxfuvYHv4gV7H+C7+a+D+Lf5beC1eE6/A7w2/7L3Br6LF+x9gO/mvwbi3+a3gdfiOf0O8Nq8aN4b+C5esPcBvpv/fIh/m98GXovn9DvAa/Oie2/gu3jB3gf4bv5zIf5tfht4LZ7T7wCvzb/OewPfxQv2PsB3858H8W/z28Br8Zx+B3ht/vXeG/guXrD3Ab6b/xyIf5vfBl6L5/Q7wGvzb/PewHfxgr0P8N38x0P82/w28Fo8p98BXpt/u/cGvosX7H2A7+Y/FuLf5reB1+I5/Q7w2vz7vDfwXbxg7wN8N/9xEP/zvDfwXbxg7wN8N/8xEP8zvTfwXbxg7wN8N/9+iP+53hv4Ll6w9wG+m38fxP9s7w18Fy/Y+wDfzb8d4n++9wa+ixfsfYDv5t8G8b/DewPfxQv2PsB386+H+N/jvYHv4gV7H+C7+ddB/O/y3sB38YK9D/DdvOgQ//u8N/BdvGDvA3w3LxrE/07vDXwXL9hDgFv5lyH+93pv4Lt4Xu8DfDcvGsT/bu8NfBfP9j7Ad/OiQ/zv997AdwHvA3w3/zqI/xseDNzKvx7i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COT0XBBpQVp5wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNorthWest;
impl IconShape for MdNorthWest {
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
                d: "M5,15h2V8.41L18.59,20L20,18.59L8.41,7H15V5H5V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOfflineShare;
impl IconShape for MdOfflineShare {
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
                d: "M14.6 10.26v1.31L17 9.33 14.6 7.1v1.28c-2.33.32-3.26 1.92-3.6 3.52.83-1.13 1.93-1.64 3.6-1.64zM16 23H6c-1.1 0-2-.9-2-2V5h2v16h10v2zm2-22h-8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 15h-8V4h8v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXsw8CD+57oE/DX/PojndRz4LuCt+Z/vVuB9gN/m3wbxnI4DTweO87/L+wDfzb8e4jn9FPDW/O+zCzwE2OVfB/FsDwaezv9eHwN8Nf86iGd7beC3+N/re4D35l8H8WyvDfwW/3v9DvDa/Osgnu21gd/if6/fAV6bfx3Es7028Fv85/gdnu3BwIP4j/c7wGvzvF6L5/U3wC6AeLbXBn6L/zjPAD4b+G6e10sDnw28Ff9xfgd4bZ6XeV6vA/w2gHi21wZ+i/8Y3wO8N/+y9wa+GjjGv9/vAK/N8zLP63WA3wYQz/bawG/x7/c9wHvzontr4Kf49/sd4LV5XuZ5vQ7w2wDi2V4b+C3+fZ4BPJh/vc8GPot/n98BXpvnZZ7X6wC/DSCe7bWB3+Lf532A7+Zf7zhwK3CMf7vfAV6b52We1+sAvw0gnu21gd/i3+4ScJx/u58G3op/u98BXpvnZZ7X6wC/DSCe7bWB3+Lf7neA1+bf7rOBz+Lf7neA1+Z5mef1OsBvA4hne23gt/i3+x3gtfm3e2vgp/i3+x3gtXle5nm9DvDbAOLZXhv4Lf7tfgd4bf7tPhv4LP7tfgd4bZ6XeV6vA/w2gHi21wZ+i3+7W4GH8G/31cBH8W/3O8Br87zM83od4LcBxLO9NvBb/Pu8DPDX/Ns8HXgw/3a/A7w2z8s8r9cBfhtAPNtrA7/Fv89PA2/Dv957A9/Fv8/vAK/N8zLP63WA3wYQz/bawG/x7/c+wHfzojsOPB04zr/P7wCvzfMyz+t1gN8GEM/22sBv8e+3C7wN8Nv8y44DvwW8NP9+vwO8Ns/LPK/XAX4bQDzbawO/xX+czwa+Btjl+Xsv4KuB4/zH+B3gtXle5nm9DvDbAOLZXhv4Lf5j7QK/Dfw18NfASwPHgbcGHsx/rN8BXpvnZZ7X6wC/DSCe7bWB3+J/r98BXpvnZZ7X6wC/DSCe7bWB3+J/r98BXpvnZZ7X6wC/DSCe7bWB3+J/r58B3prnZZ7X6wC/DSCe7aWBv+J/r88BPpvnZZ7X6wC/DSCe063Ag/jf6SHArTwv87xeB/htAPGcXhv4Lf73+Rrgo3n+zPN6HeC3AcTzem/gq4Fj/O/wNcBH84KZ5/U6wG8DiOfvOPDewEsDD+Z/nl3gr4HvBm7lhTPP63WA3wYQ//eZ5/U6wG8DiP/7zPN6HeC3AcT/feZ5vQ7w2wDi/z7zvF4H+G0A8X+feV6vA/w2gHjRvTbwW/zf8DrAbwOIF91rA7/F/w0PAW4FEC+61wZ+i//9fgZ4a65AvOheG/gt/nf7G+C1gV2uQLzoXhv4Lf73+hngvYFdng3xojsOvDT/O90K3MrzQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BBqhtkE6CxAWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPayments;
impl IconShape for MdPayments {
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
                d: "M19 14V6c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zm-9-1c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm13-6v11c0 1.1-.9 2-2 2H4v-2h17V7h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHjvHCvA/w2z+m1gd/iP9cu8NPA5wC38i9DvGjeG/guXnSvA/w2z+m1gd/iv877AN/NC4f4lz0YeDr/Oq8D/DbP6bWB3+K/1kOAW3nBEP+y7wbei3+d1wF+m+f02sBv8V/re4D35gVD/MsuAsf513kd4Ld5Tq8N/Bb/tXaBE7xgiH+Z+dd7HeC3eU6vDfwW//XEC4b4l5l/vdcBfpvn9NrAb/FfT7xgiH+Z+dd7HeC3eU6vDfwW//XEC4b4l5l/vdcBfpvn9NrAb/FfT7xgiH+Z+dd7HeC3eU6vDfwW//XEC4b4l5l/vdcBfpvn9NrAb/FfT7xgiH+Z+dd7HeC3eU6vDfwW//XEC4b4l5l/vdcBfpvn9NrAb/FfT7xgiH/Za/Ov99fALs/pOPDS/Nf7bV4wxP9viP/fEP+7fRdXvA//Noj/vb4LeG+u+G7gffjXQ/zLXot/vb8BdvnP813Ae/Ocvht4H/51EP8y86/3OsBv85/ju4D35vn7buB9eNEh/mXmX+91gN/mP953Ae/NC/fdwPvwokH8y8y/3usAv81/rO8C3psXzXcD78O/DPEvM/96rwP8Nv9xvgt4b/51vht4H144xL/M/Ou9DvDb/Mf4LuC9+bf5buB9eMEQ/zLzr/c6wG/z7/ddwHvz7/PdwPvw/CH+ZeZf73WA3+bf57uA9+Y/xncD78PzQvzLzL/e6wC/zb/dewPvzQu2BRSeUwMOeMG+G/hunhPiX2b+9V4H+G3+8/w28Fo8p98BXpt/HcS/zPzrvQ7w2/zn+W3gtXhOvwO8Nv86iH+Z+dd7HeC3+dd5b+C7edH8NvBaPKffAV6bfx3Ev8z8670O8Nu86L4LeAjw2rxofht4LZ7T7wCvzb8O4l9m/vVeB/htXjTfBbw38DvAa/Oi+W3gtXhOXwN8NP86iH+Z+dd7HeC3+Zd9F/DeXPE7wGvzonk68GCe0+cAn82/DuJfZv71Xgf4bV647wLem2f7HeC1+Ze9NvBbPK/3Ab6bfx3Ev8z8670O8Nu8YN8FvDfP6XeA1+Zf9lfAS/O8TgC7/Osg/mXmX+91gN/m+fsu4L15Xr8DvDYv3HcB783z+hngrfnXQ/zLzL/e6wC/zfP6LuC9ef5+B3htnr/XBr4KeGmev4cAt/Kvh/iXmX+91wF+m+f0XcB784L9BbDP83ow8GBesK8BPpp/G8S/zPzrvQ7w2zzbdwHvzQv318BL86/zN8BL82+H+JeZf73XAX6bK74LeG/+ZX8NvDQvur8BXhvY5d8O8S8z/3qvA/w2V7w38F38y/4aeGleNF8DfDT/foh/mfnXex3gt3m29wa+ixfur4GX5oX7GeCjgVv5j4H4l5l/vdcBfpvn9N7Ad/GC/QVwwHP6a2AXuBX4aWCX/1iIf9kucIx/ndcBfpvn9d7Ad/H8/Q7w2vzXQvzLvht4L/51Xgf4bZ6/9wa+i+f1O8Br818L8S97MPB0/nVeB/htXrD3Br6L5/Q7wGvzXwvxonlv4Lt40b0O8Nu8cO8NfBfP9jvAa/NfC/GiezDw2cBbA8d44V4H+G3+Ze8NfBdX/A7w2vzXQvz3e2/gu4DfAV6b/1qI/xneG3hv4LX5r4X4n+Olgb/mvxbi/zfE/2+I/98Q/78h/n/jHwE2a81BnERT5wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPivotTableChart;
impl IconShape for MdPivotTableChart {
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
                d: "M10 8h11V5c0-1.1-.9-2-2-2h-9v5zM3 8h5V3H5c-1.1 0-2 .9-2 2v3zm2 13h3V10H3v9c0 1.1.9 2 2 2zm8 1l-4-4 4-4zm1-9l4-4 4 4z",
            }
            path {
                d: "M14.58 19H13v-2h1.58c1.33 0 2.42-1.08 2.42-2.42V13h2v1.58c0 2.44-1.98 4.42-4.42 4.42z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dLA8d4TpeAv+Y/xnFgl38Z4r/GWwFvDbw28GBeuFuBnwZ+G/gZ/vW+C/ge4Lf5lyH+8zwY+CzgrYHj/NvsAl8NfA9wK/+y7wLeG3gd4Lf5lyH+4x0HPgv4aP5jfTbwNcAuz993Ae/NFa8D/Db/MsR/rNcGfgo4zn+OW4H3AX6b5/RdwHvzbK8D/Db/MsR/nM8CPpv/Gp8NfA5XfBfw3jyn1wF+m38Z4j/GdwHvzb/sZ4CfBm4FdoG/5oqXBo4DLw28NvBW/Mu+myvem+f1OsBv8y9D/Pt9F/DevGCXgI8GfhrY5UVzHPho4KOBY/zrvQ7w2/zLEP8+nw18Fi/Y5wBfDezyb3Mc+Gzgo/jXeR3gt/mXIf7tXhv4LZ6/S8BrA3/Nf4y3Br4bOMaL5nWA3+Zfhvi3OQ78FfBgntffAK8N7PIf66WBv+JF8zrAb/MvQ/zbfDbwWTyvS8CDgV3+430X8N68aF4H+G3+ZYh/vQcDT+f5exngr/mP913Ae/Oiex3gt/mXIf71Phv4LJ7X5wCfzX+87wLem3+d1wF+m38Z4l/vInCc53QJeDCwy3+s7wLem3+91wF+m38Z4l/nrYGf4nm9D/Dd/Mf6LuC9+bd5HeC3+Zch/nW+GvgontcJYJf/ON8FvDf/dq8D/Db/MsS/zl8BL81z+hngrfmP813Ae/Pv8zrAb/MvQ/zrmOf1PsB38x/ju4D35t/vdYDf5l+GeNG9NPBXPK/XAX6bf7/jwEvzH+OvgV3+ZYgX3WsDv8Xzehngr/nfCfGie23gt3he4n8vxIvutYHf4nmJ/70QL7rXBn6L5/UywF/zvxPiRffSwF/xvF4H+G3+d0L865jn9T7Ad/O/E+Jf56+Bl+I5/Qzw1vzvhPjX+Wrgo3heJ4Bd/msdB36K5/U9wHfzokH867w18FM8r/cBvpv/Wh8NfBXP62WAv+ZFg/jX2wWO8Zx2gYcAu/zXeTrwYJ7TM4AH86JD/Ot9NvBZPK/PAT6b/xqfDXwWz+tzgM/mRYf413sw8HSev5cB/pr/XC8N/BXP6xLwYGCXFx3i3+azgc/iee0CDwF2+c9xHHg6cJzn9TnAZ/Ovg/i3OQ78NfAgntdfA68D7PIf6zjwW8BL87yeATyYfz3Ev91rA7/F87cLvA7w1/zHeGngt4DjPH8vA/w1/3qIf5/PBj6LF+yzga8Bdvm3+yzgs3nB3gf4bv5tEP9+3w28Fy/YLvDRwM8Au7xojgPvBXw08GBesO8B3pt/O8R/jO8G3ot/2U8DPw3cClwC/porHgw8CHhp4LWB1waO88J9DfDR/Psg/uN8NvBZ/Nd4H+C7+fdD/Md6beC7gQfxn+NvgPcG/pr/GIj/eMeBzwY+iv84l4CvBj6b/1iI/zwPBj4aeG/gGP82l4CvBr4a2OU/HuK/xlsDrw28NfAgXri/AX4b+Gngt/nPhfjv8dLAcZ7TrcCt/NdC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8Erjy1QVdM3VUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRefresh;
impl IconShape for MdRefresh {
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
                d: "M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Pb4aeCme098AH81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pu9NPDSwHfzovtt4LV4Tr8DvDYvuvcG/hr4a/7tEP8+Lw38FnAceB/gu3nR/DbwWjyn3wFemxfNewPfBewCrwP8Nf82iH+7lwZ+CzjOs70P8N38y34beC2e0+8Ar82/7L2B7+LZdoHXAf6afz3Ev81LA78FHOd5vQ/w3bxwvw28Fs/pd4DX5oV7b+C7eF67wOsAf82/DuLf5qWB3waO8fy9D/DdvGC/DbwWz+l3gNfmBXtv4Lt4/i4Brw38Nf86iH+7lwZ+GzjG8/c+wHfz/P028Fo8p98BXpvn772B7+L5uwS8NvDX/Osh/n1eGvht4BjP3/sA383z+m3gtXhOvwO8Ns/rvYHv4vm7BLw28Nf82yD+/V4a+G3gGM/f+wDfzXP6beC1eE6/A7w2z+m9ge/i+bsEvDbw1/zbIf5jvDTw28Axnr/3Ab6bZ/tt4LV4Tr8DvDbP9t7Ad/H8XQJeG/hr/n0Q/3FeGvht4BjP3/sA380Vvw28Fs/pd4DX5or3Br6L5+8S8NrAX/Pvh/iP9dLAbwPHeP7eB/hu4LeB1+I5/Q7w2sB7A9/F83cJeG3gr/mPgfiP99LAbwPHeP7eB3hv4LV4Tr8DfDfwXTx/l4DXBv6a/ziI/xwvDfw2cIznbxc4znPaBY7z/F0CXhv4a/5jIf7zvDTw28Ax/n0uAa8N/DX/8RD/uV4a+G3gGP82l4DXBv6a/xyI/3wvDfw2cIx/nUvAawN/zX8exH+NlwZ+GzjGi+YS8NrAX/OfC/Ff56WB3waO8cJdAl4b+Gv+8yH+a7008NvAMZ6/S8BrA3/Nfw3Ef72XBn4bOMZzugS8NvDX/NdB/Pd4aeC3gWNccQl4beCv+a+F+O/z0sBvc8VrA3/Nfz3Ef6+X5oq/5r8H4v83xP9viP/fEP+/If5/4x8BE/CvQeLgDCEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSouth;
impl IconShape for MdSouth {
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
                d: "M19,15l-1.41-1.41L13,18.17V2H11v16.17l-4.59-4.59L5,15l7,7L19,15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+7BwK3874b4t3lv4LuA9wG+m/+9EP967w18F8/2PsB3878T4l/nvYHv4nm9D/Dd/O+DeNE9GHg6L9j7AN/N/y6If533Br6LF+x9gO/mfw/Ev957A9/FC/Y+wHfzvwPi3+a9ge/iBXsf4Lv5nw/xb/fewHfxgr0P8N38z4b493lv4Lt4wd4H+G7+50L8+7038F28YO8DfDf/MyH+Y7w38F28YO8DfDf/8yD+47w38F28YO8DfDf/syD+Y7038F28YO8DfDf/cyD+47038F28YO8DfDf/Pr8NvBbP6XeA1+ZfB/Gf472B7+IFex/gu/m3+23gtXhOvwO8Nv86iP887w18Fy/Y+wDfzb/NbwOvxXP6HeC1+ddB/Od6b+C7eMHeB/hu/vV+G3gtntPvAK/Nvw7iP997A9/FC/Y+wHfzr/PbwGvxnH4HeG3+dRD/Nd4b+C5esPcBvpsX3W8Dr8Vz+h3gtfnXQfzXeW/gu3jB3gf4bl40vw28Fs/pd4DX5l8H8V/rvYHv4gV7H+C7+Zf9NvBaPKffAV6bfx3Ef733Br6LF+x9gO/mhftt4LV4Tr8DvDb/Ooj/Hu8NfBcv2PsA380L9tvAa/Gcfgd4bf51EP993hv4Ll6w9wG+m+fvt4HX4jn9DvDa/Osg/nu9N/BdvGDvA3w3z+u3gdfiOf0O8Nr86yD++7038F28YO8DfDfP6beB1+I5/Q7w2vzrIP5neG/gu3jB3gf4bp7tt4HX4jn9DvDa/Osg/ud4b+C7eMHeB/hurvht4LV4Tr8DvDb/Ooj/Wd4b+C5esPcBvhv4beC1eE6/A7w2/zqI/3neG/guXrD3Ad4beC2e0+8Ar82/DuJ/pvcGvosXbBc4znP6HeC1+ddB/M/13sB38aL7HeC1+ddB/M/23sB38aL5HeC1+ddB/M/33sB38S/7HeC1+ddB/Of7Lf79Xho4zgv3O8Br86+D+M9n/mv8DvDa/Osg/vOZ/xq/A7w2/zqI/3zmv8bvAK/Nvw7iP5/5r/E7wGvzr4P4z/fb/Nf4a+Cj+ddB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjX0RzQb0k1WAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSouthEast;
impl IconShape for MdSouthEast {
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
                d: "M19,9h-2v6.59L5.41,4L4,5.41L15.59,17H9v2h10V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD90lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/4YHA7fyr4f43++9ge8C3gf4bv51EP+7vTfwXTzb+wDfzYsO8b/XewPfxfN6H+C7edEg/nd6b+C7eMEeAtzKvwzxv897A9/FC/Y+wHfzokH87/LewHfxgr0P8N286BD/e7w38F28YO8DfDf/Ooj/Hd4b+C5esPcBvpt/PcT/fO8NfBcv2PsA382/DeJ/tvcGvosX7H2A7+bfDvE/13sD38UL9j7Ad/Pvg/if6b2B7+IFex/gu/n3Q/zP897Ad/GCvQ/w3fzHQPzb/DbwWjyn3wFem3+f9wa+ixfsfYDv5j8O4t/mt4HX4jn9DvDa/Nu9N/BdvGDvA3w3/7EQ/za/DbwWz+l3gNfm3+a9ge/iBXsf4Lv5j4f4t/lt4LV4Tr8DvDb/eu8NfBcv2PsA381/DsS/zW8Dr8Vz+h3gtfnXeW/gu3jB3gf4bv7zIP5tfht4LZ7T7wCvzYvuvYHv4gV7H+C7+c+F+Lf5beC1eE6/A7w2L5r3Br6LF+x9gO/mPx/i3+a3gdfiOf0O8Nr8y94b+C5esPcBvpv/Goh/m98GXovn9DvAa/PCvTfwXbxg7wN8N/91EP82vw28Fs/pd4DX5gV7b+C7eMHeB/hu/msh/m1+G3gtntPvAK/N8/fewHfxgr0P8N3810P82/w28Fo8p98BXpvn9d7Ad/GCvQ/w3fz3QPzb/DbwWjyn3wFem+f03sB38YK9D/Dd/PdB/Nv8NvBaPKffAV6bZ3tv4Lt4wd4H+G7+eyH+bX4beC2e0+8Ar80V7w18Fy/Y+wDfzX8/xL/NbwOvxXP6HeC1gfcGvosX7H2A7+Z/BsS/zW8Dr8Vz+h3gu4Hv4gV7H+C7+Z8D8W/z28Br8Zx2geO8YO8DfDf/syD+bX4beC1edO8DfDf/8yD+bX4beC1eNO8DfDf/MyH+bX4beC3+Ze8DfDf/cyH+bX4beC1euF3gr/n3ex3+8yD+bX4beC3+a4j/PIh/m98GXov/GuI/D+Lf5reB1+K/hvjPg/i3+W3gtfivIf7zIP5tvhp4af5rvDb/eRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjE3HBBJhYzYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSouthWest;
impl IconShape for MdSouthWest {
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
                d: "M15,19v-2H8.41L20,5.41L18.59,4L7,15.59V9H5v10H15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEA0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif7/X4nn9DbDLvwzxv595Xq8D/Db/MsT/fuZ5vQ7w2/zLEP/7mef1OsBv8y9D/O9nntfrAL/Nvwzxv595Xq8D/Db/MsT/fuZ5vQ7w2/zLEP/7mef1OsBv8y9D/O9nntfrAL/Nvwzxv595Xq8D/Db/MsT/fuZ5vQ7w2/zLEP/7mef1OsBv8y9D/O9nntfrAL/Nvwzxv595Xq8D/Db/MsR/vZcGvgp4G2CXfz/zvF4H+G3+ZYj/Wi8N/BZwHPhr4HWAXf59zPN6HeC3+Zch/uu8NPBbwHGe7a+B1wF2+bczz+t1gN/mX4b4r/HSwG8Bx3lefw28DP925nm9DvDb/MsQ//leGvgt4DjP3/sA382/nXlerwP8Nv8yxH+ulwZ+CzjO8/c+wHfz72Oe1+sAv82/DPGf56WB3wKO8/y9D/Dd/PuZ5/U6wG/zL0P853hp4LeA4zx/7wN8N/8xzPN6HeC3+Zch/uO9NPBbwHGev/cBvpv/OOZ5vQ7w2/zLEP+xXhr4LeA4z9/7AN/NfyzzvF4H+G3+ZYj/OC8N/BZwnOfvfYDv5j+eeV6vA/w2/zLEf4yXBn4LOM7z9z7Ad/Ofwzyv1wF+m38Z4t/vpYHfAo7z/L0P8N385zHP63WA3+Zfhvj3eWngt4DjPH+7wF/zn+u1eV6vA/w2/zLEv91LA78FHOd/ntcBfpt/GeLf5qWB3wKO8z/T6wC/zb8M8W/z0sBvA8f4n+l1gN/mX4b4t3tp4LeBY/zP8zrAb/MvQ/z7vDTw28Axnr9LwF/zX++jgb/mX4b493tp4LeBYzx/7wN8N/8zIf5jvDTw28Axnr/3Ab6b/3kQ/3FeGvht4BjP3/sA383/LIj/WC8N/DZwjOfvfYDv5n8OxH+8lwZ+GzjG8/c+wHfzPwPiP8dLA78NHOP5ex/gu/nvh/jP89LAbwPHeP7eB/hu/nsh/nO9NPDbwDGev/cBvpv/Poj/fC8N/DZwjOfvfYDv5r8H4r/GSwO/DRzjef0N8NL890D813lp4LeBYzzb3wCvDezy3wPxX+ulgd8GjgF/A7w2sMt/H8R/vZcGvhp4a2CX/16I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CN7Cn5BztAy+QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubdirectoryArrowLeft;
impl IconShape for MdSubdirectoryArrowLeft {
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
                d: "M11 9l1.42 1.42L8.83 14H18V4h2v12H8.83l3.59 3.58L11 21l-6-6 6-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEAklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcceCme1+/wvxfiRffawG/xvMT/XogX3WsDv8XzEv97IV50rw38Fs9L/O+FeNG9NvBbPC/xvxfiRffawG/xvMT/XogX3WsDv8XzEv97IV50rw38Fs9L/O+FeNG9NvBbPC/xvxfiRffawG/xvMT/XogX3WsDv8XzEv97IV50rw38Fs9L/O+FeNG9NvBbPC/xvxfiRffawG/xvMT/XogX3WsDv8XzEv9+x4GfAj4G+Gv+6yBedK8N/BbPS/z7HAd+C3hpYBd4HeCv+a+BeNG9NvBbPC/xb3cc+C3gpXm2XeB1gL/mPx/iRffawG/xvMS/3V8BL83z2gVeB/hr/nMhXnSvDfwWz0v827038F08f7vA6wB/zX8exIvutYHf4nmJf5/3Br6L528XeB3gr/nPgXjRvTbwWzwv8e/33sB38fztAq8D/DX/8RAvutcGfovnJf5jvDfwXTx/u8DrAH/NfyzEi+61gd/ieYn/OO8NfBfP3y7wOsBf8x8H8aJ7beC3eF7iP9Z7A9/F87cLvA7w1/zHQLzoXhv4LZ6X+I/33sB38fztAq8D/DX/fogX3WsDv8XzEv853hv4Lp6/XeB1gL/m3wfxontt4Ld4XuI/z3sD38Xztwu8DvDX/NshXnSvDfwWz+u3+c/10sBxnr9d4HWAv+bfBvGie23gt/ifZxd4HeCv+ddDvOheG/gt/mfaBV4H+Gv+dRAvutcGfov/mS4Brw38Nf86iBfdawO/xf88l4DXBv6afz3Ei+6lga/mv95LA8d4/i4Brw38Nf82iP/Z3hv4Lp6/S8BrA3/Nvx3if673Br6L5+8S8NrAX/Pvg/if6b2B7+L5uwS8NvDX/Psh/ud5b+C7eP4uAa8N/DX/MRD/s7w38F08f5eA1wb+mv84iP853hv4Lp6/S8BrA3/NfyzE/wzvDXwXz98l4LWBv+Y/HuK/33sD38Xzdwl4beCv+c+B+O/13sB38fxdAl4b+Gv+8yD+e/018FI8r0vAawN/zX8uxH+v48BvAy/Fs10CXhv4a/7zIf77HQd+G3gp4BLw2sBf818D8T/DceCngY8G/pr/Ooj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IyL8ekH2qPxUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubdirectoryArrowRight;
impl IconShape for MdSubdirectoryArrowRight {
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
                d: "M19 15l-6 6-1.42-1.42L15.17 16H4V4h2v10h9.17l-3.59-3.58L13 9l6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3XeG/huXjTvDXw3//kQ/zW+C3gI8Nq8aH4beDrwPvznQvzn+y7gvYHfAV6bF81vA68FfDfwPvznQfzn+i7gvbnid4DX5kXz28BrccV3A+/Dfw7Ef57vAt6bZ/sd4LV50fw28Fo823cD78N/PMR/ju8C3pvn9DvAa/Oi+W3gtXhO3w28D/+xEP/xvgt4b57X7wCvzYvmt4HX4nl9N/A+/MdB/Mf6LuC9ef5+B3htXjS/DbwWz993A+/DfwzEf5zvAt6bF+x3gNfmRfPbwGvxgn038D78+yH+Y3wX8N68cL8DvDYvmt8GXosX7ruB9+HfB/Hv913Ae/OcLgGfDXwVz/Y7wGvzovlt4LX4l3038D782yH+fb4LeG+e0yXgtYHjwG/xbL8DvDYvmt8GXosXzXcD78O/DeLf7ruA9+Y5XQJeG/hr4LWB3+LZfgd4bV40vw28Fi+67wbeh389xL/NdwHvzXO6BLw28Ndc8drAb/FsvwO8Ni+a3wZei3+d7wbeh38dxL/edwHvzXO6BLw28Nc822sDv8Wz/Q7w2rxofht4Lf71vht4H150iH+d7wLem+d0CXht4K95Tq8N/BbP9jvAa/Oi+W3gtfi3+W7gfXjRIF503wW8N8/pEvDawF/zvF4b+C2e7XeA1+ZF89vAa/Fv993A+/AvQ7xovgt4b57TJeC1gb/m+Xtt4Ld4tt8BXpsXzW8Dr8W/z3cD78MLh/iXfTXwUTynS8BrA3/NC/bawG/xbL8DvDYvmt8GXot/v+8G3ocXDPEve2/gu3hOu8DrAH/NC/bawG/xbL8DvDYvmt8GXot/v/cBvpsXDPGieW/gu3hOu8DrAH/N8/fawG/xbL8DvDYvmt8GXot/n/cBvpsXDvGie2/gu3hOu8DrAH/N83pt4Ld4tt8BXpsXzW8Dr8W/3fsA382/DPGv897Ad/GcdoHXAf6a5/TawG/xbL8DvDYvmt8GXot/m/cBvpsXDeJf772B7+I57QKvA/w1z/bawG/xbL8DvDYvmt8GXot/vfcBvpsXHeLf5r2B7+I57QKvA/w1V7w28Fs82+8Ar82L5reB1+Jf532A7+ZfB/Fv997Ad/GcdoHXAf4aeG3gt3i23wFemxfNbwOvxYvufYDv5l8P8e/z3sB38Zx2gdcBjgO/xbP9DvDavGh+G3gtXjTvA3w3/zaIf7/3Br6L57QLfA7wVTzb7wCvzYvmt4HX4l/2PsB382+H+I/x3sB38cL9DvDavGh+G3gtXrj3Ab6bfx/Ef5z3Br6LF+x3gNfmRfPbwGvxgr0P8N38+yH+Y7038F08f78DvDYvmt8GXovn732A7+Y/BuI/3nsD38Xz+h3gtXnR/DbwWjyv9wG+m/84iP8c7w18F8/pd4DX5kXz28Br8ZzeB/hu/mMh/vO8N/BdPNvvAK/Ni+a3gdfi2d4H+G7+4yH+c7038F1c8TvAa/Oi+W3gtbjifYDv5j8H4j/fewPfBfwO8Nq8aH4beC3gfYDv5j8P4r/GewPvDbw2L5rfBr4b+G7+cyH+67w08Ne8aF4a+Gv+8yH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGi98RBj4tn+QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwitchLeft;
impl IconShape for MdSwitchLeft {
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
                d: "M8.5,8.62v6.76L5.12,12L8.5,8.62 M10,5l-7,7l7,7V5L10,5z M14,5v14l7-7L14,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3XeG/huXjTvDXw3//kQ/zW+C3gI8Nq8aH4beDrwPvznQvzn+y7gvYHfAV6bF81vA68FfDfwPvznQfzn+i7gvbnid4DX5kXz28BrccV3A+/Dfw7Ef57vAt6bZ/sd4LV50fw28Fo823cD78N/PMR/ju8C3pvn9DvAa/Oi+W3gtXhO3w28D/+xEP/xvgt4b57X7wCvzYvmt4HX4nl9N/A+/MdB/Mf6LuC9ef5+B3htXjS/DbwWz993A+/DfwzEf5zvAt6bF+x3gNfmRfPbwGvxgn038D78+yH+Y3wX8N68cL8DvDYvmt8GXosX7ruB9+HfB/Hv913Ae/Mv+x3gtXnR/DbwWjzbxwCfDRzjOX038D782yH+fb4LeG9eNL8DvDYvmt8GXotnex1gF/ht4BjP6buB9+HfBvFv913Ae/Oi+x3gtXnR/DbwWjzb6wC/Dbw08NvAMZ7TdwPvw78e4t/mu4D35l/nd4DX5kXz28Br8WyvA/w2V7w08NvAMZ7TdwPvw78O4l/vu4D35l/vd4DX5kXz28Br8WyvA/w2z/bSwG8Dx3hO3w28Dy86xL/OdwHvzb/N7wCvzYvmt4HX4tleB/htntNLA78NHOM5fTfwPrxoEC+67wLem3+73wFemxfNbwOvxbO9DvDbPK+XBn4bOMZz+m7gffiXIV403wW8N/8+vwO8Ni+a3wZei2d7HeC3ef5eGvht4BjP6buB9+GFQ/zLvhr4KP79fgd4bV40vw28Fs/2OsBv84K9NPDbwDGe03cD78MLhviXvTfwXfz7/Q7w2rxofht4LZ7tdYDf5gV7aeC3gOM8p/cBvpsXDPGieW/gu/j3+R3gtXnR/DbwWjzb6wC/zfP30sBvAcd5Tu8DfDcvHOJF997Ad/Fv9zvAa/Oi+W3gtXi21wF+m+f10sBvAcd5Tu8DfDf/MsS/znsD38W/ze8Ar82L5reB1+LZXgf4bZ7TSwO/BRznOb0P8N28aBD/eu8NfBf/er8DvDYvmt8GXotnex3gt3m2lwZ+CzjOc3of4Lt50SH+bd4b+C7+dX4HeG1eNL8NvBbP9jrAb3PFSwO/BRznOb0P8N386yD+7d4b+C5edL8DvDYvmt8GXotnex3gt4GXBn4LOM5zeh/gu/nXQ/z7vDfwXbxofgd4bV40vw28Fs/2OsAu8FvAcZ7T+wDfzb8N4t/vvYHv4l/2O8Br86L5beC1eLaPAT4LOM5zeh/gu/m3Q/zHeG/gu3jhfgd4bV40vw28Fi/c+wDfzb8P4j/OewPfxQv2O8Br86L5beC1eMHeB/hu/v0Q/7HeG/gunr/fAV6bF81vA6/F8/c+wHfzHwPxH++9ge/ief0O8Nq8aH4beC2e1/sA381/HMR/jvcGvovn9DvAa/Oi+W3gtXhO7wN8N/+xEP953hv4Lp7td4DX5kXz28Br8WzvA3w3//EQ/7neG/gurvgd4LV50fw28Fpc8T7Ad/OfA/Gf772B7wJ+B3htXjS/DbwW8D7Ad/OfB/Ff472B9wZemxfNbwPfDXw3/7kQ/3VeGvhrXjQvDfw1//kQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8Ie+m0QbfgksMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwitchRight;
impl IconShape for MdSwitchRight {
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
                d: "M15.5,15.38V8.62L18.88,12L15.5,15.38 M14,19l7-7l-7-7V19L14,19z M10,19V5l-7,7L10,19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9eDgVv593kwcCv/ORD/ed4b+C7gfYDv5t/mvYHvAt4H+G7+4yH+c7w38F082/sA382/znsD38WzvQ/w3fzHQvzHe2/gu3he7wN8Ny+a9wa+i+f1PsB38x8H8R/rOHArcIzn732A7+aFe2/gu3j+LgEPBnb5j4H4j/fSwG8Dx3j+3gf4bp6/9wa+i+fvEvDawF/zHwfxn+Olgd8GjvH8vQ/w3Tyn9wa+i+fvEvDawF/zHwvxn+elgd8GjvH8vQ/w3Vzx3sB38fxdAl4b+Gv+4yH+c7008NvAMZ6/9+GK7+L5uwS8NvDX/OdA/Od7aeC3gWP861wCXhv4a/7zIP5rvDTw28AxXjSXgNcG/pr/XIj/Oi8N/DZwjBfuEvDawF/znw/xX+ulgd8GjvH8XQJeG/hr/msg/uu9NPDbwDGe0yXgtYG/5r8O4r/HSwO/DRzjikvAawN/zX8txH+flwZ+myteG/hr/ush/nu9NFf8Nf89EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P893pprvhr/nsg/vu8NPBbXPE6wF/zXw/x3+Olgd8CjnPFLvA6wF/zXwvxX++lgd8CjvOcdoHXAf6a/zqI/1ovDfwWcJznbxd4HeCv+a+B+K/z0sBvAcd54XaB1wH+mv98iP8aLw38FnCcF80u8DrAX/OfC/Gf76WB3wKO86+zC7wO8Nf850H853pp4LeA4zx/78MV38Xztwu8DvDX/OdA/Od5aeC3gOM8f+8DfDdXvDfwXTx/u8DrAH/NfzzEf46XBn4LOM7z9z7Ad/Oc3hv4Lp6/XeB1gL/mPxbiP95LA78FHOf5ex/gu3n+3hv4Lp6/XeB1gL/mPw7iP9Zx4OnAcZ6/9wG+mxfuvYHv4vnbBR4C7PIfA/Ef772B7+J5vQ/w3bxo3hv4Lp7X+wDfzX8cxH+O9wa+i2d7H+C7+dd5b+C7eLb3Ab6b/1iI/zzvDXwX8D7Ad/Nv897AdwHvA3w3//EQ/7keDNzKv8+DgVv5z4H4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjqnXZBlfwGjAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUnfoldLess;
impl IconShape for MdUnfoldLess {
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
                d: "M7.41 18.59L8.83 20 12 16.83 15.17 20l1.41-1.41L12 14l-4.59 4.59zm9.18-13.18L15.17 4 12 7.17 8.83 4 7.41 5.41 12 10l4.59-4.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdwO8A381/D8R/n+8C3psr3gf4bv7rIf57fBfw3jyn9wG+m/9aiP963wW8N8/f+wDfzX8dxH+t7wLemxfufYDv5r8G4r/OdwHvzYvmfYDv5j8f4r/GdwHvzb/O+wDfzX8uxH++7wLem+fvElcc4/l7H+C7+c+D+M/1XcB78/xdAl6bK34bOMbz9z7Ad/OfA/Gf57uA9+b5uwS8NvDXXPHSwG8Dx3j+3gf4bv7jIf5zfBfw3jx/l4DXBv6a5/TSwG8Dx3j+3gf4bv5jIf7jfRfw3jx/l4DXBv6a5++lgd8GjvH8vQ/w3fzHQfzHOg78NvBSPK9LwGsDf80L99LAbwPHeF5/A7w0/3EQ//GOA78NvBTPdgl4beCvedG8NPDbwDGe7W+A1wZ2+Y+D+M9xHPht4KWAS8BrA3/Nv85LA78NHAP+BnhtYJf/WIj/PMeBnwY+Gvhr/m1eGvhq4K2BXf7jIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/vMcB34K+Bjgr/m3eWngq4C3AXb5j4f4z3Ec+C3gpYFd4HWAv+Zf56WB3wKOA38NvA6wy38sxH+848BvAS/Ns+0CrwP8NS+alwZ+CzjOs/018DrALv9xEP/x/gp4aZ7XLvA6wF/zwr008FvAcZ7XXwMvw38cxH+89wa+i+dvF3gd4K95/l4a+C3gOM/f+wDfzX8cxH+O9wa+i+dvF3gd4K95Ti8N/BZwnOfvfYDv5j8W4j/PewPfxfO3C7wO8Ndc8dLAbwHHef7eB/hu/uMh/nO9N/BdPH+7wOtwxW8Bx3n+3gf4bv5zIP7zvTfwXTx/u1xxnOfvfYDv5j8P4r/GewPfxb/O+wDfzX8uxH+d9wa+ixfN+wDfzX8+xH+t9wa+ixfufYDv5r8G4r/eewPfxfP3PsB3818H8d/jvYHv4jm9D/Dd/NdC/Pf5auCjuOJ9gO/mvx7iv9d3A78NfDf/PRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RyAackESmsN9AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUnfoldMore;
impl IconShape for MdUnfoldMore {
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
                d: "M12 5.83L15.17 9l1.41-1.41L12 3 7.41 7.59 8.83 9 12 5.83zm0 12.34L8.83 15l-1.41 1.41L12 21l4.59-4.59L15.17 15 12 18.17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6aXBo7xgv0NsMsVr8UL9zu8YIj/mX4beC1esNcBfpsrzAsnXjDE/0y/DbwWL9jrAL/NFeaFEy8Y4r/OSwNfxQv2N8BHc8VvA6/FC/Y6wG9zhXnhxAuG+K/z2sBv8YL9DvDaXPHbwGvxgr0O8NtcYV448YIh/uu8NvBbvGC/A7w2V/w28Fq8YK8D/DZXmBdOvGCI/zqvDfwWL9jvAK/NFb8NvBYv2OsAv80V5oUTLxjiv85rA7/FC/Y7wGtzxW8Dr8UL9jrAb3OFeeHEC4b4r/PawG/xgv0O8Npc8dXAS/OCfTTw11xhXjjxgiH+67w28Fu8YL8DvDb/euaFEy8Y4r/OawO/xQv2O8Br869nXjjxgiH+67w28Fu8YL8DvDb/euaFEy8Y4r/OawO/xQv2O8Br869nXjjxgiH+67w28Fu8YL8DvDb/euaFEy8Y4n8/88KJFwzxv5954cQLhviPcxx4KV6wS8Bf8x/PvHDiBUP8x3lt4Ld4wX4HeG3+45kXTrxgiP84rw38Fi/Y7wCvzX8888KJFwzxH+e1gd/iBfsd4LX5j2deOPGCIf7jvDbwW7xgvwO8Nv/xzAsnXjDEf5zXBn6LF+x3gNfmP5554cQLhviP89rAb/GC/Q7w2vzHMy+ceMEQ/3FeG/gtXrDfAV6b/3jmhRMvGOJf9lq8cL/DFa8N/BYv2O8Ar81/PPPCiRcM8S8zL5y44rWB3+IF+x3gtfmPZ1448YIh/mXmhRNXvDbwW7xgvwO8Nv/xzAsnXjDEv8y8cOKK48BL84LtAn/NfzzzwokXDPEvMy+c+O9lXjjxgiH+ZeaFE/+9zAsnXjDEv8y8cOK/l3nhxAuG+JeZF0789zIvnHjBEP8y88KJ/17mhRMvGOJfZl448d/LvHDiBUP8y8wLJ/57mRdOvGCIf5l54cR/L/PCiRcM8S8zL5z472VeOPGCIf5l5oUT/73MCydeMMS/zLxw4r+XeeHEC4b4l5kXTvz3Mi+ceMEQ/zLzwon/XuaFEy8Y4l9mXjjx38u8cOIFQ/zLzAsn/nuZF068YIh/2Wvzwv02/71emxfut3nBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAUhugEEe2nAZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWaterfallChart;
impl IconShape for MdWaterfallChart {
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
                d: "M18 4h3v16h-3zM3 13h3v7H3zm11-9h3v3h-3zm-4 1h3v4h-3zm-3 5h3v4H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADa0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7lemiv+mv+ZEP95Xhr4La54HeCv+Z8H8Z/jpYHfAo5zxS7wOsBf8z8L4j/eSwO/BRznOe0CrwP8Nf9zIP5jvTTwW8Bxnr/3Ab6b/zkQ/3FeGvgt4DjP3/sA383/LIj/GC8N/BZwnOfvfYDv5n8exL/fSwO/BRzn+Xsf4Lv5nwnx7/PSwG8Bx3n+3gf4bv7nQvzbvTTwW8Bxnr/3Ab6b/9kQ/zYvDfwWcJzn732A7+Z/PsS/3ksDvwUc5/l7H+C7+d8B8a/z0sBvAcd5/t4H+G7+90C86F4a+C3gOM/f+wDfzf8uiBfNSwO/BRzn+Xsf4Lv53wfxL3tp4LeA4zx/7wN8N/87IV64lwZ+CzjO87cL/DX/O/wN8NE8J8QL9tLAbwHH+b/hd4DX5jkhnr+XBn4LOM7/Hb8DvDbPCfH8vTTw28Ax/u/4HeC1eU6IF+ylgd8GjvF/w+8Ar81zQrxwLw38NnCM5+8S8Nf87/DXwEfznBD/spcGfhs4xvP3PsB3878T4kXz0sBvA8d4/t4H+G7+90G86F4a+G3gGM/f+wDfzf8uiH+dlwZ+GzjG8/c+wHfzvwfiX++lgd8GjvH8vQ/w3fzvgPi3eWngt4FjPH/vA3w3//Mh/u1eGvht4BjP3/sA383/bIh/n5cGfhs4xvP3PsB38z8X4t/vpYHfBo7x/L0P8N38z4T4j/HSwG8Dx3j+3gf4bv7nQfzHeWngt4FjPH/vA3w3/7Mg/mO9NPDbwDGev/cBvpv/ORD/8V4a+G3gGM/pEvDawF/zPwfiP8dLA78NHOOKS8BrA3/N/yyI/zwvDfw2V7w28Nf8z4P4z/XSXPHX/M+E+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHFilSQeBCvyoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWest;
impl IconShape for MdWest {
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
                d: "M9,19l1.41-1.41L5.83,13H22V11H5.83l4.59-4.59L9,5l-7,7L9,19z",
            }
        }
    }
}
