use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md6FtApart;
impl IconShape for Md6FtApart {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6,6c1.1,0,2-0.9,2-2S7.1,2,6,2S4,2.9,4,4S4.9,6,6,6z M10,9.43c0-0.81-0.48-1.53-1.22-1.85C7.93,7.21,6.99,7,6,7 C5.01,7,4.07,7.21,3.22,7.58C2.48,7.9,2,8.62,2,9.43V10h8V9.43z M18,6c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S16.9,6,18,6z M22,9.43 c0-0.81-0.48-1.53-1.22-1.85C19.93,7.21,18.99,7,18,7c-0.99,0-1.93,0.21-2.78,0.58C14.48,7.9,14,8.62,14,9.43V10h8V9.43z M19,17 v-2.01L5,15v2l-3-3l3-3v2.01L19,13v-2l3,3L19,17z M10,19v-1H7.5C7.22,18,7,18.22,7,18.5v3C7,21.78,7.22,22,7.5,22h2 c0.28,0,0.5-0.22,0.5-0.5V20c0-0.28-0.22-0.5-0.5-0.5H8V19H10z M9,20.5V21H8v-0.5H9z M17.5,19h-1v3h-1v-3h-1v-1h3V19z M12.5,19v0.5 h1v1h-1V22h-1v-4H14v1H12.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/14O54lb+eyD+e7w28F7Ae3PFdwPfA/w2/7UQ/7XeC3hv4LV5/n4b+G7ge/ivgfjPdxx4L+CjgQfzorkV+Grge4Bd/vMg/vM8GPgo4L2B4/zb7ALfDXwNcCv/8RD/8V4beC/gvfmP9d3A9wC/zX8cxH+c9wLeG3ht/nP9NvDdwPfw74f49zkOvBfw0cCD+a91K/DVwPcAu/zbIP5tHgx8FPDewHH+e+0C3w18DXAr/zqIf53XBt4LeG/+Z/pu4HuA3+ZFg3jRvBfw3sBr87/DbwPfDXwPLxziX/Z04MH873Qr8BBeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87/UM4MG8YIh/2V8DL8X/Tr8DvDYvGOJf9tvAa/G/088Ab80LhviXfTbwWfzP8zs8pwcDD+I5fQ7w2bxgiH/ZewPfxX+/nwF+GvhpYJfn7zjw1sBrA+8FvA3w07xgiH/ZSwN/xX+f7wE+G7iVf50HA7vALi8Y4kWzCxzjv9bfAG8N3Mp/HsSL5qeBt+K/zvcAHw3s8p8L8aJ5b+C7+K/xPcB7818D8aI5DlzkP9/vAK/Nv+y3gdfiOf0O8Nr86yBedN8NvBf/eZ4BvDSwy7/st4HX4jn9DvDa/OsgXnSvDfwW/3neBvhpXjS/DbwWz+l3gNfmXwfxr/PbwGvxH+93gNfmRffbwGvxnH4HeG3+dRD/Om8N/BT/8V4H+G1edL8NvBbP6XeA1+ZfB/Gv99vAa/Ef5xJwnH+d3wZei+f0O8Br86+D+Nd7beC3+I/zPcB786/z28Br8Zx+B3ht/nUQ/zZfDXwU/zHeBvhp/nV+G3gtntPvAK/Nvw7i3+Y48NfAg/j3ex3gt3lOvw28Fv+xfgd4bZ4T4t/utYHf4t/vIcCtPKffBl6L/1i/A7w2zwnx7/PRwFfx7yOe128Dr8V/rN8BXpvnhPj3+27gvfi3E8/rt4HX4j/W7wCvzXNC/Mf4buC9+Ld5GeCveU6/DbwW/7F+B3htnhPiP8Zx4KeB1+Jf73WA3+Y5vTRwnBfsq4GX4jn9DfDRvGC7wF/znBD/sb4beC/+dT4H+Gz+dX4beC2e0+8Ar82/DuI/3mcDn8WL7q+Bl+Ff57eB1+I5/Q7w2vzrIP5zvDXw3cAxXjQPAW7lRffbwGvxnH4HeG3+dRD/eY4D3w28Ff+y7wHemxfdbwOvxXP6HeC1+ddB/Od7beC7gQfxgu0CLwPcyovmt4HX4jn9DvDa/Osg/uu8N/DRwEvx/P028Dq8aH4beC2e0+8Ar82/DuK/3ksDHw28NvAgrngG8NPAZwO7/Mt+G3gtntPvAK/Nvw7iv9dLc8Vf86/z0sBxntMu8Nf86yD+f0P8/8Y/AlCprUHjZ/lcAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddModerator;
impl IconShape for MdAddModerator {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
            }
            path {
                d: "M13.22 22.61c-.4.15-.8.29-1.22.39-5.16-1.26-9-6.45-9-12V5l9-4 9 4v6c0 .9-.11 1.78-.3 2.65-.81-.41-1.73-.65-2.7-.65-3.31 0-6 2.69-6 6 0 1.36.46 2.61 1.22 3.61zM19 20v2.99s-1.99.01-2 0V20h-3v-2h3v-3h2v3h3v2h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP9arw18FfDSPH9/DXwM8Nv810D813lv4Lt40bwP8N3850P813hr4Kf413kb4Kf5z4X4z3cceDpwnH+dXeAhwC7/eRD/+T4a+Cqev9/hitfi+fsY4Kv5z4P4z/fbwGvxnC4Brw38NVe8NPDbwDGe0+8Ar81/HsR/PvO8vgb4aJ7TVwMfxfMS/3kQ//nM83of4Lt5Th8NfBXPS/znQfznuxV4EM/pp4G34Tn9FPDWPKdnAA/mPw/iP99PA2/F8/ps4Hu44r2Az+Z5fQ/w3vznQfzne2/gu/i3eR/gu/nPg/iv8dfAS/Gv8zvAa/OfC/Ff46WB3waO8aK5BLw0cCv/uRD/dV4a+GngQbxwzwDeGvhr/vMh/msdBz4aeG/gQTynZwDfDXw1sMt/DcR/vs8Gfhv4bZ7TSwPHuWIX+Gue02sDrw18Nv95EP/5LgLHga8GPgfY5YU7DnwW8NHALnCC/zyI/1xvDfwUz7YL/DTwOcCtPKcHA58FvDVwnGd7G+Cn+c+B+M/13cB78ZwuAQ8GdnlOx4GLPK/vAd6b/xyI/zzHgYs8r+8B3pvn77uB9+J5nQB2+Y+H+M/z3sB38bzeBvhpnr+3Bn6K5/U+wHfzHw/xn+e3gNfmOT0DeDAv3C5wjOf028Dr8B8P8Z/jwcDTeV5fA3w0L9xXAx/F83oIcCv/sRD/OT4a+Cqe18sAf80L99LAX/G8Pgb4av5jIf5zPB14MM/pb4CX5kVzK/AgntOtwEP4j4X4j/fSwF/xvD4G+GpeNB8NfBXP62WAv+Y/DuI/3lcDH8XzeghwKy+aBwNP53l9DfDR/MdB/Me7CBznOf0M8Nb86/w08FY8p13gBP9xEP+x3hr4KZ7X+wDfzb/OewPfxfN6G+Cn+Y+B+I/13cB78ZwuAQ8GdvnXOQ7cChzjOX0P8N78x0D8xzkOXOR5fQ/w3vzbfDfwXjyvE8Au/36I/zjvDXwXz+ttgJ/m3+atgZ/ieb0P8N38+yH+4/wW8No8p2cAD+bf51bgQTyn3wZeh38/xH+MBwNP53l9DfDR/Pt8NfBRPK+HALfy74P4j/HRwFfxvF4G+Gv+fV4a+Cue1+cAn82/D+I/xtOBB/Oc/gZ4af5j/DXwUjynW4GH8O+D+I9hntfHAF/Nf4yPBr6K5yX+fRD/MW4FHsRzeghwK/8xHgw8ned0CTjOvw/iP8ZvA6/Fs70P8N38x3pv4Lt4tt8BXpt/H8R/jJ8G3gq4BLw38NP853hv4KuBY8DPAG/Nvw/iP8ZnAx8NvDbw1/znemngt4GvBj6bfx/Ef4yPBn4b+Gv+a7w08NrAV/Pvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AoK8q0EJQj+jAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArchitecture;
impl IconShape for MdArchitecture {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.36,18.78L6.61,21l1.62-1.54l2.77-7.6c-0.68-0.17-1.28-0.51-1.77-0.98L6.36,18.78z",
            }
            path {
                d: "M14.77,10.88c-0.49,0.47-1.1,0.81-1.77,0.98l2.77,7.6L17.39,21l0.26-2.22L14.77,10.88z",
            }
            path {
                d: "M15,8c0-1.3-0.84-2.4-2-2.82V3h-2v2.18C9.84,5.6,9,6.7,9,8c0,1.66,1.34,3,3,3S15,9.66,15,8z M12,9c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C13,8.55,12.55,9,12,9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjv9dpc8dv890D893o6VzyE/x6I/z4fDXwVV3wM8NX810P89zgOPB04zhW7wEOAXf5rIf57fDfwXjynrwE+mv9aiP96Lw38Fc/fywB/zX8dxH+93wJem+fvt4HX4b8O4r/WWwM/xQv3NsBP818D8V/r6cCDeeH+GngZ/msg/uu8NPBXvGheBvhr/vMh/uu8NvBbvGheB/ht/vMh/uu8NvBbvGheB/ht/vMh/mvtAsd44S4Bx/mvgfiv9dnAZ/HCfQ7w2fzXQPzX+27gvXj+vgd4b/7rIP57vDfw0cBLccXvAN8NfDf/tRD/vyH+f0P893hp4BjP6RLw1/zXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/3oPBj4KeG3gpfmf4a+B3wa+BriVFx3iX+ergI/mf7avBj6GFw3iRfdXwEvzv8NfAy/Dvwzxovlq4KP43+VrgI/mhUP8yx4MPJ3/nR4C3MoLhviXfTXwUfzv9DXAR/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+de7BHw08NXAMf59LgEfDXw1cIx/PfGCIf5l5l/nEvDawF8DLw38NnCMf5tLwGsDfw28NPDbwDH+dcQLhviXmRfdJeC1gb/m2V4a+G3gGP86l4DXBv6aZ3tp4LeBY7zoxAuG+Jf9NfBS/MsuAa8N/DXP66WB3waO8aL5G+CtgVt5Xi8N/DZwjH/Z7wCvzQuG+Je9N/BdvHCXgNcG/poX7KWB3waO8cL9DfDawC4v2EsDvw0c44V7H+C7ecEQL5rvBt6L5+8S8NrAX/OcjgO7PKeXBn4bOMbz9zfAawO7PKfjwC7P6aWB3waO8fx9D/DevHCIF91nA5/Fc/od4KOBv+Y5vTTwW8DHAN/NczoOfDXwXjyn7wE+GtjlOb038FXA6wB/zXN6aeCrgdfi2S4BXw18Nv8yxL/OceClueJW4Fae10sDvwUc54r3Ab6b53UceGmu+Gtgl+f13sB3ccUu8DrAX/O8Xho4zhW/zYsO8R/rpYHfAo7znN4H+G7+dd4b+C6e0y7wOsBf8x8D8S87DuzyL3tv4KuA4zx/Xw18DC+arwI+mudvF/gY4Lv5lx0HdnnBEP+yi8BXA98D3Mrzeivgo4HX5l92K/DZwO8At/KcHgy8FvDZwIP5l/028NXAz/C8Hgy8F/DRwAleMMS/zDzbrcCtPNtr82+3C/w1V7w0cJx/m13gr3m2BwMP5tnEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviX7QLH+N/pGcCDecEQ/7LPBj6L/50+B/hsXjDEi+atgbcGHsz/DrcCPw38NC8c4v83xP9viP/fEP+/If5/4x8BnMqyQamIwmoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCake;
impl IconShape for MdCake {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 6c1.11 0 2-.9 2-2 0-.38-.1-.73-.29-1.03L12 0l-1.71 2.97c-.19.3-.29.65-.29 1.03 0 1.1.9 2 2 2zm4.6 9.99l-1.07-1.07-1.08 1.07c-1.3 1.3-3.58 1.31-4.89 0l-1.07-1.07-1.09 1.07C6.75 16.64 5.88 17 4.96 17c-.73 0-1.4-.23-1.96-.61V21c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-4.61c-.56.38-1.23.61-1.96.61-.92 0-1.79-.36-2.44-1.01zM18 9h-5V7h-2v2H6c-1.66 0-3 1.34-3 3v1.54c0 1.08.88 1.96 1.96 1.96.52 0 1.02-.2 1.38-.57l2.14-2.13 2.13 2.13c.74.74 2.03.74 2.77 0l2.14-2.13 2.13 2.13c.37.37.86.57 1.38.57 1.08 0 1.96-.88 1.96-1.96V12C21 10.34 19.66 9 18 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCleanHands;
impl IconShape for MdCleanHands {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.99,5l0.63,1.37L18.99,7l-1.37,0.63L16.99,9l-0.63-1.37L14.99,7l1.37-0.63L16.99,5 M11,6.13V4h2 c0.57,0,1.1,0.17,1.55,0.45l1.43-1.43C15.15,2.39,14.13,2,13,2c-1.48,0-5.5,0-5.5,0v2H9v2.14C7.23,6.51,5.81,7.8,5.26,9.5h3.98 L15,11.65v-0.62C15,8.61,13.28,6.59,11,6.13z M1,22h4V11H1V22z M20,17h-7l-2.09-0.73l0.33-0.94L13,16h2.82 c0.65,0,1.18-0.53,1.18-1.18l0,0c0-0.49-0.31-0.93-0.77-1.11L8.97,11H7v9.02L14,22l8-3l0,0C21.99,17.9,21.11,17,20,17z M20,14 c1.1,0,2-0.9,2-2c0-1.1-2-4-2-4s-2,2.9-2,4C18,13.1,18.9,14,20,14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdConnectWithoutContact;
impl IconShape for MdConnectWithoutContact {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11,14H9c0-4.97,4.03-9,9-9v2C14.13,7,11,10.13,11,14z M18,11V9c-2.76,0-5,2.24-5,5h2C15,12.34,16.34,11,18,11z M7,4 c0-1.11-0.89-2-2-2S3,2.89,3,4s0.89,2,2,2S7,5.11,7,4z M11.45,4.5h-2C9.21,5.92,7.99,7,6.5,7h-3C2.67,7,2,7.67,2,8.5V11h6V8.74 C9.86,8.15,11.25,6.51,11.45,4.5z M19,17c1.11,0,2-0.89,2-2s-0.89-2-2-2s-2,0.89-2,2S17.89,17,19,17z M20.5,18h-3 c-1.49,0-2.71-1.08-2.95-2.5h-2c0.2,2.01,1.59,3.65,3.45,4.24V22h6v-2.5C22,18.67,21.33,18,20.5,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdConstruction;
impl IconShape for MdConstruction {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                height: "8.48",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -6.8717 17.6255)",
                width: "3",
                x: "16.34",
                y: "12.87",
            }
            path {
                d: "M17.5,10c1.93,0,3.5-1.57,3.5-3.5c0-0.58-0.16-1.12-0.41-1.6l-2.7,2.7L16.4,6.11l2.7-2.7C18.62,3.16,18.08,3,17.5,3 C15.57,3,14,4.57,14,6.5c0,0.41,0.08,0.8,0.21,1.16l-1.85,1.85l-1.78-1.78l0.71-0.71L9.88,5.61L12,3.49 c-1.17-1.17-3.07-1.17-4.24,0L4.22,7.03l1.41,1.41H2.81L2.1,9.15l3.54,3.54l0.71-0.71V9.15l1.41,1.41l0.71-0.71l1.78,1.78 l-7.41,7.41l2.12,2.12L16.34,9.79C16.7,9.92,17.09,10,17.5,10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCoronavirus;
impl IconShape for MdCoronavirus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.25,10.5c-0.41,0-0.75,0.34-0.75,0.75h-1.54c-0.15-1.37-0.69-2.63-1.52-3.65l1.09-1.09l0.01,0.01 c0.29,0.29,0.77,0.29,1.06,0s0.29-0.77,0-1.06L18.54,4.4c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.76-0.01,1.05l-1.09,1.09 c-1.02-0.82-2.27-1.36-3.64-1.51V3.5h0.01c0.41,0,0.75-0.34,0.75-0.75C13.5,2.34,13.16,2,12.75,2h-1.5c-0.41,0-0.75,0.34-0.75,0.75 c0,0.41,0.33,0.74,0.74,0.75v1.55C9.87,5.19,8.62,5.74,7.6,6.56L6.51,5.47l0.01-0.01c0.29-0.29,0.29-0.77,0-1.06 c-0.29-0.29-0.77-0.29-1.06,0L4.4,5.46c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.76,0.29,1.05,0.01l1.09,1.09 c-0.82,1.02-1.36,2.26-1.5,3.63H3.5c0-0.41-0.34-0.75-0.75-0.75C2.34,10.5,2,10.84,2,11.25v1.5c0,0.41,0.34,0.75,0.75,0.75 c0.41,0,0.75-0.34,0.75-0.75h1.54c0.15,1.37,0.69,2.61,1.5,3.63l-1.09,1.09c-0.29-0.29-0.76-0.28-1.05,0.01 c-0.29,0.29-0.29,0.77,0,1.06l1.06,1.06c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06l-0.01-0.01l1.09-1.09 c1.02,0.82,2.26,1.36,3.63,1.51v1.55c-0.41,0.01-0.74,0.34-0.74,0.75c0,0.41,0.34,0.75,0.75,0.75h1.5c0.41,0,0.75-0.34,0.75-0.75 c0-0.41-0.34-0.75-0.75-0.75h-0.01v-1.54c1.37-0.14,2.62-0.69,3.64-1.51l1.09,1.09c-0.29,0.29-0.28,0.76,0.01,1.05 c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06c0.29-0.29,0.29-0.77,0-1.06c-0.29-0.29-0.77-0.29-1.06,0l-0.01,0.01l-1.09-1.09 c0.82-1.02,1.37-2.27,1.52-3.65h1.54c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75v-1.5C22,10.84,21.66,10.5,21.25,10.5z M13.75,8c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S13.2,8,13.75,8z M12,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C13,12.55,12.55,13,12,13z M10.25,8c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S9.7,8,10.25,8z M8.5,13c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C9.5,12.55,9.05,13,8.5,13z M10.25,16c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C11.25,15.55,10.8,16,10.25,16z M13.75,16c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14.75,15.55,14.3,16,13.75,16z M14.5,12 c0-0.55,0.45-1,1-1s1,0.45,1,1c0,0.55-0.45,1-1,1S14.5,12.55,14.5,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeck;
impl IconShape for MdDeck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                points: "22,9 12,2 2,9 11,9 11,22 13,22 13,9",
            }
            polygon {
                points: "4.14,12 2.18,12.37 3,16.74 3,22 5,22 5.02,18 7,18 7,22 9,22 9,16 4.9,16",
            }
            polygon {
                points: "19.1,16 15,16 15,22 17,22 17,18 18.98,18 19,22 21,22 21,16.74 21.82,12.37 19.86,12",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDomain;
impl IconShape for MdDomain {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,7V3H2v18h20V7H12z M6,19H4v-2h2V19z M6,15H4v-2h2V15z M6,11H4V9h2V11z M6,7H4V5h2V7z M10,19H8v-2h2V19z M10,15H8v-2h2 V15z M10,11H8V9h2V11z M10,7H8V5h2V7z M20,19h-8v-2h2v-2h-2v-2h2v-2h-2V9h8V19z M18,11h-2v2h2V11z M18,15h-2v2h2V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdElderly;
impl IconShape for MdElderly {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S12.4,5.5,13.5,5.5z M20,12.5V23h-1V12.5c0-0.28-0.22-0.5-0.5-0.5 S18,12.22,18,12.5v1h-1v-0.69c-1.46-0.38-2.7-1.29-3.51-2.52C13.18,11.16,13,12.07,13,13c0,0.23,0.02,0.46,0.03,0.69L15,16.5V23h-2 v-5l-1.78-2.54L11,19l-3,4l-1.6-1.2L9,18.33V13c0-1.15,0.18-2.29,0.5-3.39L8,10.46V14H6V9.3l5.4-3.07l0,0.01 c0.59-0.31,1.32-0.33,1.94,0.03c0.36,0.21,0.63,0.51,0.8,0.85l0,0l0.79,1.67C15.58,10.1,16.94,11,18.5,11C19.33,11,20,11.67,20,12.5 z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w2sAv8NVfsAn8N/A2wy38+xH+utwJeG3hr4MH86/w18NvAzwC/zX8OxH+848BHAe8NPJj/GLcC3w18DbDLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jvBfw1cBx/mvsAu8D/DT/Poh/n+PAdwFvzX+PnwbeB9jl3wbxb/fSwE8BD+a/163A2wB/zb8e4t/mpYHfAo7zP8Mu8DrAX/Ovg/jXe2ngt4Dj/M+yC7wO8Ne86BD/Oi8N/BZwnP+ZdoHXAf6aFw3iRXcc+CvgwfzPdivwMsAu/zLEi+6ngLfmf4efBt6GfxniRfPewHfxv8v7AN/NC4f4lx0Hng4c53+XXeAhwC4vGOJf9tnAZ/G/0+cAn80LhnjhjgNPB47zv9Mu8BBgl+cP8cJ9NvBZ/OtcAr4b+GvgwcB7Aw/i3+ZvgJ8GbgVeGnhv4Bj/Op8DfDbPH+KFezrwYF50fwO8NXArz+m7gffiX+d7gPfmOT0Y+GngpXjR3Qo8hOcP8YK9NfBT/Ou8DPDXPH+3Ag/iRfM3wEvz/L008Ff867wO8Ns8L8QL9tXAR/Gi+xvgpXnBPhv4LF40nwN8Ni/YXwMvxYvua4CP5nkhXrC/Al6aF93vAK/NC/bRwFfxonkf4Lt5wX4beC1edH8NvAzPC/H8PRh4Ov86twIP4QX7auCjeNF8DfDRvGAXgeP865wAdnlOiOfvtYHf4l/vY4Cv5nk9GPgr4Dgvml3gZYBbeV4fDXwV/3qvA/w2zwnx/H008FX86+0Cnw18Dc/20sB3AS/Nv85fA+8D/DXP9lHAZwPH+df7GOCreU6I5++zgc/i324X+GvgOPDS/Pv8NbALvDRwnH+7zwE+m+eEeP4+G/gs/m/5HOCzeU6I5++3gdfi/5afAd6a54R4/n4beC3+b/kZ4K15Tojn77OBz+L/ls8BPpvnhHj+Phv4LP5v+Rzgs3lOiOfvo4Gv4v+WjwG+mueEeP5eG/gt/m95HeC3eU6I5+/BwNP517kVeB/+a3wX8GD+dU4AuzwnxAv218BL8a/zEOBW/nM9GHg6/zp/A7w0zwvxgn018FH863wN8NH85/pq4KP41/ka4KN5XogX7K2Bn+Jf7yHArfzneDDwdP71Xgf4bZ4X4oW7FXgQ/zo/DbwN/zl+C3ht/nWeATyY5w/xwn028Fn8630M8NX8x/po4Kv41/sc4LN5/hAv3HHgVuAY/3rvA3w3/zHeG/gu/vUuAQ8Gdnn+EP+yzwY+i3+b9wG+m3+f9wa+i3+bzwE+mxcM8S87DtwKHOPf5rOBz+Hf5rOAz+bf5hLwYGCXFwzxonlv4Lv4t7sV+Gzge3jRvBfw2cCD+bd7G+CneeEQL7qfBt6Kf59bgZ8Gfht4BvDXXPHSwIOA1wbeGngw/z4/A7w1/zLEi+448NfAg/if7RnASwO7/MsQ/zovDfw2cIz/mS4Brw38NS8axL/eSwO/DRzjf5ZLwGsDf82LDvFv89LAbwPH+J/hEvDawF/zr4P4t3tp4KeBB/Hf6xnAWwN/zb8e4t/nOPDdwFvx3+N7gI8Gdvm3QfzHeG/gq4Fj/Nd4BvDRwE/z74P4j3Mc+Gjgo4Fj/Oe4BHw18NXALv9+iP94x4GPBt4beBD/MZ4BfDfw1cAu/3EQ/7leG3hr4LWBl+Jf52+A3wZ+Gvht/nMg/us8GHgw8NLAca54aa74a67YBf4a+Gtgl/98iP/fEP+/If5/Q/z/hvj/jX8EG6PVQS8oXVQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiEmotions;
impl IconShape for MdEmojiEmotions {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.99,2C6.47,2,2,6.48,2,12c0,5.52,4.47,10,9.99,10C17.52,22,22,17.52,22,12C22,6.48,17.52,2,11.99,2z M8.5,8 C9.33,8,10,8.67,10,9.5S9.33,11,8.5,11S7,10.33,7,9.5S7.67,8,8.5,8z M12,18c-2.28,0-4.22-1.66-5-4h10C16.22,16.34,14.28,18,12,18z M15.5,11c-0.83,0-1.5-0.67-1.5-1.5S14.67,8,15.5,8S17,8.67,17,9.5S16.33,11,15.5,11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEuElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/zpvBbw08No8r+PAS/Nf46+BXZ7XbwN/DfwMLxrEi+bBwE8BL83/Dn8NvA1wKy8c4l92HPgr4MH87/LXwOsAu7xgiH/ZZwOfxf9OnwN8Ni8Y4l/2dODB/O90K/AQXjDEv8w8r78BdnlOx4GX4r/G3wC7PKfjwEvxvMQLhviXmef1OsBv85xeG/gt/mu8DvDbPKfXBn6L5yVeMMS/zDyv1wF+m+f02sBv8V/jdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i/8arwP8Ns/prYGf4nmJFwzxL9sFjvGcPgf4bJ7TawO/xX+N1wF+m+f02cBn8ZyeATyYFwzxL/tt4LV4Tt8DvDfP6bWB3+K/xusAv81z+mngrXhOvwO8Ni8Y4l/21cBH8Zx2gYcAuzzbawO/xX+N1wF+m2c7DjwdOM5z+hrgo3nBEP+ytwZ+iuf1OcBn82yvDfwW/zVeB/htnu2rgY/ieb0N8NO8YIgXza3Ag3hebwP8NFe8NvBb/Nd4HeC3ueKtgZ/ieT0DeDAvHOJF897Ad/H8vQ/w3cBrA7/Ff43XAX4beG/gu3j+3gf4bl44xIvur4GX4vn7aeCvgc/mv8ZnAy8NvDXP398AL82/DPGie2ngt4Fj/M92CXht4K/5lyH+dd4a+G7gGP8zXQLeG/hpXjSIf72XBn4bOMb/LJeA1wb+mhcd4t/mOPDdwFvxP8PPAO8N7PKvg/j3eW3gvYH34r/H9wBfDfw1/zaI/zivDbw18FH85/oc4LeB3+bfD/Ef77eB1+I/x+8Ar81/HMR/vNcGfov/HK8D/Db/cRD/Ob4a+Cj+Y30O8Nn8x0L85/lu4L34j/E9wHvzHw/xn+u7gffi3+drgI/mPwfiP99bA98NHONf5xnARwM/zX8exH+N48B7Ax8NPIgX7hnAVwPfDezynwvxX++lge8GXorn9DfAewN/zX8dxH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/cY4DL8WL5quBl+Y5/TXw0bxo/gbY5d8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPpC6ZQcAN9OQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiEvents;
impl IconShape for MdEmojiEvents {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19,5h-2V3H7v2H5C3.9,5,3,5.9,3,7v1c0,2.55,1.92,4.63,4.39,4.94c0.63,1.5,1.98,2.63,3.61,2.96V19H7v2h10v-2h-4v-3.1 c1.63-0.33,2.98-1.46,3.61-2.96C19.08,12.63,21,10.55,21,8V7C21,5.9,20.1,5,19,5z M5,8V7h2v3.82C5.84,10.4,5,9.3,5,8z M19,8 c0,1.3-0.84,2.4-2,2.82V7h2V8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJemiv+GvgeYJf/XRD/eu8NfBVwnOe0C3wM8N3874H413lt4Ld44V4H+G3+d0D86zwdeDAv3K3AQ/jfAfGie2ngr3jRvAzw1/zPh3jRvTbwW7xoXgf4bf7nQ7zoXhv4LV40rwP8Nv/zIV50x4FbgWO8cJeA4/zvgPjX+Wzgs3jhPgf4bP53QPzrfTfwXjx/3wO8N/97IP5tfht4LZ7T7wCvzf8uiH+b3wZei+f0O8Br878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE5/DXw0/3aXgL/mvxbi3+a3gdfiP95DgFv5r4P4t/lt4LX4j/c9wHvzXwfxb/PbwGvxn+MhwK3810D82/w28Fr85/ge4L35r4H4t/lt4LX4z/MQ4Fb+8yH+bX4beC2e0+8Ar82/3m8Dr8V/rNcBfpt/GeLf5reB1+I5/Q7w2vzrvTbwW/zHeh3gt/mXIf5tfht4LZ7T7wCvzb/NbwOvxX+c1wF+m38Z4t/mt4HX4jn9DvDa/Nu8NvBb/Md5HeC3+Zch/m1+G3gtntPvAK/Nv91vA6/Ff4zXAX6bfxni3+a3gdfiOf0O8Nr82z0YeDD/er/F83od4Lf5lyH+bX4beC2e0+8Ar81/PfO8Xgf4bf5liH+b3wZei+f0O8Br81/PPK/XAX6bfxni3+a3gdfiOf0O8Nr81zPP63WA3+Zfhvi3+W3gtXhOvwO8Nv/1zPN6HeC3+Zch/m1+G3gtntPvAK/Nfz3zvF4H+G3+ZYh/m98GXovn9DvAa/Nfzzyv1wF+m38Z4t/mt4HX4jn9DvDa/Nczz+t1gN/mX4b4t/lt4LV4Tr8DvDb/9czzeh3gt/mXIf5tfht4LZ7T7wCvzX8987xeB/ht/mWIf5vfBl6L5/Q7wGvzX888r9cBfpt/GeLf5reB1+I/1q3AQ/jXM8/rdYDf5l+G+Lf5beC1+I/3PsB3869jntfrAL/Nvwzxb/PbwGvxH+9W4CH865jn9TrAb/MvQ/zb/DbwWvzneB/gu3nRmef1OsBv8y9D/Nv8NvBa/Oe4FXgILzrzvF4H+G3+ZYh/m98GXovn9DfAR/Ov993Ag3hOfw3s8qJ5bZ7X6wC/zb8M8W/z28Br8Zx+B3ht/vXeG/gu/mO9DvDb/MsQ/za/DbwWz+l3gNfm3+ZW4EH8x3kd4Lf5lyH+bX4beC2e0+8Ar82/zXsD38V/nNcBfpt/GeLf5reB1+I5/Q7w2vzb3Qo8iP8YrwP8Nv8yxL/NbwOvxXP6HeC1+bd7a+Cj+Y/x0cBf8y9D/Nv8NvBaPKffAV6b/10Q/za/DbwWz+l3gNfmfxfEv81vA6/Fc/od4LX53wXxb/PbwGvxnH4HeG3+d0H82/w28Fo8p98BXpv/XRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RxAPokHb4gAlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiFlags;
impl IconShape for MdEmojiFlags {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,9l-1-2H7V5.72C7.6,5.38,8,4.74,8,4c0-1.1-0.9-2-2-2S4,2.9,4,4c0,0.74,0.4,1.38,1,1.72V21h2v-4h5l1,2h7V9H14z M18,17h-4 l-1-2H7V9h5l1,2h5V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEjklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fu8aF4H+G2uMP89bgW+G/gaYJfnD/Gie23gt3jRvA7w21xh/nvdCrwOcCvPC/Gie23gt3jRvA7w21xh/vv9NfAyPC/Ei+61gd/iRfM6wG9zhfmf4W2An+Y5IV50rw38Fi+a1wF+myvM/wyfA3w2zwnxontt4Ld40bwO8NtcYf5zvQ7P66uBl+I5/Q7w2jwnxIvutYHf4l92CXhp4Fau+GvgpfjPI57XbwOvxXP6HeC1eU6IF91rA7/FC/c3wFsDt/Kcvht4L/5ziOf128Br8Zx+B3htnhPiRffawG/xgn0P8NHALs/fewPfxX888bx+G3gtntPvAK/Nc0K86F4b+C2ev48Bvpp/2UsDvw0c4z+OeF6/DbwWz+l3gNfmOSFedK8N/BbP632A7+ZFdxy4FTjGfwzxvH4beC2e0+8Ar81zQrzoXhv4LZ6X+Nf7beC1+I8hntdvA6/Fc/od4LV5TogX3WsDv8XzEv96vw28Fv8xxPP6beC1eE6/A7w2zwnxontt4Ld4XuJf77eB1+I/xm/zvF4aOM5z+hngrXlOiBfdawO/xfMS/3q/DbwW/7U+B/hsnhPiRffawG/xvMS/3m8Dr8V/rYcAt/KcEC+61wZ+i+cl/vV+G3gt/ut8DfDRPC/Ei+61gd/ieYl/vd8GXov/Gl8DfDTPH+JF99rAb/G8xL/ebwOvxX+M3+F57QJ/Dfw08Ne8YIgX3WsDv8XzEv96vw28Fv8xxL8d4kX32sBv8bzEv95vA6/Ffwzxb4d40b028Fs8L/Gv99vAa/EfQ/zbIV50rw38Fs9L/Ov9NvBa/McQ/3aIF91rA7/F83of4Lt50T0Y+CvgOP8xxL8d4kX32sBv8T+P+LdDvOheG/gt/ucR/3aIF91rA7/F/zzi3w7xontt4Lf4n+UScJx/O8SL7qWBv+J/lt8BXpt/O8S/zq3Ag/if42uAj+bfDvGv893Ae/E/x+sAv82/HeJf58HA0/mf4XeA1+bfB/Gv99XAR/Hf73WA3+bfB/Fv89fAS/Hf52OAr+bfD/Fvcxz4beCl+K/3OcBn8x8D8e/z1cBH8V/jGcBHAz/NfxzEv9+Dgc8GXht4EP/x/gb4buC7gV3+YyH+Y700cJz/OH8N7PKfB/H/G+L/N8T/b4j/3xAvuuPAS/G/w98Au/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFN6JQUTl7hUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiFoodBeverage;
impl IconShape for MdEmojiFoodBeverage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,3H9v2.4l1.81,1.45C10.93,6.94,11,7.09,11,7.24v4.26c0,0.28-0.22,0.5-0.5,0.5h-4C6.22,12,6,11.78,6,11.5V7.24 c0-0.15,0.07-0.3,0.19-0.39L8,5.4V3H4v10c0,2.21,1.79,4,4,4h6c2.21,0,4-1.79,4-4v-3h2c1.11,0,2-0.9,2-2V5C22,3.89,21.11,3,20,3z M20,8h-2V5h2V8z",
            }
            rect {
                height: "2",
                width: "16",
                x: "4",
                y: "19",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiNature;
impl IconShape for MdEmojiNature {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.94,4.88C21.76,4.35,21.25,4,20.68,4c-0.03,0-0.06,0-0.09,0H19.6l-0.31-0.97C19.15,2.43,18.61,2,18,2h0 c-0.61,0-1.15,0.43-1.29,1.04L16.4,4h-0.98c-0.03,0-0.06,0-0.09,0c-0.57,0-1.08,0.35-1.26,0.88c-0.19,0.56,0.04,1.17,0.56,1.48 l0.87,0.52L15.1,8.12c-0.23,0.58-0.04,1.25,0.45,1.62C15.78,9.91,16.06,10,16.33,10c0.31,0,0.61-0.11,0.86-0.32L18,8.98l0.81,0.7 C19.06,9.89,19.36,10,19.67,10c0.27,0,0.55-0.09,0.78-0.26c0.5-0.37,0.68-1.04,0.45-1.62l-0.39-1.24l0.87-0.52 C21.89,6.05,22.12,5.44,21.94,4.88z M18,7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C19,6.55,18.55,7,18,7z",
            }
            path {
                d: "M13.49,10.51c-0.43-0.43-0.94-0.73-1.49-0.93V8h-1v1.38c-0.11-0.01-0.23-0.03-0.34-0.03c-1.02,0-2.05,0.39-2.83,1.17 c-0.16,0.16-0.3,0.34-0.43,0.53L6,10.52c-1.56-0.55-3.28,0.27-3.83,1.82c0,0,0,0,0,0c-0.27,0.75-0.23,1.57,0.12,2.29 c0.23,0.48,0.58,0.87,1,1.16c-0.38,1.35-0.06,2.85,1,3.91c1.06,1.06,2.57,1.38,3.91,1c0.29,0.42,0.68,0.77,1.16,1 C9.78,21.9,10.21,22,10.65,22c0.34,0,0.68-0.06,1.01-0.17c0,0,0,0,0,0c1.56-0.55,2.38-2.27,1.82-3.85l-0.52-1.37 c0.18-0.13,0.36-0.27,0.53-0.43c0.87-0.87,1.24-2.04,1.14-3.17H16v-1h-1.59C14.22,11.46,13.92,10.95,13.49,10.51z M4.67,14.29 c-0.25-0.09-0.45-0.27-0.57-0.51s-0.13-0.51-0.04-0.76c0.19-0.52,0.76-0.79,1.26-0.61l3.16,1.19C7.33,14.2,5.85,14.71,4.67,14.29z M10.99,19.94c-0.25,0.09-0.52,0.08-0.76-0.04c-0.24-0.11-0.42-0.32-0.51-0.57c-0.42-1.18,0.09-2.65,0.7-3.8l1.18,3.13 C11.78,19.18,11.51,19.76,10.99,19.94z M12.2,14.6l-0.61-1.61c0-0.01-0.01-0.02-0.02-0.03c-0.02-0.04-0.04-0.08-0.06-0.12 c-0.02-0.04-0.04-0.07-0.07-0.11c-0.03-0.03-0.06-0.06-0.09-0.09c-0.03-0.03-0.06-0.06-0.09-0.09c-0.03-0.03-0.07-0.05-0.11-0.07 c-0.04-0.02-0.07-0.05-0.12-0.06c-0.01,0-0.02-0.01-0.03-0.02L9.4,11.8c0.36-0.29,0.79-0.46,1.26-0.46c0.53,0,1.04,0.21,1.41,0.59 C12.8,12.66,12.84,13.81,12.2,14.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFF0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/Ng4K2A1waOA6/Ns/01sAvsAj8N/Aywy38+xH+u48BbAZ8NPJh/nb8Gvhr4Hv7zIP7zfBTw2cBx/n1uBT4G+Gn+4yH+4z0Y+C7gtfmP9dvA2wC7/MdB/Md6aeC3gOP857gVeBvgr/mPgfiP89bAdwHH+c+1C7wO8Nf8+yH+Y7w08FvAcf5r7AIvA9zKvw/i3+848HTgOP+1/hp4HWCXfzvEv99PAW/Nf4/vAd6bfzvEv89rA7/Ff6+HALfyb4P49/kt4LX57/XbwOvwb4P4t3tt4Lf4n+FlgL/mXw/xb/fdwHvxP8PXAB/Nvx7i3+4icJz/GW4FHsK/HuLf5qWBv+J/locAt/Kvg/i3+Wjgq/jXewbwIF64ZwAP4l/vbYCf5l8H8W/z2cBn8a/zN8BrA28NfBfP3/sAPw38NvBS/Ot8DvDZ/Osg/m2+G3gvXnS7wEOAXa54b+C7eE7vA3w3VxwHng4c50X3OcBn86+D+Lf5beC1+Nf5buB9eLb3Br6LK94H+G6e7buA9+Zf52eAt+ZfB/Fv89PAW/Gv993A+/Bs780V382zfRfw3vzrfQ3w0fzrIP5tPhv4LP5tvht4H56/7wLem3+bzwE+m38dxL/NRwNfxb/ddwPvw3P6LuC9+bf7GOCr+ddB/Nu8NvBb/Pt8DfDRXPHVwEfx7/M6wG/zr4P4tzP/Pr8DvDZX/DbwWvz7iH89xL/dTwNvxb/d7wCvzRW/DbwW/3Y/A7w1/3qIf7v3Br6Lf7vfAV6bK34beC3+7d4H+G7+9RD/PrcCD+Lf5neA1+aK3wZei3+bZwAP5t8G8e/z0cBX8W/zO8Brc8VvA6/Fv837AN/Nvw3i3++vgZfiX+93gNfmit8GXot/vb8BXpp/O8S/30sDf8W/3i7w11zx0sBx/vVeBvhr/u0Q/zHeG/gu/mu9D/Dd/Psg/uN8N/Be/Nf4GOCr+fdD/Mf6buC9+M/1McBX8x8D8R/vp4G34j/H9wDvzX8cxH+848BFntcl4K950bw0cIzndQLY5T8O4j/HXwMvxXP6HeC1edH8NvBaPKffAV6b/1iI/xy/DbwWz+l3gNfmRfPbwGvxnH4HeG3+YyH+c/w28Fo8p13gr3nRvDRwnOf0O8Br8x8L8Z/jt4HX4j/W7wCvzX8sxH+O3wZei/9YvwO8Nv+xEP85fht4LZ7T7wCvzYvmt4HX4jn9DvDa/MdC/Of4beC1eE6/A7w2L5rfBl6L5/Q7wGvzHwvxn+O3gdfiOf018NG8aL4aeGme0+8Ar81/LMR/js8GPov/WJ8DfDb/sRD/Od4a+Cn+Y70N8NP8x0L85/lr4KX4j/E3wEvzHw/xn+fBwE8DL8W/z98Abw3cyn88xH+u48BHA+8NPIh/nWcA3w18NbDLfw7Ef62XBv6KF+5lgL/mvwbiv95bA98NHOM5XQLeG/hp/usg/nscB94beDBX3Ap8N7DLfy3E/2+I/98Q/78h/n9D/P/GPwI5FKlBAvKWGAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiObjects;
impl IconShape for MdEmojiObjects {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,3c-0.46,0-0.93,0.04-1.4,0.14C7.84,3.67,5.64,5.9,5.12,8.66c-0.48,2.61,0.48,5.01,2.22,6.56C7.77,15.6,8,16.13,8,16.69 V19c0,1.1,0.9,2,2,2h0.28c0.35,0.6,0.98,1,1.72,1s1.38-0.4,1.72-1H14c1.1,0,2-0.9,2-2v-2.31c0-0.55,0.22-1.09,0.64-1.46 C18.09,13.95,19,12.08,19,10C19,6.13,15.87,3,12,3z M14,19h-4v-1h4V19z M14,17h-4v-1h4V17z M12.5,11.41V14h-1v-2.59L9.67,9.59 l0.71-0.71L12,10.5l1.62-1.62l0.71,0.71L12.5,11.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE50lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAvupcGvorn9Tr86x0H3gt4aa74a+B7gF3+ayFedK8N/BbP6yHArbzo3hv4KuA4z2kX+Bjgu/mvg3jRvTbwWzyv1wF+mxfNWwM/xQv3OsBv818D8a9jntfHAF/Ni+bpwIN54W4FHsJ/DcS/zl8DL8Vz+hngrfmXvTTwV7xoXgb4a/7zIf51vhr4KJ7XCWCXF+61gd/iRfM6wG/znw/xr/PawG/xvL4HeG9euNcGfosXzesAv81/PsS/3q3Ag3heDwFu5YXbBY7xwl0CjvNfA/Gv997Ad/G8/hp4HWCXF+yzgc/ihfsc4LP5r4H4t7kVeBDP66eBt+GF+27gvXj+vgd4b/7rIP5tXhv4LZ6/3wbeBtjlBXtv4KOBl+KK3wG+G/hu/msh/u2+Gvgonr9bgfcBfpv/2RD/Pj8NvBUv2G8DXw38DP8zIf59jgO/DbwUL9wu8NPArVzx27xwzwBu5T8f4t/vOPDVwHvxH2sX+Grga4Bd/nMg/uN8NvBZ/MfbBV4H+Gv+4yH+Yz0Y+G7gtfiPtQu8DvDX/MdC/Od4beC9gffiP84u8DrAX/MfB/Gf6zhwkf84u8DrAH/NfwzEfz7zH2sXeB3gr/n3Q/znM//xdoHXAf6afx/Efz7zn2MXeB3gr/m3Q/znM/8+7wN8F8/fLvA6wF/zb4P4z2f+fQS8N/BdPH+7wOsAf82/HuI/n/n3EVe8N/BdPH+7wOsAf82/DuI/n/n3Ec/23sB38fztAq8D/DUvOsR/PvPvI57TewPfxfO3C7wO8Ne8aBD/+cy/j3he7w18F8/fLvA6wF/zL0P85zP/PuL5e2/gu3j+doGHALu8cIj/fObfR7xg7w18F8/rfYDv5l+G+M9n/n3EC/fewHfxbO8DfDcvGsR/PvPvI/5l7w18F/A+wHfzokP85zP/PuJF82DgVv51EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/GfzzyvvwF2eU7HgZfieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP99v87w+GvhrntNLA1/N83pt/vMg/n9D/P+G+P8N8f8b4v83/hEQjLNBtT6PCgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiPeople;
impl IconShape for MdEmojiPeople {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                d: "M15.89,8.11C15.5,7.72,14.83,7,13.53,7c-0.21,0-1.42,0-2.54,0C8.24,6.99,6,4.75,6,2H4c0,3.16,2.11,5.84,5,6.71V22h2v-6h2 v6h2V10.05L18.95,14l1.41-1.41L15.89,8.11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiSymbols;
impl IconShape for MdEmojiSymbols {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                height: "2",
                width: "8",
                x: "3",
                y: "2",
            }
            polygon {
                points: "6,11 8,11 8,7 11,7 11,5 3,5 3,7 6,7",
            }
            rect {
                height: "2",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -7.0416 16.9999)",
                width: "11",
                x: "11.5",
                y: "16",
            }
            circle {
                cx: "14.5",
                cy: "14.5",
                r: "1.5",
            }
            circle {
                cx: "19.5",
                cy: "19.5",
                r: "1.5",
            }
            path {
                d: "M15.5,11c1.38,0,2.5-1.12,2.5-2.5V4h3V2h-4v4.51C16.58,6.19,16.07,6,15.5,6C14.12,6,13,7.12,13,8.5 C13,9.88,14.12,11,15.5,11z",
            }
            path {
                d: "M9.74,15.96l-1.41,1.41l-0.71-0.71l0.35-0.35c0.98-0.98,0.98-2.56,0-3.54c-0.49-0.49-1.13-0.73-1.77-0.73 c-0.64,0-1.28,0.24-1.77,0.73c-0.98,0.98-0.98,2.56,0,3.54l0.35,0.35l-1.06,1.06c-0.98,0.98-0.98,2.56,0,3.54 C4.22,21.76,4.86,22,5.5,22s1.28-0.24,1.77-0.73l1.06-1.06l1.41,1.41l1.41-1.41l-1.41-1.41l1.41-1.41L9.74,15.96z M5.85,14.2 c0.12-0.12,0.26-0.15,0.35-0.15s0.23,0.03,0.35,0.15c0.19,0.2,0.19,0.51,0,0.71l-0.35,0.35L5.85,14.9 C5.66,14.71,5.66,14.39,5.85,14.2z M5.85,19.85C5.73,19.97,5.59,20,5.5,20s-0.23-0.03-0.35-0.15c-0.19-0.19-0.19-0.51,0-0.71 l1.06-1.06l0.71,0.71L5.85,19.85z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmojiTransportation;
impl IconShape for MdEmojiTransportation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.57,10.66C20.43,10.26,20.05,10,19.6,10h-7.19c-0.46,0-0.83,0.26-0.98,0.66L10,14.77l0.01,5.51 c0,0.38,0.31,0.72,0.69,0.72h0.62C11.7,21,12,20.62,12,20.24V19h8v1.24c0,0.38,0.31,0.76,0.69,0.76h0.61 c0.38,0,0.69-0.34,0.69-0.72L22,18.91v-4.14L20.57,10.66z M12.41,11h7.19l1.03,3h-9.25L12.41,11z M12,17c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S12.55,17,12,17z M20,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S20.55,17,20,17z",
            }
            polygon {
                points: "14,9 15,9 15,3 7,3 7,8 2,8 2,21 3,21 3,9 8,9 8,4 14,4",
            }
            rect {
                height: "2",
                width: "2",
                x: "5",
                y: "11",
            }
            rect {
                height: "2",
                width: "2",
                x: "10",
                y: "5",
            }
            rect {
                height: "2",
                width: "2",
                x: "5",
                y: "15",
            }
            rect {
                height: "2",
                width: "2",
                x: "5",
                y: "19",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEngineering;
impl IconShape for MdEngineering {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9,15c-2.67,0-8,1.34-8,4v2h16v-2C17,16.34,11.67,15,9,15z",
            }
            path {
                d: "M22.1,6.84c0.01-0.11,0.02-0.22,0.02-0.34c0-0.12-0.01-0.23-0.03-0.34l0.74-0.58c0.07-0.05,0.08-0.15,0.04-0.22l-0.7-1.21 c-0.04-0.08-0.14-0.1-0.21-0.08L21.1,4.42c-0.18-0.14-0.38-0.25-0.59-0.34l-0.13-0.93C20.36,3.06,20.29,3,20.2,3h-1.4 c-0.09,0-0.16,0.06-0.17,0.15L18.5,4.08c-0.21,0.09-0.41,0.21-0.59,0.34l-0.87-0.35c-0.08-0.03-0.17,0-0.21,0.08l-0.7,1.21 c-0.04,0.08-0.03,0.17,0.04,0.22l0.74,0.58c-0.02,0.11-0.03,0.23-0.03,0.34c0,0.11,0.01,0.23,0.03,0.34l-0.74,0.58 c-0.07,0.05-0.08,0.15-0.04,0.22l0.7,1.21c0.04,0.08,0.14,0.1,0.21,0.08l0.87-0.35c0.18,0.14,0.38,0.25,0.59,0.34l0.13,0.93 C18.64,9.94,18.71,10,18.8,10h1.4c0.09,0,0.16-0.06,0.17-0.15l0.13-0.93c0.21-0.09,0.41-0.21,0.59-0.34l0.87,0.35 c0.08,0.03,0.17,0,0.21-0.08l0.7-1.21c0.04-0.08,0.03-0.17-0.04-0.22L22.1,6.84z M19.5,7.75c-0.69,0-1.25-0.56-1.25-1.25 s0.56-1.25,1.25-1.25s1.25,0.56,1.25,1.25S20.19,7.75,19.5,7.75z",
            }
            path {
                d: "M19.92,11.68l-0.5-0.87c-0.03-0.06-0.1-0.08-0.15-0.06l-0.62,0.25c-0.13-0.1-0.27-0.18-0.42-0.24l-0.09-0.66 C18.12,10.04,18.06,10,18,10h-1c-0.06,0-0.11,0.04-0.12,0.11l-0.09,0.66c-0.15,0.06-0.29,0.15-0.42,0.24l-0.62-0.25 c-0.06-0.02-0.12,0-0.15,0.06l-0.5,0.87c-0.03,0.06-0.02,0.12,0.03,0.16l0.53,0.41c-0.01,0.08-0.02,0.16-0.02,0.24 c0,0.08,0.01,0.17,0.02,0.24l-0.53,0.41c-0.05,0.04-0.06,0.11-0.03,0.16l0.5,0.87c0.03,0.06,0.1,0.08,0.15,0.06l0.62-0.25 c0.13,0.1,0.27,0.18,0.42,0.24l0.09,0.66C16.89,14.96,16.94,15,17,15h1c0.06,0,0.12-0.04,0.12-0.11l0.09-0.66 c0.15-0.06,0.29-0.15,0.42-0.24l0.62,0.25c0.06,0.02,0.12,0,0.15-0.06l0.5-0.87c0.03-0.06,0.02-0.12-0.03-0.16l-0.52-0.41 c0.01-0.08,0.02-0.16,0.02-0.24c0-0.08-0.01-0.17-0.02-0.24l0.53-0.41C19.93,11.81,19.94,11.74,19.92,11.68z M17.5,13.33 c-0.46,0-0.83-0.38-0.83-0.83c0-0.46,0.38-0.83,0.83-0.83s0.83,0.38,0.83,0.83C18.33,12.96,17.96,13.33,17.5,13.33z",
            }
            path {
                d: "M4.74,9h8.53c0.27,0,0.49-0.22,0.49-0.49V8.49c0-0.27-0.22-0.49-0.49-0.49H13c0-1.48-0.81-2.75-2-3.45V5.5 C11,5.78,10.78,6,10.5,6S10,5.78,10,5.5V4.14C9.68,4.06,9.35,4,9,4S8.32,4.06,8,4.14V5.5C8,5.78,7.78,6,7.5,6S7,5.78,7,5.5V4.55 C5.81,5.25,5,6.52,5,8H4.74C4.47,8,4.25,8.22,4.25,8.49v0.03C4.25,8.78,4.47,9,4.74,9z",
            }
            path {
                d: "M9,13c1.86,0,3.41-1.28,3.86-3H5.14C5.59,11.72,7.14,13,9,13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFacebook;
impl IconShape for MdFacebook {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,4.84,3.44,8.87,8,9.8V15H8v-3h2V9.5C10,7.57,11.57,6,13.5,6H16v3h-2 c-0.55,0-1,0.45-1,1v2h3v3h-3v6.95C18.05,21.45,22,17.19,22,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFireplace;
impl IconShape for MdFireplace {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2,2v20h20V2H2z M11.86,16.96c0.76-0.24,1.4-1.04,1.53-1.63c0.13-0.56-0.1-1.05-0.2-1.6c-0.08-0.46-0.07-0.85,0.08-1.28 c0.54,1.21,2.15,1.64,1.98,3.18C15.06,17.33,13.14,18.01,11.86,16.96z M20,20h-2v-2h-2.02c0.63-0.84,1.02-1.87,1.02-3 c0-1.89-1.09-2.85-1.85-3.37C12.2,9.61,13,7,13,7c-6.73,3.57-6.02,7.47-6,8c0.03,0.96,0.49,2.07,1.23,3H6v2H4V4h16V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFollowTheSigns;
impl IconShape for MdFollowTheSigns {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S8.4,5.5,9.5,5.5z M5.75,8.9L3,23h2.1l1.75-8L9,17v6h2v-7.55L8.95,13.4 l0.6-3C10.85,12,12.8,13,15,13v-2c-1.85,0-3.45-1-4.35-2.45L9.7,6.95C9.35,6.35,8.7,6,8,6C7.75,6,7.5,6.05,7.25,6.15L2,8.3V13h2 V9.65L5.75,8.9 M13,2v7h3.75v14h1.5V9H22V2H13z M18.01,8V6.25H14.5v-1.5h3.51V3l2.49,2.5L18.01,8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3EeDLwVcBx4aeCvgb8GfgfY5d/mOPBWwIOBlwb+GtgFfga4lX8/xL/fceCrgPfm+dsFvhr4HP51Pgv4aOA4z993Ax8D7PJvh/j3eWngt4Dj/Mt+GngfYJcX7jjwU8Br8y/bBV4H+Gv+bRD/dseBpwPHedF9N/A+vHDfBbw3L7pd4CHALv96iH+77wbei3+91wF+m+fvrYGf4l/ve4D35l8P8W/zYODp/Nv8NfAyPH9/Bbw0/zYPAW7lXwfxb/PRwFfxb3cC2OU5PRh4Ov927wN8N/86iH+brwY+in+71wF+m+f02sBv8W/3OcBn86+D+Lf5aeCt+Ld7HeC3eU6vDfwW/3ZfA3w0/zqIf5vPBj6Lf7vXAX6b5/TawG/xb/c5wGfzr4P4t3lv4Lv4t7kEHOf5M/92bwP8NP86iH+b48CtwDH+9b4HeG+ev+8G3ot/vUvAcf71EP92nw18Fv96DwFu5fl7MPB0/vU+Bvhq/vUQ/z6/DbwWL7r3Ab6bF+69ge/iRfczwFvzb4P49zkOfDXwXvzL3gf4bl407w18F/+y7wE+Gtjl3wbxH+Otgc8GXorn9T3AZwO38q/zYOCzgffief0N8NHAb/Pvg/iP9WDgwTzbb/Mf47V5tluBW/mPgfj/DfH/G+L/N8T/b4j/OC8NHON5PQO4lX+7lwaO8ZwuAX/Nvx/i3+a1gNcGXhp4aeDB/Mv+Gvhr4LeB3wFu5UXz0sBvA8d4XrcCfw38NfDbwO/wr4N40b0V8NbAWwPH+fe7Ffhp4HuAv+aFezDw08BL8cLtAj8N/DTwM/zLEC/cceCjgPcGHsx/nt8Gvhv4Hl6wlwZ+GzjGi+ZW4LuBrwF2ef4QL9hHAZ8NHOe/zm8DHwP8Nc/fewPfxb/OLvDZwNfwvBDP6zjwU8Br89/no4Gv4fn7a+Cl+Nf7aeB9gF2eDfGcjgO/Bbw0//0+B/hsntd7A9/Fv81fA68D7HIF4jl9NfBR/M/xOsBv87zMv93XAB/NFYhnOw5c5H+WnwHemud1K/Ag/u1OALsA4tleG/gt/ucRz+u3gdfi3+51gN8GEM/22sBv8T+PeF6/DbwW/3avA/w2gHi21wZ+i/95xPP6beC1+Ld7HeC3AcSzvTbwW/zPI57XbwOvxb/d6wC/DSCe7bWB3+J/HvG8fht4Lf7tXgf4bQDxbK8N/Bb/84jn9dvAa/Fv9zrAbwOIZ3tt4Lf4n0c8r98GXot/u9cBfhtAPNtLA1/N/zyvzfP6auCl+bf7aOCvAcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIVe5xBaApKWQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGroup;
impl IconShape for MdGroup {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/72OA28FPBh4aeCvgV3gZ4BbecGOAx8FPJgrbgV+Bvhr/nUQ/30+C/ho4DjP33cDHwPs8ryOA7cCx3hOfw18DPDbvGgQ//WOAz8FvDb/sl3gdYC/5nm9N/BdPH/vA3w3/zLEf73vAt6bF90u8BBgl+e1Cxzj+Xsb4Kd54RD/td4a+Cn+9b4HeG+e13cD78Xztws8BNjlBUP8y16L5/U3wC7/en8FvDT/Ng8BbuU5vTfwXbxgHwN8NS8Y4l9mntfrAL/Nv86Dgafzb/c+wHfznF4b+C1esJ8B3poXDPEvM8/rdYDf5l/ntYHf4t/uc4DP5nmZF+xW4CG8YIh/mXlerwP8Nv86rw38Fv92XwN8NM/LvHDiBUP8y8zzeh3gt/nXeW3gt/i3+xzgs3lODwaezgv2N8BL84Ih/mXmeb0O8Nv865l/u7cBfprn9NrAb/GCfQ/w3rxgiH+ZeV6vA/w2/3rfDbwX/3qXgOM8r48GvooX7HWA3+YFQ/zLzPN6HeC3+dd7MPB0/vU+BvhqntdPA2/F8/c7wGvzwiHgtXjhfpvn9dHAX/OC/Q2wy/P33sB38aL7GeCteV4PBp7O83cJeGngVl44BJj/eK8D/DYv2HsD38W/7HuAjwZ2eV7fDbwXz+tvgPcG/pp/GQLMf7zXAX6bF+7BwGcD78Xz+hvgo4Hf5vl7aeCveE7PAL4b+GpglxcNAsx/vNcBfpsX3WvzbLcCt/Ive2ngOFfsAn/Nvx4CzH+81wF+m//5EGD+470O8Nv8z4f4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV6vA/w2/37HgZfieT0DuJX/fIh/mXlerwP8Nv96rwW8NvDawEsDx3nh/hr4beC3gd8BdvmXvTbwUcBvA78D/DUvGOJfZp7X6wC/zYvmtYH3At4aOM6/z08DPw18Dy/cewPfxRW3At8NfA9wK88J8S8zz+t1gN/mhXtt4LOA1+Y/3q3AdwNfA+zy/H018FE8p+8GPge4lSsQ/zLzvF4H+G2ev+PAVwHvzX++vwbeB/hrntdx4FbgGM9pF/hs4GsAxL/MPK/XAX6b53Uc+C3gpfmvswu8DvDXPK/vBt6L5++7gfcR/7G+C3hv/uvdCrwMsMtzemvgp3jBXkf8xzkOXOS/z/sA381zejDwdF6w3xH/cV4b+C3++3wP8N48L/OCIf7jvDbwW/z3+R3gtXle5gVD/Md5beC3+O/zO8Br87zMC4b4j/PawG/x3+d3gNfmeZkXDPEf57WB3+K/z+8Ar83zMi8Y4j/OawO/xX+f3wFem+dlXjDE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RFMO8OxPKayAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGroupAdd;
impl IconShape for MdGroupAdd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 10H5V7H3v3H0v2h3v3h2v-3h3v-2zm10 1c1.66 0 2.99-1.34 2.99-3S19.66 5 18 5c-.32 0-.63.05-.91.14.57.81.9 1.79.9 2.86s-.34 2.04-.9 2.86c.28.09.59.14.91.14zm-5 0c1.66 0 2.99-1.34 2.99-3S14.66 5 13 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm6.62 2.16c.83.73 1.38 1.66 1.38 2.84v2h3v-2c0-1.54-2.37-2.49-4.38-2.84zM13 13c-2 0-6 1-6 3v2h12v-2c0-2-4-3-6-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEh0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K/3YOCjgNcGXpor/hr4beBrgFv5r4P4r/VVwEfzwn018DH810D81/kr4KV50fw18DL850P81/hq4KP41/ka4KP5z4X4z/dg4On82zwEuJX/PIj/fF8NfBT/Nl8DfDT/eRD/+f4KeGn+bf4aeBn+8yD+85l/H/GfB/HCHQc+Cnhv4MHALvDTwOcAt/KiMf8+4kXzYOCzgLcGjgO3At8NfA2wy/OHeMEeDPwW8GCev/cBvpt/2V8DL8W/zd8AL82/7L2B7+L5+2vgbYBbeV6IF+yvgJfmhXsIcCsv3FcDH8W/zdcAH80L92Dg6bxwfw28DM8L8fy9NfBT/Mu+B3hvXrgHA0/n3+YhwK28cN8NvBf/srcBfprnhHj+Phv4LP5lu8AJ/mVfDXwU/zpfA3w0/7KLwHH+ZZ8DfDbPCfH8/TTwVrxoxIvmr4GX4kXzN8BL86IxL5qfAd6a54R4/j4b+Cz+Zc8AHsyL7quBj+KF+xrgo3nR3Qo8iH/Z5wCfzXNCPH8vDfwV/7LPAT6bf50HAx8NvDbwUlzxN8BvA18N3Mq/zmcDn8W/7CHArTwnxAv21cBH8YL9DfDawC7/vY4Dvw28FC/Y1wAfzfNCvHBfDXwUz+tvgLcGbuV/hgcDPw28FM/ra4CP5vlD/MteGnhr4KWBvwb+Gvhp/nVeGngt4MHASwPHgZfmOf01sAv8NXAr8DvAX/Ov89bASwMvDfw18N3ArbxgiP88bwW8NfDWwHH+bXaBnwZ+GvgZ/uMh/uN9FvDewIP5j3Ur8N3A5/AfB/GCvTTwUsCDgePASwO3ArcCfwP8NM/ppYHvAl6a/1x/DbwP8Nc8p7cGXgp4MPBg4K+BXeBW4G+Av+Z5IZ7TSwMfBbw1cJwX7HWA3+bZjgNPB47zX2MXeAiwy7O9NvBbvGC7wE8DXwP8NVcgrjgOfBfw1vzL/gZ4aZ7TbwGvzX+t3wZeh+f018BL8S/7aeB9gF0Bx4HfAl6aF83HAF/Ns7028Fv893gd4Ld5to8GvooXzV8DLyPgvYHv4kX3OsBv82w/DbwV/z1+Bnhrnu21gd/iRfc+An4beC1edOI5/RXw0vz3+GvgZXhO5kX3OwLMv454Tua/l3hO5kWHAPOvI56T+e8lnpN50SHA/OuI52T+e4nnZF50CDD/OuI5mf9e4jmZFx0CzL+OeE7mv5d4TuZFh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJYtZ5ckCZVYwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGroups;
impl IconShape for MdGroups {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,12.75c1.63,0,3.07,0.39,4.24,0.9c1.08,0.48,1.76,1.56,1.76,2.73L18,18H6l0-1.61c0-1.18,0.68-2.26,1.76-2.73 C8.93,13.14,10.37,12.75,12,12.75z M4,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C2,12.1,2.9,13,4,13z M5.13,14.1 C4.76,14.04,4.39,14,4,14c-0.99,0-1.93,0.21-2.78,0.58C0.48,14.9,0,15.62,0,16.43V18l4.5,0v-1.61C4.5,15.56,4.73,14.78,5.13,14.1z M20,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C18,12.1,18.9,13,20,13z M24,16.43c0-0.81-0.48-1.53-1.22-1.85 C21.93,14.21,20.99,14,20,14c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l4.5,0V16.43z M12,6c1.66,0,3,1.34,3,3 c0,1.66-1.34,3-3,3s-3-1.34-3-3C9,7.34,10.34,6,12,6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFW0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP99r8d/rd3jBEP/5zH8v8YIh/vOZ/17iBUP86xwH3gp4beDBwHHgpbnir4FdYBf4aeB3gFsB899LvGCIF817AR8NvDT/On8NvDT/vcQLhnjhXhv4LuDB/O8lXjDE83cc+C7grfnP8TrAb/Mf67WB3+J5iRcM8bweDPwU8NL853kb4Kf5j/XawG/xvMQLhnhetwIP4oV7BvDTwC6wyxXHgePAWwMP4oXbBV4H+Gv+47w28Fs8L/GCIZ7XSwO/DRzj+fsc4LN54R4MvDfw0cAxnr9d4HWAv+Y/xmsDv8XzEi8Y4vl7aeC3gWM8f+8DfDf/sgcDXw28Fc/fLvA6wF/z7/fawG/xvMQLhnjBXhr4beAYz9/7AN/Ni+ajga/i+dsFXgf4a/59Xhv4LZ6XeMEQL9xLA78NHONfRzyv9wa+i+dvF3gd4K/5t3tt4Ld4XuIFQ/zLXhr4beAYLzrx/L038F08f7vA6wB/zb/NawO/xfMSLxjiRfPSwG8Dx3jRiBfsvYHv4vnbBV4H+Gv+9V4b+C2el3jBEC+6lwZ+GzjGv0y8cO8NfBfP3y7wOsBf86/z2sBv8bzEC4b413lp4LeBY7xw4oV7b+C7eMF2gdcB/poX3WsDv8XzEi8Y4l/vwcCDueKlga/ieZ0Adnlex4GPAj6bf9ku8DrAX/OieW3gt3he4gVD/Pu8NvBbPH/fDdwK/DXw0sCDgbcGjvOi2wVeB/hr/mWvDfwWz0u8YIh/n9cGfov/XLvA6wB/zQv32sBv8bzEC4b493kw8NfAMf7tLgEfzRXfxfO3C7wO8Ne8YK8N/BbPS7xgiH+/48BvAy/Fv94zgLcG/por3hv4Lp6/XeB9gJ/m+Xtt4Ld4XuIFQ/zHeW/gs4EH8S+7BHw18NXALs/pvYHv4gV7H+C7eV6vDfwWz0u8YIj/eK8NvDXw0sCDgQfxvF4H+G1esPcGvosX7GWAv+Y5vTbwWzwv8YIh/vOZ5/U6wG/zwr038F08f68D/DbP6bWB3+J5iRcM8Z/PPK/XAX6bf9l7A9/F83od4Ld5Tq8N/BbPS7xgiP985nm9DvDbvGjeG/guntPrAL/Nc3pt4Ld4XuIFQ/znM8/rdYDf5kX30sBxnu2vgV2e02sDv8XzEi8Y4j+feV6vA/w2/7FeG/gtnpd4wRD/+czzeh3gt/mP9drAb/G8xAuG+Pcz//VeB/htntNrA7/F8xIvGOLfz/zXex3gt3lOrw38Fs9LvGCIfz/zX+91gN/mOb028Fs8L/GCIf79zH+91wF+m+f02sBv8bzEC4b49zP/9V4H+G2e02sDv8XzEi8Y4t/P/Nd7H+C7eU7vDXwXz0u8YIh/P/Nf76eBt+E5/RTw1jynZwAP5gVD/PuZ/x6fDXwPV7wX8Nk8r+8B3psXDPHvZ/7neh/gu3nBEP9+fw28FP/z/A7w2rxwiH+/9wa+i/9ZLgEvDdzKC4f4j/HdwHvxP8MzgLcG/pp/GeI/zmcDHw0c47/HM4DvBr4a2OVFg/iP99r819sF/pp/PcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R5ontEEMIrSUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHistoryEdu;
impl IconShape for MdHistoryEdu {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9,4v1.38c-0.83-0.33-1.72-0.5-2.61-0.5c-1.79,0-3.58,0.68-4.95,2.05l3.33,3.33h1.11v1.11c0.86,0.86,1.98,1.31,3.11,1.36 V15H6v3c0,1.1,0.9,2,2,2h10c1.66,0,3-1.34,3-3V4H9z M7.89,10.41V8.26H5.61L4.57,7.22C5.14,7,5.76,6.88,6.39,6.88 c1.34,0,2.59,0.52,3.54,1.46l1.41,1.41l-0.2,0.2c-0.51,0.51-1.19,0.8-1.92,0.8C8.75,10.75,8.29,10.63,7.89,10.41z M19,17 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2h-6v-2.59c0.57-0.23,1.1-0.57,1.56-1.03l0.2-0.2L15.59,14H17v-1.41l-6-5.97V6h8V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdIosShare;
impl IconShape for MdIosShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 5l-1.42 1.42-1.59-1.59V16h-1.98V4.83L9.42 6.42 8 5l4-4 4 4zm4 5v11c0 1.1-.9 2-2 2H6c-1.11 0-2-.9-2-2V10c0-1.11.89-2 2-2h3v2H6v11h12V10h-3V8h3c1.1 0 2 .89 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zbHgfcCXhp4MP+9bgX+GvgeYJd/HcS/3nsDXwUc53+WXeBjgO/mRYf413lt4Lf4n+11gN/mRYP413k68GD+Z7sVeAgvGsSL7qWBv+J/h5cB/pp/GeJF99rAb/G8fod/vZcGjvGcLgF/zb/ea/G8Xgf4bf5liBfdawO/xfMS/3q/DbwWz+l3gNfmX888r9cBfpt/GeJF99rAb/G8xL/ebwOvxXP6HeC1+dczz+t1gN/mX4Z40b028Fs8L/Gv99vAa/Gcfgd4bf71zPN6HeC3+ZchXnSvDfwWz0v86/028Fo8p98BXpt/PfO8Xgf4bf5liBfdawO/xfMS/3q/DbwWz+l3gNfmX888r9cBfpt/GeJF99rAb/G8xL/ebwOvxXP6HeC1+dczz+t1gN/mX4Z40b028Fs8L/Gv99vAa/Gcfgd4bf71zPN6HeC3+ZchXjQvDbwW8NU8r9fmX++rgZfmOf018NH86/02z+ujgd8B/poXDvHCvTbwXcCD+d/pVuB9gN/m+UO8YO8NfBf/N7wP8N08L8Tzdxx4OnCc/xt2gYcAuzwnxPP30cBX8X/LxwBfzXNCPH/fDbwX/7d8D/DePCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8fy8NHOfZfovn73X4n+G3eP5eh2fbBf6a54R40fw08FY8p58B3pr/GX4aeCue088Ab80Lh3jRvDfwXTyn9wG+m/8Z3hv4Lp7T+wDfzQuHeNEcBy7ynE4Au/zPcBy4yHM6AezywiFedD8NvBVX/Azw1vzP8tPAW3HFzwBvzb8M8aJ7a+CjueKrgZ/mf5a3Bj6aK74a+Gn+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRIWnlBrU7pwgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdKingBed;
impl IconShape for MdKingBed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                height: "3",
                width: "5",
                x: "6",
                y: "7",
            }
            rect {
                height: "3",
                width: "5",
                x: "13",
                y: "7",
            }
            path {
                d: "M20,10V7c0-1.1-0.9-2-2-2H6C4.9,5,4,5.9,4,7v3c-1.1,0-2,0.9-2,2v5h1.33L4,19h1l0.67-2h12.67L19,19h1l0.67-2H22v-5 C22,10.9,21.1,10,20,10z M11,10H6V7h5V10z M18,10h-5V7h5V10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vV6aK/6a/x6I/z4vDfwWV7wO8Nf810P893hp4LeA41yxC7wO8Nf810L813tp4LeA4zynXeB1gL/mvw7iv9ZLA78FHOf52wVeB/hr/msg/uu8NPBbwHFeuF3gdYC/5j8f4r/GSwO/BRznRbMLvA7w1/znQvzne2ngt4Dj/OvsAq8D/DX/eRD/+S4Cx3lOHwP8Nc/ppYGv4jntAif4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYn/PIj/fOZ5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/iXvRb/Pr/N8/po4K95Ti8NfDXPS/znQfzLzH8v8Z8H8S8z/73Efx7Ev8z89xL/eRD/MvPfS/znQfzLzH8v8Z8H8S8zz+t1gN/mOb028Fu8aF4H+G2e02sDv8XzEv95EP8y87xeB/htntNrA7/Fi+Z1gN/mOb028Fs8L/GfB/EvM8/rdYDf5jm9NvBbvGheB/htntNrA7/F8xL/eRD/MvO8Xgf4bZ7TawO/xYvmdYDf5jm9NvBbPC/xnwfxLzPP63WA3+Y5vTbwW7xoXgf4bZ7TawO/xfMS/3kQ/zLzvP4a2OU5HQdemhfNXwO7PKfjwEvzvH6bf5/X4QVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8zz+htgl+d0HHgpXjR/A+zynI4DL8WL5m+AXZ7TceCleF7iBUP8y8zzeh3gt3lOrw38Fi+a1wF+m+f02sBv8aJ5HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4kXzOsBv85xeG/gtXjSvA/w2z+m1gd/ieYkXDPEvM8/rdYDf5jm9NvBbvGheB/htntNrA7/Fi+Z1gN/mOb028Fs8L/GCIf5l5nm9DvDbPKfXBn6LF83rAL/Nc3pt4Ld40bwO8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xonkd4Ld5Tq8N/BYvmtcBfpvn9NrAb/G8xAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmef1McBf85xeGvgqXjQfA/w1z+mlga/iRfMxwF/znF4a+Cqel3jBEP8y87xeB/htntNrA7/Fi+Z1gN/mOb028Fu8aF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld40bwO8Ns8p9cGfosXzesAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/BYvmtcBfpvn9NrAb/GieR3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+JF8zrAb/OcXhv4LV40rwP8Ns/ptYHf4nmJFwzxLzPP66OBv+Y5vTTw1bxoPhr4a57TSwNfzYvmo4G/5jm9NPDVPC/xgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/HDK5BE6ivcAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationCity;
impl IconShape for MdLocationCity {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 11V5l-3-3-3 3v2H3v14h18V11h-6zm-8 8H5v-2h2v2zm0-4H5v-2h2v2zm0-4H5V9h2v2zm6 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V9h2v2zm0-4h-2V5h2v2zm6 12h-2v-2h2v2zm0-4h-2v-2h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLuggage;
impl IconShape for MdLuggage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,6h-2V3c0-0.55-0.45-1-1-1h-4C9.45,2,9,2.45,9,3v3H7C5.9,6,5,6.9,5,8v11c0,1.1,0.9,2,2,2c0,0.55,0.45,1,1,1 c0.55,0,1-0.45,1-1h6c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1c1.1,0,2-0.9,2-2V8C19,6.9,18.1,6,17,6z M9.5,18H8V9h1.5V18z M12.75,18 h-1.5V9h1.5V18z M13.5,6h-3V3.5h3V6z M16,18h-1.5V9H16V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMasks;
impl IconShape for MdMasks {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.5,6c-1.31,0-2.37,1.01-2.48,2.3C15.14,7.8,14.18,6.5,12,6.5c-2.19,0-3.14,1.3-5.02,1.8C6.87,7.02,5.81,6,4.5,6 C3.12,6,2,7.12,2,8.5V9c0,6,3.6,7.81,6.52,7.98C9.53,17.62,10.72,18,12,18s2.47-0.38,3.48-1.02C18.4,16.81,22,15,22,9V8.5 C22,7.12,20.88,6,19.5,6z M3.5,9V8.5c0-0.55,0.45-1,1-1s1,0.45,1,1v3c0,1.28,0.38,2.47,1.01,3.48C4.99,14.27,3.5,12.65,3.5,9z M20.5,9c0,3.65-1.49,5.27-3.01,5.98c0.64-1.01,1.01-2.2,1.01-3.48v-3c0-0.55,0.45-1,1-1s1,0.45,1,1V9z M10.69,10.48 c-0.44,0.26-0.96,0.56-1.69,0.76V10.2c0.48-0.17,0.84-0.38,1.18-0.58C10.72,9.3,11.23,9,12,9s1.27,0.3,1.8,0.62 c0.34,0.2,0.71,0.42,1.2,0.59v1.04c-0.75-0.21-1.26-0.51-1.71-0.78C12.83,10.2,12.49,10,12,10C11.51,10,11.16,10.2,10.69,10.48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/cV4a+Cr+a3wM8Nf8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xonkd4Ld5Tq8N/BYvmtcBfpt/P8R/nNcGfosXzesAv81zem3gt3jRvA7w2/z7If7jvDbwW7xoXgf4bZ7TawO/xYvmdYDf5t8P8R/ntYHf4kXzOsBv85xeG/gtXjSvA/w2/36I/zivDfwWL5rXAX6b5/TawG/xonkd4Lf590P8x3lt4Ld40bwO8Ns8p9cGfosXzesAv82/H+I/zmsDv8WL5nWA3+Y5vTbwW7xoXgf4bf79EP9xXhv4LV40rwP8Ns/ptYHf4kXzOsBv8++H+I/z2sBv8aJ5HeC3eU6vDfwWL5rXAX6bfz/Ef5zXBn6LF83rAL/Nc3pt4Ld40bwO8Nv8+yH+47w28Fu8aF4H+G2e02sDv8WL5nWA3+bfD/Ef57WB3+JF8zrAb/OcXhv4LV40rwP8Nv9+iP84rw38Fi+a1wF+m+f02sBv8aJ5HeC3+fdD/Md5beC3eNG8DvDbPKfXBn6LF83rAL/Nvx/iP85rA7/Fi+Z1gN/mOb028Fu8aF4H+G3+/RD/cV4b+C1eNK8D/DbP6bWB3+JF8zrAb/Pvh/iP89rAb/GieR3gt3lOrw38Fi+a1wF+m38/xH+c1wZ+ixfN6wC/zXN6beC3eNG8DvDb/Psh/uO8NvBbvGheB/htntNrA7/Fi+Z1gN/m3w/xH+e1gd/iRfM6wG/znF4b+C1eNK8D/Db/foj/OC8N/BUvmtcBfpvn9NrAb/GieRngr/n3Q/zH+m7gvfiXvQ7w2zyn1wZ+i3/Z9wDvzX8MxH+8jwY+GzjGC/Y6wG/znF4b+C1esEvAZwNfzX8cxH+O48BXA+/F8/c6wG/znF4b+C2ev+8BPhrY5T8W4j/XawNfDbwUz+l9gO/mOb038F08p78BPhr4bf5zIP5rfDTw2cAxnu2ngY/hiq8C3ppnuwR8NvDV/OdC/Nc5Dnw18F482y5XHOfZvgf4aGCX/3yI/3qvDXw18FI8p78BPhr4bf7rIP77fDTwVVzxMcBX818P8d/npYG/4oqXAf6a/3qI/z7fDbwXV3wP8N7810P89zgOPB04zrOdAHb5r4X47/HRwFfxnD4H+Gz+ayH+ezwdeDDP6VbgIfzXQvzXe2vgp3j+3gb4af7rIP5jPRh4EPBg4MFc8WDgwVzxYODBvHC3Ardyxa3ArVxxK3Ar8AzgVv5jIP5jHQd+G3gp/nP8DfDawC7/MRD/Ob4beC/+Y30P8N78x0L85/ls4LP4j/E1wEfzHw/xn+u9ge/i3+d9gO/mPwfiP99rAz8NHONf5xLw1sBv858H8V/jpYHfBo7xorkEvDbw1/znQvzXeTrwYF40twIP4T8f4r/Gg4Gn86/zEOBW/nMh/mu8N/Bd/Ou8D/Dd/OdC/Nf4auCj+Nf5HuC9+c+F+K/xV8BL85z+Bvhorvhu4EE8p78GXob/XIj/fMeBizzbJeCrgc/mOX028NHAMZ7tBLDLfx7Ef763Bn6KK34G+GjgVp6/BwNfDbwVV7wN8NP850H85/tq4K2B9wZ+mxfNawPfDfw08NH850H853tr4Kf5t3lr4Kf5z4P4/w3x/xvi/zfE/2+I/9/4Rw/7ukGxjkkFAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMilitaryTech;
impl IconShape for MdMilitaryTech {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,10.43V2H7v8.43c0,0.35,0.18,0.68,0.49,0.86l4.18,2.51l-0.99,2.34l-3.41,0.29l2.59,2.24L9.07,22L12,20.23L14.93,22 l-0.78-3.33l2.59-2.24l-3.41-0.29l-0.99-2.34l4.18-2.51C16.82,11.11,17,10.79,17,10.43z M13,12.23l-1,0.6l-1-0.6V3h2V12.23z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMood;
impl IconShape for MdMood {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoodBad;
impl IconShape for MdMoodBad {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 3c-2.33 0-4.31 1.46-5.11 3.5h10.22c-.8-2.04-2.78-3.5-5.11-3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFI0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dY4DLwW8NHAcuAR8Nf+9EP+53gp4beCtgQfznN4G+Gn+eyH+4z0YeC/gvYEH8/z9DvDa/PdD/Mc5DnwU8NHAcV641wF+m/9+iP8Y7wV8NXCcf9kzgAfzPwPi3+c48FPAa/Oi+xjgq/mfAfFv99LATwEP5l/nBLDL/wyIf5uXBn4LOM6/zu8Ar83/HIh/vdcGfgo4zr/exwBfzf8ciH+dlwZ+CzjOv83rAL/N/xyIF91x4OnAcf7txP8siBfdbwGvzb/d3wAvzf8siBfNRwNfxb/P7wCvzf8siH/ZceDpwHH+fX4HeG3+Z0H8y74beC/+/T4H+Gz+Z0G8cA8Gns5/jM8BPpv/WRAv3FcDH8V/jO8B3pv/WRAv3EXgOP8xfgd4bf5nQbxgbw38FP9xfgd4bf5nQbxg3w28F/+xxP8siBfs6cCD+Y/1MsBf8z8H4vl7MPB0/uN9DPDV/M+BeP7eGvgp/uP9DPDW/M+BeP4+G/gs/nOcAHb5nwHx/H028Fn853gf4Lv5nwHx/P028Fr85/hr4GX4nwHx/P028Fr853kd4Lf574d4/n4b2AV+G/hr4FbgVq54MPDSwFsDrw08iH+93wZeh/9+iH+/9wY+G3gQ/zofA3w1/70Q/zGOA18NvBcvul3gIcAu/30Q/7HeG/guXnTfA7w3/30Q//HeG/guXnTvA3w3/z0QL9xx4K2ABwMvDdwK3Ar8DHArL9hXAx/Fi+51gN/mvx7iBfss4KOB4zx/3w18DLDL83cc+Gjgo4FjvHC7wOsAf81/LcTzOg78FPDa/Mt2gdcB/poX7MHATwMvxQu3C7wO8Nf810E8r+8C3psX3S7wEGCXF+w48NvAS/Evex/gu/mvgXhObw38FP963wO8Ny/cceCvgQfxL/ts4HP4z4d4Tn8FvDT/Nm8N7AK/wwv22sBv8aK5FXgf4Lf5z4N4tgcDT+ffbxf4auBzeP5+GngrXnTfDXwOcCv/8RDP9trAb/Ef57uB9+F5vTXwU/zrfTfwPcBv8x8H8WyvDfwW/7HeB/hunpf5t7sV+Gnge4C/5t8H8WyvDfwW/7F+B3htntdvA6/Ff4zfBv4a2AVuBW7lit/hX4Z4TuY/nnhevw28Fv95vgd4b/5liOf03cB78R9LPK/fBl6L/xx/A7w2sMu/DPGcHgw8nf84vwO8Ns/rt4HX4j/eJeC1gb/mRYN4Xu8NfBf/Md4H+G6el/mPdwl4beCvedEhnr/3Br6Lf5/vAd6b5/XWwE/xH+sS8NrAX/Ovg3jBHgx8NvBe/OtcAr4a+Gyev98CXpv/OH8DvDfw1/zrIV40r82L7rd5wV4b+C3+4/wM8N7ALv82iP86x4G/Ah7Mv98l4L2Bn+bfB/Ff4zjwW8BL8+9zCfhq4KuBXf79EP/5Xhr4LuCl+bd7BvDdwFcDu/zHQfznOg58NfDawIP413kG8NPAdwN/zX8OxH+d48BLA6/NFQ8GHswVv82z/Tbw18Au//kQ/78h/n9D/P+G+P8N8f8b/whunrFB5bL31gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNightsStay;
impl IconShape for MdNightsStay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.1,12.08C8.77,7.57,10.6,3.6,11.63,2.01C6.27,2.2,1.98,6.59,1.98,12c0,0.14,0.02,0.28,0.02,0.42 C2.62,12.15,3.29,12,4,12c1.66,0,3.18,0.83,4.1,2.15C9.77,14.63,11,16.17,11,18c0,1.52-0.87,2.83-2.12,3.51 c0.98,0.32,2.03,0.5,3.11,0.5c3.5,0,6.58-1.8,8.37-4.52C18,17.72,13.38,16.52,11.1,12.08z",
            }
            path {
                d: "M7,16l-0.18,0C6.4,14.84,5.3,14,4,14c-1.66,0-3,1.34-3,3s1.34,3,3,3c0.62,0,2.49,0,3,0c1.1,0,2-0.9,2-2 C9,16.9,8.1,16,7,16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoLuggage;
impl IconShape for MdNoLuggage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.75,9v0.92l1.75,1.75V9H16v4.17l3,3V8c0-1.1-0.9-2-2-2h-2V3c0-0.55-0.45-1-1-1h-4C9.45,2,9,2.45,9,3v3H8.83l3,3H12.75z M10.5,3.5h3V6h-3V3.5z M21.19,21.19L2.81,2.81L1.39,4.22l3.63,3.63C5.02,7.9,5,7.95,5,8v11c0,1.1,0.9,2,2,2c0,0.55,0.45,1,1,1 c0.55,0,1-0.45,1-1h6c0,0.55,0.45,1,1,1s1-0.45,1-1c0.34,0,0.65-0.09,0.93-0.24l1.85,1.85L21.19,21.19z M8,18v-7.17l1.5,1.5V18H8z M12.75,18h-1.5v-3.92l1.5,1.5V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADh0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z3eG/gq4DhX7AIfA3w3/7UQ//VeG/gtnr/XAX6b/zqI/3q/DbwWz9/vAK/Nfx3Efz3zwon/Ooj/euaFE/91EP91jgNvBXw3L9x7Az8D7PKfD/Gf76WBzwLemn+dnwY+B/hr/vMg/vMcB74KeG/+fb4b+Bhgl/94iP8cLw38FPBg/mP8NfA+wF/zHwvxH++lgd8CjvMfaxd4HeCv+Y+D+I/1YOCvgOP859gFHgLs8h8D8R/rr4CX5j/XXwMvw38MxH+cjwa+iv8aHwN8Nf9+iP8Yx4GnA8f5r7ELPATY5d8H8R/jo4Gv4r/WxwBfzb8P4j/G04EH81/rr4GX4d8H8e/30sBf8d/jZYC/5t8O8e/30cBX8d/jY4Cv5t8O8e/33cB78d/je4D35t8O8e/328Br8d/jd4DX5t8O8e/3dODB/Pe4FXgI/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3OQ5c5L/XCWCXfxvEv91x4LeAl+a/118DrwPs8q+H+Lc5DvwW8NL8z/DXwOsAu/zrIP71jgO/Bbw0/7P8NfA6wC4vOsS/znHgt4CX5n+mvwZeB9jlRYN40R0Hfgt4af5n+2vgdYBd/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffSwFfzv8NHA3/Nvwzx/xvi/zfEf52XBr6KF83HAH/Nfz7Ef52XBv6KF83LAH/Nfz7Ef61bgQfxwj0DeDD/NRD/tV4b+C1euNcBfpv/Goj/eu8NfDVwjOd0CXhv4Kf5r4P473EceG/gpbnir4HvBnb5r4X4/w3x/xvi/zfE/2+I/9/4R/aQZEGs+KbUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotifications;
impl IconShape for MdNotifications {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z4vDXwV8DbALi+69wa+CjjOFbvAxwDfzYvuOPBTwMcAf82/DeLf7r2BrwKOA38NvA6wy7/stYHf4vl7HeC3+ZcdB34LeGlgF/gY4Lv510P827w38F08p78GXgfY5YX7beC1eP5+B3htXrjjwG8BL81zeh/gu/nXQfzrvTfwXTx/fw28DrDLC2ZeOPGCHQd+C3hpnr/3Ab6bFx3iX+elgb/iBfsY4Kt54cwLJ164jwa+ihfsZYC/5kWDeNEdB/4KeDDP6xLw1sBv84IdB94K+G5euPcGfgbY5QV7beCngWM8r1uBlwF2+ZchXnS/Bbw2z+sS8NrAX/P8vTTwWcBb86/z08DnAH/N8/fSwG8Dx3hePw28Df8yxIvmrYGf4vl7HeC3eV7Hga8C3pt/n+8GPgbY5Xm9NvBbPH9vA/w0LxziX3YceDpwnOf1McBX87xeGvgp4MH8x/hr4H2Av+Z5fTTwVTyvW4GXAXZ5wRD/ss8GPovn9TPAW/O8Xhr4LeA4/7F2gdcB/prn9dvAa/G8Pgf4bF4wxAt3HHg6cJzndAl4aeBWntODgb8CjvOfYxd4CLDLc3ow8NfAMZ7TLnCCFwzxwn008FU8r88BPpvn9VfAS/Of66+Bl+F5fTbwWTyv9wG+m+cP8cI9HXgwz+kS8GBgl+f00cBX8V/jY4Cv5jkdB24FjvGc/hp4GZ4/xAv20sBf8bw+B/hsntNx4OnAcf5r7AIPAXZ5Tp8NfBbP6yHArTwvxAv21cBH8bweAtzKc/po4Kv4r/UxwFfznB4MPJ3n9THAV/O8EC/YbwOvxXP6HeC1eV5PBx7Mf62/Bl6G5/XXwEvxnH4HeG2eF+IFM8/rc4DP5jm9NPBX/Pd4GeCveU6fDXwWz0s8L8Tz99LAX/G8Xgf4bZ7TRwNfxX+PjwG+muf02sBv8bxeBvhrnhPi+Xtr4Kd4XuJ5fTfwXvz3+B7gvXle5nm9DvDbPCfE8/fZwGfxnC4Bx3levw28Fv89fgd4bZ6XeV6fA3w2zwnx/H028Fk8p98BXpvn9XTgwfz3uBV4CM/rt4HX4jl9DvDZPCfEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7HAcu8t/rBLDLvw3i3+448FvAS/Pf66+B1wF2+ddD/NscB34LeGn+Z/hr4HWAXf51EP96x4HfAl6a/1n+GngdYJcXHeJf5zjwW8BL8z/TXwOvA+zyokG86I4DvwW8NP+z/TXwOsAu/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontp4Kv53+Gjgb/mX4b4/w3x/xviv85LA1/Fi+ZjgL/mPx/iv85LA3/Fi+ZlgL/mPx/iv9atwIN44Z4BPJj/Goj/Wm8N/BQv3OsAv81/DcR/vfcGvho4xnO6BLw38NP810H89zgOvDfw0sBx4KeBnwZ2+a+F+P8N8f8b4v83xP9viP/f+Edf18BBk5DQMgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotificationsActive;
impl IconShape for MdNotificationsActive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.58 4.08L6.15 2.65C3.75 4.48 2.17 7.3 2.03 10.5h2c.15-2.65 1.51-4.97 3.55-6.42zm12.39 6.42h2c-.15-3.2-1.73-6.02-4.12-7.85l-1.42 1.43c2.02 1.45 3.39 3.77 3.54 6.42zM18 11c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2v-5zm-6 11c.14 0 .27-.01.4-.04.65-.14 1.18-.58 1.44-1.18.1-.24.15-.5.15-.78h-4c.01 1.1.9 2 2.01 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z3eG/gq4DhX7AIfA3w3/7UQ//VeG/gtnr/XAX6b/zqI/3q/DbwWz9/vAK/Nfx3Efz3zwon/Ooj/euaFE/91EP91jgNvBXw3L9x7Az8D7PKfD/Gf76WBzwLemn+dnwY+B/hr/vMg/vMcB74KeG/+fb4b+Bhgl/94iP8cLw38FPBg/mPcCrwN8Nf8x0L8x3tp4LeA4/zH2gVeB/hr/uMg/mM9GPgr4Dgv2CXgp4FbgVu54sHAg4G3Bo7xgu0CLwPcyn8MxH+svwJemufvGcBHAz/NC/fWwFcDD+L5+2vgZfiPgfiP89HAV/H8fQ/w0cAuL5rjwFcD78Xz9zHAV/Pvh/iPcRx4OnCc5/U9wHvzb/PdwHvxvHaBhwC7/Psg/mN8NPBVPK/fAV6bf5/fBl6L5/UxwFfz74P4j/F04ME8p0vAg4Fd/n0eDPw1cIzn9NfAy/Dvg/j3e2ngr3heXwN8NP8xvht4L57XywB/zb8d4t/vo4Gv4nk9BLiV/xgPBp7O8/oY4Kv5t0P8+3038F48p2cAD+Y/1q3Ag3hO3wO8N/92iH+/3wZei+f0PcB78x/ru4H34jn9DvDa/Nsh/v2eDjyY5/Q5wGfzH+uzgc/iOd0KPIR/O8S/n3leHwN8Nf+xPhr4Kp6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/n3lerwP8Nv+xXhv4LZ6X+LdD/PuZ5/U6wG/zH+u1gd/ieYl/O8S/z3HgIs/rdYDf5j/WawO/xfM6Aezyb4P4tzsO/Bbw0jyv1wF+m/9Yrw38Fs/rr4HXAXb510P82xwHfgt4aZ6/1wF+m/9Yrw38Fs/fXwOvA+zyr4P41zsO/Bbw0rxgfw3s8h/rOPDSvGB/DbwOsMuLDvGvcxz4LeCl+Z/pr4HXAXZ50SBedMeB3wJemv/Z/hp4HWCXfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b008NX87/DRwF/zL0P8/4b4/w3xX+elga/iRfMxwF/znw/xX+elgb/iRfMywF/znw/xX+tW4EG8cM8AHsx/DcR/rdcGfosX7nWA3+a/BuK/3nsDXw0c4zldAt4b+Gn+6yD+exwH3ht4aa74a+C7gV3+ayH+f0P8/4b4/w3x/xvi/zf+EeqKo0GkihhsAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotificationsNone;
impl IconShape for MdNotificationsNone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.07-1.63-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.64 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2zm-2 1H8v-6c0-2.48 1.51-4.5 4-4.5s4 2.02 4 4.5v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z3eG/gq4DhX7AIfA3w3/7UQ//VeG/gtnr/XAX6b/zqIf5uX5oq/5l/vt4HX4vn7HeC1+a+D+Nd7aeC3uOJ1gL/mX8e8cOK/DuJf56WB3wKOc8Uu8DrAX/OiMy+c+K+DeNE9GPgr4DjPaRd4HeCveeGOA28FfDcv3HsDPwPs8p8P8a/z3cB78bx2gdcB/prn9drAewHvzb/OTwOfA/w1/3kQ/3rfDbwXz2sXeB3gr3lO7w18F/923w18DLDLfzzEv813A+/F89oFXgf4a57TewPfxb/dXwPvA/w1/7EQ/3bfDbwXz2sXeB3gr3lO7w18F/92u8DrAH/NfxzEv893A+/F89oFHgLs8pw+G/gs/u12gZcBbuU/BuLf77uB9+J5/TXwOsAuz+m7gffi3+6vgZfhPwbiP8Z3A+/F8/pr4HWAXZ7TdwPvxb/dxwBfzb8f4j/GceAiz99fA68D7PKcvht4L/5tdoGHALv8+yD+Y3w08FW8YH8NvA6wy3P6buC9+Lf5GOCr+fdB/Md4OvBgXri/Bl4H2OXZjgO/DbwU/3p/DbwM/z6If7+XBv6KF81fA68D7PJsx4HfBl6Kf72XAf6afzvEv99HA1/Fi+6vgdcBdnm248BvAy/Fv87HAF/Nvx3i3++7gffiX+e3gdfhOR0Hfht4KV503wO8N/92iH+/3wZei3+97wbeh+d0HPhr4EG8aH4HeG3+7RD/fubf7ruB9+E5vTTw28Ax/mW3Ag/h3w7x72f+fb4beB+e00sDvw0c418m/u0Q/37m3++7gffhOb008NvAMV448W+H+Pcz/zG+G3gfntNLA78NHOMFE/92iH8/8x/nu4H34Tm9NvBbvGDi3w7x72f+Y3038D48p/cGvovnT/zbIf79zH+8zwE+m+f03sB38bzEvx3i38/853gf4Lt5Tu8NfBfPSfzbIf79zH+e9wG+m+f03sB38Wzi3w7x72f+c70P8N08p68GPoorxL8d4t/P/Od7H+C7eU7fDbwXIP7tEP9+5r/G+wDfzXP6buC9+bdD/PuZ/zrvA3w3/3EQ/37mv9b7AN/NfwzEv5/5r/c6wG/z74f49zkOXOS/3i7wOsBf8++D+Lc7DvwW8NL899gFXgf4a/7tEP82x4HfAl6a/167wOsAf82/DeJf7zjwW8BL8z/DLvA6wF/zr4f41zkO/Bbw0vzPsgu8DvDX/OsgXnTHgd8CXpr/mXaB1wH+mhcd4kX32sBv8T/bLvA6wF/zokG86F4b+C3+59sFHgLs8i9DvOheGvhq/md5MPAgntdfA68D7PLCIf53Ow78NvBSPK+/Bl4H2OUFQ/zvdxz4beCleF6/A7w2Lxjiv85LA1/Fi+ZjgL/mRXcc+G3gpXi2S8BrA3/NC4b4r/PSwF/xonkZ4K/51zkO/DbwUsAl4LWBv+aFQ/zXuhV4EC/cM4AH829zHPhp4KOBv+Zfhviv9dbAT/HCvQ3w0/zXQPzXe2/gq4FjPKdLwEcD381/HcR/j+PAewMvzRV/DXw3sMt/LcT/b4j/3xD/vyH+f0P8/8Y/AiRCwUFIQpmkAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotificationsOff;
impl IconShape for MdNotificationsOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18.69L7.84 6.14 5.27 3.49 4 4.76l2.8 2.8v.01c-.52.99-.8 2.16-.8 3.42v5l-2 2v1h13.73l2 2L21 19.72l-1-1.03zM12 22c1.11 0 2-.89 2-2h-4c0 1.11.89 2 2 2zm6-7.32V11c0-3.08-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68c-.15.03-.29.08-.42.12-.1.03-.2.07-.3.11h-.01c-.01 0-.01 0-.02.01-.23.09-.46.2-.68.31 0 0-.01 0-.01.01L18 14.68z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70WV/wO/z0Q/z3eG/gq4DhX7AIfA3w3/7UQ//VeG/gtnr/XAX6b/zqI/3q/DbwWz9/vAK/Nfx3Efz3zwon/Ooj/euaFE/91EP91jgNvBXw3L9x7Az8D7PKfD/Gf76WBzwLemn+dnwY+B/hr/vMg/vMcB74KeG/+fb4b+Bhgl/94iP8cLw38FPBg/mP8NfA+wF/zHwvxH++lgd8CjvMfaxd4HeCv+Y+D+I/1YOCvgOP859gFHgLs8h8D8R/rr4CX5j/XXwMvw38MxH+cjwa+iv8aHwN8Nf9+iP8Yx4GnA8f5r7ELPATY5d8H8R/jo4Gv4gX7GOCv+dd5aeCreME+Bvhq/n0Q/zGeDjyYF+x1gN/mX+e1gd/iBftr4GX490H8+7008Fe8cK8D/Db/Oq8N/BYv3MsAf82/HeLf76OBr+KFex3gt/nXeW3gt3jhPgb4av7tEP9+3w28Fy/c6wC/zb/OawO/xQv3PcB782+H+Pf7beC1eOH+Gtjl+fsa4Kd5Xt8FvDcv3O8Ar82/HeLf7+nAg/m3+R7gvXleHw18Ff+yW4GH8G+H+Pcz/zZ/A7w2sMtzemvgp3jRiX87xL+f+de7BDwY2OU5vTTwW8BxXnTi3w7x72f+9V4G+Gue03Hgr4AH868j/u0Q/37mX+d9gO/mef0W8Nr864l/O8S/n3nRfQ3w0Tyv7wLem38b8W+H+PczL5qfAd6a5/XewHfxbyf+7RD/fuZf9jfAawO7PKfXBn6Lfx/xb4f49zP/srcBfprn9dLAcZ6/lwa+in+Z+LdD/PuZf9nrAL/Nv85rA7/Fv0z82yH+/cy/7HWA3+Zf57WB3+JfJv7tEP9+5l/2OsBv86/z2sBv8S8T/3aIfz/zL3sd4Lf513lt4Lf4l4l/O8S/n/nvJf7tEP9+5r+X+LdD/PscBy7y3+sEsMu/DeLf7jjwW8BL89/rr4HXAXb510P82xwHfgt4af5n+GvgdYBd/nUQ/3rHgd8CXpr/Wf4aeB1glxcd4l/nOPBbwEvzP9NfA68D7PKiQbzojgO/Bbw0/7P9NfA6wC7/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie2ngq/nf4aOBv+Zfhvj/DfH/G+K/zksDX8WL5mOAv+Y/H+K/zksDf8WL5mWAv+Y/H+K/1q3Ag3jhngE8mP8aiP9arw38Fi/c6wC/zX8NxH+99wa+GjjGc7oEvDfw0/zXQfz3OA68N/DSXPHXwHcDu/zXQvz/hvj/DfH/G+L/N8T/b/wjJh6PQcvE+OIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotificationsPaused;
impl IconShape for MdNotificationsPaused {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.93 6 11v5l-2 2v1h16v-1l-2-2zm-3.5-6.2l-2.8 3.4h2.8V15h-5v-1.8l2.8-3.4H9.5V8h5v1.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOutdoorGrill;
impl IconShape for MdOutdoorGrill {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,22c1.66,0,3-1.34,3-3s-1.34-3-3-3c-1.3,0-2.4,0.84-2.82,2H9.14l1.99-3.06C11.42,14.98,11.71,15,12,15 s0.58-0.02,0.87-0.06l1.02,1.57c0.42-0.53,0.96-0.95,1.6-1.21l-0.6-0.93C17.31,13.27,19,10.84,19,8H5c0,2.84,1.69,5.27,4.12,6.37 l-3.95,6.08c-0.3,0.46-0.17,1.08,0.29,1.38h0c0.46,0.3,1.08,0.17,1.38-0.29l1-1.55h6.34C14.6,21.16,15.7,22,17,22z M17,18 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C16,18.45,16.45,18,17,18z",
            }
            path {
                d: "M9.41,7h1c0.15-1.15,0.23-1.64-0.89-2.96C9.1,3.54,8.84,3.27,9.06,2H8.07C7.86,3.11,8.1,4.05,8.96,4.96 C9.18,5.2,9.75,5.63,9.41,7z",
            }
            path {
                d: "M11.89,7h1c0.15-1.15,0.23-1.64-0.89-2.96c-0.42-0.5-0.68-0.78-0.46-2.04h-0.99c-0.21,1.11,0.03,2.05,0.89,2.96 C11.67,5.2,12.24,5.63,11.89,7z",
            }
            path {
                d: "M14.41,7h1c0.15-1.15,0.23-1.64-0.89-2.96C14.1,3.54,13.84,3.27,14.06,2h-0.99c-0.21,1.11,0.03,2.05,0.89,2.96 C14.18,5.2,14.75,5.63,14.41,7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/Pu8DvDbPKfXBn6Lf59d4KeBzwFu5V+GeNG8N/Bd/Md5HeC3eU6vDfwW/3HeB/huXjjEv+zBwNP5j/U6wG/znF4b+C3+Yz0EuJUXDPEv+27gvfiP9TrAb/OcXhv4Lf5jfQ/w3rxgiH/ZReA4/7FeB/htntNrA7/Ff6xd4AQvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/MvGguAbvAg/iXvQ7w2zyn1wZ+i3/ZM4DjwDFeNOIFQ/zLzIvmVuAhwIOB1wZeG3ht4EE8r9cBfpvn9NrAb/G8ngH8NvDbwG8DtwJPBx7Mi0a8YIh/mXnRPQS4lef0YOC1gdcGXht4EPA6wG/znF4b+C3gGcBvA78N/DZwK8/pwcDTedGJFwzxLzMvuvcBvpsX7sHALrDLczoOHAdu5YV7b+C7eNGJFwzxLzMvuu8B3pv/XN8NvBcvOvGCIf5l5kV3K/AQ/nM9HXgwLzrxgiH+ZeZf5yHArfzneDDwdP51xAuG+JeZf533Ab6b/xzvDXwX/zriBUP8y8y/zvcA781/ju8G3ot/HfGCIf5l5l/nVuAh/Od4OvBg/nXEC4b4l5l/vYcAt/If68HA0/nXEy8Y4l/22vzr/TWwy3+s48BL86/327xgiP/fEP+/If5/Q/z/hviXvRb/en8D7PIf6zjwUvzr/Q4vGOJfZv71HgLcyn+sBwNP519PvGCIf5n513kG8GD+c9wKPIh/HfGCIf5l5l/ne4D35j/HdwPvxb+OeMEQ/zLzr/M+wHfzn+O9ge/iX0e8YIh/mfnXeQhwK/85Hgw8nX8d8YIh/mXmRfcM4MH857oVeBAvOvGCIf5l5kX3PcB785/ru4H34kUnXjDEv8y86N4H+G5euAcDu8Auz+k4cBy4lRfuvYHv4kUnXjDEv8y86B4C3MpzejDwWsBrA68NPBh4HeC3eU6vDfwWcCvw28BvA78D3MpzejDwdF504gVD/MvMi+YZwIOBBwOvBbw28NrAg3lerwP8Ns/ptYHf4nndCvw28NvA7wC3ArcCD+JFI14wxL/MvGh2gV3gwfzLXgf4bZ7TawO/xb/sVuA4cJwXjXjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mW7wDH+Y70O8Ns8p9cGfov/WJeA47xgiH/ZdwPvxX+s1wF+m+f02sBv8R/re4D35gVD/MseDDyd/1ivA/w2z+m1gd/iP9ZDgFt5wRAvmvcGvov/OK8D/DbP6bWB3+I/zvsA380Lh3jRPRj4bOCtgWP8+7wO8Ns8p9cGfot/n0vATwOfDdzKvwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwQLYLpBSXMqGQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPages;
impl IconShape for MdPages {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 5v6h5L7 7l4 1V3H5c-1.1 0-2 .9-2 2zm5 8H3v6c0 1.1.9 2 2 2h6v-5l-4 1 1-4zm9 4l-4-1v5h6c1.1 0 2-.9 2-2v-6h-5l1 4zm2-14h-6v5l4-1-1 4h5V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/uV4a+Cr+fT4G+Gv+cyD+87w08FvAcf59doHXAf6a/3iI/xzHgb8CHsx/jFuBlwF2+Y+F+I93HPgt4KX5j/XXwOsAu/zHQfzHOg78FvDS/Of4a+B1gF3+YyBeNC8NHONf9tHAW/Of66eBr+Zfdgn4a144xAv32sB3AQ/mf6dbgfcBfpvnD/GCvTfwXfzf8D7Ad/O8EM/fceDpwHH+b9gFHgLs8pwQz99HA1/F/y0fA3w1zwnx/H038F783/I9wHvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxX+u3+HZjgMvxX+u3wFem+eEeP5+G3gt/mM9A/hu4KeBv+b5e2ngvYH3Bo7xH+t3gNfmOSGev98GXov/GJeAjwa+m3+djwY+GzjGf4zfAV6b54R4/n4beC3+/f4GeG1gl3+bBwM/DbwU/36/A7w2zwnx/P028Fr8+3wP8N78+x0Hfht4Kf59fgd4bZ4T4vn7beC1+Lf7G+Cl+Y9zHPht4KX4t/sd4LV5Tojn77eB1+Lf5hLwYGCX5+/BwEcBrw28NP91fgd4bZ4T4vn7beC1+Ld5H+C7ef6+Cvho/nv8DvDaPCfE8/fbwGvxr/cM4ME8f38FvDT/fX4HeG2eE+L5+23gtfjX+xzgs3leXw18FP+9fgd4bZ4T4vn7beC1+Nd7GeCveU4PBp7Of7/fAV6b54R4/n4beC3+9cTz+mrgo/i3+Rngq3lePw0c41/nd4DX5jkhnr/fBl6Lf53fAV6b5/VXwEvzb/PXwMvwvH4beC3+dX4HeG2eE+L5+23gtfjX+R3gtXle5t9HPK/fBl6Lf53fAV6b54R4/n4beC3+dX4HeG2el/n3Ec/rt4HX4l/nd4DX5jkhnr/fBl6Lf53fAV6b5/XXwEvxb/M3wEvzvH4beC3+dX4HeG2eE+L5+23gtfjX2QXemuf10cBb82/zNcBH87zMv97vAK/Nc0I8f78NvBb//R4C3Mpzemngr/jX+x3gtXlOiOfvt4HX4r/X1wAfzfP6bOCz+Nf7HeC1eU6I5++3gdfiv8/fAC/N8/d04MH86/0O8No8J8Tz99vAa/Hf42uAj+b5+2jgq/i3+R3gtXlOiOfvt4HX4r/O3wC/DXw1cCvP33Hg6cBx/m1+B3htnhPi+ftt4LX4t/tr4HWAXf7j/BXw0vzb/Q7w2jwnxPP328Br8e/z18DrALv8+30X8N78+/wO8No8J8Tz99vAa/Hv99fA2wC38m9zHPgt4KX59/sd4LV5Tojn77eB1+I/xi7w2cDX8K/zUcBnA8f5j/E7wGvznBDP328Dr8V/rF3gu4HvAf6a5++lgbcC3ht4MP+xfgd4bZ4T4vn7beC1+M/118Auz/ba/Of6HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77uB9+L/lq8BPprnhHj+Phr4Kv5v+Rjgq3lOiOfvOHArcIz/Gy4BDwZ2eU6IF+y9ge/i/4a3AX6a54V44V4b+G7gQfzv9AzgvYHf5vlDvGheGjjO/y67wF/zwiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJYwLlBVlMQMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPartyMode;
impl IconShape for MdPartyMode {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4h-3.17L15 2H9L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 3c1.63 0 3.06.79 3.98 2H12c-1.66 0-3 1.34-3 3 0 .35.07.69.18 1H7.1c-.06-.32-.1-.66-.1-1 0-2.76 2.24-5 5-5zm0 10c-1.63 0-3.06-.79-3.98-2H12c1.66 0 3-1.34 3-3 0-.35-.07-.69-.18-1h2.08c.07.32.1.66.1 1 0 2.76-2.24 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEq0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/3EeDLwVcBx4aeCvgb8GfgfY5d/mOPBWwIOBlwb+GtgFfga4lX8/xL/fceCrgPfm+dsFvhr4HP51Pgv4aOA4z993Ax8D7PJvh/j3eWngt4Dj/Mt+GngfYJcX7jjwU8Br8y/bBV4H+Gv+bRD/dseBpwPHedF9N/A+vHDfBbw3L7pd4CHALv96iH+77wbei3+91wF+m+fvrYGf4l/ve4D35l8P8W/zYODp/Nv8NfAyPH9/Bbw0/zYPAW7lXwfxb/PRwFfxb3cC2OU5PRh4Ov927wN8N/86iH+brwY+in+71wF+m+f02sBv8W/3OcBn86+D+Lf5aeCt+Ld7HeC3eU6vDfwW/3ZfA3w0/zqIf5vPBj6Lf7vXAX6b5/TawG/xb/c5wGfzr4P4t3lv4Lv4t7kEHOf5M/92bwP8NP86iH+b48CtwDH+9b4HeG+ev+8G3ot/vUvAcf71EP92nw18Fv96DwFu5fl7MPB0/vU+Bvhq/vUQ/z6/DbwWL7r3Ab6bF+69ge/iRfczwFvzb4P49zkOfDXwXvzL3gf4bl407w18F/+y7wE+Gtjl3wbxH+Otgc8GXorn9T3AZwO38q/zYOCzgffief0N8NHAb/Pvg/iP9WDgwTzbb/Mf47V5tluBW/mPgfj/DfH/G+L/N8T/b4j/OC8NHON5PQO4lX+7lwaO8ZwuAX/Nvx/i3+a1gNcGXhp4aeDB/Mv+Gvhr4LeB3wFu5UXz0sBvA8d4XrcCfw38NfDbwO/wr4N40b0V8NbAWwPH+fe7Ffhp4HuAv+aFezDw08BL8cLtAj8N/DTwM/zLEC/cceCjgPcGHsx/nt8Gvhv4Hl6wlwZ+GzjGi+ZW4LuBrwF2ef4QL9hHAZ8NHOe/zm8DHwP8Nc/fewPfxb/OLvDZwNfwvBDP6zjwU8Br89/no4Gv4fn7a+Cl+Nf7aeB9gF2eDfGcjgO/Bbw0//0+B/hsntd7A9/Fv81fA68D7HIF4jl9NfBR/M/xOsBv87zMv93XAB/NFYhnOw5c5H+WnwHemud1K/Ag/u1OALsA4tleG/gt/ucRz+u3gdfi3+51gN8GEM/22sBv8T+PeF6/DbwW/3avA/w2gHi21wZ+i/95xPP6beC1+Ld7HeC3AcSzvTbwW/zPI57XbwOvxb/d6wC/DSCe7bWB3+J/HvG8fht4Lf7tXgf4bQDxbK8N/Bb/84jn9dvAa/Fv9zrAbwOIZ3tt4Lf4n0c8r98GXot/u9cBfhtAPNtLA1/N/zyvzfP6auCl+bf7aOCvAcT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIVe5xBaApKWQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPeople;
impl IconShape for MdPeople {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP89r8Zx+h3+d1+I5/Q7/8RD/sd4aeC/grXn+fhr4HuCnecFeG/gtnr+fBr4H+Gn+YyD+Y7w08FXAa/Oi+W3gY4C/5vn7beC1eMF+G/gY4K/590H8+7008FvAcf51doHXAf6a5/Vg4Om8cLvA6wB/zb8d4t/npYHfAo7zb7MLvA7w1zyv7wbeixduF3gd4K/5t0H82x0H/gp4MP8+twIP4Xk9GHg6/7JbgYfwb4P4t/ts4LP4j/E5wGfzvP4aeCn+ZZ8DfDb/eoh/u4vAcf5j7AIneF5fDXwU/7Jd4AT/eoh/m9cGfov/WC8D/DXP6bWB3+JF8zLAX/Ovg/i3+Wzgs/iP9TnAZ/OcXhv4LV40nwN8Nv86iH+bzwY+i/9YnwN8Ns/LvGg+B/hs/nUQ/zY/DbwV/7F+Bnhrnpd50fwM8Nb86yD+bT4b+Cz+Y30O8Nk8p+PARV40nwN8Nv86iH+bjwa+iv9YHwN8Nc/ptYHf4kXzMcBX86+D+Ld5MPB0/mM9BLiV5/TewHfxonkIcCv/Ooh/u98GXov/GL8DvDbP67uB9+Jf9jvAa/Ovh/i3e23gt/iP8TrAb/O8ng48mH/Z6wC/zb8e4t/nu4H34t/ne4D35nm9NfBT/Mu+B3hv/m0Q/z7Hgd8GXop/m78BXhvY5Xn9FvDavHB/A7w2sMu/DeLf7zjw08Br8a/zO8BbA7s8r7cGfooX7neAtwZ2+bdD/Mf5aOCzgWO8cJeAzwa+mufvOPB04DjP3yXgs4Gv5t8P8R/rOPDWwFsDDwZeiiv+BrgV+Gngp4FdXrDXBt4beGngpbjib4BbgZ8GfhrY5T8G4v83xP9viP94x4GX4vn7G2CX/zkQ/z4vDbwW8NLAg4HX5kXz28CtwF8DvwP8NS+6BwMfDRwD/hr4HeCv+bdB/Ou9FfDWwFsDx/mPsQv8NPDTwM/wovlu4L24Yhf4aeCngZ/hRYd40TwY+CzgrYHj/OfaBX4a+BzgVl647wbei+e0C/w08DnArbxwiBfuwcBnAe/Nf4/vBj4HuJUX7LeB1+L5+27gc4Bbef4QL9hnAZ/N/wyfDXwOz99x4FbgGM/fLvDVwOfwvBDP6zjwU8Br8z/LbwNvA+zyvD4b+CxeuO8GPgbY5dkQz+uvgJfmf6a/Bl6G53UcuMi/7LeB1+HZEM/ps4HP4n+2zwE+m+f128Br8S/7GOCruQLxnC4Cx/mfbRc4wfP6bOCz+JftAie4AvFsrw38Fv87vA7w2zynjwa+ihfNywB/DSCe7bWB3+J/h9cBfpvn9NrAb/GieR3gtwHEs7028Fv87/A6wG/znF4b+C1eNK8D/DaAeLbXBn6L/x1eB/htntNrA7/Fi+Z1gN8GEM/22sBv8b/D6wC/zXN6beC3eNG8DvDbAOLZXhv4Lf53eB3gt3lOrw38Fi+a1wF+G0A822sDv8X/Dq8D/DbP6bWB3+JF8zrAbwOIZ3tt4Lf43+F1gN/mOb028Fu8aF4H+G0A8WyvDfwW/zu8DvDbPKfXBn6LF83rAL8NIJ7tOPDS/O/w18Auz+k48NK8aP4a2AUQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BnjDJQYu3L24AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPeopleAlt;
impl IconShape for MdPeopleAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.67,13.13C18.04,14.06,19,15.32,19,17v3h4v-3 C23,14.82,19.43,13.53,16.67,13.13z",
                fill_rule: "evenodd",
            }
            circle {
                cx: "9",
                cy: "8",
                fill_rule: "evenodd",
                r: "4",
            }
            path {
                d: "M15,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4c-0.47,0-0.91,0.1-1.33,0.24 C14.5,5.27,15,6.58,15,8s-0.5,2.73-1.33,3.76C14.09,11.9,14.53,12,15,12z",
                fill_rule: "evenodd",
            }
            path {
                d: "M9,13c-2.67,0-8,1.34-8,4v3h16v-3C17,14.34,11.67,13,9,13z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPeopleOutline;
impl IconShape for MdPeopleOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.5 13c-1.2 0-3.07.34-4.5 1-1.43-.67-3.3-1-4.5-1C5.33 13 1 14.08 1 16.25V19h22v-2.75c0-2.17-4.33-3.25-6.5-3.25zm-4 4.5h-10v-1.25c0-.54 2.56-1.75 5-1.75s5 1.21 5 1.75v1.25zm9 0H14v-1.25c0-.46-.2-.86-.52-1.22.88-.3 1.96-.53 3.02-.53 2.44 0 5 1.21 5 1.75v1.25zM7.5 12c1.93 0 3.5-1.57 3.5-3.5S9.43 5 7.5 5 4 6.57 4 8.5 5.57 12 7.5 12zm0-5.5c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9 5.5c1.93 0 3.5-1.57 3.5-3.5S18.43 5 16.5 5 13 6.57 13 8.5s1.57 3.5 3.5 3.5zm0-5.5c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADj0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv89r8Zx+h/96iP9abw28F/DWPH8/DXwP8NP810D813hp4KuA1+ZF89vAxwB/zX8uxH++lwZ+CzjOv84u8DrAX/OfB/Gf66WB3wKO82+zC7wO8Nf850D85zkO/BXwYP59bgUewn8OxH+ezwY+i/8YnwN8Nv/xEP95LgLH+Y+xC5zgPx7iP8drA7/Ff6yXAf6a/1iI/xyfDXwW/7E+B/hs/mMh/nN8NvBZ/Mf6HOCz+Y+F+M/x08Bb8R/rZ4C35j8W4j/HZwOfxX+szwE+m/9YiP8cHw18Ff+xPgb4av5jIf5zPBh4Ov+xHgLcyn8sxH+e3wZei/8YvwO8Nv/xEP95Xhv4Lf5jvA7w2/zHQ/zn+m7gvfj3+R7gvfnPgfjPdRz4beCl+Lf5G+C1gV3+cyD+8x0Hfhp4Lf51fgd4a2CX/zyI/zofDXw2cIwX7hLw2cBX858P8V/rOPDWwFsDDwZeiiv+BrgV+Gngp4Fd/msg/n9D/P+G+P8N8f8b4v83xH+dBwMPAl6bKx4MPJgrbgVu5YrfBp4B3Mp/PsR/ntcCXht4beC1+bf5beC3gd8Gfof/eIj/WG8FvDfw2sBx/mPtAj8N/DTwM/zHQPz7HQc+Cnhv4MH817gV+G7ga4Bd/u0Q/z6fBXw0cJz/HrvAVwOfw78N4t/mpYHvAl6a/xn+Gngb4Fb+dRD/ei8N/BZwnP9ZdoHXAf6aFx3iX+fBwF8Bx/mfaRd4GeBWXjSIf52fBt6K/9m+B3hvXjSIF92Dgafzv8NDgFv5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRHQdemv8d/hrY5V+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IXelkQXsy3UQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPerson;
impl IconShape for MdPerson {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEPElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif67X4jn9Dv/xEP+zvDXwXsBb8/z9NPA9wE/zHwPxP8NLA18FvDYvmt8GPgb4a/59EP/9Xhr4LeA4/zq7wOsAf82/HeK/10sDvwUc599mF3gd4K/5t0H89zkO/BXwYP59bgUewr8N4r/PZwOfxX+MzwE+m389xH+fi8Bx/mPsAif410O86I4DL8Xz+h3+9V4b+C3+Y70M8Nf86yBedK8N/BbPS/zrfTbwWfzH+hzgs/nXQbzoXhv4LZ6X+Nf7bOCz+I/1OcBn86+DeNG9NvBbPC/xr/fTwFvxH+tngLfmXwfxontt4Ld4XuJf77OBz+I/1ucAn82/DuJF99rAb/G8xL/eRwNfxX+sjwG+mn8dxIvutYHf4nmJf70HA0/nP9ZDgFv510G86F4b+C2el/i3+W3gtfiP8TvAa/Ovh3i248BL8YK9NPDVPK/X5oX7HZ6/1wZ+i/8YrwP8Nv96iGd7beC3+I8nXrDvBt6Lf5/vAd6bfxvEs7028Fv8xxMv2HHgt4GX4t/mb4DXBnb5t0E822sDv8V/PPHCHQd+Gngt/nV+B3hrYJd/O8SzvTbwW/zHEy+ajwY+GzjGC3cJ+Gzgq/n3QzzbawO/xX888aI7Drw18NbAg4GX4oq/AW4Ffhr4aWCX/xiIF91rA7/F8xL/eyFedK8N/BbPS/zvhXjRvTbwWzwv8b8X4kX32sBv8bzE/16IF91rA7/F8xL/eyFedK8N/BbPS/zHeDDwIOC1ueLBwIO54lbgVq74beAZwK38+yFedK8N/BbPS/zbvBbw2sBrA6/Nv81vA78N/DbwO/zrIV50rw38Fs9LvOjeCnhv4LWB4/zH2gV+Gvhp4Gd40SD+8x0HPgp4b+DB/Ne4Ffhu4GuAXV4wxH+uzwI+GjjOf49d4KuBz+H5Q/zneGngu4CX5n+GvwbeBriV54T4j/fSwG8Bx/mfZRd4HeCveTbEf6wHA38FHOd/pl3gZYBbuQLxH+ungbfif7bvAd6bKxD/cR4MPJ3/HR4C3Aog/uO8NvBb/O/wOsBvA4j/OK8N/Bb/O7wO8NsA4j/OawO/xf8OrwP8NoD4j/PawG/xv8PrAL8NIP7jvDbwW/zv8DrAbwOI/zivDfwW/zu8DvDbAOI/znHgpfnf4a+BXQDx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hFvK3tBDkHnjgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonAdd;
impl IconShape for MdPersonAdd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm-9-2V7H4v3H1v2h3v3h2v-3h3v-2H6zm9 4c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP89r8Zx+h/95EP+x3hp4L+Ctef5+Gvge4Kf5nwHxH+Olga8CXpsXzW8DHwP8Nf+9EP9+Lw38FnCcf51d4HWAv+a/D+Lf56WB3wKO82+zC7wO8Nf890D82x0H/gp4MP8+twIP4b8H4t/us4HP4vn7HuCrgb/mipcGPhp4L56/zwE+m/96iH+7i8Bxntf7AN/N8/fewHfxvHaBE/zXQ/zbvDbwWzyvzwE+mxfus4HP4nm9DPDX/Ou9Fs/rb4Bd/mWIf5vPBj6L53UC2OWFOw5c5Hl9DvDZ/OuZ5/U6wG/zL0P823w28Fk8p98BXpsXzW8Dr8Vz+hzgs/nXM8/rdYDf5l+G+Lf5aeCteE6/A7w2L5rfBl6L5/QzwFvzr2ee1+sAv82/DPFv89nAZ/Gc/hp4GV40fwW8NM/pc4DP5l/PPK/XAX6bfxni3+ajga/ieT0EuJUX7sHA03leHwN8Nf965nm9DvDb/MsQ/zYPBp7O8/pt4HV44X4LeG2e10OAW/nXM8/rdYDf5l+G+Lf7beC1eF7fDXwMsMtzOg58F/DWPK/fAV6bfxvzvF4H+G3+ZYh/u9cGfovnbxf4buCvueKlgfcGjvP8vQ7w2zx/r8UL99s8r48G/poX7G+AXQDx7/PdwHvx7/M9wHvzgpn/eK8D/DaA+Pc5Dvw28FL82/wN8NrALi+Y+Y/3OsBvA4h/v+PATwOvxb/O7wBvDezywpn/eK8D/DaA+I/z0cBnA8d44S4Bnw18NS8a8x/vdYDfBhD/sY4Dbw28NfBg4KW44m+AW4GfBn4a2OVFZ/7jvQ7w2wDifz/zvF4H+G3+ZYj//czzeh3gt/mXIf73M8/rdYDf5l+G+N/PPK/XAX6bfxnifz/zvF4H+G3+ZYj/OA8GHgS8Nlc8GHgwV9wK3MoVvw08A7iV/xjmeb0O8Nv8yxD/dq8FvDbw2sBr82/z28BvA78N/A7/NuZ5vQ7w2/zLEP86bwW8N/DawHH+Y+0CPw38NPAzvOjM83od4Lf5lyH+ZceBjwLeG3gw/zVuBb4b+Bpgl/88iBfus4CPBo7z32MX+Grgc/jPgXj+Xhr4LuCl+de7BPw1z99LA8f41/tr4G2AW/mPhXheLw38FnCcf9kl4LeBvwZ+G/hrYJcX7jjw0sBrAy8NvDZwjH/ZLvA6wF/zHwfxnB4M/BVwnBfsEvDTwE8DP81/jLcG3hp4L164XeBlgFv5j4F4Tj8NvBXP3yXgs4HvBnb5z3Ec+Gjgo4FjPH/fA7w3/zEQz/Zg4Ok8f58DfDWwy3+N48BnAx/F8/cQ4Fb+/RDP9trAb/G83gf4bv57vDfwXTyv1wF+m38/xLO9NvBb/O/wOsBv8++HeLbXBn6L/x1eB/ht/v0Qz/bawG/xv8PrAL/Nvx/i2V4b+C3+d3gd4Lf590M822sDv8X/Dq8D/Db/fohnOw68NP87/DWwy78f4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjvNfAQUQ/lYIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonAddAlt;
impl IconShape for MdPersonAddAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13,8c0-2.21-1.79-4-4-4S5,5.79,5,8s1.79,4,4,4S13,10.21,13,8z M11,8c0,1.1-0.9,2-2,2S7,9.1,7,8s0.9-2,2-2S11,6.9,11,8z M1,18v2h16v-2c0-2.66-5.33-4-8-4S1,15.34,1,18z M3,18c0.2-0.71,3.3-2,6-2c2.69,0,5.78,1.28,6,2H3z M20,15v-3h3v-2h-3V7h-2v3h-3v2 h3v3H20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP89r8Zx+h/95EP+x3hp4L+Ctef5+Gvge4Kf5nwHxH+Olga8CXpsXzW8DHwP8Nf+9EP9+Lw38FnCcf51d4HWAv+a/D+Lf56WB3wKO82+zC7wO8Nf890D82x0H/gp4MP8+twIP4b8H4t/us4HP4j/G5wCfzX89xL/dReA4/zF2gRP810P827w28Fv8x3oZ4K/513stntffALv8yxD/Np8NfBb/sT4H+Gz+9czzeh3gt/mXIf5tPhv4LP5jfQ7w2fzrmef1OsBv8y9D/Nv8NPBW/Mf6GeCt+dczz+t1gN/mX4b4t/ls4LP4j/U5wGfzr2ee1+sAv82/DPFv89HAV/Ef62OAr+Zfzzyv1wF+m38Z4t/mwcDT+Y/1EOBW/vXM83od4Lf5lyH+7X4beC3+Y/wO8Nr825jn9TrAb/MvQ/zbvTbwW/zHeB3gt3n+XosX7rd5Xh8N/DUv2N8AuwDi3+e7gffi3+d7gPfmBTP/8V4H+G0A8e9zHPht4KX4t/kb4LWBXV4w8x/vdYDfBhD/fseBnwZei3+d3wHeGtjlhTP/8V4H+G0A8R/no4HPBo7xwl0CPhv4al405j/e6wC/DSD+Yx0H3hp4a+DBwEtxxd8AtwI/Dfw0sMuLzvzHex3gtwHE/37meb0O8Nv8yxD/+5nn9TrAb/MvQ/zvZ57X6wC/zb8M8b+feV6vA/w2/zLE/37meb0O8Nv8yxD/cR4MPAh4ba54MPBgrrgVuJUrfht4BnAr/zHM83od4Lf5lyH+7V4LeG3gtYHX5t/mt4HfBn4b+B3+bczzeh3gt/mXIf513gp4b+C1geP8x9oFfhr4aeBneNGZ5/U6wG/zL0P8y44DHwW8N/Bg/mvcCnw38DXALv95EC/cZwEfDRznv8cu8NXA5/CfA/H8vTTwXcBL8z/DXwNvA9zKfyzE83pp4LeA4/zPsgu8DvDX/MdBPKcHA38FHOd/pl3gZYBb+Y+BeE4/DbwV/7N9D/De/MdAPNuDgafzv8NDgFv590M822sDv8X/Dq8D/Db/fohne23gt/jf4XWA3+bfD/Fsrw38Fv87vA7w2/z7IZ7ttYHf4n+H1wF+m38/xLO9NvBb/O/wOsBv8++HeLbXBn6L/x1eB/ht/v0Qz3YceGn+d/hrYJd/P8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RwHdkkHIvRwAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonAddAlt1;
impl IconShape for MdPersonAddAlt1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13,8c0-2.21-1.79-4-4-4S5,5.79,5,8s1.79,4,4,4S13,10.21,13,8z M15,10v2h3v3h2v-3h3v-2h-3V7h-2v3H15z M1,18v2h16v-2 c0-2.66-5.33-4-8-4S1,15.34,1,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv89r8Zx+h/96iP9abw28F/DWPH8/DXwP8NP810D813hp4KuA1+ZF89vAxwB/zX8uxH++lwZ+CzjOv84u8DrAX/OfB/Gf66WB3wKO82+zC7wO8Nf850D85zkO/BXwYJ6/S8Bfc8VLA8d4/m4FHsJ/DsR/ns8GPovndQn4aOC7eU7vDXw1cIzn9TnAZ/MfD/Gf5yJwnOf0N8BrA7s8f8eBvwYexHPaBU7wHw/xn+O1gd/ieb0M8Ne8cK8N/BbP62WAv+Y/FuI/x2cDn8Vz+hngrXnR/DTwVjynzwE+m/9YiP8cnw18Fs/pc4DP5kXz2cBn8Zw+B/hs/mMh/nP8NPBWPKfPAT6bF81nA5/Fc/oZ4K35j4X4z/HZwGfxnL4G+GheNF8NfBTP6XOAz+Y/FuI/x0cDX8VzuhV4CC+apwMP5jl9DPDV/MdC/Od4MPB0ntfHAF/NC/fRwFfxvB4C3Mp/LMR/nt8GXovn9T7Ad/P8vTfwXTyv3wFem/94iP88rw38Fs/fTwPfDfwNV7wU8N7AW/P8vQ7w2/zHQ/zn+m7gvfj3+R7gvfnPgfjPdRz4beCl+Lf5G+C1gV3+cyD+8x0Hfhp4Lf51fgd4a2CX/zyI/zofDXw2cIwX7hLw2cBX858P8V/rOPDWwFsDDwZeiiv+BrgV+Gngp4Fd/msg/n9D/P+G+K93HHgpnr+/AXb5r4P4z/XSwGsBLw08GHhtXjS/DdwK/DXwO8Bf858D8R/vrYC3Bt4aOM5/jF3gp4GfBn6G/ziI/xgPBj4LeGvgOP+5doGfBj4HuJV/H8S/z4OBzwLem/8e3w18DnAr/zaIf7vPAj6bf5tnALfynB4MPIh/m88GPod/PcS/3nHgp4DX5kXzO8BvA78N3Arcygv3YODBwGsDrw28Fi+a3wbeBtjlRYf41/sr4KV54X4G+Gngp4Fd/n2OA28NvDXwVrxwfw28DC86xL/OZwOfxfN3Cfhq4KuBXf5zHAc+Gvho4BjP3+cAn82LBvGvcxE4zvP6HOCrgV3+axwHPhr4LJ7XLnCCFw3iRffawG/xvF4H+G3+e7w18FM8r9cBfpt/GeJF99rAb/Gcfgd4bf57/TbwWjyn1wF+m38Z4kX32sBv8Zx+B3ht/nv9NvBaPKfXAX6bfxniRffawG/xnH4HeG3+e/028Fo8p9cBfpt/GeJF99rAb/GcdoG/5r/XSwPHeU6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86I4DL83/Dn8N7PIvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BDrCnUHXXvkyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonOutline;
impl IconShape for MdPersonOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9+DgQfxbJeAv+Z/BsR/juPAewHvDbw0z+tW4LeBzwFu5b8P4j/eWwPfBRznRfPVwMfw3wPxH+urgI/mX++vgdcBdvmvhfiP89HAV/Fv99fAy/BfC/Ef47WB3+Lf72uAj+a/DuI/xm8Br81/jIcAt/JfA/Hv99bAT/Ef53uA9+a/BuLf76uBj+I/zq3AQ/ivgfj3+23gtfiP9TLAX/OfD/HvZ/7jvQ7w2/znQ/z7mf94rwP8Nv/5EP9+twIP4j/W6wC/zX8+xL/fbwOvxX+sE8Au//kQ/34fDXwV/3H+Bnhp/msg/v0eDDyd/zgfA3w1/zUQ/zG+G3gv/v2eAbw0sMsL99LAMf7t/gbYBRD/MY4DtwLH+Pd5G+Cn+Zf9NvBa/Nu9DvDbAOI/zksDf8W/3ecAn82L5reB1+Lf7nWA3wYQ/7FeGvht4Bj/Oh8DfDUvut8GXot/u9cBfhtA/Mc7Dnw18F78y34H+Gzgt/nX+W3gtfi3ex3gtwHEf54HA28NvDXwYOBBXPE7wF8DPw38Nv82vw28Fv92rwP8NoD4/w3x/xvi/zfE/2+I/98Q/zleCzgOvDRXvDRwnCt+myt2gb8G/gbY5b8H4t/vOPBWwGsDLw28NP96twJ/Dfw28DvAX/NfA/Fvcxx4L+C9gZfmP96twE8D3wP8Nf95EP86DwY+C3hv/uv8NvDdwPfwHw/xonkw8FnAe/Pf51bgfYDf5j8O4l/23sBXAcf5n+GrgY/hPwbihXtv4Lv4n+evgdcBdvn3Qbxg7w18F/9z/TbwOvz7IJ6/48DTgeP8z/Y+wHfzb4d4/j4a+Cr+5/sZ4K35t0M8f18NfBT/O4h/O8Tz99vAa/G/g/i3Qzx/vw28Fv87iH87xPP328Br8b+D+LdDPH+/DbwW/zuIfzvE8/fbwGvxv4P4t0M8fy8NHOd/h9/m3w7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hE7M2VBb0IY6gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonRemove;
impl IconShape for MdPersonRemove {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9+DgQfxbJeAv+Z/BsR/juPAewHvDbw0z+tW4LeBzwFu5b8P4j/eWwPfBRznRfPVwMfw3wPxH+urgI/mX++vgdcBdvmvhfiP89HAV/Fv99fAy/BfC/Ef47WB3+Lf72uAj+a/DuI/xm8Br81/jIcAt/JfA/Hv99bAT/Ef53uA9+a/BuLf76uBj+I/zq3AQ/ivgfj3+23gtfiP9TLAX/OfD/HvZ/7jvQ7w2/znQ/z7mf94rwP8Nv/5EP9+twIP4j/W6wC/zX8+xL/fbwOvxX+sE8Au//kQ/34fDXwV/3H+Bnhp/msg/v0eDDyd/zgfA3w1/zUQ/zG+G3gv/v2eAbw0sMsL99LAMf7t/gbYBRD/MY4DtwLH+Pd5G+Cn+Zf9NvBa/Nu9DvDbAOI/zksDf8W/3ecAn82L5reB1+Lf7nWA3wYQ/7FeGvht4Bj/Oh8DfDUvut8GXot/u9cBfhtA/Mc7Dnw18F78y34H+Gzgt/nX+W3gtfi3ex3gtwHEf54HA28NvDXwYOBBXPE7wF8DPw38Nv82vw28Fv92rwP8NoD4/w3x/xvi/zfE/2+I/98Q/zleCzgOvDRXvDRwnCt+myt2gb8G/gbY5b8H4t/vOPBWwGsDLw28NP96twJ/Dfw28DvAX/NfA/Fvcxx4L+C9gZfmP96twE8D3wP8Nf95EP86DwY+C3hv/uv8NvDdwPfwHw/xonkw8FnAe/Pf51bgfYDf5j8O4l/23sBXAcf5n+GrgY/hPwbihXtv4Lv4n+evgdcBdvn3Qbxg7w18F/9z/TbwOvz7IJ6/48DTgeP8z/Y+wHfzb4d4/j4a+Cr+5/sZ4K35t0M8f18NfBT/O4h/O8Tz99vAa/G/g/i3Qzx/vw28Fv87iH87xPP328Br8b+D+LdDPH+/DbwW/zuIfzvE8/fbwGvxv4P4t0M8fy8NHOd/h9/m3w7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hE7M2VBb0IY6gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonRemoveAlt1;
impl IconShape for MdPersonRemoveAlt1 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M17,10v2h6v-2H17z M2,18v2h16v-2c0-2.66-5.33-4-8-4 S2,15.34,2,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//s9GHgt4ATw1fzrIP73eWngtYDXBl4bOM4VvwO8Nv86iP/5Xgt4beClgdcGjvP8/Q7w2vzrIP7nemngr3jR/Q7w2vzrIP7nem3gt3jR/Q7w2vzrIP7nem3gt3jR/Q7w2vzrIP7nem3gt3hezwCOA8d4Tr8DvDb/Ooj/uV4b+G7gr4G/Bn4b+GtgF/ht4LV4Tr8DvDb/Ooj/HF8NvBTP6W+Aj+Y/xm8Dr8Vz+h3gtfnXQfzn+G3gtXhOvwO8Nv8xfht4LZ7T7wCvzb8O4j/HbwOvxXP6HeC1+Y/x28Br8Zx+B3ht/nUQ/zl+G3gtntPvAK/Nf4zfBl6L5/Q7wGvzr4P4z/HbwGvxnH4HeG3+Y/w28Fo8p98BXpt/HcR/jt8GXovn9DvAa/Mf47eB1+I5/Q7w2vzrIP5z/DbwWjyn3wFem/8Yvw28Fs/pd4DX5l8H8Z/jt4HX4jn9DvDa/Mf4beC1eE6/A7w2/zqI/xy/DbwWz+l3gNfmP8ZvA6/Fc/od4LX510H85/ht4LV4Tr8DvDb/MX4beC2e0+8Ar82/DuI/x28Dr8Vz+h3gtfmP8dvAa/Gcfgd4bf51EC+61wZ+i/8arwP8Ni/YbwOvxXP6HeC1+ddBvOheG/gt/mu8DvDbvGC/DbwWz+l3gNfmXwfxontt4Lf4r/E6wG/zgv028Fo8p98BXpt/HcSL7rWB3+K/xusAv80L9tvAa/Gcfgd4bf51EC+61wZ+i/8arwP8Ni/YbwOvxXP6HeC1+ddBvOiOAy/Ni+argZfiOf0N8NG8aP4a2OUF+23gtXhOvwO8Nv86iP8cvw28Fs/pd4DX5j/GbwOvxXP6HeC1+ddB/Of4beC1eE6/A7w2/zF+G3gtntPvAK/Nvw7iP8dvA6/Fc/od4LX5j/HbwGvxnH4HeG3+dRD/OX4beC2e0+8Ar81/jN8GXovn9DvAa/Ovg/jP8dvAa/Gcfgd4bf5j/DbwWjyn3wFem38dxH+O3wZei+f0O8Br8x/jt4HX4jn9DvDa/Osg/nP8NvBaPKffAV6b/xi/DbwWz+l3gNfmXwfxn+O3gdfiOf0O8Nr8x/ht4LV4Tr8DvDb/Ooj/HL8NvBbP6XeA1+Y/xm8Dr8Vz+h3gtfnXQfzn+G3gtXhOvwO8Nv8xfht4LZ7T7wCvzb8O4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CMCJg0G0ZmVaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlusOne;
impl IconShape for MdPlusOne {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 8H8v4H4v2h4v4h2v-4h4v-2h-4zm4.5-1.92V7.9l2.5-.5V18h2V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+xtgl+d0HHgp/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mXnRXAL+muf1Wjyv1wF+m+f02sBv8R9PvGCIf5l50fwO8No8L/O8Xgf4bZ7TawO/xX888YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fv/xxAuG+JeZF83vAK/N8zLP63WA3+Y5vTbwW/zHEy8Y4l9mXjS/A7w2z8s8r9cBfpvn9NrAb/EfT7xgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8V/PPGCIf5l5kXzO8Br87zM83od4Ld5Tq8N/Bb/8cQLhviXmRfN7wCvzfMyz+t1gN/mOb028Fv8xxMvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvH6H5/XSwDFeNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi+R3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8WLRrxgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8XzEs/rt4HX4kUjXjDEv8y8aH4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxohEvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvMTz+m3gtXjRiBcM8S8zL5rfAV6b52We1+sAv81zem3gt3he4nn9NvBavGjEC4b4l5kXze8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC1eNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi2QX+muf12jyvvwZ2eU7HgZfmef02z+ulgeO8aMQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj3ZSqQTTQwmcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPoll;
impl IconShape for MdPoll {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFNklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HSwFsBr80VDwYezBW/zRW3Aj8N/Az/eRD/dY4DnwW8NfBgXnS7wG8DXwP8Nv+xEP/5jgMfBXw0cJx/n58GPga4lf8YiP9cLw38FnCc/1ifDXwO/36I/zzvDXwX/3m+G/gYYJd/O8R/jq8GPor/fH8NvA6wy78N4j/eewPfxX+dnwbehn8bxH+s1wZ+i/96XwN8NP96iP84x4G/Ah7Mv84JYJfn9NPAW/Gv8zrAb/Ovg/iP89nAZ/Gv8zfAS/O8Phv4LP51bgUewr8O4j/Gg4G/Ao7zr/M7wGvzvN4b+C7+9d4H+G5edIj/GJ8NfBYv3NcAfw18NPBSwDOA9wZ+m+d1HPhu4K2AZwDfDfw18FO8cLcCD+FFh/iP8XTgwbxgzwBeGtjlipcG/pp/2UsDf82zfTfwXrxwLwP8NS8axL/fSwN/xQv3NsBP8+93HLgVOMYL9jXAR/OiQfz7fTbwWbxgvwO8Nv+y48Au/7KPBr6KF+yvgZfhRYP49/tu4L14wd4H+G5esI8CPhs4zhVfDXwOsMvzdxy4yAsnXjSIf7/fBl6LF2wX+Bjgu3leHw18Fc/rt4HX4Xm9NPBdwEvzwp0AdvmXIf79LgLH+Zd9DvDZPKeLwHGev9cBfptne23gp4Dj/MteB/ht/mWIf5+vBj6KF83vAK/Nsx0HLvKCfQzw1TzbawO/xYvmdYDf5l+G+Ld7a+CneNH9DPDWPCfzgr0N8NM821sDP8WL5nWA3+Zfhvi3eTDwV8BxXjTfA3w0sMtz+mrgo3hezwBeGtjlOb038F38y14H+G3+ZYh/m78CXpoXzdcAH83zdxz4aeC1eLZnAG8N/DXP30sDf8UL9xDgVv5liH+9rwY+ihfd5wCfzQv32sBLA7cCvw3s8oK9NvBbvGDPAB7Miwbxr/PWwE/xr/cQ4Fb+Y/wV8NK8YD8DvDUvGsSL7jjwdOA4/3q/DbwO/37vDXwXL9z7AN/Niwbxontt4Lf4t3sd4K+B9wI+Gvhu4GuAXZ6/9wI+G/ht4GuAW4GnA8d5wS4Bx3nRIf51Pht4bZ7Ta/Fv9zPAW/O8Xhv4Lf71vgd4b150iH8/82/3O8Br87w+G/gs/nUuAQ8GdnnRIf79zL+PeF7fDbwX/zqfA3w2/zqIfz/z7/PXwC7P6aWB47zo/gZ4af71EP9+5r/XJeClgVv510P8+5n/PpeA1wb+mn8bxL+f+e9xCXht4K/5t0P8+5n/en8DvDVwK/8+iH8/81/nEvDVwGfzHwPx77cLHOM/3zOAlwZ2+Y+D+Pf7buC9+M/3O8Br8x8L8e/3YODp/Of7HeC1+Y+F+I/x3sB38Z/rd4DX5j8W4j/Og4HPBt4aOMZ/vN8BXpv/WIj/fOY/xu8Ar81/LMR/PvMf43eA1+Y/FuI/n/mP8TvAa/MfC/Gfz/zH+B3gtfmPhfjPZ/5j/A7w2vzHQvznM/8xfgd4bf5jIf7zmf8YvwO8Nv+xEP/5zH+M3wFem/9YiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjurC+QWv8MY0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPsychology;
impl IconShape for MdPsychology {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13,8.57c-0.79,0-1.43,0.64-1.43,1.43s0.64,1.43,1.43,1.43s1.43-0.64,1.43-1.43S13.79,8.57,13,8.57z",
            }
            path {
                d: "M13,3C9.25,3,6.2,5.94,6.02,9.64L4.1,12.2C3.85,12.53,4.09,13,4.5,13H6v3c0,1.1,0.9,2,2,2h1v3h7v-4.68 c2.36-1.12,4-3.53,4-6.32C20,6.13,16.87,3,13,3z M16,10c0,0.13-0.01,0.26-0.02,0.39l0.83,0.66c0.08,0.06,0.1,0.16,0.05,0.25 l-0.8,1.39c-0.05,0.09-0.16,0.12-0.24,0.09l-0.99-0.4c-0.21,0.16-0.43,0.29-0.67,0.39L14,13.83c-0.01,0.1-0.1,0.17-0.2,0.17h-1.6 c-0.1,0-0.18-0.07-0.2-0.17l-0.15-1.06c-0.25-0.1-0.47-0.23-0.68-0.39l-0.99,0.4c-0.09,0.03-0.2,0-0.25-0.09l-0.8-1.39 c-0.05-0.08-0.03-0.19,0.05-0.25l0.84-0.66C10.01,10.26,10,10.13,10,10c0-0.13,0.02-0.27,0.04-0.39L9.19,8.95 c-0.08-0.06-0.1-0.16-0.05-0.26l0.8-1.38c0.05-0.09,0.15-0.12,0.24-0.09l1,0.4c0.2-0.15,0.43-0.29,0.67-0.39l0.15-1.06 C12.02,6.07,12.1,6,12.2,6h1.6c0.1,0,0.18,0.07,0.2,0.17l0.15,1.06c0.24,0.1,0.46,0.23,0.67,0.39l1-0.4c0.09-0.03,0.2,0,0.24,0.09 l0.8,1.38c0.05,0.09,0.03,0.2-0.05,0.26l-0.85,0.66C15.99,9.73,16,9.86,16,10z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPublic;
impl IconShape for MdPublic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPublicOff;
impl IconShape for MdPublicOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11,8.17L6.49,3.66C8.07,2.61,9.96,2,12,2c5.52,0,10,4.48,10,10c0,2.04-0.61,3.93-1.66,5.51l-1.46-1.46 C19.59,14.87,20,13.48,20,12c0-3.35-2.07-6.22-5-7.41V5c0,1.1-0.9,2-2,2h-2V8.17z M21.19,21.19l-1.41,1.41l-2.27-2.27 C15.93,21.39,14.04,22,12,22C6.48,22,2,17.52,2,12c0-2.04,0.61-3.93,1.66-5.51L1.39,4.22l1.41-1.41L21.19,21.19z M11,18 c-1.1,0-2-0.9-2-2v-1l-4.79-4.79C4.08,10.79,4,11.38,4,12c0,4.08,3.05,7.44,7,7.93V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRecommend;
impl IconShape for MdRecommend {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm6 9.8a.9.9 0 0 1-.1.5l-2.1 4.9a1.34 1.34 0 0 1-1.3.8H9a2 2 0 0 1-2-2v-5a1.28 1.28 0 0 1 .4-1L12 5l.69.69a1.08 1.08 0 0 1 .3.7v.2L12.41 10H17a1 1 0 0 1 1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReduceCapacity;
impl IconShape for MdReduceCapacity {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16,4c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S16,5.1,16,4z M20.78,7.58C19.93,7.21,18.99,7,18,7c-0.67,0-1.31,0.1-1.92,0.28 C16.66,7.83,17,8.6,17,9.43V10h5V9.43C22,8.62,21.52,7.9,20.78,7.58z M6,6c1.1,0,2-0.9,2-2S7.1,2,6,2S4,2.9,4,4S4.9,6,6,6z M7.92,7.28C7.31,7.1,6.67,7,6,7C5.01,7,4.07,7.21,3.22,7.58C2.48,7.9,2,8.62,2,9.43V10h5V9.43C7,8.6,7.34,7.83,7.92,7.28z M10,4 c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S10,5.1,10,4z M16,10H8V9.43C8,8.62,8.48,7.9,9.22,7.58C10.07,7.21,11.01,7,12,7 c0.99,0,1.93,0.21,2.78,0.58C15.52,7.9,16,8.62,16,9.43V10z M15,16c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S15,17.1,15,16z M21,22h-8 v-0.57c0-0.81,0.48-1.53,1.22-1.85C15.07,19.21,16.01,19,17,19c0.99,0,1.93,0.21,2.78,0.58C20.52,19.9,21,20.62,21,21.43V22z M5,16 c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S5,17.1,5,16z M11,22H3v-0.57c0-0.81,0.48-1.53,1.22-1.85C5.07,19.21,6.01,19,7,19 c0.99,0,1.93,0.21,2.78,0.58C10.52,19.9,11,20.62,11,21.43V22z M12.75,13v-2h-1.5v2H9l3,3l3-3H12.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveModerator;
impl IconShape for MdRemoveModerator {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.27 21.73l-3.54-3.55L5.78 5.23 2.27 1.72 1 2.99 3.01 5H3v6c0 5.55 3.84 10.74 9 12 2.16-.53 4.08-1.76 5.6-3.41L21 23l1.27-1.27zM13 9.92l6.67 6.67C20.51 14.87 21 12.96 21 11V5l-9-4-5.48 2.44L11 7.92l2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSanitizer;
impl IconShape for MdSanitizer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.5,6.5C15.5,5.66,17,4,17,4s1.5,1.66,1.5,2.5C18.5,7.33,17.83,8,17,8S15.5,7.33,15.5,6.5z M19.5,15 c1.38,0,2.5-1.12,2.5-2.5c0-1.67-2.5-4.5-2.5-4.5S17,10.83,17,12.5C17,13.88,18.12,15,19.5,15z M13,14h-2v-2H9v2H7v2h2v2h2v-2h2V14z M16,12v10H4V12c0-2.97,2.16-5.43,5-5.91V4H7V2h6c1.13,0,2.15,0.39,2.99,1.01l-1.43,1.43C14.1,4.17,13.57,4,13,4h-2v2.09 C13.84,6.57,16,9.03,16,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/PSwFdxxccAf81/PcR/vQcDnwW8N8/pu4HPAW7lvw7iv85x4KOAz+aF+2zga4Bd/vMh/mt8FvDRwHFeNLvAVwOfw38uxH+u9wI+G3gw/za3Ap8NfA//ORD/OV4b+CrgpfmP8dfAxwC/zX8sxH+slwa+Cnht/nP8NvAxwF/zHwPxH+PBwGcB781/je8GPge4lX8fxL/PceCjgM/mv8dnA18D7PJvg/i3+yzgo4Hj/PfaBb4a+Bz+9RD/eu8FfDbwYP5nuRX4bOB7eNEhXnSvDXwV8NL8z/bXwMcAv82/DPEve2ngq4DX5n+X3wY+BvhrXjDEC/Zg4LOA9+Z/t+8GPge4leeFeF7HgY8CPpv/Wz4b+Bpgl2dDPKfPAj4aOM7/TbvAVwOfwxWI5/TZwEcDx/i/7XWA3wYQz+s48NHAZ/F/1+sAvw0gXrAHA58NvBf/97wO8NsA4l/20sBXA6/F/y6/A3w08Fc8r9cBfhtAvOheG/hq4KX4n+1vgI8GfpsrzPN6HeC3AcS/3nsDnw08iP9ZngF8NvDdPCfzvF4H+G0A8W/32cBHA8f473UJ+Grgs3n+zPN6HeC3AcS/z3Hgo4HP4r/H5wBfDezygpnn9TrAbwOI/xgPBj4beC/+a3wP8NnArfzLzPN6HeC3AcR/rJcGvhp4Lf5z/A7w0cBf86Izz+t1gN8GEP85Xhv4auCl+I/xN8BHA7/Nv555Xq8D/DaA+Lf5bOC3gd/mhXtv4LOBB/Fv8wzgs4Hv5oV7beC1gc/meZnn9TrAbwOIf5vfBl4L+GngY4BbeeE+G/ho4BgvmkvAVwOfzQv3YOCrgLcGfgd4bZ6XeV6vA/w2gPi3+W3gtXi27wY+BtjlBTsOfDTwWbxwnwN8NbDLC3Yc+CrgvXm23wFem+dlntfrAL8NIP5tfht4LZ7TLvDVwNcAu7xgDwY+G3gvntP3AJ8N3MoLdhz4KOCjgeM8p98BXpvnZZ7X6wC/DSD+bX4beC2ev13go4Hv4YV7aeCrueKjgb/mhXsv4KuB4zx/vwO8Ns/LPK/XAX4bQPzb/DbwWrxwtwLvA/w2/z5vDXwV8GBeuN8BXpvnZZ7X6wC/DSD+bX4beC1eNL8NfA7w2/zrvDbwWcBr86L5HeC1eV7meb0O8NsA4t/mt4HX4l/nu4HPAW7lhXsw8FXAW/Ov8zvAa/O8zPN6HeC3AcS/zW8Dr8W/zVcDnwPs8pyOA18FvDf/Nr8DvDbPyzyv1wF+G0D82/w28Fr82+0CXw18DVd8FPDRwHH+7X4HeG2el3lerwP8NoD4t/lp4K3497uVKx7Mv9/PAG/N83ptntdfA7sA4t/us4GPBo7x3+sS8NXAZ/Ovh/j3OQ58NPBZ/Pf4HOCrgV3+bRD/MR4MfDbwXvzX+B7gs4Fb+fdB/Md6aeCrgdfiP8fvAB8N/DX/MRD/OV4b+GrgpfiP8TfARwO/zX8sxH+u9wY+G3gQ/zbPAD4b+G7+cyD+a3w28NHAMV40l4CvBj6b/1yI/zrHgY8GPosX7nOArwZ2+c+H+K/3YOCzgffiOX0P8NnArfzXQfz3eWngq7nio4G/5r8e4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8ITHTRQSzsd4kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSchool;
impl IconShape for MdSchool {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 13.18v4L12 21l7-3.82v-4L12 17l-7-3.82zM12 3L1 9l11 6 9-4.91V17h2V9L12 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9dLA8f493kGcCv/ORD/eR4M/BVwnH+fXeBlgFv5j4f4z3McuMh/jBPALv/xEP+53hv4Lv593gf4bv5zIP7zfTfwXvzbfA/w3vznQfzX+GvgpfjX+RvgpfnPhfivcRy4FTjGi+YS8GBgl/9ciP86bw38FC+atwF+mv98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86rw38Fi+a1wF+m/98iP86Hw18FS+ajwG+mv98iP8aLw38FnCcF80u8DrAX/OfC/Gf7zjwW8BL86/z18DrALv850H85/su4L35t/lu4H34z4P4z/XewHfx7/M+wHfznwPxn+elgd8CjvPvswu8DvDX/MdD/Oc4DvwW8NL8x/hr4HWAXf5jIf5zfBfw3vzH+m7gffiPhfiP997Ad/Gf432A7+Y/DuI/1ksDvwUc5z/HLvA6wF/zHwPxH+c48FvAS/Of66+B1wF2+fdD/Mf5LuC9+a/x3cD78O+H+I/x3sB38V/rfYDv5t8H8e/30sBvAcf5r7ULvA7w1/zbIf59jgO/Bbw0/z3+GngdYJd/G8S/z3cB781/r+8G3od/G8S/3XsD38X/DO8DfDf/eoh/m5cGfgs4zv8Mu8DrAH/Nvw7iX+848FvAS/M/y18DrwPs8qJD/Ot9F/De/M/03cD78KJD/Ou8N/Bd/M/2PsB386JBvOiOA08HjvM/2y7wEGCXfxniRffawG/xv8PrAL/Nvwzxontp4K/43+EhwK38yxD/Ol8NfBT/s30N8NG8aBD/ei8NHOd/pl3gr3nRIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AmTpZUGrlzfPAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScience;
impl IconShape for MdScience {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.8,18.4L14,10.67V6.5l1.35-1.69C15.61,4.48,15.38,4,14.96,4H9.04C8.62,4,8.39,4.48,8.65,4.81L10,6.5v4.17L4.2,18.4 C3.71,19.06,4.18,20,5,20h14C19.82,20,20.29,19.06,19.8,18.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8+DgQdxxTOAW/mvh/ivdxz4LuCteU4/DbwPsMt/HcR/rePA04HjPH+7wEOAXf5rIP5r/RTw1rxwPw28Df81EP91Hgw8nRfNQ4Bb+c+H+K/z2sBv8aJ5HeC3+c+H+K/z2sBv8aJ5HeC3+c+H+K/zYODpvGgeAtzKfz7Ef62fBt6KF+5ngLfmvwbiv9Zx4FbgGM/fJeDBwC7/NRD/9Y4D3w28Fc/pZ4D3Bnb5r4P47/Ng4MFccStwK//1EP+/If5/Q/zXOg58FPDewIN5TrcC3w18DbDLfw3Ef52XBn4LOM4Ltwu8DvDX/OdD/Nc4DvwV8GBeNLcCLwPs8p8L8V/jr4CX5l/nr4HXAXb5z4P4z/ddwHvzb/PdwPvwnwfxn+ujga/i3+dzgM/mPwfiP897A9/Ff4z3Ab6b/3iI/xzvDXwX/7HeB/hu/mMh/uO9N/Bd/Od4H+C7+Y+D+I/1WcBn85/rs4HP4T8G4j/GawNfBbw0z98l4LOBr+JF8zHAZwPHeP7+GvgY4Lf590H8+7w08FHAe/OCXQJeGzgO/BYvmtcBdoHfBo7xgn038DXAX/Nvg/jXe2ngtYD3Bl6aF+4S8NrAXwOvDfwWL5rXAX4beGngt4FjvHB/DXw38DvAX/OiQ/zLfosrjgMvzYvub4C3Bm7litcGfosXzesAv80VLw18N/BSvOj+GtjlitfhBUP8y8y/3tcAH81zem3gt3jRvA7w2zzbceCzgY/iX0+8YIh/mXnR/Q7w2cBv87xeG/gtXjSvA/w2z+u1gc8GXosXnXjBEP8y8y/7HeCzgd/meT0Y+CjgvYHjvGh2ge8Gvga4lef12sBnA6/Fv0y8YIh/mXn+fgf4beC7gVt5Xu8FvDfw2vz7/Dbw3cD38LweDLw38NrAa/H8iRcM8S/7aeCvueJW4Fbgt3n+Hgx8FPDewHH+Y+0C3w18DXArz99rAw8GHswVLw28NS8Y4j/GewHvDbw2/zV+G/hu4Hv490H8+7wX8NXAcf577AIfDXwP/zaIf7vPBj6Lf72fAf4a+Gtglxfuo4G34l/2McBX86+H+Ld5MPB0/nX+Bnhr4Fb+dcyL5iHArfzrIP5tPhv4LF50vwO8NbDLv857A9/Fi+ZzgM/mXwfxb/PTwFvxorkEPBjY5YrXBj4LeG3+Y/0M8Nb86yD+bT4b+CxeNF8DfDRXvDfwXfzn+Bzgs/nXQfzbvDbwW7xoXgb4a664CBznP8frAL/Nvw7i3+67gffiXyaueG3gt/jP8T3Ae/Ovh/j3+Wzgs3jhxBWvDfwW//E+B/hs/m0Q/37HgZfmBfttrjgOvDT/sf4a2OXfDvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+Edppo0HKloMeAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSelfImprovement;
impl IconShape for MdSelfImprovement {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cy: "6",
                r: "2",
            }
            path {
                d: "M21,16v-2c-2.24,0-4.16-0.96-5.6-2.68l-1.34-1.6C13.68,9.26,13.12,9,12.53,9h-1.05c-0.59,0-1.15,0.26-1.53,0.72l-1.34,1.6 C7.16,13.04,5.24,14,3,14v2c2.77,0,5.19-1.17,7-3.25V15l-3.88,1.55C5.45,16.82,5,17.48,5,18.21C5,19.2,5.8,20,6.79,20H9v-0.5 c0-1.38,1.12-2.5,2.5-2.5h3c0.28,0,0.5,0.22,0.5,0.5S14.78,18,14.5,18h-3c-0.83,0-1.5,0.67-1.5,1.5V20h7.21 C18.2,20,19,19.2,19,18.21c0-0.73-0.45-1.39-1.12-1.66L14,15v-2.25C15.81,14.83,18.23,16,21,16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentDissatisfied;
impl IconShape for MdSentimentDissatisfied {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-3.5c.73 0 1.39.19 1.97.53.12-.14.86-.98 1.01-1.14-.85-.56-1.87-.89-2.98-.89-1.11 0-2.13.33-2.99.88.97 1.09.01.02 1.01 1.14.59-.33 1.25-.52 1.98-.52z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentNeutral;
impl IconShape for MdSentimentNeutral {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 15.5h6v1H9v-1z",
            }
            circle {
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentSatisfied;
impl IconShape for MdSentimentSatisfied {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-4c-.73 0-1.38-.18-1.96-.52-.12.14-.86.98-1.01 1.15.86.55 1.87.87 2.97.87 1.11 0 2.12-.33 2.98-.88-.97-1.09-.01-.02-1.01-1.15-.59.35-1.24.53-1.97.53z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentVeryDissatisfied;
impl IconShape for MdSentimentVeryDissatisfied {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-6c-2.33 0-4.32 1.45-5.12 3.5h1.67c.69-1.19 1.97-2 3.45-2s2.75.81 3.45 2h1.67c-.8-2.05-2.79-3.5-5.12-3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH2klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w2sAv8NVfsAn8N/A2wy38+xH+utwJeG3hr4MH86/w18NvAzwC/zX8OxH+848BHAe8NPJj/GLcC3w18DbDLfxzEf5zjwEcBHw0c5z/HLvDVwNcAu/z7If5jvBfw1cBx/mvsAu8D/DT/Poh/n+PAdwFvzb/O3wC7PKfjwEvxr/PTwPsAu/zbIP7tXhr4KeDB/Mt+Bvhp4LeBW3nhHgy8NvDWwFvxL7sVeBvgr/nXQ/zbvDTwW8BxXrBnAJ8N/DSwy7/NceCtgc8GHsQLtgu8DvDX/Osg/vVeGvgt4DjP3yXgs4Gv5j/WRwOfDRzj+dsFXgf4a150iH+dlwZ+CzjO8/c7wFsDu/znOA78NPBaPH+7wOsAf82LBvGiOw78FfBgnr/PAT6b/xqfDXwWz9+twMsAu/zLEC+6nwLemufvfYDv5r/WewPfxfP308Db8C9DvGjeG/gunr/3Ab6b/x7vDXwXz9/7AN/NC4f4lx0Hng4c53l9DvDZ/Pf6bOCzeF67wEOAXV4wxL/ss4HP4nn9DvDa/M/w28Br8bw+B/hsXjDEC3cceDpwnOd0CXgwsMv/DA8G/ho4xnPaBR4C7PL8IV64zwY+i+f1McBX8/wdB94LeGngVuB7gFv5t3lp4K2ABwN/DXwPsMvz99HAV/G8Pgf4bJ4/xAv3dODBPKdnAA/m+Xtp4KeAB/Oc3gf4bv513hv4Lp7TrcDbAH/N83cr8CCe063AQ3j+EC/YWwM/xfN6H+C7ef7+Cnhpnr+HALfyonlp4K94/v4aeBmev48Gvorn9TrAb/O8EC/YVwMfxfM6AezyvF4a+CtesM8BPpsXzWcDn8UL9jLAX/O8jgMXeV5fA3w0zwvxgv0V8NI8p58B3prn77WB3+IF+xrgo3nRfDfwXrxgrwP8Ns/fTwNvxXP6a+BleF6I5+/BwNN5Xu8DfDfP34OBp/OCfQzw1bxoPhr4Kl6wE8Auz99HA1/F8zoB7PKcEM/fawO/xfN6CHArL9hXAx/F83oG8NLALi+a48BfAw/ieX0N8NG8YA8Gns7zeh3gt3lOiOfvo4Gv4nmJF+448NnAR/FsfwO8N/DX/Ou8NPDdwEvxbF8DfDawywtnntfHAF/Nc0I8f58NfBbP6W+Al+ZFcxx4aWAX+Gv+fV4aOA78NbDLi+avgZfiOX0O8Nk8J8Tz99nAZ/Gcfgd4bf53+G3gtXhOnwN8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPPAG/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DPAW/OcEM/fZwOfxXP6HeC1+d/ht4HX4jl9DvDZPCfE8/fZwGfxnP4aeBn+d/gr4KV5Tp8DfDbPCfH8fTTwVTwv8b+DeV4fA3w1zwnx/L028Fs8r4cAt/I/24OBp/O8Xgf4bZ4T4vl7MPB0ntf7AN/N8/dg4Lv4r/E+wK08fx8NfBXP6wSwy3NCvGB/DbwUz+lngLfmBbsVeBD/uZ4BPJgX7KeBt+I5/Q3w0jwvxAv21cBH8bxOALs8fx8NfBX/uT4G+Gqev+PARZ7X1wAfzfNCvGBvDfwUz+t9gO/mBbsVeBD/OZ4BPJgX7KOBr+J5vQ7w2zwvxAt3K/AgntOtwEN4wd4a+Cn+c7wO8Nu8YE8HHsxzegbwYJ4/xAv32cBn8bw+BvhqXrCvBj6K/1hfA3w0L9hHA1/F8/oc4LN5/hAv3HHgVuAYz2kXeAiwywv23cB78R/je4D35gV7MPBXwHGe0yXgwcAuzx/iX/bZwGfxvH4beB1euO8G3ot/n+8B3psX7reA1+Z5fQ7w2bxgiH/ZceBW4BjP63OAz+aF+2zgs/i3+Rzgs3nhPhv4LJ7XJeDBwC4vGOJF897Ad/H8vQ/w3bxwDwY+G3gvXjTfA3w2cCsv3HsD38Xz9zbAT/PCIV50Pw28Fc/f+wDfzb/swcBbA68NPBh4Ka74G+BW4LeBnwZu5V/23sB38fz9DPDW/MsQL7rjwF8DD+L5+2zgc/iv8VnAZ/P8PQN4aWCXfxniX+elgd8GjvH8/TbwNsAu/zkeDHwX8No8f5eA1wb+mhcN4l/vpYHfBo7x/O0Cnw18Df+xPgr4bOA4z98l4LWBv+ZFh/i3eWngt4FjvGC3Al8NfA+wy7/NceC9gI8GHswLdgl4beCv+ddB/Nu9NPDTwIP4l/008NvAzwC38sI9GHgr4LWBt+Zf9gzgrYG/5l8P8e9zHPhu4K341/lrYJfndBx4af51vgf4aGCXfxvEf4z3Br4aOMZ/jWcAHw38NP8+iP84x4GPBj4aOMZ/jkvAVwNfDezy74f4j3cc+GjgvYEH8R/jGcB3A18N7PIfB/Gf67WBtwZeG3gp/nX+Bvht4KeB3+Y/B+K/zoOBBwMvDRznipfmir/mil3gr4G/Bnb5z4f4/w3x/xvi/zfE/2+I/9/4R0tfOVB8wvy3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentVerySatisfied;
impl IconShape for MdSentimentVerySatisfied {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm-5-6c.78 2.34 2.72 4 5 4s4.22-1.66 5-4H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEsklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O7008FbAg7niVuBngL/mXwfxv8trA98FPJjn71bgfYDf5kWD+N/jvYHv4kXzPsB38y9D/O/w3sB38a/zPsB388Ih/uc7DjwdOM6/zi7wEGCXFwzxP99HA1/Fv83HAF/NC4b4n++3gdfi3+Z3gNfmBUP8z2f+fcQLhvifz/z7iBcM8T/frcCD+Ld5BvBgXjDE/2zvBXw1cJx/m+8B3psXDPE/02sDnwW8Nv8+7wN8Ny8Y4n+W48BXAe/Nv9/vAK/NC4f4n+OzgI8GjvPvdwl4aeBWXjjEf7/XBr4LeDAv2O8A3w18NPBSvHDPAN4a+Gv+ZYj/Pg8Gvgt4bV6wS8BHA9/NFceBjwbeG3gQz+kZwHcDXw3s8qJB/Nc7DnwU8Nm8cJ8DfDWwy/P30sBxrtgF/pp/PcR/rfcCvho4zgv2O8B7A7fynw/xX+O1gc8CXpsX7BnAewO/zX8dxL/eg4GPAl4beGmu+Gvgt4GvAW7l2Y4DXwW8Ny/YJeCrgc/mvx7iX+ergI/mhftq4GOAzwI+GjjOC/Y9wEcDu/z3QLzo/gp4af5j/A7w2cBv898L8aL5auCj+Pe7BHw08N38z4D4lz0YeDr/fp8DfDWwy/8ciH/ZVwMfxb/d7wDvDdzK/zyIf9lfAS/Nv81fAy/D/1yIf5n59xH/cyH+ZebfR/zPhfiX/TXwUvzb/A3w0vzPhfiXfTXwUfzb/TbwPsCt/M+D+Jc9GHg6/36fDXwNsMv/HIgXzVcDH8W/3y7w0cD38D8D4kX318BL8R/jt4HPAX6b/16If52vBj6KF+5rgI8GPhv4aOAYL9h3Ax8D7PLfA/Gv92Dgo4HXBl6KK/4G+G3gq4FbebbjwFcD78ULtgt8NfA5/NdD/Nd4beCzgdfiBbsVeB/gt/mvg/iv9d7AVwPHeMF+G3gf4Fb+8yH+6x0HPhr4LF64rwY+B9jlPw/iv8+Dge8GXosXbBf4aOB7eLbXBj4KeG3gOFfsAr8NfA3w27zoEP/9Xhv4buBBvGB/DXwO8FbAe/PCfTfwMcAu/zLE/xyfDXw0cIx/v78GXoZ/GeJ/lgcDnw28F/9+nwN8Ni8c4n+m1wa+Gngp/n0eAtzKC4b4n+29ga8GjvFv8zHAV/OCIf7n+2ngrfi3+R3gtXnBEP/z/TbwWvzb/A7w2rxgiP/5fht4Lf5tfgd4bV4wxP98Xw18FP82XwN8NC8Y4n++lwb+in+blwH+mhcM8b/DbwOvxb/O7wCvzQuH+N/hwcBfA8d40VwCXhq4lRcO8b/HSwO/DRzjhbsEvDbw1/zLEP+7HAe+Gngvnr+vAT4b2OVFg/jf67WBl+aKvwZ+m389xP9viP/fEP+/If5/Q/z/xj8CC/+fQRme/nsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShare;
impl IconShape for MdShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 16.08c-.76 0-1.44.3-1.96.77L8.91 12.7c.05-.23.09-.46.09-.7s-.04-.47-.09-.7l7.05-4.11c.54.5 1.25.81 2.04.81 1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3c0 .24.04.47.09.7L8.04 9.81C7.5 9.31 6.79 9 6 9c-1.66 0-3 1.34-3 3s1.34 3 3 3c.79 0 1.5-.31 2.04-.81l7.12 4.16c-.05.21-.08.43-.08.65 0 1.61 1.31 2.92 2.92 2.92 1.61 0 2.92-1.31 2.92-2.92s-1.31-2.92-2.92-2.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSick;
impl IconShape for MdSick {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,9c-1.1,0-2-0.9-2-2c0-1.1,2-4,2-4s2,2.9,2,4C23,8.1,22.1,9,21,9z M17.5,7c0-0.73,0.41-1.71,0.92-2.66 C16.68,2.88,14.44,2,11.99,2C6.47,2,2,6.48,2,12c0,5.52,4.47,10,9.99,10C17.52,22,22,17.52,22,12c0-0.55-0.06-1.09-0.14-1.62 C21.58,10.45,21.3,10.5,21,10.5C19.07,10.5,17.5,8.93,17.5,7z M15.62,7.38l1.06,1.06L15.62,9.5l1.06,1.06l-1.06,1.06L13.5,9.5 L15.62,7.38z M7.32,8.44l1.06-1.06L10.5,9.5l-2.12,2.12l-1.06-1.06L8.38,9.5L7.32,8.44z M15.44,17c-0.69-1.19-1.97-2-3.44-2 s-2.75,0.81-3.44,2H6.88c0.3-0.76,0.76-1.43,1.34-1.99L5.24,13.3c-0.45,0.26-1.01,0.28-1.49,0c-0.72-0.41-0.96-1.33-0.55-2.05 c0.41-0.72,1.33-0.96,2.05-0.55c0.48,0.28,0.74,0.78,0.74,1.29l3.58,2.07c0.73-0.36,1.55-0.56,2.43-0.56c2.33,0,4.32,1.45,5.12,3.5 H15.44z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7HeCnhp4LX5z/HbwF8DP8N/DMR/jAcDPwW8NP81/hp4G+BW/n0Q/37Hgb8CHsx/rb8GXgfY5d8O8e/32cBn8d/jc4DP5t8O8e/3dODB/Pe4FXgI/3aIfz/zvP4G2OUFe2ngGM/pEvDXvGDHgZfieYl/O8S/n3lerwP8Ni/YbwOvxXP6HeC1ecFeG/gtnpf4t0P8+5nn9TrAb/OC/TbwWjyn3wFemxfstYHf4nmJfzvEv595Xq8D/DYv2G8Dr8Vz+h3gtXnBXhv4LZ6X+LdD/PuZ5/U6wG/zgv028Fo8p98BXpsX7LWB3+J5iX87xL+feV6vA/w2L9hvA6/Fc/od4LV5wV4b+C2el/i3Q/z7mef1OsBv84L9NvBaPKffAV6bF+y1gd/ieYl/O8S/n3lerwP8Ni/YbwOvxXP6HeC1ecFeG/gtnpf4t0P82z0YeBDw2zyvjwb+mhfsq4GX5jn9NfDRvGAvDXw1z+u1gWcAt/Kvh/jXOw58F/DW/M/y08D7ALu86BD/OseBpwPH+Z9pF3gIsMuLBvGv81PAW/M/208Db8OLBvGiezDwdP53eAhwK/8yxIvutYHf4n+H1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+648BL82yfDbwWz+tzgN/mP8drA5/F8/od4LN5tr8GdvmXIf7t3hv4Lp7XCWCX/xzHgYs8r/cBvpt/PcS/3XHgIs/pZ4C35j/XTwNvxXM6Aezyr4f49/lp4K14tvcBvpv/XO8NfBfP9jPAW/Nvg/j3eW/gu3i2E8Au/7mOAxd5tvcBvpt/G8S/z3Hgp7nir4GP5r/GVwMvzRVvDezyb4P4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ec6hYRB15t9AwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSingleBed;
impl IconShape for MdSingleBed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,12c0-1.1-0.9-2-2-2V7c0-1.1-0.9-2-2-2H8C6.9,5,6,5.9,6,7v3c-1.1,0-2,0.9-2,2v5h1.33L6,19h1l0.67-2h8.67L17,19h1l0.67-2 H20V12z M16,10h-3V7h3V10z M8,7h3v3H8V7z M6,12h12v3H6V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEu0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JcdB94LeG/gpYFd4K+B3wY+h3/ZceC1gLcGHgwcB16a/3q/A7w2zwnxwr028FPAcZ6/W4G3Af6a5/XawEcBb83/DL8DvDbPCfGCvTTwW8BxXrhd4HWAv+aKBwOfBbw3/7P8DvDaPCfEC/Z04MG8aH4beB3grYHvAo7zP8/vAK/Nc0I8f28N/BT/Oj8NvDX/c/0O8No8J8Tz99nAZ/G8PocrPov/fX4HeG2eE+L5+2ngrXhOvwO8Nlf8NvBa/O/yO8Br85wQz99nA5/Fc9oFXoYr/go4zv8uvwO8Ns8J8fy9N/Bd/Me5BHw38NPAb3PFg4GXBt4aeC/+8/0O8No8J8Tzdxy4FTjGv9/3AB8N7PKCvTbw3cCD+M/zO8Br85wQL9hHA1/Fv8/3AO/Nc3otrrgE/DXPdhz4beCleNH9Di+6vwY+mueEeOG+Gvgo/m1+B3htnu2zgI8GjvNstwLvA/w2VzwY+GvgGC/cM4DXBm7l3wfxL3tr4LOBl+Jf5yHArVzxXcB784K9D/DdXPHZwGfxwr0N8NP8+yFedA8GHswV7w28Fy/Y3wAvzRXvDXwXL9wu8BBgF3gw8HReuNcBfpt/P8S/zWcDn8UL9jnAZ3PFXwEvzb/sY4Cv5oq/Bl6KF+x1gN/m3w/xb/PdwHvxgr0P8N1cYV40XwN8NFf8NvBavGCvA/w2/36If5vPBj6LF+x9gO/mCvOi+Rrgo7nit4HX4gV7HeC3+fdD/Nt8NvBZvGCfA3w2V/w18FL8yz4G+Gqu+CvgpXnBXgf4bf79EP82nw18Fi/YXwMvwxXvDXwXL9wl4MHALvBg4Om8cCeAXf79EP82Lw38FS/cQ4BbueK7gffiBXsf4Lu54rOBz+IFuwQc5z8G4t9uFzjGC/bbwOvwbJ8NfDRwjGd7BvDewG9zxYOBvwKO84J9D/De/MdA/Nv9NPBWvHDfDbwPz+m1uWIX+Gue7TjwW8BL88K9D/Dd/MdA/Nu9NvBb/Mu+G/gYYJcX7LWB7wIezAv3DODB/MdB/Pv8NvBa/Mt2ge8Gfhr4Ha54MPBawGsD782L5n2A7+Y/DuLf57WB3+K/xt8AL81/LMS/30cDX8V/vpcB/pr/WIj/GD8NvBX/ed4H+G7+4yH+YxwHvhp4L/7jvQ/w3fznQPzH+m7gvfiPcQl4b+Cn+c+D+I/31sBXAw/i3+53gPcGbuU/F+I/z0cDHw08iBfd7wCfDfw2/zUQ//leGnhr4LW54rV4tmcAfw38NvDTwK3810L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BsvWcQVOeQaoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSports;
impl IconShape for MdSports {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.23,6C9.57,6,8.01,6.66,6.87,7.73C6.54,6.73,5.61,6,4.5,6C3.12,6,2,7.12,2,8.5C2,9.88,3.12,11,4.5,11 c0.21,0,0.41-0.03,0.61-0.08c-0.05,0.25-0.09,0.51-0.1,0.78c-0.18,3.68,2.95,6.68,6.68,6.27c2.55-0.28,4.68-2.26,5.19-4.77 c0.15-0.71,0.15-1.4,0.06-2.06c-0.09-0.6,0.38-1.13,0.99-1.13H22V6H11.23z M4.5,9C4.22,9,4,8.78,4,8.5C4,8.22,4.22,8,4.5,8 S5,8.22,5,8.5C5,8.78,4.78,9,4.5,9z M11,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S12.66,15,11,15z",
            }
            circle {
                cx: "11",
                cy: "12",
                r: "2",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsBaseball;
impl IconShape for MdSportsBaseball {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.81,6.28C2.67,7.9,2,9.87,2,12s0.67,4.1,1.81,5.72C6.23,16.95,8,14.68,8,12S6.23,7.05,3.81,6.28z",
            }
            path {
                d: "M20.19,6.28C17.77,7.05,16,9.32,16,12s1.77,4.95,4.19,5.72C21.33,16.1,22,14.13,22,12S21.33,7.9,20.19,6.28z",
            }
            path {
                d: "M14,12c0-3.28,1.97-6.09,4.79-7.33C17.01,3.02,14.63,2,12,2S6.99,3.02,5.21,4.67C8.03,5.91,10,8.72,10,12 s-1.97,6.09-4.79,7.33C6.99,20.98,9.37,22,12,22s5.01-1.02,6.79-2.67C15.97,18.09,14,15.28,14,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsBasketball;
impl IconShape for MdSportsBasketball {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.09,11h4.86c-0.16-1.61-0.71-3.11-1.54-4.4C18.68,7.43,17.42,9.05,17.09,11z",
            }
            path {
                d: "M6.91,11C6.58,9.05,5.32,7.43,3.59,6.6C2.76,7.89,2.21,9.39,2.05,11H6.91z",
            }
            path {
                d: "M15.07,11c0.32-2.59,1.88-4.79,4.06-6c-1.6-1.63-3.74-2.71-6.13-2.95V11H15.07z",
            }
            path {
                d: "M8.93,11H11V2.05C8.61,2.29,6.46,3.37,4.87,5C7.05,6.21,8.61,8.41,8.93,11z",
            }
            path {
                d: "M15.07,13H13v8.95c2.39-0.24,4.54-1.32,6.13-2.95C16.95,17.79,15.39,15.59,15.07,13z",
            }
            path {
                d: "M3.59,17.4c1.72-0.83,2.99-2.46,3.32-4.4H2.05C2.21,14.61,2.76,16.11,3.59,17.4z",
            }
            path {
                d: "M17.09,13c0.33,1.95,1.59,3.57,3.32,4.4c0.83-1.29,1.38-2.79,1.54-4.4H17.09z",
            }
            path {
                d: "M8.93,13c-0.32,2.59-1.88,4.79-4.06,6c1.6,1.63,3.74,2.71,6.13,2.95V13H8.93z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/e7008FZc8dLAXwO3Aj8D7PKiQfzv89rAdwEP5gX7buBzgFt54RD/u3wX8N68aHaB9wF+mhcM8b/HdwHvzb/e2wA/zfOH+N/ho4Gv4t9mF3gZ4FaeF+Lf57WAvwF2+c9zHHg6cJx/u+8B3pvnhfi3eWngt4DjXPE+wHfzn+Ojga/i3+8EsMtzQvzrvTTwW8BxntP7AN/Nf7yfBt6Kf7/3Ab6b54T413lp4LeA4zx/7wN8N/+xng48mH+/zwE+m+eEeNG9NPBbwHFeuPcBvpv/OOY/xs8Ab81zQrxoXhr4LeA4L5r3Ab6b/xjmP8bnAJ/Nc0L8y14a+C3gOP867wN8N/9+vw28Fv9+HwN8Nc8J8cK9NPBbwHH+bd4H+G7+fb4a+Cj+/V4G+GueE+IFe2ngt4Dj/Pu8D/Dd/Ns9GHg6/z5/A7w0zwvx/D0Y+CvgOP8x3gf4bv7tvht4L/7tXgf4bZ4X4vn7bOCz+I/1PsB3829zHPht4KX41/sa4KN5/hDP32cDn8V/vPcBvpt/m5cGvht4KV503wO8Ny8Y4vk7DtwKHOM/3vsA382/zXHgq4H34oW7BHw28NW8cIgX7KWB3waO8R/vfYDv5t/uwcBHAy8NvBZXPAP4a+CngZ8GdvmXIV64lwZ+GzjGf7z3Ab6b/16If9lLA78NHOM/3vsA381/H8SL5qWB3waO8R/vfYDv5r8H4kX30sBvA8f4j/c+wHfzXw/xr/PSwG8Dx/iP9z7Ad/NfC/Gv99LAbwPH+I/3PsB3818H8W/z0sBvA8f4j/c+wHfzXwPxb/fSwG8Dx/iP9z7Ad/OfD/Hv89LAbwPH+I/3PsB3858L8e/30sBvA8f4j/c+wHfznwfxH+Olgd8GjvEf722An+Y/B+I/zksDvw0c4z/WXwMvw38OxH+slwZ+GzjGf5y/AV6a/xyI/3gvDfw2cIz/GB8DfDX/ORD/OV4a+G3gGP8+3wO8N/95EP95Xhr4beAY/zbfA7w3/7kQ/7leGvht4Bj/Ot8DvDf/+RD/+V4a+G3gGC+a7wHem/8aiP8aLw38NnCMF+57gPfmvw7iv85LA78NHOP5+x7gvXn+jnPFLv+xEP+1Xhr4beAYz+l7gPfm+TsO/BZXvA6wy38cxH+9lwZ+GngQV3wO8Nk8f8eB3wJemiv+GngdYJf/GIj/Pq8N/DWwy/N3HPgt4KV5Tn8NvA6wy78f4n+uzwY+i+fvr4HXAXb590H8z/bdwHvx/P018DrALv92iP/5vht4L56/vwZeB9jl3wbxv8N3A+/F8/fXwOsAu/zrIf73+G7gvXj+/hp4HWCXfx3E/y7fDbwXz99fA68D7PKiQ/zv893Ae/H8/TXwOsAuLxrE/07fDbwXz99fA68D7PIvQ/zv9d3Ae/G8vgd4b140iP/dvht4L57te4D35kWH+N/vu4H3Ar4HeG/+dRD/N7w38N386yH+f0P8/4b4/w3x/xvi/zf+ER9VoUGSDM2+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsCricket;
impl IconShape for MdSportsCricket {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.05,12.81L6.56,4.32c-0.39-0.39-1.02-0.39-1.41,0L2.32,7.15c-0.39,0.39-0.39,1.02,0,1.41l8.49,8.49 c0.39,0.39,1.02,0.39,1.41,0l2.83-2.83C15.44,13.83,15.44,13.2,15.05,12.81z",
            }
            rect {
                height: "6",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -8.5264 17.7562)",
                width: "2",
                x: "16.17",
                y: "16.17",
            }
            circle {
                cx: "18.5",
                cy: "5.5",
                r: "3.5",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/37HgbcCHgw8GHgw/7luBW4FbgV+Btjl3w7xb/dg4LOA9+a/13cDnwPcyr8e4t/ms4DP5n+WzwY+h38dxL/edwHvzf9M3w28Dy86xL/OZwOfxf9snwN8Ni8axIvupYG/4n+HlwH+mn8Z4kX3W8Br87/DzwBvzb8M8aJ5MPB0/nd5CHArLxziRfPRwFfxovsY4K+54rf4t/kb4Lu54r2Bl+Jf532A7+aFQ7xovht4L150rwP8NleYf72/AV6a5/TXwEvxovse4L154RAvmt8GXosX3esAv80V5l/vY4Cv5jl9NPBVvOh+B3htXjjEi8b867wO8NtcYf71Pgb4ap7TRwNfxYtuFzjBC4f4lx0HLvKv8zrAb3OF+df7a+BleE5/Bbw0/zrihUP8y14b+C1esN/heX008Ndc8ds8r5cGjvHC/TXw3Vzx3sBL86/3OsBv84Ih/mXvDXwXL5j41/tt4LX4z/c2wE/zgiH+ZZ8NfBYvmPjX+23gtfjP9znAZ/OCIf5l3w28Fy/Yb/O8Pgb4a674LZ7XSwPH+de7BHw38NfAWwNvxQv3PcB784Ih/mW/DbwW/zqvA/w2V5j/OC8D/DXP9tnAZ/GC/Q7w2rxgiH/ZReA4/zqvA/w2V5j/GL8DvDbP6cHA03nBdoETvGCIf5n513sd4Le5wvzH+B3gtXlODwaezgsnXjDEC/fawG/xr/c6wG9zhfmP8zLAX/Nsnw18Fi/c6wC/zfOHeOHeGvgp/vU+Gvhrrvht/uPsAt8N3Aq8NvDW/MveBvhpnj/EC/fZwGfxv9vnAJ/N84d44b4beC/+d/se4L15/hAv3G8Dr8X/br8DvDbPH+KFuwgc53+3XeAEzx/ihTP/N4jnD/GCvTbwW/zf8DrAb/O8EC/YWwM/xf8NbwP8NM8L8YJ9NvBZ/PtcAv6af5+XBo7x7/M5wGfzvBAv2GcDn8W/3SXgtYG/5t/npYHfBo7xb/c5wGfzvBAv2GcDn8W/zSXgtYG/5j/GSwO/DRzj3+ZzgM/meSFesPcGvot/vUvAawN/zX+slwZ+GzjGv977AN/N80K8YMeBi/zrXAJeG/hr/nO8NPDbwDH+dU4AuzwvxAv31cBH8aK5BLw28Nf853pp4LeBY7xoPgf4bJ4/xL/st4HX4oW7BLw28Nf813hp4LeBY7xwvwO8Ni8Y4kXz2cBn8fz9DvDewK3813pp4KuB1+L5+xzgs3nhEC+6BwNvDTwYOA7cCvw08Nf893pp4K2BBwO7wK3ATwO38i9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjCVifQes5UvEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsEsports;
impl IconShape for MdSportsEsports {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.58,16.09l-1.09-7.66C20.21,6.46,18.52,5,16.53,5H7.47C5.48,5,3.79,6.46,3.51,8.43l-1.09,7.66 C2.2,17.63,3.39,19,4.94,19h0c0.68,0,1.32-0.27,1.8-0.75L9,16h6l2.25,2.25c0.48,0.48,1.13,0.75,1.8,0.75h0 C20.61,19,21.8,17.63,21.58,16.09z M11,11H9v2H8v-2H6v-1h2V8h1v2h2V11z M15,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C16,9.55,15.55,10,15,10z M17,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C18,12.55,17.55,13,17,13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsFootball;
impl IconShape for MdSportsFootball {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.02,15.62c-0.08,2.42,0.32,4.34,0.67,4.69s2.28,0.76,4.69,0.67L3.02,15.62z",
            }
            path {
                d: "M13.08,3.28C10.75,3.7,8.29,4.62,6.46,6.46s-2.76,4.29-3.18,6.62l7.63,7.63c2.34-0.41,4.79-1.34,6.62-3.18 s2.76-4.29,3.18-6.62L13.08,3.28z M9.9,15.5l-1.4-1.4l5.6-5.6l1.4,1.4L9.9,15.5z",
            }
            path {
                d: "M20.98,8.38c0.08-2.42-0.32-4.34-0.67-4.69s-2.28-0.76-4.69-0.67L20.98,8.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsGolf;
impl IconShape for MdSportsGolf {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,16c3.87,0,7-3.13,7-7c0-3.87-3.13-7-7-7S5,5.13,5,9C5,12.87,8.13,16,12,16z M12,4c2.76,0,5,2.24,5,5s-2.24,5-5,5 s-5-2.24-5-5S9.24,4,12,4z",
            }
            circle {
                cx: "10",
                cy: "8",
                r: "1",
            }
            circle {
                cx: "14",
                cy: "8",
                r: "1",
            }
            circle {
                cx: "12",
                cy: "6",
                r: "1",
            }
            path {
                d: "M7,19h2c1.1,0,2,0.9,2,2v1h2v-1c0-1.1,0.9-2,2-2h2v-2H7V19z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsHandball;
impl IconShape for MdSportsHandball {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.27,6C13.72,6.95,14.05,8.18,15,8.73c0.95,0.55,2.18,0.22,2.73-0.73c0.55-0.95,0.22-2.18-0.73-2.73 C16.05,4.72,14.82,5.05,14.27,6z",
            }
            path {
                d: "M15.84,10.41c0,0-1.63-0.94-2.6-1.5c-2.38-1.38-3.2-4.44-1.82-6.82l-1.73-1C8.1,3.83,8.6,7.21,10.66,9.4l-5.15,8.92 l1.73,1l1.5-2.6l1.73,1l-3,5.2l1.73,1l6.29-10.89c1.14,1.55,1.33,3.69,0.31,5.46l1.73,1C19.13,16.74,18.81,12.91,15.84,10.41z",
            }
            path {
                d: "M12.75,3.8c0.72,0.41,1.63,0.17,2.05-0.55c0.41-0.72,0.17-1.63-0.55-2.05c-0.72-0.41-1.63-0.17-2.05,0.55 C11.79,2.47,12.03,3.39,12.75,3.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsHockey;
impl IconShape for MdSportsHockey {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2,17v3l2,0v-4H3C2.45,16,2,16.45,2,17z",
            }
            path {
                d: "M9,16H5v4l4.69-0.01c0.38,0,0.72-0.21,0.89-0.55l0.87-1.9l-1.59-3.48L9,16z",
            }
            path {
                d: "M21.71,16.29C21.53,16.11,21.28,16,21,16h-1v4l2,0v-3C22,16.72,21.89,16.47,21.71,16.29z",
            }
            path {
                d: "M13.6,12.84L17.65,4H14.3l-1.76,3.97l-0.49,1.1L12,9.21L9.7,4H6.35l4.05,8.84l1.52,3.32L12,16.34l1.42,3.1 c0.17,0.34,0.51,0.55,0.89,0.55L19,20v-4h-4L13.6,12.84z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsKabaddi;
impl IconShape for MdSportsKabaddi {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "16.5",
                cy: "2.38",
                r: "2",
            }
            path {
                d: "M24,11.88v-4.7l-5.05-2.14c-0.97-0.41-2.09-0.06-2.65,0.84l0,0l-1,1.6c-0.67,1.18-1.91,2.06-3.41,2.32l0.06,0.06 c0.69,0.69,1.52,1.07,2.46,1.17c0.8-0.42,1.52-0.98,2.09-1.64l0.6,3l-1.16,1.1L15,14.38v0.76v6.74h2v-6l2.1-2l1.8,8H23l-2.18-11 l-0.62-3.1l1.8,0.7v3.4H24z",
            }
            path {
                d: "M10.29,8.09c0.22,0.15,0.47,0.24,0.72,0.29c0.13,0.02,0.25,0.04,0.38,0.04s0.26-0.01,0.38-0.04 c0.13-0.02,0.25-0.06,0.37-0.11c0.24-0.1,0.47-0.24,0.66-0.44c0.49-0.49,0.67-1.17,0.55-1.8C13.28,5.66,13.1,5.29,12.8,5 c-0.19-0.19-0.42-0.34-0.66-0.44c-0.12-0.05-0.24-0.09-0.37-0.11s-0.25-0.04-0.38-0.04c-0.12,0-0.23,0.01-0.35,0.03 c-0.14,0.02-0.28,0.06-0.41,0.11C10.4,4.66,10.17,4.81,9.98,5C9.68,5.29,9.5,5.66,9.43,6.03c-0.12,0.63,0.06,1.31,0.55,1.8 C10.07,7.93,10.18,8.01,10.29,8.09z",
            }
            path {
                d: "M11.24,10.56l-2-2c-0.1-0.1-0.2-0.18-0.31-0.26C8.71,8.16,8.46,8.06,8.21,8.02C8.08,7.99,7.96,7.98,7.83,7.98 c-0.51,0-1.02,0.2-1.41,0.59l-3.34,3.34c-0.41,0.41-0.62,0.98-0.58,1.54C2.5,13.63,2.54,13.82,2.61,14l1.07,2.95l-3.63,3.63 L1.46,22l4.24-4.24v-2.22L7,16.75v5.13h2v-6l-2.12-2.12l2.36-2.36l0.71,0.71l0,0c1.29,1.26,2.97,2.04,5.03,2.04l-0.14-2.07 C13.34,12.06,12.14,11.46,11.24,10.56z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zkOfBTw3sCD+c/z18BXA9/DfyzEv92Dgd8CHsx/ne8G3of/OIh/u78CXpr/ep8DfDb/MRD/Nm8N/BT/PXaBE/zHQPzbfDbwWfz3eR3gt/n3Q/zb/DbwWvz3eR3gt/n3Q/zb/DbwWvz3eR3gt/n3Q/zb/DbwWvz3eR3gt/n3Q/zb/DbwWvz3eR3gt/n3Q/zb/DbwWvz3eR3gt/n3Q/zb/DbwWjynS8Bf8x/vtXherwP8NvBaXPE7/Nsg/m1+G3gtntPvAK/NfzzzvN4G+Czgpbnir4HXAXb510H82/w28Fo8p98BXpv/eOZ5/QzwVjynjwG+mn8dxL/NbwOvxXP6HeC1+Y9nXjQ/A7w1/zqIf5vfBl6L5/Q7wGvzH8+8aH4HeG3+dRD/Nr8NvBbP6XeA1+Y/nnnR/A7w2vzrIP5tfht4LZ7T7wCvzX8886L5HeC1+ddB/Nv8NvBaPKffAV6b/3jmRfM7wGvzr4P4t/lt4LV4Tn8NfDT/8X6bF83vAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vvBt6L/1m+B3hv/nUQ/zYfDXwV/7N8DPDV/Osg/m1eG/gt/md5HeC3+ddB/NuZ/1nEvx7i3+6vgZfif4a/AV6afz3Ev913A+/F/wzfA7w3/3qIf7uPBr6K/xk+Bvhq/vUQ/3avDfwW/zO8DvDb/Osh/n3M/wzi3wbx7/PXwEvx3+tvgJfm3wbx7/PdwHvx3+t7gPfm3wbx7/PRwFfx3+tjgK/m3wbx73McuBU4xn+PvwFeG9jl3wbx73cc+Gjgtfmvswv8NvDdwC7/doj/3xD/vyH+47w08FX81/gY4K/590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Mc5DtwKHOM/1yXgwcAu/36I/1gvDXw38FL85/gb4L2Bv+Y/BuL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CG2jdkHEIBO7AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsMma;
impl IconShape for MdSportsMma {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7,20c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-3H7V20z",
            }
            path {
                d: "M18,7c-0.55,0-1,0.45-1,1V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v5.8c0,0.13,0.01,0.26,0.04,0.39l0.8,4 c0.09,0.47,0.5,0.8,0.98,0.8h10.36c0.45,0,0.89-0.36,0.98-0.8l0.8-4C18.99,11.06,19,10.93,19,10.8V8C19,7.45,18.55,7,18,7z M15,10 H7V7h8V10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif4YHAw/ief0O/7kQ//XeCnht4KWBBwMP5oXbBX4b+G3gZ4Bb+Y+D+M93HHgr4L2B1+bf76+Brwa+h38/xH+eBwPvBXw0cJz/eLcCnw18D/92iP94x4HPAj6a/xp/DbwP8Nf86yH+Y7028F3Ag/mv99HA1/Cvg/iPcxx4b+CjgQfx3+O7gffhRYf4z/HWwEcDr8V/ve8G3ocXDeI/14OBzwbeGjjGf53vBt6Hfxniv8Zx4K2BzwYexH+NjwG+mhcO8W/3XcBfA98D7PKie23go4G34j/fywB/zQuG+Lf5auCjuGIX+Gngc4BbedE9GPho4KP4z/PbwOvwgiH+9d4a+Cmev98Gvhv4Hl50Dwa+G3gt/nO8D/DdPH+If50HA38FHOeFuxX4buBrgF1eNN8NvBf/8W4FHsLzh/jX+W7gvfjX+W7ga4C/5l/228Br8R/vdYDf5nkhXnSvDfwW/3Z/DXw18D28YMeBW4Fj/Mf6HuC9eV6IF91vAa/Nv98u8NXA5/D8fTXwUfzH2gVO8LwQL5rXBn6L/1h/DbwOsMtzejDwdP7jvQzw1zwnxIvmt4DX5j/e5wCfzfPaBY7xH+ttgJ/mOSH+ZQ8Gns5/jluBh/C8fht4Lf5jfQ7w2TwnxL/su4H34j+PeF6/DbwW/7E+B/hsnhPiX3YROM5/jt8BXpvn9dvAa/Ef63uA9+Y5IV64twZ+iv88bwP8NM/L/Mf7GeCteU6IF+67gffiP8fXAB/N83pr4Kf4j/c5wGfznBAv3HcDD+Y/1m8Dvw38Ns/fdwPvxX+8zwE+m+eE+J/ltYHf4j/H2wA/zXNC/M9xHPgr4MH853gZ4K95Toj/GY4DvwW8NP85ngE8mOeF+O/3XsBXA8f5z/M9wHvzvBD/PV4LeGngo4EH85/vdYDf5nkhnr/XBn6L/xueATyY5w/x/L028Fv83/A+wHfz/CGev88GPov//Z4BPJgXDPH8fTbwWfzv9zrAb/OCIZ6/rwY+iv/dvgb4aF44xPP328Br8b/X3wAvzb8M8fz9NvBa/O/0N8BrA7v8yxDP328Dr8X/Pn8DvDawy4sG8fx9N/Be/O/yO8BbA7u86BDP30cDX8X/Hp8DfDb/eojn7zhwK3CM/9l+B/ho4K/5t0G8YO8NfBf/M/0M8NXAb/Pvg3jhXhv4buBB/Pe6BPw28NPATwO7/MdAvGheGjjOf4+/Bnb5z4H4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wisNI1BuqstuQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsMotorsports;
impl IconShape for MdSportsMotorsports {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,11.39c0-0.65-0.39-1.23-0.98-1.48L5.44,7.55c-1.48,1.68-2.32,3.7-2.8,5.45h7.75C11.28,13,12,12.28,12,11.39z",
            }
            path {
                d: "M21.96,11.22c-0.41-4.41-4.56-7.49-8.98-7.2c-2.51,0.16-4.44,0.94-5.93,2.04l4.74,2.01c1.33,0.57,2.2,1.87,2.2,3.32 c0,1.99-1.62,3.61-3.61,3.61H2.21C2,16.31,2,17.2,2,17.2V18c0,1.1,0.9,2,2,2h10C18.67,20,22.41,15.99,21.96,11.22z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsRugby;
impl IconShape for MdSportsRugby {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.49,3.51c-0.56-0.56-2.15-0.97-4.16-0.97c-3.08,0-7.15,0.96-9.98,3.79C1.66,11.03,2.1,19.07,3.51,20.49 c0.56,0.56,2.15,0.97,4.16,0.97c3.08,0,7.15-0.96,9.98-3.79C22.34,12.97,21.9,4.93,20.49,3.51z M7.76,7.76 c2.64-2.64,6.35-3.12,8.03-3.19c-2.05,0.94-4.46,2.45-6.61,4.61c-2.16,2.16-3.67,4.58-4.62,6.63C4.66,13.33,5.44,10.07,7.76,7.76z M16.24,16.24c-2.64,2.64-6.35,3.12-8.03,3.19c2.05-0.94,4.46-2.45,6.61-4.61c2.16-2.16,3.67-4.58,4.62-6.63 C19.34,10.67,18.56,13.93,16.24,16.24z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsSoccer;
impl IconShape for MdSportsSoccer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13,5.3l1.35-0.95 c1.82,0.56,3.37,1.76,4.38,3.34l-0.39,1.34l-1.35,0.46L13,6.7V5.3z M9.65,4.35L11,5.3v1.4L7.01,9.49L5.66,9.03L5.27,7.69 C6.28,6.12,7.83,4.92,9.65,4.35z M7.08,17.11l-1.14,0.1C4.73,15.81,4,13.99,4,12c0-0.12,0.01-0.23,0.02-0.35l1-0.73L6.4,11.4 l1.46,4.34L7.08,17.11z M14.5,19.59C13.71,19.85,12.87,20,12,20s-1.71-0.15-2.5-0.41l-0.69-1.49L9.45,17h5.11l0.64,1.11 L14.5,19.59z M14.27,15H9.73l-1.35-4.02L12,8.44l3.63,2.54L14.27,15z M18.06,17.21l-1.14-0.1l-0.79-1.37l1.46-4.34l1.39-0.47 l1,0.73C19.99,11.77,20,11.88,20,12C20,13.99,19.27,15.81,18.06,17.21z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsTennis;
impl IconShape for MdSportsTennis {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.52,2.49c-2.34-2.34-6.62-1.87-9.55,1.06c-1.6,1.6-2.52,3.87-2.54,5.46c-0.02,1.58,0.26,3.89-1.35,5.5l-4.24,4.24 l1.42,1.42l4.24-4.24c1.61-1.61,3.92-1.33,5.5-1.35s3.86-0.94,5.46-2.54C21.38,9.11,21.86,4.83,19.52,2.49z M10.32,11.68 c-1.53-1.53-1.05-4.61,1.06-6.72s5.18-2.59,6.72-1.06c1.53,1.53,1.05,4.61-1.06,6.72S11.86,13.21,10.32,11.68z",
            }
            path {
                d: "M18,17c0.53,0,1.04,0.21,1.41,0.59c0.78,0.78,0.78,2.05,0,2.83C19.04,20.79,18.53,21,18,21s-1.04-0.21-1.41-0.59 c-0.78-0.78-0.78-2.05,0-2.83C16.96,17.21,17.47,17,18,17 M18,15c-1.02,0-2.05,0.39-2.83,1.17c-1.56,1.56-1.56,4.09,0,5.66 C15.95,22.61,16.98,23,18,23s2.05-0.39,2.83-1.17c1.56-1.56,1.56-4.09,0-5.66C20.05,15.39,19.02,15,18,15L18,15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSportsVolleyball;
impl IconShape for MdSportsVolleyball {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6,4.01C3.58,5.84,2,8.73,2,12c0,1.46,0.32,2.85,0.89,4.11L6,14.31V4.01z",
            }
            path {
                d: "M11,11.42V2.05C9.94,2.16,8.93,2.43,8,2.84v10.32L11,11.42z",
            }
            path {
                d: "M12,13.15l-8.11,4.68c0.61,0.84,1.34,1.59,2.18,2.2L15,14.89L12,13.15z",
            }
            path {
                d: "M13,7.96v3.46l8.11,4.68c0.42-0.93,0.7-1.93,0.82-2.98L13,7.96z",
            }
            path {
                d: "M8.07,21.2C9.28,21.71,10.6,22,12,22c3.34,0,6.29-1.65,8.11-4.16L17,16.04L8.07,21.2z",
            }
            path {
                d: "M21.92,10.81c-0.55-4.63-4.26-8.3-8.92-8.76v3.6L21.92,10.81z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwitchAccount;
impl IconShape for MdSwitchAccount {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 2c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H8v-1.5c0-1.99 4-3 6-3s6 1.01 6 3V16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3++1+M/1DOBWXnQvDRzjeV0C/prnhPi3e2/gs4AH85/vVuBjgJ/mhfts4LN4/n4HeG2eE+Lf5rOBz+K/3vsA383z99LAX/GC/Q7w2jwnxL/eSwN/xX+f1wF+m+f12sBv8YL9DvDaPCfEv953A+/Ff59bgYfwvF4b+C1esN8BXpvnhPjXeTDwdP77PQS4lef02sBv8YL9DvDaPCfEv85nA5/Ff7/XAX6b5/TawG/xgv0O8No8J8S/zkXgOP/9Xgf4bZ7TawO/xQv2O8Br85wQL7r3Br6L/xleB/htntNrA7/FC/Y7wGvznBAvur8CXpr/GV4H+G2e02sDv8UL9jvAa/OcEC+a1wZ+i/85Xgf4bZ7TawO/xQv2O8Br85wQL5rvBt6L/zleB/htntNrA7/FC/Y7wGvznBD/sgcDT+d/ltcBfpvn9NrAb/GC/Q7w2jwnxL/sq4GP4n+W1wF+m+f02sBv8YL9DvDaPCfEC3cceDpwnP9ZXgf4bZ7TawO/xQv2O8Br85wQL9x7A9/F/zyvA/w2z+m1gd/iBfsd4LV5TogX7unAg/mf53WA3+Y5vTbwW7xgvwO8Ns8J8YK9NvBb/M/0OsBv85xeG/gtXrDfAV6b54R4wX4LeG3+Z3od4Ld5Tq8N/BYv2O8Ar81zQjx/Dwaezv9crwP8Ns/ptYHf4gX7HeC1eU6I5++jga/if67XAX6b5/TawG/xgv0O8No8J8Tz99PAW/E/1+sAv81zem3gt3jBfgd4bZ4T4vn7beC1+J/rdYDf5jm9NvBbvGC/A7w2zwnx/P028Fr8z/U6wG/znF4b+C1esN8BXpvnhHj+fht4Lf7neh3gt3lOrw38Fi/Y7wCvzXNCPH+fDXwW/3O9DvDbPKfXBn6LF+x3gNfmOSGev7cGfor/uV4H+G2e02sDv8UL9jvAa/OcEC/YXwMvxf9MrwP8Ns/ptYHf4gX7HeC1eU6IF+zBwE8DL8X/PK8D/DbP6ThwK3CM5+93gNfmOSFeuOPARwPvDTyI/zlOALs8r5cGvht4KZ7X7wCvzXNC/Nf4beC1+I/xNcBH8x8D8V/D/Mf4G+Cl+Y+D+M/30sBf8e/3N8BrA7v8x0H853tv4Lv49/kb4LWBXf5jIf7zfTXwUfzb/Q3w2sAu//EQ//l+G3gt/m3+BnhtYJf/HIj/fObf5m+A1wZ2+c+D+M/10sBf8a/3N8BrA7v850L853pv4Lv41/kb4LWBXf7zIf5zfTbwWbzo/gZ4bWCX/xqI/1y/DbwWL5q/AV4b2OW/DuI/10XgOP+yvwFeG9jlvxbiP8+DgafzL/sb4LWBXf7rIf7zPBh4Oi/c3wCvDezy3wPxn+u3gdfi+fsb4LWBXf77IP5zPRj4a+AYz+lvgNcGdvnvhfjP997Ad/FsfwO8NrDLfz/Ef433Br4L+BvgtYFd/mdA/Nd5b+CngV3+50D8/4b4/w3x/xvi/zfE/2/8I26Rr0GkzeAlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbDownAlt;
impl IconShape for MdThumbDownAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 4h-2c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h2V4zM2.17 11.12c-.11.25-.17.52-.17.8V13c0 1.1.9 2 2 2h5.5l-.92 4.65c-.05.22-.02.46.08.66.23.45.52.86.88 1.22L10 22l6.41-6.41c.38-.38.59-.89.59-1.42V6.34C17 5.05 15.95 4 14.66 4h-8.1c-.71 0-1.36.37-1.72.97l-2.67 6.15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8xwHXhv4af7zIf5nOQ78FvDSwPsA381/LsT/HMeB3wJemmd7H+C7+c+D+J/hOPBbwEvznHaBlwFu5T8H4r/fceC3gJfm+ftp4G34z4H473Uc+C3gpXnhHgLcyn88xH+f48BvAS/Nv+wEsMt/PMR/j+PAbwEvzb/sEnCc/xyI/3rHgd8CXpoXze8Ar81/DsR/rePAbwEvzYvuc4DP5j8H4r/OceC3gJfmX+dtgJ/mPwfiv8Zx4LeAl+Zf7yHArfznQPznOw78FvDS/OtdAo7znwfxn+s48FvAS/Nv8zvAa/OfB/Gf5zjwW8BL82/3NcBH858H8Z/jOPBbwEvz7/M+wHfznwfxH+848FvAS/Pv9zLAX/OfB/Ef6zjwW8BL8x9D/OdC/Mf6auCj+I/xO8Br858L8fz9NvBaPK+/Bt4H+Gue13HgIv9z3Ap8N/A1wC7PH+L5+23gtXj+doGHALs8p9cGfov/ef4aeBvgVp4X4vn7beC1eMFeB/htntNrA7/F/0x/DbwMzwvx/P028Fq8YK8D/DbP6bWB3+J/rrcBfprnhHj+fht4LV6w1wF+m+f02sBv8T/X5wCfzXNCPH+/DbwWL9jrAL/Nc3pt4Lf4n+t3gNfmOSGev98GXosX7HWA3+Y5vTbwW/zP9TvAa/OcEM/fbwOvxQv2OsBv85xeG/gt/uf6HeC1eU6I5++3gdfiBXsd4Ld5Tq8N/Bb/c30P8N48J8Tz99vAa/GCvQ7w2zyn1wZ+i/+5Pgb4ap4T4vn7beC1eMFeB/htntNrA7/F/1wPAW7lOSGev98GXosX7HWA3+Y5vTbwW/zP9DPAW/O8EM/fbwOvxQv2OsBv85xeG/gt/md6HeC3eV6I5++3gdfiBXsd4Ld5Tq8N/Bb/8zwDeDDPH+L5+23gtXjBXgf4bZ7TawO/xf887wN8N88f4vn7beC1eMFeB/htntNrA7/F/yyXgAcDuzx/iOfvt4HX4gV7HeC3eU6vDfwW/7N8DfDRvGCI5++3gdfiBXsd4Ld5Tq8N/Bb/szwEuJUXDPH8/TbwWrxgrwP8Ns/ptYHf4n+OnwHemhcO8fz9NvBavGCvA/w2z+m1gd/if47XAX6bFw7x/P028Fq8YK8D/DbP6bWB3+J/hr8BXpp/GeL5+23gtXjBXgf4bZ7TawO/xf8M7wN8N/8yxPP328Br8YK9DvDbPKfXBn6L/36XgOO8aBDP328Dr8UL9jrAb/OcXhv4Lf77fQ7w2bxoEM/fbwOvxQv2OsBv85xeGvgr/vs9BLiVFw3i+ftt4LV4wV4H+G2e163Ag/jv8z3Ae/OiQzx/vw28Fi/Y6wC/zfN6a+Cn+O/zOsBv86JDPH+/DbwWL9jLAH/N8/fewHfxX+9zgM/mXwfx/P028Fo8f58DfDYv3FsDXw08iP98zwA+G/hu/vUQz99LA8d5XrvAX/OiezDwYP5z/Tb/doj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj3leo0GayscWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdThumbUpAlt;
impl IconShape for MdThumbUpAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 20h2c.55 0 1-.45 1-1v-9c0-.55-.45-1-1-1H2v11zm19.83-7.12c.11-.25.17-.52.17-.8V11c0-1.1-.9-2-2-2h-5.5l.92-4.65c.05-.22.02-.46-.08-.66-.23-.45-.52-.86-.88-1.22L14 2 7.59 8.41C7.21 8.79 7 9.3 7 9.83v7.84C7 18.95 8.05 20 9.34 20h8.11c.7 0 1.36-.37 1.72-.97l2.66-6.15z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWhatshot;
impl IconShape for MdWhatshot {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5.67s.74 2.65.74 4.8c0 2.06-1.35 3.73-3.41 3.73-2.07 0-3.63-1.67-3.63-3.73l.03-.36C5.21 7.51 4 10.62 4 14c0 4.42 3.58 8 8 8s8-3.58 8-8C20 8.61 17.41 3.8 13.5.67zM11.71 19c-1.78 0-3.22-1.4-3.22-3.14 0-1.62 1.05-2.76 2.81-3.12 1.77-.36 3.6-1.21 4.62-2.58.39 1.29.59 2.65.59 4.04 0 2.65-2.15 4.8-4.8 4.8z",
            }
        }
    }
}
