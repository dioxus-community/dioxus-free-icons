use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAExUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4r/Wg4EH8cI9A7iV/xqI/xyvBbw28GDgwcBLA8f519kF/hq4FbgV+G3gd/iPhfiP8dLAawFvDbw2/7l+G/hp4HeAv+bfB/Fvdxx4L+CjgQfz3+NW4KuB7wF2+ddD/Os9GPgs4L35n2MX+Gngc4BbedEh/nU+C/ho4Dj/M+0CXw18Di8axIvmpYHvAl6af53fAW4FbgX+GtjliluBW3lODwYezBXHgZcGHgw8GHgt/nX+Gngf4K954RD/svcGvgo4zr/sb4CfBn4b+G3+Y7028NrAWwMvxb9sF3gf4Kd5wRAv3FsDP8UL9wzgu4HvBm7lv8aDgfcG3ht4EC/c2wA/zfOHeMEeDPwVcJzn7xnAZwPfzX+v9wY+G3gQz98u8DLArTwvxAv2W8Br8/x9DfDR/M/y1cBH8fz9NvA6PC/E8/fSwF/xvC4B7w38NP8zvTXw3cAxntfLAH/Nc0I8f58NfBbP632A7+Z/tvcGvovn9THAV/OcEM/fdwPvxXP6G+Cl+d/hr4GX4jl9DfDRPCfE8/fVwEfxnP4aeBn+9b4LeB/+a/0V8NI8p68BPprnhHj+Phr4Kp7X2wA/zb+Oge8G3of/Gm8N/BTP63OAz+Y5IZ6/BwNP53ntAu8D/DQvOnPFdwPvw3+u1wZ+CjjO83oIcCvPCfGCfTfwXjx/Xw18DrDLv8w823cD78N/vOPARwGfzfP3PcB787wQL9iDgb8GjvH83Qp8NvA9vHDmOX038D78x3kv4LOBB/P8XQJeGriV54V44V4a+CteuF3gu4GvAW7leZnn9d3A+/Bv92DgvYD3Bh7MC/c6wG/z/CH+ZW8NfDdwjH/ZXwO/Dfw08DfALmCev+8G3ocX3XHgo4C3Bl6af9kl4LWBv+YFQ7xoXhr4buCl+Nf5a+ClecG+G3gfXjSvDfwWL5q/Ad4b+GteOMS/zmcDHw0c4z/OdwPvw7/stYHf4oW7BHw18Nm8aBD/eseBrwbeGjjGf4zvBt6HF+61gd/i+bsEfDXw1cAuLzrEv91x4K2BzwYexL/f5wCfzQv22sBv8Zz+Bvhu4LuBXf71EP8xHgy8NfDWwGvxr/c3wGsDu7xgrw38FvA7wE8DPw3cyr8P4j/HawMvDbw08F68cH8DvDawywt3HNjlPxbiP595wf4GeG1gl/8eiP985vn7G+C1gV3++yD+85nn9TfAawO7/PdC/Oczz+lvgNcGdvnvh/jPZ57tb4DXBnb5nwHxn89c8TfAawO7/M+B+M9n4G+A1wZ2+Z8F8Z/vr4HXBnb5nwfxn+84sMv/TIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj/brskGE7Tv7AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md360;
impl IconShape for Md360 {
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
                d: "M12 7C6.48 7 2 9.24 2 12c0 2.24 2.94 4.13 7 4.77V20l4-4-4-4v2.73c-3.15-.56-5-1.9-5-2.73 0-1.06 3.04-3 8-3s8 1.94 8 3c0 .73-1.46 1.89-4 2.53v2.05c3.53-.77 6-2.53 6-4.58 0-2.76-4.48-5-10-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi+Xtp4Bj/c/0NsMu/H+L5+23gtfif63WA3+bfD/H8/TbwWvzP9TrAb/Pvh3j+fht4Lf7neh3gt/n3Qzx/vw28Fv9zvQ7w2/z7IZ6/3wZei/+5Xgf4bf79EP+/If5/Qzx/Lw0c43+uvwF2+fdDPH9/Bbw0/3N9DvDZ/Pshnj/zP9vPAG/Nvx/ieb028Fv8z3Yr8BD+/RDP66OBr+J/vhPALv8+iOf13cB78T/f6wC/zb8P4nn9FfDS/M/3OcBn8++DeF7mf4efAd6afx/Ec3pt4Lf43+FW4CH8+yCe00cDX8X/HieAXf7tEM/pu4H34n+P1wF+m387xHP6K+Cl+d/jc4DP5t8O8ZzM/y4/A7w1/3aIZ3tt4Lf43+VW4CH82yGe7bWB3+J/H/Fvh3i21wZ+i/99xL8d4tleG/gt/vcR/3aIZ3tt4Lf430f82yGe7bWB3+J/H/Fvh3i248BLc8VXAy/Fc/ob4KP51/stntfHAH/Ni+a3eF4fA/w1V/w2/3aI5++3gdfiOf0O8Nr865nn9TrAb/OiMc/rdYDf5t8P8fz9NvBaPKffAV6bfz3zvF4H+G1eNOZ5vQ7w2/z7IZ6/3wZei+f0O8Br869nntfrAL/Ni8Y8r9cBfpsrzPN6HeC3+Zchnr/fBl6L5/Q7wGvzr2ee1+sAv82Lxjyv1wF+myvM83od4Lf5lyGev98GXovn9DvAa/OvZ57X6wC/zYvGPK/XAX6bK8zzeh3gt/mXIZ6/3wZei+f0O8Br869nntfrAL/Ni8Y8r9cBfpsrzPN6HeC3+Zchnr/fBl6L5/Q7wGvzr2ee1+sAv82Lxjyv1wF+myvM83od4Lf5lyGev98GXovn9DvAa/OvZ57X6wC/zYvmtXlefw3scoV5Xq8D/Db/MsTz99vAa/Gcfgd4bf71zPN6HeC3+Y9hntfrAL/Nvwzx/P028Fo8p98BXpt/PfO8Xgf4bf5jmOf1OsBv8y9DPH+/DbwW/3leB/htXjTmP97rAL8NIJ6/3wZei/88rwP8Ni8a8x/vdYDfBhDP328Dr8V/ntcBfpsXjfmP9zrAbwOI5++3gdfiP8/rAL/Ni8b8x3sd4LcBxPP328Br8Z/ndYDf5kVj/uO9DvDbAOL5e2ngOP95/hrY5UXz2rxwv8Xz+hjgr3nB/hrYBRD/+5nn9TrAb/MvQ/zvZ57X6wC/zb8M8b+feV6vA/w2/zLE/37meb0O8Nv8yxD/+5nn9TrAb/MvQ/zvZ57X6wC/zb8M8b+feV6vA/w2/zLE/36vzfP6a2CXfxni/zfE/2/8I1/XpkEVS/FVAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddBusiness;
impl IconShape for MdAddBusiness {
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
                d: "M15,17h2v-3h1v-2l-1-5H2l-1,5v2h1v6h9v-6h4V17z M9,18H4v-4h5V18z",
            }
            rect {
                height: "2",
                width: "15",
                x: "2",
                y: "4",
            }
            polygon {
                points: "20,18 20,15 18,15 18,18 15,18 15,20 18,20 18,23 20,23 20,20 23,20 23,18",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Nf67fBt4G2OXfD/Ef57eA1+a/xm8Dr8O/H+I/xncB781/re8G3od/H8S/33sD38W/zscAf81zemngq/jXeR/gu/m3Q/z7HAeeDhznX+d1gN/mOb028Fv86+wCDwF2+bdB/Pt8NvBZ/Ou9DvDbPKfXBn6Lf73PAT6bfxvEv92Dgb8CjvOv9zrAb/OcXhv4Lf71doGXAW7lXw/xb/fRwFfxb/M6wG/znF4b+C3+bT4G+Gr+9RD/dr8NvBb/Nq8D/DbP6bWB3+Lf5meAt+ZfD/Fvcxy4yL/d6wC/zXN6beC3+Lc7Aezyr4P4t3lt4Lf4t3sd4Ld5Tq8N/Bb/dq8D/Db/Ooh/m88GPosX7GOAv+YF+2tgl+d0HHhpXrCXBr6KF+xjgK/mXwfxb/PZwGfxgr0O8Nv8x3pt4Ld4wT4H+Gz+dRD/Nt8NvBcv2OsAv81/rNcGfosX7GuAj+ZfB/Fv893Ae/GCvQ7w2/zHem3gt3jBvgd4b/51EP82nw18Fi/Y6wC/zX+s1wZ+ixfsc4DP5l8H8W/z2cBn8YJ9NPDXvGB/A+zynI4DL8UL9tLAV/OCfQ7w2fzrIP5tXhv4Lf7tXgf4bZ7TawO/xb/d6wC/zb8O4t/mOHCRf7vXAX6b5/TawG/xb3cC2OVfB/FvdyvwIP5tXgf4bZ7TawO/xb/NM4AH86+H+Lf7auCj+Ld5HeC3eU6vDfwW/zZfA3w0/3qIf7uXBv6Kf5vXAX6b5/TawG/xb/MywF/zr4f497kVeBD/eq8D/DbP6bWB3+Jf7xnAg/m3Qfz7fDbwWfzrvQ7w2zyn1wZ+i3+9zwE+m38bxL/Pg4Gn86/30cBf85xeGvhq/vUeAtzKvw3i3++7gffiv8f3AO/Nvx3i3++lgb/iv8dDgFv5t0P8x/ht4LX4r/UzwFvz74P4j/HawG/xX+tlgL/m3wfxH+e7gffiv8b3AO/Nvx/iP86DgafzX+MhwK38+yH+Y3028Fn85/oc4LP5j4H4j3Uc+GvgQfzneAbw0sAu/zEQ//HeGvgp/nO8DfDT/MdB/Of4aeCt+I/1M8Bb8x8L8Z/jwcBfA8f4j3EJeDCwy38sxH+ejwa+iv8YHwN8Nf/xEP+5fhp4K/59fgZ4a/5zIP5zHQduBY7xb3MJeDCwy38OxH++twZ+in+btwF+mv88iP8aXw18FP86XwN8NP+5EP91/hp4KV40fwO8NP/5EP91Hgz8NXCMF+4S8NLArfznQ/zXemvgp3jh3gb4af5rIP7rfTbwWTx/nwN8Nv91EP89fhp4K57T9wDvzX8txH+P48BvAy/FFX8DvDawy38txH+f48CtXPFgYJf/eoj/Xi/NFX/Nfw/E/2+I/98Q/78h/n9D/P/GPwLaIKpBmCHftgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddLocation;
impl IconShape for MdAddLocation {
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
                d: "M12 2C8.14 2 5 5.14 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.86-3.14-7-7-7zm4 8h-3v3h-2v-3H8V8h3V5h2v3h3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/v9fief0NsMu/DPG/n3lerwP8Nv8yxP9+5nm9DvDb/MsQ//uZ5/U6wG/zL0P872ee1+sAv82/DPG/n3lerwP8Nv8yxH+d1wJeG3ht4Djw0jyn3wZ2gb8GvgbY5UVjntfrAL/Nvwzxn+u1gfcC3ho4zovudYDf5kVjntfrAL/Nvwzxn+O1gc8CXpt/m9cBfpsrXosX7rd5Xh8N/DUv2N8AuwDiP9Zx4LuAt+bf53WA3+YK8x/vdYDfBhD/cV4b+CngOP9+rwP8NleY/3ivA/w2gPiP8d7Ad/Ef53WA3+YK8x/vdYDfBhD/fu8NfBf/sV4H+G2uMP/xXgf4bQDx7/PWwE/xH+91gN/mCvMf73WA3wYQ/3YPBv4KOM5/vNcBfpsXjXlerwP8Nv8yxL/dbwGvzX+O1wF+mxeNeV6vA/w2/zLEv81HA1/Ff56PBv6aF+xvgF2uMM/rdYDf5l+G+Ld5OvBg/vu8DvDbXGGe1+sAv82/DPGv997Ad/Hf63WA3+YK87xeB/ht/mWIf72/Al6a/16vA/w2V5jn9TrAb/MvQ/zrvDTwV/z3ex3gt7nCPK/XAX6bfxniX+ezgc/iv9/rAL/NFeZ5vQ7w2/zLEP86vw28Fv9+3wN8NfDXXPHSwEcD78WL5nWA3+bfD/GvY/793gf4bp6/9wa+i3/Z6wC/zb8f4kV3HLjIv8/nAJ/NC/fZwGfxwn008Ne8YH8D7PIvQ7zoXhv4Lf59TgC7vHDHgYv8+7wO8Nv8yxAvutcGfot/u98BXpsXzW8Dr8W/3esAv82/DPGie23gt/i3+x3gtXnR/DbwWvzbvQ7w2/zLEC+61wZ+i3+7vwZehhfNXwEvzb/d6wC/zb8M8aJ7beC3+Pd5CHArL9yDgafz7/M6wG/zL0P865h/n98GXocX7reA1+bfR7xoEP86u8Ax/n2+G/gYYJfndBz4LuCt+fe5BBznRYP41/lu4L3499sFvhv4a654aeC9geP8+30P8N68aBD/Ou8NfBf/s70P8N28aBD/Og8Gns7/bCeAXV40iH+9nwbeiv+ZfgZ4a150iH+91wZ+i/+ZXgf4bV50iH+bvwZeiv9Zfgd4bf51EP82bw38FP+zvA3w0/zrIP7tfht4Lf5n+B3gtfnXQ/zbPRh4Ov8zPAS4lX89xL/PZwOfxX+vzwE+m38bxL/PceC3gZfiv8ffAK8N7PJvg/j3e2ngt4Fj/Ne6BLw28Nf82yH+Y7w38F3813of4Lv590H8x/lp4K34r/E9wHvz74f4j3Mc+G3gpfjP9TfAawO7/Psh/mMdB24FjvGf4xLwYGCX/xiI/3gvDfw2cIz/WJeA1wb+mv84iP8crw38Fv+xXgf4bf5jIf7zvDfwXfzHeB/gu/mPh/jP9d7Ad/Hv8z7Ad/OfA/Gf772B7+Lf5n2A7+Y/D+K/xnsD38W/zvsA381/LsR/nfcGvosXzfsA381/PsR/rfcGvho4xvN3Cfho4Lv5r4H4r/fSwG8Dx3hOl4DXBv6a/zqI/x4vDfw2cIwrLgGvDfw1/7UQ/32OA7/NFa8N7PJfD/Hf6zhX7PLfA/H/G+L/N/4RwUvAQW+iaysAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddLocationAlt;
impl IconShape for MdAddLocationAlt {
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
                d: "M20 1v3h3v2h-3v3h-2V6h-3V4h3V1h2zm-8 12c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm2-9.75V7h3v3h2.92c.05.39.08.79.08 1.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 6.22 7.8 3 12 3c.68 0 1.35.08 2 .25z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddRoad;
impl IconShape for MdAddRoad {
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
            polygon {
                points: "20,18 20,15 18,15 18,18 15,18 15,20 18,20 18,23 20,23 20,20 23,20 23,18",
            }
            rect {
                height: "9",
                width: "2",
                x: "18",
                y: "4",
            }
            rect {
                height: "16",
                width: "2",
                x: "4",
                y: "4",
            }
            rect {
                height: "4",
                width: "2",
                x: "11",
                y: "4",
            }
            rect {
                height: "4",
                width: "2",
                x: "11",
                y: "10",
            }
            rect {
                height: "4",
                width: "2",
                x: "11",
                y: "16",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAgriculture;
impl IconShape for MdAgriculture {
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
                d: "M19.5,12c0.93,0,1.78,0.28,2.5,0.76V8c0-1.1-0.9-2-2-2h-6.29l-1.06-1.06l1.41-1.41l-0.71-0.71L9.82,6.35l0.71,0.71 l1.41-1.41L13,6.71V9c0,1.1-0.9,2-2,2h-0.54c0.95,1.06,1.54,2.46,1.54,4c0,0.34-0.04,0.67-0.09,1h3.14 C15.3,13.75,17.19,12,19.5,12z",
            }
            path {
                d: "M19.5,13c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5S21.43,13,19.5,13z M19.5,18 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S20.33,18,19.5,18z",
            }
            path {
                d: "M4,9h5c0-1.1-0.9-2-2-2H4C3.45,7,3,7.45,3,8C3,8.55,3.45,9,4,9z",
            }
            path {
                d: "M9.83,13.82l-0.18-0.47L10.58,13c-0.46-1.06-1.28-1.91-2.31-2.43l-0.4,0.89l-0.46-0.21l0.4-0.9C7.26,10.13,6.64,10,6,10 c-0.53,0-1.04,0.11-1.52,0.26l0.34,0.91l-0.47,0.18L4,10.42c-1.06,0.46-1.91,1.28-2.43,2.31l0.89,0.4l-0.21,0.46l-0.9-0.4 C1.13,13.74,1,14.36,1,15c0,0.53,0.11,1.04,0.26,1.52l0.91-0.34l0.18,0.47L1.42,17c0.46,1.06,1.28,1.91,2.31,2.43l0.4-0.89 l0.46,0.21l-0.4,0.9C4.74,19.87,5.36,20,6,20c0.53,0,1.04-0.11,1.52-0.26l-0.34-0.91l0.47-0.18L8,19.58 c1.06-0.46,1.91-1.28,2.43-2.31l-0.89-0.4l0.21-0.46l0.9,0.4C10.87,16.26,11,15.64,11,15c0-0.53-0.11-1.04-0.26-1.52L9.83,13.82z M7.15,17.77c-1.53,0.63-3.29-0.09-3.92-1.62c-0.63-1.53,0.09-3.29,1.62-3.92c1.53-0.63,3.29,0.09,3.92,1.62 C9.41,15.38,8.68,17.14,7.15,17.77z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/cT4L+Bpgl/9cx4GPAj6Hfz/Ef4zvAt4b+GvgdYBd/nMcB34LeGngu4H34d8H8e/3XcB782x/DbwOsMt/rOPAbwEvzbN9N/A+/Nsh/n2+C3hvntdfA68D7PIf4zjwW8BL87y+G3gf/m0Q/3bfBbw3L9hfA68D7PLvcxz4LeClecG+G3gf/vUQ/zbfBbw3/7K/Bl4H2OXf5jjwW8BL8y/7buB9+NdB/Ot9F/DevOj+GngdYJd/nePAbwEvzYvuu4H34UWH+Nf5LuC9+df7a+B1gF1eNMeB3wJemn+97wbehxcN4kX3XcB782/318DrALu8cMeB3wJemn+77wbeh38Z4kXzXcB78+/318DrALs8f8eB3wJemn+/7wbehxcO8S/7LuC9+Y/z18DrALs8p+PAbwEvzX+c7wbehxcM8e9nntfnAJ/Nf6yPBr6K5yX+7RD/frcCD+I5fQ7w2fzH+mzgs3hOfwO8NP92iH+/3wZei+f0OcBn8x/rs4HP4jn9DvDa/Nsh/v1+G3gtntP3AO/Nf6zvBt6L5/Q7wGvzb4f49/tq4KN4tr8Bvhv4av5j/TbwWjyn7wHem387xL/fewMvDfw28NvALv85zPP6GOCr+bdD/O/wYODpPK/XAX6bfzvE/w4fDXwVz+sEsMu/HeI/3nHgtYAHA1/Df4y/Al6a5/Q3wEvz74P4j/HSwHsBrw28NM/2EOBW/n1eGvgrntfHAF/Nvw/iP8aDgafzvL4HeG/+fX4KeGue18sAf82/D+I/zm8Dr8Xzeh3gt/m3eW3gt3hevwO8Nv9+iP84bw38FM9rF3gIsMu/znHg6cBxntfrAL/Nvx/iP9ZvA6/F8/pr4GV40R0Hfgt4aZ7X7wCvzX8MxH+slwb+iuf1PsB386I5DvwW8NI8fw8BbuU/BuI/3kcDX8WzvQ/w3bxojgO/Bbw0z9/7AN/NfxzEf47vBt4LeB/gu3lOLw3cCuzynF4a+CngwTx/3wO8N/+xEP95Xhv4bZ7TSwO/BewCbwP8NVe8NPBbwHGev+8B3pv/eIj/Oi8N/BZwnCtuBV6GK34LeGmev+8B3pv/HIj/Gi8N/BZwnOf021zx2jx/3wO8N/95EP81vhr4KP51Pgf4bP5zIf7rfDXwUbxoPgb4av7zIf5rfTfwXrxw3wO8N/81EP+1jgO3Asd4/i4BDwZ2+a+B+K/32cBn8fx9DvDZ/NdB/Nd7MPB0nr+HALfyXwfx32MXOMZzugQc578W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x5fDbw0z+mvgY/mvxbi/zfE/2+I/98Q/78h/n/jHwG827RB52T5WQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAltRoute;
impl IconShape for MdAltRoute {
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
                d: "M9.78,11.16l-1.42,1.42c-0.68-0.69-1.34-1.58-1.79-2.94l1.94-0.49C8.83,10.04,9.28,10.65,9.78,11.16z M11,6L7,2L3,6h3.02 C6.04,6.81,6.1,7.54,6.21,8.17l1.94-0.49C8.08,7.2,8.03,6.63,8.02,6H11z M21,6l-4-4l-4,4h2.99c-0.1,3.68-1.28,4.75-2.54,5.88 c-0.5,0.44-1.01,0.92-1.45,1.55c-0.34-0.49-0.73-0.88-1.13-1.24L9.46,13.6C10.39,14.45,11,15.14,11,17c0,0,0,0,0,0h0v5h2v-5 c0,0,0,0,0,0c0-2.02,0.71-2.66,1.79-3.63c1.38-1.24,3.08-2.78,3.2-7.37H21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnjBPgv4aOA4L9ytwHcDn8Oz/TbwWvzbvQ7w2zynzwI+GjjOv86twHcDn8PzQjx/bw38FP86bwP8NFf8NvBa/Nu9DvDbPNtbAz/Fv8/bAD/Nc0I8f98NvBf/Ot8DvDdX/DbwWvzbvQ7w2zzbdwPvxb/P9wDvzXNCPH+/DbwW/zq/A7w2V/w28Fr8270O8Ns8228Dr8W/z+8Ar81zQjx/vw28Fs/2OsBv85xeG/gtnu13gNfmhftt4LV4ttcBfpt/2W8Dr8WzvQ7w21xhnpO44rWB3+LZfgd4bZ4T4vn7beC1eLbXAX6b5/TawG/xbL8DvDYv3G8Dr8WzvQ7w2/zLfht4LZ7tdYDf5grznMQVrw38Fs/2O8Br85wQz99vA6/Fs70O8Ns8p9cGfotn+x3gtXnhfht4LZ7tdYDf5l/228Br8WyvA/w2V5jnJK54beC3eLbfAV6b54R4/n4beC2e7XWA3+Y5vTbwWzzb7wCvzQv328Br8WyvA/w2/7LfBl6LZ3sd4Le5wjwnccVrA7/Fs/0O8No8J8Tz99vAa/Gv8zvAa/PC/TbwWjzb6wC/zb/st4HX4tleB/htrjDPSVzx2sBv8Wy/A7w2zwnx/P028Fr86/wO8Nq8cL8NvBbP9jrAb/Mv+23gtXi21wF+myvMcxJXvDbwWzzb7wCvzXNCPH+/DbwW/zq/A7w2L9xvA6/Fs70O8Nv8y34beC2e7XWA3+YK85zEFa8N/BbP9jvAa/OcEM/fbwOvxb/O7wCvzQv328Br8WyvA/w2/7LfBl6LZ3sd4Le5wjwnccVrA7/Fs/0O8No8J8Tz99vAa/FsrwP8Ns/ptYHf4tl+B3htXrjfBl6LZ3sd4Lf5l/028Fo82+sAv80V5jmJK14b+C2e7XeA1+Y5IZ6/3wZei2d7HeC3eU6vDfwWz/Y7wGvzwv028Fo82+sAv82/7LeB1+LZXgf4ba54bZ7Tb3PFawO/xbP9DvDaPCfE8/fbwGvxbK8D/DbP6bWB3+LZfgd4bV643wZei2d7HeC3+Zf9NvBaPNvrAL/NC/fawG/xbL8DvDbPCfH8/TbwWjzb6wC/zXN6beC3eLbfAV6bF+63gdfi2V4H+G3+Zb8NvBbP9jrAb/PCvTbwWzzb7wCvzXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGA5JRBoHxvkgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAtm;
impl IconShape for MdAtm {
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
                d: "M8 9v1.5h2.25V15h1.5v-4.5H14V9zM6 9H3c-.55 0-1 .45-1 1v5h1.5v-1.5h2V15H7v-5c0-.55-.45-1-1-1zm-.5 3h-2v-1.5h2V12zM21 9h-4.5c-.55 0-1 .45-1 1v5H17v-4.5h1V14h1.5v-3.51h1V15H22v-5c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAttractions;
impl IconShape for MdAttractions {
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
                d: "M10.43,18.75C10.8,18.29,11.37,18,12,18c0.63,0,1.19,0.29,1.56,0.75c0.39-0.09,0.76-0.21,1.12-0.36l-1.42-3.18 c-0.39,0.15-0.82,0.23-1.26,0.23c-0.46,0-0.9-0.09-1.3-0.25l-1.43,3.19C9.65,18.54,10.03,18.67,10.43,18.75z M5.15,10 c-0.16,0.59-0.25,1.21-0.25,1.85c0,0.75,0.12,1.47,0.33,2.15c0.63,0.05,1.22,0.4,1.56,0.99c0.33,0.57,0.35,1.23,0.11,1.79 c0.27,0.27,0.56,0.53,0.87,0.76l1.52-3.39v0c-0.47-0.58-0.75-1.32-0.75-2.13c0-1.89,1.55-3.41,3.46-3.41 c1.91,0,3.46,1.53,3.46,3.41c0,0.82-0.29,1.57-0.78,2.16l1.5,3.35c0.32-0.24,0.62-0.5,0.9-0.79c-0.22-0.55-0.2-1.2,0.12-1.75 c0.33-0.57,0.9-0.92,1.52-0.99c0.22-0.68,0.34-1.41,0.34-2.16c0-0.64-0.09-1.27-0.25-1.86c-0.64-0.04-1.26-0.39-1.6-1 c-0.36-0.62-0.35-1.36-0.03-1.95c-0.91-0.98-2.1-1.71-3.44-2.05C13.39,5.6,12.74,6,12,6c-0.74,0-1.39-0.41-1.74-1.01 C8.92,5.33,7.73,6.04,6.82,7.02C7.15,7.62,7.17,8.37,6.8,9C6.45,9.62,5.81,9.97,5.15,10z M3.85,9.58C3.07,8.98,2.83,7.88,3.34,7 c0.51-0.88,1.58-1.23,2.49-0.85c1.11-1.17,2.56-2.03,4.18-2.42C10.15,2.75,10.99,2,12,2c1.01,0,1.85,0.75,1.98,1.73 c1.63,0.39,3.07,1.24,4.18,2.42c0.91-0.38,1.99-0.03,2.49,0.85c0.51,0.88,0.27,1.98-0.51,2.58c0.23,0.77,0.35,1.58,0.35,2.42 s-0.12,1.65-0.35,2.42c0.78,0.6,1.02,1.7,0.51,2.58c-0.51,0.88-1.58,1.23-2.49,0.85c-0.4,0.43-0.85,0.81-1.34,1.15l1.34,3H16.3 l-0.97-2.17c-0.43,0.18-0.88,0.33-1.34,0.44C13.85,21.25,13.01,22,12,22c-1.01,0-1.85-0.75-1.98-1.73 C9.54,20.15,9.08,20,8.64,19.81L7.66,22H5.78l1.36-3.03c-0.47-0.33-0.91-0.71-1.3-1.12C4.92,18.23,3.85,17.88,3.34,17 c-0.51-0.88-0.27-1.98,0.51-2.58C3.62,13.65,3.5,12.84,3.5,12S3.62,10.35,3.85,9.58z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFT0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/td4KeGngtXn+fhv4a+Bn+K+B+K/xYOCngJfmRfPXwNsAt/KfC/Gf7zjwV8CD+df5a+B1gF3+8yD+83028Fn823wO8Nn850H853s68GD+bW4FHsJ/HsR/PvO8/gbY5TkdB16K5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIgXzUsDx/i3+W2e1+sAv81zem3gt3her82/zSXgr3nhEC/cawPfBTyY/1ivA/w2z+m1gd/iP9atwPsAv83zh3jB3hv4Lv5zvA7w2zyn1wZ+i/8c7wN8N88L8fwdB54OHOc/x+sAv81zem3gt/jPsQs8BNjlOSGev48Gvor/PK8D/DbP6bWB3+I/z8cAX81zQjx/3w28F/95Xgf4bZ7TawO/xX+e7wHem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+7Z4BfDTw11zx0sBXAw/iRfM6wG/zojEvmt8BXpvnhHj+fht4Lf5tngG8NLDLczoO/DXwIP5lrwP8Ni8a86L5HeC1eU6I5++3gdfi3+ZtgJ/m+Xtr4Kf4l70O8Nu8aMyL5neA1+Y5IZ6/3wZei3+bE8Auz9+DgafzL3sd4Ld50ZgXze8Ar81zQjx/vw28Fv82J4Bdnr8HA0/nv8fvAK/Nc0I8f78NvBb/Nm8D/DTP31sDP8V/j98BXpvnhHj+fht4Lf5tbgVeBtjlOR0H/gp4MP89fgd4bZ4T4vn7beC1+Le7Ffho4G+44qWArwYezH+f3wFem+eEeP5+G3gt/vu8DvDbvGjMi+Z3gNfmOSGev98GXov/Pq8D/DYvGvOi+R3gtXlOiOfvt4HX4l/vb4CfBn4buBW4lSseDDwYeG3grYGX4oV7HeC3edGYF83vAK/Nc0I8f78NvBYvur8BPhr4bV40rw18NfBSPH+vA/w2Lxrzovkd4LV5Tojn77eB1+JF8z3Ae/Nv89PAW/Ff43eA1+Y5IZ6/3wZeixfNQ4Bb+bd5MPB0/mv8DvDaPCfE8/fbwGvxohH/Pua/xu8Ar81zQjx/vw28Fi8a8e9j/mv8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH8/DbwV/7f8DPDWPCfE8/fZwGfxf8vnAJ/Nc0I8fy8N/BX/tzwEuJXnhHjBvhr4KP5v+Brgo3leiBfuq4GP4n+3rwE+mucP8S97aeCtgZcGjvO/wy7w18B3A7fygiH+f0P8/4b4/w3x/xvi/zf+Eb4LukHpzCAkAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBadge;
impl IconShape for MdBadge {
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
                d: "M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M9,12c0.83,0,1.5,0.67,1.5,1.5S9.83,15,9,15s-1.5-0.67-1.5-1.5S8.17,12,9,12z M12,18H6v-0.75c0-1,2-1.5,3-1.5 s3,0.5,3,1.5V18z M13,9h-2V4h2V9z M18,16.5h-4V15h4V16.5z M18,13.5h-4V12h4V13.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEtUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4r/OceCleNH8DbDLfz7Ef40HA38FHOdFcyvwOsCt/OdC/Nf4buC9+Nf5HuC9+c+F+K/x28Br8a/zO8Br858L8R/rOPBSwDOAW3m23wZei3+d7wHem2d7MPAg4Hf4j4P4j/NewFcDx7nibYCf5orPBj6Lf53PAT6bK94b+C6uuBX4bOB7+PdD/Pu9NvBdwIN5TrcCD+GKzwY+i3+dzwE+myueDjyY5/TbwOcAv82/HeLf7jjwU8Br84KJK94a+Cn+dV4H+G3gOHCRF+y7gY8BdvnXQ/zbfTfwXrxwrwP8NvDawG/xr/M6wG8Drw38Fi/c1wAfzb8e4t/ur4CX5oV7HeC3gZcG/op/nRPALvDWwE/xwv0O8Nr86yH+7f4aeCleuM8BPpsrzL+OuOKzgc/ihfsZ4K3510P827038F28cJ8DfDZXmBfdJeA4V3w28Fm8cO8DfDf/eoh/n1uBB/GC/Qzw1lzx28Br8aL5HeC1ueK3gdfiBXsG8GD+bRD/Pu8NfBcv2O8Ar80Vvw28Fi+a3wFemyt+G3gtXrD3Ab6bfxvEv9+twIN4/naBE1zx1cBH8aL5HOCzucK8YM8AHsy/HeLf772B7+IFE1d8NvBZvGg+B/hsrjAv2NsAP82/HeJf56uA1wZuBT4H+GuuuBV4EM/fywB/DXw08FW8aN4H+G7gpYG/4vn7HeC1ueKlga8CHgz8NPAxvGgQL7qvBj6KZ9sFHgLsAq8N/BbP3+sAvw28NvBbvGheB/ht4LWB3+L5ex3gt4HjwNOB4zzb5wCfzb8M8aJ5aeCveF6fA3w2V/w28Fo8r/cBvht4beC3eNG8DPDXwEcDX8Xz+h3gtbniq4GP4nm9DPDXvHCIF81fAS/N89oFXga4FXht4Ld4Xp8DfDZXmBeNuOKzgc/ieb0O8NvAg4Gn8/z9NfAyvHCIf9lnA5/FC/Y9wHtzxW8Dr8Vz+h7gvbnCvGjEFd8NvBfP6XeA1+aK7wbeixfsY4Cv5gVD/MueDjyYF+4hwK3AawO/xXP6HeC1ueKvgZfihfsb4KW54reB1+I5vQ7w28BLA3/FC/fXwMvwgiH+ZbcCD+IF+xngvYFd/msdB74beCtesL8BXpoXDPEv+27gvXj+Pgb4av57fTTwVTx/3wO8Ny8Y4l/2YODpPKe/Ad4b+Gv+Z3hp4KeBB/GcHgLcyguGeNG8N/DZwIOArwE+G9jlf5bjwGcDHwU8A/hs4Lt54RD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EGOmhQX38C6oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBakeryDining;
impl IconShape for MdBakeryDining {
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
                d: "M19.28,16.34C18.07,15.45,17.46,15,17.46,15s0.32-0.59,0.96-1.78 c0.38-0.59,1.22-0.59,1.6,0l0.81,1.26c0.19,0.3,0.21,0.68,0.06,1l-0.22,0.47C20.42,16.49,19.76,16.67,19.28,16.34z M4.72,16.34 c-0.48,0.33-1.13,0.15-1.39-0.38L3.1,15.49c-0.15-0.32-0.13-0.7,0.06-1l0.81-1.26c0.38-0.59,1.22-0.59,1.6,0 C6.22,14.41,6.54,15,6.54,15S5.93,15.45,4.72,16.34z M15.36,9.37c0.09-0.68,0.73-1.06,1.27-0.75l1.59,0.9 c0.46,0.26,0.63,0.91,0.36,1.41L16.5,15h-1.8L15.36,9.37z M8.63,9.37L9.3,15H7.5l-2.09-4.08c-0.27-0.5-0.1-1.15,0.36-1.41l1.59-0.9 C7.89,8.31,8.54,8.69,8.63,9.37z M13.8,15h-3.6L9.46,8.12C9.39,7.53,9.81,7,10.34,7h3.3c0.53,0,0.94,0.53,0.88,1.12L13.8,15z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Jf77X4n+13eNEhXnSfBXw0cJz/2XaBrwY+h38Z4kXzXcB787/LdwPvwwuH+Je9N/Bd/O/0PsB384Ih/mU/DbwV/zv9DvDavGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+Z/pfbjiu3jhxAuG+JeZ/3neB/hurnhv4Lt4wcQLhviXmf9Z3gf4bp7TewPfxfMnXjDEv8z8z/E+wHfz/H038F48L/GCIf5l5n+G9wG+m+fvpYHfAo7zvMQLhviXmf9+7wN8N8/fSwO/BRzn+RMvGOJfZv57vQ/w3Tx/Lw38FnCcF0y8YIh/mfnv8z7Ad/P8vTTwW8BxXjjxgiH+Zea/x/sA383z99LAbwHH+ZeJFwzxLzP/9d4H+G6ev5cGfgs4zotGvGCIf5l54S4BtwIvxX+M9wG+m+fvpYHfAo7zohMvGOJfZl6wS8BrA7cCvw28FP8+7wN8N8/fSwO/BRznX0e8YIh/mXn+LgGvDfw1VxwHfht4Kf5t3gf4bp6/lwZ+CzjOv554wRD/MvO8LgGvDfw1z+k48NvAS/Gv8z7Ad/P8vTTwW8Bx/m3EC4b4l5nn9dPA2/D8HQd+G3gpXjTvA3w3z99LA78FHOffTrxgiH+Zef6+G3gfnr/jwG8DL8UL9z7Ad/P8vTTwW8Bx/n3EC4b4l5kX7LuB9+H5Ow78NvBSPH/vA3w3z99LA78FHOffT7xgiH+ZeeG+G3gfnr/jwG8DL8Vzeh/gu3n+Xhr4LeA4/zHEC4b4l5l/2XcD78Pzdxz4beCluOJ9gO/m+Xtp4LeA4/zHES8Y4l9mXjTfDbwPz99x4LeBrwa+m+fvpYHfAo7zH0u8YIh/mXnRfTfwPvzrvTTwW8Bx/uOJFwzxLzP/Ot8NvA8vupcGfgs4zn8O8YIh/mXmX++7gffhX/bSwG8Bx/nPI14wxL/M/Nt8N/A+vGAvDfwWcJz/XOIFQ/zLzL/ddwPvw/N6aeC3gOP85xMvGOJfZv59vht4H57tpYHfAo7zX0O8YIh/mfn3+27gfYCXBn4LOM5/HfGCIf5l5j/GTwOvDRznv5Z4wRD/sluBB/G/098AL80LhviXfTXwUfzv9DXAR/OCIf5lx4FbgWP873IJeDCwywuGeNG8NPDbwDH+d7gEvDbw17xwiBfdceC7gbfif7afAd4b2OVfhvjXe2vgu4Fj/M9yCXhv4Kd50SH+bY4D3w28Ff8z/Azw3sAu/zqIf5+3Br4bOMZ/j0vAewM/zb8N4t/vOPDdwFvxX+tngPcGdvm3Q/zHeWvgu4Fj/Oe6BLw38NP8+yH+Yx0Hvht4K/5z/Azw3sAu/zEQ/zneGvhu4Bj/MS4B7w38NP+xEP95jgPfDbwV/z4/A7w3sMt/PMR/vrcGvhs4xr/OJeC9gZ/mPw/iv8Zx4LuBt+JF8zPAewO7/OdC/Nd6a+C7gWM8f5eA9wZ+mv8aiP96x4HvBt6K5/QzwHsDu/zXQfz3eWvgu7nivYGf5r8e4r/Xca7Y5b8H4v83xP9v/CNkpa9B1zkH4wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBeenhere;
impl IconShape for MdBeenhere {
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
                d: "M19 1H5c-1.1 0-1.99.9-1.99 2L3 15.93c0 .69.35 1.3.88 1.66L12 23l8.11-5.41c.53-.36.88-.97.88-1.66L21 3c0-1.1-.9-2-2-2zm-9 15l-5-5 1.41-1.41L10 13.17l7.59-7.59L19 7l-9 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBikeScooter;
impl IconShape for MdBikeScooter {
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
                d: "M10,14h0.74L8.82,5.56C8.61,4.65,7.8,4,6.87,4H3v2h3.87l1.42,6.25c0,0-0.01,0-0.01,0C6.12,12.9,4.47,14.73,4.09,17H0v2h6 v-1C6,15.79,7.79,14,10,14z",
            }
            path {
                d: "M19,8h-0.82l-1.35-3.69C16.55,3.52,15.8,3,14.96,3H11v2h3.96l1.1,3H10.4l0.46,2H15c-0.43,0.58-0.75,1.25-0.9,2h-2.79 l0.46,2h2.33c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5C24,10.2,21.8,8,19,8z M19,16c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,16,19,16z",
            }
            path {
                d: "M10,15c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S11.66,15,10,15z M10,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S10.55,19,10,19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b41zkOvBXw1sCDgZfmf4a/Bm4Ffhr4GWCXFw3iRfdRwGcDx/mfbRf4bOBr+Jch/mXHgZ8CXpv/XX4beBtglxcM8cIdB34LeGn+d/pr4HWAXZ4/xAv3U8Bb87/bdwPvw/OHeMFeG/gt/m94HeC3eV6IF+yvgJfm/4bfBl6H54V4/h4MPJ3/Wx4C3MpzQjx/Hw18Ff+3fAzw1TwnxPP32cBn8X/L5wCfzXNCPH8/DbwV/7f8DPDWPCfE8/fZwGfxf8vnAJ/Nc0I8fx8NfBX/t3wM8NU8J8Tz99LAX/F/y8sAf81zQrxgtwIP4v+GZwAP5nkhXrD3Br6L/xveB/hunhfiBTsO/DbwUvzv9jfAawO7PC/EC/dg4K+BY/zvdAl4aeBWnj/Ev+ylgZ8GHsT/Ls8A3hr4a14wxIvmOPDTwGvxv8PvAG8N7PLCIV50rw38Fv87vA7w2/zLEC+61wZ+i3+7vwE+mhfNVwMvxb/d6wC/zb8M8aJ7beC3+Lf7HeC1edH8NvBa/Nu9DvDb/MsQL7rXBn6Lf7vfAV6bF81vA6/Fv93rAL/Nvwzxontt4Lf4t/sd4LV50fw28Fr8270O8Nv8yxAvutcGfot/u98BXpsXzW8Dr8W/3esAv82/DPGie23gt/i3+x3gtXnR/DbwWvzbvQ7w2/zLEC+61wZ+i3+73wFemxfNbwOvxb/d6wC/zb8M8aJ7beC3+Lf7HeC1edH8NvBa/Nu9DvDb/MsQL7rXBn6Lf7vfAV6bF81vA6/Fv93rAL/Nvwzxontt4Lf4t/sd4LV50fw28Fr8270O8Nv8yxAvutcGfot/u98BXpsXzW8Dr8W/3esAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQ/zq7wDH+Z7sEHOdFg/jX+Wzgs/if7XOAz+ZFg/jX+27gvfif6XuA9+ZFh/i3eW/go4GX4n+G3wG+G/hu/nUQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EetFIFBhGA5OgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBreakfastDining;
impl IconShape for MdBreakfastDining {
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
                d: "M18,3H6C3.79,3,2,4.79,2,7c0,1.48,0.81,2.75,2,3.45V19c0,1.1,0.9,2,2,2h12 c1.1,0,2-0.9,2-2v-8.55c1.19-0.69,2-1.97,2-3.45C22,4.79,20.21,3,18,3z M14,15h-4v-4h4V15z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/e7w08FX82/0N8NE8J8T/Hq8N/Bb/dr8DvDbPCfG/x2sDv8W/3e8Ar81zQvzv8drAb/Fv9zvAa/OcEP97vDbwW/zb/Q7w2jwnxP8erw38Fs/rd3heLw0c4zn9DvDaPCfE/x6vDfwWz0s8r98GXovn9DvAa/OcEP97vDbwWzwv8bx+G3gtntPvAK/Nc0L87/HawG/xvMTz+m3gtXhOvwO8Ns8J8b/HawO/xfMSz+u3gdfiOf0O8No8J8T/Hq8N/BbPSzyv3wZei+f0O8Br85wQ/3u8NvBbPC/xvH4beC2e0+8Ar81zQvzv8drAb/G8xPP6beC1eE6/A7w2zwnxv8drA7/F8xLP67eB1+I5/Q7w2jwnxP8erw38Fs9LPK/fBl6L5/Q7wGvznBD/e7w28Fs8L/G8fht4LZ7T7wCvzXNC/O/x2sBv8bx+m+f10sBxntPvAK/Nc0L87/HawG/xb/c7wGvznBD/e7w28Fv82/0O8No8J8T/Hq8N/Bb/dr8DvDbPCfG/x2sDv8W/3e8Ar81zQvzv8drAb/Fv9zvAa/OcEP97vDbwW/zb/Q7w2jwnxP8erw38Fv92vwO8Ns8J8b/HawO/xb/d7wCvzXNC/O/x2sBv8W/3O8Br85wQ/3u8NvBb/Nt9DvDZPCfE/x4vDfwV/3afA3w2zwnxv4v5t/sY4Kt5Toj/Xf4aeCn+bR4C3MpzQvzv8tHAV/Gv9wzgwTwvxP8uDwaezr/e5wCfzfNC/Md5aeCreNF8DPDX/Nt8N/BevOguAQ8GdnleiP84rw38Fi+a1wF+m3+bBwNP50X3OcBn8/wh/uO8NvBbvGheB/ht/u0+Gvgq/mV/A7w2sMvzh/iP89rAb/GieR3gt/n3+W7gvXjBLgGvDfw1LxjiP85rA7/Fi+Z1gN/m3+c48NvAS/H8vQ/w3bxwiBfstfjXeWngq3nRfDTw1/zr/A7P6zhwkef1McBX8y9DPH+fDXwW/7N8DvDZPC/zvF4H+G3+ZYjn77eB1+J/lt8BXpvnZZ7X6wC/zb8M8fz9NvBa/M/yO8Br87zM83od4Lf5lyGev98GXov/WX4GeGuel3lerwP8Nv8yxPP31sBP8T/L2wA/zfMyz+t1gN/mX4b43888r9cBfpt/GeJ/P/O8Xgf4bf5liP/9zPN6HeC3+Zch/vczz+t1gN/mX4b43888r9cBfpt/GeL5+2rgpfif5W+Aj+Z5mef118Auz+lvgI/mOSGev98GXov/WX4HeG2el3nR/A7w2jwnxPP328Br8T/L7wCvzfMyL5rfAV6b54R4/n4beC3+Z/kd4LV5XuZF8zvAa/OcEM/fdwPvxf8s3wO8N8/LvGh+B3htnhPi+Xtp4KOBB/M/w63AVwN/zfP6bV40fw18NM8J8f8b4v83xP9viP/fEP+/8Y8ytaFBIH3ZIAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrunchDining;
impl IconShape for MdBrunchDining {
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
                d: "M18,8h2V4h-2V8z M15.51,22H2.49C2.22,22,2,21.78,2,21.5V20h14v1.5 C16,21.78,15.78,22,15.51,22z M18,15.89l-0.4-0.42c-1.02-1.08-1.6-2.52-1.6-4V2h6v9.51c0,1.46-0.54,2.87-1.53,3.94L20,15.97V20h2v2 h-4V15.89z M7,16v-2h4v2h4.5c0.28,0,0.5,0.22,0.5,0.5v1c0,0.28-0.22,0.5-0.5,0.5h-13C2.22,18,2,17.78,2,17.5v-1 C2,16.22,2.22,16,2.5,16H7z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBusAlert;
impl IconShape for MdBusAlert {
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
                d: "M16 1a7 7 0 0 0-5.78 3.05l.02-.03C9.84 4 9.42 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V22a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1h8v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1.78c.61-.55 1-1.34 1-2.22v-3.08A7 7 0 0 0 16 1zM4.5 19a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zM3 13V8h6c0 1.96.81 3.73 2.11 5H3zm10.5 6a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm2.5-6a5 5 0 1 1 0-10 5 5 0 0 1 0 10zm-1-9h2v5h-2zm0 6h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCarRental;
impl IconShape for MdCarRental {
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
                d: "M16.39,9H7.61C7.18,9,6.8,9.28,6.66,9.68l-1.66,5v6.81C5,21.78,5.23,22,5.5,22h1C6.78,22,7,21.78,7,21.5V20h10v1.5 c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-6.81l-1.66-5C17.2,9.28,16.82,9,16.39,9z M7.78,18 c-0.68,0-1.22-0.54-1.22-1.22s0.54-1.22,1.22-1.22S9,16.11,9,16.78S8.46,18,7.78,18z M16.22,18C15.55,18,15,17.46,15,16.78 s0.54-1.22,1.22-1.22s1.22,0.54,1.22,1.22S16.9,18,16.22,18z M6.29,14l1.33-4h8.78l1.33,4H6.29z",
            }
            path {
                d: "M10.83,3C10.41,1.83,9.3,1,8,1C6.34,1,5,2.34,5,4c0,1.65,1.34,3,3,3c1.3,0,2.41-0.84,2.83-2H16v2h2V5h1V3H10.83z M8,5 C7.45,5,7,4.55,7,4s0.45-1,1-1s1,0.45,1,1S8.55,5,8,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFG0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j3Mc+CrgvfnP9d3AxwC7/Psh/mMcB34LeGn+a/w18DrALv8+iP8YHw18FVf8DbDLf47jwEtxxfsA382/D+I/xl8BL80VJ4Bd/nMcBy5yxV8DL8O/D+Lf78HA07niZ4C35j/XTwNvxRUPAW7l3w7x7/fRwFdxxfsA381/rvcGvosrPgb4av7tEP9+fwW8NFecAHb5z3UcuMgVfw28DP92iH+fBwNP54qfAd6a/xo/DbwVVzwEuJV/G8S/z0cDX8UV7wN8N/813hv4Lq74GOCr+bdB/Pv8FfDSXHEC2OW/xnHgIlf8NfAy/Nsg/u0eDDydK34GeGv+a/008FZc8RDgVv71EP92Hw18FVe8D/Dd/Nd6b+C7uOJjgK/mXw/xb/dXwEtzxQlgl/9ax4GLXPHXwMvwr4f4t3kw8HSu2AX+mv8eLw0c54qHALfyr4P4t/lo4Kv4n+VjgK/mXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F8/ob4KeBBwNvDRzjP8Yl4KeBW4G3Bl6K5/U7wGvzr4P4t/lt4LV4Tp8DfDbP9mDgt4EH8e/zDOC1gVt5ts8GPovn9DvAa/Ovg/i3+W3gtXi2vwFemuf11sBP8e/zNsBP87z+Gngpnu13gNfmXwfxb/PbwGvxbJ8DfDbP6zhwkX8f8fx9NvBZPNvvAK/Nvw7i3+a3gdfi2b4HeG+e10sDf8W/z0OAW3le3w28F8/2O8Br86+D+Lf5beC1eLZd4GWAW3lOXw18FP8+XwN8NM/pwcBfAcd5tt8BXpt/HcS/zW8Dr8VzuhX4aOB3gAcD7wV8NP8xvhr4HuCvgbcCvhp4MM/pd4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tfht4LZ7T3wAfzX+NrwZeiuf0O8Br86+D+Lf5beC1eE6/A7w2/zV+G3gtntPvAK/Nvw7i3+a3gdfiOf0O8Nr81/ht4LV4Tr8DvDb/Ooh/m58G3orn9DPAW/Nf46eBt+I5/Qzw1vzrIP5t3hv4Lp7T+wDfzX+N9wa+i+f0PsB386+D+Ld7b+CtueKnge/mv9Z7A2/NFT8NfDf/eoj/3xD/vyFedC8NfBX/O3wM8Nf8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie2ngq/mP8dLAMZ7TJeCv+Y/x0cBf8y9D/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf46uBl+Y5/TXw0fzXQvz/hvj/DfH/G+L/N8T/b/wj4p6qQc4xaHQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCarRepair;
impl IconShape for MdCarRepair {
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
                d: "M16.22,12c0.68,0,1.22-0.54,1.22-1.22c0-0.67-0.54-1.22-1.22-1.22S15,10.11,15,10.78C15,11.46,15.55,12,16.22,12z M6.56,10.78c0,0.67,0.54,1.22,1.22,1.22S9,11.46,9,10.78c0-0.67-0.54-1.22-1.22-1.22S6.56,10.11,6.56,10.78z M7.61,4L6.28,8h11.43 l-1.33-4H7.61z M16.28,3c0,0,0.54,0.01,0.92,0.54c0.02,0.02,0.03,0.04,0.05,0.07c0.07,0.11,0.14,0.24,0.19,0.4 C17.66,4.66,19,8.69,19,8.69v6.5c0,0.45-0.35,0.81-0.78,0.81h-0.44C17.35,16,17,15.64,17,15.19V14H7v1.19C7,15.64,6.65,16,6.22,16 H5.78C5.35,16,5,15.64,5,15.19v-6.5c0,0,1.34-4.02,1.55-4.69c0.05-0.16,0.12-0.28,0.19-0.4C6.77,3.58,6.78,3.56,6.8,3.54 C7.18,3.01,7.72,3,7.72,3H16.28z M4,17.01h16V19h-7v3h-2v-3H4V17.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vR7MFbfy3wPx3+u3uOJ1+O+B+O/z1sBPccXbAD/Nfz3Ef4/jwF8BD+aKW4GXAXb5r4X47/HZwGfxnD4H+Gz+ayH+67008Fc8fy8D/DX/dRD/9X4LeG2ev98GXof/Ooj/Wu8NfBcv3PsA381/DcR/nePA04HjvHC7wEOAXf7zIf7rfDXwUbxovgb4aP7zIf5rvDbwW/zrvA7w2/znQvzX+CvgpfnX+WvgZfjPhfjP99HAV/Fv8zHAV/OfB/Gf68HAXwHH+bfZBR4C7PKfA/Gf66eAt+bf56eBt+E/B+I/z2sDv8V/jNcBfpv/eIj/PE8HHsx/jFuBh/AfD/Gf47OBz+I/1ucAn81/LMR/vAcDT+c/3i7wMsCt/MdB/Mf7LeC1+c/x28Dr8B8H8R/rrYGf4j/X2wA/zX8MxH+c48BfAQ/mP9etwMsAu/z7If7jfDXwUfzX+Bzgs/n3Q/zHeGngr/iv9TLAX/Pvg/iP8WDgwfzXuhW4lX8fxP9viP/fEP+/If5/Q/zv82DgQTzbJeCv+bdB/O/wYOCjgLcGHszz2gV+Gvge4Ld50SH+ZeY/3usAv82/7DjwVcB786L7beBjgL/mX4b4l5n/eK8D/DYv3EsD3wW8NP96u8DHAN/NC4f4l5n/eK8D/DYv2EsDvwUc59/nfYDv5gVD/MvMf7zXAX6b5+848FvAS/Mf43WA3+b5Q/zLzH+81wF+m+fvu4H34j/OrcBDeP4Q/zLzH+91gN/meT0YeDr/8T4H+GyeF+JfZv7jvQ7w2zyvrwY+iv94twIP4Xkh/mXmP97rAL/N83o68GD+c7wM8Nc8J8S/zPzHex3gt3lODwaezn+ezwE+m+eE+JeZ/3ivA/w2z+m1gd/iP8/nAJ/Nc0L8y8x/vNcBfpvn9NrAb/Gf52eAt+Y5If5l5j/e6wC/zXN6beC3+M/zOcBn85wQ/zLzH+91gN/mOb008Ff85/kc4LN5Toh/mfmP9zrAb/O8doFj/Od4G+CneU6If5n5j/c6wG/zvL4beC/+410CjvO8EP8y8x/vdYDf5nm9NvBb/Mf7HuC9eV6If5n5j/c6wG/z/P028Fr8x3oIcCvPC/EvM//xXgf4bZ6/lwb+iv84nwN8Ns8f4l9m/uO9DvDbvGDvDXwX/37fA7w3LxjiX2b+470O8Nu8cO8NfBf/dt8DfDSwywuG+JeZ/3ivA/w2/7LXBr4beBD/Op8DfDb/MsS/7LX5j/fXwC4vus8G3ht4EC/YJeCngc8GbuVFg/jf5aWBt+aKlwb+miv+Gvhp/vUQ/78h/n9D/P+G+P8N8f8b/wjwSJtB+NcjcQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCategory;
impl IconShape for MdCategory {
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
                d: "M12 2l-5.5 9h11z",
            }
            circle {
                cx: "17.5",
                cy: "17.5",
                r: "4.5",
            }
            path {
                d: "M3 13.5h8v8H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCelebration;
impl IconShape for MdCelebration {
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
            polygon {
                points: "2,22 16,17 7,8",
            }
            path {
                d: "M14.53,12.53l5.59-5.59c0.49-0.49,1.28-0.49,1.77,0l0.59,0.59l1.06-1.06l-0.59-0.59c-1.07-1.07-2.82-1.07-3.89,0 l-5.59,5.59L14.53,12.53z",
            }
            path {
                d: "M10.06,6.88L9.47,7.47l1.06,1.06l0.59-0.59c1.07-1.07,1.07-2.82,0-3.89l-0.59-0.59L9.47,4.53l0.59,0.59 C10.54,5.6,10.54,6.4,10.06,6.88z",
            }
            path {
                d: "M17.06,11.88l-1.59,1.59l1.06,1.06l1.59-1.59c0.49-0.49,1.28-0.49,1.77,0l1.61,1.61l1.06-1.06l-1.61-1.61 C19.87,10.81,18.13,10.81,17.06,11.88z",
            }
            path {
                d: "M15.06,5.88l-3.59,3.59l1.06,1.06l3.59-3.59c1.07-1.07,1.07-2.82,0-3.89l-1.59-1.59l-1.06,1.06l1.59,1.59 C15.54,4.6,15.54,5.4,15.06,5.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/3mvxwv0O/3UQ/3U+C/ho4Dgv3C7w1cDn8J8P8V/ju4D35l/nu4H34T8X4j/fewPfxb/N+wDfzX8exH++vwJemn+b3wFem/88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+YxwH3gp4MFe8Ns/22vz7/DbP9ttccSvwM8Au/z6If5/XBj4KeGv+e/w08DXAb/Nvg/i3eWngq4DX5n+G3wY+Bvhr/nUQ/3qvDfwUcJz/WXaBtwF+mxcd4l/nrYGf4n+2twF+mhcN4kX30sBvAcd5wZ4B3Mp/rgcDD+IF2wVeB/hr/mWIF93TgQfz/P0M8NnAX/Nf46WBzwbeiufvVuAh/MsQL5qPBr6K53UJ+Gjgu/nv8d7AVwPHeF7vA3w3LxziRXMROM7zehvgp/nv9dbAT/G8doETvHCIf9lbAz/F8/oc4LP5n+Gzgc/ieb0N8NO8YIh/2VcDH8VzugQ8GNjlf4bjwK3AMZ7T1wAfzQuG+Jf9NvBaPKefAd6a/1l+GngrntPvAK/NC4b4lz0deDDP6XOAz+Z/ls8GPovndCvwEF4wxL/MPK+3AX6a/1neGvgpnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5nkdB94LeGngZ4Cf5gV7aeC9uOJ7gL/mBXtr4K2Avwa+B9jleb028Fs8L/GCIf5l5nm9DvDbPK+/Al6aZ/sc4LN5Xi8N/BXP6WWAv+Z5fTbwWTzbXwMvw/N6beC3eF7iBUP8y8zzeh3gt3lOrw38Fs/pVuAhPK+vBj6K5/Q1wEfzvC4Cx3lOrwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL3tt/nf7bV4wxP9viP/f+EeiKb9BHyODJAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCleaningServices;
impl IconShape for MdCleaningServices {
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
                d: "M16,11h-1V3c0-1.1-0.9-2-2-2h-2C9.9,1,9,1.9,9,3v8H8c-2.76,0-5,2.24-5,5v7h18v-7C21,13.24,18.76,11,16,11z M19,21h-2v-3 c0-0.55-0.45-1-1-1s-1,0.45-1,1v3h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H9v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H5v-5 c0-1.65,1.35-3,3-3h8c1.65,0,3,1.35,3,3V21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/dawEsDx4HX5ooHAw/mOd0K3MoVvw3sAn8N/A7/eRD/8V4aeCvgtYHX5j/GbwO/DfwM8Nf8x0H8x3gw8FbAewMvzX+uvwa+G/gZ4Fb+fRD/Pg8GPgt4b/57fDfwOcCt/Nsg/m0eDHwW8N78z/DdwOcAt/Kvg/jXOQ58FPDZ/M/02cDXALu8aBAvutcGvgt4MP+z3Qq8D/Db/MsQL5rPAj6b/10+G/gcXjjEC3cc+Cngtfnf6beBtwF2ef4QL9hx4LeAl+Z/t78G3ga4leeFeP5eGvgt4Dj/N+wCrwP8Nc8J8fy9NPDbwDH+b7gEvDbw1zwnxAv20sBvA8f43+0S8NrAX/O8EC/cSwO/DRzjf6dLwGsDf83zh/iXvTTw28Ax/ne5BLw28Ne8YIgXzUsDvw0c43+HS8BrA3/NC4d40b008NvAMf5nuwS8NvDX/MsQ/zovDfw2cIz/mS4Brw38NS8axL/eSwO/DRzj3+8SVxzj3+8S8NrAX/OiQ/zbvDTw28AxXnR/A3w38NfAXwO7PKfjwEsDLw28N/BSvOguAa8N/DX/Ooh/u5cGfhs4xgt2Cfhs4LuBXf51jgNvDXw1cIwX7BLw2sBf86+H+Pd5aeC3gWM8p0vAVwNfDezy73Mc+Gjgo4FjPKdLwGsDf82/DeLf76WB3waOccXfAG8N3Mp/rAcDPw28FFdcAl4b+Gv+7RD/MV4a+G3gt4H3Bnb5z3Ec+G7gtYHXBv6afx/Ef5wHA7fyX+PBwK38+yH+f0P893ktntPv8F8P8V/rrYH3At6a5++nge8Bfpr/Goj/Gi8NfBXw2rxofhv4GOCv+c+F+M/30sBvAcf519kFXgf4a/7zIP5zvTTwW8Bx/m12gdcB/pr/HIj/PMeBvwIezL/PrcBD+M+B+M/z2cBn8R/jc4DP5j8e4j/PReA4/zF2gRP8x0P853ht4Lf4j/UywF/zHwvxn+Ozgc/iP9bnAJ/NfyzEf47PBj6L/1ifA3w2/7EQ/zl+Gngr/mP9DPDW/MdC/Of4bOCz+I/1OcBn8x8L8Z/jo4Gv4j/WxwBfzX8sxH+OBwNP5z/WQ4Bb+Y+F+M/z28Br8R/jd4DX5j8e4j/PawO/xX+M1wF+m/94iP9c3w28F/8+3wO8N/85EP+5jgO/DbwU/zZ/A7w2sMt/DsR/vuPATwOvxb/O7wBvDezynwfxX+ejgc8GjvHCXQI+G/hq/vMh/msdB94aeGvgwcBLccXfALcCPw38NLDLfw3E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFSNpZB4D7XqAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCompassCalibration;
impl IconShape for MdCompassCalibration {
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
                cx: "12",
                cy: "17",
                r: "4",
            }
            path {
                d: "M12 10.07c1.95 0 3.72.79 5 2.07l5-5C19.44 4.59 15.9 3 12 3S4.56 4.59 2 7.15l5 5c1.28-1.28 3.05-2.08 5-2.08z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zN9NfBSvGC7wF8DPwP8Nf92iP+Zfht4LV40Xw18DP82iP+Zfht4LV50XwN8NP96iH+b3wZei3+d3wFemxfNbwOvxb/OQ4Bb+ddB/Nv8NvBa/Ov8DvDavGh+G3gt/nU+B/hs/nUQ/za/DbwW/zq/A7w2L5qvBl6aF+ylgWM8p58B3pp/HcS/zW8Dr8W/zu8Ar81/jN8GXovn9DvAa/Ovg/i3+W3gtfjX+R3gtfmP8dvAa/Gcfgd4bf51EP82Lw0c519nF/hr/mP8NvBaPKffAV6bfx3E/06/DbwWz+l3gNfmXwfxv9NvA6/Fc/od4LX510H86xwH3gp4a+DBwEvzovsd4LX5j/HbwGvxnH4HeG3+dRAvuo8CPhs4zr/N7wCvzX+M3wZei+f0O8Br86+D+JcdB34KeG3+fX4HeG3+/d4a+Cme1+8Ar82/DuKFOw78FvDS/Pv9DvDa/Pu8NPBbwHGe188Ab82/DuKF+yngrfmP8TvAa/Nvdxz4LeCleV7PAF4a2OVfB/GCvTbwW/zH+R3gtfm3+yngrXlel4DXBv6afz3EC/ZXwEvzH+d3gNfm3+azgc/i+Xsf4Lv5t0E8fw8Gns5/rN8BXpt/vbcGforn72uAj+bfDvH8fTTwVfzH+h3gtfnXeWngt4DjPK/fAV6bfx/E8/fZwGfxH+t3gNfmRXcc+C3gpXlezwBeGtjl3wfx/P008Fb8x/od4LV50f0U8NY8r0vAawN/zbO9Fs/2DOBWXjSI5++3gdfiP9bvAK/Ni+azgc/i+Xsf4LuBBwOfBbw3z+uvgc8BfpoXDvH8/TbwWvzH+h3gtfmXvTXwUzx/XwN8NPDewHfxL/tu4GOAXZ4/xPP328Br8R9rF/hr/mUvDRznef0O8NrAewPfxYvup4G34flDPH+/DbwW/3M8A3hp4DjwdP71Pgf4bJ4X4vn7beC1+J/hEvDawF8D3w28F/96u8BDgF2eE+L5+23gtfif4X2A7waOAxd5XpeAjwa+G3hp4LuBl+J5vQ3w0zwnxPP328Br8d/va4CP5orXBn6L5/U+wHfzbMeBW4FjPKfPAT6b54R4/r4aeGme03HgpXhev8PzemngGM/pEvDXvOhuBd6bZ3tt4Ld4XuJ5/TbwWjynnwHemueEeNG9NvBbPK+XAf6a5/RXwEvznD4H+Gz+7V4b+C2e18sAf81z+ivgpXlOnwN8Ns8J8aJ7MPB0ntdfA68D7HLFZwGfzfP6GOCr+bd7MPB0ntdfA68D7HLFZwGfzfP6GOCreU6If52/Bl6K57UL/DVwHHhpnr+HALfy7/PXwEvxvHaBvwaOAy/N8/cQ4FaeE+Jf562Bn+Jf73uA9+bf762Bn+Jf73uA9+Z5If71vht4L150l4AHA7v8x/hu4L140V0CHgzs8rwQ/3rHgZ8GXot/2SXgtYG/5j/OceCngdfiX3YJeG3gr3n+EP92nw18NHCM5+97gI8GdvnP8dnARwPHeP6+B/hoYJcXDPHvcxx4beClgZcG/hrYBX4auJX/fMeB1wZeGnhp4K+BXeCngVv5lyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFpa8FBH5ABmwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeliveryDining;
impl IconShape for MdDeliveryDining {
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
                d: "M19,7c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,14H10V9H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35V7 z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
            }
            rect {
                height: "2",
                width: "5",
                x: "5",
                y: "6",
            }
            path {
                d: "M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,17,19,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDepartureBoard;
impl IconShape for MdDepartureBoard {
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
                d: "M16 1c-2.4 0-4.52 1.21-5.78 3.05.01-.01.01-.02.02-.03C9.84 4 9.42 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V22c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h8v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1.78c.61-.55 1-1.34 1-2.22v-3.08c3.39-.49 6-3.39 6-6.92 0-3.87-3.13-7-7-7zM4.5 19c-.83 0-1.5-.67-1.5-1.5S3.67 16 4.5 16s1.5.67 1.5 1.5S5.33 19 4.5 19zM3 13V8h6c0 1.96.81 3.73 2.11 5H3zm10.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm2.5-6c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm.5-9H15v5l3.62 2.16.75-1.23-2.87-1.68z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDesignServices;
impl IconShape for MdDesignServices {
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
                d: "M16.24,11.51l1.57-1.57l-3.75-3.75l-1.57,1.57L8.35,3.63c-0.78-0.78-2.05-0.78-2.83,0l-1.9,1.9 c-0.78,0.78-0.78,2.05,0,2.83l4.13,4.13L3,17.25V21h3.75l4.76-4.76l4.13,4.13c0.95,0.95,2.23,0.6,2.83,0l1.9-1.9 c0.78-0.78,0.78-2.05,0-2.83L16.24,11.51z M9.18,11.07L5.04,6.94l1.89-1.9c0,0,0,0,0,0l1.27,1.27L7.02,7.5l1.41,1.41l1.19-1.19 l1.45,1.45L9.18,11.07z M17.06,18.96l-4.13-4.13l1.9-1.9l1.45,1.45l-1.19,1.19l1.41,1.41l1.19-1.19l1.27,1.27L17.06,18.96z",
            }
            path {
                d: "M20.71,7.04c0.39-0.39,0.39-1.02,0-1.41l-2.34-2.34c-0.47-0.47-1.12-0.29-1.41,0l-1.83,1.83l3.75,3.75L20.71,7.04z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5l/v58Gvgb4bf5nQfzLzH+c9wG+m/85EP+y3+aK48BLccXfALtc8VpccQn4a57Xa/GcXgf4bf5nQLzoXhv4La54HeC3ucJc8TvAa/O8zHO6FXgI/zMgXnSvDfwWV7wO8NtcYa74HeC1eV7meb0M8Nf890P8y36LK44DL80Vfw3scsVrc8Uu8Nc8r9fmef01sMt/jdfhBUP8y8z/buIFQ/zLzP9u4gVDvOheG/gtrngd4Le5wlzxO8Br87zM83od4Lf574d40b028Ftc8TrAb3OFueJ3gNfmeZnndAk4zv8MiH/Za3HFSwNfzRUfDfw1V/w2V/w18NE8r9/mOX0O8Nn8z4D4l5n/OH8DvDT/cyD+ZeY/xvcAHw3s8j8H4l/22lzx0sBXccXHAH/NFb/FFX8DfDTP363ArfzPg3jRvTbwW1zxOsBvc4W54neA1+Z/F8SL7rWB3+KK1wF+myvMFb8DvDb/uyBedK8N/BZXvA7w21xhrvgd4LX53wXxontt4Le44nWA3+YKc8XvAK/N/y6IF91rA7/FFa8D/DZXmCt+B3ht/ndBvOheG/gtrngd4Le5wlzxO8Br878L4kX32sBvccXrAL/NFeaK3wFem/9dEC+61wZ+iyteB/htrjBX/A7w2vzvgnjRvTbwW1zxOsBvc4W54neA1+Z/F8SL7rWB3+KK1wF+myvMFb8DvDb/uyBedK8N/BZXvA7w21xhrvgd4LX5tzkOvBXw2sCDgdfm2X6bK34a+BngVv7jIF50rw38Fle8DvDbXGGu+B3gtfnXeTDwWcBbA8d50fw28DnAb/Pvh3jRvTbwW1zxOsBvc4W54neA1+ZF91nARwPH+bf5beBtgF3+7RAvutcGfosrXgf4ba4wV/wO8Nr8y44D3wW8Nf9+u8DrAH/Nvw3iRffawG9xxesAv80V5orfAV6bF+448FvAS/MfZxd4G+C3+ddDvOheG/gtrngd4Le5wlzxO8Br88L9FvDa/MfbBV4GuJV/HcSL7rWB3+KK1wF+myvMFb8DvDYv2EcDX8V/nr8GXoZ/HcSL7rWB3+KK1wF+myvMFb8DvDbP34OBp/Of732A7+ZFh3jRvTbwW1zxOsBvAy8N/BXP66+BXeCngZ8BPht4L/7z7QIPAXZ50SBedK8N/BZXfDbw2sBr8z/P+wDfzYsG8aJ7beC3+J/vZ4C35kWDeNG9NvBb/O8gXjSIF91PAW/N/w4vA/w1/zLEi+a9ge/if4/XAX6bfxniRXMROM7/Hq8D/Db/MsS/7L2B7+J/l9cBfpt/GeL/N8T/b4jn78HAg/i/5RnArTwnxPN3HPht4KX4v+FvgNcGdnlOiBfsOPDbwEvxv9vfAK8N7PK8EC/cceC3gZfif6e/AV4b2OX5Q/zLjgO/DbwU/7v8DfDawC4vGOJFcxz4beCl+N/hb4DXBnZ54RD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R6KHxEHgP9N3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDinnerDining;
impl IconShape for MdDinnerDining {
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
                d: "M2,19h20l-2,2H4L2,19z M5,6h1v1H5V6z M5,4h1v1H5V4z M9,4v1H7V4H9z M9,7H7V6h2V7z M6,15.23c-0.36,0.11-0.69,0.28-1,0.47V8h1 V15.23z M4,16.52C3.62,16.96,3.32,17.45,3.16,18h16.82c0.01-0.16,0.03-0.33,0.03-0.5c0-3.04-2.46-5.5-5.5-5.5 c-2.29,0-4.25,1.4-5.08,3.4C8.84,15.15,8.19,15,7.5,15c-0.17,0-0.33,0.02-0.5,0.04V8h2c1.03,0.06,1.9-0.96,2-2h10V5H11 c-0.1-1.05-0.97-1.97-2-2H3v1h1v1H3v1h1v1H3v1h1V16.52z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFTElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fV4aeC+u+B7gr/mvh/jv8d7Ad/Gc3gf4bv5rIf7rvTfwXTx/7wN8N/91EP+13hv4Ll649wG+m/8aiP867w18Fy+a9wG+m/98iP8a7w18F/867wN8N/+5EP/53hv4Lv5t3gf4bv7zIP5zvTfwXfz7vA/w3fznQPzneW/gu/iP8T7Ad/MfD/Gf472B7+I/1vsA381/LMR/vPcGvov/HO8DfDf/cRD/sd4b+C7+c70P8N38x0D8x3lv4Lv4r/E+wHfz74f4j/HewHfxX+t9gO/m3wfx7/fewHfx3+N9gO/m3w7x7/PewHfxH+9jgM8GjvEvex/gu/m3QfzbvTfwXfzneB1gF/ht4Bj/svcBvpt/PcS/zXsD38V/ntcBfht4aeC3gWP8y94H+G7+dRD/eu8NfBf/uV4H+G2ueGngt4Fj/MveB/huXnSIf533Br6L/3yvA/w2z/bSwG8Dx/iXvQ/w3bxoEC+69wa+i/8arwP8Ns/ppYHfBo7xL3sf4Lv5lyFeNO8NfBf/er/Dv81HA3/N83pp4LeBY/zL3gf4bl44xL/svYHv4l/nEvDawF/zH++lgd8GjvEvex/gu3nBEC/cewPfxb/e1wAfzX+elwZ+GzjGv+x9gO/m+UO8YO8NfBf/Nq8D/Db/uX4KeGteNO8DfDfPC/H8vTTwV/zbvQ7w2/zn+S7gvfnXeRngr3lOiOfvq4GP4t/udYDf5j/HdwHvzb/e1wAfzXNCPH+fDXwW/3avA/w2z+m1gd/iRfM6wG/zvL4LeG/+bb4G+GieE+L5ezDw18Ax/m1eB/htntNrA7/Fi+Z1gN/mOX0X8N78270M8Nc8J8QL9tLAbwPH+Nd7HeC3eU6vDfwWL5rXAX6bZ/su4L35t3sf4Lt5XogX7qWB3waO8a/zOsBv85xeG/gtXjSvA/w2V3wX8N78270P8N08f4h/2UsDvw0c40X3OsBv85xeG/gtXjSvA/w28F3Ae/Nv9z7Ad/OCIV40Lw38NnCMF83rAL/Nc3pt4Ld40bwO8F7Ae/Nv9z7Ad/PCIV50Lw38NnCMf9nrAL/Nc3pt4Ld40fw18NL8270P8N38yxD/Oi8N/DZwjBfudYDf5jm9NvBb/Od7H+C7edEg/vVeGvht4Bj/87wP8N286BD/Ni8N/DZwjP853gf4bv51EP92Lw38NnCM/37vA3w3/3qIf5+XBn4bOMZ/n/cBvpt/G8S/30sDvw0c47/e+wDfzb8d4j/GSwO/DRzjv877AN/Nvw/iP85LA78NHOM/3/sA382/H+I/1ksDvw0c4z/P+wDfzX8MxH+8lwZ+GzjGf7z3Ab6b/ziI/xwvDfw2cIz/OO8DfDf/sRD/eV4a+G3gGP9+7wN8N//xEP+5Xhr4beAY/3bvA3w3/zkQ//leGvht4Bj/eu8DfDf/eRD/NV4a+G3gGC+69wG+m/9ciP86Lw38NnCMf9n7AN/Nfz7Ef62XBn4bOMYL9j7Ad/NfA/Ff76WB3waO8bzeB/hu/usg/nu8NPDbwDGuuAR8NPDd/NdC/Pd5MPDeXPHdwK3810P8/4b4/w3x/xvi/zfE/2/8I1U6okFq+2l5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirections;
impl IconShape for MdDirections {
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
                d: "M21.71 11.29l-9-9c-.39-.39-1.02-.39-1.41 0l-9 9c-.39.39-.39 1.02 0 1.41l9 9c.39.39 1.02.39 1.41 0l9-9c.39-.38.39-1.01 0-1.41zM14 14.5V12h-4v3H8v-4c0-.55.45-1 1-1h5V7.5l3.5 3.5-3.5 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsBike;
impl IconShape for MdDirectionsBike {
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
                d: "M15.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5zm5.8-10l2.4-2.4.8.8c1.3 1.3 3 2.1 5.1 2.1V9c-1.5 0-2.7-.6-3.6-1.5l-1.9-1.9c-.5-.4-1-.6-1.6-.6s-1.1.2-1.4.6L7.8 8.4c-.4.4-.6.9-.6 1.4 0 .6.2 1.1.6 1.4L11 14v5h2v-6.2l-2.2-2.3zM19 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsBoat;
impl IconShape for MdDirectionsBoat {
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
                d: "M20 21c-1.39 0-2.78-.47-4-1.32-2.44 1.71-5.56 1.71-8 0C6.78 20.53 5.39 21 4 21H2v2h2c1.38 0 2.74-.35 4-.99 2.52 1.29 5.48 1.29 8 0 1.26.65 2.62.99 4 .99h2v-2h-2zM3.95 19H4c1.6 0 3.02-.88 4-2 .98 1.12 2.4 2 4 2s3.02-.88 4-2c.98 1.12 2.4 2 4 2h.05l1.89-6.68c.08-.26.06-.54-.06-.78s-.34-.42-.6-.5L20 10.62V6c0-1.1-.9-2-2-2h-3V1H9v3H6c-1.1 0-2 .9-2 2v4.62l-1.29.42c-.26.08-.48.26-.6.5s-.15.52-.06.78L3.95 19zM6 6h12v3.97L12 8 6 9.97V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/eV4aOAY8GHgwz3YceGmev78Gdnm2W4FbgUvAX/MfD/Ef4zjwVsBrAy8NvDT/Of4a+Gvgt4GfAXb590H8+zwY+Czgvfnv8d3A5wC38m+D+Lc5DnwW8NH8z/DVwMfwr4f41zsO/Bbw0vzP8tfA6wC7vOgQ/3q/Bbw2/zP9NvA6vOgQ/zpvDfwU/7O9DfDTvGgQ/zq/Bbw2/7P9NvA6vGgQL7rjwEX+dxAvGsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2el/jvZZ7X6wC/zb8M8aJ7beC3eF7iv5d5Xq8D/Db/MsSL7rWB3+J5if9e5nm9DvDb/MsQL7rXBn6L5yX+e5nn9TrAb/MvQ7zoXhv4LZ6X+O9lntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ief02/71em+f1OsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fu8YN8D/DVXvDTwXvzH+h7gr7nipYH34gV7HeC3+ZchXnSvDfwWz9/rAL/Nc3pt4Lf4j/E6wG/znF4b+C2ev9cBfpt/GeJF99rAb/G8Pgf4bJ6/zwY+i3+fzwE+m+fvs4HP4nm9DvDb/MsQL7rXBn6L5/UywF/z/L008Ff8+7wM8Nc8fy8N/BXP63WA3+ZfhnjRvTbwWzwv8cKZfx/xwpnn9TrAb/MvQ7zoXhv4LZ7X6wC/zfP32sBv8e/zOsBv8/y9NvBbPK/XAX6bfxniRffawG/xvH4beB2ev98CXpt/n98GXofn77eA1+Z5vQ7w2/zLEC+6BwNP5/n7buBjgF2uOA58FfDe/Mf4buBjgF2uOA58FfDePH8PAW7lX4b41/lr4KV4wX6bK16b/xy/zRWvzQv2N8BL86JB/Ou8NvBb/M/2OsBv86JB/Ot9N/Be/M/0PcB786JD/Nv8NfBS/M/yN8BL86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8YL9DbDLf6zjwEvxgv0O8Nr86yD+bX4beC1esNcBfpv/WK8N/BYv2O8Ar82/DuLf5reB1+IFex3gt/mP9drAb/GC/Q7w2vzrIP5tvht4L16w1wF+m/9Yrw38Fi/Y9wDvzb8O4t/mrYGf4vm7BDwY2OU/1nHgVuAYz9/bAD/Nvw7i3+6zgfcGHsSz/Q3w3sBf85/jpYHvBl6KZ3sG8N3AZ/Ovh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AllprUEdoMteAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsBus;
impl IconShape for MdDirectionsBus {
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
                d: "M4 16c0 .88.39 1.67 1 2.22V20c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h8v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1.78c.61-.55 1-1.34 1-2.22V6c0-3.5-3.58-4-8-4s-8 .5-8 4v10zm3.5 1c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm9 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6H6V6h12v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEXUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/z4PBt4LeG3+e/w28D3ArfzbIP7t3hv4Lv5neB/gu/nXQ/zbvDTwV/zP8hDgVv51EP823w28F/+zfA/w3vzrIP5tLgLHebaPAf6a/1ovDXwVz7YLnOBfB/Gv99bAT/Fsl4Dj/PfYBY7xbG8D/DQvOsS/3ncD78WzfQ/w3vz3+G7gvXi27wHemxcd4l/nOHCR5/Q2wE/z3+OtgZ/iOZ0AdnnRIP513hv4Lp7tEnCc/167wDGe7X2A7+ZFg/jX+WngrXi27wHem/9e3w28F8/2M8Bb86JBvOiOAxd5Tm8D/DT/vd4a+Cme0wlgl38Z4kX33sB38WyXgOP8z7ALHOPZ3gf4bv5liBfdbwGvzbN9D/De/M/w3cB78Ww/A7w1/zLEi+bBwNN5Tm8D/DT/M7w18FM8p4cAt/LCIV40Hw18Fc92CTjO/yy7wDGe7WOAr+aFQ7xo/gp4aZ7te4D35n+W7wbei2f7a+BleOEQ/7IHA0/nOd0K3Mr/LA8GHsxzeghwKy8Y4l/20cBX8b/TxwBfzQuG+Jf9FfDS/O/018DL8IIh/mXmfzfxgiH+ZeZ/N/GCIf5l5oX7G2AXeDDwIP5rPAO4FTgOvBQvnHjBEP8y8/z9DfDewF/zbB8NfDZwjP8cl4DPBr6aZ3tp4LuBl+L5Ey8Y4l9mntcl4KWBW3leHw18Ff85Pgb4ap7Xg4G/Bo7xvMQLhviXmef1NcBH84LtAsf4j/UM4MG8YF8NfBTPS7xgiH+ZeV7vA3w3L9hvA6/Ff6zfAV6bF+yjga/ieYkXDPEvM8/rc4DP5gX7K+Cl+Y/118DL8IJ9NvBZPC/xgiH+ZeZ5/TXwMjx/Lw38Ff85Xgb4a56/pwMP5nmJFwzxLzPP33cD78NzejDwU8BL85/jr4G3AW7lOX0X8N48f+IFQ/zLzAt2K/DdwC7wYOC9geP859oFvhu4FTgOvDfwYF4w8YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeaF+x3+e70WL5x4wRD/MvPCif9e5oUTLxjiX2ZeOPHfy7xw4gVD/MvMCyf+e5kXTrxgiH/ZdwPvxfP3PcB789/ru4H34vn7HuC9ecEQ/7LjwFcD78Vz+h7go4Fd/nsdB74aeC+e0/cAHw3s8oIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/rAY9BX644cgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsCar;
impl IconShape for MdDirectionsCar {
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
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5h-11c-.66 0-1.21.42-1.42 1.01L3 12v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE40lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/xksDx/jXuQT8Nf+5EP+xXgt4beClgQcDL81/jL8GbgX+Gvht4Hf4j4H493sv4K2Bt+a/1k8D3w38DP92iH+71wa+C3gw/71uBd4H+G3+9RD/Np8FfDb/s3w28Dn86yD+9T4a+Cr+Z3of4Lt50SH+dV4a+Cv+Z3sIcCsvGsS/zncD78X/bN8DvDcvGsS/jvmfbxc4wYsG8aJ7aeCv+N/hZYC/5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gtntfr8N/rt3herwP8Nv8yxIvutYHf4nmJ/17meb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3he4r+XeV6vA/w2/zLEi+61gd/ieYn/XuZ5vQ7w2/zLEC+61wZ+i+cl/nuZ5/U6wG/zL0O86F4b+C2el/jvZZ7X6wC/zb8M8aJ7beC3eF7iv5d5Xq8D/Db/MsSL7rWB3+J5if9e5nm9DvDb/MsQL7rXBn6L5yX+e5nn9TrAb/MvQ7zoXhv4LZ6X+O9lntfrAL/Nvwzxontt4Ld4XuK/l3lerwP8Nv8yxIvutYHf4nm9Nv+9fpvn9TrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/Md5BnArVzwYeBD/cV4H+G3+ZYgX3WsDv8W/3yXgvYGf5jm9NfDdwDH+/V4H+G3+ZYgX3WsDv8W/zyXgwcAuz99x4FbgGP8+rwP8Nv8yxIvutYHf4t/nbYCf5oV7a+Cn+Pd5HeC3+ZchXnSvDfwW/3bPAB7Mi+ZW4EH8270O8Nv8yxAvutcGfot/u98BXpsXzW8Dr8W/3esAv82/DPGie23gt/i3+x3gtXnR/DbwWvzbvQ7w2/zLEC+61wZ+i3+7W4GH8KJ5OvBg/u1eB/ht/mWIF91rA7/Fv8/bAD/NC/fWwE/x7/M6wG/zL0O86I4DF/n32QUeAuzy/L008FvAcf59TgC7/MsQ/zq/DbwW/z67wPsAP81zemvgu4Dj/Pv8DPDWvGgQ/zpvDfwU/zFuBW7ligcDD+Y/xusAv82LBvGv99vAa/E/0+8Ar82LDvGvdxz4a+BB/M/yDOClgV1edIh/m+PAbwMvxf8MfwO8NrDLvw7i3+ezgY8GjvHf4xLw1cBn82+D+I/x1sBLAy8NvBX/uX4G+Gvgr4Gf5t8H8R/nOPBbwEvzn+uvgdcBdvn3Q/zHOA78FvDS/Nf4a+B1gF3+fRD/fseB3wJemv9afw28DrDLvx3i3++vgJfmv8dfAy/Dvx3i38/89xL/doj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I7yfokHvbd+UAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsRailway;
impl IconShape for MdDirectionsRailway {
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
                d: "M4 15.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V5c0-3.5-3.58-4-8-4s-8 .5-8 4v10.5zm8 1.5c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6-7H6V5h12v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/Gd4L+Gjgpbnir4GvBr6H/1yI/36/Bbw2z993A+/Dfx7Ef6+vBj6KF+5zgM/mPwfiv89x4CL/sl3gBP85EP99Xhv4LV40rwP8Nv/xEP99Xhv4LV40rwP8Nv/xEP99Xhr4K140DwFu5T8e4r/XrcCDeOF+B3ht/nMg/nu9NvBbvHAvA/w1/zkQ//3eGvhu4BjP6RnAewO/zX8exP8Mx4G3Bh7MFbcCPw3s8p8L8f8b4r/eawPvBbw28GCu+G3ga4Cf5r8W4r/OewEfDbw0L9hXAx/Dfx3Ef64HA+8FfDRwnBfN+wDfzX8NxH+O1wY+Cnhr/vV2gZcBbuU/H+I/znHgrYDPBh7Mv89PA2/Dfz7Ev9+DgY8C3hs4zn+ctwF+mv9ciH8/85/jVuBlgF3+8yD+/cy/7BnAg/jX+xrgo/nPg/j3My/YzwDfDfw2cJF/m/cBvpv/HIh/P/O8vgb4auBWrnht4Lf4t9kF3gb4bf7jIf79zPN6HeC3ebbPBj6Lf5+PBr6G/1iIfz/zvF4H+G2e7buB9+Lf77eB9wFu5T8G4t/PPK/XAX6bZ/sr4KX5lz0D+Grgq3jhfhr4aeB3gFv5t0P8+5nn9TrAb/Ns5kXzM8BbA58NfBYvml3gr4G/AT6afx3Ev595Xq8D/DZXvDbwW7xoPgf4bK54b+CrgWO8aH4HeG3+dRD/fuZ5vQ7w21zx0cBX8aJ5HeC3ebYHA98NvBb/st8BXpt/HcS/n3lerwP8Nld8N/BevGhOALs8r9cGvhp4KV6w3wFem38dxL+feV6vA/w2V/wV8NL8y54BPJgX7qWBjwZeG3gQz+l3gNfmXwfx7/fZPK/vBm7lCvOi+RngrXnRvTTwYOClueJW4Lv510H853pt4Ld40XwO8Nn810L85/po4Kt40fw18DrALv91EP+5vht4L150fw28D/DX/NdA/Of6K+Cl+dfZBV4H+Gv+8yH+c5l/m13gdYC/5j8X4j/PawO/xfO6BHw08F28cLvA6wB/zX8exH+ejwa+iuf1M8BbA+8NfBcv3C5wgv88iH/ZceClgN/hX+e7gffieX0M8NVc8d7Ad/HCif88iH/ZRwNfxRV/Dfw18NfAXwO/wwv2V8BL87xeB/htnu2lgd8GjvH8if88iH/ZdwPvxQv218BfA38N/A7w11xhnj/xvF4a+G3gGM9L/OdB/Mv+Cnhp/nX+GnhpntfvAK/N8/fWwE/xvMR/HsS/zPzH+Rrgo3n+Xhv4LZ6X+M+DeOFeG/gt/uO8DfDTPH+vDfwWz0v850H8yx4MvDTw0sBrAy8NHOPf5gSwy/P32sBv8bzEfx7Ev82DgZcGXhp4beClgWO8cM8AHswL9trAb/G8xH8exH+cBwMvDbw08NrASwPHeLbvAd6bF+y1gd/ieYn/PIj/XA8GXhp4aeBW4Lt5wV4b+C2el/jPg/if47WB3+J5if88iP85Xhv4LZ6X+M+D+J/jtYHf4nmJ/zyI/zleG/gtnpf4z4P4n+O1gd/ieYn/PIj/OV4b+C2el/jPg/if47WB3+J5if88iP85Xhv4LZ6X+M+D+J/jtYHf4nmJ/zyI/zmOAy/N8/pt/vMg/n9D/P/GPwIKNblBAg0QZAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsRun;
impl IconShape for MdDirectionsRun {
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
                d: "M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFlklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/eV4aOAY8GHgwz3YceGmev78Gdnm2W4FbgUvAX/MfD/Ef4zjwVsBrAy8NvDT/Of4a+Gvgt4GfAXb590H8+zwY+Czgvfnv8d3A5wC38m+D+Lc5DnwW8NH8z/DVwMfwr4f41zsO/Bbw0vzP8tfA6wC7vOgQ/3q/Bbw2/zP9NvA6vOgQ/zpvDfwU/7O9DfDTvGgQ/zq/Bbw2/7P9NvA6vGgQL7rjwEX+dxAvGsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2e12/zr/fSwHGe0y7w1/zrvTbP63WA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3eMG+B/hrrnhp4L34j/U9wF9zxUsD78UL9jrAb/MvQ7zoXhv4LZ6/1wF+m+f02sBv8R/jdYDf5jm9NvBbPH+vA/w2/zLEi+61gd/ieX0O8Nk8f58NfBb/Pp8DfDbP32cDn8Xzeh3gt/mXIV50rw38Fs/rZYC/5vl7aeCv+Pd5GeCvef5eGvgrntfrAL/Nvwzxontt4Ld4XuKFM/8+4oUzz+t1gN/mX4Z40b028Fs8r9cBfpvn77WB3+Lf53WA3+b5e23gt3herwP8Nv8yxIvuOHCR5/XbwOvw/P0W8Nr8+/w28Do8f78FvDbP6wSwy78M8a/z28Br8by+G/gYYJcrjgNfBbw3/zG+G/gYYJcrjgNfBbw3z+tngLfmRYP413lr4Kd4wX6bK16b/xy/zRWvzQv2OsBv86JB/Ov9NvBa/M/0O8Br86JD/OsdB/4aeBD/szwDeGlglxcd4t/mOPDbwEvxP8PfAK8N7PKvg/j3+Wzgo4Fj/Pe4BHw18Nn82yD+Y7w18NLASwNvxX+unwH+Gvhr4Kf590H8xzkO/Bbw0vzn+mvgdYBd/v0Q/zGOA78FvDT/Nf4aeB1gl38fxL/fceC3gJfmv9ZfA68D7PJvh/j3+yvgpfnv8dfAy/Bvh/j3M/+9xL8d4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8Im7bBQbxKoE0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsSubway;
impl IconShape for MdDirectionsSubway {
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
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFlklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/eV4aOAY8GHgwz3YceGmev78Gdnm2W4FbgUvAX/MfD/Ef4zjwVsBrAy8NvDT/Of4a+Gvgt4GfAXb590H8+zwY+Czgvfnv8d3A5wC38m+D+Lc5DnwW8NH8z/DVwMfwr4f41zsO/Bbw0vzP8tfA6wC7vOgQ/3q/Bbw2/zP9NvA6vOgQ/zpvDfwU/7O9DfDTvGgQ/zq/Bbw2/7P9NvA6vGgQL7rjwEX+dxAvGsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2el/jX+23gtXhOvwO8Nv965nm9DvDb/MsQL7rXBn6L5yX+9X4beC2e0+8Ar82/nnlerwP8Nv8yxIvutYHf4nmJf73fBl6L5/Q7wGvzr2ee1+sAv82/DPGie23gt3he4l/vt4HX4jn9DvDa/OuZ5/U6wG/zL0O86F4b+C2e12/zr/fSwHGe0y7w1/zrvTbP63WA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3eMG+B/hrrnhp4L34j/U9wF9zxUsD78UL9jrAb/MvQ7zoXhv4LZ6/1wF+m+f02sBv8R/jdYDf5jm9NvBbPH+vA/w2/zLEi+61gd/ieX0O8Nk8f58NfBb/Pp8DfDbP32cDn8Xzeh3gt/mXIV50rw38Fs/rZYC/5vl7aeCv+Pd5GeCvef5eGvgrntfrAL/Nvwzxontt4Ld4XuKFM/8+4oUzz+t1gN/mX4Z40b028Fs8r9cBfpvn77WB3+Lf53WA3+b5e23gt3herwP8Nv8yxIvuOHCR5/XbwOvw/P0W8Nr8+/w28Do8f78FvDbP6wSwy78M8a/z28Br8by+G/gYYJcrjgNfBbw3/zG+G/gYYJcrjgNfBbw3z+tngLfmRYP413lr4Kd4wX6bK16b/xy/zRWvzQv2OsBv86JB/Ov9NvBa/M/0O8Br86JD/OsdB/4aeBD/szwDeGlglxcd4t/mOPDbwEvxP8PfAK8N7PKvg/j3+Wzgo4Fj/Pe4BHw18Nn82yD+Y7w18NLASwNvxX+unwH+Gvhr4Kf590H8xzkO/Bbw0vzn+mvgdYBd/v0Q/zGOA78FvDT/Nf4aeB1gl38fxL/fceC3gJfmv9ZfA68D7PJvh/j3+yvgpfnv8dfAy/Bvh/j3M/+9xL8d4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8Im7bBQbxKoE0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsTransit;
impl IconShape for MdDirectionsTransit {
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
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/Gd4L+Gjgpbnir4GvBr6H/1yI/37fBbw3z993A+/Dfx7Ef6/PBj6LF+5zgM/mPwfiv9dF4Dgv3C5wgv8ciP8+rw38Fi+a1wF+m/94iP8+rw38Fi+a1wF+m/94iP8+Lw38FS+alwH+mv94iP9etwIP4oV7BvBg/nMg/nu9NvBbvHCvA/w2/zkQ//3eG/hq4BjP6RLw3sBP858H8T/DceC9gZfmir8GvhvY5T8X4v83xH+9BwOfBTwYeG2u+Gnga4Df5r8W4r/OWwEfDbw2L9hXAx/Dfx3Ef67jwHsBHw08mBfN+wDfzX8NxH+OlwY+Cnhv/vV2gZcBbuU/H+I/1nsB7w28Nv8+Pw28Df/5EP9+DwbeC3hv4MH8x3kb4Kf5z4X49zP/OW4FXgbY5T8P4t/P/MsuAcf41/sa4KP5z4P49zMv2O8A3w18N2D+bd4G+Gn+cyD+/czz+h7gs4FbueK1gd/i32YXeB3gr/mPh/j3M8/rdYDf5tk+Gvgq/n3eB/hu/mMh/v3M83od4Ld5tu8G3ot/v58GPga4lf8YiH8/87xeB/htnu2vgJfmX/YM4LeB9+KF+27gp4HfAXb5t0P8+5nn9TrAb/Ns5kXzM8BbA18NfBQvmluBW4G/AT6afx3Ev595Xq8D/DZXvDbwW7xoPgf4bK54b+CrgWO8aH4HeG3+dRD/fuZ5vQ7w21zx0cBX8aJ5HeC3ebYHA98NvBb/st8BXpt/HcS/n3lerwP8Nld8N/BevGhOALs8r/cGPht4EC/Y7wCvzb8O4t/PPK/XAX6bK/4KeGn+Zc8AHswL99rAewNvDRzjOf0O8Nr86yD+/X6b5/XRwF9zhXnR/Azw1rzoXhp4aeDBXHEc+Gj+dRD/uV4b+C1eNJ8DfDb/tRD/uT4a+CpeNH8NvA6wy38dxH+u7wbeixfdXwOvA+zyXwPxn+uvgJfmX2cXeB3gr/nPh/jPZf5tdoHXAf6a/1yI/zyvDfwWz+sS8NHAd/HC7QKvA/w1/3kQ/3neG/guntfvAK8NvDfwXbxwu8AJ/vMg/vN8NfBRPK/PAT6bK94b+C5eOPGfB/Gf57eB1+J5vQ7w2zzbewNfDRzj+RP/eRD/eczzdwLY5Tm9NPDbwDGel/jPg/jP8dLAX/G8/gZ4aZ6/twZ+iucl/vMg/nO8N/BdPK/vAd6b5++1gd/ieYn/PIj/HF8NfBTP632A7+b5e23gt3he4j8P4j/HbwOvxfN6CHArz99rA7/F8xL/eRD/OczzugQc5wV7beC3eF7iPw/iP95LA3/F8/oZ4K15wV4b+C2el/jPg/iP997Ad/G8Pgf4bF6w1wZ+i+cl/vMg/uN9NfBRPK/XAX6bF+y1gd/ieYn/PIj/eL8NvBbPS7xwrw38Fs9L/OdB/Mczz+tvgJfmhXtt4Ld4XuI/D+I/1ksDf8Xz+hrgo3nhXhv4LZ6X+M+D+I/13sB38bzeB/huXrjXBn6L5yX+8yD+Y3018FE8r4cAt/LCvTbwWzwv8Z8H8R/rt4HX4jk9A3gw/7LXBn6L5yX+8yD+Y5nn9TPAW/Mve23gt3he4j8P4j/OSwN/xfP6GOCr+Ze9NvBbPC/xnwfxH+c48NI8r78GdvmXHQdemuf12/znQfz/hvj/jX8E/VjAQWCNVi8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsWalk;
impl IconShape for MdDirectionsWalk {
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
                d: "M13.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM9.8 8.9L7 23h2.1l1.8-8 2.1 2v6h2v-7.5l-2.1-2 .6-3C14.8 12 16.8 13 19 13v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1L6 8.3V13h2V9.6l1.8-.7",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PV4aeCvgpYFd4FbgZ4C/5r8W4r/WSwM/BTyY5+9W4H2A3+a/BuK/znsD38WL5n2A7+Y/H+K/xmsDv8W/zvsA381/LsR/vuPA04Hj/OvsAg8BdvnPg/jP997Ad/H8/Q5XvBbP38cAX81/HsR/vt8GXovndAl4beCvueKlgd8GjvGcfgd4bf7zIP7zmef1NcBH85y+Gvgonpf4z4P4z2ee1/sA381zejDwYJ7Xb/OfB/Gfbxc4xnP6aeBt+O+H+M/308Bb8bw+G/ge4Fb++yD+87018FO8YLcC3w18Dv/1EP81/hp4KV64twF+mv9aiP8aDwb+GjjGC/Y9wHvzXwvxX+elge8GXorn73eA1+a/FuK/1nHgo4GPBo7xnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/eg8Gfgt4MM/pVuB1gFv5r4P4r/PawHsB780L993A9wC/zX8+xH++twI+Gnht/nV+G/hu4Hv4z4P4z3EceCvgs4EH8+9zK/DVwPcAu/zHQvzHejDwXsBHA8f5j7ULfDfwNcCt/MdA/Md4aeCjgPfmRfcM4LO54rOBB/Gi+27ge4Df5t8H8e/zVsBHA6/Ni+53gM8Gfpvn9NrAZwOvxYvut4HvBr6HfxvEv95x4K2AzwYezIvue4DPBm7lhXsw8NnAe/GiuxX4auB7gF1edIh/nc8CPho4zovmEvDVwFcDu/zrHAc+Gvho4Bgvml3gq4HP4UWDeNEcB34LeGleNM8APhv4bv5jvDfw2cCDeNH8NfA6wC4vHOJF81fAS/Mv+x3gs4Hf5j/HawOfDbwW/7K/Bl6GFw7xL3tv4Lt44b4H+GzgVv5rPBj4bOC9eOHeB/huXjDEv+y3gdfieV0Cvhr4amAXOA68FP81/gbYBY4DHw18NHCM5/UzwFvzgiH+ZReB4zynnwHemuf02sBv8V/jdYDf5jn9NPBWPKdd4AQvGOJfZp7X+wDfzXN6beC3+K/xOsBv85zeG/gunpd4wRD/sluBB/GcdoG/5jkdB16a/xp/DezynF4aOM5zegbwYF4wxL/sq4GP4n+nrwE+mhcM8S97MPDXwDH+d7kEvDRwKy8Y4kXz3sB38b/L+wDfzQuHeNG9NfDVwIP4n+0ZwEcDP82/DPGvcxx4beClgdfmOR0HXor/Gn8D7PKcfhv4a+CnedEh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4a+Gr+a3w08Nf8+yH+f0P8/4b4/w3x/xvi/zf+ES3isEEuIU8EAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDryCleaning;
impl IconShape for MdDryCleaning {
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
                d: "M19.56,11.36L13,8.44V7c0-0.55-0.45-1-1-1l0,0c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1h2c0-1.84-1.66-3.3-3.56-2.95 C10.26,2.27,9.29,3.22,9.06,4.4C8.76,5.96,9.66,7.34,11,7.82v0.63l-6.56,2.92C3.56,11.75,3,12.62,3,13.57v0.01 C3,14.92,4.08,16,5.42,16H7v6h10v-6h1.58c1.34,0,2.42-1.08,2.42-2.42v-0.01C21,12.62,20.44,11.75,19.56,11.36z M18.58,14H17v-1H7v1 H5.42C5.19,14,5,13.81,5,13.57c0-0.17,0.1-0.32,0.25-0.38l6.75-3l6.75,3C18.9,13.26,19,13.41,19,13.58C19,13.81,18.81,14,18.58,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/npYFj/Pe6BPw1/zaIf73XBt4LeG/+Z/lu4HuA3+ZFh3jRPRj4KuCt+Z/tp4GPAW7lX4Z40bw08FvAcf532AVeB/hrXjjEv+ylgd8CjvO/yy7wOsBf84IhXrjjwF8BD+Z/p1uBlwF2ef4QL9x3A+/F/25fA3w0zx/iBXsw8HT+4/0M8NrAMf7rPAS4leeFeMG+G3gv/mN9D/DewEsDvw0c47/G9wDvzfNCvGAXgeP8x/ke4L15tp8C3pr/GrvACZ4X4vl7aeCv+I/zPcB782zfBbw3/7VeBvhrnhPi+fts4LP4j/E9wHvzbN8FvDf/9T4G+GqeE+L5+2zgs3jBvgf4auC3gWO8YN8DvDfP9l3Ae/Pf43OAz+Y5IZ6/7wbei+fvZ4C35oqXBn4bOMbz+h7gvXm27wLem/8+3wO8N88J8fx9N/BePH+7wOsAf80VLw38NnCMZ/se4L15tu8C3pv/Xt8DvDfPCfH8fTbwWbxgu8DrAH/NFS8N/DZwDPge4L15tu8C3pv/fp8DfDbPCfH8fTbwWbxwu8DrAH/NFS8NvDfw0TzbdwHvzf8MHwN8Nc8J8fy9NPBX/Mt2gdcB/prn9V3Ae/M/x8sAf81zQrxgu8Ax/mW7wOsAf82zfRfw3vzPcQk4zvNCvGDfDbwXL5pd4HWAvwa+C3hv/mf5HuC9eV6IF+zBwNN50e0Cvw28Nf/zPAS4leeFeOG+Gvgo/nf7GuCjef4QL9xx4K+BB/G/0zOAlwZ2ef4Q/7KXBn4bOMb/LpeA1wb+mhcM8aJ5aeC3gWP873AJeG3gr3nhEC+6BwNfDbwV/7P9DPDRwK38yxD/eq8NvDfwXvzPcQn4aeC7gd/mRYf493lp4Dj/vXaBv+bfBvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hG0FG1BFtHVngAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEditAttributes;
impl IconShape for MdEditAttributes {
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
                d: "M17.63 7H6.37C3.96 7 2 9.24 2 12s1.96 5 4.37 5h11.26c2.41 0 4.37-2.24 4.37-5s-1.96-5-4.37-5zM7.24 14.46l-2.57-2.57.7-.7 1.87 1.87 3.52-3.52.7.7-4.22 4.22z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Nf67fBt4G2OXfD/Ef57eA1+a/xm8Dr8O/H+I/xncB781/re8G3od/H8S/33sD38V/j/cBvpt/O8S/z3Hg6cBx/nvsAg8Bdvm3Qfz7fDbwWfz3+hzgs/m3QfzbPRj4K+A4/3G+B3hv4KOBr+JFswu8DHAr/3qIf7uPBr6K/zjfA7w3z/bewHfxovkY4Kv510P82/028Fr8x3kf4Lt5Th8NfBX/sp8B3pp/PcS/zXHgIv8xvgd4aeClgPcBvptnOw5c5EVzAtjlXwfxb/PawG/x7/c9wHsDx4HfBl4KeB/gu7niu4D35kXzOsBv86+D+Lf5bOCz+Pf5HuC9ebbjwG8DLwW8D/BawHvzovsY4Kv510H823w28Fn8230P8N48r+PAbwMvxb/e5wCfzb8O4t/mu4H34t/me4D35gX7LuC9+df7GuCj+ddB/Nt8N/Be/Ot9D/DevGDfBbw3/zbfA7w3/zqIf5vPBj6Lf53vAd6bF+y7gPfm3+5zgM/mXwfxb/PZwGfxovse4L15wb4LeG/+fT4H+Gz+dRD/Nq8N/BYvmu8B3psX7LuA9+bf73WA3+ZfB/Fvcxy4yItGvGDfBbw3/zFOALv86yD+7W4FHsS/TDx/3wW8N/8xngE8mH89xL/dVwMfxb9MPK/vAt6b/zhfA3w0/3qIf7uXBv6Kf5l4Tt8FvDf/sV4G+Gv+9RD/PrcCD+K/1zOAB/Nvg/j3+Wzgs/jv9TnAZ/Nvg/j3eTDwdP57PQS4lX8bxL/fdwPvxX+P7wHem387xL/fSwN/xX+PhwC38m+H+I/x28Br8V/rZ4C35t8H8R/jtYHf4r/WywB/zb8P4j/OdwPvxX+N7wHem38/xH+cBwNP57/GQ4Bb+fdD/Mf6bOCz+M/1OcBn8x8D8R/rOPDXwIP4z/EM4KWBXf5jIP7jvTXwU/zneBvgp/mPg/jP8dPAW/Ef62eAt+Y/FuI/x4OBvwaO8R/jEvBgYJf/WIj/PB8NfBX/MT4G+Gr+4yH+c/008Fb8+/wM8Nb850D85zoO3Aoc49/mEvBgYJf/HIj/fG8N/BT/Nm8D/DT/eRD/Nb4a+Cj+db4G+Gj+cyH+6/w18FK8aP4GeGn+8yH+6zwY+GvgGC/cJeClgVv5z4f4r/XWwE/xwr0N8NP810D81/ts4LN4/j4H+Gz+6yD+e/w08FY8p+8B3pv/Woj/HseB3wZeiiv+BnhtYJf/Woj/PseBW7niwcAu//UQ/71emiv+mv8eiP/fEP+/If5/Q/z/hvj/jX8E1BmaQUfDdJQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEditLocation;
impl IconShape for MdEditLocation {
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
                d: "M12 2C8.14 2 5 5.14 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.86-3.14-7-7-7zm-1.56 10H9v-1.44l3.35-3.34 1.43 1.43L10.44 12zm4.45-4.45l-.7.7-1.44-1.44.7-.7c.15-.15.39-.15.54 0l.9.9c.15.15.15.39 0 .54z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEditRoad;
impl IconShape for MdEditRoad {
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
            polygon {
                points: "18,4 16,4 16,11.9 18,9.9",
            }
            rect {
                height: "16",
                width: "2",
                x: "4",
                y: "4",
            }
            rect {
                height: "4",
                width: "2",
                x: "10",
                y: "4",
            }
            rect {
                height: "4",
                width: "2",
                x: "10",
                y: "10",
            }
            rect {
                height: "4",
                width: "2",
                x: "10",
                y: "16",
            }
            path {
                d: "M22.56,12.59l-1.15-1.15c-0.59-0.59-1.54-0.59-2.12,0L14,16.73V20h3.27l5.29-5.29C23.15,14.12,23.15,13.17,22.56,12.59z M16.58,18.45h-1.03v-1.03L19,13.97L20.03,15L16.58,18.45z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricBike;
impl IconShape for MdElectricBike {
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
                d: "M19,7h-0.82l-1.7-4.68C16.19,1.53,15.44,1,14.6,1H12v2h2.6l1.46,4h-4.81l-0.36-1H12V4H7v2h1.75l1.82,5H9.9 C9.46,8.77,7.59,7.12,5.25,7.01C2.45,6.87,0,9.2,0,12c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99 c2.8,0.13,5.25-2.19,5.25-5C24,9.2,21.8,7,19,7z M7.82,13c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3 c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,11h-1.4l-0.73-2H15C14.56,9.58,14.24,10.25,14.1,11z M19,15c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67C18.94,9.01,18.97,9,19,9c1.68,0,3,1.32,3,3S20.68,15,19,15z",
            }
            polygon {
                points: "11,20 7,20 13,23 13,21 17,21 11,18",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77X47/U7/Nsh/u1eGvgp4MH897oVeBvgr/nXQ/zbHAeeDhznf4Zd4CHALv86iH+bzwY+i/9ZPgf4bP51EP82TwcezLP9DbDLf63jwEvxbLcCD+FfB/Gv99LAX/GcTgC7/Nc6DlzkOb0M8Ne86BD/el8NfBTP9jPAW/Pf46eBt+LZvgb4aF50iH+9pwMP5tneB/hu/nu8N/BdPNutwEN40SH+dV4a+Cue0wlgl/8ex4GLPKeXAf6aFw3iX+e7gffi2X4GeGv+e/008FY82/cA782LBvGvcxE4zrO9D/Dd/Pd6b+C7eLZd4AQvGsSL7q2Bn+I5nQB2+e91HLjIc3ob4Kf5lyFedN8NvBfP9jPAW/M/w08Db8WzfQ/w3vzLEC+6i8Bxnu19gO/mf4b3Br6LZ9sFTvAvQ7xo3hr4KZ7TCWCX/xmOAxd5Tm8D/DQvHOJF89PAW/FsPwO8Nf+z/DTwVjzb9wDvzQuH+JcdBy7ynL4a+Gn+Z3lr4KN5TieAXV4wxL/svYHv4n+n9wG+mxcM8S/7aeCt+N/pZ4C35gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvMC/Y5wE8Dfw28NvDZwGvxn+t3gM8Gfht4aeCtgc/iBRMvGOJfZp6/1wF+m+f13cB78Z/je4D35nm9NvBbPH/iBUP8y8zz+h7gvXn+jgMX+c9xAtjl+ftu4L14XuIFQ/zLzPP6GOCrecF+G3gt/mP9DvDavGAfDXwVz0u8YIh/mXleHwN8NS/YbwOvxX+s3wFemxfso4Gv4nmJFwzxLzPP63uA9+b5Ow5c5D/HCWCX5++7gffieYkXDPEvM8/f6wC/zfP6LuC9+c/x3cD78LxeG/gtnj/xgiH+ZeYF+2zgZ4C/Bl4L+GzgtfnP9dvAZwO/A7w08FbAZ/OCiRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvMy+c+O9lXjjxgiH+ZeaFE/+9zAsnXjDEv8y8cOK/l3nhxAuG+Jf9NfBSPH9/A7w0/73+Gngpnr+/AV6aFwzxL3tp4LeBYzynS8BrA3/Nf6+XBn4bOMZzugS8NvDXvGCIF81x4KV5Tn8N7PI/w3HgpXlOfw3s8sIh/n9D/P+G+O/x3cBvAd/Dfy/Ef4/fBl4LuBX4bOB7+O+B+O/x28Br8Wy3Ap8NfA//tRD/PX4beC2e163AZwPfw38NxH+P3wZeixfsVuCzge/hPxfiP85x4KOB1+Jf9tLAcf5ltwKfDXwP/zkQ//FeG/hs4LX4j3Mr8NnA9/AfC/Gf57WBzwZei/84twKfDXwP/zEQ//leG/hs4LV44X4HeG3+ayH+67w28NnAa/H8/Q7w2vzXQvzXe23gs4HX4jn9DvDa/NdC/Pd5beCzgdfiit8BXpv/Woj/fq8NfDZXvDb/tRD/czwYuJX/Woj/3xD/v/GPJ1+tQWcCVPgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricCar;
impl IconShape for MdElectricCar {
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
                d: "M18.92,2.01C18.72,1.42,18.16,1,17.5,1h-11C5.84,1,5.29,1.42,5.08,2.01L3,8v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h12 v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1V8L18.92,2.01z M6.5,12C5.67,12,5,11.33,5,10.5S5.67,9,6.5,9S8,9.67,8,10.5 S7.33,12,6.5,12z M17.5,12c-0.83,0-1.5-0.67-1.5-1.5S16.67,9,17.5,9S19,9.67,19,10.5S18.33,12,17.5,12z M5,7l1.5-4.5h11L19,7H5z",
            }
            polygon {
                points: "7,20 11,20 11,18 17,21 13,21 13,23",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricMoped;
impl IconShape for MdElectricMoped {
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
                d: "M19,5c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,12H10V7H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,8.35V5z M7,15c-0.55,0-1-0.45-1-1h2C8,14.55,7.55,15,7,15z",
            }
            rect {
                height: "2",
                width: "5",
                x: "5",
                y: "4",
            }
            path {
                d: "M19,11c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,11,19,11z M19,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,15,19,15z",
            }
            polygon {
                points: "7,20 11,20 11,18 17,21 13,21 13,23",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricRickshaw;
impl IconShape for MdElectricRickshaw {
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
                d: "M21,11.18V9.72c0-0.47-0.16-0.92-0.46-1.28L16.6,3.72C16.22,3.26,15.66,3,15.06,3H3C1.9,3,1,3.9,1,5v8c0,1.1,0.9,2,2,2 h0.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.41,1.16,1.51,2,2.82,2c1.66,0,3-1.34,3-3C23,12.7,22.16,11.6,21,11.18z M18.4,9H16V6.12L18.4,9z M3,5h4v4H3V5z M6,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M9,13v-2h3V9H9V5h5v8H9z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S20.55,15,20,15z",
            }
            polygon {
                points: "7,20 11,20 11,18 17,21 13,21 13,23",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricScooter;
impl IconShape for MdElectricScooter {
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
                d: "M7.82,16H15v-1c0-2.21,1.79-4,4-4h0.74l-1.9-8.44C17.63,1.65,16.82,1,15.89,1H12v2h3.89l1.4,6.25c0,0-0.01,0-0.01,0 c-2.16,0.65-3.81,2.48-4.19,4.75H7.82c-0.48-1.34-1.86-2.24-3.42-1.94c-1.18,0.23-2.13,1.2-2.35,2.38C1.7,16.34,3.16,18,5,18 C6.3,18,7.4,17.16,7.82,16z M5,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S5.55,16,5,16z",
            }
            path {
                d: "M19,12c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,12,19,12z M19,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,16,19,16z",
            }
            polygon {
                points: "11,20 7,20 13,23 13,21 17,21 11,18",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEhElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++lgWO86J4B3Mr/TIh/nc8GPot/vVuB7wa+Btjlfw7Ei+6lgb/i32cXeBvgt/mfAfGie23gt/iP8T7Ad/PfD/GiOw7cChzj328XeB3gr/nvhfjXeWngu4GX4t/vt4HX4b8X4j/XSwNvDXwWz9/bAD/Nfx/Ef43XBn6L5/U9wHvz3wfxX+e7gffiOf018DL890H81/lo4Kt4XuK/D+K/zkcDX8XzEv99EP91vht4L57T3wAvzX8fxH+N1wZ+i+f1PcB7898H8Z/rpYG3Aj6b5+9tgJ/mvw/i3+7BwFsBbw08GHgw/zq/A7w2/70Q/3rHga8C3pt/u0vAawN/zX8vxL/OSwO/BRzn3+d9gO/mvx/iRffSwF/x7/MM4K2Bv+Z/BsSL5jjwdOA4/zbPAL4b+Gpgl/85EC+a7wbei+fvb4CfBv4a2OV53Qrcyv9MiH/Zg4Gn8/x9DPDV/O+F+Jd9NPBVPK+PAb6a/90Q/7LfBl6L5/Q3wEvzH+vBwGcBbw0c54V7HeC3+fdD/MueDjyY5/Q5wGfzH+e9ge/iRfc6wG/z74f4l5nn9TbAT/Mf48HA0/nXeR3gt7nitXj+/gbY5YVD/MvM8/oc4LP5j/HdwHvxr/M6wG9zhXn+doHXAf6aFwzxL/tt4LV4Tj8DvDX/MS4Cx/nXeR3gt7nCvGB/DbwMLxjiX/bVwEfxvF4G+GtedC8NfBXP67X513sd4Le5wrxw4gVD/MveGvgpntetwNsAf82L5rWB3+I/xusAv80V5gX7HuC9ecEQL5rfBl6L57ULfDXwPcCtvHCvDfwW/zFeB/htrjDP3/cAHw3s8oIhXjSvDfwW/zbiitcGfov/GK8D/Db/fogX3UcDX8W/nrjitYHf4j/G6wC/zb8f4l/ns4HP4l9HXPHawG/xH+N1gN/m3w/xr/fWwHcDx3jRiCteG/gt/mO8DvDb/Psh/m2OA+8NvDfwUrxw4orXBn6L/xivA/w2/36If7/jwEvzgv02V7w28Fv8x3gd4Le54rd4XrcCPwP8NC8c4r/OawO/xX+M1wF+myvMC/Y5wGfzgiH+67w28Fv8x3gd4Le5wrxgtwIP4QVD/Pcz/3qvA/w2V5gX7BnAg3nBEP/9/hp4Kf51Xgf4ba4wL9jnAJ/NC4b47/fewHfxr/M6wG9zxW/zvG4Ffhr4aV44xP8M3w28Fy+61wF+m38/xP8cnw18NHCMf9nrAL/Nvx/if57X5l/218Au/36I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+I4JZBPrTI2QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElectricalServices;
impl IconShape for MdElectricalServices {
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
                d: "M21,14c0-0.55-0.45-1-1-1h-2v2h2C20.55,15,21,14.55,21,14z",
            }
            path {
                d: "M20,17h-2v2h2c0.55,0,1-0.45,1-1C21,17.45,20.55,17,20,17z",
            }
            path {
                d: "M12,14h-2v4h2c0,1.1,0.9,2,2,2h3v-8h-3C12.9,12,12,12.9,12,14z",
            }
            path {
                d: "M5,13c0-1.1,0.9-2,2-2h1.5c1.93,0,3.5-1.57,3.5-3.5S10.43,4,8.5,4H5C4.45,4,4,4.45,4,5c0,0.55,0.45,1,1,1h3.5 C9.33,6,10,6.67,10,7.5S9.33,9,8.5,9H7c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4h2v-2H7C5.9,15,5,14.1,5,13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEvStation;
impl IconShape for MdEvStation {
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
                d: "M19.77 7.23l.01-.01-3.72-3.72L15 4.56l2.11 2.11c-.94.36-1.61 1.26-1.61 2.33 0 1.38 1.12 2.5 2.5 2.5.36 0 .69-.08 1-.21v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v16h10v-7.5h1.5v5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5V9c0-.69-.28-1.32-.73-1.77zM18 10c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zM8 18v-4.5H6L10 6v5h2l-4 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/j+PAS/G8fod/O8T/Hq8N/BbPS/zbIf73eG3gt3he4t8O8b/HawO/xfMS/3aI/z1eG/gtnpf4t0P87/HawG/xvMS/HeJ/j9cGfovnJf7tEP97vDbwWzwv8W+H+N/jtYHf4nmJfzvE/x6vDfwWz0v82yH+93ht4Ld4XuLfDvHf47WB1+Jf58HAe/O8Ppt/ve8BbgUQ/z3eG/gu/vu8DvDbAOK/x2sDv8V/n4cAtwKI/x4PBp7Ofx9xBeK/j/nvI65A/Pe5FXgQ//V+B3htrkD89/lu4L34r/c7wGtzBeK/14OBtwbeG3gp/mt8D/DeXIH4n+O1gfcG3ov/XJ8DfDZXIP7neTDw3cBr8Z/jc4DP5grE/1xvDXw18CD+Y70O8Ntcgfj3OQ68FPDawEsDx4EHAw/mOf02V/w18NfA7wC38i87Dvw08Fr8x3kd4Le5AvFv817AWwNvzb/drcB3A98D3MoL993Ae/Ef4yHArVyBeNEdBz4K+GjgOP+xfhv4GOCvecG+G3gv/v3EsyFeNO8FfDbwYP5zfTfwMcAuz993A+/Fv494NsS/7LuA9+a/zi7wOsBf87yOA78NvBT/Nr8DvDbPhnjBjgO/Bbw0//V2gbcBfpvn9WDgr4Fj/Ov9DvDaPBviBfsr4KX57/U6wG/zvN4b+C7+9b4HeG+eDfH8fTXwUfz32wUeAuzyvP4aeCn+dT4H+GyeDfG8Xhv4Lf7n+GngbXhe7w18F/86nwN8Ns+GeF5/Bbw0/7O8DvDbPK9bgQfxonsd4Ld5NsRzemngr/if53uA9+Z5fTfwXrzoXgf4bZ4N8Zy+G3gv/ufZBR4C7PKc3hr4KV50DwFu5dkQz+kicJz/mV4H+G2e04OBp/OiE88J8ZzM/1yfA3w2z8u86MRzQjzbSwN/xf9cnwN8Ns/LvGh+B3htnhPi2V4b+C3+5/od4LV5Xn8NvBT/st8BXpvnhPjf77eB1+Jf9j3Ae/OcEP/7/TbwWvzLPgf4bJ4T4n+/3wZei3/Z5wCfzXNC/O/328Br8S97HeC3eU6I//1+G3gt/mWvA/w2zwnxbC8NfBX/c/0N8NE8r98GXot/2UOAW3lOiGd7beC3+J/rd4DX5nn9NvBa/MvE80I822sDv8X/XL8DvDbP67eB1+JfJp4X4tleG/gt/uf6HeC1eV6/DbwWL9zvAK/N80I822sDv8X/XL8DvDbP67eB1+KF+x3gtXleiGd7aeCr+Z/rr4GP5nn9NvBavHDfA7w3zwvxv99vA6/FC/c5wGfzvBD/+/028Fq8cJ8DfDbPC/G/328Dr8UL9zrAb/O8EP/7/TbwWrxwrwP8Ns8L8b/fbwOvxQv3EOBWnhfi2V4b+C3+5/od4LV5Xr8NvBYvnHj+EM/22sBv8T/X7wCvzfP6beC1eMGeATyY5w/xbK8N/Bb/c/0O8No8r98GXosX7HuA9+b5QzzbSwN/xf9cXwN8NM/rt4HX4vm7BDwY2OX5QzynzwY+i/95/gZ4bWCX5/XbwGvxvH4H+Gjgr3nBEM/rwcCD+Z/lt3nBXho4znO6FbiVfxni/zfE/2/8I5ESqkH3INICAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFastfood;
impl IconShape for MdFastfood {
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
                d: "M18.06 22.99h1.66c.84 0 1.53-.64 1.63-1.46L23 5.05h-5V1h-1.97v4.05h-4.97l.3 2.34c1.71.47 3.31 1.32 4.27 2.26 1.44 1.42 2.43 2.89 2.43 5.29v8.05zM1 21.99V21h15.03v.99c0 .55-.45 1-1.01 1H2.01c-.56 0-1.01-.45-1.01-1zm15.03-7c0-8-15.03-8-15.03 0h15.03zM1.02 17h15v2h-15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx/jXeQZwK/+xEP89fht4LV50l4CXBm7lPxbiv8dvA6/Fi+5jgK/mPx7iv8dvA6/Fi+Z3gNfmPwfiv8dvA6/Fi+YhwK3850D89/ht4LV40XwO8Nn850D89/ht4LV40b0O8Nv8x0P89/ht4LV40d0KvAywy38sxH+P3wZei+d0CfhrXrDvBr6b/1iI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/eq8N/BRwnOe0C7wN8Nv810H81/os4LN54T4b+Bz+ayD+a7w08F3AS/Oi+WvgfYC/5j8X4j/fRwGfDRznX2cX+Gzga/jPg/jPcxz4KeC1+ff5beBtgF3+4yH+c7w18F3Acf5j7ALvA/w0/7EQ/7GOA18FvDf/Ob4b+Bhgl/8YiP84rw18F/Bg/nPdCrwP8Nv8+yH+Y3wW8Nn81/ps4HP490H8+7w08F3AS/Pf46+B9wH+mn8bxL/dRwGfDRznv9cu8NnA1/Cvh/jXOw78FPDa/M/y28DbALu86BD/Om8NfBdwnP+ZdoH3AX6aFw3iRXMc+Crgvfnf4buBjwF2eeEQ/7LXBr4LeDD/u9wKvA/w27xgiBfus4DP5n+3zwY+h+cP8fy9NPBdwEvzf8NfA+8D/DXPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/Oe6BHw28NfAdwMP4j/X7wCvzXNCPH+/DbwW/3l+BnhvYJcrjgPfDbwV/3l+B3htnhPi+ftt4LX4j3cJeG/gp3n+3hr4buAY//F+B3htnhPi+ftt4LX4j/UzwHsDu7xwx4HvBt6K/1i/A7w2zwnx/P028Fr8x7gEvDfw0/zrvDXw3cAx/mP8DvDaPCfE8/fbwGvxH+MEsMu/zXHgIv8xfgd4bZ4T4vn7beC1+I8h/n3Mf4zfAV6b54R4/n4beC3+Y4h/H/Mf43eA1+Y5IZ6/3wZei/8Y4t/H/Mf4HeC1eU6I5++3gdfiP4b49zH/MX4HeG2eE+L5+23gtfiPIf59zH+M3wFem+eEeP5+G3gt/mOIfx/zH+N3gNfmOSGev98GXov/GOLfx/zH+B3gtXlOiOfvt4HX4j+G+Pcx/zF+B3htnhPi+ftt4LX4jyH+fcx/jN8BXpvnhHj+fht4Lf5jiH8f8x/jd4DX5jkhnr/fBl6L/xji38f8x/gd4LV5Tojn77eB1+I/hvj3Mf8xfgd4bZ4T4vn7auCl+Y/x2vz7/Db/Mf4a+GieE+L/N8T/b4j/3xD/vyH+f+MfAVUSokHu4VSbAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFestival;
impl IconShape for MdFestival {
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
            polygon {
                points: "13,5.7 13,4 16,4 15,2.51 16,1 11,1 11,5.7 2,12 2,22 9,22 9,17 12.03,15 15,17 15,22 22,22 22,12",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9Y4D7wW8NHAr8D3Arfz3QPzXemngp4AH85zeB/hu/ush/mv9FfDSPH8PAW7lvxbiv85LA3/FC/Y5wGfzXwvxX+e1gd/iBfsZ4K35r4X4r/PawG/xgv0O8Nr810L813lt4Ld4wX4HeG3+ayH+67w28Fu8YL8DvDb/tRD/dV4b+C1esN8BXpv/Woj/Oq8N/BYv2O8Ar81/LcR/ndcGfosX7HeA1+a/FuK/zmsDv8UL9jvAa/NfC/Ff57WB3+IF+x3gtfmvhfiv89rAb/GC/Q7w2vzXQvzXeW3gt3jBfgd4bf5rIf7rvDbwW7xgvwO8Nv+1EP91Xhv4LV6w3wFem/9aiP86rw38Fi/Y7wCvzX8txH+d1wZ+ixfsd4DX5r8W4r/GawPfBTyYF2wX+Grgc/ivg/jPdRz4LuCtedH9NfAxwG/znw/xn+ejgM8GjvNv89XA5wC7/OdB/Md7aeCrgNfm328XeB/gp/nPgfiPcxz4KOCz+Y/328D7ALfyHwvxH+O1ge8CHsx/nl3gq4HP4T8O4t/nOPBdwFvzX+evgY8Bfpt/P8S/3UcBnw0c57/HVwOfA+zyb4f413tp4KuA1+a/3y7wPsBP82+DeNEdBz4K+Gz+5/lt4H2AW/nXQbxoXhv4LuDB/M+1C3w18Dm86BAv3HHgu4C35n+PvwY+Bvht/mWIF+yjgM8GjvNvdwn4beCngV3gp3jBfgf4auC1gbcGHsS/z1cDnwPs8oIhnr/vBt6Lf5tLwE8DPw38NM/22sBv8YL9DvDaPNtLA+8NvDbwUvzb7AIneMEQz99vA6/Fi+4ZwE8D3w38Nc/fawO/xQv2O8Br8/w9GHhr4L2Bl+JfR7xgiOfvt4HX4oX7G+C7gd8G/pp/2WsDv8UL9jvAa/MvOw68NfDWwFvxLxMvGOL5+23gtXhePwP8NvDTwK3867w28Fu8YL8DvDb/OseB1wbeGnhr4BjPS7xgiOfvt4HXAi4Bvw38NPDTwC7/dq8N/BYv2O8Ar82/z1sDrw28NfAgrhAvGOL5+2jgVuCn+Y/z2sBv8YL9DvDa/Md5aeC9gY/mBUP813lt4Ld4wX4HeG3+ayH+67w28Fu8YL8DvDb/tRD/dV4b+C1esN8BXpv/Woj/Oq8N/BYv2O8Ar81/LcR/ndcGfosX7HeA1+a/FuK/zmsDv8UL9jvAa/NfC/Ff57WB3+IF+x3gtfmvhfiv89bAT/GC/TXwMvzXQvzX+Cjgq/mX/TTwPsAu/zUQ/7mOA98FvDUvuluBtwH+mv98iP88Lw38FPBg/m0+Gvga/nMh/nO8N/Bd/Pv9NPA2/OdB/Of4beC1+I8h/vMg/nP8NvBaPK9nAL8N/DTw28BLA28NvDbwUjx/4j8P4j/HbwOvxRW/A/w08NvAX/OCPRh4beCtgdcGjnGF+M+D+M/x0cCtwG8Du/zbvDbw1sBH858H8f8b4v83xP9viP/fEP+/8Y9BJqdBwPfCkgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFlight;
impl IconShape for MdFlight {
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
                d: "M21 16v-2l-8-5V3.5c0-.83-.67-1.5-1.5-1.5S10 2.67 10 3.5V9l-8 5v2l8-2.5V19l-2 1.5V22l3.5-1 3.5 1v-1.5L13 19v-5.5l8 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHail;
impl IconShape for MdHail {
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
                d: "M12 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm5-4h2v.4c-.1 2.2-.8 3.9-2.3 5.1-.5.4-1.1.7-1.7.9V22h-2v-6h-2v6H9V10.1c-.3.1-.5.2-.6.3-.9.7-1.39 1.6-1.4 3.1v.5H5v-.5c0-2 .71-3.59 2.11-4.79C8.21 7.81 10 7 12 7s2.68-.46 3.48-1.06C16.48 5.14 17 4 17 2.5V2zM4 16h3v6H4v-6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHandyman;
impl IconShape for MdHandyman {
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
                d: "M21.67,18.17l-5.3-5.3h-0.99l-2.54,2.54v0.99l5.3,5.3c0.39,0.39,1.02,0.39,1.41,0l2.12-2.12 C22.06,19.2,22.06,18.56,21.67,18.17z",
            }
            path {
                d: "M17.34,10.19l1.41-1.41l2.12,2.12c1.17-1.17,1.17-3.07,0-4.24l-3.54-3.54l-1.41,1.41V1.71L15.22,1l-3.54,3.54l0.71,0.71 h2.83l-1.41,1.41l1.06,1.06l-2.89,2.89L7.85,6.48V5.06L4.83,2.04L2,4.87l3.03,3.03h1.41l4.13,4.13l-0.85,0.85H7.6l-5.3,5.3 c-0.39,0.39-0.39,1.02,0,1.41l2.12,2.12c0.39,0.39,1.02,0.39,1.41,0l5.3-5.3v-2.12l5.15-5.15L17.34,10.19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADdUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/XSwGsBbw0cB16afz/xgr038F08r9cBfpt/GeI/xmsDnwW8Nv/xxPP33sB38fy9DvDb/MsQ/z7Hge8C3pr/POJ5vTfwXbxgrwP8Nv8yxL/dg4GfAl6a/1ziOb038F28cK8D/Db/MsS/zXHgt4CX5j+feLb3Br6Lf9nrAL/Nvwzxb/NbwGvzX0Nc8d7Ad/GieR3gt/mXIf713hv4Lv7rCHhv4Lt40b0O8Nv8yxD/OseBpwPH+a/zPsB38a/zOsBv8y9D/Ou8N/Bd/M/3OsBv8y9D/Ov8NPBW/M/3OsBv8y9D/OuY/x1eB/ht/mWIF91LA3/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRXcceGn+9X6Lf5+/AV6Kf53XAX6bfxniP5/59zkB/DbwUrzoXgf4bf5liP985t9HwHHgt4GX4kXzOsBv8y9D/Ocz/z7iiuPAbwMvxb/sdYDf5l+G+M9n/n3Esx0Hfht4KV641wF+m38Z4j+f+fcRz+k48NvAS/GCvQ7w2/zLEP/5zL+PeF7Hgd8GXorn73WA3+ZfhvjPZ/59xPN3HPht4KV4Xq8D/Db/MsR/vtfm3+e3ecGOAy/N8/prYJd/GeL/N8T/b4j/3xD/vyH+870W/z6/w38exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP983w28F/823wO8N/95EP/5jgNfDbwX/zrfA3w0sMt/HsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EYGpVkGOo2DbAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHardware;
impl IconShape for MdHardware {
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
                d: "M18,3l-3,3V3H9C6.24,3,4,5.24,4,8h5v3h6V8l3,3h2V3H18z",
            }
            path {
                d: "M9,13v7c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-7H9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP8+DgQfxH+MZwK38x0P8xzsOfBfw1vzH+mngfYBd/uMg/mMdB54OHOc/xy7wEGCX/xiI/1g/Bbw1/7l+Gngb/mMg/uM8GHg6/zUeAtzKvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L5yX+fczzeh3gt/n3Q/zHeW3gt3he4t/HPK/XAX6bfz/Ef5zXBn6L5yX+fczzeh3gt/n3Q/zHeW3gt3he4t/HPK/XAX6bfz/Ef5zXBn6L5yX+fczzeh3gt/n3Q/zLXhp4K+ClgeO8YMeBl+Z5/Tb/Pq/N8/prYJcXbBf4a+BngL/mBUO8cF8FfDT/u3018DE8f4gX7KuBj+L/hq8BPprnhXj+Xhr4K/5veQhwK88J8fx9NvBZ/N/yOcBn85wQz99PA2/F/y0/A7w1zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/Fi0Y8r98GXov/WL8DvDbPy7xofgd4bZ4T4vn7beC1eNGI5/XbwGvxH+t3gNfmeZkXze8Ar81zQjx/vw28Fi8a8bx+G3gt/mP9DvDaPC/zovkd4LV5Tojn77eB1+JFI57XbwOvxX+s3wFem+dlXjS/A7w2zwnx/P028Fq8aMTz+m3gtfiP9TvAa/O8zIvmd4DX5jkhnr+XBo7zovltntdLA8f5j7UL/DXP67V50ewCf81zQvz/hvj/DfH8/TbwWrxoxPP6beC1+I/1O8Br87zMi+Z3gNfmOSGev98GXosXjXhevw28Fv+xfgd4bZ6XedH8DvDaPCfE8/fbwGvxonltntdXAy/Nf6y/Bj6a5/XbvGh+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vl7aeA4/7fsAn/Nc0L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwQjhntBYHe17AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHomeRepairService;
impl IconShape for MdHomeRepairService {
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
            polygon {
                points: "18,16 16,16 16,15 8,15 8,16 6,16 6,15 2,15 2,20 22,20 22,15 18,15",
            }
            path {
                d: "M20,8h-3V6c0-1.1-0.9-2-2-2H9C7.9,4,7,4.9,7,6v2H4c-1.1,0-2,0.9-2,2v4h4v-2h2v2h8v-2h2v2h4v-4C22,8.9,21.1,8,20,8z M15,8 H9V6h6V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz/bSwFfxvF6H/7sQz/bawG/xvMT/XYhne23gt3he4v8uxLO9NvBbPC/xfxfi2V4b+C2el/i/C/Fsrw38Fs9L/Mtei2d7BnArL9xLA8f4z3EJ+GteNIhne23gt3he4vl7MPBZwHvzvP4a+Bzgp3n+fht4Lf7z3Ar8NvA5wK28YIhne23gt3he4nm9N/Bd/Mu+G/gYYJfn9NvAa/Ff46uBj+H5QzzbawO/xfMSz+m9ge/iRffTwNvwnH4beC3+6/w18DrALs8J8WyvDfwWz0s824OBp/Ov9znAZ/Nsvw28Fv+1/hp4GZ4T4tleG/gtnpd4tu8G3ot/vV3gIcAuV/w28Fr81/sa4KN5NsSzvTbwWzwvccVx4CL/dm8D/DRX/DbwWvz3eAhwK1cgnu21gd/ieYkrXhv4Lf7tPgf4bK74beC1+O/xPcB7cwXi2V4b+C2el7jitYHf4t/uZ4C35orfBl6L/x63Ag/hCsSzvTbwWzwvccVrA7/Fv93nAJ/NFb8NvBb/fV4G+GsA8WyvDfwWz0tc8WDg6fzbfQzw1Vzx28Br8d/ndYDfBhDP9trAb/G8xLP9NfBS/Ns8BLiVK34beC3++7wO8NsA4tleG/gtnpd4trcGfop/ve8B3ptn+23gtfjv8zrAbwOIZ3tt4Ld4XuI5fTfwXrzoLgEPBnZ5tt8GXov/Pq8D/DaAeLbXBn6L5yWe03Hgp4HX4l92CXht4K95Tr8NvBb/fV4H+G0A8WyvDfwWz0s8f58NfDRwjOfve4CPBnZ5Xr8NvBb/fV4H+G0A8WyvDfwWz0u8YMeB1wZeGnhp4K+BXeCngVt5wX4beC3++7wO8NsA4tleG/gtnpf4j/fbwGvx3+d1gN8GEM/22sBv8bzEf7zfBl6L/z6vA/w2gHi21wZ+i+f12/zHe2ngOP99Xgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi21wZ+i+f1Ovzv9ls8r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tmOAy/N8/pt/nd7bZ7XXwO7AOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I6ySl0GAxBKQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHotel;
impl IconShape for MdHotel {
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm12-6h-8v7H3V5H1v15h2v-3h18v3h2v-9c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Mf5GeCngVuBXeCvueKlgePAg4G3Bt6K/zjiBUP8y8y/zyXgo4GfBnZ50RwH3hr4auAY/z7iBUP8y8y/3ecAXw3s8m9zHPho4LP4txMvGOJfZv71LgGvDfw1/zFeGvht4Bj/euIFQ/zLzL/O3wCvDezyH+vBwE8DL8W/jnjBEP8y86K7BDwY2OUFOw68FvDSwIO54lbgr4HfAXZ5wR4M/DVwjBedeMEQ/zLzonsZ4K95/h4MfBbw3rxw3w18DnArz99LA3/Fi068YIh/mXnRfA7w2Tx/7w18FXCcF80u8D7AT/P8fTbwWbxoxAuG+JeZf9kl4MHALv81jgO3Asf4l4kXDPEvM/+y9wG+m/9a7w18F/8y8YIh/mXmX3YC2OU5HQdeiv8YfwPs8pyOAxf5l4kXDPEvMy/czwBvzfP6auCj+I/xNcBH87x+GngrXjjxgiH+ZeaFex/gu3leF4Hj/MfYBU7wvD4a+CpeOPGCIf5l5oV7HeC3eU6vDfwW/7FeB/htntNrA7/FCydeMMS/zLxwLwP8Nc/pvYH35j/WdwPfzXN6aeCveOHEC4b4l5kXTvz3Mi+ceMEQ/zLzwon/XuaFEy8Y4l9mXriXAf6a5/TewHvxH+t7gO/mOb008Fe8cOIFQ/zLzAv3OsBv85xeG/gt/mO9DvDbPKfXBn6LF068YIh/mXnh3gf4bp7XLnCM/xiXgAcDuzynjwa+ihdOvGCIf5l54X4GeGue11cDH8V/jO8B3pvn9dPAW/HCiRcM8S8z/7ITwC7P6Tjw0vzH+Gtgl+d0HLjIv0y8YIh/mfmXvQ/w3fzXem/gu/iXiRcM8S8z/7Jd4CHALv81jgNPB47zLxMvGOJfZl40nwN8Ns/fewNfDRzjRXMJeG/gp3n+Phv4LF404gVD/MvMi+5lgL/m+Xsw8NnAe/HCfQ/w2cCtPH8vDfwVLzrxgiH+ZeZFtws8BNjlBTsOvDbw0sCDueJW4K+B3wZ2ecEeDPwVcJwXnXjBEP8y86/z18DrALv8x3ow8FPAS/OvI14wxL/M/OvtAq8D/DX/MV4a+C3gOP964gVD/MvMv91nA18D7PJvcxz4KOCz+bcTLxjiX2b+fXaBjwZ+BtjlRXMceCvgq4Hj/PuIFwzxLzP/cX4a+GngVuAS8Ndc8dLAMeClgdcG3pr/OOIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLdoFj/O90CTjOC4b4l3038F787/Q9wHvzgiH+ZQ8Gns7/Tg8BbuUFQ7xo3hv4Lv53eR/gu3nhEC+6BwOfDbw1cIz/mS4BPw18NnAr/zLE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hErmLlBdgsKLAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHvac;
impl IconShape for MdHvac {
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
                d: "M12,16c1.01,0,1.91-0.39,2.62-1H9.38C10.09,15.61,10.99,16,12,16z",
            }
            path {
                d: "M8.56,14h6.89c0.26-0.45,0.44-0.96,0.51-1.5h-7.9C8.12,13.04,8.29,13.55,8.56,14z",
            }
            path {
                d: "M12,8c-1.01,0-1.91,0.39-2.62,1h5.24C13.91,8.39,13.01,8,12,8z",
            }
            path {
                d: "M8.56,10c-0.26,0.45-0.44,0.96-0.51,1.5h7.9c-0.07-0.54-0.24-1.05-0.51-1.5H8.56z",
            }
            path {
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12,18c-3.31,0-6-2.69-6-6 s2.69-6,6-6s6,2.69,6,6S15.31,18,12,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9V4aeCuueDBX3MoVPwP8Nf91EP81Hgx8FvDawIN54W4Ffhv4HOBW/nMh/nMdB74KeG/+bb4a+Bxgl/8ciP88Lw18F/DS/Pv8NfA+wF/zHw/xn+Olgd8CjvMfYxd4HeCv+Y+F+I/30sBvAcf5j7ULvA7w1/zHQfzHOg78FfBg/nPcCrwMsMt/DMR/rM8GPov/XJ8DfDb/MRD/cR4MPJ3/Gg8BbuXfD/Ef56uBj+K/xtcAH82/H+I/zkXgOP81bgUewr8f4j/GawO/xX+t1wF+m38fxH+M9wa+i/9a7wN8N/8+iP8Y3wW8N/+1vht4H/59EP92DwY+C3hv/nt9N/A5wK386yH+bd4b+C7+Z3kf4Lv510H867038F38z/Q+wHfzokP86zwYeDr/sz0EuJUXDeJf57uB9+J/tu8B3psXDeJfx/zvIF40iBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7MPB0/nd4CHAr/zLEv85fAy/F/2x/A7w0LxrEv85bAz/F/2xvA/w0LxrEv953A+/F/0zfA7w3LzrEv95x4LuBt+Jfdgn4aeC9+Lf5HuCtgWP8y34GeG9glxcd4t/uo4HPBo7xgn0P8N7Ag4GPBt4bOMYLdwn4buCrgVuB7wbeixfsEvDVwGfzr4f493tv4MHAewMP4nm9DPDXPNtrAy8NHAcezBW3ArvAXwO/zbM9GHg6z+sS8NXArcB382+H+I/z3sB38bx+G3gd/m2+G3gvntf7AN/Nvx/iP9ZfAy/F83od4Lf513kw8HSe1zOAB/MfA/Ef67WB3+J5/TbwOvzrfDfwXjyv9wG+m/8YiP94vw28Fs/rdYDf5kXzYODpPK9nAA/mPw7iP95rA7/F8/pt4HV40fwU8NY8r/cBvpv/OIj/HL8NvBbP632A7+aFe23gt3hezwAezH8sxH+O1wZ+i+d1K/AQXrjfAl6b5/U+wHfzHwvxn+e7gffieb0P8N08f68N/BbP62+Al+Y/HuI/z4OBp/O8bgUewvP3W8Br87xeB/ht/uMh/nN9N/BePK/3Ab6b5/TawG/xvH4HeG3+cyD+cz0YeDrPaxd4CLDLs/0W8No8r9cBfpv/HIj/fN8NvBfP63OAz+aK1wZ+i+f1O8Br858H8Z/vwcBfA8d4TrvAQ4Bd4LeA1+Z5vQ7w2/znQfzX+Gzgs3henwP8NvBbPK/fAV6b/1yI/xrHgVuBYzynXeCvgdfmeb0O8Nv850L81/ls4LN40fwO8Nr850P81zkO3Aoc41/2OsBv858P8V/rs4HP4oX7HeC1+a+B+K93K/AgXrCXAf6a/xqI/3rvDXwXz9/3AO/Nfx3Ef49bgQfxvB4C3Mp/HcR/j/cGvovn9D3Ae/NfC/Hf51bgQTzbQ4Bb+a+F+O/z2sBvccX3AO/Nfz3Ef6/fBl4LeAhwK//1EP+9Xht4b+C9+e+B+O93HNjlvwfi/zfE/2/8I7bqq0EeJcADAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdIcecream;
impl IconShape for MdIcecream {
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
                d: "M8.79,12.4l3.26,6.22l3.17-6.21c-0.11-0.08-0.21-0.16-0.3-0.25 C14.08,12.69,13.07,13,12,13s-2.08-0.31-2.92-0.84C8.99,12.25,8.89,12.33,8.79,12.4z M6.83,12.99C5.25,12.9,4,11.6,4,10 c0-1.49,1.09-2.73,2.52-2.96C6.75,4.22,9.12,2,12,2s5.25,2.22,5.48,5.04C18.91,7.27,20,8.51,20,10c0,1.59-1.24,2.9-2.81,2.99 L12.07,23L6.83,12.99z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vd6aK36a/x6I/z6fBXw2V3w28Dn810P81zsO/BTw2jyn3wbeBtjlvw7iv9ZLAz8FPJjn71bgbYC/5r8G4r/OewPfxYvmfYDv5j8f4j/fceCrgPfmX+e7gY8BdvnPg/jP9WDgp4CX5t/mr4G3AW7lPwfiP89bA98FHOffZxd4H+Cn+Y+H+M/xWcBn8x/rs4HP4T8W4j/WceCngNfmP8dvA28D7PIfA/Ef56WBnwIezH+uW4G3Af6afz/Ef4z3Br6L/1rvA3w3/z6If5/jwFcB781/j+8G3od/O8S/3YOBnwJemv9efw28DXAr/3qIf5u3Br4LOM7/DLvA2wC/zb8O4l/vs4DP5n+mzwY+hxcd4kV3HPgp4LX5n+2ngfcBdvmXIV40Lw38FPBg/ne4FXgb4K954RD/svcGvov/fXaBjwG+mxcM8S97b+C7+N/nEvDRwHfzgiFeNC8N/DTwIP53eAbw1sBf88IhXnTHgZ8GXov/2X4GeG9gl38Z4l/vs4HP4n+mzwE+mxcd4t/mrYHvBo7xP8Ml4K2B3+ZfB/Fv92Dgp4GX4r/X3wBvDdzKvx7i3+c48NXAe/Hf43uA9+bfDvEf472B7+K/ziXgo4Hv5t8H8R/npYGfBh7Ef65nAG8N/DX/foj/WMeBnwZei/8cPwO8N7DLfwzEv85LAx8FfAywywv22cBn8R/rc4DP5gU7DnwX8DnAX/OiQbzoPgr4aq64FXgb4K95wd4a+G7gGP8+l4C3Bn6bF+ylgZ8CHswVHw18Df8yxL/sOPBVwHvznHaBjwG+mxfswcBPAy/Fv83fAG8N3MoL9t7Ad/G8vhv4GGCXFwzxL/tu4L14wb4beB9esOPAVwPvxb/O9wDvzQt2HPgq4L15wb4HeG9eMMS/7Djw3cBb8YL9NfA2wK28YO8NfBf/skvARwPfzQv20sB3AS/NC/YzwHsDu7xgiBfdRwNfxQu2C7wN8Nu8YC8N/DTwIJ6/ZwBvDfw1L9hbA98FHOcF+xjgq/mXIf51Xhv4aeAYL9hnA5/DC3Yc+GngtXhOPwO8N7DLC/ZVwEfzgl0C3hr4bV40iH+948BvAy/FC/bTwPsAu7xgnw18Fld8DvDZvGDHgZ8CXpsX7G+A1wZ2edEh/u2+GvgoXrBbgbcB/poX7LW54rd5wV4a+C3gOC/Y1wAfzb8e4t/nvYGvBo7x/O0CHwN8N/82HwV8NS/YJeCjge/m3wbx7/fSwHcDL8UL9t3A+/CiOw58FfDevGB/A7w38Nf82yH+YxwHvht4K16wvwbeBriVF+6lge8CXpoX7GeA9wZ2+fdB/Mf6aOCreMF2gbcBfpvn762B7wKO84J9DPDV/MdA/Md7beCngWO8YJ8NfA7P6auAj+YFuwS8NfDb/MdB/Oc4Dvw28FK8YD8NvA9X/BTw2rxgfwO8NrDLfyzEf66vBj6KF+xW4DhwnBfsa4CP5j8H4j/fewNfDRzjX+cS8NHAd/OfB/Ff46WB7wZeihfN3wDvDfw1/7kQ/3WOA98NvBUv3M8A7w3s8p8P8V/vo4Gv4vn7GOCr+a+D+O/x2sBPA8e44hLw1sBv818L8d/nOPDbXPHawC7/9RD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R+/hxEHtNfgLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLayers;
impl IconShape for MdLayers {
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
                d: "M11.99 18.54l-7.37-5.73L3 14.07l9 7 9-7-1.63-1.27-7.38 5.74zM12 16l7.36-5.73L21 9l-9-7-9 7 1.63 1.27L12 16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLayersClear;
impl IconShape for MdLayersClear {
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
                d: "M19.81 14.99l1.19-.92-1.43-1.43-1.19.92 1.43 1.43zm-.45-4.72L21 9l-9-7-2.91 2.27 7.87 7.88 2.4-1.88zM3.27 1L2 2.27l4.22 4.22L3 9l1.63 1.27L12 16l2.1-1.63 1.43 1.43L12 18.54l-7.37-5.73L3 14.07l9 7 4.95-3.85L20.73 21 22 19.73 3.27 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLiquor;
impl IconShape for MdLiquor {
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
                d: "M3,14c0,1.3,0.84,2.4,2,2.82V20H3v2h6v-2H7v-3.18C8.16,16.4,9,15.3,9,14V6H3V14z M5,8h2v3H5V8z",
            }
            path {
                d: "M20.63,8.54l-0.95-0.32C19.28,8.09,19,7.71,19,7.28V3c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v4.28 c0,0.43-0.28,0.81-0.68,0.95l-0.95,0.32C11.55,8.82,11,9.58,11,10.44V20c0,1.1,0.9,2,2,2h7c1.1,0,2-0.9,2-2v-9.56 C22,9.58,21.45,8.82,20.63,8.54z M16,4h1v1h-1V4z M13,10.44l0.95-0.32C15.18,9.72,16,8.57,16,7.28V7h1v0.28 c0,1.29,0.82,2.44,2.05,2.85L20,10.44V12h-7V10.44z M20,20h-7v-2h7V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/vNd4opj/Of7HuC9eU6I5++3gdfiP9/3cMV78Z/vd4DX5jkhnr/fBl6L/3wvwxV/xX++3wFem+eEeP5+G3gt/nP9DfDSXPHXwEvxn+t3gNfmOSGev98GXov/XO8DfDdXfDTwVfzn+h3gtXlOiOfvt4HX4j/PJeDBwC5XHAcu8p/rd4DX5jkhnr/fBl6L/zzfA7w3z+m7gffiP8/vAK/Nc0I8f78NvBb/eV4G+Gue02sDv8V/nt8BXpvnhHj+fht4Lf5zPAN4MM/frcCD+M/xO8Br85wQz99vA6/Ff46PAb6a5++zgc/iP8fvAK/Nc0I8f78NvBb/OU4Auzx/Dwaezn+O3wFem+eEeP6+Gnht4KX4lz0D+G6e02/znP4a2OVf56WB4zzbg4EH82zHgfcGjvEv+xvgt4GP5jkhXrgHA18NvBUv3HcD78N/re8C3psX7neA9wZu5flDvGi+G3gvXri/Bl4H2OU/13Hgt4CX5oX7HuC9eeEQL5rjwK3AMV64W4G3Af6a/xwvDfwU8GBeuEvAg4FdXjjEi+6rgY/iX7YLfAzw3fzHem/gq4Dj/Mu+Bvho/mWIF917A9/Fi+6zgc/hP8ZnAZ/Ni+59gO/mX4Z40X008FX86/w08D7ALv82x4HvAt6af533Ab6bfxniRffVwEfxr/fXwOsAu/zrHAd+C3hp/vW+Bvho/mWIF81x4OnAcf5tHgLcyr/OSwN/xb/NLvAQYJcXDvGi+S7gvfm3eQbwYP5tdoFj/Nt8N/A+vHCIF+7BwHcBr82/3fcA782/zU8Db8W/3W8D7wPcyvOHeP6+Gngt4KX593sf4Lv5t3lv4Lv49/tr4HeAj+Y5IZ6/3wZei/8YDwFu5fk7zhW7PH8PBp7Of4zfAV6b54R4/n4beC3+/f4GeGmev/cCvporPhr4Hp6/W4EH8e/3O8Br85wQz99vA6/Fv9/XAB/Nc3ow8F3Aa/Ocfht4H+BWntN3A+/Fv9/vAK/Nc0I8f78NvBb/fm8D/DTP9lHAZwPHef52gc8GvoZne2vgp/j3+x3gtXlOiOfvt4HX4t/vBLALvDTwXcBL86L5a+B9gL8GjgMX+ff7HeC1eU6I5++3gdfi3+dvgJcGPgv4bP5tPhv4HOCvgZfi3+d3gNfmOSGev98GXot/n98GHgw8mH+fW4Fbgdfm3+d3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7buC9+L/la4CP5jkhnr+PBr6K/1s+BvhqnhPi+TsO3Aoc4/+GS8CDgV2eE+IFe2/gu/i/4W2An+Z5IV641wa+G3gQ/zs9A3hv4Ld5/hAvmpcGjvO/yy7w17xwiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPMX6zQfE6ROcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalActivity;
impl IconShape for MdLocalActivity {
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
                d: "M20 12c0-1.1.9-2 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.9-1.99 2v4c1.1 0 1.99.9 1.99 2s-.89 2-2 2v4c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-4c-1.1 0-2-.9-2-2zm-4.42 4.8L12 14.5l-3.58 2.3 1.08-4.12-3.29-2.69 4.24-.25L12 5.8l1.54 3.95 4.24.25-3.29 2.69 1.09 4.11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fd4LeGmu+Gvge/ivh/jv8VvAa/Ocfht4Hf5rIf7rfTbwWTx/nwN8Nv91EP/1/gp4aZ6/vwZehv86iP965oUT/3UQ//XMCyf+6yD+65kXTvzXQfzXMy+c+K+D+K9nXjjxXwfxX8+8cOK/DuK/nnnhxH8dxH8988KJ/zqI/3rmhRP/dRD/9cwLJ/7rIP7rmRdO/NdB/NczL5z4r4P4r2deOPFfB/Ffz7xw4r8O4r+eeeHEfx3Ef50HA98FvDYv3G8D7wPcyn8+xH++48BHAZ/Nv85nA18D7PKfB/Gf672AzwYezL/NrcBnA9/Dfw7Ef46XBr4KeG3+Y/w28DHAX/MfC/Ef6zjwWcBH85/jq4HPAXb5j4H4j/NRwGcDx/nPtQt8NvA1/Psh/v1eG/gq4KX5r/XXwMcAv82/HeLf7sHAZwHvzX+v7wY+B7iVfz3Ev81nAR8NHOd/hl3gq4HP4V8H8a/z2sB3AQ/mf6ZbgfcBfpsXDeJF82Dgu4DX5n+H3wbeB7iVFw7xwh0HPgr4bP53+mzga4Bdnj/EC/ZewGcDD+bf72eA3wZ+Gng6L9xDgLcG3hp4Lf79bgU+G/genhfi+ftu4L34t3sG8NvATwO/DezybOaFE892HHht4K2BtwaO8W/3PcB785wQz99vA6/Fv87fAD8N/DTw17xg5oUTL9hLA28NvDXwUvzr/A7w2jwnxPP328Br8cJdAn4b+Gngp4FdXjTmhRMvmgcDrw28NfDawDFeuN8BXpvnhHj+fht4LZ7XM4CfBn4a+G3+bcwLJ/5tXht4a+CtgQfxvH4HeG2eE+L5+23gtbjiZ4DfBn4auJV/P/PCiX+/BwNvDbw28FZc8TvAa/OcEM/fRwO3Ar8N7PIfy7xw4j/WceC1gQcDX81zQvzXMy+c+K+D+K9nXjjxXwfxX8+8cOK/DuK/nnnhxH8dxH8988KJ/zqI/3rmhRP/dRD/9cwLJ/7rIP5rHQcu8sKdAHb5r4H4r/NRwGcDx3nhdoHPBr6G/3yI/3yvDXwV8NL86/w18DHAb/OfB/Gf5zjwVcB78+/z3cDHALv8x0P853hv4KuA4/zH2AU+Bvhu/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8fw9A/ht4LeB3+aK1wZeG3ht4EE8f78DvDb/sRD/OX4beC2ueAbw28BvA78N3MoL92DgtYHXBl4beBBX/A7w2vzHQvzn+GhgF/ht4Fb+fR4MvDZwHPhq/mMh/n9D/P+G+P8N8f8b4v83/hGmI5RB47bm9wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalAirport;
impl IconShape for MdLocalAirport {
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
                d: "M0,0h24v24H0V0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalAtm;
impl IconShape for MdLocalAtm {
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
                d: "M11 17h2v-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1h-3v-1h4V8h-2V7h-2v1h-1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h3v1H9v2h2v1zm9-13H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4V6h16v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Jf9NfBSPH+fA/w2/71eGvgqnr+/AV6aFwzxLzsO/DXwIJ7XLvA6wF/z3+Olgd8CjvO8ngG8NLDLC4Z40bw08NvAMZ7XLvAQYJf/WseBpwPHeV6XgNcG/poXDvGie2ngt4FjPK+/Bl4H2OW/xnHgt4CX5nldAl4b+Gv+ZYh/nfcGvovn76+Bl+G/xl8BL83z9z7Ad/OiQfzrvTfwXTx/3w18N/+53ht4b56/9wG+mxcd4t/mq4GP4n+WrwE+mn8dxL/ddwPvxf8M3wO8N/96iH+f3wZei/9evwO8Nv82iH+f48BvAy/Ff4+/AV4b2OXfBvHvdxz4a+BB/Nd6BvDSwC7/doj/GC8N/DZwjP8al4DXBv6afx/Ef5yXBn4bOMZ/rkvAawN/zb8f4j/WewPfxX+u9wG+m/8YiP947w18F/853gf4bv7jIP5zfDXwUfzH+hrgo/mPhfjP893Ae/Ef43uA9+Y/HuI/128Dr8W/z+8Ar81/DsR/ruPAbwMvxb/N3wCvDezynwPxn+848NfAg/jXeQbw0sAu/3kQ/zVeGvht4BgvmkvAawN/zX8uxH+dlwZ+GzjGC3cJeG3gr/nPh/iv9d7Ad/HCvQ/w3fzXQPzXe2/gu3j+3gf4bv7rIP57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiH+/1+Jf76uBl+Y5/TXw0fzr/Q7/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPNStxQaTcv5kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalBar;
impl IconShape for MdLocalBar {
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
                d: "M21 5V3H3v2l8 9v5H6v2h12v-2h-5v-5l8-9zM7.43 7L5.66 5h12.69l-1.78 2H7.43z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADsElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fv873Er8N3A1wC7PH+IF91rA7/F/z63Aq8D3MrzQrzoXhv4Lf53+mvgZXheiBfdawO/xf9ebwP8NM8J8aJ7beC3+N/rc4DP5jkhXnSvDfwW//O8Ds/rq4GX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7nEc/rt4HX4jn9DvDaPCfEi+61gd/ifx7xvH4beC2e0+8Ar81zQrzoXhv4Lf7n+W2e10sDx3lOPwO8Nc8J8aJ7beC3+N/rc4DP5jkhXnSvDfwW/3s9BLiV54R40b028Fv87/Q1wEfzvBAvutcGfov/fb4G+GieP8SL7rWB3+J/nt/hee0Cfw38NPDXvGCIF91rA7/F/zzi3w7xontt4Lf4n0f82yFedK8N/Bb/84h/O8SL7rWB3+J/HvFvh3jRvTbwW/zPI/7tEC+61wZ+i/95xL8d4kX32sBv8T+P+LdDvOheG/gt/ucR/3aIF91rA7/F/yyXgOP82yFedC8N/BX/s/wO8Nr82yH+dW4FHsT/HF8DfDT/doh/ne8G3ov/OV4H+G3+7RD/Og8Gns7/DL8DvDb/Poh/va8GPor/fq8D/Db/Poh/m78GXor/Ph8DfDX/foh/m+PAbwMvxX+9zwE+m/8YiH+frwY+iv8azwA+Gvhp/uMg/v0eDHw28NrAg/iP9zfAdwPfDezyHwvxH+ulgeP8x/lrYJf/PIj/3xD/vyH+f0P8/4Z40R0HXor/Hf4G2OVfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RLP9iQV+xVrAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalCafe;
impl IconShape for MdLocalCafe {
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
                d: "M20 3H4v10c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.11 0 2-.9 2-2V5c0-1.11-.89-2-2-2zm0 5h-2V5h2v3zM4 19h16v2H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalCarWash;
impl IconShape for MdLocalCarWash {
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
                d: "M17 5c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zm-5 0c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zM7 5c.83 0 1.5-.67 1.5-1.5C8.5 2.5 7 .8 7 .8S5.5 2.5 5.5 3.5C5.5 4.33 6.17 5 7 5zm11.92 3.01C18.72 7.42 18.16 7 17.5 7h-11c-.66 0-1.21.42-1.42 1.01L3 14v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 18c-.83 0-1.5-.67-1.5-1.5S5.67 15 6.5 15s1.5.67 1.5 1.5S7.33 18 6.5 18zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 13l1.5-4.5h11L19 13H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP8z3IJ+Gv+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI5++lgWO8YF8NvDQv2N8Au7xgLw0c4zldAv6aK14aOMYVv8MVx4GX4gX7a+CjecEuAX/Nc0I8f78NvBb/dq8D/DYv2G8Dr8Vz+h3gtbnit4HX4gpxxWsDv8W/3e8Ar81zQjx/vw28Fv92rwP8Ni/YbwOvxXP6HeC1ueK3gdfiCnHFawO/xb/d7wCvzXNCPH+/DbwW/3Z/DezynP4G+Giu+G3gtXhOvwO8Nlf8NvBaXCGueG3gt/i3+x3gtXlOiOfvt4HX4j/W7wCvzRW/DbwWz+l3gNfmit8GXosrxBWvDfwW/3a/A7w2zwnx/P028Fr8x/od4LW54reB1+I5/Q7w2lzx28BrcYW44rWB3+Lf7neA1+Y5IZ6/3wZei3+71wF+mxfst4HX4jn9DvDavGCvDfwW/3a/A7w2zwnx/P028Fr8270O8Nu8YL8NvBbP6XeA1+YFe23gt/i3+x3gtXlOiOfvt4HX4t/udYDf5orX4nl9NfDSPKe/Bj6a5/U7XPHawG/xb/c7wGvznBDP328Dr8W/3esAv80V5t9HXPHawG/xb/c7wGvznBDP328Dr8W/3esAv80V5t9HXPHawG/xb/c7wGvznBDP328Dr8W/3esAv80Vr83z+mrgpXjRiCteG/gt/u1+B3htnhPi+ftt4LX4t3sd4Ld5wX4beC1eNOKK1wZ+i3+73wFem+eEeP5+G3gt/u1eB/htXrDfBl6LF4244rWB3+Lf7neA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/WJeAv+ZF89LAMf5j/Q7w2jwnxPP328Br8R/rd4DX5kXz28Br8R/rd4DX5jkhnr/fBl6L/1i/A7w2L5rfBl6L/1i/A7w2zwnx/P028Fr8x/od4LV50fw28Fr8x/od4LV5Tojn77eB1+I/1u8Ar82L5reB1+I/1u8Ar81zQjx/vw28Fv+xfgd4bV40vw28Fv+xfgd4bZ4T4vn7beC1+I/1O8Br86L5beC1+I/1O8Br85wQz99vA6/Ff6zfAV6bF81vA6/Ff6zfAV6b54R4/n4beC3+Y/0O8Nq8aH4beC3+Y/0O8No8J8Tz99vAa/Ef63eA1+ZF89vAa/Ef63eA1+Y5IZ6/3wZei/9YvwO8Ni+a3wZei/9YvwO8Ns8J8fy9NHCc/1i7wF/zonlp4Dj/sXaBv+Y5If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Ak/NrkF9/CcjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalConvenienceStore;
impl IconShape for MdLocalConvenienceStore {
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
                d: "M19 7V4H5v3H2v13h8v-4h4v4h8V7h-3zm-8 3H9v1h2v1H8V9h2V8H8V7h3v3zm5 2h-1v-2h-2V7h1v2h1V7h1v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalDining;
impl IconShape for MdLocalDining {
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
                d: "M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/st/if7fX4QVD/MtuBR7E/07PAB7MC4b4l/028Fr87/Q7wGvzgiH+ZT8NvBX/O/0M8Na8YIh/2WcDn8X/Tp8DfDYvGOJf9tnAZ/G8fof/WV6L5/U5wGfzgiH+Za8N/BbPS/zPYp7X6wC/zQuG+Je9NvBbPK+XAf6a/xleG/gtntfrAL/NC4b4lx0HLvK8Xgf4bf5neG3gt3heJ4BdXjDEi8Y8r/cBvpv/GT4a+Cqel3jhEC+aW4EH8Zw+B/hs/mf4bOCzeE7PAB7MC4d40fw28Fo8p88BPpv/Gb4a+Cie0+8Ar80Lh3jR/DTwVjyn3wFem/8Zfht4LZ7TzwBvzQuHeNF8NvBZPKffAV6b/xl+G3gtntPnAJ/NC4d40Xw28Fk8p13gBP8zmOf1OcBn88IhXjSvDfwWz0v8z2Ce1+sAv80Lh3jRvDbwWzyv3+Z/htfmeb0O8Nu8cIgXzXHgIv+7nAB2eeEQLzrzv4v4lyFedLcCD+J/h2cAD+ZfhnjR/TbwWvzv8DvAa/MvQ7zofht4Lf53+BngrfmXIV50nw18Fv87fA7w2fzLEC+6zwY+i/9Yf8MVL8V/rM8BPpt/GeJF99bAT/Ef5xnAS3PFXwMP4j/O6wC/zb8M8aJ7beC3+I/zMsBfc8VLA3/Ff5zXAX6bfxniRfdg4On8x/gY4Kt5Th8NfBX/MU4Au/zLEP865t/vZ4C35vn7aeCt+PcTLxrEv84ucIx/u0vAg4Fdnr/jwK3AMf7tLgHHedEg/nV+G3gt/u3eBvhpXri3Bn6Kf7vfAV6bFw3iX+e3gdfi3+ZngLfmRfPTwFvxb/M7wGvzokH863w28Fn82zwEuJUXzYOBp/Nv8znAZ/OiQfzrfDbwWfzrfQ/w3vzrfDfwXvzrfQ7w2bxoEP86bw38FP96DwFu5V/nwcDT+dd7G+CnedEg/nVeG/gt/nV+B3ht/m1+G3gt/nVeB/htXjSIf50HA0/nX+d9gO/m3+a9ge/iX+chwK28aBD/euZf52WAv+bf5rWB3+JfR7zoEP96u8AxXnTi38e86C4Bx3nRIf71fht4LV504t/uOHCRF93vAK/Niw7xr/fbwGvxonsd4Lf5t3lt4Ld40f0O8Nq86BD/ep8NfBYvuq8BPpp/m68GPooX3ecAn82LDvGv99nAZ/Gi2wUeAuzyr3MceDpwnBfd5wCfzYsO8a/31sBP8a/z3cD78K/zXcB786/zNsBP86JD/Ou9NvBb/Ot9N/A+vGi+C3hv/vVeB/htXnSIf70HA0/n3+a3gfcBbuX5e2ngq4DX5t/mIcCtvOgQ/zbm3+evgZ8GbuWKBwNvDbw0/z7iXwfxb7MLHON/lkvAcf51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzbfDTwVfzP8jHAV/Ovg/i3eTDwdP5neQhwK/86iH+7zwY+i/8ZPgf4bP71EP8+Xw18FP+9Pgf4bP5tEP9+DwbeG3gw8GD+a9wK3Ap8N3Ar/3aI/98Q/78h/n9D/P+G+P+NfwSZa6tBaP5gwAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalDrink;
impl IconShape for MdLocalDrink {
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
                d: "M3 2l2.01 18.23C5.13 21.23 5.97 22 7 22h10c1.03 0 1.87-.77 1.99-1.77L21 2H3zm9 17c-1.66 0-3-1.34-3-3 0-2 3-5.4 3-5.4s3 3.4 3 5.4c0 1.66-1.34 3-3 3zm6.33-11H5.67l-.44-4h13.53l-.43 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalFireDepartment;
impl IconShape for MdLocalFireDepartment {
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
                d: "M19.48,12.35c-1.57-4.08-7.16-4.3-5.81-10.23c0.1-0.44-0.37-0.78-0.75-0.55C9.29,3.71,6.68,8,8.87,13.62 c0.18,0.46-0.36,0.89-0.75,0.59c-1.81-1.37-2-3.34-1.84-4.75c0.06-0.52-0.62-0.77-0.91-0.34C4.69,10.16,4,11.84,4,14.37 c0.38,5.6,5.11,7.32,6.81,7.54c2.43,0.31,5.06-0.14,6.95-1.87C19.84,18.11,20.6,15.03,19.48,12.35z M10.2,17.38 c1.44-0.35,2.18-1.39,2.38-2.31c0.33-1.43-0.96-2.83-0.09-5.09c0.33,1.87,3.27,3.04,3.27,5.08C15.84,17.59,13.1,19.76,10.2,17.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalFlorist;
impl IconShape for MdLocalFlorist {
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
                d: "M12 22c4.97 0 9-4.03 9-9-4.97 0-9 4.03-9 9zM5.6 10.25c0 1.38 1.12 2.5 2.5 2.5.53 0 1.01-.16 1.42-.44l-.02.19c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5l-.02-.19c.4.28.89.44 1.42.44 1.38 0 2.5-1.12 2.5-2.5 0-1-.59-1.85-1.43-2.25.84-.4 1.43-1.25 1.43-2.25 0-1.38-1.12-2.5-2.5-2.5-.53 0-1.01.16-1.42.44l.02-.19C14.5 2.12 13.38 1 12 1S9.5 2.12 9.5 3.5l.02.19c-.4-.28-.89-.44-1.42-.44-1.38 0-2.5 1.12-2.5 2.5 0 1 .59 1.85 1.43 2.25-.84.4-1.43 1.25-1.43 2.25zM12 5.5c1.38 0 2.5 1.12 2.5 2.5s-1.12 2.5-2.5 2.5S9.5 9.38 9.5 8s1.12-2.5 2.5-2.5zM3 13c0 4.97 4.03 9 9 9 0-4.97-4.03-9-9-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalGasStation;
impl IconShape for MdLocalGasStation {
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
                d: "M19.77 7.23l.01-.01-3.72-3.72L15 4.56l2.11 2.11c-.94.36-1.61 1.26-1.61 2.33 0 1.38 1.12 2.5 2.5 2.5.36 0 .69-.08 1-.21v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v16h10v-7.5h1.5v5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5V9c0-.69-.28-1.32-.73-1.77zM12 10H6V5h6v5zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP9tLAV/HC/Qzw1fzfgXi21wZ+ixfuVuAh/N+BeLbXBn6Lf9n7AN/N/w2IZ3tt4Lf4l90KPIT/GxDP9trAb/GieR/gu/nfD/FsLw18Nc/rtXhefw18NP+z/Q2wywuH+Jd9N/Be/O+zC7wO8Ne8YIh/2YOBp/O/018DL8MLhnjRfDfwXvzvJF4wxIvmwcDT+d/nZ4C35gVDvOh+G3gt/nd5GeCvecEQL7rXBn6L/z2+B3hvXjjEv85vA6/F/w4PAW7lhUP867w28Fv8z/c5wGfzL0P86/018FL8z3UJeDCwy78M8a/33sB38T/XxwBfzYsG8W9zK/Ag/ud5BvBgXnSIf5v3Br6L/3neBvhpXnSIf7tbgQfxP8fvAK/Nvw7i3+6jga/if47XAX6bfx3Ev91x4FbgGP/9fgZ4a/71EP8+nw18Fv/9HgLcyr8e4t/nOHArcIz/Pl8DfDT/Noh/v88GPov/HpeABwO7/Nsg/v0eDDyd/x6fA3w2/3aI/xjfDbwX/7WeATyYfx/Ef4zfAl6b/1rvA3w3/z6If7/XBn6L/1o/A7w1/36If7+nAw/mv87PAO8N7PLvh/j3eW/gu3hezwBu5T/OLvDXwE8Df81/HMS/3XHgr4AH87weAtzK/3yIf7vPBj6L5/U9wHvzvwPi3+Y48HTgOM/pEvBgYJf/HRD/Nh8NfBXP63OAz+Z/D8S/zVcDH8Xzehvgp/nfA/Fv893Ae/E/y+8Ar82/DuLf5qOBr+J/lt8BXpt/HcS/zXHgVuAY/3P8DvDa/Osg/u3eG/gu/uf4HeC1+ddB/Pu8NvDdwIP47/c7wGvzr4P4j/HSwHH+e+0Cf82/DuL/N8T/b4h/n+PARwHvDTwY2AV+Gvgc4Fb+Yz0Y+CzgrYHjwK3AdwNfA+zyb4P4t3sw8FPAS/P8vQ/w3fzHeG/gu3j+/hp4G+BW/vUQ/3Z/Bbw0L9xDgFv593kw8HReuL8GXoZ/PcS/zVsDP8W/7HuA9+bf57uB9+Jf9jbAT/Ovg/i3+Wzgs/iX7QIn+Pe5CBznX/Y5wGfzr4P4t/lq4KN40Yh/H/Oi+RngrfnXQfzbfDbwWfzLngE8mH+fvwZein/Z5wCfzb8O4t/mwcDT+Zd9DvDZ/Pt8NvBZ/MseAtzKvw7i3+6rgY/iBfsb4LWBXf59jgO/DbwUL9jXAB/Nvx7i3+ergY/ief0N8NbArfzHeDDw08BL8by+Bvho/m0Q/34PBt4beGngr4G/Bn6a/xxvDbw08NLAXwPfDdzKvx3i/zfE/2+I/98Q/78h/n/jHwH8C5BB8aANkQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalGroceryStore;
impl IconShape for MdLocalGroceryStore {
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
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H5.21l-.94-2H1zm16 16c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/Mf4HeC1edH8NvBa/McQLxjiX2b+Y/wO8Nq8aH4beC3+Y4gXDPEvM/8xfgd4bV40vw28Fv8xxAuG+JeZ/xi/A7w2L5rfBl6L/xjiBUP8y8x/jN8BXpsXzW8Dr8V/DPGCIf5l5j/G7wCvzYvmt4HX4j+GeMEQ/zLzH+N3gNfmRfPbwGvxH0O8YIh/mfmP8TvAa/Oi+W3gtfiPIV4wxL/M/Mf4HeC1edH8NvBa/McQLxjiX2b+Y/wO8Nq8aH4beC3+Y4gXDPEvMy/c7/Ci+Wvgo3nRfDXw0rxoXosXTrxgiH+ZeeHEfy/zwokXDPEvMy+c+O9lXjjxgiH+ZeaFE/+9zAsnXjDEv8y8cOK/l3nhxAuG+JeZF0789zIvnHjBEP8y88KJ/17mhRMvGOJfZl448d/LvHDiBUP8y8wLJ/57mRdOvGCIf5l54cR/L/PCiRcM8S8zL5z472VeOPGCIf5l5oX7bV40fwN8NC+arwZeihfNa/PCiRcM8S8z/zF+B3htXjS/DbwW/zHEC4b4l5n/GL8DvDYvmt8GXov/GOIFQ/zLzH+M3wFemxfNbwOvxX8M8YIh/mXmP8bvAK/Ni+a3gdfiP4Z4wRD/MvMf43eA1+ZF89vAa/EfQ7xgiH+Z+Y/xO8Br86L5beC1+I8hXjDEv8z8x/gd4LV50fw28Fr8xxAvGOJfZv5j/A7w2rxofht4Lf5jiBcM8S8z/zF+B3htXjS/DbwW/zHEC4b4l5n/GL8DvDYvmt8GXov/GOIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLdoFj/O90CTjOC4b4l3038F787/Q9wHvzgiH+ZQ8Gns7/Tg8BbuUFQ7xo3hv4Lv53eR/gu3nhEC+6BwOfDbw1cIz/mS4BPw18NnAr/zLE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFNh3ZBRAjNrwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalHospital;
impl IconShape for MdLocalHospital {
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
                d: "M19 3H5c-1.1 0-1.99.9-1.99 2L3 19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-1 11h-4v4h-4v-4H6v-4h4V6h4v4h4v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz/bSwFfxvF6H/7sQz/bawG/xvMT/XYhne23gt3he4v8uxLO9NvBbPC/xfxfi2V4b+C2el/i/C/Fsrw38Fs9L/Mtei2d7BnArL9xLA8f4z3EJ+GteNIhne23gt3he4vl7MPBZwHvzvP4a+Bzgp3n+fht4Lf7z3Ar8NvA5wK28YIhne23gt3he4nm9N/Bd/Mu+G/gYYJfn9NvAa/Ff46uBj+H5QzzbawO/xfMSz+m9ge/iRffTwNvwnH4beC3+6/w18DrALs8J8WyvDfwWz0s824OBp/Ov9znAZ/Nsvw28Fv+1/hp4GZ4T4tleG/gtnpd4tu8G3ot/vV3gIcAuV/w28Fr81/sa4KN5NsSzvTbwWzwvccVx4CL/dm8D/DRX/DbwWvz3eAhwK1cgnu21gd/ieYkrXhv4Lf7tPgf4bK74beC1+O/xPcB7cwXi2V4b+C2el7jitYHf4t/uZ4C35orfBl6L/x63Ag/hCsSzvTbwWzwvccVrA7/Fv93nAJ/NFb8NvBb/fV4G+GsA8WyvDfwWz0tc8WDg6fzbfQzw1Vzx28Br8d/ndYDfBhDP9trAb/G8xLP9NfBS/Ns8BLiVK34beC3++7wO8NsA4tleG/gtnpd4trcGfop/ve8B3ptn+23gtfjv8zrAbwOIZ3tt4Ld4XuI5fTfwXrzoLgEPBnZ5tt8GXov/Pq8D/DaAeLbXBn6L5yWe03Hgp4HX4l92CXht4K95Tr8NvBb/fV4H+G0A8WyvDfwWz0s8f58NfDRwjOfve4CPBnZ5Xr8NvBb/fV4H+G0A8WyvDfwWz0u8YMeB1wZeGnhp4K+BXeCngVt5wX4beC3++7wO8NsA4tleG/gtnpf4j/fbwGvx3+d1gN8GEM/22sBv8bzEf7zfBl6L/z6vA/w2gHi21wZ+i+f12/zHe2ngOP99Xgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8f/D6wC/DSCe7bWB3+L/h9cBfhtAPNtrA7/F/w+vA/w2gHi21wZ+i+f1Ovzv9ls8r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tmOAy/N8/pt/nd7bZ7XXwO7AOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I6ySl0GAxBKQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalHotel;
impl IconShape for MdLocalHotel {
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm12-6h-8v7H3V5H1v15h2v-3h18v3h2v-9c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalLaundryService;
impl IconShape for MdLocalLaundryService {
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
                d: "M9.17 16.83c1.56 1.56 4.1 1.56 5.66 0 1.56-1.56 1.56-4.1 0-5.66l-5.66 5.66zM18 2.01L6 2c-1.11 0-2 .89-2 2v16c0 1.11.89 2 2 2h12c1.11 0 2-.89 2-2V4c0-1.11-.89-1.99-2-1.99zM10 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM7 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm5 16c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PV4aeCvgwVxxK/AzwF/zXwvxX+u1ge8CHszzdyvwPsBv818D8V/nvYHv4kXzPsB3858P8V/jvYHv4l/nfYDv5j8X4j/fceDpwHH+dXaBhwC7/OdB/Of7aOCr+Lf5GOCr+c+D+M/328Br8W/zO8Br858H8Z/P/PuI/zyI/3zm30f850H857sVeBD/Ns8AHsx/HsR/vp8G3op/m+8B3pv/PIj/fO8NfBf/Nu8DfDf/eRD/Nf4aeCn+dX4HeG3+cyH+a7w08NvAMV40l4CXBm7lPxfiv85LAz8NPIgX7hnAWwN/zX8+xH+t48BHA+8NPIjn9Azgu4GvBnb5r4H4l/0W8Ns8218Du8Dv8O/z0sBxrtgF/pp/n9cCjgMvzbO9NvA6vGCIf5l5wf4a+G3gr4GfAXb5r3EceCvgtYGXBl6aF0y8YIh/mXnR/TTw08DPALv8xzoOvBXw1sBb86ITLxjiX2b+9XaBrwa+Btjl3+c48FHARwPH+dcTLxjiX2b+7XaBrwa+BtjlX+c48FHAZ/PvI14wxL/M/PvtAh8NfA8vmvcCvho4zr+feMEQ/zLzH+e3gbcBdnn+jgM/Bbw2/3HEC4b4l5n/WLvA+wA/zXN6a+C7gOP8xxIvGOJfZv5zfDTwNVzxWcBn859DvGCIf5n5z/PdXPHe/OcRLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJf9ts823Hgpfif7W+AXZ7ttXnBEP82DwYeDLw28NrAa/Hf43eA3wZ+G7gVuJV/HcR/nLcG3hp4a+AY/zkuAT8N/DTw0/z7If5zvDfw2cCD+I/xDOCzge/mPxbiP9drA58NvBb/Nn8DfDbw0/znQPzXeG/gq4FjvGguAR8NfDf/uRD/dY4Dnw18FC/c1wCfDezynw/xX++tge8GjvGcLgHvDfw0/3UQ/z2OA78NvBRX/A3w2sAu/7UQ/32OA9/NFe8N7PJfD/H/G+L/N8T/b4j/3/hHDoeFQQ3erQcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalLibrary;
impl IconShape for MdLocalLibrary {
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
                d: "M12 11.55C9.64 9.35 6.48 8 3 8v11c3.48 0 6.64 1.35 9 3.55 2.36-2.19 5.52-3.55 9-3.55V8c-3.48 0-6.64 1.35-9 3.55zM12 8c1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3 1.34 3 3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/z2vxnH6H/3qI/zoPBt4LeGvgpXn+/hr4aeB7gFv5z4f4z3cc+CrgvfnX+Wrgc4Bd/vMg/nO9NPBbwHH+bXaB1wH+mv8ciP887w18F/8x3gf4bv7jIf5zvDTwV/zHehngr/mPhfiPdxx4OnCc5+8ZwFcDvw38NVe8NPDawEcDD+L52wUeAuzyHwfxH++7gffi+fsY4Kt54T4a+Cqev68BPpr/OIj/WA8Gns7z9zLAX/OieWngr3j+HgLcyn8MxH+szwY+i+f1McBX86/z0cBX8bw+B/hs/mMg/mP9FfDSPKdnAA/m3+ZW4EE8p78GXob/GIj/WOZ5fQzw1fzbfDTwVTwv8R8D8R/ntYHf4nm9DPDX/Nu8NPBXPK/XAX6bfz/Ef5zXBn6L5yX+fczzeh3gt/n3Q7zoHgx8FvDWwHH+Z9oFfhr4HOBW/mWIF817A9/F/y7vA3w3LxziX/Zg4On87/QQ4FZeMMS/7LuB9+J/p+8B3psXDPEvuwgc53+nXeAELxjiX2ZeNH8DfDTP67f493kdntdXAy/Fi0a8YIh/mXnR/A7w2jyvW4EH8W/zDODBPK/fBl6LF414wRD/MvOiuRV4CM/rp4G34t/me4D35nldBI7zohEvGOJfZl50J4BdntN7A9/Fv837AN/Nc3ow8HRedOIFQ/zLzIvufYDv5nn9NfBS/Ov8DvDaPK/3Br6LF514wRD/MvOi+23gdXheLw38NnCMF80l4KWBW3levwW8Ni868YIh/mXmX+d1gN/meb008NPAg3jhngG8NfDXPK/XBn6Lfx3xgiH+ZeZf57eB1+H5Ow58NPDewIN4Ts8Avhv4amCX5++3gNfmX0e8YIh/mfnX+xrgo3nhXho4zhW7wF/zwn018FH864kXDPEvM/827wN8N/8x3hv4Lv5txAuG+JeZf7uPBr6Gf5/PAj6bfzvxgiH+Zebf56eBjwFu5V/nwcBXAW/Nv494wRD/MvMf47OBnwH+mhfupYG3Aj6b/xjiBUP8y8x/rFuBnwZ2gVu54sHAceCtgQfzH0u8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZrcCD+N/pGcCDecEQ/7LPBj6L/50+B/hsXjDEv+w48NvAS/G/y98Arw3s8oIhXjQPBn4aeCn+d/gb4K2BW3nhEP86bw28NPDa/M/028BfAz/Niwbx/xvi/zfE/2+I/98Q/7/xj/krn0EF5l2mAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalMall;
impl IconShape for MdLocalMall {
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
                d: "M19 6h-2c0-2.76-2.24-5-5-5S7 3.24 7 6H5c-1.1 0-1.99.9-1.99 2L3 20c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-7-3c1.66 0 3 1.34 3 3H9c0-1.66 1.34-3 3-3zm0 10c-2.76 0-5-2.24-5-5h2c0 1.66 1.34 3 3 3s3-1.34 3-3h2c0 2.76-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F8xLP67eB1+I/1u8Ar83zMs/rdYDf5l+GeNG9NvBbPC/xvH4beC3+Y/0O8No8L/O8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+ZfhnjRvTbwWzwv8bx+G3gt/mP9DvDaPC/zvF4H+G3+ZYgX3WsDv8Xz+m2e10sDx/mPtQv8Nc/rtXlerwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gtntfv8LxeGjjGf6xLwF/zvF6L5/U6wG/zL0O86F4b+C2el3hevw28Fv+xfgd4bZ6XeV6vA/w2/zLEi+61gd/ieYnn9dvAa/Ef63eA1+Z5mef1OsBv8y9DvOheG/gtnpd4Xr8NvBb/sX4HeG2el3lerwP8Nv8yxIvutYHf4nmJ5/XbwGvxH+t3gNfmeZnn9TrAb/MvQ7zoXhv4LZ6XeF6/DbwW/7F+B3htnpd5Xq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fs/rdXheXw28FP+x/gb4aJ7Xb/G8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+ZfhnjRvTbwWzwv8bx+G3gt/mP9DvDaPC/zvF4H+G3+ZYgX3WsDv8XzEs/rt4HX4j/W7wCvzfMyz+t1gN/mX4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F83ptntdXAy/Nf6y/Bj6a5/XbPK/XAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfovnJZ7XbwOvxX+s3wFem+dlntfrAL/Nvwzxontt4Ld4XuJ5/TbwWvzH+h3gtXle5nm9DvDb/MsQL7rXBn6L5yWe128Dr8V/rN8BXpvnZZ7X6wC/zb8M8aJ7beC3eF7ief028Fr8x/od4LV5XuZ5vQ7w2/zLEC+61wZ+i+clntdvA6/Ff6zfAV6b52We1+sAv82/DPGie23gt3hev83zemngOP+xdoG/5nm9Ns/rdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F8/odntdLA8f4j3UJ+Gue12vxvF4H+G3+ZYgX3WsDv8XzEs/rt4HX4j/W7wCvzfMyz+t1gN/mX4Z40b028Fs8L/G8fht4Lf5j/Q7w2jwv87xeB/ht/mWIF91rA7/F8xLP67eB1+I/1u8Ar83zMs/rdYDf5l+GeNG9NvBbPC/xvH4beC3+Y/0O8No8L/O8Xgf4bf5liBfdawO/xfMSz+u3gdfiP9bvAK/N8zLP63WA3+Zfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AlAJukHZQLk/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalMovies;
impl IconShape for MdLocalMovies {
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
                d: "M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zF2gb8Gfgb4a/77IF64rwI+mv9cXw18DP89EC/YVwMfxX+N7wbeh/96iOfvpYG/4r/WdwPvw38txPP32cBn8V/vu4H34b8O4vn7aeCteMGeAdwKHAdeiv9Y3w28D/81EM/fbwOvxfO6BHw28NU820sD3w28FP9xvht4H/7zIZ6/3wZei+f1McBX87weDPw1cIz/ON8NvA//uRDP328Dr8VzegbwYF6wrwY+iv9Y3w28D/95EM/fbwOvxXP6HeC1ecE+Gvgq/uN9N/A+/OdAPH+/DbwWz+mvgZfhBfts4LP4z/HdwPvwHw/x/P028Fo8r5cB/prn7+nAg/nP893A+/AfC/H8/TbwWjyvvwbeBriV5/RdwHvzn++7gffhPw7i+ftt4LV4/naB7wZuBY4D7w08mP863w28D/8xEM/fbwOvxf9c3w28D/9+iOfvt4HX4n+27wbeh38fxPP328Br8T/fdwPvw78d4vn7beC1+N/hu4H34d8G8fz9NvBa/O/x3cD78K+HeP5+G3gt/nf5buB9+NdBPH+/DbwW//t8N/A+vOgQz99vA6/F/07fDbwPLxrE8/fbwGvxv9d3A+/Dvwzx/P028Fr87/bdwPvwwiGev58G3or//b4beB9eMMTz99nAZ/F/w3cD78Pzh3j+Xhr4K/7v+Brgo3leiBfsq4GP4v+OhwC38pwQL9x3A+/F/w2fA3w2zwnxL/tu4L343+9ngLfmOSFeNN8NvBf/u30O8Nk8J8SL7ruB9+J/r7cBfprnhPjX+W7gvfjf52+Al+Z5If71vht4L/73uAS8NHArzwvxb/PdwHvxP98l4LWBv+b5Q/zbfTfwXvzPdQl4beCvecEQ/z7fDbwX//NcAl4b+GteOMS/33cD78X/HJeA1wb+mn8Z4j/GdwPvxX+/S8BrA3/NiwbxH+e7gffiv88l4LWBv+ZFh/iP9d3Ae/Ff7xLw2sBf86+D+I/33cB78V/nEvDawF/zr4f4z/HdwHvxn+8S8NrAX/Nvg/jP893Ae/Gf5xLw2sBf82+H+M/13cB78R/vEvDawF/z74P4z/fdwHvxH+cS8NrAX/Pvh/iv8d3Ae/Hvdwl4beCv+Y+B+K/z3cB78W93CXht4K/5j4P4r/XdwHvxr3cJeG3gr/mPhfiv993Ae/GiuwS8NvDX/MdD/Pf4buC9+JddAl4b+Gv+cyD++3w38F68YJeA1wb+mv88iP9eXw18FM/rb4C3Bm7lPxfiv99LA28NvDTw18BfAz/Nfw3E/2+I/98Q/78h/n9D/P/GPwJ3i5hBzdcvtAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalOffer;
impl IconShape for MdLocalOffer {
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
                d: "M21.41 11.58l-9-9C12.05 2.22 11.55 2 11 2H4c-1.1 0-2 .9-2 2v7c0 .55.22 1.05.59 1.42l9 9c.36.36.86.58 1.41.58.55 0 1.05-.22 1.41-.59l7-7c.37-.36.59-.86.59-1.41 0-.55-.23-1.06-.59-1.42zM5.5 7C4.67 7 4 6.33 4 5.5S4.67 4 5.5 4 7 4.67 7 5.5 6.33 7 5.5 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zP/sX6bK34a+B3gr/nPg/j3M/+5bgU+G/ge/uMh/v3Mf41bgY8Bfpr/OIh/P/Nf66uBj+E/BuLfz/zX+2ngfYBd/n0Q/37mv8d3A+/Dvw/i38/89/kY4Kv5t0P8+5n/Xg8BbuXfBvHvZ/57/TbwOvzbIP79zH+/1wF+m389xL+fedFcAv6af9mDgQfxr/M9wHvzr4f49zMvmt8BXpsXzWsDXw28FC+aXeAE/3qIfz/zovkd4LV50R0Hfht4KV40rwP8Nv86iH8/86L5HeC1+dd5MPB0XjQfA3w1/zqIfz/zovkd4LX51/tp4K34l30O8Nn86yD+/cyL5neA1+Zf77OBz+Jf9jXAR/Ovg/j3My+a3wFem3+9zwY+i3/Z1wAfzb8O4t/PvGh+B3ht/vV+Gngr/mWfA3w2/zqIfz/zovkd4LX513kw8HReNB8DfDX/Ooh/P/Oi+R3gtXnRHQd+C3hpXjSvA/w2/zqIfz/zovkd4LV50bw28FXAS/OiuQQc518P8e9nXjS7wF/zL3sw8GD+db4HeG/+9RD/fua/3+sAv82/HuLfz/z3+h3gtfm3Qfz7mf9eDwFu5d8G8e9n/vt8DPDV/Nsh/v3Mf4/vAd6bfx/Ev5/5r/czwFvz74f49zP/tb4G+Gj+YyD+/cx/jWcAHw38NP9xEP9+5j/XM4DPBr6b/3iIfz/zH+t3uOKngd8G/pr/PIh/P/Oi+R3gtfmfBfHvZ140vwO8Nv+zIP79zIvmd4DX5n8WxL+fedH8DvDa/M+C+PczL5rfAV6b/1kQ/37mRfM7wGvzPwvi38+8aH4HeG3+Z0H8+5kXze8Ar83/LIh/P/Oi+R3gtfmfBfHvZ140vwO8Nv+zIP79zIvmd4DX5n8WxL+fedH8DvDa/M+C+PczL5rfAV6b/1kQ/37mRfM7wGvzPwvi38+8aH4HeG3+Z0H8+5kXze8Ar83/LIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I+/IgUHmv0+YAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalParking;
impl IconShape for MdLocalParking {
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
                d: "M13 3H6v18h4v-6h3c3.31 0 6-2.69 6-6s-2.69-6-6-6zm.2 8H10V7h3.2c1.1 0 2 .9 2 2s-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/p+Ncscu/D+J/p68GPgr4aeC7gZ/h3wbxv9NF4DjPtgv8NPA1wF/zokP87/PWwE/x/P0N8NK86BD/+3w38F48fx8DfDUvOsT/LseBi7xgJ4BdXnSI/13eG/gunr+fAd6afx3E/y5/Bbw0z9/7AN/Nvw7if48HA0/n+bsEHOdfD/G/x2cDn8Xz9z3Ae/Ovh/jf4+nAg3n+Xgf4bf71EP+y3+J/htfmBfttXrDX4QVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLfhp4K/53+hngrXnBEP+y9wa+i/+d3gf4bl4wxL/sOHCR/51OALu8YIgXzU8Db8X/Lj8DvDUvHOJF897Ad/G/y/sA380Lh3jRHAcu8h/ndYDf5jm9NvBb/Mc5AezywiFedD8NvBX/MV4H+G2e02sDv8V/jJ8B3pp/GeJF997Ad/Ef43WA3+Y5vTbwW/zHeB/gu/mXIV50x4GL/Md4HeC3eU6vDfwW/zFOALv8yxD/Oj8NvBX/fq8D/DbP6bWB3+Lf72eAt+ZFg/jXeW/gu/j3ex3gt3lOrw38Fv9+7wN8Ny8axL/OceAi/36vA/w2z+m1gd/i3+8EsMuLBvGv99PAW/Hv8zrAb/OcXhv4Lf59fgZ4a150iH+99wa+ixfudYDf5j/WawO/xQv3PsB386JD/OsdBy7ywr0O8Nv8x3pt4Ld44U4Au7zoEP82vw28Fi/Y6wC/zX+s1wZ+ixfsd4DX5l8H8W9zETjOC/Y6wG/zH+u1gd/iBdsFTvCvg/jXe2vgp3jhXgf4bf5jvTbwW7xwbwP8NC86xL/edwPvxQv318AuL9jHAH/Nc3pp4Kt4wY4DL80L9z3Ae/OiQ/zrXQSO8+/zOsBv85xeG/gt/n12gRO86BD/Om8N/BT/fq8D/DbP6bWB3+Lf722An+ZFg/jX+W7gvfj3ex3gt3lOrw38Fv9+3wO8Ny8axL/OReA4/36vA/w2z+m1gd/i328XOMGLBvGie2vgp/iP8TrAb/OcXhv4Lf5jvA3w0/zLEC+67wbei/8YrwP8Ns/ptYHf4j/G9wDvzb8M8aK7CBznP8brAL/Nc3pt4Lf4j7ELnOBfhnjRvDXwU/zH+Wtgl+d0HHhp/uO8DfDTvHCIF813A+/F/y7fA7w3LxziRXMROM7/LrvACV44xL/srYGf4n+ntwF+mhcM8S/7buC9+N/pe4D35gVD/MsuAsf532kXOMELhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLcF5JBohZdhgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPharmacy;
impl IconShape for MdLocalPharmacy {
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
                d: "M21 5h-2.64l1.14-3.14L17.15 1l-1.46 4H3v2l2 6-2 6v2h18v-2l-2-6 2-6V5zm-5 9h-3v3h-2v-3H8v-2h3V9h2v3h3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO//Mh/mXfDbwX/3pvA/w0/7Mh/mXm3+Z7gPfmfzbEv8z82/wO8Nr8z4b4l5l/m98BXpv/2RD/sr8GXop/vd8BXpv/2RD/st8GXot/G/E/G+Jf9tvAa/FvI/5nQ/zLvhr4KP5tXgf4bf7nQvzLPhv4LP5tPgb4av7nQvzLXhv4Lf5tfgZ4a/7nQvzLHgw8nX878T8X4kVzK/Ag/m1eB/ht/mdCvGi+G3gv/m1eB/ht/mdCvGjeG/gu/vX+Bnhp/udCvGgeDDydf52/AV4b2OV/LsSL7lbgQbxo/gZ4bWCX/9kQL7qPBr6Kf9nfAK8N7PI/H+JF92Dg6bxwfwO8NrDL/w6If53fBl6L5+9vgNcGdvnfA/Gv897Ad/G8ngG8NLDL/y6If71bgQfxnHaBlwFu5X8XxL/eewPfxfP6HuC9+d8F8W9zK/AgntfLAH/N/x6If5v3Br6L5/XXwMvwvwfi3+63gdfieX0O8Nn874D4t3tp4K94/l4H+G3+50P8+3w18FE8r13gIcAu/7Mh/n2OA78NvBTP66+B1wF2+Z8L8e/30sBvA8d4Xt8NvA//cyH+Y7w38F08f98NvA//MyH+43w38F48f98NvA//Pq/FFb/DfxzEf6zfBl6L5++ngfcBdvnXOQ78FvDSXLEL/Dbw28BfA7/Dvx3iP9Zx4LeBl+L5+2vgdYBdXjTHgd8CXpoXbhf4a57tVuBngJ/mhUP8xzsO/DbwUjx/u8DbAL/NC3cc+C3gpfm3+xjgq3nBEP85jgO/DbwUL9hnA5/D83cc+C3gpfn3uRV4CC8Y4j/PceCngdfiBftr4H2Av+bZjgO/Bbw0/zHEC4b4z/fdwHvxwn038DnALvBbwEvzH0e8YIj/Gu8NfDVwjBdsF9gFHsx/LPGCIf7rvDTw3cBL8V9LvGCI/1rHgc8GPor/OuIFQ/z3eGngq4HX4j+feMEQ/73eG/hs4EH85xEvGOJ/hvcGPht4EP/xxAuG+J/lvYG3Bt6K/zjiBUP8z/Rg4K2BjwYexL+PeMEQ//M9GHht4LWB1wYexIvue4D35gVD/O/zYODBwEsDDwZemiuOAy/Fs30P8NHALi8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IAJR4QS14xVMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPhone;
impl IconShape for MdLocalPhone {
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
                d: "M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4aeBDw0jzbS3PFX/Nsfw08A/hr/vMh/vO8NPBWwGsDr82/zW8Dvw38DPDX/MdD/Md6MPBewHsDD+Y/1q3AdwPfA9zKfwzEf4zXBj4LeG3+a/w08DnAX/Pvg/j3eTDwWcB789/ju4HPAW7l3wbxb/dZwGfz328X+Grgc/jXQ/zrvTTwXcBL8z/LXwPvA/w1LzrEv85bA98FHOd/pl3gbYDf5kWDeNG9N/BdvGguAd8N/DVXvDTw3sAx/mu8D/Dd/MsQL5r3Br6LF833AB8N7PKcjgNfDbwX/zXeB/huXjjEv+y9ge/iRfM7wGvzwv028Fr813gf4Lt5wRD/st8GXosXzUOAW3nhHgw8nf8aPwO8NS8Y4l/2YODp/Mv+BnhpXjR/DbwU//keAtzKC4Z40Xw18FG8cL8DvDYvmt8GXov/XF8DfDQvHOJFcxy4FTjGC/Y7wGvzovlt4LX4z3MJeDCwywuHeNF9NvBZvHAngF1euOPARf5zfQzw1fzLEP86twIP4gX7HOCzeeE+G/gs/vM8A3gwLxrEv85bAz/FC/c+wHfz/L038F3853ob4Kd50SD+9X4beC1euO8Gvgb4a654LeC9gffmP9fvAK/Niw7xr/fawG/xP9PrAL/Niw7xb/PdwHvxP8v3AO/Nvw7i3+bBwF8Dx/if4RLw0sCt/Osg/u0+G/gs/mf4HOCz+ddD/NsdB24FjvHf6xLwYGCXfz3Ev897A9/Fv94l4LuBv+aKlwbeGzjGv977AN/Nvw3i3++vgZfiRfc9wEcDuzyn48BXA+/Fi+5vgJfm3w7x7/fawG/xovkd4LV54X4beC1eNK8D/Db/doj/GL8NvBb/socAt/LCPRh4Ov+y3wFem38fxH+MBwNP54X7G+CledH8NfBSvHAPAW7l3wfxH+ergY/iBfsd4LV50fw28Fq8YF8DfDT/foj/OMeBW4FjPH+/A7w2L5rfBl6L5+8S8GBgl38/xH+szwY+ixfsBLDLC3ccuMgL9jnAZ/MfA/Ef71bgQTx/nwN8Ni/cZwOfxfP3DODB/MdB/Md7b+C7eMHeB/hunr/3Br6LF+xtgJ/mPw7iP8dvA6/FC/bdwNcAf80VrwW8N/DevGC/A7w2/7EQ/zleG/gt/mO9DvDb/MdC/Of5buC9+I/xPcB78x8P8Z/nwcDT+fe7BLw0cCv/8RD/uT4b+Cz+fT4H+Gz+cyD+cx0HbgWO8W9zCXgwsMt/DsR/vvcGvot/m/cBvpv/PIj/GrcCD+Jf52+Al+Y/F+K/xmsDv8W/zusAv81/LsR/nd8GXosXze8Ar81/PsR/nQcDT+dF8xDgVv7zIf5rfTfwXrxw3wO8N/81EP+1jgO3Asd4/i4BDwZ2+a+B+K/32cBn8fx9DvDZ/NdB/Pe4FXgQz+kZwIP5r4X47/HewHfxnN4H+G7+ayH++/w28Fpc8TvAa/NfD/Hf57WB3+KK1wF+m/96iP9e380V781/D8R/rwdzxa3890D8/4b4/w3x/xvi/zfE/2/8IzACrkFIeDD7AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPizza;
impl IconShape for MdLocalPizza {
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
                d: "M12 2C8.43 2 5.23 3.54 3.01 6L12 22l8.99-16C18.78 3.55 15.57 2 12 2zM7 7c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2zm5 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/vNd4opj/Of7HuC9eU6I5++3gdfiP9/3cMV78Z/vd4DX5jkhnr/fBl6L/3wvwxV/xX++3wFem+eEeP5+G3gt/nP9DfDSXPHXwEvxn+t3gNfmOSGev98GXov/XO8DfDdXfDTwVfzn+h3gtXlOiOfvt4HX4j/PJeDBwC5XHAcu8p/rd4DX5jkhnr/fBl6L/zzfA7w3z+m7gffiP8/vAK/Nc0I8f78NvBb/eV4G+Gue02sDv8V/nt8BXpvnhHj+fht4Lf5zPAN4MM/frcCD+M/xO8Br85wQz99vA6/Ff46PAb6a5++zgc/iP8fvAK/Nc0I8f78NvBb/OU4Auzx/Dwaezn+O3wFem+eEeP6+Gnht4KX4lz0D+G6e02/znP4a2OVf56WB4zzbg4EH82zHgfcGjvEv+xvgt4GP5jkhXrgHA18NvBUv3HcD78N/re8C3psX7neA9wZu5flDvGi+G3gvXri/Bl4H2OU/13Hgt4CX5oX7HuC9eeEQL5rjwK3AMV64W4G3Af6a/xwvDfwU8GBeuEvAg4FdXjjEi+6rgY/iX7YLfAzw3fzHem/gq4Dj/Mu+Bvho/mWIF917A9/Fi+6zgc/hP8ZnAZ/Ni+59gO/mX4Z40X008FX86/w08D7ALv82x4HvAt6af533Ab6bfxniRffVwEfxr/fXwOsAu/zrHAd+C3hp/vW+Bvho/mWIF81x4OnAcf5tHgLcyr/OSwN/xb/NLvAQYJcXDvGi+S7gvfm3eQbwYP5tdoFj/Nt8N/A+vHCIF+7BwHcBr82/3fcA782/zU8Db8W/3W8D7wPcyvOHeP6+Gngt4KX593sf4Lv5t3lv4Lv49/tr4HeAj+Y5IZ6/3wZei/8YDwFu5fk7zhW7PH8PBp7Of4zfAV6b54R4/n4beC3+/f4GeGmev/cCvporPhr4Hp6/W4EH8e/3O8Br85wQz99vA6/Fv9/XAB/Nc3ow8F3Aa/Ocfht4H+BWntN3A+/Fv9/vAK/Nc0I8f78NvBb/fm8D/DTP9lHAZwPHef52gc8GvoZne2vgp/j3+x3gtXlOiOfvt4HX4t/vBLALvDTwXcBL86L5a+B9gL8GjgMX+ff7HeC1eU6I5++3gdfi3+dvgJcGPgv4bP5tPhv4HOCvgZfi3+d3gNfmOSGev98GXot/n98GHgw8mH+fW4Fbgdfm3+d3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7buC9+L/la4CP5jkhnr+PBr6K/1s+BvhqnhPi+TsO3Aoc4/+GS8CDgV2eE+IFe2/gu/i/4W2An+Z5IV641wa+G3gQ/zs9A3hv4Ld5/hAvmpcGjvO/yy7w17xwiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPMX6zQfE6ROcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPlay;
impl IconShape for MdLocalPlay {
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
                d: "M20 12c0-1.1.9-2 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.9-1.99 2v4c1.1 0 1.99.9 1.99 2s-.89 2-2 2v4c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-4c-1.1 0-2-.9-2-2zm-4.42 4.8L12 14.5l-3.58 2.3 1.08-4.12-3.29-2.69 4.24-.25L12 5.8l1.54 3.95 4.24.25-3.29 2.69 1.09 4.11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFmUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3ht/nP9NvDdwPfw74f49zkOvBfw0cCD+a91K/DVwPcAu/zbIP5tHgx8FPDewHH+e+0C3w18DXAr/zqIf53XBt4LeG/+Z/pu4HuA3+ZFg3jRvBfw3sBr87/DbwPfDXwPLxziX/Z04MH873Qr8BBeMMS/zPzvJl4wxL/M/O8mXjDEv8z85/kbrngp/vOIFwzxLzP/ed6HK76L/zziBUP8y8x/jkvAca7YBY7xn0O8YIh/mfnP8TXAR3PFVwMfxX8O8YIh/mXmP8dDgFu54sHA0/nPIV4wxL/M/Mf7GeCteU4/DbwV//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/rGcCDef5uBR7EfyzxgiH+ZeY/1scAX83z997Ad/EfS7xgiH+Z+Y9xCfhr4K2BXZ6/48BPAw8GHsR/DPGCIf5l5l/2DOC7ebbf5tl+m3+fBwMP5oqXBo5zxVsDL8W/TLxgiH+Z+ZftAu8D/DT/Nd4b+CrgOP8y8YIh/mXmRfc+wHfzn+ujgK/mRSdeMMS/zPzrfDfwPvzn+C7gvfnXES8Y4l9m/vV+GngfYJf/GMeBnwJem3898YIh/mXm3+avgdcBdvn3OQ78FvDS/NuIFwzxL/tr4KX4t7kVeBvgr/m3eWngt4Dj/Nv8DvDavGCIf9lvA6/Fv90u8DLArfzrPBj4K+A4/3a/A7w2LxjiX/bZwGfx7/MQ4Fb+dR4MPJ1/n88BPpsXDPEve2/gu/i3ewbwYP5tbgUexL/d+wDfzQuG+Je9NPBX/Nv9DPDW/Nv8NPBW/Nu9DPDXvGCIF80ucIx/m48Bvpp/m48Gvop/m0vAcV44xIvmp4G34t/mZYC/5vl7ba74bZ6/lwb+in+b7wHemxcO8aJ5b+C7+Ne7BBzneR0Hvgp4b674buBjgF2e1y5wjH+99wG+mxcO8aI5DlzkX+93gNfmOX0U8NnAcZ7TLvDZwNfwnH4beC3+9U4Au7xwiBfddwPvxb/O5wCfzRWvDXwV8NK8cH8NfAzw21zx2cBn8a/zPcB78y9DvOheG/gt/nVeB/hr4KuA9+Zf57uBjwFeGvgt/nVeB/ht/mWIf53fBl6LF91HA58NHOffZhf4bOCredH9DvDavGgQ/zpvDfwU/7O9DfDTvGgQ/3q/DbwW/zP9DvDavOgQ/3qvDfwW/zO9DvDbvOgQ/zZfDXwU/7N8DfDR/Osg/m2OA38NPIj/GZ4BvDSwy78O4t/utYHf4n+G1wF+m389xL/PRwNfxX+vjwG+mn8bxL/fdwPvxX+P7wHem387xH+M7wbei/9a3wO8N/8+iP8Yx4GfBl6L/xq/A7w2/36I/1jfDbwX/7m+B3hv/mMg/uN9NvBZ/Of4HOCz+Y+D+M/x1sB3A8f4j3EJeGvgt/mPhfjPcxz4buCt+Pf5GeC9gV3+4yH+87028N3Ag/jXeQbw3sBv858H8V/nvYGPBl6KF+5vgK8Gvpv/fIj/ei8NfDTw2sCDuOIZwG8DXw38Nf91EP+9Xpor/pr/Hoj/3xD/v/GPemnFQSfTgA8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPolice;
impl IconShape for MdLocalPolice {
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
                d: "M12,1L3,5v6c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12V5L12,1z M14.5,12.59l0.9,3.88L12,14.42l-3.4,2.05l0.9-3.87 l-3-2.59l3.96-0.34L12,6.02l1.54,3.64L17.5,10L14.5,12.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5HuC9eU6I5++3gdfiOf0N8NPAZ/E/2+cAbw28FM/pd4DX5jkhnr/fBl6L5/Q7wGsDLw18NfBa/M/yO8BHA38N/DbwWjyn3wFem+eEeP5+G3gtntPvAK/Ns3008NnAMf57XQI+G/hqnu23gdfiOf0O8No8J8Tz99vAa/Gcfgd4bZ7TceC7gbfiv8fPAO8N7PKcfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm+Xtt4LuBB/Ff4xnAewO/zfP328Br8Zx+B3htnhPi+ftt4LV4TrcCbwP8Nc/fceCjgc/iP9fnAF8N7PL8vTTwU8CDeU6/A7w2zwnx/P028Fo8f58NfA2wy/P30sBXA6/Ff6zfAT4a+Guev+PARwGfzfP3O8Br85wQz99vA6/FC3Yr8D7Ab/OCfTTw2cAx/n0uAZ8NfDUv2GsD3wU8mBfsd4DX5jkhnr/fBl6Lf9lPA+8D7PL8HQe+G3gr/m1+BnhvYJfn7zjwXcBb8y/7HeC1eU6I5++3gdfiRbMLfDbwNbxgrw18N/AgXjTPAN4b+G1esI8CPhs4zovmd4DX5jkhnr/fBl6Lf53fBj4G+Guev+PARwOfxQv3OcBXA7s8fy8NfBXw2vzr/A7w2jwnxPP328Br8W/z2cDXALs8fy8NfDXwWjyn3wE+Gvhrnr/jwEcBn82/ze8Ar81zQjx/vw28Fv92twLvA/w2L9hHA5/NFZ8NfDUv2GsD3wU8mH+73wFem+eEeP5+G3gt/v1+GngfYJfn7zhX7PL8HQe+C3hr/v1+B3htnhPi+ftt4LX4j7ELfDbwNfzrfBTw2cBx/mP8DvDaPCfE8/fbwGvxH+u3gY8B/poX7qWBrwJem/9YvwO8Ns8J8fz9NvBa/Of4bOBrgF2e03Hgo4DP5j/H7wCvzXNCPH+/DbwW/3luBd4H+G2ueG3gu4AH85/nd4DX5jkhnr/fBl6L/3w/zRVvzX++3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFMuzQffOqdQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPostOffice;
impl IconShape for MdLocalPostOffice {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH+/3+a/12vzb4f4/w3x/xviRfdg4K2A48Br8z/TbwO7wM8At/IvQ/zLjgNfBbw3/7t8N/AxwC4vGOKFe2ngt4Dj/O+0C7wO8Nc8f4gX7DjwdOA4/7vtAg8BdnleiBfsu4H34v+G7wHem+eFeP4eDDyd/z2eAdwKHAdeiufvIcCtPCfE8/fRwFfxv8PfAK8N7HLFZwOfxfP6GOCreU6I5++rgY/if4eXAf6a5/TXwEvxnD4H+GyeE+L5+23gtfjfQTyv3wZei+f0O8Br85wQz99vA6/F/w7vA3w3z3YceDpwnOf0O8Br85wQz99vA6/F/w67wEcD3wO8NPBdwEvzvH4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXot/u48B/pr/WC8NfBX/dr8DvDbPCfH8/TbwWvzbvQ7w2/zHem3gt/i3+x3gtXlOiOfvt4HX4t/udYDf5j/WawO/xb/d7wCvzXNCPH+/DbwW/3avA/w2/7FeG/gt/u1+B3htnhPi+ftt4LX4t3sd4Lf5j/XawG/xb/c7wGvznBDP328Dr8W/3esAv81/rNcGfot/u98BXpvnhHj+fht4Lf7tXgf4bf5jvTbwW/zb/Q7w2jwnxPP328Br8W/3OsBv8x/rtYHf4t/ud4DX5jkhnr+XBo7zovktntfrAL/Nf6zXBn6L5/U6vGh2gb/mOSH+/czzeh3gt/mP9drAb/G8xL8d4t/PPK/XAX6b/1ivDfwWz0v82yH+/czzeh3gt/mP9drAb/G8xL8d4t/PPK/XAX6b/1ivDfwWz0v82yH+/czz+mjgr/mP9dLAV/O8xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMUNnZBng0GSQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalPrintshop;
impl IconShape for MdLocalPrintshop {
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
                d: "M19 8H5c-1.66 0-3 1.34-3 3v6h4v4h12v-4h4v-6c0-1.66-1.34-3-3-3zm-3 11H8v-5h8v5zm3-7c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-1-9H6v4h12V3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalSee;
impl IconShape for MdLocalSee {
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
                cx: "12",
                cy: "12",
                r: "3.2",
            }
            path {
                d: "M9 2L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2h-3.17L15 2H9zm3 15c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBXsw8CD+Z/obYJd/P8TzOg58F/DW/M/1OsBv8++HeE7HgacDx/mf7XWA3+bfD/Gcfgp4a/7nex3gt/n3Qzzbg4Gn87/D6wC/zb8f4tleG/gt/nd4HeC3+fdDPNtrA7/F/w6vA/w2/36IZ3tt4Lf43+F1gN/m3w/xbK8N/Bb/O7wO8Nv8+yGe7bWB3+J/h9cBfpt/P8SzvTbwW/zv8DrAb/Pvh3i21wZ+i/8d/hrY5QX7HuC7+Zchnu21gd/i/46XAf6aFw7xbK8N/Bb/d9wKvAywywuGeLbXBn6L/1t+GngbXjDEs7028Fv87/AxwF/z/H018FI828cAX83zh3i21wZ+i/8dXgf4bZ6/lwZ+GzjGs70M8Nc8L8SzvTbwW/zv8DrAb/OCvTXwUzzbrcDLALs8J8SzvTbwW/zv8DrAb/PCfTbwWTzbTwNvw3NCPNtrA7/F/w6vA/w2/7LfBl6LZ/sY4Kt5NsSzvTbwW/zv8DrAb/MvOw78NfAgnu1lgL/mCsSzvTbwW/zv8DrAb/OieWngt4FjXHEr8DLALoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwWL7rvAf6aK14aeC/+a30P8Ndc8dLAe/Giex3gtwHEs7028Fu8aF4H+G2e02sDv8V/jdcBfpvn9NrAb/GieR3gtwHEs7028Fv8yz4H+Gyev88GPov/XJ8DfDbP32cDn8W/7HWA3wYQz/bawG/xL3sZ4K95/l4a+Cv+c70M8Nc8fy8N/BX/stcBfhtAPNtx4KV5Xr/FcxIvnHlOr8O/z2/xnMQLZ57T6/C8/hrYBRD/sluBB/FsrwP8Ns/fawO/xbM9A3gw/z63Ag/i2V4H+G2ev9cGfotnewbwYF4wxL/sp4G34tl+G3gdnr/fAl6bZ/se4L359/lp4K14tt8GXofn77eA1+bZvgd4b14wxL/svYHv4jl9N/AxwC5XHAe+CnhvntP7AN/Nv897A9/Fc/pu4GOAXa44DnwV8N48p/cBvpsXDPGi+WvgpXhev80Vr83z+h3gtfmP8dfAS/G8fpsrXpvn9TvAa/PCIV40Lw38NnCMF80l4KWBW/mP8dLAbwPHeNFcAl4auJUXDvGie2ngp4EH8cI9A3hr4K/5j/XSwE8DD+KFewbw1sBf8y9D/OscBz4aeG/gQTynZwDfDXw1sMt/juPARwPvDTyI5/QM4LuBrwZ2edEg/u1eGjjOFbvAX/Nf66WB41yxC/w1/3qI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/w+6dBaCEY1QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalShipping;
impl IconShape for MdLocalShipping {
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
                d: "M20 8h-3V4H3c-1.1 0-2 .9-2 2v11h2c0 1.66 1.34 3 3 3s3-1.34 3-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3h2v-5l-3-4zM6 18.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm13.5-9l1.96 2.5H17V9.5h2.5zm-1.5 9c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H8+zwYeC/gtXnBXpt/n9/mBftt4HuAW/m3QfzbvTfwXfzP8D7Ad/Ovh/i3eWngr/if5SHArfzrIP5tvht4L/5n+R7gvfnXQfzbXASO82wfA/w1/7VeGvgqnm0XOMG/DuJf762Bn+LZLgHH+e+xCxzj2d4G+GledIh/ve8G3otn+x7gvfnv8d3Ae/Fs3wO8Ny86xL/OceAiz+ltgJ/mv8dbAz/FczoB7PKiQfzrvDfwXTzbJeA4/712gWM82/sA382LBvGv89PAW/Fs3wO8N/+9vht4L57tZ4C35kWDeNEdBy7ynN4G+Gn+e7018FM8pxPALv8yxIvuvYHv4tkuAcf5n2EXOMazvQ/w3fzLEC+63wJem2f7HuC9+Z/hu4H34tl+Bnhr/mWIF82DgafznN4G+Gn+Z3hr4Kd4Tg8BbuWFQ7xoPhr4Kp7tEnCc/1l2gWM828cAX80Lh3jR/BXw0jzb9wDvzf8s3w28F8/218DL8MIh/mUPBp7Oc7oVuJX/WR4MPJjn9BDgVl4wxL/so4Gv4n+njwG+mhcM8S/7K+Cl+d/pr4GX4QVD/MvM/27iBUP8y8z/buIFQ/zLzAv3N8Au8GDgQfzXeAZwK3AceCleOPGCIf5l5vn7G+C9gb/m2T4a+GzgGP85LgGfDXw1z/bSwHcDL8XzJ14wxL/MPK9LwEsDt/K8Phr4Kv5zfAzw1TyvBwN/DRzjeYkXDPEvM8/ra4CP5gXbBY7xH+sZwIN5wb4a+Ciel3jBEP8y87zeB/huXrDfBl6L/1i/A7w2L9hHA1/F8xIvGOJfZp7X5wCfzQv2V8BL8x/rr4GX4QX7bOCzeF7iBUP8y8zz+mvgZXj+Xhr4K/5zvAzw1zx/TwcezPMSLxjiX2aev+8G3ofn9GDgp4CX5j/HXwNvA9zKc/ou4L15/sQLhviXmRfsVuC7gV3gwcB7A8f5z7ULfDdwK3AceG/gwbxg4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8wL9zv893otXjjxgiH+ZeaFE/+9zAsnXjDEv8y8cOK/l3nhxAuG+JeZF0789zIvnHjBEP+y7wbei+fve4D35r/XdwPvxfP3PcB784Ih/mXHga8G3ovn9D3ARwO7/Pc6Dnw18F48p+8BPhrY5QVD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BtYGVQbmTEIUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocalTaxi;
impl IconShape for MdLocalTaxi {
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
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5H15V3H9v2H6.5c-.66 0-1.21.42-1.42 1.01L3 12v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDZ/NvtAl8NfA2wy388xH+OjwI+GzjOf4xd4GOA7+Y/FuI/3ncB781/ju8G3of/OIj/OMeB3wJemv9cfw28DrDLvx/iP85fAS/Nf42/Bl6Gfz/Ef4zvAt6b/1rfDbwP/z6If7+PBr6K/x7vA3w3/3aIf5/jwNOB4/z32AUeAuzyb4P49/ls4LP47/U5wGfzb4P4t3sw8HT+++0CLwPcyr8e4t/uo4Gv4n+GjwG+mn89xL/dbwOvxb/eJeC7ge8G/porXhp4b+C9gWP86/0O8Nr86yH+bY4DF/nX+x3grYFdnr/jwG8DL8W/3glgl38dxL/NawO/xb/O3wCvDezywh0Hfht4Kf51Xgf4bf51EP82nw18Fv86DwFu5UXz0sBf8a/zMcBX86+D+Lf5bOCzeNH9DPDW/Ov8NPBWvOg+B/hs/nUQ/zbfDbwXL7rPAT6bf53PBj6LF93XAB/Nvw7i3+a7gffiRfc2wE/zr/PWwE/xovse4L3510H823w28Fm86D4H+Gz+dT4b+CxedJ8DfDb/Ooh/m88GPosX3fcA782/zncD78WL7nOAz+ZfB/Fv89HAV/Gi2wUeAuzyojkOPB04zovufYDv5l8H8W/z0sBf8a/zNcBH86L5LuC9+dd5GeCv+ddB/NuZf72PBr6GF+6jgK/mX+cScJx/PcS/3W8Dr8W/3k8DnwP8Nc/ppYHPAt6af73fAV6bfz3Ev917A9/Fv92twK1c8WDgwfzbvQ/w3fzrIf7tjgMX+Z/hBLDLvx7i3+e7gffiv9f3AO/Nvw3i3+e1gd/iv9fLAH/Nvw3i3++7gffiv8f3AO/Nvx3i3+/BwNP57/EQ4Fb+7RD/MT4b+Cz+a30O8Nn8+yD+4/w18FL81/gb4KX590P8x3lp4K/4z3cJeG3gr/n3Q/zH+mjgq/jP9T7Ad/MfA/Ef76eBt+I/x/cA781/HMR/vOPAXwMP4j/W3wCvDezyHwfxn+Olgd8GjvEf4xLw2sBf8x8L8Z/nvYHv4j/G+wDfzX88xH+u7wbei3+f7wHem/8ciP9cx4G/Bh7Ev80zgJcGdvnPgfjP99rAb/Fv8zrAb/OfB/Ff46uBj+Jf52uAj+Y/F+K/xnHgVuAYL5pLwIOBXf5zIf7rfDTwVbxo3gf4bv7zIf5r3Qo8iBfuGcCD+a+B+K/13sB38cK9D/Dd/NdA/Ne7FXgQz98zgAfzXwfxX++jga/i+Xsf4Lv5r4P4r3ccuBU4xnO6BDwY2OW/DuK/x3cD78Vz+hrgo/mvhfjvcRy4FTjGFZeABwO7/NdC/Pd5MPDRXPHVwK3810P8/4b4/w3x/xvi/zfE/2/8I1AAn0EuIbmKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationPin;
impl IconShape for MdLocationPin {
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
                d: "M12,2L12,2C8.13,2,5,5.13,5,9c0,1.74,0.5,3.37,1.41,4.84c0.95,1.54,2.2,2.86,3.16,4.4c0.47,0.75,0.81,1.45,1.17,2.26 C11,21.05,11.21,22,12,22h0c0.79,0,1-0.95,1.25-1.5c0.37-0.81,0.7-1.51,1.17-2.26c0.96-1.53,2.21-2.85,3.16-4.4 C18.5,12.37,19,10.74,19,9C19,5.13,15.87,2,12,2z M12,11.75c-1.38,0-2.5-1.12-2.5-2.5s1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 S13.38,11.75,12,11.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9Zx4KV44f4G2OW/BuI/x4OBlwJeGnht4MHAg/nXuRW4Ffht4K+BvwFu5T8W4j/OSwPvBbw28NL857gV+GngZ4Df5t8P8e9zHPgo4L2BB/Nf61bgu4HvAW7l3wbxb/Ng4LOA9+Z/hu8GPge4lX8dxL/eZwEfDRznf57PBj6HFx3iRffSwHcBL83/bLcCbwP8Nf8yxIvmrYHvAo7zv8Mu8DHAd/PCIf5l7w18F/87vQ/w3bxgiBfupYG/4n+31wF+m+cP8YIdB54OHOd/t13gIcAuzwvxgn028Fn83/A1wEfzvBDP33Hg6cBx/u84AezynBDP31sDP8X/Le8DfDfPCfH8fTbwWfzf8jXAR/OcEM/fTwNvxf8tvwO8Ns8J8fy9N/De/N/y18BH85wQ/78h/n9D/P+G+Nd7LZ7TM4Bb+dc5DrwUz+lvgF3+dR4MPIjn9Du86BAvmtcGPgp4a56/vwa+G/geYJcX7L2AjwZemufvp4GfBr6HF+w48FHAewMP5nntAj8NfA/w27xwiBfuwcBXAW/Ni+ZW4LOB7+E5vTbwVcBL86L5beBzgN/mOb0X8NXAcV403w18DfDXPH+I5++rgdcCXpp/m1uBW7niwcCD+bf5a2CXK14aOM6/za3A9wCfzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBbP63eArwZ+miseDLw38NHAMV503wN8NfDXXPHSwEcD78WL7hLw1cB3A7dyxVsDHw28Fs/rd4DX5jkhnr/fBl6LZ3sG8NnAd/P8HQe+GngvXrjfAT4b+G2ev5cGvhp4LV647wE+G7iV5++tgc8GXopn+x3gtXlOiOfvs7liF/ht4K950RwHXht4aeA4cBy4FbgV+G3gVl40DwZeG3gw8GBgF9gF/hr4bWCXF81LA68NHOeKz+Y5If5/Q/z/hvj/DfH8vTRwjP9bLgF/zXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP33cB78X/L1wAfzXNCPH8fDXwV/7d8DPDVPCfE83ccuBU4xv8Nl4AHA7s8J8QL9t7Ad/F/w9sAP83zQrxwrw18N/Ag/nd6BvDewG/z/CFeNC8NHOd/l13gr3nhEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAVG0kEHMFBxZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLunchDining;
impl IconShape for MdLunchDining {
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
                d: "M22,10c0.32-3.28-4.28-6-9.99-6C6.3,4,1.7,6.72,2.02,10H22z",
                fill_rule: "evenodd",
            }
            path {
                d: "M5.35,13.5c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.64,2.18,0.64 s1.73-0.37,2.18-0.64c0.37-0.23,0.59-0.36,1.15-0.36c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.64,2.18,0.64 c1.11,0,1.73-0.37,2.18-0.64c0.37-0.23,0.59-0.36,1.15-0.36c0.55,0,0.78,0.14,1.15,0.36c0.45,0.27,1.07,0.63,2.17,0.64v-1.98 c0,0-0.79-0.16-1.16-0.38c-0.45-0.27-1.07-0.64-2.18-0.64c-1.11,0-1.73,0.37-2.18,0.64c-0.37,0.23-0.6,0.36-1.15,0.36 s-0.78-0.14-1.15-0.36c-0.45-0.27-1.07-0.64-2.18-0.64s-1.73,0.37-2.18,0.64c-0.37,0.23-0.59,0.36-1.15,0.36 c-0.55,0-0.78-0.14-1.15-0.36c-0.45-0.27-1.07-0.64-2.18-0.64c-1.11,0-1.73,0.37-2.18,0.64C2.78,12.37,2.56,12.5,2,12.5v2 c1.11,0,1.73-0.37,2.21-0.64C4.58,13.63,4.8,13.5,5.35,13.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M2,16v2c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-2H2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/Fg4K2A9+aK7wZ+BriV/3yvDbwX8N7AdwPvwwuG+I/zYOCtgPcGXprn76+B7wa+B9jlP86DgbcCPhp4MM9JvGCIf58HA28FvDfw0vzr/DTw08DPALv827wV8N7AW/OCiRcM8a/3YOCtgPcGXpr/GD8NfDfwM/zLHgx8FPDWwIP5l4kXDPGieTDwVsB7Ay/Nf55d4KeBnwZ+hmc7DrwV8N7Aa/OvI14wxL/sq4GP4t/ua4C3Bh7Ev84u8N3AceCtgeP824gXDPEvM/8+4oqXBt4beGvgQfzXES8Y4l9m/n3E83pr4K2BtwaO8Z9LvGCIf5n59xEv3FsD7w28Ff85xAuG+JeZfx/xojkOvDXw1sBb8R9HvGCIf5n59xH/eseB9wbeG3gp/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX8f8e9j/n3EC4b4l5l/H/HvY/59xAuG+JeZfx/x72P+fcQLhviXmX+fvwa+G/gZ4Fb+9cy/j3jBEP8y8x/np4GfBn4G2OVFY/59xAuG+JeZ/xw/DXw38DO8cObfR7xgiH+Z+c+1C/w08NPAz/C8zL+PeMEQ/zLzX+dW4KeB7wH+mivMv494wRD/MvNvcwn4aWAXeG/gGP86twI/DXw0/3afA3w2LxjiX2b+dX4H+G7gu3lObw28NfDWwDH+8/wN8N3ATwO38sIh/mXmX/YM4KeBrwZu5V/21sB7A2/Ff4y/Ab4b+GngVl50iH+ZecF+Bvhu4Kf5tzkOvDXw1sBb8a/zN8B3Az8N3Mq/DeJfZp7TM4CvBn4auJX/OMeB9wbeG3gpnr+/Ab4b+GngVv79EP+y7wbeGvhp4LuB3+Y/34OBtwbemyu+G/hp4Fb+YyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj3aonUG2TbD1AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMap;
impl IconShape for MdMap {
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
                d: "M20.5 3l-.16.03L15 5.1 9 3 3.36 4.9c-.21.07-.36.25-.36.48V20.5c0 .28.22.5.5.5l.16-.03L9 18.9l6 2.1 5.64-1.9c.21-.07.36-.25.36-.48V3.5c0-.28-.22-.5-.5-.5zM15 19l-6-2.11V5l6 2.11V19z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMapsUgc;
impl IconShape for MdMapsUgc {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29L1,23l6.71-1.97 C9.02,21.64,10.46,22,12,22c5.52,0,10-4.48,10-10C22,6.48,17.52,2,12,2z M16,13h-3v3h-2v-3H8v-2h3V8h2v3h3V13z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+V4aeCvgpYHjvGh2gb8Gfgb4a/7zIP5zfRXw0fz7fDXwMfznQPzn+Wrgo/iP8TXAR/MfD/Gf46WBv+I/1kOAW/mPhfjP8dnAZ/Ef63OAz+Y/FuI/x08Db8VzugT8NS+alwaO8Zx+Bnhr/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4V44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/3n+BtjlOR0HXor/PL8DvDbPCfH8/TbwWvzneR3gt3lOrw38Fv95fgd4bZ4T4vn7beC1+M/zOsBv85xeG/gt/vP8DvDaPCfE8/fbwGvxn+d1gN/mOb028Fv85/kd4LV5Tojn77eB1+I/z+sAv81zem3gt/jP8zvAa/OcEM/fbwOvxX+e1wF+m+f02sBv8Z/nd4DX5jkhnr/fBl6L/zyvA/w2z+m1gd/iP8/vAK/Nc0I8f78NvBb/eV4H+G2e02sDv8V/nt8BXpvnhHj+fht4Lf7t/gbY5QX7aOCveU4vDXw1L9hx4KX4t/sd4LV5Tojn77eB1+Lf7nWA3+Y/1msDv8W/3e8Ar81zQjx/vw28Fv92rwP8Nv+xXhv4Lf7tfgd4bZ4T4vn7beC1+Ld7HeC3+Y/12sBv8W/3O8Br85wQz99vA6/Fv93rAL/Nf6zXBn6Lf7vfAV6b54R4/n4beC3+7V4H+G3+Y7028Fv82/0O8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/Ff57XAX6b5/TawG/xn+d3gNfmOSGev98GXov/PK8D/DbP6bWB3+I/z+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4HeG2eE+L5+23gtfjP8zrAb/OcXhv4Lf7z/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/Pw28Ff+3/Azw1jwnxPP32cBn8X/L5wCfzXNCPH8vDfwV/7c8BLiV54R4wb4a+Cj+b/ga4KN5XogX7quBj+J/t68BPprnD/Eve2ngrYGXBo7zv8Mu8NfAdwO38oIh/n9D/P+G+P8N8f8b4v83/hHjHa1BSRtqLgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMedicalServices;
impl IconShape for MdMedicalServices {
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
                d: "M20,6h-4V4c0-1.1-0.9-2-2-2h-4C8.9,2,8,2.9,8,4v2H4C2.9,6,2,6.9,2,8v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M10,4h4v2h-4V4z M16,15h-3v3h-2v-3H8v-2h3v-3h2v3h3V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMenuBook;
impl IconShape for MdMenuBook {
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
                d: "M21,5c-1.11-0.35-2.33-0.5-3.5-0.5c-1.95,0-4.05,0.4-5.5,1.5c-1.45-1.1-3.55-1.5-5.5-1.5S2.45,4.9,1,6v14.65 c0,0.25,0.25,0.5,0.5,0.5c0.1,0,0.15-0.05,0.25-0.05C3.1,20.45,5.05,20,6.5,20c1.95,0,4.05,0.4,5.5,1.5c1.35-0.85,3.8-1.5,5.5-1.5 c1.65,0,3.35,0.3,4.75,1.05c0.1,0.05,0.15,0.05,0.25,0.05c0.25,0,0.5-0.25,0.5-0.5V6C22.4,5.55,21.75,5.25,21,5z M21,18.5 c-1.1-0.35-2.3-0.5-3.5-0.5c-1.7,0-4.15,0.65-5.5,1.5V8c1.35-0.85,3.8-1.5,5.5-1.5c1.2,0,2.4,0.15,3.5,0.5V18.5z",
            }
            path {
                d: "M17.5,10.5c0.88,0,1.73,0.09,2.5,0.26V9.24C19.21,9.09,18.36,9,17.5,9c-1.7,0-3.24,0.29-4.5,0.83v1.66 C14.13,10.85,15.7,10.5,17.5,10.5z",
            }
            path {
                d: "M13,12.49v1.66c1.13-0.64,2.7-0.99,4.5-0.99c0.88,0,1.73,0.09,2.5,0.26V11.9c-0.79-0.15-1.64-0.24-2.5-0.24 C15.8,11.66,14.26,11.96,13,12.49z",
            }
            path {
                d: "M17.5,14.33c-1.7,0-3.24,0.29-4.5,0.83v1.66c1.13-0.64,2.7-0.99,4.5-0.99c0.88,0,1.73,0.09,2.5,0.26v-1.52 C19.21,14.41,18.36,14.33,17.5,14.33z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMiscellaneousServices;
impl IconShape for MdMiscellaneousServices {
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
                d: "M14.17,13.71l1.4-2.42c0.09-0.15,0.05-0.34-0.08-0.45l-1.48-1.16c0.03-0.22,0.05-0.45,0.05-0.68s-0.02-0.46-0.05-0.69 l1.48-1.16c0.13-0.11,0.17-0.3,0.08-0.45l-1.4-2.42c-0.09-0.15-0.27-0.21-0.43-0.15L12,4.83c-0.36-0.28-0.75-0.51-1.18-0.69 l-0.26-1.85C10.53,2.13,10.38,2,10.21,2h-2.8C7.24,2,7.09,2.13,7.06,2.3L6.8,4.15C6.38,4.33,5.98,4.56,5.62,4.84l-1.74-0.7 c-0.16-0.06-0.34,0-0.43,0.15l-1.4,2.42C1.96,6.86,2,7.05,2.13,7.16l1.48,1.16C3.58,8.54,3.56,8.77,3.56,9s0.02,0.46,0.05,0.69 l-1.48,1.16C2,10.96,1.96,11.15,2.05,11.3l1.4,2.42c0.09,0.15,0.27,0.21,0.43,0.15l1.74-0.7c0.36,0.28,0.75,0.51,1.18,0.69 l0.26,1.85C7.09,15.87,7.24,16,7.41,16h2.8c0.17,0,0.32-0.13,0.35-0.3l0.26-1.85c0.42-0.18,0.82-0.41,1.18-0.69l1.74,0.7 C13.9,13.92,14.08,13.86,14.17,13.71z M8.81,11c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C10.81,10.1,9.91,11,8.81,11z",
            }
            path {
                d: "M21.92,18.67l-0.96-0.74c0.02-0.14,0.04-0.29,0.04-0.44c0-0.15-0.01-0.3-0.04-0.44l0.95-0.74 c0.08-0.07,0.11-0.19,0.05-0.29l-0.9-1.55c-0.05-0.1-0.17-0.13-0.28-0.1l-1.11,0.45c-0.23-0.18-0.48-0.33-0.76-0.44l-0.17-1.18 C18.73,13.08,18.63,13,18.53,13h-1.79c-0.11,0-0.21,0.08-0.22,0.19l-0.17,1.18c-0.27,0.12-0.53,0.26-0.76,0.44l-1.11-0.45 c-0.1-0.04-0.22,0-0.28,0.1l-0.9,1.55c-0.05,0.1-0.04,0.22,0.05,0.29l0.95,0.74c-0.02,0.14-0.03,0.29-0.03,0.44 c0,0.15,0.01,0.3,0.03,0.44l-0.95,0.74c-0.08,0.07-0.11,0.19-0.05,0.29l0.9,1.55c0.05,0.1,0.17,0.13,0.28,0.1l1.11-0.45 c0.23,0.18,0.48,0.33,0.76,0.44l0.17,1.18c0.02,0.11,0.11,0.19,0.22,0.19h1.79c0.11,0,0.21-0.08,0.22-0.19l0.17-1.18 c0.27-0.12,0.53-0.26,0.75-0.44l1.12,0.45c0.1,0.04,0.22,0,0.28-0.1l0.9-1.55C22.03,18.86,22,18.74,21.92,18.67z M17.63,18.83 c-0.74,0-1.35-0.6-1.35-1.35s0.6-1.35,1.35-1.35s1.35,0.6,1.35,1.35S18.37,18.83,17.63,18.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoney;
impl IconShape for MdMoney {
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
                d: "M5 8h2v8H5zm7 0H9c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zm-1 6h-1v-4h1v4zm7-6h-3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zm-1 6h-1v-4h1v4z",
            }
            path {
                d: "M4 6h16v12H4z",
            }
            path {
                d: "M2 4v16h20V4H2zm2 14V6h16v12H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zN9NfBSvGC7wF8DPwP8Nf92iP+Zfht4LV40Xw18DP82iP+Zfht4LV50XwN8NP96iH+b3wZei3+d3wFemxfNbwOvxb/OQ4Bb+ddB/Nv8NvBa/Ov8DvDavGh+G3gt/nU+B/hs/nUQ/za/DbwW/zq/A7w2L5qvBl6aF+ylgWM8p58B3pp/HcS/zW8Dr8W/zu8Ar81/jN8GXovn9DvAa/Ovg/i3+W3gtfjX+R3gtfmP8dvAa/Gcfgd4bf51EP82Lw0c519nF/hr/mP8NvBaPKffAV6bfx3E/06/DbwWz+l3gNfmXwfxv9NvA6/Fc/od4LX510H86xwH3gp4a+DBwEvzovsd4LX5j/HbwGvxnH4HeG3+dRAvuo8CPhs4zr/N7wCvzX+M3wZei+f0O8Br86+D+JcdB34KeG3+fX4HeG3+/d4a+Cme1+8Ar82/DuKFOw78FvDS/Pv9DvDa/Pu8NPBbwHGe188Ab82/DuKF+yngrfmP8TvAa/Nvdxz4LeCleV7PAF4a2OVfB/GCvTbwW/zH+R3gtfm3+yngrXlel4DXBv6afz3EC/ZXwEvzH+d3gNfm3+azgc/i+Xsf4Lv5t0E8fw8Gns5/rN8BXpt/vbcGforn72uAj+bfDvH8fTTwVfzH+h3gtfnXeWngt4DjPK/fAV6bfx/E8/fZwGfxH+t3gNfmRXcc+C3gpXlezwBeGtjl3wfx/P008Fb8x/od4LV50f0U8NY8r0vAawN/zbO9Fs/2DOBWXjSI5++3gdfiP9bvAK/Ni+azgc/i+Xsf4LuBBwOfBbw3z+uvgc8BfpoXDvH8/TbwWvzH+h3gtfmXvTXwUzx/XwN8NPDewHfxL/tu4GOAXZ4/xPP328Br8R9rF/hr/mUvDRznef0O8NrAewPfxYvup4G34flDPH+/DbwW/3M8A3hp4DjwdP71Pgf4bJ4X4vn7beC1+J/hEvDawF8D3w28F/96u8BDgF2eE+L5+23gtfif4X2A7waOAxd5XpeAjwa+G3hp4LuBl+J5vQ3w0zwnxPP328Br8d/va4CP5orXBn6L5/U+wHfzbMeBW4FjPKfPAT6b54R4/r4aeGme03HgpXhev8PzemngGM/pEvDXvOhuBd6bZ3tt4Ld4XuJ5/TbwWjynnwHemueEeNG9NvBbPK+XAf6a5/RXwEvznD4H+Gz+7V4b+C2e18sAf81z+ivgpXlOnwN8Ns8J8aJ7MPB0ntdfA68D7HLFZwGfzfP6GOCr+bd7MPB0ntdfA68D7HLFZwGfzfP6GOCreU6If52/Bl6K57UL/DVwHHhpnr+HALfy7/PXwEvxvHaBvwaOAy/N8/cQ4FaeE+Jf562Bn+Jf73uA9+bf762Bn+Jf73uA9+Z5If71vht4L150l4AHA7v8x/hu4L140V0CHgzs8rwQ/3rHgZ8GXot/2SXgtYG/5j/OceCngdfiX3YJeG3gr3n+EP92nw18NHCM5+97gI8GdvnP8dnARwPHeP6+B/hoYJcXDPHvcxx4beClgZcG/hrYBX4auJX/fMeB1wZeGnhp4K+BXeCngVv5lyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFpa8FBH5ABmwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoped;
impl IconShape for MdMoped {
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
                d: "M19,7c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,14H10V9H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35V7 z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
            }
            rect {
                height: "2",
                width: "5",
                x: "5",
                y: "6",
            }
            path {
                d: "M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,17,19,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6fjwC7/foj/nX4a+Gzgr/n3Qfzv9NvASwGvA/w1/3aI/51+G3gtYBd4HeCv+bdB/O/028BrccUu8DrAX/Ovh/jf6beB1+LZdoHXAf6afx3E/06/DbwWz2kXeB3gr3nRIf53+m3gtXheu8DrAH/Niwbx7/NaXPE7vHAPBh4EXAL+mufvOPBSvGi+Gnhpnr9d4HWAv+Zfhvi3OQ78FvDSXPHXwOsAuzyvlwZ+CzjOFZ8DfDbP67WB3+I/xi7wOsBf88Ih/m0+GvgqntPHAF/N8/or4KV5Ti8D/DXP6bWB3+I/zi7wOsBf84Ih/m2+G3gvntPXAB/N8zLP63WA3+Y5vTbwW/zH+h3gtXnBEP82bw38FM/pbYCf5nl9N/BePNsl4MHALs/ptYHf4j/O3wCvDezygiH+7T4beG+u+G7gs3n+jgNfDbwX8DfAewN/zfN6beC3+I/xN8BrA7u8cIj/nX4beC2ev78BXhvY5V+G+N/pt4HX4nn9DfDawC4vGsT/Tr8NvBbP6W+A1wZ2edEh/nf6beC1eLa/AV4b2OVfB/G/028Dr8UVfwO8NrDLvx7if6ffBl4L+BvgtYFd/m0Q/zv9NnAceG1gl387xP9OXw18NrDLvw/iP8Zx4LOBj+Z/F8S/33Hgt4BLwGvzvwvi3+c48FvASwO/A7w2/7sg/u2OA78FvDRX/A7w2vzvgvi3OQ78FvDSPNvvAK/N/y6If73jwG8BL81z+h3gtfnfBfGvcxz4LeCleV6/A7w2/7sgXnTHgd8CXprn73eA1+Z/F8SL5jjwW8BL8x/jdYDf5nm9NPBdwEsD3w18DLDL8/dZwEcDu8B3A5/Dvx7iX3Yc+C3gpfmP8zrAb/OcHgz8FXCcZ/se4L15Xm8N/BTP6W2An+ZfB/Ev+23gtfiP9TrAb/OcXhv4LZ6XeF7fDbwXz+lrgI/mXwfxL3tp4LeBY/zHeR3gt3lOLw38Fc/pb4CX5nl9NPBVPKePAb6afx3Ei+algd8GjvEf43WA3+Z5fTbwWVxxCXht4K95XseB3wZeiiv+BnhtYJd/HcSL7qWB3waO8fz9DfDRvGj+Gtjl+Xtp4Djw18AuL9xrc8Vv82+D+Nd5aeC3gWM8r98BXpv/XRD/ei8N/DZwjOf0O8Br878L4t/mpYHfBo7xbL8DvDb/uyD+7V4a+G3gGFf8DvDa/O+C+Pd5aeC3gWPA7wCvzf8uiH+/lwZ+G/hr4LX53wXxH+Olgc8G3pr/XRD/cY4Du/zvgvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CM02mkFgweVoAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMultipleStop;
impl IconShape for MdMultipleStop {
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
                d: "M17,4l4,4l-4,4V9h-4V7h4V4z M10,7C9.45,7,9,7.45,9,8s0.45,1,1,1s1-0.45,1-1S10.55,7,10,7z M6,7C5.45,7,5,7.45,5,8 s0.45,1,1,1s1-0.45,1-1S6.55,7,6,7z M7,17h4v-2H7v-3l-4,4l4,4V17z M14,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1 C13,16.55,13.45,17,14,17z M18,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1C17,16.55,17.45,17,18,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMuseum;
impl IconShape for MdMuseum {
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
                d: "M22,11V9L12,2L2,9v2h2v9H2v2h20v-2h-2v-9H22z M16,18h-2v-4l-2,3l-2-3v4H8v-7h2l2,3l2-3h2V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx3hOl4C/5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X4r3EceCngtYGXBl4bOM5z2gV+G/hr4LeBvwF2+c+F+M/1XsB7A6/Nv81vA98NfA//ORD/8Y4DnwW8N3Cc/xi7wFcDXwPs8h8H8R/rvYCvBo7zn2MX+Gjge/iPgfiP8dLAVwGvzX+N3wbeB7iVfx/Ev99LA78FHOdfdgn4a+C3ueJWrngwV7w28NLAMf5lu8DrAH/Nvx3i3+e9ge/ihXsG8NPATwO/zYvmtYG3Bt4aeBAv3PsA382/DeLf7r2B7+IFuwR8NfDZ/NsdBz4a+GjgGC/Y+wDfzb8e4t/mpYG/4gX7GuCzgV3+YxwHvhp4L16wlwH+mn8dxL/eSwO/BRzn+Xsf4Lv5z/HewHfx/O0CLwPcyosO8a/3W8Br87wuAa8N/DX/uV4a+G3gGM/rt4HX4UWH+Nd5b+C7eP5eBvhr/mu8NPBXPH/vA3w3LxrEi+448HTgOM/rfYDv5r/WewPfxfPaBR4C7PIvQ7zovhr4KJ7X1wAfzYvuOPBWwIOBB3PFrcCtwM8Au7zovht4L57X5wCfzb8M8aK7CBznOV0CHgzs8i97MPBZwHvzwn038DnArfzLjgO3Asd4TrvACf5liBfNewPfxfP6HOCz+Ze9N/Bd/Ou8D/Dd/Ms+G/gsntf7AN/NC4d40fwW8No8p2cAD+Zf9l3Ae/Nv893A+/DCHQf+GngQz+lngLfmhUP8y44DF3leXwN8NC/cewPfxb/P+wDfzQv31cBH8bxOALu8YIh/2WsDv8Xzeh3gt3nBHgw8nf8YDwFu5QV7beC3eF6vA/w2LxjiX/bZwGfxnC4Bx3nhvht4L/5jfA/w3rxwu8AxntPnAJ/NC4b4l/008FY8p98BXpsX7Dhwkf9YJ4BdXrDfBl6L5/QzwFvzgiGe7aWBr+J5vTRwnOf0OcBn84K9N/Bd/Md6G+CnecG+GvgontMu8Nc8r48B/hpAPNtrA7/Fi+ZzgM/mBfts4LP4j/U5wGfzgn028Fm8aF4H+G0A8WyvDfwWL5r3Ab6bF+y7gffiP9bXAB/NC/bRwFfxonkd4LcBxLO9NvBbvGjeB/huXrDvBt6L/1hfA3w0L9hHA1/Fi+Z1gN8GEM/22sBv8aL5HOCzecE+G/gs/mN9DvDZvGCfDXwWL5rXAX4bQDzbSwNfzfN6aeAYz+lzgM/mBXtv4Lv4j/U2wE/zgn018FE8p0vAX/O8Phr4awDxL/tp4K14Tr8DvDYv2HHgIv+xTgC7vGC/DbwWz+lngLfmBUP8yz4b+Cye0y5wghfuu4H34j/G9wDvzQt3ETjOc/oc4LN5wRD/stcGfovn9TrAb/OCPRh4Ov8xHgLcygv21sBP8bxeB/htXjDEv+w4cJHn9TXAR/PCvTfwXfz7vA/w3bxwXw18FM/rBLDLC4Z40fw28Fo8p1uBlwF2eeG+G3gv/m2+B3hvXrjjwF8BD+Y5/Qzw1rxwiBfNewPfxfP6HOCz+Ze9N/Bd/Ou8D/Dd/Ms+G/gsntf7AN/NC4d40e0Cx3hOu8BDgF3+ZQ8GPht4L1647wE+G7iVf9lx4OnAcZ7TJeA4/zLEi+6rgY/ieX0N8NG86I4Dbw08GHgwV9wK/DXw28AuL7rvAt6b5/U5wGfzL0O86I4DtwLHeF7vA3w3/7XeG/guntcl4MHALv8yxL/OewPfxfP3MsBf81/jpYG/4vl7H+C7edEg/vV+G3gtntcu8DrAX/Of66WB3wKO87x+B3htXnSIf70HA38NHOP5ex/gu/nP8d7Ad/H8XQJeGriVFx3i3+algb/iBftu4GOAXf5jHAe+CnhvXrCXAf6afx3Ev917A9/FC7YLfDXwNcAu/zbHgY8CPho4zgv2PsB386+H+Pd5b+C7eOFuBX4a+Gngd3jRvBbw1sBbAw/mhXsf4Lv5t0H8+7008NvAMf5lu8BfA38N7AK3csWDgePASwMvDRznX3YJeG3gr/m3Q/zHeDDw3cBr8V/jd4D3Bm7l3wfxH+u9ga8GjvGf4xLw0cB38x8D8R/vOPDRwEcDx/iPcQn4auCrgV3+4yD+c7038N7Aa/Fv8zPATwPfzX8OxH+N48BLA68NvDTw2sAxntMl4LeBvwZ+G/hrYJf/XIj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HSwPHeU67wF/zXwvx/xvi/zf+EQFrJVBXwSZ5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMyLocation;
impl IconShape for MdMyLocation {
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
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vY5zxS7/PRD/vT6bKz6b/x6I/15P54qH8N8D8d/nrYGf4oq3AX6a/3qI/z4/DbwVV/wM8Nb810P893gw8HSe00OAW/mvhfjv8dXAR/GcPgf4bP5rIf57XASO85x2gRP810L813tv4Lt4/t4H+G7+6yD+6/0W8No8f78NvA7/dRD/tR4MPJ0X7iHArfzXQPzX+m7gvXjhvgd4b/5rIP7rHAeeDhznhdsFHgLs8p8P8V/nvYHv4kXzPsB3858P8V/nr4CX5kXz18DL8J8P8V/jtYHf4l/nZYC/5j8X4r/GdwPvxb/O9wDvzX8uxH++48BF/m1OALv850H85/to4Kv4t/kY4Kv5z4P4z/d04MH829wKPIT/PIj/XK8N/Bb/Pq8D/Db/ORD/uX4aeCv+fX4GeGv+cyD+8zwYeDr/MR4C3Mp/PMR/ns8GPov/GJ8DfDb/8RD/eZ4OPJj/GLcCD+E/HuI/x1sDP8V/rLcBfpr/WIj/HL8FvDb/sX4beB3+YyH+4z0YeDr/OR4C3Mp/HMR/vK8GPor/HF8DfDT/cRD/8S4Cx/nPsQuc4D8O4j/WewPfxX+u9wG+m/8YiP9YfwW8NP+5fht4Hf5jIP7jvDTwV/zXeBngr/n3Q/zH+W7gvfiv8T3Ae/Pvh/iPcRx4OnCc/xq7wEOAXf59EP8x3hv4Lv5rvQ/w3fz7IP5jPB14MP+1bgUewr8P4t/vtYHf4r/H6wC/zb8d4t/vu4H34r/H9wDvzb8d4t/nOHCR/14ngF3+bRD/Pp8NfBb/vT4G+Gr+bRD/Pk8HHsx/r1uBh/Bvg/i3e2vgp/if4W2An+ZfD/Fv99PAW/Fv9z3Ad3PFewPvxb/dzwBvzb8e4t/mwcDT+de7BHw38NXArTynBwMfDbw3cIx/vYcAt/Kvg/i3+Wzgs3jRPQP4bOCngV1euOPAewMfDTyIF93nAJ/Nvw7i3+YicJx/2e8AXw38NP82bw18NPBa/Mt2gRP86yD+9d4b+C5euO8Bvhr4a/5jvDTw0cB78cK9D/DdvOgQ/3q/Bbw2z+sS8NXAdwO38p/jwcB7Ax8NHON5/TbwOrzoEP963w28F8/2DOCzgZ8GdvmvcRx4a+CzgQfxbN8DvDcvOsS/zXcDDwY+G/ht/nu9NfDRwK3Ae/Ovg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AjvOf0EPe7jrAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNavigation;
impl IconShape for MdNavigation {
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
                d: "M12 2L4.5 20.29l.71.71L12 18l6.79 3 .71-.71z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4v+u9gNcB3psXDPF/y4OBjwLeGzgOvA/w3bxgiP8b3gp4b+CtebZLwHFeOMT/XseB9wI+Gngwz+tzgM/mhUP87/PSwEcBbw0c5wV7CHArLxzif4/3At4beG3+ZT8DvDX/MsT/bA8G3gv4aOA4L7rXAX6bfxnif6bXBj4KeGv+9Z4BPJgXDeJ/juPAWwGfDTyYf7uPAb6aFw3iv9+Dgc8C3ho4zr/PJeDBwC4vGsR/n/cC3ht4bf7jfA/w3rzoEP+1Hgy8F/DewIP5j/cywF/zokP813ht4L2A9+Y/z+8Ar82/DuI/z3HgrYDPBh7Mf773Ab6bfx3Ef7wHAx8FvDdwnP8azwAezL8e4j/OWwEfDbw2//U+B/hs/vUQ/z7HgY8C3ht4MP99HgLcyr8e4t/mtYH3At6b/37fA7w3/zaIF91x4K2AjwZemv85Xgf4bf5tEP+yBwMfBbw3cJz/WZ4BPJh/O8S/zPzP9T7Ad/Nvh/iXmf+ZLgEPBnb5t0P8y24FHsT/PF8DfDT/PogXzVsDbw28F/9zPAS4lX8fxL/OceC9gfcGXor/Pr8DvDb/foh/u5cGPhp4a+AY/7XeBvhp/v0Q/37HgbcG3ht4Lf7zPQN4MP8xEP+xHgx8NPDWwIP4z/E5wGfzHwPxn+etgfcG3or/WCeAXf5jIP7zPRh4a+CzgWP8+3wP8N78x0H81/lo4Kv493kd4Lf5j4P4r/Ng4On82/0N8NL8x0L81/pp4K34t3kf4Lv5j4X4r/XWwE/xr3cJOM5/PMR/vV3gGP86XwN8NP/xEP/1vhr4KP51HgLcyn88xH+9BwNP50X3M8Bb858D8d/jr4GX4kXzNsBP858D8d/jvYHv4l/2DODB/OdB/Pc4DtwKHOOF+xjgq/nPg/jv893Ae/GCXQIeDOzynwfx3+e1gd/iBfse4L35z4X473Ur8CCev5cB/pr/XIj/Xh8NfBXP63eA1+Y/H+K/13HgIs/rfYDv5j8f4r/fTwNvxbNdAo7zXwPx3++tgZ/i2T4H+Gz+ayD+Z7gVeBBXPAS4lf8aiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjbM9iQTYBKAgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNearMe;
impl IconShape for MdNearMe {
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
                d: "M21 3L3 10.53v.98l6.84 2.65L12.48 21h.98L21 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNearMeDisabled;
impl IconShape for MdNearMeDisabled {
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
                d: "M12,6.34L21,3l-3.34,9L12,6.34z M22.61,19.78L4.22,1.39L2.81,2.81l5.07,5.07L3,9.69v1.41l7.07,2.83L12.9,21h1.41l1.81-4.88 l5.07,5.07L22.61,19.78z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz/Zg4Lv4n+UZwHvzvL4aeCn+df4G+GieE+I5fTXwUfzP8TrAb/O8fht4Lf51fgd4bZ4T4jkdB/4aeBD//b4G+Giev98GXot/nd8BXpvnhHherw38Fv+9ngG8NLDL8/fbwGvxr/M7wGvznBDP31cDH8V/n9cBfpsX7LeB1+Jf53eA1+Y5IZ6/48BfAw/ief0O/zEeDDyI5/U1wEfzwv028Fr86/wO8No8J8QL9trAb/G8vgb4aP59jgN/BTyY5/QM4KWBXV643wZei3+d3wFem+eEeOG+GvgontfrAL/Nv913Ae/N83od4Lf5l/028Fr86/wO8No8J8QLdxz4a+BBPKdbgZcBdvnXe2vgp3heXwN8NC+a3wZei+f0N8BH84LtAn/Nc0L8y14b+C2e108Db8O/znHg6cBxntMzgJcGdnnR/DbwWjyn3wFem38dxIvmq4GP4nm9DfDTvOh+C3htntfrAL/Ni+63gdfiOf0O8Nr86yBeNMeBvwYexH+8rwE+mn+d3wZei+f0O8Br86+DeNG9NvBb/Md6BvDSwC7/Or8NvBbP6XeA1+ZfB/Gv89XAR/Ef53WA3+Zf77eB1+I5/Q7w2vzrIP51jgN/DTyIf7+vAT6af5vfBl6L5/Q7wGvzr4P413tt4Lf493kG8NLALv82vw28Fs/pd4DX5l8H8W/z1cBH8W/3OsBv82/328Br8Zx+B3ht/nUQ/zbHgb8GHsS/3tcAH82/z28Dr8Vz+h3gtfnXQfzbvTbwW/zrPAN4aWCXf5/fBl6L5/Q7wGvzr4P49/lq4KN40b0O8Nv8+/028Fo8p98BXpt/HcS/z3Hgr4EH8S/7GuCj+Y/x28Br8Zx+B3ht/nUQ/36vDfwWL9wzgJcGdvmP8dvAa/Gcfgd4bf51EP8xvhr4KF6w1wF+m/84vw28Fs/pd4DX5l8H8R/jOPDXwIN4Xl8DfDT/sX4beC2e0+8Ar82/DuI/zlsDP8XzOgHs8m9zHHgr4MHASwO3ArcC7w28NM/pd4DX5l8H8R/ntYHf4nmJf5vPAj4aOM6L5neA1+ZfB/Ef57WB3+J5iX+d48BPAa/Nv87vAK/Nvw7iP85rA7/F8xL/Ot8FvDf/er8DvDb/Ooj/OK8N/BbPS7zo3hr4Kf5tfgd4bf51EP9xXhv4LZ6XeNH9FfDS/Nv8DvDa/Osg/uO8NvBbPC/xonkw8HT+7X4HeG3+dRD/cV4b+C2el3jRvDbwW/zb/Q7w2vzrIP7jvDbwWzwv8aJ5beC3+Lf7GeCt+ddB/Md5beC3eF7iRfPawG/xb/c5wGfzr4P4j/PawG/xvMSLzvzbvQ3w0/zrIP5tfht4Lf7tXgf4bZ7XdwPvxb/eJeDBwC7/Ooh/m98GXot/u9cBfpvn9WDg6fzrfQ7w2fzrIf5tfht4Lf7tXgf4bZ6/9wa+ixfdzwBvzb8N4t/mt4HX4t/udYDf5gV7b+C7+Jd9D/DRwC7/Noh/m98GXot/u9cBfpsX7sHAZwPvxfP6G+CzgZ/m3wfxb/PSwHH+7f4a2OVF99o8263ArfzHQPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BCG9tEGC3xEpAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNightlife;
impl IconShape for MdNightlife {
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
                d: "M1,5h14l-6,9v4h2v2H5v-2h2v-4L1,5z M10.1,9l1.4-2H4.49l1.4,2H10.1z M17,5h5v3h-3v9h0c0,1.66-1.34,3-3,3s-3-1.34-3-3 s1.34-3,3-3c0.35,0,0.69,0.06,1,0.17L17,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoMeals;
impl IconShape for MdNoMeals {
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
                d: "M16,14V6c0-1.76,2.24-4,5-4v16.17l-2-2V14H16z M20.49,23.31L10.02,12.85C9.69,12.94,9.36,13,9,13v9H7v-9c-2.21,0-4-1.79-4-4 V5.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M6.17,9L5,7.83V9H6.17z M9,2H7v2.17l2,2V2z M13,9V2h-2v6.17l1.85,1.85 C12.94,9.69,13,9.36,13,9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoMealsOuline;
impl IconShape for MdNoMealsOuline {
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
                d: "M16,14V6c0-1.76,2.24-4,5-4v16.17l-2-2V14H16z M20.49,23.31L10.02,12.85C9.69,12.94,9.36,13,9,13v9H7v-9c-2.21,0-4-1.79-4-4 V5.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M6.17,9L5,7.83V9H6.17z M9,2H7v2.17l2,2V2z M13,9V2h-2v6.17l1.85,1.85 C12.94,9.69,13,9.36,13,9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoTransfer;
impl IconShape for MdNoTransfer {
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
                d: "M21.19,21.19L2.81,2.81L1.39,4.22L4,6.83V16c0,0.88,0.39,1.67,1,2.22V20c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h8v1 c0,0.55,0.45,1,1,1h1c0.05,0,0.09-0.02,0.14-0.03l1.64,1.64L21.19,21.19z M7.5,17C6.67,17,6,16.33,6,15.5C6,14.67,6.67,14,7.5,14 S9,14.67,9,15.5C9,16.33,8.33,17,7.5,17z M6,11V8.83L8.17,11H6z M8.83,6L5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10 c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Nf67fBt4G2OXfD/Ef57eA1+a/xm8Dr8O/H+I/xncB781/re8G3od/H8S/33sD38W/zs8Afw3cyhUPBl4aeCv+dd4H+G7+7RD/PseBpwPH+ZddAr4a+Gpgl+fvOPDRwGfxotkFHgLs8m+D+Pf5bOCz+JddAl4b+GteNC8N/DZwjH/Z5wCfzb8N4t/uwcBfAcd54S4BLw3cyr/Og4G/Bo7xwu0CLwPcyr8e4t/uo4Gv4l/2OsBv82/z2sBv8S/7GOCr+ddD/Nv9NvBavHA/A7w1z+u1gY8C3porfhr4GuC3eV6/DbwWL9zPAG/Nvx7i3+Y4cJF/2esAv81zem/gu3j+3gf4bp7TWwM/xb/sBLDLvw7i3+a1gd/ihbsEHOc5PRh4Oi/cQ4BbeU67wDFeuNcBfpt/HcS/zWcDn8UL9zvAa/OcPhv4LF64zwE+m+f028Br8cJ9DPDV/Osg/m0+G/gsXri/Bj6a5/TZwGvzwn0P8N48p98GXosX7nOAz+ZfB/Fv893Ae/Gf43OAz+Y5/TbwWrxwXwN8NP86iH+b7wbei/8cLwP8Nc/J/Mu+B3hv/nUQ/zafDXwW//HeB/huntN7A9/Fv+xzgM/mXwfxb/PZwGfxH+cS8NHAd/OcjgNPB47zL/sc4LP510H827w28Fv8x7gEvDbw1zyn48BvAS/Ni+Z1gN/mXwfxb3McuMi/3zOAlwZ2eU7Hgd8CXpoX3Qlgl38dxL/drcCD+Le7BLw28Nc8p5cGfgp4MC+6ZwAP5l8P8W/31cBH8W/3NcBH85yOA08HjvOv8zXAR/Ovh/i3e2ngr/i3ewhwK8/pt4DX5l/vZYC/5l8P8e9zK/Ag/vUuAcd5Tq8N/Bb/es8AHsy/DeLf57OBz+Jf73eA1+Y5fTbwWfzrfQ7w2fzbIP59Hgw8nX+9XeCveU4PBh7Mv95DgFv5t0H8+3038F789/ge4L35t0P8+7008Ff893gIcCv/doj/GL8NvBYvur8BPprn9NXAS/Gi+xngrfn3QfzHeG3gt3jR/Q7w2jyn3wZeixfdywB/zb8P4j/OdwPvxYvmd4DX5jn9NvBavGi+B3hv/v0Q/3EeDDydF83vAK/Nc/pt4LV40TwEuJV/P8R/rM8GPot/2e8Ar81z+m3gtfiXfQ7w2fzHQPzHOg78NfAg/nM8A3hpYJf/GIj/eG8N/BT/Od4G+Gn+4yD+c/w08Fb8x/oZ4K35j4X4z/Fg4K+BY/zHuAQ8GNjlPxbiP89HA1/Ff4yPAb6a/3iI/1w/DbwV/z4/A7w1/zkQ/7mOA7cCx/i3uQQ8GNjlPwfiP99bAz/Fv83bAD/Nfx7Ef42vBj6Kf52vAT6a/1yI/zp/DbwUL5q/AV6a/3yI/zoPBv4aOMYLdwl4aeBW/vMh/mu9NfBTvHBvA/w0/zUQ//U+G/gsnr/PAT6b/zqI/x4/DbwVz+l7gPfmvxbiv8dx4LeBl+KKvwFeG9jlvxbiv89x4FaueDCwy389xH+vl+aKv+a/B+L/N8T/b4j/3xD/vyH+f+MfAaeuxEHpeEuIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotListedLocation;
impl IconShape for MdNotListedLocation {
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
                d: "M12 2C8.14 2 5 5.14 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.86-3.14-7-7-7zm.88 13.75h-1.75V14h1.75v1.75zm0-2.87h-1.75c0-2.84 2.62-2.62 2.62-4.38 0-.96-.79-1.75-1.75-1.75s-1.75.79-1.75 1.75H8.5C8.5 6.57 10.07 5 12 5s3.5 1.57 3.5 3.5c0 2.19-2.62 2.41-2.62 4.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vV6aK/6a/x6I/15/xRUvw38PxH+fzwY+iys+B/hs/ush/nu8NPBXPKfXAX6b/1qI/3rHgb8CHsxzuhV4GWCX/zqI/3pfDXwUz993A+/Dfx3Ef623Bn6KF+5tgJ/mvwbiv85x4OnAcV64XeAhwC7/+RD/dX4KeGteNL8NvA7/+RD/Nd4b+C7+dT4G+Gr+cyH+8z0Y+CvgOP86u8DrAH/Nfx7Ef77fAl6bf5u/Bl6G/zyI/1wfDXwV/z6fA3w2/zkQ/3leGvgr/mO8DvDb/MdD/Of5K+Cl+Y9xK/AywC7/sRD/Ob4a+Cj+Y3038D78x0L8x3tt4Lf4z/E2wE/zHwfxH+s48FfAg/nPsQs8BNjlPwbiP9Z3Ae/Nf66fBt6G/xiI/zhvDfwU/zU+Bvhq/v0Q/zGOA08HjvNfYxd4GeBW/n0Q/zF+C3ht/mv9NfAy/Psg/v0+Gvgq/nt8DvDZ/Nsh/n0eDPwVcJz/Pi8D/DX/Noh/n78CXpr/XrcCLwPs8q+H+Lf7bOCz+J/ha4CP5l8P8W/z0sBf8T/L2wA/zb8O4j/OTwFvzX+Nnwbehn8/xH+MtwZ+iv9abwP8NP8+iH+/48DTgeP819oFHgLs8m+H+Pf7auCj+O/x08Db8G+H+Pd5beC3+O/1NsBP82+D+Lc7DvwV8GD+e+0CDwF2+ddD/Nt9NfBR/M/w08Db8K+H+Ld5beC3+J/lbYCf5l8H8W/zV8BL8z/LLvAQYJcXHeJf77OBz+J/pp8G3oYXHeJf56WBv+J/trcBfpoXDeJf56+Al+Z/tl3gIcAu/zLEi+6zgc/if4efBt6GfxniRfNg4K+A4/zv8TbAT/PCIV40vwW8Nv+77AIPAXZ5wRD/so8Gvor/nX4aeBteMMR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/nZcGvpoXzUcDf81/PsT/b4j/3xD/vyH+f0P8/8Y/Apc2dkGWSecaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPark;
impl IconShape for MdPark {
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
            polygon {
                points: "17,12 19,12 12,2 5.05,12 7,12 3.1,18 10.02,18 10.02,22 13.98,22 13.98,18 21,18",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPedalBike;
impl IconShape for MdPedalBike {
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
                d: "M18.18,10l-1.7-4.68C16.19,4.53,15.44,4,14.6,4H12v2h2.6l1.46,4h-4.81l-0.36-1H12V7H7v2h1.75l1.82,5H9.9 c-0.44-2.23-2.31-3.88-4.65-3.99C2.45,9.87,0,12.2,0,15c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99 c2.8,0.13,5.25-2.19,5.25-5c0-2.8-2.2-5-5-5H18.18z M7.82,16c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3 c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,14h-1.4l-0.73-2H15C14.56,12.58,14.24,13.25,14.1,14z M19,18c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,18,19,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonPin;
impl IconShape for MdPersonPin {
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
                d: "M12 2c-4.97 0-9 4.03-9 9 0 4.17 2.84 7.67 6.69 8.69L12 22l2.31-2.31C18.16 18.67 21 15.17 21 11c0-4.97-4.03-9-9-9zm0 2c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm0 14.3c-2.5 0-4.71-1.28-6-3.22.03-1.99 4-3.08 6-3.08 1.99 0 5.97 1.09 6 3.08-1.29 1.94-3.5 3.22-6 3.22z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDRwHH+bXaBrwa+BtjlPx7iP8d7A18FHOc/xi7wMcB38x8L8R/vu4D35j/HdwPvw38cxH+c48BPAa/Ni+YZwK1c8WDgQbxofht4G2CXfz/Ef5zfAl6bf9kl4L2Bn+Y5vTXw3cAx/mW/DbwO/36I/xjfBbw3/7JLwIOBXZ6/48CtwDH+Zd8NvA//Poh/v/cGvosXzdsAP80L99bAT/GieR/gu/m3Q/z7HAeeDhznX/YM4MG8aG4FHsS/bBd4CLDLvw3i3+ezgc/iRfM7wGvzovlt4LV40XwO8Nn82yD+7R4M/BVwnBfN7wCvzYvmt4HX4kWzC7wMcCv/eoh/u48GvooX3a3AQ3jRPB14MC+6jwG+mn89xL/dbwOvxb/O2wA/zQv31sBP8a/zM8Bb86+H+Lc5DlzkX28XeAiwy/N3HHg6cJx/vRPALv86iH+b1wZ+i3+bXeB9gJ/mOb018F3Acf5tXgf4bf51EP82nw18Fv8+twK3csWDgQfz7/MxwFfzr4P4t/ls4LP4n+VzgM/mXwfxb/PdwHvxP8vXAB/Nvw7i3+a7gffif5bvAd6bfx3Ev81nA5/F/yyfA3w2/zqIf5vPBj6Lf72/AX4b+GvgVp7Tg4GXBl4beCn+9T4H+Gz+dRD/Nq8N/BYvmkvAVwPfDdzKi+bBwHsDHw0c40XzOsBv86+D+Lc5DlzkX/Y1wGcDu/zbHAc+G/go/mUngF3+dRD/drcCD+IFex/gu/mP8d7Ad/GCPQN4MP96iH+7rwY+iufve4D35j/WdwPvxfP3NcBH86+H+Ld7aeCveP4+Bvhq/mN9NPBVPH8vA/w1/3qIf59bgQfxvHaBhwC7/Mc4DjwdOM7zegbwYP5tEP8+nw18Fs/fXwOvA+zy73Mc+C3gpXn+Pgf4bP5tEP8+Dwaezgv218D7AH/Nv81LA98FvDQv2EOAW/m3Qfz7fTfwXrxgu8BXA5/Dv85nAR8NHOcF+x7gvfm3Q/z7vTTwV/zLbgW+G/gZ4K95/l4aeCvgo4Hj/MseAtzKvx3iP8ZvA6/Fi24X+Gue00sDx3nR/Qzw1vz7IP5jvDbwW/zXehngr/n3QfzH+W7gvfiv8T3Ae/Pvh/iP82Dg6fzXeAhwK/9+iP9Ynw18Fv+5Pgf4bP5jIP5jHQf+GngQ/zmeAbw0sMt/DMR/vLcGfor/HG8D/DT/cRD/OX4aeCv+Y/0M8Nb8x0L853gw8NfAMf5jXAIeDOzyHwvxn+ejga/iP8bHAF/NfzzEf66fBt6Kf5+fAd6a/xyI/1zHgVuBY/zbXAIeDOzynwPxn++tgZ/i3+ZtgJ/mPw/iv8ZXAx/Fv87XAB/Nfy7Ef52/Bl6KF83fAC/Nfz7Ef50HA38NHOOFuwS8NHAr//kQ/7XeGvgpXri3AX6a/xqI/3qfDXwWz9/nAJ/Nfx3Ef4+fBt6K5/Q9wHvzXwvx3+M48NvAS3HF3wCvDezyXwvx3+c4cCtXPBjY5b8e4r/XS3PFX/PfA/H/G+L/N8T/b4j/3xD/v/GPzY65Qfp7dkUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonPinCircle;
impl IconShape for MdPersonPinCircle {
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
                d: "M12,2C8.14,2,5,5.14,5,9c0,5.25,7,13,7,13s7-7.75,7-13C19,5.14,15.86,2,12,2z M12,4c1.1,0,2,0.9,2,2c0,1.11-0.9,2-2,2 s-2-0.89-2-2C10,4.9,10.9,4,12,4z M12,14c-1.67,0-3.14-0.85-4-2.15c0.02-1.32,2.67-2.05,4-2.05s3.98,0.73,4,2.05 C15.14,13.15,13.67,14,12,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P853kv4Hv4j/FewPfwHw/xn+O7gPcGvht4H/59vgt4b+C7gffhPxbiP953Ae/Ns3038D7823wX8N4823cD78N/HMR/rO8C3pvn9d3A+/CcXovn9Ds8p+8C3pvn9d3A+/AfA/Ef5zjw28BL8fx9N/A9wEcBb83z99PA1wDvBbw3z9/fAK8N7PLvh/iPdRz4beCl+M/xN8BrA7v8x0D8xzsO/DbwUvzH+hvgtYFd/uMg/nMcB34beCn+Y/wN8NrALv+xEP95Hgw8nf8YJ4Bd/uMh/vN8NvBZ/Mf4HOCz+Y+H+M9zETjOf4xbgYfwHw/xn+O1gd/iP9brAL/NfyzEf47PBj6L/1ifA3w2/7EQ/zm+Gvgo/mN9DfDR/MdC/Of4beC1+I/1O8Br8x8L8Z/jq4GP4j/W5wCfzX8sxL/eTwG/DXwNL9hPAW/Nf6yfBt6GF+yjgNcG3oYXHeJf562Bn+KKW4H3AX6b5/RdwHvzn+O7gffhOb028F3Ag7nidYDf5kWD+Nd5OvBgntNvA+8D3Aq8NfBT/Od6G+CngQcD3wW8Ns/pVuAhvGgQL7rPBj6LF+yrgfcGjvOfaxf4buCjecE+Bvhq/mWIF92twIP43+EZwIP5lyFedMeBzwY+iv/Zvgb4bGCXfxniX+/BwHcDr8W/3ccAf81zemngq/i3+x3gvYFbedEh/u1eG/hu4EH8670O8Ns8p9cGfot/vWcAHw38NP96iH+/zwY+i3+d1wF+m+f02sBv8a/zNcBH82+H+Pf7aOCr+Nd5HeC3eU6vDfwW/zofA3w1/3aIf7/vBt6Lf53XAX6b5/TawG/xr/M9wHvzb4f4l70WL9xXAy/Nv87rAL/Nc3pt4Lf41/lr4KN54X6HFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOLf77eB1+Jf53WA3+Y5vTbwW/zr/A7w2vzbIf79vhr4KP51Xgf4bZ7TawO/xb/O1wAfzb8d4t/vvYHv4l/ndYDf5jm9NvBb/Ou8D/Dd/Nsh/n1eG/gq4KX513kd4Ld5Tq8N/Bb/Or8NfAzw1/zbIP5tHgx8FfDW/Nu8DvDbPKfXBn6Lf5vvBj4G2OVfB/Gvcxz4KOCz+ff5a2CX53QceGn+7XaBrwY+hxcd4kX3XsBnAw/mf7ZbgY8Bfpp/GeJF99fAS/G/w98AL82/DPGie23gt3j+fgf4bOCzgdfiP9fvAJ8NfDbwWjx/rwP8Nv8yxL/OdwPvxbM9A/hs4Lu54sHAXwPH+M9xCXhp4FaueG/gq4FjPNvPAG/Niwbxr/Ng4K+54quBrwZ2eU5vDfwU/zneBvhpntNx4KOBz+KKhwC38qJB/Ou9NfDXwK28YO8NfBf/sd4H+G5esAcDLw38NC86xH+e9wa+GjjGv88l4KOB7+Y/HuI/10sD3w28FP82fwO8NXAr/zkQ/zXeG/hs4EG8aJ4BfDbw3fznQvzXemngrYHXBl4aOMYVl4C/Bn4b+Gngr/mvgfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AqgrvkE2YGOQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPestControl;
impl IconShape for MdPestControl {
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
                d: "M21,15v-2h-3.07c-0.05-0.39-0.12-0.77-0.22-1.14l2.58-1.49l-1-1.73L16.92,10c-0.28-0.48-0.62-0.91-0.99-1.29 C15.97,8.48,16,8.25,16,8c0-0.8-0.24-1.55-0.65-2.18L17,4.17l-1.41-1.41l-1.72,1.72c-1.68-0.89-3.1-0.33-3.73,0L8.41,2.76L7,4.17 l1.65,1.65C8.24,6.45,8,7.2,8,8c0,0.25,0.03,0.48,0.07,0.72C7.7,9.1,7.36,9.53,7.08,10L4.71,8.63l-1,1.73l2.58,1.49 c-0.1,0.37-0.17,0.75-0.22,1.14H3v2h3.07c0.05,0.39,0.12,0.77,0.22,1.14l-2.58,1.49l1,1.73L7.08,18c1.08,1.81,2.88,3,4.92,3 s3.84-1.19,4.92-3l2.37,1.37l1-1.73l-2.58-1.49c0.1-0.37,0.17-0.75,0.22-1.14H21z M13,17h-2v-6h2V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/XSwHcBL83zuhX4beBzgFv5nwHxH+fBwF8Bx/mXfTbwOfz3Q/zHeW3gt3jRfQ7w2fz3QvzHeWngr/jXeQhwK/99EP+xPhv4LF503wO8N/99EP/xXho4znN6MPDRwEvxnG4FHsJ/H8R/nY8GvornJf77IP7r/BTw1jynS8Bx/vsg/u3eCnht4KWBBwMP5l/vZ4C35r8P4l/vo4DPBo7z7/c+wHfz3wfxontp4KeAB/Mf43eA1+a/F+JF89bAdwHH+Y/xN8BrA7v890L8y14b+C3+Y1wCvhr4amCX/36IF+6lgd8CjvP8XQJ+GrgV+Gtglxfut/mfBfHC/Rbw2jx/nwN8NbDL/16IF+y1gd/ieV0C3hr4bf73Q7xgPw28Fc/rfYDv5v8GxPN3HLjI8/oZ4K35vwPx/L038F08r5cB/pp/u5cGjvFsl4C/5r8P4vn7bOCzeE7PAB7Mv957AW8NvDZwnOe1C/w28N3Az/BfC/H8fTbwWTyn3wFemxfdWwNfBTyYF80zgLcG/pr/Oojn76eBt+I5/Q7w2vzLjgPfBbw1L7rvAT4a2OXZHgy8FfBg4MHAXwO3Aj8D7PIfA/H8fTbwWTyn3wFemxfuOPBbwEvzovse4L15ttcGPgt4bV6w7wY+Btjl3wfx/H028Fk8L/HC/Rbw2rzofgd4bZ7tq4CP5kWzC7wP8NP82yGev9cGfovn9T7Ad/P8fTXwUbzoLgEvDdzKFd8FvDf/eu8DfDf/NogXbBc4xnO6FXgIz+vBwNP51/kc4LO54qOBr+LfZhd4GeBW/vUQL9h3A+/F8/pu4H14Tt8NvBf/Og8BbgWOAxf59/kZ4K3510O8YA8Gns7z993A+3DFceAi/zp/A7w0V3w08FX8+z0EuJV/HcQL99nAZ/H83Qp8NnAC+Cr+db4G+Giu+G3gtfj3+xjgq/nXQfzL/hp4Kf5jfQ7w2VzxdODB/Pt9DvDZ/Osg/mXHgd8GXor/OK8D/DZXmP8YvwO8Nv86iBfNceC3gZfiP8bnAJ/NFeY/xucAn82/DuJf56uBj+Lf73OAz+aK3wZei3+/rwE+mn8dxL/eg4HPBt4aOMa/zc8Ab80VXw18FP8x3gf4bl50iH+f1wZ+i3+bE8Au8GDg6fzH+Wzgc3jRIP79doFj/Ou9D/DdXPHdwHvxH+e7gffhX4b49/tt4LX417sVeAhXHAd+G3gp/uP8NPA+wC4vGOLf77uB9+Lf5nOAz+aK7wLemxfd3wDfzRXvDbwUz+uvgdcBdnn+EP9+bw38FP92bwO8FfDevOj+BnhpntNfAy/F8/pr4HWAXZ4X4t/vOHArcIz/Oh8DfDXP6aOBr+L5+23gdXheiP8Ynw18Fv91Pgb4ap7TRwNfxQv2OsBv85wQ/zGOA7cCx/iv8dfAy/Cc/gp4aV6wzwE+m+eE+I/z1sBP8V/nr4Hv5or3Bl6aF+57gPfmOSH+Y3028Fn8z/Q5wGfznBD/8T4b+Cz+53kZ4K95Toj/HG8NfDdwjP8ZfgZ4a54X4j/PceCjgY8GjvHf52+A1wZ2eV6I/3zHgdcG3hp4MPDSwDH+8/0O8N3Ad/OCIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPVxy9QbZcKd0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPestControlRodent;
impl IconShape for MdPestControlRodent {
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
                d: "M21.31,17.38l-2.39-2.13C19.44,12.89,17.56,11,15.5,11c-1.16,0-3.5,0.9-3.5,3.5c0,0.97,0.39,1.84,1.03,2.47l-0.71,0.71 C11.5,16.87,11,15.74,11,14.5c0-1.7,0.96-3.17,2.35-3.93c-0.7-0.36-1.48-0.57-2.28-0.57c-2.38,0-4.37,1.65-4.91,3.87 C4.91,13.5,4,12.36,4,11c0-1.66,1.34-3,3-3c0.94,0,1.56,0,2.5,0C10.88,8,12,6.88,12,5.5C12,4.12,10.88,3,9.5,3H8C7.45,3,7,3.45,7,4 c0,0.55,0.45,1,1,1h1.5C9.78,5,10,5.22,10,5.5C10,5.78,9.78,6,9.5,6C9.47,6,9,6,7,6c-2.76,0-5,2.24-5,5c0,2.42,1.72,4.44,4,4.9 v0.03C6,18.73,8.27,21,11.07,21h8.86C21.8,21,22.74,18.66,21.31,17.38z M18,19c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C19,18.55,18.55,19,18,19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9V4LeG2ueDBX3MoVvw38Dv91EP81Xht4L+CtgeO8cLvATwPfA/w2/7kQ/7leGvgq4LX5t/lt4H2AW/nPgfjP81nAZ/Mf46OBr+E/HuI/3nHgq4D35j/WdwMfA+zyHwfxH++ngLfmP8dPA2/DfxzEf6yvBj6K/1xfA3w0/zEQ/3HeGvgp/mu8DfDT/Psh/uM8HXgw/zVuBR7Cvx/iP8Z7A9/Ff633Ab6bfx/Ef4ynAw/mv9atwEP490H8+7008Ff863wP8NXAX3PFSwMfDbwX/zovA/w1/3aIf7+PBr6KF937AN/N8/fewHfxovsY4Kv5t0P8+/028Fq8aD4H+GxeuM8GPosXze8Ar82/HeLf77eB1+JFcwLY5YU7DlzkRfM7wGvzb4f49zMvmt8BXpsXzW8Dr8WLRvzbIf79zIvmd4DX5kXz28Br8aIR/3aIfz/zovlr4GV40fwV8NK8aMS/HeLf77eB1+JF8xDgVl64BwNP50XzO8Br82+H+Pf7buC9eNH8NvA6vHC/Bbw2L5rvAd6bfzvEv99HA1/Fi+67gY8BdnlOx4GvAt6bF93HAF/Nvx3i3+/BwNP519kFvhv4a654aeC9geP86zwEuJV/O8R/jFuBB/Ff6xnAg/n3QfzH+Gzgs/iv9TnAZ/Pvg/iP8WDg6fzXeghwK/8+iP84Pw28Ff81fgZ4a/79EP9xXhv4Lf5rvA7w2/z7If5j/TbwWvzn+h3gtfmPgfiP9drAb/Gf63WA3+Y/BuI/3m8Dr8V/jt8BXpv/OIj/eA8Gns5/jocAt/IfB/Gf46uBj+I/1tcAH81/LMR/juPAXwMP4j/GM4CXBnb5j4X4z/PWwE/xH+NtgJ/mPx7iP9dPA2/Fv8/PAG/Nfw7Ef67jwK3AMf5tLgEPBnb5z4H4z/fWwE/xb/M2wE/znwfxX+OrgY/iX+drgI/mPxfiv8Zx4LeBl+JF8zfAS/OfD/Ff56WB3waO8cJdAl4b+Gv+8yH+a7018FO8cG8D/DT/NRD/9b4a+Ciev68BPpr/Ooj/Hj8NvBXP6WeAt+a/FuK/x3Hgt4GX4oq/AV4b2OW/FuK/z4OBv+aKlwZu5b8e4r/XS3PFX/PfA/H/G+L/N8S/zVcDL8X/LH8DfDT/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lq4KX5n+WvgY/mXwfx/xvi/zfE/2+I/98Q/7/xj6JwhkHZvCb5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPinDrop;
impl IconShape for MdPinDrop {
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
                d: "M18 8c0-3.31-2.69-6-6-6S6 4.69 6 8c0 4.5 6 11 6 11s6-6.5 6-11zm-8 0c0-1.1.9-2 2-2s2 .9 2 2-.89 2-2 2c-1.1 0-2-.9-2-2zM5 20v2h14v-2H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDZ/NvtAl8NfA2wy388xH+OjwI+GzjOf4xd4GOA7+Y/FuI/3ncB781/ju8G3of/OIj/OMeB3wJemv9cfw28DrDLvx/iP85fAS/Nf42/Bl6Gfz/Ef4zvAt6b/1rfDbwP/z6If7+PBr6K/x7vA3w3/3aIf5/jwNOB4/z32AUeAuzyb4P49/ls4LP47/U5wGfzb4P4t3sw8HT+++0CLwPcyr8e4t/uo4Gv4t/mb4DfBv6aK14aeG3gpfi3+Rjgq/nXQ/zb/TbwWvzrXAI+Gvhunr/3Br4aOMa/zu8Ar82/HuLf5jhwkX+dS8BrA3/NC/fSwG8Dx/jXOQHs8q+D+Ld5beC3+Nd5H+C7edG8N/Bd/Ou8DvDb/Osg/m0+G/gsXnR/A7w0/zq3Ag/iRfcxwFfzr4P4t/ls4LN40X0N8NH863w18FG86D4H+Gz+dRD/Nt8NvBcvuvcBvpt/nY8GvooX3dcAH82/DuLf5ruB9+JF9z7Ad/Ov89HAV/Gi+x7gvfnXQfzbfDbwWbzovgb4aP51vhr4KF50nwN8Nv86iH+bzwY+ixfdXwMvw7/O04EH86L7HOCz+ddB/Nu8NvBb/Ou8D/DdvGjeG/gu/nVeB/ht/nUQ/zbHgYv86+wCrwP8NS/cSwO/BRznX+cEsMu/DuLf7lbgQfzr7AIfDXwPz997AV8NHOdf5xnAg/nXQ/zbfTXwUfzb3Ar8NPDXXPHSwFsDD+bf5muAj+ZfD/Fv99LAX/E/w8sAf82/HuLf51bgQfz3egbwYP5tEP8+nw18Fv+9Pgf4bP5tEP8+Dwaezn+vhwC38m+D+Pf7buC9+O/xPcB782+H+Pd7aeCv+O/xEOBW/u0Q/zF+G3gt/mv9DPDW/Psg/mO8NvBb/Nd6GeCv+fdB/Mf5buC9+K/xPcB78++H+I/zYODp/Nd4CHAr/36I/1ifDXwW/7k+B/hs/mMg/mMdB/4aeBD/OZ4BvDSwy38MxH+8twZ+iv8cbwP8NP9xEP85fhp4K/5j/Qzw1vzHQvzneDDw18Ax/mNcAh4M7PIfC/Gf56OBr+I/xscAX81/PMR/rp8G3op/n58B3pr/HIj/XMeBW4Fj/NtcAh4M7PKfA/Gf762Bn+Lf5m2An+Y/D+K/xlcDH8W/ztcAH81/LsR/nb8GXooXzd8AL81/PsR/nQcDfw0c44W7BLw0cCv/+RD/td4a+CleuLcBfpr/Goj/ep8NfBbP3+cAn81/HcR/j58G3orn9D3Ae/NfC/Hf4zjw28BLccXfAK8N7PJfC/Hf5zhwK1c8GNjlvx7iv9dLc8Vf898D8f8b4v83xP9viP/fEP+/8Y8DHZxBeLG/SAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlace;
impl IconShape for MdPlace {
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
                d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+Z/so4L2Bl+aKvwa+Gvge/mMg/mc6DvwU8No8fz8NvA+wy78P4n+e48BvAS/NC/fXwOsAu/zbIf5nOQ78FvDSvGj+GngdYJd/G8T/HMeB3wJemn+dvwZeB9jlXw/xP8Nx4LeAl+bf5q+B1wF2+ddB/Pc7DvwW8NL8+/w18DrALi86xH+v48BvAS/Nf4yfBt6GFx3iP9+DueJWntNx4LeAl+Y/1ucAn82LBvGf57WB7wIezBW3Au8D/DZXHAd+G3gp/mXfwxXvxb9sF3gIsMu/DPGf46WBv+L5ex/gu7niOPDbwEvxgn0P8N5c8d3Ae/Evex/gu/mXIf5z/DTwVrxg7wN8N1ccB34beCme1/cA781z+m7gvXjhvgd4b/5liP8cF4HjvHDvA3w3VxwHfht4KZ7te4D35vn7buC9eMF+B3ht/mWI/xy3Ag/iX/Y+wHdzxXHgt4GXAr4HeG9euO8G3ovn73eA1+ZfhvjP8dXAR/GieR/gu7niOPDRwGfzbMeBXZ6/7wbei+f1OcBn8y9D/Oc4Dvw28FK8aN4H+G6e13cBLw28DrDL8/fdwHvxnF4H+G3+ZYj/PMeB3wZeihfN+wDfzbN9F/DeXPHXwOsAuzx/3w28F1f8DvDavGgQ/7mOA78NvBQvmvcBvhv4LuC9eU5/DbwOsMvz993AewEvA/w1LxrEf7zjwGsDP80Vx4HfBl6KF81vA6/N8/fXwOsAuzx/rw38Ni86xH+s48BvAS8NvA/w3VxxHPht4KX49/tr4HWAXf79EP9xjgO/Bbw0z/Y+wHdzxXHgt4GX4t/vr4HXAXb590H8xzgO/Bbw0jyv9wG+myuOA78NvBT/fn8NvA6wy78d4t/vOPBbwEvzgr0N8NNccRz4beCl+Pf7a+Bl+LdD/PscB34LeGleuF3gdYC/5orjwG8DL8W/3+sAv82/DeLf7jjwW8BL86L5GuCjebbvAt6bf7+3AX6afxvEv91PAW/Ni+Z7gPfm2b4LeG/+/Z4BvDSwy78N4t/mpYG/4kXzPcB782zfBbw3/zbvwxXvDdwKfDZwK/92iH+bzwY+i3/Z9wDvzbN9F/De/Nu8D/Dd/MdC/Nt8NPBVvHDfA7w3z/ZdwHvzb/M+wHfzHw/xb/Ng4Om8YN8DvDfP9l3Ae/Nv8z7Ad/OfA/Fv99nAZ/G8vgb4aJ7tu4D35t/mfYDv5j8P4t/ntYHPBo4DtwJfDfw2z/ZdwHvzb/M+wHfznwvxn+e7gPfm3+Z9gO/mPx/iP8d3Ae/Nv837AN/Nfw3Ef7zvAt6bf5v3Ab6b/zqI/1jfBbw3/zbvA3w3/7UQ/3G+C3hv/m3eB/hu/ush/mN8F/De/Nu8D/Dd/PdA/Pt9F/De/Nu8D/Dd/PdB/Pt8F/De/Nu8D/Dd/PdC/Nt9F/De/Nu8D/Dd/PdD/Nt8F/De/Nu8D/Dd/M+A+Nf7LuC9+bd5H+C7+Z8D8a/z1cBH8W/zPsB38z8L4kX32sBv8W/zPsB38z8P4kX32cBn8a/3PsB38z8T4kX328Br8a/zPsB38z8X4kX33cB78aJ7H+C7+Z8N8aL7aOCreNG8D/Dd/M+H+Nf5a+CleOHeB/hu/ndA/Ou8NPDbwDGev/cBvpv/PRD/eg8GPht4a+AYV/wM8NnAX/O/C+L/N8T/b4j/3xD/vyH+f0P8/8Y/AmoevkE1U/wOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlumbing;
impl IconShape for MdPlumbing {
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
                d: "M19.28,4.93l-2.12-2.12c-0.78-0.78-2.05-0.78-2.83,0L11.5,5.64l2.12,2.12l2.12-2.12l3.54,3.54 C20.45,8,20.45,6.1,19.28,4.93z",
            }
            path {
                d: "M5.49,13.77c0.59,0.59,1.54,0.59,2.12,0l2.47-2.47L7.96,9.17l-2.47,2.47C4.9,12.23,4.9,13.18,5.49,13.77L5.49,13.77z",
            }
            path {
                d: "M15.04,7.76l-0.71,0.71l-0.71,0.71l-3.18-3.18C9.85,5.4,8.9,5.4,8.32,5.99c-0.59,0.59-0.59,1.54,0,2.12l3.18,3.18 L10.79,12l-6.36,6.36c-0.78,0.78-0.78,2.05,0,2.83c0.78,0.78,2.05,0.78,2.83,0L16.45,12c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L15.04,7.76z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRailwayAlert;
impl IconShape for MdRailwayAlert {
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
                d: "M23 8a7 7 0 0 0-11.95-4.95A33.8 33.8 0 0 0 9 3c-4.42 0-8 .5-8 4v10.5A3.5 3.5 0 0 0 4.5 21L3 22.5v.5h12v-.5L13.5 21a3.5 3.5 0 0 0 3.5-3.5v-2.58A7 7 0 0 0 23 8zM3 12V7h6.08a6.96 6.96 0 0 0 1.18 5H3zm6 7c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm7.71-6.06l-.2.03L16 13l-.47-.02-.16-.02-.29-.04-.2-.04-.22-.06a1.55 1.55 0 0 1-.23-.07l-.13-.05A4.99 4.99 0 0 1 11.1 7c.04-.19.09-.37.15-.54l.05-.14.15-.38.07-.15.2-.36.07-.12.3-.42.02-.02c.24-.3.52-.57.82-.81l.01-.01.46-.32.03-.02A5.25 5.25 0 0 1 16 3a5 5 0 0 1 .71 9.94zM15 4h2v5h-2zm0 6h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/d70WV7w0cBwQ8Nk8J8T/Tg8GHgQcB14aOA68NFe8Ns/f7wCvzXNC/M/0Wlzx0sBx4KWB48CDgQfzb/M7wGvznBD/9R4MPAg4Drw0cBx4aa54bf59/gbYBf4a2AVuBW4FbgVu5Xkh/uO9Fle8NHAceGngOPBg4MH8210C/hrYBf6aK36bK36bfxvEv81rA68FHAdemitem3+fvwF2gb8GdoFbgVuBW4Fb+c+B+Lf5beC1+Ne5BPw28Ndc8dtc8dv890H82/w28Fo8299wxUtxxecAvw3cCjydK34HeG3+Z0H82/w28FpcIa54beC3uOJ1gN/mCnPF7wCvzf8siH+b3wZeiyvEFa8N/BZXvA7w21xhrvgd4LX5nwXxb/PbwGtxhbjitYHf4orXAX6bK8wVvwO8Nv+zIP5tfht4Lf53+R3gtXlOiH+b3wZei/9dfgd4bZ4T4t/mt4HX4orf4YrjwEtxxd8Au1zxWlxxCfhr/vv8NfDRPCfEv81vA6/FFeKK1wZ+iyteB/htrjBX/A7w2vzPgvi3+W3gtbhCXPHawG9xxesAv80V5orfAV6b/1kQ/za/DbwWV4grXhv4La54HeC3ucJc8TvAa/M/C+Lf5reB1+IKccVrA7/FFa8D/DZXmCt+B3ht/mdB/Nv8NvBaXCGueG3gt7jidYDf5gpzxe8Ar83/LIh/m98GXosrxBWvDfwWV7wO8NtcYa74HeC1+Z8F8W/z28BrcYW44rWB3+KK1wF+myvMFb8DvDb/syD+bX4beC2uEFe8NvBbXPE6wG9zhbnid4DX5n8WxL/NbwOvxRXiitcGfosrXgf4ba4wV/wO8Nr8z4L4t/lt4LW4Qlzx2sBvccXrAL/NFeaK3wFem/9ZEP82vw28FleIK14b+C2ueB3gt7nCXPE7wGvzPwvi3+a3gdfiCnHFawO/xRWvA/w2V5grfgd4bf5nQfzb/DbwWlwhrnht4Le44nWA3+YKc8XvAK/N/yyI5++7gffi/5bvAd6b54R4/j4b+Cz+b/kc4LN5Tojn77OBz+L/ls8BPpvnhHj+3hv4Lv5veR/gu3lOiOfvwcDT+b/lIcCtPCfEC3Yr8CD+b3gG8GCeF+IF+27gvfi/4XuA9+Z5IV6wtwZ+iv8b3gb4aZ4X4oW7FXgQ/7s9A3gwzx/ihfto4Kv43+19gO/m+UO8cMeBW4Fj/O90CXgwsMvzh/iXvTXwU/zv9DbAT/OCIV40vw28Fv+7/Azw1rxwiBfNg4G/Bo7xv8Ml4MHALi8c4kX30sBvA8f4n+0S8NrAX/MvQ/zrvDfwXfzP9jbAT/OiQfzrvTfw1cAx/me5BHw08N286BD/Ni8N/DZwjP8ZLgGvDfw1/zqIf7uXBr4beCn+e/0O8NHAX/Ovh/j3+2jgs4Fj/Ne6BHw28NX82yH+YzwY+Gzgvfiv8T3AZwO38u+D+I/1YOCzgffiP8f3AJ8N3Mp/DMR/jt8GXov/WL8DvDb/sRD/OX4beC3+Y/0O8Nr8x0L85/ht4LX4j/U7wGvzHwvxn+O3gdfiP9bvAK/NfyzEf46vBl6a/1h/DXw0/7EQ/78h/n9D/P+G+P8N8f8b/whoF8BBeTFQiAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRamenDining;
impl IconShape for MdRamenDining {
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
                d: "M9,6H8V4.65l1-0.12V6z M9,12H8V7h1V12z M6,7h1v5H6V7z M6,4.88l1-0.12V6H6V4.88z M22,3V2L5,4v8H2c0,3.69,2.47,6.86,6,8.25 V22h8v-1.75c3.53-1.39,6-4.56,6-8.25H10V7h12V6H10V4.41L22,3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/utcAt4b+G3gq4H34j/e7wCvzXNCPH+/DbwW/zUuAa8N/DXP9t3Ae/Ef63eA1+Y5IZ6/3wZei/98l4DXBv6a5/XdwHvxH+d3gNfmOSGev98GXov/XJeA1wb+mhfsu4H34j/G7wCvzXNCPH+/DbwW/3kuAa8N/DX/su8G3ot/v98BXpvnhHj+fht4Lf5zXAJeG/hrXnR/DbwU/z6/A7w2zwnx/P028Fr8x7sEvDbw1/zr/BXw0vz7/A7w2jwnxPP328Br8R/rEvDawF/zr/NdwHvz7/c7wGvznBDP328Dr8V/nEvAawN/zb/OdwHvzX+M3wFem+eEeP5+G3gt/mNcAl4b+Gv+db4LeG/+4/wO8No8J8Tz99vAa/Hvdwl4beCv+df5LuC9+Y/1O8Br85wQz99vA6/Fv88l4LWBv+Zf57uA9+Y/3u8Ar81zQjx/vw28Fv92l4DXBv6af53vAt6b/xy/A7w2zwnx/P028Fr821wCXhv4a/51vgt4b/7z/A7w2jwnxPP328Br8a93CXht4K/51/ku4L35z/U7wGvznBDP328Dr8W/ziXgtYG/5l/nu4D35j/f7wCvzXNCPH+/DbwWL7pLwGsDf82/zncB781/jd8BXpvnhHj+fht4LV40l4DXBv6af53vAt6b/zq/A7w2zwnx/P028Fq8aF4G+Gv+db4LeG/+a/0O8No8J8Tz99vAa/GiEf863wW8N//1fgd4bZ4T4vn7beC1eNGIF913Ae/Nc/ob4LWBXf5tzIvmd4DX5jkhnr/fBl6LF4140XwX8N48p78BXhvY5d/OvGh+B3htnhPi+ftt4LV40Yh/2XcB781z+hvgtYFd/n3Mi+Z3gNfmOSGev98GXosXjXjhvgt4b57T3wCvDezy72deNL8DvDbPCfH8/TbwWrxoxAv2XcB785z+BnhtYJf/GOZF8zvAa/OcEM/fbwOvxYvmt3n+jgMvzfP6a2CX/zivzYvmd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/lc4DP5jkhnr/fBl6L/1veBvhpnhPi+ftt4LX4v+NvgJfmeSGev98GXov/G/4GeGvgVp4X4vn7beC1+N/tGcB3A18N7PL8IZ6/3wZei3/Z+wDfzf9eiOfvt4HX4oV7H+C7+d8N8fz9NvBavGDvA3w3//shnr/fBl6L5+99gO/m/wbE8/fbwGvxvN4H+G7+70A8f78NvBbP6X2A7+b/FsTz99vAa/Fs7wN8N//3IJ6/3wZeiyveB/hu/m9CPH+/DbwW8D7Ad/N/F+L5+23gu4Hv5v82xPP30sBf838f4v83xP9viP/fEP+/If5/4x8B9frAQaICrMgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRateReview;
impl IconShape for MdRateReview {
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 14v-2.47l6.88-6.88c.2-.2.51-.2.71 0l1.77 1.77c.2.2.2.51 0 .71L8.47 14H6zm12 0h-7.5l2-2H18v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRestaurant;
impl IconShape for MdRestaurant {
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
                d: "M11 9H9V2H7v7H5V2H3v7c0 2.12 1.66 3.84 3.75 3.97V22h2.5v-9.03C11.34 12.84 13 11.12 13 9V2h-2v7zm5-3v8h2.5v8H21V2c-2.76 0-5 2.24-5 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRestaurantMenu;
impl IconShape for MdRestaurantMenu {
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
                d: "M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGrklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/PJ8DfDVwHHhv4LP4t/tp4G34lyFeNO8NfBf/eX4GeGue008Db8W/3fsA380Lh/iXHQeeDhznP8/7AN/Nc/po4Kv4t9sFHgLs8oIh/mWfDXwW/7m+BvhontNXAx/Fv8/nAJ/NC4Z44Y4DTweO859rF3gd4K+54qWB3wKO8++zCzwE2OX5Q7xwnw18Fv91fpsrXpv/OJ8DfDbPH+KFezrwYP7zXAJ+Gvht4KWB9waO8R/rVuAhPH+IF+ytgZ/iP8fPAD8NfDfP6aWBv+I/3usAv83zQrxgXw18FP9x/gb4buC7gV1esK8GPor/WF8DfDTPC/GC/RXw0vz7fQ3w3cBf86I5DtwKHOM/zl8DL8PzQjx/Dwaezn8M8ZweDLwVcAL4bJ6/1wZ+i/9YJ4BdnhPi+Xtt4Lf4jyGueDDwVcBbc8XnAJ/Nsz0YOA78NVe8N/DVwDH+Y7wO8Ns8J8Tz99HAV/EfQ1zx2sBv8WyvA/w2z/bawE8BLwPcyhUPBr4beC3+/T4G+GqeE+L5+2zgs/iPIa54beC3eLYTwC7P9tnAZwF/DbwP8Nc824OBtwa+in+7zwE+m+eEeP4+G/gs/mOIK14b+C2ueAbwYJ7TTwNvxbN9NPA1PNtrA7/Fv93nAJ/Nc0I8f78NvBb/McQVrw38Flf8DvDaPKe/Al6a53Qr8N3AbwMCfot/u58B3prnhHj+fht4Lf7zfA7w2Twn85/rZ4C35jkhnr/PBj6L/zxvA/w0z/Zg4On85/oc4LN5Tojn77OBz+I/z0OAW3lO7w18F/95Pgf4bJ4T4vn7aOCr+M9xCTjOFa8N/DWwyxXvDXwX/zk+BvhqnhPi+Xtt4Lf4z/E7wGtzxWcDbwW8DrDLFZ8NfBb/8V4H+G2eE+L5ezDwdP5tngHcCrw0cIzn9TnAZ3PFbwOvBfw18DrALlf8NfBS/Mc6AezynBAv2F8DL8W/3vcA780VLw28NPDSwEsDrwW8DfDTXHEROM4VXwN8NFf8NvBa/Mf5G+CleV6IF+yrgY/iX+9jgK/mX/Zg4Ok82+8Ar80Vvw28Fv9xvgb4aJ4X4gV7beC3+Nf7a+Cngb8G/ga4lefvrYGf4tl+B3htrvht4LX4j/M6wG/zvBAv3K3Ag/j32QX+Gvht4K+BvwFuBT4b+Cye7XeA1+aK3wZei/8YzwAezPOHeOE+G/gs/uPtcsVxnu13gNfmit8GXov/GJ8DfDbPH+KFOw7cChzjP9/vAK/NFb8NvBb/fpeABwO7PH+If9lnA5/Ff77fAV6bK34beC3+/T4H+GxeMMS/7DhwK3CM/1y/A7w2V/w28Fr8+1wCHgzs8oIhXjTvDXwX/7l+B3htrvht4LX493kb4Kd54RAvup8G3or/PL8DvDZX/DbwWvzb/Qzw1vzLEC+648BfAw/if7ZnAC8N7PIvQ/zrvDTw28Ax/me6BLw28Ne8aBD/ei8N/DZwjP9ZLgGvDfw1LzrEv81LA78NHON/hkvAawN/zb8O4t/upYGfBh7Ef69nAG8N/DX/eoh/n+PAdwNvxX+PnwHeG9jl3wbxH+O9ga8GjvFf4xLw3sBP8++D+I9zHPho4KOBY/znuAR8NfDVwC7/foj/eMeBjwbeG3gQ/zGeAXw38NXALv9xEP+5Xht4a+C1gZfiX+dvgN8Gfhr4bf5zIP7rHAdeGnhp4DhXvDRX/DVX7AJ/Dfw1sMt/PsT/b4j/3xD/vyH+f0P8/8Y/Asj9AFCMmI7UAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRunCircle;
impl IconShape for MdRunCircle {
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.5,6c0.55,0,1,0.45,1,1 c0,0.55-0.45,1-1,1s-1-0.45-1-1C12.5,6.45,12.95,6,13.5,6z M16,12c-0.7,0-2.01-0.54-2.91-1.76l-0.41,2.35L14,14.03V18h-1v-3.58 l-1.11-1.21l-0.52,2.64L7.6,15.08l0.2-0.98l2.78,0.57l0.96-4.89L10,10.35V12H9V9.65l3.28-1.21c0.49-0.18,1.03,0.06,1.26,0.53 C14.37,10.67,15.59,11,16,11V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFD0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+PAf6aF+yrgZfiOf0O8Nk8r9/iP5d4wRD/MvO8Xgf4bV6w3wZei+f0OcBn85weDDyd/1ziBUP8y8zzeh3gt3n+jgMXeV6fA3w2z+m1gd/iP5d4wRD/MvO8Xgf4bZ6/9wa+i+f1MsBf85xeG/gt/nOJFwzxLzPP63WA3+b5ezrwYJ7TJeA4z+urgY/iP5d4wRD/MvO8Xgf4bZ7XdwHvzfP6HuC9eV5/Bbw0/7nEC4b4l5nn9TrAb/Ocvgt4b57XJeClgVt5Tg8Gns5/PvGCIf5l5nm9DvDbXPFewGcDD+b5+xzgs3lenw18Fv/5xAuG+JeZ5/XXwC7w2rxwfwO8NM/rOPB04Dj/+cQLhviXmX+bvwFeG9jleX028Fn81xAvGOJfZv71/gZ4bWCX53UceDpwnP8a4gVD/MvMi+4S8NXAZ/OC/RXw0vzXES8Y4l9m/mWXgJ8GPhu4lRfsu4D35r+WeMEQ/zLzvH4H+G2u+Gngr/mXfRbw2fzXEy8Y4l9mntfrAL/Ni+67gPfmv4d4wRD/MvO8Xgf4bf5lx4HfAl6a/z7iBUP8y8zzeh3gt3nBjgMfBXw0cJz/XuIFQ/zLzPN6HeC3eV4PBt4K+GzgOP8ziBcM8S8zz+t1gFuBB3HFWwOvDbw0//OIFwzxLzP/Pd4G+GrgQfz7iBcM8S8z//U+B/hs4KWBv+LfR7xgiH+Z+a/1O8Br82wfDXwV/3biBUP8y8x/nWcALw3s8px+Gngr/m3EC4b4l5l/m7cBPht4KV50LwP8Nc/rOPDXwIP41xMvGOJfZv71Pgf4bOClgd8GjvEvex/gu3nBXhr4K/71xAuG+JeZf52fAd6aZ3tt4Ld44b4HeG/+ZR8NfBX/OuIFQ/zLzIvuGcBLA7s8p/cGvovn72+A1wZ2edH8NPBWvOjEC4b4l5kXzSXgtYG/5vn7buC9eE6XgJcGbuVFdxz4a+BBvGjEC4b4l5kXzfsA380L99vAa/FsbwP8NP96Lw38FS8a8YIh/mXmX/Y9wHvzLzsO/DbwUsDnAJ/Nv91HA1/Fv0y8YIh/mXnh/gZ4aV50Lw18NvDW/Pv9NPBWvHDiBUP8y8wLdgl4aeBW/nscB/4aeBAvmHjBEP8y84K9DvDb/Pd6aeCveMHEC4b4l5nn73OAz+Z/ho8GvornT7xgiH+ZeV4/A7w1/7P8NPBWPC/xgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7IHA0/nf6eHALfygiFeNO8NfBf/u7wP8N28cIgX3YOBzwbeGjjG/0yXgJ8GPhu4lX8Z4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8Isy6xQXdpXE0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSatellite;
impl IconShape for MdSatellite {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM5 4.99h3C8 6.65 6.66 8 5 8V4.99zM5 12v-2c2.76 0 5-2.25 5-5.01h2C12 8.86 8.87 12 5 12zm0 6l3.5-4.5 2.5 3.01L14.5 12l4.5 6H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z44Y4DHwW8N/Bg/ne5Ffhu4GuAXZ4/xAv2YOC3gAfzv9tfA28D3MrzQrxgfwW8NP83/DXwMjwvxPP31sBP8X/L2wA/zXNCPH+fDXwW/7d8DvDZPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/MS4Bf83zejDwIP7r/A7w2jwnxPP328Br8a/3DOC3gd8Gfhu4lX/ZawMvDbw08NbAMf5z/A7w2jwnxPP328Br8aL7HuC7gd/m3++tgY8GXosX3c8APw38NfDXwG8Dr8Vz+h3gtXlOiOfvt4HX4l/2NcBnA7v8x3tt4LOB1+IF+x3go4G/5jn9NvBaPKffAV6b54R4/n4beC1esL8B3hv4a/7zvTfw1cAxntP3AO/N8/fbwGvxnH4HeG2eE+L5+23gtXj+vgb4aP5rHQd+G3gprvgd4LV5wX4beC2e0+8Ar81zQjx/vw28Fs/rfYDv5r/PdwPvBbwO8Nu8YL8NvBbP6XeA1+Y5IZ6/3wZei+f0PsB389/vs4HP5oX7beC1eE6/A7w2zwnx/P028Fo82+cAn83/Hr8NvBbP6XeA1+Y5IZ6/3wZei2fbBV4GuJX/fq8N/DYv3G8Dr8Vz+h3gtXlOiOfvt4HX4jn9NfA+wF/z3+e9ge8CTgC7vGC/DbwWz+l3gNfmOSGev98GXovntQu8DfDb/Nf7KOCrueJzgM/mBftt4LV4Tr8DvDbPCfH8/TbwWrxgHw18Df81jgPfBbw1z+l9gO/m+ftt4LV4Tr8DvDbPCfH8/TbwWrxwfw18DPDb/Of5KOCzgeM8fx8NfA3P67eB1+I5/Q7w2jwnxPP328Br8aL5beCrgZ/hP8Zx4K2AzwYezL/sVuCzgZ8Bdrnit4HX4jn9DvDaPCfE8/fbwGvxr3Mr8NPAbwO/A+zyojsOvBXw1sBb82/318Au8NLAcZ7T7wCvzXNCPH+/DbwW/z5/DdwK/DVX7AJ/Dbw2z/bSwEsDD+Y/3+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP33cB78X/L1wAfzXNCPH8fDXwV/7d8DPDVPCfE83ccuBU4xv8Nl4AHA7s8J8QL9t7Ad/F/w9sAP83zQrxwrw18N/Ag/nd6BvDewG/z/CFeNC8NHOd/l13gr3nhEP+/If5/Q/z/hvj/DfF/22sBv8MLhvif5TjwUjyn48BL85weDDyY5/TSwHGel3jBEP9+x4GX4jk9GHgwz+mlgeM8p9fmP594wRD/st/iOb02/7uIFwzxLzP/e/0N8NK8YIh/2S5wjP86v8Nz2gX+mud0K3Arz+lW4Fb+dRD/st8GXosX7nd4TrvAX/OcbgVu5Tn9NbDLfx/Ev+ylgeM8218Du/zfgPj/DfH/G+JfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8ImrOrQQDost0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSetMeal;
impl IconShape for MdSetMeal {
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
                d: "M21.05,17.56L3.08,18.5L3,17l17.98-0.94L21.05,17.56z M21,19.48H3v1.5h18V19.48z M22,5v7c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h16C21.1,3,22,3.9,22,5z M20,6c-1.68,0-3.04,0.98-3.21,2.23C16.15,7.5,14.06,5.5,10.25,5.5 c-4.67,0-6.75,3-6.75,3s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73C16.96,10.02,18.32,11,20,11V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD7ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcceCn+d/gbYJd/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEP+/If5/Q7zoHgw8iP8d/gbY5V+GeNF9NvBZ/O/wOcBn8y9DvOh+G3gt/nf4GeCt+ZchXnQXgeP873Ar8BD+ZYgXzYOBp/O/ywlglxcO8aJ5a+Cn+N/ldYDf5oVDvGg+G/gs/nf5HOCzeeEQL5rfBl6L/11+BnhrXjjEi+YicJz/XW4FHsILh/iXPRh4Ov87nQB2ecEQ/7K3Bn6K/51eB/htXjDEv+yzgc/if6fPAT6bFwzxL/tt4LX43+lngLfmBUP8yy4Cx/nf6VbgIbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/2WtzxUsDX8Xzeh3+fX6L5/UxwF/zovktntfHAH/NFb/NC4Z40b028Fs8L/HvY57X6wC/zYvGPK/XAX6bfxniRffawG/xvMS/j3lerwP8Ni8a87xeB/ht/mWIF91rA7/F8xL/PuZ5vQ7w27xozPN6HeC3+ZchXnSvDfwWz0v8+5jn9TrAb/OiMc/rdYDf5l+GeNG9NvBbPC/x72Oe1+sAv82Lxjyv1wF+m38Z4kX32sBv8bzEv495Xq8D/DYvGvO8Xgf4bf5liBfdawO/xfMS/z7meb0O8Nu8aMzzeh3gt/mXIV50rw38Fs9L/PuY5/U6wG/zojHP63WA3+ZfhnjRvTbwWzwv8e9jntfrAL/Ni8Y8r9cBfpt/GeJF99rAb/G8xL+PeV6vA/w2LxrzvF4H+G3+ZYgX3WsDv8V/jdcBfpsXjXlerwP8Nv8yxIvutYHf4r/G6wC/zYvGPK/XAX6bfxniRffawG/xX+N1gN/mRWOe1+sAv82/DPGie23gt/iv8TrAb/OiMc/rdYDf5l+GeNG9NvBb/Nd4HeC3edGY5/U6wG/zL0O86I4DL81/jb8GdnnRvDbP66+BXf5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPX6V9QfUObC8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStoreMallDirectory;
impl IconShape for MdStoreMallDirectory {
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
                d: "M20 4H4v2h16V4zm1 10v-2l-1-5H4l-1 5v2h1v6h10v-6h4v6h2v-6h1zm-9 4H6v-4h6v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/r9fiOf0O/3qI/z0eDLwX8NbAS/P8/TXw08D3ALfyL0P8z3cc+CrgvfnX+Wrgc4BdXjDE/2wvDfwWcJx/m13gdYC/5vlD/M/13sB38R/jfYDv5nkh/md6aeCv+I/1MsBf85wQL7oHA58FvDVwnP847wN8N892HHg6cJz/WLvAQ4Bdng3xonlv4Lv4j3cJeDCwy7N9N/Be/Of4GuCjeTbEv+zBwNP5z/E9wHvzbA8Gns5/rocAt3IF4l/23cB78Z/jbYCf5tk+G/gs/nN9DvDZXIH4l10EjvOfQzynvwJemv9cfw28DFcg/mXmP8czgAfznMx/DXEF4l9m/nP8DvDaPNtrA7/Ff43XAX4bQPzLzH+O3wFem2d7beC3+K/xOsBvA4h/mfnP8TvAa/Nsrw38Fv81Xgf4bQDxLzP/OX4HeG2e7bWB3+K/xusAvw0g/mXmP494Tua/hrgC8S8z/3lOALs8218DL8V/rr8BXporEP8y85/nbYCf5tk+G/gs/nN9DvDZXIH4l5n/PN8DvDfP9mDg6fzneghwK1cg/mXmP88ucILn9N3Ae/Gf42uAj+bZEP8y85/rY4Cv5tmOA7cCx/iPdQl4MLDLsyH+ZeY/1y7wEGCXZ3tp4K/4j/UywF/znBD/MvOf73OAz+Y5vTfwXfzHeB/gu3leiH+Z+a/xNsBP85xeGvht4Bj/NpeA1wb+mucP8S8z/zV2gdcB/prndBz4bOCj+Nf5GuCzgV1eMMS/zPzXeR/gu3n+Hgy8N/DWwEvx/P0N8NPAdwO38i9D/MvMf433Ab6bF91r85x+m389xL/M/Od7H+C7+a+H+JeZ/1zvA3w3/z0Q/zLzn+d9gO/mvw/iX2b+c7wP8N3867w08FLAg7niwcCDgd/miluBW4Hf4UWD+JeZ/3jvA3w3/7LjwHsBrw28NnCcF91vAz8NfA0vGOJfZv5jvQ/w3bxwDwY+C3hv/v3EC4b4l5n/OO8DfDcv2IOBrwLemv844gVD/MvMf4z3Ab6bF+yjgM8GjvMfS7xgiH+Z+fd7H+C7ef6OA78FvDT/OcQLhviXmX+f9wG+mxfss4HP4j+PeMEQ/zLzb/c+wHfzwl0EjvOfR7xgiH+Z+bd5H+C7+ZeZ/1ziBUP8y8y/3vsA382LxvznEi8Y4l9m/nXeB/huXnTmP5d4wRD/MvOiex/gu/nXMf+5xAuG+JeZF837AN/Nv575zyVeMMS/zPzL3gf4bv5tzH8u8YIh/mXmhXsf4Lv5tzP/ucQLhviXmRfsfYDv5t/H/OcSLxjiX2aev/cBvpt/P/OfS7xgiH+ZeV7vA3w3/zHMfy7xgiH+ZbvAMZ7tfYDv5j+O+c9zCTjOC4b4l3038F5c8T7Ad/Mfy/zn+R7gvXnBEP+yBwNPB94H+G7+45n/PA8BbuUFQ7xoHgzcyn8O85/jfYDv5oVD/Pcz/3EuAT8NfDZwK/8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RIWe/QXbDkjAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStreetview;
impl IconShape for MdStreetview {
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
                d: "M12.56 14.33c-.34.27-.56.7-.56 1.17V21h7c1.1 0 2-.9 2-2v-5.98c-.94-.33-1.95-.52-3-.52-2.03 0-3.93.7-5.44 1.83z",
            }
            circle {
                cx: "18",
                cy: "6",
                r: "5",
            }
            path {
                d: "M11.5 6c0-1.08.27-2.1.74-3H5c-1.1 0-2 .9-2 2v14c0 .55.23 1.05.59 1.41l9.82-9.82C12.23 9.42 11.5 7.8 11.5 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubway;
impl IconShape for MdSubway {
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
                cx: "15.5",
                cy: "16",
                r: "1",
            }
            circle {
                cx: "8.5",
                cy: "16",
                r: "1",
            }
            path {
                d: "M7.01 9h10v5h-10zM17.8 2.8C16 2.09 13.86 2 12 2c-1.86 0-4 .09-5.8.8C3.53 3.84 2 6.05 2 8.86V22h20V8.86c0-2.81-1.53-5.02-4.2-6.06zm.2 13.08c0 1.45-1.18 2.62-2.63 2.62l1.13 1.12V20H15l-1.5-1.5h-2.83L9.17 20H7.5v-.38l1.12-1.12C7.18 18.5 6 17.32 6 15.88V9c0-2.63 3-3 6-3 3.32 0 6 .38 6 3v6.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADxElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9dx4KX49/kbYJf/HIj/PA8Gfgp4af59/hp4HWCX/3iI/xwvDfwWcJz/GLvA6wB/zX8sxH+89wa+i/94u8DHAN/NfxzEf6zPAj6b/1yfDXwO/zEQ/7LjwIOBv+YFOw58FfDe/Nf4buBjgF1esJcGbgV2ecEQL9xx4LeABwOvA/w1z+s48FvAS/Nf66+B1wF2eV4vDfwWcCvwOsAuzx/iBTsO/Bbw0lyxC7wO8Nc820sDvwUc57/HrcDbAH/Ns7008FvAca74a+B1gF2eF+L5Ow78FvDSPKdd4HWAvwbeGvgu4Dj/vXaB9wF+Gnhp4LeA4zynvwZeB9jlOSGev7cGfornbxf4auCz+Z/ls4GPBo7z/L0N8NM8J8QL9t7Ad/F/w/sA383zQrxw7w18F/+7vQ/w3Tx/iH/ZewPfxf9O7wN8Ny8Y4kXz3sB38b/L+wDfzQuHeNG9N/Bd/O/wPsB38y9D/Ou8N/Bd/M/2PsB386JB/Ou9N/De/M/03cB386JD/P+G+P8N8W/3W/zP8jr86yH+7XaBY/zPcAk4zr8e4t/ut4HX4n+G3wFem389xL/dbwOvxf8MvwO8Nv96iH+73wZei/8Zfgd4bf71EP92nw18Fv8zfA7w2fzrIf7tPhv4LP5n+Bzgs/nXQ/zbfTbwWfzP8DnAZ/Ovh/i3e2/gu/if4W2An+ZfD/Fv99rAb/E/w+sAv82/HuLf7rWB3+J/htcBfpt/PcS/3UsDf8X/DC8D/DX/eoh/H/M/g/i3Qfz7mP8ZxL8N4t/H/M8g/m0Q/z5/DbwU/71+B3ht/m0Q/z6/DbwW/71+B3ht/m0Q/z6/DbwW/71+B3ht/m0Q/z7fDbwX/72+B3hv/m0Q/z6fDXwW/70+B/hs/m0Q/z6fDXwW/70+B/hs/m0Q/z4fDXwV/70+Bvhq/m0Q/z6vDfwW/71eB/ht/m0Q/z6vDfwW/71eB/ht/m0Q/z7HgZfmv9dfA7v82yD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJvW25Bi3FFjQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTakeoutDining;
impl IconShape for MdTakeoutDining {
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
                d: "M5.26,11h13.48l-0.67,9H5.93L5.26,11z M9.02,4h5.95L19,7.38l1.59-1.59L22,7.21 L19.21,10H4.79L2,7.21l1.41-1.41L5,7.38L9.02,4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTaxiAlert;
impl IconShape for MdTaxiAlert {
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
                d: "M23 8A7 7 0 0 0 9.68 5H7v2H4.5a1.5 1.5 0 0 0-1.42 1.01L1 14v8a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-1h12v1a1 1 0 0 0 1 1h1a1 1 0 0 0 1-1v-7.68A7.01 7.01 0 0 0 23 8zm-18.5.5h4.53a6.93 6.93 0 0 0 2.08 4.5H3l1.5-4.5zm0 9.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm11 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm2.93-5.63l-.21.11-.18.09a4.97 4.97 0 0 1-.42.16l-.22.07-.23.06-.2.05a5 5 0 0 1-5.94-4.41A4.07 4.07 0 0 1 11 8l.02-.47.02-.17.04-.28.04-.21.05-.21.07-.24.05-.13a4.99 4.99 0 0 1 9.69 1.7 4.96 4.96 0 0 1-2.55 4.38zM15 4h2v5h-2zm0 6h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+J/nwcBnAe/Dfz7E/zx/Bbw08DHAV/OfC/E/y1cDH8WzvQzw1/znQfzP8dbAT/GcbgVeBtjlPwfif4YHA38FHOd5/TTwNvznQPzP8FfAS/OCfQzw1fzHQ/z3+2rgo/iXvQzw1/zHQvz3emvgp3jR3Aq8DLDLfxzEf58HA38FHOdF99PA2/AfB/Hf56+Al+Zf72OAr+Y/BuI/znFglxfNVwMfxb/dywB/zb8f4j/GceCvgI8BfpoX7q2Bn+Lf51bgZYBd/n0Q/zF+CnhrYBd4GeBWnr8HA38FHOff76eBt+HfB/Hv99HAV/Fsfw28DM/fZwOfxX+cjwG+mn87xL/PSwN/xfP6GuCjef7+Gngp/uO8DPDX/Nsg/u2OA38FPJjn722An+Z5PRj4a+AY/zFuBV4G2OVfD/Fv91PAW/OC7QIvA9zK83pr4Kf4j/PTwNvwr4f4t/lo4Kv4l/018DI8f18NfBT/cT4G+Gr+dRD/ei8N/BUvuq8BPprn76+Bl+I/zssAf82LDvGvcxz4K+DB/Ou8DfDTPK8HA38NHOM/xq3AywC7vGgQ/zo/Bbw1/3q7wMsAt/K83hr4Kf7j/DTwNrxoEC+6jwa+in+7vwZehufvq4GP4j/OxwBfzb8M8aJ5aeCv+Pf7GuCjef7+Gngp/uO8DPDXvHCIf9lx4K+AB/Mf422An+Z5vTXwU/zHuRV4GWCXFwzxL/sp4K35j7MLvAxwK8/2UcBX8x/vp4G34QVDvHAfDXwV//H+GngZ4DjwXcBb85/nY4Cv5vlDvGAvDfwV/3l+Gnhp4MH853sZ4K95Xojn7zjwV8CD+b/hVuBlgF2eE+L5+yngrfm/5aeBt+E5IZ7XRwNfxf9NHwN8Nc+GeE4vDfwV/7e9DPDXXIF4tuPAXwEP5v+2W4GXAXYBxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EaTza0HigazgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTerrain;
impl IconShape for MdTerrain {
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
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTheaterComedy;
impl IconShape for MdTheaterComedy {
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
                d: "M2,16.5C2,19.54,4.46,22,7.5,22s5.5-2.46,5.5-5.5V10H2V16.5z M7.5,18.5C6.12,18.5,5,17.83,5,17h5 C10,17.83,8.88,18.5,7.5,18.5z M10,13c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C9,13.45,9.45,13,10,13z M5,13 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C4,13.45,4.45,13,5,13z",
            }
            path {
                d: "M11,3v6h3v2.5c0-0.83,1.12-1.5,2.5-1.5c1.38,0,2.5,0.67,2.5,1.5h-5V14v0.39c0.75,0.38,1.6,0.61,2.5,0.61 c3.04,0,5.5-2.46,5.5-5.5V3H11z M14,8.08c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C15,7.64,14.55,8.08,14,8.08z M19,8.08 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C20,7.64,19.55,8.08,19,8.08z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFhklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/XSwHcBL81/jr8G3gf4a/5jIP7jPBj4K+A4/7l2gYcAu/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Gie2ngq3jBjgMvzYvmEvDdwF9zxUsD7w0c40Xz18AuL9jHAH/Nvwzxontt4Lf49/se4KOBXZ7TceCrgffi3+91gN/mX4Z40R0HLvLv8zPAW/PC/TbwWvz7nAB2+Zch/nX+Gngp/u0eAtzKC/dg4On82/0N8NK8aBD/Oh8NfBX/Nn8DvDQvmr8GXop/m48BvpoXDeJf5zhwK3CMf73fAV6bF81vA6/Fv94l4MHALi8axL/eWwM/xb/e7wCvzYvmt4HX4l/vbYCf5kWH+Lf5aOCr+Nc7Aezywh0HLvKv9zHAV/Ovg/i3+2jgq/jX+Rzgs3nhPhv4LP51Pgb4av71EP8+rw18NvBavOjeB/hunr/3Br6LF93vAB8N/DX/Noj/GC8NfDTwXrxovhv4GuCvueK1gPcG3psXzfcAXw38Nf8+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7IV50x4GX4gV7aeCredE9A7iVKx4MPIgX3UcDf80L9jfALv8yxIvutYHf4t/vEvDewE/znN4a+G7gGP9+rwP8Nv8yxIvupYG/4t/nEvBgYJfn7zhwK3CMf5+XAf6afxniX2cXOMa/3dsAP80L99bAT/Fv9wzgwbxoEP863w28F/82zwAezIvmVuBB/Nt8D/DevGgQ/zoPBp7Ov83vAK/Ni+a3gdfi3+YhwK28aBD/ep8NfBb/er8DvDYvmt8GXot/vc8BPpsXHeLf5q+Bl+Jf51bgIbxong48mH+dvwFemn8dxL/NceC3gZfiX+dtgJ/mhXtr4Kf41/kd4K2BXf51EP8+nw18Fi+6XeAhwC7P33Hg6cBxXnSfA3w2/zaIf78HAx8NvDXwIP5lu8D7AD/Nc3pr4LuA4/zLngH8NPDVwK382yH+47w28Fu86G4FbuWKBwMP5kX3OsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4kX32sBv8R/je4CvBv6aK14a+GjgvfiP8TrAb/MvQ7zoXhv4Lf793gf4bp6/9wa+i3+/1wF+m38Z4l/H/Pt8DvDZvHCfDXwW/z7iRYP41/lp4K34tzsB7PLCHQcu8m/3M8Bb86JB/Ou8NfBT/Nv8DvDavGh+G3gt/m1eB/htXjSIf73fBl6Lf73fAV6bF81vA6/Fv97vAK/Niw7xr/dg4K+BY/zr/DXwMrxo/gp4af51LgEvDdzKiw7xb/PSwG8Dx/jXeQhwKy/cg4Gn869zCXht4K/510H827008N3AS/Gi+23gdXjhfgt4bV50fwO8N/DX/Osh/v0+Gvho4EG8aL4b+Bhgl+d0HPgq4L150TwD+Grgq/m3Q/zHeW/gu3jR7ALfDfw1V7w08N7AcV407wN8N/9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHOQ7cChzjP9cl4MHALv9+iP9YLw18N/BS/Of4G+C9gb/mPwbi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjdO79BP6AO+AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTraffic;
impl IconShape for MdTraffic {
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
                d: "M20 10h-3V8.86c1.72-.45 3-2 3-3.86h-3V4c0-.55-.45-1-1-1H8c-.55 0-1 .45-1 1v1H4c0 1.86 1.28 3.41 3 3.86V10H4c0 1.86 1.28 3.41 3 3.86V15H4c0 1.86 1.28 3.41 3 3.86V20c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-1.14c1.72-.45 3-2 3-3.86h-3v-1.14c1.72-.45 3-2 3-3.86zm-8 9c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2 0-1.11.89-2 2-2 1.1 0 2 .89 2 2 0 1.1-.89 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTrain;
impl IconShape for MdTrain {
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
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h2.23l2-2H14l2 2h2v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-7H6V6h5v4zm2 0V6h5v4h-5zm3.5 7c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/cV4a+Cr+a3wM8Nf8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+c48NL863wX8GCe088AX80L99fALv9+iP8+rw38Fs/rIcCt/NdA/Pf5LeC1eU7fA7w3/3UQ/z1eG/gtntdDgFv5r4P4z/Vg4EHAg4EH82xvDbw0z2kX+GqebRf4a674Hf5zIP5jPRh4K+CtgZcGjvMf56+B3wZ+Bvht/mMg/mO8NvBZwGvzX+NW4KuBr+HfB/Hvcxz4KuC9+e/x28DbALv82yD+fX4KeGv+e/018DrALv96iH+7zwY+i/8Zvgd4b/71EP82x4GnA8f5n+MEsMu/DuLf5r2B7+J/lvcBvpt/HcS/zWcDn8X/LJ8DfDb/Ooh/m58G3or/Wb4HeG/+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC2e0+8Ar81/jd8GXovn9DvAa/Ovg/i3+W3gtXhOvwO8Nv81fht4LZ7T7wCvzb8O4t/mt4HX4jn9DvDa/Nf4beC1eE6/A7w2/zqIf5vfBl6L5/Q7wGvzX+O3gdfiOf0O8Nr86yD+bX4beC2e0+8Ar81/jd8GXovn9DvAa/Ovg/i3+W3gtXhOvwO8Nv81fht4LZ7T7wCvzb8O4t/mt4HX4jn9DvDa/Nf4beC1eE6/A7w2/zqIf5vfBl6L5/Q7wGvzX+O3gdfiOf0O8Nr86yD+bX4beC2e0+8Ar81/jd8GXovn9DvAa/Ovg/i3+W3gtXhOvwO8Nv81fht4LZ7T7wCvzb8O4t/mt4HX4jn9DvDa/Nf4beC1eE6/A7w2/zqIf5vfBl6L5/Q7wGvzX+O3gdfiOf0O8Nr86yD+bX4beC2e0+8Ar81/jd8GXovn9DvAa/Ovg/i3+W3gtXhOu8Bf81/jpYHjPKffAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/n2+B/hrrnhp4L349/kd4LX510H82/w28Fr8270O8Ns8p9cGfot/u98BXpt/HcS/zW8Dr8W/zecAn83z99nAZ/Fv8zvAa/Ovg/i3+W3gtfi3eRngr3n+Xhr4K/5tfgd4bf51EP823w28F/824oUz/zY/A7w1/zqIf5vPBj6Lf5vXAX6b5++1gd/i3+ZzgM/mXwfxb/PawG/xb/PbwOvw/P0W8Nr827wP8N386yD+7f4aeCn+bb4b+BhglyuOA18FvDf/dg8BbuVfB/Fv99LAX/Hv89tc8dr8+/wO8Nr86yH+fd4b+C7++70M8Nf86yH+/d4a+G7gGP893gf4bv5tEP8xjgMfDbw38CD+a/wO8NnAb/Nvh/iP9dLAbwHHeV6XgPcGdvmXvTTwVTx/HwP8NHAr/36I/zgvDfwWcJzndQl4beCvedG9N/BdPK9d4HWAv+bfD/Ef46WB3wKO87wuAa8N/DX/eu8NfBfPaxd4HeCv+fdB/Pu9NPBbwHGe1yXgtYG/5t/uvYHv4nntAq8D/DX/doh/v4vAcZ7XJeC1gb/m3++9ge/iee0CJ/i3Q/z7/TbP30cDf81/nPcG3pvn9dr82yH+f0P8/4b4/w3x/xvi/zf+ESq0uEHjp0c6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTram;
impl IconShape for MdTram {
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
                d: "M19 16.94V8.5c0-2.79-2.61-3.4-6.01-3.49l.76-1.51H17V2H7v1.5h4.75l-.76 1.52C7.86 5.11 5 5.73 5 8.5v8.44c0 1.45 1.19 2.66 2.59 2.97L6 21.5v.5h2.23l2-2H14l2 2h2v-.5L16.5 20h-.08c1.69 0 2.58-1.37 2.58-3.06zm-7 1.56c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5-4.5H7V9h10v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGsUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/PA8GPgt4a+A4cCvw3cDXALv8z4D4z/HewHfx/P018DbArfz3Q/zHezDwdF64vwZehv9+iP943w28F/+ytwF+mv9eiP94F4Hj/Ms+B/hs/nsh/uOZF83PAG/Nfy/Ef7xbgQfxL/sc4LP574X4j/fZwGfxL3sIcCv/vRD/8Y4Dvw28FC/Y1wAfzX8/xH+OBwM/DbwUz+trgI/mfwbEf663Bl4aeGngr4HvBm7lfw7E/2+I/zivDbwX8NbAceCvgZ8GvgbY5X8mxL/PceC9gI8GHszzdyvwOsCt/M+D+Ld5aeCjgPfmRfPXwMvwPw/iRXcceCvgo4GX5l/vY4Cv5n8WxL/swcBnAW8NHOffbhd4CLDL/xyIF+y9gPcGXpv/ON8DvDf/cyCev98GXov/HK8D/Db/MyCev98GXot/2SXgGP86twIvA+zy3w/x/P028Fo8f5eArwa+G/gp4KX51/tu4H3474d4/n4beC2e0yXgo4Hv5tnMv913A+/Dfy/E8/fbwGvxnH4HeG2e7bWB3+Lf56eB9wF2+e+BeP5+G3gtntPvAK/Ns3008FX8++0CHw18D//1EM/fbwOvxXP6HeC1ebbvBt6LF837AB8NvBQv2K3AVwM/A9zKfw3E8/fbwGvxnH4HeG2e7beB1+JF8xBgF/ht4KX4l+0Cf80Vr8N/HsTz99vAa/Gcfgd4bZ7NvGguAce54jjw3cBb8aIT/3kQz99vA6/Fc/od4LW54qWBv+JF8zvAa/OcPhr4bOAY/zLxnwfx/P028Fo8p98BXpsr3hv4Ll40nwN8Ns/rwcBnA+/FCyf+8yCev98GXovn9DvAa3PFVwMfxYvmbYCf5gV7MPDewFsDL8XzEv95EM/fVwMvzXP6a+CjueK3gdfiRfMQ4FZeNA8GHgy8Nle8NPDW/OdB/NuYF534nwvxr/fSwF/xovtu4H34j3Ec2OU/DuJf772B7+Jf57uB9+Hf5zjwW8DL8B8H8a/32cBn8a/33cD78G9zHPgt4KUB8R8H8a/328Br8W/z3cD78K9zHPgt4KW5QvzHQfzrXQSO87zeBvhu4Bgv3HcD78OL5jjwW8BL82ziPw7iX+fBwNN5XpeA48BLA78NHOOFex3gt3nhjgO/Bbw0z+m3+bf5G+CjeU6If523Bn6K5/U7wGtzxUsDvw0c4wV7HeC3ecGOA78FvDT/cX4HeG2eE+Jf57OBz+J5fQ7w2Tzbg4GfBl6K5+91gN/mBfts4LP4j/U7wGvznBD/Or8NvBbP63WA3+Y5HQd+G3gpntfrAL/NC/fdwHvxH+d3gNfmOSH+dS4Cx3leJ4Bdntdx4CLP63WA3+Zf9t3Ae/GcXod/m13gr3lOiBfdg4Gn87z+BnhpXjDzvF4H+G1eNN8NvBfPJv7jIF50bw38FM/re4D35gUzz+t1gN/mRffdwHtxhfiPg3jRfTbwWTyv9wG+mxfMPK/XAX6bf53vBt4LEC/cdwHvw4sG8aL7aeCteF4vA/w1L5h5Xq8D/Db/et8NvDcvnIHvBt6HfxniRfd04ME8L/HCmef1OsBv85/DXPHdwPvwwiFeNMeBizyv3wFemxfOPK/XAX6b/xzm2b4beB9eMMSL5rWB3+J5fQ7w2bxw5nm9DvDbvGAvDRzj3+a3eU7fDbwPzx/iRfPZwGfxvN4G+GleOPO8Xgf4bV6w3wZei/84XwN8NM8L8aL5aeCteF4ngF1eOPO8Xgf4bV6w3wZei/9YbwP8NM8J8aJ5OvBgntMzgAfzLzPP63WA3+YF+23gtfiP8zfAawO7PCfEv+w4cJHn9T3Ae/MvM8/rdYDf5gX7auCl+bd5LZ7T3wCvDezyvBD/stcGfovn9THAV/MvM8/rdYDf5j+Heba/AV4b2OX5Q/zLPhv4LJ7X6wC/zb/MPK/XAX6b/xzmir8BXhvY5QVD/Mu+G3gvnpd40Zjn9TrAb/Ofw8DfAK8N7PLCIf5lfwW8NM/pd4DX5kVjntfrAL/Nf46/Bl4b2OVfhviXvTbPaxf4a140r83z+mtgl/8cx4FdXjSI/98Q/7/xj86e/UH7dSWiAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTransferWithinAStation;
impl IconShape for MdTransferWithinAStation {
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
                d: "M16.49 15.5v-1.75L14 16.25l2.49 2.5V17H22v-1.5zm3.02 4.25H14v1.5h5.51V23L22 20.5 19.51 18zM9.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5.75 8.9L3 23h2.1l1.75-8L9 17v6h2v-7.55L8.95 13.4l.6-3C10.85 12 12.8 13 15 13v-2c-1.85 0-3.45-1-4.35-2.45l-.95-1.6C9.35 6.35 8.7 6 8 6c-.25 0-.5.05-.75.15L2 8.3V13h2V9.65l1.75-.75",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADKklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+N/hOPBTwMcAf81/HMT/fMeB3wJeGtgFXgf4a/5jIP5nOw78FvDSPNsu8DrAX/Pvh/if68HATwEvzfPaBV4H+Gv+fRD/M7008FvAcV6wXeB1gL/m3w7x7/dbvHCvw7/OSwO/BRznX/Y7wGvzb4f49zMvnHjRvTTwW8Bx/mV/A7w2sMu/HeLfz7xw4kXz0sBvAcf5l/0N8NrALv8+iH8/88KJf9lLA78FHOdf9jfAawO7/Psh/v3MCydeuJcGfgs4zr/sb4DXBnb5j4H49zMvnHjBXhr4LeA4/7JnAC8N7PIfB/HvZ1448fy9NvBTwHH+ZZeA1wb+mv9YiH8/88KJ5/XewHfxorkEvDbw1/zHQ/z7mRdOPKf3Br6LF80l4LWBv+Y/B+Lfz7xw4tneG/guXjSXgNcG/pr/PIh/P/PCiSveG/guXjSXgNcG/pr/XIh/P/PCCXhv4Lt40VwCXhv4a/7zIf79zAv3PsB38aK5BLw28Nf810D8+5n/GJeA1wb+mv86iH8/8+93CXht4K/5r4X49zP/PpeA1wb+mv96iH8/8293CXht4K/574H49zP/NpeA1wb+mv8+iH8/8693CXht4K/574X49zP/NX4HeG3+YyH+/cx/jd8BXpv/WIh/P/Nf43eA1+Y/FuLfz/zX+B3gtfmPhfj3M/81fgd4bf5jIf79zH+N3wFem/9YiH8/81/jd4DX5j8W4t/P/Nf4HeC1+Y+F+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AnksY0FmrswyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTransitEnterexit;
impl IconShape for MdTransitEnterexit {
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
                d: "M16 18H6V8h3v4.77L15.98 6 18 8.03 11.15 15H16v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTripOrigin;
impl IconShape for MdTripOrigin {
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
                d: "M2 12C2 6.48 6.48 2 12 2s10 4.48 10 10-4.48 10-10 10S2 17.52 2 12zm10 6c3.31 0 6-2.69 6-6s-2.69-6-6-6-6 2.69-6 6 2.69 6 6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTwoWheeler;
impl IconShape for MdTwoWheeler {
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
                d: "M20,11c-0.18,0-0.36,0.03-0.53,0.05L17.41,9H20V6l-3.72,1.86L13.41,5H9v2h3.59l2,2H11l-4,2L5,9H0v2h4c-2.21,0-4,1.79-4,4 c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4l2,2h3l3.49-6.1l1.01,1.01C16.59,12.64,16,13.75,16,15c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4 C24,12.79,22.21,11,20,11z M4,17c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2C6,16.1,5.1,17,4,17z M20,17c-1.1,0-2-0.9-2-2 c0-1.1,0.9-2,2-2s2,0.9,2,2C22,16.1,21.1,17,20,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVolunteerActivism;
impl IconShape for MdVolunteerActivism {
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
                height: "11",
                width: "4",
                x: "1",
                y: "11",
            }
            path {
                d: "M16,3.25C16.65,2.49,17.66,2,18.7,2C20.55,2,22,3.45,22,5.3c0,2.27-2.91,4.9-6,7.7c-3.09-2.81-6-5.44-6-7.7 C10,3.45,11.45,2,13.3,2C14.34,2,15.35,2.49,16,3.25z",
            }
            path {
                d: "M20,17h-7l-2.09-0.73l0.33-0.94L13,16h2.82c0.65,0,1.18-0.53,1.18-1.18v0c0-0.49-0.31-0.93-0.77-1.11L8.97,11H7v9.02 L14,22l8.01-3v0C22,17.9,21.11,17,20,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfzzyvjwH+mv9YLw18Fc9L/Nsh/v3M83od4Lf5j/XawG/xvMS/HeLfzzyv1wF+m/9Yrw38Fs9L/Nsh/v3M83od4Lf5j/XawG/xvMS/HeLfzzyv1wF+m/9Yrw38Fs9L/Nsh/v3M83od4Lf5j/XawG/xvMS/HeLfzzyv1wF+m/9Yrw38Fs9L/Nsh/v3M83od4Lf5j/XawG/xvMS/HeLfzzyvvwZ2+Y91HHhpnpf4t0P8+5n/XuLfDvHvZ/57iX87xL+f+e8l/u0Q/363Ag/iv8ffAC/Nvx3i3++3gdfiv8fvAK/Nvx3i3++rgY/iv8fXAB/Nvx3i3++tgZ/iv8fbAD/Nvx3iP8YucIz/WpeA4/z7IP5jfDXwUfzX+hrgo/n3QfzHeDDwdP5rPQS4lX8fxH+czwY+i/8anwN8Nv9+iP9Yfw28FP+5/gZ4af5jIP5jPRj4a+AY/zkuAS8N3Mp/DMR/vJcGfhs4xn+sS8BrA3/NfxzEf44HAz8NvBT/MX4HeG/gVv5jIf5zfTbwWfz7fA7w2fznQPznezDw3sBHA8d40VwCvhr4buBW/vMg/mu9NfDawHsDx3hOl4DvBn4a+G3+ayD+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiP8dLA8d4wb4aeGme018DH80Ldgn4a/5jIf5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwI0FmpBScfuNQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWineBar;
impl IconShape for MdWineBar {
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
                d: "M6,3l0,6c0,2.97,2.16,5.43,5,5.91V19H8v2h8v-2h-3v-4.09c2.84-0.48,5-2.94,5-5.91l0-6H6z M16,8H8l0-3h8C16,5,16,8,16,8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWrongLocation;
impl IconShape for MdWrongLocation {
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
                d: "M14,10V3.26C13.35,3.09,12.68,3,12,3c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 c0-0.41-0.04-0.81-0.09-1.2H14z M12,13c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C14,12.1,13.1,13,12,13z",
            }
            polygon {
                points: "22.54,2.88 21.12,1.46 19,3.59 16.88,1.46 15.46,2.88 17.59,5 15.46,7.12 16.88,8.54 19,6.41 21.12,8.54 22.54,7.12 20.41,5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5l/u/cBvpt/n/cGvot/O/GCIf5l5t/mfYDv5j/GewPfxb+NeMEQ/zLzr/c+wHfzH+u9ge/iX0+8YIh/mfnXeR/gu/nP8d7Ad/GvI14wxL/MvOjeB/hu/nO9N/BdvOjEC4b4l5kXzfsA381/jfcGvosXjXjBEP8y8y/bBV4H+Gv+a7w08FvAcf5l4gVD/MvMi2YXeB3gr/nP9dLAbwHHedGIFwzxLzMvul3gdYC/5j/HewPfxb+OeMEQ/zLz/F0CjvH8vQ/w3fzHem/gu3j+LgHHeP7EC4b4l5nn9T7AXwO/DRzj+Xsf4Lv5j/HewHfx/F0CXht4aeC7eF7iBUP8y8xzeh/gu7nipYHfBo7x/L0P8N38+7w38F08f5eA1wb+miveG/gunpN4wRD/MvNs7wN8N8/pvYHv4gV7H+C7+bd5b+C7eP4uAa8N/DXP6b2B7+LZxAuG+JeZK94H+G6ev/cGvosX7G2An+Zf562Bn+IFexvgp3n+3hv4Lq4QLxjiX2bgfYDv5oV7b+C7eF5/A7w2sMu/znHgt4GX4nm9D/DdvHDvDXwXIF4wxL/svYHv5kXz3sB38Wx/A7w2sMu/zXHgt4GX4tneB/huXjTvDXw3LxjiP957A98F/A3w2sAu/z7Hgd8GXgp4H+C7+Y+D+M/x1sBvA7v8xzgOvDbw0/zHQvz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEf463Bn4b2OU/xnHgtYGf5j8W4j/eewPfBfw18DrALv8+x4HfAl4aeB/gu/mPg/iXfRfwPrxo3hv4Lp7tr4HXAXb5tzkO/Bbw0jzb+wDfzYvmu4D34QVD/MsMfDfwPrxw7w18F8/rr4HXAXb51zkO/Bbw0jyv9wG+mxfuu4D3BsQLhviXmSu+G3gfnr/3Br6LF+xtgJ/mX+etgZ/iBXsf4Lt5/r4LeG+uEC8Y4l9mnu27gffhOb038F28YO8DfDf/Nu8NfBfP3y7wOsBf85y+C3hvnk28YIh/mXlO3w28D1e8NPBbwHGev/cBvpt/n/cGvovnbxd4HeCvueK7gPfmOYkXDPEvM8/ru4GvAX4LOM7z9z7Ad/Mf472B7+L52wVeB/go4L15XuIFQ/zLzL/OJeCtgd/mP9Z7A9/Fv554wRD/MvOiuwS8NvDX/Od4beCngWO86MQLhviXmRfNJeC1gb/mP9dLA78NHONFI14wxL/MvGheBvhr/mu8NPBXvGjEC4b4l5kXzXcD78N/je8C3psXjXjBEP8y86L7buB9+M/1XcB786ITLxjiX2b+db4beB/+c3wX8N7864gXDPEvM/963w28D/+xvgt4b/71xAuG+JeZf5vvBt6H/xjfBbw3/zbiBUP8y8y/3XcD78O/z3cB782/nXjBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHog+uQQ8wUFoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdZoomOutMap;
impl IconShape for MdZoomOutMap {
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
                d: "M15,3l2.3,2.3l-2.89,2.87l1.42,1.42L18.7,6.7L21,9V3H15z M3,9l2.3-2.3l2.87,2.89l1.42-1.42L6.7,5.3L9,3H3V9z M9,21 l-2.3-2.3l2.89-2.87l-1.42-1.42L5.3,17.3L3,15v6H9z M21,15l-2.3,2.3l-2.87-2.89l-1.42,1.42l2.89,2.87L15,21h6V15z",
            }
        }
    }
}
