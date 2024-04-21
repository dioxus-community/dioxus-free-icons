use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fd4LeGmu+Gvge/ivh/ivdxz4KeC1eU6/DbwNsMt/HcR/vZ8C3prn72uAj+a/DuK/nnnhxH8dxH+t1wZ+ixfudYDf5r8G4r/WawO/xQv3OsBv818D8V/nwcBHAR/NC/fVwNcAt/KfD/Gf772AjwZemn+dvwa+Gvge/vMg/vO8F/DZwIP597kV+Bjgp/mPh/iPdxz4KuC9+Y/13cDHALv8x0H8x3pp4LuAl+Y/x18D7wP8Nf8xEP9xXhr4LeA4/7l2gdcB/pp/P8R/jOPAXwEP5r/GXwOvA+zy74P4j/FbwGvzX+u3gdfh3wfx7/fRwFfx3+NjgK/m3w7x7/Ng4K+A4/z32AVeBriVfxvEv89nA5/Fv97HAH/Nc3pp4Kv41/sc4LP5t0H82x0Hng4c51/vdYDf5jm9NvBb/OvtAg8BdvnXQ/zbvTfwXfzbvA7w2zyn1wZ+i3+b9wG+m389xL/ddwPvxb/N6wC/zXN6beC3+Lf5HuC9+ddD/Ns9HXgw/zavA/w2z+m1gd/i3+ZW4CH86yH+bY4DF/m3ex3gt3lOrw38Fv92J4Bd/nUQ/zavDfwW/3avA/w2z+m1gd/i3+51gN/mXwfxb/PawG/xb/c6wG/znF4b+C3+7V4H+G3+dRD/Nq8N/BYv2McAf80L9tfALs/pOPDSvGAvDXwVL9jrAL/Nvw7i3+a1gd/iBXsd4Lf5j/XawG/xgr0O8Nv86yD+bV4b+C1esNcBfpv/WK8N/BYv2OsAv82/DuLf5rWB3+IFex3gt/mP9drAb/GCvQ7w2/zrIP5tXhv4LV6w1wF+m/9Yrw38Fi/Y6wC/zb8O4t/mtYHf4gX7aOCvecH+BtjlOR0HXooX7KWBr+YFex3gt/nXQfzbvDbwW/zbvQ7w2zyn1wZ+i3+71wF+m38dxL/NawO/xb/d6wC/zXN6beC3+Ld7HeC3+ddB/Nu8NvBb/Nu9DvDbPKfXBn6Lf7vXAX6bfx3Ev81rA7/Fv93rAL/Nc3pt4Lf4t3sd4Lf510H827w28Fv8270O8Ns8p9cGfot/u9cBfpt/HcS/zWsDv8W/3esAv81zem3gt/i3ex3gt/nXQfzbvDbwW/zbvQ7w2zyn1wZ+i3+71wF+m38dxL/NawO/xb/dRwN/zXN6aeCr+bd7HeC3+ddB/Nu8NvBb/M/yOsBv86+D+Ld5beC3+J/ldYDf5l8H8a93HPgt4KX5n+WvgdcBdnnRIf51jgO/Bbw0/zP9NfA6wC4vGsSL7jjwW8BL8z/bXwOvA+zyL0O8aI4DvwW8NP87/DXwOsAuLxziX3Yc+C3gpfnf5a+B1wF2ecEQ/7K/Al6a/53+GngZXjDEv8z87yZeMMS/zPzvJl4wxL/stfnf7bd5wRD/vyH+f0P813lt4Ld40bwO8Nv850P81zkOXORFcwLY5T8f4r/WTwNvxQv3PcB7818D8V/rOHArcIzn7xnASwO7/NdA/Nc7Dnw38FY8p58B3hvY5b8O4r/Pg4EHc8WtwK3810P8/4b4/41/BDWUq0F0KKn2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddAlert;
impl IconShape for MdAddAlert {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.01 21.01c0 1.1.89 1.99 1.99 1.99s1.99-.89 1.99-1.99h-3.98zm8.87-4.19V11c0-3.25-2.25-5.97-5.29-6.69v-.72C13.59 2.71 12.88 2 12 2s-1.59.71-1.59 1.59v.72C7.37 5.03 5.12 7.75 5.12 11v5.82L3 18.94V20h18v-1.06l-2.12-2.12zM16 13.01h-3v3h-2v-3H8V11h3V8h2v3h3v2.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAutoDelete;
impl IconShape for MdAutoDelete {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/zh/A+zynI4DL8V/nF3gIcAuLxjiX/bZwGfxH+t1gN/mOb028Fv8x/oc4LN5wRAv3HHg6cBx/mO9DvDbPKfXBn6L/1i7wEOAXZ4/xAv32cBn8R/vdYDf5jm9NvBb/Mf7HOCzef4QL9zTgQfzH+91gN/mOb028Fv8x7sVeAjPH+IFe2vgp/jP8TrAb/OcXhv4Lf5zvA7w2zwvxAv21cBH8Z/jdYDf5jm9NvBb/Of4GuCjeV6IF+yvgJfmP8frAL/Nc3pt4Lf4z/HXwMvwvBDP34OBp/Of53WA3+Y5vTbwW/znOQHs8pwQz99rA7/Ff57XAX6b5/TawG/xn+d1gN/mOSGev48Gvor/PK8D/DbP6bWB3+I/z8cAX81zQjx/nw18Fv95Xgf4bZ7TawO/xX+ezwE+m+eEeP4+G/gs/vO8DvDbPKfXBn6L/zyfA3w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4GeGueE+L5+23gtfjP8zrAb/OcXhv4Lf7z/Azw1jwnxPP32cBn8Z/ndYDf5jm9NvBb/Of5HOCzeU6I5++zgc/iP89HA3/Nc3pp4Kv5z/M5wGfznBDP30cDX8X/LR8DfDXPCfH8vTbwW/zf8jrAb/OcEM/fg4Gn83/LCWCX54R4wf4aeCn+b/gb4KV5XogX7KuBj+L/hq8BPprnhXjBXhv4Lf5zvA7w2zyn1wZ+i/8crwP8Ns8L8cLdCjyI/3ivA/w2z+m1gd/iP94zgAfz/CFeuM8GPov/eK8D/DbP6bWB3+I/3ucAn83zh3jhjgO3Asf4j/U6wG/znF4b+C3+Y10CHgzs8vwh/mWfDXwW/7FeB/htntNrA7/Ff6zPAT6bFwzxLzsO3Aoc4z/OXwO7PKfjwEvzH+cS8GBglxcM8aJ5b+C7+N/lbYCf5oVDvOh+Gngr/nf4GeCt+ZchXnTHgb8GHsT/bM8AXhrY5V+G+Nd5aeC3gWP8z3QJeG3gr3nRIP71Xhr4beAY/7NcAl4b+GtedIh/m5cGfhs4xv8Ml4DXBv6afx3Ev91LAz8NPIj/Xs8A3hr4a/71EP8+x4HvBt6K/x4/A7w3sMu/DeI/xnsDXw0c47/GJeC9gZ/m3wfxH+c48NHARwPH+M9xCfhq4KuBXf79EP/xjgMfDbw38CD+YzwD+G7gq4Fd/uMg/nO9NvDWwGsDL8W/zt8Avw38NPDb/OdA/Nc5Drw08NLAca54aa74a67YBf4a+Gtgl/98iP/fEP+/If5/Q/z/hvj/jX8EV8q+QedAsU4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdError;
impl IconShape for MdError {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdErrorOutline;
impl IconShape for MdErrorOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 15h2v2h-2zm0-8h2v6h-2zm.99-5C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAESklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z3eG/gq4DhX7AIfA3w3/7UQ//VeG/gtnr/XAX6b/zqI/3q/DbwWz9/vAK/Nfx3Efz3zwon/Ooj/euaFE/91EP91jgNvBXw3L9x7Az8D7PKfD/Gf76WBzwLemn+dnwY+B/hr/vMg/vMcB74KeG/+fb4b+Bhgl/94iP8cLw38FPBg/mP8NfA+wF/zHwvxH++lgd8CjvMfaxd4HeCv+Y+D+I/1YOCvgOP859gFHgLs8h8D8R/rr4CX5j/XXwMvw38MxH+cjwa+iv8aHwN8Nf9+iP8Yx4GnA8f5r7ELPATY5d8H8R/jo4Gv4kX3McBf85xeGvgqXnQfA3w1/z6I/xhPBx7Mi+51gN/mOb028Fu86P4aeBn+fRD/fi8N/BX/Oq8D/DbP6bWB3+Jf52WAv+bfDvHv99HAV/Gv8zrAb/OcXhv4Lf51Pgb4av7tEP9+3w28F/86rwP8Ns/ptYHf4l/ne4D35t8O8e/328Br8a/zOsBv85xeG/gt/nV+B3ht/u0Q/35PBx7Mv87rAL/Nc3pt4Lf417kVeAj/doh/P/Ov9zrAb/OcXhv4Lf71xL8d4t/P/Ou9DvDbPKfXBn6Lfz3xb4f49zP/eq8D/DbP6bWB3+JfT/zbIf79zL/e6wC/zXN6beC3+NcT/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cy/3scAf81zemngq/jXE/92iH8/86/3OsBv85xeG/gt/vXEvx3i38/8670O8Ns8p9cGfot/PfFvh/j3M/96rwP8Ns/ptYHf4l9P/Nsh/v3Mv97rAL/Nc3pt4Lf41xP/doh/n+PARf71Phr4a57TSwNfzb/eCWCXfxvEv91x4LeAl+a/118DrwPs8q+H+Lc5DvwW8NL8z/DXwOsAu/zrIP71jgO/Bbw0/7P8NfA6wC4vOsS/znHgt4CX5n+mvwZeB9jlRYN40R0Hfgt4af5n+2vgdYBd/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffSwFfzv8NHA3/Nvwzx/xvi/zfEf52XBr6KF83HAH/Nfz7Ef52XBv6KF83LAH/Nfz7Ef61bgQfxwj0DeDD/NRD/tV4b+C1euNcBfpv/Goj/eu8NfDVwjOd0CXhv4Kf5r4P473EceG/gpbnir4HvBnb5r4X4/w3x/xvi/zfE/2+I/9/4RwoShkGclpeuAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotificationImportant;
impl IconShape for MdNotificationImportant {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 16v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2zm-5 0h-2v-2h2v2zm0-4h-2V8h2v4zm-1 10c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR7MFbfy3wPx3+unuOJt+O+B+O/z2sBvccXrAL/Nfz3Ef5+/Al6aK/4aeBn+6yH+e3w08FU8p48Bvpr/Woj/eseBpwPHeU67wEOAXf7rIP7rfTXwUTx/XwN8NP91EP+1Hgw8nRfuIcCt/NdA/Nf6LeC1eeF+G3gd/msg/uu8NfBTvGjeBvhp/vMh/us8HXgwL5pbgYfwnw/xX+Ozgc/iX+dzgM/mPxfiP99x4OnAcf51doGHALv850H85/tu4L34t/ke4L35z4P4z/XawG/x7/M6wG/znwPxn+u3gNfm3+e3gdfhPwfiP897A9/Ff4z3Ab6b/3iI/xzHgb8CHsx/jFuBlwF2+Y+F+M/x2cBn8R/rc4DP5j8W4j/eg4Gn85/jIcCt/MdB/Mf7KeCt+c/x08Db8B8H8R/rtYHf4l/2N8Auz+k48FL8y14H+G3+YyD+Y/0V8NL8y14H+G2e02sDv8W/7K+Bl+E/BuI/zkcDX8WL5nWA3+Y5vTbwW7xoPgb4av79EP8xjgNPB47zonkd4Ld5Tq8N/BYvml3gIcAu/z6I/xhfDXwUL7rXAX6b5/TawG/xovsa4KP590H8+z0YeDr/Oq8D/DbP6bWB3+Jf5yHArfzbIf79fgt4bf51Xgf4bZ7TawO/xb/ObwOvw78d4t/nrYGf4l/vdYDf5jm9NvBb/Ou9DfDT/Nsg/n2eDjyYf73XAX6b5/TawG/xr3cr8BD+bRD/dp8NfBb/Nq8D/DbP6bWB3+Lf5nOAz+ZfD/Fvcxx4OnCcf5vXAX6b5/TawG/xb7MLPATY5V8H8W/z3cB78W/318Auz+k48NL8230P8N786yD+9V4b+C3+Z3od4Ld50SH+9X4LeG3+Z/pt4HV40SH+dd4b+C7+Z3sf4Lt50SBedMeBvwIezP9stwIvA+zyL0O86D4b+Cz+Y/wNsMtzOg68FP8xPgf4bP5liBfNg4Gn8x/ndYDf5jm9NvBb/Md5CHArLxziRfNTwFvzH+d1gN/mOb028Fv8x/lp4G144RD/stcGfov/WK8D/DbP6bWB3+I/1usAv80LhviX/RXw0vzHeh3gt3lOrw38Fv+x/hp4GV4wxAv30cBX8R/vdYDf5jm9NvBb/Mf7GOCref4QL9hx4OnAcf532wUeAuzyvBAv2FcDH8X/DV8DfDTPC/H8PRh4Ov+3PAS4leeEeP5+C3ht/m/5beB1eE6I5/XWwE/xf9PbAD/NsyGe19OBB/N/063AQ3g2xHP6bOCz+L/tc4DP5grEsx0Hng4c5/+2XeAhwC6A+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8COL6QQdQeduIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWarning;
impl IconShape for MdWarning {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
