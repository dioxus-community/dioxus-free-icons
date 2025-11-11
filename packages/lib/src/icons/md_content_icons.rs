use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADOklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/z2+GngpntPfAB/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/Nl8NvBT/di8NHOc57QJ/zb/d3wAfzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tvhp4af7tXho4xnO6BPw1/3Z/DXw0/zqI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e3w18NI8p78GPpr/Woj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwS7j1BBEgmaaAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAdd;
impl IconShape for MdAdd {
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
                d: "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeY/3t8Auzyn48BL8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5kXzN8BH86L5a2CX53QceGleNF8NvBQvGvGCIf5l5kXzO8Br81/jt4HX4kUjXjDEv8y8aH4HeG3+a/w28Fq8aMQLhviXmRfN7wCvzX+N3wZeixeNeMEQ/zLzovkd4LX5r/HbwGvxohEvGOJfZl40fw18NC+avwF2eU7HgZfiRfPVwEvzohEvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Md7HeC3eU6vDfwW//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/fXwC7P6Tjw0vzHEy8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJftgsc43+nS8BxXjDEv+y7gffif6fvAd6bFwzxL3tp4K/43+khwK28YIgXzXsD38X/Lu8DfDcvHOJF92Dgs4G3Bo7xP9Ml4KeBzwZu5V+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFMR6QaJTOJUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddBox;
impl IconShape for MdAddBox {
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
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-2 10h-4v4h-2v-4H7v-2h4V7h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddCircle;
impl IconShape for MdAddCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11h-4v4h-2v-4H7v-2h4V7h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddCircleOutline;
impl IconShape for MdAddCircleOutline {
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
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4V7zm-1-5C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEwUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/vwcCD+Nf5G2CX53QceCn+dZ4B3Mq/HeLf5rWB9wLeGjjOv97rAL/Nc3pt4Lf419sFfhr4HuC3+ddB/Ou8NvBZwGvz7/M6wG/znF4b+C3+fX4b+Bzgt3nRIF50HwV8Nf8xXgf4bZ7TawO/xX+Mjwa+hn8Z4kXzXcB78x/ndYDf5jm9NvBb/Mf5buB9eOEQ/7KvBj6KF+5vgF1edB8N/DXP6aWBr+ZFdxx4KV64zwE+mxcM8cK9NfBTPH/PAD4b+Glgl/8ex4G3Bj4beBDP39sAP83zh3jhng48mOf1PcB78z/LdwPvxfO6FXgIzx/iBfts4LN4Xl8DfDT/M3018FE8r88BPpvnhXjBng48mOf0O8Br8z/bbwOvxXP6a+BleF6I5+/BwNN5Xq8D/Db/s7028Fs8r4cAt/KcEM/fRwNfxXP6G+CledF8NfBS/Mf6G+CjedH8NfBSPKePAb6a54R4/j4b+Cye09cAH82L5reB1+I/1u8Ar82L5quBj+I5fQ7w2TwnxPP33cB78Zw+B/hsXjS/DbwW/7F+B3htXjSfDXwWz+lrgI/mOSGev+8G3ovn9DnAZ/Oi+W3gtfiP9TvAa3PFbwOvxXP6HeC1+ddBPH+fDXwWz+lrgI/mRfPbwGvxH+t3gNfmit8GXovn9DvAa/Ovg3j+Phr4Kp7TXwMvw4vmq4GX5j/WXwMfzRW/DbwWz+l3gNfmXwfx/D0YeDrP63WA3+a/328Dr8Vz+h3gtfnXQbxgfw28FM/pt4HX4b/fbwOvxXP6HeC1+ddBvGCfDXwWz+trgI/mv9dvA6/Fc/od4LX510G8cLcCD+J5fTfwPvz3+W3gtXhOvwO8Nv86iBfurYGf4vm7Ffhs4GeAXf5r/TbwWjyn3wFem38dxL/sq4GP4oX7a2CXF93HAH/NC/bbwGvxH+t3gNfmOSFeNN8NvBf/cV4H+G1esN8GXov/WL8DvDbPCfGi+2jgq/iP8TrAb/OC/TbwWvzH+h3gtXlOiH+d1wY+G3gt/n1eB/htXrDfBl6L/1i/A7w2zwnxb/PawHsDrw08iH+91wF+mxfst4HX4j/W7wCvzXNC/PsdB16af52/BnZ5wV4aOM4L9tXAS/Gc/gb4aF6wXeCveU6I/51+G3gtntPvAK/Nvw7if6ffBl6L5/Q7wGvzr4P43+m3gdfiOf0O8Nr86yD+d/pt4LV4Tr8DvDb/Ooj/nX4beC2e0+8Ar82/DuJ/p98GXovn9DvAa/Ovg/jf6beB1+I5/Q7w2vzrIP53emngOM9pF/hr/nUQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BZQGZQUKeHfsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddLink;
impl IconShape for MdAddLink {
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
                d: "M8 11h8v2H8zm12.1 1H22c0-2.76-2.24-5-5-5h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1zM3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM19 12h-2v3h-3v2h3v3h2v-3h3v-2h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP85x4KX4r/E3wC7/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xLzPP63WA3+Y5vTbwW/zXeB3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+K/xusAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/Bb/NV4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4r/E6wG/znF4b+C2el3jBEP8y87xeB/htntNrA7/Ff43XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gt/mu8DvDbPKfXBn6L5yVeMMS/zDyv1wF+m+f02sBv8V/jdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i/8arwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwW/zXeB3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+K/xusAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/Bb/NV4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4r/E6wG/znF4b+C2el3jBEP8y87xeB/htntNrA7/Ff43XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gt/mu8DvDbPKfXBn6L5yVeMMS/zDyv1wF+m+f02sBv8V/jdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i/8arwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwW/zXeB3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+K/xusAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/Bb/NV4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4r/E6wG/znF4b+C2el3jBEP8y87xeB/htntNrA7/Ff43XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gt/mu8DvDbPKfXBn6L5yVeMMS/zDyv1wF+m+f02sBv8V/jdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i/8arwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwW/zXeB3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+K/xusAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/Bb/NV4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4r/E6wG/znF4b+C2el3jBEP8y87xeB/htntNrA7/Ff43XAX6b5/TawG/xvMQLhviXvTbP66+BXZ7TawO/xX+N1wF+m+d0HHhpntdv84Ih/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OSwNfzX+Njwb+mn8/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AshcrEE6NO/sAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAmpStories;
impl IconShape for MdAmpStories {
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
            rect {
                height: "15",
                width: "10",
                x: "7",
                y: "4",
            }
            rect {
                height: "11",
                width: "2",
                x: "3",
                y: "6",
            }
            rect {
                height: "11",
                width: "2",
                x: "19",
                y: "6",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t3kw8FXAW/M/w08DHwPcyr8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrHAf+CngwV1wC/pr/Xq/Fs/018DrALi8axL/ObwGvzRWXgNcG/pr/Xu8NfBfP9tvA6/CiQbzovgt4b57tfYDv5n+G9wa+i2f7buB9+JchXjTvDXwXz7YL/DX/s7w2z+l9gO/mhUP8y14a+Cv+d3oZ4K95wRD/sp8G3or/nb4HeG9eMMS/7CJwnP+ddoETvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n5j3EJ+GteNC8NHOM/hnjBEP8y8x/jd4DX5kXz28Br8R9DvGCIf5n5j/E7wGvzovlt4LX4jyFeMMS/zPzH+B3gtXnR/DbwWvzHEC8Y4l9m/mP8DvDavGh+G3gt/mOIFwzxLzP/MX4HeG1eNL8NvBb/McQLhviXmX/ZXwOvA+zyX+M48FvAS/MvEy8Y4l9mXjR/DbwOsMt/ruPAbwEvzYtGvGCIf5l50f018DrALv85jgO/Bbw0LzrxgiH+ZeZf56+B1wF2+Y91HPgt4KX51xEvGOJfZv71/hp4HWCX/xjHgd8CXpp/PfGCIf5l5t/mr4HXAXb59zkO/Bbw0vzbiBcM8S8z/3Z/DbwOsMu/zXHgt4CX5t9OvGCIf5n59/lr4HWAXf51jgO/Bbw0/z7iBUP8y8y/318DrwPs8qI5DvwW8NL8+4kXDPEvM/8x/hp4HWCXF+448FvAS/MfQ7xgiH+Z+Y/z18DrALs8f8eB3wJemv844gVD/MvMf6y/Bl4H2OU5HQd+C3hp/mOJFwzxLzP/8f4aeB1glyuOA78FvDT/8cQLhviXmf8cfw28Dlf8FvDS/OcQLxjiX2b+8/w1V7w0/3nEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8yx4MPJ3/nR4C3MoLhnjRvDfwXfzv8j7Ad/PCIV50DwY+G3hr4Bj/M10Cfhr4bOBW/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPeeYJBaztU9gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArchive;
impl IconShape for MdArchive {
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
                d: "M20.54 5.23l-1.39-1.68C18.88 3.21 18.47 3 18 3H6c-.47 0-.88.21-1.16.55L3.46 5.23C3.17 5.57 3 6.02 3 6.5V19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V6.5c0-.48-.17-.93-.46-1.27zM12 17.5L6.5 12H10v-2h4v2h3.5L12 17.5zM5.12 5l.81-1h12l.94 1H5.12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFBklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49/ss4K2Bl+Z/vl3gp4HPAW4FEP92x4HfAl6a/53eB/hu8W/3U8Bb87/bQ8S/zVsDP8X/ft8j/vWOA08HjvO/36741/sp4K35vwHxr/PWwE/xfwfiRXcceDpwnP87EC+6nwLemv9bEC+atwZ+iv97EP+y48DTgeP8+z0DeBD/MS4Bx/j3QfzLfgp4a/79/gZ4beCtge/i3+cS8NrASwPfxb8d4oV7a+Cn+Pf7G+C1gV2ueG/gu/i3uQS8NvDXXPHewHfxb4N4wY4DTweO8+/zN8BrA7s8p/cGvot/nUvAawN/zXN6b+C7+NdDvGA/Bbw1/36vA/w2z997A9/Fi+YS8NrAX/P8fTfwXvzrIJ6/twZ+iv8Yu8DrAH/N8/fewHfxwl0CXhv4a56/9wa+i389xPM6DjwdOM5/nF3gdYC/5vl7b+C7eP4uAa8N/DXP33sD38W/DeJ5/RTw1vzH2wVeB/hrnr/3Br6L53QJeG3gr3n+3hv4Lv7tEM/prYGf4j/PLvA6wF/z/L038F1ccQl4beCvef7eG/gu/n0Qz3YceDpwnP9cu8DrAH/N8/fewFcDrw38Nc/fewPfxb8f4tl+Cnhr/mvsAq8D/DXP33Fgl+fvvYHv4j8G4oq3Bn6K/1q7wOsAf82L7r2B7+I/DgKOA08HjvNfbxd4HeCv+Ze9N/Bd/MdCwE8Bb81/n13gdYC/5gV7b+C7+I+HgJ8G3or/PpeA1wb+mhfsvYHv4j8eAo4DtwLH+K93CXht4K/5l7038F38x0Jc8dbAT/Ff6xLw2sBf86J7b+C7+I+DeLafBt6K/xqXgNcG/prn7ziwy/P33sB38R8D8WzHgVuBY/znugS8NvDXPH/vDXwV8DrAX/P8vTfwXfz7IZ7TWwM/xX+eS8BrA3/N8/fewHdxxS7wOsBf8/y9N/Bd/PsgntdPA2/Ff7xLwGsDf83z997Ad/GcdoHXAf6a5++9ge/i3w7xvI4DtwLH+I9zCXht4K95/t4b+C6ev13gdYC/5vl7b+C7+LdBPH9vDfwU/zEuAa8N/DXP33sD38ULtwu8DvDXPH/vDXwX/3qIF+yngbfi3+91gN/m+Xtv4Lt40ewCrwP8Nc/fdwPvxb8O4gU7DtwKHOPf56+B1wF2eU7vDXwX/zq7wOsAf81zem/gu/jXQ7xwbw38FP9+fw28DrDLFe8NfBf/NrvA6wB/zRXvDXwX/zaIf9lPA2/Fv99fA68DvDXwXfz77AKvA7w08F382yH+ZceBW4Fj/PvdCjyY/xi7wHH+fRAvmrcGfor/exAvup8G3or/WxAvuuPArcAx/u9A/Ou8NfBT/N+B+Nf7aeCt+L8B8a93HLgVOMb/fpfEv81bAz/F/37fI/7tvht4L/53e4j4tzsO/DbwUvzv9D7Ad4t/v48G3ht4Kf7nuwT8NPDZwK0A4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8I6pS9ekNTdbwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBackspace;
impl IconShape for MdBackspace {
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
                d: "M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-3 12.59L17.59 17 14 13.41 10.41 17 9 15.59 12.59 12 9 8.41 10.41 7 14 10.59 17.59 7 19 8.41 15.41 12 19 15.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/MvHDiX++3gdfiv4Z4wRD/MvPCiX+93wZei/8a4gVD/MvMCyeuOA68FC/YJeCvueK3gdfiv4Z4wRD/MvPCiSteG/gtXrDfAV6bK34beC3+a4gXDPEvMy+cuOK1gd/iBfsd4LW54reB1+I5/Q7w2vzrmRdOvGCIf5l54cQVrw38Fi/Y7wCvzRW/DbwWz+l3gNfmX8+8cOIFQ/zLzAsnrnht4Ld4wX4HeG2u+G3gtXhOvwO8Nv965oUTLxjiX2ZeOHHFawO/xQv2O8Brc8VvA6/Fc/od4LX51zMvnHjBEP8y88KJK14b+C1esN8BXpsrfht4LZ7T7wCvzb+eeeHEC4b4l5kXTlzx2sBv8YL9DvDaXPHbwGvxnHaBv+Zf77V54cQLhviXmRdOXPHSwFfzgv018NFc8dvAa/FfQ7xgiH+ZeeHEv95vA6/Ffw3xgiH+ZeaFE/96vw28Fv81xAuG+JeZF+63+dd7aeA4/zXEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmRfud/jXe2ngGP81xAuG+JeZF0786/028Fr81xAvGOJfZl448a/328Br8V9DvGCIf5l54cQVLw18FS/Y3wAfzRW/DbwW/zXEC4b4l5kXTlzx2sBv8YL9DvDaXPHbwGvxnC4Bf82/3mvxwokXDPEvMy+cuOK1gd/iBfsd4LW54reB1+I5/Q7w2vzrmRdOvGCIf5l54cQVrw38Fi/Y7wCvzRW/DbwWz+l3gNfmX8+8cOIFQ/zLzAsnrnht4Ld4wX4HeG2u+G3gtXhOvwO8Nv965oUTLxjiX2ZeOHHFawO/xQv2O8Brc8VvA6/Fc/od4LX51zMvnHjBEP8y88KJK14b+C1esN8BXpsrfht4LZ7T7wCvzb+eeeHEC4b4l5kXTlzx2sBv8YL9DvDaXPHbwGvxX0O8YIh/mXnhxBXHgZfmBdsF/porfht4Lf5riBcM8S8zL5z41/tt4LX4ryFeMMS/zLxw4l/vt4HX4r+GeMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/7Jd4Bj/O10CjvOCIf5l3w28F/87fQ/w3rxgiH/Zg4Gn87/TQ4BbecEQL5r3Br6L/13eB/huXjjEi+7BwGcDbw0c43+mS8BPA58N3Mq/DPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BJUulkEQwmj0AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBallot;
impl IconShape for MdBallot {
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
                d: "M13,9.5h5v-2h-5V9.5z M13,16.5h5v-2h-5V16.5z M19,21H5c-1.1,0-2-0.9-2-2V5 c0-1.1,0.9-2,2-2h14c1.1,0,2,0.9,2,2v14C21,20.1,20.1,21,19,21z M6,11h5V6H6V11z M7,7h3v3H7V7z M6,18h5v-5H6V18z M7,14h3v3H7V14z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vY4DbwU8A/ht/ush/nu8FfDWwHtzxc8Ab81/PcR/ve8G3ovn9dbAWwMvDbwM/zUQ//VeG/gtXri3AX6a/3yI/3oPBv4KOM4L9j3Ae/OfD/Ff58HAVwFvzb9sF3gIsMt/LsR/nePArcAxXjTvA3w3/7kQ/7W+G3gvXjQ/A7w1/7kQ/7XeGvgpXnQPAW7lPw/iv95PAy8NPIh/2ccAX81/HsR/n5cGvhp4LV6wvwZehv88iP9+Hw18FS/YywB/zX8OxH+d9wJemiv+Gvgenu27gffi+fsa4KP5z4H4r/FbwGvznH4beB2uOA7cChzjed0KPIT/HIj/HMeBXa74bOCzeP4+B/hsrvhu4L14/t4G+Gn+4yH+Y7028FnAzwBfzRV/Bbw0z99fAy/DFZ8NfBbP3/cA781/PMR/nM8CPpsr3gf4bq4wL5y44q2Bn+L52wUeAuzyHwvxH+O7gPfm2T4H+Gyu+G3gtXj+fgd4ba74bOCzeMHeB/hu/mMh/v2+GvgontNvA6/DFa8N/BbP3+sAv80VPw28FS/YzwBvzX8sxL/PWwM/xfP3OsBvc8V7A18NHOOKS8BHA9/NFS8N/BX/socAt/IfB/Fvdxz4K+DBPK9LwGsDf81zem2u+G2e7TjwW8BL8y/7GOCr+Y+D+Lf7aOCreF5/A7w2sMuL5rOBz+JF89fAy/AfB/Fv93TgwTynS8BrA3/Ni+6lgb/ihXsG8NPAdwN/zX8cxL/NSwN/xfP6HOCz+df7a+CleMF+Bnhr/uMh/m0+GvgqntdDgFv51/to4Ku44hnAg3heJ4Bd/mMh/m0+G/gsntMzgAfzb3MceG/gp4EHA7/Fc/od4KOBv+Y/FuLf5ruB9+I5/Q7w2vzHuJUrvhr4aeBW/nMg/m2+G3gvntPvAK/Nf4wHA7fynw/xb/PZwGfxnP4aeBn+d0H823w08FU8r4cAt/K/B+Lf5sHA03lenwN8Nv97IP7t/hp4KZ7TLvAywK3874D4t/ts4LN4Xn8NvA6wy/98iH+fW4EH8bz+GvgY4Lf5nw3x7/PWwE/xgn038NPA3wC38j8P4t/vq4GP4r+P+LdD/Mf4buC9+K/3O8Br82+H+I/z0cBX8V/rfYDv5t8O8R/rtYHPBl6L/3zfA7w3/z6I/xyvDbw38NrAg/iPdQn4auCz+fdD/Oc7Drw08NLAV/GC/Q3w0fzLfpv/OIj/Oq8N/BYv2O8Ar81/LcR/ndcGfosX7HeA1+a/FuK/zmsDv8UL9jvAa/NfC/Ff57WB3+IF+x3gtfmvhfiP8dLAMV64lwa+mhfsr4GP5kV3Cfhr/n0Q/z6vDXwX8GD+e9wKvA/w2/zbIP7t3hv4Lv5neB/gu/nXQ/zbHAeeDhznf4Zd4CHALv86iH+bjwa+iv9ZPgb4av51EP823w28F/+zfA/w3vzrIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP83ymQSO2W8wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBiotech;
impl IconShape for MdBiotech {
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
                d: "M7,19c-1.1,0-2,0.9-2,2h14c0-1.1-0.9-2-2-2h-4v-2h3c1.1,0,2-0.9,2-2h-8c-1.66,0-3-1.34-3-3c0-1.09,0.59-2.04,1.46-2.56 C8.17,9.03,8,8.54,8,8c0-0.21,0.04-0.42,0.09-0.62C6.28,8.13,5,9.92,5,12c0,2.76,2.24,5,5,5v2H7z",
            }
            path {
                d: "M10.56,5.51C11.91,5.54,13,6.64,13,8c0,0.75-0.33,1.41-0.85,1.87l0.59,1.62l0.94-0.34l0.34,0.94l1.88-0.68l-0.34-0.94 l0.94-0.34L13.76,2.6l-0.94,0.34L12.48,2L10.6,2.68l0.34,0.94L10,3.97L10.56,5.51z",
            }
            circle {
                cx: "10.5",
                cy: "8",
                r: "1.5",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlock;
impl IconShape for MdBlock {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM4 12c0-4.42 3.58-8 8-8 1.85 0 3.55.63 4.9 1.69L5.69 16.9C4.63 15.55 4 13.85 4 12zm8 8c-1.85 0-3.55-.63-4.9-1.69L18.31 7.1C19.37 8.45 20 10.15 20 12c0 4.42-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBlockFlipped;
impl IconShape for MdBlockFlipped {
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
                d: "M22,12c0-5.5-4.5-10-10-10S2,6.5,2,12s4.5,10,10,10S22,17.5,22,12z M5.7,7.1l11.2,11.2c-1.3,1.1-3,1.7-4.9,1.7 c-4.4,0-8-3.6-8-8C4,10.1,4.6,8.4,5.7,7.1z M20,12c0,1.9-0.6,3.6-1.7,4.9L7.1,5.7C8.4,4.6,10.1,4,12,4C16.4,4,20,7.6,20,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+O7gffiitcBfpv/fIj/GV4a+Cue7SHArfznQ/zP8FvAa/Ns4r8G4r/fWwM/xbP9DfDS/NdA/Pd7OvBgnu13gNfmvwbiv9dnA5/Fc/oc4LP5r4H473MceDpwnOf0McBX818D8d/nu4H34nm9DvDb/NdA/Pd4aeCveP4eAtzKfw3Ef4/fAl6b50/810H813tr4Kd4/v4GeGn+6yD+6z0deDDP3+8Ar81/HcR/rc8GPosX7HOAz+a/DuK/znHg6cBxXrCPAb6a/zqI/zpfDXwUL9zrAL/Nfx3Ef40HA0/nX/YQ4Fb+6yD+a/wW8Nr8y8R/LcR/vtcGfot/2d8AL81/LcR/vqcDD+Zf9tfAR/MvuwT8Nf8xEP+5Phr4Kv7jvQ7w2/z7If7zHAeeDhznP9Yl4MHALv9+iP88Xw18FP/xPgf4bP5jIP5zPBh4Ov/xngE8mP84iP8cvwW8Nv/xXgf4bf7jIP7jvTbwW/zH+x3gtfmPhfiP93TgwfzHewhwK/+xEP+x3hv4Lv7jfQ3w0fzHQ/zHOQ48HTjOf6xLwIOBXf7jIf7jvDbwW/zH+xjgq/nPgfiP9dq8aL4aeCn+Zc8AHsx/HsR/j78CXpp/2esAv81/HsR/D/Mv+x3gtfnPhfiv92Dg6fzLXgb4a/5zIf7rvTbwW7xw3wO8N//5EP/1Phr4Kl6wS8CDgV3+8yH+63028Fm8YJ8DfDb/NRD/9X4beC2ev2cAD+a/DuK/3l8BL83z9zbAT/NfB/Ffzzx/vwO8Nv+1EP+1Hgw8nefvZYC/5r8W4r/WawO/xfP6HuC9+a+H+K/10cBX8ZwuAQ8Gdvmvh/iv9dnAZ/GcPgf4bP57IP5r/TbwWjzbM4AH898H8V/rr4CX5tneBvhp/vsg/muZZ/sd4LX574X4r/Ng4Ok828sAf81/L8R/ndcGfosrvgd4b/77If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP0b5jQW0xhxwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBolt;
impl IconShape for MdBolt {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEv0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Ne6BLw18NXAS/G8vgf4beC7eNGIFwzxLzP/dS4Brw38NXAc+G3gpXi27wHemyveG/gu/mXiBUP8y8x/jUvAawN/zbMdB34beCnge4D35jl9NPBVvHDiBUP8y8x/nfcBvpvndBz4aOCzeU7Hgd8CXpoXTrxgiH+ZeeGeAXw3/3qfxfP3PsB388IdB34LeGn+ZeIFQ/zLzAv3O8Br869nXrD3Ab6b5+848FvAS/OiES8Y4l9mXrjfAV6bfz3zwr0O8Ns8r+8G3osXnXjBEP8y88L9DvDa/OuZF+x7gPfm+TsO/DbwUrxoxAuG+JeZF+5W4Lv51/tsnr/vAd6bF+448NvAS/EvEy8Y4l9m/ut8D/DePK/XBn6b53Qc+G3gpXjhxAuG+JeZ/xq7wEOAXZ7TdwHvDbwP8N08p9cGfosXTrxgiH+Z+a/z18DrALtc8V3Ae/Ns7wN8N1e8NPBbwHFeOPGCIf5l5r/WXwOvA3wV8N48r/cB/hr4LeA4/zLxgiH+Zea/3i5wnBdsFzjOi0a8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnX+xjgr3m2rwZeiv8e4gVD/MvMv97rAL/Ns/028Fr89xAvGOJfZv71Xgf4bZ7tt4HX4oX7G+Cj+df7LV448YIh/mXmX+91gN/m2X4beC1euN8BXpt/PfPCiRcM8S8z/3qvA/w2z/bbwGvxwv0O8Nr865kXTrxgiH+ZeeF+h+f10cBf82xfDbw0z+k48FI82+8Ar82/nnnhxAuG+JeZF07827w28Fs82y7w1/zrvTYvnHjBEP8y88KJf5vXBn6L/3ziBUP8y8wLJ/5tXhv4Lf7ziRcM8S8zL9xr87z+Btjl2V4aOMZzemngq3m23wFem38988KJFwzxLzP/eq8D/DbP9tvAa/HC/Q7w2vzrmRdOvGCIf5n513sd4Ld5tt8GXosX7neA1+Zfz7xw4gVD/MvMv97rAL/Ns/028Fq8cL8DvDb/euaFEy8Y4l9m/vVeB/htnu23gdfiv4d4wRD/MvOv9zrAb/Nsvw28Fv89xAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JftAsf43+kScJwXDPEv+27gvfjf6XuA9+YFQ/zLHgw8nf+dHgLcyguGeNG8N/Bd/O/yPsB388IhXnQPBj4beGvgGP8zXQJ+Gvhs4Fb+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I11OpUGhuZc3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCalculate;
impl IconShape for MdCalculate {
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
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M13.03,7.06L14.09,6l1.41,1.41 L16.91,6l1.06,1.06l-1.41,1.41l1.41,1.41l-1.06,1.06L15.5,9.54l-1.41,1.41l-1.06-1.06l1.41-1.41L13.03,7.06z M6.25,7.72h5v1.5h-5 V7.72z M11.5,16h-2v2H8v-2H6v-1.5h2v-2h1.5v2h2V16z M18,17.25h-5v-1.5h5V17.25z M18,14.75h-5v-1.5h5V14.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/34vDbw08N3813pv4K+Bv+bfDvHv89LAbwHHgfcBvpv/Gu8NfBewC7wO8Nf82yD+7V4a+C3gOM/2PsB385/rvYHv4tl2gdcB/pp/PcS/zUsDvwUc53m9D/Dd/Od4b+C7eF67wOsAf82/DuLf5r2B7+IFex/gu/mP9d7Ad/GCvQ/w3fzrIP7t3hv4Ll6w9wG+m/8Y7w18Fy/Y+wDfzb8e4t/nvYHv4gV7H+C7+fd5b+C7eMHeB/hu/m0Q/37vDXwXL9j7AN/Nv817A9/FC/Y+wHfzb4f4j/HewHfxgr0P8N3867w38F28YO8DfDf/Poj/OO8NfBcv2PsA382L5r2B7+IFex/gu/n3Q/zHem/gu3jB3gf4bl649wa+ixfsfYDv5j8G4j/eewPfxQv2PsB38/y9N/BdvGDvA3w3/3EQ/zneG/guXrD3Ab6b5/TewHfxgr0P8N38x0L853lv4Lt4wd4H+G6ueG/gu3jB3gf4bv7jIf5zvTfwXbxg78MV38UL9j7Ad/OfA/Gf772B7+Lf5n2A7+Y/D+K/xnsD38W/zvsA381/LsR/nfcGvosXzfsA381/PsR/rfcGvosX7n2A7+a/BuK/1nsD38UL9z7Ad/NfA/Ff572B7+JF8z7Ad/OfD/Ff472B7+Jf532A7+Y/F+I/33sD38W/zfsA381/HsR/rvcGvosX7H244rt4wd4H+G7+cyD+87w38F28YO8DfDdXvDfwXbxg7wN8N//xEP853hv4Ll6w9wG+m+f03sB38YK9D/Dd/MdC/Md7b+C7eMHeB/hunr/3Br6LF+x9gO/mPw7iP9Z7A9/FC/Y+wHfzwr038F28YO8DfDf/MRD/cd4b+C5esPcBvpsXzXsD38UL9j7Ad/Pvh/iP8d7Ad/GCvQ/w3fzrvDfwXbxg7wN8N/8+iH+/9wa+ixfsfYDv5t/mvYHv4gV7H+C7+bdD/Pu8N/BdvGDvA3w3/z7vDXwXL9j7AN/Nvw3i3+69ge/iBXsf4Lv5j/HewHfxgr0P8N386yH+bd4b+C5esPcBvpv/WO8NfBcv2PsA382/DuLf5qWB3waO8bzeB/hu/nO8N/BdPK9LwGsDf82/DuLf7qWB3waO8WzvA3w3/7neG/gunu0S8NrAX/Ovh/j3eWngt4FjwPsA381/jfcGvgu4BLw28Nf82yD+/V4aeGngu/mv9d7AXwN/zb8d4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAXeSikEEZ8WbAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClear;
impl IconShape for MdClear {
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
                d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+JF89LAMf57XQL+mv9YiBfutYHvAh7Mf7/fAV6b/1iIF+y9ge/if47fAV6b/1iI5+848HTgOP9z/A7w2vzHQjx/Hw18Ff+z/A7w2vzHQjx/3w28F/+z/A7w2vzHQjx/vw28Fs/pd4DX5v8WxPP328Br8Zx+B3ht/m9BPH+/DbwWz+l3gNfm/xbE8/fbwGvxnH4HeG3+b0E8f78NvBbP6XeA1+b/FsTz99vAa/Gcfgd4bV64twJeGnht/nv8NvDXwM/wokE8f78NvBbP6XeA1+b5ezDwU8BL8z/DXwNvA9zKC4d4/n4beC2e0+8Ar83zOg78FfBg/mf5a+B1gF1eMMTz99vAa/Gcfgd4bZ7XZwOfxf9MnwN8Ni8Y4vn7beC1eE6/A7w2z+vpwIP5n+lW4CG8YIjn77eB1+I5/Q7w2jwv87z+Btjlv9Zx4KV4XuIFQzx/vw28Fs/pd4DX5nmZ5/U6wG/zX+u1gd/ieYkXDPH8/TbwWjyn3wFem+dlntfrAL/Nf63XBn6L5yVeMMTz99vAa/Gcfgd4bZ6XeV6vA/w2/7VeG/gtnpd4wRDP328Dr8Vz+h3gtXle5nm9DvDb/Nd6beC3eF7iBUM8f78NvBbP6XeA1+Z5mef1OsBv81/rtYHf4nmJFwzx/P028Fo8p98BXpvnZZ7X6wC/zX+t1wZ+i+clXjDE8/fbwGvxnH4HeG2el3lerwP8Nv+1Xhv4LZ6XeMEQz99vA6/Fc/od4LV5XuZ5vQ7w2/zXem3gt3he4gVDPH+/DbwWz+l3gNfmeZnn9TrAb/Nf67WB3+J5iRcM8fz9NvBaPKffAV6b52We1+sAv81/rdcGfovnJV4wxPP328Br8Zx+B3htnpd5Xq8D/Db/tV4b+C2el3jBEM/fbwOvxXP6HeC1eV7meb0O8Nv813pt4Ld4XuIFQzx/vw28Fs/pd4DX5nmZ5/U6wG/zX+u1gd/ieYkXDPH8/TbwWjyn3wFem+dlntfrAL/Nf63XBn6L5yVeMMTz99vAa/Gcfgd4bZ6XeV6vA/w2/7VeG/gtnpd4wRDP328Dr8Vz+h3gtXle5nm9DvDb/Nd6beC3eF7iBUM8f78NvBbP6XeA1+Z5mef1OsBv81/rtYHf4nmJFwzx/P028Fo8p98BXpvnZZ7X6wC/zX+t1wZ+i+clXjDE8/fbwGvxnH4HeG2el3lerwP8Nv+1Xhv4LZ6XeMEQz99vA6/Fc/od4LV5XuZ5vQ7w2/zXem3gt3he4gVDPH+/DbwWz+l3gNfmeZnn9TrAb/Nf67WB3+J5iRcM8fz9NvBaPKffAV6b52We1+sAv81/rdcGfovnJV4wxPP328Br8Zx+B3htnpd5Xq8D/Db/tV4b+C2el3jBEM/fbwOvxXP6HeC1eV7meb0O8Nv813pt4Ld4XuIFQzx/vw28Fs/pd4DX5nmZ5/U6wG/zX+u1gd/ieYkXDPH8/TbwWjyn3wFem+dlntfrAL/Nf63XBn6L5yVeMMTz99LAcZ7TLvDXPC/zvF4H+G3+a7028Fs8L/GCIf79zPN6HeC3+a/12sBv8bzEC4b49zPP63WA3+a/1msDv8XzEi8Y4t/PPK/XAX6b/1qvDfwWz0u8YIh/P/O8Xgf4bf5rvTbwWzwv8YIh/v3M83od4Lf5r/XawG/xvMQLhvj3M8/rdYDf5r/WawO/xfMSLxji3888r9cBfpv/Wq8N/BbPS7xgiH8/87xeB/ht/mu9NvBbPC/xgiH+/czzeh3gt/mv9drAb/G8xAuG+Pczz+t1gN/mv9ZrA7/F8xIvGOLfz/zPJl4wxL/fXwMvxf9MvwO8Ni8Y4t/vvYHv4n+m9wG+mxcM8R/ju4H34n+W7wHemxcO8R/ns4GPBo7x3+sS8NXAZ/MvQ/zHe23+e/02LzrE/2+I/9/4R26c4kERBz+GAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContentCopy;
impl IconShape for MdContentCopy {
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
                d: "M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContentCut;
impl IconShape for MdContentCut {
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
            circle {
                cx: "6",
                cy: "18",
                r: "2",
            }
            circle {
                cx: "12",
                cy: "12",
                r: ".5",
            }
            circle {
                cx: "6",
                cy: "6",
                r: "2",
            }
            path {
                d: "M9.64 7.64c.23-.5.36-1.05.36-1.64 0-2.21-1.79-4-4-4S2 3.79 2 6s1.79 4 4 4c.59 0 1.14-.13 1.64-.36L10 12l-2.36 2.36C7.14 14.13 6.59 14 6 14c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4c0-.59-.13-1.14-.36-1.64L12 14l7 7h3v-1L9.64 7.64zM6 8c-1.1 0-2-.89-2-2s.9-2 2-2 2 .89 2 2-.9 2-2 2zm0 12c-1.1 0-2-.89-2-2s.9-2 2-2 2 .89 2 2-.9 2-2 2zm6-7.5c-.28 0-.5-.22-.5-.5s.22-.5.5-.5.5.22.5.5-.22.5-.5.5zM19 3l-6 6 2 2 7-7V3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContentPaste;
impl IconShape for MdContentPaste {
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
                d: "M19 2h-4.18C14.4.84 13.3 0 12 0c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+mjgI8GjgPfDXwM/zaI/32+C3hvntN3A+/Dvx7if5fvAt6b5++7gffhXwfxv8d3Ae/NC/fdwPvwokP87/BdwHvzovlu4H140SD+5/ts4LP41/lu4H34lyH+5zsO/DbwUvzrfDfwPrxwiP8djgO/DbwU/zpfA3w0Lxjif6bXBn6b53Qc+G3gpXjR7QIneMEQ//N8F/DewPsA381zOg78NvBSvGieATyYFwzxP8t3Ae/Ns70P8N08p+PAbwMvxb/sfYDv5gVD/M/xXcB787zeB/huntNx4LeBl+IFex/gu3nhEP8zfBfw3rxg7wN8N8/pOPDbwEvxvN4H+G7+ZYj/ft8FvDf/svcBvpvndBz4beCleLb3Ab6bFw3iv9d3Ae/Ni+59gO/mOR0Hfht4KeB9gO/mRYf47/NdwHvzr/c+wHfznI4Drw38NP86iP8e3wW8N/927wN8N/9+iP963wW8N/9+7wN8N/8+iP9a3wW8N/9xHgLcyr8d4r/OdwHvzX+c9wG+m38fxH+N7wLem/847wN8N/9+iP983wW8N/9x3gf4bv5jIP5zfRfw3vzHeR/gu/mPg/jP813Ae/Mf532A7+Y/FuI/x3cB781/nPcBvpv/eIj/eN8FvDf/cd4H+G7+cyD+Y30X8N78x3kf4Lv5z4P4j/NdwHvzH+d9gO/mPxfiP8Z3Ae/Nf5z3Ab6b/3yIf7/vAt6b/zjvA3w3/zUQ/z7fBbw3/3HeB/hu/usg/u2+C3hv/uO8D/Dd/NdC/Nt8F/De/Md5H+C7+a+H+Nf7LuC9+Y/zPsB3898D8a/zXcB78x/nfYDv5r8P4kX3XcB78x/nfYDv5r8X4kXzXcB78x/nfYDv5r8f4l/23sB38R/nfYDv5n8GxL/M/Md5H+C7+Z8D8S8z/zHeB/hu/mdB/MvMv9/7AN/N/zyIf5n593kf4Lv5nwnxLzP/du8DfDf/cyH+Zebf5n2A7+Z/NsS/zPzrvQ/w3fzPh/iXmX+d9wG+m/8dEP8y86J7H+C7+d8D8S8zL5rvAd6b/10Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcMCG5BedOXfwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCreate;
impl IconShape for MdCreate {
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
                d: "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+c48FK86H6H/1kQ/3bHgd8CXpoX3XcD78P/HIh/m+PAbwEvzb/edwPvw/8MiOfvq4GX4gV7MPBg/u3+GtjlBfsY4K/5z4d4/n4beC3++7wO8Nv850M8f78NvBb/fV4H+G3+8yGev98GXov/Pq8D/Db/+RDP328Dr8V/n9cBfpv/fIjn76uBl+a/z0cDf81/PsT/b4j/3xD/st/iv8bfAB/N8/ot/mP8DfDRPCfEv8z81/gd4LV5XuY/xu8Ar81zQvzLzH+N3wFem+dl/mP8DvDaPCfEv8z81/gd4LV5XuY/xu8Ar81zQvzLzH+N3wFem+dl/mP8DvDaPCfEv8z81/gd4LV5Xr/Nf4y/Bj6a54T4l5n/Gr8DvDb/tRD/MvNf43eA1+a/FuJfZv5r/A7w2vzXQvzLzH+N3wFem/9aiH+Z+a/xO8Br818L8S8z/zV+B3htnpf59xEvGOJfZv5r/A7w2jwv8+8jXjDEv8z81/gd4LV5XubfR7xgiH+Z+a/xO8Br87zMv494wRD/MvNf43eA1+Z5mX8f8YIh/mXmv8bvAK/N83pt/n1+mxcM8S8z/zV+B3ht/msh/mXmv8bvAK/Nfy3Ev8z81/gd4LX5r4X4l5n/Gr8DvDb/tRD/MvNf43eA1+Z5vRYvmkvAX/Ovg/iXmf8avwO8Ns/LvGh+B3ht/nUQ/zLzX+N3gNfmeZkXze8Ar82/DuJfZv5r/A7w2jwv86L5HeC1+ddB/MvMf43fAV6b52VeNL8DvDb/Ooh/mfmv8TvAa/O8zIvmd4DX5l8H8S8z/zW+Bvho/msh/mV/DbwU//k+Bvhq/msh/mXvDXwX/7kuAQ8GdvmvhXjRfDfwXvzneRvgp/mvh3jRfTbw0cAx/uM8A3hv4Lf574H413tt/mPsAn/Nfy/E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EezLnVBorN+EgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeleteSweep;
impl IconShape for MdDeleteSweep {
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
                d: "M15 16h4v2h-4zm0-8h7v2h-7zm0 4h6v2h-6zM3 18c0 1.1.9 2 2 2h6c1.1 0 2-.9 2-2V8H3v10zM14 5h-3l-1-1H6L5 5H2v2h12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13Gu2OW/B+K/z2cBH80VXw18Dv/1EP/1Xhv4LuDBPKdbgfcBfpv/Ooj/Og8Gvgt4bV643wbeB7iV/3yI/3zHgY8CPpt/nc8GvgbY5T8P4j/XewGfDTyYf5tbgc8Gvof/HIj/HC8NfBXw2rxgzwDemyu+G3gQL9hvAx8D/DX/sRD/sY4DXwW8Ny/YJeCrgc/mOX028NHAMV6w7wY+BtjlPwbiP85HAZ8NHOcF+x7go4Fdnr8HA58NvBcv2C7w2cDX8O+H+Pd7beC7gAfzgv0N8NHAb/OieW3gq4GX4gX7a+BjgN/m3w7xb/dg4KuAt+YFuwR8NPDd/Nu8N/DVwDFesJ8GPga4lX89xL/eceCjgI8GjvOCfQ3w2cAu/z7Hgc8GPooXbBf4auBrgF1edIh/nbcGvgp4MC/Y7wDvDdzKf6wHA98NvBYv2K3AxwA/zYsG8aJ5aeCrgNfmBXsG8NHAT/Of662BrwYexAv228DHAH/NC4d44Y4DnwV8NC/YJeCrga8GdvmvcRz4aOCjgWO8YF8NfA6wy/OHeMEeDPwU8NK8YD8DfDRwK/89Hgx8NfBWvGB/DbwNcCvPC/GC/RXw0jx/fwN8NPDb/M/w2sBXAy/F8/fXwMvwvBDP31sDP8XzugR8NvDV/M/00cBnA8d4Xq8D/DbPCfH8fTbwWTyn7wE+Gtjlf7bjwFcD78Vz+hzgs3lOiOfvt4HX4jn9NvA+wK38z/bSwFcBr81z+hngrXlOiOfvt4HX4vn7bOBz+J/nOPBRwGfz/P0O8No8J8Tz99vAa/GC3Qq8D/Db/M/w2sB3AQ/mBfsd4LV5Tojn77eB1+Jf9t3AxwC7/Pc4DnwX8Nb8y34HeG2eE+L5+23gtXjR7AIfDXwP/7U+Cvhs4Dgvmt8BXpvnhHj+fht4Lf51fhv4GOCv+c/10sBXAa/Nv87vAK/Nc0I8f78NvBb/Np8NfA2wy3+s48BHAZ/Nv83vAK/Nc0I8f78NvBb/drcC7wP8Nv8xXhv4LuDB/Nv9DvDaPCfE8/fbwGvx7/fTwPsAu/zbHAe+C3hr/v1+B3htnhPi+ftt4LX4j7ELfDbwNfzrfBTw2cBx/mP8DvDaPCfE8/fbwGvxH+u3gY8B/poX7qWBrwJem/9YvwO8Ns8J8fz9NvBa/Of4bOBrgF2e03Hgo4DP5j/H7wCvzXNCPH+/DbwW/3luBd4H+G2ueG3gu4AH85/nd4DX5jkhnr/fBl6L/3w/zRVvzX++3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C+GnGQe3mNA4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDrafts;
impl IconShape for MdDrafts {
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
                d: "M21.99 8c0-.72-.37-1.35-.94-1.7L12 1 2.95 6.3C2.38 6.65 2 7.28 2 8v10c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2l-.01-10zM12 13L3.74 7.84 12 3l8.26 4.84L12 13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/FewEcDL81/nluB7wa+Btjl+UP81/su4L35r/PXwNsAt/K8EP+1Phv4LP7r/TXwMjwvxH+ti8Bx/nu8DfDTPCfEf53XBn6L/z6fA3w2zwnxX+e1gd/iv8/vAK/Nc0L813lt4Lf47/M7wGvznBD/dV4b+C3++/wO8No8J8R/ndcGfov/Pr8DvDbPCfFf57WB3+K/z+8Ar81zQvzXeW3gt3hev8N/vJcGjvGcfgd4bZ4T4r/OawO/xfMS//F+G3gtntPvAK/Nc0L813lt4Ld4XuI/3m8Dr8Vz+h3gtXlOiH+/3+J5fQzw1zyn1wZ+i+cl/uP9NvBaPKffAV6b54T49zPP63WA3+Y5vTbwWzwv8R/vt4HX4jn9DvDaPCfEv595Xq8D/DbP6bWB3+J5if94vw28Fs/pd4DX5jkh/v3M83od4Ld5Tq8N/BbPS/zH+23gtXhOvwO8Ns8J8e9nntfrAL/Nc3pt4Ld4XuI/3m8Dr8Vz+h3gtXlOiH8/87xeB/htntNrA7/F8xL/8X4beC2e0+8Ar81zQvz7mef1OsBv85xeG/gtnpf4j/fbwGvxnH4HeG2eE+Lfzzyv1wF+m+f02sBv8bzEf7zfBl6L5/Q7wGvznBD/fuZ5vQ7w2zyn1wZ+i+f12/zHe2ngOM/pZ4C35jkh/v3M83od4Ld5Ti8N/BX/fT4H+GyeE+Lfzzyv1wF+m+d1K/Ag/ns8BLiV54R4/n4beC2e0+8Ar83zMs/rdYDf5nm9NvBb/Nf7GuCjeV6I5++3gdfiOf0O8No8L/O8Xgf4bZ6/9wa+GjjGf42vAT6a5w/x/P028Fo8p98BXpvnZZ7X6wC/zQt2HHhv4KWBB/Mfbxf4a+C7gVt5wRDP328Dr8Vz+h3gtXle5nm9DvDb/M+HeP5+G3gtntPvAK/N8zLP63WA3+Z/PsTz99vAa/Gcfgd4bZ6XeV6vA/w2//Mhnr/fBl6L5/Q7wGvzvMzzeh3gt/mfD/H8/TbwWjyn3wFem+dlntfrAL/N/3yI5++3gdfiOf0O8No8L/Nf73WA3+bfD/H8/TbwWjyn3wFem+f118BL8V/rdYDf5t8P8fz9NvBaPKffAV6b5/XewHfxX+t1gN/m3w/x/P028Fo8p98BXpvn77uB9+K/zusAv82/H+L5+23gtXhOvwO8Ni/YZwMfDRzjP9/rAL/Nvx/i+ftt4LV4Tr8DvDb/stfmP99fA7v8+yGev98GXovn9DvAa/N/C+L5+23gtXhOvwO8Nv+3IJ6/3wZei+f0O8Br838L4vn7beC1eE6/A7w2/7cgnr/fBl6L5/TXwEfzn+cS8Nf810I8f58NfBb/tX4HeG3+ayGev7cGfor/Wr8DvDb/tRAv2F8DL8V/nd8BXpv/WogX7MHATwMvxX+N3wFem/9aiBfuOPDRwHsDD+I/1+8Ar81/LcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EcQrq0GDk2V/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDynamicFeed;
impl IconShape for MdDynamicFeed {
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
                d: "M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z",
            }
            path {
                d: "M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z",
            }
            path {
                d: "M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z",
            }
            g {
                display: "none",
            }
            g {
                display: "inline",
            }
            g {
                display: "inline",
            }
            path {
                d: "M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z",
            }
            path {
                d: "M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z",
            }
            path {
                d: "M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+JF89LAMf57XQL+mv9YiBfutYHvAh7Mf7/fAV6b/1iIF+y9ge/if47fAV6b/1iI5+848HTgOP9z/A7w2vzHQjx/Hw18Ff+z/A7w2vzHQjx/3w28F/+z/A7w2vzHQjx/vw28Fs/pd4DX5v8WxPP328Br8Zx+B3ht/m9BPH+/DbwWz+l3gNfm/xbE8/fbwGvxnH4HeG3+b0E8f78NvBbP6XeA1+b/FsTz99vAa/Gcfgd4bV64twJeGnht/uN8DPDX/OdAPH+/DbwWz+l3gNfm+Xsw8FPAS/Mfbxd4HeCv+Y+HeP5+G3gtntPvAK/N8zoO/Bbw0vzn2QVeB/hr/mMhnr/fBl6L5/Q7wGvzvD4b+Cz+8+0CrwP8Nf9xEM/fbwOvxXP6HeC1eV5/Bbw0/7G+B3gvntcu8DrAX/MfA/H8/TbwWjyn3wFem+dl/uO9DvBg4Lt4XrvA6wB/zb8f4vn7beC1eE6/A7w2z8v8x3sd4LeB9wa+i+e1C7wO8Nf8+yCev98GXovn9DvAa/O8zH+81wF+myveG/guntcu8DrAX/Nvh3j+fht4LZ7T7wCvzfMy//FeB/htnu29ge/iee0CrwP8Nf82iOfvt4HX4jn9DvDaPC/zH+91gN/mOb038F08r13gdYC/5l8P8fz9NvBaPKffAV6b52X+470O8Ns8r/cGvovntQu8DvDX/Osgnr/fBl6L5/Q7wGvzvMx/vNcBfpvn772B7+J57QKvA/w1LzrE8/fbwGvxnH4HeG2el/mP9zrAb/OCvTfwXTyvXeB1gL/mRYN4/n4beC2e0+8Ar83zMv/xXgf4bV649wa+i+e1C7wO8Nf8yxDP328Dr8Vz+h3gtXle5j/e6wC/zb/svYHv4nntAq8D/DUvHOL5+23gtXhOvwO8Ns/L/Md7HeC3edG8N/BdPK9d4HWAv+YFQzx/vw28Fs/pd4DX5nmZ/3ivA/w2L7r3Br6L57ULnOAFQzx/vw28Fs/pd4DX5nmZ/3ivA/w2/zrvDXwXz0u8YIjn77eB1+I5/Q7w2jwv8x/vdYDf5l/vvYHv4jmJFwzx/P028Fo8p98BXpvnZf5nEy8Y4vn7beC1eE6/A7w2z8v8zyZeMMTz99vAa/Gcfgd4bZ6X+Z9NvGCI5++3gdfiOf0O8No8L/M/m3jBEM/fbwOvxXP6HeC1eV7mfzbxgiGev98GXovn9DvAa/O8zP9s4gVDPH+/DbwWz+l3gNfmeZn/2cQLhnj+fht4LZ7T7wCvzfMy/7OJFwzx/P028Fo8p98BXpvnZf5nEy8Y4vn7beC1eE6/A7w2z8v8zyZeMMTz99vAa/Gcfgd4bZ6X+Z9NvGCI5++3gdfiOf0O8No8L/M/m3jBEM/fbwOvxXP6HeC1eV7mfzbxgiGev5cGjvOcdoG/5nmZ/9nEC4b49zP/s4kXDPHvZ/5nEy8Y4t/P/M8mXjDEv5/5n028YIh/P/M/m3jBEP9+5n828YIh/v3M/2ziBUP8+5n/2cQLhvj3M/+ziRcM8e9n/mcTLxji38/8zyZeMMS/318DL8X/TL8DvDYvGOLf772B7+J/pvcBvpsXDPEf47uB9+J/lu8B3psXDvEf57OBjwaO8d/rEvDVwGfzL0P8x3tt/nv9Ni86xP9viP/f+EcqispBZs0GVQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFileCopy;
impl IconShape for MdFileCopy {
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
                d: "M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm-1 4l6 6v10c0 1.1-.9 2-2 2H7.99C6.89 23 6 22.1 6 21l.01-14c0-1.1.89-2 1.99-2h7zm-1 7h5.5L14 6.5V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB8klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8e/3W/z3eh3+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fr/Nf6/X5t8O8f8b4v83xP9viP/fEP+/If5/Q/z/hvivcxx4KV40fwPs8p8P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BKzhGEGDNYA3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFilterList;
impl IconShape for MdFilterList {
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
                d: "M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADyElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP85/kbYJf/fIh/m98GXov/PJ8DfDb/+RD/Nr8NvBb/eX4GeGv+8yH+bX4beC3+89wKPIT/fIh/m98GXov/XCeAXf5zIf5tfht4Lf5zvQ7w2/znQvzb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWvzP9TrAb/MvQ/zb/DbwWjynvwE+mn+b1wY+i/84rwP8Nv8yxL/NbwOvxXP6HeC1+bd5aeCv+I/zOsBv8y9D/Nv8NvBaPKffAV6bfzvzH+d1gN/mX4b4t/lt4LV4Tr8DvDb/dr8NvBb/MV4H+G3+ZYh/m98GXovn9DvAa/Nv99XAR/Ef43WA3+Zfhvi3+W3gtXhOvwO8Nv92bw18NP8xPhr4a/5liH+b3wZei+f0O8Br878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE6/A7w2/7sg/m1+G3gtntPvAK/N/y6If5vfBl6L5/Q7wGvzvwvi3+a3gdfiOf0O8Nr874L4t/lt4LV4Tr8DvDb/uyD+bX4beC2e0+8Ar83/Loh/m98GXovn9DvAa/O/C+Lf5reB1+I5/Q7w2vzvgvi3+W3gtXhOvwO8Nv+7IP5tfht4LZ7T7wCvzf8uiH+b3wZei+f0O8Br878L4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IPg1oQSr5DioAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlag;
impl IconShape for MdFlag {
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
                d: "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFontDownload;
impl IconShape for MdFontDownload {
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
                d: "M9.93 13.5h4.14L12 7.98zM20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-4.05 16.5l-1.14-3H9.17l-1.12 3H5.96l5.11-13h1.86l5.11 13h-2.09z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv99xYJf/Hoj/fn8FvA6wy389xH8/A38NvA6wy38txH8/c8VfA68D7PJfB/HfzzzbXwOvA+zyXwPx3888p78GXgfY5T8f4r+feV5/DbwOsMt/LsR/P/P8/TXwOsAu/3kQ//3MC/bXwOsAu/znQPz3My/cXwOvA+zyHw/x38/8y/4aeB1gl/9YiBfdSwNfxX+81+ZF89fA6wC7/MdBvOheG/gt/nv9NfA6wC7/MRAvutcGfov/fn8NvA6wy78f4kX32sBv8T/DXwOvA+zy74N40b028Fv8z/HXwOsAu/zbIV50rw38Fv+z/DXwOsAu/zaIF91rA7/F/zx/DbwOsMu/HuJF99rAb/E/018DrwPs8q+DeNG9NvBb/M/118DrALu86BAvutcGfov/2f4aeB1glxcN4kX32sBv8T/fXwOvA+zyL0O86F4b+C3+5/sb4LWBXf5liBfdawO/xf9sfwO8NrDLiwbxontt4Lf4n+tvgNcGdnnRIV50rw38Fv8z/Q3w2sAu/zqIF91rA7/F/zx/A7w2sMu/HuJF99rAb/E/y98Arw3s8m+DeNG9NvBb/M/xN8BrA7v82yFedK8N/Bb/M/wN8NrALv8+iBfdawO/xX+/vwFeG9jl3w/xontt4Lf47/U3wGsDu/zHQLzoXhr4av7jvRYvmr8BXhvY5T8O4r+f+Zf9DfDawC7/sRD//cwL9zfAawO7/MdD/PczL9jfAK8N7PKfA/Hfzzx/fwO8NrDLfx7Efz/zvP4GeG1gl/9ciP9+5jn9DfDawC7/+RD//cyz/Q3w2sAu/zUQ//3MFX8DvDawy38dxH8/A38DvDawy38txH+/vwZeG9jlvx7iv99xYJf/Hoj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj+W5fkHl+wVlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForward;
impl IconShape for MdForward {
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
                d: "M12 8V4l8 8-8 8v-4H4V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGesture;
impl IconShape for MdGesture {
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
                d: "M4.59 6.89c.7-.71 1.4-1.35 1.71-1.22.5.2 0 1.03-.3 1.52-.25.42-2.86 3.89-2.86 6.31 0 1.28.48 2.34 1.34 2.98.75.56 1.74.73 2.64.46 1.07-.31 1.95-1.4 3.06-2.77 1.21-1.49 2.83-3.44 4.08-3.44 1.63 0 1.65 1.01 1.76 1.79-3.78.64-5.38 3.67-5.38 5.37 0 1.7 1.44 3.09 3.21 3.09 1.63 0 4.29-1.33 4.69-6.1H21v-2.5h-2.47c-.15-1.65-1.09-4.2-4.03-4.2-2.25 0-4.18 1.91-4.94 2.84-.58.73-2.06 2.48-2.29 2.72-.25.3-.68.84-1.11.84-.45 0-.72-.83-.36-1.92.35-1.09 1.4-2.86 1.85-3.52.78-1.14 1.3-1.92 1.3-3.28C8.95 3.69 7.31 3 6.44 3 5.12 3 3.97 4 3.72 4.25c-.36.36-.66.66-.88.93l1.75 1.71zm9.29 11.66c-.31 0-.74-.26-.74-.72 0-.6.73-2.2 2.87-2.76-.3 2.69-1.43 3.48-2.13 3.48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9ZLA8d4tmcAt/LfB/Gf78HAZwGvDTyY5/XXwHcD3wPs8l8L8Z/rq4CP5kWzC7wP8NP810H85zgO/Bbw0vzrfTbwOfzXQPzn+Cvgpfm3+xjgq/nPh/iP99XAR/Hv9zrAb/OfC/Ef68HA0/mP8dvA6/CfC/Ef67uB9+I/zusAv81/HsR/rKcDD+Y/ztcAH81/HsR/nJcG/or/WL8DvDb/eRD/cV4b+C3+44n/PIj/OK8N/Bb/8cR/HsR/nNcGfov/WM8AHsx/HsR/nAcDT+c/1u8Ar81/HsR/rL8GXor/OB8DfDX/eRD/sT4a+Cr+4zwEuJX/PIj/WMeBW4Fj/Pt9D/De/OdC/Md7a+Cn+Pe5BDwY2OU/F+I/x1cDH8W/zSXgtYG/5j8f4j/PRwNfxb/OJeC1gb/mvwbiP9drA58NvBb/su8BPhrY5b8O4r/GWwOvDbw08Fpc8QzgVuCngZ8GbuW/HuL/N8T/XS/NFX/NC4b4v+mlgd/iitcB/prnD/F/z0sDvwUc54pd4HWAv+Z5If5veWngt4DjPKdd4HWAv+Y5If5zHQdeCnhp4DhwHHhprtgFvgf4af5jvDTwW8Bxnr/3Ab6b54T4j/XSwGsBrw28NPBgXrC/AV4b2OXf76WB3wKO8/y9D/DdPC/Ev99LA+8FvDXwYF40fwO8NrDLv99LA78FHOf5ex/gu3n+EP927wW8N/Da/Ov8DfDawC7/fi8N/BZwnOfvfYDv5gVD/Ou9NvBdwIP51/sb4LWBXZ6/BwO38qJ5aeC3gOM8f+8DfDcvHOJf56uAj+bf5m+A1wZ2ef5eGvgt4KeB9+GFe2ngt4DjPH/vA3w3/zLEi+Y48FvAS/Nv8zfAawO7PH8vDfwWcJwrvht4H56/lwZ+CzjO8/c+wHfzokG8aH4LeG3+bf4GeG1gl+fvpYHfAo7znL4beB+e00sDvwUc5/l7H+C7edEh/mXvDXwX/3avA/w2L9hvA6/F8/fdwPtwxUsDvwUc5/l7H+C7+ddB/Mt+Gngr/u12gdcB/prn7zjw28BL8fx9N/A1wG8Bx3n+3gf4bv71EP8y8++3C7wO8Nc8f8eB3wZein+99wG+m38bxL/M/MfYBV4H+Guev+PAbwMvxYvufYDv5t8O8S8z/3F2gdcB/prn7zjw28BL8S97H+C7+fdB/MvMf6xd4HWAv+b5Ow78NvBSvGDvA3w3/36If5n5j7cLvA7w1zx/x4HfBl6K5/U+wHfzHwPxLzP/OXaB1wH+mufvOPDbwEvxbO8DfDf/cRD/stfmP88u8Ne8YMeB3wZeCngf4Lv5j4X4n+848NrAT/MfD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CI5EnEF4nuFAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHowToReg;
impl IconShape for MdHowToReg {
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
            g {
                fill_rule: "evenodd",
            }
            path {
                d: "M9 17l3-2.94c-.39-.04-.68-.06-1-.06-2.67 0-8 1.34-8 4v2h9l-3-3zm2-5c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4",
            }
            path {
                d: "M15.47 20.5L12 17l1.4-1.41 2.07 2.08 5.13-5.17 1.4 1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHowToVote;
impl IconShape for MdHowToVote {
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
                d: "M18 13h-.68l-2 2h1.91L19 17H5l1.78-2h2.05l-2-2H6l-3 3v4c0 1.1.89 2 1.99 2H19c1.1 0 2-.89 2-2v-4l-3-3zm-1-5.05l-4.95 4.95-3.54-3.54 4.95-4.95L17 7.95zm-4.24-5.66L6.39 8.66c-.39.39-.39 1.02 0 1.41l4.95 4.95c.39.39 1.02.39 1.41 0l6.36-6.36c.39-.39.39-1.02 0-1.41L14.16 2.3c-.38-.4-1.01-.4-1.4-.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FnAawPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5nl9DPDX/M/y0sBX8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mfn3Ef8+5t9HvGCIf5n593kZ4K/5t3lp4K/49xEvGOJfZv59Pgb4av5tPhr4Kv59xAuG+JeZf59bgYfwb/N04MH8+4gXDPEvM/9+HwN8Nf86Hw18Ff9+4gVD/MvMf4yXAf6aF81LA3/FfwzxgiH+ZeY/zkcDX8ML91HAV/MfR7xgiH+Z+Y91K/DVwO8Af80VLw28FvDRwIP5jyVeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/bBc4xv9Ol4DjvGCIf9lPA2/F/07fA7w3LxjiX/bSwF/xv9NDgFt5wRAvmvcGvov/Xd4H+G5eOMSL7qWBjwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8I1VmFQV+PeR0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInbox;
impl IconShape for MdInbox {
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
                d: "M19 3H4.99c-1.11 0-1.98.89-1.98 2L3 19c0 1.1.88 2 1.99 2H19c1.1 0 2-.9 2-2V5c0-1.11-.9-2-2-2zm0 12h-4c0 1.66-1.35 3-3 3s-3-1.34-3-3H4.99V5H19v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsights;
impl IconShape for MdInsights {
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
                d: "M21,8c-1.45,0-2.26,1.44-1.93,2.51l-3.55,3.56c-0.3-0.09-0.74-0.09-1.04,0l-2.55-2.55C12.27,10.45,11.46,9,10,9 c-1.45,0-2.27,1.44-1.93,2.52l-4.56,4.55C2.44,15.74,1,16.55,1,18c0,1.1,0.9,2,2,2c1.45,0,2.26-1.44,1.93-2.51l4.55-4.56 c0.3,0.09,0.74,0.09,1.04,0l2.55,2.55C12.73,16.55,13.54,18,15,18c1.45,0,2.27-1.44,1.93-2.52l3.56-3.55 C21.56,12.26,23,11.45,23,10C23,8.9,22.1,8,21,8z",
            }
            polygon {
                points: "15,9 15.94,6.93 18,6 15.94,5.07 15,3 14.08,5.07 12,6 14.08,6.93",
            }
            polygon {
                points: "3.5,11 4,9 6,8.5 4,8 3.5,6 3,8 1,8.5 3,9",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sgcD7wW8NHCc/x12gb8Gvge4lRcM8cJ9FvDZ/O/22cDn8PwhXrCvBj6K/xs+B/hsnhfi+Xsw8HT+b3kIcCvPCfH8fTbwWfzf8jnAZ/OcEM/fTwNvxXO6BPw1/71uBW7lX/bewIN4Tj8DvDXPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gc/hr4aP53+GrgpXlOvwO8Ns8J8fx9NvBZ/N/yOcBn85wQz99rA7/F/y2vA/w2zwnxgv028Fr83/A7wGvzvBAv2HHgt4GX4n+3vwFeG9jleSH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t/nt/n3eW3+fcQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JfdCjyI/53+BnhpXjDEv+y9ge/if6f3Ab6bFwzxovlr4KX43+VvgJfmhUO8aI4D3w28Ff87/Azw3sAuLxziX+elgbcGXpv/mX4b+Gngr3nRIP5/Q/z/hvj/DfH/G+L/N/4Rw7BsQYd/g+wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInventory;
impl IconShape for MdInventory {
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
                d: "M20 2H4c-1 0-2 .9-2 2v3.01c0 .72.43 1.34 1 1.69V20c0 1.1 1.1 2 2 2h14c.9 0 2-.9 2-2V8.7c.57-.35 1-.97 1-1.69V4c0-1.1-1-2-2-2zm-5 12H9v-2h6v2zm5-7H4V4l16-.02V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/vwcCD+Nf5G2CX53QceCn+dZ4B3Mq/HeLf5rWB9wLeGjjOv97rAL/Nc3pt4Lf419sFfhr4HuC3+ddB/Ou8NvBZwGvz7/M6wG/znF4b+C3+fX4b+Bzgt3nRIF50HwV8Nf8xXgf4bZ7TawO/xX+Mjwa+hn8Z4kXzXcB78x/ndYDf5jm9NvBb/Mf5buB9eOEQ/7KvBj6KF+5vgF1edB8N/DXP6aWBr+ZFdxx4KV64zwE+mxcM8cK9NfBTPH/PAD4b+Glgl/8ex4G3Bj4beBDP39sAP83zh3jhng48mOf1PcB78z/LdwPvxfO6FXgIzx/iBfts4LN4Xl8DfDT/M3018FE8r88BPpvnhXjBng48mOf0O8Br8z/bbwOvxXP6a+BleF6I5+/BwNN5Xq8D/Db/s7028Fs8r4cAt/KcEM/fRwNfxXP6G+CledF8NfBS/Mf6G+CjedH8NfBSPKePAb6a54R4/j4b+Cye09cAH82L5reB1+I/1u8Ar82L5quBj+I5fQ7w2TwnxPP33cB78Zw+B/hsXjS/DbwW/7F+B3htXjSfDXwWz+lrgI/mOSGev+8G3ovn9DnAZ/Oi+W3gtfiP9TvAa/Oi+Wzgs3hOXwN8NM8J8fx9NvBZPKevAT6aF81vA6/Ff6zfAV6bF81XAx/Fc/oc4LN5Tojn76OBr+I5/TXwMrxovhp4af5j/TXw0bxong48mOf0McBX85wQz9+DgafzvF4H+G3+Z3tt4Ld4Xg8BbuU5IV6wvwZeiuf028Dr8D/bbwGvzXP6G+CleV6IF+yzgc/ieX0N8NH8z/TVwEfxvD4G+GqeF+KFuxV4EM/ru4H34X+W7wLem+f1DODBPH+IF+6tgZ/i+bsV+GzgZ4Bd/nscB94K+GzgwTx/bwP8NM8f4l/21cBH8cL9NbDLi+5jgL/mOb008FW86I4DL80L9znAZ/OCIV403w28F/9xXgf4bZ7TawO/xX+c7wHemxcO8aL7aOCr+I/xOsBv85xeG/gt/mN8DPDV/MsQ/zqvDXw28Fr8+7wO8Ns8p9cGfot/n98BPhv4bV40iH+b1wbeG3ht4EH8670O8Ns8p9cGfot/vWcAvw18N/Db/Osg/v2OAy/Nv85fA7s8p+PAS/Ov89fALv92iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjwYoikF2+AX2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLink;
impl IconShape for MdLink {
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
                d: "M3.9 12c0-1.71 1.39-3.1 3.1-3.1h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1zM8 13h8v-2H8v2zm9-6h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1s-1.39 3.1-3.1 3.1h-4V17h4c2.76 0 5-2.24 5-5s-2.24-5-5-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLinkOff;
impl IconShape for MdLinkOff {
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
                d: "M17 7h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1 0 1.43-.98 2.63-2.31 2.98l1.46 1.46C20.88 15.61 22 13.95 22 12c0-2.76-2.24-5-5-5zm-1 4h-2.19l2 2H16zM2 4.27l3.11 3.11C3.29 8.12 2 9.91 2 12c0 2.76 2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1 0-1.59 1.21-2.9 2.76-3.07L8.73 11H8v2h2.73L13 15.27V17h1.73l4.01 4L20 19.74 3.27 3 2 4.27z",
            }
            path {
                d: "M0 24V0",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3EeDLwU8NJc8dLAcf5lr8Pz+mrgpfiP9TfAR/OcEP8+DwbeCnhv4KX5txHP67eB1+I/1u8Ar81zQvzbHAc+Cvho4Dj/PuJ5/TbwWvzH+h3gtXlOiH+9twa+CzjOfwzxvH4beC3+Y/0O8No8J8S/zmcBn81/LPG8fht4Lf5j/Q7w2jwnxIvuu4D35l/2DOBWXnSvzfP6auCl+Y/118BH85wQL5qPBr6KF+wZwFcDPw3cyv8eiH/ZWwM/xfN3Cfho4Lv53wnxwh0Hng4c53n9DfDewF/zvxfihfts4LN4Xn8DvDawy/9uiBfswcBfAcd5TpeA1wb+mv/9EC/YRwNfxfP6GOCr+b8B8YL9FfDSPKdnAA/m/w7E8/dg4Ok8r48Bvpr/OxDP31sDP8XzeghwK/93IJ6/zwY+i+f0DODB/Nf4beC1+I/1O8Br85wQz99nA5/Fc/od4LX5r/HbwGvxH+t3gNfmOSGev58G3orn9DvAa/Nf47eB1+I/1u8Ar81zQjx/nw18Fs/pd4DX5r/GbwOvxX+s3wFem+eEeP4+G/gsntNfAy/Df43fBl6L/1i/A7w2zwnx/L038F08rxPALv/5Xho4zn+sXeCveU6I5++lgb/ieb0P8N3834F4wW4FHsRz+m3gdfi/A/GCfTXwUTyv1wF+m/8bEC/YawO/xfP6a+B1gF1eNC8N/DX/MyFeuN8GXovn9d3A+/CiuQi8DvDX/M+DeOEeDDyd5++7gffhX2ZgF3gd4K/5nwXxL/tq4KN4/n4beB/gVl4wc8Uu8DrAX/M/B+JF89vAa/GCfTbwPcCtPC/zbLvA6wB/zf8MiBfNceC3gZfihftr4Ke54q+BXeC3eU67wOsAf80L9tLAMf5jXQL+mueEeNEdB34beCn+/XaB1wH+mufvt4HX4j/W7wCvzXNC/Ot9N/Be/PvtAq8D/DXP67eB1+I/1u8Ar81zQvzbvDXw1cCD+Pf5HeC1eV6/DbwW/7F+B3htnhPi3+e9gY8GXop/vb8BXhvY5Xn9NvBa/Mf6HeC1eU6I/xgPBt4aeGuueDDwIF6wvwFeG9jl+ftt4LX4j/U7wGvznBD/+cxz+hvgtYFd/vsh/vOZZ/sb4LWBXf5nQPznM1f8DfDawC7/cyD+8xn4G+C1gV3+Z0H85/tr4LWBXf7nQfznOw7s8j8T4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjqp+TQaGu1IsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLowPriority;
impl IconShape for MdLowPriority {
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
                d: "M14 5h8v2h-8zm0 5.5h8v2h-8zm0 5.5h8v2h-8zM2 11.5C2 15.08 4.92 18 8.5 18H9v2l3-3-3-3v2h-.5C6.02 16 4 13.98 4 11.5S6.02 7 8.5 7H12V5H8.5C4.92 5 2 7.92 2 11.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5HuC9eU6I5++3gdfiOf0N8NPAZ/E/2+cAbw28FM/pd4DX5jkhnr/fBl6L5/Q7wGsDLw18NfBa/M/yO8BHA38N/DbwWjyn3wFem+eEeP5+G3gtntPvAK/Ns3008NnAMf57XQI+G/hqnu23gdfiOf0O8No8J8Tz99vAa/Gcfgd4bZ7TceC7gbfiv8fPAO8N7PKcfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm+Xtt4LuBB/Ff4xnAewO/zfP328Br8Zx+B3htnhPi+ftt4LV4TrcCbwP8Nc/fceCjgc/iP9fnAF8N7PL8vTTwU8CDeU6/A7w2zwnx/P028Fo8f58NfA2wy/P30sBXA6/Ff6zfAT4a+Guev+PARwGfzfP3O8Br85wQz99vA6/FC3Yr8D7Ab/OCfTTw2cAx/n0uAZ8NfDUv2GsD3wU8mBfsd4DX5jkhnr/fBl6Lf9lPA+8D7PL8HQe+G3gr/m1+BnhvYJfn7zjwXcBb8y/7HeC1eU6I5++3gdfiRbMLfDbwNbxgrw18N/AgXjTPAN4b+G1esI8CPhs4zovmd4DX5jkhnr/fBl6Lf53fBj4G+Guev+PARwOfxQv3OcBXA7s8fy8NfBXw2vzr/A7w2jwnxPP328Br8W/z2cDXALs8fy8NfDXwWjyn3wE+Gvhrnr/jwEcBn82/ze8Ar81zQjx/vw28Fv92twLvA/w2L9hHA5/NFZ8NfDUv2GsD3wU8mH+73wFem+eEeP5+G3gt/v1+GngfYJfn7zhX7PL8HQe+C3hr/v1+B3htnhPi+ftt4LX4j7ELfDbwNfzrfBTw2cBx/mP8DvDaPCfE8/fbwGvxH+u3gY8B/poX7qWBrwJem/9YvwO8Ns8J8fz9NvBa/Of4bOBrgF2e03Hgo4DP5j/H7wCvzXNCPH+/DbwW/3luBd4H+G2ueG3gu4AH85/nd4DX5jkhnr/fBl6L/3w/zRVvzX++3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFMuzQffOqdQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMail;
impl IconShape for MdMail {
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
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5HuC9eU6I5++3gdfiOf0N8NPAZ/E/2+cAbw28FM/pd4DX5jkhnr/fBl6L5/Q7wGsDLw18NfBa/M/yO8BHA38N/DbwWjyn3wFem+eEeP5+G3gtntPvAK/Ns3008NnAMf57XQI+G/hqnu23gdfiOf0O8No8J8Tz99vAa/Gcfgd4bZ7TceC7gbfiv8fPAO8N7PKcfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm+Xtt4LuBB/Ff4xnAewO/zfP328Br8Zx+B3htnhPi+ftt4LV4TrcCbwP8Nc/fceCjgc/iP9fnAF8N7PL8vTTwU8CDeU6/A7w2zwnx/P028Fo8f58NfA2wy/P30sBXA6/Ff6zfAT4a+Guev+PARwGfzfP3O8Br85wQz99vA6/FC3Yr8D7Ab/OCfTTw2cAx/n0uAZ8NfDUv2GsD3wU8mBfsd4DX5jkhnr/fBl6Lf9lPA+8D7PL8HQe+G3gr/m1+BnhvYJfn7zjwXcBb8y/7HeC1eU6I5++3gdfiRbMLfDbwNbxgrw18N/AgXjTPAN4b+G1esI8CPhs4zovmd4DX5jkhnr/fBl6Lf53fBj4G+Guev+PARwOfxQv3OcBXA7s8fy8NfBXw2vzr/A7w2jwnxPP328Br8W/z2cDXALs8fy8NfDXwWjyn3wE+Gvhrnr/jwEcBn82/ze8Ar81zQjx/vw28Fv92twLvA/w2L9hHA5/NFZ8NfDUv2GsD3wU8mH+73wFem+eEeP5+G3gt/v1+GngfYJfn7zhX7PL8HQe+C3hr/v1+B3htnhPi+ftt4LX4j7ELfDbwNfzrfBTw2cBx/mP8DvDaPCfE8/fbwGvxH+u3gY8B/poX7qWBrwJem/9YvwO8Ns8J8fz9NvBa/Of4bOBrgF2e03Hgo4DP5j/H7wCvzXNCPH+/DbwW/3luBd4H+G2ueG3gu4AH85/nd4DX5jkhnr/fBl6L/3w/zRVvzX++3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFMuzQffOqdQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkunread;
impl IconShape for MdMarkunread {
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
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FnAawPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/Zg4Gn87/TQ4BbecEQ/7LvBt6L/52+B3hvXjDEv+wicJz/nXaBE7xgiH+ZeV4fA/w1/7O8NPBVPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/OscB16KF83fALv867w28Fs8L/GCIf5l5nm9DvDb/Ou8NvBbvGheB/ht/nVeG/gtnpd4wRD/MvO8Xgf4bf51Xhv4LV40rwP8Nv86rw38Fs9LvGCIf5l5Xq8D/Db/Oq8N/BYvmtcBfpt/ndcGfovnJV4wxL/MPK/XAX6bf53XBn6LF83rAL/Nv85rA7/F8xIvGOJfZp7X6wC/zb/OawO/xYvmdYDf5l/ntYHf4nmJFwzxLzPP63WA3+Zf57WB3+JF8zrAb/Ov89rAb/G8xAuG+JeZ5/U6wG/zr/PawG/xonkd4Lf513lt4Ld4XuIFQ/zLzPN6HeC3ecEeDDyI5/TSwFfzovlo4K95Ts8AbuUFe23gt3he4gVD/MvM83od4Ld5wY4Dvw28FP8x/gZ4bWCXF+y1gd/ieYkXDPEvM8/rdYDf5oU7Dvw28FL8+/wN8NrALi/cawO/xfMSLxjiX2ae1+sAv82/7Djw28BL8W/zN8BrA7v8y14b+C2el3jBEP8y87xeB/htXjTHgd8GXop/nb8BXhvY5UXz2sBv8bzEC4b4l5nn9TrAb/OiOw78NvBSvGj+BnhtYJcX3WsDv8XzEi8Y4l9mntfrAL/Nv85x4LeBl+KF+xvgtYFd/nVeG/gtnpd4wRD/MvO8Xgf4bf71jgO/DbwUz9/fAK8N7PKv99rAb/G8xAuG+JeZ5/U6wG/zb3Mc+G3gpXhOfwO8NrDLv81rA7/F8xIvGOJfZp7X6wC/zb/dceC3gZfiir8BXhvY5d/utYHf4nmJFwzxLzPP63WA3+bf5zjw21zx2sAu/z6vDfwWz0u8YIh/mXlerwP8Nv9+x7lil3+/1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8y/j/j3Mf8+4gVD/MvMv8/LAH/Nv81LA3/Fv494wRD/MvPv8zHAV/Nv89HAV/HvI14wxL/M/PvcCjyEf5unAw/m30e8YIh/mfn3+xjgq/nX+Wjgq/j3Ey8Y4l9m/mO8DPDXvGheGvgr/mOIFwzxLzP/cT4a+BpeuI8Cvpr/OOIFQ/zLzH+sW4GvBn4H+GuueGngtYCPBh7MfyzxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mt+Gngr/nf6HuC9ecEQ/7KXBv6K/50eAtzKC4Z40bw38F387/I+wHfzwiFedC8NfDTw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHHFa3QV4TQWoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoveToInbox;
impl IconShape for MdMoveToInbox {
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
                d: "M19 3H4.99c-1.11 0-1.98.9-1.98 2L3 19c0 1.1.88 2 1.99 2H19c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 12h-4c0 1.66-1.35 3-3 3s-3-1.34-3-3H4.99V5H19v10zm-3-5h-2V7h-4v3H8l4 4 4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3Uc+CjgvYEH869zK/DdwNcAu/znQPzneTDwW8CD+ff5a+BtgFv5j4f4z/NXwEvzH+OvgZfhPx7iP8dbAz/Ff6y3AX6a/1iI/xyfDXwW/7E+B/hs/mMh/nP8NvBaPKe/AT6aF81XAy/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4V40bw0cIwX3VcDL81z+mvgo3nRfDXw0jynvwY+mhfdJeCveeEQL9xrA98FPJj/nW4F3gf4bZ4/xAv23sB38X/D+wDfzfNCPH/HgacDx/m/YRd4CLDLc0I8fx8NfBX/t3wM8NU8J8Tz993Ae/F/y/cA781zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/Of4G+Gjgp4Fj/Nf5HeC1eU6I5++3gdfiP97fAK8N7AIvDfw2cIz/Gr8DvDbPCfH8/TbwWvzH2gUeAuzybC8N/DZwjP98vwO8Ns8J8fz9NvBa/Mf7buB9eE4vDfw2cIz/XL8DvDbPCfH8/TbwWvzn+G7gfXhOLw38NnCM/zy/A7w2zwnx/P028Fr85/lu4H14Ti8N/DZwjP8cvwO8Ns8J8fz9NvBa/Of6buB9eE4vDfw2cIz/eL8DvDbPCfH8/TbwWvzn+27gfXhOLw38NnCM/1i/A7w2zwnx/P028Fr81/hu4H14Ti8N/DZwjP84vwO8Ns8J8fz9NvBa/Nf5buB9eE4vDfw2cIz/GL8DvDbPCfH8/TbwWvzX+hzgs3lOrw38Fv8xfgd4bZ4T4vn7beC1+K/zN8BrA7s823Hgt4CX5j/G7wCvzXNCPH+/DbwW/zX+BnhtYJdnOw78FvDS/Mf5HeC1eU6I5++3gdfiP9/fAK8N7PJsx4HfAl6a/1i/A7w2zwnx/P028Fr85/ob4LWBXZ7tOPBbwEvzH+93gNfmOSGev98GXov/PH8DvDawy7MdB34LeGn+c/wO8No8J8Tz99vAa/Gf42+A1wZ2ebbjwG8BL81/nt8BXpvnhHj+fht4Lf7j/Q3w2sAuz3Yc+C3gpfnP9TvAa/OcEM/fbwOvxX+sXeAhwC7Pdhz4LeCl+c/3O8Br85wQz99vA6/Ff7zvBt6HK44DvwW8NP81fgd4bZ4T4vn7beC1+M/x3cDHAL8FvDT/dX4HeG2eE+L5+23gtfjPswsc57/W7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99PAW/F/y88Ab81zQjx/nw18Fv+3fA7w2TwnxPP30sBf8X/LQ4BbeU6IF+yrgY/i/4avAT6a54V44b4a+Cj+d/sa4KN5/hD/spcG3hp4aeA4/zvsAn8NfDdwKy8Y4v83xP9viP/fEP+/If5/4x8B75CwQf5SIuIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNextWeek;
impl IconShape for MdNextWeek {
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
                d: "M20,7h-4V5c0-0.55-0.22-1.05-0.59-1.41C15.05,3.22,14.55,3,14,3h-4C8.9,3,8,3.9,8,5v2H4C2.9,7,2,7.9,2,9v11 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9C22,7.9,21.1,7,20,7z M10,5h4v2h-4V5z M11,18.5l-1-1l3-3l-3-3l1-1l4,4L11,18.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP8x7oE/DX/tRD/Nr8NvBb/8R4C3Mp/HcS/zW8Dr8V/vO8B3pv/Ooh/m98GXov/HA8BbuW/BuLf5reB1+I/x/cA781/DcS/zW8Dr8V/nocAt/KfD/Fv89vAa/Gcfgd4bf71fht4Lf5jvQ7w2/zLEP82vw28Fs/pd4DX5l/vtYHf4j/W6wC/zb8M8W/z28Br8Zx+B3ht/m1+G3gt/uO8DvDb/MsQ/za/DbwWz+l3gNfm3+a1gd/iP87rAL/Nvwzxb/PbwGvxnH4HeG3+7X4beC3+Y7wO8Nv8yxD/Nr8NvBbP6XeA1+bf7sHAg/nX+y2e1+sAv82/DPFv89vAa/Gcfgd4bf7rmef1OsBv8y9D/Nv8NvBaPKffAV6b/3rmeb0O8Nv8yxD/Nr8NvBbP6XeA1+a/nnlerwP8Nv8yxL/NbwOvxXP6HeC1+a9nntfrAL/Nvwzxb/PbwGvxnH4HeG3+65nn9TrAb/MvQ/zb/DbwWjyn3wFem/965nm9DvDb/MsQ/za/DbwWz+l3gNfmv555Xq8D/Db/MsS/zW8Dr8Vz+h3gtfmvZ57X6wC/zb8M8W/z28Br8Zx+B3ht/uuZ5/U6wG/zL0P82/w28Fo8p98BXpv/euZ5vQ7w2/zLEP82vw28Fv+xbgUewr+eeV6vA/w2/zLEv81vA6/Ff7z3Ab6bfx3zvF4H+G3+ZYh/m98GXov/eLcCD+Ffxzyv1wF+m38Z4t/mt4HX4j/H+wDfzYvOPK/XAX6bfxni3+a3gdfiP8etwEN40Znn9TrAb/MvQ/zb/DbwWjynvwE+mn+97wYexHP6a2CXF81r87xeB/ht/mWIf5vfBl6L5/Q7wGvzr/fewHfxH+t1gN/mX4b4t/lt4LV4Tr8DvDb/NrcCD+I/zusAv82/DPFv89vAa/Gcfgd4bf5t3hv4Lv7jvA7w2/zLEP82vw28Fs/pd4DX5t/uVuBB/Md4HeC3+Zch/m1+G3gtntPvAK/Nv91bAx/Nf4yPBv6afxni3+a3gdfiOf0O8Nr874L4t/lt4LV4Tr8DvDb/uyD+bX4beC2e0+8Ar83/Loh/m98GXovn9DvAa/O/C+Lf5reB1+I5/Q7w2vzvgvi3+W3gtXhOvwO8Nv+7IP5tfht4LZ7T7wCvzf8uiH+b3wZei+f0O8Br878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE6/A7w2/7sg/m1+G3gtntPvAK/N/y6If5vfBl6L5/Q7wGvzvwvi3+a3gdfiOf0O8Nr874L4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKReJRBKDiv2AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutlinedFlag;
impl IconShape for MdOutlinedFlag {
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
                d: "M14 6l-1-2H5v17h2v-7h5l1 2h7V6h-6zm4 8h-4l-1-2H7V6h5l1 2h5v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPolicy;
impl IconShape for MdPolicy {
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
                d: "M21,5l-9-4L3,5v6c0,5.55,3.84,10.74,9,12c2.3-0.56,4.33-1.9,5.88-3.71l-3.12-3.12c-1.94,1.29-4.58,1.07-6.29-0.64 c-1.95-1.95-1.95-5.12,0-7.07c1.95-1.95,5.12-1.95,7.07,0c1.71,1.71,1.92,4.35,0.64,6.29l2.9,2.9C20.29,15.69,21,13.38,21,11V5z",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "3",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Md4aeCvgwfzXuBX4GeCn+fdB/Pt9NvBZ/Pf4HOCz+bdD/PtdBI7z3+NW4CH82yH+/XaBY/z3eAbwYP7tEP9+nw18Fv89Pgf4bP7tEP8x3hp4a+DBXPHSwDH+Y10C/porbgV+Gvhp/n0Q/zl+G3gt/mP9DvDa/MdC/Of4beC1+I/1O8Br8x8L8Z/jt4HX4j/W7wCvzX8sxH+O3wZei/9YvwO8Nv+xEP85fht4Lf5j/Q7w2vzHQvzn+G3gtfiP9TvAa/MfC/Gf47eB1+I/1u8Ar81/LMR/jt8GXov/WL8DvDb/sRD/OX4beC3+Y/0O8Nr8x0L85/ht4LX4j/U7wGvzHwvxn+O3gdfiP9bvAK/NfyzEf47fBl6L/1i/A7w2/7EQ/zl+G3gt/mP9DvDa/MdC/Of4beC1+I/1O8Br8x8L8Z/jr4CX5j/WXwMvw38sxH+8BwNP5z/HQ4Bb+Y+D+I/32cBn8Z/jc4DP5j8O4j/Wg4G/Ao7zn2MXeBngVv5jIP7jHAd+C3hp/nP9NfA6wC7/foj/GK8NfBfwYP5r/DXwMcBv8++D+Ld7MPBawGsD781/j+8Gfhr4G+BW/vUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mpYHj/Nt9NfBSPKfvAb6bf7td4K/510H89/ht4LV4Tp8DfDb/tRD/PX4beC2e0+cAn81/LcR/j98GXovn9DnAZ/NfC/Hf47eB1+I5fQ7w2fzXQvz3+G3gtXhOnwN8Nv+1EP89fht4LZ7T5wCfzX8txH+P3wZei+f0OcBn818L8d/jt4HX4jl9DvDZ/NdC/Pf4beC1eE6fA3w2/7UQ/z1+G3gtntPnAJ/Nfy3Ef4/fBl6L5/Q5wGfzXwvx3+O3gdfiOX0O8Nn810L89/ht4LV4Tp8DfDb/tRD/PX4beC2e0+cAn81/LcR/j98GXovn9DnAZ/NfC/Hf47eB1+I5fQ7w2fzXQvz3+G3gtXhOnwN8Nv+1EP89fht4LZ7T5wCfzX8txH+PlwZ+GzjGFZeA1wb+mv9aiP8+Lw38Nle8NvDX/NdD/Pd6aa74a/57IP5/Q/z/hvj/DfH/G+L/N/4REml3QaEKCE8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPushPin;
impl IconShape for MdPushPin {
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
                d: "M16,9V4l1,0c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7C6.45,2,6,2.45,6,3v0 c0,0.55,0.45,1,1,1l1,0v5c0,1.66-1.34,3-3,3h0v2h5.97v7l1,1l1-1v-7H19v-2h0C17.34,12,16,10.66,16,9z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD7UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v+P48Auzwnx/8NLA58NvDXPCfF/30sDvwX8DfDaPCfEf40HAw8CXptne2lgF7iVK24FbgV+h/84Lw38FnAc+B3gtXlOiP8cDwbeCnhp4LWBB/Ov89fAbwPfA/w1/zYvDfwWcJwrfgd4bZ4T4j/WawMfBbw1/3FuBb4a+B5glxfNSwO/BRzn2X4HeG2eE+I/xlsDXwU8mP88u8BXA18D7PKCvTTwW8BxntPvAK/Nc0L8+zwY+C7gtfmvswu8D/DTPK+XBn4LOM7z+h3gtXlOiH+7zwI+GjjOf4+fBt4H2OWKlwZ+CzjO8/c7wGvznBD/eseB7wLemv9+u8DrcMVvAcd5wX4HeG2eE+Jf5zjwW8BL86K7BPw28NvAX3PFb/Nsr80VDwZeG3hr4Bj/OrvAcV643wFem+eEeNG9NPBTwIN50fwM8NXAb/Ov99LAewPvDRzjP8bvAK/Nc0K8aI4DvwW8NP+yrwG+GriVf7/jwEcDHw0c49/nd4DX5jkhXjR/Bbw0L9zfAB8N/Db/8Y4DPw28Fv92vwO8Ns8J8S/7LuC9eeG+B3hv/vN9NPBV/Nv8DvDaPCfEC/fWwE/xwn0M8NX813hp4LeA4/zr/Q7w2jwnxAt2HPgr4MG8YO8DfDf/NV4a+C3gOP82vwO8Ns8J8YJ9NfBRvGCfA3w2/zVeGvgt4Dj/dr8DvDbPCfH8PRh4Oi/Y9wDvzX+NlwZ+CzjOv8/vAK/Nc0I8f98NvBfP3yXgwcAu//leGvgt4Dj/fr8DvDbPCfG8jgNPB47z/L0N8NP853tp4LeA4/zH+B3gtXlOiOf10cBX8fz9DvDa/Od7aeC3gOP8x/kd4LV5Tojn9dXAR/H8vQ3w0/znemngt4Dj/Mf6HeC1eU6I5+848NbAWwNvxRXPAB7Mf66XBn4LOM5/vN8BXpvnhPiXHQfeGtgFfpr/PMeB7waO85/jr4GP5jkh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ESwbb0HggndFAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRedo;
impl IconShape for MdRedo {
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
                d: "M18.4 10.6C16.55 8.99 14.15 8 11.5 8c-4.65 0-8.58 3.03-9.96 7.22L3.9 16c1.05-3.19 4.05-5.5 7.6-5.5 1.95 0 3.73.72 5.12 1.88L13 16h9V7l-3.6 3.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABsUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/Nl8NvBT/s/wN8NH86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+argZfmf5a/Bj6afx3E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKTlwxBgmdSFgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemove;
impl IconShape for MdRemove {
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
                d: "M19 13H5v-2h14v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5v+UEsMtzQjx/rw38Fv+3vA7w2zwnxPP30cBX8X/LxwBfzXNCPH+fDXwW/zp/A3w0/zW+Gngp/nU+B/hsnhPi+fts4LP41/kd4LX5r/HbwGvxr/M5wGfznBDP328Dr8W/zu8Ar81/jd8GXot/nZ8B3prnhHj+fht4Lf51fgd4bf5r/DbwWvzr/Azw1jwnxPP32cBn8a/zO8Br81/jt4HX4l/nc4DP5jkhnr/PBj6Lf52/Bj6a/xpfDbw0/zqfA3w2zwnx/H008FX83/IxwFfznBDP32sDv8X/La8D/DbPCfH8PRh4Ov+3nAB2eU6IF+yvgZfi/4a/AV6a54V4wb4a+Cj+b/ga4KN5XogX7LWB3+L/htcBfpvnhXjhbgUexP9uzwAezPOHeOE+G/gs/nf7HOCzef4QL9xx4FbgGP87XQIeDOzy/CH+ZZ8NfBb/O30O8Nm8YIh/2XHgVuAY/7tcAh4M7PKCIV407w18F/+7vA3w07xwiBfdTwNvxf8OPwO8Nf8yxIvuOPDXwIP4n+0ZwEsDu/zLEP86Lw38NnCM/5kuAa8N/DUvGsS/3ksDvw0c43+WS8BrA3/Niw7xb/PSwG8Dx/if4RLw2sBf86+D+Ld7aeCngQfx3+sZwFsDf82/HuLf5zjw3cBb8d/jZ4D3Bnb5t0H8x3hv4KuBY/zXuAS8N/DT/Psg/uMcBz4a+GjgGP85LgFfDXw1sMu/H+I/3nHgo4H3Bh7Ef4xnAN8NfDWwy38cxH+u1wbeGnht4KX41/kb4LeBnwZ+m/8ciP86x4GXBl4aOM4VL80Vf80Vu8BfA38N7PKfD/H/G+L/N8T/b4j/3xD/v/GPXU2cQW9x8/wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveCircle;
impl IconShape for MdRemoveCircle {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11H7v-2h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveCircleOutline;
impl IconShape for MdRemoveCircleOutline {
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
                d: "M7 11v2h10v-2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3XeG/hu/mdB/Nf4LuAhwGvzPwviP993Ae8N/A7w2vzPgvjP9V3Ae3PF7wCvzf8siP883wW8N8/2O8Br8z8L4j/HdwHvzXP6HeC1+Z8F8R/vu4D35nn9DvDa/M+C+I/1XcB78/z9DvDa/M+C+I/zXcB784L9DvDa/M+C+I/xXcB788L9DvDa/M+C+Pf7LuC9+Zf9DvDa/M+C+Pf5LuC9edH9NXAr8NfAbwO/w38vxL/ddwHvzb/fTwM/DfwMsMt/LcS/zXcB781/vO8Gvgf4bf5rIP71vgt4b/5z/TbwOcBv858L8a/zXcB781/nt4H3AW7lPwfiRfddwHvz3+Ozgc/hPx7iRfNdwHvz3+u3gbcBdvmPg/iXfTXwUfzPsAu8DvDX/MdA/MveG/gu/ufYBV4H+Gv+/RAvmvcGvov/OXaB1wH+mn8fxIvuvYHv4n+OXeB1gL/m3w7xr/PewHfxP8cu8BBgl38bxL/eewPfxf8cfw28DP82iH+b9wa+i/85Pgf4bP71EP927w18F/9zvAzw1/zrIP593hv4Ll40l4C/5tleGjjGf5y/Bl6Gfx3Ev997A9/Fv+x3gNfm+Xtt4Djw0sBLA68NHONf732A7+ZFh/iP8d7Ad/HC/Q7w2rzoXhp4beCtgdfiRbMLnOBFh/iP897Ad/GC/Q7w2vzbPBh4b+C9gQfxwr0P8N28aBD/sd4b+C6ev98BXpt/v/cGvho4xvN3K/AQXjSI/3jvDXwXz+t3gNfmP8Zx4KOBjwaO8bzeBvhp/mWI/xzvDXwXz+l3gNfmP9aDgZ8GXorn9NvA6/AvQ/zneW/gu3i23wFem/8cXw18FM/pIcCtvHCI/1zvDXwXV/wO8Nr853lv4Lt4to8BvpoXDvGf772B7wJ+B3ht/nO9NfDdwDHgr4GX4YVD/Nd4b+C9gdfmP99LA78NHANOALu8YIj/Oi8N/DX/Nd4a+CngbYCf5gVD/N/10cCDgY/mBUP83/bRwFfzgiH+bzsO7PKCIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/ArB8cUGf50lwAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReply;
impl IconShape for MdReply {
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
                d: "M10 9V5l-7 7 7 7v-4.1c5 0 8.5 1.6 11 5.1-1-5-4-10-11-11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3HeG/hu/u3eG/hu/msh/mN8F/AQ4LX5t/ku4CHAa/NfC/Hv913AewO/A7w2/3rfBbw38DvAa/NfC/Hv813Ae3PF7wCvzb/OdwHvzRW/A7w2/7UQ/3bfBbw3z/Y7wGvzovsu4L15tt8BXpv/Woh/m+8C3pvn9DvAa/Oi+S7gvXlOvwO8Nv+1EP963wW8N8/rd4DX5l/2XcB787x+B3ht/msh/nW+C3hvnr/fAV6bF+67gPfm+fsd4LX5r4V40X0X8N68YO8DfDcv2HcB780L9jvAa/NfC/Gi+S7gvXnB3gf4bl6w7wLemxfud4DX5r8W4l/2XcB784K9D/DdvGDfBbw3/7LfAV6b/1qIF+67gPfmBXsf4Lt5wb4LeG9edH8N3Ar8NfDbwO/wnwvxgn0X8N68YO8DfDcv2HcB782/308DPw38DLDLfyzE8/ddwHvzgr0P8N28YN8FvDf/8b4b+B7gt/mPgXhe3wW8Ny/Y+wDfzQv2XcB785/rt4HPAX6bfx/Ec/ou4L15wd4H+G5esO8C3pv/Or8NvA9wK/82iGf7LuC9ecHeB/huXrDvAt6b/x6fDXwO/3qIK74LeG9esPcBvpsX7LuA9+a/128DbwPs8qJDwFcDH8UL9j7Ad/OCfTXwUfzPsAu8DvDXvGgQ8N7Ad/GCfTfwPrxg7w18F/9z7AKvA/w1/zLEFe8NfBcv2HcD78ML9t7Ad/E/xy7wOsBf88Ihnu29ge/iBftu4H14wd4b+C7+59gFXgf4a14wxHN6b+C7eMG+G3gfXrD3Br6L/zl2gYcAuzx/iOf13sB38YJ9N/A+vGDvDXwX/3P8NfAyPH+I5++9ge/iBftu4H14wd4b+C7+5/gc4LN5XogX7L2B7+IF+27gfXjB3hv4Lv7neBngr3lOiBfuvYHv4gX7buB9eMHeG/guXjSXgL/m2V4aOMZ/nL8GXobnhPiXvTfwXbxg3w28Dy/YewPfxb/sd4DX5vl7beA48NLASwOvDRzjX+99gO/m2RAvmvcGvosX7LuB9+EFe2/gu3jhfgd4bV50Lw28NvDWwGvxotkFTvBsiBfdewPfxQv2NcBH84K9N/BdvGC/A7w2/zYPBt4beG/gQbxw7wN8N1cg/nXeG/gunr/fAV6bF+69ge/i+fsd4LX593tv4KuBYzx/twIP4QrEv957A9/F8/od4LX5l7038F08r98BXpv/GMeBjwY+GjjG83ob4KcBxL/NewPfxXP6HeC1edG8N/BdPKffAV6b/1gPBn4aeCme028DrwMg/u3eG/gunu13gNfmRffewHfxbL8DvDb/Ob4a+Cie00OAW8W/z3sD38UVvwO8Nv867w18F1f8DvDa/Od5b+C7eLaPAb5a/Pu9N/BdwO8Ar82/3nsD3wX8DvDa/Od6a+C7gWPAXwMvI/5jvDfw3sBr82/z3sB7A6/Nf76XBn4bOAacEP9xXhr4a/7tXhr4a/5rvDXwU8DbiP+/Php4sPj/7aPF/2/Hxf9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHXgq1sfLu+EgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplyAll;
impl IconShape for MdReplyAll {
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
                d: "M7 8V5l-7 7 7 7v-3l-4-4 4-4zm6 1V5l-7 7 7 7v-4.1c5 0 8.5 1.6 11 5.1-1-5-4-10-11-11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/HewHfxH+t9gO/mPxbiP957A9/Ff473Ab6b/ziI/1jvDXwX/7neB/hu/mMg/uO8N/Bd/Nd4H+C7+fdD/Md4b+C7+K/1PsB38++D+Pd7b+C7+O/xPsB382+H+Pd5b+C7+O/1PsB382+D+Ld7b+C7+J/hfYDv5l8P8W/z3sB38T/L+wDfzb8O4l/vvYHv4n+m9wG+mxcd4l/nvYHv4t/vb4BdntNx4KX493sf4Lt50SBedO8NfBf/MV4H+G2e02sDv8V/jPcBvpt/GeJF897Ad/Ef53WA3+Y5vTbwW/zHeR/gu3nhEP+y9wa+i/9YrwP8Ns/ptYHf4j/W+wDfzQuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP99HAX/OcXhr4av7jiRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/Ob4H+G6ueG/gvfjPIV4wxL/M/Md7H+C7eU7vDXwX//HEC4b4l5n/WH8DvDTP318DL8V/LPGCIf5l7w18F/9xvgb4aJ6/rwY+iv847wN8Ny8Y4kXz3sB38R/jZ4C35vn7beC1+I/xPsB388IhXnTvDXwX/367wOsAf81zemngr/iP8T7Ad/MvQ/zrvDfwXfz73Qq8D/DbXPHawHcBD+bf732A7+ZFg/jXe2/gu/if6X2A7+ZFh/i3eW/gu/if5X2A7+ZfB/Fv997Ad/E/w/sA382/HuLf572B7+K/1/sA382/DeLf772B7+K/x/sA382/HeI/xnsD38V/rfcBvpt/H8R/nPcGvov/Gu8DfDf/foj/WO8NfBf/ud4H+G7+YyD+47038F3853gf4Lv5j4P4z/HewHfxH+t9gO/mPxbi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjF9H5BeNPeLQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReport;
impl IconShape for MdReport {
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
                d: "M15.73 3H8.27L3 8.27v7.46L8.27 21h7.46L21 15.73V8.27L15.73 3zM12 17.3c-.72 0-1.3-.58-1.3-1.3 0-.72.58-1.3 1.3-1.3.72 0 1.3.58 1.3 1.3 0 .72-.58 1.3-1.3 1.3zm1-4.3h-2V7h2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReportOff;
impl IconShape for MdReportOff {
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
                d: "M11 7h2v2.92l6.91 6.91 1.09-1.1V8.27L15.73 3H8.27L7.18 4.1 11 7.92zm11.27 14.73l-20-20.01L1 2.99l3.64 3.64L3 8.27v7.46L8.27 21h7.46l1.64-1.63L21 23l1.27-1.27zM12 17.3c-.72 0-1.3-.58-1.3-1.3s.58-1.3 1.3-1.3 1.3.58 1.3 1.3-.58 1.3-1.3 1.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+e/118DrALv8+yBeNO8NfBf/s/w18DrALv92iH/ZSwN/xf9Mfw28DrDLvw3iX/bdwHvxP9dfA68D7PKvh/iXXQSO8z/bXwOvA+zyr4P4l5nn9THAX/Nf47d40fw18DrALi86xL/MPK/XAX6b/xrmRffXwOsAu7xoEP8y87xeB/ht/muYf52/Bl4H2OVfhviXmef1OsBv81/D/Ov9NfA6wC4vHOJfZp7X6wC/zX8N82/z18DrALu8YIh/mXlerwP8Nv81zL/dXwMvwwuG+JeZ5/U6wG/zX8P8+4gXDPEvM8/rdYDf5r+G+fcRLxjiX2ae1+sAv81/DfPvI14wxL/MPK/XAX6b/xrm30e8YIh/mXlerwP8Nv81zL+PeMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvOf42+AXa44DrwU/znEC4b4l5n/OJeArwa+G7iV5/Rg4L2BjwaO8R9HvGCIf5n5j/E3wFsDt/LCPRj4aeCl+I8hXjDEv8z8+/0N8NrALi+a48BfAw/i30+8YIh/mfn3exngr/nXeW3gt/j3Ey8Y4l9m/n2+B3hv/m2+G3gv/n3EC4b4l5l/n7cBfpp/m7cGfop/H/GCIf5l5t/nIcCt/Ns8GHg6/z7iBUP8y8y/j/j3Mf8+4gVD/MvMv4/4tzsOXOTfR7xgiH+Z+fd5HeC3+bd5beC3+PcRLxjiX2b+fb4G+Gj+bb4a+Cj+fcQLhviXmX+fXeAhwC7/OseBpwPH+fcRLxjiX2b+/b4beB/+db4LeG/+/cQLhviXmf8Y3w28Dy+a7wLem/8Y4gVD/MvMf5zfBt4HuJXn76WBrwJem/844gVD/MvMf7y/Bn4auJUrHgy8NfDS/McTLxjiX7YLHON/p0vAcV4wxL/su4H34n+n7wHemxcM8S97aeCv+N/pIcCtvGCIF817A9/F/y7vA3w3LxziRfdg4LOBtwaO8T/TJeCngc8GbuVfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AvubiUHmS0ztAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSave;
impl IconShape for MdSave {
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
                d: "M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4j/HSwM/BbwN8Nc8r98GXovn9DvAa/O8Xhr4KeBtgL/mPxbiP95LA78FHAd2gdcB/prn9NvAa/Gcfgd4bZ7TSwO/BRwHdoHXAf6a/ziI/1gvDfwWcJxn2wVeB/hrnu23gdfiOf0O8No820sDvwUc59l2gdcB/pr/GIj/OMeBpwPHeV67wOsAf80Vvw28Fs/pd4DX5oqXBn4LOM7z2gUeAuzy74f4j/XewHfx/O0CrwP8NfDbwGvxnH4HeG3gpYHfAo7z/L0P8N38x0D8y8zzeh3gt3n+3hv4Lp6/XeB1gK8GXovn9DvARwO/BRzn+Xsf4Lt5/l4b+C2el3jBEP8y87xeB/htXrD3Br6L528X2AUezHO6FTgOHOf5ex/gu3nBXhv4LZ6XeMEQ/zLzvF4H+G1euPcGvov/GO8DfDcv3GsDv8XzEi8Y4l9mntfrAL/Nv+y9ge/i3+d9gO/mX/bawG/xvMQLhviXmef1OsBv86J5b+C7+Ld5H+C7edG8NvBbPC/xgiH+ZeZ5vQ7w27zo3hv4Lv513gf4bl50rw38Fs9LvGCIf5l5Xq8D/Db/Ou8NfBcvmvcBvpt/ndcGfovnJV4wxL/MPK/XAX6bf733Br6LF+59gO/mX++1gd/ieYkXDPEvM8/rdYDf5t/mvYHv4vl7H+C7+bd5beC3eF7iBUP8y8zzeh3gt/m3e2/gu3hO7wN8N/92rw38Fs9LvGCIf5l5Xq8D/Db/Pu8NfBdXvA/w3fz7vDbwWzwv8YIh/mXmeb0O8Nv8+703cBz4av79Xhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/ro4G/5n+Wlwa+muclXjDEv2wXOMb/TpeA47xgiH/ZdwPvxf9O3wO8Ny8Y4l/2YODp/O/0EOBWXjDEi+a9ge/if5f3Ab6bFw7xonsw8NnAWwPH+J/pEvDTwGcDt/IvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAVgDrkG0VxqeAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSaveAlt;
impl IconShape for MdSaveAlt {
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
                d: "M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSelectAll;
impl IconShape for MdSelectAll {
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
                d: "M3 5h2V3c-1.1 0-2 .9-2 2zm0 8h2v-2H3v2zm4 8h2v-2H7v2zM3 9h2V7H3v2zm10-6h-2v2h2V3zm6 0v2h2c0-1.1-.9-2-2-2zM5 21v-2H3c0 1.1.9 2 2 2zm-2-4h2v-2H3v2zM9 3H7v2h2V3zm2 18h2v-2h-2v2zm8-8h2v-2h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2zm0-12h2V7h-2v2zm0 8h2v-2h-2v2zm-4 4h2v-2h-2v2zm0-16h2V3h-2v2zM7 17h10V7H7v10zm2-8h6v6H9V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/n4a+Cvge4Bb+b8L8fz9NvBaXPHdwPcAv83/PYjn77eB1+I5/TXw1cD38H8H4vn7beC1eP52ga8Gvge4lf/dEM/fbwOvxb/su4HvAX6b/50Qz99vA6/Fi+6vga8Gvof/XRDP328Dr8W/3i7w1cD3ALfyPx/i+ftt4LX49/lu4HuA3+Z/LsTz99vAa/Ef46+Brwa+h/95EM/fbwOvxX+sXeCrge8BbuV/BsTz99vAa/Gf57uB7wF+m/9eiOfvt4HX4j/fXwNfDXwP/z0Qz99vA6/Ff51d4KuB7wFu5b8O4vn7beC1+O/x3cD3AL/Nfz7E8/fbwGvx3+uvga8Gvof/PIjn77eB1+J/hl3gq4HvAW7lPxbi+ftt4LX4n+e7ge8Bfpv/GIjn77eB1+J/rr8Gvhr4Hv59EM/fVwMvDbwW/7PtAl8NfA9wK/96iH/ZceClgePASwMPBh4MPBh4EP9zfDfwPcBv86JD/Pu9NHAceGngOPDaXPFa/Pf4a+Crge/hX4b4z3UceGngOPDSwIOBBwMPBh7Ef65bge8GvgbY5flD/Pd6aeA48NLAceC1ueK1+I/13cDXAH/Nc0L8z3UceGngOPDSwIOBBwMPBh7Ev81vA98NfA9XIP7nOg68FHAceGngwcCDgQcDD+bf5neA7wa+mysQ/71eGjgGvDRwHHhtrnht/mN9D/DVwF/znBD/uY4DLwUcB14aeDDwYODBwIP5z3UJ+Grgq4Fdnj/Ev99LA8eAlwaOA6/NFa/Nf4+/Ab4a+G7+ZYh/2XHgpYDjwEsDDwYeDDwYeDD/c3wP8N3Ab/OiQzx/Xw28FPDa/M92Cfhq4LuBW/nXQzx/vw28Fv9z/Q3w1cB38++DeP5+G3gt/uf5HuC7gd/mPwbi+ftt4LX4n+ES8NXAdwO38h8L8fz9NvBa/Pf6G+Crge/mPw/i+ftt4LX47/E9wHcDv81/PsTz99vAa/Ff5xLw1cB3A7fyXwfx/P028Fr85/sb4KuB7+a/B+L5+23gtfjP8z3AdwO/zX8vxPP328Br8R/rEvDVwHcDt/I/A+L5+23gtfiP8TfAVwPfzf88iOfvt4HX4t/ne4DvBn6b/7kQz99vA6/Fv94l4KuB7wZu5X8+xPP328Br8aL7G+Crge/mfxfE8/fbwGvxL/se4LuB3+Z/J8Tz99vAa/H8XQK+Gvhu4Fb+d0M8f78NvBbP6W+Arwa+m/87EM/fbwOvxRXfA3w38Nv834N4/n4a+Gvgu4Fb+b8L8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EPM+eQS1PYIwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSend;
impl IconShape for MdSend {
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
                d: "M2.01 21L23 12 2.01 3 2 10l15 2-15 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3ht/nP9NvDdwPfw74f49zkOvBfw0cCD+a91K/DVwPcAu/zbIP5tHgx8FPDewHH+e+0C3w18DXAr/zqIf53XBt4LeG/+Z/pu4HuA3+ZFg3jRvBfw3sBr87/DbwPfDXwPLxziX/Z04MH873Qr8BBeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/7K+Bl+J/p98BXpsXDPEv+23gtfjf6XeA1+YFQ/zLPhv4LP53+hzgs3nBEP+y9wa+i/+d3gf4bl4wxL/spYG/4n+nlwH+mhcM8aLZBY7xv8sl4DgvHOJF89PAW/G/y/cA780Lh3jRvDfwXfzv8j7Ad/PCIV40x4GL/O9yAtjlhUO86L4beC/+d/ge4L35lyFedK8N/Bb/O7wO8Nv8yxD/Or8NvBb/s/0O8Nq8aBD/Om8N/BT/s70N8NO8aBD/er8NvBb/M/0O8Nq86BD/eq8N/Bb/M70O8Nu86BD/Nl8NfBT/s3wN8NH86yD+bY4Dfw08iP8ZngG8NLDLvw7i3+61gd/if4bXAX6bfz3Ev89HA1/Ff6+PAb6afxvEv993A+/Ff4/vAd6bfzvEf4zvBt6L/1rfA7w3/z6I/xjHgZ8GXov/Gr8DvDb/foj/WN8NvBf/ub4HeG/+YyD+43028Fn85/gc4LP5j4P4z/HWwHcDx/iPcQl4a+C3+Y+F+M9zHPhu4K349/kZ4L2BXf7jIf7zvTbw3cCD+Nd5BvDewG/znwfxX+e9gY8GXooX7m+Arwa+m/98iP96Lw18NPDawIO44hnAbwNfDfw1/3UQ/71emiv+mv8eiP/fEP+/8Y9swnlBAQzHVwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShield;
impl IconShape for MdShield {
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
                d: "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABzElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8S/7Lf57vQ7/eRD/MvPfS/znQfzLzH8v8Z8H8S8z/73Efx7Ev8z89xL/eRD/st/mv9dr858H8f8b4v83xP9viP/fEP+/If5/Q/z/hviXvRb/Pr/D/1yIf5n59xH/cyH+ZebfR/zPhfiXmX8f8T8X4l9m/n3E/1yIf5n59xH/cyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9EUBJBJRX3kgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSort;
impl IconShape for MdSort {
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
                d: "M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcc2OX/FsSL7rWB9wLeh/87EC+61wZ+C/hu4H34vwHxontt4Le44ruB9+F/P8SL7rWB3+LZvht4H/53Q7zoXhv4LZ7TdwPvw/9eiBfdawO/xfP6buB9+N8J8aJ7beC3eF7vA3w3/zshXnSvDfwWz+l9gO/miuPALv+7IF50rw38Fs/2McBXc8Vx4LeAvwbeh/89EC+61wZ+i2f7a+B1uOK3gJfmiu8G3of/HRAvutcGfovn9Ndc8dI8p+8G3of/+RAvutcGfosX3XcD78P/bIgX3WsDv8W/zncD78P/XIgX3WsDv8W/zvsA383/XIgX3WsDv8WL7n2A7+aK48Au//MgXnSvDfwWL5qPAb6aK44DvwX8NfA+/M+CeNG9NvBbvGj+Gngdrvgt4KW54ruB9+F/DsSL7rWB3+JF99dc8dI8p+8G3of/GRAvutcGfov/GN8NvA///RAvutcGfosX7G+44qV40Xw38D7890K86F4b+C1esN8B3hr4beCl+Jd9D/De/PdCvOheG/gtXrDfAV4bOA78NvBSvGDfA7w3//0QL7rXBn6LF+x3gNfmiuPAbwMvxfP6HuC9ebbv4or34b8e4kX32sBv8YL9DvDaPNtx4LeBl+I5/TXwOsAu8F3Ae3PFdwPvw38txIvutYHf4gX7HeC1eU7Hgd8GXorn9NfAXwPvzXP6buB9+K+DeNG9NvBbvGC/A7w2z+s48NvAS/Gi+W7gffivgXjRvTbwW7xgvwO8Ns/fceC3gZfiRfPdwPvwnw/xontt4Ld4wX4HeG1esOPAbwMvxb/sb4DXBnb5z4V40b028Fu8YL8DvDYv3HHgt4GX4gX7G+C1gV3+8yFedK8N/BYv2O8Ar82/7Djw28BL8bz+BnhtYJcr3ht4LeB9+M+BeNG9NvBbvGC/A7w2L5rjwG8DL8Vz+mvgdYBd4L2B7+KK7wbeh/94iBfdawO/xQv2O8Br86I7Dvw28FI8p78Gvhv4ap7TdwPvw38sxIvutYHf4gX7HeC1+dc5Dvw28FK8aL4beB/+4yBedK8N/BYv2O8Ar82/3nHgt4GX4kXz3cD78B8D8aJ7beC3eMF2gb/m3+Y48NK8aP4GeG1gl38/xIvutYHf4r/X3wCvDezyHwPxontt4Lf47/M3wGsDu1zx3sBrAe/Dvx3iRffawG/x3+evgdcBdoH3Br6LK74beB/+bRAvugcDT+e/118D3wN8Fc/pu4H34V8P8a/z08Bb8T/TdwPvw78O4l/nOHArcIz/mb4beB9edIh/vePAdwNvxf9M3w28Dy8axL/dg4EH8z/TXwO7/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BG4aZQfBTbqIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSquareFoot;
impl IconShape for MdSquareFoot {
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
                d: "M17.66,17.66l-1.06,1.06l-0.71-0.71l1.06-1.06l-1.94-1.94l-1.06,1.06l-0.71-0.71l1.06-1.06l-1.94-1.94l-1.06,1.06 l-0.71-0.71l1.06-1.06L9.7,9.7l-1.06,1.06l-0.71-0.71l1.06-1.06L7.05,7.05L5.99,8.11L5.28,7.4l1.06-1.06L4,4v14c0,1.1,0.9,2,2,2 h14L17.66,17.66z M7,17v-5.76L12.76,17H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAENUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/36/xQv3OvzPhfj3My+c+J8L8e9nXjjxPxfi38+8cOJ/LsS/n3nhxP9ciH8/88KJ/7kQ/37mhRP/cyH+/cwLJ/7nQvz7mRdO/M+F+PczL5z4nwvx72deOPE/F+K/x28Dr8UL9jrAb/OfD/Hf47eB1+IFex3gt/nPh/j3ey1euN/hef028Fq8YK8D/Db/+RD/fuaFE8/rt4HX4gV7HeC3+c+H+PczL5x4Xr8NvBYv2OsAv81/PsS/n3nhxPP6beC1eMFeB/ht/vMh/v3MCyee128Dr8UL9jrAb/OfD/HvZ1448bx+G3gtXrDXAX6b/3yIfz/zwon/uRD/fuaFE/9zIf79zAsnntdLA8d4wf4G2OU5HQdeihfsEvDX/Osg/v3MCyee128Dr8UL9jrAb/OcXhv4LV6w3wFem38dxL+feeHE8/pt4LV4wV4H+G2e02sDv8UL9jvAa/Ovg/j3My+ceF6/DbwWL9jrAL/Nc3pt4Ld4wX4HeG3+dRD/fuaFE8/rt4HX4gV7HeC3eU6vDfwWL9jvAK/Nvw7i38+8cOJ5/TbwWrxgrwP8Ns/ptYHf4gX7HeC1+ddB/PuZF048r98GXosX7HWA3+Y/H+Lfz7xw4nn9NvBavGCvA/w2//kQ/37mhRPP67eB1+IFex3gt3lOx4GX4gW7BPw1/zqIfz/zwonn9dvAa/GCvQ7w2zyn1wZ+ixfsd4DX5l8H8e9nXjjxvH4beC1esNcBfpvn9NrAb/GC/Q7w2vzrIP79zAsnntdvA6/FC/Y6wG/znF4b+C1esN8BXpt/HcS/n3nhxPP6beC1eMFeB/htntNrA7/FC/Y7wGvzr4P49zMvnHhevw28Fi/Y6wC/zXN6beC3eMF+B3ht/nUQ/37mhRPP67eB1+IFex3gt3lOrw38Fi/Y7wCvzb8O4t/PvHDief028Fq8YK8D/DbP6bWB3+IF+x3gtfnXQfz7mRdOPK/fBl6LF+x1gN/mOb028Fu8YL8DvDb/Ooh/P/PCief128Br8YK9DvDbPKfXBn6LF+x3gNfmXwfx72deOPG8fht4LV6w1wF+m+f02sBv8YL9DvDa/Osg/v1emxfut3leLw0c5wX7a2CX53QceGlesF3gr/nXQfz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BCTEg0F9UctdAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStackedBarChart;
impl IconShape for MdStackedBarChart {
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
                d: "M6 10h3v10H6zm0-5h3v4H6zm10 11h3v4h-3zm0-3h3v2h-3zm-5 0h3v7h-3zm0-4h3v3h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStream;
impl IconShape for MdStream {
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
            circle {
                cx: "20",
                cy: "12",
                r: "2",
            }
            circle {
                cx: "4",
                cy: "12",
                r: "2",
            }
            circle {
                cx: "12",
                cy: "20",
                r: "2",
            }
            path {
                d: "M10.05 8.59L6.03 4.55h-.01l-.31-.32-1.42 1.41 4.02 4.05.01-.01.31.32zm3.893.027l4.405-4.392L19.76 5.64l-4.405 4.393zM10.01 15.36l-1.42-1.41-4.03 4.01-.32.33 1.41 1.41 4.03-4.02zm9.75 2.94l-3.99-4.01-.36-.35L14 15.35l3.99 4.01.35.35z",
            }
            circle {
                cx: "12",
                cy: "4",
                r: "2",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP8dLA8d4TpeAv+ZF89LAMZ7TJeCv+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIgX3UsDX8WL5qWB4zynXeCvedG8NHCc57QL/DUvmo8B/pp/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50Lw18NS+alwaO8ZwuAX/Ni+algWM8p0vAX/Oi+Wjgr/mXIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiRffSwFfxonlp4DjPaRf4a140Lw0c5zntAn/Ni+ZjgL/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3UsDX82L5qWBYzynS8Bf86J5aeAYz+kS8Ne8aD4a+Gv+ZYj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x28Br8Zx+B3htXjS/DbwWz+l3gNfmPxbiP8dvA6/Fc/od4LV50fw28Fo8p98BXpv/WIj/HL8NvBbP6XeA1+ZF89vAa/Gcfgd4bf5jIf5z/DbwWjyn3wFemxfNbwOvxXP6HeC1+Y+F+M/x0sBxntMu8Ne8aF4aOM5z2gX+mv9YiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPVkWoQbTUmgcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTag;
impl IconShape for MdTag {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD1klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv89r8Zx+h/96iP8e7w18F8/pfYDv5r8W4r/HXwEvzXP6a+Bl+K+F+K/32sBv8fy9DvDb/NdB/Nf7buC9eP6+B3hv/usg/ms9GHg6L9xDgFv5r4H4r/XZwGfxbH/DFS/Fs30O8Nn810D817oIHOfZ3gc4DnwVz7YLnOC/BuK/znsD38VzOsEVF3lO7wN8N//5EP91/gp4aZ7tZ4C35oqfBt6KZ/tr4GX4z4f4r/HawG/xnN4G+GmueGvgp3hOrwP8Nv+5EP81fhp4K57tEnCc57QLHOPZvgd4b/5zIf7zPRh4Os/pa4CP5jl9NfBRPKeHALfynwfxn++rgY/iOb0M8Nc8p5cG/orn9DXAR/OfB/Gf6zjwdOA4z/Y3wEvz/P018FI82y7wEGCX/xyI/1zvDXwXz+ljgK/m+fto4Kt4Tu8DfDf/ORD/uZ4OPJjndALY5fk7DlzkOd0KPIT/HIj/PK8N/BbP6WeAt+aF+2ngrXhOrwP8Nv/xEP95fhp4K57T2wA/zQv31sBP8Zx+Bnhr/uMh/nM8GHg6z+u3edG8Ns/rIcCt/MdC/Of4auCj+I/1NcBH8x8L8R/vOPB04Dj/sXaBhwC7/MdB/Mf7aOCr+M/xPsB38x8H8R/v6cCD+c9xK/AQ/uMg/mO9NfBTPK/3AW7lX+fBwHfxvN4G+Gn+YyD+Y/0W8No8p98BXpt/m98GXovn9NvA6/AfA/Ef58HA03le7wN8N/827w18F8/rIcCt/Psh/uN8N/BePKdLwHH+fXaBYzyn7wHem38/xH+M48DTgeM8p68BPpp/n68GPorntAs8BNjl3wfxH+Ojga/ieT0EuJV/nwcDT+d5fQzw1fz7IP5/Q/z/hvj/DfH/G+L/N8S/zVcDL8X/LH8DfDT/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lq4KX5n+WvgY/mXwfx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8y4G5Br34M7QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextFormat;
impl IconShape for MdTextFormat {
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
                d: "M5 17v2h14v-2H5zm4.5-4.2h5l.9 2.2h2.1L12.75 4h-1.5L6.5 15h2.1l.9-2.2zM12 5.98L13.87 11h-3.74L12 5.98z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t3kw8FXAW/M/w08DHwPcyr8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrHAd+C3hprrgE/DX/vV6LZ7sVeBlglxcN4l/nt4DX5opLwGsDf81/r/cGvotn+2vgZXjRIF503wW8N8/2PsB38z/DewPfxbN9N/A+/MsQL5r3Br6LZ9sF/pr/WV6b5/QxwFfzwiH+ZQ8Gns7/Tg8BbuUFQ/zLvht4L/53+h7gvXnBEP+ypwMP5n+nvwZehhcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/95/oYrXor/POIFQ/zLzH+OvwFemyt+G3gp/nOIFwzxLzP/8f4GeG1glyuOA78NvBT/8cQLhviXmf9YfwO8NrDLczoO/DbwUvzHEi8Y4l9m/uP8DfDawC7P33Hgt4GX4j+OeMEQ/zLzH+NvgNcGdnnhjgO/DbwU/zHEC4b4l5l/v78BXhvY5UVzHPht4KX49xMvGOJfZv59/gZ4bWCXf53jwG8DL8W/j3jBEP8y82/3N8BrA7v82xwHfht4Kf7txAuG+JeZf5u/AV4b2OXf5zjw28BL8W8jXjDEv8z86/0N8NrALv8xjgO/DbwU/3riBUP8y8y/zt8Arw3s8h/rOPDbwEvxryNeMMS/zLzo/gZ4bWCX/xzHgd8GXooXnXjBEP8y86L5G+C1gV3+cx0Hfht4KV404gVD/MvMv+xvgNcGdvmvcRz4beCl+JeJFwzxLzP/MX4HeG1eNL8NvBb/McQLhviXmf8YvwO8Ni+a3wZei/8Y4gVD/MvMf4zfAV6bF81vA6/FfwzxgiH+ZeY/xu8Ar82L5reB1+I/hnjBEP8y8x/jd4DX5kXz28Br8R9DvGCIf5n5j7EL/DUvmpcGjvMfQ7xgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/2S5wjP+dLgHHecEQ/7LvBt6L/52+B3hvXjDEv+ylgb/if6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IORqCQT+gqWQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUnarchive;
impl IconShape for MdUnarchive {
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
                d: "M20.55,5.22l-1.39-1.68C18.88,3.21,18.47,3,18,3H6C5.53,3,5.12,3.21,4.85,3.55L3.46,5.22C3.17,5.57,3,6.01,3,6.5V19 c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6.5C21,6.01,20.83,5.57,20.55,5.22z M12,9.5l5.5,5.5H14v2h-4v-2H6.5L12,9.5z M5.12,5 l0.82-1h12l0.93,1H5.12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4vk7Duzyfx/i+ftp4LOBv+b/NsTz99vASwGvA/w1/3chnr/fBl4L2AVeB/hr/uO8FvBg4MFc8WDgOPDXPNtvA88AbuU/F+L5+23gtbhiF3gd4K/5t3lp4L2A1wZemn+dW4HfBv4a+BngVv5jIZ6/3wZei2fbBV4H+GteNMeB9wI+Gngw/3G+G/ge4Lf5j4F4/n4beC2e0y7wOsBf84IdBz4K+GjgOP95bgU+Bvhp/n0Qz99vA6/F89oFXgf4a57XWwPfBRznv85vA+8D3Mq/DeL5+23gtXj+doHXAf6aK44D3wW8Nf89doGvBj6Hfz3E8/fbwGvxgu0Cr8MVvwUc57/fTwPvA+zyokM8f78NvBYv3C5wnH+dZwC/Dfw2cCtX/DbP9tpc8dLAawOvDRzjRffXwOsAu7xoEM/fbwOvxX+MS8BXAz8N/DX/eq8NfDTwVrxobgXeBvhr/mWI5++3gdfi3+cS8NXAVwO7/Ps9GPhs4L34l/018DrALi8c4vn7beC1+Lf7HeCtgV3+47028N3Ag3jh/hp4GV44xPP328Br8a93Cfhs4Kv5z/fdwHvxwn038D68YIjn77eB1+Jfbxd4HeCv+a/x0cBX8cK9DfDTPH+I5++3gdfi32YXeB3gr/mv8d7Ad/GC7QIPAXZ5Xojn77eB1+Lfbhd4HeCv+a/x1cBH8YJ9DfDRPC/E8/fbwGvx77MLvA7w1/zX+G7gvXjBHgLcynNCPH+/DbwW/367wOsAf81/vuPArcAxnr/vAd6b54R4/n4beC3+Y+wCrwP8Nf/53hv4Ll6wE8Auz4Z4/n4beC3+4+wCrwP8Nf/5fht4LZ6/jwG+mmdDPH+/DbwW/7F2gdcB/pr/XG8N/BTP39cAH82zIZ6/3wZei/94u8DrAH/Nf65bgQdxxc8APw38NLDLc0I8f18NvDT/OXaB9wZ2+c/z3lzx08AuLxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAUSyh0HWxcOrAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUndo;
impl IconShape for MdUndo {
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
                d: "M12.5 8c-2.65 0-5.05.99-6.9 2.6L2 7v9h9l-3.62-3.62c1.39-1.16 3.16-1.88 5.12-1.88 3.54 0 6.55 2.31 7.6 5.5l2.37-.78C21.08 11.03 17.15 8 12.5 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWaves;
impl IconShape for MdWaves {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEIElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zbHgfcCXhp4MP+9bgX+GvgeYJd/HcS/3nsDXwUc53+WXeBjgO/mRYf413lt4Lf4n+11gN/mRYP413k68GD+Z7sVeAgvGsSL7qWBv+J/h5cB/pp/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdg4HvBl6L/9n+Bnhr4Fb+ZYh/vbcGvhs4xv8sl4DPBr6aFx3iBXsw8CCueAZwK8/20sBvA8f4n+ES8NrAX/NsDwYexBXPAG7leSGe13Hgu4C35jn9NPA+wC5XfDTwVfzP8DHAV3PFceC7gLfmOf008D7ALs+GeE7HgacDx3n+doGHALtc8dfAS/Hf62+Al+aK48DTgeM8f7vAQ4BdrkA8p58C3poX7qeBt+GK9wa+i/9e7wN8N1f8FPDWvHA/DbwNVyCe7cHA03nRPAS4FXgw8HT+ez0EuBV4MPB0XjQPAW4FEM/22sBv8aJ5HeC3ucL89xJXvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpsrzH8vccVrA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3ucL89xJXvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpsrzH8vccVrA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3ucL89xJXvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpsrzH8vccVrA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3ueK1+e/121zx2sBv8aJ5HeC3AcSzvTbwW7xoXgf4bf5neW3gt3jRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbSwN/xf8PLwP8NYB4TrcCD+L/tmcAD+YKxHN6beC3+L/tdYDf5grE83pv4KuBY/zfcgl4b+CneTbE83cceG/gpYEH87/brcBfA98N7PKcEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CNcyhkFYBM9TAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWeekend;
impl IconShape for MdWeekend {
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
                d: "M21,10c-1.1,0-2,0.9-2,2v3H5v-3c0-1.1-0.89-2-2-2s-2,0.9-2,2v5c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2v-5 C23,10.9,22.1,10,21,10z M18,5H6C4.9,5,4,5.9,4,7v2.15c1.16,0.41,2,1.52,2,2.81V14h12v-2.03c0-1.3,0.84-2.4,2-2.81V7 C20,5.9,19.1,5,18,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Nf67fBt4G2OXfD/Ef57eA1+a/xm8Dr8O/H+I/xncB781/re8G3od/H8S/33sD38V/j/cBvpt/O8S/z3Hg6cBx/nvsAg8Bdvm3Qfz7fDbwWfz3+hzgs/m3QfzbPRj4K+A4/7kuAW8NfDXwUjyvXeBlgFv510P823008FX857oEvDbw18Bx4LeBl+J5fQzw1fzrIf7tfht4Lf7zXAJeG/hrnu048NvAS/GcfgZ4a/71EP82x4GL/Oe5BLw28Nc8r+PAXwMP4jmdAHb510H827w28Fv857gEvDbw1zx/7w18F8/rdYDf5l8H8W/z2cBn8R/vEvDawF/z/L038F08fx8DfDX/Ooh/m88GPov/WJeA1wb+mufvvYHv4gX7HOCz+ddB/Nt8N/Be/Me5BLw28Nc8f+8NfBcv3NcAH82/DuLf5ruB9+IFuwR8NfBZ/MsuAa8N/DXP33sD38W/7HuA9+ZfB/Fv89nAZ/H8XQJeG/hr4L2B7+IFuwS8NvDXPH/vDXwXL5rPAT6bfx3Ev81nA5/F87oEvDbw1zzbewPfxfO6BLw28Nc8f+8NfBcvus8BPpt/HcS/zWsDv8VzugS8NvDXPK/3Br6LZ7sEvDbw1zx/7w18F/86rwP8Nv86iH+b48BFntMu8DrAX/P8vTfwXcAl4LWBv+b5e2/gu/jXOwHs8q+D+Le7FXgQz2kXeB3gr3n+3hv4a+Cvef7eG/gu/vWeATyYfz3Ev91XAx/F89oFXgf4a/513hv4Lv5tvgb4aP71EP92Lw38Fc/fLvA6wF/zonlv4Lv4t3sZ4K/510P8+9wKPIjnbxd4HeCveeHeG/gu/u2eATyYfxvEv89nA5/FC7YLvA7w1zx/7w18F/8+nwN8Nv82iH+fBwNP54XbBV4H+Gue03sD38W/30OAW/m3Qfz7fTfwXrxwu8DrAH/NFe8NfBf/ft8DvDf/doh/v5cG/op/2S7wOsBLA9/Ff4yHALfyb4f4j/HbwGvxL9sFjvMf42eAt+bfB/Ef47WB3+K/1ssAf82/D+I/zncD78V/je8B3pt/P8R/nAcDT+e/xkOAW/n3Q/zH+mzgs/jP9TnAZ/MfA/Ef6zjw18CD+M/xDOClgV3+YyD+47018FP853gb4Kf5j4P4z/HTwFvxH+tngLfmPxbiP8eDgb8GjvEf4xLwYGCX/1iI/zwfDXwV/zE+Bvhq/uMh/nP9NPBW/Pv8DPDW/OdA/Oc6DtwKHOPf5hLwYGCX/xyI/3xvDfwU/zZvA/w0/3kQ/zW+Gvgo/nW+Bvho/nMh/uv8NfBSvGj+Bnhp/vMh/us8GPhr4Bgv3CXgpYFb+c+H+K/11sBP8cK9DfDT/NdA/Nf7bOCzeP4+B/hs/usg/nv8NPBWPKfvAd6b/1qI/x7Hgd8GXoor/gZ4bWCX/1qI/z7HgVu54sHALv/1EP+9Xpor/pr/Hoj/3xD/vyH+f0P8/4b4/41/BPAtyEGW3teyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWhereToVote;
impl IconShape for MdWhereToVote {
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
                d: "M12 2c3.86 0 7 3.14 7 7 0 5.25-7 13-7 13S5 14.25 5 9c0-3.86 3.14-7 7-7zm-1.53 12L17 7.41 15.6 6l-5.13 5.18L8.4 9.09 7 10.5l3.47 3.5z",
            }
        }
    }
}
