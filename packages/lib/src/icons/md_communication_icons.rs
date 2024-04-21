use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE00lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO/7FeG/gtnpf4t0P8y74beC/+9d4G+Gn+47w28Fs8L/Fvh/iXmX+b7wHem/84rw38Fs9L/Nsh/mXm3+Z3gNfmP85rA7/F8xL/doh/mfm3+R3gtfmP89rAb/G8xL8d4l/218BL8a/3O8Br8x/ntYHf4nmJfzvEv+y3gdfi30b8x3lt4Ld4XuLfDvEv+23gtfi3Ef9xXhv4LZ6X+LdD/Mu+Gvgo/m1eB/htXjSvDfwW//HEC4b4l3028Fn823wM8NW8aF4b+C3+44kXDPEve23gt/i3+RngrXnRvDbwW/zHEy8Y4l/2YODp/NuJF81rA7/FfzzxgiFeNLcCD+Lf5nWA3+Zf9trAb/EfT7xgiBfNdwPvxb/N6wC/zb/sOPDSvGAvDXwVz+t1eOF+mxcM8aJ5b+C7+Nf7G+Cl+Y/x2sBv8bzEvx3iRfNg4On86/wN8NrALv8xXhv4LZ6X+LdDvOhuBR7Ei+ZvgNcGdvmP89rAb/G8xL8d4kX30cBX8S/7G+C1gV3+Y7028Fs8L/Fvh3jRPRh4Oi/c3wCvDezyH++1gd/ieYl/O8S/zm8Dr8Xz9zfAawO7/Od4beC3eF7i3w7xr/PewHfxvJ4BvDSwy3+e1wZ+i+cl/u0Q/3q3Ag/iOe0CLwPcyn+e48BL87x+m387xL/eewPfxfP6HuC9+d8F8W9zK/AgntfLAH/N/x6If5v3Br6L5/XXwMvwvwfi3+63gdfieX0O8Nn874D4t3tp4K94/l4H+G3+50P8+3w18FE8r13gIcAu/7Mh/n2OA78NvBTP66+B1wF2+Z8L8e/30sBvA8d4Xj8NvA3/cyH+Y7w38F08f98NvA//MyH+43w38F48f98NvA//Pq/FFb/DfxzEf6zfBl6L5++ngfcBdvnXOQ78FvDSPNtPA38N/DbwO/zbIf5jHQd+G3gpnr+/Bt4GuJUXzXHgt4CX5l/22zzbrcDPAD/NC4f4j3cc+G3gpXj+doG3AX6bF+448FvAS/Nv9znAZ/OCIf5zHAd+G3gpXrCvBj4H2OV5HQd+C3hp/n1uBR7CC4b4z3Mc+GngtXjBbgXeB/htnu048FvAS/MfQ7xgiP983w28Fy/cdwOfA+wCvwW8NP9xxAuG+K/x3sBXA8d44W4FHsx/LPGCIf7rvDTw3cBL8V9LvGCI/1rHgc8GPor/OuIFQ/z3eGngq4HX4j+feMEQ/73eG/hs4EH85xEvGOJ/hvcGPht4EP/xxAuG+J/lvYG3Bt6K/zjiBUP8z/Rg4K2BjwYexL+PeMEQ//M9GHht4LWB1wYexIvue4D35gVD/O/zYODBwEsDDwZemiuOAy/Fs30P8NHALi8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IWLKNQTCTWRgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddIcCall;
impl IconShape for MdAddIcCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM21 6h-3V3h-2v3h-3v2h3v3h2V8h3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAlternateEmail;
impl IconShape for MdAlternateEmail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10h5v-2h-5c-4.34 0-8-3.66-8-8s3.66-8 8-8 8 3.66 8 8v1.43c0 .79-.71 1.57-1.5 1.57s-1.5-.78-1.5-1.57V12c0-2.76-2.24-5-5-5s-5 2.24-5 5 2.24 5 5 5c1.38 0 2.64-.56 3.54-1.47.65.89 1.77 1.47 2.96 1.47 1.97 0 3.5-1.6 3.5-3.57V12c0-5.52-4.48-10-10-10zm0 13c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAppRegistration;
impl IconShape for MdAppRegistration {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                height: "4",
                width: "4",
                x: "10",
                y: "4",
            }
            rect {
                height: "4",
                width: "4",
                x: "4",
                y: "16",
            }
            rect {
                height: "4",
                width: "4",
                x: "4",
                y: "10",
            }
            rect {
                height: "4",
                width: "4",
                x: "4",
                y: "4",
            }
            polygon {
                points: "14,12.42 14,10 10,10 10,14 12.42,14",
            }
            path {
                d: "M20.88,11.29l-1.17-1.17c-0.16-0.16-0.42-0.16-0.58,0L18.25,11L20,12.75l0.88-0.88C21.04,11.71,21.04,11.45,20.88,11.29z",
            }
            polygon {
                points: "11,18.25 11,20 12.75,20 19.42,13.33 17.67,11.58",
            }
            rect {
                height: "4",
                width: "4",
                x: "16",
                y: "4",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBusiness;
impl IconShape for MdBusiness {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 7V3H2v18h20V7H12zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm10 12h-8v-2h2v-2h-2v-2h2v-2h-2V9h8v10zm-2-8h-2v2h2v-2zm0 4h-2v2h2v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD/ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b150Pw28D7DL/2yIf9lPA2/Fv95vA6/D/2yIf5n5txP/syH+ZebfTvzPhviXmX878T8b4l/228Br8W/zOsBv8z8X4l/228Br8W/zOsBv8z8X4l/228Br8W/zMcBX8z8X4l/21cBH8W/zPcB78z8X4l/20cBX8W/z18DL8D8X4l/22sBv8W93AtjlfybEv+w4cJF/u/cBvpv/mRAvmluBB/Fv8z3Ae/M/E+JF893Ae/Fv8znAZ/M/E+JF89bAT/Gvdwl4aeBW/mdCvGiOAxf513kG8NbAX/M/F+JF99vAa/Gi+RvgtYFd/mdDvOjeG/gu/mXfA7w3/zsgXnTHgYu8cJ8DfDb/eyD+db4beC+ev1uBh/C/C+Jf57WB3+IFex/gu/nfA/Gv99vAa/H83Qq8DLDL/w6If73XBn6LF+xzgM/mfwfEv81vA6/FC/YywF/zPx/i3+a1gd/iBftr4GX4nw/xb/fdwHvxgn038D78z4b4tzsO3Aoc4wV7H+C7+Z8L8e/z0cBX8cK9DPDX/M+E+Pf7aeCteMF2gdcB/pr/eRD/fseBvwYexAu2C7wO8Nf8z4L4j/HSwF/xwu0CrwP8Nf9zIP7jvDfwXbxwu8DbAL/Nv85x4KO44q+B3wF2+fdD/Mf6auCj+Jd9NPA1vGiOA08HjvOcfhv4a+CvgVuBS8Bf86+D+I/33cB78S/7buBjgF1euL8CXpp/ve8GPgbY5QVD/Of4buC9+JfdCrwP8Ns8f98FvDf/dt8DvDcvGOI/z3cD78WL5quBzwF2ebb3Br6Lfz/xgiH+c3018FG8aHaBjwa+B3hp4K/4jyFeMMR/vvcGvosX3a3AceA4/zHEC4b4r/HSwE8DD+K/nnjBEP91jgPfDbwV/7XEC4b4r/fRwGcDx/ivIV4wxH+P48BXA+/Ffz7xgiH+e7028NnAa/GfR7xgiP8ZXhv4bOC1+I8nXjDE/yyvDbw38NbAMf5jiBcM8T/TceCtgfcGXot/u0vAcV4wxP98x4HXBt4aeG3gQbzovgd4b14wxP8+x4GXBl4aOA68Ns/2WjzbzwDvDezygiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj5j2dEHVXUVgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCall;
impl IconShape for MdCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.01 15.38c-1.23 0-2.42-.2-3.53-.56-.35-.12-.74-.03-1.01.24l-1.57 1.97c-2.83-1.35-5.48-3.9-6.89-6.83l1.95-1.66c.27-.28.35-.67.24-1.02-.37-1.11-.56-2.3-.56-3.53 0-.54-.45-.99-.99-.99H4.19C3.65 3 3 3.24 3 3.99 3 13.28 10.73 21 20.01 21c.71 0 .99-.63.99-1.18v-3.45c0-.54-.45-.99-.99-.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4r/WceCleOH+BtjlvwbiP8dx4LWAlwZeGngw8NL86/w1cCvw18BfA78D7PIfC/Ef562A1wZeG3hp/nP8NfDbwG8DP8O/H+Lf56WB9wLeGzjOf61bgZ8Gvgf4a/5tEP827wV8NvBg/mf4a+Crge/hXwfxr/NewGcDD+Z/pluBzwa+hxcN4kXz1sBXAQ/mX+cS8NfAbwO7wF8Du8Bf8/y9NHAceGngOPDawEsDx/jXuRX4GOCneeEQL9yDge8CXpsX3c8Avw38NHAr/zEeDLw18NrAW/Gi+23gfYBbef4QL9hHAV/Nv87vAK/Nf67fBl6Lf52PBr6G54V4XseBnwJem3+93wFem/9cvw28Fv96vw28DbDLsyGe02sDPwUc59/md4DX5j/XbwOvxb/NLvA2wG9zBeLZ3hv4Lv59fgd4bf5z/TbwWvz7vA/w3QDiis8CPpt/v98BXpv/XL8NvBb/fp8NfI6AzwY+i/8YvwO8Nv+5fht4Lf5jfI6Ai8Bx/mP8DvDa/Of6beC1+I9xq4Bd4Bj/MX4HeG3+c/028Fr8x7gk4LOBz+I/xs8Ab81/rt8GXov/GJ8jrvhu4L3493sb4Kf5z/XewHfx7/c9wHuLZ/tu4L34t7kEfDbw1fzX+Gjgs4Fj/Nt8D/DeAOI5fTfwXvzrfA/w0cAu/7UeDHw18Fb863wP8N5cgXhe3w28F/+yvwE+Gvht/nu9NvDVwEvxL/se4L15NsTz993Ae/GCfQ/w3vzPcRz4auC9eMG+B3hvnhPiBftu4L14Xu8DfDf/M3008FU8r+8B3pvnhXjhvhr4KK54BvDewG/zP9trAz8NHOOKrwE+mucP8S97MPBg4Lf53+W1gVuBW3nBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjY6m8Q5iTE7AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallEnd;
impl IconShape for MdCallEnd {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 9c-1.6 0-3.15.25-4.6.72v3.1c0 .39-.23.74-.56.9-.98.49-1.87 1.12-2.66 1.85-.18.18-.43.28-.7.28-.28 0-.53-.11-.71-.29L.29 13.08c-.18-.17-.29-.42-.29-.7 0-.28.11-.53.29-.71C3.34 8.78 7.46 7 12 7s8.66 1.78 11.71 4.67c.18.18.29.43.29.71 0 .28-.11.53-.29.71l-2.48 2.48c-.18.18-.43.29-.71.29-.27 0-.52-.11-.7-.28-.79-.74-1.69-1.36-2.67-1.85-.33-.16-.56-.5-.56-.9v-3.1C15.15 9.25 13.6 9 12 9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q//l+i/8afwN8NP86iP985r/G7wCvzb8O4j+f+a/xO8Br86+D+M9n/mv8DvDa/Osg/vOZ/xq/A7w2/zqI/3y/zb/fSwPHeOF+B3ht/nUQ//O9N/Bd/Mt+B3ht/nUQ/7O9N/BdvGh+B3ht/nUQ/3O9N/BdvOh+B3ht/nUQ/zO9N/BdvGCXgGM8p98BXpt/HcT/PO8NfBcv2PsA7w28Fs/pd4DX5l8H8T/LewPfxQv2PsB3A78NvBbP6XeA1+ZfB/E/x3sD38UL9j7Ad3PFbwOvxXP6HeC1+ddB/M/w3sB38YK9D/DdPNtvA6/Fc/od4LX510H893tv4Lt4wd4H+G6e028Dr8Vz+h3gtfnXQfz3em/gu3jB3gf4bp7XbwOvxXP6HeC1+ddB/Pd5b+C7eMHeB/hunr/fBl6L5/Q7wGvzr4P47/HewHfxgr0P8N28YL8NvBbP6XeA1+ZfB/Ff772B7+IFex/gu3nhfht4LZ7T7wCvzb8O4r/WewPfxQv2PsB38y/7beC1eE6/A7w2/zqI/zrvDXwXL9j7AN/Ni+a3gdfiOf0O8Nr86yD+a7w38F28YO8DfDcvut8GXovn9DvAa/Ovg/jP997Ad/GCvQ/w3fzr/DbwWjyn3wFem38dxH+u9wa+ixfsfYDv5l/vt4HX4jn9DvDa/Osg/vO8N/BdvGDvA3w3/za/DbwWz+l3gNfmXwfxn+O9ge/iBXsf4Lv5t/tt4LV4Tr8DvDb/Ooj/eO8NfBcv2PsA382/z28Dr8Vz+h3gtfnXQfzHem/gu3jB3gf4bv7nQPzHeW/gu3jB3gf4bv5nQfzHeG/gu3jB3gf4bv7nQfz7vTfwXbxg7wN8N/8zIf593hv4Ll6w9wG+m/+5EP927w18Fy/Y+wDfzf9siH+b9wa+ixfsfYDv5n8+xL/eewPfxQv2PsB3878D4l/nvYHv4gV7H+C7+d8D8aJ7MPB0XrD3Ab6b/10Q/zrvDXwXz+t9gO/mfx/Ev957A9/Fs70P8N3874T4t3lv4LuA9wG+m/+9EP92DwZu5X83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHrlBzQb3SJZ4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallMade;
impl IconShape for MdCallMade {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 5v2h6.59L4 18.59 5.41 20 17 8.41V15h2V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+ulueKv+e+B+O/z0sBvccXrAH/Nfz3Ef4+XBn4LOM4Vu8DrAH/Nfy3Ef72XBn4LOM5z2gVeB/hr/usg/mu9NPBbwHGev13gdYC/5r8G4r/OSwO/BRznhdsFXgf4a/7zIf5rvDTwW8BxXjS7wOsAf81/LsR/vpcGfgs4zr/OLvA6wF/znwfxn+ulgd8CjvNvswu8DvDX/OdA/Od5aeC3gOP8++wCrwP8Nf/xEP85Xhr4LeA4/zF2gdcB/pr/WIj/eC8N/BZwnP9Yu8DrAH/NfxzEf7yXBo7zwn018FI8p78BPpoXbhf4a/7jIP57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/eceCvgAfznH4HeG3+ayH+ax0Hfgt4aZ7X7wCvzX8txH+d48BvAS/N8/c1wEfzXwvxX+M48FvAS/P8fQ/w3vzXQ/znOw78FvDSPH/fA7w3/z0Q/7mOA78FvDTP3/cA780LdpwrdvnPgfjPcxz4LeClef6+B3hvXrDjwG9xxesAu/zHQ/znOA78FvDSPH/fA7w3L9hx4LeAl+aKvwZeB9jlPxbiP95x4LeAl+b5+x7gvXnBjgO/Bbw0z+mvgdcBdvmPg/iPdRz4LeClef6+B3hvXrjPBj6L5++vgdcBdvmPgfiPcxz4LeClef6+B3hvXjTfDbwXz99fA68D7PLvh/iPcRz4LeClef6+B3hv/nW+G3gvnr+/Bl4H2OXfB/Hvdxz4LeClef6+B3hv/m2+G3gvnr+/Bl4H2OXfDvHvcxz4LeClef6+B3hv/n2+G3gvnr+/Bl4H2OXfBvHv81fAS/P8fQ/w3vzH+G7gvXj+/hp4Gf5tEP8+7w18F8/re4D35j/WdwPvxfN6H+C7+bdB/Pu9N/BdPNv3AO/Nf47vBt6LZ3sf4Lv5t0P8x3hv4LuA7wHem/9c3w28F/A+wHfz74P4j/PawG/zX+O1gd/m3w/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wj6NYdBgNa3aQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallMerge;
impl IconShape for MdCallMerge {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 20.41L18.41 19 15 15.59 13.59 17 17 20.41zM7.5 8H11v5.59L5.59 19 7 20.41l6-6V8h3.5L12 3.5 7.5 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEG0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/2WvzH+xtgl/8cDwZu5UWD+JeZ/3ivA/w2//G+C3hr4HWAv+ZfhviXmf94rwP8Nv+xvgt4b67YBV4H+GteOMS/zPzHex3gt/mP813Ae/OcdoHXAf6aFwzxLzP/8V4H+G3+Y3wX8N48f78DvDYvGOJfZv7jvQ7w2/z7fRfw3jx/fwO8NrDLC4b4l5l/2XcD78N/re8C3pvn72+A1wZ2eeEQ/zLzovlu4H34r/FdwHvz/P0N8NrALv8yxL/MvOi+G3gf/nN9F/DePH9/A7w2sMuLBvEvM8/rEnCM5++7gffhP8d3Ae/N8/c3wGsDu7zoEP8y87zeBvhs4KV4/r4beB/+Y30X8N48f38DvDawy78O4l9mntfrAH8N/DbwUjx/3w28D/8xvgt4b56/vwFeG9jlXw/xLzPP63WA3waOA78NvBTP33cD78O/z3cB783z9zfAawO7/Nsg/mXmeb0O8NtccRz4beCleP6+G3gf/m2+C3hvnr+/AV4b2OXfDvEvM8/rdYDf5tmOA78NvBTP33cD78O/zncB783z9zfAawO7/Psg/mXmeb0O8Ns8p+PAbwMvxfP33cD78KL5LuC9ef7+BnhtYJd/P8S/zDyv1wF+m+d1HPht4KV4/r4beB9euO8C3pvn72+A1wZ2+Y+B+JeZ5/U6wG/z/B0Hfht4KZ6/7wbeh+fvu4D35vn7G+C1gV3+4yD+ZeZ5vQ7w27xgx4HfBl6K5++7gffhOX0X8N48f38DvDawy38sxL/MPK/XAX6bF+448NvAS/H8fTfwPlzxXcB78/z9DfDawC7/8RD/MvO8Xgf4bf5lx4HfBl6K5++7ueK9ef7+BnhtYJf/HIh/mXlerwP8Ni+a48BvAy/Fv87fAK8N7PKfB/Ff4zjw28BL8aL5G+C1gV3+cyH+6xwHfht4KV64vwFeG9jlPx/iv9Zx4LeBl+L5+xvgtYFd/msg/usdB34beCme098Arw3s8l8H8d/jOPDbwEtxxd8Arw3s8l8L8d/nOPDbXPHawC7/9RD/vY5zxS7/PRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8xKJFB3P6D1QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallMissed;
impl IconShape for MdCallMissed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.59 7L12 14.59 6.41 9H11V7H3v8h2v-4.59l7 7 9-9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEG0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/nwcCt/Oc4DrwU//F+hxcM8aJ7aeC3gJ8G3of/eK8N/Bb/8cQLhnjRvDTwW8Bxrvhu4H34j/XawG/xH0+8YIh/2UsDvwUc5zl9N/A+/Md5beC3+I8nXjDEv+y3gdfi+ftu4H34j/HawG/xH0+8YIh/2XHgt4GX4vn7buB9+Pd7beC3+I8nXjDEi+Y48NvAS/H8fTfwPvzX+i7gvfmXiRcM8aI7Dvw28FI8f98NvA//Nb4LeG9eNOIFQ/zrHAd+G3gpnr/vBt6H/1zfBbw3LzrxgiH+9Y4Dvw28FM/fdwPvw3+O7wLem+fvEnCM5yVeMMS/zXHgt4GX4vn7buB9+I/1XcB78/z9DfDZwE/xvMQLhvi3Ow78NvBSPH/fDbwP/zG+C3hvnr+/AV4beGngt3he4gVD/PscB34beCmev+8G3od/n+8C3pvn72+A1wZ2gdcGfovnJV4wxL/fceC3gZfi+ftu4H34t/ku4L15/v4GeG1glyteG/gtnpd4wRD/MY4Dvw28FM/fdwPvw7/OdwHvzfP3N8BrA7s822sDv8XzEi8Y4j/OceC3gZfi+ftu4H140XwX8N48f38DvDawy3N6beC3eF7iBUP8xzoO/DbwUjx/3w28Dy/cdwHvzfP3N8BrA7s8r9cGfovnJV4wxH+848BvAy/F8/fdwPvw/H0X8N48f38DvDawy/P32sBv8bzEC4b4z3Ec+G3gpXj+vht4H57TdwHvzfP3N8BrA7u8YK8N/BbPS7xgiP88x4HfBl6K5++7gffhiu8C3pvn72+A1wZ2eeFeG/gtnpd4wRD/uY4Dvw28FM/fd3PFe/P8/Q3w2sAu/7LXBn6L5yVeMMR/vuPAbwMvxb/O3wCvDezyonlt4Ld4XuIFQ/zXOA78NvBSvGj+BnhtYJf/XIj/OseB3wZeihfub4DXBnb5z4f4r3Uc+G3gpXj+/gZ4bWCX/xqI/3rHgd8GXorn9DfAawO7/NdB/Pc4Dvw28FJc8TfAawO7/NdC/Pc5Dvw2V7w2sMt/PcR/r+Ncsct/D8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COPO3NBE4r1PgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallMissedOutgoing;
impl IconShape for MdCallMissedOutgoing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3,8.41l9,9l7-7V15h2V7h-8v2h4.59L12,14.59L4.41,7L3,8.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD90lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/4YHA7fyr4f43++9ge8C3gf4bv51EP+7vTfwXTzb+wDfzYsO8b/XewPfxfN6H+C7edEg/nd6b+C7eMEeAtzKvwzxv897A9/FC/Y+wHfzokH87/LewHfxgr0P8N286BD/e7w38F28YO8DfDf/Ooj/Hd4b+C5esPcBvpt/PcT/fO8NfBcv2PsA382/DeJ/tvcGvosX7H2A7+bfDvE/13sD38UL9j7Ad/Pvg/if6b2B7+IFex/gu/n3Q/zP897Ad/GCvQ/w3fzHQPzb/DbwWjyn3wFem3+f9wa+ixfsfYDv5j8O4t/mt4HX4jn9DvDa/Nu9N/BdvGDvA3w3/7EQ/za/DbwWz+l3gNfm3+a9ge/iBXsf4Lv5j4f4t/lt4LV4Tr8DvDb/eu8NfBcv2PsA381/DsS/zW8Dr8Vz+h3gtfnXeW/gu3jB3gf4bv7zIP5tfht4LZ7T7wCvzYvuvYHv4gV7H+C7+c+F+Lf5beC1eE6/A7w2L5r3Br6LF+x9gO/mPx/i3+a3gdfiOf0O8Nr8y94b+C5esPcBvpv/Goh/m98GXovn9DvAa/PCvTfwXbxg7wN8N/91EP82vw28Fs/pd4DX5gV7b+C7eMHeB/hu/msh/m1+G3gtntPvAK/N8/fewHfxgr0P8N3810P82/w28Fo8p98BXpvn9d7Ad/GCvQ/w3fz3QPzb/DbwWjyn3wFem+f03sB38YK9D/Dd/PdB/Nv8NvBaPKffAV6bZ3tv4Lt4wd4H+G7+eyH+bX4beC2e0+8Ar80V7w18Fy/Y+wDfzX8/xL/NbwOvxXP6HeC1gfcGvosX7H2A7+Z/BsS/zW8Dr8Vz+h3gu4Hv4gV7H+C7+Z8D8W/z28Br8Zx2geO8YO8DfDf/syD+bX4beC1edO8DfDf/8yD+bX4beC1eNO8DfDf/MyH+bX4beC3+Ze8DfDf/cyH+bX4beC1euF3gr/n3ex3+8yD+bX4beC3+a4j/PIh/m98GXov/GuI/D+Lf5reB1+K/hvjPg/i3+W3gtfivIf7zIP5tvhp4af5rvDb/eRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjE3HBBJhYzYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallReceived;
impl IconShape for MdCallReceived {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 5.41L18.59 4 7 15.59V9H5v10h10v-2H8.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE30lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXcceCn+7Z4B3MqL5sHAg/i3+xtgl38Z4kX32sBv8W/zN8BrA7u8aI4Dvw28FP82rwP8Nv8yxIvutYHf4l/vb4DXBnb51zkO/DbwUvzrvQ7w2/zLEC+61wZ+i3+dvwFeG9jl3+Y48NvAS/Gv8zrAb/MvQ7zoXhv4LV50fwO8NrDLv89x4LeBl+JF9zrAb/MvQ7zoXhv4LV40fwO8NrDLf4zjwG8DL8WL5nWA3+ZfhnjRvTbwW/zLngG8NLDLf6zjwF8DD+Jf9jrAb/MvQ7zoXhv4LV407wN8N/+x3hv4Ll40rwP8Nv8yxIvutYHf4kX3PsB38x/jvYHv4kX3OsBv8y9DvOheG/gt/nXeB/hu/n3eG/gu/nVeB/ht/mWIF91rA7/F83oG8CBesPcBvpt/m/cGvosX7BnAg3herwP8Nv8yxIvutYHf4jn9DfDawFsD38UL9j7Ad/Ov897Ad/GCvQ/w08BvAy/Fc3od4Lf5lyFedK8N/BbP9jfAawO7XPHewHfxgr0P8N28aN4b+C5esPcBvpsrjgO/DbwUz/Y6wG/zL0O86F4b+C2u+BvgtYFdntN7A9/FC/Y+wHfzwr038F28YO8DfDfP6Tjw28BLccXrAL/Nvwzxontt4LeAvwFeG9jl+Xtv4Lt4wd4H+G6ev+PArcAxnr/3Ab6b5+848NvASwGvA/w2/zLEi+61ga8GXhvY5YV7b+C7eMHeB/hunr+XBn4bOMZzeh/gu3nhjgO/DXw08Nv8yxAvugcDu8AuL5r3Br6LF+x9gO/m+Xtp4LeBY1zxPsB386I5DhwHbuVfhvjP9d7Ad/GCvQ/w3Tx/Lw38NvDRwHfznwPxn++9ge/iBXsf4Lt5/o4Du/znQfzXeG/gu3jB3gf4bv7rIf7rvDfwXbxg7wN8N/+1EP+13hv4Lp6/3wFem/9aiP967w18F8/rd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem/9aiP8evw28Fs/pd4DX5r8W4r/HbwOvxXP6HeC1+a+F+O/x28Br8Zx+B3ht/msh/nv8NvBaPKffAV6b/1qI/x6/DbwWz+l3gNfmvxbiv8dvA6/Fc/od4LX5r4X47/HbwGvxnH4HeG3+ayH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x0sDx3lOu8Bf818L8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4Rg3KfQdXjXpoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCallSplit;
impl IconShape for MdCallSplit {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14 4l2.29 2.29-2.88 2.88 1.42 1.42 2.88-2.88L20 10V4zm-4 0H4v6l2.29-2.29 4.71 4.7V20h2v-8.41l-5.29-5.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf59Xhp4aeC7+dd5b+Cvgb/m38c8r9cBfhtAPNtrA7/F8xL/di8N/BZwHHgf4Lt50bw38F3ALvA6wF/zb2ee1+sAvw0gnu21gd/ieYl/m5cGfgs4zrO9D/DdvHDvDXwXz7YLvA7w1/zbmOf1OsBvA4hne23gt3he4l/vpYHfAo7zvN4H+G6ev/cGvovntQu8DvDX/OuZ5/U6wG8DiGd7beC3eF7iX++9ge/iBXsf4Lt5Tu8NfBcv2PsA382/nnlerwP8NoB4ttcGfovnJf5t3hv4Ll6w9wG+myveG/guXrD3Ab6bfxvzvF4H+G0A8WyvDfwWz0v827038F28YO/DFd/FC/Y+wHfzb2ee1+sAvw0gnu21gd/ieYl/n/cGvot/m/cBvpt/H/O8Xgf4bQDxbK8N/BbPS/z7vTfwXfzrvA/w3fz7mef1OsBvA4hne23gt3he4j/GewPfxYvmfYDv5j+GeV6vA/w2gHi21wZ+i+cl/uO8N/BdvHDvA3w3/3HM83od4LcBxLO9NvBbPC/xH+e9ge/ihXsf4Lv5j2Oe1+sAvw0gnu21gd/ieYn/GO8NfBcvmvcBvpv/GOZ5vQ7w2wDi2V4b+C2el/j3e2/gu/jXeR/gu/n3M8/rdYDfBhDP9trAb/G8xL/PewPfxb/N+wDfzb+PeV6vA/w2gHi21wZ+i+cl/u3eG/guXrD34Yrv4gV7H+C7+bczz+t1gN8GEM/22sBv8bzEv817A9/FC/Y+wHdzxXsD38UL9j7Ad/NvY57X6wC/DSCe7bWB3+J5iX+99wa+ixfsfYDv5jm9N/BdvGDvA3w3/3rmeb0O8NsA4tleG/gtnpf413tp4LeBYzyv9wG+m+fvvYHv4nldAl4b+Gv+9czzeh3gtwHEs7028Fs8L/Fv89LAbwPHeLb3Ab6bF+69ge/i2S4Brw38Nf825nm9DvDbAOLZXhv4LZ6X+Ld7aeC3gWPA+wDfzYvmvYHvAi4Brw38Nf925nm9DvDbAOLZXhv4LZ6X+Pd5aeClge/mX+e9gb8G/pp/H/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/Bb/P7wO8NsA4jntAsf4v+0ScJwrEM/ps4HP4v+2zwE+mysQz+u7gffi/6bvAd6bZ0M8f+8NfDTwUvzf8DvAdwPfzXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8B8tjSQaUBaZoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCancelPresentation;
impl IconShape for MdCancelPresentation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 19.1H3V5h18v14.1zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
            path {
                d: "M21 19.1H3V5h18v14.1zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
            path {
                d: "M14.59 8L12 10.59 9.41 8 8 9.41 10.59 12 8 14.59 9.41 16 12 13.41 14.59 16 16 14.59 13.41 12 16 9.41z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdCellWifi;
impl IconShape for MdCellWifi {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,9.98L6,22h12h4V5.97L18,9.98z M20,20h-2v-7.22l2-2V20z M5.22,7.22L3.93,5.93c3.9-3.91,10.24-3.91,14.15,0l-1.29,1.29 C13.6,4.03,8.41,4.03,5.22,7.22z M12.93,11.07L11,13l-1.93-1.93C10.14,10.01,11.86,10.01,12.93,11.07z M14.22,9.79 c-1.78-1.77-4.66-1.77-6.43,0L6.5,8.5c2.48-2.48,6.52-2.48,9,0L14.22,9.79z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fq8aMR/L/Oi+R3gtXlOiOfvt4HX4kUj/nuZF83vAK/Nc0I8f78NvBYvmt/mv9dr86L5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+JFI/57mRfN7wCvzXNCPH+/DbwWLxrx38u8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxovlt/nu9Ni+a3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC1eNOI/3m8Dr8V/rN8BXpvnhHj+fht4LV404j/ebwOvxX+s3wFem+eEeP5+G3gtXjTiP95vA6/Ff6zfAV6b54R4/n4beC1eNOI/3m8Dr8V/rN8BXpvnhHj+fht4LV404j/ebwOvxX+s3wFem+eEeP5+G3gtXjS/zX+8lwaO8x/rd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/lc4DP5jkhnr/fBl6L/1veBvhpnhPi+ftt4LX4v+NvgJfmeSGev98GXov/G/4GeGvgVp4X4vn7beC1+N/tGcB3A18N7PL8IZ6/3wZei3/Z+wDfzf9eiOfvt4HX4oV7H+C7+d8N8fz9NvBavGDvA3w3//shnr/fBl6L5+99gO/m/wbE8/fbwGvxvN4H+G7+70A8f78NvBbP6X2A7+b/FsTz99vAa/Fs7wN8N//3IJ6/3wZeiyveB/hu/m9CPH+/DbwW8D7Ad/N/F+L5+23gu4Hv5v82xPP30sBf838f4v83xP9viP/fEP+/If5/4x8B9BGCQZlK3j8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChat;
impl IconShape for MdChat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 9h12v2H6V9zm8 5H6v-2h8v2zm4-6H6V6h12v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD00lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5WeAt+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1s+B/hsnhPi+ftt4LX4v+VtgJ/mOSGev98GXov/O/4GeGmeF+L5+23gtfi/4W+AtwZu5Xkhnr/fBl6L/92eAXw38NXALs8f4vn7beC1+Je9D/Dd/O+FeP5+G3gtXrj3Ab6b/90Qz99vA6/FC/Y+wHfzvx/i+ftt4LV4/t4H+G7+b0A8f78NvBbP632A7+b/DsTz99vAa/Gc3gf4bv5vQTx/vw28Fs/2PsB3838P4vn7beC1uOJ9gO/m/ybE8/fbwGsB7wN8N/93IZ6/3wa+G/hu/m9DPH8vDfw1//ch/n9D/P+G+P8N8f8b4v83/hHqE3BBXApqNAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChatBubble;
impl IconShape for MdChatBubble {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrXhOl4C/5n+HlwaO8Zx+BnhrnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8Nr87/DbwGvxnH4HeG2eE+L5+23gtXhOvwO8Nv87/DbwWjyn3wFem+eEeP5+G3gtntPvAK/N/w6/DbwWz+l3gNfmOSGev98GXovn9DvAa/O/w28Dr8Vz+h3gtXlOiOfvt4HX4jn9DvDa/O/w28Br8Zx+B3htnhPi+ftt4LV4Tr8DvDb/O/w28Fo8p98BXpvnhHj+fht4LZ7T7wCvzf8Ovw28Fs/pd4DX5jkhnr/fBl6L5/Q7wGvzv8NvA6/Fc/od4LV5Tojn77eB1+I5/Q7w2vzv8NvAa/Gcfgd4bZ4T4vn7beC1eE6/A7w2/zv8NvBaPKffAV6b54R4/n4beC2e0+8Ar83/Dr8NvBbP6XeA1+Y5IZ6/3wZei+f0O8Br87/DbwOvxXP6HeC1eU6I5++3gdfiOf0O8No8p+PAS/E/z1cDL81z+h3gtXlOiOfvt4HX4jn9DvDaPNtx4LeAl+Z/h88BPpvnhHj+fht4LZ7T7wCvzRXHgd8CXpr/Pd4G+GmeE+L5+23gtXhOvwO8NnAc+C3gpfnf42+Al+Z5IZ6/3wZei+f0O8BbA78FvDT/e/wN8NbArTwvxPP328Br8Zz+BjDw0vzv8Azgu4GvBnZ5/hDP328Dr8W/7H2A7+Z/L8Tz99vAa/HCvQ/w3fzvhnj+fht4LV6w9wG+m//9EM/fbwOvxfP3PsB3838D4vn7beC1eF7vA3w3/3cgnr/fBl6L5/Q+wHfzfwvi+ftt4LV4tvcBvpv/exDP328Dr8UV7wN8N/83IZ6/3wZeC3gf4Lv5vwvx/P028N3Ad/N/G+L5e2ngr/m/D/H/G+L/N8T/b4j/3xD/v/GPCqS8QZD8DVoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdChatBubbleOutline;
impl IconShape for MdChatBubbleOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H6l-2 2V4h16v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACGElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4j/OceCl+J/nd3jBEP9xXhv4Lf7nES8Y4j/OawO/xf884gVD/Md5beC3+J9HvGCI/zivDfwW//OIFwzxH+e1gd/ifx7xgiH+f0P8/4b4/w3x/xvi/zfEv81XAy/F/yx/A3w0/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5auCl+Z/lr4GP5l8H8f8b4v83xP9viP/fEP+/If5l5n+e1wF+m38/xL/M/M/zOsBv8++H+JeZ/3leB/ht/v0Q/zLzP8/rAL/Nvx/iX2b+53kd4Lf590P8y16b/3n+Gtjl3w/x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RvsoeQW9YPD4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdClearAll;
impl IconShape for MdClearAll {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 13h14v-2H5v2zm-2 4h14v-2H3v2zM7 7v2h14V7H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5XuA9+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fq8aH6b/16vzYvmd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXosXjfjvZV40vwO8Ns8J8fz9NvBavGjEfy/zovkd4LV5Tojn77eB1+JFI/57mRfN7wCvzXNCPH+/DbwWLxrx38u8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi+a3+e/12rxofgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LV40Yj/XuZF8zvAa/OcEM/fbwOvxYtG/PcyL5rfAV6b54R4/n4beC1eNOK/l3nR/A7w2jwnxPP328Br8aIR/73Mi+Z3gNfmOSGev98GXosXjfjvZV40vwO8Ns8J8fz9NvBavGh+m/9er82L5neA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+2zgs/i/5XeA1+Y5IZ6/twZ+iv9bfgd4bZ4T4gX7a+Cl+L/jd4DX5jkhXrAHAz8NvBT/N/wO8No8J8QLdxz4aOC9gQfxv9vvAK/Nc0L87/fewHfxL/sd4LV5Toj/G94b+C5euN8BXpvnhPi/472B7+IF+x3gtXlOiP9b3hv4Lp6/3wFem+eE+L/nvYHv4nn9DvDaPCfE/03vDXwXz+l3gNfmOSH+73pv4Lt4tt8BXpvnhPi/7b2B7+KK3wFem+eE+L/vvYHvAn4HeG2eE+L/h/cG3ht4bZ4T4v+Plwb+mueE+P8N8f8b4v83xP9viP/f+EeUIX5BaWPIxAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdComment;
impl IconShape for MdComment {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.99 4c0-1.1-.89-2-1.99-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18zM18 14H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEuklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4tgcDnwW8NXCc/5t2gZ8GPge4FUBc8d7Ad/H/y/sA3y3gwcDT+f/pIQK+G3gv/n/6HgEXgeP8/7QrwPz/hQDz/xcCzP9fCDD/fpeA7wZ+Gvhtrngw8NLAWwPvxb/O6wC/zX+s1wZ+i+eEAPPv8z3ARwO7vGCvDXw38CBeNK8D/Db/sV4b+C2eEwLMv933AO/Ni+Y48NvAS/Evex3gt7niOLDLv81xYJcrXhv4LZ4TAsy/ze8Ar82/zoOBvwaO8cK9DvDbXHEReB/gp/nXeWvgu4ATXPHawG/xnBBg/m0eAtzKv95nA5/FC/c6wG9zhbnip4H3AXZ54Y4D3wW8NVeIK14b+C2eEwLMv97fAC/Nv82Dgafzwr0O8Ntc8dPAW3HFLvA+wE/z/L018F3Aca74GeCtueK1gd/iOSHA/Ot9DvDZ/Nv9NfBSvGCvA/w2z/bWwHcDx7jip4H3AXa54jjwXcBbc8Ul4L2Bn+bZXhv4LZ4TAsy/3vsA382/3W8Dr8UL9jrAb/OcjgPfDbwVV+wC78MV3wUc54qfAd4b2OU5vTbwWzwnBJh/vfcBvpt/u98GXosX7HWA3+b5e2vgu4FjPKdLwHsDP83z99rAb/GcEGD+9T4H+Gz+7f4KeGlesNcBfpsX7Djw3cBbccXPAO8N7PKCvTbwWzwnBJh/vb8GXoZ/mwcDT+eFex3gt/mXvTVX/DT/stcGfovnhADzb/MQ4Fb+9T4b+CxeuNcBfpv/WK8N/BbPCQHm3+a3gdfhX+fBwF8Bx3nhXgf4bf5jvTbwWzwnBJh/u+8G3ocXzXHgt4CX5l/2OsBv8x/rtYHf4jkhwPz7fDfwMcAuL9hrA98FPJgXzesAv81/rNcGfovnhADz77cLfDfw08DvcMWDgdcCXht4b/51Xgf4bf5jvTbwWzwnBJj/vxBg/v9CgPn/CwHm/y8EmP+/EGD+7Z4B/DZwK/DbXPHXwC7w0sBx4Djw0sBLA68NHON/DgSYf52/Ab4b+GngVv71Xhp4b+CtgQfx3wsB5kXzPcBnA7fyH+etgY8GXov/HggwL9zvAO8N3Mp/ntcGfho4xn8tBJgX7HOAz+a/xnHgp4HX4r8OAszz9znAZ/Nf6zjw28BL8V8DAeZ5/Q3w0vz3eGngr/ivgQDzvL4G+Gj++9wKPIj/fAgwz+ttgJ/mv89vA6/Ffz4EmOf1OsBv89/nt4HX4j8fAszzeh3gt/nv89vAa/GfDwHm/y8EmP+/EGD+/0LALnCM/5+eIeC7gffi/6fvEfBg4On8/3MJeGlxxXsD38X/L+8DfLd4tgcDnw28NXCM/5suAT8NfDZwK4D4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLeScBqISrLbQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContactMail;
impl IconShape for MdContactMail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M21 8V7l-3 2-3-2v1l3 2 3-2zm1-5H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1zm8-6h-8V6h8v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE8klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4tgcDnwW8NXCc/5t2gZ8GPge4FUBc8d7Ad/H/y/sA3y3gwcDT+f/pIQK+G3gv/n/6HgEXgeP8/7QrwPz/hQDz/xcCzP9fCDD/fpeA7wZ+Gvhtrngw8NLAWwPvxX+uS8BbA18NvBQvOgSYf5/vAT4a2OUFe23gu4EH8R/vb4D3Bv4aOA78NvBSvGgQYP7tvgd4b140x4HfBl6K/ziXgAcDuzzbceC3gZfiX4YA82/zO8Br86/zYOCvgWP8+10CXhv4a57XceC3gZfihUOA+bd5CHAr/3qfDXwW/37vA3w3L9hx4LeBl+IFQ4D51/sb4KX5t3kw8HT+fX4HeG3+Za8N/BYvGALMv97nAJ/Nv91fAy/Fv91DgFt54Y4DvwW8NC8YAsy/3vsA382/3W8Dr8W/zfcA782/7LuA9+aFQ4D513sf4Lv5t/tt4LX4t3kZ4K954d4b+C7+ZQgw/3qfA3w2/3Z/Bbw0/3rPAB7MC/fSwG8Bx/mXIcD86/018DL82zwYeDr/Nh8DfDXP9mDgQTynrwZemhcNAsy/zUOAW/nX+2zgs/i3eR3gt3m27wbei387BJh/m98GXod/nQcDfwUc599GPKenAw/m3w4B5t/uu4H34UVzHPgt4KX5t/kb4KV5tuPARf59EGD+fb4b+BhglxfstYHvAh7Mv93vAK/Ns7028Fv8+yDA/PvtAt8N/DTwO1zxYOC1gNcG3pt/v98BXptne23gt/j3QYD53+F3gNfm2V4b+C3+fRBg/nf4HeC1ebbXBn6Lfx8EmP8d/hp4GZ7twcDT+fdBgPnfQzynW4EH8W+HAPO/x+sAv82zfTfwXvzbIcD82z0D+G3gVuC3ueKvgV3gpYHjwHHgpYGXBl4bOMa/3ccAX82zPRh4MM/pq4GX4kWDAPOv8zfAdwM/DdzKv95LA+8NvDXwIP51bgUewgv30sBvA8f4lyHAvGi+B/hs4Fb+47w18NHAa/Giexngr3nh3hv4Lv5lCDAv3O8A7w3cyn+e1wZ+GjjGv+x7gPfmX/bdwHvxwiHAvGCfA3w2/zWOAz8NvBb/spcB/poX7jjw28BL8YIhwDx/nwN8Nv+1jgO/DbwUL9xvA6/Dv+y1gd/iBUOAeV5/A7w0/z1eGvgr/mXvA3w3L9hx4LeAl+YFQ4B5Xl8DfDT/fW4FHsS/7GWAv+Z5HQd+C3hpXjgEmOf1NsBP89/nt4HX4l+2CzwE2OXZjgO/Bbw0/zIEmOf1OsBv89/nt4HX4kVzK/A2wF8Dx4HfAl6aFw0CzPN6HeC3+e/z28Br8aLbBd4G+CrgpXnRIcD8/4UA8/8XAsz/XwjYBY7x/9MzBHw38F78//Q9Ah4MPJ3/fy4BLy2ueG/gu/j/5X2A7xbP9mDgs4G3Bo7xf9Ml4KeBzwZuBRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4RzrZw2o99hGhAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContactPhone;
impl IconShape for MdContactPhone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 3H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1zm3.85-4h1.64L21 16l-1.99 1.99c-1.31-.98-2.28-2.38-2.73-3.99-.18-.64-.28-1.31-.28-2s.1-1.36.28-2c.45-1.62 1.42-3.01 2.73-3.99L21 8l-1.51 2h-1.64c-.22.63-.35 1.3-.35 2s.13 1.37.35 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvuOPDS/O/w18Au/zLE/2+I/98Q/78h/n9DvGheGjjG/y6XgL/mhUO8cK8NfBfwYP53uhV4H+C3ef4QL9h7A9/F/w3vA3w3zwvx/B0Hng4c5/+GXeAhwC7PCfH8fTTwVfzf8jHAV/OcEM/fdwPvxf8t3wO8N88J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr85/ke4KuBv+aKlwY+Gngv/vP8DvDaPCfE8/fbwGvxH+8S8NrAX/P8vTTw28Ax/uP9DvDaPCfE8/fbwGvxH+9lgL/mhXtp4K/4j/c7wGvznBDP328Dr8V/rK8BPpoXzXcD78V/rN8BXpvnhHj+fht4Lf5jvQzw17xoXhr4K/5j/Q7w2jwnxPP328Br8R9L/OuY/1i/A7w2zwnx/P028Fr8xxL/OuY/1u8Ar81zQjx/vw28Fv+xXgb4a140Lw38Ff+xfgd4bZ4T4vn7beC1+I/1PcB786L5buC9+I/1O8Br85wQz99vA6/Ff7yXAf6aF+6lgb/iP97vAK/Nc0I8f78NvBb/8XaB1wH+mufvpYHfAo7zH+93gNfmOSGev98GXov/PN8NfA3w11zx0sBHAe/Nf57fAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/vM8A7iV5/Rg4EH85/kd4LV5Tojn77eB1+I/zs8Avw38NvDXvHAvDbw28NrAW/Ef53eA1+Y5IZ6/3wZei3+fZwCfDfw0sMu/zXHgrYHPBh7Ev8/vAK/Nc0I8f78NvBb/NpeAzwa+mv9YHw18NnCMf5vfAV6b54R4/n4beC3+9S4Brw38Nf85Xhr4beAY/3q/A7w2zwnx/P028Fr8670O8Nv853pt4Lf41/sd4LV5Tojn77eB1+Jf5xnAg/mvcSvwIP51fgd4bZ4T4vn7beC1+Nf5HeC1+a/x28Br8a/zO8Br85wQz99vA6/Fv87vAK/Nf43fBl6Lf53fAV6b54R4/n4beC3+dX4HeG3+a/w28Fr86/wO8No8J8Tz99vAa/Gv8zvAa/Nf47eB1+Jf53eA1+Y5IZ6/3wZei3+dXeCv+a/x0sBx/nV+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+27gvfi/5WuAj+Y5IZ6/jwa+iv9bPgb4ap4T4vk7DtwKHOP/hkvAg4FdnhPiBXtv4Lv4v+FtgJ/meSFeuNcGvht4EP87PQN4b+C3ef4QL5qXBo7zv8su8Ne8cIj/3xD/vyH+f0P8/4Z40R0HXor/Hf4G2OVfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8y/hFbiqxBfu56BgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdContacts;
impl IconShape for MdContacts {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 0H4v2h16V0zM4 24h16v-2H4v2zM20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 2.75c1.24 0 2.25 1.01 2.25 2.25s-1.01 2.25-2.25 2.25S9.75 10.24 9.75 9 10.76 6.75 12 6.75zM17 17H7v-1.5c0-1.67 3.33-2.5 5-2.5s5 .83 5 2.5V17z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDesktopAccessDisabled;
impl IconShape for MdDesktopAccessDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23 16c0 1.1-.9 2-2 2h-1l-2-2h3V4H6L4 2h17c1.1 0 2 .9 2 2v12zm-5.5 2l-2-2zm-2.6 0l6 6 1.3-1.3-4.7-4.7-2-2L1.2 1.8 0 3.1l1 1V16c0 1.1.9 2 2 2h7v2H8v2h8v-2h-2v-2h.9zM3 16V6.1l9.9 9.9H3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwOLxrzwokrXhv4La54HeC3ucK8cOIFQ/zLvht4L/713gb4af5l5oUTV7w28Ftc8TrAb3OFeeHEC4b4l5l/m+8B3pt/mbniEvDXPK/X5orXBn6LK14H+G2uMFdcAv6aK14aOMYV4gVD/MvMv83vAK/Nv8xc8TvAa/OCvTbwW1zxOsBvc4W54neA1+aK3wZeiyvEC4b4l5l/m98BXpt/mbnid4DX5gV7beC3uOJ1gN/mCnPF7wCvzRW/DbwWV4gXDPEv+2vgpfjX+x3gtfmXmSt2gb/meb0OV7w28Ftc8TrAb3OFuWIX+GuueGngOFeIFwzxL/tt4LX4txH/MvPCiSteG/gtrngd4Le5wrxw4gVD/Mt+G3gt/m3Ev8y8cOKK1wZ+iyteB/htrjAvnHjBEP+yrwY+in+b1wF+m3+93wZeiyvEFa8N/BZXvA7w21xhrvgd4LX510H8yz4b+Cz+bT4G+Gr+9X4beC2uEFe8NvBbXPE6wG9zhbnid4DX5l8H8S97beC3+Lf5GeCteeFei+f11cBLc4W44rWB3+KK1wF+myvMFb8DvDb/Ooh/2YOBp/NvJ14488KJK14b+C2ueB3gt7nCXPE7wGvzr4N40dwKPIh/m9cBfpsXzLxw4orXBn6LK14H+G2uMFf8DvDa/OsgXjTfDbwX/zavA/w2L9hr88L9NlccB16aK/4a2OWK1+aKXeCv+ddBvGjeG/gu/vX+Bnhp/udCvGgeDDydf52/AV4b2OV/LsSL7lbgQbxo/gZ4bWCX/9kQL7qPBr6Kf9nfAK8N7PI/H+JF92Dg6bxwfwO8NrDL/w6If53fBl6L5+9vgNcGdvnfA/Gv897Ad/G8ngG8NLDL/y6If71bgQfxvB4C3Mr/Loh/vfcGvovn9T3Ae/O/C+Lf5lbgQTyv1wF+m/89EP827w18F8/rVuBlgF3+d0D82/028Fo8r68BPpr/HRD/di8N/BXP3+sAv83/fIh/n68GPorntQu8DHAr/7Mh/n2OA78NvBTP66+B1wF2+Z8L8e/30sBvA8d4Xr8NvA7/cyH+Y7w38F08f98NvA//MyH+43w38F48f98NvA//Pq/FFb/DfxzEf6zfBl6L5++3gbcBdvnXOQ78FvDSPNtPA38N/DbwO/zbIf5jHQd+G3gpnr+/Bt4GuJUXzXHgt4CX5l/22zzbrcDPAD/NC4f4j3cc+G3gpXj+doG3AX6bF+448FvAS/Nv9znAZ/OCIf5zHAd+G3gpXrCvBj4H2OV5HQd+C3hp/n1uBR7CC4b4z3Mc+GngtXjBbgXeB/htnu048FvAS/MfQ7xgiP983w28Fy/cdwOfA+wCvwW8NP9xxAuG+K/x3sBXA8d44W4FHsx/LPGCIf7rvDTw3cBL8V9LvGCI/1rHgc8GPor/OuIFQ/z3eGngq4HX4j+feMEQ/73eG/hs4EH85xEvGOJ/hvcGPht4EP/xxAuG+J/lvYG3Bt6K/zjiBUP8z/Rg4K2BjwYexL+PeMEQ//M9GHht4LWB1wYexIvue4D35gVD/O/zYODBwEsDDwZemiuOAy/Fs30P8NHALi8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IE2vCQcuAjjsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDialerSip;
impl IconShape for MdDialerSip {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 3h-1v5h1V3zm-2 2h-2V4h2V3h-3v3h2v1h-2v1h3V5zm3-2v5h1V6h2V3h-3zm2 2h-1V4h1v1zm0 10.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.01.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.27-.26.35-.65.24-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDialpad;
impl IconShape for MdDialpad {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 19c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 1c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12-8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-6 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDomainDisabled;
impl IconShape for MdDomainDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5h2v2h-.9L12 9.9V9h8v8.9l2 2V7H12V3H5.1L8 5.9zm8 6h2v2h-2zM1.3 1.8L.1 3.1 2 5v16h16l3 3 1.3-1.3-21-20.9zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm4 8H8v-2h2v2zm0-4H8v-2h2v2zm2 4v-2h2l2 2h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE+UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX++1+J/td3jRIV50nwV8NHCc/9l2ga8GPod/GeJF813Ae/O/y3cD78MLh/iXvTfwXfzv9D7Ad/OCIf5lfwW8NP87/Q7w2rxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5n53028YIh/mfnfTbxgiH+ZeV4fA/w1/7O8NPBVPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/+Y4DXwV8DLDLC/fawG/xvMQLhviXmef1OsBv85/rOPBbwEsDfw28DrDLC/bawG/xvMQLhviXmef1OsBv85/nOPBbwEvzbH8NvA6wy/P32sBv8bzEC4b4l5nn9TrAb/Of4zjwW8BL87z+GngdYJfn9drAb/G8xAuG+JeZ5/U6wG/zH+848FvAS/P8/Q3w2sAuz+u1gd/ieYkXDPEvM8/rdYDf5j/WceC3gJfm+fsb4LWBXZ6/1wZ+i+clXjDEv8w8r9cBfpsX7qWBnwLeBvhrXrjjwG8BL83z9zfAawO7vGCvDfwWz0u8YIh/mXlerwP8Ni/YSwO/BRwHdoHXAf6a5+848FvAS/P8/Q3w2sAuL9xrA7/F8xIvGOJfZp7X6wC/zfP30sBvAcd5tl3gdYC/5jkdB34LeGmev78BXhvY5V/22sBv8bzEC4b4l5nn9TrAb/O8jgNPB47zvHaB1wH+miuOA78FvDTP398Arw3s8qJ5beC3eF7iBUP8y8zzeh3gt3n+3hv4Lp6/XeB1gFuB3wJemufvb4DXBnZ50b028Fs8L/GCIf5l5nm9DvDbvGDvDXwXz98ucCvw0jx/fwO8NrDLv85rA7/F8xIvGOJfZp7X6wC/zQv33sB38a/zN8BrA7v867028Fs8L/GCIf5l5nm9DvDb/MveG/guXjR/A7w2sMu/zWsDv8XzEi8Y4l9mntfrAL/Ni+a9ge/ihfsb4LWBXf7tXhv4LZ6XeMEQ/zLzvF4H+G1edO8NfBfP398Arw3s8u/z2sBv8bzEC4b4l5nn9TrAb/Ov897Ad/Gc/gZ4bWCXf7/XBn6L5yVeMMS/zDyv1wF+m3+99wa+iyv+BnhtYJf/GK8N/BbPS7xgiH+ZeV6vA/w2/zbvDXw08NrALv9xXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3+Z/ltYHf4nmJFwzxLzPP63WA3+Z/ltcGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO/m3jBEP+yvwZeiv+dfgd4bV4wxL/svYHv4n+n9wG+mxcM8aL5buC9+N/le4D35oVDvOg+G/ho4Bj/s10Cvhr4bP5liH+91+Z/tt/mRYf4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wgL9rNBbDHgLwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDomainVerification;
impl IconShape for MdDomainVerification {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                points: "16.6,10.88 15.18,9.46 10.94,13.71 8.82,11.58 7.4,13 10.94,16.54",
            }
            path {
                d: "M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.11,4,19,4z M19,18H5V8h14V18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dY4DLwW8NHCcK14aOM5/vF3gr4GfAf6aFwzxn+u1gbcCXht4af57fDXwMTx/iP94x4GPAt4beDD/M3wN8NE8L8R/nOPARwEfDRznf56HALfynBD/MV4b+C7gwfzP9TnAZ/OcEP9+XwV8NP/z/Qzw1jwnxL/dceC3gJfmf4ffAV6b54T4t3kw8FPAS/O/x+8Ar81zQvzrPRj4K+A4/7v8DvDaPCfEv85x4LeAl+Z/n98BXpvnhPjX+S3gtfnf6XeA1+Y5IV50nw18Fv97/Q7w2jwnxIvmpYG/4n+33wFem+eEeNH8FvDa/O/2O8Br85wQ/7L3Br6L//1+B3htnhPiX/Z04MH87/c7wGvznBAv3HsD38X/Db8DvDbPCfHC/TTwVvzf8DvAa/OcEC/Yg4Gn83/H7wCvzXNCvGAfDXwVL7rfAV6b/xi/DbwWz9/XAK8NvBT/Or8DvDbPCfGC/TTwVrzofgd4bf5j/DbwWjynZwDvDfw28NvAa/Gv8zvAa/OcEC/YReA4L7rfAV6b/xi/DbwWz/Y1wGcDu1zx28Br8a/zO8Br85wQz99LA3/Fv87vAK/Nf4zfBl4LeAbw3sBv85x+G3gt/nV+B3htnhPi+Xtr4Kf41/kd4LX5j/HbwF8Dnw3s8rx+G3gt/nV+B3htnhPi+fts4LP41/kd4LX5j/Fg4FZesN8GXot/nd8BXpvnhHj+Phv4LP51fgd4bf5r/DbwWvzr/A7w2jwnxPP31cBH8a/zO8Br81/jt4HX4l/nd4DX5jkhnr/fBl6Lf53fAV6b/xq/DbwW/zo/A7w1zwnx/H038F786/wO8Nr81/ht4LX41/kc4LN5Tojn77OBz+Jf53eA1+Y/xoOBW3nBfht4Lf51Pgf4bJ4T4vn7bOCz+Nf5HeC1+Y/x28BfAZ8D7PK8fht4Lf51Pgb4ap4T4vl7a+Cn+Nf5HeC1+Y/x28BrAbcC7wP8Ns/pt4HX4l/ndYDf5jkhnr+XBv6Kf53fAV6b/xi/DbwWz/bVwOcAu1zx28Br8a9zAtjlOSFesF3gGC+63wFem/8Yvw28Fs/pVuB9gN8Gfht4LV50fwO8NM8L8YL9NPBWvOh+B3ht/mP8NvBaPH9fDbwO8FK86L4G+GieF+IFe2/gu/i/4XWA3+Z5IV6w48BF/vd7BvBgnj/EC/fdwHvxv9vnAJ/N84d44R4MPJ3/vS4BDwZ2ef4Q/7LvBt6L/50+B/hsXjDEv+zBwNP53+cZwEsDu7xgiBfNZwOfxf8ubwP8NC8c4kX318BL8b/D9wDvzb8M8aJ7MPDXwDH+Z/sb4KV50SD+dV4b+C3+53oG8NLALi8axL/eewPfxf88l4DXBv6aFx3i3+a9ge/if46/Ad4b+Gv+dRD/dq8N/DRwjP9efwO8NrDLvx7i3+fBwE8DL8V/j88BPpt/O8R/jM8GPov/Or8DfDbw2/z7IP7jPBj4bOC9+M/zDOCzge/mPwbiP96Dgc8G3ov/OL8DfDfw3fzHQvznOQ68NfDWwGsDx3jRXQJ+G/ht4KeBW/nPgfiv89LAg4GX5ooHAw8Gfptn+21gF/hr/msg/n9D/P+G+P8N8f8b4v83/hEV5LhBONNWvAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDuo;
impl IconShape for MdDuo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2h-8C6.38 2 2 6.66 2 12.28 2 17.5 6.49 22 11.72 22 17.39 22 22 17.62 22 12V4c0-1.1-.9-2-2-2zm-3 13l-3-2v2H7V9h7v2l3-2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4a+Cn+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5HuC9eU6I5++3gdfiOf0N8NPAZ/E/2+cAbw28FM/pd4DX5jkhnr/fBl6L5/Q7wGsDLw18NfBa/M/yO8BHA38N/DbwWjyn3wFem+eEeP5+G3gtntPvAK/Ns3008NnAMf57XQI+G/hqnu23gdfiOf0O8No8J8Tz99vAa/Gcfgd4bZ7TceC7gbfiv8fPAO8N7PKcfht4LZ7T7wCvzXNCPH+/DbwWz+l3gNfm+Xtt4LuBB/Ff4xnAewO/zfP328Br8Zx+B3htnhPi+ftt4LV4TrcCbwP8Nc/fceCjgc/iP9fnAF8N7PL8vTTwU8CDeU6/A7w2zwnx/P028Fo8f58NfA2wy/P30sBXA6/Ff6zfAT4a+Guev+PARwGfzfP3O8Br85wQz99vA6/FC3Yr8D7Ab/OCfTTw2cAx/n0uAZ8NfDUv2GsD3wU8mBfsd4DX5jkhnr/fBl6Lf9lPA+8D7PL8HQe+G3gr/m1+BnhvYJfn7zjwXcBb8y/7HeC1eU6I5++3gdfiRbMLfDbwNbxgrw18N/AgXjTPAN4b+G1esI8CPhs4zovmd4DX5jkhnr/fBl6Lf53fBj4G+Guev+PARwOfxQv3OcBXA7s8fy8NfBXw2vzr/A7w2jwnxPP328Br8W/z2cDXALs8fy8NfDXwWjyn3wE+Gvhrnr/jwEcBn82/ze8Ar81zQjx/vw28Fv92twLvA/w2L9hHA5/NFZ8NfDUv2GsD3wU8mH+73wFem+eEeP5+G3gt/v1+GngfYJfn7zhX7PL8HQe+C3hr/v1+B3htnhPi+ftt4LX4j7ELfDbwNfzrfBTw2cBx/mP8DvDaPCfE8/fbwGvxH+u3gY8B/poX7qWBrwJem/9YvwO8Ns8J8fz9NvBa/Of4bOBrgF2e03Hgo4DP5j/H7wCvzXNCPH+/DbwW/3luBd4H+G2ueG3gu4AH85/nd4DX5jkhnr/fBl6L/3w/zRVvzX++3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++7gffi/5avAT6a54R4/j4a+Cr+b/kY4Kt5Tojn7zhwK3CM/xsuAQ8GdnlOiBfsvYHv4v+GtwF+mueFeOFeG/hu4EH87/QM4L2B3+b5Q7xoXho4zv8uu8Bf88Ih/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CFMuzQffOqdQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEmail;
impl IconShape for MdEmail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xAv2EsD7wW8NP9z/DTwPcAu/zEQz99LA3/F/0x/DbwM/zEQz99XAx/F/1yvA/w2/36I5++3gdfif67XAX6bfz/E8/fbwGvxP9frAL/Nvx/i+ftt4LX4n+t1gN/m3w/x/P028Fr8z/U6wG/z74d4/n4beC3+53od4Lf590M8f78NvBb/c70O8Nv8+yGev98GXov/uV4H+G3+/RDP328Dr8X/XK8D/Db/fojn77eB1+J/rtcBfpvn9NrAb/G8bgW+G/gcnhfi+ftt4LX4n+t1gN/mOb028Fu8YG8D/DTPCfH8/TbwWvzP9TrAb/OcXhv4LV6w7wHem+eEeP5+G3gt/ud6HeC3eU6vDfwWL9jvAK/Nc0I8f78NvBb/c70O8Ns8p9cGfosX7HeA1+Y5IZ6/3wZei/+5Xgf4bZ7TawO/xQv2O8Br85wQz99vA6/F/1yvA/w2z+m1gd/iBfsd4LV5Tojn77eB1+J/rtcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8T/X6wC/zXN6beC3eMF+B3htnhPi+ftt4LX4n+t1gN/mOb028Fu8YL8DvDbPCfH8/TbwWvzP9TrAb/OcXhv4LV6w3wFem+eEeP5+G3gt/ud6HeC3eU6vDfwWL9jvAK/Nc0I8f78NvBb/c70O8Ns8p9cGfosX7HeA1+Y5IZ6/3wZei/+5Xgf4bZ7TawO/xQv2O8Br85wQz99vA6/F/1yvA/w2z+m1gd/iBfsd4LV5Tojn77eB1+J/rtcBfpvn9NrAb/GC/Q7w2jwnxPP328Br8T/X6wC/zXN6beC3eMF+B3htnhPi+ftt4LX4n+lvgJfmeb028Fu8YL8DvDbPCfH8/TbwWvzLLgF/zX+NXeC3ge8Gdnlerw38Fi/Y7wCvzXNCPH+/DbwWL9wl4LWBv+Z/htcGfosX7HeA1+Y5IZ6/3wZeixfsEvDawF/zP8drA7/FC/Y7wGvznBDP328Dr8Xzdwl4beCv+Z/ltYHf4gX7HeC1eU6I5++3gdfieV0CXhv4a/7neW3gt3jBfgd4bZ4T4vn7beC1eE6XgNcG/pr/mV4b+C1esN8BXpvnhHj+fht4LZ7tEvDawF/zvMz/Dr8DvDbPCfH8/TbwWlxxCXht4K95/sz/Dr8DvDbPCfH8/TbwWsAl4LWBv+YFM/87/A7w2jwnxPP328BLA68N/DUvnPnf4XeA1+Y5IZ6/nwY+G/hr/mXmf4ffAV6b54R4/o4Du7xozP8OvwO8Ns8J8e/33cB78T/f7wCvzXNC/PsdB74aeC/+Z/sd4LV5Toj//d4b+C7+Zb8DvDbPCfF/w3sD38UL9zvAa/OcEP93vDfwXbxgvwO8Ns8J8X/LewPfxfP3O8Br85wQ//e8N/BdPK/fAV6b54T4v+m9ge/iOf0O8No8J8T/Xe8NfBfP9jvAa/OcEP+3vTfwXVzxO8Br85wQ//e9N/BdwO8Ar81zQvz/8N7AewOvzXNC/P/x0sBf85wQ/78h/n9D/P+G+P8N8f8b/wjeu7NBNyGVgQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForum;
impl IconShape for MdForum {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 6h-2v9H6v2c0 .55.45 1 1 1h11l4 4V7c0-.55-.45-1-1-1zm-4 6V3c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v14l4-4h10c.55 0 1-.45 1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdForwardToInbox;
impl IconShape for MdForwardToInbox {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h9v-2H4V8l8,5l8-5v5h2V6C22,4.9,21.1,4,20,4z M12,11L4,6h16L12,11z M19,15l4,4 l-4,4v-3h-4v-2h4V15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/Ff6/X4d8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9nntffALv8xzoOvBTPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r9cBfpv/WK8N/BbPS/zbIf79zPN6HeC3+Y/12sBv8bzEvx3i3888r78GXgfY5T/GceC3gJfmeYl/O8S/318DL8Xz+mvgdYBd/n2OA78FvDTP62+Al+bfDvHvdxz4beCleF5/DbwOsMu/zXHgt4CX5nn9DfDawC7/doj/GMeB3wZeiuf118DrALv86xwHfgt4aZ7X3wCvDezy74P4j3Mc+G3gpXhefw28DrDLi+Y48FvAS/O8/gZ4bWCXfz/Ef6zjwG8DL8Xz+mvgdYBdXrjjwG8BL83z+hvgtYFd/mMg/uMdB34beCme118DrwPs8vwdB34LeGme198Arw3s8h8H8Z/jOPDbwEvxvP4aeB1gl+d0HPgt4KV5Xn8DvDawy38sxH+e48BvAy/F8/pr4HWAXa44DvwW8NI8r78BXhvY5T8e4j/XceC3gZfief018Dpc8VvAS/O8/gZ4bWCX/xyI/3zHgd8GXorn9ddc8dI8r78BXhvY5T8P4r/GceC3gZfiRfM3wGsDu/znQvzXOA78FvDSvGj+GngdYJf/XIj/fMeB3wJemn+dvwZeB9jlPw/iP9dx4LeAl+bf5q+B1wF2+c+B+M9zHPgt4KX59/lr4HWAXf7jIf5zHAd+C3hp/mP8NfA6wC7/sRD/8Y4DvwW8NP+x/hp4HWCX/ziI/1jHgd8CXpr/HH8NvA6wy38MxH+c48BvAS/Nf66/Bl4H2OXfD/Ef4zjwW8BL81/jr4HXAXb590H8+x0Hfgt4af5r/TXwOsAu/3aIf7+/Al6a/x5/DbwM/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/vt/nv9dr82yH+f0P8/4b4/w3x/xvi/zf+EVqqkkEcjvu3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHourglassBottom;
impl IconShape for MdHourglassBottom {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18,22l-0.01-6L14,12l3.99-4.01L18,2H6v6l4,4l-4,3.99V22H18z M8,7.5V4h8v3.5l-4,4L8,7.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/Ff6/X4d8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7mf9e4t8O8e9n/nuJfzvEv5/57yX+7RD/fua/l/i3Q/z7/TXwUvz3+Bvgpfm3Q/z7HQd+G3gp/mv9DfDawC7/doj/GMeB3wZeiv8afwO8NrDLvw/iP85x4LeBl+I/198Arw3s8u+H+I91HPht4KX4z/E3wGsDu/zHQPzHOw78NvBS/Mf6G+C1gV3+4yD+cxwHfht4Kf5j/A3w2sAu/7EQ/3mOA78NvBT/Pn8DvDawy388xH+u48BvAy/Fv83fAK8N7PKfA/Gf7zjw28BL8a/zN8BrA7v850H81zgO/DbwUrxo/gZ4bWCX/1yI/xrHgd8CXpoXzV8DrwPs8p8L8Z/vOPBbwEvzvP6GK16K5/XXwOsAu/znQfznOg78FvDSPK+/AV6bK34beCme118DrwPs8p8D8Z/nOPBbwEvzvP4GeG1glyuOA78NvBTP66+B1wF2+Y+H+M9xHPgt4KV5Xn8DvDawy3M6Dvw28FI8r78GXgfY5T8W4j/eceC3gJfmef0N8NrALs/fceC3gZfief018DrALv9xEP+xjgO/Bbw0z+tvgNcGdnnhjgO/DbwUz+uvgdcBdvmPgfiPcxz4LeCleV5/A7w2sMuL5jjw28BL8bz+GngdYJd/P8R/jOPAbwEvzfP6G+C1gV3+dY4Dvw28FM/rr4HXAXb590H8+x0Hfgt4aZ7X3wCvDezyb3Mc+G3gpXhefw28DrDLvx3i3++vgJfmef0N8NrALv8+x4HfBl6K5/XXwMvwb4f49zPP62+A1wZ2+Y9xHPht4KV4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37mef01sMt/rOPAS/O8xL8d4t/P/PcS/3aIfz/z30v82yH+/cx/L/Fvh/j3M/+9xL8d4t/vt/nv9dr82yH+f0P8/4b4/w3x/xvi/zf+EdAJkkGLa9IrAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdHourglassTop;
impl IconShape for MdHourglassTop {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6,2l0.01,6L10,12l-3.99,4.01L6,22h12v-6l-4-4l4-3.99V2H6z M16,16.5V20H8v-3.5l4-4L16,16.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+IF81x4KWA1wYeDDyYK16b57QL/DVX/DXw18CtwO/wb/NawIOBlwZemiteGjjOc/ptrrgVuBX4beBvgF1eOMQL9mDgrYD3Bl6af59d4KeBnwZ+hhfurYC3Bt4aOM6/z08DPw38DLDL80I8rwcDnwW8N/85doGvBj6H5/RZwEcDx/nP8d3A5wC38myI5/RZwEcDx/nPdyvwPlzxXcCD+c+3C3w18DlcgXi21wZ+i/8fXgf4bQDxbK8N/Bb/M10C/hr4ba7YBf6aK16bZ3tp4MHAS/HCvQ7w2wDi2V4b+C3+Z7gE/DTw28BvA7fyr/fawFsDH8Xzeh3gtwHEs7028Fv89/oe4LuB3+Y/jnlerwP8NoB4ttcGfov/epeArwa+GtjlP555Xq8D/DaAeLbXBn6L/3ongF3+85jn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4ryf+c5nn9TrAbwOIZ3tt4Lf4rydeuNcCXhp4aeDBwIOBB/OcbgVuBXaBvwb+GvgdYBcwz+t1gN8GEM/22sBv8V9PPKfjwFsBbw28Nf8+fw28NM/rdYDfBhDP9trAb/FfT1zx2sBHAW/Nf77XAX4bQDzbawO/xX+91wG+C3gw/3VeB/htAPFsrw38Fv8/vA7w2wDi2V4b+C3+Zc8A/hr4a67YBf4aeGngOFccB14aeGngGP9+l4C/Bn6bK3aBvwYeDDyYZ3tt4MHAg3jhXgf4bQDxbK8N/BbP62+A3wZ+Gvht/vUeDLw28NbAW/GiuQT8NPDbwG8Dt/Kvcxx4beCtgdcGHsRzeh3gtwHEs7008FdccQn4auC7gVv5j3MceGvgs4EH8by+B/hu4Lf5j/XewHsDrwVcAl4auBVAPKeXBo4Dv81/vs8GPporvhr4amCX/1wvDewCt3IF4r/Xca7Y5b8H4v83xP9viP/fEP+/If5/Q/z/xj8CQrPKQfwYbfUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImportContacts;
impl IconShape for MdImportContacts {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.5,4.5c-1.95,0-4.05,0.4-5.5,1.5c-1.45-1.1-3.55-1.5-5.5-1.5S2.45,4.9,1,6v14.65c0,0.65,0.73,0.45,0.75,0.45 C3.1,20.45,5.05,20,6.5,20c1.95,0,4.05,0.4,5.5,1.5c1.35-0.85,3.8-1.5,5.5-1.5c1.65,0,3.35,0.3,4.75,1.05 C22.66,21.26,23,20.86,23,20.6V6C21.51,4.88,19.37,4.5,17.5,4.5z M21,18.5c-1.1-0.35-2.3-0.5-3.5-0.5c-1.7,0-4.15,0.65-5.5,1.5V8 c1.35-0.85,3.8-1.5,5.5-1.5c1.2,0,2.4,0.15,3.5,0.5V18.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/VdXPE+/M+E+M/zXcB7c8V3A+/D/zyI/xzfBbw3z+m7gffhfxbEf7zvAt6b5++7gffhfw7Ef6zvAt6bF+67gffhfwbEf5zvAt6bF813A+/Dfz/Ef4zvAt6bf53vBt6H/16If7/vAt6bf5vvBt6H/z6If5/vAt6bf5/vBt6H/x6If7vvAt6b/xjfDbwP//UQ/zbvDbw3L9hLA8d4TpeAv+YF+27gu/mvhfjP8dvAa/Gcfgd4bf5nQfzn+G3gtXhOvwO8Nv+zIP5z/DbwWjyn3wFem/9ZEP85fht4LZ7T7wCvzf8siP8cvw28Fs/pd4DX5n8WxH+O3wZei+f0O8Br8z8L4j/HbwOvxXP6HeC1+Z8F8Z/jt4HX4jn9DvDavGheGjjGc7oE/DX/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jt4HX4jn9DvDavGh+G3gtntPvAK/NfyzEf47fBl6L5/Q7wGvzovlt4LV4Tr8DvDb/sRD/OX4beC2e0+8Ar82L5reB1+I5/Q7w2vzHQvzn+G3gtXhOvwO8Ni+a3wZei+f0O8Br8x8L8Z/jpYHjPKdd4K950fw28Fo8p98BXpv/WIj/mX4beC2e0+8Ar81/LMT/TL8NvBbP6XeA1+Y/FuJ/pt8GXovn9DvAa/MfC/E/028Dr8Vz+h3gtfmPhfif6beB1+I5/Q7w2vzHQvzP9NvAa/Gcfgd4bf5jIf5n+m3gtXhOvwO8Nv+xEP893ht4L16wlwaO85x2gb/mBfse4Lv510H89/lu4L34j/E9wHvzr4f47/XdwHvx7/M9wHvzb4P47/fdwHvxb/M9wHvzb4f4n+G7gffiX+d7gPfm3wfxP8d3A+/Fi+Z7gPfm3w/xP8t3A+/FC/c9wHvzHwPxP893A+/F8/c9wHvzHwfxP9N3A+/Fc/oe4L35j4X4n+u7gffiio8Bvpr/eIj/2b4b+G3gu/nPgfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Ajlhe0FidiQyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImportExport;
impl IconShape for MdImportExport {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 3L5 6.99h3V14h2V6.99h3L9 3zm7 14.01V10h-2v7.01h-3L15 21l4-3.99h-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdInvertColorsOff;
impl IconShape for MdInvertColorsOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.65 20.87l-2.35-2.35-6.3-6.29-3.56-3.57-1.42-1.41L4.27 4.5 3 5.77l2.78 2.78c-2.55 3.14-2.36 7.76.56 10.69C7.9 20.8 9.95 21.58 12 21.58c1.79 0 3.57-.59 5.03-1.78l2.7 2.7L21 21.23l-.35-.36zM12 19.59c-1.6 0-3.11-.62-4.24-1.76C6.62 16.69 6 15.19 6 13.59c0-1.32.43-2.57 1.21-3.6L12 14.77v4.82zM12 5.1v4.58l7.25 7.26c1.37-2.96.84-6.57-1.6-9.01L12 2.27l-3.7 3.7 1.41 1.41L12 5.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l7008FXAa/O/y28DbwPs8oIh/mW/Bbw2/zv9DPDWvGCIf5n53028YIh/mfnfTbxgiH+Z+d9NvGCIf5l5Xh8D/DX/s7w08FU8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Ns8p+PAS/G8fofn9dLAMf7t/gbY5Tm9NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i+clntdvA6/Fv93rAL/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXot/u9cBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C2el3hevw28Fv92rwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gt/u1eB/htntNrA7/F8xIvGOJfZp7X6wC/zXN6beC3eF7ief028Fr8270O8Ns8p9cGfovnJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/WV4b+C2el3jBEP8y87xeB/htntNLA1/F83odntdXAy/Fv93HAH/Nc3pt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXot/u9cBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C2el3hevw28Fv92rwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gt/u1eB/htntNrA7/F8xIvGOJfZp7X6wC/zXN6beC3eF7ief028Fr8270O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TSwFfzvF6b5/XVwEvzb/fRwF/znF4b+C2el3jBEP8y87xeB/ht/md5beC3eF7iBUP8y8zzeh3gt/mf5bWB3+J5iRcM8S8zz+t1gN/mf5bXBn6L5yVeMMS/zDyv1wF+m/9ZXhv4LZ6XeMEQ/zLzvF4H+G3+Z3lt4Ld4XuIFQ/zLzPN6HeC3eU6vDfwWz0s8r98GXot/u9cBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C2el3hevw28Fv92rwP8Ns/ptYHf4nmJFwzxLzPP63WA3+Y5vTbwWzwv8bx+G3gt/u1eB/htntNrA7/F8xIvGOJfZp7X6wC/zXN6beC3eF7ief028Fr8270O8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xvMTz+m3gtfi3ex3gt3lOrw38Fs9LvGCIf5l5Xq8D/DbP6Tjw0jyv3+Z5vTRwnH+7vwZ2eU6vDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae1+sAv83/LK8N/BbPS7xgiH+ZeV4fDfw1/7O8NPDVPC/xgiH+ZeZ/N/GCIf5l5n838YIh/mXmfzfxgiH+Zb8NvBb/O30P8N68YIh/2XHgu4G34n+X7wE+GtjlBUP8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwHsH7hBEvj75QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdListAlt;
impl IconShape for MdListAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 5v14H5V5h14m1.1-2H3.9c-.5 0-.9.4-.9.9v16.2c0 .4.4.9.9.9h16.2c.4 0 .9-.5.9-.9V3.9c0-.5-.5-.9-.9-.9zM11 7h6v2h-6V7zm0 4h6v2h-6v-2zm0 4h6v2h-6zM7 7h2v2H7zm0 4h2v2H7zm0 4h2v2H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/Om8FvDTw2vzP9NvAXwM/w4sG8aJ5MPBTwEvzv8NfA28D3MoLh/iXHQf+Cngw/7v8NfA6wC4vGOJf9tnAZ/G/0+cAn80LhviXPR14MP873Qo8hBcM8S8z/zl+h+f0WvznEC8Y4l9m/uP8DPDdwE/z/L018N7AW/EfR7xgiH+Z+ff7G+Cjgd/mRfPawFcDL8W/n3jBEP8y8+/zN8BrA7v86xwHfht4Kf59xAuG+JeZf7u/AV4b2OXf5jjw28BL8W8nXjDEv8z821wCXhq4lX+fBwNP599OvGCIf5n5t/kc4LN5/t4L+Gjgpbnir4GvBr6H5++zgc/i30a8YIh/mfm3OQHs8ry+C3hvnr/vBt6H53UcuMi/jXjBEP8y86/3O8Br87w+G/gsXrjPAT6b5/XXwEvxrydeMMS/zPzrfQ7w2Tyvi8BxXrhd4ATP67OBz+JfT7xgiH+Z+df7HOCzeU6vDfwWL5rXAX6b5/TZwGfxrydeMMS/zPzr/TWwy3M6Drw0L5rXAX6b5/TWwE/xrydeMMS/zPzXexngr3lOnw18Fv964gVD/MvMf61nAA/meX038F7864kXDPEvM/+1Xgf4bZ7TceDpwHH+9cQLhviXmf867wN8N8/rs4HP4t9GvGCIf5n5r/E+wHfzvF4a+C3gOP824gVD/MvMf773Ab6b53Uc+CvgwfzbiRcM8S8z/7neB/huntdx4LeAl+bfR7xgiH+Z+c/zPcB787yOA78FvDT/fuIFQ/zLzH+OZwAP5nkdB34LeGn+Y4gXDPEvM/853gf4bp7XTwFvzX8c8YIh/mXmP94l4DjP662Bn+I/lnjBEP8y8x/ve4D35nn9NPBW/McSLxjiX2b+430O8Nk8p+PARf7jiRcM8S8z//HeB/huntNrA7/FfzzxgiH+ZeY/3usAv81zem3gt/iPJ14wxL/M/Mf7a2CX53QceGn+44kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z//H+BtjlOR0HXor/eOIFQ/zLzH+81wF+m+f02sBv8R9PvGCIf5n5j/c6wG/znF4b+C3+44kXDPEvM//xXgf4bZ7TawO/xX888YIh/mXmP97rAL/Nc3pt4Lf4jydeMMS/zPzHex3gt3lOrw38Fv/xxAuG+JeZ/93EC4b4l/018FL87/Q7wGvzgiH+Ze8NfBf/O70P8N28YIgXzXcD78X/Lt8DvDcvHOJF99nARwPH+J/tEvDVwGfzL0P86702/zpfDbwU/zZ/A3w0/zq/zYsO8Z/vOPDbwEvxr/M3wGsDu/znQfzXOA78NvBSvGj+BnhtYJf/XIj/OseB3wZeihfub4DXBnb5z4f4r3Uc+G3gpXj+/gZ4bWCX/xqI/3rHgd8GXorn9DfAawO7/NdB/Pc4Dvw28FJc8TfAawO7/NdC/Pc5Dvw2V7w2sMt/PcR/r+Ncsct/D8T/b4j/3/hHupSqQfg+lj4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLiveHelp;
impl IconShape for MdLiveHelp {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 2H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 16h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 11.9 13 12.5 13 14h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationOff;
impl IconShape for MdLocationOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 6.5c1.38 0 2.5 1.12 2.5 2.5 0 .74-.33 1.39-.83 1.85l3.63 3.63c.98-1.86 1.7-3.8 1.7-5.48 0-3.87-3.13-7-7-7-1.98 0-3.76.83-5.04 2.15l3.19 3.19c.46-.52 1.11-.84 1.85-.84zm4.37 9.6l-4.63-4.63-.11-.11L3.27 3 2 4.27l3.18 3.18C5.07 7.95 5 8.47 5 9c0 5.25 7 13 7 13s1.67-1.85 3.38-4.35L18.73 21 20 19.73l-3.63-3.63z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tY4DLwW8Nlc8mCtu5YrfBv4G2OW/BuI/34OBtwLeGnhtXjS/Dfw08DPArfznQfznOQ58FPDZ/NvtAl8NfA2wy388xH+OjwI+GzjOf4xd4GOA7+Y/FuI/3ncB781/ju8G3of/OIj/OMeB3wJemv9cfw28DrDLvx/iP85fAS/Nf42/Bl6Gfz/Ef4zvAt6b/1rfDbwP/z6If7+PBr6K/x7vA3w3/3aIf5/jwNOB4/z32AUeAuzyb4P49/ls4LP47/U5wGfzb4P4t3sw8HT+++0CLwPcyr8e4t/uo4Gv4t/mb4DfBv6aK14aeG3gpfi3+Rjgq/nXQ/zb/TbwWvzrXAI+Gvhunr/3Br4aOMa/zu8Ar82/HuLf5jhwkX+dS8BrA3/NC/fSwG8Dx/jXOQHs8q+D+Ld5beC3+Nd5H+C7edG8N/Bd/Ou8DvDb/Osg/m0+G/gsXnR/A7w0/zq3Ag/iRfcxwFfzr4P4t/ls4LN40X0N8NH863w18FG86D4H+Gz+dRD/Nt8NvBcvuvcBvpt/nY8GvooX3dcAH82/DuLf5ruB9+JF9z7Ad/Ov89HAV/Gi+x7gvfnXQfzbfDbwWbzovgb4aP51vhr4KF50nwN8Nv86iH+bzwY+ixfdXwMvw7/O04EH86L7HOCz+ddB/Nu8NvBb/Ou8D/DdvGjeG/gu/nVeB/ht/nUQ/zbHgYv86+wCrwP8NS/cSwO/BRznX+cEsMu/DuLf7lbgQfzr7AIfDXwPz997AV8NHOdf5xnAg/nXQ/zbfTXwUfzb3Ar8NPDXXPHSwFsDD+bf5muAj+ZfD/Fv99LAX/E/w8sAf82/HuLf51bgQfz3egbwYP5tEP8+nw18Fv+9Pgf4bP5tEP8+Dwaezn+vhwC38m+D+Pf7buC9+O/xPcB782+H+Pd7aeCv+O/xEOBW/u0Q/zF+G3gt/mv9DPDW/Psg/mO8NvBb/Nd6GeCv+fdB/Mf5buC9+K/xPcB78++H+I/zYODp/Nd4CHAr/36I/1ifDXwW/7k+B/hs/mMg/mMdB/4aeBD/OZ4BvDSwy38MxH+8twZ+iv8cbwP8NP9xEP85fhp4K/5j/Qzw1vzHQvzneDDw18Ax/mNcAh4M7PIfC/Gf56OBr+I/xscAX81/PMR/rp8G3op/n58B3pr/HIj/XMeBW4Fj/NtcAh4M7PKfA/Gf762Bn+Lf5m2An+Y/D+K/xlcDH8W/ztcAH81/LsR/nb8GXooXzd8AL81/PsR/nQcDfw0c44W7BLw0cCv/+RD/td4a+CleuLcBfpr/Goj/ep8NfBbP3+cAn81/HcR/j58G3orn9D3Ae/NfC/Hf4zjw28BLccXfAK8N7PJfC/Hf5zhwK1c8GNjlvx7iv9dLc8Vf898D8f8b4v83xP9viP/fEP+/8Y8DHZxBeLG/SAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLocationOn;
impl IconShape for MdLocationOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMailOutline;
impl IconShape for MdMailOutline {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V8l8 5 8-5v10zm-8-7L4 6h16l-8 5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz99LA3/F/y0PAW7lOSGev88GPov/Wz4H+GyeE+L5+2ngrfi/5WeAt+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX43+EZwK08p5cGjvGcfgd4bZ4T4vn7beC1+J/pb4DvBn4auJUX7Djw2sBbA28N/DXw2jwnxPP328Br8T/L9wCfDdzKv95x4KWB3+Y5IZ6/3wZei/8Zfgd4b+BW/uMhnr/fBl6L/34fA3w1/3kQz99vA6/Ff59LwFsDv81/LsTz99vAa/Hf4xLw2sBf858P8fz9NvBa/Pd4H+C7+a+BeP5+G3gt/ut9DvDZ/Md7L+B7eF6I5++3gdfiv9YzgAfzH++7gPcGvht4H54T4vn7beC1+K/1OsBv8x/ru4D35tm+G3gfng3x/P028Fr81/kd4LX5j/VdwHvzvL4beB+uQDx/vw28Fv913gb4af7jfBfw3jx/l4CXBm4FEM/fbwOvxX+dE8Au/zG+C3hvnr9LwGsDf80ViOfvt4HX4r/GzwBvzfN3HPgq4GOAXf5l3wW8N8/fJeC1gb/m2RDP328Dr8V/jfcBvpvndRz4LeClgb8GXgfY5QX7LuC9ef4uAa8N/DXPCfH8/TbwWvzXeB3gt3lOx4HfAl6aZ/tr4HWAXZ7XdwHvzfN3CXht4K95Xojn77eB1+K/xusAv82zHQd+C3hpntdfA68D7PJs3wW8N8/fJeC1gb/m+UM8f78NvBb/NcTz+m7gvXj+/hp4HWAX+C7gvXn+LgGvDfw1Lxji+ftt4LX4ryGev+8G3ovn76+Bvwbem+fvEvDawF/zwiGev98GXov/GuIF+27gvfjXuQS8NvDX/MsQz99vA6/Ffw3xwn038F68aC4Brw38NS8axPP328Br8S97H+C7+c/33cB78cJdAl4b+GtedIjn77eB1+KFex/gu/mv893Ae/H8XQJeG/hr/nUQz99vA6/FC/Y+wHfzX++7gffiOV0CXhv4a/71EM/fbwOvxfP3PsB389/nu4H34opLwGsDf82/DeL5+23gtXhe7wN8N//9vht4a+C1gb/m3w7x/P028Fo8p/cBvpv/OR4M3Mq/D+L5+23gtXi29wG+m/97EM/fbwOvxRXvA3w3/zchnr/fBl4LeB/gu/m/C/H8/Tbw3cB3838b4vl7aeCv+b8P8f8b4v83xP9viP/fEP+/8Y+7ALhBqzkGlgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkChatRead;
impl IconShape for MdMarkChatRead {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.34,20l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24L23,14.34L17.34,20z M12,17c0-3.87,3.13-7,7-7c1.08,0,2.09,0.25,3,0.68 V4c0-1.1-0.9-2-2-2H4C2.9,2,2,2.9,2,4v18l4-4h6v0c0-0.17,0.01-0.33,0.03-0.5C12.01,17.34,12,17.17,12,17z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjf7b2AtwZeGngwV/w28NPA9wC7vHCI/51eGvgu4KV5wXaBjwG+mxcM8b/PSwO/BRznRfM+wHfz/CH+dzkO/BXwYP51Xgf4bZ4X4n+XzwY+i3+9vwZehueF+Je9NPBWwEsDx/mv8T3Ad/O8ng48mH+blwH+mueEeOG+Cvho/ut9DvDZPKeXBv6Kf7vPAT6b54R4wb4a+Cj+e7wO8Ns8p9cGfot/u68BPprnhHj+Xhr4K/77vA/w3Tyn1wZ+i3+7rwE+mueEeP4+G/gs/vt8DvDZPKeXBv6Kf7vPAT6b54R4/n4aeCv++3wN8NE8r1uBB/Fv8zLAX/OcEM/fbwOvxX+fW4GH8Lw+G/gs/vX+Bnhpnhfi+ftt4LX47/UywF/znI4Dfw08iH+d1wF+m+eFeP5+G3gt/nt9DvDZPK+XBn4bOMaL5n2A7+b5Qzx/vw28Fv/9HgLcyvN6aeCrgdfiBbsEfDTw3bxgiOfvt4HX4r/fTwNvwwv23sBbAy8NPIgrfgf4aeC7gV1eOMTz99vAa/E/w8cAX81/DsTz99vAa/E/x/sA381/PMTz99vAa/E/y1cDH8O/3YOBW3lOiOfvt4HX4n+e3wY+B/htXnQPBr4KOAG8Ns8J8fz9NvBa/M/128B3A78D3Mrz91bAWwPvzRW/A7w2zwnx/P028Fr877AL/DXP6bV5Xr8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2/A7w2zwnx/P028Fr83/I7wGvznBDP328Dr8X/Lb8DvDbPCfH8/TbwWvzf8jvAa/OcEM/fbwOvxf8tvwO8Ns8J8fz9NvBa/N/yO8Br85wQz99vA6/F/y2fA3w2zwnx/P028Fr83/I2wE/znBDP328Dr8X/HX8DvDTPC/H8/TbwWvzf8DfAWwO38rwQz99vA6/F/27PAL4b+Gpgl+cP8fz9NvBa/MveB/hu/vdCPH+/DbwWL9z7AN/N/26I5++3gdfiBXsf4Lv53w/x/P028Fo8f+8DfDf/NyCev98GXovn9T7Ad/N/B+L5+23gtXhO7wN8N/+3IJ6/3wZei2d7H+C7+b8H8fz9NvBaXPE+wHfzfxPi+ftt4LWA9wG+m/+7EM/fbwPfDXw3/7chnr+XBv6a//sQ/78h/n9D/P+G+P8N8f8b/wjqEp1Bv7l7qQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkChatUnread;
impl IconShape for MdMarkChatUnread {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,6.98V16c0,1.1-0.9,2-2,2H6l-4,4V4c0-1.1,0.9-2,2-2h10.1C14.04,2.32,14,2.66,14,3c0,2.76,2.24,5,5,5 C20.13,8,21.16,7.61,22,6.98z M16,3c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,1.34,16,3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkEmailRead;
impl IconShape for MdMarkEmailRead {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12,19c0-3.87,3.13-7,7-7c1.08,0,2.09,0.25,3,0.68V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h8.08 C12.03,19.67,12,19.34,12,19z M4,6l8,5l8-5v2l-8,5L4,8V6z M17.34,22l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24L23,16.34L17.34,22z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMarkEmailUnread;
impl IconShape for MdMarkEmailUnread {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,8.98V18c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h10.1C14.04,4.32,14,4.66,14,5 c0,1.48,0.65,2.79,1.67,3.71L12,11L4,6v2l8,5l5.3-3.32C17.84,9.88,18.4,10,19,10C20.13,10,21.16,9.61,22,8.98z M16,5 c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,3.34,16,5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxohH/vcyL5neA1+Y5IZ6/3wZeixeN+O9lXjS/A7w2zwnx/P028Fq8aMR/L/Oi+R3gtXlOiOfvt4HX4kUj/nuZF83vAK/Nc0I8f78NvBYvmt/mv9dr86L5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+JFI/57mRfN7wCvzXNCPH+/DbwWLxrx38u8aH4HeG2eE+L5+23gtXjRiP9e5kXzO8Br85wQz99vA6/Fi0b89zIvmt8BXpvnhHj+fht4LV404r+XedH8DvDaPCfE8/fbwGvxovlt/nu9Ni+a3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC1eNOK/l3nR/A7w2jwnxPP328Br8aIR/73Mi+Z3gNfmOSGev98GXosXjfjvZV40vwO8Ns8J8fz9NvBavGjEfy/zovkd4LV5Tojn77eB1+JFI/57mRfN7wCvzXNCPH+/DbwWL5rf5r/Xa/Oi+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+Rzgs3lOiOfvt4HX4v+WtwF+mueEeP5+G3gt/u/4G+CleV6I5++3gdfi/4a/Ad4auJXnhXj+fht4Lf53ewbw3cBXA7s8f4jn77eB1+Jf9j7Ad/O/F+L5+23gtXjh3gf4bv53Qzx/vw28Fi/Y+wDfzf9+iOfvt4HX4vl7H+C7+b8B8fz9NvBaPK/3Ab6b/zsQz99vA6/Fc3of4Lv5vwXx/P028Fo82/sA383/PYjn77eB1+KK9wG+m/+bEM/fbwOvBbwP8N3834V4/n4b+G7gu/m/DfH8vTTw1/zfh/j/DfH/G+L/N8T/b4j/3/hHExmCQUJxGf0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMessage;
impl IconShape for MdMessage {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 12H6v-2h12v2zm0-3H6V9h12v2zm0-3H6V6h12v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/xksDx/ivdQn4a/59EP8+rw18F/Bg/nvcCrwP8Nv82yD+7d4a+Cn+Z3gb4Kf510P82xwHng4c53+GXeAhwC7/Ooh/m/cGvov/WT4G+Gr+dRD/Nl8NfBT/s3wP8N786yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/Fc/ob4KP5r/HVwEvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/u3eG/huXjS/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG3+7X4buAi8D7DLC/fbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+bf7reB1wL+GngdYJcX7LeB1+I5/Q7w2vzrIP5tfht4LZ7T7wCvzb/dbwOvxRV/DbwOsMvz99vAa/Gcfgd4bf51EP82vw28Fs/pd4DX5l/vwcBLAV8NPJhn+2vgdYBdntdvA6/Fc/od4LX510H82/w28Fo8p98BXpsXzYOBjwLeGngwL9gu8DrAX/Ocfht4LZ7T7wCvzb8O4t/mt4HX4jn9DvDavHDHga8C3psX3S7wOsBf82y/DbwWz+l3gNfmXwfxb/PbwGvxnH4HeG1esLcGvgs4zr/eLvA6wF9zxW8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem+fvvYHv4t/uEvDawF9zxW8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem+f12sBv8W/3N8B7A3/Ns/028Fo8p98BXpt/HcS/zW8Dr8Vz+h3gtXlOx4GnA8f5t/kb4LWBXZ7TbwOvxXP6HeC1+ddB/Nv8NvBaPKffAV6b5/TdwHvxvH4H+G1gF/hrrvhq4KV4tr8BXhvY5Xn9NvBaPKffAV6bfx3Ev81vA6/Fc/od4LV5tgcDT+fZfgf4auCnef5+G3gtrvgb4LWBXZ6/3wZei+f0O8Br86+D+Lf5beC1eE6/A7w2z/bVwEcBzwDeG/htXrjfBl4L+B7gvXnhfht4LZ7T7wCvzb8O4t/mt4HX4jn9DvDaPNtF4GeAjwZ2+Zf9NnAr8N78y34beC2e0+8Ar82/DuLf5reB1+I5/Q7w2lzx2sB7A+/Ni+69ge/mRfPbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+aKtwZ+mv88vw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/od4LX5r/HbwGvxnH4HeG3+dRD/Nr8NvBbP6XeA1+a/xm8Dr8Vz+h3gtfnXQfzb/DbwWjyn3wFem/8avw28Fs/pd4DX5l8H8W/z28Br8Zx+B3ht/mv8NvBaPKffAV6bfx3Ev81vA6/Fc/pr4KP5r/HVwEvznH4HeG3+dRD/Nr8NvBb/s/wO8Nr86yD+bX4beC3+Z/kd4LX510H82/w28Fr8z/I7wGvzr4P4t/lt4LX4n+V3gNfmXwfxb/PbwGvxP8vvAK/Nvw7i3+argY/if5avAT6afx3Ev817A9/F/ywfA3w1/zqIf5vjwK3AMf5nuAQ8GNjlXwfxb/fWwE/xP8PbAD/Nvx7i3+e1ge8GHsR/j2cA7w38Nv82iP8YLw0c57/WLvDX/Psg/n9D/P/GPwII0wFQgF2O9QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMobileScreenShare;
impl IconShape for MdMobileScreenShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1.01L7 1c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14zm-4.2-5.78v1.75l3.2-2.99L12.8 9v1.7c-3.11.43-4.35 2.56-4.8 4.7 1.11-1.5 2.58-2.18 4.8-2.18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMoreTime;
impl IconShape for MdMoreTime {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                points: "10,8 10,14 14.7,16.9 15.5,15.7 11.5,13.3 11.5,8",
            }
            path {
                d: "M17.92,12c0.05,0.33,0.08,0.66,0.08,1c0,3.9-3.1,7-7,7s-7-3.1-7-7c0-3.9,3.1-7,7-7c0.7,0,1.37,0.1,2,0.29V4.23 C12.36,4.08,11.69,4,11,4c-5,0-9,4-9,9s4,9,9,9s9-4,9-9c0-0.34-0.02-0.67-0.06-1H17.92z",
            }
            polygon {
                points: "20,5 20,2 18,2 18,5 15,5 15,7 18,7 18,10 20,10 20,7 23,7 23,5",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNat;
impl IconShape for MdNat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.82,13H11v-2H6.82C6.4,9.84,5.3,9,4,9c-1.66,0-3,1.34-3,3s1.34,3,3,3C5.3,15,6.4,14.16,6.82,13z M4,13 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C5,12.55,4.55,13,4,13z",
            }
            path {
                d: "M23,12l-4-3v2h-4.05C14.45,5.95,10.19,2,5,2v2c4.42,0,8,3.58,8,8s-3.58,8-8,8v2c5.19,0,9.45-3.95,9.95-9H19v2L23,12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF1ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3Ec+C3gpfmPcyvw3cDXALv82yD+8x0Hfgt4af5z/DXwNsCt/OshXjRvDfw2sMu/znHgt4CX5j/XbwOvw78e4l/23sB3AX8NvA6wy4vmOPBbwEvzX+NtgJ/mXwfxwr038F08218DrwPs8sIdB34LeGn+63wO8Nn86yBesLcGforn9dfA6wC7PH/Hgd8CXpr/Wj8DvDX/OogX7Djw28BL8bz+GngdYJfn9dbAT/Ff73eA1+ZfB/HCHQd+G3gpntdfA68D7PK83hv4Lv5r/Q7w2vzrIP5lx4HfBl6K5/XXwOsAuzyv9wa+i/86vwO8Nv86iBfNceC3gZfief018DrALs/rvYHv4r/G7wCvzb8O4kV3HPht4KV4Xn8NvA6wy/N6b+C7+M/3O8Br86+D+Nc5Dvw28FI8r78GXgfY5Xm9N/Bd/Of6HeC1+ddB/OsdB/4aeBDP66+B1wF2eV7vDXwX/3l+B3ht/nUQ/za/DbwWz99fA68D7PK83hv4Lv5z/A7w2vzrIP5tfht4LV6wvwZeB9jleb038F38x/sd4LX510H82/w28Fq8cH8NvA6wy/N6b+C7+I/1O8Br86+D+Lf5beC1+Jf9NfA6wC7P672B7+I/zu8Ar82/DuLf5reB1+JF89fA6wC7PK/3Br6L/xi/A7w2/zqIf5vfBl6LF91fA68D7PK83hv4Lv79fgd4bf51EP82vw28Fv86fw28DrDL83pv4Lv49/kd4LX510H82/w28Fr86/018DrALs/rvYHv4t/ud4DX5l8H8W/z28Br8W/z18DrALs8r/cGvot/m98BXpt/HcS/zW8Dr8W/3V8DrwPs8rzeG/gu/vV+B3ht/nUQ/za/DbwW/z5/DbwOsMvzem/gu/jX+R3gtfnXQfzb/DbwWvz7/TXwOsAuz+u9ge/iRfc7wGvzr4P4t/lt4LX4j/HXwOsAuzyv9wa+ixfN7wCvzb8O4t/mt4HX4j/OXwOvA+zyvN4b+C7+Zb8DvDb/Ooh/m98GXov/WH8NvA6wy/N6b+C7eOF+B3ht/nUQ/za/DbwW//H+GngdYJfn9d7Ad/GC/Q7w2vzrIP5tfht4Lf5z/DXwOsAuz+u9ge/i+fsd4LX510H82/w28Fr85/lr4HWAXZ7XewPfxfP6HeC1+ddB/Nv8NvBa/Of6a+B1gF2e13sD38Vz+h3gtfnXQfzb/DbwWvzn+2vgdYBdntd7A9/Fs/0O8Nr86yD+bX4beC3+a/w18DrALs/rvYHv4orfAV6bfx3Ev81vA6/Ff52/Bl4H2OV5vTfwXcDvAK/Nvw7i3+a3gdfiv9ZfA68D7PK83ht4b+C1+ddB/Nv8NvBa/Nf7a+B1gF2e10sDf82/DuLf5reB1+K/x18DrwPs8u+H+Lf5beC1+O/z18DrALv8+yD+bX4beC3+e/018DrALv92iH+b3wZei/9+fw28DrDLvw3i3+a3gdfif4a/Bl4H2OVfD/Fv89vAa/E/x18DrwPs8q+D+Lf5bOCz+J/lr4HXAXZ50SH+bd4a+Cn+5/lr4HWAXV40iH+7vwZeiv95/hp4HWCXfxni3+7BwE8DL8X/PH8NvA6wywuH+Pc5Dnw08N7Ag/iv9TfATwNvDbwUz+uvgdcBdnnBEP/7HQd+G3gpntdfA68D7PL8If5vOA78NvBSPKfvAd6bFwzxf8dx4LeBl+KK7wHemxcO8X/LceC3gb8G3pt/GeL/nuPALi8axP9viP/fEP+/8Y+As+BBFCKrWAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoSim;
impl IconShape for MdNoSim {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf59fovn9THAX/OcXhr4Kp7X6/DvY57X6wC/DSCe7bWB3+J5iX8f87xeB/htntNrA7/F8xL/PuZ5vQ7w2wDi2V4b+C2el/j3Mc/rdYDf5jm9NvBbPC/x72Oe1+sAvw0gnu21gd/ieYl/H/O8Xgf4bZ7TawO/xfMS/z7meb0O8NsA4tleG/gtnpf49zHP63WA3+Y5vTbwWzwv8e9jntfrAL8NIJ7ttYHf4nmJfx/zvF4H+G2e02sDv8XzEv8+5nm9DvDbAOLZXhv4LZ6X+Pcxz+t1gN/mOb028Fs8L/HvY57X6wC/DSCe7bWB3+J5iX8f87xeB/htntNrA7/F8xL/PuZ5vQ7w2wDi2V4b+C2el/j3Mc/rdYDf5jm9NvBbPC/x72Oe1+sAvw0gnu21gd/ieYl/H/O8Xgf4bZ7TawO/xfMS/z7meb0O8NsA4tleG/gtnpf49zHP63WA3+Y5vTbwWzwv8e9jntfrAL8NIJ7ttYHf4nmJfx/zvF4H+G2e02sDv8XzEv8+5nm9DvDbAOLZXhv4LZ6X+Pcxz+t1gN/mOb028Fs8L/HvY57X6wC/DSCe7bWB3+J5iX8f87xeB/htntNrA7/F8xL/PuZ5vQ7w2wDi2V4b+C2el/j3Mc/rdYDf5jm9NvBbPC/x72Oe1+sAvw0gnu21gd/ieYl/H/O8Xgf4bZ7TawO/xfMS/z7meb0O8NsA4tleG/gtnpf49zHP63WA3+Y5vTbwWzwv8e9jntfrAL8NIJ7ttYHf4nmJfx/zvF4H+G2e02sDv8XzEv8+5nm9DvDbAOLZXhv4LZ6X+Pcxz+t1gN/mOb028Fs8L/HvY57X6wC/DSCe7bWB3+J5iX8f87xeB/htntNrA7/F8xL/PuZ5vQ7w2wDi2V4b+C2el/j3Mc/rdYDf5jm9NvBbPC/x72Oe1+sAvw0gnu21gd/ieYl/n9/meX008Nc8p5cGvprn9dr8+5jn9TrAbwOIZ3tt4Ld4XuJ/N/O8Xgf4bQDxbK8N/BbPS/zvZp7X6wC/DSCe7bWB3+J5if/dzPN6HeC3AcSzvTbwWzwv8b+beV6vA/w2gHi21wZ+i+cl/nczz+t1gN8GEM/22sBv8bzE/27meb0O8NsA4tleG/gtnpf43808r9cBfhtAPNtrA7/F8xL/u5nn9TrAbwOIZ3tt4Lf4/+F1gN8GEM9pFzjG/22XgONcgXhOnw18Fv+3fQ7w2VyBeF7fDbwX/zd9D/DePBvi+Xtv4KOBl+L/ht8Bvhv4bp4T4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IqhXKQXi88dUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPausePresentation;
impl IconShape for MdPausePresentation {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 19.1H3V5h18v14.1zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
            path {
                d: "M21 19.1H3V5h18v14.1zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
            path {
                d: "M9 8h2v8H9zm4 0h2v8h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonAddDisabled;
impl IconShape for MdPersonAddDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "15",
                cy: "8",
                r: "4",
            }
            path {
                d: "M23 20v-2c0-2.3-4.1-3.7-6.9-3.9l6 5.9h.9zm-11.6-5.5C9.2 15.1 7 16.3 7 18v2h9.9l4 4 1.3-1.3-21-20.9L0 3.1l4 4V10H1v2h3v3h2v-3h2.9l2.5 2.5zM6 10v-.9l.9.9H6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9+DgQfxbJeAv+Z/BsR/juPAewHvDbw0z+tW4LeBzwFu5b8P4j/eWwPfBRznRfPVwMfw3wPxH+urgI/mX++vgdcBdvmvhfiP89HAV/Fv99fAy/BfC/Ef47WB3+Lf72uAj+a/DuI/xm8Br81/jIcAt/JfA/Hv99bAT/Ef53uA9+a/BuLf76uBj+I/zq3AQ/ivgfj3+23gtfiP9TLAX/OfD/HvZ/7jvQ7w2/znQ/z7mf94rwP8Nv/5EP9+twIP4j/W6wC/zX8+xL/fbwOvxX+sE8Au//kQ/34fDXwV/3H+Bnhp/msg/v0eDDyd/zgfA3w1/zUQ/zG+G3gv/v2eAbw0sMt/DcR/jOPArcAx/n3eBvhp/usg/uO8NPBX/Nt9DvDZ/NdC/Md6aeC3gWP863wM8NX810P8xzsOfDXwXvzLfgf4bOC3+e+B+M/zYOCtgbcGHgw8iCt+B/hr4KeB3+a/F+J/rgcDbwW8NfBg4MFc8dvAXwO/DfwM/z6I/3mOA18FvDf/st8GPgf4bf5tEP+zvDTwW8Bx/nU+Gvga/vUQ/3O8NPBX/Nt9DvDZ/Osg/mc4DjwdOM6/z9sAP82LDvGf46WBBwEvzRXHgZcGvgf4bp7XdwPvxfP3N8BPA38NvDTw0sBb8fzdCrwMsMuLBvHvdxx4LeCtgZcGXpoX7HWA3+Y5PRh4Os/fxwBfzfN6aeCngQfxvD4G+GpeNIh/m+PAewHvDbw0LzrxvD4a+Cqe18cAX80L9tLAbwPHeE5/DbwMLxrEv86Dgc8C3pt/vb8BXprn9dvAa/Gc/gZ4af5lnw18Fs/rBLDLvwzxonkw8FnAe/Nv9zvAa/O8ng48mOf0OcBn8y97MPB0ntfrAL/NvwzxL3tv4KuA4/z7/A7w2jwv87zeBvhpXjTmeb0O8Nv8yxAv3HsD38V/jN8BXpvnZZ7X5wCfzb/sOHCR5/U6wG/zL0O8YO8NfBf/cX4HeG2e128Dr8Vz+hngrfmXvTfwXTyvhwC38i9DPH/HgacDx/mPswuc4Hl9NfBRPK+XAf6aF+7pwIN5TpeA47xoEM/fRwNfxX+8hwC38pzeGvgpntetwNsAf83z913Ae/O8vgb4aF40iOfvq4GP4j/e+wDfzfP6beC1eF67wFcD3wPcChwH3gr4bODBPH8PAW7lRYN4/n4beC3+4/028Do8r9cGfov/GO8DfDcvGsTz99vAa/Gf43WA3+Z5fTTwVfzHeB/gu/mXIZ6/3wZei/8cvw28Ds/fZwOfxX+M9wG+mxcO8fz9NvBa/Od5G+Cnef7eGvhu4Bj/fu8DfDcvGOL5+23gtfjPswu8DvDXPH/HgfcG3ht4KZ7XJeC7ga8GXhv4Ll6w9wG+m+cP8fy9NHCc/1y3ArfyLzsOvDTPditwK8/pvYHv4gV7H+C7eV6I/zveG/guXrD3Ab6b54T4v+W9ge/ieV0CXhv4a54T4v+e9wa+i2e7BLw28Nc8L8T/Te8NfBdwCXht4K95/hD/d7038NfAX/OCIf5/Q/z/hvj/DfH/G+L/N/4Rwg6xQWjJu7sAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonSearch;
impl IconShape for MdPersonSearch {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "10",
                cy: "8",
                r: "4",
            }
            path {
                d: "M10.35,14.01C7.62,13.91,2,15.27,2,18v2h9.54C9.07,17.24,10.31,14.11,10.35,14.01z",
            }
            path {
                d: "M19.43,18.02C19.79,17.43,20,16.74,20,16c0-2.21-1.79-4-4-4s-4,1.79-4,4c0,2.21,1.79,4,4,4c0.74,0,1.43-0.22,2.02-0.57 L20.59,22L22,20.59L19.43,18.02z M16,18c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C18,17.1,17.1,18,16,18z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO//Mh/mXfDbwX/3pvA/w0/7Mh/mXm3+Z7gPfmfzbEv8z82/wO8Nr8z4b4l5l/m98BXpv/2RD/sr8GXop/vd8BXpv/2RD/st8GXot/G/E/G+Jf9tvAa/FvI/5nQ/zLvhr4KP5tXgf4bf7nQvzLPhv4LP5tPgb4av7nQvzLXhv4Lf5tfgZ4a/7nQvzLHgw8nX878T8X4kVzK/Ag/m1eB/ht/mdCvGi+G3gv/m1eB/ht/mdCvGjeG/gu/vX+Bnhp/udCvGgeDDydf52/AV4b2OV/LsSL7lbgQbxo/gZ4bWCX/9kQL7qPBr6Kf9nfAK8N7PI/H+JF92Dg6bxwfwO8NrDL/w6If53fBl6L5+9vgNcGdvnfA/Gv897Ad/G8ngG8NLDL/y6If71bgQfxnHaBlwFu5X8XxL/eewPfxfP6HuC9+d8F8W9zK/AgntfLAH/N/x6If5v3Br6L5/XXwMvwvwfi3+63gdfieX0O8Nn874D4t3tp4K94/l4H+G3+50P8+3w18FE8r13gIcAu/7Mh/n2OA78NvBTP66+B1wF2+Z8L8e/30sBvA8d4Xt8NvA//cyH+Y7w38F08f98NvA//MyH+43w38F48f98NvA//Pq/FFb/DfxzEf6zfBl6L5++ngfcBdvnXOQ78FvDSXLEL/Dbw28BfA7/Dvx3iP9Zx4LeBl+L5+2vgdYBdXjTHgd8CXpoXbhf4a57tVuBngJ/mhUP8xzsO/DbwUjx/u8DbAL/NC3cc+C3gpfm3+xjgq3nBEP85jgO/DbwUL9hnA5/D83cc+C3gpfn3uRV4CC8Y4j/PceCngdfiBftr4H2Av+bZjgO/Bbw0/zHEC4b4z/fdwHvxwn038DnALvBbwEvzH0e8YIj/Gu8NfDVwjBdsF9gFHsx/LPGCIf7rvDTw3cBL8V9LvGCI/1rHgc8GPor/OuIFQ/z3eGngq4HX4j+feMEQ/73eG/hs4EH85xEvGOJ/hvcGPht4EP/xxAuG+J/lvYG3Bt6K/zjiBUP8z/Rg4K2BjwYexL+PeMEQ//M9GHht4LWB1wYexIvue4D35gVD/O/zYODBwEsDDwZemiuOAy/Fs30P8NHALi8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IAJR4QS14xVMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhone;
impl IconShape for MdPhone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneDisabled;
impl IconShape for MdPhoneDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.34 14.54l-1.43-1.43c.56-.73 1.05-1.5 1.47-2.32l-2.2-2.2c-.28-.28-.36-.67-.25-1.02.37-1.12.57-2.32.57-3.57 0-.55.45-1 1-1H20c.55 0 1 .45 1 1 0 3.98-1.37 7.64-3.66 10.54zm-2.82 2.81C11.63 19.64 7.97 21 4 21c-.55 0-1-.45-1-1v-3.49c0-.55.45-1 1-1 1.24 0 2.45-.2 3.57-.57.35-.12.75-.03 1.02.24l2.2 2.2c.81-.42 1.58-.9 2.3-1.46L1.39 4.22l1.42-1.41L21.19 21.2l-1.41 1.41-5.26-5.26z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+OzgI8GjvOi+W7gY4BdXjDE/w5vDfwU/3rfA7w3Lxjif4fvBt6LfxvxgiH+d/ht4LX4txEvGOJ/h98GXot/G/GCIf53+G3gtfjX+x3gtXnBEP87mH+b3wFemxcM8b+D+bf5HeC1ecEQ//O9NvBb/Nt8DfDRvGCI//k+G/gs/m0+BvhqXjDE/3w/DbwV/zavA/w2Lxjifz7zb/cQ4FZeMMT/bK8N/Bb/Ns8AHswLh/if7bWB3+Lf5nuA9+aFQ/zP99fAS/Gv9z7Ad/PCIf7nOw78NvBS/Os8BLiVFw7xv8Nx4LeBl+JF8wzgwfzLEP97HAd+G3gp/mUfA3w1/zLE/y7Hgd8GXooX7iHArfzLEP/7HAd+G3gpnr+fAd6aFw3if6fjwF8DD+J5vQ/w3bxoEP87PRj4K+A4z+kZwIN50SH+d/pu4L14Xu8DfDcvOsT/Pi8N/BXP6xnAg/nXQfzv81fAS/O83gf4bv51EP+7fDbwWTyv3wFem389xP8erw38Fs/fywB/zb8e4n+HBwN/BRzneX0N8NH82yD+5zsO/Bbw0jyvvwFeG9jl3wbxP99PAW/N87oEvDbw1/zbIf5n+y7gvXn+3gf4bv59EP/xXosrfod/n+8C3pvn73uA9+bfD/Hv91rASwOvDbw2cJwr/hp4HWCXf53jwHcBb83z9zPAW/MfA/GieWvgrYAH82wvDRznhftr4HWAXV40DwZ+Cnhpnr+/AV4b2OU/BuJf9tnAZ/Fv99fA6wC7vHCvDfwUcJzn72+A1wZ2+Y+D+Jc9HXgw/z5/DbwOsMvz91nAZ/OC/Q3w2sAu/7EQ/zLzH+OvgdcBdnm21wa+CnhpXrCfAd4b2OU/HuJfZv7j/DXwOsBx4LOA9+aF+x7gvfnPg/iXmf9YtwLHgeO8YJeAjwa+m/9ciH+Z+a/1N8B7A3/Nfz7Ev8z81/ka4LOBXf5rIP5l5j/f7wAfDfw1/7UQ/zLzn+cZwGcD381/D8S/zPzHewbw2cB3898L8S8z/3F+Bvhp4Lv5nwHxLzP/Ps8Avhr4aeBW/mdB/Mu+G3gvXnTPAH4b+G3gt4Fb+Z8L8S87Dnw18F48298Au1zx18CtwF8DtwK38r8H4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IHQeIQUXWzB0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneEnabled;
impl IconShape for MdPhoneEnabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.38 10.79l-2.2-2.2c-.28-.28-.36-.67-.25-1.02.37-1.12.57-2.32.57-3.57 0-.55.45-1 1-1H20c.55 0 1 .45 1 1 0 9.39-7.61 17-17 17-.55 0-1-.45-1-1v-3.49c0-.55.45-1 1-1 1.24 0 2.45-.2 3.57-.57.35-.12.75-.03 1.02.24l2.2 2.2c2.83-1.45 5.15-3.76 6.59-6.59z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelinkErase;
impl IconShape for MdPhonelinkErase {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 8.2l-1-1-4 4-4-4-1 1 4 4-4 4 1 1 4-4 4 4 1-1-4-4 4-4zM19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFa0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+I/z4OBB/Ff73d40SH+4x0Hvgt4a/577AJfDXwO/zLEf6zjwNOB4/z3+27gfXjhEP+xfgp4a/7neB/gu3nBEP9xHgw8nf9Zfgd4bV4wxH+c1wZ+i/95xAuG+I/z2sBv8T+PeMEQ/3FeG/gt/ucRLxjiP85rA7/F8/od/mscB16K5yVeMMR/nNcGfovnJf5rvDbwWzwv8YIh/uO8NvBbPC/xX+O1gd/ieYkXDPEf57WB3+J5if8arw38Fs9LvGCI/zivDfwWz0v813ht4Ld4XuIFQ/zHeW3gt3he4r/GawO/xfMSLxji/47XBn6L5yVeMMT/Ha8N/BbPS7xgiP9Yx4GX4tl+h/86rw38Fs9LvGCI/xivDXwW8No8r+8GPge4lf9crw38Fs9LvGCIf7/vAt6bF24X+Bjgu/nP89rAb/G8xAuG+Pf5LuC9eV6XgGM8r7cBfpr/HK8N/BbPS7xgiH+7twZ+imf7G+Cjgd/m2T4b+GjgGFfsAg8BdvmP99rAb/G8xAuG+Lf7K+ClueIZwEsDuzyvtwZ+imf7GOCr+Y/32sBv8bzEC4b4tzPP9jnAZ/OC3Qo8iCt+B3ht/uO9NvBbPC/xgiH+bV4a+Cue7W2An+YF+23gtbjid4DX5j/eawO/xfMSLxji3+a1gd/i2V4H+G1esN8GXosrfgd4bf7jvTbwWzwv8YIh/m1eG/gtnu11gN/mBftt4LW44neA1+Y/3msDv8XzEi8Y4t/mtYHf4tleB/htXrDfBl6LK34HeG3+47028Fs8L/GCIf71Xht4L+C9eba/BnZ5wV4aOM4Vu8BXA18D7PIf57WB3+J5iRcM8a/z3sB38R9jF3gZ4Fb+Y7w28Fs8L/GCIV50Dwaezn+snwHemv8Yrw38Fs9LvGCIF91rA7/Ff6xd4AT/MV4b+C2el3jBEC+61wZ+i/944j/GawO/xfMSLxjiRffawG/xH0/8x3ht4Ld4XuIFQ7zoXhv4Lf7jif8Yrw38Fs9LvGCIF91rA7/FfzzxH+O1gd/ieYkXDPGie23gt/iPJ/5jvDbwWzwv8YIhXnSvDfwW//HEf4zXBn6L5yVeMMSL7rWB3+I/nviP8drAb/G8xAuGeNG9NvBb/McT/zFeG/gtnpd4wRAvutcGfov/eOI/xmsDv8XzEi8Y4kX32sBv8R/rEnCc/xivDfwWz0u8YIh/nb8GXor/OF8DfDT/MV4b+C2el3jBEP86Lw38NPAg/v1+Bnhr/uO8NvBbPC/xgiH+bV6bf59bgVv5j/XawG/xvMQLhvi/47WB3+J5iRcM8X/HawO/xfMSLxjiP85rA7/F8xL/NV4b+C2el3jBEP9xXhv4LZ6X+K/x2sBv8bzEC4b4j/PawG/xvMR/jdcGfovnJV4wxH+c1wZ+i+cl/mu8NvBbPC/xgiH+47w28Fs8L/Ff47WB3+J5iRcM8R/ntYHf4nn9Nv81jgMvzfMSLxjiP85rA7/F/zziBUP8x3lt4Lf4n0e8YIj/OK8N/Bb/84gXDPEf58HA0/mf5XeA1+YFQ/zH+mngrfif432A7+YFQ/zHOg7cChzjv9/3AO/NC4f4j3cc+G7grfjvcQn4auCz+Zch/vM8GHgw//V+mxcd4v83xP9v/COrU5lByhnhhgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelinkLock;
impl IconShape for MdPhonelinkLock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm-8.2 10V9.5C10.8 8.1 9.4 7 8 7S5.2 8.1 5.2 9.5V11c-.6 0-1.2.6-1.2 1.2v3.5c0 .7.6 1.3 1.2 1.3h5.5c.7 0 1.3-.6 1.3-1.2v-3.5c0-.7-.6-1.3-1.2-1.3zm-1.3 0h-3V9.5c0-.8.7-1.3 1.5-1.3s1.5.5 1.5 1.3V11z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelinkRing;
impl IconShape for MdPhonelinkRing {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20.1 7.7l-1 1c1.8 1.8 1.8 4.6 0 6.5l1 1c2.5-2.3 2.5-6.1 0-8.5zM18 9.8l-1 1c.5.7.5 1.6 0 2.3l1 1c1.2-1.2 1.2-3 0-4.3zM14 1H4c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 19H4V4h10v16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonelinkSetup;
impl IconShape for MdPhonelinkSetup {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.82 12.49c.02-.16.04-.32.04-.49 0-.17-.02-.33-.04-.49l1.08-.82c.1-.07.12-.21.06-.32l-1.03-1.73c-.06-.11-.2-.15-.31-.11l-1.28.5c-.27-.2-.56-.36-.87-.49l-.2-1.33c0-.12-.11-.21-.24-.21H5.98c-.13 0-.24.09-.26.21l-.2 1.32c-.31.12-.6.3-.87.49l-1.28-.5c-.12-.05-.25 0-.31.11l-1.03 1.73c-.06.12-.03.25.07.33l1.08.82c-.02.16-.03.33-.03.49 0 .17.02.33.04.49l-1.09.83c-.1.07-.12.21-.06.32l1.03 1.73c.06.11.2.15.31.11l1.28-.5c.27.2.56.36.87.49l.2 1.32c.01.12.12.21.25.21h2.06c.13 0 .24-.09.25-.21l.2-1.32c.31-.12.6-.3.87-.49l1.28.5c.12.05.25 0 .31-.11l1.03-1.73c.06-.11.04-.24-.06-.32l-1.1-.83zM7 13.75c-.99 0-1.8-.78-1.8-1.75s.81-1.75 1.8-1.75 1.8.78 1.8 1.75S8 13.75 7 13.75zM18 1.01L8 1c-1.1 0-2 .9-2 2v3h2V5h10v14H8v-1H6v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPortableWifiOff;
impl IconShape for MdPortableWifiOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.56 14.24c.28-.69.44-1.45.44-2.24 0-3.31-2.69-6-6-6-.79 0-1.55.16-2.24.44l1.62 1.62c.2-.03.41-.06.62-.06 2.21 0 4 1.79 4 4 0 .21-.02.42-.05.63l1.61 1.61zM12 4c4.42 0 8 3.58 8 8 0 1.35-.35 2.62-.95 3.74l1.47 1.47C21.46 15.69 22 13.91 22 12c0-5.52-4.48-10-10-10-1.91 0-3.69.55-5.21 1.47l1.46 1.46C9.37 4.34 10.65 4 12 4zM3.27 2.5L2 3.77l2.1 2.1C2.79 7.57 2 9.69 2 12c0 3.7 2.01 6.92 4.99 8.65l1-1.73C5.61 17.53 4 14.96 4 12c0-1.76.57-3.38 1.53-4.69l1.43 1.44C6.36 9.68 6 10.8 6 12c0 2.22 1.21 4.15 3 5.19l1-1.74c-1.19-.7-2-1.97-2-3.45 0-.65.17-1.25.44-1.79l1.58 1.58L10 12c0 1.1.9 2 2 2l.21-.02.01.01 7.51 7.51L21 20.23 4.27 3.5l-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFJUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf7zvDRX/DX/eczzeh3gtwHEs7028Fs8L/Gf46WB3+KK1wH+mv8c5nm9DvDbAOLZXhv4LZ6X+I/30sBvAce5Yhd4HeCv+Y9nntfrAL8NIJ7ttYHf4nmJ/1gvDfwWcJzntAu8DvDX/Mcyz+t1gN8GEM/22sBv8bzEf5yXBn4LOM7ztwu8DvDX/Mcxz+t1gN8GEM/22sBv8bzEf4yXBn4LOM4Ltwu8DvDX/Mcwz+t1gN8GEM/22sBv8bzEv99LA78FHOdFswu8DvDX/PuZ5/U6wG8DiGd7beC3eF7i3+elgd8CjvOvswu8DvDX/PuY5/U6wG8DiGd7beC3eF7i3+6lgd8CjvNvswu8DvDX/NuZ5/U6wG8DiGd7beC3eF7i3+algd8CjvPvswu8DvDX/NuY5/U6wG8DiGd7beC3eF7iX++lgd8CjvMfYxd4HeCv+dczz+t1gN8GEM/22sBv8bzEf4zXBn6LF83rAL/NfwzzvF4H+G0A8WyvDfwWz0v8x3ht4Ld40bwO8Nv8xzDP63WA3wYQz/bawG/xvMR/jNcGfosXzesAv81/DPO8Xgf4bQDxbK8N/BbPS/zHeG3gt3jRvA7w2/zHMM/rdYDfBhDP9trAb/G8xH+M1wZ+ixfN6wC/zX8M87xeB/htAPFsrw38Fs9L/Md4beC3eNG8DvDb/Mcwz+t1gN8GEM/22sBv8bzEf4zXBn6LF83rAL/NfwzzvF4H+G0A8WyvDfwWz0v8x3ht4Ld40bwO8Nv8xzDP63WA3wYQz/bawG/xvMR/jNcGfosXzesAv81/DPO8Xgf4bQDxbK8N/BbPS/zHeG3gt3jRvA7w2/zHMM/rdYDfBhDP9trAb/G8xH+Mlwa+mhfNRwN/zX8M87xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3her83/br/N83od4LcBxLO9NvBb/P/wOsBvA4jntAsc4/+2S8BxrkA8p88GPov/2z4H+GyuQDyv7wbei/+bvgd4b54N8fy9N/DRwEvxf8PvAN8NfDfPCfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BNuMvEGLZdiQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPresentToAll;
impl IconShape for MdPresentToAll {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.11 0-2 .89-2 2v14c0 1.11.89 2 2 2h18c1.11 0 2-.89 2-2V5c0-1.11-.89-2-2-2zm0 16.02H3V4.98h18v14.04zM10 12H8l4-4 4 4h-2v4h-4v-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPrintDisabled;
impl IconShape for MdPrintDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.1 17H22v-6c0-1.7-1.3-3-3-3h-9l9.1 9zm-.1-7c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-1-3V3H6v1.1L9 7zM1.2 1.8L0 3l4.9 5C3.3 8.1 2 9.4 2 11v6h4v4h11.9l3 3 1.3-1.3-21-20.9zM8 19v-5h2.9l5 5H8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQrCode;
impl IconShape for MdQrCode {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3,11h8V3H3V11z M5,5h4v4H5V5z",
            }
            path {
                d: "M3,21h8v-8H3V21z M5,15h4v4H5V15z",
            }
            path {
                d: "M13,3v8h8V3H13z M19,9h-4V5h4V9z",
            }
            rect {
                height: "2",
                width: "2",
                x: "19",
                y: "19",
            }
            rect {
                height: "2",
                width: "2",
                x: "13",
                y: "13",
            }
            rect {
                height: "2",
                width: "2",
                x: "15",
                y: "15",
            }
            rect {
                height: "2",
                width: "2",
                x: "13",
                y: "17",
            }
            rect {
                height: "2",
                width: "2",
                x: "15",
                y: "19",
            }
            rect {
                height: "2",
                width: "2",
                x: "17",
                y: "17",
            }
            rect {
                height: "2",
                width: "2",
                x: "17",
                y: "13",
            }
            rect {
                height: "2",
                width: "2",
                x: "19",
                y: "15",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdQrCodeScanner;
impl IconShape for MdQrCodeScanner {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.5,6.5v3h-3v-3H9.5 M11,5H5v6h6V5L11,5z M9.5,14.5v3h-3v-3H9.5 M11,13H5v6h6V13L11,13z M17.5,6.5v3h-3v-3H17.5 M19,5h-6v6 h6V5L19,5z M13,13h1.5v1.5H13V13z M14.5,14.5H16V16h-1.5V14.5z M16,13h1.5v1.5H16V13z M13,16h1.5v1.5H13V16z M14.5,17.5H16V19h-1.5 V17.5z M16,16h1.5v1.5H16V16z M17.5,14.5H19V16h-1.5V14.5z M17.5,17.5H19V19h-1.5V17.5z M22,7h-2V4h-3V2h5V7z M22,22v-5h-2v3h-3v2 H22z M2,22h5v-2H4v-3H2V22z M2,2v5h2V4h3V2H2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADlUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4j/HcWCXf53jwEvxn+cS8Nc8J8R/jp8GPhv4a150rw38Fv95fgd4bZ4T4j/HbwMvBbwO8Ne8aF4b+C3+8/wO8No8J8R/jt8GXgvYBV4H+Gv+Za8N/Bb/eX4HeG2eE+I/x28Dr8UVu8DrAH/NC/fawG/xn+d3gNfmOSH+c/w28Fo82y7wOsBf84K9NvBb/Of5HeC1eU6I/xy/DbwWz2kXeB3gr/mfA/Gf47eB1+J57QKvA/w1/zMg/nP8NvBaPH+7wOsAf81/P8R/jt8GXosXbBd4HeCv+e+F+M/x28Br8cLtAq8D/DX/fRDP31cDL8W/3UsDx/mX7QKvA/w1/7KXBr6Kf7u/AT6a54R4/n4beC3+a+wCrwP8NS/cawO/xb/d7wCvzXNCPH+/DbwW/3V2gdcB/poX7LWB3+Lf7neA1+Y5IZ6/3wZei/86l4DXBv6aF+y1gd/i3+53gNfmOSGev98GXov/GpeA1wb+mhfutYHf4t/ud4DX5jkhnr+vBl6af7uXBo7xL7sEvDbw1/zLXhr4av7t/hr4aJ4T4j/HbwOvxQt3CXht4K/574P4z/HbwGvxgl0CXhv4a/57If5z/DbwWjx/l4DXBv6a/36I/xy/DbwWz+sS8NrAX/M/A+I/x28Dr8VzugS8NvDX/M+B+M/x28Br8WyXgNcG/poX7LWB3+I/z+8Ar81zQvzn+G3gtbjiEvDawF/zwr028Fv85/kd4LV5Toj/HL8NvBZwCXht4K/5l7028Fv85/kd4LV5Toj/HL8NvDTw2sBf86J5beC3+M/zO8Br85wQ/zl+Gvhs4K950b028Fv85/kd4LV5Toj/HMeBXf51jgMvzX+eXeCveU6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPxmxwQc863NUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdReadMore;
impl IconShape for MdReadMore {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                width: "9",
                x: "13",
                y: "7",
            }
            rect {
                height: "2",
                width: "9",
                x: "13",
                y: "15",
            }
            rect {
                height: "2",
                width: "6",
                x: "16",
                y: "11",
            }
            polygon {
                points: "13,12 8,7 8,11 2,11 2,13 8,13 8,17",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRingVolume;
impl IconShape for MdRingVolume {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M23.71 16.67C20.66 13.78 16.54 12 12 12 7.46 12 3.34 13.78.29 16.67c-.18.18-.29.43-.29.71 0 .28.11.53.29.71l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.11.7-.28.79-.74 1.69-1.36 2.66-1.85.33-.16.56-.5.56-.9v-3.1c1.45-.48 3-.73 4.6-.73s3.15.25 4.6.72v3.1c0 .39.23.74.56.9.98.49 1.87 1.12 2.66 1.85.18.18.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71 0-.27-.11-.52-.29-.7zM21.16 6.26l-1.41-1.41-3.56 3.55 1.41 1.41s3.45-3.52 3.56-3.55zM13 2h-2v5h2V2zM6.4 9.81L7.81 8.4 4.26 4.84 2.84 6.26c.11.03 3.56 3.55 3.56 3.55z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFdUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEi+44V+zyfwfiRffawG/xnG4FbgV+G9gFfgf4a/73QLzoXhv4Lf5lu8BvA78N/AxwK/9zIV50rw38Fv96vw18N/AzwC7/syBedK8N/Bb/drvATwOfA9zK/wyIF91rA7/Ff4zvBj4HuJX/XogX3WsDv8V/rM8GvgbY5b8H4kX32sBv8R/vVuB9gN/mvx7iRfdg4L15tgcDDwZeGjjGv99XAx/Dfy3Ef4zjwGsDrw28NvBS/Nv8NfA+wF/zXwPxn+OlgfcG3hp4EP86u8D7AD/Nfz7Ef773Bj4beBD/Ou8DfDf/uRD/dd4b+GzgQbzo3gf4bv7zIP7rfTbw0cAxXjTvA3w3/zkQ/z0eDPw08FK8aN4H+G7+4yH+e3018FG8aN4G+Gn+YyFedK8N/BZX7AJ/DdwK/DbwO8Ct/Nu8N/Bd/Mt2gZcBbuU/DuJF99rAb/GC3Qr8NPA1wK3867w18N3AMV64vwZehv84iBfdawO/xYvmt4HPAX6bF91LA78NHOOF+xrgo/mPgXjRvTbwW/zr/DbwPsCtvGjeGvgp/mWvA/w2/36IF91rA7/Fv81nA5/Di+a9ge/ihbsVeAj/fogX3WsDv8W/3W8DbwPs8i/7auCjeOE+B/hs/n0QL7rXBn6Lf59d4HWAv+Zf9tfAS/HCPQS4lX87xIvuwcB7Ay8NHAceDDyIf71d4HWAv+aFezDw18AxXrDvAd6bfzvEv89LA28NvDXwUrzodoHXAf6aF+6zgc/ihXsIcCv/Noj/OK8NfDbwWrxodoGHALu8cLcCD+IF+xrgo/m3QfzHe2vgq4EH8S/7beB1eOHeG/guXrBd4CHALv96iP8cx4HfBl6Kf9nnAJ/NC3cr8CBesPcBvpt/PcR/ru8G3ot/2UOAW3nB3hv4Ll6wnwHemn89xH++7wbeixfut4HX4YXbBY7xgj0EuJV/HcS/z0sDx4BLwF/zgv018FK8cK8D/DYv2FcDH8UL9jHAV/Ovg/i3+Szgo4HjPNutwFcDX8PzOg78NfAgXrDfBl6HF+ylgb/iBfsZ4K3510H8630X8N68YN8NvA/P662Bn+KFewhwKy/YrcCDeP52gRP86yD+dT4b+Cz+ZR8DfDXP67eB1+IF+xrgo3nBvhr4KF6wlwH+mhcd4l/nInCcf9mtwEN4Xq8N/BYv2K3AQ3jB3hr4KV6w9wG+mxcd4kX32sBv8aJ7GeCveV5/DbwUL9hDgFt5/o4DF3nBPgf4bF50iBfdawO/xYvudYDf5nl9NvBZvGDvA3w3L9gucIzn73eA1+ZFh3jRvTbwW7zoXgb4a57XSwN/xQv2NcBH84L9NvBaPH9/DbwMLzrEv84ucIx/2SXgOC/YrcCDeP5+B3htXrDvBt6LF0y86BD/Op8NfBb/ss8BPpsX7LeB1+L52wVO8IJ9NvBZvGDiRYf41/tp4K14wb4HeG9euN8GXosXTLxgnw18Fi+YeNEh/m0+G/ho4BjPdgn4auCz+Zd9NvBZvGDiBfts4LN4wU4Au7xoEP8+r82z/TYvugcDD+YF+21esAcDD+YF+2tglxcN4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wj0Ky/QY8Rec0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRssFeed;
impl IconShape for MdRssFeed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cx: "6.18",
                cy: "17.82",
                r: "2.18",
            }
            path {
                d: "M4 4.44v2.83c7.03 0 12.73 5.7 12.73 12.73h2.83c0-8.59-6.97-15.56-15.56-15.56zm0 5.66v2.83c3.9 0 7.07 3.17 7.07 7.07h2.83c0-5.47-4.43-9.9-9.9-9.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFa0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z2f+Z/ga4KN5Toj/XC8N/BX/M3wM8NU8J8R/rrcGfor/GV4H+G2eE+I/12cDn8X/DA8BbuU5If5zfTfwXvzPIJ4X4kX31sBH8aL5GOCvgd8GXov/fr8DvDbPC/Gi+2zgs3jRvAzw14B5wX6H5+848FI8f5eAv+b5ezDwIJ6/nwHemueFeNH9NPBWvGgEHAcu8vw9A3gwz99rA7/F8/c1wEfz/H028Fk8f58DfDbPC/Gi+yvgpfmX/Q3w0sBrA7/F8/c7wGvz/H008FU8fx8DfDXP328Dr8Xz9zbAT/O8EC8686L5HeC1gY8Gvorn73OAz+b5+2zgs3j+Xgf4bZ6/pwMP5vl7HeC3eV6IF81x4Kd50Xw38N3AVwMfxfP3McBX8/z9NvBaPH8PAW7l+TMvmHj+EP95fht4LZ6/1wF+m+fv6cCDef7E8/fSwF/x/D0DeDDPH+I/z9OBB/P8nQB2ef7M8/c7wGvz/L018FM8f78DvDbPH+I/j3nBxPP32sBv8fz9DPDWPH+fDXwWz9/nAJ/N84f4z/HawG/x/P0O8No8f28N/BTP3+cAn83z99PAW/H8fQzw1Tx/iP8c7w18F8/f9wDvzfP32cBn8fy9DfDTPH+/DbwWz9/rAL/N84d40ZgXzfcA7w18NvBZPH+fA3w2z99PA2/F8/cywF/z/JkX7ASwy/OH+Je9NvBbvGg+B/hs4KeBt+L5ex3gt3n+/gp4aZ4/8fw9GHg6L5h4wRD/srcGfooXzesAvw18NfDSPH8fDfw1z595/p4BPJjn76WBr+b5+2vgo3nBEP+yzwY+ixfNywB/zb/Ng4Gn8/z9DvDa/MdD/Mt+GngrXjTi3+61gd/i+fsc4LP5j4f4l700cJwXzW/zb/fZwGfx/H0M8NX8x0P8z/HdwHvx/L0O8Nv8x0P8z/HbwGvx/J0AdvmPh/if4yJwnOdP/OdA/M9wHLjI8/c7wGvznwPxwh0HXop/u0vAX/Mve23gt3j+vgd4b/5zIF64twZ+in+73wFem3/ZewPfxfP3OcBn858D8cJ9NvBZ/Nt9DvDZ/Ms+G/gsnr/XAX6b/xyIF+67gffi3+59gO/mX/bbwGvx/L0M8Nf850C8cL8NvBb/dq8D/Db/sqcDD+b5E/95EC+c+fc5AezyLzPP398AL81/HsR/v5cG/orn73eA1+Y/D+K/31sDP8Xz9znAZ/OfB/Hf77OBz+L5ex/gu/nPg/jv99PAW/H8vQ7w2/znQfz3+23gtXj+xH8uxAv2U8Bx/u3eBtjlX2aev0vAcf5zIV4w8+8j/mUPBp7O8/c7wGvznwvx/L008Ff82/0N8NL8y14b+C2ev68BPpr/XIjn762Bn+Lf7meAt+Zf9tnAZ/H8fQ7w2fznQjx/nw18Fv92nwN8Ns/21cBL8bweDDyY5++vgV2e1/cA381/DMTz99HAW/Nv99XAT/Ns5j/OxwBfzX8MxH++BwNP5z/O6wC/zX8MxH++1wZ+i/84J4Bd/mMg/vN9NvBZ/Me4BBznPw7iP993A+/Ff4zfAV6b/ziI/3y/DbwW/zG+B3hv/uMg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8o9bhBAhBKngAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdRtt;
impl IconShape for MdRtt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.03 3l-1.11 7.07h2.62l.7-4.5h2.58L11.8 18.43H9.47L9.06 21h7.27l.4-2.57h-2.35l2-12.86h2.58l-.71 4.5h2.65L22 3H9.03zM8 5H4l-.31 2h4L8 5zm-.61 4h-4l-.31 2h4l.31-2zm.92 8h-6L2 19h6l.31-2zm.62-4h-6l-.31 2h6.01l.3-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe2vgq4AH87/TrcD7AL/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvo4Gv4v+WjwG+mueEeP6+G3gv/m/5GuCjeU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+K/1iXgGP95fgd4bZ4T4vn7beC1+K/1O8B3A9/Ff47fAV6b54R4/n4beC3+a/0O8NrAewPfxX+83wFem+eEeP5+G3gt/mv9DvDaXPHewHfxH+t3gNfmOSGev98GXov/Wr8DvDbP9trATwPH+I/xO8Br85wQz99vA6/Ff63fAV6b5/TSwG8Dx/j3+x3gtXlOiOfvt4HX4r/G3wB/Dfw08NM8r5cGfhs4xr/P7wCvzXNCPH+/DbwW/3l+B/hu4Lt50bw08NPAg/i3+x3gtXlOiOfvt4HX4j/e7wAfDfw1/3rHgd8GXop/m98BXpvnhHj+fht4Lf7jXALeG/hp/n2OA78NvBT/er8DvDbPCfH8/TbwWvzH+BvgtYFd/mMcB54OHOdf53eA1+Y5IZ6/3wZei3+/vwFeG9jlP853Ae/Nv97vAK/Nc0I8f78NvBb/PpeABwO7/Mf5LuC9+bf5HeC1eU6I5++3gdfi3+d1gN/mP8Zx4KeA1+bf7neA1+Y5IZ6/3wZei3+7zwE+m3/ZceCluOIS8Nc8r+PAbwEvzb/P7wCvzXNCPH+/DbwW/zaXgAcDuzyvtwJeG3hp4LV5Tr8DvDbP6TjwW8BL8+/3O8Br85wQz99vA6/Fv83nAJ/Nsz0Y+CzgrYHjvGC/A7w2z3Yc+C3gpfmP8TvAa/OcEM/fbwOvxb/NCWCXKz4L+GxeNL8DvDZXvDTwW8Bx/uP8DvDaPCfE8/fbwGvxr/c9wHsDLw18F/DSvOh+B3ht4KWB3wKO8x/rd4DX5jkhnr/fBl6Lf723AW4Ffgs4zr/O7wAfDfwWcJz/eL8DvDbPCfH8/TbwWvzrvQ7wU8Bx/vVuBY4Dx/nP8TvAa/OcEM/fbwOvxb/OM4CLwEvzP9PvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/Azw1jwnxPP32cBn8X/L5wCfzXNCPH9vDfwU/7e8DfDTPCfEC/bXwEvxf8PvAK/N80K8YA8Gfht4EP+7/Q3w2sAuzwvxwh0HPhp4b+BB/O/yDOC7ga8Gdnn+EGD+/0KA+f8LAeb/LwSY/78QYP7/QsBr8/8X4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjp2OWfEH19bcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdScreenShare;
impl IconShape for MdScreenShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.11-.9-2-2-2H4c-1.11 0-2 .89-2 2v10c0 1.1.89 2 2 2H0v2h24v-2h-4zm-7-3.53v-2.19c-2.78 0-4.61.85-6 2.72.56-2.67 2.11-5.33 6-5.87V7l4 3.73-4 3.74z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSentimentSatisfiedAlt;
impl IconShape for MdSentimentSatisfiedAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-2.5c2.33 0 4.32-1.45 5.12-3.5h-1.67c-.69 1.19-1.97 2-3.45 2s-2.75-.81-3.45-2H6.88c.8 2.05 2.79 3.5 5.12 3.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/3oOBB/H8PQO4lf86iP9cx4HXAl4beGngtXnR/Dbw18BvA78D7PKfA/Gf472Atwbemv8YPw38NPA9/MdC/Mc5DnwU8NHAcf5z7AJfDXwNsMu/H+I/xmcBHw0c57/GLvDZwNfw74P493lp4LuAl+a/x18D7wP8Nf82iH+7zwI+mxfNJeCngVuB3wZuBW7lOT0YeDDw2sBLA68NHONF89HA1/Cvh/jXOw78FPDavHCXgO8Gvhv4a/5tXhr4aOCtgWO8cD8NvA+wy4sO8a9zHPgt4KV5wZ4BfDXw3cAu/zGOAx8NvDfwIF6wvwZeB9jlRYP413lr4Kd4wT4H+Gpgl/8cx4GPBj6LF+xtgJ/mRYP413tv4Lt4Ts8A3hr4a/5rvDTw08CDeE7vA3w3LzrEv817A9/FFd8DfDSwy7/sOPBSwGtzxYO54lau+G3gb4Bd/mXHge8G3oor3gf4bv51EP927w28NvDevHAPBt4KeG/gpXnR/DXw3cDPALfywn038NvAd/Ovh/jPcxz4LOCj+ff5auBzgF3+4yH+c3wW8NHAcf5j7AJfDXwO/7EQ/7GOA18FvDf/Ob4b+Bhgl/8YiP84x4HfAl6aF80zgFu54sHAg3jR/DXwOsAu/36I/1jfDbwXz98zgJ8Gvhv4a56/lwbeG3hr4EE8f98DvDf/MRD/8b4beC+e7RLw1cBn86I7Dnw08NHAMZ7te4D35j8O4j/HdwPvBfwN8NrALv82x4HfBl4K+B7gvfmPhfjP897ATwO7/PscB94a+G7+4yH+f0P8/4b4/w3xn+ulgWP8+zwDuJX/HIj/PN8FvDf/Mb4a+Bj+4yH+c7w38F38x3of4Lv5j4X4z/HdwHvxH+trgI/mPxbiP8dvA6/Ff6zfAV6b/1iI/xy/DbwW/7F+B3ht/mMh/nP8NvBaXCH+fcwVvwO8Nv+xEP85fht4La4Q/z7mit8BXpv/WIj/HL8NvBZXiH8fc8XvAK/NfyzEf47fBl6LK8S/j7nid4DX5j8W4j/HbwOvxRXi38dc8TvAa/MfC/Gf47eB1+IK8e9jrvgd4LX5j4X4z/HbwGtxhfj3MVf8DvDa/MdC/Of4beC1uEL8+5grfgd4bf5jIf5z/DbwWlwh/n3MFb8DvDb/sRD/OX4beC2uEP8+5orfAV6b/1iI/xy/DbwWV4h/H3PF7wCvzX8sxH+O3wZeiyvEv4+54neA1+Y/FuI/x28Dr8UV4t/HXPE7wGvzHwvxn+O3gdfiCvHvY674HeC1+Y+F+M/x28BrcYX49zFX/A7w2vzHQvzn+G3gtbhC/PuYK34HeG3+YyH+c/w28FpcIf59zBW/A7w2/7EQ/zl+G3gtrhD/PuaK3wFem/9YiP8cvw28FleIfx9zxe8Ar81/LMR/jt8GXosrxL+PueJ3gNfmPxbiP8dvA6/FFeLfx1zxO8Br8x8L8Z/jt4HX4orf5t/ntbnid4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cPw28Ff+xfgZ4a/5jIf5zfDTwVfzH+hjgq/mPhfjPcRz4beCl+I/xN8BrA7v8x0L85zkOvDfw2sBx/m12gd8GvhvY5T8e4v83xP9viP/fEP+/If5/4x8Bpf7AQTFIO58AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSpeakerPhone;
impl IconShape for MdSpeakerPhone {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 7.07L8.43 8.5c.91-.91 2.18-1.48 3.57-1.48s2.66.57 3.57 1.48L17 7.07C15.72 5.79 13.95 5 12 5s-3.72.79-5 2.07zM12 1C8.98 1 6.24 2.23 4.25 4.21l1.41 1.41C7.28 4 9.53 3 12 3s4.72 1 6.34 2.62l1.41-1.41C17.76 2.23 15.02 1 12 1zm2.86 9.01L9.14 10C8.51 10 8 10.51 8 11.14v9.71c0 .63.51 1.14 1.14 1.14h5.71c.63 0 1.14-.51 1.14-1.14v-9.71c.01-.63-.5-1.13-1.13-1.13zM15 20H9v-8h6v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAESklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz99x4L2AlwYezP9utwJ/DXwPsMtzQjyv9wa+CjjO/y27wMcA382zIZ7TawO/xf9trwP8NlcgntPTgQfzf9utwEO4AvFsLw38Ff8/vAzw1wDi2V4b+C1eNH8D7PI/y3HgpXjRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfN6wC/zf8srw38Fi+a1wF+G0A822sDv8WL5nWA3+Z/ltcGfosXzesAvw0gnu21gd/iRfM6wG/zP8trA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3+Z/ltYHf4kXzOsBvA4hne23gt3jRvA7w2/zP8trAb/GieR3gtwHEs7028Fu8aF4H+G3+Z3lt4Ld40bwO8NsA4tleG/gtXjSvA/w2/7O8NvBbvGheB/htAPFsrw38Fi+a1wF+m/9ZXhv4LV40rwP8NoB4ttcGfosXzesAv83/LK8N/BYvmtcBfhtAPNtrA7/Fi+Z1gN/mf5bXBn6LF83rAL8NIJ7ttYHf4kXzOsBv8z/LawO/xYvmdYDfBhDP9trAb/GieR3gt/mf5bWB3+JF8zrAbwOIZ3tt4Ld40bwO8Nv8z/LawG/xonkd4LcBxLO9NvBbvGheB/ht/md5beC3eNG8DvDbAOLZXhv4LV40rwP8Nv+zvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpv/WV4b+C1eNK8D/DaAeLbXBn6LF83rAL/N/yyvDfwWL5rXAX4bQDzbawO/xYvmdYDf5n+W1wZ+ixfN6wC/DSCe7bWB3+JF8zrAb/M/y2sDv8WL5nWA3wYQz/bawG/xonkd4Lf5n+W1gd/iRfM6wG8DiGd7beC3eNG8DvDb/M/y2sBv8aJ5HeC3AcSzvTbwW7xoXgf4bf5neW3gt3jRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfNXwO7/M9yHHhpXjSvA/w2gHi2lwb+iv8fXgb4awDxnG4FHsT/bc8AHswViOf02sBv8X/b6wC/zRWI5/XewFcDx/i/5RLw3sBP82yI5+848N7ASwMP5n+3W4G/Br4b2OU5If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EVUCikH7a3mjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStayCurrentLandscape;
impl IconShape for MdStayCurrentLandscape {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.01 7L1 17c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2H3c-1.1 0-1.99.9-1.99 2zM19 7v10H5V7h14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStayCurrentPortrait;
impl IconShape for MdStayCurrentPortrait {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1.01L7 1c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAESklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Qz99x4L2AlwYezP9utwJ/DXwPsMtzQjyv9wa+CjjO/y27wMcA382zIZ7TawO/xf9trwP8NlcgntPTgQfzf9utwEO4AvFsLw38Ff8/vAzw1wDi2V4b+C1eNH8D7PI/y3HgpXjRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfN6wC/zf8srw38Fi+a1wF+G0A822sDv8WL5nWA3+Z/ltcGfosXzesAvw0gnu21gd/iRfM6wG/zP8trA7/Fi+Z1gN8GEM/22sBv8aJ5HeC3+Z/ltYHf4kXzOsBvA4hne23gt3jRvA7w2/zP8trAb/GieR3gtwHEs7028Fu8aF4H+G3+Z3lt4Ld40bwO8NsA4tleG/gtXjSvA/w2/7O8NvBbvGheB/htAPFsrw38Fi+a1wF+m/9ZXhv4LV40rwP8NoB4ttcGfosXzesAv83/LK8N/BYvmtcBfhtAPNtrA7/Fi+Z1gN/mf5bXBn6LF83rAL8NIJ7ttYHf4kXzOsBv8z/LawO/xYvmdYDfBhDP9trAb/GieR3gt/mf5bWB3+JF8zrAbwOIZ3tt4Ld40bwO8Nv8z/LawG/xonkd4LcBxLO9NvBbvGheB/ht/md5beC3eNG8DvDbAOLZXhv4LV40rwP8Nv+zvDbwW7xoXgf4bQDxbK8N/BYvmtcBfpv/WV4b+C1eNK8D/DaAeLbXBn6LF83rAL/N/yyvDfwWL5rXAX4bQDzbawO/xYvmdYDf5n+W1wZ+ixfN6wC/DSCe7bWB3+JF8zrAb/M/y2sDv8WL5nWA3wYQz/bawG/xonkd4Lf5n+W1gd/iRfM6wG8DiGd7beC3eNG8DvDb/M/y2sBv8aJ5HeC3AcSzvTbwW7xoXgf4bf5neW3gt3jRvA7w2wDi2V4b+C1eNK8D/Db/s7w28Fu8aF4H+G0A8WyvDfwWL5rXAX6b/1leG/gtXjSvA/w2gHi21wZ+ixfNXwO7/M9yHHhpXjSvA/w2gHi2lwb+iv8fXgb4awDxnG4FHsT/bc8AHswViOf02sBv8X/b6wC/zRWI5/XewFcDx/i/5RLw3sBP82yI5+848N7ASwMP5n+3W4G/Br4b2OU5If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EVUCikH7a3mjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStayPrimaryLandscape;
impl IconShape for MdStayPrimaryLandscape {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1.01 7L1 17c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2H3c-1.1 0-1.99.9-1.99 2zM19 7v10H5V7h14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStayPrimaryPortrait;
impl IconShape for MdStayPrimaryPortrait {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1.01L7 1c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdStopScreenShare;
impl IconShape for MdStopScreenShare {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21.22 18.02l2 2H24v-2h-2.78zm.77-2l.01-10c0-1.11-.9-2-2-2H7.22l5.23 5.23c.18-.04.36-.07.55-.1V7.02l4 3.73-1.58 1.47 5.54 5.54c.61-.33 1.03-.99 1.03-1.74zM2.39 1.73L1.11 3l1.54 1.54c-.4.36-.65.89-.65 1.48v10c0 1.1.89 2 2 2H0v2h18.13l2.71 2.71 1.27-1.27L2.39 1.73zM7 15.02c.31-1.48.92-2.95 2.07-4.06l1.59 1.59c-1.54.38-2.7 1.18-3.66 2.47z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSwapCalls;
impl IconShape for MdSwapCalls {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 4l-4 4h3v7c0 1.1-.9 2-2 2s-2-.9-2-2V8c0-2.21-1.79-4-4-4S5 5.79 5 8v7H2l4 4 4-4H7V8c0-1.1.9-2 2-2s2 .9 2 2v7c0 2.21 1.79 4 4 4s4-1.79 4-4V8h3l-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/nV+B3htnpd5Xq8D/DbP6bWB3+J5ief128Br8a/zO8Br85wQz99vA6/Fv87vAK/N8zLP63WA3+Y5vTbwWzwv8bx+G3gt/nV+B3htnhPi+ftt4LX41/kd4LV5XuZ5vQ7w2zyn1wZ+i+clntdvA6/Fv87vAK/Nc0I8f78NvBb/Or8DvDbPyzyv1wF+m+f02sBv8bzE8/pt4LX41/kd4LV5Tojn77eB1+Jf53eA1+Z5mef1OsBv85xeG/gtnpd4Xr8NvBb/Or8DvDbPCfH8/TbwWvzr7AJ/zfN6bZ7XXwO7PKfjwEvzvH6b5/XSwHH+dX4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bPgf4bJ4T4vn7beC1+L/lbYCf5jkhnr/fBl6L/zv+Bnhpnhfi+ftt4LX4v+FvgLcGbuV5IZ6/3wZei//dngF8N/DVwC7PH+L5+23gtfiXvQ/w3fzvhXj+fht4LV649wG+m//dEM/fbwOvxQv2PsB3878f4vn7beC1eP7eB/hu/m9APH+/DbwWz+t9gO/m/w7E8/fbwGvxnN4H+G7+b0E8f78NvBbP9j7Ad/N/D+L5+23gtbjifYDv5v8mxPP328BrAe8DfDf/dyGev98Gvhv4bv5vQzx/Lw38Nf/3If5/Q/z/hvj/DfH/G+L/N/4RCV+OQf6lHVAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTextsms;
impl IconShape for MdTextsms {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM9 11H7V9h2v2zm4 0h-2V9h2v2zm4 0h-2V9h2v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z40T0Y+CzgrYHj/M+0C/w08DnArfzLEC+a9wa+i/9d3gf4bl44xL/swcDT+d/pIcCtvGCIf9l3A+/F/07fA7w3LxjiX3YROM7/TrvACV4wxL/MPK+vAd4aeBD/MzwD+Gngo3he4gVD/MvM83od4LeB9wY+G3gQ/z2eAXw28N3AawO/xfMSLxjiX2ae1+sAv82zvTfw2cCD+K/xDOCzge/m2V4b+C2el3jBEP8y87xeB/htntd7A58NPIj/HM8APhv4bp7XawO/xfMSLxjiX2ae1+sAv80L9t7AZwMP4j/GM4DPBr6bF+y1gd/ieYkXDPEvM8/rfYDv5l/23sBnAw/i3+YZwGcD382/7L2B7+J5iRcM8S8zz99vA58D/Db/svcGPht4EC+aZwCfDXw3/7LXBj4LeG2eP/GCIf5l5oX7beBzgN/mX/bewGcDD+L5ewbw2cB38y97beCzgNfmhRMvGOJfZl40vw18DvDb/MveG/hs4EFc8Qzgs4Hv5l/22sBnAa/Ni0a8YIh/mfnX+W3gc4Df5l/23lzx3fzLXhv4LOC1+dcRLxjiX2b+bX4b+Bzgt/n3eW3gs4DX5t9GvGCIf5n59/lt4HOA3+Zf57WBzwJem38f8YIh/mXmP8ZvA58D/DYv3GsDnwW8Nv8xxAuG+JeZ/1i/DXwO8Ns8p9cGPgt4bf5jiRcM8S8z/zl+G/gcrvgs4LX5zyFeMMS/zPzvJl4wxL/M/O8mXjDEv8z8z/U3wC7P9mDgQTwn8YIh/mXmf45LwE8DPw38NM/fceCtgfcGXgsQLxjiX2b+Z/ge4LOBW3nRvTbw27xgiH+Z+e91CXhr4Lf5j4f4l5n/Ps8A3hr4a56/lwaO8Wx/A+zyokP8y8x/j0vAawN/zXM6DnwU8N7Ag3levw18DfDT/MsQ/zLz3+NtgJ/mOb028FPAcf5lvw28DbDLC4b4l5n/er8DvDbP6b2B7+Jf51bgZYBdnj/Ev8z813sd4Ld5tpcGfgs4zr/eXwMvw/OH+JeZ/1p/A7w0z+m3gNfm3+5zgM/meSH+Zea/1tcAH82zvTXwU/z77AIneF6If9lfAy/Ff53XAX6bZ/tu4L3493sb4Kd5Toh/2XsD38V/nZcB/ppn+yvgpXm23+FFcxx4KZ7tc4DP5jkhXjTfDbwX/zXEczLPSbxoXhv4LZ7te4D35jkhXnSfDXw0cIz/XOI5meckXjSvDfwWz/Y1wEfznBD/eq/Nf67f5jn9NfBSPNtv86I5Drw0z/Y5wGfznBD/83038F78+70N8NM8J8T/fG8N/BT/PpeA4zwvxP8Ovw28Fv92nwN8Ns8L8b/DSwO/DRzjX+9vgJfm+UP87/HewHfxr/MM4KWBXZ4/xP8urw38NHCMf9nvAG8N7PKCIf73OQ58NPDewIN4Xj8DfDXw2/zLEP+7PRh4MM/218AuLzrE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EfH5rVB9wWCIAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdUnsubscribe;
impl IconShape for MdUnsubscribe {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
pub struct MdVoicemail;
impl IconShape for MdVoicemail {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.5 6C15.46 6 13 8.46 13 11.5c0 1.33.47 2.55 1.26 3.5H9.74c.79-.95 1.26-2.17 1.26-3.5C11 8.46 8.54 6 5.5 6S0 8.46 0 11.5 2.46 17 5.5 17h13c3.04 0 5.5-2.46 5.5-5.5S21.54 6 18.5 6zm-13 9C3.57 15 2 13.43 2 11.5S3.57 8 5.5 8 9 9.57 9 11.5 7.43 15 5.5 15zm13 0c-1.93 0-3.5-1.57-3.5-3.5S16.57 8 18.5 8 22 9.57 22 11.5 20.43 15 18.5 15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Lc5DrwV8NbAceDBwIO54q+BXeC3gZ8B/pr/uRD/Oq8NfBbw2rzobgW+GvgeYJf/WRAvmgcDXwW8Nf92twLvA/w2/3Mg/mWvDfwUcJz/GN8NvA//MyBeuPcGvov/eD8NvA3//RAv2HsD38V/nu8G3of/Xojn76WB3wKO85/rc4DP5r8P4vl7OvBg/mu8DPDX/PdAPK/3Br6L/zq/DbwO/z0Qz+vpwIP5r/U6wG/zXw/xnN4a+Cn+9f4G2OWK1+Jf76eBr+a/zt8AuwDiOX038F686H4HeG/gVp7tOPDRwGfxP9frAL8NIJ7TReA4L5rvAd6bF+y9ge/if6bXAX4bQDwn86K5BDwY2OWF+27gvfif53WA3wYQz/bawG/xovka4KP5l7028Fv8z/M6wG8DiGd7beC3eNG8D/DdvGjM/zyvA/w2gHi21wZ+ixfN+wDfzYvG/M/zOsBvA4hne23gt3jRfA3w0fzLXhv4Lf7neR3gtwHEczIvml3gIcAuL9x3A+/F/zyvA/w2gHhOu8AxXjTfDbwPL9h7A9/F/0yvA/w2gHhO3w28Fy+63wbeB7iVZzsOfBTw2fzP9TrAbwOI5/TWwE/xr/fXwC5XvDb/ej8DfDUvuq8GXooXzevwvP4a2AUQz+tW4EH813od4Ld50f028Fq8aMQLhnhe7w18F/91fgd4bf51fht4LV404gVDPH+3Ag/iv8bLAH/Nv85vA6/Fi0a8YIjn76WB3waO8Z/rc4DP5l/vt4HX4kUjXjDEC/bewHfxn+d7gPfm3+a3gdfiRSNeMMQL997Ad/Ef72eAt+bf7reB1+JFI14wxL/stYGfBo7xH+NrgI/m3+e3gdfiRSNeMMSL5sHAVwNvxb/dM4D3Bn6bf7/fBl6LF414wRD/Oq8NfDTwVrzongF8NfDdwC7/MX4beC1eNOIFQ/zbHAfeGnhr4DjwYOBBXPE3wC7w28BPA3/Nf7zfBl6LF414wRD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ec6CXdB9PqwwAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVpnKey;
impl IconShape for MdVpnKey {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12.65 10C11.83 7.67 9.61 6 7 6c-3.31 0-6 2.69-6 6s2.69 6 6 6c2.61 0 4.83-1.67 5.65-4H17v4h4v-4h2v-4H12.65zM7 14c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFYElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO/3bHgdcCXhp4ba54aeA4V+wCf80Vvw38NfA7wC7/Ooh/2XcD78W/3tsAP82/znsBbw28Nf82Pw38NPA9vGgQ/zLzb/M9wHvzLzsOfBTw3sCD+Y9xK/DdwOfwwiH+Zebf5neA1+aFe2vgq4AH85/jVuBjgJ/m+UP8y8y/ze8Ar83zdxz4LuCt+a/x08D7ALs8J8S/7K+Bl+Jf73eA1+b5e2/gu/iv9T7Ad/OcEP+y3wZei38b8YJ9N/Be/Nf4HuC9eV6If9lvA6/Fv414wY4Dvw28FP+5/gZ4bWCX54X4l3018FH827wO8Nu8YC8N/DZwjP8cl4DXBv6a5w/xL/ts4LP4t/kY4Kt54d4a+Cn+c7wN8NO8YIh/2WsDv8W/zc8Ab82/7KuBj+I/1ucAn80Lh/iXPRh4Ov924kXz28Br8R/jd4DX5l+GeNHcCjyIf5vXAX6bf9lx4FbgGP8+zwBeGtjlX4Z40Xw38F7827wO8Nu8aF4a+Cv+fV4G+GteNIgXzXsD38W/3t8AL82/znsD38W/zfsA382LDvGieTDwdP51/gZ4bWCXf73vBt6Lf53vAd6bfx3Ei+5W4EG8aP4GeG1gl3+b48BvAy/Fi+ZvgNcGdvnXQbzoPhr4Kv5lfwO8NrDLv8+Dgb8GjvHCXQJeGriVfz3Ei+7BwNN54f4GeG1gl/8Ybw38FC/c2wA/zb8N4l/nt4HX4vn7G+C1gV3+Y3028Fk8f58DfDb/doh/nfcGvovn9QzgpYFd/nP8NvBaPKffAV6bfx/Ev96twIN4TrvAywC38p/jOPDXwIO44hnASwO7/Psg/vXeG/guntf3AO/Nf56XBv4KuAS8NvDX/Psh/m1uBR7E83oZ4K/5z/PeXPHd/MdA/Nu8N/BdPK+/Bl6G/z0Q/3a/DbwWz+tzgM/mfwfEv91LA3/F8/c6wG/zPx/i3+ergY/iee0CDwF2+Z8N8e9zHPht4KV4Xn8NvA6wy/9ciH+/lwZ+GzjG8/pu4H34nwvxH+O9ge/i+ftu4H34nwnxH+e7gffi+ftu4H3493ktrvgd/uMg/mP9NvBaPH8/DbwPsMu/znHgt4CX5opd4LeB3wb+Gvgd/u0Q/7GOA78NvBTP318DrwPs8qI5DvwW8NK8cLvAX/NstwI/A/w0LxziP95x4LeBl+L52wXeBvhtXrjjwG8BL82/3ccAX80LhvjPcRz4beCleME+G/gcnr/jwG8BL82/z63AQ3jBEP95jgM/DbwWL9hfA+8D/DXPdhz4LeCl+Y8hXjDEf77vBt6LF+67gc8BdoHfAl6a/zjiBUP813hv4KuBY7xgu8Au8GD+Y4kXDPFf56WB7wZeiv9a4gVD/Nc6Dnw28FH81xEvGOK/x0sDXw28Fv/5xAuG+O/13sBnAw/iP494wRD/M7w38NnAg/iPJ14wxP8s7w28NfBW/McRLxjif6YHA28NfDTwIP59xAuG+J/vwcBrA68NvDbwIF503wO8Ny8Y4n+fBwMPBl4aeDDw0lxxHHgpnu17gI8GdnnBEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHZES2QYGx8MgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifiCalling;
impl IconShape for MdWifiCalling {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z",
            }
            path {
                d: "M20,15.51c-1.24,0-2.45-0.2-3.57-0.57c-0.35-0.12-0.75-0.03-1.02,0.24l-2.2,2.2c-2.83-1.45-5.15-3.76-6.59-6.59l2.2-2.2 C9.1,8.31,9.18,7.92,9.07,7.57C8.7,6.45,8.5,5.25,8.5,4c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4c0,9.39,7.61,17,17,17 c0.55,0,1-0.45,1-1v-3.49C21,15.96,20.55,15.51,20,15.51z",
            }
        }
    }
}
