use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFMElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDwn8cKZZ/sb4L2Bv+YFezDw3sBn8Zx+B3htnu21gd/i2Z4BvDSwy7O9N/BdPCfxgiH+ZeY5iRfOXPEM4KWBXV40Pw28Fc/2O8Br82yvDfwWV1wCXhv4a57tpYHfAo7znMQLhviXmeckXjhzxe8Ar82L7r2B7+LZfgd4bZ7ttYHf4or3Ab6bZzsO/Bbw0jwv8YIh/mXmOYkXzlzxO8Br86J7beC3eLbfAV6bZ3tt4LeA7wHem+f0XcB78/yJFwzxLzMvnHhO5orfAV6bZ3tt4Ld4tt8BXptne23gt3i23wFem2d7beCrgZfmOb038F28YOIFQ/zLzAsnnpO54neA1+bZXhv4LZ7td4DX5tleG/gtnu13gNfm2V4a2AVu5dleGvgt4DgvmHjBEP8y88KJ52Su+B3gtXm21wZ+i2f7HeC1ebbXBn6LZ/sd4LV5wY4DvwW8NC+ceMEQ/zLzwonnZK74HeC1ebbXBn6LZ/sd4LV5ttcGfotn+x3gtXnBvgt4b/5l4gVD/MvMCyeek7nid4DX5tleG/gtnu13gNfm2V4b+C2e7XeA1+YF+27gvfiXiRcM8S8zL5x4TuaK3wFem2d7beC3eLbfAV6bZ3tt4Ld4tt8BXpsX7Djw28BL8cKJFwzxLzMvnHhO5orfAV6bZ3tt4Ld4tt8BXptne23gt3i23wFem2d7aWAXuJVne2ngt4FjvGDiBUP8y8wLJ56TueJ3gNfm2V4b+C2e7XeA1+bZXhv4LZ7td4DX5tleG/gq4GV4Tu8NfBcvmHjBEP8y88KJ52Su+B3gtXnRvTbwWzzb7wCvzbO9NvBbwHcD78Nz+m7gvXj+xAuG+JeZF048J3PF7wCvzYvuvYHv4tl+B3htnu21gd/iivcBvptnOw78NvBSPC/xgiH+ZeaFE8/JXLELPATY5UXzU8Bb82y/A7w2z/bawG9xxS7wOsBf82wvDfw2cIznJF4wxL/MvHDiOZln+2vgfYC/5gV7MPBewGfznH4HeG2e7bWB3+LZbgVeBtjl2d4b+C6ek3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/sl3gGP87XQKO84Ih/mXfDbwX/zt9D/DevGCIf9mDgafzv9NDgFt5wRAvmvcGvov/Xd4H+G5eOMSL7sHAZwNvDRzjf6ZLwE8Dnw3cyr8M8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E5uq8QWoYIB0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md10k;
impl IconShape for Md10k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 10.5h1.5v3H10zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM7.5 15H6v-4.5H4.5V9h3v6zm5.5-1c0 .55-.45 1-1 1H9.5c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1H12c.55 0 1 .45 1 1v4zm6.5 1h-1.75L16 12.75V15h-1.5V9H16v2.25L17.75 9h1.75l-2.25 3 2.25 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEF0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzriBfOPK/XAX6bZ/tt4LV4tu8B3pvn9FPAW/MvEy8Y4l9m/nXEC2ee1+sAv82z/TbwWlzxN8BL85w+GvgqXjTiBUP8y8y/jnjhzPN6HeC3ebbfBl4LuAS8NHArz/bSwF/xohMvGOJfZv51xAtnntfrAL/Ns/028FrA6wC/zbMdB/4KeDAvOvGCIf5l5t9HPCfzvF4H+G2e7beB3wY+m+f0U8Bb868jXjDEv8z8+4jnZJ7X6wC/zbN9NPDVPKePBr6Kfz3xgiH+ZebfRzwn87xeB/htXrCXBv6KfxvxgiH+ZebfRzwn87xeB/htnr/jwF8BD+bfRrxgiH+Z+fcRz8k8r9cBfpvn7zjw18CD+LcRLxjiX2b+fcRzMs/rdYDf5gV7aeCv+LcRLxjiX2b+fcRzMs/rdYDf5tk+GvhqntNHA1/Fv554wRD/MvPvI56TeV6vA/w2z/bbwG8Dn81z+mngrfjXES8Y4l9m/n3EczLP63WA3+bZfht4LeB1gN/m2Y4Dfw08iBedeMEQ/zLz7yOek3lerwP8Ns/228BrAbvAywC38mwvDfwVLzrxgiH+ZebfRzwn87xeB/htnu23gdfiir8GXobn9NHAV/GiES8Y4l9m/n3EczLP63WA3+bZfht4LZ7tu4H34Tn9NPBW/MvEC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JftAsf43+kScJwXDPEv+27gvfjf6XuA9+YFQ/zLHgw8nf+dHgLcyguGeNG8N/Bd/O/yPsB388IhXnQPBj4beGvgGP8zXQJ+Gvhs4Fb+ZYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I1REhkGCb0+/AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md1k;
impl IconShape for Md1k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8.5 12H9v-4.5H7.5V9h3v6zm7 0h-1.75L14 12.75V15h-1.5V9H14v2.25L15.75 9h1.75l-2.25 3 2.25 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAENklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxw4oUzz0k8J/OcXgf4bZ7twcBfAcf5txEvGOJfZl448cKZ5ySek3m2zwE+m+f0V8BL828nXjDEv8y8cOKFM89JPCdzxc8Ab81z+mrgo3hefwPscsVr8cKJFwzxLzMvnHjhzHMSz8nAM4CXBnZ5trcGforn73WA3+YK88KJFwzxLzP/OuI5meckntMu8NrAX/NsDwb+CjjO8/c6wG9zhXnhxAuG+JeZfx3xnMxzEs/pvYHv5jn9FfDSvGCvA/w2V5gXTrxgiH+Z+dcRz8k8J/HCfTXwUTynvwF2ebaPBv6aK36b5/RaPCfxgiH+ZeZfRzwn85zEC/bWwE/xvF4H+G1eNOY5iRcM8S8z/zriOZnnJF6wtwZ+iuf1OsBv86Ixz0m8YIh/mfnXEc/JPCfxwn018FE8p78Gdnm2jwH+mit+i+f02jwn8YIh/mXmX0c8J/OcxHN6b+C7eU5/DbwUL9jrAL/NFeaFEy8Y4l9m/nXEczLPSTyni8DrAH/Nsz0Y+GvgGM/f6wC/zRXmhRMvGOJfZv51xHMyz0k8JwO3Ai8D7PJsbw38FM/f6wC/zRXmhRMvGOJfZv51xHMyz0k8J3PFTwNvw3P6auCjeF5/DexyxWvzwokXDPEvM/864jmZ5ySek3m2zwE+m+f018BL8W8nXjDEv8z864jnZJ6TeE7mOb0O8Ns824OBvwaO8W8jXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL9sFzjG/06XgOO8YIh/2XcD78X/Tt8DvDcvGOJf9mDg6fzv9BDgVl4wxIvmvYHv4n+X9wG+mxcO8aJ7MPDZwFsDx/if6RLw08BnA7fyL0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFRgJJBKMBvKAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md1kPlus;
impl IconShape for Md1kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 15H7.5v-4.5H6V9h3v6zm4.75 0L12 12.75V15h-1.5V9H12v2.25L13.75 9h1.75l-2.25 3 2.25 3h-1.75zm5.75-2.5H18V14h-1v-1.5h-1.5v-1H17V10h1v1.5h1.5v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzLXgf4bf5lLw18N/BSPK/fAV6bZ3tt4Ld4tmcALw3s8mzvDXwXL5x4wRD/MvMvex3gt3nRHAduBY7xnH4HeG2e7bWB3+KKS8BrA3/Ns7008FvAcV448YIh/mXmX/Y6wG/zovtt4LV4Tr8DvDbP9trAb3HF+wDfzbMdB34LeGn+ZeIFQ/zLzL/sdYDf5kX328Br8Zx+B3htnu21gd8Cvgd4b57TdwHvzYtGvGCIf5n593sd4Ld5tt8GXovn9DvAa/Nsrw18NfDSPKf3Br6LF514wRD/MvPv9zrAb/Nsvw28Fs/pd4DX5tleGtgFbuXZXhr4LeA4LzrxgiH+ZeZF9zs8fx8N/DXP9tvAa/Gcfgd4bV6w48BvAS/Nv454wRD/MvMvuwS8NvDXvGj+CnhpntPvAK/NC/ZdwHvzrydeMMS/zPzLvgb4aF40nwV8Ns/rd4DX5gX7buC9+NcTLxjiX2b+Za8D/DbP9tXAS/G8jgMvzfP3O8Br84IdB34beCn+dcQLhviXmX/Z6wC/zbP9NvBa/Ov8DvDaPNtLA7vArTzbSwO/DRzjRSdeMMS/zPzLXgf4bZ7tt4HX4l/nd4DX5tleG/gq4GV4Tu8NfBcvOvGCIf5l5l/2OsBv86L7beC1eE6/A7w2z/bawG8B3w28D8/pu4H34kUjXjDEv8z8y14H+G1edL8NvBbP6XeA1+bZXhv4La54H+C7ebbjwG8DL8W/TLxgiH+Z+Ze9DvDbvOh+G3gtntPvAK/Ns7028FtcsQu8DvDXPNtLA78NHOOFEy8Y4l9m/mWvA/w2L7rfBl6L5/Q7wGvzbK8N/BbPdivwMsAuz/bewHfxwokXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHBMGaQQpbU44AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md2k;
impl IconShape for Md2k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 9.5H8v1h3V15H6.5v-2.5c0-.55.45-1 1-1h2v-1h-3V9H10c.55 0 1 .45 1 1v1.5c0 .55-.45 1-1 1zm8 2.5h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDyv1wF+m3/ZSwPfDbwUz+l1gN/m2X4beC2e7XuA9+Y5/RTw1vzriRcM8S8zz+t1gN/mRXMcuBU4xrO9DvDbPNtvA6/FFX8DvDTP6aOBr+LfRrxgiH+ZeV6vA/w2L7rfBl6LZ3sd4Ld5tt8GXgu4BLw0cCvP9tLAX/G8LgF/zRUvDRzj+RMvGOJfZp7X6wC/zYvut4HX4tleB/htnu23gdcCXgf4bZ7tOPBXwIN5Xr8DvDZX/DbwWjx/4gVD/MvMv97rAL/Ns/028Fo82+sAv82z/Tbw28Bn85x+Cnhrnr/fAV6bK34beC2eP/GCIf5l5l/vdYDf5tl+G3gtnu11gN/m2T4a+Gqe00cDX8UL9jvAa3PFbwOvxfMnXjDEv8y8YL/D8/fRwF/zbL8NvBbP9jrAb/OCvTTwVzynS8Bf82x/DXw0V3w18NI820sDx7hCvGCIf5l5XpeA1wb+mhfNXwEvzbO9DvDbPH/Hgb8CHsxz+h3gtXnR/DbwWlwhXjDEv8w8r68BPpoXzWcBn81zeh3gt3n+jgN/DTyI5/Q7wGvzovlt4LW4QrxgiH+ZeV6vA/w2z/bVwEvxvI4DL83zeh3gt3nBXhr4K57TLvDXPNvfAB/NFV8NvBTP9tLAca4QLxjiX2ae1+sAv82z/TbwWrzoXgf4bZ7to4Gv5jl9NPBVvGC/A7w2V/w28Fo8f+IFQ/zLzPN6HeC3ebbfBl6LF93rAL/Ns/028NvAZ/Ocfhp4K56/3wFemyt+G3gtnj/xgiH+ZeZ5vQ7w27zofht4LZ7tdYDf5tl+G3gt4HWA3+bZjgN/DTyI5/U7wGtzxW8Dr8XzJ14wxL/MPK/XAX6bF91vA6/Fs70O8Ns8228DrwXsAi8D3MqzvTTwVzyvXeCvueKlgeM8f+IFQ/zLzPN6HeC3edH9NvBaPNvrAL/Ns/028Fpc8dfAy/CcPhr4Kv5txAuG+JeZ5/U6wG/zovtt4LV4ttcBfptn+23gtXi27wbeh+f008Bb8a8nXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL9sFzjG/06XgOO8YIh/2XcD78X/Tt8DvDcvGOJf9mDg6fzv9BDgVl4wxIvmvYHv4n+X9wG+mxcO8aJ7MPDZwFsDx/if6RLw08BnA7fyL0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwGGsrRB28vNtAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md2kPlus;
impl IconShape for Md2kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9.5 8.5c0 .55-.45 1-1 1h-2v1h3V15H5v-2.5c0-.55.45-1 1-1h2v-1H5V9h3.5c.55 0 1 .45 1 1v1.5zm4.75 3.5l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15h-1.75zM20 12.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEmUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzLXgf4bf5lLw18N/BSPK/fAV6bZ3tt4Ld4tmcALw3s8mzvDXwXL5x4wRD/MvMvex3gt3nRHAduBY7xnH4HeG2e7bWB3+KKS8BrA3/Ns7008FvAcV448YIh/mXmX/Y6wG/zovtt4LV4Tr8DvDbP9trAb3HF+wDfzbMdB34LeGn+ZeIFQ/zLzL/sdYDf5kX328Br8Zx+B3htnu21gd8Cvgd4b57TdwHvzYtGvGCIf5n593sd4Ld5tt8GXovn9DvAa/Nsrw18NfDSPKf3Br6LF514wRD/MvPv9zrAb/Nsvw28Fs/pd4DX5tleGtgFbuXZXhr4LeA4LzrxgiH+ZeZf53d4Xh8N/DXP9tvAa/Gcfgd4bV6w48BvAS/Nv454wRD/MvOvI/5lvw28Fs/pd4DX5gX7LuC9+dcTLxjiX2b+dcS/7LeB1+I5/Q7w2rxg3w28F/964gVD/MvMv85v87w+Bvhrnu23gdfiOf0O8Nq8YMeB3wZein8d8YIh/mXm3+91gN/m2X4beC2e0+8Ar82zvTSwC9zKs7008NvAMV504gVD/MvMv9/rAL/Ns/028Fo8p98BXptne23gq4CX4Tm9N/BdvOjEC4b4l5l/2esAv82L7reB1+I5/Q7w2jzbawO/BXw38D48p+8G3osXjXjBEP8y8y97HeC3edH9NvBaPKffAV6bZ3tt4Le44n2A7+bZjgO/DbwU/zLxgiH+ZeZf9jrAb/OiOQ48HTjOc/od4LV5ttcGfosrdoHXAf6aZ3tp4LeBY7xw4gVD/MvMv+x1gN/mX/bSwHcBL83z+h3gtXm21wZ+i2e7FXgZYJdne2/gu3jhxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviXmf/dxAuG+JeZ/93EC4b4l5n/3cQLhviX7QLH+N/pEnCcFwzxL/tu4L343+l7gPfmBUP8yx4MPJ3/nR4C3MoLhnjRvDfwXfzv8j7Ad/PCIV50DwY+G3hr4Bj/M10Cfhr4bOBW/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COL6ZhBiGvoWgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md3k;
impl IconShape for Md3k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 11c0 .55-.45 1-1 1H6.5v-1.5h3v-1h-2v-1h2v-1h-3V9H10c.55 0 1 .45 1 1v4zm7 1h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDyv1wF+m3/ZSwPfDbwUz+l1gN/m2X4beC2e7XuA9+Y5/RTw1vzriRcM8S8zz+t1gN/mRXMcuBU4xrO9DvDbPNtvA6/FFX8DvDTP6aOBr+LfRrxgiH+ZeV6vA/w2L7rfBl6LZ3sd4Ld5tt8GXgu4BLw0cCvP9tLAX/G8LgF/zRUvDRzj+RMvGOJfZp7X6wC/zYvut4HX4tleB/htnu23gdcCXgf4bZ7tOPBXwIN5Xr8DvDZX/DbwWjx/4gVD/MvMv97rAL/Ns/028Fo82+sAv82z/Tbw28Bn85x+Cnhrnr/fAV6bK34beC2eP/GCIf5l5l/vdYDf5tl+G3gtnu11gN/m2T4a+Gqe00cDX8UL9jvAa3PFbwOvxfMnXjDEv8y8cL/D8/po4K95tt8GXotnex3gt3nBXhr4K57TJeCveba/Bj6aK74aeGme7aWBY1whXjDEv8y8cOJf9tvAa/FsrwP8Ns/fceCvgAfznH4HeG1eNL8NvBZXiBcM8S8zL5z4l/028Fo82+sAv83zdxz4a+BBPKffAV6bF81vA6/FFeIFQ/zLzAv32zyvjwH+mmf7beC1eLbXAX6bF+ylgb/iOe0Cf82z/Q3w0Vzx1cBL8WwvDRznCvGCIf5l5l/vdYDf5tl+G3gtnu11gN/m2T4a+Gqe00cDX8UL9jvAa3PFbwOvxfMnXjDEv8z8670O8Ns8228Dr8WzvQ7w2zzbbwO/DXw2z+mngbfi+fsd4LW54reB1+L5Ey8Y4l9mntfrAL/Ni+63gdfi2V4H+G2e7beB1wJeB/htnu048NfAg3hevwO8Nlf8NvBaPH/iBUP8y8zzeh3gt3nR/TbwWjzb6wC/zbP9NvBawC7wMsCtPNtLA3/F89oF/porXho4zvMnXjDEv8w8r9cBfpsXzXHg6cBxnu11gN/m2X4beC2u+GvgZXhOHw18Ff824gVD/MvM83od4Lf5l7008F3AS/OcXgf4bZ7tt4HX4tm+G3gfntNPA2/Fv554wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/7Jd4Bj/O10CjvOCIf5l3w28F/87fQ/w3rxgiH/Zg4Gn87/TQ4BbecEQL5r3Br6L/13eB/huXjjEi+7BwGcDbw0c43+mS8BPA58N3Mq/DPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BO3askFcsJTlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md3kPlus;
impl IconShape for Md3kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9.5 14c0 .55-.45 1-1 1H5v-1.5h3v-1H6v-1h2v-1H5V9h3.5c.55 0 1 .45 1 1v4zm6.5 1h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzLXgf4bZ7tt4HX4tleB/htnu23gdfiOf0O8No822sDv8WzPQN4aWCXZ3tv4Lt44cQLhviXmX/Z6wC/zbP9NvBaPNvrAL/Ns/028Fo8p98BXptne23gt7jiEvDawF/zbC8N/BZwnBdOvGCIf5n5l70O8Ns8228Dr8WzvQ7w2zzbbwOvxXP6HeC1ebbXBn6LK94H+G6e7TjwW8BL8y8TLxjiX2b+Za8D/DbP9tvAa/FsrwP8Ns/228Br8Zx+B3htnu21gd8Cvgd4b57TdwHvzYtGvGCIf5n5l70O8Ns8228Dr8WzvQ7w2zzbbwOvxXP6HeC1ebbXBr4aeGme03sD38WLTrxgiH+Z+Ze9DvDbPNtvA6/Fs70O8Ns8228Dr8Vz+h3gtXm2lwZ2gVt5tpcGfgs4zotOvGCIf5n5l70O8Ns8228Dr8WzvQ7w2zzbbwOvxXP6HeC1ecGOA78FvDT/OuIFQ/zLzL/sdYDf5tl+G3gtnu11gN/m2X4beC2e0+8Ar80L9l3Ae/OvJ14wxL/M/MteB/ht/m3MFb8DvDYv2HcD78W/nnjBEP8y8y97HeC3+bcxV/wO8Nq8YMeB3wZein8d8YIh/mXmX/Y6wG/zb2Ou+B3gtXm2lwZ2gVt5tpcGfhs4xotOvGCIf5n5l70O8Nv825grfgd4bZ7ttYGvAl6G5/TewHfxohMvGOJfZv79Xgf4bZ7tt4HX4jn9DvDaPNtrA78FfDfwPjyn7wbeixeNeMEQ/zLz7/c6wG/zbL8NvBbP6XeA1+bZXhv4La54H+C7ebbjwG8DL8W/TLxgiH+Z+fd7HeC3ebbfBl6L5/Q7wGvzbK8N/BZX7AKvA/w1z/bSwG8Dx3jhxAuG+JeZf7/XAX6bZ/tt4LV4Tr8DvDbP9trAb/FstwIvA+zybO8NfBcvnHjBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/sl3gGP87XQKO84Ih/mXfDbwX/zt9D/DevGCIf9mDgafzv9NDgFt5wRAvmvcGvov/Xd4H+G5eOMSL7sHAZwNvDRzjf6ZLwE8Dnw3cyr8M8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8E+nmoQXnPzwgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md4k;
impl IconShape for Md4k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 10.5h-1V15H9.5v-1.5h-3V9H8v3h1.5V9H11v3h1v1.5zm6 1.5h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDyv1wF+m2f7beC1eLbXAX6bZ/tt4LV4ttcBfptn+23gtXi27wHem+f0U8Bb868nXjDEv8w8r9cBfptn+23gtXi21wF+m2f7beC1eLbXAX6bZ/tt4LW44m+Al+Y5fTTwVfzbiBcM8S8zz+t1gN/m2X4beC2e7XWA3+bZfht4LZ7tdYDf5tl+G3gt4BLw0sCtPNtLA3/F87oE/DVXvDRwjOdPvGCIf5l5Xq8D/DbP9tvAa/FsrwP8Ns/228Br8WyvA/w2z/bbwGsBrwP8Ns92HPgr4ME8r98BXpsrfht4LZ4/8YIh/mXmeb0O8Ns8228Dr8WzvQ7w2zzbbwOvxbO9DvDbPNtvA78NfDbP6aeAt+b5+x3gtbnit4HX4vkTLxjiX2ae1+sAv82z/TbwWjzb6wC/zbP9NvBaPNvrAL/Ns3008NU8p48GvooX7HeA1+aK3wZei+dPvGCIf5l5Xq8D/DbP9tvAa/FsrwP8Ns/228Br8WyvA/w2L9hLA3/Fc7oE/DXP9tfAR3PFVwMvzbO9NHCMK8QLhviXmef1OsBv82y/DbwWz/Y6wG/zbL8NvBbP9jrAb/P8HQf+Cngwz+l3gNfmRfPbwGtxhXjBEP8y87xeB/ht/m0MvA7w2zx/x4G/Bh7Ec/od4LV50fw28FpcIV4wxL/MPK/XAX6bfxsDrwP8Ni/YSwN/xXPaBf6aZ/sb4KO54quBl+LZXho4zhXiBUP8y8zzeh3gt/m3MfA6wG/zbB8NfDXP6aOBr+IF+x3gtbnit4HX4vkTLxjiX2ae1+sAv82/jYHXAX6bZ/tt4LeBz+Y5/TTwVjx/vwO8Nlf8NvBaPH/iBUP8y8y/3usAv82z/TbwWjzb6wC/zbP9NvBawOsAv82zHQf+GngQz+t3gNfmit8GXovnT7xgiH+Z+dd7HeC3ebbfBl6LZ3sd4Ld5tt8GXgvYBV4GuJVne2ngr3heu8Bfc8VLA8d5/sQLhviXmX+91wF+m2f7beC1eLbXAX6bZ/tt4LW44q+Bl+E5fTTwVfzbiBcM8S8z/3qvA/w2z/bbwGvxbK8D/DbP9tvAa/Fs3w28D8/pp4G34l9PvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJciMJB6Qef6AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md4kPlus;
impl IconShape for Md4kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8.5 10.5h-1V15H8v-1.5H5V9h1.5v3H8V9h1.5v3h1v1.5zM16 15h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/2WvzHuAT8Nf+y1+Lf53d40SH+ZeY/xu8Ar80L9lnARwPH+ffZBb4a+Bz+ZYh/mfmP8TvAa/P8fRfw3vzH+m7gfXjhEP8y8x/jd4DX5nm9N/Bd/Od4H+C7ecEQ/zLzH+N3gNfmef0V8NL85/gd4LV5wRD/MvMf43eA1+Z5mf9c4gVD/MvM83od4Lf5j2Ge1+sAv81/PsS/zDyv1wF+m/8Y5nm9DvDb/OdD/MvM83od4Lf5j2Ge1+sAv81/PsS/zDyv1wF+m/8Y5nm9DvDb/OdD/MvM83od4Lf5j2Ge1+sAv81/PsS/zDyvvwZ2ecF2gb8Gfgb4a14487xeB/htXrCvBl6KF83r8IIh/mXm3+ergY/hBTPP63WA3+YF+23gtXjRiBcM8S8z/35fA3w0z595Xq8D/DYv2G8Dr8WLRrxgiH+Z+Y/xEOBWnpd5Xq8D/DYv2G8Dr8WLRrxgiH+Z+Y/xOcBn87zM83od4Ld5wX4beC1eNOIFQ/zLfpt/vZcGjvGcfgZ4a56XeV6vA/w2L9hXAy/NczoOvBTPS7xgiP8cvw28Fs/pd4DX5nmZ5/U6wG/zr/PawG/xvMQLhvjP8dvAa/Gcfgd4bZ6XeV6vA/w2/zqvDfwWz0u8YIj/HL8NvBbP6XeA1+Z5mef1OsBv86/z2sBv8bzEC4b4z/HbwGvxnH4HeG2el3lerwP8Nv86rw38Fs9LvGCI/xy/DbwWz+l3gNfmeZnn9TrAb/Ov89rAb/G8xAuG+JeZ/xhfA3w0z8v85xIvGOJfZv5jfAzw1TyvvwZeiv8cvwO8Ni8Y4l9m/v0uAQ8Gdnle7w18F/853gf4bl4wxL/M/Pu9DfDTvGDfDbwX/7G+B3hvXjjEv8z82z0DeG/gt/mXfTbw0cAx/n0uAV8NfDb/MsS/7LX5t9kF/pp/vdfm3+e3edEh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AjdUf0E5yMwpAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md5g;
impl IconShape for Md5g {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17,13h2v2h-5V9h7c0-1.1-0.9-2-2-2h-5c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-4h-4V13z",
            }
            path {
                d: "M3,13h5v2H3v2h5c1.1,0,2-0.9,2-2v-2c0-1.1-0.9-2-2-2H5V9h5V7H3V13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzLXgf4bV50vw28Fs/pd4DX5tleG/gtnu0ZwEsDuzzbewPfxQsnXjDEv8z8y14H+G1edL8NvBbP6XeA1+bZXhv4La64BLw28Nc820sDvwUc54UTLxjiX2b+Za8D/DYvut8GXovn9DvAa/Nsrw38Fle8D/DdPNtx4LeAl+ZfJl4wxL/M/MteB/htXnS/DbwWz+l3gNfm2V4b+C3ge4D35jl9F/DevGjEC4b4l5l/2esAv82z/TbwWvzr/A7w2jzbawNfDbw0z+m9ge/iRSdeMMS/zPzLXgf4bZ7tt4HX4l/nd4DX5tleGtgFbuXZXhr4LeA4LzrxgiH+ZeZf9jrAb/NsXw28NM/rOPBSPH+/A7w2L9hx4LeAl+ZfR7xgiH+Z+Ze9DvDbvGg+G/gsntfvAK/NC/ZdwHvzrydeMMS/zPzLXgf4bV50fw28FM/pd4DX5gX7buC9+NcTLxjiX2b+ZX8N7PKCfQzw1zzbbwOvxXP6HeC1ecGOA78NvBT/OuIFQ/zLzL/f6wC/zbP9NvBaPKffAV6bZ3tpYBe4lWd7aeC3gWO86MQLhviXmX+/1wF+m2f7beC1eE6/A7w2z/bawFcBL8Nzem/gu3jRiRcM8S8z/7LXAX6bF91vA6/Fc/od4LV5ttcGfgv4buB9eE7fDbwXLxrxgiH+ZeZf9jrAb/Oi+23gtXhOvwO8Ns/22sBvccX7AN/Nsx0Hfht4Kf5l4gVD/MvMv+x1gN/mRXMceDpwnOf0O8Br82yvDfwWV+wCrwP8Nc/20sBvA8d44cQLhviXmX/Z6wC/zb/spYHvAl6a5/U7wGvzbK8N/BbPdivwMsAuz/bewHfxwokXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxL9sFjvG/0yXgOC8Y4l/23cB78b/T9wDvzQuG+Jc9GHg6/zs9BLiVFwzxonlv4Lv43+V9gO/mhUO86B4MfDbw1sAx/me6BPw08NnArfzLEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHBPaaQcywPBUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md5k;
impl IconShape for Md5k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 7.5H8v1h2c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H6.5v-1.5h3v-1h-3V9H11v1.5zm7 4.5h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDyv1wF+mxfdbwOvxbO9DvDbPNtvA6/Fs30P8N48p58C3pp/PfGCIf5l5nm9DvDbvOh+G3gtnu11gN/m2X4beC2u+BvgpXlOHw18Ff824gVD/MvM83od4Ld50f028Fo82+sAv82z/TbwWsAl4KWBW3m2lwb+iud1Cfhrrnhp4BjPn3jBEP8y87xeB/htXnS/DbwWz/Y6wG/zbL8NvBbwOsBv82zHgb8CHszz+h3gtbnit4HX4vkTLxjiX2ae1+sAv82z/TbwWrzoXgf4bZ7tt4HfBj6b5/RTwFvz/P0O8Npc8dvAa/H8iRcM8S8zz+t1gN/m2X4beC1edK8D/DbP9tHAV/OcPhr4Kl6w3wFemyt+G3gtnj/xgiH+ZeZ5vQ7w2zzbVwMvzfM6DrwUz+t1gN/mBXtp4K94TpeAv+bZ/hr4aK74auClebaXBo5xhXjBEP8y87xeB/htXjSfDXwWz+l1gN/m+TsO/BXwYJ7T7wCvzYvmt4HX4grxgiH+ZeZ5vQ7w27zo/hp4KZ7tdYDf5vk7Dvw18CCe0+8Ar82L5reB1+IK8YIh/mXmef01sMsL9jHAX/Nsvw28Fs/2OsBv84K9NPBXPKdd4K95tr8BPporvhp4KZ7tpYHjXCFeMMS/zPzrvQ7w2zzbbwOvxbO9DvDbPNtHA1/Nc/po4Kt4wX4HeG2u+G3gtXj+xAuG+JeZf73XAX6bZ/tt4LV4ttcBfptn+23gt4HP5jn9NPBWPH+/A7w2V/w28Fo8f+IFQ/zLzPN6HeC3edH9NvBaPNvrAL/Ns/028FrA6wC/zbMdB/4aeBDP63eA1+aK3wZei+dPvGCIf5l5Xq8D/DYvut8GXotnex3gt3m23wZeC9gFXga4lWd7aeCveF67wF9zxUsDx3n+xAuG+JeZ5/U6wG/zojkOPB04zrO9DvDbPNtvA6/FFX8NvAzP6aOBr+LfRrxgiH+ZeV6vA/w2/7KXBr4LeGme0+sAv82z/TbwWjzbdwPvw3P6aeCt+NcTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l+2Cxzjf6dLwHFeMMS/7LuB9+J/p+8B3psXDPEvezDwdP53eghwKy8Y4kXz3sB38b/L+wDfzQuHeNE9GPhs4K2BY/zPdAn4aeCzgVv5lyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj4bntEGpbYXCAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md5kPlus;
impl IconShape for Md5kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9.5 7.5h-3v1h2c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H5v-1.5h3v-1H5V9h4.5v1.5zM16 15h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxoPgf4amCXf9lvA6/Fc/od4LV5ttcGfotnewbw0sAuz/bewHfxwokXDPEvM/+ynwHemhfdbwOvxXP6HeC1ebbXBn6LKy4Brw38Nc/20sBvAcd54cQLhviXmX/Z+wDfzYvut4HX4jn9DvDaPNtrA7/FFe8DfDfPdhz4LeCl+ZeJFwzxLzP/stcBfpsX3W8Dr8Vz+h3gtXm21wZ+C/ge4L15Tt8FvDcvGvGCIf5l5l/2OsBv82y/DbwW/zq/A7w2z/bawFcDL81zem/gu3jRiRcM8S8z/7LXAX6bZ/tt4LX41/kd4LV5tpcGdoFbebaXBn4LOM6LTrxgiH+Z+Ze9DvDbPNtXAy/N8zoOvBTP3+8Ar80Ldhz4LeCl+dcRLxjiX2b+Za8D/DYvms8GPovn9TvAa/OCfRfw3vzriRcM8S8z/7LXAX6bF91fAy/Fc/od4LV5wb4beC/+9cQLhviXmX/Z6wC/zbN9NfBSPNvHAH/Ns/028Fo8p98BXpsX7Djw28BL8a8jXjDEv8z8y14H+G2e7beB1+LZXgf4bZ7tt4HX4jn9DvDaPNtLA7vArTzbSwO/DRzjRSdeMMS/zPzLXgf4bZ7tt4HX4tleB/htnu23gdfiOf0O8No822sDXwW8DM/pvYHv4kUnXjDEv8z8y14H+G2e7beB1+LZXgf4bZ7tt4HX4jn9DvDaPNtrA78FfDfwPjyn7wbeixeNeMEQ/zLzL3sf4Lt5tpcGjvNsfw3s8my/DbwWz+l3gNfm2V4b+C2ueB/gu3m248BvAy/Fv0y8YIh/mfmX/TTwNrxojgNPB47znH4HeG2e7bWB3+KKXeB1gL/m2V4a+G3gGC+ceMEQ/zLzovls4HuAW3nBXhr4LuCleV6/A7w2z/bawG/xbLcCLwPs8mzvDXwXL5x4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/7Jd4Bj/O10CjvOCIf5l3w28F/87fQ/w3rxgiH/Zg4Gn87/TQ4BbecEQL5r3Br6L/13eB/huXjjEi+7BwGcDbw0c43+mS8BPA58N3Mq/DPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BKpUpEGa4oGTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md6k;
impl IconShape for Md6k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 12.5h1.5V14H8zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 7.5H8v1h2c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H7.5c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1H11v1.5zm7 4.5h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFU0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDx/nwN8NbDLv+y3gdfi2V4H+G2e7beB1+LZvgd4b57TTwFvzb+eeMEQ/zLzvH4GeGtedL8NvBbP9jrAb/Nsvw28Flf8DfDSPKePBr6KfxvxgiH+ZeZ5vQ/w3bzofht4LZ7tdYDf5tl+G3gt4BLw0sCtPNtLA3/F87oE/DVXvDRwjOdPvGCIf5l5Xq8D/DYvut8GXotnex3gt3m23wZeC3gd4Ld5tuPAXwEP5nn9DvDaXPHbwGvx/IkXDPEvM8/rdYDf5tl+G3gtXnSvA/w2z/bbwG8Dn81z+ingrXn+fgd4ba74beC1eP7EC4b4l5nn9TrAb/Nsvw28Fi+61wF+m2f7aOCreU4fDXwVL9jvAK/NFb8NvBbPn3jBEP8y87xeB/htnu2rgZfmeR0HXorn9TrAb/OCvTTwVzynS8Bf82x/DXw0V3w18NI820sDx7hCvGCIf5l5Xq8D/DYvms8GPovn9DrAb/P8HQf+Cngwz+l3gNfmRfPbwGtxhXjBEP8y87xeB/htXnR/DbwUz/Y6wG/z/B0H/hp4EM/pd4DX5kXz28BrcYV4wRD/MvO8Xgf4bZ7tq4GX4tk+Bvhrnu23gdfi2V4H+G1esJcG/orntAv8Nc/2N8BHc8VXAy/Fs700cJwrxAuG+JeZ5/U6wG/zbL8NvBbP9jrAb/Nsvw28Fs/2OsBv82wfDXw1z+mjga/iBfsd4LW54reB1+L5Ey8Y4l9mntfrAL/Ns/028Fo82+sAv82z/TbwWjzb6wC/zbP9NvDbwGfznH4aeCuev98BXpsrfht4LZ4/8YIh/mXmeb0O8Ns8228Dr8WzvQ7w2zzbbwOvxbO9DvDbPNtvA68FvA7w2zzbceCvgQfxvH4HeG2u+G3gtXj+xAuG+JeZ5/U+wHfzbC8NHOfZ/hrY5dl+G3gtnu11gN/m2X4beC1gF3gZ4Fae7aWBv+J57QJ/zRUvDRzn+RMvGOJfZp7XTwNvw4vmOPB04DjP9jrAb/Nsvw28Flf8NfAyPKePBr6KfxvxgiH+Zeb5+2zge4BbecFeGvgu4KV5Tq8D/DbP9tvAa/Fs3w28D8/pp4G34l9PvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLMVL5BbKOZHQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md6kPlus;
impl IconShape for Md6kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 12.5H8V14H6.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9.5 7.5h-3v1h2c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H6c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3.5v1.5zM16 15h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzLXgf4bf5lLw18N/BSPK/fAV6bZ3tt4Ld4tmcALw3s8mzvDXwXL5x4wRD/MvMvex3gt3nRHAduBY7xnH4HeG2e7bWB3+KKS8BrA3/Ns7008FvAcV448YIh/mXmX/Y6wG/zovtt4LV4Tr8DvDbP9trAb3HF+wDfzbMdB34LeGn+ZeIFQ/zLzL/sdYDf5kX3V8BL85x+B3htnu21gd8Cvgd4b57TdwHvzYtGvGCIf5n593kZ4K95tgcDT+d5/Q7w2jzbawNfDbw0z+m9ge/iRSdeMMS/zPzbPQN4MM/pvYHv4nn9DvDaPNtLA7vArTzbSwO/BRznRSdeMMS/zPzbfQ3w0Tynnwbeiuf1O8Br84IdB34LeGn+dcQLhviXmX+7twF+mmc7Dlzk+fsd4LV5wb4LeG/+9cQLhviXmX+bS8BxntN7A9/F8/c7wGvzgn038F7864kXDPEvM/823wO8N8/pu4H34vn7HeC1ecGOA78NvBT/OuIFQ/zLzL/N+wDfzXO6CBzn+fsd4LV5tpcGdoFbebaXBn4bOMaLTrxgiH+Z+bc5AezybG8N/BQv2O8Ar82zvTbwVcDL8JzeG/guXnTiBUP8y8y/3s8Ab81z+mrgo3jBfgd4bZ7ttYHfAr4beB+e03cD78WLRrxgiH+Z+df7GOCreU5PBx7MC/Y7wGvzbK8N/BZXvA/w3TzbceC3gZfiXyZeMMS/zPzrPQS4lWd7aeCveOF+B3htnu21gd/iil3gdYC/5tleGvht4BgvnHjBEP8y86/zN8BL85y+GvgoXrjfAV6bZ3tt4Ld4tluBlwF2ebb3Br6LF068YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/swcDT+d/pIcCtvGCIF817A9/F/y7vA3w3LxziRfdg4LOBtwaO8T/TJeCngc8GbuVfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AuDNmEE14tdKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md7k;
impl IconShape for Md7k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9.5 15H7.75l1.38-4.5H6.5V9H10c.67 0 1.15.65.96 1.29L9.5 15zm8.5 0h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFE0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDyv1wF+m3/ZSwPfDbwUz+l1gN/m2X4beC2e7XuA9+Y5/RTw1vzriRcM8S8zz+t1gN/mRXMcuBU4xrO9DvDbPNtvA6/FFX8DvDTP6aOBr+LfRrxgiH+ZeV6vA/w2L7rfBl6LZ3sd4Ld5tt8GXgu4BLw0cCvP9tLAX/G8LgF/zRUvDRzj+RMvGOJfZp7X6wC/zYvur4CX5tleB/htnu23gdcCXgf4bZ7tOPBXwIN5Xr8DvDZX/DbwWjx/4gVD/MvMv87LAH/Nsz0YeDrP6XWA3+bZfhv4beCzeU4/Bbw1z9/vAK/NFb8NvBbPn3jBEP8y86J7BvBgntN7A9/Fc3od4Ld5to8Gvprn9NHAV/GC/Q7w2lzx28Br8fyJFwzxLzMvuq8BPprn9NPAW/GcXgf4bV6wlwb+iud0Cfhrnu2vgY/miq8GXppne2ngGFeIFwzxLzMvurcBfppnOw5c5Hm9DvDbPH/Hgb8CHsxz+h3gtXnR/DbwWlwhXjDEv8y8aC4Bx3lO7w18F8/rdYDf5vk7Dvw18CCe0+8Ar82L5reB1+IK8YIh/mXmRfM9wHvznL4beC+e1+sAv80L9tLAX/GcdoG/5tn+Bvhorvhq4KV4tpcGjnOFeMEQ/zLzonkf4Lt5TheB4zyv1wF+m2f7aOCreU4fDXwVL9jvAK/NFb8NvBbPn3jBEP8y86I5AezybG8N/BTP3+sAv82z/Tbw28Bn85x+Gngrnr/fAV6bK34beC2eP/GCIf5l5l/2M8Bb85y+Gvgonr/XAX6bZ/tt4LWA1wF+m2c7Dvw18CCe1+8Ar80Vvw28Fs+feMEQ/zLzL/sY4Kt5Tk8HHszz9zrAb/Nsvw28FrALvAxwK8/20sBf8bx2gb/mipcGjvP8iRcM8S8z/7KHALfybC8N/BUv2OsAv82z/TbwWlzx18DL8Jw+Gvgq/m3EC4b4l5kX7m+Al+Y5fTXwUbxgrwP8Ns/228Br8WzfDbwPz+mngbfiX0+8YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf9kucIz/nS4Bx3nBEP+y7wbei/+dvgd4b14wxL/swcDT+d/pIcCtvGCIF817A9/F/y7vA3w3LxziRfdg4LOBtwaO8T/TJeCngc8GbuVfhvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AkLNskGDLn58AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md7kPlus;
impl IconShape for Md7kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM8 15H6.25l1.38-4.5H5V9h3.5c.67 0 1.15.65.96 1.29L8 15zm8 0h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFBklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxoPgf4amCXF+ylge8GXorn9TvAa/Nsrw38Fs/2DOClgV2e7b2B7+KFEy8Y4l9m/mU/A7w1L5rjwK3AMZ7T7wCvzbO9NvBbXHEJeG3gr3m2lwZ+CzjOCydeMMS/zPzL3gf4bp7tpYFjPNvfALs8228Dr8Vz+h3gtXm21wZ+iyveB/hunu048FvAS/MvEy8Y4l9m/mWvA/w2z/bbwGvxbK8D/DbP9tvAa/Gcfgd4bZ7ttYHfAr4HeG+e03cB782LRrxgiH+Z+Ze9DvDbPNtvA6/Fs70O8Ns8228Dr8Vz+h3gtXm21wa+GnhpntN7A9/Fi068YIh/mfmXvQ7w2zzbbwOvxbO9DvDbPNtvA6/Fc/od4LV5tpcGdoFbebaXBn4LOM6LTrxgiH+Z+Ze9DvDbPNtXAy/Ns3008Nc8228Dr8Vz+h3gtXnBjgO/Bbw0/zriBUP8y8y/7HWA3+ZF99vAa/Gcfgd4bV6w7wLem3898YIh/mXmX/Y6wG/zovtt4LV4Tr8DvDYv2HcD78W/nnjBEP8y8y97HeC3ebavBl6KZ/sY4K95tt8GXovn9DvAa/OCHQd+G3gp/nXEC4b4l5l/2esAv82z/TbwWjzb6wC/zbP9NvBaPKffAV6bZ3tpYBe4lWd7aeC3gWO86MQLhviXmX/Z6wC/zbP9NvBaPNvrAL/Ns/028Fo8p98BXptne23gq4CX4Tm9N/BdvOjEC4b4l5l/2esAv82z/TbwWjzb6wC/zbP9NvBaPKffAV6bZ3tt4LeA7wbeh+f03cB78aIRLxjiX2b+Ze8DfDfP9tLAcZ7tr4Fdnu23gdfiOf0O8No822sDv8UV7wN8N892HPht4KX4l4kXDPEvM/+ynwbehhfNceDpwHGe0+8Ar82zvTbwW1yxC7wO8Nc820sDvw0c44UTLxjiX2ZeNJ8NfA9wKy/YSwPfBbw0z+t3gNfm2V4b+C2e7VbgZYBdnu29ge/ihRMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX7YLHON/p0vAcV4wxL/su4H34n+n7wHemxcM8S97MPB0/nd6CHArLxjiRfPewHfxv8v7AN/NC4d40T0Y+GzgrYFj/M90Cfhp4LOBW/mXIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP7JeuQZt6e9AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md8k;
impl IconShape for Md8k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 12.5h1.5V14H8zM8 10h1.5v1.5H8zm11-7H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 11c0 .55-.45 1-1 1H7.5c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1H10c.55 0 1 .45 1 1v4zm7 1h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDx/nwN8NbDLC/bSwHcDL8Vzeh3gt3m23wZei2f7HuC9eU4/Bbw1/3riBUP8y8zz+hngrXnRHAduBY7xbK8D/DbP9tvAa3HF3wAvzXP6aOCr+LcRLxjiX2ae1/sA382zvTRwjGf7G2CXZ/tt4LV4ttcBfptn+23gtYBLwEsDt/JsLw38Fc/rEvDXXPHSwDGeP/GCIf5l5nm9DvDbPNtvA6/Fs70O8Ns8228Dr8WzvQ7w2zzbbwOvBbwO8Ns823Hgr4AH87x+B3htrvht4LV4/sQLhviXmef1OsBv82y/DbwWz/Y6wG/zbL8NvBbP9jrAb/Nsvw38NvDZPKefAt6a5+93gNfmit8GXovnT7xgiH+ZeV6vA/w2z/bbwGvxbK8D/DbP9tvAa/FsrwP8Ns/20cBX85w+GvgqXrDfAV6bK34beC2eP/GCIf5l5nm9DvDbPNtXAy/Ns3008Nc8228Dr8WzvQ7w27xgLw38Fc/pEvDXPNtfAx/NFV8NvDTP9tLAMa4QLxjiX2ae1+sAv82L7reB1+LZXgf4bZ6/48BfAQ/mOf0O8Nq8aH4beC2uEC8Y4l9mntfrAL/Ni+63gdfi2V4H+G2ev+PAXwMP4jn9DvDavGh+G3gtrhAvGOJfZp7X6wC/zbN9NfBSPNvHAH/Ns/028Fo82+sAv80L9tLAX/GcdoG/5tn+Bvhorvhq4KV4tpcGjnOFeMEQ/zLzvF4H+G2e7beB1+LZXgf4bZ7tt4HX4tleB/htnu2jga/mOX008FW8YL8DvDZX/DbwWjx/4gVD/MvM83od4Ld5tt8GXotnex3gt3m23wZei2d7HeC3ebbfBn4b+Gye008Db8Xz9zvAa3PFbwOvxfMnXjDEv8w8r9cBfptn+23gtXi21wF+m2f7beC1eLbXAX6bZ/tt4LWA1wF+m2c7Dvw18CCe1+8Ar80Vvw28Fs+feMEQ/zLzvN4H+G6e7aWB4zzbXwO7PNtvA6/Fs70O8Ns8228DrwXsAi8D3MqzvTTwVzyvXeCvueKlgeM8f+IFQ/zLzPP6aeBteNEcB54OHOfZXgf4bZ7tt4HX4oq/Bl6G5/TRwFfxbyNeMMS/zDx/nw18D3ArL9hLA98FvDTP6XWA3+bZfht4LZ7tu4H34Tn9NPBW/OuJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S/bBY7xv9Ml4DgvGOJf9t3Ae/G/0/cA780LhviXPRh4Ov87PQS4lRcM8aJ5b+C7+N/lfYDv5oVDvOgeDHw28NbAMf5nugT8NPDZwK38yxD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R66myEFOAdP7AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md8kPlus;
impl IconShape for Md8kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 12.5H8V14H6.5zm0-2.5H8v1.5H6.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9.5 14c0 .55-.45 1-1 1H6c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm6.5 1h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zLxoPgf4amCXF+ylge8GXorn9TvAa/Nsrw38Fs/2DOClgV2e7b2B7+KFEy8Y4l9m/mU/A7w1L5rjwK3AMZ7T7wCvzbO9NvBbXHEJeG3gr3m2lwZ+CzjOCydeMMS/zPzL3gf4bp7tpYFjPNvfALs8228Dr8Vz+h3gtXm21wZ+iyveB/hunu048FvAS/MvEy8Y4l9m/mWvA/w2z/bbwGvxbK8D/DbP9tvAa/Gcfgd4bZ7ttYHfAr4HeG+e03cB782LRrxgiH+Z+Ze9DvDbPNtvA6/Fs70O8Ns8228Dr8Vz+h3gtXm21wa+GnhpntN7A9/Fi068YIh/mfmXvQ7w2zzbbwOvxbO9DvDbPNtvA6/Fc/od4LV5tpcGdoFbebaXBn4LOM6LTrxgiH+Z+Ze9DvDbPNtXAy/Ns3008Nc8228Dr8Vz+h3gtXnBjgO/Bbw0/zriBUP8y8y/7GuAj+ZF99vAa/Gcfgd4bV6w7wLem3898YIh/mXmX7YLvA7w17xofht4LZ7T7wCvzQv23cB78a8nXjDEv8y86H6b5+9jgL/m2X4beC2e0+8Ar80Ldhz4beCl+NcRLxjiX2b+/V4H+G2e7beB1+I5/Q7w2jzbSwO7wK0820sDvw0c40UnXjDEv8z8+70O8Ns8228Dr8Vz+h3gtXm21wa+CngZntN7A9/Fi068YIh/mfmXvQ7w27zofht4LZ7T7wCvzbO9NvBbwHcD78Nz+m7gvXjRiBcM8S8z/7LXAX6bF91vA6/Fc/od4LV5ttcGfosr3gf4bp7tOPDbwEvxLxMvGOJfZv5lrwP8Ni+a48DTgeM8p98BXptne23gt7hiF3gd4K95tpcGfhs4xgsnXjDEv8z8y14H+G3+ZS8NfBfw0jyv3wFem2d7beC3eLZbgZcBdnm29wa+ixdOvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7MHA0/nf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIYmKVB95LGMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md9k;
impl IconShape for Md9k {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 10h1.5v1.5H8zm11-7H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 11c0 .55-.45 1-1 1H6.5v-1.5h3v-1h-2c-.55 0-1-.45-1-1V10c0-.55.45-1 1-1H10c.55 0 1 .45 1 1v4zm7 1h-1.75l-1.75-2.25V15H13V9h1.5v2.25L16.25 9H18l-2.25 3L18 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zDx/nwN8NbDLC/bSwHcDL8Vzeh3gt3m23wZei2f7HuC9eU4/Bbw1/3riBUP8y8zz+hngrXnRHAduBY7xbK8D/DbP9tvAa3HF3wAvzXP6aOCr+LcRLxjiX2ae1/sA382zvTRwjGf7G2CXZ/tt4LV4ttcBfptn+23gtYBLwEsDt/JsLw38Fc/rEvDXXPHSwDGeP/GCIf5l5nm9DvDbPNtvA6/Fs70O8Ns8228Dr8WzvQ7w2zzbbwOvBbwO8Ns823Hgr4AH87x+B3htrvht4LV4/sQLhviXmef1OsBv82y/DbwWz/Y6wG/zbL8NvBbP9jrAb/Nsvw38NvDZPKefAt6a5+93gNfmit8GXovnT7xgiH+ZeV6vA/w2z/bbwGvxbK8D/DbP9tvAa/FsrwP8Ns/20cBX85w+GvgqXrDfAV6bK34beC2eP/GCIf5l5nm9DvDbPNtXAy/Ns3008Nc8228Dr8WzvQ7w27xgLw38Fc/pEvDXPNtfAx/NFV8NvDTP9tLAMa4QLxjiX2ae19cAH82L7reB1+LZXgf4bZ6/48BfAQ/mOf0O8Nq8aH4beC2uEC8Y4l9mntcu8DrAX/Oi+W3gtXi21wF+m+fvOPDXwIN4Tr8DvDYvmt8GXosrxAuG+JeZF+y3ef4+Bvhrnu23gdfi2V4H+G1esJcG/orntAv8Nc/2N8BHc8VXAy/Fs700cJwrxAuG+JeZf73XAX6bZ/tt4LV4ttcBfptn+2jgq3lOHw18FS/Y7wCvzRW/DbwWz594wRD/MvOv9zrAb/Nsvw28Fs/2OsBv82y/Dfw28Nk8p58G3orn73eA1+aK3wZei+dPvGCIf5l5Xq8D/DYvut8GXotnex3gt3m23wZeC3gd4Ld5tuPAXwMP4nn9DvDaXPHbwGvx/IkXDPEvM8/rdYDf5kX328Br8WyvA/w2z/bbwGsBu8DLALfybC8N/BXPaxf4a654aeA4z594wRD/MvO8Xgf4bV40x4GnA8d5ttcBfptn+23gtbjir4GX4Tl9NPBV/NuIFwzxLzPP63WA3+Zf9tLAdwEvzXN6HeC3ebbfBl6LZ/tu4H14Tj8NvBX/euIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzP9u4gVD/MvM/27iBUP8y3aBY/zvdAk4zguG+Jd9N/Be/O/0PcB784Ih/mUPBp7O/04PAW7lBUO8aN4b+C7+d3kf4Lt54RAvugcDnw28NXCM/5kuAT8NfDZwK/8yxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RSpi/QSgAzX0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Md9kPlus;
impl IconShape for Md9kPlus {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 10H8v1.5H6.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9.5 14c0 .55-.45 1-1 1H5v-1.5h3v-1H6c-.55 0-1-.45-1-1V10c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zm6.5 1h-1.75l-1.75-2.25V15H11V9h1.5v2.25L14.25 9H16l-2.25 3L16 15zm4-2.5h-1.5V14h-1v-1.5H16v-1h1.5V10h1v1.5H20v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v8x3tp4BjP6RLw1/zHM8/rdYDfBhDP9trAb/G8xH+83wZei+f0O8Br8x/PPK/XAX4bQDzbawO/xfMS//F+G3gtntPvAK/NfzzzvF4H+G0A8WyvDfwWz0v8x/tt4LV4Tr8DvDb/8czzeh3gtwHEs7028Fs8L/Ef77eB1+I5/Q7w2vzHM8/rdYDfBhDP9trAb/G8xH+83wZei+f0O8Br8x/PPK/XAX4bQDzbawO/xfMS//F+G3gtntPvAK/NfzzzvF4H+G0A8WyvDfwWz0v8x/tt4LV4Tr8DvDb/8czzeh3gtwHEs7028Fs8L/Gv99LAMV6wrwZemuf018BH84JdAv6afz3zvF4H+G0A8WyvDfwWz0v86/028Fr8x/od4LX51zPP63WA3wYQz/bawG/xvMS/3m8Dr8V/rN8BXpt/PfO8Xgf4bQDxbK8N/BbPS/zr/TbwWvzH+h3gtfnXM8/rdYDfBhDP9trAb/G8xL/ebwOvxX+s3wFem38987xeB/htAPFsrw38Fs9L/Ov9NvBa/Mf6HeC1+dczz+t1gN8GEM/22sBv8bzEf7zfBl6L5/Q7wGvzH888r9cBfhtAPNtrA7/F8xL/8X4beC2e0+8Ar81/PPO8Xgf4bQDxbK8N/BbPS/zH+23gtXhOvwO8Nv/xzPN6HeC3AcSzvTbwWzwv8R/vt4HX4jn9DvDa/Mczz+t1gN8GEM/22sBv8bzEf7zfBl6L5/Q7wGvzH888r9cBfhtAPNtrA7/F8xL/8X4beC2e0+8Ar81/PPO8Xgf4bQDxbK8N/BbPS/zH+23gtXhOvwO8Nv/xzPN6HeC3AcSzvTbwWzwv8R/vt4HX4jn9DvDa/Mczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5/Tb/u702z+t1gN8GEM/20sBf8f/DywB/DSCe063Ag/i/7RnAg7kC8ZxeG/gt/m97HeC3uQLxvN4b+GrgGP+3XALeG/hpng3x/B0H3ht4aeDB/Ou9NHCM/1iXgL/mX+9W4K+B7wZ2eU6I/xy/DbwW/7F+B3ht/mMh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cvw28Fv+xfgd4bf5jIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GP6hu1QY8wLkMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddToQueue;
impl IconShape for MdAddToQueue {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.11 0-2 .89-2 2v12c0 1.1.89 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.11-.9-2-2-2zm0 14H3V5h18v12zm-5-7v2h-3v3h-2v-3H8v-2h3V7h2v3h3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v85znOFbv85zHP63WA3wYQz/bawG/xvMR/juPAb3HF6wC7/Ocwz+t1gN8GEM/22sBv8bzEf7zjwG8BL80Vfw28DrDLfzzzvF4H+G0A8WyvDfwWz+u3edH8DfDR/MuOA78FvDTP6a+B1wF2+Zd9NfBSvGhem+f1OsBvA4hne2ngr/i3+x3gtXnhjgO/Bbw0z99fA68D7PLC/TbwWvzbvQzw1wDiOd0KPIh/m98BXpsX7DjwW8BL88L9NfA6wC4v2G8Dr8W/zTOAB3MF4jm9NvBb/Nv8DvDaPH/Hgd8CXpoXzV8DrwPs8vz9NvBa/Nu8DvDbXIF4Xu8NfDVwjH+d3wFem+d1HPgt4KX51/lr4HWAXZ7XbwOvxb/OJeC9gZ/m2RDP33HgvYGXBh7Mi+avgY/mOR0Hfgt4af5t/hp4HWCX5/TVwEvzorkV+Gvgu4FdnhPiP89x4LeAl+bf56+B1wF2+Y+H+M9xHPgt4KX5j/HXwOsAu/zHQvzHOw78FvDS/Mf6a+B1gF3+4yD+Yx0Hfgt4af5z/DXwOsAu/zEQ/3GOA78FvDT/uf4aeB1gl38/xH+M48BvAS/Nf42/Bl4H2OXfB/Hvdxz4LeCl+a/118DrALv82yH+/b4aeGn+e/w18NH82yH+f0P8/4b4/w3x/xvi/zf+ERk3tkHPWzG8AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirplay;
impl IconShape for MdAirplay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                points: "6,22 18,22 12,16",
            }
            path {
                d: "M21,3H3C1.9,3,1,3.9,1,5v12c0,1.1,0.9,2,2,2h4v-2H3V5h18v12h-4v2h4c1.1,0,2-0.9,2-2V5C23,3.9,22.1,3,21,3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlbum;
impl IconShape for MdAlbum {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADxUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4kXz0sAx/mP8Dv+ylwaO8R/rEvDXPCfEC/fawHcBD+Y/zi7w1cDn8IL9NvBa/Mf6HeC1eU6IF+y9ge/iP893A+/D8/fbwGvxH+t3gNfmOSGev+PA04Hj/Od6H+C7eV6/DbwW/7F+B3htnhPi+fto4Kv4z/c7wGvzvH4beC3+Y/0O8No8J8Tz993Ae/FfQzyv3wZei/9YvwO8Ns8J8fz9NvBa/NcQ/30Qz99vA6/Ffw3x3wfx/P028Fr81xD/fRDP328Dr8V/DfHfB/H8/TbwWvzXEP99EM/fbwOvxX8N8by+Gngp/mP9DfDRPCfE8/fbwGvxX0M8r98GXov/WL8DvDbPCfH8/TbwWvzXEM/rt4HX4j/W7wCvzXNCPH+/DbwW/3bfwxXvxb9MPK/fBl6L/1i/A7w2zwnx/P028Fr82/wN8Npc8dvAS/HCief128Br8R/rd4DX5jkhnr/fBl6LF+x7uOK9eE6XgJcGbuWKlwZ+GzjGCyae11cDL81/rL8GPprnhHj+fht4LZ6/vwFemit+G3gtnu1tgJ/mOb038F28YOK/D+L5+23gtXhel4CXBm7liuPAbwMvBXwO8Nk8f98NvBfPn/jvg3j+fht4LZ7X6wC/zXN6aeCzgbfmBTsO/DbwUjwv8d8H8fz9NvBaPKfPAT6bf7uXBn4bOMZzEv99EM/fbwOvxbP9DPDW/Pu9N/BdPCfx3wfx/H038F781xDP67eB1+I/1u8Ar81zQjx/Hw18Ff/5fgd4bZ7XbwOvxX+s3wFem+eEeP6OA7cCx/jP9T7Ad/O8fht4Lf5j/Q7w2jwnxAv23sB38Z/ne4D35vn7beC1+I/1O8Br85wQL9xrA98NPIj/OJeArwY+mxfst4HX4j/W7wCvzXNCvGheGjjOf4zf5l/20sBx/mPtAn/Nc0L8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8Eh11hQXeJbccAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdArtTrack;
impl IconShape for MdArtTrack {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 13h-8v-2h8v2zm0-6h-8v2h8V7zm-8 10h8v-2h-8v2zm-2-8v6c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V9c0-1.1.9-2 2-2h6c1.1 0 2 .9 2 2zm-1.5 6l-2.25-3-1.75 2.26-1.25-1.51L3.5 15h7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHcklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b47/HbwGvxnG4FbgV+G7gV+Blgl/9ciP8evw28Fv+yvwa+G/geYJf/eIj/Hr8NvBYvul3gu4GvAW7lPw7iv8dvA6/Fv94u8NXA5/AfA/Ef4ziwy4vut4HX4t/ur4G3AW7l3wfx7/fawE8BHwN8Ny+a3wZei+f0DOBW4LV40ewC7wP8NP92iH+f9wa+i2d7H+C7+Zf9NvBaPKffAV6bK14beGvgvYFjvHDvA3w3/zaIf7v3Br6L5/U+wHfzwv028Fo8p98BXpvndBx4b+CzgWO8YO8DfDf/eoh/m7cGfooX7H2A7+YF+23gtXhOvwO8Ns/fceC7gbfiBXsb4Kf510H86z0Y+CvgOC/c+wDfzfP328Br8Zx+B3htXrj3Br6L528XeBngVl50iH+9vwJemhfN+wDfzfP6auCleU5/DXw0/7LXBn4aOMbz+mvgZXjRIf51Phr4Kv513gf4bv5jvTTw28AxntfnAJ/NiwbxojsOPB04zvP6HOCjgWM8f+8DfDf/sd4b+C6e1y7wMsCt/MsQL7qvBj6K5/UzwFsDLw38NnCM5+99gO/mP9ZPA2/F8/oc4LP5lyFeNMeBpwPHeU7PAF4a2OWKlwZ+GzjG8/c+wHfzH+c4cCtwjOe0CzwE2OWFQ7xoPhr4Kp7X+wDfzXN6aeC3gWM8f+8DfDf/cT4b+Cye1/sA380Lh3jR/BXw0jynS8CDgV2e10sDvw0c4/l7H+C7+Y9xHLgVOMZz+mvgZXjhEP+y48BFntfXAB/NC/bSwG8Dx3j+3gf4bv5jfDXwUTyvE8AuLxjiX/bewHfxvF4G+GteuJcGfhs4xvP3PsB38+/32sBv8bzeBvhpXjDEv+yzgc/iOV0CjvOieWngt4FjPH/vA3w3/367wDGe0+cAn80LhviX/TbwWjynnwHemhfdSwO/DRzj+Xsf4Lv59/lt4LV4Tr8DvDYvGOJf9lfAS/OcPgf4bJ7trYG3Av4a+B5gl+f10sBvA8d4/t4H+G5euOPAewEvDfwM8NM822cDn8VzuhV4CC8Y4l9mntfnAJ/NFZ8NfBbP9tfAy/D8vTTw28Axnr/3Ab6bF+yvgJfm2T4H+Gyu+Gzgs3he4gVD/MvM83of4Lu54iJwnOf0OsBv8/y9NPDbwDGe0yXgtYG/5vl7beC3eE63Ag/hivcGvovnJV4wxL/MPK/3Ab6bK3aBYzyn1wF+mxfspYHfBo5xxSXgtYG/5gV7beC3eE7PAB7MFR8NfBXPS7xgiH+ZeV6fA3w2V3w28Fk8298AL82/7KWB3+aK1wb+mn/ZXwMvxbN9DvDZXPHZwGfxvMQLhviX3Qo8iOf0OcBn82xvDbw2cCvw3cAuL5qX5oq/5kVzHHhv4MHAbwM/zbN9NfBRPKdnAA/mBUP8y34beC2e0+8Ar83/LL8NvBbP6XeA1+YFQ/zLvhr4KJ7TLnCC/1kuAsd5Tp8DfDYvGOJf9t7Ad/G8Xgf4bf5neGvgp3hebwP8NC8Y4l92HLjI8/oa4KP5n+GrgY/ieZ0AdnnBEC+avwZeiud0K/AywC7/vY4DfwU8mOf0N8BL88IhXjQfDXwVz+tzgM/mv9dnA5/F83of4Lt54RAvmuPArcAxntMu8BBgl/8ex4GnA8d5TpeABwO7vHCIF91XAx/F8/oa4KP5t3ktrvgd/m2+C3hvntfnAJ/Nvwzxonsw8NfAMZ7X+wDfzYvuOPBbwEtzxV8DrwPs8qJ7b+C7eF6XgJcGbuVfhvjX+Wzgs3j+Xgb4a140Hw18Fc/pY4Cv5kXz0sBf8fx9DvDZvGgQ/3p/DbwUz2sXeB3gr/mXfTfwXjynrwE+mn/ZSwO/BRznef0N8NK86BD/ei8N/DZwjOfvfYDv5oV7a+CneE5vA/w0L9x7A9/F83cJeGngVl50iH+b9wa+ixfsu4GPAXZ5wT4beG+u+G7gs3nBjgNfBbw3L9jbAD/Nvw7i3+69ge/iBdsFvhr4GmCXf5vjwEcBHw0c5wV7H+C7+ddD/Pu8N/BdvHC3Aj8N/DTwO7xoXgt4a+CtgQfzwr0P8N382yD+/d4b+GrgGP+yXeCvgb8GdoFbueLBwHHgpYGXBo7zL7sEvDfw0/zbIf5jvDTw3cBL8V/jb4C3Bm7l3wfxH+uzgY8GjvGf4xLw1cBn8x8D8R/vwcBHA+8NHOM/xiXgq4HvBm7lPw7iP89x4L2B9wZein+bvwG+GvhpYJf/eIj/GseBtwYeDLw28GDgQTynZwC3Ar8N/DXw28Au/7kQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcUHSJQP/sKKgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAvTimer;
impl IconShape for MdAvTimer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 17c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm0-14v4h2V5.08c3.39.49 6 3.39 6 6.92 0 3.87-3.13 7-7 7s-7-3.13-7-7c0-1.68.59-3.22 1.58-4.42L12 13l1.41-1.41-6.8-6.8v.02C4.42 6.45 3 9.05 3 12c0 4.97 4.02 9 9 9 4.97 0 9-4.03 9-9s-4.03-9-9-9h-1zm7 9c0-.55-.45-1-1-1s-1 .45-1 1 .45 1 1 1 1-.45 1-1zM6 12c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADrklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv81/sd/nO9Fs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/FfT/znMs/rdYDfBhDP9trAb/Ff77X5z/XbPK/XAX4bQDzbawO/xf8PrwP8NoB4TrvAMf5vuwQc5wrEc/ps4LP4v+1zgM/mCsTz+m7gvfi/6XuA9+bZEM/fewMfDbwU/zf8DvDdwHfznBD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R5f1c0FDslYNAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBrandingWatermark;
impl IconShape for MdBrandingWatermark {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16h-9v-6h9v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fs8r9/hf7fX4nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8Xpv/3X6b5/U6wG8DiGd7beC3+P/hdYDfBhDPaRc4xv9tl4DjXIF4Tp8NfBb/t30O8NlcgXhe3w28F/83fQ/w3jwb4vl7b+CjgZfi/4bfAb4b+G6eE+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CDfya0HAMPTRAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallToAction;
impl IconShape for MdCallToAction {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3v-3h18v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeeEuAR8NfDf/spcGvht4KZ6/vwHeG/hr/mXvDXw1cIwXTrxgiH+ZeeHeB/huXnTHgVuBYzynZwAvDezyontv4Lt44cQLhviXmRdO/Ov9NvBaPKffAV6bfz3zwokXDPEvMy+c+Nf7beC1eE6/A7w2/3rmhRMvGOJfZl448ZzM83od4Ld5tt8GXovn9DvAa/Nsrw38Fs9LPCfzwokXDPEvMy+ceE7mef01sMuzvTRwnOf0O8Br82yvDfwWz+u3eU6vzQsnXjDEv8y8cOI5mX+b3wFem2d7beC3+PcTLxjiX2ZeOPGczL/N7wCvzbO9NvBb/PuJFwzxLzMvnHhO5t/md4DX5tleG/gt/v3EC4b4l5kXTjwn82/zO8Br82yvDfwW/37iBUP8y8wLJ56TeV5/A+zybC8NHOM5/Q7w2jzbawO/xfP6HZ7Ta/HCiRcM8S8zL5x4TuZ5vQ7w2zzbbwOvxXP6HeC1ebbXBn6L5yWek3nhxAuG+JeZF0786/028Fo8p98BXpt/PfPCiRcM8S8zL5z41/tt4LV4Tr8DvDb/euaFEy8Y4l9mXrj3Ab6bF91x4OnAcZ7TLvAQYJcX3XsD38ULJ14wxL/MvHC7wEcD38O/7KWB7wJemufvr4H3Af6af9l7AV8NHOeFEy8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l9m/ncTLxjiX2b+dxMvGOJfZv53Ey8Y4l/218BL8b/T7wCvzQuG+Je9N/Bd/O/0PsB384IhXjTfDbwX/7t8D/DevHCIF91nAx8NHON/tkvAVwOfzb8M8a/32vzP9tu86BD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFEPnxB3idFUQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClosedCaption;
impl IconShape for MdClosedCaption {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 7H9.5v-.5h-2v3h2V13H11v1c0 .55-.45 1-1 1H7c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1zm7 0h-1.5v-.5h-2v3h2V13H18v1c0 .55-.45 1-1 1h-3c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFn0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0O8cK8N/Db/dyFesO8C3ht4H+C7+b8J8fx9F/DePNv7AN/N/z2I5/VdwHvzvN4H+G5esAcDD+K/3+/wokM8pwcDfw0c4/l7H+C7ef6OA78NvBT/vXaBrwY+h38Z4nm9NPDbwDGev/cBvpvn7zjw28BL8d/vu4H34YVDPH8vDfw2cIzn732A7+b5Ow78NvBS/Pd7H+C7ecEQL9hLA78NHOP5ex/gu3n+jgO/DbwU/71+B3htXjDEC/fSwF/xgr0P8N08f8eB3wZeiv9e4gVD/MvMC/c+wHfz/B0Hfht4Kf77iBcM8S8z/7L3Ab6b5+848NvAS/HfQ7xgiH+ZedG8D/DdPH/Hgd8GXor/euIFQ/zLzIvufYDv5vk7Dvw28FL81xIvGOJfZv513gf4bp6/48BvAy/Ffx3xgiH+ZeZf732A7+b5Ow78NvBS/NcQLxjiX2b+bd4H+G6ev+PAbwMvxX8+8YIh/mXm3+59gO/m+TsO/DbwUvznEi8Y4l9mXrhLXHGM5+99gO/m+TsO/DbwUlzxN8B7A3/Nv+y9ga8GjvHCiRcM8S8zL9z7AH8N/DZwjOfvfYDv5vk7Dvw2cBx4aWCXF917A9/FCydeMMS/zLxwJ4Bd4KWB3waO8fy9D/DdPH/Hgc8GPpp/PfPCiRcM8S8zL5x4tpcGfhs4xvP3PsB38x/LvHDiBUP8y8wLJ56TeeHeB/hu/mWvDfwWz0s8J/PCiRcM8S8zL5x4TuZf9j7Ad/PCvTbwWzyv3+Y5vTYvnHjBEP8y88KJ52ReNO8DfDcv2GsDv8W/n3jBEP8y88KJ52RedO8DfDfP33Hgt4GX4t9HvGCIf5l54cRzMv867wN8N8/fceC3gZfi3068YIh/mXnhxHMy/3rvA3w3z99x4LeBl+LfRrxgiH+ZeeHEczLP62+AXeClgWM8f+8DfDfP33Hgt4GXAn6H5/RavHDiBUP8y8wLJ56TeV6vA/w28NLAbwPHeP7eB/hunr/jwG8DL81zMi+ceMEQ/zLzwol/nZcGfhs4xvP3PsB386IzL5x4wRD/MvPCiX+93wZeixfsfYDv5l92HLjICydeMMS/zLxw7wN8Ny+648DTgeO8cO8DfDcv3HsD38ULJ14wxL/MvHC7wEcD38O/7KWB7wJemhfN+wDfzQv228Br8cKJFwzxLzP/vd4H+G6ev+PAbwMvxQsmXjDEv8z893sf4Lt5/o4Dvw28FM+feMEQ/zLzP8P7AN/N83cc+G3gpXhe4gVD/MvM/xzvA3w3z99x4LeBl+I5iRcM8S8z/7O8D/DdPH/Hgd8GXopnEy8Y4l9m/ud5H+C7ef6OA78NvBRXiBcM8S8z/zO9D/DdPH/Hgd8GXgoQLxjiX2b+53of4Lt5/o4Dvw28NC8Y4l9m/md7H+C7ef6OA7u8YIh/2V8DL8X/bO8DfDf/eoh/2XsD38X/fO8DfDf/OogXzXcD78X/fO8DfDcvOsSL7rOBjwaO8T/b+wDfzYsG8a/32vz3Ow58N3CM5+99gO/mX4b43+ulgd8GjvH8vQ/w3bxwiP/dXhr4beAYz9/7AN/NC4b43++lgd8GjvH8PQS4lecP8X/DSwO/DRzjOb0P8N28YIj/O14a+G3gGFd8D/DevHCI/1teGvht4KeB9+Zfhvi/58HArbxoEP+/If5/Q/z/xj8CTda5QRUIvWUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClosedCaptionDisabled;
impl IconShape for MdClosedCaptionDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.83,4H19c1.1,0,2,0.9,2,2v12c0,0.05-0.01,0.1-0.02,0.16l-3.38-3.38C17.84,14.59,18,14.32,18,14v-1h-1.5v0.5h-0.17 l-1.83-1.83V10.5h2V11H18v-1c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v0.17L6.83,4z M19.78,22.61L17.17,20H5c-1.11,0-2-0.9-2-2V6 c0-0.05,0.02-0.1,0.02-0.15L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M11,13.83L10.17,13H9.5v0.5h-2v-3h0.17L6.4,9.22 C6.16,9.41,6,9.68,6,10v4c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1V13.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEk0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NM/2O/zP9lo82+8Ar80LhviXmeck/mczz0m8YIh/mXlO4n8285zEC4b4l5nnJP5nM89JvGCIf5l5TuJ/NvOcxAuG+JeZ5yT+ZzPPSbxgiH+ZeU7ifzbznMQLhviXmeck/mczz0m8YIh/mXlO4n8285zEC4b4l5nnJP5nM89JvGCIf5l5TuI5HQe+Cnhv/mV/DbwP8Nc8fy8NfBfw0vzLvhv4GGCX52Sek3jBEP8y85zEc/pu4L140e0CDwF2eU4PBv4KOM6L7nuA9+Y5meckXjDEv8w8J/GczL/e6wC/zXN6beC3+NcTz8k8J/GCIf5l5jmJ52T+9V4H+G2e02sDv8W/nnhO5jmJFwzxLzPPSTwn85zE8/pt4LV4ttcBfpvn9NrAb/FsvwO8Ns/LPCfxnMxzEi8Y4l9mnpN4TuY5ief10sBxnu2vgV2e02sDv8Wz/Q7w2jyv1+Y5/TbPyTwn8YIh/mXmOYnnZJ6T+Ld5beC3eLbfAV6bfz3znMQLhviXmecknpN5TuLf5rWB3+LZfgd4bf71zHMSLxjiX2aek3hO5jmJf5vXBn6LZ/sd4LX51zPPSbxgiH+ZeU7iOZnnJP5tXhv4LZ7td4DX5l/PPCfxgiH+ZeY5iedknpN4Xi8NHOPZ/gbY5Tm9NvBbPNvvAK/N83otntPv8JzMcxIvGOJfZp6TeE7mOYnn9dvAa/FsrwP8Ns/ptYHf4tl+B3htnpd5TuI5meckXjDEv8w8J/GczL/e6wC/zXN6beC3+NcTz8k8J/GCIf5l5jmJ52T+9V4H+G2e02sDv8W/nnhO5jmJFwzxLzPPSTyn7wbeixfdJeDBwC7P6ThwK3CMF933AO/NczLPSbxgiH+ZeU7iOR0Hvhp4L/5lfwO8N/DXPH8vDXw38FL8y74H+Ghgl+dknpN4wRD/MvOcxP9s5jmJFwzxLzPPSfzPZp6TeMEQ/zLznMT/bOY5iRcM8S8zz0n8z2aek3jBEP8y85zE/2zmOYkXDPEvM89J/M9mnpN4wRD/MvOcxP9s5jmJFwzxLzPPSfzPZp6TeMEQ/zLznMT/bOY5iRcM8S/7a+CleLbf5n+21+bZfgd4bV4wxL/svYHv4n+n9wG+mxcM8aL5buC9+N/le4D35oVDvOg+G/ho4Bj/s10Cvhr4bP5liH+91+Z/tt/mRYf4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wgQ2qBBnr8OuAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClosedCaptionOff;
impl IconShape for MdClosedCaptionOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.5 5.5v13h-15v-13h15zM19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 7H9.5v-.5h-2v3h2V13H11v1c0 .55-.45 1-1 1H7c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1zm7 0h-1.5v-.5h-2v3h2V13H18v1c0 .55-.45 1-1 1h-3c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdControlCamera;
impl IconShape for MdControlCamera {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.54 5.54L13.77 7.3 12 5.54 10.23 7.3 8.46 5.54 12 2zm2.92 10l-1.76-1.77L18.46 12l-1.76-1.77 1.76-1.77L22 12zm-10 2.92l1.77-1.76L12 18.46l1.77-1.76 1.77 1.76L12 22zm-2.92-10l1.76 1.77L5.54 12l1.76 1.77-1.76 1.77L2 12z",
            }
            circle {
                cx: "12",
                cy: "12",
                r: "3",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEqualizer;
impl IconShape for MdEqualizer {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 20h4V4h-4v16zm-6 0h4v-8H4v8zM16 9v11h4V9h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADXklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPz7/A7/Pq/Fv494wRD/MvPvI/59zL+PeMEQ/zLz7yP+fcy/j3jBEP8y8+8j/n3Mv494wRD/MvPvI/59zL+PeMEQ/zLz7yP+fcy/j3jBEP8y8+8jntdvA6/Ffw3xgiH+ZebfRzyv3wZei/8a4gVD/MvMv494Xr8NvBb/NcQLhviXmX8f8bx+G3gt/muIFwzxLzP/PuJ5/TbwWvzXEC8Y4l9m/n3E83pp4Dgvmt/i30e8YIh/mfn3Ef8+5t9HvGCIf5n59xH/PubfR7xgiH+Z+fcR/z7m30e8YIh/mfn3Ef8+5t9HvGCIf5n59xHP66WBY7xofpt/H/GCIf5l5t9HPK/fBl6L/xriBUP8y8y/j3hevw28Fv81xAuG+JeZfx/xvH4beC3+a4gXDPEvM/8+4nn9NvBa/NcQLxjiX2b+fcTz+m3gtfivIV4wxL/M/PuIfx/z7yNeMMS/zPz7iH8f8+8jXjDEv8z8+4h/H/PvI14wxL/M/PuIfx/z7yNeMMS/zPz7iH8f8+8jXjDEv8z8+/w2/z6vzb+PeMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP+yXeAY/ztdAo7zgiH+Zd8NvBf/O30P8N68YIh/2YOBp/O/00OAW3nBEC+a9wa+i/9d3gf4bl44xIvuwcBnA28NHON/pkvATwOfDdzKvwzx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSY8lpB7vSQzAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdExplicit;
impl IconShape for MdExplicit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-4 6h-4v2h4v2h-4v2h4v2H9V7h6v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeNG9NPBawNfwn+OlgdcCvob/OogX3WsDvwX8NPA+wC7/sV4b+C3gp4H3AXb5z4d40b028FtcsQu8D/DT/Md5beC3uGIXeB/gp/nPhXjRvTbwWzynrwY+B9jl3++1gd/iOX018DnALv85EC+61wZ+i+f118D7AH/Nv89rA7/F8/pr4H2Av+Y/HuJF99rAb/GCfTbwOfzbvTbwW7xgnw18Dv+xEC+61wZ+ixfut4H3AW7lX++1gd/ihftt4H2AW/mPgXjRvTbwW/zLdoH3AX6af53XBn6Lf9ku8D7AT/Pvh3jRvTbwW7zofhp4H2CXF81rA7/Fi+6ngfcBdvm3Q7zoXhv4Lf51bgXeB/ht/mWvDfwW/zq3Au8D/Db/NogX3WsDv8W/zWcDn8ML99rAb/Fv89nA5/Cvh3jRvTbwW/zb/TXwPsBf8/y9NvBb/Nv9NfA+wF/zokO86F4b+C3+fXaBzwa+huf12sBv8e+zC3w28DW8aBAvutcGfov/GD8NvA+wy7O9NvBb/Mf4aeB9gF1eOMSL7rWB3+I/zi7wPsBPc8VrA7/Ff5xd4H2An+YFQ7zoXhv4Lf7jfTXwOcBLA7/Ff7yvBj4H2OV5IV50rw38Fv/xvgb4bOClgd/iP97XAJ8N7PK8EC+61wZ+i/84l4D3Bn6aK14b+C3+41wC3hv4aV4wxIvutYHf4j/GzwDvDezybK8N/Bb/MX4GeG9glxcO8aJ7beC3+Pe5BHw28NU8r9cGfot/n0vAZwNfzYsG8aJ7beC3+Lf7G+C9gb/m+Xtt4Lf4t/sb4L2Bv+ZFh3jRvTbwW/zbfA7w2bxwrw38Fv82nwN8Nv96iBfdawO/xb/OM4D3Bn6bf9lrA7/Fv84zgPcGfpt/G8SL7rWB3+JF9zPAewO7vGheG/gtXnQ/A7w3sMu/HeJF99rAb/EvuwS8N/DT/Ou8NvBb/MsuAe8N/DT/fogX3WsDv8UL9zvAewO38q/32sBv8cL9DvDewK38x0C86F4b+C1esM8BPpt/u9cGfosX7HOAz+Y/FuJF99rAb/G8/gZ4b+Cv+fd5beC3eF5/A7w38Nf8x0O86F4b+C2e09cAnw3s8u/32sBv8Zy+BvhsYJf/HIgX3WsDv8UVl4D3Bn6a/zivDfwWV1wC3hv4af5zIV50rw38FvAzwHsDu/zHem3gt4CfAd4b2OU/H+JF99LAawNfzX+OlwZeG/hq/usg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPZOeuQZT/i6MAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFastForward;
impl IconShape for MdFastForward {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K/3UcDvAH/Nf46PAn4H+Gv+ZYj/OseB7wLeGngd4Lf5j3Uc+C7grYHXAX6bfxniv8ZbA98FHOeK1wF+m/84bw18F3CcK14H+G3+ZYj/XMeBzwI+muf0OsBv8+93HPgs4KN5Tq8D/Db/MsR/npcGvgt4aZ7X6wC/zb/PSwPfBbw0z+t1gN/mX4b4z/FZwGfzgr0O8Nv8230W8Nm8YK8D/Db/MsR/rAcD3wW8Ni/c6wC/zb/eg4HvAl6bF+51gN/mX4b4j/PWwHcBx/mXvQ7w2/zrvDXwXcBx/mWvA/w2/zLEv99x4LuAt+ZF9zrAb/OiOQ58F/DWvOheB/ht/mWIf5/XBr4LeDD/Oq8D/Db/stcGvgt4MP86rwP8Nv8yxL/dZwGfzb/N6wC/zQv3WcBn82/zOsBv8y9D/Ou9NPBdwEvzb/c6wG/z/L008F3AS/Nv9zrAb/MvQ/zrfBTw2cBx/n1eB/htntdHAZ8NHOff53WA3+ZfhnjRHAe+C3hr/mO8DvDbPNtx4LuAt+Y/xusAv82/DPEve2vgu4Dj/Md5HeC3ueKtge8CjvMf53WA3+ZfhnjBjgOfBXw0//FeB/hr4LOAj+Y/3usAv82/DPGCHQc+G/go/uO9DvDXwGcDH8V/vNcBfpt/GeJf9tbAdwPH+I/zOsBvc8VbA98NHOM/zusAv82/DPGiOQ58N/BW/Md4HeC3ebbjwHcDb8V/jNcBfpt/GeJf56OBzwaO8e/zOsBv87w+Gvhs4Bj/Pq8D/Db/MsS/3ksD3w28FP92rwP8Ns/fSwPfDbwU/3avA/w2/zLEv91nA5/Fv83rAL/NC/fZwGfxb/M6wG/zL0P8+7w28N3Ag/jXeR3gt/mXvTbw3cCD+Nd5HeC3+Zch/v2OA98NvBUvutcBfpsXzXHgu4G34kX3OsBv8y9D/Md5a+C7gWP8y14H+G3+dd4a+G7gGP+y1wF+m38Z4j/Wg4HvBl6LF+51gN/mX+/BwHcDr8UL9zrAb/MvQ/zn+Gzgs3jBXgf4bf7tPhv4LF6w1wF+m38Z4j/PSwPfDbwUz+t1gN/m3+elge8GXorn9TrAb/MvQ/znOg58NvBRPKfXAX6bf7/jwGcDH8Vzeh3gt/mXIf5rvDXw3cAxrngd4Lf5j/PWwHcDx7jidYDf5l+G+K9zHPhu4K2A1wF+m/9Yx4HvBt4KeB3gt/mXIf7rfTTw28Bf85/jo4HfBv6afxni/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IzpmAQZaQal8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFastRewind;
impl IconShape for MdFastRewind {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7l+i/96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovn9dr85/pt/uu9DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xfMS/7nMf73XAX4bQDzbawO/xfMS/7nMf73XAX4bQDzbawO/xfMS/7nMf73XAX4bQDzbawO/xfMS/7nMf73XAX4bQDzbawO/xfMS/7nMf73XAX4bQDzbawO/xfP6bf5zvTb/9V4H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDynXeAY/7ddAo5zBeI5fTbwWfzf9jnAZ3MF4nl9N/Be/N/0PcB782yI5++9gY8GXor/G34H+G7gu3lOiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjOJ1uQWIwzAoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFeaturedPlayList;
impl IconShape for MdFeaturedPlayList {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 8H3V9h9v2zm0-4H3V5h9v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7l+i/96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfovnJf5zmf96rwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiOe0Cxzj/7ZLwHGuQDynzwY+i//bPgf4bK5APK/vBt6L/5u+B3hvng3x/L038NHAS/F/w+8A3w18N88J8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EalZ1Qc2QX6gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFeaturedVideo;
impl IconShape for MdFeaturedVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 9H3V5h9v7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFiberDvr;
impl IconShape for MdFiberDvr {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.5,10.5h2v1h-2V10.5z M4.5,10.5h2v3h-2V10.5z M21,3H3C1.89,3,1,3.89,1,5v14c0,1.1,0.89,2,2,2h18c1.11,0,2-0.9,2-2V5 C23,3.89,22.11,3,21,3z M8,13.5C8,14.35,7.35,15,6.5,15H3V9h3.5C7.35,9,8,9.65,8,10.5V13.5z M12.62,15h-1.5L9.37,9h1.5l1,3.43 l1-3.43h1.5L12.62,15z M21,11.5c0,0.6-0.4,1.15-0.9,1.4L21,15h-1.5l-0.85-2H17.5v2H16V9h3.5c0.85,0,1.5,0.65,1.5,1.5V11.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADlUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dLA8d4TpeAv+a/FuK/xlsBbw28NvBgXrhbgd8Gfhr4Gf5zIf7zPBj4LOCtgeP82+wC3w18DXAr//EQ//GOA58FfDT/sb4a+Bxgl/84iP9Yrw38FHCc/xy7wNsAv81/DMR/nM8CPpv/Gp8NfA7/foj/GN8FvDf/tb4beB/+fRD/ft8FvDf/Pb4beB/+7RD/Pp8NfBb/vT4H+Gz+bRD/dq8N/Bb/M7wO8Nv86yH+bY4DTweO8z/DrcDLALv86yD+bb4a+Cj+Z/ka4KP510H86z0YeDr/Mz0EuJUXHeJf77uB9+J/pq8BPpoXHeJf7yJwnP+ZdoETvOgQ/zpvDfwU/7O9DfDTvGgQ/zrfDbwX/7N9DfDRvGgQ/zpPBx7M/2y3Ag/hRYP41zH/O4gXDeJF99LAX/G/w8sAf82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+6lwb+iv8dXgb4a/5liH8d87+DeNEg/nVuBR7E/2zPAB7Miwbxr/PdwHvxP9vXAB/Niwbxr/PWwE/xP9vbAD/Niwbxr7cLHON/pkvAcV50iH+9rwY+iv+ZPgf4bF50iH+9BwNP53+mhwC38qJD/Nt8NfBR/M/yNcBH86+D+Lc5DtwKHON/hmcALw3s8q+D+Ld7beC3+J/hdYDf5l8P8e/z2cBn8d/rc4DP5t8G8e/33cB78d/je4D35t8O8R/ju4H34r/W9wDvzb8P4j/OZwOfxX+NzwE+m38/xH+s1wa+G3gQ/zmeAbw38Nv8x0D8xzsOfDbwUfzH+hrgs4Fd/uMg/vM8GPho4L2BY/zbXAK+Gvhu4Fb+4yH+a7w18NrAWwMP4oV7BvDTwG8DP81/LsR/j5cGjvOcdoG/5r8W4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjkQZkQYizjhQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFiberManualRecord;
impl IconShape for MdFiberManualRecord {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cy: "12",
                r: "8",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFiberNew;
impl IconShape for MdFiberNew {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,4H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h16c1.11,0,2-0.89,2-2V6C22,4.89,21.11,4,20,4z M8.5,15H7.3 l-2.55-3.5V15H3.5V9h1.25l2.5,3.5V9H8.5V15z M13.5,10.26H11v1.12h2.5v1.26H11v1.11h2.5V15h-4V9h4V10.26z M20.5,14 c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1V9h1.25v4.51h1.13V9.99h1.25v3.51h1.12V9h1.25V14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFiberPin;
impl IconShape for MdFiberPin {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 10.5h2v1h-2zM20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zM9 11.5c0 .85-.65 1.5-1.5 1.5h-2v2H4V9h3.5c.85 0 1.5.65 1.5 1.5v1zm3.5 3.5H11V9h1.5v6zm7.5 0h-1.2l-2.55-3.5V15H15V9h1.25l2.5 3.5V9H20v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP8dLA8d4TpeAv+Z/FsR/jLcC3hp4beDBvHC3Ar8N/DTwM/zbfDTw28Bf8++D+Ld7MPBZwFsDx/m32QW+G/ga4FZedL8NvBZwK/DdwNcAu/zrIf71jgOfBXw0/7G+GvgcYJd/2W8Dr8Wz7QJfDXwNsMuLDvGv89rATwHH+c+xC7wN8Nu8cL8NvBbP61bgbYC/5kWDeNF9FvDZ/Nf4bOBzeMF+G3gtXrCPBr6GfxniRfNdwHvzX+u7gffh+ftt4LV44b4beB9eOMS/7LuA9+a/x3cD78PzOg68N/DRwIN4wb4beB9eMMQL99nAZ/Hf63OAz+YF+2zgs3jB3gf4bp4/xAv22sBv8T/D6wC/zQv20sBvA8d4/l4H+G2eF+L5Ow48HTjO/wy3Ai8D7PKCHQd+G3gpntetwMsAuzwnxPP31cBH8T/L1wAfzQt3HLgVOMbz+hrgo3lOiOf1YODp/M/0EOBWXriXBv6K57ULPATY5dkQz+u7gffif6avAT6af9lnA5/F8/oc4LN5NsTzuggc53+mXeAEL5pbgQfxnHaBEzwb4jm9NfBT/M/2NsBP8y/7aOCreF6vA/w2VyCe03cD78X/bF8DfDT/suPArcAxntPXAB/NFYjn9HTgwfzPdivwEF40Pw28Fc/pr4GX4QrEczL/O4gXzUcDX8XzElcgnu2lgb/if4eXAf6af9lrA7/F83oIcCuAeLbXBn6L/x1eB/ht/mUPBp7O83od4LcBxLO9NvBb/O/wOsBv86Ixz+t1gN8GEM/22sBv8b/D6wC/zYvGPK/XAX4bQDzbawO/xf8OrwP8Nv+yBwNP53m9DvDbAOLZXhr4K/53eBngr/mXvTbwWzyvhwC3AojnZP53EC+ajwa+iuclrkA8p1uBB/E/2zOAB/Oi+WngrXhOfwO8NFcgntN3A+/F/2xfA3w0/7LjwNOB4zynrwE+misQz+mtgZ/if7a3AX6af9lHA1/F83od4Le5AvG8doFj/M90CTjOv+w48FfAg3lOl4DjPBvieX018FH8z/Q5wGfzL/ts4LN4Xp8DfDbPhnheDwaezv9MDwFu5YV7aeCveF6XgAcDuzwb4vn7auCj+J/la4CP5oU7DjwdOM7z+hzgs3lOiOfvOHArcIz/GZ4BvDSwywt2HPgt4KV5Xs8AHszzQrxgrw38Fv8zvA7w27xgLw38FnCc5+91gN/meSFeuM8GPov/Xp8DfDYv2GcBn80L9j7Ad/P8If5l3w28F/89vgd4b57XceC9gI8GHswL9j3Ae/OCIV403w28F/+1vgd4b56/3wZeixfue4D35oVDvOg+G/gs/mt8DvDZvGC/DbwWL9j7AN/Nvwzxr/PawHcDD+I/xzOA9wZ+mxfut4HX4nk9A3hr4K950SD+9Y4Dnw18FP+xvgb4bGCXf9lvA6/Fs10Cvhr4amCXFx3i3+7BwEcD7w0c49/mEvDVwHcDt/Ki+23gtYBnAN8NfDWwy78e4j/GWwOvDbw18CBeuGcAPw38NvDT/Nu8N/DXwF/z74P4z/HSwHGe0y7w1/zPgvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/COwbw0G4XrUVAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFiberSmartRecord;
impl IconShape for MdFiberSmartRecord {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cy: "12",
                r: "8",
            }
            path {
                d: "M17 4.26v2.09c2.33.82 4 3.04 4 5.65s-1.67 4.83-4 5.65v2.09c3.45-.89 6-4.01 6-7.74s-2.55-6.85-6-7.74z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForward10;
impl IconShape for MdForward10 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,13c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6v4l5-5l-5-5v4c-4.42,0-8,3.58-8,8c0,4.42,3.58,8,8,8s8-3.58,8-8H18z",
            }
            polygon {
                points: "10.86,15.94 10.86,11.67 10.77,11.67 9,12.3 9,12.99 10.01,12.68 10.01,15.94",
            }
            path {
                d: "M12.25,13.44v0.74c0,1.9,1.31,1.82,1.44,1.82c0.14,0,1.44,0.09,1.44-1.82v-0.74c0-1.9-1.31-1.82-1.44-1.82 C13.55,11.62,12.25,11.53,12.25,13.44z M14.29,13.32v0.97c0,0.77-0.21,1.03-0.59,1.03c-0.38,0-0.6-0.26-0.6-1.03v-0.97 c0-0.75,0.22-1.01,0.59-1.01C14.07,12.3,14.29,12.57,14.29,13.32z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForward30;
impl IconShape for MdForward30 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,13c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6v4l5-5l-5-5v4c-4.42,0-8,3.58-8,8c0,4.42,3.58,8,8,8s8-3.58,8-8H18z",
            }
            path {
                d: "M10.06,15.38c-0.29,0-0.62-0.17-0.62-0.54H8.59c0,0.97,0.9,1.23,1.45,1.23c0.87,0,1.51-0.46,1.51-1.25 c0-0.66-0.45-0.9-0.71-1c0.11-0.05,0.65-0.32,0.65-0.92c0-0.21-0.05-1.22-1.44-1.22c-0.62,0-1.4,0.35-1.4,1.16h0.85 c0-0.34,0.31-0.48,0.57-0.48c0.59,0,0.58,0.5,0.58,0.54c0,0.52-0.41,0.59-0.63,0.59H9.56v0.66h0.45c0.65,0,0.7,0.42,0.7,0.64 C10.71,15.11,10.5,15.38,10.06,15.38z",
            }
            path {
                d: "M13.85,11.68c-0.14,0-1.44-0.08-1.44,1.82v0.74c0,1.9,1.31,1.82,1.44,1.82c0.14,0,1.44,0.09,1.44-1.82V13.5 C15.3,11.59,13.99,11.68,13.85,11.68z M14.45,14.35c0,0.77-0.21,1.03-0.59,1.03c-0.38,0-0.6-0.26-0.6-1.03v-0.97 c0-0.75,0.22-1.01,0.59-1.01c0.38,0,0.6,0.26,0.6,1.01V14.35z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForward5;
impl IconShape for MdForward5 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,13c0,3.31-2.69,6-6,6s-6-2.69-6-6s2.69-6,6-6v4l5-5l-5-5v4c-4.42,0-8,3.58-8,8c0,4.42,3.58,8,8,8c4.42,0,8-3.58,8-8 H18z",
            }
            path {
                d: "M12.03,15.38c-0.44,0-0.58-0.31-0.6-0.56h-0.84c0.03,0.85,0.79,1.25,1.44,1.25c0.93,0,1.44-0.63,1.44-1.43 c0-1.33-0.97-1.44-1.3-1.44c-0.2,0-0.43,0.05-0.64,0.16l0.11-0.92h1.7v-0.71h-2.39l-0.25,2.17l0.67,0.17 c0.13-0.13,0.28-0.23,0.57-0.23c0.4,0,0.69,0.23,0.69,0.75C12.62,14.64,12.65,15.38,12.03,15.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEL0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/+X6Lf5/X4T8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP987w18F/823wO8N/95EP813hv4Lv51vgd4b/5zIf7rvDfwXbxovgd4b/7zIf5rvTfwXbxw3wO8N/81EM/fbwOvxb/NdwPvwwv23sB38fx9D/DevGDvDXwX/za/A7w2zwnx/P028Fr823038D68YO8NfBfP6XuA9+YFe2/gu/i3+x3gtXlOiOfvt4HX4t/nu4H34QV7b+C7uOJ7gPfmBXtv4Lv49/kd4LV5Tojn77eB1+Lf77uB9+EFe2/gtYH35gV7b+C7+Pf7HeC1eU6I5++3gdfiP8Z3A+/Dv817A9/Ff4zfAV6b54R4/n4beC3+43w38D7867w38F38x/kd4LV5Tojn77eB1+I/1ncD78OL5r2B7+I/1u8Ar81zQjx/vw28Fv/xvht4H1649wa+i/94vwO8Ns8J8fz9NvBa/Mf7HuC9eeHeG/gu/uP9DvDaPCfE8/fbwGvxH+t7gPfmRfPewHfxH+t3gNfmOSGev98GXov/ON8DvDf/Ou8NfBf/cX4HeG2eE+L5+23gtfiP8T3Ae/Nv897Ad/Ef43eA1+Y5IZ6/3wZei3+/7wHemxfsvYHXAt6HF+y9ge/i3+93gNfmOSGev98GXot/n+8B3psX7L2B7+KK7wbehxfsvYHv4t/nd4DX5jkhnr/fBl6Lf7vvAd6bF+y9ge/iOX038D68YO8NfBf/dr8DvDbPCfH8/TbwWvzbfA/w3rxg7w18F8/fdwPvwwv23sB38W/zO8Br85wQ/7XeG/guXrjvBt6H/xqI/zrvDXwXL5rvBt6H/3yI/xrvDXwX/zrfDbwP/7kQ//neG/gu/m2+G3gf/vMg/vOZfx/xnwfxn8/8+4j/PIj/fObfR/znQfznM/8+4j8P4j+f+fcR/3kQ//nMv4/4z4P4z2f+fcR/HsR/PvPvI/7zIP7zmX8f8Z8H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M/32/z7vDb/eRD/vyH+f0P8/4b4/w3x/xv/CM5mfEFlx04RAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdGames;
impl IconShape for MdGames {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeaFE8/JPK/XAX6bZ/tt4LV4Tr8DvDYvuvcGvho4xgsnXjDEv8y8cOI5mef1OsBv82y/DbwWz+l3gNfmX+e9ge/ihRMvGOJfZl448ZzM83od4Ld5tt8GXovn9DvAa/OvZ1448YIh/mXmhRPPyTyv1wF+m2f7beC1eE6/A7w2/3rmhRMvGOJfZl448ZzM83od4Ld5tt8GXovn9DvAa/Nsrw38Fs9LPCfzwokXDPEvMy+ceE7meb0O8Ns8228Dr8Vz+h3gtXm21wZ+i+clnpN54cQLhviXmRdOPKff5nl9NPDXPNtvA6/Fc/od4LV5ttcGfovnJZ6TeeHEC4b4l5kXTvzr/TbwWjyn3wFem2d7beC3eF7iOZkXTrxgiH+ZeeHEv95vA6/Fc/od4LV5ttcGfovnJZ6TeeHEC4b4l5kXTvzr/TbwWjyn3wFem2d7beC3eF7iOZkXTrxgiH+ZeeHEc3otntffALs8228Dr8Vz+h3gtXm21wZ+i+clnpN54cQLhviXmRdOPCfzvF4H+G2e7beB1+I5/Q7w2jzbawO/xfMSz8m8cOIFQ/zLzAsnnpN5Xq8D/DbP9tvAa/Gcfgd4bf71zAsnXjDEv8y8cOI5mef1OsBv82y/DbwWz+l3gNfmX8+8cOIFQ/zLzAsnnpN5Xq8D/DbP9tvAa/Gcfgd4bf513hv4Ll448YIh/mXmhRPPyTyv1wF+m2f7beC1eE6/A7w2L7r3Ar4aOM4LJ14wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/zPzvJl4wxL/M/O8mXjDEv8z87yZeMMS/bBc4xv9Ol4DjvGCIf9l3A+/F/07fA7w3LxjiX/bSwF/xv9NDgFt5wRAvmvcGvov/Xd4H+G5eOMSL7sHAZwNvDRzjf6ZLwE8Dnw3cyr8M8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EV+iQQVppxmMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHd;
impl IconShape for MdHd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 12H9.5v-2h-2v2H6V9h1.5v2.5h2V9H11v6zm2-6h4c.55 0 1 .45 1 1v4c0 .55-.45 1-1 1h-4V9zm1.5 4.5h2v-3h-2v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHearing;
impl IconShape for MdHearing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 20c-.29 0-.56-.06-.76-.15-.71-.37-1.21-.88-1.71-2.38-.51-1.56-1.47-2.29-2.39-3-.79-.61-1.61-1.24-2.32-2.53C9.29 10.98 9 9.93 9 9c0-2.8 2.2-5 5-5s5 2.2 5 5h2c0-3.93-3.07-7-7-7S7 5.07 7 9c0 1.26.38 2.65 1.07 3.9.91 1.65 1.98 2.48 2.85 3.15.81.62 1.39 1.07 1.71 2.05.6 1.82 1.37 2.84 2.73 3.55.51.23 1.07.35 1.64.35 2.21 0 4-1.79 4-4h-2c0 1.1-.9 2-2 2zM7.64 2.64L6.22 1.22C4.23 3.21 3 5.96 3 9s1.23 5.79 3.22 7.78l1.41-1.41C6.01 13.74 5 11.49 5 9s1.01-4.74 2.64-6.36zM11.5 9c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5-1.12-2.5-2.5-2.5-2.5 1.12-2.5 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHearingDisabled;
impl IconShape for MdHearingDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.03,3.2C7.15,2.44,8.51,2,10,2c3.93,0,7,3.07,7,7c0,1.26-0.38,2.65-1.07,3.9c-0.02,0.04-0.05,0.08-0.08,0.13l-1.48-1.48 C14.77,10.69,15,9.8,15,9c0-2.8-2.2-5-5-5C9.08,4,8.24,4.26,7.5,4.67L6.03,3.2z M17.21,14.38l1.43,1.43C20.11,13.93,21,11.57,21,9 c0-3.04-1.23-5.79-3.22-7.78l-1.42,1.42C17.99,4.26,19,6.51,19,9C19,11.02,18.33,12.88,17.21,14.38z M10,6.5 c-0.21,0-0.4,0.03-0.59,0.08l3.01,3.01C12.47,9.4,12.5,9.21,12.5,9C12.5,7.62,11.38,6.5,10,6.5z M21.19,21.19L2.81,2.81L1.39,4.22 l2.13,2.13C3.19,7.16,3,8.05,3,9h2c0-0.36,0.05-0.71,0.12-1.05l6.61,6.61c-0.88,0.68-1.78,1.41-2.27,2.9c-0.5,1.5-1,2.01-1.71,2.38 C7.56,19.94,7.29,20,7,20c-1.1,0-2-0.9-2-2H3c0,2.21,1.79,4,4,4c0.57,0,1.13-0.12,1.64-0.35c1.36-0.71,2.13-1.73,2.73-3.55 c0.32-0.98,0.9-1.43,1.71-2.05c0.03-0.02,0.05-0.04,0.08-0.06l6.62,6.62L21.19,21.19z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeeHEczLP63WA3+bZfht4LZ7X3wDvDfw1/7L3Br4aOMYLJ14wxL/MvHDiOZnn9TrAb/Nsvw28Fs/pGcBLA7u86N4b+C5eOPGCIf5l5oUTz8k8r9cBfptn+23gtXhOvwO8Nv965oUTLxjiX2ZeOPGczPN6HeC3ebbfBl6L5/Q7wGvzr2deOPGCIf5l5oUTz8k8r9cBfptn+23gtXhOvwO8Ns/22sBv8bzEczIvnHjBEP8y88KJ52Se1+sAv82z/TbwWjyn3wFem2d7beC3eF7iOZkXTrxgiH+ZeeHEc/ptntdHA3/Ns/028Fo8p98BXptne23gt3he4jmZF068YIh/mXnhxL/ebwOvxXP6HeC1ebbXBn6L5yWek3nhxAuG+JeZF0786/028Fo8p98BXptne23gt3he4jmZF068YIh/mXnhxL/ebwOvxXP6HeC1ebbXBn6L5yWek3nhxAuG+JeZF048p9fief0NsMuz/TbwWjyn3wFem2d7beC3eF7iOZkXTrxgiH+ZeeHEczLP63WA3+bZfht4LZ7T7wCvzbO9NvBbPC/xnMwLJ14wxL/MvHDiOZnn9TrAb/Nsvw28Fs/pd4DX5l/PvHDiBUP8y8wLJ56TeV6vA/w2z/bbwGvxnH4HeG3+9cwLJ14wxL/MvHDiOZnn9TrAb/Nsvw28Fs9pF3gIsMuL7r2B7+KFEy8Y4l9mXjjxnMzzeh3gt3m23wZei+f118D7AH/Nv+y9gK8GjvPCiRcM8S8z/31eB/htnu23gdfiX0e8YIh/mfnv8zrAb/Nsvw28Fv864gVD/MvMf5/XAX6bZ/tt4LX41xEvGOJfZv77vA7w2zzbbwOvxb+OeMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP+yvwZeiv+dfgd4bV4wxL/svYHv4n+n9wG+mxcM8aL5buC9+N/le4D35oVDvOg+G/ho4Bj/s10Cvhr4bP5liH+91+Z/tt/mRYf4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjbB5RBX4M9SgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHighQuality;
impl IconShape for MdHighQuality {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 11H9.5v-2h-2v2H6V9h1.5v2.5h2V9H11v6zm7-1c0 .55-.45 1-1 1h-.75v1.5h-1.5V15H14c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v4zm-3.5-.5h2v-3h-2v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLibraryAdd;
impl IconShape for MdLibraryAdd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-1 9h-4v4h-2v-4H9V9h4V5h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLibraryAddCheck;
impl IconShape for MdLibraryAddCheck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7.53 12L9 10.5l1.4-1.41 2.07 2.08L17.6 6 19 7.41 12.47 14zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/sd4KeGngtfnvtQv8NfAzwF/zgiH+YzwY+Cngpfmf56uBj+H5Q/z7HQf+Cngw/3N9DfDRPC/Ev99nA5/F/3wPAW7lOSH+/Z4OPJj/+T4H+GyeE+Lfz/zv8DPAW/OcEP9+5n+H3wFem+eE+Pcz/zv8DvDaPCfEv5954V6H/xovDXwVL9jvAK/Nc0L8+5kXTvzXeG3gt3jBfgd4bZ4T4t/PvHDiv8ZrA7/FC/Y7wGvznBDP328Dr8Vz+h3gtXle5oUT/zVeG/gtXrDfAV6b54R4/n4beC2e0+8Ar83zMi+c+K/x2sBv8YL9DvDaPCfE8/fbwGvxnH4HeG2el3nhXpv/Gi8NfDUv2O8Ar81zQjx/vw28Fs/pd4DX5nmZ/x1+B3htnhPi+ftt4LV4Tr8DvDbPy/zv8DvAa/OcEM/fbwOvxXP6HeC1eV7mf4ffAV6b54R4/n4beC2e0+8Ar83zMv87/A7w2jwnxPP328Br8Zx+B3htnpf53+F3gNfmOSGev98GXovn9DvAa/O8zAsn/mu8NvBbvGC/A7w2zwnx/P028Fo8p98BXpvnZV448V/jtYHf4gX7HeC1eU6I5++3gdfiOf0O8No8L/PCif8arw38Fi/Y7wCvzXNCPH+/DbwWz+l3gNfmeZkXTvzXeG3gt3jBfgd4bZ4T4vn7beC1eE6/A7w2z8u8cOK/xmsDv8UL9jvAa/OcEM/fbwOvxXP6HeC1eV7mhftt/mscB16aF+x3gNfmOSGev98GXovn9DvAa/O8zP8OvwO8Ns8J8fz9NvBaPKffAV6b52X+d/gd4LV5Tojn77eB1+I5/Q7w2jwv87/D7wCvzXNCPH+/DbwWz+l3gNfmeZn/HX4HeG2eE+L5+23gtXhOvwO8Ns/LvHC/w7/Pa/Ef43eA1+Y5IZ6/3wZei+f0O8Br87zMCyf+fcx/jN8BXpvnhHj+fht4LZ7T7wCvzfMyL5z49zH/MX4HeG2eE+L5+23gtXhOvwO8Ns/LvHDi38f8x/gd4LV5Tojn77eB1+I5/Q7w2jwv88KJfx/zH+N3gNfmOSGev98GXovn9DvAa/O8zAsn/n3Mf4zfAV6b54R4/n4beC2e0+8Ar83zMv87/A7w2jwnxPP328Br8Zx+B3htnpf53+F3gNfmOSGev98GXovn9DvAa/O8zP8OvwO8Ns8J8fz9NvBaPKffAV6b57ULHON/vs8BPpvnhHj+fht4LZ7T7wCvzfP6buC9+J/vbYCf5jkhnr/fBl6L5/Q7wGvzvB4MPJ3/2f4GeGmeF+L5+23gtXhOvwO8Ns/fewPfxf9MfwO8NXArzwvx/P028Fo8p98BXpsX7MHAZwNvDRzjv98zgO8GvhrY5flDPH+/DbwWz+l3gNfm/xbE8/fbwGvxnH4HeG3+b0E8f78NvBbP6XeA1+b/FsTz99vAa/Gcfgd4bf5vQTx/vw28Fs/pd4DX5v8WxPP308Bb8Zx2gb/mf57X4d8O8fx9NvBZ/O8g/u0Qz99LA3/F/w7i3w7xgn018FH8zyf+7RAv3FcDH8X/bOLfDvEve2ngrYGXBo7zP89r82+H+P8N8f8b4v83xP9viP/f+EelSr9BD7TIAwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLibraryBooks;
impl IconShape for MdLibraryBooks {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-1 9H9V9h10v2zm-4 4H9v-2h6v2zm4-8H9V5h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLibraryMusic;
impl IconShape for MdLibraryMusic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 5h-3v5.5c0 1.38-1.12 2.5-2.5 2.5S10 13.88 10 12.5s1.12-2.5 2.5-2.5c.57 0 1.08.19 1.5.51V5h4v2zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLoop;
impl IconShape for MdLoop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEkklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PV4aeCvgwVxxK/AzwF/zXwvxX+u1ge8CHszzdyvwPsBv818D8V/nvYHv4kXzPsB3858P8V/jvYHv4l/nfYDv5j8X4j/fceDpwHH+dXaBhwC7/OdB/Of7aOCr+Lf5GOCr+c+D+M/328Br8W/zO8Br858H8Z/P/PuI/zyI/3zm30f850H85zP/PuI/D+I/n/n3Ef95EP/5zL+P+M+D+M9n/n3Efx7Efz7z7yP+8yD+85l/H/GfB/Gfz/z7iP88iP985t9H/OdB/Ocz/z7iPw/iP5/59xH/eRD/+cy/j/jPg/jPZ/59xH8exH8+8+8j/vMg/vOZfx/xnwfxn8/8+4j/PIh/m88GXotn2wXehufP/PuI5++ngOM82+8An82/DuLf5rOBz+I5iefvVuBB/Nv8DfDSPH/mOX0O8Nn86yD+bT4b+Cye0wlgl+f108Bb8W/zPcB787yOAxd5Tp8DfDb/Ooh/m7cGforn9DbAT/O83hv4Lv5tXgf4bZ7XWwM/xXN6G+Cn+ddB/Ns8GHg6z+l7gPfm+ftr4KX41/kd4LV5/r4beC+e0wlgl38dxL/dXwMvxbPtAid4/l4a+G3gGC+aS8BLA7fy/F0EjvNsfwO8NP96iH+7jwa+iuf0McBX8/y9NPDTwIN44f4GeG/gr3n+Phr4Kp7TxwBfzb8e4t/uOHArcIxn2wUeAuzy/B0HPhp4b+BBPKdnAN8NfDWwy/N3HHg6cJxnuwQ8GNjlXw/x7/PZwGfxnH4aeBv+ZS8NHOeKXeCv+Zf9FPDWPKfPAT6bfxvEv89x4FbgGM/pu4H34T/WdwHvzXO6BDwY2OXfBvHv99rAb/G8vhv4GGCXf5/jwFcB783zeh3gt/m3Q/zH+Gzgs3hefw28D/DX/Nu8NPBdwEvzvD4H+Gz+fRD/cb4beC+ev+8GPge4lRfNg4HPAt6b5+97gPfm3w/xH+u7gffiBftr4LeBn+aKZ3DFg7jirYHXBl6aF+x7gPfmPwbiP95HA58NHOM/1iXgs4Gv5j8O4j/Hg4HvBl6L/xi/A7w3cCv/sRD/ud4beG/gtfi3+R3gu4Hv5j8H4r/GSwPvDbw28FK8cH8D/Dbw3cBf858L8V/vOPDbwEvxnP4GeG1gl/86iP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IxG0lkFo6DNWAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMic;
impl IconShape for MdMic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 14c1.66 0 2.99-1.34 2.99-3L15 5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3zm5.3-3c0 3-2.54 5.1-5.3 5.1S6.7 14 6.7 11H5c0 3.41 2.72 6.23 6 6.72V21h2v-3.28c3.28-.48 6-3.3 6-6.72h-1.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/PV4aeCvgwVxxK/AzwF/zXwvxX+u1ge8CHszzdyvwPsBv818D8V/nvYHv4kXzPsB3858P8V/jvYHv4l/nfYDv5j8X4j/fceDpwHH+dXaBhwC7/OdB/Of7aOCreE6XgLcGfpsr3hr4buAYz+ljgK/mPw/iP99vA6/Fc3od4Ld5Tq8N/BbP6XeA1+Y/D+I/n3lOl4DjPH+7wDGek/jPg/jPZ57T7wCvzfP328Br8ZzEfx7Efz7znH4HeG2ev98GXovnJP7zIP7zmef0O8Br8/z9NvBaPCfxnwfxn888p98BXpvn77eB1+I5if88iP985jn9DvDaPH+/DbwWz0n850H85zPP6XeA1+b5+23gtXhO4j8P4j+feU6/A7w2z99vA6/FcxL/eRD/+cxz+h3gtXn+fht4LZ6T+M+D+M9nntPvAK/N8/fbwGvxnMR/HsR/PvOcfgd4bZ6/3wZei+ck/vMg/vOZ5/Q7wGvz/P028Fo8J/GfB/Gfzzyn3wFem+fvt4HX4jmJ/zyI/3zmOf0O8No8f78NvBbPSfznQfznM8/pd4DX5vn7beC1eE7iPw/iP595Tr8DvDbP328Dr8VzEv95EP/5zHP6HeC1ef5+G3gtnpP4z4P4t/ls4LV4tl3gbXj+zHP6HeC1ef7+CnhpnpN4/n4KOM6z/Q7w2fzrIP5tPhv4LJ6TeP5uBR7EcxLPn3lOfwO8NM+feU6fA3w2/zqIf5vPBj6L53QC2OV5/TTwVjynzwE+m+f02cBn8Zy+B3hvntdx4CLP6XOAz+ZfB/Fv89bAT/Gc3gb4aZ7XewPfxfP6aeC7ueK9gbfmeb0O8Ns8r7cGforn9DbAT/Ovg/i3eTDwdJ7T9wDvzfP318BL8a/zO8Br8/x9N/BePKcTwC7/Ooh/u78GXopn2wVO8Py9NPDbwDFeNJeAlwZu5fm7CBzn2f4GeGn+9RD/dh8NfBXP6WOAr+b5e2ngp4EH8cL9DfDewF/z/H008FU8p48Bvpp/PcS/3XHgVuAYz7YLPATY5fk7Dnw08N7Ag3hOzwC+G/hqYJfn7zjwdOA4z3YJeDCwy78e4t/ns4HP4jn9NPA2/MteGjjOFbvAX/Mv+yngrXlOnwN8Nv82iH+f48CtwDGe03cD78N/rO8C3pvndAl4MLDLvw3i3++1gd/ieX038DHALv8+x4GvAt6b5/U6wG/zb4f4j/HZwGfxvP4aeB/gr/m3eWngu4CX5nl9DvDZ/Psg/uN8N/BePH/fDXwOcCsvmgcDnwW8N8/f9wDvzb8f4j/WdwPvxQv218BvAz/NFc/gigdxxVsDrw28NC/Y9wDvzX8MxH+8jwY+GzjGf6xLwGcDX81/HMR/jgcD3w28Fv8xfgd4b+BW/mMh/nO9N/DewGvxb/M7wHcD381/DsR/jZcG3ht4beCleOH+Bvht4LuBv+Y/F+K/3nHgt4GX4jn9DfDawC7/dRD/PX4beC2e0+8Ar81/LcR/j98GXovn9DvAa/NfC/Hf47eB1+I5/Q7w2vzXQvz3+G3gtXhOvwO8Nv+1EP89fht4LZ7T7wCvzX8txH+P3wZei+f0O8Br818L8d/jt4HX4jn9DvDa/NdC/Pf4beC1eE6/A7w2/7UQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EfD8sRBoTzIYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMicNone;
impl IconShape for MdMicNone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 14c1.66 0 2.99-1.34 2.99-3L15 5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3zm-1.2-9.1c0-.66.54-1.2 1.2-1.2.66 0 1.2.54 1.2 1.2l-.01 6.2c0 .66-.53 1.2-1.19 1.2-.66 0-1.2-.54-1.2-1.2V4.9zm6.5 6.1c0 3-2.54 5.1-5.3 5.1S6.7 14 6.7 11H5c0 3.41 2.72 6.23 6 6.72V21h2v-3.28c3.28-.48 6-3.3 6-6.72h-1.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMicOff;
impl IconShape for MdMicOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 11h-1.7c0 .74-.16 1.43-.43 2.05l1.23 1.23c.56-.98.9-2.09.9-3.28zm-4.02.17c0-.06.02-.11.02-.17V5c0-1.66-1.34-3-3-3S9 3.34 9 5v.18l5.98 5.99zM4.27 3L3 4.27l6.01 6.01V11c0 1.66 1.33 3 2.99 3 .22 0 .44-.03.65-.08l1.66 1.66c-.71.33-1.5.52-2.31.52-2.76 0-5.3-2.1-5.3-5.1H5c0 3.41 2.72 6.23 6 6.72V21h2v-3.28c.91-.13 1.77-.45 2.54-.9L19.73 21 21 19.73 4.27 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JcdB74KeG/++/018D7AX/MfA/Ev+27gvfifYxd4CLDLv+ylgb/mBUP8y8z/PK8D/DYv3EsDvwWc4AVD/MvM/zyvA/w2L9hLA78FHAfEC4b4l5n/eV4H+G2ev5cGfgs4zhXiBUP8y8z/PK8D/DbP66WB3wKO82ziBUP8y8z/PK8D/DbP6aWB3wKO85zEC4b4l5n/eV4H+G2e7aWB3wKO87zEC4b4l5nn9T3Arbzo3ht4EC+6vwEeDBzj+Xsd4Le54qWB3wKO8/yJFwzxLzPP63WA3+ZF99vAa/Gi+RvgtYEHA78NHON5vQ7w28BLA78FHOcFEy8Y4l9mntfrAL/Ni+63gdfiX/Y3wGsDu1zx2sBv8bxeB9gFfgs4zgsnXjDEv8w8r9cBfpvn9NLAW/H8vTfwYF64vwFeG9jliuPAbwEvzfP6GOCzgOP8y8QLhviXmef1OsBv87y+G3gv/vX+BnhtYJcrjgO/Bbw0/37iBUP8y8zzeh3gt7niOLDLs3038F686P4GeG1glyuOA78FvDT/McQLhviXmef1OsBvc8VPA58N/DXP9t3Ae/Ev+xvgtYFdrjgO/Bbw0vzHES8Y4l9mntfrAL/NFb8NvBTwOsBf82zfDbwXL9jfAK8N7HLFceC3gJfmP5Z4wRD/MvO8Xgf4ba74beC1gF3gdYC/5tm+G3gvntffAK8N7HLFceC3gJfmP554wRD/MvO8Xgf4ba74beC1uGIXeB3gr3m27wbei2f7G+C1gV2uOA78FvDS/OcQLxjiX2ae1+sAv80Vvw28Fs+2C7wO8Nc823cD7wX8DfDawC5XHAd+C3hp/vOIFwzxLzPP63WA3+aK3wZei+e0C7wO8Nc822cDXw3scsVx4LeAl+Y/l3jBEP8y87y+G7iVK94beDDPaxd4HeCveV7Hgd8CXpp/m48BPhs4xr9MvGCIf5n5t9sFXgf4a57tOPBbwEvzb/c6wC7w28AxXjjxgiH+ZebfZxd4HeCvgePAbwEvzb/P6wC/Dbw08NvAMV4w8YIh/mXm328XeBvgq4CX5t/vdYDf5oqXBn4bOMbzJ14wxL/M/M/zOsBv82wvDfw2cIznJV4wxL/M/M/zOsBv85xeGvht4BjPSbxgiH+Z+Z/ndYDf5nm9NPDbwDGeTbxgiH+Z+Z/ndYDf5vl7aeC3gWNcIV4wxL/M/M/zOsBv84K9NPDbwDFAvGCIf5n5n+d1gN/mhXtp4LeB47xgiH/ZdwPvxf8cl4AHA7v8y14a+GteMMS/7Djw1cB78d/vb4D3Bv6a/xiI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjs0imQa8HmU0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMissedVideoCall;
impl IconShape for MdMissedVideoCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4zM10 15l-3.89-3.89v2.55H5V9.22h4.44v1.11H6.89l3.11 3.1 4.22-4.22.78.79-5 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE6UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDGe7RnArbxgDwYexHO6BPw1L9hx4KV4Xr/DC/davGguAX/Nc0K8cK8NfBfwYJ7TbwOvwwv22sBv8bweAtzKC3Yr8CCe0/sA380L9t3Ae/Ev+x3gtXlOiBfsrYGf4gV7HeC3ecF+G3gtntP3AO/NC/bewHfxnG4FHsIL9mDg6fzLfgd4bZ4T4vk7DjwdOM4L9tvA6/CCvTbwWzyvhwC38oLdCjyI5/Q+wHfzgn038F68cL8DvDbPCfH8vTfwXfzLXgf4bV6w3wZei+f0PcB784K9N/BdPKdbgYfwgj0YeDov3O8Ar81zQjx/Xw18FP+y3wZehxfstYHf4nk9BLiVF+xW4EE8p/cBvpsX7LuB9+IF+x3gtXlOiOfvt4HX4kXzOsBv84L9NvBaPKfvAd6bF+y9ge/iOd0KPIQX7MHA03nBfgd4bZ4T4vn7beC1eNH8NvA6vGCvDfwWz+shwK28YLcCD+I5vQ/w3bxg3w28F8/f7wCvzXNCPH+/DbwWL7rXAX6bF+y3gdfiOX0P8N68YO8NfBfP6VbgIbxgDwaezvP3O8Br85wQz99vA6/Fi+63gdfhBXtt4Ld4Xg8BbuUFuxV4EM/pfYDv5gX7buC9eF6/A7w2zwnx/P028Fr867wO8Nu8YL8NvBbP6XuA9+YFe2/gu3hOtwIP4QV7MPB0ntfvAK/Nc0I8f78NvBb/OrcCt/KCPRh4MM/rt3nhXpvn9dfALi/YSwPHeU6/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8fTfwXvzf8jXAR/OcEM/fRwNfxf8tHwN8Nc8J8fwdB24FjvF/wyXgwcAuzwnxgr038F383/A2wE/zvBAv3GsD3w08iP+dngG8N/DbPH+IF81LA8f532UX+GteOMT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzO2oEEI6DBaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMovie;
impl IconShape for MdMovie {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 4l2 4h-3l-2-4h-2l2 4h-3l-2-4H8l2 4H7L5 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7nMv97rAL/Ni8Y8r9cBfhtAPNtrA7/F8xL/ucy/3usAv82Lxjyv1wF+G0A822sDv8XzEv+5zL/e6wC/zYvGPK/XAX4bQDzbawO/xfMS/7nMv97rAL/Ni8Y8r9cBfhtAPNtrA7/F8xL/ucy/3usAv82Lxjyv1wF+G0A822sDv8XzEv+5zPP6GOCvecH+GtjlRWOe1+sAvw0gnu21gd/ieYn/XOZ5vQ7w2/zHMM/rdYDfBhDP9trAb/G8xH8u87xeB/ht/mOY5/U6wG8DiGd7beC3eF7iP5d5Xq8D/Db/Mczzeh3gtwHEs7028Fs8L/Gfyzyv1wF+m/8Y5nm9DvDbAOLZXhv4LZ6X+M9lntfrAL/NfwzzvF4H+G0A8WyvDfwWz0v85zLP63WA3+Y/hnlerwP8NoB4ttcGfovnJf5zmef1OsBv8x/DPK/XAX4bQDzbawO/xfMS/7nM83od4Lf5j2Ge1+sAvw0gnu21gd/ieYn/XOZ5vQ7w2/zHMM/rdYDfBhDP9trAb/G8xH8u87xeB/ht/mOY5/U6wG8DiGd7beC3eF7i3+848F7AWwOvzRW7wG8Db83zeh3gt/mPYZ7X6wC/DSCe7bWB3+J5iX+f9wa+CjjOi+51gN/mP4Z5Xq8D/DaAeLbXBn6L5yX+7d4b+C7+9V4H+G3+Y5jn9TrAbwOIZ3tt4Ld4XuLf5rWB3+Lf5nWA3+Y/hnlerwP8NoB4ttcGfovnJf5tng48mH+b1wF+m/8Y5nm9DvDbAOLZXhv4LZ6X+Nd7aeCv+Ld7HeC3+Y9hntfrAL8NIJ7ttYHf4nmJf73PBj6Lf7vXAX6b/xjmeb0O8NsA4tleG/gtnpf41/tu4L34t3sd4Lf5j2Ge1+sAvw0gnu21gd/ieYl/ve8G3ot/u9cBfpv/GOZ5vQ7w2wDi2V4b+C2el/jX+2zgs/i3exngr/mPYZ7X6wC/DSCe7bWB3+J5iX+9lwb+in+bZwAP5j+OeV6vA/w2gHi21wZ+i+cl/m1uBR7Ev97nAJ/NfxzzvF4H+G0A8WyvDfwWz0v827w28Fv86zwDeGlgl/845nm9DvDbAOLZXhv4LZ6X+Ld7b+C7eNFcAl4b+Gv+Y5nn9TrAbwOIZ3tt4Ld4XuLf572BrwaO8YL9DvDewK38xzPP63WA3wYQz/bawG/xvMS/33HgvYG3Bl6LK54B/Dbw28B385/HPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz+u1+d/tt3lerwP8NoB4ttcGfov/H14H+G0A8Zx2gWP833YJOM4ViOf02cBn8X/b5wCfzRWI5/XdwHvxf9P3AO/NsyGev/cGPhp4Kf5v+B3gu4Hv5jkh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/+jNFBIAs5PAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMusicVideo;
impl IconShape for MdMusicVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14zM8 15c0-1.66 1.34-3 3-3 .35 0 .69.07 1 .18V6h5v2h-3v7.03c-.02 1.64-1.35 2.97-3 2.97-1.66 0-3-1.34-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNewReleases;
impl IconShape for MdNewReleases {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 12l-2.44-2.78.34-3.68-3.61-.82-1.89-3.18L12 3 8.6 1.54 6.71 4.72l-3.61.81.34 3.68L1 12l2.44 2.78-.34 3.69 3.61.82 1.89 3.18L12 21l3.4 1.46 1.89-3.18 3.61-.82-.34-3.68L23 12zm-10 5h-2v-2h2v2zm0-4h-2V7h2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNotInterested;
impl IconShape for MdNotInterested {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8 0-1.85.63-3.55 1.69-4.9L16.9 18.31C15.55 19.37 13.85 20 12 20zm6.31-3.1L7.1 5.69C8.45 4.63 10.15 4 12 4c4.42 0 8 3.58 8 8 0 1.85-.63 3.55-1.69 4.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD/UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH++/0O/7EQL9xrA98FPJj/Gb4beB/+4yBesPcGvov/eb4beB/+YyCev+PA04Hj/M/03cD78O+HeP4+Gvgq/me4BBzjeX038D78+yCev+8G3ov/GV4G+G3gGM/ru4H34d8O8fz9NvBa/M8g4KWB3waO8by+G3gf/m0Qz99vA6/F/wziipcGfhs4xvP6buB9+NdDPH+/DbwW/zOIZ3tp4LeBYzyv7wbeh38dxPP328Br8T+DeE4vDfw2cIzn9d3A+/CiQzx/vw28Fv8ziOf10sBvA8d4Xt8NvA8vGsTz99vAa/E/g3j+Xhr4beAYz+u7gffhX4Z4/n4beC3+ZxAv2EsDvw0c43l9N/A+vHCI5++3gdfifwbxwr008NvAMZ7XdwPvwwuGeP5+G3gt/mcQ/7KXBn4bOMbz+m7gfXj+EM/fbwOvxf8M4kXz0sBvA8d4Xt8NvA/PC/H8/TbwWvzPIF50Lw38NnCM5/U1wEfznBDP328Dr8X/DOJf56WB3waO8Zx+B3htnhPi+ftt4LX4n0H867008NvAMZ7td4DX5jkhnr/fBl6L/xl+m3+bBwMP5tl+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftu4L34v+VrgI/mOSGev48Gvor/Wz4G+GqeE+L5Ow7cChzj/4ZLwIOBXZ4T4gV7b+C7+L/hbYCf5nkhXri3Br4aeBD/Oz0DeG/gt3n+EC+alwaO87/LLvDXvHCI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+x72lBMIGPjQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNote;
impl IconShape for MdNote {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 10l-6-6H4c-1.1 0-2 .9-2 2v12.01c0 1.1.9 1.99 2 1.99l16-.01c1.1 0 2-.89 2-1.99v-8zm-7-4.5l5.5 5.5H15V5.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD5UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/36/xYvmb4CP5kXz1cBL8aJ5Hf7tEP9+5kXzO8Br86L5beC1eNGIfzvEv5950fwO8Nq8aH4beC1eNOLfDvHvZ140vwO8Ni+a3wZeixeN+LdD/PuZF83vAK/Ni+a3gdfiRSP+7RD/fuZF8zvAa/Oi+W3gtXjRiH87xL+fedH8DvDavGh+G3gtXjTi3w7x72deNL8DvDYvmt8GXosXjfi3Q/z7mRfN7wCvzYvmt4HX4kUj/u0Q/37mRfM7wGvzovlt4LV40Yh/O8S/n3nR/A7w2rxofht4LV404t8O8e9nXjS/A7w2L5rfBl6LF434t0P8+5kXze8Ar82L5reB1+JFI/7tEP9+5kXzO8Br86L5beC1eNGIfzvEv5950fwO8Nq8aH4beC1eNOLfDvHvZ140vwO8Ni+a3wZeixeN+LdD/PuZF83vAK/Ni+a3gdfiRSP+7RD/fuZF8zvAa/Oi+W3gtXjRiH87xL+fedH8DvDavGh+G3gtXjTi3w7x72deNL8DvDYvmt8GXosXjfi3Q/z7mRfN7wCvzYvmt4HX4kUj/u0Q/37mRfM7wGvzovlt4LV40Yh/O8S/n3nR/A7w2rxofht4LV404t8O8e9nXjS/A7w2L5rfBl6LF434t0P8+5kXze8Ar82L5reB1+JFI/7tEP9+5kXzO8Br86L5beC1eNGIfzvEv5950fwO8Nq8aH4beC1eNOLfDvHvZ140vwO8Ni+a3wZeixeN+LdD/PuZF83vAK/Ni+a3gdfiRSP+7RD/fuZF8zvAa/Oi+W3gtXjRiH87xL+fedH8DvDavGh+G3gtXjTi3w7x72deNL8DvDYvmt8GXosXjfi3Q/z7mRfN7wCvzYvmt4HX4kUj/u0Q/37mRfM7wGvzovlt4LV40Yh/O8S/n3nR/A7w2rxofht4LV404t8O8e9nXjS/A7w2L5rfBl6LF434t0P8+5kXze8Ar82L5reB1+JFI/7tEP9+v82L5q+Bj+ZF89XAS/OieW3+7RD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wiGYXJBIfubcAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPause;
impl IconShape for MdPause {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 19h4V5H6v14zm8-14v14h4V5h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPauseCircleFilled;
impl IconShape for MdPauseCircleFilled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14H9V8h2v8zm4 0h-2V8h2v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH1UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/zp/A+zynI4DL8W/zk8D7wPs8m+D+Ld7aeCngAfzL/sZ4LeBnwZu5YV7MPDWwGsDb8W/7FbgbYC/5l8P8W/z0sBvAcd5wZ4BfDXw3cAu/zbHgY8G3ht4EC/YLvA6wF/zr4P413tp4LeA4zx/l4DPBr6a/1ifDXw0cIznbxd4HeCvedEh/nVeGvgt4DjP3+8A7w3cyn+OBwPfDbwWz98u8DrAX/OiQbzojgN/BTyY5+9zgM/mv8ZnA5/F83cr8DLALv8yxIvup4C35vl7H+C7+a/13sB38fz9NPA2/MsQL5r3Br6L5+99gO/mv8d7A9/F8/c+wHfzwiH+ZceBpwPHeV6fA3w2/70+G/gsntcu8BBglxcM8S/7bOCzeF6/A7w2/zP8NvBaPK/PAT6bFwzxwh0Hng4c5zldAl4auJX/GR4M/DVwjOe0CzwE2OX5Q7xwnw18Fs/rY4Cv5kXzWzyvjwH+muf00sBX8bxehxfNZwOfxfP6HOCzef4QL9zTgQfznJ4BPJgXnXlerwP8Ns/ptYHf4nmJF92twIN4TrcCD+H5Q7xgbw38FM/rY4Cv5kVnntfrAL/Nc3pt4Ld4XuJF99nAZ/G8Xgf4bZ4X4gX7auCjeF4ngF1edOZ5vQ7w2zyn1wZ+i+clXnTHgYs8r68BPprnhXjB/gp4aZ7TzwBvzb+OeV6vA/w2z+m1gd/ieYl/nZ8G3orn9NfAy/C8EM/fg4Gn87zeB/hu/nXM83od4Ld5Tq8N/BbPS/zrfDTwVTyvE8Auzwnx/L028Fs8r4cAt/KvY57X6wC/zXN6beC3eF7iX+elgb/ieb0O8Ns8J8Tz99HAV/G8xL+eeV6vA/w2z+m1gd/ieYl/PfO8Pgb4ap4T4vn7bOCzeE5/A7w0/3rmeb0O8Ns8p9cGfovnJf71/hp4KZ7T5wCfzXNCPH+fDXwWz+l3gNfmX888r9cBfpvn9NrAb/G8xL/ebwOvxXP6HOCzeU6I5++3gdfiOf0O8Nr865nn9TrAb/OcXhv4LZ6X+Nf7beC1eE4/A7w1zwnx/P028Fo8p98BXpt/PfO8Xgf4bZ7TawO/xfMS/3q/DbwWz+lngLfmOSGev88GPovn9DvAa/OvZ57X6wC/zXN6beC3eF7iX++3gdfiOX0O8Nk8J8Tz99nAZ/Gc/hp4Gf71zPN6HeC3eU6vDfwWz0v86/0V8NI8p88BPpvnhHj+Phr4Kp6X+Nczz+t1gN/mOb028Fs8L/GvZ57XxwBfzXNCPH+vDfwWz+shwK3865jn9TrAb/OcXhv4LZ6X+Nd5aeCveF6vA/w2zwnx/D0YeDrP62OAr+Zfxzyv1wF+m+f02sBv8bzEv85HA1/F8zoB7PKcEC/YXwMvxXP6GeCt+dcxz+t1gN/mOb028Fs8L/Gv89PAW/Gc/gZ4aZ4X4gX7auCjeF4ngF1edOZ5vQ7w2zyn1wZ+i+clXnTHgYs8r68BPprnhXjBXhv4LZ7XxwBfzYvOPK/XAX6b5/TawG/xvMSL7rOBz+J5vQ7w2zwvxAt3K/AgntOtwEN40Znn9TrAb/OcXhv4LZ6XeNE9HXgwz+kZwIN5/hAv3GcDn8Xz+hzgs3nR/DbP66OBv+Y5vTTw1Tyv1+ZF89nAZ/G8Pgf4bJ4/xAt3HLgVOMZz2gVeBriV/xkeDPwVcJzndAl4MLDL84f4l3028Fk8r98GXof/GX4LeG2e1+cAn80LhviXHQduBY7xvD4H+Gz+e3028Fk8r0vAg4FdXjDEi+a9ge/i+Xsf4Lv57/HewHfx/L0N8NO8cIgX3U8Db8Xz9z7Ad/Nf672B7+L5+xngrfmXIV50x4G/Bh7E8/fZwOfwX+OzgM/m+XsG8NLALv8yxL/OSwO/DRzj+ftt4H2AW/nP8WDgu4DX5vm7BLw28Ne8aBD/ei8N/DZwjOdvF/hq4HP4j/VZwEcDx3n+LgGvDfw1LzrEv81LA78NHOMFuxX4buBrgF3+bY4DHwW8N/BgXrBLwGsDf82/DuLf7qWBnwYexL/sp4HfBn4GuJUX7qWB1wJeG3hr/mXPAN4a+Gv+9RD/PseB7wbein+dvwZ2eU7HgZfmX+dngPcGdvm3QfzHeG/gq4Fj/Ne4BLw38NP8+yD+4xwHPhr4aOAY/zkuAV8NfDWwy78f4j/eceCjgfcGHsR/jGcA3w18NbDLfxzEf67XBt4aeG3gpfjX+Rvgt4GfBn6b/xyI/zrHgZcGXho4zhUvzRV/zRW7wF8Dfw3s8p8P8f8b4v83xP9viP/fEP+/8Y+Y9zZQF6BRZAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPauseCircleOutline;
impl IconShape for MdPauseCircleOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 16h2V8H9v8zm3-14C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm1-4h2V8h-2v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/zneG/hpYJf/2RD/OX4beBDwPsBv8z8X4j/HbwOvxRU/DbwPsMv/PIj/HL8NvBbPtgt8NPA9/M+C+M/x28Br8bx+G3gf4Fb+Z0D85/ht4LV4/naBrwY+h/9+iP8cvw28Fi/cXwPvA/w1/30Q/zl+G3gtXjRfDXwOsMt/PcR/jt8GXosX3a3A+wC/zX8txH+O3wZei3+9nwbeB9jlvwbiP8dvA6/Fv80u8NHA9/CfD/Gf47eB1+Lf57eB9wFu5T8P4j/HbwOvxb/fLvDVwOfwnwPxn+O3gdfiP85fA+8D/DX/sRD/OX4beC3+43018DnALv8xEP85fht4Lf5z3Aq8D/Db/Psh/nP8NvBa/Of6aeB9gF3+7RD/OX4beC3+8+0CHw18D/82iP8cvw28Fv91fht4H+BW/nUQ/zl+G3gt/uv8DvDewK386yD+c/w28Fr857sEfDTw3fzbIP5z/DbwWvzn+hngvYFd/u0Q/zl+G3gt/nM8A3hv4Lf590P85/ht4LX4j/c1wGcDu/zHQPzn+G3gtfiP8zfAewN/zX8sxH+O3wZei3+/S8BXA5/Nfw7Ef47fBl6Lf5/fAd4buJX/PIj/HL8NvBb/NpeAjwa+m/98iP8cvw28Fv96PwO8N7DLfw3Ef47fBl6LF90zgPcGfpv/Woj/HL8NvBYvmq8BPhvY5b8e4j/HbwOvxQv3N8B7A3/Nfx/Ef47fBl6L5+8S8NXAZ/PfD/Gf47eB1+J5/Q7w3sCt/M+A+M/x28Br8WyXgI8Gvpv/WRD/OX4beC2u+BngvYFd/udB/Of4beDBwHsDv83/XIj/HO8N/DSwy/9siP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BLj2bkFPLZueAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlayArrow;
impl IconShape for MdPlayArrow {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5v14l11-7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM6/398Afw28F/91doGHALs8f4gX7rOBz+I/xu8Arw28N/DVwDH+a3wO8Nk8f4gX7unAg/mP8TvAa3PFSwPfDbwU//luBR7C84d4wd4a+Cn+4/wO8No823Hgq4H34j/f6wC/zfNCvGBfDXwU/3F+B3htntd7A18NHOM/z9cAH83zQrxgfwW8NP9xfgd4bZ6/lwa+G3gp/nP8NfAyPC/E8/dg4On8x/od4LV5wY4DXw28F/85TgC7PCfE8/fawG/xH+t3gNfmX/bewFcDx/iP9TrAb/OcEM/fRwNfxX+s3wFemxfNSwPfDbwU/3E+BvhqnhPi+fts4LP4j/U7wGvzojsOfDXwXvzH+Bzgs3lOiOfvs4HP4j/W7wCvzb/eewNfDRzj3+dzgM/mOSGev98GXov/WL8DvDb/Ni8NfDfwUvzb/Qzw1jwnxPP328Br8R/rd4DX5t/mpYHvAl6af7ufAd6a54R4/j4b+Cz+Y/0O8Nr867038FXAcf59Pgf4bJ4T4vn7bOCz+I/1O8Br86I7DnwV8N78x/gc4LN5Tojn76OBr+I/1u8Ar82L5qWB7wJemv84HwN8Nc8J8fy9NvBb/Mf6HeC1+Ze9N/BVwHH+Y70O8Ns8J8Tz92Dg6fzH+h3gtXnBjgNfBbw3/zlOALs8J8QL9tfAS/Ef53eA1+b5e2ngu4CX5j/H3wAvzfNCvGBfDXwU/3F+B3htntd7A18FHOc/z9cAH83zQrxgrw38Fv9xfgd4bZ7tOPBVwHvzn+91gN/meSFeuFuBB/Ef43eA1+aKlwa+C3hp/vM9A3gwzx/ihfts4LP4j/E7wGsD7w18FXCc/xqfA3w2zx/ihTsO3Aoc49/vb4C/At6b/zqXgAcDuzx/iH/ZZwOfxf9OnwN8Ni8Y4l92HLgVOMb/LpeABwO7vGCIF817A9/F/y5vA/w0LxziRffTwFvxv8PPAG/NvwzxojsO/DXwIP5newbw0sAu/zLEv85LA78NHON/pkvAawN/zYsG8a/30sBvA8f4n+US8NrAX/OiQ/zbvDTw28Ax/me4BLw28Nf86yD+7V4a+GngQfz3egbw1sBf86+H+Pc5Dnw38Fb89/gZ4L2BXf5tEP8x3hv4auAY/zUuAe8N/DT/Poj/OMeBjwY+GjjGf45LwFcDXw3s8u+H+I93HPho4L2BB/Ef4xnAdwNfDezyHwfxn+u1gbcGXht4Kf51/gb4beCngd/mPwfiv85x4KWBlwaOc8VLc8Vfc8Uu8NfAXwO7/OdD/P+G+P8N8f8b4v83xP9v/COLWMBBBCjgCAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlayCircleFilled;
impl IconShape for MdPlayCircleFilled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlayCircleOutline;
impl IconShape for MdPlayCircleOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 16.5l6-4.5-6-4.5v9zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX+c4sMv/HYgX3XsDXwW8DvDX/N+AeNG8N/BdXLELvA7w1/zvh/iXvTfwXTynXeB1gL/m+Xtv4KeBXf5nQ/zLvht4L57XLvA6wF/zvF4a+G7go4Hf5n8uxIvmu4H34nntAq8D/DXP66WB3wZ+C3gfYJf/eRAvuu8G3ovntQu8DvDXPK+XBn4bMPDRwPfwPwviX+e7gffiee0CrwP8Nc/rpYHfBo4Bvw28D3Ar/zMg/vW+G3gvntcu8DrAX/O8Xhr4beAYsAt8NfA5/PdD/Nt8N/BePK9d4HWAv+Z5vTTw28Axrvhr4H2Av+a/D+Lf7ruB9+J57QKvA/w1z+vBwE8DL8WzfTXwOcAu//UQ/z7fDbwXz2sXeB3gr3lex4HfBl6KZ7sVeB/gt/mvhfj3+27gvXheu8DrAH/N8zoO/DbwUjynnwbeB9jlvwbiP8Z3A+/F89oFXgf4a57XceC3gZfiOe0CHw18D//5EP9xvht4L57XLvA6wF/zvI4Dvw28FM/rt4H3AW7lPw/iP9Z3A+/F89oFXgf4a57XceC3gZfiee0CXw18Dv85EP/xvht4L57XLvA6wF/zvI4Dvw28FM/fXwPvA/w1/7EQ/zn+Gngpntcu8DrAX/O8jgO/DbwUL9hXA58D7PIfA/Gf47eB1+L52wVeB/hrntdx4LeBl+IFuxV4H+C3+fdD/Of4beC1eMF2gdcB/prndRz4beCleOF+GngfYJd/O8R/jt8GXosXbhd4HeCveV7Hgd8GXooXbhf4aOB7+LdB/Of4beC1+JftAq8D/DXP6zjw28BL8S/7beB9gFv510H85/ht4LV40ewCrwP8Nc/rOPDbwEvxwv0O8N7ArfzrIP5z/DbwWrzodoHXAf6a53Uc+G3gpXhel4CPBr6bfxvEf47fBl6Lf51d4HWAv+Z5HQd+G3gpnu1ngPcGdvm3Q/zn+G3gtfjX2wVeB/hrntdx4LeB48BHAz/Nvx/iP8dvA6/Fv80u8DrAX/O8jnPFLv8xEP85fht4Lf7tdoHXAf6a/1yI/xy/DbwW/z67wOsAf81/HsR/jt8GXot/v13gdYC/5j8H4j/HbwOvxX+MXeB1gL/mPx7iP8dvA6/Fv97fAC/F89oFXgf4a/5jIf5z/DbwWrzongG8N/DbwHcD78Xz2gVeB/hr/uMg/nP8NvBavGg+B/hqYJdn+27gvXheu8DrAH/NfwzEf47fBl6LF+5vgPcG/prn77uB9+J57QKvA/w1/36I/xy/DbwWz98l4LOBr+Zf9t3Ae/G8doHXAf6afx/Ef47fBl6L5/U7wHsDt/Ki+27gvXheu8DrAH/Nvx3iP8dvA6/Fs10CPhr4bv5tvht4L57XLvA6wF/zb4P4z/HbwGtxxc8A7w3s8u/z3cB78bx2gdcB/pp/PcR/jt8GHgy8N/Db/Mf5buC9eF67wOsAf82/DuI/x3sDPw3s8h/vu4H34nntAq8D/DUvOsT/Tt8NvBfPaxd4HeCvedEg/vf6buC9eF67wEOAXf5liP/dvht4L57T+wDfzYsG8b/fdwPvxRXvA3w3LzrE/w3fDfw28N386yD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj4iL4UGdGOIcAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlayDisabled;
impl IconShape for MdPlayDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5.19V5l11 7-2.55 1.63L8 5.19zm12 14.54l-5.11-5.11L8 7.73 4.27 4 3 5.27l5 5V19l5.33-3.4 5.4 5.4L20 19.73z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP5+G3gt/vu8DvDb/OdDPH+/DbwW/31eB/ht/vMhnr/fBl6L/z6vA/w2//kQz99vA6/Ff5/XAX6b/3yI5++3gdfiv8/rAL/Nfz7E8/fSwHH++/w1sMt/PsT/b4j/3xD/vyH+f0M8fy8NHOO/z98Auzyn48BL8bx+h387xPP328Br8d/ndYDf5jm9NvBbPC/xb4d4/n4beC3++7wO8Ns8p9cGfovnJf7tEM/fbwOvxX+f1wF+m+f02sBv8bzEvx3i+ftt4LX47/M6wG/znF4b+C2el/i3Qzx/vw28Fv99Xgf4bZ7TawO/xfMS/3aI/z1eG/gtnpf4t0P87/HawG/xvMS/HeJ/j9cGfovnJf7tEP97vDbwWzwv8W+H+N/jtYHf4nmJfzvE8/fVwEvxH+tvgI/mef0WL5rjwEvzvH6bF83fAB/Nc0I8f78NvBb/sX4HeG2el/mv8TvAa/OcEM/fbwOvxX+s3wFem+dl/mv8DvDaPCfE8/fbwGvxH+t3gNfmeZn/Gr8DvDbPCfH8/TbwWvzH+h3gtXle5r/G7wCvzXNCPH9fDbw0/7H+Gvhontdv86I5DrwUz+t3eNH8NfDRPCfE/x6vDfwWz0v82yH+93ht4Ld4XuLfDvG/x2sDv8XzEv92iP89Xhv4LZ6X+LdD/O/x2sBv8bzEvx3if4/XBn6L5yX+7RD/e7w28Fs8L/Fvh/jf47WB3+J5iX87xP8erw38Fs9L/Nsh/vd4beC3eF7i3w7xv8dx4KV5Xr/Nvx3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CM83UhBfAL4vwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlaylistAdd;
impl IconShape for MdPlaylistAdd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 10H2v2h12v-2zm0-4H2v2h12V6zm4 8v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zM2 16h8v-2H2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADuklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP5+G3gt/vu8DvDb/OdDPH+/DbwW/31eB/ht/vMhnr/fBl6L/z6vA/w2//kQz99vA6/Ff5/XAX6b/3yI5++3gdfiv8/rAL/Nfz7E8/fSwHH++/w1sMt/PsT/b4j/3xD/vyH+f0M8fy8NHOO/z98Au/znQzx/vw28Fv99Xgf4bf7zIZ6/3wZei/8+rwP8Nv/5EM/fbwOvxX+f1wF+m/98iOfvt4HX4r/P6wC/zX+cBwO38rwQz99vA6/Ff5/XAX6b/xjfBbw18DrAX/OcEP+3fRfw3lyxC7wO8Nc8G+L/rvcGvovntAu8DvDXXIH4v+m9ge/i+dsFHgLsAoj/e94b+C5esPcBvpsrEP+3vDfwXbxg7wN8N8+GeP6+Gngp/mP9DfDRvGCfBXwNsMu/zXsD38UL9jnAZ/OcEM/fbwOvxX+s3wFem+fvu4D3Bv4aeB1gl3+d9wa+ixfse4D35nkhnr/fBl6L/1i/A7w2z+u7gPfm2f4aeB1glxfNewPfxQv2PcB78/whnr/fBl6L/1i/A7w2z+m7gPfmef018DrALi/cewPfxQv2PcB784Ihnr/fBl6L/1i/A7w2z/Zg4K+BYzx/fw28DrDL8/fewHfxgn0P8N68cIjn76uBl+Y/1l8DH81zemngt4FjPH9/DbwOsMtzem/gu3jBvgd4b/5liP9+Lw38NnCM5++vgdcBdrnivYHv4gX7HuC9edEg/md4aeC3gWM8f38NvA7w1sB38YJ9D/DevOgQ/3O8NPDbwDGev1uBB/OCfQ/w3vzrIP5neWngt4Fj/Ot8D/De/Osh/ud5aeC3gWO8aL4HeG/+bRD/M7008NvAMV647wHem387xP9cLw38NnCM5+97gPfm3wfxP9tLA78NHOM5fQ/w3vz7If7ne2ngt4FjXPE9wHvzHwPxv8NLA78N/DTw3vzHQfzv8WDgVv5jIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AjYXbEFYalntAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlaylistAddCheck;
impl IconShape for MdPlaylistAddCheck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14,10H2v2h12V10z M14,6H2v2h12V6z M2,16h8v-2H2V16z M21.5,11.5L23,13l-6.99,7l-4.51-4.5L13,14l3.01,3L21.5,11.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC+0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeNG9NvBb/Pf5HeC1+Y+FeNG9NvBb/Pf5HeC1+Y+FeNG9NvBb/Pf5HeC1+Y+FeNG9NvBb/Pf5HeC1+Y+FeNG9NvBb/Pf5HeC1+Y+FeNEdB16a/z67wF/zHwvx/xvi/zfE/2+I/98QL7rjwEvx3+cS8Nf8x0K86F4b+C3++/wO8Nr8x0K86F4b+C3++/wO8Nr8x0K86F4b+C3++/wO8Nr8x0K86F4b+C3++/wO8Nr8x0K86F4b+C3++/wO8Nr8x0L8/4b4/w3x/xvi/zfE/2+IF91LA1/Ff7zX4Xm9N/DTwC7/uRAvutcGfov/eOJ5/TbwUsBHA9/Dfx7Ei+61gd/iP554Xr8NvBZX/DbwMcBf8x8P8aJ7beC3+I8nntdvA6/Fc/pq4HOAXf7jIF50rw38Fv/xxPP6beC1eF67wEcD38N/DMSL7qWBr+Y/3mvzvH4beC1esN8GPgb4a/59EP8z/TbwWvzLvhr4HGCXfxvE/0y/DbwWL5pd4KOB7+FfD/E/028Dr8WL7hLw1sBv86+D+J/pt4HX4kXzPcBHA7v86yH+Z/pt4LV44f4G+Gjgt/m3Q/zP9NvAa/H8XQI+G/hq/v0Q/zP9NvBaPK/vAT4a2OU/BuJ/pt8GXotn+xvgo4Hf5j8W4n+m3wZeC7gEfDbw1fznQPzP9NvArcBHA7v850H8z/Rg4Fb+8yH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLky01BLk4gtwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPlaylistPlay;
impl IconShape for MdPlaylistPlay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 10h12v2H4zm0-4h12v2H4zm0 8h8v2H4zm10 0v6l5-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQueue;
impl IconShape for MdQueue {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-1 9h-4v4h-2v-4H9V9h4V5h2v4h4v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADrElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JeZ/17ief028Fr86/wO8No8J8S/zPz3Es/rt4HX4l/nd4DX5jkh/mXmv5d4Xr8NvBb/Or8DvDbPCfEvM/+9xPP6beC1+Nf5HeC1eU6If5n57yWe128Dr8W/zu8Ar81zQvzLXpv/Xr/N8/pt4LV4Tn8DfDQv2C7w1zwnxP9Ovw28Fs/pd4DX5l8H8b/TbwOvxXP6HeC1+ddB/O/028Br8Zx+B3ht/nUQ/zv9NvBaPKffAV6bfx3Ev+y1+O/1Ozyv3wZei+f0O8Br86+D+JeZ/17ief028Fo8p98BXpt/HcS/zPz3Es/rt4HX4jn9DvDa/Osg/mXmv5d4Xr8NvBbP6XeA1+ZfB/EvM/+9xPP6beC1eE6/A7w2/zqIf5n57yWe128Dr8Vz+h3gtfnXQfzv9NvAa/Gcfgd4bf51EP87/TbwWjyn3wFem38dxP9Ovw28Fs/pd4DX5l8H8b/TbwOvxXP6HeC1+ddB/O/028Br8Zx+B3ht/nUQ/7Lf4j/exwB/zb/sOPBWwIOBlwb+GtgF3ht4aZ7T7wCvzb8O4l9m/uO9DvDbvHCfBXw0cJwXze8Ar82/DuJfZv7jvQ7w2zx/x4GfAl6bf53fAV6bfx3Ev8z8x3sd4Ld5/r4LeG/+9X4HeG3+dRD/MvMf73WA3+Z5vTXwU/zb/A7w2vzrIP5lv81/vI8G/prn9VfAS/Nv8zvAa/Ovg/if48HA0/m3+x3gtfnXQfzP8drAb/Fv9zvAa/Ovg/if47WB3+Lf7meAt+ZfB/E/x2sDv8W/3ecAn82/DuJ/FvNv9zbAT/Ovg/if5buB9+Jf7xLwYGCXfx3E/ywPBp7Ov97nAJ/Nvx7if573Br6LF93PAG/Nvw3if6b3Br6Lf9n3AB8N7PJvg/if68HAZwPvxfP6G+CzgZ/m3wfxv8Nr82y3ArfyHwPx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hGpCGtBmNhw5AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQueueMusic;
impl IconShape for MdQueueMusic {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 6H3v2h12V6zm0 4H3v2h12v-2zM3 16h8v-2H3v2zM17 6v8.18c-.31-.11-.65-.18-1-.18-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3V8h3V6h-5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQueuePlayNext;
impl IconShape for MdQueuePlayNext {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,3H3C1.89,3,1,3.89,1,5v12c0,1.1,0.89,2,2,2h5v2h8v-2h2v-2H3V5h18v8h2V5C23,3.89,22.1,3,21,3z M13,10V7h-2v3H8v2h3v3 h2v-3h3v-2H13z M24,18l-4.5,4.5L18,21l3-3l-3-3l1.5-1.5L24,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRadio;
impl IconShape for MdRadio {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.24 6.15C2.51 6.43 2 7.17 2 8v12c0 1.1.89 2 2 2h16c1.11 0 2-.9 2-2V8c0-1.11-.89-2-2-2H8.3l8.26-3.34L15.88 1 3.24 6.15zM7 20c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm13-8h-2v-2h-2v2H4V8h16v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRecentActors;
impl IconShape for MdRecentActors {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 5v14h2V5h-2zm-4 14h2V5h-2v14zM14 5H2c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6c0-.55-.45-1-1-1zM8 7.75c1.24 0 2.25 1.01 2.25 2.25S9.24 12.25 8 12.25 5.75 11.24 5.75 10 6.76 7.75 8 7.75zM12.5 17h-9v-.75c0-1.5 3-2.25 4.5-2.25s4.5.75 4.5 2.25V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v86700cIz/WJeAv+Zfzzyv1wF+G0A822sDv8XzEv96vw28Fv+xfgd4bf71zPN6HeC3AcSzvTbwWzwv8a/328Br8R/rd4DX5l/PPK/XAX4bQDzbawO/xfMS/3q/DbwW/7F+B3ht/vXM83od4LcBxLO9NvBbPC/xr/fbwGvxH+t3gNfmX888r9cBfhtAPNtrA7/F8xL/er8NvBb/sX4HeG3+9czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF6/zf9ur83zeh3gtwHEs7008Ff8//AywF8DiOd0K/Ag/m97BvBgrkA8p9cGfov/214H+G2uQDyv9wa+GjjG/y2XgPcGfppnQzx/x4H3Bl4aeDD/ei8NHOM/1iXgr/nXuxX4a+C7gV2eE+I/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4z/HbwGvxH+t3gNfmPxbiP8dvA6/Ff6zfAV6b/1iI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CNk2pNBjCae8wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRemoveFromQueue;
impl IconShape for MdRemoveFromQueue {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,3H3C1.89,3,1,3.89,1,5v12c0,1.1,0.89,2,2,2h5v2h8v-2h5c1.1,0,1.99-0.9,1.99-2L23,5C23,3.89,22.1,3,21,3z M21,17H3V5 h18V17z M16,10v2H8v-2H16z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O7038N38+yH+d/pt4OnA+/Dvg/jf6beB1wK+G3gf/u0Q/zv9NvBaXPHdwPvwb4P43+m3gdfi2b4beB/+9RD/O/028Fo8p+8G3od/HcT/Tr8NvBbP67uB9+FFh/jf6beB1+L5+27gfXjRIP5tvhp4Kf77vDRwnBfsu4H34V+G+Lf5beC1+J/tu4H34YVD/Nv8NvBa/M/33cD78IIh/m1+G3gt/ud7H+C7ecEQ/za/DbwW/7O9D/DdvHCIf5vfBl6L53QJ+Gv+a7w0cIwX7H2A7+Zfhvi3+W3gtXhOvwO8Nv81fht4LZ6/9wG+mxcN4t/mt4HX4jn9DvDa/Nf4beC1eF7vA3w3LzrEv81vA6/Fc/od4LX5r/HbwGvxnN4H+G7+dRD/Nr8NvBbP6XeA1+a/xm8Dr8WzvQ/w3fzrIf5tfht4LZ7T7wCvzX+N3wZeiyveB/hu/m0Q/za/DbwWz+l3gNfmv8ZvA68FvA/w3fzbIf5tfht4LZ7T7wCvzX+N3wa+G/hu/n0Q/za/DbwWz+l3gNfmv8ZLA3/Nvx/i3+a3gdfiOf0O8Nr874L4t/lt4LV4Tr8DvDb/uyD+bV4aOM5z2gX+mv9dEP+/If5/Q/z/hvj/DfG/00sDx3hOl4C/5l8H8b/TbwOvxXP6HeC1+ddB/O/028Br8Zx+B3ht/nUQ/3HeG/hu/mv8NvBaPKffAV6bfx3Ef4zvAh4CvDb/NX4beC2e0+8Ar82/DuLf77uA9wZ+B3ht/mv8NvBaPKffAV6bfx3Ev893Ae/NFb8DvDb/NX4beC2e0+8Ar82/DuLf7ruA9+bZfgd4bf5r/DbwWjyn3wFem38dxL/NdwHvzXP6HeC1+a/x28Br8Zx+B3ht/nUQ/3rfBbw3z+t3gNfmv8ZvA6/Fc/od4LX510H863wX8N48f78DvDb/NX4beC2e0+8Ar82/DuJF913Ae/OC7QJ/zX+NlwaO85x+B3ht/nUQL5rvAt6b/9l+B3ht/nUQ/7LvAt6b//l+B3ht/nUQ/7L3Br6L//l+B3ht/nUQL5r3Br6L/9l+B3ht/nUQL7r3Br6LF+wS8Nf89/lr4KP510H867w38F08f78DvDb/uyD+9d4b+C6e1+8Ar83/Loh/m/cGvovn9DvAa/O/C+Lf7r2B7+LZfgd4bf53Qfz7vDfwXVzxO8Br878L4t/vvYHvAn4HeG3+d0H8x3hv4L2B1+Z/F8R/nJcG/pr/XRD/vyH+f0P8/4b4/w3x/xv/CJL9hEG3+V+rAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRepeat;
impl IconShape for MdRepeat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRepeatOn;
impl IconShape for MdRepeatOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1H3c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zM7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/O7038N38+yH+d/pt4OnA+/Dvg/jf6beB1wK+G3gf/u0Q/zv9NvBaXPHdwPvwb4P43+m3gdfi2b4beB/+9RD/O/028Fo8p+8G3od/HcT/Tr8NvBbP67uB9+FFh/jf6beB1+L5+27gfXjRIP5tvhp4Kf77vDRwnBfsu4H34V+G+Lf5beC1+J/tu4H34YVD/Nv8NvBa/M/33cD78IIh/m1+G3gt/ud7H+C7ecEQ/za/DbwW/7O9D/DdvHCIf5vfBl6L53QJ+Gv+a7w0cIwX7H2A7+Zfhvi3+W3gtXhOvwO8Nv81fht4LZ6/9wG+mxcN4t/mt4HX4jn9DvDa/Nf4beC1eF7vA3w3LzrEv81vA6/Fc/od4LX5r/HbwGvxnN4H+G7+dRD/Nr8NvBbP6XeA1+a/xm8Dr8WzvQ/w3fzrIf5tfht4LZ7T7wCvzX+N3wZeiyveB/hu/m0Q/za/DbwWz+l3gNfmRffawGcBr80VvwO8Ni+a3wZeC3gf4Lv5t0P82/w28Fo8p98BXpt/2WsDnwW8Ns/pd4DX5kXz28B3A9/Nvw/i3+a3gdfiOf0O8Nq8YA8Gvgt4bZ6/3wFemxfNSwN/zb8f4t/mt4HX4jn9DvDavGCvDfwWL9jvAK/Nfy3Ev81vA6/Fc/od4LV5wV4b+C1esN8BXpv/Woh/m5cGjvOcdoG/5gU7Drw0z/bSwFfxbL8DvDb/tRD/fV4b+C2e7XeA1+a/FuK/z2sDv8Wz/Q7w2vzXQvz3eW3gt3i23wFem/9aiP8+rw38Fs/2O8Br818L8d/ntYHf4tl+B3htXjQvDRzjOV0C/pp/HcR/n9cGfotn+x3gtXnR/DbwWjyn3wFem38dxH+f1wZ+i2f7HeC1edH8NvBaPKffAV6bfx3Ef5z3Br6bF91rA7/Fs/0O8Nq8aH4beC2e0+8Ar82/DuI/xncBDwFemxfdawO/xbP9DvDavGh+G3gtntPvAK/Nvw7i3++7gPcGfgd4bV50rw38Fs/2O8Br86L5beC1eE6/A7w2/zqIf5/vAt6bK34HeG3+a/w28Fo8p98BXpt/HcS/3XcB782z/Q7w2vzX+G3gtXhOvwO8Nv86iH+b7wLem+f0O8Br81/jt4HX4jn9DvDa/Osg/vW+C3hvntfvAK/Nf43fBl6L5/Q7wGvzr4P41/ku4L15/n4HeG3+a/w28Fo8p98BXpt/HcSL7ruA9+YF2wX+mv8aLw0c5zn9DvDa/OsgXjTfBbw3/7P9DvDa/Osg/mXfBbw3//P9DvDa/Osg/mXvDXwX//P9DvDa/OsgXjTvDXwX/7P9DvDa/OsgXnTvDXwXL9gl4K/57/PXwEfzr4P413lv4Lt4/n4HeG3+d0H867038F08r98BXpv/XRD/Nu8NfBfP6XeA1+Z/F8S/3XsD38Wz/Q7w2vzvgvj3eW/gu7jid4DX5n8XxL/fewPfBfwO8Nr874L4j/HewHsDr83/Loj/OC8N/DX/uyD+f0P8/4b4/w3x/xvi/zf+EVbBpEGr1cd6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRepeatOne;
impl IconShape for MdRepeatOne {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4zm-4-2V9h-1l-2 1v1h1.5v4H13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRepeatOneOn;
impl IconShape for MdRepeatOneOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1H3c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zM7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4zm-4-2V9h-1l-2 1v1h1.5v4H13z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFSElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13Fgl/8+iP8+x4HfAl6G/z6I/x7Hgd8CXhoQ/30Q//WOA78FvDRXiP8+iP9ax4HfAl6aZxP/fRD/dY4DvwW8NM9J/PdB/Nc4DvwW8NI8L/HfB/Gf7zjwW8BL8/yJ/z6I/1zHgd8CXpoXTPz3QfznOQ78FvDSvHDivw/iP8dx4LeAl+ZfJv77IP7jHQd+C3hpXjQ/Dfw1V9wK/A3w1/zXQPzHOg78FvDS/PvsAj8N/DTwM/znQfzHOQ78FvDS/Me6Ffhs4Hv4j4f4j3Mc+G3gpfjPcSvwPsBv8x8H8R/rOPDbwEvxn+e7gY8Bdvn3Q/zHOw78NvBSvGh+hyteixfdXwOvA+zy74P4z3Ec+G3gpfiXiWc7Drw18NbAW/HC/TXwOsAu/3aI/zzHgd8GXooXTjx/DwY+G3gvXrC/Bl4H2OXfBvGf6zjw28BL8YKJF+7BwE8DL8Xz99vA6/Bvg/jPdxz4beCleP7Ev+w48N3AW/H8fQ7w2fzrIf5rHAd+G3gpnpd40X038F48fy8D/DX/Ooj/OseB3wZeiuck/nV+G3gtntdvA6/Dvw7iv9Zx4LeBl+LZxL/OceCvgQfxvF4H+G1edIj/eseB3wZeiivEv95bAz/F8/oZ4K150SH+exwHfht4KUD82/w28Fo8rxPALi8axH+f48BvAy/Nv81bAz/F8/oY4Kt50SD+ex0Hdvm32wWO8Zx+BnhrXjSI/92+G3gvntMucIIXDeJ/t7cGforndQLY5V+G+N/ttYHf4nm9DvDb/MsQ/7s9GHg6z+t1gN/mX4Z40R0HXorn9Tv89zLP63WA3+ZfhnjRvTbwWzwv8d/nOHCR5/U6wG/zL0O86F4b+C2e1+sAv81/j9cGfovn9TrAb/MvQ7zoHgw8nef1OsBv89/jtYHf4nk9BLiVfxniX8c8r68BPpr/Hl8NfBTPS7xoEP86vw28Fs/pr4GX4b/HXwEvzXP6HeC1edEg/nU+G/gsntdDgFv5r/Vg4Ok8r88BPpsXDeJf56WBv+J5fQ/w3vzX+m7gvXheLwP8NS8axL/ercCDeF4PAW7lv8aDgafzvP4GeGledIh/vfcGvovn9dPA2/Bf46eAt+Z5vQ/w3bzoEP82twIP4nl9DPDV/Of6bOCzeF7PAB7Mvw7i3+a9ge/i+Xsb4Kf5z/HWwE/x/L0N8NP86yD+7X4beC2e1y7wMcB38x/rvYHv4vn7HeC1+ddD/NsdB24FjvH8fTXwMfzH+Crgo3n+LgEPBnb510P8+7w08Fe8YLcC7wP8Nv82rw18F/BgXrCXAf6afxvEv997A9/FC/fbwHcDPwPs8sIdB94KeG/gtXnh3gf4bv7tEP8x3hv4auAY/7LfBv4a2AVu5YoHA8eBlwZem3/ZJeC9gZ/m3wfxH+elgZ8GHsR/rmcAbw38Nf9+iP9Yx4GvBt6L/xzfA3w0sMt/DMR/jtcGPht4Lf5j/A7w2cBv8x8L8Z/rtYH3Bt4aOMa/ziXgp4HvBn6b/xyI/zqvDbw28NrAceCleE5/A+wCvw38NvDb/OdD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BFNenQfTNctoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplay;
impl IconShape for MdReplay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplay10;
impl IconShape for MdReplay10 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11.99,5V1l-5,5l5,5V7c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6h-2c0,4.42,3.58,8,8,8s8-3.58,8-8S16.41,5,11.99,5z",
            }
            path {
                d: "M10.89,16h-0.85v-3.26l-1.01,0.31v-0.69l1.77-0.63h0.09V16z",
            }
            path {
                d: "M15.17,14.24c0,0.32-0.03,0.6-0.1,0.82s-0.17,0.42-0.29,0.57s-0.28,0.26-0.45,0.33s-0.37,0.1-0.59,0.1 s-0.41-0.03-0.59-0.1s-0.33-0.18-0.46-0.33s-0.23-0.34-0.3-0.57s-0.11-0.5-0.11-0.82V13.5c0-0.32,0.03-0.6,0.1-0.82 s0.17-0.42,0.29-0.57s0.28-0.26,0.45-0.33s0.37-0.1,0.59-0.1s0.41,0.03,0.59,0.1c0.18,0.07,0.33,0.18,0.46,0.33 s0.23,0.34,0.3,0.57s0.11,0.5,0.11,0.82V14.24z M14.32,13.38c0-0.19-0.01-0.35-0.04-0.48s-0.07-0.23-0.12-0.31 s-0.11-0.14-0.19-0.17s-0.16-0.05-0.25-0.05s-0.18,0.02-0.25,0.05s-0.14,0.09-0.19,0.17s-0.09,0.18-0.12,0.31 s-0.04,0.29-0.04,0.48v0.97c0,0.19,0.01,0.35,0.04,0.48s0.07,0.24,0.12,0.32s0.11,0.14,0.19,0.17s0.16,0.05,0.25,0.05 s0.18-0.02,0.25-0.05s0.14-0.09,0.19-0.17s0.09-0.19,0.11-0.32s0.04-0.29,0.04-0.48V13.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplay30;
impl IconShape for MdReplay30 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,5V1L7,6l5,5V7c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6H4c0,4.42,3.58,8,8,8s8-3.58,8-8S16.42,5,12,5z",
            }
            path {
                d: "M9.56,13.49h0.45c0.21,0,0.37-0.05,0.48-0.16s0.16-0.25,0.16-0.43c0-0.08-0.01-0.15-0.04-0.22s-0.06-0.12-0.11-0.17 s-0.11-0.09-0.18-0.11s-0.16-0.04-0.25-0.04c-0.08,0-0.15,0.01-0.22,0.03s-0.13,0.05-0.18,0.1s-0.09,0.09-0.12,0.15 s-0.05,0.13-0.05,0.2H8.65c0-0.18,0.04-0.34,0.11-0.48s0.17-0.27,0.3-0.37s0.27-0.18,0.44-0.23s0.35-0.08,0.54-0.08 c0.21,0,0.41,0.03,0.59,0.08s0.33,0.13,0.46,0.23s0.23,0.23,0.3,0.38s0.11,0.33,0.11,0.53c0,0.09-0.01,0.18-0.04,0.27 s-0.07,0.17-0.13,0.25s-0.12,0.15-0.2,0.22s-0.17,0.12-0.28,0.17c0.24,0.09,0.42,0.21,0.54,0.39s0.18,0.38,0.18,0.61 c0,0.2-0.04,0.38-0.12,0.53s-0.18,0.29-0.32,0.39s-0.29,0.19-0.48,0.24s-0.38,0.08-0.6,0.08c-0.18,0-0.36-0.02-0.53-0.07 s-0.33-0.12-0.46-0.23s-0.25-0.23-0.33-0.38s-0.12-0.34-0.12-0.55h0.85c0,0.08,0.02,0.15,0.05,0.22s0.07,0.12,0.13,0.17 s0.12,0.09,0.2,0.11s0.16,0.04,0.25,0.04c0.1,0,0.19-0.01,0.27-0.04s0.15-0.07,0.2-0.12s0.1-0.11,0.13-0.18s0.04-0.15,0.04-0.24 c0-0.11-0.02-0.21-0.05-0.29s-0.08-0.15-0.14-0.2s-0.13-0.09-0.22-0.11s-0.18-0.04-0.29-0.04H9.56V13.49z",
            }
            path {
                d: "M15.3,14.24c0,0.32-0.03,0.6-0.1,0.82s-0.17,0.42-0.29,0.57s-0.28,0.26-0.45,0.33s-0.37,0.1-0.59,0.1 s-0.41-0.03-0.59-0.1s-0.33-0.18-0.46-0.33s-0.23-0.34-0.3-0.57s-0.11-0.5-0.11-0.82V13.5c0-0.32,0.03-0.6,0.1-0.82 s0.17-0.42,0.29-0.57s0.28-0.26,0.45-0.33s0.37-0.1,0.59-0.1s0.41,0.03,0.59,0.1s0.33,0.18,0.46,0.33s0.23,0.34,0.3,0.57 s0.11,0.5,0.11,0.82V14.24z M14.45,13.38c0-0.19-0.01-0.35-0.04-0.48c-0.03-0.13-0.07-0.23-0.12-0.31s-0.11-0.14-0.19-0.17 s-0.16-0.05-0.25-0.05s-0.18,0.02-0.25,0.05s-0.14,0.09-0.19,0.17s-0.09,0.18-0.12,0.31s-0.04,0.29-0.04,0.48v0.97 c0,0.19,0.01,0.35,0.04,0.48s0.07,0.24,0.12,0.32s0.11,0.14,0.19,0.17s0.16,0.05,0.25,0.05s0.18-0.02,0.25-0.05 s0.14-0.09,0.19-0.17s0.09-0.19,0.11-0.32c0.03-0.13,0.04-0.29,0.04-0.48V13.38z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13Fgl/8+iP8+x4HfAl6G/z6I/x7Hgd8CXhoQ/30Q//WOA78FvDRXiP8+iP9ax4HfAl6aZxP/fRD/dY4DvwW8NM9J/PdB/Nc4DvwW8NI8L/HfB/Gf7zjwW8BL8/yJ/z6I/1zHgd8CXpoXTPz3QfznOQ78FvDSvHDivw/iP8dx4LeAl+ZfJv77IP7jHQd+C3hpXjQ/Dfw1V9wK/A3w1/zXQPzHOg78FvDS/PvsAj8N/DTwM/znQfzHOQ78FvDS/Me6Ffhs4Hv4j4f4j3Mc+G3gpfjPcSvwPsBv8x8H8R/rOPDbwEvxn+e7gY8Bdvn3Q/zHOw78NvBSvGh+hyteixfdXwOvA+zy74P4z3Ec+G3gpfiXiWc7Drw18NbAW/HC/TXwOsAu/3aI/zzHgd8GXooXTjx/DwY+G3gvXrC/Bl4H2OXfBvGf6zjw28BL8YKJF+7BwE8DL8Xz99vA6/Bvg/jPdxz4beCleP7Ev+w48N3AW/H8fQ7w2fzrIf5rHAd+G3gpnpd40X038F48fy8D/DX/Ooj/OseB3wZeiuck/nV+G3gtntdvA6/Dvw7iv9Zx4LeBl+LZxL/OceCvgQfxvF4H+G1edIj/eseB3wZeiivEv95bAz/F8/oZ4K150SH+exwHfht4KUD82/w28Fo8rxPALi8axH+f48BvAy/Nv81bAz/F8/oY4Kt50SD+ex0Hdvm32wWO8Zx+BnhrXjSI/92+G3gvntMucIIXDeK/x3cBD+Z5fQ/w3bzo3hr4KZ7XCWCXfxniv4d5/j4H+GxedK8N/BbP63WA3+Zfhvivdxy4yBWvw3O6FbiVF92DgafzvF4H+G3+ZYgX3XHgpXhev8O/zmsDvwX8DfDS/PuZ5/U6wG/zL0O86F4b+C2el/jXeW3gt4Bbgb8GjgO3Al8D/DX/OseBizyv1wF+m38Z4kX32sBv8bxeB/htXnSfDXwWz9/LAH/Ni+61gd/ieb0O8Nv8yxAvugcDT+d5vQ7w27zojgOvzRU/DRwHfhp4LeBngLfmRffawG/xvB4C3Mq/DPGvY57X1wAfzb/PZwOfBfw18DK86L4a+Ciel3jRIP51fht4LZ7TXwMvw4vuvYEHAb8D/DZXfDbwWcDfAC/Ni+6vgJfmOf0O8Nq8aBD/Op8NfBbP6yHArbxovht4L+CvgbcBdoHfAl4a+B7gvXnRPBh4Os/rc4DP5kWD+Nd5aeCveF7fA7w3L5oHA38NHOM5XQJeGriVF813A+/F83oZ4K950SD+9W4FHsTzeghwKy+a48BnAy/NFX8NfDVwKy+aBwNP53n9DfDSvOgQ/3rvDXwXz+ungbfhv8ZPAW/N83of4Lt50SH+bW4FHsTz+hjgq/nP9dnAZ/G8ngE8mH8dxL/NewPfxfP3NsBP85/jrYGf4vl7G+Cn+ddB/Nv9NvBaPK9d4GOA7+Y/1nsD38Xz9zvAa/Ovh/i3Ow7cChzj+ftq4GP4j/FVwEfz/F0CHgzs8q+H+Pd5aeCveMFuBd4H+G3+bV4b+C7gwbxgLwP8Nf82iH+/9wa+ixfut4HvBn4G2OWFOw68FfDewGvzwr0P8N382yH+Y7w38NXAMf5lvw38NbAL3MoVDwaOAy8NvDb/skvAewM/zb8P4j/OSwM/DTyI/1zPAN4a+Gv+/RD/sY4DXw28F/85vgf4aGCX/xiI/xyvDXw28Fr8x/gd4LOB3+Y/FuI/12sD7w28NXCMf51LwE8D3w38Nv85EP91Xht4beC1gePAS/Gc/gbYBX4b+G3gt/nPh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AlZOy0HsvsKOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplay5;
impl IconShape for MdReplay5 {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,5V1L7,6l5,5V7c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6H4c0,4.42,3.58,8,8,8s8-3.58,8-8S16.42,5,12,5z",
            }
            path {
                d: "M10.69,13.9l0.25-2.17h2.39v0.71h-1.7l-0.11,0.92c0.03-0.02,0.07-0.03,0.11-0.05s0.09-0.04,0.15-0.05 s0.12-0.03,0.18-0.04s0.13-0.02,0.2-0.02c0.21,0,0.39,0.03,0.55,0.1s0.3,0.16,0.41,0.28s0.2,0.27,0.25,0.45s0.09,0.38,0.09,0.6 c0,0.19-0.03,0.37-0.09,0.54s-0.15,0.32-0.27,0.45s-0.27,0.24-0.45,0.31s-0.39,0.12-0.64,0.12c-0.18,0-0.36-0.03-0.53-0.08 s-0.32-0.14-0.46-0.24s-0.24-0.24-0.32-0.39s-0.13-0.33-0.13-0.53h0.84c0.02,0.18,0.08,0.32,0.19,0.41s0.25,0.15,0.42,0.15 c0.11,0,0.2-0.02,0.27-0.06s0.14-0.1,0.18-0.17s0.08-0.15,0.11-0.25s0.03-0.2,0.03-0.31s-0.01-0.21-0.04-0.31 s-0.07-0.17-0.13-0.24s-0.13-0.12-0.21-0.15s-0.19-0.05-0.3-0.05c-0.08,0-0.15,0.01-0.2,0.02s-0.11,0.03-0.15,0.05 s-0.08,0.05-0.12,0.07s-0.07,0.06-0.1,0.09L10.69,13.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReplayCircleFilled;
impl IconShape for MdReplayCircleFilled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm6 10c0 3.31-2.69 6-6 6s-6-2.69-6-6h2c0 2.21 1.79 4 4 4s4-1.79 4-4-1.79-4-4-4v3L8 7l4-4v3c3.31 0 6 2.69 6 6z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEOklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40b008FHAWwPH+Z9pF/hp4HOAW/mXIV407w18F/+7vA/w3bxwiH/ZSwN/xf9ODwFu5QVD/Mu+G3gv/nf6HuC9ecEQ/7KLwHH+d9oFTvCCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeaFuwR8NPDd/MteGvhu4KV4Xr8DvDYvuvcGvho4xgsnXjDEv8y8cO8DfDcvuuPArcAxntPvAK/Nv857A9/FCydeMMS/zLxw4l/vt4HX4jn9DvDa/OuZF068YIh/mXnhxL/ebwOvxXP6HeC1+dczL5x4wRD/MvPCiedkntfrAL/Ns/028Fo8p98BXptne23gt3he4jmZF068YIh/mXnhxHMyz+uvgV2e7aWB4zyn3wFem2d7beC3eF7iOZkXTrxgiH+ZeeHEc/ptXrDjwEvx/P0O8No822sDv8XzEs/JvHDiBUP8y8wL9zLAX/Oi+2zgs3hevwO8Ns/22sBv8bzEczIvnHjBEP8y88L9NfA6wC4vur8GXorn9DvAa/Nsrw38Fs9LPCfzwokXDPEvM/+yXeCvecE+Bvhrnu23gdfiOf0O8No822sDv8XzEs/JvHDiBUP8y8wL9zs8p9fieb0O8Ns8228Dr8Vz+h3gtXm21wZ+i+clnpN54cQLhviXmRdOPCfzvF4H+G2e7beB1+I5/Q7w2jzbawO/xfMSz8m8cOIFQ/zLzAsn/vV+G3gtntPvAK/Nv5554cQLhviXmRdO/Ov9NvBaPKffAV6bfz3zwokXDPEvMy/c+wDfzYvuOPB04DjP6XeA1+Zf572B7+KFEy8Y4l9mXrhd4KOB7+Ff9tLAdwEvzfP6HeC1edG9F/DVwHFeOPGCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+ZbvAMf53ugQc5wVD/Mu+G3gv/nf6HuC9ecEQ/7KXBv6K/50eAtzKC4Z40bw38F387/I+wHfzwiFedA8GPht4a+AY/zNdAn4a+GzgVv5liP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjCTCKQcfrcD8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSd;
impl IconShape for MdSd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-6 6h4c.55 0 1 .45 1 1v4c0 .55-.45 1-1 1h-4V9zm-3.5 4.5v-1H7c-.55 0-1-.45-1-1V10c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1H9.5v-.5h-2v1H10c.55 0 1 .45 1 1V14c0 .55-.45 1-1 1H7c-.55 0-1-.45-1-1v-1h1.5v.5h2zm5 0h2v-3h-2v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3+7BwK3853ot/m3+BtjlX4b4t3lv4LuA9wG+m/887w18F/96rwP8Nv8yxL/eewPfxbO9D/Dd/Od5b+C7+Nd5HeC3+Zch/nXeG/guntf7AN/Nf573Br6LF93rAL/Nvwzxonsw8HResPcBvpv/PO8NfBcvmtcBfpt/GeJf572B7+IFex/gu/nP897Ad/Evex3gt/mXIf713hv4Ll6w9wG+m/8c7w18F/+y1wF+m38Z4t/mvYHv4gV7H+C7+Y/13sB38aJ5HeC3+Zch/u3eG/guXrD3Ab6b/xjvDXwXL7rXAX6bfxni3+e9ge/iBXsf4Lv593lv4Lv413kd4Lf5lyH+/d4b+C5esPcBvpt/m/cGvosX7H244rt4Tq8D/Db/MsR/jPcGvosX7H2A7+Zf572B7+IFex/gu7nivYHv4tleB/ht/mWI/zjvDXwXL9j7AN/Ni+a9ge/iBXsf4Lt5Tu8NfBdXvA7w2/zLEP+x3hv4Ll6w9wG+mxfuvYHv4gV7H+C7ef7eG/gu4HWA3+ZfhviP997Ad/H8XQIeDOzy/L038F28YO8DfDcv3HsDtwK/zb8M8Z/jvYHv4jldAl4b+Guev/cGvosX7H2A7+Y/FuI/z3sD38UVl4DXBv6a5++9ge/iBXsf4Lv5j4f4z/XewFcDrw38Nc/fewPfxQv2PsB3858D8Z/vOLDL8/fewHfxgr0P8N3850H893lv4Lt4wd4H+G7+cyH+e7w38F28YO8DfDf/+RD/9d4b+C5esPcBvpv/Goj/Wu8NfBcv2PsA381/HcR/nfcGvosX7H2A7+a/FuK/xnsD38UL9j7Ad/NfD/Gf772B7+IFex/gu3n+jgO7/OdB/Od6b+C7eMHeB/hunr+XBn4L+Bjgu/nPgfjP897Ad/GCvQ/w3Tx/Lw38FnCcK94H+G7+4yH+c7w38F28YO8DfDfP30sDvwUc5zm9D/DdvGiOA7v8yxD/8d4b+C5esPcBvpsX7FbgQTx/7wN8Ny/cdwHfA/w2/zLEf6z3Br6LF+x9gO/mhXtp4LeBYzx/7wN8N8/fdwHvDbwO8Nv8yxD/cd4b+C5esPcBvpsXzUsDvw0c4/l7H+C7eU7fBbw3V7wO8Nv8yxD/Md4b+C5esPcBvpt/nZcGfhs4xvP3PsB3c8V3Ae/Ns70O8Nv8yxD/fu8NfBcv2PsA382/zUsDvw0c4/l7H+C1gPfmOb0O8Nv8yxD/Pu8NfBcv2PsA382/z0sDvw0c40X3OsBv8y9D/Nu9N/BdvGDvA3w3/zFeGvht4BgvmtcBfpt/GeLf5r2B7+IFex/gu/mP9dLAbwPH+Je9DvDb/MsQ/3rvDXwXL9j7AN/Nf46XBv6Kf9nrAL/Nvwzxr/PewHfxgr0P8N385/ku4L35l70O8Nv8yxAvugcDT+cFex/gu/nP813Ae/OieR3gt/mXIf513hv4Lp7X+wDfzX+e7wLemxfd6wC/zb8M8a/33sB38WzvA3w3/3m+C3hv/nVeB/ht/mWIf5v3Br4LeB/gu/nP813Ae/Ov9zrAb/MvQ/zbPRi4lf88x4GX5t/mr4Fd/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8z3LZBwxk6QwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShuffle;
impl IconShape for MdShuffle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH2klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+IFezDwIP5veAZwK88L8byOA98FvDX/t/w08D7ALs+GeE7HgacDx/m/aRd4CLDLFYjn9FPAW/N/208Db8MViGd7MPB0/n94CHArgHi21wZ+i/8fXgf4bQDxbK8N/Bb/P7wO8NsA4tleG/gt/n94HeC3AcSzvTbwW7zongE8iP9cv8O/zUsDx3jBXgf4bQDxbK8N/BYvmu8B3hv4buC9+M/z3cD78K/328Br8YK9DvDbAOLZXhv4Lf5l3wO8N8/23cB78Z/nu4H34V/nt4HX4gV7HeC3AcSzvTbwW7xw3wO8N8/ru4H34j/PdwPvw4vut4HX4gV7HeC3AcSzvTbwW7xgtwIP4QX7buC9+M/z3cD78KL5beC1eMFeB/htAPFsrw38Fi/cdwPvwwv23cB78Z/nu4H34V/228Br8YK9DvDbAOLZXhv4Lf5l3w28Dy/YdwPvxX+O7wHem3/ZbwOvxQv2OsBvA4hne23gt3jRfDfwPrxg3w28F/+xvgd4b140vw28Fi/Y6wC/DSCe7bWB3+JF993A+/CCfTfwXvzH+B7gvXnR/TbwWrxgrwP8NoB4ttcGfot/ne8G3ocX7LuB9+Lf53uA9+Zf57eB1+IFex3gtwHEs7028Fv863038D68YN8NvBf/Nt8DvDcv2HdxxfvwnH4beC1esNcBfhtAPNtrA7/Fv813A+/DC/bdwHvxr/M9wHvzgn0X8N5c8d3A+/Bsvw28Fi/Y6wC/DSCe7bWB3+Lf7ruB9+EF+27gvXjRfA/w3rxg3wW8N8/pu4H34YrfBl6LF+x1gN8GEM/22sBv8e/z3cD78IJ9N/BevHDfA7w3L9h3Ae/N8/fdwPsAvw28Fi/Y6wC/DSCe7bWB3+Lf77uB9+H5Ow48HTjO8/c9wHvzgn0X8N68cN8NPAR4LV6w1wF+G0A822sDv8V/jO8G3ofndBz4LeClef6+B3hvXrDvAt6b/xivA/w2gHi21wZ+i/843w28D1ccB34LeGmev+8B3psX7LuA9+Y/zusAvw0gnu21gd/iP9Z3Ax8D/Bbw0jx/3wO8Ny/YdwHvzX+s1wF+G0A822sDv8V/vF3gOM/f9wDvzQv2XcB78x/vdYDfBhDP9trAb/Ff53uA9+YF+y7gvfnP8TrAbwOIZ3tt4Lf4r/E9wHvzgn0X8N7853kd4LcBxLO9NvBb/Of7HuC9ecG+C3hv/nO9DvDbAOLZXhv4Lf5zfQ/w3rxg3wW8N//5Xgf4bQDxbK8N/Bb/eb4HeG9esO8C3pv/Gq8D/DaAeLbXBn6L/xzfA7w3L9h3Ae/Nf53XAX4bQDzbawO/xX+87wHemxfsu4D35vm7BBzjP97rAL8NIJ7ttYHf4j/W9wDvzQv2XcB78/z9DfDawFcD78V/rNcBfhtAPNtrA7/Ff5zvAd6bF+y7gPfm+fsb4LWBXa74buC9+I/zOsBvA4hne23gt/iP8T3Ae/OCfRfw3jx/fwO8NrDLc/pu4L140VwCjvGCvQ7w2wDi2V4b+C3+/b4HeG9esO8C3psX7CHArTx/3w28Fy/c+wDvDbwWL9jrAL8NIJ7ttYHf4t/ne4D35gX7LuC9eeH+GngdYJfn77uB9+L5ex/gu4HfBl6LF+x1gN8GEM/22sBv8W/3PcB784J9F/DevGj+GngdYJfn77uB9+I5vQ/w3Vzx28Br8YK9DvDbAOLZXhv4Lf5tvgd4b16w7wLem3+dvwZeB9jl+ftu4L244n2A7+bZfht4LV6w1wF+G0A822sDv8W/3vcA780L9l3Ae/Nv89fA6wC7PH/fDfw28N08p98GXosX7HWA3wYQz/bawG/xr/M9wHvzgn0X8N78+/w18DrALi+63wZeixfsdYDfBhDP9trAb/Gi+x7gvXnBvgt4b/5j/DXwOsAuL5rfBl6LF+x1gN8GEM/22sBv8aL5HuC9ecG+C3hv/mP9NfA6wC7/st8GXosX7HWA3wYQz/bawG/xL/se4L15wb4LeG/+c/w18DL8y34beC1esNcBfhtAPNtrA7/FC/c9wHvzgn0X8N7853kf4Lv5l/028Fq8YK8D/DaAeLbXBn6LF+xW4CG8YN8FvDf/ed4H+G5eNL8NvBYv2OsAvw0gnu21gd/ihftu4H14Xt8FvDf/ed4H+G5edL8NvBYv2OsAvw0gnu21gd/iX/bdwPvwbN8FvDf/ed4H+G7+dX4beC1esNcBfhtAPNtrA7/Fi+a7gfcBvgt4b/7zvA/w3fzr/TbwWrxgrwP8NoB4ttcGfosX3a3Ag/nPswv8Nf82Lw0c5wV7HeC3AcSzvTbwW/z/8DrAbwOIZ3tt4Lf4/+F1gN8GEM/22sBv8f/D6wC/DSCe7cHA0/n/4SHArQDiOf008Fb83/YzwFtzBeI5HQduBY7xf9PfAK8N7HIF4nkdB74beCv+b/kZ4L2BXZ4N8YI9GHgw/zfcCtzK80L8/4b4/41/BPXSNlCCr6aIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdShuffleOn;
impl IconShape for MdShuffleOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 1H3c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zM10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADoklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pf7KeB9gF3+c7028Fs8L/Fvh/j3M3Ar8D7Ab/Of57WB3+J5iX87xL+febbPBj6H/xyvDfwWz0v82yH+/cxz+mvgfYC/5j/WawO/xfMS/3aIfz/zvHaBzwa+hv84rw38Fs9L/Nsh/v3MC/bTwPsAu/z7vTbwWzwv8W+H+PczL9wu8D7AT/Pv89rAb/G8xL8d4t/PvGi+GvgcYJd/m9cGfovnJf7tEP9+5kX318D7AH/Nv95rA7/F8xL/doh/P/Ov99nA5/Cv89rAb/G8xL8d4t/P/Nv8NvA+wK28aF4b+C2el/i3Q/z7mX+7XeB9gJ/mX/bawG/xvMS/HeLfz/z7/TTwPsAuL9hrA7/F8xL/doh/P/Mf41bgfYDf5vl7beC3eF7i3w7x72f+Y3028Dk8r9cGfovnJf7tEP9+5j/eXwPvA/w1z/bawG/xvMS/HeLfz/zH+xvgvYG/5tleG/gtnpf4t0P8+5n/WJ8DfDbP67WB3+J5iX87xL+f+Y/xDOC9gd/m+Xtt4Ld4XuLfDvHvZ/79fgZ4b2CXF+y1gd/ieYl/O8S/n/m3uwS8N/DT/MteG/gtnpf4t0P8+5l/m98B3hu4lRfNawO/xfMS/3aIfz/zr/c5wGfzr/PawG/xvMS/HeLfz7zo/gZ4b+Cv+dd7beC3eF7i3w7x72deNF8DfDawy7/NawO/xfMS/3aIfz/zwl0C3hv4af59Xhv4LZ6X+LdD/PuZF+xngPcGdvn3e23gt3he4t8O8e9nntcl4LOBr+Y/zmsDv8XzEv92iH8/85z+Bnhv4K/5j/XawG/xvMS/HeLfzzzb5wCfzX+O1wZ+i+cl/u0Q/34GngG8N/Db/Od5beC3eF7i3w7x7/fTwHsDu/znem3gt3he4t8O8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BBIXeEFb/EigAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSkipNext;
impl IconShape for MdSkipNext {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pczz+t1gN/mP9dx4LuAt+HfDvHvZ57X6wC/zX+e1wa+C3gwIP7tEP9+5nm9DvDb/Of4LOCzeTbxb4f49zPP63WA3+Y/1ksD3wW8NM9J/Nsh/v3M83od4Lf5j/NRwGcDx3le4t8O8e9nntfrAL/Nv99x4LuAt+YFE/92iH8/87xeB/ht/n3eGvgu4DgvnPi3Q/z7mef1OsBv829zHPgs4KN50Yh/O8S/n3lerwP8Nv96Lw18F/DSvOjEvx3i3888r9cBfpt/nc8CPpt/PfFvh/j3M8/rdYDf5kXzYOC7gNfm30b82yH+/czzeh3gt/mXvTXwXcBx/u3Evx3i3888r9cBfpsX7DjwXcBb8+8n/u0Q/37meb0O8Ns8f68NfBfwYP5jiH87xL+feV6vA/w2z+uzgM/mP5b4t0P8+5nn9TrAb/NsLw18F/DS/McT/3aIfz/zvF4H+G2e7aWB7wZeiv944t8O8e9nntfrAL/N8/ps4LP4jyX+7RD/fuZ5vQ7w2zx/rw18N/Ag/mOIfzvEv595Xq8D/DYv2HHgu4G34t9P/Nsh/v3M83od4Lf5l7018N3AMf7txL8d4t/PPK/XAX6bF82Dge8GXot/G/Fvh/j3M8/rdYDf5l/ns4HP4l9P/Nsh/v3M83od4Lf513tp4LuBl+JFJ/7tEP9+5nm9DvDb/NscBz4b+CheNOLfDvHvZ57X6wC/zb/PWwPfDRzjhRP/doh/P/O8Xgf4bf79jgPfDbwVL5j4t0P8+5nn9TrAb/Mf56OBzwaO8bzEvx3i3888r9cBfpv/WC8NfDfwUjwn8W+H+Pczz+t1gN/mP8dnA5/Fs4l/O8S/n3lerwP8Nv95Xhv4buBBgPi3Q/z7mef1OsBv85/rOPDdwFvzb4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CkSZgQf3ReQoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSkipPrevious;
impl IconShape for MdSkipPrevious {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 6h2v12H6zm3.5 6l8.5 6V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSlowMotionVideo;
impl IconShape for MdSlowMotionVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.05 9.79L10 7.5v9l3.05-2.29L16 12zm0 0L10 7.5v9l3.05-2.29L16 12zm0 0L10 7.5v9l3.05-2.29L16 12zM11 4.07V2.05c-2.01.2-3.84 1-5.32 2.21L7.1 5.69c1.11-.86 2.44-1.44 3.9-1.62zM5.69 7.1L4.26 5.68C3.05 7.16 2.25 8.99 2.05 11h2.02c.18-1.46.76-2.79 1.62-3.9zM4.07 13H2.05c.2 2.01 1 3.84 2.21 5.32l1.43-1.43c-.86-1.1-1.44-2.43-1.62-3.89zm1.61 6.74C7.16 20.95 9 21.75 11 21.95v-2.02c-1.46-.18-2.79-.76-3.9-1.62l-1.42 1.43zM22 12c0 5.16-3.92 9.42-8.95 9.95v-2.02C16.97 19.41 20 16.05 20 12s-3.03-7.41-6.95-7.93V2.05C18.08 2.58 22 6.84 22 12z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSnooze;
impl IconShape for MdSnooze {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm-3-9h3.63L9 15.2V17h6v-2h-3.63L15 10.8V9H9v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSortByAlpha;
impl IconShape for MdSortByAlpha {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.94 4.66h-4.72l2.36-2.36zm-4.69 14.71h4.66l-2.33 2.33zM6.1 6.27L1.6 17.73h1.84l.92-2.45h5.11l.92 2.45h1.84L7.74 6.27H6.1zm-1.13 7.37l1.94-5.18 1.94 5.18H4.97zm10.76 2.5h6.12v1.59h-8.53v-1.29l5.92-8.56h-5.88v-1.6h8.3v1.26l-5.93 8.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviv8dx4KV4tkvAX/NfD/Gf78HAawGvDbw08NK8cH8N/DXw28DfAH/Nfx7Ef47jwHsB7w28NP8+fw18N/A9wC7/sRD/sR4MfBbw1sBx/uN9N/A5wK38x0D8xzgOfBTw0cBx/vN9NvA1wC7/Poh/v5cGvgt4af7r/Azw3sAu/z6If5/3Br6LF83vAL8N/DVwK/DXPKfXBo4DLw28NvBaPK9LwHsDP81/DMS/3XcB780L9zfAVwM/Dezyr3MceGvgo4GXAn4GeG9gl/84iH+b7wLemxfsd4DPBn6b/xgPBm7lhXswcCv/Ooh/va8GPorn7xLw2cBX81/nOPBdwOcAf82/DuJf572B7+L5+xvgrYFb+a/z1sB3AV8DfDb/eogX3VsDP8Xz9z3ARwO7/Nc4DnwX8NbA3wAvzb8N4kVzHPgr4ME8r+8B3pv/Om8NfBdwnCteBvhrntdxYJcXDvGi+Wrgo3hefwO8NrDLf77jwHcBb82zfQ7w2TyvBwNfBbwNLxziX/bSwF/xvC4BLw3cyn++twa+CzjOs/0N8NI8f78FvDbwOsBv84Ih/mW/Bbw2z+t1gN/mP9dx4LuAt+Z5vQzw1zyvjwa+iituBR7CC4Z44R4MPJ3n9TvAa/Of662B7wKO87w+B/hsnteDgb8CjvNs7wN8N88f4oX7buC9eF4PAW7lP8dx4LuAt+b5+xvgpXn+fgt4bZ7TzwBvzfOHeMGOA08HjvOcvgd4b/5zvDXwXcBxXrCXAf6a5++7gffieT0EuJXnhXjB3hv4Lp7XywB/zX+s48B3AW/NC/c5wGfzgr008Fc8r48BvprnhXjBvhr4KJ7TM4AH8x/rrYHvAo7zwv0N8NL8y/4aeCme0+8Ar83zQrxgfwW8NM/pc4DP5j/GceC7gLfmRfMywF/zL/tq4KN4TrvACZ4X4vk7Dlzkeb0O8Nv8+7018F3AcV40nwN8Ni+a1wZ+i+f1MsBf85wQz99rA7/F8zoB7PLv813Ae/Oi+xvgpfnXMc/rbYCf5jkhnr+3Bn6K5/QM4MH8+3w28Fn867wM8Nf86+wCx3hOnwN8Ns8J8fx9NvBZPKffAV6bf5+LwHFedJ8DfDb/er8NvBbP6XOAz+Y5IZ6/zwY+i+f0O8Br82/32sBv8aL7G+Cl+bf5beC1eE4/A7w1zwnx/H028Fk8p98BXpt/u9cGfosX3csAf82/zW8Dr8Vz+hrgo3lOiOfvs4HP4jn9DvDa/Nu9NPBXvGg+B/hs/u1+G3gtntPnAJ/Nc0I8f58NfBbP6a+Bl+Hf51bgQbxwfwO8NP8+vw28Fs/pc4DP5jkhnr/XBn6L5yX+fV4b+C1euJcB/pp/H/O8Pgf4bJ4T4vl7beC3eF4ngF3+fd4a+G7gGM/pEvDRwHfz7/Ng4Ok8r9cBfpvnhHj+jgMXeV7vA3w3/37HgbcGXporbgW+G9jl3++tgZ/ieZ0AdnlOiBfsr4GX4jl9D/De/M/23cB78ZwuAcd5XogX7KuBj+I57QIn+J/tInCc5/Q9wHvzvBAv2GsDv8Xzeh/gu/mf6a2Bn+J5vQ3w0zwvxAu3CxzjOf018DL8z/RbwGvznC4Bx3n+EC/cZwOfxfN6HeC3+Z/ltYHf4nl9D/DePH+IF+7BwNP53+0hwK08f4h/2XcD78X/Tt8DvDcvGOJf9mDg6fzv9BDgVl4wxIvmvYHv4n+X9wG+mxcO8aJ7a+CrgQfxP9szgI8Gfpp/GeJf77X5n+23edEh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CNkjWQRzw/1UAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeed;
impl IconShape for MdSpeed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.38 8.57l-1.23 1.85a8 8 0 0 1-.22 7.58H5.07A8 8 0 0 1 15.58 6.85l1.85-1.23A10 10 0 0 0 3.35 19a2 2 0 0 0 1.72 1h13.85a2 2 0 0 0 1.74-1 10 10 0 0 0-.27-10.44zm-9.79 6.84a2 2 0 0 0 2.83 0l5.66-8.49-8.49 5.66a2 2 0 0 0 0 2.83z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+Pcz/73Evx3i38/89xL/doh/P/PfS/zbIf79zH8v8W+H+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AiWMIEEKPTnKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStop;
impl IconShape for MdStop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 6h12v12H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxr/cxwF/zH+ulga/iX+9zgM/m+UO8cE8HHsy/3usAv81/rNcGfot/vVuBh/D8IV6wtwZ+in+b1wF+m/9Yrw38Fv82rwP8Ns8L8YJ9NfBR/Nu8DvDb/Md6beC3+Lf5GuCjeV6IF+yvgJfm3+Z1gN/mP9ZrA7/Fv81fAy/D80I8fw8Gns6/3esAv81/rNcGfot/uxPALs8J8fy9NvBb/Nu9DvDb/Md6beC3+Ld7HeC3eU6I5++jga/i3+51gN/mP9ZrA7/Fv93HAF/Nc0I8f58NfBb/dq8D/Db/sV4b+C3+7T4H+GyeE+L5+2zgs/i3ex3gt/mP9drAb/Fv9znAZ/OcEM/fbwOvxb/d6wC/zX+s1wZ+i3+7nwHemueEeP5+G3gt/u1eB/ht/mO9NvBb/Nv9DPDWPCfE8/fZwGfxb/c6wG/zH+u1gd/i3+5zgM/mOSGev88GPot/u9cBfpv/WK8N/Bb/dp8DfDbPCfH8fTTwVfzbvQ7w2/zHem3gt/i3+xjgq3lOiOfvtYHf4t/udYDf5j/WawO/xb/d6wC/zXNCPH8PBp7Ov93rAL/Nf6zXBn6Lf7sTwC7PCfGC/TXwUvzbvA7w2/zHem3gt/i3+RvgpXleiBfsq4GP4t/mdYDf5j/WawO/xb/N1wAfzfNCvGCvDfwW/zavA/w2/7FeG/gt/m1eB/htnhfihbsVeBD/eq8D/Db/sV4b+C3+9Z4BPJjnD/HCfTbwWfzrfTTw1/zHemngq/nX+xzgs3n+EC/cceBW4Bj/O10CHgzs8vwh/mWfDXwW/zt9DvDZvGCIf9lx4FbgGP+7XAIeDOzygiFeNO8NfBf/u7wN8NO8cIgX3U8Db8X/Dj8DvDX/MsSL7jjw18CD+J/tGcBLA7v8yxD/Oi8N/DZwjP+ZLgGvDfw1LxrEv95LA78NHON/lkvAawN/zYsO8W/z0sBvA8f4n+ES8NrAX/Ovg/i3e2ngp4EH8d/rGcBbA3/Nvx7i3+c48N3AW/Hf42eA9wZ2+bdB/Md4b+CrgWP817gEvDfw0/z7IP7jHAc+Gvho4Bj/OS4BXw18NbDLvx/iP95x4KOB9wYexH+MZwDfDXw1sMt/HMR/rtcG3hp4beCl+Nf5G+C3gZ8Gfpv/HIj/OseBlwZeGjjOFS/NFX/NFbvAXwN/Dezynw/x/xvi/zfE/2+I/98Q/7/xj+yCvEHZmFFUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStopCircle;
impl IconShape for MdStopCircle {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8,16h8V8H8V16z M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10 S17.52,2,12,2L12,2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/Ff6/X4d8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e/32/z3em3+7RD/vyH+f0P8/4b4/w3x/xviRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvuuPAS/O/w18Du/zLEP+/If5/Q/z/hvj/DfGieWngGP+7XAL+mhcO8cK9NvBdwIP53+lW4H2A3+b5Q7xg7w18F/83vA/w3TwvxPN3HHg6cJz/G3aBhwC7PCfE8/fRwFfxf8vHAF/Nc0I8f98NvBf/t3wP8N48J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8R/rd4DvBr4aOMZ/vd8BXpvnhHj+fht4Lf5j/Q7w2sBx4KOBjwaO8V/nd4DX5jkhnr/fBl6L/1i/A7w2z3Yc+Grgvfiv8TvAa/OcEM/fbwOvxX+s3wFem+f1YOCrgbfiP9fvAK/Nc0I8f78NvBb/sX4HeG1esNcGPht4Lf5z/A7w2jwnxPP328Br8R/rd4DX5l/22sB3Aw/iP9bvAK/Nc0I8f78NvBb/sX4HeG1edO8NfDbwIP5j/A7w2jwnxPP328Br8R/rd4DX5l/nOPDRwEcDx/j3+R3gtXlOiOfvt4HX4j/W7wCvzb/NceCngdfi3+53gNfmOSGev98GXov/WL8DvDb/eg8Gvgt4bf59fgd4bZ4T4vn7beC1+I/1O8Br86J7MPBZwHvzH+N3gNfmOSGev98GXov/WL8DvDb/suPARwEfDRznP87vAK/Nc0I8f78NvBb/sX4HeG1euM8CPho4zn+83wFem+eEeP5+G3gt/mP9DvDaPH/vBXw28GD+8/wO8No8J8Tz99vAa/Ef63eA1+Y5vTbwVcBL85/vd4DX5jkhnr/fBl6L/1i/A7w2V7w08FXAa/Nf53eA1+Y5IZ6/3wZei/9YvwO8N/BZwHvzX+93gNfmOSGev98GXov/WLvAcf77/A7w2jwnxPP328Br8X/L7wCvzXNCPH8/DbwV/7f8DPDWPCfE8/fZwGfxf8vnAJ/Nc0I8fy8N/BX/tzwEuJXnhHjBvhr4KP5v+Brgo3leiBfuq4GP4n+3rwE+mucP8S97aeCtgZcGjvO/wy7w18B3A7fygiH+f0P8/4b4/w3x/xvi/zf+EZNNe0GxbQ4XAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubscriptions;
impl IconShape for MdSubscriptions {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvo4Gv4v+WjwG+mueEeP6+G3gv/m/5HuC9eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4jn9DvDavGh+G3gtntPvAK/Nf43fBl6L5/Q7wGvznBDP328Dr8Vz+h3gtXnR/DbwWjyn3wFem/8avw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/NX4beC2e0+8Ar81zQjx/vw28Fs/pd4DX5kXz28Br8Zx+B3ht/mv8NvBaPKffAV6b54R4/n4beC2e0+8Ar82L5reB1+I5/Q7w2vzX+G3gtXhOvwO8Ns8J8fz9NvBaPKdd4K950bw0cJzntAv8Nf81Xho4znP6HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4jldAv6a/xovDRzjOV0C/poXzUsDx3hOvwO8Ns8J8fz9NvBaPKffAV6b/xq/DbwWz+l3gNfmRfPbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv81fht4LZ7T7wCvzYvmt4HX4jn9DvDaPCfE8/fbwGvxnH4HeG3+a/w28Fo8p98BXpsXzW8Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/Nf4beC1eE6/A7w2L5rfBl6L5/Q7wGvznBDP328Dr8Vz+h3gtfmv8dvAa/Gcfgd4bV40vw28Fs/pd4DX5jkhnr/vBt6L/1u+BvhonhPi+fto4Kv4v+VjgK/mOSGev+PArcAx/m+4BDwY2OU5IV6w9wa+i/8b3gb4aZ4X4oV7beC7gQfxv9MzgPcGfpvnD/GieWngOP+77AJ/zQuH+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IBXqMQR3V8vQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSubtitles;
impl IconShape for MdSubtitles {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM4 12h4v2H4v-2zm10 6H4v-2h10v2zm6 0h-4v-2h4v2zm0-4H10v-2h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSurroundSound;
impl IconShape for MdSurroundSound {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM7.76 16.24l-1.41 1.41C4.78 16.1 4 14.05 4 12c0-2.05.78-4.1 2.34-5.66l1.41 1.41C6.59 8.93 6 10.46 6 12s.59 3.07 1.76 4.24zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4zm5.66 1.66l-1.41-1.41C17.41 15.07 18 13.54 18 12s-.59-3.07-1.76-4.24l1.41-1.41C19.22 7.9 20 9.95 20 12c0 2.05-.78 4.1-2.34 5.66zM12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JcdB74KeG/++/018D7AX/MfA/Ev+27gvfifYxd4CLDLv+ylgb/mBUP8y8z/PK8D/DYv3EsDvwWc4AVD/MvM/zyvA/w2L9hLA78FHAfEC4b4l5n/eV4H+G2ev5cGfgs4zhXiBUP8y8y/z+vwvL4aeCn+7V4H+G2e10sDvwUc59nEC4b4l5l/H/G8fht4Lf7tXgf4bZ7TSwO/BRznOYkXDPEvM/8+4nn9NvBa/Nu9DvDbPNtLA78FHOd5iRcM8S8z/z7ief028Fr8270O8Ntc8dLAbwHHef7EC4b4l5l/H/G8fht4Lf7tXgf4beClgd8CjvOCiRcM8S8z/z7ief028Fr8270OsAv8FnCcF068YIh/mfn3Ec/rt4HX4t/uY4DPAo7zLxMvGOJfZv59xPP6beC1+K8hXjDEv8y8cK/DC/fbPK+XBo7zgn018FL8xxAvGOJfZl448R/vt4HX4j+GeMEQ/zLzwon/eL8NvBb/McQLhviXmRdO/Mf7beC1+I8hXjDEv8y8cOI/3m8Dr8V/DPGCIf5l5oV7bV643+F5vTRwjBfsq4GX5j+GeMEQ/zLz7yOe128Dr8V/DfGCIf5l5t9HPK/fBl6Lf7uPAT4bOMa/TLxgiH+Z+fcRz+u3gdfi3+51gF3gt4FjvHDiBUP8y8y/j3hevw28Fv92rwP8NvDSwG8Dx3jBxAuG+JeZfx/xvH4beC3+7V4H+G2ueGngt4FjPH/iBUP8y8y/j3hevw28Fv92rwP8Ns/20sBvA8d4XuIFQ/zLzL+PeF6/DbwW/3avA/w2z+mlgd8GjvGcxAuG+JeZf5/X5nl9NfDS/Nu9DvDbPK+XBn4bOMaziRcM8S8z//O8DvDbPH8vDfw2cIwrxAuG+JeZ/3leB/htXrCXBn4bOAaIFwzxLzP/87wO8Nu8cC8N/DZwnBcM8S/7buC9+J/jEvBgYJd/2UsDf80LhviXHQe+Gngv/vv9DfDewF/zHwPx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EVQdoQcIi9CIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideoCall;
impl IconShape for MdVideoCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4zM14 13h-3v3H9v-3H6v-2h3V8h2v3h3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz+u1+d/tt3lerwP8NoB4ttcGfov/H14H+G0A8WyvDfwW/z+8DvDbAOLZXhv4Lf5/eB3gtwHEs7028Fv8//A6wG8DiGd7beC3+P/hdYDfBhDP9trAb/H/w+sAvw0gnu21gd/i/4fXAX4bQDzbawO/xf8PrwP8NoB4ttcGfov/H14H+G0A8Zx2gWP833YJOM4ViOf02cBn8X/b5wCfzRWI5/XdwHvxf9P3AO/NsyGev/cGPhp4Kf5v+B3gu4Hv5jkh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8OfIBBvvTTfQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideoLabel;
impl IconShape for MdVideoLabel {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 13H3V5h18v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideoLibrary;
impl IconShape for MdVideoLibrary {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8 12.5v-9l6 4.5-6 4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideoSettings;
impl IconShape for MdVideoSettings {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3,6h18v5h2V6c0-1.1-0.9-2-2-2H3C1.9,4,1,4.9,1,6v12c0,1.1,0.9,2,2,2h9v-2H3V6z",
            }
            polygon {
                points: "15,12 9,8 9,16",
            }
            path {
                d: "M22.71,18.43c0.03-0.29,0.04-0.58,0.01-0.86l1.07-0.85c0.1-0.08,0.12-0.21,0.06-0.32l-1.03-1.79 c-0.06-0.11-0.19-0.15-0.31-0.11L21.23,15c-0.23-0.17-0.48-0.31-0.75-0.42l-0.2-1.36C20.26,13.09,20.16,13,20.03,13h-2.07 c-0.12,0-0.23,0.09-0.25,0.21l-0.2,1.36c-0.26,0.11-0.51,0.26-0.74,0.42l-1.28-0.5c-0.12-0.05-0.25,0-0.31,0.11l-1.03,1.79 c-0.06,0.11-0.04,0.24,0.06,0.32l1.07,0.86c-0.03,0.29-0.04,0.58-0.01,0.86l-1.07,0.85c-0.1,0.08-0.12,0.21-0.06,0.32l1.03,1.79 c0.06,0.11,0.19,0.15,0.31,0.11l1.27-0.5c0.23,0.17,0.48,0.31,0.75,0.42l0.2,1.36c0.02,0.12,0.12,0.21,0.25,0.21h2.07 c0.12,0,0.23-0.09,0.25-0.21l0.2-1.36c0.26-0.11,0.51-0.26,0.74-0.42l1.28,0.5c0.12,0.05,0.25,0,0.31-0.11l1.03-1.79 c0.06-0.11,0.04-0.24-0.06-0.32L22.71,18.43z M19,19.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S19.83,19.5,19,19.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+JcdB74KeG/++/018D7AX/MfA/Ev+27gvfifYxd4CLDLv+ylgb/mBUP8y8z/PK8D/DYv3EsDvwWc4AVD/MvM/zyvA/w2L9hLA78FHAfEC4b4l5n/eV4H+G2ev5cGfgs4zhXiBUP8y8z/PK8D/DbP66WB3wKO82ziBUP8y8z/PK8D/DbP6aWB3wKO85zEC4b4l5n/eV4H+G2e7aWB3wKO87zEC4b4l5n/eV4H+G2ueGngt4DjPH/iBUP8y8z/PK8D/Dbw0sBvAcd5wcQLhviXmf95XgfYBX4LOM4LJ14wxL/M/M/zMcBnAcf5l4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/83wM8NnAMf5l4gVD/MvM/zyvA+wCvw0c44UTLxjiX2b+53kd4LeBlwZ+GzjGCyZeMMS/zPzP8zrAb3PFSwO/DRzj+RMvGOJfZv7neR3gt3m2lwZ+GzjG8xIvGOJfZv7neR3gt3lOLw38NnCM5yReMMS/zPzP8zrAb/O8Xhr4beAYzyZeMMS/zPzP8zrAb/P8vTTw28AxrhAvGOJfZv7neR3gt3nBXhr4beAYIF4wxL/M/M/zOsBv88K9NPDbwHFeMMS/7LuB9+J/jkvAg4Fd/mUvDfw1LxjiX3Yc+Grgvfjv9zfAewN/zX8MxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EdwTUEFLmZwTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideocam;
impl IconShape for MdVideocam {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 10.5V7c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-3.5l4 4v-11l-4 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFOUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/srcGfhvY5f8exAv33sB3AX8NvA6wy/8tiBfsvYHv4tn+GngdYJf/OxDP31sDP8Xz+mvgdYBd/m9APH/Hgd8GXorn9dfA6wC7/O+HeMGOA78NvBTP66+B1wF2+d8N8cIdB34beCme118DrwPs8r8X4l92HPht4KV4Xn8NvA6wy/9OiBfNceC3gZfief018DrALv/7IF50x4HfBl6K5/XXwOsAu/zvgvjXOQ78NvBSPK+/Bl4H2OV/D8S/7DjwVcB78y/7a+B1gF2e13sD38W/z18D7wP8Nf8xEP+y7wbeixfdXwOvA+zyvN4b+C7+fXaBhwC7/MteGvhrXjDEv8z86/018DrALs/rvYHv4t/ndYDf5oV7aeC3gBO8YIh/mfm3+WvgdYBdntd7A9/Fv93rAL/NC/bSwG8BxwHxgiH+Zebf7q+B1wF2eV7vDXwX/zavA/w2z99LA78FHOcK8YIh/mXm3+evgdcBdnle7w18F/96rwP8Ns/rpYHfAo7zbOIFQ/zLzL/fXwOvA+zyvN4b+C7+dV4H+G2e00sDvwUc5zmJFwzxLzP/Mf4aeB1gl+f13sB38aJ7HeC3ebaXBn4LOM7zEi8Y4l9m/uP8NfA6wC7P672B7+JF8zrAb3PFSwO/BRzn+RMvGOJfZv5j/TXwOsAuz+u9ge/iX/Y6wG8DLw38FnCcF0y8YIh/mfmP99fA6wC7PK/3Br6LF+51gF3gt4DjvHDiBUP8y8x/jr8GXgfY5Xm9N/BdvGAfA3wWcJx/mXjBEP8y85/nr4HXAXZ5Xu8NfBf/fuIFQ/zLzH+uvwZeB9jleb038F38+4gXDPEvM//5/hp4HWCX5/XewHfxbydeMMS/zPzX+GvgdYBdntd7A9/Fv414wRD/MvNf56+B1wF2eV7vDXwX/3riBUP8y8x/rb8GXgfY5Xm9N/Bd/OuIFwzxLzP/9f4aeB1gl+f13sB38aITLxjiX2b+e/w18DrALs/rvYHv4kUjXjDEv8z89/lr4HWAXZ7XewPfxb9MvGCIf5n57/XXwOsAuzyv9wa+ixdOvGCIf5n57/fXwOsAuzyv9wa+ixdMvGCIf5n5n+GvgdcBdnle7w18F8+feMEQ/zLzP8dfA68D7PK83hv4Lp6XeMEQ/zLzP8tfA68D7PK83hv4Lp6TeMEQ/zLzP89fA68D7PK83hv4Lp5NvGCIf5n5n+mvgdcBdnle7w18F1eIFwzxLzP/c/018DrALs/rvYHvAsQLhviXmf/Z/hp4HWCX5/XewHfzgiH+Zd8NvBf/s/018DrALv86iH/ZceCrgffiv9/fcMVL8bz+GngdYJcXHeJ/n+PAbwMvxfP6a+B1gF1eNIj/nY4Dvw28FM/rr4HXAXb5lyH+9zoO/DbwUjyvvwZeB9jlhUP873Yc+G3gpXhefw28DrDLC4b43+848NvAS/G8fgd4bV4wxP8Nx4HfBl6KZ7sEvDbw17xgiP87jgO/DbwUcAl4beCveeEQ/7ccB34a+Gjgr/mXIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPN7nNQTJL+dgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVideocamOff;
impl IconShape for MdVideocamOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 6.5l-4 4V7c0-.55-.45-1-1-1H9.82L21 17.18V6.5zM3.27 2L2 3.27 4.73 6H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.21 0 .39-.08.54-.18L19.73 21 21 19.73 3.27 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6bjwC7/+RD/83wX8D3Ab/OfD/E/y3cB7w28DvDb/OdD/M/xXcB7c8XrAL/Nfz7E/wzfBbw3z/Y6wG/znw/x3++7gPfmOb0O8Nv850P89/ou4L15Xq8D/Db/+RD/fb4LeG+ev9cBfpv/fIj/Ht8FvDcv2OsAv81/PsR/ve8C3psX7nWA3+Y/H+K/1ncB782/7HWA3+Y/H+K/zncB782L5nWA3+Y5vTRwHPht/uMg/mt8F/DevOheB/htntNrA78FfDfwPvzHQPzn+2rgo/jXeR3gt3lOrw38Flf8NfAy/Psh/m1+G3gt/vO8DvDbPKfXBn6LZ/tu4H3490H82/w28Fr853kd4Ld5Tq8N/BbP6WOAr+bfDvFv89vAa/Gf53WA3+Y5vTbwWzynXeAhwC7/Noh/m98GXov/PK8D/DbP6bWB3+J5fQzw1fzbIP5tfht4Lf7zvA7w2zyn1wZ+i+f118DL8G+D+Lf5beC1+M/zOsBv85xeG/gtnj/xb4P4t/lt4LX4z/M6wG/znF4b+C2ev9cBfpt/PcS/zW8Dr8V/ntcBfpvn9NrAb/H8vQ7w2/zrIf5tfht4Lf7zvA7w2zyn1wZ+i+fvdYDf5l8P8W/z28Br8Z/ndYDf5jm9NvBbPH+vA/w2/3qIf5vfBl6L/zyvA/w2z+m1gd/i+RP/Noh/m98GXov/PK8D/DbP6bWB3+J5/Q3w0vzbIP5tfht4Lf7zvA7w2zyn1wZ+i+f1McBX82+D+Lf5beC1+M/zOsBv85xeG/gtntMl4MHALv82iH+b3wZei/88rwP8Ns/ptYHf4jm9DfDT/Nsh/m1+G3gt/vO8DvDbPKfXBn6LZ/se4L3590H853tv4Lv413kd4Ld5Tq8N/BZX/Azw1vz7If5rvDfwXbzoXgf4bZ7TawO/BXwN8NH8x0D813lv4Lt40bwO8Ns8pwcDDwZ+m/84iP9a7w18F/+y1wF+m/98iP967w18Fy/c6wC/zX8+xH+P9wa+ixfsdYDf5j8f4r/PewPfxfP3OsBv858P8d/rvYHv4nm9DvDb/OdD/Pd7b+C7eE6vA/w2//kQ/zO8N/BdPNvrAL/Nfz7E/xzvDXwXV7wO8Nv850P8z/LewHcBrwP8Nv/5EP/zvDdwK/Db/OdD/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8I3ed/Qfdg26QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVolumeDown;
impl IconShape for MdVolumeDown {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.5 12c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM5 9v6h4l5 5V4L9 9H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif4/jwC7/sRD/O7w08NnAW/MfC/E/30sDvwX8DfDa/MdC/M/20sBvAceB3wFem/9YiP+5Xhr4LeA4V/wO8Nr8x0L8z/TSwG8Bx3m23wFem/9YiP95Xhr4LeA4z+l3gNfmPxbif5aXBn4LOM7z+h3gtfmPhfif46WB3wKO8/z9DvDa/MdC/M/w0sBvAcd5wX4HeG3+YyH++7008FvAcV643wFem/9YiP9eLw38FnCcf9nvAK/NfyzEf5+XBn4LOM6L5neA1+Y/FuK/x0sDvwUc50X3O8Br8x8L8R/ntYHf4j/P7wCvzX8sxH+c1wZ+i/88vwO8Nv+xEP9xXhv4Lf7z/A7w2vzHQvzHeW3gt/jP8zvAa/MfC/Ef57WB3+I/z+8Ar81/LMR/nNcGfov/PL8DvDb/sRD/cV4b+C3+8/wO8Nr8x0L8x3lt4Lf4z/M7wGvzHwvxH+e1gd/iP8/vAK/NfyzEf5zXBn6L/zy/A7w2/7EQ/3FeG/gt/vP8DvDa/MdC/Md5beC3+M/zO8Br8x8L8R/ntYHf4j/P7wCvzX8sxH+c1wZ+i/88vwO8Nv+xEP9xXhv4Lf7z/A7w2vzHQvzHeW3gt/jP8zvAa/MfC/Hf46WB3waO8aL7HeC1+Y+F+O/z0sBvA8d40fwO8Nr8x0L893pp4LeBY/zLfgd4bf5jIf77vTTw28AxXrjfAV6b/1iI/xleGvht4Bgv2O8Ar81/LMT/HC8N/DZwjOfvd4DX5j8W4n+WlwZ+GzjG8/od4LX5j4X4n+elgd8GjvGcfgd4bf5jIf5nemngt4FjPNvvAK/NfyzE/1wvDfw2cIwrfgd4bf5jIf5ne2ngt4FjwO8Ar81/LMT/fC8N/Dbw18Br8x8L8b/DSwOfDbw1/7EQ/3scB3b5j4X4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/whUI1RB4LSYqQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVolumeMute;
impl IconShape for MdVolumeMute {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 9v6h4l5 5V4l-5 5H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVolumeOff;
impl IconShape for MdVolumeOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVolumeUp;
impl IconShape for MdVolumeUp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5HuC9eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+I5/Q7w2vzXM1f8DvDavGh+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+a/nrnid4DX5kXz28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/9cwVvwO8Ni+a3wZei+f0O8Br85wQz99vA6/Fc/od4LX5r2eu+B3gtXnR/DbwWjyn3wFem+eEeP5+G3gtntPvAK/Nfz1zxe8Ar82L5reB1+I5/Q7w2jwnxPP328Br8Zx+B3ht/uuZK34HeG1eNL8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br81/PXPE7wGvzovlt4LV4Tr8DvDbPCfH8/TbwWjyn3wFem/965orfAV6bF81vA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzXM1f8DvDavGh+G3gtntPvAK/Nc0I8f78NvBbP6XeA1+a/nrnid4DX5kXz28Br8Zx+B3htnhPi+ftt4LV4Tn8NfDT/9X6bK34HeG1eNL8NvBbP6XeA1+Y5IZ6/3wZei/9Zfgd4bV40vw28Fs/pd4DX5jkhnr/fBl6L/1l+B3htXjS/DbwWz+l3gNfmOSGev98GXovn9DfAR/Nf77e44neA1+ZF89vAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2//XMFb8DvDYvmt8GXovn9DvAa/OcEM/fbwOvxXP6HeC1+a9nrvgd4LV50fw28Fo8p98BXpvnhHj+fht4LZ7T7wCvzX89c8XvAK/Ni+a3gdfiOf0O8No8J8Tz99vAa/Gcfgd4bf7rmSt+B3htXjS/DbwWz+l3gNfmOSGev98GXovn9DvAa/Nfz1zxO8Br86L5beC1eE6/A7w2zwnx/P028Fo8p98BXpv/euaK3wFemxfNbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr81zNX/A7w2rxofht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfmv5654neA1+ZF89vAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2//XMFb8DvDYvmt8GXovn9DvAa/OcEM/fbwOvxXP6HeC1+a9nrvgd4LV50fw28Fo8p98BXpvnhHj+vht4L/5v+Rrgo3lOiOfvo4Gv4v+WjwG+mueEeP6OA7cCx/i/4RLwYGCX54R4wd4b+C7+b3gb4Kd5XogX7rWB7wYexP9OzwDeG/htnj/Ei+algeP877IL/DUvHOL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I2v1uEEIIHuIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWeb;
impl IconShape for MdWeb {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-5 14H4v-4h11v4zm0-5H4V9h11v4zm5 5h-4V9h4v9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADsklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeV4fA/w1/7O8NPBVPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZv53Ey8Y4l/218BL8b/T7wCvzQuG+Je9N/Bd/O/0PsB384IhXjTfDbwX/7t8D/DevHCIF91nAx8NHON/tkvAVwOfzb8M8a/32vzP9tu86BD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFfI2xBziKHqgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWebAsset;
impl IconShape for MdWebAsset {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.89-2-2-2zm0 14H5V8h14v10z",
            }
        }
    }
}
