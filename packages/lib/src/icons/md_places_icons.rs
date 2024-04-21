use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAcUnit;
impl IconShape for MdAcUnit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 11h-4.17l3.24-3.24-1.41-1.42L15 11h-2V9l4.66-4.66-1.42-1.41L13 6.17V2h-2v4.17L7.76 2.93 6.34 4.34 11 9v2H9L4.34 6.34 2.93 7.76 6.17 11H2v2h4.17l-3.24 3.24 1.41 1.42L9 13h2v2l-4.66 4.66 1.42 1.41L11 17.83V22h2v-4.17l3.24 3.24 1.42-1.41L13 15v-2h2l4.66 4.66 1.41-1.42L17.83 13H22z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz99x4L2AlwYezP88fw18DP9+iOf13sBXAcf5n+27gffh3wfxnN4a+Cn+9/hu4H34t0M8p6cDD+Z/l+8G3od/G8SzvTTwV/zv9N3A+/Cvh3i21wZ+i+f1O7xojgMvxfP6HV40x4GX4nn9Ds/2Wrxg3w28D/86iGd7beC3eF7iRfPawG/xvMSL5rWB3+J5iWd7b+C7eMG+G3gfXnSIZ3tt4Ld4XuJF89rAb/G8xIvmtYHf4nmJ5/TewHfxgn038D68aBDP9trAb/G8xIvmtYHf4nmJF81rA7/F8xLP672B7+IF+27gffiXIZ7ttYHf4nmJF81rA7/F8xIvmtcGfovnJZ6/9wa+ixfsu4H34YVDPNtrA7/F8xIvmtcGfovnJV40rw38Fs9LvGDvDXwXL9h3A+/DC4Z4ttcGfovnJV40rw38Fs9LvGheG/gtnpd44d4b+C5esO8G3ofnD/Fsrw38Fs9LvGheG/gtnpd40bw28Fs8L/Eve2/gu3jBvht4H54X4tleG/gtnpd40bw28Fs8L/GieW3gt3he4kXz3sB38YJ9N/A+PCfEs7028Fs8L/GieW3gt3he4kXz2sBv8bzEi+69ge/iBftu4H14NsSzvTbwWzwv8aJ5beC3eF7iRfPawG/xvMS/znsD38UL9t3A+3AF4tleG/gtntdv86I5Drw0z+u3edEcB16a5/Xb/Ou9NHCcF+x1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi21wZ+i/8fXgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8a/zO1zxWvzXegZwK3AceCn+9V4H+G0A8WyvDfwWL5rvAT4a2OWK48BXA+/Ff65LwGcDX82zvTTw3cBL8aJ7HeC3AcSzvTbwW/zLfgd4bZ6/3wZei/88HwN8Nc/rwcBfA8d40bwO8NsA4tleG/gt/mWvA/w2z99rA7/Ff45nAA/mBftq4KN40bwO8NsA4tleGvhqntdr8ZzEC2ee0+/wb3MceCme7XeA1+YF+2jgq3i2S8Bf8/x9NPDXAOJfZp7TywB/zfP30sBf8WyXgOP827w28Fs8218DL8ML9tnAZ/FsnwN8Ni8c4l/228Br8WyfA3w2z99nA5/Fs/0O8Nr82zwYeDrP6WWAv+b5ezrwYJ7tY4Cv5oVD/Ms+GvgqntPrAL/Nc3pt4Ld4Th8DfDX/dn8NvBTP9tfA2wC38py+C3hvntNDgFt54RD/suPArcAxntN3A3/NFS8NvDfP6RLwYGCXf7u3Bn6K57QLfDdwK3AceG/gwTyn7wHem38Z4kXz3sB38a/zPsB38+/33cB78aK7BDwY2OVfhnjRvTfwXbxo3gf4bv5jHAe+G3gr/mWXgNcG/poXDeJf57WBrwZeiufvb4CPBn6b/3ifDXw0cIzn73uAjwZ2edEh/m1eGnhr4MFccSvw08Bf85/rOPDawEsDLw38NbAL/DRwK/96iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BFUOsUGz8G0eAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirportShuttle;
impl IconShape for MdAirportShuttle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 5H3c-1.1 0-2 .89-2 2v9h2c0 1.65 1.34 3 3 3s3-1.35 3-3h5.5c0 1.65 1.34 3 3 3s3-1.35 3-3H23v-5l-6-6zM3 11V7h4v4H3zm3 6.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm7-6.5H9V7h4v4zm4.5 6.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM15 11V7h1l4 4h-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAllInclusive;
impl IconShape for MdAllInclusive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.6 6.62c-1.44 0-2.8.56-3.77 1.53L12 10.66 10.48 12h.01L7.8 14.39c-.64.64-1.49.99-2.4.99-1.87 0-3.39-1.51-3.39-3.38S3.53 8.62 5.4 8.62c.91 0 1.76.35 2.44 1.03l1.13 1 1.51-1.34L9.22 8.2C8.2 7.18 6.84 6.62 5.4 6.62 2.42 6.62 0 9.04 0 12s2.42 5.38 5.4 5.38c1.44 0 2.8-.56 3.77-1.53l2.83-2.5.01.01L13.52 12h-.01l2.69-2.39c.64-.64 1.49-.99 2.4-.99 1.87 0 3.39 1.51 3.39 3.38s-1.52 3.38-3.39 3.38c-.9 0-1.76-.35-2.44-1.03l-1.14-1.01-1.51 1.34 1.27 1.12c1.02 1.01 2.37 1.57 3.82 1.57 2.98 0 5.4-2.41 5.4-5.38s-2.42-5.37-5.4-5.37z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdApartment;
impl IconShape for MdApartment {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,11V3H7v4H3v14h8v-4h2v4h8V11H17z M7,19H5v-2h2V19z M7,15H5v-2h2V15z M7,11H5V9h2V11z M11,15H9v-2h2V15z M11,11H9V9h2 V11z M11,7H9V5h2V7z M15,15h-2v-2h2V15z M15,11h-2V9h2V11z M15,7h-2V5h2V7z M19,19h-2v-2h2V19z M19,15h-2v-2h2V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/3mtxxSXgr/mfDfEf57OAjwaO82y3Au8D/Db/MyH+Y3wX8N68YO8DfDf/8yD+/d4b+C5euF3gIcAu/7Mg/v3+Cnhp/mUfA3w1/7Mg/v3Mi+Z7gPfmfxbEv5950XwN8NH8z4L49/tr4KX4l30M8NX8z4L493tv4Lt44S4BDwZ2+Z8F8R/ju4H34gV7G+Cn+Z8H8R/ns4GPBo7xbM8A3hv4bf5nQvzHe22u2AX+mv/ZEP+/If5/Q/zbPRj4LOC1gQcDtwLfDXwNsMv/Doh/m/cGvosX7LuBrwH+mv/ZEP96Lw38FS+a3wa+GvgZ/mdC/Ov9NPBW/OvcCnw28DPALv9zIP51jgMX+bfbBb4a+B7gVv77If51Phr4Kv5j/DYv2McAf81/PsS/zl8BL81/vtcBfpv/fIgX3YOBp/Oi+x7gvfi3eR3gt/nPh3jRfTTwVbxo/gZ4aeDBwEcD7w0c40X3OsBv858P8aL7K+CledF8DPDVPNtx4K2BzwYexL/sdYDf5j8f4kXzYODpvOgeAtzK8/fWwHsDx3nBPhr4a/7zIV40Hw18FS+avwFemv8dEC+avwJemhfNxwBfzf8OiH/Zg4Gn86J7CHArz/ZbvHB/DXwP8Nc8f7/F8/oY4K/590P8yz4a+CpeNH8DvDTPybxoXgb4a56XeV6vA/w2/36If9lfAS/Ni+ZjgK/mOZkXzdcAH83zMs/rdYDf5t8P8S8zL7qHALfynMyL5neA1+Z5mef1OsBv8++H+JeZF83fAC/N8zIvmt8BXpvnZZ7X6wC/zb8f4l9mXjS/A7w2z+u3eV4vDRzjOf018NE8r9/meb0O8Ns8r9fiit/hRYP4l5kXze8Ar82L5reB1+Lf7nWA3+bZ3hv4KuA4V+wCHwN8Ny8c4l9mXjS/A7w2L5rfBl6Lf7vXAX6bK14b+C2ev9cBfpsXDPEvMy+a3wFemxfNTwNvxb/d6wC/zRW/DbwWz9/vAK/NC4b4l5kXze8Ar82L5rOBz+Lf7mWAv+YK88KJFwzxLzMvmt8BXpsXzUsDf8W/zTOAB/Nsfw28FM/f3wAvzQuG+JeZF83vAK/Ni+6rgY/iX+91gN/m2T4b+Cyev88BPpsXDPEvMy+a3wFem3+drwY+ihfNJeC9gZ/mef028Fo8p98BXpsXDvEvMy+a3wFem3+9lwbeGnhp4DjP61bgr4HvBnZ5wd4beGmu+Gvgu/mXIf5l5kXzO8Br878L4l9mXjS/A7w2/7sg/mXmRfM7wGvzvH6L/16vwwuG+JeZF83vAK/N8zL/vcQLhviXmRfN7wCvzfMy/73EC4b4l5kXze8Ar83zMv+9xAuG+JeZF83vAK/N8zL/vcQLhviXmRfN7wCvzfP6bf57vTYvGOJfZl40vwO8Nv+7IP5l5kXzO8Br878L4l9mXjS/A7w2/7sg/mXmRfM7wGvzvwviX2ZeNL8DvDb/uyD+ZeZF8zvAa/O/C+JfZl40vwO8Nv+7IP5lv82L5q+Bj+Z/F8T/b4j/3xD/vyH+f0P8/8Y/AnNcxUF2fZX2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBabyChangingStation;
impl IconShape for MdBabyChangingStation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,8v2h-3L8.31,8.82L7,12.75V22H3V12l1.58-4.63C4.96,6.25,6.22,5.69,7.3,6.18l4.15,1.83L14,8z M8,1C6.9,1,6,1.9,6,3 s0.9,2,2,2s2-0.9,2-2S9.1,1,8,1z M9,19h12v-2H9V19z M19.5,16c0.83,0,1.5-0.67,1.5-1.5c0-0.83-0.67-1.5-1.5-1.5S18,13.67,18,14.5 C18,15.33,18.67,16,19.5,16z M13,12c0-0.55-0.45-1-1-1H9v2h2v1c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2v-3h-2v2h-2V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/cV4a+CpesL8BPpoXzVcDL8UL9jHAX/Pvh/iP89rAb/GC/Q7w2rxofht4LV6w1wF+m38/xH+c1wZ+ixfsd4DX5kXz28Br8YK9DvDb/Psh/uO8NvBbvGC/A7w2L5rfBl6LF+x1gN/m3w/xH+e1gd/iBfsd4LV50fw28Fq8YK8D/Db/foj/OK8N/BYv2F8DH82L5quBl+YFex3gt/n3Q/z7PRj4KOCtgQfzX+NW4KeBrwFu5d8O8e/zVcBH89/rs4HP4d8G8W9zHPgt4KX5n+GvgdcBdvnXQfzrHQd+C3hp/mf5a+Bl+NdB/Ot9NfBR/M/0OcBn86JD/Os8GHg6/7M9BLiVFw3iX+e7gffif7bvAd6bFw3iX+cicJz/2XaBE7xoEC+6lwb+iv8dXgb4a/5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvH6b/xp/A3w0z8s8r9cBfpt/GeJF99rAb/Hf53eA1+Z5mef1OsBv8y9DvOheG/gt/vv8DvDaPC/zvF4H+G3+ZYgX3WsDv8V/n98BXpvnZZ7X6wC/zb8M8aJ7beC3+O/zO8Br87zM83od4Lf5lyFedK8N/Bb/ff4a+Gie12/zvF4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7aeCv+N/hZYC/5l+G+Ne5FXgQ/7M9A3gwLxrEv85rA7/F/2yvA/w2LxrEv957A18NHON/lkvAewM/zYsO8W9zHHhv4KWBB/Pf61bgr4HvBnb510H8/4b4/w3x/xvi/zfE/2/8I32BkEEM+pnvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBackpack;
impl IconShape for MdBackpack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,8v12c0,1.1-0.9,2-2,2H6c-1.1,0-2-0.9-2-2V8c0-1.86,1.28-3.41,3-3.86V2h3v2h4V2h3v2.14C18.72,4.59,20,6.14,20,8z M6,12v2h10v2h2v-4H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFF0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/u7wV8NLASwO3ArcCPwPcyr8N4n+Htwa+Cngwz993Ax8D7PKvg/if77OAz+ZfdivwNsBf86JD/M/2XcB786L7a+B1gF1eNIj/ub4LeG/+9T4H+GxeNIj/OV4aOMYV7w28Ny/Y7wDHgZfied0KPIQXDeJ/hp8G3op/2SXgtYG/5oqvBj6K5/UywF/zL0P8z/DSwG8Dx3jBLgGvDfw1z3YcuMjzeh3gt/mXIf79Xhp4K+Clgb8G/hr4Gf71Xhr4beAYz+sS8NrAX/O8zPN6HeC3+Zch/n2+CvhontdfA28D3Mq/zksDvw0c4zm9DPDXPH/meb0O8Nv8yxD/dl8NfBQv2K3AywC7/Ou8NPDbwDGeTbxg5nm9DvDb/MsQ/zYvDfwV/7LPAT6bf72XBn4bOMYV4gUzz+t1gN/mX4b4t/ls4LP4l90KPIR/m5cGfhvYBR7MC2ae1+sAv82/DPFv89PAW/GiEf92Lw08GPhpXjDzvF4H+G3+ZYh/m88GPot/2SXgOP+5zPN6HeC3+Zch/m3eGvgp/mXfA7w3/7nM83od4Lf5lyH+7f4aeCleuIcAt/Kfyzyv1wF+m38Z4t/uwcBvAw/i+Xsf4Lv5z2ee1+sAv82/DPHvcxz4aOC9gQcBl4CfBj4buJX/GuZ5vQ7w2/zLEP/7mef1OsBv8y9D/O9nntfrAL/NvwzxH+M48FJccQn4a/7rmOf1OsBv8y9D/PscB74KeG+e063AVwNfw38+87xeB/ht/mWIf7vjwG8BL80L9t3A+/Cfyzyv1wF+m38Z4t/ut4DX5l/23cD78J/HPK/XAX6bfxni3+algb/iRfc1wEfzn8M8r9cBfpt/GeLf5rOBz+Jf532A7+Y/nnlerwP8Nv8yxL/NTwNvxb/e+wDfzX8s87xeB/ht/mWIf5vfBl6Lf5v3Ab6b/zjmeb0O8Nv8yxDP30sDx3jBvhp4af7tPhv4bf5j/DbP66OBv+Y5XQL+mueEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+Phv4LP5v+Rzgs3lOiOfvrYGf4v+WtwF+mueEeMH+Gngp/m/4G+CleV6IF+zBwE8DL8X/bn8DvDVwK88L8cIdBz4aeG/gQfzv8gzgu4GvBnZ5/hAvutcGfov/HV4H+G3+ZYh/nb8GXor/2f4GeGleNIh/nePAewOvDRznf5Zd4LeB7wZ2edEg/n9D/P+G+P8N8f8b4v83/hH8HLZBqq5vogAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBathtub;
impl IconShape for MdBathtub {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "7",
                cy: "7",
                r: "2",
            }
            path {
                d: "M20,13V4.83C20,3.27,18.73,2,17.17,2c-0.75,0-1.47,0.3-2,0.83l-1.25,1.25C13.76,4.03,13.59,4,13.41,4 c-0.4,0-0.77,0.12-1.08,0.32l2.76,2.76c0.2-0.31,0.32-0.68,0.32-1.08c0-0.18-0.03-0.34-0.07-0.51l1.25-1.25 C16.74,4.09,16.95,4,17.17,4C17.63,4,18,4.37,18,4.83V13h-6.85c-0.3-0.21-0.57-0.45-0.82-0.72l-1.4-1.55 c-0.19-0.21-0.43-0.38-0.69-0.5C7.93,10.08,7.59,10,7.24,10C6,10.01,5,11.01,5,12.25V13H2v6c0,1.1,0.9,2,2,2c0,0.55,0.45,1,1,1 h14c0.55,0,1-0.45,1-1c1.1,0,2-0.9,2-2v-6H20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBeachAccess;
impl IconShape for MdBeachAccess {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.127 14.56l1.43-1.43 6.44 6.443L19.57 21zm4.293-5.73l2.86-2.86c-3.95-3.95-10.35-3.96-14.3-.02 3.93-1.3 8.31-.25 11.44 2.88zM5.95 5.98c-3.94 3.95-3.93 10.35.02 14.3l2.86-2.86C5.7 14.29 4.65 9.91 5.95 5.98zm.02-.02l-.01.01c-.38 3.01 1.17 6.88 4.3 10.02l5.73-5.73c-3.13-3.13-7.01-4.68-10.02-4.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7KXBt4KeGngOP81Pgb4a57TSwNfxYtmF/hr4GeAv+YFQ7xwXwV8NP/1Xgf4bZ7TawO/xb/eVwMfw/OHeMG+Gvgo/nu8DvDbPKfXBn6Lf5uvAT6a54V4/l4a+Cv++7wO8Ns8p9cGfot/u4cAt/KcEM/fZwOfxX+f1wF+m+f02sBv8W/3OcBn85wQz99PA2/Ff5/XAX6b5/TawG/xb/czwFvznBDP328Dr8V/n9cBfpvn9NrAb/Fv9zvAa/OcEM/fbwOvxX+f1wF+m+f02sBv8W/3O8Br85wQz99vA6/Ff5/XAX6b5/TawG/xb/c7wGvznBDP328Dr8V/n9cBfpvn9NrAb/Fv9zvAa/OcEM/fbwOvxX+f1wF+m+f02sBv8W/3O8Br85wQz99vA6/Ff5/XAX6b5/TawG/xb/c7wGvznBDP328Dr8V/n9cBfpvn9NrAb/Fv9zvAa/OcEM/fbwOvxX+f1wF+m+f02sBv8W/3O8Br85wQz99vA6/Ff5/XAX6b5/TawG/xb/c7wGvznBDP328Dr8W/3ucAPw38NfDawGcDr8W/3usAv81zem3gt/i3+x3gtXlOiOfvt4HX4l/ndYDf5nl9N/Be/Ou8DvDbPKfjwEvzb7cL/DXPCfH8/TbwWrzovgd4b56/48BF/nVeB/ht/vMhnr/fBl6LF93HAF/NC/bbwGvxonsd4Lf5z4d4/n4beC1edB8DfDUv2G8Dr8WL7nWA3+Y/H+L5+23gtXjRfQ/w3jx/x4GL/Ou8DvDb/OdDPH+/DbwW/zqvA/w2z+u7gPfmX+d1gN/mOR0HXop/u0vAX/OcEM/fbwOvxb/eZwM/A/w18FrAZwOvzb/e6wC/zXN6beC3+Lf7HeC1eU6I5++3gdfiv8/rAL/Nc3pt4Lf4t/sd4LV5Tojn77eB1+K/z+sAv81zem3gt/i3+x3gtXlOiOfvt4HX4r/P6wC/zXN6beC3+Lf7HeC1eU6I5++3gdfiv8/rAL/Nc3pt4Lf4t/sd4LV5Tojn77eB1+K/z+sAv81zem3gt/i3+x3gtXlOiOfvt4HX4r/P6wC/zXN6beC3+Lf7HeC1eU6I5++3gdfiv8/rAL/Nc3pt4Lf4t/sd4LV5Tojn77eB1+K/z+sAv81zem3gt/i3+x3gtXlOiOfvt4HX4r/P6wC/zXN6beC3+Lf7HeC1eU6I5++ngbfiv8/rAL/Nc3pt4Lf4t/sZ4K15Tojn77OBz+K/z+sAv81zem3gt/i3+xzgs3lOiOfvpYG/4r/P6wC/zXN6beC3+Ld7CHArzwnxgn018FH893gd4Ld5Tq8N/Bb/Nl8DfDTPC/HCfTXwUfzXex3gt3lOrw38Fv96XwN8NM8f4l/20sBbAy8NHOe/xkcDf81zemngq3nR7AJ/DXw3cCsvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGrR6ZBzRvcBgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBento;
impl IconShape for MdBento {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16,11V5h4c1.1,0,2,0.9,2,2v4H16z M20,19c1.1,0,2-0.9,2-2v-4h-6v6H20z M14,5v14H4c-1.1,0-2-0.9-2-2V7c0-1.1,0.9-2,2-2H14z M9.5,12c0-0.83-0.67-1.5-1.5-1.5S6.5,11.17,6.5,12s0.67,1.5,1.5,1.5S9.5,12.83,9.5,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3Ec+C3gpXnR/DXwOsAu/7kQ//mOA78FvDT/On8NvA6wy38exH+u48BvAS/Nv81fA68D7PKfA/Gf5zjwW8BL8+/z18DrALv8x0P85zgO/Bbw0vzH+GvgdYBd/mMh/nP8NPBWPKe/AT6aF81XAy/Fc/oZ4K35j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4V40bw0cIwX3VcDL81z+mvgo3nRfDXw0jynvwY+mhfdJeCveeEQL9xrA98FPJj/nW4F3gf4bZ4/xAv23sB38X/D+wDfzfNCPH/HgacDx/m/YRd4CLDLc0I8fx8NfBX/t3wM8NU8J8Tz993Ae/F/y/cA781zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH/fDbwX/7F+B3htXjS/DbwW/7G+BvhonhPi+fto4Kv4j/U7wGvzovlt4LX4j/UxwFfznBDP33HgVuAY/3F+B3htXjS/DbwW/3EuAQ8GdnlOiBfsvYHv4j/O7wCvzYvmt4HX4j/O2wA/zfNCvHBvDXw18CD+/X4HeG1eNL8NvBb/fs8A3hv4bZ4/xIvmpYHj/PvsAn/Ni+algeP8++wCf80Lh/j/DfH/G+JfZv5j/A7w2rxofht4Lf5jiBcM8S8z/zF+B3htXjS/DbwW/zHEC4b4l5n/GH8NfDQvmq8GXpr/GOIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y3aBY/zvdAk4zguG+Jd9N/Be/O/0PcB784Ih/mUvDfwV/zu9DPDXvGCIF817A9/F/y7vA3w3LxziRffSwEcDbw0c43+mS8BPA18N/DX/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ETb8fEFOn0maAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBusinessCenter;
impl IconShape for MdBusinessCenter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 16v-1H3.01L3 19c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2v-4h-7v1h-4zm10-9h-4.01V5l-2-2h-4l-2 2v2H4c-1.1 0-2 .9-2 2v3c0 1.11.89 2 2 2h6v-2h4v2h6c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zm-6 0h-4V5h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/GMeBnwI+Bvhr/vdA/PsdB34LeGlgF3gd4K/53wHx73Mc+C3gpXm2XeB1gL/mfz7Ev91x4LeAl+Z57QKvA/w1/7Mh/m2OA78FvDQv2C7wOsBf8z8X4l/vOPBbwEvzL9sFXgf4a/5nQvzrHAd+C3hpXnS7wOsAf83/PIgX3XHgt4CX5l9vF3gd4K/5nwXxojkO/Bbw0vzb7QKvA/w1/3Mg/mXHgd8CXpp/v13gdYC/5n8GxL/ss4HP4j/OLvA6wF/z3w/xLzsO/DXwIP7j7AKvA/w1/70QL5rXBn6L/1i7wOsAf81/H8SL7rOBz+I/1i7wOsBf898D8a/z18BL8R9rF3gd4K/5r4f413lp4K/4j7cLvA7w1/zXQvzrfTTwVfzH2wVeB/hr/usg/m1+G3gt/uPtAq8D/DX/NRD/Ng8G/ho4xn+8XeB1gL/mPx/i3+6tgZ/iP8cu8DrAX/OfC/Hv89PAW/GfYxd4HeCv+c+D+Pc5Dvw18CD+c+wCrwP8Nf85EP9+rw38Fv95doHXAf6a/3iI/xhfDXwU/3l2gdcB/pr/WIj/GMeB3wZeiv88u8DrAH/NfxzEf5yXBv6K/1y7wOsAf81/DMR/rM8GPov/XLvA6wB/zb8f4j/ebwOvxX+uXeB1gL/m3wfxH+/BwF8Dx/jPtQu8DvDX/Nsh/nO8N/Bd/OfbBV4H+Gv+bRD/eX4aeCv+8+0CrwP8Nf96iP88x4FbgWP859sFXgf4a/51EP+53hr4Kf5r7AKvA/w1LzrEf76vBj6K/xq7wOsAf82LBvGf7zjw18CD+I/xPcBvA9/F87cLPATY5V+G+K/x0sBf8e/3PcB7c8V7A9/F8/c9wHvzL0P81/ls4LP4t/se4L15Tu8NfBfPaxc4wb8M8V/rr4GX4l/ve4D35vn7aOCreF4PAW7lhUP813pp4LeBY7zovgd4b56/48BvAS/N8xL/MsR/vY8GvooXzfcA783zdxz4LeCleV5/A7w0/zLEf4/fBl6LF+57gPfm+TsO/Bbw0jx/rwP8Nv8yxH+PBwNP5wX7HuC9ef6OA78FvDTP39cAH82LBvHf46eAt+b5+x7gvXn+jgO/Bbw0z9/3AO/Niw7xX++lgb/i+fse4L15/o4DvwW8NM/f9wDvzb8O4r/eRwNfxfP6HuC9ef6OA78FvDTP3/cA782/HuK/3mcDn8Xzehngr3lex4HfAl6a5+97gPfm3wbxX++9ge/iee0CrwP8Nc92HPgt4KV5/r4HeG/+7RD/9R4M/DVwjOe1C7wO8NfAceC3gJfm+fse4L3590H89/ho4Kt4/naBtwG+Cnhpnr/vAd6bfz/Ef5/vBt6Lf73vAd6b/xiI/17fDbwXL7rvAd6b/ziI/37fDbwX/7LvAd6b/1iI/xm+G3gvXrDvAd6b/3iI/zneG/hq4BjPdgn4aOC7+c+B+J/ntXm23+Y/F+L/N8T/b4j/3xD/vyH+f+MfAUMAr0G6XU1UAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCarpenter;
impl IconShape for MdCarpenter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.73,14.23L7,1.5L3.11,5.39l8.13,11.67c-0.78,0.78-0.78,2.05,0,2.83l1.41,1.41c0.78,0.78,2.05,0.78,2.83,0l4.24-4.24 C20.51,16.28,20.51,15.01,19.73,14.23z M14.07,19.88l-1.41-1.41l4.24-4.24l1.41,1.41L14.07,19.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/MueAXw08Ndc8dLAVwMP4j/GM4CPBv6aK14a+GrgQfzLxAuG+JeZF+4ZwEsDuzyn48BfAw/i3+cZwEsDuzyn48BfAw/ihRMvGOJfZl64twF+mufvrYGf4t/nbYCf5vl7a+CneOHEC4b4l5kX7gSwy/P3YODp/PucAHZ5/h4MPJ0XTrxgiH+ZeeFOALs8fw8Gns6/zwlgl+fvwcDTeeHEC4b4l5kX7m2An+b5e2vgp/j3eRvgp3n+3hr4KV448YIh/mXmhbsVeBlgl+d0HPgr4MH8+9wKvAywy3M6DvwV8GBeOPGCIf5l5l92K/DRwN9wxUsBXw08mP8YtwIfDfwNV7wU8NXAg/mXiRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/8Z4BfDTw11zx0sBXAw/iP554wRD/MvMf6xnASwO7PKfjwF8DD+I/lnjBEP8y8x/rbYCf5vl7a+Cn+I8lXjDEv8z8xzoB7PL8PRh4Ov+xxAuG+JeZ/1gngF2evwcDT+c/lnjBEP8y8x/rbYCf5vl7a+Cn+I8lXjDEv8z8x7oVeBlgl+d0HPgr4MH8xxIvGOJfZv7j3Qp8NPA3XPFSwFcDD+Y/nnjBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzL3sG8NHAX3PFSwNfDTyI/xjPAD4a+GuueGngq4EH8S8TLxjiX2ZeuGcALw3s8pyOA38NPIh/n2cALw3s8pyOA38NPIgXTrxgiH+ZeeHeBvhpnr+3Bn6Kf5+3AX6a5++tgZ/ihRMvGOJfZl64E8Auz9+Dgafz73MC2OX5ezDwdF448YIh/mXmhTsB7PL8PRh4Ov8+J4Bdnr8HA0/nhRMvGOJfZl64twF+mufvrYGf4t/nbYCf5vl7a+CneOHEC4b4l5kX7lbgZYBdntNx4K+AB/PvcyvwMsAuz+k48FfAg3nhxAuG+JeZf9mtwEcDf8MVLwV8NfBg/mPcCnw08Ddc8VLAVwMP5l8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv2wXOMb/TpeA47xgiH/ZdwPvxf9O3wO8Ny8Y4l/2YODp/O/0EOBWXjDEi+a9ge/if5f3Ab6bFw7xonsw8NnAWwPH+J/pEvDTwGcDt/IvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfASzHokFpkV/hAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCasino;
impl IconShape for MdCasino {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM7.5 18c-.83 0-1.5-.67-1.5-1.5S6.67 15 7.5 15s1.5.67 1.5 1.5S8.33 18 7.5 18zm0-9C6.67 9 6 8.33 6 7.5S6.67 6 7.5 6 9 6.67 9 7.5 8.33 9 7.5 9zm4.5 4.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4.5 4.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm0-9c-.83 0-1.5-.67-1.5-1.5S15.67 6 16.5 6s1.5.67 1.5 1.5S17.33 9 16.5 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChargingStation;
impl IconShape for MdChargingStation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5,11l-3,6v-4h-2l3-6v4H14.5z M7,1h10c1.1,0,2,0.9,2,2v18c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V3C5,1.9,5.9,1,7,1z M7,6 v12h10V6H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv89r8Zx+h/96iP9aDwY+C3hr4DjP67uB7wF+m/8aiP86nwV8Ni+arwY+hv98iP8a3wW8N/863w28D/+5EP/5Phv4LP5tvgb4aP7zIP5zPRh4Ov8+DwFu5T8H4j/XdwPvxfN6BvDRwF9zxUsDXw08iOf1PcB7858D8Z/rInCc5/QM4KWBXZ7TceCvgQfxnHaBE/znQPzneW3gt3hebwP8NM/fWwM/xfN6HeC3+Y+H+M/z2sBv8bxOALs8fw8Gns7zeh3gt/mPh/if5cHA03lerwP8Nv/xEP+zvDXwUzyvlwH+mv94iP85jgN/BTyY53QJOM5/DsR/v+PAawFfDTyY5/U9wHvznwPx3+ezgI8GjvPCPQS4lf8ciP8e7w18F/+yjwG+mv88iP8e3w28Fy/c9wDvzX8uxH+P3wZei+fvEvDVwGfznw/x3+O3gdfiOV0Cvhv4auBW/msg/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv96Dgd8CHsxzuhV4HeBW/usg/mu9NfBdwHGev13gfYCf5r8G4r/OVwEfzYvmq4GP4T8f4j/fg4GfAl6af52/Bt4GuJX/PIj/XG8NfBdwnH+bXeB9gJ/mPwfiP89XAR/NC3YJ+Giu+GrgGC/YVwMfw388xH+8BwM/Bbw0L9jfAO8N/DVXvDTw3cBL8YL9NfA2wK38x0H8x3pr4LuA47xg3wN8NLDLczoOfDXwXrxgu8D7AD/NfwzEf5yvAj6aF+wS8NHAd/PCvTfw1cAxXrCvBj6Gfz/Ev9+DgZ8CXpoX7G+A9wb+mhfNSwPfDbwUL9hfA28D3Mq/HeLf562B7wKO84J9D/DRwC7/OseBrwbeixdsF3gf4Kf5t0H8230V8NG8YJeAjwa+m3+f9wa+GjjGC/bVwMfwr4f413sw8FPAS/OC/Q3w3sBf8x/jpYHvBl6KF+yvgbcBbuVFh/jXeWvgu4DjvGDfA3w0sMt/rOPAVwPvxQu2C7wP8NO8aBAvuq8CPpoX7BLw0cB385/rvYGvBo7xgn018DH8yxD/sgcDPwW8NC/Y3wDvDfw1/zVeGvhu4KV4wf4aeBvgVl4wxAv31sB3Acd5wb4H+Ghgl/9ax4GvBt6LF2wXeB/gp3n+EC/YVwEfzf8NXw18DM8L8fy9N/Bd/N/yPsB385wQz99PA2/F/y3fA7w3zwnx/H008FX83/IxwFfznBDP33Hgt4GX4v+GvwFeG9jlOSFeuNfm/4bf5vlD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E5FuUQWBSMk8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCheckroom;
impl IconShape for MdCheckroom {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.6,18.2L13,11.75v-0.91c1.65-0.49,2.8-2.17,2.43-4.05c-0.26-1.31-1.3-2.4-2.61-2.7C10.54,3.57,8.5,5.3,8.5,7.5h2 C10.5,6.67,11.17,6,12,6s1.5,0.67,1.5,1.5c0,0.84-0.69,1.52-1.53,1.5C11.43,8.99,11,9.45,11,9.99v1.76L2.4,18.2 C1.63,18.78,2.04,20,3,20h9h9C21.96,20,22.37,18.78,21.6,18.2z M6,18l6-4.5l6,4.5H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChildCare;
impl IconShape for MdChildCare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "14.5",
                cy: "10.5",
                r: "1.25",
            }
            circle {
                cx: "9.5",
                cy: "10.5",
                r: "1.25",
            }
            path {
                d: "M22.94 12.66c.04-.21.06-.43.06-.66s-.02-.45-.06-.66c-.25-1.51-1.36-2.74-2.81-3.17-.53-1.12-1.28-2.1-2.19-2.91C16.36 3.85 14.28 3 12 3s-4.36.85-5.94 2.26c-.92.81-1.67 1.8-2.19 2.91-1.45.43-2.56 1.65-2.81 3.17-.04.21-.06.43-.06.66s.02.45.06.66c.25 1.51 1.36 2.74 2.81 3.17.52 1.11 1.27 2.09 2.17 2.89C7.62 20.14 9.71 21 12 21s4.38-.86 5.97-2.28c.9-.8 1.65-1.79 2.17-2.89 1.44-.43 2.55-1.65 2.8-3.17zM19 14c-.1 0-.19-.02-.29-.03-.2.67-.49 1.29-.86 1.86C16.6 17.74 14.45 19 12 19s-4.6-1.26-5.85-3.17c-.37-.57-.66-1.19-.86-1.86-.1.01-.19.03-.29.03-1.1 0-2-.9-2-2s.9-2 2-2c.1 0 .19.02.29.03.2-.67.49-1.29.86-1.86C7.4 6.26 9.55 5 12 5s4.6 1.26 5.85 3.17c.37.57.66 1.19.86 1.86.1-.01.19-.03.29-.03 1.1 0 2 .9 2 2s-.9 2-2 2zM7.5 14c.76 1.77 2.49 3 4.5 3s3.74-1.23 4.5-3h-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/c7w08FHArVzx28AzgFv5z4P4n+O1gd/ief018NvA1wC38h8L8T/HawO/xQv328DnAL/NfwzE/xyvDfwWL5qfBj4GuJV/H8T/HK8N/BYvul3gY4Dv5t8O8T/HawO/xb/edwPvw78N4n+O1wZ+i3+b7wbeh389xP8crw38Fv923w28D/86iP85Xhv4Lf59Pgb4al50iP85Xhv4Lf79Xgb4a140iP85Xhv4Lf79/hp4GV40iP85Xhv4Lf5jvA/w3fzLEP9zvDbwW/zHuBV4CP8yxP8crw38Fv9x3gf4bl44xP8crw38Fv9xfgZ4a144xP8crw38Fv+xTgC7vGCI/zleG/gt/mO9DfDTvGCI/zleG/gt/mN9DfDRvGCI/zleG/gt/mP9DvDavGCI5++3gdfihbsVeAj/cV4b+C3+Y90KPIQXDPH8/TbwWvzL3gf4bv5jvDbwW/zHEy8Y4vn7beC1+JfdCjyE/xgvDXw1//FemxcM8fz9NvBavGheB/ht/ndCPH+/DbwWL5rfBl6H/50Qz99LA8d5Xh8NvBXP67f5n+t1eMEQ/zqvDfwW/3tcAo7zgiH+9X4beC3+d/gd4LV5wRD/eq8N/Bb/O/wO8Nq8YIh/m78GXor/+b4G+GheMMS/zXsD38X/fG8D/DQvGOLf7lbgQfzPJl44xL/dewPfxf9cPwO8NS8c4t/nVuBB/M/0OsBv88Ih/n3eG/gu/ud5BvBg/mWIf79bgQfxP8v7AN/Nvwzx7/fewHfxP8fvAK/Niwbx73Mc+Czgo/mf4RLw0sCtvGgQ/3YPBn4KeGn+5/ht4HV40SH+bY4DvwW8NP/zfA3w0bxoEP82Xw18FP9zvQ7w2/zLEP96DwaezvP3N8BPA7cCLw28N3CM/zh/A/w0cCvw0sB7A8d4Xr8NvA7/MsS/3mcDn8Xz+h7gvXlODwZ+Gngp/v2+B3hvntODgZ8GXorn9RDgVl44xL/ebwOvxXP6G+Clef5eGvgr/n3+Bnhpnr+XBv6K5/U5wGfzwiH+9Z4OPJjn9DnAZ/OC/TXwUvzbfQ7w2bxgfw28FM/pc4DP5oVD/OuZ5/U+wHfzgv028Fr8270P8N28YL8NvBbP6WeAt+aFQ/zr3Qo8iOf0NcBH84JdBI7znJ4B3MrzemngGM/pa4CP5gW7CBznOX0O8Nm8cIh/vZ8G3orntAu8DHArz+ujga/ieb0P8N08r58G3orntAu8DHArz+ujga/ieb0P8N28cIh/vfcGvovn9dfA+wB/zbN9FPDZwHGe10OAW3le7w18F8/rr4H3Af6aZ/so4LOB4zyvE8AuLxziX+84cCtwjOfvr4Fd4KWB4zx/3wO8N8/fceBW4BjP318Du8BLA8d5/r4HeG/+ZYh/m48Gvop/m0vASwO38oJ9NPBV/NtcAl4auJV/GeLf7ruB9+Jf732A7+Zf9t3Ae/Gv9zbAT/OiQfz7fDXwUbxoLgHvDfw0L7qvBj6KF80l4L2Bn+ZFh/j3e23gq4GX4gX7HuCzgVv513tr4LOBl+IF+x7gs4Fb+ddB/Md5aeC1gePASwN/DdwK/DZwK/9+Lw28NnAceGngr4FbgZ8Gdvm3Qfz/hvj/DfH/G+L/N8T/b/wjrui9QWOp2QcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChildFriendly;
impl IconShape for MdChildFriendly {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 2v8h8c0-4.42-3.58-8-8-8zm6.32 13.89C20.37 14.54 21 12.84 21 11H6.44l-.95-2H2v2h2.22s1.89 4.07 2.12 4.42c-1.1.59-1.84 1.75-1.84 3.08C4.5 20.43 6.07 22 8 22c1.76 0 3.22-1.3 3.46-3h2.08c.24 1.7 1.7 3 3.46 3 1.93 0 3.5-1.57 3.5-3.5 0-1.04-.46-1.97-1.18-2.61zM8 20c-.83 0-1.5-.67-1.5-1.5S7.17 17 8 17s1.5.67 1.5 1.5S8.83 20 8 20zm9 0c-.83 0-1.5-.67-1.5-1.5S16.17 17 17 17s1.5.67 1.5 1.5S17.83 20 17 20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCorporateFare;
impl IconShape for MdCorporateFare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,7V3H2v18h20V7H12z M10,19H4v-2h6V19z M10,15H4v-2h6V15z M10,11H4V9h6V11z M10,7H4V5h6V7z M20,19h-8V9h8V19z M18,11h-4v2 h4V11z M18,15h-4v2h4V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcceCn+7S4Bf82L7qWBY1xxCfhr/uMhXnSvDfwW/3a/A7w2L9xx4KOA9wYezHO6Ffhu4GuAXf5jIF50rw38Fv92vwO8Ni/YSwM/BTyYF+5W4G2Av+bfD/Gie23gt/i3+x3gtXn+Xhr4LeA4L5pd4GWAW/n3QbzoXhv4Lf7tfgd4bZ6/vwJemn+d3wZeh38fxIvutYHf4t/ud4DX5nm9N/Bd/Nu8D/Dd/NshXnSvDfwW/3a/A7w2z+ungbfieX0O8NXAceC9gc/ief0M8Nb82yFedK8N/Bb/dr8DvDbP6+nAg3lOPwO8Nc/pp4G34jndCjyEfzvEi+61gd/i3+53gNfmef02z+urgZ/mOb038F08L/Fvh3jRvTbwW/zb/Q7w2vzbvTbwWzwv8W+HeNG9NvBb/Nv9DvDa/Nu9NvBbPC/xb4d40b028Fv82/0M8Nb827028Fs8L/Fvh3jRvTTwV/zbfQ7w2fzbvTbwWzwv8W+H+Ne5FXgQ/zYPAW7l3+61gd/ieYl/O8S/zmsDv8W/3tcAH82/z2sDv8XzEv92iH+99wa+GjjGi+ZrgI/m3++1gd/ieYl/O8Tz99LAMV6w48B7Ay8NPJjntQv8NfDdwK38x3hp4Kt5Xq/Ni+YS8Nc8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0K86F4b+C3+Y7wO8Ns8p9cGfov/GK8D/Db/MsSL7rWB3+I/xusAv81zem3gt/iP8TrAb/MvQ7zoXhv4Lf5jvA7w2zyn1wZ+i/8YrwP8Nv8yxIvutYHf4j/G6wC/zXN6beC3+I/xOsBv8y9DvOheG/gt/mO8DvDbPKfXBn6L/xivA/w2/zLEi+61gd/iP8brAL/Nc3pt4Lf4j/E6wG/zL0O86F4b+C3+Y7wO8Ns8p9cGfov/GK8D/Db/MsSL7rWB3+I/xusAv81zem3gt/iP8TrAb/MvQ7zoXhv4Lf5jvA7w2zyn1wZ+i/8YrwP8Nv8yxIvutYHf4j/G6wC/zXN6beC3+I/xOsBv8y9DvOheG/gt/mO8DvDbPKfXBn6L/xivA/w2/zLEi+61gd/iP8brAL/Nc3pt4Lf4j/E6wG/zL0O86F4b+C3+Y7wO8Ns8p9cGfov/GK8D/Db/MsSL7rWB3+I/xusAv81zem3gt/iP8TrAb/MvQ7zoXhv4Lf5jvA7w2zyn1wZ+i/8YrwP8Nv8yxIvutYHf4j/G6wC/zXN6beC3+I/xOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRXcceGn+d/hrYJd/GeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I+Mqn0F0abLVAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCountertops;
impl IconShape for MdCountertops {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,10V7c0-1.66-1.34-3-3-3c-1.66,0-3,1.34-3,3h2c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v3H8c1.1,0,2-0.9,2-2V4H4v4 c0,1.1,0.9,2,2,2H2v2h2v8h16v-8h2v-2H18z M13,18h-2v-6h2V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotStep;
impl IconShape for MdDoNotStep {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.39,4.22l7.9,7.9c0.18,0.2,0.18,0.5-0.01,0.7c-0.1,0.1-0.23,0.15-0.35,0.15s-0.26-0.05-0.35-0.15L6.87,11.1 c-0.11,0.4-0.26,0.78-0.45,1.12l1.4,1.4c0.2,0.2,0.2,0.51,0,0.71c-0.1,0.1-0.23,0.15-0.35,0.15s-0.26-0.05-0.35-0.15l-1.27-1.27 c-0.24,0.29-0.5,0.56-0.77,0.8l1.28,1.28c0.2,0.2,0.2,0.51,0,0.71C6.26,15.95,6.13,16,6,16s-0.26-0.05-0.35-0.15l-1.38-1.38 c-0.69,0.46-1.39,0.79-1.97,1.02C1.52,15.8,1,16.53,1,17.37V20h9.5l3.33-3.33l5.94,5.94l1.41-1.41L2.81,2.81L1.39,4.22z M18.51,15.68l-1.41-1.41l4.48-4.48L23,11.2L18.51,15.68z M20.88,9.08l-4.48,4.48L9.3,6.47L13.8,2L20.88,9.08z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIK0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z0sDx4C/AXb574H4r3cc+CngtXm2jwG+mv96iP96Xw18FM/rdYDf5r8W4r/eReA4z+trgI/mOX0U8NLArcD3ALfyHwvxX888f78DvDbP9lfAS/Nsu8DrAH/NfxzEC/dewPfwH8s8f78DvDZXfDTwVTyvnwHemv84iBfsu4D3Br4beB9eNMeBzwLeG9gFfhr4HGCXZzPP3+8Ar80V3w28F8+f+I+DeP6+C3hvnu27gffhX/ZTwFvznH4aeBuezTx/vwO8Nlf8NvBaPH/iPw7ieX0X8N48r+8G3ocX7MHA03n+Xgb4a64wz9/vAK/NFb8NvBbPn3her8UVv8O/DuI5PRj4a+AYz993A+/D83cc+G3gpXherwP8NleY5+93gNfmit8GXovnTzzbSwM/BTyYK24F3gb4a140iOf10sBvA8d4/r4beB+ev+PAbwMvxXN6HeC3ucI8f78DvDZX/DbwWjx/4tmeDjyY53Qr8BBeNIjn76WB3waO8fx9N/A+PH/Hgd8GXopnex3gt7nCPH+/A7w2V/w28Fo8f+KK1wZ+i+fvdYDf5l+GeMFeGvht4BjP33cD78Pzdxz4beCluOJ1gN/mCvP8/Q7w2lzx28Br8fyJK14b+C2ev9cBfpt/GeKFe2ngt4FjPH/fDbwPz99x4LeBlwJeB/htrjDP3+8Ar80Vvw28Fs+fuOK1gd/i+Xsd4Lf5lyH+ZS8N/DZwjOfvu4H34fk7Dvw28NHAb3OFef5+B3htrvht4LV4/sQVrw38Fs/f6wC/zb8M8aJ5aeC3gWM8f98NvA/P33HgOHArV5jn73eA1+aK3wZei+dPXPHawG/x/L0O8Nv8yxAvupcGfhs4xvP33cD78C8zz9/vAK/NFb8NvBbPn7jitYHf4vl7HeC3+Zch/nVeGvht4BjP33cD78MLZ56/3wFemyt+G3gtnj9xxWsDv8Xz9zrAb/MvQ/zrvTTw28Axnr/vBt6HF+yvgZfief0O8Npc8dvAa/H8iSteG/gtnr/XAX6bfxni3+algd8GjvH8fTfwPjx/x4HfBl6K5/Q7wGtzxW8Dr8XzJ654beC3eP5eB/ht/mWIf7uXBn4bOMbz993A+/D8HQd+G3gpnu13gNfmit8GXovnT1zx2sBv8fy9DvDb/MsQ/z4vDfw2cIzn77uB9+H5Ow78NvBSXPE7wGtzxW8Dr8XzJ654beC3eP5eB/ht/mWIf7+XBn4bOMbz993A+/D8HQd+G3gp4HeA1+aK3wZei+dPXPHawG/x/L0O8Nv8yxD/MV4a+G3gGM/fdwPvw/N3HPhtYBd4ba74beC1eP7EFa8N/BbP3+sAv82/DPEf562Bn+IF+27gfXj+jgOfDXw0V/w28Fo8f+KK1wZ+i+fvdYDf5l+G+I/z2sBv8cJ9N/A+/Mt+G3gtnj9xxWsDv8Xz9zrAb/MvQ/zHeW3gt/iXfTfwPrxwvw28Fs+fuOK1gd/i+Xsd4Lf5lyH+47w28Fu8aL4beB9esJ8G3orn77e54jjw0jx/rwP8Nv8yxH+c1wZ+ixfddwPvw/N3HPht4KX4t3kd4Lf5lyH+47w28Fv863w38D48f8eB3wZein+91wF+m38Z4j/OawO/xb/edwPvw/N3HPht4KX413kd4Lf5lyH+ZceB9wIeDPw18D08f68N/Bb/Nt8NvA/P33Hgt4GX4kX3OsBv8y9DvHAvDfwWcJxn+2vgY4Df5jm9NvBb/Nt9N/A+PH/Hgd8GXooXzesAv82/DPGCvTTwW8Bxnr/fBt4HuJUrXhv4Lf59vht4H56/48BvAy/Fv+x1gN/mX4Z4/l4a+C3gOP+yrwY+B3hp4Lf49/tu4H14/o4Dvw28FC/c6wC/zb8M8fy9NPDbwDFeNLvATwPvzX+M7wbeh+fvOPDbwEvxgr0O8Nv8yxAv2FsDP8V/n+8G3ofn7zjw28BL8fy9DvDb/MsQL9x7A9/Ff5/vBt6H5+848NvAS/G8Xgf4bf5liH/ZewPfxX+f7wbeh+fvOPDbwEvxnF4H+G3+ZYgXzXsD38V/n+8G3ofn7zjw28BL8WyvA/w2/zLEi+69ge/iv893A+/D83cc+G3gpbjidYDf5l+G+Nf5buC9+O/z3cD78PwdB34beCngdYDf5l+G+Nf7buC9+O/z3cD78PwdB34b+Gjgt/mXIf5tvht4L/77fDfwPjx/x4HjwK38yxD/dt8NvBf/fb4beB/+fRD/Pj8NvBX/fb4beB/+7RD/PseB3wZeiv8+3w28D/82iH+/48BvAy/Ff5/vBt6Hfz3Ef4zjwG8DL8V/n+8G3od/HcR/nOPAbwMvxX+f7wbehxcd4j/WceBW4Bj/fb4beB9eNIj/eC8N/DZwjP8+3w28D/8yxH+OlwZ+GzjGf5/vBt6HFw7xn+elgd8GjvHf57uB9+EFQ/znemngt4Fj/Pd5H+C7ef4Q//keDHw18Fb857gEfDXw0cAxntP3AO/NC4b4r/Ng4K2B48Br8++zC/w18NfAT3PFSwO/DRzjiu8B3psXDvF/y0sDvw38NPDe/MsQ//c8GLiVFw3i/zfE/2+I/9/4R41cJ1DweRQoAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotTouch;
impl IconShape for MdDoNotTouch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13,10.17l-2.5-2.5V2.25C10.5,1.56,11.06,1,11.75,1S13,1.56,13,2.25V10.17z M20,12.75V11V5.25C20,4.56,19.44,4,18.75,4 S17.5,4.56,17.5,5.25V11h-1V3.25C16.5,2.56,15.94,2,15.25,2S14,2.56,14,3.25v7.92l6,6V12.75z M9.5,4.25C9.5,3.56,8.94,3,8.25,3 c-0.67,0-1.2,0.53-1.24,1.18L9.5,6.67V4.25z M13,10.17l-2.5-2.5V2.25C10.5,1.56,11.06,1,11.75,1S13,1.56,13,2.25V10.17z M20,12.75 V11V5.25C20,4.56,19.44,4,18.75,4S17.5,4.56,17.5,5.25V11h-1V3.25C16.5,2.56,15.94,2,15.25,2S14,2.56,14,3.25v7.92l6,6V12.75z M9.5,4.25C9.5,3.56,8.94,3,8.25,3c-0.67,0-1.2,0.53-1.24,1.18L9.5,6.67V4.25z M21.19,21.19L2.81,2.81L1.39,4.22l5.63,5.63L7,9.83 v4.3c-1.11-0.64-2.58-1.47-2.6-1.48c-0.17-0.09-0.34-0.14-0.54-0.14c-0.26,0-0.5,0.09-0.7,0.26C3.12,12.78,2,13.88,2,13.88 l6.8,7.18c0.57,0.6,1.35,0.94,2.18,0.94H17c0.62,0,1.18-0.19,1.65-0.52l-0.02-0.02l1.15,1.15L21.19,21.19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/s/wWz+t1eF7vDbwXz+lngK/mXwfxP8drA7/Fc/ob4KV5Xp8NfBbP6XOAz+ZfB/E/x2sDv8Vz+h3gtXlenw18Fs/pY4Cv5l8H8T/HawO/xXP6HeC1eV6fDXwWz+ltgJ/mXwfxP8dLA3/Fc/pr4GV4Xp8NfBbP6XOAz+ZfB/E/i3leDwFu5Tm9NfBTPKe/Bl6Gfx3E/yy/DbwWz+mngbfhOT0YeDrP63WA3+ZFh/if5b2B7+J5vQ/w3Tynnwbeiue0CzwE2OVFg/jPdRx4beCnedHdCjyI57QLvA7w1zzbawO/xfP6a+B1gF3+ZYj/PMeB3wJeGngf4Lt50bw18FM8r13gIcAuz/bdwHvxvL4beB/+ZYj/HMeB3wJemmd7H+C7edF8NPBVPK/fBl6H5/TXwEvxvN4H+G5eOMR/vJcGfgs4zvN6G+CnedF8N/BePK+HALfybMeBvwYexHO6FXgILxziP9ZLA78FHOf5+2vgZXjRHAd+G3gpntP3AO/Nc3pp4K94Xu8DfDcvGOI/zksDvwUc54UTL7q3Bn6K57QLnOB5fTfwXjynrwE+mhcM8R/jpYHfAo7zwv0M8Nb86+wCx3hODwFu5Tm9NvBbPKe/Bl6GFwzx7/fewFcBx/mXPQS4lX+d3wZei+f0OsBv85yOAxd5XuIFQ/z7vDfwXbxovgb4aP71fht4LZ7T6wC/zfMyz0u8YIh/u/cGvosXzSXgwcAu/3q/DbwWz+l1gN/mOT0YeDrP6RnAg3nBEP827w18Fy+6jwG+mn8b87weAtzKc3pt4Ld4Tr8DvDYvGOJf77OAz+ZF9wzgwfzPhPjX+S7gvfnXeR3gt/mfCfGi+y7gvfnXuRV4b/7r/A7/OogXzXcB783/fLcCbwP8NS8axL/su4D35n+PW4GH8KJBvGDHgd8CXpr/fV4H+G3+ZYjn7zjwW8BL87/T6wC/zb8M8byOA78FvDT/Oz0DeDAvGsTz+i3gtfnf6RnAWwN/zYsG8ZzeG/gu/mu9Dv9xfpt/HcRz+ivgpfmvJf77IJ7twcDT+a8n/vsgnu21gd/i/56fBj4H+GueF+LZXhv4Lf5v2gUeAuzynBDP9trAb/F/1+cAn81zQjzbawO/xf9dXwN8NM8J8WyvDfwW/3d9DPDVPCfEs7028Fv83/Q3wEvzvBDP9trAb/Ff73f4z3Mr8NfAV/P8IZ7ttYHf4r+e+O+DeLbXBn6L/3rivw/i2V4b+C3+6/02//F+G/ge4FZeOMSzvTbwW/zfsQu8DvDXvGCIZ3tt4Lf4v+VngLfmBUM822sDv8X/PeIFQzzbawO/xf8tfwO8NC8Y4tleG/gt/m95G+CnecEQz/bawG/xf8PfAJ8N/DQvHOLZXhv4Lf7rif8+iGd7beC3+K8n/vsgnu21gd/iv95r8+/zN8Au/zaI52T+d/oY4Kv510M8p+8G3ov/nV4H+G3+dRDP6cHA0/nf6WuAj+ZfB/G83hv4Lv73+Rrgo/nXQTx/7w18F/+7vA7w2/zrIF6wBwOfDbwX//N9DPDV/OshXjSvzf9cfw3s8m+D+P8N8f8b/wjtALNB1w5/CQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDry;
impl IconShape for MdDry {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.65,4.86l-0.07-0.07c-0.57-0.62-0.82-1.41-0.67-2.2L15,2h-1.89l-0.06,0.43c-0.2,1.36,0.27,2.71,1.3,3.72l0.07,0.06 c0.57,0.62,0.82,1.41,0.67,2.2L14.98,9h1.91l0.06-0.43C17.16,7.21,16.68,5.86,15.65,4.86z M19.65,4.86l-0.07-0.07 c-0.57-0.62-0.82-1.41-0.67-2.2L19,2h-1.89l-0.06,0.43c-0.2,1.36,0.27,2.71,1.3,3.72l0.07,0.06c0.57,0.62,0.82,1.41,0.67,2.2 L18.98,9h1.91l0.06-0.43C21.16,7.21,20.68,5.86,19.65,4.86z M9.12,5l-7.18,6.79C1.34,12.35,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3 h6.25H12h5.75c0.69,0,1.25-0.56,1.25-1.25s-0.56-1.25-1.25-1.25H12v-1h7.75c0.69,0,1.25-0.56,1.25-1.25S20.44,17,19.75,17H12v-1 h8.75c0.69,0,1.25-0.56,1.25-1.25s-0.56-1.25-1.25-1.25H12v-1h6.75c0.69,0,1.25-0.56,1.25-1.25S19.44,10,18.75,10H8.86 c0.64-1.11,1.48-2.58,1.49-2.61c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7C10.22,6.12,9.12,5,9.12,5L9.12,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Ov8DfDZwE8DLw18NPBe/PcRLxjiX2ZedM8AXhrY5Tl9N/Be/PcQLxjiX2ZedJ8DfDbP66WBv+Lf7hJwjH8b8YIh/mXmRfc+wHfz/Jl/m78B3hv4K/5txAuG+JeZF93XAB/N83pr4Kf4t3kZ4K+BzwY+i3898YIh/mXmX+dlgL/m2Y4DvwW8NP96XwN8NFccB/4aeBD/OuIFQ/zLzL/eVwO3AseBjwaO86/3DOClgV2e7bWB3+JfR7xgiH+Z+e/xNsBP87x+GngrXnTiBUP8y8x/vZ8B3prn7zhwK3CMF414wRD/MvOi+xngr4Hf5tkeDLw08N7AMf5ll4CXBm7lBfto4Kt40YgXDPEvM/+yZwBvDfw1L9hx4KuB9+KF+xjgq/mX/TbwWvzLxAuG+JeZF+4S8NLArbxofht4LZ6/vwFemhfNSwN/xb9MvGCIf5l54T4H+GxedA8Gns7z9zLAX/Oi+2zgs3jhxAuG+JeZF+4hwK386/w18FI8p13gr/nXOQ68NC+ceMEQ/zLzwol/vd8GXov/GuIFQ/zLzAsn/vV+G3gt/muIFwzxLzMvnPjX+23gtfivIV4wxL/MvHDiX++3gdfiv4Z4wRD/MvPCiX+93wZei+d0Cfhr/nWOAy/FCydeMMS/zLxw4l/vt4HX4nm9DPDXvOg+G/gsXjjxgiH+ZeaFE/96vw28Fs/rr4GX4UXz0sBf8S8TLxjiX2ZeuN/mir8BPpoX7KWBr+KKlwaO8/x9DPDV/Mt+C3ht/mXiBUP8y8yL5neA1+YFe23gt/iX7QIvA9zKC/bRwFfxohEvGOJfZl40vwO8Ni/YawO/xYvmp4G34fk7DjwdOM6LRrxgiH+ZedH8DvDavGCvDfwWL7q3AX6a5/VTwFvzohMvGOJfZl40vwO8Ni/YawO/xYvuVuBlgF2e7bWB3+JfR7xgiH+ZedH8DvDavGCvDfwW/zpfA3w0VxwH/gp4MP864gVD/MvMi+Z3gNfmBXtt4Lf413sZ4K+BzwY+i3898YIh/mXmRfM7wGvzgr028Fv86/018D7AX/FvI14wxL/MvGh+B3htXrDXBn6Lf5td4Dj/NuIFQ/zLzIvmd4DX5gV7beC3+K8nXjDEv8y8aH4HeG1esNcGfov/euIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLdoFj/O90CTjOC4b4l3038F787/Q9wHvzgiH+ZQ8Gns7/Tg8BbuUFQ7xo3hv4Lv53eR/gu3nhEC+6BwOfDbw1cIz/mS4BPw18NnAr/zLE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hG5iqtBzBtgMQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElevator;
impl IconShape for MdElevator {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M8.5,6c0.69,0,1.25,0.56,1.25,1.25 c0,0.69-0.56,1.25-1.25,1.25S7.25,7.94,7.25,7.25C7.25,6.56,7.81,6,8.5,6z M11,14h-1v4H7v-4H6v-2.5c0-1.1,0.9-2,2-2h1 c1.1,0,2,0.9,2,2V14z M15.5,17L13,13h5L15.5,17z M13,11l2.5-4l2.5,4H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Nd6HeC3+Ze9NvDZwGvxwokXDPEvM/91fgd4bf51vht4L14w8YIh/mXmv85DgFv51zkOXOQFEy8Y4l9m/mt8D/De/Nv8NvBaPH/iBUP8y8x/vkvAg4Fd/m1+G3gtnj/xgiH+ZeY/3+cAn82/zXHgIi+YeMEQ/zLzn+sZwEsDu/zbfBfw3rxg4gVD/MvMf673Ab6bf73XAj4beG1eOPGCIf5l5j/P3wAvzQv32cBn8W8nXjDEv8z853kd4Ld5wY4DTweO828nXjDEv8z85/gd4LV54b4beC/+fcQLhviXmf8cDwFu5QV7MPB0/v3EC4b4l5n/eN8DvDcv3G8Br82/n3jBEP8y8x/rEvBgYJcX7LWB3+I/hnjBEP8y8x/rc4DP5oX7K+Cl+Y8hXjDEv8z8x3kG8NLALi/YewPfxX8c8YIh/mXmP877AN/NC3Yc+CvgwfzHES8Y4l9m/mP8DfDSvHCfDXwW/7HEC4b4l5n/GK8D/DYv2HHg6cBx/mOJFwzxLzP/fr8DvDYv3HcD78V/PPGCIf5l5t/vIcCtvGAPBp7Ofw7xgiH+Zebf53uA9+aF+y3gtfnPIV4wxL/M/NtdAh4M7PKCvTbwW/znES8Y4l9m/u0+B/hsXri/Al6a/zziBUP8y8wL9zvAZwO/zb/eewPfxX8u8YIh/mXmBfse4L35tzkO/BXwYP5ziRcM8S8zL9gJYJd/m88GPov/fOIFQ/zLzPP3O8Br829zHHg6cJz/fOIFQ/zLzPP3O8Br82/z3cB78V9DvGCIf5l5wU4Au/zrPBh4Ov91xAuG+JeZF+y7gffhX+e3gNfmv454wRD/MvPC/Tbw2cDv8C97beC3+K8lXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv2wXOMb/TpeA47xgiH/ZdwPvxf9O3wO8Ny8Y4l/2YODp/O/0EOBWXjDEi+a9ge/if5f3Ab6bFw7xonsw8NnAWwPH+J/pEvDTwGcDt/IvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAS0hhUHBuuwlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEscalator;
impl IconShape for MdEscalator {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2l0-14C21,3.9,20.1,3,19,3z M17,9h-1.7l-5,9H7 c-0.83,0-1.5-0.67-1.5-1.5S6.17,15,7,15h1.7l5-9H17c0.83,0,1.5,0.67,1.5,1.5S17.83,9,17,9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Pm8FvDTw0sBfAz8D/DX/eyD+bR4M/BTw0jyvrwY+hv8dEP96x4G/Ah7MC/Y1wEfzPx/iX++zgc/iX/YQ4Fb+Z0P86z0deDD/ss8BPpv/2RD/euZF8zPAW/M/G+Jfbxc4xr/sc4DP5n82xL/edwPvxb/sbYCf5n82xL/eg4Gn88L9DfDS/M+H+Ld5b+C7eP7+Bnhr4Fb+50P82z0Y+GzgrYFjwDOA7wa+GtjlfwfE/2+I/98Q/3qvxYvmEvDX/M+GeNF9FvDRwHFedH8NvA/w1/zPhHjRfBfw3vzbvQ7w2/zPg/iXvTfwXfz77AIPAXb593lp4K2ABwN/DXwPsMu/HeJf9lfAS/Pv9zXAR/Nv997Ad/GcbgXeBvhr/m0Q/zLzH+dlgL/mX++lgb/i+ftr4GX4t0H8y8x/nN8GXod/vc8GPosX7GWAv+ZfD/EvM/+x3gb4af51vht4L16w1wF+m389xL/M/Me6FXgI/zofDXwVL9gJYJd/PcS/zPzH+xzgs3nRHQf+GngQz+trgI/m3wbxLzP/8XaBlwFu5UX30sB3Ay/Fs30N8NnALv82iH+Z+c/xPcB786/30sBx4K+BXf59EP8y85/ndYDf5r8P4l9m/v2+B3gvntdvA6/Dfx/Ev8z8+70O8NnAa/G83gf4bp6/twY+Cnht/mW/DXwO8Nu86BD/MvPv9zpc8Vs8r1uBlwF2eU4fDXwV/3rvA3w3LxrEv8z8+70O8NvAdwPvxfP6HOCzebbXBn6Lf5td4AQvGsS/zPz7vQ7w28CDgb8GjvG8HgLcChwH/gp4MP92rwP8Nv8yxL/M/Pu9DvDbXPHZwGfxvH4aeBvgp4C35t/ndYDf5l+G+JeZf7/XAX6bZ7sVeBDP66uBj+bf73WA3+ZfhviXmX+/1wF+m2d7a+Cn+M/zOsBv8y9D/MvMv9/rAL/Nc/pt4LX4z/E6wG/zL0P865l/vdcBfpvn9NLAX/HCfQzwVfzrvQ7w2/zLEP965l/vdYDf5nl9N/BePH+/A7w2YP71Xgf4bf5liH8986/3OsBv87yOAz/N8/fewK2A+dd7HeC3+Zch/vXMv97rAL/Nv43513sd4Lf5lyH+9cy/3usAv82/jfnXex3gt/mXIf71zL/e6wC/zb+NedH8DfDRXPHXwC7/MsS/nvnXex3gt/m3MS+a3wFem38dxL+e+dd7HeC3+bcxL5rfAV6bfx3Ev57513sd4Lf5tzEvmt8BXpt/HcS/nvnXex3gt/m3MS+a3wFem38dxL+e+dd7HeC3+bcxL5rfAV6bfx3Ev57513sd4Lf5tzEvmt8BXpt/HcS/nvnXex3gt/m3MS+a3wFem38dxL+e+dd7HeC3+bcxL5rfAV6bfx3Ev57513sd4Lf5tzEvmt8BXpt/HcS/nvnXex3gt/m3MS+a3wFem38dxL+e+dd7HeC3+bcxL5rfAV6bfx3Ev57513sd4Lf5tzEvmt8BXpt/HcS/3m/zr/fRwF/zb/PbvGj+Gvho/nUQ/78h/n9D/P+G+P8N8f8b/wg4Qs1BxIJP3wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEscalatorWarning;
impl IconShape for MdEscalatorWarning {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5,2c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S5.4,2,6.5,2z M15.5,9.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 S17.83,8,17,8S15.5,8.67,15.5,9.5z M18.5,12h-2.84c-0.58,0.01-1.14,0.32-1.45,0.86l-0.92,1.32L9.72,8C9.35,7.37,8.69,7.01,8.01,7H5 C3.9,7,3,7.9,3,9v6h1.5v7h5V11.61L12.03,16h2.2L15,14.9V22h4v-5h1v-3.5C20,12.68,19.33,12,18.5,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFamilyRestroom;
impl IconShape for MdFamilyRestroom {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S16,5.11,16,4z M20,22v-6h2.5l-2.54-7.63C19.68,7.55,18.92,7,18.06,7h-0.12 c-0.86,0-1.63,0.55-1.9,1.37l-0.86,2.58C16.26,11.55,17,12.68,17,14v8H20z M12.5,11.5c0.83,0,1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5 S11,9.17,11,10S11.67,11.5,12.5,11.5z M5.5,6c1.11,0,2-0.89,2-2s-0.89-2-2-2s-2,0.89-2,2S4.39,6,5.5,6z M7.5,22v-7H9V9 c0-1.1-0.9-2-2-2H4C2.9,7,2,7.9,2,9v6h1.5v7H7.5z M14,22v-4h1v-4c0-0.82-0.68-1.5-1.5-1.5h-2c-0.82,0-1.5,0.68-1.5,1.5v4h1v4H14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFence;
impl IconShape for MdFence {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,12v-2h-2V7l-3-3l-2,2l-2-2l-2,2L8,4L5,7v3H3v2h2v2H3v2h2v4h14v-4h2v-2h-2v-2H21z M16,6.83l1,1V10h-2V7.83l0.41-0.41 L16,6.83z M12,6.83l0.59,0.59L13,7.83V10h-2V7.83l0.41-0.41L12,6.83z M11,14v-2h2v2H11z M13,16v2h-2v-2H13z M7,7.83l1-1l0.59,0.59 L9,7.83V10H7V7.83z M7,12h2v2H7V12z M7,16h2v2H7V16z M17,18h-2v-2h2V18z M17,14h-2v-2h2V14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/dg4KOAlwZemytuBX4a+BrgVp7XSwOvBbw08NLAxwC/zb8f4r/WVwEfzQv32cCtwEsDLw28Ns/rdYDf5t8P8V/nu4D35j/G6wC/zb8f4r/GZwOfxX+c1wF+m38/xH+e1wJeG3hp4K35j/U6wG/z74f4j/FawEsDLw28NPDS/Mv+Bvhurnhv4KV40b0O8Nv8+yH+bd4aeC3gpYHX5l/vb4CX5jn9NfBSvGheB/ht/v0Q/za/DbwW/3YfA3w1z+mjga/iRfM6wG/z74f4t/lt4LX4t/sY4Kt5Th8NfBUvmtcBfpt/P8S/zVsDrw28NPBa/Ov9NfAyPKe/Al6aF83rAL/Nvx/iP8ZLA68NvDTw0sBL8S/7a+C7ueK9gZfmRfc6wG/z74f4z/PawEsDrw28Ff+xXgf4bf79EP81Phv4LP7jvA7w2/z7If7rfDfwXvzHeB3gt/n3Q/zXeS/gu3nR/DbwN8BLA6/F83od4Lf590P853tr4KuAB/OvcyvwPsBvAy8NvDbw0sBLAx8N/Db/foj/XF8FfDT/Pp8NfA7/ORD/eb4LeG/+Y3w38D78x0P85/hs4LP4j/UxwFfzHwvxH++1gd/iP8fLAH/NfxzEf7zfAl6b/xy/DbwO/3EQ/7FeG/gt/nO9DvDb/MdA/Mf6aeCt+M/1PcB78x8D8R/rInCc/1y7wAn+YyD+f0P8/4b4j3MceCn+a/wNsMu/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/P+G+P8N8R/nOPBS/Nf4G2CXfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NPBX/Nd4GeCv+fdD/Me6FXgQ/7meATyY/xiI/1ivDfwW/7leB/ht/mMg/uO9N/DVwDH+Y10C3hv4af7jIP5zHAfeG3hp4MH8+9wK/DXw3cAu/7EQ/78h/n9D/P+G+P8N8f8b/whHVHhBVt/I4gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFireExtinguisher;
impl IconShape for MdFireExtinguisher {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7,19h10v1c0,1.1-0.9,2-2,2H9c-1.1,0-2-0.9-2-2V19z M7,18h10v-5H7V18z M17,3v6l-3.15-0.66c-0.01,0-0.01,0.01-0.02,0.02 c1.55,0.62,2.72,1.98,3.07,3.64H7.1c0.34-1.66,1.52-3.02,3.07-3.64c-0.33-0.26-0.6-0.58-0.8-0.95L5,6.5v-1l4.37-0.91 C9.87,3.65,10.86,3,12,3c0.7,0,1.34,0.25,1.85,0.66L17,3z M13,6c-0.03-0.59-0.45-1-1-1s-1,0.45-1,1s0.45,1,1,1S13,6.55,13,6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8d4L+B6ev+PAawM/zf8MiP9Y3wW8N/DdwPvwnI4DvwW8NPA+wHfz3w/xontp4KWB7+b5+y7gvXm27wbehyuOA78FvDTP9j7Ad/PfC/GieWngt4DjwPsA381z+i7gvXle3w18DPBbwEvzvN4H+G7++yD+ZS8N/BZwnGd7H+C7ueK7gPfmBdsFjvP8fQ/w3vz3QbxwLw38FnCc5/U+wGsB782/zfcA781/L8QL997Ad/Ef73uA9+a/H+Jf9t7Ad/Ef53uA9+Z/BsSL5r2B7+Lf73uA9+Z/DsSL7r2B7+Lf7nuA9+Z/FsSL7ruA9+bf5nuA9+Z/HsSL5ruA9+bf5nuA9+Z/JsS/7LuA9+bf5nuA9+Z/LsQL913Ae/Nv8zvAa/M/G+IF+y7gvfn3eR/gu/mfC/H8vTfwXfzHeB/gu/mfCfGCfTfwXvzHeB/gu/mfB/HCfTfwXrxofgd4LV6w9wG+m/9ZEP+y7wbeixfue4D3Bt4b+C5esPcBvpv/ORD/suPA04HjPH/fA7w3z/bewHfxgr0P8N38z4B44Y4DvwW8NM/f9wDvzfN6b+C7eMHeB/hu/vshXrDjwG8BL83z9z3Ae/OCvTfwXbxg7wN8N/+9EM/fceC3gJfm+fse4L35l7038F28YO8DfDf/fRDP31sDP8Xz9z3Ae/Oie2/gu3jB3gf4bv57IF6w9wa+i+f0PcB786/33sB38YK9D/Dd/NdDvHDvDXwXV3wP8N7827038F28YO8DfDf/tRD/svcGXht4b/793hv4Ll6w9wG+m/86iP967w18Fy/Y+wDfzb/fdwG/A3w3Lxjiv8d7A9/FC/Y+wHfzb/ddwHtzxfsA383zh/jv897Ad/GCvQ/w3fzrfRfw3jyn9wG+m+eF+O/13sB38YK9D/DdvOi+C3hvnr+3AX6a54T47/fewHfxgr0P8N38y74LeG+ev78BXhvY5Tkh/md4b+C7eMHeB/huXrDvAt6b5+9vgNcGdnleiP853hv4Ll6w9wG+m+f1XcB78/z9DfDawC7PH+J/lvcGvosX7H2A7+bZvgt4b56/XeAhwC4vGOJ/nvcGvosX7H2A7wa+C3hvXrjvBt6HFwzxP9N7A9/FC/bbwGvzovlu4H14/hD/c7038F38x/hu4H14Xoj/2d4b+C7+Y7wP8N08J8T/fO8NfBf/Pt8DvDfPC/G/w28Br82/zfcA783zh/if77uA9+bf5nuA9+YFQ/zP9l3Ae/Nv8z3Ae/PCIf7n+i7gvfm3+R7gvfmXIf5n+i7gvfm3ex/gu/mXIf7n+S7gvfn3ex/gu3nhEP+zfBfw3vzHeR/gu3nBEP9zfBfw3vzHex/gu3n+EP8zfBfw3vzbfA/w28B38bwuAa8N/DXPH+K/33cB783zdwk4xgv2PcB7c8V7A9/Fs10CXhv4a14wxH+/9wa+i+f1N8BrA18NvBfP63uA9+Y5vTfwXcAl4LWBv+aFQ/zP8N7Ad/FsfwO8NrDLFd8NvBfP9j3Ae/P8vTfw18Bf8y9D/M/x3sB3AX8DvDawy3P6buC9gO8B3pv/GIj/Wd4a+G1gl+fvvYHv5j8O4v83xP9viP/fEP+/If5/4x8B4Oq3QfVD0qEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFitnessCenter;
impl IconShape for MdFitnessCenter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.57 14.86L22 13.43 20.57 12 17 15.57 8.43 7 12 3.43 10.57 2 9.14 3.43 7.71 2 5.57 4.14 4.14 2.71 2.71 4.14l1.43 1.43L2 7.71l1.43 1.43L2 10.57 3.43 12 7 8.43 15.57 17 12 20.57 13.43 22l1.43-1.43L16.29 22l2.14-2.14 1.43 1.43 1.43-1.43-1.43-1.43L22 16.29z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFoodBank;
impl IconShape for MdFoodBank {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3L4,9v12h16V9L12,3z M12.5,12.5c0,0.83-0.67,1.5-1.5,1.5v4h-1v-4c-0.83,0-1.5-0.67-1.5-1.5v-3h1v3H10v-3h1v3h0.5v-3h1 V12.5z M15,18h-1v-3.5h-1v-3c0-1.1,0.9-2,2-2V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFoundation;
impl IconShape for MdFoundation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,12h3L12,3L2,12h3v3H3v2h2v3h2v-3h4v3h2v-3h4v3h2v-3h2v-2h-2V12z M7,15v-4.81l4-3.6V15H7z M13,15V6.59l4,3.6V15H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADsElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fv873Er8N3A1wC7PH+IF91rA7/F/z63Aq8D3MrzQrzoXhv4Lf53+mvgZXheiBfdawO/xf9ebwP8NM8J8aJ7beC3+N/rc4DP5jkhXnSvDfwW//O8Ds/rq4GX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7n+W2e10sDx3lOPwO8Nc8J8aJ7beC3+N/rc4DP5jkhXnSvDfwW/3s9BLiV54R40b028Fv87/Q1wEfzvBAvutcGfov/fb4G+GieP8SL7rWB3+J/nt/hee0Cfw38NPDXvGCIF91rA7/F/zzi3w7xontt4Lf4n0f82yFedK8N/Bb/84h/O8SL7rWB3+J/HvFvh3jRvTbwW/zPI/7tEC+61wZ+i/95xL8d4kX32sBv8T+P+LdDvOheG/gt/ucR/3aIF91rA7/F/yyXgOP82yFedC8N/BX/s/wO8Nr82yH+dW4FHsT/HF8DfDT/doh/ne8G3ov/OV4H+G3+7RD/Og8Gns7/DL8DvDb/Poh/va8GPor/fq8D/Db/Poh/m78GXor/Ph8DfDX/foh/m+PAbwMvxX+9zwE+m/8YiH+frwY+iv8azwA+Gvhp/uMg/v0eDHw28NrAg/iP9zfAdwPfDezyHwvxH+ulgeP8x/lrYJf/PIj/3xD/vyH+f0P8/4Z40R0HXor/Hf4G2OVfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RLP9iQV+xVrAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFreeBreakfast;
impl IconShape for MdFreeBreakfast {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 3H4v10c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.11 0 2-.9 2-2V5c0-1.11-.89-2-2-2zm0 5h-2V5h2v3zM4 19h16v2H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAESklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+d4L+B7+Z0L85zNwK/DdwNcAu/zPgfjPZ55tF/hq4GuAXf77If7zmee1C3w18DXALv99EP/5zAu2C3w18DXALv/1EP/5zL9sF/hq4GuAXf7rIP7zmRfdLvDVwNcAu/znQ/znM/96u8B3A18D3Mp/HsR/PvPv893A5wC38h8P8Z/P/Mf4buBzgFv5j4P4z2f+Y3038DHALv9+iP985j/O7wCfDfw2/zEQ//nMv9/vAJ8N/Db/sRD/+cy/3e8Anw38Nv85EP/5zL/e7wCfDfw2/7kQ//nMi+53gM8Gfpv/Goj/fOZf9gzgvYHf5r8W4j+fecGeAXw28N3890D85zPP6xnAZwPfzX8vxH8+87xeB/ht/vsh/vOZ5/U6wG/z3w/xn888r9cBfpv/foj/fOZ5vQ7w2/z3Q/znM8/rdYDf5r8f4j+feV6vA/w2//0Q//nM83od4Lf574f4z2ee1+sAv81/P8R/PvO8Xgf4bf77If7zmef1OsBv898P8Z/PPK/XAX6b/36I/3zmeb0O8Nv890P85zPP63WA3+a/H+I/n3lerwP8Nv/9EP/5zPN6HeC3+e+H+M9nntfrAL/Nfz/Efz7zvF4H+G3++yH+85nn9TrAb/PfD/Gfzzyv1wF+m/9+iP985nm9DvDb/PdD/Oczz+t1gN/mvx/iP595Xq8D/Db//RD/+czzeh3gt/nvh/jPZ57X6wC/zX8/xH+clwb+mudlntcu8NdccStwK/DbwN8Au/zrPRj4KuClueKvgY8BbuWFQ/zbPRh4K+C9gZcGfgd4bZ6X+de5Ffhp4HuAv+Zf9mDgr4DjPKdd4GWAW3nBEP96DwY+C3hvntPvAK/N8zL/drcCHwP8NC/YTwNvxfP3M8Bb84Ih/nXeG/gunr/fAV6b52X+/b4beB+ev4vAcZ6/W4GH8IIhXnQPBp7OC/Y7wGvzvMx/jI8BvprntQsc4/l7BvBgXjDEi+61gd/iBdsF/prn9dr8x/ga4KN5Xj8NvBXP388Ab80LhvjX+Wvgpfjv8TrAb/O8Hgz8NXCM53QJeGngVl4wxL/OceCngdfiv84l4L2Bn+YFezDw1cBLc8VfAx8N3MoLh/i3eW/gvYHX4j/PM4DvBr4a2OU/B+Lf58HAWwOvDbw08CD+fX4H+G3gp4G/5j8f4j/WceClgePAS/NsDwYeDOwCf82z3QrcCtwK3Mp/PcT/b4j/3xD/vyH+f0P8/8Y/As/ptEG3Qfw3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGolfCourse;
impl IconShape for MdGolfCourse {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "19.5",
                cy: "19.5",
                r: "1.5",
            }
            path {
                d: "M17 5.92L9 2v18H7v-1.73c-1.79.35-3 .99-3 1.73 0 1.1 2.69 2 6 2s6-.9 6-2c0-.99-2.16-1.81-5-1.97V8.98l6-3.06z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGrass;
impl IconShape for MdGrass {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,20H2v-2h5.75l0,0C7.02,15.19,4.81,12.99,2,12.26C2.64,12.1,3.31,12,4,12C8.42,12,12,15.58,12,20z M22,12.26 C21.36,12.1,20.69,12,20,12c-2.93,0-5.48,1.58-6.88,3.93c0.29,0.66,0.53,1.35,0.67,2.07c0.13,0.65,0.2,1.32,0.2,2h2h6v-2h-5.75 C16.98,15.19,19.19,12.99,22,12.26z M15.64,11.02c0.78-2.09,2.23-3.84,4.09-5C15.44,6.16,12,9.67,12,14c0,0.01,0,0.02,0,0.02 C12.95,12.75,14.2,11.72,15.64,11.02z M11.42,8.85C10.58,6.66,8.88,4.89,6.7,4C8.14,5.86,9,8.18,9,10.71c0,0.21-0.03,0.41-0.04,0.61 c0.43,0.24,0.83,0.52,1.22,0.82C10.39,10.96,10.83,9.85,11.42,8.85z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHotTub;
impl IconShape for MdHotTub {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "7",
                cy: "6",
                r: "2",
            }
            path {
                d: "M11.15 12c-.31-.22-.59-.46-.82-.72l-1.4-1.55c-.19-.21-.43-.38-.69-.5-.29-.14-.62-.23-.96-.23h-.03C6.01 9 5 10.01 5 11.25V12H2v8c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-8H11.15zM7 20H5v-6h2v6zm4 0H9v-6h2v6zm4 0h-2v-6h2v6zm4 0h-2v-6h2v6zm-.35-14.14l-.07-.07c-.57-.62-.82-1.41-.67-2.2L18 3h-1.89l-.06.43c-.2 1.36.27 2.71 1.3 3.72l.07.06c.57.62.82 1.41.67 2.2l-.11.59h1.91l.06-.43c.21-1.36-.27-2.71-1.3-3.71zm-4 0l-.07-.07c-.57-.62-.82-1.41-.67-2.2L14 3h-1.89l-.06.43c-.2 1.36.27 2.71 1.3 3.72l.07.06c.57.62.82 1.41.67 2.2l-.11.59h1.91l.06-.43c.21-1.36-.27-2.71-1.3-3.71z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv95x4KeA1+Y5/TbwNsAuz99x4KV4wS4Bf82/DuK/1nHgt4CX5vn7a+B1gF2e12sDv8UL9jvAa/Ovg/iv89LAdwEvzQv318D7AH/Nc3pt4Ld4wX4HeG3+dRD/NV4a+C3gOC+aXeB1gL/m2V4b+C1esN8BXpt/HcR/vtcGfgo4zr/OLvA2wG9zxWsDv8UL9jvAa/Ovg/jP9d7Ad/Hv8z7AdwOvDfwWL9jvAK/Nvw7iP897A9/Ff4z3AW4FfosX7HeA1+ZfB/Gf47uA9+Y/1l8DL80L9jvAa/Ovg/iP913Ae/Nf73eA1+ZfB/Ef5zjwU8Br89/jd4DX5l8H8R/jOPBbwEvz3+d3gNfmXwfx7/fSwHcBL82L7hLw3cBfc8VLA+8NHOPf7neA1+ZfB/Hv89LAbwHHedF9D/DRwC7P6Tjw1cB78W/zO8Br86+D+Ld7beCngOO86H4HeG1euN8GXot/vd8BXpt/HcS/zXsD38W/3kOAW3nhHgw8nX+93wFem38dxL/eewPfxb/e3wAvzYvmr4GX4l/vfYDv5kWH+Nf5LuC9+bf5a+CjedF8NfDS/Nt8N/A+vGgQL7rvAt6b/x2+G3gf/mWIf9lx4KeA1+Z/l98G3gbY5QVDvHDHgd8CXpr/nf4aeB1gl+cP8YK9NPBdwEvzv9tfA+8D/DXPC/H8vTTwW8Bx/m/YBV4H+GueE+Lf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr86/wN8NG8aL4aeCn+dX4HeG3+dRD/Nr8NvBb/Or8DvDYvmt8GXot/nd8BXpt/HcS/zW8Dr8W/zu8Ar82L5reB1+Jf53eA1+ZfB/Fv89vAa/Gv8zvAa/Oi+W3gtfjX+R3gtfnXQfzb/DbwWvzr/A7w2rxofht4Lf51fgd4bf51EP82vw28Fv86vwO8Ni+a3wZei3+d3wFem38dxL/NbwOvxb/O7wCvzYvmt4HX4l/nd4DX5l8H8W/z28Br8a/zO8Br86L5beC1+Nf5HeC1+ddB/Nv8NvBa/Ov8DvDavGh+G3gt/nV+B3ht/nUQ/za/DbwW/zq/A7w2L5rfBl6Lf53fAV6bfx3Ev81vA6/Fv87vAK/Ni+a3gdfiX+d3gNfmXwfxb/PbwGvxr/M7wGvzovlt4LX41/kd4LX510H82/w28Fr86/wO8Nq8aH4beC3+dX4HeG3+dRD/Nr8NvBb/Or8DvDYvmt8GXot/nd8BXpt/HcS/zW8Dr8W/zu8Ar82L5reB1+Jf53eA1+ZfB/Fv89vAa/Gv8zvAa/Oi+W3gtfjX+R3gtfnXQfzbvDRwnH+dXeCvedG8NHCcf51d4K/510H8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwT3TqNB5ZURqgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHouse;
impl IconShape for MdHouse {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,9.3V4h-3v2.6L12,3L2,12h3v8h5v-6h4v6h5v-8h3L19,9.3z M10,10c0-1.1,0.9-2,2-2s2,0.9,2,2H10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv95x4KeA1+Y5/TbwNsAu/3UQ/7WOA78FvDTP318DrwPs8l8D8V/npYHvAl6aF+6vgfcB/pr/fIj/Gi8N/BZwnBfNLvA6wF/znwvxn++1gZ8CjvOvswu8DfDb/OdB/Od6b+C7eMG+hyveixfsfYDv5j8H4j/PewPfxQv2NcBHc8VXAx/FC/Y+wHfzHw/xn+O7gPfmBXsf4Lt5Tu8NfBcv2HcD78N/LMR/vO8C3psX7H2A7+b5e2/gu3jBvht4H/7jIP7jHAd+Cnhtnr9LwGsDf80L99LAbwPHeP5+G3gbYJd/P8R/jOPAbwEvzfN3CXht4K950bw08NvAMZ6/vwZeB9jl3wfx7/fSwHcBL80L9tfALv86DwYezAv218D7AH/Nvx3i3+elgd8CjvPfYxd4HeCv+bdB/Nu9NvBTwHH+e+0CbwP8Nv96iH+b9wa+i/9Z3gf4bv51EP967w18F/8zvQ/w3bzoEP863wW8Ny/Y7/Bf47V4wb4beB9eNIgX3XcB780L9jvAa/Nf47eB1+IF+27gffiXIf5lx4GfAl6bF+53gNfmv8ZvA6/FC/fbwNsAu7xgiBfuOPBbwEvzL/sd4LX5r/HbwGvxL/tr4HWAXZ4/xAv20sB3AS/Ni+Z3gNfmv8ZvA6/Fi+avgfcB/prnhXj+Xhr4LeA4L7rfAV6b/xq/DbwWL7pd4HWAv+Y5If5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8Vz+hvgo/mv8dXAS/Gcfgd4bf51EP82vw28Fs/pd4DX5r/GbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b/xq/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+a/w28Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtfmv8dvAa/Gcfgd4bf51EP82vw28Fs/pr4GP5r/GVwMvzXP6HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei+f0O8Br81/jt4HX4jn9DvDa/Osg/m1+G3gtntPvAK/Nf43fBl6L5/Q7wGvzr4P4t/lt4LV4Tr8DvDb/NX4beC2e0+8Ar82/DuLf5reB1+I5/Q7w2vzX+G3gtXhOvwO8Nv86iH+b3wZei+f0O8Br81/jt4HX4jn9DvDa/Osg/m1eGjjOc9oF/pr/Gi8NHOc57QJ/zb8O4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjgZ6bQYqMK/cAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHouseSiding;
impl IconShape for MdHouseSiding {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,12h3L12,3L2,12h3v8h2v-2h10v2h2V12z M7.21,10h9.58L17,10.19V12H7v-1.81L7.21,10z M14.57,8H9.43L12,5.69L14.57,8z M7,16 v-2h10v2H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKitchen;
impl IconShape for MdKitchen {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2.01L6 2c-1.1 0-2 .89-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.11-.9-1.99-2-1.99zM18 20H6v-9.02h12V20zm0-11H6V4h12v5zM8 5h2v3H8zm0 7h2v5H8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t/lt4LX4z/M6wG/znw/xb/PbwGvxn+d1gN/mPx/i3+a3gdfiP89HA3/Nv84l4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/zyvA/w2L9hvA6/Fc/od4LX510H82/w28Fr853kd4Ld5wX4beC2e0+8Ar82/DuLf5reB1+I/z+sAv80L9tvAa/Gcfgd4bf51EP82vw28Fv95Xgf4bV6w3wZei+f0O8Br86+D+Lf5beC1+M/zOsBv84L9NvBaPKffAV6bfx3Ev81vA6/Ff57XAX6bF+y3gdfiOf0O8Nr86yD+bX4beC3+87wO8Nu8YL8NvBbP6XeA1+ZfB/Fv89vAa/Gf53WA3+YF+23gtXhOvwO8Nv86iH+b3wZei/88rwP8Ni/YbwOvxXP6HeC1+ddB/Nv8NvBa/Od5HeC3ecF+G3gtntPvAK/Nvw7i3+a3gdfiP8/rAL/NC/bbwGvxnH4HeG3+dRD/Nr8NvBb/eV4H+G1esN8GXovn9DvAa/Ovg/i3+W3gtfjP8zrAb/OC/TbwWjyn3wFem38dxL/NbwOvxb/O3wAfzfP6LZ7X6wC/zQv228Br8Zx+B3ht/nUQ/za/DbwW/zq/A7w2z8s8r9cBfpsX7LeB1+I5/Q7w2vzrIP5tfht4Lf51fgd4bZ6XeV6vA/w2L9hvA6/Fc/od4LX510H82/w28Fr86/wO8No8L/O8Xgf4bV6w3wZei+f0O8Br86+D+Lf5beC1+Nf5HeC1eV7meb0O8Nu8YL8NvBbP6XeA1+ZfB/Fv89vAa/Gv89fAR/O8fpvn9TrAb/OC/TbwWjyn3wFem38dxL/NbwOvxX+e1wF+mxfst4HX4jn9DvDa/Osg/m1+G3gt/vO8DvDbvGC/DbwWz+l3gNfmXwfxb/PbwGvxn+d1gN/mBftt4LV4Tr8DvDb/Ooh/m98GXov/PK8D/DYv2G8Dr8Vz+h3gtfnXQfzb/DbwWvzneR3gt3nBfht4LZ7T7wCvzb8O4t/mt4HX4j/P6wC/zQv228Br8Zx+B3ht/nUQ/za/DbwW/3leB/htXrDfBl6L5/Q7wGvzr4P4t/lt4LX4z/M6wG/zgv028Fo8p98BXpt/HcS/zW8Dr8V/ntcBfpsX7LeB1+I5/Q7w2vzrIP5tfht4Lf7zvA7w27xgvw28Fs/pd4DX5l8H8W/z28Br8Z/ndYDf5gX7beC1eE6/A7w2/zqIf5vfBl6L/zyvA/w2L9hvA6/Fc/od4LX510H82/w28Fr853kd4Ld5wX4beC2e0+8Ar82/DuLf5reB1+I/z+sAv80L9tvAa/Gcfgd4bf51EP82vw28Fv95Xgf4bV6w3wZei+f0O8Br86+D+Je9Fs/rq4GX5j/P6wC/zQv228Br8Zz+Gvhontfv8IIh/mXmv97rAL/NC/bbwGvxohEvGOJfZv7rvQ7w27xgvw28Fi8a8YIh/mXmv97rAL/NC/bbwGvxohEvGOJfZv7rvQ7w27xgvw28Fi8a8YIh/mXmv97rAL/NC/bbwGvxohEvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CEMoukFURv6rAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMeetingRoom;
impl IconShape for MdMeetingRoom {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMicrowave;
impl IconShape for MdMicrowave {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.8,10.61L5.37,9.19C5.73,8.79,6.59,8,7.75,8c0.8,0,1.39,0.39,1.81,0.67C9.87,8.88,10.07,9,10.25,9 c0.37,0,0.8-0.41,0.95-0.61l1.42,1.42c-0.36,0.4-1.22,1.19-2.37,1.19c-0.79,0-1.37-0.38-1.79-0.66C8.13,10.12,7.94,10,7.75,10 C7.38,10,6.95,10.41,6.8,10.61z M7.75,15c0.19,0,0.38,0.12,0.71,0.34c0.42,0.28,1,0.66,1.79,0.66c1.16,0,2.01-0.79,2.37-1.19 l-1.42-1.42c-0.15,0.2-0.59,0.61-0.95,0.61c-0.18,0-0.38-0.12-0.69-0.33C9.14,13.39,8.55,13,7.75,13c-1.16,0-2.02,0.79-2.38,1.19 l1.42,1.42C6.95,15.41,7.38,15,7.75,15z M22,6v12c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h16C21.1,4,22,4.9,22,6z M14,6H4v12h10V6z M19,16c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1C18.55,17,19,16.55,19,16z M19,12 c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1C18.55,13,19,12.55,19,12z M19,7h-2v2h2V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b473Mc+Cqu+Bhgl/96iP8eLw18F/DSXPHXwPsAf81/LcR/vfcGvgo4znPaBT4G+G7+6yD+6xwHvgp4b1647wY+BtjlPx/iv8ZLA98FvDQvmr8G3gf4a/5zIf7zvTfwVcBx/nV2gY8Bvpv/PIj/PMeBrwLem3+f7wY+BtjlPx7iP8dLA98FvDT/Mf4aeB/gr/mPhfiP997AVwHH+Y+1C3wM8N38x0H8xzkOfBXw3vzn+m7gY4Bd/v0Q/zFeGvgu4KX5r/HXwPsAf82/D+Lf772BrwKO819rF/gY4Lv5t0P82x0Hvgp4b/57fTfwMcAu/3qIf5uXBr4LeGn+Z/hr4H2Av+ZfB/Gv997AVwHH+Z9lF/gY4Lt50SFedMeBrwLem//Zvhv4GGCXfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfosX7G+Aj+aKrwZeiv8+rwP8Nv8yxIvutYHf4gX7HeC1ueK3gdfiv8/rAL/Nvwzxontt4Ld4wX4HeG2u+G3gtfjv8zrAb/MvQ7zoXhv4LV6w3wFemyt+G3gtXrjfAV6bF81LAx8NvBcvmtcBfpt/GeJF99rAb/GC/Q7w2lzx28BrccXvAD/NFW8NvBZX/A7w2vzrvDfwXfzLXgf4bf5liBfdawO/xQv2O8Brc8VvA68FfA3w0TynrwY+Cvgd4LX51/ts4LN44V4H+G3+ZYgX3WsDv8UL9jvAa3PFbwMPBh7M87cL/DXw2vzrHQcu8sK9DvDb/MsQL7rXBn6LF+x3gNfmit/mitfm+fttrnht/m1+G3gtXrDXAX6bfxniRffawG/xgv0O8Npc8dvAMeBleP7+CrgEvDb/Nr8NvBYv2OsAv82/DPGie23gt3jBfgd4ba74beC1gNcBfpvn9NbATwG/A7w2/za/DbwWL9jrAL/Nvwzxontt4Ld4wX4HeG2u+G3gtYBd4LOBn+GKtwI+GzgO/A7w2vzb/DbwWrxgrwP8Nv8yxIvutYHf4gX7HeC1ueK3gdfihfsd4LX5t/lt4LV4wV4H+G3+ZYgX3WsDv8UL9jvAa3PFSwPHeeF2gb/m3+a3gdfiBXsd4Lf5lyFedK8N/BYv2O8Ar81/jd8GXosX7HWA3+ZfhnjRvTbwW7xgvwO8Nv81fht4LV6w1wF+m38Z4kX32sBv8YL9DvDaXPHbwGvxH+t3gNfmit8GXosX7HWA3+ZfhnjRvTbwW7xgvwO8Nlf8NvBa/Mf6HeC1ueK3gdfiBXsd4Lf5lyFedK8N/BYv2O8Ar80Vvw28Fv+xfgd4ba74beC1eMFeB/ht/mWIF91rA7/FC/Y7wGtzxW8Dr8V/rN8BXpsrfht4LV6w1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxni/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjMxbxBJWQTlQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNightShelter;
impl IconShape for MdNightShelter {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3L4,9v12h16V9L12,3z M9.75,12.5c0.69,0,1.25,0.56,1.25,1.25S10.44,15,9.75,15S8.5,14.44,8.5,13.75S9.06,12.5,9.75,12.5z M17,18h-1v-1.5H8V18H7v-7h1v4.5h3.5V12H15c1.1,0,2,0.9,2,2V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoBackpack;
impl IconShape for MdNoBackpack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.19,21.19L2.81,2.81L1.39,4.22l2.76,2.76C4.06,7.31,4,7.64,4,8v12c0,1.1,0.9,2,2,2h12c0.34,0,0.65-0.09,0.93-0.24 l0.85,0.85L21.19,21.19z M6,14v-2h3.17l2,2H6z M14.83,12L6.98,4.15c0.01,0,0.01-0.01,0.02-0.01V2h3v2h4V2h3v2.14 c1.72,0.45,3,2,3,3.86v9.17l-2-2V12H14.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoCell;
impl IconShape for MdNoCell {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.83,6l-3.7-3.7C5.42,1.55,6.15,1,7,1l10,0.01c1.1,0,2,0.89,2,1.99v13.17l-2-2V6H8.83z M19.78,22.61l-0.91-0.91 C18.58,22.45,17.85,23,17,23H7c-1.1,0-2-0.9-2-2V7.83L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M15.17,18L7,9.83V18H15.17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0O8cK8N/Db/dyFesO8C3hv4buB9eP6OA78NvBT/c4kXDPH8fRfw3jzbdwPvw/N3HPht4KX4n0m8YIjn9V3Ae/O8vht4H56/48BvAy/F/zziBUM8pwcDfw0c4/n7buB9eP6OA78NvBT/s4gXDPG8Xhr4beAYz993A+/D83cc+G3gpfifQ7xgiOfvpYHfBo7x/H038D48f8eBl+a/zksDX8Xz9zfAS/OCIV6wlwZ+GzjG8/fdwPvw3+ulgd8CjvO8ngG8NLDLC4Z44V4a+G3gGM/fdwPvw3+P48DTgeM8r0vAawN/zQuH+Je9NPDbwDGev+8G3of/WseB3wJemud1CXht4K/5lyFeNC8N/DZwjOfvu4H34b/OXwEvzfP3NsBP86JBvOheGvht4BjP33cD78ML9mDgQfz7vTfw3jx/7wN8Ny86xL/OSwO/DRzj+ftu4H14/o4Dvw28FP85Pgf4bP51EP96Lw38NnCM5++7gffh+TsO/DbwUvzH+h7gvfnXQ/zbvDTw28Axnr/vBt6H5+848NvAS/Ef42eAt+bfBvFv99LAbwPHeP6+G3gfnr/jwG8DL8W/z98Arw3s8m+D+Pd5aeC3gWM8f98NvA/P33Hgt4GX4t/mb4DXBnb5t0P8+7008NvAMZ6/7wbeh+fvOPDbwEvxr3MJeG3gr/n3QfzHeGngt4FjPH/fDbwPz99x4LeBl+JFcwl4beCv+fdD/Md5aeC3gWM8f98NvA/P33Hgt4GX4l/2NsBP8x8D8R/rpYHfBo7x/H038D48f8eB3wZeihfsfYDv5j8O4j/eSwO/DRzj+ftu4H14/o4Dvw28FM/rc4DP5j8W4j/HSwO/DRzj+ftu4H14/o4Dvw28FM/2PcB78x8P8Z/npYHfBo7x/H038D48f8eB3wZeCvgZ4K35z4H4z/XSwG8Dx3j+vht4H56/48BXAx8N7PKfA/Gf76WB3waO8fx9N/A+/PdA/Nd4aeC3gWM8f98NvA//9RD/dV4a+G3gGM/fdwPvw38txH+t9wa+ixfsu4H34b8O4r/eewPfxQv23cD78F8D8d/jt4HX4gX7buB9+M+H+O/x28Br8cJ9N/A+/OdC/Pf4beC1+Jd9N/A+/OdB/Pf4beC1eNF8N/A+/OdA/Pf4beC1eNF9N/A+/MdD/Pf4beC1eE7PAI4Dx3j+vht4H/5jIf57/DbwWjyn3wE+Gvht4BjP33cD78N/HMR/j98GXovn9DvAawMvDfw2cIzn77uB9+E/BuK/x28Dr8Vz+h3gtbnipYHfBo7x/H038D78+yH+e/w28Fo8p98BXptne2ngt4FjPH/fDbwP/z6I/x6/DbwWz+l3gNfmOb008NvAMZ6/7wbeh387xH+P3wZei+f0O8Br87xeGvht4BjP33cD78O/DeK/x28Dr8Vz+h3gtXn+Xhr4beAYz993A+/Dvx7i3++1+Nf7auCleU5/DXw0L9hLA1/NC/bdwPvwr4P49zP/c3w38D686BD/fuZ/lu8G3ocXDeLfz/zP893A+/AvQ/z7mf+Zvht4H144xL+f+e91CTjG8/fdwPvwgiH+93tp4LeBYzx/7wN8N88f4v+GlwZ+GzjGc/oe4L15wRD/d7w08NvAMa74HuC9eeEQ/7e8NPDbwE8D782/DPF/z4OBW3nRIP5/Q/z/hvj/jX8E6F2uQYpJUFsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoDrinks;
impl IconShape for MdNoDrinks {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.83,3H21v2l-6.2,6.97L9.83,7h6.74l1.78-2H7.83L5.83,3z M19.78,22.61L18,20.83V21H6v-2h5v-5l-1.37-1.54L1.39,4.22 l1.41-1.41L3,3l18.19,18.19L19.78,22.61z M16.17,19L13,15.83V19H16.17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoFlash;
impl IconShape for MdNoFlash {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.93,13.93L2.45,2.45L1.04,3.87l5.3,5.3L6.14,9.4H3.6C2.72,9.4,2,10.12,2,11v9.4C2,21.28,2.72,22,3.6,22h12.8 c0.75,0,1.38-0.52,1.55-1.22l2.18,2.18l1.41-1.41L18,18L13.93,13.93z M10,20c-2.21,0-4-1.79-4-4c0-1.95,1.4-3.57,3.25-3.92 l1.57,1.57c-0.26-0.09-0.53-0.15-0.82-0.15c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5c1.38,0,2.5-1.12,2.5-2.5 c0-0.29-0.06-0.56-0.15-0.82l1.57,1.57C13.57,18.6,11.95,20,10,20z M18,15.17L10.83,8h1.75l1.28,1.4h2.54c0.88,0,1.6,0.72,1.6,1.6 V15.17z M20.4,5.6H22L19,11V7h-1V2h4L20.4,5.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoFood;
impl IconShape for MdNoFood {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.35,8.52L11,5h5V1h2v4h5l-1.38,13.79L11.35,8.52z M1,21v1c0,0.55,0.45,1,1,1h13c0.55,0,1-0.45,1-1v-1H1z M21.9,21.9 L2.1,2.1L0.69,3.51l5.7,5.7C3.28,9.87,1,11.99,1,15h11.17l2,2H1v2h15v-0.17l4.49,4.49L21.9,21.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoMeetingRoom;
impl IconShape for MdNoMeetingRoom {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHOklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP32sDv83/fYjn9d7AdwHfDbwP/7chntN7A9/Fs3038D7834V4tvcGvovn9d3A+/Bv897Ad/Hv8z7Ad/OfA3HFceBW4BjP33cD78O/znsD38V/jPcBvpv/eIhne2ngt4FjPH/fDbwPL5r3Br6L/1jvA3w3/7EQz+mlgd8GjvH8fTfwPrxwLw38FnCc/1i7wOsAf81/HMTzemngt4FjPH/fDbwPL9hvA6/Ff45d4HWAv+Y/BuL5e2ngt4FjPH/fDbwPz99x4LeBl+I/xy7w17xwu8BfAz8D/DUvGOIFe2ngr3jBvht4H56/48BvAy/Ff7+vBj6G5w/xgn018FG8cN8NvA/P33Hgt4GX4r/f1wAfzfNCPH8PBp7Oi+a7gffh+TsO/DbwUvz3ewhwK88J8fx9NvBZvOi+G3gfnr/jwG8DL8V/r88BPpvnhHj+fhp4K/51vht4H56/48BvAy/Ff5+fAd6a54R4/n4beC3+9b4beB+ev+PAbwMvxX+P3wFem+eEeP5+G3gt/m2+G3gfnr/jwG8DL8V/vd8BXpvnhHj+fht4Lf7tvht4H56/48BHc8VfA8eBBwPHgbcGHsR/jt8BXpvnhHj+fht4Lf59vht4H/71Xhp4a+Cz+I/1O8Br85wQz99vA6/Fv993A+/Dv82Dga8G3or/GL8DvDbPCfH8/TbwWvzH+G7gffi3+2zgs/j3+x3gtXlOiOfvt4HX4j/OdwPvw7/dewPfxb/P7wCvzXNCPH+/DbwW/7G+G3gf/u2+Gvgo/u1+B3htnhPi+ftt4LX4j/fdwPvwb/fbwGvxb/M7wGvznBDP328Dr8W/zccAnw0c4/n7buB9+Ld5beC3+Lf5HeC1eU6I5++3gdfiX+93gNcGXhr4beAYz993A+/Dv81vA6/Fv97vAK/Nc0I8f78NvBb/eu8DfDdXvDTw28Axnr/vBt6Hf733Br6Lf73fAV6b54R4/n4beC3+9U4AuzzbSwO/DRzj+ftu4H3413kw8HT+9X4HeG2eE+L5+23gtfjXeQbwYJ7XdwPvxQv23cD78K+zCxzjX+d3gNfmOSGev98GXot/nd8BXpvn9XTgwbxw3w28Dy+63wZei3+d3wFem+eEeP5+G3gt/nV+B3htnpd50Xw38D68aH4beC3+dX4HeG2eE+L5+23gtfjX+R3gtXle5kX33cD78C/7beC1+Nf5HeC1eU6I5++3gdfiX+evgZfhef028Fq86L4beB9euL8CXpp/nd8BXpvnhHj+fht4Lf71xPP6aOCr+Nf5buB9eMH+Gngp/nV+B3htnhPi+ftt4LX413sb4Kd5TseBW4Fj/Ot8N/A+PH/Hgd8GXooX3e8Ar81zQjx/vw28Fv963wO8N8/rvYHv4l/vu4H34fk7Dvw28FK8aH4HeG2eE+L5+23gtfjX2wVeBriV5/XewHfxr/fdwPvw/B0Hfht4Kf5lvwO8Ns8J8fz9NvBa/Nt8D/DePH+vDXw38CD+db4beB+ev+PAbwMvxQv3O8Br85wQz99vA6/Fv93rAL/NC/bSwFsDDwZ2gV3gr4HvBo7x/H038D48f8eB3wZeihfsd4DX5jkhnr/fBl6Lf7td4HWAv+Zf56WB3waO8fx9N/A+PH/Hgd8GXorn73eA1+Y5IZ6/3wZei3+fW4G3Af6af52XBn4bOMbz993A+/D8HQd+G3gpntfvAK/Nc0I8f78NvBb/frvA+wA/zb/OSwO/DRzj+ftu4H14/o4Dvw28FM/pd4DX5jkhnr/fBl6L/zi/DXwO8Nu8aF4b+CrgpXnBvht4H56/48BvAy/Fs/0O8No8J8Tz99vAa/Ef76+B3wZ+GrgE/DVXHAdeCnht4L2BB/Oi+W7gfXj+jgO/DbwUV/wO8No8J8Tz99vAa/G/w3cD78Pzdxz4beClgN8BXpvnhHj+fht4Lf73+G7gfXj+jgO/DewCr81zQjx/vw28Fv+7fDfwPjx/x4HPBj6a54R4/j4b+Cz+9/lu4H140SGev7cGfor/nb4beB9eNIgX7K+Bl+J/p+8G3od/GeIFezDw08BL8b/TdwPvwwuHeOGOAx8NvDfwIP73+W7gfXjBEP/7vTTw28Axnr/vBt6H5w/xf8NLA78NHOP5+27gfXheiP87Xhr4beAYz9/rAL/Nc0L83/LSwG8Dx3hO7wN8N88L8X/PSwO/DRzjivcBvpvnD/F/00sDvw18NPDdvGCI/7uOA7u8cIj/3/hHZ3X/Qc68y8EAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoPhotography;
impl IconShape for MdNoPhotography {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.94,8.12L7.48,4.66L9,3h6l1.83,2H20c1.1,0,2,0.9,2,2v12c0,0.05-0.01,0.1-0.02,0.16l-5.1-5.1C16.96,13.71,17,13.36,17,13 c0-2.76-2.24-5-5-5C11.64,8,11.29,8.04,10.94,8.12z M20.49,23.31L18.17,21H4c-1.1,0-2-0.9-2-2V7c0-0.59,0.27-1.12,0.68-1.49l-2-2 L2.1,2.1l19.8,19.8L20.49,23.31z M14.49,17.32l-1.5-1.5C12.67,15.92,12.35,16,12,16c-1.66,0-3-1.34-3-3c0-0.35,0.08-0.67,0.19-0.98 l-1.5-1.5C7.25,11.24,7,12.09,7,13c0,2.76,2.24,5,5,5C12.91,18,13.76,17.75,14.49,17.32z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoStroller;
impl IconShape for MdNoStroller {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6,18c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S4.9,18,6,18z M18.65,3c-1.66,0-2.54,1.27-3.18,2.03l-3.5,4.11L17,14.17v-7.9 C17.58,5.59,17.97,5,18.65,5C19.42,5,20,5.66,20,6.48V7h2V6.48C22,4.56,20.52,3,18.65,3z M10.67,10.67L2.81,2.81L1.39,4.22 l7.97,7.97L6.7,15.31c-0.55,0.65-0.09,1.65,0.76,1.65h6.66l1.17,1.17C14.54,18.42,14,19.14,14,20c0,1.1,0.9,2,2,2 c0.86,0,1.58-0.54,1.87-1.3l1.91,1.91l1.41-1.41l-4.8-4.8L10.67,10.67z M13.47,5.03c0.27-0.32,0.58-0.72,0.98-1.09 c-2.46-1.19-5.32-1.22-7.81-0.13l4.25,4.25L13.47,5.03z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPool;
impl IconShape for MdPool {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 21c-1.11 0-1.73-.37-2.18-.64-.37-.22-.6-.36-1.15-.36-.56 0-.78.13-1.15.36-.46.27-1.07.64-2.18.64s-1.73-.37-2.18-.64c-.37-.22-.6-.36-1.15-.36-.56 0-.78.13-1.15.36-.46.27-1.08.64-2.19.64-1.11 0-1.73-.37-2.18-.64-.37-.23-.6-.36-1.15-.36s-.78.13-1.15.36c-.46.27-1.08.64-2.19.64v-2c.56 0 .78-.13 1.15-.36.46-.27 1.08-.64 2.19-.64s1.73.37 2.18.64c.37.23.59.36 1.15.36.56 0 .78-.13 1.15-.36.46-.27 1.08-.64 2.19-.64 1.11 0 1.73.37 2.18.64.37.22.6.36 1.15.36s.78-.13 1.15-.36c.45-.27 1.07-.64 2.18-.64s1.73.37 2.18.64c.37.23.59.36 1.15.36v2zm0-4.5c-1.11 0-1.73-.37-2.18-.64-.37-.22-.6-.36-1.15-.36-.56 0-.78.13-1.15.36-.45.27-1.07.64-2.18.64s-1.73-.37-2.18-.64c-.37-.22-.6-.36-1.15-.36-.56 0-.78.13-1.15.36-.45.27-1.07.64-2.18.64s-1.73-.37-2.18-.64c-.37-.22-.6-.36-1.15-.36s-.78.13-1.15.36c-.47.27-1.09.64-2.2.64v-2c.56 0 .78-.13 1.15-.36.45-.27 1.07-.64 2.18-.64s1.73.37 2.18.64c.37.22.6.36 1.15.36.56 0 .78-.13 1.15-.36.45-.27 1.07-.64 2.18-.64s1.73.37 2.18.64c.37.22.6.36 1.15.36s.78-.13 1.15-.36c.45-.27 1.07-.64 2.18-.64s1.73.37 2.18.64c.37.22.6.36 1.15.36v2zM8.67 12c.56 0 .78-.13 1.15-.36.46-.27 1.08-.64 2.19-.64 1.11 0 1.73.37 2.18.64.37.22.6.36 1.15.36s.78-.13 1.15-.36c.12-.07.26-.15.41-.23L10.48 5C8.93 3.45 7.5 2.99 5 3v2.5c1.82-.01 2.89.39 4 1.5l1 1-3.25 3.25c.31.12.56.27.77.39.37.23.59.36 1.15.36z",
            }
            circle {
                cx: "16.5",
                cy: "5.5",
                r: "2.5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG1klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/zp/A+zynI4DL8W/zk8D7wPs8m+D+Ld7aeCngAfzovkb4L2Bv+aFe2ngu4GX4kVzK/A2wF/zr4f4t3lp4LeA47zofgd4bV40vw28Fi+6XeB1gL/mXwfxr/fSwG8Bx3n+LnHFMZ7T7wCvzYvmt4HX4jld4opjPH+7wOsAf82LDvGv89LAbwHHef5+B3hv4LuB1+I5/Q7w2rxofht4LZ7T7wDvDXw38Fo8f7vA6wB/zYsG8aI7DvwV8GCev+8B3psrfht4LZ7T7wCvzYvmt4HX4jn9DvDaXPHdwHvx/N0KvAywy78M8aL7KeCtef7eB/hunu23gdfiOf0O8Nq8aH4beC2e0+8Ar82zvTfwXTx/Pw28Df8yxIvmvYHv4vl7H+C7eU6/DbwWz+l3gNfmRfPbwGvxnH4HeG2e03sD38Xz9z7Ad/PCIf5lx4GnA8d5Xl8DfDTP67eB1+I5/Q7w2rxofht4LZ7T7wCvzfP6buC9eF67wEOAXV4wxL/ss4HP4nn9DvDaPH+/DbwWz+l3gNfmRfPbwGvxnH4HeG2ev98GXovn9TnAZ/OCIV6448DTgeM8p0vASwO38vz9NvBaPKffAV6bF81vA6/Fc/od4LV5/h4M/DVwjOe0CzwE2OX5Q7xwnw18Fs/rc4DP5gX7beC1eE6/A7w2L5rfBl6L5/Q7wGvzgn028Fk8r88BPpvnD/HCPR14MM/pEvBgYJcX7LeB1+I5/Q7w2rxofht4LZ7T7wCvzQt2HLgVOMZzuhV4CM8f4gV7a+CneF6fA3w2L9xvA6/Fc/od4LV50fw28Fo8p98BXpsX7rOBz+J5vQ7w2zwvxAv21cBH8bxOALu8cL8NvBbP6XeA1+ZF89vAa/Gcfgd4bV6448BFntfXAB/N80K8YH8FvDTP6WeAt+Zf9tvAa/Gcfgd4bV40vw28Fs/pd4DX5l/208Bb8Zz+GngZnhfi+Xsw8HSe1/sA382/7LeB1+I5/Q7w2rxofht4LZ7T7wCvzb/so4Gv4nmdAHZ5Tojn77WB3+J5vQzw1/zLfht4LZ7T7wCvzYvmt4HX4jn9DvDa/MteGvgrntfrAL/Nc0I8fx8NfBXPS7xofht4LZ7T7wCvzYvmt4HX4jn9DvDavGjM8/oY4Kt5Tojn77OBz+I5/Q7w2rxofht4LZ7T7wCvzYvmt4HX4jn9DvDavGh+G3gtntPnAJ/Nc0I8f58NfBbP6XeA1+ZF89vAa/Gcfgd4bV40vw28Fs/pd4DX5kXz28Br8Zw+B/hsnhPi+ftt4LV4Tr8DvDYvmt8GXovn9DvAa/Oi+W3gtXhOvwO8Ni+a3wZei+f0M8Bb85wQz993A+/F/y3fA7w3zwnx/H028Fn83/I5wGfznBDP32cDn8X/LZ8DfDbPCfH8vTfwXfzf8j7Ad/OcEM/fg4Gn83/LQ4BbeU6IF+xW4EH83/AM4ME8L8QL9t3Ae/F/w/cA783zQrxgbw38FP83vA3w0zwvxAt3K/Ag/nd7BvBgnj/EC/fRwFfxv9v7AN/N84d44Y4DtwLH+N/pEvBgYJfnD/Eve2vgp/jf6W2An+YFQ7xofht4Lf53+RngrXnhEC+aBwN/DRzjf4dLwIOBXV44xIvupYHfBo7xP9sl4LWBv+ZfhvjXeW/gu/if7W2An+ZFg/jXe2/gq4Fj/M9yCfho4Lt50SH+bV4a+G3gGP8zXAJeG/hr/nUQ/3YvDXw38FL89/od4KOBv+ZfD/Hv99HAZwPH+K91Cfhs4Kv5t0P8x3gw8NnAe/Ff43uAzwZu5d8H8R/rwcBnA+/Ff47vAT4buJX/GIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/xxfDbw0/7H+Gvho/mMh/n9D/P+G+P8N8f8b4v83/hFf//9BDhuSeAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRiceBowl;
impl IconShape for MdRiceBowl {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,12L22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,3.69,2.47,6.86,6,8.25V22h8v-1.75C19.53,18.86,22,15.69,22,12z M20,12h-4 V5.08C18.39,6.47,20,9.05,20,12z M14,4.26V12h-4V4.26C10.64,4.1,11.31,4,12,4S13.36,4.1,14,4.26z M4,12c0-2.95,1.61-5.53,4-6.92V12 H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv95x4KeA1+Y5/TbwNsAuz99x4KV4wS4Bf82/DuK/1nHgt4CX5vn7a+B1gF2e12sDv8UL9jvAa/Ovg/iv89LAdwEvzQv318D7AH/Nc3pt4Ld4wX4HeG3+dRD/NV4a+C3gOC+aXeB1gL/m2V4b+C1esN8BXpt/HcR/vtcGfgo4zr/OLvA2wG9zxWsDv8UL9jvAa/Ovg/jP9d7Ad/GCfQ9XvBcv2PsA3w28NvBbvGC/A7w2/zqI/zzvDXwXL9jXAB/NFV8NfBQv2PsAtwK/xQv2O8Br86+D+M/xXcB784K9D/DdPKf3Br6LF+yvgZfmBfsd4LX510H8x/su4L15wd4H+G6ev/cGvot/m98BXpt/HcR/nOPATwGvzfN3CXht4K954V4a+G3gGP86vwO8Nv86iP8Yx4HfAl6a5+8S8NrAX/OieWngt4FjvOh+B3ht/nUQ/34vDXwX8NI8f38DvDVwK/86DwZ+GngpXjS/A7w2/zqIf5+XBn4LOM7z9zfAawO7/NscB34beCn+Zb8DvDb/Ooh/u9cGfgo4zvP3M8B7A7v8+xwHfhp4LV643wFem38dxL/NewPfxQv2PcB78x/ru4H34gX7HeC1+ddB/Ou9N/BdvGDfA7w3/zm+G3gvXrD3Ab6bFx3iX+e7gPfmBXsf4Lv5z/XewHfxgn038D68aBAvuu8C3psX7H2A7+a/xnsD38UL9t3A+/AvQ/zLjgM/Bbw2z98l4K2B3+a/1msDPw0c4/n7beBtgF1eMMQLdxz4LeClef4uAa8N/DX/PV4a+G3gGM/fXwOvA+zy/CFesJcGvgt4aZ6/vwHeG/hr/nu9NPDdwEvx/P018D7AX/O8EM/fSwO/BRzn+fsb4LWBXf5nOA78NvBSPH+7wOsAf81zQvz/hvj/DfH/G+L/N8T/b4j/fL/Fv8/r8J8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zPP6G2CX53QceCmel/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gf77X59/lt/vMg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CgcuOQVbFDU4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRoofing;
impl IconShape for MdRoofing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13,18h-2v-2h2V18z M15,14H9v6h6V14L15,14z M19,9.3L19,9.3V4h-3v2.6v0L12,3L2,12h3l7-6.31L19,12h3L19,9.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRoomPreferences;
impl IconShape for MdRoomPreferences {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,11.26V6h3v4h2V4h-5V3H5v16H3v2h9.26C11.47,19.87,11,18.49,11,17C11,14.62,12.19,12.53,14,11.26z M10,11h2v2h-2V11z M21.69,16.37l1.14-1l-1-1.73l-1.45,0.49c-0.32-0.27-0.68-0.48-1.08-0.63L19,12h-2l-0.3,1.49c-0.4,0.15-0.76,0.36-1.08,0.63 l-1.45-0.49l-1,1.73l1.14,1c-0.08,0.5-0.08,0.76,0,1.26l-1.14,1l1,1.73l1.45-0.49c0.32,0.27,0.68,0.48,1.08,0.63L17,22h2l0.3-1.49 c0.4-0.15,0.76-0.36,1.08-0.63l1.45,0.49l1-1.73l-1.14-1C21.77,17.13,21.77,16.87,21.69,16.37z M18,19c-1.1,0-2-0.9-2-2s0.9-2,2-2 s2,0.9,2,2S19.1,19,18,19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/z2OA+8FvDRX/DXwPcAu/7UQ//XeG/gq4DjPaRf4GOC7+a+D+K/12sBv8cK9DvDb/NdA/Nd6OvBgXrhbgYfwXwPxX+elgb/iRfMywF/znw/xX+e1gd/iRfM6wG/znw/xX+e1gd/iRfM6wG/znw/xX+c4cJEXzQlgl/98iP9cLw28FvDWwGvzr/PbwE8DvwP8Nf85EP85Xhv4LOC1+Y/x28DnAL/NfyzEf6yXBr4KeG3+c/w28DHAX/MfA/Ef57OAz+a/xmcDn8O/H+Lf78HATwEvzX+tvwbeBriVfzvEv89LA78FHOe/xy7wOsBf82+D+Ld7aeC3gOP899oFXgf4a/71EP82Lw38FnCc/xl2gdcB/pp/HcS/3nHgt4CX5n+WvwZeB9jlRYf41/su4L35n+m7gffhRYf413lr4Kf4n+1tgJ/mRYP413k68GD+Z7sVeAgvGsSL7r2B7+J/h/cBvpt/GeJFcxz4K+DB/O9wK/AQ/mWIF817A9/F/y7vA3w3LxziRfPTwFvxv8vPAG/NC4f4lx0HLvK/0wlglxcM8S97b+C7+N/pbYCf5gVD/Ms+G/gs/nf6HOCzecEQ/7KfBt6K/51+BnhrXjDEv+yvgJfmf6e/Bl6GFwzxL/tt/nd7bV4wxP9viP/fEM/fVwMvxf8tfwN8NM8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8fTXw0vzf8tfAR/OcEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CIPMUkG3pJR+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRoomService;
impl IconShape for MdRoomService {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 17h20v2H2zm11.84-9.21c.1-.24.16-.51.16-.79 0-1.1-.9-2-2-2s-2 .9-2 2c0 .28.06.55.16.79C6.25 8.6 3.27 11.93 3 16h18c-.27-4.07-3.25-7.4-7.16-8.21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE5klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O7038N38+yH+d/pt4OnA+/Dvg/jf6beB1wK+G3gf/u0Q/zv9NvBaXPHdwPvwb4P43+m3gdfi2b4beB/+9RD/+V6L/3hfDbw0z+m7gffhXwfxn8/81/lu4H140SH+85n/Wt8NvA8vGsR/PvNf6xLw0sCt/MsQ//nMf51LwGsDf82LBvGfz/zXuAS8NvDXvOgQ/zv9NvBaPNsl4LWBv+ZfB/G/028Dr8UVl4DXBv6afz3Ef5zjwC7/NX4beC3gEvDawF/zb4P4j/HSwGcDb81/jd8GXhp4beCv+bdD/Pu9NPBbwN8Ar81/jZ8GPhv4a/59EP8+Lw38FnAc+B3gtfmvcRzY5d8P8W/30sBvAce54neA1+Z/F8S/zUsDvwUc59l+B3ht/ndB/Ou9NPBbwHH+5/hr4KuB7+FfB/Gv89LAbwHH+Z/pu4H34UWHeNEdB54OHOd/ts8BPpsXDeJf572B7+J/tl3gBC8axL/eewPfxf9srwP8Nv8yxL/NewPfxXP6G+CjedF8NfBSvGAfA/w1L5rf4nm9DvDb/MsQ/3bvDXwXz/Y7wGvzovlt4LV4wV4H+G1eNOZ5vQ7w2/zLEP8+7w18F1f8DvDavGh+G3gtXrDXAX6bF415Xq8D/Db/MsS/33sD3wX8DvDavGh+G3gtXrDXAX6bF415Xq8D/Db/MsR/jPcG3ht4bV40vw28Fi/Y6wC/zYvGPK/XAX6bfxniP85LA3/Ni+a3gdfiBXsd4Ld50Zjn9TrAb/MvQ/z3+G3gtXjBXgf4bV405nm9DvDb/MsQ/z1+G3gtXrDXAX6bF415Xq8D/Db/MsSL7qWBr+I/xksDx3nB/hrY5UXz2jyv1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4e/BnZ5Tn8DfDTPCfGie2ngr/jf63eA1+Y5If51bgUexP9OvwO8Ns8J8a/z2sBv8W/3N8B7A38NvDfw1cAx/mv8DvDaPCfEv957A18NHONf5xnASwO7PNt7A9/Ff43fAV6b54T4tzkOvDfw0sCDeV4vDRzjOf0O8No8L/O8/gbY5T/WXwMfzXNC/Of4bOCzeE5/DbwMz+mlgb/ieb0O8Nv850P85/ho4Kt4Xp8NfA5XHAd+C3hpntdDgFv5z4f4z/Fg4Ok8f38N7AIvDRznef0N8NL810D85/lu4L3413sb4Kf5r4H4z3McuBU4xovue4D35r8O4j/XSwO/DRzjX/YzwHsDu/zXQfznOw58NfBePH+XgK8GPpv/eoj/Og8G3ho4Drw08NfAXwO/Dezy3wPx/xvi/zfE/2+I/98Q/7/xj7DKqUGrjzjIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRvHookup;
impl IconShape for MdRvHookup {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 11h4v3h-4z",
            }
            path {
                d: "M20 17v-6c0-1.1-.9-2-2-2H7V7l-3 3 3 3v-2h4v3H4v3c0 1.1.9 2 2 2h2c0 1.66 1.34 3 3 3s3-1.34 3-3h8v-2h-2zm-9 3c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm7-6h-4v-3h4v3zM17 2v2H9v2h8v2l3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSmokeFree;
impl IconShape for MdSmokeFree {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6l6.99 7H2v3h9.99l7 7 1.26-1.25-17-17zm18.5 7H22v3h-1.5zM18 13h1.5v3H18zm.85-8.12c.62-.61 1-1.45 1-2.38h-1.5c0 1.02-.83 1.85-1.85 1.85v1.5c2.24 0 4 1.83 4 4.07V12H22V9.92c0-2.23-1.28-4.15-3.15-5.04zM14.5 8.7h1.53c1.05 0 1.97.74 1.97 2.05V12h1.5v-1.59c0-1.8-1.6-3.16-3.47-3.16H14.5c-1.02 0-1.85-.98-1.85-2s.83-1.75 1.85-1.75V2c-1.85 0-3.35 1.5-3.35 3.35s1.5 3.35 3.35 3.35zm2.5 7.23V13h-2.93z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+3wXcCvw18DP8O+D+N/HPNsu8NHA9/Bvg/jfxzyvnwbeB9jlXwfxv495/n4beB3+dRD/+7w38NrAWwPHeE6fA3w2LzrE/17Hgd8GXopn2wVeBriVFw3if7fjwF8DD+LZPgb4al40iP/9Phr4Kp7td4DX5kWD+J/lpYG3Al4b+Bvgo/mXvTTwVzzbXwMvw4sG8T/DSwNfBbw2z/Y7wGvzLzsOXOTZLgHHedEg/vu9N/BVwHGe0+8Ar82/7LWB3+LZfgd4bV40iP9eLw38Fc/f7wCvzb/su4H34tm+B3hvXjSI/z7HgacDx3lO3wP8NHAr8Ne8cO8NfBfP6W2An+ZFg/jv89nAZ/Fsl4C3Bn6bf9lx4KOAz+Y5/Q7w2rzoEP99/gp4aZ7tY4Cv5l/2V8BL8/y9DPDXvOgQ/z2OAxd5tkvAcV405vn7GuCj+ddB/Pd4beC3eLbfAV6bF415wX4a+BjgVl40iP8erw38Fs/2O8Br86IxL9wu8DbAb/MvQ/z3eDDwdJ7TCWCXf9lrA8eB1wbeGngQz9/7AN/NC4f477MLHOPZPgf4bP71Xhv4bOC1eF6vA/w2Lxjiv893A+/Fs+0CrwP8Nf82nw18Fs9pF3gIsMvzh/jv82Dg6TynXeCzga/h3+a9ge/iOX0P8N48f4j/Xh8NfBXP32/z/O0CPw38DLDL8/pq4KN4Tg8BbuV5If77fTfwXvzbfDfwMcAuz+lW4EE829cAH83zQvzP8N7AVwPH+Ne7FXgb4K95tvcGvotn2wVO8LwQ/3McB94beG/gpfjX2QVeB/hrnm0XOMazvQ7w2zwnxP8uDwZeG3hv4LV4Tr8NvA7P9tPAW/FsHwN8Nc8J8b/XdwPvxXN6G+CnueKzgc/i2T4H+GyeE+J/t98GXotn+xngrbnio4Gv4tm+B3hvnhPif7f3Br6LZ7sVeAhXvDbwWzzb7wCvzXNC/O92HLjIcxJXvDbwWzzb7wCvzXNC/O9nnpO44rWB3+LZfgd4bZ4T4n8/85zEFa8N/BbP9jvAa/OcEP/7meckrnht4Ld4tt8BXpvnhPjfzzwnccVrA7/Fs/0O8No8J8T/b4j/3xDP30sDx/if62+AXa54LZ7T73DFceCleLZLwF/znBDP328Dr8X/XK8D/DZXmOckrnht4Ld4tt8BXpvnhHj+fht4Lf7neh3gt7nCPCdxxWsDv8Wz/Q7w2jwnxPP328Br8T/X6wC/zRXmOYkrXhv4LZ7td4DX5jkhnr/fBl6L/7leB/htrjDPSVzx2sBv8Wy/A7w2zwnx/P028Fr8z/U6wG9zhXlO4orXBn6LZ/sd4LV5Tojn77eB1+J/rtcBfpsrzHMSV7w28Fs82+8Ar81zQjx/vw28Fv9zvQ7w21xhnpO44rWB3+LZfgd4bZ4T4vn7auCl+Z/ro4G/5orf5jm9Nle8NPDVPNtfAx/Nc0L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COAErZBAzd60wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSmokingRooms;
impl IconShape for MdSmokingRooms {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 16h15v3H2zm18.5 0H22v3h-1.5zM18 16h1.5v3H18zm.85-8.27c.62-.61 1-1.45 1-2.38C19.85 3.5 18.35 2 16.5 2v1.5c1.02 0 1.85.83 1.85 1.85S17.52 7.2 16.5 7.2v1.5c2.24 0 4 1.83 4 4.07V15H22v-2.24c0-2.22-1.28-4.14-3.15-5.03zm-2.82 2.47H14.5c-1.02 0-1.85-.98-1.85-2s.83-1.75 1.85-1.75v-1.5c-1.85 0-3.35 1.5-3.35 3.35s1.5 3.35 3.35 3.35h1.53c1.05 0 1.97.74 1.97 2.05V15h1.5v-1.64c0-1.81-1.6-3.16-3.47-3.16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/l9fiit/hPwbif4f3Br4KOM4Vu8DHAN/Nvw/if77XBn6L5+91gN/m3w7xP99vA6/F8/c7wGvzb4f4n8+8cOLfDvE/318DL8Xz9zfAS/Nvh/if77OBz+L5+xzgs/m3Q/zv8NvAa/Gcfgd4bf59EP97vDfw0lzx18B387xeGjjGFb/Dvwzxf8NrA98FPJhn2wW+GvgcXjDE/yzHgZfiikvAX/Mve2/gu3jBvht4H54/xH+u48BrAz/NC3cc+CrgvXlOtwKfDXwPz99x4OnAcV649wG+m+eF+M9zHPgt4KWB9wG+m+fvOPBbwEvzgn038D48r48Gvorn9DvASwPHeLbfAV6b54X4z3Ec+C3gpXm29wG+m+f1W8Br8y97H+C7eU7fDbwXz/Y5wGcDDwaeznMSzwvxH++lgd8CjvO83gb4aZ7tpYG/4jk9A7gVeGngGM92K/AQntN3A+/Fs30O8NnAg4Gn85zE80L8x3pp4LeA4zx/fw28DM/22cBn8Wy/A7w2V7w08NvAMZ7tZYC/5tk+GvgqntNvAy8NHOfZfgd4bZ4X4j/OSwO/BRznhRPP9tPAW/Fs7wN8N8/228Br8WyvA/w2z3YcuBU4xgv3PsB387wQ/zFeGvgt4Dgv3M8Ab82zfTbwWTzb1wAfzRXHgb8CHsyzvQ7w2zyn9wa+ixfse4D35vlD/Pu9N/BVwHH+ZQ8BbuXZ3hv4Lp7TVwO3Am8NvDbP6QSwy/N6beC7gQfxbJeArwY+mxcM8e/z3sB38aL5GuCjeU7Hgb8GHsS/7HuA9+aFe2ngOFf8Nv8yxL/dewPfxYvmEvBgYJfn9drAb/HC/Q3w2sAu/7EQ/zbvDXwXL7qPAb6aF+y1ge8GHsTz+h3grYFd/uMh/vU+C/hsXnTPAB7Mv+w48NbAg4GXBv4a+Gngr/nPg/jX+S7gvfnXeR3gt/mfCfGi+y7gvfnXuRV4b/7r/A7/OogXzXcB783/fLcCbwP8NS8axL/su4D35n+PW4GH8KJBvGDHgd8CXpr/fV4H+G3+ZYjn7zjwW8BL87/T6wC/zb8M8byOA78FvDT/Oz0DeDAvGsTz+i3gtfnf6RnAWwN/zYsG8ZzeG/gu/mu9Dv9xfpt/HcRz+ivgpfmvJf77IJ7twcDT+a8n/vsgnu21gd/i/56fBj4H+GueF+LZXhv4Lf5v2gUeAuzynBDP9trAb/F/1+cAn81zQjzbawO/xf9dXwN8NM8J8WyvDfwW/3d9DPDVPCfEs7028Fv83/Q3wEvzvBDP9trAb/Ff73f4z3Mr8NfAV/P8IZ7ttYHf4r+e+O+DeLbXBn6L/3rivw/i2V4b+C3+6/02//F+G/ge4FZeOMSzvTbwW/zfsQu8DvDXvGCIZ3tt4Lf4v+VngLfmBUM822sDv8X/PeIFQzzbawO/xf8tfwO8NC8Y4tleG/gt/m95G+CnecEQz/bawG/xf8PfAJ8N/DQvHOLZXhv4Lf7rif8+iGd7beC3+K8n/vsgnu21gd/iv95r8+/zN8Au/zaI52T+d/oY4Kv510M8p+8G3ov/nV4H+G3+dRDP6cHA0/nf6WuAj+ZfB/G83hv4Lv73+Rrgo/nXQTx/7w18F/+7vA7w2/zrIF6wBwOfDbwX//N9DPDV/OshXjSvzf9cfw3s8m+D+P8N8f8b/wha87tBEuUT2AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSoap;
impl IconShape for MdSoap {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.12,5l-7.18,6.79C1.34,12.35,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3h6.25H12h5.75c0.69,0,1.25-0.56,1.25-1.25 s-0.56-1.25-1.25-1.25H12v-1h7.75c0.69,0,1.25-0.56,1.25-1.25S20.44,17,19.75,17H12v-1h8.75c0.69,0,1.25-0.56,1.25-1.25 s-0.56-1.25-1.25-1.25H12v-1h6.75c0.69,0,1.25-0.56,1.25-1.25S19.44,10,18.75,10H8.86c0.64-1.11,1.48-2.58,1.49-2.61 c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7C10.22,6.12,9.12,5,9.12,5L9.12,5z M14,6.25c0.41,0,0.75,0.34,0.75,0.75 S14.41,7.75,14,7.75S13.25,7.41,13.25,7S13.59,6.25,14,6.25 M14,4.75c-1.24,0-2.25,1.01-2.25,2.25S12.76,9.25,14,9.25 S16.25,8.24,16.25,7S15.24,4.75,14,4.75L14,4.75z M19.75,5.5c0.28,0,0.5,0.22,0.5,0.5s-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5 S19.47,5.5,19.75,5.5 M19.75,4c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S20.85,4,19.75,4L19.75,4z M16.5,1C15.67,1,15,1.67,15,2.5 S15.67,4,16.5,4C17.33,4,18,3.33,18,2.5S17.33,1,16.5,1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vY4Dbw18N/89EP99jgO/Bbw08DLAX/NfD/Hf56+Al+aKXeB1gL/mvxbiv8d3Ae/Nc/pr4HWAXf7rIP7rvTfwXTx/3w28D/91EP+1jgNPB47zgr0N8NP810D81/op4K154W4FXgbY5T8f4r/OawO/xYvmc4DP5j8f4r/ObwGvzYtmF3gIsMt/LsR/jdcGfot/nc8BPpv/XIj/Gr8FvDb/OrvACf5zIf7zPRh4Ov82bwP8NP95EP/5vhr4KP5tfgZ4a/7zIP7zPR14MP924j8P4j/XSwN/xb/P2wA/zX8OxH+ujwa+in+fzwE+m/8ciP9cPw28Ff8+vwO8Nv85EP+5ng48mH+fXeAE/zkQ/7nMfwzxnwPxn+elgb/iP8brAL/NfzzEf56XBv6K/xgvA/w1//EQz99LA8e44hnArfzbPBj4buC1+Lf5HeC9gVv5t3kw8CCuuAT8Nc8J8fz9NPBWPKdbgb8G/hr4beB3eNG9NfDdwDFeNJeA9wZ+mhfdawGvDbw08NLAg3lO3wO8N88J8fx9NvBZvHC7wE8DPw38DP+y48BPA6/FC/c7wHsDt/IveyvgrYG3Bo7zwn0O8Nk8J8Tz99nAZ/GiuxX4bOB7+Jd9NfBRPH9fA3w0/7L3Aj4beDAvus8BPpvnhHj+Phv4LP71bgW+GvgaXrj3Br6L5/Q+wHfzwn0W8N7Ag/nX+xjgq3lOiOfvtYHf4t/ur4H3Af6aF+ytge/mivcGfpoX7KWB7wJemn+71wF+m+eEeP4eDDydf7+vBj4H2OX5e2mu+Guev+PAZwEfzb/fQ4BbeU6IF2wXOMa/318DrwPs8q9zHPgt4KX597sEHOd5IV6wnwbeiv8Yu8DrAH/Ni+algd8CjvMf42eAt+Z5IV6wjwa+iv84u8DrAH/NC/fSwG8Bx/mP8zHAV/O8EC/YSwN/xX+sXeB1gL/m+Xtp4LeA4/zHeghwK88L8cL9NfBS/MfaBV4H+Gue00sDvwUc5z/W3wAvzfOHeOE+Gvgq/uPtAg8BdrniOPB04Dj/8T4G+GqeP8QLdxy4yH+O3wZehyt+C3ht/uNdAh4M7PL8If5l3w28F/85PocrPov/HN8DvDcvGOJf9mDg6fzv9BDgVl4wxIvmq4GP4n+XrwE+mhcO8aI5DtwKHON/h0vAg4FdXjjEi+6jga/if4e3AX6afxniX+e3gdfif7bfAV6bFw3iX+c48NfAg/if6RnASwO7vGgQ/3ovDfw2cIz/WS4Brw38NS86xL/NewPfxf8s7wN8N/86iH+79wa+i/8Z3gf4bv71EP8+7w18F/+93gf4bv5tEP9+bw18N3CM/1qXgPcGfpp/O8R/jJcGvht4Kf5r/A3w3sBf8++D+I9zHPhs4KP4z/U1wGcDu/z7If7jvTbw1cBL8R/rb4CPBn6b/ziI/zzvDXw28CD+fZ4BfDbw3fzHQ/zne2/go4GX4l/nb4CvBr6b/zyI/zovDbw38NbAg3j+/gb4beC7gb/mPx/iv8eDgdcGXporfhv4a+BW/msh/n9D/P+G+P8N8f8b4v83/hGkq7ZBL28AYwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpa;
impl IconShape for MdSpa {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.55 12c-1.07-.71-2.25-1.27-3.53-1.61 1.28.34 2.46.9 3.53 1.61zm10.43-1.61c-1.29.34-2.49.91-3.57 1.64 1.08-.73 2.28-1.3 3.57-1.64z",
            }
            path {
                d: "M15.49 9.63c-.18-2.79-1.31-5.51-3.43-7.63-2.14 2.14-3.32 4.86-3.55 7.63 1.28.68 2.46 1.56 3.49 2.63 1.03-1.06 2.21-1.94 3.49-2.63zm-6.5 2.65c-.14-.1-.3-.19-.45-.29.15.11.31.19.45.29zm6.42-.25c-.13.09-.27.16-.4.26.13-.1.27-.17.4-.26zM12 15.45C9.85 12.17 6.18 10 2 10c0 5.32 3.36 9.82 8.03 11.49.63.23 1.29.4 1.97.51.68-.12 1.33-.29 1.97-.51C18.64 19.82 22 15.32 22 10c-4.18 0-7.85 2.17-10 5.45z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/uV4aeC3gOM/pt4Hf4b8f4j/eceCjgPcGHswLtgv8NPA5wK3890D8x3pv4KuA4/zrfDbwOfzXQ/zH+S7gvfm3uxXYBV6aK24FbgV+GvgZ4Fb+4yH+Y3wX8N785/pu4GOAXf7jIP79Phr4Kl64ZwC3csVLA8f4t9kFXgf4a/5jIP59Hgw8nefvEvDVwFcDuzyn9wY+G3gQ/zYvA/w1/36If7vjwHcBb83z+hvgvYG/5gU7Dnw18F786+0CDwF2+fdB/OsdBz4L+Giev0vAg4FdXjQfDXwV8DfATwO7wK3ASwPvDTyI5+97gPfm3wfxr/PSwG8Bx3nB3gb4af51jgO7PH8fDXwVz99DgFv5t0O86F4a+C3gOC/Y7wCvzX+8jwa+iuf1McBX82+HeNEcB54OHOeFex/gu/nPcSvwIJ7T7wCvzb8d4kXz3cB78fw9A/hq4KeBW/nP89nAZ/GcbgUewr8d4l/2YODpPH8fA3w1/36vxb/srYGP5nmJfzvEv+yzgc/ieX0M8NX82xwH3gv4aODB/PuIfzvEv+y3gdfiOT0DeDD/Nu8NfBVwnH+/vwFemn87xL/MPK/PAT6bf73vAt6b/zhfA3w0/3aIf5l5Xu8DfDf/Ot8FvDf/sV4G+Gv+7RD/MvO8Pgf4bF503wW8N/+xfgd4bf59EP+yvwZeiuf0M8Bb86L5LuC9+a+xC/w08DnArfzLEP+yrwY+iuf1MsBf88J9F/De/Pd4H+C7eeEQ/7LXBn6L53Ur8DbAX/O8jgNfBbw3/70eAtzKC4Z40fw28Fo8r13gq4HvAW4FXhp4KeCzgQfz3+97gPfmBUO8aF4a+Cv+99kFTvCCIV507w18F/8zfAzw1zynlwa+iuclXjDEv85nA5/Ff7/XAX6b5/TawG/xvMQLhvjXe2vgq4EH8d/ndYDf5jm9NvBbPC/xgiH+7T4aeG/gpfiv9zrAb/OcXhv4LZ6XeMEQ/37mv97rAL/Nc3pt4Ld4XuIFQ/z7mf96rwP8Ns/ptYHf4nmJFwzx72f+670O8Ns8p9cGfovnJV4wxL+f+a/3OsBv85xeG/gtnpd4wRD/fua/3usAv81zem3gt3he4gVD/PuZ/3qvA/w2z+m1gd/ieYkXDPHvZ/7rvQ7w2zyn1wZ+i+clXjDEv5/5r/c6wG/znF4b+C2el3jBEP9+5r/e6wC/zXN6beC3eF7iBUP8+5n/eq8D/DbP6bWB3+J5iRcM8e9n/uu9DvDbPKfXBn6L5yVeMMS/n/mv9zrAb/OcXhv4LZ6XeMEQ/37mv95fA7s8p+PAS/O8xAuG+Pcz/3M9A3gwLxji38/8z/U5wGfzgiH+/cz/TH8DvDawywuG+Pcz//P8DfDWwK28cIh/P/Nf72+AXZ7XbwN/Dfw0LxrEv5/5r/c6wG/z74f49zP/9V4H+G3+/RD/fua/3usAv82/H+Lfz/zXex3gt/n3Q/z7mf96rwP8Nv9+iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjsBG4QWJ9xToAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsBar;
impl IconShape for MdSportsBar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,9h-1.56C17.79,8.41,18,7.73,18,7c0-2.21-1.79-4-4-4c-0.34,0-0.66,0.05-0.98,0.13C12.2,2.45,11.16,2.02,10,2.02 c-1.89,0-3.51,1.11-4.27,2.71C4.15,5.26,3,6.74,3,8.5c0,1.86,1.28,3.41,3,3.86L6,21h11v-2h2c1.1,0,2-0.9,2-2v-6C21,9.9,20.1,9,19,9z M7,10.5c-1.1,0-2-0.9-2-2c0-0.85,0.55-1.6,1.37-1.88l0.8-0.27l0.36-0.76C8,4.62,8.94,4.02,10,4.02c0.79,0,1.39,0.35,1.74,0.65 l0.78,0.65c0,0,0.64-0.32,1.47-0.32c1.1,0,2,0.9,2,2c0,0-3,0-3,0C9.67,7,9.15,10.5,7,10.5z M19,17h-2v-6h2V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Nd6HeC3edGZF068YIh/mfmv9TrAb/OiMy+ceMEQ/zLzX+t1gN/mRWdeOPGCIf5l5r/W6wC/zYvOvHDiBUP8y8x/rdcBfpsXnXnhxAuG+JeZ/1qvA/w2z+m9gffi+XttXjjxgiH+Zea/1usAv81z+mzgs/i3ES8Y4l9m/mu9DvDbPKfPBj6LfxvxgiH+Zea/1usAv81z+mzgs/i3ES8Y4l9m/uO9DvDbvOg+G/gs/m3EC4b4l5n/eK8D/DYvus8GPot/G/GCIf5l5j/e6wC/zYvus4HP4t9GvGCIf5n5j/c6wG/zovts4LP4txEvGOJfZv7jvQ7w27zoPhv4LP5txAuG+JeZ/3ivA/w2z+mlga/i+Xsw8GD+bcQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv5tngHcyvP30cBf85xeG/gt/uOJFwzxLzP/Np8DfDYvutcGfov/eOIFQ/zLzL/N5wCfzYvutYHf4j+eeMEQ/zLzb/M5wGfzontt4Lf4jydeMMS/zPzbfA7w2bzoXhv4Lf7jiRcM8S8z/zafA3w2L7rXBn6L/3jiBUP8y8y/zecAn81zem3gt/ivJV4wxL/M/Nt8DvDZPKfXBn6L/1riBUP8y8y/zecAn81zem3gt/ivJV4wxL/MvHC/w/P33cB385xeG/gt/muJFwzxLzMvnHjRvTbwW/zXEi8Y4l9mXjjxontt4Lf4ryVeMMS/zLxw4kX32sBv8V9LvGCIf5l54cSL7rWB3+K/lnjBEP8y88KJF91rA7/Ffy3xgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IRVt8QbRTxTUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStairs;
impl IconShape for MdStairs {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M18,8h-2.42v3.33H13v3.33h-2.58 V18H6v-2h2.42v-3.33H11V9.33h2.58V6H18V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40R0HPgt4beCl+Z/pr4HfBj4H2OVfhnjRvDTwW8Bx/nfYBV4H+GteOMSL5iJwnP9dbgUewguH+Je9N/Bd/O/0PsB384Ih/mU/DbwV/zv9DPDWvGCIf9lF4DjP6WuAn+Y5vTTwVTyv1+F5fTXwUjynvwE+muf1WzyvjwH+muf03sB78Zx2gRO8YIgX7qWBv+J5vQzw1zynzwY+i+clntdvA6/Fc/od4LV5XuZ5fQzw1TyntwZ+iuf1MsBf8/whXriPBr6K53QJOM7z+mzgs3hOfwO8NM/rt4HX4jn9DvDaPK+/Bl6K5/Q5wGfznI4DF3leHwN8Nc8f4oX7aeCteE6/A7w2z+ungbfiOf0O8No8r98GXovn9DvAa/O8fht4LZ7TzwBvzfP6a+CleE4/A7w1zx/ihbsIHOc5fQ7w2Tyv3wZei+f0NcBH87x+G3gtntPvAK/N8/pu4L14Tr8DvDbP66uBj+I57QIneP4QL9iDgafzvF4H+G2e10XgOM/pc4DP5nn9NvBaPKffAV6b5/XZwGfxnHaBEzyvtwZ+iuf1EOBWnhfiBXtv4Lt4XuL5M8/rbYCf5nn9NvBaPKffAV6b5/XewHfxvMTzOg5c5Hm9D/DdPC/EC/bdwHvxnP4GeGme14OBp/O8Xgf4bZ7XbwOvxXP6HeC1eV6vDfwWz+shwK08r1uBB/Gcvgd4b54X4gV7OvBgntPXAB/N83pt4Ld4XieAXZ7XbwOvxXP6HeC1eV4PBp7O83od4Ld5Xt8NvBfP6VbgITwvxPP3YODpPK+3AX6a5/XewHfxvMTz99vAa/Gcfgd4bZ4/87zeB/huntd7A9/F83oIcCvPCfH8vTfwXTyvhwC38rw+G/gsntMzgAfz/P028Fo8p98BXpvnbxc4xnP6HOCzeV4PBp7O83of4Lt5Tojn77OBz+I5PQN4MM/fdwPvxXP6HeC1ef5+G3gtntPvAK/N8/fbwGvxnL4G+Giev1uBB/GcPgf4bJ4T4vn7buC9eE6/A7w2z99vA6/Fc/od4LV5/n4beC2e0+8Ar83z99vAa/Gcfgd4bZ6/3wZei+f0PcB785wQz99nA5/Fc/od4LV5/p4OPJjn9DvAa/P8/TbwWjyn3wFem+fvt4HX4jndCjyE5++3gdfiOX0O8Nk8J8Tz99nAZ/G8HgLcynN6beC3eP4eAtzKc3pp4K94/h4C3MpzOg5c5Pl7HeC3eU4PBp7O8/oc4LN5Tojn76WBv+J5/TXwOsAuV7w08FPAg3n+/hp4G+BWrngw8FPAS/P8/TXwNsCtXPFg4KeAl+b5uxV4G+CvueI48FvAS/O8Xgb4a54T4gW7FXgQz99vc8Vr86L5ba54bV40v80Vr82L5re54rV5/p4BPJjnhXjBXhv4Lf5veBvgp3leiBfuu4H34n+37wHem+cP8S8zL7rPAR4MvBcvmu8BdoGP4kXzPcBfA1/Fi068YIh/mfmX/Q3w0cBvc8V7A58NPIjn7xnARwM/zRVvDXw18CCev2cAnw18N1e8NvDVwEvxLxMvGOJfZp7X7wB/DewCPw38Nc/fawOvDRwHjgO3Ar8N/DbP30sDbw0cB44DtwJ/Dfw0z99LA28NHAdeGngtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r48G/pr/WV4a+Gqel3jBEP+yXeAY/ztdAo7zgiH+Zd8NvBf/O30P8N68YIh/2YOBp/O/00OAW3nBEC+a9wa+i/9d3gf4bl44xIvuwcBnA28NHON/pkvATwOfDdzKvwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRJmA5QTgzRvQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStorefront;
impl IconShape for MdStorefront {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.9,8.89l-1.05-4.37c-0.22-0.9-1-1.52-1.91-1.52H5.05C4.15,3,3.36,3.63,3.15,4.52L2.1,8.89 c-0.24,1.02-0.02,2.06,0.62,2.88C2.8,11.88,2.91,11.96,3,12.06V19c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-6.94 c0.09-0.09,0.2-0.18,0.28-0.28C21.92,10.96,22.15,9.91,21.9,8.89z M18.91,4.99l1.05,4.37c0.1,0.42,0.01,0.84-0.25,1.17 C19.57,10.71,19.27,11,18.77,11c-0.61,0-1.14-0.49-1.21-1.14L16.98,5L18.91,4.99z M13,5h1.96l0.54,4.52 c0.05,0.39-0.07,0.78-0.33,1.07C14.95,10.85,14.63,11,14.22,11C13.55,11,13,10.41,13,9.69V5z M8.49,9.52L9.04,5H11v4.69 C11,10.41,10.45,11,9.71,11c-0.34,0-0.65-0.15-0.89-0.41C8.57,10.3,8.45,9.91,8.49,9.52z M4.04,9.36L5.05,5h1.97L6.44,9.86 C6.36,10.51,5.84,11,5.23,11c-0.49,0-0.8-0.29-0.93-0.47C4.03,10.21,3.94,9.78,4.04,9.36z M5,19v-6.03C5.08,12.98,5.15,13,5.23,13 c0.87,0,1.66-0.36,2.24-0.95c0.6,0.6,1.4,0.95,2.31,0.95c0.87,0,1.65-0.36,2.23-0.93c0.59,0.57,1.39,0.93,2.29,0.93 c0.84,0,1.64-0.35,2.24-0.95c0.58,0.59,1.37,0.95,2.24,0.95c0.08,0,0.15-0.02,0.23-0.03V19H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFm0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/dg4LWA1wYezBWvzbPtAn8N7AJ/Dfw28Dv867028F7AawMP5orfBv4a+BrgVv5liP8YDwbeC3hv4MH86+0CPw38NPAzvHAPBr4LeG1euK8GPoYXDvHv89rAewHvzX+cW4HPBr6H5/XSwG8Bx3nR/DXwOsAuzx/i3+Y48FnAR/Of43uA9+Y5vTTwW8Bx/nV+Gngbnj/Ev95x4OnAcf5z/A3w0jyn48DTgeP827wP8N08L8S/zXsD38V/vL8BXhvY5dmOA78FvDTP3/cAvw08GHhv4EE8r1uBh/C8EP927w18F/9xLgGvDfw1z+m7gPfm+Xsf4Lt5tuPAdwNvxfN6GeCveU6If5/3Br6L/xivA/w2z+mrgY/i+Xsf4Lt5Xg8Gns7z+hzgs3lOiH+/9wa+i3+f9wG+m+f03sB38fx9DvDZvGB/DbwUz+lzgM/mOSH+Y7w38F3823wN8NE8p9cGfosX7HWA3+YF+23gtXhOvwO8Ns8J8R/nvYHv4l/ne4D35jm9NPBbwHFesNcBfpsX7KuBl+Y5/TXw0TwnxH+s9wa+ixfN3wCvDezybMeB3wJemhfudYDf5t8P8R/vvYHv4oW7BDwY2OU5/RXw0vzLXgf4bf79EP853hv4Lp6/S8BrA3/Nc/ou4L150bwO8Nv8+yH+87w38F08r7cBfprn9NnAZ/Giex3gt/n3Q/znem/gu3i29wG+m+f03sB38a/zOsBv8++H+M/33sB3Ad8DvDfP6aWBv+Jf73WA3+bfD/Ff462Bn+Y5vTTwW8Bx/vVeB/ht/v0Q/z2OA78FvDT/Nq8D/Db/foj/Hr8FvDb/dq8D/Db/foj/et8FvDf/Pq8D/Db/foj/Wh8NfBX/fq8D/Db/foj/Ou8NfBf/MV4H+G3+/RD/NV4a+C3gOP8xXgf4bf79EP/5Hgz8FXCc/zivA/w2/36I/1zHgd8CXpr/WK8D/Db/foj/XL8FvDb/8V4H+G3+/RD/eb4LeG/+c7wO8Nv8+yH+c7w38F3853kd4Lf590P8x3tr4Kf4z/U6wG/z74f4j/XSwG8Bx/nP9TrAb/Pvh/iPcxx4OnCc/3yvA/w2/36I/xjHgd8CXpr/Gq8D/Db/foj/GF8NfBT/dR4C3Mq/H+I/xm8Dr8V/jZ8B3pr/GIj/GD8NvBX/+f4GeG1gl/8YiP8YLw38NnCM/zw/A7w3sMt/HMR/nOPAS/Of41bgVv7jIf5/Q/z/hvi3eS/go4GX5oq/Br4a+B7+Yx0HPgp4b+DBwC7w08DnALfy74f41/su4L15/r4beB/+YzwY+C3gwTx/7wN8N/8+iH+dzwY+ixfuc4DP5t/vr4CX5oV7CHAr/3aIf52LwHFeuF3gBP8+bw38FP+y7wHem387xIvutYHf4kXzOsBv82/31cBH8S/bBU7wb4d40b028Fu8aF4H+G3+7X4aeCteNOLfDvGie2ngr3jRvAzw1/zbfTbwWfzLngE8mH87xL/OrcCDeOGeATyYf5+XBv6Kf9nnAJ/Nvx3iX+e1gd/ihXsd4Lf59/tq4KN4wf4GeG1gl387xL/eewNfDRzjOV0CPhr4bv7jfDXwUTyvvwHeGriVfx/Ev81x4L2Bl+aKvwa+G9jlP95LA28NvDTw18BfAz/NfwzE/2+I/98Q/78h/n9D/P/GPwIorr9B7+AcowAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStroller;
impl IconShape for MdStroller {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "16",
                cy: "20",
                r: "2",
            }
            circle {
                cx: "6",
                cy: "20",
                r: "2",
            }
            path {
                d: "M22,7V6.48C22,4.56,20.52,3,18.65,3c-1.66,0-2.54,1.27-3.18,2.03l-8.8,10.32C6.12,16,6.58,17,7.43,17L15,17 c1.1,0,2-0.9,2-2V6.27C17.58,5.59,17.97,5,18.65,5C19.42,5,20,5.66,20,6.48V7H22z",
            }
            path {
                d: "M14.3,4.1C13.03,3.4,11.56,3,10,3C8.03,3,6.21,3.64,4.72,4.72l4.89,4.89L14.3,4.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTapas;
impl IconShape for MdTapas {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,10V1h-8v9c0,1.86,1.28,3.41,3,3.86V21h-2v2h6v-2h-2v-7.14C20.72,13.41,22,11.86,22,10z M20,3v3h-4V3H20z M12.5,11.5 c0,1.38-1.12,2.5-2.5,2.5H8v9H6v-9H4c-1.38,0-2.5-1.12-2.5-2.5C1.5,10.12,2.62,9,4,9h2V8H4C2.62,8,1.5,6.88,1.5,5.5 C1.5,4.12,2.62,3,4,3h2V1h2v2h2c1.38,0,2.5,1.12,2.5,2.5C12.5,6.88,11.38,8,10,8H8v1h2C11.38,9,12.5,10.12,12.5,11.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTty;
impl IconShape for MdTty {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,4h2v2h-2V4z M13,7h2v2h-2V7z M11,4h2v2h-2V4z M18,9h-2V7h2V9z M19,6h-2V4h2V6z M21,9h-2V7h2V9z M22,6h-2V4h2V6z M14.62,14.38L12.1,16.9c-2.5-1.43-4.57-3.5-6-6l2.52-2.52C8.86,8.14,8.96,7.8,8.9,7.48L8.16,3.8C8.07,3.34,7.66,3,7.18,3H3.03 C2.47,3,2,3.47,2.03,4.03C2.2,6.92,3.05,9.63,4.43,12c1.58,2.73,3.85,4.99,6.57,6.57c2.37,1.37,5.08,2.23,7.97,2.4 c0.56,0.03,1.03-0.44,1.03-1v-4.15c0-0.48-0.34-0.89-0.8-0.98l-3.67-0.73C15.2,14.04,14.86,14.14,14.62,14.38z M14,10h2v2h-2V10z M11,10h2v2h-2V10z M19,12h-2v-2h2V12z M22,12h-2v-2h2V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUmbrella;
impl IconShape for MdUmbrella {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.5,6.92L13,5.77V3.88V3.4c0-0.26,0.22-0.48,0.5-0.48c0.28,0,0.5,0.21,0.5,0.48V4h2V3.4C16,2.07,14.88,1,13.5,1 C12.12,1,11,2.07,11,3.4v0.48v1.89L9.5,6.92L6,6.07l5.05,15.25C11.2,21.77,11.6,22,12,22s0.8-0.23,0.95-0.69L18,6.07L14.5,6.92z M13.28,8.5l0.76,0.58l0.92-0.23L13,14.8V8.29L13.28,8.5z M9.96,9.09l0.76-0.58L11,8.29v6.51L9.03,8.86L9.96,9.09z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFC0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/huPALv96iP/93hv4KuBlgFv510H87/bSwF9xxV8DL8O/DuJ/r+PAXwEP5tm+BvhoXnSI/71+CnhrntfbAD/Niwbxv9NbAz/F87cLPATY5V+G+N/nOPB04Dgv2PcA782/DPG/z2cDn8W/7HWA3+aFQ/zv8mDg6bxofht4HV44xP8uXw18FC+6lwH+mhcM8b/L04EH86L7GuCjecEQ/7mOA68N/DQvmpfmir/meb008Ff86/wO8Nq8YIj/PMeB3wJeGngf4Lt54Y4DT+eKhwC7PKfXBn6Lfz3xgiH+cxwHfgt4aZ7tfYDv5gX7LeC1ueK3gdfhOb028Fv861wCjvOCIf7jvTTwW8BxntfbAD/N8/ps4LN4Th8DfDXPdhy4yL/OzwBvzQuG+I/10sBvAcd5/v4aeBme04OBvwKO85x2gZcBbuXZvht4L1507wN8Ny8Y4j/OSwO/BRznhRPP6buB9+L5+x7gvXm2BwN/DRzjX/Y7wGvzwiH+Y7w08FvAcV64nwHemud0ETjO87cLnOA5vTbw08AxXrC/AV4b2OWFQ/z7vTfwVcBx/mUPAW7lOe0Cx3j+ngE8mOf1YOCzgffiOV0Cvhr4amCXfxni3+e9ge/iRfM1wEfzvH4aeCuev58B3poX7Djw0jzbb/Ovg/i3e2/gu3jRXAIeDOzyvB4M/DVwjOd0CXhp4Fb+8yD+bd4b+C5edB8DfDUv2IOBrwZemiv+Gvho4Fb+cyH+9T4L+GxedM8AHsz/TIh/ne8C3pt/ndcBfpv/mRAvuu8C3pt/nVuB9+a/zu/wr4N40XwX8N78z3cr8DbAX/OiQfzLvgt4b/73uBV4CC8axAt2HPgt4KX53+d1gN/mX4Z4/o4DvwW8NP87vQ7w2/zLEM/rOPBbwEvzv9MzgAfzokE8r98CXpv/nZ4BvDXw17xoEM/pvYHv4r/W6/Af57f510E8p78CXpr/WuK/D+LZHgw8nf964r8P4tleG/gt/u/5aeBzgL/meSGe7bWB3+L/pl3gIcAuzwnxbK8N/Bb/d30O8Nk8J8SzvTbwW/zf9TXAR/OcEM/22sBv8X/XxwBfzXNCPNtrA7/F/01/A7w0zwvxbK8N/Bb/9X6H/zy3An8NfDXPH+LZXhv4Lf7rif8+iGd7beC3+K8n/vsgnu21gd/iv95v8x/vt4HvAW7lhUM822sDv8X/HbvA6wB/zQuGeLbXBn6L/1t+BnhrXjDEs7028Fv83yNeMMSzvTbwW/zf8jfAS/OCIZ7ttYHf4v+WtwF+mhcM8WyvDfwW/zf8DfDZwE/zwiGe7bWB3+K/nvjvg3i21wZ+i/964r8P4tleG/gt/uu9Nv8+fwPs8m+DeE7mf6ePAb6afz3Ec/pu4L343+l1gN/mXwfxnB4MPJ3/nb4G+Gj+dRDP672B7+J/n68BPpp/HcTz997Ad/G/y+sAv82/DuIFezDw2cB78T/fxwBfzb8e4kXz2vzP9dfALv82iP/fEP+/8Y+JEadBNKDcQgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWash;
impl IconShape for MdWash {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.5,8C19.88,8,21,6.88,21,5.5C21,3.83,18.5,1,18.5,1S16,3.83,16,5.5C16,6.88,17.12,8,18.5,8z M13.5,9 C14.33,9,15,8.33,15,7.5C15,6.66,13.5,5,13.5,5S12,6.66,12,7.5C12,8.33,12.67,9,13.5,9z M9.12,5l-7.18,6.79 C1.34,12.35,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3h6.25H12h5.75c0.69,0,1.25-0.56,1.25-1.25s-0.56-1.25-1.25-1.25H12v-1h7.75 c0.69,0,1.25-0.56,1.25-1.25S20.44,17,19.75,17H12v-1h8.75c0.69,0,1.25-0.56,1.25-1.25s-0.56-1.25-1.25-1.25H12v-1h6.75 c0.69,0,1.25-0.56,1.25-1.25S19.44,10,18.75,10H8.86c0.64-1.11,1.48-2.58,1.49-2.61c0.09-0.16,0.14-0.33,0.14-0.53 c0-0.26-0.09-0.5-0.26-0.7C10.22,6.12,9.12,5,9.12,5L9.12,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEfUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv95x4KeA1+Y5/TbwNsAu/3UQ/7WOA78FvDTP318DrwPs8l8D8V/npYHvAl6aF+6vgfcB/pr/fIj/Gi8N/BZwnBfNLvA6wF/znwvxn++1gZ8CjvOvswu8DfDb/OdB/Od6b+C7+Pd5H+C7+c+B+M/z3sB38R/jfYDv5j8e4j/HdwHvzX+s7wbeh/9YiP943wW8N/85vht4H/7jIP7jHAd+Cnht/nP9NvA2wC7/foj/GMeB3wJemv8afw28DrDLvw/i3++lge8CXpr/Wn8NvA/w1/zbIf59Xhr4LeA4/z12gdcB/pp/G8S/3WsDPwUc57/XLvA2wG/zr4f4t3lv4Lv4n+V9gO/mXwfxr/fewHfxP9P7AN/Niw7xr/NdwHvzH+cSVxzjP853A+/Diwbxovsu4L35j/UxXPFV/Mf6buB9+Jch/mXHgZ8CXpv/WL8DvDZX/DXwUvzH+m3gbYBdXjDEC3cc+C3gpfmP9zLAX3PFSwN/xX+8vwZeB9jl+UO8YC8NfBfw0vzH+xrgo3lOXw18FP/x/hp4H+CveV6I5++lgd8CjvMf7xLwYGCX53QcuBU4xn+8XeB1gL/mOSH+bX4beC3+bT4G+Gqev88GPot/m98BXpt/HcS/zW8Dr8W/3iXgOC/YceBW4Bj/er8DvDb/Ooh/m98GXot/vc8BPpsX7rOBz+Jf73eA1+ZfB/Fv89vAa/Gv9xDgVl64BwNP51/vd4DX5l8H8W/z28Br8a/zO8Br86L5a+Cl+Nf5HeC1+ddB/Nv8NvBa/Ov8DvDavGh+G3gt/nV+B3ht/nUQ/za/DbwW/zp/DbwML5q/Al6af53fAV6bfx3Ev81vA6/Fv95DgFt54R4MPJ1/vd8BXpt/HcS/zW8Dr8W/3m8Dr8ML91vAa/Ov9zvAa/Ovg/i3+W3gtfi3+W7gY4BdntNx4LuAt+bf5neA1+ZfB/Fv89vAa/Fvtwt8N/DXXPHSwHsDx/m3+x3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89vAa/E/y+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PSwHH+Z9kF/pp/HcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzsKjEHQ0SE3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWaterDamage;
impl IconShape for MdWaterDamage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3L2,12h3v8h14v-8h3L12,3z M12,16c-1.1,0-2-0.9-2-2c0-1.1,2-4,2-4s2,2.9,2,4C14,15.1,13.1,16,12,16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWheelchairPickup;
impl IconShape for MdWheelchairPickup {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S4.5,5.11,4.5,4z M10,10.95V9c0-1.1-0.9-2-2-2H5C3.9,7,3,7.9,3,9v6h2v7 h3.5v-0.11c-1.24-1.26-2-2.99-2-4.89C6.5,14.42,7.91,12.16,10,10.95z M16.5,17c0,1.65-1.35,3-3,3s-3-1.35-3-3 c0-1.11,0.61-2.06,1.5-2.58v-2.16C9.98,12.9,8.5,14.77,8.5,17c0,2.76,2.24,5,5,5s5-2.24,5-5H16.5z M19.54,14H15V8h-2v8h5.46 l2.47,3.71l1.66-1.11L19.54,14z",
            }
        }
    }
}
