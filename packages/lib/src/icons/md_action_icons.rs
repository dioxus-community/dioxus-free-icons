use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md3dRotation;
impl IconShape for Md3dRotation {
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
                d: "M7.52 21.48C4.25 19.94 1.91 16.76 1.55 13H.05C.56 19.16 5.71 24 12 24l.66-.03-3.81-3.81-1.33 1.32zm.89-6.52c-.19 0-.37-.03-.52-.08-.16-.06-.29-.13-.4-.24-.11-.1-.2-.22-.26-.37-.06-.14-.09-.3-.09-.47h-1.3c0 .36.07.68.21.95.14.27.33.5.56.69.24.18.51.32.82.41.3.1.62.15.96.15.37 0 .72-.05 1.03-.15.32-.1.6-.25.83-.44s.42-.43.55-.72c.13-.29.2-.61.2-.97 0-.19-.02-.38-.07-.56-.05-.18-.12-.35-.23-.51-.1-.16-.24-.3-.4-.43-.17-.13-.37-.23-.61-.31.2-.09.37-.2.52-.33.15-.13.27-.27.37-.42.1-.15.17-.3.22-.46.05-.16.07-.32.07-.48 0-.36-.06-.68-.18-.96-.12-.28-.29-.51-.51-.69-.2-.19-.47-.33-.77-.43C9.1 8.05 8.76 8 8.39 8c-.36 0-.69.05-1 .16-.3.11-.57.26-.79.45-.21.19-.38.41-.51.67-.12.26-.18.54-.18.85h1.3c0-.17.03-.32.09-.45s.14-.25.25-.34c.11-.09.23-.17.38-.22.15-.05.3-.08.48-.08.4 0 .7.1.89.31.19.2.29.49.29.86 0 .18-.03.34-.08.49-.05.15-.14.27-.25.37-.11.1-.25.18-.41.24-.16.06-.36.09-.58.09H7.5v1.03h.77c.22 0 .42.02.6.07s.33.13.45.23c.12.11.22.24.29.4.07.16.1.35.1.57 0 .41-.12.72-.35.93-.23.23-.55.33-.95.33zm8.55-5.92c-.32-.33-.7-.59-1.14-.77-.43-.18-.92-.27-1.46-.27H12v8h2.3c.55 0 1.06-.09 1.51-.27.45-.18.84-.43 1.16-.76.32-.33.57-.73.74-1.19.17-.47.26-.99.26-1.57v-.4c0-.58-.09-1.1-.26-1.57-.18-.47-.43-.87-.75-1.2zm-.39 3.16c0 .42-.05.79-.14 1.13-.1.33-.24.62-.43.85-.19.23-.43.41-.71.53-.29.12-.62.18-.99.18h-.91V9.12h.97c.72 0 1.27.23 1.64.69.38.46.57 1.12.57 1.99v.4zM12 0l-.66.03 3.81 3.81 1.33-1.33c3.27 1.55 5.61 4.72 5.96 8.48h1.5C23.44 4.84 18.29 0 12 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PY4D7wW8NFf8NfA9wC7/tRD/9d4b+CrgOM9pF/gY4Lv5r4P4r/XawG/xwr0O8Nv810D813o68GBeuFuBh/BfA/Ff56WBv+JF8zLAX/OfD/Ff57WB3+JF8zrAb/OfD/Ff57WB3+JF8zrAb/OfD/Ffaxc4xgt3CTjOfw3Ef63PBj6LF+5zgM/mvwbiv953A+/F8/c9wHvzXwfx3+O9gY8GXoorfgf4buC7+a+F+P8N8f8b4l/2Wvzv9ju8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+feV5/A+zynI4DL8XzEv95EP/5zPN6HeC3eU6vDfwWz0v850H85zPP63WA3+Y5vTbwWzwv8Z8H8Z/PPK/XAX6b5/TawG/xvMR/HsR/PvO8Xgf4bZ7TawO/xfMS/3kQ//nM83od4Ld5Tq8N/BbPS/znQfznM8/rdYDf5jm9NvBbPC/xnwfxn888r9cBfpvn9NrAb/G8xH8exH8+87xeB/htntNrA7/F8xL/eRD/+czzeh3gt3lOrw38Fs9L/OdB/Oczz+t1gN/mOb028Fs8L/GfB/Gfzzyv1wF+m+f02sBv8bzEfx7Efz7zvF4H+G2e02sDv8XzEv95EP/5zPN6HeC3eU6vDfwWz0v850H85zPP63WA3+Y5vTbwWzwv8Z8H8Z/PPK/XAX6b5/TawG/xvMR/HsR/vt/meX008Nc8p5cGvprn9dr850H8/4b4/w3x/xvi/zfE/2/8I5eCZEFVASV4AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessibility;
impl IconShape for MdAccessibility {
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
                d: "M12 2c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9 7h-6v13h-2v-6h-2v6H9V9H3V7h18v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PY4D7wW8NFf8NfA9wC7/tRD/9d4b+CrgOM9pF/gY4Lv5r4P4r/XawG/xwr0O8Nv810D813o68GBeuFuBh/BfA/Ff56WBv+JF8zLAX/OfD/Ff57WB3+JF8zrAb/OfD/Ff57WB3+JF8zrAb/OfD/Ffaxc4xgt3CTjOfw3Ef63PBj6LF+5zgM/mvwbiv953A+/F8/c9wHvzXwfx3+O9gY8GXoorfgf4buC7+a+FeNH8FbAL/Dbw18DvALv8z3IceC3gpYHXBo4DL8MLh/iXHQcu8rxuBW4FfhvYBf4aeAZwK/+5Hgw8CHhp4Djw2sCDgQfzvE4Au7xgiH/ZWwM/xb/OLvDXXLEL/DXP61bgVq54MPBgntdLA8e54qWB4/zrvA3w07xgiH/ZVwMfxf9OXwN8NC8Y4l/2V8BL87/TXwMvwwuG+Jd9NPDawGsDx/jf4RLw28BvA1/NC4b413lp4KWBBwOvDbw0cIz/XpeAvwZ+G7gV+Gvgr3nRIP79jgMvDTwYeDDwYODBwHHgpfiP8TfALnArcCtwK3Ar8NfALv92iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n3lefwPs8pyOAy/F8xL/eRD/+czzeh3gt3lOrw38Fs9L/OdB/Oczz+t1gN/mOb028Fs8L/GfB/Gfzzyv1wF+m+f02sBv8bzEfx7Efz7zvF4H+G2e02sDv8XzEv95EP/5zPN6HeC3eU6vDfwWz0v850H85zPP63WA3+Y5vTbwWzwv8Z8H8Z/PPK/XAX6b5/TawG/xvMR/HsR/PvO8Xgf4bZ7TawO/xfMS/3kQ//nM83od4Ld5Tq8N/BbPS/znQfznM8/rdYDf5jm9NvBbPC/xnwfxn888r9cBfpvn9NrAb/G8xH8exH8+87xeB/htntNrA7/F8xL/eRD/+czzeh3gt3lOrw38Fs9L/OdB/Oczz+t1gN/mOb028Fs8L/GfB/Gfzzyv1wF+m+f02sBv8bzEfx7Ef77f5nl9NPDXPKeXBr6a5/Xa/OdB/P+G+P8N8f8b4v83xP9v/CPp/YtB02xYegAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessibilityNew;
impl IconShape for MdAccessibilityNew {
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
                d: "M20.5 6c-2.61.7-5.67 1-8.5 1s-5.89-.3-8.5-1L3 8c1.86.5 4 .83 6 1v13h2v-6h2v6h2V9c2-.17 4.14-.5 6-1l-.5-2zM12 6c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessible;
impl IconShape for MdAccessible {
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
            circle {
                cx: "12",
                cy: "4",
                r: "2",
            }
            path {
                d: "M19 13v-2c-1.54.02-3.09-.75-4.07-1.83l-1.29-1.43c-.17-.19-.38-.34-.61-.45-.01 0-.01-.01-.02-.01H13c-.35-.2-.75-.3-1.19-.26C10.76 7.11 10 8.04 10 9.09V15c0 1.1.9 2 2 2h5v5h2v-5.5c0-1.1-.9-2-2-2h-3v-3.45c1.29 1.07 3.25 1.94 5 1.95zm-6.17 5c-.41 1.16-1.52 2-2.83 2-1.66 0-3-1.34-3-3 0-1.31.84-2.41 2-2.83V12.1c-2.28.46-4 2.48-4 4.9 0 2.76 2.24 5 5 5 2.42 0 4.44-1.72 4.9-4h-2.07z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessibleForward;
impl IconShape for MdAccessibleForward {
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
            circle {
                cx: "17",
                cy: "4.54",
                r: "2",
            }
            path {
                d: "M14 17h-2c0 1.65-1.35 3-3 3s-3-1.35-3-3 1.35-3 3-3v-2c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5zm3-3.5h-1.86l1.67-3.67C17.42 8.5 16.44 7 14.96 7h-5.2c-.81 0-1.54.47-1.87 1.2L7.22 10l1.92.53L9.79 9H12l-1.83 4.1c-.6 1.33.39 2.9 1.85 2.9H17v5h2v-5.5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+ezwY+Czgvbniu4HPAW7lvxbiv9aDgc8C3pvn77uBzwFu5b8G4r/Gg4HPAt6bF813A58D3Mp/LsR/rgcDnwW8N/823w18DnAr/zkQ/zkeDHwW8N78x/hu4HOAW/mPhfiP9WDgs4D35j/HdwOfA9zKfwzEf4wHA58FvDf/Nb4b+BzgVv59EP8+DwY+C3hv/nt8N/A5wK382yD+bR4MfBbw3vzP8N3A5wC38q+D+Nd5MPBZwHvzP9N3A58D3MqLBvGieTDwWcB787/DdwOfA9zKC4d44R4MfBbw3vzv9N3A5wC38vwhnr8HA58FvDf/N3w38DnArTwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/Lw0c5/+WXeCveU6I/98Q/78h/n9D/P+GeNEdB16KF+wS8Nf8y16LF+53+Je9NHCMF+xvgF3+ZYgX3WsDv8UL9jvAa/MvMy+c+Jf9NvBavGCvA/w2/zLEi+61gd/iBfsd4LX5l5kXTvzLfht4LV6w1wF+m38Z4kX32sBv8YL9DvDa/MvMCyf+Zb8NvBYv2OsAv82/DPGie23gt3jBfgd4bf5l5oUT/7LfBl6LF+x1gN/mX4Z40b028Fu8YL8DvDb/MvPCiX/ZbwOvxQv2OsBv8y9DvOheG/gtXrDfAV6bf5l54cS/7LeB1+IFex3gt/mXIV50rw38Fi/Y7wCvzb/MvHDiX/bbwGvxgr0O8Nv8yxAvutcGfosX7HeA1+ZfZl448S/7beC1eMFeB/ht/mWIF91rA7/FC/Y7wGvzLzMvnPiX/TbwWrxgrwP8Nv8yxIvutYHf4gX7HeC1+ZeZF078y34beC1esNcBfpt/GeJF99rAb/GC/Q7w2vzLzAsn/mW/DbwWL9jrAL/Nvwzxontt4Ld4wX4HeG3+ZeaFE/+y3wZeixfsdYDf5l+GeNG9NvBbvGC/A7w2/zLzwol/2W8Dr8UL9jrAb/MvQ7zoXhv4LV6w3wFem3+ZeeHEv+y3gdfiBXsd4Lf5lyFedK8N/BYv2O8Ar82/zLxw4l/228Br8YK9DvDb/MsQL7rXBn6LF+x3gNfmX2ZeOPEv+23gtXjBXgf4bf5liBfdawO/xQv2O8Br8y8zL5z4l/028Fq8YK8D/Db/MsSL7rWB3+IF+x3gtfmXmRdO/Mt+G3gtXrDXAX6bfxniRXcceGlesF3gr/mXvTYv3G/zL3tp4Dgv2F8Du/zLEP+/If5/Q/z/hvj/DfH8vTRwjP9bLgF/zXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fVwEvzf8tfAx/Nc0L8/4b4/w3x/xvi/zfE/2/8I/JtqkHyac9GAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccountBalance;
impl IconShape for MdAccountBalance {
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
            rect {
                height: "7",
                width: "3",
                x: "4",
                y: "10",
            }
            rect {
                height: "7",
                width: "3",
                x: "10.5",
                y: "10",
            }
            rect {
                height: "3",
                width: "20",
                x: "2",
                y: "19",
            }
            rect {
                height: "7",
                width: "3",
                x: "17",
                y: "10",
            }
            polygon {
                points: "12,1 2,6 2,8 22,8 22,6",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5j/e9wBfDfw1/70Q/zLzH+t9gO/mfwbEv8z8x/kc4LP5nwPxLzP/cU4Au/zPgfiXmf8YvwO8Nv+zIP5l5j/G7wCvzfP6Lf5r/A3w0TwnxL/M/Mf4HeC1eV7mv8bvAK/Nc0L8y8x/jN8BXpvnZf5r/A7w2jwnxL/M/Mf4HeC1eV7mv8bvAK/Nc0L8y8x/jN8BXpvnZf5r/A7w2jwnxL/M/Mf4HeC1eV7mv8bvAK/Nc0L8y8x/jN8BXpvnZf5r/A7w2jwnxL/M/Mf4HeC1eV7mhfsd4LOB3wZeGnhr4LP41/sd4LV5Toh/mfmP8TvAa/O8zAv2PcB787xeG/gt/nV+B3htnhPiX2b+Y/wO8No8L/OCnQB2ef6+G3gvXnS/A7w2zwnxLzP/MX4HeG2el3n+fgd4bV6wjwa+ihfd7wCvzXNC/MvMf4zfAV6b52Wev98BXpsX7KOBr+JF9zvAa/OcEP8y8x/jd4DX5nmZF+wEsMvz993Ae/Gi+x3gtXlOiH+Z+Y/xO8Br87zMC/bdwPvwvF4b+C3+dX4HeG2eE+JfZv5j/A7w2jwv88L9NvDZwO8ALw28FfDZ/Ov9DvDaPCfEv8z8x/gd4LV5Xua/xu8Ar81zQvzLzH+M3wFem+dl/mv8DvDaPCfEv8z8x/gd4LV5Xua/xu8Ar81zQvzLzH+M3wFem+dl/mv8DvDaPCfEv8z8x/gd4LV5Xua/xu8Ar81zQvzLzH+M3wFem+dl/mv8DvDaPCfEv8z8x/gd4LV5Xr/Nf42/Bj6a54T4l5n/GL8DvDb/syD+ZeY/zglgl/85EP8y8x/nc4DP5n8OxL/M/Md6H+C7+Z8B8S8z//G+G/ga4K/574X4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MteGvgr/nd6CHArLxjiRfPewHfxv8v7AN/NC4d40T0Y+GzgrYFj/M90Cfhp4LOBW/mXIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPf76eQT8iyV0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccountBalanceWallet;
impl IconShape for MdAccountBalanceWallet {
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
                d: "M21 18v1c0 1.1-.9 2-2 2H5c-1.11 0-2-.9-2-2V5c0-1.1.89-2 2-2h14c1.1 0 2 .9 2 2v1h-9c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h9zm-9-2h10V8H12v8zm4-2.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5j/WM4CvBn4b+GuueGngtYGPBh7EfyzxgiH+ZeY/zscAX80L99HAV/EfR7xgiH+Z+Y/xMsBf86J5aeCv+I8hXjDEv8z8+30M8NX863w08FX8+4kXDPEvM/8+zwAezL/NrcCD+PcRLxjiX2b+fT4G+Gr+bT4a+Cr+fcQLhviXmX+flwH+mn+blwb+in8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+Zebf52WAv+bf5qWBv+LfR7xgiH+Z+ff5GOCr+bf5aOCr+PcRLxjiX2b+fW4FHsK/zdOBB/PvI14wxL/M/Pt9DPDV/Ot8NPBV/PuJFwzxLzP/MV4G+GteNC8N/BX/McQLhviXmf84Hw18DS/cRwFfzX8c8YIh/mXmP9atwFcDvwP8NVe8NPBawEcDD+Y/lnjBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvMf42+A3wZuBf6aK36bK16bK14aeDDw2sBL8R9DvGCIf5n5t/se4KeB3wZ2+dc5Drw28NbAe/FvJ14wxL/M/OtcAr4a+Gpgl/8Yx4GPBj4aOMa/jnjBEP8y86L7HOCrgV3+cxwHPhr4LF504gVD/MvMv+wS8NbAb/Nf47WBnwaO8S8TLxjiX2b+ZS8D/DX/tV4a+Cv+ZeIFQ/zLzAv3OcBn89/js4HP4oUTLxjiX2ZeuJcB/pr/Hi8N/BUvnHjBEP8y88KJ/17mhRMvGOJfZl448d/LvHDiBUP8y8wLJ/57mRdOvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/spYG/4n+nhwC38oIhXjTvDXwX/7u8D/DdvHCIF92Dgc8G3ho4xv9Ml4CfBj4buJV/GeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CFX+fEED6KHSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccountBox;
impl IconShape for MdAccountBox {
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
                d: "M3 5v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2H5c-1.11 0-2 .9-2 2zm12 4c0 1.66-1.34 3-3 3s-3-1.34-3-3 1.34-3 3-3 3 1.34 3 3zm-9 8c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1H6v-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccountCircle;
impl IconShape for MdAccountCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 3c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm0 14.2c-2.5 0-4.71-1.28-6-3.22.03-1.99 4-3.08 6-3.08 1.99 0 5.97 1.09 6 3.08-1.29 1.94-3.5 3.22-6 3.22z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx3hOl4C/5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/mshnu2lga/ihfsZ4Kv59/tt4LV4Tr8DvDb/tRDP9trAb/HC3Qo8hH+/3wZei+f0O8Br818L8WyvDfwW/7L3Ab6bf5/fBl6L5/Q7wGvzXwvxbK8N/Bb/sluBh/Dv89vAa/Gcfgd4bf5rIZ7ttYHf4kXzPsB382/328Br8Zx+B3ht/mshnu2lga/meb0Wz+uvgZfhBXtp4Bgv2FcDL81z+mvgo3nBLgF/zQt3nCt2edEg/mXfDbwXz+t1gN/m+ftt4LX4j/U7wGvzwv0W8GDgs4Hv4V+G+Jc9GHg6z+u3gdfh+ftt4LX4j/U7wGvzgr028Fs8263A+wC/zQuGeNF8N/BePK/XAX6b5/XbwGvxH+t3gNfmBfst4LV5Xi8D/DXPH+JF82Dg6Tyv3wZeh+f128Br8R/rd4DX5vl7b+C7eF6/A7w2LxjiRffbwGvxvF4G+Gue028Dr8V/rN8BXpvn7+nAg3leLwP8NS8Y4kX32sBv8by+B3hv/nV+G3gtntPvAK/Nv95nA5/F8/oe4L154RD/Or8NvBbP6yHArbzofht4LZ7T7wCvzb/OceDpwHGe0yXgpYFbeeEQ/zqvDfwWz+t7gPfmRffbwGvxnH4HeG3+db4a+Cie1+cAn82/DPGv99fAS/G8HgLcyovmt4HX4jn9DvDavOgeDDyd53UJeDCwy78M8a/33sB38by+BvhoXjS/DbwWz+l3gNfmRffdwHvxvD4G+GpeNIh/m1uBB/GcdoGHALv8y34beC2e0+8Ar82L5rWB3+J5PQN4MC86xL/NewPfxfP6HOCz+Zf9NvBaPKffAV6bF81vAa/N83ob4Kd50SH+7W4FHsRz2gUeAuzywv028Fo8p98BXpt/2VsDP8Xz+h3gtfnXQfzbfTTwVTyvzwE+m/88TwcezPN6HeC3+ddB/NsdB24FjvGcbgUewn+Ojwa+iuf1PcB786+H+Pf5bOCzeF7vA3w3/7GOA08HjvO8HgLcyr8e4t/nOHArcIzndCvwEP5jfTbwWTyvrwE+mn8bxL/fZwOfxfN6H+C7+Y/xYOCvgOM8p0vAg4Fd/m0Q/34PBp7O89oF/pr/GA8GHszz+hzgs/m3Q/zH+G7gvfiv9Qzgwfz7IP5j/Bbw2vzXeh/gu/n3Qfz7vTbwW/zX+hngrfn3Q/z7PR14MP91fgZ4b2CXfz/Ev897A9/F83oGcCv/cXaBvwZ+Gvhr/uMg/u2OA38FPJjn9RDgVv7nQ/zbfTbwWTyv7wHem/8dEP82x4GnA8d5TpeABwO7/O+A+Lf5aOCreF6fA3w2/3sg/m2+GvgontfbAD/N/x6If5vvBt6L/1l+B3ht/nUQ/zYfDXwV/7P8DvDa/Osg/m2OA7cCx/if43eA1+ZfB/Fv997Ad/E/x+8Ar82/DuLf57WB7wYexH+/3wFem38dxH+MlwaO899rF/hr/nUQ/78h/n9D/PscBz4KeG/gwcAu8NPA5wC38h/rwcBnAW8NHAduBb4b+Bpgl38bxL/dg4GfAl6a5+99gO/mP8Z7A9/F8/fXwNsAt/Kvh/i3+yvgpXnhHgLcyr/Pg4Gn88L9NfAy/Osh/m3eGvgp/mXfA7w3/z7fDbwX/7K3AX6afx3Ev81nA5/Fv2wXOMG/z0XgOP+yzwE+m38dxL/NVwMfxYtG/PuYF83PAG/Nvw7i3+azgc/iX/YM4MH8+/w18FL8yz4H+Gz+dRD/Ng8Gns6/7HOAz+bf57OBz+Jf9hDgVv51EP92Xw18FC/Y3wCvDezy73Mc+G3gpXjBvgb4aP71EP8+Xw18FM/rb4C3Bm7lP8aDgZ8GXorn9TXAR/Nvg/j3ezDw3sBLA38N/DXw0/zneGvgpYGXBv4a+G7gVv7tEP+/If5/Q/z/hvj/DfH/G/8IHBXnQX6enD8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddShoppingCart;
impl IconShape for MdAddShoppingCart {
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
                d: "M11 9h2V6h3V4h-3V1h-2v3H8v2h3v3zm-4 9c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zm10 0c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2zm-9.83-3.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.86-7.01L19.42 4h-.01l-1.1 2-2.76 5H8.53l-.13-.27L6.16 6l-.95-2-.94-2H1v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.13 0-.25-.11-.25-.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddTask;
impl IconShape for MdAddTask {
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
                d: "M22,5.18L10.59,16.6l-4.24-4.24l1.41-1.41l2.83,2.83l10-10L22,5.18z M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8 c1.57,0,3.04,0.46,4.28,1.25l1.45-1.45C16.1,2.67,14.13,2,12,2C6.48,2,2,6.48,2,12s4.48,10,10,10c1.73,0,3.36-0.44,4.78-1.22 l-1.5-1.5C14.28,19.74,13.17,20,12,20z M19,15h-3v2h3v3h2v-3h3v-2h-3v-3h-2V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddToDrive;
impl IconShape for MdAddToDrive {
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
                d: "M7.71 3.52L1.15 15l3.42 5.99 6.56-11.47-3.42-6zM13.35 15H9.73L6.3 21h8.24c-.96-1.06-1.54-2.46-1.54-4 0-.7.13-1.37.35-2zM20 16v-3h-2v3h-3v2h3v3h2v-3h3v-2h-3zm.71-4.75L15.42 2H8.58v.01l6.15 10.77C15.82 11.68 17.33 11 19 11c.59 0 1.17.09 1.71.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddchart;
impl IconShape for MdAddchart {
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
                d: "M22 5v2h-3v3h-2V7h-3V5h3V2h2v3h3zm-3 14H5V5h6V3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-6h-2v6zm-4-6v4h2v-4h-2zm-4 4h2V9h-2v8zm-2 0v-6H7v6h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/NewHtzxXcD38N/P8R/ruPARwHvDTyY53Qr8N3A1wC7/PdA/Od4MPBZwHvzL9sFfhr4HOBW/msh/mO9FfDRwGvzb/PbwFcDP8N/DcS/33HgrYDPBh7Mf4xbgc8GfgbY5T8P4t/uwcBHAe8NHOc/xy7w3cDXALfyHw/xr/fawEcBb81/re8Gvgf4bf7jIF507wV8NPDS/Pf6a+Crge/h3w/xL/ss4KOB4/zPsgt8NfA5/Nsh/mXmfzbxb4f4l5n/2cS/HeJfZv5nE/92iH+Z+Z9N/Nsh/mXmfzbxb4f4l5n/2cS/HeJfZv5nE/92iH+Z+Z9N/Nsh/mXmfzbxb4f4l5n/2cS/HeJfZv5nE/92iH+Z+Z9N/Nsh/mXmv98zgN8GbuWK3wZeGjgOfDb/doh/mfnv8Qzgq4GfBm7lPwfiX/bXwEvxX+cS8NXAZ/OfD/Ev+23gtfiv8TfAawO7/NdA/Mt+G3gt/vN9D/De/NdC/Mu+Gvgo/nN9D/De/MteGjjGsz0DuJV/O8S/7K2Bn+I/z98AL80L9mDgs4DXBh7M8/pr4LuB7wF2+ddB/MuOAxf5z3EJeDCwy/P3VcBH86LZBd4H+GledIgXzV8DL8V/vM8BPpvndRz4LeCl+df7bOBzeNEgXjSfDXwW/7GeATyY5++vgJfmeV0CvhrYBV4aeC+ev48Bvpp/GeJF82Dg6fzH+hjgq3leXw18FM/rEvBgYJdne2/gu3j+Xgf4bV44xIvuu4H34j/OQ4BbeU4PBp7O8/c5wGfzvG4FHsTz+m3gdXjhEC+61wZ+i/8YzwAezPP6buC9eP4+BvhqntdvA6/F8/c6wG/zgiH+dX4beC3+/b4HeG+e19OBB/P8fQ/w3jyn48DTgeM8f18DfDQvGOJf56WBv+Lf73OAz+Y5vTTwV7xwHw18DVccB74KeG9esN8BXpsXDPGv99nAZ/Hv8znAZ/OcXhv4Lf5ltwK3Ai8NHOdfJl4wxL/eceCvgQfxb/c6wG/znF4b+C3+44kXDPFv89LAX/Fv9znAZ/OcXhv4LV64ZwC38mwvDRzjBXsG8GBeMMS/3XsD38W/zecAn81zejDwdJ7XJeCrge8GbuV5vTTw0cB78bx+B3htXjDEv897A9/Fv97PAG/N8/pr4KV4tr8BXhvY5V/22sBv8Zw+BvhqXjDEv993A+/Fv84ucILn9dHAV/FsXwN8NC+6vwJemmd7CHArLxjiP8Z7A9/Fv87LAH/NczoO3Aoc44pd4HWAv+Zf9l3Ae/Ns3wO8Ny8c4j/OSwO/DRzjRfM9wHvzvN4a+Cme02cDn8Pz92Dgu4DX5tkuAQ8GdnnhEP+xjgM/DbwWL5oTwC7P66uBj+I57QLfDewCfw28NPDSwFvznC4Brw38Nf8yxH+O1wa+GngpXrjvAd6b5++jga/iX+cS8NrAX/OiQfznem/go4GX4vl7BvDSwC7P32sDnw28Fv+y7wE+GtjlRYf4r/Fg4K2B9waOAz8NfDfw17xo3hp4beClgdfiimcAtwI/Dfw0cCv/eoj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I+/uwkF3y0YRAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAdminPanelSettings;
impl IconShape for MdAdminPanelSettings {
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
                d: "M17,11c0.34,0,0.67,0.04,1,0.09V6.27L10.5,3L3,6.27v4.91c0,4.54,3.2,8.79,7.5,9.82c0.55-0.13,1.08-0.32,1.6-0.55 C11.41,19.47,11,18.28,11,17C11,13.69,13.69,11,17,11z",
            }
            path {
                d: "M17,13c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C21,14.79,19.21,13,17,13z M17,14.38c0.62,0,1.12,0.51,1.12,1.12 s-0.51,1.12-1.12,1.12s-1.12-0.51-1.12-1.12S16.38,14.38,17,14.38z M17,19.75c-0.93,0-1.74-0.46-2.24-1.17 c0.05-0.72,1.51-1.08,2.24-1.08s2.19,0.36,2.24,1.08C18.74,19.29,17.93,19.75,17,19.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlarm;
impl IconShape for MdAlarm {
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
                d: "M22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM12.5 8H11v6l4.75 2.85.75-1.23-4-2.37V8zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlarmAdd;
impl IconShape for MdAlarmAdd {
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
                d: "M7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm1-11h-2v3H8v2h3v3h2v-3h3v-2h-3V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlarmOff;
impl IconShape for MdAlarmOff {
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
                d: "M12 6c3.87 0 7 3.13 7 7 0 .84-.16 1.65-.43 2.4l1.52 1.52c.58-1.19.91-2.51.91-3.92 0-4.97-4.03-9-9-9-1.41 0-2.73.33-3.92.91L9.6 6.43C10.35 6.16 11.16 6 12 6zm10-.28l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM2.92 2.29L1.65 3.57 2.98 4.9l-1.11.93 1.42 1.42 1.11-.94.8.8C3.83 8.69 3 10.75 3 13c0 4.97 4.02 9 9 9 2.25 0 4.31-.83 5.89-2.2l2.2 2.2 1.27-1.27L3.89 3.27l-.97-.98zm13.55 16.1C15.26 19.39 13.7 20 12 20c-3.87 0-7-3.13-7-7 0-1.7.61-3.26 1.61-4.47l9.86 9.86zM8.02 3.28L6.6 1.86l-.86.71 1.42 1.42.86-.71z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlarmOn;
impl IconShape for MdAlarmOn {
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
                d: "M22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm-1.46-5.47L8.41 12.4l-1.06 1.06 3.18 3.18 6-6-1.06-1.06-4.93 4.95z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6a/1leGvgqnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzP/PuLfx/z7iBcM8S8z/z4vA/w1/zYvDfwV/z7iBUP8y8y/z8cAX82/zUcDX8W/j3jBEP8y8+9zK/AQ/m2eDjyYfx/xgiH+Zebf72OAr+Zf56OBr+LfT7xgiH+Z+Y/xPsB386J5b+C7+I8hXjDEv8z8x/lo4Gt44T4K+Gr+44gXDPEvM/+xbgW+Gvgd4K+54qWB1wI+Gngw/7HEC4b4l5n/3cQLhviX/TXwUvzv9DvAa/OCIf5l7w18F/87vQ/w3bxgiBfNdwPvxf8u3wO8Ny8c4kX32cBHA8f4n+0S8NXAZ/MvQ/zrvTb/s/02LzrE/2+I/98Q/78h/n9D/Mtei3+f3+Hf57X49/kdXjDEv8z8+4h/u+PARf59xAuG+JeZf5/XAX6bf5vXBn6Lfx/xgiH+Zebf52uAj+bf5quBj+LfR7xgiH+Z+ffZBR4C7PKvcxx4OnCcfx/xgiH+Zebf77uB9+Ff57uA9+bfT7xgiH+Z+Y/x3cD78KL5LuC9+Y8hXjDEv8z8x/lt4H2AW3n+Xhr4KuC1+Y8jXjDEv8z8x/tr4KeBW7niwcBbAy/NfzzxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IReB5QXN3TyEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAllInbox;
impl IconShape for MdAllInbox {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 6h-4c0 1.62-1.38 3-3 3s-3-1.38-3-3H5V5h14v4zm-4 7h6v3c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2v-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAllOut;
impl IconShape for MdAllOut {
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
                d: "M.21.16h24v24h-24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz3+h2e10sDx3jRiBcM8S8z/73E8/pt4LV40YgXDPEvM/+9xPP6beC1eNGIFwzxLzP/vcTz+m3gtXjRiBcM8S8z/73E8/pt4LV40YgXDPEvM/+9xPP6beC1eNGIFwzxLzP/vcTz+m3gtXjRiBcM8S8z/73E8/pt4LV40YgXDPEvM//x/gbY5TkdB16K5yWe128Dr8WLRrxgiH+Z+Y/3OsBv85xeG/gtnpd4Xr8NvBYvGvGCIf5l5j/e6wC/zXN6beC3eF7ief028Fq8aMQLhviXmf94rwP8Ns/ptYHf4nmJ5/XbwGvxohEvGOJfZv7jvQ7w2zyn1wZ+i+clntdvA6/Fi0a8YIh/mfmP9zrAb/OcXhv4LZ6XeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zL+PeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zL+PeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zL+PeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zL+PeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zL+PeF6/DbwWLxrxgiH+ZeZF8zvAa/O8zPP6GOCveU4vDXwVz0s8r98GXosXjXjBEP8y86L5HeC1eV7meb0O8Ns8p9cGfovnJZ7XbwOvxYtGvGCIf5l50fwO8No8L/O8Xgf4bZ7TawO/xfMSz+u3gdfiRSNeMMS/zLxofgd4bZ6XeV6vA/w2z+m1gd/ieYnn9dvAa/GiES8Y4l9mXjS/A7w2z8s8r9cBfpvn9NrAb/G8xPP6beC1eNGIFwzxLzMvmt8BXpvnZZ7X6wC/zXN6beC3eF7ief028Fq8aMQLhviXmRfN7wCvzfMyz+t1gN/mOb028Fs8L/G8fht4LV404gVD/MvMi+Z3gNfmeZnn9TrAb/OcXhv4LZ6XeF6/DbwWLxrxgiH+ZeZFswv8Nc/rtXlefw3s8pyOAy/N8/ptntdLA8d50YgXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHpu+UQdraSBcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAnalytics;
impl IconShape for MdAnalytics {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-5h2v5zm4 0h-2v-3h2v3zm0-5h-2v-2h2v2zm4 5h-2V7h2v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAnchor;
impl IconShape for MdAnchor {
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
                d: "M17,15l1.55,1.55c-0.96,1.69-3.33,3.04-5.55,3.37V11h3V9h-3V7.82C14.16,7.4,15,6.3,15,5c0-1.65-1.35-3-3-3S9,3.35,9,5 c0,1.3,0.84,2.4,2,2.82V9H8v2h3v8.92c-2.22-0.33-4.59-1.68-5.55-3.37L7,15l-4-3v3c0,3.88,4.92,7,9,7s9-3.12,9-7v-3L17,15z M12,4 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,4,12,4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/vVeGvgo4GeAn+Z/hrcG3gr4GuCvedEh/vUuAse54nWA3+a/12sDv8UVtwIP4UWH+NfbBY5xxa3AQ/jv9XTgwVxxCTjOiw7xr/fRwFfxbJ8DfDb/PT4b+Cye7WOAr+ZFh/i3uRV4EFfsAg8BdvmvdRx4OnCcK54BPJh/HcS/zWsDv8WzfQ/w3vzX+m7gvXi21wF+m38dxL/dbwOvxbO9DPDX/Nd4aeCveLafAd6afz3Ev92DgafzbL8NvA4v2IOBBwGvzbO9NFf8Nc/228AzgFt5wX4LeG2e7SHArfzrIf59Phv4LJ7tfYDv5ooHA28FvDbw2sBx/nV2gd8Gfhv4GeBWrnhv4Lt4ts8BPpt/G8S/z3HgVuAYV9wKfDbw0cBL8x/rt4HvBj4beDBXXAIeDOzyb4P493tv4Lv47/E+wHfzb4f493tr4Kf47/E2wE/zb4f4t3sw8FXAW/Pf66eB9wF2+ddD/Nu8NfBdwHH+Z9gF3gb4bf51EP967w18F/8zvQ/w3bzoEP863wW8N/+zfTfwPrxoEC+6rwY+iv8dPgf4bP5liBfNewPfxf8u7wN8Ny8c4l/22sBv8b/T6wC/zQuGeOGOA08HjvO/0y7wEGCX5w/xwv0U8Na8cH8DfDbw08BLAx8NvBf/Ob4G+G7gr4G3Bj4beCleuJ8G3obnD/GCvTXwU7xwzwBeGtjlOX038F78x/oa4KN5TseBW4FjvHBvA/w0zwvxgj0deDAv3OcAn83zemngr/iP9TLAX/O8Phv4LF64W4GH8LwQz99HA1/Fv+x9gO/m+TP/scTz99HAV/Evex/gu3lOiOfv6cCD+Zd9DfDRPK+3Bn6K/1hvA/w0z+urgY/iX3Yr8BCeE+J5vTXwU7zoXgb4a57tOPBbwEvzH+uvgdcBdnm2lwb+ihfd2wA/zbMhntdPA2/Fv85XA7cCx4GPBo7zn+NW4LuBXeDBwEfzr/MzwFvzbIjndBy4yP9tJ4BdrkA8p7cGfor/294G+GmuQDynrwY+iv/bvgb4aK5APKe/Al6a/9v+GngZrkA8J/P/g7gC8WwPBp7O/w8PAW4FEM/22sBv8f/D6wC/DSD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8xPodBrDaTYwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAndroid;
impl IconShape for MdAndroid {
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
                d: "M17.6,9.48l1.84-3.18c0.16-0.31,0.04-0.69-0.26-0.85c-0.29-0.15-0.65-0.06-0.83,0.22l-1.88,3.24 c-2.86-1.21-6.08-1.21-8.94,0L5.65,5.67c-0.19-0.29-0.58-0.38-0.87-0.2C4.5,5.65,4.41,6.01,4.56,6.3L6.4,9.48 C3.3,11.25,1.28,14.44,1,18h22C22.72,14.44,20.7,11.25,17.6,9.48z M7,15.25c-0.69,0-1.25-0.56-1.25-1.25 c0-0.69,0.56-1.25,1.25-1.25S8.25,13.31,8.25,14C8.25,14.69,7.69,15.25,7,15.25z M17,15.25c-0.69,0-1.25-0.56-1.25-1.25 c0-0.69,0.56-1.25,1.25-1.25s1.25,0.56,1.25,1.25C18.25,14.69,17.69,15.25,17,15.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfiP8/HAH/Nc3pp4Kv4z/M7wGvznBDP328Dr8V/ntcBfpvn9NrAb/Gf53eA1+Y5IZ6/3wZei/88rwP8Ns/ptYHf4j/P7wCvzXNCPH+/DbwW/3leB/htntNrA7/Ff57fAV6b54R4/n4beC3+87wO8Ns8p9cGfov/PL8DvDbPCfH8/TbwWvzneR3gt3lOrw38Fv95fgd4bZ4T4vn7beC1+M/zOsBv85xeG/gt/vP8DvDaPCfE8/fbwGvxn+d1gN/mOb028Fv85/kd4LV5Tojn77eB1+I/z+sAv81zem3gt/jP8zvAa/OcEM/fbwOvxX+e1wF+m+f02sBv8Z/nd4DX5jkhnr/fBl6L/zyvA/w2z+m1gd/iP8/vAK/Nc0I8f78NvBb/eV4H+G2e02sDv8V/nt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/PXwO7PKfjwEvzn+d3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/zx/A+zynI4DL8V/nt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jnAZ/OcEM/fbwOvxf8tbwP8NM8J8fz9NvBa/N/xN8BL87wQz99vA6/F/w1/A7w1cCvPC/H8/TbwWvzv9gzgu4GvBnZ5/hDP328Dr8W/7H2A7+Z/L8Tz99vAa/HCvQ/w3fzvhnj+fht4LV6w9wG+m//9EM/fbwOvxfP3PsB3838D4vn7beC1eF7vA3w3/3cgnr/fBl6L5/Q+wHfzfwvi+ftt4LV4tvcBvpv/exDP328Dr8UV7wN8N/83IZ6/3wZeC3gf4Lv5vwvx/P028N3Ad/N/G+L5e2ngr/m/D/H/G+L/N8T/b4j/3xD/v/GPlDOeQaznA2sAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAnnouncement;
impl IconShape for MdAnnouncement {
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 9h-2V5h2v6zm0 4h-2v-2h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdApi;
impl IconShape for MdApi {
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
                d: "M14,12l-2,2l-2-2l2-2L14,12z M12,6l2.12,2.12l2.5-2.5L12,1L7.38,5.62l2.5,2.5L12,6z M6,12l2.12-2.12l-2.5-2.5L1,12 l4.62,4.62l2.5-2.5L6,12z M18,12l-2.12,2.12l2.5,2.5L23,12l-4.62-4.62l-2.5,2.5L18,12z M12,18l-2.12-2.12l-2.5,2.5L12,23l4.62-4.62 l-2.5-2.5L12,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAppBlocking;
impl IconShape for MdAppBlocking {
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
                d: "M18 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm-2.5 4c0-1.38 1.12-2.5 2.5-2.5.42 0 .8.11 1.15.29l-3.36 3.36c-.18-.35-.29-.73-.29-1.15zm2.5 2.5c-.42 0-.8-.11-1.15-.29l3.36-3.36c.18.35.29.73.29 1.15 0 1.38-1.12 2.5-2.5 2.5zM17 18H7V6h10v1h2V3c0-1.1-.9-2-2-2H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-4h-2v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowCircleDown;
impl IconShape for MdArrowCircleDown {
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
                d: "M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10 c5.52,0,10-4.48,10-10C22,6.48,17.52,2,12,2L12,2z M13,12l0-4h-2l0,4H8l4,4l4-4H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowCircleUp;
impl IconShape for MdArrowCircleUp {
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
                d: "M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20 M12,22c5.52,0,10-4.48,10-10c0-5.52-4.48-10-10-10 C6.48,2,2,6.48,2,12C2,17.52,6.48,22,12,22L12,22z M11,12l0,4h2l0-4h3l-4-4l-4,4H11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAChElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP97PBjYBXb5j4P43+O1ga8CXgfY5T8G4n+P1wZ+C/hr4HWAXf79EP97vDbwW1zx18DrALv8+yD+93ht4Ld4tr8GXgfY5d8O8b/HawO/xXP6a+B1gF3+bRD/e7w28Fs8r78GXgfY5V8P8b/HawO/xfP318DrALv86yBedC8NfBX/fY4DL80L9tfA6wC7vOgQL7rXBn6L/9n+GngdYJcXDeJF99rAb/E/318DrwPs8i9DvOheG/gt/uf7G+C1gV3+ZYgX3WsDv8X/bH8DvDawy4sG8aJ7aeCr+e9zHHgpXrC/AV4b2OVFh/jf47WB3+L5+xvgtYFd/nUQ/3u8NvBbPK+/AV4b2OVfD/G/x2sDv8Vz+hvgtYFd/m0Q/3u8NvBbPNvfAK8N7PJvh/jf47WB3+KKvwFeG9jl3wfxv8drA78F/A3w2sAu/36I/z1eG/hq4LWBXf5jIP73eDCwC+zyHwfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPa0w+QdHTTagAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArrowRightAlt;
impl IconShape for MdArrowRightAlt {
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
                d: "M16.01 11H4v2h12.01v3L20 12l-3.99-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxoLgF/zX+NlwaO8aIRLxjiX2ZeNL8DvDb/NX4beC1eNOIFQ/zLzIvmd4DX5r/GbwOvxYtGvGCIf5l50fwO8Nr81/ht4LV40YgXDPEvMy+a3wFem/8avw28Fi8a8YIh/mXmRfM7wGvzX+O3gdfiRSNeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8y8aP4G+Gj+a3w18FK8aMQLhviXmRfN7wCvzX+N3wZeixeNeMEQ/zLzovkd4LX5r/HbwGvxohEvGOJfZl40vwO8Nv81fht4LV404gVD/MvMi+Z3gNfmv8ZvA6/Fi0a8YIh/mXnR/DXw0fzX+GrgpXnRiBcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvMy+a3wFem/8Yvw28Fv8xxAuG+JeZF83vAK/Nf4zfBl6L/xjiBUP8y8yL5neA1+Y/xm8Dr8V/DPGCIf5l5kXzO8Br8x/jt4HX4j+GeMEQ/zLzovkd4LX5j/HbwGvxH0O8YIh/mXnR7AJ/zX+MlwaO8x9DvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKKBmJBctWl0wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArticle;
impl IconShape for MdArticle {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0s8r5cGjvGvcwn4a/7rmef1OsBvA4hne23gt3he4nn9NvBa/Ov8DvDa/Nczz+t1gN8GEM/22sBv8bzE8/pt4LX41/kd4LX5r2ee1+sAvw0gnu21gd/ieYnn9dvAa/Gv8zvAa/Nfzzyv1wF+G0A822sDv8XzEs/rt4HX4l/nd4DX5r+eeV6vA/w2gHi21wZ+i+clntdvA6/Fv87vAK/Nfz3zvF4H+G0A8WyvDfwWz0s8r98GXovn9DvAa/M/j3lerwP8NoB4ttcGfovnJZ7XbwOvxXP6HeC1+Z/HPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Nr8z2Oe1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4bf7nMc/rdYDfBhDP9trAb/G8xPP6beC1eE6/A7w2//OY5/U6wG8DiGd7beC3eF7ief028Fo8p98BXpv/eczzeh3gtwHEs7028Fs8L/G8fht4LZ7T7wCvzf885nm9DvDbAOLZXhv4LZ6XeF6/DbwWz+l3gNfmfx7zvF4H+G0A8WyvDfwWz0v8z/PbwGvxnH4HeG2el3lerwP8NoB4ttcGfovnJf7n+W3gtXhOvwO8Ns/LPK/XAX4bQDzbawO/xfMS//P8NvBaPKffAV6b52We1+sAvw0gnu21gd/ieYn/eX4beC2e0+8Ar83zMs/rdYDfBhDP9trAb/G8xP88vw28Fs/pd4DX5nmZ5/U6wG8DiGd7beC3eF7if57fBl6L5/Q7wGvzvMzzeh3gtwHEs7028Fs8L/E/z28Dr8Vz+h3gtXle5nm9DvDbAOLZXhv4LZ6X+J/nt4HX4jn9DvDaPC/zvF4H+G0A8WyvDfwWz0v81/tt4LX41/kd4LV5XuZ5vQ7w2wDi2V4b+C2el/iv99vAa/Gv8zvAa/O8zPN6HeC3AcSzvTbwWzwv8V/vt4HX4l/nd4DX5nmZ5/U6wG8DiGd7beC3eF7iv95vA6/Fv87vAK/N8zLP63WA3wYQz/bawG/xvMR/vd8GXot/nd8BXpvnZZ7X6wC/DSCe7bWB3+J5if96Lw0c519nF/hrnpd5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3her83/br/N83od4LcBxLO9NvBb/P/wOsBvA4jntAsc4/+2S8BxrkA8p88GPov/2z4H+GyuQDyv7wbei/+bvgd4b54N8fy9N/DRwEvxf8PvAN8NfDfPCfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BBGgwEGe8XQdAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAspectRatio;
impl IconShape for MdAspectRatio {
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
                d: "M19 12h-2v3h-3v2h5v-5zM7 9h3V7H5v5h2V9zm14-6H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+xtgl+d0HHgp/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mXnRXAL+muf1Wjyv1wF+m+f02sBv8R9PvGCIf5l50fwO8No8L/O8Xgf4bZ7TawO/xX888YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fv/xxAuG+JeZF83vAK/N8zLP63WA3+Y5vTbwW/zHEy8Y4l9mXjS/A7w2z8s8r9cBfpvn9NrAb/EfT7xgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8V/PPGCIf5l5kXzO8Br87zM83od4Ld5Tq8N/Bb/8cQLhviXmRfN7wCvzfMyz+t1gN/mOb028Fv8xxMvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvH6H5/XSwDFeNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi+R3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8WLRrxgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8XzEs/rt4HX4kUjXjDEv8y8aH4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxohEvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvMTz+m3gtXjRiBcM8S8zL5rfAV6b52We1+sAv81zem3gt3he4nn9NvBavGjEC4b4l5kXze8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC1eNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi2QX+muf12jyvvwZ2eU7HgZfmef02z+ulgeO8aMQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj3ZSqQTTQwmcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssessment;
impl IconShape for MdAssessment {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxoLgF/zX+NlwaO8aIRLxjiX2ZeNL8DvDb/NX4beC1eNOIFQ/zLzIvmd4DX5r/GbwOvxYtGvGCIf5l50fwO8Nr81/ht4LV40YgXDPEvMy+a3wFem/8avw28Fi8a8YIh/mXmRfM7wGvzX+O3gdfiRSNeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8y8aP4G+Gj+a3w18FK8aMQLhviXmRfN7wCvzX+N3wZeixeNeMEQ/zLzovkd4LX5r/HbwGvxohEvGOJfZl40vwO8Nv81fht4LV404gVD/MvMi+Z3gNfmv8ZvA6/Fi0a8YIh/mXnR/DXw0fzX+GrgpXnRiBcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvMy+a3wFem/8Yvw28Fv8xxAuG+JeZF83vAK/Nf4zfBl6L/xjiBUP8y8yL5neA1+Y/xm8Dr8V/DPGCIf5l5kXzO8Br8x/jt4HX4j+GeMEQ/zLzovkd4LX5j/HbwGvxH0O8YIh/mXnR7AJ/zX+MlwaO8x9DvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJyqH5BiPxMkAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignment;
impl IconShape for MdAssignment {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEr0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzn+BtglyuOAy/Ffw7xgiH+ZeY/ziXgq4HvBm7lOT0YeG/go4Fj/McRLxjiX2b+Y/wN8NbArbxwDwZ+Gngp/mOIFwzxLzP/fn8DvDawy4vmOPDXwIP49xMvGOJfZv79Xgb4a/51Xhv4Lf79xAuG+JeZf5/vAd6bf5vvBt6Lfx/xgiH+Zebf522An+bf5q2Bn+LfR7xgiH+Z+fd5CHAr/zYPBp7Ov494wRD/MvPvI/59zL+PeMEQ/zLz7yP+7Y4DF/n3ES8Y4l9m/n1eB/ht/m1eG/gt/n3EC4b4l5l/n68BPpp/m68GPop/H/GCIf5l5t9nF3gIsMu/znHg6cBx/n3EC4b4l5l/v+8G3od/ne8C3pt/P/GCIf5l5j/GdwPvw4vmu4D35j+GeMEQ/zLzH+e3gfcBbuX5e2ngq4DX5j+OeMEQ/zLzH++vgZ8GbuWKBwNvDbw0//HEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/eH8D7PKcjgMvxX888YIh/mXm3+cZwG8Dvw38NfDXvHAvDbw08NrAawMP4t9HvGCIf5n517sE/DTw08BP8+/z1sBbA28NHONfT7xgiH+ZedFdAr4a+Gpgl/9Yx4GPBj4aOMaLTrxgiH+ZedH8DfDWwK3853ow8NPAS/GiES8Y4l9m/mXfA7w3/7V+Gngr/mXiBUP8y8wL9zfAawO7/Nc6Dvw28FK8cOIFQ/zLzAv3PsB389/jvYHv4oUTLxjiX2ZeuIcAt/Lf48HA03nhxAuG+JeZF0789zIvnHjBEP8y88KJ/17mhRMvGOJfZl448d/LvHDiBUP8y8wL99r89/ptXjjxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IMuKgQV4wB+AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignmentInd;
impl IconShape for MdAssignmentInd {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 4c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1.4c0-2 4-3.1 6-3.1s6 1.1 6 3.1V19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+xjgr3lOLw18Ff/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Mf7a2CX53QceGn+44kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z//H+BtjlOR0HXor/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JftAsf43+kScJwXDPEv+27gvfjf6XuA9+YFQ/zLHgw8nf+dHgLcyguGeNG8N/Bd/O/yPsB388IhXnQPBj4beGvgGP8zXQJ+Gvhs4Fb+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I6tgiEGuGUSzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignmentLate;
impl IconShape for MdAssignmentLate {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-6 15h-2v-2h2v2zm0-4h-2V8h2v6zm-1-9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzn+RvgpfjPJV4wxL/M/Of4G+C1gYv85xIvGOJfZv7j/Q3w2sAuYP5ziRcM8S8z/7H+BnhtYJcrzH8u8YIh/mXmP87fAK8N7PJs5j+XeMEQ/zLzH+NvgNcGdnlO5j+XeMEQ/zLz7/c3wGsDuzwv859LvGCIf5n59/kb4LWBXZ4/859LvGCIf5n5t/sb4LWBXV6w1+ZF89LAV/GvJ14wxL/M/Nv8DfDawC7/MV4b+C3+9cQLhviXmX+9vwFeG9jlP85rA7/Fv554wRD/MvOv8zfAawO7/Md6beC3+NcTLxjiX2ZedH8DvDawy3+81wZ+i3898YIh/mXmRbMLPATY5T/HawO/xb+eeMEQ/zLzovtu4H34z/HawG/xrydeMMS/zPzrfDfwPvzHe23gt/jXEy8Y4l9m/vW+G3gf/mO9NvBb/OuJFwzxLzP/Nt8NvA//cV4b+C3+9cQLhviXmX+77wbeh/8Yrw38Fv964gVD/MvMv893A+/DC2b+c4kXDPEvM/9+3w28D8+f+c8lXjDEv8z8x/hu4H14XuY/l3jBEP8y8x/nu4H34TmZ/1ziBUP8y8x/rO8G3odnM/+5xAuG+JeZ/3jfDbwPV5j/XOIFQ/zLzH+O7wbeBzD/ucQLhviXmf883w28N/+5xAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JftAsf43+kScJwXDPEv+27gvfjf6XuA9+YFQ/zLHgw8nf+dHgLcyguGeNG8N/Bd/O/yPsB388IhXnQPBj4beGvgGP8zXQJ+Gvhs4Fb+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8Ix++mkHwx+FWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignmentReturn;
impl IconShape for MdAssignmentReturn {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm4 12h-4v3l-5-5 5-5v3h4v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+B3gtXnR/DbwWvzHEC8Y4l9m/mP8DvDavGh+G3gt/mOIFwzxLzP/MX4HeG1eNL8NvBb/McQLhviXmf8YvwO8Ni+a3wZei/8Y4gVD/MvMf4zfAV6bF81vA6/FfwzxgiH+ZeY/xu8Ar82L5reB1+I/hnjBEP8y8x/jd4DX5kXz28Br8R9DvGCIf5n5j/E7wGvzovlt4LX4jyFeMMS/zPzH+B3gtXnR/DbwWvzHEC8Y4l9m/mP8DvDavGh+G3gt/mOIFwzxLzMvut/hBftr4KN50Xw18NK8YK/Fi068YIh/mXnRfTfwPvzn+i7gvXnRiRcM8S8z/zrfDbwP/zm+C3hv/nXEC4b4l5l/ve8G3of/WN8FvDf/euIFQ/zLzL/NdwPvw3+M7wLem38b8YIh/mXm3+67gffh3+e7gPfm3068YIh/mfn3+W7gffi3+S7gvfn3ES8Y4l9m/v2+G3gf/nW+C3hv/v3EC4b4l5n/GN8NvA8vmu8C3pv/GOIFQ/zLzH+c7wbehxfuu4D35j+OeMEQ/zLzH+u7gffh+fsu4L35jyVeMMS/zPzH+27gfXhO3wW8N//xxAuG+JeZ/xzfDbwPV3wX8N785xAvGOJfZv7zfDdwCfgo/vOIFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHVTeLQXKvNy0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignmentReturned;
impl IconShape for MdAssignmentReturned {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 15l-5-5h3V9h4v4h3l-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/Fv83bAD/Nfx7Ei+7BwGcBbw0c59/vc4CvBo4D7w18Fv9+u8BPA58D3Mq/DPGieW/gu/iP8zPAW/Ocfhp4K/7jvA/w3bxwiH/Zg4Gn8x/rfYDv5jl9NPBV/Md6CHArLxjiX/bdwHvxH+trgI/mOX018FH8x/oe4L15wRD/sovAcf5j7QKvA/w1V7w08FvAcf5j7QIneMEQ/zLzn+e3ueK1+c8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z8z/E3wHcDX8WLTrxgiH+Z+Z/hb4DXBnaB9wa+ixeNeMEQ/zLz3+9vgNcGdnm29wa+i3+ZeMEQ/zLz3+tvgNcGdnlenw18Fi+ceMEQ/zLz3+dvgNcGdnn+vgt4b1448YIh/mXmv8ffAK8N7PL8fRfw3vzLxAuG+JeZ/3p/A7w2sMvz913Ae/OiES8Y4l9m/mv9DfDawC7P33cB782LTrxgiH+Z+a/zN8BrA7s8f98FvDf/OuIFQ/zLzH+NvwFeG9jl+fsu4L351xMvGOJfZv7z/Q3w2sAuz993Ae/Nv414wRD/MvOiuQS8NvDRwHvxovsb4LWBXZ6/7wLem3878YIh/mXmX3YJeG3gr7niu4H34l/2N8BrA7s8f98FvDf/PuIFQ/zLzAt3CXht4K95Tt8NvBcv2N8Arw3s8vx9F/De/PuJFwzxLzMv3OsAv83z993Ae/G8/gZ4bWCX5++7gPfmP4Z4wRD/MvPC/TXwOsAuz993A+/Fs/0N8NrALs/fdwHvzX8c8YIh/mXmX/bXwOsAuzx/3w28F/A3wGsDuzx/3wW8N/+xxAuG+JeZF81fA68D7PL8fTbw1cAuz993Ae/NfzzxgiH+ZeZF99fA6wC7/Ot8F/De/OcQLxjiX2b+df4aeB1glxfNdwHvzX8e8YIh/mXmX++vgdcBdnnhvgt4b/5ziRcM8S8z/zZ/DbwOsMvz913Ae/OfT7xgiH+Z+bf7a+B1gF2e03cB781/DfGCIf5l5t/nr4HXAXa54ruA9+a/jnjBEP8y8+/318DrAF8FvDf/tcQLhviXmf8Yu8Bx/uuJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S/bBY7xv9Ml4DgvGOJf9t3Ae/G/0/cA780LhviXPRh4Ov87PQS4lRcM8aJ5b+C7+N/lfYDv5oVDvOgeDHw28NbAMf5nugT8NPDZwK38yxD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4Rx2dtkFFC3XxAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAssignmentTurnedIn;
impl IconShape for MdAssignmentTurnedIn {
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-2 14l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutorenew;
impl IconShape for MdAutorenew {
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
                d: "M12 6v3l4-4-4-4v3c-4.42 0-8 3.58-8 8 0 1.57.46 3.03 1.24 4.26L6.7 14.8c-.45-.83-.7-1.79-.7-2.8 0-3.31 2.69-6 6-6zm6.76 1.74L17.3 9.2c.44.84.7 1.79.7 2.8 0 3.31-2.69 6-6 6v-3l-4 4 4 4v-3c4.42 0 8-3.58 8-8 0-1.57-.46-3.03-1.24-4.26z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4Dj/NfYBV4H+Gv+fRD/fseBpwPH+a+1C7wMcCv/doh/v78CXpr/Hr8NvA7/doh/n68GPor/Xp8DfDb/Noh/u5cG/or/GV4H+G3+9RD/eq8FvDTw3sBL8+/zN1zxUvz7fA/w3vzrIV40rw28F/De/Mf5G+C1ueK3gZfi3+chwK386yBeuJcGvgp4bf5j/Q3w2sAuVxwHfht4Kf7tvgd4b/51EC/YZwGfzX+8vwFeG9jlOR0Hfht4Kf5tdoET/Osgnr/vAt6b/3h/A7w2sMvzdxz4beCl+Ld5GeCvedEhntdPAW/Nf7y/AV4b2OWFOw78NvBS/Ot9N/AxwC4vGsRz+mrgo/iP9zfAawO7vGiOA78NvBT/NrvAbwPfDfwMLxji2d4a+Cn+4/0N8NrALv86x4HfBl6Kf59bgY8BfprnhXi2pwMP5j/W3wCvDezyb3Mc+G3gpfj3+23gbYBdng1xxXsD38V/rL8BXhvY5d/nOPDbwEvx7/fXwOsAu1yBuOLpwIP5j/M3wGsDu/zHOA78NvBS/Pv9NfA6wC6AgJcG/or/OH8DvDawy3+s48BvAy/Fv993A+8DIOCzgc/iP8bfAK8N7PKf4zjw28BL8e/3OsBvC/ht4LX499sF3hvY5fm7BPw1L5qXBo7x/B0Hvhs4zr/PbwOvI+C3gdfiP9/vAK/Ni+a3gdfiP9/LCDD/NX4HeG1eNL8NvBb/+T5GgPmv8TvAa/Oi+W3gtfjP9zsCbgUexH++3wFemxfNbwOvxX++vxbw28Br8Z/vd4DX5kXz28Br8Z8PAT8NvBX/+X4HeG1eNL8NvBb/+RDw3sB38Z/vd4DX5kXz28Br8Z/vbwQcBy7yn+93gNfmRfPbwGvxn+93xBW/DbwW/7l+B3htXjS/DbwW//k+Rlzx2sBv8Z/rd4DX5kXz28Br8Z/vIeLZfhp4K/7z7AJ/zYvmpYHj/Of6HeC1xbM9GPhr4Bj/P7wO8NviOb028Fv83/c9wHsDiOf13sB38X/X3wCvDewCiOfvrYHvBo7xf8vfAG8N3MoViBfspYGfBh7E/w2/A7w1sMuzIf5l7w18NvAg/nd6BvDZwHfzvBAvurcG3hp4a+AY//P9DPDTwE8Duzx/iH+71+Z/pl3gr3nRIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Av92po5KirvwAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBackup;
impl IconShape for MdBackup {
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBackupTable;
impl IconShape for MdBackupTable {
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
                d: "M20,6v14H6v2h14c1.1,0,2-0.9,2-2V6H20z",
            }
            path {
                d: "M16,2H4C2.9,2,2,2.9,2,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C18,2.9,17.1,2,16,2z M9,16H4v-5h5V16z M16,16h-5v-5h5 V16z M16,9H4V4h12V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8Y4D7wW8NPBg/mPcCvw18D3ALv9xEP+xXhr4KeDB/Oe4FXgb4K/5j4H4j/VXwEvzn+uvgZfhPwbiP85LA3/Ff42XAf6afz/Ef5zjwEvzX+OvgV3+/RD/vyH+f0P8/4b4j/NewEsDL81/rr8G/hr4Hv79EP8xfgt4bf5r/TbwOvz7IP79Phv4LP57fA7w2fzbIf79/gp4af57/DXwMvzbIf79Xpv/Xr/Nvx3i/zfE/2+I/98Q/34vDbwV8NLAcf5r7AJ/DfwM8Nf82yH+fb4K+Gj+e3018DH82yD+7b4a+Cj+Z/ga4KP510P827w08Ff8z/IQ4Fb+dRD/Np8NfBb/dj8D/DVwK1c8GHhp4K34t/sc4LP510H82/w08Fb861wCvhr4amCX5+848NHAZ/Gv9zPAW/Ovg/i3+W3gtXjRXQJeG/hrXjQvDfw2cIwX3e8Ar82/DuLf5reB1+JFcwl4aeBW/nUeDPw1cIwXze8Ar82/DuLf5reB1+JF8zrAb/Nv89rAb/Gi+R3gtfnXQfzb/DbwWvzLfgZ4a/59fht4Lf5lvwO8Nv86iH+b3wZei3/Z6wC/zb/PWwM/xb/sd4DX5l8H8W/z28Br8cJdAo7zH2MXOMYL9zvAa/Ovg/i3+W3gtXjhfgd4bf5j/DbwWrxwvwO8Nv86iH+b3wZeixfud4DX5j/GbwOvxQv3O8Br86+D+Lf5beC1eOF+B3ht/mP8NvBavHC/A7w2/zqIf5vfBl6Lf5n4j2H+Zb8DvDb/Ooh/m98GXot/2dsAP82/z3sD38W/7HeA1+ZfB/Fv89vAa/Ev+23gdfj3+S3gtfmX/Q7w2vzrIP5tfht4LV407wN8N/827w18Fy+a3wFem38dxL/NbwOvxYtmF3gIsMu/znHg6cBxXjS/A7w2/zqIf5vfBl6LF91vA6/Dv85vAa/Ni+53gNfmXwfxb/PbwGvxr/M5wGfzovls4LP41/kd4LX510H82/w28Fr8670O8Nu8cK8N/Bb/er8DvDb/Ooh/m98GXot/vVuBlwF2ef6OA08HjvOv9zvAa/Ovg/i3+W3gtfi3+Rzgs3n+Phv4LP5tfgd4bf51EP82vw28Fv82u8AJntdx4OnAcf5tfgd4bf51EP82vw28Fv92rwP8Ns/ptYHf4t/ud4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxb/d3wC7PKfjwEvxb/c7wGvzr4P4t/lt4LX4t3sd4Ld5Tq8N/Bb/dr8DvDb/Ooh/m98GXot/u9cBfpvn9NrAb/Fv9zvAa/Ovg/i3+Wngrfi3ex3gt3lOrw38Fv92PwO8Nf86iH+bzwY+i3+7jwb+muf00sBX82/3OcBn86+D+Ld5aeCv+J/lIcCt/Osg/u2+Gvgo/mf4GuCj+ddD/Pt8NfBR/Pf6GuCj+bdB/Pu9NPDWwEsDx/mvsQv8NfDdwK382yH+f0P8/4b4/w3x/xvi/zf+EWqTpUFYag7MAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatchPrediction;
impl IconShape for MdBatchPrediction {
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
                d: "M17,8H7c-1.1,0-2,0.9-2,2v10c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V10C19,8.9,18.1,8,17,8z M13,20.5h-2V19h2V20.5z M13,18h-2 c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5c1.93,0,3.5,1.57,3.5,3.5C15.5,15,13,16.5,13,18z M18,6.5H6v0C6,5.67,6.67,5,7.5,5h9 C17.33,5,18,5.67,18,6.5L18,6.5z M17,3.5H7v0C7,2.67,7.67,2,8.5,2h7C16.33,2,17,2.67,17,3.5L17,3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw/71uBf4a+B5gl38dxL/eewNfBRznf5Zd4GOA7+ZFh/jXeW3gt/if7XWA3+ZFg/jXeTrwYP5nuxV4CC8axIvupYG/4n+HlwH+mn8Z4kX32sBv8bx+h3+9lwaO8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8bzeGvgs4KeBz+F5/TbwWvzneR3gt/mXIV50rw38Fs9LPNuDge8CXptnuxV4H+C3ebbfBl6L/zyvA/w2/zLEi+61gd/ieQk4DnwU8Nm8YD8NfAxwK/DbwGvxn+d1gN/mX4Z40b028Fs8r7cBvgp4MP+yXeCrgdcBXov/PK8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvupcG/or/HV4G+Gv+ZYh/nVuBB/E/2zOAB/OiQfzrvDbwW/zP9jrAb/OiQfzrvTfw1cAx/me5BLw38NO86BD/NseB9wZeGngw/71uBf4a+G5gl38dxP9viP/fEP+/If5/Q/z/xj8C4/CSQd+fOzsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBook;
impl IconShape for MdBook {
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
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBookOnline;
impl IconShape for MdBookOnline {
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
                d: "M17,1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M7,18V6h10v12H7z M16,11V9.14 C16,8.51,15.55,8,15,8H9C8.45,8,8,8.51,8,9.14l0,1.96c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1l0,1.76C8,15.49,8.45,16,9,16h6 c0.55,0,1-0.51,1-1.14V13c-0.55,0-1-0.45-1-1C15,11.45,15.45,11,16,11z M12.5,14.5h-1v-1h1V14.5z M12.5,12.5h-1v-1h1V12.5z M12.5,10.5h-1v-1h1V10.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAELUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zkOfBTw3sCD+a91K/DdwNcAu/zbIP7tHgz8FPDS/Pf6a+BtgFv510P82/0V8NL8z/DXwMvwr4f4t3lr4Kf4n+VtgJ/mXwfxb/PZwGfxP8vnAJ/Nvw7i3+angbfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv8+3wN8NXAceG/gvfj3+R3gtfnXQfzb/DbwWvzrPQP4buCrgV2e04OB9wY+GjjGv97vAK/Nvw7i3+a3gdfiRfc7wHcD382L5r2BjwZeihfd7wCvzb8O4t/mt4HX4l/2PcBXA3/Nv81rA+8NvBf/st8BXpt/HcS/zW8Dr8Xz9wzgu4GvBnb5j/Fg4L2BjwaO8fz9DvDa/Osg/m1+G3gtntPvAN8NfDf/ud4b+GjgpXhOvwO8Nv86iH+b3wZeiyu+B/hq4K/5r/XawHsD78UVvwO8Nv86iH+b7wZuBb4a2OW/14OB9wZeGnhr/nUQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Efxg3tBbE6+8gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBookmark;
impl IconShape for MdBookmark {
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBookmarkBorder;
impl IconShape for MdBookmarkBorder {
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/z4OBB/Ff73d40SH+4x0Hfgp4bf577AJfDXwO/zLEf6zjwNOB4/z3+27gfXjhEP+xfgt4bf7neB/gu3nBEP9xHgw8nf9Zfgd4bV4wxH+c1wZ+i/95xAuG+N/rwcB7Ax8NHOMFEy8Y4n+/lwZ+GngQz594wRD/N7w08Fc8f+IFQ/zf8dPAW/G8xAuG+L/js4HP4nmJFwzxr/NWwEsDr81/nVuBvwa+B9jlBXtt4Ld4XuIFQ7xoHgz8FPDS/PfZBT4G+G6ev9cGfovnJV4wxL/sOPBXwIP5n+F1gN/meb028Fs8L/GCIf5lnw18Fv9z3Ao8hOf12sBv8bzEC4b4lz0deDD/s7wM8Nc8p9cGfovnJV4wxL/M/M/zOsBv85xeG/gtnpd4wRD/MvM/z+sAv81zem3gt3he4gVD/MvM/zyvA/w2z+m1gd/ieYkXDPEvM//zvA7w2zyn1wZ+i+clXjDEv8z8z/M6wG/znF4b+C2el3jBEP8y8z/P6wC/zXN6beC3eF7iBUP8y8z/PK8D/DbP6bWB3+J5iRcM8S8z//O8DvDbPKfXBn6L5yVeMMS/zPzP8zrAb/OcXhv4LZ6XeMEQ/zLzP8/rAL/Nc3pt4Ld4XuIFQ/zLzP88rwP8Ns/ptYHf4nmJFwzxLzP/87wO8Ns8p9cGfovnJV4wxL/M/M/zOsBv85xeG/gtnpd4wRD/MvM/z+sAv81zem3gt3he4gVD/MvM/zyvA/w2z+m1gd/ieYkXDPEvM//zvA7w2zyn1wZ+i+clXjDEv8z8z/M6wG/znF4b+C2el3jBEP8y8z/P6wC/zXN6beC3eF7iBUP8y8z/PK8D/DbP6bWB3+J5iRcM8S8z//O8DvDbPKfXBn6L5yVeMMS/zPzP8zrAb/OcXhv4LZ6XeMEQ/zLzP8/rAL/Nc3pt4Ld4XuIFQ/zLzP88rwP8Ns/ptYHf4nmJFwzxLzP/87wO8Ns8p9cGfovnJV4wxL/M/M/zOsBv85xeG/gtnpd4wRD/MvM/z+sAv81zem3gt3he4gVD/MvM/zyvA/w2z+m1gd/ieYkXDPEvM//zvA7w2zyn1wZ+i+clXjDEv8z8z/M6wG/znF4b+C2el3jBEP8y8z/P6wC/zXN6beC3eF7iBUP8y8z/PK8D/DbP672BzwYexLOJFwzxLzP/87wO8Nu8YO8NfDbwIEC8YIh/mfmf53WA3+Zf9t7Ad/OCIf5l5n+e1wF+m38/xL/M/M/zOsBv8++H+JeZf79nAF8NvDbwVvz7vQ7w2/z7If5l5t/uZ4CvBn6bZ3sw8NHAewPH+Ld5HeC3+fdD/MvMv84l4LuBrwZu5QU7Drw18NnAg/jXeR3gt/n3Q/zLzIvmGcBnAz8N7PKv89rARwNvxYvmdYDf5t8P8S8zL9zPAF8N/Db/fg8GPhp4b+AYL9jrAL/Nvx/iX2ae1yXgu4GvBm7lP95x4K2BzwYexPN6HeC3+fdD/MvMsz0D+Gzgp4Fd/mu8NvDRwFvxbK8D/Db/foh/mYGfAb4a+G3++zwY+GjgvYG3Bn6bfz/Ev+zBwK38z3GcK3b590P8/4b4/41/BFv/tkFtlXSNAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBookmarks;
impl IconShape for MdBookmarks {
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
                d: "M19 18l2 1V3c0-1.1-.9-2-2-2H8.99C7.89 1 7 1.9 7 3h10c1.1 0 2 .9 2 2v13zM15 5H5c-1.1 0-2 .9-2 2v16l7-3 7 3V7c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/HSwFcBbwPs8u9zHPgp4GOAv+Y/FuI/3ksDvwUcB/4aeB1gl3+b48BvAS8N7AKvA/w1/3EQ/7FeGvgt4DjP9tfA6wC7/OscB34LeGmebRd4HeCv+Y+B+I/zYOCvgOM8r78GXgfY5UVzHPgt4KV5XrvAywC38u+H+I/13cB78fz9NfA6wC4v3HHgt4CX5vn7HuC9+Y+B+I/33cB78fz9NfDRwGtzxUtzxV9zxW8DXw28NM/f9wDvzX8cxH+O7wbei/9Y3wO8N/+xEP95vht4L/5jfA/w3vzHQ/zn+Szgs/mP8dnA5/AfD/Ef7zjwXcBb8x/ru4H34T8W4j/ebwGvzX+Onwbehv84iP9Y3wW8N/+5vht4H/5jIP7jvDXwU/zXeB3gt/n3Q7zoXhr4Kl6wlwaO819jF/hrXrCPAf6afxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedC8NfDXP66WBY/zLLgF/zYvmpYFj/MsuAX/N8/po4K/5lyH+/Z4OPJh/2e8Ar82L5reB1+JfdivwEP7tEP9+5kXzO8Br86L5beC1eNGIfzvEv5950fwO8Nq8aH4beC1eNOLfDvHvZ140vwO8Ni+a3wZeixeN+LdD/PuZF83vAK/Ni+a3gdfiRSP+7RAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRHQdemuf1W7xo/gb4aF40Xw28FC+a1+F5/TWwy78M8e9nXjS/A7w2L5rfBl6LF434t0P8+5kXze8Ar82L5reB1+JFI/7tEP9+5kXzO8Br86L5beC1eNGIfzvEv98ucIx/2e8Ar82L5reB1+Jfdgk4zr8d4kV3HHgpntdXAy/Nv+yvgY/mRfPVwEvzL/tr4KN5Xn8D7PIvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4j/OZwOfxX+NzwE+m38/xH+svwZeiv9cfwO8NP8xEP+xjgO3Asf4z3EJeDCwy38MxH+8lwZ+GzjGf6xLwGsDf81/HMR/juPAbwMvxX+MvwFeG9jlPxbiP9dnA5/Fv8/nAJ/Nfw7Ef74HA58NvDVwjBfNJeCngc8GbuU/D+K/1nsDLw28NPDSwDGuuAT8NfDXwF8D381/DcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EWVPlUGuhHMJAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBugReport;
impl IconShape for MdBugReport {
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
                d: "M20 8h-2.81c-.45-.78-1.07-1.45-1.82-1.96L17 4.41 15.59 3l-2.17 2.17C12.96 5.06 12.49 5 12 5c-.49 0-.96.06-1.41.17L8.41 3 7 4.41l1.62 1.63C7.88 6.55 7.26 7.22 6.81 8H4v2h2.09c-.05.33-.09.66-.09 1v1H4v2h2v1c0 .34.04.67.09 1H4v2h2.81c1.04 1.79 2.97 3 5.19 3s4.15-1.21 5.19-3H20v-2h-2.09c.05-.33.09-.66.09-1v-1h2v-2h-2v-1c0-.34-.04-.67-.09-1H20V8zm-6 8h-4v-2h4v2zm0-4h-4v-2h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8S/3UsDbwW8NvBg4ME8p98GdoGfBn4HuJX/eRD/eseBvwIezL/ObwOfA/w2/3Mg/m3eG/gu/m1+G/gc4Lf574f4t3tv4Lv4t/to4Gv474X493lv4Lv4t/tu4H3474P493tv4Lv4t/tu4H3474H4j/HewHfxb/fdwPvwXw/xH+e9ge/i3+5tgJ/mvxbiP9Z7A9/Fv82twMsAu/zXQTyv48Au/3bvDXwX/zafA3w2/3UQz+nBwF8BHwN8N/927w18F/96u8BDgF3+ayCe01cDH8UV7wN8N/927w18F/967wN8N/81EM/pr4CX5tneB/hu/u3eG/gu/nV+Bnhr/msgnu04cJHn9T7Ad/Nv897Ad/Gvswuc4L8G4tleG/gtnr/3Ab6bf533Br6Lf5uXAf6a/3yIZ3tt4Ld4wd4H+G5eNO8NfBf/dq8D/Db/+RDP9tnAZ/HCvQ/w3bxw7w18F/8+HwN8Nf/5EM/22cBn8S97H+C7ef7eG/gu/v3eBvhp/vMhnu21gd/iRfM+wHfznN4b+C7+Y7wO8Nv850M822sDv8WL7n2A7+aK9wa+i/84rwP8Nv/5EM/J/Ou8D1d8F/+xTgC7/OdDPKffBl6L/17PAB7Mfw3Ec/po4Kv47/U1wEfzXwPxnB4MPJ3/Xh8DfDX/NRDP67uB9+K/1/sA381/PsTzejDw18Ax/nu9D/Dd/OdCPH+fDXwW//3eB/hu/vMgXrC/Bl6K/37vA3w3/zkQL9hx4LeBl+K/3/sA381/PMQL99LAdwMvxX+/9wG+m/9YiH/ZceC7gbfiv9/7AN/NfxzEi+6zgY8GjvHf632A7+Y/BuJf58HAZwPvxX+v9wG+m38/xL/Ng4G3Bl4beCteuJ8Bfht4aeC9+I/zPsB38++D+I/x2lzxYK64lSt+m+f03cB78R/nfYDv5t8O8V/vu4H34j/O+wDfzb8N4r/HdwPvxX+c9wG+m389xH+f7wbei/847wN8N/86iP9e3w28F/9x3gf4bl50iP9+3w28F/9x3gf4bl40iP8Zvht4L/7jvA/w3fzLEP9zfDfwXvzHeR/gu3nhEP+zfDfwXvzHeR/gu3nBEP/zfDfwXvzHeR/gu3n+EP8zfTfwXvzHeR/gu3leiP+5vht4L/7jvA/w3TwnxP9s3w28F/9x3gf4bp4N8T/fdwPvxX+c9wG+mysQ/zt8N/Be/Md5CHArgPjf47uB9+I/xusAvw0g/nf5buC9+Pe5BBznCsT/Pt8NvBf/NpeA1wb+misQ/zt9N/Be/OtcAl4b+GueDfG/13cD78WL5hLw2sBf85wQ/7t9N/BevHCXgNcG/prnhfjf77uB9+L5uwS8NvDXPH+I/xu+G3gvntMl4LWBv+YFQ/zf8dXAR3HFM4C3Bv6aFw7xf8tLA8eBvwZ2+Zch/n9D/P/GPwK1vp5BPrS3ZwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBuild;
impl IconShape for MdBuild {
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
                d: "M22.7 19l-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBuildCircle;
impl IconShape for MdBuildCircle {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10 C22,6.48,17.52,2,12,2z M16.9,15.49l-1.4,1.4c-0.2,0.2-0.51,0.2-0.71,0l-3.41-3.41c-1.22,0.43-2.64,0.17-3.62-0.81 c-1.11-1.11-1.3-2.79-0.59-4.1l2.35,2.35l1.41-1.41L8.58,7.17c1.32-0.71,2.99-0.52,4.1,0.59c0.98,0.98,1.24,2.4,0.81,3.62 l3.41,3.41C17.09,14.98,17.09,15.3,16.9,15.49z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dLA8d4Xr/Dfy3Ef423At4aeG3gwbxwfw38NvDbwM/wnwvxn+fBwGcBbw0c59/mVuCngc8BdvmPh/iPdxz4LOCj+Y+zC3w18DXALv9xEP+xXhv4KeA4//H+BnhtYJf/OIj/OJ8FfDb/Of4GeG1gl/9YiP8Y3wW8N/+ynwF+GrgV2AX+mmd7beClgdcGXhs4xhV/A7w2sMt/PMS/33cB780Ldgn4aOCngV1edO8NvDXw3sAu/zkQ/z6fDXwWL9jnAF8N7PI/E+Lf7rWB3+L5uwS8NvDX/M+G+Lc5DjwdOM7z+hvgtYFd/udD/Nt8NfBRPK9LwIOBXf7rfRbwNcAuLzrEv96Dgafz/L0M8Nf81/su4L2BvwZeB9jlRYP41/tu4L14Xp8DfDb/9b4LeG+e7a+B1wF2+Zch/vUuAsd5TpeABwO7/Nf6LuC9eV5/DbwOsMsLh/jXeWvgp3he7wN8N/+1vgt4b16wvwZeB9jlBUP863w38F48rxPALv91vgt4b/5lfw28DrDL84f413k68GCe088Ab81/ne8C3psX3V8DrwPs8rwQ/zrmeb0P8N381/gu4L351/tr4HWAXZ4T4kX30sBf8bxeB/ht/vN9F/De/Nv9NfA6wC7PhnjRvTbwWzyvlwH+mv9c3wW8N/9+fw28DrDLFYgX3WsDv8XzEv+5vgt4b/7j/DXwOsAugHi248BvAy/Ff5z3Ab6b/1zmeb0O8Nv8yxDP6Tjw28BL8e/3PsB385/rpYG/4nm9DvDb/MsQz+s48NvAS/Fv9z7Ad/Of77WB3+J5vQzw1/zLEM/fceC3gZfiX+99gO/mv8ZHA1/F8xIvGsQLdhz4beCleNG9D/Dd/Nf5aeCteE7PAB7Miwbxwh0Hfht4Kf5l7wN8N/91jgMXeV5fA3w0LxrEv+w48NvAS/GCvQ/w3fzXem/gu3hebwP8NC8axIvmOPDbwEvxvN4H+G7+ax0Hng4c5zldAo7zokO86I4Dvw28FM/2PsB381/vs4HP4nl9DfDRvOgQ/zrHgd8GXgp4H+C7+a/30sBf8fw9BLiVFx3iX+848NrAT/Nf78HAXwHHeV5fA3w0/zqI/z0eDPwU8NI8r2cALw3s8q+D+N/hpYHfAo7z/L0O8Nv86yH+ZzsOfBTw2bxgnwN8Nv82iP9cx4HvAn4a+BlglxfNceCtgK8GjvOCfQ/w3vzbIf7zHAd+C3hprtgFfhv4beCvgd/h2V4aOAa8NPDawFvzL/se4L3590H85zgO/Bbw0vzn+Bzgs/n3Q/zHOw78FvDS/Md7BvDewG/zHwPxH+848NHARwPH+I/zNcBnA7v8x0H85zkOfDbw1sCD+Le5BHw38NXArfzHQ/zXeGvgtYHXBl6KF+4ZwE8Dvw38NP+5EP89Xho4znPaBf6a/1qI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/BzbBBjaln9QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCached;
impl IconShape for MdCached {
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
                d: "M19 8l-4 4h3c0 3.31-2.69 6-6 6-1.01 0-1.97-.25-2.8-.7l-1.46 1.46C8.97 19.54 10.43 20 12 20c4.42 0 8-3.58 8-8h3l-4-4zM6 12c0-3.31 2.69-6 6-6 1.01 0 1.97.25 2.8.7l1.46-1.46C15.03 4.46 13.57 4 12 4c-4.42 0-8 3.58-8 8H1l4 4 4-4H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCalendarToday;
impl IconShape for MdCalendarToday {
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
                d: "M20 3h-1V1h-2v2H7V1H5v2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 18H4V8h16v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/mWvxf9uv8MLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+P8N8f8b4v83xP9viP/fEP+y3+J/t9fhBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLfpv/3V6bFwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+LXRpBYA+wMQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCalendarViewDay;
impl IconShape for MdCalendarViewDay {
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
                d: "M3 17h18v2H3zm0-7h18v5H3zm0-4h18v2H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCameraEnhance;
impl IconShape for MdCameraEnhance {
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
                d: "M9 3L7.17 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-3.17L15 3H9zm3 15c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
            path {
                d: "M12 17l1.25-2.75L16 13l-2.75-1.25L12 9l-1.25 2.75L8 13l2.75 1.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCancelScheduleSend;
impl IconShape for MdCancelScheduleSend {
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
                d: "M16.5,9c-0.42,0-0.83,0.04-1.24,0.11L1.01,3L1,10l9,2l-9,2l0.01,7l8.07-3.46C9.59,21.19,12.71,24,16.5,24 c4.14,0,7.5-3.36,7.5-7.5S20.64,9,16.5,9z M16.5,22c-3.03,0-5.5-2.47-5.5-5.5s2.47-5.5,5.5-5.5s5.5,2.47,5.5,5.5S19.53,22,16.5,22 z",
            }
            polygon {
                points: "18.27,14.03 16.5,15.79 14.73,14.03 14.03,14.73 15.79,16.5 14.03,18.27 14.73,18.97 16.5,17.21 18.27,18.97 18.97,18.27 17.21,16.5 18.97,14.73",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCardGiftcard;
impl IconShape for MdCardGiftcard {
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
                d: "M20 6h-2.18c.11-.31.18-.65.18-1 0-1.66-1.34-3-3-3-1.05 0-1.96.54-2.5 1.35l-.5.67-.5-.68C10.96 2.54 10.05 2 9 2 7.34 2 6 3.34 6 5c0 .35.07.69.18 1H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-5-2c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM9 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm11 15H4v-2h16v2zm0-5H4V8h5.08L7 10.83 8.62 12 11 8.76l1-1.36 1 1.36L15.38 12 17 10.83 14.92 8H20v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0vA/w1zwnx/H028Fn83/I5wGfznBDP308Db8VzugT8Nf87vDRwjOf0M8Bb85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6a+Bj+Z/h68GXprn9DvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBaPKdLwF/zv8NLA8d4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/3w28F/+3fA3w0TwnxPP30cBX8X/LxwBfzXNCPH/HgVuBY/zfcAl4MLDLc0K8YO8NfBf/N7wP8N08L8QL99bAVwMP4n+nZwAfDfw0zx/iRfPSwHFedF8NvBT/sf4G+GhedLvAX/PCIf5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/xy/DbwW/7Lv4Yr34l/2O8Br8x8L8Z/jt4HX4gX7HuCzgVu54sHAZwPvxQv2O8Br8x8L8Z/jt4HX4nl9D/DZwK08fw8GPht4L57X7wCvzX8sxH+O3wZei2f7HuCzgVt50TwY+GzgvXi23wFem/9YiP8cvw28FvA9wGcDt/Jv82Dgs4H3An4HeG3+YyH+c3w08NPArfzHeDDw1sBX8x8L8f8b4v83xP9viP/fEP+/8Y8Xma5Bcg4AggAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCardMembership;
impl IconShape for MdCardMembership {
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
                d: "M20 2H4c-1.11 0-2 .89-2 2v11c0 1.11.89 2 2 2h4v5l4-2 4 2v-5h4c1.11 0 2-.89 2-2V4c0-1.11-.89-2-2-2zm0 13H4v-2h16v2zm0-5H4V4h16v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/OY4D7wW8NPBg/n1uBf4a+B5gl/9YiP947w18FXCc/1i7wMcA381/HMR/rLcGfor/XG8D/DT/MRD/sZ4OPJj/XLcCD+E/BuI/zksDf8V/jZcB/pp/P8R/nNcGfovn9Tv8+7wWz+t1gN/m3w/xH+e1gd/ieYl/H/O8Xgf4bf79EP9xXhv4LZ6X+Pcxz+t1gN/m3w/xH+e1gd/ieYl/H/O8Xgf4bf79EP9xXhv4LZ6X+Pcxz+t1gN/m3w/xH+e1gd/ieYl/H/O8Xgf4bf79EC/cceCjgPcGHsz/LrcC3w18DbDL84d4wV4a+CngwfzvdivwOsCtPC/EC/ZXwEvzf8NfAy/D80I8f28N/BT/t7wN8NM8J8Tz99XAR/F/y+cAn81zQjx/vw28Fs/pb4CP5gV7aeCreF6vw7/Pb/G8Pgb4a16wrwZeiuf0O8Br85wQz99vA6/Fc/od4LV5wV4b+C2el/j3Mc/rdYDf5gX7beC1eE6/A7w2zwnx/P028Fo8p98BXpsX7LWB3+J5iX8f87xeB/htXrDfBl6L5/Q7wGvznBDP328Dr8Vz+h3gtXnBXhv4LZ6X+Pcxz+t1gN/mBftt4LV4Tr8DvDbPCfH8/TbwWjyn3wFemxfstYHf4nmJfx/zvF4H+G1esN8GXovn9DvAa/OcEM/fbwOvxXP6HeC1ecFeGvhqntdr8+/z2zyvjwb+mhfst4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbPaRf4a/53eGngOM/pd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LV4Tn8DfDT/O3w18FI8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e018DH83/Dl8NvDTP6XeA1+Y5IZ6/zwY+i/9bPgf4bJ4T4vl7a+Cn+L/lbYCf5jkhXrC/Bl6K/xv+BnhpnhfiBXtp4KeBB/G/2zOA1wZu5XkhXrjjwEcD7w08iP9dngF8N/DVwC7PH+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CO5OtUExBHNCAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCardTravel;
impl IconShape for MdCardTravel {
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
                d: "M20 6h-3V4c0-1.11-.89-2-2-2H9c-1.11 0-2 .89-2 2v2H4c-1.11 0-2 .89-2 2v11c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zM9 4h6v2H9V4zm11 15H4v-2h16v2zm0-5H4V8h3v2h2V8h6v2h2V8h3v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dxrtjlvwfiv9dnc8Vn898D8d/npYG/4oqXAf6a/3qI/z6/Bbw2V/w28Dr810P89/ho4Kt4Th8DfDX/tRD/9Y4DTweO85x2gYcAu/zXQfzX+yngrXn+fhp4G/7rIP5rvTbwW7xwrwP8Nv81EP91jgN/BTyYF+5W4GWAXf7zIf7rfDbwWbxoPgf4bP7zIf5rvDTwV/zrvAzw1/znQvzX+C3gtXlen8MVn8Xz+m3gdfjPhfjP99HAV/G8ngE8GDgO/DXwIJ7XxwBfzX8exH+u48DTgeM8r9cBfpsrXhv4LZ7XLvAQYJf/HIj/XD8FvDXP63uA9+Y5/TTwVjyvnwbehv8ciP88rw38Fs/rEvBgYJfndBy4FTjG83od4Lf5j4f4z3Ec+CvgwTyv9wG+m+fvo4Gv4nndCrwMsMt/LMR/js8GPovn9TvAa/PC/TbwWjyvzwE+m/9YiP94Lw38Fc/fQ4BbeeFeGvgrnr+XAf6a/ziI/3i/Bbw2z+tzgM/mRfPZwGfxvH4beB3+4yD+Y3008FU8r2cAD+ZFdxz4a+BBPK+PAb6a/xiI/zjHgacDx3lerwP8Nv86rw38Fs9rF3gIsMu/H+I/zk8Bb83z+h7gvfm3+WngrXhePw28Df9+iP8Yrw38Fs/rEvBgYJd/m+PArcAxntfrAL/Nvw/i3+848FfAg3le7wN8N/8+Hw18Fc/rVuBlgF3+7RD/fp8NfBbP63eA1+Y/xm8Dr8Xz+hzgs/m3Q/z7vDTwVzx/LwP8Nf8xXhr4K56/lwH+mn8bxL/PbwGvzfP6HOCz+Y/12cBn8bx+G3gd/m0Q/3YfDXwVz+sZwEsDu/zHOg78NfAgntfHAF/Nvx7i3+Y48HTgOM/rdYDf5j/HawO/xfPaBR4C7PKvg/i3+SngrXlePwO8Nf+5fhp4K57XTwNvw78O4l/vtYHf4nldAh4M7PKf6zhwK3CM5/U6wG/zokP86xwH/gp4MM/rY4Cv5r/GRwNfxfO6FXgZYJcXDeJf57OBz+J5/Q7w2vzX+m3gtXhenwN8Ni8axIvupYG/4vl7GeCv+a/10sBf8fy9DPDX/MsQL7rfAl6b5/U5wGfz3+Ozgc/ief028Dr8yxAvmo8Gvorn9QzgpYFd/nscB/4aeBDP62OAr+aFQ/zLjgNPB47zvF4H+G3+e7028Fs8r13gIcAuLxjiX/ZTwFvzv9NPA2/DC4Z44V4b+C3+d3sd4Ld5/hAv2HHgr4AH87/brcDLALs8L8QL9tnAZ/F/w+cAn83zQjx/Lw38Ff+3vAzw1zwnxPP3YODB/N9yK3Arzwnx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGkiJpBy9b1IAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChangeHistory;
impl IconShape for MdChangeHistory {
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
                d: "M12 7.77L18.39 18H5.61L12 7.77M12 4L2 20h20L12 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheckCircle;
impl IconShape for MdCheckCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheckCircleOutline;
impl IconShape for MdCheckCircleOutline {
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
                d: "M16.59 7.58L10 14.17l-3.59-3.58L5 12l5 5 8-8zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXsw8CD+b3gGcCvPC/G8jgPfBbw1/7f8NPA+wC7PhnhOx4GnA8f5v2kXeAiwyxWI5/RTwFvzf9tPA2/DFYhnezDwdP5/eAhwK4B4ttcGfov/H14H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEfy7zvF4H+G0A8WyvDfwW//XEFS8NfBX/MT4G+GuuMM/rdYDfBhDP9trAb/FfT1zx2sBv8R/jdYDf5grzvF4H+G0A8WyvDfwW//XEFa8N/Bb/MV4H+G2uMM/rdYDfBhDP9trAb/FfT1zx2sBv8R/jdYDf5grzvF4H+G0A8WyvDfwW//XEFceBl+Y/xl8Du1xhntfrAL8NIJ7ttYHf4r+e+M9lntfrAL8NIJ7ttYHf4r+e+M9lntfrAL8NIJ7ttYHf4r+euOK1gd/iP8brAL/NFeZ5vQ7w2wDi2V4b+C3+64krXhv4Lf5jvA7w21xhntfrAL8NIJ7ttYHf4r+euOK1gd/iP8brAL/NFeZ5vQ7w2wDi2V4b+C3+64krXhv4Lf5jvA7w21xhntfrAL8NIJ7ttYHf4r+e+M9lntfrAL8NIJ7ttYHf4r+e+M9lntfrAL8NIJ7ttYHf4r+euOI48FL8x/gbYJcrzPN6HeC3AcSzvTbwW/zXE1e8NvBb/Md4HeC3ucI8r9cBfhtAPNtrA7/Ffz1xxWsDv8V/jNcBfpsrzPN6HeC3AcSzvTbwW/zXE1e8NvBb/Md4HeC3ucI8r9cBfhtAPNtrA7/Ffz1xxUsDX81/jI8G/porzPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXE/+5zPN6HeC3AcSzvTbwW/zXe23+c/02z+t1gN8GEM/22sBv8f/D6wC/DSCe0y5wjP/bLgHHuQLxnD4b+Cz+b/sc4LO5AvG8vht4L/5v+h7gvXk2xPP33sBHAy/F/w2/A3w38N08J8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ESiLnkGHbzOrAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChromeReaderMode;
impl IconShape for MdChromeReaderMode {
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
                d: "M13 12h7v1.5h-7zm0-2.5h7V11h-7zm0 5h7V16h-7zM21 4H3c-1.1 0-2 .9-2 2v13c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 15h-9V6h9v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCircleNotifications;
impl IconShape for MdCircleNotifications {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 16.5c-.83 0-1.5-.67-1.5-1.5h3c0 .83-.67 1.5-1.5 1.5zm5-2.5H7v-1l1-1v-2.61C8 9.27 9.03 7.47 11 7v-.5c0-.57.43-1 1-1s1 .43 1 1V7c1.97.47 3 2.28 3 4.39V14l1 1v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw/71uBf4a+B5gl38dxL/eewNfBRznf5Zd4GOA7+ZFh/jXeW3gt/if7XWA3+ZFg/jXeTrwYP5nuxV4CC8axIvupYG/4n+HlwH+mn8Z4kX32sBv8bx+h3+9lwaO8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8a/328Br8Z/ndYDf5l+GeNG9NvBbPC/xr/fbwGvxn+d1gN/mX4Z40b028Fs8L/Gv99vAa/Gf53WA3+ZfhnjRvTbwWzwv8bzeGvgs4KeBz+F5/TbwWvzneR3gt/mXIV50rw38Fs9LPNuDge8CXptnuxV4H+C3ebbfBl6L/zyvA/w2/zLEi+61gd/ieQk4DnwU8Nm8YD8NfAxwK/DbwGvxn+d1gN/mX4Z40b028Fs8r7cBvgp4MP+yXeCrgdcBXov/PK8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvupcG/or/HV4G+Gv+ZYh/nVuBB/E/2zOAB/OiQfzrvDbwW/zP9jrAb/OiQfzrvTfw1cAx/me5BLw38NO86BD/NseB9wZeGngw/71uBf4a+G5gl38dxP9viP/fEP+/If5/Q/z/xj8C4/CSQd+fOzsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClass;
impl IconShape for MdClass {
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
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/d7038NfAX/OCIf5vem/gu4Bd4HWAv+b5Q/zf897Ad/Fsu8DrAH/N80L83/LewHfxvHaB1wH+mueE+L/jvYHv4gV7H+C7eU6I/37HgV3+fd4b+C5esPcBvpvnhfjv91fA6wC7/Nu8N/BdvGDvA3w3zx/iv5+BvwZeB9jlX+e9ge/iBXsf4Lt5wRD//cwVfw28DrDLi+a9ge/iBXsf4Lt54RD//cyz/TXwOsAuL9x7A9/FC/Y+wHfzL0P89zPP6a+B1wF2ef7eG/guXrD3Ab6bFw3iv595Xn8NvA6wy3N6b+C7eMHeB/huXnSI/37m+ftr4HWAXa54b+C7eMHeB/hu/nUQ//3MC/bXwOsAbw18Fy/Y+wDfzb8e4r+feeFuBR7MC/Y+wHfzb4P472f+7d4H+G7+7RD//cy/zfsA382/D+K/n/nXex/gu/n3Q/z3M/86twIvA+zy74f472f+9f4aeB1gl38fxH8/82/z18DrALv82yH++5l/u78GXgfY5d8G8d/P/Pv8NfA6wC7/eoj/fubf76+B1wF2+ddB/Pcz/zH+GngdYJcXHeK/n/mP89fA6wC7vGgQ//3Mf6y/Bl4H2OVfhnjRHQd+G3gp/mOJ/z6If53jwG8DL8V/HPHfB/Gvdxz4beCl+I8h/vsg/m2OA78NvBT/fuK/D+Lf7jjw28BL8e8j/vsg/n2OA78NvBT/duK/D+Lf7zjw28BL8W8j/vsg/mMcB34beCn+9cR/H8R/nOPAXwMP4l9H/PdB/Md5b+C7+NcT/30Q/zHeG/gu/m3Efx/Ev997A9/Fv53474P493lv4Lt4wZ4BPIgXTvz3QfzbvTfwXbxg7wP8NPDbwEvxgon/Poh/m/cGvosX7H2A7+aK48BvAy/F8yf++yD+9d4b+C5esPcBvpvndBz4beCleF7ivw/iX+e9ge/iBXsf4Lt5/o4Dvw28FM9J/PdBvOjeG/guXrD3Ab6bF+448NvAS/Fs4r8P4kXz3sB38YK9D/DdvGiOA78NvBRXiP8+iH/ZewPfxQv2PsB3869zHPht4KUA8d8H8cK9N/BdvGDvA3w3/zbHgd8GXpr/PogX7L2B7+IFex/gu/n3OQ7s8t8H8fy9N/BdvGDvA3w3//shnr+XBn4bOMbzeh/gu/m/AfGCvTTw28Axnu19gO/m/w7EC/fSwG8Dx4D3Ab6b/1sQ/7KXBl4a+G7+70H8/4b4/w3x/xvi/zfE/2/8I2HXjEEQMLoGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloseFullscreen;
impl IconShape for MdCloseFullscreen {
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
                d: "M22,3.41l-5.29,5.29L20,12h-8V4l3.29,3.29L20.59,2L22,3.41z M3.41,22l5.29-5.29L12,20v-8H4l3.29,3.29L2,20.59L3.41,22z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEv0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+I/30sBXAW8D7PIf4zjwU8DHAH/NfxzEf6yXBn4LOA78NfA6wC7/PseB3wJeGtgFXgf4a/5jIP7jvDTwW8Bxnu2vgdcBdvm3OQ78FvDSPNsu8DrAX/Pvh/iP8dLAbwHHeV5/DbwM/zZ/Bbw0z2sXeB3gr/n3Qfz7vTTwW8Bxnr/3Ab6bf5v3Br6L528XeB3gr/m3Q/z7vDTwW8Bxnr/3Ab6bf5/3Br6L528XeB3gr/m3QfzbvTTwW8Bxnr/3Ab6b/xjvDXwXz98u8DrAX/Ovh/i3eWngt4DjPH/vA3w3/7HeG/gunr9d4HWAv+ZfB/Gv99LAbwHHef7eB/hu/nO8N/BdPH+7wOsAf82LDvGv89LAbwHHef7eB/hu/nO9N/BdPH+7wOsAf82LBvGie2ngt4DjPH/vA3w3/zXeG/gunr9d4HWAv+ZfhnjRvDTwW8Bxnr/3Ab6b/1rvDXwXz98u8DrAX/PCIf5lLw38FnCc5+99gO/mv8d7A9/F87cLvA7w17xgiBfupYHfAo7z/L0P8N3893pv4Lt4/naB1wH+mucP8YK9NPBbwHGev/cBvpv/Gd4b+C6ev13gdYC/5nkhnr+XBn4LOM7z9z7Ad/M/y3sD38Xztwu8DvDXPCfE8/fSwG8Dx3j+3gf4bv5neW/gu3j+LgGvDfw1zwnxgr008NvAMZ6/9wG+m/8Z3hv4Lp6/S8BrA3/N80K8cC8N/DZwjOfvfYDv5r/XewPfxfN3CXht4K95/hD/spcGfhs4xvP3PsB389/jvYHv4vm7BLw28Ne8YIgXzUsDvw0c4/l7H+C7+a/13sB38fxdAl4b+GteOMSL7qWB3waO8fy9D/Dd/Nd4b+C7eP4uAa8N/DX/MsS/zksDvw0c4/l7H+C7+c/13sB38fxdAl4b+GteNIh/vZcGfhs4xvP3PsB385/jvYHv4vm7BLw28Ne86BD/Ni8N/DZwjOfvfYDv5j/WewPfxfN3CXht4K/510H827008NvAMZ6/9wG+m/8Y7w18F8/fJeC1gb/mXw/x7/PSwG8Dx3j+3gf4bv593hv4Lp6/S8BrA3/Nvw3i3++lgd8GjvH8vQ/w3fzbvDfwXTx/l4DXBv6afzvEf4yXBn4bOMbz+hvgpfm3+WvgpXhel4DXBv6afx/Ef5yXBn4bOMaz/Q3w2sAu/zbHgd8GXopnuwS8NvDX/Psh/mO9NPDbwDHgb4DXBnb59zkO/DbwUsAl4LWBv+Y/BuI/3ksDXw28NbDLf4zjwE8DHw38Nf9xEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzOVokFHB5nKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCode;
impl IconShape for MdCode {
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
                d: "M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0l4.6-4.6-4.6-4.6L16 6l6 6-6 6-1.4-1.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5WeAt+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/z4fA/w1/zovDXwVL9jvAK/Nc0I8f78NvBb/fV4H+G3+dV4b+C1esN8BXpvnhHj+fht4Lf77vA7w2/zrvDbwW7xgvwO8Ns8J8fz9NvBa/Pd5HeC3+dd5beC3eMF+B3htnhPi+ftt4LX47/M6wG/zr/PawG/xgv0O8No8J8Tz99vAa/Hf53WA3+Zf57WB3+IF+x3gtXlOiOfvt4HX4r/P6wC/zb/OawO/xQv2O8Br85wQz99vA6/Ff5/XAX6bf53XBn6LF+x3gNfmOSGev98GXov/Pq8D/Db/Oq8N/BYv2O8Ar81zQjx/vw28Fv99Xgf4bf51Xhv4LV6w3wFem+eEeP5+G3gt/vu8DvDb/Ou8NvBbvGC/A7w2zwnx/P028Fr893kd4Lf513lt4Ld4wX4HeG2eE+L5+23gtfjv8zrAb/Ov89rAb/GC/Q7w2jwnxPP328Br8d/ndYDf5l/ntYHf4gX7HeC1eU6I5++3gdfiv8/rAL/Nv85rA7/FC/Y7wGvznBDP328Dr8V/n9cBfpt/ndcGfosX7HeA1+Y5IZ6/3wZei/8+rwP8Nv86rw38Fi/Y7wCvzXNCPH+/DbwW/31eB/htntdxrtjleb028Fu8YL8DvDbPCfH8/TbwWvz3eR3gt3lOHwV8Nld8NvA1PKfXBn6LF+x3gNfmOSGev98GXov/Pq8D/DZXvDbwVcBL85z+GvgY4Le54rWB3+IF+x3gtXlOiOfvt4HX4r/P6wB/DXwV8N68cN8NfAzw0sBv8YL9DvDaPCfE8/fbwGvx3+e7gbcGjvOi2QV+GnhvXrDfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+Rzgs3lOiOfvt4HX4v+WtwF+mueEeP5+G3gt/u/4G+CleV6I5++3gdfi/4a/Ad4auJXnhXj+fht4Lf53ewbw3cBXA7s8f4jn77eB1+Jf9j7Ad/O/F+L5+23gtXjh3gf4bv53Qzx/vw28Fi/Y+wDfzf9+iOfvt4HX4vl7H+C7+b8B8fz9NvBaPK/3Ab6b/zsQz99vA6/Fc3of4Lv5vwXx/P028Fo82/sA383/PYjn77eB1+KK9wG+m/+bEM/fbwOvBbwP8N3834V4/n4b+G7gu/m/DfH8vTTw1/zfh/j/DfH/G+L/N8T/b4j/3/hH+XKlQY/qu4AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCommentBank;
impl IconShape for MdCommentBank {
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
                d: "M20,2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M19,13l-2.5-1.5L14,13V5h5V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCommute;
impl IconShape for MdCommute {
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
                d: "M12 4H5C3.34 4 2 5.34 2 7v8c0 1.66 1.34 3 3 3l-1 1v1h1l2-2.03L9 18v-5H4V5.98L13 6v2h2V7c0-1.66-1.34-3-3-3zM5 14c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm15.57-4.34c-.14-.4-.52-.66-.97-.66h-7.19c-.46 0-.83.26-.98.66L10 13.77l.01 5.51c0 .38.31.72.69.72h.62c.38 0 .68-.38.68-.76V18h8v1.24c0 .38.31.76.69.76h.61c.38 0 .69-.34.69-.72l.01-1.37v-4.14l-1.43-4.11zm-8.16.34h7.19l1.03 3h-9.25l1.03-3zM12 16c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm8 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7O9NPDX/OdB/M/10sBvASf4z4P4n+mlgd8CjgPiPw/if56XBn4LOM4V4j8P4n+WlwZ+CzjOs4n/PIj/OV4a+C3gOM9J/OdB/M/w0sBvAcd5XuI/D+K/30sDvwUc5/kT/3kQ/71eGvgt4Dgv2G/zH+NvgI/mOSH++7w08FvAcf5r/A7w2jwnxH+PlwZ+CzjOf53fAV6b54T47/HSwG8Dx/iv8zvAa/OcEP99Xhr4beAY/zV+B3htnhPiv9dLA78NHOMF+x3+Y/w18NE8J8R/v5cGfhs4xvMn/vMg/md4aeC3gWM8L/GfB/Gf76WBv+Zf9tLAbwPHeE7iPw/iP99F4HWAv+Zf9tLAbwPHeDbxnwfxn8/ALvA6wF/zL3tp4LeBY1wh/vMg/vOZK3aB1wH+mn/ZSwO/DRwDxH8exH8+82y7wOsAf82/7KWB3waO858H8Z/PPKdd4HWAv+Zf9tLAX/OfB/GfzzyvXeB1gL/mvxfiP595/naB1wH+mv8+iOfvq4GX4j/Ga/OC7QKvA/w1/z0Qz99vA6/Ff41d4HWAv+a/HuL5+23gtfivswu8DvDX/NdCPH+/DbwW/3UuAa8N/DX/tRDP328Dr8V/jUvAawN/zX89xPP31cBL8x/jtXjBLgGvDfw1/z0Q//nM83cJeG3gr/nvg/jPZ57XJeC1gb/mvxfiP595TpeA1wb+mv9+iP985tkuAa8N/DX/MyD+85krLgGvDfw1/3Mg/vMZuAS8NvDX/M+C+M+3C7w28Nf8z4P4z/fSwF/zPxPi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8B6z1aQTwh8EsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCompareArrows;
impl IconShape for MdCompareArrows {
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
                d: "M9.01,14H2v2h7.01v3L13,15l-3.99-4V14z M14.99,13v-3H22V8h-7.01V5L11,9L14.99,13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD5klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx3hOl4C/5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/HA8GHsQL9tXAS/Oc/hr4aF6wZwC38h8L8Z/jOPDbwEvxH+NvgNcGdvmPhfjPcxz4beCl+Pf5G+C1gV3+4yH+cx0Hfht4Kf5t/gZ4bWCX/xyI/3zHgd8GXop/nb8BXhvY5T8P4r/GceC3gZfiRfM3wGsDu/znQvzXOQ78NvBSvHB/A7w2sMt/PsR/rePAbwMvxfP3N8BrA7v810D81zsO/DbwUjynvwFeG9jlvw7iv8dx4LeBl+KKvwFeG9jlvxbiv89x4Le54rWBXf7rIf57HeeKXf57IP5/Q/z/hnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvuOPDS/O/w18Au/zLE/2+I/98QL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40R0HXpr/Hf4a2OVfhvj/DfH/G+K/13dxxfvw3wPx3+e7gPfmiu8G3of/eoj/Ht8FvDfP6buB9+G/FuK/3ncB783z993A+/BfB/Ff67uA9+aF+27gffivgfiv813Ae/Oi+W7gffjPh/iv8V3Ae/Ov893A+/CfC/Gf77uA9+bf5ruB9+E/D+I/13cB782/z3cD78N/DsR/nu8C3pv/GN8NvA//8RD/Od4beG9esJcGjvGcLgF/zQv23cB38x8L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j68GXprn9NfAR/NfC/H/G+L/N8T/b4j/3xD/v/GPPjRxQW0rxz8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCompress;
impl IconShape for MdCompress {
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
                d: "M8 19h3v3h2v-3h3l-4-4-4 4zm8-15h-3V1h-2v3H8l4 4 4-4zM4 9v2h16V9H4z",
            }
            path {
                d: "M4 12h16v2H4z",
            }
            path {
                d: "M0 0h24v24H0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/Pb4beB/+4yFedK8N/Bb/fb4beB/+YyFedK8N/Bb/vb4beB/+4yBedK8N/Bb//b4beB/+YyBedK8N/Bb/M3w38D78+yFedK8N/Bb/c3w38D78+yBedK8N/Bb/s3w38D782yFedK8N/Bb/83w38D782yBedK8N/Bb/M3038D786yFedK8N/Bb/c3038D786yBedK8N/Bb/s3038D686BAvutcGfov/+b4beB9eNIgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf7jPAO4lSseDDyI/zivA/w2/zLEi+61gd/i3+8S8N7AT/Oc3hr4buAY/36vA/w2/zLEi+61gd/i3+cS8GBgl+fvOHArcIx/n9cBfpt/GeJF99rAb/Hv8zbAT/PCvTXwU/z7vA7w2/zLEC+61wZ+i3+7ZwAP5kVzK/Ag/u1eB/ht/mWIF91rA7/Fv93vAK/Ni+a3gdfi3+51gN/mX4Z40b028Fv82/0O8Nq8aH4beC3+7V4H+G3+ZYgX3WsDv8W/3a3AQ3jRPB14MP92rwP8Nv8yxIvutYHf4t/nbYCf5oV7a+Cn+Pd5HeC3+ZchXnSvDfwW/z67wEOAXZ6/lwZ+CzjOv8/rAL/Nvwzxontt4Lf499sF3gf4aZ7TWwPfBRzn3+91gN/mX4Z40b028Fv8x7kVuJUrHgw8mP84rwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+Lf7G+C3gd8GdoHf5jm9NnAceG3gtYGX4t/udYDf5l+GeNG9NvBb/Ot9D/DVwF/zr/PSwEcD78W/3usAv82/DPGie23gt3jRXQLeGvht/n1eG/hp4BgvutcBfpt/GeJF99rAb/Giex3gt/mP8drAb/Giex3gt/mXIV50rw38Fi+a3wFem/9Yvw28Fi+a1wF+m38Z4kX32sBv8aL5HOCz+Y/12cBn8aJ5HeC3+ZchXnSvDfwWL5q3AX6a/1hvDfwUL5rXAX6bfxniRffawG/xonkd4Lf5j/XawG/xonkd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTTwV/zv8DLAX/MvQ/zr3Ao8iP/ZngE8mBcN4l/ntYHf4n+21wF+mxcN4l/vvYGvBo7xP8sl4L2Bn+ZFh/i3OQ68N/DSwIP573Ur8NfAdwO7/Osg/n9D/P+G+P8N8f8b4v83/hFk0aBBZ8f+owAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContactPage;
impl IconShape for MdContactPage {
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
                d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8L14,2z M12,10c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2s-2-0.9-2-2 C10,10.9,10.9,10,12,10z M16,18H8v-0.57c0-0.81,0.48-1.53,1.22-1.85C10.07,15.21,11.01,15,12,15c0.99,0,1.93,0.21,2.78,0.58 C15.52,15.9,16,16.62,16,17.43V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContactSupport;
impl IconShape for MdContactSupport {
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
                d: "M11.5 2C6.81 2 3 5.81 3 10.5S6.81 19 11.5 19h.5v3c4.86-2.34 8-7 8-11.5C20 5.81 16.19 2 11.5 2zm1 14.5h-2v-2h2v2zm0-3.5h-2c0-3.25 3-3 3-5 0-1.1-.9-2-2-2s-2 .9-2 2h-2c0-2.21 1.79-4 4-4s4 1.79 4 4c0 2.5-3 2.75-3 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContactless;
impl IconShape for MdContactless {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M8.46,14.45L7.1,13.83 c0.28-0.61,0.41-1.24,0.4-1.86c-0.01-0.63-0.14-1.24-0.4-1.8l1.36-0.63c0.35,0.75,0.53,1.56,0.54,2.4 C9.01,12.8,8.83,13.64,8.46,14.45z M11.53,16.01l-1.3-0.74c0.52-0.92,0.78-1.98,0.78-3.15c0-1.19-0.27-2.33-0.8-3.4l1.34-0.67 c0.64,1.28,0.96,2.65,0.96,4.07C12.51,13.55,12.18,14.86,11.53,16.01z M14.67,17.33l-1.35-0.66c0.78-1.6,1.18-3.18,1.18-4.69 c0-1.51-0.4-3.07-1.18-4.64l1.34-0.67C15.56,8.45,16,10.23,16,11.98C16,13.72,15.56,15.52,14.67,17.33z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCopyright;
impl IconShape for MdCopyright {
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
                d: "M11.88,9.14c1.28,0.06,1.61,1.15,1.63,1.66h1.79c-0.08-1.98-1.49-3.19-3.45-3.19C9.64,7.61,8,9,8,12.14 c0,1.94,0.93,4.24,3.84,4.24c2.22,0,3.41-1.65,3.44-2.95h-1.79c-0.03,0.59-0.45,1.38-1.63,1.44C10.55,14.83,10,13.81,10,12.14 C10,9.25,11.28,9.16,11.88,9.14z M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8 s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe2vgq4AH87/TrcDHAD/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5GuCjeU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4TrvAX/O/w0sDx3lOvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/H038F783/I1wEfznBDP30cDX8X/LR8DfDXPCfH8HQduBY7xf8Ml4MHALs8J8YK9N/Bd/N/wPsB387wQL9xbA18NPIj/nZ4BfDTw0zx/iBfNSwPH+d9lF/hrXjjE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EdeLohBa44cLQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCreditCard;
impl IconShape for MdCreditCard {
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
                d: "M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDangerous;
impl IconShape for MdDangerous {
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
                d: "M15.73 3H8.27L3 8.27v7.46L8.27 21h7.46L21 15.73V8.27L15.73 3zM17 15.74L15.74 17 12 13.26 8.26 17 7 15.74 10.74 12 7 8.26 8.26 7 12 10.74 15.74 7 17 8.26 13.26 12 17 15.74z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Nv/9EP8y8x/vdYDf5r8f4l9m/uO9DvDb/PdD/MvMf7zXAX6b/36If5n5j/c6wG/z3w/xLzP/8V4H+G2e00sDX8V/vNfhBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv+y3+Y/30cBf85xeG/gt/uOJFwzxP8drA7/FfzzxgiH+53ht4Lf4jydeMMT/HK8N/Bb/8cQLhvif47WB3+I/nnjBEP9zvDbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Md7HeC3eU6vDfwW//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjZuoFB5J04ogAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDashboard;
impl IconShape for MdDashboard {
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
                d: "M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S97bf7j/TWwy3M6Drw0//F+mxcM8f8b4v83xP9viP/fEP+y1+I/3t8Au/zrHAdeiuf1O/zbIf5l5j/e6wC/zb/OawO/xfMS/3aIf5n5j/c6wG/zr/PawG/xvMS/HeJfZv7jvQ7w2/zrvDbwWzwv8W+H+JeZ/3ivA/w2/zqvDfwWz0v82yH+ZeY/3usAv82/zmsDv8XzEv92iH+Z+Y/3OsBv86/z2sBv8bzEvx3iX2b+470O8Nv867w28Fs8L/Fvh/iXmf94rwP8Ns/pOPBSvGAvDXw1z+u1eeF+hxcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Md7HeC3eU6vDfwW//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Nv867w28Fs8L/Fvh/iXmf94rwP8Nv86rw38Fs9L/Nsh/mXmP97rAL/Nv85rA7/F8xL/doh/mfmP9zrAb/Ov89rAb/G8xL8d4l9m/uO9DvDb/Ou8NvBbPC/xb4f4l5n/eK8D/Db/Oq8N/BbPS/zbIf5l5j/e6wC/zb/OawO/xfMS/3aIf5n5j/c6wG/zr/PawG/xvMS/HeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CHgOhUFA6EUQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDashboardCustomize;
impl IconShape for MdDashboardCustomize {
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
                d: "M3 3h8v8H3zm10 0h8v8h-8zM3 13h8v8H3zm15 0h-2v3h-3v2h3v3h2v-3h3v-2h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/F8/oY4K/5j/XSwFfxvF6HfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv95r8Zx+m+f10cBf8x/rpYGv5nm9Ns/pd3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lvw28Fv87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDbPKeXBr6K5/U6PK+vBl6K5/Q3wEfzvH6L5/UxwF/znF4b+C2el3jBEP8y87xeB/htntNrA7/F8xLP67eB1+I5/Q7w2jwv87xeB/htntNrA7/F8xIvGOJfZp7X6wC/zXN6beC3eF7ief028Fo8p98BXpvnZZ7X6wC/zXN6beC3eF7iBUP8y8zzeh3gt3lOrw38Fs9LPK/fBl6L5/Q7wGvzvMzzeh3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5ief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5iRcM8S8zz+t1gN/mOb008NU8r9fmeX018NI8p78GPprn9ds8r48G/prn9NrAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlefw3s8j/LceCleV7iBUP8y24FHsT/Ts8AHswLhviXfTbwWfzv9DnAZ/OCIf5lx4G/Bh7E/y5/A7w2sMsLhnjRPBj4aeCl+N/hb4C3Bm7lhUP867w18NLAa/M/028Dfw38NC8axP9viP/fEP+/If5/Q/z/xj8CVCSqQYHvdMMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDateRange;
impl IconShape for MdDateRange {
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
                d: "M9 11H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2zm2-7h-1V2h-2v2H8V2H6v2H5c-1.11 0-1.99.9-1.99 2L3 20c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H5V9h14v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fSwG8Bx/nX2QVeB/hr/vMg/nO9NPBbwHH+bXaB1wH+mv8ciH+blwaO8cIdB74bOM6/zy7w3sAuL9wl4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI/98Q/78h/v1ei/9ev8O/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfbxc4xn+PS8Bx/u0Q/37fDbwX/z2+B3hv/u0Q/34PBp7Of4+HALfyb4f4j/HewHfxX+t9gO/m3wfxH+fBwGcDbw0c4z/HJeCngc8GbuXfD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BCx7QkHi8DXpAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDelete;
impl IconShape for MdDelete {
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
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fSwG8Bx/nX2QVeB/hr/vMg/nO9NPBbwHH+bXaB1wH+mv8ciH+blwaO8cIdB74bOM6/zy7w3sAuL9wl4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI/98Q/78h/v1ei/9ev8O/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/8y/7HuCrgd8GjvHCXQLeGvhq4KX4l4l/O8S/n3nhvgd4b654aeC3gWM8f5eA1wb+GjgO/DbwUrxw4t8O8e9nXrDvAd6b5/TSwG8Dx3hOl4DXBv6aZzsO/DbwUrxg4t8O8e9nnr/fAV6b5++lgd8GjnHFJeC1gb/meR0H/hp4EM+f+LdD/PuZF+x9gO/m+Xtp4Le54rWBv+b5e2/gu3jBxL8d4t/PvHDvA3w3z99Lc8Vf8/y9N/BdvHDi3w7x72f+Ze8DfDf/Ou8NfBf/MvFvh/j3My+a9wG+mxfNewPfxYtG/Nsh/v3Mi+59gO/mhXtv4Lt40Yl/O8S/n3nRXQIeDOzy/B0HbgWO8aIT/3aIfz/zorkEvDbw17xwLw38NnCMF434t0P8+5l/2SXgtYG/5kXz0sBvA8f4l4l/O8S/n3nhLgGvDfw1/zovDfw2cIwXTvzbIf79zAt2CXht4K95/t6bK76b5++lgd8GjvGCiX87xL+fef4uAa8N/DXP33sD38UV7wN8N8/fSwO/DRzj+RP/doh/P/P87QKvA/w1z+u9ge/iOb0P8N08r5cGfgs4zvMn/u0Q/37mBdsFXgf4a57tvYHv4vl7H+C7ebaXBn4LOM4LJv7tEP9+5oXbBV4H+GvgvYHv4oV7H+C7gZcGfgs4zgsn/u0Q/37mX7YLfDXw2bxoPhv4aOA4/zLxb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f499sFjvHf4xJwnH87xL/fdwPvxX+P7wHem387xL/fg4Gn89/jIcCt/Nsh/mO8N/Bd/Nd6H+C7+fdB/Md5MPDZwFsDx/jPcQn4aeCzgVv590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwF2QIlBVZhPZwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeleteForever;
impl IconShape for MdDeleteForever {
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
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zm2.46-7.12l1.41-1.41L12 12.59l2.12-2.12 1.41 1.41L13.41 14l2.12 2.12-1.41 1.41L12 15.41l-2.12 2.12-1.41-1.41L10.59 14l-2.13-2.12zM15.5 4l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fSwG8Bx/nX2QVeB/hr/vMg/nO9NPBbwHH+bXaB1wH+mv8ciH+blwaO8cIdB74bOM6/zy7w3sAuL9wl4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI/98Q/78h/v1ei/9ev8O/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87xeB/ht/mO9NvBbPC/xb4f49zPP63WA3+Y/1msDv8XzEv92iH8/87w+Gvhr/mO9NPDVPC/xb4f499sFjvHf4xJwnH87xL/fdwPvxX+P7wHem387xL/fg4Gn89/jIcCt/Nsh/mO8N/Bd/Nd6H+C7+fdB/Md5MPDZwFsDx/jPcQn4aeCzgVv590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFDD3hBXArkMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeleteOutline;
impl IconShape for MdDeleteOutline {
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
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM8 9h8v10H8V9zm7.5-5l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/eZ4BHAeO8by+G3gf/uMhXnSvDfwW/3l+B/ho4LeBYzyv7wbeh/9YiBfdawO/xX+e3wFeG3hp4LeBYzyv7wbeh/84iBfdawO/xX+e3wFemyteGvht4BjP67uB9+E/BuJF99rAb/Gf53eA1+bZXhr4beAYz+u7gffh3w/xontt4Lf4z/M7wGvznF4a+G3gGM/ru4H34d8H8aJ7beC3+M/zO8Br87xeGvht4BjP67uB9+HfDvGie23gt/jP8zvAa/P8vTTw28Axntd3A+/Dvw3iRffawG/xn+d3gNfmBXtp4LeBYzyv7wbeh389xIvutYHf4j/P7wCvzQv30sBvA8d4Xt8NvA//OogX3WsDv8V/nt8BXpt/2UsDvw0c43l9N/A+vOgQL7rXBn6L/zy/A7w2L5qXBn4bOMbz+m7gfXjRIF50rw38Fv95fgd4bV50Lw38NnCM5/U6wG/zL0O86F4b+C3+8/wO8Nr867w08NvAMZ7T6wC/zb8M8aJ7beC3+M/zO8Br86/30sBvA8d4ttcBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8aJ5HeC3+Y/12sBv8aJ5HeC3+ZchXnSvDfwWL5rXAX6b/1ivDfwWL5rXAX6bfxniRffawG/xonkd4Lf5j/XawG/xonkd4Lf5lyFedK8N/BYvmtcBfpv/WK8N/BYvmtcBfpt/GeJF99rAb/GieR3gt/mP9drAb/GieR3gt/mXIV50rw38Fi+avwZ2+Y91HHhpXjSvA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C1eNH8D7PIf6zjwUrxoXgf4bf5liBfdawO/xYvmdYDf5j/WawO/xYvmdYDf5l+GeNG9NvBbvGheB/ht/mO9NvBbvGheB/ht/mWIF91rA7/Fi+Z1gN/mP9ZrA7/Fi+Z1gN/mX4Z40b028Fu8aF4H+G3+Y7028Fu8aF4H+G3+ZYgX3WsDv8WL5nWA3+Y/1msDv8WL5nWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX30sBf8b/DywB/zb8M8a9zK/Ag/md7BvBgXjSIf523Bn6K/9leB/htXjSIf733Br4aOMb/LJeA9wZ+mhcd4t/mOPDewEsDD+a/163AXwPfDezyr4P4/w3x/xvi/zfE/2+I/9/4R8C2pEGhYgp9AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDescription;
impl IconShape for MdDescription {
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEU0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmX+bZwAP4j/GM4AH8W8jXjDEv8z8630P8N7AdwPvxb/P9wDvDXw38F7864kXDPEvM/863wO8N8/23cB78W/zPcB782zfDbwX/zriBUP8y8yL7nuA9+Z5fTfwXvzrfA/w3jyv7wbeixedeMEQ/zLzorkVeAgv2HcD78WL5nuA9+YFezrwYF404gVD/MvMi+67gffhBftu4L144b4HeG9esO8C3psXnXjBEP8y86/z3cD78IJ9N/BePH/fA7w3L9h3Ae/Nv454wRD/MvOv993A+/CCfTfwXjyn7wHemxfsu4D35l9PvGCIf5n5t/lu4H14wb4beC+u+B7gvXnBvgt4b/5txAuG+JeZf7vvBt6HF+y7ueK9ecG+C3hv/u3EC4b4l5l/n+8G3od/m+8C3pt/H/GCIf5l5t/vu4H34V/nu4D35t9PvGCIf5n5j/HdwPvwovku4L35jyFeMMS/zPzH+W7gfXjhvgt4b/7jiBcM8S8z/3G+B3hvXrjvBt6L/zjiBUP8y8x/jO8B3psXzXcD78V/DPGCIf5l5t/ve4D35l/nu4H34t9PvGCIf5n59/ke4L35t/lu4L349xEvGOJfZv7tvgd4b16w7+KK9+EF+27gvfi3Ey8Y4l9m/m2+B3hvXrDvAt6bK74beB9esO8G3ot/G/GCIf5l5l/ve4D35gX7LuC9eU7fDbwPL9h3A+/Fv554wRD/MvOv8z3Ae/OCfRfw3jx/3w28Dy/YdwPvxb+OeMEQ/zLzovse4L15wb4LeG9euO8G3ocX7LuB9+JFJ14wxL/MvGhuBR7CC/ZdwHvzovlu4H14wZ4OPJgXjXjBEP8y86L7buB9eF7fBbw3/zrfDbwPz+u7gPfmRSdeMMS/zPzrfDfwPjzbdwHvzb/NdwPvw7N9F/De/OuIFwzxLzP/et8NvA/wXcB78+/z3cD7AN8FvDf/euIFQ/zLzL/NrcCD+Y9xK/Bg/m3EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AuHWikH5cbfXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDisabledByDefault;
impl IconShape for MdDisabledByDefault {
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
                d: "M3,3v18h18V3H3z M17,15.59L15.59,17L12,13.41L8.41,17L7,15.59L10.59,12L7,8.41L8.41,7L12,10.59L15.59,7L17,8.41L13.41,12 L17,15.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADoElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b/53+W7gY4BdXjDEv+y7gffif6fvAd6bFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzMv3N8APw38NfDSwEsDb8X/HOIFQ/zLzAv2McBX87xeGvhp4EH89xMvGOJfZp6/jwG+mhfspYHfBo7x30u8YIh/mXlefwO8NP+yzwY+i/9e4gVD/MvM8/oc4LP5lz0YeDr/vcQLhviXmef1NsBP86Ix/73EC4b4l5nn9TnAZ/MvOw5c5L+XeMEQ/zLzvH4GeGv+Ze8NfBf/vcQLhviXmefvZYC/5oV7OvBg/nuJFwzxLzPP363A2wB/zfP3XcB7899PvGCIf5l5wXaBrwa+B7gVOA68FfDZwIP5n0G8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/2V8DL8X/Tn8DvDQvGOJf9tLAbwPH+N/lEvDawF/zgiFeNMeBl+Z/l78GdnnhEP+/If5/Q/z/hvj/DfGieTDwIP53+RtglxcO8S97aeC3gOP877ILvA7w17xgiH/ZXwEvzf9Ofw28DC8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9mXrBLwFcD3w3cChwH3hr4bOBB/M8gXjDEv8w8f88AXhu4lefvu4H34r+feMEQ/zLz/L0M8Ne8cLcCD+K/l3jBEP8y87x+Bnhr/mXvDXwX/73EC4b4l5nn9TnAZ/MvOw5c5L+XeMEQ/zLzvN4G+GleNOa/l3jBEP8y87w+B/hs/mUPBp7Ofy/xgiH+ZeZ5/TXwMvzLPhv4LP57iRcM8S8zz9/HAF/NC/bSwG8Bx/nvJV4wxL/MvGAfDXwNz+ulgZ8CHsx/P/GCIf5l5oX7a+Cngb8GXhp4aeCt+Z9DvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9l3A+/F/07fA7w3LxjiX3Yc+Grgvfjf5XuAjwZ2ecEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EfLLm5B67PXAQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDns;
impl IconShape for MdDns {
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
                d: "M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zM7 19c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM20 3H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM7 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/N/y3sBHAa8D7PIvQ/zf8d7Ad3HFXwOvA+zywiH+b3hv4Lt4Tn8NvA6wywuG+N/vvYHv4vn7HeC1ecEQ/7u9N/BdPH+XgNcG/poXDPG/13sD38Xzdwl4beCveeEQ/zu9N/BdPH+XgNcG/pp/GeJ/n/cGvovn7xLw2sBf86JB/O/y3sB38fxdAl4b+GtedIj/Pd4b+C6ev0vAawN/zb8O4n+H9wa+i+fvEvDawF/zr4f4n++9ge/i+bsEvDbw1/zbIP5ne2/gu3j+LgGvDfw1/3aI/7neG/gunr9LwGsDf82/D+J/pvcGvovn7xLw2sBf8++H+J/nvYHv4vm7BLw28Nf8x0D8z/LewHfx/F0CXhv4a/7jIP71vgv4HeC7+Y/13sB38fxdAl4b+Gv+YyH+db4LeG+ueB/gu/mP8d7Ad/H8XQJeG/hr/uMhXnTfBbw3z+l9gO/m3+e9ge/i+bsEvDbw1/znQLxovhr4KJ6/9wG+m3+b9wa+i+fvEvDawF/znwfxonlp4LeBYzx/7wN8N/867w18F8/fJeC1gb/mPxfiRffSwG8Dx3j+3gf4bl407w18F8/fJeC1gb/mPx/iX+elgd8GjvH8vQ/w3bxw7w18F8/fJeC1gb/mvwbiX++lgd8GjvH8vQ/w3Tx/7w18F8/fJeC1gb/mvw7i3+algd8GjvH8vQ/w3Tyn9wa+i+fvEvDawF/zXwvxb/fSwG8Dx3j+3gf4bq54b+C7eP4uAa8N/DX/9RD/Pi8N/DZwjOfvfbjiu3j+LgGvDfw1/z0Q/34vDfw2cIx/nUvAawN/zX8fxH+MlwZ+GzjGi+YS8NrAX/PfC/Ef56WB3waO8cJdAl4b+Gv++yH+Y7008NvAMZ6/S8BrA3/N/wyI/3gvDfw2cIzndAl4beCv+Z8D8Z/jpYHfBo5xxSXgtYG/5n8WxH+elwZ+myteG/hr/udB/Od6aa74a/5nQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I7gqfEFHeQXEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDone;
impl IconShape for MdDone {
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
                d: "M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/O/y3sBHAa8D7PKv897ARwGvA+xyBeJ/j/cGvosr/hp4HWCXF817A9/FFX8NvA6wCyD+d3hv4Lt4Tn8NvA6wywv33sB38Zz+GngdYFf8z/fewHfx/P0O8Nq8YO8NfBfP318DLyP+Z3tv4Lt4/i4Brw38Nc/fewPfxQv2PsB3i/+53hv4Lp6/S8BrA3/N8/fewHfxgr0P8N0A4n+m9wa+i+fvEvDawF/z/L038F28YO8DfDdXIP7neW/gu3j+LgGvDfw1z997A9/FC/Y+wHfzbIj/Wd4b+C6ev0vAawN/zfP33sB38YK9D/DdPCfE/xzvDXwXz98l4LWBv+b5e2/gu3jB3gf4bp4X4n+G9wa+i+fvEvDawF/z/L038F28YO8DfDfPH+K/33sD38Xzdwl4beCvef7eG/guXrD3Ab6bFwzx3+u9ge/i+bsEvDbw1zx/7w18Fy/Y+wDfzQuH+O/z3sB38fxdAl4b+Guev/cGvosX7H2A7+Zfhvjv8d7Ad/H8XQJeG/hrnr/3Br6LF+x9gO/mRYP4r/fewHfx/F0CXhv4a56/9wa+ixfsfYDv5kWH+K/11sBP8fxdAl4b+Guev/cGvosX7H2A7+ZfB/G8vgv4HeC7+df7LuB3gO/m+TsO/DbwUjynS8BrA3/N8/fewHfxgr0P8N386yGe03cB780V7wN8Ny+67wLemyveB/hunr/jwG8DL8UVl4DXBv6a5++9ge/iBXsf4Lv5t0E823cB781zeh/gu/mXfRfw3jyn9wG+m+fvOPDbwIOB1wb+mufvvYHv4gV7H+C7+bdDXPHVwEfx/L0P8N28YF8NfBTP3/sA383zdxx4MPDXPH/vDXwXL9j7AN/Nvw/iipcGfhs4xvP3PsB38/y9NPDbwDGev/cBvpt/nfcGvosX7H2A7+bfD/FsLw38NnCM5+99gO/m+Xtp4LeBYzx/7wN8Ny+a9wa+ixfsfYDv5j8G4jm9NPDbwDGev/cBvpvn76WB3waO8fy9D/DdvHDvDXwXL9j7AN/NfxzE83pp4LeBYzx/7wN8N8/fSwO/DRzj+Xsf4Lt5/t4b+C5esPcBvpv/WIjn76WB3waO8fy9D/DdPH8vDfw2cIzn732A7+Y5vTfwXbxg7wN8N//xEC/YSwO/DRzj+Xsf4Lt5/l4a+G3gGM/f+wDfzRXvDXwXL9j7AN/Nfw7EC/fSwG8Dx3j+3gf4bp6/lwZ+GzjG8/c+XPFdvGDvA3w3/3kQ/7KXBn4bOMbz9z7Ad/P8vTTw28Ax/vXeB/hu/nMhXjQvDfw2cIzn732A7+b5e2ngt4FjvOjeB/hu/vMhXnQvDfw2cIzn732A7+b5e2ngt4Fj/MveB/hu/msg/nVeGvht4BjP3/sA383z99LAbwPHeMHeB/hu/usg/vVeGvht4BjP6RLw2sBf84K9NPDbwDGe1/sA381/LcS/zUsDvw0c44pLwGsDf82/7KWB3waO8WzvA3w3//UQ/3YvDfw2V7w28Ne86F4a+G3gGPA+wHfz3wPx7/PSXPHX/Ou9NPDSwHfz3wfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+7v79B639SDQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoneAll;
impl IconShape for MdDoneAll {
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
                d: "M18 7l-1.41-1.41-6.34 6.34 1.41 1.41L18 7zm4.24-1.41L11.66 16.17 7.48 12l-1.41 1.41L11.66 19l12-12-1.42-1.41zM.41 13.41L6 19l1.41-1.41L1.83 12 .41 13.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoneOutline;
impl IconShape for MdDoneOutline {
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
                d: "M19.77 5.03l1.4 1.4L8.43 19.17l-5.6-5.6 1.4-1.4 4.2 4.2L19.77 5.03m0-2.83L8.43 13.54l-4.2-4.2L0 13.57 8.43 22 24 6.43 19.77 2.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDonutLarge;
impl IconShape for MdDonutLarge {
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
                d: "M11,5.08V2C6,2.5,2,6.81,2,12s4,9.5,9,10v-3.08c-3-0.48-6-3.4-6-6.92S8,5.56,11,5.08z M18.97,11H22c-0.47-5-4-8.53-9-9 v3.08C16,5.51,18.54,8,18.97,11z M13,18.92V22c5-0.47,8.53-4,9-9h-3.03C18.54,16,16,18.49,13,18.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDonutSmall;
impl IconShape for MdDonutSmall {
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
                d: "M11 9.16V2c-5 .5-9 4.79-9 10s4 9.5 9 10v-7.16c-1-.41-2-1.52-2-2.84s1-2.43 2-2.84zM14.86 11H22c-.48-4.75-4-8.53-9-9v7.16c1 .3 1.52.98 1.86 1.84zM13 14.84V22c5-.47 8.52-4.25 9-9h-7.14c-.34.86-.86 1.54-1.86 1.84z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE5ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP8+DgQdxxTOAW/nXeTDwIK54BnAr//EQ//GOA98FvDXP6aeB9wF2eeGOA98FvDXP6aeB9wF2+Y+D+I91HHg6cJznbxd4CLDL83cceDpwnOdvF3gIsMt/DMR/rJ8C3poX7qeBt+H5+yngrXnhfhp4G/5jIP7jPBh4Oi+ahwC38pweDDydF81DgFv590P8x3lt4Ld40bwO8Ns8p9cGfosXzesAv82/H+I/zmsDv8WL5nWA3+Y5vTbwW7xoXgf4bf79EP9xHgw8nRfNQ4BbeU4PBp7Oi+YhwK38+yH+Y/008Fa8cD8DvDXP308Db8UL9zPAW/MfA/Ef6zhwK3CM5+9vgNcGdnn+jgO3Asd4/v4GeG1gl/8YiP94x4HvBt6K5/QzwHsDu7xwx4HvBt6K5/QzwHsDu/zHQfzneTDwYK64FbiVf50HAw/miluBW/mPh/j/DfH/G+L/N8T/b4j/PA8GHsQVzwBu5V/nwcCDuOIZwK38x0P8xzsOfBfw1jynnwbeB9jlhTsOfBfw1jynnwbeB9jlPw7iP9Zx4OnAcZ6/XeAhwC7P33Hg6cBxnr9d4CHALv8xEP+xfgp4a164nwbehufvp4C35oX7aeBt+I+B+I/zYODpvGgeAtzKc3ow8HReNA8BbuXfD/Ef57WB3+JF8zrAb/OcXhv4LV40rwP8Nv9+iP84rw38Fi+a1wF+m+f02sBv8aJ5HeC3+fdD/Md5MPB0XjQPAW7lOT0YeDovmocAt/Lvh/iP9dPAW/HC/Qzw1jx/Pw28FS/czwBvzX8MxH+s48CtwDGev78BXhvY5fk7DtwKHOP5+xvgtYFd/mMg/uMdB74beCue088A7w3s8sIdB74beCue088A7w3s8h8H8Z/nwcCDueJW4Fb+dR4MPJgrbgVu5T8e4v83xP9viP/fEP+/If7zPBh4EFc8A7iVf50HAw/iimcAt/IfD/Ef7zjwXcBb85x+GngfYJcX7jjwXcBb85x+GngfYJf/OIj/WMeBpwPHef52gYcAuzx/x4GnA8d5/naBhwC7/MdA/Mf6KeCteeF+Gngbnr+fAt6aF+6ngbfhPwbiP86DgafzonkIcCvP6cHA03nRPAS4lX8/xH+c1wZ+ixfN6wC/zXN6beC3eNG8DvDb/Psh/uO8NvBbvGheB/htntNrA7/Fi+Z1gN/m3w/xH+fBwNN50TwEuJXn9GDg6bxoHgLcyr8f4j/WTwNvxQv3M8Bb8/z9NPBWvHA/A7w1/zEQ/7GOA7cCx3j+/gZ4bWCX5+84cCtwjOfvb4DXBnb5j4H4j3cc+G7grXhOPwO8N7DLC3cc+G7grXhOPwO8N7DLfxzEf54HAw/miluBW/nXeTDwYK64FbiV/3iI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9QBqhBgWwwBQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDragIndicator;
impl IconShape for MdDragIndicator {
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
                d: "M11 18c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zm-2-8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 4c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+c10C/prn9Vr8x3gGcCvPCfHCvTbwXcCD+c/3O8Br87zMf4z3Ab6b54R4wd4b+C7+6/wO8No8L/Pvdwk4zvNCPH/HgacDx/mv8zvAa/O8zL/f1wAfzfNCPH8fDXwV/7V+B3htnpf593sIcCvPC/H8fTfwXvzX+h3gtXle5t/nd4DX5vlDPH+/DbwW/7V+B3htnpf593kf4Lt5/hDP328Dr8V/rd8BXpvnZf7tLgHHecEQz99vA6/F8/c7wGvzbK8N/Bb/fr8DvDbPy/zbfQ3w0bxgiOfvt4HX4vn7HeC1ebbXBn6Lf7/fAV6b52X+7R4C3MoLhnj+fht4LZ6/3wFem2d7beC3+Pf7HeC1eV7m3+Z3gNfmhUM8f78NvBbP3+8Ar82zvTbwW/z7/Q7w2jwv82/zPsB388Ihnr/fBl6L/1q/A7w2z8v8610CjvMvQzx/vw28Fv+1fgd4bZ6X+df7GuCj+Zchnr/vBt6L/1q/A7w2z8v8x/gY4Kt5Tojn76OBr+K/1u8Ar83zMv9+l4AHA7s8J8Tzdxy4FTjGf53fAV6b52X+/T4H+GyeF+IFe2/gu/iv8zvAa/O8zL/fQ4BbeV6IF+61ge8GHsR/vt8BXpvnZf59vgd4b54/xIvmpYHj/OfaBf6a5/XaPK+vBl6KF81DgFt5/hD/O/028Fr8y34HeG1eMMT/Tr8NvBb/stcBfpsXDPG/028Dr8UL9zfAS/PCIf53+m3gtXjh3gf4bl44xIvmpYFj/Pf4HZ7XbwOvxQv2DODB/MsQL9xrA98FPJj/PuJ5/TbwWrxgHwN8Nf8yxAv23sB38d9PPK/fBl6L5+8S8GBgl38Z4vk7DjwdOM5/P/G8fht4LZ6/zwE+mxcN4vn7aOCr+J9BPK/fBl6L5+8hwK28aBDP33cD78X/DOJ5/TbwWjyv7wHemxcd4vn7beC1+J9BPK/fBl6L5/UQ4FZedIjn77eB1+J/BvG8fht4LZ7T7wCvzb8O4vn7beC1eP5+B3htnu21gd/iP494Xr8NvBbP6XWA3+ZfB/H8/TbwWjx/vwO8Ns/22sBv8Z9HPK/fBl6LZ/sb4KX510M8f78NvBbP3+8Ar82zvTbwW/znEc/rt4HX4tneB/hu/vUQz99vA6/F8/c7wGvzbK8N/Bb/ecTz+m3gtbjiGcCD+bdBPH+/DbwW/zOI5/XbwGtxxccAX82/DeL5+23gtfifQTyv3wZeC7gEPBjY5d8G8fx9N/Be/M8gntdvA68FfA7w2fzbIZ6/jwa+iv8ZxPP6beC1gIcAt/Jvh3j+jgO3Asf47yee128DtwLvzb8P4gV7b+C7+O8nntdvA+8N3Mq/D+KFe23gu4EH8d9HPK/PBj6bfz/Ei+algeP89/htntdxYJd/P8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RxhbpUEjksihAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDynamicForm;
impl IconShape for MdDynamicForm {
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
                d: "M17,20v-9h-2V4h7l-2,5h2L17,20z M15,13v7H4c-1.1,0-2-0.9-2-2v-3c0-1.1,0.9-2,2-2H15z M6.25,15.75h-1.5v1.5h1.5V15.75z M13,4v7H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2H13z M6.25,6.75h-1.5v1.5h1.5V6.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAElElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/44HAw/iir8BdvmXIf73eDDwIOC1ueK1ueK1eV6vA/w2/zLE/zzHgZcCXht4MPDSwEvzr/M6wG/zL0P893tp4KWA1wZeG3gw/34ngF3+ZYj/eseB1wLeGnht4MH8xxMvGsR/jePAWwFvDbw1/7kuAcd50SD+c7008FHAWwPH+a/xO8Br86JB/Od4beCzgNfmv97vAK/NiwbxH+vBwHcBr81/n88BPpsXDeI/zmcBHw0c57/XxwBfzYsG8e93HPgu4K35n+F1gN/mRYP49zkO/Bbw0vzP8RDgVl40iH+fnwLemv9ZxIsO8W/31cBH8T/L7wCvzYsO8W/z1sBP8T/P9wDvzYsO8W/zdODB/M/zOcBn86JD/Ou9N/Bd/Oe4BBzj3+51gN/mRYf413s68GD+Yz0D+GngvYDj/NudAHZ50SH+dV4a+Cv+Y30OsAt8Ff8+zwAezL8O4l/nq4GP4j/GM4C3Bl4a+C7+/b4HeG/+dRD/On8FvDT/ft8DfDTwYOCv+I/xPsB386+D+Ncx/37vA3w38NLAbwHH+Y/xEOBW/nUQL7oHA0/n3+4S8N7ATwPHgb8CHsx/jGcAD+ZfD/Gie23gt/i3uQS8NvDXXPFbwGvzH+d7gPfmXw/xontt4Lf417sEvDbw11zx2cBn8R/rbYCf5l8P8aJ7beC3+Ne5BLw28Ndc8dLAX/Ef7wSwy78e4kX32sBv8a/zOsBv82x/Bbw0/7F+Bnhr/m0Q/zrmRfc+wHfzbB8NfBX/8d4H+G7+bRD/OrvAMf5l3wO8N892HHg6cJz/WJeABwO7/Nsg/nW+G3gvXri/AV4b2OXZPhv4LP7jfQ/w3vzbIf513hr4KV64lwH+mmc7DjwdOM5/vIcAt/Jvh/jXOQ7cChzj+fsc4LN5Tp8NfBb/8X4HeG3+fRD/ep8NfBbP6xnAg3leTwcezH+81wF+m38fxL/eceBW4BjP6XWA3+Y5vTfwXfzH+x3gtfn3Q/zbfDbwWTzbzwBvzfP6LeC1+Y91CXhp4Fb+/RD/dn8NvBRXvA7w2zyn48BF/uN9DPDV/MdA/NsdB24F/hp4bZ7XewPfxX+s7wHem/84iH+flwaOA7/N8/pp4K34j/M3wGsDu/zHQfznMf9xfgZ4b2CX/1iI/xwvDfwV/zE+B/hs/nMg/nN8NPBV/Ps8A3hv4Lf5z4P4z/HRwGcDx/jXewbw2cB3858P8Z/nOPDWwGsDbw0c4wX7G+CngZ8G/pr/Ooj/OseBl+Y5/TWwy38fxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHcpWNQUa3ApkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEco;
impl IconShape for MdEco {
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
                d: "M6.05,8.05c-2.73,2.73-2.73,7.15-0.02,9.88c1.47-3.4,4.09-6.24,7.36-7.93c-2.77,2.34-4.71,5.61-5.39,9.32 c2.6,1.23,5.8,0.78,7.95-1.37C19.43,14.47,20,4,20,4S9.53,4.57,6.05,8.05z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEditOff;
impl IconShape for MdEditOff {
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
                d: "M12.126 8.125l1.937-1.937 3.747 3.747-1.937 1.938zM20.71 5.63l-2.34-2.34a1 1 0 0 0-1.41 0l-1.83 1.83 3.75 3.75L20.71 7a1 1 0 0 0 0-1.37zM2 5l6.63 6.63L3 17.25V21h3.75l5.63-5.62L18 21l2-2L4 3 2 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADAUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/70ezBW38t8D8d/rt7jidfjvgfjv89HAV3HFxwBfzX89xH+PBwN/BRznil3gZYBb+a+F+O/xW8Br85x+G3gd/msh/ut9NPBVPH8fA3w1/3UQ/7UeDPwVcJznbxd4GeBW/msg/mv9FvDavHC/DbwO/zUQ/3U+GvgqXjQfA3w1//kQ/zUeDPwVcJwXzS7wMsCt/OdC/Nf4LeC1+df5beB1+M+F+M/30cBX8W/zMcBX858H8Z/rwcBfAcf5t9kFXga4lf8ciP9cvwW8Nv8+vw28Dv85EP95Phr4Kv5jfAzw1fzHQ/zneDDwV8Bx/mPsAi8D3Mp/LMR/jt8CXpv/WL8NvA7/sRD/8T4a+Cr+c3wM8NX8x0H8x3ow8FfAcf5z7AIvA9zKfwzEf6zfAl6b/1y/DbwO/zEQ/3E+Gvgq/mt8DPDV/Psh/mM8GPgr4Dj/NXaBlwFu5d8H8R/jt4DX5r/WbwOvw78P4t/vo4Gv4r/HxwBfzb8d4t/nwcBfAcf577ELvAxwK/82iH+f3wJem/9evw28Dv82iH+7jwa+iv8ZPgb4av71EP+/If5/Q/z/hvj/DfH/G+Lf5quBl+J/lr8BPpp/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H823w18NL8z/LXwEfzr4P4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcNE0hBpp0jNwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEject;
impl IconShape for MdEject {
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
                d: "M5 17h14v2H5zm7-12L5.33 15h13.34z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEV0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+E48FrAawMvzRUPBh7Ms90K3Ar8NfDbwN8At/Lvg/jvcxx4K+CjgZfm3+avge8GvgfY5V8P8V/vOPBZwHsDx/mPsQv8NPAxwC4vOsR/rY8CPhs4zn+OXeCrgc/hRYP4r/HSwFcBr81/jb8G3ga4lRcO8Z/vpYHfAo7zX2sXeB/gp3nBEP+53hv4Lv71/gbY5Tm9NHCMf53vAd6bFwzxn+e9ge/iRXMJ+Gngu4G/BnZ5/o4Drw28NfDWwDFesO8B3psXDvGf46WBv+Jf9gzgs4Hv5l/vOPDRwHsDD+I5fQ/w3vzLEP/xHgz8FXCcF+5zgK8Gdvn3+2rgo7jie4D35kWD+I/3W8Br84JdAt4a+G3+Y7018NrAR/OiQ/zH+mjgq3jBLgGvDfw1/zMg/uMcB54OHOf5uwS8NvDX/M+B+I/z2cBn8YK9DvDb/M+C+I9zETjO8/c5wGfzPw/iP8Z7A9/F8/cM4KWBXf7nQfzLzH8v8Z8H8S8z/73Efx7Ev8z89xL/eRD/MvPfS/znQfzLzH8v8Z8H8S97bV647wIezPP6G+Cj+ff7bf7zIP79zPP3O8Br8z8b4t/PPH+/A7w2/7Mh/v3M8/c7wGvzPxvi3888f78DvDb/syH+Za/FC/fdwIN5Xn8NfDT/fr/Dfx7Ev8z89xL/eRD/MvPfS/znQfzLzH8v8Z8H8S8z/73Efx7Ev8z89xL/eRD/Md4b+C6ev1uBlwF2+Z8H8R9nFzjG8/c5wGfzPw/iP85nA5/FC/Y6wG/zPwviP85x4FbgGM/fLvA6wF/zPwfiP9ZHA1/FC7YLvA7w1/zPgPiP99vAa/GC7QJvA/w2/7HeGngt4GN40SH+4z0Y+GvgGC/cZwNfA+zy73Mc+Czgo7niu4H34UWD+M/x0sBf8S+7Ffhs4Hv41zsOfBTw0cBxntN3A+/Dvwzxn+e9ge/iRbML/DTw3cDfALs8f8eB1wLeGnhr4Dgv2HcD78MLh/jP9d7Ad/GvdytwK8/ppYHj/Ot8N/A+vGCI/3wvDfw2cIz/WpeA9wZ+mhcM8V/jpYGvBl6L/xp/A7w1cCsvHOK/1kcDnw0c4z/HJeCrgc/mRYP4r3cc+GzgvYFj/Me4BHw38NnALi86xH+v9wY+Gngp/m3+Bvhu4LuBXf71EP8zHAdeG3hp4LW54sHAg3i2ZwC3An8N/Dbw18Ct/Psg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/PWpJBpX+8DgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEuroSymbol;
impl IconShape for MdEuroSymbol {
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
                d: "M15 18.5c-2.51 0-4.68-1.42-5.76-3.5H15v-2H8.58c-.05-.33-.08-.66-.08-1s.03-.67.08-1H15V9H9.24C10.32 6.92 12.5 5.5 15 5.5c1.61 0 3.09.59 4.23 1.57L21 5.3C19.41 3.87 17.3 3 15 3c-3.92 0-7.24 2.51-8.48 6H3v2h3.06c-.04.33-.06.66-.06 1 0 .34.02.67.06 1H3v2h3.52c1.24 3.49 4.56 6 8.48 6 2.31 0 4.41-.87 6-2.3l-1.78-1.77c-1.13.98-2.6 1.57-4.22 1.57z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Y9h/vVeB/htntNrA7/F8xIvGOJfZp7X6wC/zX8M86/3OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5jmH+91wF+m+f02sBv8bzEC4b4l5nn9TrAb/Mfw/zrvQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpv/GOZf73WA3+Y5vTbwWzwv8YIh/mXmeb0O8Nv8xzD/eq8D/DbP6bWB3+J5iRcM8S8zz+t1gN/mP4b513sd4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2/zHMv97rAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3+Y9h/vVeB/htntNrA7/F8xIvGOJfZp7X6wC/zX8M86/3OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5jmH+91wF+m+f02sBv8bzEC4b4l5nn9TrAb/Mfw/zrvQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpv/GOZf73WA3+Y5vTbwWzwv8YIh/mXmeb0O8Nv8x3ht/vX+GtjlOb028Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeX008Nf8z/LSwFfzvMQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8y14a+Cv+d3oIcCsvGOJF897Ad/G/y/sA380Lh3jRPRj4bOCtgWP8z3QJ+Gngs4Fb+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+vA6hBVV+wHQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEvent;
impl IconShape for MdEvent {
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
                d: "M17 12h-5v5h5v-5zM16 1v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2h-1V1h-2zm3 18H5V8h14v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/FewEcDL81/jL8Gvhr4Hv5jIf7jfRfw3vzn+G7gffiPg/iP9dnAZ/Gf63OAz+Y/BuI/1kXgOP+5doET/MdA/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RDP30sDx3jB/gbY5Tm9NvBb/Nd4HeC3eU7HgZfiBbsE/DXPCfH8/TbwWrxgrwP8Ns/ptYHf4r/G6wC/zXN6beC3eMF+B3htnhPi+ftt4LV4wV4H+G2e02sDv8V/jdcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8YK9DvDbPKfXBn6L/xqvA/w2z+m1gd/iBfsd4LV5Tojn77eB1+IFex3gt3lOrw38Fv81Xgf4bZ7TawO/xQv2O8Br85wQz99vA6/FC/Y6wG/znF4b+C3+a7wO8Ns8p9cGfosX7HeA1+Y5IZ6/3wZeixfsdYDf5jm9NvBb/Nd4HeC3eU6vDfwWL9jvAK/Nc0I8f78NvBYv2OsAv81zem3gt/iv8TrAb/OcXhv4LV6w3wFem+eEeP6+GnhpXrCPBv6a5/TSwFfzX+Ojgb/mOb008NW8YH8NfDTPCfH/G+L/N8T/b4j/3xD/vyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fi/Y7wCvzX+N3wZeixfsdYDf5l+GeNG9NvBbvGC/A7w2/zV+G3gtXrDXAX6bfxniRffawG/xgv0O8Nr81/ht4LV4wV4H+G3+ZYgX3WsDv8UL9jvAa/Nf47eB1+IFex3gt/mXIV50rw38Fi/Y7wCvzX+N3wZeixfsdYDf5l+GeNG9NvBbvGC/A7w2/zV+G3gtXrDXAX6bfxniRffawG/xgv0O8Nr81/ht4LV4wV4H+G3+ZYgX3WsDv8UL9jvAa/Nf47eB1+IFex3gt/mXIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP06WLQcdGdiwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEventSeat;
impl IconShape for MdEventSeat {
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
                d: "M4,18v3h3v-3h10v3h3v-6H4V18z M19,10h3v3h-3V10z M2,10h3v3H2V10z M17,13H7V5c0-1.1,0.9-2,2-2h6c1.1,0,2,0.9,2,2V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5nl9DPDX/M/y0sBX8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8+/zYOBW/uO8NvBbPC/xgiH+ZeZ5vQ7w2/zbvTTwW8BPA+/Df4zXBn6L5yVeMMS/zDyv1wF+m3+blwZ+CzjOFd8NvA//fq8N/BbPS7xgiH+ZeV6vA/w2/3ovDfwWcJzn9N3A+/Dv89rAb/G8xAuG+JeZ5/U6wG/zr3cr8CCev+8G3od/u9cGfovnJV4wxL/MPK/XAX6bf72XBn4bOMbz993A+/Bv89rAb/G8xAuG+K/30sBvA8d4/r4beB/+9V4b+C2el3jBEP89Xhr4beAYz993A+/Dv85rA7/F8xIvGOK/z0sDvw0c4/n7buB9eNG9NvBbPC/xgiH+e7008NvAMZ6/7wbehxfNawO/xfMSLxjiv99LA78NHOP5+27gffiXvTbwWzwv8YIh/mW/xX++BwMP5gX7buB9eOFeG/gtnpd4wRD/MvM/w3cD78ML9trAb/G8xAuG+JeZ/zm+G3gfnr/XBn6L5yVeMMS/zPzP8T7Ad/P8vTbwWzwv8YIh/mXmf4b3Ab6bF+y1gd/ieYkXDPEv+23+8z0YeBAv2PsA380L99rAb/G8xAuG+O/30sBvAcd5/t4H+G7+Za8N/BbPS7xgiP9eLw38FnCc5+99gO/mRfPawG/xvMQLhvjv89LAbwHHef7eB/huXnSvDfwWz0u8YIj/Hi8N/BZwnOfvfYDv5l/ntYHf4nmJFwzxX++lgd8CjvP8vQ/w3fzrvTbwWzwv8YIh/mXmeb0O8Nv867008FvAcZ6/9wG+m3+b1wZ+i+clXjDEv8w8r9cBfpt/vVuBB/H8vQ/w3fzbvTbwWzwv8YIh/mXmeb0O8Nv867008NvAMZ7T+wDfzb/PawO/xfMSLxjiX2ae1+sAv82/zUsDvw0c44r3Ab6bf7/XBn6L5yVeMMS/zDyv1wF+m3+7lwZ+G/ho4Lv5j/HawG/xvMQLhviXmef1OsBv8+9zHNjlP85rA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntdHA3/N/ywvDXw1z0u8YIh/2S5wjP+dLgHHecEQ/7LvBt6L/52+B3hvXjDEv+ylgb/if6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IWsKgQaDvqC4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExitToApp;
impl IconShape for MdExitToApp {
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
                d: "M10.09 15.59L11.5 17l5-5-5-5-1.41 1.41L12.67 11H3v2h9.67l-2.58 2.59zM19 3H5c-1.11 0-2 .9-2 2v4h2V5h14v14H5v-4H3v4c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAvupcGvor/HT4G+Gv+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTTw1fzv8NHAX/MvQ/z/hvj/DfHf66W54q/574H47/PSwG9xxesAf81/PcR/j5cGfgs4zhW7wOsAf81/LcR/vZcGfgs4znPaBV4H+Gv+6yD+a7008FvAcZ6/XeB1gL/mvwbiv85LA78FHOeF2wVeB/hr/vMh/mu8NPBbwHFeNLvA6wB/zX8uxH++lwZ+CzjOv84u8DrAX/OfB/Gf66WB3wKO82+zC7wO8Nf850D853lp4LeA4/z77AKvA/w1//EQ/zleGvgt4Dj/MXaB1wH+mv9YiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/xwvDfw2cIz/GJeA1wb+mv9YiP88Lw38NnCMf59LwGsDf81/PMR/rpcGfhs4xr/NJeC1gb/mPwfiP99LA78NHONf5xLw2sBf858H8V/jpYHfBo7xorkEvDbw1/znQvzXeWngt4FjvHCXgNcG/pr/fIj/Wi8N/DZwjOfvEvDawF/zXwPxX++lgd8GjvGcLgGvDfw1/3UQ/z1eGvht4BhXXAJeG/hr/msh/vu8NPDbXPHawF/zXw/x3+ulueKv+e+B+P8N8f8b4kX30sBX8b/DxwB/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7qXBr6a/x0+Gvhr/mWI/98Q/78h/n9D/P+G+P+NfwTXTXRBWjmXMQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExpand;
impl IconShape for MdExpand {
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
                d: "M4 20h16v2H4zM4 2h16v2H4zm9 7h3l-4-4-4 4h3v6H8l4 4 4-4h-3z",
            }
            path {
                d: "M0 0h24v24H0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExplore;
impl IconShape for MdExplore {
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
                d: "M12 10.9c-.61 0-1.1.49-1.1 1.1s.49 1.1 1.1 1.1c.61 0 1.1-.49 1.1-1.1s-.49-1.1-1.1-1.1zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm2.19 12.19L6 18l3.81-8.19L18 6l-3.81 8.19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHqElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4ba54MPBgYBf4a67YBf4aeAZwK//5EM/fewOvBbwP/z5vBbw18NrAg/nX+Wvgt4GfAX6b/xyI5/XewHdxxXcD78O/znHgo4D3Bh7Mf4xbge8GvgbY5T8O4jm9N/BdPKfvBt6Hf9lx4KOAjwaO859jF/hq4GuAXf79EM/2YODpPH/fDbwPL9hbA98FHOe/xi7w0cD38O+DeE7vDXwXz993A+/D8/fdwHvxX++ngfcBdvm3QTyv9wa+i+fvu4H34fn7buC9+K93K/A2wF/zr4d4/t4b+C6ev+8G3ofn77uB9+K/3i7wOsBf86+DeMHeG/gunr/vBt6H5++7gffiv94u8DrAX/OiQ7xw7w18F8/fdwPvw/P33cB78V9vF3gd4K950SD+Ze8NfBfP33cD78Pz993Ae/Ff71bgZYBd/mWIF817A9/F8/fdwPvw/H038F785/gZ4K+Bz+J5/TTwNvzLEC+atwZ+ihfsu4H34fn7buC9+I/zPcBnA7cCPw28Fc/f+wDfzQuH+JcdB54OHOeF+27gfXj+vht4L/7tLgHfDXw1cCtXPBh4Oi/YLvAQYJcXDPEv+2zgs3jRfDfwPjx/3w28F/86l4CvBr4a2OU5fTfwXrxwnwN8Ni8Y4oU7DjwdOM6L7ruB9+H5+27gvfiXPQP4bOC7ef4eDDydf9ku8BBgl+cP8cJ9NvBZ/Ot9N/A+PH/fDbwXz9/fAF8NfDcv3FcDH8WL5nOAz+b5Q7xwTwcezL/NdwPvw/P33cB78Wy/A3w28Nv8y44DTweO86K5FXgIzx/iBXtr4Kf49/lu4H14/r6bK74a+GtedJ8NfBb/Oq8D/DbPC/GCfTfwXvz7fTfwPvzHuQgc51/na4CP5nkhXrCnAw/mP8Z3A+/Dv997A9/Fv95fAy/D80I8f8eBi/zH+m7gffj3eTrwYP5tTgC7PCfE8/fawG/xH++7gffh3+a9ge/i3+51gN/mOSGev88GPov/HN8NvA//en8FvDT/dh8DfDXPCfH8fTbwWfzn+W7gfXjRvTbwW/z7fA7w2TwnxPP31cBH8Z/ru4H34UXzW8Br8+/zOcBn85wQz99vA6/Fv9/3AL8NfBfP33cD78ML99LAX/Hv9zPAW/OcEM/fbwOvxb/fywB/Dbw38F08f98NvA8v2HcD78W/3+8Ar81zQjx/nw18Fv8+vwO8Ns/23sB38fx9N/A+PH/fDbwX/36fA3w2zwnx/H028Fn8+7wO8Ns8p/cGvovn77uB9+H5+27gvfj3+Rzgs3lOiOfvo4Gv4t/ub4CX5vl7b+C7eP6+G3gfnr/vBt6Lf7uPAb6a54R4/l4b+C3+7d4H+G5esPcGvovn77uB9+H5+27gvfi3eR3gt3lOiOfvwcDT+bd5BvBg/mXvDXwXz993A+/D8/fdwHvxr3cC2OU5IV6wvwZein+99wG+mxfNewPfxfP33cD78Pz9NfBSvOieATyY54V4wb4a+Cj+dS4Bx/nXeW/gu3j+vht4H57XceC3gZfiRfM1wEfzvBAv2GsDv8W/zucAn82L7qWBjwLemxfsu4H34XkdB34beCn+Za8D/DbPC/HC3Qo8iBfNJeDBwC7/stcGPgt4bV403w28D8/rOPDbwEvxgj0DeDDPH+KF+2zgs3jRfA3w0bxw7wV8NPDS/Ot9N/A+PK/jwG8DL8Xz9znAZ/P8IV6448CtwDH+ZQ8BbuX5ey/gs4EH8+/z3cD78LyOA78NvBTP6RLwYGCX5w/xL/ts4LN44b4HeG+e03Hgo4CPBo7zH+e7gffheR0Hfht4KZ7tc4DP5gVD/MuOA7cCx3jBHgLcyhUPBj4KeG/gOP85vht4H57XceC3gZcCLgEPBnZ5wRAvmvcGvovn72eAtwYeDHwW8N781/hu4H14XseB3wY+G/hpXjjEi+6ngbfieX0O8FLAW/Nf77uB9+F5HQd2+ZchXnTHgb8GHsT/LN8NvA//Noh/nZcGfhs4xv8s3w28D/96iH+9lwZ+GzjG/yzfDbwP/zqIf5uXBn4bOMb/LN8NvA8vOsS/3UsDPw08iP9Zvht4H140iH+f48B3A2/F/yzfDbwP/zLEf4z3Br4aOMZ/jWcAH80VP8Xz993A+/DCIf7jHAc+Gvho4Bj/OS4BXw18NbDLFe8NfBfP33cD78MLhviPdxz4aOC9gQfxH+MZwHcDXw3s8rzeG/gunr/vBt6H5w/xn+u1gbcGXht4Kf51ngH8NPDTwG/zL3tv4Lt4/r4beB+eF+K/znHgpYGXBo5zxUsDx4G/BnaBXeCvgb8GdvnXe2/gu3he7wN8N88L8X/PewPfxbO9D/DdPH+I/5veG/gu4H2A7+YFQ/zf9WDgVl44xP9viP/f+EectyZQI4+4fgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExploreOff;
impl IconShape for MdExploreOff {
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
                d: "M14.19 14.19l-1.41-1.41-1.56-1.56L11 11 9.81 9.81 4.93 4.93 2.27 2.27 1 3.54l2.78 2.78c-.11.16-.21.32-.31.48-.04.07-.09.14-.13.21-.09.15-.17.31-.25.47-.05.1-.1.21-.16.32-.06.14-.13.28-.19.43-.1.24-.19.48-.27.73l-.09.3c-.05.2-.1.39-.14.59-.02.11-.04.22-.07.33-.04.2-.07.4-.09.61-.01.1-.03.2-.03.3-.03.29-.05.6-.05.91 0 5.52 4.48 10 10 10 .31 0 .62-.02.92-.05l.3-.03c.2-.02.41-.06.61-.09.11-.02.22-.04.33-.07.2-.04.39-.09.58-.15.1-.03.2-.05.3-.09.25-.08.49-.17.73-.27.15-.06.29-.13.43-.19.11-.05.22-.1.33-.16.16-.08.31-.16.46-.25.07-.04.14-.09.21-.13.16-.1.32-.2.48-.31L20.46 23l1.27-1.27-2.66-2.66-4.88-4.88zM6 18l3-6.46L12.46 15 6 18zm16-6c0 .31-.02.62-.05.92l-.03.3c-.02.2-.06.41-.09.61-.02.11-.04.22-.07.33-.04.2-.09.39-.15.58-.03.1-.05.21-.09.31-.08.25-.17.49-.27.73-.06.15-.13.29-.19.43-.05.11-.1.22-.16.33-.08.16-.16.31-.25.46-.04.07-.09.14-.13.21-.1.16-.2.32-.31.48L15 12.46 18 6l-6.46 3-5.22-5.22c.16-.11.32-.21.48-.31.07-.04.14-.09.21-.13.15-.09.31-.17.46-.25.11-.05.22-.1.33-.16.14-.06.28-.13.43-.19.24-.1.48-.19.73-.27l.31-.09c.19-.05.38-.11.58-.15.11-.02.22-.04.33-.07.2-.04.4-.07.61-.09.1-.01.2-.03.3-.03.29-.02.6-.04.91-.04 5.52 0 10 4.48 10 10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/zoOBB3HFM4Bb+e+H+M/31sBXAQ/mOd0KfAzw0/z3Qfzn+irgo3nhvhr4GP57IP7zfDTwVbxoPgb4av7rIf5zHAeeDhznRbMLPATY5b8W4j/HewPfxb/O+wDfzX8txH+OzwY+i3+dzwE+m/9aiP8cvw28Fv86vwO8Nv+1EP85fht4Lf51fgd4bf5rIf5z/DbwWvzr/A7w2vzXQvzn+G3gtfjX+R3gtfmvhfiXvTTwVsBLA8d50bw0cJx/nV3gr3nR7AJ/DfwM8Nf82yFeuK8CPpr/2b4a+Bj+bRAv2FcDH8X/Dl8DfDT/eojn78HA0/nf5SHArfzrIJ6/zwY+i/9dPgf4bP51EM/fdwPvxf8uPwO8Nf86iOfvt4HX4n+X3wFem38dxPP328Br8b/L7wCvzb8O4vn7beC1+N/ld4DX5l8H8fz9NvBa/O/yO8Br86+DeP5+G3gt/nf5HeC1+ddBPH+/DbwW/7v8DvDa/Osgnr/fBl6L/11+B3ht/nUQz99vA6/F/y6/A7w2/zqI5++3gdfif5ffAV6bfx3E8/fVwEvznF6L/9l+B3ht/nUQ/zovDbw38FH8z7ML/DXP6beBW4GfAXZ5Xoh/m5cGfhp4EP877AKfDXwNzwnxb/fSwG8Dx/jf42uAj+bZEP8+7w18F/+7vA3w01yB+PfbBY7xv8dfAy/DFYh/v+8G3ov/XR4C3Aog/v0+G/gs/nd5HeC3AcS/32cDn8X/Lq8D/DaA+Pf7buC9+N/lIcCtAOLf7yJwnP89/gZ4aa5A/Pu8N/Bd/O/yNsBPcwXi3+6lgd8CjvO/x9cAH82zIf5tXhr4LeA4/ztcAj4a+G6eE+Jf56WB9wI+mv95LgF/zXP6beBW4KeBXZ4X4vn7auCleE6vzf9svwO8Nv86iOfvt4HX4n+X3wFem38dxPP328Br8b/L7wCvzb8O4vn7beC1+N/ld4DX5l8H8fz9NvBa/Me5BPw0cCtXPBh4a+AY/3F+B3ht/nUQz99vA6/Fv98l4KOB7+b5e2/gq4Fj/Pv9DvDa/Osgnr/fBl6Lf59nAG8N/DUv3EsDPw08iH+f3wFem38dxPP328Br8e/zMsBf86J5aeCv+Pf5HeC1+ddBPH+/DbwW/3afA3w2/zpfDXwU/3a/A7w2/zqI5++3gdfi3+4hwK3867w08Ff82/0O8Nr86yCev58G3op/m0vAcf5tzL/dzwBvzb8O4vn7bOCz+Lf5HeC1+bf5beC1+Lf5HOCz+ddBPH8vDfwV/za/A7w2/za/DbwW/zYPAW7lXwfxgn018FH86/0O8Nr82/w28Fr8630N8NH86yFeuK8GPop/nd8BXpt/m98GXot/na8BPpp/G8S/7KWBtwZeGjjOv+yvgY/m3+argZfmX7YL/DXw3cCt/Nsh/n9D/P+G+P8N8f8b4v83/hGSLp9BhLjdCwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExtension;
impl IconShape for MdExtension {
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
                d: "M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-1.99.9-1.99 2v3.8H3.5c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7H2V20c0 1.1.9 2 2 2h3.8v-1.5c0-1.49 1.21-2.7 2.7-2.7 1.49 0 2.7 1.21 2.7 2.7V22H17c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFace;
impl IconShape for MdFace {
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
                d: "M9 11.75c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zm6 0c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8 0-.29.02-.58.05-.86 2.36-1.05 4.23-2.98 5.21-5.37C11.07 8.33 14.05 10 17.42 10c.78 0 1.53-.09 2.25-.26.21.71.33 1.47.33 2.26 0 4.41-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwWL9jfALv86xwHXor/Pr8DvDbPCfH8/TbwWrxgrwP8Nv86rw38Fv99fgd4bZ4T4vn7beC1eMFeB/ht/nVeG/gt/vv8DvDaPCfE8/fbwGvxgr0O8Nv867w28Fv89/kd4LV5Tojn77eB1+IFex3gt/nXeW3gt/jv8zvAa/OcEM/fbwOvxQv2OsBv86/z2sBv8d/nd4DX5jkhnr/fBl6L//neB3ht4L34l/0O8No8J8Tz99vAa/E/2/sA380V3w28Fy/c7wCvzXNCPH+/DbwW/3O9D/DdPKfvBt6LF+x3gNfmOSGev98GXov/md4H+G6ev+8G3ovn73eA1+Y5IZ6/3wZei/96zwAexAv2PsB38/wdB34LeGmev98BXpvnhHj+fht4LV6wjwH+mn+dlwa+ihfsb4DXBt4a+C6e1/sA383zdxz4LeClecF+B3htnhPi+ftt4LV4wV4H+G3+dV4b+C2ev78BXhvY5Yr3Br6LZ3sf4Lt5/o4DvwW8NC/c7wCvzXNCPH+/DbwWL9jrAL/Nv85rA7/F8/ob4LWBXZ7TewPfBbwP8N08f8eB3wJemn/Z7wCvzXNCPH+/DbwWL9jrAL/Nv85rA7/F83od4Ld5/h4M3Mrzdxz4LeCledH8DvDaPCfE8/fbwGvxgr0O8Nv867w28Fs8r13gdYC/5kV3HPgt4KV50f0O8No8J8Tz99vAa/GCfTTw1/zrvDTw1Tx/u8DrAH/Nv+w48FvAS/Ov8zvAa/OcEM/fbwOvxX+tXeB1gL/mBTsO/Bbw0vzr/Q7w2jwnxPP328Br8V9vF3gd4K95XseB3wJemn+b3wFem+eEeP5+G3gt/nvsAq8D/DXPdhz4LeCl+bf7HeC1eU6I5++3gdfiv88u8DrAXwPHgd8CXpp/n98BXpvnhHj+fht4Lf577QJvA3wV8NL8+/0O8No8J8Tz99vAa/GCvQ7w2/zrvDbwW/z3+R3gtXlOiOfvt4HX4gV7HeC3+dd5beC3+O/zO8Br85wQz99vA6/FC/Y6wG/zr/PawG/x3+d3gNfmOSGev98GXosX7HWA3+Zf57WB3+K/z+8Ar81zQjx/vw28Fi/Y6wC/zb/OawO/xX+f3wFem+eEeP5+G3gtXrC/Bnb51zkOvDT/fX4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev88GPov/Wz4H+GyeE+L5e2vgp/i/5W2An+Y5IV6wvwZeiv8b/gZ4aZ4X4gV7MPDTwEvxv9vfAG8N3MrzQrxwx4GPBt4beBD/uzwD+G7gq4Fdnj/E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hF9hblBDXg38AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFactCheck;
impl IconShape for MdFactCheck {
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
                d: "M20,3H4C2.9,3,2,3.9,2,5v14c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V5 C22,3.9,21.1,3,20,3z M10,17H5v-2h5V17z M10,13H5v-2h5V13z M10,9H5V7h5V9z M14.82,15L12,12.16l1.41-1.41l1.41,1.42L17.99,9 l1.42,1.42L14.82,15z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAELElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b493kt4K2BlwaOAy/NFb/NFX8NfA/w1/zbvDTwXsBLc8Vrc8VfA7vAXwM/DfwO/zaIf73jwEcBHw0c50VzK/DdwNcAu7xwx4GPAt4beDAvml3gq4GvAXZ50SH+dd4a+Crgwfzb7AIfDXwPz997AV8NHOff5lbgY4Cf5kWDeNF9FvDZ/Mf4beBtgF2uOA78FPDa/Mf4bOBz+JchXjTfBbw3/7H+Gngdrvgt4KX5j/XdwPvwwiH+ZR8NfBX/Of6aK16a/xwfA3w1LxjihXtt4Lf43+11gN/m+UO8cL8FvDb/u/028Do8f4gX7K2Bn+L/hrcBfprnhXjBfhp4K/5v+BngrXleiOfvOHCR/1vE80I8f28N/BT/t7wN8NM8J8Tz99nAZ/F/y+cAn81zQjx/nw18Fv+3fA7w2TwnxPP308Bb8X/LzwBvzXNCPH+fDXwW/7d8DvDZPCfE8/fZwGfxf8vnAJ/Nc0I8f28N/BT/t7wN8NM8J8Tz99LAX/F/y8sAf81zQrxgtwIP4v+GZwAP5nkhXrCvBj6K/xu+BvhonhfiBXtp4K/4v+FlgL/meSFeuN8GXov/3X4HeG2eP8QL99rAb/G/28sAf83zh/iXfTfwXvzv9D3Ae/OCIf5lDwb+GjjG/y6XgJcGbuUFQ7xo3hv4Lv53eR/gu3nhEC+67wbei/8dvgd4b/5liBfdceC3gZfif7a/AV4b2OVfhvjXeWngt4Fj/M90CXht4K950SD+9V4a+Cv+Z3oZ4K950SH+bd4b+C7+Z3kf4Lv510H827038F38z/A+wHfzr4f493lv4Lv47/U+wHfzb4P493tv4Lv47/E+wHfzb4f4j/HewHfxX+t9gO/m3wfxH+e9ge/iv8b7AN/Nvx/iP9Z7A9/Ff673Ab6b/xiI/3jvDXwX/zneB/hu/uMg/nO8N/Bd/Md6H+C7+Y+F+M/z1sB3A8f497kEvDfw0/zHQ/znemngt4Fj/NtcAl4b+Gv+cyD+87008NvAMf51LgGvDfw1/3kQ/zUeDPw08FK8aP4GeGvgVv5zIf7rHAd+G3gpXri/AV4b2OU/H+K/1nHgp4HX4vn7G+C1gV3+ayD+e3w38F48p+8B3pv/Woj/Pt8NvBdXfA/w3vzXQ/z3+miu+Gr+eyD+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R6Y9gEFFJmUHAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFavorite;
impl IconShape for MdFavorite {
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
                d: "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFavoriteBorder;
impl IconShape for MdFavoriteBorder {
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
                d: "M16.5 3c-1.74 0-3.41.81-4.5 2.09C10.91 3.81 9.24 3 7.5 3 4.42 3 2 5.42 2 8.5c0 3.78 3.4 6.86 8.55 11.54L12 21.35l1.45-1.32C18.6 15.36 22 12.28 22 8.5 22 5.42 19.58 3 16.5 3zm-4.4 15.55l-.1.1-.1-.1C7.14 14.24 4 11.39 4 8.5 4 6.5 5.5 5 7.5 5c1.54 0 3.04.99 3.57 2.36h1.87C13.46 5.99 14.96 5 16.5 5c2 0 3.5 1.5 3.5 3.5 0 2.89-3.14 5.74-7.9 10.05z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/Ff57XAX6b5/TawG/xn+d3gNfmOSGev98GXov/PK8D/DbP6bWB3+I/z+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4HeG2eE+L5+23gtfjP89HAX/OcXhr4av7z/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4HeG2eE+L5+23gtfjP8zrAb/OcXhv4Lf7z/A7w2jwnxPP328Br8Z/ndYDf5jm9NvBb/Of5HeC1eU6I5++3gdfiP89fA7s8p+PAS/Of53eA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XOAz+Y5IZ6/3wZei/9b3gb4aZ4T4vn7beC1+L/jb4CX5nkhnr/fBl6L/xv+Bnhr4FaeF+L5+23gtfjf7RnAdwNfDezy/CGev98GXot/2fsA383/Xojn77eB1+KFex/gu/nfDfH8/TbwWrxg7wN8N//7IZ6/3wZei+fvfYDv5v8GxPP328Br8bzeB/hu/u9APH+/DbwWz+l9gO/m/xbE8/fbwGvxbO8DfDf/9yCev98GXosr3gf4bv5vQjx/vw28FvA+wHfzfxfi+ftt4LuB7+b/NsTz99LAX/N/H+L/N8T/b4j/3xD/vyH+f+MfAbZzkkEd94cQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFeedback;
impl IconShape for MdFeedback {
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 12h-2v-2h2v2zm0-4h-2V6h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilePresent;
impl IconShape for MdFilePresent {
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
                d: "M15 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V7l-5-5zM6 20V4h8v4h4v12H6zm10-10v5c0 2.21-1.79 4-4 4s-4-1.79-4-4V8.5c0-1.47 1.26-2.64 2.76-2.49 1.3.13 2.24 1.32 2.24 2.63V15h-2V8.5c0-.28-.22-.5-.5-.5s-.5.22-.5.5V15c0 1.1.9 2 2 2s2-.9 2-2v-5h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX+/BwIP4n+kZwK286BD/Ou8NfBf/s70P8N28aBAvuuPARf53OAHs8i9DvOheG/gt/nd4GeCv+Zch/nVuBR7E/2zPAB7Miwbxr/PawG/xP9vLAH/Niwbxr/fRwFfxP9PHAF/Niw7xb/PTwFvxP8v3AO/Nvw7i3+Y48NvAS/E/w98Arw3s8q+D+Ld7aeC3gWP897oEvDbw1/zrIf593hv4Lv57vQ3w0/zbIP79vhr4KP57fA3w0fzbIf5j/DXwUvzX+h3gtfn3QfzHOA7cChzjv8Yl4MHALv8+iP84rw38Fv81Xgb4a/79EP+xPhr4Kv5zfQzw1fzHQPzH+2ngrfjP8T3Ae/MfB/Ef7zjw28BL8R/rb4DXBnb5j4P4z/HSwG8Dx/iPcQl4beCv+Y+F+M/z3sB38R/jfYDv5j8e4j/XVwMfxb/P1wAfzX8OxH++vwZein+b3wFem/88iP98x4FbgWP861wCHgzs8p8H8V/jtYHf4l/ndYDf5j8X4r/ORwNfxYvmY4Cv5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OawO/xYvmdYDf5j8f4r/OSwN/xYvmZYC/5j8f4r/WZwOfxQv3OcBn818D8V/vpYHjPH+7wF/zXwfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHJhV5BrqWoOAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterAlt;
impl IconShape for MdFilterAlt {
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
                d: "M4.25,5.61C6.27,8.2,10,13,10,13v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-6c0,0,3.72-4.8,5.74-7.39 C20.25,4.95,19.78,4,18.95,4H5.04C4.21,4,3.74,4.95,4.25,5.61z",
            }
            path {
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++lgWP8z3QJ+GtedIh/nc8GPov/2T4H+GxeNIgX3UsDf8X/Di8D/DX/MsSL7rWB3+J/h9cBfpt/GeJFdxy4FTjG/2yXgAcDu/zLEP867w18F/+zvQ/w3bxoEP96Xw18FP8zfQ3w0bzoEP82fw28FP+z/A7w2vzrIP5tjgO3Asf4n+ES8GBgl38dxL/dSwN/xf8MLwP8Nf96iH+fjwa+iv9eHwN8Nf82iH+/7wbei/8e3wO8N/92iH+/48BvAy/Ff62/AV4b2OXfDvEf46WB3waO8V/jEvDSwK38+yD+47w18FP813gb4Kf590P8x/ps4LP4z/U5wGfzHwPxH++3gdfiP8fvAK/NfxzEf7zjwF8DD+I/1jOAlwZ2+Y+D+M/x0sBf8R/nEvDawF/zHwvxn+e9ge/iP8b7AN/NfzzEf67vBt6Lf5/vAd6b/xyI/3x/DbwU/zZ/A7w0/3kQ//keDPw1cIx/nUvASwO38p8H8V/jtYHf4l/ndYDf5j8X4r/ORwNfxYvma4CP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmc4DP5j8f4r/OawO/xYvmY4Cv5j8f4r/OSwN/xYvmZYC/5j8f4r/WZwOfxQv3NcBH818D8V/vpYHjPH+7wF/zXwfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hE++l9BRssDDQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterListAlt;
impl IconShape for MdFilterListAlt {
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
                d: "M0 0h24m0 24H0",
            }
            path {
                d: "M4.25 5.66c.1.13 5.74 7.33 5.74 7.33V19c0 .55.45 1 1.01 1h2.01c.55 0 1.01-.45 1.01-1v-6.02s5.49-7.02 5.75-7.34C20.03 5.32 20 5 20 5c0-.55-.45-1-1.01-1H5.01C4.4 4 4 4.48 4 5c0 .2.06.44.25.66z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/Pb4beB/+4yFedK8N/Bb/fb4beB/+YyFedK8N/Bb/vb4beB/+4yBedK8N/Bb//b4beB/+YyBedK8N/Bb/M3w38D78+yFedK8N/Bb/c3w38D78+yBedK8N/Bb/s3w38D782yFedK8N/Bb/83w38D782yBedK8N/Bb/M3038D786yFedK8N/Bb/c3038D786yBedK8N/Bb/s3038D686BAvutcGfov/+b4beB9eNIgX3WsDv8W/3zOAnwZ2gVu54sHAceCtgQfx7/c6wG/zL0O86F4b+C3+7T4H+G7gVl64lwbeGvgs/u1eB/ht/mWIF91rA7/Fv97PAB8N3Mq/zoOBrwbein+91wF+m38Z4kX32sBv8a/zMcBX8+/z2cBn8a/zOsBv8y9DvOheG/gtXnTvA3w3/zHeG/guXnSvA/w2/zLEi+61gd/iRfM1wEfzwr00cIwrLgF/zQv31cBH8aJ5HeC3+ZchXnSvDfwW/7LfAV6b5+848FHAewMP5jndCnw38DXALs/fbwOvxb/sdYDf5l+GeNG9NvBb/MteB/htntdLAz8FPJgX7lbgbYC/5nm9NvBb/MteB/ht/mWIF91rA7/FC/c7wGvzvF4a+C3gOC+aXeBlgFt5Xr8NvBYv3OsAv82/DPGie23gt3jh3gf4bp7XXwEvzb/ObwOvw/N6b+C7eOFeB/ht/mWIF91rA7/FC3cC2OU5vTfwXfzbvA/w3TynBwNP54V7HeC3+ZchXnSvDfwWL9gzgAfzvH4aeCv+bX4GeGue1y5wjBfsdYDf5l+GeNG9NvBbvGC/A7w2z+vpwIP5t7kVeAjP67eB1+IFex3gt/mXIV50rw38Fi/Y7wCvzfMy/z7ief028Fq8YK8D/Db/MsSL7rWB3+IF+x3gtXle5t/uEnCc5/XbwGvxgr0O8Nv8yxAvutcGfosX7K+Bl+F5/TbwWvzb/A7w2jyv3wZeixfsdYDf5l+GeNG9NvBbvHDieX008FX823wM8NU8L/PCvQ7w2/zLEC+61wZ+ixfubYCf5jkdB24FjvGvcwl4MLDLc3pr4Kd44V4H+G3+ZYgX3WsDv8UL9z3Ae/O83hv4Lv513gf4bp7XdwPvxQv3OsBv8y9DvOheG/gtXrhd4GWAW3le7w18Fy+a9wG+m+d1HHg6cJwX7nWA3+ZfhnjRvTbwW/zLvgd4b56/1wa+G3gQz9/fAB8N/DbP33cD78W/7HWA3+ZfhnjRvTbwW7xoXgf4bV6wlwbeGngwV9wK/DTw17xw7w18F/+y1wF+m38Z4kX32sBv8aLZBV4H+Gv+47038F28cK8D/Db/MsSL7rWB3+JFdyvwNsBf8x/vvYHv4gV7HeC3+ZchXnSvDfwW/zq7wPsAP82/zXFgl+fvvYHv4vl7HeC3+ZchXnSvDfwW/za/DXwO8Nu8aF4b+CzgpYHXAf6a5++9ge/ieb0O8Nv8yxAvutcGfot/n78Gfhv4aeAS8NdccRx4KeC1gbcGXppn2wVeB/hrnr/3Br6L5/Q6wG/zL0O86F4b+C3+e+wCrwP8Nc/fewPfxbO9DvDb/MsQL7rXBn6L/z67wOsAf83z997Ad3HF6wC/zb8M8aJ7beC3+O+1C7wO8Nc8f+8NfBfwOsBv8y9DvOheG/gt/vvtAq8D/DXP33sDtwK/zb8M8aJ7beC3+J9hF3gd4K/590G86F4b+C3+59gFXgf4a/7tEC+6lwb+iv9ZdoHXAf6afxvEv86twIP4n2UXeB3gr/nXQ/zrvDXwU/zPswu8DvDX/Osg/vXeG/hq4Bj/szwDeDD/Ooh/m+PAewMvDTyY/15/DdwKfDewy78O4v83xP9viP/fEP+/If5/4x8BNIjuQYqjV5gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFindInPage;
impl IconShape for MdFindInPage {
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
                d: "M20 19.59V8l-6-6H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c.45 0 .85-.15 1.19-.4l-4.43-4.43c-.8.52-1.74.83-2.76.83-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5c0 1.02-.31 1.96-.83 2.75L20 19.59zM9 13c0 1.66 1.34 3 3 3s3-1.34 3-3-1.34-3-3-3-3 1.34-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9Zr8ZyeAdzKv91xYJd/O8R/nuPAWwGvDbw1cJwX7K+B3wa+B/hrXjTHgd8CXoZ/O8R/vOPARwEfDRznX+9W4LOB7+EFOw78FvDSgPi3Q/zH+izgo4Hj/PvdCrwP8Ns8p+PAbwEvzRXi3w7xH+PBwHcBr81/vK8GPoYrjgO/Bbw0zyb+7RD/fi8N/BZwnP88Pw18DPBTwEvznMS/HeLf56WB3wKO84I9A/ht4KeBXeC3ueLBwIOBlwZeG3gr/m3Evx3i3+7BwF8Bx3n+ngF8NvDdvGiOA58NfBT/OuLfDvFv91vAa/P8fQ3w2cAu/3oPBn4aeCleNOLfDvFv89HAV/H8vQ/w3fz7HAe+Gngv/mXi3w7xr/dg4K+A4zyv9wG+m3+/48BvAS/Nv0z82yH+9T4b+Cye19cAH82/33Hgt4CX5kUj/u0Q/3oXgeM8p2cALw3s8u9zHPgt4KV50Yl/O8S/znsD38Xzeh/gu/n3OQ78FvDS/OuIfzvEv853A+/Fc3oG8GD+fY4DvwW8NP964t8O8a/zV8BL85y+B3hv/u2OA78FvDT/NuLfDvGv89s8r88Gfpt/u68GXpp/u9fm3w7x/xvi/zfE/2+I/98Q/78hXnSvDfwW/zaXgNcG/pp/u98GXovn9DPAW/Nvh3jRvTbwW/zrXQJeG/hr/u2OAxd5Xp8DfDb/dogX3WsDv8W/ziXgtYG/5t/nvYHv4nm9DPDX/NshXnSvDfwWL7pLwGsDf82/39OBB/OcngE8mH8fxIvutYHf4kVzCXht4K/59/to4Kt4Xp8DfDb/PogX3WsDv8W/7BLw2sBf8+/30sBf8fw9BLiVfx/Ei+61gd/iX/YywF/z7/fSwG8Bx3leXwN8NP9+iBfdawO/xb/st4G3AXb5t3tp4LeA4zyvZwAvDezy74d40b028Fu8aP4a+Bjgt/nX+yzgs3nBPgf4bP5jIF50rw38Fv86vw18NfAzvHDHgbcCPht4MP+y9wG+m38/xIvutYHf4jldAm4FXooXbhf4aeBWrvhr4KW54rWB1+Zf732A7+bfB/Gie23gt3i2S8BrA7cCXw28F//13gf4bv7tEC+61wZ+iysuAa8N/DXP9tnAZ/Ef6xnAdwOfxQv2PsB382+DeNG9NvBbwCXgtYG/5nk9GPhq4K349/sc4KuBXeC9ge/iBXsf4Lv510O86F4b+GngtYG/5oV7beCjgdcGjvGiewbw3cBXA7s8p/cGvosX7H2A7+ZfB/Gie2mu+Gv+dd4aeGvgwVzxWlxxCfhrYBf4beCvgd/mhXtv4Lt4wd4H+G5edIj/fd4b+C5esPcBvpsXDeJ/p/cGvosX7H2A7+Zfhvjf672B7+IFex/gu3nhEP+7vTfwXbxg7wN8Ny8Y4n+/9wa+ixfsfYDv5vlD/N/w3sB38fxdAh4M7PK8EP93vDfwXTynS8BrA3/N84f4v+W9ge/iikvAawN/zQuG+L/nvYGvBl4b+GteOMT/TceBXf5liP/fEP+/If5/Q/z/hvj/DfH/G/8ISfSyQZmhmF4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFindReplace;
impl IconShape for MdFindReplace {
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
                d: "M11 6c1.38 0 2.63.56 3.54 1.46L12 10h6V4l-2.05 2.05C14.68 4.78 12.93 4 11 4c-3.53 0-6.43 2.61-6.92 6H6.1c.46-2.28 2.48-4 4.9-4zm5.64 9.14c.66-.9 1.12-1.97 1.28-3.14H15.9c-.46 2.28-2.48 4-4.9 4-1.38 0-2.63-.56-3.54-1.46L10 12H4v6l2.05-2.05C7.32 17.22 9.07 18 11 18c1.55 0 2.98-.51 4.14-1.36L20 21.49 21.49 20l-4.85-4.86z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFingerprint;
impl IconShape for MdFingerprint {
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
                d: "M17.81 4.47c-.08 0-.16-.02-.23-.06C15.66 3.42 14 3 12.01 3c-1.98 0-3.86.47-5.57 1.41-.24.13-.54.04-.68-.2-.13-.24-.04-.55.2-.68C7.82 2.52 9.86 2 12.01 2c2.13 0 3.99.47 6.03 1.52.25.13.34.43.21.67-.09.18-.26.28-.44.28zM3.5 9.72c-.1 0-.2-.03-.29-.09-.23-.16-.28-.47-.12-.7.99-1.4 2.25-2.5 3.75-3.27C9.98 4.04 14 4.03 17.15 5.65c1.5.77 2.76 1.86 3.75 3.25.16.22.11.54-.12.7-.23.16-.54.11-.7-.12-.9-1.26-2.04-2.25-3.39-2.94-2.87-1.47-6.54-1.47-9.4.01-1.36.7-2.5 1.7-3.4 2.96-.08.14-.23.21-.39.21zm6.25 12.07c-.13 0-.26-.05-.35-.15-.87-.87-1.34-1.43-2.01-2.64-.69-1.23-1.05-2.73-1.05-4.34 0-2.97 2.54-5.39 5.66-5.39s5.66 2.42 5.66 5.39c0 .28-.22.5-.5.5s-.5-.22-.5-.5c0-2.42-2.09-4.39-4.66-4.39-2.57 0-4.66 1.97-4.66 4.39 0 1.44.32 2.77.93 3.85.64 1.15 1.08 1.64 1.85 2.42.19.2.19.51 0 .71-.11.1-.24.15-.37.15zm7.17-1.85c-1.19 0-2.24-.3-3.1-.89-1.49-1.01-2.38-2.65-2.38-4.39 0-.28.22-.5.5-.5s.5.22.5.5c0 1.41.72 2.74 1.94 3.56.71.48 1.54.71 2.54.71.24 0 .64-.03 1.04-.1.27-.05.53.13.58.41.05.27-.13.53-.41.58-.57.11-1.07.12-1.21.12zM14.91 22c-.04 0-.09-.01-.13-.02-1.59-.44-2.63-1.03-3.72-2.1-1.4-1.39-2.17-3.24-2.17-5.22 0-1.62 1.38-2.94 3.08-2.94 1.7 0 3.08 1.32 3.08 2.94 0 1.07.93 1.94 2.08 1.94s2.08-.87 2.08-1.94c0-3.77-3.25-6.83-7.25-6.83-2.84 0-5.44 1.58-6.61 4.03-.39.81-.59 1.76-.59 2.8 0 .78.07 2.01.67 3.61.1.26-.03.55-.29.64-.26.1-.55-.04-.64-.29-.49-1.31-.73-2.61-.73-3.96 0-1.2.23-2.29.68-3.24 1.33-2.79 4.28-4.6 7.51-4.6 4.55 0 8.25 3.51 8.25 7.83 0 1.62-1.38 2.94-3.08 2.94s-3.08-1.32-3.08-2.94c0-1.07-.93-1.94-2.08-1.94s-2.08.87-2.08 1.94c0 1.71.66 3.31 1.87 4.51.95.94 1.86 1.46 3.27 1.85.27.07.42.35.35.61-.05.23-.26.38-.47.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDFeuEvAX/Nf46WBY7xwl4C/5oVDvHCvDXwX8GD+Zb8DvDb/NX4beC3+ZbcC7wP8Ns8f4gV7b+C7eNH9DvDa/Nf4beC1eNG9D/DdPC/E83cceDpwnBfd7wCvzX+N3wZeixfdLvAQYJfnhHj+Phr4Kv51fgd4bf5r/DbwWvzrfAzw1TwnxPP33cB78a/zO8Br81/jt4HX4l/ne4D35jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++lgeM8p13gr3lev8V/r9fheb00cJzntAv8Nc8J8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8fy9NHCM53QJ+Gue12/z3+u1eV4vDRzjOV0C/prnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/7wbei3+d3wFem/8avw28Fv86XwN8NM8J8fx9NPBV/Ov8DvDa/Nf4beC1+Nf5GOCreU6I5+84cCtwjBfd7wCvzX+N3wZeixfdJeDBwC7PCfGCvTfwXbzofgd4bf5r/DbwWrzo3gb4aZ4X4oV7beC7gQfxL/sd4LX5r/HbwGvxL3sG8N7Ab/P8IV40Lw0c54XbBf6a/xovDRznhdsF/poXDvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ERyGekG/MDmWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFitScreen;
impl IconShape for MdFitScreen {
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
                d: "M17 4h3c1.1 0 2 .9 2 2v2h-2V6h-3V4zM4 8V6h3V4H4c-1.1 0-2 .9-2 2v2h2zm16 8v2h-3v2h3c1.1 0 2-.9 2-2v-2h-2zM7 18H4v-2H2v2c0 1.1.9 2 2 2h3v-2zM18 8H6v8h12V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlaky;
impl IconShape for MdFlaky {
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
                d: "M14.05,17.58l-0.01,0.01l-2.4-2.4l1.06-1.06l1.35,1.35L16.54,13l1.06,1.06 l-3.54,3.54L14.05,17.58z M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M7.34,6.28l1.41,1.41l1.41-1.41 l1.06,1.06L9.81,8.75l1.41,1.41l-1.06,1.06L8.75,9.81l-1.41,1.41l-1.06-1.06l1.41-1.41L6.28,7.34L7.34,6.28z M12,20 c-2.2,0-4.2-0.9-5.7-2.3L17.7,6.3C19.1,7.8,20,9.8,20,12C20,16.4,16.4,20,12,20z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tb4K+Gngd/ifAfFfyzzbbwO/DXwNsMt/D8R/LfO83gf4bv57IP5rmef1M8Bb898D8V/LPH/ivwfiv5Z5/t4G+Gn+6yH+a5nn73uA9+a/HuK/lnn+bgUewn89xH8t84K9DPDX/NdC/NcyL9jXAB/Nfy3Efy3zgv018DL810K8cA8GdoFd/mOYF+4hwK3810G8cJ8NfBbw18BPA18D7PJvZ164jwG+mv86iBfus4HP4tm+B3hv/u3MC/c7wGvzXwfxwn028Fk82y7wEGCXfxvzLzsB7PJfA/HCfTbwWTyn9wG+m38b8y97H+C7+a+BeOE+G/gsntNfAy/Dv435l30P8N7810C8cJ8NfBbP62WAv+Zfz/zLdoET/NdAvHCfDXwWz+t7gPfmX8+8aN4G+Gn+8yFeuM8GPovnbxf4beCngd8BbuVfZl40XwN8NP/5EC/cZwOfxYvmVuC3gZ8GfgfY5XmZF82twEP4z4d44T4b+Cz+bf4a+Gngt4Hf4QrzonsZ4K/5z4V44T4b+Cz+Y/w08Na86D4H+Gz+cyFeuM8GPov/Hn8NvAz/uRAv3GcDn8V/n4cAt/KfB/HCvTbw0cBb8d/jq4GP4T8P4kX32sBbA68NvBT/tX4a+G3gd4C/5j8O4t/mOPDawFsDrw08iP86u8BPA78N/A5wK/92iP8YDwZeG3hr4LWBY/zXuRX4aeC3gd8BdnnRIf5zvDTw1sBrA6/Ff62/Br4b+B5glxcO8V/jrYHXBl4beCn+a+wC7wP8NC8Y4r/eceCtgdcGXht4EP95doGXAW7l+UP893sw8NbAawOvDRzjP9bHAF/N84f4n+e1gdcGXht4Lf79Pgf4bJ4/xP9sx4HXBl4beG3gpfjX+xjgq3n+EP+7PBh4beC1gdcGHsQLdwl4MLDL84f43+2lgdcGXht4K57TM4C3Bv6aFwzxf8trAw8GbgV+m38Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfGCHQdeiv8b/gbY5XkhXrDXBn6L/xteB/htnhfiBXtt4Lf4v+F1gN/meSFesNcGfov/G14H+G2eF+IFe23gt/i/4XWA3+Z5IV6w1wZ+i/8bXgf4bZ4X4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8ILV+JQWBmkQwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlightLand;
impl IconShape for MdFlightLand {
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
                d: "M2.5,19h19v2h-19V19z M19.34,15.85c0.8,0.21,1.62-0.26,1.84-1.06c0.21-0.8-0.26-1.62-1.06-1.84l-5.31-1.42l-2.76-9.02 L10.12,2v8.28L5.15,8.95L4.22,6.63L2.77,6.24v5.17L19.34,15.85z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xH++1gBPAT/M/H+Lf76WB1wJeG3hrnu19gO/mfzbEv997A9/F8/fZwOfwPxfiP8ZfAy/F8/fdwPvwPxPiP8ZrA7/FC/bdwPvwPw/iP85PA2/FC/bdwMcAu/zPgfiP82Dg6bxwfw28DrDL/wyI/1ifDXwWL9xfA68D7PLfD/Ef6zjw18CDeOH+GngdYJf/Xoj/eO8NfBf/sluBtwH+mv8+iP8cvw28Fv+yXeB1gL/mvwfiP8drA7/Fi2YXeB3gr/mvh/jP893Ae/Gi2QXeBvht/msh/vM8GPhr4BgvuvcBvpv/GK8FXAL+mhcM8Z/rs4HP4l/nfYDv5l/vpYHXAt4aeG2e7VbgY4Cf5nkh/vPdCjyIf533Ab6bF+7BwGsBbw28NnCcF+51gN/mOSH+87018FP863038D4823HgtYC3Bl4beDD/Oj8DvDXPCfFf47eB1+Jf77uBXeC1gZfm32cXOMFzQvzXeGngr/jv9QzgwTwnxH+drwY+iv8+nwN8Ns8J8cK9NPBWwNcAu/z7HAduBY7xX+9vgJfmeSGev+8G3go4zhW7wOsAf82/3msBbw28NvDS/Nf6HeC7ge/m+UM8f58NfBbP66OBr+GFe2ngtYDXBt6a/1p/A/w28NvAbwO7vHCI5+848NfAg3he3w18DLDLFQ8GXgt4beCtgeP813kG8NvAbwO/DdzKvw7iBXtv4Lt4/v4a+G3grYEH81/rZ4DfBn4b+Gv+fRAv3G8Dr8V/r78Bfhr4beC3+Y+FeOFeG/gt/ms9A/hp4LeB3wZ2+c+D+Jd9N/Be/Oe5BPw08NvAbwO38l8H8S87DtwKHOM/zs8Avw38NvDX/PdBvGg+G/gs/u3+Bvht4KeB3+Z/DsSL7lbgQbxongH8NvDTwG8Du/zPhHjRvTXwUzx/l4DfBn4a+G3gVv53QPzr/DbwWlzxO8BPA78N/DX/OyH+dV4aOA78Nv83IP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xAv2HHgpfi/4W+AXZ4X4gV7beC3+L/hdYDf5nkhXrDXBn6L/xteB/htnhfiBXtt4Lf4v+F1gN/meSFesNcGfov/G14H+G2eF+IFe23gt/i/4XWA3+Z5If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP9AOKQSCLMGUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlightTakeoff;
impl IconShape for MdFlightTakeoff {
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
                d: "M2.5,19h19v2h-19V19z M22.07,9.64c-0.21-0.8-1.04-1.28-1.84-1.06L14.92,10l-6.9-6.43L6.09,4.08l4.14,7.17l-4.97,1.33 l-1.97-1.54l-1.45,0.39l2.59,4.49c0,0,7.12-1.9,16.57-4.43C21.81,11.26,22.28,10.44,22.07,9.64z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/FewEcDLw2I5/XbwGvxnH4HeG2el3lerwP8Ns/ptYGfAn4a+BzgVv5liP943wW8N88mntdvA6/Fc/od4LV5XuZ5vQ7w2zyn1wZ+i2d7H+C7eeEQ/7E+G/gsnpN4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtntNDgFt5wRD/sS4Cx3lO4nn9NvBaPKffAV6b52We1+sAv81zem3gt3hO3wO8Ny8Y4j/OawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xXPaBU7wgiH+4xwHXprn9ds8r5cGjvOcdoG/5nm9Ns/rr4FdntNx4KV5Xr/NC4b4/w3x/xvi/zfE/2+If9lr8bz+BtjlOR0HXorn9Tv81zgOvBTP63d4wRD/MvO8Xgf4bZ7TawO/xfMS/zVeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMS/zVeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMS/zVeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMS/zVeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMS/zVeG/gtnpd4wRD/MvO8Xgf4bf73Q/zLzPN6HeC3+d8P8S8zz+t1gN/mfz/Ev8w8r9cBfpv//RD/MvO8Xgf4bf73Q/zLzPN6HeC3eU4vDXwVz+t1+K/x0sBX8bxehxcM8S8zz+t1gN/mOb028Fs8L/Ff47WB3+J5iRcM8S8zz+t1gN/mOb028Fs8L/Ff47WB3+J5iRcM8S8zz+t1gN/mOb028Fs8L/Ff47WB3+J5iRcM8S8zz+t1gN/mOb028Fs8L/Ff47WB3+J5iRcM8S8zz+t1gN/mOb008NU8r9fmv8ZLA1/N83ptXjDEv8w8r9cBfpv//RD/MvO8Xgf4bf73Q/zLzPN6HeC3+d8P8S8zz+t1gN/mfz/Ev8w8r9cBfpv//RD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zejDwdJ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ7T7wCvzQuG+JeZ5/U6wG/zvH4aeCuek3hevw28Fs/pd4DX5nmZ5/U6wG/znF4b+C2e0/sA380LhviXmef1OsBv87yOA7cCx3g28bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzzb9wDvzQuH+JeZ5/U6wG/z/B0Hvht4K64Qz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/BVwCvhr4bP5liH+ZeV6vA/w2L9yDgQcDv83zemngOM9pF/hrntdr87z+GtjlOR0HXhr4bV50iH+ZeV6vA/w2//sh/mXmeb0O8Nv874f4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV4fDfw1/33+Btjl3w/xL9sFjvE/y+sAv82/H+Jf9t3Ae/E/y+sAv82/H+Jf9tLAX/E/y+sAv82/H+JF897Ad/E/x+sAv82/H+JF92Dgs4G3Bo7x3+t1gN/m3w/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwTbFblB5lovuwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlipToBack;
impl IconShape for MdFlipToBack {
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
                d: "M9 7H7v2h2V7zm0 4H7v2h2v-2zm0-8c-1.11 0-2 .9-2 2h2V3zm4 12h-2v2h2v-2zm6-12v2h2c0-1.1-.9-2-2-2zm-6 0h-2v2h2V3zM9 17v-2H7c0 1.1.89 2 2 2zm10-4h2v-2h-2v2zm0-4h2V7h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2zM5 7H3v12c0 1.1.89 2 2 2h12v-2H5V7zm10-2h2V3h-2v2zm0 12h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/FewEcDL81/rV3gp4HPAW7lX4b4j/ddwHvz3+99gO/mhUP8x/ps4LP4n+MhwK28YIj/WBeB4/zP8T3Ae/OCIf7jvDbwW/zPsguc4AVD/Md5beC3eF6vw3+Nlwa+iuclXjDEf5zXBn6L5yX+a7w28Fs8L/GCIf7jvDbwWzwv8V/jtYHf4nmJFwzxH+e1gd/ieYn/Gq8N/BbPS7xgiP84rw38Fs9L/Nd4beC3eF7iBUP8y16L5/U3wC7P6bWB3+J5if8arw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5if8arw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5if8arw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5if8arw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5if8arw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+J5if8arw38Fs9LvGCI/zivDfwWz0v813ht4Ld4XuIFQ/zHeW3gt3he4r/GawO/xfMSLxjiP85rA7/F8xL/NV4b+C2el3jBEP9xXhv4LZ6X+K/x2sBv8bzEC4b4j/PawG/xvMR/jdcGfovnJV4wxL/st3heHwP8Nc/ptYHf4nmJ/xqvDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ/xqvDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ/xqvDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ/xqvDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ/xqvDfwWz0u8YIh/2W/zvD4a+Gue02sDv8XzEv81Xhv4LZ6XeMEQ/3FeG/gtnpf4r/HawG/xvMQLhviP89rAb/G8xH+N1wZ+i+clXjDEf5zXBn6L5yX+a7w28Fs8L/GCIf7jvDbwWzwv8V/jtYHf4nmJFwzxH+e1gd/ieYn/Gq8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/ifx7xgiH+ZeZ5vQ7w2zynBwNP53+W3wFemxcM8S8zz+t1gN/mef008Fb8z/E+wHfzgiH+ZeZ5vQ7w2zyv48CtwDH++30P8N68cIh/mXlerwP8Ns/fceC7gbfiv8cl4KuBz+ZfhviXvTbP66+BXV64BwMP5r/eb/OiQ/z/hvj/DfH/G+L/N8S/7LV4Xn8D7PKcjgMvxfP6HZ7XSwPHeE6XgL/meb0Wz+tvgF3+/RD/sl3gGM/pdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5t8P8S/7buC9eE6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2/36If9lLA3/Fc3od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Lf590O8aN4b+C6e7XWA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+bfD/GiezDw2cBbA28N/DbP6bWB3+J5ief128Br8Zx+B3htnpd5Xq8D/Db/foj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I1svm0E/wk3JAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlipToFront;
impl IconShape for MdFlipToFront {
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
                d: "M3 13h2v-2H3v2zm0 4h2v-2H3v2zm2 4v-2H3c0 1.1.89 2 2 2zM3 9h2V7H3v2zm12 12h2v-2h-2v2zm4-18H9c-1.11 0-2 .9-2 2v10c0 1.1.89 2 2 2h10c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 12H9V5h10v10zm-8 6h2v-2h-2v2zm-4 0h2v-2H7v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGTranslate;
impl IconShape for MdGTranslate {
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
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGavel;
impl IconShape for MdGavel {
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
            rect {
                height: "20",
                transform: "matrix(0.7075 -0.7067 0.7067 0.7075 -5.6854 13.7194)",
                width: "4",
                x: "11.73",
                y: "3.73",
            }
            rect {
                height: "8",
                transform: "matrix(0.707 -0.7072 0.7072 0.707 0.3157 11.246)",
                width: "4",
                x: "11.73",
                y: "1.24",
            }
            rect {
                height: "8",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -8.1722 7.7256)",
                width: "4",
                x: "3.24",
                y: "9.73",
            }
            rect {
                height: "2",
                width: "12",
                x: "1",
                y: "21",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/i3eWngt4Fj/M9wCXht4K/510H827008NvAMf57XQJeG/hr/vUQ/z4vDfw2cIz/HpeA1wb+mn8bxL/fSwO/DRzjv9Yl4LWBv+bfDvEf46WB3waO8V/jEvDawF/z74P4j/PSwG8Dx/jPdQl4beCv+fdD/Md6aeC3gWP857gEvDbw1/zHQPzHe2ngt4Fj/Me6BLw28Nf8x0H853hp4LeBY/zHuAS8NvDX/MdC/Od5aeC3gWP8+1wCXhv4a/7jIf5zvTTw28Ax/m0uAa8N/DX/ORD/+V4a+G3gGP86l4DXBv6a/zyI/xovDfw2cIwXzSXgtYG/5j8X4r/OSwO/DRzjhbsEvDbw1/znQ/zXemngt4FjPH+XgNcG/pr/Goj/ei8N/DZwjOd0CXht4K/5r4P47/HSwG8Dx7jiEvDawF/zXwvx3+elgd/mitcG/pr/eoj/Xi/NFX/Nfw/E/2+I/98Q/78h/n9D/P+G+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82Lw0c53+WXeCv+ddB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E+uNSQbycyZQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGetApp;
impl IconShape for MdGetApp {
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
                d: "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvi3+yzgvYEH88LdCnw38Dk8p9cGfotn+x3gtXm21wZ+ixfd7wCvzb8O4t/mrYGf4l/nbYCf5tleG/gtnu13gNfm2V4b+C1edL8DvDb/Ooh/m+8G3ot/ne8B3ptne23gt3i23wFem2d7beC3eNH9DvDa/Osg/m1+G3gt/nV+B3htnu21gd/i2X4HeG2e7bWB3+JF9zvAa/Ovg/i3+W3gtXi21wF+m3+d1wZ+i2f7HeC1ebbXBn6LZ/sd4LX5j4X4t/lt4LV4ttcBfpt/ndcGfotn+x3gtXm21wZ+i2f7HeC1+Y+F+Lf5beC1eLbXAX6bf53XBn6LZ/sd4LV5ttcGfotn+2vgo3lev8O/HeLf5reB1+LZXgf4bf51Xhv4LZ7td4DX5tleG/gt/mXi3w7xb/PbwGvxbK8D/DbP6bWB3+LZfgd4bZ7ttYHf4tl+B3htnu21gd/iXyb+7RD/Nr8NvBbP9jrAb/OcXhv4LZ7td4DX5tleG/gtnu13gNfm2V4b+C3+ZeLfDvFv89vAa/FsrwP8Ns/ptYHf4tl+B3htnu21gd/i2X4HeG2e7bWB3+LZLgF/zfN6bf7tEP82vw28Fs/2OsBv85xeG/gtnu13gNfm2V4b+C2e7XeA1+bZXhv4LZ7td4DX5j8W4t/mt4HX4l/nd4DX5tleG/gtnu13gNfm2V4b+C2e7XeA1+Y/FuLf5ruB9+Jf53uA9+bZXhv4LZ7td4DX5tleG/gtnu13gNfmPxbi3+atgZ/iX+dtgJ/m2V4b+C2e7XeA1+bZXhv4LZ7td4DX5j8W4t/us4H3Bh7EC/cM4LuBz+Y5vTbwWzzb7wCvzbO9NvBbPNvvAK/NfyzE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EatYaUGDODEAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGif;
impl IconShape for MdGif {
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
            rect {
                height: "6",
                width: "1.5",
                x: "11.5",
                y: "9",
            }
            path {
                d: "M9,9H6c-0.6,0-1,0.5-1,1v4c0,0.5,0.4,1,1,1h3c0.6,0,1-0.5,1-1v-2H8.5v1.5h-2v-3H10V10C10,9.5,9.6,9,9,9z",
            }
            polygon {
                points: "19,10.5 19,9 14.5,9 14.5,15 16,15 16,13 18,13 18,11.5 16,11.5 16,10.5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR4M7AK7/PdA/Pf6bK74bP57IP57PZ0rHsJ/D8R/n/cGvosr3gf4bv7rIf77/Bbw2lzx28Dr8F8P8d/jpYG/4jm9DPDX/NdC/Pf4buC9eE7fA7w3/7UQ//WOAxd5/k4Au/zXQfzX+2jgq3j+Pgb4av7rIP7rPR14MM/frcBD+K+D+K/11sBP8cK9DfDT/NdA/Nf6LeC1eeF+G3gd/msg/us8GHg6L5qHALfynw/xX+e7gffiRfM9wHvznw/xX+M48HTgOC+aXeAhwC7/uRD/NT4a+Cr+dd4H+G7+cyH+azwdeDD/OrcCD+E/F+I/32sDv8W/zesAv81/HsS/3UsDx7jipYHjXPHSwHGueGngOP8+u8Bfc8Uu8NdcsQv8NVdcAv6afz3E8/fawGtxxYOBB3PFSwPH+Z9tF/hrrrgVuJUrfgf4bZ4T4gX7buC9+L/he4D35nkhXrjPBj6L/90+B/hsnj/Ev+y9ge/if6f3Ab6bFwzxonlt4KeBY/zvcAl4a+C3eeEQL7qXBn4aeBD/sz0DeGvgr/mXIf51jgO/DbwU/zP9DfDawC4vGsS/3nHgu4G34n+WnwHeG9jlRYf4t/tu4L34n+F7gPfmXw/x7/PewHfx3+t9gO/m3wbx7/fewFcDx/ivdQn4aOC7+bdD/Md4aeC3gWP817gEvDbw1/z7IP7jPBj4aeCl+M/1N8BbA7fy74f4j/XawG/xn+t1gN/mPwbiP9Z7A9/Ff673Ab6b/xiI/1jfDbwX/7m+B3hv/mMg/mM9HXgw/7luBR7CfwzEf5wHA0/nv8YJYJd/P8R/nLcGfor/Gm8D/DT/foj/OF8NfBT/Nb4G+Gj+/RD/cf4KeGn+a/w18DL8+yH+45j/WuLfD/Ef47WB3+Jf5xLw0Vzx1cAx/nVeB/ht/n0Q/zE+G/gsXnRfA3w2sMsVx4HPBj6KF93nAJ/Nvw/iP8ZvA6/Fv+x3gI8G/prn76WBrwZei3/Z7wCvzb8P4j+GeeEuAR8NfDcvmvcGvho4xgu2C5zg3wfx7/fSwF/xgn0N8NnALv86x4HPBj6KF+xlgL/m3w7x7/fRwFfxvH4H+Gjgr/n3eWngq4HX4nl9DPDV/Nsh/v1+Gngrnu0S8NHAd/Mf672BrwaO8Ww/A7w1/3aIf7+nAw/miq8BPhvY5T/HceCzgY/iiluBh/Bvh/j3eTDwdOBvgPcG/pr/Gq8NfDXwUsBDgFv5t0H8+7w18GDgq/nv8dHArcBP82+D+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CffGQQa2YPe0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGrade;
impl IconShape for MdGrade {
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
                d: "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7jjw0vzv8NfALv8yxP9viP/fEP+/If5/Q7zojgMvxf8OfwPs8i9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxni/zfE/2+I/98Q/78h/n9DvOheGvgq/nf4GOCv+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffSwFfzv8NHA3/Nvwzx/xvi/zfE/2+I/98Q/78hXnSvDfwW/zFeB/ht/mMcB34K+Bjgr/nXQbzoXhv4Lf5jvA7w2/z7HQd+C3hpYBd4HeCvedEhXnSvDfwW/zFeB/ht/n2OA78FvDTPtgu8DvDXvGgQL7rXBn6L/xivA/w2/3bHgd8CXprntQu8DHAr/zLEi+61gd/iP8brAL/NC/Zg4Faev+PAbwEvzfP3PcB786JBvOiOAy/Nf4y/BnZ5/r4LeGvgdYC/5jkdB34LeGmev+8B3psXHeJ/lu8C3psrdoHXAf6aK44DvwW8NM/f9wDvzb8O4n+O7wLem+e0C7wOcCvwW8BL8/x9D/De/Osh/md4beC3eP52gVuBl+b5+x7gvfm3QfzP8d7Ad/Gv8z3Ae/Nvh3jRHQdeiv8YfwPs8rzeG/guXjTfA7w3/z6IF91rA7/Ff4zXAX6b5++9ge/ihfse4L3590O86F4b+C3+Y7wO8Nu8YO8NfBfP3/cA781/DMSL7rWB3+I/xusAv80L997Ad/Gcvgd4b/7jIF50rw38Fv8xXgf4bf5l7w18F1d8D/De/MdCvOheG/gt/mO8DvDbvGjeG3ht4L35j4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwK9V2lB/vvmGAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGrading;
impl IconShape for MdGrading {
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
                d: "M4,7h16v2H4V7z M4,13h16v-2H4V13z M4,17h7v-2H4V17z M4,21h7v-2H4V21z M15.41,18.17L14,16.75l-1.41,1.41L15.41,21L20,16.42 L18.58,15L15.41,18.17z M4,3v2h16V3H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGroupWork;
impl IconShape for MdGroupWork {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM8 17.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5zM9.5 8c0-1.38 1.12-2.5 2.5-2.5s2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5S9.5 9.38 9.5 8zm6.5 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHelp;
impl IconShape for MdHelp {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAER0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Mf6HuCngb8GbuWK1waOA28NvDVwjP844gVD/MvMf4yfAT4auJUX7jjw2cBH8R9DvGCIf5n59/sc4LP513lv4Lv49xMvGOJfZv59vgb4aP5tPhr4Kv59xAuG+JeZf7tnAA/m+Xsw8FLAceBvgL/m+ftt4LX4txMvGOJfZv7t3gb4aZ7TceC7gLfmOf018DbArTyn9wa+i3878YIh/mXm32YXeBngVp7TTwFvzfP318DL8JyOAxf5txMvGOJfZv59vhv4HuC3gfcGvosX7mWAv+Y57QLH+LcRLxjiX2b+Y/w1V7w0L9zrAL/Nc/pt4LX4txEvGOJfZv5rvQ7w2zwn828nXjDEv8z81/kb4KV5Tg8Gns6/nXjBEP8y81/jEvDSwK08p88GPot/O/GCIf5l5j/fJeC1gb/mOb008FvAcf7txAuG+JeZ/1yXgNcG/prndBz4K+DB/PuIFwzxLzP/eS4Brw38Nc/pOPBbwEvz7ydeMMS/zPzneRngr3lOx4HfAl6a/xjiBUP8y8x/js8BPpvn9VPAW/MfR7xgiH+Z+c9xAtjlOX018FH8xxIvGOJfZv7j/Q7w2jynBwNP5z+eeMEQ/zLzH+93gNfmOX008FX8xxMvGOJfZv7jfQ7w2TynzwY+i/944gVD/MvMf7zvBr6b5/TewHvzH0+8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnP8TPAbwO7wFsDb8V/DvGCIf5l5j/e5wCfzXP6aOCr+I8nXjDEv8z8x3oG8GCev1uBB/EfS7xgiH+Z+Y/1O8Br8/z9NvBa/McSLxjiX2b+Y/018DI8f38FvDT/scQLhviXmf94bwP8NM/ptYHf4j+eeMEQ/zLzH28X+GzgZ4Bd4L2AzwaO8x9PvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/swcDT+d/pIcCtvGCIF817A9/F/y7vA3w3LxziRfdg4LOBtwaO8T/TJeCngc8GbuVfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AuetjkEMpnRjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHelpCenter;
impl IconShape for MdHelpCenter {
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
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12.01,18 c-0.7,0-1.26-0.56-1.26-1.26c0-0.71,0.56-1.25,1.26-1.25c0.71,0,1.25,0.54,1.25,1.25C13.25,17.43,12.72,18,12.01,18z M15.02,10.6 c-0.76,1.11-1.48,1.46-1.87,2.17c-0.16,0.29-0.22,0.48-0.22,1.41h-1.82c0-0.49-0.08-1.29,0.31-1.98c0.49-0.87,1.42-1.39,1.96-2.16 c0.57-0.81,0.25-2.33-1.37-2.33c-1.06,0-1.58,0.8-1.8,1.48L8.56,8.49C9.01,7.15,10.22,6,11.99,6c1.48,0,2.49,0.67,3.01,1.52 C15.44,8.24,15.7,9.59,15.02,10.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHelpOutline;
impl IconShape for MdHelpOutline {
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
                d: "M11 18h2v-2h-2v2zm1-16C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14c-2.21 0-4 1.79-4 4h2c0-1.1.9-2 2-2s2 .9 2 2c0 2-3 1.75-3 5h2c0-2.25 3-2.5 3-5 0-2.21-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYG3AX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawE8BPw18DnAr/zLEi+a9ge/i2V4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8WzvQ/w3bxwiH/Zg4Gn85xeB/htntNrA7/F8xLP67eB1+I5/Q7w2jwv87xeB/htntNrA7/Fc3oIcCsvGOJf9t3Ae/GcXgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xXP6HuC9ecEQ/7KLwHGe0+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3hOu8AJXjDEv+y1eV5/DezynI4DL83z+m2e10sDx3lOu8Bf87xem+f118Auz+k48NI8r9/mBUP8/4b4/w3x/xvi/zfEv+y1eF5/A+zyP8tx4KV4Xr/DC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxji/zfE/2+I/98Q/78h/n9D/Mt+i+f1McBf8z/LSwNfxfN6HV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv+y3eV4fDfw1/7O8NPDVPK/X5gVD/P+G+P8N8f8b4v83xP9viH+ZeV6vA/w2/zXMv494wRD/MvO8Xgf4bf5rmH8f8YIh/mXmeb0O8Nv81zD/PuIFQ/zLzPN6HeC3+a9h/n3EC4b4l5nn9TrAb/Nfw/z7iBcM8S97bZ7XXwO7/Ncw/7LvAb6b5++3ecEQ//OZF837AN/Nvw7ifz7zonsf4Lt50SH+5zP/Ou8DfDcvGsT/fOZ5fQ/wXrxg7wN8N/8yxL/stXhefwPs8pyOAy/F8/odntdLA8d4TpeAv+Z5mef1OsCDge/iBXsf4Lt54RD/sl3gGM/pdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDfBt4b+C5esPcBvpsXDPEv+27gvXhOrwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Nle8N/BdPH/fA7w3LxjiX/Zg4Ok8p9cBfpvn9NrAb/G8xPP6beC1eE6/A7w2z8s8r9cBfptne2/gu3hO3wO8Ny8c4kXz3sB38WyvA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m9ge/iiu8B3pt/GeJF92Dgs4G3Bt4a+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e13sDrw28Ny8axP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RG9qrQYETkGAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHighlightAlt;
impl IconShape for MdHighlightAlt {
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
                d: "M17 5h-2V3h2v2zm-2 16h2v-2.59L19.59 21 21 19.59 18.41 17H21v-2h-6v6zm4-12h2V7h-2v2zm0 4h2v-2h-2v2zm-8 8h2v-2h-2v2zM7 5h2V3H7v2zM3 17h2v-2H3v2zm2 4v-2H3c0 1.1.9 2 2 2zM19 3v2h2c0-1.1-.9-2-2-2zm-8 2h2V3h-2v2zM3 9h2V7H3v2zm4 12h2v-2H7v2zm-4-8h2v-2H3v2zm0-8h2V3c-1.1 0-2 .9-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHighlightOff;
impl IconShape for MdHighlightOff {
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
                d: "M14.59 8L12 10.59 9.41 8 8 9.41 10.59 12 8 14.59 9.41 16 12 13.41 14.59 16 16 14.59 13.41 12 16 9.41 14.59 8zM12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHistory;
impl IconShape for MdHistory {
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
                d: "M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHistoryToggleOff;
impl IconShape for MdHistoryToggleOff {
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
                d: "M15.1,19.37l1,1.74c-0.96,0.44-2.01,0.73-3.1,0.84v-2.02C13.74,19.84,14.44,19.65,15.1,19.37z M4.07,13H2.05 c0.11,1.1,0.4,2.14,0.84,3.1l1.74-1C4.35,14.44,4.16,13.74,4.07,13z M15.1,4.63l1-1.74C15.14,2.45,14.1,2.16,13,2.05v2.02 C13.74,4.16,14.44,4.35,15.1,4.63z M19.93,11h2.02c-0.11-1.1-0.4-2.14-0.84-3.1l-1.74,1C19.65,9.56,19.84,10.26,19.93,11z M8.9,19.37l-1,1.74c0.96,0.44,2.01,0.73,3.1,0.84v-2.02C10.26,19.84,9.56,19.65,8.9,19.37z M11,4.07V2.05 c-1.1,0.11-2.14,0.4-3.1,0.84l1,1.74C9.56,4.35,10.26,4.16,11,4.07z M18.36,7.17l1.74-1.01c-0.63-0.87-1.4-1.64-2.27-2.27 l-1.01,1.74C17.41,6.08,17.92,6.59,18.36,7.17z M4.63,8.9l-1.74-1C2.45,8.86,2.16,9.9,2.05,11h2.02C4.16,10.26,4.35,9.56,4.63,8.9z M19.93,13c-0.09,0.74-0.28,1.44-0.56,2.1l1.74,1c0.44-0.96,0.73-2.01,0.84-3.1H19.93z M16.83,18.36l1.01,1.74 c0.87-0.63,1.64-1.4,2.27-2.27l-1.74-1.01C17.92,17.41,17.41,17.92,16.83,18.36z M7.17,5.64L6.17,3.89 C5.29,4.53,4.53,5.29,3.9,6.17l1.74,1.01C6.08,6.59,6.59,6.08,7.17,5.64z M5.64,16.83L3.9,17.83c0.63,0.87,1.4,1.64,2.27,2.27 l1.01-1.74C6.59,17.92,6.08,17.41,5.64,16.83z M13,7h-2v5.41l4.29,4.29l1.41-1.41L13,11.59V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv95x4KeA1+Y5/TbwNsAu/3UQ/7WOA78FvDTP318DrwPs8l8D8V/npYHvAl6aF+6vgfcB/pr/fIj/Gi8N/BZwnBfNLvA6wF/znwvxn++1gZ8CjvOvswu8DfDb/OdB/Od6b+C7+Pd5H+C7+c+B+M/z3sB38R/jfYDv5j8e4j/HdwHvzX+s7wbeh/9YiP943wW8N/85vht4H/7jIP7jHAd+Cnht/nP9NvA2wC7/foj/GMeB3wJemv8afw28DrDLvw/i3++lge8CXpr/Wn8NvA/w1/zbIf59Xhr4LeA4/z12gdcB/pp/G8S/3WsDPwUc57/XLvA2wG/zr4f4t3lv4Lv4n+V9gO/mXwfxr/fewHfxP9P7AN/Niw7xr/NdwHvzP9t3A+/Diwbxovsu4L353+G7gffhX4b4lx0Hfgp4bf53+W3gbYBdXjDEC3cc+C3gpfnf6a+B1wF2ef4QL9hLA98FvDT/u/018D7AX/O8EM/fSwO/BRzn/4Zd4HWAv+Y5If5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8W/zt8AH82L5quBl+Jf53eA1+ZfB/Fv89vAa/Gv8zvAa/Oi+W3gtfjX+R3gtfnXQfzb/DbwWvzr/A7w2rxofht4Lf51fgd4bf51EP82vw28Fv86vwO8Ni+a3wZei3+d3wFem38dxL/NbwOvxb/O7wCvzYvmt4HX4l/nd4DX5l8H8W/z28Br8a/zO8Br86L5beC1+Nf5HeC1+ddB/Nv8NvBa/Ov8DvDavGh+G3gt/nV+B3ht/nUQ/za/DbwW/zq/A7w2L5rfBl6Lf53fAV6bfx3Ev81vA6/Fv87vAK/Ni+a3gdfiX+d3gNfmXwfxb/PbwGvxr/M7wGvzovlt4LX41/kd4LX510H82/w28Fr86/wO8Nq8aH4beC3+dX4HeG3+dRD/Nr8NvBb/Or8DvDYvmt8GXot/nd8BXpt/HcS/zW8Dr8W/zu8Ar82L5reB1+Jf53eA1+ZfB/Fv89vAa/Gv8zvAa/Oi+W3gtfjX+R3gtfnXQfzb/DbwWvzr/A7w2rxofht4Lf51fgd4bf51EP82vw28Fv86vwO8Ni+a3wZei3+d3wFem38dxL/NSwPH+dfZBf6aF81LA8f519kF/pp/HcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R068i0FXzTvnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHome;
impl IconShape for MdHome {
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
                d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b473Mc+Cqu+Bhgl/96iP8eLw18F/DSXPHXwPsAf81/LcR/vfcGvgo4znPaBT4G+G7+6yD+6xwHvgp4b1647wY+BtjlPx/iv8ZLA98FvDQvmr8G3gf4a/5zIf7zvTfwVcBx/nV2gY8Bvpv/PIj/PMeBrwLem3+f7wY+BtjlPx7iP8dLA98FvDT/Mf4aeB/gr/mPhfiP997AVwHH+Y+1C3wM8N38x0H8xzkOfBXw3vzn+m7gY4Bd/v0Q/zFeGvgu4KX5r/HXwPsAf82/D+Lf772BrwKO819rF/gY4Lv5t0P82x0Hvgp4b/57fTfwMcAu/3qIf5uXBr4LeGn+Z/hr4H2Av+ZfB/Gv997AVwHH+Z9lF/gY4Lt50SFedMeBrwLem//Zvhv4GGCXfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i3+91+Hf57f413sd4Lf5lyFedK8N/Bb/euLfx/zrvQ7w2/zLEC+61wZ+i3898e9j/vVeB/ht/mWIF91rA7/Fv5749zH/eq8D/Db/MsSL7rWB3+JfT/z7mH+91wF+m38Z4kX32sBv8a8n/n3Mv97rAL/Nvwzxontt4Lf41xP/PuZf73WA3+ZfhnjRvTbwW/zriX8f86/3OsBv8y9DvOheG/gt/vXEv4/513sd4Lf5lyFedK8N/Bb/euLfx/zrvQ7w2/zLEC+61wZ+i3898e9j/vVeB/ht/mWIF91rA7/Fv5749zH/eq8D/Db/MsSL7rWB3+JfT/z7mH+91wF+m38Z4kX32sBv8a8n/n3Mv97rAL/Nvwzxontt4Lf41xP/PuZf73WA3+ZfhnjRvTbwW/zriX8f86/3OsBv8y9DvOheG/gt/vXEv4/513sd4Lf5lyFedK8N/Bb/euLfx/zrvQ7w2/zLEC+61wZ+i3898e9j/vVeB/ht/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNUkYJBwdSJlgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHomeFilled;
impl IconShape for MdHomeFilled {
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
                d: "M12 3L4 9v12h5v-7h6v7h5V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACAUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7Lf4n+31+EFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/Mt+m//dXpsXDPH/G+L/N8T/b4j/3xD/vyH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mWvzf9uv80Lhvj/DfH/G+L/N8T/b4h/2Wvxv9vv8IIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mW/zf9ur80Lhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4Rz9gHUFn1H0hAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHorizontalSplit;
impl IconShape for MdHorizontalSplit {
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
                d: "M3 19h18v-6H3v6zm0-8h18V9H3v2zm0-6v2h18V5H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHourglassDisabled;
impl IconShape for MdHourglassDisabled {
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
                points: "8,4 16,4 16,7.5 13.16,10.34 14.41,11.59 18,8.01 17.99,8 18,8 18,2 6,2 6,3.17 8,5.17",
            }
            path {
                d: "M2.1,2.1L0.69,3.51l8.9,8.9L6,16l0.01,0.01H6V22h12v-1.17l2.49,2.49l1.41-1.41L2.1,2.1z M16,20H8v-3.5l2.84-2.84L16,18.83 V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/Ff6/X4d8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9nntffALv8xzoOvBTPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r78GXgfY5T/GceC3gJfmeYl/O8S/318DL8Xz+mvgdYBd/n2OA78FvDTP62+Al+bfDvHvdxz4beCleF5/DbwOsMu/zXHgt4CX5nn9DfDawC7/doj/GMeB3wZeiuf118DrALv86xwHfgt4aZ7X3wCvDezy74P4j3Mc+G3gpXhefw28DrDLi+Y48FvAS/O8/gZ4bWCXfz/Ef6zjwG8DL8Xz+mvgdYBdXrjjwG8BL83z+hvgtYFd/mMg/uMdB34beCme118DrwPs8vwdB34LeGme198Arw3s8h8H8Z/jOPDbwEvxvP4aeB1gl+d0HPgt4KV5Xn8DvDawy38sxH+e48BvAy/F8/pr4HWAXa44DvwW8NI8r78BXhvY5T8e4j/XceC3gZfief018Dpc8VvAS/O8/gZ4bWCX/xyI/3zHgd8GXorn9ddc8dI8r78BXhvY5T8P4r/GceC3gZfiRfM3wGsDu/znQvzXOA78FvDSvGj+GngdYJf/XIj/fMeB3wJemuf1N1zxUjyvvwZeB9jlPw/iP9dx4LeAl+Z5/Q3w2lzx28BL8bz+GngdYJf/HIj/PMeB3wJemuf1N8BrA7tccRz4beCleF5/DbwOsMt/PMR/juPAbwEvzfP6G+C1gV2e03Hgt4GX4nn9NfA6wC7/sRD/8Y4DvwW8NM/rb4DXBnZ5/o4Dvw28FM/rr4HXAXb5j4P4j3Uc+C3gpXlefwO8NrDLC3cc+G3gpXhefw28DrDLfwzEf5zjwG8BL83z+hvgtYFdXjTHgd8GXorn9dfA6wC7/Psh/mMcB34LeGme198Arw3s8q9zHPht4KV4Xn8NvA6wy78P4t/vOPBbwEvzvP4GeG1gl3+b48BvAy/F8/pr4HWAXf7tEP9+fwW8NM/rb4DXBnb59zkO/DbwUjyvvwZehn87xL+feV5/A7w2sMt/jOPAbwMvxfMS/3aIfz/zvF4H+G3+Y7028Fs8L/Fvh/j3M8/rdYDf5j/WawO/xfMS/3aIfz/zvF4H+G3+Y7028Fs8L/Fvh/j3M8/rdYDf5j/WawO/xfMS/3aIfz/zvF4H+G3+Y7028Fs8L/Fvh/j3M8/rdYDf5j/WawO/xfMS/3aIfz/zvF4H+G3+Y7028Fs8L/Fvh/j3M8/rdYDf5j/WawO/xfMS/3aIfz/zvF4H+G3+Y7028Fs8L/Fvh/j3M8/rr4Fd/mMdB16a5yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fr/Nf6/X5t8O8f8b4v83xP9viP/fEP+/8Y8KssRBji3diQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHourglassEmpty;
impl IconShape for MdHourglassEmpty {
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
                d: "M6 2v6h.01L6 8.01 10 12l-4 4 .01.01H6V22h12v-5.99h-.01L18 16l-4-4 4-3.99-.01-.01H18V2H6zm10 14.5V20H8v-3.5l4-4 4 4zm-4-5l-4-4V4h8v3.5l-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/Ff6/X4d8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7/TXwUvz3+Bvgpfm3Q/z7HQd+G3gp/mv9DfDawC7/doj/GMeB3wZeiv8afwO8NrDLvw/iP85x4LeBl+I/198Arw3s8u+H+I91HPht4KX4z/E3wGsDu/zHQPzHOw78NvBS/Mf6G+C1gV3+4yD+cxwHfht4Kf5j/A3w2sAu/7EQ/3mOA78NvBT/Pn8DvDawy388xH+u48BvAy/Fv83fAK8N7PKfA/Gf7zjw28BL8a/zN8BrA7v850H81zgO/DbwUrxo/gZ4bWCX/1yI/xrHgd8CXpoXzV8DrwPs8p8L8Z/vOPBbwEvzr/PXwOsAu/znQfznOg78FvDS/Nv8NfA6wC7/ORD/eY4DvwW8NP8+fw28DrDLfzzEf47jwG8BL81/jL8GXgfY5T8W4j/eceC3gJfmP9ZfA68D7PIfB/Ef6zjwW8BL85/jr4HXAXb5j4H4j3Mc+C3gpfnP9dfA6wC7/Psh/mMcB34LeGn+a/w18DrALv8+iH+/48BvAS/Nf62/Bl4H2OXfDvHv91fAS/Pf46+Bl+HfDvHvZ/57iX87xL+f+e8l/u0Q/37mv5f4t0P8+5n/XuLfDvHvZ/57iX87xL+f+e8l/u0Q/37mv5f4t0P8+5n/XuLfDvHvZ/57iX87xL+f+e8l/u0Q/37mv5f4t0P8+5n/XuLfDvHvZ/57iX87xL+f+e8l/u0Q/37mv5f4t0P8+/02/71em387xP9viP/fEP+/If5/Q/z/xj8CIBBgQcv3BDoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHourglassFull;
impl IconShape for MdHourglassFull {
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
                d: "M6 2v6h.01L6 8.01 10 12l-4 4 .01.01H6V22h12v-5.99h-.01L18 16l-4-4 4-3.99-.01-.01H18V2H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEN0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hni21wZ+i2f7HeC1eV7mOYkrXhv4LV50vwO8Nlf8NvBavOheB/htrjAvmt8GPgf4bZ4N8WyvDfwWz/Y7wGvzvMxzEle8NvBbvOh+B3htrvht4LV40b0O8NtcYf513gb4aa5APNtrA7/Fs/0O8No8L/OcxBWvDfwWL7rfAV6bK34beC1edK8D/DZXmH+dXeAEVyCe7bWB3+LZfgd4bZ6XeU7iitcGfosX3e8Ar80Vvw28Fi+61wF+myvMv97rAL8NIJ7ttYHf4tl+B3htnpd5TuL5e23gt3i23wFemxfNbwOvxbO9DvDbPH/mOYnn9dvAa/FsrwP8NoB4ttcGfotn2wX+muf12jwn8fy9NvBbPNvvAK/Ni+a3gdfi2V4H+G2eP/OcxPP6beC1eLbXAX4bQDzbawO/xb+eeP5eG/gtnu13gNfmRfPbwGvxbK8D/DbPn3lOr83z+mrgpXm21wF+G0A822sDv8W/nnj+Xhv4LZ7td4DX5kXz28Br8WyvA/w2z5/513sIcCuAeLbXBn6Lfz3x/L028Fs82+8Ar82L5reB1+LZXgf4bZ4/86/zN8BLcwXi2V4b+C2e7W+Aj+Z5/RbPSTx/rw38Fs/2O8Br86L5beC1eLbXAX6b58+86J4BvDZwK1cgnu21gd/i2X4HeG2el3lO4vl7beC3eLbfAV6bF81vA6/Fs70O8Ns8f+Y5/Q7P61bgr4HvBnZ5NsSzvTbwWzzb7wCvzfMyz0k8f68N/BbP9jvAa/Oi+W3gtXi21wF+m+fPPCfxokM822sDv8Wz/Q7w2jwv85zE8/fawG/xbL8DvDYvmt8GXotnex3gt3n+zHMSLzrEs7028Fs82+8Ar83zMs9JPH+vDfwWz/Y7wGvzovlt4LV4ttcBfpvnzzwn8aJDPNtrA7/Fs/0O8No8L/OcxPP32sBv8Wy/A7w2L5rfBl6LZ3sd4Ld5/sxzEi86xLO9NvBbPNvvAK/N8zLPSTx/rw38Fs/2O8Br86L5beC1eLbXAX6b5888J/GiQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BUQSNQf2nCC8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHttp;
impl IconShape for MdHttp {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z2vxnH6H/3qI/zoPBt4LeGvgpXn+/hr4aeB7gFv5z4f4z3cc+CrgvfnX+Wrgc4Bd/vMg/nO9NPBbwHH+bXaB1wH+mv8ciP887w18F/8x3gf4bv7jIf5zvDTwV7xwzwBu5YoHAw/ihXsZ4K/5j4X4j3cceDpwnOd1Cfhq4LuBW3lODwbeG/ho4BjPaxd4CLDLfxzEf7zvBt6L5/U3wFsDt/LCPRj4aeCleF5fA3w0/3EQ/7EeDDyd5/U3wGsDu7xojgO/DbwUz+shwK38x0D8x/ps4LN4TpeAlwZu5V/nwcDTeV6fA3w2/zEQ/7H+CnhpntPnAJ/Nv81nA5/Fc/pr4GX4j4H4j2We10OAW/m3eWngr3he4j8G4j/OawO/xXN6BvBg/n12gWM8p9cBfpt/P8R/nNcGfovn9DvAa/Pv89vAa/GcXgf4bf79EP9xXhv4LZ7T7wCvzb/PbwOvxXN6HeC3+fdD/Md5beC3eE6/A7w2/z6/DbwWz+l1gN/m3w/xH+e1gd/iOf0O8Nr8+/w28Fo8p9cBfpt/P8R/nNcGfovn9DvAa/Pv89vAa/GcXgf4bf79EP9xXhv4LZ7T7wCvzb/PbwOvxXN6HeC3+fdD/NscB94LeGngwVxxHHhpntMu8Nf8+7w0cJzn9NfALlfcCvw18D3ALv86iH+99wa+CjjO/yy7wMcA382LDvGv89rAb/E/2+sAv82LBvGv83TgwfzPdivwEF40iBfdSwN/xf8OLwP8Nf8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/iP84zgFu54sHAg/iP8zrAb/MvQ7zoXhv4Lf79LgHvDfw0z+mtge8GjvHv9zrAb/MvQ7zoXhv4Lf59LgEPBnZ5/o4DtwLH+Pd5HeC3+ZchXnSvDfwW/z5vA/w0L9xbAz/Fv8/rAL/Nvwzxontt4Lf4t3sG8GBeNLcCD+Lf7nWA3+ZfhnjRvTbwW/zb/Q7w2rxofht4Lf7tXgf4bf5liBfdawO/xb/d7wCvzYvmt4HX4t/udYDf5l+GeNG9NvBb/NvdCjyEF83TgQfzb/c6wG/zL0O86F4b+C3+fd4G+GleuLcGfop/n9cBfpt/GeJF99rAb/Hvsws8BNjl+Xtp4LeA4/z7vA7w2/zLEC+61wZ+i3+/XeB9gJ/mOb018F3Acf79Xgf4bf5liBfdawO/xX+cW4FbueLBwIP5j/M6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhr4K/53eBngr/mXIf51bgUexP9szwAezIsG8a/z2sBv8T/b6wC/zYsG8a/33sBXA8f4n+US8N7AT/OiQ/zbHAfeG3hp4MH897oV+Gvgu4Fd/nUQ/78h/n9D/P+G+P8N8f8b/whc+rxBZ7l5eAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHttps;
impl IconShape for MdHttps {
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
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImportantDevices;
impl IconShape for MdImportantDevices {
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
                d: "M23 11.01L18 11c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-9c0-.55-.45-.99-1-.99zM23 20h-5v-7h5v7zM20 2H2C.89 2 0 2.89 0 4v12c0 1.1.89 2 2 2h7v2H7v2h8v-2h-2v-2h2v-2H2V4h18v5h2V4c0-1.11-.9-2-2-2zm-8.03 7L11 6l-.97 3H7l2.47 1.76-.94 2.91 2.47-1.8 2.47 1.8-.94-2.91L15 9h-3.03z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/zh/A+zynI4DL8V/nF3gIcAuLxjiX/bZwGfxH+t1gN/mOb028Fv8x/oc4LN5wRAv3HHg6cBx/mO9DvDbPKfXBn6L/1i7wEOAXZ4/xAv32cBn8R/vdYDf5jm9NvBb/Mf7HOCzef4QL9zTgQfzH+91gN/mOb028Fv8x7sVeAjPH+IFe2vgp/jP8TrAb/OcXhv4Lf5zvA7w2zwvxAv21cBH8X/D1wAfzfNCvGB/Bbw0/zf8NfAyPC/E8/dg4On833IC2OU5IZ6/1wZ+i/9bXgf4bZ4T4vn7aOCr+L/lY4Cv5jkhnr/PBj6L/zwfA/w1z+mlga/iP8/nAJ/Nc0I8f58NfBb/eV4H+G2e02sDv8V/ns8BPpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88PwO8Nc8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+BnhrnhPi+fts4LP4z/M6wG/znF4b+C3+83wO8Nk8J8Tz99nAZ/Gf53WA3+Y5vTbwW/zn+Rzgs3lOiOfvo4Gv4j/P6wC/zXN6beC3+M/zMcBX85wQz99rA7/Ff57XAX6b5/TawG/xn+d1gN/mOSGevwcDT+c/z+sAv81zem3gt/jPcwLY5TkhXrC/Bl6K/xyvA/w2z+m1gd/iP8ffAC/N80K8YF8NfBT/OV4H+G2e02sDv8V/jq8BPprnhXjBXhv4Lf5zvA7w2zyn1wZ+i/8crwP8Ns8L8cLdCjyI/3ivA/w2z+m1gd/iP94zgAfz/CFeuM8GPov/eK8D/DbP6bWB3+I/3ucAn83zh3jhjgO3Asf4j/U6wG/znF4b+C3+Y10CHgzs8vwh/mWfDXwW/7FeB/htntNrA7/Ff6zPAT6bFwzxLzsO3Aoc4z/OXwO7PKfjwEvzH+cS8GBglxcM8aJ5b+C7+N/lbYCf5oVDvOh+Gngr/nf4GeCt+ZchXnTHgb8GHsT/bM8AXhrY5V+G+Nd5aeC3gWP8z3QJeG3gr3nRIP71Xhr4beAY/7NcAl4b+GtedIh/m5cGfhs4xv8Ml4DXBv6afx3Ev91LAz8NPIj/Xs8A3hr4a/71EP8+x4HvBt6K/x4/A7w3sMu/DeI/xnsDXw0c47/GJeC9gZ/m3wfxH+c48NHARwPH+M9xCfhq4KuBXf79EP/xjgMfDbw38CD+YzwD+G7gq4Fd/uMg/nO9NvDWwGsDL8W/zt8Avw38NPDb/OdA/Nc5Drw08NLAca54aa74a67YBf4a+Gtgl/98iP/fEP+/If5/Q/z/hvj/jX8E42G+QZP5j7oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInfo;
impl IconShape for MdInfo {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInfoOutline;
impl IconShape for MdInfoOutline {
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
                d: "M11,7h2v2h-2V7z M11,11h2v6h-2V11z M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20 c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4aeC/grfm/4aeB7wF+mueEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf7jvTfw3fzXMM/rdYDfBhDP9trAb/G8xH+83waeDrwP//nM83od4LcBxLO9NvBbPC/xH++3gdcCvht4H/5zmef1OsBvA4j/Hr8NvBZXfDfwPvznMc/rdYDfBhD/PX4beC2e7buB9+E/h3lerwP8NoD47/HbwGvxnL4beB/+45nn9TrAbwOI/x6/DbwWz+u7gffhP5Z5Xq8D/DaA+O/x28Br8fx9N/A+/Mcxz+t1gN8GEM/20sBX8V/jpYHjvGDfDbwP/zHM83od4LcBxLO9NvBb/M/x3cD78O9nntfrAL8NIJ7ttYHf4n+W7wbeh38f87xeB/htAPFsrw38Fv+zvA/w3fz7mOf1OsBvA4hne23gt/if432A7+bfzzyv1wF+G0A820sDX81/jZcGjvGCvQ/w3fzHMM/rdYDfBhD/PX4beC2ev/cBvpv/OOZ5vQ7w2wDiv8dvA6/F83of4Lv5j2We1+sAvw0g/nv8NvBaPKf3Ab6b/3jmeb0O8NsA4r/HbwOvxbO9D/Dd/Ocwz+t1gN8GEP89fht4La54H+C7+c9jntfrAL8NIJ7ttYHf4nmJ/3i/DbwW8D7Ad/Ofyzyv1wF+G0A822sDv8XzEv/xfhv4buC7+c9nntfrAL8NIJ7ttYHf4nmJ/3gvDfw1/zXM83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nm9Nv+7/TbP63WA3wYQz/bawG/x/8PrAL8NIJ7TLnCM/9suAce5AvGcPhv4LP5v+xzgs7kC8by+G3gv/m/6HuC9eTbE8/fewFsDb8X/DT8D/DTw3TwnxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RjoWiQXcdIOgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInput;
impl IconShape for MdInput {
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
                d: "M21 3.01H3c-1.1 0-2 .9-2 2V9h2V4.99h18v14.03H3V15H1v4.01c0 1.1.9 1.98 2 1.98h18c1.1 0 2-.88 2-1.98v-14c0-1.11-.9-2-2-2zM11 16l4-4-4-4v3H1v2h10v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8NHAcZ7TLvDVwNcAu/zXQPzXeWngp4AH88LdCrwN8Nf850P813gw8FfAcV40u8DLALfynwvxX+O3gNfmX+e3gdfhPxfiP99bAz/F87oEfDZXfDZwjOf1NsBP858H8aJ7aeCjgLcGjvPv9zrAb3PFawO/xb/fLvDTwOcAt/IvQ7xo3hv4KuA4/3HEczL/sd4H+G5eOMS/7MHA0/mP9zHAV3PFRwNfxX+8hwC38oIh/mXfDbwX/zn+mitemv8c3wO8Ny8Y4l92ETjO/067wAleMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz7fQ/wXvzrfA/wXvz7iRcM8S8z/z7vA3w38N7Ad/GieR/gu4H3Br6Lfx/xgiH+Zebf7n2A7+bZ3hv4Ll649wG+m2d7b+C7+LcTLxjiX2b+bd4H+G6e13sD38Xz9z7Ad/O83hv4Lv5txAuG+JeZf733Ab6b5+848FfAg3lOtwIvA+zy/L038F3864kXDPEvM/867wN8N8/fceC3gJfm+ftr4HWAXZ6/9wa+i38d8YIh/mXmRfc+wHfz/B0Hfgt4aV64vwZeB9jl+Xtv4Lt40YkXDPEvMy+a9wG+m+fvOPBbwEvzovlr4HWAXZ6/9wa+ixeNeMEQ/zLzL3sf4Lt5/o4DvwW8NP86fw28DrDL8/fewHfxLxMvGOJfZl649wG+m+fvOPBbwEvzb/PXwOsAuzx/7w18Fy+ceMEQ/zLzwn038D48f8eB3wZein+bvwFeG9jl+fsu4L154cQLhviXmX/ZdwPvw/N3HPht4KX41/kb4LWBXZ6/7wLem3+ZeMEQ/zLzovlu4H14/o4Dvw28FC+avwFeG9jl+fsu4L150YgXDPEvMy+67wbeh+fvOPDbwEvxwv0N8NrALs/fdwHvzYtOvGCIf5n51/lu4H14/o4Dvw28FM/f3wCvDezy/H0X8N7864gXDPEvM/963w28D8/fceCvgQfxnJ4BvDSwy/P3XcB7868nXjDEv8z823w38D48r/cGvovn732A7+Z5fRfw3vzbiBcM8S8z/3bfDbwPz/bewHfxwr0P8N0823cB782/nXjBEP8y8+/z3cD7AO8NfBcvmvcBvhv4LuC9+fcRLxjiX2b+/X4beG3+dX4beG3+/cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8yx4MPJ3/nR4C3MoLhnjRvDfw1cAx/vd4H+C7eeEQL7qXBj4aeGvgGP8zXQJ+Gvhs4Fb+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IxMIrkGCxtpAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdIntegrationInstructions;
impl IconShape for MdIntegrationInstructions {
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
            circle {
                cx: "12",
                cy: "3.5",
                r: ".75",
            }
            circle {
                cx: "12",
                cy: "3.5",
                r: ".75",
            }
            circle {
                cx: "12",
                cy: "3.5",
                r: ".75",
            }
            path {
                d: "M19,3h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C4.86,3,4.73,3.01,4.6,3.04C4.21,3.12,3.86,3.32,3.59,3.59 c-0.18,0.18-0.33,0.4-0.43,0.64C3.06,4.46,3,4.72,3,5v14c0,0.27,0.06,0.54,0.16,0.78c0.1,0.24,0.25,0.45,0.43,0.64 c0.27,0.27,0.62,0.47,1.01,0.55C4.73,20.99,4.86,21,5,21h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M11,14.17l-1.41,1.42L6,12 l3.59-3.59L11,9.83L8.83,12L11,14.17z M12,4.25c-0.41,0-0.75-0.34-0.75-0.75S11.59,2.75,12,2.75s0.75,0.34,0.75,0.75 S12.41,4.25,12,4.25z M14.41,15.59L13,14.17L15.17,12L13,9.83l1.41-1.42L18,12L14.41,15.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e703V3w3/z0Q/33eG/gurngf4Lv5r4f47/HewHfxnN4H+G7+ayH+67038F08f+8DfDf/dRD/td4b+C5euPcBvpv/Goj/Ou8NfBcvmvcBvpv/fIj/Gu8NfBf/Ou8DfDf/uRD/+d4b+C6ev0vAMV6w9wG+m/88iP9c7w18F8/fJeC1gb/ihXsf4Lv5z4H4z/PewHfx/F0CXhv4a8D8y94H+G7+4yH+c7w38F08f5eA1wb+mivMi+Z9gO/mPxbiP957A9/F83cJeG3gr3k286J7H+C7+Y+D+I/13sB38fxdAl4b+Guek/nXeR/gu/mPgfiP897Ad/H8XQJeG/hrnpf513sf4Lv590P8x3hv4Lt4/i4Brw38Nc+f+bd5H+C7+fdB/Pu9N/BdPH+XgNcG/poXzPzbvQ/w3fzbIf59Xhr4K56/S8BrA3/NC2f+fV4G+Gv+bRD/dseBpwPHeV6XgNcG/pp/mfn32QUeAuzyr4f4t/sr4KV5XpeA1wb+mheN+ff7a+Bl+NdD/Nu8N/BdPH8vA/w1LzrzH+N9gO/mXwfxb/N04ME8r/cBvpt/HfMf41bgIfzrIP713hv4Lp7X9wDvzb+e+Y/zPsB386JD/Os9HXgwz+kS8NLArfzrmf84twIP4UWH+Nd5aeCveF6fA3w2/zbmP9ZDgFt50SD+dT4a+Cqe1wlgl38b8x/rY4Cv5kWD+Nf5beC1eE5/A7w0/3bmP9bvAK/Niwbxr2Oe18cAX82/nfmPJ140iH8d87xeB/ht/u3MfzzxokG86F4b+C2e1+sAv82/nfmP9zLAX/MvQ7zoXhv4LZ6X+Pcx//FeB/ht/mWIF91rA7/F8xL/PuY/3usAv82/DPGie23gt3heDwFu5d/O/Md7HeC3+ZchXnQPBp7O83od4Lf5tzP/8U4Au/zLEP865nm9D/Dd/NuZ/3jiRYP417kVeBDP6XuA9+bfzvzHegbwYF40iH+d7wbei+e0C5zg3878x/oa4KN50SD+dd4a+Cme19sAP82/jfmP9TbAT/OiQfzrmef118DL8G9j/mOJFx3iX++7gffieb0P8N3865n/ON8DvDcvOsS/3oOBp/O8doGHALv865j/OA8BbuVFh/i3+W7gvXhefw28DrDLi878x/ge4L3510H82zwY+GvgGM/ru4H34UVn/v0uAQ8GdvnXQfzbvTfwXTx/fw28DrDLv8z8+70P8N386yH+fb4a+Ciev1uB9wF+mxfO/Pt8DfDR/Nsg/v1+G3gtXrDfBt4HuJXnz/zb/Q7w2vzbIf79jgO/DbwUL9xfA98N/A7w1zyb+bf5G+C1gV3+7RD/cb4beC9edLvAXwOvzb/e9wDvzb8f4j/WewPfxX+u9wG+m/8YiP94Dwa+Gngr/mP9DPDRwK38x0H853lt4LOB1+Lf53eAzwZ+m/94iP98DwbeGnht4K140fwM8NvATwO38p8H8V/rOPDSXPHSwHGu2AX+miv+Gtjlvwbi/zfE/2+I/98Q/78h/n9D/P/GPwIISMtB06M3LgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInvertColors;
impl IconShape for MdInvertColors {
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
                d: "M17.66 7.93L12 2.27 6.34 7.93c-3.12 3.12-3.12 8.19 0 11.31C7.9 20.8 9.95 21.58 12 21.58c2.05 0 4.1-.78 5.66-2.34 3.12-3.12 3.12-8.19 0-11.31zM12 19.59c-1.6 0-3.11-.62-4.24-1.76C6.62 16.69 6 15.19 6 13.59s.62-3.11 1.76-4.24L12 5.1v14.49z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zpvBbw08NLAcf7r/Tbw18DP8B8D8aJ5MPBTwEvzP8NfA+8D/DX/Poh/2XHgt4CX5n+WvwZeB9jl3w7xL/ts4LP4n+mngbfh3w7xL3s68GD+53of4Lv5t0H8y8z/bLvAywC38q+H+JeZ//n+GngZ/vUQ/zLzv8PnAJ/Nvw7iX2b+93gZ4K950SH+ZeZ/j1uBlwF2edEg/mXmf5fvBt6HFw3iX2b+93kb4Kf5lyH+ZeZ/n13gZYBbeeEQ/zLzv9NvA6/DC4f4l5n/vT4H+GxeMMS/zPzv9jLAX/P8If5l5n+3vwZeB9jleSH+ZeZ/v68BPprnhfiXmf8b3gb4aZ4T4l9m/m94G+CneU6If5n53+9rgI/meSH+ZeZ/t78BXhvY5Xkh/mXmf7eXAf6a5w/xLzP/e30O8Nm8YIh/mfnf6XeA1+aFQ/zLzP8+l4CXBm7lhUP8y8z/Pm8D/DT/MsS/zPzv8j3Ae/OiQfzLzP8ezwBeGtjlRYP4l5n/PV4H+G1edIh/mfnf4XOAz+ZfB/EvM//z/Q3w0vzrIf5l5n+2S8BLA7fyr4f4l90KPIj/ud4H+G7+bRD/ss8GPov/mX4GeGv+7RD/suPAbwMvxf8sfwO8NrDLvx3iRfNg4KeBl+J/hr8B3hq4lX8fxL/OWwMvDbw0cJz/er8N/DXw0/zHQPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8Iw0TWkGTbf39AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLabel;
impl IconShape for MdLabel {
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
                d: "M17.63 5.84C17.27 5.33 16.67 5 16 5L5 5.01C3.9 5.01 3 5.9 3 7v10c0 1.1.9 1.99 2 1.99L16 19c.67 0 1.27-.33 1.63-.84L22 12l-4.37-6.16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL5r3Bt6L/3q/Dfw18DP850C8aI4Dvw28FP89/hp4H+Cv+Y+FeNG9NPBX/Pf5a+B1gF3+4yD+dT4b+Cz++/w08Db8x0H86/018FL893kf4Lv5j4H413sw8NfAMf577AIvA9zKvx/i3+ajga/iv89fAy/Dvx/i3+63gdfiv8/nAJ/Nvw/i3+44cCtwjP8+LwP8Nf92iH+ftwZ+iv8+twIvA+zyb4P49/tp4K347/PdwPvwb4P49zsO/DXwIP77vA3w0/zrIf5jvDbwW/z32QVeBriVfx3Ef5yvBj6K/z6/DbwO/zqI/zjHgd8GXor/Pp8DfDYvOsR/rJcG/or/Xi8D/DUvGsR/vM8GPov/Pn8NvA6wy78M8Z/jr4GX4r/P1wAfzb8M8Z/jwcBfA8f47/M2wE/zwiH+czwY+CvgOP993gb4aV44xH+OvwJemv8+XwN8NP8yxH+8zwY+i/8+fwO8NrDLvwzxH+ulgb/iv9fLAH/NiwbxH+c48FvAS/Pf53OAz+ZFh/iP89XAR/Hf53eA1+ZfB/Ef47WB3+K/zyXgpYFb+ddB/PsdB/4KeDD/fd4G+Gn+9RD/fj8FvDX/fb4HeG/+bRD/Pm8N/BT/fZ4BvDSwy78N4t/uOPB04Dj/fV4H+G3+7RD/dr8FvDb/fT4H+Gz+fRD/Nh8NfBX/ff4GeGn+/RD/eg8G/go4zn+PS8BLA7fy74f41/sr4KX57/M+wHfzHwPxr/PZwGfx3+dngLfmPw7iRffSwF/x3+dvgNcGdvmPg3jRHAd+C3hp/nv8DfDWwK38x0K8aN4beG/+6/028NfAT/OfA/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjwfcWkF+KklaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLabelImportant;
impl IconShape for MdLabelImportant {
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
                d: "M3.5 18.99l11 .01c.67 0 1.27-.33 1.63-.84L20.5 12l-4.37-6.16c-.36-.51-.96-.84-1.63-.84l-11 .01L8.34 12 3.5 18.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEpklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7LvBh7Ef5+/Bm4FvgfY5T8W4l/22sBv8d9vF3gf4Kf5j4N40Xw18FH8z/A2wE/zHwPxojkO3Aoc47/fLvAQYJd/P8SL7q2Bn+J/hp8G3oZ/P8S/zk8Db8XzegZwK/85Xho4xvP6GOCr+fdB/OscB24FjvGcbgVeBtjlP95HA1/F89oFXgf4a/7tEP96Hw18Fc/ru4H34T/HbwOvxfP6a+Bl+LdD/Nv8NvBaPK+3AX6a/3jHgVuBYzyvrwE+mn8bxL/Ng4G/Bo7xnHaBhwC7/Md7a+CneP5eB/ht/vUQ/3afDXwWz+u3gdfhP8d3A+/F89oFHgLs8q+D+Pf5a+CleF4fA3w1//GOA38NPIjn9dPA2/Cvg/j3eWngr3heu8DLALfyH++1gd/i+Xsf4Lt50SH+/T4b+Cye118DL8N/js8GPovntQu8DHArLxrEv99x4K+BB/G8Pgf4bP5z/DXwUjyvvwZehhcN4j/GawO/xfP3MsBf8x/vpYHfBo7xvD4H+Gz+ZYj/OF8NfBTP61bgZYBd/uN9NPBVPH+vA/w2LxziP85x4FbgGM/ra4CP5j/HbwOvxfO6FXgZYJcXDPEf662Bn+L5exvgp/mPdxy4FTjG8/pp4G14wRD/sd4a+Cmev7cBfpr/eMeBpwPHeV4/A7w1LxjiP85x4OnAcZ7X1wAfzX+O3wJem+f1DOClgV1eMMR/nK8GPorn9QzgpYFd/uN9NPBVPH+vA/w2LxziP8ZrA7/F8/cywF/zH++lgd8CjvO8Pgf4bP5liH+/48BfAQ/meX0O8Nn85/gr4KV5Xn8DvDQvGsS/32cDn8Xz+hvgpfnP8dnAZ/G8LgEvDdzKiwbx7/PSwF/xvC4BLw3cyn+81wZ+i+fvfYDv5kWH+Pf5K+CleV4fA3w1//GOA38FPJjn9TPAW/Ovg/i3+2zgs3hePwO8Nf85vgt4b57XJeDBwC7/Ooh/mwcDfwUc5zldAh4M7PIf762Bn+L5ex3gt/nXQ/zb/Bbw2jyvtwF+mv94x4GnA8d5Xl8DfDT/Noh/vY8Gvorn9TXAR/Of47eA1+Z5/Q3w0vzbIf51jgNPB47znJ4BvDSwy3+8jwa+iud1CXht4K/5t0P86/wU8NY8r1uBW/nP8dLAcZ7XxwBfzb8P4kX31sBP8T/DzwBvzb8f4kVzHHg6cJz/fpeABwO7/PshXjRfDXwU/zO8DfDT/MdA/MteG/gt/vtdAt4b+Gn+4yD+Zd8NPJj/Pn8N/DXw08Au/7EQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IZXScQdfJH8wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLabelImportantOutline;
impl IconShape for MdLabelImportantOutline {
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
                d: "M15 19H3l4.5-7L3 5h12c.65 0 1.26.31 1.63.84L21 12l-4.37 6.16c-.37.52-.98.84-1.63.84zm-8.5-2H15l3.5-5L15 7H6.5l3.5 5-3.5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8yz4L+Bpgl/97EC/cdwHvDfw18DrALv+3IF6w7wLem2f7a+B1gF3+70A8f58NfBbP66+B1wF2+b8B8fwdB34beCme118DrwPs8r8f4gU7Dvw28FI8r78GXgfY5X83xAt3HPht4KV4Xn8NvA6wy/N6aeCr+I/328BfAz/DfwzEv+w48NvAS/G8/hp4HWCX5/XewHfxn+OvgfcB/pp/H8S/7DjwW8BL8/z9NfA6wC7P672B7+I/x18DrwPs8m+H+Jd9NvBZvHB/DbwOsMvzem/gu/jP8dPA2/Bvh/iXPR14MP+yvwZeB9jleb038F3853gf4Lv5t0H8y8yL7q+B1wF2eV7vDXwX//F2gZcBbuVfD/EvM/86fw28DrDL83pv4Lv4j/fXwMvwr4f4l5l/vb8GXgfY5Xm9N/Bd/Mf7HOCz+ddB/MvMv81fA68D7PK83hv4Lv7jvQzw17zoEP8y82/318DrALs8r/cGvov/WLcCLwPs8qJB/MvMv89fA68D7PK83hv4Lv5jfTfwPrxoEP8y8+/318DrALs8r/cGvov/WG8D/DT/MsS/zPzH+GvgdYBdntd7A9/Ff5xd4GWAW3nhEP8y8x/nr4HXAXZ5Xu8NfBf/cX4beB1eOMS/zPzH+mvgdYBdntd7A9/Ff5zPAT6bFwzxLzP/8f4aeB1gl+f13sB38R/nZYC/5vlD/MvMf46/Bl4H2OV5vTfwXfzH+GvgdYBdnhfiX2b+8/w18DrALs/rvYHv4j/G1wAfzfNC/MvMf66/Bl4H2OV5vTfwXfzHeBvgp3lOiH+Z+c/318DrALs8r/cGvot/v7cBfprnhPiXmf8afw28DrDL83pv4Lv4t/sa4KN5Xoh/mfmv89fA6wC7PK/3Br6Lf72/AV6a5w/xLzP/tf4aeB1gl+f13sB38aK7BLw0cCvPH+JfZv7r/TXwOsAuz+u9ge/iRfM2wE/zgiH+Zea/x18DrwPs8rzeG/guXrivAT6aFw7xLzP/ff4aeB1gl+f13sB38fz9DfDS/MsQ/zLz3+uvgdcBdnle7w18F8/pEvDSwK38yxD/MvPf76+B1wF2eV7vDXwXz/Y2wE/zokH8y8z/DH8NvA6wy/N6b+C7gK8BPpoXHeJfZv7n+GvgdYBdntdbAz/Nvw7iX2b+Z/lr4HWAXf79EP8y8z/PXwOvA+zy74P4l5n/mf4aeB1gl387xL/sVuBB/M/018DrALv82yD+ZZ8NfBb/c/018DrALv96iH/ZceC3gZfif66/Bl4H2OVfB/GieTDw08BL8T/XXwOvA+zyokP867w18NLAa/Pf68HAg3hefw28DrDLiwbxv9Nx4LeBl+J5/TXwOsAu/zLE/17Hgd8GXorn9TvAa/MvQ/zvdhz4beCleLZLwGsDf82/DPG/33Hgt4GXAi4Brw38NS8axP8Nx4GfBj4a+GtedIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IyLM1EH2hl3tAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLabelOff;
impl IconShape for MdLabelOff {
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
                d: "M3.25 2.75l17 17L19 21l-2-2H5c-1.1 0-2-.9-2-2V7c0-.55.23-1.05.59-1.41L2 4l1.25-1.25zM22 12l-4.37-6.16C17.27 5.33 16.67 5 16 5H8l11 11 3-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEN0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zpvBbw08NLAcf7r/Tbw18DP8B8D8aJ5MPBTwEvzP8NfA+8D/DX/Poh/2XHgt4CX5n+WvwZeB9jl3w7xL/ts4LP4n+mngbfh3w7xL3s68GD+53of4Lv5t0H8y8zz+htgl/9ax4GX4nntAi8D3Mq/HuJfZp7X6wC/zX+93wZei+f118DL8K+H+JeZ5/U6wG/zX+84cCtwjOf1OcBn86+D+JeZ5/U6wG/z3+OtgZ/i+XsZ4K950SH+ZeZ5vQ7w2/z3+W7gvXhetwIvA+zyokH8y8zzeh3gt/nvcxz4a+BBPK/vBt6HFw3iX2ae1+sAv81/r5cG/orn722An+ZfhviXmef1OsBv89/vs4HP4nntAi8D3MoLh/iXmef1OsBv8z/DbwOvxfP6beB1eOEQ/zLzvF4H+G3+Z3gw8NfAMZ7X5wCfzQuG+JeZ5/U6wG/zP8dbAz/F8/cywF/z/CH+ZeZ5vQ7w2/zP8tPAW/G8/hp4HWCX54X4l5nn9TrAb/M/y3Hgr4EH8by+BvhonhfiX2ae1+sAv83/PK8N/BbP39sAP81zQvzLzPN6HeC3+Z/ntYHf4vl7G+CneU6If5l5Xq8D/Db/sxwH/gp4MM/ra4CP5nkh/mXmeb0O8Nv8z/JTwFvzvP4GeG1gl+eF+JeZ5/U6wG/zP8dbAz/F8/cywF/z/CH+ZeZ5vQ7w2/zP8GDgr4DjPK/PAT6bFwzxLzPP63WA3+Z/ht8CXpvn9TvAa/PCIf5l5nm9DvDb/Pf7bOCzeF6XgJcGbuWFQ/zLzPN6HeC3+e/10sBf8fy9DfDT/MsQ/zLzvF4H+G3++xwH/gp4MM/re4D35kWD+JeZ5/U6wG/z3+e7gPfmeT0DeGlglxcN4l9mntfrAL/Nf4+3Bn6K5+91gN/mRYf4l5nn9TrAb/Nf7zjwdOA4z+tzgM/mXwfxLzPP63WA3+a/3m8Br83z+hvgpfnXQ/zLzPP6a2CX/1rHgZfmeV0CXhq4lX89xL/sVuBB/M/1PsB382+D+Jd9NvBZ/M/0M8Bb82+H+JcdB34beCn+Z/kb4LWBXf7tEC+aBwM/DbwU/zP8DfDWwK38+yD+dd4aeGngpYHj/Nf7beCvgZ/mPwbi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8ByquSQVYCW5wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLabelOutline;
impl IconShape for MdLabelOutline {
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
                d: "M17.63 5.84C17.27 5.33 16.67 5 16 5L5 5.01C3.9 5.01 3 5.9 3 7v10c0 1.1.9 1.99 2 1.99L16 19c.67 0 1.27-.33 1.63-.84L22 12l-4.37-6.16zM16 17H5V7h11l3.55 5L16 17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLanguage;
impl IconShape for MdLanguage {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zm6.93 6h-2.95c-.32-1.25-.78-2.45-1.38-3.56 1.84.63 3.37 1.91 4.33 3.56zM12 4.04c.83 1.2 1.48 2.53 1.91 3.96h-3.82c.43-1.43 1.08-2.76 1.91-3.96zM4.26 14C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2 0 .68.06 1.34.14 2H4.26zm.82 2h2.95c.32 1.25.78 2.45 1.38 3.56-1.84-.63-3.37-1.9-4.33-3.56zm2.95-8H5.08c.96-1.66 2.49-2.93 4.33-3.56C8.81 5.55 8.35 6.75 8.03 8zM12 19.96c-.83-1.2-1.48-2.53-1.91-3.96h3.82c-.43 1.43-1.08 2.76-1.91 3.96zM14.34 14H9.66c-.09-.66-.16-1.32-.16-2 0-.68.07-1.35.16-2h4.68c.09.65.16 1.32.16 2 0 .68-.07 1.34-.16 2zm.25 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95c-.96 1.65-2.49 2.93-4.33 3.56zM16.36 14c.08-.66.14-1.32.14-2 0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2h-3.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Y8jntdvA6/FfwzxgiFeNO8NfBf/OcTz+m3gtfiPIV4wxL/spYG/4j+PeF6/DbwW/zHEC4b4l3038F785xHP67eB1+I/hnjBEP+yi8Bx/vOI5/XbwGvxH0O8YIh/mXleHwP8Nf8xfpvn9dLAcV649wbei3+ZeMEQ/zLzvF4H+G3++7w38F28aMQLhviXmef1OsBv89/jvYHv4kUnXjDEv8w8r9cBfpv/eu8NfBf/OuIFQ/zLzPN6HeC3+a/13sB38YJ9D/BePC/xgiH+ZeZ5vQ7w2/zXeW/gu3jB3ge4Ffgtnpd4wRD/MvO8Xgf4bf5rvDfwXbxg7wN8N/DawG/xvMQLhviXmef1OsBv85/vvYHv4gV7H+C7ueK1gd/ieYkXDPEvM8/rdYDf5j/XewPfxQv2PsB382yvDfwWz0u8YIh/mXlerwP8Nv953hv4Ll6w9wG+m+f02sBv8bzEC4b4l5nn9TrAb/Of472B7+IFex/gu3lerw38Fs9LvGCIf5l5Xq8D/Db/8d4b+C5esPcBvpvn77WB3+J5iRcM8S8zz+t1gN/mP9Z7A9/FC/Y+wHfzgr028Fs8L/GCIf5l5nm9DvDb/Md5b+C7eMHeB/huXriXBr6a5/XavGCIf5l5Xq8D/Db/Md4b+C5esPcBvpv/HIh/mXlerwP8Nv9+7w18Fy/Y+wDfzX8exL/MPK/XAX6bf5/3Br6LF+x9gO/mPxfiX2ae1+sAv82/3XsD38UL9j7Ad/OfD/EvM8/rdYDf5t/mvYHv4gV7H+C7+a+B+JeZ5/U6wG/zr/fewHfxgr0P8N3827w28Fs8L/GCIf5l5nm9DvDb/Ou8N/BdvGDvA3w3/3avDfwWz0u8YIh/mXlerwP8Ni+69wa+ixfsfYDv5t/ntYHf4nmJFwzxLzPP63WA3+ZF897Ad/GCvQ/w3fz7vTbwWzwv8YIh/mXmeb0O8Nv8y94b+C5esPcBvpv/GK8N/BbPS7xgiH+ZeV6vA/w2L9x7A9/FC/Y+wHfzH+e1gd/ieYkXDPEvM8/rdYDf5gV7b+C7eMHeB/hu/mO9NvBbPC/xgiH+ZeZ5vQ7w2zx/Dwaezgv2PsB38x/vtYHf4nmJFwzxLzPP63WA3+YFe2/gu3he7wN8N/85Xhv4LZ6XeMEQ/zLzvF4H+G1euPcGvotnex/gu/nP89rAb/G8xAuG+JeZ5/U6wG/zL3tv4LuA9wG+m/9crw38Fs9LvGCIf5l5Xq8D/DYvmgcDt/Kf77WB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a/5neWngq3le4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lLw38Ff87PQS4lRcM8aJ5b+C7+N/lfYDv5oVDvOgeDHw28NbAMf5nugT8NPDZwK38yxD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R5OCuEET+V0RAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLaunch;
impl IconShape for MdLaunch {
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
                d: "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fawG/xb/M6wG/znwfxn++1gd/i3+Z1gN/mPw/iP99rA7/Fv83rAL/Nfx7Ef77XBn6Lf5vXAX6b/zyI/3yvDfwW/zavA/w2/3kQ//leG/gt/m1eB/ht/vMg/vO9NvBb/Nu8DvDb/OdB/Od7beC3+Ld5HeC3+c+D+M/32sBv8W/zOsBv858H8Z/vtYHf4t/mdYDf5j8P4j/fawO/xb/N6wC/zX8exH++1wZ+i3+b1wF+m/88iP98rw38Fv82rwP8Nv95EP/5Xhv4Lf5tXgf4bf7zIP7zvTbwW/zbvA7w2/znQfzne23gt/i3eR3gt/nPg3j+fht4Lf5txHN6beC3+Ld5HeC3+c+DeP5+G3gt/m3Ec3pt4Lf4t3kd4Lf5z4N4/n4beC3+bcRzem3gt/i3eR3gt/nPg3j+fht4Lf5txHN6beC3+Ld5HeC3+c+DeP5+G3gt/m3Ec3pt4Lf4t3kd4Lf5z4N4/n4beC3+bcRzem3gt/i3eR3gt3lOv8W/zd8AH81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE8/fbwGvxbyOe02sDv8W/zesAv81zMv82vwO8Ns8J8fz9NvBa/NuI5/TawG/xb/M6wG/znMy/ze8Ar81zQjx/vw28Fv824jm9NvBb/Nu8DvDbPCfzb/M7wGvznBDP328Dr8W/jXhOrw38Fv82rwP8Ns/J/Nv8DvDaPCfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFRmLZBax33HwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLeaderboard;
impl IconShape for MdLeaderboard {
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
                d: "M7.5,21H2V9h5.5V21z M14.75,3h-5.5v18h5.5V3z M22,11h-5.5v10H22V11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD1ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeG3hpXjR/DXw38DPArfznQfznOQ58FvDR/Pt8NfA5wC7/8RD/OT4L+GjgOP8xdoGvBj6H/1iI/1jHga8C3pv/HN8NfAywy38MxH+c48BvAS/Nf66/Bl4H2OXfD/Ef4zjwW8BL81/jr4HXAXb590H8x/gu4L35r/XdwPvw74P49/ts4LP47/E5wGfzb4f49zkOPB04zn+PXeAhwC7/Noh/n68GPor/Xl8DfDT/Noh/uwcDT+d/hocAt/Kvh/i3+2jgq/if4WOAr+ZfD/Fv91fAS/M/w+8Ar82/HuLf5jhwkf9ZTgC7/Osg/m1eG/gt/md5HeC3+ddB/Nt8NvBZ/M/yOcBn86+D+Lf5bOCz+J/lc4DP5l8H8W/z3cB78T/L9wDvzb8O4t/mp4G34n+W7wHem38dxL/NZwOfxf8snwN8Nv86iH+bzwY+i/9ZPgf4bP51EP82bw38FP+zvA7w2/zrIP5tHgw8nf9ZTgC7/Osg/u3+Gngp/mf4HeC1+ddD/Nt9NPBV/M/wMcBX86+H+Ld7MPB0/vtdAh4M7PKvh/j3+W7gvfjv9TnAZ/Nvg/j3OQ7cChzjv8czgJcGdvm3Qfz7fTTwVfz3eB3gt/m3Q/zH+G7gvfiv9TnAZ/Pvg/iPcRz4beCl+K/xPcB78++H+I9zHPht4KX4z/U9wHvzHwPxH++rgY/iP8fnAJ/NfxzEf47fBl6L/1i/A7w2/7EQ/zl+G3gt/mP9DvDa/MdC/Of4beC1+I/1O8Br8x8L8Z/jt4HX4j/W7wCvzX8sxH+O3wZei/9YvwO8Nv+xEP85fht4Lf5j/Q7w2vzHQvzn+G7gvfiP9T3Ae/MfC/Gf462Bn+I/1tsAP81/LMR/ns8G3ht4EP8+zwC+G/hs/uMh/n9D/P+G+P8N8f8b4v83xH++3+Lf53X4z4P4z2f+fcR/HsR/PvPvI/7zIP7z7QLH+Ld5BvBg/vMg/vN9NvBZ/Nt8DvDZ/OdB/Nd4a+CtgQfzorkV+Gngp/nPhfj/DfH/G+L/N8T/b4j/3/hHqdBtQQ4VG2QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLightbulb;
impl IconShape for MdLightbulb {
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
                d: "M9 21c0 .5.4 1 1 1h4c.6 0 1-.5 1-1v-1H9v1zm3-19C8.1 2 5 5.1 5 9c0 2.4 1.2 4.5 3 5.7V17c0 .5.4 1 1 1h6c.6 0 1-.5 1-1v-2.3c1.8-1.3 3-3.4 3-5.7 0-3.9-3.1-7-7-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Ni+Z3eE6vxYvmt4G3AXb590P8x/kt4LV5wZ4BfDfw08Bf8/y9NPDWwHsDD+IF+23gdfj3Q/zH+C7gvXn+LgEfDXw3/zofDXw2cIzn77uB9+HfB/Hv997Ad/H8/Q3w2sAu/zbHgd8GXorn732A7+bfDvHvcxx4OnCc5/U9wHvzH+O7gffiee0CDwF2+bdB/Pt8NvBZPK+/AV6a/1h/DbwUz+tzgM/m3wbxb/dg4K+A4zynS8CDgV3+Yx0HbgWO8Zx2gZcBbuVfD/Fv99HAV/G83gf4bv5zfDTwVTyvjwG+mn89xL/dbwOvxXN6BvBg/nPdCjyI5/QzwFvzr4f4tzkOXOR5fQ7w2fzn+mzgs3heJ4Bd/nUQ/zavDfwWz+tlgL/mP9dLA3/F83od4Lf510H823w28Fk8L/FfwzyvjwG+mn8dxL/NZwOfxXP6HeC1+a/x28Br8Zw+B/hs/nUQ/zbfDbwXz+l3gNfmv8ZvA6/Fc/oa4KP510H82/w08FY8p98BXpv/Gr8NvBbP6XuA9+ZfB/Fv89nAZ/Gcfgd4bf5r/DbwWjynzwE+m38dxL/NZwOfxXO6FXgI/zWeDjyY5/Q5wGfzr4P4t3lr4Kd4Xg8BbuU/14OBp/O8Xgf4bf51EP82DwaezvN6H+C7+c/13sB38bweAtzKvw7i3+6vgZfiOd0KPIT/XE8HHsxz+hvgpfnXQ/zbfTTwVTyvtwF+mv8cHw18Fc/rY4Cv5l8P8W93HLjI89oFHgLs8h/rwcBfAcd5XieAXf71EP8+3w28F8/rr4HXAXb5j3Ec+C3gpXle3wO8N/82iH+f48CtwDGe118DrwPs8u9zHPgt4KV5XpeABwO7/Nsg/v0+Gvgqnr+/Bt4GuJV/m5cGvgt4aZ6/jwG+mn87xH+M7wbei+dvF3gf4Kf513lr4LuA4zx/3wO8N/8+iP8Yx4HfBl6KF+xzgM/mRfPZwGfxgv0N8NrALv8+iP84x4HfBl6K5+93gNfmRfPbwGvx/P0N8NrALv9+iP9Yx4GvBt6L5/U7wGvzovlt4LV4Xt8DfDSwy38MxH+OvwZeiuf0O8Br86L5beC1eE5/A7w0/7EQ/zl+G3gtntPvAK/Ni+a3gdfiOf0O8Nr8x0L85/ht4LV4Tr8DvDYvmt8GXovn9DvAa/MfC/Gf47eB1+I5/TXw0bxovhp4aZ7T7wCvzX8sxH+O3wZei/9YvwO8Nv+xEP85fht4Lf5j/Q7w2vzHQvzn+G7gvfiP9T3Ae/MfC/Gf462Bn+I/1tsAP81/LMR/ns8G3ht4EP8+zwC+G/hs/uMh/n9D/P+G+P8N8f8b4v83xH++3+Lf53X4z4P4z2f+fcR/HsR/PvPvI/7zIP7z7QLH+Ld5BvBg/vMg/vN9NvBZ/Nt8DvDZ/OdB/Nd4a+CtgQfzorkV+Gngp/nPhfj/DfH/G+L/N8T/b4j/3/hHPi65QblsMNIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLightbulbOutline;
impl IconShape for MdLightbulbOutline {
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
                d: "M9,21c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1H9V21z M12,2C8.14,2,5,5.14,5,9c0,2.38,1.19,4.47,3,5.74V17 c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-2.26c1.81-1.27,3-3.36,3-5.74C19,5.14,15.86,2,12,2z M14,13.7V16h-4v-2.3 C8.48,12.63,7,11.53,7,9c0-2.76,2.24-5,5-5s5,2.24,5,5C17,11.49,15.49,12.65,14,13.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX/Za/O/2O7xgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9lr87/bb/OCIf5/Q/z/hvj/DfH/G+Jf9lr8x/sbYJfndBx4Kf7j/Q4vGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxP9viP/fEP+/If5/Q/z/hviX/Rb/eh8D/DXP9tXAS/Gv8zfAR/NsLw18Ff96r8MLhviXmX+91wF+m2f7beC1+Nf5HeC1ebbXBn6Lfz3xgiH+ZeZf73WA3+bZfht4Lf51fgd4bZ7ttYHf4l9PvGCIf5n513sd4Ld5tt8GXot/nd8BXptne23gt/jXEy8Y4l9m/vVeB/htnu23gdfiX+d3gNfm2V4b+C3+9cQLhviX/Tb/eh8N/DXP9tXAS/Ov89fAR/NsLw18Nf96r80Lhvj/DfH/G+L/N8T/b4j/3xD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xgiH+Za/N8/prYJfndBx4aZ7Xb/O8Xho4znPaBf6a5/XaPK+/BnZ5TseBl+Z5/TYvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I5M5bEHrQC6SAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLineStyle;
impl IconShape for MdLineStyle {
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
                d: "M3,16h5v-2H3V16z M9.5,16h5v-2h-5V16z M16,16h5v-2h-5V16z M3,20h2v-2H3V20z M7,20h2v-2H7V20z M11,20h2v-2h-2V20z M15,20 h2v-2h-2V20z M19,20h2v-2h-2V20z M3,12h8v-2H3V12z M13,12h8v-2h-8V12z M3,4v4h18V4H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX/Za/O/2O7xgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9lr87/bb/OCIf5/Q/z/hvj/DfH/G+Jf9lr87/Y7vGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/2W/zv9tr84Ih/n9D/P+G+P8N8f8b4v83xL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/7LX53+23ecEQ/78h/n9D/P+G+P8N8S97Lf53+x1eMMS/zPzvJl4wxL/M/O8mXjDEv+y1+d/tt3nBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAdHhH0G+bkF9AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLineWeight;
impl IconShape for MdLineWeight {
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
                d: "M3,17h18v-2H3V17z M3,20h18v-1H3V20z M3,13h18v-3H3V13z M3,4v4h18V4H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/2WjyvvwF2eU7HgZfif57f4QVD/MvM83od4Ld5Tq8N/Bb/84gXDPEvM8/rdYDf5jm9NvBb/M8jXjDEv8w8r9cBfpvn9NrAb/E/j3jBEP8y87xeB/htntNrA7/F/zziBUP8y8zzeh3gt3lOrw38Fv/ziBcM8f8b4v83xP9viP/fEP+/If5lv8Xz+hjgr3lOLw18Ff/zvA4vGOJfZp7X6wC/zXN6beC3+J9HvGCIf5l5Xq8D/DbP6bWB3+J/HvGCIf5l5nm9DvDbPKfXBn6L/3nEC4b4l5nn9TrAb/OcXhv4Lf7nES8Y4l/22zyvjwb+muf00sBX8z/Pa/OCIf5/Q/z/hvj/DfH/G+L/N8S/zDyv1wF+m+f02sBv8T+PeMEQ/zLzvF4H+G2e02sDv8X/POIFQ/zLzPN6HeC3eU6vDfwW//OIFwzxLzPP63WA3+Y5vTbwW/zPI14wxL/MPK/XAX6b5/TawG/xP494wRD/stfmef01sMtzOg68NP/z/DYvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMP0zZBzZLl1AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdList;
impl IconShape for MdList {
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
                d: "M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z2vxnH6H/3qI/zoPBt4LeGvgpXn+/hr4aeB7gFv5z4f4z3cc+CrgvfnX+Wrgc4Bd/vMg/nO9NPBbwHH+bXaB1wH+mv8ciP887w18F/8x3gf4bv7jIf5zvDTwV7xwzwBu5YoHAw/ihXsZ4K/5j4X4j3cceDpwnOd1Cfhq4LuBW3lODwbeG/ho4BjPaxd4CLDLfxzEf7zvBt6L5/U3wFsDt/LCPRj4aeCleF5fA3w0/3EQ/7EeDDyd5/U3wGsDu7xojgO/DbwUz+shwK38x0D8x/ps4LN4TpeAlwZu5V/nwcDTeV6fA3w2/zEQ/7H+CnhpntPnAJ/Nv81nA5/Fc/pr4GX4j4H4j2We10OAW/m3eWngr3he4j8G4j/OawO/xXN6BvBg/n12gWM8p9cBfpt/P8R/nNcGfovn9DvAa/Pv89vAa/GcXgf4bf79EP9xXhv4LZ7T7wCvzb/PbwOvxXN6HeC3+fdD/Md5beC3eE6/A7w2/z6/DbwWz+l1gN/m3w/xH+e1gd/iOf0O8Nr8+/w28Fo8p9cBfpt/P8R/nNcGfovn9DvAa/Pv89vAa/GcXgf4bf79EP9xXhv4LZ7T7wCvzb/PbwOvxXN6HeC3+fdD/NscB94LeGngwVxxHHhpntMu8Nf8+7w0cJzn9NfALlfcCvw18D3ALv86iH+99wa+CjjO/yy7wMcA382LDvGv89rAb/E/2+sAv82LBvGv83TgwfzPdivwEF40iBfdSwN/xf8OLwP8Nf8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/iP84zgFu54sHAg/iP8zrAb/MvQ7zoXhv4Lf79LgHvDfw0z+mtge8GjvHv9zrAb/MvQ7zoXhv4Lf59LgEPBnZ5/o4DtwLH+Pd5HeC3+ZchXnSvDfwW/z5vA/w0L9xbAz/Fv8/rAL/Nvwzxontt4Lf4t3sG8GBeNLcCD+Lf7nWA3+ZfhnjRvTbwW/zb/Q7w2rxofht4Lf7tXgf4bf5liBfdawO/xb/d7wCvzYvmt4HX4t/udYDf5l+GeNG9NvBb/NvdCjyEF83TgQfzb/c6wG/zL0O86F4b+C3+fd4G+GleuLcGfop/n9cBfpt/GeJF99rAb/Hvsws8BNjl+Xtp4LeA4/z7vA7w2/zLEC+61wZ+i3+/XeB9gJ/mOb018F3Acf79Xgf4bf5liBfdawO/xX+cW4FbueLBwIP5j/M6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhr4K/53eBngr/mXIf51bgUexP9szwAezIsG8a/z2sBv8T/b6wC/zYsG8a/33sBXA8f4n+US8N7AT/OiQ/zbHAfeG3hp4MH897oV+Gvgu4Fd/nUQ/78h/n9D/P+G+P8N8f8b/whc+rxBZ7l5eAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLock;
impl IconShape for MdLock {
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
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLockClock;
impl IconShape for MdLockClock {
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
                d: "M14.5 14.2l2.9 1.7-.8 1.3L13 15v-5h1.5v4.2zM22 14c0 4.41-3.59 8-8 8-2.02 0-3.86-.76-5.27-2H4c-1.15 0-2-.85-2-2V9c0-1.12.89-1.96 2-2v-.5C4 4.01 6.01 2 8.5 2c2.34 0 4.24 1.79 4.46 4.08.34-.05.69-.08 1.04-.08 4.41 0 8 3.59 8 8zM6 7h5v-.74C10.88 4.99 9.8 4 8.5 4 7.12 4 6 5.12 6 6.5V7zm14 7c0-3.31-2.69-6-6-6s-6 2.69-6 6 2.69 6 6 6 6-2.69 6-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z2vxnH6H/3qI/zoPBt4LeGvgpXn+/hr4aeB7gFv5z4f4z3cc+CrgvfnX+Wrgc4Bd/vMg/nO9NPBbwHH+bXaB1wH+mv8ciP887w18F/8x3gf4bv7jIf5zvDTwV7xwzwBu5YoHAw/ihXsZ4K/5j4X4j3cceDpwnOd1Cfhq4LuBW3lODwbeG/ho4BjPaxd4CLDLfxzEf7zvBt6L5/U3wFsDt/LCPRj4aeCleF5fA3w0/3EQ/7EeDDyd5/U3wGsDu7xojgO/DbwUz+shwK38x0D8x/ps4LN4TpeAlwZu5V/nwcDTeV6fA3w2/zEQ/7H+CnhpntPnAJ/Nv81nA5/Fc/pr4GX4j4H4j2We10OAW/m3eWngr3he4j8G4j/OawO/xXN6BvBg/n12gWM8p9cBfpt/P8R/nNcGfovn9DvAa/Pv89vAa/GcXgf4bf79EP/z/TbwWjyn1wF+m38/xP98vw28Fs/pdYDf5t8P8T/fbwOvxXN6HeC3+fdD/M/328Br8ZxeB/ht/v0Q//P9NvBaPKfXAX6bfz/Ev81x4L2AlwYezH+ulwaO85z+GtjliluBvwa+B9jlXwfxr/fewFcBx/mfZRf4GOC7edEh/nVeG/gt/md7HeC3edEg/nWeDjyY/9luBR7Ciwbxontp4K/43+FlgL/mX4Z40b028Fs8r9/hv9dr8bxeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMS/zYOBB3HFM4Bb+bcxz+t1gN/mX4Z40b028Fs8L/Gvcxz4LuCteU4/DbwPsMu/jnlerwP8Nv8yxIvutYHf4nmJF91x4OnAcZ6/XeAhwC4vOvO8Xgf4bf5liBfdawO/xfMSL7qfAt6aF+6ngbfhRWee1+sAv82/DPGie23gt3he4kXzYODpvGgeAtzKi8Y8r9cBfpt/GeJF99rAb/G8xIvmtYHf4kXzOsBv86Ixz+t1gN/mX4Z40b028Fs8L/GieW3gt3jRvA7w27xozPN6HeC3+ZchXnSvDfwWz0u8aB4MPJ0XzUOAW3nRmOf1OsBv8y9DvOheG/gtnpd40f008Fa8cD8DvDUvOvO8Xgf4bf5liBfdawO/xfMSL7rjwK3AMZ6/vwFeG9jlRWee1+sAv82/DPGie23gt3he4l/nOPDdwFvxnH4GeG9gl38d87xeB/ht/mWIF91rA7/F8xL/Ng8GHswVtwK38m9jntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2e12/z3+u1eV6vA/w2/zLEi+6lgb/if4eXAf6afxniX+dW4EH8z/YM4MG8aBD/Oq8N/Bb/s70O8Nu8aBD/eu8NfDVwjP9ZLgHvDfw0LzrEv81x4L2BlwYezH+vW4G/Br4b2OVfB/H/G+L/N8T/b4j/3xD/v/GPzxfMQbkDlmIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLockOpen;
impl IconShape for MdLockOpen {
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
                d: "M12 17c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6h1.9c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm0 12H6V10h12v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLockOutline;
impl IconShape for MdLockOutline {
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
                d: "M12,17c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,17,12,17z M18,8h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6 c-1.1,0-2,0.9-2,2v10c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.9,19.1,8,18,8z M8.9,6c0-1.71,1.39-3.1,3.1-3.1 s3.1,1.39,3.1,3.1v2H8.9V6z M18,20H6V10h12V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b472f+c9wKfDfwNcAuzx/iv5/5z/XXwNsAt/K8EP/9zH++vwZehueF+O9n/mu8DfDTPCfEfz/zX+NzgM/mOSH++702//G+GngpntPvAK/Nc0L83/TbwGvxnH4HeG2eE+L/pt8GXovn9DvAa/OcEP83/TbwWjyn3wFem+eE+L/pt4HX4jn9DvDaPCfEf48HA7fyn+e3gdfiOf0O8No8J8R/ve8C3hp4HeCv+c/x28Br8Zx+B3htnhPiv9Z3Ae/NFbvA6wB/zX+83wZei+f0O8Br85wQ/3W+C3hvntMu8DrAX/Mf67eB1+I5/Q7w2jwnxH+N1wZ+i+dvF3gd4K/5j/PbwGvxnH4HeG2eE+K/znsD38Xztwu8DvDX/Mf4beC1eE6/A7w2zwnxX+u9ge/i+dsFXgf4a/79fht4LZ7T7wCvzXNC/Nd7b+C7eP52gdcB/pp/n98GXovn9DvAa/OcEP893hv4Lp6/XeB1gL/m3+63gdfiOf0O8No8J8R/n/cGvovnbxd4HeCv+bf5beC1eE6/A7w2zwnx3+u9ge/i+dsFXgf4a/71fht4LZ7T7wCvzXNCPH9fDbwU/zVeGjjO87cLvA7w1/zr/DbwWjyn3wFem+eEeP5+G3gt/mfYBV4H+GtedL8NvBbP6XeA1+Y5IZ6/3wZei/85doHXAf6aF81vA6/Fc/od4LV5Tojn77eB1+J/jkvAawN/zYvmt4HX4jn9DvDaPCfE8/fbwGvxP8Ml4LWBv+ZF99vAa/Gcfgd4bZ4T4vn7auCl+a/x0sAxnr9LwGsDf82/zm8Dr8Vz+h3gtXlOiP9e7w18F8/fJeC1gb/mX++3gdfiOf0O8No8J8R/n/cGvovn7xLw2sBf82/z28Br8Zx+B3htnhPiv8d7A9/F83cJeG3gr/m3+23gtXhOvwO8Ns8J8V/vvYHv4vm7BLw28Nf8+/w28Fo8p98BXpvnhPiv9d7Ad/H8XQJeG/hr/v1+G3gtntPvAK/Nc0L813lv4Lt4/i4Brw38Nf8xfht4LZ7T7wCvzXNC/Nd4MPB0nr9LwGsDf81/nN8GXovn9DvAa/OcEP913hv4Lp7TJeC1gb/mP9ZvA6/Fc/od4LV5Toj/Wu8NfBdXXAJeG/hr/uP9NvBaPKffAV6b54T4r/fewFcDrw38Nf85fht4LZ7T7wCvzXNC/Pc4Duzyn+e3gdfiOf0O8No8J8T/Tb8NvBbP6XeA1+Y5If5v+m3gtXhOvwO8Ns8J8X/TbwOvxXP6HeC1eU6I/5t+G3gtntPvAK/Nc0L893st/uN9NfDSPKffAV6b54T472f+a3wO8Nk8J8R/P/Nf422An+Y5If77mf98fwO8NM8L8d/P/Of6G+CtgVt5Xoj/fuY/xzOA7wa+Gtjl+UP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGwMbRBIig7VwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLogin;
impl IconShape for MdLogin {
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
                d: "M11,7L9.6,8.4l2.6,2.6H2v2h10.2l-2.6,2.6L11,17l5-5L11,7z M20,19h-8v2h8c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2h-8v2h8V19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/nOI/z6IF+zBwG8BD+Y/l/jvg3jB/gp4af7zif8+iOfvrYGf4r+G+O+DeP4+G/gs/muI/z6I5++3gdfiOf0N8NH8x/tt/vsgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDb/tyCev98GXovn9DvAa/N/C+L5+23gtXhOvwO8Nv+3IJ6/3wZei+f0O8Br8z/Dg4Fb+fdDPH+/DbwWz+l3gNfmv993AW8NvA7w1/z7IJ6/3wZei+f0O8Br89/ru4D35opd4HWAv+bfDvH8/TbwWjyn3wFem/8+3wW8N89pF3gd4K/5t0E8f78NvBbP6XeA1+a/x2sDv8Xztwu8DvDX/Oshnr/fBl6L5/Q7wGvz3+e9ge/i+dsFXgf4a/51EM/fbwOvxXP6HeC1+e/13sB38fztAq8D/DUvOsTz99vAa/Gcfgd4bf77vTfwXTx/u8DrAH/Niwbx/P028Fo8p98BXpv/Gd4b+C6ev13gdYC/5l+GeP5+G3gtntPvAK/N/xzvDXwXz98u8DrAX/PCIZ6/3wZei+f0O8Br8z/LewPfxfO3C7wO8Ne8YIjn77eB1+I5/Q7w2rxovhp4Kf5rvDRwnOdvF3gd4K95/hDP328Dr8Vz+h3gtXnR/DbwWvzPsAu8DvDXPC/E8/fbwGvxnH4HeG1eNL8NvBb/c+wCrwP8Nc8J8fz9NvBaPKffAV6bF81vA6/F/xyXgNcG/prnhHj+fht4LZ7T7wCvzYvmt4HX4n+GS8BrA3/N80I8f78NvBbP6XeA1+ZF89XAS/Nf46WBYzx/l4DXBv6a5w/x/P028Fo8p98BXpv/Wd4b+C6ev0vAawN/zQuGeP5+G3gtntPvAK/N/xzvDXwXz98l4LWBv+aFQzx/vw28Fs/pd4DX5n+G9wa+i+fvEvDawF/zL0M8f78NvBbP6XeA1+a/33sD38Xzdwl4beCvedEgnr/fBl6L5/Q7wGvz3+u9ge/i+bsEvDbw17zoEM/fbwOvxXP6HeC1+e/z3sB38fxdAl4b+Gv+dRDP328Dr8Vz+h3gtfnv8WDg6Tx/l4DXBv6afz3E8/fbwGvxnH4HeG3++7w38F08p0vAawN/zb8N4vn7beC1eE6/A7w2/73eG/gurrgEvDbw1/zbIZ6/3wZei+f0O8Br89/vo4HPBl4b+Gv+fRDP328Dr8Vz+h3gtfmf4cHArfz7IZ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/Q7wGvzfwvi+ftt4LV4Tr8DvDb/tyCev98GXovn9NfAR/Mf73f474N4/j4b+Cz+a4j/Pojn762Bn+K/hvjvg3jB/hp4Kf7zif8+iBfswcBPAy/Ffy7x3wfxwh0HPhp4b+BB/OcQ/30Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ee6Y6ZBbvRLxgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLogout;
impl IconShape for MdLogout {
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
                d: "M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLoyalty;
impl IconShape for MdLoyalty {
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
                d: "M21.41 11.58l-9-9C12.05 2.22 11.55 2 11 2H4c-1.1 0-2 .9-2 2v7c0 .55.22 1.05.59 1.42l9 9c.36.36.86.58 1.41.58.55 0 1.05-.22 1.41-.59l7-7c.37-.36.59-.86.59-1.41 0-.55-.23-1.06-.59-1.42zM5.5 7C4.67 7 4 6.33 4 5.5S4.67 4 5.5 4 7 4.67 7 5.5 6.33 7 5.5 7zm11.77 8.27L13 19.54l-4.27-4.27C8.28 14.81 8 14.19 8 13.5c0-1.38 1.12-2.5 2.5-2.5.69 0 1.32.28 1.77.74l.73.72.73-.73c.45-.45 1.08-.73 1.77-.73 1.38 0 2.5 1.12 2.5 2.5 0 .69-.28 1.32-.73 1.77z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkAsUnread;
impl IconShape for MdMarkAsUnread {
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
                d: "M18.83 7h-2.6L10.5 4 4 7.4V17c-1.1 0-2-.9-2-2V7.17c0-.53.32-1.09.8-1.34L10.5 2l7.54 3.83c.43.23.73.7.79 1.17zM20 8H7c-1.1 0-2 .9-2 2v9c0 1.1.9 2 2 2h13c1.1 0 2-.9 2-2v-9c0-1.1-.9-2-2-2zm0 3.67L13.5 15 7 11.67V10l6.5 3.33L20 10v1.67z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj3M//xXgf4bf7zIf79zH+81wF+m/98iH8/8x/vdYDf5j8f4t/P/Md7HeC3+c+H+Pcz//FeB/ht/vMh/v3Mf7zXAX6b/3yIfz/zH+91gN/mPx/i38/8x3sd4Lf5z4f49zP/8V4H+G3+8yH+/cx/vNcBfpv/fIh/P/O8/gbY5d/uo4G/5j8f4t/PPK/XAX6b//kQ/37meb0O8Nv8z4f49zPP63WA3+Z/PsS/n3lerwP8Nv/zIf79zPN6HeC3+Z8P8cIdBz4KeG/gwbzoXgf4bZ7TawO/xX+dW4HvBr4G2OX5Q7xgDwZ+C3gw/3qvA/w2z+m1gd/iv95fA28D3MrzQrxgfwW8NP82rwP8Ns/ptYHf4r/HXwMvw/NCPH9vDfwU/3avA/w2z+m1gd/iv8/bAD/Nc0I8f58NfBb/dq8D/DbP6bWB3+K/z+cAn81zQjx/vw28Fv92rwP8Ns/ptYHf4r/P7wCvzXNCPH+/DbwW/3avA/w2z+m1gd/iv8/vAK/Nc0I8f78NvBb/dq8D/DbP6bWB3+K/z+8Ar81zQjx/vw28Fv92rwP8Ns/ptYHf4r/P7wCvzXNCPH+/DbwW/3avA/w2z+m1gd/iv8/vAK/Nc0I8f78NvBb/dq8D/DbP6bWB3+K/z+8Ar81zQjx/vw28Fv92rwP8Ns/ptYHf4r/P7wCvzXNCPH+/DbwW/3avA/w2z+m1gd/iv8/vAK/Nc0I8f78NvBb/dq8D/DbP6bWB3+K/z+8Ar81zQjx/vw28Fv92rwP8Ns/ptYHf4r/P7wCvzXNCPH+/DbwW/3avA/w2z+m1gd/iv8/vAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP308Bb8X/LzwBvzXNCPH+fDXwW/7d8DvDZPCfE8/fSwF/xf8tDgFt5TogX7KuBj+L/hq8BPprnhXjhvhr4KP53+xrgo3n+EP+ylwbeGnhp4Dj/O+wCfw18N3ArLxji/zfE/2+I/98Q/78h/n/jHwEIwp1BJsG9TwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkunreadMailbox;
impl IconShape for MdMarkunreadMailbox {
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
                d: "M20 6H10v6H8V4h6V0H6v6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjUOcGQeBNCJcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMaximize;
impl IconShape for MdMaximize {
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
                d: "M3 3h18v2H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMediation;
impl IconShape for MdMediation {
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
                d: "M22 12l-4 4-1.41-1.41L18.17 13h-5.23c-.34 3.1-2.26 5.72-4.94 7.05C7.96 21.69 6.64 23 5 23c-1.66 0-3-1.34-3-3s1.34-3 3-3c.95 0 1.78.45 2.33 1.14 1.9-1.03 3.26-2.91 3.58-5.14h-3.1C7.4 14.16 6.3 15 5 15c-1.66 0-3-1.34-3-3s1.34-3 3-3c1.3 0 2.4.84 2.82 2h3.1c-.32-2.23-1.69-4.1-3.59-5.14C6.78 6.55 5.95 7 5 7 3.34 7 2 5.66 2 4s1.34-3 3-3c1.64 0 2.96 1.31 2.99 2.95 2.68 1.33 4.6 3.95 4.94 7.05h5.23l-1.58-1.59L18 8l4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+Lf77X47/U7/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwK2xQZB+Ra6sAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMinimize;
impl IconShape for MdMinimize {
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
                d: "M6 19h12v2H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdModelTraining;
impl IconShape for MdModelTraining {
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
                d: "M15.5,13.5c0,2-2.5,3.5-2.5,5h-2c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5h0C13.93,10,15.5,11.57,15.5,13.5z M13,19.5h-2 V21h2V19.5z M19,13c0,1.68-0.59,3.21-1.58,4.42l1.42,1.42C20.18,17.27,21,15.23,21,13c0-2.74-1.23-5.19-3.16-6.84l-1.42,1.42 C17.99,8.86,19,10.82,19,13z M16,5l-4-4v3c0,0,0,0,0,0c-4.97,0-9,4.03-9,9c0,2.23,0.82,4.27,2.16,5.84l1.42-1.42 C5.59,16.21,5,14.68,5,13c0-3.86,3.14-7,7-7c0,0,0,0,0,0v3L16,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNextPlan;
impl IconShape for MdNextPlan {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M18,13.97h-5l2.26-2.26 c-0.91-1.06-2.25-1.74-3.76-1.74c-2.37,0-4.35,1.66-4.86,3.88l-0.96-0.32c0.64-2.62,3-4.56,5.82-4.56c1.78,0,3.37,0.79,4.47,2.03 L18,8.97V13.97z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/s7wW8NLAceA48NLALvDXXLEL/DXwN8Au/36I/16vDbwV8NLAa/Ov89PA+wC7/Nsh/usdB94K+Gzgwfz7fA7w2fzbIf5rfRbw0cBx/mP8DvDa/Nsh/mu8NvBdwIP5j/U9wHvzb4f4z3Uc+C7grfmP9zfAawO7/Nsh/vMcB34LeGn+Yz0D+Gzgu/n3Q/zneGngp4AH8x/nGcBnA9/NfxzEf7yXBn4LOM5/nJ8B3hvY5T8W4j/WceDpwHH+47wP8N3850D8x/or4KX5j3EJeGvgt/nPg/iP813Ae/Mf4xLw2sBf858L8R/jtYHf4j/O+wDfzX8+xH+MpwMP5j/G1wAfzX8NxL/fZwOfxX+MvwFemv86iH+f48DTgeP8x3gd4Lf5r4P49/lo4Kv4j/E9wHvzXwvx7/NXwEvzH+MhwK3810L827008Ff8x/gd4LX5r4f4t/tu4L34j/E+wHfzXw/xb/dXwEvzH0P890D825n/GL8DvDb/PRD/Nq8N/Bb/MT4H+Gz+eyD+bd4b+C7+Y7wO8Nv890D823w28Fn8x3gd4Lf574H4t/ls4LP4j/EywF/z3wPxb/PdwHvxH0P890H82/w08Fb8xxD/fRD/Np8NfBb/MR4C3Mp/D8S/zWcDn8V/jNcBfpv/Hoh/m48Gvor/GG8D/DT/PRD/Nq8N/Bb/MT4H+Gz+eyD+bY4DF/mP8TvAa/PfA/Fv99fAS/EfQ/z3QPzbfTXwUfzHeB/gu/mvh/i3e2ngr/iP8dvA6/BfD/HvcyvwIP5jPAS4lf9aiH+fjwa+iv8Y3wO8N/+1EP8+x4FbgWP8x3gd4Lf5r4P49/ts4LP4j/HbwOvwXwfx73ccuBU4xn+MrwE+mv8aiP8Ybw38FP9x3gf4bv7zIf7j/DTwVvzH2AVeB/hr/nMh/uMcB34beCn+Y+wC7wP8NP95EP+xXhr4beAY/3HeB/hu/nMg/uO9NPDbwDH+4/w08D7ALv+xEP85Xhr4beAY/3F2gY8Gvof/OIj/PC8NfDfwUvzHuhX4bOB7+PdD/Oc6Dnw38Fb8x/tr4HWAXf7tEP813hr4buAY/7G+B3hv/u0Q/3WOAx8NfDRwjP8YvwO8Nv92iP96x4GPBt4beBD/Pp8DfDb/doj/Xq8NvDXw2sBL8a/zM8B7A7v82yH+5zgOvDTw0sBxrnhprvhrrtgF/hr4a2CXfz/E/2+I/98Q/78h/n9D/P/GPwJlKX1Bj3QGygAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNightlightRound;
impl IconShape for MdNightlightRound {
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
                d: "M12.01 12c0-3.57 2.2-6.62 5.31-7.87.89-.36.75-1.69-.19-1.9-1.1-.24-2.27-.3-3.48-.14-4.51.6-8.12 4.31-8.59 8.83C4.44 16.93 9.13 22 15.01 22c.73 0 1.43-.08 2.12-.23.95-.21 1.1-1.53.2-1.9-3.22-1.29-5.33-4.41-5.32-7.87z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotAccessible;
impl IconShape for MdNotAccessible {
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
                d: "M14,11.05l-3.42-3.42c0.32-0.34,0.74-0.57,1.23-0.61c0.48-0.04,0.84,0.07,1.2,0.26c0.19,0.1,0.39,0.22,0.63,0.46l1.29,1.43 c0.98,1.08,2.53,1.85,4.07,1.83v2C17.25,12.99,15.29,12.12,14,11.05z M12,6c1.1,0,2-0.9,2-2s-0.9-2-2-2c-1.1,0-2,0.9-2,2 S10.9,6,12,6z M2.81,2.81L1.39,4.22L10,12.83V15c0,1.1,0.9,2,2,2h2.17l5.61,5.61l1.41-1.41L2.81,2.81z M10,20c-1.66,0-3-1.34-3-3 c0-1.31,0.84-2.41,2-2.83V12.1c-2.28,0.46-4,2.48-4,4.9c0,2.76,2.24,5,5,5c2.42,0,4.44-1.72,4.9-4h-2.07 C12.42,19.16,11.31,20,10,20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotStarted;
impl IconShape for MdNotStarted {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M11,16H9V8h2V16z M12,16V8l5,4L12,16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoteAdd;
impl IconShape for MdNoteAdd {
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 14h-3v3h-2v-3H8v-2h3v-3h2v3h3v2zm-3-7V3.5L18.5 9H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOfflineBolt;
impl IconShape for MdOfflineBolt {
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
                d: "M12 2.02c-5.51 0-9.98 4.47-9.98 9.98s4.47 9.98 9.98 9.98 9.98-4.47 9.98-9.98S17.51 2.02 12 2.02zM11.48 20v-6.26H8L13 4v6.26h3.35L11.48 20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOfflinePin;
impl IconShape for MdOfflinePin {
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
                d: "M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M17,18H7v-2h10V18z M10.3,14L7,10.7l1.4-1.4l1.9,1.9 l5.3-5.3L17,7.3L10.3,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOnlinePrediction;
impl IconShape for MdOnlinePrediction {
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
                d: "M15.5,11.5c0,2-2.5,3.5-2.5,5h-2c0-1.5-2.5-3-2.5-5C8.5,9.57,10.07,8,12,8S15.5,9.57,15.5,11.5z M13,17.5h-2V19h2V17.5z M22,12c0-2.76-1.12-5.26-2.93-7.07l-1.06,1.06C19.55,7.53,20.5,9.66,20.5,12c0,2.34-0.95,4.47-2.49,6.01l1.06,1.06 C20.88,17.26,22,14.76,22,12z M3.5,12c0-2.34,0.95-4.47,2.49-6.01L4.93,4.93C3.12,6.74,2,9.24,2,12c0,2.76,1.12,5.26,2.93,7.07 l1.06-1.06C4.45,16.47,3.5,14.34,3.5,12z M17.5,12c0,1.52-0.62,2.89-1.61,3.89l1.06,1.06C18.22,15.68,19,13.93,19,12 c0-1.93-0.78-3.68-2.05-4.95l-1.06,1.06C16.88,9.11,17.5,10.48,17.5,12z M7.05,16.95l1.06-1.06c-1-1-1.61-2.37-1.61-3.89 c0-1.52,0.62-2.89,1.61-3.89L7.05,7.05C5.78,8.32,5,10.07,5,12C5,13.93,5.78,15.68,7.05,16.95z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFDklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e700V/w1/z0Q/31eGvgtrngd4K/5r4f47/HSwG8Bx7liF3gd4K/5r4X4r/fSwG8Bx3lOu8DrAH/Nfx3Ef62XBn4LOM7ztwu8DvDX/NdA/Nd5aeC3gOO8cLvA6wB/zX8+xH+NlwZ+CzjOi2YXeB3gr/nPhfjP997Ad/Gvtwu8DvDX/OdB/Od6b+C7eMG+hyvei+dvF3gd4K/5z4H4z/PewHfxgn0P8N5c8d3Ae/H87QKvA/w1//EQ/zneG/guXrDvAd6b5/TdwHvx/O0C7wP8NP+xEP/x3hv4Ll6w7wHem+fvu4H34gV7H+C7+Y+D+I/13sB38YJ9D/DevHDfDbwXL9j7AN/NfwzEf5z3Br6LF+x7gPfmRfPdwHvxgr0P8N38+yH+Y7w38F28YN8DvDf/On8NvBQv2PsA382/D+Lf77WB3+IF+x7gvfnXOw78NvBSvGAvA/w1/3aIf58HA38FHOf5+x7gvfm3Ow78NvBSPH+7wMsAt/Jvg/i3Ow78FvDSPH8/A7w1/37Hgd8GXorn76+B1wF2+ddD/Nt9F/DePH9/A7w2sMt/jOPAbwMvxfP33cD78K+H+Ld5beC3eP7+BnhtYJf/WMeB3wZeiufvdYDf5l8H8W/zW8Br87z+BnhtYJf/HA8G/ho4xvP6beB1+NdB/Ou9NvBbPK9LwGsDf81/rpcGfhs4xvN6HeC3edEh/vX+Cnhpntf7AN/Nf42PBr6K5/XbwOvwokP86zwYeDrP62+Al+a/1q3Ag3heDwFu5UWD+Nd5b+C7eF5vA/w0/7XeG/guntf7AN/Niwbxr/PTwFvxnC4Bx/nvsQsc4zn9DPDWvGgQ/zrmeX0P8N789/hp4K14TrvACV40iH8d87zeB/hu/nu8N/BdPC/xokG86F4a+Cue1+sAv81/j9cGfovn9TLAX/MvQ7zoXhv4LZ7XywB/zX+P1wZ+i+f1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3hev81/r9fmeb0O8Nv8yxAvugcDT+d/h4cAt/IvQ/zrmP8dxIsG8a+zCxzjf7ZnAA/mRYP41/lu4L34n+17gPfmRYP413lr4Kf4n+1tgJ/mRYP41zkOXOR/thPALi8axL/eTwNvxf9MPwO8NS86xL/eawO/xf9MLwP8NS86xL/NdwPvxf8s3wO8N/86iH+bBwNP53+WhwC38q+D+Lf7bOCz+J/hc4DP5l8P8e/z08Bb8d/rZ4C35t8G8e9zHPht4KX47/E3wGsDu/zbIP79Xhr4beAY/7WeAbw18Nf82yH+YxwHfht4Kf5r/A3w2sAu/z6I/1jfDbwX/7m+B3hv/mMg/uO9NfDVwIP4j/UM4KOBn+Y/DuI/z3sDXw0c49/nGcBnA9/NfzzEf67jwFsDrw28NXCMF80l4KeB3wZ+GtjlPwfiv9ZrA6/NFceBl+aKvwZ2ueKngb/mvwbi/zfE/2+I/98Q/78h/n9D/P/GPwKm+KtBcSmQxAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOpacity;
impl IconShape for MdOpacity {
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
                d: "M17.66 8L12 2.35 6.34 8C4.78 9.56 4 11.64 4 13.64s.78 4.11 2.34 5.67 3.61 2.35 5.66 2.35 4.1-.79 5.66-2.35S20 15.64 20 13.64 19.22 9.56 17.66 8zM6 14c.01-2 .62-3.27 1.76-4.4L12 5.27l4.24 4.38C17.38 10.77 17.99 12 18 14H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFMElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeV4fA/w1/7O8NPBVPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/Psd54pd/v1eG/gtnpd4wRD/MvO8Xgf4bf59jgO/xRWvA+zy7/PawG/xvMQLhviXmef1OsBv8293HPgt4KW54q+B1wF2+bd7beC3eF7iBUP8y8zzeh3gt/m3OQ78FvDSPKe/Bl4H2OXf5rWB3+J5iRcM8S8zz+t1gN/mX+848FvAS/P8/TXwOsAu/3qvDfwWz0u8YIh/mXlerwP8Nv86x4HfAl6aF+6vgdcBdvnXeW3gt3he4gVD/MvM83od4Ld50R0Hfgt4aV40fw28DrDLi+61gd/ieYkXDPEvM8/rdYDf5kVzHPgt4KX51/lr4HWAXV40rw38Fs9LvGCIf5l5Xq8D/Db/suPAbwEvzb/NXwOvA+zyL3tt4Ld4XuIFQ/zLzPN6HeC3eeGOA78FvDT/Pn8NvA6wywv32sBv8bzEC4b4l5nn9TrAb/OCHQd+C3hp/mP8NfA6wC4v2GsDv8XzEi8Y4l9mntfrAL/NC/Zg4MG8YF8NvBTP6W+Aj+YFuxW4lRfstYHf4nmJFwzxLzPP63WA3+bf7reB1+I5/Q7w2vzbvTbwWzwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/MvM83od4Lf5t/tt4LV4Tr8DvDb/dq8N/BbPS7xgiH+ZeV6vA/w2/3a/DbwWz+l3gNfm3+61gd/ieYkXDPEvM8/rdYDf5t/ut4HX4jn9DvDa/Nu9NvBbPC/xgiH+ZeZ5vQ7w2/zb/TbwWjyn3wFem3+71wZ+i+clXjDEv8z8+4jn9dvAa/Gcfgd4bZ6X+fcRLxjiX/bXwEvxbyee128Dr8Vz+h3gtXle5t/ud4DX5gVD/MveG/gu/u3E8/pt4LV4Tr8DvDbPy/zbvQ/w3bxgiBfNdwPvxb+NeF6/DbwWz+l3gNfmeZl/m+8B3psXDvGi+2zgo4Fj/OuI5/XbwGvxnH4HeG2el/nXuQR8NfDZ/MsQ/3qvzb/Ob/O8Xho4znPaBf6a5/Xa/Ov8Ni86xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hH0z60QXAabuwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOpenInBrowser;
impl IconShape for MdOpenInBrowser {
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
                d: "M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h4v-2H5V8h14v10h-4v2h4c1.1 0 2-.9 2-2V6c0-1.1-.89-2-2-2zm-7 6l-4 4h3v6h2v-6h3l-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+M48NvAS/EfS7xgiP9ZjgO/DbwU/3HEC4b4n+c48NvAS/EfQ7xgiP+ZjgO/DbwU/37iBUP8z3Uc+G3gpfj3ES8Y4n+248BvAy/Fv514wRD/8x0Hfht4Kf5txAuG+N/hOPDbwEvxrydeMMT/HseBvwYexL+OeMEQ/3u8N/Bd/OuJFwzxv8N7A9/Fv414wRD/87038F3824kXDPE/23sD38UL9gzgQbxw4gVD/M/13sB38YK9D/DTwG8DL8ULJl4wxP9M7w18Fy/Y+wDfzRXHgd8GXornT7xgiP953hv4Ll6w9wG+m+d0HPht4KV4XuIFQ/zP8t7Ad/GCvQ/w3Tx/x4HfBl6K5yReMMT/HO8NfBcv2PsA380Ldxz4beCleDbxgiH+Z3hv4Lt4wd4H+G5eNMeB3wZeiivEC4b47/fewHfxgr0P8N386xwHfht4KUC8YIj/Xu8NfBcv2PsA382/zXHgt4GX5gVD/Pd5b+C7eMHeB/hu/n2OA7u8YIj/Hu8NfBcv2PsA381/PsR/vfcGvosX7H2A7+a/BuK/1nsD38UL9j7Ad/NfB/Ff572B7+IFex/gu/mvhfiXHQd2+fd5b+C7eMHeB/hu/ush/mV/BbwOsMu/zXsD38UL9j7Ad/PfA/EvM/DXwOsAu/zrvDfwXbxg7wN8N/99EP8yc8VfA68D7PKieW/gu3jB3gf4bv57If5l5tn+GngdYJcX7r2B7+IFex/gu/nvh/iXmef018DrALs8f+8NfBcv2PsA383/DIh/mXlefw28DrDLc3pv4Lt4wd4H+G7+50D8y8zz99fA6wC7XPHewHfxgr0P8N38z4L4l5kX7K+B1wHeGvguXrD3Ab6b/3kQ/zLzwt0KPJgX7H2A7+Z/JsS/zPzbvQ/w3fzPhfiXmX+b9wG+m//ZEP8y86/3PsB38z8f4l9m/nVuBV4G2OV/PsS/zPzr/TXwOsAu/7Mh/mXm3+avgdcBdvmfC/EvM/92fw28DrDL/0yIf5n59/lr4HWAXf7nQfzLzL/fXwOvA+zyPwviX2b+Y/w18DrALv9zIP5l5j/OXwOvA+zyPwPiX2b+Y/018DrALv/9EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHxj5+QVkVrh0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOpenInFull;
impl IconShape for MdOpenInFull {
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
                points: "21,11 21,3 13,3 16.29,6.29 6.29,16.29 3,13 3,21 11,21 7.71,17.71 17.71,7.71",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Y8jntdvA6/FfwzxgiFeNO8NfBf/OcTz+m3gtfiPIV4wxL/spYG/4j+PeF6/DbwW/zHEC4b4l3038F785xHP67eB1+I/hnjBEP+yi8Bx/vOI5/XbwGvxH0O8YIh/mXleHwP8Nf8xfpvn9dLAcV649wbei3+ZeMEQ/zLzvF4H+G3++7w38F28aMQLhviXmef1OsBv89/jvYHv4kUnXjDEv8w8r9cBfpv/eu8NfBf/OuIFQ/zLzPN6HeC3+a/13sB38YJ9D/BePC/xgiH+ZeZ5vQ7w2/zXeW/gu3jB3ge4Ffgtnpd4wRD/MvO8Xgf4bf5rvDfwXbxg7wN8N/DawG/xvMQLhviXmef1OsBv85/vvYHv4gV7H+C7ueK1gd/ieYkXDPEvM8/rdYDf5j/XewPfxQv2PsB382yvDfwWz0u8YIh/mXlerwP8Nv953hv4Ll6w9wG+m+f02sBv8bzEC4b4l5nn9TrAb/Of472B7+IFex/gu3lerw38Fs9LvGCIf5l5Xq8D/Db/8d4b+C5esPcBvpvn77WB3+J5iRcM8S8zz+t1gN/mP9Z7A9/FC/Y+wHfzgr028Fs8L/GCIf5l5nm9DvDb/Md5b+C7eMHeB/huXriXBr6a5/XavGCIf5l5Xq8D/Db/Md4b+C5esPcBvpv/HIh/mXlerwP8Nv9+7w18Fy/Y+wDfzX8exL/MPK/XAX6bf5/3Br6LF+x9gO/mPxfiX2ae1+sAv82/3XsD38UL9j7Ad/OfD/EvM8/rdYDf5t/mvYHv4gV7H+C7+a+B+JeZ5/U6wG/zr/fewHfxgr0P8N3827w28Fs8L/GCIf5l5nm9DvDb/Ou8N/BdvGDvA3w3/3avDfwWz0u8YIh/mXlerwP8Ni+69wa+ixfsfYDv5t/ntYHf4nmJFwzxLzPP63WA3+ZF897Ad/GCvQ/w3fz7vTbwWzwv8YIh/mXmeb0O8Nv8y94b+C5esPcBvpv/GK8N/BbPS7xgiH+ZeV6vA/w2L9x7A9/FC/Y+wHfzH+e1gd/ieYkXDPEvM8/rdYDf5gV7b+C7eMHeB/hu/mO9NvBbPC/xgiH+ZeZ5vQ7w2zx/Dwaezgv2PsB38x/vtYHf4nmJFwzxLzPP63WA3+YFe2/gu3he7wN8N/85Xhv4LZ6XeMEQ/zLzvF4H+G1euPcGvotnex/gu/nP89rAb/G8xAuG+JeZ5/U6wG/zL3tv4LuA9wG+m/9crw38Fs9LvGCIf5l5Xq8D/DYvmgcDt/Kf77WB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Phr4a/5neWngq3le4gVD/Mt2gWP873QJOM4LhviXfTfwXvzv9D3Ae/OCIf5lLw38Ff87PQS4lRcM8aJ5b+C7+N/lfYDv5oVDvOgeDHw28NbAMf5nugT8NPDZwK38yxD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R5OCuEET+V0RAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOpenInNew;
impl IconShape for MdOpenInNew {
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
                d: "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF0UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13Gu2OW/B+K/z3Hgt7jidYBd/ush/nscB34LeGmu+GvgdYBd/msh/usdB34LeGme018DrwPs8l8H8V/rOPBbwEvz/P018DrALv81EP91jgO/Bbw0L9xfA68D7PKfD/Ff4zjwW8BL86L5a+B1gF3+cyH+8x0Hfgt4af51/hp4HWCX/zyI/1zHgd8CXpp/m78GXgfY5T8H4j/PceC3gJfm3+evgdcBdvmPh/jPcRz4LeCl+Y/x18DrALv8x0L8xzsO/Bbw0vzH+mvgdYBd/uMg/mMdB34LeGn+c/w18DrALv8xEP9xjgO/Bbw0/7n+GngdYJd/P8R/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsS/z3FglxfNawO/xYvmdYDf5kVzHNjl3wbxb3cc+C3gZXjRvDbwW7xoXgf4bV40fwW8DrDLvx7i3+Y48FvASwPiRfPawG/xonkd4Ld50Rj4a+B1gF3+dRD/eseB3wJemivEi+a1gd/iRfM6wG/zojFX/DXwOsAuLzrEv85x4LeAl+bZxIvmtYHf4kXzOsBv86Ixz/bXwOsAu7xoEC+648BvAS/NcxIvmtcGfosXzesAv82LxjynvwZeB9jlX4Z40RwHfgt4aZ6X+O9lntdfA68D7PLCIf5lx4HfAl6a50/89zLP318DrwPs8oIhXrjjwG8BL80L9tq8cL/Dv89r8cL9Ni/YXwOvA+zy/CFesOPAbwEvzb+P+Pcx/z5/DbwOsMvzQjx/x4HfAl6afz/x72P+/f4aeB1gl+eEeF7Hgd8CXpr/GOLfx/zH+GvgdYBdng3xnI4DvwW8NP9xxL+P+Y/z18DrALtcgXi248BvAS/Nfyzx72P+Y/018DrALoB4tuPAbwMvxX8s8e9j/mP9DfDawC6AeE7Hgd8GXor/OOLfx/zH+RvgtYFdrkA8r+PAbwMvxX8M8e9j/mP8DfDawC7Phnj+jgO/DbwU/37i38f8+/0N8NrALs8J8YIdB34beCn+fcS/j/n3+RvgtYFdnhfihTsO/DbwUrxgr8ML99v8+7w2L9xv8YL9DfDawC7PH+Jfdhz4beCleP7Efy/z/P0N8NrALi8Y4kVzHPht4KV4XuK/l3lefwO8NrDLC4d40R0Hfht4KZ6TeNG8NvBbvGheB/htXjTmOf0N8NrALv8yxL/OceC3gZfi2cSL5rWB3+JF8zrAb/OiMc/2N8BrA7u8aBD/eseB3wZeiivEi+a1gd/iRfM6wG/zojFX/A3w2sAuLzrEv81x4LeBlwLEi+a1gd/iRfM6wG/zojHwN8BrA7v86yD+7Y4Dvw28NC+a1wZ+ixfN6wC/zYvmr4HXBnb510P8+xwHdnnRvDbwW7xoXgf4bV40x4Fd/m0Q/3VeG/gtXjSvA/w2//kQ/3VeG/gtXjSvA/w2//kQ/3GOA78NvBT/uf4GeG1gl38/xH+s48BvAy/Ff46/AV4b2OU/BuI/3nHgt4GX4j/W3wCvDezyHwfxn+M48NvAS/Ef42+A1wZ2+Y+F+M9zHPht4KX49/kb4LWBXf7jIf5zHQd+G3gp/m3+BnhtYJf/HIj/fMeB3wZein+dvwFeG9jlPw/iv8Zx4LeBl+JF8zfAawO7/OdC/Nc5Dvw28FK8cH8DvDawy38+xH+t48BvAy/F8/c3wGsDu/zXQPzXOw78NvBSPKe/AV4b2OW/DuK/x3Hgt4GX4oq/AV4b2OW/FuK/z3Hgt7nitYFd/ush/nsd54pd/nsg/n9D/P/GPwJFT9pBka9uVgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOpenWith;
impl IconShape for MdOpenWith {
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
                d: "M10 9h4V6h3l-5-5-5 5h3v3zm-1 1H6V7l-5 5 5 5v-3h3v-4zm14 2l-5-5v3h-3v4h3v3l5-5zm-9 3h-4v3H7l5 5 5-5h-3v-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxH+evgY/h3+alga/iX+9zgM/m+UO8cE8HHsx/rO8G3od/vdcGfot/vVuBh/D8IV6wtwZ+iv8c3w28D/86rw38Fv82rwP8Ns8L8YJ9NfBR/Of5buB9eNG9NvBb/Nt8DfDRPC/EC/ZXwEvzn+u7gffhRfPawG/xb/PXwMvwvBDP34OBp/Nf47uB9+Ff9trAb/FvdwLY5Tkhnr/XBn6L/xp/A7w2sMsL99rAb/Fv9zrAb/OcEM/fRwNfxX++vwFeG9jlX/bawG/xb/cxwFfznBDP32cDn8V/rr8BXhvY5UXz2sBv8W/3OcBn85wQz99nA5/Ff56/AV4b2OVF99rAb/Fv9znAZ/OcEM/fbwOvxX+OvwFeG9jl+fsurngfntNrA7/Fv93PAG/Nc0I8f78NvBb/8f4GeG1gl+fvu4D35orvBt6HZ3tt4Lf4t/sZ4K15Tojn77OBz+I/1t8Arw3s8vx9F/DePKfvBt6HK14b+C3+7T4H+GyeE+L5+2zgs/iP8zfAawO7PH/fBbw3z993A+8DvDbwW/zbfQ7w2TwnxPP30cBX8R/jb4DXBnZ5/r4LeG9euO8Gvgf4Lf7tPgb4ap4T4vl7beC3+Pf7G+C1gV2ev+8C3psXzV8DL82/3esAv81zQjx/Dwaezr/P3wCvDezy/H0X8N781zkB7PKcEC/YXwMvxb/N3wCvDezy/H0X8N781/kb4KV5XogX7KuBj+Jf72+A1wZ2ef6+C3hv/mt9DfDRPC/EC/bawG/xr/cxwFfz/H0X8N7813sd4Ld5XogX7lbgQfzrvQ/w3Tyn7wLem/96zwAezPOHeOE+G/gs/m3eB/hurvgu4L357/E5wGfz/CFeuOPArcAx/m3eB3gt4L3573EJeDCwy/OH+Jd9NvBZ/O/0OcBn84Ih/mXHgVuBY/zvcgl4MLDLC4Z40bw38F387/I2wE/zwiFedD8NvBX/O/wM8Nb8yxAvuuPAXwMP4n+2ZwAvDezyL0P867w08NvAMf5nugS8NvDXvGgQ/3ovDfw2cIz/WS4Brw38NS86xL/NSwO/DRzjf4ZLwGsDf82/DuLf7qWBnwYexH+vZwBvDfw1/3qIf5/jwHcDb8V/j58B3hvY5d8G8R/jvYGvBo7xX+MS8N7AT/Pvg/iPcxz4aOCjgWP857gEfDXw1cAu/36I/3jHgY8G3ht4EP8xngF8N/DVwC7/cRD/uV4beGvgtYGX4l/nb4DfBn4a+G3+cyD+6xwHXhp4aeA4V7w0V/w1V+wCfw38NbDLfz7E/2+I/98Q/78h/n9D/P/GPwJ2itRBxbtMYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutbond;
impl IconShape for MdOutbond {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.88,11.54l-4.96,4.96l-1.41-1.41 l4.96-4.96L10.34,8l5.65,0.01L16,13.66L13.88,11.54z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FnAawPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/Zg4Gn87/TQ4BbecEQ/7LvBt6L/52+B3hvXjDEv+wicJz/nXaBE7xgiH+ZeV4fA/w1/7O8NPBVPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/Psd54pd/v1eG/gtnpd4wRD/MvO8Xgf4bf59jgO/xRWvA+zy7/PawG/xvMQLhviXmef1OsBv8293HPgt4KW54q+B1wF2+bd7beC3eF7iBUP8y8zzeh3gt/m3OQ78FvDSPKe/Bl4H2OXf5rWB3+J5iRcM8S8zz+t1gN/mX+848FvAS/P8/TXwOsAu/3qvDfwWz0u8YIh/mXlerwP8Nv86x4HfAl6aF+6vgdcBdvnXeW3gt3he4gVD/MvM83od4Ld50R0Hfgt4aV40fw28DrDLi+61gd/ieYkXDPEvM8/rdYDf5kVzHPgt4KX51/lr4HWAXV40rw38Fs9LvGCIf5l5Xq8D/Db/suPAbwEvzb/NXwOvA+zyL3tt4Ld4XuIFQ/zLzPN6HeC3eeGOA78FvDT/Pn8NvA6wywv32sBv8bzEC4b4l5nn9TrAb/OCHQd+C3hp/mP8NfA6wC4v2GsDv8XzEi8Y4l9mntfrAL/NC/Zg4ME8p5cGvooXzccAf81zuhW4lRfstYHf4nmJFwzxLzPP63WA3+Zf57WB3+JF8zrAb/Ov89rAb/G8xAuG+JeZ5/U6wG/zr/PawG/xonkd4Lf513lt4Ld4XuIFQ/zLzPN6HeC3+dd5beC3eNG8DvDb/Ou8NvBbPC/xgiH+ZeZ5vQ7w2/zrvDbwW7xoXgf4bf51Xhv4LZ6XeMEQ/zLzvF4H+G3+dV4b+C1eNK8D/Db/Oq8N/BbPS7xgiH+ZeV6vA/w2/zqvDfwWL5rXAX6bf53XBn6L5yVeMMS/zDyv1wF+m3+d1wZ+ixfN6wC/zb/OawO/xfMSLxjiX2ae1+sAv82/znHgpXnR/DWwy7/OawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZebfR/z7mH8f8YIh/mXm3+dlgL/m3+algb/i30e8YIh/mfn3+Rjgq/m3+Wjgq/j3ES8Y4l9m/n1uBR7Cv83TgQfz7yNeMMS/zPz7fQzw1fzrfDTwVfz7iRcM8S8z/zFeBvhrXjQvDfwV/zHEC4b4l5n/OB8NfA0v3EcBX81/HPGCIf5l5j/WrcBXA78D/DVXvDTwWsBHAw/mP5Z4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/sl3gGP87XQKO84Ih/mU/DbwV/zt9D/DevGCIf9lLA3/F/04PAW7lBUO8aN4b+C7+d3kf4Lt54RAvupcGPhp4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjRJC3QTVQu+MAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutbox;
impl IconShape for MdOutbox {
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
                d: "M19 3H4.99c-1.11 0-1.98.9-1.98 2L3 19c0 1.1.88 2 1.99 2H19c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 12h-4c0 1.66-1.35 3-3 3s-3-1.34-3-3H4.99V5H19v10zM8 11h2v3h4v-3h2l-4-4-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutgoingMail;
impl IconShape for MdOutgoingMail {
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
                d: "M18.5,11c0.17,0,0.34,0.01,0.5,0.03V6.87C19,5.84,18.16,5,17.13,5H3.87C2.84,5,2,5.84,2,6.87v10.26 C2,18.16,2.84,19,3.87,19h9.73C13.22,18.25,13,17.4,13,16.5C13,13.46,15.46,11,18.5,11z M10.4,13L4,9.19V7h0.23l6.18,3.68L16.74,7 H17v2.16L10.4,13z",
            }
            polygon {
                points: "19,13 17.59,14.41 19.17,16 15,16 15,18 19.17,18 17.59,19.59 19,21 23,17",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutlet;
impl IconShape for MdOutlet {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M9,12c-0.55,0-1-0.45-1-1V8 c0-0.55,0.45-1,1-1s1,0.45,1,1v3C10,11.55,9.55,12,9,12z M14,18h-4v-2c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2V18z M16,11 c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1V11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPageview;
impl IconShape for MdPageview {
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
                d: "M11.5 9C10.12 9 9 10.12 9 11.5s1.12 2.5 2.5 2.5 2.5-1.12 2.5-2.5S12.88 9 11.5 9zM20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-3.21 14.21l-2.91-2.91c-.69.44-1.51.7-2.39.7C9.01 16 7 13.99 7 11.5S9.01 7 11.5 7 16 9.01 16 11.5c0 .88-.26 1.69-.7 2.39l2.91 2.9-1.42 1.42z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPanTool;
impl IconShape for MdPanTool {
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
                d: "M23,5.5V20c0,2.2-1.8,4-4,4h-7.3c-1.08,0-2.1-0.43-2.85-1.19L1,14.83c0,0,1.26-1.23,1.3-1.25 c0.22-0.19,0.49-0.29,0.79-0.29c0.22,0,0.42,0.06,0.6,0.16C3.73,13.46,8,15.91,8,15.91V4c0-0.83,0.67-1.5,1.5-1.5S11,3.17,11,4v7 h1V1.5C12,0.67,12.67,0,13.5,0S15,0.67,15,1.5V11h1V2.5C16,1.67,16.67,1,17.5,1S19,1.67,19,2.5V11h1V5.5C20,4.67,20.67,4,21.5,4 S23,4.67,23,5.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe2vgq4AH87/TrcDHAD/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5GuCjeU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4TrvAX/O/w0sDx3lOvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/H038F783/I1wEfznBDP30cDX8X/LR8DfDXPCfH8HQduBY7xf8Ml4MHALs8J8YK9N/Bd/N/wPsB387wQL9xbA18NPIj/nZ4BfDTw0zx/iBfNSwPH+d9lF/hrXjjE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EdeLohBa44cLQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPayment;
impl IconShape for MdPayment {
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
                d: "M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFh0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5v+UEsMtzQjx/rw38Fv+3vA7w2zwnxPP30cBX8aL5HeCzgd8GXhp4a+CzeOGeAXw08Ndc8dLAVwMP4oX7HOCngb8GXhv4bOC1eNF8DPDVPCfE8/fZwGfxL/se4L15Xq8N/BbP3zOAlwZ2eU7Hgb8GHsTz9zrAb/O8vht4L/5lnwN8Ns8J8fx9NvBZ/MtOALs8f98NvBfP622An+b5e2vgp3he3wO8N8/fceAi/7LPAT6b54R4/n4beC1euN8BXpsX7KOBr+J5nQB2ef4eDDyd5/UxwFfzgv028Fq8cD8DvDXPCfH8/TbwWrxwvwO8Ni/YRwNfxfM6Aezy/D0YeDrP62OAr+YF+23gtXjhfgZ4a54T4vn7bOCz+JedAHZ5/r4beC+e19sAP83z99bAT/G8vgd4b56/48BF/mWfA3w2zwnx/H028Fn8y74beB+e12sDv8XzdyvwMsAuz+k48FfAg3n+Xgf4bZ7XdwHvzb/sc4DP5jkhnr+PBr6KF81vA58N/A7w0sBbAZ/NC3cr8NHA33DFSwFfDTyYF+6zgZ8B/hp4LeCzgdfmRfMxwFfznBDP32sDv8X/La8D/DbPCfH8PRh4Ov+3nAB2eU6IF+yvgZfi/4a/AV6a54V4wb4a+Cj+b/ga4KN5XogX7LWB3+L/htcBfpvnhXjhbgUexP9uzwAezPOHeOE+G/gs/nf7HOCzef4QL9xx4FbgGP87XQIeDOzy/CH+ZZ8NfBb/O30O8Nm8YIh/2XHgVuAY/7tcAh4M7PKCIV407w18F/+7vA3w07xwiBfdTwNvxf8OPwO8Nf8yxIvuOPDXwIP4n+0ZwEsDu/zLEP86Lw38NnCM/5kuAa8N/DUvGsS/3ksDvw0c43+WS8BrA3/Niw7xb/PSwG8Dx/if4RLw2sBf86+D+Ld7aeCngQfx3+sZwFsDf82/HuLf5zjw3cBb8d/jZ4D3Bnb5t0H8x3hv4KuBY/zXuAS8N/DT/Psg/uMcBz4a+GjgGP85LgFfDXw1sMu/H+I/3nHgo4H3Bh7Ef4xnAN8NfDWwy38cxH+u1wbeGnht4KX41/kb4LeBnwZ+m/8ciP86x4GXBl4aOM4VL80Vf80Vu8BfA38N7PKfD/H/G+L/N8T/b4j/3xD/v/GPhvDIQdjq0+IAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPending;
impl IconShape for MdPending {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M7,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C8.5,12.83,7.83,13.5,7,13.5z M12,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C13.5,12.83,12.83,13.5,12,13.5z M17,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C18.5,12.83,17.83,13.5,17,13.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPendingActions;
impl IconShape for MdPendingActions {
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
                d: "M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermCameraMic;
impl IconShape for MdPermCameraMic {
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
                d: "M20 5h-3.17L15 3H9L7.17 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7v-2.09c-2.83-.48-5-2.94-5-5.91h2c0 2.21 1.79 4 4 4s4-1.79 4-4h2c0 2.97-2.17 5.43-5 5.91V21h7c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm-6 8c0 1.1-.9 2-2 2s-2-.9-2-2V9c0-1.1.9-2 2-2s2 .9 2 2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEve2ngr/jf6SHArbxgiH/ZdwPvxf9O3wO8Ny8Y4l92ETjO/067wAleMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzHegbw1cBvA3/NFS8NvDbw0cCD+I8lXjDEv8z8x/kY4Kt54T4a+Cr+44gXDPEvM/8xXgb4a140Lw38Ff8xxAuG+JeZf7+PAb6af52PBr6Kfz/xgiH+Zebf5xnAg/m3uRV4EP8+4gVD/MvMv8/HAF/Nv81HA1/Fv494wRD/MvPv8zLAX/Nv89LAX/HvI14wxL/M/PuIfx/z7yNeMMS/zPz7iH8f8+8jXjDEv8z8+7wM8Nf827w08Ff8+4gXDPEvM/8+HwN8Nf82Hw18Ff8+4gVD/MvMv8+twEP4t3k68GD+fcQLhviXmX+/jwG+mn+djwa+in8/8YIh/mXmP8bLAH/Ni+algb/iP4Z4wRD/MvMf56OBr+GF+yjgq/mPI14wxL/M/Me6Ffhq4HeAv+aKlwZeC/ho4MH8xxIvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+Y/wN8NvArcBfc8Vvc8Vrc8VLAw8GXht4Kf5jiBcM8S8z/3bfA/w08NvALv86x4HXBt4aeC/+7cQLhviXmX+dS8BXA18N7PIf4zjw0cBHA8f41xEvGOJfZl50nwN8NbDLf47jwEcDn8WLTrxgiH+Z+ZddAt4a+G3+a7w28NPAMf5l4gVD/MvMv+xlgL/mv9ZLA3/Fv0y8YIh/mXnhPgf4bP57fDbwWbxw4gVD/MvMC/cywF/z3+Olgb/ihRMvGOJfZl448d/LvHDiBUP8y8wLJ/57mRdOvGCIf5l54cR/L/PCiRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S/bBY7xv9Ml4DgvGOJf9t3Ae/G/0/cA780LhviXvTTwV/zv9BDgVl4wxIvmvYHv4n+X9wG+mxcO8aJ7MPDZwFsDx/if6RLw08BnA7fyL0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHPE45BR/MLcQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermContactCalendar;
impl IconShape for MdPermContactCalendar {
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
                d: "M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 3c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermDataSetting;
impl IconShape for MdPermDataSetting {
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
                d: "M18.99 11.5c.34 0 .67.03 1 .07L20 0 0 20h11.56c-.04-.33-.07-.66-.07-1 0-4.14 3.36-7.5 7.5-7.5zm3.71 7.99c.02-.16.04-.32.04-.49 0-.17-.01-.33-.04-.49l1.06-.83c.09-.08.12-.21.06-.32l-1-1.73c-.06-.11-.19-.15-.31-.11l-1.24.5c-.26-.2-.54-.37-.85-.49l-.19-1.32c-.01-.12-.12-.21-.24-.21h-2c-.12 0-.23.09-.25.21l-.19 1.32c-.3.13-.59.29-.85.49l-1.24-.5c-.11-.04-.24 0-.31.11l-1 1.73c-.06.11-.04.24.06.32l1.06.83c-.02.16-.03.32-.03.49 0 .17.01.33.03.49l-1.06.83c-.09.08-.12.21-.06.32l1 1.73c.06.11.19.15.31.11l1.24-.5c.26.2.54.37.85.49l.19 1.32c.02.12.12.21.25.21h2c.12 0 .23-.09.25-.21l.19-1.32c.3-.13.59-.29.84-.49l1.25.5c.11.04.24 0 .31-.11l1-1.73c.06-.11.03-.24-.06-.32l-1.07-.83zm-3.71 1.01c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermDeviceInformation;
impl IconShape for MdPermDeviceInformation {
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
                d: "M13 7h-2v2h2V7zm0 4h-2v6h2v-6zm4-9.99L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv89r8Zx+h/96iP9abw28F/DWPH8/DXwP8NP810D813hp4KuA1+ZF89vAxwB/zX8uxH++lwZ+CzjOv84u8DrAX/OfB/Gf66WB3wKO82+zC7wO8Nf850D85zkO/BXwYJ6/S8Bfc8VLA8d4/m4FHsJ/DsR/ns8GPovndQn4aOC7eU7vDXw1cIzn9TnAZ/MfD/Gf5yJwnOf0N8BrA7s8f8eBvwYexHPaBU7wHw/xn+O1gd/ieb0M8Ne8cK8N/BbP62WAv+Y/FuI/x2cDn8Vz+hngrXnR/DTwVjynzwE+m/9YiP8cnw18Fs/pc4DP5kXz2cBn8Zw+B/hs/mMh/nP8NPBWPKfPAT6bF81nA5/Fc/oZ4K35j4X4z/HZwGfxnL4G+GheNF8NfBTP6XOAz+Y/FuI/x0cDX8VzuhV4CC+apwMP5jl9DPDV/MdC/Od4MPB0ntfHAF/NC/fRwFfxvB4C3Mp/LMR/nt8GXovn9T7Ad/P8vTfwXTyv3wFem/94iP88rw38Fs/fTwPfDfwNV7wU8N7AW/P8vQ7w2/zHQ/zn+m7gvfj3+R7gvfnPgfjPdRz4beCl+Lf5G+C1gV3+cyD+8x0Hfhp4Lf51fgd4a2CX/zyI/zofDXw2cIwX7hLw2cBX858P8V/rOPDWwFsDDwZeiiv+BrgV+Gngp4Fd/msg/n9D/P+G+K93HHgpnr+/AXb5r4P4z/XSwGsBLw08GHhtXjS/DdwK/DXwO8Bf858D8R/vrYC3Bt4aOM5/jF3gp4GfBn6G/ziI/xgPBj4LeGvgOP+5doGfBj4HuJV/H8S/z4OBzwLem/8e3w18DnAr/zaIf7vPAj6bf5tnALfynB4MPIh/m88GPod/PcS/3nHgp4DX5kXzO8BvA78N3Arcygv3YODBwGsDrw28Fi+a3wbeBtjlRYf41/sr4KV54X4G+Gngp4Fd/n2OA28NvDXwVrxwfw28DC86xL/OZwOfxfN3Cfhq4KuBXf5zHAc+Gvho4BjP3+cAn82LBvGvcxE4zvP6HOCrgV3+axwHPhr4LJ7XLnCCFw3iRffawG/xvF4H+G3+e7w18FM8r9cBfpt/GeJF99rAb/Gcfgd4bf57/TbwWjyn1wF+m38Z4kX32sBv8Zx+B3ht/nv9NvBaPKfXAX6bfxniRffawG/xnH4HeG3+e/028Fo8p9cBfpt/GeJF99rAb/GcdoG/5r/XSwPHeU6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86I4DL83/Dn8N7PIvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BDrCnUHXXvkyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermIdentity;
impl IconShape for MdPermIdentity {
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
                d: "M12 5.9c1.16 0 2.1.94 2.1 2.1s-.94 2.1-2.1 2.1S9.9 9.16 9.9 8s.94-2.1 2.1-2.1m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 9c-2.67 0-8 1.34-8 4v3h16v-3c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFC0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw/z4fA/w1/z0Q/3rvDXwVcJz/GLvA6wB/zX89xL/OawO/xX+8XeB1gL/mvxbiX+fpwIP5z7ELvA7w1/zXQbzoXhr4K/5z7QKvA/w1/zUQL7rXBn6L/3y7wHsDu/zn+B2eDfGie23gt/jfbxf4auBzAMSL7rWB3+L/ju8G3ke86F4b+C3+b3kf8aJ7beC3+L/ld8SL7rWB3+L/FgSY5/U6wG/znF4b+C3+b0GAeV6vA/w2z+m1gd/i/xYEmOf1OsBv85xeG/gt/m9BgHlerwP8Ns/ptYHf4v8WBJjn9TrAb/OcXhv4Lf5vQYB5Xq8D/DbP6bWB3+L/FgSY5/U6wG/znF4b+C3+b0GAeV6vA/w2z+m1gd/i/xYEmOf1OsBv85xeG/gt/uO8D/DZwIP474MA87xeB/htntNrA7/Ff4yvAT4aeGngr/jvgwDzvF4H+G2e02sDv8W/398AL82zvTfwXfz3QIB5Xq8D/DbP6bWB3+Lf5xLw0sCtPKfvBt6L/3oIMM/rdYDf5jm9NvBb/Pu8DfDTPK/jwG8DL8WL7m2ArwYexL8dAszzeh3gt3lOrw38Fv92XwN8NC/Yg4G/Bo7xL3sf4LuBlwb+in87BJjn9TrAb/OcXhv4Lf5t/gZ4af5lbw38FC/c9wDvzbO9N/Bd/NsgwDyv1wF+m+f02sBv8a93CXhp4FZeNJ8NfBbP3+8Ar83z+m7gvfjXQ4B5Xq8D/DbP6bWB3+Jf722An+Zf57eB1+I5/Q3w2sAuz+s48NvAS/GvgwDzvF4H+G2e02sDv8W/ztcAH82/3nHgr4EHccUl4LWBv+YFezDw18AxXnQIMM/rdYDf5jm9NvBbvOj+Bnhp/u1eGvgrrngd4Lf5l7018FO86BBgntfrAL/Nc3pt4Ld40VwCXhq4lX+f9+aK7+ZF99nAZ/GiQYB5Xq8D/DbP6bWB3+JF8zbAT/Pf57eB1+JfhgDzvF4H+G2e02sDv8W/7GuAj+a/13Hgr4EH8cIhwDyv1wF+m+f02sBv8cL9DfDS/M/w0sBf8cIhwDyv1wF+m+f02sBv8YJdAl4auJX/Od4b+C5eMASY5/U6wG/znF4b+C3+b0GAeV6vA/w2z+m1gd/i/xYEmOf1OsBv85xeG/gt/m9BgHlerwP8Ns/ptYHf4v+WSwLM83od4Ld5XrvAMf7v+B4B5nm9DvDbPK/PBj6L/zseIsA8r9cBfpvn77uB9+J/v/cBvluAeV6vA/w2L9h7Ax8NvBT/u1wCfhr4bOBWAAHmeb0O8Nv834cA87xeB/ht/u9DgHlerwP8Nv/3IcA8r9cBfpv/+xBgntfrAL/N/30IMM/rr4Fd/vf6GOCv+Zch4FbgQfzf8jrAb/MvQ8BnA5/F/y2vA/w2/zIEHAd+G3gp/u94HeC3+Zchrngw8NPAS/F/w+sAv82/DPGc3hp4aeC1+d/to4G/5l+G+P8N8f8b4v83xP9viP/f+Ec/gMv1yTAN1gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermMedia;
impl IconShape for MdPermMedia {
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
                d: "M2 6H0v5h.01L0 20c0 1.1.9 2 2 2h18v-2H2V6zm20-2h-8l-2-2H6c-1.1 0-1.99.9-1.99 2L4 16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM7 15l4.5-6 3.5 4.51 2.5-3.01L21 15H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwOLxrzn0u8YIh/2XcD78W/3tsAP82/zPznEi8Y4l9m/m2+B3hv/mXmP5d4wRD/MvNv8zvAa/MvM/+5xAuG+JeZf5vfAV6bf5n5zyVeMMS/7K+Bl+Jf73eA1+ZfZv5ziRcM8S/7beC1+LcR/zLzn0u8YIh/2W8Dr8W/jfiXmf9c4gVD/Mu+Gvgo/m1eB/htXjjzn0u8YIh/2WcDn8W/zccAX80LZ/5ziRcM8S97beC3+Lf5GeCteeHMfy7xgiH+ZQ8Gns6/nXjhzH8u8YIhXjS3Ag/i3+Z1gN/mBTP/ucQLhnjRfDfwXvzbvA7w27xg5j+XeMEQL5r3Br6Lf72/AV6aF8785xIvGOJF82Dg6fzr/A3w2sAuL5z5zyVeMMSL7lbgQbxo/gZ4bWCXf5n5zyVeMMSL7qOBr+Jf9jfAawO7vGjMfy7xgiFedA8Gns4L9zfAawO7vOjMC3YJ+Gv+fV6bFwzxr/PbwGvx/P0N8NrALv865vm7BLw28Nf850H867w38F08r2cALw3s8q9nntcl4LWBv+Y/F+Jf71bgQTynXeBlgFv51zPP6RLw2sBf858P8a/33sB38by+B3hv/vXMs10CXhv4a/5rIP5tbgUexPN6GeCv+dcxV1wCXhv4a/7rIP5t3hv4Lp7XXwMvw7+OgUvAawN/zX8txL/dbwOvxfP6HOCzedHtAq8N/DX/9RD/di8N/BXP3+sAv82L5qWBv+a/B+Lf56uBj+J57QIPAXb5nw3x73Mc+G3gpXhefw28DrDL/1yIf7+XBn4bOMbz+m7gffifC/Ef472B7+L5+27gffifCfEf57uB9+L5+27gffj3eS2u+B3+4yD+Y/028Fo8fz8NvA+wy7/OceC3gJfm2X4a+Gvgt4Hf4d8O8R/rOPDbwEvx/P018DbArbxojgO/Bbw0/7Lf5tluBX4G+GleOMR/vOPAbwMvxfO3C7wN8Nu8cMeB3wJemn+7zwE+mxcM8Z/jOPDbwEvxgn018DnALs/rOPBbwEvz73Mr8BBeMMR/nuPATwOvxQt2K/A+wG/zbMeB3wJemv8Y4gVD/Of7buC9eOG+G/gcYBf4LeCl+Y8jXjDEf433Br4aOMYLdyvwYP5jiRcM8V/npYHvBl6K/1riBUP81zoOfDbwUfzXES8Y4r/HSwNfDbwW//nEC4b47/XewGcDD+I/j3jBEP8zvDfw2cCD+I8nXjDE/yzvDbw18Fb8xxEvGOJ/pgcDbw18NPAg/n3EC4b4n+/BwGsDrw28NvAgXnTfA7w3Lxjif58HAw8GXhp4MPDSXHEceCme7XuAjwZ2ecEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Edw/aBBFVxmeAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermPhoneMsg;
impl IconShape for MdPermPhoneMsg {
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
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.58l2.2-2.21c.28-.27.36-.66.25-1.01C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM12 3v10l3-3h6V3h-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/Ng4EHASwPHgdfm2V4aOM5z2gX+mmf7bWAX+GvgGcCt/MdD/Md5aeC1gNcGXhp4MP+xbgV+G/hr4HeAv+bfD/Hv89LAewFvDTyY/1q3Aj8NfA/w1/zbIP71jgMfBbw38GD+Z7gV+Grge4BdXnSIF91LAx8FvDf/c+0CPw18DnAr/zLEv+zBwFcBb83/Lt8NfA5wKy8Y4gV7MPBZwHvzv9tXA58D7PK8EM/fRwGfDRzn/4Zd4LOBr+E5IZ7TSwPfBbw0/7leB/htntNrA7/Ff67fBj4G+GuuQDzbZwGfzX+N1wF+m+f02sBv8V/jo4GvARDwYOCngJfmv87rAL/Nc3pt4Lf4r/PbwNsI+G3gtfiv9TrAb/OcXhv4Lf5r/Y6A48CtwDH+67wO8Ns8p9cGfov/OpeAlxZXvDTwV/zX+Wtgl+d0HHhp/uu8DvDb4tneG/gu/n94H+C7AcRz+m7gvfi/7XuA9+YKxPP6beC1+L/pb4CX5tkQz+s48NfAg/i/5RLwYGCXZ0M8fy8N/DZwjP8cfwPs8pyOAy/Ff45LwGsDf81zQrxg7w18F/85Xgf4bZ7TawO/xX+O9wG+m+eFeOE+G/gs/uO9DvDbPKfXBn6L/3hfA3w0zx/iX/bTwFvxH+t1gN/mOb028Fv8x/od4LV5wRD/suPAbwMvxX+c1wF+m+f02sBv8R/nb4DXBnZ5wRAvmpcGfhs4xn+M1wF+m+f02sBv8R/jEvDawF/zwiFedK8N/Bb/MV4H+G2e02sDv8V/jLcBfpp/GeJf56OBr+Lf73WA3+Y5vTbwW/z7fQzw1bxoEP963w28F/8+rwP8Ns/ptYHf4t/ne4D35kWH+Nc7Dvw28FL8270O8Ns8p9cGfot/u78BXhvY5UWH+Lc5DtwKHOPf5nWA3+Y5vTbwW/zbXAJeGriVfx3Ev91LA3/Fv83rAL/Nc3pt4Lf4t3kZ4K/510P8+7w38F38670O8Ns8p9cGfot/vfcBvpt/G8S/33cD78W/zusAv81zem3gt/jX+R7gvfm3Q/zH+G3gtXjRvQ7w2zyn1wZ+ixfd3wAvzb8P4j/GceCvgQfxonkd4Ld5Tq8N/BYvmmcALw3s8u+D+I/z0sBvA8f4l3008Nc8p5cGvpp/2SXgtYG/5t8P8R/rvYHv4j/X+wDfzX8MxH+8zwY+i/8cXwN8NP9xEP85fhp4K/5j/Qzw1vzHQvznOA78NvBS/Mf4G+C1gV3+YyH+87w08NvAMf59LgGvDfw1//EQ/7leG/gt/n1eB/ht/nMg/vN9NPBV/Nt8DPDV/OdB/Nf4buC9+Nf5HuC9+c+F+K9xHPht4KV40fwN8NrALv+5EP91jgO3Asd44S4BDwZ2+c+H+K/10sBf8cK9DPDX/NdA/Nd7b+C7eP7eB/hu/usg/nt8N/BePKfvAd6b/1qI/z6/DbwWV/wO8Nr810P89zkO/DVXvDSwy389xH+vl+aKv+a/B+L/N8T/b4j/3xD/vyH+f+MfAZcgsTFCiB0lAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPermScanWifi;
impl IconShape for MdPermScanWifi {
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
                d: "M12 3C6.95 3 3.15 4.85 0 7.23L12 22 24 7.25C20.85 4.87 17.05 3 12 3zm1 13h-2v-6h2v6zm-2-8V6h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPets;
impl IconShape for MdPets {
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
            circle {
                cx: "4.5",
                cy: "9.5",
                r: "2.5",
            }
            circle {
                cx: "9",
                cy: "5.5",
                r: "2.5",
            }
            circle {
                cx: "15",
                cy: "5.5",
                r: "2.5",
            }
            circle {
                cx: "19.5",
                cy: "9.5",
                r: "2.5",
            }
            path {
                d: "M17.34 14.86c-.87-1.02-1.6-1.89-2.48-2.91-.46-.54-1.05-1.08-1.75-1.32-.11-.04-.22-.07-.33-.09-.25-.04-.52-.04-.78-.04s-.53 0-.79.05c-.11.02-.22.05-.33.09-.7.24-1.28.78-1.75 1.32-.87 1.02-1.6 1.89-2.48 2.91-1.31 1.31-2.92 2.76-2.62 4.79.29 1.02 1.02 2.03 2.33 2.32.73.15 3.06-.44 5.54-.44h.18c2.48 0 4.81.58 5.54.44 1.31-.29 2.04-1.31 2.33-2.32.31-2.04-1.3-3.49-2.61-4.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v8x3tp4Bj/sS4Bf83zMs/rdYDfBhDP9trAb/G8xH+83wZei/9YvwO8Ns/LPK/XAX4bQDzbawO/xfMS//F+G3gt/mP9DvDaPC/zvF4H+G0A8WyvDfwWz0v8x/tt4LX4j/U7wGvzvMzzeh3gtwHEs7028Fs8L/Ef77eB1+I/1u8Ar83zMs/rdYDfBhDP9trAb/G8xH+83wZei/9YvwO8Ns/LPK/XAX4bQDzbawO/xfMS//F+G3gt/mP9DvDaPC/zvF4H+G0A8WyvDfwWz0v8x/tt4LX4j/U7wGvzvMzzeh3gtwHEs7028Fs8L/Ef77eB1+I/1u8Ar83zMs/rdYDfBhDP9trAb/G8xH+83wZei/9YvwO8Ns/LPK/XAX4bQDzbawO/xfMS//F+G3gt/mP9DvDaPC/zvF4H+G0A8WyvDfwWz0v8x/tt4LX4j/U7wGvzvMzzeh3gtwHEs7028Fs8L/Ef77eB1+I/1u8Ar83zMs/rdYDfBhDP9trAb/G8xH+83wZei/9YvwO8Ns/LPK/XAX4bQDzbawO/xfMS//F+G3gt/mP9DvDaPC/zvF4H+G0A8WyvDfwWz0v8x/tt4LX4j/U7wGvzvMzzeh3gtwHEs7028Fs8L/Ef76uBl+Y/1l8DH83zMs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovn9dr87/bbPK/XAX4bQDzbawO/xf8PrwP8NoB4TrvAMf5vuwQc5wrEc/ps4LP4v+1zgM/mCsTz+m7gvfi/6XuA9+bZEM/fawMfDbw2cIz/3S4Bvw18NfDbPCfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hEBS6tBtMlzZwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPictureInPicture;
impl IconShape for MdPictureInPicture {
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
                d: "M19 7h-8v6h8V7zm2-4H3c-1.1 0-2 .9-2 2v14c0 1.1.9 1.98 2 1.98h18c1.1 0 2-.88 2-1.98V5c0-1.1-.9-2-2-2zm0 16.01H3V4.98h18v14.03z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeLZjgNPB47zf9sucIIrEM/22sBv8f/D6wC/DSCe7bWB3+J5vQ7/u/0Wz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/uN9NfBS/Mf6G+CjeV7meb0O8NsA4tleG/gtnpf4j/fbwGvxH+t3gNfmeZnn9TrAbwOIZ3tt4Ld4XuI/3m8Dr8V/rN8BXpvnZZ7X6wC/DSCe7bWB3+J5if94vw28Fv+xfgd4bZ6XeV6vA/w2gHi21wZ+i+cl/uP9NvBa/Mf6HeC1eV7meb0O8NsA4tleG/gtnpf4j/fbwGvxH+t3gNfmeZnn9TrAbwOIZ3tt4Ld4XuI/3m8Dr8V/rN8BXpvnZZ7X6wC/DSCe7bWB3+J5if94vw28Fv+xfgd4bZ6XeV6vA/w2gHi21wZ+i+cl/uP9NvBa/Mf6HeC1eV7meb0O8NsA4tleG/gtnpf4j/fbwGvxH+t3gNfmeZnn9TrAbwOIZ3tt4Ld4XuI/3m8Dr8V/rN8BXpvnZZ7X6wC/DSCe7bWB3+J5if94vw28Fv+xfgd4bZ6XeV6vA/w2gHi21wZ+i+cl/uP9NvBa/Mf6HeC1eV7meb0O8NsA4tleG/gtnpf4j/fbwGvxH+t3gNfmeZnn9TrAbwOIZ3tt4Ld4XuI/3m8Dr8V/rN8BXpvnZZ7X6wC/DSCe7bWB3+J5if94vw28Fv+xfgd4bZ6XeV6vA/w2gHi21wZ+i+cl/uO9NHCc/1i7wF/zvMzzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfN6bf53+22e1+sAvw0gnu21gd/i/4fXAX4bQDynXeAY/7ddAo5zBeI5fTbwWfzf9jnAZ3MF4nl9N/Be/N/0PcB782yI5++9gY8GXor/G34H+G7gu3lOiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjUkOqQXGqZv4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPictureInPictureAlt;
impl IconShape for MdPictureInPictureAlt {
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
                d: "M19 11h-8v6h8v-6zm4 8V4.98C23 3.88 22.1 3 21 3H3c-1.1 0-2 .88-2 1.98V19c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2zm-2 .02H3V4.97h18v14.05z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlagiarism;
impl IconShape for MdPlagiarism {
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
                d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2V8L14,2z M15.04,19.45l-1.88-1.88 c-1.33,0.71-3.01,0.53-4.13-0.59c-1.37-1.37-1.37-3.58,0-4.95c1.37-1.37,3.58-1.37,4.95,0c1.12,1.12,1.31,2.8,0.59,4.13l1.88,1.88 L15.04,19.45z M13,9V3.5L18.5,9H13z",
            }
            circle {
                cx: "11.5",
                cy: "14.5",
                r: "1.5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEEklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/z2+GngpntPfAB/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L8x3tv4L144V4aOM5z2gX+mhfue4Dv5j8O4j/HdwPvxX+s7wHem/9YiP883w28F/8xvgd4b/7jIf5zfTfwXvz7fA/w3vznQPzn+27gvfi3+R7gvfnPg/iv8d3Ae/Gv8z3Ae/OfC/Ff57uB9+JF8z3Ae/OfD/Ff67uB9+KF+x7gvfmvgfiv993Ae/H8fQ/w3vzXQfz7/RbP63OA3+YF+27gvXhO3wO8Ny/YawOfxfN6Hf7tEP9+twIP4jl9DvDZvHDfDbwXV3wP8N68cJ8NfBbP6RnAg/m3Q/z7/TbwWjynrwE+mn/Zd3PFe/Mv+2rgo3hOvwO8Nv92iH+/nwbeiud0K/AQ/mM9HXgwz+lngLfm3w7x7/fewHfxvF4H+G3+Y7w28Fs8r/cBvpt/O8S/33HgIs/rt4HX4T/GbwGvzfM6Aezyb4f4j/HbwGvxvN4H+G7+fd4b+C6e188Ab82/D+I/xmsDv8Xz2gVeB/hr/m1eGvgt4DjP63WA3+bfB/Ef56eBt+J57QJvA/w2/zqvDfwUcJzn9TPAW/Pvh/iP82Dgr4FjPH9fDXwOsMsLdxz4LOCjef4uAS8N3Mq/H+I/1msDv8ULtgv8NPDTwDOAv+aKlwYeBLw18NbAcV6w1wF+m/8YiP947w18F/853gf4bv7jIP5zvDXw3cAx/mNcAt4b+Gn+YyH+87w08NPAg/j3eQbw1sBf8x8P8Z/vvYHPBh7Ev84zgM8Gvpv/PIj/Om8NvDXw1sAxnr9LwE8DPw38NP/5EP99Xpvn9Nv810P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwTl0nlB+EhjcAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlayForWork;
impl IconShape for MdPlayForWork {
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
                d: "M11 5v5.59H7.5l4.5 4.5 4.5-4.5H13V5h-2zm-5 9c0 3.31 2.69 6 6 6s6-2.69 6-6h-2c0 2.21-1.79 4-4 4s-4-1.79-4-4H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGh0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWO8aH6Hf7vjwEvxorkE/DX/Ooh/vQcDT+dF8znAZ/Nv99nAZ/GiewhwKy86xL/ebwGvzb/sEvBgYJd/m5cG/op/nZ8G3oYXHeJf57WB3+JF8z7Ad/Nv91vAa/Ov9zrAb/OiQfzr/BXw0vzLngE8mH+7jwa+in+bvwZehhcN4kX33sB38aJ5HeC3+bc5DjwdOM6/3fsA382/DPGiOQ48HTjOv+x3gNfm3+6ngLfm3+dW4GWAXV44xIvms4HP4kXzEOBW/m1eG/gt/mN8DvDZvHCIf9mDgb8CjvMv+x7gvfm3OQ78FfBg/mPsAi8D3MoLhviXfTfwXvzLLgEPBnb5t/ls4LN40VwCjvEv+x7gvXnBEC/cawO/xYvmc4DP5t/mpYG/4kXzOVzxWbxoXgb4a54/xAv3W8Br8y97BvBg/u1+C3ht/mXPAB4MHAf+GngQ/7LfBl6H5w/xgr018FO8aN4H+G7+bT4a+CpeNK8D/DZXvDfwXbxo3gb4aZ4X4gV7OvBg/mW/A7w2/zbHgacDx/mXfQ/w3jyn3wZei3/ZrcBDeF6I5++zgc/iRfM6wG/zb/NTwFvzL7sEPBjY5Tm9NvBbvGg+BvhqnhPieR0Hng4c51/2PcB782/z2sBv8aJ5H+C7ef5+Gngr/mW7wEOAXZ4N8by+GvgoXjQPAW7lX+848FfAg/mX/Q7w2rxgDwaezovma4CP5tkQz+nBwNN50XwO8Nn823w28Fm8aB4C3MoL99nAZ/GieQhwK1cgntNvAa/Nv+wS8GBgl3+9lwb+ihfN5wCfzb/sOHArcIx/2U8Db8MViGd7beC3eNG8D/Dd/Nv8FvDa/MueATyYF917A9/Fi+Z1gN8GEM/2V8BL8y97BvBg/m0+GvgqXjSvA/w2/zq3Ag/iX/bXwMsAiCveG/guXjSvA/w2/3rHgacDx/mXfQ/w3vzrvTbwW7xo3gf4bnHFewPfxYvmdYDf5l/vp4C35l92CXgwsMu/3msDv8WL5n2A7xbP9tfAS/EvuxV4CP86rw38Fi+a9wG+m3+bpwMP5l/2N8BLA4hne23gt3jRvA/w3bxojgN/BTyYf9nvAK/Nv817A9/Fi+Z1gN8GEM/pt4HX4l+2CzwE2OVf9tnAZ/GieQhwK/96x4GnA8f5l/0M8NZcgXhODwaezovmc4DP5oV7aeCveNF8DvDZ/Nt8NvBZvGgeAtzKFYjn9dXAR/GieQhwKy/YbwGvzb/sGcCD+bd5MPB0XjRfA3w0z4Z4XseBW4Fj/Mu+B3hvnr+PBr6KF83rAL/Nv813A+/Fv+wS8GBgl2dDPH+fDXwWL5rXAX6b53QceDpwnH/Z9wDvzb/NawO/xYvmY4Cv5jkhXrBbgQfxL/tt4HV4Tj8FvDX/skvAg4Fd/m1+C3ht/mXPAB7M80K8YG8N/BQvmvcBvpsrXhv4LV407wN8N/827w18Fy+atwF+mueFeOF+G3gt/mW3Ai/DFX8FPJh/2e8Ar82/3dOBB/Mv+x3gtXn+EC/cawO/xYvmc7jis3jRPAS4lX+bzwY+ixfNywB/zfOH+Jd9N/Be/Mt2geO8aD4H+Gz+bY4DTweO8y/7HuC9ecEQ/7IHA38NHOM/xjOAlwZ2+bf5buC9+JddAl4auJUXDPGi+Wzgs/iP8TrAb/Nv82Dg6bxoPgf4bF44xIvmOHArcIx/n58B3pp/u98CXpt/2TOAlwZ2eeEQL7r3Br6Lf7tLwIOBXf5tXhv4LV407wN8N/8yxL/OXwMvxb/NxwBfzb/d04EH8y/7G+CledEg/nVeG/gt/vV+B3ht/u3eG/guXjSvA/w2LxrEv95vA6/Fv87LAH/Nv81x4OnAcf5lPwO8NS86xL/eg4Gn86L7HOCz+bf7bOCzeNE8BLiVFx3i3+algeO8aP4a2OXf7rV50ewCf82/DuL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I23v7kGjHg0ZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPolymer;
impl IconShape for MdPolymer {
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
                d: "M19 4h-4L7.11 16.63 4.5 12 9 4H5L.5 12 5 20h4l7.89-12.63L19.5 12 15 20h4l4.5-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPowerSettingsNew;
impl IconShape for MdPowerSettingsNew {
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
                d: "M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/td4KeGngpYG/Bn4G+Gv++yD+a7w08F3AS/O8vhr4GP57IP7zHQf+CngwL9jXAB/Nfz3Ef77PBj6Lf9nLAH/Nfy3Ef76nAw/mX/Y5wGfzXwvxn8+8aH4GeGv+ayH+8+0Cx/iXfQ7w2fzXQvzn+27gvfiXvQ3w0/zXQvzne2ngr3jh/gZ4af7rIf5rvDfwXTx/zwBeG7iV/3qI/zovDXw08NbAMeAZwHcDXw3s8t8D8f8b4v83xH+tBwMP4tl+h/9eiP8abw18FvDSPK/vBj4HuJX/eoj/XMeBrwLem3/Z+wDfzX8txH+u3wJemxfd+wDfzX8dxH+ezwY+i3+9hwC38l8D8Z/jOPB04Dj/et8DvDf/NRD/Od4b+C7+7cR/DcR/js8GPot/u9cBfpv/fIj/HD8NvBX/du8DfDf/+RD/OX4beC3+fX4beBtgl/88iP8cvw28Fv9+u8DrAH/Nfw7Ef47fBl6L/xh/DbwOsMt/PMR/jt8GXov/OJ8DfDb/8RD/OX4beC3+4+wCDwF2+Y+F+M/x28Br8R/rfYDv5j8W4j/HbwOvxX+szwE+m/9YiP8cvw28Fv+xfgZ4a/5jIf5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/xy/DbwW/7F+B3ht/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HSwPHeU4vDXwVL9jfAB/NC7YL/DX/sRD/dV4b+C1esN8BXpv/Woj/Oq8N/BYv2O8Ar81/LcR/ndcGfosX7HeA1+a/FuK/zmsDv8UL9jvAa/NfC/Ff57WB3+IF+x3gtfmvhfiv89rAb/GC/Q7w2vzXQvzXeW3gt3jBfgd4bf5rIf7rvDbwW7xgvwO8Nv+1EP91Xhv4LV6w3wFem/9aiP86rw38Fi/Y7wCvzX8txH+d1wZ+ixfsd4DX5r8W4r/OawO/xQv2O8Br818L8V/npYGv5gX7a+Cj+a+F+P8N8f8b4v83xP9viP/f+Ef2jXFBnCZR0wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPregnantWoman;
impl IconShape for MdPregnantWoman {
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
                d: "M9,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S9,5.11,9,4z M16,13c-0.01-1.34-0.83-2.51-2-3c0-1.66-1.34-3-3-3 s-3,1.34-3,3v7h2v5h3v-5h3V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPreview;
impl IconShape for MdPreview {
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
                d: "M19,3H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.11,3,19,3z M19,19H5V7h14V19z M13.5,13 c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5S13.5,12.17,13.5,13z M12,9c-2.73,0-5.06,1.66-6,4 c0.94,2.34,3.27,4,6,4s5.06-1.66,6-4C17.06,10.66,14.73,9,12,9z M12,15.5c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5 c1.38,0,2.5,1.12,2.5,2.5C14.5,14.38,13.38,15.5,12,15.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH+/3+a/12vzb4f4/w3x/xviRfdg4K2A48Br8z/TbwO7wM8At/IvQ/zLjgNfBbw3/7t8N/AxwC4vGOKFe2ngt4Dj/O+0C7wO8Nc8f4gX7DjwdOA4/7vtAg8BdnleiBfsu4H34v+G7wHem+eFeP4eDDyd/z2eAdwKHAdeiufvIcCtPCfE8/fRwFfxv8PfAK8N7HLFZwOfxfP6GOCreU6I5++rgY/if4eXAf6a5/TXwEvxnD4H+GyeE+L5+23gtfjfQTyv3wZei+f0O8Br85wQz99vA6/F/w7vA3w3z3YceDpwnOf0O8Br85wQz99vA6/F/w67wEcD3wO8NPBdwEvzvH4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXot/u48B/pr/WC8NfBX/dr8DvDbPCfH8/TbwWvzbvQ7w2/zHem3gt/i3+x3gtXlOiOfvt4HX4t/udYDf5j/WawO/xb/d7wCvzXNCPH+/DbwW/3avA/w2/7FeG/gt/u1+B3htnhPi+ftt4LX4t3sd4Lf5j/XawG/xb/c7wGvznBDP328Dr8W/3esAv81/rNcGfot/u98BXpvnhHj+fht4Lf7tXgf4bf5jvTbwW/zb/Q7w2jwnxPP328Br8W/3OsBv8x/rtYHf4t/ud4DX5jkhnr+XBo7zovktntfrAL/Nf6zXBn6L5/U6vGh2gb/mOSH+/czzeh3gt/mP9drAb/G8xL8d4t/PPK/XAX6b/1ivDfwWz0v82yH+/czzeh3gt/mP9drAb/G8xL8d4t/PPK/XAX6b/1ivDfwWz0v82yH+/czz+mjgr/mP9dLAV/O8xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMUNnZBng0GSQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPrint;
impl IconShape for MdPrint {
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
                d: "M19 8H5c-1.66 0-3 1.34-3 3v6h4v4h12v-4h4v-6c0-1.66-1.34-3-3-3zm-3 11H8v-5h8v5zm3-7c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-1-9H6v4h12V3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFA0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3ht/nP9NvDdwPfw74f49zkOvBfw0cCD+a91K/DVwPcAu/zbIP5tHgx8FPDewHH+e+0C3w18DXAr/zqIf53XBt4LeG/+Z/pu4HuA3+ZFg3jRvBfw3sBr87/DbwPfDXwPLxziX/Z04MH873Qr8BBeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Mf7G2CX53QceCn+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv7jfQzw1zynlwa+iv944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S/7a+Cl+I/1OsBv85xeG/gt/mP9DvDavGCIf9lvA6/Ff6zXAX6b5/TawG/xH+t3gNfmBUP8yz4b+Cz+Y70O8Ns8p9cGfov/WJ8DfDYvGOJf9t7Ad/Ef63WA3+Y5vTbwW/zHeh/gu3nBEP+ylwb+iv9YrwP8Ns/ptYHf4j/WywB/zQuGeNHsAsf4j/M6wG/znF4b+C3+41wCjvPCIV40Pw28Ff9xXgf4bZ7TawO/xX+c7wHemxcO8aJ5b+C7+I/zOsBv85xeG/gt/uO8D/DdvHCIF81x4CL/cV4H+G2e02sDv8V/nBPALi8c4kX33cB78R/jdYDf5jm9NvBb/Mf4HuC9+ZchXnSvDfwW/zFeB/htntNrA7/Ff4zXAX6bfxniX+e3gdfi3+91gN/mOb028Fv8+/0O8Nq8aBD/Om8N/BT/fq8D/DbP6bWB3+Lf722An+ZFg/jX+23gtfj3+Wtgl+d0HHhp/n1+B3htXnSIf73XBn6L/5leB/htXnSIf5uvBj6K/1m+Bvho/nUQ/zbHgb8GHsT/DM8AXhrY5V8H8W/32sBv8T/D6wC/zb8e4t/no4Gv4r/XxwBfzb8N4t/vu4H34r/H9wDvzb8d4j/GdwPvxX+t7wHem38fxH+M48BPA6/Ff43fAV6bfz/Ef6zvBt6L/1zfA7w3/zEQ//E+G/gs/nN8DvDZ/MdB/Od4a+C7gWP8x7gEvDXw2/zHQvznOQ58N/BW/Pv8DPDewC7/8RD/+V4b+G7gQfzrPAN4b+C3+c+D+K/z3sBHAy/FC/c3wFcD381/PsR/vZcGPhp4beBBXPEM4LeBrwb+mv86iP9eL80Vf81/D8T/b4j/3/hHoqSnQZi7ve0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPrivacyTip;
impl IconShape for MdPrivacyTip {
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
                d: "M12,1L3,5v6c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12V5L12,1L12,1z M11,7h2v2h-2V7z M11,11h2v6h-2V11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPublishedWithChanges;
impl IconShape for MdPublishedWithChanges {
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
                d: "M17.66,9.53l-7.07,7.07l-4.24-4.24l1.41-1.41l2.83,2.83l5.66-5.66L17.66,9.53z M4,12c0-2.33,1.02-4.42,2.62-5.88L9,8.5v-6H3 l2.2,2.2C3.24,6.52,2,9.11,2,12c0,5.19,3.95,9.45,9,9.95v-2.02C7.06,19.44,4,16.07,4,12z M22,12c0-5.19-3.95-9.45-9-9.95v2.02 c3.94,0.49,7,3.86,7,7.93c0,2.33-1.02,4.42-2.62,5.88L15,15.5v6h6l-2.2-2.2C20.76,17.48,22,14.89,22,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQueryBuilder;
impl IconShape for MdQueryBuilder {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
            path {
                d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv2EsD7wW8NP9z/DTwPcAu/zEQz99LA3/F/0x/DbwM/zEQz99XAx/F/1yvA/w2/36I5++3gdfif67XAX6bfz/E8/fbwGvxP9frAL/Nvx/i+ftt4LX4n+t1gN/m3w/x/P028Fr8z/U6wG/z74d4/n4beC3+53od4Lf590M8f78NvBb/c70O8Nv8+yGev98GXov/uV4H+G3+/RDP328Dr8X/XK8D/Db/fojn77eB1+J/rtcBfpvn9NrAb/G8bgW+G/gcnhfi+ftt4LX4n+t1gN/mOb028Fu8YG8D/DTPCfH8/TbwWvzP9TrAb/OcXhv4LV6w7wHem+eEeP5+G3gt/ud6HeC3eU6vDfwWL9jvAK/Nc0I8f78NvBb/c70O8Ns8p9cGfosX7HeA1+Y5IZ6/3wZei/+5Xgf4bZ7TawO/xQv2O8Br85wQz99vA6/F/1yvA/w2z+m1gd/iBfsd4LV5Tojn77eB1+J/rtcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8T/X6wC/zXN6beC3eMF+B3htnhPi+ftt4LX4n+t1gN/mOb028Fu8YL8DvDbPCfH8/TbwWvzP9TrAb/OcXhv4LV6w3wFem+eEeP5+G3gt/ud6HeC3eU6vDfwWL9jvAK/Nc0I8f78NvBb/c70O8Ns8p9cGfosX7HeA1+Y5IZ6/3wZei/+5Xgf4bZ7TawO/xQv2O8Br85wQz99vA6/F/1yvA/w2z+m1gd/iBfsd4LV5Tojn77eB1+J/rtcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8T/X6wC/zXN6beC3eMF+B3htnhPi+ftt4LX4n+lvgJfmeb028Fu8YL8DvDbPCfH8/TbwWvzLLgF/zX+NXeC3ge8Gdnlerw38Fi/Y7wCvzXNCPH+/DbwWL9wl4LWBv+Z/htcGfosX7HeA1+Y5IZ6/3wZeixfsEvDawF/zP8drA7/FC/Y7wGvznBDP328Dr8Xzdwl4beCv+Z/ltYHf4gX7HeC1eU6I5++3gdfieV0CXhv4a/7neW3gt3jBfgd4bZ4T4vn7beC1eE6XgNcG/pr/mV4b+C1esN8BXpvnhHj+fht4LZ7tEvDawF/zvMz/Dr8DvDbPCfH8/TbwWlxxCXht4K95/sz/Dr8DvDbPCfH8/TbwWsAl4LWBv+YFM/87/A7w2jwnxPP328BLA68N/DUvnPnf4XeA1+Y5IZ6/nwY+G/hr/mXmf4ffAV6b54R4/o4Du7xozP8OvwO8Ns8J8e/33cB78T/f7wCvzXNC/PsdB74aeC/+Z/sd4LV5Toj//d4b+C7+Zb8DvDbPCfF/w3sD38UL9zvAa/OcEP93vDfwXbxgvwO8Ns8J8X/LewPfxfP3O8Br85wQ//e8N/BdPK/fAV6b54T4v+m9ge/iOf0O8No8J8T/Xe8NfBfP9jvAa/OcEP+3vTfwXVzxO8Br85wQ//e9N/BdwO8Ar81zQvz/8N7AewOvzXNC/P/x0sBf85wQ/78h/n9D/P+G+P8N8f8b/wjeu7NBNyGVgQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQuestionAnswer;
impl IconShape for MdQuestionAnswer {
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
                d: "M21 6h-2v9H6v2c0 .55.45 1 1 1h11l4 4V7c0-.55-.45-1-1-1zm-4 6V3c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v14l4-4h10c.55 0 1-.45 1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+Z/gd/mP8NfDRPCfE8/fbwGvxP4P4z4N4/n4beC3+ZxD/eRDP328Dr8X/DOI/D+L5+23gtfifQfznQTx/vw28Fv8ziP88iOfvt4HX4n8G8bx+G3gt/nU+BvhqnhPi+ftt4LX4n0E8r98GXosX3SXgwcAuzwnx/P028Fr8zyCe128Dr8WL7nuA9+Z5IZ6/3wZei/8ZxPP6beC1eNE9BLiV54V4/n4beC3+ZxDP67eB1+JF8zvAa/P8IZ6/3wZei/8ZxPP6beC1eNG8D/DdPH+I5++3gdfifwbxvH4beC3+Zc8AHswLhnj+fht4Lf5nEM/rt4HX4l/2OcBn84Ihnr/fBl6L/xnE8/pt4LX4l50AdnnBEM/fbwOvxf8M4nn9NvBavHDfA7w3Lxzi+ftt4LX4n0E8r98GXosX7qOBv+Z5/Q7Phnj+fht4Lf5nEM/rt4HX4l/ve4D35tkQz99vA6/F/wzief028Fr86z0EuJVnQzx/vw28Fv8ziOf128Br8a/zPcB785wQz99vA6/F/wzief028Fr86zwEuJXnhHj+fht4Lf5nEM/rt4HX4kX3PcB787wQz99vA6/Fv+x9gO/mf4bXBn6L5+8hwK08L8Tz99vAa/HCvQ/w3fzP8drAb/G8vgd4b54/xPP328Br8YK9D/Dd/M/y2sBv8bweAtzK84d4/n4beC2ev/cBvpv/eV4b+C2e0/cA780Lhnj+fht4LZ7X+wDfzf9Mrw38Fs/pIcCtvGCI5++3gdfiOb0P8N38z/XawG/xbN8DvDcvHOL5+23gtXi29wG+m//ZXhv4LZ7tIcCtvHCI5++3gdfiivcBvpv/+V4b+C2u+B7gvfmXIZ6/3wZeC3gf4Lv53+G1gd/iiocAt/IvQzx/vw18N/Dd/O/x2sBvAd8DvDcvGsTz99LAX/O/y2sDvwU8BLiVFw3i/47XBt4beG9edIj/O14buBW4lRcd4v83xP9viP/f+Ed1VZRBLWcmDAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQuickreply;
impl IconShape for MdQuickreply {
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
                d: "M22,4c0-1.1-0.9-2-2-2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h9v-8h7V4z",
            }
            polygon {
                points: "22.5,16 20.3,16 22,12 17,12 17,18 19,18 19,23",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG/prn76W54q95/l6aK/6a5++lueKvef5emiv+mufvpbnir3n+Xhr4a14wxL/sIvA6wF/znF4a+C2ueB3gr3lOLw38Fle8DvDXPKeXBn6LK14H+Gue00sDv8UVrwP8Nc/ppYHf4orXAf6a5/TSwG8BJ3jBEP8yA7vA6wB/zRUvDfwWcJwrdoHXAf6aK14a+C3gOFfsAq8D/DVXvDTwW8BxrtgFXgf4a654aeC3gONcsQu8DvDXXPHSwG8Bx7liF3gd4K+54qWB3wKOA+IFQ/zLzBW7wOtwxW8Bx3lOu8DrcMVvAcd5TrvA63DFbwHHeU67wOtwxW8Bx3lOu8DrcMVvAcd5TrvA63DFbwHHuUK8YIh/mXm2Xa44zvO3yxXHef52ueI4z98uVxzn+dvliuM8f7tccZxnEy8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZl643+G/12vxwokXDPEvMy+c+O9lXjjxgiH+ZeaFE/+9zAsnXjDEv8y8cOK/l3nhxAuG+JeZF0789zIvnHjBEP8y88KJ/17mhRMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2ZeuNfhv9dv8cKJFwzxLzMvnPjvZV448YIh/mXmhRP/vcwLJ14wxL/MvHDiv5d54cQLhviXmRdO/PcyL5x4wRD/MvPCvTb/vX6bF068YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5l54cR/L/PCiRcM8S8zL5z472VeOPGCIf5l5oUT/73MCydeMMS/zLxw4r+XeeHEC4b4l5kXTvz3Mi+ceMEQ/zLzwv02/71emxdOvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZebZLXHGM5+8SVxzj+bvEFcd4/i5xxTGev0tccYzn7xJXHOPZxAuG+JeZKy4Br80Vvw0c4zldAl6bK34bOMZzugS8Nlf8NnCM53QJeG2u+G3gGM/pEvDaXPHbwDGe0yXgtbnit4FjXCFeMMS/zMAl4LWBv+aKlwZ+GzjGFZeA1wb+miteGvht4BhXXAJeG/hrrnhp4LeBY1xxCXht4K+54qWB3waOccUl4LWBv+aKlwZ+GzjGFZeA1wb+miteGvht4BggXjDEv2wXeG3gr3lOLw38Nle8NvDXPKeXBn6bK14b+Gue00sDv80Vrw38Nc/ppYHf5orXBv6a5/TSwG9zxWsDf81zemngt4HjvGCIf9lLA3/N8/fSXPHXPH8vzRV/zfP30lzx1zx/L80Vf83z99Jc8dc8fy8N/DUvGOL/N8T/b4j/3xD/vyH+f+MfATqStEHrINf6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReceipt;
impl IconShape for MdReceipt {
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
                d: "M18 17H6v-2h12v2zm0-4H6v-2h12v2zm0-4H6V7h12v2zM3 22l1.5-1.5L6 22l1.5-1.5L9 22l1.5-1.5L12 22l1.5-1.5L15 22l1.5-1.5L18 22l1.5-1.5L21 22V2l-1.5 1.5L18 2l-1.5 1.5L15 2l-1.5 1.5L12 2l-1.5 1.5L9 2 7.5 3.5 6 2 4.5 3.5 3 2v20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRecordVoiceOver;
impl IconShape for MdRecordVoiceOver {
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
            circle {
                cx: "9",
                cy: "9",
                r: "4",
            }
            path {
                d: "M9 15c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4zm7.76-9.64l-1.68 1.69c.84 1.18.84 2.71 0 3.89l1.68 1.69c2.02-2.02 2.02-5.07 0-7.27zM20.07 2l-1.63 1.63c2.77 3.02 2.77 7.56 0 10.74L20.07 16c3.9-3.89 3.91-9.95 0-14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGpElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/OV4aeCvgwcAusAv8DPDXvGheGngr4DhwHLgV+Bngr/mPhfiP9drAdwEP5vn7a+B9gL/m+Xtp4LuAl+b5uxV4H+C3+Y+B+I/z3sB38aJ5H+C7eU7vDXwXL5r3Ab6bfz/Ef4z3Br6Lf533Ab6bK94a+Cn+dd4H+G7+fRD/fseBpwPH+dfZBV4H2AX+CjjOv84u8BBgl387xL/fRwNfxfP3O1zxWjx/f80VL83z9ztc8Vo8fx8DfDX/doh/v98GXovndAl4beCvueKlgd8GjvGiuQS8NvDXXPHSwG8Dx3hOvwO8Nv92iH8/87y+BvhontNXAx/Fi+ZrgI/mOX018FE8L/Fvh/j3M8/rfYDv5jl9NPBVvGjeB/huntNHA1/F8xL/doh/v13gGM/pp4G34Tn9FPDWvGh+GngbntNPAW/Nc3oG8GD+7RD/fj8NvBXP67OB7+GK9wI+m3+dzwa+hyveC/hsntf3AO/Nvx3ihTsOfBTw3sCD+d/lVuC7ga8Bdnn+EC/Yg4HfAh7M/263Aq8D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW//k+hiu+iv98nwN8Ns8J8fz9NPBWPKe/AT6af53jwHcBx3lePwO8NVf8NPBWPK9d4H2AXf51vhp4KZ7T7wCvzXNCPH+/DbwWz+l3gNfmX+engLfmeT0DeGlglyuOA7cCx3he3w28D/86vw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzontv4Lt4/l4H+G2e02sDv8Xz9z7Ad/Oi+23gtXhOvwO8Ns8J8fz9NvBaPKffAV6bF81LA78FHOd5fQ7w2Tx/nw18Fs9rF3gd4K950fw28Fo8p98BXpvnhHj+fht4LZ7T7wCvzb/sOPBbwEvzvH4HeG1euN8GXovn9dfAy/Ci+W3gtXhOvwO8Ns8J8fz9NvBaPKffAV6bf9lXAx/F87oEPBjY5YU7DtwKHON5fQ3w0fzLfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmhXtr4Kd4/t4G+GleNG8N/BTP39sAP80L99vAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2L9iDgb8CjvO8vgb4aP51vhr4KJ7XLvAywK28YL8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br88J9NvBZPKe/AV6af5u/Bl6K5/Q5wGfzwv028Fo8p98BXpvnhHj+fht4LZ7T7wCvzb/spYGfBh4EXAJeGriVf5sHA38NHAOeAbw18Nf8y34beC2e0+8Ar81zQjx/vw28Fs/pd4DX5kVzHPhu4KeB7+bf572B1wY+GtjlRfPbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7TLvDX/O/w0sBxntPvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/Gc/gb4aP53+GrgpXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjynvwY+mv8dvhp4aZ7T7wCvzXNCPH+fDXwW/7d8DvDZPCfE8/fWwE/xf8vbAD/Nc0K8YH8NvBT/N/wN8NI8L8QL9tLATwMP4n+3ZwCvDdzK80K8cMeBjwbeG3gQ/7s8A/hu4KuBXZ4/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RWFr2QREYISsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRedeem;
impl IconShape for MdRedeem {
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
                d: "M20 6h-2.18c.11-.31.18-.65.18-1 0-1.66-1.34-3-3-3-1.05 0-1.96.54-2.5 1.35l-.5.67-.5-.68C10.96 2.54 10.05 2 9 2 7.34 2 6 3.34 6 5c0 .35.07.69.18 1H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-5-2c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM9 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm11 15H4v-2h16v2zm0-5H4V8h5.08L7 10.83 8.62 12 11 8.76l1-1.36 1 1.36L15.38 12 17 10.83 14.92 8H20v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveDone;
impl IconShape for MdRemoveDone {
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
                d: "M1.79 12l5.58 5.59L5.96 19 .37 13.41 1.79 12zm.45-7.78L12.9 14.89l-1.28 1.28L7.44 12l-1.41 1.41L11.62 19l2.69-2.69 4.89 4.89 1.41-1.41L3.65 2.81 2.24 4.22zm14.9 9.27L23.62 7 22.2 5.59l-6.48 6.48 1.42 1.42zM17.96 7l-1.41-1.41-3.65 3.66 1.41 1.41L17.96 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveShoppingCart;
impl IconShape for MdRemoveShoppingCart {
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
                d: "M22.73 22.73L2.77 2.77 2 2l-.73-.73L0 2.54l4.39 4.39 2.21 4.66-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h7.46l1.38 1.38c-.5.36-.83.95-.83 1.62 0 1.1.89 2 1.99 2 .67 0 1.26-.33 1.62-.84L21.46 24l1.27-1.27zM7.42 15c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h2.36l2 2H7.42zm8.13-2c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H6.54l9.01 9zM7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7Lf4n+31+EFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/Mt+m//dXpsXDPH/G+L/N8T/b4j/3xD/vyH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mWvzf9uv80Lhvj/DfH/G+L/N8T/b4h/2Wvxv9vv8IIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5/Q/z/hvj/DfH/G+L/N8S/7Lf43+11eMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/st/mf7fX5gVD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjUq4YQbbX6pQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReorder;
impl IconShape for MdReorder {
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
                d: "M3 15h18v-2H3v2zm0 4h18v-2H3v2zm0-8h18V9H3v2zm0-6v2h18V5H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR7MFbfy3wPx3+unuOJt+O+B+O/z2sBvccXrAL/Nfz3Ef5+/Al6aK/4aeBn+6yH+e3w08FU8p48Bvpr/Woj/eseBpwPHeU67wEOAXf7rIP7rfTXwUTx/XwN8NP91EP+1Hgw8nRfuIcCt/NdA/Nf6LeC1eeF+G3gd/msg/uu8NfBTvGjeBvhp/vMh/us8HXgwL5pbgYfwnw/xX+Ozgc/iX+dzgM/mPxfiP99x4OnAcf51doGHALv850H85/tu4L34t/ke4L35z4P4z/XawG/x7/M6wG/znwPxn+u3gNfm3+e3gdfhPwfiP897A9/Ff4z3Ab6b/3iI/xzHgb8CHsx/jFuBlwF2+Y+F+M/x2cBn8R/rc4DP5j8W4j/eg4Gn85/jIcCt/MdB/Mf7KeCt+c/x08Db8B8H8R/rtYHf4l/2N8Auz+k48FL8y14H+G3+YyD+Y/0V8NL8y14H+G2e02sDv8W/7K+Bl+E/BuI/zkcDX8WL5nWA3+Y5vTbwW7xoPgb4av79EP8xjgNPB47zonkd4Ld5Tq8N/BYvml3gIcAu/z6I/xhfDXwUL7rXAX6b5/TawG/xovsa4KP590H8+z0YeDr/Oq8D/DbP6bWB3+Jf5yHArfzbIf79fgt4bf51Xgf4bZ7TawO/xb/ObwOvw78d4t/nrYGf4l/vdYDf5jm9NvBb/Ou9DfDT/Nsg/n2eDjyYf73XAX6b5/TawG/xr3cr8BD+bRD/dp8NfBb/Nq8D/DbP6bWB3+Lf5nOAz+ZfD/Fvcxx4OnCcf5vXAX6b5/TawG/xb7MLPATY5V8H8W/z3cB78W/318Auz+k48NL8230P8N786yD+9V4b+C3+Z3od4Ld50SH+9X4LeG3+Z/pt4HV40SH+dd4b+C7+Z3sf4Lt50SBedMeBvwIezP9stwIvA+zyL0O86D4b+Cz+Y/wNsMtzOg68FP8xPgf4bP5liBfNg4Gn8x/ndYDf5jm9NvBb/Md5CHArLxziRfNTwFvzH+d1gN/mOb028Fv8x/lp4G144RD/stcGfov/WK8D/DbP6bWB3+I/1usAv80LhviX/RXw0vzHeh3gt3lOrw38Fv+x/hp4GV4wxAv30cBX8R/vdYDf5jm9NvBb/Mf7GOCref4QL9hx4OnAcf532wUeAuzyvBAv2FcDH8X/DV8DfDTPC/H8PRh4Ov+3PAS4leeEeP5+C3ht/m/5beB1eE6I5/XWwE/xf9PbAD/NsyGe19OBB/N/063AQ3g2xHP6bOCz+L/tc4DP5grEsx0Hng4c5/+2XeAhwC6A+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8COL6QQdQeduIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReportProblem;
impl IconShape for MdReportProblem {
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
                d: "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/Pb4beB/+4yFedK8N/Bb/fb4beB/+YyFedK8N/Bb/vb4beB/+4yBedK8N/Bb//b4beB/+YyBedK8N/Bb/M3w38D78+yFedK8N/Bb/c3w38D78+yBedK8N/Bb/s3w38D782yFedK8N/Bb/83w38D782yBedK8N/Bb/M3038D786yFedK8N/Bb/c3038D786yBedK8N/Bb/s3038D686BAvutcGfov/+b4beB9eNIgX3WsDv8V/jI8B/prn9NLAV/Ef43WA3+ZfhnjRvTbwW/zHeB3gt3lOrw38Fv8xXgf4bf5liBfdawO/xX+M1wF+m+f02sBv8R/jdYDf5l+GeNG9NvBb/OtdAj4a+G7+bcy/3usAv82/DPGie23gt/jXex/gu/m3M/96rwP8Nv8yxIvutYHf4l9P/PuYf73XAX6bfxniRffawG/xryf+fcy/3usAv82/DPGie23gt/jXE/8+5l/vdYDf5l+GeNG9NvBb/OuJ5/XVwEvxonlt/vVeB/ht/mWIF91rA7/Fv554Xr8NvBb/eV4H+G3+ZYgX3WsDv8W/nnhevw28Fv95Xgf4bf5liBfdawO/xb+e+Ld5b+CrgWP8670O8Nv8yxAvutcGfot/PfFv997Ad/Gv9zrAb/MvQ7zoXhv4Lf71xL+P+dd7HeC3+ZchXnSvDfwW/3ovA/w1/3bmX+91gN/mX4Z40b028Fv86/018DrALv825l/vdYDf5l+GeNG9NvBb/NvsAn/NC/c3wEfzvMy/3usAv82/DPGie23gt/jP8zvAa/O8zL/e6wC/zb8M8aJ7beC3+M/zO8Br87zMv97rAL/Nvwzxontt4Lf41xP/PuZf73WA3+ZfhnjRvTbwW/zriX8f86/3OsBv8y9DvOheG/gt/vXEv4/513sd4Lf5lyFedK8N/Bb/euLf7qWBv+Jf73WA3+ZfhnjRvTbwW/zriX+b48BvAS/Nv97rAL/Nvwzxontt4Lf41/ttntfHAH/Nc3pp4Kt4tpcGjvNv8zrAb/MvQ7zoXhv4Lf5jvA7w2zyn1wZ+i/8YrwP8Nv8yxIvutYHf4j/G6wC/zXN6beC3+I/xOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontp4K/43+FlgL/mX4b417kVeBD/sz0DeDAvGsS/zmsDv8X/bK8D/DYvGsS/3nsDXw0c43+WS8B7Az/Niw7xb3MceG/gpYEH89/rVuCvge8GdvnXQfz/hvj/DfH/G+L/N8T/b/wjVoucQblKeDYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRequestPage;
impl IconShape for MdRequestPage {
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
                d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8L14,2z M15,11h-4v1h3c0.55,0,1,0.45,1,1v3 c0,0.55-0.45,1-1,1h-1v1h-2v-1H9v-2h4v-1h-3c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h1V8h2v1h2V11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRestore;
impl IconShape for MdRestore {
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
                d: "M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fSwG8Bx/nX2QVeB/hr/vMg/nO9NPBbwHH+bXaB1wH+mv8ciH+blwaO8cIdB74bOM6/zy7w3sAuL9wl4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI/98Q/78h/v1ei/9ev8O/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/dpe44hj/duLfDvHvZ/5tLgGvzRW/DRzj30b82yH+/cy/3iXgtYG/5oqXBn4bOMa/nvi3Q/z7mX+dS8BrA3/Nc3pp4LeBY/zriH87xL+fedFdAl4b+Guev5cGfhs4xotO/Nsh/v3Mi+YS8NrAX/PCvTTw28AxXjTi3w7x72f+ZZeA1wb+mhfNSwO/DRzjXyb+7RD/fuaFuwS8NvDX/Ou8NPDbwDFeOPFvh/j3My/YJeC1gb/m3+algd8GjvGCiX87xL+fef4uAa8N/DX/Pi8N/DZwjOdP/Nsh/v3M87oEvDbw1/zHeGngt4FjPC/xb4f49zPP61bgVp7T3wAfzYvmq4GX4jk9GHgwz0v82yH+/cyL5neA1+ZF89vAa/GiEf92iH8/86L5HeC1edH8NvBavGjEvx3i38+8aH4HeG1eNL8NvBYvGvFvh/j3My+a3wFemxfNbwOvxYtG/Nsh/v3Mi+Z3gNfmRfPbwGvxohH/doh/P/Oi+R3gtXnR/DbwWrxoxL8d4t/PvGh+B3htXjS/DbwWLxrxb4f49zMvmt8BXpsXzW8Dr8WLRvzbIf79zIvmd4DX5kXz28Br8aIR/3aIfz/zovkd4LV50fw28Fq8aMS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f499sFjvHf4xJwnH87xL/fdwPvxX+P7wHem387xL/fg4Gn89/jIcCt/Nsh/mO8N/Bd/Nd6H+C7+fdB/Md5MPDZwFsDx/jPcQn4aeCzgVv590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHjFnhBo/YzYwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRestoreFromTrash;
impl IconShape for MdRestoreFromTrash {
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
                d: "M19 4h-3.5l-1-1h-5l-1 1H5v2h14zM6 7v12c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6zm8 7v4h-4v-4H8l4-4 4 4h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRestorePage;
impl IconShape for MdRestorePage {
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm-2 16c-2.05 0-3.81-1.24-4.58-3h1.71c.63.9 1.68 1.5 2.87 1.5 1.93 0 3.5-1.57 3.5-3.5S13.93 9.5 12 9.5c-1.35 0-2.52.78-3.1 1.9l1.6 1.6h-4V9l1.3 1.3C8.69 8.92 10.23 8 12 8c2.76 0 5 2.24 5 5s-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDZ/NvtAl8NfA2wy388xH+OjwI+GzjOf4xd4GOA7+Y/FuI/3ncB781/ju8G3of/OIj/OMeB3wJemv9cfw28DrDLvx/iP85fAS/Nf42/Bl6Gfz/Ef4zvAt6b/1rfDbwP/z6If7+PBr6K/x7vA3w3/3aIf5/jwNOB4/z32AUeAuzyb4P49/ls4LP47/U5wGfzb4P4t3sw8HT+++0CLwPcyr8e4t/uo4Gv4t/mb4DfBv6aK14aeG3gpfi3+Rjgq/nXQ/zb/TbwWvzrXAI+Gvhunr/3Br4aOMa/zu8Ar82/HuLf5jhwkX+dS8BrA3/NC/fSwG8Dx/jXOQHs8q+D+Ld5beC3+Nd5H+C7edG8N/Bd/Ou8DvDb/Osg/m0+G/gsXnR/A7w0/zq3Ag/iRfcxwFfzr4P4t/ls4LN40X0N8NH863w18FG86D4H+Gz+dRD/Nt8NvBcvuvcBvpt/nY8GvooX3dcAH82/DuLf5ruB9+JF9z7Ad/Ov89HAV/Gi+x7gvfnXQfzbfDbwWbzovgb4aP51vhr4KF50nwN8Nv86iH+bzwY+ixfdXwMvw7/O04EH86L7HOCz+ddB/Nu8NvBb/Ou8D/DdvGjeG/gu/nVeB/ht/nUQ/zbHgYv86+wCrwP8NS/cSwO/BRznX+cEsMu/DuLf7lbgQfzr7AIfDXwPz997AV8NHOdf5xnAg/nXQ/zbfTXwUfzb3Ar8NPDXXPHSwFsDD+bf5muAj+ZfD/Fv99LAX/E/w8sAf82/HuLf51bgQfz3egbwYP5tEP8+nw18Fv+9Pgf4bP5tEP8+Dwaezn+vhwC38m+D+Pf7buC9+O/xPcB782+H+Pd7aeCv+O/xEOBW/u0Q/zF+G3gt/mv9DPDW/Psg/mO8NvBb/Nd6GeCv+fdB/Mf5buC9+K/xPcB78++H+I/zYODp/Nd4CHAr/36I/1ifDXwW/7k+B/hs/mMg/mMdB/4aeBD/OZ4BvDSwy38MxH+8twZ+iv8cbwP8NP9xEP85fhp4K/5j/Qzw1vzHQvzneDDw18Ax/mNcAh4M7PIfC/Gf56OBr+I/xscAX81/PMR/rp8G3op/n58B3pr/HIj/XMeBW4Fj/NtcAh4M7PKfA/Gf762Bn+Lf5m2An+Y/D+K/xlcDH8W/ztcAH81/LsR/nb8GXooXzd8AL81/PsR/nQcDfw0c44W7BLw0cCv/+RD/td4a+CleuLcBfpr/Goj/ep8NfBbP3+cAn81/HcR/j58G3orn9D3Ae/NfC/Hf4zjw28BLccXfAK8N7PJfC/Hf5zhwK1c8GNjlvx7iv9dLc8Vf898D8f8b4v83xP9viP/fEP+/8Y8DHZxBeLG/SAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRoom;
impl IconShape for MdRoom {
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
                d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwW/za/DewCPw38DLDLvx3iX2ae1+sAv81zem3gt3he4nn9NvBa/Mf4buBjgF3+9RD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiP84u8DHAd/Ovg/iXmef1OsBv85xeG/gtnpd4Xr8NvBb/8b4beB9edIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxn+O7gffhRYP4l702z+uvgV2e03HgpXlev83zemngOP96rw0cB94aeBAv2PsA382/DPG/13sDXw0c43ntAg8BdnnhEP+7HQd+G3gpntf3AO/NC4f43+84cCtwjOd1AtjlBUP83/DewHfxvN4H+G5eMMS/7LV4Xn8D7PI/y63Ag3hOPwO8NS8Y4l9mntfrAL/N/yxfDXwUz+l3gNfmBUP8y8zzeh3gt/mf5bOBz+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zf8drAb/G8xAuG+L/jtYHf4nmJFwzxf8drA7/F8xIvGOL/jtcGfovnJV4wxP8drw38Fs9LvGCIf9lv8bw+Bvhr/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLfpvn9dHAX/M/y0sDX83zem1eMMT/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX/baPK+/Bnb5n+U48NI8r9/mBUP8/4b4/w3x/xvi/zfEv+y1eF5/A+zynI4DL8Xz+h2e10sDx3hOl4C/5nm9Fs/rb4BdntNx4KV4Xr/DC4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSH2plBc0HdkwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRoundedCorner;
impl IconShape for MdRoundedCorner {
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
                d: "M19,19h2v2h-2V19z M19,17h2v-2h-2V17z M3,13h2v-2H3V13z M3,17h2v-2H3V17z M3,9h2V7H3V9z M3,5h2V3H3V5z M7,5h2V3H7V5z M15,21h2v-2h-2V21z M11,21h2v-2h-2V21z M15,21h2v-2h-2V21z M7,21h2v-2H7V21z M3,21h2v-2H3V21z M21,8c0-2.76-2.24-5-5-5h-5v2h5 c1.65,0,3,1.35,3,3v5h2V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRowing;
impl IconShape for MdRowing {
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
                d: "M8.5,14.5L4,19l1.5,1.5L9,17h2L8.5,14.5z M15,1c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S16.1,1,15,1z M21,21.01L18,24 l-2.99-3.01V19.5l-7.1-7.09C7.6,12.46,7.3,12.48,7,12.48v-2.16c1.66,0.03,3.61-0.87,4.67-2.04l1.4-1.55 C13.42,6.34,14.06,6,14.72,6h0.03C15.99,6.01,17,7.02,17,8.26v5.75c0,0.84-0.35,1.61-0.92,2.16l-3.58-3.58v-2.27 c-0.63,0.52-1.43,1.02-2.29,1.39L16.5,18H18L21,21.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/7bXBn6bFwzxf9d3Ae8NvA/w3Tx/iP+bvgt4b57tfYDv5nkh/u/5LuC9eV7vA3w3zwnxf8t3Ae/N8/c3wEvznBD/d3wX8N48f38DvDawy3NC/M90HPgq4GOAXf5l3wW8N8/f3wCvDezyvBD/8xwHfgt4aeCvgdcBdnnBvgt4b56/vwFeG9jl+UM8fy8NHOM/z98Auzyv48BvAS/Ns/018DrALs/ru4D35vn7G+C1gV1eMMTz99vAa/Gf53WA3+Y5HQd+C3hpntdfA68D7PJs3wW8N8/f3wCvDezywiGev98GXov/PK8D/DbP67uB9+L5+2vgdYBd4LuA9+b5+xvgtYFd/mWI5++3gdfiP8/rAL/N8/fdwHvx/P018NfAe/P8/Q3w2sAuLxrE8/fbwGvxn+d1gN/mBftu4L341/kb4LWBXV50iOfvt4HX4j/P6wC/zQv33cB78aL5G+C1gV3+dRD/s3038F68cH8DvDawy78e4n++7wbei+fvb4DXBnb5t0H87/DdwHvxnP4GeG1gl387xP8e3w28F1f8DfDawC7/Poj/Xb4beGngtYFd/v0Q//scB3b5j4H4/w3x/xvif6aXBr4KeBtglxfdewOvBbwPLxrE/zwvDfwWcBz4a+B1gF3+Ze8NfBdXfDfwPvzLEP+zvDTwW8Bxnu2vgdcBdnnB3hv4Lp7TdwPvwwuH+J/jwcBfAcd5Xn8NvA6wy/N6b+C7eP6+G3gfXjDE/yzfDbwXz99fA68D7PJs7w18Fy/Y+wDfzQuG+J/nu4H34vn7a+B1gF3gvYHv4gV7H+C7eeEQ/zN9N/BePH9/DXwP8FW8YO8DfDf/MsT/XN8NvBf/eu8DfDcvGsTz99vAa/Gf53WA3+Zf9t3Ae/Giex/gu3nRIZ6/3wZei/88rwP8Ni+a7wbei3/Z+wDfzb8O4vn7beC1+M/zOsBv86L7beC1eME+Bvhq/vUQz99vA6/Ff57XAX6bF817A9/FC/fXwOsAu/zrIJ6/3wZei/88rwP8Nv+y9wa+ixfNXwOvA+zyokM8fy8NHOc/z18Du7xw7w18F/86fw28DrDLiwbxP9N7A9/FC/Y7wGvx/P018DrALv8yxP887w18Fy/Y+wDfDXw38F48f38NvA6wywuH+J/lvYHv4gV7H+C7ebbvBt6L5++vgdcBdnnBEP9zPBh4Oi/Y+wDfzfP6buC9eP5+B3htXjDE/yzvDXwXz+t9gO/mBftu4L14TpeA1wb+mhcM8T/PewPfxbO9D/Dd/Mu+G3gvrrgEvDbw17xwiP+Z3hv4LuB9gO/mRffdwFsDrw38Nf8yxP9cDwZu5V/vwcCtvGgQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BJc+uQXzZ++8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRule;
impl IconShape for MdRule {
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
                d: "M16.54,11L13,7.46l1.41-1.41l2.12,2.12l4.24-4.24l1.41,1.41L16.54,11z M11,7H2v2h9V7z M21,13.41L19.59,12L17,14.59 L14.41,12L13,13.41L15.59,16L13,18.59L14.41,20L17,17.41L19.59,20L21,18.59L18.41,16L21,13.41z M11,15H2v2h9V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSavedSearch;
impl IconShape for MdSavedSearch {
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14zm-2.17-1.5l2.14-1.53 2.14 1.53-.83-2.46 2.15-1.5h-2.62L9.47 6l-.84 2.54H6l2.14 1.49z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSchedule;
impl IconShape for MdSchedule {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
            path {
                d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScheduleSend;
impl IconShape for MdScheduleSend {
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
                d: "M16.5 12.5H15v4l3 2 .75-1.23-2.25-1.52V12.5zM16 9L2 3v7l9 2-9 2v7l7.27-3.11C10.09 20.83 12.79 23 16 23c3.86 0 7-3.14 7-7s-3.14-7-7-7zm0 12c-2.75 0-4.98-2.22-5-4.97v-.07c.02-2.74 2.25-4.97 5-4.97 2.76 0 5 2.24 5 5S18.76 21 16 21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFN0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/fSwHsBL80Vr80VtwK3csVPAz8D3Mp/L8R/jOPAVwFvDRznRffXwFcD38N/D8S/z3Hgo4CPBo7zb3cr8DHAT/NfC/Fv99LAdwEvzX+c7wY+Btjlvwbi3+atge8CjvMf76+B1wF2+c+H+Nd7a+Cn+Jf9DPDXXPHXwEtzxVsDL8ULdyvwMsAu/7kQ/zovDfwWcJzn7xnAZwPfzQv3YOC9gc/iBftt4HX4z4V40R0Hfgt4aZ6/jwG+mn+dBwPfDbwWz9/XAB/Nfx7Ei+6zgc/ieV0C3hv4af7tvht4L56/hwC38p8D8aI5DjwdOM7zehvgp/n3+27gvXhevw28Dv85EC+a7wbei+f1McBX8x/nr4GX4nm9DvDb/MdDvGguAsd5Ts8AHsx/rNcGfovn9TXAR/MfD/Eve2ngr3he7wN8N//xfht4LZ7TrcBD+I+H+Jd9NfBRPC/xn+O9ge/ieb0M8Nf8x0L8y34beC2e088Ab81/juPARZ7X+wDfzX8sxL/st4HX4jl9DvDZ/Of5a+CleE6fA3w2/7EQ/zLzvD4H+Gz+8/w28Fo8p88BPpv/WIh/mXlebwP8NP95fht4LZ7TzwBvzX8sxL/sVuBBPKfPAT6b/zy/DbwWz+lzgM/mPxbiX/bbwGvxnD4H+Gz+81wEjvOcPgf4bP5jIf5lvw28Fs/pr4GX4T/Hg4Gn87zeB/hu/mMh/mUfDXwVz+shwK38x/to4Kt4XieAXf5jIf5lDwaezvP6HOCz+Y/3dODBPKe/AV6a/3iIF81fAy/Fc9oFXga4lf847w18F8/rc4DP5j8e4kXz3sB38bx+G3gd/mO8NPBbwHGe0yXgwcAu//EQL7pbgQfxvL4beB/+fY4DvwW8NM/rc4DP5j8H4kX31sBP8fx9N/A+/Nu8NPBdwEvzvC4BDwZ2+c+B+Nf5buC9eP7+GvgY4Ld50b0X8NXAcZ6/1wF+m/88iH+d48BvAy/FC/bTwE8DPwPs8rweDLwV8NHAg3nBfgZ4a/5zIf71jgN/DTyIf9lfA7s820sDx3nR/TZXfA/w3fzHQ/zbHAd+Gngt/uP9DPBWPK/3Ab6b/1iIf5+vBj6K/xiXgLcGfhv4beC1eF7vA3w3/3EQ/34PBr4beC3+bS4BXw18NbDLFb8NvBbP3/sA381/DMR/nNcG3hp4a+BB/Mv+Bvhp4KuBXZ7TewPfxQv2PsB38++H+M/x0sBLAw/mipcG/porbgV+GtjlhXtv4Lt4wd4H+G7+fRD/s7038F28YO8DfDf/doj/+d4b+C5esPcBvpt/G8T/Du8NfBcv2PsA382/HuJ/j/cGvosX7H2A7+ZfB/G/y3sD38UL9j7Ad/OiQ/zv897Ad/GCvQ/w3bxoEP87vTfwXbxg7wN8N/8yxP9e7w18Fy/Y2wA/zQuH+N/tvYHv4nn9DfDawC4vHOJ/v/cGvotn+xvgtYFd/mWI/xveG/gu4G+A1wZ2edEg/u94a+C3gV1edIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R93ktEF/da30AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSearch;
impl IconShape for MdSearch {
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSearchOff;
impl IconShape for MdSearchOff {
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
                d: "M15.5,14h-0.79l-0.28-0.27C15.41,12.59,16,11.11,16,9.5C16,5.91,13.09,3,9.5,3C6.08,3,3.28,5.64,3.03,9h2.02 C5.3,6.75,7.18,5,9.5,5C11.99,5,14,7.01,14,9.5S11.99,14,9.5,14c-0.17,0-0.33-0.03-0.5-0.05v2.02C9.17,15.99,9.33,16,9.5,16 c1.61,0,3.09-0.59,4.23-1.57L14,14.71v0.79l5,4.99L20.49,19L15.5,14z",
            }
            polygon {
                points: "6.47,10.82 4,13.29 1.53,10.82 0.82,11.53 3.29,14 0.82,16.47 1.53,17.18 4,14.71 6.47,17.18 7.18,16.47 4.71,14 7.18,11.53",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABy0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8Z/vt/jv9Tq8YIj/fOa/l3jBEP/5zH8v8YIh/vOZ/17iBUP85zP/vcQLhvjP99v893ptXjDE/2+I/98Q/78h/n9D/P+G+P8N8f8b4j/fa/Hf63d4wRD/+cx/L/GCIf7zmf9e4gVD/Ocz/73EC4b4z2f+e4kXDPGfz/z3Ei8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CBKjEkEH025cAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSegment;
impl IconShape for MdSegment {
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
                d: "M9 18h12v-2H9v2zM3 6v2h18V6H3zm6 7h12v-2H9v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSendAndArchive;
impl IconShape for MdSendAndArchive {
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
                d: "M21 10h-3L2 3v7l9 2-9 2v7l8-3.5V21c0 1.1.9 2 2 2h9c1.1 0 2-.9 2-2v-9c0-1.1-.9-2-2-2zm0 11h-9v-9h9v9zm-4.5-1L13 16h2v-3h3v3h2l-3.5 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+63wUcJwXzS7wNfznQ/zXeG/gu/jXeR/gu/nPhfiv8dnAZ/Gv8znAZ/OfC/Ff46eBt+Jf53eA1+Y/F+K/xm8Dr8W/zl8DL8N/LsR/DfNvI/5zIf7zHQcu8m9zAtjlPw/iP99rA7/Fv83rAL/Nfx7Ev957A+8F3Ap8DnArL9iDga8C3pp/m58GPga4lRfswcBHAS8NfA/w3bzoEP86x4GnA8d5tt8Gvhv4Hp7tOPBVwHvzH+O7gY8Bdnm29wLeG3htnm0XeAiwy4sG8a/z1cBH8fztAt8N/DXw1cBx/mPtAh8NvDTw3sBxnr/vAd6bFw3iRfdg4On87/AywF/zL0O86H4LeG3+d/ht4HX4lyFeNG8N/BT/u7wP8N28cIgXzdOBB/O/y63AywC7vGCIf9lnA5/F/06fA3w2LxjihTsOPB04zn+svwF+muf01sBL8R9rF3gZ4FaeP8QL993Ae/Ef52uAzwZ2ef6OA58NfBT/cb4HeG+eP8QL99vAa/Hvdwl4beCvedG8NPDbwDH+/Z4BPJjnD/HCvTTw08CD+Le7BDwY2OVf5zhwK3CMf7tLwGsDf83zh/iXHQd+Gngt/m1eBvhr/m1eGvgr/m1+B3hrYJcXDPGi+2zgs/jX+Rrgo/n3+W7gvfjX+Rrgo/mXIf513hr4buAYL5oTwC7/Pg8Gns6L5hLw3sBP86JB/Os9GHg6/7K/AV6a/xh/DbwU/7KXAf6aFx3iX+84cJF/2ecAn81/jM8GPot/2UOAW3nRIf71Xhv4Lf5lnwN8Nv8xPhv4LP5lrwP8Ni86xL/eawO/xb/sc4DP5j/GZwOfxb/sdYDf5kWH+Nd7MPB0/mWfA3w2/zE+G/gs/mUPAW7lRYf413tp4K/4l/018DL8x/gr4KX5l70M8Ne86BD/Ou8NfBVwnBfNCWCXf58HA0/nRbMLfAzw3bxoEC+6rwI+mn+drwE+mn+f7wbei3+drwY+hn8Z4l92HPgt4KX5t3kZ4K/5t3lp4K/4t/lt4G2AXV4wxAv30sBvAcf5t9sFHgLs8q9zHHg6cJx/u13gdYC/5vlDvHC/DbwW/367wOsAf82L5qWB3wKO8+/3O8Br8/whXrjvBt6L/zjfDXwOcCvP34OBzwLem/843wO8N88f4oV7MPDXwDH+Y/018NM8p7cGXpr/WJeABwO7PH+If9lnA5/F/06fA3w2LxjiX3Yc+GvgQfzv8gzgwbxwiBfNewPfxf8ubwP8NC8c4kX328Br8b/D7wCvzb8M8aJ7aeCv+N/hIcCt/MsQ/zrfDbwXz98l4LuBvwa+GjjGf6xLwEcDLw28N3CM5+9rgI/mRYP41zkO3Aoc49l+B/hu4Lt5tuPAVwPvxX+M7wE+Gtjl2d4beG/gtXi2S8CDgV1eNIh/vfcG3hv4a+CrgVt5wR4MfDXwVvzb/Azw0cCtvGAPBj4beDDw3cB386JD/Od7beC3+Ld5HeC3+c+D+M/3YODp/NucAHb5z4P4r2H+bcR/LsR/jb8GXop/nd8BXpv/XIj/Gr8NvBb/Oj8DvDX/uRD/NT4b+Cz+dT4H+Gz+cyH+a7w38F3867wP8N3850L81/lo4Dgvml3gq/nPh/j/DfH/G+L/N8T/b4j/3xD/v/GPauzBQXqWl5gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettings;
impl IconShape for MdSettings {
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
                d: "M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4GuAv+ZfhnjRvDfwXfzv8j7Ad/PCIf5lLw38Ff87PQS4lRcM8S/7buC9+N/pe4D35gVD/MsuAsf532kXOMELhviXmf84vwN8NbDL83cc+GjgtfiPI14wxL/M/Md5HeC3eeFeG/gt/uOIFwzxLzP/cR4C3MoL92Dg6fzHES8Y4l9m/uOIF435jyNeMMS/zPzH+BvgpXnR3Ao8iP8Y4gVD/MvMf4zfAV6bF81vA6/FfwzxgiH+ZeZf9jPAWwNvDbw38FY8p+8BPhu4lRfNg4HPBt6L5/Q9wHcDvw38NPBW/MvEC4b4l5kX7hLw0sCtPNuDgfcGXhv4bOC3+bd5aeCzgb8GvhrY5dkeDPw1cIwXTrxgiH+ZeeE+B/hs/nt8NvBZvHDiBUP8y8wL9gzgpYFd/nscB/4aeBAvmHjBEP8y84K9D/Dd/Pd6b+C7eMHEC4b4l5nn73eA1+Z/ht8GXovnT7xgiH+Zef5eB/ht/md4aeCveP7EC4b4l5nnT/z7PBh4EFc8A7iVfx/z/IkXDPEvM8/fdwMfA+zyr3Mc+C7grXlOPw28D7DLv85x4LuAt+b5Ey8Y4l9mXrC/Bt4H+GteNMeBpwPHef52gYcAu7xoXhr4LuClecHEC4b4l5kXbhd4H+Cn+Zf9FPDWvHA/DbwN/7L3Br4KOM4LJ14wxL/MvGjEC/dg4Om8aB4C3MoLZ1404gVD/MvMi0a8cK8N/BYvmtcBfpsXzrxoxAuG+JeZF4144V4b+C1eNK8D/DYvnHnRiBcM8S8zLxrxwj0YeDovmocAt/LCmReNeMEQ/zLzwl0CPhr4bv5lPw28FS/czwBvzb/srYHvBo7xwokXDPEvMy/Y3wDvDfw1L5rjwK3AMZ6/vwFeG9jlRfPSwHcDL8ULJl4wxL/MPH8/A7w3sMu/znHgu4G34jn9DPDewC7/OseBrwbei+dPvGCIf5l5/sS/z4OBB3PFrcCt/PuY50+8YIh/mXn+Xgb4a/5neG3gt3j+xAuG+JeZ5++3gdfhf4bfAl6b50+8YIh/mXnB3gb4af57vTfwXbxg4gVD/MvMC3Yr8DLALv89jgN/BTyYF0y8YIh/mXnhPgf4bP57fDbwWbxw4gVD/MvMC7cLvAxwK8/2YOC9gJcGPgf4a/5tXhv4LOC3ge8BbuXZHgw8nX+ZeMEQ/zLzL/tp4G2AtwLeG3hrntN3A58D3MqL5sHAZwHvzXP6aeC7gZ8Bfgp4a/5l4gVD/MvMf4zfAV6bF81vA6/FfwzxgiH+ZeY/xl8DL8OL5unAg/mPIV4wxL/M/McRLxrzH0e8YIh/mfmP8xDgVl64lwb+iv844gVD/MvMf5zXAX6bF+61gd/iP454wRD/MvMf57eBrwZ2ef6OA58NvDT/ccQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8y14a+Cv+d3oIcCsvGOJF897Ad/G/y/sA380Lh3jRPRj4bOCtgWP8z3QJ+Gngs4Fb+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y89/MBBIELOTgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsApplications;
impl IconShape for MdSettingsApplications {
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
                d: "M12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm7-7H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-1.75 9c0 .23-.02.46-.05.68l1.48 1.16c.13.11.17.3.08.45l-1.4 2.42c-.09.15-.27.21-.43.15l-1.74-.7c-.36.28-.76.51-1.18.69l-.26 1.85c-.03.17-.18.3-.35.3h-2.8c-.17 0-.32-.13-.35-.29l-.26-1.85c-.43-.18-.82-.41-1.18-.69l-1.74.7c-.16.06-.34 0-.43-.15l-1.4-2.42c-.09-.15-.05-.34.08-.45l1.48-1.16c-.03-.23-.05-.46-.05-.69 0-.23.02-.46.05-.68l-1.48-1.16c-.13-.11-.17-.3-.08-.45l1.4-2.42c.09-.15.27-.21.43-.15l1.74.7c.36-.28.76-.51 1.18-.69l.26-1.85c.03-.17.18-.3.35-.3h2.8c.17 0 .32.13.35.29l.26 1.85c.43.18.82.41 1.18.69l1.74-.7c.16-.06.34 0 .43.15l1.4 2.42c.09.15.05.34-.08.45l-1.48 1.16c.03.23.05.46.05.69z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsBackupRestore;
impl IconShape for MdSettingsBackupRestore {
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
                d: "M14 12c0-1.1-.9-2-2-2s-2 .9-2 2 .9 2 2 2 2-.9 2-2zm-2-9c-4.97 0-9 4.03-9 9H0l4 4 4-4H5c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.51 0-2.91-.49-4.06-1.3l-1.42 1.44C8.04 20.3 9.94 21 12 21c4.97 0 9-4.03 9-9s-4.03-9-9-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsBluetooth;
impl IconShape for MdSettingsBluetooth {
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
                d: "M11 24h2v-2h-2v2zm-4 0h2v-2H7v2zm8 0h2v-2h-2v2zm2.71-18.29L12 0h-1v7.59L6.41 3 5 4.41 10.59 10 5 15.59 6.41 17 11 12.41V20h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 3.83l1.88 1.88L13 7.59V3.83zm1.88 10.46L13 16.17v-3.76l1.88 1.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsBrightness;
impl IconShape for MdSettingsBrightness {
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02zM8 16h2.5l1.5 1.5 1.5-1.5H16v-2.5l1.5-1.5-1.5-1.5V8h-2.5L12 6.5 10.5 8H8v2.5L6.5 12 8 13.5V16zm4-7c1.66 0 3 1.34 3 3s-1.34 3-3 3V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviP82Dgs4C3Bo7zn2MX+Gngc4Bb+fdD/Md4b+C7+K/1PsB38++D+Pd7MPB0/ns8BLiVfzvEv993A+/Ff4/vAd6bfzvEv99F4Dj/PXaBE/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pczz+tvgF3+Yx0HXornJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee10cDf81/rJcGvprnJf7tEP9+5r+X+LdD/PuZ/17i3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PuZ/17i3w7x7/fXwEvx3+N3gNfm3w7x7/fewHfx3+N9gO/m3w7xH+O7gffiv9b3AO/Nvw/iP85nAx8NHOM/1yXgq4HP5t8P8R/vtfnP9dv8x0H8/4b4/w3x/xvi/zfEf5zjwEvxvH6H5/XSwDGe0yXgr3ler8Xz+htgl38/xH+c1wZ+i+clntdvA6/Fc/od4LV5XuZ5vQ7w2/z7If7jvDbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+bfD/Ef57WB3+J5ief128Br8Zx+B3htnpd5Xq8D/Db/foj/OK8N/BbPSzyv3wZei+f0O8Br87zM83od4Lf590P8x3lt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nvx//CIxvokF4sw57AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsCell;
impl IconShape for MdSettingsCell {
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
                d: "M7 24h2v-2H7v2zm4 0h2v-2h-2v2zm4 0h2v-2h-2v2zM16 .01L8 0C6.9 0 6 .9 6 2v16c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V2c0-1.1-.9-1.99-2-1.99zM16 16H8V4h8v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/v1eGvhr/nu8NPDX/Nsh/n1eGvgt4KeBjwF2+a9xHPgq4K2B1wH+mn8bxL/dceCvgAdzxV8DrwPs8p/rOPBbwEtzxa3AywC7/Osh/u3+CnhpntMu8DrAX/Of46WB3wKO85z+GngZ/vUQ/zbfBbw3z9/7AN/Nf473Br6L5++7gffhXwfxr/fRwFfx/H0O8Nn85/ps4LN4/j4G+GpedIh/nbcGforn73uA9+a/xncD78Xz9zrAb/OiQbzoXhr4LeA4z+tvgNcGdvmvcRz4beCleF67wOsAf82/DPGiOQ78FvDSPK9LwIOBXf5rHQduBY7xvP4aeB1glxcO8aL5LeC1eV6XgNcG/pr/Hi8N/DZwjOf128Dr8MIh/mXfBbw3z9/7AN/Nf6/3Br6L5++7gffhBUO8cO8NfBfP3+cAn83/DJ8NfBbP3/sA383zh3jBXhv4LZ6/7wHem/9Zfhp4K56/1wF+m+eFeP5eGvgt4DjP62+A1wZ2+Z/lOPDbwEvxvHaB1wH+mueEeF7Hgd8CXprndQl4MLDL/0wPBv4aOMbz+mvgdYBdng3xvH4LeG2ev5cB/prn76WBr+J5vQ7P66uBl+I5/Q3w0Tyv3+J5fQzw1zx/Lw38Fc/fbwOvw7MhntN3Ae/N8/c+wHfzgr028Fs8L/G8fht4LZ7T7wCvzfMyz+t1gN/mBXtv4Lt4/r4beB+uQDzbewPfxfP3NcBH88K9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5oX7auCjeP7eB/huAPFs7w18F8/f1wAfzQv32sBv8bzE8/pt4LV4Tr8DvDbPyzyv1wF+mxfuq4GP4vl7H+C7AcRz+m7gvXj+3gf4bl6w1wZ+i+clntdvA6/Fc/od4LV5XuZ5vQ7w27xg7w18F8/f9wDvzRWI5/XbwGvx/L0M8Nc8fy8NfDXP67V5Xl8NvDTP6a+Bj+Z5/TbP66OBv+b5e2ngr3j+fgd4bZ4N8byOA78NvBTPaxd4CLDL/0wPBv4KOM7z+hvgtYFdng3x/L008NvAMZ7XXwOvA+zyP8tx4LeAl+Z5XQJeG/hrnhPiBXtt4Ld4/r4beB/+Z/kp4K15/l4H+G2eF+KFe2/gu3j+Pgf4bP5n+Gzgs3j+3gf4bp4/xL/su4H34vl7H+C7+e/13sB38fx9D/DevGCIF81vA6/F89oFXgf4a/57vDTwW8BxntfvAK/NC4d40RwHfht4KZ7XLvAQYJf/WseBpwPHeV5/A7w2sMsLh3jRvTTw28AxntdfA68D7PJf4zjwW8BL87wuAa8N/DX/MsS/zlsDP8Xz993A+/Bf47uA9+b5ex3gt3nRIP71Phr4Kp6/zwE+m/9cnw18Fs/f+wDfzYsO8W/z3cB78fy9D/Dd/Od4b+C7eP6+B3hv/nUQ/3Z/DbwUz+kS8NrAX/Of46WB3waO8Zz+Bnhp/vUQ/3bHgb8GHsQVfwO8NrDLf67jwG8DL8UVzwBeGtjlXw/x7/PSwG8DPw18NLDLf43jwFcDbw28NvDX/Nsg/v0eDNzKf4+XBv6afzvE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EApnEQR1CSkUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsEthernet;
impl IconShape for MdSettingsEthernet {
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
                d: "M7.77 6.76L6.23 5.48.82 12l5.41 6.52 1.54-1.28L3.42 12l4.35-5.24zM7 13h2v-2H7v2zm10-2h-2v2h2v-2zm-6 2h2v-2h-2v2zm6.77-7.52l-1.54 1.28L20.58 12l-4.35 5.24 1.54 1.28L23.18 12l-5.41-6.52z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/3oOBB/H8PQO4lf86iP9cx4HXAl4beGngtXnR/Dbw18BvA78D7PKfA/Ef7zjwVsBbA2/Nf4yfBn4a+Blgl/84iP84x4GPAj4aOM5/jl3gq4GvAXb590P8x/gs4KOB4/zX2AW+Gvgc/n0Q/z6vDXwX8GD+e9wKvA/w2/zbIP7tPgv4bF40l4CfBm4Ffhu4FbiV5/Rg4MHAawMPBt4aOMaL5rOBz+FfD/Gvdxz4KeC1eeEuAd8NfDfw1/zbvDTw3sB7A8d44X4beBtglxcd4l/nOPBbwEvzgj0D+Grgu4Fd/mMcB94b+GjgQbxgfw28DrDLiwbxojsO/Bbw0rxgnwN8NbDLf47jwEcDn8UL9tfA6wC7/MsQL5rjwG8BL83z9zfAewN/zX+NlwZ+GngQz99fA68D7PLCIV40vwW8Ns/f9wAfDezyLzsOvBTw2lzxYK64lSt+G/gbYJd/2XHgu4G34vn7beB1eOEQ/7LPBj6L5+97gPfmhXsw8FbAWwOvzYvmt4GfBn4GuJUX7ruB9+L5+xzgs3nBEC/cawO/xfP3PcB784IdBz4K+Gz+7XaBrwa+BtjlBftu4L14/l4H+G2eP8QL93TgwTyv7wHemxfso4DPBo7zH2MX+Bjgu3nBfhp4K57XrcBDeP4QL9hnA5/F8/ob4LWBXZ6/7wLem/8c3w28D8/fceCvgQfxvD4H+GyeF+L5Ow48HTjO83oZ4K95XseB3wJemhfN7/CcXosXzV8DrwPs8rxeGvgrntcu8BBgl+eEeP4+G/gsntfnAJ/N8/dXwEvzgj0D+G7gp4G/5vl7aeCtgfcGHsQL9tfAy/D8fTbwWTyvzwE+m+eEeF7HgacDx3lOzwBeGtjleX0X8N48f5eAjwa+m3+djwY+GzjG8/fdwPvwvI4Dfw08iOe0CzwE2OXZEM/rvYHv4nl9DPDVPK+PBr6K5+9vgNcGdvm3OQ78NvBSPH/vA3w3z+uzgc/ieb0P8N08G+J5/TTwVjynS8CDgV2e03Hg6cBxntf3AO/Nf4zvBt6L57ULPATY5TkdB24FjvGcfgZ4a54N8ZyOAxd5Xl8DfDTP67OBz+J5/Q3w0vzH+mvgpXhenwN8Ns/ru4H34nmdAHa5AvGc3hr4KZ7XywB/zXN6MPB0ntcl4MHALv+xjgO3Asd4TrvAywC38pxeGvgrntfbAD/NFYjn9N3Ae/GcLgHHeV4fDXwVz+t9gO/mhXtp4LWAl+aKvwZ+B/hrXriPBr6K5/UxwFfzvHaBYzynrwE+misQz+m3gdfiOX0P8N48r98GXovn9Azgwbxgx4GvAt6b5++7gY8BdnnBbgUexHP6HeC1eV4/DbwVz+l3gNfmCsRzMs/rY4Cv5jkdBy7yvD4H+Gyev+PAbwEvzQv318DrALs8f58NfBbP6wSwy3P6bOCzeF7iCsSzPRh4Os/rdYDf5jm9NvBbPK+XAf6a5++7gffiRfM9wHvz/L008Fc8r9cBfpvn9NrAb/G8TgC7AOLZXhv4LZ7XQ4BbeU6fDXwWz0s8fy8N/BX/Og8BbuX5M8/rY4Cv5jk9GHg6z+t1gN8GEM/22sBv8bzE8/ps4LN4Tr8DvDbP30cDX8W/zscAX83z99vAa/GcPgf4bJ6XeV6vA/w2gHi21wZ+i+clntd3A+/Fc/od4LV5/r4beC/+db4G+Giev98GXovn9DXAR/O8zPN6HeC3AcR/ja8GPop/na8BPpr/XIj/Gp8NfBb/Op8DfDb/uRD/NR4MPJ1/nYcAt/KfC/Ff57uB9+JF8z3Ae/OfD/Ff5zjw28BL8cL9DfDawC7/+RD/tY4Dnw18FM/f1wCfDezyXwPx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Nd7b+C7eP7eB/hu/usg/mu9N/BdvHDvA3w3/zUQ/3XeG/guXjTvA3w3//kQ/zXeG/gu/nXeB/hu/nMh/vO9N/Bd/Nu8D/Dd/OdB/Od6b+C7eMHehyu+ixfsfYDv5j8H4j/PewPfxQv2PsB3c8V7A9/FC/Y+wHfzHw/xn+O9ge/iBXsf4Lt5Tu8NfBcv2PsA381/LMR/vPcGvosX7H2A7+b5e2/gu3jB3gf4bv7jIP7jvTfwXTx/7wN8Ny/cewPfxfP3PsB38x8H8Z/jvYHv4jm9D/DdvGjeG/guntP7AN/NfyzEf573Br6LK94H+G7+dd4b+C6ueB/gu/mPh/jP9d5c8d3827w3V3w3/zkQ/78h/n9D/P+G+P+NfwT4jS1QKNTvYwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsInputAntenna;
impl IconShape for MdSettingsInputAntenna {
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
                d: "M12 5c-3.87 0-7 3.13-7 7h2c0-2.76 2.24-5 5-5s5 2.24 5 5h2c0-3.87-3.13-7-7-7zm1 9.29c.88-.39 1.5-1.26 1.5-2.29 0-1.38-1.12-2.5-2.5-2.5S9.5 10.62 9.5 12c0 1.02.62 1.9 1.5 2.29v3.3L7.59 21 9 22.41l3-3 3 3L16.41 21 13 17.59v-3.3zM12 1C5.93 1 1 5.93 1 12h2c0-4.97 4.03-9 9-9s9 4.03 9 9h2c0-6.07-4.93-11-11-11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsInputComponent;
impl IconShape for MdSettingsInputComponent {
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
                d: "M5 2c0-.55-.45-1-1-1s-1 .45-1 1v4H1v6h6V6H5V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2H9v2zm-8 0c0 1.3.84 2.4 2 2.82V23h2v-4.18C6.16 18.4 7 17.3 7 16v-2H1v2zM21 6V2c0-.55-.45-1-1-1s-1 .45-1 1v4h-2v6h6V6h-2zm-8-4c0-.55-.45-1-1-1s-1 .45-1 1v4H9v6h6V6h-2V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2h-6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsInputComposite;
impl IconShape for MdSettingsInputComposite {
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
                d: "M5 2c0-.55-.45-1-1-1s-1 .45-1 1v4H1v6h6V6H5V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2H9v2zm-8 0c0 1.3.84 2.4 2 2.82V23h2v-4.18C6.16 18.4 7 17.3 7 16v-2H1v2zM21 6V2c0-.55-.45-1-1-1s-1 .45-1 1v4h-2v6h6V6h-2zm-8-4c0-.55-.45-1-1-1s-1 .45-1 1v4H9v6h6V6h-2V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2h-6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEr0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sd4KeGngtfnP8dvAXwM/w38MxH+MBwM/Bbw0/zX+Gngb4Fb+fRD/fseBvwIezH+tvwZeB9jl3w7x7/fZwGfx3+NzgM/m3w7x7/d04MH897gVeAj/doh/P/O8/gbY5T/WceCleF7i3w7x72ee1+sAv81/rNcGfovnJf7tEP9+5nm9DvDb/Md6beC3eF7i3w7x72ee1+sAv81zemngq7jiY4C/5jm9NPBVXPExwF/znF4b+C2el/i3Q/z7mef1OsBv85xeG/gtrngd4Ld5Tq8N/BZXvA7w2zyn1wZ+i+cl/u0Q/37meb0O8Ns8p9cGfosrXgf4bZ7TawO/xRWvA/w2z+m1gd/ieYl/O8S/n3lerwP8Ns/ptYHf4orXAX6b5/TawG9xxesAv81zem3gt3he4t8O8e9nntfrAL/Nc3pt4Le44nWA3+Y5vTbwW1zxOsBv85xeG/gtnpf4t0P827w0cIwrfpvn9dHAX/OcXhr4aq74aOCveU4vDXw1V3w08Nc8p5cGvprn9dpccQn4a/51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5ufBt6K/1l+Bnhr/nUQ/zavDfwW/7O8DvDb/Osg/u1+G3gt/mf4HeC1+ddD/Nu9NvBb/M/wOsBv86+H+Pf5beC1+O/1O8Br82+D+Pd5beC3+O/1OsBv82+D+Pf7beC1+O/xO8Br82+H+Pd7beC3+O/xOsBv82+H+I/x28Br8V/rd4DX5t8H8R/jtYHf4r/W6wC/zb8P4j/ObwOvxX+N3wFem38/xH+c1wZ+i/8arwP8Nv9+iP9Yvw28Fv+5fgd4bf5jIP5jvTbwW/zneh3gt/mPgfiP99vAa/Gf43eA1+Y/DuI/3msDv8V/jtcBfpv/OIj/HH8NvBT/sf4GeGn+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/xy/DbwW/7F+B3ht/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cXw28NP+x/hr4aP5jIf5/Q/z/hvj/DfH/G+L/N/4RhUaSQZkOnmYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsInputHdmi;
impl IconShape for MdSettingsInputHdmi {
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
                d: "M18 7V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v3H5v6l3 6v3h8v-3l3-6V7h-1zM8 4h8v3h-2V5h-1v2h-2V5h-1v2H8V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsInputSvideo;
impl IconShape for MdSettingsInputSvideo {
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
                d: "M8 11.5c0-.83-.67-1.5-1.5-1.5S5 10.67 5 11.5 5.67 13 6.5 13 8 12.33 8 11.5zm7-5c0-.83-.67-1.5-1.5-1.5h-3C9.67 5 9 5.67 9 6.5S9.67 8 10.5 8h3c.83 0 1.5-.67 1.5-1.5zM8.5 15c-.83 0-1.5.67-1.5 1.5S7.67 18 8.5 18s1.5-.67 1.5-1.5S9.33 15 8.5 15zM12 1C5.93 1 1 5.93 1 12s4.93 11 11 11 11-4.93 11-11S18.07 1 12 1zm0 20c-4.96 0-9-4.04-9-9s4.04-9 9-9 9 4.04 9 9-4.04 9-9 9zm5.5-11c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm-2 5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xH+e41yxy38e87xeB/htAPFsrw38Fs9L/Of5LuA48Db85zHP63WA3wYQz/bawG/xvMR/jo8GvoorPgf4bP5zmOf1OsBvA4hne23gt3he4j/eawO/xXN6G+Cn+Y9nntfrAL8NIJ7ttYHf4nmJ/1gPBv4KOM5z2gVeBriV/1jmeb0O8NsA4tleG/gtnpf4j3Mc+C3gpXn+/hp4HWCX/zjmeb0O8NsA4tleG/gtnpf4j/NdwHvzwn038D78xzHP63WA3wYQz/bawG/xvMR/jOPAS/Oi+Wtgl/8Y5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYkX7Diwy3+v48AuL5h5Xq8D/DaAeLbXBn6L5yWev5cGfgp4CP+9ng68DfDXPH/meb0O8NsA4tleG/gtnpd4Xu8NfBVwHBD/vQzsAh8DfDfPyzyv1wF+G0A822sDv8XzEs/pq4CP5tnEfy/zbJ8NfA7PyTyv1wF+G0A822sDv8XzElccB74LeGuek/jvZZ7TTwPvA+xyhXlerwP8NoB4ttcGfovnJeClge8CXprnJf57mef118D7AH8NmOf1OsBvA4hne23gt3heAl4a+GngQTwv8d/LPK9nAG8N/DVgntfrAL8NIJ7ttYHf4nmJK44DPw28Fs9J/Pcyz+l3gLcGdrnCPK/XAX4bQDzbawO/xfMSz+mrgY/i2cR/L/NsXwN8NM/JPK/XAX4bQDzbawO/xfMSz+u9ga8GjgHiv5eBS8BHA9/N8zLP63WA3wYQz/bawG/xvMTz99LATwMP5r/XrcBbA3/N82ee1+sAvw0gnu21gd/ieYkX7Diwy3+v48AuL5h5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4j/GceCleNH8DbDLfwzzvF4H+G0A8WyvDfwWz0v8x/lu4L144b4HeG/+45jn9TrAbwOIZ3tt4Ld4XuI/znHgt4GX4vn7G+C1gV3+45jn9TrAbwOIZ3tt4Ld4XuI/1oOBvwaO8ZwuAS8N3Mp/LPO8Xgf4bQDxbK8N/BbPS/zHe23gt3hObwP8NP/xzPN6HeC3AcSzvTbwWzwv8Z/jo4Gv4orPAT6b/xzmeb0O8NsA4tleG/gtnpf4z/PdwHHgrfnPY57X6wC/DSCe7bWB3+J5if88x7lil/885nm9DvDbAOLZXhv4LZ7Xa/O/22/zvF4H+G0A8WyvDfwW/z+8DvDbAOI57QLH+L/tEnCcKxDP6bOBz+L/ts8BPpsrEM/ru4H34v+m7wHem2dDPH/vDXw08FL83/A7wHcD381zQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfARvD0EHy0EjzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsOverscan;
impl IconShape for MdSettingsOverscan {
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
                d: "M12.01 5.5L10 8h4l-1.99-2.5zM18 10v4l2.5-1.99L18 10zM6 10l-2.5 2.01L6 14v-4zm8 6h-4l2.01 2.5L14 16zm7-13H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO//Mh/mXfDbwX/3pvA/w0/7Mh/mXm3+Z7gPfmfzbEv8z82/wO8Nr8z4b4l5l/m98BXpv/2RD/sr8GXop/vd8BXpv/2RD/st8GXot/G/E/G+Jf9tvAa/FvI/5nQ/zLvhr4KP5tXgf4bf7nQvzLPhv4LP5tPgb4av7nQvzLXhv4Lf5tfgZ4a/7nQvzLHgw8nX878T8X4kVzK/Ag/m1eB/ht/mdCvGi+G3gv/m1eB/ht/mdCvGjeG/gu/vX+Bnhp/udCvGgeDDydf52/AV4b2OV/LsSL7lbgQbxo/gZ4bWCX5++3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiRffRwFfxL/sb4LWBXV6w3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiBfdg4Gn88L9DfDawC4v3G8Dr8Vz+h3gtXle5nm9DvDbPKfXBn6L5yVeMMS/zm8Dr8Xz9zfAawO7/Mt+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/nXeG/guntczgJcGdnnR/DbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zr3Qo8iOe0C7wMcCsvmpcGjvOcdoG/5nm9Ns/rr4FdntNx4KV5Xr/NC4b413tv4Lt4Xt8DvDf/uyD+bW4FHsTzehngr/nfA/Fv897Ad/G8/hp4Gf73QPzb/TbwWjyvzwE+m/8dEP92Lw38Fc/f6wC/zf98iH+frwY+iue1CzwE2OV/NsS/z3Hgt4GX4nn9NfA6wC7/cyH+/V4a+G3gGM/ru4H34X8uxH+M9wa+i+fvu4H34X8mxH+c7wbei+fvu4H34d/ntbjid/iPg/iP9dvAa/H8/TTwPsAu/zrHgd8CXppn+2ngr4HfBn6HfzvEf6zjwG8DL8Xz99fA2wC38qI5DvwW8NL8y36bZ7sV+Bngp3nhEP/xjgO/DbwUz98u8DbAb/PCHQd+C3hp/u0+B/hsXjDEf47jwG8DL8UL9tXA5wC7PK/jwG8BL82/z63AQ3jBEP95jgM/DbwWL9itwPsAv82zHQd+C3hp/mOIFwzxn++7gffihftu4HOAXeC3gJfmP454wRD/Nd4b+GrgGC/crcCD+Y8lXjDEf52XBr4beCn+a4kXDPFf6zjw2cBH8V9HvGCI/x4vDXw18Fr85xMvGOK/13sDnw08iP884gVD/M/w3sBnAw/iP554wRD/s7w38NbAW/EfR7xgiP+ZHgy8NfDRwIP49xEvGOJ/vgcDrw28NvDawIN40X0P8N68YIj/fR4MPBh4aeDBwEtzxXHgpXi27wE+GtjlBUP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGVgZRB3DXF2AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsPhone;
impl IconShape for MdSettingsPhone {
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
                d: "M13 9h-2v2h2V9zm4 0h-2v2h2V9zm3 6.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.58l2.2-2.21c.28-.27.36-.66.25-1.01C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 9v2h2V9h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsPower;
impl IconShape for MdSettingsPower {
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
                d: "M7 24h2v-2H7v2zm4 0h2v-2h-2v2zm2-22h-2v10h2V2zm3.56 2.44l-1.45 1.45C16.84 6.94 18 8.83 18 11c0 3.31-2.69 6-6 6s-6-2.69-6-6c0-2.17 1.16-4.06 2.88-5.12L7.44 4.44C5.36 5.88 4 8.28 4 11c0 4.42 3.58 8 8 8s8-3.58 8-8c0-2.72-1.36-5.12-3.44-6.56zM15 24h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsRemote;
impl IconShape for MdSettingsRemote {
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
                d: "M15 9H9c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h6c.55 0 1-.45 1-1V10c0-.55-.45-1-1-1zm-3 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM7.05 6.05l1.41 1.41C9.37 6.56 10.62 6 12 6s2.63.56 3.54 1.46l1.41-1.41C15.68 4.78 13.93 4 12 4s-3.68.78-4.95 2.05zM12 0C8.96 0 6.21 1.23 4.22 3.22l1.41 1.41C7.26 3.01 9.51 2 12 2s4.74 1.01 6.36 2.64l1.41-1.41C17.79 1.23 15.04 0 12 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx7jiEvDX/NdD/Nc6DnwU8N7Ag3lOtwLfDXwNsMt/DcR/nZcGfgp4MC/crcDbAH/Nfz7Ef42XBn4LOM6LZhd4GeBW/nMh/mv8FfDS/Ov8NvA6/OdC/Od7b+C7+Ld5H+C7+c+D+M/308Bb8W/zM8Bb858H8Z/v6cCD+be5FXgI/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/Ni8NHOM5/Q7Pn/n3Ec/fa/GcLgF/zb8O4t/ms4HP4jmJ58/82z0DeDDPn3lOnwN8Nv86iH+bzwY+i+cknr/fBl6Lf5ufAd6a5888p88BPpt/HcS/zUcDX8Vzehngr3leHw18Ff82HwN8Nc/rtYHf4jm9D/Dd/Osg/m1eGvgrntPHAF/N8zoO3Aoc41/nEvBgYJfn9dHAV/GcXgb4a/51EP92twIP4tn+GngZnr/3Br6Lf523AX6a5++vgJfm2Z4BPJh/PcS/3VcDH8Vzeh3gt3n+3hv4Ll407wN8N8/fawO/xXP6GuCj+ddD/Nu9NvBbPKe/Bl6GF+y1ge8GHsTz9zfARwO/zQv2V8BL85xeBvhr/vUQ/z4/DbwVz+ljgK/mhXtp4K2BB3PFrcBPA3/NC/fRwFfxnL4HeG/+bRD/Pg8Gns7zeh/gu/mP9d7Ad/G8HgLcyr8N4t/vs4HP4jntAu8D/DT/Md4a+C7gOM/pc4DP5t8O8R/jt4HX4nl9NvA5/Pt8FfDRPK/fAV6bfx/Ef4zjwG8DL8XzuhX4bOB7+Nd5L+CzgQfzvP4GeG1gl38fxH+c48BvAy/F83cr8N3AzwB/zfP30sBbAe8NPJjn72+A1wZ2+fdD/Mc6Dvw08Fr8y24FbuWKBwMP5l/2O8BbA7v8x0D85/ho4LOBY/zHuAR8NvDV/MdC/Od5MPDZwHvx7/M9wGcDt/IfD/Gf78HARwNvDTyIF80zgO8Gvhu4lf88iP9aDwZeG/hs4EE8p2cAnw38NnAr/zUQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89Xho4znPaBf6a/1qI/98Q/78h/n9D/P+G+I9zHHgpntfv8LxeGjjGc7oE/DXP67V4Xn8D7PLvh/iP89rAb/G8xPP6beC1eE6/A7w2z8s8r9cBfpt/P8R/nNcGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Nv8+yH+47w28Fs8L/G8fht4LZ7T7wCvzfMyz+t1gN/m3w/xH+e1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2/36I/zivDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3+ffjHwEfDbZBzqe4KQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsVoice;
impl IconShape for MdSettingsVoice {
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
                d: "M7 24h2v-2H7v2zm5-11c1.66 0 2.99-1.34 2.99-3L15 4c0-1.66-1.34-3-3-3S9 2.34 9 4v6c0 1.66 1.34 3 3 3zm-1 11h2v-2h-2v2zm4 0h2v-2h-2v2zm4-14h-1.7c0 3-2.54 5.1-5.3 5.1S6.7 13 6.7 10H5c0 3.41 2.72 6.23 6 6.72V20h2v-3.28c3.28-.49 6-3.31 6-6.72z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+V4aeCvgpYHjvGh2gb8Gfgb4a/7zIP5zfRXw0fz7fDXwMfznQPzn+Wrgo/iP8TXAR/MfD/Gf46WBv+I/1ssAf81/LMR/js8GPov/WJ8DfDb/sRD/OX4aeCue0yXgr3nRvDRwjOf0M8Bb8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/Ef43WAzwZei/9evwO8Ns8J8fz9NvBa/McQV7w38NnAg/jv8TvAa/OcEM/fbwOvxX8M8ZzeG/hs4EH81/od4LV5Tojn77eB1+I/hnhex4GPBj4aOMZ/jd8BXpvnhHj+fht4Lf5jiBfsOPDRwEcDx/jP9TvAa/OcEM/fbwOvxX8M8S97MPDZwHvxn+d3gNfmOSGev98GXov/GOJF92Dgs4H34j/e7wCvzXNCPH+/DbwW/zHEv95rA58NvBb/cX4HeG2eE+L5+23gtfiPIf7tXhv4bOC1+Pf7HeC1eU6I5++3gdfiP4b493tv4LOBB/Fv9zvAa/OcEM/fbwOvxX8M8R/no4Gv4t/md4DX5jkhnr/fBl6L/xjiP8ZbA98FHOff5neA1+Y5IZ6/3wZei/8Y4t/nOPBdwFvz7/M7wGvznBDP328Dr8V/DPFv99bAdwHH+ff7HeC1eU6I5++3gdfiP4b41zsOfBfw1vzH+R3gtXlOiOfvt4HX4j+G+Nd5a+C7gOP8x/od4LV5Tojn77eB1+I/hnjRHAe+C3hr/nP8DvDaPCfE8/fbwGvxH0P8y94a+C7gOP95fgd4bZ4T4vn7beC1+I8hXrDjwHcBb81/vt8BXpvnhHj+fht4Lf5jiOfvrYHvAo7zX+N3gNfmOSGev98GXov/GOI5HQe+C3hr/mv9DvDaPCfE8/fbwGvxH0M821sD3wUc57/e7wCvzXNCPH+/DbwW/zEEHAe+C3hr/vv8DvDaPCfE8/fbwGvxH+NtgO8CjvPf63eA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+fts4LP4v+VzgM/mOSGev7cGfor/W94G+GmeE+IF+2vgpfi/4W+Al+Z5IV6wlwZ+GngQ/7s9A3ht4FaeF+KFOw58NPDewIP43+UZwHcDXw3s8vwh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y88K49B1/vw4QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShop;
impl IconShape for MdShop {
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
                d: "M16 6V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H2v13c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6h-6zm-6-2h4v2h-4V4zM9 18V9l7.5 4L9 18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z4OBB/Gv9zv8x0H81zsO/BTw2vzb7AJfDXwO/36I/1rHgacDx/n3+27gffj3QfzX+i3gtfmP8z7Ad/Nvh/iv82Dg6fzH+mvgZfi3Q/zXeW3gt/iPJ/7tEP91Xhv4LZ6XeNG8NvBbPC/xb4f4r/PawG/xvMSL5rWB3+J5iX87xH+d1wZ+i+clXjSvDfwWz0v82yH+67w28Fs8L/GieW3gt3he4t8O8V/ntYHf4nmJF81rA7/F8xL/doh/m68GXop/nePAS/O8fpsXzXHgpXlev82/3scAfw0g/m1+G3gt/vd6HeC3AcS/zW8Dr8X/Xq8D/DaA+Lf5beC1+N/rdYDfBhD/Nr8NvBb/e70O8NsA4t/mt4HX4n+v1wF+G0D82/w28Fr87/U6wG8DiH+b3wZei/+9Xgf4bQDxb/PbwGvxH+N7gPfiv9brAL8NIP5tfht4Lf5jCHgw8N3Aa/Ff43WA3wYQ/za/DbwW/zHEs7028NnAa/Gf63WA3wYQz/bawG/xvMTz+m3gtfiPIZ7XewOfDTyI/xyvA/w2gHi21wZ+i+clntdvA6/Ffwzxgn008NnAMf5jvQ7w2wDi2V4b+C2el3hevw28Fv8xxAt3HPho4KOBY/zHeB3gtwHEs7028Fs8L/G8fht4Lf5jiBfNg4HPBt6Lf7/XAX4bQDzbawO/xfMSz+u3gdfiP4b413kw8N3Aa/Fv9zrAbwOIZ3tt4Ld4XuJ5/TbwWvzHEP82vwW8Nv82rwP8NoB4ttcGfovnJZ7XbwOvxX8M8a/zYOCngJfm3+51gN8GEM/22sBv8bzE8/pt4LX4jyFedO8NfBVwnH+f1wF+G0A822sDv8XzEs/rt4HX4j+G+JcdB74KeG/+Y7wO8NsA4tleG/gtnpd4Xr8NvBb/McQL99LATwEP5j/O6wC/DSCe7bWB3+J5ief128Br8R9DvGAfBXw1//FeB/htAPFsrw38Fs9LPK/fBl6L/xjieR0Hfgp4bf5zvA7w2wDi2V4b+C2el3hevw28Fv8xxHN6beCngOP853kd4LcBxLO9NvBbPC/xvH4beC3+Y4hn+yrgo/nP9zrAbwOIZ3tt4Ld4XuJ5/TbwWvzHEPBg4KeAl+a/xusAvw0gnu21gd/ieYnn9dvAa/Ef432ArwKO81/ndYDfBhDP9trAb/G8xPP6beC1+N/rdYDfBhDP9trAb/G8xPP6beC1+N/rdYDfBhDP9trAb/G8xPP6beC1+N/rdYDfBhDP9trAb/G8xPP6bOCz+N/rdYDfBhDP9trAb/G8xPN6a+Cn+N/pEnCcKxDP9trAb/G8xPP318BL8b/P5wCfzRWIZ3tt4Ld4XuL5e2ngp4EH8b/H9wDvzbMhnu21gd/ieYkX7Djw0cB7Aw/if67fAb4b+G6eE+LZXhv4LZ6X+L8L8WyvDfwWz0v834V4ttcGfovnJf7vQjzbawO/xfMS/3chnu21gd/ieYn/uxDP9trAb/G8fpv/HX4H+Gz+dRDP9tLAX/G/1+cAn82/DuI53Qo8iP+dXgb4a/51EM/prYGf4n+frwE+mn89xPN6b+CrgWP87/A1wEfzb4N4/o4D7w28NPBg/ufZBf4a+Gngr/m3Q/z/hvj/DfH/G+L/N8T/b/wjhuCdQf0M9+kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShopTwo;
impl IconShape for MdShopTwo {
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
                d: "M3 9H1v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2H3V9zm15-4V3c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H5v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2V5h-5zm-6-2h4v2h-4V3zm0 12V8l5.5 3-5.5 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9Y4DbwU8GHgwV9wK3Ar8DLDLfx3Ef50HA58FvDcv3HcDnwPcyn8+xH+N9wa+i3+d9wG+m/9ciP983wW8N/823w28D/95EP+53hv4Lv593gf4bv5zIP7zPBh4Oi/YM4BbueLBwIN4wR4C3Mp/PMR/nu8G3ovndQl4b+CneU5vDXw3cIzn9T3Ae/MfD/Gf4zhwked1CXgwsMvzdxy4FTjG8zoB7PIfC/Gf472B7+J5vQ3w07xwbw38FM/rbYCf5j8W4j/HZwOfxXN6BvBgXjS3Ag/iOX0O8Nn8x0L85/hu4L14Tr8DvDYvmt8GXovn9DXAR/MfC/Fv817ARwMvzf8Mfw18NfA9/Osg/vW+C3hv/mf6buB9eNEh/nU+G/gs/mf7HOCzedEg/nUuAsf5n20XOMGLBvGie23gt/jf4XWA3+ZfhnjRvTbwW7xoPgb4a57TSwNfxYvmY4C/5jm9NPBVvGheB/ht/mWIF91rA7/Fi+Z1gN/mOb028Fu8aF4H+G2e02sDv8WL5nWA3+ZfhnjRvTbwW7xoXgf4bZ7TawO/xYvmdYDf5jm9NvBbvGheB/ht/mWIF91rA7/Fi+Z1gN/mOb028Fu8aF4H+G2e02sDv8WL5nWA3+ZfhnjRvTbwW7xoXgf4bZ7TawO/xYvmdYDf5jm9NvBbvGheB/ht/mWIF91rA7/Fi+Z1gN/mOb018FO8aF4H+G2e01sDP8WL5nWA3+ZfhnjRvTbwW7xovgb4aJ7TVwMfxYvma4CP5jl9NfBRvGheB/ht/mWIF91rA7/Fi2YXeB3gr7nipYHfAo7zotkFXgf4a654aeC3gOO8aF4H+G3+ZYgX3WsDv8W/zm9zxWvzb/PbXPHa/Ou8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7qXBv6K/x1eBvhr/mWIf51bgQfxP9szgAfzokH867w28Fv8z/Y6wG/zokH867038NXAMf5nuQS8N/DTvOgQ/zbHgfcGXhp4MP+9bgX+GvhuYJd/HcT/b4j/3xD/vyH+f0P8/8Y/AgRkpUFbyFa4AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShoppingBag;
impl IconShape for MdShoppingBag {
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
                d: "M18,6h-2c0-2.21-1.79-4-4-4S8,3.79,8,6H6C4.9,6,4,6.9,4,8v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8C20,6.9,19.1,6,18,6z M10,10c0,0.55-0.45,1-1,1s-1-0.45-1-1V8h2V10z M12,4c1.1,0,2,0.9,2,2h-4C10,4.9,10.9,4,12,4z M16,10c0,0.55-0.45,1-1,1 s-1-0.45-1-1V8h2V10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fR4MvBdXfA9wK//1EP89Xhr4LeA4V+wCrwP8Nf+1EP89/gp4aZ7TXwMvw38txH+9zwY+i+fvc4DP5r8O4r/WSwN/xQv3MsBf818D8V/rr4CX5oW7FXgZYJf/fIj/Op8NfBYvmq8BPpr/fIj/Gq8N/BbP62+44qV4Xq8D/Db/uRD/+Y4DfwU8mOf1MlzxVzyvW4GXAXb5z4P4z/fVwEfxvD4H+Gyu+Gzgs3heXwN8NP95EP+5Xhv4LZ7X3wAvzXP6a+CleF6vA/w2/zkQ/3mOA38FPJjndAl4aeBWntODgb8GjvGcbgVeBtjlPx7iP89XAx/F8/oY4Kt5/j4a+Cqe19cAH81/PMR/jtcGfovn9TvAa/PC/TbwWjyv1wF+m/9YiP94x4G/Ah7Mc7oEvDRwKy/cg4G/Bo7xnG4FXgbY5T8O4j/eVwMfxfP6GOCredF8NPBVPK+vAT6a/ziI/1ivDfwWz+tngLfmX+engbfieb0O8Nv8x0D8xzkO/BXwYJ7TJeDBwC7/OseBW4FjPKdbgZcBdvn3Q/zH+Wrgo3hebwP8NP82bw38FM/ra4CP5t8P8ZxeGvgu4KX5v+mvgfcB/porEM/2YOCvgOP837YLPATYBRDP9trAb/H/w+sAvw0gnu2lgb/i/4eXAf4aQDynXeAY/7ddAo5zBeI5fTfwXvzf9jPAW3MF4jm9N/Bd/N/2PsB3cwXiOT0YeDr/tz0EuJUrEM/rr4GX4v+mZwAP5tkQz+urgY/i/6avAT6aZ0M8r7cGfor/fM8AbuWKBwMP4j/f2wA/zbMhnj/zn+cS8N7AT/Oc3hr4buAY/3lOALs8G+L5+23gtfiPdwl4MLDL83ccuBU4xn+83wFem+eEeP4+Gvgq/uO9DfDTvHBvDfwU//E+B/hsnhPi+Xtp4K/4j/UM4MG8aG4FHsR/rJcB/prnhHjBdoFj/Mf5HeC1edH8NvBa/Me5BBzneSFesO8G3ov/OL8DvDYvmt8GXov/ON8DvDfPC/GCvTfwXfzHuRV4CC+apwMP5j/O+wDfzfNCvGAPBp7Of6y3AX6aF+6tgZ/iP9ZDgFt5XogX7q+Bl+I/zi7wEGCX5++lgd8CjvMf52+Al+b5Q7xwXw18FP+xdoH3AX6a5/TWwHcBx/mP9TXAR/P8IV641wZ+i/8ctwK3csWDgQfzn+NtgJ/m+UP8y8z/buIFQ/zLfhp4K/53+h3gtXnBEP+yjwa+iv+dPgb4al4wxL/spYG/4n+nlwH+mhcM8aK5FXgQ/7tcAo7zwiFeNN8NvBf/u3wP8N68cIgXzWsDv8X/Lq8D/DYvHOJF99fAS/G/w98AL82/DPGiOw78NvBS/M/2N8BrA7v8yxD/OseBjwZeG3gt/mf5HeC3ga8GdnnRIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPER6lQW9x0swAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShoppingBasket;
impl IconShape for MdShoppingBasket {
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
                d: "M17.21 9l-4.38-6.56c-.19-.28-.51-.42-.83-.42-.32 0-.64.14-.83.43L6.79 9H2c-.55 0-1 .45-1 1 0 .09.01.18.04.27l2.54 9.27c.23.84 1 1.46 1.92 1.46h13c.92 0 1.69-.62 1.93-1.46l2.54-9.27L23 10c0-.55-.45-1-1-1h-4.79zM9 9l3-4.4L15 9H9zm3 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP9tLAV/HC/Qzw1fzfgXi21wZ+ixfuVuAh/N+BeLbXBn6Lf9n7AN/N/w2IZ3tt4Lf4l90KPIT/GxDP9trAb/GieR/gu/nfD/FsLw18Nc/rtXhefw18NP+z/Q2wywuH+Jd9N/Be/O+zC7wO8Ne8YIh/2YOBp/O/018DL8MLhnjRfDfwXvzvJF4wxIvmwcDT+d/nZ4C35gVDvOh+G3gt/nd5GeCvecEQL7rXBn6L/z2+B3hvXjjEv85vA6/F/w4PAW7lhUP867w28Fv8z/c5wGfzL0P86/018FL8z3UJeDCwy78M8a/33sB38T/XxwBfzYsG8W9zK/Ag/ud5BvBgXnSIf5v3Br6L/3neBvhpXnSIf7tbgQfxP8fvAK/Nvw7i3+6jga/if47XAX6bfx3Ev91x4FbgGP/9fgZ4a/71EP8+nw18Fv/9HgLcyr8e4t/nOHArcIz/Pl8DfDT/Noh/v88GPov/HpeABwO7/Nsg/v0eDDyd/x6fA3w2/3aI/xjfDbwX/7WeATyYfx/Ef4zfAl6b/1rvA3w3/z6If7/XBn6L/1o/A7w1/36If7+nAw/mv87PAO8N7PLvh/j3eW/gu3hezwBu5T/OLvDXwE8Df81/HMS/3XHgr4AH87weAtzK/3yIf7vPBj6L5/U9wHvzvwPi3+Y48HTgOM/pEvBgYJf/HRD/Nh8NfBXP63OAz+Z/D8S/zVcDH8Xzehvgp/nfA/Fv893Ae/E/y+8Ar82/DuLf5qOBr+J/lt8BXpt/HcS/zXHgVuAY/3P8DvDa/Osg/u3eG/gu/uf4HeC1+ddB/Pu8NvDdwIP47/c7wGvzr4P4j/HSwHH+e+0Cf82/DuL/N8T/b4h/n+PARwHvDTwY2AV+Gvgc4Fb+Yz0Y+CzgrYHjwK3AdwNfA+zyb4P4t3sw8FPAS/P8vQ/w3fzHeG/gu3j+/hp4G+BW/vUQ/3Z/Bbw0L9xDgFv593kw8HReuL8GXoZ/PcS/zVsDP8W/7HuA9+bf57uB9+Jf9jbAT/Ovg/i3+Wzgs/iX7QIn+Pe5CBznX/Y5wGfzr4P4t/lq4KN40Yh/H/Oi+RngrfnXQfzbfDbwWfzLngE8mH+fvwZein/Z5wCfzb8O4t/mwcDT+Zd9DvDZ/Pt8NvBZ/MseAtzKvw7i3+6rgY/iBfsb4LWBXf59jgO/DbwUL9jXAB/Nvx7i3+ergY/ief0N8NbArfzHeDDw08BL8by+Bvho/m0Q/34PBt4beGngr4G/Bn6a/xxvDbw08NLAXwPfDdzKvx3i/zfE/2+I/98Q/78h/n/jHwH8C5BB8aANkQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShoppingCart;
impl IconShape for MdShoppingCart {
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
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H5.21l-.94-2H1zm16 16c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4kXz0sAx/ne5BPw1LxzihXtt4LuAB/O/063A+wC/zfOHeMHeG/gu/m94H+C7eV6I5+848HTgOP837AIPAXZ5Tojn76OBr+L/lo8BvprnhHj+vht4L/5v+R7gvXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Mx4FdXrDfBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnvdxz4buCtecF+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+a/32cDnwU8BLiV5++3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf77XQSOA98DvDfP328Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/Pd5MPBVwFvzbO8N/Aywy3P6beC1eE6/A7w2zwnx/P028Fo8p98BXpv/eq8NvBfw3jx/u8BXA98D3MoVvw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzX+ejga8GXht4b+C9eP4uAV8NfDdwK1f8NvBaPKffAV6b54R4/n4beC2e0+8Ar81/jePA04GPAb6bKx4MfDXwVjzb+wA/DezynH4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5r/GZwOfBfw28Do8p13gGPA9wHvz/P028Fo8p98BXpvnhHj+fht4LZ7T7wCvzX+NpwMP5orXAX6bZ/ts4LOAhwC38vz9NvBaPKffAV6b54R4/n4beC2e0+8Ar81/vvcGvotn+xngrXm248B3A2/NC/bbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv/5ng48mOf0EOBWnu04sMsL9tvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/3mOAx8FfDbP67eB9wFu5UXz28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/8R4MfBTw3sBxXrifBr4G+G1euN8GXovn9DvAa/OcEM/fdwPvxX+s3wFem2d7aeCjgPfmX++vga8Gvodne23gt3jBvgb4aJ4T4vn7aOCr+I/1O8Brc8VLAz8NPIh/u0vAZwNfzRWvDfwWL9jHAF/Nc0I8f8eBW4Fj/Mf5HeC1eU5vDXw08Fq86J4BfDfw1cAuz/bawG/x/F0CHgzs8pwQL9h7A9/Ff5zfAV6b5++1gfcG3osX7G+Arwa+m+fvtYHf4vl7G+CneV6IF+61ge8GHsS/3+8Ar80L99LAX/G8Pgf4bF641wZ+i+f0DOC9gd/m+UO8aF4aOM6/zy7w1/zLvht4L57tEvBgYJcX7jjw0jzbLvDXvHCI/3leG/gtnu1rgI/mPwfif6bfBl6LKx4C3Mp/DsT/TG8N/BTwPcB7858H8T/XrcBbA3/Nfx7E/1wvDfw1/7kQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IUjSuQVgbl2AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSmartButton;
impl IconShape for MdSmartButton {
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
                d: "M22,9v6c0,1.1-0.9,2-2,2h-1l0-2h1V9H4v6h6v2H4c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h16C21.1,7,22,7.9,22,9z M14.5,19 l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38L19,12l-1.38-0.62L17,10l-0.62,1.38 L15,12l1.38,0.62L17,14z M14.5,19l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38 L19,12l-1.38-0.62L17,10l-0.62,1.38L15,12l1.38,0.62L17,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxYvmd/jv9Vq8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/Fi+Z1+I/31cBL8R/rd4DX5jkhnr/fBl6LF434j/fbwGvxH+t3gNfmOSGev98GXosXjfiP99vAa/Ef63eA1+Y5IZ6/3wZeixeN+I/328Br8R/rd4DX5jkhnr/fBl6LF434j/fbwGvxH+t3gNfmOSGev98GXosXzWvzH++rgZfmP9bvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fdwHvxf8vXAB/Nc0I8fx8NfBX/t3wM8NU8J8Tzdxy4FTjG/w2XgAcDuzwnxAv23sB38X/D2wA/zfNCvHCvDXw38CD+d3oG8N7Ab/P8IV40Lw0c53+XXeCveeEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BQF5oQcN+HfIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSource;
impl IconShape for MdSource {
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
                d: "M20,6h-8l-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M14,16H6v-2h8V16z M18,12H6v-2h12V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4LV404nn9NvBaPKffAV6b/3jmRfM7wGvznBDP328Dr8WLRjyv3wZei+f0O8Br8x/PvGh+B3htnhPi+ftt4LV40Yjn9dvAa/Gcfgd4bf7jmRfN7wCvzXNCPH+/DbwWLxrxvH4beC2e0+8Ar81/PPOi+R3gtXlOiOfvt4HX4kUjntdvA6/Fc/od4LX5j2deNL8DvDbPCfH8/TbwWrxofpvn9dLAcZ7TLvDX/Md7bV40vwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fq8aMTz+m3gtXhOvwO8Nv/xzIvmd4DX5jkhnr/fBl6LF414Xr8NvBbP6XeA1+Y/nnnR/A7w2jwnxPP328Br8aIRz+u3gdfiOf0O8Nr8xzMvmt8BXpvnhHj+fht4LV404nn9NvBaPKffAV6b/3jmRfM7wGvznBDP328Dr8WLRjyv3wZei+f0O8Br8x/PvGh+B3htnhPi+ftt4LV40fw2z+ulgeM8p13gr/mP99q8aH4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1eNGI5/XbwGvxnH4HeG3+9cx/jN8BXpvnhHj+fht4LV404nn9NvBaPKffAV6bfz3zH+N3gNfmOSGev98GXosXjXhevw28Fs/pd4DX5l/P/Mf4HeC1eU6I5++3gdfiRSOe128Dr8Vz+h3gtfnXM/8xfgd4bZ4T4vn7beC1eNGI5/XbwGvxnH4HeG3+9cx/jN8BXpvnhHj+fht4LV40v83zemngOM9pF/hr/vVem/8YvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yNsBP85wQz99vA6/F/x1/A7w0zwvx/P028Fr83/A3wFsDt/K8EM/fbwOvxf9uzwC+G/hqYJfnD/H8/TbwWvzL3gf4bv73Qjx/vw28Fi/c+wDfzf9uiOfvt4HX4gV7H+C7+d8P8fz9NvBaPH/vA3w3/zcgnr/fBl6L5/U+wHfzfwfi+ftt4LV4Tu8DfDf/tyCev98GXotnex/gu/m/B/H8/TbwWlzxPsB3838T4vn7beC1gPcBvpv/uxDP328D3w18N/+3IZ6/lwb+mv/7EP+/If5/Q/z/hvj/DfH/G/8IMKumQbHNe0AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeakerNotes;
impl IconShape for MdSpeakerNotes {
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM8 14H6v-2h2v2zm0-3H6V9h2v2zm0-3H6V6h2v2zm7 6h-5v-2h5v2zm3-3h-8V9h8v2zm0-3h-8V6h8v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeakerNotesOff;
impl IconShape for MdSpeakerNotesOff {
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
                d: "M10.54 11l-.54-.54L7.54 8 6 6.46 2.38 2.84 1.27 1.73 0 3l2.01 2.01L2 22l4-4h9l5.73 5.73L22 22.46 17.54 18l-7-7zM8 14H6v-2h2v2zm-2-3V9l2 2H6zm14-9H4.08L10 7.92V6h8v2h-7.92l1 1H18v2h-4.92l6.99 6.99C21.14 17.95 22 17.08 22 16V4c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpellcheck;
impl IconShape for MdSpellcheck {
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
                d: "M12.45 16h2.09L9.43 3H7.57L2.46 16h2.09l1.12-3h5.64l1.14 3zm-6.02-5L8.5 5.48 10.57 11H6.43zm15.16.59l-8.09 8.09L9.83 16l-1.41 1.41 5.09 5.09L23 13l-1.41-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vY5zxS7/PRD/vd6bK76b/x6I/17fzRXvzX8PxH+vi1xxgv8eiP8+bw38FFe8DfDT/NdD/Pf5auCjuOJrgI/mvx7iv8/TgQdzxa3AQ/ivh/jv8dLAX/GcXgb4a/5rIf57fDTwVTynjwG+mv9aiP8efwW8NM/pr4GX4b8W4r/eg4Gn8/ydAHb5r4P4r/fewHfx/L0P8N3810H81/tp4K14/n4GeGv+6yD+65kXTvzXQfzXemvgp3jh3gb4af5rIP5rfTfwXrxw3wO8N/81EP+1LgLHeeFuBR7Cfw3Ef52XBv6KF83LAH/Nfz7Ef52vBj6KF83XAB/Nfz7Ef52/Al6aF81fAy/Dfz7Ef40HA0/nX+chwK3850I8f8eBl+I/zlsDH82/zlcDP81/nL8BdnlOiBfss4HP4v+GzwE+m+eFeOFeGvhp4EH87/QM4K2Bv+b5Q/zLjgNfDbwX/7t8D/DRwC4vGOJF99bAdwPH+J/tEvDewE/zL0P86zwY+G7gtfif6XeA9wZu5UWD+Lf5bOCz+J/lc4DP5l8H8W/30sBPAw/iv9czgLcG/pp/PcS/z3Hgq4H34r/HzwDvDezyb4P4j/HWwHcDx/ivcQn4aOC7+fdB/Md5MPDdwGvxn+tvgLcGbuXfD/Ef77OBz+I/x+cAn81/HMR/jrcGfor/WO8DfDf/sRD/Ob4a+Cj+Y30N8NH8x0L85/gr4KX5j3Ur8BD+YyH+4z0YeDr/OR4C3Mp/HMR/vI8Gvor/HB8DfDX/cRD/8f4KeGn+c/w18DL8x0H8xzoOXOQ/10OAW/mPgfiP9d7Ad/Gf632A7+Y/BuI/1k8Db8WL7mu44qN40f0M8Nb8x0D8xzIvmkvAewM/zRVvDXw3cIwXzQlgl38/xH+ctwZ+in/Z3wBvDdzKc3ow8NPAS/Evexvgp/n3Q/zH+W7gvXjhvgb4aF64rwY+ihfue4D35t8P8R/nInCc5+8S8N7AT/OieWvgu4FjPH+7wAn+/RD/MV4a+Cuev78B3hv4a/51Xhr4buCleP5eB/ht/n0Q/zG+Gvgontf3AB8N7PJvcxz4auC9eF5fA3w0/z6I/xhPBx7Ms10CPhr4bv5jvDfw1cAxnu1W4CH8+yD+/V4a+Cue7W+A9wb+mv9YLw18N/BSPNvLAH/Nvx3i3++jga/iiu8BPhrY5T/HceCrgffiis8BPpt/O8S/318BLw18DPDV/Nd4b+C7gL8GXoZ/O8S/z4OB3wbeGvhr/mu9NPDTwGsDt/Jvg/j3eTCwC+zy3+M48GDgr/m3Qfz/hvj/DfH/G+L/N8T/b/wjH0CRQZRbjh4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStarRate;
impl IconShape for MdStarRate {
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
                points: "14.43,10 12,2 9.57,10 2,10 8.18,14.41 5.83,22 12,17.31 18.18,22 15.83,14.41 22,10",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStars;
impl IconShape for MdStars {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zm4.24 16L12 15.45 7.77 18l1.12-4.81-3.73-3.23 4.92-.42L12 5l1.92 4.53 4.92.42-3.73 3.23L16.23 18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FnAawPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZF8zfAR/Nf46uBl+JFI14wxL/MvGh+B3ht/mv8NvBavGjEC4b4l5kXze8Ar81/jd8GXosXjXjBEP8y86L5HeC1+a/x28Br8aIRLxjiX2ZeNL8DvDb/NX4beC1eNOIFQ/zLzIvmr4GP5r/GVwMvzYtGvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZedH8DvDa/OuZ/1ziBUP8y8yL5neA1+Zfz/znEi8Y4l9mXjS/A7w2/3rmP5d4wRD/MvOi+R3gtfnXM/+5xAuG+JeZF83vAK/Nv575zyVeMMS/zLxodoG/5l/vtXleHwP8Nf96Lw18Fc9JvGCIf5n5r/c6wG/zr/PSwG8Bx3lO4gVD/MvMf73XAX6bF91LA78FHOd5iRcM8S8z//VeB/htXjQvDfwWcJzn9T3Ae/OCIf5l5r/e6wC/zb/spYHfAo7zvL4HeG9eOMS/zPzXex3gt3nhXhr4LeA4z+t7gPfmX4b4l5n/eq8D/DYv2EsDvwUc53l9D/DevGgQ/zLzX+91gN/m+Xtp4LeA4zyv7wHemxcd4l9m/uu9DvDbPK+XBn4LOM7z+h7gvfnXQfzLzH+91wF+m+f00sBvAcd5Xt8DvDf/eoh/mfmv9zrAb/NsLw38FnCc5/U9wHvzb4P4l5n/eq8D/DZXvDTwW8Bxntf3AO/Nvx3iX2b+670O8NvASwO/BRzneX0P8N78+yD+Zea/3usAu8BvAcd5Xt8DvDf/foh/2S5wjP9aHwN8FnCc5/U9wHvzHwPxL/tu4L34n+F7gPfmPw7iX/Zg4On89/se4L35j4V40bw38F389/ke4L35j4d40T0Y+GzgrYFj/Nf5HuC9+c+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CSgd8QbBZyRoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStickyNote2;
impl IconShape for MdStickyNote2 {
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
                d: "M19,3H4.99C3.89,3,3,3.9,3,5l0.01,14c0,1.1,0.89,2,1.99,2h10l6-6V5C21,3.9,20.1,3,19,3z M7,8h10v2H7V8z M12,14H7v-2h5V14z M14,19.5V14h5.5L14,19.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD7ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcceCn+d/gbYJd/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEP+/If5/Q7zoHgw8iP8d/gbY5V+GeNF9NvBZ/O/wOcBn8y9DvOh+G3gt/nf4GeCt+ZchXnQXgeP873Ar8BD+ZYgXzYOBp/O/ywlglxcO8aJ5a+Cn+N/ldYDf5oVDvGg+G/gs/nf5HOCzeeEQL5rfBl6L/11+BnhrXjjEi+YicJz/XW4FHsILh/iXPRh4Ov87nQB2ecEQ/7K3Bn6K/51eB/htXjDEv+yzgc/if6fPAT6bFwzxL/tt4LX43+lngLfmBUP8yy4Cx/nf6VbgIbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/2WtzxUsDX8Xzeh3+fX6L5/UxwF/zovktntfHAH/NFb/NC4Z40b028Fs8L/HvY57X6wC/zYvGPK/XAX6bfxniRffawG/xvMS/j3lerwP8Ni8a87xeB/ht/mWIF91rA7/F8xL/PuZ5vQ7w27xozPN6HeC3+ZchXnSvDfwWz0v8+5jn9TrAb/OiMc/rdYDf5l+GeNG9NvBbPC/x72Oe1+sAv82Lxjyv1wF+m38Z4kX32sBv8bzEv495Xq8D/DYvGvO8Xgf4bf5liBfdawO/xfMS/z7meb0O8Nu8aMzzeh3gt/mXIV50rw38Fs9L/PuY5/U6wG/zojHP63WA3+ZfhnjRvTbwWzwv8e9jntfrAL/Ni8Y8r9cBfpt/GeJF99rAb/G8xL+PeV6vA/w2LxrzvF4H+G3+ZYgX3WsDv8V/jdcBfpsXjXlerwP8Nv8yxIvutYHf4r/G6wC/zYvGPK/XAX6bfxniRffawG/xX+N1gN/mRWOe1+sAv82/DPGie23gt/iv8TrAb/OiMc/rdYDf5l+GeNG9NvBb/Nd4HeC3edGY5/U6wG/zL0O86I4DL81/jb8GdnnRvDbP66+BXf5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPX6V9QfUObC8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStore;
impl IconShape for MdStore {
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
                d: "M20 4H4v2h16V4zm1 10v-2l-1-5H4l-1 5v2h1v6h10v-6h4v6h2v-6h1zm-9 4H6v-4h6v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACgklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6K/x0+Bvhr/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b008NX87/DRwF/zL0P8/4b4/w3x/xvi/zfE/2+IF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86I4DL83/Dn8N7PIvQ/z/hvj/DfH/G+L/N8SL7jjwUvzv8DfALv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+Zch/n9D/P+G+P8N8f8b4v83xIvupYGv4r/GxwB/zX8+xIvutYHf4r/G6wC/zX8+xIvutYHf4r/G6wC/zX8+xIvutYHf4r/G6wC/zX8+xIvutYHf4r/G6wC/zX8+xIvupYGv5r/GRwN/zX8+xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AtWbMEFrV8QOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubject;
impl IconShape for MdSubject {
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
                d: "M14 17H4v2h10v-2zm6-8H4v2h16V9zM4 15h16v-2H4v2zM4 5v2h16V5H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubtitlesOff;
impl IconShape for MdSubtitlesOff {
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
                d: "M20,4H6.83l8,8H20v2h-3.17l4.93,4.93C21.91,18.65,22,18.34,22,18V6C22,4.9,21.1,4,20,4z",
            }
            path {
                d: "M1.04,3.87l1.2,1.2C2.09,5.35,2,5.66,2,6v12c0,1.1,0.9,2,2,2h13.17l2.96,2.96l1.41-1.41L2.45,2.45L1.04,3.87z M8,12v2H4 v-2H8z M14,16.83V18H4v-2h9.17L14,16.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSupervisedUserCircle;
impl IconShape for MdSupervisedUserCircle {
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
                d: "M11.99 2c-5.52 0-10 4.48-10 10s4.48 10 10 10 10-4.48 10-10-4.48-10-10-10zm3.61 6.34c1.07 0 1.93.86 1.93 1.93 0 1.07-.86 1.93-1.93 1.93-1.07 0-1.93-.86-1.93-1.93-.01-1.07.86-1.93 1.93-1.93zm-6-1.58c1.3 0 2.36 1.06 2.36 2.36 0 1.3-1.06 2.36-2.36 2.36s-2.36-1.06-2.36-2.36c0-1.31 1.05-2.36 2.36-2.36zm0 9.13v3.75c-2.4-.75-4.3-2.6-5.14-4.96 1.05-1.12 3.67-1.69 5.14-1.69.53 0 1.2.08 1.9.22-1.64.87-1.9 2.02-1.9 2.68zM11.99 20c-.27 0-.53-.01-.79-.04v-4.07c0-1.42 2.94-2.13 4.4-2.13 1.07 0 2.92.39 3.84 1.15-1.17 2.97-4.06 5.09-7.45 5.09z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zleGngr4MFccSvwM8Bf8z8L4j/WawPfBTyY5++vgY8Bfpv/GRD/cd4b+C5eNO8DfDf//RD/Md4b+C7+dd4G+Gn+eyH+/Y4DTweO86+zCzwE2OW/D+Lf76OBr+Lf5mOAr+ZF82DgQVzxDOBW/v0Q/36/DbwW/zY/A7w1L9xbA18FPJjndCvwMcBP82+H+Pcz/3a3Ag/hBfsq4KN54b4a+Bj+bRD/fubfRzx/Hw18FS+a9wG+m389xL/frcCD+Lf5G+CleV7HgacDx3nR7AIPAXb510H8+/008Fb823wP8N48r/cGvot/nfcBvpt/HcS/33sD38W/zesAv83z+mzgs/jX+Rzgs/nXQfzH+GvgpfjX+R3gtXn+fhp4K/51vgb4aP51EP8xXhr4beAYL5pLwEsDt/L8fTbwWfzrfA7w2fzrIP7jvDTw08CDeOH+Bnhv4K95wd4a+Cn+dV4H+G3+dRD/sY4DHw28N/AgntMzgO8GvhrY5V92K/AgXjS/A7w2/3qI/zwvDRznil3gr/nXeWngt4FjvHCXgNcG/pp/PcT/bC8N/DZwjOfvEvDawF/zb4P4n+848N7AewMvxRV/A3w38N3ALv92iP/fEP+/If5zvBbP3+/wPwvi3+c48FrAawMvDbw0cJwXbhf4GeCjgV3+eyH+9Y4D7wW8NfDa/Ot9D/DRwC7//RAvugcDnwW8N/927wN8Ny+6lwYeBLw0V7w0V/w1V/w18Azgr/m3QfzLjgNfBbw3/z7vA3w3L9xx4K2AtwZeGzjOi2YX+G3gp4GfAXZ50SBeuJcGfgs4zr/PzwBvzQv20sBHAe/Nf4zvBr4G+GteOMQL9tLAbwHH+fe5BDwY2OV5HQe+C3hr/nP8NPA+wC7PH+L5Ow78FfBg/v2+B3hvntdLAz8FPJj/XLcCbwP8Nc8L8fy9N/Bd/Md4G+CneU7Hgd8CXpr/Gn8NvA6wy3NCPH9fDXwU/zFOALs8p/cGvov/Wu8DfDfPCfH8/TbwWvzHEM/ru4H34r/W9wDvzXNCPH+/DbwW/zHE8/pt4LX4r/U7wGvznBDP328Dr8V/DPG8fht4Lf5r/Q7w2jwnxPP328Br8R9DPK/fBl6L/1q/A7w2zwnx/P028Fr8xxDP67eB1+K/1u8Ar81zQjx/vw28Fv8xxPP6beC1+K/1O8Br85wQz99XAy/Nf4zX5nl9NfDS/Nf6a+CjeU6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EPr6NQQMzAtgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSupervisorAccount;
impl IconShape for MdSupervisorAccount {
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
                d: "M16.5 12c1.38 0 2.49-1.12 2.49-2.5S17.88 7 16.5 7C15.12 7 14 8.12 14 9.5s1.12 2.5 2.5 2.5zM9 11c1.66 0 2.99-1.34 2.99-3S10.66 5 9 5C7.34 5 6 6.34 6 8s1.34 3 3 3zm7.5 3c-1.83 0-5.5.92-5.5 2.75V19h11v-2.25c0-1.83-3.67-2.75-5.5-2.75zM9 13c-2.33 0-7 1.17-7 3.5V19h7v-2.25c0-.85.33-2.34 2.37-3.47C10.5 13.1 9.66 13 9 13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSupport;
impl IconShape for MdSupport {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M19.46,9.12l-2.78,1.15 c-0.51-1.36-1.58-2.44-2.95-2.94l1.15-2.78C16.98,5.35,18.65,7.02,19.46,9.12z M12,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S13.66,15,12,15z M9.13,4.54l1.17,2.78c-1.38,0.5-2.47,1.59-2.98,2.97L4.54,9.13C5.35,7.02,7.02,5.35,9.13,4.54z M4.54,14.87 l2.78-1.15c0.51,1.38,1.59,2.46,2.97,2.96l-1.17,2.78C7.02,18.65,5.35,16.98,4.54,14.87z M14.88,19.46l-1.15-2.78 c1.37-0.51,2.45-1.59,2.95-2.97l2.78,1.17C18.65,16.98,16.98,18.65,14.88,19.46z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADVUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zu9N/Dd/Psh/nf6beDpwPvw74P43+m3gdcCvht4H/7tEP87/TbwWlzx3cD78G+D+N/pt4HX4tm+G3gf/vUQ/zv9NvBaPKfvBt6Hfx3E/06/DbwWz+u7gffhRYf43+m3gdfi+ftu4H140SD+67w08FX8x3hp4Dgv2HcD78O/DPFf57WB3+K/zucAn80Lh/iv89rAb/Ff632A7+YFQ/zXeW3gt/iv9T3Ae/OCIf7rvDbwW/zX+R7gvXnhEP91Xhr4av5jvDRwjBfse4D35l+G+N/pt4HX4vn7HuC9edEg/nf6beC1eF7fA7w3LzrEf5z3Br6b/xq/DbwWz+l7gPfmXwfxH+O7gIcAr81/jd8GXotn+x7gvfnXQ/z7fRfw3sDvAK/Nf43fBl6LK74HeG/+bRD/Pt8FvDdX/A7w2vzX+G3gtYDvAd6bfzvEv913Ae/Ns/0O8Nr81/ht4Fbgvfn3QfzbfBfw3jyn3wFem/8a7w18N/9+iH+97wLem+f1O8Br878L4l/nu4D35vn7HeC1+d8F8aL7LuC9ecF2gb/mP8bHAH/Nfz7Ei+a7gPfmv87rAL/Nfz7Ev+y7gPfmv9brAL/Nfz7Ev+y7gffiv9brAL/Nfz7Ei+a7gffiv87rAL/Nfz7Ei+67gffiBbsE/DX/MT4a+Gv+8yH+db4beC+ev98BXpv/XRD/et8NvBfP63eA1+Z/F8S/zXcD78Vz+h3gtfnfBfFv993Ae/FsvwO8Nv+7IP59vht4L674HeC1+d8F8e/33cB7Ab8DvDb/uyD+Y3w38GDgtfnfBfEf572B7+Z/F8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLFLlZBOrEs8wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwapHoriz;
impl IconShape for MdSwapHoriz {
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
                d: "M6.99 11L3 15l3.99 4v-3H14v-2H6.99v-3zM21 9l-3.99-4v3H10v2h7.01v3L21 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwapHorizontalCircle;
impl IconShape for MdSwapHorizontalCircle {
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
                d: "M22 12c0-5.52-4.48-10-10-10S2 6.48 2 12s4.48 10 10 10 10-4.48 10-10zm-7-5.5l3.5 3.5-3.5 3.5V11h-4V9h4V6.5zm-6 11L5.5 14 9 10.5V13h4v2H9v2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/VdXPE+/M+E+M/zXcB7c8V3A+/D/zyI/xzfBbw3z+m7gffhfxbEf7zvAt6b5++7gffhfw7Ef6zvAt6bF+67gffhfwbEf5zvAt6bF813A+/Dfz/Ef4zvAt6bf53vBt6H/16If7/vAt6bf5vvBt6H/z6If5/vAt6bf5/vBt6H/x6If7vvAt6b/xjfDbwP//UQ/zbvDbw3L9hLA8d4TpeAv+YF+27gu/mvhfjP8dvAa/Gcfgd4bf5nQfzn+G3gtXhOvwO8Nv+zIP5z/DbwWjyn3wFem/9ZEP85fht4LZ7T7wCvzf8siP8cvw28Fs/pd4DX5n8WxH+O3wZei+f0O8Br8z8L4j/HbwOvxXP6HeC1+Z8F8Z/jt4HX4jn9DvDavGheGjjGc7oE/DX/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jpYHjPKdd4K950fw28Fo8p98BXpv/WIj/mX4beC2e0+8Ar81/LMT/TL8NvBbP6XeA1+Y/FuJ/pt8GXovn9DvAa/MfC/E/028Dr8Vz+h3gtfmPhfif6beB1+I5/Q7w2vzHQvzP9NvAa/Gcfgd4bf5jIf5n+m3gtXhOvwO8Nv+xEP893ht4L16wlwaO85x2gb/mBfse4Lv510H89/lu4L34j/E9wHvzr4f47/XdwHvx7/M9wHvzb4P47/fdwHvxb/M9wHvzb4f4n+G7gffiX+d7gPfm3wfxP8d3A+/Fi+Z7gPfm3w/xP8t3A+/FC/c9wHvzHwPxP893A+/F8/c9wHvzHwfxP9N3A+/Fc/oe4L35j4X4n+u7gffiio8Bvpr/eIj/2b4b+G3gu/nPgfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Ajlhe0FidiQyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwapVert;
impl IconShape for MdSwapVert {
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
                d: "M16 17.01V10h-2v7.01h-3L15 21l4-3.99h-3zM9 3L5 6.99h3V14h2V6.99h3L9 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwapVerticalCircle;
impl IconShape for MdSwapVerticalCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM6.5 9L10 5.5 13.5 9H11v4H9V9H6.5zm11 6L14 18.5 10.5 15H13v-4h2v4h2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwipe;
impl IconShape for MdSwipe {
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
                d: "M19.75 16.25c0 .06-.01.13-.02.2l-.75 5.27c-.11.73-.69 1.28-1.44 1.28h-6.79c-.41 0-.79-.17-1.06-.44l-4.94-4.94.79-.8c.2-.2.48-.33.79-.33.09 0 .16.02.24.03l3.43.72V6.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v6h.76c.19 0 .37.04.54.11l4.54 2.26c.53.22.91.76.91 1.38zm.38-12.38C18.69 2.17 15.6 1 12 1S5.31 2.17 3.87 3.87L2 2v5h5L4.93 4.93c1-1.29 3.7-2.43 7.07-2.43s6.07 1.14 7.07 2.43L17 7h5V2l-1.87 1.87z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif7/jwC7/Noj//f4KeB1gl389xP9+Bv4aeB1gl38dxP9+5oq/Bl4H2OVFh/jfzzzbXwOvA+zyokH872ee018DrwPs8i9D/O9nntdfA68D7PLCIf73M8/fXwOvA+zygiH+Za/F/2y/zQv218DrALs8f4h/mfnf7a+B1wF2eV6If5n53++vgdcBdnlOiH+Z+b/hbYCf5jkh/mXmf7/3Ab6b54X4l5n/3d4H+G6eP8T/fuYFex/gu3nBEP/7mefvfYDv5oVD/O9nntf7AN/Nvwzxv595Tu8DfDcvGsT/fubZ3gf4bl50iP/9zBXvA3w3/zqI//0MvA/w3fzrIf73e2/gu/m3Qfz7fBfwPvzvhfi3+y7gvQHxvxfi3+a7gPfmCvG/F+Jf77uA9+bZxP9eiH+d7wLem+ck/vdCvOi+C3hvnpf43wvxovku4L15/sT/Xoh/2XcB780LJv73Qrxw3wW8N/+7iRcM8YJ9F/De/O8nXjDE8/fZwGfxf4N4wRDP33Hgt4GX4n8/8YIhXrDjwG8DL8X/buIFQ7xwx4HfBl6KF+x1+J/tt3nBEP+y48BvAy/F8yf+90K8aI4Dvw28FM9L/O+FeNEdB34beCmek/jfC/Gvcxz4beCleDbxvxfiX+848NvAS3GF+N8L8W9zHPht4KUA8b8X4t/uOPDbwEvzvxfi3+c4sMv/Xoj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj3qZSkH9TXlqAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSyncAlt;
impl IconShape for MdSyncAlt {
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
                d: "M22,8l-4-4v3H3v2h15v3L22,8z",
            }
            path {
                d: "M2,16l4,4v-3h15v-2H6v-3L2,16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSystemUpdateAlt;
impl IconShape for MdSystemUpdateAlt {
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
                d: "M12 16.5l4-4h-3v-9h-2v9H8l4 4zm9-13h-6v1.99h6v14.03H3V5.49h6V3.5H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2v-14c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/zVeGvgq/mu8DvDbAOLZXhv4LZ6X+K/x2sBv8V/jdYDfBhDP9trAb/G8xH+N1wZ+i/8arwP8NoB4ttcGfovnJf5rvDbwW/zXeB3gtwHEs7028Fs8L/Ff47WB3+K/xusAvw0gnu21gd/ieYn/Gq8N/Bb/NV4H+G0A8WyvDfwWz0v813ht4Lf4r/E6wG8DiGd7beC3eF7iv8ZrA7/Ff43XAX4bQDzbawO/xfMS/zVeG/gt/mu8DvDbAOLZXhv4LZ6X+K/x2sBv8V/jdYDfBhDP9trAb/G8xH+N1wZ+i/8arwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ7Xa/O/22/zvF4H+G0A8WyvDfwW/z+8DvDbAOI57QLH+L/tEnCcKxDP6bOBz+L/ts8BPpsrEM/ru4H34v+m7wHem2dDPH/vDXw08FL83/A7wHcD381zQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfARaniEFEy+lyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTab;
impl IconShape for MdTab {
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h10v4h8v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+GjgpQHxvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBb/Of4a+Crge/hOSGe13cB782zief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+I/13cD78OzIZ7TZwOfxXMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xX++zwE+mysQz+kicJznJJ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfov/fLvACa5APNtrA7/F8xLP67eB1+I5/Q7w2jwv87xeB/htntNrA7/Ff43XAX4bQDzbceCleV6/zfN6aeA4z2kX+Gue12vzvP4a2OU5vTbwW/zXeB3gtwHE/xyvDfwW/zVeB/htAPE/x2sDv8V/jdcBfhtA/M/x2sBv8V/jdYDfBhD/c7w28Fv813gd4LcBxLMdB16K5/U7/Nd4beC3+K/xOsBvA4hne23gt3he4r/GawO/xX+N1wF+G0A822sDv8XzEv81Xhv4Lf5rvA7w2wDi2V4b+C2el/iv8drAb/Ff43WA3wYQz/bawG/xvMR/jdcGfov/Gq8D/DaAeLbXBn6L5yX+a7w28Fv813gd4LcBxP9viP/fEP+/If5/Q/z/hni2lwa+iuf1Ovzv9ls8r48B/hpAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTTw1Tyv1+Z/t9/meX008NcA4v83xP9viP/fEP+/If5/QzzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4tuPAS/O8fpv/3V6b5/XXwC6A+P8N8f8b4v83xP9viGc7DrwUz+t3eF4vDRzjOV0C/prn9Vo8r78BdnlOx4GX4nn9Ds/rpYFjPKdLwF/zvF6L5/U3wC6AeLbXBn6L5yWe128Dr8Vz+h3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8Vz+h3gtXle5nm9DvDbAOI57QLHeE7ief028Fo8p98BXpvnZZ7X6wC/zXN6beC3eF7ief028Fo8p98BXpvnZZ7TJeA4VyCe02cDn8VzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/znD4H+GyuQDyv7wbei2cTz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/Ns3wO8N8+GeP7eG/ho4KUA8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLwO8B3A9/Nc0L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHExqZBD0uE1AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTabUnselected;
impl IconShape for MdTabUnselected {
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
                d: "M1 9h2V7H1v2zm0 4h2v-2H1v2zm0-8h2V3c-1.1 0-2 .9-2 2zm8 16h2v-2H9v2zm-8-4h2v-2H1v2zm2 4v-2H1c0 1.1.9 2 2 2zM21 3h-8v6h10V5c0-1.1-.9-2-2-2zm0 14h2v-2h-2v2zM9 5h2V3H9v2zM5 21h2v-2H5v2zM5 5h2V3H5v2zm16 16c1.1 0 2-.9 2-2h-2v2zm0-8h2v-2h-2v2zm-8 8h2v-2h-2v2zm4 0h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/Nf6a+Crge/hPxbiRfPewHfx3++7gffhPw7iX/Zg4On8z/E5wGfzHwPxL/tu4L34n2MXOMF/DMS/7CJwnP9ZXgf4bf79EP8y87w+Bvhr/mv8Fs/rdYDf5t8P8S8zz+t1gN/mv4Z5Xq8D/Db/foh/mXlerwP8Nv81zPN6HeC3+fdD/MvM83od4Lf53w/xLzPP63WA3+Z/P8S/zDyv1wF+mxfuwcCD+K/3O7zoEP8y87xeB/htnr/jwHcBb81/j13gq4HP4V+G+JeZ5/U6wG/zvI4DTweO89/vu4H34YVD/MvM83od4Ld5Xj8FvDX/c7wP8N28YIh/mXlerwP8Ns/pwcDT+Z/ld4DX5gVD/MvM83od4Ld5Tq8N/Bb/84gXDPEvM8/rdYDf5jm9NvBbPC/xX+O1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xX+O1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xX+O1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xX+O1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xX+O1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPK/f5r/GceCleV7iBUP8y8zzeh3gt3lOrw38Fv/ziBcM8S8zz+t1gN/mOb028Fv8zyNeMMS/zDyv1wF+m+f02sBv8T+PeMEQ/zLzvF4H+G2e02sDv8X/POIFQ/zLzPN6HeC3eU6vDfwWz+t3eF4vDRzjOV0C/prn9Vo8r78BdnlOx4GX4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ZeOHHFawO/xf884gVD/Mv+GngpXjBxxWsDv8X/POIFQ/zL3hv4Ll4wccVrA7/F/zziBUO8aL4beC+eP3HFawO/xf884gVDvOg+G/ho4BjPSVzx2sBv8T+PeMEQ/3qvzXP6ba54beC3eF6vw/P6auCleE5/A3w0z+u3eF4fA/w1z+mlga/ieYkXDPEf57WB3+J5ief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5iRcM8R/ntYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxH+e1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEf57WB3+J5ief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5iRcM8R/ntYHf4nm9Ns/rq4GX5jn9NfDRPK/f5nl9NPDXPKeXBr6a5yVeMMR/nNcGfov/WS4Bx3nBEP+xdoFj/M/xPcB784Ih/mN9NvBZ/M/xEOBWXjDEf7zvBt6L/37vA3w3LxziP8d7Ax8NvBT/tS4BPw18NnAr/zLE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGiIsFBylD8rAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTableView;
impl IconShape for MdTableView {
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
                d: "M19,7H9C7.9,7,7,7.9,7,9v10c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V9C21,7.9,20.1,7,19,7z M19,9v2H9V9H19z M13,15v-2h2v2H13z M15,17v2h-2v-2H15z M11,15H9v-2h2V15z M17,13h2v2h-2V13z M9,17h2v2H9V17z M17,19v-2h2v2H17z M6,17H5c-1.1,0-2-0.9-2-2V5 c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2v1h-2V5H5v10h1V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFkUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/24vzRV/zb8N4n+vlwZ+iyteB/hr/vUQ/zu9NPBbwHGu2AVeB/hr/nUQ//u8NPBbwHGe0y7wOsBf86JD/O/y0sBvAcd5/naB1wH+mhcN4n+PlwZ+CzjOC7cLvA7w1/zLEP87vDTwW8BxXjS7wOsAf80Lh/if76WB3wKO86+zC7wO8Ne8YIj/ed4K+BvgVq54aeA4z+mrgZfiOf0N8NE8p13gr3nBEP8zHAfeC/ho4MHA6wC/zQv228Br8Zx+B3ht/nUQ/70eDHwW8NbAcZ7tdYDf5gX7beC1eE6/A7w2/zqI/x6vDXwW8No8f68D/DYv2G8Dr8Vz+h3gtfnXQfzXOQ68FfDZwIN54V4H+G1esN8GXovn9DvAa/Ovg/jP92Dgo4D3Bo7zonkd4Ld5wX4beC2e0+8Ar82/DuI/z2sDHwW8NS+6nwG+GvhtXrjfBl6L5/Q7wGvzr4P4j3UceCvgs4EH86J5BvDdwHcDt/Ki+W3gtXhOvwO8Nv86iP8YDwbeC/ho4Dgvuu8B3pt/vd8GXovn9DvAa/Ovg/j3eW3gvYD35t/mc4DP5l/vt4HX4jn9DvDa/Osg/m3eC/ho4KX5l/0N8N3ArcBP8Zw+B/hs/vV+G3gtntPvAK/Nvw7iRfdg4L2AjwaO88JdAr4b+G7gr7nitYHf4jl9DvDZ/Ov9NvBaPKffAV6bfx3Ev+y1gfcC3pt/2c8A3w38NM/rtYHf4jl9DvDZ/Ov9NvBaPKffAV6bfx3Ev8y8cH8DfDfw3cAuL9hrA7/Fc/oc4LP51/tt4LV4Tr8DvDb/Ooh/mXlezwB+Gvhq4FZeNK8N/BbP6XOAz+Zf77eB1+I5/Q7w2vzrIP5l5nndCvw08DXArbxoXhv4LZ7T5wCfzb/ebwOvxXP6HeC1+ddB/MvMC/fXwHcD3wPs8oK9NvBbPKfPAT6bf73fBl6L5/Q7wGvzr4P4l7028N7Ae/Ev+2ngu4Gf4Xm9NvBbPKfPAT6bf73fBl6L5/Q7wGvzr4N40T0YeG/go4FjvHC7wHcD3wP8NVe8NvBbPKfPAT6bf73fBl6L5/Q7wGvzr4P4t3lv4KOBl+Jf9tfAdwPPAH6K5/Q5wGfzr/fbwGvxnH4HeG3+dRD/Pq8NvDfwXvzbfA7w2fzr/TbwWjyn3wFem38dxH+MBwPvDXw0cIwX3XcD78O/3m8Dr8Vz+h3gtfnXQfzHOg68NfDZwIN40dwKfDfwPcCtvGh+G3gtntPvAK/Nvw7iP89rAx8NvBUvup8Gvgb4bV643wZei+f0O8Br86+D+M/3YOCjgfcGjvGieR3gt3nBfht4LZ7T7wCvzb8O4r/OceCtgc8GHsQL9zrAb/OC/TbwWjyn3wFem38dxH+P1wY+G3gtnr/XAX6bF+y3gdfiOf0O8Nr86yD+ez0Y+GzgrYFjPNvrAL/NC/bbwGvxnH4HeG3+dRD/MxwH3hv4aOBBwOsAv80L9tvAa/Gcfgd4bf51EP/zvDXw18CtvGC/DbwWz+l3gNfmXwfxv9NvA6/Fc/od4LX510H87/TbwGvxnH4HeG3+dRD/O/028Fo8p98BXpt/HcT/Tl8NvDTP6a+Bj+ZfB/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I+0DwUEio/iHAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotateUp;
impl IconShape for MdTextRotateUp {
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
                d: "M3 12v1.5l11 4.75v-2.1l-2.2-.9v-5l2.2-.9v-2.1L3 12zm7 2.62l-5.02-1.87L10 10.88v3.74zm8-10.37l-3 3h2v12.5h2V7.25h2l-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFj0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEv81XAy/Fc/ob4KP53wXxb/PbwGvxnH4HeG3+d0H82/w28Fo8p98BXpv/GA8Gvovn9DbALv+xEP82vw28Fs/pd4DX5j/GZwOfxXP6HOCz+Y+F+Lf5beC1eE6/A7w2/zEuAsd5TrvACf5jIf5tfht4LZ7T7wCvzb/fewPfxfP3PsB38x8H8W/z28Br8Zx+B3ht/v3+Cnhpnr+/Bl6G/ziIf5vfBl6L5/Q7wGvz7/PawG/xwr0O8Nv8x0D82/w28Fo8p98BXpt/n+8G3otn+x6ueC+e7XuA9+Y/BuLf5reB1+I5/Q7w2vzbPRh4Os/pbbjip3hODwFu5d8P8W/z28Br8Zx+B3ht/u0+G/gsnu0ZwIO5Yhc4xrN9DvDZ/Psh/m1+G3gtntPvAK/Nv91F4DjP9jXAR3PFVwMfxbPtAif490P82/w28Fo8p98BXpt/m/cGvovn9DLAX3PFSwN/xXN6H+C7+fdB/Nv8NvBaPKffAV6bf5unAw/m2f4GeGme018DL8Wz/TXwMvz7IP5tfht4LZ7T7wCvzb/eawO/xXP6GOCreU4fDXwVz+l1gN/m3w7xb/PbwGvxnH4HeG3+9X4aeCue0wlgl+d0HLjIc/oZ4K35t0P82/w28Fo8p98BXpt/nQcDT+c5/Qzw1jx/Pw28Fc/pIcCt/Nsg/m1+G3gtntPvAK/Nv85XAx/Fc3ob4Kd5/t4a+Cme09cAH82/DeLf5reB1+I5/Q7w2rzojgNPB47zbJeA47xwu8Axnm0XeAiwy78e4t/mt4HX4jn9DvDavOjeG/guntPXAB/NC/fVwEfxnN4H+G7+9RD/Nr8NvBbP6XeA1+ZF93TgwTynlwH+mhfupYG/4jndCjyEfz3Ev81vA6/Fc/od4LV50bw28Fv8x3od4Lf510H82/w28Fo8p98BXpsXzU8Db8V/rJ8B3pp/HcS/zW8Dr8Vz+h3gtfmXPRh4Ov85HgLcyosO8W/z28Br8Zx+B3ht/mXfDbwX/zm+BvhoXnSIf5vfBl6L5/Q7wGvzwh0Hng4c5zn9DbDLv85x4KV4TrvAQ4BdXjSIf5vfBl6L5/Q7wGvzwn008FU8rxPALv86x4GLPK+PAb6aFw3i3+a3gdfiOf0O8Nq8cE8HHsxz+h7gvfm3+W7gvXhOtwIP4UWD+Lf5beC1eE6/A7w2L9hbAz/F83od4Lf5t3lt4Ld4Xm8D/DT/MsS/zW8Dr8Vz+h3gtXnBfgt4bZ7T3wAvzb/PXwMvxXP6beB1+Jch/m1+G3gtntPvAK/N8/dg4Ok8r/cBvpt/n/cGvovn9RDgVl44xL/NbwOvxXP6HeC1ef4+GnhrntdbA7v8+xwHfprn9dPAV/PCIf5tfht4LZ7T7wCvzf8uiH/ZSwPHeE5fDbw0z+mvgY/mOV0C/pr/uRD/spcGfhs4xr/OJeC1gb/mfy7Ei+algd8GjvGiuQS8NvDX/M+GeNG9NPDbwDFeuEvAawN/zf98iH+dlwZ+GzjG83cJeG3gr/nfAfGv99LAbwPHeE6XgNcG/pr/PRD/Ni8N/DZwjCsuAa8N/DX/uyD+7V4a+G2ueG3gr/nfB/Hv89Jc8df874T4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHnbr9BwpblywAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotateVertical;
impl IconShape for MdTextRotateVertical {
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
                d: "M15.75 5h-1.5L9.5 16h2.1l.9-2.2h5l.9 2.2h2.1L15.75 5zm-2.62 7L15 6.98 16.87 12h-3.74zM6 19.75l3-3H7V4.25H5v12.5H3l3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif7eXBt4L+B7gr/nXQ/zvcxx4K+CjgZfmil3gdYC/5l8H8b/HSwMfBbw1cJzntQu8DvDXvOgQ/7MdB94K+GjgpfmXfQ3w0bzoEP8zvTTwUcBbA8d50T0EuJUXHeJ/juPAWwEfDbw0L9wzgF3gpXi23wFem38dxH+/1wbeC3hr4Dgv3N8AXw18N3AROM6zvQ/w3fzrIP57HAfeC/ho4MH8y34H+Gzgt7nivYHv4tkuAcf510P813pt4L2A9+ZF8z3AVwN/zXN6OvBgnu17gPfmXw/xn+848F7ARwMP5l92Cfhu4KuBW3lebw38FM/pZYC/5l8P8Z/ntYH3At6bF80zgO8GvhrY5QX7LeC1eba/AV6afxvEf6zjwHsBHw08mBfdXwMvw7/stYHf4jm9D/Dd/Nsg/mO9N/Bd/Nt8N/A+vHA/DbwVz3YJeDCwy78N4j/eewPfxQv2O8BLA8d4Xt8NvA/P34OBp/Ocvgd4b/7tEP853hv4Lp7tEvDTwGcDtwIvDfw2cIzn9d3A+/C8vht4L57TywB/zb8d4kVzHPgt4GuA7+ZF897AZwNfDXw3sMtzemngt4FjPK/vBt6HZzsOXOQ5/Q3w0vz7IP5lx4HfAl6aK94H+G7+Y7w08NvAMZ7XdwPvwxWfDXwWz+l9gO/m3wfxwh0Hfgt4aZ7T+wDfzX+MlwZ+GzjG8/pu4GOApwPHebZLwIOBXf59EC/cbwOvxfP3PsB38x/jpYHfBo7xvG4FHsxz+h7gvfn3Q7xwLw38NnCM5+99gO/mP8ZLA78NHONf9jLAX/Pvh/iXvTTw28Axnr/3Ab6b/xgvDfw2cIwX7G+Al+Y/BuJF89LAbwPHeP7eB/hu/mO8NPDbwDGev/cBvpv/GIgX3UsDvw0c4/l7H+C7+Y/x0sBvA8d4TpeABwO7/MdA/Ou8NPDbwDGev/cBvpv/GH8FvDTP6XuA9+Y/DuJf76WB3waO8fy9D/Dd/Pu8NPBXPK+XAf6a/ziIf5uXBn4bOMbz9z7Ad/Nv993Ae/Gc/gZ4af5jIf7tXhr4beAYz9/7AN/Nv95x4CLP632A7+Y/FuLf56WB3waO8fy9D/Dd/Ot8NvBZPKdLwIOBXf5jIf79Xhr4beAYz9/7AN/Ni+7pwIN5Tt8DvDf/8RD/MV4a+G3gGM/f+wDfzYvmvYHv4jm9DPDX/MdD/Md5aeC3gWM8f+8DfDcvmvcGvosr/gZ4af5zIP5jvTTw28Axnr/3Ab6bF817A98FvA/w3fznQPzHe2ngt4FjPH/vA3w3L5r3Bn4a2OU/B+I/x0sDvw0c4/l7H+C7+e+H+M/z0sBvA8d4/t4H+G6ev+PALv/5EP+5Xhr4beAYz9/7AN/NczoO/BbwMvznQ/zne2ngt4FjPH/vA3w3VxwHfgt4aUD850P813hp4LeBYzx/7wP8NPBbwEtzhfjPh/iv89LAbwPHeP5uBR7Ms4n/fIj/Wi8N/DZwjH+Z+M+H+K/30sBvA8d44cR/PsR/j5cGfhs4xgsm/vMh/nscB34LeGleMPGfD/Ff7zjwW8BL88KJ/3yI/1rHgd8CXpp/mfjPh/ivcxz4LeCledGI/3yI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPXwa1B+c+wdQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotationAngledown;
impl IconShape for MdTextRotationAngledown {
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
                d: "M19.4 4.91l-1.06-1.06L7.2 8.27l1.48 1.48 2.19-.92 3.54 3.54-.92 2.19 1.48 1.48L19.4 4.91zm-6.81 3.1l4.87-2.23-2.23 4.87-2.64-2.64zM14.27 21v-4.24l-1.41 1.41-8.84-8.84-1.42 1.42 8.84 8.84L10.03 21h4.24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEv857Ay8FfA1wK//7IV507w18F8/228B3A9/D/16IF817A9/F87cL/DTwNcBf878L4kXz08Bb8S/7a+CrgZ8BdvmfD/GieWvgp3jR7QI/DXwN8Nf8z4V40d0KPIhn+xvgOPAgXri/Br4a+Blgl/9ZEC+6zwY+i2f7a+BlgPcGPht4EC/cLvDTwNcAf83/DIgX3YOBp/OcXgf4ba54a+CjgdfiX/bXwFcDPwPs8t8H8a/z08Bb8Wy/DbwOz+m1gfcG3ot/2S7w08DXAH/Nfz3Ev85bAz/Fc3oIcCvP68HAZwNvDRzjX3Yr8NXA9wC7/NdA/OvdCjyIZ/se4L15wY4DHw18NHCMF813A98D/Db/uRD/ep8NfBbP6SHArbxwDwb+CjjOi+5W4KuB7wF2+Y+H+Nd7MPB0ntPnAJ/NC3Yc+C3gpfnXuwS8NvDX/MdD/Nv8NPBWPNsu8BBgl+d1HPgt4KV5Xn8D7AKvxfN3CXht4K/5z4H4t3lr4Kd4Th8DfDXP6TjwW8BL87z+BnhtYBd4aeCjgffi2S4Brw38Nc92HPht4KX4l/0O8Nq8cIh/u1uBB/FstwIP4dmOA78FvDTP62+A1wZ2eU7HgfcG3ht4b+CveV7Hgd8GXooX7neA1+aFQ/zbfTbwWTyn9wG+GzgO/Bbw0jyvvwFeG9jl3+448NvAS/GC/Q7w2rxwiH+7BwNP5zn9NfA6wG8BL83z+hvgtYFd/v2OA08HjvP8/Q7w2rxwiH+fnwbeiud0K/BgntffAK8N7PIf47uA9+YF+x3gtXnhEP8+7w18F/+yvwFeG9jlP8Z3Ae/NC/c7wGvzwiH+/XaBY7xgfwO8NrDLf4zvAt6bf9nvAK/NC4f49/tq4KN4/v4GeG1gl/8Y3wW8N8/fJeBW4KW44neA1+aFQ/z7PRh4Os/rb4DXBnb5j/FdwHvz/F0CXhu4Ffht4KWA3wFemxcO8R/jt4HX4jl9D/De/Mf4LuC9ef4uAa8N/DVXHAd+G9gFXpsXDvEf472B7+J5PQS4lX+f7wLem+fvEvDawF/znI4Dnw18NC8c4j/OLnCM5/Q5wGfzb/ddwHvz/F0CXhv4a/7tEP9xvhr4KJ7TrcBD+Lf5LuC9ef4uAa8N/DX/Poj/OA8Gns7zeh/gu/nX+S7gvXn+LgGvDfw1/36I/1i/DbwWz+lngLfmRfddwHvz/F0CXhv4a/5jIP5jvTfwXTzbJeC1gb/mRfNdwHvz/F0CXhv4a/7jIP7j7QLHgEvAawN/zYvmu4D35vm7BLw28Nf8x0L8x/tq4L2B1wb+mhfNdwHvzfN3CXht4K/5j4f4j/dg4Djw17xovgt4b56/S8BrA3/Nfw7Ef6/vAt6b5+8S8NrAX/OfB/Hf57uA9+b5uwS8NvDX/OdC/Pf4LuC9ef4uAa8N/DX/+RD/9b4LeG+ev0vAawN/zX8NxH+t7wLem+fvEvDawF/zXwfxX+e7gPfm+bsEvDbw1/zXQvzX+C7gvXn+LgGvDfw1//UQ//m+C3hvnr9LwGsDf81/D8R/ruPAbwMvxfO6BLw28Nf890H85zsO/DbwUjzbJeC1gb/mvxfiv8Zx4LeBlwIuAa8N/DX//RD/dY4DPw18NPDX/M+A+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CP1nfQZf9XfQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotationAngleup;
impl IconShape for MdTextRotationAngleup {
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
                d: "M4.49 4.21L3.43 5.27 7.85 16.4l1.48-1.48-.92-2.19 3.54-3.54 2.19.92 1.48-1.48L4.49 4.21zm3.09 6.8L5.36 6.14l4.87 2.23-2.65 2.64zm12.99-1.68h-4.24l1.41 1.41-8.84 8.84L10.32 21l8.84-8.84 1.41 1.41V9.33z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFj0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEv81XAy/Fc/ob4KP53wXxb/PbwGvxnH4HeG3+d0H82/w28Fo8p98BXpv/XRD/Nr8NvBbP6XeA1+Z/F8S/zW8Dr8Vz+h3gtXnBHgy8FPAz/M+B+Lf5beC1eE6/A7w2L9hrA78F3Ap8NfA9wC7/vRD/Nr8NvBbP6XeA1+YFe23gt3i2XeCngc8BbuW/B+Lf5reB1+I5/Q7w2rxgrw38Fs/fbwOfA/w2/7UQ/za/DbwWz+l3gNfmBXtt4Ld44W4FPhv4GWCX/3yIf5vfBl6L5/Q7wGvzgr028Fu8aHaB7wa+BriV/zyIf5vfBl6L5/Q7wGvzwr028NHAW/Gi+2nga4Df5j8e4t/mt4HX4jn9DvDavGgeDLw38N7Ag3jR3Ap8NvAzwC7/MRD/Nr8NvBbP6XeA1+Zf77uB9+JFtwt8NfA9wK38+yD+bX4beC2e0+8Ar82/3mcDn8W/zXcD3wP8Nv82iH+b3wZei+f0O8Br86/32cBn8ZzeBngw8N7AS/Ev+2vgq4Hv4V8H8W/z28Br8Zx+B3ht/vU+G/gsntPrAL/NFS8NvDfw3sAxXrhd4KuB7wFu5V+G+Lf5beC1eE6/A7w2/3qfDXwWz+l1gN/meb018N7AW/Ev+27ge4Df5gVD/Nv8NvBaPKffAV6bf73PBj6L5/Q6wG/zgh0H3ht4b+CleOHEC4b4t/lt4LV4Tr8DvDb/ep8NfBbP6XWA3+ZF89LAewNvDTyI5yVeMMS/zW8Dr8Vz+h3gtfnX+2zgs3hOrwP8Ni+alwbeC3hr4ME8L/GCIf5tfht4LZ7T7wCvzb/eZwOfxXN6HeC3ecGOA+8FvDfw0rxw4gVD/Nv8NvBaPKffAV6bf73PBj6L5/Q6wG/zvN4KeG/grfmXfQ/w3cBv84Ih/m1+G3gtntPvAK/Nv95nA5/Fc3od4Le54qWB9wLeGzjOC3cJ+Grgu4Fb+Zch/m1+G3gtntPvAK/Nv95nA5/Fc3ob4EHAewMvzb/sb4CvBr6bfx3Ev81vA6/Fc/od4LX51/ts4LP4t/ke4LuB3+bfBvFv89vAa/Gcfgd4bf71vgt4b150l4CvBr4buJV/H8S/zW8Dr8Vz+h3gtXnRPBh4L+C9gQfzonkG8NnAd/MfB/Fv89vAa/Gcfgd4bV641wY+CnhrXnQ/A3w18Nv8x0P82/w28Fo8p98BXpsX7LWB3+JFcwn4buCrgVv5z4P4t/lt4LV4Tr8DvDYv2GsDv8UL9wzgs4GfBnb5z4f4t/lt4LV4Tr8DvDYv2GsDv8Xz9zPAVwO/zX8txL/NbwOvxXP6HeC1ecFeG/gtnu0S8NPAZwO38t8D8W/z28Br8Zx+B3htXrDXBn4LeAbw1cB3A7v890L8y14aOMZz+mrgpXlOfw18NM/pEvDXXPFg4KWBn+Z/DsS/7KWB3waO8a9zCXht4K/5nwvxonlp4LeBY7xoLgGvDfw1/7MhXnQvDfw2cIwX7hLw2sBf8z8f4l/npYHfBo7x/F0CXhv4a/53QPzrvTTw28AxntMl4LWBv+Z/D8S/zUsDvw0c44pLwGsDf83/Loh/u5cGfpsrXhv4a/73Qfz7vDRX/DX/OyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRhI8hB1R94zAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotationDown;
impl IconShape for MdTextRotationDown {
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
                d: "M21 12v-1.5L10 5.75v2.1l2.2.9v5l-2.2.9v2.1L21 12zm-7-2.62l5.02 1.87L14 13.12V9.38zM6 19.75l3-3H7V4.25H5v12.5H3l3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HawG/xnF4G+Gv+ayH+e3w38F48p+8B3pv/Woj/eg8Gns7z9xDgVv7rIP7rfTbwWTx/nwN8Nv91EP/1LgLHef52gRP810H813pv4Lt4tmdwxYN4tvcBvpv/Goj/Wn8FvDTP9jVc8VE8218DL8N/DcR/ndcGfovn9BDgOPBXPKfXAX6b/3yI/zrfDbwXz/Y3wEtzxV8DL8WzfQ/w3vznQ/zXeDDwdJ7TxwBfzRUfDXwVz+khwK3850L81/hs4LN4TieAXa44DlzkOX0O8Nn850L85zsOPB04zrP9DPDWPKefBt6KZ9sFTvCfC/Gf772B7+I5vQ3w0zyntwZ+iuf0PsB3858H8Z/v6cCDebZLwHGev13gGM92K/AQ/vMg/nO9NvBbPKevAT6a5++rgY/iOb0O8Nv850D85/pp4K14Ti8D/DXP30sDf8Vz+hngrfnPgfjP82Dg6TynvwFemhfur4GX4jk9BLiV/3iI/zxfDXwUz+ljgK/mhfto4Kt4Tl8DfDT/8RD/OY4DTweO85zeGtjlhTsO/DTPaRd4CLDLfyzEf473Br6L/1jvA3w3/7EQ/zmeDjyY/1i3Ag/hPxbiP95rA7/Ff47XAX6b/ziI/3i/Bbw2/zl+Bnhr/uMg/mM9GHg6z+t7gFv513kw8F48r4cAt/IfA/Ef67uB9+I5PQN4MP82twIP4jl9D/De/MdA/Mc5DjwdOM5z+hzgs/m3+Wzgs3hOu8BDgF3+/RD/cT4a+Cqe10OAW/m3eTDwdJ7XxwBfzb8f4j/O04EH85x+Bnhr/n1+GngrntOtwEP490P8x3hr4Kd4Xq8D/Db/Pq8N/BbP622An+bfB/Ef48HAg3lev81/jNfmed0K3Mq/D+L/N8T/b4j/vV6aK/6afzvE/16vDfwU8DrAX/Nvg/jf67WB3wJ2gdcB/pp/PcT/Xq8N/BZX7AKvA/w1/zqI/71eG/gtnm0XeB3gr3nRIf5tvhp4Kf57HQdemue0C7wO8Ne8aBD/Nr8NvBb/M+0CrwP8Nf8yxL/NbwOvxf9cu8DrAH/NC4f4t/lt4LX4n+sS8NrAX/PCIf5tfht4Lf5nugS8NvDX/MsQ/zZfDbw0/72OAy/Fc7oEvDbw17xoEP97vTbwWzzbJeC1gb/mRYf43+u1gd/iikvAawN/zb8O4n+v1wZ+C7gEvDbw1/zrIf73em3gp4HXBv6afxvE/14vzRV/zb8d4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8ImgaOQRjRhNkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextRotationNone;
impl IconShape for MdTextRotationNone {
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
                d: "M12.75 3h-1.5L6.5 14h2.1l.9-2.2h5l.9 2.2h2.1L12.75 3zm-2.62 7L12 4.98 13.87 10h-3.74zm10.37 8l-3-3v2H5v2h12.5v2l3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F8xLP67eB1+I/1u8Ar83zMs/rdYDf5l+GeNG9NvBbPC/xvH4beC3+Y/0O8No8L/O8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+ZfhnjRvTbwWzwv8bx+G3gt/mP9DvDaPC/zvF4H+G3+ZYgX3WsDv8Xz+m2e10sDx/mPtQv8Nc/rtXlerwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gtntfv8LxeGjjGf6xLwF/zvF6L5/U6wG/zL0O86F4b+C2el3hevw28Fv+xfgd4bZ6XeV6vA/w2/zLEi+61gd/ieYnn9dvAa/Ef63eA1+Z5mef1OsBv8y9DvOheG/gtnpd4Xr8NvBb/sX4HeG2el3lerwP8Nv8yxIvutYHf4nmJ5/XbwGvxH+t3gNfmeZnn9TrAb/MvQ7zoXhv4LZ6XeF6/DbwW/7F+B3htnpd5Xq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fs/rdXheXw28FP+x/gb4aJ7Xb/G8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+ZfhnjRvTbwWzwv8bx+G3gt/mP9DvDaPC/zvF4H+G3+ZYgX3WsDv8XzEs/rt4HX4j/W7wCvzfMyz+t1gN/mX4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F83ptntdXAy/Nf6y/Bj6a5/XbPK/XAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfovnJZ7XbwOvxX+s3wFem+dlntfrAL/Nvwzxontt4Ld4XuJ5/TbwWvzH+h3gtXle5nm9DvDb/MsQL7rXBn6L5yWe128Dr8V/rN8BXpvnZZ7X6wC/zb8M8aJ7beC3eF7ief028Fr8x/od4LV5XuZ5vQ7w2/zLEC+61wZ+i+clntdvA6/Ff6zfAV6b52We1+sAv82/DPGie23gt3hev83zemngOP+xdoG/5nm9Ns/rdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F8/odntdLA8f4j3UJ+Gue12vxvF4H+G3+ZYgX3WsDv8XzEs/rt4HX4j/W7wCvzfMyz+t1gN/mX4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F8xLP67eB1+I/1u8Ar83zMs/rdYDf5l+GeNG9NvBbPC/xvH4beC3+Y/0O8No8L/O8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+Zfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AlAJukHZQLk/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTheaters;
impl IconShape for MdTheaters {
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
                d: "M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t3kv4L2BlwaO85/vr4GvBr6HF+y1gd/iRfM6wG8DiH+97wLem/8e3w28D8/fawO/xYvmdYDfBhD/Op8NfBb/vT4H+Gye12sDv8WL5nWA3wYQL7rjwNOB4/z32gVO8LxeG/gtXjSvA/w2gHjRfTTwVfzP8DrAb/OcXhv4LV40rwP8NoB40T0deDD/M7wO8Ns8p9cGfosXzesAvw0gXjRvDfwU/3O8DvDbPKfXBn6LF83rAL8NIF40vwW8Nv9zvA7w2zyn1wZ+ixfN6wC/DSD+ZQ8Gns7/LK8D/DbP6bWB3+JF8zrAbwOIf9l3A+/F/yyvA/w2z+m1gd/iRfM6wG8DiBfuOPB04Dj/s7wO8Ns8p9cGfosXzesAvw0gXriPBr6K/3leB/htntNrA7/Fi+Z1gN8GEC/c04EH8z/P6wC/zXN6beC3eNG8DvDbAOIFe2vgp/if6XWA3+Y5vTbwW7xoXgf4bQDxgv0W8Nr8z/Q6wG/znF4b+C1eNK8D/DaAeP4eDDyd/7leB/htntNrA7/Fi+Z1gN8GEM/fdwPvxf9crwP8Ns/ptYHf4kXzOsBvA4jndRx4OnCc/7leB/htntNrA7/Fi+Z1gN8GEM/ro4Gv4n+21wF+m+f02sBv8aJ5HeC3AcTzejrwYP5nex3gt3lOrw38Fi+a1wF+G0A8p7cGfor/+V4H+G2e02sDv8WL5nWA3wYQz+m3gNfmf77XAX6b5/TawG/xonkd4LcBxLM9GHg6/zu8DvDbPKfXBn6LF83rAL8NIJ7ttYHf4n+H1wF+m+f02sBv8aJ5HeC3AcSzvTbwW/zv8DrAb/OcXhv4LV40rwP8NoB4ttcGfov/HV4H+G2e02sDv8WL5nWA3wYQz/bawG/xv8PrAL/Nc3pt4Ld40bwO8NsA4tleG/gt/nd4HeC3eU6vDfwWL5rXAX4bQDzbawO/xf8OrwP8Ns/ptYHf4kXzOsBvA4hne23gt/jf4XWA3+Y5vTbwW7xoXgf4bQDxbC8N/BX/O7wO8Ns8p9cGfosXzesAvw0gntOtwIP4n+91gN/mOb028Fu8aF4H+G0A8ZxeG/gt/ud7HeC3+fdDPK/3Br4aOMb/XA8BbuXfD/H8HQfeG3hp4MH8+xwHXor/OD8DvDX/MRD/+d4b+C7+Y1wCHgzs8h8D8Z/vq4GP4t/vEvDawF/zHwfxn++3gdfi3+cS8NrAX/MfC/Gfz/z7XAJeG/hr/uMh/nO9NPBX/NtdAl4b+Gv+cyD+c7038F3821wCXhv4a/7zIP5zfTXwUfzrXQJeG/hr/nMh/nP9FfDS/OtcAl4b+Gv+8yH+c5l/nUvAawN/zX8NxH+e1wZ+ixfdJeC1gb/mvw7iP89HA1/Fi+YS8NrAX/NfC/Gf57uB9+Jfdgl4beCv+a+H+M/z28Br8cJdAl4b+Gv+eyD+8/w08Fa8YJeA1wb+mv8+iP887w18F8/fJeC1gb/mvxfiP9d3A+/Fc7oEvDbw1/z3Q/zn+27gvbjiEvDawF/zPwPiv8Z3A28NvDbw1/zPgfiv82DgVv5nQfz/hvj/jX8Ely69QR5BcgMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbDown;
impl IconShape for MdThumbDown {
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
                d: "M15 3H6c-.83 0-1.54.5-1.84 1.22l-3.02 7.05c-.09.23-.14.47-.14.73v2c0 1.1.9 2 2 2h6.31l-.95 4.57-.03.32c0 .41.17.79.44 1.06L9.83 23l6.59-6.59c.36-.36.58-.86.58-1.41V5c0-1.1-.9-2-2-2zm4 0v12h4V3h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbDownOffAlt;
impl IconShape for MdThumbDownOffAlt {
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
                d: "M10.89 18.28l.57-2.89c.12-.59-.04-1.2-.42-1.66-.38-.46-.94-.73-1.54-.73H4v-1.08L6.57 6h8.09c.18 0 .34.16.34.34v7.84l-4.11 4.1M10 22l6.41-6.41c.38-.38.59-.89.59-1.42V6.34C17 5.05 15.95 4 14.66 4h-8.1c-.71 0-1.36.37-1.72.97l-2.67 6.15c-.11.25-.17.52-.17.8V13c0 1.1.9 2 2 2h5.5l-.92 4.65c-.05.22-.02.46.08.66.23.45.52.86.88 1.22L10 22zm10-7h2V4h-2c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/pgcDt/KfD/E/z0sDvwX8NPA+/OdC/M/y0sBvAce54ruB9+E/D+J/jpcGfgs4znP6buB9+M+B+J/hpYHfAo7z/L0P8N38x0P893tp4LeA47xgPwO8Nf/xEP+9Xhr4LeA4L9zvAK/NfzzEf5+XBn4LOM6/7HuA9+Y/HuK/x0sDvwUc50XzMcBX8x8P8V/vpYHfAo7zonsd4Lf5j4f4r/XSwG8Bx/nXEf85EP91Xhr4LeA4/zq/A7w2/zkQ/zVeGvgt4Dj/el8DfDT/ORD/+V4a+C3gOP827wN8N/85EP+5Xhr4LeA4/3YvA/w1/zkQ/3leGvgt4Dj/PuI/D+I/x0sDvwUc59/nd4DX5j8P4j/eSwO/BRzn3+9rgI/mPw/iP9Zx4OnAcf5jvA3w0/znQfzH+ingrfmP89fALv8+twJ/DXwPsMtzQvzHeTDwdP7n2gU+Bvhung3xH+e1gd/if77XAX6bKxDP9trAb/GieR3gt3lOrw38Fv/z3Qo8hCsQz/bawG/xonkd4Ld5Tq8N/Bb/O7wM8NcA4tleG/gtXjSvA/w2z+m1gd/if4fXAX4bQDzbawO/xYvmdYDf5jm9NvBb/O/wOsBvA4hne23gt3jRvA7w2zyn1wZ+i/8dXgf4bQDxbK8N/BYvmtcBfpvn9NrAb/G/w+sAvw0gnu21gd/iRfM6wG/znF4b+C3+d3gd4LcBxLO9NvBbvGheB/htntNrA7/F/w6vA/w2gHi21wZ+ixfN6wC/zXN6beC3+N/hdYDfBhDP9trAb/GieR3gt3lOrw38Fv87nAB2AcSzvTbwW7xoXgf4bZ7TawO/xf98vwO8Nlcgnu21gd/iRfM6wG/znF4b+C3+53sb4Ke5AvFsrw38Fi+a1wF+m+f02sBv8T/bM4AH82yIZ3tt4Ld40bwO8Ns8p9cGfov/2T4G+GqeDfFsrw38Fi+a1wF+m+f02sBv8T/XJeDBwC7Phni21wZ+ixfN6wC/zXN6beC3+J/re4D35jkhnu21gd/iRfM6wG/znF4b+C3+53oIcCvPCfFsrw38Fi+a1wF+m+f02sBv8T/T7wCvzfNCPNtrA7/Fi+Z1gN/mOb028Fv8z/Q2wE/zvBDP9trAb/GieR3gt3lOrw38Fv/zPAN4MM8f4tleG/gtXjSvA/w2z+m1gd/if56PAb6a5w/xbK8N/BYvmtcBfpvn9NrAb/E/yyXgwcAuzx/i2V4b+C1eNK8D/DbP6bWB3+J/lu8B3psXDPFsrw38Fi+a1wF+m+f02sBv8T/LQ4BbecEQz/bawG/xonkd4Ld5Tq8N/Bb/c/wO8Nq8cIhne23gt3jRvA7w2zyn1wZ+i/853gb4aV44xLO9NvBbvGheB/htntNrA7/F/wzPAB7MvwzxbK8N/BYvmtcBfpvn9NrAb/E/w8cAX82/DPFsrw38Fi+a1wF+m+e1Cxzjv9cl4MHALv8yxLO9NvBbvGheB/htntdnA5/Ff6+PAb6aFw3i2V4b+C1eNK8D/DbP33cD78V/j+8B3psXHeLZXhv4LV40rwP8Ni/YewMfDbwU//kuAb8N/DTw3fzrIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPNuOzQeZBX+gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbUp;
impl IconShape for MdThumbUp {
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
                d: "M1 21h4V9H1v12zm22-11c0-1.1-.9-2-2-2h-6.31l.95-4.57.03-.32c0-.41-.17-.79-.44-1.06L14.17 1 7.59 7.59C7.22 7.95 7 8.45 7 9v10c0 1.1.9 2 2 2h9c.83 0 1.54-.5 1.84-1.22l3.02-7.05c.09-.23.14-.47.14-.73v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8xwHXhv4af7zIf5nOQ78FvDSwPsA381/LsT/HMeB3wJemmd7H+C7+c+D+J/hOPBbwEvznHaBlwFu5T8H4r/fceC3gJfm+ftp4G34z4H473Uc+C3gpXnhHgLcyn88xH+f48BvAS/Nv+wEsMt/PMR/j+PAbwEvzb/sEnCc/xyI/3rHgd8CXpoXze8Ar81/DsR/rePAbwEvzYvuc4DP5j8H4r/OceC3gJfm+fsZ4K14Xm8D/DT/ORD/NY4DvwW8NM/f9wDfDfwWz+shwK3850D85zsO/Bbw0jx/3wO8N/DZwGfxnC4Bx/nPg/jPdRz4LeClef6+B3hvrvhp4K14Tr8DvDb/eRD/eY4DvwW8NM/f9wDvzbM9HXgwz+lrgI/mPw/iP8dx4LeAl+b5+x7gvXm248BFntf7AN/Nfx7Ef7zjwG8BL83z9z3Ae/OcXhv4LZ7XywB/zX8exH+s48BvAS/N8/c9wHvzvD4b+Cyel/jPhfiP9dXAR/H8fQ/w3jx/Xw18FM/pd4DX5j8X4vn7beC1eF5/DbwP8Nc8r+PARZ6/7wHemxfsu4H34j/WrcB3A18D7PL8IZ6/3wZei+dvF3gIsMtzem3gt3he3wO8Ny/cRwNfxX+OvwbeBriV54V4/n4beC1esNcBfpvn9NrAb/G8xL/sOHArcIz/HH8NvAzPC/H8/TbwWrxgrwP8Ns/ptYHf4nmJF81bAz/Ff563AX6a54R4/n4beC1esNcBfpvn9NrAb/G8xIvutYHvBh7Ef7zPAT6b54R4/n4beC1esNcBfpvn9NrAb/G8xL/eSwPH+bf7auCleE6/A7w2zwnx/P028Fq8YK8D/DbP6bWB3+J5if96vw28Fs/pd4DX5jkhnr/fBl6LF+x1gN/mOb028Fs8L/Ff77eB1+I5/Q7w2jwnxPP328Br8YK9DvDbPKfXBn6L5yX+6/0V8NI8p+8B3pvnhHj+fht4LV6w1wF+m+f02sBv8bzEf63XBn6L5/UxwFfznBDP328Dr8UL9jrAb/OcXhv4LZ6X+K/13cB78bweAtzKc0I8f78NvBYv2OsAv81zem3gt3he4r/Og4Gn87x+Bnhrnhfi+ftt4LV4wV4H+G2e02sDv8XzEv91Phv4LJ7X6wC/zfNCPH+/DbwWL9jrAL/Nc3pt4Ld4XuK/xnHg6cBxntMzgAfz/CGev98GXosX7HWA3+Y5vTbwWzwv8V/jvYHv4nm9D/DdPH+I5++3gdfiBXsd4Ld5Tq8N/BbPS/zXeDrwYJ7TJeDBwC7PH+L5+23gtXjBXgf4bZ7TawO/xfMS//leG/gtntfnAJ/NC4Z4/n4beC1esNcBfpvn9NrAb/G8xH++nwbeiuf1EOBWXjDE8/fbwGvxgr0O8Ns8p9cGfovnJf5zPRh4Os/re4D35oVDPH+/DbwWL9jrAL/Nc3pt4Ld4XuI/13cD78Xzeh3gt3nhEM/fbwOvxQv2OsBv85xeG/gtnpf4z3MceDpwnOf0N8BL8y9DPH+/DbwWL9jrAL/Nc3pt4Ld4XuI/z0cDX8Xzeh/gu/mXIZ6/3wZeixfsdYDf5jm9NvBbPC/xn+fpwIN5TpeA47xoEM/fbwOvxQv2OsBv85xeG/gtntfLAH/Nf7y3Bn6K5/U5wGfzokE8f78NvBYv2OsAv81zemngr/jvdwLY5UWDeP5+G3gtXrDXAX6b53Ur8CD++3wP8N686BDP328Dr8UL9jrAb/O83hr4Kf77vAzw17zoEM/fbwOvxQv2MsBf8/y9N/Bd/Nf7HOCz+ddBPH+/DbwWz9/nAJ/NC/fawHcDD+I/3zOAzwa+m389xPP30sBxntcu8Ne86B4MPJj/XL/Nvx3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPlS89Bf2mFOAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbUpOffAlt;
impl IconShape for MdThumbUpOffAlt {
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
                d: "M13.11 5.72l-.57 2.89c-.12.59.04 1.2.42 1.66.38.46.94.73 1.54.73H20v1.08L17.43 18H9.34c-.18 0-.34-.16-.34-.34V9.82l4.11-4.1M14 2L7.59 8.41C7.21 8.79 7 9.3 7 9.83v7.83C7 18.95 8.05 20 9.34 20h8.1c.71 0 1.36-.37 1.72-.97l2.67-6.15c.11-.25.17-.52.17-.8V11c0-1.1-.9-2-2-2h-5.5l.92-4.65c.05-.22.02-.46-.08-.66-.23-.45-.52-.86-.88-1.22L14 2zM4 9H2v11h2c.55 0 1-.45 1-1v-9c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3eWngq4C3AXb53wvxr/fSwG8Bx4G/Bl4H2OV/J8S/zksDvwUc59l+G3gd/ndCvOheGvgt4DjP62WAv+Z/H8SL5qWB3wKO8/y9DvDb/O+D+Je9NPBbwHFesIcAt/K/D+KFe2ngt4DjvGCXgOP874R4wV4a+C3gOC/c7wCvzf9OiOfvpYHfAo7zL/sc4LP53wnxvF4a+C3gOC+atwF+mv+dEM/ppYHfAo7zonsIcCv/OyGe7aWB3wKO86J7BvBg/vdCXPHSwG8Bx/nX+RngrfnfCwEPBv4KOM6/3q3Arfzr3Qr8DPDT/PdCwE8Db8V/j88BPpv/Pgi4CBznv8etwEP474MA899L/PdBgPnvJf77IMD89xL/fRBg/nuJ/z4IMP99/gZ4af77IMD893kf4Lv574MA89/jEvBgYJf/Pggw/z2+Bvho/nshwPz3eAhwK/+9EGD+6/0M8Nb890OA+a/3OsBv86I5DrwU/7F+hysQYP5rPQN4MC+a9wa+CjjOf6xd4GOA7xZg/mu9D/Dd/MveGvgp/nO9jgDzX+cS8GBgl3/Z04EH85/rdwSY/zpfA3w0/7K3Bn6K/3wIMP91HgLcyr/sp4G34j8fAsx/jZ8B3pp/2YOBp/NfAwHmv8brAL/Nv+yrgY/ivwYCfht4Lf5zPQN4MP+y48DTgeP810DAawO/xX+utwF+mn/ZewPfxX8dxBXvDXw1cIz/WJeAjwa+mxfN04EH818H8Zxem/9Yfw3s8qJ5beC3+K+F+J/jp4G34r8W4n+GBwNP578e4n+GrwY+iv96iP9+x4GnA8f5r4f47/fewHfx3wPx3++vgJfmvwfiv5/574P472f++yD++5n/Poj/fua/D+K/3y5wjP8eiP9+nw18Fv89fkb8z/DWwFsDD+Zf78HAg/jXuwS8tPjf76eBt+Jf5xLw2sBfi//9LgLHedFdAl4b+GsA8b/bg4Gn86K7BLw28Ndcgfjf7a2Bn+JFcwl4beCveTbE/26fDXwW/7JLwGsDf81zQvzv9tvAa/HCXQJeG/hrnhfif7eLwHFesEvAawN/zfOH+N/rwcDTecEuAa8N/DUvGOJ/r9cGfovn7xLw2sBf88Ih/vd6aeCveF6XgNcG/pp/GeJ/t98GXotnuwS8NvDXvGgQ/7sdB34beCngEvDawF/zokP873cc+Gngo4G/5l+HfwRQl5L+3DWNDwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbsUpDown;
impl IconShape for MdThumbsUpDown {
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
                d: "M12 6c0-.55-.45-1-1-1H5.82l.66-3.18.02-.23c0-.31-.13-.59-.33-.8L5.38 0 .44 4.94C.17 5.21 0 5.59 0 6v6.5c0 .83.67 1.5 1.5 1.5h6.75c.62 0 1.15-.38 1.38-.91l2.26-5.29c.07-.17.11-.36.11-.55V6zm10.5 4h-6.75c-.62 0-1.15.38-1.38.91l-2.26 5.29c-.07.17-.11.36-.11.55V18c0 .55.45 1 1 1h5.18l-.66 3.18-.02.24c0 .31.13.59.33.8l.79.78 4.94-4.94c.27-.27.44-.65.44-1.06v-6.5c0-.83-.67-1.5-1.5-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+L/pvYCPBl6aK/4a+Grge3hOiP97vgt4b56/7wbeh2dD/Nd4aeAYV/wO/3k+G/gsXrjPAT6bKxD/uV4b+C7gwTzbLvDVwOfwH+8icJwXbhc4wRWI/zzvDXwXL9h3A+/Df5zXBn6LF83rAL8NIP5zHAeeDhznhXsf4Lv5j/HawG/xonkd4LcBxH+Ojwa+in/Z7wCvzX+M1wZ+ixfNywB/DSD+c3w38F68aMS/33Hgt4CX5l/2DODBXIH4z/HTwFvxohH/PseB3wJemhfN6wC/zRWI/xw/Bbw1/7JbgZcBdvm3OQ78FvDS/MsuAe8N/DTPhviP913Ae/Oi+2vgdYBd/nWOA78FvDTP389wxS7w18B3A7s8J8R/rO8C3pt/vb8GXgfY5UVzHPgt4KV5/r4HeG/+ZYj/ON8FvDf/dn8NvA6wywt3HPgt4KV5/r4HeG9eNIj/GN8FvDfP398AH82zvTfwXjx/fw28DrDL83cc+C3gpXn+vgd4b150iH+/7wLem+fvb4DXBnZ5Tt8NvBfP318DrwPs8pyOA78FvDTP3/cA782/DuLf57uA9+b5+xvgtYFdnr/vBt6L5++vgdcBdrniOPBbwEvz/H0P8N786yH+7b4LeG+ev78BXhvY5YX7buC9eP5uBR7MFbcCD+b5+x7gvfm3QfzbfBfw3jx/fwO8NrDLi+a7gffi3+Z7gPfm3w7xr/ddwHvz/P0N8NrALv863w28F/863wO8N/8+iH+d7wLem+fvb4DXBnb5t/lu4L140XwP8N78+yFedN8FvDfP398Arw3s8m/32sBv8aJ5HeC3+fdDPH/HgfcCXho4zhVvzfP3N8BrA7v8+7w28Fu8aF4H+G3+/RDP672BrwKO8y/7G+C1gV3+/V4a+CteNC8D/DX/fojn9NrAb/Gi+RvgtYFd/uPcCjyIF+4ZwIP5j4F4Tk8HHsy/7G+A1wZ2+Y/12sBv8cK9DvDb/MdAPNtLA3/Fi+Z1gN/mP8d7A18NHOM5XQLeG/hp/uMgnu21gd/iRfM6wG/zn+c48N7AS3PFXwPfDezyHwvxbK8N/BYvmtcBfpv//RDPaRc4xgt3CTjO/w2I5/TZwGfxwn0O8Nn834B4Xt8NvBfP3/cA783/HYjn772BjwZeiit+B/hu4Lv5vwXx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EpbiSQd9o34YAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimeline;
impl IconShape for MdTimeline {
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
                d: "M23,8c0,1.1-0.9,2-2,2c-0.18,0-0.35-0.02-0.51-0.07l-3.56,3.55C16.98,13.64,17,13.82,17,14c0,1.1-0.9,2-2,2s-2-0.9-2-2 c0-0.18,0.02-0.36,0.07-0.52l-2.55-2.55C10.36,10.98,10.18,11,10,11s-0.36-0.02-0.52-0.07l-4.55,4.56C4.98,15.65,5,15.82,5,16 c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2c0.18,0,0.35,0.02,0.51,0.07l4.56-4.55C8.02,9.36,8,9.18,8,9c0-1.1,0.9-2,2-2s2,0.9,2,2 c0,0.18-0.02,0.36-0.07,0.52l2.55,2.55C14.64,12.02,14.82,12,15,12s0.36,0.02,0.52,0.07l3.55-3.56C19.02,8.35,19,8.18,19,8 c0-1.1,0.9-2,2-2S23,6.9,23,8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/2WvzP8zfALs/pOPBSPK/f4QVD/MvM/zyvA/w2z+m1gd/ieYkXDPEvM//zvA7w2zyn1wZ+i+clXjDEv8z8z/M6wG/znF4b+C2el3jBEP8y8z/P6wC/zXN6beC3eF7iBUP8y8z/PK8D/DbP6bWB3+J5iRcM8f8b4v83xP9viP/fEP+/If5lv8X/PB8D/DXP6aWBr+J5vQ4vGOJfZv7neR3gt3lOrw38Fs9LvGCIf5n5n+d1gN/mOb028Fs8L/GCIf5l5n+e1wF+m+f02sBv8bzEC4b4l5n/eV4H+G2e02sDv8XzEi8Y4l/22/zP89HAX/OcXhr4ap7Xa/OCIf5/Q/z/hvj/DfH/G+L/N8S/zPzP8zrAb/OcXhv4LZ6XeMEQ/zLzP8/rAL/Nc3pt4Ld4XuIFQ/zLzP88rwP8Ns/ptYHf4nmJFwzxLzP/87wO8Ns8p9cGfovnJV4wxL/M/M/zOsBv85xeG/gtnpd4wRD/stfmf56/BnZ5TseBl+Z5/TYvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMOxTZB40VerwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToc;
impl IconShape for MdToc {
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
                d: "M3 9h14V7H3v2zm0 4h14v-2H3v2zm0 4h14v-2H3v2zm16 0h2v-2h-2v2zm0-10v2h2V7h-2zm0 6h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpvndBx4Kf71fof/GK8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/iX0/8x3ht4Ld4XuIFQ/zLzPN6HeC3eU6vDfwW/3riP8ZrA7/F8xIvGOJfZp7X6wC/zXN6beC3+NcT/zFeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xb+e+I/x2sBv8bzEC4b4l5nn9TrAb/OcXhv4Lf71xH+M1wZ+i+clXjDEv8w8r9cBfpvn9NrAb/GvJ/5jvDbwWzwv8YIh/mXmeb0O8Ns8p9cGfot/PfEf47WB3+J5iRcM8S8zz+t1gN/mOb028Fv864n/GK8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/iX0/8x3ht4Ld4XuIFQ/zLzPN6HeC3eU6vDfwW/3riP8ZrA7/F8xIvGOJfZp7X6wC/zXN6beC3+NcT/zFeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xb+e+I/x2sBv8bzEC4b4l5nn9TrAb/OcXhv4Lf71xH+M1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM8/po4K/5n+Wlga/meYkXDPEv2wWO8b/TJeA4LxjiX/bdwHvxv9P3AO/NC4b4l7008Ff87/QQ4FZeMMSL5r2B7+J/l/cBvpsXDvGiezDw2cBbA8f4n+kS8NPAZwO38i9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BroSaQWv6XkoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToday;
impl IconShape for MdToday {
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
                d: "M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToll;
impl IconShape for MdToll {
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
                d: "M15 4c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
            path {
                d: "M3 12c0-2.61 1.67-4.83 4-5.65V4.26C3.55 5.15 1 8.27 1 12s2.55 6.85 6 7.74v-2.09c-2.33-.82-4-3.04-4-5.65z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/PSwHsBLw08GHgwV/w2cCvw08DvALv810H853tt4LOA1+Zftgt8NvA1/NdA/Of6LuC9+df7a+BtgFv5z4X4z3Ec+Cngtfm32wVeB/hr/vMg/nN8F/De/PvdCrwMsMt/DsR/vPcGvosX7GeAvwb+Gnhp4MHAWwPHeP5+G3gd/nMg/mMdB54OHOd5/Q3w1sCtPK/jwGcDH8Xz9zrAb/MfD/Ef66OBr+J5/Q7w1sAuL9xHA1/F8/pt4HX4j4f4j/VXwEvznC4BDwZ2ueK1gc8CXhv4a+Cngc/h2X4aeCue1wlgl/9YiP84x4GLPK+vAT6aK94b+C6e128Dr8MVLw38Fc/rfYDv5j8W4j/OawO/xfN6GeCvueIicJzn732A7+aKW4EH8Zw+B/hs/mMh/uO8NvBbPC9xxWsDv8UL9jXAR3PFbwOvxXP6GuCj+Y+F+I/z2sBv8bzEFa8N/BYv2O8Ar80Vvw28Fs/pa4CP5j8W4j/OSwN/xfMSV7w28Fu8YL8DvDZX/DbwWjynzwE+m/9YiP9Y5nmJK14b+C1esN8BXpsrfht4LZ7T2wA/zX8sxH+s3wZei+ckrnht4Ld4wX4HeG2u+G3gtXhODwFu5T8W4j/WewPfxXMSV7w28Fu8YL8DvDZX/DbwWjzbzwBvzX88xH+s48CtwDGeTVzx2sBv8YL9DvDaXPHbwGvxbK8D/Db/8RD/8d4a+CmeTVzx2sBv8YL9DvDaXPHbwGtxxc8Ab81/DsR/ju8G3osrxBWvDfwWL9jvAK/NFb8NvBbwDOClgV3+cyD+cxwHvhp4L0Bc8drAb/GC/Q7w2lzx28CDgbcG/pr/PIj/XF8NfDRXvDbwW7xgvwO8Nld8NfDZwC7/uRD/dV4b+C1esN8BXpv/Woj/Oq8N/BYv2O8Ar81/LcR/ndcGfosX7HeA1+a/FuK/zmsDv8UL9jvAa/NfC/Ff57WB3+IF+x3gtfmvhfiv89rAb/GC/Q7w2vzXQvzXeW3gt3jR3Ap8N/A1wC7/eRD/dV4b+C3+dXaBrwa+BtjlPx7iv85rA7/Fv80u8NXA1wC7/MdB/Nd5beC3+PfZBb4a+Bpgl38/xH+d1wZ+i/8Yu8BXA18D7PJvh/iv89rAb/Efaxf4auBrgF3+9RD/dV4b+C3+c+wCXw18DbDLiw7xX+e1gd/iP9dvA6/Diw7xX+e1gd/iP9/7AN/NiwbxX+e1gd/iP9/XAB/Niwbx73MceCmu+BtglxfstYHf4j/f9wDvzYsG8W/30sBvAcd5tluBvwb+Gvht4Hd4ttcGfov/fJ8DfDYvGsS/zUsDvwUc51/218BfA7vAR/Of722An+ZFg/jXOw48HTjO/0yvA/w2LxrEv817A9/F/0ziRYf4t3tv4Lv4n+UScJwXHeLf572B7+J/jt8BXpsXHeLf772B7+J/hu8B3psXHeI/xnsD38V/v88BPpsXHeI/znsD38V/r7cBfpoXHeI/1nsD38V/n9cBfpsXHeI/3nsD38V/D/Gvg/jP8d7Ad/Ff6xJwnH8dxH+e9wa+i/86vwO8Nv86iP9c7w18F/81vgd4b/51EP/53hv4Lv7zvQ3w0/zrIP5rvDfwXfzn+R3gtfnXQ/zXeWvgs4GX4j/OM4DvBr4a2OVfj38EodPIQZQZfVUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTouchApp;
impl IconShape for MdTouchApp {
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
                d: "M9,11.24V7.5C9,6.12,10.12,5,11.5,5S14,6.12,14,7.5v3.74c1.21-0.81,2-2.18,2-3.74C16,5.01,13.99,3,11.5,3S7,5.01,7,7.5 C7,9.06,7.79,10.43,9,11.24z M18.84,15.87l-4.54-2.26c-0.17-0.07-0.35-0.11-0.54-0.11H13v-6C13,6.67,12.33,6,11.5,6 S10,6.67,10,7.5v10.74c-3.6-0.76-3.54-0.75-3.67-0.75c-0.31,0-0.59,0.13-0.79,0.33l-0.79,0.8l4.94,4.94 C9.96,23.83,10.34,24,10.75,24h6.79c0.75,0,1.33-0.55,1.44-1.28l0.75-5.27c0.01-0.07,0.02-0.14,0.02-0.2 C19.75,16.63,19.37,16.09,18.84,15.87z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Nl8NvBTP6W+Aj+Z/F8S/zW8Dr8Vz+h3gtfnfBfFv89vAa/Gcfgd4bf53Qfzb/DbwWjyn3wFem/9dEP82vw28Fs/pd4DX5n8XxL/NbwOvxXP6a+Cj+Z/nd3jBEP82vw28Fv/zfQ/w3rxgiH+b3wZei//5Xgf4bV4wxL/NbwOvxf9szwAezAuH+Lf5beC1+J/tY4Cv5oVD/Nv8NvBa/M92AtjlhUP82/w28Fr8z/U9wHvzL0P82/w28Fr8z/U6wG/zL0P82/w28Fr8x/kbYJcrXot/n2cAD+ZFg/i3+W3gtfj3+x3gvYFbebbjwEcDn8W/zccAX82LBvFv89vAa/Hv8z3Ae/OCvTfwXfzrnQB2edEg/m1+G3gt/u0uAQ8Gdnnhvht4L1503wO8Ny86xL/NbwOvxb/d1wAfzb/stYHf4kX3OsBv86JD/Nv8NvBa/Nu9D/DdvGjMi+YZwIP510H82/w28Fr8270P8N28aMyL5lbgIfzrIP5tfht4Lf7tvgb4aP5lrw38Fi+61wF+mxcd4t/mt4HX4t9uF3gIsMsL993Ae/Gi+x7gvXnRIf5tfht4Lf59vht4H16w9wa+i3+9E8AuLxrEv81vA6/Fv99vA+8D3MqzHQc+Cvhs/m0+BvhqXjSIf5vfBl6L/zh/DexyxWvz73Mr8BBeNIh/m98GXov/uV4H+G3+ZYh/m98GXov/ub4HeG/+ZYh/m98GXov/2U4Au7xwiH+b3wZei//ZPgb4al44xL/NbwOvxf9stwIP4YVD/Nv8NvBa/M/3OsBv84Ih/m1+G3gt/uf7HuC9ecEQ/za/DbwWz+lvgI/mf57f5gVD/Nv8NvBaPKffAV6b/10Q/za/DbwWz+l3gNfmfxfEv81vA6/Fc/od4LX53wXxb/PbwGvxnH4HeG3+d0H82/w28Fo8p98BXpv/XRD/Nr8NvBbP6XeA1+Z/F8S/zW8Dr8Vz+h3gtfnfBfFv89vAa/Gcfgd4bf53Qfzb/DbwWjyn3wFem/9dEP82vw28Fs/pd4DX5n8XxL/NbwOvxXP6HeC1+d8F8W/z28Br8Zx+B3ht/ndB/Nv8NvBaPKffAV6b/10Q/za/DbwWz+l3gNfmfxfEv81vA6/Fc/od4LX53wXxb/PbwGvxnH4HeG3+d0H82/w28Fo8p98BXpv/XRD/Nr8NvBbP6XeA1+Z/F8S/zW8Dr8Vz+h3gtfnfBfFv89vAa/Gcfgd4bf53QfzbfDXw0jynvwY+mv9dEP+/If5/Q/z/hvj/DfH/G/8Ig8uKQVDmT9wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTour;
impl IconShape for MdTour {
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
                d: "M21 4H7V2H5v20h2v-8h14l-2-5 2-5zm-6 5c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTrackChanges;
impl IconShape for MdTrackChanges {
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
                d: "M19.07 4.93l-1.41 1.41C19.1 7.79 20 9.79 20 12c0 4.42-3.58 8-8 8s-8-3.58-8-8c0-4.08 3.05-7.44 7-7.93v2.02C8.16 6.57 6 9.03 6 12c0 3.31 2.69 6 6 6s6-2.69 6-6c0-1.66-.67-3.16-1.76-4.24l-1.41 1.41C15.55 9.9 16 10.9 16 12c0 2.21-1.79 4-4 4s-4-1.79-4-4c0-1.86 1.28-3.41 3-3.86v2.14c-.6.35-1 .98-1 1.72 0 1.1.9 2 2 2s2-.9 2-2c0-.74-.4-1.38-1-1.72V2h-1C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10c0-2.76-1.12-5.26-2.93-7.07z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTranslate;
impl IconShape for MdTranslate {
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
                d: "M12.87 15.07l-2.54-2.51.03-.03c1.74-1.94 2.98-4.17 3.71-6.53H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEW0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Je9N/BawPvwfw/ihXtv4Lu44ruB9+H/FsQL9t7Ad/Gcvht4H/7vQDx/bw38FM/fdwPvw/8NiOfvOPDbwEvx/H038D7874d4wY4Dvw28FM/fdwPvw3+stwZ+G9jlvwbihTsO/DbwUjx/3w28D/8x3hv4LuCvgdcBdvnPh/iXHQd+G3gpnr/vBt6Hf5/3Br6LZ/tr4HWAXf5zIV40x4HfBl6K5++7gffh3+a9ge/ief018DrALv95EC+648BvAy/F8/fdwPvwr/PewHfxgv018DrALv85EP86x4HfBl6K5++7gffhRfPewHfxL/tr4HWAXf7jIf71jgO/DbwUz993A+/DC/fewHfxovtr4HWAXf5jIf5tjgO/DbwUz993A+/D8/fewHfxr/fXwOsAu/zHQfzbHQd+G3gpnr/vBt6H5/TewHfx/F0CXhv4aOC9eP7+GngdYJf/GIh/n+PAbwMvxfP33cD7cMV7A9/F83cJeG3gr7niu4H34vn7a+B1gF3+/RD/fseB3wZeiufvu4HfAb6L5+8S8NrAX/Ocvht4L56/vwZeB9jl3wfxH+M48NvAS/Gvcwl4beCvef6+G3gvnr+/Bl4H2OVfdhz4bOCjeU6I/zjHgd8GXooXzSXgtYG/5oX7buC9eP7+GngdYJcX7DjwW8Al4LV5Toj/WMeB3wZeihfuEvDawF/zovlu4L14/v4aeB1gl+d1HPgt4KWB3wFem+eE+I93HPht4KV4/i4Brw38Nf863w28F8/fXwOvA+zybMeB3wJemit+B3htnhPiP8dx4LeBl+I5XQJeG/hr/m2+G3gvnr+/Bl4H2AWOA78FvDTP9jvAa/OcEP95jgO/DbwUV1wCXhv4a/59vht4L56/vwbeBvgp4KV5Tr8DvDbPCfGf6zjw28CDgdcG/pr/GN8NvBf/Or8DvDbPCfGf7zjwYOCv+Y/13cB78aL7HeC1eU6I/92+G3gvXjS/A7w2zwnxv993A+/Fv+x3gNfmOSH+dzsO/Bbw0vzLfgd4bZ4T4n+v48BvAS/Ni+Z3gNfmOSH+dzoO/Bbw0rzofgd4bZ4T4n+f48BvAS/Nv87vAK/Nc0L873Ic+C3gpfnX+x3gtXlOiP89jgO/Bbw0/za/A7w2zwnx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EJ1aMQdMJBv4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTrendingDown;
impl IconShape for MdTrendingDown {
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
                d: "M16 18l2.29-2.29-4.88-4.88-4 4L2 7.41 3.41 6l6 6 4-4 6.3 6.29L22 12v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP/7vTTw1/zbIP73uwi8DvDX/Osh/vczsAu8DvDX/Osg/vczV+wCrwP8NS86xP9+5tl2gdcB/poXDeJ/P/OcdoHXAf6afxnifz/zvHaB1wH+mhcO8b+fef52gdcB/poXDPEv+y3+Z3ttXrBd4HWAv+b5Q/zLzP9uu8DrAH/N80L8y8z/frvA6wB/zXNC/MvM/36XgNcG/prnhPiXmf/dLgGvDfw1zwvxL/tt/md7LV6wS8BrA3/N84f43888f5eA1wb+mhcM8b+feV6XgNcG/poXDvG/n3lOl4DXBv6afxnifz/zbJeA1wb+mhcN4n8/c8Ul4LWBv+ZFh/jfz8Al4LWBv+ZfB/G/3y7w2sBf86+H+N/vpYG/5t8G8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj4WwPEF7SkjTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTrendingFlat;
impl IconShape for MdTrendingFlat {
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
                d: "M22 12l-4-4v3H3v2h15v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+N/jOPDbwEvxb/M7wGvznBD/uxwHfht4Kf71fgd4bZ4T4n+f48BvAy/Fv87vAK/Nc0L873Qc+G3gpXjR/Q7w2jwnxP9ex4HfBl6KF83vAK/Nc0L873Yc+G3gpfiX/Q7w2jwnxP9+3wW8N/+y3wFem+eE+N/tu4D35kXzO8Br85wQ//mOAw8G/pr/WN8FvDcvut8BXpvnhPjPdRz4LeDBwOsAf81/jO8C3pt/nd8BXpvnhPjPcxz4LeCluWIXeB3gr/n3+S7gvXn+/gZ4a+CngZfiOf0O8No8J8R/juPAbwEvzXPaBV4H+Gv+bb4LeG+ev78BXhvYBY4Dvw28FM/2O8Br85wQ//GOA78FvDTP3y7wOsBf86/zXcB78/z9DfDawC7Pdhz4beCluOJ3gNfmOSH+Yx0Hfgt4aV64XeB1gL/mRfNdwHvz/P0N8NrALs/rOPDbwEsBvwO8Ns8J8R/nOPBbwEvzotkFXgf4a1647wLem+fvb4DXBnZ5wY4Dvw3sAq/Nc0L8xzgO/Bbw0vzr7AKvA/w1z993Ae/N8/c3wGsDu/zLjgOfDXw0zwnx73cc+C3gpXn+vgf4beC7eP52gdcB/prn9F3Ae/P8/Q3w2sAu/z6If5/jwG8BL83z9z3Ae3PFewPfxfO3C7wO8Ndc8V3Ae/P8/Q3w2sAu/36If7vjwG8BL83z9z3Ae/Oc3hv4Lp6/XeB1gI8C3pvn72+A1wZ2+Y+B+Lc5DvwW8NI8f98DvDfP33sD38W/3t8Arw3s8h8H8a93HPgt4KV5/r4HeG9euPcGvosX3d8Arw3s8h8L8a9zHPgt4KV5/r4HeG9eNO8NfBf/sr8BXhvY5T8e4kV3HPgt4KV5/r4HeG/+dd4b+C5esL8BXhvY5T8H4kVzHPgt4KV5/r4HeG/+bd4b+C6e198Arw3s8p8H8S87DvwW8NI8f98DvDf/Pu8NfBfP9jfAawO7/OdCvHDHgd8CXprn73uA9+Y/xnsD3wX8DfDawC7/+RAv2HHgt4CX5vn7HuC9+Y/11sBvA7v810A8f8eB3wJemufve4D35n8/xPP31sBP8fx9D/De/N+AeMHeG/guntP3AO/N/x2IF+69ge/iiu8B3pv/WxD/svcGXht4b/7vQfz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHKI4xBMxeMcgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTrendingUp;
impl IconShape for MdTrendingUp {
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
                d: "M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAELUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zkOfBTw3sCD+a91K/DdwNcAu/zbIP7tHgz8FPDS/Pf6a+BtgFv510P82/0V8NL8z/DXwMvwr4f4t3lr4Kf4n+VtgJ/mXwfxb/PZwGfxP8vnAJ/Nvw7i3+angbfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv8+3wN8NXAceG/gvfj3+R3gtfnXQfzb/DbwWvzrPQP4buCrgV2e04OB9wY+GjjGv97vAK/Nvw7i3+a3gdfiRfc7wHcD382L5r2BjwZeihfd7wCvzb8O4t/mt4HX4l/2PcBXA3/Nv81rA+8NvBf/st8BXpt/HcS/zW8Dr8Xz9wzgu4GvBnb5j/Fg4L2BjwaO8fz9DvDa/Osg/m1+G3gtntPvAN8NfDf/ud4b+GjgpXhOvwO8Nv86iH+b3wZeiyu+B/hq4K/5r/XawHsD78UVvwO8Nv86iH+b7wZuBb4a2OW/14OB9wZeGnhr/nUQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Efxg3tBbE6+8gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTurnedIn;
impl IconShape for MdTurnedIn {
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTurnedInNot;
impl IconShape for MdTurnedInNot {
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUnpublished;
impl IconShape for MdUnpublished {
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
                d: "M21.19,21.19L2.81,2.81L1.39,4.22l2.27,2.27C2.61,8.07,2,9.96,2,12c0,5.52,4.48,10,10,10c2.04,0,3.93-0.61,5.51-1.66 l2.27,2.27L21.19,21.19z M10.59,16.6l-4.24-4.24l1.41-1.41l2.83,2.83l0.18-0.18l1.41,1.41L10.59,16.6z M13.59,10.76l-7.1-7.1 C8.07,2.61,9.96,2,12,2c5.52,0,10,4.48,10,10c0,2.04-0.61,3.93-1.66,5.51l-5.34-5.34l2.65-2.65l-1.41-1.41L13.59,10.76z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUpdate;
impl IconShape for MdUpdate {
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
                d: "M21,10.12h-6.78l2.74-2.82c-2.73-2.7-7.15-2.8-9.88-0.1c-2.73,2.71-2.73,7.08,0,9.79s7.15,2.71,9.88,0 C18.32,15.65,19,14.08,19,12.1h2c0,1.98-0.88,4.55-2.64,6.29c-3.51,3.48-9.21,3.48-12.72,0c-3.5-3.47-3.53-9.11-0.02-12.58 s9.14-3.47,12.65,0L21,3V10.12z M12.5,8v4.25l3.5,2.08l-0.72,1.21L11,13V8H12.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADVklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9dxrtjlvwfiv89x4Le44nWAXf7rIf57HAd+C3hprvhr4HWAXf5rIf7rHQd+C3hpntNfA68D7PJfB/Ff6zjwW8BL8/z9NfA6wC7/NRD/dY4DvwW8NC/cXwOvA+zynw/xX+M48FvAS/Oi+WvgdYBd/nMh/vMdB34LeGn+df4aeB1gl/88iP9cx4HfAl6af5u/Bl4H2OU/B+I/z3Hgt4CX5t/nr4HXAXb5j4f4z3Ec+C3gpfmP8dfA6wC7/MdC/Od4MPBgXrCvBl6K5/Q3wEfzgt0K3Mp/LMR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4+vBl6a5/TXwEfzXwvx/xvi/zfE/2+I/98Q/78h/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cvw28Fv+xfgd4bf5jIf5z/DbwWvzH+h3gtfmPhfjP8dLAcf5j7QJ/zX8sxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHu+RYQWKvLKoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUpgrade;
impl IconShape for MdUpgrade {
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
                d: "M16,18v2H8v-2H16z M11,7.99V16h2V7.99h3L12,4L8,7.99H11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerified;
impl IconShape for MdVerified {
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
                d: "M23,12l-2.44-2.79l0.34-3.69l-3.61-0.82L15.4,1.5L12,2.96L8.6,1.5L6.71,4.69L3.1,5.5L3.44,9.2L1,12l2.44,2.79l-0.34,3.7 l3.61,0.82L8.6,22.5l3.4-1.47l3.4,1.46l1.89-3.19l3.61-0.82l-0.34-3.69L23,12z M10.09,16.72l-3.8-3.81l1.48-1.48l2.32,2.33 l5.85-5.87l1.48,1.48L10.09,16.72z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3ht/nP9NvDdwPfw74f49zkOvBfw0cCD+a91K/DVwPcAu/zbIP5tHgx8FPDewHH+e+0C3w18DXAr/zqIf53XBt4LeG/+Z/pu4HuA3+ZFg3jRvBfw3sBr87/DbwPfDXwPLxziX/Z04MH873Qr8BBeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzP8TfAdwNfxYtOvGCIf5n5n+FvgNcGdoH3Br6LF414wRD/MvPf72+A1wZ2ebb3Br6Lf5l4wRD/MvPf62+A1wZ2eV6fDXwWL5x4wRD/MvPf52+A1wZ2ef6+C3hvXjjxgiH+Zea/x98Arw3s8vx9F/De/MvEC4b4l5n/en8DvDawy/P3XcB786IRLxjiX2b+a/0N8NrALs/fdwHvzYtOvGCIf5n5r/M3wGsDuzx/3wW8N/864gVD/MvMf42/AV4b2OX5+y7gvfnXEy8Y4l9m/vP9DfDawC7P33cB782/jXjBEP8y86K5BLw28NHAe/Gi+xvgtYFdnr/vAt6bfzvxgiH+ZX8NvBQv3CXgtYG/5orvBt6Lf9nfAK8N7PL8fRfw3vzb/Q7w2rxgiH/ZbwOvxQt2CXht4K95Tt8NvBcv2N8Arw3s8vx9F/De/Pv8DvDavGCIf9lnA5/FC/Y6wG/z/H038F48r78BXhvY5fn7LuC9+ff7HOCzecEQ/7L3Br6LF+yvgdcBdnn+vht4L57tb4DXBnZ5/r4LeG/+Y7wP8N28YIh/2UsDf8UL99fA6wC7PH/fDbwX8DfAawO7PH/fBbw3/3FeBvhrXjDEi2YXOMYL99fA6wC7PH+fDXw1sMvz913Ae/Mf5xJwnBcO8aL5aeCt+Jf9NfA6wC7/Ot8FvDf/sb4HeG9eOMSL5r2B7+JF89fA6wC7vGi+C3hv/uO9D/DdvHCIF81x4CIvur8GXgfY5YX7LuC9+c9xAtjlhUO86L4beC9edH8NvA6wy/P3XcB785/je4D35l+GeNG9NvBb/Ov8NfA6wC7P6buA9+Y/z+sAv82/DPGv89vAa/Gv89fA6wC7XPFdwHvzn+d3gNfmRYP413lr4Kf41/tr4HWArwLem/9cbwP8NC8axL/ebwOvxb/eLnCc/1y/A7w2LzrEv95rA7/F/0yvA/w2LzrEv81XAx/F/yxfA3w0/zqIf5vjwF8DD+J/hmcALw3s8q+D+Ld7beC3+J/hdYDf5l8P8e/z0cBX8d/rY4Cv5t8G8e/33cB78d/je4D35t8O8R/ju4H34r/W9wDvzb8P4j/GceCngdfiv8bvAK/Nvx/iP9Z3A+/Ff67vAd6b/xiI/3ifDXwW/zk+B/hs/uMg/nO8NfDdwDH+Y1wC3hr4bf5jIf7zHAe+G3gr/n1+BnhvYJf/eIj/fK8NfDfwIP51ngG8N/Db/OdB/Nd5b+CjgZfihfsb4KuB7+Y/H+K/3ksDHw28NvAgrngG8NvAVwN/zX8dxH+vl+aKv+a/B+L/N8T/b/wjA/HVQYTKOaUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerifiedUser;
impl IconShape for MdVerifiedUser {
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
                d: "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADdElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7Lf4j/exwB/zXN6aeCr+I/3OrxgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviX/Tb/8T4a+Gue02sDv8V/PPGCIf7neG3gt/iPJ14wxP8crw38Fv/xxAuG+J/jtYHf4j+eeMEQ/3O8NvBb/McTLxjif47XBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXvTb/8f4a2OU5vTbwW/zHEy8Y4n+O1wZ+i/944gVD/M/x2sBv8R9PvGCI/zleG/gt/uOJFwzxP8drA7/FfzzxgiH+Za/Ff7y/AXZ5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8T/HawO/xX888YIh/ud4beC3+I8nXjDE/xyvDfwW//HEC4b4n+O1gd/iP554wRD/c7w28Fv8xxMvGOJf9lv8x/sY4K95Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/2W/zH++jgb/mOb008NX8x3ttXjDE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8Cwg9WQT70i3oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalSplit;
impl IconShape for MdVerticalSplit {
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
                d: "M3 15h8v-2H3v2zm0 4h8v-2H3v2zm0-8h8V9H3v2zm0-6v2h8V5H3zm10 0h8v14h-8V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC+UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4wT4L+GjgOP87fTfwMcAuLxji+Xtr4Kf43+97gPfmBUM8f98NvBf/N4gXDPH8/TbwWvzfIF4wxPP328Br8X+DeMEQz99vA6/F/w3iBUM8f78NvBb/N4gXDPH8/TbwWvzfIF4wxPP328Br8X+DeMEQz99vA6/F/w3iBUM8f78NvBb/N4gXDPH8/TbwWvzfIF4wxPP328Br8X+DeMEQz99vA6/F/w3iBUM8f78NvBb/N4gXDPH8/TbwWvzfIF4wxPP328Br8X+DeMEQz99vA6/F/w3iBUM8f78NvBb/N4gXDPH8fTTwVfzv9zfAS/OCIZ6/48BvAy/F/16XgNcG/poXDPHCvTb/e/01sMsLh/j/DfH/G+L/N8T/b4gX7rX43+tvgF1eOMTzdxz4LeCl+d9rF3gd4K95wRDP30cDX8X/fn8NvAwvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4Z4/n4beC3+bxAvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4Z4/n4beC3+bxAvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4Z4/n4beC3+bxAvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4Z4/n4beC3+bxAvGOL5+27gvfi/QbxgiOfvrYGf4n+/7wHemxcM8YJ9NvDewIP43+l7gI8GdnnBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHQBk6QZWwl+kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewAgenda;
impl IconShape for MdViewAgenda {
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
                d: "M20 13H3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h17c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zm0-10H3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h17c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6KF+xvgI/miq8GXor/PB8D/DVX/BYv3OvwgiFedK8N/BYv2O8Ar80Vvw28Fv95Xgf4ba4wL5x4wRAvutcGfosX7HeA1+aK3wZei/88rwP8NleYF068YIgX3WsDv8UL9jvAa3PFbwOvxX+e1wF+myvMCydeMMSL7rWB3+IF+x3gtbnit4HX4j/P6wC/zRXmhRMvGOJF99rAb/GC/Q7w2lzx28Br8Z/ndYDf5grzwokXDPGie23gt3jBfgd4ba74beC1+M/zOsBvc4V54cQLhnjRvTbwW7xgvwO8Nlf8NvBa/Od5HeC3ucK8cOIFQ7zoXhv4LV6w3wFemyt+G3gt/vO8DvDbXGFeOPGCIV50rw38Fi/Y7wCvzRW/DbwW/3leB/htrjAvnHjBEC+61wZ+ixfsd4DX5orfBl6L/zyvA/w2V5gXTrxgiBfdawO/xQv2O8Brc8VvA6/Ff57XAX6bK8wLJ14wxIvutYHf4gX7HeC1ueK3gdfiP8/rAL/NFeaFEy8Y4kX32sBv8YL9DvDaXPHbwGvxn+d1gN/mCvPCiRcM8aJ7beC3eMF+B3htrvht4LX4z/M6wG9zhXnhxAuGeNG9NvBbvGC/A7w2V/w28Fr853kd4Le5wrxw4gVDvOheG/gtXrDfAV6bK34beC3+87wO8NtcYV448YIhXnSvDfwWL9jvAK/NFb8NvBb/eV4H+G2uMC+ceMEQL7rXBn6LF+x3gNfmit8GXov/PK8D/DZXmBdOvGCIF91rA7/FC/Y7wGtzxW8Dr8V/ntcBfpsrzAsnXjDEi+61gd/iBfsd4LW54reB1+I/z+sAv80V5oUTLxjiRffawG/xgv0O8Npc8dvAa/Gf53WA3+YK88KJFwzxontt4Ld4wX4HeG2u+G3gtfjP8zrAb3OFeeHEC4Z40b028Fu8YL8DvDZX/DbwWvzneR3gt7nCvHDiBUO86F4b+C1esN8BXpsrfht4Lf7zvA7w21xhXjjxgiFedK8N/BYv2O8Ar80Vvw28Fv95Xgf4ba4wL5x4wRAvutcGfosX7HeA1+aK3wZei/88rwP8NleYF068YIgX3WsDv8UL9jvAa3PFbwOvxX+e1wF+myvMCydeMMSL7rWB3+IF+x3gtbnit4HX4j/P6wC/zRXmhRMvGOJF99rAb/GC/Q7w2lzx28Br8Z/ndYDf5grzwokXDPGie23gt3jBfgd4ba74beC1+M/zOsBvc4V54cQLhnjRvTbwW7xgvwO8Nlf8NvBa/Od5HeC3ucK8cOIFQ7zoXhv4LV6w3wFemyt+G3gt/vO8DvDbXGFeOPGCIV50rw38Fi/Y7wCvzRW/DbwW/3leB/htrjAvnHjBEC+61wZ+ixfsd4DX5orfBl6L/zyvA/w2V5gXTrxgiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I0dbr0E8sPiDAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewArray;
impl IconShape for MdViewArray {
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
                d: "M4 18h3V5H4v13zM18 5v13h3V5h-3zM8 18h9V5H8v13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP85x4KX4r/E3wC7/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/x/P028Fq8aMQVrw38Fv81Xgf4ba4wL5rfAV6b54R4/n4beC1eNOKK1wZ+i/8arwP8NleYF83vAK/Nc0I8f78NvBYvGnHFawO/xX+N1wF+myvMi+Z3gNfmOSGev98GXosXjbjitYHf4r/G6wC/zRXmRfM7wGvznBDP328Dr8WLRlzx2sBv8V/jdYDf5grzovkd4LV5Tojn77eB1+JFI654beC3+K/xOsBvc4V50fwO8No8J8Tz99vAa/GiEVe8NvBb/Nd4HeC3ucK8aH4HeG2eE+L5+23gtXjRiCteG/gt/mu8DvDbXGFeNL8DvDbPCfH8/TbwWrxoxBWvDfwW/zVeB/htrjAvmt8BXpvnhHj+fht4LV404orXBn6L/xqvA/w2V5gXze8Ar81zQjx/vw28Fi8accVrA7/Ff43XAX6bK8yL5neA1+Y5IZ6/3wZeixeNuOK1gd/iv8brAL/NFeZF8zvAa/OcEM/fbwOvxYtGXPHawG/xX+N1gN/mCvOi+R3gtXlOiOfvt4HX4kUjrnht4Lf4r/E6wG9zhXnR/A7w2jwnxPP328Br8aIRV7w28Fv813gd4Le5wrxofgd4bZ4T4vn7beC1eNGIK14b+C3+a7wO8NtcYV40vwO8Ns8J8fz9NvBavGjEFa8N/Bb/NV4H+G2uMC+a3wFem+eEeP5+G3gtXjTiitcGfov/Gq8D/DZXmBfN7wCvzXNCPH+/DbwWLxpxxWsDv8V/jdcBfpsrzIvmd4DX5jkhnr/fBl6LF4244rWB3+K/xusAv80V5kXzO8Br85wQz99vA6/Fi0Zc8drAb/Ff43WA3+YK86L5HeC1eU6I5++3gdfiRSOueG3gt/iv8TrAb3OFedH8DvDaPCfE8/fbwGvxohFXvDbwW/zXeB3gt7nCvGh+B3htnhPi+ftt4LV40YgrXhv4Lf5rvA7w21xhXjS/A7w2zwnx/P028Fq8aMQVrw38Fv81Xgf4ba4wL5rfAV6b54R4/n4beC1eNOKK1wZ+i/8arwP8NleYF83vAK/Nc0I8f78NvBYvGnHFawO/xX+N1wF+myvMi+Z3gNfmOSGev98GXosXjbjitYHf4r/G6wC/zRXmRfM7wGvznBDP328Dr8WLRlzx2sBv8V/jdYDf5grzovkd4LV5Tojn76WB47xofpsrXhv4Lf5rvA7w21zx2rxodoG/5jkh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OSwNfzX+Njwb+mn8/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AlnGrEHBE6qWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewCarousel;
impl IconShape for MdViewCarousel {
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
                d: "M7 19h10V4H7v15zm-5-2h4V6H2v11zM18 6v11h4V6h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6Kf73X4YqXBr6Kf73X4YqXBr6Kf73X4QVDvOheG/gt/vXEFa8N/Bb/euKK1wZ+i3898YIhXnSvDfwW/3riitcGfot/PXHFawO/xb+eeMEQL7rXBn6Lfz1xxWsDv8W/nrjitYHf4l9PvGCIF91rA7/Fv5644rWB3+JfT1zx2sBv8a8nXjDEi+61gd/iX09c8drAb/GvJ654beC3+NcTLxjiRffawG/xryeueG3gt/jXE1e8NvBb/OuJFwzxontt4Lf41xNXvDbwW/zriSteG/gt/vXEC4Z40b028Fv864krXhv4Lf71xBWvDfwW/3riBUO86F4b+C3+9cQVrw38Fv964orXBn6Lfz3xgiFedK8N/Bb/euKK1wZ+i389ccVrA7/Fv554wRAvutcGfot/PXHFawO/xb+euOK1gd/iX0+8YIgX3WsDv8W/nrjitYHf4l9PXPHawG/xrydeMMSL7rWB3+JfT1zx2sBv8a8nrnht4Lf41xMvGOJF99rAb/GvJ654beC3+NcTV7w28Fv864kXDPGie23gt/jXE1e8NvBb/OuJK14b+C3+9cQLhnjRvTbwW/zriSteG/gt/vXEFa8N/Bb/euIFQ7zoXhv4Lf71xBWvDfwW/3riitcGfot/PfGCIV50rw38Fv964orXBn6Lfz1xxWsDv8W/nnjBEC+61wZ+i389ccVrA7/Fv5644rWB3+JfT7xgiBfdawO/xb+euOK1gd/iX09c8drAb/GvJ14wxIvutYHf4l9PXPHawG/xryeueG3gt/jXEy8Y4kX32sBv8a8nrnht4Lf41xNXvDbwW/zriRcM8aJ7beC3+NcTV7w28Fv864krXhv4Lf71xAuGeNG9NvBb/OuJK14b+C3+9cQVrw38Fv964gVDvOheG/gt/vXEFa8N/Bb/euKK1wZ+i3898YIhXnSvDfwW/3riitcGfot/PXHFawO/xb+eeMEQL7rXBn6Lfz1xxWsDv8W/nrjitYHf4l9PvGCIF91rA7/Fv5644rWB3+JfT1zx2sBv8a8nXjDEi+61gd/iX09c8drAb/GvJ654beC3+NcTLxjiRffawG/xryeueG3gt/jXE1e8NvBb/OuJFwzxontt4Lf41xNXvDbwW/zriSteG/gt/vXEC4Z40b028Fv864krXhv4Lf71xBWvDfwW/3riBUO86F4b+C3+9cQVrw38Fv964orXBn6Lfz3xgiFedK8N/Bb/euKK1wZ+i389ccVrA7/Fv554wRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EdmIWlBwv1UbwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewColumn;
impl IconShape for MdViewColumn {
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
                d: "M10 18h5V5h-5v13zm-6 0h5V5H4v13zM16 5v13h5V5h-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACuklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/n4beC3+bxAvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4Z4/n4beC3+bxAvGOL5+23gtfi/QbxgiOfvt4HX4v8G8YIhnr/fBl6L/xvEC4b4/w3x/xvi/zfE/2+I/98QL9hLA+8FvDT/+9wK/Azw07xwiOfvpYG/4n+/zwE+mxcM8fx9NfBR/O93K/AQXjDE8/fbwGvxf4N4wRDP328Dr8X/DeIFQzx/vw28Fv83iBcM8fz9NvBa/N8gXjDE8/fbwGvxf4N4wRDP328Dr8X/DeIFQzx/vw28Fv83iBcM8fz9NvBa/N8gXjDE8/fbwGvxf4N4wRDP328Dr8X/DeIFQzx/vw28Fv83iBcM8fz9NvBa/N8gXjDE8/fbwGvxf4N4wRDP328Dr8X/DeIFQzx/vw28Fv83iBcM8fz9NvBa/N8gXjDE8/fVwEfxv98zgAfzgiGev5cG/or//T4H+GxeMMQL9tLAewMvzf8+twI/Dfw0Lxzi/zfE/2+I/98Q/78h/n9DPH+/DbwW/zeIFwzx/P028Fr83yBeMMTz99vAa/F/g3jBEM/fbwOvxf8N4gVDPH+/DbwW/zeIFwzx/P028Fr83yBeMMTz99vAa/F/g3jBEM/fbwOvxf8N4gVD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BXtYuQQFMOMUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewDay;
impl IconShape for MdViewDay {
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
                d: "M2 21h19v-3H2v3zM20 8H3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h17c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zM2 3v3h19V3H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6K/x0+Bvhr/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b008NX87/DRwF/zL0P8/4b4/w3x/xvi/zfE/2+IF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86I4DL83/Dn8N7PIvQ/z/hvj/DfH/G+L/N8SL7jjwUvzv8DfALv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+Zch/n9D/P+G+P8N8f8b4v83xIvupYGv4n+HjwH+mn8Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnQvDXw1/zt8NPDX/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8INEwwQcRn3sIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewHeadline;
impl IconShape for MdViewHeadline {
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
                d: "M4 15h16v-2H4v2zm0 4h16v-2H4v2zm0-8h16V9H4v2zm0-6v2h16V5H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewInAr;
impl IconShape for MdViewInAr {
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
                d: "M18.25 7.6l-5.5-3.18c-.46-.27-1.04-.27-1.5 0L5.75 7.6c-.46.27-.75.76-.75 1.3v6.35c0 .54.29 1.03.75 1.3l5.5 3.18c.46.27 1.04.27 1.5 0l5.5-3.18c.46-.27.75-.76.75-1.3V8.9c0-.54-.29-1.03-.75-1.3zM7 14.96v-4.62l4 2.32v4.61l-4-2.31zm5-4.03L8 8.61l4-2.31 4 2.31-4 2.32zm1 6.34v-4.61l4-2.32v4.62l-4 2.31zM7 2H3.5C2.67 2 2 2.67 2 3.5V7h2V4h3V2zm10 0h3.5c.83 0 1.5.67 1.5 1.5V7h-2V4h-3V2zM7 22H3.5c-.83 0-1.5-.67-1.5-1.5V17h2v3h3v2zm10 0h3.5c.83 0 1.5-.67 1.5-1.5V17h-2v3h-3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6KF83HAH/NFb/Ff6/X4QVDvOheG/gtXjSvA/w2V5j/XuIFQ7zoXhv4LV40rwP8NleY/17iBUO86F4b+C1eNK8D/DZXmP9e4gVDvOheG/gtXjSvA/w2V5j/XuIFQ7zoXhv4LV40rwP8NleY/17iBUO86F4b+C1eNK8D/DZXmP9e4gVDvOheG/gtXjSvA/w2V5j/XuIFQ7zoXhv4LV40rwP8NleY/17iBUO86F4b+C1eNK8D/DZXmP9e4gVDvOheG/gtXjSvA/w2V5j/XuIFQ/z/hvj/DfGiOw68FC+avwF2ueK1+O/1O7xgiBfdawO/xYvmdYDf5grz30u8YIgX3WsDv8WL5nWA3+YK899LvGCIF91rA7/Fi+Z1gN/mCvPfS7xgiBfdawO/xYvmdYDf5grz30u8YIgX3WsDv8WL5nWA3+YK899LvGCIF91rA7/Fi+Z1gN/mCvPfS7xgiBfdawO/xYvmdYDf5grz30u8YIgX3WsDv8WL5nWA3+YK899LvGCIF91rA7/Fi+Z1gN/mCvPfS7xgiBfdawO/xYvmdYDf5grz30u8YIgX3XHgpXnR/DWwyxWvzX+v3+YFQ/z/hvj/DfGie23gt3jRvA7w21xh/nuJFwzxontt4Ld40bwO8NtcYf57iRcM8aJ7beC3eNG8DvDbXGH+e4kXDPGie23gt3jRvA7w21xh/nuJFwzxontt4Ld40bwO8NtcYf57iRcM8aJ7beC3eNG8DvDbXGH+e4kXDPGie23gt3jRvA7w21xh/nuJFwzxontt4Ld40bwO8NtcYf57iRcM8aJ7beC3eNG8DvDbXGH+e4kXDPGie23gt3jRvA7w21xh/nuJFwzxontp4Kt50Xw08Ndc8dv893ptXjDE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CF/hmQbP7ETsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewList;
impl IconShape for MdViewList {
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
                d: "M4 14h4v-4H4v4zm0 5h4v-4H4v4zM4 9h4V5H4v4zm5 5h12v-4H9v4zm0 5h12v-4H9v4zM9 5v4h12V5H9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEfklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6Kf73X4YqXBr6Kf73X4YqXBr6Kf73X4QVDvOheG/gt/vXEFa8N/Bb/euKK1wZ+i3898YIhXnSvDfwW/3riitcGfot/PXHFawO/xb+eeMEQL7rXBn6Lfz1xxWsDv8W/nrjitYHf4l9PvGCIF91rA7/Fv5644rWB3+JfT1zx2sBv8a8nXjDEi+61gd/iX09c8drAb/GvJ654beC3+NcTLxjiRffawG/xryeueG3gt/jXE1e8NvBb/OuJFwzxontt4Lf41xNXvDbwW/zriSteG/gt/vXEC4Z40b028Fv864krXhv4Lf71xBWvDfwW/3riBUO86F4b+C3+9cQVrw38Fv964orXBn6Lfz3xgiFedK8N/Bb/euKK1wZ+i389ccVrA7/Fv554wRAvutcGfot/PXHFawO/xb+euOK1gd/iX0+8YIgX3WsDv8W/nrjitYHf4l9PXPHawG/xrydeMMSL7rWB3+JfT1zx2sBv8a8nrnht4Lf41xMvGOJF99rAb/GvJ654beC3+NcTV7w28Fv864kXDPGie23gt/jXE1e8NvBb/OuJK14b+C3+9cQLhnjRHQdemn+93+aK48BL86/321xxHHhp/vV+mxcM8f8b4v83xIvutYHf4l9PXPHawG/xryeueG3gt/jXEy8Y4kX32sBv8a8nrnht4Lf41xNXvDbwW/zriRcM8aJ7beC3+NcTV7w28Fv864krXhv4Lf71xAuGeNG9NvBb/OuJK14b+C3+9cQVrw38Fv964gVDvOheG/gt/vXEFa8N/Bb/euKK1wZ+i3898YIhXnSvDfwW/3riitcGfot/PXHFawO/xb+eeMEQL7rXBn6Lfz1xxWsDv8W/nrjitYHf4l9PvGCIF91rA7/Fv5644rWB3+JfT1zx2sBv8a8nXjDEi+61gd/iX09c8drAb/GvJ654beC3+NcTLxjiRffawG/xryeueG3gt/jXE1e8NvBb/OuJFwzxontt4Lf41xNXvDbwW/zriSteG/gt/vXEC4Z40b028Fv864krXhv4Lf71xBWvDfwW/3riBUO86F4b+C3+9cQVrw38Fv964orXBn6Lfz3xgiFedK8N/Bb/euKK1wZ+i389ccVrA7/Fv554wRAvutcGfot/PXHFawO/xb+euOK1gd/iX0+8YIgX3WsDv8W/nrjitYHf4l9PXPHawG/xrydeMMT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hG1/WNB9X54CQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewModule;
impl IconShape for MdViewModule {
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
                d: "M4 11h5V5H4v6zm0 7h5v-6H4v6zm6 0h5v-6h-5v6zm6 0h5v-6h-5v6zm-6-7h5V5h-5v6zm6-6v6h5V5h-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6Kf73X4YqXBr6K/3qvwwuGeNG9NvBb/OuJK14b+C3+64kXDPGie23gt/jXE1e8NvBb/NcTLxjiRffawG/xryeueG3gt/ivJ14wxIvutYHf4l9PXPHawG/xX0+8YIgX3WsDv8W/nrjitYHf4r+eeMEQL7rXBn6Lfz1xxWsDv8V/PfGCIV50rw38Fv964orXBn6L/3riBUO86F4b+C3+9cQVrw38Fv/1xAuGeNG9NvBb/OuJK14b+C3+64kXDPGie23gt/jXE1e8NvBb/NcTLxjiRffawG/xryeueG3gt/ivJ14wxIvutYHf4l9PXPHawG/xX0+8YIgX3WsDv8W/nrjitYHf4r+eeMEQL7rXBn6Lfz1xxWsDv8V/PfGCIV50rw38Fv964orXBn6L/3riBUO86F4b+C3+9cQVx4GX5r/eb/OCIV50rw38Fv964n8uxIvutYHf4l9P/M+FeNG9NvBb/OuJK14b+C3+9cQVrw38Fv964gVDvOheG/gt/vXEFa8N/Bb/euKK1wZ+i3898YIhXnSvDfwW/3riitcGfot/PXHFawO/xb+eeMEQL7rXBn6Lfz1xxWsDv8W/nrjitYHf4l9PvGCIF91rA7/Fv5644rWB3+JfT1zx2sBv8a8nXjDEi+61gd/iX09c8drAb/GvJ654beC3+NcTLxjiRffawG/xryeueG3gt/jXE1e8NvBb/OuJFwzxontt4Lf41xNXvDbwW/zriSteG/gt/vXEC4Z40b028Fv864krXhv4Lf71xBWvDfwW/3riBUO86F4b+C3+9cQVrw38Fv964orXBn6Lfz3xgiFedK8N/Bb/euKK1wZ+i389ccVrA7/Fv554wRAvutcGfot/PXHFawO/xb+euOK1gd/iX0+8YIgX3WsDv8W/nrjitYHf4l9PXPHawG/xrydeMMSL7rWB3+JfT1zx2sBv8a8nrnht4Lf41xMvGOJF99rAb/GvJ654beC3+NcTV7w28Fv864kXDPGie23gt/jXE1e8NvBb/OuJK14b+C3+9cQLhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJUAFRBXESQtAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewQuilt;
impl IconShape for MdViewQuilt {
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
                d: "M10 18h5v-6h-5v6zm-6 0h5V5H4v13zm12 0h5v-6h-5v6zM10 5v6h11V5H10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi+Xtp4Bj/s1wC/prn9Vq8aC4Bf81zQjx/vw28Fv+z/A7w2jwv86L5HeC1eU6I5++3gdfif5bfAV6b52VeNL8DvDbPCfH8/TbwWvzP8jvAa/O8zIvmd4DX5jkhnr/fBl6L/1l+B3htnpd50fwO8No8J8Tz99vAa/E/y+8Ar83zMi+a3wFem+eEeP5+G3gt/mf5HeC1eV7mRfM7wGvznBDP328Dr8X/LL8DvDbPy7xofgd4bZ4T4vn7beC1+J/ld4DX5nmZF83vAK/Nc0I8f78NvBb/s/wO8No8L/Oi+R3gtXlOiOfvt4HX4n+W3wFem+dlXjS/A7w2zwnx/P028Fr8z/I7wGvzvF6bF80u8Nc8J8Tz99vAa/E/y+8Ar81/LMTz99vAa/E/y+8Ar81/LMTz99vAa/E/y+8Ar81/LMTz99vAa/E/y+8Ar81/LMTz99vAa/E/y+8Ar83zei1eNJeAv+Y5IZ6/3wZei/9Zfgd4bZ6XedH8DvDaPCfE8/fbwGvxP8vvAK/N8zIvmt8BXpvnhHj+fht4Lf5n+R3gtXle5kXzO8Br85wQz99vA6/F/yy/A7w2z8u8aH4HeG2eE+L5+23gtfif5XeA1+Z5mRfN7wCvzXNCPH+/DbwW/7P8DvDaPC/zovkd4LV5Tojn77eB1+J/lt8BXpvnZV40vwO8Ns8J8fz9NvBa/M/yO8Br87zMi+Z3gNfmOSGev98GXov/WX4HeG2el3nR/A7w2jwnxPP328Br8T/L7wCvzfMyL5rfAV6b54R4/n4beC3+Z/kd4LV5Xq/Ni2YX+GueE+L5+23gtfif5XeA1+Y/FuL5+23gtfif5XeA1+Y/FuL5+23gtfif5XeA1+Y/FuL5+23gtfif5XeA1+Y/FuL5+23gtfif5XeA1+Z5vRYvmkvAX/OcEM/fbwOvxf8svwO8Ns/LvGh+B3htnhPi+ftt4LX4n+V3gNfmeZkXze8Ar81zQjx/vw28Fv+z/A7w2jwv86L5HeC1eU6I5++3gdfif5bfAV6b52VeNL8DvDbPCfH8/TbwWvzP8jvAa/O8zIvmd4DX5jkhnr/fBl6L/1l+B3htnpd50fwO8No8J8Tz99vAa/E/y+8Ar83zMi+a3wFem+eEeP5+G3gt/mf5HeC1eV7mRfM7wGvznBDP328Dr8X/LL8DvDbPy7xofgd4bZ4T4vn7beC1+J/ld4DX5nmZF83vAK/Nc0I8fy8NHOd/ll3gr3ler82LZhf4a54T4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjOUugQaee3zoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewSidebar;
impl IconShape for MdViewSidebar {
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
                d: "M16,20H2V4h14V20z M18,8h4V4h-4V8z M18,20h4v-4h-4V20z M18,14h4v-4h-4V14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6K/31ehxcM8aJ7beC3+N9HvGCIF91rA7/F/z7iBUO86F4b+C3+9xEvGOJF99rAb/G/j3jBEC+61wZ+i/99xAuGeNG9NvBb/O8jXjDEi+61gd/ifx/xgiFedK8N/Bb/+4gXDPGie23gt/jfR7xgiBfdawO/xf8+4gVDvOheG/gt/vcRLxjiRffawG/xv494wRAvutcGfov/fcQLhnjRvTbwW/zvI14wxIvutYHf4n8f8YIhXnTHgZfmf5/f5gVD/P+G+P8N8aJ7beC3+N9HvGCIF91rA7/F/z7iBUO86F4b+C3+9xEvGOJF99rAb/G/j3jBEC+61wZ+i/99xAuGeNG9NvBb/O8jXjDEi+61gd/ifx/xgiFedK8N/Bb/+4gXDPGie23gt/jfR7xgiBfdawO/xf8+4gVDvOheG/gt/vcRLxjiRffawG/xv494wRAvutcGfov/fcQLhnjRvTbwW/zvI14wxIvutYHf4n8f8YIhXnSvDfwW//uIFwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EMwchQd2Ee64AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewStream;
impl IconShape for MdViewStream {
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
                d: "M4 18h17v-6H4v6zM4 5v6h17V5H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdViewWeek;
impl IconShape for MdViewWeek {
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
                d: "M6 5H3c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h3c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1zm14 0h-3c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h3c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1zm-7 0h-3c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h3c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVisibility;
impl IconShape for MdVisibility {
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
                d: "M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVisibilityOff;
impl IconShape for MdVisibilityOff {
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
                d: "M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVoiceOverOff;
impl IconShape for MdVoiceOverOff {
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
                d: "M12.99 9.18c0-.06.01-.12.01-.18 0-2.21-1.79-4-4-4-.06 0-.12.01-.18.01l4.17 4.17zm-6.1-3.56L4.27 3 3 4.27l2.62 2.62C5.23 7.5 5 8.22 5 9c0 2.21 1.79 4 4 4 .78 0 1.5-.23 2.11-.62L19.73 21 21 19.73l-8.62-8.62-5.49-5.49zM9 15c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4zm7.76-9.64l-1.68 1.69c.84 1.18.84 2.71 0 3.89l1.68 1.69c2.02-2.02 2.02-5.07 0-7.27zM20.07 2l-1.63 1.63c2.77 3.02 2.77 7.56 0 10.74L20.07 16c3.9-3.89 3.91-9.95 0-14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWatchLater;
impl IconShape for MdWatchLater {
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
                d: "M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M16.2,16.2L11,13V7h1.5v5.2l4.5,2.7L16.2,16.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifiProtectedSetup;
impl IconShape for MdWifiProtectedSetup {
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
                d: "M16.71,5.29L19,3h-8v8l2.3-2.3c1.97,1.46,3.25,3.78,3.25,6.42c0,1.31-0.32,2.54-0.88,3.63c2.33-1.52,3.88-4.14,3.88-7.13 C19.55,9.1,18.44,6.85,16.71,5.29z",
            }
            path {
                d: "M7.46,8.88c0-1.31,0.32-2.54,0.88-3.63C6,6.77,4.46,9.39,4.46,12.38c0,2.52,1.1,4.77,2.84,6.33L5,21h8v-8l-2.3,2.3 C8.74,13.84,7.46,11.52,7.46,8.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+V4aeCvgpYHjvGh2gb8Gfgb4a/7zIP5zfRXw0fz7fDXwMfznQPzn+Wrgo/iP8TXAR/MfD/Gf46WBv+I/1ssAf81/LMR/js8GPov/WJ8DfDb/sRD/OX4aeCue0yXgr3nRvDRwjOf0M8Bb8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEC3cc+CjgvYEH87/LrcB3A18D7PL8IV6wBwO/BTyY/91uBV4HuJXnhXjB/gp4af5v+GvgZXheiOfvrYGf4v+WtwF+mueEeP4+G/gs/m/5HOCzeU6I5++ngbfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++zgc/i/5bPAT6b54R4/t4a+Cn+b3kb4Kd5TogX7K+Bl+L/hr8BXprnhXjBXhr4aeBB/O/2DOC1gVt5XogX7jjw0cB7Aw/if5dnAN8NfDWwy/OH+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CZph6QeJ/xHMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWork;
impl IconShape for MdWork {
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
                d: "M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWorkOff;
impl IconShape for MdWorkOff {
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
                d: "M23 21.74l-1.46-1.46L7.21 5.95 3.25 1.99 1.99 3.25l2.7 2.7h-.64c-1.11 0-1.99.89-1.99 2l-.01 11c0 1.11.89 2 2 2h15.64L21.74 23 23 21.74zM22 7.95c.05-1.11-.84-2-1.95-1.95h-4V3.95c0-1.11-.89-2-2-1.95h-4c-1.11-.05-2 .84-2 1.95v.32l13.95 14V7.95zM14.05 6H10V3.95h4.05V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+V4aeCvgpYHjvGh2gb8Gfgb4a/7zIP5zfRXw0fz7fDXwMfznQPzn+Wrgo/iP8TXAR/MfD/Gf46WBv+I/1ssAf81/LMR/js8GPov/WJ8DfDb/sRD/OX4aeCue0yXgr3nRvDRwjOf0M8Bb8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEC3cc+CjgvYEH87/LrcB3A18D7PL8IV6wBwO/BTyY/91uBV4HuJXnhXjB/gp4af5v+GvgZXheiOfvrYGf4v+WtwF+mueEeP4+G/gs/m/5HOCzeU6I5++ngbfiOf0N8NH87/DVwEvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tn8NfDT/O3w18NI8p98BXpvnhHj+Phv4LP5v+Rzgs3lOiOfvrYGf4v+WtwF+mueEeMH+Gngp/m/4G+CleV6IF+ylgZ8GHsT/bs8AXhu4leeFeOGOAx8NvDfwIP53eQbw3cBXA7s8f4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IyxhtkGvwF90AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWorkOutline;
impl IconShape for MdWorkOutline {
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
                d: "M14 6V4h-4v2h4zM4 8v11h16V8H4zm16-2c1.11 0 2 .89 2 2v11c0 1.11-.89 2-2 2H4c-1.11 0-2-.89-2-2l.01-11c0-1.11.88-2 1.99-2h4V4c0-1.11.89-2 2-2h4c1.11 0 2 .89 2 2v2h4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEm0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4GuAv+ZfhnjRvDfwXfzv8j7Ad/PCIf5lLw38Ff87PQS4lRcM8S/7buC9+N/pe4D35gVD/MsuAsf532kXOMELhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5nn9TfALv+zHAdeiuclXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Y5HQdeiv8afwPs8pxeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xX+N1wF+m+f02sBv8bzEC4b4l5nn9TrAb/OcXhv4Lf5rvA7w2zyn1wZ+i+clXjDEv8w8r9cBfpvn9NrAb/Ff43WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfov/Gq8D/DbP6bWB3+J5iRcM8S8zz+t1gN/mOb028Fv813gd4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDbPKeXBr6Kf7u/AT6af7vXBn6L5yVeMMS/zDyv1wF+m+f02sBv8W/3O8Br82/32sBv8bzEC4b4l5nn9TrAb/OcXhv4Lf7tfgd4bf7tXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8W/3e8Ar82/3WsDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4t/sd4LX5t3tt4Ld4XuIFQ/zLzPN6HeC3eU4vDXw1/3Z/DXw0/3avDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7XRwN/zf8sLw18Nc9LvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/spYG/4n+nhwC38oIhXjTvDXwX/7u8D/DdvHCIF92Dgc8G3ho4xv9Ml4CfBj4buJV/GeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CE7mmEGqDrh4AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWysiwyg;
impl IconShape for MdWysiwyg {
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
                d: "M19,3H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.11,3,19,3z M19,19H5V7h14V19z M17,12H7v-2 h10V12z M13,16H7v-2h6V16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/fSwHsBL80VLw0cB24FbuWKnwZ+BriV/1qI/xzHgY8C3ht4MC+6vwa+G/ga/msg/uN9FPDZwHH+7W4FPhv4Hv5zIf7jHAd+C3hp/uN8N/AxwC7/ORD/MV4a+C3gOP/x/hp4HWCX/3iIf7+XBn4LOM4L9zPAX3PFXwMvzRVvDbwUL9ytwMsAu/zHQvz7HAeeDhzn+XsG8NnATwO7vGAPBt4b+GjgGM/fXwMvw38sxL/PXwEvzfP3OcBn86/zYOC7gdfi+fsa4KP5j4P4t/to4Kt4XpeA9wZ+mn+77wbei+fvdYDf5j8G4t/mOPB04DjP632A7+bf77uB9+J5/TbwOvzHQPzbfDbwWTyvzwE+m/84fw28FM/rdYDf5t8P8W/zdODBPKdnAA/mP9ZrA7/F8/oe4L3590P867008Fc8r/cBvpv/eD8NvBXPaRc4wb8f4l/vs4HP4nmdAHb5j/fWwE/xvF4G+Gv+fRD/er8NvBbP6WeAt+Y/x3HgIs/rY4Cv5t8H8a/328Br8Zw+B/hs/vP8NvBaPKfPAT6bfx/Ev95F4DjP6XOAz+Y/z28Dr8Vz+hrgo/n3QTx/7w28F8/fa/O8bgVu5YX7GOCv+bf5beC1eE6/A7w2/z6IF+y7gffiP8b7AN/Nv91vA6/Fc/oc4LP590G8cN8NvBf/Pu8DfDf/Pk8HHsxz+hzgs/n3QfzLvht4L/5t3gf4bv59Hgw8nef1PsB38++DeNF8N/Be/Ou8D/Dd/Pt9NPBVPK+HALfy74N40X038F68aN4H+G7+YzwdeDDP6W+Al+bfD/Gv89vAa/GCXQLeG/hp/mO8N/BdPK/PAT6bfz/Ev85x4LeBl+J5XQJeG/hr/mO8NPBbwHGe10OAW/n3Q/zrHQd+G3gpnu0S8NrAX/Mf4zjwW8BL87y+B3hv/mMg/m2OA78NvBRwCXht4K/5j/HSwHcBL83zugQ8GNjlPwbi3+448NPARwN/zX+M9wK+GjjO8/c2wE/zHwfxn++zgFuBnwF2eV4PBt4K+GjgwbxgPwO8Nf+xEP+5vgt4b57tVuBWnu3BwIN50f02V/w18DH8+yH+83wX8N78+/0M8FY8r+8G3od/H8R/ju8C3pt/n0vAWwO/Dfw28Fo8r+8G3od/O8R/vAcD3w28Fv82l4CvBr4a2OWK3wZei+fvu4H34d8G8Z/ntYH3Bt4aOMa/7G+Anwa+GtjlOb038F28YN8NvA//eoj/Gi8NvDTwYOA48GDgr7niVuC3gVt54d4b+C5esO8G3od/HcT/Lu8NfBcv2HcD78OLDvG/z3sD38UL9t3A+/CiQfzv9N7Ad/GCfTfwPvzLEP97vTfwXbxg3w28Dy8c4n+39wa+ixfsu4H34QVD/O/33sB38YJ9DfDRPH+I/xveG/guXrD3Ab6b54X4v+O9ge/iBXsb4Kd5Toj/W94b+C6e198Arw3s8pwQ//e8N/BdPNvfAK8N7PK8EP83vTfwXcDfAK8N7PL8If7vemvgt4FdXjDE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNbVLhBFA4QIgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdYoutubeSearchedFor;
impl IconShape for MdYoutubeSearchedFor {
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
                d: "M17.01 14h-.8l-.27-.27c.98-1.14 1.57-2.61 1.57-4.23 0-3.59-2.91-6.5-6.5-6.5s-6.5 3-6.5 6.5H2l3.84 4 4.16-4H6.51C6.51 7 8.53 5 11.01 5s4.5 2.01 4.5 4.5c0 2.48-2.02 4.5-4.5 4.5-.65 0-1.26-.14-1.82-.38L7.71 15.1c.97.57 2.09.9 3.3.9 1.61 0 3.08-.59 4.22-1.57l.27.27v.79l5.01 4.99L22 19l-4.99-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdZoomIn;
impl IconShape for MdZoomIn {
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            }
            path {
                d: "M12 10h-2v2H9v-2H7V9h2V7h1v2h2v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/fSwHsBL80Vr80VtwK3csVPAz8D3Mp/L8R/jOPAVwFvDRznRffXwFcD38N/D8S/z3Hgo4CPBo7zb3cr8DHAT/NfC/Fv99LAdwEvzX+c7wY+Btjlvwbi3+atge8CjvMf76+B1wF2+c+H+Nd7a+Cn+Jf9DPDXXPHXwEtzxVsDL8ULdyvwMsAu/7kQ/zovDfwWcJzn7xnAZwPfzQv3YOC9gc/iBftt4HX4z4V40R0Hfgt4aZ6/jwG+mn+dBwPfDbwWz9/XAB/Nfx7Ei+6zgc/ieV0C3hv4af7tvht4L56/hwC38p8D8aI5DjwdOM7zehvgp/n3+27gvXhevw28Dv85EC+a7wbei+f1McBX8x/nr4GX4nm9DvDb/MdDvGguAsd5Ts8AHsx/rNcGfovn9TXAR/MfD/Eve2ngr3he7wN8N//xfht4LZ7TrcBD+I+H+Jd9NfBRPC/xn+O9ge/ieb0M8Nf8x0L8y34beC2e088Ab81/juPARZ7X+wDfzX8sxL/st4HX4jl9DvDZ/Of5a+CleE6fA3w2/7EQ/zLzvD4H+Gye02sDv8W/nnhevw28Fs/pc4DP5j8W4l9mntfbAD/Nc3pt4Lf41xPP67eB1+I5/Qzw1vzHQvzLbgUexHP6HOCzeU4vDXw1/3qvzfP6beC1eE6fA3w2/7EQ/7LfBl6L5/Q5wGfzn+cicJzn9DnAZ/MfC/Ev+23gtXhOfw28DP85Hgw8nef1PsB38x8L8S/7aOCreF4PAW7lP95HA1/F8zoB7PIfC/EvezDwdJ7X5wCfzX+8pwMP5jn9DfDS/MdDvGj+GngpntMu8DLArfzHeW/gu3henwN8Nv/xEC+a9wa+i+f128Dr8B/jpYHfAo7znC4BDwZ2+Y+HeNHdCjyI5/XdwPvw73Mc+C3gpXlenwN8Nv85EC+6twZ+iufvu4H34d/mpYHvAl6a53UJeDCwy38OxL/OdwPvxfP318DHAL/Ni+69gK8GjvP8vQ7w2/znQfzrHAd+G3gpXrCfBn4a+Blgl+f1YOCtgI8GHswL9jPAW/OfC/Gvdxz4a+BB/Mv+Gtjl2V4aOM6L7re54nuA7+Y/HuLf5jjw08Br8R/vZ4C34nm9D/Dd/MdC/Pt8NfBR/Me4BLw18NvAbwOvxfN6H+C7+Y+D+Pd7MPDdwGvxb3MJ+Grgq4Fdrvht4LV4/t4H+G7+YyD+47w28NbAWwMP4l/2N8BPA18N7PKc3hv4Ll6w9wG+m38/xH+OlwZeGngwV7w08NdccSvw08AuL9x7A9/FC/Y+wHfz74P4n+29ge/iBXsf4Lv5t0P8z/fewHfxgr0P8N382yD+d3hv4Lt4wd4H+G7+9RD/e7w38F28YO8DfDf/Ooj/Xd4b+C5esPcBvpsXHeJ/n/cGvosX7H2A7+ZFg/jf6b2B7+IFex/gu/mXIf73em/gu3jB3gb4aV44xP9u7w18F8/rb4DXBnZ54RD/+7038F08298Arw3s8i9D/N/w3sB3AX8DvDawy4sG8X/HWwO/DezyokP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJfF7dBRCxYZwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdZoomOut;
impl IconShape for MdZoomOut {
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14zM7 9h5v1H7z",
            }
        }
    }
}
