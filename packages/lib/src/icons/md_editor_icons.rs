use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddChart;
impl IconShape for MdAddChart {
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
                d: "M6 9.99h2v7H6zm8 3h2v4h-2zm-4-6h2v10h-2zM20 7V4h-2v3h-3v2h3v3h2V9h3V7zm-2 12H4V5h12V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-5h-2v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5XuA9+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/zwfA/w1z+mlga/iP8/vAK/Nc0I8f78NvBb/eV4H+G2e02sDv8V/nt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/Ff57XAX6b5/TawG/xn+d3gNfmOSGev98GXov/PK8D/DbP6bWB3+I/z+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr86/wO8Nr81/ht4LX41/kd4LV5Tojn77eB1+Jf53eA1+a/xm8Dr8W/zu8Ar81zQjx/vw28Fv86vwO8Nv81fht4Lf51fgd4bZ4T4vn7beC1+Nf5HeC1+a/x28Br8a/zO8Br85wQz99vA6/Fv87vAK/Nf43fBl6Lf53fAV6b54R4/n4beC3+dXaBv+ZF8zHAX/OcXhr4Kl40Lw0c51/nd4DX5jkhnr/fBl6L/zyvA/w2z+m1gd/iP8/vAK/Nc0I8f78NvBb/eV4H+G2e02sDv8V/nt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/Ff57XAX6b5/TawG/xn+d3gNfmOSGev98GXov/PK8D/DbP6bWB3+I/z+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77OBz+L/lt8BXpvnhHj+3hr4Kf5v+R3gtXlOiBfsr4GX4v+O3wFem+eEeMEeDPw08FL83/A7wGvznBAv3HHgo4H3Bh7E/26/A7w2zwnxv997A9/Fv+x3gNfmOSH+b3hv4Lt44X4HeG2eE+L/jvcGvosX7HeA1+Y5If5veW/gu3j+fgd4bZ4T4v+e9wa+i+f1O8Br85wQ/ze9N/BdPKffAV6b54T4v+u9ge/i2X4HeG2eE+L/tvcGvosrfgd4bZ4T4v++9wa+C/gd4LV5Toj/H94beG/gtXlOiP8/Xhr4a54T4v83xP9viP/fEP+/If5/4x8BjpSkQXvjATwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddComment;
impl IconShape for MdAddComment {
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
                d: "M21.99 4c0-1.1-.89-2-1.99-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18zM17 11h-4v4h-2v-4H7V9h4V5h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAttachFile;
impl IconShape for MdAttachFile {
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
                d: "M16.5 6v11.5c0 2.21-1.79 4-4 4s-4-1.79-4-4V5c0-1.38 1.12-2.5 2.5-2.5s2.5 1.12 2.5 2.5v10.5c0 .55-.45 1-1 1s-1-.45-1-1V6H10v9.5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5V5c0-2.21-1.79-4-4-4S7 2.79 7 5v12.5c0 3.04 2.46 5.5 5.5 5.5s5.5-2.46 5.5-5.5V6h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/PawG/xgv0O8Nr810L813lt4Ld4wX4HeG3+ayH+67w28Fu8YL8DvDb/tRD/dV4b+C1esN8BXpv/Woj/Oq8N/BYv2O8Ar81/LcR/nfcGvosX7HeA1+a/FuI/10sDHwW8NXCcf9mtwE8DXwPcyn8+xH+OBwNfBbw1/3afDXwO/7kQ//HeGvgu4Dj/fr8NvA2wy38OxH+s9wa+i/9Yfw28DP85EP9xHgz8FXCcF+xvgN8GdnlObw28FC/Y5wCfzX88xH+c3wJem+fvd4D3Bm7lBXsw8NPAS/H8nQB2+Y+F+I/xYODpPH/fA7w3L5rjwG8DL8Xz+hzgs/mPhfiP8dHAV/G8ngE8mH+d1wZ+i+f118DL8B8L8R/js4HP4nl9DfDR/OvdCjyI5yX+YyH+Y/w08FY8r88BPpt/vd8GXovn9TrAb/MfB/Ef47OBz+J5fQ/w3vzPhfiP8dHAV/H8vQzw1/zPhPiP8WDg6Tx/u8DbAL/N/zyI/zi/DbwWL9h3A98D/Db/cyD+4zwY+GvgGC/crcB3Az8D/DX/vRD/sd4b+GrgGC+aXeCngd8GfgbY5b8W4j/eSwO/DRzjX++nga8Bfpv/Goj/HMeBrwbei3+bvwY+Bvht/nMh/nM9GPhs4L34t/lu4GOAXf5zIP5rHAfeGnhv4LX41/lr4HWAXf7jIf7rHQdeG3ht4K2BB/Ev+2vgZfiPh/jv99LAewPvDRzjBfsa4KP5j4X4n+M48NXAe/GCPQS4lf84iP95Phr4Kp6/jwG+mv84iP+Zfht4LZ7X7wCvzX8cxP9Mbw38FM/rr4GX4T8O4t/vOHCR5/U7wGvzb/PawG/x/In/OIj/GOb5OwHs8q/32sBv8fyJ/ziI/xg/DbwVz+tzgM/mX++zgc/ief0O8Nr8x0H8x3hv4Lt4/t4H+G5edC8N/BZwnOf1NcBH8x8H8R/nVuBBPH+fDXwPcCsv2HHgvYDPBo7z/L0M8Nf8x0H8x3lt4Ld44X4b+Gtgl+f02sBLA8d5wX4GeGv+YyH+Y7038F38x/sb4LWBXf5jIf7jvTfw1cAx/mP8DfDawC7/8RD/OR4MfDXwVvzbXQK+Gvhs/vMg/nM9GPhs4LWBB/Gi+Rvgu4HvBnb5z4X4r/PewHfxgv0N8NrALv91EP91Xhv4LV6w3wFem/9aiP86rw38Fi/Y7wCvzX8txH+d1wZ+ixfsd4DX5r8W4r/OawO/xQv2O8Br818L8V/ntYHf4gX7HeC1+a+F+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CAI2NQaLnm2QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAttachMoney;
impl IconShape for MdAttachMoney {
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
                d: "M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFG0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//W+Gngpnr+/AT6a/zqI/3q/DbwWz9/vAK/Nfx3Ef73fBl6L5+93gNfmvw7iv95vA6/F8/c7wGvzXwfxX++3gdfi+fsd4LX5r4P4r/fbwGvx/P0O8Nr810H81/tt4LV4/n4HeG3+6yD+6/028Fo8f78DvDb/dRD/9X4beC2ev98BXpv/Ooj/er8NvBbP3+8Ar81/HcR/vd8GXovn73eA1+a/DuLf5r2BB/GCfQ9wK8/fbwOvxfP3O8Br818H8W/z28Br8YK9DvDbPH+/DbwWz9/vAK/Nfx3Ev81vA6/FC/Y6wG/z/P028Fo8f78DvDb/dRD/Nr8NvBYv2OsAv83z99vAa/H8/Q7w2vzXQfzb/DbwWrxgrwP8Ns/fbwOvxfP3O8Br818H8W/z28Br8YK9DvDbPH+/DbwWz9/vAK/Nfx3Ev81vA6/FC/Y6wG/z/P028Fo8f78DvDb/dRD/Nr8NvBYv2OsAv83z99vAa/H8/Q7w2vzXQfzb/DbwWrxgrwP8Ns/fbwOvxfP3O8Br818H8W/z28Br8YK9DvDbPH+/DbwWz9/vAK/Nfx3Ev81vA6/FC/Y6wG/z/P028Fo8f78DvDYv2EsDx3j+LgF/zb8O4t/mt4HX4gV7HeC3ef5+G3gtnr/fAV6bF+y3gdfi+fsd4LX510H82/w28Fq8YK8D/DbP328Dr8Xz9zvAa/OC/TbwWjx/vwO8Nv86iH+b3wZeixfsdYDf5vn7beC1eP5+B3htXrDfBl6L5+93gNfmXwfxb/PbwGvxgr0O8Ns8f78NvBbP3+8Ar80L9tvAa/H8/Q7w2vzrIP5tfht4LV6w1wF+m+fvt4HX4vn7HeC1ecF+G3gtnr/fAV6bfx3Ev81vA6/FC/Y6wG/z/P028Fo8f78DvDYv2G8Dr8Xz9zvAa/Ovg/i3+W3gtXjBXgf4bZ6/3wZei+fvd4DX5gX7beC1eP5+B3ht/nUQ/za/DbwWL9jrAL/N8/fbwGvx/P0O8Nq8YL8NvBbP3+8Ar82/DuLf5reB1+IFex3gt3n+fht4LZ6/3wFemxfst4HX4vn7HeC1+ddB/Nv8NvBavGCvA/w2z99vA6/F8/c7wGvzgv028Fo8f78DvDb/Ooh/m98GXosX7HWA3+b5+23gtXj+fgd4bV6w3wZei+fvd4DX5l8H8W/z28Br8YK9DvDbPH+/DbwWz9/vAK/NC/bbwGvx/P0O8Nr86yD+bX4beC1esNcBfpvn77eB1+L5+x3gtXnBfht4LZ6/3wFem38dxL/NbwOvxQv2OsBv8/z9NvBaPH+/A7w2L9hvA6/F8/c7wGvzr4P4t/lt4LV4wV4H+G2ev98GXovn73eA1+YF+23gtXj+fgd4bf51EP82Xw28NC/YRwN/zfP31cBL8/z9NfDRvGBfDbw0z99fAx/Nvw7i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BkHGkQaVcTogAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBarChart;
impl IconShape for MdBarChart {
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
                d: "M5 9.2h3V19H5zM10.6 5h2.8v14h-2.8zm5.6 8H19v6h-2.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ5/UxwF/zb/fVwEvxnP4G+Gj+7V4a+Cqel3jBEP8y87xeB/ht/u1+G3gtntPvAK/Nv91rA7/F8xIvGOJfZp7X6wC/zb/dbwOvxXP6HeC1+bd7beC3eF7iBUP8y8zzeh3gt/m3+23gtXhOvwO8Nv92rw38Fs9LvGCIf5l5Xq8D/Db/dr8NvBbP6XeA1+bf7rWB3+J5iRcM8S8zz+t1gN/m3+63gdfiOf0O8Nr827028Fs8L/GCIf5l5nm9DvDb/Nv9NvBaPKffAV6bf7vXBn6L5yVeMMS/zDyv1wF+m3+73wZei+f0O8Br82/32sBv8bzEC4b4l5nn9TrAb/Nv99vAa/Gcfgd4bf7tXhv4LZ6XeMEQ/zLzvF4H+G3+7X4beC2e0+8Ar82/3WsDv8XzEi8Y4l9mntfrAL/Nv91vA6/Fc/od4LX5t3tt4Ld4XuIFQ/zLzPN6HeC3+bf7beC1eE6/A7w2/3avDfwWz0u8YIh/mXlerwP8Nv92vw28Fs/pd4DX5t/utYHf4nmJFwzxLzPP63WA3+bf7reB1+I5/Q7w2vzbvTbwWzwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef118Au/3YvDRznOe0Cf82/3XHgpXle4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8zz+htgl3+7lwaO8ZwuAX/Nv91x4KV4XuIFQ/zLzPN6HeC3+bf7beC1eE6/A7w2/3avDfwWz0u8YIh/mXlerwP8Nv92vw28Fs/pd4DX5t/utYHf4nmJFwzxLzPP63WA3+bf7reB1+I5/Q7w2vzbvTbwWzwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/MvM83od4Lf5t/tt4LV4Tr8DvDb/dq8N/BbPS7xgiH+ZeV6vA/w2/3a/DbwWz+l3gNfm3+61gd/ieYkXDPEvM8/rdYDf5t/ut4HX4jn9DvDa/Nu9NvBbPC/xgiH+ZeZ5vQ7w2/zb/TbwWjyn3wFem3+71wZ+i+clXjDEv8w8r9cBfpt/u98GXovn9DvAa/Nv99rAb/G8xAuG+JeZ5/U6wG/zb/fbwGvxnH4HeG3+7V4b+C2el3jBEP8y87xeB/ht/u1+G3gtntPvAK/Nv91rA7/F8xIvGOJfZp7XRwN/zb/dVwMvzXP6a+Cj+bd7aeCreV7iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BLT+4QUEyvmYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderAll;
impl IconShape for MdBorderAll {
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
                d: "M3 3v18h18V3H3zm8 16H5v-6h6v6zm0-8H5V5h6v6zm8 8h-6v-6h6v6zm0-8h-6V5h6v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxL3ttntdfA7s8p+PAS/O8fpvn9dLAcZ7TLvDXPK/X5nn9NbDLczoOvDTP67d5wRD/vyH+f0P8/4b4/w3xL3stntffALv82700cIzndAn4a/7tjgMvxfP6HV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/MvM83od4Lf5t/tt4LV4Tr8DvDb/dq8N/BbPS7xgiP/fEP+/If5/Q/z/hvj/DfEv+y2e18cAf81zemngq3her8Pz+mrgpXhOfwN8NM/rt3heHwP8Nc/ppYGv4nm9Di8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mW/zfP6aOCveU4vDXw1z+u1eV5fDbw0z+mvgY/mef02z+ujgb/mOb008NU8r9fmBUP8/4b4/w3x/xvi/zfE/2+If5l5Xq8D/Db/dr8NvBbP6XeA1+bf7rWB3+J5iRcM8S8zz+t1gN/m3+63gdfiOf0O8Nr827028Fs8L/GCIf5l5nm9DvDb/Nv9NvBaPKffAV6bf7vXBn6L5yVeMMS/zDyv1wF+m3+73wZei+f0O8Br82/32sBv8bzEC4b4l5nn9TrAb/Nv99vAa/Gcfgd4bf7tXhv4LZ6XeMEQ/7LX5nn9NbDLv91LA8d5TrvAX/Nvdxx4aZ7Xb/OCIf5/Q/z/hvj/DfH/G+Jf9lr87/Y7vGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjce+iQcf4rYEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderBottom;
impl IconShape for MdBorderBottom {
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
                d: "M9 11H7v2h2v-2zm4 4h-2v2h2v-2zM9 3H7v2h2V3zm4 8h-2v2h2v-2zM5 3H3v2h2V3zm8 4h-2v2h2V7zm4 4h-2v2h2v-2zm-4-8h-2v2h2V3zm4 0h-2v2h2V3zm2 10h2v-2h-2v2zm0 4h2v-2h-2v2zM5 7H3v2h2V7zm14-4v2h2V3h-2zm0 6h2V7h-2v2zM5 11H3v2h2v-2zM3 21h18v-2H3v2zm2-6H3v2h2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderClear;
impl IconShape for MdBorderClear {
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
                d: "M7 5h2V3H7v2zm0 8h2v-2H7v2zm0 8h2v-2H7v2zm4-4h2v-2h-2v2zm0 4h2v-2h-2v2zm-8 0h2v-2H3v2zm0-4h2v-2H3v2zm0-4h2v-2H3v2zm0-4h2V7H3v2zm0-4h2V3H3v2zm8 8h2v-2h-2v2zm8 4h2v-2h-2v2zm0-4h2v-2h-2v2zm0 8h2v-2h-2v2zm0-12h2V7h-2v2zm-8 0h2V7h-2v2zm8-6v2h2V3h-2zm-8 2h2V3h-2v2zm4 16h2v-2h-2v2zm0-8h2v-2h-2v2zm0-8h2V3h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/u3wU8NLAbwPfw78f4n+H48BvAS/Ns3038D78+yD+5zsO/Bbw0jyv7wbeh387xP9sx4HfAl6aF+y7gffh3wbxP9dx4LeAl+Zf9t3A+/Cvh/if6TjwW8BL86L7buB9+NdB/M90HPht4KX41/lu4H140SH+5zoO/DbwUvzrfDfwPrxoEP+zHQd+G3gp/nVeBvhr/mWI/xmOA78FfA3w3Tyn48BvAy/Fi+51gN/mX4b473cc+C3gpbnifYDv5jkdB34beCn+Zc8AXhrY5V+G+O91HPgt4KV5Tu8DfDfP6Tjw28BL8YJdAl4b+GteNIj/PseB3wJemufvfYDv5jkdB34beCme1yXgtYG/5kWH+O9xHPgt4KV54d4H+G6e03Hgt4GX4tkuAa8N/DX/Ooj/eseB3wJemhfN+wDfzXM6Dvw28FLAJeC1gb/mXw/xX+s48FvAS/Ov8z7Ad/OcjgM/DXw08Nf82yD+6xwHfgt4af5t3gf4bv5jIf5rHAd+C3hp/n3eB/hu/uMg/vMdB34LeGn+Y7wP8N38x0D85zoO/Bbw0vzHuAS8NvDX/MdA/Oc5DvwW8NL8x7gEvDbw1/zHQfznOA78FvDS/Me4BLw28Nf8x0L8xzsO/Bbw0vzHuAS8NvDX/MdD/Mc6DvwW8NL8x7gEvDbw1/znQPzHOQ78FvDS/Me4BLw28Nf850H8xzgO/Bbw0vzHuAS8NvDX/OdC/PsdB34LeGn+Y1wCXhv4a/7zIf59jgO/Bbw0/zEuAa8N/DX/NRD/dseB3wJemv8Yl4DXBv6a/zqIf5vjwG8BL81/jEvAawN/zX8txL/eceC3gJfmP8Yl4LWBv+a/HuJf5zjwW8BL8x/jEvDawF/z3wPxojsO/Bbw0vzHuAS8NvDX/PdBvOheG/gt/mNcAl4b+Gv+eyFedK8N/Bb/fpeA1wb+mv9+iBfdawO/xb/PJeC1gb/mfwbEi+61gd/i3+4S8NrAX/M/B+JF99rAb/Fvcwl4beCv+Z8F8aJ7beC3+Ne7BLw28Nf8z4N40b028Fv861wCXhv4a/5nQrzoXhv4LV50l4DXBv6a/7kQL7rXBn6LF80l4LWBv+Z/NsSL7rWB3+Jfdgl4beCv+Z8P8f8b4v83xP9viP/fEP+/IZ6/rwZeiv9b/gb4aJ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE/8IqHiXQV1vF/oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderColor;
impl IconShape for MdBorderColor {
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
                d: "M22,24H2v-4h20V24z M13.06,5.19l3.75,3.75L7.75,18H4v-3.75L13.06,5.19z M17.88,7.87l-3.75-3.75 l1.83-1.83c0.39-0.39,1.02-0.39,1.41,0l2.34,2.34c0.39,0.39,0.39,1.02,0,1.41L17.88,7.87z",
                enable_background: "new",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxL3ttntdfA7s8p+PAS/O8fpvn9dLAcZ7TLvDXPK/X5nn9NbDLczoOvDTP67d5wRD/vyH+f0P8/4b4/w3xL3stntffALv82700cIzndAn4a/7tjgMvxfP6HV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/MvM83od4Lf5t/tt4LV4Tr8DvDb/dq8N/BbPS7xgiP/fEP+/If5/Q/z/hvj/DfEv+y3+d3sdXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/7Lf53+21ecEQ/78h/n9D/P+G+P8N8f8b4l9mntfrAL/Nv91vA6/Fc/od4LX5t3tt4Ld4XuIFQ/zLzPN6HeC3+bf7beC1eE6/A7w2/3avDfwWz0u8YIh/mXlerwP8Nv92vw28Fs/pd4DX5t/utYHf4nmJFwzxLzPP63WA3+bf7reB1+I5/Q7w2vzbvTbwWzwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/stXlefw3s8m/30sBxntMu8Nf82x0HXprn9du8YIj/3xD/vyH+f0P8/4b4l70Wz+tvgF2e03HgpXhev8PzemngGM/pEvDXPK/X4nn9DbDLczoOvBTP63d4wRD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xgiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj+HlokE5lA9oAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderHorizontal;
impl IconShape for MdBorderHorizontal {
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
                d: "M3 21h2v-2H3v2zM5 7H3v2h2V7zM3 17h2v-2H3v2zm4 4h2v-2H7v2zM5 3H3v2h2V3zm4 0H7v2h2V3zm8 0h-2v2h2V3zm-4 4h-2v2h2V7zm0-4h-2v2h2V3zm6 14h2v-2h-2v2zm-8 4h2v-2h-2v2zm-8-8h18v-2H3v2zM19 3v2h2V3h-2zm0 6h2V7h-2v2zm-8 8h2v-2h-2v2zm4 4h2v-2h-2v2zm4 0h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderInner;
impl IconShape for MdBorderInner {
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
                d: "M3 21h2v-2H3v2zm4 0h2v-2H7v2zM5 7H3v2h2V7zM3 17h2v-2H3v2zM9 3H7v2h2V3zM5 3H3v2h2V3zm12 0h-2v2h2V3zm2 6h2V7h-2v2zm0-6v2h2V3h-2zm-4 18h2v-2h-2v2zM13 3h-2v8H3v2h8v8h2v-8h8v-2h-8V3zm6 18h2v-2h-2v2zm0-4h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGS0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5HQdemuf12zyvlwaO85x2gb/meb02z+uvgV2e03HgpXlev80LhviXmef1OsBv878f4l9mntfrAL/N/36If5l5Xq8D/Db/+yH+ZeZ5vQ7w2/zvh/iXmef1OsBv82/30sAxntMl4K/5tzsOvBTP63d4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX2ae1+sAv82/3W8Dr8Vz+h3gtfm3e23gt3he4gVD/MvM83od4Lf5t/tt4LV4Tr8DvDb/dq8N/BbPS7xgiH+ZeV6vA/w2/3a/DbwWz+l3gNfm3+61gd/ieYkXDPEvM8/rdYDf5t/ut4HX4jn9DvDa/Nu9NvBbPC/xgiH+ZeZ5vQ7w2/zvh/iXmef1OsBv878f4l9mntfrAL/N/36If5l5Xq8D/Db/+yH+ZeZ5vQ7w2/zvh/iXmef1OsBv85xeGvgqntfr8Ly+GngpntPfAB/N8/otntfHAH/Nc3pp4Kt4Xq/DC4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ppYGv5nm9Ns/rq4GX5jn9NfDRPK/f5nl9NPDXPKeXBr6a5/XavGCIf5l5Xq8D/Db/+yH+ZeZ5vQ7w2/zvh/iXmef1OsBv878f4l9mntfrAL/N/36If5l5Xq8D/Db/+yH+ZeZ5vQ7w2/zb/TbwWjyn3wFem3+71wZ+i+clXjDEv8w8r9cBfpt/u98GXovn9DvAa/Nv99rAb/G8xAuG+JeZ5/U6wG/zb/fbwGvxnH4HeG3+7V4b+C2el3jBEP8y87xeB/ht/u1+G3gtntPvAK/Nv91rA7/F8xIvGOJfZp7X6wC/zb/dbwOvxXP6HeC1+bd7beC3eF7iBUP8y8zzeh3gt/m3e2ngOM9pF/hr/u2OAy/N8/ptXjDEv8w8r9cBfpv//RD/MvO8Xgf4bf73Q/zLzPN6HeC3+d8P8S8zz+t1gN/mfz/Ev8w8r9cBfpvndBx4KZ7X7/C8Xho4xnO6BPw1z+u1eF5/A+zynI4DL8Xz+h1eMMS/zDyv1wF+m+f02sBv8bzE8/pt4LV4Tr8DvDbPyzyv1wF+m+f02sBv8bzEC4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I68c8EFcMD9dAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderLeft;
impl IconShape for MdBorderLeft {
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
                d: "M11 21h2v-2h-2v2zm0-4h2v-2h-2v2zm0-12h2V3h-2v2zm0 4h2V7h-2v2zm0 4h2v-2h-2v2zm-4 8h2v-2H7v2zM7 5h2V3H7v2zm0 8h2v-2H7v2zm-4 8h2V3H3v18zM19 9h2V7h-2v2zm-4 12h2v-2h-2v2zm4-4h2v-2h-2v2zm0-14v2h2V3h-2zm0 10h2v-2h-2v2zm0 8h2v-2h-2v2zm-4-8h2v-2h-2v2zm0-8h2V3h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ5/UxwF/zP8tLA1/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/Nv91LA8d4TpeAv+bf7rWB3+J5iRcM8S8zz+t1gN/m3+63gdfiOf0O8Nr827028Fs8L/GCIf5l5nm9DvDb/Nv9NvBaPKffAV6bf7vXBn6L5yVeMMS/zDyv1wF+m3+73wZei+f0O8Br82/32sBv8bzEC4b4l5nn9TrAb/Nv99vAa/Gcfgd4bf7tXhv4LZ6XeMEQ/zLzvF4H+G3+7X4beC2e0+8Ar82/3WsDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2z+mlga/ieb0Oz+urgZfiOf0N8NE8r9/ieX0M8Nc8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4gVD/MvM83od4Ld5Ti8NfDXP67V5Xl8NvDTP6a+Bj+Z5/TbP66OBv+Y5vTbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zb/fbwGvxnH4HeG3+7V4b+C2el3jBEP8y87xeB/ht/u1+G3gtntPvAK/Nv91rA7/F8xIvGOJfZp7X6wC/zb/dbwOvxXP6HeC1+bd7beC3eF7iBUP8y8zzeh3gt/m3+23gtXhOvwO8Nv92rw38Fs9LvGCIf5l5Xq8D/Db/dr8NvBbP6XeA1+bf7rWB3+J5iRcM8S8zz+t1gN/m3+6lgeM8p13gr/m3e23gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zz+mjgr/mf5aWBr+Z5iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BJACskFZY2jYAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderOuter;
impl IconShape for MdBorderOuter {
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
                d: "M13 7h-2v2h2V7zm0 4h-2v2h2v-2zm4 0h-2v2h2v-2zM3 3v18h18V3H3zm16 16H5V5h14v14zm-6-4h-2v2h2v-2zm-4-4H7v2h2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderRight;
impl IconShape for MdBorderRight {
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
                d: "M7 21h2v-2H7v2zM3 5h2V3H3v2zm4 0h2V3H7v2zm0 8h2v-2H7v2zm-4 8h2v-2H3v2zm8 0h2v-2h-2v2zm-8-8h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm8 8h2v-2h-2v2zm4-4h2v-2h-2v2zm4-10v18h2V3h-2zm-4 18h2v-2h-2v2zm0-16h2V3h-2v2zm-4 8h2v-2h-2v2zm0-8h2V3h-2v2zm0 4h2V7h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ5/UxwF/zv8Nv84Ih/mXmeb0O8Nv874f4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV6vA/w2//sh/mXmeb0O8Nv8z3IceCme1+/wgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV6vA/w2//sh/mXmeb0O8Nv874f4l5nn9TrAb/O/H+JfZp7X6wC/zf8sLw18Fc/rdXjBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lp4Kt5Xq/NC4b4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV6vA/w2//sh/mXmeb0O8Nv874f4l5nn9TrAb/O/H+JfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zHAdemuf127xgiH+ZeV6vA/w2//sh/mXmeb0O8Nv874f4l5nn9TrAb/O/H+JfZp7X6wC/zf9+iH+ZeV6vA/w2z+k48FI8r9/heb00cIzndAn4a57Xa/G8/gbY5TkdB16K5/U7vGCIf5l5Xq8D/DbP6bWB3+J5ief128Br8Zx+B3htnpd5Xq8D/DbP6bWB3+J5iRcM8S8zz+t1gN/mOb028Fs8L/G8fht4LZ7T7wCvzfMyz+t1gN/mOb028Fs8L/GCIf5l5nm9DvDbPKfXBn6L5yWe128Dr8Vz+h3gtXle5nm9DvDbPKfXBn6L5yVeMMS/zDyv1wF+m+f02sBv8bzE8/pt4LV4Tr8DvDbPyzyv1wF+m+f02sBv8bzEC4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcRDZJBIri9nQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderStyle;
impl IconShape for MdBorderStyle {
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
                d: "M15 21h2v-2h-2v2zm4 0h2v-2h-2v2zM7 21h2v-2H7v2zm4 0h2v-2h-2v2zm8-4h2v-2h-2v2zm0-4h2v-2h-2v2zM3 3v18h2V5h16V3H3zm16 6h2V7h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+Za/F8/obYJd/u5cGjvGcLgF/zb/dceCleF6/wwuG+JeZ5/U6wG/zb/fbwGvxnH4HeG3+7V4b+C2el3jBEP8y87xeB/ht/u1+G3gtntPvAK/Nv91rA7/F8xIvGOJfZp7X6wC/zb/dbwOvxXP6HeC1+bd7beC3eF7iBUP8y8zzeh3gt/m3+23gtXhOvwO8Nv92rw38Fs9LvGCIf5l5Xq8D/Db/dr8NvBbP6XeA1+bf7rWB3+J5iRcM8f8b4v83xP9viP/fEP+/If5lv8Xz+hjgr3lOLw18Fc/rdXheXw28FM/pb4CP5nn9Fs/rY4C/5jm9NPBVPK/X4QVD/MvM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i+clntdvA6/Fc/od4LV5XuZ5vQ7w2zyn1wZ+i+clXjDEv+y3eV4fDfw1z+mlga/meb02z+urgZfmOf018NE8r9/meX008Nc8p5cGvprn9dq8YIj/3xD/vyH+f0P8/4b4/w3xLzPP63WA3+bf7reB1+I5/Q7w2vzbvTbwWzwv8YIh/mXmeb0O8Nv82/028Fo8p98BXpt/u9cGfovnJV4wxL/MPK/XAX6bf7vfBl6L5/Q7wGvzb/fawG/xvMQLhviXmef1OsBv82/328Br8Zx+B3ht/u1eG/gtnpd4wRD/MvO8Xgf4bf7tfht4LZ7T7wCvzb/dawO/xfMSLxjiX/baPK+/Bnb5t3tp4DjPaRf4a/7tjgMvzfP6bV4wxP9viP/fEP+/If5/Q/zLXovn9TfALs/pOPBSPK/f4Xm9NHCM53QJ+Gue12vxvP4G2OU5HQdeiuf1O7xgiH+ZeV6vA/w2z+m1gd/ieYnn9dvAa/Gcfgd4bZ6XeV6vA/w2z+m1gd/ieYkXDPEvM8/rdYDf5jm9NvBbPC/xvH4beC2e0+8Ar83zMs/rdYDf5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i+clntdvA6/Fc/od4LV5XuZ5vQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpvn9NrAb/G8xPP6beC1eE6/A7w2z8s8r9cBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C2el3hevw28Fs/pd4DX5nmZ5/U6wG/znF4b+C2el3jBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHJoqiQTqvgcwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderTop;
impl IconShape for MdBorderTop {
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
                d: "M7 21h2v-2H7v2zm0-8h2v-2H7v2zm4 0h2v-2h-2v2zm0 8h2v-2h-2v2zm-8-4h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2v-2H3v2zm0-4h2V7H3v2zm8 8h2v-2h-2v2zm8-8h2V7h-2v2zm0 4h2v-2h-2v2zM3 3v2h18V3H3zm16 14h2v-2h-2v2zm-4 4h2v-2h-2v2zM11 9h2V7h-2v2zm8 12h2v-2h-2v2zm-4-8h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb/OcXhv4LZ6XeF6/DbwWz+l3gNfmeZnn9TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8XzEs/rt4HX4jn9DvDaPC/zvF4H+G2e02sDv8XzEi8Y4l9mntfrAL/Nc3pt4Ld4XuJ5/TbwWjyn3wFem+dlntfrAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXovn9DvAa/O8zPN6HeC3eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxL3ttntdfA7s8p+PAS/O8fpvn9dvAa/Gcfgd4bZ7Xa/O8/hrY5TkdB16a5/XbvGCI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woh/2WvxvP4G2OXf7reB1+I5/Q7w2vzbHQdeiuf1O7xgiH+ZeV6vA/w2/3a/DbwWz+l3gNfm3+61gd/ieYkXDPEvM8/rdYDf5t/ut4HX4jn9DvDa/Nu9NvBbPC/xgiH+ZeZ5vQ7w2/zb/TbwWjyn3wFem3+71wZ+i+clXjDEv8w8r9cBfpt/u98GXovn9DvAa/Nv99rAb/G8xAuG+JeZ5/U6wG/zb/fbwGvxnH4HeG3+7V4b+C2el3jBEP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ev+y3eF4fA/w1z+mlga/ieb0Oz+u3gdfiOf0O8No8r9/ieX0M8Nc8p5cGvorn9Tq8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviX/TbP66OBv+Y5vTTw1Tyv1+Z5/TbwWjyn3wFem+f12zyvjwb+muf00sBX87xemxcM8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L8y8zzeh3gt/m3+23gtXhOvwO8Nv92rw38Fs9LvGCIf5l5Xq8D/Db/dr8NvBbP6XeA1+bf7rWB3+J5iRcM8S8zz+t1gN/m3+63gdfiOf0O8Nr827028Fs8L/GCIf5l5nm9DvDb/Nv9NvBaPKffAV6bf7vXBn6L5yVeMMS/zDyv1wF+m3+73wZei+f0O8Br82/32sBv8bzEC4b4l702z+uvgV3+7X4beC2e0+8Ar82/3XHgpXlev80Lhvjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+Je9Fs/rb4BdntNx4KV4Xr/D8/pt4LV4Tr8DvDbP67V4Xn8D7PKcjgMvxfP6HV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bZ7TawO/xfMSz+u3gdfiOf0O8No8L/O8Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt3he4nn9NvBaPKffAV6b52We1+sAv81zem3gt3he4gVD/MvM83od4Ld5Tq8N/BbPSzyv3wZei+f0O8Br87zM83od4Ld5Tq8N/BbPS7xgiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj9DzwQcVGyCoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBorderVertical;
impl IconShape for MdBorderVertical {
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
                d: "M3 9h2V7H3v2zm0-4h2V3H3v2zm4 16h2v-2H7v2zm0-8h2v-2H7v2zm-4 0h2v-2H3v2zm0 8h2v-2H3v2zm0-4h2v-2H3v2zM7 5h2V3H7v2zm12 12h2v-2h-2v2zm-8 4h2V3h-2v18zm8 0h2v-2h-2v2zm0-8h2v-2h-2v2zm0-10v2h2V3h-2zm0 6h2V7h-2v2zm-4-4h2V3h-2v2zm0 16h2v-2h-2v2zm0-8h2v-2h-2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEK0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif64HAw/i2S4Bf81/LMT/LK8NvBfw1sBxntcu8NPATwM/w78f4n+G1wY+C3htXnS/DXwO8Nv82yH++30U8NX823028Dn82yD+e30X8N78+3038D786yH++7w38F38x/kc4LP510H893hp4K/4j/c6wG/zokP89/gt4LX5j/fbwOvwokP813tt4Lf4z/M2wE/zokH81/tq4KP4z/M9wHvzokH813s68GD+8+wCJ3jRIP7rmf98DwFu5V+G+K/12sBv8Z/vdYDf5l+G+K/12sBv8Z/vdYDf5l+G+K/12sBv8Z/vdYDf5l+G+K9n/vM9BLiVfxniv96twIP4z3MJOM6LBvFf76uBj+I/z/cA782LBvFf76WBv+I/z+sAv82LBvEf6ziwy7/st4HX4j/e7wCvzYsO8e/z2sB7AW8NHOfZ/hr4buB7gF2e14OBp/Mf73WA3+ZFh/i3OQ78FPDavHC3Au8D/DbP672B7+I/zvsA382/DuJf7zjwW8BL86J7H+C7eV6fDXwW/37fA7w3/3qIf72/Al6af51d4HWAv+Z5vTfwXfzbvQ/w3fzbIP513hv4Lv5tfht4HZ6/BwNfDbwVL7rfAT4a+Gv+7RD/On8FvDT/di8D/DUv2EsD7w28NfAgntczgJ8Gfhr4bf79EC+6BwNP59/nc4DP5kX30sBxrvht/uMhXnSvDfwW/z4/A7w1/3MgXnSvDfwW/z6/A7w2/3MgXnSvDfwW/z7fA7w3/3MgXnTHgYv8+3wM8NX8z4H41/lp4K34t3sIcCv/cyD+dV4b+C3+bb4HeG9edK/Fs/0O/zkQ/3rfDbwX/zqXgAcDu/zLPgr4bOA4z3Yr8DHAT/MfC/Fv89PAW/GiuQS8NvDX/Mu+GvgoXrD3Ab6b/ziIf7vPBj4aOMYL9jvAewO38i97beC3+Jc9BLiV/xiIf5/jwHsDbw0cBx4M/DXw18BPA7/Ni+67gffiX/Y1wEfzHwPxP8dvA6/Fv+x3gNfmPwbif47fBl6Lf9nvAK/NfwzE/xzfDbwX/7KvAT6a/xiI/zleG/gt/mUPAW7lPwbif5avBj6KF+x9gO/mPw7if56PBj4bOMazPQP4aOCn+Y+F+J/rtbliF/hr/nMg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8Cl6+AQSPeYBMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBubbleChart;
impl IconShape for MdBubbleChart {
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
                cx: "7.2",
                cy: "14.4",
                r: "3.2",
            }
            circle {
                cx: "14.8",
                cy: "18",
                r: "2",
            }
            circle {
                cx: "15.2",
                cy: "8.8",
                r: "4.8",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvuOPDS/O/w18Au/zLE/2+I/98Q/78h/n9DvOiOAy/F/w5/A+zyL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8II74YQR1iySYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDragHandle;
impl IconShape for MdDragHandle {
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
                d: "M20,9H4v2h16V9z M4,15h16v-2H4V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+4xwHXor/Gn8D7PLvh/iP89rAb/Ff43WA3+bfD/Ef57WB3+K/xusAv82/H+I/zmsDv8V/jdcBfpt/P8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/vyH+f0P8/4b4/w3x/xviX/Zb/O/2OrxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9lv87/ba/OCIf5/Q/z/hvj/DfH/G+L/N8R/nNcGfov/Gq8D/Db/foj/OK8N/Bb/NV4H+G3+/RD/cV4b+C3+a7wO8Nv8+yH+47w28Fv813gd4Lf590P8x3lt4Lf4r/E6wG/z74f4j3MceGn+a/w1sMu/H+L/N8T/b4j/3xD/vyH+Za/F/26/wwuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/At2nKkEZGhf8AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatAlignCenter;
impl IconShape for MdFormatAlignCenter {
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
                d: "M7 15v2h10v-2H7zm-4 6h18v-2H3v2zm0-8h18v-2H3v2zm4-6v2h10V7H7zM3 3v2h18V3H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+Za/F/26/wwuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhvj/DfH/G+L/N8T/b4j/3xD/st/if7fX4QVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y36b/91emxcM8f8b4v83xP9viP/fEP+/If5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+Za/N/26/zQuG+P8N8f8b4v83xP9viH/Za/G/2+/wgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+EjB5BBdT8IgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatAlignJustify;
impl IconShape for MdFormatAlignJustify {
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
                d: "M3 21h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18v-2H3v2zm0-4h18V7H3v2zm0-6v2h18V3H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACC0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+Za/Ff6/f4T8P4l9m/nuJ/zyIf5n57yX+8yD+Zea/l/jPg/iXmf9e4j8P4l9m/nuJ/zyI/98Q/78h/n9D/P+G+P8N8S/7Lf53ex1eMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/st/nf7bV5wRD/vyH+f0P8/4b4/w3x/xviX2b+e4n/PIh/mfnvJf7zIP5l5r+X+M+D+JeZ/17iPw/iX2b+e4n/PIh/2Wvz3+u3+c+D+P8N8f8b4v83xP9viH/Za/G/2+/wgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/nTB5BQgctLgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatAlignLeft;
impl IconShape for MdFormatAlignLeft {
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
                d: "M15 15H3v2h12v-2zm0-8H3v2h12V7zM3 13h18v-2H3v2zm0 8h18v-2H3v2zM3 3v2h18V3H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACCklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+870W/71+hxcM8Z/P/PcSLxjiP5/57yVeMMR/PvPfS7xgiP985r+XeMEQ//nMfy/xgiH+f0P8/4b4/w3x/xvi/zfEv+y3+N/tdXjBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/7Lf5n+31+YFQ/z/hvj/DfH/G+L/N8T/b4j/fOa/l3jBEP/5zH8v8YIh/vOZ/17iBUP85zP/vcQLhvjPZ/57iRcM8Z/vtfnv9du8YIj/3xD/vyH+f0P8/4b4l70W/7v9Di8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/COcfHkGnCDRnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatAlignRight;
impl IconShape for MdFormatAlignRight {
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
                d: "M3 21h18v-2H3v2zm6-4h12v-2H9v2zm-6-4h18v-2H3v2zm6-4h12V7H9v2zM3 3v2h18V3H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP85x4KX4j/EM4Fb+8yH+47w28Fv8x/lr4KeBrwF2+c+B+I/z2sBv8R9vF/gY4Lv5j4f4j/PawG/xn+e7gffhPxbiP85rA7/Ff66vAT6a/ziI/zivDfwW//leB/ht/mMg/uO8NvBb/Oe7FXgI/zEQ/3FeG/gtXrC/AT6aF+61gfcGHsQL9zbAT/Pvh/iP89rAb/GC/Q7w2rxovht4L16w7wHem38/xH+c1wZ+ixfsd4DX5kV3K/Agnr/fAV6bfz/Ef5zXBn6LF+x3gNfmRffZwGfxgol/P8R/nNcGfosX7HeA1+ZF99HAV/GCiX8/xH+c1wZ+ixfsd4DX5kX31cBH8fz9DvDa/Psh/uO8NvBbvGC/A7w2L5oHA38FHOf5+x7gvfn3Q/zHeW3gt3jBfgd4bf5lLw18F/DSvGAPAW7l3w/xH+e1gd/iBdsF/poX7jjw0rxwPwO8Nf8xEP9xXhv4Lf5zPQN4aWCX/xiI/zivDfwW/3kuAa8N/DX/cRD/cV4b+C3+czwDeGlgl/9YiP84rw38Fv95fht4H+BW/uMg/uO8NvBb/OfaBV4H+Gv+YyD+47w28Fv859sFXga4lX8/xH+c1wZ+i/8avw28Dv9+iP84rw38Fi/Y3wAfzb/spYGXBt6LF+59gO/m3wfxH+e1gd/iBfsd4LV50b028Fu8YL8NvA7/Poj/OK8N/BYv2O8Ar82/zmcDn8ULJv59EP9xXhv4LV6w3wFem3+dlwb+ihfsdYDf5t8O8R/ntYHf4gX7HeC1+dczL9jrAL/Nvx3iP85rA7/FC/Y7wGvzr/PawG/xgr0O8Nv82yH+47w28Fu8YL8DvDb/Or8FvDYv2OsAv82/HeI/zmsDv8UL9jvAa/OiOQ58FfDevGCXgOP8+yD+47w28Fu8YLvAX/OieW3+Zd8DvDf/Poj/OK8N/Bb/dV4H+G3+fRD/cV4b+C3+a/wM8Nb8+yH+47w28Fv85/sb4LWBXf79EP9xXhv4Lf5z/Q3w2sAu/zEQ/3FeG/gt/vN8DvDVwC7/cRD/cV4b+C3+41wC/hr4aeCngVv5j4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C5mt8QZVmKn0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatBold;
impl IconShape for MdFormatBold {
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
                d: "M15.6 10.79c.97-.67 1.65-1.77 1.65-2.79 0-2.26-1.75-4-4-4H7v14h7.04c2.09 0 3.71-1.7 3.71-3.79 0-1.52-.86-2.82-2.15-3.42zM10 6.5h3c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-3v-3zm3.5 9H10v-3h3.5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7K3Bn4b2OX5+y3+5/kY4K/5lyFeuPcGvgv4a+B1gF2e13sD38X/LK8D/Db/MsQL9t7Ad/Fsfw28DrDL83pv4Lv4n+N1gN/mX4Z4/t4a+Cme118DrwPs8rzeG/gu/md4HeC3+Zchnr/jwG8DL8Xz+mvgdYBdntd7A9/Ff7/XAX6bfxniBTsO/DbwUjyvvwZeB9jleb038F3893od4Lf5lyFeuOPAbwMvxfP6a+B1gF2e13sD38V/n9cBfpt/GeJfdhz4beCleF5/DbwOsMvzem/gu/jv8TrAb/MvQ7xojgO/DbwUz+uvgdcBdnleLw0c53k9GPguXrC/AT6af7u/Bnb5lyFedMeB3wZeiuf118DrALu8aL4beC9esPcBvpv/fIh/nePAbwMvxfP6a+B1gF1euOPA04HjPH+XgAcDu/znQ/zrHQd+G3gpntdfA68D7PKCfTTwVbxgXwN8NP81EP82x4HfBl6K5/XXwOsAuzx/TwcezAv2EOBW/msg/u2OA78NvBTP66+B1wF2eV7vDXwXz9/PAG/Nfx3Ev89x4LeBl+J5/TXwOsAuz+u9ge/ieb0O8Nv810H8+x0Hfht4KZ7XXwOvA+zyvN4b+C6e7RnAg/mvhfiPcRz4beCleF5/DbwOsMvzem/gu7jifYDv5r8W4j/OceC3gZfief018DrALs/rvYGvBh4M7PJfC/Ef6zjw28BL8bz+GngdYJfn9WDgVv7rIf7jHQd+G3gpntdfA68D7PI/A+I/x3Hgt4GX4nn9NfA6wC7//RD/eY4Dvw28FM/rr4HXAXb574X4z/XSwF/x/P018DrALv99EP+5vht4L16wvwZeB9jlvwfiP89x4OnAcV64vwZeB9jlvx7iP89HA1/Fi+avgdcBdvmvhfjP83Tgwbzo/hp4HWCX/zqI/xxvDfwU/3p/DbwOsMt/DcR/jt8CXpsX7G2AzwZeiuf118DrALv850P8x3sw8HResGcADwaOA78NvBTP66+B1wF2+c+F+I/33cB78YK9D/DdXHEc+G3gpXhefw28DrDLfx7Ef6zjwNOB4zx/l4AHA7s823Hgt4GX4nn9NfA6wC7/ORD/sT4a+CpesK8BPprndRz4beCleF5/DbwOsMt/PMR/rKcDD+YFewhwK8/fceC3gZfief018DrALv+xEP9x3hr4KV6wnwHemhfuOPDbwEvxvP4aeB1gl/84iP84vwW8Ni/Y6wC/zb/sOPDbwEvxvP4aeB1gl/8YiP8YDwaezgv2DODBvOiOA78NvBTP66+B1wF2+fdD/Mf4aOCtecG+Gvhp/nWOA78NvBTP66+B1wF2+fdB/M92HPht4KV4Xn8NvA6wy78d4n++48BvAy/F8/od4LX5t0P873Ac+G3gpXi2S8BrA3/Nvx3if4/jwG8DLwVcAl4b+Gv+fRD/uxwHfhr4aOCv+fdD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BTXe6QR+ElsQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatClear;
impl IconShape for MdFormatClear {
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
                d: "M3.27 5L2 6.27l6.97 6.97L6.5 19h3l1.57-3.66L16.73 21 18 19.73 3.55 5.27 3.27 5zM6 5v.18L8.82 8h2.4l-.72 1.68 2.1 2.1L14.21 8H20V5H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFvUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviP9dLAVwFvA+zyPx/iP85LA78FHAf+GngdYJf/2RD/MV4a+C3gOM/218DrALv8z4X493sw8FfAcZ7XXwOvA+zyPxPiP8Z3A+/F8/fXwOsAu/zPg/iP893Ae/H8/TXwOsAu/7Mg/mN9N/BePH9/DbwOsMv/HIj/eN8NvBfP318DrwPs8j8D4j/HdwPvxfP318DrALv890P85/lu4L14/v4aeB1gl/9eiP887w18Fy/YXwOvA+zy3wfxn+O9ge/iX/bXwOsAu/z3QPzHe2/gu3jR/TXwOsAu//UQ/7HeG/gu/vX+GngdYJf/Woj/OO8NfBfP3yXgtYGPBt6L5++vgdcBdvmvg/iP8d7Ad/H8XQJeG/hrrvhu4L14/v4aeB1gl/8aiH+/9wa+i+fvEvDawF/znL4beC+ev78GXgfY5T8f4t/nvYHv4vm7BLw28Nc8f98NvBfP318DrwPs8p8L8W/33sB38fxdAl4b+GteuO8G3ovn76+B1wF2+c+D+Ld5b+C7eP4uAa8N/DUvmu8G3ovn76+B1wF2+c+B+Nd7b+C7eP4uAa8N/DX/Ot8NvBfP318DrwPs8h8P8a/z3sB38fxdAl4b+Gv+bb4beC+ev78GXgfY5T8W4kX33sB38fxdAl4b+Gv+fb4beC+ev78GXgfY5T8O4kXz3sB38fxdAl4b+Gv+Y3w38F48f38NvA6wy38MxL/stYHf4vm7BLw28Nf8x/pu4L14/r4G+Gj+YyD+Zb8NvBbP6xLw2sBf85/ju4H34vl7CHAr/36If5l5/j4a+Gv+c3018NI8r9cBfpt/P8S/7K+Bl+J/lpcB/pp/P8S/7LOBz+J/jr8BXpr/GIgXzV8DL8V/v0vAawN/zX8MxIvmOPDbwEvx3+cS8NrAX/P8HQd2+ddBvOiOA78NvBT/9S4Brw38Nc/fSwO/BXwM8N286BD/OseB3wZeiv86l4DXBv6a5+848FvAS3PFywB/zYsG8a93HPht4KX4z3cJeG3gr3nBvgt4b57tVuBlgF3+ZYh/m+PAbwMvxX+eS8BrA3/NC/bWwE/xvH4aeBv+ZYh/u+PAbwMvxX+8S8BrA3/NC3YceDpwnOfvbYCf5oVD/PscB34beCn+41wCXhv4a164zwY+ixfsVuAhvHCIf7/jwG8DL8W/3yXgtYG/5oU7DjwdOM4L9z7Ad/OCIf5jHAd+G3gp/u0uAa8N/DX/svcGvot/2c8Ab80LhviPcxz4beCl+Ne7BLw28Ne8aL4a+Cj+ZbvACV4wxH+s48BvAy/Fi+4S8NrAX/Oi+27gvXjRiBcM8R/vOPDbwEvxL7sEvDbw1/zrfDTwVfzLfgd4bV4wxH+O48BvAy/FC3YJeG3gr/nXOw7cChzjhXsf4Lt5wRD/eY4Dvw28FM/rEvDawF/zb/fewHfxgn0P8N68cIj/XMeBzwY+imf7G+C9gb/m3++1ge8GHsSzXQK+Gvhs/mWI/xrHgZcGdoG/5j/eSwPHueK3edEh/n9D/P+G+P8N8f8b4v83xP9viP/fEM/fVwMvxf8tfwN8NM8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPiX8EPArdQeQOahwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatColorFill;
impl IconShape for MdFormatColorFill {
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
                d: "M16.56,8.94L7.62,0L6.21,1.41l2.38,2.38L3.44,8.94c-0.59,0.59-0.59,1.54,0,2.12l5.5,5.5C9.23,16.85,9.62,17,10,17 s0.77-0.15,1.06-0.44l5.5-5.5C17.15,10.48,17.15,9.53,16.56,8.94z M5.21,10L10,5.21L14.79,10H5.21z M19,11.5c0,0-2,2.17-2,3.5 c0,1.1,0.9,2,2,2s2-0.9,2-2C21,13.67,19,11.5,19,11.5z M2,20h20v4H2V20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFBElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b473WcK3b574H473Mc+C2ueB1gl/96iP8+3wW8N1d8N/A+/NdD/Pf4aOCreE4fA3w1/7UQ//VeG/gtnr/XAX6b/zqI/1oPBv4KOM7ztwu8DHAr/zUQ/zbvDbwW8D786/wV8NK8cH8NvAz/NRD/eu8NfBdXfDfwPrxovhr4KF40XwN8NP/5EP867w18F8/pu4H34YV7beC3+Nd5G+Cn+c+FeNE9GHg6z993A+/D83cceDpwnH+dXeAhwC7/eRD/Ou8NfBfP33cD78Pz993Ae/Gv99PA2/CfB/Gv997Ad/H8fTfwPjx/3w28F/96HwN8Nf85EP827w18F8/fdwPvw/P33cB78a+zCzwE2OU/HuLf7r2B7+L5+27gfXj+vht4L/51fhp4G/7jIf593hv4Lp6/7wbeh+fvu4H34l/nbYCf5j8W4t/vvYHv4vn7buB9eP6+G3gvXnS3Ai8D7PIfB/Ef472B7+L5+27gfXj+vht4L150nwN8Nv9xEP9x3hv4Lp6/7wbeh+fvu4H34kWzCzwE2OU/BuI/1k8Db8Xz993A+/D8fTfwXrxovgd4b/5jIP7jHAcu8sJ9N/A+PH/fDbwXL5qHALfy74f4j/PZwGfxL/tu4H14/r4beC/+Zd8DvDf/foj/GMeBpwPHedF8N/A+PH/fDbwXL9wu8BBgl38fxH+M9wa+i3+d7wbeh+fvu4H34oX7GOCr+fdB/Mf4K+Cl+df7buB9eP6+G3gvXrC/Bl6Gfx/Ev9+Dgafzb/fdwPvw/H038F68YA8BbuXfDvHv997Ad/Hv893A+/D8fTfwXjx/HwN8Nf92iH+/nwbein+/7wbeh+fvu4H34nl9D/De/Nsh/v1+G3gt/mN8N/A+PH/fDbwXz+l3gNfm3w7x72f+Y3038D48f98NvBfPSfzbIf79zH+87wbeh+fvu4H34tnEvx3i3+9W4EH8x/tu4H14/r4beC+uEP92iH+/3wZei/8c3w28D8/fdwMvDbw0/3aIf7+fBt6K/zzfDbwPz997A9/Nvx3i3++9ge/iP9d3A+/DfzzEv99x4CL/+b4beB/+YyH+Y/w28Fr85/tu4H34j4P4j/HawG/xX+O7gffhPwbiP85PA2/Ff43vBt6Hfz/Ef5wHA38NHOO/xncD78O/D+I/1msDv8V/ne8G3od/O8R/vPcGvov/Ot8NvA//Noj/HG8NfDdwjP8Yl4CfBt6L5++7gffhXw/xn+elgZ8GHsS/z98A7w38NfDewHfx/H038D786yD+87038NnAg/jXeQbw2cB385zeG/gunr/vBt6HFx3iv85bA28NvDVwjOfvGcBvAz8N/DQv2HsD38Xz993A+/CiQfz3eW2e018Du7zo3hv4Lp7X+wDfzYsG8b/bewPfxbO9D/DdvOgQ//u9N/BdwPsA382/DuL/hgcDt/Kvh/j/DfH/G+L/N8T/b4j/3xD/vyH+f+MfASaUpEH7fv/yAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatColorReset;
impl IconShape for MdFormatColorReset {
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
                d: "M18 14c0-4-6-10.8-6-10.8s-1.33 1.51-2.73 3.52l8.59 8.59c.09-.42.14-.86.14-1.31zm-.88 3.12L12.5 12.5 5.27 5.27 4 6.55l3.32 3.32C6.55 11.32 6 12.79 6 14c0 3.31 2.69 6 6 6 1.52 0 2.9-.57 3.96-1.5l2.63 2.63 1.27-1.27-2.74-2.74z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEtUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/fVwEfx/H0N8NH810H813s68GCev1uBh/BfB/Ff662Bn+KFexvgp/mvgfiv9d3Ae/HCfQ/w3vzXQPzXOQ5c5EVzAtjlPx/iv857A9/Fi+Z9gO/mPx/iv85fAS/Ni+avgZfhPx/iv8aDgafzvL6HK96L5/UQ4Fb+cyH+a3w18FE8r7fhip/ieX0N8NH850L813g68GCe0yXgOFfsAsd4TrcCD+E/F+I/31sDP8Xz+h7gvbniu4H34nm9DfDT/OdB/Of7buC9eF4vA/w1V7w08Fc8r+8B3pv/PIj/XMeBizyvZwAP5jndCjyI53UC2OU/B+I/13sD38Xz+hrgo3lOXw18FM/rfYDv5j8H4j/XXwEvzfN6GeCveU4vDfwVz+uvgZfhPwfiP8+DgafzvJ4BPJjn71bgQTyvhwC38h8P8Z/nq4GP4nl9DPDVPH8fDXwVz+trgI/mPx7iP8/TgQfzvB4C3Mrz92Dg6TyvW4GH8B8P8Z/jrYGf4nn9DfDSvHB/DbwUz+ttgJ/mPxbiP8d3A+/F8/oY4Kt54T4a+Cqe1/cA781/LMR/vOPARZ6/E8AuL9yDgafz/J0AdvmPg/iP997Ad/G8fgZ4a140Pw28Fc/rfYDv5j8O4j/eXwEvzfO6FbiVF82DgQfzvP4aeBn+4yD+Yz0YeDr/uR4C3Mp/DMR/rK8GPor/XF8DfDT/MRD/sZ4OPJj/XLcCD+E/BuI/zlsDP8V/jbcBfpp/P8R/nJ8G3orn9QzgVv5tHgw8iOf1PcB78++H+I9xHLjI8/cywF/zb/PSwF/x/J0Advn3QfzH+Gjgq3hezwAezL/PrcCDeF7vA3w3/z6I/xh/Bbw0z+trgI/m3+ezgc/ief018DL8+yD+/V4a+Cuev4cAt/Lv82Dg6Tx/DwFu5d8O8e/31cBH8bz+Bnhp/mP8NfBSPK+vAT6afzvEv99F4DjP62OAr+Y/xkcDX8XzuhV4CP92iH+ftwZ+iufvIcCt/Md4MPB0nr+3AX6afxvEv89PA2/F8/oZ4K35j/XTwFvxvL4HeG/+bRD/dseBizx/7wN8N/+x3hv4Lp6/E8Au/3qIf7vjwEvz/P01sMt/rOPAS/P8/TWwy78e4v83xP9viP/fEP+/If5/Q/z/hvj/DfH8fTXwUvzf8jfAR/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznPhHk/WWQQNRsdUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatColorText;
impl IconShape for MdFormatColorText {
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
                d: "M2,20h20v4H2V20z M5.49,17h2.42l1.27-3.58h5.65L16.09,17h2.42L13.25,3h-2.5L5.49,17z M9.91,11.39l2.03-5.79h0.12l2.03,5.79 H9.91z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADK0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+e7w0cIz/Gr/DC4b47/HbwGvxX0O8YIj/Hr8NvBb/NcQLhviP897Ad/Oi+W3gtfivIV4wxH+M7wIeArw2L5rfBl6L/xriBUP8+30X8N7A7wCvzYvmt4HX4r+GeMEQ/z7fBbw3V/wO8Nr874L4t/su4L15tt8BXpv/XRD/Nt8FvDfP6XeA1+Z/F8S/3ncB783z+h3gtfnfBfGv813Ae/P8/Q7w2vzvgnjRfRfw3rxgvwO8Ni+arwZeiv8ar8MLhnjRfBfw3rxwvwO8Ni+a3wZei/8a4gVD/Mu+C3hv/mW/A7w2L5rfBl6L/xriBUP8y94b+C7+Zb8DvDYvmt8GXov/GuIFQ7xo3hv4Ll643wFemxfNbwOvxX8N8YIhXnTvDXwXL9jvAK/Ni+argZfmv8Zr84Ih/nXeG/gunr/fAV6b/10Q/3rvDXwXz+t3gNfmfxfEv817A9/Fc/od4LX53wXxb/fewHfxbL8DvDb/uyD+fd4b+C6u+B3gtfnfBfHv997AdwG/A7w2L5rfBl6L/xriBUP8x3hv4L2B1+ZF89vAa/FfQ7xgiP84Lw38NS+a3wZei/8a4gVD/Pf4beC1+K8hXjDEf4/fBl6L/xriBUP893hp4Dj/NX6bFwzx/xvi/zfE/2+I/98Q/7LX4n+33+EFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEuUkZBU+fVgwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatIndentDecrease;
impl IconShape for MdFormatIndentDecrease {
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
                d: "M11 17h10v-2H11v2zm-8-5l4 4V8l-4 4zm0 9h18v-2H3v2zM3 3v2h18V3H3zm8 6h10V7H11v2zm0 4h10v-2H11v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+e7w0cIz/Gr/DC4b47/HbwGvxX0O8YIj/Hr8NvBb/NcQLhviXvTTw1/zH+m3gtfivIV4wxL/sIvA6wF/zH+e3gdfiv4Z4wRD/MgO7wOsAf81/jN8GXov/GuIFQ/zLzBW7wOsAf83/HYh/mXm2XeB1gL/m/wbEv8w8p13gdYC/5n8/xL/MPK9d4HWAv+Z/N8S/zDx/u8DrAH/N/16If5l5wXaB1wH+mn+drwZeiv8ar8MLhviXmRduF3gd4K950f028Fr81xAvGOJfZv5lu8DrAH/Ni+a3gdfiv4Z4wRD/MvMvuwS8NvDXvGh+G3gt/muIFwzxLzMv3CXgtYG/5kX328Br8V9DvGCIf5l5wS4Brw38Nf86Xw28NP81XpsXDPEvM8/fJeC1gb/mfy/Ev8w8r0vAawN/zf9uiH+ZeU6XgNcG/pr//RD/MvNsl4DXBv6a/xsQ/zJzxSXgtYG/5v8OxL/MwCXgtYG/5j/GbwOvxX8N8YIh/mW7wGsDf81/nN8GXov/GuIFQ/zLXhr4a/5j/TbwWvzXEC8Y4r/HbwOvxX8N8YIh/nv8NvBa/NcQLxjiv8dLA8f5r/HbvGCI/98Q/78h/n9D/P+G+Je9Fv+7/Q4vGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/whrt1pBONWLGQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatIndentIncrease;
impl IconShape for MdFormatIndentIncrease {
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
                d: "M3 21h18v-2H3v2zM3 8v8l4-4-4-4zm8 9h10v-2H11v2zM3 3v2h18V3H3zm8 6h10V7H11v2zm0 4h10v-2H11v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv85x4KX4j/c7/Nsh/uu8NvBb/McT/3aI/zqvDfwW//HEvx3iv85rA7/Ffzzxb4f4r/PawG/xH0/82yH+67w28Fv8xxP/doj/Oq8N/Bb/8cS/HeK/zmsDv8V/PPFvh/iv89LAV/O8Hgw8iBfsGcCtvGCvzb8d4r/fXwEvzQv2MsBf858D8d/rtYHf4gX7HeC1+c+D+O/13cB78YK9D/Dd/OdB/Pd5MPB0XrBnAA/mPxfiv89nA5/FC/Y5wGfznwvx3+cicJwX7ASwy38uxH+P9wa+ixfse4D35j8f4r/HXwEvzQv2MsBf858P8V/vtYHf4gX7HeC1+a+B+K/33cB78YK9D/Dd/NdA/Nd6MPB0XrBnAA/mvw7iv9ZnA5/FC/Y5wGfzXwfxX+sicJwX7ASwy38dxH+d9wa+ixfse4D35r8W4r/OXwEvzQv2MsBf818L8V/jpYG/4gX7G+Cl+a+H+K/x3cB78YK9D/Dd/NdD/Oc7DlzkBbsEHOe/B+I/32cDn8UL9jnAZ/PfA/Gf7+nAg3nBHgLcyn8PxH+u9wa+ixfse4D35r8P4t/P/Md7HeC3+c+H+Pcz//FeB/ht/vMh/v3Mf7zXAX6b/3yIfz/zH+91gN/mPx/i38/8x3sd4Lf5z4f49zP/8V4H+G3+8yH+/cx/vNcBfpv/fIh/P/Mf73WA3+Y/H+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wj4vUVBZEnJCQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatItalic;
impl IconShape for MdFormatItalic {
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
                d: "M10 4v3h2.21l-3.42 8H6v3h8v-3h-2.21l3.42-8H18V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xr3cc+Cjgc/jfD/Gvcxz4LeClge8G3of/3RAvuuPAbwEvzbN9N/A+/O+FeNEcB34LeGme13cD78P/Toh/2XHgt4CX5gX7buB9eOFeGvgq/vv8DfDRPCfEC3cc+C3gpfmXfTfwPrxgrw38Fv99fgd4bZ4T4gU7DvwW8NK86L4beB+ev9cGfov/Pr8DvDbPCfH8HQd+C3hp/vW+G3gfntdrA7/Ff5/fAV6b54R4XseB3wJemn+77wbeh+f02sBv8d/nd4DX5jkhntdXAy/N8zoOvBTP63d4/r4b+G6e7aWBr+a/z18DH81zQrzoXhv4LZ6X+N8L8aJ7beC3eF7ify/Ei+61gd/ieYn/vRAvutcGfovnJf73QrzoXhv4LZ6X+N8L8aJ7beC3eF7ify/Ei+61gd/ieYn/vRAvutcGfovnJf73QrzoXhv4LZ6X+N8L8aJ7beC3eF7ify/Ei+61gd/ieYkXzUsDX8V/n78BPprnhHjRvTbwWzwv8aJ5beC3+O/zO8Br85wQL7rXBn6L5yVeNK8N/Bb/fX4HeG2eE+JF99rAb/G8xIvmtYHf4r/P7wCvzXNCvOheG/gtnpd40bw28Fv89/kd4LV5TogX3WsDv8XzEi+alwa+mv8+fw18NM8J8aJ7beC3eF7ify/Ei+61gd/ieYn/vRAvutcGfovnJf73QrzoXhv4LZ6X+N8L8aJ7beC3eF7ify/Ei+61gd/ieYn/vRAvutcGfovnJf73QrzoXhv4LZ6X+N8L8aJ7beC3eF7ify/Ei+61gd/ieYn/vRDP66uBl+J5HQdemuf12zx/HwP8Nc/20sBX8d/nb4CP5jkhntdx4LeBl+Lf7n2A7+Y5vTbwW/z3+R3gtXlOiOfvOPDbwEvxr/c+wHfzvF4b+C3++/wO8No8J8QLdhz4beCleNG9D/DdPH+vDfwW/31+B3htnhPihTsO/DbwUvzL3gf4bl6w1wZ+i/8+vwO8Ns8J8S87Dvw28FK8YO8DfDcv3EsDX81/n78GPprnhHjRHAd+G3gpntf7AN/N/06IF91x4LeBl+LZ3gf4bv73QvzrHAd+G3gp4H2A7+Z/N8S/3nHgtYGf5n8/xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj7admQWYlO0wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatLineSpacing;
impl IconShape for MdFormatLineSpacing {
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
                d: "M6 7h2.5L5 3.5 1.5 7H4v10H1.5L5 20.5 8.5 17H6V7zm4-2v2h12V5H10zm0 14h12v-2H10v2zm0-6h12v-2H10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If9lrA58FvDbw18BPA5/D/w2IF+69ge/ief028Do8p5cGvor/eV6HFwzxwl0EjvP8vQ/w3TzbawO/xf884gVDvGCvDfwWL9jXAB/Ns7028Fv8zyNeMMQL9trAb/GCfQ3w0TzbawO/xf884gVDvHC7wDGev/cBvptne23gt/ifR7xgiBfuvYHv4nn9DvDaPKeXBr6a/3lemxcM8S97beCzgdcC/gb4aeCz+b8B8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfEve23gs4DXBv4a+Gngc/i/AfHCvTfwXTyv3wZeh+f00sBX8T/P6/CCIV64i8Bxnr/3Ab6bZ3tt4Lf4n0e8YIgX7LWB3+IF+xrgo3m21wZ+i/95xAuGeMFeG/gtXrCvAT6aZ3tt4Lf4n0e8YIgXbhc4xvP3PsB382yvDfwW//OIFwzxwr038F08r98BXpvn9NLAV/M/z2vzgiH+Za8NfDbwWsDfAD8NfDb/NyD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/mWvDXwW8NrAXwM/DXwO/zcgXrj3Br6L5/XTwNvwnF4a+Cr+53kdXjDEC3cROM7z9z7Ad/Nsrw38Fv/ziBcM8YK9NvBbvGBfA3w0z/bawG/xP494wRAv2GsDv8UL9jXAR/Nsrw38Fv/ziBcM8cLtAsd4/t4G+Gme7bWB3+J/HvGCIV649wa+i+f1M8Bb85xeGvhq/ud5bV4wxL/stYHPBl4L+Bvgp4HP5v8GxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAbobSEGrDv6jAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatListBulleted;
impl IconShape for MdFormatListBulleted {
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
                d: "M4 10.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0-6c-.83 0-1.5.67-1.5 1.5S3.17 7.5 4 7.5 5.5 6.83 5.5 6 4.83 4.5 4 4.5zm0 12c-.83 0-1.5.68-1.5 1.5s.68 1.5 1.5 1.5 1.5-.68 1.5-1.5-.67-1.5-1.5-1.5zM7 19h14v-2H7v2zm0-6h14v-2H7v2zm0-8v2h14V5H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi+Xtp4BjP6RLw1/zfgnj+fht4LZ7T7wCvzf8tiOfvt4HX4jn9DvDa/N+CeP5eGjjOFb/FFb8DvDYv2EsDX8X/PK/DC4b4l5krfgd4bV6w1wZ+i/95xAuG+JeZK34HeG1esNcGfov/ecQLhviXmSt+B3htXrDXBn6L/3nEC4b4l5krfgd4bV6w1wZ+i/95xAuG+JeZK34HeG1esJcGvpr/eV6bFwzxLzNX/A7w2vzfgviXmSt+B3ht/m9B/Mtemyt2gb/m/xbE/2+I/98Q/78h/n9DPH8vDRzjBfsbYJf//RDP328Dr8UL9jrAb/O/H+L5+23gtXjBXgf4bf73Qzx/Lw0c59mOAz/FFc8AXhrY5Tm9NPBV/M/zOrxgiH/ZceC3gJcGLgGvDfw1z+u1gd/ifx7xgiH+ZT8FvDVXvA7w2zx/rw38Fv/ziBcM8cJ9F/DeXPE+wHfzgr028Fv8zyNeMMQL9t7Ad3HF1wAfzQv32sBv8T+PeMEQz99bAz/FFX8NfDTP6W+AXZ7TSwNfzf88r80Lhnj+fht4LV6w1wF+m//9EM/fbwOvxQv2OsBv878f4vl7aeA4L9hfA7v874f4/w3x/xvi/zfE/2+I5++lgWO8YH8D7PK/H+L5+23gtXjBXgf4bf73Qzx/vw28Fi/Y6wC/zf9+iOfvpYHjPKeXBr6KK14H+G2e00sDX8X/PK/DC4b4l70WV7w08NVc8TrAb/OcXhv4Lf7nES8Y4l9mntfrAL/Nc3pt4Lf4n0e8YIh/mXlerwP8Ns/ptYHf4n8e8YIh/mWvzRUvDXwVV7wO8Ns8p9cGfov/ecQLhnj+Xho4xnN6aeCrueJ1gN/mOb008NX8z/PavGCI5++3gdfiBXsd4Lf53w/x/P028Fq8YK8D/Db/+yGev5cGjvOC/TWwy/9+iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPY3leQdRVqtUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatListNumbered;
impl IconShape for MdFormatListNumbered {
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
                d: "M2 17h2v.5H3v1h1v.5H2v1h3v-4H2v1zm1-9h1V4H2v1h1v3zm-1 3h1.8L2 13.1v.9h3v-1H3.2L5 10.9V10H2v1zm5-6v2h14V5H7zm0 14h14v-2H7v2zm0-6h14v-2H7v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif7/X4nn9DbDLvwzxv595Xq8D/Db/MsT/fuZ5vQ7w2/zLEM/fVwMvxf8sfwN8NM/rtbnipYGv4orXAX6bfxni+ftt4LX4n+V3gNfmBXtt4Le44nWA3+Zfhnj+fht4Lf5n+R3gtXnBXhv4La54HeC3+Zchnr/fBl6L/1l+B3htXrDXBn6LK14H+G3+ZYjn77eB1+J/lt8BXpsX7LWB3+KK1wF+m38Z4vn7auCl+Z/lr4GP5gV7beC3uOJ1gN/mX4b4v+O1gd/iitcBfpt/GeL/jtcGfosrXgf4bf5liP87jgMvzRV/DezyL0P8/4b4/w3x/xvi/zfE/36vxQv3O7xgiP/9zAsnXjDE/37mhRMvGOL5+2rgpfif5W+Aj+Z5vTbP6b2B9+KK7wHemxcM8fz9NvBa/M/yO8Br88K9NPBXXPE3wEvzwiGev98GXov/WX4HeG1esJcGfgs4DvwN8NrALi8c4vn7beC1+J/ld4DX5vk7DvwW8NLAJeC1gb/mX4Z4/n4beC3+Z/kd4LV5/v4KeGmueBngr3nRIJ6/rwZemv9Z/hr4aJ7XdwHvzRXfDXw3z+l3eMEQ//uZF068YIj//cwLJ14wxP9+r80L99u8YIj/3xD/vyH+f0P8/4b43++1eOF+hxcM8b+feeHEC4b438+8cOIFQzx/Xw28FP+z/A3w0Tyv1+Z5fTXwUlwhXjDE8/fbwGvxP8vvAK/NC3YceCmu+GrgpblCvGCI5++3gdfif5bfAV6bF+y1gd/ieYkXDPH8/TbwWvzP8jvAa/OCvTbwWzwv8YIhnr/fBl6L/1l+B3htXrDjwEtzxVcDL8UV4gVDPH9fDbw0/7P8NfDRPK/X4nl9NfDSXCFeMMT/fuaFEy8Y4n8/88KJFwzxv99r88L9Ni8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj6kldQQsQrXQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatListNumberedRtl;
impl IconShape for MdFormatListNumberedRtl {
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
                d: "M18 17h2v.5h-1v1h1v.5h-2v1h3v-4h-3zm1-9h1V4h-2v1h1zm-1 3h1.8L18 13.1v.9h3v-1h-1.8l1.8-2.1V10h-3zM2 5h14v2H2zm0 12h14v2H2zm0-6h14v2H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADeUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/OseB9wLemv9etwI/A/w0/z6If52/Al6a/zk+B/hs/u0QL7rXBn6L/1luBR7Cvx3iRffawG/xP4/4t0O86F4b+C3+5xH/dogX3WsDv8X/PK/NC/c7vGCIF91rA7/F/z7iBUO86F4b+C3+9xEvGOJF99rAb/G/j3jBEC+61wZ+i/99xAuGeNG9NvBb/O8jXjDEi+61gd/ifx5xxWsDv8XzEi8Y4kX32sBv8T+PuOK1gd/ieYkXDPGie23gt/ifR1zx2sBv8bzEC4Z40b008Ff8z/I3wEtzxWsDv8XzEi8Y4l/ns4HP4n+GS8BrA3/NFa8N/BbPS7xgiH+9lwaO89/vr4Fdnu21gd/ieYkXDPF/x2sDv8XzEi8Y4v+O1wZ+i+clXjDE/x2vDfwWz0u8YIj/O14b+C2el3jBEP/5Xov/Gi8NfDXPS7xgiP985r+XeMEQ//nMfy/xgiH+85n/XuIFQ/znM/+9xAuG+M9n/nuJFwzxn8+8aH4HeG3+ayH+85kXze8Ar81/LcR/PvOi+R3gtfmvhfjPZ140vwO8Nv+1EP/5zIvmd4DX5r8W4j+fedH8DvDa/NdC/OczL5rfAV6b/1qI/3zmRfM7wGvzXwvxn8+8aH4HeG3+ayH+85kXze8Ar81/LcR/PvOi+R3gtfmvhfjPZ140vwO8Nv+1EP/5zIvmd4DX5r8W4j+fedH8DvDa/NdC/OczL5rfAV6b/1qI/3zmRfM7wGvzXwvxn8+8aH4HeG3+ayH+85kXze8Ar81/LcR/PvOi+R3gtfmvhfjPZ140vwO8Nv+1EP/5zIvmd4DX5r8W4j+fedH8DvDa/NdC/OczL5rfAV6b/1qI/3zmRfM7wGvzXwvxn28XOMa/7GuAj+a/FuI/32cDn8W/7GWAv+a/FuK/xlsDbw08mOf118B3A3/Nfz3E/2+I/98Q/78h/n9D/P/GPwJ6uWFBN26oxAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatPaint;
impl IconShape for MdFormatPaint {
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
                d: "M18 4V3c0-.55-.45-1-1-1H5c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6h1v4H9v11c0 .55.45 1 1 1h2c.55 0 1-.45 1-1v-9h8V4h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/mpYFj/Nv9DbDLczoOvBT/dpeAv+ZfB/Fv89vAa/Fv9zrAb/OcXhv4Lf7tfgd4bf51EP82vw28Fv92rwP8Ns/ptYHf4t/ud4DX5l8H8W/z28Br8W/3OsBv85xeG/gt/u1+B3ht/nUQ/za/DbwW/3avA/w2z+m1gd/i3+53gNfmXwfxb/PbwGvxb/c6wG/znF4b+C3+7X4HeG3+dRD/Nr8NvBb/dq8D/DbP6bWB3+Lf7neA1+ZfB/Fv89vAa/Fv9zrAb/OcXhv4Lf7tfgd4bf51EP82vw28Fv92rwP8Ns/ptYHf4t/ud4DX5l8H8W/z28Br8W/3OsBv85xeG/gt/u1+B3ht/nUQ/za/DbwW/3avA/w2z+m1gd/i3+53gNfmXwfxb/PbwGvxb/c6wG/znF4b+C3+7X4HeG3+dRD/Nr8NvBb/dq8D/DbP6bWB3+Lf7neA1+ZfB/Fv89vAa/Fv9zrAb/OcXhv4Lf7tfgd4bf51EP82vw28Fv92rwP8Ns/ptYHf4t/ud4DX5l8H8W/z28Br8W/3OsBv85xeG/gt/u1+B3ht/nUQ/zZfDbw0L9hLA8d4wV4H+G2e00sDX80L9mDgQbxgPwO8Nf86iP94Dwaezgv2DODB/Ov9FvDavGCvA/w2/zqI/3jfDbwXL9j7AN/Nv85rA7/FC/Y7wGvzr4f4j/Vg4Om8YM8AHsy/3m8Br80L9jrAb/Ovh/iP9d3Ae/GCvQ/w3fzrvDbwW7xgvwO8Nv82iP84Dwaezgv2DODB/Ov9FvDavGCvA/w2/zaI/zjfDbwXL9j7AN/Nv85rA7/FC/Y7wGvzb4f4j/Fg4Om8YM8AHsy/3m8Br80L9jrAb/Nvh/iP8d3Ae/GCvQ/w3fzrvDbwW7xgvwO8Nv8+iH+/BwNP5wV7BvBg/vV+C3htXrDXAX6bfx/Ev993A+/FC/Y+wHfzr/PawG/xgv0O8Nr8+yH+/V6bF+63+dd7MPBgXrBbgVv590P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EpXNvQWFwYnwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatQuote;
impl IconShape for MdFormatQuote {
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
                d: "M6 17h3l2-4V7H5v6h3zm8 0h3l2-4V7h-6v6h3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatShapes;
impl IconShape for MdFormatShapes {
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
                d: "M23 7V1h-6v2H7V1H1v6h2v10H1v6h6v-2h10v2h6v-6h-2V7h2zM3 3h2v2H3V3zm2 18H3v-2h2v2zm12-2H7v-2H5V7h2V5h10v2h2v10h-2v2zm4 2h-2v-2h2v2zM19 5V3h2v2h-2zm-5.27 9h-3.49l-.73 2H7.89l3.4-9h1.4l3.41 9h-1.63l-.74-2zm-3.04-1.26h2.61L12 8.91l-1.31 3.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP99r8T/DJeCveU6I/3zmf4bfAV6b54T4z2f+Z/gd4LV5Toj/fOZ/ht8BXpvnhPjPZ/5n+B3gtXlOiP985n+G3wFem+eE+M9n/mf4HeC1eU6I/3zmf4bfAV6b54T4z/fb/Ou9NHCMF+xvgF3+df4a+GieE+J/pt8GXosX7HWA3+bfD/E/028Dr8UL9jrAb/Pvh/if6beB1+IFex3gt/n3Q/zP9NvAa/GCvQ7w2/z7If5n+m3gtXjBXgf4bf79EP8y859LPK/fBl6LF+x1gN/m3w/xLzP/ucTz+m3gtXjBXgf4bf79EP8y859LPK/fBl6LF+x1gN/m3w/xLzP/ucTz+m3gtXjBXgf4bf79EP8y859LPK/fBl6LF+x1gN/m3w/xLzP/ucTz+m3gtXjBXgf4bf79EP8y859LPK/fBl6LF+x1gN/m3w/xLzP/ucTz+m3gtXjBXgf4bf79EP9+5oUT/3q/DbwWL9jrAL/Nvx/i38+8cOJf77eB1+IFex3gt/n3Q/z7mRdO/Ov9NvBavGCvA/w2/36Ifz/zwol/vd8GXosX7HWA3+bfD/HvZ1448a/328Br8YK9DvDb/Psh/v3MCyf+9X4beC1esNcBfpt/P8S/n3nhxL/ebwOvxQv2OsBv8++H+PczL5z41/tt4LV4wV4H+G3+/RD/fuaFE/96vw28Fi/Y6wC/zb8f4t/PvHDiX++3gdfiBXsd4Lf590P8+5kXTvzr/TbwWrxgrwP8Nv9+iH8/88KJf73fBl6LF+x1gN/m3w/x72deOPGv99vAa/GCvQ7w2/z7If79zAsn/vV+G3gtXrDXAX6bfz/Ev5954cS/3m8Dr8UL9jrAb/Pvh/j3My+c+Nf7beC1eMFeB/ht/v0Q/37mhRP/er8NvBYv2OsAv82/H+Lfz7xw4l/vt4HX4gV7HeC3+fdD/Pv9Ni/ca/Ov99XAS/OCfTTw1/z7If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+Ebl/b0G6N+cgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatSize;
impl IconShape for MdFormatSize {
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
                d: "M9 4v3h5v12h3V7h5V4H9zm-6 8h3v7h3v-7h3V9H3v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACx0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP8z3IJ+Gv+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5uvBl6af53jwEvxovkbYJd/nb8GPpp/HcR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/ndcGfosXzesAv81/PsR/nZcGvpoXzUcDf81/PsT/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+Je9Nv+7/TYvGOL/N8T/b4j/3xD/vyH+6xwHXooXzd8Au/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeW3gt3jRvA7w2/znQ/zXeWngq3nRfDTw1/znQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IwxAPEHHH4D+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatStrikethrough;
impl IconShape for MdFormatStrikethrough {
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
                d: "M10 19h4v-3h-4v3zM5 4v3h5v3h4V7h5V4H5zM3 14h18v-2H3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8Y4DbwU8GHgw8GD+430M8Nf8+yH+4zwY+CzgvfnP9zrAb/Pvh/iP8d7Ad/Ff53WA3+bfD/Hv913Ae/Nf63WA3+bfD/Hv897Ad/Ff73WA3+bfD/Fv92Dg6fzb/A2wy3M6DrwUz+t3eF4fDfw1/36If7vvBt6Lf5vXAX6b5/TawG/xvMR/HsS/zXHgIv92rwP8Ns/ptYHf4nmJ/zyIf5v3Br6Lf7vXAX6b5/TawG/xvMR/HsS/zWcDn8W/3esAv81zem3gt3he4j8P4t/mu4H34t/udYDf5jm9NvBbPC/xnwfxb/PdwHvxb/c6wG/znF4b+C2el/jPg/i3+Wzgs/i3ex3gt3lOrw38Fs9L/OdB/Nu8N/Bd/Nu9DvDbPKfXBn6L5yX+8yD+bY4DF/m3ex3gt3lOrw38Fs9L/OdB/Nt9N/Be/Nu8DvDbPKfXBn6L5yX+8yD+7R4MPJ1/m9cBfpvn9NrAb/G8xH8exL/PewPfxb/e6wC/zXN6beC3eF7iPw/i3++7gffiX+d1gN/mOb028Fs8L/GfB/Ef472B7+JF9zrAb/OcXhv4LZ6X+M+D+I/zYOCzgffiX/Y6wG/znF4b+C2el/jPg/iPdxx4a+DBwIOB9+J5vQ7w2zyn1wZ+i+cl/vMg/vOZ5/U6wG/znF4b+C2el/jPg/jPZ57X6wC/zXN6beC3eF7iPw/iP595Xq8D/DbP6bWB3+J5if88iP985nm9DvDbPKfXBn6L5yX+8yD+85nn9TrAb/OcXhv4LZ6X+M+D+M9nntfrAL/Nc3pt4Ld4XuI/D+I/n3lerwP8Ns/ptYHf4nmJ/zyI/3zmeb0O8Ns8p9cGfovnJf7zIP7zmef1OsBv85xeG/gtnpf4z4P4z2ee1+sAv81zem3gt3he4j8P4j+feV6vA/w2z+m1gd/ieYnn9d7Ad/Pvh/jPZ57X6wC/zXN6beC3eF7ief028HTgffj3QfznM8/rdYDf5jm9NvBbPC/xvH4beC3gu4H34d8O8b/TbwOvxRXfDbwP/zaI/51+G3gtnu27gffhXw/xv9NvA6/Fc/pu4H3410H87/TbwGvxvL4beB9edIj/nX4beC2ev+8G3ocXDeLf5quBl+K/z0sDx3nBvht4H/5liH+b3wZei//Zvht4H144xL/NbwOvxf983w28Dy8Y4t/mt4HX4n++9wG+mxcM8W/z28Br8T/b+wDfzQuH+Lf5auCl+e/z0sAxXrD3Ab6bfxnif6ffBl6L5+99gO/mRYP43+m3gdfieb0P8N286BD/O/028Fo8p/cBvpt/HcT/Tr8NvBbP9j7Ad/Ovh/jf6beB1+KK9wG+m38bxP9Ovw28FvA+wHfzb4f43+m3ge8Gvpt/H8T/Ti8N/DX/foj/3xD/vyH+f0P8/4b4/41/BC1ikEE5lfwfAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatTextdirectionLToR;
impl IconShape for MdFormatTextdirectionLToR {
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
                d: "M9 10v5h2V4h2v11h2V4h2V2H9C6.79 2 5 3.79 5 6s1.79 4 4 4zm12 8l-4-4v3H5v2h12v3l4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/uR4MvBVwHHhp4Dj/8V6HfzvEf47XBj4LeG3+84l/O8R/rOPAdwFvzX8d8W+H+I9zHPgt4KX5ryX+7RD/cX4LeG3+64l/O8R/jI8Gvop/m9/heb00cIzndAn4a57Xa/Nvh/j3Ow48HTjOv414Xr8NvBbP6XeA1+Y/FuLf772B7+LfTjyv3wZei+f0O8Br8x8L8e/308Bb8W8nntdvA6/Fc/od4LX5j4X49/sr4KX5txPP67eB1+I5/Q7w2vzHQvz7mX8f8bx+G3gtntPvAK/NfyzEv5/59xHP67eB1+I5/Q7w2vzHQvz7/TXwUvzbief128Br8Zx+B3ht/mMh/v1+Gngr/u3E8/pt4LV4Tr8DvDb/sRD/fu8NfBf/duJ5/TbwWjyn3wFem/9YiH+/48CtwDH+bcTz+m3gtXhOvwO8Nv+xEP8xPhr4Kv5txPP6beC1eE6/A7w2/7EQ/3F+G3gt/vXE8/pt4LV4Tr8DvDb/sRD/cY4Dvw28FP864nn9NvBaPKffAV6b/1iI/1jHga8G3osXnXhevw28Fs/pd4DX5j8W4j/HawOfDbwW/zLxvH4beC2e0+8Ar81/LMR/rgcDbw0cB14beC2el3hevw28Fs/pd4DX5j8W4r/OawO/xfMSz+u3gdfiOf0O8Nr8x0L813lt4Ld4XuJ5/TbwWjyn3wFem/9YiP86rw38Fs9LPK/fBl6L5/Q7wGvzHwvxX+e1gd/ieYnn9dvAa/Gcfgd4bf5jIf7rvDbwWzwv8bx+G3gtntPvAK/NfyzEf53XBn6L5yWe128Dr8Vz+h3gtfmPhfiv89rAb/G8xPP6beC1eE6/A7w2/7EQ/3VeG/gtnpd4Xr8NvBbP6XeA1+Y/FuK/zmsDv8XzEs/rt4HX4jn9DvDa/MdC/Nd5beC3eF7ief028Fo8p98BXpv/WIj/eMeB48CtPKfXBn6L5yWe128Dr8Vz+h3gtfmPhfiPdRz4LeBjgN/mOb028Fs8L/G8fht4LZ7T7wCvzX8sxH+c48BvAS8NvA7w2zyn1wZ+i+clntdvA6/Fc/od4LX5j4X4j3Ec+C3gpbnidYDf5n8+xL/fceC3gJfm2V4H+G3+50P8+xwHfgt4aZ7T6wC/zf98iH+748BvAS/N83od4Lf5nw/xb3Mc+C3gpXn+Xgf4bf7nQ/zrHQd+C3hpXrC/Bnb57/MxwF/zL0P86xwHfgt4af5nex3gt/mXIV50x4HfAl6a//leB/ht/mWIF91x4LeBl+J/vtcBfpt/GeJf5zjw28BL8T/b6wC/zb8M8a93HPht4KV4wf4G2OW/z0cDf82/DPFvcxz4beCleP5eB/ht/udD/NsdB34beCme1+sAv83/fIh/n+PAbwMvxXN6HeC3+Z8P8e93HPht4KV4ttcBfpv/+RD/MY4Dvw28FFe8DvDb/M+H+I9zHPht4KWA1wF+m//5EP+xjgO/DXw08Nv8z4f4j3ccOA7cyv98iP/fEP+/If5/Q/z/hvj/jX8EnV2OQbbe99AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatTextdirectionRToL;
impl IconShape for MdFormatTextdirectionRToL {
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
                d: "M10 10v5h2V4h2v11h2V4h2V2h-8C7.79 2 6 3.79 6 6s1.79 4 4 4zm-2 7v-3l-4 4 4 4v-3h12v-2H8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49zPP3+8Ar81/jN8GXovnT/zbIf79zPP3O8Br8x/jt4HX4vkT/3aIfz/z/P0O8Nr8x/ht4LV4/sS/HeLfzzx/vwO8Nv8xfht4LZ4/8W+H+Pczz9/vAK/Nf4zfBl6L50/82yH+/czz9zvAa/Mf47eB1+L5E/92iH8/8/z9DvDa/Mf4beC1eP7Evx3i3888f78DvDb/MX4beC2eP/Fvh/j3M8/f7wCvzX+M3wZei+dP/Nsh/v3M8/c7wGvzH+O3gdfi+RP/doh/P/P8/Q7w2vzH+G3gtXj+xL8d4t/PPH+/A7w2/zF+G3gtnj/xb4f49zPP3+8Ar81/jN8GXovnT/zbIf79zPP3O8Br8x/jt4HX4vkT/3aIfz/z/P0O8Nr8x/ht4LV4/sS/HeLfzzx/vwO8Nv8xfht4LZ4/8W+H+Pczz9/vAK/Nf4zfBl6L50/82yH+/czz9zvAa/Mf47eB1+L5E/92iH8/8/z9DvDa/Mf4beC1eP7Evx3i3888f78DvDb/MX4beC2eP/Fvh/j3M8/f7wCvzX+M3wZei+dP/Nsh/v3M8/c7wGvzH+O3gdfi+RP/doh/v1uBB/G8fgd4bf5j/DbwWjyvZwAP5t8O8e/328Br8bx2gRP8x7gIHOd5/Q7w2vzbIf79fhp4K56/1wF+m3+ftwZ+iufvZ4C35t8O8e/33sB38fz9NPA2/Pv8FvDaPH/vA3w3/3aIf7/jwEVesNcBfpt/m9cGfosX7ASwy78d4j/GbwOvxfO3C7wMcCv/Og8G/go4zvP3M8Bb8++D+I/x2sBv8YLtAq8D/DUvmpcGfgs4zgv2OsBv8++D+I/z08Bb8cJ9NvA1wC7P33Hgo4DP5oX7GeCt+fdD/Md5MPDXwDH+ZT8N/DVwK1c8GHhp4K35l10CXhq4lX8/xH+s1wZ+i/9crwP8Nv8xEP/x3hv4Lv5zvA/w3fzHQfzneGvgu4Fj/Me4BLw38NP8x0L853lp4KeBB/Hv8wzgrYG/5j8e4j/fewOfDTyIf51nAJ8NfDf/eRD/dd4aeGvgrYFjPH+XgJ8Gfhr4af7zIf77vDbP6bf5r4f4/w3x/xvi/zfE/2+If5uXBo7xP8sl4K/510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CM7NolBIeQw/wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFormatUnderlined;
impl IconShape for MdFormatUnderlined {
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
                d: "M12 17c3.31 0 6-2.69 6-6V3h-2.5v8c0 1.93-1.57 3.5-3.5 3.5S8.5 12.93 8.5 11V3H6v8c0 3.31 2.69 6 6 6zm-7 2v2h14v-2H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3++1+O/1O/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pd7b+C7+O8j/u0Q/zHeG/gu/nuIfzvEf5z3Br6LF80zgFv5j/Ha/Nsh/mO9NPDbwDFeuF3gdYC/5r8X4j/eSwO/DRzjhdsFXgf4a/77IP5zvDTw28AxXrhd4HWAv+a/B+I/z3Hgt4GX4oXbBV4H+Gv+6yH+cx0Hfht4KV64XeB1gL/mvxbiP99x4LeBl+KF2wVeB/hr/usg/ut8N/BevHC7wOsAf81/DcR/re8G3osXbhd4HeCv+c+H+K/33cB78cLtAq8D/DX/uRD/Pd4b+C5euF3gdYC/5j8P4r/PewPfxQu3C7wO8Nf850D893pv4Lt44XaB1wH+mv94iP9+Lw38NnCMF2wXeB3gr/mPhfjv99LAbwHHecEuAa8N/DX/sRD/vd4b+C5euEvAawN/zX88xH+f9wa+ixfuEvDawF/znwPx3+O9ge/ihbsEvDbw1/znQfzX+y7gvXnhLgGvDfw1/7kQ/7W+C3hvXrhLwGsDf81/PsR/ne8C3psX7hLw2sBf818D8Z/vOPBbwEvzwl0CXhv4a/7rIP5zHQd+C3hpXrhLwGsDf81/LcR/nuPAbwEvzQt3CXht4K/5r4f4z/HSwG8Bx3nhLgGvDfw1/z0Q//FeGvgt4Dgv3CXgtYG/5r8P4j/WSwO/BRznhbsEvDbw1/z3QvzHeW/gu3jR3Arcyn+M1+HfDvEf472B7+K/h/i3Q/z7vTfwXfz3Ef92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v1em/9ev82/HeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I/WSZEFTszD5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFunctions;
impl IconShape for MdFunctions {
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
                d: "M18 4H6v2l6.5 6L6 18v2h12v-3h-7l5-5-5-5h7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv8d3Ae/Nc/pu4H34r4X4r/ddwHvz/H038D7810H81/ou4L154b4beB/+ayD+63wX8N68aL4beB/+8yH+a3wX8N7863w38D7850L85/su4L35t/lu4H34z4P4z/VdwHvz7/PdwPvwnwPxn+e7gPfmP8Z3A+/DfzzEf473Bt6bF+ylgWM8p0vAX/OCfTfw3fzHQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4/fBl6L5/Q7wGvzXwvx3+O3gdfiOf0O8Nr810L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+O9wbeixfspYHjPKdd4K95wb4H+G7+YyH+83w38F78x/ge4L35j4f4z/XdwHvx7/M9wHvznwPxn++7gffi3+Z7gPfmPw/iv8Z3A+/Fv873AO/Nfy7Ef53vBt6LF833AO/Nfz7Ef63vBt6LF+57gPfmvwbiv953A+/F8/c9wHvzXwfx3+O7gffiOX0P8N7810L89/lu4L244mOAr+a/HuK/13cDvw18N/89EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHieRjQVqfSKoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHeight;
impl IconShape for MdHeight {
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
                points: "13,6.99 16,6.99 12,3 8,6.99 11,6.99 11,17.01 8,17.01 12,21 16,17.01 13,17.01",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADw0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Pb4aeCme098AH81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Ou9N/BawPvwb/fbwGvxnH4HeG3+7b4L+B3gu3nRIf513hv4Lq74buB9+Ld5aeA4z2kX+Gv+bb4LeG+ueB/gu3nRIF507w18F8/pu4H34b/XdwHvzXN6H+C7+ZchXjRvDfwUz993A+/Df4/vAt6b5+9tgJ/mhUO8aI4Dvw28FM/fdwPvw3+t7wLem+fvb4DXBnZ54RAvuuPAbwMvxfP3OcBn81/js4HP4vn7G+C1gV3+ZYh/nePAbwMvxXP6G+C1gV3+axwHfht4KZ7T3wCvDezyokH86x0Hfht4Ka74G+C1gV3+ax0Hfht4Ka74G+C1gV1edIh/m+PAb3PFawO7/Pc4Dvw2V7w2sMu/DuLf7jhX7PLf6zhX7PKvh/j/DfH/G+Lfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH+/vwZeiv8efwO8NP92iH+/48BvAy/Ff62/AV4b2OXfDvEf4zjw28BL8V/jb4DXBnb590H8xzkO/DbwUvzn+hvgtYFd/v0Q/7GOA78NvBT/Of4GeG1gl/8YiP94x4HfBl6K/1h/A7w2sMt/HMR/juPAbwMvxX+MvwFeG9jlPxbiP89x4LeBl+Lf52+A1wZ2+Y+H+M91HLjIv88JYJf/HIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M/32/z7vDb/eRD/vyH+f0P8/4b4/w3x/xv/CDYccEEusZviAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHighlight;
impl IconShape for MdHighlight {
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
                d: "M6,14l3,3v5h6v-5l3-3V9H6V14z M11,2h2v3h-2V2z M3.5,5.88l1.41-1.41l2.12,2.12L5.62,8L3.5,5.88z M16.96,6.59l2.12-2.12 l1.41,1.41L18.38,8L16.96,6.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABr0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xAvupcGvor/HT4G+Gv+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTTw1fzv8NHAX/MvQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjcR8MQcnrRNkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHorizontalRule;
impl IconShape for MdHorizontalRule {
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
                fill_rule: "evenodd",
                height: "2",
                width: "16",
                x: "4",
                y: "11",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzH+xtgl+d0HHgp/uOJFwzxLzP/8V4H+G2e02sDv8V/PPGCIf5l5j/e6wC/zXN6beC3+I8nXjDEv8z8x3sd4Ld5Tq8N/Bb/8cQLhviXmf94rwP8Ns/ptYHf4j+eeMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mXnRXAL+muf1Wjyv1wF+m+f02sBv8R9PvGCIf5l50fwO8No8L/O8Xgf4bZ7TawO/xX888YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fv/xxAuG+JeZF83vAK/N8zLP63WA3+Y5vTbwW/zHEy8Y4l9mXjS/A7w2z8s8r9cBfpvn9NrAb/EfT7xgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8V/PPGCIf5l5kXzO8Br87zM83od4Ld5Tq8N/Bb/8cQLhviXmRfN7wCvzfMyz+t1gN/mOb028Fv8xxMvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvH6H5/XSwDFeNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi+R3gtXle5nm9DvDbPKfXBn6L5yWe128Dr8WLRrxgiH+ZedH8DvDaPC/zvF4H+G2e02sDv8XzEs/rt4HX4kUjXjDEv8y8aH4HeG2el3lerwP8Ns/ptYHf4nmJ5/XbwGvxohEvGOJfZl40vwO8Ns/LPK/XAX6b5/TawG/xvMTz+m3gtXjRiBcM8S8zL5rfAV6b52We1+sAv81zem3gt3he4nn9NvBavGjEC4b4l5kXze8Ar83zMs/rdYDf5jm9NvBbPC/xvH4beC1eNOIFQ/zLzIvmd4DX5nmZ5/U6wG/znF4b+C2el3hevw28Fi8a8YIh/mXmRfM7wGvzvMzzeh3gt3lOrw38Fs9LPK/fBl6LF414wRD/MvOi2QX+muf12jyvvwZ2eU7HgZfmef02z+ulgeO8aMQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj3ZSqQTTQwmcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertChart;
impl IconShape for MdInsertChart {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertChartOutlined;
impl IconShape for MdInsertChartOutlined {
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
                d: "M9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4zm2.5 2.1h-15V5h15v14.1zm0-16.1h-15c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5WeAt+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fq8aH6b/16vzYvmd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXosXjfjvZV40vwO8Ns8J8fz9NvBavGjEfy/zovkd4LV5Tojn77eB1+JFI/57mRfN7wCvzXNCPH+/DbwWLxrx38u8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi+a3+e/12rxofgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LV40Yj/XuZF8zvAa/OcEM/fbwOvxYtG/PcyL5rfAV6b54R4/n4beC1eNOK/l3nR/A7w2jwnxPP328Br8aIR/73Mi+Z3gNfmOSGev98GXosXjfjvZV40vwO8Ns8J8fz9NvBavGh+m/9er82L5neA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+2zgs/i/5XeA1+Y5IZ6/twZ+iv9bfgd4bZ4T4gX7a+Cl+L/jd4DX5jkhXrAHAz8NvBT/N/wO8No8J8QLdxz4aOC9gQfxv9vvAK/Nc0L87/fewHfxL/sd4LV5Toj/G94b+C5euN8BXpvnhPi/472B7+IF+x3gtXlOiP9b3hv4Lp6/3wFem+eE+L/nvYHv4nn9DvDaPCfE/03vDXwXz+l3gNfmOSH+73pv4Lt4tt8BXpvnhPi/7b2B7+KK3wFem+eE+L/vvYHvAn4HeG2eE+L/h/cG3ht4bZ4T4v+Plwb+mueE+P8N8f8b4v83xP9viP/f+EeUYX5BM6XkLQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertComment;
impl IconShape for MdInsertComment {
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEsUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/eZ4BHAeO8by+G3gf/uMhXnSvDfwW/3l+B/ho4LeBYzyv7wbeh/9YiBfdawO/xX+e3wFeG3hp4LeBYzyv7wbeh/84iBfdawO/xX+e3wFemyteGvht4BjP67uB9+E/BuJF99rAb/Gf53eA1+bZXhr4beAYz+u7gffh3w/xontt4Lf4z/M7wGvznF4a+G3gGM/ru4H34d8H8aJ7beC3+M/zO8Br87xeGvht4BjP67uB9+HfDvGie23gt/jP8zvAa/P8vTTw28Axntd3A+/Dvw3iRffawG/xn+d3gNfmBXtp4LeBYzyv7wbeh389xIvutYHf4j/P7wCvzQv30sBvA8d4Xt8NvA//OogX3WsDv8V/nt8BXpt/2UsDvw0c43l9N/A+vOgQL7rXBn6L/zy/A7w2L5qXBn4bOMbz+m7gfXjRIF50rw38Fv95fgd4bV50Lw38NnCM5/U6wG/zL0O86F4b+C3+8/wO8Nr867w08NvAMZ7T6wC/zb8M8aJ7beC3+M/zO8Br86/30sBvA8d4ttcBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdSwN/xf8OLwP8Nf8yxL/OrcCD+J/tGcCDedEg/nXeGvgp/md7HeC3edEg/vXeG/hq4Bj/s1wC3hv4aV50iH+b48B7Ay8NPJj/XrcCfw18N7DLvw7i/zfE/2+I/98Q/78h/n/jHwFGEIxBrSiODwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertDriveFile;
impl IconShape for MdInsertDriveFile {
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
                d: "M6 2c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6H6zm7 7V3.5L18.5 9H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertEmoticon;
impl IconShape for MdInsertEmoticon {
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Y9h/vVeB/htntNrA7/F8xIvGOJfZp7X6wC/zX8M86/3OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5jmH+91wF+m+f02sBv8bzEC4b4l5nn9TrAb/Mfw/zrvQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpv/GOZf73WA3+Y5vTbwWzwv8YIh/mXmeb0O8Nv8xzD/eq8D/DbP6bWB3+J5iRcM8S8zz+t1gN/mP4b513sd4Ld5Tq8N/BbPS7xgiH+ZeV6vA/w2/zHMv97rAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3+Y9h/vVeB/htntNrA7/F8xIvGOJfZp7X6wC/zX8M86/3OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5jmH+91wF+m+f02sBv8bzEC4b4l5nn9TrAb/Mfw/zrvQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpv/GOZf73WA3+Y5vTbwWzwv8YIh/mXmeb0O8Nv8x3ht/vX+GtjlOb028Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeX008Nf8z/LSwFfzvMQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8y14a+Cv+d3oIcCsvGOJF897Ad/G/y/sA380Lh3jRPRj4bOCtgWP8z3QJ+Gngs4Fb+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+vA6hBVV+wHQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertInvitation;
impl IconShape for MdInsertInvitation {
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
                d: "M17 12h-5v5h5v-5zM16 1v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2h-1V1h-2zm3 18H5V8h14v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4t/vwcCD+Nf5G2CX53QceCn+dZ4B3Mq/HeLf5rWB9wLeGjjOv97rAL/Nc3pt4Lf419sFfhr4HuC3+ddB/Ou8NvBZwGvz7/M6wG/znF4b+C3+fX4b+Bzgt3nRIF50HwV8Nf8xXgf4bZ7TawO/xX+Mjwa+hn8Z4kXzXcB78x/ndYDf5jm9NvBb/Mf5buB9eOEQ/7KvBj6KF+5vgF1edB8N/DXP6aWBr+ZFdxx4KV64zwE+mxcM8cK9NfBTPH/PAD4b+Glgl/8ex4G3Bj4beBDP39sAP83zh3jhng48mOf1PcB78z/LdwPvxfO6FXgIzx/iBfts4LN4Xl8DfDT/M3018FE8r88BPpvnhXjBng48mOf0O8Br8z/bbwOvxXP6a+BleF6I5+/BwNN5Xq8D/Db/s7028Fs8r4cAt/KcEM/fRwNfxXP6G+CledF8NfBS/Mf6G+CjedH8NfBSPKePAb6a54R4/j4b+Cye09cAH82L5reB1+I/1u8Ar82L5quBj+I5fQ7w2TwnxPP33cB78Zw+B/hsXjS/DbwW/7F+B3htXjSfDXwWz+lrgI/mOSGev+8G3ovn9DnAZ/Oi+W3gtfiP9TvAa/Oi+Wzgs3hOXwN8NM8J8fx9NvBZPKevAT6aF81vA6/Ff6zfAV6bF81XAx/Fc/oc4LN5Tojn76OBr+I5/TXwMrxovhp4af5j/TXw0bxong48mOf0McBX85wQz9+DgafzvF4H+G3+Z3tt4Ld4Xg8BbuU5IV6wvwZeiuf028Dr8D/bbwGvzXP6G+CleV6IF+yzgc/ieX0N8NH8z/TVwEfxvD4G+GqeF+KFuxV4EM/ru4H34X+W7wLem+f1DODBPH+IF+6tgZ/i+bsV+GzgZ4Bd/nscB94K+GzgwTx/bwP8NM8f4l/21cBH8cL9NbDLi+5jgL/mOb008FW86I4DL80L9znAZ/OCIV403w28F/9xXgf4bZ7TawO/xX+c7wHemxcO8aL7aOCr+I/xOsBv85xeG/gt/mN8DPDV/MsQ/zqvDXw28Fr8+7wO8Ns8p9cGfot/n98BPhv4bV40iH+b1wbeG3ht4EH8670O8Ns8p9cGfot/vWcAvw18N/Db/Osg/v2OAy/Nv85fA7s8p+PAS/Ov89fALv92iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjwYoikF2+AX2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertLink;
impl IconShape for MdInsertLink {
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z893gb4KuBB/HvI14wxL/M/Nf7HOCzgZcG/op/H/GCIf5l5r/W7wCvzbN9NPBV/NuJFwzxLzP/dZ4BvDSwy3P6aeCt+LcRLxjiX2b+bd4G+GzgpXjRvQzw1zyv48BfAw/iX0+8YIh/mfnX+xzgs4GXBn4bOMa/7H2A7+YFe2ngr/jXEy8Y4l9m/nV+Bnhrnu21gd/ihfse4L35l3008FX864gXDPEvMy+6ZwAvDezynN4b+C6ev78BXhvY5UXz08Bb8aITLxjiX2ZeNJeA1wb+mufvu4H34jldAl4auJUX3XHgr4EH8aIRLxjiX2ZeNO8DfDcv3G8Dr8WzvQ3w0/zrvTTwV7xoxAuG+JeZf9n3AO/Nv+w48NvASwGfA3w2/3YfDXwV/zLxgiH+ZeaF+xvgpXnRvTTw2cBb8+/308Bb8cKJFwzxLzMv2CXgpYFb+e9xHPhr4EG8YOIFQ/zLzAv2OsBv89/rpYG/4gUTLxjiX2aev88BPpv/GT4a+CqeP/GCIf5l5nn9DPDW/M/y08Bb8bzEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l+0Cx/jf6RJwnBcM8S/7buC9+N/pe4D35gVD/MseDDyd/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjyDdxQRvlagYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInsertPhoto;
impl IconShape for MdInsertPhoto {
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
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADHUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfGieWngtYCXBnaBXeB7gFv5t3kw8F7Aca7YBb4HuJV/mwcD7wUcB44Dfw38DvDXvHCIF+448FXAe/P8fTfwMcAuL5rjwFcB783z993AxwC7vGiOA18FvDfP33cDHwPs8vwhXrDjwG8BL80L99fA6wC7vHDHgd8CXpoX7q+B1wF2eeGOA78FvDQv3F8DrwPs8rwQL9h3A+/Fi+ZrgI/mhftq4KN40XwN8NG8cF8NfBQvmu8B3pvnhXj+Xhr4K/51fpsX7rX51/ltXrjX5l/nIcCtPCfE8/fRwFfxf8vHAF/Nc0I8f98NvBf/t3wN8NE8J8Tz993Ae/F/y9cAH81zQjx/Hw18Ff+3fAzw1TwnxPP30sBf8a/zO7xwr8W/zu/wwr0W/zoPAW7lOSFesO8G3osXzdcAH80L99XAR/Gi+Rrgo3nhvhr4KF403wO8N88L8YIdB34beCleuL8BXhvY5YU7Dvw28FK8cH8DvDawywt3HPht4KV44f4GeG1gl+eFeOGOA18NvBfP3/cAHw3s8qI5Dnw18F48f98DfDSwy4vmOPDVwHvx/H0P8NHALs8f4kXzYOCtgZcGdoFd4LuBW/m3eTDw3sBxrtgFvhu4lX+bBwPvDRwHjgN/Dfw0cCsvHOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4R1R9UQWpNSCMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLinearScale;
impl IconShape for MdLinearScale {
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
                d: "M19.5,9.5c-1.03,0-1.9,0.62-2.29,1.5h-2.92C13.9,10.12,13.03,9.5,12,9.5s-1.9,0.62-2.29,1.5H6.79 C6.4,10.12,5.53,9.5,4.5,9.5C3.12,9.5,2,10.62,2,12s1.12,2.5,2.5,2.5c1.03,0,1.9-0.62,2.29-1.5h2.92c0.39,0.88,1.26,1.5,2.29,1.5 s1.9-0.62,2.29-1.5h2.92c0.39,0.88,1.26,1.5,2.29,1.5c1.38,0,2.5-1.12,2.5-2.5S20.88,9.5,19.5,9.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ5/UxwF/zP8tLA1/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/NczoOvBTP63d4Xi8NHOM5XQL+muf1WjyvvwF2eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt3lOLw18Fc/rdXheXw28FM/pb4CP5nn9Fs/rY4C/5jm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i+clntdvA6/Fc/od4LV5XuZ5vQ7w2zyn1wZ+i+clXjDEv8w8r9cBfpvn9NrAb/G8xPP6beC1eE6/A7w2z8s8r9cBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C2el3hevw28Fs/pd4DX5nmZ5/U6wG/znF4b+C2el3jBEP8y87xeB/htntNrA7/F8xLP67eB1+I5/Q7w2jwv87xeB/htntNrA7/F8xIvGOJfZp7X6wC/zXN6aeCreV6vzfP6auCleU5/DXw0z+u3eV4fDfw1z+m1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zz+mjgr/mf5aWBr+Z5iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BLwHuEEhvHLEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMargin;
impl IconShape for MdMargin {
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
                d: "M3 3v18h18V3H3zm16 16H5V5h14v14zM11 7h2v2h-2zM7 7h2v2H7zm8 0h2v2h-2zm-8 4h2v2H7zm4 0h2v2h-2zm4 0h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+ulueKv+e+B+O/z0sBvccXrAH/Nfz3Ef4+XBn4LOM4Vu8DrAH/Nfy3Ef72XBn4LOM5z2gVeB/hr/usg/mu9NPBbwHGev13gdYC/5r8G4r/OSwO/BRznhdsFXgf4a/7zIf5rvDTwW8BxXjS7wOsAf81/LsR/vpcGfgs4zr/OLvA6wF/znwfxn+ulgd8CjvNvswu8DvDX/OdA/Od5aeC3gOP8++wCrwP8Nf/xEP85Xhr4LeA4/zF2gdcB/pr/WIj/eC8N/BZwnP9Yu8DrAH/NfxzEf7yXBo7zwn018FI8p78BPpoXbhf4a/7jIP57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/eceCvgAfznH4HeG3+ayH+ax0Hfgt4aZ7X7wCvzX8txH+d48BvAS/N8/c1wEfzXwvxX+M48FvAS/P8fQ/w3vzXQ/znOw78FvDSPH/fA7w3/z0Q/7mOA78FvDTP3/cA780LdpwrdvnPgfjPcxz4LeClef6+B3hvXrDjwG9xxesAu/zHQ/znOA78FvDSPH/fA7w3L9hx4LeAl+aKvwZeB9jlPxbiP95x4LeAl+b5+x7gvXnBjgO/Bbw0z+mvgdcBdvmPg/iPdRz4LeClef6+B3hvXrjPBj6L5++vgdcBdvmPgfiPcxz4LeClef6+B3hvXjTfDbwXz99fA68D7PLvh/iPcRz4LeClef6+B3hv/nW+G3gvnr+/Bl4H2OXfB/Hvdxz4LeClef6+B3hv/m2+G3gvnr+/Bl4H2OXfDvHvcxz4LeClef6+B3hv/n2+G3gvnr+/Bl4H2OXfBvHv81fAS/P8fQ/w3vzH+G7gvXj+/hp4Gf5tEP8+7w18F8/re4D35j/WdwPvxfN6H+C7+bdB/Pu9N/BdPNv3AO/Nf47vBt6LZ3sf4Lv5t0P8x3hv4LuA7wHem/9c3w28F/A+wHfz74P4j/PawG/zX+O1gd/m3w/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wj6NYdBgNa3aQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMergeType;
impl IconShape for MdMergeType {
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
                d: "M17 20.41L18.41 19 15 15.59 13.59 17 17 20.41zM7.5 8H11v5.59L5.59 19 7 20.41l6-6V8h3.5L12 3.5 7.5 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5XuA9+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/PBj6L/1t+B3htnhPi+Xtr4Kf4v+V3gNfmOSFesL8GXor/O34HeG2eE+IFezDw08BL8X/D7wCvzXNCvHDHgY8G3ht4EP+7/Q7w2jwnxP9+7w18F/+y3wFem+eE+L/hvYHv4oX7HeC1eU6I/zveG/guXrDfAV6b54T4v+W9ge/i+fsd4LV5Toj/e94b+C6e1+8Ar81zQvzf9N7Ad/Gcfgd4bZ4T4v+u9wa+i2f7HeC1eU6I/9veG/gurvgd4LV5Toj/+94b+C7gd4DX5jkh/n94b+C9gdfmOSH+/3hp4K95Toj/3xD/vyH+f0P8/4b4/41/BGqqbEGay/l6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdModeComment;
impl IconShape for MdModeComment {
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
                d: "M21.99 4c0-1.1-.89-2-1.99-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+mjgI8GjgPfDXwM/zaI/32+C3hvntN3A+/Dvx7if5fvAt6b5++7gffhXwfxv8d3Ae/NC/fdwPvwokP87/BdwHvzovlu4H140SD+5/ts4LP41/lu4H34lyH+5zsO/DbwUvzrfDfwPrxwiP8djgO/DbwU/zpfA3w0Lxjif6bXBn6b53Qc+G3gpXjR7QIneMEQ//N8F/DewPsA381zOg78NvBSvGieATyYFwzxP8t3Ae/Ns70P8N08p+PAbwMvxb/sfYDv5gVD/M/xXcB787zeB/huntNx4LeBl+IFex/gu3nhEP8zfBfw3rxg7wN8N8/pOPDbwEvxvN4H+G7+ZYj/ft8FvDf/svcBvpvndBz4beCleLb3Ab6bFw3iv9d3Ae/Ni+59gO/mOR0Hfht4KeB9gO/mRYf47/NdwHvzr/c+wHfznI4Drw38NP86iP8e3wW8N/927wN8N/9+iP963wW8N/9+7wN8N/8+iP9a3wW8N/9xHgLcyr8d4r/OdwHvzX+c9wG+m38fxH+N7wLem/847wN8N/9+iP983wW8N/9x3gf4bv5jIP5zfRfw3vzHeR/gu/mPg/jP813Ae/Mf532A7+Y/FuI/x3cB781/nPcBvpv/eIj/eN8FvDf/cd4H+G7+cyD+Y30X8N78x3kf4Lv5z4P4j/NdwHvzH+d9gO/mPxfiP8Z3Ae/Nf5z3Ab6b/3yIf7/vAt6b/zjvA3w3/zUQ/z7fBbw3/3HeB/hu/usg/u2+C3hv/uO8D/Dd/NdC/Nt8F/De/Md5H+C7+a+H+Nf7LuC9+Y/zPsB3898D8a/zXcB78x/nfYDv5r8P4kX3XcB78x/nfYDv5r8X4kXzXcB78x/nfYDv5r8f4l/23sB38R/nfYDv5n8GxL/M/Md5H+C7+Z8D8S8z/zHeB/hu/mdB/MvMv9/7AN/N/zyIf5n593kf4Lv5nwnxLzP/du8DfDf/cyH+Zebf5n2A7+Z/NsS/zPzrvQ/w3fzPh/iXmX+d9wG+m/8dEP8y86J7H+C7+d8D8S8zL5rvAd6b/10Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcMCG5BedOXfwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdModeEdit;
impl IconShape for MdModeEdit {
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

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMonetizationOn;
impl IconShape for MdMonetizationOn {
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1.41 16.09V20h-2.67v-1.93c-1.71-.36-3.16-1.46-3.27-3.4h1.96c.1 1.05.82 1.87 2.65 1.87 1.96 0 2.4-.98 2.4-1.59 0-.83-.44-1.61-2.67-2.14-2.48-.6-4.18-1.62-4.18-3.67 0-1.72 1.39-2.84 3.11-3.21V4h2.67v1.95c1.86.45 2.79 1.86 2.85 3.39H14.3c-.05-1.11-.64-1.87-2.22-1.87-1.5 0-2.4.68-2.4 1.64 0 .84.65 1.39 2.67 1.91s4.18 1.39 4.18 3.91c-.01 1.83-1.38 2.83-3.12 3.16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HbwGvxgr0O8Nv850P89/ht4LV4wV4H+G3+8yH+e/w28Fq8YK8D/Db/+RD/Nu8NfBTwOsAu/3q/DbwWL9jrAL/Nfz7Ev957A9/FFX8NvA6wy7/ObwOvxQv2OsBv858P8a/z3sB38Zz+GngdYJcX3V8BL80L9jbAT/OfD/Gv89fAS/G8/hp4HWCXF+ytgPcG3poXzV8DXw18D/95EP86x4HfBl6K5/XXwOsAuzyvtwZ+in+bW4H3AX6b/3iIf73jwG8DL8Xz+mvgdYBdntd7A9/Fv937AN/NfyzEv81x4LeBl+J5/TXwOsAuz+u9ge/i3+59gO/mPw7i3+448NvAS/G8/hp4HWCX5/XewHcBfwP8NM/pwcBrAw/iBXsZ4K/5j4H49zkO/DbwUjyvvwZeB9jlX++9ga8GjvG8fht4Hf5jIP79jgO/DbwUz+uvgdcBdvnXe2ngt4FjPK+XAf6afz/Ef4zjwG8DL8Xz+mvgdYBd/vW+GvgontfnAJ/Nvx/iP85x4LeBl+J5/TXwOsAu/zqvDfwWz+t3gNfm3w/xH+s4cJHn76+B1wF2+dcxz+uvgZfh3w/xH28XOMbz99fA6wC7/M+A+I/328Br8YL9NfA6wC7//RD/8T4a+CpeuL8GXgfY5b8X4j/eceBW4Bgv3F8DrwPs8t8H8Z/jo4Gv4l/218DrALv890D85/lu4L34l/018DrALv/1EP+5Phv4LP5lfw28DrDLfy3Ef77XBj4beC1euL8GXgfY5b8O4r/OewPvDbwWL9hfA68D7PJfA/Ff78HAewPvDTyI5/XXwOsAu/znQ/z3OQ78NvBSPK+/Bl4H2OU/F+K/13Hgt4GX4nn9NfA6wC7/eRD//Y4Dvw28FM/rr4HXAXb5z4H4n+E48NvAS/G8/hp4HWCX/3iI/zmOA78NvBTP66+B1wF2+Y+F+J/lOPDbwEvxvP4aeB1gl/84iP9YHw18Fc/rc4DP5kVzHPht4KV4Xn8NvA6wy38MxH+s1wZ+i+f1M8Bb86I7Dvw28FI8r78GXgfY5d8P8R/rOHCR5+9lgL/mRXcc+G3gpXhefw28DrDLvw/iP95PA2/F8/pr4HWAXV503wW8N8/fXwOvA+zyb4f4j/fSwF/x/O0CHw18Dy/cg4GvAt6aF+6vgdcBdvm3Qfzn+Grgo3jBdoHfBv6a53QceG3gpXnR/TXwOsAu/3qI/zw/DbwV/zX+GngdYJd/HcR/ru8G3ov/OF8DvDbwUjyvvwZeB9jlRYf4z/fawHcDD+Lf7hnARwM/DRwHfht4KZ7XXwOvA+zyokH813lr4L2Bt+JF9zPAdwM/zXM6Dvw28FI8r78GXoYXDeK/x18DL8UL9jHAV/PCHQd+G3gpntP7AN/Niwbx3+O3gdfiBXsd4Lf5lx0Hfht4Ka54H+C7edEh/nv8NvBavGCvA/w2L5rjwG8DXw18N/86iP8evw28Fi/Y6wC/zX8+xH+P3wZeixfsdYDf5j8f4r/HbwOvxQv2OsBv858P8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EN/m/QWwP0HIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoneyOff;
impl IconShape for MdMoneyOff {
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
                d: "M12.5 6.9c1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-.53.12-1.03.3-1.48.54l1.47 1.47c.41-.17.91-.27 1.51-.27zM5.33 4.06L4.06 5.33 7.5 8.77c0 2.08 1.56 3.21 3.91 3.91l3.51 3.51c-.34.48-1.05.91-2.42.91-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c.96-.18 1.82-.55 2.45-1.12l2.22 2.22 1.27-1.27L5.33 4.06z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMultilineChart;
impl IconShape for MdMultilineChart {
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
                d: "M22 6.92l-1.41-1.41-2.85 3.21C15.68 6.4 12.83 5 9.61 5 6.72 5 4.07 6.16 2 8l1.42 1.42C5.12 7.93 7.27 7 9.61 7c2.74 0 5.09 1.26 6.77 3.24l-2.88 3.24-4-4L2 16.99l1.5 1.5 6-6.01 4 4 4.05-4.55c.75 1.35 1.25 2.9 1.44 4.55H21c-.22-2.3-.95-4.39-2.04-6.14L22 6.92z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABy0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXvTb/u/02Lxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8S/7Lf53ex1eMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/st/nf7bV5wRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If9lr8d/rd/jPg/iXmf9e4j8P4l9m/nuJ/zyIf5n57yX+8yD+Zea/l/jPg/iXmf9e4j8P4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CPGBEkFBKv/CAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotes;
impl IconShape for MdNotes {
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
                d: "M3 18h12v-2H3v2zM3 6v2h18V6H3zm0 7h18v-2H3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ5/UxwF/zP8tLA1/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/NczoOvBTP63d4Xi8NHOM5XQL+muf1WjyvvwF2eU6vDfwWz0u8YIh/mXlerwP8Ns/ptYHf4nmJ5/XbwGvxnH4HeG2el3lerwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gtntPvAK/N8zLP63WA3+Y5vTbwWzwv8YIh/mXmeb0O8Ns8p9cGfovnJZ7XbwOvxXP6HeC1eV7meb0O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtXhOvwO8Ns/LPK/XAX6b5/TawG/xvMQLhviXmef1OsBv85xeG/gtnpd4Xr8NvBbP6XeA1+Z5mef1OsBv85xeG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/ro4G/5n+Wlwa+muclXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RDcGaQehxH6kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPadding;
impl IconShape for MdPadding {
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
                d: "M3 3v18h18V3H3zm16 16H5V5h14v14zM11 7h2v2h-2zM7 7h2v2H7zm8 0h2v2h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPieChart;
impl IconShape for MdPieChart {
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
                d: "M11 2v20c-5.07-.5-9-4.79-9-10s3.93-9.5 9-10zm2.03 0v8.99H22c-.47-4.74-4.24-8.52-8.97-8.99zm0 11.01V22c4.74-.47 8.5-4.25 8.97-8.99h-8.97z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPieChartOutlined;
impl IconShape for MdPieChartOutlined {
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
                d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm1 2.07c3.61.45 6.48 3.33 6.93 6.93H13V4.07zM4 12c0-4.06 3.07-7.44 7-7.93v15.87c-3.93-.5-7-3.88-7-7.94zm9 7.93V13h6.93c-.45 3.61-3.32 6.48-6.93 6.93z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O3018FI8p78BPpp/HcT/Tr8NvBbP6XeA1+ZfB/G/028Dr8Vz+h3gtfnXQfzv9NvAa/Gcfgd4bf51EP87/TbwWjyn3wFem38dxP9Ovw28Fs/pd4DX5l8H8b/TbwOvxXP6HeC1+ddB/O/028Br8Zx+B3ht/nUQ/zpvBbw08Nr8x3kdntdXAy/FC/bSwHGe0y7w17xgfwN8NM8J8aJ5MPBTwEvzH088r98GXov/WL8DvDbPCfEvOw78FfBg/nOI5/XbwGvxH+t3gNfmOSH+ZZ8NfBb/ecTz+m3gtfiP9TvAa/OcEP+ypwMP5j+PeF6/DbwW/7F+B3htnhPiX2ae198Au/zHeG2e11cDL80L9tLAMZ7TJeCvecH+GvhonhPiX2ae1+sAv81/n98GXovn9DvAa/Ovg/iXmef1OsBv89/nt4HX4jn9DvDa/Osg/mXmeb0O8Nv89/lt4LV4Tr8DvDb/Ooh/mXlerwP8Nv99fht4LZ7T7wCvzb8O4l9mntfrAL/Nf5/fBl6L5/Q7wGvzr4P4l5nn9TrAb/OcXhv4Lf7jief128Br8Zx+B3ht/nUQ/zLzvF4H+G2e02sDv8V/PPG8fht4LZ7T7wCvzb8O4l9mntfrAL/Nc3pt4Lf4jyee11cDL81z+mvgo/nXQfzLzPN6HeC3eU6vDfwW//HEfx7Ev8w8r9cBfpvn9NrAb/EfT/znQfzLzPN6HeC3eU7HgZfmP95v858H8S8zz+t1gN/mfz/Ev8w8r9cBfpv//RD/MvO8Xgf4bZ7TawO/xX888bx+G3gtntPvAK/Nvw7iX2ae1+sAv81zem3gt/iPJ57XbwOvxXP6HeC1+ddB/MvM83od4Ld5Tq8N/Bb/8cTz+m3gtXhOvwO8Nv86iH+ZeV6vA/w2z+m1gd/iP554Xr8NvBbP6XeA1+ZfB/EvM8/rdYDf5jm9NvBb/McTz+u3gdfiOf0O8Nr86yD+ZeZ5vQ7w2zyn48BL8x/vt3levw28Fs/pd4DX5l8H8S8zz+t1gN/mv89vA6/Fc/od4LX510H8y8zzeh3gt/nv89vAa/Gcfgd4bf51EP8y87xeB/htntNrA7/FfzzxvH4beC2e0+8Ar82/DuJfZp7X6wC/zXN6beC3+I8nntdvA6/Fc/od4LX510H8y8zzeh3gt3lOrw38Fv/xxPP6beC1eE6/A7w2/zqIf5l5Xq8D/DbP6bWB3+I/nnhevw28Fs/pd4DX5l8H8S8zz+t1gN/mOb028Fv8xxPP67eB1+I5/Q7w2vzrIP5l5nm9DvDbPKfjwEvzH++3eV6/DbwWz+l3gNfmXwfxLzPP63WA3+a/z28Dr8Vz+h3gtfnXQfzLzPN6HeC3+e/z28Br8Zx+B3ht/nUQ/zLzvF4H+G3++/w28Fo8p98BXpt/HcS/zDyv1wF+m/8+vw28Fs/pd4DX5l8H8S8zz+t1gN/mv89vA6/Fc/od4LX510H8y3aBYzynvwZ2+e/z0sBxntPnAJ/Nvw7iX/bdwHvxP9/bAD/Nvw7iX/Zg4On8z/Y3wEvzr4d40bw38F38z/Q3wFsDt/Kvh3jRPRj4bOCtgWP893sG8N3AVwO7/Nsg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+hfL9BxcvSMQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPostAdd;
impl IconShape for MdPostAdd {
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
                d: "M17,19.22H5V7h7V5H5C3.9,5,3,5.9,3,7v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-7h-2V19.22z",
            }
            path {
                d: "M19,2h-2v3h-3c0.01,0.01,0,2,0,2h3v2.99c0.01,0.01,2,0,2,0V7h3V5h-3V2z",
            }
            rect {
                height: "2",
                width: "8",
                x: "7",
                y: "9",
            }
            polygon {
                points: "7,12 7,14 15,14 15,12 12,12",
            }
            rect {
                height: "2",
                width: "8",
                x: "7",
                y: "15",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP8z3IJ+Gv+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i/zfE/2+I/17HuWKX/x6I/z7Hgd/iitcBdvmvh/jvcRz4LeClueKvgdcBdvmvhfivdxz4LeCleU5/DbwOsMt/HcR/rePAbwEvzfP318DrALv810D81zkO/Bbw0rxwfw28DrDLfz7Ef43jwG8BL82L5q+B1wF2+c+F+M93HPgt4KX51/lr4HWAXf7zIP5zHQd+C3hp/m3+GngdYJf/HIj/PMeB3wJemn+fvwZeB9jlPx7iP8dx4LeAl+Y/xl8DrwPs8h8L8R/vOPBbwEvzH+uvgdcBdvmPg/iPdRz4LeCl+c/x18DrALv8x0D8xzkO/Bbw0vzn+mvgdYBd/v0Q/zGOA78FvDT/Nf4aeB1gl38fxL/fceC3gJfmv9ZfA68D7PJvh/j3OQ78FvDS/Pf4a+B1gF3+bRD/dseB3wJemv9efw28DrDLvx7i3+Y48FvAS/M/w18DrwPs8q+D+Ld5MPBgXjS/xb/P6/CiuRW4lX8dxH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M/32vz7/Db/eRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwEWClVB9i5DrQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPublish;
impl IconShape for MdPublish {
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
                d: "M5 4v2h14V4H5zm0 10h4v6h6v-6h4l-7-7-7 7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3EceC/grYHX5opbgb8Gfhr4Hv57IP7zvTfwVcBxXrDfBt4HuJX/Woj/XO8NfBcvml3gdYC/5r8O4j/PawO/xb/OrcDLALv810D853k68GD+9T4H+Gz+ayD+c7w08Ff829wKPIT/Goj/HJ8NfBb/di8D/DX/+RD/Ob4beC/+7V4H+G3+8yH+c3w38F78270O8Nv850P85/hs4LP4t3sZ4K/5z4f4z/HSwF/xb/MM4MH810D857kVeBD/ep8DfDb/NRD/eV4b+C3+dZ4BvDSwy38NxH+u9wa+ixfNJeC1gb/mvw7iP997A18NHOMF+x3gvYFb+a+F+K9xHHhv4K2B1+KKZwC/Dfw28N3890D8/4b4/w3x/xvi/zfE/2+If5/jwFsBDwZeGvhrYBf4GeBW/udD/Nt9FvDRwHGev+8GPgbY5X8uxL/eceCngNfmX7YLvA7w1/zPhPjX+y7gvXnR7QIPAXb5nwfxr/PWwE/xr/c9wHvzPw/iX+evgJfm3+YhwK38z4J40T0YeDr/dh8DfDX/syBedK8N/Bb/dp8DfDb/syBedK8N/Bb/dj8DvDX/syBedK8N/Bb/dp8DfDb/syD+dcy/3dsAP82/3oOBjwLeGngwz/bTwE8D38O/HeJf57uB9+Jf7xLwYGCXf52PAr6aF+6vgfcB/pp/PcS/zoOBp/Ov9znAZ/Ov813Ae/Oi2QVeB/hr/nUQ/3rvDXwXL7qfAd6af523Bn6Kf51bgYfwr4P4t3lv4Lv4l30P8NHALi+648BfAQ/mX+9zgM/mRYf4t3sw8NnAe/G8/gb4bOCn+dd7a+Cn+Le5FXgILzrEf4zX5tluBW7l3+6rgY/i3+4hwK28aBD/8/w28Fr8270O8Nu8aBD/8/w28Fr8270O8Nu8aBD/83w28Fn8250AdnnRIP7neWvgp/i3+RvgpXnRIf5nuhV4EP967wN8Ny86xP9Mbw38FP86fwO8NP86iP+53hv4Ll40fwO8NrDLvw7if7a3Br4bOMYL9jPAewO7/Osh/uc7Drw18NbAg4EHA38N/DXw08Bv82+H+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8ENvZ7QV56RAQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScatterPlot;
impl IconShape for MdScatterPlot {
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
                cx: "7",
                cy: "14",
                r: "3",
            }
            circle {
                cx: "11",
                cy: "6",
                r: "3",
            }
            circle {
                cx: "16.6",
                cy: "17.6",
                r: "3",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGqklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MvGj+BvhoXjRfDbwUV7wOz+m3eF7vA9zKs7028Fm8aMQLhviXmRfN7wCvzYvmt4HX4grxnMxz+hrgo3m248BfAQ/mRSNeMMS/zLxofgd4bV40vw28FleI52Se7RnASwO7PNtXAx/Fi068YIh/mXnR/A7w2rxofht4La4Qz8k82+sAv82zvTbwW/zriBcM8S8zL5pd4K95/j4G+Gue7beB1+IK8ZzMFV8DfDTPdhz4K+DB/OuIFwzxLzP/fq8D/DbP9tvAa3GFeE4GngG8NLDLs3018FH864kXDPEvMy+avwE+mufvr4Fdnu23gdfiCvGcDLwO8Ns822sDv8W/jXjBEP8y86L5HeC1edH8NvBaXCGe01cDH82zHQf+Cngw/zbiBUP8y8yL5neA1+ZF89vAa3GFeE4vDfw1z/bawG/xbydeMMS/zLxofgd4bV40vw28FleI5/RbwOvwnH4aeCv+bcQLhviXmRfN7wCvzbO9NPBVPH8vDRznCvGcDHwM8NU823HgVuAY/3riBUP8y8yL5neA1+bZXhv4Lf5l4jkZ2AVeBriVZ3tr4Kf41xMvGOJfZl40vwO8Ns92HHhpnr+vBl6KK8RzMlf8NvA6PKefBt6Kfx3xgiH+ZeZF8zvAa/Oi+W3gtbhCPCfzbB8DfDXPdhy4FTjGi068YIh/mXnR/A7w2rxofht4La4Qz8k82y7wMsCtPNtbAz/Fi068YIh/mXnR/A7w2rxofht4La4Qz8k8p98GXofn9NPAW/Fsl4DPBr6K5yVeMMS/zLxodoG/5kXz0sBxrvhtntNr87z+Gtjl2Y4DL80Vl4DXBo4Dv8XzEi8Y4l9m/ue6BLw28NfAawO/xfMSLxjiX2b+Z7oEvDbw11zx2sBv8bzEC4b4l5n/eS4Brw38Nc/22sBv8bzEC4b4l5l/u7/hipfiP84l4LWBv+Y5vTbwWzwv8YIh/mXm3+ZvgNfmit8GXop/v0vAawN/zfN6a+CneF7iBUP8y8y/3t8Arw3scsVx4LeBl+Lf7hLw2sBf87xeGvgt4DjPS7xgiH+Z+df5G+C1gV2e03Hgt4GX4l/vEvDawF/zvF4a+C3gOM+feMEQ/zLzovsb4LWBXZ6/48BvAy/Fi+4S8NrAX/O8Xhr4LeA4L5h4wRD/MvOi+RvgtYFdXrjjwG8DL8W/7BLw2sBf87xeGvgt4DgvnHjBEP8y8y/7G+C1gV1eNMeB3wZeihfsEvDawF/zvF4a+C3gOP8y8YIh/mXmhfsb4LWBXZ7Xd3HF+/C8jgO/DbwUz+sS8NrAX/O8Xhr4LeA4LxrxgiH+ZeYF+xvgtYFdntd3Ae/NFd8NvA/P6zjw28BL8WyXgNcG/prn9dLAbwHHedGJFwzxLzPP398Arw3s8ry+C3hvntN3A+/D8zoO/DbwUsAl4LWBv+Z5vTTwW8Bx/nXEC4b4l5nn9TfAawO7PK/vAt6b5++7gffheR0Hfhr4aOCveV4vDfwWcJx/PfGCIf5l5nm9DvDbPK/vAt6bF+67gffhRffSwG8Bx/m3ES8Y4l9mntfrAL/Nc/ou4L150Xw38D78y14a+C3gOP924gVD/MvM83od4Ld5tu8C3pt/ne8G3ocX7KWB3wKO8+8jXjDEv8w8r9cBfpsrvgt4b/5tvht4H57XSwO/BRzn30+8YIh/mXlerwP8NvBdwHvz7/PdwPvwbC8N/BZwnP8Y4gVD/MvM83od4L2A9+Y/xncD7wO8NPBbwHH+44gXDPEvM8/rr4GX5j/WTwOvDRznP5Z4wRD/sl3gGP87XQKO84Ih/mXfDbwX/zt9D/DevGCIf9mDgafzv9NDgFt5wRAvmvcGvov/Xd4H+G5eOMSL7sHAZwNvDRzjf6ZLwE8Dnw3cyr8M8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8Ek7sDUJrgfiIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScore;
impl IconShape for MdScore {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 2h1.5v3l2-3h1.7l-2 3 2 3h-1.7l-2-3v3H12V5zM7 7.25h2.5V6.5H7V5h4v3.75H8.5v.75H11V11H7V7.25zM19 13l-6 6-4-4-4 4v-2.5l4-4 4 4 6-6V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAB9UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvuOPDS/O/w18Au/zLE/2+I/98Q/78h/n9DvOiOAy/Ff42/AXb5z4d40b028Fv813gd4Lf5z4d40b028Fv813gd4Lf5z4d40b028Fv813gd4Lf5z4d40b028Fv813gd4Lf5z4d40b028Fv813gd4Lf5z4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AtVNGEEp9aRxAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShortText;
impl IconShape for MdShortText {
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
                d: "M4,9h16v2H4V9z M4,13h10v2H4V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEQklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/u86DuzywiH+bzoO/Bbw18D78IIh/m/6LuC9ueK7gffh+UP83/NdwHvznL4beB+eF+L/lo8Gvorn732A7+Y5If7veG/gu3j+fgZ4a54X4v+G1wZ+i+fvb4DXBnZ5Xoj//V4a+C3gOM/rb4DXBnZ5/hD/uz0Y+CvgOM/rEvDawF/zgiH+9zoO/Bbw0jyvS8BrA3/NC4f43+k48FvAS/P8vQ3w0/zLEP/5jgMfBXwO/3F+Cnhrnr/3Ab6bFw3iP9dx4LeAlwa+G3gf/v2+C3hvnr/PAT6bFx3iP89x4LeAl+bZvht4H/7tPhv4LJ6/7wHem38dxH+O48BvAS/N8/pu4H3413tv4Lt4/n4GeGv+9RD/8Y4DvwW8NC/YdwPvw4vurYGf4vn7G+C1gV3+9RD/sY4DvwW8NP+y7wbeh3/ZSwO/BRznef0N8NrALv82iP84x4HfAl6aF913A+/DC/bSwG8Bx3lel4DXBv6afzvEf4zjwG8BL82/3ncD78PzOg78FvDSPK9LwGsDf82/D+Lf7zjwW8BL8/xdAr4a+CxesO8G3odnOw78FvDSPH9vA/w0/36If5/jwG8BL83zdwl4beCvgfcGvosX7LuB9+GKnwLemufvfYDv5j8G4t/uOPBbwEvz/F0CXhv4a57tvYHv4gX7bq54b56/zwE+m/84iH+b48BvAS/N83cJeG3gr3le7w18F/963wO8N/+xEP96x4HfAl6a5+8S8NrAX/OCvTfwXbzofgZ4a/7jIf51jgO/Bbw0z98l4LWBv+Zf9t7Ad/Ev+xvgtYFd/uMhXnTHgd8CXprn7xLw2sBf86J7b+C7eMH+BnhtYJf/HIgXzXHgt4CX5vm7BLw28Nf867038F08r0vAawN/zX8exL/sOPBbwEvz/F0CXhv4a/7t3hv4Lp7tEvDawF/znwvxwh0Hfgt4aZ6/S8BrA3/Nv997A9/FFW8D/DT/+RAv2HHgt4CX5vm7BLw28Nf8x3lvrvhu/msgnr/jwG8BL83zdwl4beCv+d8N8fy9NfBTPH+XgNcG/pr//RAv2HsD38VzugS8NvDX/N+AeOHeG/gurrgEvDbw1/zfgfiXvTfw1cBrA3/N/y2IF81xYJf/exD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGP0YlB0KGPkwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShowChart;
impl IconShape for MdShowChart {
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
                d: "M3.5 18.49l6-6.01 4 4L22 6.92l-1.41-1.41-7.09 7.97-4-4L2 16.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACV0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/z3Ms/rdYDf5l+GeNG9NvBbPC/x38s8r9cBfpt/GeJF99rAb/G8xH8v87xeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8Xzem3+e/02z+t1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RWrorQRZ7S7IAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpaceBar;
impl IconShape for MdSpaceBar {
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
                d: "M18 9v4H6V9H4v6h16V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStackedLineChart;
impl IconShape for MdStackedLineChart {
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
                d: "M2,19.99l7.5-7.51l4,4l7.09-7.97L22,9.92l-8.5,9.56l-4-4l-6,6.01L2,19.99z M3.5,15.49l6-6.01l4,4L22,3.92l-1.41-1.41 l-7.09,7.97l-4-4L2,13.99L3.5,15.49z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3MceCvgpYGXBh4MPJhn+2tgF/hr4K+B3wFu5T8X4j/fawMfBbw1/3rfDXwP8Nv850D853lp4KuA1+bf76eB9wF2+Y+F+M/x1sB3Acf5j7MLvA7w1/zHQfzHe2vgp/jPsQu8DvDX/MdA/Md6MPBXwHH+89wKPIT/GIj/WD8NvBX/sr8BfhvY5dmOA+8NHONf9jnAZ/Pvh/iP82Dg6bxwl4C3Bn6bF+yjga/ihdsFTvDvh/iP89HAV/GCXQJeG/hr/mWvDfwWL9z7AN/Nvw/iP85nA5/FC/Y1wEfzovtp4K14wb4HeG/+fRD/cX4beC1esNcBfpsX3VsDP8UL9jvAa/Pvg/iP89vAa/GCvQ3w07zoXhr4K16wW4GH8O+D+I/z1cBH8YL9NvA6/M+C+I/z3sB38cJ9NvA5/M+B+I9zHLjIv+yngY8BbuW/H+I/1lcDH8WL5ruBrwH+mv8+iP/fEP+/If5lr8X/br/DC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+J/jvYH3Bl6Lf9mtwEP490P8z/PawHcDD+KFex/gu/n3QfzPdBy4FTjGC/Y9wHvz74P4j7ELHOMFE/96Hw18FS/Y7wCvzb8P4j/GbwOvxQv2EOBW/nVeG/gtXjjx74P4j/HTwFvxgr0P8N3867w28Fu8cOLfB/Ef47OBz+IF+23gdfjXeWvgp3jBfgd4bf59EP8xHgw8nRfuY4Cv5kX3V8BL84L9DvDa/Psg/uP8NvBavHDfDXwMsMsLdhz4LuCteeE+Bvhq/n0Q/3FeGvgrXjQ/Dfw18NfALle8NvBg4L35l10CHgzs8u+D+I/10cBX8Z/va4CP5t8P8R/vu4H34j/P7wCvzX8MxH+O9wa+i/94vwO8NbDLfwzEf56XBr4aeC3+Y3wO8Nn8x0L853tt4KOBt+Jf7xLw08BnA7fyHw/xX+c48NrAawMvDbw0cIzn9DfALvDbwG8Dv81/LsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGJc3RB0XjKhQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStrikethroughS;
impl IconShape for MdStrikethroughS {
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
                d: "M6.85,7.08C6.85,4.37,9.45,3,12.24,3c1.64,0,3,0.49,3.9,1.28c0.77,0.65,1.46,1.73,1.46,3.24h-3.01 c0-0.31-0.05-0.59-0.15-0.85c-0.29-0.86-1.2-1.28-2.25-1.28c-1.86,0-2.34,1.02-2.34,1.7c0,0.48,0.25,0.88,0.74,1.21 C10.97,8.55,11.36,8.78,12,9H7.39C7.18,8.66,6.85,8.11,6.85,7.08z M21,12v-2H3v2h9.62c1.15,0.45,1.96,0.75,1.96,1.97 c0,1-0.81,1.67-2.28,1.67c-1.54,0-2.93-0.54-2.93-2.51H6.4c0,0.55,0.08,1.13,0.24,1.58c0.81,2.29,3.29,3.3,5.67,3.3 c2.27,0,5.3-0.89,5.3-4.05c0-0.3-0.01-1.16-0.48-1.94H21V12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFMElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+848FI8f5eAv+bf56WBYzx/fwPs8m+H+Pc7DtwKHOP5ewhwK/82Lw38Fc/fM4CXBnb5t0P8x3hr4Kd4/n4beB3+bX4LeG2ev7cBfpp/H8R/nJ8G3orn722An+Zf56OBr+L5+xngrfn3Q/zHOQ7cChzjee0CDwF2edEcB54OHOd5XQIeDOzy74f4j/XRwFfx/H0N8NG8aL4beC+ev/cBvpv/GIj/eL8NvBbP3+sAv80L99rAb/H8/Q7w2vzHQfzHezDw18AxntdfAy/DC/d04ME8r0vASwO38h8H8Z/js4HP4vn7GOCref4+G/gsnr+PAb6a/1iI/zx/DbwUz2sXeBngVp7Tg4G/Ao7zvH4HeG3+4yH+87w08Fc8fz8NvA3P6beA1+b5exngr/mPh/jP9dnAZ/H8vQ3w01zx1sBP8fx9DvDZ/OdA/Oc6Dvw18CCe163Ay3DFXwEP5nn9DfDS/OdB/Od7beC3eP4+BzgOfBTP3+sAv81/HsR/je8G3ot/na8BPpr/XIj/GseBW4FjvGieAbw0sMt/LsR/nbcGfooXzesAv81/PsR/rZ8G3ooX7meAt+a/BuK/1ksDf8ULdwLY5b8G4r/WZwOfxQv3OcBn818D8V/npYG/4kXzEOBW/vMh/uv8FfDSvGh+G3gd/vMh/mt8NPBV/Ot8DPDV/OdC/Od7MPBXwHGe1zO44kE8r13gIcAu/3kQ//l+C3htnr/X4Yrf4vn7aeBt+M+D+M/11sBP8fx9DfDRXPHdwHvx/L0N8NP850D85zkOPB04zvO6BDwY2OWK48CtwDGe1y7wEGCX/3iI/zzfDbwXz9/bAD/Nc3pv4Lt4/r4G+Gj+4yH+c7w28Fs8f78DvDbP328Dr8Xz9zrAb/MfC/Ef7zjwV8CDeV6XgJcGbuX5ezDw18AxntetwEP4j4X4j/fZwGfx/H0O8Nm8cJ8NfBbP3+cAn83z99rAb/HC/TXwPsBfcwXiP9ZLA3/F8/c3wEvzovlr4KV4/l4G+Gue12sDv8W/bBd4CLALIP5j/RXw0jx/rwP8Ni+a1wZ+i+fvr4GX4Xm9NPDVvGDHgZfiitcBfhtA/Mf5bOCzeP6+Bvho/nW+Gvgonr+PAb6af53XBn6LK14H+G0A8R/jwcBfAcd5XpeABwO7/OscB24FjvG8doGXAW7lRffawG9xxesAvw0g/mP8FvDaPH9vA/w0/zZvDfwUz99vA6/DczoOvBfw1jyv48BLc8XrAL8NIP793hv4Lp6/3wFem3+f3wZei+fvbYCf5tn+CnhpXrhLwIOBXQDx73MceDpwnOd1CXhp4Fb+fR4M/DVwjOe1CzwE2AVeG/gtXri/Ad4b+GuuQPzf8drAb3HF6wC/zb8M8X/HawO/xRWvA/w2/zLE/x2vDfwWV3w08Ne8YH8D7AKI/zteG/gtXjSvA/w2gPi/47WB3+JF8zrAbwOI/zuOAy/Ni+avgV0A8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RWVOzQaNQbDEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubscript;
impl IconShape for MdSubscript {
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
                d: "M22,18h-2v1h3v1h-4v-2c0-0.55,0.45-1,1-1h2v-1h-3v-1h3c0.55,0,1,0.45,1,1v1C23,17.55,22.55,18,22,18z M5.88,18h2.66 l3.4-5.42h0.12l3.4,5.42h2.66l-4.65-7.27L17.81,4h-2.68l-3.07,4.99h-0.12L8.85,4H6.19l4.32,6.73L5.88,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/47jwEvxwl0C/ppnQ/zf8drAb/Ev+xzgs7kC8X/HawO/xYvmZYC/BhD/dxwHXpoX7KWBr+KK1wF+G0D8//HawG9xxesAvw0g/v94beC3uOJ1gN8GEP9+Lw38Fc/frcDLALv82xwHng4c5/l7GeCvebaXBr4LeGleuNcBfhtA/Mf4bOCzeP4+B/hs/m2+Gvgonr/PAT6bZ3sw8FfAcV64vwFemisQ/zGOA38NPIjn72WAv+Zf56WBv+L5ewbwYJ7TawO/xRV/A+zynHaB3wa+G9jlCsR/nNcGfovn77eB1+Ff56+Al+b5ex3gt3lOrw38Fle8DvDb/MsQ/7G+Gvgonr/3Ab6bF81HA1/F8/c1wEfzvF4b+C2ueB3gt/mXIf5jHQduBY7xvHaBhwC7vHDHgacDx3lezwBeGtjleb028Ftc8dfALi/YxwB/DSD+47018FM8f98DvDcv3E8Bb83z9zbAT/P8vTbwW7xoXgf4bQDxn+Ongbfi+Xsd4Ld5/l4b+C2ev58B3poX7LWB3+JF8zrAbwOI/xzHgVuBYzyvW4GH8Pw9HXgwz+sS8GBgl/9YiP88Hw18Fc/f5wCfzXP6bOCzeP7eBvhp/uMh/nP9NvBaPK9d4GWAW7niwcDTef5+B3ht/nMg/nM9GHg6z99vA6/DFb8FvDbP6xLw0sCt/OdA/Of7bOCzeP7ehit+iufvY4Cv5j8P4r/GXwMvxfPa5YrjPK/fAV6b/1yI/xovDfwV/zovA/w1/7kQ/3W+GvgoXjSfA3w2//kQ/3WOA38NPIgX7m+Al+a/BuK/1msDv8UL9zLAX/NfA/Ff662Bn+KFex3gt/mvgfivcxx4OnCcF+5W4GWAXf7zIf7rfDXwUbxovgb4aP7zIf5rvDbwW/zrvAzw1/znQvzXeDrwYP51/hp4Gf5zIf7zfTbwWTx/HwMcBz6L5+9zgM/mPw/iP9dLA3/F8/c3wEtzxV8DL8Xz2gVeBriV/xyI/1y/Bbw2z9/LAH/NFS8N/BXP328Dr8N/DsR/no8Gvorn72uAj+Y5fTXwUTx/HwN8Nf/xEP85Hgz8FXCc5/UM4KWBXZ7TceCvgQfxvHaBhwC7/MdC/Of4KeCtef7eBvhpnr+3Bn6K5++ngbfhPxbiP95bAz/F8/czwFvzwv008FY8f28D/DT/cRD/sY4DTweO87wuAS8N3MoL92Dgr4FjPK9bgZcBdvmPgfiP9dXAR/H8fQzw1bxoPhr4Kp6/rwE+mv8YiP84rw38Fs/f3wAvzb/OXwMvxfP3OsBv8++H+I/zdODBPH8vA/w1/zovDfwVz99fAy/Dvx/iP8ZnA5/F8/c5wGfzb/PVwEfx/H0O8Nn8+yD+/V4a+Cuev2cALw3s8m9zHPhr4EE8fy8D/DX/doh/v98CXpvn73WA3+bf562Bn+L5+23gdfi3Q/z7HAdemhfst/mP8dq8YH8N7PJvg/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CGhvsUGt2S7cAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSuperscript;
impl IconShape for MdSuperscript {
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
                d: "M22,7h-2v1h3v1h-4V7c0-0.55,0.45-1,1-1h2V5h-3V4h3c0.55,0,1,0.45,1,1v1C23,6.55,22.55,7,22,7z M5.88,20h2.66l3.4-5.42h0.12 l3.4,5.42h2.66l-4.65-7.27L17.81,6h-2.68l-3.07,4.99h-0.12L8.85,6H6.19l4.32,6.73L5.88,20z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFT0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M93K/DdwNcAuzx/iBfNewPfxf9Ofw28DXArzwvxL3sw8HT+d/tr4GV4Xoh/2XcD78X/fm8D/DTPCfEvuwgc53+/zwE+m+eE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5l5v+G3wFem+eE+JeZ/xt+B3htnhPiX2b+b/gd4LV5Toh/mfm/4XeA1+Y5If5l5v+G3wFem+eE+Je9Nv837AJ/zXNC/P+G+P8N8f8b4v83xL/stfjX+xtgl+d0HHgp/vV+h+f10sAx/nUuAX/Nc0L8y8y/3usAv81zem3gt/jXE8/rt4HX4l/nd4DX5jkh/mXmX+91gN/mOb028Fv864nn9dvAa/Gv8zvAa/OcEP8y86/3OsBv85xeG/gt/vXE8/pt4LX41/kd4LV5Toh/mfnXex3gt3lOrw38Fv964nn9NvBa/Ov8DvDaPCfEv8z8670O8Ns8p9cGfot/PfG8fht4Lf51fgd4bZ4T4l9m/vVeB/htntNrA7/Fv554Xr8NvBb/Or8DvDbPCfEvM/96rwP8Ns/ptYHf4l9PPK/fBl6Lf53fAV6b54T4l5l/vdcBfpvn9NrAb/GvJ57XbwOvxb/O7wCvzXNC/MvMv97rAL/Nc3pt4Lf41xPP67eB1+Jf53eA1+Y5If5l5l/vdYDf5jm9NvBb/OuJ5/XbwGvxr/M7wGvznBD/MvOv9zrAb/OcXhv4Lf71xPP6beC1+Nf5HeC1eU6If5n513sd4Ld5Tq8N/Bb/euJ5/TbwWvzr/A7w2jwnxL/M/Ou9DvDbPKfXBn6Lfz3xvH4beC3+dX4HeG2eE+JfZv71Xgf4bZ7TawO/xb+eeF6/DbwW/zq/A7w2zwnxLzP/eq8D/DbP6bWB3+JfTzyv3wZei3+d3wFem+eE+JeZf73XAX6b5/TawG/xryee128Dr8W/zu8Ar81zQvzLzL/e6wC/zXN6beC3+NcTz+u3gdfiX+d3gNfmOSH+ZeZf73WA3+Y5vTbwW/zrief128Br8a/zO8Br85wQ/zLzr/c6wG/znF4b+C3+9cTz+m3gtfjX+R3gtXlOiH+Z+dd7HeC3eU6vDfwW/3rief028Fr86/wO8No8J8S/zPzrvQ7w2zyn1wZ+i3898bx+G3gt/nV+B3htnhPiX2b+9V4H+G2e02sDv8W/nnhevw28Fv86vwO8Ns8J8S8z/3qvA/w2z+m1gd/iX088r98GXot/nd8BXpvnhPiXmX+91wF+m+f02sBv8a8nntdvA6/Fv87vAK/Nc0L8y3aBY/zrvA7w2zyn1wZ+i3898bx+G3gt/nU+B/hsnhPiX/bdwHvxr/M6wG/znF4b+C3+9cTz+m3gtfjXeRvgp3lOiH/Zg4Gn86/zOsBv85xeG/gt/vXE8/pt4LV40f0N8NI8L8SL5r2B7+JF9zrAb/OcXhv4Lf71xPP6beC1eNH8DfDWwK08L8SL7sHAZwNvDRzjhXsd4Ld5Tq8N/Bb/euJ5/TbwWrxwzwC+G/hqYJfnD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BKeouEG/8ApeAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTableChart;
impl IconShape for MdTableChart {
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
                d: "M10 10.02h5V21h-5zM17 21h3c1.1 0 2-.9 2-2v-9h-5v11zm3-18H5c-1.1 0-2 .9-2 2v3h19V5c0-1.1-.9-2-2-2zM3 19c0 1.1.9 2 2 2h3V10H3v9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADRklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xDP31cDL8X/LX8DfDTPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8fTXw0vzf8tfAR/OcEP+/If5/Q/z/hvj/DfH/G+L/N8Tz99XAS/F/y98AH81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/Xw28NP+3/DXw0TwnxP9viP/fEP+/If5/Q/z/hvj/DfH8fTXwUvzf8jfAR/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fVwMvzf8tfw18NM8J8f8b4v83xP9viP/fEP+/8Y9UL1RBhAnqeQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTableRows;
impl IconShape for MdTableRows {
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
                d: "M22,7H2V2h20V7z M22,9.5H2v5h20V9.5z M22,17H2v5h20V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADs0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviBTsOvBT/M/wNsMt/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9trAb/E/w+sAv81/PMQL9tLAV/Ov91q8cL/Dv95HA3/NfzzEfzzzwon/ORD/8cwLJ/7nQPzHMy+c+J8D8R/PvHDifw7Efzzzwon/ORD/8cwLJ57XbwOvxX+e1wF+m+eF+I9nXjjxvH4beC3+87wO8Ns8L8R/PPPCief128Br8Z/ndYDf5nkh/uOZF048r98GXov/PK8D/DbPC/Efz7xw4nn9NvBa/Od5HeC3eV6I/3jmhRPP67eB1+I/z+sAv83zQvzHMy+ceF6/DbwW/3leB/htnhfiP5554cTz+m3gtfjP8zrAb/O8EP/xzAsn/vV+G3gtXrDXAX6bfz3Efzzzwol/vd8GXosX7HWA3+ZfD/Efz7xw4l/vt4HX4gV7HeC3+ddD/MczL5z41/tt4LV4wV4H+G3+9RD/8cwLJ/71fht4LV6w1wF+m389xH8888KJf73fBl6LF+x1gN/mXw/xH8+8cOJf77eB1+IFex3gt/nXQ/zHMy+c+Nf7beC1eMFeB/ht/vUQ//HMCyf+9X4beC1esNcBfpt/PcR/PPPCiX+93wZeixfsdYDf5l8P8R/PvHDiX++3gdfiBXsd4Lf510P8xzMvnPjX+23gtXjBXgf4bf71EP/xzAsn/vV+G3gtXrDXAX6bfz3Efzzzwol/vd8GXosX7HWA3+ZfD/Efz7xw4l/vt4HX4gV7HeC3+ddD/MczL5z41/tt4LV4wV4H+G3+9RD/8cwLJ/71fht4LV6w1wF+m389xH8888KJf73fBl6LF+x1gN/mXw/xH++3eeFem3+9rwZemhfso4G/5l8P8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPVVhpQQyP00QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextFields;
impl IconShape for MdTextFields {
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
                d: "M2.5,4v3h5v12h3V7h5V4H2.5z M21.5,9h-9v3h3v7h3v-7h3V9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACfUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+algWP8z3IJ+Gv+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5uvBl6af5vX4oX7Hf5t/hr4aP51EP/1zAsn/usg/uuZF07810H81zMvnPivg/ivZ1448V8H8V/PvHDivw7iv5554cR/HcR/PfPCif86iP965oUT/3UQ//XMCyf+6yD+65kXTvzXQfzXMy+c+K+D+K9nXjjxXwfxX8+8cOK/DuK/nnnhxH8dxH8988KJ/zqI/3rmhRP/dRD/9cwLJ/7rIP7rmRdO/NdB/NczL5z4r4P4r2deOPFfB/Ffz7xw4r8O4r+eeeHEfx3Efz3zwon/Ooj/euaFE/91EP/1zAsn/usg/uuZF07810H81zMvnPivg/ivZ1448V8H8V/PvHDivw7iv5554cR/HcR/PfPCif86iP96v80L99r810H8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNQFzNBFGFMgAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTitle;
impl IconShape for MdTitle {
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
                d: "M5 4v3h5.5v12h3V7H19V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADjklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/HA8GHsQL9tXAS/Oc/hr4aF6wZwC38h8L8Z/jOPDbwEvxH+NvgNcGdvmPhfjPcxz4beCl+Pf5G+C1gV3+4yH+cx0Hfht4Kf5t/gZ4bWCX/xyI/3zHgd8GXop/nb8BXhvY5T8P4r/GceC3gZfiRfM3wGsDu/znQvzXOQ78NvBSvHB/A7w2sMt/PsR/rePAbwMvxfP3N8BrA7v810D81zsO/DbwUjynvwFeG9jlvw7iv8dx4LeBl+KKvwFeG9jlvxbiv89x4Le54rWBXf7rIf57HeeKXf57IP5/Q/z/hvj/DfH/G+JFdxx4Kf53+Btgl38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EfIcWJBzz+9CwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalAlignBottom;
impl IconShape for MdVerticalAlignBottom {
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
                d: "M16 13h-3V3h-2v10H8l4 4 4-4zM4 19v2h16v-2H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/x0sDx3hOl4C/5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf5zvDTwVbxgLw0c5zntAn/NC/YxwF/zHwvxn+e9ge/iP8b7AN/NfzzEf673Br6Lf5/3Ab6b/xyI/3zvDXwX/zbvA3w3/3kQ/zXeG/gu/nXeB/hu/nMh/uu8N/BdvGjeB/hu/vMh/mu9N/BdvHDvA3w3/zUQ//XeG/gunr/3Ab6b/zqI/x7vDXwXz+l9gO/mvxbiv897A9/FFe8DfDf/9RD/vd6bK76b/x6I/98Q/78h/n9D/P+G+P8N8aJ7aeCr+N/hY4C/5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91LA1/N/w4fDfw1/zLE/2+I/98Q/78h/n9D/P+G+O/1XVzxPvz3QPz3+S7gvbniu4H34b8e4r/HdwHvzXP6buB9+K+F+K/3XcB78/x9N/A+/NdB/Nf6LuC9eeG+G3gf/msg/ut8F/DevGi+G3gf/vMh/mt8F/De/Ot8N/A+/OdC/Of7LuC9+bf5buB9+M+D+M/1XcB78+/z3cD78J8D8Z/nu4D35j/GdwPvw388xH+O9wbemxfspYFjPKdLwF/zgn038N38x0L89/ht4LV4Tr8DvDb/tRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/z1+G3gtntPvAK/Nfy3Ef4+XBo7znHaBv+a/FuL/N8T/b/wj3SVmQZhjUJAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalAlignCenter;
impl IconShape for MdVerticalAlignCenter {
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
                d: "M8 19h3v4h2v-4h3l-4-4-4 4zm8-14h-3V1h-2v4H8l4 4 4-4zM4 11v2h16v-2H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADjklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7jjw0vzv8NfALv8yxP9viP/fEP+/If5/Q/z3Os4Vu/z3QPz3OQ78Fle8DrDLfz3Ef4/jwG8BL80Vfw28DrDLfy3Ef73jwG8BL81z+mvgdYBd/usg/msdB34LeGmev78GXgfY5b8G4r/OceC3gJfmhftr4HWAXf7zIf5rHAd+C3hpXjR/DbwOsMt/LsR/vuPAbwEvzb/OXwOvA+zynwfxn+s48FvAS/Nv89fA6wC7/OdA/Oc5DvwW8NL8+/w18DrALv/xEP85jgO/Bbw0/zH+GngdYJf/WIj/HA8GHswL9tXAS/Gc/gb4aF6wW4Fb+Y+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CM282JBrrjYgwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVerticalAlignTop;
impl IconShape for MdVerticalAlignTop {
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
                d: "M8 11h3v10h2V11h3l-4-4-4 4zM4 3v2h16V3H4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98QL7qXBr6K/x0+Bvhr/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b008NX87/DRwF/zL0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8aJ7aeCr+J9hF/hrYBf4GeBW/m0QL7rXBn6L/5l+G/gc4Lf510G86F4b+C3+Z/tu4GOAXV40iBfdawO/xf98fw28DrDLvwzxontt4Lf43+G3gdfhX4Z40b008NX8z/Jg4EE8fx8DfDUvHOJ/v7cGvhs4xnPaBR4C7PKCIf5vOA7cChzjOb0P8N28YIj/O94a+Cme088Ab80Lhvi/5VbgQTzbXwMvwwuG+L/lt4HX4jmJFwzxP9t7A9/Ni+63gdfiOYkXDPE/13sD3wWIF93TgQfzbH8DvDQvGOJ/pvcGvosrxIvmrYGf4jn9DPDWvGCI/3neG/gunk38y14a+C3gOM/pfYDv5gVD/M/y3sB38ZzEC/fWwHcBx3lOl4AHA7u8YIgX3UsDX8W/3d8AH80L9t7Ad/G8fpsX7MHAg3n+Pgb4al44xIvutYHf4t/ud4DX5vl7b+C7+I/zO8Br8y9DvOheG/gt/u1+B3htntd7A9/Ff5y/AV4b2OVfhnjRvTbwW/zb/Q7w2jyv9wa+i/8Y3wN8NLDLiwbxontt4Lf4t/sd4LV5/t4b+C7+7X4H+Gzgt/nXQbzoXhr4av7t/hr4aF6w9wa+i+f1OzyvXeCvgV3gp4Fb+bdB/M/y3sB38ZzEfx7E/zzvDXwXzyb+8yD+Z3pv4Lu4QvznQfzP9d7AdwHiPw/if7b3Br6b/zyI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COZ/1xBguYH8wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWrapText;
impl IconShape for MdWrapText {
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
                d: "M4 19h6v-2H4v2zM20 5H4v2h16V5zm-3 6H4v2h13.25c1.1 0 2 .9 2 2s-.9 2-2 2H15v-2l-3 3 3 3v-2h2c2.21 0 4-1.79 4-4s-1.79-4-4-4z",
            }
        }
    }
}
