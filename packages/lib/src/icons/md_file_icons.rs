use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/9R4MvBVwHHgwV9wK7AI/A9zKfx3Ef53PAt4beDAv3F8DPw18Dv/5EP/53hr4KuDB/OvcCnwM8NP850H85/oq4KP59/ls4HP4z4H4z/NdwHvzH+O7gffhPx7iP8dHA1/FC/c3wC5XHAdeihfuY4Cv5j8W4j/eawO/xfN3Cfhq4LuBW3lODwbeG/ho4BjP3+sAv81/HMR/vN8CXpvn9TfAWwO38sI9GPhp4KV4Xr8NvA7/cRD/sV4b+C2e198Arw3s8qI5Dvw18CCe1+sAv81/DMR/rO8G3ovn9TLAX/Ov89rAb/G8vgd4b/5jIP5jXQSO85y+B3hv/m2+G3gvntOtwEP4j4H4j/Ng4Ok8r7cBfpp/m7cGforndQLY5d8P8R/ntYHf4nk9BLiVf5sHA0/neb0O8Nv8+yH+47w28Fs8rxPALv82x4GLPK/XAX6bfz/Ef6zfBl6L5/TdwPvwb/NdwHvznH4HeG3+YyD+Yx0HbgWO8ZzeB/hu/nXeG/guntMl4MHALv8xEP/x3hr4KZ7TLvA6wF/zonlp4LeA4zyntwF+mv84iP8cXw18FM/pr4HXAXZ54Y4DvwW8NM/pa4CP5j8W4j/HceC3gZfiOX038D68cN8FvDfP6W+A1wZ2+Y+F+M/z0sBf8bzeB/hunr/3Br6L5/UywF/zHw/xn+ujga/iOe0CrwP8Nc/ppYHfAo7znD4G+Gr+cyD+8/028Fo8p78GXgfY5YrjwG8BL81z+h3gtfnPg/jPdxy4FTjGc/pu4H244ruA9+Y5XQIeDOzynwfxX+OtgZ/ieb0PV3wXz+ttgJ/mPxfiv85XAx/Fc9rliuM8p68BPpr/fIj/OseB3wZeihfub4DXBnb5z4f4r/XSwF/xwr0M8Nf810D81/to4Kt4/j4G+Gr+6yD+e/w28Fo8p98BXpv/Woj/HseBW4FjXHEJeDCwy38txH+ftwZ+iiveBvhp/ush/nt9NVd8NP89EP82x4H3Al4aeDD/dse5Ypd/u1uBvwa+B9jlXwfxr/fewFcBx/mfZRf4GOC7edEh/nVeG/gt/md7HeC3edEg/nWeDjyY/9luBR7Ciwbxontp4K/43+FlgL/mX4Z40b028Fs8r9/hv9dr8bxeB/ht/mWIF91rA7/F8xL/vczzeh3gt/mXIV50rw38Fs9L/Pcyz+t1gN/mX4Z40b028Fs8L/Hfyzyv1wF+m38Z4kX32sBv8bzEfy/zvF4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3UsDX83/Dh8N/DX/MsT/b4j/3xD/vyH+f0P8/8Y/Ao6lrEGWtotXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdApproval;
impl IconShape for MdApproval {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 16v6h16v-6c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2zm14 2H6v-2h12v2zM12 2C9.24 2 7 4.24 7 7l5 7 5-7c0-2.76-2.24-5-5-5zm0 9L9 7c0-1.66 1.34-3 3-3s3 1.34 3 3l-3 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAttachEmail;
impl IconShape for MdAttachEmail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,10V4c0-1.1-0.9-2-2-2H3C1.9,2,1.01,2.9,1.01,4L1,16c0,1.1,0.9,2,2,2h11v-5c0-1.66,1.34-3,3-3H21z M11,11L3,6V4l8,5 l8-5v2L11,11z",
            }
            path {
                d: "M21,14v4c0,1.1-0.9,2-2,2s-2-0.9-2-2v-4.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V18h2v-4.5c0-1.38-1.12-2.5-2.5-2.5 S15,12.12,15,13.5V18c0,2.21,1.79,4,4,4s4-1.79,4-4v-4H21z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4j/Oa/Hf63f410P827008F7AWwMP5n+Gnwa+B/hpXjSIf73XBj4LeG3+5/pt4GOAv+aFQ/zrfBfw3vzvsAu8DvDXvGCIF81x4KeA1+Z/l13gdYC/5vlD/MuOA78FvDT/O90KPITnD/Ev+y7gvXnBfgb4aeCvgb/mv8dLAx8NvBfP3+cAn83zQrxw7w18F8/f3wAfDfw2/3O8N/BdPK9d4ATPC/HCPR14MM/re4CPBnZ54Y4DL8V/vL8Bdnn+Phv4LJ7XywB/zXNCvGAfDXwVz+t3gNfmhXsw8FnAe/Of43WA3+b5Ow5c5Hl9DvDZPCfEC/ZXwEvznC4BDwZ2ecFeG/gp4Dj/eV4H+G1esN8GXovn9DnAZ/OcEM/fg4Gn87w+B/hsXrAHA38FHOc/1+sAv80L9tvAa/GcfgZ4a54T4vl7b+C7eF4PAW7lBftu4L34z/c6wG/zgv0V8NI8p88BPpvnhHj+Phv4LJ7T3wAvzQt2HLjIf43XAX6b5+/BwNN5Xh8DfDXPCfH8fTXwUTyn3wFemxfstYHf4nm9DvDb/Nf5LeC1eV4PAW7lOSGev58G3orn9DvAa/OCvTbwWzwv8V/jOPBdwFvzvH4HeG2eF+L5+2zgs3hOvwO8Ni/YawO/xfP6aeCr+c/zYOClgfcGjvP8vQ7w2zwvxPP32cBn8Zx2gRO8cLvAMf5n+R7gvXn+EM/fWwM/xfN6HeC3ecE+G/gs/uf4G+C1gV2eP8QLZp7XTwNvwwt2HPht4KX47/c7wFsDu7xgiBfsp4G34nm9DvDbvGDHgZ8GXov/HpeAzwa+mn8Z4gV7beC3eF67wMsAt/LCvTfw1sBb8Z/vb4BbgZ8GfhrY5UWDeOF+G3gtntcu8DrAX/O/G+KFe2ngr3jBPhv4GmCX/50Q/7L3Br6LF+6ngb8GbgVu5T/f3wC7/PshXjRfDXwU/3O8DvDb/PshXnTvDXwX/zO8DvDb/Psh/nVeG/hu4EH893od4Lf590P823w08NnAMf57vA7w2/z7If59Xhp4b+ClgePAS/Ff43WA3+bfD/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRo/oJBYK4KxAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAttachment;
impl IconShape for MdAttachment {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 12.5C2 9.46 4.46 7 7.5 7H18c2.21 0 4 1.79 4 4s-1.79 4-4 4H9.5C8.12 15 7 13.88 7 12.5S8.12 10 9.5 10H17v2H9.41c-.55 0-.55 1 0 1H18c1.1 0 2-.9 2-2s-.9-2-2-2H7.5C5.57 9 4 10.57 4 12.5S5.57 16 7.5 16H17v2H7.5C4.46 18 2 15.54 2 12.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADV0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4Dj/NfYBV4H+Gv+fRD/fseBpwPH+a+1C7wMcCv/doh/v78CXpr/Hr8NvA7/doh/n68GPor/Xp8DfDb/Noh/u5cG/or/GV4H+G3+9RD/eq8FvDTw3sBL8z/D9wDvzb8e4kXz2sB7Ae/N/1wPAW7lXwfxwr008FXAa/M/3/cA782/DuIF+yzgs/nfYxc4wb8O4vn7LuC9+d/nZYC/5kWHeF4/Bbw1/zt9N/AxwC4vGsRz+mrgo/jfbxf4beC7gZ/hBUM821sDP8X/PbcCHwP8NM8L8WxPBx7M/12/DbwNsMuzIa54b+C7+L/vr4HXAXa5AnHF04EH8//DXwOvA+wCCHhp4K/4/+W7gfcBEPDZwGfx/8/rAL8t4LeB1+L/n98GXkfAbwOvxf9PLyPA/P/1MQLM/1+/I+BW4EH8//TXAn4beC3+f0LATwNvxf9PCHhv4Lv4/+lvBBwHLvL/0++IK34beC3+//kYccVrA7/F/z8PEc/208Bb8f/H7wCvLZ7twcBfA8f4/+F1gN8Wz+m1gd/i/77vAd4bQDyv9wa+i/+7/gZ4bWAXQDx/bw18N3CM/1v+Bnhr4FauQLxgLw38NPAg/m/4HeCtgV2eDfEve2/gs4EH8b/TM4DPBr6b54V40b018NbAWwPH+J/vZ4CfBn4a2OX5Q/zbvTb/M+0Cf82LBvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EWagZ45LVyNWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloud;
impl IconShape for MdCloud {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudCircle;
impl IconShape for MdCloudCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm4.5 14H8c-1.66 0-3-1.34-3-3s1.34-3 3-3l.14.01C8.58 8.28 10.13 7 12 7c2.21 0 4 1.79 4 4h.5c1.38 0 2.5 1.12 2.5 2.5S17.88 16 16.5 16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEs0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4Dj/NfYBV4H+Gv+fRD/fseBpwPH+a+1C7wMcCv/doh/v78CXpr/Hr8NvA7/doh/n68GPor/Xp8DfDb/Noh/u5cG/or/GV4H+G3+9RD/eq8FvDTw3sBL8z/D9wDvzb8e4kXz2sB7Ae/N/1wPAW7lXwfxwr008FXAa/M/3/cA782/DuIF+yzgs/nv9TfARwM/DRzjhdsFTvCvg3j+vgt4b/57/Q3w2sAu8NLAbwPHeOFeBvhrXnSI5/VTwFvz3+tvgNcGdnm2lwZ+GzjGC/bdwMcAu7xoEM/pq4GP4r/X3wCvDezyvF4a+Cv+ZbvAbwPfDfwMLxji2d4a+Cn+e/0N8NrALs/fdwHvzb/OrcDHAD/N80I829OBB/Pf52+A1wZ2ef6+C3hv/u1+G3gbYJdnQ1zx3sB38d/nb4DXBnZ5/r4LeG/+/f4aeB1glysQVzwdeDD/Pf4GeG1gl+fvu4D35j/OXwOvA+wCCHhp4K/4j/M3wF8D78W/7G+A1wZ2ef6+C3hv/uN9N/A+AAI+G/gs/mP8DfDawC7w3cB78YL9DfDawC7P33cB781/ntcBflvAbwOvxb/f3wCvDezybN8NvBfP62+A1wZ2ef6+C3hv/nP9NvA6An4beC3+ff4GeG1gl+f13cB78Wx/A7w2sMvz913Ae/Nf42UEmH+/vwZeB9jl+ftu4L2AvwFeG9jl+fsu4L35r/MxAsx/jL8GXgfY5fn7bOCrgV2ev+8C3pv/Wr8j4FbgQfzH+GvgdYBd/nW+C3hv/uv9tYDfBl6L/zh/DbwOsMuL5ruA9+a/BwJ+Gngr/mP9NfA6wC4v3HcB781/HwS8N/Bd/Mf7a+B1gF2ev+8C3pv/Xn8j4Dhwkf8cfw28DrDLc/ou4L357/c74orfBl6L/xx/DbwOsMsV3wW8N/8zfIy44rWB3+I/z18DrwN8FfDe/M/xEPFsPw28Ff95doHj/M/xO8Bri2d7MPDXwDH+f3gd4LfFc3pt4Lf4v+97gPcGEM/rvYHv4v+uvwFeG9gFEM/fWwPfDRzj/5a/Ad4auJUrEC/YSwM/DTyI/xt+B3hrYJdnQ/zL3hv4bOBB/O/0DOCzge/meSFedG8NvDXw1sAx/uf7GeCngZ8Gdnn+EP92r83/TLvAX/OiQfz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BJLntY7rBtOXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudDone;
impl IconShape for MdCloudDone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM10 17l-3.5-3.5 1.41-1.41L10 14.17 15.18 9l1.41 1.41L10 17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4Dj/NfYBV4H+Gv+fRD/fseBpwPH+a+1C7wMcCv/doh/v78CXpr/Hr8NvA7/doh/n68GPor/Xp8DfDb/Noh/u5cG/or/GV4H+G3+9RD/eq8FvDTw3sBL8z/D9wDvzb8e4kXz2sB7Ae/N/1wPAW7lXwfxwr008FXAa/M/3/cA782/DuIF+yzgs/mP8zvAa/Oi+W3gtfjX2QVO8K+DeP6+C3hv/mP9DvDavGh+G3gt/vVeBvhrXnSI5/VTwFvzH+93gNfmRfPbwGvxr/fdwMcAu7xoEM/pq4GP4j/H7wCvzYvmt4HX4t9uF/ht4LuBn+EFQzzbWwM/xX+e3wFemxfNbwOvxX+MW4GPAX6a54V4tqcDD+Y/z+8Ar82L5reB1+I/1m8DbwPs8myIK94b+C7+c/0O8Nq8aH4beC3+4/018DrALlcgrng68GD+c/0O8Nq8aH4beC3+c/w18DrALoCAlwb+iv98vwO8Ni+a3wZei/883w28D4CAzwY+i/98vwO8Ni+a3wZei/9crwP8toDfBl6L/xi/wwv218BH86L5auClecFei3+/3wZeR8BvA6/Ff4zvBt6H/1zfBbw3/zFeRoD5j/XdwPvwn+O7gPfmP87HCDD/8b4beB/+Y30X8N78x/odAbcCD+I/3ncD78N/jO8C3pv/eH8t4LeB1+I/x3cD78O/z3cB781/DgT8NPBW/Of5buB9+Lf5LuC9+c+DgPcGvov/XN8NvA//Ot8FvDf/uf5GwHHgIv/5vht4H1403wW8N//5fkdc8dvAa/Gf77uB9+GF+y7gvfmv8THiitcGfov/Gt8NvA/P33cB781/nYeIZ/tp4K34r/HdwPvwnL4LeG/+6/wO8Nri2R4M/DVwjP8a3w28D1d8F/De/Nd6HeC3xXN6beC3+K/z3cAl4KP4r/U9wHsDiOf13sB38X/X3wCvDewCiOfvrYHvBo7xf8vfAG8N3MoViBfspYGfBh7E/w2/A7w1sMuzIf5l7w18NvAg/nd6BvDZwHfzvBAvurcG3hp4a+AY//P9DPDTwE8Duzx/iH+71+Z/pl3gr3nRIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AjVpmI6bcax6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudDownload;
impl IconShape for MdCloudDownload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM17 13l-5 5-5-5h3V9h4v4h3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudOff;
impl IconShape for MdCloudOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4c-1.48 0-2.85.43-4.01 1.17l1.46 1.46C10.21 6.23 11.08 6 12 6c3.04 0 5.5 2.46 5.5 5.5v.5H19c1.66 0 3 1.34 3 3 0 1.13-.64 2.11-1.56 2.62l1.45 1.45C23.16 18.16 24 16.68 24 15c0-2.64-2.05-4.78-4.65-4.96zM3 5.27l2.75 2.74C2.56 8.15 0 10.77 0 14c0 3.31 2.69 6 6 6h11.73l2 2L21 20.73 4.27 4 3 5.27zM7.73 10l8 8H6c-2.21 0-4-1.79-4-4s1.79-4 4-4h1.73z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4DjvHC/A/w08NfArcCtXPHSwHHgpYH3Bl6KF24XeB3gr/n3Qfz7HQeeDhzn+bsEfDfw1cCtvGiOA18NvBcv2C7wMsCt/Nsh/v3+Cnhpnr/fAd4buJV/mwcD3w28Fs/fbwOvw78d4t/nq4GP4vn7HuC9+Y/x3cB78fx9DvDZ/Nsg/u1eGvgrnr/3Ab6b/1gfDXwVz9/rAL/Nvx7iX++1gJcG3ht4aZ7X9wDvzX+O7wbei+f1PcB786+HeNG8NvBewHvzwv0N8NL85/pr4KV4Xg8BbuVfB/HCvTTwVcBr86J5HeC3+c/12sBv8by+B3hv/nUQL9hnAZ/Ni+53gNfmv8ZvA6/Fc9oFTvCvg3j+vgt4b/513gb4af5rvDXwUzyvlwH+mhcd4nn9FPDW/Mt+h+f02vzXOQ5c5Hl9N/AxwC4vGsRz+mrgo3jBfgb4buCn+e/308Bb8fztAr8NfDfwM7xgiGd7a+CneP7+Bvho4Lf5n+Ozgc/iX3Yr8DHAT/O8EM/2dODBPK+/AV4b2OV/ls8GPosX3W8DbwPs8myIK94b+C6e198Arw3s8j/PWwM/xb/OXwOvA+xyBeKKpwMP5jldAl4auJX/mV4b+C3+9f4aeB1gF0DASwN/xfP6HOCz+d/ntYG3Bt4bOMbz993A+wAI+Gzgs3heJ4Bd/vc6Dnw18F48f68D/LaA3wZei+f0O8Br83/DdwPvxfP6beB1BPw28Fo8p88BPpv/O34beC2e18sIMM/rc4DP5v+Olwb+iuf1MQLM83ob4Kf5v+WvgZfiOf2OgFuBB/GcPgf4bP5v+Wzgs3hOfy3gt4HX4jl9DfDR/N/y0cBX8ZwQ8NPAW/GcbgUewv8tHw18Fc8JAe8NfBfP63WA3+b/js8GPovn9DcCjgMXeV6/DbwO/3f8FfDSPKffEVf8NvBaPK/3Ab6b//1eGvgrntfHiCteG/gtntcu8DrAX/O/228Br83zeoh4tp8G3orntQu8DfDb/O/0XcB787x+B3ht8WwPBv4aOMbz99XA5wC7/O9wHPgq4L15/l4H+G3xnF4b+C1esF3gp4GfBp4B/DX/87wW8NbAewPHef6+B3hvAPG83hv4Lv7v+hvgtYFdAPH8vTXw3cAx/m/5G+CtgVu5AvGCvTTw08CD+L/hd4C3BnZ5NsS/7L2BzwYexP9OzwA+G/hunhfiRffWwFsDbw0c43++nwF+GvhpYJfnD/Fv99r8z7QL/DUvGsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R4yzuI5lniPOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudQueue;
impl IconShape for MdCloudQueue {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4s1.79-4 4-4h.71C7.37 7.69 9.48 6 12 6c3.04 0 5.5 2.46 5.5 5.5v.5H19c1.66 0 3 1.34 3 3s-1.34 3-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dr8byeAdzKfy3Ef42XBt4LeGngtXnhfhv4beBngL/mPxfiP9d7AZ8NPJh/m1uBzwa+h/8ciP8cbw18FfBg/mPcCnwM8NP8x0L8xzoOfBXw3vzn+Grgc4Bd/mMg/uO8NPBdwEvzn+uvgdcBdvn3Q/zHeGngt4Dj/NfYBV4H+Gv+fRD/fseBpwPH+a+1C7wMcCv/doh/v78CXpr/Hr8NvA7/doh/n68GPor/Xp8DfDb/Noh/u5cG/or/GV4H+G3+9RD/eq8FvDTw3sBL8+/zN1zxUvz7fA/w3vzrIV40rw28F/De/Mf5G+C1ueK3gZfi3+chwK386yBeuJcGvgp4bf5j/Q3w2sAuVxwHfht4Kf7tvgd4b/51EC/YZwGfzX+8vwFeG9jlOR0Hfht4Kf5tdoET/Osgnr/vAt6b/3h/A7w2sMvzdxz4beCl+Ld5GeCvedEhntdPAW/Nf7y/AV4b2OWFOw78NvBS/Ot9N/AxwC4vGsRz+mrgo/iP9zfAawO7vGiOA78NvBT/NrvAbwPfDfwMLxji2d4a+Cn+4/0N8NrALv86x4HfBl6Kf59bgY8BfprnhXi2pwMP5j/W3wCvDezyb3Mc+G3gpfj3+23gbYBdng1xxXsD38V/rL8BXhvY5d/nOPDbwEvx7/fXwOsAu1yBuOLpwIP5j/M3wGsDu/zHOA78NvBS/Pv9NfA6wC6AgJcG/or/OH8DvDawy3+s48BvAy/Fv993A+8DIOCzgc/iP8bfAK8N7PKf4zjw28BL8e/3OsBvC/ht4LX499sF3hvY5fm7BPw1L5qXBo7x/B0Hvhs4zr/PbwOvI+C3gdfiP9/vAK/Ni+a3gdfiP9/LCDD/NX4HeG1eNL8NvBb/+T5GgPmv8TvAa/Oi+W3gtfjP9zsCbgUexH++3wFemxfNbwOvxX++vxbw28Br8Z/vd4DX5kXz28Br8Z8PAT8NvBX/+X4HeG1eNL8NvBb/+RDw3sB38Z/vd4DX5kXz28Br8Z/vbwQcBy7yn+93gNfmRfPbwGvxn+93xBW/DbwW/7l+B3htXjS/DbwW//k+Rlzx2sBv8Z/rd4DX5kXz28Br8Z/vIeLZfhp4K/7z7AJ/zYvmpYHj/Of6HeC1xbM9GPhr4Bj/P7wO8NviOb028Fv83/c9wHsDiOf13sB38X/X3wCvDewCiOfvrYHvBo7xf8vfAG8N3MoViBfspYGfBh7E/w2/A7w1sMuzIf5l7w18NvAg/nd6BvDZwHfzvBAvurcG3hp4a+AY//P9DPDTwE8Duzx/iH+71+Z/pl3gr3nRIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Av92po5KirvwAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCloudUpload;
impl IconShape for MdCloudUpload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xbA18FPJj/GN8NvA//cyBesPcGvov/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N9wK/A6wK08L8Tz99vAa/F/x18DL8PzQjx/vw28Fv+3vA3w0zwnxPP328Br8X/L5wCfzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/Hf53WA3+Y5vTbwW/zb/Q7w2jwnxPP328Br8d/ndYDf5jm9NvBb/Nv9DvDaPCfE8/fbwGvx3+d1gN/mOb028Fv82/0O8No8J8Tz99vAa/Hf53WA3+Y5vTbwW/zb/Q7w2jwnxPP328Br8d/ndYDf5jm9NvBb/Nv9DvDaPCfE8/fbwGvx3+d1gN/mOb028Fv82/0O8No8J8Tz99vAa/Hf53WA3+Y5vTbwW/zb/Q7w2jwnxPP328Br8d/ndYDf5jm9NvBb/Nv9DvDaPCfE8/fbwGvxn+d1gN/mP9ZrA7/FC/Y7wGvznBDP328Dr8V/ntcBfpv/WK8N/BYv2O8Ar81zQjx/vw28Fv95Xgf4bf5jvTbwW7xgvwO8Ns8J8fz9NvBa/Od5HeC3+Y/12sBv8YL9DvDaPCfE8/fbwGvxn+d1gN/mP9ZrA7/FC/Y7wGvznBDP328Dr8V/nr8GdnnBPgb4a57TSwNfxQt2HHhpXrDfAV6b54R4/n4beC3++7wO8Ns8p9cGfot/u98BXpvnhHj+fht4Lf77vA7w2zyn1wZ+i3+73wFem+eEeP5+G3gt/vu8DvDbPKfXBn6Lf7vfAV6b54R4/n4beC3++7wO8Ns8p9cGfot/u98BXpvnhHj+fht4Lf77vA7w2zyn1wZ+i3+73wFem+eEeP5+G3gt/vu8DvDbPKfXBn6Lf7vfAV6b54R4/n4beC3++7wO8Ns8p9cGfot/u98BXpvnhHj+fht4Lf77/DWwy3M6Drw0/3a/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8fTfwXvzf8jXAR/OcEM/fRwNfxf8tHwN8Nc8J8fwdB24FjvF/wyXgwcAuzwnxgr038F383/A+wHfzvBAv3FsDXw08iP+dngF8NPDTPH+IF81LA8f532UX+GteOMT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R61wikGle/TvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCreateNewFolder;
impl IconShape for MdCreateNewFolder {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-1 8h-3v3h-2v-3h-3v-2h3V9h2v3h3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEr0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesPcGvov/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP4+Gvgq/nN9N/A+/PdCPH/fDbwX//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/Pf5HuC9+I/1O8Br85wQz99vA6/Ff5/XAR4MfBf/cX4HeG2eE+L5+23gtfjv8zrAbwPvDXwX/zF+B3htnhPi+ftt4LX47/M6wG9zxXsD38W/3+8Ar81zQjx/vw28Fv99Xgf4bZ7tvYHv4t/nd4DX5jkhnr/fBl6L/z6vA/w2z+m9ge/i3+53gNfmOSGev98GXov/Pq8D/DbP672B7+Lf5neA1+Y5IZ6/3wZei/8+rwP8Ns/fewPfxb/e7wCvzXNCPH+/DbwW/7H+BvhoXjR/Dezygr038F386/wO8No8J8Tz99vAa/Ef63eA1+Y/znsD38WL7neA1+Y5IZ6/3wZei/9YvwO8Nv+x3hv4Ll40vwO8Ns8J8fz9NvBa/Mf6HeC1+Y/33sB38S/7HeC1eU6I5++3gdfiP9bvAK/Nf473Br6LF+53gNfmOSGev98GXov/WL8DvDb/eT4b+CxesN8BXpvnhHj+fht4Lf5j/Q7w2vzneGngt4DjvGC/A7w2zwnx/P028Fr8x/od4LX5j/fSwG8Bx3nhfgd4bZ4T4vn7beC1+I/1O8Br8x/rpYHfAo7zL/sd4LV5Tojn77eB1+I/1u8Ar81/nJcGfgs4zovmd4DX5jkhnr/fBl6L/1i/A7w2/zFeGvgt4Dgvut8BXpvnhHj+fht4Lf77vA7w2zx/Lw38FnCcf53fAV6b54R4/n4beC3++7wO8Ns8r5cGfgs4zr/e7wCvzXNCPH+/DbwW/31eB/htntNLA78FHOff5neA1+Y5IZ6/3wZei/8+rwP8Ns/20sBvAcf5t/sd4LV5Tojn77eB1+K/z+sAv80VLw38FnCcf5/fAV6b54R4/n4beC3++7wO8NvASwO/BRzn3+93gNfmOSGev98GXov/Pq8D7AK/BRznP8bvAK/Nc0I8f78NvBb/fT4G+CzgOP9xfgd4bZ4T4vn7buC9+L/la4CP5jkhnr+PBr6K/1s+BvhqnhPi+TsO3Aoc4/+GS8CDgV2eE+IFe2/gu/i/4W2An+Z5IV641wa+G3gQ/zs9A3hv4Ld5/hAvmpcGjvO/yy7w17xwiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPRyKRQXY8UXYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDriveFileMove;
impl IconShape for MdDriveFileMove {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-6 12v-3h-4v-4h4V8l5 5-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDriveFileMoveOutline;
impl IconShape for MdDriveFileMoveOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10zm-8.01-9l-1.41 1.41L12.16 12H8v2h4.16l-1.59 1.59L11.99 17 16 13.01 11.99 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif77jwEtxxTOAW/mPg/if6zjwVcB785x+G3gf4Fb+/RD/Mx0Hfgt4aZ6/XeB1gL/m3wfxP89x4LeAl+aF2wVeB/hr/u0Q/7McB34LeGleNLvA6wB/zb8N4n+O48BvAS/Nv84u8DLArfzrIf5nOA78FvDS/Nv8NvA6/Osh/vsdB34LeGn+fd4G+Gn+dRD/vY4DvwW8NP9+nwN8Nv86iP8+x4HfAl6a/xjfA7w3/zqI/x7Hgd8CXpr/OB8DfDX/Ooj/eseB3wJemufvEnCMf51LwEsDt/Kvg/ivdRz4LeClef6+B/ho4LeBl+JF9zHAV/Ovh/ivcxz4LeClef6+B3hvrjgO/DbwUvzLvgd4b/5tEP81jgO/Bbw0z9/3AO/NczoO/DbwUrxg3wO8N/92iP98x4HfAl6a5+97gPfmeR0Hfgt4aZ6/7wHem38fxH+u48BvAS/N8/c9wHvzvI4DvwW8NM/f9wDvzb8f4j/PceC3gJfm+fse4L15XseB3wJemufve4D35j8G4j/HceC3gJfm+fse4L15XseB3wJemufve4D35j8O4j/eceC3gJfm+fse4L15XseB3wJemufve4D35j8W4j/WceC3gJfm+fse4L15XseB3wJemufve4D35j8e4j/OceC3gJfm+fse4L15XseB3wJemufve4D35j8H4j/GceC3gJfm+fse4L15XseB3wJemufve4D35j8P4t/vOPBbwEvz/H0P8N48r+PAbwEvzfP3PcB7858L8e9zHPgt4KV5/r4HeG+e13Hgt4CX5vn7HuC9+c+H+Lc7DvwW8NI8f98DvDfP6zjwW8BL8/x9D/De/NdA/NscB34LeGmev+8B3pvndRz4LeClef6+B3hv/usg/vWOA78FvDTP3/cA783zOg78FvDSPH/fA7w3/7UQ/zrHgd8CXprn73uA9+Z5HQd+C3hpnr/vAd6b/3qIF91x4LeAl+b5+x7gvXlex4HfAl6a5+97gPfmvwfiRXMc+C3gpXn+vgd4b57XceC3gJfm+fse4L3574P4lx0Hfgt4aZ6/7wHem+d1HPgt4KV5/r4HeG/+eyH+ZX8FvDTP3/cA783zOg78FvDSPH/fA7w3/7LjwEvx7/M7vGCIf5l5/r4HeG+e13Hgt4CX5vn7HuC9+ZcdB34LeGn+fcQLhviXmef1PcB787yOA78FvDTP3/cA782/7DjwW8BL8+8nXjDEv8w8r9cBfpvndBz4LeClef6+B3hv/mXHgd8CXpr/GOIFQ/zLzPN6HeC3ebbjwG8BL83z9z3Ae/MvOw78FvDS/McRLxjiX2ae1+sAv80Vx4HfAl6a5+97gPfmX3Yc+C3gpfmPJV4wxL/M/Nt9D/De/MuOA78FvDT/8cQLhviXmX+b7wHem3/ZceC3gJfmP4d4wRD/MvOv9z3Ae/MvOw78FvDS/OcRLxjiX2b+db4HeG/+ZceB3wJemv9c4gVD/MvMi+57gPfmX3Yc+C3gpfnPJ14wxL/stXnR/TYvmgcDD+a/xm/zgiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwI5vZ5BuC3ETgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDriveFileRenameOutline;
impl IconShape for MdDriveFileRenameOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.41 5.8L17.2 4.59c-.78-.78-2.05-.78-2.83 0l-2.68 2.68L3 15.96V20h4.04l8.74-8.74 2.63-2.63c.79-.78.79-2.05 0-2.83zM6.21 18H5v-1.21l8.66-8.66 1.21 1.21L6.21 18zM11 20l4-4h6v4H11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDriveFolderUpload;
impl IconShape for MdDriveFolderUpload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10zM8 13.01l1.41 1.41L11 12.84V17h2v-4.16l1.59 1.59L16 13.01 12.01 9 8 13.01z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/i3eWngt4Fj/M9wCXht4K/510H827008NvAMf57XQJeG/hr/vUQ/z4vDfw2cIz/HpeA1wb+mn8bxL/fSwO/DRzjv9Yl4LWBv+bfDvEf46WB3waO8V/jEvDawF/z74P4j/PSwG8Dx/jPdQl4beCv+fdD/Md6aeC3gWP857gEvDbw1/zHQPzHe2ngt4Fj/Me6BLw28Nf8x0H853hp4LeBY/zHuAS8NvDX/MdC/Od5aeC3gWP8+1wCXhv4a/7jIf5zvTTw28Ax/m0uAa8N/DX/ORD/+V4a+G3gGP86l4DXBv6a/zyI/xovDfw2cIwXzSXgtYG/5j8X4r/OSwO/DRzjhbsEvDbw1/znQ/zXemngt4FjPH+XgNcG/pr/Goj/ei8N/DZwjOd0CXht4K/5r4P47/HSwG8Dx7jiEvDawF/zXwvx3+elgd/mitcG/pr/eoj/Xi/NFX/Nfw/E/2+I/98Q/78h/n9D/P+G+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82Lw0c53+WXeCv+ddB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E+uNSQbycyZQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFileDownload;
impl IconShape for MdFileDownload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6cHA7fy74f43+e7gLcGXgf4a/59EP+7fBfw3lyxC7wO8Nf82yH+9/gu4L15TrvA6wB/zb8N4n+H7wLem+dvF3gd4K/510P8z/ddwHvzwu0CrwP8Nf86iP/Zvgt4b/5ll4DXBv6afx3E/1zfBbw3/7JLwGsDf82/HuJ/pu8C3pt/2SXgtYG/5t8G8T/PdwHvzb/sEvDawF/zb4f4n+W7gPfmX3YJeG3gr/n3QfzP8V3Ae/MvuwS8NvDX/Psh/mf4LuC9+ZddAl4b+Gv+YyD+47w28Nv8630X8N78yy4Brw38Nf9xEP8x3hv4LuC7gffhRfddwHvzL7sEvDbw1/zHQvz7fRTw1TzbdwPvw7/su4D35l92CXht4K/5j4f49/ku4L15Xt8NvA8v2HcB782/7BLw2sBf858D8W/3XcB784J9N/A+PK/vAt6bf9kl4LWBv+Y/D+Lf5r2B7+Jf9t3A+/Bs3wW8N/+yS8BrA3/Nfy7Ev913A+/Fv+y7gfcBvgt4b/5ll4DXBv6a/3yIf5/vBt6Lf9mtwIP5lz0DeGvgr/mvgfj3+27gvfj3+xvgtYFd/usg/mN8N/Be/Nv9DfDawC7/tRD/cb4beC/+9f4GeG1gl/96iP9Y3w28Fy+6vwFeG9jlvwfiP953A+/Fv+xvgNcGdvnvg/jP8d3Ae/GC/Q3w2sAu/70Q/3m+G3gvntffAK8N7PLfD/Gf67uB9+LZ/gZ4bWCX/xkQ//m+G3gv4G+A1wZ2+Z8D8V/js4GvBnb5nwXx/xvi/zfE/2+I/98Q/78h/n9D/P+G+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+b3wZei/9Zfgd4bf51EP82Lw0c53+WXeCv+ddB/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8ESSFzQafOX1QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFileDownloadDone;
impl IconShape for MdFileDownloadDone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 18h14v2H5v-2zm4.6-2.7L5 10.7l2-1.9 2.6 2.6L17 4l2 2-9.4 9.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/VdXPE+/PdA/Pf5LuC9ueK7gffhvx7iv8d3Ae/Nc/pu4H34r4X4r/ddwHvz/H038D7810H81/ou4L154b4beB/+ayD+63wX8N68aL4beB/+8yH+a3wX8N7863w38D7850L85/su4L35t/lu4H34z4P4z/VdwHvz7/PdwPvwnwPxn+e7gPfmP8Z3A+/DfzzEf47vAt6b/1jfDbwP/7EQ//G+C3hv/nN8N/A+/MdB/Mf6LuC9+c/13cD78B8D8R/nu4D35r/GdwPvw78f4j/GdwHvzX+t7wbeh38fxL/fdwHvzX+P7wbeh387xL/PdwHvzX+v7wbeh38bxL/ddwHvzf8M3w28D/96iH+b9wbemxfNa/Hv8zu8aL4b+G7+dRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/32/z7/Pa/OdB/P+G+P8N8f8b4v83xP9viH+b3wZei/9Zfgd4bf51EP82vw28Fv+z/A7w2vzrIP5tfht4Lf5n+R3gtfnXQfzb/DbwWvzP8jvAa/Ovg/i3+W3gtfif5XeA1+ZfB/Fv89LAcf5n2QX+mn8dxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHqpVCQfNHL+cAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFileUpload;
impl IconShape for MdFileUpload {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADbUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP33cD78X/LV8DfDTPCfH8fTTwVfzf8jHAV/OcEM/fceBW4Bj/N1wCHgzs8pwQL9h7A9/F/w1vA/w0zwvxwr028N3Ag/jf6RnAewO/zfOHeNG8NHCc/112gb/mhUP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRiaFxBSEhO8QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFolder;
impl IconShape for MdFolder {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBaPKe/AT6a/x2+GngpntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+d/ht4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/8dfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmf4ffBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfnf4beB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/nf4beC1eE6/A7w2zwnx/P028Fo8p98BXpv/HX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5n+H3wZei+f0O8Br85wQz99vA6/Fc/od4LX53+G3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf53+G3gtXhOvwO8Ns8J8fz9NvBaPKffAV6b/x1+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+Z/h98GXovn9DvAa/OcEM/fbwOvxXP6HeC1+d/ht4HX4jn9DvDaPCfE8/fdwHvxf8vXAB/Nc0I8fx8NfBX/t3wM8NU8J8Tzdxy4FTjG/w2XgAcDuzwnxAv23sB38X/D2wA/zfNCvHCvDXw38CD+d3oG8N7Ab/P8IV40Lw0c53+XXeCveeEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BrymSQXxzT0sAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFolderOpen;
impl IconShape for MdFolderOpen {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8V/n+8Bvhr4a654aeCjgffi3+53gNfmOSGev98GXov/Hu8DfDfP33sD38W/ze8Ar81zQjx/vw28Fv/1Pgf4bF64zwY+i3+93wFem+eEeP5+G3gt/uudAHZ54Y4DF/nX+x3gtXlOiOfvt4HX4r/W7wCvzYvmt4HX4l/nd4DX5jkhnr/fBl6L/1q/A7w2L5rfBl6Lf53fAV6b54R4/n4beC3+a/018DK8aP4KeGn+dX4HeG2eE+L5+23gtfiv9xDgVl64BwNP51/vd4DX5jkhnr/fBl6L/3q/DbwOL9xvAa/Nv97vAK/Nc0I8f78NvBb/Pb4b+Bhgl+d0HPgu4K35t/kd4LV5Tojn77eB1+K/zy7w3cBfc8VLA+8NHOff7neA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/x6/A/w2V/w1V7w0V7w28Fr82/wO8No8J8Tz99vAa/Ff53uAnwZ+mhfNWwNvDbwXL7rfAV6b54R4/n4beC3+830P8NnArfzbPBj4bOC9+Jf9DvDaPCfE8/fbwGvxn+t9gO/mP8Z7A9/FC/c7wGvznBDP328Dr8V/nu8B3pv/WD8NvBUv2O8Ar81zQjx/vw28Fv953gf4bv5jvTfwXbxgvwO8Ns8J8fz9NvBa/Od5HeC3+Y/12sBv8YL9DvDaPCfE8/fbwGvxn+d1gN/mP9ZrA7/FC/Y7wGvznBDP328Dr8V/nr8GdvmPdRx4aV6w3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/r4beC/+b/ka4KN5Tojn76OBr+L/lo8BvprnhHj+jgO3Asf4v+ES8GBgl+eEeMHeG/gu/m94G+CneV6IF+61ge8GHsT/Ts8A3hv4bZ4/xIvmpYHj/O+yC/w1Lxzi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CMvrIpBamXU8QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFolderShared;
impl IconShape for MdFolderShared {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-5 3c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm4 8h-8v-1c0-1.33 2.67-2 4-2s4 .67 4 2v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFlUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5n/eK8D/DbP6bWB3+I/nnjBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM8/rY4C/5kXzWzyv1wF+m+f02sBv8bxehxfNSwNfxfMSLxjiX2ae1+sAv82Lxjyv1wF+m+f02sBv8bzEi+a1gd/ieYkXDPEvM8/rdYDf5kVjntfrAL/Nc3pt4Ld4XuJF89rAb/G8xAuG+JeZ5/U6wG/zojHP63WA3+Y5vTbwWzwv8aJ5beC3eF7iBUP8y8zzeh3gt3nRmOf1OsBv85xeG/gtnpd40bw28Fs8L/GCIf5l5nm9DvDbvGjM83od4Ld5Tq8N/BbPS7xoXhv4LZ6XeMEQ/zLzvF4H+G1eNOZ5vQ7w2zyn1wZ+i+clXjSvDfwWz0u8YIh/mXlerwP8Ni8a87xeB/htntNrA7/F8xIvmtcGfovnJV4wxL/MPK/XAX6bF415Xq8D/DbP6bWB3+J5iRfNawO/xfMSLxjiX2ae1+sAv82Lxjyv1wF+m+f02sBv8bzEi+a1gd/ieYkXDPEvM8/rdYDf5kVjntfrAL/Nc3pt4Ld4XuJF89rAb/G8xAuG+JeZ/3ivA/w2z+m1gd/iP554wRD/MvMf73WA3+Y5vTbwW/zHEy8Y4l9m/uO9DvDbPKfXBn6L/3jiBUP8y8x/vNcBfpvn9NrAb/EfT7xgiH+Z+Y/3OsBv85xeG/gt/uOJFwzxL3tt/uP9NbDLczoOvDT/8X6bFwzx/xvi/zfE/2+I/98Q/7LX4j/e3wC7PKfjwEvxH+93eMEQ/zLzH+91gN/mOb028Fv8xxMvGOJfZv7jvQ7w2zyn1wZ+i/944gVD/MvMf7zXAX6b5/TawG/xH0+8YIh/mfmP9zrAb/OcXhv4Lf7jiRcM8S8z//FeB/htntNrA7/FfzzxgiH+ZeZ5vQ7w27xozPN6HeC3eU6vDfwWz0u8aF4b+C2el3jBEP8y87xeB/htXjTmeb0O8Ns8p9cGfovnJV40rw38Fs9LvGCIf5l5Xq8D/DYvGvO8Xgf4bZ7TawO/xfMSL5rXBn6L5yVeMMS/zDyv1wF+mxeNeV6vA/w2z+m1gd/ieYkXzWsDv8XzEi8Y4l9mntfrAL/Ni8Y8r9cBfpvn9NrAb/G8xIvmtYHf4nmJFwzxLzPP63WA3+ZFY57X6wC/zXN6beC3eF7iRfPawG/xvMQLhviXmef1OsBv86Ixz+t1gN/mOb028Fs8L/GieW3gt3he4gVD/MvM83od4Ld50Zjn9TrAb/OcXhv4LZ6XeNG8NvBbPC/xgiH+ZeZ5vQ7w27xozPN6HeC3eU6vDfwWz0u8aF4b+C2el3jBEP8y87xeB/htXjTmeb0O8Ns8p9cGfovnJV40rw38Fs9LvGCIf5l5Xh8N/DUvmt/meb0O8Ns8p9cGfovn9dq8aF4a+Gqel3jBEP8y8x/vdYDf5jm9NvBb/McTLxjiX2b+470O8Ns8p9cGfov/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+8f8ZBob/zVwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGridView;
impl IconShape for MdGridView {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M3 3v8h8V3H3zm6 6H5V5h4v4zm-6 4v8h8v-8H3zm6 6H5v-4h4v4zm4-16v8h8V3h-8zm6 6h-4V5h4v4zm-6 4v8h8v-8h-8zm6 6h-4v-4h4v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/eZ4BHAeO8by+G3gf/uMhXnSvDfwW/3l+B/ho4LeBYzyv7wbeh/9YiBfdawO/xX+e3wFeG3hp4LeBYzyv7wbeh/84iBfdawO/xX+e3wFemyteGvht4BjP67uB9+E/BuJF99rAb/Gf53eA1+bZXhr4beAYz+u7gffh3w/xontt4Lf4z/M7wGvznF4a+G3gGM/ru4H34d8H8aJ7beC3+M/zO8Br87xeGvht4BjP67uB9+HfDvGie23gt/jP8zvAa/P8vTTw28Axntd3A+/Dvw3iRffawG/xn+d3gNfmBXtp4LeBYzyv7wbeh389xIvutYHf4j/P7wCvzQv30sBvA8d4Xt8NvA//OogX3WsDv8V/nt8BXpt/2UsDvw0c43l9N/A+vOgQL7rXBn6L/zy/A7w2L5qXBn4bOMbz+m7gfXjRIF50rw38Fv95doG/5kX3YODBPH+vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf4j/E6wG/znF4b+C3+Y7wO8Nv8yxAvutcGfov/GK8D/DbP6bWB3+I/xusAv82/DPGie23gt/i3eQZwK8/20cBf85xeGvhqntdr8a/3OsBv8y9DvOheG/gt/vX+BnhtYJd/G/Ov9zrAb/MvQ7zoXhv4Lf71Xgb4a/7tzL/e6wC/zb8M8aJ7beC3+NcT/z7mX+91gN/mX4Z40b028Fv864l/H/Ov9zrAb/MvQ7zoXhv4Lf71xL+P+dd7HeC3+ZchXnSvDfwW/3rief028Fr853kd4Lf5lyFedK8N/Bb/euJ5/TbwWvzneR3gt/mXIV50rw38Fv964nl9NfDSvHAvDRzj3+Z1gN/mX4Z40b028Fv864l/m+PAbwMvxb/e6wC/zb8M8aJ7beC3+NcT/3YvDfwV/3qvA/w2/zLEi+61gd/iX0/8+5h/vdcBfpt/GeJF99rAb/Gv9z7Ad/NvZ/71Xgf4bf5liBfdawO/xb/eLvDRwPfwb2P+9V4H+G3+ZYgX3WsDv8V/nt8BXpvnZf71Xgf4bf5liBfdawO/xX+e3wFem+dl/vVeB/ht/mWIF91rA7/Fv97v8KL5a+CjeV7mX+91gN/mX4Z40b028Fv864l/H/Ov9zrAb/MvQ7zoXhv4Lf71xL+P+dd7HeC3+ZchXnSvDfwW/3ri38f8670O8Nv8yxAvutcGfot/PfFv997Ad/Gv9zrAb/MvQ7zoXhv4Lf71xL/NewFfDRznX+91gN/mX4Z40b028Fv8x3gd4Ld5Tq8N/Bb/MV4H+G3+ZYgX3WsDv8V/jNcBfpvn9NrAb/Ef43WA3+ZfhnjRvTbwW/zH+Gjgr3lOLw18Nf8xXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7aeCv+N/hZYC/5l+G+Ne5FXgQ/7M9A3gwLxrEv85bAz/F/2yvA/w2LxrEv957A18NHON/lkvAewM/zYsO8W9zHHhv4KWBB/Pf61bgr4HvBnb510H8/4b4/w3x/xvi/zfE/2/8IwEvtUF9zlbKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRequestQuote;
impl IconShape for MdRequestQuote {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2V8L14,2z M15,12h-4v1h3c0.55,0,1,0.45,1,1v3 c0,0.55-0.45,1-1,1h-1v1h-2v-1H9v-2h4v-1h-3c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h1V9h2v1h2V12z M13,8V3.5L17.5,8H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvznegbwIP7r/A7w2jwnxPP328Br8Z/nfYCfBn4beCmev2cAD+IFuwQc40X3O8Br85wQz99vA6/Ff473Ab6bK44Dvw28FM/pe4D3Br4beC+e1yXgtYGXBr6LF83vAK/Nc0I8f78NvBb/8d4H+G6e03Hgt4GX4orvAd6bZ/tu4L14tkvAawN/zRXvDXwX/7LfAV6b54R4/n4beC3+Y70P8N08f68N/BbwPcB787y+G3gv4BLw2sBf85zeG/guXrjfAV6b54R4/n4beC3+47wP8N08fy8N/BawCzyEF+yrge8G/prn77uB9+IF+x3gtXlOiOfvt4HX4kXzN1zxUjx/7wN8N8/fSwO/BRzniu8G3od/vfcGvosX7neA1+Y5IZ6/3wZei3/Z3wCvzRW/DbwUz+l9gO/m+TsOPB04znP6buB9eNG9N/Bd/Mt+B3htnhPi+ftt4LV44f4GeG1glyuOA78NvBRXvA/w3Tx/x4HfAl6a5++7gffhX/bewHfxovkd4LV5Tojn77eB1+IF+xvgtYFdntNx4LeBrwa+m+fvOPBbwEvzwn0N8NG8YK8N/BYvut8BXpvnhHj+fht4LV6w7wbeh3+948BvAS/NC3cJeG3gr3nhvht4L140vwO8Ns8J8fz9NvBavHDfDbwPL7rjwG8BL80Ldwl4beCvedF8N/Be/Mt+B3htnhPi+ftt4LX4l3038D78y44DvwW8NC/cJeC1gb/mX+e7gffihfsd4LV5Tojn77eB1+JF893A+/CCHQd+C3hpXrhLwGsDf83z99rAb/OCfTfwXrxgvwO8Ns8J8fz9NvBavOi+G3gfntdx4LeAl+ZF8z7Ad/O83hv4LuC7gffheR0Hfgt4aV6w3wFem+eEeP5+G3gt/nW+G3gfnu048FvAS/Ov8z7Ad/Ns7w18F8/23cD78GzHgd8CXpoX7neA1+Y5IZ6/3wZei3+97wbeBzgO/Bbw0vzbvA/w3cB7A9/F8/pu4H2A48BvAS/Nv+x3gNfmOSGev98GXot/m+8GXhp4af59vht4b16w7wZeGnhpXjS/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz993A+/F/y1fA3w0zwnx/H008FX83/IxwFfznBDP33HgVuAY/zdcAh4M7PKcEC/YewPfxf8NbwP8NM8L8cK9NvDdwIP43+kZwHsDv83zh3jRvDRwnP9ddoG/5oVD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EM5W+QdPSXEMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRuleFolder;
impl IconShape for MdRuleFolder {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,6h-8l-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M7.83,16L5,13.17 l1.41-1.41l1.41,1.41l3.54-3.54l1.41,1.41L7.83,16z M17.41,13L19,14.59L17.59,16L16,14.41L14.41,16L13,14.59L14.59,13L13,11.41 L14.41,10L16,11.59L17.59,10L19,11.41L17.41,13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8V/rd8BXpt/2XsD38W/3u8Ar81zQjx/vw28Fv+1fgd4bV407w18F/86vwO8Ns8J8fz9NvBa/Nf6HeC1edG9N/BdvOh+B3htnhPi+ftt4LX4r/U7wGvzr/PewHfxovkd4LV5Tojn77eB1+K/1u8Ar82zvTbwWzzbXwOvA+zynN4b+C7+Zb8DvDbPCfH8/TbwWvzX+h3gtXm21wZ+i+f018DrALs8p/cGvosX7neA1+Y5IZ6/3wZei/9avwO8Ns/22sBv8bz+GngdYJfn9NHAV/GC/Q7w2jwnxPP328Br8V/rd4DX5tleG/gtnr+/Bl4H2OXZXhv4LV6w3wFem+eEeP5+G3gt/mv9DvDaPNtrA7/FC/YzwFvzbK8N/BYv2O8Ar81zQjx/vw28Fv+1fgd4bZ7ttYHf4gX7HeC1ebbXBn6LF+x3gNfmOSGev98GXov/Wr8DvDbP9trAb/GC/Q7w2jzbawO/xQv2O8Br85wQz99vA6/Ff63fAV6bZ3tt4Ld4wX4HeG2e7bWB3+IF+x3gtXlOiOfvt4HX4r/W7wCvzbO9NvBbvGC/A7w2z/bawG/xgv0O8No8J8Tz99vAa/Ff63eA1+bZXhv4LV6w3wFem2d7beC3eMF+B3htnhPi+ftt4LX4r/U7wGvzbK8N/BYv2O8Ar82zvTbwW7xgvwO8Ns8J8fz9NvBa/Nf6HeC1ebbXBn6LF+x3gNfm2V4b+C1esN8BXpvnhHj+fht4Lf5r/Q7w2jzbawO/xQv2O8Br82yvDfwWL9jvAK/Nc0I8f78NvBb/tX4HeG2e7Tjw0rxgu8Bf82yvDfwWL9jvAK/Nc0I8f78NvBb/tX4HeG3+7V4b+C1esN8BXpvnhHj+fht4Lf5r/Q7w2vzbvTbwW7xgvwO8Ns8J8fz9NvBa/Nf6HeC1+bd7beC3eMF+B3htnhPi+ftt4LX4r7UL/DX/dseBl+YF+x3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8Cf+ioQd033b0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSnippetFolder;
impl IconShape for MdSnippetFolder {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.88,10.5l1.62,1.62v3.38l-3,0v-5H15.88z M22,8v10c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2L2.01,6C2.01,4.9,2.9,4,4,4h6l2,2 h8C21.1,6,22,6.9,22,8z M19,11.5L16.5,9H13v8l6,0V11.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/Nf4aeBrgN/mPwfiRfPewHfx3+ezgc/hPx7iX/Zg4On89/tu4H34j4X4l3038F78z/DdwPvwHwfxL7sIHOd/ju8G3of/GIh/mfmf57uB9+HfD/EvM/8zfTfwPvz7IP5l5n+u7wbeh387xL/M/M/23cD78G+D+JeZ//m+G3gf/vUQ/zLzorkE/DX/MV4aOMa/zncD78O/DuJfZl40vwO8Nv8xfht4Lf71vht4H150iH+ZedH8DvDa/Mf4beC1+Lf5buB9eNEg/mXmRfM7wGvzH+O3gdfi3+67gffhX4b4l5kXze8Ar81/jN8GXot/n88BPpsXDvEvMy+a3wFem/8Yvw28Fv9+rwP8Ni8Y4l9m/vf6GeCtecEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvOi+Rvgo/mv8dXAS/GiES8Y4l9mXjS/A7w2/zV+G3gtXjTiBUP8y8yL5neA1+a/xm8Dr8WLRrxgiH+ZedH8DvDa/Nf4beC1eNGIFwzxLzMvmt8BXpv/Gr8NvBYvGvGCIf5l5kXz18BH81/jq4GX5kUjXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/MvGh+B3ht/mv8NvBavGjEC4b4l5kXze8Ar81/jd8GXosXjXjBEP8y86L5HeC1+a/x28Br8aIRLxjiX2ZeNL8DvDb/NX4beC1eNOIFQ/zLzIvmd4DX5r/GbwOvxYtGvGCIf5l50ewCf81/jZcGjvOiES8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJftgsc43+nS8BxXjDEv+y7gffif6fvAd6bFwzxL3sw8HT+d3oIcCsvGOJF897Ad/G/y/sA380Lh3jRPRj4bOCtgWP8z3QJ+Gngs4Fb+Zch/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/Ylm1BHgDlJAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextSnippet;
impl IconShape for MdTextSnippet {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.41,8.41l-4.83-4.83C15.21,3.21,14.7,3,14.17,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V9.83 C21,9.3,20.79,8.79,20.41,8.41z M7,7h7v2H7V7z M17,17H7v-2h10V17z M17,13H7v-2h10V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesLcGfor/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP7eG/gu/nN9N/A+/PdCPH9fDXwU//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxYvmd/jv9Vq8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/Fi+Z1+I/31cBL8R/rd4DX5jkhnr/fBl6LF434j/fbwGvxH+t3gNfmOSGev98GXosXjfiP99vAa/Ef63eA1+Y5IZ6/3wZeixeN+I/328Br8R/rd4DX5jkhnr/fBl6LF434j/fbwGvxH+t3gNfmOSGev98GXosXzWvzH++rgZfmP9bvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fdwHvxf8vXAB/Nc0I8fx8NfBX/t3wM8NU8J8Tzdxy4FTjG/w2XgAcDuzwnxAv23sB38X/D2wA/zfNCvHCvDXw38CD+d3oG8N7Ab/P8IV40Lw0c53+XXeCveeEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BQF5oQcN+HfIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTopic;
impl IconShape for MdTopic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/NseB9wJeGngw//H+GvgY/vMh/vXeG/gq4Dj/ub4beB/+cyH+dV4b+C3+63w38D7850H86zwdeDD/tb4beB/+cyBedC8N/BX/Pb4beB/+4yFedK8N/BbP63f4j3EceClesO8G3of/WIgX3WsDv8XzEv8xXhv4LV647wbeh/84iBfdawO/xfMS/zFeG/gt/mXfDbwP/zEQL7rXBn6L5yX+Y7w28Fu8aL4beB/+/RAvutcGfovnJf5jvDbwW7zovht4H/59EC+61wZ+i+cl/mO8NvBb/Ot8N/A+/NshXnSvDfwWz0v8x3ht4Lf41/tu4H34t0G86F4b+C2el/iP8drAb/Fv893A+/Cvh3jRvTbwWzwv8R/jtYHf4t/uu4H34V8H8aJ7beC3eF7iP8ZrA7/Fv893A+/Diw7xontt4Ld4XuI/xmsDv8W/33cD78OLBvGie23gt3he4j/GawO/xX+M1wF+m38Z4kX32sBv8bzEf4zXBn6L/xivA/w2/zLEi+61gd/ieYn/GK8N/Bb/MV4H+G3+ZYgX3WsDv8XzEv+9zPN6HeC3+ZchXnSvDfwWz0v89zLP63WA3+ZfhnjRvTbwWzwv8d/LPK/XAX6bfxniRffawG/xvMR/L/O8Xgf4bf5liBfdawO/xfMS/73M83od4Lf5lyFedK8N/BbPS/zbvTRX/DX/duZ5vQ7w2/zLEC+61wZ+i+cl/m1eGvgtrngd4K/5tzHP63WA3+ZfhnjRvTbwWzwv8a/30sBvAce5Yhd4HeCv+dczz+t1gN/mX4Z40b028Fs8L/Gv89LAbwHHeU67wOsAf82/jnlerwP8Nv8yxIvutYHf4nmJF91LA78FHOf52wVeB/hrXnTmeb0O8Nv8yxAvutcGfovnJV40Lw38FnCcF24XeB3gr3nRmOf1OsBv8y9DvOheG/gtnpf4l7008FvAcV40u8DrAH/Nv8w8r9cBfpt/GeJF99rAb/G8xAv30sBvAcf519kFXgf4a14487xeB/ht/mWIF91rA7/F8xIv2EsDvwUc599mF3gd4K95wczzeh3gt/mXIV50rw38Fs9LPH8vDfwWcJx/n13gdYC/5vkzz+t1gN/mX4Z40b028Fs8L/G8Xhr4LeA4z98l4BjP6RJwjOdvF3gd4K95XuZ5vQ7w2/zLEC+61wZ+i+clntdLA78NHON5vQ/w3sBr8Zx+B/hu4Lt4XpeA1wb+mudlntfrAL/Nvwzxontt4Ld4XuL5e2ngt4FjPNv7AN8N/DbwWjyn3wFeG3hv4Lt4tkvAawN/zfNnntfrAL/Nvwzxontt4Ld4XuIFe2ngt4FjwPsA380Vvw28Fs/pd4DX5or3Br4LuAS8NvDXvGDmeb0O8Nv8yxAvutcGfovnJV64lwZeGvhunu23gdfiOf0O8No823sDfw38NS+ceV6vA/w2/zLEi+61gd/ieYl/vd8GXovn9DvAa/OvZ57X6wC/zb8M8aJ7beC3eF7iX++3gdfiOf0O8Nr865nn9TrAb/MvQ7zoXhv4LZ6X+Nf7beC1eE6/A7w2/3rmeb0O8Nv8yxAvutcGfovnJf71fht4LZ7T7wCvzb+eeV6vA/w2/zLEi+61gd/ieYl/vd8GXovn9DvAa/OvZ57X6wC/zb8M8aJ7beC3eF7iX++3gdfiOf0O8Nr865nn9TrAb/MvQ7zoXhv4LZ6X+Nf7auCleU5/DXw0/3rmeb0O8Nv8yxAvutcGfovnJf57mef1OsBv8y9DvOheG/gtnpf472We1+sAv82/DPGie23gt3hev81/r9fmeb0O8Nv8yxAvupcG/or/HV4G+Gv+ZYh/nVuBB/E/2zOAB/OiQfzrvDXwU/zP9jrAb/OiQfzrvTfw1cAx/me5BLw38NO86BD/NseB9wZeGngw/71uBf4a+G5gl38dxP9viP/fEP+/If5/Q/z/xj8CSvjiQYgJFjEAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUploadFile;
impl IconShape for MdUploadFile {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm4 18H6V4h7v5h5v11zM8 15.01l1.41 1.41L11 14.84V19h2v-4.16l1.59 1.59L16 15.01 12.01 11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3UceCvgrYEHAy/NFX8N3Ar8NPAzwC7/NRD/dT4K+GzgOC/cLvDRwPfwnw/xn+848F3AW/Ov89PA+wC7/OdB/Oc6DvwW8NL82/w18DrALv85EP+5vgt4b/59vht4H/5zIP7zvDbwW/zHeB3gt/mPh/jP81vAa/Mf47eB1+E/HuI/x4OBp/Mf6yHArfzHQvzn+Gjgq/iP9THAV/MfC/Gf47OBz+I/1ucAn81/LMR/jp8G3or/WD8DvDX/sRD/OT4b+Cz+Y30O8Nn8x0L85/hs4LP4j/U5wGfzHwvxn+Olgb/iP9bLAH/NfyzEf55d4Bj/MS4Bx/mPh/jP89nAZ/Ef42OAr+Y/HuI/163Ag/j3+RvgpfnPgfjP9dLAbwPH+Le5BLw28Nf850D853tp4LeBY/zrXAJeG/hr/vMg/ms8GPhu4LV40fwO8N7ArfznQvzXem3go4G34vn7GeCrgd/mvwbiv89r85x+m/96iP/fEP+/If5/Q/z/hvi3ey2e0+/wX+u1eE6/w78e4l/ntYGPAt6a5++nga8Bfpv/HK8NfBTw1jx/Pw18DfDbvGgQL5oHA98FvDYvmt8G3ge4lf8YDwa+C3htXjS/DbwPcCsvHOJf9tLAbwHH+dfZBV4H+Gv+fV4a+C3gOP86u8DrAH/NC4Z44V4a+C3gOP82u8DrAH/Nv81LA78FHOffZhd4HeCvef4QL9zTgQfz7/PXwMvwb/N04MH8+/w18DI8f4gX7LOBz+I/xscAX82/zmcDn8V/jI8BvprnhXjBLgLH+Y+xC5zgX+cicJz/GLvACZ4X4vl7aeCv+I/1MsBf86J5aeCv+I/1MsBf85wQz99nA5/Ff6zPAT6bF81nA5/Ff6zPAT6b54R4/j4b+Cz+Y30O8Nm8aD4b+Cz+Y30O8Nk8J8Tz99PAW/Ef62eAt+ZF89PAW/Ef62eAt+Y5IZ6/zwY+i/9YnwN8Ni+azwY+i/9YnwN8Ns8J8fx9NPBV/Mf6GOCredF8NPBV/Mf6GOCreU6I5+/BwNP5j/UQ4FZeNA8Gns5/rIcAt/KcEC/YbwOvxX+M3wFem3+d3wZei/8YvwO8Ns8L8YK9NvBb/Md4HeC3+dd5beC3+I/xOsBv87wQL9x3A+/Fv8/3AO/Nv813A+/Fv8/3AO/N84d44Y4Dvw28FP82fwO8NrDLv81x4LeBl+Lf5m+A1wZ2ef4Q/7LjwHcDb8W/zs8A7w3s8u9zHPhu4K341/kZ4L2BXV4wxIvuvYGvBo7xwl0CPhr4bv5jvTfw1cAxXrhLwEcD382/DPGvcxx4a+CtgQcDL8UVfwPcCvw08NPALv85jgNvDbw18GDgpbjib4BbgZ8GfhrY5UWD+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CKq+fQaFaC/oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWorkspacesFilled;
impl IconShape for MdWorkspacesFilled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 13c-2.2 0-4 1.8-4 4s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4zm6-10C9.8 3 8 4.8 8 7s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4zm6 10c-2.2 0-4 1.8-4 4s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3UceCvgrYEHAy/NFX8N3Ar8NPAzwC7/NRD/dT4K+GzgOC/cLvDRwPfwnw/xn+848F3AW/Ov89PA+wC7/OdB/Oc6DvwW8NL82/w18DrALv85EP+5vgt4b/59vht4H/5zIP7zvDbwWzx/l4DvBv6aK14aeG/gGM/f6wC/zX88xH+e3wJem+f1PcBHA7s8p+PAVwPvxfP6beB1+I+H+M/xYODpPK/fAV6bF+63gdfieT0EuJX/WIj/HB8NfBXP6yHArbxwDwaezvP6GOCr+Y+F+M/x2cBn8Zz+BnhpXjR/DbwUz+lzgM/mPxbiP8dPA2/Fc/od4LV50fw28Fo8p58B3pr/WIj/HJ8NfBbP6XeA1+ZF89vAa/GcPgf4bP5jIf5zfDbwWTyvE8AuL9xx4CLP63OAz+Y/FuI/x0sDf8Xz+hzgs3nhPhv4LJ7XywB/zX8sxH+eXeAYz+t9gO/m+Xtv4Lt4XpeA4/zHQ/zn+Wzgs3j+vhv4GuCvueK1gPcG3pvn72OAr+Y/HuI/163Ag/j3+RvgpfnPgfjP9dLAbwPH+Le5BLw28Nf850D853tp4LeBY/zrXAJeG/hr/vMg/ms8GPhu4LV40fwO8N7ArfznQvzXem3go4G34vn7GeCrgd/mvwbiv89r85x+m/96iP/fEP+/If5/Q/z/hvi3ey2e0+/wX+u1eE6/w78e4l/ntYGPAt6a5++nga8Bfpv/HK8NfBTw1jx/Pw18DfDbvGgQL5oHA98FvDYvmt8G3ge4lf8YDwa+C3htXjS/DbwPcCsvHOJf9tLAbwHH+dfZBV4H+Gv+fV4a+C3gOP86u8DrAH/NC4Z44V4a+C3gOP82u8DrAH/Nv81LA78FHOffZhd4HeCvef4QL9zTgQfz7/PXwMvwb/N04MH8+/w18DI8f4gX7LOBz+L5+x7gq4G/5oqXBj4aeC+ev48Bvpp/nc8GPovn73uArwb+miteGvho4L14/j4G+GqeF+IFuwgc53m9D/DdPH/vDXwXz2sXOMG/zkXgOM/rfYDv5vl7b+C7eF67wAmeF+L5e2ngr3henwN8Ni/cZwOfxfN6GeCvedG8NPBXPK/PAT6bF+6zgc/ieb0M8Nc8J8Tz99nAZ/G8TgC7vHDHgYs8r88BPpsXzWcDn8XzOgHs8sIdBy7yvD4H+GyeE+L5+2zgs3hOvwO8Ni+a3wZei+f0OcBn86L5bOCzeE6/A7w2L5rfBl6L5/Q5wGfznBDP308Db8Vz+h3gtXnR/DbwWjynnwHemhfNTwNvxXP6HeC1edH8NvBaPKefAd6a54R4/j4b+Cye018DL8OL5q+Al+Y5fQ7w2bxoPhv4LJ7TXwMvw4vmr4CX5jl9DvDZPCfE8/fRwFfxvB4C3MoL92Dg6TyvjwG+mhfNRwNfxfN6CHArL9yDgafzvD4G+GqeE+L5ezDwdJ7XbwOvwwv3W8Br87weAtzKi+bBwNN5Xr8NvA4v3G8Br83zeghwK88J8YL9NvBaPK/vBj4G2OU5HQe+C3hrntfvAK/Nv85vA6/F8/pu4GOAXZ7TceC7gLfmef0O8No8L8QL9trAb/H87QLfDfw1V7w08N7AcZ6/1wF+m3+d1wZ+i+dvF/hu4K+54qWB9waO8/y9DvDbPC/EC/fdwHvx7/M9wHvzb/PdwHvx7/M9wHvz/CFeuOPAbwMvxb/N3wCvDezyb3Mc+G3gpfi3+RvgtYFdnj/Ev+w48N3AW/Gv8zPAewO7/PscB74beCv+dX4GeG9glxcM8aJ7b+CrgWO8cJeAjwa+m/9Y7w18NXCMF+4S8NHAd/MvQ/zrHAfeGnhr4MHAS3HF3wC3Aj8N/DSwy3+O48BbA28NPBh4Ka74G+BW4KeBnwZ2edEg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+M8epBhYai0wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWorkspacesOutline;
impl IconShape for MdWorkspacesOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 15c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2m0-2c-2.2 0-4 1.8-4 4s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4zm6-8c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2m0-2C9.8 3 8 4.8 8 7s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4zm6 12c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2m0-2c-2.2 0-4 1.8-4 4s1.8 4 4 4 4-1.8 4-4-1.8-4-4-4z",
            }
        }
    }
}
