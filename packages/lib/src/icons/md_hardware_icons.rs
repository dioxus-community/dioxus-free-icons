use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xwr008FPA2wB/zf89iBfspYHfAo4Du8DrAH/N8/dg4EH8z/A7vOgQz99LA78FHOfZdoHXAf6a53Uc+G3gpfjvtwt8NfA5/MsQz99XAx/F89oFXgf4a57XceC3gZfif4bvBt6HFw7xgn038F48r13gdYC/5nkdB34beCn+Z3gf4Lt5wRAv3HcD78Xz2gVeB/hrntdx4LeBl+K/3+8Ar80LhviXmedvF3gd4K95XseB3wZeiv9+4gVD/MvMC7YLvA7w1/z3e23gt3he4gVD/MvMC7cLvA7w1/z3em3gt3he4gVD/MvMv2wXeB3gr/nv89rAb/G8xAuG+JeZF80u8DrAX/Pf47WB3+J5iRcM8S8zz+tvgJfiee0CrwP8Nf/1Xhv4LZ6XeMEQ/zLzvF4HeG/gvXheu8DrAH/Nf63XBn6L5yVeMMS/zDyv1wF+G/hu4L14XrvA6wB/zX+d1wZ+i+clXjDEv8w8r9cBfpsrvht4L57XLvA6wF/zX+O1gd/ieYkXDPEvM8/rdYDf5tm+G3gvntcu8DrAX/Of77WB3+J5iRcM8S8zz+t1gN/mOX038F48r13gdYC/5j/XawO/xfMSLxjiX2ae1+sAv83z+m7gvXheu8DrAH/Nf57XBn6L5yVeMMS/zDyv1wF+m+fvu4H34nntAq8D/DX/OV4b+C2el3jBEP8y87xeB/htXrDvBt6L57ULvA7w1/zHe23gt3he4gVD/MvM83od4Ld54b4beC+e1y7wOsBf8x/rtYHf4nmJFwzxLzPP63WA3+Zf9t3Ae/G8doHXAf6a/zivDfwWz0u8YIh/mXlerwP8Ni+a7wbei+e1C7wO8Nf8x3ht4Ld4XuIFQ/zLzPN6HeC3edF9N/BePK9d4HWAv+bf77WB3+J5iRcM8S8zz+t1gN/mX+e7gffiee0CrwP8Nf8+rw38Fs9LvGCIf5l5Xq8D/Db/et8NvBfPaxd4HeCv+bd7beC3eF7iBUP8y8zzeh3gt/m3+W7gvXheu8DrAH/Nv81rA7/F8xIvGOJfZp7X6wC/zb/ddwPvxfPaBV4H+Gv+9V4b+C2el3jBEP8y87xeB/ht/n2+G3gvntcu8DrAX/Ov89rAb/G8xAuG+JeZ5/U6wG/z7/fdwHvxvHaB1wH+mhfdawO/xfMSLxjiX2ae1+sAv81/jO8G3ovntQu8DvDXvGheG/gtnpd4wRD/MvO8Xgf4bf7jfDfwXjyvXeB1gL/mX/bawG/xvMQLhviXmef1OsBv8x/ru4H34nntAq8D/DUv3GsDv8XzEi8Y4l9mntfrAL/Nf7zvBt6L57ULvA7w17xgrw38Fs9LvGCIf5l5Xq8D/Db/Ob4beC+e1y7wOsBf8/y9NvBbPC/xgiH+ZeZ5vQ7w2/zn+W7gvXheu8DrAH/N8zoO/DbwUjwn8YIh/mXmeb0O8Nv85/pu4L14XrvA6wB/zfM6Dvw28FI8m3jBEP8y87xeB/ht/vN9N/BePK9d4HWAv+Z5HQd+G3gprhAvGOJfZp7X6wC/zX+N7wbei+e1C7wO8Nc8r+PAbwMvBYgXDPEvM/9z7QKvA/w1z+s48NvAS/OCIf5lfw28FP9z7QKvA/w1/3qIf9l7A9/F/2y7wOsAf82/DuJF893Ae/E/2y7wOsBf86JDvOg+G/ho4Bj/c+0CrwP8NS8axL/ea/M/w3sD78Xz2gVeB/hr/mWI/92+G3gvntcu8DrAX/PCIf73+27gvXhezwAezAuH+L/hu4H34tkuAa8N/DUvHOL/ju8G3gu4BLw28Nf8yxD/t3w18N3AX/OiQfz/hvj/DfH/G+L/N8T/b/wjZ/D1QTyQhP4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrowserNotSupported;
impl IconShape for MdBrowserNotSupported {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,6v10.5l1.95,1.95C20.98,18.3,21,18.15,21,18V6c0-1.1-0.9-2-2-2H6.5l2,2H19z",
            }
            path {
                d: "M3.22,3.32L1.95,4.59L3,5.64L3,18c0,1.1,0.9,2,2,2h12.36l2.06,2.06l1.27-1.27L3.22,3.32z M15,18H5V7.64L15.36,18H15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCast;
impl IconShape for MdCast {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCastConnected;
impl IconShape for MdCastConnected {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCastForEducation;
impl IconShape for MdCastForEducation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvo4Gv4v+WjwG+mueEeP6+G3gv/m/5HuC9eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntNfAx/N/w5fDbw0z+lngLfmOSGev88GPov/Wz4H+GyeE+L5e2vgp/i/5W2An+Y5IV6wvwZeiv8bfgd4bZ4X4gV7MPDTwEvxv9vfAK8N7PK8EC/cceCjgfcGHsT/Ls8Avhv4amCX5w8B5v8vBJj/vxBg/v9CgPn/CwHm/y8EvDb/fyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwI0yYx8CixV0AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdComputer;
impl IconShape for MdComputer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdConnectedTv;
impl IconShape for MdConnectedTv {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12zM4 14v2h2c0-1.11-.89-2-2-2zm0-3v1.43c1.97 0 3.57 1.6 3.57 3.57H9c0-2.76-2.24-5-5-5zm0-3v1.45c3.61 0 6.55 2.93 6.55 6.55H12c0-4.42-3.59-8-8-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP33HgvYCXBh7M/263An8NfA+wy3NCPK/3Br4KOM7/LbvAxwDfzbMhntNrA7/F/22vA/w2VyCe09OBB/N/263AQ7gC8WwvDfwV/z+8DPDXAOLZXhv4LZ7X7/C/22vxvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8r9/mf7fX5nm9DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4TrvAMf5vuwQc5wrEc/ps4LP4v+1zgM/mCsTz+m7gvfi/6XuA9+bZEM/fewMfDbwU/zf8DvDdwHfznBD/dX4KeGteND8NvA3/+RD/Nd4a+Cn+dd4G+Gn+cyH+8x0Hng4c519nF3gIsMt/HsR/vp8C3pp/m58G3ob/PIj/XG8N/BT/Pm8D/DT/ORD/eY4DTweO8++zCzwE2OU/HuI/z08Bb81/jJ8G3ob/eIj/HG8N/BT/sd4G+Gn+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dXAy/Nf6y/Bj6a/1iI/98Q/78h/n9D/P+G+P+NfwRzQ45Bihbt8gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDesktopMac;
impl IconShape for MdDesktopMac {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7l-2 3v1h8v-1l-2-3h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 12H3V4h18v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP33HgvYCXBh7M/263An8NfA+wy3NCPK/3Br4KOM7/LbvAxwDfzbMhntNrA7/F/22vA/w2VyCe09OBB/N/263AQ7gC8WwvDfwV/z+8DPDXAOLZXhv4LZ7X7/C/22vxvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF6vzf9uv83zeh3gtwHEs7028Fv8//A6wG8DiOe0Cxzj/7ZLwHGuQDynzwY+i//bPgf4bK5APK/vBt6L/5u+B3hvng3x/L038NHAS/F/w+8A3w18N88J8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8V/ntYHf4kXzOsBv858P8Z/jq4GX4jkdB16aF81fA7s8p78BPpr/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/xxfDbw0/7H+Gvho/mMh/n9D/P+G+P8N8f8b4v83/hEj6pNBLgSR1wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDesktopWindows;
impl IconShape for MdDesktopWindows {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7v2H8v2h8v-2h-2v-2h7c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H3V4h18v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeveloperBoard;
impl IconShape for MdDeveloperBoard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/dg4KOA1wZemiv+Gvht4GuAW/mvg/iv9VXAR/PCfTXwMfzXQPzX+SvgpXnR/DXwMvznQ/zX+Grgo/jX+Rrgo/nPhfjP92Dg6fzbPAS4lf88iP98Xw18FP82XwN8NP95EP/5/gp4af5t/hp4Gf7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3x/DbwU/zZ/A7w0/3kQ//m+G3gv/m2+Bvho/vMg/vM9GHg6/zYPAW7lPw/iv8ZXAx/Fv87XAB/Nfy7Ef52fBt6KF83PAG/Nfz7Ef62vBj6KF+5rgI/mvwbiv96Dgd8GHsRzegbw2sCt/NdB/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvzXe2ngt4DjPKdd4HWAv+a/DuK/1ksDvwUc5/nbBV4H+Gv+ayD+67w08FvAcV64XeB1gL/mPx/iv8ZLA78FHOdFswu8DvDX/OdC/Od7aeC3gOM8r0tccYzntQu8DvDX/OdB/Od6aeC3gOM8r0vAa3PFbwPHeF67wOsAf81/DsR/npcGfgs4zvP3OsBvc8VrA7/F87cLvA7w1/zHQ/zneGngt4DjPH/vA3w3z+m9ge/i+dsFXgf4a/5jIf7jvTTwW8Bxnr/3Ab6b5++9ge/i+dsFXgf4a/7jIP5jvTTwW8Bxnr/3Ab6bF+69ge/i+dsFXgf4a/5jIP5lr8WL5jjw3cBxnr/3Ab6bF817A9/F87cLvDewy4vmd3jBEP8y8+/3McBX86/z0cBX8e8nXjDEv8z8+3wP8N7823w38F78+4gXDPEvM/923wO8N/8+3w28F/924gVD/MvMv833AO/Nf4zvBt6LfxvxgiH+ZeZf73uA9+Y/1ncD78W/nnjBEP8y86/3OsBv8x/rtYHf4l9PvGCIf5n513sd4Lf5j/XawG/xrydeMMS/zPzrvQ7w2/zHem3gt/jXEy8Y4l9m/vVeB/ht/mO9NvBb/OuJFwzxLzP/eq8D/Db/sV4b+C3+9cQLhviXmX+91wF+m/9Yrw38Fv964gVD/MvMv97rAL/Nf6zXBn6Lfz3xgiH+ZeZf73WA3+Y/1msDv8W/nnjBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHwcaHQUe7y9EAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeviceHub;
impl IconShape for MdDeviceHub {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M17 16l-4-4V8.82C14.16 8.4 15 7.3 15 6c0-1.66-1.34-3-3-3S9 4.34 9 6c0 1.3.84 2.4 2 2.82V12l-4 4H3v5h5v-3.05l4-4.2 4 4.2V21h5v-5h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeviceUnknown;
impl IconShape for MdDeviceUnknown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14zM12 6.72c-1.96 0-3.5 1.52-3.5 3.47h1.75c0-.93.82-1.75 1.75-1.75s1.75.82 1.75 1.75c0 1.75-2.63 1.57-2.63 4.45h1.76c0-1.96 2.62-2.19 2.62-4.45 0-1.96-1.54-3.47-3.5-3.47zm-.88 8.8h1.76v1.76h-1.76z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDevicesOther;
impl IconShape for MdDevicesOther {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 6h18V4H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h4v-2H3V6zm10 6H9v1.78c-.61.55-1 1.33-1 2.22s.39 1.67 1 2.22V20h4v-1.78c.61-.55 1-1.34 1-2.22s-.39-1.67-1-2.22V12zm-2 5.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM22 8h-6c-.5 0-1 .5-1 1v10c0 .5.5 1 1 1h6c.5 0 1-.5 1-1V9c0-.5-.5-1-1-1zm-1 10h-4v-8h4v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEK0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/3mvxn+t3+I+D+I/zWcBHA8f5z7ULfDXwOfz7If5jfBfw3vzX+m7gffj3Qfz7vTfwXfz3eB/gu/m3Q/z7/RXw0vz3+B3gtfm3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv595Xh8D/DX/sV4a+Cqel/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mef1OsBv8x/rtYHf4nmJfzvEv595Xq8D/Db/sV4b+C2el/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv9+twIP47/EM4MH82yH+/T4b+Cz+e3wO8Nn82yH+/Y4Dvw28FP+1/gZ4bWCXfzvEf4wHAz8NvBT/Nf4GeGvgVv59EP+x3hp4aeC1+c/x28BfAz/NfwzE/2+I/98Q/78h/n9D/P+G+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cvw28Fv+xfgd4bf5jIf5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x0sDx/mPtQv8Nf+xEP+/If5/4x8B94CAQQnIktIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDock;
impl IconShape for MdDock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 23h8v-2H8v2zm8-21.99L8 1c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM16 15H8V5h8v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+X6Lf5/X4T8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP987w18F/823wO8N/95EP813hv4Lv51vgd4b/5zIf7rvDfwXbxovgd4b/7zIf5rvTfwXbxw3wO8N/81EM/fbwOvxb/NdwPvwwv23sB38fx9D/DevGDvDXwX/za/A7w2zwnx/P028Fr823038D68YO8NfBfP6XuA9+YFe2/gu/i3+x3gtXlOiOfvt4HX4t/nu4H34QV7b+C7uOJ7gPfmBXtv4Lv49/kd4LV5Tojn77eB1+Lf77uB9+EFe2/gtYH35gV7b+C7+Pf7HeC1eU6I5++3gdfiP8Z3A+/Dv817A9/Ff4zfAV6b54R4/n4beC3+43w38D7867w38F38x/kd4LV5Tojn77eB1+I/1ncD78OL5r2B7+I/1u8Ar81zQjx/vw28Fv/xvht4H1649wa+i/94vwO8Ns8J8fz9NvBa/Mf7HuC9eeHeG/gu/uP9DvDaPCfE8/fbwGvxH+t7gPfmRfPewHfxH+t3gNfmOSGev98GXov/ON8DvDf/Ou8NfBf/cX4HeG2eE+L5+23gtfiP8T3Ae/Nv897Ad/Ef43eA1+Y5IZ6/3wZei3+/7wHemxfsvYHXAt6HF+y9ge/i3+93gNfmOSGev98GXot/n+8B3psX7L2B7+KK7wbehxfsvYHv4t/nd4DX5jkhnr/fBl6Lf7vvAd6bF+y9ge/iOX038D68YO8NfBf/dr8DvDbPCfH8/TbwWvzbfA/w3rxg7w18F8/fdwPvwwv23sB38W/zO8Br85wQ/7XeG/guXrjvBt6H/xqI/zrvDXwXL5rvBt6H/3yI/xrvDXwX/zrfDbwP/7kQ//neG/gu/m2+G3gf/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M/32/z7vDb/eRD/vyH+f0P8/4b4/w3x/xv/CM5mfEFlx04RAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGamepad;
impl IconShape for MdGamepad {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 7.5V2H9v5.5l3 3 3-3zM7.5 9H2v6h5.5l3-3-3-3zM9 16.5V22h6v-5.5l-3-3-3 3zM16.5 9l-3 3 3 3H22V9h-5.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x2vx/P0O/7UQ//keDLwV8NrAawPHeeF2gd8Gfhv4GeBW/vMg/vO8F/DRwEvz7/PbwHcD38N/PMR/vPcCPht4MP+xbgU+G/ge/uMg/uO8NPBVwGvzn+u3gY8B/pp/P8R/jI8Cvpr/Wh8NfA3/Poh/v+8C3pt/2SXgp4GfBnaBXeCvueKlgePAceCtgbcGjvEv+27gffi3Q/zbHQe+C3hrXrjvAb4a+Gv+dV4a+GjgvXjhfhp4H2CXfz3Ev913Ae/NC/Y9wGcDt/Lv82Dgq4G34gX7buB9+NdD/Nt8F/DePH+XgLcGfpv/WK8N/DRwjOfva4CP5l8H8a/33sB38fz9DfDWwK3853gw8NPAS/H8vQ3w07zoEP86Dwb+CjjO8/ob4LWBXf5zHQd+G3gpntcu8DLArbxoEP86PwW8Nc/rGcBLA7v81zgO/DXwIJ7XTwNvw4sG8aJ7beC3eF6XgNcG/pr/Wi8N/DZwjOf1OsBv8y9DvOh+C3htntf7AN/Nf4/3Br6L5/XbwOvwL0O8aF4a+Cue198AL81/r1uBB/G8Xgb4a144xIvmq4GP4nm9DvDb/Pd6beC3eF5fA3w0LxziRfN04ME8p78BXpr/Gf4aeCme063AQ3jhEP+ylwb+iuf1McBX8z/DRwNfxfN6GeCvecEQ/7KPBr6K5/UQ4Fb+Z3gw8HSe18cAX80LhviXfTfwXjynvwFemv9ZbgUexHP6HuC9ecEQ/7LfBl6L5/QzwFvzP8tPA2/Fc/od4LV5wRD/sovAcZ7T5wCfzf8snw18Fs9pFzjBC4b4l5nn9TnAZ/M/y2cDn8XzEi8Y4l9mntfbAD/N/yxvDfwUz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5ltwIP4t9O/PuYf7tnAA/mBUP8y34aeCv+7cS/j/m3+x7gvXnBEP+y9wa+i3878e9j/u3eB/huXjDEi+avgZfi30b8+5h/m98BXpsXDvGieWngt4Fj/OuJfx/zr3cJeGngVl44xIvupYGfBh7Ev4749zH/Os8A3hr4a/5liH+d48BHA+8NPIgXjfj3MS+aZwDfDXw1sMuLBvFv99LAcf5lv82/z2vzL9sF/pp/PcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4Rxnfr0HiGO5WAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHeadset;
impl IconShape for MdHeadset {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 1c-4.97 0-9 4.03-9 9v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-3.87 3.13-7 7-7s7 3.13 7 7v2h-4v8h3c1.66 0 3-1.34 3-3v-7c0-4.97-4.03-9-9-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x2vx/P0O/7UQ//keDLwV8NrAawPHeeF2gd8Gfhv4GeBW/vMg/vO8F/DRwEvz7/PbwHcD38N/PMR/vPcCPht4MP+xbgU+G/ge/uMg/uO8NPBVwGvzn+u3gY8B/pp/P8R/jI8Cvpr/Wh8NfA3/Poh/v+8C3pt/2SXgp4GfBnaBXeCvueKlgePAceCtgbcGjvEv+27gffi3Q/zbHQe+C3hrXrjvAb4a+Gv+dV4a+GjgvXjhfhp4H2CXfz3Ev913Ae/NC/Y9wGcDt/Lv82Dgq4G34gX7buB9+NdD/Nt8F/DePH+XgLcGfpv/WK8N/DRwjOfva4CP5l8H8a/33sB38fz9DfDWwK3853gw8NPAS/H8vQ3w07zoEP86Dwb+CjjO8/ob4LWBXf5zHQd+G3gpntcu8DLArbxoEP86PwW8Nc/rGcBLA7v81zgO/DXwIJ7XTwNvw4sG8aJ7beC3eF6XgNcG/pr/Wi8N/DZwjOf1OsBv8y9DvOh+C3htntf7AN/Nf4/3Br6L5/XbwOvwL0O8aF4a+Cue198AL81/r1uBB/G8Xgb4a144xIvmq4GP4nm9DvDb/Pd6beC3eF5fA3w0LxziRfN04ME8p78BXpr/Gf4aeCme063AQ3jhEP+ylwb+iuf1McBX8z/DRwNfxfN6GeCvecEQ/7KPBr6K5/UQ4Fb+Z3gw8HSe18cAX80LhviXfTfwXjynvwFemv9ZbgUexHP6HuC9ecEQ/7LfBl6L5/QzwFvzP8tPA2/Fc/od4LV5wRD/sovAcZ7T5wCfzf8snw18Fs9pFzjBC4b4l5nn9TnAZ/M/y2cDn8XzEi8Y4l9mntfbAD/N/yxvDfwUz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5l5t9H/PuYfx/xgiH+ZebfR/z7mH8f8YIh/mXm30f8+5h/H/GCIf5ltwIP4t9O/PuYfx/xgiH+ZT8NvBX/duLfx/z7iBcM8S97b+C7+LcT/z7m30e8YIgXzV8DL8W/jfj3Mf8+4gVDvGheGvht4Bj/euLfx/z7iBcM8aJ7aeCngQfxryP+fcy/j3jBEP86x4GPBt4beBAvGvHvY/59xAuG+Ld7aeA4/7Lf5t/ntXnRvDTwVTwv8YIh/u94beC3eE7PAB7MC4b4v+O1gd/iOX0P8N68YIj/fuY/z/sA380LhvjvZ/5z/A7w2rxwiP9+5j/eJeClgVt54RD//cx/rGcAbw38Nf8yxH8/8x/jGcB3A18N7PKiQfz3e23+/XaBv+ZfD/H/G+L/N/4R+pW3QeiZwjMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHeadsetMic;
impl IconShape for MdHeadsetMic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 1c-4.97 0-9 4.03-9 9v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-3.87 3.13-7 7-7s7 3.13 7 7v2h-4v8h4v1h-7v2h6c1.66 0 3-1.34 3-3V10c0-4.97-4.03-9-9-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHeadsetOff;
impl IconShape for MdHeadsetOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 4c3.87 0 7 3.13 7 7v2h-2.92L21 17.92V11c0-4.97-4.03-9-9-9-1.95 0-3.76.62-5.23 1.68l1.44 1.44C9.3 4.41 10.6 4 12 4zM2.27 1.72L1 3l3.33 3.32C3.49 7.68 3 9.29 3 11v7c0 1.66 1.34 3 3 3h3v-8H5v-2c0-1.17.29-2.26.79-3.22L15 17v4h3c.3 0 .59-.06.86-.14L21 23l1.27-1.27-20-20.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboard;
impl IconShape for MdKeyboard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/zHOQ78FvA1wHfzn+O9gY8CXgfY5d8P8R/jOPBbwEtzxfsA381/rPcGvosr/hp4HWCXfx/Ev99x4LeAl+Y5vQ/w3fzHeG/gu3hOfw28DrDLvx3i3++3gdfi+Xsf4Lv593lv4Lt4/n4HeG3+7RD/fi8N/DZwjOfvfYDv5t/mvYHv4vm7BLw28Nf82yH+Y7w08NvAMZ6/9wG+m3+d9wa+i+fvEvDawF/z74P4j/PSwG8Dx3j+3gf4bl407w18F8/fJeC1gb/m3w/xH+ulgd8GjvH8vQ/w3bxw7w18F8/fJeC1gb/mPwbiP95LA78NHOP5ex/gu3n+3hv4Lp6/S8BrA3/NfxzEf46XBn4bOMbz9z7Ad/Oc3hv4Lp6/S8BrA3/NfyzEf56XBn4bOMbz9z7Ad3PFewPfxfN3CXht4K/5j4f4z/XSwG8Dx3j+3ocrvovn7xLw2sBf858D8Z/vpYHfBo7xr3MJeG3gr/nPg/iv8dLAbwPHeNFcAl4b+Gv+cyH+67w08NvAMV64S8BrA3/Nfz7Ef62XBn4bOMbzdwl4beCv+a+B+K/30sBvA8d4TpeA1wb+mv86iP8eLw38NnCMKy4Brw38Nf+1EP99Xhr4ba54beCv+a+H+O/10lzx1/z3QPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNwMk5BgXw0hgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardArrowDown;
impl IconShape for MdKeyboardArrowDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/npYGvAt4G2OU/F+J/lpcGfgs4Dvw18DrALv95EP9zvDTwW8Bxnu2vgdcBdvnPgfif4aWB3wKO87z+GngZ/nMg/vu9NPBbwHGev/cBvpv/HIj/Xi8N/BZwnOfvfYDv5j8P4r/PSwO/BRzn+Xsf4Lv5z4X47/HSwG8Bx3n+3gf4bv7zIf7rvTTwW8Bxnr/3Ab6b/xqI/1ovDfwWcJzn732A7+a/DuK/zksDvwUc5/l7H+C7+a+F+K/x0sBvAcd5/t4H+G7+6yH+87008FvAcZ6/9wG+m/8eiP9cLw38FnCc5+99gO/mvw/iP89LA78FHOf5ex/gu/nvhfjP8dLAbwHHef7eB/hu/vsh/nO8NPDbwDGev/cBvpv/foj/PC8N/DZwjOfvfYDv5r8X4j/XSwO/DRzj+Xsf4Lv574P4z/fSwG8Dx3j+3gf4bv57IP5rvDTw28Axnr/3Ab6b/3qI/zovDfw2cIzn732A7+a/FuK/1ksDvw0c4/l7H+C7+a+D+K/30sBvA8d4/t4H+G7+ayD+e7w08NvAMZ6/9wG+m/98iP8+Lw38NnCM5+99gO/mPxfiv9dLA78NHOP5ex/gu/nPg/jv99LAbwPHeP7eB/hu/nMg/md4aeC3gWM8r78BXpr/HIj/OV4a+G3gGM/2N8BrA7v850D8z/LSwG8Dx4C/AV4b2OU/D+J/npcGvhp4a2CX/1yI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjL21EQf4E52gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardArrowLeft;
impl IconShape for MdKeyboardArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.41 16.59L10.83 12l4.58-4.59L14 6l-6 6 6 6 1.41-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K9xHPgp4GOAv+Z/DsR/vuPAbwEvDewCrwP8Nf8zIP5zHQd+C3hpnm0XeB3gr/nvh/jP9VfAS/O8doHXAf6a/16I/1zvDXwXz98u8DrAX/PfB/Gf772B7+L52wVeB/hr/nsg/mu8N/BdPH+7wOsAf81/PcR/nfcGvovnbxd4HeCv+a+F+K/13sB38fztAq8D/DX/dRD/9d4b+C6ev13gdYC/5r8G4r/HewPfxfO3C7wO8Nf850P893lv4Lt4/naB1wH+mv9ciP9e7w18F8/fLvA6wF/znwfx3++9ge/i+dsFXgf4a/5zIP5neG/gu3j+doHXAf6a/3iI/zneG/gunr9d4HWAv+Y/FuJ/jvcGvovn7xLw2sBf8x8L8T/DewPfxfN3CXht4K/5j4f47/fewHfx/F0CXhv4a/5zIP57vTfwXTx/l4DXBv6a/zyI/z7vDXwXz98l4LWBv+Y/F+K/x3sD38Xzdwl4beCv+c+H+K/33sB38fxdAl4b+Gv+ayD+a7038F08f5eA1wb+mv86iP867w18F8/fJeC1gb/mvxbiv8Z7A9/F83cJeG3gr/mvh/jP997Ad/H8XQJeG/hr/nsg/nO9N/BdPH+XgNcG/pr/Poj/XH8NvBTP6xLw2sBf898L8Z/rOPDbwEvxbJeA1wb+mv9+iP98x4HfBl4KuAS8NvDX/M+A+K9xHPhp4KOBv+Z/DsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHtWl5Boy4ybgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardArrowRight;
impl IconShape for MdKeyboardArrowRight {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+9Xpor/pr/Hoj/Pi8N/BZXvA7w1/zXQ/z3eGngt4DjXLELvA7w1/zXQvzXe2ngt4DjPKdd4HWAv+a/DuK/1ksDvwUc5/nbBV4H+Gv+ayD+67w08FvAcV64XeB1gL/mPx/iv8ZLA78FHOdFswu8DvDX/OdC/Od7aeC3gOP86+wCrwP8Nf95EP+5Xhr4LeA4z9/7cMV38fztAq8D/DX/ORD/eV4a+C3gOM/f+wDfzRXvDXwXz98u8DrAX/MfD/Gf46WB3wKO8/y9D/DdPKf3Br6L528XeB3gr/mPhfiP99LAbwHHef7eB/hunr/3Br6L528XeB3gr/mPg/iP9dLAbwHHef7eB/huXrj3Br6L528XeB3gr/mPgfiP89LAbwHHef7eB/huXjTvDXwXz98u8DrAX/Pvh/iP8dLAbwHHef7eB/hu/nXeG/gunr9d4HWAv+bfB/Hv99LAbwHHef7eB/hu/m3eG/gunr9d4HWAv+bfDvHv99vAa/H8vQ/w3fz7vDfwXTx/vwO8Nv92iH+/48BvAy/Fc3of4Lv5j/HewHfxnP4GeG1gl387xH+M48BvAy/FFe8DfDf/sd4b+C6u+BvgtYFd/n0Q/3GOA78NfDXw3fzneG/go4HXBnb590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPLjBOQTCgKTcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardArrowUp;
impl IconShape for MdKeyboardArrowUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+M/13sBrAe/D/0yI/zzvDXwXV3w38D78z4P4z/HewHfxnL4beB/+Z0H8x3tv4Lt4/r4G+Gj+50D8x3pv4Lt4/i4Brw38Nf9zIP7jvDfwXTx/l4DXBv6a/1kQ/zHeG/gunr9LwGsDf83/PIh/v/cGvovn7xLw2sBf8z8T4t/nvYHv4vm7BLw28Nf8z4X4t3tv4Lt4/i4Brw38Nf+zIf5t3hv4Lp6/S8BrA3/N/3yIf733Br6L5+8S8NrAX/O/A+Jf572B7+L5uwS8NvDX/O+BeNG9N/BdvGC3ArfyP8/r8IIhXjTvDXwX/zuJFwzxL3tv4Lv430u8YIh/2XsD38X/XuIFQ7xo3hv4Lv53Ei8Y4kX33sB38YI9A7iV/3lemxcM8a/z3sB38fztAq8D/DX/eyD+9d4b+C6ev13gdYC/5n8HxL/NewPfxfO3C7wO8Nf8z4f4t3tv4Lt4/naB1wH+mv/ZEP8+7w18F8/fLvA6wF/zPxfi3++9ge/i+dsFXgf4a/5nQvzHeG/gu3j+doHXAf6a/3kQ/3HeG/gunr9d4HWAv+Z/FsR/rPcGvovnbxd4HeCv+Z8D8R/vvYHv4vl7H+C7+Z8D8Z/jvYHv4jm9D/Dd/M+C+M/z3sB3ccX7AN/N/zyI/1zvzRXfzf9MiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I7s5TkF3j0ruAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardBackspace;
impl IconShape for MdKeyboardBackspace {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 11H6.83l3.58-3.59L9 6l-6 6 6 6 1.41-1.41L6.83 13H21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/Pf6Lq54H/57IP77fBfw3lzx3cD78F8P8d/ju4D35jl9N/A+/NdC/Nf7LuC9ef6+G3gf/usg/mt9F/DevHDfDbwP/zUQ/3W+C3hvXjTfDbwP//kQ/zW+C3hv/nW+G3gf/nMh/vN9F/DePH9/wxUvxfP33cD78J8H8Z/ru4D35vn7G+C1ueK3gZfi+ftu4H34z4H4z/NdwHvz/P0N8NrALlccB34beCmev+8G3of/eIj/HN8FvDfP398Arw3s8pyOA78NvBTP33cD78N/LMR/vO8C3pvn72+A1wZ2ef6OA78NvBTP33cD78N/HMR/rO8C3pvn72+A1wZ2eeGOA78NvBTP33cD78N/DMR/nO8C3pvn72+A1wZ2edEcB34beCmev+8G3od/P8R/jO8C3pvn72+A1wZ2+dc5Dvw28FI8f98NvA//Poh/v+8C3pvn72+A1wZ2+bc5Dvw28FI8f98NvA//doh/v/cGvovn9TfAawO7/PscB34beCme1/sA382/HeI/xnsD38Wz/Q3w2sAu/zGOA78NvBTP9j7Ad/Pvg/iP897AdwF/A7w2sMt/rOPAbwMvBbwP8N38+yH+Y7018NvALv85jgOvDfw0/zEQ/78h/n9D/P+G+P8N8f8b4v83xP9viH+/1+K/1+/wb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjXoE9BgRQxxgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardCapslock;
impl IconShape for MdKeyboardCapslock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 8.41L16.59 13 18 11.59l-6-6-6 6L7.41 13 12 8.41zM6 18h12v-2H6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOCngJfmf7e/Bt4GuJXnhXjB/gp4af5v+GvgZXheiOfvrYGf4v+WtwF+mueEeP4+G/gs/m/5HOCzeU6I5++ngbfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4LV6w1wF+m+f02sBvccXrAL/Nc3pt4Le44nWA3+Y5vTbwW1zxOsBv85xeG/gtrngd4Ld5Tq8N/BYv2O8Ar81zQjx/vw28Fi/Y6wC/zXN6beC3uOJ1gN/mOb028Ftc8TrAb/OcXhv4La54HeC3eU6vDfwWV7wO8Ns8p9cGfosX7HeA1+Y5IZ6/3wZeixfsdYDf5jm9NvBbXPE6wG/znF4b+C2ueB3gt3lOrw38Fle8DvDbPKfXBn6LK14H+G2e02sDv8UL9jvAa/OcEM/fbwOvxQv2OsBv85xeG/gtrngd4Ld5Tq8N/BZXvA7w2zyn1wZ+iyteB/htntNrA7/FFa8D/DbP6bWB3+IF+x3gtXlOiOfvt4HX4gV7HeC3eU6vDfwWV7wO8Ns8p9cGfosrXgf4bZ7TawO/xRWvA/w2z+m1gd/iitcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8YL9NbDLczoOvDRX/DWwy3M6Drw0V/w1sMtzOg68NFf8NbDLczoOvDRX/DWwy3M6Drw0L9jvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fi/Y6wC/zXN6beC3uOJ1gN/mOb028Ftc8TrAb/OcXhv4La54HeC3eU6vDfwWV7wO8Ns8p9cGfosX7HeA1+Y5IZ6/3wZeixfsdYDf5jm9NvBbXPE6wG/znF4b+C2ueB3gt3lOrw38Fle8DvDbPKfXBn6LK14H+G2e02sDv8UL9jvAa/OcEM/fbwOvxQv2OsBv85xeG/gtrngd4Ld5Tq8N/BZXvA7w2zyn1wZ+iyteB/htntNrA7/FFa8D/DbP6bWB3+IF+x3gtXlOiOfvt4HX4gV7HeC3eU6vDfwWV7wO8Ns8p9cGfosrXgf4bZ7TawO/xRWvA/w2z+m1gd/iitcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8YK9DvDbPKfXBn6LK14H+G2e02sDv8UVrwP8Ns/ptYHf4orXAX6b5/TawG9xxesAv81zem3gt3jBfgd4bZ4T4vn7beC1eMH+GtjlOR0HXpor/hrY5TkdB16aK/4a2OU5HQdemiv+GtjlOR0HXpor/hrY5TkdB16aF+x3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6Lf7u/AXb5j3UceCn+7X4HeG2eE+L5+23gtfi3ex3gt/mP9drAb/Fv9zvAa/OcEM/fbwOvxb/d6wC/zX+s1wZ+i3+73wFem+eEeP5+G3gt/u1eB/ht/mO9NvBb/Nv9DvDaPCfE8/fbwGvxb/c6wG/zH+u1gd/i3+53gNfmOSGev98GXot/u9cBfpv/WK8N/Bb/dr8DvDbPCfH8fTfwXvzf8jXAR/OcEM/fRwNfxf8tHwN8Nc8J8fwdB24FjvF/wyXgwcAuzwnxgr038F383/A2wE/zvBAv3GsD3w08iP+dngG8N/DbPH+IF81LA8f532UX+GteOMT/b4j/3xD/vyH+f0P853gw8CD+Yz0DuJX/WIj/HMeB3wZeiv8YfwO8NrDLfyzEf57jwG8DL8W/z98Arw3s8h8P8Z/rOPDbwEvxb/M3wGsDu/znQPznOw78NvBS/Ov8DfDawC7/eRD/NY4Dvw28FC+avwFeG9jlPxfiv85x4LeBl+KF+xvgtYFd/vMh/msdB34beCmev78BXhvY5b8G4r/eceC3gZfiOf0N8NrALv91EP89jgO/DbwUV/wN8NrALv+1EP99jgO/zRWvDezyXw/x3+s4V+zy3wPx/xvi/zf+Ebyz9UHHfQi3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardHide;
impl IconShape for MdKeyboardHide {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 3H4c-1.1 0-1.99.9-1.99 2L2 15c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 3h2v2h-2V6zm0 3h2v2h-2V9zM8 6h2v2H8V6zm0 3h2v2H8V9zm-1 2H5V9h2v2zm0-3H5V6h2v2zm9 7H8v-2h8v2zm0-4h-2V9h2v2zm0-3h-2V6h2v2zm3 3h-2V9h2v2zm0-3h-2V6h2v2zm-7 15l4-4H8l4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+I/30sBXAW8D7PI/G+I/1ksDvwUcB/4aeB1gl/+5EP9xXhr4LeA4z/bXwOsAu/znOw68FM/rd3jBEP8xXhr4LeA4z+uvgZfhP99rA7/F8xIvGOLf76WB3wKO8/y9D/Dd/Od7beC3eF7iBUP8+7w08FvAcZ6/9wG+m/8arw38Fs9LvGCIf7uXBn4LOM7z9z7Ad/Nf57WB3+J5iRcM8W/z0sBvAcd5/t4H+G7+a7028Fs8L/GCIf71Xhr4LeA4z9/7AN/Nf73XBn6L5yVeMMS/zksDvwUc5/l7H+C7+e/x2sBv8bzEC4Z40b008FvAcZ6/9wG+m/8+rw38Fs9LvGCIF81LA78FHOf5ex/gu/nv9drAb/G8xAuG+Je9NPBbwHGev/cBvpv/fq8N/BbPS7xgiBfupYHfAo7z/O0Cf83/DMeBl+Z5iRcM8YK9NPBbwHH+dxMvGOL5e2ngt4Dj/O8nXjDE8/fSwG8Dx/jfT7xgiBfspYHfBo7xv5t4wRAv3EsDvw0c4/m7BPw1/7O9Ni8Y4l/20sBvA8d4/t4H+G7+d0K8aF4a+G3gGM/f+wDfzf8+iBfdSwO/DRzj+Xsf4Lv53wXxr/PSwG8Dx3j+3gf4bv73QPzrvTTw28Axnr/3Ab6b/x0Q/zYvDfw2cIzn732A7+Z/PsS/3UsDvw0c4/l7H+C7+Z8N8e/z0sBvA8d4/t4H+G7+50L8+7008NvAMZ6/9wG+m/+ZEP8xXhr4beAYz+tvgJfmfybEf5yXBn4bOMaz/Q3w2sAu/zMh/mO9NPDbwDHgb4DXBnb5nwvxH++lga8G3hrY5X82xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EfNqTUFcOHRmAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardReturn;
impl IconShape for MdKeyboardReturn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 7v4H5.83l3.58-3.59L8 6l-6 6 6 6 1.41-1.41L5.83 13H21V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/hOPBTwMcAf82/328Dr8Vz+h3gtXlOiP9+x4HfAl4a2AVeB/hr/n1+G3gtntPvAK/Nc0L89zoO/Bbw0jzbLvA6wF/zb/fbwGvxnH4HeG2eE+K/118BL83z2gVeB/hr/m1+G3gtntPvAK/Nc0L893pv4Lt4/naB1wH+mn+93wZei+f0O8Br85wQ//3eG/gunr9d4HWAv+Zf57eB1+I5/Q7w2jwnxP8M7w18F8/fLvA6wF/zovtt4LV4Tr8DvDbPCfE/x3sD38Xztwu8DvDXvGh+G3gtntPvAK/Nc0L8z/LewHfx/O0CrwP8Nf+y3wZei+f0O8Br85wQ//O8N/BdPH+7wOsAf80L99vAa/Gcfgd4bZ4T4n+m9wa+i+dvF3gd4K95wX4beC2e0+8Ar81zQvzP9d7Ad/H87QKvA/w1z99vA6/Fc/od4LV5Toj/2d4b+C6ev13gdYC/5nn9NvBaPKffAV6b54R4tpcGvor/eV4aOM7ztwu8DvDXPKffBl6L5/Q7wGvznBDP9trAb/G/zy7wOsBf82y/DbwWz+l3gNfmOSGe7bWB3+J/p13gdYC/5orfBl6L5/Q7wGvznBDP9trAb/G/0yXgtYG/5orfBl6L5/Q7wGvznBDP9trAb/G/zyXgtYG/5tl+G3gtntPvAK/Nc0I820sDX83/PC8NHOP5uwS8NvDXPKffBl6L5/Q7wGvznBD/s7038F08f5eA1wb+muf128Br8Zx+B3htnhPif673Br6L5+8S8NrAX/P8/TbwWjyn3wFem+eE+J/pvYHv4vm7BLw28Ne8YL8NvBbP6XeA1+Y5If7neW/gu3j+LgGvDfw1L9xvA6/Fc/od4LV5Toj/Wd4b+C6ev0vAawN/zb/st4HX4jn9DvDaPCfE/xzvDXwXz98l4LWBv+ZF89vAa/Gcfgd4bZ4T4n+G9wa+i+fvEvDawF/zovtt4LV4Tr8DvDbPCfHf772B7+L5uwS8NvDX/Ov8NvBaPKffAV6b54T47/XewHfx/F0CXhv4a/71fht4LZ7T7wCvzXNC/Pf6a+CleF6XgNcG/pp/m98GXovn9DvAa/OcEP+9jgO/DbwUz3YJeG3gr/m3+23gtXhOvwO8Ns8J8d/vOPDbwEsBl4DXBv6af5/fBl6L5/Q7wGvznBD/MxwHfhr4aOCv+ff7beC1eE6/A7w2zwnx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EWOKeQcp/msgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardTab;
impl IconShape for MdKeyboardTab {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.59 7.41L15.17 11H1v2h14.17l-3.59 3.59L13 18l6-6-6-6-1.41 1.41zM20 6v12h2V6h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/dg4KOA1wZemiv+Gvht4GuAW/mvg/iv9VXAR/PCfTXwMfzXQPzX+SvgpXnR/DXwMvznQ/zX+Grgo/jX+Rrgo/nPhfjP92Dg6fzbPAS4lf88iP98Xw18FP82XwN8NP95EP/5/gp4af5t/hp4Gf7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxb/PdwHvxnMTzZ/59xPNnntPXAB/Nvw7i3+azgc/iOYnn76+Bl+Lf5neA1+b5M8/pc4DP5l8H8W/z2cBn8ZweAtzK8/pq4KP4t/ka4KN5Xg8Gns5z+hjgq/nXQfzbvDbwWzyn9wG+m+f1YODp/Ns8BLiV5/XewHfxnF4H+G3+dRD/duY5/Qzw1jx/Xw18FP86XwN8NM/fTwNvxbNdAo7zr4f4t/tp4K14TieAXZ6/vwZeihfN7wCvzfN3HLjIc/oZ4K3510P827038F08p+8B3psX7KuBj+KF+xrgo3nBvht4L57T+wDfzb8e4t/nVuBBPKfXAX6bF+zBwEcDrw28FFf8DfDbwFcDt/KCvTbwWzynZwAP5t8G8e/z3sB38Zx2gYcAu/zHejDwV8BxntP7AN/Nvw3i3++vgZfiOf018DrALv8xjgO/Bbw0z+lvgJfm3w7x73ccuBU4xnO6FXgb4K/593lp4KeAB/OcLgEPBnb5t0P8x3ht4Ld4/j4b+Bpgl3+d48BHAZ/N8/c6wG/z74P4j/PewHfx/O0CXw38DPDXvHAvDbwV8NHAcZ6/9wG+m38/xH+slwZ+GzjGC3Yr8NfAX/OcXhp4aeDBvGCXgNcG/pr/GIj/eA8Gvht4Lf5j/Q7w3sCt/MdB/Of5aOCzgWP8+1wCPhv4av7jIf5zHQfeG/ho4EH86zwD+Grgu4Fd/nMg/uu8NvDawGsDLw0c4zldAv4a+G3gt4Hf5j8f4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x1cDL81z+mvgo/mvhfj/DfH/G+L/N8T/b4j/3/hHDRGSQQ3D+n0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKeyboardVoice;
impl IconShape for MdKeyboardVoice {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 15c1.66 0 2.99-1.34 2.99-3L15 6c0-1.66-1.34-3-3-3S9 4.34 9 6v6c0 1.66 1.34 3 3 3zm5.3-3c0 3-2.54 5.1-5.3 5.1S6.7 15 6.7 12H5c0 3.42 2.72 6.23 6 6.72V22h2v-3.28c3.28-.48 6-3.3 6-6.72h-1.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvo4Gv4v+WjwG+mueEeP6+G3gv/m/5HuC9eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntNfAx/N/w5fDbw0z+l3gNfmOSGev88GPov/Wz4H+GyeE+L5e2vgp/i/5W2An+Y5IV6wvwZeiv8b/gZ4aZ4X4gV7MPDTwEvxv9vfAG8N3MrzQrxwx4GPBt4beBD/uzwD+G7gq4Fdnj8EmP+/EGD+/0KA+f8LAeb/LwSY/78Q8Nr8/4X4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wg1iYx8EmImXgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLaptop;
impl IconShape for MdLaptop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,18c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v10c0,1.1,0.9,2,2,2H0v2h24v-2H20z M4,6h16v10H4V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfiOf0N8NH87/DVwEvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4j/U3wEfzovlq4KX4j/U7wGvznBDP328Dr8V/rN8BXpsXzW8Dr8V/rN8BXpvnhHj+fht4Lf5j/Q7w2rxofht4Lf5j/Q7w2jwnBJj/vxBg/v9CgPn/CwHm/y8EmP+/EPDa/P+F+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8InPiRfPp8PBEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLaptopChromebook;
impl IconShape for MdLaptopChromebook {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 18V3H2v15H0v2h24v-2h-2zm-8 0h-4v-1h4v1zm6-3H4V5h16v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxnP4G+Gj+d/hq4KV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p78GPpr/Hb4aeGme088Ab81zQjx/nw18Fv+3fA7w2TwnxPP31sBP8X/L2wA/zXNCvGB/DbwU/3n+Bvhurnhv4KX4z/M7wGvzvBAv2IOBnwZeiv94fwO8NM/pr4GX4j/e3wCvDezyvBAv3HHgo4H3Bh7Ef5yPAb6a5/TRwFfxH+cZwHcDXw3s8vwhwPzX+xjgq3lOHw18Ff+1EPDXwEvxX+uvgZfhOf0V8NL81/odAe8NfBf/9f4a+G6ueG/gpfmv9z7iiu8G3ov/X74HeG/xbJ8NfDRwjP/bLgFfDXw2gHher83/bb/NsyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJkZ6hgMnAmfQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLaptopMac;
impl IconShape for MdLaptopMac {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2H0c0 1.1.9 2 2 2h20c1.1 0 2-.9 2-2h-4zM4 5h16v11H4V5zm8 14c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxnP4G+Gj+d/hq4KV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP33cD78X/LZ8DfDbPCfH8fTTwVfzf8j7Ad/OcEM/fceBW4Bj/N1wCHgzs8pwQL9h7A9/F/w1vA/w0zwvxwr028N3Ag/jf6RnAewO/zfOHeNG8NPDawFfxv8PHAL8N/DUvHOJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhgDz/xcCzP9fCDD/fyHA/P+FAPP/FwJem/+/EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAapKkXxTMruvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLaptopWindows;
impl IconShape for MdLaptopWindows {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18v-1c1.1 0 1.99-.9 1.99-2L22 5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2v1H0v2h24v-2h-4zM4 5h16v10H4V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/i3++lgbcCXho4zvN6bZ7XXwO7PKfjwEvzvH6b57UL/DXwM8Bf82+H+Pf5KuCj+e/11cDH8G+D+Lf7auCj+J/ha4CP5l8P8W/z0sBf8T/LQ4Bb+ddB/Nt8NvBZ/M/yOcBn86+D+Lf5aeCteE6XgL/mv8ZLA8d4Tj8DvDX/Ooh/m98GXovn9DvAa/Nf47eB1+I5/Q7w2vzrIP5tfht4LZ7T7wCvzX+N3wZei+f0O8Br86+D+Lf5beC1eE6/A7w2/zV+G3gtntPvAK/Nvw7i3+a3gdfiOf0O8Nr81/ht4LV4Tr8DvDb/Ooh/m98GXovn9DvAa/Nf47eB1+I5/Q7w2vzrIP5l5kXzO8Br87zMv494Xr8NvBYvGvGCIf5l5kXzO8Br87zMv494Xr8NvBYvGvGCIf5l5kXzO8Br87zMv494Xr8NvBYvGvGCIf5l5kXzO8Br87zMv494Xr8NvBYvGvGCIf5l5kXzO8Br87zMv494Xr8NvBYvGvGCIf5lr83z+mrgpXhOvwO8Ns/LPK+PAf6a5/TSwFfxvMTz+m3gtXhOfwN8NM/rt3nBEP82vw28Fs/pd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fs/pd4DX5l8H8W/z28Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5ief128Br8Zx+B3ht/nUQ/za/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG3+dRD/stfieX018NI8p98BXpvnZZ7XRwN/zXN6aeCreV7ief028Fo8p78GPprn9Tu8YIh/mXnR/A7w2jwv8+8jntdvA6/Fi0a8YIh/mXnR/A7w2jwv8+8jntdvA6/Fi0a8YIh/mXnR/A7w2jwv8+8jntdvA6/Fi0a8YIh/mXnR/A7w2jwv8+8jntdvA6/Fi0a8YIh/mXnR/A7w2jwv8+8jntdvA6/Fi0a8YIh/m98GXovn9DvAa/Nf47eB1+I5/Q7w2vzrIP5tfht4LZ7T7wCvzX+N3wZei+f0O8Br86+D+Lf5beC1eE6/A7w2/zV+G3gtntPvAK/Nvw7i3+a3gdfiOf0O8Nr81/ht4LV4Tr8DvDb/Ooh/m98GXovn9DvAa/Nf47eB1+I5/Q7w2vzrIP5tfhp4K57TLvDX/Nd4aeA4z+lngLfmXwfxb/PZwGfxP8vnAJ/Nvw7i3+algb/if5aHALfyr4P4t/tq4KP4n+FrgI/mXw/x7/PVwEfx3+trgI/m3wbx7/fSwFsDLw0c53m9Fs/rb4BdntNx4KV4Xr/D89oF/hr4buBW/u0Q//nM83od4Ld5Tq8N/BbPS/znQfznM8/rdYDf5jm9NvBbPC/xnwfxn888r9cBfpvn9NrAb/G8xH8exH8+87xeB/htntNrA7/F8xL/eRD/+czzeh3gt3lOrw38Fs9L/OdB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BN7vAQVN/1zcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMemory;
impl IconShape for MdMemory {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 9H9v6h6V9zm-2 4h-2v-2h2v2zm8-2V9h-2V7c0-1.1-.9-2-2-2h-2V3h-2v2h-2V3H9v2H7c-1.1 0-2 .9-2 2v2H3v2h2v2H3v2h2v2c0 1.1.9 2 2 2h2v2h2v-2h2v2h2v-2h2c1.1 0 2-.9 2-2v-2h2v-2h-2v-2h2zm-4 6H7V7h10v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxnP4G+Gj+d/hq4KV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p78GPpr/Hb4aeGme0+8Ar81zQjx/nw18Fv+3fA7w2TwnxPP31sBP8X/L2wA/zXNCvGB/DbwU/zf8DfDSPC/EC/Zg4KeBl+J/t78B3hq4leeFeOGOAx8NvDfwIP53eQbw3cBXA7s8f4j/GMeB3wJemv8afw28DrDLvw/i3+848FvAS/Nf66+B1wF2+bdD/Pv9FfDS/Pf4a+Bl+LdD/PuZ/17i3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PuZ/17i3w7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwR2e59BHATvfAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMonitor;
impl IconShape for MdMonitor {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {

        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8R/jbcCXhp4beC1gdcBfpvn9NrAbwG/Dfw28NfAz/CfC/Gf5zjwUcB7Aw/mOb0O8Ns8p9cGfovndCvw1cD3ALv8x0P85/gs4LN5wV4H+G2e02sDv8UL9tnA5/AfC/Ef67WBrwJemhfudYDf5jm9NvBbvHB/DXwM8Nv8x0D8x/ko4Kt50bwO8Ns8p9cGfosXzfsA382/H+I/xncB782L7nWA3+Y5vTbwW7zovht4H/59EP9+3wW8N/86rwP8Ns/ptYHf4l/nu4H34d8O8e/z0cBX8a/3OsBv85xeG/gt/vU+Bvhq/m0Q/3YvDfwV/zavA/w2z+m1gd/i3+ZlgL/mXw/xb3Mc+CvgwfzbvA7w2zyn1wZ+i3+bW4GXAXb510H823w28Fn8270O8Ns8p9cGfot/u88BPpt/HcS/3nHg6cBx/u1eB/htntNrA7/Fv90u8BBglxcd4l/vs4HP4t/ndYDf5jm9NvBb/Pt8DvDZvOgQ/3oXgeP8+7wO8Ns8p9cGfot/n13gBC86xL/OawO/xb/f6wC/zXN6beC3+Pd7HeC3edEg/nW+Gvgo/v1eB/htntNrA7/Fv9/XAB/Niwbxr/NXwEvz7/c6wG/znF4b+C3+/f4aeBleNIh/HfMf43WA3+Y5vTbwW/zHEC8axIvuOHCR/xivA/w2z+m1gd/iP8YJYJd/GeJF99rAb/Ef43WA3+Y5vTbwW/zHeB3gt/mXIV50rw38Fv8xXgf4bZ7TawO/xX+M1wF+m38Z4v83xP9viP/fEP+/If5/Q7zoXhr4Kv53+Bjgr/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffSwF/xv8PLAH/Nvwzxr2P+dxAvGsS/zq3Ag/if7RnAg3nRIP51vht4L/5n+xrgo3nRIP513hr4Kf5nexvgp3nRIP71doFj/M90CTjOiw7xr/fdwHvxP9PXAB/Niw7xr/dg4On8z/QQ4FZedIh/m68GPor/Wb4G+Gj+dRD/NseBW4Fj/M/wDOClgV3+dRD/dq8N/Bb/M7wO8Nv86yH+fT4b+Cz+e30O8Nn82yD+/b4beC/+e3wP8N782yH+Y3w38F781/oe4L3590H8x/ls4LP4r/E5wGfz74f4j/XawE8Dx/jP8QzgvYHf5j8G4j/eceCzgY/iP9bXAJ8N7PIfB/Gf58HARwPvDRzj3+YS8N3AVwO38h8P8V/jrYHXBt4aeBAv3DOAnwZ+G/hp/nMh/nu8NHCc57QL/DX/tRD/vyH+f+MfARyCokFhp3iLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMouse;
impl IconShape for MdMouse {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 1.07V9h7c0-4.08-3.05-7.44-7-7.93zM4 15c0 4.42 3.58 8 8 8s8-3.58 8-8v-4H4v4zm7-13.93C7.05 1.56 4 4.92 4 9h7V1.07z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/1oOBB/Gf6xnArfzHQPzHeGvgs4CX5r/GXwOfA/w0/z6If5/jwFcB781/j+8GPgbY5d8G8e/zW8Br89/rp4G34d8G8W/32cBn8T/D5wCfzb8e4t/mOPB04Dj/M+wCDwF2+ddB/Nu8N/Bd/M/yNsBP86+D+Lf5bOCz+J/lc4DP5l8H8W/z08Bb8Zx+h/9ar8Vz+hngrfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/FcxL/tcxz+h3gtfnXQfzb/DbwWjwn8V/LPKffAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5aeCt+Nf5G+CjedF8NfBS/Ov8DPDW/Osg/m0+G/gs/nV+B3htXjS/DbwW/zqfA3w2/zqIf5v3Br6Lf53fAV6bF81vA6/Fv87bAD/Nvw7i3+Y4cCtwjP8ZLgEPBnb510H823028Fn8z/A5wGfzr4f49/lt4LX47/UzwFvzb4P49zkOfDXwXvz3+B7go4Fd/m0Q/zHeGvhs4KX4r/E3wGcDP82/D+I/1oOBB/Of61bgVv5jIP5/Q/z/xj8Cw0OqQZWNwVUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneAndroid;
impl IconShape for MdPhoneAndroid {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 1H8C6.34 1 5 2.34 5 4v16c0 1.66 1.34 3 3 3h8c1.66 0 3-1.34 3-3V4c0-1.66-1.34-3-3-3zm-2 20h-4v-1h4v1zm3.25-3H6.75V4h10.5v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneIphone;
impl IconShape for MdPhoneIphone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.5 1h-8C6.12 1 5 2.12 5 3.5v17C5 21.88 6.12 23 7.5 23h8c1.38 0 2.5-1.12 2.5-2.5v-17C18 2.12 16.88 1 15.5 1zm-4 21c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4.5-4H7V4h9v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7sE/DXPCfHCvTbwXcCD+d/vd4DX5jkhXrD3Br6L/zt+B3htnhPi+TsOPB04zv8dvwO8Ns8J8fx9NPBV/N/yO8Br85wQz993A+/F/y2/A7w2zwnx/P028Fo8p98BXpv/WxDP328Dr8Vz+h3gtfm/BfH8/TbwWjyn3wFem/9bEM/fbwOvxXP6HeC1+b8F8fz9NvBaPKffAV6b/1sQz99vA6/Fc/od4LX573UceC/grfn3uRX4GeCnxfP328Br8Zx+B3ht/nv9FfDS/Mf5HPH8/TbwWjyn3wFem/8+rw38Fv+xbhXP328Dr8Vz+h3gtfnv89rAb/EfC/H8/TbwWjyn3wFem/8+rw38Fv+xEM/fbwOvxXP6HeC1+e/z2sBv8bx+hxfNceCleE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSGev98GXovn9DvAa/Pf57WB3+J5iRfNawO/xXNCPH+/DbwWz+l3gNfmv89rA7/F8xIvmtcGfovnhHj+fht4LZ7T7wCvzX+f1wZ+i+clXjSvDfwWzwnx/P028Fo8p98BXpv/Pq8N/BbPS7xoXhv4LZ4T4vn7beC1eE6/A7w2/31eG/gtnpd40bw28Fs8J8Tz99vAa/Gcfgd4bf77vDbwWzwv8aJ5beC3eE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSGev98GXovn9DvAa/Pf57WB3+J5iRfNawO/xXNCPH+/DbwWz+l3gNfmv89rA7/F8xIvmtcGfovnhHj+fht4LZ7T7wCvzX+f1wZ+i+clXjSvDfwWzwnx/P028Fo8p98BXpv/Pq8N/BbPS7xoXhv4LZ4T4vn7beC1eE6/A7w2/31eG/gtnpd40bw28Fs8J8Tz99vAa/Gcfgd4bf77vDbwWzwv8aJ5beC3eE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSHgt3heLw0c5zntAn/Nf42PAf6a5/TawG/xvH6bF81x4KV5Tggw//O8DvDbPKfXBn6L/1gIMP/zvA7w2zyn1wZ+i/9YCDD/87wO8Ns8p9cGfov/WAgw//O8DvDbPKfXBn6L/1gIMP/zvA7w2zyn1wZ+i/9YCDD/87wO8Ns8p5cG/or/WH8jwPzP8zrAb/O8Phv4LP5jXAJeW8Br8z/PXwO7PH8vDRzn3++vgV3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHqopWFG0LLXwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelink;
impl IconShape for MdPhonelink {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6h18V4H4c-1.1 0-2 .9-2 2v11H0v3h14v-3H4V6zm19 2h-6c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zm-1 9h-4v-7h4v7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelinkOff;
impl IconShape for MdPhonelinkOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 6V4H6.82l2 2H22zM1.92 1.65L.65 2.92l1.82 1.82C2.18 5.08 2 5.52 2 6v11H0v3h17.73l2.35 2.35 1.27-1.27L3.89 3.62 1.92 1.65zM4 6.27L14.73 17H4V6.27zM23 8h-6c-.55 0-1 .45-1 1v4.18l2 2V10h4v7h-2.18l3 3H23c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fi8NvBXw0sBx/mvsAn8N/Azw1/zbIf59vgr4aP57fTXwMfzbIP7tvhr4KP5n+Brgo/nXQ/zbvDTwV/zP8hDgVv51EP82nw18Fv+zfA7w2fzrIP5tfhp4K57TJeCv+a/x0sAxntPPAG/Nvw7i3+a3gdfiOf0O8Nr81/ht4LV4Tr8DvDb/Ooh/m98GXovn9DvAa/Nf47eB1+I5/Q7w2vzrIP5tfht4LZ7T7wCvzX+N3wZei+f0O8Br86+D+Lf5beC1eE6/A7w2/zV+G3gtntPvAK/Nvw7i3+a3gdfiOf0O8Nr81/ht4LV4Tr8DvDb/Ooh/m+8G3ov/Wb4G+Gj+dRD/Nh8NfBX/s3wM8NX86yD+bY4DtwLH+J/hEvBgYJd/HcS/3XsD38X/DG8D/DT/eoh/n9cGvht4EP89ngG8N/Db/Nsg/mO8NHCc/1q7wF/z74P4/w3x/xvi3+/BwGcBrw08mP8atwK/DXwOcCv/doh/n5cGfgs4zn+PXeB1gL/m3wbxb3cceDpwnP9eu8BDgF3+9RD/dl8NfBT/M3wN8NH86yH+bY4DTweO8z/DLvAQYJd/HcS/zUcDX8UL9jfAVwO3csWDgY8GXoor/gb4auBWrngw8NHAS3HF3wBfDdzKFQ8GPhp4KV6wjwG+mn8dxL/N04EH84K9D/DdPKePBr6KK94H+G6e00cDX8UV7wN8N8/po4Gv4gW7FXgI/zqIf733Br6LF+6ngbfhOf0U8NZc8dPA2/Ccfgp4a674aeBteE4/Bbw1L9z7AN/Niw7xr/dbwGvzP9NvA6/Diw7xr/PawG/xP9vrAL/Niwbxr/PdwHvxovkdntNr8Zx+h+f0Wjyn3+E5vRYvmu8B3psXDeJF92Dg6bxoPgf4bJ7TZwOfxRWfA3w2z+mzgc/iis8BPpvn9NnAZ/GieQhwK/8yxIvuq4GP4kXzOcBn85w+G/gsrvgc4LN5Tp8NfBZXfA7w2TynzwY+ixfN1wAfzb8M8aI5DjwdOM6L7rd5Tq/Nc/ptntNr85x+m+f02rzodoGHALu8cIgXzUcDX8X/Lh8DfDUvHOJF83TgwfzvcivwEF44xL/svYHv4l/nZ4C35jn9NPBWXPEzwFvznH4aeCuu+BngrXlOPw28Ff867wN8Ny8Y4l/2W8Br86/zPsB385w+Gvgqrngf4Lt5Th8NfBVXvA/w3Tynjwa+in+d3wZehxcM8cK9NvBb/Ov9NfDVwK1c8WDgo4GX5oq/Br4auJUrHgx8NPDSXPHXwFcDt3LFg4GPBl6af73XAX6b5w/xwn038F787/Y9wHvz/CFesAcDT+f/hocAt/K8EC/YVwMfxf8NXwN8NM8L8fwdB54OHOf/hl3gIcAuzwnx/H008FX83/IxwFfznBD/vyH+f0M8fy8NHOP/lkvAX/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99PA2/F/y0/A7w1zwnx/H028Fn83/I5wGfznBDP30sDf8X/LQ8BbuU5IV6wrwY+iv8bvgb4aJ4X4oX7auCj+N/ta4CP5vlD/MteGnhr4KWB4/zvsAv8NfDdwK28YIj/3xD/vyH+f0P8/4b4/41/BN8PuUG7JpxaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPointOfSale;
impl IconShape for MdPointOfSale {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,2H7C5.9,2,5,2.9,5,4v2c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V4C19,2.9,18.1,2,17,2z M17,6H7V4h10V6z M20,22H4 c-1.1,0-2-0.9-2-2v-1h20v1C22,21.1,21.1,22,20,22z M18.53,10.19C18.21,9.47,17.49,9,16.7,9H7.3c-0.79,0-1.51,0.47-1.83,1.19L2,18 h20L18.53,10.19z M9.5,16h-1C8.22,16,8,15.78,8,15.5C8,15.22,8.22,15,8.5,15h1c0.28,0,0.5,0.22,0.5,0.5C10,15.78,9.78,16,9.5,16z M9.5,14h-1C8.22,14,8,13.78,8,13.5C8,13.22,8.22,13,8.5,13h1c0.28,0,0.5,0.22,0.5,0.5C10,13.78,9.78,14,9.5,14z M9.5,12h-1 C8.22,12,8,11.78,8,11.5C8,11.22,8.22,11,8.5,11h1c0.28,0,0.5,0.22,0.5,0.5C10,11.78,9.78,12,9.5,12z M12.5,16h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,15.78,12.78,16,12.5,16z M12.5,14h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,13.78,12.78,14,12.5,14z M12.5,12h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C13,11.78,12.78,12,12.5,12z M15.5,16h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,15.78,15.78,16,15.5,16z M15.5,14h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,13.78,15.78,14,15.5,14z M15.5,12h-1 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h1c0.28,0,0.5,0.22,0.5,0.5C16,11.78,15.78,12,15.5,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnj+fht4Lf5vEC8Y4vn7beC1+L9BvGCI5++3gdfi/wbxgiGev98GXov/G8QLhnj+fht4Lf5vEC8Y4vl7aeA4/zf8Ni8Y4v83xP9viP/fEP+/IZ6/lwaO8a9zCfhrntdr8a/3N8Auz+k48FL86/0OLxji+ftt4LX41/kd4LV5XuZf73WA3+Y5vTbwW/zriRcM8fz9NvBa/Ov8DvDaPC/zr/c6wG/znF4b+C3+9cQLhnj+fht4Lf51fgd4bZ6X+dd7HeC3eU6vDfwW/3riBUM8f78NvBb/Or8DvDbPy/zrvQ7w2zyn1wZ+i3898YIhnr/fBl6Lf53fAV6b52X+9V4H+G2e02sDv8W/nnjBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EeaECRByOlAAgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPowerInput;
impl IconShape for MdPowerInput {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 9v2h19V9H2zm0 6h5v-2H2v2zm7 0h5v-2H9v2zm7 0h5v-2h-5v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4n+mlgdcC3porXho4zhW/zRV/Dfw28DP82yH+5zgOfBTw3sCDedHtAr8NfA3w2/zrIP77HQc+Cvho4Dj/Pt8NfA5wKy8axH+vlwZ+CzjO83oG8NPArcBf82wPBh4MvDbwWjx/Hw18Df8yxH+f9wa+i+d0Cfhq4LuBW/mXHQfeGvhs4EE8p+8G3ocXDvHf46OBr+I5fQ7w1cAu/zbvDXw28CCe7XeA1+YFQ/z3OA78NvBSwDOAtwb+mud0HHgr4K2B4zynnwZ+B/hrntNx4LuBtwIuAa8N/DUvGOK/z3Hgs4HPBnZ5tgcDnwW8N/+yvwY+BvhtntNHA78N/DUvHOJ/ls8CPpt/vd8G3gbY5V8H8T/HdwHvzXN6BvDTwC7w18CDgePAWwMvxXP6a+B9gL/mRYf4n+O9ge/iir8BPhr4bV6wBwOfDbwXV/wN8NrALi86xP8s7w28NvDevOjeG3hv4K2BXf51EP+/If5/Q/z/hvj/DfE/n3lerwP8Nv9+iP/5zPN6HeC3+fdD/M9nntfrAL/Nvx/ifz7zvF4H+G3+/RD/85nn9TrAb/Pvh/ifzzyv1wF+m38/xP985nm9DvDb/Psh/uczz+t1gN/m3w/xP595Xq8D/Db/foj/+czzeh3gt/n3Q/zrvRb/tX6b5/XRwF/z/P0OLzrEi+6zgI8GjvM/2y7w1cDn8C9DvGi+C3hv/nf5buB9eOEQ/7L3Br6L/53eB/huXjDEv+yvgJfmf6ffAV6bFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzMv3O/wvF4aOMaz/Q2wy3M6DrwUz3YJ+Gue12vx7yNeMMS/zLxw4nn9NvBaPNvrAL/Nc3pt4Ld4tt8BXpvnZf59xAuG+JeZF048r98GXotnex3gt3lOrw38Fs/2O8Br87zMv494wRD/MvPCief128Br8WyvA/w2z+m1gd/i2X4HeG2el/n3ES8Y4l9mXjjxvH4beC2e7XWA3+Y5vTbwWzzb7wCvzfMy/z7iBUP8y8wLJ57XbwOvxbO9DvDbPKfXBn6LZ/sd4LV5XubfR7xgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJWYpVBpGWJHgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRouter;
impl IconShape for MdRouter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.2 5.9l.8-.8C19.6 3.7 17.8 3 16 3s-3.6.7-5 2.1l.8.8C13 4.8 14.5 4.2 16 4.2s3 .6 4.2 1.7zm-.9.8c-.9-.9-2.1-1.4-3.3-1.4s-2.4.5-3.3 1.4l.8.8c.7-.7 1.6-1 2.5-1 .9 0 1.8.3 2.5 1l.8-.8zM19 13h-2V9h-2v4H5c-1.1 0-2 .9-2 2v4c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-4c0-1.1-.9-2-2-2zM8 18H6v-2h2v2zm3.5 0h-2v-2h2v2zm3.5 0h-2v-2h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADxUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zqfBfwM8Nf834B40b008Fdc8dfAdwM/A9zK/16IF913A+/F8/pp4KeBnwF2+d8F8aK7CBznBdsFfhr4aeBn+N8B8aJ5b+C7eNHdCvw08D3AX/M/F+JF89bAZwMvxb/eXwPfDXwPsMv/LIh/nZcG3ht4a+BB/Ov9NPDTwPfwPwPi3+6tgbcG3ho4xr/OLvDTwPcAv81/H8S/33HgrYG3Bt6Kf71bgZ8Gvga4lf9aiP9Yx4H3Bt4beCn+9f4a+G7ge4Bd/vMh/vM8GPho4K2BB/Gv99PAdwM/w38exH+N1wbeG3hr4Bj/OrvATwNfA/w1/7EQ//XeG3hr4K3417sV+G7ge4Bb+fdD/Pc5Drw18NHAS/Gv99fAVwM/A+zyb4P4n+HBwHsD7w08iH+9nwa+G/gZ/nUQ//O8NPDRwFsDx/jX2QW+G/ga4Fb+ZYj/2d4aeG/grfjXex/gu3nhEP87HAfeGnhv4LV40T0EuJUXDPG/z4OBtwY+GngQL9z3AO/NC4Z40T0Y+CzgrYHj/O9wK/AQXjDEi+a9ge/ifyfxgiH+ZQ8Gns7/XuIFQ/zLvht4L/73Ei8Y4l92ETjO/17iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8zzeh3gt3lOrw38Fs9L/Nd4beC3eF7iBUP8y8zzeh3gt3lOrw38Fs9L/Nd4beC3eF7iBUP8y8zzeh3gt3lOrw38Fs9L/Nd4beC3eF7iBUP8y8zzeh3gt3lOrw38Fs9L/Nd4beC3eF7iBUP8y8zzeh3gt3lOrw38Fs9L/Nd4beC3eF7iBUP8y8zz+mtgl+d0HHhpntdv81/jOPDSPC/xgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZX8NvBT/O/0O8Nq8YIh/2XsD38X/Tu8DfDcvGOJF893Ae/G/y/cA780Lh3jRfTbw0cAx/me7BHw18Nn8yxD/eq/N/2y/zYsO8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4Rn3lvQTQdFU4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScanner;
impl IconShape for MdScanner {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.8 10.7L4.2 5l-.7 1.9L17.6 12H5c-1.1 0-2 .9-2 2v4c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-5.5c0-.8-.5-1.6-1.2-1.8zM7 17H5v-2h2v2zm12 0H9v-2h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3htXnS/A7wW/zq/DXw38D38+yH+fY4D7wV8NPBgXnTfA3w2cCtg/m1uBb4a+B5gl38bxL/Ng4GPAt4bOM6L5hLw1cBXA7s8m/n32QW+G/ga4Fb+dRD/Oq8NvBfw3rzongF8NvDdPH/mP853A98D/DYvGsSL5r2A9wZemxfd7wCfDfw2L5z5j/fbwHcD38MLh/iXPR14MC+67wE+G7iVF435z3Mr8BBeMMS/zPzLLgFfDXw1sMu/jvnPJV4wxL/MvGDPAD4b+G7+7cx/LvGCIf5l5nn9DvDZwG/z72f+c4kXDPEvM8/rdYDf5j+G+c8lXjDEv8w8r9cBfpv/GOY/l3jBEP8y87xeB/ht/mOY/1ziBUP8y8zzeh3gt/mPYf5ziRcM8S8zz+t1gN/mP4b5zyVeMMS/zDyv1wF+m/8Y5j+XeMEQ/zLzvF4H+G3+Y5j/XOIFQ/zLzPN6HeC3+Y9h/nOJFwzxLzPP63WA3+Y/hvnPJV4wxL/MPK/XAX6b/xjmP5d4wRD/MvO8Xgf4bf5jmP9c4gVD/MvM83od4Lf5j2H+c4kXDPEvM8/rdYDf5j+G+c8lXjDEv8w8r9cBfpv/GOY/l3jBEP8y87xeB/ht/mOY/1ziBUP8y/4aeCn+84j/GB8NfBXP6XeA1+YFQ/zLfht4Lf7ziP8Y3w28F8/pd4DX5gVD/Ms+G/gs/vOI/xh/Bbw0z+lzgM/mBUP8y94b+C7+84j/GOZ5vQ/w3bxgiH/ZSwN/xX8e8e/31sBP8bxeBvhrXjDEi2YXOMZ/DvHv993Ae/GcLgHHeeEQL5qfBt6K/xzi3+8icJzn9D3Ae/PCIV407w18F/85xL/PWwM/xfN6H+C7eeEQL5rjwEX+c4h/n98CXpvndQLY5YVDvOi+G3gv/uOJf7vXBn6L5/U9wHvzL0O86F4b+C3+44l/u98CXpvn9TrAb/MvQ/zr/DbwWvzHEv82rw38Fs/rd4DX5kWD+Nd5a+Cn+I8l/vWOA38FPJjn9TbAT/OiQfzr/TbwWvzHEf96Xw18FM/rd4DX5kWH+Nd7beC3+I8j/nVeG/gtnr/XAX6bFx3i3+argY/iP4Z40b008FvAcZ7X1wAfzb8O4t/mOPDXwIP49xMvmuPAbwEvzfN6BvDSwC7/Ooh/u9cGfot/P/EvOw78FvDSPH+vA/w2/3qIf5+PBr6Kfx/xwh0Hfgt4aZ6/jwG+mn8bxL/fdwPvxb+deMFeGvgt4DjP3/cA782/HeI/xncD78W/jXj+Pgr4bOA4z9/3AO/Nvw/iP8Zx4KeB1+JfTzyn1wa+CnhpXrDfAV6bfz/Ef6zvBt6Lfx1xxUsDHwW8Ny/c9wDvzX8MxH+8zwY+ixfdewMfDbw0/7LPAT6b/ziI/xxvDXw3cIz/GJeAtwZ+m/9YiP88x4HvBt6Kf5+fAd4b2OU/HuI/32sD3w08iH+dZwDvDfw2/3kQ/3XeG/ho4KV44f4G+Grgu/nPh/iv99LARwOvDTyIK54B/Dbw1cBf818H8d/rpbnir/nvgfj/DfH/G/8IuVnKQUfJSXMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSecurity;
impl IconShape for MdSecurity {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFxUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/db4KeGn+490K/DXwPcAu/zqI/xrfBbw3/7l2gfcBfpoXHeI/33cB781/ndcBfpsXDeI/13cB781/rVuBh/CiQfzn+S7gvfnv8TLAX/MvQ/zn+C7gvfnv8zrAb/MvQ/zH+y7gvfnv9TrAb/MvQ/zH+i7gvfnv9zrAb/MvQ/zH+S7gvfmf4XWA3+ZfhviP8V3Ae/M/x+sAv82/DPHv913Ae/M/y+sAv82/DPHv813Ae/M/z+sAv82/DPFv913Ae/M/0+sAv82/DPFv813Ae/M/1+sAv82/DPGv913Ae/M/2+sAv82/DPGv813Ae/M/3+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3eMH+Bvhontdv8bw+BvhrntNLA1/F83odntdXAy/FC/Y6wG/zL0O86F4b+C1esN8BXpvnZZ7X6wC/zXN6beC3eF7ief028Fq8YK8D/Db/MsSL7rWB3+IF+x3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8UL9jrAb/MvQ7zoXhv4LV6w3wFem+dlntfrAL/Nc3pt4Ld4XuJ5/TbwWrxgrwP8Nv8yxIvutYHf4gX7HeC1eV7meb0O8Ns8p9cGfovnJZ7XbwOvxQv2OsBv8y9DvOheG/gtXrDfAV6b52We10cDf81zemngq3le4nn9NvBavGCvA/w2/zLEi+61gd/iBfsd4LV5XubfRzyv3wZeixfsdYDf5l+GeNG9NvBbvGC/A7w2z8v8+4jn9dvAa/GCvQ7w2/zLEC+61wZ+ixfsd4DX5nmZfx/xvH4beC1esNcBfpt/GeJF99rAb/GC/Q7w2jwv8+8jntdvA6/FC/Y6wG/zL0O86F4b+C1esN8BXpvnZf59xPP6beC1eMFeB/ht/mWIF91rA7/Ff4zXAX6b5/TawG/xH+N1gN/mX4Z40b028Fv8x3gd4Ld5Tq8N/Bb/MV4H+G3+ZYgX3WsDv8V/jNcBfpvn9NrAb/Ef43WA3+ZfhnjRvTbwW/zHeB3gt3lOrw38Fv8xXgf4bf5liBfdawO/xX+M1wF+m+f02sBv8R/jdYDf5l+GeNG9NvBbvGB/A3w0z+u3eF6vA/w2z+m1gd/ieb0Oz+urgZfiBXsd4Lf5lyFedK8N/BYv2O8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC1esNcBfpt/GeJF99rAb/GC/Q7w2jwv87xeB/htntNrA7/F8xLP67eB1+IFex3gt/mXIV50rw38Fi/Y7wCvzfMyz+t1gN/mOb028Fs8L/G8fht4LV6w1wF+m38Z4kX32sBv8YL9DvDaPC/zvF4H+G2e02sDv8XzEs/rt4HX4gV7HeC3+ZchXnSvDfwWL9hfAx/N8/ptntdHA3/Nc3pp4Kt5Xq/N8/pq4KV5wV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie2ngr/jf4WWAv+ZfhvjXuRV4EP+zPQN4MC8axL/OawO/xf9srwP8Ni8axL/eewNfDRzjf5ZLwEcD382LDvFvcxx4b+ClgQfz3+tW4K+B7wZ2+ddB/P+G+P8N8f8b4v83xP9v/CPMMcdBVddHfAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSimCard;
impl IconShape for MdSimCard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.99 4c0-1.1-.89-2-1.99-2h-8L4 8v12c0 1.1.9 2 2 2h12.01c1.1 0 1.99-.9 1.99-2l-.01-16zM9 19H7v-2h2v2zm8 0h-2v-2h2v2zm-8-4H7v-4h2v4zm4 4h-2v-4h2v4zm0-6h-2v-2h2v2zm4 2h-2v-4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSmartphone;
impl IconShape for MdSmartphone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeaker;
impl IconShape for MdSpeaker {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 2H7c-1.1 0-2 .9-2 2v16c0 1.1.9 1.99 2 1.99L17 22c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5 2c1.1 0 2 .9 2 2s-.9 2-2 2c-1.11 0-2-.9-2-2s.89-2 2-2zm0 16c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeakerGroup;
impl IconShape for MdSpeakerGroup {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXsw8CD+b3gGcCvPC/G8jgPfBbw1/7f8NPA+wC7PhnhOx4GnA8f5v2kXeAiwyxWI5/RTwFvzf9tPA2/DFYhnezDwdP5/eAhwK4B4ttcGfov/H14H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfN6wC/zf8srw38Fi+a1wF+G0A822sDv8WL5nWA3+Z/ltcGfosXzesAvw0gnu21gd/iRfM6wG/zP8trA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3+Z/ltYHf4kXzOsBvA4hne23gt3jRvA7w2/zP8trAb/GieR3gtwHEs7028Fu8aF4H+G3+Z3lt4Ld40bwO8NsA4tleG/gtXjSvA/w2/7O8NvBbvGheB/htAPFsrw38Fi+a1wF+m/9ZXhv4LV40rwP8NoB4ttcGfosXzesAv83/LK8N/BYvmtcBfhtAPNtrA7/Fi+Z1gN/mf5bXBn6LF83rAL8NIJ7ttYHf4kXzOsBv8z/LawO/xYvmdYDfBhDP9trAb/GieR3gt/mf5bWB3+JF8zrAbwOIZ3tt4Ld40bwO8Nv8z/LawG/xonkd4LcBxLO9NvBbvGheB/ht/md5beC3eNG8DvDbAOLZXhv4LV40rwP8Nv+zvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpv/WV4b+C1eNK8D/DaAeLbXBn6LF83rAL/N/yyvDfwWL5rXAX4bQDzbawO/xYvmdYDf5n+W1wZ+ixfN6wC/DSCe7bWB3+JF8zrAb/M/y2sDv8WL5nWA3wYQz/bawG/xonkd4Lf5n+W1gd/iRfM6wG8DiGd7beC3eNG8DvDb/M/y2sBv8aJ5HeC3AcSzvTbwW7xoXgf4bf5neW3gt3jRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfN6wC/zf8srw38Fi+a1wF+G0A822sDv8WL5nWA3+Z/ltcGfosXzesAvw0gnu21gd/iRfM6wG/zP8trA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3+Z/ltYHf4kXzOsBvA4hne23gt3jRvA7w2/zP8trAb/GieR3gtwHEs7028Fu8aF4H+G3+Z3lt4Ld40bwO8NsA4tleG/gtXjSvA/w2/7O8NvBbvGheB/htAPFsrw38Fv8/vA7w2wDi2R4MPJ3/H04AuwDiOf008Fb83/Y9wHtzBeI5HQduBY7xf9PfAK8N7HIF4nkdB74beCv+b/ke4KOBXZ4N8YI9GHgw/zfcCtzK80L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRiaZxBE//jIQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTablet;
impl IconShape for MdTablet {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 4H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h18c1.1 0 1.99-.9 1.99-2L23 6c0-1.1-.9-2-2-2zm-2 14H5V6h14v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFF0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXezDwUcBrAy/N/wx/Dfw28DXArbzoEP86XwV8NP+zfTXwMbxoEC+6vwJemv8d/hp4Gf5liBfNVwMfxf8uXwN8NC8c4l/2YODp/O/0EOBWXjDEv+yrgY/if6evAT6aFwzxL/sr4KX53+mvgZfhBUP8y8z/buIFQ/zLzHP6HeC1+Z/pt4HX4jmJFwzxLzPP6XeA1+Z/pt8GXovnJF4wxL/MPKffAV6b/5l+G3gtnpN4wRD/MvOcfgd4bf5n+m3gtXhO4gVD/MvMc/od4LX5n+m3gdfiOYkXDPEvM8/pd4DX5n+m3wZei+ckXjDEv8w8p98BXpv/mX4beC2ek3jBEP8y85x+B3ht/mf6beC1eE7iBUP8y8xz+h3gtfmf6beB1+I5iRcM8S8zz+l3gNfmf6bfBl6L5yReMMS/zDyn3wFem/+Zfht4LZ6TeMEQ/zLznH4HeG3+Z/pt4LV4TuIFQ/zLzHP6HeC1+Z/pt4HX4jmJFwzxLzPP6XeA1+Z/pt8GXovnJF4wxL/MPKffAV6b/5l+G3gtnpN4wRD/MvOcfgd4bf5n+m3gtXhO4gVD/MvMc/od4LX5n+m3gdfiOYkXDPEvM8/pd4DX5n+m3wZei+ckXjDEv8w8p98BXpv/mX4beC2ek3jBEP8y85x+B3ht/mf6beC1eE7iBUP8y8xz+h3gtfmf6beB1+I5iRcM8S8zz+l3gNfmf6bfBl6L5yReMMS/zDyn3wFem/+Zfht4LZ6TeMEQ/zLznH4HeG3+Z/pt4LV4TuIFQ/zLzHP6HeC1+Z/pt4HX4jmJFwzxLzPP6XeA1+Z/pt8GXovnJF4wxL/MPKffAV6b/5l+G3gtnpN4wRD/MvOcfgd4bf5n+m3gtXhO4gVD/MvMc/od4LX5n+m3gdfiOYkXDPEvM8/pd4DX5n+m3wZei+ckXjDEv8w8p98BXpv/mX4beC2ek3jBEP8y85x+B3ht/mf6beC1eE7iBUP8y8xz+h3gtfmf6beB1+I5iRcM8S8zz+l3gNfmf6bfBl6L5yReMMS/zDyn3wFem/+Zfht4LZ6TeMEQ/zLznH4HeG3+Z/pt4LV4TuIFQ/zLzHP6HeC1+Z/pt4HX4jmJFwzxLzPP6XeA1+Z/pt8GXovnJF4wxL/MPKffAV6b/5l+G3gtnpN4wRD/MvOcfgd4bf5n+m3gtXhO4gVD/MvMc/od4LX5n+m3gdfiOYkXDPEvM8/pd4DX5n+m3wZei+ckXjDEv8w8p78GPpr/mb4aeGmek3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvMf43eA1+ZF89vAa/EfQ7xgiH/ZXwMvxb/f7wCvzYvmt4HX4t/vb4CX5gVD/Mu+Gvgo/v3+GvhoXjRfDbw0/35fA3w0LxjiX/Zg4On87/QQ4FZeMMSL5quBj+J/l68BPpoXDvGi+2vgpfjf4W+Al+ZfhvjX+Wrgo/if7WuAj+ZFg/jXezDw0cBrAy/F/wx/A/w28NXArbzo+Edpq65BX64wFAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTabletAndroid;
impl IconShape for MdTabletAndroid {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,0H6C4.34,0,3,1.34,3,3v18c0,1.66,1.34,3,3,3h12c1.66,0,3-1.34,3-3V3C21,1.34,19.66,0,18,0z M14,22h-4v-1h4V22z M19.25,19H4.75V3h14.5V19z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTabletMac;
impl IconShape for MdTabletMac {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {

        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z2txxe/w3wfxX+elgfcC3hp4MM8m/vsg/vM9GPgu4LV5/sR/H8R/rq8CPpoXTvz3QfznOA58F/DW/MvEfx/Ef47fAl6bF43474P4j/ddwHvzohP/fRD/sd4a+Cn+dcR/H8R/rKcDD+ZfR/z3QfzHeW/gu/jXE/99EP9x/gp4af71xH8fxH+MBwNP599G/PdB/Md4b+C7+LcR/30Q/zE+G/gs/m3Efx/Ef4zPBj6Lfxvx3wfxH+Ongbfi30a86N4KeGmueGngr7niVuBngF3+dRD/MT4b+Cz+bcQL99rAewFvDRznhftr4KuB7+FFg/iP8dnAZ/FvI56/BwNfBbw1/3q3Ah8D/DQvHOI/xlsDP8W/jXhebw18F3Ccf5/vBt6HFwzxH2cXOMa/nnhO7w18F/9xfht4G2CX54X4j/PdwHvxryee7b2B7+I/3ncD78PzQvzHeW3gt/jXE1e8NPBX/Of5HOCzeU6I/1g/DbwV/zriiqcDD+Y/10OAW3k2xH+sBwN/DRzjRSfgvYHv4j/fbwOvw7Mh/uO9NfBTvOgEPB14MP81Xgb4a65A/Od4b+C7eNG8DPBX/Nf5GuCjuQLxn+e9ga8GjvHCfQ3wUfzXuRV4CFcg/nO9NPDdwEvxgv0O8Fr81xJXIP5rvDfw2cCDeF5/Dbw0/7VeB/htAPFf66WB9wZeGjgOvBT/PV4H+G0A8WyvDfwW/z3+GtgF/hr4HuCv+a+BeLbXBn6L/1p/A7wUz+tW4LOB7+E/F+I5mf9avwO8Fi/YXwPvA/w1/zkQz+lW4EH81/ka4KN44XaBjwG+m/94iOf01cBH8V/nZYC/4kXzPsB38x8L8ZxeGvgr/ms8A3gwYF50bwP8NP9xEM/rt4HX4j/f+wDfDZgX3S7wMsCt/MdAPK8HA0/nP9ffAC/NFeZf56eBt+E/BuL5+2zgs/jPcQl4beCvucL8670O8Nv8+yFesJ8G3or/eO8DfDfPZv71vgd4b/79EC/YceCngdfiP877AN/NczL/ervACf79EP+y7wbei3+fS8B7Az/N8zL/Nq8D/Db/PogXzVsDXw08iH+9nwE+GriV58/823wO8Nn8+yD+dd4b+GjgpXjhLgE/DXw38Nu8cObf5nOAz+bfB/Fvcxx4a+DBXPHSwF9zxW8Dv82Lzvzb/Azw1vz7IP77mX+bzwE+m38fxH8/82/zOcBn8++D+O9n/m3eB/hu/n0Q//3Mv80JYJd/H8R/P/Ov9zvAa/Pvh/jvZ/713gf4bv79EP/9zL/OM4AH8x8D8d/P/Ou8DvDb/MdA/PczL7rvAd6b/ziI/37mRfM7wGvzHwvx38/8y74H+Ghgl/9YiP9+5oX7HOCz+c+B+O9nnr/fAd4buJX/PIj/fubZngH8NPDdwF/znw/x3++1ueK3+a+H+P8N8f8b/whwuaNBl9ExUgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdToys;
impl IconShape for MdToys {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 12c0-3 2.5-5.5 5.5-5.5S23 9 23 12H12zm0 0c0 3-2.5 5.5-5.5 5.5S1 15 1 12h11zm0 0c-3 0-5.5-2.5-5.5-5.5S9 1 12 1v11zm0 0c3 0 5.5 2.5 5.5 5.5S15 23 12 23V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovn9dv87/baPK/XAX4bQDzbSwN/xf8PLwP8NYB4TrcCD+L/tmcAD+YKxHN6beC3+L/tdYDf5grE83pv4KuBY/zfcgl4b+CneTbE83cceG/gpYEH86/30sAx/mNdAv6af71bgb8GvhvY5Tkh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cvw28Fv+xfgd4bf5jIf5z/DbwWvzH+h3gtfmPhfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AgiPh0E3WHlAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTv;
impl IconShape for MdTv {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAESElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP7eC/ho4KX5v+Gvga8GvofnhHhe3wW8N/83fTfwPjwb4jl9NvBZ/N/2OcBncwXiOV0EjvN/2y5wgisQz/bawG/x/8PrAL8NIJ7ttYHf4l/vdXheXw28FP9zvQ7w2wDi2V4b+C3+9cTz+m3gtfif63WA3wYQz/bawG/xryee128Dr8X/XK8D/DaAeLbXBn6Lfz3xvH4beC3+6zwD+Gjgr7nipYGvBh7E8/c6wG8DiGd7beC3+NcTz+u3gdfiv8YzgJcGdnlOx4G/Bh7E83od4LcBxLO9NvBb/OuJ5/XbwGvxX+NtgJ/m+Xtr4Kd4Xq8D/DaAeLbXBn6Lfz3xvH4beC3+a5wAdnn+Hgw8nef1OsBvA4hne23gt/jXE8/rt4HX4r/GCWCX5+/BwNN5Xq8D/DaAeLbXBn6L5/U6vHC/zfN6aeA4L9hXAy/Ff4y3AX6a5++tgZ/ieb0O8NsA4tleG/gtnpf4j/fbwGvxH+NW4GWAXZ7TceCvgAfzvF4H+G0A8WyvDfwWz0v8x/tt4LX4j3Mr8NHA33DFSwFfDTyY5+91gN8GEM/22sBv8bzEf7zfBl6Lf9nnAD8N/DXw2sBnA6/Fv9/rAL8NIJ7ttYHf4nmJ/3i/DbwWL9zrAL/N8/pu4L3493kd4LcBxLO9NvBbPK/X5oX7HZ7XSwPHeMG+GnhpXrDvAd6b5+84cJF/n9cBfhtAPNtrA7/Fv554Xr8NvBb/dh8DfDUv2G8Dr8W/3esAvw0gnu21gd/iX088r98GXot/u48BvpoX7LeB1+Lf7nWA3wYQz/bawG/xryee128Dr8W/3fcA783zdxy4yL/P6wC/DSCe7bWB3+JfTzyv3wZei3+f1wF+m+f1XcB78+/zOsBvA4hne23gt/jXE8/rt4HX4t/vs4GfAf4aeC3gs4HX5t/vdYDfBhDP9trAb/GvJ57XbwOvxf9crwP8NoB4ttcGfot/PfG8fht4Lf7neh3gtwHEs7028Fv86702z+urgZfmf67XAX4bQDzbawO/xf8PrwP8NoB4TrvAMf5vuwQc5wrEc/ps4LP4v+1zgM/mCsTz+m7gvfi/6XuA9+bZEM/fewMfDbwU/zf8DvDdwHfznBD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ed4X4BBcpAZFwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideogameAsset;
impl IconShape for MdVideogameAsset {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 6H3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-10 7H8v3H6v-3H3v-2h3V8h2v3h3v2zm4.5 2c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4-3c-.83 0-1.5-.67-1.5-1.5S18.67 9 19.5 9s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWatch;
impl IconShape for MdWatch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 12c0-2.54-1.19-4.81-3.04-6.27L16 0H8l-.95 5.73C5.19 7.19 4 9.45 4 12s1.19 4.81 3.05 6.27L8 24h8l.96-5.73C18.81 16.81 20 14.54 20 12zM6 12c0-3.31 2.69-6 6-6s6 2.69 6 6-2.69 6-6 6-6-2.69-6-6z",
            }
        }
    }
}
