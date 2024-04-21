use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessAlarm;
impl IconShape for MdAccessAlarm {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
pub struct MdAccessAlarms;
impl IconShape for MdAccessAlarms {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 5.7l-4.6-3.9-1.3 1.5 4.6 3.9L22 5.7zM7.9 3.4L6.6 1.9 2 5.7l1.3 1.5 4.6-3.8zM12.5 8H11v6l4.7 2.9.8-1.2-4-2.4V8zM12 4c-5 0-9 4-9 9s4 9 9 9 9-4 9-9-4-9-9-9zm0 16c-3.9 0-7-3.1-7-7s3.1-7 7-7 7 3.1 7 7-3.1 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccessTime;
impl IconShape for MdAccessTime {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
pub struct MdAdUnits;
impl IconShape for MdAdUnits {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14zM8 6h8v2H8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddAlarm;
impl IconShape for MdAddAlarm {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFF0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/3mvxX+9vgF3+9RD/cT4L+GjgOP89vhv4GGCXFx3iP8Z3Ae/Nf7+/Bl4H2OVFg/j3e2/gu/if46eBt+FFg/j3+yvgpfmf5SHArfzLEP9+5n+e1wF+m38Z4t/P/M/zOsBv8y9D/PuZ/3leB/ht/mWIfz/zP8/rAL/Nvwzx72f+53kd4Lf5lyH+/cz/PK8D/Db/MsS/n3leHwP8Nf81fovn9TrAb/MvQ/z7mef1OsBv81/DPK/XAX6bfxni3888r9cBfpv/GuZ5vQ7w2/zLEP/7mef1OsBv8y9D/O9nntfrAL/Nvwzxv595Xq8D/Db/MsT/fuZ5vQ7w2/zLEP/7mef1OsBv8y9D/Nt8NfBS/Md4Hf59zPN6HeC3+Zch/m1+G3gt/mOIfx/zvF4H+G3+ZYh/m98GXov/GOLfxzyv1wF+m38Z4t/mt4HX4j+G+Pcxz+t1gN/mX4b4t/lt4LX4jyH+fczzeh3gt/mXIf5tvhp4aV6448BL8S8T/z7meb0O8Nv8yxD/OY4DvwW8NP8y8e9jntfrAL/NvwzxH+848FvAS/OiEf8+5nm9DvDb/MsQ/7GOA78FvDQvOvHvY57X6wC/zb8M8R/nOPBbwEvz/P0N8FI8L/HvY57X6wC/zb8M8R/jOPBbwEvz/H0P8N3Ab/G8xL+PeV6vA/w2/zLEv99x4LeAl+b5+x7gvYHXBn6L5yX+fczzeh3gt/mXIf59jgO/Bbw0z9/3AO/NFa8N/BbPS/z7mOf1OsBv8y9D/NsdB34LeGmev+8B3ptne23gt3he4t/HPK/XAX6bfxni3+Y48FvAS/P8fQ/w3jyn1wZ+i+cl/n3M83od4Lf5lyH+9Y4DvwW8NM/f9wDvzfN6beC3eF7i38c8r9cBfpt/GeJf5zjwW8BL8/x9D/DePH+vDfwWz0v8+5jn9TrAb/MvQ7zojgO/Bbw0z9/3AO/NC/bawG/xvMS/j3lerwP8Nv8yxIvmOPBbwEvz/H0P8N68cK8N/BbPS/z7mOf1OsBv8y9D/MuOA78FvDTP3/cA781/H/O8Xgf4bf5liH/ZbwOvxfP3PcB789/LPK/XAX6bfxniX/bSwG8Dx3hO3wO8N//9zPN6HeC3+ZchXjQvDfw2cIwrvgd4b/5nMM/rdYDf5l+GeNG9NPDbwE8D783/HOZ5vQ7w2/zLEP86DwZu5X8W87xeB/ht/mWI//3M83od4Lf5lyH+9zPP63WA3+Zfhvj3M8/rdYDf5r+GeV6vA/w2/zLEv595Xq8D/Db/Nczzeh3gt/mXIf79zPP6aOCv+a/x2zyv1wF+m38Z4t/P/M/zOsBv8y9D/PuZ/3leB/ht/mWIfz/zP8/rAL/Nvwzx72f+53kd4Lf5lyH+/cz/PK8D/Db/MsS/n/mf53WA3+Zfhvj3+2vgpfif5SHArfzLEP9+7w18F/9z/Azw1rxoEP8xvht4L/77/Q3w2sAuLxrEf5zPBj4aOMZ/j58B3hvY5UWH+I/32vzXuxW4lX89xP9viP/f+Ec1D7lBkmha6QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddToHomeScreen;
impl IconShape for MdAddToHomeScreen {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M18 1.01L8 1c-1.1 0-2 .9-2 2v3h2V5h10v14H8v-1H6v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM10 15h2V8H5v2h3.59L3 15.59 4.41 17 10 11.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fd4LeGmu+Gvge/ivh/jv8VvAa/Ocfht4Hf5rIf7rfTbwWTx/nwN8Nv91EP/1/gp4aZ6/vwZehv86iP965oUT/3UQ//XMCyf+6yD+65kXTvzXQfzXMy+c+K+D+K9nXjjxXwfxX8+8cOK/DuK/nnnhxH8dxH8988KJ/zqI/3rmhRP/dRD/9cwLJ/7rIP7rmRdO/NdB/NczL5z4r4P4r2deOPFfB/Ffz7xw4r8O4r+eeeHEfx3Ef50HA98FvDYv3G8D7wPcyn8+xH++48BHAZ/Nv85nA18D7PKfB/Gf672AzwYezL/NrcBnA9/Dfw7Ef46XBr4KeG3+Y/w28DHAX/MfC/Ef6zjwWcBH85/jq4HPAXb5j4H4j/NRwGcDx/nPtQt8NvA1/Psh/v1eG/gq4KX5r/XXwMcAv82/HeLf7sHAZwHvzX+v7wY+B7iVfz3Ev81nAR8NHOd/hl3gq4HP4V8H8a/z2sB3AQ/mf6ZbgfcBfpsXDeJF82Dgu4DX5n+H3wbeB7iVFw7xwh0HPgr4bP53+mzga4Bdnj/EC/ZewGcDD+bf72eA3wZ+Gng6L9xDgLcG3hp4Lf79bgU+G/genhfi+ftu4L34t3sG8NvATwO/DezybOaFE892HHht4K2BtwaO8W/3PcB785wQz99vA6/Fv87fAD8N/DTw17xg5oUTL9hLA28NvDXwUvzr/A7w2jwnxPP328Br8cJdAn4b+Gngp4FdXjTmhRMvmgcDrw28NfDawDFeuN8BXpvnhHj+fht4LZ7XM4CfBn4a+G3+bcwLJ/5tXht4a+CtgQfxvH4HeG2eE+L5+23gtbjiZ4DfBn4auJV/P/PCiX+/BwNvDbw28FZc8TvAa/OcEM/fRwO3Ar8N7PIfy7xw4j/WceC1gQcDX81zQvzXMy+c+K+D+K9nXjjxXwfxX8+8cOK/DuK/nnnhxH8dxH8988KJ/zqI/3rmhRP/dRD/9cwLJ/7rIP5rHQcu8sKdAHb5r4H4r/NRwGcDx3nhdoHPBr6G/3yI/3yvDXwV8NL86/w18DHAb/OfB/Gf5zjwVcB78+/z3cDHALv8x0P853hv4KuA4/zH2AU+Bvhu/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8fw9A/ht4LeB3+aK1wZeG3ht4EE8f78DvDb/sRD/OX4beC2ueAbw28BvA78N3MoL92DgtYHXBl4beBBX/A7w2vzHQvzn+GhgF/ht4Fb+fR4MvDZwHPhq/mMh/n9D/P+G+P8N8f8b4v83/hGmI5RB47bm9wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirplanemodeActive;
impl IconShape for MdAirplanemodeActive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirplanemodeInactive;
impl IconShape for MdAirplanemodeInactive {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE5ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4a+CpeNB8D/DX/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/OV4aOMZzemngq3nRfDTw1zynS8Bf8x8L8R/rpYHvAl6a/xx/DbwP8Nf8x0D8x/or4KX5z/XXwMvwHwPxH+elgb/iv8bLAH/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fi+a1wF+m+f02sBv8aJ5HeC3+fdD/Md5beC3eNG8DvDbPKfXBn6LF83rAL/Nvx/iP85rA7/Fi+Z1gN/mOb028Fu8aF4H+G3+/RD/cV4b+C1eNK8D/DbP6bWB3+JF8zrAb/Pvh/iP89rAb/GieR3gt3lOrw38Fi+a1wF+m38/xH+c1wZ+ixfN6wC/zXN6beC3eNG8DvDb/Psh/uO8NvBbvGheB/htntNrA7/Fi+Z1gN/m3w/xH+e1gd/iRfM6wG/znF4b+C1eNK8D/Db/foj/OK8N/BYvmtcBfpvn9NrAb/GieR3gt/n3Q/zHeW3gt3jRvA7w2zyn1wZ+ixfN6wC/zb8f4j/OawO/xYvmdYDf5jm9NvBbvGheB/ht/v0Q/3FeG/gtXjSvA/w2z+m1gd/iRfM6wG/z74f4j/PawG/xonkd4Ld5Tq8N/BYvmtcBfpt/P8R/nNcGfosXzV8Duzyn48BL86J5HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+ixfN3wC7PKfjwEvxonkd4Lf590P8x3lt4Ld40bwO8Ns8p9cGfosXzesAv82/H+I/zmsDv8WL5nWA3+Y5vTbwW7xoXgf4bf79EP9xXhv4LV40rwP8Ns/ptYHf4kXzOsBv8++H+I/z2sBv8aJ5HeC3eU6vDfwWL5rXAX6bfz/Ef5zXBn6LF83rAL/Nc3pt4Ld40bwO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uMcB24FjvGf6xJwnP8YiP9Y7w18F/+53gb4af5jIP7jvTTw1sBr8x/rt4GfBv6a/ziI/98Q/78h/n9D/P+G+P+NfwTVo5hBXckV+QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatteryAlert;
impl IconShape for MdBatteryAlert {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zM13 18h-2v-2h2v2zm0-4h-2V9h2v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4a+CpeNB8D/DX/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/OV4aOMZzemngq3nRfDTw1zynS8Bf8x8L8R/rpYHvAl6a/xx/DbwP8Nf8x0D8x/or4KX5z/XXwMvwHwPxH+elgb/iv8bLAH/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/NtdAo7xonkd4Lf590P8x3lt4Lf4t/sc4LN40bwO8Nv8+yH+47w28Fv82/wM8NXAb/GieR3gt/n3Q/zHeW3gt/i3eQjwYOC3eNG8DvDb/Psh/uO8NvBb/Ot9D/DewGsDv8WL5nWA3+bfD/Ef57WB3+Jf7yHArcBrA7/Fi+Z1gN/m3w/xH+e1gd/iX+dzgM/mitcGfosXzesAv82/H+I/zmsDv8WL7hLwYGCXK14b+C1eNK8D/Db/foj/OK8N/BYvus8BPptne23gt3jRvA7w2/z7If7jvDbwW7xoLgEPBnZ5ttcGfosXzesAv82/H+I/zmsDv8WL5n2A7+Y5vTbwW7xoXgf4bf79EP9xXhv4Lf5lzwAezPN6beC3eNG8DvDb/Psh/uO8NvBb/MveB/huntdrA7/Fi+Z1gN/m3w/xH+e1gd/ihfsb4KV5/l4b+C1eNK8D/Db/foj/OK8N/BYv3OsAv83z99rAb/GieR3gt/n3Q/zHeW3gt3jBfgd4bV6w48BL85weDHwXz+t1gN/m3w/xH+e1gd/iBXsd4Lf51/lu4L14Xq8D/Db/foj/OK8N/BbP388Ab82/zoOBp/P8vQ7w2/z7If7jvDbwWzx/DwFu5V/np4C35vl7HeC3+fdD/Md5beC3eF7fA7w3/zqvDfwWL9jrAL/Nvx/iP85rA7/F83pv4Fae098Au7xgvwW8Ni/Y6wC/zb8f4j/OawO/xYvmdYDf5vl7beC3eOFeB/ht/v0Q/3FeG/gtXjSvA/w2z99fAS/NC/c6wG/z74f4j/PawG/xonkd4Ld5Xu8NfBf/stcBfpt/P8R/nNcGfosXzesAv83zejrwYP5lrwP8Nv9+iP84rw38Fi+a1wF+m+f03sB38aJ5HeC3+fdD/Md5beC3eNG8DvDbPNtx4OnAcV40rwP8Nv9+iP84rw38Fi+a1wF+m2f7bOCzeNG9DvDb/Psh/uO8NvBbvGheB/htrjgOPB04zovudYDf5t8P8R/ntYHf4kXzOsBvc8VnA5/Fv87rAL/Nvx/iP85rA7/Fi+Z1gN8GHgw8nX+91wF+m38/xH+c1wZ+ixfN6wC/DXw38F78670O8Nv8+yH+47w28Fu8aF4HuBV4Ov82rwP8Nv9+iP84rw38Fi+a1wE+Cnhr/m1eB/ht/v0Q/3FeG/gtXjSfA3wW/3avA/w2/36I/zivDfwWL5pd4Dj/dq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+4xwHbgWO8Z/rEnCc/xiI/1jvDXwX/7neBvhp/mMg/uO9NPDWwGvzH+u3gZ8G/pr/OIj/3xD/vyH+f0P8/4b4/41/BOr9tkEmRvNwAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatteryChargingFull;
impl IconShape for MdBatteryChargingFull {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zM11 20v-5.5H9L13 7v5.5h2L11 20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4a+CpeNB8D/DX/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/OV4aOMZzemngq3nRfDTw1zynS8Bf8x8L8R/rpYHvAl6a/xx/DbwP8Nf8x0D8x/or4KX5z/XXwMvwHwPxH+elgb/iv8bLAH/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OMeBW4Fj/Oe6BBznPwbiP9Z7A9/Ff663AX6a/xiI/3gvDbw18Nr8x/pt4KeBv+Y/DuL/N8T/b4j/3xD/vyH+f+MfAaMfcEEtGmTBAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatteryFull;
impl IconShape for MdBatteryFull {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4a+CpeNB8D/DX/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/OV4aOMZzemngq3nRfDTw1zynS8Bf8x8L8R/rpYHvAl6a/xx/DbwP8Nf8x0D8x/or4KX5z/XXwMvwHwPxH+elgb/iv8bLAH/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j/PawG/xX+N1gN/m3w/xH+e1gd/iv8brAL/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4r/G6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OMeBW4Fj/Oe6BBznPwbiP9Z7A9/Ff663AX6a/xiI/3gvDbw18Nr8x/pt4KeBv+Y/DuL/N8T/b4j/3xD/vyH+f+MfAaMfcEEtGmTBAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatteryStd;
impl IconShape for MdBatteryStd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dV4a+CpeNB8D/DX/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/dV4b+C1eNK8D/Db/+RD/OV4aOMZzemngq3nRfDTw1zynS8Bf8x8L8R/rpYHvAl6a/xx/DbwP8Nf8x0D8x/or4KX5z/XXwMvwHwPxH+elgb/iv8bLAH/Nvx/iP85rA7/Ff43XAX6bfz/Ef5zXBn6L/xqvA/w2/36I/zivDfwW/zVeB/ht/v0Q/3FeG/gt/mu8DvDb/Psh/uO8NvBb/Nd4HeC3+fdD/Md5beC3+K/xOsBv8++H+I/z2sBv8V/jdYDf5t8P8R/ntYHf4kXzN8BPA7dyxYOBtwZeihfN6wC/zb8f4j/OawO/xQv3O8B7A7fy/D0Y+G7gtXjhXgf4bf79EP9xXhv4LV6w7wHemxfNdwPvxQv2OsBv8++H+I/z2sBv8fx9D/De/Ot8N/BePH+vA/w2/36I/zivDfwWz+sS8GBgl+f0XsBLc8VfA9/DczoO3Aoc43m9DvDb/Psh/uO8NvBbPK+vAT6a5/RbwGvznH4beB2e01cDH8Xzeh3gt/n3Q/zHeW3gt3herwP8Ns/22cBn8fx9DvDZPNtrA7/F83od4Lf590P8x3lt4Ld4XuI5/RXw0jx/fw28DM/JPK/XAX6bfz/Ef5zXBn6L5/XbPKfX5oUTz8k8r9cBfpt/P8R/nNcGfot/n78BXppnezDwdJ7X6wC/zb8f4j/OawO/xb/P2wA/zbO9N/BdPK/XAX6bfz/Ef5zXBn6Lf7v3Ab6b5/R04ME8r9cBfpt/P8R/nNcGfot/m/cBvpvn9F3Ae/P8vQ7w2/z7If7jvDbwW/zrfQ3w0Tynjwa+ihfsdYDf5t8P8R/ntYHf4l/ne4D35jm9N/BdvHCvA/w2/36I/zivDfwWL7pnAC8N7PJsbw38FP+y1wF+m38/xH+c1wZ+ixfdxwBfzbMdB54OHOdf9jrAb/Pvh/iP89rAb/GiOwHs8myfDXwWL5rXAX6bfz/Ef5zXBn6LF514Tr8NvBYvmtcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C1eNM8AbuU5PRh4EC+a1wF+m38/xH+c1wZ+ixfN5wCfzXP6bOCzeNG8DvDb/Psh/uO8NvBbvGg+B/hsntNnA5/Fi+Z1gN/m3w/xH+e1gd/iRfM5wGfznD4b+CxeNK8D/Db/foj/OK8N/BYvms8BPpvn9NnAZ/GieR3gt/n3Q/zHeW3gt3jRfDfw3Tyn9wbemxfN6wC/zb8f4j/OawO/xX+N1wF+m38/xH+c1wZ+i/8arwP8Nv9+iP84rw38Fv81Xgf4bf79EP9xXhv4Lf5rvA7w2/z7If7jvDbwW/zXeB3gt/n3Q/zHeW3gt/iv8TrAb/Pvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/znHgVuAY/7kuAcf5j4H4j/XewHfxn+ttgJ/mPwbiP95LA28NvDb/sX4b+Gngr/mPg/j/DfH/G+L/N8T/b4j/3/hH5i6sQchu/HwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBatteryUnknown;
impl IconShape for MdBatteryUnknown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.67 4H14V2h-4v2H8.33C7.6 4 7 4.6 7 5.33v15.33C7 21.4 7.6 22 8.33 22h7.33c.74 0 1.34-.6 1.34-1.33V5.33C17 4.6 16.4 4 15.67 4zm-2.72 13.95h-1.9v-1.9h1.9v1.9zm1.35-5.26s-.38.42-.67.71c-.48.48-.83 1.15-.83 1.6h-1.6c0-.83.46-1.52.93-2l.93-.94c.27-.27.44-.65.44-1.06 0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5H9c0-1.66 1.34-3 3-3s3 1.34 3 3c0 .66-.27 1.26-.7 1.69z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBluetooth;
impl IconShape for MdBluetooth {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.71 7.71L12 2h-1v7.59L6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 5.83l1.88 1.88L13 9.59V5.83zm1.88 10.46L13 18.17v-3.76l1.88 1.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Pb4aeCmu+Bjgr/nvgfjv8dvAa3HFLvA6wF/zXw/x3+O3gdfi2XaB1wH+mv9aiP8evw28Fs9pF3gd4K/5r4P47/HbwGvxvHaB1wH+mv8aiP8evw28Fs/fLvA6wF/znw/x3+O3gdfiBdsFXgf4a/5zIf57/DbwWrxwu8DrAH/Nfx7Ev99LAy8NfDcvut8GXot/2S7wOsBf858D8e/z0sBvAceB9wG+mxfNbwOvxYtmF3gd4K/5j4f4t3tp4LeA4zzb+wDfzb/st4HX4jldAo7x/O0CrwP8Nf+xEP82Lw38FnCc5/U+wHfzwv028Fo8p98Bvhv4Lp6/XeB1gL/mPw7i3+a9ge/iBXsf4Lt5wX4beC2e0+8Arw28N/BdPH+7wOsAf81/DMS/3XsD38UL9j7Ad/P8/TbwWjyn3wFemyveG/gunr9d4HWAv+bfD/Hv897Ad/GCvQ/w3Tyv3wZei+f0O8Br82zvDXwXz98u8DrAX/Pvg/j3e2/gu3jB3gf4bp7TbwOvxXP6HeC1eU7fDbwXz98u8BBgl387xH+M9wa+ixfsfYDv5tl+G3gtntPvAK/Ns7038F28YO8DfDf/Poj/OO8NfBcv2PsA380Vvw28Fs/pd4DX5or3Br6LF+x9gO/m3w/xH+u9ge/iBXsf4LuB3wZei+f0O8BrA+8NfBcv2PsA381/DMR/vPcGvosX7H2A9wZei+f0O8B3A9/FC/Y+wHfzHwfxn+O9ge/iBdsFjvOcdoHjvGDvA3w3/7EQ/zYPBm7lhXtv4Lv4j/E+wHfzwj0YuJV/HcS/3ncBbw28DvDXvHDvDXwX/z7vA3w3L9xLA78F/DTwPrzoEP863wW8N1fsAq8D/DUv3HsD38W/zfsA380L99LAbwHHueK7gffhRYN40X0X8N48p13gdYC/5oV7b+C7+Nd5H+C7eeFeGvgt4DjP6buB9+FfhnjRfBfw3jx/u8DrAH/NC/fewHfxonkf4Lt54V4a+C3gOM/fdwPvwwuH+Jd9F/DevHC7wOsAf80L997Ad/HCvQ/w3bxwLw38FnCcF+67gffhBUP8y94b+C5euEvAawN/zQv33sB38cK9D/DdvHAvDfw2cIwX7n2A7+YFQ7xo3hv4Lp6/S8BrA3/NC/fewHfxonkf4Lt54V4a+G3gGM/f+wDfzQuHeNG9N/BdPKdLwGsDf80L997Ad/Gv8z7Ad/PCvTTw28AxntP7AN/Nvwzxr/PewHdxxSXgtYG/5oV7b+C7+Ld5H+C7eeFeGvht4BhXvA/w3bxoEP967w18NfDawF/zwr038F38+7wP8N28cC8N/Dbw0cB386JD/NscB3Z54d4b+C7+Y7wP8N28cMeBXf51EP853hv4Ll6wS8AxntMl4Bgv2PsA381/LMR/vPcGvosX7H2A9wZei+f0O8B3A9/FC/Y+wHfzHwfxH+u9ge/iBXsf4LuB3wZei+f0O8BrA+8NfBcv2PsA381/DMR/nPcGvosX7H2A7+aK3wZei+f0O8Brc8V7A9/FC/Y+wHfz74f4j/HewHfxgr0P8N08228Dr8Vz+h3gtXm29wa+ixfsfYDv5t8H8e/33sB38YK9D/DdPKffBl6L5/Q7wGvznL4beC+ev0vAg4Fd/u0Q/z7vDXwXL9j7AN/N8/pt4LV4Tr8DvDbP9t7Ad/H8XQJeG/hr/n0Q/3bvDXwXL9j7AN/N8/fbwGvxnH4HeG2ueG/gu3j+LgGvDfw1/36If5v3Br6LF+x9gO/mBftt4LV4Tr8DvDbw3sB38fxdAl4b+Gv+YyD+bV4a+G3gGM/rfYDv5oX7beC1eE6/A3w38F08f5eA1wb+mv84iH+7lwZ+GzjGs70P8N38y34beC2e0y5wnOfvEvDawF/zHwvx7/PSwG8Dx4D3Ab6bF81vA6/Fi+YS8NrAX/MfD/Hv99LASwPfzYvut4HX4l92CXht4K/5z4H47/HbwGvxwl0CXhv4a/7zIP57/DbwWrxgl4DXBv6a/1yI/x6/DbwWz98l4LWBv+Y/H+K/x28Dr8XzugS8NvDX/NdA/Pf4beC1eE6XgNcG/pr/Ooj/Hr8NvBbPdgl4beCv+a+F+O/x28BrccUl4LWBv+a/HuK/x1cDL80VHw38Nf89EP+/If5/Q/z/hvj/DfH/G/8I8+MUULANSaEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBluetoothConnected;
impl IconShape for MdBluetoothConnected {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 12l-2-2-2 2 2 2 2-2zm10.71-4.29L12 2h-1v7.59L6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 5.83l1.88 1.88L13 9.59V5.83zm1.88 10.46L13 18.17v-3.76l1.88 1.88zM19 10l-2 2 2 2 2-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBluetoothDisabled;
impl IconShape for MdBluetoothDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 5.83l1.88 1.88-1.6 1.6 1.41 1.41 3.02-3.02L12 2h-1v5.03l2 2v-3.2zM5.41 4L4 5.41 10.59 12 5 17.59 6.41 19 11 14.41V22h1l4.29-4.29 2.3 2.29L20 18.59 5.41 4zM13 18.17v-3.76l1.88 1.88L13 18.17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBluetoothSearching;
impl IconShape for MdBluetoothSearching {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.24 12.01l2.32 2.32c.28-.72.44-1.51.44-2.33 0-.82-.16-1.59-.43-2.31l-2.33 2.32zm5.29-5.3l-1.26 1.26c.63 1.21.98 2.57.98 4.02s-.36 2.82-.98 4.02l1.2 1.2c.97-1.54 1.54-3.36 1.54-5.31-.01-1.89-.55-3.67-1.48-5.19zm-3.82 1L10 2H9v7.59L4.41 5 3 6.41 8.59 12 3 17.59 4.41 19 9 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM11 5.83l1.88 1.88L11 9.59V5.83zm1.88 10.46L11 18.17v-3.76l1.88 1.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightnessAuto;
impl IconShape for MdBrightnessAuto {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.85 12.65h2.3L12 9l-1.15 3.65zM20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM14.3 16l-.7-2h-3.2l-.7 2H7.8L11 7h2l3.2 9h-1.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightnessHigh;
impl IconShape for MdBrightnessHigh {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69L23.31 12 20 8.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6zm0-10c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightnessLow;
impl IconShape for MdBrightnessLow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13dxxfvw3wPx3+e7gPfmiu8G3of/eoj/Ht8FvDfP6buB9+G/FuK/3ncB783z993A+/BfB/Ff67uA9+aF+27gffivgfiv813Ae/Oi+W7gffjPh/iv8V3Ae/Ov893A+/CfC/Gf77uA9+bf5ruB9+E/D+JFdxx4Kf513ht4b/59vhv4bv51/gbY5V+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/iP56AlwaOAw8G3hp4K/59Xgf4bf5liBfdawO/xX888byOA28NfDVwjH+91wF+m38Z4kX32sBv8R9PvGDHgY8GPot/ndcBfpt/GeJF99rAb/EfT/zLXhr4beAYL5rXAX6bfxniRffawG/xH0+8aB4M/DTwUvzLXgf4bf5liBfdawO/xX888aJ7MPDXwDFeuNcBfpt/GeJF99rAb/EfT/zrvDTwV7xwrwP8Nv8yxIvuu4D35j+e+Nf7bOCzeMG+G3gf/mWIF813Ae/Nfw7xr3ccuBU4xgv23cD78MIh/mXfBbw3/3nEv817A9/FC/fdwPvwgiFeuO8C3pv/XOLf5jhwkX/ZdwPvw/OHeMG+C3hv/vOJf7ufBt6Kf9l3A+/D80I8f98FvDf/NcS/3UcDX8WL5ruB9+E5IZ7XdwHvzX8d8W/32sBv8aL7buB9eDbEc/ou4L35ryX+7V4a+Cv+db4beB+uQDzbdwHvzX898e9j/vW+G3gfAPFs3w28F//1xL+P+df7HuC9AcRz+m7gvfivJf7tXhr4K/51vgd4b65APK/vBt6L/zri3+61gd/iRfc9wHvzbIjn77uB9+K/hvi3+2jgq3jRfA/w3jwnxAv23cB78Z9P/Nv9NPBW/Mu+B3hvnhfihftu4L34zyX+bY4DF/mXfQ/w3jx/iH/ZdwPvxX8e8W/z3sB38cJ9D/DevGCIF813A+/Ffw7xr3cceDpwnBfse4D35oVDvOi+G3gv/uOJf73PBj6LF+x7gPfmX4Z40b028Fv8xxP/Oi8N/BUv3OsAv82/DPGie23gt/iPJ150Dwb+CjjOC/c6wG/zL0O86F4b+C3+44kXzYOBnwJemn/Z6wC/zb8M8aJ7beC3+I8n/mUvDfwWcJwXzesAv82/DPGie23gt/iPJ16w48BHAZ/Nv87rAL/Nvwzxontt4Lf4jyee13HgrYCvBo7zr/c6wG/zL0O86F4b+C3+4wl4aeAY8NLAawNvzb/P6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91x4KX513lv4L349/ke4Lv51/lrYJd/GeI/33cD78W/zfcA781/HsR/je8G3ot/ne8B3pv/XIj/Ot8NvBcvmu8B3pv/fIj/Wt8NvBcv3PcA781/DcR/ve8G3ovn73uA9+a/DuK/x3cD78Vz+h7gvfmvhfjv893Ae3HF9wDvzX89xH+v7+aK9+a/B+L/N8T/b/wjI+WaQRuKGYsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrightnessMedium;
impl IconShape for MdBrightnessMedium {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.31L23.31 12 20 8.69V4h-4.69L12 .69 8.69 4H4v4.69L.69 12 4 15.31V20h4.69L12 23.31 15.31 20H20v-4.69zM12 18V6c3.31 0 6 2.69 6 6s-2.69 6-6 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDataUsage;
impl IconShape for MdDataUsage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 2.05v3.03c3.39.49 6 3.39 6 6.92 0 .9-.18 1.75-.48 2.54l2.6 1.53c.56-1.24.88-2.62.88-4.07 0-5.18-3.95-9.45-9-9.95zM12 19c-3.87 0-7-3.13-7-7 0-3.53 2.61-6.43 6-6.92V2.05c-5.06.5-9 4.76-9 9.95 0 5.52 4.47 10 9.99 10 3.31 0 6.24-1.61 8.06-4.09l-2.6-1.53C16.17 17.98 14.21 19 12 19z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeveloperMode;
impl IconShape for MdDeveloperMode {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 5h10v2h2V3c0-1.1-.9-1.99-2-1.99L7 1c-1.1 0-2 .9-2 2v4h2V5zm8.41 11.59L20 12l-4.59-4.59L14 8.83 17.17 12 14 15.17l1.41 1.42zM10 15.17L6.83 12 10 8.83 8.59 7.41 4 12l4.59 4.59L10 15.17zM17 19H7v-2H5v4c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-4h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PV4aeCvgwVxxK/AzwF/zXwvxX+u1ge8CHszzdyvwPsBv818D8V/nvYHv4kXzPsB3858P8V/jvYHv4l/nfYDv5j8X4j/fceDpwHH+dXaBhwC7/OdB/Of7aOCreP5+hytei+fvY4Cv5j8P4j/fbwOvxXO6BLw28Ndc8dLAbwPHeE6/A7w2/3kQ//nM8/oa4KN5Tl8NfBTPS/znQfznM8/rdYDf5orf4ooHAw/meYn/PIj/fOZ5vQ7w21xhXjjxnwfxn888r9cBfpsrzAsn/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/NFb/FFceBl+Z5if88iP985nm9DvDbXGFeOPGfB/Gfzzyv1wF+myvMCyf+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee118Duzyn48BL87zEfx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Oc6Dlzk3+cEsMt/DsR/nuPAbwEvzb/PXwOvA+zyHw/xn+M48FvAS/Mf46+B1wF2+Y+F+M/xU8Bb8x/rp4G34T8W4j/eWwM/xX+OtwF+mv84iP94F4Hj/OfYBU7wHwfxH+u9ge/iP9fbAD/NfwzEf6yfBt6K/1w/A7w1/zEQ/7HMfw3xHwPxH+fBwNP5r3EC2OXfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OA8Gns5/jRPALv9+iP9Yu8Ax/nM9A3gw/zEQ/7G+G3gv/nN9D/De/MdA/Md6beC3+M/1OsBv8x8D8R/vt4HX4j/H7wCvzX8cxH+81wZ+i/8crwP8Nv9xEP85Phr4Kv5jfQzw1fzHQvzn+W7gvfiP8T3Ae/MfD/Gf66uBj+Lf53OAz+Y/B+I/31sDXw08iH+dZwAfDfw0/3kQ/3U+G3hr4KV44f4G+Gngs/nPh/iv92DgrYHjwIO54lZgF/hp4Fb+6yD+f0P8/4b4/w3x/xvi/zf+EaTWhkGIJjS/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDeviceThermostat;
impl IconShape for MdDeviceThermostat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 13V5c0-1.66-1.34-3-3-3S9 3.34 9 5v8c-1.21.91-2 2.37-2 4 0 2.76 2.24 5 5 5s5-2.24 5-5c0-1.63-.79-3.09-2-4zm-4-8c0-.55.45-1 1-1s1 .45 1 1h-1v1h1v2h-1v1h1v2h-2V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEtElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7sE/DXPCfHCvTbwXcCD+d/vd4DX5jkhXrD3Br6L/zt+B3htnhPi+TsOPB04zv8dvwO8Ns8J8fx9NPBV/N/yO8Br85wQz993A+/F/y2/A7w2zwnx/P028Fo8p98BXpv/WxDP328Dr8Vz+h3gtfm/BfH8/TbwWjyn3wFem/9bEM/fbwOvxXP6HeC1+b8F8fz9NvBaPKffAV6b/1sQz99vA6/Fc/od4LX573UceC/grfn3uRX4GeCnxfP328Br8Zx+B3ht/nv9FfDS/Mf5HPH8/TbwWjyn3wFem/8+rw38Fv+xbhXP328Dr8Vz+h3gtfnv89rAb/EfC/H8/TbwWjyn3wFem/8+rw38Fv+xEM/fbwOvxXP6HeC1+e/z2sBv8bx+hxfNceCleE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSGev98GXovn9DvAa/Pf57WB3+J5iRfNawO/xXNCPH+/DbwWz+l3gNfmv89rA7/F8xIvmtcGfovnhHj+fht4LZ7T7wCvzX+f1wZ+i+clXjSvDfwWzwnx/P028Fo8p98BXpv/Pq8N/BbPS7xoXhv4LZ4T4vn7beC1eE6/A7w2/31eG/gtnpd40bw28Fs8J8Tz99vAa/Gcfgd4bf77vDbwWzwv8aJ5beC3eE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSGev98GXovn9DvAa/Pf57WB3+J5iRfNawO/xXNCPH+/DbwWz+l3gNfmv89rA7/F8xIvmtcGfovnhHj+fht4LZ7T7wCvzX+f1wZ+i+clXjSvDfwWzwnx/P028Fo8p98BXpv/Pq8N/BbPS7xoXhv4LZ4T4vn7beC1eE6/A7w2/31eG/gtnpd40bw28Fs8J8Tz99vAa/Gcfgd4bf77vDbwWzwv8aJ5beC3eE6I5++3gdfiOf0O8Nr893lt4Ld4XuJF89rAb/GcEM/fbwOvxXP6HeC1+e/z2sBv8bzEi+a1gd/iOSHgt3heLw0c5zntAn/Nf42PAf6a5/TawG/xvH6bF81x4KV5Tggw//O8DvDbPKfXBn6L/1gIMP/zvA7w2zyn1wZ+i/9YCDD/87wO8Ns8p9cGfov/WAgw//O8DvDbPKfXBn6L/1gIMP/zvA7w2zyn1wZ+i/9YCDD/87wO8Ns8p5cG/or/WH8jwPzP8zrAb/O8Phv4LP5jXAJeW8Br8z/PXwO7PH8vDRzn3++vgV3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hHqopWFG0LLXwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDevices;
impl IconShape for MdDevices {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJZ7XVwMvxXP6G+CjueKrgZfiv9bfAB/N8zLP63WA3wYQz/bawG/xvMTz+m3gtXhOvwO8Nlf8NvBa/Nf6HeC1eV7meb0O8NsA4tleG/gtnpd4Xr8NvBbP6XeA1+aK3wZei/9avwO8Ns/LPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Npc8dvAa/Ff63eA1+Z5mef1OsBvA4hne23gt3he4nn9NvBaPKffAV6bK34beC3+a/0O8No8L/O8Xgf4bQDxbK8N/BbPSzyvrwZemuf018BHc8VXAy/Nf62/Bj6a52We1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMSz+u3gdfiOf0O8Npc8dvAa/Ff63eA1+Z5mef1OsBvA4hne23gt3he4nn9NvBaPKffAV6bK34beC3+a/0O8No8L/O8Xgf4bQDxbK8N/BbPSzyv3wZei+f0O8Brc8VvA6/Ff63fAV6b52We1+sAvw0gnu21gd/ieYnn9dvAa/Gcfgd4ba74beC1+K/1O8Br87zM83od4LcBxLO9NvBbPC/xvH4beC2e0+8Ar80Vvw28Fv+1fgd4bZ6XeV6vA/w2gHi21wZ+i+clntdLA8d5TrvAX3PFSwPH+a+1C/w1z8s8r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtntdv87/ba/O8Xgf4bQDxbC8N/BX/P7wM8NcA4jndCjyI/9ueATyYKxDP6bWB3+L/ttcBfpsrEM/rvYGvBo7xf8sl4L2Bn+bZEM/fceC9gZcGHsy/3ksDx/iPdQn4a/71bgX+GvhuYJfnhPjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wg2LbdBBqEhbAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDvr;
impl IconShape for MdDvr {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.1-.9-2-2-2zm0 14H3V5h18v12zm-2-9H8v2h11V8zm0 4H8v2h11v-2zM7 8H5v2h2V8zm0 4H5v2h2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx3hOl4C/5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X4r3EceCngtYGXBl4bOM5z2gV+G/hr4LeBvwF2+c+F+M/1XsB7A6/Nv81vA98NfA//ORD/8Y4DnwW8N3Cc/xi7wFcDXwPs8h8H8R/rvYCvBo7zn2MX+Gjge/iPgfiP8dLAVwGvzX+N3wbeB7iVfx/Ev99LA78FHOdfdgn4a+C3ueJWrngwV7w28NLAMf5lu8DrAH/Nvx3i3+e9ge/ihXsG8NPATwO/zYvmtYG3Bt4aeBAv3PsA382/DeLf7r2B7+IFuwR8NfDZ/NsdBz4a+GjgGC/Y+wDfzb8e4t/mpYG/4gX7GuCzgV3+YxwHvhp4L16wlwH+mn8dxL/eSwO/BRzn+Xsf4Lv5z/HewHfx/O0CLwPcyosO8a/3W8Br87wuAa8N/DX/uV4a+G3gGM/rt4HX4UWH+Nd5b+C7eP5eBvhr/mu8NPBXPH/vA3w3LxrEi+448HTgOM/rfYDv5r/WewPfxfPaBR4C7PIvQ7zovhr4KJ7X1wAfzYvuOPBWwIOBB3PFrcCtwM8Au7zovht4L57X5wCfzb8M8aK7CBznOV0CHgzs8i97MPBZwHvzwn038DnArfzLjgO3Asd4TrvACf5liBfNewPfxfP6HOCz+Ze9N/Bd/Ou8D/Dd/Ms+G/gsntf7AN/NC4d40fwW8No8p2cAD+Zf9l3Ae/Nv893A+/DCHQf+GngQz+lngLfmhUP8y44DF3leXwN8NC/cewPfxb/P+wDfzQv31cBH8bxOALu8YIh/2WsDv8Xzeh3gt3nBHgw8nf8YDwFu5QV7beC3eF6vA/w2LxjiX/bZwGfxnC4Bx3nhvht4L/5jfA/w3rxwu8AxntPnAJ/NC4b4l/008FY8p98BXpsX7Dhwkf9YJ4BdXrDfBl6L5/QzwFvzgiGe7aWBr+J5vTRwnOf0OcBn84K9N/Bd/Md6G+CnecG+GvgontMu8Nc8r48B/hpAPNtrA7/Fi+ZzgM/mBfts4LP4j/U5wGfzgn028Fm8aF4H+G0A8WyvDfwWL5r3Ab6bF+y7gffiP9bXAB/NC/bRwFfxonkd4LcBxLO9NvBbvGjeB/huXrDvBt6L/1hfA3w0L9hHA1/Fi+Z1gN8GEM/22sBv8aL5HOCzecE+G/gs/mN9DvDZvGCfDXwWL5rXAX4bQDzbSwNfzfN6aeAYz+lzgM/mBXtv4Lv4j/U2wE/zgn018FE8p0vAX/O8Phr4awDxL/tp4K14Tr8DvDYv2HHgIv+xTgC7vGC/DbwWz+lngLfmBUP8yz4b+Cye0y5wghfuu4H34j/G9wDvzQt3ETjOc/oc4LN5wRD/stcGfovn9TrAb/OCPRh4Ov8xHgLcygv21sBP8bxeB/htXjDEv+w4cJHn9TXAR/PCvTfwXfz7vA/w3bxwXw18FM/rBLDLC4Z40fw28Fo8p1uBlwF2eeG+G3gv/m2+B3hvXrjjwF8BD+Y5/Qzw1rxwiBfNewPfxfP6HOCz+Ze9N/Bd/Ou8D/Dd/Ms+G/gsntf7AN/NC4d40e0Cx3hOu8BDgF3+ZQ8GPht4L1647wE+G7iVf9lx4OnAcZ7TJeA4/zLEi+6rgY/ieX0N8NG86I4Dbw08GHgwV9wK/DXw28AuL7rvAt6b5/U5wGfzL0O86I4DtwLHeF7vA3w3/7XeG/guntcl4MHALv8yxL/OewPfxfP3MsBf81/jpYG/4vl7H+C7edEg/vV+G3gtntcu8DrAX/Of66WB3wKO87x+B3htXnSIf70HA38NHOP5ex/gu/nP8d7Ad/H8XQJeGriVFx3i3+algb/iBftu4GOAXf5jHAe+CnhvXrCXAf6afx3Ev917A9/FC7YLfDXwNcAu/zbHgY8CPho4zgv2PsB386+H+Pd5b+C7eOFuBX4a+Gngd3jRvBbw1sBbAw/mhXsf4Lv5t0H8+7008NvAMf5lu8BfA38N7AK3csWDgePASwMvDRznX3YJeG3gr/m3Q/zHeDDw3cBr8V/jd4D3Bm7l3wfxH+u9ga8GjvGf4xLw0cB38x8D8R/vOPDRwEcDx/iPcQn4auCrgV3+4yD+c7038N7Aa/Fv8zPATwPfzX8OxH+N48BLA68NvDTw2sAxntMl4LeBvwZ+G/hrYJf/XIj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HSwPHeU67wF/zXwvx/xvi/zf+EQFrJVBXwSZ5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGpsFixed;
impl IconShape for MdGpsFixed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGpsNotFixed;
impl IconShape for MdGpsNotFixed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGpsOff;
impl IconShape for MdGpsOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06c-1.13.12-2.19.46-3.16.97l1.5 1.5C10.16 5.19 11.06 5 12 5c3.87 0 7 3.13 7 7 0 .94-.19 1.84-.52 2.65l1.5 1.5c.5-.96.84-2.02.97-3.15H23v-2h-2.06zM3 4.27l2.04 2.04C3.97 7.62 3.25 9.23 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c1.77-.2 3.38-.91 4.69-1.98L19.73 21 21 19.73 4.27 3 3 4.27zm13.27 13.27C15.09 18.45 13.61 19 12 19c-3.87 0-7-3.13-7-7 0-1.61.55-3.09 1.46-4.27l9.81 9.81z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGraphicEq;
impl IconShape for MdGraphicEq {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 18h2V6H7v12zm4 4h2V2h-2v20zm-8-8h2v-4H3v4zm12 4h2V6h-2v12zm4-8v4h2v-4h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationDisabled;
impl IconShape for MdLocationDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06c-1.13.12-2.19.46-3.16.97l1.5 1.5C10.16 5.19 11.06 5 12 5c3.87 0 7 3.13 7 7 0 .94-.19 1.84-.52 2.65l1.5 1.5c.5-.96.84-2.02.97-3.15H23v-2h-2.06zM3 4.27l2.04 2.04C3.97 7.62 3.25 9.23 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c1.77-.2 3.38-.91 4.69-1.98L19.73 21 21 19.73 4.27 3 3 4.27zm13.27 13.27C15.09 18.45 13.61 19 12 19c-3.87 0-7-3.13-7-7 0-1.61.55-3.09 1.46-4.27l9.81 9.81z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationSearching;
impl IconShape for MdLocationSearching {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.94 11c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/z4OBB/Ff73d40SH+4x0Hvgt4a/577AJfDXwO/zLEf6zjwNOB4/z3+27gfXjhEP+xfgp4a/7neB/gu3nBEP9xHgw8nf9Zfgd4bV4wxH+c1wZ+i/95xAuG+I/z2sBv8T+PeMEQ/3FeG/gt/ucRLxjiP85rA7/F8/od/mscB16K5yVeMMR/nNcGfovnJf5rvDbwWzwv8YIh/uO8NvBbPC/xX+O1gd/ieYkXDPEf57WB3+J5if8arw38Fs9LvGCI/zivDfwWz0v813ht4Ld4XuIFQ/zHeW3gt3he4r/GawO/xfMSLxji/47XBn6L5yVeMMT/Ha8N/BbPS7xgiP87Xhv4LZ6XeMEQ/3e8NvBbPC/xgiH+ZzgOfBXwMcAu/zavDfwWz0u8YIj/fseB3wJeGvhr4HWAXf71Xhv4LZ6XeMEQ/72OA78FvDTP9tfA6wC7/Ou8NvBbPC/xgiH++xwHfgt4aZ7X9wDvzb/OawO/xfMSLxjiv8dx4LeAl+Z5/Q3w2sAu/zqvDfwWz0u8YIj/eseB3wJemuf1N8BrA7v867028Fs8L/GCIf5rHQd+C3hpntffAK8N7PJv89rAb/G8xAuG+K9zHPgt4KV5Xn8DvDawy7/dawO/xfMSLxjiv8Zx4LeAl+Z5/Q3w2sAu/z6vDfwWz0u8YIh/nffmiu/mRXcc+C3gpXlefwO8NrDLv99rA7/F8xIvGOJF997Ad3HF+wDfzb/sOPBbwEvzvP4GeG1gl/8Yrw38Fs9LvGCIF817A9/Fc3of4Lt5wY4DvwW8NM/rb4DXBnb5j/PawG/xvMQLhviXPRh4Os/f+wDfzfM6DvwW8NI8r78BXhvY5T/WawO/xfMSLxjiRfPewHfx/L0P8N0823Hgt4CX5nn9DfDawC7/8V4b+C2el3jBEC+69wa+i+fvfYDvBo4DvwW8NM/rb4DXBnb5z/HawG/xvMQLhvjXeW/gu3j+PgZ4L+CleV5/A7w2sMt/ntcGfovnJV4wxL/eewPfxYvub4DXBnb5z/XawG/xvMQLhvi3eW/gu/iX/Q3w2sAu//leG/gtnpd4wRD/du8NfBcv2N8Arw3s8l/jtYHf4nmJFwzx7/PewHfxvP4GeG1gl/86rw38Fs9LvGCIf7/3Br6LZ/sb4LWBXf5rvTbwWzwv8YIh/mO8N/BdwN8Arw3s8l/vtYHf4nmJFwzxH+etgd8Gdvnv8drAb/G8xAuG+L/jtYHf4nmJFwzxf8drA7/F8xIvGOL/jtcGfovnJV4wxP8drw38Fs9LvGCI/zteG/gtnpd4wRD/cV4b+C2el/iv8drAb/G8xAuG+I/z2sBv8bzEf43XBn6L5yVeMMR/nNcGfovnJf5rvDbwWzwv8YIh/uO8NvBbPC/xX+O1gd/ieYkXDPEf57WB3+J5if8arw38Fs9LvGCI/zivDfwWz+u3+a9xHHhpnpd4wRD/cV4b+C3+5xEvGOI/zmsDv8X/POIFQ/zHeW3gt/ifR7xgiP84Dwaezv8svwO8Ni8Y4j/WTwNvxf8c7wN8Ny8Y4j/WceBW4Bj//b4HeG9eOMR/vOPAdwNvxX+PS8BXA5/Nvwzxn+fBwIP5r/fbvOgQ/78h/n/jHwEAkKBByBRQ4wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMobileFriendly;
impl IconShape for MdMobileFriendly {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zM7.01 13.47l-2.55-2.55-1.27 1.27L7 16l7.19-7.19-1.27-1.27z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMobileOff;
impl IconShape for MdMobileOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.76 2.49L1.49 3.76 5 7.27V21c0 1.1.9 2 2 2h10c1.02 0 1.85-.77 1.98-1.75l1.72 1.72 1.27-1.27L2.76 2.49zM7 19V9.27L16.73 19H7zM17 5v9.17l2 2V3c0-1.1-.9-2-2-2H7c-.85 0-1.58.54-1.87 1.3L7.83 5H17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/f7w38N08J8T/D98FPAR4bZ4T4v++7wLeG/gd4LV5Toj/274LeG+u+B3gtXlOiP+7vgt4b57td4DX5jkh/m/6LuC9eU6/A7w2zwnxf893Ae/N8/od4LV5Toj/W74LeG+ev98BXpvnhPi/47uA9+YF+x3gtXlOiP8bvgt4b1643wFem+eE+N/vu4D35l/2O8Br85wQ/7t9F/DevGh+B3htnhPif6/vAt6bF93vAK/Nc0L87/RdwHvz/F0CdoEH8Zx+B3htnhPif5/vAt6b5+8S8NrAVwOvxXP6HeC1eU6I/12+C3hvnr9LwGsDfw38NvBaPKffAV6b54T43+O7gPfm+bsEvDbw11zx28Br8Zx+B3htnhPif4fvAt6b5+8S8NrAX/Nsvw28Fs/pd4DX5jkh/uf7LuC9ef4uAa8N/DXP6beB1+I5/Q7w2jwnxP9s3wW8N8/fJeC1gb/mef028Fo8p98BXpvnhPif67uA9+b5uwS8NvDXPH+/DbwWz+l3gNfmOSH+Z/ou4L15wd4G+GlesN8GXovn9DvAa/OcEP/zfBfw3rxwrwP8Ni/YbwOvxXP6HeC1eU6I/1m+C3hv/mWvA/w2L9hvA6/Fc/od4LV5Toj/Ob4LeG9eNK8D/DYv2G8Dr8Vz+h3gtXlOiP8Zvgt4b150rwP8Ni/YbwOvxXP6HeC1eU6I/37fBbw3/zqvA/w2L9hvA6/Fc/od4LV5Toj/Xt8FvDf/eq8D/DYv2G8Dr8Vz+h3gtXlOiP8+3wW8N/82rwP8Ni/YbwOvxXP6HeC1eU6I/x7fBbw3/3avA/w2L9hvA6/Fc/od4LV5Toj/et8FvDf/Pq8D/DYv2G8Dr8Vz+h3gtXlOiP9a3wW8N/9+rwP8Ni/YbwOvxXP6HeC1eU6I/zrfBbw3/zFeB/htXrDfBl6L5/Q7wGvznBD/Nb4LeG/+47wO8Nu8YL8NvBbP6XeA1+Y5If7zfRfw3vzHeh3gt3nBfht4LZ7T7wCvzXNC/Of6LuC9+Y/3OsBv84L9NvBaPKffAV6b54T4z/NdwHvzn+N1gN/mBftt4LV4Tr8DvDbPCfGf47uA9+Y/z+sAv80L9tvAa/Gcfgd4bZ4T4j/edwHvzX+u1wF+mxfst4HX4jn9DvDaPCfEf6zvAt6b/3yvA/w2L9hvA6/Fc/od4LV5Toj/ON8FvDf/NV4H+G1esN8GXovn9DvAa/OcEP8xvgt4b/7rvA7w27xgvw28Fs/pd4DX5jkh/v2+C3hv/mu9DvDbvGC/DbwWz+l3gNfmOSH+fb4LeG/+670O8Nu8YL8NvBbP6XeA1+Y5If7tvgt4b/57vA7w27xgvw28Fs/pd4DX5jkh/m2+C3hv/vu8DvDbvGC/DbwWz+l3gNfmOSH+9b4LeG/+e70O8Nu8YL8NvBbP6XeA1+Y5If51vgt4b/77vQ7w27xgvw28Fs/pd4DX5jkhXnTfBbw3/zP8NbDLC/bSwHGe0+8Ar81zQrxovgt4b/53+x3gtXlOiH/ZdwHvzf9+vwO8Ns8J8cJ9F/De/N/wO8Br85wQL9h3Ae/N/x2/A7w2zwnx/L038N783/LXwEfznBD/vyH+f0P8/4b4/w3x/xv/CPvxuEFc2j8aAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNetworkCell;
impl IconShape for MdNetworkCell {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2,22h20V2L2,22z M20,20h-3V9.83l3-3V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv9Zx4KV44f4G2OW/BuI/x3HgtYCXBl4aeDDw0vzr/DVwK/DXwF8DvwPs8h8L8R/nrYDXBl4beGn+c/w18NvAbwM/w78f4t/npYH3At4bOM5/rVuBnwa+B/hr/m0Q/zbvBXw08NL8z/DXwFcD38O/DuJf572AzwYezP9MtwKfDXwPLxrEi+atga8CHsy/ziXgr4HfBnaBv+aK3+b5e22ueGngOPDawEsDx/jXuRX4GOCneeEQL9yDga8C3poX3c8APw38NnAr/zEeDLw28NbAW/Gi+2ngY4Bbef4QL9hHAZ8NHOdf9jvAdwPfzX+N9wbeG3gt/mW7wGcDX8PzQjyv48B3AW/NC3cJ+Gngs4Fb+e/xYOCzgbcGjvHC/TTwPsAuz4Z4Ti8N/BTwYF6wS8BXA18N7PI/w3Hgo4HP4oW7FXgb4K+5AvFs7w18Fy/czwAfDdzKv95rAceBl+bZXpsrfptn+2tgF/gd/vUeDHw18Fa8cO8DfDeAuOK9ge/iBbsEvDXw27zo3gp4a+ClgZfm3+avgb8Gfhr4GV50rw38NHCMF+x9gO8W8FXAR/OC/Qzw3sAu/7K3At4beGv+c3w38NPAz/AvOw58N/BWvGCfLeC9ge/i+dsFXgf4a16w48B7AR8NPJj/GrcCXw18D7DLC/bSwG8Bx3n+3kdc8d7Ad/H87QKvA/w1z+ujgM8GjvPfYxf4bOBreF4vDfwWcJzn732A7xbP9t7Ad/H87QKvA/w1V7w28F3Ag/mf4VbgfYDf5oqXBn4LOM7z9z7AdwOI5/TewHfx/O0CbwO8FfDR/M/01cDPAD8FHOf5ex/gu7kC8bzeG/gu/m96H+C7eTbE8/fewHfxf8v7AN/Nc0K8YO8NfBf/N7wP8N08L8QL997Ad/G/2/sA383zh/iXvTfwXfzv9D7Ad/OCIV407w18F/+7vA/w3bxwiBfdewPfxf8O7wN8N/8yxL/OewPfxf9s7wN8Ny8axL/eewPfxf9M7wN8Ny86xL/NewPfxf8s7wN8N/86iH+79wa+i/8Z3gf4bv71EP8+7w18F/+93gf4bv5tEP9+7w18F/893gf4bv7tEP8x3hv4Lv5rvQ/w3fz7IP7jvDfwXfzXeB/gu/n3Q/zHem/gu/jP9T7Ad/MfA/Ef772B7+I/x/sA381/HMR/jvcGvov/WO8DfDf/sRD/ed4b+C7+Y7wP8N38x0P853pv4Lv493kf4Lv5z4H4z/fewHfxb/M+wHfznwfxX+O9ge/iX+d9gO/mPxfiv857A9/Fi+Z9gO/mPx/iv9Z7A9/FC/c+wHfzXwPxX++9ge/i+Xsf4Lv5r4P47/HRwFfxnN4H+G7+ayH++3w38F5c8T7Ad/NfD/Hf67uB3wa+m/8eiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjGamqIL1abloAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNetworkWifi;
impl IconShape for MdNetworkWifi {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M24,8.98C20.93,5.9,16.69,4,12,4C7.31,4,3.07,5.9,0,8.98L12,21v0l0,0L24,8.98z M2.92,9.07C5.51,7.08,8.67,6,12,6 s6.49,1.08,9.08,3.07l-1.43,1.43C17.5,8.94,14.86,8,12,8s-5.5,0.94-7.65,2.51L2.92,9.07z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNfc;
impl IconShape for MdNfc {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 20h16V4H4v16z",
            }
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 18H4V4h16v16zM18 6h-5c-1.1 0-2 .9-2 2v2.28c-.6.35-1 .98-1 1.72 0 1.1.9 2 2 2s2-.9 2-2c0-.74-.4-1.38-1-1.72V8h3v8H8V8h2V6H6v12h12V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v81zgO7PIfzzyv1wF+G0A822sDv8XzEv/5Xhr4bOCt+Y9nntfrAL8NIJ7ttYHf4nmJ/1wvDfwW8DfAa/Mfzzyv1wF+G0A822sDv8XzEv95Xhr4LeA48DvAa/Mf77V5Xn8N7AKIZ3tt4Ld4XuI/x0sDvwUc54rfAV6b/1qIZ3tt4Ld4XuI/3ksDvwUc59l+B3ht/mshnu21gd/ieYn/WC8N/BZwnOf0O8Br818L8WyvDfwWz0v8x3lp4LeA4zyv3wFem/9aiGd7beC3eF7iP8ZLA78FHOf5+2vgo/nP8QzgVp4X4tleG/gtnpf493tp4LeA4/z3+WngfYBdng3xbK8N/BbPS/z7vDTwW8Bx/vvtAg8BdrkC8WyvDfwWz0v8+/w28Fr8z/HTwNtwBeLZXhv4LZ6X+Pc5Dvw28FL8z/EQ4FYA8WyvDfwWz0v8+x0Hfht4Kf5neB3gtwHEs7028Fs8L/Ef4zjw28BL8fz9DvDa/Mczz+t1gN8GEM/22sBv8bzEf5zjwG8DL8Xz+h3gtfmPZ57X6wC/DSCe7bWB3+J5if9Yx4HfBl6K5/Q7wGvzH888r9cBfhtAPNtrA7/F8xL/8Y4Dvw28FM/2O8Br8x/PPK/XAX4bQDzbawO/xfMS/zmOA78NvBRX/A7w2vzHM8/rdYDfBhDP9trAb/G8xH+e48BvAy8F/A7w2vzHM8/rdYDfBhDP9trAb/G8xH+u48BvA7vAa/Mfzzyv1wF+G0A822sDv8XzEv/5jgOfDXw0//HM83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3hev83/bq/N83od4LcBxLO9NPBX/P/wMsBfA4jndCvwIP5vewbwYK5APKfXBn6L/9teB/htrkA8r/cGvho4xv8tl4D3Bn6aZ0M8f8eB9wZeGngw/3ovDRzjP9Yl4K/517sV+Gvgu4FdnhPiP8dvA6/Ff6zfAV6b/1iI/xy/DbwW/7F+B3ht/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjg0+gQYiaYr8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdResetTv;
impl IconShape for MdResetTv {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 10h-8.01V7L9 11l3.99 4v-3H21v5H3V5h18v3h2V5c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2v-5H23c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz99x4L2AlwYezP9utwJ/DXwPsMtzQjyv9wa+CjjO/y27wMcA382zIZ7TawO/xf9trwP8NlcgntPTgQfzf9utwEO4AvFsLw38Ff8/vAzw1wDi2V4b+C1eNH8D7PI/y3HgpXjRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfN6wC/zb/dceC9gJfmir8GvgfY5d/utYHf4kXzOsBvA4hne23gt3jRvA7w2/zbvDfwVcBxntMu8DHAd/Nv89rAb/GieR3gtwHEs7028Fu8aF4H+G3+9d4a+CmueAbw01zx1sCDuOJ1gN/mX++1gd/iRfM6wG8DiGd7beC3eNG8DvDb/Os9HXgw8DPAewO7XHEc+G7grYBbgYfwr/fawG/xonkd4LcBxLO9NvBbvGheB/ht/nVeGvgrrjgB7PKcjgMXueJlgL/mX+e1gd/iRfM6wG8DiGd7beC3eNG8DvDb/Ou8NvBbwN8AL83z99fASwGvA/w2/zqvDfwWL5rXAX4bQDzbawO/xYvmdYDf5l/ntYHfAn4HeG2ev98GXgt4HeC3+dd5beC3eNG8DvDbAOLZXhv4LV40rwP8Nv86rw38FvA7wGvz/P028FrA6wC/zb/OawO/xYvmdYDfBhDP9trAb/GieR3gt3nRvTXwXsBbA7vAX/P8vTRwHPhp4HuAn+ZF99rAb/GieR3gtwHEs7028Fu8aF4H+G1eNJ8NfBb/Np8DfDYvmtcGfosXzesAvw0gnu21gd/iRfM6wG/zorkIHOff5lbgIbxoXhv4LV40rwP8NoB4ttcGfosXzesAv82Lxvz7iBfNawO/xYvmdYDfBhDP9trAb/GieR3gt3nRmH8f8aJ5beC3eNG8DvDbAOLZXhv4LV40rwP8Ni8a8+8jXjSvDfwWL5rXAX4bQDzbawO/xYvmdYDf5kVj/n3Ei+a1gd/iRfM6wG8DiGd7beC3eNG8DvDbvGjMv4940bw28Fu8aF4H+G0A8WyvDfwWL5rXAX6bF4359xEvmtcGfosXzesAvw0gnu21gd/iRfM6wG/zojH/PuJF89rAb/GieR3gtwHEs7028Fu8aF4H+G1eNObfR7xoXhv4LV40rwP8NoB4ttcGfosXzesAv82LZhc4xr/NM4AH86J5beC3eNG8DvDbAOLZXhv4LV40rwP8Ni+azwY+i3+bzwE+mxfNawO/xYvmdYDfBhDP9trAb/GieR3gt3nRvTXw1sCDedHcCvw08NO86F4b+C1eNK8D/DaAeLbXBn6LF83rAL/N/yyvDfwWL5rXAX4bQDzbawO/xYvmdYDf5n+W1wZ+ixfN6wC/DSCe7bWB3+JF89fALv+zHAdemhfN6wC/DSCe7aWBv+L/h5cB/hpAPKdbgQfxf9szgAdzBeI5vTbwW/zf9jrAb3MF4nm9N/DVwDH+b7kEvDfw0zwb4vk7Drw38NLAg/nf7Vbgr4HvBnZ5Toj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSy0sFBlKXo8QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenLockLandscape;
impl IconShape for MdScreenLockLandscape {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 5H3c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm-2 12H5V7h14v10zm-9-1h4c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1v-1c0-1.11-.9-2-2-2-1.11 0-2 .9-2 2v1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1zm.8-6c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2v1h-2.4v-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenLockPortrait;
impl IconShape for MdScreenLockPortrait {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 16h4c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1v-1c0-1.11-.9-2-2-2-1.11 0-2 .9-2 2v1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1zm.8-6c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2v1h-2.4v-1zM17 1H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenLockRotation;
impl IconShape for MdScreenLockRotation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23.25 12.77l-2.57-2.57-1.41 1.41 2.22 2.22-5.66 5.66L4.51 8.17l5.66-5.66 2.1 2.1 1.41-1.41L11.23.75c-.59-.59-1.54-.59-2.12 0L2.75 7.11c-.59.59-.59 1.54 0 2.12l12.02 12.02c.59.59 1.54.59 2.12 0l6.36-6.36c.59-.59.59-1.54 0-2.12zM8.47 20.48C5.2 18.94 2.86 15.76 2.5 12H1c.51 6.16 5.66 11 11.95 11l.66-.03-3.81-3.82-1.33 1.33zM16 9h5c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1v-.5C21 1.12 19.88 0 18.5 0S16 1.12 16 2.5V3c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1zm.8-6.5c0-.94.76-1.7 1.7-1.7s1.7.76 1.7 1.7V3h-3.4v-.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenRotation;
impl IconShape for MdScreenRotation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.48 2.52c3.27 1.55 5.61 4.72 5.97 8.48h1.5C23.44 4.84 18.29 0 12 0l-.66.03 3.81 3.81 1.33-1.32zm-6.25-.77c-.59-.59-1.54-.59-2.12 0L1.75 8.11c-.59.59-.59 1.54 0 2.12l12.02 12.02c.59.59 1.54.59 2.12 0l6.36-6.36c.59-.59.59-1.54 0-2.12L10.23 1.75zm4.6 19.44L2.81 9.17l6.36-6.36 12.02 12.02-6.36 6.36zm-7.31.29C4.25 19.94 1.91 16.76 1.55 13H.05C.56 19.16 5.71 24 12 24l.66-.03-3.81-3.81-1.33 1.32z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenSearchDesktop;
impl IconShape for MdScreenSearchDesktop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,18 C21.1,18 21.99,17.1 21.99,16 L22,6 C22,4.89 21.1,4 20,4 L4,4 C2.89,4 2,4.89 2,6 L2,16 C2,17.1 2.89,18 4,18 L0,18 L0,20 L24,20 L24,18 L20,18 Z M4,16 L4,6 L20,6 L20,16 L20,16.01 L4,16 Z M9.0967,9.9531 C9.0967,8.9261 9.9327,8.0891 10.9607,8.0891 C11.9877,8.0891 12.8247,8.9261 12.8247,9.9531 C12.8247,10.9801 11.9877,11.8171 10.9607,11.8171 C9.9327,11.8171 9.0967,10.9801 9.0967,9.9531 Z M16.1287,14.1891 L13.6467,11.7071 C13.9777,11.2021 14.1737,10.6001 14.1737,9.9531 C14.1737,8.1811 12.7327,6.7401 10.9607,6.7401 C9.1887,6.7401 7.7467,8.1811 7.7467,9.9531 C7.7467,11.7251 9.1887,13.1671 10.9607,13.1671 C11.5967,13.1671 12.1857,12.9751 12.6847,12.6561 L15.1737,15.1441 L16.1287,14.1891 Z",
            }
            rect {
                height: "24",
                style: "fill:none",
                width: "24",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/db4KeGn+490K/DXwPcAu/zqI/xqfDXwW/7l2gY8BvpsXHeI/33sD38V/ndcBfpsXDeI/13sD38V/rVuBh/CiQfzneW/gu/jv8TLAX/MvQ/zneG/gu3hOl4C/5nm9FldcAv6a5/VaXHEJ+Gue12vxvF4H+G3+ZYj/eO8NfBfP63eA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Md6b+C7eP5+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsR/nPcGvosX7HeA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Md4b+C7eOF+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsS/33sD38W/7HeA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Pu8N/BdvGh+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsS/3XsD38WL7neA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Nu8N/Bd/Ov8DvDaPC9zxe8Ar83zMlf8DvDaPC/zvF4H+G3+ZYh/vfcGvot/vd8BXpvnZa74HeC1eV7mit8BXpvnZZ7X6wC/zb8M8a/z3sB38W/zO8Br87zMFb8DvDbPy1zxO8Br87zM83od4Lf5lyFedC8N/BX/drvAX/O8XpsrdoG/5nm9NlfsAn/N83ptntfrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie2ngr/jf4WWAv+ZfhvjXuRV4EP+zPQN4MC8axL/OawO/xf9srwP8Ni8axL/eewNfDRzjf5ZLwHsDP82LDvFvcxx4b+ClgQfz3+tW4K+B7wZ2+ddB/P+G+P8N8f8b4v83xP9v/COCA7pB1W+fYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSdStorage;
impl IconShape for MdSdStorage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2h-8L4.02 8 4 20c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 6h-2V4h2v4zm3 0h-2V4h2v4zm3 0h-2V4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xksDx/ivdQn4a/59EP8+rw18F/Bg/nvcCrwP8Nv82yD+7d4b+C7+Z3gf4Lv510P82xwHng4c53+GXeAhwC7/Ooh/m48Gvor/WT4G+Gr+dRD/Nt8NvBf/s3wP8N786yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gtntPvAK/Nf43fBl6L5/Q7wGvzr4P4t/lt4LV4Tr8DvDb/NX4beC2e0+8Ar82/DuLf5reB1+I5/Q7w2vzX+GrgpXlOfw18NP86iH+b3wZei+f0O8Br878L4t/mt4HX4jn9DvDa/O+C+Lf5beC1eE6/A7w2//VeGvhr/m0Q/za/DbwWz+l3gNfmv95F4HWAv+ZfD/Fv89vAa/Gcfgd4bf7rGdgFXgf4a/51EP82vw28Fs/pd4DX5r+euWIXeB3gr3nRIf5tfht4LZ7T7wCvzX8982y7wOsAf82LBvFv89vAa/Gcfgd4bf7rmee0C7wO8Nf8yxD/Nr8NvBbP6XeA1+a/nnleu8DrAH/NC4f4t/lt4LV4Tr8DvDb/9czztwu8DvDXvGCIf5vfBl6L5/Q7wGvzb/fSwFfxr/favGC7wOsAf83zh/i3+W3gtXhOvwO8Nv92rw38Fv/xdoHXAf6a54X4t/lt4LV4Tr8DvDb/dq8N/Bb/OXaB1wH+mueE+Lf5beC1eE6/A7w2/3avDfwW/zkuAa8N/DXPCfFv89vAa/Gcfgd4bf7tXhv4Lf7jXQJeG/hrnhfi3+a3gdfiOf0O8Nr827008NX8670WL9gl4LWBv+b5Q/zb/DbwWjyn3wFem/965vm7BLw28Ne8YIh/m98GXovn9DvAa/NfzzyvS8BrA3/NC4f4t/lt4LV4Tr8DvDb/9cxzugS8NvDX/MsQ/za/DbwWz+l3gNfmv555tkvAawN/zYsG8W/z28Br8Zx+B3ht/uuZKy4Brw38NS86xL/NbwOvxXP6HeC1+a9n4BLw2sBf86+D+Lf5beC1eE6/A7w2//V2gdcG/pp/PcS/zW8Dr8Vz+h3gtfmv99LAX/Nvg/i3+W3gtXhOvwO8Nv+7IP5tfht4LZ7T7wCvzf8uiH+b3wZei+f0O8Br81/jq4GX4jn9DfDR/Osg/m1+G3gtntPvAK/Nf43fBl6L5/Q7wGvzr4P4t/lt4LV4Tr8DvDb/NX4beC2e0+8Ar82/DuLf5reB1+J/lt8BXpt/HcS/zW8Dr8X/LL8DvDb/Ooh/m98GXov/WX4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a7gffif5avAT6afx3Ev81HA1/F/ywfA3w1/zqIf5vjwK3AMf5nuAQ8GNjlXwfxb/fewHfxP8PbAD/Nvx7i3+e1ge8GHsR/j2cA7w38Nv82iP8YLw0c57/WLvDX/Psg/n9D/P/GPwLzI8RBEqDqsQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSendToMobile;
impl IconShape for MdSendToMobile {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,17h2v4c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V3c0-1.1,0.9-1.99,2-1.99L17,1c1.1,0,2,0.9,2,2v4h-2V6H7v12h10V17z M22,12 l-4-4v3h-5v2h5v3L22,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf7jvRXw0sCDueJW4K+Bn+E/nnlerwP8NoB4ttcGfovnJf7jfBbw0cBxnr9d4KuBz+E/jnlerwP8NoB4ttcGfovnJf79jgO/Bbw0L5q/Bl4H2OXfzzyv1wF+G0A822sDv8XzEv8+x4HfAl6af52/Bl4H2OXfxzyv1wF+G0A822sDv8XzEv8+vwW8Nv82Pw28Df8+5nm9DvDbAOLZXhv4LZ6X+Ld5aeC1gK/m3+d1gN/m3848r9cBfhtAPNtrA7/F8xIvuuPARwHvDTyY/xg/A7w1/3bmeb0O8NsA4tleG/gtnpd40bw08FPAg/mPJ/7tzPN6HeC3AcSzvTbwWzwv8S97aeC3gOP85/ht4LeBvwZ+hn8d87xeB/htAPFsrw38Fs9L/Mv+Cnhp/mvcCrwN8Ne8aMzzeh3gtwHEs7028Fs8L/HCvTfwXfzX2gVeB/hr/mXmeb0O8NsA4tleG/gtnpd44X4aeCv+6+0CDwF2eeHM83od4LcBxLO9NvBbPC/xwl0EjvPf42OAr+aFM8/rdYDfBhDP9trAb/G8xAtn/vv8NfAyvHDmeb0O8NsA4tleG/gtnpd44cx/L/HCmef1OsBvA4hne23gt3he4oX7beC1+O8jXjjzvF4H+G0A8WyvDfwWz0u8cB8NfBX/Pf4GeGleOPO8Xgf4bQDxbK8N/BbPS7xwx4FbgWP81/sY4Kt54czzeh3gtwHEs7028Fs8L/Eve2/gu/ivdQl4MLDLC2ee1+sAvw0gnu21gd/ieYkXzXsD38V/jUvAawN/zb/MPK/XAX4bQDzbawO/xfMSL7rXBr4beBD/ef4GeG/gr3nRmOf1OsBvA4hne23gt3he4l/vpYG3Bh4MPJh/v13gr4G/Bn6afx3zvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF6vzf9uv83zeh3gtwHEs7028Fv8//A6wG8DiOe0Cxzj/7ZLwHGuQDynzwY+i//bPgf4bK5APK/vBt6L/5u+B3hvng3x/L038NHAS/F/w+8A3w18N88J8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EBtG/QZZF1x4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSettingsSystemDaydream;
impl IconShape for MdSettingsSystemDaydream {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 16h6.5c1.38 0 2.5-1.12 2.5-2.5S16.88 11 15.5 11h-.05c-.24-1.69-1.69-3-3.45-3-1.4 0-2.6.83-3.16 2.02h-.16C7.17 10.18 6 11.45 6 13c0 1.66 1.34 3 3 3zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/f7w38N08J8T/D98FPAR4bZ4T4v++7wLeG/gd4LV5Toj/274LeG+u+B3gtXlOiP+7vgt4b57td4DX5jkh/m/6LuC9eU6/A7w2zwnxf893Ae/N8/od4LV5Toj/W74LeG+ev98BXpvnhPi/47uA9+YF+x3gtXlOiP8bvgt4b1643wFem+eE+N/vu4D35l/2O8Br85wQ/7t9F/DevGh+B3htnhPif6/vAt6bF93vAK/Nc0L87/RdwHvz/F0CdoEH8Zx+B3htnhPif5/vAt6b5+8S8NrAVwOvxXP6HeC1eU6I/12+C3hvnr9LwGsDfw38NvBaPKffAV6b54T43+O7gPfm+bsEvDbw11zx28Br8Zx+B3htnhPif4fvAt6b5+8S8NrAX/Nsvw28Fs/pd4DX5jkh/uf7LuC9ef4uAa8N/DXP6beB1+I5/Q7w2jwnxP9s3wW8N8/fJeC1gb/mef028Fo8p98BXpvnhPif67uA9+b5uwS8NvDXPH+/DbwWz+l3gNfmOSH+Z/ou4L15/i4Brw38NS/YbwOvxXP6HeC1eU6I/3m+C3hvnr9LwGsDf80L99vAa/Gcfgd4bZ4T4n+W7wLem+fvEvDawF/zL/tt4LV4Tr8DvDbPCfE/x3cB783zdwl4beCvedH8NvBaPKffAV6b54T4n+G7gPfm+bsEvDbw17zofht4LZ7T7wCvzXNC/Pf7LuC9ef4uAa8N/DX/Or8NvBbP6XeA1+Y5If57fRfw3jx/l4DXBv6af73fBl6L5/Q7wGvznBD/fb4LeG+ev0vAawN/zb/NbwOvxXP6HeC1eU6I/x7fBbw3z98l4LWBv+bf7reB1+I5/Q7w2jwnxH+97wLem+fvEvDawF/z7/PbwGvxnH4HeG2eE+K/1ncB783zdwl4beCv+ff7beC1eE6/A7w2zwnxX+e7gPfm+bsEvDbw1/zH+G3gtXhOvwO8Ns8J8V/ju4D35vm7BLw28Nf8x/lt4LV4Tr8DvDbPCfGf77uA9+b5uwS8NvDX/Mf6beC1eE6/A7w2zwnxn+u7gPfm+bsEvDbw1/zH+23gtXhOvwO8Ns8J8Z/nu4D35vm7BLw28Nf85/ht4LV4Tr8DvDbPCfGf47uA9+b5uwS8NvDX/Of5beC1eE6/A7w2zwnxH++7gPfm+bsEvDbw1/zn+m3gtXhOvwO8Ns8J8R/ru4D35vm7BLw28Nf85/tt4LV4Tr8DvDbPCfEf57uA9+b5uwS8NvDX/Nf4beC1eE6/A7w2zwnxH+O7gPfm+bsEvDbw1/zX+W3gtXhOvwO8Ns8J8e/3XcB78/xdAl4b+Gv+a/028Fo8p98BXpvnhPj3+S7gvXn+LgGvDfw1//V+G3gtntPvAK/Nc0L8230X8N48f5eA1wb+mv8evw28Fs/pd4DX5jkh/m2+C3hvnr9LwGsDf81/n98GXovn9DvAa/OcEP963wW8N8/fJeC1gb/mv9dvA6/Fc/od4LV5Toh/ne8C3pvn7xLw2sBf89/vt4HX4jn9DvDaPCfEi+67gPfmBbsVuJX/GV4aOM5z+h3gtXlOiBfNdwHvzf9uvwO8Ns8J8S/7LuC9+d/vd4DX5jkhXrjvAt6b/xt+B3htnhPiBfsu4L35v+N3gNfmOSGev/cG3pv/W/4a+GieE+L/N8T/b4j/3xD/vyH+f+MfAXeq00GMVGjfAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellular0Bar;
impl IconShape for MdSignalCellular0Bar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,6.83V20H6.83L20,6.83 M22,2L2,22h20V2L22,2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/f7w38N08J8T/D98FPAR4bZ4T4v++7wLeG/gd4LV5Toj/274LeG+u+B3gtXlOiP+7vgt4b57td4DX5jkh/m/6LuC9eU6/A7w2zwnxf893Ae/N8/od4LV5Toj/W74LeG+ev98BXpvnhPi/47uA9+YF+x3gtXlOiP8bvgt4b1643wFem+eE+N/vu4D35l/2O8Br85wQ/7t9F/DevGh+B3htnhPif6/vAt6bF93vAK/Nc0L87/RdwHvzr/M7wGvznBD/+3wX8N786/0O8No8J8T/Lt8FvDf/Nr8DvDbPCfG/x3cB782/3e8Ar81zQvzv8F3Ae/Pv8zvAa/OcEP/zfRfw3vz7/Q7w2jwnxP9s3wW8N/8xfgd4bZ4T4n+u7wLem/84vwO8Ns8J8T/TdwHvzX+s3wFem+eE+J/nu4D35j/e7wCvzXNC/M/yXcB785/jd4DX5jkh/uf4LuC9+c/zO8Br85wQ/zN8F/De/Of6HeC1eU6I/37fBbw3//l+B3htnhPiv9d3Ae/Nf43fAV6b54T47/NdwHvzX+d3gNfmOSH+e3wX8N781/od4LV5Toj/et8FvDf/9X4HeG2eE+K/1ncB781/j98BXpvnhPiv813Ae/Pf53eA1+Y5If5rfBfw3vz3+h3gtXlOiP983wW8N//9fgd4bZ4T4j/XdwHvzf8MvwO8Ns8J8Z/nu4D35n+O3wFem+eE+M/xXcB78z/L7wCvzXNC/Mf7LuC9+Z/nd4DX5jkh/mN9F/De/M/0O8Br85wQ/3G+C3hv/uf6HeC1eU6I/xjfBbw3/7P9DvDaPCfEv993Ae/N/3y/A7w2zwnx7/NdwHvzv8PvAK/Nc0L8230X8N787/E7wGvznBD/Nt8FvDf/u/wO8No8J8S/3ncB783/Pr8DvDbPCfGv813Ae/O/0+8Ar81zQrzovgt4b/73+h3gtXlOiBfNdwHvzf9uvwO8Ns8J8S/7LuC9+d/vd4DX5jkhXrjvAt6b/xt+B3htnhPiBfsu4L35v+N3gNfmOSGev/cG3pv/W/4a+GieE+L/N8T/b4j/3xD/vyH+f+MfAT69akH90ysaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellular4Bar;
impl IconShape for MdSignalCellular4Bar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 22h20V2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6eXBo7xgv0NsMu/DPG/028Dr8UL9jrAb/MvQ/zv9NvAa/GCvQ7w2/zLEP87/TbwWrxgrwP8Nv8yxP9Ovw28Fi/Y6wC/zb8M8b/TbwOvxQv2OsBv8y9D/O/028Br8YK9DvDb/MsQ/zv9NvBavGCvA/w2/zLE/06/DbwWL9jrAL/Nvwzxv9NvA6/FC/Y6wG/zL0P87/TbwGvxgr0O8Nv8yxD/O/028Fq8YK8D/Db/MsT/Tr8NvBYv2OsAv82/DPG/028Dr8UL9jrAb/MvQ/z3+G3gtXjBXgf4bV6w3wZeixfsdYDf5l+G+O/x28Br8YK9DvDbvGC/DbwWL9jrAL/Nvwzx3+O3gdfiBXsd4Ld5wX4beC1esNcBfpt/GeK/x28Dr8UL9jrAb/OC/TbwWrxgrwP8Nv8yxH+P3wZeixfsdYDf5gX7beC1eMFeB/ht/mWI/x6/DbwWL9jrAL/NC/bbwGvxgr0O8Nv8yxD/PX4beC1esNcBfpsX7LeB1+IFex3gt/mXIf57/DbwWrxgrwP8Ni/YbwOvxQv2OsBv8y9D/Pf4beC1eMFeB/htXrDfBl6LF+x1gN/mX4b47/HbwGvxgr0O8Nu8YL8NvBYv2OsAv82/DPHf47eB1+IFex3gt3nBfht4LV6w1wF+m38Z4r/HbwOvxQv2OsBv84L9NvBavGCvA/w2/zLEf4/fBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m68GXooX7GOAv+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m98GXosX7HWA3+YF+23gtXjBXgf4bV6w3wZeixfsdYDf5l+G+Lf5beC1eMFeB/htXrDfBl6LF+x1gN/mBftt4LV4wV4H+G3+ZYh/m5cGjvOC/TWwywv20sBxXrC/BnZ5wV4aOM4L9tfALv8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHKPe2QTAwlcEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellularAlt;
impl IconShape for MdSignalCellularAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 4h3v16h-3zM5 14h3v6H5zm6-5h3v11h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/f7w38N08J8T/D98FPAR4bZ4T4v++7wLeG/gd4LV5Toj/274LeG+u+B3gtXlOiP+7vgt4b57td4DX5jkh/m/6LuC9eU6/A7w2zwnxf893Ae/N8/od4LV5Toj/W74LeG+ev98BXpvnhPi/47uA9+YF+x3gtXlOiP8bvgt4b1643wFem+eE+N/vu4D35l/2O8Br85wQ/7t9F/DevGh+B3htnhPif6/vAt6bF93vAK/Nc0L87/RdwHvzr/M7wGvznBD/+3wX8N786/0O8No8J8T/Lt8FvDf/Nr8DvDbPCfG/x3cB783zeh1eNLvAX/OcEP87fBfw3jx/4t8O8T/fdwHvzQsm/u0Q/7N9F/DevHDi3w7xP9d3Ae/Nv0z82yH+Z/ou4L150Yjn9dLAMZ7TJeCveU6I/3m+C3hvXnTief028Fo8p98BXpvnhPif5buA9+ZfRzyv3wZei+f0O8Br85wQ/3N8F/De/OuJ5/XbwGvxnH4HeG2eE+J/hu8C3pt/G/G8fht4LZ7T7wCvzXNC/Pf7LuC9+bcTz+u3gdfiOf0O8No8J8R/r+8C3pt/H/G8fht4LZ7T7wCvzXNC/Pf5LuC9+fcTz+u3gdfiOf0O8No8J8R/j+8C3pv/GOJ5/TbwWjyn3wFem+eE+K/3XcB78x9HPK/fBl6L5/Q7wGvznBD/tb4LeG/+Y4nn9dvAa/Gcfgd4bZ4T4r/OdwHvzX888bx+G3gtntPvAK/Nc0L81/gu4L35zyGe128Dr8Vz+h3gtXlOiP983wW8N/95xPP6beC1eE6/A7w2zwnxn+u7gPfmP5d4Xr8NvBbP6XeA1+Y5If7zfBfw3vznE8/rt4HX4jn9DvDaPCfEf47vAt6b/xrief028Fo8p98BXpvnhPiP913Ae/NfRzyv3wZei+f0O8Br85wQ/7G+C3hv/muJ5/XbwGvxnH4HeG2eE+I/zncB781/PfG8fht4LZ7T7wCvzXNC/Mf4LuC9+e8hntdvA6/Fc/od4LV5Toh/v+8C3pv/PuJ5/TbwWjyn3wFem+eE+Pf5LuC9+e8l/u0Q/3bfBbw3//3Evx3i3+a7gPfmfwbxb4f41/su4L35n0P82yH+db4LeG/+ZxH/dogX3XcB783/POJ5fTXwUjynvwE+mueEeNF8F/De/M8kntdvA6/Fc/od4LV5Toh/2XcB783/XOJ5/TbwWjyn3wFem+eEeOG+C3hv/mcTz+u3gdfiOf0O8No8J8QL9l3Ae/M/n3hevw28Fs/pd4DX5jkhnr/3Bt6b/x1em+f11cBL85z+GvhonhPi/zfE/2+I/98Q/78h/n/jHwHUUX5B/VF2FwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellularConnectedNoInternet4Bar;
impl IconShape for MdSignalCellularConnectedNoInternet4Bar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18h2v-8h-2v8zm0 4h2v-2h-2v2zM2 22h16V8h4V2L2 22z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellularNoSim;
impl IconShape for MdSignalCellularNoSim {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.99 5c0-1.1-.89-2-1.99-2h-7L7.66 5.34 19 16.68 18.99 5zM3.65 3.88L2.38 5.15 5 7.77V19c0 1.1.9 2 2 2h10.01c.35 0 .67-.1.96-.26l1.88 1.88 1.27-1.27L3.65 3.88z",
            }
            path {
                d: "M.01 0h24v24h-24z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/f7w38N08J8T/D98FPAR4bZ4T4v++7wLeG/gd4LV5Toj/274LeG+u+B3gtXlOiP+7vgt4b57td4DX5jkh/m/6LuC9eU6/A7w2zwnxf893Ae/N8/od4LV5Toj/W74LeG+ev98BXpvnhPi/47uA9+YF+x3gtXlOiP8bvgt4b1643wFem+eE+N/vu4D35l/2O8Br85wQ/7t9F/DevGh+B3htnhPif6/vAt6bF93vAK/Nc0L87/RdwHvz/F0CdoEH8Zx+B3htnhPif5/vAt6b5+8S8NrAVwOvxXP6HeC1eU6I/12+C3hvnr9LwGsDfw38NvBaPKffAV6b54T43+O7gPfm+bsEvDbw11zx28Br8Zx+B3htnhPif4fvAt6b5+8S8NrAX/Nsvw28Fs/pd4DX5jkh/uf7LuC9ef4uAa8N/DXP6beB1+I5/Q7w2jwnxP9s3wW8N8/fJeC1gb/mef028Fo8p98BXpvnhPif67uA9+b5uwS8NvDXPH+/DbwWz+l3gNfmOSH+Z/ou4L15/i4Brw38NS/YbwOvxXP6HeC1eU6I/3m+C3hvnr9LwGsDf80L99vAa/Gcfgd4bZ4T4n+W7wLem+fvEvDawF/zL/tt4LV4Tr8DvDbPCfE/x3cB783zdwl4beCvedH8NvBaPKffAV6b54T4n+G7gPfm+bsEvDbw17zofht4LZ7T7wCvzXNC/Pf7LuC9ef4uAa8N/DX/Or8NvBbP6XeA1+Y5If57fRfw3jx/l4DXBv6af73fBl6L5/Q7wGvznBD/fb4LeG+ev0vAawN/zb/NbwOvxXP6HeC1eU6I/x7fBbw3z98l4LWBv+bf7reB1+I5/Q7w2jwnxH+97wLem+fvEvDawF/z7/PbwGvxnH4HeG2eE+K/1ncB783zdwl4beCv+ff7beC1eE6/A7w2zwnxX+e7gPfm+bsEvDbw1/zH+G3gtXhOvwO8Ns8J8V/ju4D35vm7BLw28Nf8x/lt4LV4Tr8DvDbPCfGf77uA9+b5uwS8NvDX/Mf6beC1eE6/A7w2zwnxn+u7gPfm+bsEvDbw1/zH+23gtXhOvwO8Ns8J8Z/nu4D35vm7BLw28Nf85/ht4LV4Tr8DvDbPCfGf47uA9+b5uwS8NvDX/Of5beC1eE6/A7w2zwnxH++7gPfm+bsEvDbw1/zn+m3gtXhOvwO8Ns8J8R/ru4D35vm7BLw28Nf85/tt4LV4Tr8DvDbPCfEf57uA9+b5uwS8NvDX/Nf4beC1eE6/A7w2zwnxH+O7gPfm+bsEvDbw1/zX+W3gtXhOvwO8Ns8J8e/3XcB78/xdAl4b+Gv+a/028Fo8p98BXpvnhPj3+S7gvXn+LgGvDfw1//V+G3gtntPvAK/Nc0L8230X8N48f5eA1wb+mv8evw28Fs/pd4DX5jkh/m2+C3hvnr9LwGsDf81/n98GXovn9DvAa/OcEP963wW8N8/fJeC1gb/mv9dvA6/Fc/od4LV5Toh/ne8C3pvn7xLw2sBf89/vt4HX4jn9DvDaPCfEi+67gPfmBbsVuJX/GV4aOM5z+h3gtXlOiBfNdwHvzf9uvwO8Ns8J8S/7LuC9+d/vd4DX5jkhXrjvAt6b/xt+B3htnhPiBfsu4L35v+N3gNfmOSGev/cG3pv/W/4a+GieE+L/N8T/b4j/3xD/vyH+f+MfAXeq00GMVGjfAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellularNull;
impl IconShape for MdSignalCellularNull {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6.83V20H6.83L20 6.83M22 2L2 22h20V2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalCellularOff;
impl IconShape for MdSignalCellularOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1l-8.59 8.59L21 18.18V1zM4.77 4.5L3.5 5.77l6.36 6.36L1 21h17.73l2 2L22 21.73 4.77 4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalWifi0Bar;
impl IconShape for MdSignalWifi0Bar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,6L12,6c3.33,0,6.49,1.08,9.08,3.07L12,18.17l-9.08-9.1C5.51,7.08,8.67,6,12,6 M12,4C7.31,4,3.07,5.9,0,8.98L12,21 L24,8.98C20.93,5.9,16.69,4,12,4L12,4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEHUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3MceCngpYHjwGtzxYOBB/P83QrcyhW/DewCfw38DbDLfzzEf5wHA68FvDbw2sCD+Y91K/DbwG8DvwPcyr8f4t/npYH3Al4beGn+a/018NPA9wC38m+D+Nc7DrwX8N7AS/M/w18DXw38DLDLiw7xonsw8FnAWwPH+Z9pF/hp4HOAW/mXIf5lDwY+C3hv/nf5buBzgFt5wRAv2IOBzwLem//dvhv4GGCX54V4/j4L+GjgOP837AJfDXwOzwnxnF4a+C7gpfm/6a+B9wH+misQz/ZZwGfz/8NnA58DIK74aeCt+P/lZ4C3Fle8N/Bd/P/yPsB3i2f7auCj+P/ha4CPBhDP6beB1+L/tt8BXpsrEM/pOPDXwIP4v+kZwEsDu1yBeF4vDfw2cIz/Wy4Brw38Nc+GeP7eG/gu/m95H+C7eU6IF+yrgY/i/4avAT6a54V44X4beC3+d/sd4LV5/hAv3HHgr4EH8b/TM4CXBnZ5/hD/spcGfhs4xv8ul4DXBv6aFwzxonlv4Lv43+V9gO/mhUO86L4a+Cj+d/ga4KP5lyH+dX4beC3+Z/sd4LV50SD+dY4Dfw08iP+ZngG8NLDLiwbxr/fSwG8Dx/if5RLw2sBf86JD/Nu8N/Bd/M/yPsB386+D+Lf7auCj+J/hc4DP5l8P8e/z28Br8d/rZ4C35t8G8e9zHPht4KX47/E3wGsDu/zbIP79Xhr4beAY/7UuAa8N/DX/doj/GG8N/BT/td4G+Gn+fRD/cT4b+Cz+a3wO8Nn8+yH+Y/008Fb85/oZ4K35j4H4j3Uc+G3gpfjP8TfAawO7/MdA/Md7aeC3gWP8x7oEvDbw1/zHQfzneGvgp/iP9TbAT/MfC/Gf57OBz+I/xucAn81/PMR/rp8G3op/n58B3pr/HIj/XMeB3wZein+bvwFeG9jlPwfiP99LA78NHONf5xLw2sBf858H8V/jrYGf4l/nbYCf5j8X4r/OZwOfxYvmc4DP5j8f4r/WTwNvxQv3M8Bb818D8V/rOPDbwEvx/P0N8NrALv81EP/1Xhr4beAYz+kS8NrAX/NfB/Hf462Bn+I5vQ7w2/zXQvz3+Wzgs7jiY4Cv5r8e4r/Xd3PFe/PfA/Hf6zhX7PLfA/H/G+L/N8T/b4j/3xD/vyH+f+MfAfkegkHW2cWzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalWifi4Bar;
impl IconShape for MdSignalWifi4Bar {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.01 21.49L23.64 7c-.45-.34-4.93-4-11.64-4C5.28 3 .81 6.66.36 7l11.63 14.49.01.01.01-.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3MceCngpYHjwGtzxYOBB/P83QrcyhW/DewCfw38DbDLfzzEf5zjwFsBrw28NPDS/Mf6a+Cvgd8GfgbY5d8P8e/zYOCtgPcGXpr/Wn8NfDfwM8Ct/Nsg/vWOA28FfDTw0vzP8NfAVwM/A+zyokO86B4MvBfw0cBx/mfaBb4a+B7gVv5liH/Zg4HPAt6b/12+G/gc4FZeMMQLdhz4LOCj+d/ts4GvAXZ5Xojn76OAzwaO83/DLvDRwPfwnBDP6cHAdwGvzf9Nvw28D3ArVyCe7aOAzwaO83/bLvDZwNcAiCu+G3gv/n/5HuC9xRWvDfwW/7+8DvDb4tk+Gvgq/n/4GOCrAcRz+m7gvfi/7XuA9+YKxHM6Dvw28FL83/Q3wGsDu1yBeF4PBv4aOMb/LZeABwO7PBvi+Xtt4Lf4v+VlgL/mOSFesI8Gvor/XX4G+Guu+GtgF3hp4Fbgp3leiBfuu4H34n+2ZwCfDfw0sMu/DuKFOw78NvBS/M9zCfho4Lv5t0P8yx4M/DVwjP85ngG8NfDX/PsgXjSvDfwW/zM8A3hpYJd/P8SL7qOBr+K/1yXgtYG/5j8G4l/nu4H34r/P+wDfzX8cxL/OceC3gZfiv94zgAfzwr028F7AawMP5oqfBn4a+B6eF+Jf78HAXwPH+K/1PsB384J9FfDRvGB/DbwOsMuzIf5tXhv4Lf5rnQB2ef6+Gvgonu0S8NfAceCleLa/Bl4H2OUKxL/dRwNfxX+NnwHemufvtYHf4opLwGcDX82zvTTw08CDuOJzgM/mCsS/z3cD78V/vs8BPpvn77uB9+KKjwG+muf1YOCvgWPALnCCKxD/PseB3wZeiv9cnwN8Ns/f04EHA5eA47xgXw18FFe8DvDbAOLf78HAXwPH+M/zNsBP8/yZK34HeG1esPcGvosrXgf4bQDxH+O1gd/iP8/rAL/N82eu+B3gtXnBXhv4La54HeC3AcR/nI8Gvor/HK8D/DbPn7nid4DX5gV7beC3uOJ1gN8GEP+xvht4L/7jvQ7w2zx/5orfAV6bF+y1gd/iitcBfhtA/Mc6Dvw28FL8x3od4Ld5Xq8F/DZX/DXw0bxgLw18NVd8NPA9wK74j/dg4K+BY/zHeR3gt3lOvwW8Nv92u8DriP8crw38Fv9xXgf4bZ7to4Gv4t/vt8V/no8Gvor/GC8D/DXP9tPAW/Hvh/jP9d3Ae/Hv8z3Ae/Ocfht4Lf79EP+5jgO/DbwU/zZ/A7w0z+u3gdfi3w/xn+/BwF8Dx/jXuQQ8GNjlef028Fr8+yH+a7w28Fv867wO8Ns8f78NvBb/foj/Oh8NfBUvmo8BvpoX7LeB1+LfD/Ff67uB9+KF+x7gvXnhfht4Lf79EP+1jgO/DbwUz9/fAK8N7PLC/TbwWvz7If7rPRj4a+AYz+kS8NLArfzLfht4Lf79EP89Xhv4LZ7T6wC/zYvmt4HX4t8P8d/no4Gv4oqPAb6aF91vA6/Fvx/iv9d3c8V786/z1cBH8e/3DPHf6zhX7PKv82Dgr4Fj/Pu8j/jf68HARwMvzb/ercBPAz8t/n9D/P+G+P8N8f8b4v83/hF23sZlL7QLkwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalWifi4BarLock;
impl IconShape for MdSignalWifi4BarLock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 16v-1.5c0-1.4-1.1-2.5-2.5-2.5S18 13.1 18 14.5V16c-.5 0-1 .5-1 1v4c0 .5.5 1 1 1h5c.5 0 1-.5 1-1v-4c0-.5-.5-1-1-1zm-1 0h-3v-1.5c0-.8.7-1.5 1.5-1.5s1.5.7 1.5 1.5V16zm-6.5-1.5c0-2.8 2.2-5 5-5 .4 0 .7 0 1 .1L23.6 7c-.4-.3-4.9-4-11.6-4C5.3 3 .8 6.7.4 7L12 21.5l3.5-4.4v-2.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSignalWifiOff;
impl IconShape for MdSignalWifiOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23.64 7c-.45-.34-4.93-4-11.64-4-1.5 0-2.89.19-4.15.48L18.18 13.8 23.64 7zm-6.6 8.22L3.27 1.44 2 2.72l2.05 2.06C1.91 5.76.59 6.82.36 7l11.63 14.49.01.01.01-.01 3.9-4.86 3.32 3.32 1.27-1.27-3.46-3.46z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi+Xtp4Bj/t1wC/prnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gtntPfAB/N8/ot/nf4HeC1eU6I5++3gdfiOf0O8No8L/O/w+8Ar81zQjx/vw28Fs/pd4DX5nmZ/x1+B3htnhPi+ftt4LV4Tr8DvDbPy/zv8DvAa/OcEM/fbwOvxXP6HeC1eV7mf4ffAV6b54R4/n4beC2e018DH83z+m3+d/gd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvpYHj/N+yC/w1zwnx/xvi/zfE/2+I/98Qz99LA8f4v+US8Nc8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8Zz+Bvhontdv8b/D7wCvzXNCPH+/DbwWz+l3gNfmeZn/HX4HeG2eE+L5+23gtXhOvwO8Ns/L/O/wO8Br85wQz99vA6/Fc/od4LV5XuZ/h98BXpvnhHj+fht4LZ7T7wCvzfMy/zv8DvDaPCfE8/fbwGvxnP4a+Gie12/zv8PvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/Lw0c5/+WXeCveU6I/98Q/78h/n9D/P+GeP5eGjjG/y2XgL/mOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei+f0N8BH87x+i/8dfgd4bZ4T4vn7beC1eE6/A7w2z8v87/A7wGvznBDP328Dr8Vz+h3gtXle5n+H3wFem+eEeP5+G3gtntPvAK/N8zL/O/wO8No8J8Tz99vAa/Gcfgd4bZ6X+d/hd4DX5jkhnr/fBl6L5/TXwEfzvH6b/x1+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5e2ngOP+37AJ/zXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EY5xsQetGya4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStorage;
impl IconShape for MdStorage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 20h20v-4H2v4zm2-3h2v2H4v-2zM2 4v4h20V4H2zm4 3H4V5h2v2zm-4 7h20v-4H2v4zm2-3h2v2H4v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13Gu2OW/B+K/13dxxfvw3wPx3+e9ge/iivcBvpv/eoj/Hi8N/BZwnCt2gdcB/pr/Woj/eseB3wJemuf018DrALv810H81/su4L15/r4beB/+6yD+a7038F28cO8DfDf/NRD/dV4a+C3gOC/cLvA6wF/znw/xX+M48FvAS/Oi+WvgdYBd/nMh/mt8F/De/Ot8N/A+/OdC/Od7b+C7+Ld5H+C7+c+D+M/30sBxntNXAy/Fc/ob4KN5TrvAX/OfB/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8e9zHHgv4K15tr8Gvga4lRfst4HX4jn9DvDaPK/X4kVzCfhr/nUQ/3YvDfwU8GCev/cBvpvn77eB1+I5/Q7w2jwv86L5HeC1+ddB/NscB54OHOeFexvgp3levw28Fs/pd4DX5nmZF83vAK/Nvw7i3+a7gffiX7YLnOB5/TbwWjyn3wFem+dlXjS/A7w2/zqIf5uLwHFeNG8D/DTP6beB1+I5/Q7w2jwv86L5HeC1+ddB/Ou9NPBXvOg+B/hsntNvA6/Fc/od4LV5XuZF8zvAa/Ovg/jXezDwdF50nwN8Ns/pt4HX4jn9DvDaPC/zovkd4LX510H82+wCx3jRvA7w2zyn3wZei+f0O8Br87zMi+Z3gNfmXwfxb/PVwEfxL3sG8GCe128Dr8Vz+h3gtXle5kXzO8Br86+D+Lc5Dvw28FK8cK8D/DbP67eB1+I5/Q7w2jwv86L5HeC1+ddB/Ns9GPhp4KV4XpeA9wZ+mufvt4HX4jn9DvDaPC/zovkd4LX510H8+5nn9TrAb/OC/TbwWjyn3wFem+f12lzx0sBX8bxehyt2gb/mXwfx72ee1+sAv80L9tvAa/Gcfgd4bV6w1wZ+i+cl/u0Q/37meb0O8Nu8YL8NvBbP6XeA1+YFe23gt3he4t8O8e9nntfrAL/NC/bbwGvxnH4HeG1esNcGfovnJf7tEP9+5nm9DvDbvGC/DbwWz+l3gNfmBXtt4Ld4XuLfDvHvZ57XRwN/zQv21cBL85z+GvhoXrCXBr6a5yX+7RD/frvAMf57XAKO82+H+Pf7buC9+O/xPcB782+H+Pd7aeCv+O/xMsBf82+H+I/x3sB38V/rfYDv5t8H8R/npYGPBt4aOMZ/jkvATwNfDfw1/36I/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP96rw38FHCc57QLvA3w2/zXQfzXem/gu3jh3gf4bv5rIP7rPBh4Ov+yXeBlgFv5z4f4r/PVwEfxovkc4LP5z4f4r/PbwGvxovkd4LX5z4f4r/PbwGvxovkd4LX5z4f4r/PbwGvxovkd4LX5z4f4r/PRwFfxovkY4Kv5z4f4r3Mc+GvgQbxwzwAezH8NxH+tlwZ+GzjG83cJeG3gr/mvgfivdxz4auCtgWNccQn4aeCjgV3+6yD+e700V/w1/z0Q/78h/n9D/P+G+P+NfwSMIcBB4UX3kAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUsb;
impl IconShape for MdUsb {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 7v4h1v2h-3V5h2l-3-4-3 4h2v8H8v-2.07c.7-.37 1.2-1.08 1.2-1.93 0-1.21-.99-2.2-2.2-2.2-1.21 0-2.2.99-2.2 2.2 0 .85.5 1.56 1.2 1.93V13c0 1.11.89 2 2 2h3v3.05c-.71.37-1.2 1.1-1.2 1.95 0 1.22.99 2.2 2.2 2.2 1.21 0 2.2-.98 2.2-2.2 0-.85-.49-1.58-1.2-1.95V15h3c1.11 0 2-.89 2-2v-2h1V7h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHXUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zE+BvhrntNLA1/Ff4xd4K+BnwH+mhcM8cJ9FfDR/Md7HeC3eU6vDfwW//G+GvgYnj/EC/bVwEfxn+N1gN/mOb028Fv85/ga4KN5Xojn76WBv+I/z+sAv81zem3gt/jP8xDgVp4T4vn7bOCz+M/zOsBv85xeG/gt/vN8DvDZPCfE8/fTwFvxnC4Bf81/jI8G/prn9NLAV/Mf46WBYzynnwHemueEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2//VeGjgGPAO4lRfNbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr813lp4LuAl+bZvhr4HGCXF+63gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf5rHAf+Cngwz+trgI/mhftt4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8aHw18FS/YCWCXF+y3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf5rfDfwXrxgrwP8Ni/YbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr81/hs4LN4wV4G+GtesN8GXovn9DvAa/OcEM/fbwOvxXP6HeC1+a/x0sBf8fz9DfDSvHC/DbwWz+l3gNfmOSGev98GXovn9DvAa/Nf572B7+I5PQN4a+CveeF+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+a/1oOB9waOA7cC3w3s8i/7beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99LA8d5TrvAX/O/w0sDx3lOu8Bf85wQ/78h/n9D/P+G+P8N8fy9NHCM53QJ+Gv+7Y4Du/zXeGngGM/pEvDXPCfE8/fbwGvxnH4HeG3+bY4DTwc+Bvhu/vP9NvBaPKffAV6b54R4/n4beC2e0+8Ar82/zV8BLw3sAq8D/DX/uX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5l/vu4D35tluBV4G2OVf9tJc8df86/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzb/OewPfxfP6beB1eOGOA7/FFa8D7PKi+23gtXhOvwO8Ns8J8fz9NvBaPKffAV6bF91LA3/FC/Y1wEfzgn0X8N5c8d3A+/Ci+23gtXhOvwO8Ns8J8fz9NvBaPKffAV6bF81x4OnAcV649wG+m+f13sB38ZzeB/huXjS/DbwWz+l3gNfmOSGev98GXovn9DvAa/Oi+SvgpfmX7QKvA/w1z/bSwG8Bx3lOu8DrAH/Nv+y3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf5l3wW8Ny+6W4GXAXaB48BvAS/N8/fXwOsAu7xwvw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzwr038F386/028DrAdwHvzQv33cD78ML9NvBaPKffAV6b54R4/n4beC2e0+8Ar80L9tLAX/Fv99vAa/OieR/gu3nBfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm+TsOPB04zn+NXeB1gL/m+ftt4LV4Tr8DvDbPCfH8/TbwWjyn3wFem+fvr4CX5r/WXwOvA+zyvH4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5nl9F/De/Pf4buB9eF6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn76eBt+I57QJ/zX+MjwH+muf00sBX8R/jpYHjPKefAd6a54R4/j4b+Cz+87wO8Ns8p9cGfov/PJ8DfDbPCfH8vTTwV/zneR3gt3lOrw38Fv95HgLcynNCvGBfDXwU/zleB/htntNrA7/Ff46vAT6a54V44b4a+Cj+470O8Ns8p9cGfov/eF8DfDTPH+Jf9tLAWwMvDRznP8ZHA3/Nc3pp4Kv5j7EL/DXw3cCtvGCI/98Q/78h/n9D/P+G+P+NfwTLxxZQSLQrigAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWallpaper;
impl IconShape for MdWallpaper {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4h7V2H4c-1.1 0-2 .9-2 2v7h2V4zm6 9l-4 5h12l-3-4-2.03 2.71L10 13zm7-4.5c0-.83-.67-1.5-1.5-1.5S14 7.67 14 8.5s.67 1.5 1.5 1.5S17 9.33 17 8.5zM20 2h-7v2h7v7h2V4c0-1.1-.9-2-2-2zm0 18h-7v2h7c1.1 0 2-.9 2-2v-7h-2v7zM4 13H2v7c0 1.1.9 2 2 2h7v-2H4v-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/fV4b+G3+YyD+d/ku4L2B9wG+m38/xP8e3wW8N8/2PsB38++D+N/hu4D35nm9D/Dd/Nsh/mXmP97rAL/Ni+a7gPfmBXsf4Lv5t0H8y8x/vNcBfpt/2XcB782/7H2A7+ZfD/EvM//xXgf4bV647wLemxfd+wDfzb8O4l9m/uO9DvDbvGDfBbw3/3rvA3w3LzrEv8z8x3sd4Ld5/r4LeG/+7d4H+G5eNIh/mfmP9zrAb/O8vgt4b/793gf4bv5liH+Z+Y/3OsBv85y+C3hv/uO8D/DdvHCIf5n5j/c6wG/zbN8FvDf/8d4H+G5eMMS/zPzHex3gt7niu4D35j/P+wDfzfOH+JeZ/3ivA/w28F3Ae/Of732A7+Z5If5l5j/e6wDvBbw3/3XeB/hunhPiX2b+430P8F7813sd4Ld5NsS/zPzHex3gvYH34r/O9wDvzXNC/MvMf7zXAX4b+G7gvfjP9z3Ae/O8EP8y8x/vdYDf5orvBt6L/zzfA7w3zx/iX2b+470O8Ns823cD78V/vO8B3psXDPEvM//xXgf4bZ7TdwPvxX+c7wHemxcO8S8z//FeB/htntd3A+/Fv9/3AO/NvwzxLzP/8V4H+G2ev+8G3ot/u+8B3psXDeJfZv7jvQ7w27xg3w28F/963wO8Ny86xL/M/Md7HeC3eeG+G3gvXnTfA7w3/zqIf9lr8x/vr4Fd/mXfDbwX/7LvAd6bfz3E/3zfDbwXL9j3AO/Nvw3if4fvBt6L5/U9wHvzb4f43+O7gffi2b4HeG/+fRD/u3w38F7A9wDvzb8f4l/2WvzH+xtgl+d0HHgp/mXvDXw3L7rf4QVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Md7HeC3eU6vDfwW//HEC4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AvgUp0GyRLvgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWidgets;
impl IconShape for MdWidgets {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 13v8h8v-8h-8zM3 21h8v-8H3v8zM3 3v8h8V3H3zm13.66-1.31L11 7.34 16.66 13l5.66-5.66-5.66-5.65z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+Y4DLwUcB16a5/TaXPHbPKe/BnaBvwF2+c+D+I/10sBLAS8NvDbw0vzH+Gvgt4G/Bv4G+Gv+YyD+fY4DrwW8NfDWwHH+a+wCPw38NPA7wC7/Noh/m/cC3hp4a/5n+Gngp4Hv4V8H8aJ7MPBRwHsDx/mfaRf4buBrgFv5lyH+ZQ8GPgt4b/53+W7gc4BbecEQL9hLA18FvDb/u/028DHAX/O8EM/rOPBZwEfzf8tXA58D7PJsiOf0UcBnA8f5v2kX+Gzga7gCccWDge8CXpv/H34beBtgV8BbA98FHOf/l13gfQR8N/Be/P/0PQKOA78NvBT/v/wN8NriipcGfhs4xv8Pl4DXBv5aPNt7A9/F/w/vA3w3gHhO3w28F/+3fQ/w3lyBeE7Hgd8GXor/m/4GeG1glysQz+ulgd8GjvF/yyXgtYG/5tkQz997A9/F/y3vA3w3zwnxgn038F783/A9wHvzvBAv2HHgt4GX4n+P3wF+myv+GjgOPBj4amCX54V44V4a+G3gGP9zPQP4bOCngV3+dRD/svcGvov/mT4H+Gz+7RAvmu8G3ov/OS4Brw38Nf8+iBfNceC3gZfiv98l4KWBW/n3Q7zoXhr4beAY/71eBvhr/mMg/nXeG/gu/vt8DvDZ/MdB/Ot9N/Be/Nd7BvBgXrgHA58FvDbwYK74aeCnge/heSH+9Y4Dvw28FP+13gf4bl6wzwI+mxfst4G3AXZ5NsS/zUsDvw0c47/OCWCX5++zgc/i2Z4B3AocB16KZ/tr4HWAXa5A/Nu9N/Bd/Nf4HeC1ef4eDDydKy4Bnw18Nc/20sB3Ay/FFZ8DfDZXIP59vht4L/7zfQ7w2Tx/3w28F1d8DPDVPK8HA38NHAN2gRNcgfj3OQ78NvBS/Of6HOCzef6eDjwYeAbwYF6wrwY+iiteB/htAPHv99LAbwPH+M/zNsBP8/yZK34HeG1esPcGvosrXgf4bQDxH+O9ge/iP8/rAL/N82eu+B3gtXnBXhv4La54HeC3AcR/nO8G3ov/HK8D/DbPn7nid4DX5gV7beC3uOJ1gN8GEP9xjgO/DbwU//FeB/htnj9zxe8Ar80L9trAb3HF6wC/DSD+Y7008NvAMf5jvQ7w2zyv1wJ+myv+GvhoXrCXBr6aKz4a+B5gV/zHe2/gu/iP9TrAb/Nsx4HfAl6af7td4HXEf47vBt6L/zivA/w2z/bRwFfx7/fX4j/HceC3gZfiP8brAL/Ns/028Fr8+yH+87w08NvAMf59LgEvDdzKs/028Fr8+yH+c7038F38+7wP8N08p98GXot/P8R/vu8G3ot/m+8B3pvn9dvAa/Hvh/jPdxz4beCl+Nf5G+C1gV2e128Dr8W/H+K/xksDvw0c40VzCXht4K95/n4beC3+/RD/dd4b+C5eNO8DfDcv2G8Dr8W/H+K/1ncD78UL9z3Ae/PC/TbwWvz7If5rHQd+G3gpnr+/AV4b2OWF+23gtfj3Q/zXe2ngt4FjPKdLwGsDf82/7LeB1+LfD/Hf472B7+I5vQ/w3bxofht4Lf79EP99vht4L674HuC9edF9NfBR/Ps9Q/z3OQ78Nle8NrDLi+6lgb/i3+9zxH+vl+aKv+Zf76WB9wZemn+9W4GfBn5a/P+G+P8N8f8b4v83xP9v/COPls2VfB5kawAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifiLock;
impl IconShape for MdWifiLock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.5 9.5c.28 0 .55.04.81.08L24 6c-3.34-2.51-7.5-4-12-4S3.34 3.49 0 6l12 16 3.5-4.67V14.5c0-2.76 2.24-5 5-5zM23 16v-1.5c0-1.38-1.12-2.5-2.5-2.5S18 13.12 18 14.5V16c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 0h-3v-1.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5V16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifiTethering;
impl IconShape for MdWifiTethering {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 11c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 2c0-3.31-2.69-6-6-6s-6 2.69-6 6c0 2.22 1.21 4.15 3 5.19l1-1.74c-1.19-.7-2-1.97-2-3.45 0-2.21 1.79-4 4-4s4 1.79 4 4c0 1.48-.81 2.75-2 3.45l1 1.74c1.79-1.04 3-2.97 3-5.19zM12 3C6.48 3 2 7.48 2 13c0 3.7 2.01 6.92 4.99 8.65l1-1.73C5.61 18.53 4 15.96 4 13c0-4.42 3.58-8 8-8s8 3.58 8 8c0 2.96-1.61 5.53-4 6.92l1 1.73c2.99-1.73 5-4.95 5-8.65 0-5.52-4.48-10-10-10z",
            }
        }
    }
}
