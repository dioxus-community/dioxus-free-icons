use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/n4beC3+Y4h/H/Mf43eA1+Y5IZ6/3wZei/8Y4t/H/Mf4HeC1eU6I5++3gdfiP4b49zH/MX4HeG2eE+L5+23gtfiPIf59zH+M3wFem+eEeP5+G3gt/mOIfx/zH+N3gNfmOSGev98GXov/GOLfx/zH+B3gtXlOiOfvt4HX4j+G+Pcx/zF+B3htnhPi+ftt4LX4jyH+fcx/jN8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+I/xuvwvL4aeCme098AH83z+i3+Y/wO8No8J8Tz99vAa/EfQzyv3wZei+f0O8Br87zMf4zfAV6b54R4/n4beC3+Y4jn9dvAa/Gcfgd4bZ6X+Y/xO8Br85wQz99vA6/FfwzxvH4beC2e0+8Ar83zMv8xfgd4bZ4T4vn7beC1+I8hntdvA6/Fc/od4LV5XuY/xu8Ar81zQjx/vw28Fv8xxPP6beC1eE6/A7w2z8v8x/gd4LV5Tojn77eB1+I/hnhevw28Fs/pd4DX5nmZ/xi/A7w2zwnx/P028Fr8xxDP67eB1+I5/Q7w2jwv8x/jd4DX5jkhnr+XBo7zH+O3eV6/DbwWz+l3gNfmeb02/zF2gb/mOSH+e/w28Fo8p98BXpv/Woj/Hr8NvBbP6XeA1+a/FuK/x28Dr8Vz+h3gtfmvhfjv8dvAa/Gcfgd4bf5rIf57/DbwWjyn3wFem+f1WvzHuAT8Nc8J8d/jt4HX4jn9DvDaPC/zH+N3gNfmOSH+e/w28Fo8p98BXpvnZf5j/A7w2jwnxH+P3wZei+f0O8Br87zMf4zfAV6b54T47/HbwGvxnH4HeG2el/mP8TvAa/OcEP89fht4LZ7T7wCvzfMy/zF+B3htnhPiv8dvA6/Fc/od4LV5XuY/xu8Ar81zQvz3+G3gtXhOvwO8Ns/L/Mf4HeC1eU6I/x6/DbwWz+mvgY/mef02/zF+B3htnhPiv8dvA6/Ff63fAV6b54T47/HbwGvxX+t3gNfmOSH+e/w28Fr81/od4LV5Toj/Hr8NvBb/tX4HeG2eE+K/x28Dr8V/rd8BXpvnhPifz/zH+B3gtXlOiP/5zH+M3wFem+eE+J/P/Mf4HeC1eU6I//nMf4zfAV6b54T4n8/8x/gd4LV5Toj/+cx/jN8BXpvnhPifz/zH+B3gtXlOiP/5zH+M3wFem+eE+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8CGNiVQVpoZtUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAccountTree;
impl IconShape for MdAccountTree {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 11V3h-7v3H9V3H2v8h7V8h2v10h4v3h7v-8h-7v3h-2V8h2v3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8R/jOPALv+1jgO7/Psg/v1eGvgt4GOA7+a/xnsDXwW8DvDX/Nsh/n1eGvgt4DhXvA/w3fznem/gu7hiF3gd4K/5t0H8+7w08NvAMZ7tfYDv5j/HawO/xbNdAl4b+Gv+bRD/fi8N/DZwjGd7H+C7+Y/10sBvAce54hLw2sBf82+H+I/x0sBvA8d4tvcBvptnezDwVsBLAw8GXpvn9NvArcBvA78D3MqzvTTwW8BxrrgEvDbw1/z7IP7jvDTw28Axnu19AAPvDbw2/zq/DXw38DfAbwHHueIS8NrAX/Pvh/iP9dLAbwPH+M9xCXht4K/5j4H4j/fawG/xn+N1gN/mPw7iP9ZLA98FvDT/Of4aeB/gr/mPgfiP89LAbwHH+c+1C7wO8Nf8+yH+Y7w08FvAcf5r7AKvA/w1/z6If7/jwG8BL81/rb8GXgfY5d8O8e/31cBH8d/ja4CP5t8O8e/z2sBv8d/rZYC/5t8G8e/zW8Br84L9Dle8Fv82v8MVr8UL9tvA6/Bvg/i3e2ngr3j+LgGvDfw1V7w08NvAMV40l4DXBv6aK14a+G3gGM/fywB/zb8e4t/uu4H34vn7GuCjeU5fDXwUL5qvAT6a5/TVwEfx/H0P8N786yH+7Z4OPJjn732A7+Y5fTTwVbxo3gf4bp7TRwNfxfN3K/AQ/vUQ/zYvDfwVL9hPA2/Dc/op4K150fw08DY8p58C3poX7GWAv+ZfB/Fv897Ad/HCfTbwPVzxXsBn86/z2cD3cMV7AZ/NC/c+wHfzr4P4t/ls4LP4n+VzgM/mXwfxb/PZwGfxP8vnAJ/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/zYvDRznf5Zd4K/510H8/4b4/w3xb/PbwGvxP8vvAK/Nvw7i3+a3gdfif5bfAV6bfx3Ev81vA6/F/yy/A7w2/zqIf5vfBl6L/1l+B3ht/nUQ/za/DbwW/7P8DvDa/Osg/m1+G3gt/mf5HeC1+ddB/Nv8NvBa/M/yO8Br86+D+Lf5beC1+J/ld4DX5l8H8W/z28Br8T/L7wCvzb8O4t/mt4HX4n+W3wFem38dxL/NbwOvxf8svwO8Nv86iH+bzwY+i/9ZPgf4bP51EP82nw18Fv+zfA7w2fzrIP5t3hv4Lv5neR/gu/nXQfzbvDbwW/zP8jrAb/Ovg/i32wWO8T/DJeA4/3qIf7vvBt6L/xm+B3hv/vUQ/3bvDXwX/zO8DfDT/Osh/n1uBR7Ef69nAA/m3wbx7/PewHfx3+t9gO/m3wbx7/fXwEvx3+N3gNfm3w7x7/fSwG8Dx/ivdQl4beCv+bdD/Md4b+C7+K/1PsB38++D+I/z2cBn8V/jc4DP5t8P8R/ro4Gv4j/XxwBfzX8MxH+8twa+GzjGf6xLwFsDv81/HMR/jgcDnw28F/8xvgf4bOBW/mMh/nO9NvDZwGvxb/M7wGcDv81/DsR/jQcDbw28NvBWvHA/A/w08NvArfznQvz3eW2e02/zXw/x/xvi/zf+EerxrUFxvP1AAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAdb;
impl IconShape for MdAdb {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 16c0 3.87 3.13 7 7 7s7-3.13 7-7v-4H5v4zM16.12 4.37l2.1-2.1-.82-.83-2.3 2.31C14.16 3.28 13.12 3 12 3s-2.16.28-3.09.75L6.6 1.44l-.82.83 2.1 2.1C6.14 5.64 5 7.68 5 10v1h14v-1c0-2.32-1.14-4.36-2.88-5.63zM9 9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE00lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO/7FeG/gtnpf4t0P8y74beC/+9d4G+Gn+47w28Fs8L/Fvh/iXmX+b7wHem/84rw38Fs9L/Nsh/mXm3+Z3gNfmP85rA7/F8xL/doh/mfm3+R3gtfmP89rAb/G8xL8d4l/218BL8a/3O8Br8x/ntYHf4nmJfzvEv+y3gdfi30b8x3lt4Ld4XuLfDvEv+23gtfi3Ef9xXhv4LZ6X+LdD/Mu+Gvgo/m1eB/htXjSvDfwW//HEC4b4l3028Fn823wM8NW8aF4b+C3+44kXDPEve23gt/i3+RngrXnRvDbwW/zHEy8Y4l/2YODp/NuJF81rA7/FfzzxgiFeNLcCD+Lf5nWA3+Zf9trAb/EfT7xgiBfNdwPvxb/N6wC/zb/sOPDSvGAvDXwVz+t1eOF+mxcM8aJ5b+C7+Nf7G+Cl+Y/x2sBv8bzEvx3iRfNg4On86/wN8NrALv8xXhv4LZ6X+LdDvOhuBR7Ei+ZvgNcGdvmP89rAb/G8xL8d4kX30cBX8S/7G+C1gV3+Y7028Fs8L/Fvh3jRPRh4Oi/c3wCvDezyH++1gd/ieYl/O8S/zm8Dr8Xz9zfAawO7/Od4beC3eF7i3w7xr/PewHfxvJ4BvDSwy3+e1wZ+i+cl/u0Q/3q3Ag/iOe0CLwPcyn+e48BL87x+m387xL/eewPfxfP6HuC9+d8F8W9zK/AgntfLAH/N/x6If5v3Br6L5/XXwMvwvwfi3+63gdfieX0O8Nn874D4t3tp4K94/l4H+G3+50P8+3w18FE8r13gIcAu/7Mh/n2OA78NvBTP66+B1wF2+Z8L8e/30sBvA8d4Xj8NvA3/cyH+Y7w38F08f98NvA//MyH+43w38F48f98NvA//Pq/FFb/DfxzEf6zfBl6L5++ngfcBdvnXOQ78FvDSPNtPA38N/DbwO/zbIf5jHQd+G3gpnr+/Bt4GuJUXzXHgt4CX5l/22zzbrcDPAD/NC4f4j3cc+G3gpXj+doG3AX6bF+448FvAS/Nv9znAZ/OCIf5zHAd+G3gpXrCvBj4H2OV5HQd+C3hp/n1uBR7CC4b4z3Mc+GngtXjBbgXeB/htnu048FvAS/MfQ7xgiP983w28Fy/cdwOfA+wCvwW8NP9xxAuG+K/x3sBXA8d44W4FHsx/LPGCIf7rvDTw3cBL8V9LvGCI/1rHgc8GPor/OuIFQ/z3eGngq4HX4j+feMEQ/73eG/hs4EH85xEvGOJ/hvcGPht4EP/xxAuG+J/lvYG3Bt6K/zjiBUP8z/Rg4K2BjwYexL+PeMEQ//M9GHht4LWB1wYexIvue4D35gVD/O/zYODBwEsDDwZemiuOAy/Fs30P8NHALi8Y4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IWLKNQTCTWRgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAddCall;
impl IconShape for MdAddCall {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4l/nwcCDeLbf4V/2WvzX+B3+9RAvmrcGPgt4aZ7XdwOfA9zK82f+6/w08D3AT/OiQbxwx4GvAt6bf9n7AN/N8zL/9X4b+Bjgr3nhEC/cTwFvzYvufYDv5jmZ/x67wOsAf80LhnjBPhv4LP71HgLcyrOZ/z67wOsAf83zh3j+jgNPB47zr/c9wHvzbOa/163AQ3j+EM/fWwM/xb/NLnCCZzP//T4H+GyeF+L5+2zgs/i3ex3gt7nC/PfbBU7wvBDP308Db8W/3esAv80V5n+GlwH+mueEeP6+Gvgo/u1eB/htrjD/M3wO8Nk8J8Tz99HAV/FvdwLY5QrzP8PnAJ/Nc0I8fw8Gns6/ze8Ar82zmf8Zfgd4bZ4T4gX7buC9+Nd7HeC3eTbzP8PvAK/Nc0K8YMeBW4FjvOi+B3hvnpP5n+F3gNfmOSFeuJcGfhs4xr/sZ4D3BnZ5TuZ/ht8BXpvnhPiXHQe+Gngvnr9LwFcDn83zZ/5n+B3gtXlOiBfdg4G3Bo4DLw38NfDXwG8Du7xgv83/DH8NfDTPCfH/G+L/N8Tz99XAS/F/y98AH81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f18NvDT/di8NHOM/1iXgr/m3+2vgo3lOiP8cvw28Fv+xfgd4bf5jIf5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9i72NB8kFSdQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatFlat;
impl IconShape for MdAirlineSeatFlat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 11v2H9V7h9c2.21 0 4 1.79 4 4zM2 14v2h6v2h8v-2h6v-2H2zm5.14-1.9c1.16-1.19 1.14-3.08-.04-4.24-1.19-1.16-3.08-1.14-4.24.04-1.16 1.19-1.14 3.08.04 4.24 1.19 1.16 3.08 1.14 4.24-.04z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFS0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If70HAx8FvDTw2lzx28BfA18D3Mr/Hoh/na8CPpoX7quBj+F/B8SL5jjwW8BL86L5beB1+J8P8aL5auCj+Nf5HOCz+Z8N8S97MPB0/m0eAtzK8zoOfBfw08DPALv890D8y74a+Cj+bb4G+Gie10cDX8UVu8BPAz8N/Az/tRD/sr8CXpp/m78GXobn9VfAS/O8bgV+Gvge4K/5z4f4l5l/H/GcHgw8nX/ZXwPfDfwMcCv/ORD/MvPvI57TVwMfxb/ObwPfDfwMsMt/HMS/7K+Bl+Lf5m+Al+Y5PR14MP923w38NPAz/Psh/mVfDXwU/zZfA3w0z/bawG/xH2MX+G7ge4C/5t8G8S97MPB0/m0eAtzKs3038F78x7sV+G7ge4BbedEhXjSfDXwW/zpfA3w0z3YceDpwnP9cfw18NfA9/MsQL7qfBt6KF83PAG/Nc3pv4Lv4r3Mr8DbAX/OCIf51vhr4KF64rwE+muf108Bb8V/vfYDv5vlDPH8vDbwW8D3ALs/pwcBHAy8NvDRwDPgd4K+BrwZu5Xk9GHg6/33eB/hunhfi+ftu4L244qeBnwa+h3+7jwa+iv9eDwFu5Tkhnr+LwHGe0y7w08BPAz/Dv87TgQfz3+u3gdfhOSGe11sDP8ULdyvw08D3AH/NC/fSwF/xP8PLAH/NsyGe108Db8WL7q+B7wZ+BriV5/Vg4KOB9waO8d/ra4CP5tkQz+urgbcGHsS/3m8D3w38DLDL83pr4K2B9+K/x+8Ar82zIV6wtwbeGnhr4Bj/et8N/DTwMzyv48BbA+8NvBb/dXaBEzwb4l92HHhr4K2Bt+Jfbxf4buB7gL/meT0YeGvgo4EH8Z9PPBviX+c48N7AewMvxb/ercB3A98D3MrzemngvYG3Bh7Ef7xLwHGeDfFv92Dgo4G3Bh7Ev95fA18N/Aywy/N6a+CtgbcGjvEf43eA1+bZEP8xXhr4aOCtgWP86/008NPA9/C8jgNvDbw18Fb8+3wO8Nk8G+I/3lsDXw08iH+9XeCnge8Bfpvn9WDgrYH3Bl6Kf72XAf6aZ0P85/ht4LX497kV+Gnga4BbeV4PBj4aeGvgQfzL/gZ4aZ4T4j/HbwOvxX+cvwa+G/gZ4Fae12sD7w28NXCM5+9jgK/mOSH+c/w28Fr85/hp4KeBnwF2eV7vDbw18FY8p4cAt/KcEP85fht4Lf5z7QI/Dfw08DM8r+PAewPvDdwKvDXPC/Gf47eB1+K/zq3ATwPfA/w1LzrEf47fBl6L/x63Al8N/AxwKy8c4j/HbwOvxX+/3wY+B/htnj/Ef47fBl6L/1jPAH4beGvgGC+6lwH+mucP8Z/jt4HX4j/W7wCvzRXvDbw18Fa8cM8AHswLhvjP8dvAa/Ef63eA1+Y5HQfeG3hv4KV4Xp8DfDYvGOI/x28Dr8V/rN8BXpsX7MHAewPvDTyIKx4C3MoLhvjP8dXAS/Mf66+Bj+ZF89LAWwOfzQuH+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHr8bAQTfKWkQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatFlatAngled;
impl IconShape for MdAirlineSeatFlatAngled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.25 14.29l-.69 1.89L9.2 11.71l2.08-5.66 8.56 3.09c2.1.76 3.18 3.06 2.41 5.15zM1.5 12.14L8 14.48V19h8v-1.63L20.52 19l.69-1.89-19.02-6.86-.69 1.89zm5.8-1.94c1.49-.72 2.12-2.51 1.41-4C7.99 4.71 6.2 4.08 4.7 4.8c-1.49.71-2.12 2.5-1.4 4 .71 1.49 2.5 2.12 4 1.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4tmOAy/F8/od/mWvxbM9A7iVF+6lgWP857gE/DUvGsSzvTbwWzwv8fw9GPgs4L15Xn8NfA7w0zx/vw28Fv95bgV+G/gc4FZeMMSzvTbwWzwv8bzeG/gu/mXfDXwMsMtz+m3gtfiv8dXAx/D8IZ7ttYHf4nmJ5/TewHfxovtp4G14Tr8NvBb/df4aeB1gl+eEeLbXBn6L5yWe7cHA0/nX+xzgs3m23wZei/9afw28DM8J8WyvDfwWz0s823cD78W/3i7wEGCXK34beC3+630N8NE8G+LZXhv4LZ6XeDbzb/c2wE9zxW8Dr8V/j4cAt3IF4tleG/gtnpe44rWB3+Lf7nOAz+aK3wZei/8e3wO8N1cgnu21gd/ieYkrXhv4Lf7tfgZ4a674beC1+O9xK/AQrkA822sDv8XzEle8NvBb/Nt9DvDZXPHbwGvx3+dlgL8GEM/22sBv8bzEFQ8Gns6/3ccAX80Vvw28Fv99Xgf4bQDxbK8N/BbPSzzbXwMvxb/NQ4BbueK3gdfiv8/rAL8NIJ7ttYHf4nmJZ3tr4Kf41/se4L15tt8GXov/Pq8D/DaAeLbXBn6L5yWe03cD78WL7hLwYGCXZ/tt4LX47/M6wG8DiGd7beC3eF7iOR0Hvht4K/5ll4DXBv6a5/TbwGvx3+d1gN8GEM/22sBv8bzE8/fZwEcDx3j+vgf4aGCX5/XbwGvx3+d1gN8GEM/22sBv8bzEC3YceG3gpYGXBv4a2AV+GriVF+y3gdfiv8/rAL8NIJ7ttYHf4nmJ/3i/DbwW/31eB/htAPFsrw38Fs9L/Mf7beC1+O/zOsBvA4hne23gt3hev81/vJcGjvPf53WA3wYQz/bawG/x/8PrAL8NIJ7ttYHf4v+H1wF+G0A822sDv8X/D68D/DaAeLbXBn6L/x9eB/htAPFsrw38Fv8/vA7w2wDi2V4b+C3+f3gd4LcBxLO9NvBb/P/wOsBvA4hnOw68NP8//DWwCyD+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C+x15QdevQZ0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatIndividualSuite;
impl IconShape for MdAirlineSeatIndividualSuite {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 13c1.65 0 3-1.35 3-3S8.65 7 7 7s-3 1.35-3 3 1.35 3 3 3zm12-6h-8v7H3V7H1v10h22v-6c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/n4beC2e0+8Ar80Vvw28Fv92rwP8Nv/9EM/fbwOvxXP6HeC1ueK3gdfi3+51gN/mvx/i+ftt4LV4Tr8DvDZX/DbwWvzbvQ7w2/z3Qzx/vw28Fs/pd4DX5orfBl6Lf7vXAX6b/36I5++3gdfiOf0O8Npc8dvAa/Fv9zrAb/PfD/H8/TbwWjyn3wFemyt+G3gt/u1eB/ht/vshnr/fBl6L5/Q7wGtzxW8Dr8W/3esAv81/P8Tz99vAa/Gcfgd4ba74beC1+Ld7HeC3+e+HeP5+G3gtntPvAK/NFb8NvBb/dq8D/Db//RDP328Dr8Vz+h3gtbnit4HX4t/udYDf5r8f4vn7beC1eE6/A7w2V/w28Fr8270O8Nv890M8f78NvBbP6XeA1+aK3wZei3+71wF+m/9+iOfvt4HX4jn9DvDaXPHbwGvxb/c6wG/z3w/x/P028Fo8p98BXpsrfht4Lf7tXgf4bf77IZ6/3wZei+f0O8Brc8VvA6/Fv93rAL/Nfz/E8/fbwGvxnH4HeG2u+G3gtfi3ex3gt/nvh3j+fht4LZ7T7wCvzRW/DbwW//luBX4b+BzgVv7jIZ6/3wZei+f0O8Brc8VvA6/Ff51d4GOA7+Y/FuL5+23gtXhOvwO8Nlf8NvBa/NfaBV4GuJX/OIjn77eB1+I5/Q7w2lzx28Br8V/vt4HX4T8O4vn7beC1eE6/A7w2V/w28Fr893gZ4K/5j4F4/n4beC2e0+8Ar80VPw28Ff89vgd4b/5jIJ6/3wZei+f0O8Brc8VnA5/Ff5+HALfy74d4/n4beC2e0+8Ar80V7w18F/99vgd4b/79EM/fdwPvxXP6HeC1ueI4cCtwjP8+DwFu5d8H8fx9NvBZPKe/Bl6GZ/ts4LP47/M9wHvz74N4/j4a+Cqe10OAW3m23wZei/8+DwFu5d8O8fw9GHg6z+tzgM/m2Y4DXw28F/89vgd4b/7tEC/YXwMvxXPaBV4GuJXn9NbAZwMvxX+9hwC38m+DeME+G/gsntdfA68D7PK8Hgw8mP9Y3w08iBfse4D35t8G8cLdCjyI5/XXwMcAv81/vvcGvosX7iHArfzrIV64twZ+ihfsu4GfBv4GuJX/HMeBW4FjvGDfA7w3/3qIf9lXAx/Ff77fAV6b5++zgc/iBdsFHgLs8q+DeNF8N/Be/Of6HeC1ef6OA7cCx3jBPgf4bP51EC+6jwa+iv88vwO8Ni/YZwOfxQu2CzwE2OVFh/jXeW3gs4HX4j/e7wCvzQt2HLgVOMYL9jnAZ/OiQ/zbvDbw3sBrAw/iP8bvAK/NC/fZwGfxgu0CDwF2gdcGPgp4a664Ffhu4HN4NsS/33Hgpfn32wX+mhfuOHArcIwX7KeBBwMvzfP3OcBncwXif5/PBj6Lf58TwC6A+N/nOHArcIx/u9cBfhtA/O/01cBH8W/3OsBvA4j/nR4MPJ1/m2cAD+YKxP9e3w28F/86l4D3Bn6aKxD/ez0YeDovup8BPhv4a54N8b/bdwPvxQv3PcBnA7fyvBD/uz0YeDrP6xLw1cB3A7fygiH+9/tu4L244hnAVwPfDezyL0P87/dg4KeBrwa+m38dxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RfCXDQXXPLXAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatLegroomExtra;
impl IconShape for MdAirlineSeatLegroomExtra {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 12V3H2v9c0 2.76 2.24 5 5 5h6v-2H7c-1.66 0-3-1.34-3-3zm18.83 5.24c-.38-.72-1.29-.97-2.03-.63l-1.09.5-3.41-6.98c-.34-.68-1.03-1.12-1.79-1.12L11 9V3H5v8c0 1.66 1.34 3 3 3h7l3.41 7 3.72-1.7c.77-.36 1.1-1.3.7-2.06z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEk0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb3OF+fcR/30Q/zLzvF4H+G2uMP8+4r8P4l9mntfrAL/NFebfR/z3QfzLzPN6HeC3ucL8+4j/Poh/mXlerwP8NleYfx/x3wfxLzPP63WA3+YK8+8j/vsg/mXmeb0O8NtcYf59xH8fxL/MPK/XAX6bK8y/j/jvg/iXmef1OsBvc4X59xH/fRD/MvO8Xgf4ba4w/z7ivw/iX2ae1+sAv80V5t9H/PdB/MvM83od4Le5wvz7iP8+iH+ZeV6vA/w2V5h/H/HfB/EvM8/rdYDf5grz7yP++yD+ZeZ5vQ7w21xh/n3Efx/Ev8w8r9cBfpsrzL+P+O+D+JeZ5/U6wG9zhfnvcSvw3cDXALv82yD+ZeZ5vQ7w21xh/nv9NfA2wK386yH+ZeZ5vQ7w21xh/vv9NfAy/Osh/mXmeb0O8NtcYf5neBvgp/nXQfzLzPN6HeC3ucL8z/A5wGfzr4P4l5nn9TrAb3OF+Z/hd4DX5l8H8S8zz+t1gN/miluBB/Hf73eA1+ZfB/EvM8/rdYDf5orvBt6L/36/A7w2/zqIf5l5Xu8DfDdXvDfwXfz3+x3gtfnXQfzL/hp4KZ7T5wCfzbP9NvBa/Pf6HeC1+ddB/Mt+G3gtntNfAy/Dsz0Y+GvgGP99fgd4bf51EP+yjwa+iuf1EOBWnu2lge8GXor/Hr8DvDb/Ooh/2UsDf8Xz+m3gdXhOx4GPBt4beBD/tX4HeG3+dRAvmt8GXovn9d3A+/D8vTRwnP84Lw18FS/Y7wCvzb8O4kXz2sBv8fx9N/AxwC7/uV4b+C1esN8BXpt/HcSL7ruB9+L52wW+G/hp4Hf4z/HawG/xgv0O8Nr86yBedMeB3wZeiv98rwP8Ns/ptYHf4gX7HeC1+ddB/Os8GPhp4KX4z/U6wG/znF4b+C1esN8BXpt/HcS/3nHgp4HX4j/P6wC/zXN6beC3eMF+B3ht/nUQ/3bvDXw28CD+470O8Ns8p9cGfosX7HeA1+ZfB/Hvcxx4a+CtgbfiP87rAL/Nc3pt4Ld4wX4HeG3+dRD/sV4aOM6/318Duzyn1wZ+ixfsd4DX5l8H8b/HawO/xQv2O8Br86+D+N/jtYHf4gX7HeC1+ddB/O/x2sBv8a/z18BPA5/D84f43+O1gd/i3+a3gdfheSH+93ht4Lf4t3sf4Lt5Toj/PV4b+C3+7b4G+GieE+J/j9cGfot/u68BPprnhPjf47WB3+Lf7n2A7+Y5If73eG3gt/i3+R3gtXleiP89Xhv4Lf51/gb4aeCzef4Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcJIaRBmheJKwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatLegroomNormal;
impl IconShape for MdAirlineSeatLegroomNormal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 12V3H3v9c0 2.76 2.24 5 5 5h6v-2H8c-1.66 0-3-1.34-3-3zm15.5 6H19v-7c0-1.1-.9-2-2-2h-5V3H6v8c0 1.65 1.35 3 3 3h7v7h4.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEfklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4l5nn9TrAb3OF+fcR/30Q/zLzvF4H+G2uMP8+4r8P4l9mntfrAL/NFebfR/z3QfzLzPN6HeC3ucL8+4j/Poh/mXlerwP8NleYfx/x3wfxLzPP63WA3+YK8+8j/vsg/mXmeb0O8NtcYf59xH8fxL/MPK/XAX6bK8y/j/jvg/iXmef1OsBvc4X59xH/fRD/MvO8Xgf4ba4w/z7ivw/iX2ae1+sAv80V5t9H/PdB/MvM83od4Le5wvz7iP8+iH+ZeV6vA/w2V5h/H/HfB/EvM8/rdYDf5grz7yP++yD+ZeZ5vQ7w21xh/n3Efx/Ev8w8r9cBfpsrzL+P+O+D+JeZ5/U6wG9zhfnvcSvw3cDXALv82yD+ZeZ5vQ7w21xh/nv9NfA2wK386yH+ZeZ5vQ7w21xh/vv9NfAy/Osh/mXmeb0O8NtcYf5neBvgp/nXQfzLzPN6HeC3ucL8z/A5wGfzr4P4l5nn9TrAb3OF+Z/he4D35l8H8S8zz+t1gN/miluBB/Hf722An+ZfB/EvM8/rdYDf5orvBt6L/34ngF3+dRD/MvO83gf4bq54b+C7+O/1O8Br86+H+Jf9NfBSPKfPAT6bZ/tt4LX47/MxwFfzr4f4l/028Fo8p78GXoZnezDw18Ax/nu8DPDX/Osh/mUfDXwVz+shwK0820sD3w28FP+1ngE8mH8bxL/spYG/4nn9NvA6PKfjwEcD7w08iP8a3wO8N/82iBfNbwOvxfP6buB9eP5eGjjOv89x4Kd44d4G+Gn+bRAvmtcGfovn77uBjwF2+Y/31sBP8cKdAHb5t0G86L4beC+ev13gu4GfBn6H/zhfDXwUL9jvAK/Nvx3iRXcc+G3gpfiPJV6wvwJemhfsY4Cv5t8O8a/zYOCngZfiP454/o4DF3nhXgb4a/7tEP96x4GfBl6L/xji+Xtr4Kd4wZ4BPJh/H8S/3XsDnw08iH8f8fx9NfBRvGDfA7w3/z6If5/jwFsDbw28Ff824vn7K+ClecHeBvhp/n0Q/7FeGjjOv85v87yOAxd54U4Au/z7IP5nemvgp3jBfgd4bf79EP8zfTXwUbxgHwN8Nf9+iP+Zfht4Lf51fhr4GuC3edEh/mf6beC1+Ld5H+C7edEg/mf6beC1+LfZBU7wokH8z/TbwGvxb/c6wG/zL0P8z/TbwGvxb/c6wG/zL0P8z/TbwGvxb3MJeDCwy78M8T/TbwOvxb/N+wDfzYsG8T/TbwOvxb/O3wCfDfw0LzrE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hExdp9BGY35OQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatLegroomReduced;
impl IconShape for MdAirlineSeatLegroomReduced {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.97 19.2c.18.96-.55 1.8-1.47 1.8H14v-3l1-4H9c-1.65 0-3-1.35-3-3V3h6v6h5c1.1 0 2 .9 2 2l-2 7h1.44c.73 0 1.39.49 1.53 1.2zM5 12V3H3v9c0 2.76 2.24 5 5 5h4v-2H8c-1.66 0-3-1.34-3-3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatReclineExtra;
impl IconShape for MdAirlineSeatReclineExtra {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.35 5.64c-.9-.64-1.12-1.88-.49-2.79.63-.9 1.88-1.12 2.79-.49.9.64 1.12 1.88.49 2.79-.64.9-1.88 1.12-2.79.49zM16 19H8.93c-1.48 0-2.74-1.08-2.96-2.54L4 7H2l1.99 9.76C4.37 19.2 6.47 21 8.94 21H16v-2zm.23-4h-4.88l-1.03-4.1c1.58.89 3.28 1.54 5.15 1.22V9.99c-1.63.31-3.44-.27-4.69-1.25L9.14 7.47c-.23-.18-.49-.3-.76-.38-.32-.09-.66-.12-.99-.06h-.02c-1.23.22-2.05 1.39-1.84 2.61l1.35 5.92C7.16 16.98 8.39 18 9.83 18h6.85l3.82 3 1.5-1.5-5.77-4.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/OY4D7wW8NFf8NfA9wC7/syD+47038FXAcZ7TLvAxwHfzPwfiP9ZLA3/FC/cywF/zPwPiP9ZfAS/NC/fXwMvwPwPiP86DgafzonkIcCv//RD/cV4b+C1eNK8D/Db//RD/cV4a+CteNC8D/DX//RD/sXaBY7xwl4Dj/M+A+I/10cBX8cJ9DPDV/M+A+I/33cB78fx9D/De/M+B+M/x3sB7A6/FFb8DfDfw3fzPgvj/DfH/G+JFdxx4KZ7X7/CCvTRwjCv+BtjlfxbEi+61gd/ieYnn9V7AVwPHeU7fDXwMsMv/DIgX3WsDv8XzEs/pu4D35gW7FXgb4K/574d40b028Fs8L/Fs7w18F/+yvwZeB9jlvxfiRffawG/xvMSzXQSO86L5a+B1gF3++yBedK8N/BbPS1zx0sBf8a/z28Dr8N8H8aJ7beC3eF7iitcGfot/ve8G3of/HogX3WsDv8XzEle8NvBb/Nt8N/A+/NdDvOheG/gtnpe44rWB3+Lf7reBtwF2+a+DeNG9NvBbPC9xxWsDv8W/zy7w2cDX8F8D8aJ7beC3eF7iitcGfov/GLcCnw18D/+5EC+61wZ+i+clrnht4Lf4j7UL/DTw08DfALfyHwvxontt4Ld4XuKK1wZ+ixfd33DFS/Gv89fA+wB/zb8f4kX32sBv8bzEFa8N/BYvut8BXhv4bOCz+Nd5HeC3+fdDvOheG/gtnpe44rWB3+JF9zvAa3PFSwNfDbwWL5rXAX6bfz/Ei+61gd/ieYkrXhv4LV50vwO8Ns/ptYHPBl6LF+51gN/m3w/xontt4Ld4XuKK1wZ+ixfd7wCvzfP30sBHA28NHON5vQ7w2/z7IV50rw38Fs9LXPHawG/xovsd4LX5l7008NbASwPHgZcG3hr4bf79EC+61wZ+i+clrnht4Ld40f0O8Nr890K86F4b+C2el7jitYHf4kX3O8Br898L8aJ7beC3eF7iitcGfosX3e8Ar81/L8SL7rWB3+J5iSteG/gtXnS/A7w2/70QL7rXBn6L5yWueG3gt/iP9z7Ad/OfA/Gie23gt3he4oqXBv6K/xzvA3w3//EQL7rXBn6L5yWe7VbgQfzneB/gu/mPhXjRvTbwWzwv8WyfDXwW/3neB/hu/uMgXnQvDfwVz+t1gN/miuPAXwMP4j/P+wDfzX8MxL+OeV5fA3w0z/bSwG8Dx/jP8z7Ad/Pvh/jX+WngrXhOu8BDgF2e7aWBrwZei/887wN8N/8+iH+d9wa+i+f13cD78LzeG3hr4KWBB/Ef732A7+bfDvGvcxz4a+BBPK/vBt6H/3jvDXwXL9j7AN/Nvw3iX++9ge/i+ftt4H2AW/mP9d7Ad/GCvQ/w3fzrIf5tfht4LV6wvwZ+GrgVuJV/n9/hivcGvosX7H2A7+ZfB/Fvcxz4beCl+M8nnu29ge/iBXsf4Lt50SH+7R4M/DTwUvznEs/pvYHv4gV7H+C7edEg/n2OA98NvBX/ecTzem/gu3jBHgLcyr8M8R/jtYHPBl6L/3ji+Xtv4Lt4Xu8DfDcvGsR/rJcGXht4a+A48FL8+4kX7L2B7+LZ3gf4bl50iP/93hv4LuB9gO/mXwfxf8ODgVv510P8/4b4/w3x/xvi/zfE/2+I/9/4R7FssEGez8S+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdAirlineSeatReclineNormal;
impl IconShape for MdAirlineSeatReclineNormal {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.59 5.41c-.78-.78-.78-2.05 0-2.83.78-.78 2.05-.78 2.83 0 .78.78.78 2.05 0 2.83-.79.79-2.05.79-2.83 0zM6 16V7H4v9c0 2.76 2.24 5 5 5h6v-2H9c-1.66 0-3-1.34-3-3zm14 4.07L14.93 15H11.5v-3.68c1.4 1.15 3.6 2.16 5.5 2.16v-2.16c-1.66.02-3.61-.87-4.67-2.04l-1.4-1.55c-.19-.21-.43-.38-.69-.5-.29-.14-.62-.23-.96-.23h-.03C8.01 7 7 8.01 7 9.25V15c0 1.66 1.34 3 3 3h5.07l3.5 3.5L20 20.07z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdBluetoothAudio;
impl IconShape for MdBluetoothAudio {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+d7kE/DUvHOKFe23gu4AH87/TrcD7AL/N84d4wd4b+C7+b3gf4Lt5Xojn7zjwdOA4/zfsAg8BdnlOiOfvvYHv4v+WjwG+mueEeP6+Gvgo/m/5GuCjeU6I5++3gdfi/5bfAV6b54R4/n4beC3+83wM8Nc8p5cGvor/PL8DvDbPCfH8/TbwWvzneR3gt3lOrw38Fv95fgd4bZ4T4vn7beC1+M/zOsBv85xeG/gt/vP8DvDaPCfE8/fbwGvxn+d1gN/mOb028Fv85/kd4LV5Tojn77eB1+I/z+sAv81zem3gt/jP8zvAa/OcEM/fbwOvxX+ejwb+muf00sBX85/nd4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+2rgtYGX4v+GvwF+G/honhPihXsw8NXAW/G/0+8A7w3cyvOHeNF8N/Be/O/yPcB788IhXjTHgVuBY/zH+Bjgr3lOLw18Ff8xLgEPBnZ54RAvuq8GPor/GK8D/DbP6bWB3+I/xtcAH82/DPGie2/gu/iP8TrAb/OcXhv4Lf5jvA/w3fzLEC+6jwa+iv8YrwP8Ns/ptYHf4j/G+wDfzb8M8aL7auCj+I/xOsBv85xeG/gt/mN8DfDR/MsQL5rjwNOB4/zH+Gjgr3lOLw18Nf8xdoGHALu8cIgXzXcB783/Lt8NvA8vHOKFezDwXcBr87/TbwPvA9zK84d4/r4aeC3gpfm/4a+B3wE+mueEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+I/z8cAf81zemngq/jP8zvAa/OcEM/fbwOvxX+e1wF+m+f02sBv8Z/nd4DX5jkhnr/fBl6L/zyvA/w2z+m1gd/iP8/vAK/Nc0I8f78NvBb/eV4H+G2e02sDv8V/nt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Of5aOCveU4vDXw1/3l+B3htnhPi+ftt4LX4v+V3gNfmOSGev+8G3ov/W74G+GieE+L5+2jgq/i/5WOAr+Y5IZ6/48CtwDH+b7gEPBjY5TkhXrD3Br6L/xveBvhpnhfihXtt4LuBB/G/0zOA9wZ+m+cP8aJ5aeA4/7vsAn/NC4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wh8pYZBr3IbXQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdConfirmationNumber;
impl IconShape for MdConfirmationNumber {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22,10V6c0-1.11-0.9-2-2-2H4C2.9,4,2.01,4.89,2.01,6v4C3.11,10,4,10.9,4,12s-0.89,2-2,2v4c0,1.1,0.9,2,2,2h16 c1.1,0,2-0.9,2-2v-4c-1.1,0-2-0.9-2-2S20.9,10,22,10z M13,17.5h-2v-2h2V17.5z M13,13h-2v-2h2V13z M13,8.5h-2v-2h2V8.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDirectionsOff;
impl IconShape for MdDirectionsOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.41,6.58L12,4h0l8,8l-2.58,2.59L18.83,16l2.58-2.59c0.78-0.78,0.78-2.05,0-2.83l-8-8c-0.78-0.78-2.05-0.78-2.83,0 L8,5.17L9.41,6.58z",
            }
            path {
                d: "M2.81,2.81L1.39,4.22L5.17,8l-2.58,2.59c-0.78,0.78-0.78,2.05,0,2.83l8,8c0.78,0.78,2.05,0.78,2.83,0L16,18.83l3.78,3.78 l1.41-1.41L2.81,2.81z M12,20l-8-8l2.58-2.59L8.17,11H7v2h3.17l1.5,1.5l-1.08,1.09L12,17l1.09-1.09l1.5,1.5L12,20z",
            }
            rect {
                height: "7.07",
                transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.0134 12.8107)",
                width: "1.54",
                x: "13.19",
                y: "6.51",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviP9dx4KV4Xs8AbuW/H+I/1oOBtwJeG3ht4Dgv3G8Dvw38DPDX/NdD/Md4beCjgLfm3+5W4LOB7+G/DuLf56WBrwJem/84twKfDXwP//kQ/3afBXw2/3l+G3gbYJf/PIh/vePATwGvzX++XeB1gL/mPwfiX+fBwE8BL81/rfcBvpv/eIgX3XHgt4CX5r/H+wDfzX8sxIvmOPBbwEvz32cXeB3gr/mXvTRwjOd0CfhrnhPiRfNdwHvz3+9W4GWAXV643wZei+f0O8Br85wQ/7LXBn6L/zm+B3hvXrjfBl6L5/Q7wGvznBD/sqcDD+Z/ltcBfpsX7LeB1+I5/Q7w2jwnxAv33sB38T/PbwOvwwv228Br8Zx+B3htnhPihXs68GD+Z3od4Ld5/n4beC2e0+8Ar81zQrxgbw38FP9zfQ/w3jx/vw28Fs/pd4DX5jkhXrDvBt6L/7l2gRM8f78NvBbP6XeA1+Y5IV6wi8Bx/mP8DbDLFa/Ff5zXAX6b5/XbwGvxnH4HeG2eE+L5e2ngr/j3+x3gvYFbebbjwEcDn8W/3+cAn83z+m3gtXhOvwO8Ns8J8fy9N/Bd/Pt8D/DevGDvDXwX/z6/A7w2z+u3gdfiOf0O8No8J8Tz99nAZ/Fvdwl4MLDLC/fdwHvxb/fXwMvwvH4beC2e0+8Ar81zQjx/nw18Fv92XwN8NP+y1wZ+i38f8bx+G3gtntPvAK/Nc0I8f78NvBb/du8DfDcvGvPvI57XbwOvxXP6HeC1eU6I5++3gdfi3+59gO/mRWP+fcS/HeL5+2zgs/i3+xrgo/mXvTbwW/z7iH87xPP32cBn8W+3CzwE2OWF+27gvfi3+xvgpfm3Qzx/7w18F/8+3w28Dy/YewPfxb/P7wCvzb8d4vl7aeCv+Pf7beB9gFt5tuPARwGfzb/f5wCfzb8d4gXbBY7xH+OvgV2ueG3+47wO8Ns8r68GXorn9DfAR/OcEC/YdwPvxf9cl4DjPH+/DbwWz+l3gNfmOSFesLcGfor/ub4HeG+ev98GXovn9DvAa/OcEC/crcCD+J/pdYDf5vn7beC1eE6/A7w2zwnxwr038F38z/M7wGvzgv028Fo8p98BXpvnhPiX3Qo8iP9ZXgf4bV6wrwZemuf018BH85wQ/7LXBn6L/zm+B3hv/mMgXjTfDbwX//2eAbw0sMt/DMSL5jjw28BL8d/nEvDawF/zHwfxojsO/DbwUvz3eBvgp/mPhfjXOQ78NvBS/Ne5BHw08N38x0P86x0Hfhp4Lf7zXQJeG/hr/nMg/u0+G/gs/vP8DvDWwC7/eRD/Pi8NfDXwWvzHeQbw0cBP858P8R/jtYGPBt6Kf7u/Ab4a+G7+6yD+Yx0H3ht4beC1gWO8cL8D/Dbw08Bf818P8Z/rOPDSPK9bgVv574f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wiLhalBmilrKwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDiscFull;
impl IconShape for MdDiscFull {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 16h2v-2h-2v2zm0-9v5h2V7h-2zM10 4c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 10c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotDisturb;
impl IconShape for MdDoNotDisturb {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIA0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dY4DLwW8NHCcK16aK/6aK3aBvwb+BtjlPx/iP9drA28FvDbw0vzr/DXw28DPAL/Nfw7Ef7zjwEcB7w08mP8YtwLfDXwNsMt/HMR/nOPARwEfDRznP8cu8NXA1wC7/Psh/mO8NfBVwIP5r3Er8DHAT/Pvg/j3OQ58FfDe/Ov8DbDLc3ow8CD+db4b+Bhgl38bxL/dSwPfBbw0/7KfAX4b+GngVl64lwZeG3ht4K34l/018D7AX/Ovh/i3eWngt4DjvGDPAL4b+Gpgl3+bBwPvDXw0cIwXbBd4HeCv+ddB/Ou9NPBbwHGev0vAVwOfzX+c48BHA5/FC7YLvA7w17zoEP86Lw38FnCc5+93gPcGbuU/x0sD3w28FM/fLvA6wF/zokG86I4DvwW8NM/f5wCfzX++lwb+ihfsr4HXAXb5lyFedN8FvDfP3/sA381/vpcGfgs4zgv33cD78C9DvGjeGvgpnr/3Ab6b/3wvDfwWcJwXzdsAP80Lh/iXHQf+Cngwz+tzgM/mP99LA78FHOdFdyvwMsAuLxjiX/bZwGfxvH4HeG3+87008FvAcZ7XJeC1ge8GXorn9TnAZ/OCIV6448DTgeM8p0vASwO38p/rpYHfAo7zvC4Brw38NfDSwF/xvHaBhwC7PH+IF+6zgc/ieX0O8Nn853pp4LeA4zyvS8BrA3/Ns3028Fk8r88BPpvnD/HCPR14MM/pGcCD+c/10sBvAcd5XpeA1wb+mud0HLgVOMZzuhV4CM8f4gV7beC3eF6fA3w2/3leGvgt4DjP6xLw2sBf8/x9NvBZPK/XAX6b54V4wb4a+Cie1wlgl/8cLw38FnCc53UJeG3gr3nBHgw8nef1NcBH87wQL9hfAS/Nc/oZ4K35z/HSwG8Bx3lel4DXBv6af9lvA6/Fc/pr4GV4Xojn7zhwkef1McBX8x/vpYHfAo7zvC4Brw38NS+ajwa+iud1AtjlOSGev9cGfovn9RDgVv5jvTTwW8Bxntcl4LWBv+ZF99LAX/G8Xgf4bZ4T4vn7aOCreF7iP9ZLA78FHOd5XQJeG/hr/vXM8/oY4Kt5Tojn77OBz+I5/Q3w0vzHeWngt4DjPK9LwGsDf82/za3Ag3hOnwN8Ns8J8fx9NvBZPKffAV6b/xgvDfwWcJzndQl4beCv+bf7beC1eE6fA3w2zwnx/P028Fo8p98BXpt/v5cGfgs4zvO6BLw28Nf8+/w28Fo8p58B3prnhHj+fht4LZ7T7wCvzb/PSwO/BRzneV0CXhv4a/79fht4LZ7TzwBvzXNCPH+fDXwWz+l3gNfm3+6lgd8CjvO8LgGvDfw1/zF+G3gtntPnAJ/Nc0I8f58NfBbP6VbgIfzbvDTwW8Bxntcl4LWBv+Y/zl8BL81z+hzgs3lOiOfvo4Gv4nmJf72XBn4LOM7zugS8NvDX/Mcyz+tjgK/mOSGev9cGfovn9TLAX/Oie2ngt4DjPK9LwGsDf81/rJcG/orn9TrAb/OcEM/fceAiz+tjgK/mRfPSwG8Bx3lel4DXBv6a/3gfDXwVz+sEsMtzQrxgfw28FM/pZ4C35l/20sBvAcd5XpeA1wb+mv8cPw28Fc/pb4CX5nkhXrCvBj6K5/UQ4FZesJcGfgs4zvO6BLw28Nf85zgOXOR5fQ3w0TwvxAv22sBv8bw+B/hsnr+XBn4LOM7zugS8NvDX/Of5bOCzeF6vA/w2zwvxwt0KPIjntAs8BNjlOb008FvAcZ7XJeC1gb/mP9dF4DjP6RnAg3n+EC/cZwOfxfP6HOCzebaXBn4LOM7zugS8NvDX/Of6bOCzeF6fA3w2zx/ihTsO3Aoc43m9DPDXwEsDvwUc53ldAl4b+Gv+cz0Y+CvgOM/pEvBgYJfnD/Ev+2zgs3hefw28D/BbwHGe1yXgtYG/5j/fbwGvzfP6HOCzecEQ/7LjwF8DD+JFdwl4beCv+c/31cBH8byeAbw0sMsLhnjRvDXwU7xoLgGvDfw1//neG/gunr+3AX6aFw7xovtu4L144S4Brw38Nf/53hv4Lp6/7wHem38Z4kV3HPht4KV4wV4G+Gv+830V8NE8f38DvDawy78M8a/z0sBvA8d4/v4aeB/gr/nP8WDgu4DX5vm7BLw28Ne8aBD/ei8N/DZwjBfss4GvAXb5j/NZwEcDx3n+LgGvDfw1LzrEv81LA78NHOMF2wW+Gvge4Fb+bY4DHwV8NHCcF+wS8NrAX/Ovg/i3e2ngu4GX4l/228BPA78D/DUv3EsDrwW8NvDW/Mv+Bnhv4K/510P8+xwHvhp4L/51bgVu5TkdB16af53vAT4a2OXfBvEf462BrwYexH+NZwAfDfw0/z6I/zjHgY8GPho4xn+OS8BXA18N7PLvh/iPdxz4aOC9gQfxH+MZwHcDXw3s8h8H8Z/rtYG3Bl4beCn+df4G+G3gu4G/5j8H4r/OceClgZcGjnPFS3PFX3PFLvDXwF8Du/znQ/z/hvj/DfH/G+L/N8T/b/wj9JdEUKEogwIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotDisturbAlt;
impl IconShape for MdDoNotDisturbAlt {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zM4 12c0-4.4 3.6-8 8-8 1.8 0 3.5.6 4.9 1.7L5.7 16.9C4.6 15.5 4 13.8 4 12zm8 8c-1.8 0-3.5-.6-4.9-1.7L18.3 7.1C19.4 8.5 20 10.2 20 12c0 4.4-3.6 8-8 8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/tV4LeGngOFc8GDgO/DVX7AJ/DTwDuJX/fIjn772B1wLeh3+f1wbeCnht4KX51/lr4LeBnwF+m/8ciOf13sB3ccV3A+/Dv85x4L2AjwYezH+MW4HvBr4G2OU/DuI5vTfwXTyn7wbehxfNZwEfDRznP8cu8NXA1wC7/Pshnu3BwNN5/r4beB9esLcGvgp4MP81doGPBr6Hfx/Ec3pv4Lt4/r4beB+ev+8G3ov/ej8NvA+wy78N4nm9N/BdPH/fDbwPz993A+/Ff71bgbcB/pp/PcTz997Ad/H8fTfwPjx/3w28F//1doHXAf6afx3EC/bewHfx/H038D48f98NvBf/9XaB1wH+mhcd4oV7b+C7eP6+G3gfnr/vBt6L/3q7wOsAf82LBvEve2/gu3j+vht4H56/7wbei/96twIvA+zyL0O8aN4b+C6ev+8G3ofn77uB9+K/3k8Db8O/DPGieWvgp3jBvht4H56/7wbei/967wN8Ny8c4kXzdODBvHDfDbwPz993A+/Ff61d4CHALi8Y4l/22cBn8aL5buB9eP6+G3gv/mt9DvDZvGCIF+448HTgOC+67wbeh+fvu4H34r/OLvAQYJfnD/HCfTTwVfzrfTfwPjx/3w28F/91Pgf4bJ4/xAv3dODB/Nt8N/A+PH/fDbwX/zVuBR7C84d4wV4b+C3+fb4beB+ev+8G3ov/Gq8D/DbPC/GCfTXwUfz7fTfwPjx/3w28F//5vgb4aJ4X4gX7K+Cl+Y/x3cD78Px9N/Be/Of6a+BleF6IF8z8x/pu4H14/r4beC/+c50AdnlOiOfvtYHf4j/edwPvw/P33cB78Z/ndYDf5jkhnr+PBr6K/xzfDbwPz993A+/Ff46PAb6a54R4/j4b+Cz+df4G+Gie7aWBr+L5+27gfXj+Xpt/2VcDL8W/zucAn81zQjx/nw18Fv86vwO8Ns/pvYHv4vn7buB9+Lf5beC1+Nf5HOCzeU6I5++7gffiX+d3gNfmeb038F08f98NvA//er8NvBb/Oj8DvDXPCfH8/TbwWvzr/A7w2jx/7w18F8/fdwPvw7/ObwOvxb/O9wDvzXNCPH+fDXwW/zq/A7w2L9h7A9/F8/fdwPvwovtt4LX41/kc4LN5Tojn77OBz+Jf56+Bj+aFe2/gvXn+vht4H56/48BL8WxfDbw0/zqfA3w2zwnx/H008FX81/tu4H14/r4beC/+7T4G+GqeE+L5e23gt/jv8d3A+/D8fTfwXvzbvA7w2zwnxPP3YODp/Pf5buB9eP6+G3gv/vXE80K8YH8NvBT/fb4beB+ev+8G3osX3e8Ar83zQrxgXw18FP+9vht4H56/7wbeixfN1wAfzfNCvGCvDfwW//2+G3gfnr/vBt6Lf9nrAL/N80K8cLcCD+K/33cD78Pz993Ae/GCPQN4MM8f4oX7bOCz+J/hu4H34fn7buC9eP4+Bvhqnj/EC3ccuBU4xv8M3w28D8/fdwPvxXO6BDwY2OX5Q/zLPhv4LP7n+G7gfXj+vht4L57tc4DP5gVD/MuOA7cCx/if47uB9+H5+27gvYBnAA/mhUO8aN4b+C7+Z/lu4H14/r4b+G7gt3nhEC+6nwbeiv9Zvht4H/7tEC+648BfAw/if5bvBt6HfxvEv85LA78NHON/lu8G3od/PcS/3ksDvw0c43+W7wbeh38dxL/NSwO/DRzjf5bvBt6HFx3i3+6lgZ8GHsT/LN8NvA8vGsS/z3Hgu4G34n+W7wbeh38Z4j/GewNfDRzjv8YzgPcGHgx8F8/fdwPvwwuH+I9zHPho4KOBY/znuAR8NfDZPNt7A9/F8/fdwPvwgiH+4x0HPhp4b+BB/Md4BvDVwHcDuzyv9wa+i+fvu4H34flD/Od6beCtgdcGXop/nd8B/hr4aeC3+Ze9N/BdPH/fDbwPzwvxX+c48NLASwPHueKlgV3gVq7YBf4a+G3+bd4b+C6e1/sA383zQvzf897Ad/Fs7wN8N88f4v+m9wa+C3gf4Lt5wRD/dz0YuJUXDvH/G+L/N/4RaIXyQeAe5WIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotDisturbOff;
impl IconShape for MdDoNotDisturbOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 11v2h-1.46l4.68 4.68C21.34 16.07 22 14.11 22 12c0-5.52-4.48-10-10-10-2.11 0-4.07.66-5.68 1.78L13.54 11H17zM2.27 2.27L1 3.54l2.78 2.78C2.66 7.93 2 9.89 2 12c0 5.52 4.48 10 10 10 2.11 0 4.07-.66 5.68-1.78L20.46 23l1.27-1.27L11 11 2.27 2.27zM7 13v-2h1.46l2 2H7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/dR4MPAh4aeA4V7w0V/w1V+wCfw08A7iV/3yI/1yvDbwV8NrAS/Ov89fAbwM/A/w2/zkQ//GOAx8FvDfwYP5j3Ap8N/A1wC7/cRD/cY4DHwV8NHCc/xy7wFcDXwPs8u+H+I/xXsBXA8f5r7ELfDTwPfz7IP59jgPfBbw1/z1+GngfYJd/G8S/3UsDPwU8mP9etwJvA/w1/3qIf5uXBn4LOM7/DLvA6wB/zb8O4l/vpYHfAo7zP8su8DrAX/OiQ/zrvDTwW8Bx/mfaBV4H+GteNIgX3XHgr4AH8z/brcDLALv8yxAvup8C3pr/HX4aeBv+ZYgXzXsD38X/Lu8DfDcvHOJfdhx4OnCc/112gYcAu7xgiH/ZZwOfxf9OnwN8Ni8Y4oU7DjwdOM7/TrvAQ4Bdnj/EC/fZwGfxv9vnAJ/N84d44Z4OPJj/3W4FHsLzh3jB3hr4Kf5veB3gt3leiBfsq4GP4v+GrwE+mueFeMH+Cnhp/m/4a+BleF6I5+/BwNP5v+UEsMtzQjx/rw38Fv+3vA7w2zwnxPP30cBX8X/LxwBfzXNCPH+fDXwW/zp/A3w0/zW+Gngp/nU+B/hsnhPi+fts4LP41/kd4LX5r/HbwGvxr/M5wGfznBDP328Dr8W/zu8Ar81/jd8GXot/nZ8B3prnhHj+fht4Lf51fgd4bf5r/DbwWvzr/Azw1jwnxPP32cBn8a/zO8Br81/jt4HX4l/nc4DP5jkhnr/PBj6Lf52/Bj6a/xpfDbw0/zqfA3w2zwnx/H008FX83/IxwFfznBDP32sDv8X/La8D/DbPCfH8PRh4Ov+3nAB2eU6IF+yvgZfi/4a/AV6a54V4wb4a+Cj+b/ga4KN5XogX7LWB3+L/htcBfpvnhXjhbgUexP9uzwAezPOHeOE+G/gs/nf7HOCzef4QL9xx4FbgGP87XQIeDOzy/CH+ZZ8NfBb/O30O8Nm8YIh/2XHgVuAY/7tcAh4M7PKCIV407w18F/+7vA3w07xwiBfdTwNvxf8OPwO8Nf8yxIvuOPDXwIP4n+0ZwEsDu/zLEP86Lw38NnCM/5kuAa8N/DUvGsS/3ksDvw0c43+WS8BrA3/Niw7xb/PSwG8Dx/if4RLw2sBf86+D+Ld7aeCngQfx3+sZwFsDf82/HuLf5zjw3cBb8d/jZ4D3Bnb5t0H8x3hv4KuBY/zXuAS8N/DT/Psg/uMcBz4a+GjgGP85LgFfDXw1sMu/H+I/3nHgo4H3Bh7Ef4xnAN8NfDWwy38cxH+u1wbeGnht4KX41/kb4LeBnwZ+m/8ciP86x4GXBl4aOM4VL80Vf80Vu8BfA38N7PKfD/H/G+L/N8T/b4j/3xD/v/GPXU2cQW9x8/wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDoNotDisturbOn;
impl IconShape for MdDoNotDisturbOn {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11H7v-2h10v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3++1+O/1O/zbIf7tXhr4KeDB/Pe6FXgb4K/510P82xwHng4c53+GXeAhwC7/Ooh/m88GPov/WT4H+Gz+dRD/Nk8HHsyz/Q2wy3+t48BL8Wy3Ag/hXwfxr/fSwF/xnE4Au/zXOg5c5Dm9DPDXvOgQ/3pfDXwUz/YzwFvz3+Ongbfi2b4G+GhedIh/vacDD+bZ3gf4bv57vDfwXTzbrcBDeNEh/nVeGvgrntMJYJf/HseBizynlwH+mhcN4l/nu4H34tl+Bnhr/nv9NPBWPNv3AO/Niwbxr3MROM6zvQ/w3fz3em/gu3i2XeAELxrEi+6tgZ/iOZ0AdvnvdRy4yHN6G+Cn+ZchXnTfDbwXz/YzwFvzP8NPA2/Fs30P8N78yxAvuovAcZ7tfYDv5n+G9wa+i2fbBU7wL0O8aN4a+Cme0wlgl/8ZjgMXeU5vA/w0LxziRfPTwFvxbD8DvDX/s/w08FY82/cA780Lh/iXHQcu8py+Gvhp/md5a+CjeU4ngF1eMMS/7L2B7+J/p/cBvpsXDPEv+2ngrfjf6WeAt+YFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzAv2OcBPA38NvDbw2cBr8Z/rd4DPBn4beGngrYHP4gUTLxjiX2aev9cBfpvn9d3Ae/Gf43uA9+Z5vTbwWzx/4gVD/MvM8/oe4L15/o4DF/nPcQLY5fn7buC9eF7iBUP8y8zz+hjgq3nBfht4Lf5j/Q7w2rxgHw18Fc9LvGCIf5l5Xh8DfDUv2G8Dr8V/rN8BXpsX7KOBr+J5iRcM8S8zz+t7gPfm+TsOXOQ/xwlgl+fvu4H34nmJFwzxLzPP3+sAv83z+i7gvfnP8d3A+/C8Xhv4LZ4/8YIh/mXmBfts4GeAvwZeC/hs4LX5z/XbwGcDvwO8NPBWwGfzgokXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzMvnPjvZV448YIh/mXmhRP/vcwLJ14wxL/MvHDiv5d54cQLhviX/TXwUjx/fwO8NP+9/hp4KZ6/vwFemhcM8S97aeC3gWM8p0vAawN/zX+vlwZ+GzjGc7oEvDbw17xgiBfNceCleU5/DezyP8Nx4KV5Tn8N7PLCIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Aql7jEH7zTyMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdDriveEta;
impl IconShape for MdDriveEta {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.92 5.01C18.72 4.42 18.16 4 17.5 4h-11c-.66 0-1.21.42-1.42 1.01L3 11v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 15c-.83 0-1.5-.67-1.5-1.5S5.67 12 6.5 12s1.5.67 1.5 1.5S7.33 15 6.5 15zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 10l1.5-4.5h11L19 10H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEnhancedEncryption;
impl IconShape for MdEnhancedEncryption {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H8.9V6zM16 16h-3v3h-2v-3H8v-2h3v-3h2v3h3v2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpv/Og8GbuWFe23gt3he4gVD/MvM83od4Lf5r/FdwFsDrwP8NS/YawO/xfMSLxjiX2ae1+sAv81/vu8C3psrdoHXAf6a5++1gd/ieYkXDPEvM8/rdYDf5j/XdwHvzXN6BvBgnr/XBn6L5yVeMMS/zDyv1wF+m/883wW8N8/pEvDawF/z/L028Fs8L/GCIf5l5nm9DvDb/Of4LuC9eU6XgNcG/poX7LWB3+J5iRcM8S8zz+t1gN/mP953Ae/Nc7oEvDbw17xwrw38Fs9LvGCIf5l5Xq8D/Db/sb4LeG+e0yXgtYG/5l/22sBv8bzEC4b4l5nn9TrAb/OCHQd+CvgY4K/5l30X8N48p0vAawN/zYvmtYHf4nmJFwzxLzPP63WA3+b5Ow78FvDSwC7wOsBf84J9F/DePKdLwGsDf82L7rWB3+J5iRcM8S8zz+t1gN/meR0Hfgt4aZ5tF3gd4K95Xt8FvDfP6RLw2sBf86/z2sBv8bzEC4b4l5nn9TrAb/P8fTfwXjynXeB1gL/m2b4LeG+e0yXgtYG/5l/vtYHf4nmJFwzxLzPP63WA3+YF+27gvXhOu8DrAH8NfBfw3jynS8BrA3/Nv81rA7/F8xIvGOJfZp7X6wC/zQv33cB78Zx2gd8G3prndAl4beCv+bd7beC3eF7iBUP8y8zzeh3gt/mXfTfwXrxwl4DXBv6af5/XBn6L5yVeMMS/zDyv1wF+mxfNdwPvxfN3CXht4K/593tt4Ld4XuIFQ/zLzPN6HeC3edF9N/BePKdLwGsDf81/jNcGfovnJV4wxL/MPK/XAX6bf53vBt6LKy4Brw38Nf9xXhv4LZ6XeMEQ/zLzvF4H+G3+9b4beGvgtYG/5j/WawO/xfMSLxjiX2ae1+sAv82/zYOBW/mP99rAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9dHAX/M/y0sDX83zEi8Y4l+2Cxzjf6dLwHFeMMS/7LuB9+J/p+8B3psXDPEve2ngr/jf6SHArbxgiBfNewPfxf8u7wN8Ny8c4kX3YOCzgbcGjvE/0yXgp4HPBm7lX4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIpktFBVP1UHAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEventAvailable;
impl IconShape for MdEventAvailable {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.53 11.06L15.47 10l-4.88 4.88-2.12-2.12-1.06 1.06L10.59 17l5.94-5.94zM19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF5klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpsXzYOBW/nXOQ7s8q/z2sBv8bzEC4b4l5nn9TrAb/Mve2ngt4CfBt6HF81x4LeAvwbehxfdawO/xfMSLxjiX2ae1+sAv80L99LAbwHHueK7gffhhTsO/Bbw0lzx3cD78KJ5beC3eF7iBUP8y8zzeh3gt3nhfht4LZ7TdwPvw/N3HPgt4KV5Tu8DfDf/stcGfovnJV4wxL/MPK/XAX6bF+448NvAS/Gcvht4H57TceC3gJfmOX0P8N68aF4b+C2el3jBEP8y87xeB/ht/mXHgd8GXorn9N3A+3DFceC3gJfmOX0P8N686F4b+C2el3jBEP8y87xeB/htXjTHgd8GXorn9N3AxwC/Bbw0z+l7gPfmX+e1gd/ieYkXDPEvM8/rdYDf5kV3HPht4KV4TrvAcZ7T9wDvzb/eawO/xfMSLxjiX2ae1+sAv82/znHgt4GX4gX7HuC9+bd5beC3eF7iBUP8y8zzeh3gt/nXOw78NvBSPK/vAd6bf7vXBn6L5yVeMMS/zDyv1wF+m3+948BvAS/N8/pu4H34t3tt4Ld4XuIFQ/zLzPN6HeC3+dc5DvwW8NK8YN8NvA//Nq8N/BbPS7xgiH+ZeV6vA/w2L7rjwG8BL81zugQc4zl9N/A+/Ou9NvBbPC/xgiH+ZeZ5vQ7w27xojgO/Bbw0z+l7gI8Gfht4KZ7TdwPvw7/OawO/xfMSLxjiX2ae1+sAv82/7DjwW8BL85y+B3hvrjgO/DbwUjyn7wbehxfdawO/xfMSLxjiX2ae1+sAv80Ldxz4LeCleU7fA7w3z+k48NvAS/Gcvht4H140rw38Fs9LvGCIf5l5Xq8D/DYv3G8Dr8Vz+h7gvXn+jgO/DbwUz+l9gO/mX/bawG/xvMQLhviXmef1OsBv88K9NPDbwDGu+B7gvXnhjgO/DbwUV3wP8N68aF4b+C2el3jBEP8y87xeB/ht/mUvDfw28NPAe/OiOQ78NvDXwHvzontt4Ld4XuIFQ/zLzPN6HeC3edE8GLiVf53jwC7/Oq8N/BbPS7xgiH+ZeV6vA/w2/7O8NvBbPC/xgiH+ZeZ5vQ7w2/zP8trAb/G8xAuG+JeZ5/U6wG/zP8trA7/F8xIvGOJfZp7X6wC/zf8srw38Fs9LvGCIf5l5Xh8N/DX/s7w08NU8L/GCIf5lu8Ax/ne6BBznBUP8y74beC/+d/oe4L15wRD/spcG/or/nR4C3MoLhnjRvDfwXfzv8j7Ad/PCIV50DwY+G3hr4Bj/M10Cfhr4bOBW/mWI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CPZ785BIFrUvgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEventBusy;
impl IconShape for MdEventBusy {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.31 17l2.44-2.44L14.19 17l1.06-1.06-2.44-2.44 2.44-2.44L14.19 10l-2.44 2.44L9.31 10l-1.06 1.06 2.44 2.44-2.44 2.44L9.31 17zM19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+Lf77V4Xn8D7PIf6zjwUjyv3+HfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0Q/37meb0O8Nv8x3pt4Ld4XuLfDvHvZ57X6wC/zX+s1wZ+i+cl/u0QL7qXBj4KeGvgOP8z7QI/DXwOcCv/MsSL5r2B7+J/l/cBvpsXDvEvezDwdP53eghwKy8Y4l/23cB78b/T9wDvzQuG+JddBI7zv9MucIIXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzPP62OAv+Z/lpcGvornJV4wxL/MPK/XAX6b/1leG/gtnpd4wRD/MvO8Xgf4bf5neW3gt3he4gVD/MvM83od4Lf5n+W1gd/ieYkXDPEvM8/rdYDf5n+W1wZ+i+clXjDEv8w8r9cBfpvndBx4Kf5r/A2wy3N6beC3eF7iBUP8y8zzeh3gt3lOrw38Fv81Xgf4bZ7TawO/xfMSLxjiX2ae1+sAv81zem3gt/iv8TrAb/OcXhv4LZ6XeMEQ/zLzvF4H+G2e02sDv8V/jdcBfpvn9NrAb/G8xAuG+JeZ5/U6wG/znF4b+C3+a7wO8Ns8p9cGfovnJV4wxL/MPK/XAX6b5/TawG/xX+N1gN/mOb028Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv85xeGvgq/mN8DPDX/Ou8NvBbPC/xgiH+ZeZ5vQ7w2zyn1wZ+i/8YrwP8Nv86rw38Fs9LvGCIf5l5Xq8D/DbP6bWB3+I/xusAv82/zmsDv8XzEi8Y4l9mntfrAL/Nc3pt4Lf4j/E6wG/zr/PawG/xvMQLhviXmef1OsBv85xeG/gt/mO8DvDb/Ou8NvBbPC/xgiH+ZeZ5vQ7w2zynlwa+mv8YHw38Nf86rw38Fs9LvGCIf5l5Xq8D/Db/s7w28Fs8L/GCIf5l5nm9DvDb/M/y2sBv8bzEC4b4l5nn9TrAb/M/y2sDv8XzEi8Y4l9mntfrAL/N/yyvDfwWz0u8YIh/mXlerwP8Nv+zvDbwWzwv8YIh/mXmeb0O8Nv8z/LawG/xvMQLhviXmef1OsBv8z/LawO/xfMSLxjiX2ae10cDf83/LC8NfDXPS7xgiH/ZLnCM/50uAcd5wRD/su8G3ov/nb4HeG9eMMS/7KWBv+J/p4cAt/KCIV407w18F/+7vA/w3bxwiBfdg4HPBt4aOMb/TJeAnwY+G7iVfxni/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wghw6RBjK3ezQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdEventNote;
impl IconShape for MdEventNote {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 10H7v2h10v-2zm2-7h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zm-5-5H7v2h7v-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEzklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRfPSwDH+/X6H/1kQL9xrA98FPJj/GN8NvA//cyBesPcGvov/eN8NvA//MyCev+PA04Hj/Of4buB9+O+HeP4+Gvgq/nN9N/A+/PdCPH/fDbwX//m+G3gf/vsgnr/fBl6L/91uBb4b+Bpgl+cP8fz9NvBa/N/w18DbALfyvBDP328Dr8X/HX8NvAzPC/H8/TbwWvzf8jbAT/OcEM/fbwOvxf8tnwN8Ns8J8fz9NvBa/Pd7BnAcOMa/3+8Ar81zQjx/vw28Fv/9PocrPot/v98BXpvnhHj+fht4Lf77PYQrns6/3+8Ar81zQjx/vw28Fv+9vgd4b674buC9+Pf5HeC1eU6I5++3gdfiv9frAL/NFa8N/Bb/Pr8DvDbPCfH8/TbwWvz3+RvgpXlOfw28FP92vwO8Ns8J8fz9NvBa/Pd5H+C7eU7vDXwX/3a/A7w2zwnx/P028Fr897gEHOf52wWO8W/zO8Br85wQz99vA6/Ff61nALcCPw18Nc/fRwNvDTwYeBD/Or8DvDbPCfH8/TbwWvzH+xvgq4FbueJW4Fb+fR4MPJgrHgx8NPBSPK/fAV6b54R4/n4beC3+c3w28Dn85/gs4LN5/n4HeG2eE+L5+23gtfjP893A+/Af67uA9+YF+x3gtXlOiOfvt4HX4j/XXwOvA+zy73Mc+C3gpXnhfgd4bZ4T4vn7beC1+M/318DbALfyb/PSwHcBL82/7HeA1+Y5IZ6/3wZei/8au8DrAH/Nv85LA78FHOdF8zvAa/OcEM/fbwOvxX+dXeB1gL/mRfPSwG8Bx3nR/Q7w2jwnxPP328Br8V9L/OuYf53fAV6b54R4/n4beC3+6/wO8Nr86/w28Fq86H4HeG2eE+L5+23gtfiv8znAZ/Ov89nAZ/Gi+x3gtXlOiOfvt4HX4r/O6wC/zfM6zhW7PK+3Bn6KF93vAK/Nc0I8f78NvBb/dU4Auzyn9wK+mis+GvgentNx4CIvut8BXpvnhHj+fht4Lf5r/A3w0jzbawOfBbw2z+m3gc8Bfptn+2vgpXjR/A7w2jwnxPP328Br8V/ja4CPBo4DXwW8Ny/cdwMfA+wCXw18FC+a3wFem+eEeP5+G3gt/mu8D/Ag4KOB47xodoGvBp4BfBcvmt8BXpvnhHj+fht4Lf5r7ALH+bfZBY7zovkd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvu4H34v+WrwE+mueEeP4+Gvgq/m/5GOCreU6I5+84cCtwjP8bLgEPBnZ5TogX7L2B7+L/hrcBfprnhXjhXhv4buBB/O/0DOC9gd/m+UO8aF4aOM7/LrvAX/PCIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AsdEnkHS6yBlAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdFolderSpecial;
impl IconShape for MdFolderSpecial {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-2.06 11L15 15.28 12.06 17l.78-3.33-2.59-2.24 3.41-.29L15 8l1.34 3.14 3.41.29-2.59 2.24.78 3.33z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/fr/F/zwfA/w1/zLEv5/5n+d1gN/mX4b49zP/87wO8Nv8yxD/fuZ/ntcBfpt/GeLfz/zP8zrAb/MvQzx/Lw0c40Xz2/zP8zrAb/MvQzx/vw28Fv97vQ7w2/zLEM/fbwOvxf9erwP8Nv8yxPP328Br8b/X6wC/zb8M8fz9NvBa/O/1OsBv8y9DPH+/DbwW/3u9DvDb/MsQz99vA6/Fc/od4LV5XuZ/ntcBfpt/GeL5+23gtXhOvwO8Ns/L/M/zOsBv8y9DPH+/DbwWz+l3gNfmeZn/eV4H+G3+ZYjn77eB1+I5/Q7w2jwv8z/P6wC/zb8M8fz9NvBaPKffAV6b52X+53kd4Lf5lyGev98GXovn9DvAa/O8Xpv/ef4a2OVfhnj+fht4LZ7T7wCvzf8tiOfvt4HX4jn9DvDa/N+CeP5+G3gtntPvAK/N/y2I5++3gdfiOf0O8Nr834J4/n4beC2e018DH81/jb8BdvnPh3j+fht4Lf77vA7w2/znQzx/vw28Fv99Xgf4bf7zIZ6/3wZei/8+rwP8Nv/5EM/fbwOvxX+f1wF+m/98iOfvt4HX4r/P6wC/zX8+xH8/87xeB/ht/vMh/vuZ5/U6wG/znw/x3888r9cBfpv/fIj/fuZ5vQ7w2/znQ/z3M8/rdYDf5j8f4r+feV6vA/w2//kQ//3M83od4Lf5z4f472ee1+sAv81/PsR/ndcGfot/u98BXpv/WIj/Oq8N/Bb/dr8DvDb/sRD/dV4b+C3+7X4HeG3+YyH+67w28Fv82/0O8Nr8x0L813lt4Lf4t/sd4LX5j4X4r/PawG/xb/c7wGvzHwvxX+e1gd/i3+53gNfmPxbiv85rA7/Fv93vAK/NfyzEf53XBn6Lf7vfAV6b/1iI/zqvDfwW/3a/A7w2/7EQ/3VeG/gt/u1+B3ht/mMh/uu8NvBb/Nv9DvDa/MdC/Nd5beC3+Lf7HeC1+Y+F+K/z2sBv8W/3O8Br8x8L8V/ntYHf4t/ud4DX5j8W4r/OawO/xb/d7wCvzX8sxH+d1wZ+i3+73wFem/9YiP86rw38Fv92vwO8Nv+xEP91Xhv4Lf7tfgd4bf5jIf7rvDbwW/zb/Q7w2vzHQvzXeW3gt/i3+x3gtfmPhfivcxx4af7tdoG/5j8W4v83xP9v/COQ04BBwEZ4FwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdImagesearchRoller;
impl IconShape for MdImagesearchRoller {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2v6H6V6H4v4h10v5h2v8h-6v-8h2v-3H2V4h4V2",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/8Y4DHwV8Dv+xPgv4GmCX/ziI/1jHgd8CXhr4buB9+I/xXcB7A38NvA6wy38MxH+stwZ+imf7buB9+Pf5LuC9eba3AX6a/xiI/3jvDXwXz/bdwPvwb/NdwHvzbO8DfDf/cRD/Od4b+C6e7buB9+Ff57uA9+bZ3gf4bv5jIf7zvDfwXTzbdwPvw4vmu4D35tneB/hu/uMh/nO9N/BdPNt3A+/DC/ddwHvzbO8DfDf/ORD/+d4b+C6e7buB9+H5+y7gvXm29wG+m/88iP8a7w18F8/23cD78Jy+C3hvnu19gO/mPxfiv857A9/Fs3038D5c8V3Ae/Ns7wN8N//5EP+13hv4Lp7tu7nivXm29wG+m/8aiOfvvYCPBl6a/xv+Gvhq4Ht4Tojn9V3Ae/N/03cD78OzIZ7TZwOfxf9tnwN8NlcgntNF4Dj/t+0CJ7gC8WyvDfwW/z+8DvDbAOLZXhv4LZ7X6/C/22/xvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xL/PcWCX/z7meb0O8NsA4tleG/gtnpf497kIfDXwOfz3MM/rdYDfBhDP9trAb/G8xL+PueJW4GOAn+a/lnlerwP8NoB4ttcGfovnJf59zHP6beBjgL/mv4Z5Xq8D/DaAeLbXBn6L5yX+fczz993AxwC7/Ocyz+t1gN8GEM/22sBv8bzEv495wXaBrwY+h/885nm9DvDbAOLZXhv4LZ6X+Pcx/7JbgY8Bfpr/eOZ5vQ7w2wDi2V4b+C2el/j3MS+63wY+Bvhr/uOY5/U6wG8DiGd7beC3eF7i38f863038DHALv9+5nm9DvDbAOLZXhv4LZ6X+Pcx/za7wFcDn8O/j3lerwP8NoB4ttcGfovnJf59zL/PrcDHAD/Nv415Xq8D/DaAeLbXBn6L5yX+fcy/3yXgrYHf5l/PPK/XAX4bQDzbawO/xfMS/z7m3+drgM8Gdvm3Mc/rdYDfBhDP9trAb/G8xL+P+bf5HeC9gVv59zHP63WA3wYQz/bawG/xvMS/j/nXeQbw3sBv8x/DPK/XAX4bQDzbawO/xfMS/z7mRXMJ+Gzgq/mPZZ7X6wC/DSCe7bWB3+J5iX8f8y/7GuCzgV3+45nn9TrAbwOIZ3tt4Ld4XuLfx7xgvwO8N3Ar/3nM83od4LcBxLO9NvBbPC/x72Oe1zOA9wZ+m/985nm9DvDbAOLZXhv4LZ6X+Pcxz3YJ+Gzgq/mvY57X6wC/DSCe7bWB3+J5iX8fc8XXAJ8N7PJfyzyv1wF+G0A822sDv8XzEv8+vw28N3Ar/z3M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5yX+dzPP63WA3wYQz/bawG/xvMT/buZ5vQ7w2wDi2V4b+C2el/jfzTyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3hev83/bq/N83od4LcBxLO9NPBX/P/wMsBfA4jndCvwIP5vewbwYK5APKfXBn6L/9teB/htrkA8r/cGvho4xv8tl4D3Bn6aZ0M8f8eB9wZeGngw/7vdCvw18N3ALs8J8f8b4v83xP9viP/fEP+/8Y9mceRBJaa3WQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdLiveTv;
impl IconShape for MdLiveTv {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 6h-7.59l3.29-3.29L16 2l-4 4-4-4-.71.71L10.59 6H3c-1.1 0-2 .89-2 2v12c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.11-.9-2-2-2zm0 14H3V8h18v12zM9 10v8l7-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+K/188Ab8V/nN8BXpvnhHj+fht4Lf77/A7w2sBHA1/Ff4zfAV6b54R4/n4beC3+e1wCHgzscsVPA2/Fv9/vAK/Nc0I8f78NvBb/PV4G+Gue7Tjw18CD+Pf5HeC1eU6I5++3gdfi3+dngLfiX+djgK/meb008Ff8+/wO8No8J8Tz99vAa/Fv9zvAawPvDXwXL5rvAd6bF+yjga/i3+53gNfmOSGev98GXot/m0vAg4Fdrvhq4KN44f4GeG1glxfup4G34t/md4DX5jkhnr/fBl6Lf5uXAf6a5/TTwFvx/F0CXhv4a/5lx4G/Bh7Ev97vAK/Nc0I8f78NvBb/eh8DfDXP6zjw28BL8bzeBvhpXnQvDfwV/3q/A7w2zwnx/P028Fr863wP8N68YA8G/ho4xrN9DfDR/Ot9NPBV/Ov8DvDaPCfE8/fbwGvxovsb4LWBXV64lwb+iit+B3ht/u1+GngrXnS/A7w2zwnx/P028Fq8aC4Brw38NS+a9wa+GngwsMu/3XHgr4EH8aL5HeC1eU6I5++3gdfiRfM2wE/zr3Mc2OXf76WBv+JF8zvAa/OcEM/fbwOvxb/sa4CP5r/XRwNfxb/sd4DX5jkhnr/fBl6LF+53gNfmf4afBt6KF+53gNfmOSGev98GXosX7BLwYGCX/xmOA38NPIgX7HeA1+Y5IZ6/3wZeixfsVuBW/md5MPBgXrDfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bPAT6b54R4/n4beC3+b3kb4Kd5Tojn77eB1+L/jr8BXprnhXj+fht4Lf5v+BvgrYFbeV6I5++3gdfif7dnAN8NfDWwy/OHeP5+G3gt/mXvA3w3/3shnr/fBl6LF+59gO/mfzfE8/fbwGvxgr0P8N3874d4/n4beC2ev/cBvpv/GxDP328Dr8Xzeh/gu/m/A/H8/TbwWjyn9wG+m/9bEM/fbwOvxbO9D/Dd/N+DeP5+G3gtrngf4Lv5vwnx/P028FrA+wDfzf9diOfvt4HvBr6b/9sQz99LA3/N/32I/98Q/78h/n9D/P+G+P+NfwQoN6hBAD8PgwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMms;
impl IconShape for MdMms {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM5 14l3.5-4.5 2.5 3.01L14.5 8l4.5 6H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD2klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b49/ss4K2Bl+Z/vl3gp4HPAW4FEP92x4HfAl6a/53eB/hu8W/3U8Bb87/bQ8S/zVsDP8X/ft8j/vWOA08HjvO/36741/sp4K35vwHxr/PWwE/xfwfiRXcceDpwnP87EC+6nwLemv9bEC+atwZ+iv97EP+y48DTgeP834P4l/0U8Nb834R44d4a+Cn+70K8YMeBpwPH+b8L8YL9FPDW/N+GeP7eGvgp/u9DPK/jwNOB4/zfh3hePwW8Nf8/IJ7TWwM/xf8fiGc7DjwdOM7/H4hn+yngrfmP8Qzgo4G/5oqXBr4aeBAv3OcAPw38NfDawGcDr8UL9zvAZwO/Dbw08NbAZ/GiQVzx1sBP8R/jGcBLA7s8p+PAXwMP4vl7HeC3eV7fDbwXz9/3AO/N83pt4Lf4lyHgOPB04Dj/Md4G+Gmev7cGforn9T3Ae/P8HQcu8vydAHZ5/r4beC9eOAT8FPDW/Mc5Aezy/D0YeDrP62OAr+YF+23gtXhOvwO8Ni/YRwNfxQuHgJ8G3or/OCeAXZ6/BwNP53l9DPDVvGC/DbwWz+l3gNfmBfto4Kt44RBwHLgVOMZ/jLcBfprn762Bn+J5fQ/w3jx/x4GLPH8ngF2ev+8G3osXDnHFWwM/xX+MW4GXAXZ5TseBvwIezPP3OsBv87y+C3hvnr/vBt6H5/XawG/xL0M8208Db8V/jFuBjwb+hiteCvhq4MG8cJ8N/Azw18BrAZ8NvDYv3G8Dnw38DvDSwFsBn82LBvFsx4FbgWP8/4F4Tm8N/BT/fyCe108Db8X/D4jndRy4FTjG/32I5++tgZ/i/z7EC/bTwFvxfxviBTsO3Aoc4/8uxAv31sBP8X8X4l/208Bb8X8T4l92HLgVOMb/PYgXzVsDP8X/PYgX3U8Db8X/LYgX3XHgVuAY/3cg/nXeGvgp/u9A/Ov9NPBW/N+A+Nc7DtwKHON/v0vi3+atgZ/if7/vEf923w28F/+7PUT82x0Hfhp4Lf53eh/gu8W/33sD7w28Fv/zXQJ+Gvhs4FYA8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EtERzekd2yr0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdMore;
impl IconShape for MdMore {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.97.89 1.66.89H22c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNetworkCheck;
impl IconShape for MdNetworkCheck {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15.9 5c-.17 0-.32.09-.41.23l-.07.15-5.18 11.65c-.16.29-.26.61-.26.96 0 1.11.9 2.01 2.01 2.01.96 0 1.77-.68 1.96-1.59l.01-.03L16.4 5.5c0-.28-.22-.5-.5-.5zM1 9l2 2c2.88-2.88 6.79-4.08 10.53-3.62l1.19-2.68C9.89 3.84 4.74 5.27 1 9zm20 2l2-2c-1.64-1.64-3.55-2.82-5.59-3.57l-.53 2.82c1.5.62 2.9 1.53 4.12 2.75zm-4 4l2-2c-.8-.8-1.7-1.42-2.66-1.89l-.55 2.92c.42.27.83.59 1.21.97zM5 13l2 2c1.13-1.13 2.56-1.79 4.03-2l1.28-2.88c-2.63-.08-5.3.87-7.31 2.88z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/puPALv8yxP893wV8D/Db/MsQ/7d8F/DewOsAv82/DPF/x3cB780VrwP8Nv8yxP8N3wW8N8/2OsBv8y9D/O/3XcB785xeB/ht/mWI/92+C3hvntfrAL/Nvwzxv9d3Ae/N8/c6wG/zL0P87/RdwHvzgr0O8Nv8yxD/+3wX8N68cK8D/Db/MsT/Lt8FvDf/stcBfpt/GeJ/j+8C3psXzesAv82/DPG/w3cB782L7nWA3+Zfhvif77uA9+Zf53WA3+Zfhvif7buA9+Zf73WA3+Zfhvif67uA9+bf5nWA3+Zfhvif6buA9+bf7nWA3+Zfhvif57uA9+bf53WA3+Zfhvif5buA9+bf73WA3+Zfhvif47uA9+Y/xusAv82/DPE/w3cB781/nNcBfpt/GeK/33cB781/rNcBfpt/GeK/13cB781/vNcBfpt/GeK/z3cB781/jtcBfpt/GeK/x3cB781/rN/h2T4a+Gv+ZYj/et8FvDf/fpeA7wZ+Gvht/m0Q/7W+C3hv/v2+B/hoYJd/H8R/ne8C3pt/v/cBvpv/GIj/Gt8FvDf/fq8D/Db/cRD/+b4LeG/+/d4H+G7+YyH+c30X8N78+/0O8No8f8eB9wLeG3hprvhr4LuB7wF2ecEQ/3m+C3hv/mO8DvDbPK/XBn4KOM7ztwu8DvDXPH+I/xzfBbw3/zH+BnhpntdLA78FHOfZfocrXotn2wVeB/hrnhfiP953Ae/Nf5yPAb6a5/V04MFc8T3ARwO7XHEc+Grgvbjir4GX4Xkh/mN9F/De/Md6HeC3eU5vDfwUV/wO8No8f78NvBZXvA3w0zwnxH+c7wLem/94DwFu5Tl9NvBZXPE6wG/z/L028Ftc8TnAZ/OcEP8xvgt4b/5ziOf108BbcYV44cwVPwO8Nc8J8e/3XcB785/jb4CX5nn9NvBaXCFeOHPF7wCvzXNC/Pt8F/De/Oe4BDwY2OV5/TbwWlwhXjhzxe8Ar81zQvzbfRfw3vzneR/gu3n+fht4La4QL5y54neA1+Y5If5tvgt4b/7zPAN4MM/fSwPfBbw0V7w2L9xvc8VfA+8D/DXPhvjX+y7gvfnP9TXAR/O8Phv4LP59Pgf4bK5A/Ot8F/De/Of7HOCzeU4vDfwV/zFeBvhrAPGi+y7gvfmv8TrAb/OcXhv4Lf5jvA7w2wDiRfNdwHvzX+d1gN/mOb028Fv8x3gd4LcBxL/su4D35r/W6wC/zXN6beC3+I/xOsBvA4gX7ruA9+a/3usAv81zem3gt/iP8TrAbwOIF+y7gPfmv8frAL/Nc3pt4Lf4j/E6wG8DiOfvu4D35r/P6wC/zXN6beC3+I/xOsBvA4jn9V3Ae/Pf63WA3+Y5vTbwW/zHeB3gtwHEc/ou4L357/c6wG/znF4b+C3+Y7wO8NsA4tmOAy/N/wx/DezynF4b+C3+Y7wO8NsA4n+P1wZ+i/8YrwP8NoD43+O1gd/iP8brAL8NIP73eG3gt/iP8TrAbwOI/13+Gngp/n3+BnhprkD873IceG/gtYHj/OvsAr8NfDewyxWI/98Q/78h/n9D/P+G+P+NfwTABbVBlwGSvQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNetworkLocked;
impl IconShape for MdNetworkLocked {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M19.5 10c.17 0 .33.03.5.05V1L1 20h13v-3c0-.89.39-1.68 1-2.23v-.27c0-2.48 2.02-4.5 4.5-4.5zm2.5 6v-1.5c0-1.38-1.12-2.5-2.5-2.5S17 13.12 17 14.5V16c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 0h-3v-1.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5V16z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdNoEncryption;
impl IconShape for MdNoEncryption {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 21.78L4.22 5 3 6.22l2.04 2.04C4.42 8.6 4 9.25 4 10v10c0 1.1.9 2 2 2h12c.23 0 .45-.05.66-.12L19.78 23 21 21.78zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H9.66L20 18.34V10c0-1.1-.9-2-2-2h-1V6c0-2.76-2.24-5-5-5-2.56 0-4.64 1.93-4.94 4.4L8.9 7.24V6z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE5ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v8+xwHdvnvY57X6wC/DSCe7bWB3+J5iX+fi8BXA5/Dfw/zvF4H+G0A8WyvDfwWz0v8+5grbgU+Bvhp/muZ5/U6wG8DiGd7beC3eF7i38c8p98GPgb4a/5rmOf1OsBvA4hne23gt3he4t/HPH/fDXwMsMt/LvO8Xgf4bQDxbK8N/BbPS/z7mBdsF/hq4HP4z2Oe1+sAvw0gnu21gd/ieYl/H/MvuxX4GOCn+Y9nntfrAL8NIJ7ttYHf4nmJfx/zovtt4GOAv+Y/jnlerwP8NoB4ttcGfovnJf59zL/edwMfA+zy72ee1+sAvw0gnu21gd/ieYl/H/Nvswt8NfA5/PuY5/U6wG8DiGd7beC3eF7i38f8+9wKfAzw0/zbmOf1OsBvA4hne23gt3he4t/H/PtdAt4a+G3+9czzeh3gtwHEs7028Fs8L/HvY/59vgb4bGCXfxvzvF4H+G0A8WyvDfwWz0v8+5h/m98B3hu4lX8f87xeB/htAPFsrw38Fs9L/PuYf51nAO8N/Db/Mczzeh3gtwHEs7028Fs8L/HvY140l4DPBr6a/1jmeb0O8NsA4tleG/gtnpf49zH/sq8BPhvY5T+eeV6vA/w2gHi21wZ+i+cl/n3MC/Y7wHsDt/Kfxzyv1wF+G0A822sDv8XzEv8+5nk9A3hv4Lf5z2ee1+sAvw0gnu21gd/ieYl/H/Nsl4DPBr6a/zrmeb0O8NsA4tleG/gtnpf49zFXfA3w2cAu/7XM83od4LcBxLO9NvBbPC/x7/PbwHsDt/Lfwzyv1wF+G0A822sDv8XzEv+7mef1OsBvA4hne23gt3he4n8387xeB/htAPFsrw38Fs9L/O9mntfrAL8NIJ7ttYHf4nmJ/93M83od4LcBxLO9NvBbPC/xv5t5Xq8D/DaAeLbXBn6L5/Xb/O/22jyv1wF+G0A820sDf8X/Dy8D/DWAeE63Ag/i/7ZnAA/mCsRzem3gt/i/7XWA3+YKxPN6b+CrgWP833IJeG/gp3k2xPN3HHhv4KWBB/Ov99LAMf5jXQL+mn+9W4G/Br4b2OU5If5z/DbwWvzH+h3gtfmPhfjP8dvAa/Ef63eA1+Y/FuI/x28Dr8V/rN8BXpv/WIj/HL8NvBb/sX4HeG3+YyH+c/w28Fr8x/od4LX5j4X4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKgmMVBduhD6AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdOndemandVideo;
impl IconShape for MdOndemandVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.11 0-2 .89-2 2v12c0 1.1.89 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.11-.9-2-2-2zm0 14H3V5h18v12zm-5-6l-7 4V7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/t4L+Gjgpfm/4a+Brwa+h+eEeF7fBbw3/zd9N/A+PBviOX028Fn83/Y5wGdzBeI5XQSO83/bLnCCKxDP9trAb/H/w+sAvw0gnu21gd/ieb0O/7v9Fs/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovnJf53M8/rdYDfBhDP9trAb/G8xP9u5nm9DvDbAOLZXhv4LZ6X+N/NPK/XAX4bQDzbawO/xfMS/7uZ5/U6wG8DiGd7beC3eF7ifzfzvF4H+G0A8WyvDfwWz0v872ae1+sAvw0gnu21gd/ieYn/3czzeh3gtwHEs7028Fs8L/G/m3lerwP8NoB4ttcGfovn9dv87/baPK/XAX4bQDzbSwN/xf8PLwP8NYB4TrcCD+L/tmcAD+YKxHN6beC3+L/tdYDf5grE83pv4KuBY/zfcgl4b+CneTbE83cceG/gpYEH86/30sAx/mNdAv6af71bgb8GvhvY5Tkh/nP8NvBa/Mf6HeC1+Y+F+M/x28Br8R/rd4DX5j8W4j/HbwOvxX+s3wFem/9YiP8cvw28Fv+xfgd4bf5jIf5z/DbwWvzH+h3gtfmPhfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AgiPh0E3WHlAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPersonalVideo;
impl IconShape for MdPersonalVideo {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21 3H3c-1.11 0-2 .89-2 2v12c0 1.1.89 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.11-.9-2-2-2zm0 14H3V5h18v12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/p5cGjgG/w78P4n+n3wZeC/hu4H34t0P87/TbwGtxxXcD78O/DeJ/p98GXotn+27gffjXQ/zP9mDgVp7XbwOvxXP6buB9+NdB/M/1XcBbA68D/DXP6beB1+J5fTfwPrzoEP+y48BXAe/Ni+ZW4LuBz+Hf7r2B7+KKXeB1gL/m2X4beC2ueB/gu3i27wbehxcN4l/23cB78a/3NsBP82/33cB7ccUu8DrAX3PFbwOvxRUC3hv4Lp7tu4H34V+G+JeZf5vvAd6bf5/vBt6LK3aB1wH+Gvht4LW4QlzxW8Brc8Ul4KWBW3nhEP8y82/zO8Br8+/33cB7ccUu8DrAVwOvxRUCvgt4b664BLw28Nf8yxD/MvNv8zvAa/Mf47uB9+KKXWAXeDBXfDfw3lxxCXht4K950SD+ZX8NvBT/er8DvDb/cb4beC9esEvAawN/zYsO8S/7beC1+LcR/7G+G3gvntcl4LWBv+ZfB/Ev+23gtfi3Ef/xvht4L57tEvDawF/zr4f4l3018FH827wO8Nv8x/ou4L15tl3gdYC/5l8P8S/7bOCz+Lf5GOCr+Y/zXcB787x2gdcB/pp/HcS/7LWB3+Lf5meAt+Y/xncB780Ltgu8DvDXvOgQ/7IHA0/n3078+30X8N5ccQnYBR7EFd8DvBdX7AKvA/w1LxrEi+ZW4EH827wO8Nv8230X8N5ccQl4beCrgdfiCgHfDbwXV+wCrwP8Nf8yxIvmu4H34t/mdYDf5t/mu4D35opLwGsDfw38NvBaXCGu+G3gtbhiF3gIsMsLh3jRvDfwXfzr/Q3w0vzbfBfw3lxxCXht4K+54reB1+IKAe8NfBfP9j7Ad/MvQ7xoHgw8nX+dvwFeG9jlX++9ge/iikvAawN/zbP9NvBaXPE+wHfxbO8DfDcvGsSL7lbgQbxo/gZ4bWCXf7vvBt4aeG3gr3lOvw28Fs/rfYDv5kWHeNF9NPBV/Mv+BnhtYJd/vwcDt/K8fht4LZ7T+wDfzb8O4kX3YODpvHB/A7w2sMt/rt8GXotnex/gu/nXQ/zr/DbwWjx/fwO8NrDLf77fBl6LK94H+G7+bRD/Ou8NfBfP6xnASwO7/Nf4beC1gPcBvpt/O8S/3q3Ag3hOu8DLALfyX+OlueKv+fdB/Ou9N/BdPK/vAd6b/10Q/za3Ag/ieb0M8Nf874H4t3lv4Lt4Xn8NvAz/eyD+7X4beC2e1+cAn83/Doh/u5cG/orn73WA3+Z/PsS/z1cDH8Xz2gUeAuzyPxvi3+c48NvAS/G8/hp4HWCX/7kQ/34vDfw2cIzn9dPA2/A/F+I/xnsD38Xz993A+/A/E+I/zncD78Xz993A+/Dv81pc8Tv8x0H8x/pt4LV4/n4aeB9gl3+d48BvAS/Ns/008NfAbwO/w78d4j/WceC3gZfi+ftr4G2AW3nRHAd+C3hp/mW/zbPdCvwM8NO8cIj/eMeB3wZeiudvF3gb4Ld54Y4DvwW8NP92nwN8Ni8Y4j/HceC3gZfiBftq4HOAXZ7XceC3gJfm3+dW4CG8YIj/PMeBnwZeixfsVuB9gN/m2Y4DvwW8NP8xxAuG+M/33cB78cJ9N/A5wC7wW8BL8x9HvGCI/xrvDXw1cIwX7lbgwfzHEi8Y4r/OSwPfDbwU/7XEC4b4r3Uc+Gzgo/ivI14wxH+Plwa+Gngt/vOJFwzx3+u9gc8GHsR/HvGCIf5neG/gs4EH8R9PvGCI/1neG3hr4K34jyNeMMT/TA8G3hr4aOBB/PuIFwzxP9+DgdcGXht4beBBvOi+B3hvXjDE/z4PBh4MvDTwYOClueI48FI82/cAHw3s8oIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y8j4+JBKcSrqQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneBluetoothSpeaker;
impl IconShape for MdPhoneBluetoothSpeaker {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.71 9.5L17 7.21V11h.5l2.85-2.85L18.21 6l2.15-2.15L17.5 1H17v3.79L14.71 2.5l-.71.71L16.79 6 14 8.79l.71.71zM18 2.91l.94.94-.94.94V2.91zm0 4.3l.94.94-.94.94V7.21zm2 8.29c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFfUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO//W+C/gd4Lt50SD+Zd8NvBf/em8D/DT/db4LeG+ueB/gu/mXIf5l5t/me4D35r/GdwHvzRWXgNcG/pp/GeJfZv5tfgd4bf5zHQe+C3hrrrgEvDbw17xoEP8y82/zO8Br85/nOPBbwEtzxSXgtYG/5kWH+Jf9NfBS/Ov9DvDa/Mt+iyv+BvhoXjTHgd8CXporLgGvDfw1/zqIf9lvA6/Fv434l5krfgd4bf5lx4HfAl6aKy4Brw38Nf96iH/ZbwOvxb+N+JeZK34HeG1euOPAbwEvzRWXgNcG/pp/G8S/7KuBj+Lf5nWA3+aFM1f8DvDavGDHgd8CXporLgGvDfw1/3aIf9lnA5/Fv83HAF/NC2eu+B3gtXn+jgO/Bbw0V1wCXhv4a/59EP+y1wZ+i3+bnwHemhfOXPE7wGvzvI4DvwW8NFdcAl4b+Gv+/RD/sgcDT+ffTrxw5orfAV6b53Qc+C3gpbniEvDawF/zHwPxorkVeBD/Nq8D/DYvmLnid4DX5tmOA78FvDRXXAJeG/hr/uMgXjTfDbwX/zavA/w2L5i54neA1+aK48BvAS/NFZeA1wb+mv9YiBfNewPfxb/e3wAvzQtnrvgd4LWBlwZ+CzjOFZeA1wb+mv94iBfNg4Gn86/zN8BrA7u8cOaK3wE+Gvgt4DhXXAJeG/hr/nMgXnS3Ag/iRfM3wGsDu/zLzBW3AseB41xxCXht4K/5z4N40X008FX8y/4GeG1glxeNeV6XgNcG/pr/XIgX3YOBp/PC/Q3w2sAuLzrzvD4a+GteNL/Dvx3iX+e3gdfi+fsb4LWBXf51zL+P+LdD/Ou8N/BdPK9nAC8N7PKvZ/59xL8d4l/vVuBBPKdd4GWAW/nXe23+fX6bfzvEv957A9/F8/oe4L353wXxb3Mr8CCe18sAf83/Hoh/m/cGvovn9dfAy/C/B+Lf7reB1+J5fQ7w2fzvgPi3e2ngr3j+Xgf4bf7nQ/z7fDXwUTyvXeAhwC7/syH+fY4Dvw28FM/rr4HXAXb5nwvx7/fSwG8Dx3he3w28D/9zIf5jvDfwXTx/3w28D/8zIf7jfDfwXjx/3w28D/8+r8UVv8N/HMR/rN8GXovn76eB9wF2+dc5DvwW8NJcsQv8NvDbwF8Dv8O/HeI/1nHgt4GX4vn7a+B1gF1eNMeB3wJemhduF/hrnu1W4GeAn+aFQ/zHOw78NvBSPH+7wNsAv80Ldxz4LeCl+bf7GOCrecEQ/zmOA78NvBQv2GcDn8Pzdxz4LeCl+fe5FXgILxjiP89x4KeB1+IF+2vgfYC/5tmOA78FvDT/McQLhvjP993Ae/HCfTfwOcAu8FvAS/MfR7xgiP8a7w18NXCMF2wX2AUezH8s8YIh/uu8NPDdwEvxX0u8YIj/WseBzwY+iv864gVD/Pd4aeCrgdfiP594wRD/vd4b+GzgQfznES8Y4n+G9wY+G3gQ//HEC4b4n+W9gbcG3or/OOIFQ/zP9GDgrYGPBh7Ev494wRD/8z0YeG3gtYHXBh7Ei+57gPfmBUP87/Ng4MHASwMPBl6aK44DL8WzfQ/w0cAuLxji/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/whtjslBMlUKugAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneCallback;
impl IconShape for MdPhoneCallback {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2zm13.54-7.1l-.71-.7L13 9.29V5h-1v6h6v-1h-4.15z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/v+PALv82iP/9/gp4HWCXfz3E/34G/hp4HWCXfx3E/37mir8GXgfY5UWH+N/PPNtfA68D7PKiQfzvZ57TXwOvA+zyL0P8y44DXwW8Ny+aW4HvBj6H/xrmef018DrALi8c4l/23cB78a/3NsBP85/PPH9/DbwOsMsLhviXmX+b7wHem3+blwaO8aL5bV6wvwZeB9jl+UP8y8y/ze8Ar82/zW8Dr8V/jL8GXgfY5Xkh/mXm3+Z3gNfm3+a3gdfiP85fA68D7PKcEP+yvwZein+93wFem3+b3wZei/9Yfw28DrDLsyH+Zb8NvBb/NuLf5reB1+I/3l8DrwPscgXiX/bbwGvxbyP+bX4beC3+c/w18DrALoD4l3018FH827wO8Nv86/028Fr85/gb4LWBXQDxL/ts4LP4t/kY4Kv51/tt4LX4j/c3wGsDu1yB+Je9NvBb/Nv8DPDW/Ov9NvBa/Mf6G+C1gV2eDfEvezDwdP7txL/ebwOvxX+cvwFeG9jlOSFeNLcCD+Lf5nWA3+Zf57eB1+I/xt8Arw3s8rwQL5rvBt6Lf5vXAX6bf52XBo7zovktXrC/AV4b2OX5Q7xo3hv4Lv71/gZ4af5zmefvb4DXBnZ5wRAvmgcDT+df52+A1wZ2+c9lntffAK8N7PLCIV50twIP4kXzN8BrA7v85zPP6W+A1wZ2+ZchXnQfDXwV/7K/AV4b2OW/hnm2vwFeG9jlRYN40T0YeDov3N8Arw3s8l/HXPE3wGsDu7zoEP86vw28Fs/f3wCvDezyX8vA3wCvDezyr4P413lv4Lt4Xs8AXhrY5b/eXwOvDezyr4f417sVeBDPaRd4GeBW/usdB3b5t0H867038F08r+8B3pv/XRD/NrcCD+J5vQzw1/zvgfi3eW/gu3hefw28DP97IP7tfht4LZ7X5wCfzf8OiH+7lwb+iufvdYDf5n8+xL/PVwMfxfPaBR4C7PI/G+Lf5zjw28BL8bz+GngdYJf/uRD/fi8N/DZwjOf108Db8D8X4j/GewPfxfP33cD78D8T4j/OdwPvxfP33cD78O/zWlzxO/zHQfzH+m3gtXj+fhp4H2CXf53jwG8BL82z/TTw18BvA7/Dvx3iP9Zx4LeBl+L5+2vgbYBbedEcB34LeGn+Zb/Ns90K/Azw07xwiP94x4HfBl6K528XeBvgt3nhjgO/Bbw0/3afA3w2LxjiP8dx4LeBl+IF+2rgc4Bdntdx4LeAl+bf51bgIbxgiP88x4GfBl6LF+xW4H2A3+bZjgO/Bbw0/zHEC4b4z/fdwHvxwn038DnALvBbwEvzH0e8YIj/Gu8NfDVwjBfuVuDB/McSLxjiv85LA98NvBT/tcQLhvivdRz4bOCj+K8jXjDEf4+XBr4aeC3+84kXDPHf672BzwYexH8e8YIh/md4b+CzgQfxH0+8YIj/Wd4beGvgrfiPI14wxP9MDwbeGvho4EH8+4gXDPE/34OB1wZeG3ht4EG86L4HeG9eMMT/Pg8GHgy8NPBg4KW54jjwUjzb9wAfDezygiH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xjz1MxEGqzf2EAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneForwarded;
impl IconShape for MdPhoneForwarded {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 11l5-5-5-5v3h-4v4h4v3zm2 4.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneInTalk;
impl IconShape for MdPhoneInTalk {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 12h2c0-4.97-4.03-9-9-9v2c3.87 0 7 3.13 7 7zm-4 0h2c0-2.76-2.24-5-5-5v2c1.66 0 3 1.34 3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFXklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+J/p9fi2f4G2OXfBvG/x3Hgo4CPBo7znL4b+BzgVv51EP87HAd+C3hpnu1vgJfi2XaBtwF+mxcd4n+HvwJeGrgEfDbw1TzbawNfDbwUsAu8DHArLxrE/3zvDXwXV7wM8Nc8r+PAbwMvBXwP8N68aBD/8/008FbA1wAfzQv22sBvccUJYJd/GeJfdhz4KuC9edHcCnw38Dn8xzBXvA7w27xw5orXAX6bfxniX/bdwHvxr/c2wE/z72eueB3gt3nhfht4LeB1gN/mX4b4l5l/m+8B3pt/n9cCfpsrPhr4a164rwZeGvho4HuAXV44xL/M/Nv8DvDa/NscB34LeGn+7XaB1wH+mhcM8S8z/za/A7w2/zYfDXwV/35/DbwMLxjiX/bXwEvxr/c7wGvzb/PbwGvxH0O8YIh/2W8Dr8W/jfi3+W3gtfiPIV4wxL/st4HX4t9G/Nv8NvBa/McQLxjiX/bVwEfxb/M6wG/zr/fbwGvxH0O8YIh/2WcDn8W/zccAX82/3m8Dr8V/DPGCIf5lrw38Fv82PwO8Nf96vw28Fv8xxAuG+Jc9GHg6/3biX++3gdfiP4Z4wRAvmluBB/Fv8zrAb/Ov89vAa/EfQ7xgiBfNdwPvxb/N6wC/zb/ObwOvxX8M8YIhXjTvDXwX/3p/A7w0/3q/DbwW/zHEC4Z40TwYeDr/On8DvDawy7/ebwOvxX8M8YIhXnS3Ag/iRfM3wGsDu/zbfDXwUfz7PQN4MC8Y4kX30cBX8S/7G+C1gV3+7V4a+Cv+/T4H+GxeMMSL7sHA03nh/gZ4bWCXf7+XBt4beGn+9W4Ffhr4aV44xL/ObwOvxfP3N8BrA7v874H413lv4Lt4Xs8AXhrY5X8XxL/ercCDeE67wMsAt/K/C+Jf772B7+J5fQ/w3vzvgvi3uRV4EM/rZYC/5n8PxL/NewPfxfP6a+Bl+N8D8W/328Br8bw+B/hs/ndA/Nu9NPBXPH+vA/w2//Mh/n2+Gvgontcu8BBgl//ZEP8+x4HfBl6K5/XXwOsAu/zPhfj3e2ngt4FjPK+fBt6G/7kQ/zHeG/gunr/vBt6H/5kQ/3G+G3gvnr/vBt6Hf5/X4orf4T8O4j/WbwOvxfP308D7ALv86xwHfgt4aZ7tp4G/Bn4b+B3+7RD/sY4Dvw28FM/fXwNvA9zKi+Y48FvAS/Mv+22e7VbgZ4Cf5oVD/Mc7Dvw28FI8f7vA2wC/zQt3HPgt4KX5t/sc4LN5wRD/OY4Dvw28FC/YVwOfA+zyvI4DvwW8NP8+twIP4QVD/Oc5Dvw08Fq8YLcC7wP8Ns92HPgt4KX5jyFeMMR/vu8G3osX7ruBzwF2gd8CXpr/OOIFQ/zXeG/gq4FjvHC3Ag/mP5Z4wRD/dV4a+G7gpfivJV4wxH+t48BnAx/Ffx3xgiH+e7w08NXAa/GfT7xgiP9e7w18NvAg/vOIFwzxP8N7A58NPIj/eOIFQ/zP8t7AWwNvxX8c8YIh/md6MPDWwEcDD+LfR7xgiP/5Hgy8NvDawGsDD+JF9z3Ae/OCIf73eTDwYOClgQcDL80Vx4GX4tm+B/hoYJcXDPH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BEdtsUGYAQoxAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneLocked;
impl IconShape for MdPhoneLocked {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM20 4v-.5C20 2.12 18.88 1 17.5 1S15 2.12 15 3.5V4c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zm-.8 0h-3.4v-.5c0-.94.76-1.7 1.7-1.7s1.7.76 1.7 1.7V4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhoneMissed;
impl IconShape for MdPhoneMissed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6.5 5.5L12 11l7-7-1-1-6 6-4.5-4.5H11V3H5v6h1.5V5.5zm17.21 11.17C20.66 13.78 16.54 12 12 12 7.46 12 3.34 13.78.29 16.67c-.18.18-.29.43-.29.71s.11.53.29.71l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.11.7-.28.79-.74 1.69-1.36 2.66-1.85.33-.16.56-.5.56-.9v-3.1c1.45-.48 3-.73 4.6-.73 1.6 0 3.15.25 4.6.72v3.1c0 .39.23.74.56.9.98.49 1.87 1.12 2.67 1.85.18.18.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71s-.12-.52-.3-.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4lx0Hvgp4b140twLfDXwO/zHM83od4Ld5Tq8N/BbPS7xgiH/ZdwPvxb/e2wA/zb+feV6vA/w2z+m1gd/ieYkXDPEvM/823wO8N/9+5nm9DvDbPKfXBn6L5yVeMMS/zPzb/A7w2vz7mef1OsBv85xeG/gtnpd4wRD/MvNv8zvAa/PvZ57X6wC/zXN6beC3eF7iBUP8y/4aeCn+9X4HeG3+/czzeh3gt3lOrw38Fs9LvGCIf9lvA6/Fv4349zPP63WA3+Y5vTbwWzwv8YIh/mW/DbwW/zbi3888r9cBfpvn9NrAb/G8xAuG+Jd9NfBR/Nu8DvDb/PuY5/U6wG/znF4b+C2el3jBEP+yzwY+i3+bjwG+mn8f87xeB/htntNrA7/F8xIvGOJf9trAb/Fv8zPAW/PvY57X6wC/zXN6beC3eF7iBUP8yx4MPJ1/O/HvY57X6wC/zXN6beC3eF7iBUO8aG4FHsS/zesAv82/nXlerwP8Ns/ptYHf4nmJFwzxovlu4L34t3kd4Lf5tzPP63WA3+Y5vTbwWzwv8YIhXjTvDXwX/3p/A7w0/z7meb0O8Ns8p9cGfovnJV4wxIvmwcDT+df5G+C1gV3+fczzeh3gt3lOrw38Fs9LvGCIF92twIN40fwN8NrALv9+5nm9DvDbPKfXBn6L5yVeMMSL7qOBr+Jf9jfAawO7/Mcwz+t1gN/mOb028Fs8L/GCIV50Dwaezgv3N8BrA7v8x/ltntdHA3/Nc3pp4Kt5Xq/NC4b41/lt4LV4/v4GeG1gl/89EP867w18F8/rGcBLA7v874L417sVeBDPaRd4GeBW/ndB/Ou9N/BdPK/vAd6b/10Q/za3Ag/ieb0M8Nf874H4t3lv4Lt4Xn8NvAz/eyD+7X4beC2e1+cAn83/Doh/u5cG/orn73WA3+Z/PsS/z1cDH8Xz2gUeAuzyPxvi3+c48NvAS/G8/hp4HWCX/7kQ/34vDfw2cIzn9dPA2/A/F+I/xnsD38Xz993A+/A/E+I/zncD78Xz993A+/Dv81pc8Tv8x0H8x/pt4LV4/n4aeB9gl3+d48BvAS/Ns/008NfAbwO/w78d4j/WceC3gZfi+ftr4G2AW3nRHAd+C3hp/mW/zbPdCvwM8NO8cIj/eMeB3wZeiudvF3gb4Ld54Y4DvwW8NP92nwN8Ni8Y4j/HceC3gZfiBftq4HOAXZ7XceC3gJfm3+dW4CG8YIj/PMeBnwZeixfsVuB9gN/m2Y4DvwW8NP8xxAuG+M/33cB78cJ9N/A5wC7wW8BL8x9HvGCI/xrvDXw1cIwX7lbgwfzHEi8Y4r/OSwPfDbwU/7XEC4b4r3Uc+Gzgo/ivI14wxH+Plwa+Gngt/vOJFwzx3+u9gc8GHsR/HvGCIf5neG/gs4EH8R9PvGCI/1neG3hr4K34jyNeMMT/TA8G3hr4aOBB/PuIFwzxP9+DgdcGXht4beBBvOi+B3hvXjDE/z4PBh4MvDTwYOClueI48FI82/cAHw3s8oIh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/zp69BXSmh/wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPhonePaused;
impl IconShape for MdPhonePaused {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 3h-2v7h2V3zm3 12.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 3v7h2V3h-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD5ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP8cvw28Fs/pd4DX5kXz28Br8Zx+B3ht/mMh/nP8NvBaPKffAV6bF81vA6/Fc/od4LX5j4X4z/HbwGvxnH4HeG1eNL8NvBbP6XeA1+Y/FuI/x28Dr8Vz+h3gtXnR/DbwWjyn3wFem/9YiP94rwV8NfDSPKe/Bj6aF81XAy/Nc/pr4KOB3+E/DuI/zlsD3wUc5z/XLvA+wE/z74f4j/HZwGfxX+tjgK/m3wfx7/fawG/x3+N1gN/m3w7x7/fTwFvx3+NngLfm3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PuZ/17i3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PuZ/17i3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PuZ/17i3w7x72f+e4l/O8S/n/nvJf7tEP9+5r+X+LdD/PvtAsf473EJOM6/HeLf76WB3waO8V/rEvDawF/zb4f4j/HSwG8Dx/ivcQl4beCv+fdB/Md5aeC3gWP857oEvDbw1/z7If5jvTTw28Ax/nNcAl4b+Gv+YyD+47008NvAMf5jXQJeG/hr/uMg/nO8NPDbwDH+Y1wCXhv4a/5jIf7zvDTw28Ax/n0uAa8N/DX/8RD/uV4a+G3gGP82l4DXBv6a/xyI/3wvDfw2cIx/nUvAawN/zX8exH+N3wZei3+d3wFem/9ciP8avw28Fv86vwO8Nv+5EP81fht4Lf51fgd4bf5zIf5r/DbwWvzr/A7w2vznQvzX+G3gtfjX+R3gtfnPhfiv8dvAa/Gv8zvAa/OfC/Ff47eB1+Jf53eA1+Y/F+K/xm8Dr8W/zu8Ar81/LsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EUykckFmTWA9AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPower;
impl IconShape for MdPower {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M16.01 7L16 3h-2v4h-4V3H8v4h-.01C7 6.99 6 7.99 6 8.99v5.49L9.5 18v3h5v-3l3.5-3.51v-5.5c0-1-1-2-1.99-1.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPowerOff;
impl IconShape for MdPowerOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 14.49V9c0-1-1.01-2.01-2-2V3h-2v4h-4V3H8v2.48l9.51 9.5.49-.49zm-1.76 1.77L7.2 7.2l-.01.01L3.98 4 2.71 5.25l3.36 3.36C6.04 8.74 6 8.87 6 9v5.48L9.5 18v3h5v-3l.48-.48L19.45 22l1.26-1.28-4.47-4.46z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4r/PawG/xonkd4Lf5z4f4/w3x/xvi/zfE/2+I/98Q/z2OA+8FvDRX/DXwPcAu/7UQ//XeG/gq4DjPaRf4GOC7+a+D+K/11sBP8cK9DvDb/NdA/Nd6OvBgXrhbgYfwXwPxX+elgb/iRfMywF/znw/xX+e1gd/iRfM6wG/znw/xX+e1gd/iRfM6wG/znw/xX2sXOMYLdwk4zn8NxH+tzwY+ixfuc4DP5r8G4r/edwPvxfP3PcB7818H8d/jvYGPBl6KK34H+G7gu/mvhfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AkswWUFo+I6dAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdPriorityHigh;
impl IconShape for MdPriorityHigh {
    fn view_box(&self) -> &str {
        "0 0 24 24"
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
                cy: "19",
                r: "2",
            }
            path {
                d: "M10 3h4v12h-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFCElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/db4KeGn+490K/DXwPcAu/zqI/xqfDXwW/7l2gY8BvpsXHeI/33sD38V/ndcBfpsXDeI/13sD38V/rVuBh/CiQfzneW/gu/jv8TLAX/MvQ/zneG/gu3hOl4C/5nm9FldcAv6a5/VaXHEJ+Gue12vxvF4H+G3+ZYj/eO8NfBfP63eA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Md6b+C7eP5+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsR/nPcGvosX7HeA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Md4b+C7eOF+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsS/33sD38W/7HeA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Pu8N/BdvGh+B3htnpe54neA1+Z5mSt+B3htnpd5Xq8D/Db/MsS/3XsD38WL7neA1+Z5mSt+B3htnpe54neA1+Z5mef1OsBv8y9D/Nu8N/Bd/Ov8DvDaPC9zxe8Ar83zMlf8DvDaPC/zvF4H+G3+ZYh/vfcGvot/vd8BXpvnZa74HeC1eV7mit8BXpvnZZ7X6wC/zb8M8a/z3sB38W/zO8Br87zMFb8DvDbPy1zxO8Br87zM83od4Lf5lyFedC8N/BX/drvAX/O8XpsrdoG/5nm9NlfsAn/N83ptntfrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+61wZ+i/8dXgf4bf5liBfdawO/xf8OrwP8Nv8yxIvutYHf4n+H1wF+m38Z4kX32sBv8b/D6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/F/w6vA/w2/zLEi+61gd/if4fXAX6bfxniRffawG/xv8PrAL/Nvwzxontt4Lf43+F1gN/mX4Z40b028Fv87/A6wG/zL0O86F4b+C3+d3gd4Lf5lyFedK8N/Bb/O7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie2ngr/jf4WWAv+ZfhvjXuRV4EP+zPQN4MC8axL/OawO/xf9srwP8Ni8axL/eewNfDRzjf5ZLwHsDP82LDvFvcxx4b+ClgQfz3+tW4K+B7wZ2+ddB/P+G+P8N8f8b4v83xP9v/COCA7pB1W+fYQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSdCard;
impl IconShape for MdSdCard {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE3klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/db4KeGn+490K/DXwPcAu/zqI/xqfDXwW/7l2gY8BvpsXHeI/33sD38V/ndcBfpsXDeI/13sD38V/rVuBh/CiQfzneW/gu/jv8TLAX/MvQ/zneG/gu/jv8zrAb/MvQ/zHe2/gu/jv9TrAb/MvQ/zHem/gu/jv9zrAb/MvQ/zHeW/gu/if4XWA3+ZfhviP8d7Ad/E/x+sAv82/DPHv997Ad/E/y+sAv82/DPHv897Ad/E/z+sAv82/DPFv997Ad/E/0+sAv82/DPFv897Ad/E/1+sAv82/DPGv997Ad/E/2+sAv82/DPGv897Ad/E/3+sAv82/DPGie2ngr/iP8THAX/OcXhr4Kv5jvA7w2/zLEC+61wZ+i/8YrwP8Ns/ptYHf4j/G6wC/zb8M8aJ7beC3+I/xOsBv85xeG/gt/mO8DvDb/MsQL7rXBn6L/xivA/w2z+m1gd/iP8brAL/Nvwzxontt4Lf4j/E6wG/znF4b+C3+Y7wO8Nv8yxAvutcGfov/GK8D/DbP6bWB3+I/xusAv82/DPGie23gt/iP8TrAb/OcXhv4Lf5jvA7w2/zLEC+61wZ+i/8YrwP8Ns/ptYHf4j/G6wC/zb8M8aJ7beC3+I/xOsBv85xeG/gt/mO8DvDb/MsQL7rXBn6L/xivA/w2z+m1gd/iP8brAL/Nvwzxontt4Lf4j/E6wG/znF4b+C3+Y7wO8Nv8yxAvutcGfov/GK8D/DbP6bWB3+I/xusAv82/DPGie23gt/iP8TrAb/OcXhv4Lf5jvA7w2/zLEC+61wZ+i/8YHw38Nc/ppYGv5j/G6wC/zb8M8aJ7beC3+N/hdYDf5l+GeNG9NvBb/O/wOsBv8y9DvOheG/gt/nd4HeC3+ZchXnSvDfwW/zu8DvDb/MsQL7rXBn6L/x1eB/ht/mWIF91rA7/Ff4zXAX6b5/TawG/xH+N1gN/mX4Z40b028Fv8x3gd4Ld5Tq8N/Bb/MV4H+G3+ZYgX3WsDv8V/jNcBfpvn9NrAb/Ef43WA3+ZfhnjRvTbwW/zHeB3gt3lOrw38Fv8xXgf4bf5liBfdawO/xX+M1wF+m+f02sBv8R/jdYDf5l+GeNG9NvBb/Mf4a2CX53QceGn+Y7wO8Nv8yxAvutcGfov/HV4H+G3+ZYgX3WsDv8X/Dq8D/Db/MsSL7rWB3+J/h9cBfpt/GeJF99rAb/G/w+sAv82/DPGie23gt/jf4XWA3+ZfhnjRvTbwW/zv8DrAb/MvQ7zoXhv4Lf53eB3gt/mXIV50rw38Fv87vA7w2/zLEC+6lwb+iv8dXgb4a/5liH+dW4EH8T/bM4AH86JB/Ou8NvBb/M/2OsBv86JB/Ou9N/DVwDH+Z7kEvDfw07zoEP82x4H3Bl4aeDD/vW4F/hr4bmCXfx3E/2+I/98Q/78h/n9D/P/GPwI0wppB6ewbXwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSimCardAlert;
impl IconShape for MdSimCardAlert {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 2h-8L4.02 8 4 20c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5 15h-2v-2h2v2zm0-4h-2V8h2v5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf5v+R3gtXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/nV+B3htnpd5Xq8D/DbP6bWB3+J5ief128Br8a/zO8Br85wQz99vA6/Fv87vAK/N8zLP63WA3+Y5vTbwWzwv8bx+G3gt/nV+B3htnhPi+ftt4LX41/kd4LV5XuZ5vQ7w2zyn1wZ+i+clntdvA6/Fv87vAK/Nc0I8f78NvBb/Or8DvDbPyzyv1wF+m+f02sBv8bzE8/pt4LX41/kd4LV5Tojn77eB1+Jf53eA1+Z5mef1OsBv85xeG/gtnpd4Xr8NvBb/Or8DvDbPCfH8/TbwWvzr7AJ/zfN6bZ7XXwO7PKfjwEvzvH6b5/XSwHH+dX4HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XeA1+Y5IZ6/3wZei/9bPgf4bJ4T4vn7beC1+L/lbYCf5jkhnr/fBl6L/zv+Bnhpnhfi+ftt4LX4v+FvgLcGbuV5IZ6/3wZei//dngF8N/DVwC7PH+L5+23gtfiXvQ/w3fzvhXj+fht4LV649wG+m//dEM/fbwOvxQv2PsB3878f4vn7beC1eP7eB/hu/m9APH+/DbwWz+t9gO/m/w7E8/fbwGvxnN4H+G7+b0E8f78NvBbP9j7Ad/N/D+L5+23gtbjifYDv5v8mxPP328BrAe8DfDf/dyGev98Gvhv4bv5vQzx/Lw38Nf/3If5/Q/z/hvj/DfH/G+L/N/4RCV+OQf6lHVAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSms;
impl IconShape for MdSms {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4Lf7zvA7w2zyn1wZ+i/88vwO8Ns8J8fz9NvBa/Od5HeC3eU6vDfwW/3l+B3htnhPi+ftt4LX4z/M6wG/znF4b+C3+8/wO8No8J8Tz99vAa/Gf53WA3+Y5vTbwW/zn+R3gtXlOiOfvt4HX4j/P6wC/zXN6beC3+M/zO8Br85wQz99vA6/Ff57XAX6b5/TawG/xn+d3gNfmOSGev98GXov/PK8D/DbP6bWB3+I/z+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4HeG2eE+L5+23gtfjP89HAX/OcXhr4av7z/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t/wO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv95Xgf4bZ7TawO/xX+e3wFem+eEeP5+G3gt/vO8DvDbPKfXBn6L/zy/A7w2zwnx/P028Fr853kd4Ld5Tq8N/Bb/eX4HeG2eE+L5+23gtfjP8zrAb/OcXhv4Lf7z/A7w2jwnxPP328Br8Z/ndYDf5jm9NvBb/Of5HeC1eU6I5++3gdfiP89fA7s8p+PAS/Of53eA1+Y5IZ6/3wZei/9bfgd4bZ4T4vn7beC1+L/ld4DX5jkhnr/fBl6L/1t+B3htnhPi+ftt4LX4v+V3gNfmOSGev98GXov/W34HeG2eE+L5+23gtfi/5XOAz+Y5IZ6/3wZei/9b3gb4aZ4T4vn7beC1+L/jb4CX5nkhnr/fBl6L/xv+Bnhr4FaeF+L5+23gtfjf7RnAdwNfDezy/CGev98GXot/2fsA383/Xojn77eB1+KFex/gu/nfDfH8/TbwWrxg7wN8N//7IZ6/3wZei+fvfYDv5v8GxPP328Br8bzeB/hu/u9APH+/DbwWz+l9gO/m/xbE8/fbwGvxbO8DfDf/9yCev98GXosr3gf4bv5vQjx/vw28FvA+wHfzfxfi+ftt4LuB7+b/NsTz99LAX/N/H+L/N8T/b4j/3xD/vyH+f+MfAbZzkkEd94cQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSmsFailed;
impl IconShape for MdSmsFailed {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 12h-2v-2h2v2zm0-4h-2V6h2v4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSupportAgent;
impl IconShape for MdSupportAgent {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M21,12.22C21,6.73,16.74,3,12,3c-4.69,0-9,3.65-9,9.28C2.4,12.62,2,13.26,2,14v2c0,1.1,0.9,2,2,2h1v-6.1 c0-3.87,3.13-7,7-7s7,3.13,7,7V19h-8v2h8c1.1,0,2-0.9,2-2v-1.22c0.59-0.31,1-0.92,1-1.64v-2.3C22,13.14,21.59,12.53,21,12.22z",
            }
            circle {
                cx: "9",
                cy: "13",
                r: "1",
            }
            circle {
                cx: "15",
                cy: "13",
                r: "1",
            }
            path {
                d: "M18,11.03C17.52,8.18,15.04,6,12.05,6c-3.03,0-6.29,2.51-6.03,6.45c2.47-1.01,4.33-3.21,4.86-5.89 C12.19,9.19,14.88,11,18,11.03z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSync;
impl IconShape for MdSync {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
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

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHA0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvif6aXBo7xb/c7vGgQL7r35orv5l/vpYHXBr6aF81vA6/Fv837AN/Niwbxonlv4Lu44n2A7+Zfdhz4KOC9gQcDvwO8Ni+a3wZei3+99wG+mxcd4l/23sB38ZzeB/hunr/jwEcBHw0c59l+B3htXjS/DbwW/zrvA3w3/zqIF+7BwNN5/t4H+G6ev+8G3ovn9DvAa/Oi+W3gtXjRvQ/w3fzrIf5l7w18F8/f+wDfzfP33cB78Wy/A7w2L5rfBl6LF837AN/Nvw3iRfPewHfx/L0P8N08f98NvBdX/A7w2rxofht4Lf5l7wN8N/92iBfdewPfxfP3PsB38/x9NfDXwE8Du7xofht4Lf5l3w28D/92iH+d9wa+i+fvfYDv5j/GbwOvxYvmfYDv5t8G8a/33sB38fy9D/Dd/Pv9NvBavOheBvhr/vUQ/zYfDXwVz9/7AN/Nv89vA6/F83oG8CCe163AywC7/Osg/m0+G/gsXrD3Ab6bf7vfBl6L5/Q+wG8Dfw0c43l9DvDZ/Osg/vWOA08HjvPCvQ/w3fzb/DbwWjzb+wDfzRUvDfwVz2sXeAiwy4sO8a/31cBH8aJ5H+C7+df7beC1uOJ9gO/mOX028Fk8r88BPpsXHeJf7+nAg3lOl4BjPH/vA3w3/zq/DbwW8D7Ad/O8jgO3Asd4TrvACV50iH+dtwZ+iuf1PlzxXTx/7wN8Ny+63wa+G/huXrD3Br6L5/U2wE/zokH863w18FE8p0vAg4Fd4L2B7+L5ex/gu3nRvDTw17xwx4GLPK+vAT6aFw3iX+evgJfmOf0M8NY823sD38Xz9z7Ad/Mf56eBt+I5/TXwMrxoEP865nl9DPDVPKf3Br6L5+99gO/mP8ZHA1/F8xIvGsSL7rWB3+J5vQ7w2zyv9wa+i+fvfYDv5t/vtYHf4nm9DPDX/MsQL7rXBn6L5yVesPcGvovn732A7+bf56WBv+J5vQ7w2/zLEC+61wZ+i+clXrj3Br6L5+99gO/m38c8r9cBfpt/GeJF99rAb/G8xL/svYHv4vl7H+C7+bczz+t1gN/mX4Z40b028Fs8r5cB/pp/2XsD38Xz9z7Ad/NvY57X6wC/zb8M8aJ7aeCveF6vA/w2L5r3Br6L5+99gO/mX+e1gd/ieb0M8Nf8yxD/OuZ5fQzw1bzo3hv4Lp6/9wG+mxfdRwNfxfMSLxrEv86twIN4Tj8DvDX/Ou8NfBfP3/sA382L5qeBt+I5/Q3w0rxoEP86Xw18FM/rBLDLC/fewHfzbO8NfBfP3/sA380Ldxy4yPP6GuCjedEg/nXeGvgpntf7AN/NC/ZdwEOA1+Y5vTfwXTx/7wN8Ny/YewPfxfN6G+CnedEg/vV2gWM8p13gIcAuz+u7gPcGfgd4bZ7XewPfxfP3PsB38/x9NfBRPKdnAA/mRYf41/ts4LN4Xp8DfDbP6buA9+aK3wFem+fvvYHv4vl7H+C7ef6+G3gvnu1zgM/mRYf41zsO3Aoc43m9DPDXXPFdwHvzbL8DvDYv2HsD38Xz9z7Ad/P8fTfwXsAl4MHALi86xL/NZwOfxfPaBV4G+CzgvXlOvwO8Ni/cewPfxfP3PsB38/x9N3Ar8Nn86yD+bY4Dfw08iOe1Cxznef0O8Nr8y94b+C6ev/cBvpvn7ziwy78O4t/upYG/4kX3O8Br86J5b+C7eP7eB/hu/mMg/n3eG/guXjS/A7w2L7r3Br6L5+99gO/m3w/x7/fdwHvxL/sd4LV50RwH3gr4auA4z9/7AN/Nvw/i3++7gPfmX/Y7wGvzovlt4LX4l70P8N382yH+fb4LeG9eNL8DvDYvmt8GXosXzfsA382/DeLf7ruA9+ZF9zvAa/Oi+W3gtXjRvQ/w3fzrIf5tvgt4b/51fgd4bV40vw28Fs92CfhqYBf4Kp6/9wG+m38dxL/edwHvzb/e7wCvzYvmt4HXAp4BfDfw1cAuV7w38F08f+8DfDcvOsS/zncB782/ze8Ar82L5r2Bvwb+mufvvYHv4vl7H+C7edEgXnTHgZfm324X+Gv+47w38F08r/cBvpsXDeJ/t/cGvotnex/gu3nRIf73e2/gu4D3Ab6bfx3E/w0PBm7lXw/x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJA2wRQjuudUQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSyncDisabled;
impl IconShape for MdSyncDisabled {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 6.35V4.26c-.8.21-1.55.54-2.23.96l1.46 1.46c.25-.12.5-.24.77-.33zm-7.14-.94l2.36 2.36C4.45 8.99 4 10.44 4 12c0 2.21.91 4.2 2.36 5.64L4 20h6v-6l-2.24 2.24C6.68 15.15 6 13.66 6 12c0-1 .25-1.94.68-2.77l8.08 8.08c-.25.13-.5.25-.77.34v2.09c.8-.21 1.55-.54 2.23-.96l2.36 2.36 1.27-1.27L4.14 4.14 2.86 5.41zM20 4h-6v6l2.24-2.24C17.32 8.85 18 10.34 18 12c0 1-.25 1.94-.68 2.77l1.46 1.46C19.55 15.01 20 13.56 20 12c0-2.21-.91-4.2-2.36-5.64L20 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSyncProblem;
impl IconShape for MdSyncProblem {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 12c0 2.21.91 4.2 2.36 5.64L3 20h6v-6l-2.24 2.24C5.68 15.15 5 13.66 5 12c0-2.61 1.67-4.83 4-5.65V4.26C5.55 5.15 3 8.27 3 12zm8 5h2v-2h-2v2zM21 4h-6v6l2.24-2.24C18.32 8.85 19 10.34 19 12c0 2.61-1.67 4.83-4 5.65v2.09c3.45-.89 6-4.01 6-7.74 0-2.21-.91-4.2-2.36-5.64L21 4zm-10 9h2V7h-2v6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdSystemUpdate;
impl IconShape for MdSystemUpdate {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14zm-1-6h-3V8h-2v5H8l4 4 4-4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTapAndPlay;
impl IconShape for MdTapAndPlay {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 16v2c2.76 0 5 2.24 5 5h2c0-3.87-3.13-7-7-7zm0 4v3h3c0-1.66-1.34-3-3-3zm0-8v2c4.97 0 9 4.03 9 9h2c0-6.08-4.92-11-11-11zM17 1.01L7 1c-1.1 0-2 .9-2 2v7.37c.69.16 1.36.37 2 .64V5h10v13h-3.03c.52 1.25.84 2.59.95 4H17c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi3++1+O/1O/zbIf7tXhr4KeDB/Pe6FXgb4K/510P82xwHng4c53+GXeAhwC7/Ooh/m88GPov/WT4H+Gz+dRD/Nk8HHsyz/Q2wy3+t48BL8Wy3Ag/hXwfxr/fSwF/xnE4Au/zXOg5c5Dm9DPDXvOgQ/3pfDXwUz/YzwFvz3+Ongbfi2b4G+GhedIh/vacDD+bZ3gf4bv57vDfwXTzbrcBDeNEh/nVeGvgrntMJYJf/HseBizynlwH+mhcN4l/nu4H34tl+Bnhr/nv9NPBWPNv3AO/Niwbxr3MROM6zvQ/w3fz3em/gu3i2XeAELxrEi+6tgZ/iOZ0AdvnvdRy4yHN6G+Cn+ZchXnTfDbwXz/YzwFvzP8NPA2/Fs30P8N78yxAvuovAcZ7tfYDv5n+G9wa+i2fbBU7wL0O8aN4a+Cme0wlgl/8ZjgMXeU5vA/w0LxziRfPTwFvxbD8DvDX/s/w08FY82/cA780Lh/iXHQcu8py+Gvhp/md5a+CjeU4ngF1eMMS/7L2B7+J/p/cBvpsXDPEv+2ngrfjf6WeAt+YFQ/zLzP9u4gVD/MvM/27iBUP8y8z/buIFQ/zLzAv2OcBPA38NvDbw2cBr8Z/rd4DPBn4beGngrYHP4gUTLxjiX2aev9cBfpvn9d3Ae/Gf43uA9+Z5vTbwWzx/4gVD/MvM8/oe4L15/o4DF/nPcQLY5fn7buC9eF7iBUP8y8zz+hjgq3nBfht4Lf5j/Q7w2rxgHw18Fc9LvGCIf5l5Xh8DfDUv2G8Dr8V/rN8BXpsX7KOBr+J5iRcM8S8zz+t7gPfm+TsOXOQ/xwlgl+fvu4H34nmJFwzxLzPP3+sAv83z+i7gvfnP8d3A+/C8Xhv4LZ4/8YIh/mXmBfts4GeAvwZeC/hs4LX5z/XbwGcDvwO8NPBWwGfzgokXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzP/u4kXDPEvM/+7iRcM8S8z/7uJFwzxLzMvnPjvZV448YIh/mXmhRP/vcwLJ14wxL/MvHDiv5d54cQLhviX/TXwUjx/fwO8NP+9/hp4KZ6/vwFemhcM8S97aeC3gWM8p0vAawN/zX+vlwZ+GzjGc7oEvDbw17xgiBfNceCleU5/DezyP8Nx4KV5Tn8N7PLCIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Aql7jEH7zTyMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTimeToLeave;
impl IconShape for MdTimeToLeave {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18.92 5.01C18.72 4.42 18.16 4 17.5 4h-11c-.66 0-1.21.42-1.42 1.01L3 11v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 15c-.83 0-1.5-.67-1.5-1.5S5.67 12 6.5 12s1.5.67 1.5 1.5S7.33 15 6.5 15zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 10l1.5-4.5h11L19 10H5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdTvOff;
impl IconShape for MdTvOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 3.54l1.53 1.53C1.65 5.28 1 6.06 1 7v12c0 1.1.9 2 2 2h15.46l2 2 1.26-1.27L2.27 2.27 1 3.54zM3 19V7h1.46l12 12H3zM21 5h-7.58l3.29-3.3L16 1l-4 4-4-4-.7.7L10.58 5H7.52l2 2H21v11.48l1.65 1.65c.22-.32.35-.71.35-1.13V7c0-1.11-.89-2-2-2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVibration;
impl IconShape for MdVibration {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 15h2V9H0v6zm3 2h2V7H3v10zm19-8v6h2V9h-2zm-3 8h2V7h-2v10zM16.5 3h-9C6.67 3 6 3.67 6 4.5v15c0 .83.67 1.5 1.5 1.5h9c.83 0 1.5-.67 1.5-1.5v-15c0-.83-.67-1.5-1.5-1.5zM16 19H8V5h8v14z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/spcG3gp4aeA4/zvsAn8N/Azw17xgiBfuq4CP5n+3rwY+hucP8YJ9NfBR/N/wNcBH87wQz9+Dgafzf8tDgFt5Tojn77OBz+L/ls8BPpvnhHj+vht4L/5v+RngrXlOiOfvt4HX4v+W3wFem+eEeP5+G3gt/m/5HeC1eU6I5++3gdfi/5bfAV6b54R4/n4beC3+b/kd4LV5Tojn77eB1+L/lt8BXpvnhHj+fht4LV404j/ebwOvxb/sfYDv4kXzO8Br85wQz99vA6/Fi0b8x/tt4LV4wS4Brw38NWBeNL8DvDbPCfH8/TbwWrxoxH+83wZei+fvb4DXBna5wrxofgd4bZ4T4vn7beC1eNGI/3i/DbwWz+t7gPfmOZkXze8Ar81zQjx/vw28Fi8a8R/vt4HX4jm9D/DdPC/zovkd4LV5Tojn77eB1+JFI/7j/TbwWlxxCXht4K95/syL5neA1+Y5IZ6/3wZeixeN+I/328BrAX8DvDawywtmXjS/A7w2zwnx/P028Fq8aMR/vN8GbgXem3+ZedH8DvDaPCfE8/fbwGvxohH/8V4a+GteNOZF8zvAa/OcEM/fbwOvxYtG/PcyL5rfAV6b54R4/n4beC1eNOK/l3nR/A7w2jwnxPP328Br8aIR/73Mi+Z3gNfmOSGev98GXosXjfiPdxzY5UVjXjS/A7w2zwnx/P028Fq8aMR/vJ8Gvhr4bf5l5kXzO8Br85wQz99vA6/Fi0b8x/tt4LWAjwa+hhfOvGh+B3htnhPi+ftt4LV40Yj/eL8NvBZXfDfwMcAuz5950fwO8No8J8Tz99vAa/GiEf/xfht4LZ7tr4G3AW7leZkXze8Ar81zQjx/vw28Fi8a8R/vt4HX4jntAm8D/DbPybxofgd4bZ4T4vn7beC1eNGI/3i/DbwWz99HA1/Ds5kXze8Ar81zQjx/vw28Fi8a8R/vt4HX4gX7buBjgF3AvGh+B3htnhPi+ftt4LV40Yj/eL8NvBYv3F8DbwM8nRfN7wCvzXNCPH+/DbwWL5rf5j/eSwPH+ZftAsd50fwO8No8J8Tz99vAa/F/y+8Ar81zQjx/vw28Fv+3/A7w2jwnxPP328Br8X/L7wCvzXNCPH+/DbwW/7f8DvDaPCfE8/fbwGvxf8vvAK/Nc0I8f78NvBb/t3wO8Nk8J8Tz99vAa/F/y9sAP81zQjx/vw28Fv93/A3w0jwvxPP328Br8X/D3wBvDdzK80I8f78NvBb/uz0D+G7gq4Fdnj/E8/fbwGvxL3sf4Lv53wvx/P028Fq8cO8DfDf/uyGev98GXosX7H2A7+Z/P8Tz99vAa/H8vQ/w3fzfgHj+fht4LZ7X+wDfzf8diOfvt4HX4jm9D/Dd/N+CeP5+G3gtnu19gO/m/x7E8/fbwGtxxfsA383/TYjn77eB1wLeB/hu/u9CPH+/DXw38N3834Z4/l4a+Gv+70P8/4b4/w3x/xvi/zfE/2/8IzYVsEENRwxAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVoiceChat;
impl IconShape for MdVoiceChat {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-2 12l-4-3.2V14H6V6h8v3.2L18 6v8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdVpnLock;
impl IconShape for MdVpnLock {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22 4v-.5C22 2.12 20.88 1 19.5 1S17 2.12 17 3.5V4c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zm-.8 0h-3.4v-.5c0-.94.76-1.7 1.7-1.7s1.7.76 1.7 1.7V4zm-2.28 8c.04.33.08.66.08 1 0 2.08-.8 3.97-2.1 5.39-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H7v-2h2c.55 0 1-.45 1-1V8h2c1.1 0 2-.9 2-2V3.46c-.95-.3-1.95-.46-3-.46C5.48 3 1 7.48 1 13s4.48 10 10 10 10-4.48 10-10c0-.34-.02-.67-.05-1h-2.03zM10 20.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L8 16v1c0 1.1.9 2 2 2v1.93z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWc;
impl IconShape for MdWc {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 22v-7.5H4V9c0-1.1.9-2 2-2h3c1.1 0 2 .9 2 2v5.5H9.5V22h-4zM18 22v-6h3l-2.54-7.63C18.18 7.55 17.42 7 16.56 7h-.12c-.86 0-1.63.55-1.9 1.37L12 16h3v6h3zM7.5 6c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2zm9 0c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEf6/jXLHLfw/Ef67XAl4bOA68NPBg4MG8cLcCtwJ/DewCvw38Dv85EP+xXhp4K+C1gdfmP9ZvA78N/Azw1/zHQPz7PRh4K+CjgQfzX+NW4KuBnwFu5d8O8W/32sBHAW/Nf6+fBr4G+G3+9RD/eq8NfBbw2vzP8tvA5wC/zYsO8aJ7MPBVwFvzP9tPAx8D3Mq/DPGi+Szgs/nfYxf4auBzeOEQL9yDgZ8CXpp/v2cAtwJ/Dezy/B0HXhp4MPAg/v3+Gngb4FaeP8QL9t7AVwHH+bf5HeC3gd8Gfpt/m9cGXht4beC1+LfZBd4H+GmeF+L5+y7gvfnX+xvgu4HvBnb5j3UceG/gvYGX4l/vu4H34Tkhntd3Ae/Nv873AN8N/Db/NV4beG/gvfjX+W7gfXg2xLMdB34KeG1edD8DfDRwK/89Hgx8NfBWvOh+G3gbYBdAPNtx4LeBl+Jf9gzgvYHf5n+G1wa+G3gQ/7K/AV4b2AUQz+k48NvAS/GCfQ3w0fzPcxz4bOCjeMH+BnhtYJcrEM/rOPDbwEvxnC4B7w38NP+zvTXw3cAxntPfAK8N7PJsiOfvOPDbwEtxxTOAtwb+mn+b1wJeGnht4Djw2jx/vw3sAr8N/DXwO/zbvDTw08CDuOJvgNcGdnlOiBfsOPDbXPHawC7/Om8FvDXw1sBx/m12gZ8Gfhr4Gf51jgO/zRWvDezyvBAv3HGu2OVF917AZwMP5j/WrcBnA9/Di+44V+zy/CH+47w18FXAg/nPdSvwMcBP8++H+Pc7DvwU8Nr81/pt4G2AXf7tEP8+rw38FHCc/x67wNsAv82/DeLf7r2B7+J/hvcBvpt/PcS/zXsD38W/zd8Au8CtwK1c8WDgwcBx4KX4t3kb4Kf510H82xwHfht4Kf5ll4DfBn4a+GlglxfuOPDWwFsDrw0c41/2N8BrA7v86yD+7Y4Dvw28FC/Y9wCfDdzKv82Dgc8G3osX7G+A1wZ2+ddD/PscB34beCme088AHw3cyn+MBwNfDbwVz+lvgNcGdvm3Qfz7HQd+G3gprvgc4LP5z/HZwGdxxd8Arw3s8m+H+I9xHPhp4KuBn+Y/13sD7w28NbDLvw/i/zfE/2+I/1rHgbcC3hp4MPDSXPHXwK3ATwM/A+zyXwPxX+M48FHARwPHeeF2ga8GvgbY5T8X4j/fSwPfBbw0/zp/DbwP8Nf850H85/tt4LX4t/kd4LX5z4P4z3cc+G3gpfjX+RvgtYFd/vMg/mscB34beCleNH8DvDawy38uxH+d48BvAy/FC/c3wGsDu/znQ/zXOg78NvBSPH9/A7w2sMt/DcR/vePAbwMvxXP6G+C1gV3+6yD+exwHfht4Ka74G+C1gV3+ayH++xwHfpsrXhvY5b8e4r/Xca7Y5b8H4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjGoe0QelBojsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifi;
impl IconShape for MdWifi {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MdWifiOff;
impl IconShape for MdWifiOff {
    fn view_box(&self) -> &str {
        "0 0 24 24"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M22.99 9C19.15 5.16 13.8 3.76 8.84 4.78l2.52 2.52c3.47-.17 6.99 1.05 9.63 3.7l2-2zm-4 4c-1.29-1.29-2.84-2.13-4.49-2.56l3.53 3.53.96-.97zM2 3.05L5.07 6.1C3.6 6.82 2.22 7.78 1 9l1.99 2c1.24-1.24 2.67-2.16 4.2-2.77l2.24 2.24C7.81 10.89 6.27 11.73 5 13v.01L6.99 15c1.36-1.36 3.14-2.04 4.92-2.06L18.98 20l1.27-1.26L3.29 1.79 2 3.05zM9 17l3 3 3-3c-1.65-1.66-4.34-1.66-6 0z",
            }
        }
    }
}
