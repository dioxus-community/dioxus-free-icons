use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoAccessibility;
impl IconShape for GoAccessibility {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.923 5.302a3 3 0 10-3.847 0A2.713 2.713 0 005.9 5.5H2A.75.75 0 002 7h3.3l-.578 5.163-.362 2.997a.75.75 0 101.49.18L6.132 13h3.736l.282 2.34a.75.75 0 101.49-.18l-.362-2.997L10.7 7H14a.75.75 0 000-1.5h-3.899a2.697 2.697 0 00-.178-.198zM9.5 3a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-.3 4.073l.495 4.427h-3.39l.496-4.427a1.207 1.207 0 012.398 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoAlert;
impl IconShape for GoAlert {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoApps;
impl IconShape for GoApps {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 3.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5A1.75 1.75 0 015.75 7.5h-2.5A1.75 1.75 0 011.5 5.75v-2.5zM3.25 3a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5A.25.25 0 006 5.75v-2.5A.25.25 0 005.75 3h-2.5zM1.5 10.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5a1.75 1.75 0 01-1.75-1.75v-2.5zM3.25 10a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5zM8.5 3.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5A1.75 1.75 0 018.5 5.75v-2.5zM10.25 3a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5zM8.5 10.25c0-.966.784-1.75 1.75-1.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a1.75 1.75 0 01-1.75 1.75h-2.5a1.75 1.75 0 01-1.75-1.75v-2.5zm1.75-.25a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8fy8D/DX/N7w08Fc8LwT8NfBS/P/0NwI+G/gs/n/6HAHHgVuBY/z/cgl4sLjivYHv4v+XtwF+WjzbewNfDRzj/7ZLwHsDPw0gntNx4KOBtwZeiv9b/gb4aeCrgV2uQPz/hvj/DfH/G+L/N8TzOg58FvDWXPHdwOfwv89x4LOAt+aK7wY+h+eEeF5fDXwUz+lrgI/mf5evBj6K5/Q1wEfzbIjndRE4znO6FXgI/7tcBI7znG4FHsKzIZ6Xef7E/y7m+RPPhnhe5vkT/zrHgc8C3hp4MP82twLfDXwO/3rm+RPPhnhe5vkT/zpfDXwU/zG+Bvho/nXM8yeeDfG8zPMn/nUuAsf5j3Er8BD+dczzJ54N8bzM8yf+dXaBY/zHeAbwYP51zPMnng3xvMzzJ/51Phv4LP5jfA7w2fzrmOdPPBvieZnnT/zrfTXw1sCD+Ld5BvDdwGfzr2eeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ4/8b+Lef7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5mefvZYC/5n+Hlwb+iudPPBvief018FL83/Q3wEvzbIjn9dnAZ/F/0+cAn82zIZ7XceBW4Bj/t1wCHgzs8myI5++9ge/i/5a3AX6a54R4wd4b+GrgGP+7XQLeG/hpnhfihTsOfDTw1sBL8b/L3wA/DXw1sMvzh/j/DfH/G+L/N8T/b/wjtJyH+OG6RnMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArchive;
impl IconShape for GoArchive {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5a.25.25 0 00-.25.25v1.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-1.5a.25.25 0 00-.25-.25H1.75zM0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v1.5A1.75 1.75 0 0114.25 6H1.75A1.75 1.75 0 010 4.25v-1.5zM1.75 7a.75.75 0 01.75.75v5.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25v-5.5a.75.75 0 111.5 0v5.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25v-5.5A.75.75 0 011.75 7zm4.5 1a.75.75 0 000 1.5h3.5a.75.75 0 100-1.5h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n0+C3hrrvhu4Gv4r/FRwHtzxU8Dn8O/DeLf7ruA9+Y5fQzw1fzn+mjgq3hO3w28D/96iH+b7wLem+f118DL8J/rr4CX5nl9N/A+/Osg/vW+C3hvnr+/AV6a/1x/DbwUz993A+/Diw7xr/NdwHvzgn0O8Nn85/ps4LN4wb4beB9eNIgX3XcB780L9j3Ae/Nf47uB9+IF+27gffiXIV403wW8Ny/Y9wDvzX+t7wbeixfsu4H34YVD/Mu+C3hvXrDvAd6b/x7fDbwXL9h3A+/DC4Z44b4LeG9esO8B3pv/Xt8NvBcv2HcD78Pzh3jBvgt4b16w7wHem/8Zvht4L16w7wbeh+eFeP6+C3hvXrDvAd6b/1m+G3gvXrDvBt6H54R4Xt8FvDcv2PcA783/TN8NvBcv2HcD78OzIZ7TdwHvzQv2PcB78z/bdwPvxQv23cD7cAXi2b4LeG9esO8B3pv/Hb4beC9esO8G3gdAXPFdwHvz/8t3A+8j4LOBz+L/p88RcBE4zv9PtwrYBY7x/9MzBHw28Fn8//Q54orvBt6L/1++B3hv8WzfDbwXL9h3A+/D/w7fBbw3L9j3AO8NIJ7TdwPvxQv23cD78D/bdwHvzQv2PcB7cwXieX038F68YN8NvA//M30X8N68YN8DvDfPhnj+vht4L16w7wbeh/9Zvgt4b16w7wHem+eEeMG+G3gvXrDvBt6H/xm+C3hvXrDvAd6b54V44b4beC9esO8G3of/Xt8FvDcv2PcA783zh/iXfTfwXrxg3w28D/89vgt4b16w7wHemxcM8aL5buC9eMG+G3gf/mt9F/DevGDfA7w3LxziRffdwHvxgn038D781/gu4L15wb4HeG/+ZYh/ne8G3osX7GuAj+Y/11cDH8UL9j3Ae/OiQfzrfTfwXrxg4j+XecG+B3hvXnSIf5vvBt6L50/85zLP3/cA782/DuLf7ruB9+I5/Qzw1vzn+mngrXhO3wO8N/96iH+frwY+iiu+B/hoYJf/XMeB7wbeCrgEfDfw0fzbIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj3c5ewDA+4bsAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowBoth;
impl IconShape for GoArrowBoth {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.72 3.72a.75.75 0 011.06 1.06L2.56 7h10.88l-2.22-2.22a.75.75 0 011.06-1.06l3.5 3.5a.75.75 0 010 1.06l-3.5 3.5a.75.75 0 11-1.06-1.06l2.22-2.22H2.56l2.22 2.22a.75.75 0 11-1.06 1.06l-3.5-3.5a.75.75 0 010-1.06l3.5-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/z7Hgc8C3porvhv4HP5rIf77fDXwUTynrwE+mv86iP8+F4HjPKdbgYfwXwfx38c8f+K/DuK/j3n+xH8dxH8f8/yJ/zqI/z7m+RP/dRD/fczzJ/7rIP77mOdP/NdB/Pcxz5/4r4P472OeP/FfB/Hfxzx/4r8O4r+Pef7Efx3Efx/z/In/Ooj/Pub5E/91EP99zPMn/usg/vuY50/810H89zHPn/ivg/jvY54/8V8H8d/HPH/ivw7i3+6jgPfmip8GPod/HfP8iX+dzwI+miu+GvgcXnSIf5uPBr6K5/TdwPvwojPPn3jRfRfw3jynzwE+mxcN4t/mr4CX5nl9N/A+vGjM8ydeNN8FvDfP61bgIbxoEP82fw28FM/fdwPvw7/MPH/iX/ZdwHvz/D0DeDAvGsS/zWcDn8UL9t3A+/DCmedPvHDfBbw3L9jnAJ/Niwbxb/fdwHvxgn038D68YOb5Ey/YdwHvzQv2PcB786JD/Pt8N/BevGDfDbwPz595/sTz913Ae/OCfQ/w3vzrIP79vht4L16w7wbeh+dlnj/xvL4LeG9esO8B3pt/PcR/jO8G3osX7LuB9+E5medPPKfvAt6bF+x7gPfm3wbxH+e7gffiBftu4H14NvP8iWf7LuC9ecG+B3hv/u0Q/7G+G3gvXrDvBt6HK8zzJ674LuC9ecG+B3hv/n0Q//G+G3gvXrDvBt4HMM+fgO8C3psX7HuA9+bfD/Gf47uB9+IF+27gvXn+vht4b16w7wHem/8YiP883w28F/+xvgd4b/7jIP5zfTfwXvzH+B7gvfmPhfjP993Ae/Hv8z3Ae/MfD/Ff47uB9+Lf5nuA9+Y/B+K/zncD78W/zvcA781/HsR/re8G3osXzfcA781/LsR/ve8G3osX7nuA9+Y/H+K/x3cD78Xz9z3Ae/NfA/Hf57uB9+I5fQ/w3vzXQfz3+mzgvbniu4HP5r8W4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RRYBrQZicdgYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowDown;
impl IconShape for GoArrowDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13.03 8.22a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06 0L3.47 9.28a.75.75 0 011.06-1.06l2.97 2.97V3.75a.75.75 0 011.5 0v7.44l2.97-2.97a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADQElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfEf63PAt6aK74b+Br+eyH+63wX8N48p48Bvpr/Poj/Gt8FvDfP66+Bl+G/D+I/33cB783z9zfAS/PfB/Gf67uA9+YF+xzgs/nvg/jP813Ae/OCfQ/w3vz3Qvzn+C7gvXnBvgd4b/77If7jfRfw3rxg3wO8N/8zIP5jfRfw3rxg3wO8N/9zIP7jfBfw3rxg3wO8N/+zIP5jfBfw3rxg3wO8N//zIP79vgt4b16w7wHem/+ZEP8+3wW8Ny/Y9wDvzf9ciH+77wLemxfse4D35n82xL/NdwHvzQv2PcB78z8f4l/vu4D35gX7HuC9+d8B8a/zXcB784J9D/De/O+BeNF9F/De/M90K/DdwOfwr4N40Xw28Fn8z/c1wEfzokO8aC4Cx/mf71bgIbzoEC+aXeAY//M9A3gwLzrEi+azgc/if77PAT6bFx3iRffdwHvxP9MzgO8GPpt/HcS/zncD78UL9t3A+/C/B+Jf77uB9+IF+27gffjfAfFv893Ae/GCfTfwPvzPh/i3+27gvXjBvht4H/5nQ/z7fDfwXrxg3w28D/9zIf79vht4L16w7wbeh/+ZEP8xvht4L16w7wbeh/95EP9xvht4L16w7wbeh/9ZEP+xvht4L16w7wbeh/85EP/xvht4L16w7wbeh/8ZEP85vht4L16w7wbeh/9+iP883w28Fy/YdwPvw38vxH+u7wbeixfsc4DP5r8P4j/fdwPvxfN3K/AQ/vsg/mt8N/BePK9nAA/mvw/iv853A+/Fc/oc4LP574P4r/XZwHtzxXcDn81/L8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwK5F1JBuLA/AwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowLeft;
impl IconShape for GoArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.78 12.53a.75.75 0 01-1.06 0L2.47 8.28a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 1.06L4.81 7h7.44a.75.75 0 010 1.5H4.81l2.97 2.97a.75.75 0 010 1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADQElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/wyfBXw0V3w18Dn810D89/ts4LN4Tt8NvA//+RD//S4Cx3le3w28D/+5EP/9doFjPH/fDbwP/3kQ//0+G/gsXrDvBt6H/xyI/xm+G3gvXrDvBt6H/3iI/zm+G3gvXrDvBt6H/1iI/1m+G3gvXrDvBt6H/ziI/3m+G3gvXrDvBt6H/xiI/5m+G3gvXrDvBt6Hfz/E/1zfDbwXL9h3A+/Dvw/if7bvBt6LF+y7gffh3w7xP993A+/FC/bdwPvwb4P43+G7gffiBftu4H3410P87/HdwHvxgn038D786yD+d/lu4L14wb4beB9edIj/fb4beC9esO8G3ocXDeJf7zjwWcBbAw/mf6bvBt6HfxniX++rgY/if77PAT6bFw7xr3cROM7/fLcCD+GFQ/zr7QLH+J/vGcCDeeEQ/3qfDXwW//N9DvDZvHCIf5uvBt4aeBD/M30P8N78yxD/+3wX8N68YN8DvDcvGsT/Lt8FvDcv2PcA782LDvG/x3cB780L9j3Ae/Ovg/jf4buA9+YF+x7gvfnXQ/zP913Ae/OCfQ/w3vzbIP5n+y7gvXnBvgd4b/7tEP9zfRfw3rxg3wO8N/8+iP+Zvgt4b16w7wHem38/xP883wW8Ny/Y9wDvzX8MxP8s3wW8Ny/Y9wDvzX8cxP8c3wW8Ny/Y9wDvzX8sxP8M3wW8Ny/Y9wDvzX88xH+/zwY+ixfse4D35j8H4r/fXwEvzfP3PcB7858H8d/vr4GX4nl9D/De/OdC/Pf7aOCreE7fA7w3//kQ/zN8NPDeXPHTwGfzXwPx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+y3FBBAdh0fgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowRight;
impl IconShape for GoArrowRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.22 2.97a.75.75 0 011.06 0l4.25 4.25a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06-1.06l2.97-2.97H3.75a.75.75 0 010-1.5h7.44L8.22 4.03a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/2T4KeG+u+Gngc/iPhfif66OBr+I5fTfwPvzHQfzP9VfAS/O8vht4H/5jIP7n+mvgpXj+vht4H/79EP9zfTbwWbxg3w28D/8+iP/Zvht4L16w7wbeh387xP983w28Fy/YdwPvw78N4n+H7wbeixfsu4H34V8P8b/HdwPvxQv23cD78K+D+N/lu4H34gX7buB9eNEh/vf5buC9eMG+G3gfXjSI/52+G3gvXrDvBt6HfxniX3Yc+CzgrYEH87/HdwPvwwuH+Jd9NfBR/O/0OcBn84Ih/mUXgeP873Qr8BBeMMS/bBc4xv9OzwAezAuG+Jd9NvBZ/O/0OcBn84IhXjRfDbw18CD+9/ge4L154RD/O30X8N68YN8DvDf/MsT/Pt8FvDcv2PcA782LBvG/y3cB780L9j3Ae/OiQ/zv8V3Ae/OCfQ/w3vzrIP53+C7gvXnBvgd4b/71EP/zfRfw3rxg3wO8N/82iP/Zvgt4b16w7wHem387xP9cXw18FC/Y9wDvzb8P4j/HZwFvzRXfDXwN/3rmBfse4L3590P8x/su4L15Th8DfDX/Oub5+x7gvfmPgfiP9V3Ae/O8/hp4Gf51fhp4K57T9wDvzX8cxH+c7wLem+fvb4CX5l/nOPDdwFsBl4DvBj6a/1iI/xjfBbw3L9jnAJ/N/zyIf7/vAt6bF+x7gPfmfybEv893Ae/NC/Y9wHvzPxfi3+67gPfmBfse4L35nw3xb/NdwHvzgn0P8N78z4f41/su4L15wb4HeG/+d0D863wX8N68YN8DvDf/eyBedN8FvDcv2PcA783/LogXzXcB783/HrcC3w18Di8c4l/22cBn8b/T1wAfzQuG+JddBI7zv9OtwEN4wRD/sl3gGP87PQN4MC8Y4l/22cBn8b/T5wCfzQuGeNF8N/Be/O/xDOC7gc/mhUO86L4beC9esO8G3of/XRD/Ot8NvBcv2HcD78P/Hoh/ve8G3osX7LuB9+F/B8S/zXcD78UL9t3A+/A/H+Lf7ruB9+IF+27gffifDfHv893Ae/GCfTfwPvzPhfj3+27gvXjBvht4H/5nQvzH+G7gvXjBPgf4bP7nQfzH+W7gvXj+/hp4Gf7nQfzH+m7gvXhefwO8NP/zIP7jfTfwXjynjwG+mv95EP85Pht4a674buCr+Z8J8f8b4v83xP9viP/f+EfElntB0tBOMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowSwitch;
impl IconShape for GoArrowSwitch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.22 14.78a.75.75 0 001.06-1.06L4.56 12h8.69a.75.75 0 000-1.5H4.56l1.72-1.72a.75.75 0 00-1.06-1.06l-3 3a.75.75 0 000 1.06l3 3zm5.56-6.5a.75.75 0 11-1.06-1.06l1.72-1.72H2.75a.75.75 0 010-1.5h8.69L9.72 2.28a.75.75 0 011.06-1.06l3 3a.75.75 0 010 1.06l-3 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADs0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+uzgI/miq8GPof/Woj/Pt8FvDfP6buB9+G/DuK/x3cB783z993A+/BfA/Ff77uA9+aF+27gffjPh/iv9V3Ae/Oi+W7gffjPhfiv813Ae/Ov893A+/CfB/Ff47uA9+bf5ruB9+E/B+I/33cB782/z3cD78N/PMR/ru8C3pv/GN8NvA//sRD/eb4LeG/+Y3038D78x0H85/gu4L15wb4HeC+ev+8B3osX7LuB9+E/BuI/3ncB780L9j3AewPm+RPw3cB78YJ9N/A+/Psh/mN9F/DevGDfA7w3V5jnT1zx3cB78YJ9N/A+/Psg/uN8F/DevGDfA7w3z2aeP/Fs3w28Fy/YdwPvw78d4j/GdwHvzQv2PcB785zM8yee03cD78UL9t3A+/Bvg/j3+y7gvXnBvgd4b56Xef7E8/pu4L14wb4beB/+9RD/Pt8FvDcv2PcA783zZ54/8fx9N/BevGDfDbwP/zqIf7vvAt6bF+x7gPfmBTPPn3jBvht4L16w7wbehxcd4t/ms4HP4gX7HuC9eeHM8ydeuO8G3osX7HOAz+ZFg/i3uQgc5/n7HuC9+ZeZ50/8y74beC+ev78GXoYXDeLfZhc4xvP6HuC9edGY50+8aL4beC+e198AL82LBvFv89nAZ/Gcvgd4b1505vkTL7rvBt6L5/QxwFfzokH823028N5c8d3AZ/OvY54/8a/z2cBbc8V3A1/Niw7x38c8f+K/DuK/j3n+xH8dxH8f8/yJ/zqI/z7m+RP/dRD/fczzJ/7rIP77mOdP/NdB/Pcxz5/4r4P472OeP/FfB/Hfxzx/4r8O4r+Pef7Efx3Efx/z/In/Ooj/Pub5E/91EP99zPMn/usg/vuY50/810H89zHPn/ivg/jvY54/8V8H8d/HPH/ivw7iv88ucIzn9AzgwfzXQfz3+Wzgs3hOnwN8Nv91EP+9vhp4a674buCz+a+F+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjhsdrQX4OiWcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoArrowUp;
impl IconShape for GoArrowUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.47 7.78a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0l4.25 4.25a.75.75 0 01-1.06 1.06L9 4.81v7.44a.75.75 0 01-1.5 0V4.81L4.53 7.78a.75.75 0 01-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/v+PAZwFvDTyY/xq3At8NfA7/Poh/v68GPor/Hl8DfDT/doh/v4vAcf573Ao8hH87xL/fLnCM/x7PAB7Mvx3i3++zgc/iv8fnAJ/Nvx3iP8ZXA28NPIj/Gs8Avhv4bP59EP/xzPMn/nXM8yf+4yD+45nnT/zrmOdP/MdB/Mczz5/41zHPn/iPg/iPZ54/8a9jnj/xHwfxH888f+Jfxzx/4j8O4j+eef7Ev455/sR/HMR/PPP8iX8d8/yJ/ziI/3jm+RP/Oub5E/9xEP/xzPMn/nXM8yf+4yD+45nnT/zrmOdP/MdB/Mczz5/41zHPn/iPg/iPZ54/8a9jnj/xHwfxH888f+Jfxzx/4j8O4j/WawO/xfMn/nXM8/c6wG/zHwPxH+vpwIN5XpeA4/zr7ALHeF63Ag/hPwbiP85nA5/F8/c1wEfzr/PVwEfx/H0O8Nn8+yH+Y7w08Fc8f5eABwO7/OscB24FjvH8vQzw1/z7IP5j/Bbw2jx/bwP8NP82bw38FM/fbwOvw78P4t/vo4Gv4vn7GeCt+ff5aeCteP4+Bvhq/u0Q/z4PBv4KOM7zugS8NHAr/z4PBv4aOMbz2gVeBriVfxvEv89PAW/N8/cxwFfzH+Ojga/i+ftp4G34t0H827018FM8f78DvDb/sX4beC2ev7cBfpp/PcS/zXHg6cBxnr+XAf6a/1gvDfwVz98u8BBgl38dxL/NVwMfxfP3OcBn85/js4HP4vn7GuCj+ddB/Ou9NvBbPH/PAB7Mf65bgQfx/L0O8Nu86BD/ek8HHszz9zrAb/Of67WB3+L5uxV4CC86xL/OZwOfxf9snwN8Ni8axIvupYG/4n+HlwH+mn8Z4kX3W8Br87/DbwOvw78M8aL5aOCr+N/lY4Cv5oVD/MseDPwVcJz/XXaBlwFu5QVD/Mt+Cnhr/nf6aeBteMEQL9xbAz/F8/c7wGvzP8NvA6/F8/c2wE/z/CFesOPA04HjPH8vA/w1/zO8NPBXPH+7wEOAXZ4X4gX7auCjeP4+B/hs/mf5bOCzeP6+Bvhonhfi+Xtt4Ld4/p4BPJj/mW4FHsTz9zrAb/OcEM/f04EH8/y9DvDb/M/02sBv8fz9NfAyPCfE83pt4Ld4/r4G+Gj+Z/tq4KN4/l4H+G2eDfG8fhp4K57XJeDBwC7/sx0HbgWO8bx+Bnhrng3xvJ4OPJjn9T7Ad/O/w3sD38XzuhV4CM+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOe1Cxzjeb0P8N387/DewHfxvJ4BPJhnQzyvnwbeiue1C3w28DX8z/ZRwGcDx3le3wO8N8+GeF5vDfwU/ze9DfDTPBvi+ftr4KX4v+VvgJfmOSGev5cGfhs4xv8Nl4DXBv6a54R4wd4a+G7gGP+7XQLeG/hpnhfihXtp4KuB1+J/p98BPhr4a54/xIvmtYGPBo4Dr8X/bL8D7AJfDfw2Lxzi/zfE/2+I/98Q/7/xj/dMrEFAkgfOAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBeaker;
impl IconShape for GoBeaker {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5 5.782V2.5h-.25a.75.75 0 010-1.5h6.5a.75.75 0 010 1.5H11v3.282l3.666 5.76C15.619 13.04 14.543 15 12.767 15H3.233c-1.776 0-2.852-1.96-1.899-3.458L5 5.782zM9.5 2.5h-3V6a.75.75 0 01-.117.403L4.73 9h6.54L9.617 6.403A.75.75 0 019.5 6V2.5zm-6.9 9.847L3.775 10.5h8.45l1.175 1.847a.75.75 0 01-.633 1.153H3.233a.75.75 0 01-.633-1.153z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFlUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv89LAWwEvDTwYeGme0y7w28BfA78N/A7/+RD/uR4MfBTw1sCD+de5Ffhu4HuAW/nPgfjPcRz4KuC9+Y/x1cDnALv8x0L8x3tr4LuA4/zH2gXeB/hp/uMg/mN9F/De/Of6buB9+I+B+I9xHPgu4K35r/HdwPvw74f4j/FTwFvzL/sb4LuBvwZ2gb/mipcGjgOvDbw18FL8y74beB/+fRD/ft8FvDcv3PcAnw3cyovmwcBnA+/FC/c5wGfzb4f493lr4Kd4wZ4BvDfw2/zbvDbw3cCDeMHeBvhp/m0Q/3bHgacDx3n+fgd4a2CXf5/jwG8DL8Xztws8BNjlXw/xb/fdwHvx/P0O8Nr8x/pr4KV4/j4H+Gz+9RD/Ng8Gns7z9wzgpYFd/mMdB/4aeBDPaxd4CLDLvw7i3+argY/i+Xsd4Lf5z/HawG/x/H0O8Nn86yD+bZ4OPJjn9T3Ae/Of66eBt+J53Qo8hH8dxL/eSwN/xfP3EOBW/nM9GHg6z99DgFt50SH+9T4b+Cye198AL81/jb8GXorn9THAV/OiQ/zr/TTwVjyvjwG+mv8anw18Fs/rZ4C35kWH+Nf7K+CleV6vA/w2/zVeG/gtntfvAK/Niw7xr2eev5cB/pr/Gi8N/BXP66+Bl+FFh/jXM8+f+K9lnj/xokP865nnT/zXMs+feNEh/vXM8yf+a5nnT7zoEP965vkT/7XM8ydedIh/PfP8if9a5vkTLzrEv555/sR/LfP8iRcd4l/PPH/iv5Z5/sSLDvGvZ54/8V/LPH/iRYf41zPPn/ivZZ4/8aJD/OuZ50/81zLPn3jRIf71zPMn/muZ50+86BD/eub5E/+1zPMnXnSIfz3z/In/Wub5Ey86xL/OawO/xfN6BvBg/mvtAsd4Xq8D/DYvGsSL7jjwV8CDeV6/A7w2/7V+G3gtntetwMsAu/zLEC+6rwY+iufvfYDv5r/WewPfxfP3NcBH8y9DvGheG/gtnr9nAA/mv8etwIN4/l4H+G1eOMS/7DjwV8CDef5eB/ht/nu8NvBbPH+3Ai8D7PKCIf5lXw18FM/f1wAfzX+vrwY+iufva4CP5gVDvHCvDfwWz98zgJcGdvnvdRz4a+BBPH+vA/w2zx/iBTsO/BXwYJ6/1wF+m/8ZXhv4LZ6/W4GXAXZ5XogX7KuBj+L5+xrgo/mf5auBj+L5+xrgo3leiOfvtYHf4vl7BvDSwC7/sxwH/hp4EM/f6wC/zXNCPK/jwF8BD+b5ex3gt/mf6bWB3+L5uxV4CM8J8bzeG/gunr+vAT6a/9m+Gvgonr/3Ab6bZ0M8r98GXovn9QzgpYFd/mc7Dvw18CCe1+8Ar82zIZ6Xef5eB/ht/nd4beC3eP7EsyGel/m/TTwb4nn9NvBa/N/0O8Br82yI5/XewHfxf9P7AN/NsyGev98GXov/W34HeG2eE+L5Ow78NvBS/N/wO8BbA7s8J8QL997AewMvDRzjf5dLwF8D3w18N88f4v83xP9viP/fEP+/If7z/RXw0vzb/DXwMvznQfzn+2vgpfi3+RvgpfnPg/jP993Ae/Fv8z3Ae/OfB/Gf78HA0/m3eQhwK/95EP81vhr4KP51vgb4aP5zIf5rHAd+G3gpXjR/A7w0//kQ/3WOA58NfBQv3NcAH81/DcR/vQcDHw28NvBSXPE3wG8DXw3cyn8d/hHJ4cdB+Y6qIwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBell;
impl IconShape for GoBell {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z",
            }
            path {
                d: "M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv81rAawOvDRwHXprn9NfALvDbwG8Dv8N/PsR/rgcDnwW8NXCcf51d4LuBrwFu5T8H4j/Hg4HPAt6b/xjfDXwOcCv/sRD/8T4K+GzgOP+xdoHPBr6G/ziI/zjHge8C3pr/XD8NvA+wy78f4j/GceC3gJfmv8ZfA68D7PLvg/j3Ow78FvDS/Nf6a+B1gF3+7RD/fj8FvDX/PX4aeBv+7RD/Ph8NfBX/vT4G+Gr+bRD/dg8G/go4zn+vXeBlgFv510P823038F78z/A9wHvzr4f4t3kw8HT+Z3kIcCv/Ooh/m+8G3ov/Wb4G+Gj+dRD/NheB4/zPsguc4F8H8a/32sBv8T/T6wC/zYsO8a/32cBn8T/T5wCfzYsO8a/328Br8T/T7wCvzYsO8a/3V8BL8z/TXwMvw4sO8a9n/mcTLzrEv575n0286BD/euZ/NvGiQ/zrmf/ZxIsO8a9n/mcTLzrEv575n0286BD/euZ/NvGiQ/zrmf/ZxIsO8a9n/mcTLzrEv575n0286BD/euZ/NvGiQ/zrmf/ZxIsO8a9n/mcTLzrEv575n0286BD/Og8Gns7/bG8D/DQvGsS/zm8Br83/bLvAQ4Bd/mWIF917A9/F/w4/DbwN/zLEi+Y48HTgOP97vA7w27xwiBfNTwFvzf8utwIvA+zygiH+ZW8N/BT/O30N8NG8YIgX7jjwdOA4/3u9DvDbPH+IF+6ngLfmf7dbgZcBdnleiBfsrYGf4v+GrwE+mueFeP6OA38FPJj/O14G+GueE+L5+2rgo/i/5a+B1wF2eTbE83pr4Kf4v+lzgM/m2RDP67eB1+L/pluBh/BsiOdl/m8Tz4Z4Xub/rkvAcZ4N8bz+Gngp/m/6HuC9eTbE83pv4Lv4v+llgL/m2RDP328Dr8X/Le8DfDfPCfH8HQd+Gngt/ve7BHw08N08L8QL99LAWwMPBm7lf59bgZ8Gdnn+EP+/If5/Q/z/hvj/DfGf762B7wKO86+zC7wP8NP850H85/tp4K34t/kZ4K35z4P4z/fRwFfxb/MxwFfznwfxn+/BwNP5t3kIcCv/eRD/Nb4a+Cj+dT4H+Gz+cyH+6/w28Fq8aH4GeGv+8yH+a3018FG8cF8DfDT/NRD/9R4MfDTw1sCDuOJvgN8Gvhq4lf86/CPh/n5B5zX/RwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBellFill;
impl IconShape for GoBellFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16c.9 0 1.7-.6 1.9-1.5.1-.3-.1-.5-.4-.5h-3c-.3 0-.5.2-.4.5.2.9 1 1.5 1.9 1.5zM3 5c0-2.8 2.2-5 5-5s5 2.2 5 5v3l1.7 2.6c.2.2.3.5.3.8 0 .8-.7 1.5-1.5 1.5h-11c-.8.1-1.5-.6-1.5-1.4 0-.3.1-.6.3-.8L3 8.1V5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBellSlash;
impl IconShape for GoBellSlash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5c-.997 0-1.895.416-2.534 1.086A.75.75 0 014.38 1.55 5 5 0 0113 5v2.373a.75.75 0 01-1.5 0V5A3.5 3.5 0 008 1.5zM4.182 4.31L1.19 2.143a.75.75 0 10-.88 1.214L3 5.305v2.642a.25.25 0 01-.042.139L1.255 10.64A1.518 1.518 0 002.518 13h11.108l1.184.857a.75.75 0 10.88-1.214l-1.375-.996a1.196 1.196 0 00-.013-.01L4.198 4.321a.733.733 0 00-.016-.011zm7.373 7.19L4.5 6.391v1.556c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01.015.015 0 00.005.012.017.017 0 00.006.004l.007.001h9.037zM8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviPcRz4LOC9geP859oFvhv4GP79EP9+x4HfAl6a/1p/DbwOsMu/HeLf5zjwW8BL89/jr4HXAXb5t0H82x0Hfgt4af57/TXwOsAu/3qIf5vjwG8BL83/DH8NvA6wy78O4l/vOPBbwEvzP8tfA68D7PKiQ/zrHAd+C3hpXrDvAd6b/xzfDbwXL9hfA68D7PKiQbzojgO/Bbw0L9j3AO/Nf67vBt6LF+yvgdcBdvmXIV40x4HfAl6aF+x7gPfmv8Z3A+/FC/bXwOsAu7xwiH/ZceC3gJfmBfse4L35r/XdwHvxgv018DrALi8Y4oU7DvwW8NK8YN8DvDf/Pb4beC9esL8GXgfY5flDvGDHgd8CXpoX7HuA9+a/13cD78UL9tfA6wC7PC/E83cc+C3gpXnBvgd4b/5n+G7gvXjB/hp4HWCX54R4XseB3wJemhfse4D35n+W7wbeixfsr4HXAXZ5NsRzOg78FvDSvGDfA7w3/zN9N/BevGB/DbwOsMsViGc7DvwW8NK8YN8DvDf/s3038F68YH8NvA6wCyCuOA78FvDSvGDfA7w3/zt8N/BevGB/DbwOsCvgOPBbwEvzgn0P8N787/LdwHvxgv018DICvhr4KF6w7wHem/+dvht4L16wrxFwETjO8/c9wHvzv9t3A+/F87crwDx/3wO8N/83fDfwXjwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+feNEcBz4LeGvgwfzXuBX4buBzeNGZ54UA8/yJF81XAx/Ff4+vAT6aF415Xggwz5940VwEjvPf41bgIbxozPNCgHn+xItmFzjGf49nAA/mRWOeFwLM8ydeNJ8NfBb/PT4H+GxeNOZ5IcA8f+JF99XAWwMP4r/GM4DvBj6bF515Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjn77uB9+H/hu8C3pvnhYBd4BjP33cD78P/bt8FvDfP3zMEfDXwUbxg3w28D/87fRfw3rxgnyPgOPDVwHvxgn038D787/JdwHvzgn0P8N7i2b4beC9esO8G3of/Hb4LeG9esO8B3htAPKfvBt6LF+y7gffhf7bvAt6bF+x7gPfmCsTz+m7gvXjBvht4H/5n+i7gvXnBvgd4b54N8fx9N/BevGDfDbwP/7N8F/DevGDfA7w3zwnxgn038F68YN8NvA//M3wX8N68YN8DvDfPC/HCfTfwXrxg3w28D/+9vgt4b16w7wHem+cP8S/7buC9eMG+G3gf/nt8F/DevGDfA7w3LxjiRfPdwHvxgn038D781/ou4L15wb4HeG9eOMSL7ruB9+IF+27gffiv8V3Ae/OCfQ/w3vzLEP863w28Fy/YdwPvw3+u7wLemxfse4D35kWD+Nf7buC9eMG+G3gf/nN8F/DevGDfA7w3LzrEv813A+/F/yzfA7w3/zqIf7vvBt6L/xm+B3hv/vUQ/z7fDbwX/72+B3hv/m0Q/37fDbwX/z2+B3hv/u0Q/zG+G3gv/mt9D/De/Psg/uN8NfDewDH+cz0D+G7gs/n34x8Bi8LZCFhzHA8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBlocked;
impl IconShape for GoBlocked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.467.22a.75.75 0 01.53-.22h6.006a.75.75 0 01.53.22l4.247 4.247c.141.14.22.331.22.53v6.006a.75.75 0 01-.22.53l-4.247 4.247a.75.75 0 01-.53.22H4.997a.75.75 0 01-.53-.22L.22 11.533a.75.75 0 01-.22-.53V4.997a.75.75 0 01.22-.53L4.467.22zm.84 1.28L1.5 5.308v5.384L5.308 14.5h5.384l3.808-3.808V5.308L10.692 1.5H5.308zM4 7.75A.75.75 0 014.75 7h6.5a.75.75 0 010 1.5h-6.5A.75.75 0 014 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD/UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4t3kw8FXASwMP5j/WbwO/DfwM8Nf850L86z0Y+CvgOP/5bgU+G/ge/nMg/vV+Gngr/mv9NfA2wK38x0L8610EjvNfbxd4G+C3+Y+D+Ncz/73eBvhp/mMg/vXMf69d4HWAv+bfD/GvZ/77/TbwOvz7If71zP8M7wN8N/8+iH8988KJf7uXBj4aeC/+ZbcCD+HfB/GvZ1448e/30sBvA8d44V4G+Gv+7RD/euaFE/8xXhr4K164rwE+mn87xL+eeeHEf5zvBt6LF+x3gNfm3w7xr2deOPEf56WBv+IFuxV4CP92iH8988KJ/1jmhRP/doh/PfPCif9Y5oUT/3aIfz3zwon/OC8N/BUv2CXgOP92iH8988KJ/zjfDbwXL9jvAK/Nvx3iX8+8cOI/xksDf8UL9zXAR/Nvh/jXMy+c+Pd7aeC3gOO8cC8D/DX/doh/PfPCiX+7lwY+Cnhv/mXPAB7Mvw/iX8/8z/A+wHfz74P41zP//X4HeG3+/RD/eua/318DbwPcyr8P4l/P/M+wC7wN8Nv82yH+9cz/HLvA6wB/zb8N4l/P/M+yC7wMcCv/eoh/PfM/z28Dr8O/HuJfz/zP9D7Ad/Ovg/jXMy+c+I/z0sBHA+/Fv+xW4CH86yD+9cwLJ/7jvTTw28AxXriXAf6aFx3iX8+8cOI/x0sDf8UL9znAZ/OiQ/zrmRdO/Of5buC9eMF+B3htXnSIfz3zwon/PC8N/BUv2K3AQ3jRIf71zAsn/nOZF0686BD/euaFE/+5zAsnXnSIfz3zwon/PC8N/BUv2CXgOC86xL+eeeHEf57vBt6LF+x3gNfmRYf41zMvnPjP8dLAX/HCfQ7w2bzoEP965oUT//FeGvgt4Dgv3MsAf82LDvGvZ1448R/npYGPAt6bf9kzgAfzr4P41zP/M70P8N386yD+9cz/PL8DvDb/eoh/PfM/yyXgtYG/5l8P8a9n/md5HeC3+bdB/OvtAsf473cJeG3gr/m3Q/zr/TTwVvz3+h3gvYFb+fdB/Os9GPhr4Bj/9Z4BfDbw3fzHQPzbPBj4auClgQfxn+cZwK3AbwM/Dfw1/7EQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ed8KoVB9UNzlQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBold;
impl IconShape for GoBold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4 2a1 1 0 00-1 1v10a1 1 0 001 1h5.5a3.5 3.5 0 001.852-6.47A3.5 3.5 0 008.5 2H4zm4.5 5a1.5 1.5 0 100-3H5v3h3.5zM5 9v3h4.5a1.5 1.5 0 000-3H5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAETklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/bwg4DnwW8N7Acf7j/Dbw18BvAz/Dv81bAa8NvDTw2vzH2QW+G/gcAb8FvDb/uXaBrwa+BtjlhTsOfBTw0cBx/nP9tgDzX2cXeB/gp3n+3hr4LuA4/zUQYP7rfTfwPjyn7wLem/9aCDD/Pb4beB+u+C7gvfmvhwDz3+e7ueK9+e+BAPP8iX+7lwZeGnhv4LX49/kd4LuBvwb+mn8787wQYJ4/8R/jtYGvBl6Kf51nAO8N/Db/MczzQoB5/sR/nOPAbwMvxYvmb4DXBnb5j2OeFwLM8yf+Yx0Hfht4KV64ZwAvDezyH8s8LwSY50/8x3tt4Ld44V4H+G3+45nnhQDz/In/HL8NvBbP3+8Ar81/DvO8EGCeP/Gf472B7+L5ex/gu/nPYZ4XAszzJ/5zvDTwVzx/rwP8Nv85zPNCgHn+xH8e8/yJ/zzmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY50/85zHPn/jPY54XAszzJ/7zmOdP/OcxzwsB5vkT/3nM8yf+85jnhQDz/In/POb5E/95zPNCgHn+xH8e8/yJ/zzmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY50/85zHPn/jPY54XAszzJ/7zmOdP/OcxzwsB5vkT/3nM8yf+85jnhQDz/In/POb5E/95zPNCgHn+xH8e8/yJ/zzmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY50/85zHPn/jPY54XAszzJ/7zmOdP/OcxzwsB5vkT/3nM8yf+85jnhQDz/In/POb5E/95zPNCgHn+xH8e8/yJ/zzmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY/xy7wF8Dfw38NvAzPJt5/sSzvRXw2sBLAy8NHOc/HgLMf41d4KuBrwEu8vydAD4K+GjgOP/5EGD+a+0Cx3n+doHj/NdBwC5wjP+fEPDZwGfx/9PviCu+Gnhr4EH8/3AJ+G7gs8V/npcGXhp4b+C1+Lf5HeC7gb8G/pr/eIj/Gq8NfDXwUrxo/gb4aOC3+c+F+K9zHPhu4K144X4GeG9gl/98iP963w28F8/f9wDvzX8dxH+P7wbei+f0PcB7818L8d/ns4H35orvBj6b/3qI/98Q/78h/n9D/P+G+P8N8f8b/wg/Iba0wB3SmAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBook;
impl IconShape for GoBook {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE1ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/n+PARwFvDbw0/7X+Gvhp4GuAXf5tEP927w18FXCc/167wMcA382/HuLf5r2B7+J/lvcBvpt/HcS/3nHg6cBx/mfZBR4C7PKiQ/zrfTbwWfzP9DnAZ/OiQ/zr/RXw0vzP9NfAy/CiQ/zrmefvZYC/5r/GSwN/xfMnXnSIfz3z/In/Wub5Ey86xL+eef7Efy3z/IkXHeJfzzx/4r+Wef7Eiw7xr2eeP/Ffyzx/4kWH+Nczz5/4r2WeP/GiQ/zrmedP/Ncyz5940SH+9czzJ/5rmedPvOgQ/3rm+RP/tczzJ150iH898/yJ/1rm+RMvOsS/nnn+xH8t8/yJFx3iX888f+K/lnn+xIsO8a9nnj/xX8s8f+JFh/jXM8+f+K9lnj/xokP865nnT/zXMs+feNEh/vXM8yf+a5nnT7zoEP965vkT/7XM8ydedIh/PfP8if9a5vkTLzrEv555/sR/LfP8iRcd4l/PPH/iv5Z5/sSLDvGvZ54/8V/LPH/iRYf41zPPn/ivZZ4/8aJD/OuZ50/81zLPn3jRIf71zPMn/muZ50+86BD/eub5E/+1zPMnXnSIfz3z/In/Wub5Ey86xL+eef7Efy3z/IkXHeJfzzx/4r+Wef7Eiw7xr2eeP/Ffyzx/4kWH+Nczz5/4r2WeP/GiQ/zrmedPvOiOAx8FfDRXfDXwNcAuLzrz/IkXHeJfzzx/4kVzHPgt4KV5Tn8NvA6wy4vGPH/iRYf41zPPn/iXvTTwW8Bxnr9d4HWAv+ZfZp4/8aJD/OuZ50+8cO8NfBf/sl3gY4Dv5oUzz5940SH+9czzJ16wrwI+mn+drwY+hhfMPH/iRYf41zPPn3hex4HvAt6af5ufBt4H2OV5medPvOgQ/3rm+RPP6aWB7wJemn+fvwbeBriV52SeP/GiQ/zrmedPPNtrAz8FHOcFuwS8N1d8N3CMF2wXeBvgt3k28/yJFx3iX888f+KK9wa+ixfuGcBbA3/NFS8N/DTwIF649wG+myvM8ydedIh/PfP8Cfgu4L154X4HeGtgl+d0HPhp4LV44b4beB/APH/iRYf41zPP318DL80L9z3Ae/PCfTfwXrxwvw28Ns+feNEh/vXMv837AN/Ni+a9ge/i30a86BD/euZf5xLw1sBv86/z2sBPA8f41xEvOsS/nnnR/Q3w1sCt/Ns8GPhp4KV40YkXHeJfz7xofgZ4b2CXf5/jwHcDb8WLRrzoEP965l/2NcBH8x/rq4GP4l8mXnSIfz3zgl0CPhr4bv5zvDfw1cAxXjDxokP86+0Cx3hel4DXBv6a/1wvDfw2cIzndQk4zosO8a/31cBH8Zz+BnhtYJf/GseB3wZeiuf0NcBH86JD/Nt8NfDeXPHVwFcDu/zXOg58NPDRXPHdwEfzr4P4/w3x/xvi/zfE/2/8Izl5rkGuAiPGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBookmark;
impl IconShape for GoBookmark {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.75 2.5a.25.25 0 00-.25.25v9.91l3.023-2.489a.75.75 0 01.954 0l3.023 2.49V2.75a.25.25 0 00-.25-.25h-6.5zM3 2.75C3 1.784 3.784 1 4.75 1h6.5c.966 0 1.75.784 1.75 1.75v11.5a.75.75 0 01-1.227.579L8 11.722l-3.773 3.107A.75.75 0 013 14.25V2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBookmarkSlash;
impl IconShape for GoBookmarkSlash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.19 1.143a.75.75 0 10-.88 1.214L3 4.305v9.945a.75.75 0 001.206.596L8 11.944l3.794 2.902A.75.75 0 0013 14.25v-2.703l1.81 1.31a.75.75 0 10.88-1.214l-2.994-2.168a1.09 1.09 0 00-.014-.01L4.196 3.32a.712.712 0 00-.014-.01L1.19 1.143zM4.5 5.39v7.341l3.044-2.328a.75.75 0 01.912 0l3.044 2.328V10.46l-7-5.07zM5.865 1a.75.75 0 000 1.5h5.385a.25.25 0 01.25.25v3.624a.75.75 0 001.5 0V2.75A1.75 1.75 0 0011.25 1H5.865z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADsElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdRz4KOCtgZfmX+evgZ8GvgbY5T8H4j/PewNfBRzn32cX+Bjgu/mPh/jP8d7Ad/Ef632A7+Y/FuI/3nHg6cBx/mPtAg8BdvmPg/iP99nAZ/Gf43OAz+Y/DuI/3l8BL81/jr8GXob/OIj/eOb5exngr3nRvDTwVzx/4j8O4j+eef7Ev455/sR/HMR/PPP8iX8d8/yJ/ziI/3jm+RP/Oub5E/9xEP/xzPMn/nXM8yf+4yD+45nnT/zrmOdP/MdBPKfjwEcBbw28NP+3/DXw08DXALtcgXi29wa+CjjO/227wMcA3w0grnhv4Lv4/+V9gO8WcBx4OnCc/192gYcI+Gzgs/j/6XME/BXw0vz/9NcCzPP3MsBf83/DSwN/xfNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vl7GeCv+b/hpYG/4nkhwDx/HwN8Nf83fDTwVTwvBJjn71bgIfzf8HTgwTwvBJgX7GOAr+Z/t48GvornDwHmhXsZ4K/53+mlgb/iBUOA+Zd9NPA1/O/yUcBX88IhwLxobgW+Gvgd4K/5n+mlgdcCPhp4MP8yBJj/vxBg/v9CgPn/CwHm+fsd4LX4v+F3gNfieSHAPH8C3hv4bOBB/O/0DOCzge8GzPNCgHn+xLO9NPDWwGsDLw0c43+mS8BfA78N/DTw1zybeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCev5cB/pr/G14a+CueFwL+Gngp/n/6GwGfDXwW/z99joDjwK3AMf5/uQQ8WFzx3sB38f/L2wA/LZ7tvYGvBo7xf9sl4L2BnwYQz+k48NHAWwMvxf8tfwP8NPDVwC5XIP5/Q/z/hvj/DfH/G/8IMNyS3BSheeIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBriefcase;
impl IconShape for GoBriefcase {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.75 0A1.75 1.75 0 005 1.75V3H1.75A1.75 1.75 0 000 4.75v8.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H11V1.75A1.75 1.75 0 009.25 0h-2.5zM9.5 3V1.75a.25.25 0 00-.25-.25h-2.5a.25.25 0 00-.25.25V3h3zM5 4.5H1.75a.25.25 0 00-.25.25V6a2 2 0 002 2h9a2 2 0 002-2V4.75a.25.25 0 00-.25-.25H5zm-1.5 5a3.484 3.484 0 01-2-.627v4.377c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V8.873a3.484 3.484 0 01-2 .627h-9z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBroadcast;
impl IconShape for GoBroadcast {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.267 1.457c.3.286.312.76.026 1.06A6.475 6.475 0 001.5 7a6.472 6.472 0 001.793 4.483.75.75 0 01-1.086 1.034 8.89 8.89 0 01-.276-.304l.569-.49-.569.49A7.971 7.971 0 010 7c0-2.139.84-4.083 2.207-5.517a.75.75 0 011.06-.026zm9.466 0a.75.75 0 011.06.026A7.975 7.975 0 0116 7c0 2.139-.84 4.083-2.207 5.517a.75.75 0 11-1.086-1.034A6.475 6.475 0 0014.5 7a6.475 6.475 0 00-1.793-4.483.75.75 0 01.026-1.06zM8.75 8.582a1.75 1.75 0 10-1.5 0v5.668a.75.75 0 001.5 0V8.582zM5.331 4.736a.75.75 0 10-1.143-.972A4.983 4.983 0 003 7c0 1.227.443 2.352 1.177 3.222a.75.75 0 001.146-.967A3.483 3.483 0 014.5 7c0-.864.312-1.654.831-2.264zm6.492-.958a.75.75 0 00-1.146.967c.514.61.823 1.395.823 2.255 0 .86-.31 1.646-.823 2.255a.75.75 0 101.146.967A4.983 4.983 0 0013 7a4.983 4.983 0 00-1.177-3.222z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADHklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+GteMPP8iRfOPH/iP89LA3/F80KAef7EC2eeP/HCmedP/OcyzwsB5vkTL5x5/sQLZ54/8Z/LPC8EmOdPvHDm+RMvnHn+xH8u87wQYJ4/8cKZ50+8cOb5E/+5zPNCgHn+xAtnnj/xwpnnT/znMs8LAeb5Ey+cef7EC2eeP/GfyzwvBJjnT7xw5vkTL5x5/sR/LvO8EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD5/wsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz9/LAH/N/w0vDfwVzwsBfw28FP8//Y2AzwY+i/+fPkfAceBW4Bj/v1wCHiyueG/gu/j/5W2AnxbP9t7AVwPH+L/tEvDewE8DiOd0HPho4K2Bl+L/lr8Bfhr4amCXKxD/vyH+f0P8/4b4/41/BAVegNTCEIcXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBrowser;
impl IconShape for GoBrowser {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25V4.5h2v-2H1.75zM5 2.5v2h2v-2H5zm3.5 0v2h6V2.75a.25.25 0 00-.25-.25H8.5zm6 3.5h-13v7.25c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V6z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoBug;
impl IconShape for GoBug {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.72.22a.75.75 0 011.06 0l1 .999a3.492 3.492 0 012.441 0l.999-1a.75.75 0 111.06 1.061l-.775.776c.616.63.995 1.493.995 2.444v.327c0 .1-.009.197-.025.292.408.14.764.392 1.029.722l1.968-.787a.75.75 0 01.556 1.392L13 7.258V9h2.25a.75.75 0 010 1.5H13v.5c0 .409-.049.806-.141 1.186l2.17.868a.75.75 0 01-.557 1.392l-2.184-.873A4.997 4.997 0 018 16a4.997 4.997 0 01-4.288-2.427l-2.183.873a.75.75 0 01-.558-1.392l2.17-.868A5.013 5.013 0 013 11v-.5H.75a.75.75 0 010-1.5H3V7.258L.971 6.446a.75.75 0 01.558-1.392l1.967.787c.265-.33.62-.583 1.03-.722a1.684 1.684 0 01-.026-.292V4.5c0-.951.38-1.814.995-2.444L4.72 1.28a.75.75 0 010-1.06zM6.173 5h3.654A.173.173 0 0010 4.827V4.5a2 2 0 10-4 0v.327c0 .096.077.173.173.173zM5.25 6.5a.75.75 0 00-.75.75V11a3.5 3.5 0 107 0V7.25a.75.75 0 00-.75-.75h-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj3Ow58FvDWXPHdwOfwH+848FnAWwMPBn4beBtgl387xL/fVwMfxXP6GuCj+Y/11cBH8Zy+Bvho/u0Q/34XgeM8p1uBh/Af6yJwnOe0C5zg3w7x72eeP/Efyzx/4t8O8e9nnj/xH8s8f+LfDvHvZ54/8R/LPH/i3w7x72eeP/Efyzx/4t8O8e9nnj/xH8s8f+LfDvHCHQc+Cnhr4KX53+WvgZ8GvgbY5flDvGDvDXwVcJz/3XaBjwG+m+eFeP7eG/gu/m95H+C7eU6I53UceDpwnP9bdoGHALs8G+J5fTbwWfzf9DnAZ/NsiOf1V8BL83/TXwMvw7Mhnpd5/l4G+Gv+d3hp4K94/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ4/8b+Lef7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m/zbxbIjnZf5vE8+GeF7m/zbxbIjnZf5vE8+GeF7m/zbxbIjnZf5vE8+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ4/8b+Lef7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ4/8b+Lef7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef5eBvhr/nd4aeCveP7EsyGe118DL8X/TX8DvDTPhnhenw18Fv83fQ7w2Twb4nkdB24FjvF/yyXgwcAuz4Z4/t4b+C7+b3kb4Kd5TogX7L2BrwaO8b/bJeC9gZ/meSFeuOPARwNvDbwU/7v8DfDTwFcDuzx//CPyu4tBovjmrgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCalendar;
impl IconShape for GoCalendar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.75 0a.75.75 0 01.75.75V2h5V.75a.75.75 0 011.5 0V2h1.25c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 16H2.75A1.75 1.75 0 011 14.25V3.75C1 2.784 1.784 2 2.75 2H4V.75A.75.75 0 014.75 0zm0 3.5h8.5a.25.25 0 01.25.25V6h-11V3.75a.25.25 0 01.25-.25h2zm-2.25 4v6.75c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V7.5h-11z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADhElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+L/ls4CP5oqvBj6HFw7xf8d3Ae/Nc/oc4LN5wRD/N3wX8N48r1uBh/CCIf73+y7gvXn+ngE8mBcM8b/bdwHvzQv2OcBn84Ih/vf6LuC9ecG+B3hvXjjE/07fBbw3L9j3AO/Nvwzxv893Ae/NC/Y9wHvzokH87/JdwHvzgn0P8N686BD/e3wX8N68YN8DvDf/Ooj/Hb4LeG9esO8B3pt/PcT/fN8FvDcv2PcA782/DeJ/tu8C3psX7HuA9+bfDvE/13cB780L9j3Ae/Pvg/if6buA9+YF+x7gvfn3Q/zP813Ae/OCfQ/w3vzHQLxoPgp4b674aeBz+M/xXcB784J9D/De/MdB/Ms+GvgqntN3A+/Df6zvAt6bF+x7gPfmPxbiX/ZXwEvzvL4beB/+Y3wX8N68YN8DvDf/8RD/sr8GXorn77uB9+Hf57uA9+YF+x7gvfnPgfiXfTbwWbxg3w28D/823wW8Ny/Y9wDvzX8exIvmu4H34gX7buB9+Nf5LuC9ecG+B3hv/nMhXnTfDbwXL9h3A+/Di+a7gPfmBfse4L35z4f41/lu4L14wb4beB9euO8C3psX7HuA9+a/BuJf77uB9+IF+27gfXj+vgt4b16w7wHem/86iH+b7wbeixfsu4H34Tl9F/DevGDfA7w3/7UQ/3bfDbwXL9h3A+/DFd8FvDcv2PcA781/PcS/z3cD78UL9t1c8d68YN8DvDf/PRD/ft8NvBf/Nt8DvDf/fRD/Mb4beC/+db4HeG/+eyH+43w38F68aL4HeG/++yH+Y3038F68cN8DvDf/MyD+43038F48f98DvDf/cyD+c3w38F48p+8B3pv/WRD/eT4beG+u+G7gs/mfB/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj7tMXUFpI9mTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheck;
impl IconShape for GoCheck {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheckCircle;
impl IconShape for GoCheckCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM0 8a8 8 0 1116 0A8 8 0 010 8zm11.78-1.72a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCheckCircleFill;
impl IconShape for GoCheckCircleFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm3.78-9.72a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFtklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjhjgMfBbw18NL8z/LXwE8DXwPs8m+DeMHeG/gq4Dj/s+0CHwN8N/96iOfvvYHv4n+X9wG+m38dxPM6DjwdOM7/LrvAQ4BdXnSI5/XZwGfxv9PnAJ/Niw7xvP4KeGn+d/pr4GV40SGel3n+Xgb4a/5neGngr3j+xIsO8bzM8yf+ZzHPn3jRIZ6Xef7E/yzm+RMvOsTzMs+f+J/FPH/iRYd4Xub5E/+zmOdPvOgQz8s8f+J/FvP8iRcd4nmZ50/8z2KeP/GiQzwv8/yJ/1nM8ydedIjnZZ4/8T+Lef7Eiw7xvMzzJ/5nMc+feNEhnpd5/sTzdxz4LOCtgQfzH+NW4LuBz+EFM8+feNEhnpd5/sTz99XAR/Gf42uAj+b5M8+feNEhnpd5/sTzdxE4zn+OW4GH8PyZ50+86BDPyzx/4vnbBY7xn+MZwIN5/szzJ150iOdlnj/x/H028Fn85/gc4LN5/szzJ150iOdlnj/xgn018NbAg/iP8Qzgu4HP5gUzz5940SGel3n+xP8s5vkTLzrE8zLPn/ifxTx/4kWHeF7m+RP/s5jnT7zoEM/LPH/ifxbz/IkXHeJ5medP/M9inj/xokM8L/P8if9ZzPMnXnSI52WeP/H8HQc+C3hr4MG8cLcC3w18Dv9+5vkTLzrE8zLPn3j+vhr4KP51vgb4aP59zPMnXnSI52WeP/H8XQSO869zK/AQ/n3M8ydedIjnZZ4/8fztAsf413kG8GD+fczzJ150iOdlnj/x/H028Fn863wO8Nn8+5jnT7zoEM/LPH/iBftq4K2BB/HCPQP4buCz+fczz5940SGel3n+xP8s5vkTLzrE8zLPn/ifxTx/4kWHeF7m+RP/s5jnT7zoEM/LPH/ifxbz/IkXHeJ5medP/M9yK/AgntMzgAfzokM8L/P8if9ZPhv4LJ7T5wCfzYsO8bzM8yf+5/ls4K254qeBz+aF+yzgo7niq4HPEc/LPH/if6/jwG8BL81z+m7xvMzzJ/53Og78FvDSPC/E8zLPn/jf5zjwW8BL8/whnpd5/sT/LseB3wJemhfse8TzMs+f+N/jOPBbwEvzgn0P8N7ieZnnT/zvcBz4LeClecG+B3hvAPG8zPMn/uc7DvwW8NK8YN8DvDdXIJ6Xef7E/2zHgd8CXpoX7HuA9+bZEM/LPH/if67jwG8BL80L9j3Ae/OcEM/LPH/iP89HAe/NFT8NfA4vuuPAbwEvzQv2PcB787wQz8s8f+I/x0cDX8Vz+m7gffiXvTTwU8CDecG+B3hvnj/E8zLPn/jP8VfAS/O8vht4H16wlwZ+CzjOC/Y9wHvzgiGel3n+Xgb4a/7j/TXwUjx/3w28D8/rpYHfAo7zgn0P8N68cIjn9dfAS/Ef56+BzwF+mufvs4HP4gX7buB9eLaXBn4LOM4L9jXAR/MvQzyvzwY+i/94rwP8Ns/fdwPvxQv23cD7AC8N/BZwnBfsfYDv5kWDeF7HgVuBY/zH+hrgo3nBvht4L16wnwZeGzjOC/Y+wHfzokM8f+8NfBf/sb4G+GheuO8G3ot/m/cBvpt/HcQL9t7AVwPH+I/xOsBv88IdB34beCn+dd4H+G7+9RAv3HHgo4G3Bl6Kf5u/AT4b+GleNMeB3wZeihfN+wDfzb8N4n+m48BvAy/FC/c+wHfzb4f4n+s48NvAS/H8vQ/w3fz7IP5nOw78NvBSPKf3Ab6bfz/E/w4fDbw1sAt8NvDX/MfgHwHND81+rnz1/wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChecklist;
impl IconShape for GoChecklist {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 1.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v7.736a.75.75 0 101.5 0V1.75A1.75 1.75 0 0011.25 0h-8.5A1.75 1.75 0 001 1.75v11.5c0 .966.784 1.75 1.75 1.75h3.17a.75.75 0 000-1.5H2.75a.25.25 0 01-.25-.25V1.75zM4.75 4a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM4 7.75A.75.75 0 014.75 7h2a.75.75 0 010 1.5h-2A.75.75 0 014 7.75zm11.774 3.537a.75.75 0 00-1.048-1.074L10.7 14.145 9.281 12.72a.75.75 0 00-1.062 1.058l1.943 1.95a.75.75 0 001.055.008l4.557-4.45z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvi3+Sjgvbnip4HP4b/HZwEfzRVfDXwO/zqIf72PBr6K5/TdwPvwX+u7gPfmOX0O8Nm86BD/en8FvDTP67uB9+G/xncB783zuhV4CC86xL/eXwMvxfP33cD78J/ru4D35vl7BvBgXnSIf73PBj6LF+y7gffhP8d3Ae/NC/Y5wGfzokP823w38F68YN8NvA//sb4LeG9esO8B3pt/HcS/3XcD78UL9t3A+/Af47uA9+YF+x7gvfnXQ/z7fDfwXrxg3w28D/8+3wW8Ny/Y9wDvzb8N4t/vu4H34gX7buB9+Lf5LuC9ecG+B3hv/u0Q/zG+G3gvXrDvBt6Hf53vAt6bF+x7gPfm3wfxH+e7gffiBftu4H140XwX8N68YN8DvDf/foj/WN8NvBcv2HcD78ML913Ae/OCfQ/w3vzHQPzH+27gvXjBvht4H56/7wLemxfse4D35j8O4j/HdwPvxQv23cD78Jy+C3hvXrDvAd6b/1iI/zzfDbwXL9h3A+/DFd8FvDcv2PcA781/PMR/ru8G3osX7Lu54r15wb4HeG/+cyD+83038F7823wP8N7850H81/hu4L341/ke4L35z4X4r/PdwHvxovke4L35z4f4r/XdwHvxwn0P8N7810D81/tu4L14/r4HeG/+6yD+e3w38F48p+8B3pv/Woj/Pp8NvDdXfDfw2fzXQ/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwKHKU1BGFBPdwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronDown;
impl IconShape for GoChevronDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M12.78 6.22a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06 0L3.22 7.28a.75.75 0 011.06-1.06L8 9.94l3.72-3.72a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/1k+C3hrrvhu4Gv4z4X4n+O7gPfmOX0M8NX850H8z/BdwHvzvP4aeBn+8yD++30X8N48f38DvDT/eRD/vb4LeG9esM8BPpv/PIj/Pt8FvDcv2PcA781/LsR/j+8C3psX7HuA9+Y/H+K/3ncB780L9j3Ae/NfA/Ff67uA9+YF+x7gvfmvg/iv813Ae/OCfQ/w3vzXQvzX+C7gvXnBvgd4b/7rIf7zfRfw3rxg3wO8N/89EP+5vgt4b16w7wHem/8+iP883wW8Ny/Y9wDvzX8vxH+O7wLemxfse4D35r8f4j/edwHvzQv2PcB78z8D4j/WdwHvzQv2PcB78z8H4j/OdwHvzQv2PcB78z8L4j/GZwOfxQv2PcB78z8P4j/GReA4z9/3AO/N/0yI/xi7wDGev+8G3of/mRD/MT4b+CxesO8G3of/eRD/cb4beC9esO8G3of/WRD/sb4beC9esO8G3of/ORD/8b4beC9esO8G3of/GRD/Ob4beC9esO8G3of/foj/PN8NvBcv2HcD78N/L8R/ru8G3osX7LuB9+G/D+I/33cD78UL9t3A+/DfA/Ff47uB9+IF+27gffivh/iv893Ae/GCfTfwPvzXQvzX+m7gvXjBvht4H/7rIP7rfTfwXrxg3w28D/81EP89vht4L16w7wbeh/98iP8+3w28Fy/YdwPvw38uxH+v7wbeixfsc4DP5j8P4r/fdwPvxfN3K/AQ/vMg/mf4buC9eF7PAB7Mfx7E/xzfDbwXz+lzgM/mPw/if5bPBt6bK74b+Gz+cyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wjZAFBBi30ByAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronLeft;
impl IconShape for GoChevronLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.78 12.78a.75.75 0 01-1.06 0L4.47 8.53a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 1.06L6.06 8l3.72 3.72a.75.75 0 010 1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADPElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/xqfBXw0V3w18Dn8z4D4z/fZwGfxnL4beB/++yH+810EjvO8vht4H/57If7z7QLHeP6+G3gf/vsg/vN9NvBZvGDfDbwP/z0Q/zW+G3gvXrDvBt6H/3qI/zrfDbwXL9h3A+/Dfy3Ef63vBt6LF+y7gffhvw7iv953A+/FC/bdwPvwXwPx3+O7gffiBftu4H34z4f47/PdwHvxgn038D7850L89/pu4L14wb4beB/+8yD++3038F68YN8NvA//ORD/M3w38F68YN8NvA//8RD/c3w38F68YN8NvA//sRD/s3w38F68YN8NvA//cRD/83w38F68YN8NvA//MRD/M3038F68YN8NvA//foj/ub4beC9esM8BPpt/H8T/bN8NvBfP363AQ/j3QfzP9l3Ae/P8PQN4MP8+iP+5vgt4b16wzwE+m38fxP9M3wW8Ny/Y9wDvzb8f4n+e7wLemxfse4D35j8G4n+W7wLemxfse4D35j8O4n+O7wLemxfse4D35j8W4n+G7wLemxfse4D35j8e4r/fdwHvzQv2PcB7858D8d/ru4D35gX7HuC9+c+D+O/zXcB784J9D/De/OdC/Pf4LuC9ecG+B3hv/vMh/ut9F/DevGDfA7w3/zUQ/7W+C3hvXrDvAd6b/zqI/zrfBbw3L9j3AO/Nfy3Ef43vAt6bF+x7gPfmvx7iP99nA5/FC/Y9wHvz3wPxn++vgJfm+fse4L3574P4z/fXwEvxvL4HeG/+eyH+83008FU8p+8B3pv/foj/Gh8NvDdX/DTw2fzPgPj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8Izk1UEHA/uJJAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronRight;
impl IconShape for GoChevronRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.22 3.22a.75.75 0 011.06 0l4.25 4.25a.75.75 0 010 1.06l-4.25 4.25a.75.75 0 01-1.06-1.06L9.94 8 6.22 4.28a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8d/ns4CP5oqvBj6H/3qI/x7fBbw3z+m7gffhvxbiv953Ae/N8/fdwPvwXwfxX+u7gPfmhftu4H34r4H4r/NdwHvzovlu4H34z4f4r/FdwHvzr/PdwPvwnwvxn++7gPfm3+a7gffhPw/iP9d3Ae/NC/Y9XPFevGDfDbwP/zkQ/3m+C3hvXrDvAd6bK74beC9esO8G3of/eIj/HN8FvDcv2PcA781z+m7gvXjBvht4H/5jIf7jfRfw3rxg3wO8N8/fdwPvxQv23cD78B8H8R/ru4D35gX7HuC9eeG+G3gvXrDvBt6H/xiI/zjfBbw3L9j3AO/Ni+a7gffiBftu4H3490P8x/gu4L15wb4HeG/+db4beC9esO8G3od/H8S/33cB780L9j3Ae/Nv893Ae/GCfTfwPvzbIf59vgt4b16w7wHem3+f7wbeixfsu4H34d8G8W/3XcB784J9D/De/Mf4buC9eMG+G3gf/vUQ/zbfBbw3L9j3AO/Nf6zvBt6LF+y7gffhXwfxr/fZwGfxgn0P8N785/hu4L14wT4H+GxedIh/vYvAcZ6/7wHem/9c3w28F8/fXwMvw4sO8a+3CxzjeX0P8N781/hu4L14Xn8DvDQvOsS/3mcDn8Vz+h7gvfmv9d3Ae/GcPgb4al50iH+bzwbemyu+G/hs/nt8NvDWXPHdwFfzr4P4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Asy1TUG5YPFGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoChevronUp;
impl IconShape for GoChevronUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.22 9.78a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0l4.25 4.25a.75.75 0 01-1.06 1.06L8 6.06 4.28 9.78a.75.75 0 01-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCircle;
impl IconShape for GoCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCircleSlash;
impl IconShape for GoCircleSlash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 0110.535-5.096l-9.131 9.131A6.472 6.472 0 011.5 8zm2.465 5.096a6.5 6.5 0 009.131-9.131l-9.131 9.131zM8 0a8 8 0 100 16A8 8 0 008 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoClock;
impl IconShape for GoClock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm.5 4.75a.75.75 0 00-1.5 0v3.5a.75.75 0 00.471.696l2.5 1a.75.75 0 00.557-1.392L8.5 7.742V4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/XSwFsBr80VDwYezLP9NrAL/DXw28Dv8F8L8R/vtYH3Al4beDD/OrvATwNfA/w1//kQ/3EeDHwW8N78x/ht4GOAv+Y/D+Lf7zjwWcBH85/jq4HPAXb5j4f49zkO/Bbw0vzn+mvgfYC/5j8W4t/upYHfAo7zX2MXeB3gr/mPg/i3eWngr3jR/A7w08BfA7vAX3PFceClgZcGXht4K/5lu8DrAH/NfwzEv95x4OnAcV6wS8BXA18N7PKiOQ58NPDRwDFesF3gdYC/5t8P8a9zHPgt4KV5wX4GeG9gl3+b48BXA+/FC/bXwOsAu/z7IP51vhr4KF6w9wG+m/8Y7w18Fy/Y1wAfzb8P4kX3YODpvGBvA/w0/7FeG/gtXrCXAf6afzvEi+67gffi+Xsf4Lv5z/HewHfx/P028Dr82yFeNK8N/BbP388Ab81/rp8G3orn77eBvwb+Gvgd4FZedIgXzXcD78XzugQ8GNjlP9dx4FbgGP+y3wa+G/ge/mWIF83TgQfzvD4H+Gz+a3w28Fm86G4F3gf4bV4wxL/spYG/4vk7AezyX+M4cJF/va8GPgfY5Xkh/mWfDXwWz+t3gNfmv9ZXAx/Fv95fA68D7PKcEP+y3wZei+f1McBX81/vq4H3Bo7xr/PXwOsAuzwb4l/228Br8bxeB/ht/nu9NHAceGvgrYEH8cL9NfAyPBviX/Z04ME8r5cB/pr/Wd4b+GzgQbxgXwN8NFcg/mXm+RP/Mx0Hvhp4L16wtwF+GkC8cO8FfDfPn/if7bOBz+L5uxV4CIB4/l4a+C7gpXnBTgC7/M/23cB78fy9D/Dd4nm9N/BVwHFeuJcB/pr/2Y4Dfw08iOf118DLiOf03sB38S97BvBg/nd4b+C7eP4eIp7tpYHfAo7zwl0CXhv4a/73uBV4EM/rfcSz/RXw0rxgvwN8N/DTwC7/u3w18FE8r58RV7w38F08f5eAjwa+m/+93hr4KZ7X74grng48mOd1CXht4K/53+2lgb/ied0q4LWB3+L5exvgp/m/wTwvBHw18FE8r78BXpr/O8zzQsBvA6/F83of4Lv5v+Glgb/ief2NgIvAcZ7XQ4Bb+b/hrYGf4nn9jgDz/In/O74a+Cie188IMM+f+L/j6cCDeV7vI8A8fy8D/DX/+7038F08fycE/DbwWjyvjwG+mv/djgN/BTyY5/U7wGsL+Grgo3hevw28Dv+7fRfw3jx/7wN8t4DXBn6L5+9jgK/mf6fvAt6b5+93gNcGEFfcCjyI57ULvA7w1/zvcRz4KuC9ecFeB/htAHHFRwNfxfO3C3w28DX8z/dewGcDD+YF+x7gvbkC8Wx/DbwUL9hvAz8N/A7w1/zP8NLAg4DXBt4aeDAv3N8AL82zIZ7tpYHfBo7xf9PfAK8N7PJsiOf01sBP8X/P3wCvDezynBDP662B7waO8X/D9wDvzfOHeP5eGvhu4KX43+t3gM8GfpsXDPHCvTfw2cCD+N/jd4DvBr6bfxniRfPSwHsDLw0cB16K/xn+BtgFdoGfBn4a2OVFh/j/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AiIkyb6RhxnoAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCloud;
impl IconShape for GoCloud {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 7.25A5.225 5.225 0 017.25 2a5.222 5.222 0 014.767 3.029A4.472 4.472 0 0116 9.5c0 2.505-1.995 4.5-4.5 4.5h-8A3.475 3.475 0 010 10.5c0-1.41.809-2.614 2.001-3.17L2 7.25zm1.54.482a.75.75 0 01-.556.832c-.86.22-1.484.987-1.484 1.936 0 1.124.876 2 2 2h8c1.676 0 3-1.324 3-3s-1.324-3-3-3a.75.75 0 01-.709-.504A3.72 3.72 0 007.25 3.5C5.16 3.5 3.5 5.16 3.5 7.25a3.276 3.276 0 00.035.436l.004.036.001.008v.002z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCloudOffline;
impl IconShape for GoCloudOffline {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.25 2c-.69 0-1.351.13-1.957.371a.75.75 0 10.554 1.394c.43-.17.903-.265 1.403-.265a3.72 3.72 0 013.541 2.496.75.75 0 00.709.504c1.676 0 3 1.324 3 3a3 3 0 01-.681 1.92.75.75 0 001.156.955A4.496 4.496 0 0016 9.5a4.472 4.472 0 00-3.983-4.471A5.222 5.222 0 007.25 2z",
            }
            path {
                d: "M.72 1.72a.75.75 0 011.06 0l2.311 2.31c.03.025.056.052.08.08l8.531 8.532a.785.785 0 01.035.034l2.043 2.044a.75.75 0 11-1.06 1.06l-1.8-1.799a4.64 4.64 0 01-.42.019h-8A3.475 3.475 0 010 10.5c0-1.41.809-2.614 2.001-3.17a5.218 5.218 0 01.646-2.622L.72 2.78a.75.75 0 010-1.06zM3.5 7.25c0-.505.096-.983.271-1.418L10.44 12.5H3.5c-1.124 0-2-.876-2-2 0-.95.624-1.716 1.484-1.936a.75.75 0 00.557-.833A4.1 4.1 0 013.5 7.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE5ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/1ifBbw1V3w38DX8+x0Hvgt4a674auBj+I+B+I/zXcB785w+Bvhq/n1+GngrntN3A+/Dvx/iP8Z3Ae/N8/pr4GX49zHP33cD78O/D+Lf77uA9+b5+xvgpfn3MS/YdwPvw78d4t/nu4D35gX7HOCz+ff5auCjeMG+G3gf/m0Q/3bfBbw3L9j3AO/Nf4zvBt6LF+y7gffhXw/xb/NdwHvzgn0P8N78x/pu4L14wb4beB/+dRD/et8FvDcv2PcA781/ju8G3osX7LuB9+FFh/jX+S7gvXnBvgd4b/5zfTfwXrxg3w28Dy8axIvuu4D35gX7HuC9+a/x3cB78YJ9N/A+/MsQL5rvAt6bF+x7gPfmv9Z3A+/FC/bdwPvwwiH+Zd8FvDcv2PcA781/j+8G3osX7LuB9+EFQ7xw3wW8Ny/Y9wDvzX+v7wbeixfsu4H34flDvGDfBbw3L9j3AO/N/wzfDbwXL9h3A+/D80I8f98FvDcv2PcA783/LN8NvBcv2HcD78NzQjyv7wLemxfse4D35n+m7wbeixfsu4H34dkQz+m7gPfmBfse4L35n+27gffiBftu4H24AvFs3wW8Ny/Y9wDvzf8O3w28Fy/YdwPvAyCu+Gzgs3jBvgd4b/53+W7gvXjBPgf4bHHFReA4z9/3AO/N/07fDbwXz9+twEPEFbvAMZ6/7wbeh/+dvgt4b56/ZwAPFld8NvBZvGDfDbwP/7t8F/DevGCfA3y2eLbvBt6LF+y7gffhf4fvAt6bF+x7gPcGEM/pu4H34gX7buB9+J/tu4D35gX7HuC9uQLxvL4beC9esO8G3of/mb4LeG9esO8B3ptnQzx/3w28Fy/YdwPvw/8s3wW8Ny/Y9wDvzXNCvGDfDbwXL9h3A+/D/wzfBbw3L9j3AO/N80K8cN8NvBcv2HcD78N/r+8C3psX7HuA9+b5Q/zLvht4L16w7wbeh/8e3wW8Ny/Y9wDvzQuGeNF8N/BevGDfDbwP/7W+C3hvXrDvAd6bFw7xovtu4L14wb4beB/+a3wX8N68YN8DvDf/MsS/zncD78UL9t3A+/Cf67uA9+YF+x7gvXnRIP71vht4L16w7wbeh/8c3wW8Ny/Y9wDvzYsO8W/z3cB78YJ9N/A+/Mf6LuC9ecG+B3hv/nUQ/3bfDbwXL9h3A+/Df4zvAt6bF+x7gPfmXw/x7/PdwHvxgn0N8NH8+3w18FG8YN8DvDf/Noh/v+8G3osXTPz7mBfse4D35t8O8R/ju4H34vkT/z7m+fse4L3590H8x/lu4L14Tj8DvDX/Pj8NvBXP6XuA9+bfD/Ef66uBj+KK7wE+Gtjl3+c48N3AWwGXgO8GPpr/GIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwJcuqBBxkIA4wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCode;
impl IconShape for GoCode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeOfConduct;
impl IconShape for GoCodeOfConduct {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.048 2.241c.964-.709 2.079-1.238 3.325-1.241a4.613 4.613 0 013.282 1.355c.41.408.757.86.996 1.428.238.568.348 1.206.347 1.968 0 2.193-1.505 4.254-3.081 5.862-1.496 1.526-3.213 2.796-4.249 3.563l-.22.163a.75.75 0 01-.895 0l-.221-.163c-1.036-.767-2.753-2.037-4.249-3.563C1.51 10.008.007 7.952.002 5.762a4.614 4.614 0 011.353-3.407C3.123.585 6.223.537 8.048 2.24zm-1.153.983c-.81.78-1.546 1.669-2.166 2.417-.184.222-.358.432-.52.623a.75.75 0 00.04 1.016c.35.35.697.697 1.043 1.047.866.875 2.292.914 3.185.032.264-.26.534-.528.802-.797.694-.694 1.8-.701 2.474-.03L12.92 8.7l.283.284c-.244.334-.515.666-.81.995l-1.384-1.28A.75.75 0 109.99 9.802l1.357 1.252c-.325.31-.656.606-.984.887l-1.48-1.366a.75.75 0 10-1.018 1.102L9.191 12.9c-.433.34-.838.643-1.191.905-1.04-.773-2.537-1.907-3.846-3.242C2.611 8.99 1.502 7.306 1.502 5.75a3.114 3.114 0 01.913-2.335c1.159-1.158 3.23-1.224 4.48-.191zm7.112 4.442c.313-.65.491-1.293.491-1.916v-.001c0-.614-.088-1.045-.23-1.385-.143-.339-.357-.633-.673-.949a3.113 3.113 0 00-2.218-.915c-1.092.003-2.165.627-3.226 1.602-.823.755-1.554 1.637-2.228 2.45l-.127.154.562.566a.756.756 0 001.066.02l.794-.79c1.258-1.258 3.312-1.31 4.594-.032.396.394.792.791 1.173 1.173l.022.023z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFnUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+Lf77OAj+aKrwY+hxfNZwEfzRVfDXwO/37meSHAPH/i3+e7gPfmOX0O8Nm8cJ8NfBbP6buB9+HfxzwvBJjnT/zbfRfw3jyvW4GH8MJdBI7zvL4beB/+7czzQoB5/sS/zXcB783z9wzgwbxwu8Axnr/vBt6HfxvzvBBgnj/xr/ddwHvzgn0O8Nm8cJ8NfBYv2HcD78O/nnleCDDPn/jX+S7gvXnBvgd4b1403w28Fy/YdwPvw7+OeV4IMM+feNF9F/DevGDfA7w3/zrfDbwXL9h3A+/Di848LwSY50+8aL4LeG9esO8B3pt/m+8G3osX7LuB9+FFY54XAszzJ/5l3wW8Ny/Y9wDvzb/PdwPvxQv23cD78C8zzwsB5vkTL9xnA5/FC/Y9wHvzH+O7gffiBfsc4LN54czzQoB5/sQLdxE4zvP3PcB78x/ru4H34vm7FXgIL5x5Xggwz5944XaBYzx/3w28D/+xvgt4b56/ZwAP5oUzzwsB5vkTL9xnA5/FC/bdwPvwH+O7gPfmBfsc4LN54czzQoB5/sS/7LuB9+IF+27gffj3+S7gvXnBvgd4b/5l5nkhwDx/4kXz3cB78YJ9N/A+/Nt8F/DevGDfA7w3LxrzvBBgnj/xovtu4L14wb4beB/+db4LeG9esO8B3psXnXleCDDPn/jX+W7gvXjBvht4H1403wW8Ny/Y9wDvzb+OeV4IMM+f+Nf7buC9eME+B/hsXrjPBj6LF+x7gPfmX888LwSY50/823w38F48f38NvAwv3F8BL83z9z3Ae/NvY54XAszzJ/7tvht4L57X3wAvzQv318BL8by+B3hv/u3M80KAef7Ev893A+/Fc/oY4Kt54T4a+Cqe0/cA782/j3leCDDPn/j3+2zgrbniu4Gv5kXz0cB7c8VPA5/Nv595Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8fy8D/DX/N7w08Fc8LwT8NfBS/NvcCnw38Dn863wW8NHAcf57/Y2AzwY+i3+frwE+mhfNdwHvzf8MnyPgOHArcIx/u1uBh/Av+y7gvfmf4RLwYHHFewPfxb+PeOG+C3hv/ud4G+CnxbO9N/DVwDH+bcQL9l3Ae/M/wyXgvYGfBhDP6Tjw0cBbAy/Fv454/r4LeG/++/0N8NPAVwO7XIH41zPPn3he3wW8Ny/Y9wDvzX8fxL+eef7Ec/ou4L15wb4HeG/+eyH+9czzJ57tu4D35gX7HuC9+e+H+Nczz5+44ruA9+YF+x7gvfmfAfGvZ54/Ad8FvDcv2PcA783/HIh/PfP8fTfw3rxg3wO8N/+zIP71zL/e9wDvzf88iH+93wZeixfd9wDvzf9MiH+9zwY+ixfN9wDvzf9ciH+948BvAy/FC/c9wHvzPxvi3+bBwG8DD+L5+x7gvfmfD/Hv89nAWwMvxRU/A3w38NP878A/AiP559x4yvUxAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeReview;
impl IconShape for GoCodeReview {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 2.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25h-6.5a.75.75 0 00-.53.22L4.5 14.44v-2.19a.75.75 0 00-.75-.75h-2a.25.25 0 01-.25-.25v-8.5zM1.75 1A1.75 1.75 0 000 2.75v8.5C0 12.216.784 13 1.75 13H3v1.543a1.457 1.457 0 002.487 1.03L8.061 13h6.189A1.75 1.75 0 0016 11.25v-8.5A1.75 1.75 0 0014.25 1H1.75zm5.03 3.47a.75.75 0 010 1.06L5.31 7l1.47 1.47a.75.75 0 01-1.06 1.06l-2-2a.75.75 0 010-1.06l2-2a.75.75 0 011.06 0zm2.44 0a.75.75 0 000 1.06L10.69 7 9.22 8.47a.75.75 0 001.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+b/hpcG/ornhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/v0+C3hrrvhu4Gt40XwW8NFc8dXA5/DvZ54XAszzJ/59vgt4b57TxwBfzQv32cBn8Zy+G3gf/n3M80KAef7Ev913Ae/N8/pr4GV44S4Cx3le3w28D/925nkhwDx/4t/mu4D35vn7G+CleeF2gWM8f98NvA//NuZ5IcA8f+Jf77uA9+YF+xzgs3nhPhv4LF6w7wbeh38987wQYJ4/8a/zXcB784J9D/DevGi+G3gvXrDvBt6Hfx3zvBBgnj/xovsu4L15wb4HeG/+db4beC9esO8G3ocXnXleCDDPn3jRfBfw3rxg3wO8N/823w28Fy/YdwPvw4vGPC8EmOdP/Mu+C3hvXrDvAd6bf5/vBt6LF+y7gffhX2aeFwLM8ydeuM8GPosX7HuA9+Y/xncD78UL9jnAZ/PCmeeFAPP8iRfuInCc5+97gPfmP9Z3A+/F83cr8BBeOPO8EGCeP/HC7QLHeP6+G3gf/mN9F/DePH/PAB7MC2eeFwLM8ydeuM8GPosX7LuB9+E/xncB780L9jnAZ/PCmeeFAPP8iX/ZdwPvxQv23cD78O/zXcB784J9D/De/MvM80KAef7Ei+a7gffiBftu4H34t/ku4L15wb4HeG9eNOZ5IcA8f+JF993Ae/GCfTfwPvzrfBfw3rxg3wO8Ny8687wQYJ4/8a/z3cB78YJ9N/A+vGi+C3hvXrDvAd6bfx3zvBBgnj/xr/fdwHvxgn0N8NG8cF8NfBQv2PcA782/nnleCDDPn/i3+W7gvXjBxAtnXrDvAd6bfxvzvBBgnj/xb/fdwHvx/IkXzjx/3wO8N/925nkhwDx/4t/nu4H34jn9DPDWvHA/DbwVz+l7gPfm38c8LwSY50/8+3018FFc8T3ARwO7vHDHge8G3gq4BHw38NH8+5nnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPP3MsBf83/DSwN/xfNCwF8DL8X/T38j4LOBz+L/p88RcBy4FTjG/y+XgAeLK94b+C7+f3kb4KfFs7038NXAMf5vuwS8N/DTAOI5HQc+Gnhr4KX4v+VvgJ8GvhrY5Qr+EcnZ3sxbDCxNAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodeSquare;
impl IconShape for GoCodeSquare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V1.75a.25.25 0 00-.25-.25H1.75zM0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25V1.75zm9.22 3.72a.75.75 0 000 1.06L10.69 8 9.22 9.47a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM6.78 6.53a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 101.06-1.06L5.31 8l1.47-1.47z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodescan;
impl IconShape for GoCodescan {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.47 4.97a.75.75 0 000 1.06L9.94 7.5 8.47 8.97a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM6.53 6.03a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 101.06-1.06L5.06 7.5l1.47-1.47z",
            }
            path {
                d: "M12.246 13.307a7.5 7.5 0 111.06-1.06l2.474 2.473a.75.75 0 11-1.06 1.06l-2.474-2.473zM1.5 7.5a6 6 0 1110.386 4.094.75.75 0 00-.292.293A6 6 0 011.5 7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodescanCheckmark;
impl IconShape for GoCodescanCheckmark {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.28 6.28a.75.75 0 10-1.06-1.06L6.25 8.19l-.97-.97a.75.75 0 00-1.06 1.06l1.5 1.5a.75.75 0 001.06 0l3.5-3.5z",
            }
            path {
                d: "M7.5 15a7.469 7.469 0 004.746-1.693l2.474 2.473a.75.75 0 101.06-1.06l-2.473-2.474A7.5 7.5 0 107.5 15zm0-13.5a6 6 0 104.094 10.386.75.75 0 01.293-.292A6 6 0 007.5 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzP8tfAz8NfA2wy4sG8aJ7b+CrgOP8z7YLfAzw3fzLEC+a9wa+i/9d3gf4bl44xL/sOPB04Dj/u+wCDwF2ecEQ/7LPBj6L/50+B/hsXjDEv+yvgJfmf6e/Bl6GFwzxLzPP38sAf83/DC8N/BXPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/y9DPDX/M/w0sBf8fyJFwzxL/tr4KX43+lvgJfmBUP8yz4b+Cz+d/oc4LN5wRD/suPArcAx/ne5BDwY2OUFQ7xo3hv4Lv53eRvgp3nhEC+69wa+GjjG/2yXgPcGfpp/GeJf5zjw0cBbAy/F/yx/A/w08NXALi8axP9viP/fEP+/If5/Qzyn48BHAW8NvDT/t/w18NPA1wC7XIF4tvcGvgo4zv9tu8DHAN8NIK54b+C7+P/lfYDvFnAceDpwnP9fdoGHCPhs4LP4/+lzBPwV8NL8//TXAszz9zLAX/N/w0sDf8XzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn3j+jgOfBbw18GCuuBX4buBz+JcdBz4LeGvgwfzHuBX4buBzeMHM80KAef7E8/fVwEfx/H0N8NG8cF8NfBT/Ob4G+GieP/O8EGCeP/H8XQSO8/zdCjyEF+4icJz/HLcCD+H5M88LAeb5E8/fLnCM5+8ZwIN54XaBY/zneAbwYJ4/87wQYJ4/8fx9NvBZPH+fA3w2L9xnA5/Ff47PAT6b5888LwSY50+8YF8NvDXwIK54BvDdwGfzovlq4K2BB/Ef4xnAdwOfzQtmnhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef5eBvhr/m94aeCveF4I+Gvgpfj/6W8EfDbwWfz/9DkCjgO3Asf4/+US8GBxxXsD38X/L28D/LR4tvcGvho4xv9tl4D3Bn4aQDyn48BHA28NvBT/t/wN8NPAVwO7XME/Aog2pPKkqXVVAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCodespaces;
impl IconShape for GoCodespaces {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 1.75C2 .784 2.784 0 3.75 0h8.5C13.216 0 14 .784 14 1.75v5a1.75 1.75 0 01-1.75 1.75h-8.5A1.75 1.75 0 012 6.75v-5zm1.75-.25a.25.25 0 00-.25.25v5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25v-5a.25.25 0 00-.25-.25h-8.5zM0 11.25c0-.966.784-1.75 1.75-1.75h12.5c.966 0 1.75.784 1.75 1.75v3A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25v-3zM1.75 11a.25.25 0 00-.25.25v3c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-3a.25.25 0 00-.25-.25H1.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M3 12.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zm4 0a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5h-4.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoColumns;
impl IconShape for GoColumns {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 0A1.75 1.75 0 001 1.75v12.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 007 14.25V1.75A1.75 1.75 0 005.25 0h-2.5zM2.5 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-2.5a.25.25 0 01-.25-.25V1.75zM10.75 0A1.75 1.75 0 009 1.75v12.5c0 .966.784 1.75 1.75 1.75h2.5A1.75 1.75 0 0015 14.25V1.75A1.75 1.75 0 0013.25 0h-2.5zm-.25 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-2.5a.25.25 0 01-.25-.25V1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8fy8D/DX/O7w08Fc8f+LZEM/rr4GX4t/mVuC7gc/hX+ezgI8GjvOf62+Al+bZEM/rs4HP4t/na4CP5kXzXcB781/jc4DP5tkQz+s4cCtwjH+7W4GH8C/7LuC9+a9xCXgwsMuzIZ6/9wa+i38f8cJ9F/De/Nd5G+CneU6IF+y9ga8GjvFvI16w7wLem/8al4D3Bn6a54V44Y4DHw28NfBS/OuI5++7gPfmP9/fAD8NfDWwy/OH+Pczz594Xt8FvDcv2PcA781/HcS/n3n+xHP6LuC9ecG+B3hv/msh/v3M8yee7buA9+YF+x7gvfmvh/j3M8+fuOK7gPfmBfse4L3574H49zPPn4DvAt6bF+x7gPfmvw/i3888f98NvDcv2PcA781/L8S/n/nX+x7gvfnvh/j3+23gtXjRfQ/w3vzPgPj3+2zgs3jRfA/w3vzPgfj3Ow78NvBSvHDfA7w3/7Mg/mM8GPht4EE8f98DvDf/8yD+Y3028NbAS3HFzwDfDfw0/zMh/n9D/P+G+P8N8f8b/wj9BItB7UezOgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoComment;
impl IconShape for GoComment {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h4.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25H2.75zM1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v7.5A1.75 1.75 0 0113.25 12H9.06l-2.573 2.573A1.457 1.457 0 014 13.543V12H2.75A1.75 1.75 0 011 10.25v-7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfmf5a/Bn4a+Bpgl/8YiGd7b+CrgOP8z7YLfAzw3fz7Ia54b+C7+N/lfYDv5t8HAceBpwPH+d9lF3gIsMu/HQI+G/gs/nf6HOCz+bdDwF8BL83/Tn8NvAz/dggwz9/LAH/N/wwvDfwVz5/4t0OAef7E/yzm+RP/dggwz5/4n8U8f+I5HQc+C3hr4ME8218DPw18DbDLFQgwz5/4n8U8f+I5fTXwUbxgu8DHAN8NIMA8f+J/FvP8ied0ETjOv+x9gO8WYJ4/8T+Lef7Ec9oFjvEv2wUeIsA8f+J/FvP8ief02cBn8aL5HAHm+RP/s5jnTzyvrwbeGngQL9xfCzDPn/ifxTx/4l/20sBf8bwQYJ4/8T+Lef7Ei8Y8LwSY50/8z2KeP/GiMc8LAeb5E/+zmOdPvGjM80KAef7E/yzm+RMvGvO8EGCeP/E/i3n+xIvGPC8EmOdP/M9inj/xojHPCwHm+RP/s5jnT7xozPNCgHn+xP8s5vkTLxrzvBBgnj/xP4t5/sSLxjwvBJjnT/zPYp4/8aIxzwsB5vkT/7OY50+8aMzzQoB5/sT/LOb5Ey8a87wQYJ4/8T+Lef7Ei8Y8LwSY50/8z2KeP/GiMc8LAeb5exngr/mf4aWBv+L5Ey8a87wQ8NfAS/GiuRX4buBz+Nf5LOCjgeP8x/ob4KV50ZjnhYDPBj6Lf52vAT6aF813Ae/Nf47PAT6bF415Xgg4DtwKHONFdyvwEP5l3wW8N/85LgEPBnZ50ZjnhbjivYHv4l9HvHDfBbw3/3neBvhpXnTmeSGe7b2BrwaO8aIRL9h3Ae/Nf45LwHsDP82/jnleiOd0HPho4K2Bl+KFE8/fdwHvzX+8vwF+GvhqYJd/PfO8EP8y8/yJ5/VdwHvzgn0P8N789zDPC/EvM8+feE7fBbw3L9j3AO/Nf4+XBv6K54X4l5nnTzzbdwHvzQv2PcB785/rOPBZwFsDD+ZF8zfiX2aeP3HFdwHvzQv2PcB785/vq4GP4l/nc8S/zDx/Ar4LeG9esO8B3pt/nePAdwFvzRVfDXwM/7KLwHFedJeAB4t/mXn+vht4b16w7wHem3+9nwbeiuf03cD78MKZf523AX5a/MvMv973AO/Nv415/r4beB9eMPOiuQS8N/DTAOJf9tvAa/Gi+x7gvfm3My/YdwPvw/NnXri/AX4a+GpglysQ/7LPBj6LF833AO/Nv89XAx/FC/bdwPvwvMzzJ14wxL/sOPDbwEvxwn0P8N78x/hu4L14wb4beB+ek3n+xAuGeNE8GPht4EE8f98DvDf/sb4beC9esO8G3odnM8+feMEQ/zqfDbw18FJc8TPAdwM/zX+O7wbeixfsu4H34Qrz/IkXDPE/33cD78UL9t3A+wDm+RMvGOJ/h+8G3osX7LuB9+b5Ey8Y4n+P7wbei3898YIh/nf5buC9eNH9DvDavGCI/32+G3gvXjQfA3w1Lxjif6fvBt6LF+5vgNcGdnnBEP97fTfwXjx/zwBeG7iVFw7xv9tbA+8NvBVX/A3w08Bn86JB/P+G+P8N8f8b4v83/hHIQtboAWQNvAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCommentDiscussion;
impl IconShape for GoCommentDiscussion {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 2.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25h-3.5a.75.75 0 00-.53.22L3.5 11.44V9.25a.75.75 0 00-.75-.75h-1a.25.25 0 01-.25-.25v-5.5zM1.75 1A1.75 1.75 0 000 2.75v5.5C0 9.216.784 10 1.75 10H2v1.543a1.457 1.457 0 002.487 1.03L7.061 10h3.189A1.75 1.75 0 0012 8.25v-5.5A1.75 1.75 0 0010.25 1h-8.5zM14.5 4.75a.25.25 0 00-.25-.25h-.5a.75.75 0 110-1.5h.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0114.25 12H14v1.543a1.457 1.457 0 01-2.487 1.03L9.22 12.28a.75.75 0 111.06-1.06l2.22 2.22v-2.19a.75.75 0 01.75-.75h1a.25.25 0 00.25-.25v-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoContainer;
impl IconShape for GoContainer {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.41.24l4.711 2.774A1.767 1.767 0 0116 4.54v5.01a1.77 1.77 0 01-.88 1.53l-7.753 4.521-.002.001a1.767 1.767 0 01-1.774 0H5.59L.873 12.85A1.762 1.762 0 010 11.327V6.292c0-.304.078-.598.22-.855l.004-.005.01-.019c.15-.262.369-.486.64-.643L8.641.239a1.75 1.75 0 011.765 0l.002.001zM9.397 1.534a.25.25 0 01.252 0l4.115 2.422-7.152 4.148a.267.267 0 01-.269 0L2.227 5.716l7.17-4.182zM7.365 9.402L8.73 8.61v4.46l-1.5.875V9.473a1.77 1.77 0 00.136-.071zm2.864 2.794V7.741l1.521-.882v4.45l-1.521.887zm3.021-1.762l1.115-.65h.002a.268.268 0 00.133-.232V5.264l-1.25.725v4.445zm-11.621 1.12l4.1 2.393V9.474a1.77 1.77 0 01-.138-.072L1.5 7.029v4.298c0 .095.05.181.129.227z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilot;
impl IconShape for GoCopilot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.25 9a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 016.25 9zm4.25.75a.75.75 0 00-1.5 0v1.5a.75.75 0 001.5 0v-1.5z",
            }
            path {
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .572-.053 1.147-.254 1.655l.168.838.066.033A2.75 2.75 0 0116 9.736V11c0 .24-.086.438-.156.567a2.173 2.173 0 01-.259.366c-.18.21-.404.413-.605.58a10.373 10.373 0 01-.792.597l-.015.01-.006.004-.028.018a8.832 8.832 0 01-.456.281c-.307.177-.749.41-1.296.642C11.296 14.528 9.756 15 8 15c-1.756 0-3.296-.472-4.387-.935a12.06 12.06 0 01-1.296-.641 8.815 8.815 0 01-.456-.281l-.028-.02-.006-.003-.015-.01a7.077 7.077 0 01-.235-.166c-.15-.108-.352-.26-.557-.43a5.19 5.19 0 01-.605-.58 2.167 2.167 0 01-.259-.367A1.19 1.19 0 010 11V9.736a2.75 2.75 0 011.52-2.46l.067-.033.167-.838C1.553 5.897 1.5 5.322 1.5 4.75c0-.87.127-1.77.693-2.485.579-.733 1.494-1.124 2.724-1.26 1.206-.134 2.262.034 2.944.765zM3.024 7.709L3 7.824v4.261c.02.013.043.025.065.038.264.152.65.356 1.134.562.972.412 2.307.815 3.801.815 1.494 0 2.83-.403 3.8-.815a10.6 10.6 0 001.2-.6v-4.26l-.023-.116c-.49.21-1.075.291-1.727.291-1.146 0-2.06-.328-2.71-.991A3.223 3.223 0 018 6.266c-.144.269-.321.52-.54.743C6.81 7.672 5.896 8 4.75 8c-.652 0-1.237-.082-1.727-.291zm3.741-4.916c-.193-.207-.637-.414-1.681-.298-1.02.114-1.48.404-1.713.7-.247.313-.37.79-.37 1.555 0 .792.129 1.17.308 1.37.162.181.52.38 1.442.38.854 0 1.339-.236 1.638-.54.315-.323.527-.827.618-1.553.117-.936-.038-1.396-.242-1.614zm2.472 0c.193-.207.637-.414 1.681-.298 1.02.114 1.48.404 1.713.7.247.313.37.79.37 1.555 0 .792-.129 1.17-.308 1.37-.162.181-.52.38-1.442.38-.854 0-1.339-.236-1.638-.54-.315-.323-.527-.827-.618-1.553-.117-.936.038-1.396.242-1.614z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilotError;
impl IconShape for GoCopilotError {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .572-.053 1.147-.254 1.655l.168.838.066.033A2.75 2.75 0 0116 9.736V11c0 .24-.086.438-.156.567a1.755 1.755 0 01-.075.125L13 9.688V7.824l-.023-.115c-.49.21-1.075.291-1.727.291-.22 0-.43-.012-.633-.036L6.824 5.22c.082-.233.143-.503.182-.813.117-.936-.038-1.396-.242-1.614-.193-.207-.637-.414-1.681-.298-.707.079-1.144.243-1.424.434l-1.251-.905c.58-.579 1.422-.899 2.51-1.02 1.205-.133 2.26.035 2.943.766zm1.376 1.023c.193-.207.637-.414 1.681-.298 1.02.114 1.48.404 1.713.7.247.313.37.79.37 1.555 0 .792-.129 1.17-.308 1.37-.162.181-.52.38-1.442.38-.854 0-1.339-.236-1.638-.54-.315-.323-.527-.827-.618-1.553-.117-.936.038-1.396.242-1.614zM.865 2.759A.75.75 0 00.31 4.107l1.193.864c.013.498.076.992.251 1.434l-.167.838-.067.033A2.75 2.75 0 000 9.736V11c0 .24.086.438.156.567.075.137.169.261.259.366.18.21.404.413.605.58a10.368 10.368 0 00.792.597l.015.01.006.004.028.018.098.065a12.06 12.06 0 001.654.859C4.704 14.527 6.244 15 8 15c1.756 0 3.296-.472 4.387-.935.395-.167.734-.335 1.008-.482l1.415 1.024a.75.75 0 001.063-1.025.753.753 0 01-.188-.1L.865 2.76zM4.75 8c.297 0 .579-.022.844-.066l6.427 4.654c-.07.032-.144.064-.22.097-.972.412-2.307.815-3.801.815-1.494 0-2.83-.403-3.8-.815a10.594 10.594 0 01-1.2-.6v-4.26l.023-.116c.49.21 1.075.291 1.727.291z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopilotWarning;
impl IconShape for GoCopilotWarning {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .463-.035.929-.155 1.359a5.967 5.967 0 00-1.398-.616c.034-.195.053-.439.053-.743 0-.766-.123-1.242-.37-1.555-.233-.296-.693-.586-1.713-.7-1.044-.116-1.488.091-1.681.298-.204.218-.359.678-.242 1.614.06.479.172.86.332 1.158a6.014 6.014 0 00-2.92 2.144C5.926 7.904 5.372 8 4.75 8c-.652 0-1.237-.082-1.727-.291L3 7.824v4.261c.02.013.043.025.065.038a10.84 10.84 0 002.495 1.035c.21.629.522 1.21.916 1.726a11.91 11.91 0 01-2.863-.819 12.06 12.06 0 01-1.296-.641 8.815 8.815 0 01-.456-.281l-.028-.02-.006-.003-.015-.01a7.077 7.077 0 01-.235-.166c-.15-.108-.352-.26-.557-.43a5.19 5.19 0 01-.605-.58 2.167 2.167 0 01-.259-.367A1.19 1.19 0 010 11V9.736a2.75 2.75 0 011.52-2.46l.067-.033.167-.838C1.553 5.897 1.5 5.322 1.5 4.75c0-.87.127-1.77.693-2.485.579-.733 1.494-1.124 2.724-1.26 1.206-.134 2.262.034 2.944.765zM6.765 2.793c-.193-.207-.637-.414-1.681-.298-1.02.114-1.48.404-1.713.7-.247.313-.37.79-.37 1.555 0 .792.129 1.17.308 1.37.162.181.52.38 1.442.38.854 0 1.339-.236 1.638-.54.315-.323.527-.827.618-1.553.117-.936-.038-1.396-.242-1.614z",
                fill_rule: "evenodd",
            }
            path {
                d: "M8.498 14.81a4.5 4.5 0 105.504-7.121 4.5 4.5 0 00-5.504 7.122zM10.5 8.75a.75.75 0 011.5 0V11a.75.75 0 01-1.5 0V8.75zm.75 5.75a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEP0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdRz4KOCtgZfmv9dfAz8NfA2wyxWI/zzvDXwVcJz/WXaBjwG+G0D853hv4Lv4n+19gO8W//GOA08HjvM/2y7wEPEf77OBz+J/h88R//H+Cnhp/nf4a/Efzzx/LwP8Nf89Xhr4K54X4j+eef7Efy/zvBD/8czzJ/57meeF+I9nnj/x38s8L8R/PPP8if9e5nkh/uOZ50/89zLPC/Efzzx/4r+XeV6I/3jm+RP/vczzQvzHM8+f+O9lnhfiP555/sR/L/O8EP/xzPMn/nuZ54X4j2eeP/HfyzwvxH888/yJ/17meSH+45nnT/z3Ms8L8ZyOAx8FvDXw0rxgtwLfDXwOz8s8f+K/l3leiGd7b+CrgOO86L4G+Giek3n+xH8v87wQV7w38F38690KPITnZJ4/8d/LPC8EHAeeDhznX+8ZwIN5Tub5E/+9zPNCwGcDn8W/zecAn81zMs+f+O9lnhcC/gp4af51ngF8N/DZPC/z/In/XuZ5IcA8fy8D/DX/eub5E/+9zPNCgHn+xL+Nef7Efy/zvBBgnj/xb2OeP/HfyzwvBJjnT/zbmOdP/PcyzwsB5vkT/zbm+RP/vczzQoB5/sS/jXn+xH8v87wQYJ4/8W9jnj/x38s8LwSY50/825jnT/z3Ms8LAeb5E/825vkT/73M80KAef7Ev415/sR/L/O8EGCeP/FvY54/8d/LPC8EmOdP/NuY5+9lgL/mv8dLA3/F80KAef7Ev81fAy/F/w5/I8A8f+Lf5rOBz+J/h88RYJ4/8W9zHLgVOMb/bJeABwswz5/4t3tv4Lv4n+1tgJ8WYJ4/8e/z3sBXA8f4n+US8N7ATwMIMM+f+Pc7Dnw08NbAS/Hf62+Anwa+GtjlCgSY50/834cA8/yJ//sQYJ4/8X8fAszzJ/7vQ4B5/sR/r+PAZwFvzRXfDXwO/7EQYJ4/8d/rq4GP4jl9DfDR/MdBgHn+xH+vi8BxntOtwEP4j4MA8/yJ/17m+RP/cRBgnj/x38s8f+I/DgLM8yf+e5nnT/zHQYB5/sR/L/P8if84CDDPn/jvZZ4/8R8HAeb5E/+9zPMn/uMgwDx/LwP8Nf89Xhr4K54/8R8HAX8NvBT/O/wN8NL8x0HAZwOfxf8OnwN8Nv9xEHAcuBU4xv9sl4AHA7v8x0Fc8d7Ad/E/29sAP81/LMSzvTfw1cAx/me5BLw38NP8x0M8p+PARwNvDbwU/73+Bvhp4KuBXf5z8I9nUKvgODdkeAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCopy;
impl IconShape for GoCopy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 010 1.5h-1.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-1.5a.75.75 0 011.5 0v1.5A1.75 1.75 0 019.25 16h-7.5A1.75 1.75 0 010 14.25v-7.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0114.25 11h-7.5A1.75 1.75 0 015 9.25v-7.5zm1.75-.25a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviPdxz4LOCtueK7gc/hX+848FnAW3PFdwOfw38sxH+8rwY+iuf0NcBH86/z1cBH8Zy+Bvho/uMg/uNdBI7znG4FHsK/zkXgOM/pVuAh/MdB/Mczz5/41zHPn/iPg/iPZ54/8a9jnj/xHwfxH888f+Jfxzx/4j8O4j+eef7Ev455/sR/HMR/PPP8iX8d8/yJ/ziIf53jwEcBbw28NP+z/DXw08DXALu8aBAvuvcGvgo4zv9su8DHAN/Nvwzxonlv4Lv43+V9gO/mhUP8y44DTweO87/LLvAQYJcXDPEv+2zgs/jf6XOAz+YFQ/zL/gp4af53+mvgZXjBEP8y8/y9DPDX/M/w0sBf8fyJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVDwHHgs4C3Bh7Mi048f8eBzwLeGngw/zFuBb4b+BxeMPOiuxX4buBzBHw18FH864nn76uBj+I/x9cAH83zZ/71vkbAReA4/3ri+bsIHOc/x63AQ3j+zL/erQJ2gWP864nnz/znEs+f+dd7hoDPBj6Lfz3x/Jn/XOL5M/96nyOu+GrgrYEH8aITz5/5zyWeP/Oiewbw3cBni3+Zef7E82eeP/GvY54/8fyZ50+8YIh/mXn+xPNnnj/xr2OeP/H8medPvGCIf5l5/sTzZ54/8a9jnj/x/JnnT7xgiH+Zef7E82eeP/GvY54/8fyZ50+8YIh/mXn+xPNnnj/xr2OeP/H8medPvGCIf5l5/sTzZ54/8a9jnj/x/JnnT7xgiH+Zef7E82eeP/GvY54/8fyZ50+8YIh/mXn+xPNnnj/xr2OeP/H8medPvGCIf5l5/sTzZ54/8a9jnj/x/JnnT7xgiH+Zef7E82eeP/GvY54/8fyZ50+8YIh/mXn+xPNnnj/xr2OeP/H8medPvGCIf5l5/sTzZ54/8a9jnj/x/JnnT7xgCDgOfBbw1sCDedGJ58/85xLPn3nR3Qp8N/A5Ar4a+Cj+9cTzZ/5ziefP/Ot9jYCLwHH+9cTzZ/5ziefP/OvdKmAXOMa/nnj+doFj/Od4BvBgnj/zr/cMAZ8NfBb/euL5+2zgs/jP8TnAZ/P8mX+9zxFXfDXw1sCDeNGJF+yzgfcGHsR/jGcA3w18Ni+YedE9A/hu4LPFv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz9/LAH/N/wwvDfwVz594wRD/sr8GXor/nf4GeGleMMS/7LOBz+J/p88BPpsXDPEvOw7cChzjf5dLwIOBXV4wxIvmvYHv4n+XtwF+mhcO8aJ7b+CrgWP8z3YJeG/gp/mXIf51jgMfDbw18FL8z/I3wE8DXw3s8qJB/Mczz5/41zHPn/iPg/iPZ54/8a9jnj/xHwfxH888f+Jfxzx/4j8O4j+eef7Ev455/sR/HMR/PPP8iX8d8/yJ/ziI/3i7wDGe0zOAB/Ovswsc4zk9A3gw/3EQ//E+G/gsntPnAJ/Nv85nA5/Fc/oc4LP5j4P4z/HVwFtzxXcDn82/zVcDb80V3w18Nv+x+EcfQs/fxshZOgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCpu;
impl IconShape for GoCpu {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.5.75a.75.75 0 00-1.5 0V2H3.75A1.75 1.75 0 002 3.75V5H.75a.75.75 0 000 1.5H2v3H.75a.75.75 0 000 1.5H2v1.25c0 .966.784 1.75 1.75 1.75H5v1.25a.75.75 0 001.5 0V14h3v1.25a.75.75 0 001.5 0V14h1.25A1.75 1.75 0 0014 12.25V11h1.25a.75.75 0 000-1.5H14v-3h1.25a.75.75 0 000-1.5H14V3.75A1.75 1.75 0 0012.25 2H11V.75a.75.75 0 00-1.5 0V2h-3V.75zm5.75 11.75h-8.5a.25.25 0 01-.25-.25v-8.5a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25zM5.75 5a.75.75 0 00-.75.75v4.5c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-4.5a.75.75 0 00-.75-.75h-4.5zm.75 4.5v-3h3v3h-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC8klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4TseBjwLeGnhp/m/5a+Cnga8BdrkC8WzvDXwVcJz/23aBjwG+G0Bc8d7Ad/H/y/sA3y3gOPB04Dj/v+wCDxHw2cBn8f/T5wj4K+Cl+f/prwWY5+9lgL/m/4aXBv6K54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY/78QYP7/QoD5/wsB5v8vBJj/vxBg/v9CgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xP8Nx4LOAtwYezPN3K/DdwOfwgpnnhQDz/In/Gb4a+CheNF8DfDTPn3leCDDPn/if4SJwnBfNrcBDeP7M80KAef7E/wy7wDFeNM8AHszzZ54XAszzJ/5n+Gzgs3jRfA7w2Tx/5nkhwDx/4n+OrwbeGngQz98zgO8GPpsXzDwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM/fywB/zf8NLw38Fc8LAX8NvBT/P/2NgM8GPov/nz5HwHHgVuAY/79cAh4srnhv4Lv4/+VtgJ8Wz/bewFcDx/i/7RLw3sBPA4jndBz4aOCtgZfi/5a/AX4a+GpglysQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Ee23m7c6FRBjwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCreditCard;
impl IconShape for GoCreditCard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.75 9a.75.75 0 000 1.5h1.5a.75.75 0 000-1.5h-1.5z",
            }
            path {
                d: "M0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5zm14.5 0V5h-13V3.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25zm0 2.75h-13v5.75c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/nd4a+Cn+/f5a/O/01sBP8e+H+N/rvYHv4t8H8b/bewPfxb8d4oU7DnwU8NbAS/Of51bgu4HP4V/vvYHv4t8G8YK9N/BVwHH+63wN8NH867038F386yGev/cGvov/ercCD+Hf5r2B7+JfB/G8jgNPB47zX+8ZwIP5t3tv4Lt40SGe12cDn8V/j88BPpt/u+8C3psXHeJ5/RXw0vzXegbw3cBn82/3XcB786+DeF7m+XsZ4K/5n+m7gPfmXw/xvMzzJ/5n+i7gvfm3QTwv8/yJ/3m+C3hvXrDvAX4b+C6eP8TzMs+f+J/lu4D35gX7HuC9ueK9ge/ieSGel3n+xP8c3wW8Ny/Y9wDvzXN6b+C7eE6I52WeP/E/w3cB780L9j3Ae/P8vTfwXTwb4nmZ50/89/su4L15wb4HeG9euPcGvosrEM/LPH/iv9d3Ae/NC/Y9wHvzonlv4LsAxPMyz5/47/NdwHvzgn0P8N7867w1gHhe5vkT/z2+C3hvXrDvAd6bfxvE8zLPn/iv913Ae/OCfQ/w3vzbIZ6Xef7Ef63vAt6bF+x7gPfm3wfxvMzzJ/7rfBfw3rxg3wO8N/9+iOdlnj/xX+OrgY/iBfse4L35j4F4Xub5E/81zAv2PcB78x8H8bzM8yf+a5jn73uA9+Y/FuJ5medP/Nf4aeCteE7fA7w3//EQz8s8f+K/xnHgu4G3Ai4B3w18NP85EM/LPH/if5fjwGcBb80V3w18Ds8J8bzM8yf+d/lq4KN4Tl8DfDTPhnhe5vkT/7tcBI7znG4FHsKzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ4/8b+Lef7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m+RP/u5jnTzwb4nmZ5+9lgL/mf4eXBv6K5088G+J5/TXwUvzb3Ap8N/A5/Ot8FvDRwHH+c/0N8NI8G+J5fTbwWfz7fA3w0bxovgt4b/5rfA7w2Twb4nkdB24FjvFvdyvwEP5l3wW8N/81LgEPBnZ5NsTz997Ad/HvI1647wLem/86bwP8NM8J8YK9N/DVwDH+bcQL9l3Ae/Nf4xLw3sBP87wQL9xx4KOBtwZein8d8fx9F/De/Of7G+Cnga8Gdnn+EP9+5vkTz+u7gPfmBfse4L35r4P49zPPn3hO3wW8Ny/Y9wDvzX8txL+fef7Es30X8N68YN8DvDf/9RD/fub5E1d8F/DevGDfA7w3/z0Q/37m+RPwXcB784J9D/De/PdB/PuZ5++7gffmBfse4L3574X49zP/et8DvDf//RD/fr8NvBYvuu8B3pv/GRD/fp8NfBYvmu8B3pv/ORD/fseB3wZeihfue4D35n8WxH+MBwO/DTyI5+97gPfmfx7Ef6zPBt4aeCmu+Bngu4Gf5n8m/hFpi65PtMtoRAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoCrossReference;
impl IconShape for GoCrossReference {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M16 1.25v4.146a.25.25 0 01-.427.177L14.03 4.03l-3.75 3.75a.75.75 0 11-1.06-1.06l3.75-3.75-1.543-1.543A.25.25 0 0111.604 1h4.146a.25.25 0 01.25.25zM2.75 3.5a.25.25 0 00-.25.25v7.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h4.5a.25.25 0 00.25-.25v-2.5a.75.75 0 111.5 0v2.5A1.75 1.75 0 0113.25 13H9.06l-2.573 2.573A1.457 1.457 0 014 14.543V13H2.75A1.75 1.75 0 011 11.25v-7.5C1 2.784 1.784 2 2.75 2h5.5a.75.75 0 010 1.5h-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABt0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4h/2XHgs4C3Bh7M/2y3At8NfA4vGsS/7KuBj+J/l68BPpp/GeJfdhE4zv8utwIP4V+G+JftAsf43+UZwIP5lyH+ZZ8NfBb/u3wO8Nn8yxAvmq8G3hp4EP+zPQP4buCzedEg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CO6oQ5BQWh2SQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDash;
impl IconShape for GoDash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 7.75A.75.75 0 012.75 7h10a.75.75 0 010 1.5h-10A.75.75 0 012 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDatabase;
impl IconShape for GoDatabase {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 3.5c0-.133.058-.318.282-.55.227-.237.592-.484 1.1-.708C4.899 1.795 6.354 1.5 8 1.5c1.647 0 3.102.295 4.117.742.51.224.874.47 1.101.707.224.233.282.418.282.551 0 .133-.058.318-.282.55-.227.237-.592.484-1.1.708C11.101 5.205 9.646 5.5 8 5.5c-1.647 0-3.102-.295-4.117-.742-.51-.224-.874-.47-1.101-.707-.224-.233-.282-.418-.282-.551zM1 3.5c0-.626.292-1.165.7-1.59.406-.422.956-.767 1.579-1.041C4.525.32 6.195 0 8 0c1.805 0 3.475.32 4.722.869.622.274 1.172.62 1.578 1.04.408.426.7.965.7 1.591v9c0 .626-.292 1.165-.7 1.59-.406.422-.956.767-1.579 1.041C11.476 15.68 9.806 16 8 16c-1.805 0-3.475-.32-4.721-.869-.623-.274-1.173-.62-1.579-1.04-.408-.426-.7-.965-.7-1.591v-9zM2.5 8V5.724c.241.15.503.286.779.407C4.525 6.68 6.195 7 8 7c1.805 0 3.475-.32 4.722-.869.275-.121.537-.257.778-.407V8c0 .133-.058.318-.282.55-.227.237-.592.484-1.1.708C11.101 9.705 9.646 10 8 10c-1.647 0-3.102-.295-4.117-.742-.51-.224-.874-.47-1.101-.707C2.558 8.318 2.5 8.133 2.5 8zm0 2.225V12.5c0 .133.058.318.282.55.227.237.592.484 1.1.708 1.016.447 2.471.742 4.118.742 1.647 0 3.102-.295 4.117-.742.51-.224.874-.47 1.101-.707.224-.233.282-.418.282-.551v-2.275c-.241.15-.503.285-.778.406-1.247.549-2.917.869-4.722.869-1.805 0-3.475-.32-4.721-.869a6.236 6.236 0 01-.779-.406z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPcxz4LOCtgQfzwt0KfDfwOfzXQvzn+Wrgo/jX+Rrgo/mvg/jPcxE4zr/OrcBD+K+D+M+zCxzjX0/810H85/ls4LP41xP/dRD/ub4aeGvgQbzoxH8dxH8f8/yJ/zqI/z7m+RP/dRD/fczzJ/7rIP77mOdP/NdB/Pcxz5/4r4P472OeP/FfB/Hfxzx/4r8O4r+Pef7Efx3Ev85rA+8FvDTw0vzP8tfAXwPfA/w2LxrEi+alga8CXpv/HX4b+Bjgr3nhEP+ylwZ+CzjO/y67wOsAf80LhnjhXhr4LeA4/zvtAq8D/DXPH+KF+y3gtfnf7beB1+H5Q7xgrw38Fv83vA7w2zwvxAv23cB78bwuAZ8N/DRwK/8zPBh4b+CzeP6+B3hvnhfiBfsr4KV5Xh8DfDX/M3008FU8r78GXobnhXjBzPN3Atjlf6YHA0/n+RPPC/GCmedP/M9mnj/xvBAvmHn+xP9s5vkTzwvxgpnnT/zPZp4/8bwQL5h5/sT/bOb5E88L8YKZ50/8z2aeP/G8EC+Yef7E/2zm+RPPC/GCmedP/M9mnj/xvBAvmHn+xAt3HPgs4K254ruBz+Ff7zjwWcBbc8V3A5/Dv8w8f+J5IV4w8/yJF+6rgY/iOX0N8NH863w18FE8p68BPpoXzjx/4nkh4DjwWcBbAw/mXyZeuIvAcZ7TrcBD+Ne5CBznOd0KPIQXzvzLbgW+G/gcAV8NfBQvOvHCmedP/OuY50+8cOZF9zUCLgLHedGJF848f+Jfxzx/4oUzL7pbBewCx3jRiRfOPH/iX8c8f+KFMy+6Zwj4bOCzeNGJF848f+Jfxzx/4oUzL7rPEVd8NfDWwIP4l4kXzjx/4l/HPH/ihTP/smcA3w18tnjBzPMnXjjz/Il/HfP8iRfOPH/ieSFeMPP8iRduFzjGc3oG8GD+dXaBYzynZwAP5oUzz594XogXzDx/4oX7bOCzeE6fA3w2/zqfDXwWz+lzgM/mhTPPn3heiBfMPH/iX/bVwFtzxXcDn82/zVcDb80V3w18Nv8y8/yJ54V4wczzJ/5nM8+feF6IF8w8f+J/NvP8ieeFeMHM8yf+ZzPPn3heiBfMPH/ifzbz/InnhXjBzPMn/mczz594XogXzDx/4n828/yJ54V4wczzJ/5nM8+feF6IF8w8fyeAXf5nejDwdJ4/8bwQL9hfAy/F8/oc4LP5n+mzgc/ief0N8NI8L8QL9t3Ae/H8fTTwM8Ct/M/wYOC9gM/m+fse4L15XogX7LWB3+L/htcBfpvnhXjhfht4Lf53+x3gtXn+EC/cSwO/DRzjf6dLwGsDf83zh/iXvTTw28Ax/ne5BLw28Ne8YIgXzUsDXw28Fv87/A7w0cBf88Ih/nVeG3hv4KWBl+J/lr8B/hr4buC3edEg/n9D/P+G+P8N8f8b4v83xP9v/CPc6awQDg2ASgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDependabot;
impl IconShape for GoDependabot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.75 7.5a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5a.75.75 0 01.75-.75zm5.25.75a.75.75 0 00-1.5 0v1.5a.75.75 0 001.5 0v-1.5z",
            }
            path {
                d: "M6.25 0a.75.75 0 000 1.5H7.5v2H3.75A2.25 2.25 0 001.5 5.75V8H.75a.75.75 0 000 1.5h.75v2.75a2.25 2.25 0 002.25 2.25h8.5a2.25 2.25 0 002.25-2.25V9.5h.75a.75.75 0 000-1.5h-.75V5.75a2.25 2.25 0 00-2.25-2.25H9V.75A.75.75 0 008.25 0h-2zM3 5.75A.75.75 0 013.75 5h8.5a.75.75 0 01.75.75v6.5a.75.75 0 01-.75.75h-8.5a.75.75 0 01-.75-.75v-6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjvcRz4LOCtueK7gc/hvx7iv8dXAx/Fc/oa4KP5r4X473EROM5zuhV4CP+1EP89zPMn/mshntNx4KOAtwZemn+9W4HvBj6HF848f+IFOw58FvDWwIP51/tr4KeBrwF2uQLxbO8NfBVwnH+/rwE+mhfMPH/iBftq4KP499sFPgb4bgBxxXsD38V/nFuBh/CCmedPvGAXgeP8x3kf4LsFHAeeDhznP84zgAfzgpnnT7xgu8Ax/uPsAg8R8NnAZ/Ef63OAz+YFM8+feME+G/gs/mN9joC/Al6a/xjPAL4b+GxeOPP8iRfuq4G3Bh7Ef4y/FmCev5cB/pr/HOb5E/85Xhr4K54XAszzJ/7zmOdP/OcxzwsB5vkT/3nM8yf+85jnhQDz/In/POb5E/95zPNCgHn+xH8e8/yJ/zzmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY50/85zHPn/jPY54XAszzJ/7zmOdP/OcxzwsB5vkT/3pvDfwU/7neBvhp/vXM80KAef7Ev95bAz/Ff663AX6afz3zvBBgnj/xb/PewHfxn+N9gO/m38Y8LwSY50/827038F38x3of4Lv5tzPPCwHm+RP/Pu8NfBf/Md4H+G7+fczzQoB5/sS/33sD38W/z/sA382/n3leCDDPn/iP8d7Ad/Fv8z7Ad/MfwzwvBJjnT/zHeW/gu/jXeR/gu/mPY54XAszzJ/5jvTfwXbxo3gf4bv5jmeeFAPP8if947w18Fy/c+wDfzX8887wQYJ4/8Z/jvYHv4vl7H+C7+c9hnhcCzPMn/vO8N/BdPKf3Ab6b/zzmeSHAPH/iP9d7A9/FFe8DfDf/uczzQoB5/sR/vrfmip/mP595Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCev5cB/pr/G14a+CueFwL+Gngp/n/6GwGfDXwW/z99joDjwK3AMf5/uQQ8WFzx3sB38f/L2wA/LZ7tvYGvBo7xf9sl4L2BnwYQz+k48NHAWwMvxf8tfwP8NPDVwC5XIP5zvDTwVzyvvwFemhfuVuBBPK+XAf6a/1iI/xzvDXwXz+t3gNfmhftt4LV4Xu8DfDf/sRD/OT4b+Cye19cAH80L993Ae/G8Pgf4bP5jIf5zPB14MM/rfYDv5oV7b+C7eF5/DbwM/7EQ//FeGvgrnr+HALfywj0YeDrP38sAf81/HMR/vO8G3ovn9QzgwbxobgUexPP6GuCj+Y+D+I/1YODpPH8fA3w1L5rPBj6L5+8hwK38x0D8x/op4K15/h4C3MqL5sHA03n+fhp4G/5jIP7jvDXwUzx/3wO8N/863w28F8/f2wA/zb8f4j/Gg4G/Ao7z/D0EuJV/nQcDT+f52wVeBriVfx/Ev99x4LeAl+a/1l8DrwPs8m+H+Pf7bOCz+O/xOcBn82+H+Pe7CBznv8cucIJ/O8S/3y5wjP8el4Dj/Nsh/v2+Gvgo/nt8DfDR/Nsh/mN8NfDewDH+a1wCvhv4aP59+Ed1ndXgG9xU1gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDesktopDownload;
impl IconShape for GoDesktopDownload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.927 5.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 5H8.75V.75a.75.75 0 10-1.5 0V5H5.104a.25.25 0 00-.177.427z",
            }
            path {
                d: "M1.573 2.573a.25.25 0 00-.073.177v7.5a.25.25 0 00.25.25h12.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-3a.75.75 0 110-1.5h3A1.75 1.75 0 0116 2.75v7.5A1.75 1.75 0 0114.25 12h-3.727c.099 1.041.52 1.872 1.292 2.757A.75.75 0 0111.25 16h-6.5a.75.75 0 01-.565-1.243c.772-.885 1.192-1.716 1.292-2.757H1.75A1.75 1.75 0 010 10.25v-7.5A1.75 1.75 0 011.75 1h3a.75.75 0 010 1.5h-3a.25.25 0 00-.177.073zM6.982 12a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5H6.982z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEg0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/h4MfBXw0sCD+fe5Ffhr4GOAW/mfBfG8Hgz8FXCc/1i7wMsAt/I/B+J5/TTwVvzn+BngrfmfA/G8LgLH+c9xK/AQ/udAXPFg4KuAlwYezP9ttwJ/DXwMcKuABwN/BRzn/5dd4GUE/DTwVvz/9DMCLgLH+f/pVgHmhRP/PuZ/LgSYF078+5j/uRBgXjjx72P+50KAeeHEv4/5nwsB5n+evwG+G/hrYBf4a654aeA48NLAewMvxb8PAsz/HN8DfDZwKy+aBwOfDbwX/zYIMP/9/gb4aOC3+bd5beCrgZfiXwcB5r/X7wBvDezy73Mc+GngtXjRIcD89/kd4LV5/t4KeGvgpYGX5oq/Bv4a+GngZ3j+fht4LV40CDD/Pf4GeG1gl+f01sBXAQ/mhbsV+Bjgp3lOx4HfBl6KfxkCzH+P1wF+m+f0WcBn86/z2cDn8JxeG/gt/mUIMP/1vgd4b57TZwOfxb/N5wCfzXP6buC9eOEQYP7rPQS4lWd7a+Cn+Pd5G+CnebYHA0/nhUOA+a/1N8BL85yeDjyYf59bgYfwnP4aeCleMASY/1ofA3w1z/bWwE/xH+NtgJ/m2T4b+CxeMASY/1qvA/w2z/bdwHvxH+N7gPfm2V4b+C1eMASY/1ovA/w1z/ZXwEvzH+OvgZfh2V4a+CteMASY/1riOZn/WOI5mRcMAea/lnhO5j+WeE7mBUOA+a/1MsBf82x/DbwU/zH+Bnhpnu2lgb/iBUOA+a/1OsBv82zfDbwX/zG+B3hvnu21gd/iBUOA+a/1McBX82xvDfwU/zHeBvhpnu2zgc/iBUOA+a/118DL8JxuBR7Ev88zgAfznP4KeGleMASY/3oPAW7l2d4a+Cn+fd4G+Gme7cHA03nhEGD+630P8N48p88GPot/m88BPpvn9N3Ae/HCIcD893gd4Ld5Tp8NfBb/Op8DfDbP6bWB3+JfhgDz3+OvgdcBdnlObw18NfAgXrhnAB8N/DTP6TjwV8CD+ZchwPz3+W3gdXj+3hp4a+ClgZfiir8B/hr4aeCnef5+C3htXjQIMP+9fht4G2CXf5/jwE8Br82LDgHmv99fAx8D/Db/Nq8NfBfwYP51EGD+5/hu4HOAW3nRPBj4LOC9+bdBgPmf56+B7wb+GrgE/DVXvDRwDHht4K2Bl+bfBwHm/y8EmP+/EGD+/0KA+f8LAbvAMf5/eoaAnwbeiv+ffkbAg4G/Bo7x/8sl4KXFFQ8Gvhp4aeBB/N/2DOCvgY8GbhX/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R+cztnwaLfmLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceCamera;
impl IconShape for GoDeviceCamera {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M15 3H7c0-.55-.45-1-1-1H2c-.55 0-1 .45-1 1-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h14c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM6 5H2V4h4v1zm4.5 7C8.56 12 7 10.44 7 8.5S8.56 5 10.5 5 14 6.56 14 8.5 12.44 12 10.5 12zM13 8.5c0 1.38-1.13 2.5-2.5 2.5S8 9.87 8 8.5 9.13 6 10.5 6 13 7.13 13 8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I53Qc+CjgrYGX5r/XXwM/DXwNsMu/33Hgq4C35orvBj5GPNt7A18FHOd/ll3gY4Dv5t/uo4DPBo7znL5GXPHewHfxP9v7AN/Nv85rA18FvDTP366A48DTgeP8z7YLPATY5V92HPgq4L154RDw2cBn8b/D5wCfzQv3UcBnA8f5lyHgr4CX5n+HvwZehufvtYGvAl6aFx0CzPP3MsBf89/jpYG/4vkTz+k48FXAe/OvhwDz/In/Xub5E8/2UcBnA8f5t0GAef7Efy/z/Al4beCrgJfmRfM1wEfxvBBgnj/x38s8f98NvDcvmt8BPhr4a8A8LwSY50/89zL/dpeAjwa+m2czzwsB5vkT/73Mv83XAJ8N7PKczPNCgHn+xH8v86/zO8BHA3/N82eeFwLM8yf+e5kXzSXgo4Hv5oUzzwsB5vkT/73Mv+xrgM8GdvmXmeeFAPP8if9e5gX7HeCjgb/mRWeeFwLM8yf+e5nn732A7+ZfzzwvBJjnT/z3Ms+f+LcxzwsB5vkT/73M8yf+bczzQoB5/sR/L/P8iX8b87wQYJ4/8d/LPH/i38Y8LwSY50/89zLPn/i3Mc8LAeb5E/+9zPMn/m3M80KAef7Efy/z/H008DX865nnhQDz/In/XuYF+2vgY4Df5kVnnhcCzPMn/nuZf9l3Ax8D7PIvM88LAeb5E/+9zItmF/hs4Gt44czzQoB5/sR/L/Ov89fAxwC/zfNnnhcCzPMn/nuZf5vvBj4G2OU5meeFAPP8if9e5t9uF/hs4Gt4NvO8EGCeP/Hfyzx/XwN8FC+avwY+BvhtwDwvBJjnT/z3Ms+fgJcGvhp4LV403w28N88LAeb5E/+9zPMnnu29ga8GjvFvgwDz/L0M8Nf893hp4K94/sRzOg58NvBR/Osh4K+Bl+J/h78BXprn76WBrwZeixcdAj4b+Cz+d/gc4LN54d4b+GrgGP8yBBwHbgWO8T/bJeDBwC7/suPAZwMfxQuHuOK9ge/if7a3AX6af52XBr4aeC2ev0vi2d4b+GrgGP+zXALeG/hp/u3eG/hq4BjP6WvEczoOfDTw1sBL8d/rb4CfBr4a2OXf7zjw2cB7c8V3Ax8t/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IzA6pHiKPy6kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceCameraVideo;
impl IconShape for GoDeviceCameraVideo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M16 3.75a.75.75 0 00-1.136-.643L11 5.425V4.75A1.75 1.75 0 009.25 3h-7.5A1.75 1.75 0 000 4.75v6.5C0 12.216.784 13 1.75 13h7.5A1.75 1.75 0 0011 11.25v-.675l3.864 2.318A.75.75 0 0016 12.25v-8.5zm-5 5.075l3.5 2.1v-5.85l-3.5 2.1v1.65zM9.5 6.75v-2a.25.25 0 00-.25-.25h-7.5a.25.25 0 00-.25.25v6.5c0 .138.112.25.25.25h7.5a.25.25 0 00.25-.25v-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8fy8D/DX/N7w08Fc8LwT8NfBS/P/0NwI+G/gs/n/6HAHHgVuBY/z/cgl4sLjivYHv4v+XtwF+WjzbewNfDRzj/7ZLwHsDPw0gntNx4KOBtwZeiv9b/gb4aeCrgV2uQPzneGngr3hefwO8NC/crcCDeF4vA/w1/7EQ/zneG/guntfvAK/NC/fbwGvxvN4H+G7+YyH+c3w28Fk8r68BPpoX7ruB9+J5fQ7w2fzHQvzneDrwYJ7X+wDfzQv33sB38bz+GngZ/mMh/uO9NPBXPH8PAW7lhXsw8HSev5cB/pr/OIj/eN8NvBfP6xnAg3nR3Ao8iOf1NcBH8x8H8R/rwcDTef4+BvhqXjSfDXwWz99DgFv5j4H4j/VTwFvz/D0EuJUXzYOBp/P8/TTwNvzHQPzHeWvgp3j+vgd4b/51vht4L56/twF+mn8/xH+MBwN/BRzn+XsIcCv/Og8Gns7ztwu8DHAr/z6If7/jwG8BL81/rb8GXgfY5d8O8e/31cBH8d/ja4CP5t8O8e93ETjOf49d4AT/doh/v13gGP89LgHH+bdD/Pt9NfBR/Pf4GuCj+bdD/Mf4auC9gWP817gEfDfw0fz78I93cZXg7AhT5wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceDesktop;
impl IconShape for GoDeviceDesktop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5h12.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25zM14.25 1H1.75A1.75 1.75 0 000 2.75v7.5C0 11.216.784 12 1.75 12h3.727c-.1 1.041-.52 1.872-1.292 2.757A.75.75 0 004.75 16h6.5a.75.75 0 00.565-1.243c-.772-.885-1.193-1.716-1.292-2.757h3.727A1.75 1.75 0 0016 10.25v-7.5A1.75 1.75 0 0014.25 1zM9.018 12H6.982a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzP8tfAz8NfA2wy4sG8aJ7b+CrgOP8z7YLfAzw3fzLEC+a9wa+i/9d3gf4bl44xL/sOPB04Dj/u+wCDwF2ecEQ/7LPBj6L/50+B/hsXjDEv+yvgJfmf6e/Bl6GFwzxLzPP38sAf83/DC8N/BXPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4t/uwcBXAS/NFX8NfAxwK/925vkTLxjiX2aeP/Fv82Dgr4DjPKdd4GWAW/m3Mc+feMEQ/zLz/Il/m58G3orn72eAt+bfxjx/4gVD/MvM8yf+bS4Cx3n+bgUewr+Nef7EC4b4l5nnT/zb7ALHeP6eATyYfxvz/IkXDPEvM8+f+Lf5aeCteP5+Bnhr/m3M8ydeMMS/zDx/4t/mwcBfA8d4TpeAlwZu5d/GPH/iBUP8y8zzJ/7tHgx8NfDSXPHXwEcDt/JvZ54/8YIh/mXm+RP/s5jnT7xgiH+Zef7E/yzm+RMvGOJfZp4/8T+Lef7EC4b4l5nnT/zPYp4/8YIh/mXm+RP/s5jnT7xgiH+Zef5eBvhr/md4aeCveP7EC4b4l/018FL87/Q3wEvzgiH+ZZ8NfBb/O30O8Nm8YIh/2XHgVuAY/7tcAh4M7PKCIV407w18F/+7vA3w07xwiBfdewNfDRzjf7ZLwHsDP82/DPGvcxz4aOCtgZfif5a/AX4a+GpglxcN/wh5/J5B9J8iIQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDeviceMobile;
impl IconShape for GoDeviceMobile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.75 0A1.75 1.75 0 002 1.75v12.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 14.25V1.75A1.75 1.75 0 0012.25 0h-8.5zM3.5 1.75a.25.25 0 01.25-.25h8.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25V1.75zM8 13a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiamond;
impl IconShape for GoDiamond {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M.527 9.237a1.75 1.75 0 010-2.474L6.777.512a1.75 1.75 0 012.475 0l6.251 6.25a1.751 1.751 0 010 2.475l-6.25 6.251a1.751 1.751 0 01-2.475 0L.527 9.238v-.001zm1.06-1.414a.25.25 0 000 .354l6.251 6.25a.25.25 0 00.354 0l6.25-6.25a.25.25 0 000-.354l-6.25-6.25a.25.25 0 00-.354 0l-6.25 6.25h-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/HseBzwLemiu+G/gc/ush/nt8NfBRPKevAT6a/1qI/x4XgeM8p1uBh/BfC/Hfwzx/4r8W4r+Hef7Efy3Efw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfjvYZ4/8V8L8d/DPH/ivxbiv4d5/sR/LcR/D/P8if9aiP8e5vkT/7UQ/z3M8yf+ayH+bY4DnwW8NfBg/nvdCnw38Dn86yH+bb4a+Cj+Z/ka4KP510H821wEjvM/y63AQ/jXQfzb7ALH+J/lGcCD+ddB/Nt8NvBZ/M/yOcBn86+D+Lf7auCtgQfx3+sZwHcDn82/HuK/h3n+xH8txH8P8/yJ/1qI/x7m+RP/tRD/PczzJ/5rIf57mOdP/NdC/Pcwz5/4r4X472GeP/FfC/Hfwzx/4r8W4r+Hef7Efy3Efw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L899gFjvGcngE8mP9aiP8enw18Fs/pc4DP5r8W4r/PVwNvzRXfDXw2//UQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/zbHAc+C3hr4MH897oV+G7gc/jXQ/zbfDXwUfzP8jXAR/Ovg/i3uQgc53+WW4GH8K+D+LfZBY7xP8szgAfzr4P4t/ls4LP4n+VzgM/mXwfxb/fVwFsDD+K/1zOA7wY+m389xP9viP/fEP+/If5/Q/z/hvj/jX8EGHVCQQ0dDwsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiff;
impl IconShape for GoDiff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.75 1.75a.75.75 0 00-1.5 0V5H4a.75.75 0 000 1.5h3.25v3.25a.75.75 0 001.5 0V6.5H12A.75.75 0 0012 5H8.75V1.75zM4 13a.75.75 0 000 1.5h8a.75.75 0 100-1.5H4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/iPcxz4LOCtueK7gc/hP5Z5/sSzIZ6Xef7Ef5yvBj6K5/Q1wEfzH8c8f+LZEM/LPH/iP85F4DjP6VbgIfzHMc+feDbE8zLPn/iPY54/8R/HPH/i2RDPyzx/4j+Oef7Efxzz/IlnQzwv8/yJ/zjm+RP/cczzJ54N8bzM8yf+45jnT/zHMc+feDbE8zLPn/iPY54/8R/HPH/i2RDPyzx/4j+Oef7Efxzz/IlnQzwv8/yJ/zjm+RP/cczzJ54N8bzM8yf+45jnT/zHMc+feDbE8zLPn/iPY54/8R/HPH/i2RDPyzx/4j+Oef7Efxzz/IlnQzwv8/yJ5+848FnAWwMP5r/GrcB3A5/DC2aeP/FsiOdlnj/x/H018FH89/ga4KN5/szzJ54N8bzM8yeev4vAcf573Ao8hOfPPH/i2RDPyzx/4vnbBY7x3+MZwIN5/szzJ54N8bzM8yeev88GPov/Hp8DfDbPn3n+xLMhnpd5/sQL9tXAWwMP4r/GM4DvBj6bF8w8f+LZEM/LPH/iP455/sR/HPP8iWdDPC/z/In/OOb5E/9xzPMnng3xvMzzJ/7jmOdP/Mcxz594NsTzMs+f+I9jnj/xH8c8f+LZEM/LPH/iP455/sR/HPP8iWdDPC/z/In/OOb5E/9xzPMnng3xvMzzJ/7jmOdP/Mcxz594NsTzMs+f+I9jnj/xH8c8f+LZEM/LPH/iP455/sR/HPP8iWdDPC/z/In/OOb5E/9xzPMnng3xvMzzJ/7j7ALHeE7PAB7Mfxzz/IlnQzwv8/yJ/zifDXwWz+lzgM/mP455/sSzIZ6Xef7Ef6yvBt6aK74b+Gz+Y5nnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J5medP/O9inj/xbIjnZZ6/lwH+mv8dXhr4K54/8WyI5/XXwEvxf9PfAC/NsyGe12cDn8X/TZ8DfDbPhnhex4FbgWP833IJeDCwy7Mhnr/3Br6L/1veBvhpnhPiBXtv4KuBY/zvdgl4b+CneV6IF+448NHAWwMvxf8ufwP8NPDVwC7PH+L/N8T/b4j/3xD/v/GPjjqoQXwTMxYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffAdded;
impl IconShape for GoDiffAdded {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13.25 2.5H2.75a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25zM2.75 1h10.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25V2.75C1 1.784 1.784 1 2.75 1zM8 4a.75.75 0 01.75.75v2.5h2.5a.75.75 0 010 1.5h-2.5v2.5a.75.75 0 01-1.5 0v-2.5h-2.5a.75.75 0 010-1.5h2.5v-2.5A.75.75 0 018 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE6UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/Pp8FvDVXfDfwNfzLzPMnng3xvMzzJ/57fBfw3jynjwG+mhfOPH/i2RDPyzx/4r/edwHvzfP6a+BleOHM8yeeDfG8zPMn/mt9F/DePH9/A7w0L5x5/sSzIZ6Xef7Ef53vAt6bF+xzgM/mhTPPn3g2xPMyz5/4r/FdwHvzgn0P8N78y8zzJ54N8bzM8yf+830X8N68YN8DvDcvGvP8iWdDPC/z/In/XN8FvDcv2PcA782Lzjx/4tkQz8s8f+I/z3cB780L9j3Ae/OvY54/8WyI52WeP/Gf47uA9+YF+x7gvfnXM8+feDbE8zLPn/iP913Ae/OCfQ/w3vzbmOdPPBvieZnnT/zH+i7gvXnBvgd4b/7tzPMnng3xvMzzJ/7jfBfw3rxg3wO8N/8+5vkTz4Z4Xub5E/8xvgt4b16w7wHem38/8/yJZ0M8L/P8iX+/7wLemxfse4D35j+Gef7EsyGel3n+xL/PdwHvzQv2PcB78x/HPH/i2RDPyzx/4t/uu4D35gX7HuC9+Y9lnj/xbIjnZZ4/8W/zXcB784J9D/De/Mczz594NsTzMs+f+Nf7LuC9ecG+B3hv/nOY5088G+J5medP/Ot8F/DevGDfA7w3/3nM8yeeDfG8zPMnXnTfBbw3L9j3AO/Nfy7z/IlnQzwv8/yJF813Ae/NC/Y9wHvzn888f+LZEM/LPH/iX/ZdwHvzgn0P8N781zDPn3g2xPMyz5944b4a+ChesO8B3pv/Oub5E8+GeF7m+RMvnHnBvgd4b/5rmedPPBvieZnnT7xw5vn7HuC9+a9nnj/xbIjnZZ4/8cL9NPBWPKfvAd6b/x7m+RPPhnhe5vkTL9xx4LuBtwIuAd8NfDT/fczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8vQzw1/zv8NLAX/H8iWdDPK+/Bl6K/5v+Bnhpng3xvD4b+Cz+b/oc4LN5NsTzOg7cChzj/5ZLwIOBXZ4N8fy9N/Bd/N/yNsBP85wQL9h7A18NHON/t0vAewM/zfNCvHDHgY8G3hp4Kf53+Rvgp4GvBnZ5/hD/vyH+f0P8/4b4/41/BAUGs0FdQPZXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffIgnored;
impl IconShape for GoDiffIgnored {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-1.97 4.78a.75.75 0 00-1.06-1.06l-5.5 5.5a.75.75 0 101.06 1.06l5.5-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+I/xYOCjgNcGXpor/hr4beBrgFv5j2GeP/FsiOdlnj/x7/dVwEfzwn018DH8+5nnTzwb4nmZ50/8+/wV8NK8aP4aeBn+fczzJ54N8bzM8yf+7b4a+Cj+db4G+Gj+7czzJ54N8bzM8yf+bR4MPJ1/m4cAt/JvY54/8WyI52WeP/Fv89XAR/Fv8zXAR/NvY54/8WyI52WeP/Fv81fAS/Nv89fAy/BvY54/8WyI52WeP/FvY/59xL+Nef7EsyGel3n+xL+N+fcR/zbm+RPPhnhe5vkT/zZ/DbwU/zZ/A7w0/zbm+RPPhnhe5vkT/zZfDXwU/zZfA3w0/zbm+RPPhnhe5vkT/zYPBp7Ov81DgFv5tzHPn3g2xPMyz5/4t/tq4KP41/ka4KP5tzPPn3g2xPMyz5/49/lr4KV40fwN8NL8+5jnTzwb4nmZ50/8+3018FG8cF8DfDT/fub5E8+GeF7m+RP/MR4MfDTw2sBLccXfAL8NfDVwK/8xzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLP38sAf83/Di8N/BXPn3g2xPP6a+Cl+L/pb4CX5tkQz+uzgc/i/6bPAT6bZ0M8r+PArcAx/m+5BDwY2OXZEM/fewPfxf8tbwP8NM8J8YK9N/DVwDH+d7sEvDfw0zwvxAt3HPho4K2Bl+J/l78Bfhr4amCX5w/x/xvi/zfE/2+I/9/4R68knEFWp7EDAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffModified;
impl IconShape for GoDiffModified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zM8 10a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+L5Ow58FvDWwIP5r3Er8N3A5/CCmedPPBvieZnnTzx/Xw18FP89vgb4aJ4/8/yJZ0M8L/P8iefvInCc/x63Ag/h+TPPn3g2xPMyz594/naBY/z3eAbwYJ4/8/yJZ0M8L/P8iefvs4HP4r/H5wCfzfNnnj/xbIjnZZ4/8YJ9NfDWwIP4r/EM4LuBz+YFM8+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/y9DPDX/O/w0sBf8fyJZ0M8r78GXor/m/4GeGmeDfG8Phv4LP5v+hzgs3k2xPM6DtwKHOP/lkvAg4Fdng3x/L038F383/I2wE/znBAv2HsDXw0c43+3S8B7Az/N80K8cMeBjwbeGngp/nf5G+Cnga8Gdnn+EP+/If5/Q/z/hvj/jX8EzmeIQRdcS1AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffRemoved;
impl IconShape for GoDiffRemoved {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-2 7.75a.75.75 0 000-1.5h-6.5a.75.75 0 000 1.5h6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/iP9VHAe3PFTwOfw38s8/yJZ0M8L/P8if84Hw18Fc/pu4H34T+Oef7EsyGel3n+xH+cvwJemuf13cD78B/DPH/i2RDPyzx/4j/OXwMvxfP33cD78O9nnj/xbIjnZZ4/8R/ns4HP4gX7buB9+Pcxz594NsTzMs+f+I/13cB78YJ9N/A+/NuZ5088G+J5medP/Mf7buC9eMG+G3gf/m3M8yeeDfG8zPMn/nN8N/BevGDfDbwP/3rm+RPPhnhe5vkT/3m+G3gvXrDvBt6Hfx3z/IlnQzwv8/yJ/1zfDbwXL9h3A+/Di848f+LZEM/LPH/iP993A+/FC/bdwPvwojHPn3g2xPMyz5/4r/HdwHvxgn038D78y8zzJ54N8bzM8yf+63w38F68YN8NvA8vnHn+xLMhnpd5/sTzdxz4LOCtgQfzX+e7gffhBTPPn3g2xPMyz594/r4a+Cj+e3wO8Nk8f+b5E8+GeF7m+RPP30XgOP89bgUewvNnnj/xbIjnZZ4/8fztAsf47/EM4ME8f+b5E8+GeF7m+RPP32cDn8V/j88BPpvnzzx/4tkQz8s8f+IF+2rgrYEH8V/ne4D35gUzz594NsTzMs+f+K/zXcB784J9D/DevHDm+RPPhnhe5vkT/zW+C3hvXrDvAd6bf5l5/sSzIZ6Xef7Ef77vAt6bF+x7gPfmRWOeP/FsiOdlnj/xn+u7gPfmBfse4L150ZnnTzwb4nmZ50/85/ku4L15wb4HeG/+dczzJ54N8bzM8yf+c3wX8N68YN8DvDf/eub5E8+GeF7m+RP/8b4LeG9esO8B3pt/G/P8iWdDPC/z/In/WN8FvDcv2PcA782/nXn+xLMhnpd5/sR/nK8GPooX7HuA9+bfxzx/4tkQz8s8f+I/jnnBvgd4b/79zPMnng3xvMzzJ/7jmOfve4D35j+Gef7EsyGel3n+xH+cnwbeiuf0PcB78x/HPH/i2RDPyzx/4j/OceC7gbcCLgHfDXw0/7HM8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLP38sAf83/Di8N/BXPn3g2xPP6a+Cl+L/pb4CX5tkQz+uzgc/i/6bPAT6bZ0M8r+PArcAx/m+5BDwY2OXZEM/fewPfxf8tbwP8NM8J8YK9N/DVwDH+d7sEvDfw0zwvxAt3HPho4K2Bl+J/l78Bfhr4amCX5w/x/xvi/zfE/2+I/9/4R5yvu0EffB/wAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDiffRenamed;
impl IconShape for GoDiffRenamed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75a.25.25 0 01.25-.25zM13.25 1H2.75A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1zm-1.47 7.53a.75.75 0 000-1.06L8.53 4.22a.75.75 0 00-1.06 1.06l1.97 1.97H4.75a.75.75 0 000 1.5h4.69l-1.97 1.97a.75.75 0 101.06 1.06l3.25-3.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K/1VsBbAw8GjgMvzRV/DewCtwI/DfwM/zUQ//keDHwW8NbAcV40u8BPAx8D7PKfB/Gf67OAjwaO82+zC3w18Dn850D85zgO/Bbw0vzH+GvgdYBd/mMh/uO9NPBbwHH+Y+0CrwP8Nf9xEP+xjgNPB47zn2MXeAiwy38MxH+c48BvAS/NC/c9wG8Dfw38NVe8NPDSwFsDb8UL99fAy/AfA/Ef57OBz+IF+xngo4FbeeEeDHw18Fa8YJ8DfDb/foj/GA8G/go4zvP3McBX86/z0cBX8fztAg8Bdvn3QfzH+G7gvXj+Pgb4av5tPhr4Kp6/7wHem38fxH+Mi8BxntfPAG/Nv89PA2/F89oFTvDvg/j3e2vgp3j+HgLcyr/Pg4Gn8/y9DfDT/Nsh/v2+G3gvntf3AO/Nf4yfBt6K5/U1wEfzb4f49/tt4LV4Xu8DfDf/Md4b+C6e1+8Ar82/HeLf76+Al+Z5vQzw1/zHeGngr3hefw28DP92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIf7+/Bl6K5/UywF/zH+Olgb/ief0N8NL82yH+/X4beC2e1/sA381/jPcGvovn9TvAa/Nvh/j3+27gvXhe3wO8N/8xfhp4K57X1wAfzb8d4t/vrYGf4vl7CHAr/z4PBp7O8/c2wE/zb4f4j7ELHON5/TTwNvz7/BTw1jyvS8Bx/n0Q/zG+G3gvnr+PAb6af5uPBr6K5+97gPfm3wfxH+M4cCtwjOfvo4Gv4V/no4Cv5vm7BDwY2OXfB/Ef57OBz+IF+2ngY4BbeeEeDHwV8Na8YJ8DfDb/foj/WH8NvBQv3E8DPw38DfDXXPHSwEsBbw28NS/c3wAvzX8MxH+s48CtwDH+c1wCHgzs8h8D8R/vpYHfBo7xH+sS8NrAX/MfB/Gf4zjw28BL8R/jb4DXBnb5j4X4z/XZwEcDx/i3uQR8NfDZ/OdA/Oc7Dnw18NbAMV40l4CfBj4a2OU/D+K/1lsDbw08GDgOvBRX/A2wC/w18NvAT/NfA/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwSEG35BGTBORgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDot;
impl IconShape for GoDot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 5.5a2.5 2.5 0 100 5 2.5 2.5 0 000-5zM4 8a4 4 0 118 0 4 4 0 01-8 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADIElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+K/1VsBbAw8GjgMvzRV/DewCtwI/DfwM/zUQ//keDHwW8NbAcV40u8BPAx8D7PKfB/Gf67OAjwaO82+zC3w18Dn850D85zgO/Bbw0vzH+GvgdYBd/mMh/uO9NPBbwHH+Y+0CrwP8Nf9xEP+xjgNPB47zn2MXeAiwy38MxH+c48BvAS/Nf66/Bl6G/xiI/zifDXwW/zU+B/hs/v0Q/zEeDPwVcJz/GrvAQ4Bd/n0Q/zG+G3gv/mt9D/De/Psg/mNcBI7zX2sXOMG/D+Lf762Bn+K/x9sAP82/HeLf77uB9+K/x9cAH82/HeLf77eB1+K/x+8Ar82/HeLf76+Al+a/x18DL8O/HeLfz/z3Ev92iH8/899L/Nsh/v3+Gngp/nv8DfDS/Nsh/v1+G3gt/nv8DvDa/Nsh/v2+G3gv/nt8DfDR/Nsh/v3eGvgp/nu8DfDT/Nsh/mPsAsf4r3UJOM6/D+I/xncD78V/re8B3pt/H8R/jOPArcAx/mtcAh4M7PLvg/iP89nAZ/Ff43OAz+bfD/Ef66+Bl+I/198AL81/DMR/rOPArcAx/nNcAh4M7PIfA/Ef76WB3waO8R/rEvDawF/zHwfxn+M48NvAS/Ef42+A1wZ2+Y+F+M/12cBHA8f4t7kEfDXw2fznQPznOw58NfDWwDFeNJeAnwY+GtjlPw/iv9ZbA28NPBg4DrwUV/wNsAv8NfDbwE/zXwPx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8EVwhSQVmRtK0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDotFill;
impl IconShape for GoDotFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 4a4 4 0 100 8 4 4 0 000-8z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/HseBzwLemiu+G/gc/ush/nt8NfBRPKevAT6a/1qI/x4XgeM8p1uBh/BfC/Hfwzx/4r8W4r+Hef7Efy3Efw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfjvYZ4/8V8L8d/DPH/ivxbiv4d5/sR/LcR/D/P8if9aiP8e5vkT/7UQ/z3M8yf+ayH+e5jnT/zXQvz3MM+f+K+F+O9hnj/xXwvx7/NRwHtzxU8Dn8OLxjx/4kXzWcBbc8V3A1/Dvw3i3+6jga/iOX038D78y8zzJ/5l3wW8N8/pY4Cv5l8P8W/3V8BL87y+G3gfXjjz/IkX7ruA9+Z5/TXwMvzrIf7t/hp4KZ6/7wbehxfMPH/iBfsu4L15/v4GeGn+9RD/dp8NfBYv2HcD78PzZ54/8fx9F/DevGCfA3w2/3qIf5/vBt6LF+y7gffheZnnTzyv7wLemxfse4D35t8G8e/33cB78YJ9N/A+PCfz/Inn9F3Ae/OCfQ/w3vzbIf5jfDfwXrxg3w28D89mnj/xbN8FvDcv2PcA782/D+I/zncD78UL9t3A+3CFef7EFd8FvDcv2PcA782/H+I/1ncD78UL9t3A+wDm+RPwXcB784J9D/De/MdA/Mf7buC9eMG+G3hvnr/vBt6bF+x7gPfmPw7iP8d3A+/Ff6zvAd6b/1iI/zzfDbwX/zG+B3hv/uMh/nN9N/Be/Pt8D/De/OdA/Of7buC9+Lf5HuC9+c+D+K/x3cB78a/zPcB7858L8V/nu4H34kXzPcB7858P8V/ru4H34oX7HuC9+a+B+K/33cB78fx9D/De/NdB/Pf4buC9eE7fA7w3/7UQ/30+G3hvrvhu4LP5r4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+Nc7DnwW8NbAg/mf4Vbgu4HP4V8H8a/31cBH8T/T1wAfzYsO8a93ETjO/0y3Ag/hRYf419sFjvE/0zOAB/OiQ/zrfTbwWfzP9DnAZ/OiQ/zbfDXw1sCD+J/hGcB3A5/Nvw7i/zfE/2+I/98Q/78h/n9D/P/GPwLrB25BACFz5QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDownload;
impl IconShape for GoDownload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.47 10.78a.75.75 0 001.06 0l3.75-3.75a.75.75 0 00-1.06-1.06L8.75 8.44V1.75a.75.75 0 00-1.5 0v6.69L4.78 5.97a.75.75 0 00-1.06 1.06l3.75 3.75zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE90lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdRz4KOCtgZfmv9dfAz8NfA2wyxWI/zzvDXwVcJz/WXaBjwG+G0D853hv4Lv4n+19gO8W//GOA08HjvM/2y7wEPEf77OBz+J/h88R//H+Cnhp/nf4a/Efzzx/LwP8Nf89Xhr4K54X4j+eef7Efy/zvBD/8czzJ/57meeF+I9nnj/x38s8L8R/PPP8if9e5nkh/uOZ50/89zLPC/Efzzx/4l/vOPBZwFtzxXcDn8O/jXleiP945vkT/3pfDXwUz+lrgI/mX888L8R/PPP8iX+9i8BxntOtwEP41zPPC/Efzzx/4l/PPH/iX888L8R/PPP8iX898/yJfz3zvBD/8czzJ/71zPMn/vXM80L8xzPPn/jXM8+f+NczzwvxH888f+L5Ow58FvDWwIP5t7kV+G7gc3jBzPNCPKfjwEcBbw28NC/YrcB3A5/D8zLPn3j+vhr4KP5jfA3w0Tx/5nkhnu29ga8CjvOi+xrgo3lO5vkTz99F4Dj/MW4FHsLzZ54X4or3Br6Lf71bgYfwnMzzJ56/XeAY/zGeATyY5888LwQcB54OHOdf7xnAg3lO5vkTz99nA5/Ff4zPAT6b5888LwR8NvBZ/Nt8DvDZPCfz/IkX7KuBtwYexL/NM4DvBj6bF8w8LwT8FfDS/Os8A/hu4LN5Xub5E/965vkT/3rmeSHAPH8vA/w1/3rm+RP/eub5E/965nkhwDx/4t/GPH/iX888f+JfzzwvBJjnT/zbmOdP/OuZ50/865nnhQDz/Il/G/P8iX+9XeAYz+kZwIP51zPPCwHm+RP/Nub5E/96nw18Fs/pc4DP5l/PPC8EmOdP/NuY50/823w18NZc8d3AZ/NvY54XAszzJ/5tzPMn/nuZ54UA8/yJfxvz/In/XuZ5IcA8f+Lfxjx/4r+XeV4IMM+f+Lcxz5/472WeFwLM8yf+bczzJ/57meeFAPP8iX8b8/y9DPDX/Pd4aeCveF4IMM+f+Lf5a+Cl+N/hbwSY50/823w28Fn87/A5AszzJ/5tjgO3Asf4n+0S8GAB5vkT/3bvDXwX/7O9DfDTAszzJ/593hv4auAY/7NcAt4b+GkAAeb5E/9+x4GPBt4aeCn+e/0N8NPAVwO7XIEA8/yJ//sQYJ4/8X8fAszzJ/7vQ4B5/sT/fQgwz5/473Uc+Czgrbniu4HP4T8WAszzJ/57fTXwUTynrwE+mv84CDDPn/jvdRE4znO6FXgI/3EQYJ4/8d/LPH/iPw4CzPMn/nuZ50/8x0GAef7Efy/z/In/OAgwz5/472WeP/EfBwHm+RP/vczzJ/7jIMA8f+K/l3n+xH8cBJjn72WAv+a/x0sDf8XzJ/7jIOCvgZfif4e/AV6a/zgI+Gzgs/jf4XOAz+Y/DgKOA7cCx/if7RLwYGCX/ziIK94b+C7+Z3sb4Kf5j4V4tvcGvho4xv8sl4D3Bn6a/3iI53Qc+GjgrYGX4r/X3wA/DXw1sMt/Dv4RuLzN4BsB18MAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoDuplicate;
impl IconShape for GoDuplicate {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.5 3a.75.75 0 01.75.75v1h1a.75.75 0 010 1.5h-1v1a.75.75 0 01-1.5 0v-1h-1a.75.75 0 010-1.5h1v-1A.75.75 0 0110.5 3z",
            }
            path {
                d: "M6.75 0A1.75 1.75 0 005 1.75v7.5c0 .966.784 1.75 1.75 1.75h7.5A1.75 1.75 0 0016 9.25v-7.5A1.75 1.75 0 0014.25 0h-7.5zM6.5 1.75a.25.25 0 01.25-.25h7.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-7.5a.25.25 0 01-.25-.25v-7.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.75 5A1.75 1.75 0 000 6.75v7.5C0 15.216.784 16 1.75 16h7.5A1.75 1.75 0 0011 14.25v-1.5a.75.75 0 00-1.5 0v1.5a.25.25 0 01-.25.25h-7.5a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1.5a.75.75 0 000-1.5h-1.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeE7HgY8C3hp4af5v+Wvgp4GvAXa5AvFs7w18FXCc/9t2gY8BvhtAXPHewHfx/8v7AN8t4DjwdOA4/7/sAg8R8NnAZ/H/0+cI+Cvgpfn/6a8FmP+/EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD513sG8NHAX3PFSwNfDTyIF80zgI8G/porXhr4auBBvGieAXw08Ndc8dLAVwMP4l8HAeZf5xnASwO7PKfjwF8DD+KFewbw0sAuz+k48NfAg3jhngG8NLDLczoO/DXwIF50CDD/Om8D/DTP31sDP8UL9zbAT/P8vTXwU7xwbwP8NM/fWwM/xYsOAeZf5wSwy/P3YODpvHAngF2evwcDT+eFOwHs8vw9GHg6LzoEmH+dE8Auz9+Dgafzwp0Adnn+Hgw8nRfuBLDL8/dg4Om86BBg/nXeBvhpnr+3Bn6KF+5tgJ/m+Xtr4Kd44d4G+Gmev7cGfooXHQLMv86twMsAuzyn48BfAQ/mhbsVeBlgl+d0HPgr4MG8cLcCLwPs8pyOA38FPJgXHQLMv96twEcDf8MVLwV8NfBgXjS3Ah8N/A1XvBTw1cCDedHcCnw08Ddc8VLAVwMP5l8HAeb/LwSY/78QYP7/QoD5/wsB5v8vBJj/vxDw18BL8f/T3wj4bOCz+P/pcwQcB24FjvH/yyXgweKK9wa+i/9f3gb4afFs7w18NXCM/9suAe8N/DSAeE7HgY8G3hp4Kf5v+Rvgp4GvBna5AvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwQHPGzsO23magAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEllipsis;
impl IconShape for GoEllipsis {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 5.75C0 4.784.784 4 1.75 4h12.5c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0114.25 12H1.75A1.75 1.75 0 010 10.25v-4.5zM4 7a1 1 0 100 2 1 1 0 000-2zm3 1a1 1 0 112 0 1 1 0 01-2 0zm5-1a1 1 0 100 2 1 1 0 000-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEye;
impl IconShape for GoEye {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.679 7.932c.412-.621 1.242-1.75 2.366-2.717C5.175 4.242 6.527 3.5 8 3.5c1.473 0 2.824.742 3.955 1.715 1.124.967 1.954 2.096 2.366 2.717a.119.119 0 010 .136c-.412.621-1.242 1.75-2.366 2.717C10.825 11.758 9.473 12.5 8 12.5c-1.473 0-2.824-.742-3.955-1.715C2.92 9.818 2.09 8.69 1.679 8.068a.119.119 0 010-.136zM8 2c-1.981 0-3.67.992-4.933 2.078C1.797 5.169.88 6.423.43 7.1a1.619 1.619 0 000 1.798c.45.678 1.367 1.932 2.637 3.024C4.329 13.008 6.019 14 8 14c1.981 0 3.67-.992 4.933-2.078 1.27-1.091 2.187-2.345 2.637-3.023a1.619 1.619 0 000-1.798c-.45-.678-1.367-1.932-2.637-3.023C11.671 2.992 9.981 2 8 2zm0 8a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoEyeClosed;
impl IconShape for GoEyeClosed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M.143 2.31a.75.75 0 011.047-.167l14.5 10.5a.75.75 0 11-.88 1.214l-2.248-1.628C11.346 13.19 9.792 14 8 14c-1.981 0-3.67-.992-4.933-2.078C1.797 10.832.88 9.577.43 8.9a1.618 1.618 0 010-1.797c.353-.533.995-1.42 1.868-2.305L.31 3.357A.75.75 0 01.143 2.31zm3.386 3.378a14.21 14.21 0 00-1.85 2.244.12.12 0 00-.022.068c0 .021.006.045.022.068.412.621 1.242 1.75 2.366 2.717C5.175 11.758 6.527 12.5 8 12.5c1.195 0 2.31-.488 3.29-1.191L9.063 9.695A2 2 0 016.058 7.52l-2.53-1.832zM8 3.5c-.516 0-1.017.09-1.499.251a.75.75 0 11-.473-1.423A6.23 6.23 0 018 2c1.981 0 3.67.992 4.933 2.078 1.27 1.091 2.187 2.345 2.637 3.023a1.619 1.619 0 010 1.798c-.11.166-.248.365-.41.587a.75.75 0 11-1.21-.887c.148-.201.272-.382.371-.53a.119.119 0 000-.137c-.412-.621-1.242-1.75-2.366-2.717C10.825 4.242 9.473 3.5 8 3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFY0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdxx4LeClgZcGjgMPBh7Mc7oVuBXYBf4a+Gvgd4Bd/vMg/nO8NPBewGsDL82/z18Dvw18D/DX/MdC/Mc5DrwX8NHAg/nPcSvw1cD3ALv8+yH+/Y4DHwV8NHCc/xq7wFcDXwPs8m+H+Pd5L+CrgeP899gFPhr4Hv5tEP82Dwa+C3ht/mf4beB9gFv510H867018F3Acf5n2QXeB/hpXnSIf52vAj6a/9m+GvgYXjSIF913Ae/N/w7fDbwP/zLEi+angLfmf5efBt6GFw7xL/su4L353+m7gffhBUO8cF8NfBT/u30N8NE8f4gX7K2Bn+L/hrcBfprnhXj+Hgz8FXCc/xt2gZcBbuU5IZ6/3wJem/9bfht4HZ4T4nm9N/Bd/N/0PsB382yI53QceDpwnH+dZwAfDfw1cCv/uR4MvDTw1cCD+NfZBR4C7HIF4jl9NvBZ/Os8A3hpYJf/WseBvwYexL/O5wCfzRWIZzsOPB04zr/O2wA/zX+PtwZ+in+dXeAhwC6AeLaPBr6Kf70TwC7/PR4MPJ1/vY8BvhpAPNvTgQfzryf+e5l/vVuBhwCIK14a+Cv+bcR/L/Nv8zLAX4srvhr4KP5txH8v82/zNcBHiyv+Cnhp/m3Efy/zb/PXwMsIOA5c5N9O/Pcy/3YnBLw18FP824n/Xubf7m0EfDbwWfzbif9e5t/ucwT8NPBW/NuJ/17m3+5nBPw28Fr824n/Xubf7ncEPB14MP924r+X+bf7awHm30f89zL/dggw/z7iv5f5t0OA+fcR/73Mvx0CzL+P+O9l/u0QcCvwIP7txH8v82/3DAG/DbwW/3biv5f5t/sdAT8NvBX/duK/l/m3+x4Bnw18Fv924r+X+bf7HAFvDfwU/3biv5f5t3sbAceBi/zbif9e5t/uhLjir4GX4t/mBLDLf48HA0/n3+ZvgJcWV3w18FH827wN8NP893hr4Kf4t/ka4KPFFS8N/BX/NrcCLwPs8l/rOPBXwIP5t3kZ4K/Fs90KPIh/m1uBjwb+BriV/1wPBl4K+GrgwfzbPAN4MIB4to8Gvor/eOJF813Ae/Nf42OArwYQz3YcuBU4xn8s8S/7LuC9+a9xCXgwsAsgntNnA5/Ffyzxwn0X8N781/kc4LO5AvGcjgO3Asf4jyNesO8C3pv/OpeABwO7XIF4Xu8NfBf/ccTz913Ae/Nf632A7+bZEM/fbwOvxX8M8by+C3hv/mv9DvDaPCfE8/dg4K+BY/z7nQB2ebbvAt6b/1qXgJcGbuU5IV6wtwZ+in+/7wY+BjgOfBXw1vzXexvgp3leiBfuq4GP4n+3rwE+mucP8S/7buC9+N/pe4D35gVDvGh+Gngr/nf5HuC9eeEQL7rvBt6L/x2+B3hv/mWIf52vBj6K/9m+BvhoXjSIf723Br4bOMb/LJeA9wZ+mhcd4t/mwcB3A6/F/wy/A7w3cCv/Ooh/n/cGvho4xn+PS8BHA9/Nvw3i3+848NHARwPH+K9xCfhq4KuBXf7tEP9xjgPvDXw08CD+czwD+Grgu4Fd/v0Q/zleGnhv4LWBl+Lf52+A3wa+G/hr/mMh/vMdB14beGngpYHjwIOBB/GcngHcCuwCfw38NfDbwC7/efhH+GTMfEFWVOgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedDiscussion;
impl IconShape for GoFeedDiscussion {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM4 5a1 1 0 011-1h6a1 1 0 011 1v5a1 1 0 01-1 1H8.707l-1.853 1.854A.5.5 0 016 12.5V11H5a1 1 0 01-1-1V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedForked;
impl IconShape for GoFeedForked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM6 6.928a1.75 1.75 0 10-1 0V7.5A1.5 1.5 0 006.5 9h1v1.072a1.75 1.75 0 101 0V9h1A1.5 1.5 0 0011 7.5v-.572a1.75 1.75 0 10-1 0V7.5a.5.5 0 01-.5.5h-3a.5.5 0 01-.5-.5v-.572z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedHeart;
impl IconShape for GoFeedHeart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm2.33-11.5c-1.22 0-1.83.5-2.323 1.136C7.513 5 6.903 4.5 5.682 4.5c-1.028 0-2.169.784-2.169 2.5 0 1.499 1.493 3.433 3.246 4.517.52.321.89.479 1.248.484.357-.005.728-.163 1.247-.484C11.007 10.433 12.5 8.5 12.5 7c0-1.716-1.14-2.5-2.17-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedMerged;
impl IconShape for GoFeedMerged {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.25-11.25a1.75 1.75 0 01-1.207 1.664A2 2 0 009 8h.571a1.75 1.75 0 110 1H9a2.99 2.99 0 01-2-.764v1.336a1.75 1.75 0 11-1 0V6.428A1.75 1.75 0 118.25 4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedPerson;
impl IconShape for GoFeedPerson {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.847-8.145a2.502 2.502 0 10-1.694 0C5.471 8.261 4 9.775 4 11c0 .395.145.995 1 .995h6c.855 0 1-.6 1-.995 0-1.224-1.47-2.74-3.153-3.145z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedRepo;
impl IconShape for GoFeedRepo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM5.5 4A1.5 1.5 0 004 5.5v5c0 .828.5 1.5 1 1.5v-1a1 1 0 011-1h5v1h-1v1h1.5a.5.5 0 00.5-.5v-7a.5.5 0 00-.5-.5h-6zm.5 7.25a.25.25 0 01.25-.25H9v2.764a.25.25 0 01-.426.178l-.898-.888a.25.25 0 00-.352 0l-.898.888A.25.25 0 016 13.764V11.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedRocket;
impl IconShape for GoFeedRocket {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm3.031-12a4.38 4.38 0 00-3.097 1.283l-.23.229c-.156.157-.308.32-.452.49H5.65a.876.876 0 00-.746.417l-.856 1.388a.375.375 0 00.21.556l1.552.477 1.35 1.35.478 1.553a.375.375 0 00.555.21l1.389-.855a.876.876 0 00.416-.746V8.747c.17-.144.333-.295.49-.452l.23-.23A4.38 4.38 0 0012 4.969v-.093A.876.876 0 0011.124 4h-.093zm-5.107 7.144a.81.81 0 01-.188.263c-.394.394-1.258.563-1.62.619a.124.124 0 01-.143-.143c.056-.362.225-1.226.62-1.62a.808.808 0 011.33.881z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedStar;
impl IconShape for GoFeedStar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zm.252-12.932a.478.478 0 00-.682.195l-1.2 2.432-2.684.39a.478.478 0 00-.266.816l1.944 1.892-.46 2.674a.478.478 0 00.694.504L8 10.709l2.4 1.261a.478.478 0 00.694-.504l-.458-2.673L12.578 6.9a.479.479 0 00-.265-.815l-2.685-.39-1.2-2.432a.478.478 0 00-.176-.195z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedTag;
impl IconShape for GoFeedTag {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.22 6.5a.72.72 0 11-1.44 0 .72.72 0 011.44 0z",
            }
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM4 8.379V5a1 1 0 011-1h3.379a1.5 1.5 0 011.06.44l3.213 3.211a1.2 1.2 0 010 1.698l-3.303 3.303a1.2 1.2 0 01-1.698 0L4.44 9.439A1.5 1.5 0 014 8.38z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFeedTrophy;
impl IconShape for GoFeedTrophy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11 5h1v1.146a1 1 0 01-.629.928L11 7.223V5zM5 7.223l-.371-.149A1 1 0 014 6.146V5h1v2.223z",
            }
            path {
                d: "M8 16A8 8 0 108 0a8 8 0 000 16zM3 5a1 1 0 011-1h8a1 1 0 011 1v1.146a2 2 0 01-1.257 1.857l-.865.346a3.005 3.005 0 01-2.294 2.094C8.78 11.405 9.342 12 10.5 12a.5.5 0 010 1h-5a.5.5 0 010-1h.002c1.156 0 1.718-.596 1.914-1.557A3.005 3.005 0 015.122 8.35l-.865-.346A2 2 0 013 6.146V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEp0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzX2sX+Gnge4Df5j8G4kX33sBXAcf57/fdwPvw74d40bw38F38z/LdwPvw74P4lx0Hng4c53+e7wbeh387xL/ss4HP4n+u7wbeh38bxL/sr4CX5n+27wbeh389xL/MPH8vA/w1//HM8/c9wHvxgn038D786yD+Zeb5E/85zPMn4LuB9+IF+27gfXjRIf5l5vkT/znM8yeu+G7gvXjBvht4H140iH+Zef7Efw7z/Iln+27gvXjBvht4H/5liH+Zef7Efw7z/Inn9N3Ae/GCfTfwPrxwiH+Zef7Efw7z/Inn9d3Ae/GCfTfwPrxgiH+Zef7Efw7z/Inn77uB9+IF+27gfXj+EP8y8/yJ/xzm+RMv2HcD78UL9t3A+/C8EP8y8/yJ/xzm+RMv3HcD78UL9t3A+/CcEP8y8/yJ/xzm+RP/su8G3osX7HWA3+bZEP8y8/yJ/xzm+RMvmu8G3ovn73uA9+bZEP8y8/yJ/xzm+XsZ4K950Xw38F48r1uBh/BsiH+Zef7Ef46/Bl6K/zzi2RD/MvP8if8cnw18Fv95xLMh/mXm+RP/OY4DtwLH+M8hng3xLzPPn/jP897Ad/GfQzwb4l9mnj/xn+u9ga8GjvEfSzwb4l9mnj/xn+848NHAWwMvxX8M8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmedP/M9mnj/xbIh/mXn+xP9s5vkTz4b4l5nnT/zPZp4/8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmedP/M9mnj/xbIh/mXn+xP9s5vkTz4b4l5nnT/zPZp4/8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmedP/M9mnj/xbIh/mXn+xP9s5vkTz4b4l5nnT/zPZp4/8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmedP/M9mnj/xbIh/mXn+xP9s5vkTz4b4l5nnT/zPZp4/8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmedP/M9mnj/xbIh/mXn+xP9s5vkTz4b4l5nnT/zPZp4/8WyIf5l5/sT/bOb5E8+G+JeZ50/8z2aeP/FsiH+Zef7E/2zm+RPPhviXmefvZYC/5n+mlwb+iudPPBviX/bXwEvxf8PfAC/NsyH+ZZ8NfBb/N3wO8Nk8G+Jfdhy4FTjG/26XgAcDuzwb4kXz3sB38b/b2wA/zXNCvOjeG/hq4Bj/u1wC3hv4aZ4X4l/nOPDRwFsDL8X/bH8D/DTw1cAuzx//CKiSoUGXiui6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFile;
impl IconShape for GoFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 019 4.25V1.5H3.75zm6.75.062V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzX2sX+Gnge4Df5j8G4kX33sBXAcf57/fdwPvw74d40bw38F38z/LdwPvw74P4lx0Hng4c53+e7wbeh387xL/ss4HP4n+u7wbeh38bxL/sr4CX5n+27wbeh389xL/MPH8vA/w1/zWOA18NvBcv2HcD78O/DuJfZp4/8V/vu4H34gX7buB9eNEh/mXm+RP/Pb4beC9esO8G3ocXDeJfZp4/8d/nu4H34gX7buB9+Jch/mXm+RP/vb4beC9esO8G3ocXDvEvM8+f+O/33cB78YJ9N/A+vGCIf5l5/sT/DN8NvBcv2HcD78Pzh/iXmedP/M/x3cB78YJ9N/A+PC/Ev8w8f+J/lu8G3osX7LuB9+E5If5l5vkT//N8N/BevGCvA/w2z4b4l5nnT/zP9N3Ae/H8fQ/w3jwb4l9mnj/xP9d3A+/F87oVeAjPhviXmedP/M91HLjI8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/j3OQ58FvDWXPHdwOfwH8c8f+LZEP8y8/yJf5+vBj6K5/QxwFfzH8M8f+LZEP8y8/yJfx/zvP4aeBn+Y5jnTzwb4l9mnj/x72OeP/Efwzx/4tkQ/zLz/Il/H/P8if8Y5vkTz4b4l5nnT/z7mOdP/Mcwz594NsS/zDx/4t/HPH/iP4Z5/sSzIf5l5vkT/z7m+RP/MczzJ54N8S8zz5/49zHPn/iPYZ4/8WyIf5l5/sQL91nAWwMvzX+MW4HvBj6HF415/sSzIf5l5vkTL9hnA5/Ff47PAT6bf5l5/sSzIf5l5vkTL9hF4Dj/OW4FHsK/zDx/4tkQ/zLz/IkXbBc4xn+OZwAP5l9mnj/xbIh/mXn+xAv21cBH8Z/jc4DP5l9mnj/xbIh/mXn+xAt2HPhs4KP4j/MM4LuBz+ZFY54/8WyIf5l5/sS/j3n+xH8M8/yJZ0P8y8zzJ/59zPMn/mOY5088G+JfZp4/8e9jnj/xH8M8f+LZEP8y8/yJfx/z/In/GOb5E8+G+JeZ50/8+5jnT/zHMM+feDbEv8w8f+Lfxzx/4j+Gef7EsyH+Zeb5E/8+u8AxntMzgAfzH8M8f+LZEP8y8/yJf5+vBj6K5/Q1wEfzH8M8f+LZEP8y8/yJf5/jwGcD780V3w18NrDLfwzz/IlnQ/zLzPMn/mczz594NsS/zDx/4n828/yJZ0P8y8zzJ/5nM8+feDbEv8w8f+J/NvP8iWdD/MvM8yf+ZzPPn3g2xL/MPH/ifzbz/IlnQ/zLzPMn/mczz594NsS/zDx/4n828/yJZ0P8y8zzJ/5nM8+feDbEv8w8f+J/NvP8iWdD/MvM8yf+ZzPPn3g2xL/MPH/ifzbz/IlnQ/zLzPP3MsBf8z/TSwN/xfMnng3xL/tr4KX4v+FvgJfm2RD/ss8GPov/Gz4H+GyeDfEvOw7cChzjf7dLwIOBXZ4N8aJ5b+C7+N/tbYCf5jkhXnTvDXw1cIz/XS4B7w38NM8L8a9zHPho4K2Bl+J/tr8Bfhr4amCX549/BI1Ou0EDhsJzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileAdded;
impl IconShape for GoFileAdded {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75zm6.23 3.508a.75.75 0 01.755.745l.01 1.497h1.497a.75.75 0 010 1.5H9v1.507a.75.75 0 01-1.5 0V9.005l-1.502.01a.75.75 0 11-.01-1.5l1.507-.01-.01-1.492a.75.75 0 01.745-.755z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileBadge;
impl IconShape for GoFileBadge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h3.5a.75.75 0 010 1.5h-3.5A1.75 1.75 0 011 13.25V1.75C1 .784 1.784 0 2.75 0h8a1.75 1.75 0 011.508.862.75.75 0 11-1.289.768.25.25 0 00-.219-.13h-8z",
            }
            path {
                d: "M8 7a4 4 0 116.49 3.13l.995 4.973a.75.75 0 01-.991.852l-2.409-.876a.25.25 0 00-.17 0l-2.409.876a.75.75 0 01-.991-.852l.994-4.973A3.993 3.993 0 018 7zm4-2.5a2.5 2.5 0 100 5 2.5 2.5 0 000-5zm0 6.5a4 4 0 001.104-.154l.649 3.243-1.155-.42c-.386-.14-.81-.14-1.196 0l-1.155.42.649-3.243A4 4 0 0012 11z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviPdRz4KOCtgZfmP9cu8NPA9wC/zb8N4j/OewNfBRznv953A+/Dvx7iP8Z7A9/Ff6/vBt6Hfx3Ev99x4OnAcf77fTfwPrzoEP9+nw18Fv9zfDfwPrxoEP9+fwW8NP+zfDfwPvzLEP9+5vl7GeCv+fczz9/3AO/FC/bdwPvwwiH+/czzJ/5jmOdPwHcD78UL9t3A+/CCIf79zPMn/mOY509c8d3Ae/GCfTfwPjx/iH8/8/yJ/xjm+RPP9t3Ae/GCfTfwPjwvxL+fef7Efwzz/Inn9N3Ae/GCfTfwPjwnxL+fef7Efwzz/Inn9d3Ae/GCfTfwPjwb4t/PPH/iP4Z5/sTz993Ae/GCfTfwPlyB+Pczz5/4j2GeP/GCfTfwXrxg3w28D4D49zPPn/iPYZ4/8cJ9N/BevGDfDbyP+PfbBY7xnJ4BPJj/GOb5E/+y7wbeixfsdcS/32cDn8Vz+hzgs/mPYZ4/8aL5buC9eP6+R/zH+Grgrbniu4HP5j+Oef5eBvhrXjTfDbwXz+tW8T/fXwMvxX8OxP98nw18Fv85EP/zHQduBY7xHw/xv8N7A9/FfzzE/x7vDXw1cIz/OIj/XY4DHw28NfBS/PshntNx4KOAtwZemhfur4GfBr4G2OUFOw58FvDWwIN5/m4Fvhv4HP7zmOeFeLb3Br4KOM6/zi7wMcB38/x9NfBRvGi+Bvho/nOY54W44r2B7+Lf532A7+Z5XQSO86K5FXgI/znM80LAceDpwHH+fXaBhwC7PKdd4BgvOvGfwzwvBHw28Fn8x/gc4LN5Tp8NfBYvOvGfwzwvBPwV8NL8x/hr4GV4Xl8NvDXwIP5l4j+HeV4IMM/fywB/zfP30sBf8fyJF415/sR/DvO8EGCeP/HCmedPvGjM8yf+c5jnhQDz/IkXzjx/4kVjnj/xn8M8LwSY50+8cOb5Ey8a8/yJ/xzmeSHAPH/ihTPPn3jRmOdP/OcwzwsB5vkTL5x5/sSLxjx/4j+HeV4IMM+feOHM8ydeNOb5E/85zPNCgHn+xAtnnj/xojHPn/jPYZ4XAszzJ1448/yJF415/sR/DvO8EGCeP/HCmedPvGjM8yf+c5jnhQDz/IkXzjx/4kVjnj/xn8M8LwSY5+9lgL/m+Xtp4K94/sSLxjx/4j+HeV4I+GvgpfiP8TfAS/OcjgOfBbw18GD+dW4Fvhv4HP79zPNCwGcDn8V/jM8BPpvn9NXAR/Hv8zXAR/PvY54XAo4DtwLH+Pe5BDwY2OU5XQSO8+9zK/AQ/n3M80Jc8d7Ad/Hv8zbAT/O8doFj/Ps8A3gw/z7meSGe7b2BrwaO8a9zCXhv4Kd5/j4b+Cz+fT4H+Gz+fczzQjyn48BHA28NvBQv3N8APw18NbDLC/fVwFsDD+Jf5xnAdwOfzb+feV6I/z/M80L8/2GeF+L/D/O8EP9/mOeF+P/DPC/E/w8vDfwVzwvxH+c48FnAWwMP5n+HvxH/cb4a+Cj+d/kc8R/nInCc/z0uAQ8W/3F2gWP87/E2wE+L/zifDXwW//NdAt4b+GkA8R/rq4G3Bh7E/zx/A/w08NXALlcg/n9D/P+G+P8N8f8b/wixUdI8DR73cwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileBinary;
impl IconShape for GoFileBinary {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4 1.75C4 .784 4.784 0 5.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0114.25 15h-9a.75.75 0 010-1.5h9a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 0110 4.25V1.5H5.75a.25.25 0 00-.25.25v2a.75.75 0 01-1.5 0v-2zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM0 7.75C0 6.784.784 6 1.75 6h1.5C4.216 6 5 6.784 5 7.75v2.5A1.75 1.75 0 013.25 12h-1.5A1.75 1.75 0 010 10.25v-2.5zm1.75-.25a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h1.5a.25.25 0 00.25-.25v-2.5a.25.25 0 00-.25-.25h-1.5zm5-1.5a.75.75 0 000 1.5h.75v3h-.75a.75.75 0 000 1.5h3a.75.75 0 000-1.5H9V6.75A.75.75 0 008.25 6h-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviPdRz4KOCtgZfmP9cu8NPA9wC/zb8N4j/OewNfBRznv953A+/Dvx7iP8Z7A9/Ff6/vBt6Hfx3Ev99x4OnAcf77fTfwPrzoEP9+nw18Fv9zfDfwPrxoEP9+fwW8NP+zfDfwPvzLEP9+5vl7GeCv+fczz9/3AO/FC/bdwPvwwiH+/czzJ/5jmOdPwHcD78UL9t3A+/CCIf79zPMn/mOY509c8d3Ae/GCfTfwPjx/iH8/8/yJ/xjm+RPP9t3Ae/GCfTfwPjwvxL+fef7Efwzz/Inn9N3Ae/GCfTfwPjwnxL+fef7Efwzz/Inn9d3Ae/GCfTfwPjwb4t/PPH/iP4Z5/sTz993Ae/GCfTfwPlyB+Pczz5/4j2GeP/GCfTfwXrxg3w28D4D49zPPn/iPYZ4/8cJ9N/BevGDfDbyP+Pczz5/4j2GeP/Ev+27gvXjBXkf8+5nnT/zHMM+feNF8N/BePH/fI/79doFjPKdnAA/mP4Z5/l4G+GteNN8NvBfP61bx7/fZwGfxnD4H+Gz+Y/w18FL850D8x/hq4K254ruBz+Y/zmcDn8V/DsT/fMeBW4Fj/MdD/O/w3sB38R8P8b/HewNfDRzjPw7if5fjwEcDbw28FP9+iP8/zPNC/P9hnhfiRfNZwFtzxXcDX8OL5rOAj+aKrwY+h/8+5nkh/mXfBbw3z+ljgK/mhfts4LN4Tt8NvA//PczzQrxw3wW8N8/rr4GX4YW7CBzneX038D781zPPC/GCfRfw3jx/fwO8NC/cLnCM5++7gffhv5Z5Xojn77uA9+YF+xzgs3nhPhv4LF6w7wbeh/865nkhntd3Ae/NC/Y9wHvzovlu4L14wb4beB/+a5jnhXhO3wW8Ny/Y9wDvzb/OdwPvxQv23cD78J/PPC/Es30X8N68YN8DvDf/Nt8NvBcv2HcD78N/LvO8EFd8F/DevGDfA7w3/z7fDbwXL9h3A+/Dfx7zvBDw2cBn8YJ9D/De/Mf4buC9eME+B/hs/nOY54WAi8Bxnr/vAd6b/1jfDbwXz9+twEP4z2GeFwJ2gWM8f98NvA//sb4LeG+ev2cAD+Y/h3leCPhs4LN4wb4beB/+Y3wX8N68YJ8DfDb/OczzQlzx3cB78YJ9N/A+/Pt8F/DevGDfA7w3/3nM80I823cD78UL9t3A+/Bv813Ae/OCfQ/w3vznMs8L8Zy+G3gvXrDvBt6Hf53vAt6bF+x7gPfmP595Xojn9d3Ae/GCfTfwPrxovgt4b16w7wHem/8a5nkhnr/vBt6LF+xzgM/mhftq4KN4wb4HeG/+65jnhXjBvht4L56/vwZehhfOvGDfA7w3/7XM80K8cN8NvBfP62+Al+aFM8/f9wDvzX8987wQ/7LvBt6L5/QxwFfzwv008FY8p+8B3pv/HuZ5IV40nw28NVd8N/DV/MuOA98NvBVwCfhu4KP572OeF+L/D/O8EP9/mOeF+P/DPC/E/x/meSH+/zDPC/H/w0sDf8XzQvzHOQ58FvDWwIP53+FvxH+crwY+iv9dPkf8x7kIHOd/j0vAg8V/nF3gGP97vA3w0+I/zmcDn8X/fJeA9wZ+GkD8x/pq4K2BB/E/z98APw18NbDLFYj/3xD/vyH+f0P8/8Y/AuU9zzzgYAdoAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileCode;
impl IconShape for GoFileCode {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4 1.75C4 .784 4.784 0 5.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0114.25 15h-9a.75.75 0 010-1.5h9a.25.25 0 00.25-.25V6h-2.75A1.75 1.75 0 0110 4.25V1.5H5.75a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013l-2.914-2.914a.272.272 0 00-.013-.011zM5.72 6.72a.75.75 0 000 1.06l1.47 1.47-1.47 1.47a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM3.28 7.78a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 001.06-1.06L1.81 9.25l1.47-1.47z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjhjgMfBbw18NL899oFfhr4HuC3+Y+BeMHeG/gq4Dj/83w38D78+yGev/cGvov/2b4beB/+fRDP6zjwdOA4//N9N/A+/NshntdnA5/F/x7fDbwP/zaI5/VXwEvzv8t3A+/Dvx7ieZnn72WAv+a/x3Hgq4H34gX7buB9+NdBPC/z/In/ft8NvBcv2HcD78OLDvG8zPMn/mf4buC9eMG+G3gfXjSI52WeP/E/x3cD78UL9t3A+/AvQzwv8/yJ/1m+G3gvXrDvBt6HFw7xvMzzJ/7n+W7gvXjBvht4H14wxPMyz5/4n+m7gffiBftu4H14/hDPyzx/4j/OceCzgLfmiu8GPod/u+8G3osX7LuB9+F5IZ6Xef7Ef5yvBj6K5/Q1wEfzb/fdwHvxgn038D48J8TzMs+f+I9zETjOc7oVeAj/Pt8NvBcv2OsAv82zIZ6Xef7Efxzz/Il/v+8G3ovn73uA9+bZEM/LPH/iP455/sR/jO8G3ovndSvwEJ4N8bzM8yf+45jnT/zHOA5c5PkTz4Z4Xub5E/9xzPMn/uOY5088G+J5medP/Mcxz5/4j2OeP/FsiOdlnj/xH8c8f+I/jnn+xLMhnpd5/sSL5jjwWcBbAw/mP8atwHcDn8OLzjx/4tkQz8s8f+JF89XAR/Gf42uAj+ZFY54/8WyI52WeP/GiuQgc5z/HrcBDeNGY5088G+J5medPvGh2gWP853gG8GBeNOb5E8+GeF7m+RMvms8GPov/HJ8DfDYvGvP8iWdDPC/z/IkX3VcDbw08iP8YzwC+G/hsXnTm+RPPhnhe5vkT/3HM8yf+45jnTzwb4nmZ50/8xzHPn/iPY54/8WyI52WeP/Efxzx/4j+Oef7EsyGel3n+xH8c8/yJ/zjm+RPPhnhe5vkT/3HM8yf+45jnTzwb4nmZ50/8xzHPn/iPY54/8WyI52WeP/EfZxc4xnN6BvBg/uOY5088G+J5medP/Mf5bOCzeE6fA3w2/3HM8yeeDfG8zPMn/mN9NfDWXPHdwGfzH8s8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8iRfNceCzgLcGHsx/jFuB7wY+hxedef7EsyGel3n+xIvmq4GP4j/H1wAfzYvGPH/i2RDPyzx/4kVzETjOf45bgYfwojHPn3g2xPMyz5940ewCx/jP8QzgwbxozPMnng3xvMzzJ140nw18Fv85Pgf4bF405vkTz4Z4Xub5Ey+6rwbeGngQ/zGeAXw38Nm86MzzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLP38sAf83/Di8N/BXPn3g2xPP6a+Cl+L/pb4CX5tkQz+uzgc/i/6bPAT6bZ0M8r+PArcAx/m+5BDwY2OXZEM/fewPfxf8tbwP8NM8J8YK9N/DVwDH+d7sEvDfw0zwvxAt3HPho4K2Bl+J/l78Bfhr4amCX549/BKjtyEH8/sMNAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDiff;
impl IconShape for GoFileDiff {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H2.75zM1 1.75C1 .784 1.784 0 2.75 0h7.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16H2.75A1.75 1.75 0 011 14.25V1.75zm7 1.5a.75.75 0 01.75.75v1.5h1.5a.75.75 0 010 1.5h-1.5v1.5a.75.75 0 01-1.5 0V7h-1.5a.75.75 0 010-1.5h1.5V4A.75.75 0 018 3.25zm-3 8a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5h-4.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm3+63gd8GvgbY5X8uxLO9N/BVwHH+4+wC7wP8NP8zIa54b+C7+M/zPsB38z8PAo4DTweO859nF3gd4K/5nwUBnw18Fv/5/hp4HWCX/zkQ8FfAS/Nf47uB9+F/DgSY5+9lgL/mX++lgd8GjvH8vQ/w3fzPgADz/Il/u/cGvovnbxd4HeCv+e+HAPP8iX+f7wbei/85/hr4aeBrgF2uQIB5/sS/z3Hgt4GX4n+WXeBjgO8GEGCeP/Hv99LAbwPH+J/nfYDvFmCeP/Ef472B7+J/nl3gIQLM8yf+47w18N3AMf5n+RwB5vkT/7GOAx8NvDbwWvzP8NcCzPMn/u94aeCveF4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8/cywF/zf8NLA3/F80LAXwMvxf9PfyPgs4HP4v+nzxFwHLgVOMb/L5eAB4sr3hv4Lv5/eRvgp8WzvTfw1cAx/m+7BLw38NMA4jkdBz4aeGvgpfi/5W+Anwa+GtjlCsT/b4j/3xD/vyH+f+MfAcf/hNzKI/X+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectory;
impl IconShape for GoFileDirectory {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H7.5c-.55 0-1.07-.26-1.4-.7l-.9-1.2a.25.25 0 00-.2-.1H1.75zM0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm3+63gd8GvgbY5X8uxLO9N/BVwHH+4+wC7wP8NP8zIa54b+C7+M/zPsB38z8PAo4DTweO859nF3gd4K/5nwUBnw18Fv/5/hp4HWCX/zkQ8FfAS/Nf47uB9+F/DgSY/1rvA3w3/zMgwPzX2gVeB/hr/vshwPz/8NfATwNfA+xyBQLM/y+7wMcA3w0gwPz/9D7Adwsw/z/tAg8RYP7/+hwB5v+vvxZg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD5/wsB5v8vBJj/vxBg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD5/wsB5v8vBJj/vxBg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD5/wsB5v8vBJj/vxDw18BL8f/T3wj4bOCz+P/pcwQcB24FjvH/yyXgweKK9wa+i/9f3gb4afFs7w18NXCM/9suAe8N/DSAeE7HgY8G3hp4Kf5v+Rvgp4GvBna5AvH/G+L/N8T/b4j/3/hHAGNW3HlnmgMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectoryFill;
impl IconShape for GoFileDirectoryFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H7.5a.25.25 0 01-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADBklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm3+e3gd8GvgbY5X8mxLO9N/BVwHH+Y+0C7wP8NP/zIK54b+C7+M/1PsB38z8LAo4DTweO859rF3gd4K/5nwMBnw18Fv81/hp4HWCX/xkQ8FfAS/Nf57uB9+F/BgSY/3rvA3w3//0QYP7r7QKvA/w1/70QYP5vuRX4a+BjgFt54RBg/m/aBV4GuJUXDAHm/66fAd6aFwwB5v+uW4GH8IIhwPzfJl4wBJj/28QLhgDzwl0CPhv4aeBW/m9BgHnhPgb4av5vQoB54U4Au/zfhADzwp0Advm/CQHmhfsc4LP5vwkB5l/20cDPALfyfwsCzP8v3w18DnArgADz/88u8BBgV4D5/+l7gPcWYP5/uhV4iADz/9MzgAcLMP8//Q7w2gLM/08/A7y1APP/0+cAny3A/P/0McBXCzD/P70O8NsCzP9PrwP8tgDz/5MABJj/nwQgwPz/8zvAawMIMP///A7w2gACzP8/XwN8NIAA8//P5wCfDSDA/P/zNsBPAwgw//+8DvDbAALM/z8PAW4FEGD+/xFXIMD8//IM4MFcgQDz/8vvAK/NFQgw/7/8DPDWXIEA8//L5wCfzRUIMP+/fAzw1VyBAPP/y+sAv80VCDD/v7wO8NtcgQDz/4t4NgSY/1/EsyHgr4GX4v+HvwFemmdDwGcDn8X/D58DfDbPhoDjwK3AMf5vuwQ8GNjl2RBXvDfwXfzf9jbAT/OcEM/23sBXA8f4v+US8N7AT/O8EM/pOPDRwFsDL8X/bn8D/DTw1cAuzx/i/zfE/2+I/98Q/7/xjy/oe0F11POJAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileDirectoryOpenFill;
impl IconShape for GoFileDirectoryOpenFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M.513 1.513A1.75 1.75 0 011.75 1h3.5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1H13a1 1 0 011 1v.5H2.75a.75.75 0 000 1.5h11.978a1 1 0 01.994 1.117L15 13.25A1.75 1.75 0 0113.25 15H1.75A1.75 1.75 0 010 13.25V2.75c0-.464.184-.91.513-1.237z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzX2sX+Gnge4Df5j8G4kX33sBXAcf57/fdwPvw74d40bw38F38z/LdwPvw74P4lx0Hng4c53+e7wbeh387xL/ss4HP4n+u7wbeh38bxL/sr4CX5n+27wbeh389xL/MPH8vA/w1/zWOA18NvBcv2HcD78O/DuJfZp4/8V/vu4H34gX7buB9eNEh/mXm+RP/Pb4beC9esO8G3ocXDeJfZp4/8d/nu4H34gX7buB9+Jch/mXm+RP/vb4beC9esO8G3ocXDvEvM8+f+O/33cB78YJ9N/A+vGCIf5l5/sT/DN8NvBcv2HcD78Pzh/iXmedP/M/x3cB78YJ9N/A+PC/Ev8w8f+J/lu8G3osX7LuB9+E5If5l5vkT//N8N/BevGCvA/w2z4b4l5nnT/zP9N3Ae/H8fQ/w3jwb4l9mnj/xP9d3A+/F87oVeAjPhviXmedP/M91HLjI8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xL9sFjvGcngE8mP/ZzPMnng3xL/ts4LN4Tp8DfDb/s5nnTzwb4kXz1cBbc8V3A5/NC/bWwDHge/jvZZ4/8WyI/3jmiu8G3of/Pub5E8+G+I9nnu27gffhv4d5/sSzIf7jmef03cD78F/PPH/i2RD/8czz+m7gffivZZ4/8WyI/3jm+ftu4H34r2OeP/FsiP945gX7buB9+K9hnj/xbIj/eOaF+27gffjPZ54/8WyI/3jmX/bdwPvwn8s8f+LZEP/xzIvmu4H34T+Pef7EsyH+45kX3XcD78N/DvP8iWdDwHHgs4C3Bh7Mf73vBt6H/3jm+RPPhoCvBj6K/17fDbwP/7HM8yeeDQEXgeP893sf4Lv5j2OeP/FsCNgFjvHf722An+Y/jnn+xLMh4LOBz+K/1/sA381/LPP8iWdDXPHVwFsDD+K/3vsA381/PPP8iWdD/MczL7r3Ab6b/xzm+RPPhviPZ1407wN8N/95zPMnng3xH8/8y94H+G7+c5nnTzwb4j+eeeHeB/hu/vOZ5088G+I/nnnB3gf4bv5rmOdPPBviP555/t4H+G7+a7w08Fc8f+LZEP/xzPN6H+C7edEcBz4LeGvgwfzH+hvgpXk2xH8885zeB/huXnRfDXwU/zk+B/hsng3xH8882/sA382/zkXgOP/xLgEPBnZ5NsR/PHPF+wDfzb/eLnCM/3hvA/w0zwnxH++tueKn+bf5bOCz+I9zCXhv4Kd5Xoj/mb4aeGvgQfzb/Q3w08BXA7s8f/wj5Pq2QVd7IgMAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileMoved;
impl IconShape for GoFileMoved {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-3.5a.75.75 0 010-1.5h3.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75a.25.25 0 00-.25.25v6.5a.75.75 0 01-1.5 0v-6.5z",
            }
            path {
                d: "M5.427 15.573l3.146-3.146a.25.25 0 000-.354L5.427 8.927A.25.25 0 005 9.104V11.5H.75a.75.75 0 000 1.5H5v2.396c0 .223.27.335.427.177z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzX2sX+Gnge4Df5j8G4kX33sBXAcf57/fdwPvw74d40bw38F38z/LdwPvw74P4lx0Hng4c53+e7wbeh387xL/ss4HP4n+u7wbeh38bxL/sr4CX5n+27wbeh389xL/MPH8vA/w1/zWOA18NvBcv2HcD78O/DuJfZp4/8V/vu4H34gX7buB9eNEh/mXm+RP/Pb4beC9esO8G3ocXDeJfZp4/8d/nu4H34gX7buB9+Jch/mXm+RP/vb4beC9esO8G3ocXDvEvM8+f+O/33cB78YJ9N/A+vGCIf5l5/sT/DN8NvBcv2HcD78Pzh/iXmedP/M/x3cB78YJ9N/A+PC/Ev8w8f+J/lu8G3osX7LuB9+E5If5l5vkT//N8N/BevGCvA/w2z4b4l5nnT/zP9N3Ae/H8fQ/w3jwb4l9mnj/xP9d3A+/F87oVeAjPhviXmedP/M91HLjI8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJF+6zgLcGXpr/GLcC3w18Di8a8/yJZ0P8y8zzJ16wzwY+i/8cnwN8Nv8y8/yJZ0P8y8zzJ16wi8Bx/nPcCjyEf5l5/sSzIf5l5vkTL9gucIz/HM8AHsy/zDx/4tkQ/zLz/IkX7KuBj+I/x+cAn82/zDx/4tkQ/zLz/IkX7Djw2cBH8R/nGcB3A5/Ni8Y8f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPPn/ifzTx/4tkQ/zLz/In/2czzJ54N8S8zz5/4n808f+LZEP8y8/yJ/9nM8yeeDfEvM8+f+J/NPH/i2RD/MvP8if/ZzPMnng3xLzPP38sAf83/TC8N/BXPn3g2xL/sr4GX4v+GvwFemmdD/Ms+G/gs/m/4HOCzeTbEv+w4cCtwjP/dLgEPBnZ5NsSL5r2B7+J/t7cBfprnhHjRvTfw1cAx/ne5BLw38NM8L8S/znHgo4G3Bl6K/9n+Bvhp4KuBXZ4//hEW5KJBdUfLsAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileRemoved;
impl IconShape for GoFileRemoved {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.75 1.5a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25h9.5a.25.25 0 00.25-.25V4.664a.25.25 0 00-.073-.177l-2.914-2.914a.25.25 0 00-.177-.073H3.75zM2 1.75C2 .784 2.784 0 3.75 0h6.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v9.586A1.75 1.75 0 0113.25 16h-9.5A1.75 1.75 0 012 14.25V1.75zM8.25 7.5h2.242a.75.75 0 010 1.5h-2.24l-2.254.015a.75.75 0 01-.01-1.5L8.25 7.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADS0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm3+63gd8GvgbY5X8uxLO9N/BVwHH+4+wC7wP8NP8zIa54b+C7+M/zPsB38z8PAo4DTweO859nF3gd4K/5nwUBnw18Fv/5/hp4HWCX/zkQ8FfAS/Nf47uB9+F/DgSY/1rvA3w3/zMgwPzX2gVeB/hr/vshwPz/8NfATwNfA+xyBQLM/y+7wMcA3w0gwPz/9D7Adwsw/z/tAg8RYP7/+hwB5v+vvxZg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY/1k+hiu+iv98CDD/s4gr3hv4Lv5zIcD8zyKe7bWBnwaO8Z8DAeZ/FvGcXhr4beAY//EQYP5nEc/rpYHfBo7xHwsB5n8W8fy9NPDdwEvxHwcB5n8W8YIdB34beCn+YyDA/M8iXrjjwG8DL8W/HwLM/yziX3Yc+Gngtfj3QYD597kEfDbw08Ct/Nf6buC9+LdDgPn3+Rjgq/nv893Ae/FvgwDz73MC2OW/108Db8W/HgLMv88JYJf/Xj8FvDX/eggw/z6fA3w2/32+C3hv/m0QYP79Phr4GeBW/mt9F/De/NshwPzPIv5lx4GfAl6bfx8EmP9ZxAt3HPgt4KX590OA+Z9FvGDHgd8CXpr/GAgw/7OI5++lge8CXpr/OAgw/7OI5/XSwG8Bx/mPhQDzP4t4Ti8N/BZwnP94CDD/s4hne23gp4Dj/OdAgPmfRVzx3sB38Z8LAeZ/lo/miq/mPx8CzP9fCDD/fyHA/P+FAPP/FwLM/18I+Gvgpfj/6W8EfDbwWfz/9DkCjgO3Asf4/+US8GBxxXsD38X/L28D/LR4tvcGvho4xv9tl4D3Bn4aQDyn48BHA28NvBT/t/wN8NPAVwO7XIH4/w3x/xvi/zfE/2/8I4WNd9xs+KibAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileSubmodule;
impl IconShape for GoFileSubmodule {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 2.75C0 1.784.784 1 1.75 1H5c.55 0 1.07.26 1.4.7l.9 1.2a.25.25 0 00.2.1h6.75c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm9.42 9.36l2.883-2.677a.25.25 0 000-.366L9.42 6.39a.25.25 0 00-.42.183V8.5H4.75a.75.75 0 100 1.5H9v1.927c0 .218.26.331.42.183z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFgUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzn2sX+Gnge4Df5j8H4kX33sBXAcf5r/fdwPvwHw/xonlv4Lv47/XdwPvwHwvxLzsOPB04zn+/7wbeh/84iH/ZZwOfxf8c3w28D/8xEP+yvwJemv9Zvht4H/79EP8y8/y9DPDX/PuZ5+97gPfiBftu4H3490H8y8zzJ/5jmOdPwHcD78UL9t3A+/Bvh/iXmedP/Mcwz5+44ruB9+IF+27gffi3QfzLzPMn/mOY508823cD78UL9t3A+/Cvh/iXmedP/Mcwz594Tt8NvBcv2HcD78O/DuJfZp4/8R/DPH/ieX038F68YN8NvA8vOsS/zDx/4j+Gef7E8/fdwHvxgn038D68aBD/MvP8if8Y5vkTL9h3A+/FC/bdwPvwL0P8y8zzJ/5jmOdPvHDfDbwXL9h3A+/DC4f4l5nnT/zHMM+f+Jd9N/BevGCvA/w2LxjiX2aeP/Efwzx/4kXz3cB78fx9D/DevGCIf5l5/sR/DPP8vQzw17xovht4L57XrcBDeMEQ/7Jd4BjP6RnAg/mP8dfAS/GfR7xgiH/ZZwOfxXP6HOCz+Y/x2cBn8Z9HvGCIF81XA2/NFd8NfDb/cY4DtwLH+M8hXjDE/wzvDXwX/znEC4b4n+O9ga8GjvEfS7xgiP9ZjgMfDbw18FL8xxAvGOL/DvP8iRcM8e/3VVzxMfz3Ms+feMEQ/37miu8G3of/Pub5Ey8Y4t/PPNtvA28D7PJfzzx/4gVD/PuZ5/TXwOsAu/zXMs+feMEQ/37mef018DrALv91zPMnXjDEv595/v4aeB/gr/mvYZ4/8YIh/v3MC7YLvA7w1/znM8+feMEQ/37mhdsFXgf4a/5zmedPvGCIfz/zL9sF3gb4bf7zmOdPvGCIF91bAW8NPBh4aeA4/3rvA3w3/znM8ydeMMQLdxz4KOCjgeP8x3gf4Lv5j2eeP/GCIV6w9wa+CjjOf7y3AX6a/1jm+RMvGOL5+yrgo/nP8zbAT/Mfyzx/4gVDPK+fAt6a/zzvA3w3//HM8ydeMMRz+mrgo/jP8z7Ad/Ofwzx/4gVDPNtbAz/FC/c9wG8DPw3scoX5l10C3hr4bf7zmOdPvGCIK44DTweO8/z9DfDewF/zvMwLdwl4beCv+c9lnj/xgiGu+Gzgs3j+fgd4a2CX58+8YJeA1wb+mv985vkTLxjiiovAcZ7X3wCvDezygpnn72+A9wb+mv8a5vkTLxgCXhv4LZ6/1wF+mxfOPK+/AV4b2OW/jnn+xAuGgK8GPorn9TvAa/MvM8/pb4DXBnb5r2WeP/GCIeC3gdfieb0P8N38y8yz/Q7w1sAu//XM8ydeMAT8FfDSPK+HALfyLzNXfA/w3vz3Mc+feMEQYJ4/8aL5aq74aP57medPvGAIMM+f+N/FPH/iBUOAef7E/y7m+RMvGALM8yf+dzHPn3jBEGCeP/G/i3n+xAuGgF3gGM/pGcCD+d/jpYG/4vkTLxgCPhv4LJ7T5wCfzb/OceCzgLcGHsz/DH8DvDQvGOKKrwbemiu+G/hs/vW+Gvgo/mf5HOCzecEQ/3EuAsf5n+MS8GBglxcM8R9nFzjG/xxvA/w0LxziP85nA5/Ff79LwHsDP82/DPEf66uBtwYexH+9vwF+GvhqYJcXDeL/N8T/b4j/3xD/v/GPiIbWQeldO2IAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileSymlinkFile;
impl IconShape for GoFileSymlinkFile {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 1.75C2 .784 2.784 0 3.75 0h5.586c.464 0 .909.184 1.237.513l2.914 2.914c.329.328.513.773.513 1.237v8.586A1.75 1.75 0 0112.25 15h-7a.75.75 0 010-1.5h7a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75a.25.25 0 00-.25.25V4.5a.75.75 0 01-1.5 0V1.75zm7.5-.188V4.25c0 .138.112.25.25.25h2.688a.252.252 0 00-.011-.013L9.513 1.573a.248.248 0 00-.013-.011zm-8 10.675a2.25 2.25 0 012.262-2.25L4 9.99v1.938c0 .218.26.331.42.183l2.883-2.677a.25.25 0 000-.366L4.42 6.39a.25.25 0 00-.42.183V8.49l-.23-.001A3.75 3.75 0 000 12.238v1.012a.75.75 0 001.5 0v-1.013z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjXOQ58FPDWwEvzn2sX+Gnge4Df5j8H4kX33sBXAcf5r/fVwMfwHw/xonlv4Lv47/XdwPvwHwvxLzsOPB04zn+/7wbeh/84iH/ZZwOfxf8c3w28D/8xEP+yvwJemv9Zvht4H/79EP8y8/y9DPDX/MuOA58FvDXwYK64Ffhu4HN4/o4DXw28Fy/YdwPvw78P4l9mnj/xovlq4KN4/r4G+GhesO8G3osX7LuB9+HfDvEvM8+feNFcBI7z/N0KPIQX7ruB9+IF+27gffi3QfzLzPMnXjS7wDGev2cAD+Zf9t3Ae/GCfTfwPvzrIf5l5vkTL5rPBj6L5+9zgM/mRfPdwHvxgn038D786yD+Zeb5Ey+6rwbeGngQVzwD+G7gs/nX+W7gvXjBvht4H150iH+Zef7Ef4zjwGcBbw08mCtuBb4b+Bye13cD78UL9t3A+/CiQfzLzPMn/mN8NfBRPH9fA3w0z+u7gffiBftu4H34lyH+Zeb5E/8xLgLHef5uBR7C8/fdwHvxgn038D68cIh/mXn+xH+MXeAYz98zgAfzgn038F68YK8D/DYvGOJfZp4/8R/js4HP4vn7HOCzeeG+G3gvnr/vAd6bFwzxLzPPn/iP89XAWwMP4opnAN8NfDYvmu8G3ovndSvwEF4wxL/MPH/iRXMc+CzgrYEHc8WtwHcDn8N/jOPARZ4/8YIh/mXm+RMvmq8GPorn72uAj+Y/hnn+xAuG+JeZ50+8aC4Cx3n+bgUewn8M8/yJFwzxLzPPn3jR7ALHeP6eATyY/xjm+RMvGOJfZp4/8aL5bOCzeP4+B/hs/mOY50+8YIh/mXn+xIvuq4G3Bh7EFc8Avhv4bP7jmOdPvGCIf5l5/sR/jOPAZwFvDTyYK24Fvhv4HF505vkTLxjiX2aeP/Ef46uBj+L5+xrgo3nRmOdPvGCIf5l5/sR/jIvAcZ6/W4GH8KIxz594wRD/MvP8if8Yu8Axnr9nAA/mRWOeP/GCIf5l5vkT/zE+G/gsnr/PAT6bF415/sQLhviXmedP/Mf5auCtgQdxxTOA7wY+mxedef7EC4b4l5nnT7xojgOfBbw18GCuuBX4buBz+I9jnj/xgiH+Zeb5Ey+arwY+iufva4CP5j+Gef7EC4b4l5nnT7xoLgLHef5uBR7Cfwzz/IkXDPEvM8+feNHsAsd4/p4BPJj/GOb5Ey8Y4l9mnj/xovls4LN4/j4H+Gz+Y5jnT7xgiH+Zef7Ei+6rgbcGHsQVzwC+G/hs/uOY50+8YIh/mXn+xH+M48BnAW8NPJgrbgW+G/gcXnTm+RMvGOJfZp4/8R/jq4GP4vn7GuCjedGY50+8YIh/mXn+xH+Mi8Bxnr9bgYfwojHPn3jBEP8y8/yJ/xi7wDGev2cAD+ZFY54/8YIh/mXm+RP/MT4b+Cyev88BPpsXjXn+xAuG+JeZ50/8x/lq4K2BB3HFM4DvBj6bF515/sQLhviXmedPvOiOAx8FvDXw0jynvwZ+GvgaYJd/O/P8iRcM8S8zz5940bw38FXAcV64XeBjgO/m38Y8f+IFQ/zLzPMn/mXvDXwX/zrvA3w3/3rm+RMvGOJfZp4/8cIdB54OHOdfZxd4CLDLv455/sQLhviXmedPvHCfDXwW/zafA3w2/zrm+RMvGOJfZp4/8cL9FfDS/Nv8NfAy/OuY50+8YIh/mXn+xAtnnr+XAf6aK14a+CueP/GvY54/8YIh/mXm+RMvnHn+xHMyz5/41zHPn3jBEP8y8/yJF848f+I5medP/OuY50+8YIh/mXn+xAtnnj/xnMzzJ/51zPMnXjDEv8w8f+KFM8+feE7m+RP/Oub5Ey8Y4l9mnr+XAf6aF8w8f+I5medPvOheGvgrnj/xgiH+Zb8NvBbPaxd4HeCvef7M8yeek3n+xPN3HPgs4K2BB/PC/Q3w0rxgiH/ZZwOfxfP328Dr8PyZ5088J/P8iefvq4GP4kXzOcBn84Ih/mXHgVuBYzx/4vkzz594Tub5E8/fReA4/7JLwIOBXV4wxIvmrYGf4nn9DPDWPH/m+RPPyTx/4vnbBY7xL3sb4Kd54RAvurcGvhs4xhWXgNcG/prnz/z7iOfvs4HP4gW7BLw38NP8yxD/OseB9+aK7wZ2ecHMv494wb4aeGvgQTzb3wA/DXw1sMuLBvGfx/z7iP98iP88u8Ax/m2eATyY/3yI/zyfDXwW/zafA3w2//kQ/7k+G3hv4EG8aJ4BfDfw2fzX4B8Bi8kcUO18dHcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFileZip;
impl IconShape for GoFileZip {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.5 1.75a.25.25 0 01.25-.25h3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h2.086a.25.25 0 01.177.073l2.914 2.914a.25.25 0 01.073.177v8.586a.25.25 0 01-.25.25h-.5a.75.75 0 000 1.5h.5A1.75 1.75 0 0014 13.25V4.664c0-.464-.184-.909-.513-1.237L10.573.513A1.75 1.75 0 009.336 0H3.75A1.75 1.75 0 002 1.75v11.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217V1.75zM8.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM6 5.25a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5A.75.75 0 016 5.25zm2 1.5A.75.75 0 018.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 6.75zm-1.25.75a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM8 9.75A.75.75 0 018.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 9.75zm-.75.75a1.75 1.75 0 00-1.75 1.75v3c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75v-3a1.75 1.75 0 00-1.75-1.75h-.5zM7 12.25a.25.25 0 01.25-.25h.5a.25.25 0 01.25.25v2.25H7v-2.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACOElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/28IOA58FvDWwIP5/+FW4LuBzxHw1cBH8f/T1wi4CBzn/6dbBewCx/j/6RkCPhv4LP5/+hxxxVcDbw08iP8fngF8N/DZ4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4h/vePAZwFvDTyY/xluBb4b+Bz+dRD/el8NfBT/M30N8NG86BD/eheB4/zPdCvwEF50iH+9XeAY/zM9A3gwLzrEv95nA5/F/0yfA3w2LzrEv81XA28NPIj/GZ4BfDfw2fzrIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3cc+CzgrYEH86K5Ffhu4HP4z4X4z/fVwEfxb/M1wEfznwfxn+8icJx/m1uBh/CfB/Gfbxc4xr/NM4AH858H8Z/vs4HP4t/mc4DP5j8P4r/GVwNvDTyIF80zgO8GPpv/XIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wgVbikQOB1FTgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFilter;
impl IconShape for GoFilter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M.75 3a.75.75 0 000 1.5h14.5a.75.75 0 000-1.5H.75zM3 7.75A.75.75 0 013.75 7h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 013 7.75zm3 4a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFlame;
impl IconShape for GoFlame {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.998 14.5c2.832 0 5-1.98 5-4.5 0-1.463-.68-2.19-1.879-3.383l-.036-.037c-1.013-1.008-2.3-2.29-2.834-4.434-.322.256-.63.579-.864.953-.432.696-.621 1.58-.046 2.73.473.947.67 2.284-.278 3.232-.61.61-1.545.84-2.403.633a2.788 2.788 0 01-1.436-.874A3.21 3.21 0 003 10c0 2.53 2.164 4.5 4.998 4.5zM9.533.753C9.496.34 9.16.009 8.77.146 7.035.75 4.34 3.187 5.997 6.5c.344.689.285 1.218.003 1.5-.419.419-1.54.487-2.04-.832-.173-.454-.659-.762-1.035-.454C2.036 7.44 1.5 8.702 1.5 10c0 3.512 2.998 6 6.498 6s6.5-2.5 6.5-6c0-2.137-1.128-3.26-2.312-4.438-1.19-1.184-2.436-2.425-2.653-4.81z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjvcRz4LOCtueK7gc/hvx7iv8dXAx/Fc/oa4KP5r4X473EROM5zuhV4CP+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfjvYZ4/8V8L8d/DPH/ivxbiP9ZbAz/Ff663AX6a/xiI/1hvDfwU/7neBvhp/mMg/uO9N/Bd/Od4H+C7+Y+D+M/x3sB38R/rfYDv5j8W4j/PewPfxX+M9wG+m/94iP9c7w18F/8+7wN8N/85EP/53hv4Lv5t3gf4bv7zIP5rvDfwXfzrvA/w3fznQvzXeW/gu3jRvA/w3fznQ/zXem/gu3jh3gf4bv5rIP7rvTfwXTx/7wN8N/91EP893hv4Lp7T+wDfzX8txH+f9wa+iyveB/hu/ush/nu9NVf8NP89EP+/If5/Q/z/hvj/DfH/G+L/N8T/b4jndRz4LOCtgQdzxa3AdwOfw/M6DnwW8NbAg7niVuC7gc/heR0HPgt4a+DBXHEr8N3A5/C8jgOfBbw18GCuuBX4buBzeF7Hgc8C3hp4MFfcCnw38Dk8J8Tz+mrgo3j+vgb4aJ7TVwMfxfP3NcBH85y+Gvgonr+vAT6a5/TVwEfx/H0N8NE8p68GPorn72uAj+bZEM/rInCc5+9W4CE8p4vAcZ6/W4GH8JwuAsd5/m4FHsJzuggc5/m7FXgIz+kicJzn71bgITwb4nntAsd4/p4BPJjntAsc4/l7BvBgntMucIzn7xnAg3lOu8Axnr9nAA/mOe0Cx3j+ngE8mGdDPK/PBj6L5+9zgM/mOX028Fk8f58DfDbP6bOBz+L5+xzgs3lOnw18Fs/f5wCfzXP6bOCzeP4+B/hsng3x/H018NbAg7jiGcB3A5/N8/fVwFsDD+KKZwDfDXw2z99XA28NPIgrngF8N/DZPH9fDbw18CCueAbw3cBn8/x9NfDWwIO44hnAdwOfzXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If57vRdXfA//PRD/fb4LeG+u+G7gffivh/jv8V3Ae/Ocvht4H/5rIf7rfRfw3jx/3w28D/91EP+1vgt4b1647wbeh/8aiP863wW8Ny+a7wbeh/98iP8a3wW8N/863w28D/+5EP/5vgt4b/5tvht4H/7zIP5zfRfw3vz7fDfwPvznQPzn+S7gvfmP8d3A+/AfD/Gf47uA9+Y/1ncD78N/LMR/vO8C3pv/HN8NvA//cRD/sd4b+C7+c70P8N38x0D8x3pr4Kf4z/U2wE/zHwPx38M8f+K/FuK/h3n+xH8txH8P8/yJ/1qI/x7m+RP/tRD/PczzJ/5rIf577ALHeE7PAB7Mfy3Ef4/PBj6L5/Q5wGfzXwvx3+ergbfmiu8GPpv/evwjHTaSQcG3jq0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFold;
impl IconShape for GoFold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.896 2H8.75V.75a.75.75 0 00-1.5 0V2H5.104a.25.25 0 00-.177.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 2zM8.75 15.25a.75.75 0 01-1.5 0V14H5.104a.25.25 0 01-.177-.427l2.896-2.896a.25.25 0 01.354 0l2.896 2.896a.25.25 0 01-.177.427H8.75v1.25zm-6.5-6.5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 016 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 0112 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78hntdx4LOAtwYezBW3At8NfA7P6zjwWcBbAw/miluB7wY+h+d1HPgs4K2BB3PFrcB3A5/D8zoOfBbw1sCDueJW4LuBz+F5HQc+C3hr4MFccSvw3cDn8JwQz+urgY/i+fsa4KN5Tl8NfBTP39cAH81z+mrgo3j+vgb4aJ7TVwMfxfP3NcBH85y+Gvgonr+vAT6aZ0M8r4vAcZ6/W4GH8JwuAsd5/m4FHsJzuggc5/m7FXgIz+kicJzn71bgITyni8Bxnr9bgYfwbIjntQsc4/l7BvBgntMucIzn7xnAg3lOu8Axnr9nAA/mOe0Cx3j+ngE8mOe0Cxzj+XsG8GCeDfG8Phv4LJ6/zwE+m+f02cBn8fx9DvDZPKfPBj6L5+9zgM/mOX028Fk8f58DfDbP6bOBz+L5+xzgs3k2xPP31cBbAw/iimcA3w18Ns/fVwNvDTyIK54BfDfw2Tx/Xw28NfAgrngG8N3AZ/P8fTXw1sCDuOIZwHcDn83z99XAWwMP4opnAN8NfDbPCfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+ezgLfmip8GPof/eoj/Hp8NfBbP6XOAz+a/FuK/x9OBB/OcbgUewn8txH8P8/yJ/1qI/x7m+RP/tRD/PczzJ/5rIf57mOdP/NdC/Pcwz5/4r4X472GeP/FfC/Hfwzx/4r8W4r+Hef7Efy3Efw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfiP9dbAT/Gf622An+Y/BuI/1lsDP8V/rrcBfpr/GIj/eO8NfBf/Od4H+G7+4yD+c7w38F38x3of4Lv5j4X4z/PewHfxH+N9gO/mPx7iP9d7A9/Fv8/7AN/Nfw7Ef773Br6Lf5v3Ab6b/zyI/xrvDXwX/zrvA3w3/7kQ/3XeG/guXjTvA3w3//kQ/7XeG/guXrj3Ab6b/xqI/3rvDXwXz9/7AN/Nfx3Ef4/3Br6L5/Q+wHfzXwvx3+e9ge/iivcBvpv/eoj/Xm/NFT/Nfw/E/2+I/98Q/78h/n9D/P+G+P+NfwSsqXRBMMHipwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFoldDown;
impl IconShape for GoFoldDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.177 14.323l2.896-2.896a.25.25 0 00-.177-.427H8.75V7.764a.75.75 0 10-1.5 0V11H5.104a.25.25 0 00-.177.427l2.896 2.896a.25.25 0 00.354 0zM2.25 5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 4.25a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5a.75.75 0 01.75.75zM8.25 5a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 4.25a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5a.75.75 0 01.75.75zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+e70XV3wP/z0Q/32+C3hvrvhu4H34r4f47/FdwHvznL4beB/+ayH+630X8N48f98NvA//dRD/tb4LeG9euO8G3of/Goj/Ot8FvDcvmu8G3of/fIj/Gt8FvDf/Ot8NvA//uRD/+b4LeG/+bb4beB/+8yD+c30X8N78+3w38D7850D85/ku4L35j/HdwPvwHw/xn+O7gPfmP9Z3A+/DfyzEf7zvAt6b/xzfDbwP/3EQ/7HeG/gu/nO9D/Dd/MdA/Md6a+Cn+M/1NsBP8x8D8d/DPH/ivxbiv4d5/sR/LcR/D/P8if9aiP8e5vkT/7UQ/z3M8yf+ayH+e5jnT/zXQvz3MM+f+K+F+O9hnj/xXwvx38M8f+K/FuK/h3n+xH8txH8P8/yJ/1qI/x7m+RP/tRD/PczzJ/5rIf573Ao8iOf0DODB/NdC/Pf4bOCzeE6fA3w2/7UQ/30+G3hrrvhp4LP5r4f4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeF7Hgc8C3hp4MFfcCnw38Dk8r+PAZwFvDTyYK24Fvhv4HJ7XceCzgLcGHswVtwLfDXwOz+s48FnAWwMP5opbge8GPofndRz4LOCtgQdzxa3AdwOfw3NCPK+vBj6K5+9rgI/mOX018FE8f18DfDTP6auBj+L5+xrgo3lOXw18FM/f1wAfzXP6auCjeP6+Bvhong3xvC4Cx3n+bgUewnO6CBzn+bsVeAjP6SJwnOfvVuAhPKeLwHGev1uBh/CcLgLHef5uBR7CsyGe1y5wjOfvGcCDeU67wDGev2cAD+Y57QLHeP6eATyY57QLHOP5ewbwYJ7TLnCM5+8ZwIN5NsTz+mzgs3j+Pgf4bJ7TZwOfxfP3OcBn85w+G/gsnr/PAT6b5/TZwGfx/H0O8Nk8p88GPovn73OAz+bZEM/fVwNvDTyIK54BfDfw2Tx/Xw28NfAgrngG8N3AZ/P8fTXw1sCDuOIZwHcDn83z99XAWwMP4opnAN8NfDbP31cDbw08iCueAXw38Nk8J8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwS8E3RBJChI6wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoFoldUp;
impl IconShape for GoFoldUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.823 1.677L4.927 4.573A.25.25 0 005.104 5H7.25v3.236a.75.75 0 101.5 0V5h2.146a.25.25 0 00.177-.427L8.177 1.677a.25.25 0 00-.354 0zM13.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zm-3.75.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zM7.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM4 11.75a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5a.75.75 0 01-.75-.75zM1.75 11a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGear;
impl IconShape for GoGear {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.429 1.525a6.593 6.593 0 011.142 0c.036.003.108.036.137.146l.289 1.105c.147.56.55.967.997 1.189.174.086.341.183.501.29.417.278.97.423 1.53.27l1.102-.303c.11-.03.175.016.195.046.219.31.41.641.573.989.014.031.022.11-.059.19l-.815.806c-.411.406-.562.957-.53 1.456a4.588 4.588 0 010 .582c-.032.499.119 1.05.53 1.456l.815.806c.08.08.073.159.059.19a6.494 6.494 0 01-.573.99c-.02.029-.086.074-.195.045l-1.103-.303c-.559-.153-1.112-.008-1.529.27-.16.107-.327.204-.5.29-.449.222-.851.628-.998 1.189l-.289 1.105c-.029.11-.101.143-.137.146a6.613 6.613 0 01-1.142 0c-.036-.003-.108-.037-.137-.146l-.289-1.105c-.147-.56-.55-.967-.997-1.189a4.502 4.502 0 01-.501-.29c-.417-.278-.97-.423-1.53-.27l-1.102.303c-.11.03-.175-.016-.195-.046a6.492 6.492 0 01-.573-.989c-.014-.031-.022-.11.059-.19l.815-.806c.411-.406.562-.957.53-1.456a4.587 4.587 0 010-.582c.032-.499-.119-1.05-.53-1.456l-.815-.806c-.08-.08-.073-.159-.059-.19a6.44 6.44 0 01.573-.99c.02-.029.086-.075.195-.045l1.103.303c.559.153 1.112.008 1.529-.27.16-.107.327-.204.5-.29.449-.222.851-.628.998-1.189l.289-1.105c.029-.11.101-.143.137-.146zM8 0c-.236 0-.47.01-.701.03-.743.065-1.29.615-1.458 1.261l-.29 1.106c-.017.066-.078.158-.211.224a5.994 5.994 0 00-.668.386c-.123.082-.233.09-.3.071L3.27 2.776c-.644-.177-1.392.02-1.82.63a7.977 7.977 0 00-.704 1.217c-.315.675-.111 1.422.363 1.891l.815.806c.05.048.098.147.088.294a6.084 6.084 0 000 .772c.01.147-.038.246-.088.294l-.815.806c-.474.469-.678 1.216-.363 1.891.2.428.436.835.704 1.218.428.609 1.176.806 1.82.63l1.103-.303c.066-.019.176-.011.299.071.213.143.436.272.668.386.133.066.194.158.212.224l.289 1.106c.169.646.715 1.196 1.458 1.26a8.094 8.094 0 001.402 0c.743-.064 1.29-.614 1.458-1.26l.29-1.106c.017-.066.078-.158.211-.224a5.98 5.98 0 00.668-.386c.123-.082.233-.09.3-.071l1.102.302c.644.177 1.392-.02 1.82-.63.268-.382.505-.789.704-1.217.315-.675.111-1.422-.364-1.891l-.814-.806c-.05-.048-.098-.147-.088-.294a6.1 6.1 0 000-.772c-.01-.147.039-.246.088-.294l.814-.806c.475-.469.679-1.216.364-1.891a7.992 7.992 0 00-.704-1.218c-.428-.609-1.176-.806-1.82-.63l-1.103.303c-.066.019-.176.011-.299-.071a5.991 5.991 0 00-.668-.386c-.133-.066-.194-.158-.212-.224L10.16 1.29C9.99.645 9.444.095 8.701.031A8.094 8.094 0 008 0zm1.5 8a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM11 8a3 3 0 11-6 0 3 3 0 016 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF70lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3Ow68FfDWwIOBl+aKvwZ2gZ8Gfga4lX+flwbeCnht4LV5Tj8N/DbwPcAu/3qIf73jwEcBHw0c51/228DHAH/Nv85rA58FvDb/sl3gq4GvAXZ50SH+dY4DvwW8NP96Xw18DrDLC3cc+Czgo/nX+2vgdYBdXjSIF91x4OnAcf7t/hp4HWCX5+848FvAS/Nvtws8BNjlX4Z40RwHfgt4af79/hp4HWCX53Qc+C3gpfn3+2vgZfiXIV40nw18Fv9xfhp4G57TTwFvzX+czwE+mxcO8S87DjwdOM7z9zPAVwO/zRUvDbw28NnAMV6wjwG+mis+GvgqXrBLwGcDvw38NVe8NvDZwGvx/O0CDwF2ecEQ/7L3Br6L5+99gO/m+TsOfDXwXrxgD+GKp/OCfQ/w0cAuz99nA5/F8/c+wHfzgiH+ZT8NvBXP62eAt+Zf9t3Ae/H8/TRXvDXP3/cA782/7LeB1+J5/Qzw1rxgiH/ZXwEvzfN6HeC3edH8NvBa/Ov8DvDavGheG/gtntetwEN4wRD/MvP8iRfdg4Gn86K7BLw0cCsvOvP8iRcM8S8zz5/41/ls4LN40XwO8Nn865jnT7xgiH/ZbwOvxfN6HeC3edEdBy7yojkB7PKie2vgp3hevwO8Ni8Y4l/208Bb8bx+Gngb/nW+G3gvXrjvAd6bf53fAl6b5/UzwFvzgiH+Ze8NfBfP30cDX8OL7q2Bn+KFex3gt3nRfRbw2Tx/7wN8Ny8Y4l92HLgVOMbz99vAZwO/w4tmFzjG83cJOM6L5rWAzwZem+fvEnCcFw7xnI4DHwW8NfDS/N/y18BPA18D7HIF4tneG/gq4Dj/t+0CHwN8N4C44r2B7+L/l/cBvlvAceDpwHH+f9kFHiLgs4HP4v+nzxHwV8BL8//TXwswz9/LAH/Nfw7z/In/HC8N/BXPCwHm+RP/eczzJ/7zmOeFAPP8if885vkT/3nM80KAef7Efx7z/In/POZ5IcA8f+I/j3n+xH8e87wQYJ4/8Z/HPH/iP495Xggwz5/4z2OeP/GfxzwvBJjnT/znMc+f+M9jnhcCzPMn/vOY50/85zHPCwHm+XsZ4K/5z2GeP/Gf46WBv+J5XRLw28Br8f/T7wj4bOCz+P/pcwQcB24FjvH/yyXgweKKtwZ+iv9f3gb4afFsbw18N3CM/9suAe8N/DSAeE7HgYv833YC2OUKxPMyz5/4j2OeP/Efxzx/4tkQz8s8f+I/jnn+xH8c8/yJZ0M8L/P8if845vkT/3HM8yeeDfG8zPMn/uOY50/8xzHPn3g2xPMyz5/4j2OeP/Efxzx/4tkQz8s8f+I/jnn+xH8c8/yJZ0M8L/P8if845vkT/3HM8yeeDfG8zPMn/uOY50/8xzHPn3g2xPMyz5/4j2OeP/Efxzx/4tkQz8s8f+I/jnn+xH8c8/yJZ0M8L/P8if845vkT/3HM8yeeDfG8zPMn/uOY50/8xzHPn3g2xPMyz5/4j2OeP/Efxzx/4tkQz8s8f+I/jnn+xH8c8/yJZ0M8L/P8if845vkT/3HM8yeeDfG8zPMn/uOY50/8xzHPn3g2xPMyz5/4j2OeP/Efxzx/4tkQz8s8f+I/jnn+xH8c8/yJZ0M8L/P8if845vkT/3HM8yeeDfG8zPP3MsBf8x/DPH/iP8ZLA3/F8yeeDfG8/hp4Kf5v+hvgpXk2xPP6bOCz+L/pc4DP5tkQz+s4cCtwjP9bLgEPBnZ5NsTz997Ad/F/y9sAP81zQrxg7w18NXCM/90uAe8N/DTPC/HCHQc+Gnhr4KX43+VvgJ8GvhrY5fnjHwHuZvDXL01MiAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGift;
impl IconShape for GoGift {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.75 1.5a1.25 1.25 0 100 2.5h2.309c-.233-.818-.542-1.401-.878-1.793-.43-.502-.915-.707-1.431-.707zM2 2.75c0 .45.108.875.3 1.25h-.55A1.75 1.75 0 000 5.75v2c0 .698.409 1.3 1 1.582v4.918c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 14.25V9.332c.591-.281 1-.884 1-1.582v-2A1.75 1.75 0 0014.25 4h-.55a2.75 2.75 0 00-2.45-4c-.984 0-1.874.42-2.57 1.23A5.086 5.086 0 008 2.274a5.086 5.086 0 00-.68-1.042C6.623.42 5.733 0 4.75 0A2.75 2.75 0 002 2.75zM8.941 4h2.309a1.25 1.25 0 100-2.5c-.516 0-1 .205-1.43.707-.337.392-.646.975-.879 1.793zm-1.84 1.5H1.75a.25.25 0 00-.25.25v2c0 .138.112.25.25.25h5.5V5.5h-.149zm1.649 0V8h5.5a.25.25 0 00.25-.25v-2a.25.25 0 00-.25-.25h-5.5zm0 4h4.75v4.75a.25.25 0 01-.25.25h-4.5v-5zm-1.5 0v5h-4.5a.25.25 0 01-.25-.25V9.5h4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitBranch;
impl IconShape for GoGitBranch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD+klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/Nc5DrwW8NrASwMPBh7MFbcCtwJ/Dfw28DvALv/5EP/5Hgx8FvDWwHFeNLvAdwNfA9zKfx7Ef67PAj6bf5/PBj6H/xyI/xwvDXwX8NL8x/hr4H2Av+Y/FuI/3ksDvwUc5z/WLvA6wF/zHwfxH+vBwF8Bx/nPsQu8DHAr/zEQ/7H+CnhpXrjvAX4b+Gvgr7nipYGXBt4aeCteuL8GXob/GIj/OF8NfBQv2M8AHw3cygv3YOCrgbfiBfsc4LP590P8x3gw8HResI8Bvpp/nY8GvooX7CHArfz7IP5jfDfwXjx/HwN8Nf82Hw18Fc/f1wAfzb8P4j+Gef5+Bnhr/n1+GngrntcucIJ/H8S/31sDP8Xz9xDgVv59Hgw8nefvbYCf5t8O8e/31cBH8by+B3hv/mP8NPBWPK+vAT6afzsEHAc+C3hv4Dj/cd4H+G7+Y7w38F38x7kV+G7gcwT8FvDa/Md7GeCv+Y/x0sBf8R/vawSY/xziP5b5j3ergF3gGP/xxH8s8x/vGQI+G/gs/uO9DPDX/Md4aeCv+I/3OeKKrwbeGngQ/3HeB/hu/mO8N/Bd/Md5BvDdwGeLf7+vBj6K5/U9wHvzH+OngbfieX0N8NH82yH+/d4a+Cmev4cAt/Lv82Dg6Tx/bwP8NP92iP8Yu8AxntdPA2/Dv89PAW/N87oEHOffB/Ef47uB9+L5+xjgq/m3+Wjgq3j+vgb4aP59EP8xHgw8nRfso4Gv4V/no4Cv5gV7CHAr/z6I/zifDXwWL9hPAx8D3MoL92Dgq4C35gX7HOCz+fdD/Mf6a+CleOF+Gvhp4G+Av+aKlwZeCnhr4K154f4GeGn+YyD+Yz0Y+GvgGP85LgEvDdzKfwzEf7yXBn4bOMZ/rEvAawN/zX8cxH+OBwM/DbwU/zH+Bnhr4Fb+YyH+c3028Fn8+3wO8Nn850D853sw8N7ARwPHeNFcAr4a+G7gVv7zIP5rvTXw2sBLA8eBl+KKvwF2gb8Gfhv4af5rIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwESUX4QzI05wQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitCommit;
impl IconShape for GoGitCommit {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.5 7.75a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0zm1.43.75a4.002 4.002 0 01-7.86 0H.75a.75.75 0 110-1.5h3.32a4.001 4.001 0 017.86 0h3.32a.75.75 0 110 1.5h-3.32z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitCompare;
impl IconShape for GoGitCompare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.573.677L7.177 3.073a.25.25 0 000 .354l2.396 2.396A.25.25 0 0010 5.646V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5h-1V.854a.25.25 0 00-.427-.177zM6 12v-1.646a.25.25 0 01.427-.177l2.396 2.396a.25.25 0 010 .354l-2.396 2.396A.25.25 0 016 15.146V13.5H5A2.5 2.5 0 012.5 11V5.372a2.25 2.25 0 111.5 0V11a1 1 0 001 1h1zm6.75 0a.75.75 0 100 1.5.75.75 0 000-1.5zM4 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFj0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/m9cG3gt4aeClueJW4LeB7wF+m/8dEP86Lw18FfDavHC/DXwM8Nf8z4Z40b008FvAcV40u8DrAH/N/1yIF81LA78FHOdfZxd4GeBW/mdCvGh+C3ht/m1+G3gd/mdC/MteG/gt/n1eB/ht/udB/Mu+G3gvnr/PAb6bK94b+Cyev+8B3pv/eRD/sr8CXprn9TnAZ/OcPhv4LJ7XrcBD+J8H8S8zz99DgFt5Tg8Gns7zJ/7nQfzLzPN3AtjlOT0YeDrPn/ifB/Ev+2vgpXheHwN8Nc/po4Gv4nk9A3gw//Mg/mXfDbwXz2sX+Gzge7jivYCv5vn7HuC9+Z8H8S97beC3+Pd5HeC3+Z8H8aL5beC1+Lf5HeC1+Z8J8aJ5MPDXwDH+dS4BLw3cyv9MiBfdSwO/DRzjRXMJeG3gr/mfC/Gv82Dgu4HX4oXbBV4GuJX/2RD/Nq8N/BYv3NsAP83/bIh/O/PC3Qq8DLDL/1yIfzvzL/tp4G34nwvxb2deNJ8DfDb/OV4beC/gtYEHc8WtwG8D3wP8Ni8c4t/OvOi+G3gf/uO8NPBVwGvzwv028D7ArTx/iH8786/z08D7ALv8+7w08FvAcV40u8DrAH/N80L825l/vVuB9wF+m3+blwZ+CzjOv84u8DLArTwnxL+def7+BngpXrjfBr4a+Bn+dX4LeG3+bX4beB2eE+Lfzjx/J4DfBl6Kf9ku8NPAXwN/DfwNsMvz99rAb/Hv8zrAb/NsiH878/wJOA78NvBS/Oe7BHw28NNc8d7AZ/H8fQ/w3jwb4t/OPH/i2b4a+Cj+c30M8NU8p48GvorndSvwEJ4N8W9nnj/xnF4b+G7gQfznOAHs8pweDDyd5088G+Lfzjx/4vl7b+CrgWP8xzoB7PKcHgw8nedPPBvi3848f+KFe2/grYG34j/G5wCfzXP6bOCzeF7PAB7MsyH+7czzJ140x4HXBl4aeG3gOPBS/Nt8NPAzXPFewGfz/H0P8N48G+Lfzjx/4j/HawO/xb/P6wC/zbMh/u3M8yf+8/w28Fr82/wO8No8J8S/nXn+xH+elwZ+GzjGv84l4KWBW3lOiH878/yJ/1wvDfw2cIwXzSXgtYG/5nkh/u3M8yf+87008NXAa/HC/Q7w3sCtPH+Ifzvz/In/Oq8NvDfw2sCDuOIZwG8D3w38Ni8c4t/mtYHf4vl7HeC3+d8B8a/zYOC7gNfmhftt4H2AW/mfDfGie2ngt4DjvGh2gdcB/pr/uRAvmgcDfwUc519nF3gZ4Fb+Z0K8aH4LeG3+bX4beB3+Z0L8y14b+C3+fV4H+G3+50H8y74beC+e1yXgs4Gf5or3Bj6L5+97gPfmfx7Ev+zpwIN5Xh8DfDXP6aOBr+J5/TXwMvzPg/iXmefvBLDLc3ow8HSeP/E/D+JfZp6/E8Auz+nBwNN5/sT/PIh/2a3Ag3henwN8Ns/ps4HP4nk9A3gw//Mg/mXfDbwXz99HAz/DFe8FfDbP3/cA783/PIh/2WsDv8W/z+sAv83/PIgXzW8Dr8W/ze8Ar83/TIgXzYOBvwaO8a9zCXhp4Fb+Z0K86F4a+G3gGC+aS8BrA3/N/1yIf50HA98NvBYv3O8A7w3cyv9siH+b1wbeG3hp4KW44m+Avwa+G/ht/ndA/P+G+P8N8f8b4v83/hGBi81B2VaV1QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitMerge;
impl IconShape for GoGitMerge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5 3.254V3.25v.005a.75.75 0 110-.005v.004zm.45 1.9a2.25 2.25 0 10-1.95.218v5.256a2.25 2.25 0 101.5 0V7.123A5.735 5.735 0 009.25 9h1.378a2.251 2.251 0 100-1.5H9.25a4.25 4.25 0 01-3.8-2.346zM12.75 9a.75.75 0 100-1.5.75.75 0 000 1.5zm-8.5 4.5a.75.75 0 100-1.5.75.75 0 000 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequest;
impl IconShape for GoGitPullRequest {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequestClosed;
impl IconShape for GoGitPullRequestClosed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.72 1.227a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 111.06 1.061l-.97.97.97.97a.75.75 0 01-1.06 1.06l-.97-.97-.97.97a.75.75 0 11-1.06-1.06l.97-.97-.97-.97a.75.75 0 010-1.06zM12.75 6.5a.75.75 0 00-.75.75v3.378a2.251 2.251 0 101.5 0V7.25a.75.75 0 00-.75-.75zm0 5.5a.75.75 0 100 1.5.75.75 0 000-1.5zM2.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.25 1a2.25 2.25 0 00-.75 4.372v5.256a2.251 2.251 0 101.5 0V5.372A2.25 2.25 0 003.25 1zm0 11a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGitPullRequestDraft;
impl IconShape for GoGitPullRequestDraft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.25 1a2.25 2.25 0 00-.75 4.372v5.256a2.251 2.251 0 101.5 0V5.372A2.25 2.25 0 003.25 1zm0 11a.75.75 0 100 1.5.75.75 0 000-1.5zm9.5 3a2.25 2.25 0 100-4.5 2.25 2.25 0 000 4.5zm0-3a.75.75 0 100 1.5.75.75 0 000-1.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M14 7.5a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0zm0-4.25a1.25 1.25 0 11-2.5 0 1.25 1.25 0 012.5 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGlobe;
impl IconShape for GoGlobe {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.543 7.25h2.733c.144-2.074.866-3.756 1.58-4.948.12-.197.237-.381.353-.552a6.506 6.506 0 00-4.666 5.5zm2.733 1.5H1.543a6.506 6.506 0 004.666 5.5 11.13 11.13 0 01-.352-.552c-.715-1.192-1.437-2.874-1.581-4.948zm1.504 0h4.44a9.637 9.637 0 01-1.363 4.177c-.306.51-.612.919-.857 1.215a9.978 9.978 0 01-.857-1.215A9.637 9.637 0 015.78 8.75zm4.44-1.5H5.78a9.637 9.637 0 011.363-4.177c.306-.51.612-.919.857-1.215.245.296.55.705.857 1.215A9.638 9.638 0 0110.22 7.25zm1.504 1.5c-.144 2.074-.866 3.756-1.58 4.948-.12.197-.237.381-.353.552a6.506 6.506 0 004.666-5.5h-2.733zm2.733-1.5h-2.733c-.144-2.074-.866-3.756-1.58-4.948a11.738 11.738 0 00-.353-.552 6.506 6.506 0 014.666 5.5zM8 0a8 8 0 100 16A8 8 0 008 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD3ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/xwPBr4KeGmu+GvgY4BbedE8GPgq4KW54q+BjwFu5T8W4j/eg4G/Ao7znHaBlwFu5YV7MPBXwHGe0y7wMsCt/MdB/Mf7aeCteP5+BnhrXrifBt6K5+9ngLfmPw7iP95F4DjP363AQ3jhLgLHef5uBR7CfxzEf7xd4BjP3zOAB/PC7QLHeP6eATyY/ziI/3g/DbwVz9/PAG/NC/fTwFvx/P0M8Nb8x0H8x3sw8NfAMZ7TJeClgVt54R4M/DVwjOd0CXhp4Fb+4yD+czwY+Grgpbnir4GPBm7lRfNg4KuBl+aKvwY+GriV/1iI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP8cDwa+Cnhprvhr4GOAW3nRPBj4KuClueKvgY8BbuU/FuI/3oOBvwKO85x2gZcBbuWFezDwV8BxntMu8DLArfzHQfzH+2ngrXj+fgZ4a164nwbeiufvZ4C35j8O4j/eReA4z9+twEN44S4Cx3n+bgUewn8cxH+8XeAYz98zgAfzwu0Cx3j+ngE8mP84iP94Pw28Fc/fzwBvzQv308Bb8fz9DPDW/MdB/Md7MPDXwDGe0yXgpYFbeeEeDPw1cIzndAl4aeBW/uMg/nM8GPhq4KW54q+BjwZu5UXzYOCrgZfmir8GPhq4lf9YiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/HA8Gvgp4aa74a+BjgFt50TwY+Crgpbnir4GPAW7lPxbiP96Dgb8CjvOcdoGXAW7lhXsw8FfAcZ7TLvAywK38x0H8x/tp4K14/n4GeGteuJ8G3orn72eAt+Y/DuI/3kXgOM/frcBDeOEuAsd5/m4FHsJ/HMR/vF3gGM/fM4AH88LtAsd4/p4BPJj/OIj/eD8NvBXP388Ab80L99PAW/H8/Qzw1vzHQfzHezDw18AxntMl4KWBW3nhHgz8NXCM53QJeGngVv7jIP5zPBj4auClueKvgY8GbuVF82Dgq4GX5oq/Bj4auJX/WIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwLT23hBcuOo9gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGrabber;
impl IconShape for GoGrabber {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10 13a1 1 0 100-2 1 1 0 000 2zm-4 0a1 1 0 100-2 1 1 0 000 2zm1-5a1 1 0 11-2 0 1 1 0 012 0zm3 1a1 1 0 100-2 1 1 0 000 2zm1-5a1 1 0 11-2 0 1 1 0 012 0zM6 5a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/bwg4DnwW8NZc8d3A5/D/AwK+GvgontPXAB/N/30IuAgc5zndCjyE//sQYJ4/8X8fAszzJ/7vQ4B5/sT/fQgwz5/4vw8B5vkT//chwDx/4v+ezwLemiu+G/gaAeb5E/+3fBfw3jynjxFgnj/xf8d3Ae/N8/prAeb5E/83fBfw3jx/fyPAPH/if7/vAt6bF+xzBJjnT/zv9l3Ae/OCfQ/w3gLM8yf+9/ou4L15wb4HeG8AAeb5E/87fRfw3rxg3wO8N1cgwDx/4n+f7wLemxfse4D35tkQYJ4/8R/rs4CP5oqvBj6H/1jfBbw3L9j3AO/Nc0KAef7Ef5zvAt6b5/TdwPvwH+O7gPfmBfse4L15Xggwz5/4j/FdwHvz/H038D78+3wX8N68YN8DvDfPHwLM8yf+/b4LeG9euO8G3od/m+8C3psX7HuA9+YFQ4B5/sS/z3cB782L5ruB9+Ff57uA9+YF+x7gvXnhEGCeP/Fv913Ae/Ov893A+/Ci+S7gvXnBvgd4b/5lCDDPn/i3+S7gvfm3+W7gfXjhvgt4b16w7wHemxcNAszzJ/71vgt4b16w7+GK9+IF+27gfXj+vgt4b16w7wHemxcdAszzJ/51vgt4b16w7wHemyu+G3gvXrDvBt6H5/RdwHvzgn0P8N786yDAPH/iRfddwHvzgn0P8N48p+8G3osX7LuB9+GK7wLemxfse4D35l8PAeb5Ey+a7wLemxfse4D35vn7buC9eMG+myvemxfse4D35t8GAeb5E/+y7wLemxfse4D35oX7buC9+Lf5HuC9+bdDgHn+xAv3XcB784J9D/DevGi+G3gv/nW+B3hv/n0QYJ4/8YJ9F/DevGDfA7w3/zrfDbwXL5rvAd6bfz8EmOdPPH+fDXwWL9j3AO/Nv813A+/FC/c9wHvzHwMB5vkTz99fAS/N8/c9wHvz7/PdwHvx/H0P8N78x0GAef7E8/fXwEvxvL4HeG/+Y3w38F48p+8B3pv/WAgwz594/j4a+Cqe0/cA781/rM8G3psrvhv4bP7jIcA8f+IF+2jgvbnip4HP5n8nBJjnT/zfhwDz/In/+xBgnj/xfx8CzPMn/u9DgHn+xP99CDDPn/i/DwHm+RP/9yHAPH/i/z4EmOdP/N+HAPP8if/7EGCeP/F/HwLM8yf+70OAef7E/30IMM+f+L8PAeb/n1uB7wY+R4D5/+trBJj/v24VsAsc4/+nZwj4bOCz+P/pc8QVnw28N/Ag/n94BvDdwGeL/98Q/78h/n9D/P/GPwKzXKKf0mKSZgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoGraph;
impl IconShape for GoGraph {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 1.75a.75.75 0 00-1.5 0v12.5c0 .414.336.75.75.75h14.5a.75.75 0 000-1.5H1.5V1.75zm14.28 2.53a.75.75 0 00-1.06-1.06L10 7.94 7.53 5.47a.75.75 0 00-1.06 0L3.22 8.72a.75.75 0 001.06 1.06L7 7.06l2.47 2.47a.75.75 0 001.06 0l5.25-5.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHash;
impl IconShape for GoHash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.368 1.01a.75.75 0 01.623.859L6.57 4.5h3.98l.46-2.868a.75.75 0 011.48.237L12.07 4.5h2.18a.75.75 0 010 1.5h-2.42l-.64 4h2.56a.75.75 0 010 1.5h-2.8l-.46 2.869a.75.75 0 01-1.48-.237l.42-2.632H5.45l-.46 2.869a.75.75 0 01-1.48-.237l.42-2.632H1.75a.75.75 0 010-1.5h2.42l.64-4H2.25a.75.75 0 010-1.5h2.8l.46-2.868a.75.75 0 01.858-.622zM9.67 10l.64-4H6.33l-.64 4h3.98z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADbklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b41zsOfBbw1lzx3cDn8F/vOPBZwFtzxXcDn8O/DuJf76uBj+I5fQ3w0fzX+mrgo3hOXwN8NC86xL/eReA4z+lW4CH817oIHOc53Qo8hBcd4l/PPH/iv5Z5/sSLDvGvZ54/8V/LPH/iRYf41zPPn/ivZZ4/8aJD/OuZ50/81zLPn3jRIf71zPMn/muZ50+86BD/eub5E/+1zPMnXnSIfz3z/In/Wub5Ey86xL+eef7Efy3z/IkXHeJfzzx/4r+Wef7Eiw7xr2eeP/Ffyzx/4kWH+Nczz5/4r2WeP/GiQ/zrmedP/Ncyz5940SH+9czzJ/5rmedPvOgQ/3rm+RP/tczzJ150iH898/yJ/1rm+RMvOsS/nnn+xH8t8/yJFx3iX888f+K/lnn+xIsO8a9n/mcTLzrEv575n0286BD/euZ/NvGiQ/zrmf/ZxIsO8a9n/mcTLzrEv575n0286BD/eub5E/+1zPMnXnSIfz3z/In/Wub5Ey86xL+eef7Efy3z/IkXHeJfzzx/4r+Wef7Eiw7xr2eeP/Ffyzx/4kWH+Nczz5/4r2WeP/GiQ/zrmedP/Ncyz5940SH+9czzJ/5rmedPvOgQ/3rm+RP/tczzJ150iH898/yJ/1rm+RMvOsS/nnn+xH8t8/yJFx3iX888f+K/lnn+xIsO8a9nnj/xX8s8f+JFh/jXM8+f+K9lnj/xokP865nnT/zXMs+feNEh/vXM8yf+a5nnT7zoEP965vkT/7XM8ydedIh/PfP8if9a5vkTLzrEv555/sR/LfP8iRcd4l9vFzjGc3oG8GD+a+0Cx3hOzwAezIsO8a/32cBn8Zw+B/hs/mt9NvBZPKfPAT6bFx3i3+argbfmiu8GPpv/Hl8NvDVXfDfw2fzrIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPvg1mQVb6c1AAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeading;
impl IconShape for GoHeading {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.75 2a.75.75 0 01.75.75V7h7V2.75a.75.75 0 011.5 0v10.5a.75.75 0 01-1.5 0V8.5h-7v4.75a.75.75 0 01-1.5 0V2.75A.75.75 0 013.75 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeart;
impl IconShape for GoHeart {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.25 2.5c-1.336 0-2.75 1.164-2.75 3 0 2.15 1.58 4.144 3.365 5.682A20.565 20.565 0 008 13.393a20.561 20.561 0 003.135-2.211C12.92 9.644 14.5 7.65 14.5 5.5c0-1.836-1.414-3-2.75-3-1.373 0-2.609.986-3.029 2.456a.75.75 0 01-1.442 0C6.859 3.486 5.623 2.5 4.25 2.5zM8 14.25l-.345.666-.002-.001-.006-.003-.018-.01a7.643 7.643 0 01-.31-.17 22.075 22.075 0 01-3.434-2.414C2.045 10.731 0 8.35 0 5.5 0 2.836 2.086 1 4.25 1 5.797 1 7.153 1.802 8 3.02 8.847 1.802 10.203 1 11.75 1 13.914 1 16 2.836 16 5.5c0 2.85-2.045 5.231-3.885 6.818a22.08 22.08 0 01-3.744 2.584l-.018.01-.006.003h-.002L8 14.25zm0 0l.345.666a.752.752 0 01-.69 0L8 14.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/vbcCXht4aeA48NI8228DtwJ/DfwMcCv/MR4MvBXw0sCDgdfm2f4a2AX+Gvhp4Hd40SFeNMeBrwLeGjjOi+63gc8Bfpt/m9cGPgt4bV50u8BPAx8D7PLCIV6448BHAR8NHOff7reBjwH+mhfNSwNfBbw2/3a7wFcDXwPs8vwhXrAHAz8FvDT/cb4a+BheuK8CPpr/OH8NvA6wy/NCPH8vDfwWcJz/eD8NvA+wy3M6DvwU8Nr8x9sFXgf4a54T4nm9NPBbwHH+8/w18DrALlccB34LeGn+8+wCrwP8Nc+GeE7HgacDx/nP99vA63DFbwGvzX++W4GXAXa5AvGc/gp4af7rfA1XfBT/df4aeBmuQDzbewPfxf8P7wN8N4B4tqcDD+b/h1uBhwCIKz4a+Cr+f/kY4KvFFX8FvDT/v/w18DICHgw8nf+fHiLgo4Gv4v+njxHw08Bb8f/Tzwj4beC1+P/pdwT8FfDS/P/01wLM/18IMP9/IeBW4EH8//QMAb8NvBb/P/2OgN8GXov/n35HwFcDH8X/T18j4K2Bn+L/p7cRcBy4yP9PJ8QVfw28FP+//A7w2uKKjwa+iv9fPgb4anHFceAi/7+cAHbFs3038F78//A9wHsDiGd7beC3+P/hZYC/BhDP6beB1+L/tt8BXpsrEM/pwcDT+b/tIcCtXIF4Xl8NfBT/N30N8NE8G+J5HQduBY7xf8sl4MHALs+GeP7eGvgp/m95G+CneU6IF+yrgY/i/4avAT6a54V4wY4Dvw28FP+7/Q3w2sAuzwvxwr008NvAMf53ugS8NvDXPH+If9lLA3/F/04vA/w1LxjiRfPewHfxv8v7AN/NC4d40b038F387/A+wHfzL0P867w38F38z/Y+wHfzokH867038F38z/Q+wHfzokP827w38F38z/I+wHfzr4P4t3tv4Lv4n+F9gO/mXw/x7/PSwG8Dx/jvcQl4beCv+bdB/Ps9GPhp4KX4r/U3wFsDt/Jvh/iPcRz4auC9+K/xPcBHA7v8+yD+Y3008FX85/oY4Kv5j4H4j/fSwE8DD+I/1jOAtwb+mv84iP8cx4GvBt6L/xg/A7w3sMt/LMR/rrcGvhs4xr/NJeC9gZ/mPwfiP99x4LOBj+Jf52eA9wZ2+c+D+K/z2sB3Aw/ihXsG8N7Ab/OfD/Ff76OBzwaO8ZwuAV8NfDb/dRD/PY4DXw28NVf8NPDRwC7/tRD/vyH+f0P8/4b4/41/BPZ7l15H3dmdAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHeartFill;
impl IconShape for GoHeartFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.655 14.916L8 14.25l.345.666a.752.752 0 01-.69 0zm0 0L8 14.25l.345.666.002-.001.006-.003.018-.01a7.643 7.643 0 00.31-.17 22.08 22.08 0 003.433-2.414C13.956 10.731 16 8.35 16 5.5 16 2.836 13.914 1 11.75 1 10.203 1 8.847 1.802 8 3.02 7.153 1.802 5.797 1 4.25 1 2.086 1 0 2.836 0 5.5c0 2.85 2.045 5.231 3.885 6.818a22.075 22.075 0 003.744 2.584l.018.01.006.003h.002z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHistory;
impl IconShape for GoHistory {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHome;
impl IconShape for GoHome {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.156 1.835a.25.25 0 00-.312 0l-5.25 4.2a.25.25 0 00-.094.196v7.019c0 .138.112.25.25.25H5.5V8.25a.75.75 0 01.75-.75h3.5a.75.75 0 01.75.75v5.25h2.75a.25.25 0 00.25-.25V6.23a.25.25 0 00-.094-.195l-5.25-4.2zM6.906.664a1.75 1.75 0 012.187 0l5.25 4.2c.415.332.657.835.657 1.367v7.019A1.75 1.75 0 0113.25 15h-3.5a.75.75 0 01-.75-.75V9H7v5.25a.75.75 0 01-.75.75h-3.5A1.75 1.75 0 011 13.25V6.23c0-.531.242-1.034.657-1.366l5.25-4.2h-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABlklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/bwg4DnwW8N7Acf5/uBX4buBzBPwW8Nr8//Q1Asz/X7cK2AWO8f/TMwR8NvBZ/P/0OeKKrwbeGngQ/z88A/hu4LPF/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P+NfwRz1g0QoZaKjQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHorizontalRule;
impl IconShape for GoHorizontalRule {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 7.75A.75.75 0 01.75 7h14.5a.75.75 0 010 1.5H.75A.75.75 0 010 7.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/2XHgs4C3Bh7M/w63At8NfA4vHOJf9tXAR/G/09cAH80LhviXXQSO87/TrcBDeMEQ/7Jd4Bj/Oz0DeDAvGOJf9tnAZ/G/0+cAn80LhnjRfDXw1sCD+N/hGcB3A5/NC4f4tzPPn/jPZZ4/8a+H+Lczz5/4z2WeP/Gvh/i3M8+f+M9lnj/xr4f4tzPPn/jPZZ4/8a+H+Lczz5/4z2WeP/Gvh/i3M8+f+M9lnj/xr4f4tzPP3+sAv81/jtcGfovnT/zrIf7t/hp4KZ7X6wC/zX+O1wZ+i+f1O8Br86+H+Lf7buC9eF5fA3w0/zm+Gvgontf3AO/Nvx7i3+6jga/ied0KPIT/HE8HHszz+hjgq/nXQ/zbPRh4Os/f6wC/zX+stwZ+iufvIcCt/Osh/n1uBR7E8/pr4GX4j/VXwEvzvJ4BPJh/G8S/z3sD38Xz9znAZ/Mf47OBz+L5ex/gu/m3Qfz7HAduBY7x/L0M8Nf8+7w08Fc8f5eABwO7/Nsg/v0+Gvgqnr9d4H2An+bf5r2BrwKO8/x9DPDV/Nsh/mP8NfBSvGDvA3w3/zrvDXwXL9jfAC/Nvw/iP8aDgb8GjvGCfTfwPrxovgt4b16wS8BLA7fy74P4j/PewHfxwn038D68cN8FvDcv3NsAP82/H+I/1nsD38UL9zbAT/P8vTXwU7xw7wN8N/8xEP/x3hr4buAYz99fAy/D8/d04ME8f5eAtwZ+m/84iP8cLw18N/BSPH8vA/w1z+mlgb/i+fsb4L2Bv+Y/FuI/z3Hgr4EH8bxeB/htntNrA7/F83oG8NLALv/xEP95jgN/BTyY5/U6wG/znF4b+C2e163AywC7/MdD/Od4aeC7gJfm+XsZ4K95Ti8N/BXP318D7wP8Nf+xEP/x3hr4LuA4z9/fAC/N8/fXwEvx/O0CbwP8Nv9xEP+x3hv4Ll64twF+mufvrYGf4oV7H+C7+Y+B+I/z3sB38cJ9D/DevHDfDbwXL9zbAD/Nvx/iP8aDgb8CjvOCfQ/w3rxovht4L16wXeBlgFv590H8x/gr4KV5wd4H+G7+dd4b+C5esL8GXoZ/H8S/30cDX8Xzdwl4b+Cn+bd5b+CrgWM8fx8DfDX/doh/n+PA04HjPH8vA/w1/z4vDfwVz98u8BBgl38bxL/PewPfxfP3OcBn8x/js4HP4vl7H+C7+bdB/Ps8HXgwz+tvgJfmP9ZfAy/F87oVeAj/Noh/uwcDT+f5ex3gt/mP9dbAT/H8PQS4lX89xL/dRwNfxfN6BvBg/nPcCjyI5/UxwFfzr4f4t/tu4L14Xl8DfDT/Ob4a+Cie1/cA782/HuLf7q+Al+Z5vQ7w2/zneG3gt3hevwO8Nv96iH878/y9DvDb/Od4beC3eP7Evx7i3848f+I/l3n+xL8e4t/OPH/iP5d5/sS/HuLfzjx/4j+Xef7Evx7i3848f+I/l3n+xL8e4t/OPH/iP5d5/sS/HuLfzjx/4j+Xef7Evx7iX3Yc+CzgrYEH87/DrcB3A5/DC4f4l3018FH87/Q1wEfzgiH+ZReB4/zvdCvwEF4wxL9sFzjG/07PAB7MC4b4l3028Fn87/Q5wGfzgiFeNF8NvDXwIP53eAbw3cBn88Ih/n9D/P+G+P8N8f8b/wixR75BFy93igAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHourglass;
impl IconShape for GoHourglass {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 1a.75.75 0 000 1.5h.75v1.25a4.75 4.75 0 001.9 3.8l.333.25c.134.1.134.3 0 .4l-.333.25a4.75 4.75 0 00-1.9 3.8v1.25h-.75a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5h-.75v-1.25a4.75 4.75 0 00-1.9-3.8l-.333-.25a.25.25 0 010-.4l.333-.25a4.75 4.75 0 001.9-3.8V2.5h.75a.75.75 0 000-1.5H2.75zM11 2.5H5v1.25a3.25 3.25 0 001.3 2.6l.333.25c.934.7.934 2.1 0 2.8l-.333.25a3.25 3.25 0 00-1.3 2.6v1.25h6v-1.25a3.25 3.25 0 00-1.3-2.6l-.333-.25a1.75 1.75 0 010-2.8l.333-.25a3.25 3.25 0 001.3-2.6V2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoHubot;
impl IconShape for GoHubot {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 8a8 8 0 1116 0v5.25a.75.75 0 01-1.5 0V8a6.5 6.5 0 10-13 0v5.25a.75.75 0 01-1.5 0V8zm5.5 4.25a.75.75 0 01.75-.75h3.5a.75.75 0 010 1.5h-3.5a.75.75 0 01-.75-.75zM3 6.75C3 5.784 3.784 5 4.75 5h6.5c.966 0 1.75.784 1.75 1.75v1.5A1.75 1.75 0 0111.25 10h-6.5A1.75 1.75 0 013 8.25v-1.5zm1.47-.53a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 011.06 0l.97.97.97-.97a.75.75 0 111.06 1.06l-1.5 1.5a.75.75 0 01-1.06 0L8 7.81l-.97.97a.75.75 0 01-1.06 0l-1.5-1.5a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE20lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovivcRz4KOCtgZfmhftr4KeBrwF2+c+F+M/33sBXAcf519kFPgb4bv7zIP5zvTfwXfz7vA/w3fznQPznOQ48HTjOv88u8BBgl/94iP88nw18Fv8xPgf4bP7jIf7z/BXw0vzH+GvgZfiPh/jPY56/lwH+mufvpYG/4vkT//EQ/3nM8ydeOPP8if94iP885vkTL5x5/sR/PMR/HvP8iRfOPH/iPx7iP495/sQLZ54/8R8P8Z/HPH/ihTPPn/iPh3hOx4GPAt4aeGn+c4gXzvzn+Gvgp4GvAXa5AvFs7w18FXCc/1zihTP/uXaBjwG+G0Bc8d7Ad/FfQ7xw5r/G+wDfLeA48HTgOP81xAtn/mvsAg8R8NnAZ/FfR7xw5r/O5wj4K+Cl+a/zMsBf8/y9NPBX/Nf5awHm+XsZ4K/5t/tr4KX4j/E3wEvzb/fSwF/xvBBgnj/x7/PZwGfxH+NzgM/m38c8LwSY50/8+zwY+GvgGP8+l4CXBm7l38c8LwSY50/8+7038F38+7wN8NP8+5nnhQDz/In/GG8NfDdwjH+dS8B7Az/NfwzzvBBgnj/xH+c48NHAWwMvxQv3N8BPA18N7PIfxzwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xPP32sB3AQ/mhbsVeB/gt3lOx4HPAt4aeDD/MW4Fvhv4HF4w87wQYJ4/8fw9HXgwL5pbgYfwnL4a+Cj+c3wN8NE8f+Z5IcA8f+L5M/864jldBI7zn+NW4CE8f+Z5IcA8f+L5M/864jntAsf4z/EM4ME8f+Z5IcA8f+L5M/864jl9NvBZ/Of4HOCzef7M80KAef7E82f+dcTz+mrgrYEH8R/jGcB3A5/NC2aeFwLM8yeeP/OvI/5nMM8LAeb5E8+f+dcR/zOY54UA8/yJ58/864j/GczzQoB5/sTzZ/51xP8M5nkhwDx/4vkz/zriOR0HPgt4a+DBvGhuBb4b+Bz+7czzQoB5/sTzZ/51xHP6auCj+Lf5GuCj+bcxzwsB5vkTz5/51xHP6SJwnH+bW4GH8G9jnhcCzPMnnj/zryOe0y5wjH+bZwAP5t/GPC8EmOdPPH+7wDFeNM8AHsxz+mzgs/i3+Rzgs/m3Mc8LAeb5E8/fawPfDTyIF+4ZwHsDv83z+mrgrYEH8aJ5BvDdwGfzb2eeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5exngr/m/4aWBv+J5IeCvgZfi/6e/EfDZwGfx/9PnCDgO3Aoc4/+XS8CDxRXvDXwX/7+8DfDT4tneG/hq4Bj/t10C3hv4aQDxnI4DHw28NfBS/N/yN8BPA18N7HIF4v83xP9viP/fEP+/8Y/g2szcG1nW7gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIdBadge;
impl IconShape for GoIdBadge {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3 7.5a.5.5 0 01.5-.5h2a.5.5 0 01.5.5v3a.5.5 0 01-.5.5h-2a.5.5 0 01-.5-.5v-3zm10 .25a.75.75 0 01-.75.75h-4.5a.75.75 0 010-1.5h4.5a.75.75 0 01.75.75zM10.25 11a.75.75 0 000-1.5h-2.5a.75.75 0 000 1.5h2.5z",
            }
            path {
                d: "M7.25 0A1.75 1.75 0 005.5 1.75V3H1.75A1.75 1.75 0 000 4.75v8.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H10.5V1.75A1.75 1.75 0 008.75 0h-1.5zm3.232 4.5A1.75 1.75 0 018.75 6h-1.5a1.75 1.75 0 01-1.732-1.5H1.75a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25h-3.768zM7 1.75a.25.25 0 01.25-.25h1.5a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25h-1.5A.25.25 0 017 4.25v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoImage;
impl IconShape for GoImage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h.94a.76.76 0 01.03-.03l6.077-6.078a1.75 1.75 0 012.412-.06L14.5 10.31V2.75a.25.25 0 00-.25-.25H1.75zm12.5 11H4.81l5.048-5.047a.25.25 0 01.344-.009l4.298 3.889v.917a.25.25 0 01-.25.25zm1.75-.25V2.75A1.75 1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25zM5.5 6a.5.5 0 11-1 0 .5.5 0 011 0zM7 6a2 2 0 11-4 0 2 2 0 014 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/u5cGPgp4aeCl+e/x18BfA58D3Mq/HuLf5ruA9+Z/lu8G3od/HcS/3ncB783/TN8NvA8vOsS/zlsDP8X/bG8D/DQvGsS/ztOBB/M/263AQ3jRIF507w18F/87vA/w3fzLEC+63wJem+f1N8BrA7v81zoO/DbwUjyv3wZeh38Z4kXz0sBf8fy9D/Dd/Pd4b+C7eP5eBvhrXjjEi+a7gffieT0DeDD/vW4FHsTz+h7gvXnhEP+y48BFnr/PAT6b/16fDXwWz98JYJcXDPEv+2zgs3j+TgC7/Pc6Dlzk+fsc4LN5wRD/sqcDD+Z5fQ/w3vzP8N3Ae/G8bgUewguGeOHeG/gunr+XAf6a/xleGvgrnr/3Ab6b5w/xwv0W8No8r98BXpv/WX4beC2e128Dr8Pzh3jBXhr4K56/9wG+m/9Z3hv4Lp6/lwH+mueFeMG+G3gvntczgAfzP9OtwIN4Xt8DvDfPC/H8HQcu8vx9DvDZ/M/02cBn8fydAHZ5Tojn77OBz+L5OwHs8j/TceAiz9/nAJ/Nc0I8f08HHszz+h7gvfmf7buB9+J53Qo8hOeEeF7vDXwXz9/LAH/N/2wvDfwVz9/7AN/NsyGe128Br83z+h3gtfnf4beB1+J5/TbwOjwb4jm9NPBXPH/vA3w3/zu8N/BdPH8vA/w1VyCe03cD78XzegbwYP53uRV4EM/re4D35grEsx0HLvL8fQ7w2fzv8tnAZ/H8nQB2AcSzfTbwWTx/J4Bd/nc5Dlzk+fsc4LMBxLM9HXgwz+t7gPfmf6fvBt6L53Ur8BAAccV7A9/F8/cywF/zv9NLA3/F8/c+wHeLK34LeG2e1+8Ar83/br8NvBbP67eB1xHw0sBf8R9jF/hu4GP4z/FVwHsDx/mP8TICvht4L/5jfTfwPvzH+i7gvfmP9T0CLgLH+Y/3PsB38x/jvYHv4j/ergDzn2MXeB3gr/n3eWngt4Dj/MdDgPnP89fA6wC7/NscB34LeGn+cyDA/Of6buB9+Lf5LuC9+c+DAPP8iRfdSwO/DRzj+Xsf4Lv513lv4Lt4/i4Brw38NS8687wQYJ4/8a/z3sB38fztAq8D/DUvmpcGfgs4zvP3PsB3869jnhcCzPMn/vW+G3gv/nN9D/De/OuZ54UA8/yJf73jwG8DL8V/jr8BXhvY5V/PPC8EmOdP/Nu8NPDbwDH+Y10CXhv4a/5tzPNCgHn+xL/dewPfxX+s9wG+m38787wQYJ4/8e/z3cB78R/je4D35t/HPC8EmOdP/Pt9NfDewDH+bS4B3w18NP9+5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef5eBvhr/m94aeCveF4I+Gvgpfj/6W8EfDbwWfz/9DkCjgO3Asf4/+US8GBxxXsD38X/L28D/LR4tvcGvho4xv9tl4D3Bn4aQDyn48BHA28NvBT/t/wN8NPAVwO7XIH4/w3x/xvi/zfE/2/8I19Sxk2fiFbPAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInbox;
impl IconShape for GoInbox {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.8 2.06A1.75 1.75 0 014.41 1h7.18c.7 0 1.333.417 1.61 1.06l2.74 6.395a.75.75 0 01.06.295v4.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25v-4.5a.75.75 0 01.06-.295L2.8 2.06zm1.61.44a.25.25 0 00-.23.152L1.887 8H4.75a.75.75 0 01.6.3L6.625 10h2.75l1.275-1.7a.75.75 0 01.6-.3h2.863L11.82 2.652a.25.25 0 00-.23-.152H4.41zm10.09 7h-2.875l-1.275 1.7a.75.75 0 01-.6.3h-3.5a.75.75 0 01-.6-.3L4.375 9.5H1.5v3.75c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V9.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFX0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4kXz0sBbAa8NHAdemiv+GtgFfhv4GeCv+a/x0sBbAa8NHAdemiv+GtgFfhv4GeCveeEQL9xrA98FPJgXza3AZwPfw3+O9wI+G3gwL5pbgfcBfpvnD/H8HQd+Cnht/m12gc8Gvob/GB8FfDZwnH+b3wbeBtjlOSGe10sDvwUc59/vt4GPAf6af5uXBr4KeG3+/XaB1wH+mmdDPKeXBn4LOM5/rI8GvoZ/nY8Cvpr/WLvA6wB/zRWIZzsOPB04zn+O7wY+BtjlhTsOfBfw1vzn2AUeAuwCiGf7LeC1ecGeAXw18NvAX3PFSwOvDbw38FL8y/4aeB/gr3n+Xhr4LuCl+Zf9DfDdwG8Df80VLw28NvDRwIN4wX4beB0AccVrA7/FC/YxwFfzwr028NnAa/HC7QJvA/w2z+m1gZ8CjvPC/Q7w2cBv88J9NPBVvGCvA/y2uOLpwIN5/t4G+GledB8NfDZwjBfufYDv5or3Br6LF+4S8NnAV/Oie2vgp3j+bgUeIuClgb/i+fsY4Kv513sw8NPAS/HCfTZXfDYv3N8Abw3cyr/eRwNfxfP3MgI+G/gsntczgAfz7/PVwEfx7/M1wEfz73Mr8CCe18cI+G3gtXheHwN8Nf9+7w18F/827wN8N/9+Hw18Fc/rdwT8FfDSPK+XAf6a/xgvDfw2cIwXzSXgtYG/5j/GSwN/xfP6awHm+RP/sV4a+GngQbxwzwDeGvhr/mOZ54UA8/yJ/1gvDfwU8GBeuFuBtwH+mv9Y5nkh4K+Bl+J5vQzw1/zHeGngt4DjvGh2gdcB/pr/GC8N/BXP628E/DbwWjyvjwG+mn+/9wa+i3+b9wG+m3+/jwa+iuf1OwI+G/gsntetwEP49/ku4L359/lq4GP493k68GCe18cIeGngr3j+Pgb4av71Hgz8FPDSvHCfwxWfxQv318DbALfyr/fRwFfx/L2MuOJW4EE8f28D/DQvuo8CPhs4zgv3PsB3c8V7A9/FC7cLfDbwNbzo3hr4KZ6/ZwAPFle8NvBbvGAfDXwNL9xrA58FvDYv3CXgrYHf5jm9NvDTwDFeuN8GPgf4bV64jwK+mhfsdYDfFs/228Br8YLdCnw18DvAX3PFawEvDbw38NL8y/4GeG/gr3n+Xhr4buCl+Jf9NfDdwF8Dv8MVLw28FvDRwIN5wX4HeG0A8WzHgVuBY/zn+B7go4FdXrjjwFcD78V/jkvAg4FdAPGcXhr4beAY/7E+Bvhq/nU+Gvgq/mNdAl4b+GuuQDyvlwZ+GzjGv9/vAB8N/DX/Ni8NfDXwWvz7XQJeG/hrng3x/B0Hfhp4Lf5tngF8NfDV/Mf4aOCjgQfxb/M7wFsDuzwnxAv32sB3Aw/iRfMM4LOB7+Y/x3sDnw08iBfNM4D3Bn6b5w/xonlp4LWBtwaOAy/FFX8D7AI/Dfw28Nf813hp4K2B1waOAy/FFX8D7AI/Dfw28Ne8cIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y+RIMqeeTBpRQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInfinity;
impl IconShape for GoInfinity {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.5 6c-1.086 0-2 .914-2 2 0 1.086.914 2 2 2 .525 0 1.122-.244 1.825-.727.51-.35 1.025-.79 1.561-1.273-.536-.483-1.052-.922-1.56-1.273C4.621 6.244 4.025 6 3.5 6zm4.5.984c-.59-.533-1.204-1.066-1.825-1.493-.797-.548-1.7-.991-2.675-.991C1.586 4.5 0 6.086 0 8s1.586 3.5 3.5 3.5c.975 0 1.878-.444 2.675-.991.621-.427 1.235-.96 1.825-1.493.59.533 1.204 1.066 1.825 1.493.797.547 1.7.991 2.675.991 1.914 0 3.5-1.586 3.5-3.5s-1.586-3.5-3.5-3.5c-.975 0-1.878.443-2.675.991-.621.427-1.235.96-1.825 1.493zM9.114 8c.536.483 1.052.922 1.56 1.273.704.483 1.3.727 1.826.727 1.086 0 2-.914 2-2 0-1.086-.914-2-2-2-.525 0-1.122.244-1.825.727-.51.35-1.025.79-1.561 1.273z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoInfo;
impl IconShape for GoInfo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm6.5-.25A.75.75 0 017.25 7h1a.75.75 0 01.75.75v2.75h.25a.75.75 0 010 1.5h-2a.75.75 0 010-1.5h.25v-2h-.25a.75.75 0 01-.75-.75zM8 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIEElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdxx4LeClgZcGjgMPBh7Mc7oVuBXYBf4a+Gvgd4Bd/vMg/nO8NPBewGsDL82/z18Dvw18D/DX/MdC/Mc5DrwX8NHAg/nPcSvw1cD3ALv8+yH+/Y4DHwV8NHCc/xq7wFcDXwPs8m+H+Pd5L+CrgeP899gFPhr4Hv5tEP82Dwa+C3ht/mf4beB9gFv510H867018F3AcV50zwB+G/hr4K+BW4FbeU4PBh4MvDTw0sBrAw/iRbcLvA/w07zoEP86XwV8NC+avwG+G/hp4Fb+bR4MvDXw3sBL8aL5auBjeNEgXnTfBbw3/7LfAT4b+G3+Y7028NnAa/Ev+27gffiXIV40PwW8NS/c3wDvDfw1/7leG/hq4KV44X4aeBteOMS/7LuA9+YFuwR8NvDV/Nf6aOCreOG+G3gfXjDEC/fVwEfxgv0N8N7AX/Pf46WBnwYexAv2NcBH8/whXrC3Bn6KF+xvgNcGdvnvdRz4beCleMHeBvhpnhfi+Xsw8FfAcZ6/7wE+Gtjlf4bjwE8Dr8Xztwu8DHArzwnx/P0W8No8f38DvDawy/8sx4HfBl6K5++3gdfhOSGe13sD38Xz9zfAawO7/M90HPhr4EE8f+8DfDfPhnhOx4GnA8d5XpeA1wb+mv/ZXhr4K56/XeAhwC5XIJ7TZwOfxfP3McBX87/DRwNfxfP3OcBncwXi2Y4DTweO87z+Bnhp/nf5a+CleF67wEOAXQDxbB8NfBXP3+sAv83/Lq8N/BbP38cAXw0gnu3pwIN5Xr8DvDb/O/028Fo8r1uBhwCIK14a+Cuev9cBfpv/nV4b+C2ev5cB/lpc8dXAR/G8/gZ4af77fRbw1lzx3cDX8KL7a+CleF5fA3y0uOKvgJfmeX0M8NX89/ou4L15Th8DfDUvmo8Gvorn9dfAywg4Dlzk+XsIcCv/fb4LeG+e118DL8OL5sHA03n+Tgh4a+CneF7PAB7Mf5/vAt6b5+9vgJfmRXcr8CCe19sI+Gzgs3heXwN8NP89vgt4b16wzwE+mxfddwPvxfP6HAE/DbwVz+tjgK/mv953Ae/NC/Y9wHvzr/PRwFfxvH5GwG8Dr8Xzeh3gt/nX+Sjgvbnip4HP4V/nu4D35gX7HuC9+dd7beC3eF6/I+DpwIN5Xg8BbuVF99HAV/Gcvht4H1403wW8Ny/Y9wDvzb/Ng4Gn87z+WoB5/sS/zl8BL83z+m7gfXjhvgt4b16w7wHem38f87wQYJ4/8a/z18BL8fx9N/A+PH/fBbw3L9j3AO/Nv595Xggwz5/41/ls4LN4wb4beB+e03cB780L9j3Ae/MfwzwvBJjnT/zrfTfwXrxg3w28D1d8F/DevGDfA7w3/3HM80LArcCDeF4PAW7lX++7gffiBfturnhvXrDvAd6b/zgPBp7O83qGgN8GXovn9TrAb/Nv893Ae/Fv8z3Ae/Mf67WB3+J5/Y6Anwbeiuf1McBX82/33cB78a/zPcB78x/vo4Gv4nl9j4DPBj6L5/U1wEfz7/PdwHvxovke4L35z/HdwHvxvD5HwFsDP8XzuhV4CP9+3w28Fy/c9wDvzX+epwMP5nm9jYDjwEWev4cAt/Lv993Ae/H8fQ/w3vzneTDwdJ6/E+KKvwZeiuf1McBX8x/ju4H34jl9D/De/Of6aOCreF5/A7y0uOKrgY/ief018DL8x/ls4L254ruBz+Y/318BL83z+hrgo8UVLw38Fc/f6wC/zf9Orw38Fs/fywB/LZ7tVuBBPK/fBl6H/51+C3htntczgAcDiGf7aOCreP5eB/ht/nd5beC3eP4+BvhqAPFsx4FbgWM8r78GXob/Xf4KeGme1yXgwcAugHhOnw18Fs/fxwBfzf8OHw18Fc/f5wCfzRWI53QcuBU4xvP3MsBf8z/bSwN/xfN3CXgwsMsViOf13sB38fzdCrwMsMv/TMeBvwIezPP3PsB382yI5++3gdfi+ftr4HWAXf5nOQ78FvDSPH+/A7w2zwnx/D0Y+GvgGM/fbwNvA+zyP8Nx4KeA1+b5uwS8NHArzwnxgr018FO8YH8NvA6wy3+v48BvAS/NC/Y2wE/zvBAv3FcDH8UL9tfA+wB/zX+PlwZ+CngwL9jXAB/N84f4l3038F68cB8NfA3/tT4K+GpeuO8B3psXDPGi+WngrXjh/hr4GOC3+c/12sBXAS/NC/c9wHvzwiFedN8NvBf/st8GPgf4bf5jvTbwWcBr8y/7HuC9+Zch/nW+GvgoXjR/DXw38DPArfzbPBh4K+C9gZfmRfM1wEfzokH867018N3AMV50twK/Dfw18NfAM4BbeU4PBh4EvDTw0sBrAw/mRXcJeG/gp3nRIf5tHgx8N/Ba/M/wO8B7A7fyr4P493lv4KuBY/z3uAR8NPDd/Nsg/v2OAx8NfDRwjP8al4CvBr4a2OXfDvEf5zjw3sBHAw/iP8czgK8GvhvY5d8P8Z/jpYH3Bl4beCn+ff4G+G3gu4G/5j8W4j/fceC1gZcGXho4DjwYeBDP6RnArcAu8NfAXwO/Dezyn4d/BDVgTYtoKOGGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueClosed;
impl IconShape for GoIssueClosed {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.28 6.78a.75.75 0 00-1.06-1.06L7.25 8.69 5.78 7.22a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l3.5-3.5z",
            }
            path {
                d: "M16 8A8 8 0 110 8a8 8 0 0116 0zm-1.5 0a6.5 6.5 0 11-13 0 6.5 6.5 0 0113 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueDraft;
impl IconShape for GoIssueDraft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.749.097a8.054 8.054 0 012.502 0 .75.75 0 11-.233 1.482 6.554 6.554 0 00-2.036 0A.75.75 0 016.749.097zM4.345 1.693A.75.75 0 014.18 2.74a6.542 6.542 0 00-1.44 1.44.75.75 0 01-1.212-.883 8.042 8.042 0 011.769-1.77.75.75 0 011.048.166zm7.31 0a.75.75 0 011.048-.165 8.04 8.04 0 011.77 1.769.75.75 0 11-1.214.883 6.542 6.542 0 00-1.439-1.44.75.75 0 01-.165-1.047zM.955 6.125a.75.75 0 01.624.857 6.554 6.554 0 000 2.036.75.75 0 01-1.482.233 8.054 8.054 0 010-2.502.75.75 0 01.858-.624zm14.09 0a.75.75 0 01.858.624 8.057 8.057 0 010 2.502.75.75 0 01-1.482-.233 6.55 6.55 0 000-2.036.75.75 0 01.624-.857zm-13.352 5.53a.75.75 0 011.048.165 6.542 6.542 0 001.439 1.44.75.75 0 01-.883 1.212 8.04 8.04 0 01-1.77-1.769.75.75 0 01.166-1.048zm12.614 0a.75.75 0 01.165 1.048 8.038 8.038 0 01-1.769 1.77.75.75 0 11-.883-1.214 6.543 6.543 0 001.44-1.439.75.75 0 011.047-.165zm-8.182 3.39a.75.75 0 01.857-.624 6.55 6.55 0 002.036 0 .75.75 0 01.233 1.482 8.057 8.057 0 01-2.502 0 .75.75 0 01-.624-.858z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueOpened;
impl IconShape for GoIssueOpened {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z",
            }
            path {
                d: "M8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAG80lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjP99LAWwEvDTwYeGlesN/mit8G/hr4HWCX/zyI/xxvBbw18NbAcf59bgW+G/ge4Fb+YyH+4xwHPgp4b+DB/Of4a+CrgZ8Bdvn3Q/z7HQc+Cvho4Dj/NXaBrwY+h38fxL/PRwGfDRznv8etwGcD38O/DeLf5qWBrwJem/8Z/hp4HWCXfx3Ev957A18FHOdF9zfAXwN/Dfw1cCtwK8/ppYHjwEsDLw28NPBSvOh2gbcBfpsXHQI+CtgFvod/2VcBH82L5m+A7wZ+GriVf5sHA28NvDfwUrxoPhv4HF40CDBXfDfwPrxg3wW8N/+ynwG+Gvht/mO9NvDZwGvxL/tu4H34lyHAPNt3A+/D8/ou4L154f4G+Gjgt/nP9dLAdwMvxQv33cD78MIhwDyn7wbeh2f7LuC9eeE+Bvhq/mt9NPDZwDFesO8G3ocXDAHmeX038D7AVwMfxQv2DOCtgb/mv8dLAz8NPIgX7GuAj+b5Q4B5/n4beG1esL8BXhvY5b/XceC3gZfiBXsb4Kd5Xggw/3p/A7w2sMv/DMeB3wZeiudvF3gIsMtzQoD51/kb4LWBXf5nOQ78NvBSPH+/DbwOzwkB5kX3DOClgV3+ZzoO/DXwIJ6/9wG+m2dDgHnRvQzw1/zP9tLAX/H83Qo8hGdDgHnRfTfwPvzP99HAV/H8fQzw1VyBAPOv893A+/A/318DL8Xz2gUeAuwCCDD/et8NvA//s7028Fs8fx8DfDWAAPNv893A+/A/208Db8Xz+mvgZQAEmH+77wbeh/+5Xhv4LZ6/hwC3CjD/Pu8DfDf/c90KPIjn9THAVwv4a+Cl+Ld7G+Cn+Z/ro4Gv4nn9NfAy4v++BwNP5/k7If5/uBV4EM/rdcR/j/cC3ho4zhW3At8D/Db/Ob4beC+e18eI/1rHgd8CXprn76uBj+E/3mcDn8Xz+h7xX+c48FvAS/PCfQ3w0fzHem3gt3hevyP+63w28Fm8aF4H+G3+4xwHLvK8fkf817kIHOdF8zXAR/Mf67eB1+I5vY/4r3EcuMiL7neA1+Y/1nHgq4H3Ai4BXw18trjis4CPBo4D3w18DLDLf5zjwEVedL8DvDb/+RDw3sB38Zx+Gngb/mPtAsd40fwM8Nb850PAbwOvxfM6AezyH+e7gffiRfM+wHfznw8Bvw28Fs/rdYDf5j/Og4G/Bo7xwv0N8NL810DAdwPvxfP6HOCz+Y/12sBPA8d4/v4GeG1gl/8aCPho4Kt4Xt8DvDf/8R4MfDbw1sAxrngG8N3AVwO7/NdBwGsDv8Xz+mvgZfjP9WDgVv77IOA4cJHn7yHArfzfhbjir4GX4nl9DvDZ/M/11sBP8W/31+KKjwa+iud1K/AQ/ud6a+Cn+LdDXPFg4Ok8f68D/Db/c7038F382yCe7a+Bl+J5/TbwOvzP9t7Ad/Gvh3i2jwa+iufvdYDf5n+29wa+i38dxLMdB24FjvG8fht4Hf7ne2/gu3jRIZ7TRwNfxfP3OcBn8z/fewPfxYsG8bxuBR7E8/cywF/zP9tLA78FHOdfhnhe7w18F8/frcDLALv8z3Qc+CvgwbxoEM/fbwOvxfP318DrALv8z3Ic+C3gpXnRIZ6/48CtwDGev78GXgfY5X+G48BvAS/Nvw7iBXtr4Kd4wf4aeB1gl/9ex4HfAl6aF+x7gPfieSFeuK8GPooX7FbgbYC/5r/HSwM/BTyYF+xzgM8G3hv4Lp4T4l/23cB78cJ9NPA1/Nf6KOCreeG+B3hvnu29ge/i2RAvmu8G3osX7q+BjwF+m/9crw18FfDSvHDfA7w3z+u9ge/iCsSL7ruB9+Jf9tvA5wC/zX+s1wY+C3ht/mXfA7w3L9h7A98FIP51Phv4LF40twI/DXwNcCv/Ng8G3gr4aODBvGi+Bvho/mVvDSD+9V4b+GngGC+6vwb+Gvhr4K+54nd4Tq/FFS8NvDTw2sCDedFdAt4b+GledIh/m+PATwOvxf8MvwO8N3Ar/zqIf5/3Bj4beBD/PZ4BfDbw3fzbIP5jfDTw2cAx/mtcAr4a+Gpgl387xH+c48BbAx8NvBT/Of4G+Grgp4Fd/v0Q/zkeDLw38NbAS/Hv8zfATwM/Dfw1/7EQ/zVeG3ht4KWB41zx0sAxnu13uOJW4Fbgt4G/Bnb5z8M/AutZG6B5oKhgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIssueReopened;
impl IconShape for GoIssueReopened {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.029 2.217a6.5 6.5 0 019.437 5.11.75.75 0 101.492-.154 8 8 0 00-14.315-4.03L.427 1.927A.25.25 0 000 2.104V5.75A.25.25 0 00.25 6h3.646a.25.25 0 00.177-.427L2.715 4.215a6.491 6.491 0 012.314-1.998zM1.262 8.169a.75.75 0 00-1.22.658 8.001 8.001 0 0014.315 4.03l1.216 1.216a.25.25 0 00.427-.177V10.25a.25.25 0 00-.25-.25h-3.646a.25.25 0 00-.177.427l1.358 1.358a6.501 6.501 0 01-11.751-3.11.75.75 0 00-.272-.506z",
            }
            path {
                d: "M9.06 9.06a1.5 1.5 0 11-2.12-2.12 1.5 1.5 0 012.12 2.12z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADoklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z3cc+CzgrYEH81/jVuC7gc/hhUP85/tq4KP47/E1wEfzgiH+810EjvPf41bgIbxgiP98u8Ax/ns8A3gwLxjiP99nA5/Ff4/PAT6bFwzxX+OrgbcGHsR/jWcA3w18Ni8c4n+e3wJem+f1O8Br8x8L8T/LSwN/xfP3PsB38x8L8T/LdwPvxfN6BvBg/uMh/uc4Dlzk+fsc4LP5j4f4n+Ozgc/i+TsB7PIfD/E/x9OBB/O8vgd4b/5zIP5neG/gu3j+Xgb4a/5zIP5n+C3gtXlevwO8Nv95EP/9Xhr4K56/9wG+m/88iP9+3w28F8/rGcCD+c+F+O91HLjI8/c5wGfznwvx3+uzgc/i+TsB7PKfC/Hf6+nAg3le3wO8N//5EP993hv4Lp6/lwH+mv98iP8+vwW8Ns/rd4DX5r8G4r/HSwN/xfP3PsB3818D8d/ju4H34nk9A3gw/3UQ//WOAxd5/j4H+Gz+6yD+63028Fk8fyeAXf7rIP7rPR14MM/re4D35r8W4r/WewPfxfP3MsBf818L8V/rt4DX5nn9DvDa/NdD/Nd5aeCveP7eB/hu/ush/ut8N/BePK9nAA/mvwfiv8Zx4CLP3+cAn81/D8R/jc8GPovn7wSwy38PxH+NpwMP5nl9D/De/PdB/Od7b+C7eP5eBvhr/vsg/vP9FvDaPK/fAV6b/16I/1wvDfwVz9/7AN/Nfy/Ef67vBt6L5/UM4MH890P85zkOXOT5+xzgs/nvh/jP89nAZ/H8nQB2+e+H+M/zdODBPK/vAd6b/xkQ/zneG/gunr+XAf6a/xkQ/zl+C3htntfvAK/N/xyIf9lx4LOAtwYezH+NW4HvBj6H/1yIf9lXAx/Ff4+vAT6a/zyIf9lF4Dj/PW4FHsJ/HsS/bBc4xn+PZwAP5j8P4l/22cBn8d/jc4DP5j8P4kXz1cBbAw/iv8YzgO8GPpv/XIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I3ahZEGbDdSJAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoItalic;
impl IconShape for GoItalic {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6 2.75A.75.75 0 016.75 2h6.5a.75.75 0 010 1.5h-2.505l-3.858 9H9.25a.75.75 0 010 1.5h-6.5a.75.75 0 010-1.5h2.505l3.858-9H6.75A.75.75 0 016 2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFmUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/PA8GXgt4aeClgQcDD+Y53QrcCvw18NfA7wC38l8H8R/rwcB7Ae8NPJh/m1uBrwZ+BriV/1yI/xgvDXwU8N78x/pu4HOAW/nPgfj3OQ58FfDe/Of6buBjgF3+YyH+7d4b+CrgOP81doH3AX6a/ziIf5vvAt6b/x5fDXwM/zEQ/zrHgZ8CXpsXzTOA3wZ+GtgFdoG/5oqXBo4Dx4G3Bl4beBAvmt8G3gbY5d8H8aI7DvwW8NL8y74G+G7gr/nXeWngvYGP4l/218DrALv82yFedL8FvDYv3PcAnw3cyr/Pg4HPBt6LF+63gdfh3w7xovku4L15wZ4BvDXw1/zHemngp4EH8YJ9DfDR/Nsg/mXvDXwXL9jfAK8N7PKf4zjw28BL8YK9DfDT/OshXrjjwNOB4zx/3wO8N/81vht4L56/XeAhwC7/OogX7ruB9+L5+xvgpfmv9dfAS/H8fQ/w3vzrIF6wlwb+iufvGcBLA7v81zoO/DXwIJ6/hwC38qJDvGDfDbwXz9/LAH/Nf4+XBv6K5+97gPfmRYd4/h4MPJ3n73uA9+a/13cD78Xz9xDgVl40iOfvs4HP4vl7CHAr/70eDDyd5+9jgK/mRYN4/p4OPJjn9TXAR/M/w1cDH8XzuhV4CC8axPN6MPB0nr+XAf6a/xleGvgrnr+HALfyL0M8r/cGvovn9QzgwfzPcivwIJ7X+wDfzb8M8by+Gvgontf3AO/N/yzfDbwXz+trgI/mX4Z4Xr8NvBbP622An+Z/lrcGforn9TvAa/MvQzyvpwMP5nm9DvDb/M/y2sBv8bxuBR7CvwzxvMzz9zLAX/M/y0sDf8XzJ/5liOdlnj/xP5N5/sS/DPG8zPMn/mfaBY7xnJ4BPJh/GeJ5mefvZYC/5n+ezwY+i+f0OcBn8y9DPK9bgQfxvF4H+G3+Z/pq4K254ruBz+ZFg3hevw28Fs/rbYCf5v8WxPP6auCjeF7fA7w3/7cgntd7A9/F87oVeAj/tyCe14OBp/P8vQzw1/zfgXj+bgUexPP6GuCj+b8D8fx9NvBZPH8PAW7l/wbE8/dg4Ok8f98DvDf/c7w1cAz4Hv71EC/YdwPvxfP3MsBf8z+DueK7gffhXwfxgr008Fc8f7cCLwPs8t/PPNt3A+/Diw7xwn038F48f38NvAz//cxz+m7gfXjRIF6448CtwDGev+8G3of/XuZ5fTfwPvzLEP+ytwZ+ihfsr4HXAXb572Gev+8G3ocXDvGi+Wrgo3jBbgXeBvhr/uuZF+y7gffhBUO86H4beC1euO8GPge4lf865oX7buB9eP4QL7rjwG8DL8W/7KuB7wH+mv985l/23cD78LwQ/zrHgZ8GXosXza3AbwM/DewCl4C/5j+WedF8N/A+PCfEv81XAx/F/z7fDbwPz4b4t3tr4LuBY/zv8t3A+3AF4t/nOPDVwHvxv8v7AN8NIP5jPBj4bOC9+N/hbYCfBhD/sR4MvDXw0cCD+J/pfYDv5grEf54HA68NvDTw0sCDgQfx3+t9gO/m2RD/+5kXzfsA381zQvzvZ/5l7wN8N88L8b+feeHeB/hunj/E/37mBXsf4Lt5wRD/+5nn732A7+aFQ/zvZ57X+wDfzb8M8b+feU7vA3w3LxrE/37m2d4H+G5edIj//cwV7wN8N/86iP/93porfpp/Pf4Rih3QQay8PfkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoIterations;
impl IconShape for GoIterations {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 7.25a4.75 4.75 0 019.5 0 .75.75 0 001.5 0 6.25 6.25 0 10-6.25 6.25H12v2.146c0 .223.27.335.427.177l2.896-2.896a.25.25 0 000-.354l-2.896-2.896a.25.25 0 00-.427.177V12H7.25A4.75 4.75 0 012.5 7.25z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC5UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnhOHwW8N/DSXPHXwFcD38N/rI8C3ht4aa74a+Crge/hP9ZHAe8NvDRX/DXw1cD3cAXiiuPAbwEvzfP318DrALv8+xwHfgt4aZ6/vwZeB9jl3+c48FvAS/P8/TXwOsCuuOKvgJfmhftr4GX49/kr4KV54f4aeBn+ff4KeGleuL8GXkbARwNfxYvmfYDv5t/mo4Gv4kXzPsB382/z0cBX8aJ5HwF/Bbw0L5q/Bl6Gf5u/Al6aF81fAy/Dv81fAS/Ni+avBZh/HfFvY/51xL+NedEhwPzriH8b868j/m3Miw4Bfw28FC+avwFemn+bvwZeihfN3wAvzb/NXwMvxYvmbwR8NPBVvGjeB/hu/m0+GvgqXjTvA3w3/zYfDXwVL5r3EVf8NfBSvHB/A7w0/z5/DbwUL9zfAC/Nv89fAy/FC/c3wEuLK44Dvw28FM/f3wCvDezy73Mc+G3gpXj+/gZ4bWCXf5/jwG8DL8Xz9zfAawO74jm9N/DRwEtxxd8AXw18N/+x3hv4aOCluOJvgK8Gvpv/WO8NfDTwUlzxN8BXA9/NFYj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/Alz0TJ7OTTu5AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKebabHorizontal;
impl IconShape for GoKebabHorizontal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKey;
impl IconShape for GoKey {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.5 5.5a4 4 0 112.731 3.795.75.75 0 00-.768.18L7.44 10.5H6.25a.75.75 0 00-.75.75v1.19l-.06.06H4.25a.75.75 0 00-.75.75v1.19l-.06.06H1.75a.25.25 0 01-.25-.25v-1.69l5.024-5.023a.75.75 0 00.181-.768A3.995 3.995 0 016.5 5.5zm4-5.5a5.5 5.5 0 00-5.348 6.788L.22 11.72a.75.75 0 00-.22.53v2C0 15.216.784 16 1.75 16h2a.75.75 0 00.53-.22l.5-.5a.75.75 0 00.22-.53V14h.75a.75.75 0 00.53-.22l.5-.5a.75.75 0 00.22-.53V12h.75a.75.75 0 00.53-.22l.932-.932A5.5 5.5 0 1010.5 0zm.5 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjBjgPvBbw28NLAg/nf5a+BW4GfBn4G2OV5IZ6/zwI+GjjO/w27wFcDXwPs8myI5/TSwHcBL83/TX8NvA6wyxWIZ3tp4LeA4/zftgs8BNgFEFccB/4KeDD/P/w18DrArrjiu4H34v+XzwE+W8CDgafzgv0M8NXAb/O/y2sDnw28Fs/fLvAQAV8NfBTP3/sA383/bp8NfBbP3/sIeDrwYJ7XzwBvzf8Nvw28Fs/rZwSY5+91gN/m/4bXBn6L5/XXAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+M9zHPgs4K254ruBz+E/l3leCDDPn/jP89XAR/Gcvgb4aP7zmOeFAPP8if88F4HjPKdbgYfwn8c8LwSY50/85zHPn/jPY54XAszzJ/7zmOdP/OcxzwsB5vkT/3nM8yf+85jnhQDz/Il/vZcG3gq4FfgZYJfnzzx/4vk7DrwV8GDgZ4C/5l/PPC8EmOdP/Ot8NPBVPNsu8NHA9/C8zPMnntd7AV8NHOfZ3gf4bv51zPNCgHn+xIvuOHCR5++3gc8BfptnM8+feLbXBj4LeG2evxPALi8687wQYJ4/8aJ7beC3eOG+G/gc4FbAPH8CHgx8FvDevHCvA/w2LzrzvBBgnj/xojsO3Aoc44XbBb4a+Gyev88GPho4zgt3CXgwsMuLzjwvBJjnT/zrvDfwXfzXeB/gu/nXMc8LAeb5E/96rw18NfBS/Of4G+Cjgd/mX888LwSY50/827038NXAMf5jXAI+Gvhu/u3M80KAef7Ev89x4LOBj+Lf52uAzwZ2+fcxzwsB5vkT/zEeDHw38Fr86/wO8N7ArfzHMM8LAeb5E/8xHgx8F/Da/Ov8NvA+wK38xzDPCwHm+RP/PseBzwI+mn+frwY+B9jl38c8LwSY50/8270X8NXAcf5j7AIfDXwP/3bmeSHAPH/iX++1ga8CXpr/HH8NfAzw2/zrmeeFAPP8iX+d9wa+i/8a7wN8N/865nkhwDx/4kV3HLjIv+wS8NXAZ/H8fQ7w0cAxXrhd4AT/OuZ5IcA8f+JF99rAb/HCfQ/w2cCtgHn+BDwY+GzgvXjhHgLcyovOPC8EmOdPvOiOAxd5/n4H+Gzgt3k28/yJZ3tt4LOB1+J5PQN4MP865nkhwDx/4l/no4Gv4tmeAXw28N08L/P8ief13sBnAw/i2d4H+G7+dczzQoB5/sS/3ksDbw3cCvw0sMvzZ54/8fwdB94aOA78NHAr/3rmeSHAPH/iP495/sR/HvO8EGCeP/Gfxzx/4j+PeV4IMM+f+M9jnj/xn8c8LwSY50/859kFjvGcngE8mP885nkhwDx/4j/PVwMfxXP6HOCz+c9jnhcCzPMn/vMcBz4beG+u+Grgs/nPZZ4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4I+GvgpXherwP8Nv83vDXwUzyvvxHw08Bb8bx+Gngb/m/4LeC1eV4/I+C9ge/i+fto4Gv43+2zgM/m+XsfAceBW4FjPH+/DXw28Dv87/JawGcDr83zdwl4sLjis4HP4v+XzwE+W1xxHPht4KX4/+FvgNcGdsWzHQduBY7xf9sl4MHALoB4TseB3wZeiv+b/gZ4bWCXKxDP6zjw0cBHA8f4v+ES8NXAVwO7PBviBTsOvDXw1sCDgZfif5e/AW4Ffhr4aWCX58U/Ajw8CQMzf0WkAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoKeyAsterisk;
impl IconShape for GoKeyAsterisk {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 2.75A2.75 2.75 0 012.75 0h10.5A2.75 2.75 0 0116 2.75v10.5A2.75 2.75 0 0113.25 16H2.75A2.75 2.75 0 010 13.25V2.75zM2.75 1.5c-.69 0-1.25.56-1.25 1.25v10.5c0 .69.56 1.25 1.25 1.25h10.5c.69 0 1.25-.56 1.25-1.25V2.75c0-.69-.56-1.25-1.25-1.25H2.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M8 4a.75.75 0 01.75.75V6.7l1.69-.975a.75.75 0 01.75 1.3L9.5 8l1.69.976a.75.75 0 01-.75 1.298L8.75 9.3v1.951a.75.75 0 01-1.5 0V9.299l-1.69.976a.75.75 0 01-.75-1.3L6.5 8l-1.69-.975a.75.75 0 01.75-1.3l1.69.976V4.75A.75.75 0 018 4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLaw;
impl IconShape for GoLaw {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF8klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv8WDgtYCXBl4aeDDwYJ7TrcCtwF8Dfw38DnAr/7kQ/3mOA+8FvDfw0vzb/DXw3cD3ALv8x0P8xzsOfBXw1sBx/mPsAj8NfAywy38cxH+sjwI+GzjOf45d4LOBr+E/BuI/xnHgp4DX5r/GbwNvA+zy74P493tp4KeAB/Nf61bgbYC/5t8O8e/z0sBvAcf5lz0D+G3gp4Fd4FbgVq54MPBg4Djw1sBrAw/iX7YLvA7w1/zbIP7tHgz8FXCcF+53gM8Gfpt/ndcGPht4LV64XeBlgFv510P82/0V8NK8YH8DfDTw2/z7vDbw1cBL8YL9NfAy/Osh/m2+GvgoXrDvAd6b/1jfDbwXL9jnAJ/Nvw7iX++lgb/iBXsf4Lv5z/HewHfxgr0M8Ne86BD/er8FvDbP39cAH81/rq8GPorn77eB1+FFh/jXeWngr3j+fgZ4a/5r/DTwVjx/LwP8NS8axL/OdwPvxfP3EOBW/ms8GHg6z9/3AO/Niwbxr3MROM7z+hzgs/mv9dnAZ/G8doETvGgQL7rXBn6L5+8EsMt/rePARZ6/1wF+m38Z4kX32cBn8bx+B3ht/nv8NfBSPK/PAT6bfxniRffbwGvxvD4G+Gr+e3w08FU8r98BXpt/GeJF91fAS/O8Xgf4bf57vDbwWzyvvwZehn8Z4kVnnr+XAf6a/x4vDfwVz5/4lyFedOb5E/+9zPMn/mWIF515/sR/L/P8iX8Z4kVnnr+XAf6a/x6vDfwWz5/4lyFedH8NvBTP63WA3+a/x2sDv8Xz+h3gtfmXIV503w28F8/ra4CP5r/HdwPvxfP6HuC9+ZchXnQfDXwVz2sXOMF/j4vAcZ7XxwBfzb8M8aJ7MPB0nr/3Ab6b/1rvDXwXz99DgFv5lyH+dX4beC2e1y7wEGCX/xrHgacDx3levwO8Ni8axL/OewPfxfP33cD78F/ju4D35vl7H+C7edEg/vVuBR7E8/c+wHfzn+u9ge/i+XsG8GBedIh/vdcGfovnbxd4HeCv+c/x0sBf8YK9DvDbvOgQ/zZfDXwUz98u8BBgl/9YDwb+CjjO8/c1wEfzr4P4tzkO/DbwUjx/fw28DrDLf4zjwG8BL83z9zfAawO7/Osg/u1eGvht4BjP33cD78N/jO8C3pvn7xLw2sBf86+H+Pd5a+CneMHeB/hu/n3eG/guXrC3AX6afxvEv99HA1/F87cLPATY5d/mwcBfAcd5/j4G+Gr+7RD/Mb4beC+ev+8B3pt/m+8G3ovn73uA9+bfB/Ef4zjw28BL8fw9BLiVf50HA0/n+fsb4LWBXf59EP9xXhr4K56/jwG+mn+djwa+iufvZYC/5t8P8R/ru4H34nn9DvDa/Ov8NvBaPK/vAd6b/xiI/1ivDfwWz2sXOMG/zkXgOM/rdYDf5j8G4j/eLnCM5/QM4MH86+wCx3hOl4Dj/MdB/Mf7bOCzeE6fA3w2/zqfDXwWz+lrgI/mPw7iP8dHA28NHAe+Gvhu/m2+Gnhvrvhq4LP5j4X4/w3x/xvi/zfE/2+I/3jHgc8C3hp4MP8xbgW+G/gc/mMh/uN9NfBR/Of4GuCj+Y+D+I93ETjOf45bgYfwHwfxH28XOMZ/jmcAD+Y/DuI/3mcDn8V/js8BPpv/OIj/HF8NvDXwIP5jPAP4buCz+Y+F+P8N8f8b4v83xP9viP98x4HPAt4aeDAvmluB7wY+h/9ciP98Xw18FP82XwN8NP95EP/5LgLH+be5FXgI/3kQ//l2gWP82zwDeDD/eRD/+T4b+Cz+bT4H+Gz+8yD+a3w18NbAg3jRPAP4buCz+c/FPwIgrdVBWKSk3wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLightBulb;
impl IconShape for GoLightBulb {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5c-2.363 0-4 1.69-4 3.75 0 .984.424 1.625.984 2.304l.214.253c.223.264.47.556.673.848.284.411.537.896.621 1.49a.75.75 0 01-1.484.211c-.04-.282-.163-.547-.37-.847a8.695 8.695 0 00-.542-.68c-.084-.1-.173-.205-.268-.32C3.201 7.75 2.5 6.766 2.5 5.25 2.5 2.31 4.863 0 8 0s5.5 2.31 5.5 5.25c0 1.516-.701 2.5-1.328 3.259-.095.115-.184.22-.268.319-.207.245-.383.453-.541.681-.208.3-.33.565-.37.847a.75.75 0 01-1.485-.212c.084-.593.337-1.078.621-1.489.203-.292.45-.584.673-.848.075-.088.147-.173.213-.253.561-.679.985-1.32.985-2.304 0-2.06-1.637-3.75-4-3.75zM6 15.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75zM5.75 12a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLink;
impl IconShape for GoLink {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/Hd4a+Cn+/f4aeBmeDfG/w1sDP8V/DPFsiP893hv4Lv79xLMh/nd5b+C7+PcRz4b41zkOfBTw1sBL8293K/DdwOfwr/fewHfxbyeeDfGie2/gq4Dj/Mf5GuCj+dd7b+C7+LcRz4Z40bw38F38x7sVeAj/Nu8NfBf/euLZEP+y48DTgeP8x3sG8GD+7d4b+C7+dcSzIf5lnw18Fv85Pgf4bP7tvgt4b/51xLMh/mV/Bbw0/7GeAXw38Nn8230X8N7864lnQ/zLzPP3MsBf89/ju4D35t9GPBviX2aeP/Hf47uA9+bfTjwb4l9mnj/xX++7gPfmBfse4LeB7+IFE8+G+JeZ50/81/ou4L15wb4HeG+ueG/gu3j+xLMh/mXm+RP/db4LeG9esO8B3pvn9N7Ad/G8xLMh/mXm+RP/Nb4LeG9esO8B3pvn772B7+I5iWdD/MvM8yf+830X8N68YN8DvDcv3HsD38WziWdD/MvM8yf+c30X8N68YN8DvDcvmvcGvosrxLMh/mXm+RP/eb4LeG9esO8B3pt/nbfmip/m2RD/MvP8if8c3wW8Ny/Y9wDvzX8MxL/MPH/iP953Ae/NC/Y9wHvzHwfxLzPPn/iP9V3Ae/OCfQ/w3vzHQvzLzPMn/uN8F/DevGDfA7w3//EQ/zLz/In/GJ8NfBYv2PcA781/DsS/zDx/4j/GXwEvzfP3PcB7858H8S8zz5/4j/HXwEvxvL4HeG/+cyH+Zeb5E/8xPhr4Kp7T9wDvzX8+xL/MPH/iP85HA+/NFT8NfDb/NRD/MvP8if9ZjgOfBbw1V3w38Dm8cIh/mXn+xP8sXw18FM/pa4CP5gVD/MvM8yf+Z7kIHOc53Qo8hBcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8vQzw1/zP8NLAX/H8iRcM8S/7a+Cl+N/pb4CX5gVD/Ms+G/gs/nf6HOCzecEQ/7LjwK3AMf53uQQ8GNjlBUO8aN4b+C7+d3kb4Kd54RAvuvcGvho4xv9sl4D3Bn6afxniX+c48NHAWwMvxf8sfwP8NPDVwC4vGsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EZ9ujUFhnVW0AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLinkExternal;
impl IconShape for GoLinkExternal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.604 1h4.146a.25.25 0 01.25.25v4.146a.25.25 0 01-.427.177L13.03 4.03 9.28 7.78a.75.75 0 01-1.06-1.06l3.75-3.75-1.543-1.543A.25.25 0 0110.604 1zM3.75 2A1.75 1.75 0 002 3.75v8.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 12.25v-3.5a.75.75 0 00-1.5 0v3.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-8.5a.25.25 0 01.25-.25h3.5a.75.75 0 000-1.5h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD60lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4TseBjwI+miveBvht/u9CXHEc+Cjgo4HjPNutwEP4vwsBLw38FnCc50/86xwHPgt4a+DB/M9wK/DdwOfwnBDwdODBvGDiX+ergY/if6avAT6aZ0PArcCDeMHEv85F4Dj/M90KPIRnQ8BLAz8NHAe+GvgsnpP419kFjvE/0zOAB/NsiOdlnpP41/ls4LP4n+lzgM/m2RDPyzwn8a/31cBbAw/if4ZnAN8NfDbPCfG8zHMS/3chnpd5TuL/LsTzMs9J/N+FeF7mOYn/uxDPyzwn8X8X4nmZ5yT+70I8L/OcxP9diOdlnpP4vwsBrw18F/BgXrhd4H2An+b/DgQ8HXgwL5pd4AT/dyBgFzjGi+YScJz/OxDw2sB3Aw/ihbsEvDXw2/zfgfj/DfH/G+I/3nHgs4C3Bh7M/wy3At8NfA7PCfEf76uBj+J/pq8BPppnQ/zHuwgc53+mW4GH8GyI/3i7wDH+Z3oG8GCeDfEf77OBz+J/ps8BPptnQ/zn+GrgrYEH8T/DM4DvBj6b54R4tuPAewHvDbw0V/w18N3A9wC7/N+DuOKlgd8CjvP83Qq8DfDX/N+CgJcGfgs4zgt3K/AywC7/dyDg6cCDedF8DvDZ/N+BgF3gGM/2OcBXc8VXA+/Fs/018DL834GA9wa+iyveB/hunu04cJHnJP7vQFzxYGAX2OU5HQcu8pzE/x2IF+69ge/i2Z4BPJj/OxAv2HHgr4AH82zfA7w3/3cgnr+XBn4KeDDP6SHArfzfgXhe7w18FXCc5/Q1wEfzfwviOX0X8N48r+8B3pv/exDP9l3Ae/O8vgd4b/5vQlzx2cBn8ZwuAR8NfDf/dyHgwcDTeU7PAN4a+Gv+9Y4DnwW8NfBg/me4Ffhu4HN4Tgj4auCjeLZLwEsDt/Jv89XAR/E/09cAH82zIeDpwIP5l+0CbwP8Ni/cReA4/zPdCjyEZ0OAedHtAid44XaBY/zP9AzgwTwbAnaBY7xoLgHHeeE+G/gs/mf6HOCzeTYEvDbw3cCDeOGeAbw38Nv8y74aeGvgQfzP8Azgu4HP5jkh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9sQ3xBg6DdmwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoListOrdered;
impl IconShape for GoListOrdered {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.003 2.5a.5.5 0 00-.723-.447l-1.003.5a.5.5 0 00.446.895l.28-.14V6H.5a.5.5 0 000 1h2.006a.5.5 0 100-1h-.503V2.5zM5 3.25a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 015 3.25zm0 5a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5A.75.75 0 015 8.25zm0 5a.75.75 0 01.75-.75h8.5a.75.75 0 010 1.5h-8.5a.75.75 0 01-.75-.75zM.924 10.32l.003-.004a.851.851 0 01.144-.153A.66.66 0 011.5 10c.195 0 .306.068.374.146a.57.57 0 01.128.376c0 .453-.269.682-.8 1.078l-.035.025C.692 11.98 0 12.495 0 13.5a.5.5 0 00.5.5h2.003a.5.5 0 000-1H1.146c.132-.197.351-.372.654-.597l.047-.035c.47-.35 1.156-.858 1.156-1.845 0-.365-.118-.744-.377-1.038-.268-.303-.658-.484-1.126-.484-.48 0-.84.202-1.068.392a1.858 1.858 0 00-.348.384l-.007.011-.002.004-.001.002-.001.001a.5.5 0 00.851.525zM.5 10.055l-.427-.26.427.26z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/h4MfBXw0lzx18DHALfyfwvieT0Y+CvgOM9pF3gZ4Fb+70A8r58G3orn72eAt+aFOw58FvDWwIP5n+FW4LuBz+E5IZ7XReA4z9+twEN44b4a+Cj+Z/oa4KN5NsTz2gWO8fw9A3gwL9xF4Dj/M90KPIRnQzyvnwbeiufvZ4C35oXbBY7xP9MzgAfzbIjn9WDgr4FjPKdLwEsDt/LCfTbwWfzP9DnAZ/NsiOfvwcBXAy/NFX8NfDRwKy+arwbeGngQ/zM8A/hu4LN5Toj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP4eDHwV8NJc8dfAxwC38n8L4nk9GPgr4DjPaRd4GeBW/u9APK+fBt6K5+9ngLfmhTsOfBbw1sCD+Z/hVuC7gc/hOSGe10XgOM/frcBDeOG+Gvgo/mf6GuCjeTbE89oFjvH8PQN4MC/cReA4/zPdCjyEZ0M8r58G3orn72eAt+aF2wWO8T/TM4AH82yI5/Vg4K+BYzynS8BLA7fywn028Fn8z/Q5wGfzbIjn78HAVwMvzRV/DXw0cCsvmq8G3hp4EP8zPAP4buCzeU6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnj+Hgx8FfDSXPHXwMcAt/J/C+J5PRj4K+A4z2kXeBngVv7vQDyvnwbeiufvZ4C35oU7DnwW8NbAg/mf4Vbgu4HP4TkhntdF4DjP363AQ3jhvhr4KP5n+hrgo3k2xPPaBY7x/D0DeDAv3EXgOP8z3Qo8hGdDPK+fBt6K5+9ngLfmhdsFjvE/0zOAB/NsiOf1YOCvgWM8p0vASwO38sJ9NvBZ/M/0OcBn82yI5+/BwFcDL80Vfw18NHArL5qvBt4aeBD/MzwD+G7gs3lOiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjnX9mQQXUec0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoListUnordered;
impl IconShape for GoListUnordered {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLocation;
impl IconShape for GoLocation {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.536 3.464a5 5 0 010 7.072L8 14.07l-3.536-3.535a5 5 0 117.072-7.072v.001zm1.06 8.132a6.5 6.5 0 10-9.192 0l3.535 3.536a1.5 1.5 0 002.122 0l3.535-3.536zM8 9a2 2 0 100-4 2 2 0 000 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv9VbAWwMPBo4DL80Vfw3sArcCPw38DP81EP/5Hgx8FvDWwHFeNLvATwMfA+zynwfxn+uzgI8GjvNvswt8NfA5/OdA/Oc4DvwW8NL8x/hr4HWAXf5jIf7jvTTwW8Bx/mPtAq8D/DX/cRD/sY4DTweO859jF3gIsMt/DMR/nOPAbwEvzQv3PcBvA38N/DVXvDTw0sBbA2/FC/fXwMvwHwPxH+ezgc/iBfsZ4KOBW3nhHgx8NfBWvGCfA3w2/36I/xgPBv4KOM7z9zHAV/Ov89HAV/H87QIPAXb590H8x/hu4L14/j4G+Gr+bT4a+Cqev+8B3pt/H8R/jIvAcZ7XzwBvzb/PTwNvxfPaBU7w74P493tr4Kd4/h4C3Mq/z4OBp/P8vQ3w0/zbIf79vht4L57X9wDvzX+Mnwbeiuf1NcBH82+H+Pf7beC1eF7vA3w3/zHeG/guntfvAK/Nvx3i3++vgJfmeb0M8Nf8x3hp4K94Xn8NvAz/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/nePARwFvDbw0/7P8NfDTwNcAu7xoEC+69wa+CjjO/2y7wMcA382/DPGieW/gu/jf5X2A7+aFQ/zLjgNPB47zv8su8BBglxcM8S/7bOCz+N/pc4DP5gVD/Mv+Cnhp/nf6a+BleMEQ/zLz/L0M8Nf8z/DSwF/x/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/L0M8Nf8z/DSwF/x/IkXDPEv+2vgpfjf6W+Al+YFQ/zLPhv4LP53+hzgs3nBEP+y48CtwDH+d7kEPBjY5QVDvGjeG/gu/nd5G+CneeEQL7r3Br4aOMb/bJeA9wZ+mn8Z4l/nOPDRwFsDL8X/LH8D/DTw1cAuLxrE/2+I/98Q/78h/n/jHwG0lKFBQ89PCAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLock;
impl IconShape for GoLock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4 4v2h-.25A1.75 1.75 0 002 7.75v5.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-5.5A1.75 1.75 0 0012.25 6H12V4a4 4 0 10-8 0zm6.5 2V4a2.5 2.5 0 00-5 0v2h5zM12 7.5h.25a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-5.5a.25.25 0 01.25-.25H12z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAH8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjhXht4L+CtgeP877IL/DTwPcBv8/whnr+XBr4KeG3+b/ht4GOAv+Y5IZ7XRwFfzf9NHw18Dc+GeE7vDXwX/7d9DPDVXIF4tvcGvov/H94H+G4AccWDgb8CjvP/wy7wMsCt4orvBt6L5+8S8N3AdwN/zX+P7wbei+fvEvDdwHcDf80VLw28N/DewDGev+8B3lvAg4Gn8/xdAl4b+Gv++zwYeDrP3yXgtYG/5vl7aeCngQfx/D1EwEcDX8Xz9zLAX/Pf66OBr+L5exngr3nhXhr4K56/jxHw08Bb8by+Bvho/vv9NvBaPK+vAT6aF81XAx/F8/oZAU8HHszzehngr/nvdxE4zvN6GeCvedG8NPBXPK9bBZjnT/zPYJ4/8a9jnhcCzPN3Atjlv595/sS/jnleCLgVeBDP622An+a/363Ag3hebwP8NC+aBwNP53k9Q8BPA2/F8/pt4HX47/fTwFvxvH4aeBteNG8N/BTP63cEfDTwVTx/Xw18D/DX/Pf5aOCreP4+G/gc/mWfDXwWz+t7BBwHbgWO8b/TdwPvwwv3V8BL87zeR1zx2cBn8b/X1wAfzfP30sBf8fy9jHi2vwZeiv+ddoETPH9fDXwUz+sZwIPFsx0Hfht4Kf73uQQc53kdB54OHOd5fQzw1eI5HQe+Gngv/nf5HOCzeV6fDXwWz99DgFvF8/dg4L2B1wZeGjjG/0yXgK8GPpvndRx4OnCc5/U9wHsDiH+7rwY+iufvdYDf5r/XZwOfxfP3OsBvA4h/m5cG/orn73eA1+a/14OBp/P8/Q7w2lyB+Lf5LeC1ef5eBvhr/nv9FvDaPH8vA/w1VyD+9d4b+C6ev+8B3pv/Xm8N/BTP3/cA782zIf51jgNPB47zvC4BDwZ2+e9zHHg6cJzndQl4aeBWng3xr/NTwFvz/H0O8Nn89/pu4L14/j4H+GyeE+JF99bAT/H8/Q3w0vz3em3gt3j+/gZ4aZ4X4kVzHPgr4ME8f68D/Db/fY4DfwU8mOfvdYDf5nkhXjRfDXwUz9/XAB/Nsx0HPgt4a+DB/PvcCnw38Dm8cF8NfBTP39cAH83zh/iXvTbwWzx/zwBeGtjl2b4a+Cj+Y30N8NE8f68N/BbP3zOAlwZ2ef4QL9xx4K+AB/P8vQ7w2zyni8Bx/mPdCjyE53Uc+CvgwTx/rwP8Ni8Y4oX7auCjeP6+BvhontcucIz/WM8AHszz+mrgo3j+vgb4aF44xAv22sBv8fw9A3hpYJfn9dnAZ/Ef63OAz+Y5vTbwWzx/zwBeGtjlhUO8YE8HHszz9zrAb/OCfTXw1sCD+Pd5BvDdwGfzvP4KeGmev9cBfpt/GeL5+2zgs3j+vgd4b/57fTTwVTx/3wO8Ny8axPN6MPBXwHGe1yXgwcAu/32OA08HjvO8LgEPBnZ50SCe13cD78Xz9z7Ad/Pf66uBj+L5exvgp3nRIZ7TawO/xfP3O8Br89/rwcDTef5+B3ht/nUQz+m3gNfm+XsIcCv/vX4LeG2ev4cAt/Kvg3i21wZ+i+fvc4DP5r/XawO/xfP3OcBn87yOA2/FFT8D7PKcEM/208Bb8bwuAQ8Gdnm248BnAW8NPJh/n1uB7wY+hxfuu4H34nldAh4M7PKcXhr4LeA4V+wCbwP8Ns+GuOLBwNN5/j4H+Gye01cDH8V/rK8BPprn78HA03n+Pgf4bJ7XTwNvxfN6H+C7uQJxxWcDn8XzugQ8GNjlOV0EjvMf61bgITx/Hw18Fc/rEvBgYJfndRE4zvP3NsBPA4grng48mOf1NcBH87x2gWP8x3oG8GCev6cDD+Z5fQ3w0Tx/vw28Fs/fLvAQYFfAg4Gn8/y9DPDXPK/PBj6L/1ifA3w2z+ulgb/i+XsIcCvP30sDvw0c4/n7HuC9BXw08FU8r2cAD+YF+2rgrYEH8e/zDOC7gc/m+Xtv4Lt4Xn8DvDQv3EsDvw0c4/k7IeC7gffieX0N8NH89/tu4L14Xp8DfDb/spcG/orn730E/DbwWjyvtwF+mv9+vw28Fs/rdYDf5kXz3cB78bx+R4B5/l4H+G3++z0deDDP6yHArbxoXhv4LZ7X7wgwz5/4n8E8f+JfxzwvBJjnT/zPYJ6/E8AuLzrzvBDw18BL8bxeBvhr/vvtAsd4Xq8D/DYvmpcG/orn9QwBPw28Fc/ra4CP5r/fbwOvxfP6HuC9edF8N/BePK+fEfDRwFfx/L0M8Nf89/po4Kt4/l4G+GteuJcG/orn72MEPBh4Os/fLvA6wF/z3+fBwNN5/naB1wH+mufvpYHfAo7z/D1EXPHdwHvxgn038DXAX/Pf47uB9+IF+27ge4DfBo4DLwW8N/DevGDfA7y3uOLBwF8Dx/j/4RLwYGBXPNtbAz/F/w/vA3w3gHhO7w18F/+3vQ/w3VyBeF5vDXw3cIz/Wy4BHw18N8+GeP4eDHw28F783/A9wEcDuzwnxAv3YOCtgdcGHgy8FP87PAP4a+C3gZ8GbuX54x8B0K1MS5dWE2YAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLog;
impl IconShape for GoLog {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5 8.25a.75.75 0 01.75-.75h4a.75.75 0 010 1.5h-4A.75.75 0 015 8.25zM4 10.5A.75.75 0 004 12h4a.75.75 0 000-1.5H4z",
            }
            path {
                d: "M13-.005H3a3 3 0 00-3 3c0 .676.224 1.254.603 1.722.526.65 1.331.783 1.907.783h1.177c-.364.662-.814 1.339-1.287 2.048-.205.309-.414.624-.623.946C.891 9.865 0 11.418 0 13a3 3 0 003 3h10a3 3 0 001.667-5.494.75.75 0 00-.834 1.246A1.5 1.5 0 1111.5 13c0-.642.225-1.347.623-2.136.397-.787.933-1.593 1.501-2.446l.011-.017c.554-.83 1.139-1.709 1.582-2.588.445-.885.783-1.836.783-2.818 0-1.672-1.346-3-3-3zm-10 1.5a1.5 1.5 0 00-1.5 1.5c0 .321.1.569.27.778.097.12.325.227.74.227h7.674A2.737 2.737 0 0110 2.995c0-.546.146-1.059.401-1.5H3zm10 0c.831 0 1.5.662 1.5 1.5 0 .646-.225 1.353-.623 2.143-.398.79-.933 1.595-1.501 2.448l-.017.026c-.552.828-1.134 1.702-1.575 2.576C10.338 11.072 10 12.021 10 13c0 .546.146 1.059.401 1.5H3A1.5 1.5 0 011.5 13c0-1.084.63-2.289 1.537-3.692.177-.274.366-.556.558-.845.632-.948 1.306-1.96 1.773-2.963h6.382a.75.75 0 00.417-1.373c-.444-.298-.667-.656-.667-1.132a1.5 1.5 0 011.5-1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLogoGist;
impl IconShape for GoLogoGist {
    fn view_box(&self) -> &str {
        "0 0 25 16"
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
                d: "M4.7 8.73h2.45v4.02c-.55.27-1.64.34-2.53.34-2.56 0-3.47-2.2-3.47-5.05 0-2.85.91-5.06 3.48-5.06 1.28 0 2.06.23 3.28.73V2.66C7.27 2.33 6.25 2 4.63 2 1.13 2 0 4.69 0 8.03c0 3.34 1.11 6.03 4.63 6.03 1.64 0 2.81-.27 3.59-.64V7.73H4.7v1zm6.39 3.72V6.06h-1.05v6.28c0 1.25.58 1.72 1.72 1.72v-.89c-.48 0-.67-.16-.67-.7v-.02zm.25-8.72c0-.44-.33-.78-.78-.78s-.77.34-.77.78.33.78.77.78.78-.34.78-.78zm4.34 5.69c-1.5-.13-1.78-.48-1.78-1.17 0-.77.33-1.34 1.88-1.34 1.05 0 1.66.16 2.27.36v-.94c-.69-.3-1.52-.39-2.25-.39-2.2 0-2.92 1.2-2.92 2.31 0 1.08.47 1.88 2.73 2.08 1.55.13 1.77.63 1.77 1.34 0 .73-.44 1.42-2.06 1.42-1.11 0-1.86-.19-2.33-.36v.94c.5.2 1.58.39 2.33.39 2.38 0 3.14-1.2 3.14-2.41 0-1.28-.53-2.03-2.75-2.23h-.03zm8.58-2.47v-.86h-2.42v-2.5l-1.08.31v2.11l-1.56.44v.48h1.56v5c0 1.53 1.19 2.13 2.5 2.13.19 0 .52-.02.69-.05v-.89c-.19.03-.41.03-.61.03-.97 0-1.5-.39-1.5-1.34V6.94h2.42v.02-.01z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoLogoGithub;
impl IconShape for GoLogoGithub {
    fn view_box(&self) -> &str {
        "0 0 45 16"
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
                d: "M18.53 12.03h-.02c.009 0 .015.01.024.011h.006l-.01-.01zm.004.011c-.093.001-.327.05-.574.05-.78 0-1.05-.36-1.05-.83V8.13h1.59c.09 0 .16-.08.16-.19v-1.7c0-.09-.08-.17-.16-.17h-1.59V3.96c0-.08-.05-.13-.14-.13h-2.16c-.09 0-.14.05-.14.13v2.17s-1.09.27-1.16.28c-.08.02-.13.09-.13.17v1.36c0 .11.08.19.17.19h1.11v3.28c0 2.44 1.7 2.69 2.86 2.69.53 0 1.17-.17 1.27-.22.06-.02.09-.09.09-.16v-1.5a.177.177 0 00-.146-.18zM42.23 9.84c0-1.81-.73-2.05-1.5-1.97-.6.04-1.08.34-1.08.34v3.52s.49.34 1.22.36c1.03.03 1.36-.34 1.36-2.25zm2.43-.16c0 3.43-1.11 4.41-3.05 4.41-1.64 0-2.52-.83-2.52-.83s-.04.46-.09.52c-.03.06-.08.08-.14.08h-1.48c-.1 0-.19-.08-.19-.17l.02-11.11c0-.09.08-.17.17-.17h2.13c.09 0 .17.08.17.17v3.77s.82-.53 2.02-.53l-.01-.02c1.2 0 2.97.45 2.97 3.88zm-8.72-3.61h-2.1c-.11 0-.17.08-.17.19v5.44s-.55.39-1.3.39-.97-.34-.97-1.09V6.25c0-.09-.08-.17-.17-.17h-2.14c-.09 0-.17.08-.17.17v5.11c0 2.2 1.23 2.75 2.92 2.75 1.39 0 2.52-.77 2.52-.77s.05.39.08.45c.02.05.09.09.16.09h1.34c.11 0 .17-.08.17-.17l.02-7.47c0-.09-.08-.17-.19-.17zm-23.7-.01h-2.13c-.09 0-.17.09-.17.2v7.34c0 .2.13.27.3.27h1.92c.2 0 .25-.09.25-.27V6.23c0-.09-.08-.17-.17-.17zm-1.05-3.38c-.77 0-1.38.61-1.38 1.38 0 .77.61 1.38 1.38 1.38.75 0 1.36-.61 1.36-1.38 0-.77-.61-1.38-1.36-1.38zm16.49-.25h-2.11c-.09 0-.17.08-.17.17v4.09h-3.31V2.6c0-.09-.08-.17-.17-.17h-2.13c-.09 0-.17.08-.17.17v11.11c0 .09.09.17.17.17h2.13c.09 0 .17-.08.17-.17V8.96h3.31l-.02 4.75c0 .09.08.17.17.17h2.13c.09 0 .17-.08.17-.17V2.6c0-.09-.08-.17-.17-.17zM8.81 7.35v5.74c0 .04-.01.11-.06.13 0 0-1.25.89-3.31.89-2.49 0-5.44-.78-5.44-5.92S2.58 1.99 5.1 2c2.18 0 3.06.49 3.2.58.04.05.06.09.06.14L7.94 4.5c0 .09-.09.2-.2.17-.36-.11-.9-.33-2.17-.33-1.47 0-3.05.42-3.05 3.73s1.5 3.7 2.58 3.7c.92 0 1.25-.11 1.25-.11v-2.3H4.88c-.11 0-.19-.08-.19-.17V7.35c0-.09.08-.17.19-.17h3.74c.11 0 .19.08.19.17z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEcElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4TseBjwLeGnhp/m/5a+Cnga8BdrkC8WzvDXwVcJz/23aBjwG+G0Bc8d7Ad/H/y/sA3y3gOPB04Dj/v+wCDxHw2cBn8f/T5wj4K+Cl+f/prwWY5+9lgL/m/4aXBv6K54UA8/yJ/1vM80KAef4+Gvga/m/4KOCreV4IMC/YXwMfA/w2/zu9NvBVwEvz/CHA/Mt+GvgY4Fb+d3gw8FXAW/PCIcC8aHaBrwa+Btjlf6bjwEcBHw0c51+GAPOvcyvwMcBP8z/LWwNfBTyYFx0CzL/NbwPvA9zKf68HA98FvDb/eggw/z5fDXwOsMt/rePAZwEfzb8dAszz9znAZ/Gi2QU+Gvge/mu8F/DVwHFeNJ8DfBbPCwHm+RPwYOC7gdfiRfPXwMcAv81/jtcGvgp4aV40vwO8N3ArYJ4XAszzJ57ttYHvBh7Ei+a7gY8BdvmPcRz4KuC9edE8A3hv4Ld5NvO8EGCeP/G8Phv4aOAY/7Jd4KuBz+Hf57OAjwaO8y+7BHw18Nk8L/O8EGCeP/H8HQe+GngvXjS3Au8D/Db/Oq8NfBfwYF403wN8NLDL82eeFwLM8ydeuNcGvhp4KV40vw28D3ArL9yDge8CXpsXzd8AHw38Ni+ceV4IMM+feNG8N/DVwDFeNJ8NfA2wy3M6DnwU8Nm8aC4BHw18Ny8a87wQYJ4/8aI7Dnw28FG8aG4FPhv4Hq54L+CzgQfzovka4LOBXV505nkhwDx/4l/vwcB3A6/Fi+a3ueK1edH8DvDewK3865nnhQDz/Il/u7cGvhp4EP8xngF8NPDT/NuZ54UA8/yJf5/jwEcDHw0c49/mEvDVwFcDu/z7mOeFAPP8if8YDwa+Gngr/nV+Bvho4Fb+Y5jnhQDz/In/WK8NfDXwUrxwfwN8NPDb/McyzwsB5vkT/zk+Gvhs4BjP6RLw2cBX85/DPC8EmOdP/Oc5Dnw18NZc8dPARwO7/OcxzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8fy8D/DX/N7w08Fc8LwTsAsf4/+lvBHw38F78//Q5Ah4M/DVwjP9fLgEPFle8N/Bd/P/yNsBPi2d7b+CrgWP833YJeG/gpwHEc3ow8N7AWwMvxf8tfwP8NPDVwC5XIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPUZG53P5lly0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMail;
impl IconShape for GoMail {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2A1.75 1.75 0 000 3.75v.736a.75.75 0 000 .027v7.737C0 13.216.784 14 1.75 14h12.5A1.75 1.75 0 0016 12.25v-8.5A1.75 1.75 0 0014.25 2H1.75zM14.5 4.07v-.32a.25.25 0 00-.25-.25H1.75a.25.25 0 00-.25.25v.32L8 7.88l6.5-3.81zm-13 1.74v6.441c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V5.809L8.38 9.397a.75.75 0 01-.76 0L1.5 5.809z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMarkGithub;
impl IconShape for GoMarkGithub {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+IZ/ss4L2BB/N/263AdwOfAyCu+Cngrfn/5aeBtxHw2cBn8f/T5wh4OvBg/n/6awHm/y8EmP+/EGD+/0KA+f8LAeaFEy/cdwPvxYvme4D35oUzL5x44cyLDgHmhRMv3HHgt4GX4oX7G+C1gV1eOPPCiRfOvOgQYF448S97aeC3gWM8f5eA1wb+mn+ZeeHEC2dedAgwL5x40bw38F08f+8DfDcvGvPCiRfOvOgQYF448aL7buC9eE7fA7w3LzrzwokXzrzoEGBeOPGiOw78NvBSXPE3wGsDu7zozAsnXjjzokOAeeHEv85LA7/NFa8N/DX/OuaFEy+cedEhwLxw4l/vvbniu/nXMy+ceOHMiw4B5oUT/7XMCydeOPOiQ4B54cR/LfPCiRfOvOgQYF448V/LvHDihTMvOgSYF0781zIvnHjhzIsOAeaFEy/cewPfzYvmvYHv5oUzV7wN8NP8x3hv4Lt4XggwL5x44Qy8DPDXvHAvDfwVIF44c8Uu8DLArfz7PBj4K+A4zwsB5oUTL5yBW4GXAXZ5/o4DfwU8GBAvnHm23wZeh3+fvwJemucPAeaFEy+cueK3gdfh+fst4LW5Qrxw5jl9DvDZ/Nt8NvBZvGAIMC+ceOHMs30O8Nk8p88GPotnEy+ceV4vA/w1/zqvDfwWLxwCzAsnXjjznN4G+GmueGvgp3hO4oUzz+tW4GWAXV40x4G/Ah7MC4cA88KJF848p13gZYDjwG8Bx3lO4oUzz993A+/Di+angLfmX4YA88KJF848r7/mipfmeYkXzrxgbwP8NC/cewPfxYsGAeaFEy+c+dcRL5x5wXaBlwFu5fl7MPBXwHFeNAgwL5x44cy/jnjhzAv328Dr8Pz9FfDSvOgQYF448cKZfx3xwpl/2ecAn81z+mzgs/jXQYB54cQLZ/51xAtnXjQvA/w1V7w28Fv86yHA/O/018DrcMXTgeP86yHA/O/1NcCDgLfm3wYB5v8vBJj/vxBwK/Ag/n/6GwGfDXwW/z+9j7jip4G34v+XnwHeWjzbZwPvDTyI/9ueAXw38NkA4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPwwafoBjclgUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMarkdown;
impl IconShape for GoMarkdown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M14.85 3H1.15C.52 3 0 3.52 0 4.15v7.69C0 12.48.52 13 1.15 13h13.69c.64 0 1.15-.52 1.15-1.15v-7.7C16 3.52 15.48 3 14.85 3zM9 11H7V8L5.5 9.92 4 8v3H2V5h2l1.5 2L7 5h2v6zm2.99.5L9.5 8H11V5h2v3h1.5l-2.51 3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMegaphone;
impl IconShape for GoMegaphone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
            g {
                fill_rule: "evenodd",
            }
            path {
                d: "M3.25 9a.75.75 0 01.75.75c0 2.142.456 3.828.733 4.653a.121.121 0 00.05.064.207.207 0 00.117.033h1.31c.085 0 .18-.042.258-.152a.448.448 0 00.075-.366A16.74 16.74 0 016 9.75a.75.75 0 011.5 0c0 1.588.25 2.926.494 3.85.293 1.113-.504 2.4-1.783 2.4H4.9c-.686 0-1.35-.41-1.589-1.12A16.42 16.42 0 012.5 9.75.75.75 0 013.25 9z",
            }
            path {
                d: "M0 6a4 4 0 014-4h2.75a.75.75 0 01.75.75v6.5a.75.75 0 01-.75.75H4a4 4 0 01-4-4zm4-2.5a2.5 2.5 0 000 5h2v-5H4z",
            }
            path {
                d: "M15.59.082A.75.75 0 0116 .75v10.5a.75.75 0 01-1.189.608l-.002-.001h.001l-.014-.01a5.829 5.829 0 00-.422-.25 10.58 10.58 0 00-1.469-.64C11.576 10.484 9.536 10 6.75 10a.75.75 0 110-1.5c2.964 0 5.174.516 6.658 1.043.423.151.787.302 1.092.443V2.014c-.305.14-.669.292-1.092.443C11.924 2.984 9.713 3.5 6.75 3.5a.75.75 0 110-1.5c2.786 0 4.826-.484 6.155-.957.665-.236 1.154-.47 1.47-.64a5.82 5.82 0 00.421-.25l.014-.01a.75.75 0 01.78-.061zm-.78.06zm.44 11.108l-.44.607.44-.607z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAI8UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjP92DgtYAHA6/NFa/NC/bXwG8Dvw38DXAr/3kQ/zleG3gr4LWBl+bf56+B3wa+B/hr/mMh/uMcB94L+GjgwfznuBX4auB7gF3+/RD/fseBjwI+GjjOf41d4KuBrwF2+bdD/Pu8F/DVwHH+e+wCHw18D/82iH+bBwPfBbw2/zP8NvA+wK386yD+9d4a+C7gOC+6vwH+Gvhr4K+54rd5Tq/NFS8NvDTw0sBL8aLbBT4G+G5edIh/ne8C3psXzd8A3w38NHAr/zYPBt4aeG/gpXjRfDXwMbxoEC+67wLem3/Z7wCfDfw2/7FeG/hs4LX4l3038D78yxAvmu8C3psX7m+AjwZ+m/9crw18NfBSvHA/DbwNLxziX/ZdwHvzwn0O8Nn81/ps4LN44b4beB9eMMQL99nAZ/GC/Q3w3sBf89/jpYGfBh7EC/Y1wEfz/CFesPcGvosX7G+A1wZ2+e91HPht4KV4wd4H+G6eF+L5ezDwV8Bxnr+fAd4b2OV/huPAdwNvxfO3C7wMcCvPCfH8/Rbw2jx/fwO8NrDL/yzHgd8GXorn77eB1+E5IZ7XWwM/xfP3DOClgV3+ZzoO/DbwUjx/HwN8Nc+GeE7HgacDx3lel4DXBv6af5vjwGsBrw28NPDSwHGe7beBvwZ+G/gZ/u1eGvgrnr9d4CHALlcgntNnA5/F8/c5wGfzr/dg4LOAtwaO86LZBX4a+BzgVv71Phv4LJ6/zwE+mysQz+kicJzn9TfAS/Ovcxz4KOCz+ff5bOBz+Nf7a+CleF67wEOAXQDxbO8NfBfP3+sAv82L7sHATwEvzX+MvwbeBriVF91rA7/F8/cxwFcDiGd7OvBgntfvAK/Ni+6lgd8CjvMfaxd4HeCvedH9NvBaPK9bgYcAiCteGvgrnr/XAX6bF81LA78FHOc/xy7wOsBf86J5beC3eP5eBvhrccVnA5/F8/ob4KV50f0V8NK8cN8D/Dbw18Bfc8VLAy8NvDXwVrxwfw28DC+6W4EH8by+BvhoccVfAS/N8/oY4Kt50Xw28Fm8YD8DfDRwKy/cg4GvBt6KF+xzgM/mRfPRwFfxvP4aeBkBx4GLPH8PAW7lX/Zg4Om8YB8DfDX/Oh8NfBUv2EOAW/mXPRh4Os/fCQGvDfwWz+tvgJfmRfPdwHvx/H0M8NX823w08FU8f18DfDQvmluBB/G83kbAZwOfxfP6HuC9edGY5+9ngLfm3+engbfiee0CDwF2+Zd9N/BePK/PEfDTwFvxvD4G+Gr+ZW8N/BTP30OAW/n3eTDwdJ6/twF+mn/ZRwNfxfP6GQG/DbwWz+t1gN/mX/bVwEfxvL4HeG/+Y/w08FY8r68BPpp/2WsDv8Xz+h0Bvw28Fs/rdYDf5l/228Br8bzeB/hu/mO8N/BdPK/fAV6bf9lrA7/F87pVgHn+xIvm6cCDeV4vA/w1/zFeGvgrntetwEN40ZjnhQDz/IkXjXn+xH8s8/yJF415Xggwz5940ZjnT/zHMs+feNGY54UA8/yJF415/l4G+Gv+Y7w08Fc8f+JFY54XAm4FHsTzehngr/mX/TbwWjyv9wG+m/8Y7w18F8+feNGY54WA3wZei+f1OsBv8y/7auCjeF7fA7w3/zF+Gngrnj/xojHPCwHfDbwXz+tjgK/mX/bWwE/x/D0EuJV/nwcDT+cFEy8a87wQ8NnAZ/G8vgd4b/5lx4GLPH8/DbwN/z4/Bbw1L5h40ZjnhYC3Bn6K5/XXwMvwovlu4L14/j4G+Gr+bT4a+CpeuNcBfpsX7q2Bn+J5PUPAceAiz99DgFv5lz0YeDov2EcDX8O/zkcBX82/7KeBt+GF+y3gtXlePyOu+GvgpXheHwN8NS+azwY+ixfsp4GPAW7lhXsw8FXAW/Oi+2jga3j+Pgv4bJ6/9xFXfDXwUTyvvwZehhfNceC3gZfihftp4KeBvwH+miteGngp4K2Bt+bf5reBzwZ+hyteC/hs4LV5/i4BDxZXvDTwVzx/rwP8Ni+aBwN/DRzjf77PAT5bPNtfAy/F8/pt4HV40b008NvAMf5j/Q3w3sB3Ay/Fv8/fAC8NIJ7to4Gv4vl7HeC3edG9NPDdwEvxH+NvgNcGdoHjwF8DD+Lf5hLwYGAXQDzbceBW4BjP66+Bl+Ff77OBz+Lf7hLw1cBXA7s824OB7wZei3+dvwHeGriVKxDP6bOBz+L5+xzgs/nXezDw2cBbA8d40VwCfhr4aGCXF+yzgY8GjvHCXQK+GvhsnhPiOR0HbgWO8fy9DPDX/Nu9N/DSwEsDLw0c44q/AXaBvwZ+G/hp/nXeG3hr4DjwWlzxDOCvgZ8GfhrY5Xkhntd7A9/F8/fXwOsAu/zfgHj+fht4LZ6/vwZeB9jlfz/E8/dg4K+BYzx/Pw28D7DL/26IF+ytgZ/iBftr4HWAXf73QrxwXw18FC/YrcDbAH/N/06If9l3A+/FC/fZwOfwvw/iRfPdwHvxwv018DHAb/Of67WBjwKOA7cCHwPs8m+DeNF9N/Be/Mt+G/gc4Lf5j/XawGcBr81z+mvgZfi3QfzrfDXwUbxo/hr4buBngFv5t3kw8FbAewMvzQv2OsBv86+H+Nd7a+C7gWO86P4a+Gvgr4G/5orf4Tm9Fle8NPDSwEsDL82L5n2A7+ZfD/Fv82Dgu4HX4r/fJeClgVv510P8+7w38NXAMf57XALeG/hp/m0Q/37HgY8GPho4xn+NS8BXA18N7PJvh/iPcxx4a+CzgQfxn+NvgK8Gvpv/GIj/HC8NvDfw2sBL8e/zN8B3Az8N3Mp/LMR/vuPAawMvDTwYeDDwYOBBPKff4Ypd4K+B3wb+GtjlPw//CCniendO4s4DAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMention;
impl IconShape for GoMention {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.75 2.37a6.5 6.5 0 006.5 11.26.75.75 0 01.75 1.298 8 8 0 113.994-7.273.754.754 0 01.006.095v1.5a2.75 2.75 0 01-5.072 1.475A4 4 0 1112 8v1.25a1.25 1.25 0 002.5 0V7.867a6.5 6.5 0 00-9.75-5.496V2.37zM10.5 8a2.5 2.5 0 10-5 0 2.5 2.5 0 005 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMeter;
impl IconShape for GoMeter {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5a6.5 6.5 0 106.016 4.035.75.75 0 011.388-.57 8 8 0 11-4.37-4.37.75.75 0 01-.569 1.389A6.479 6.479 0 008 1.5zm6.28.22a.75.75 0 010 1.06l-4.063 4.064a2.5 2.5 0 11-1.06-1.06L13.22 1.72a.75.75 0 011.06 0zM7 8a1 1 0 112 0 1 1 0 01-2 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovivdxz4LOCtueK7gc/hvwfiv95XAx/Fc/oa4KP5r4f4r3cROM5zuhV4CP/1EP/1zPMn/ush/uuZ50/810P81zPPn/ivh/ivZ54/8V8P8V/PPH/ivx7iv555/sR/PcR/PfP8if96iP965vkT//UQ//XM8yf+6yFeuOPARwFvDbw0/3PtAr8NfDfwM7zoEC/YewNfBRznf5ffBt4G2OVfhnj+3hv4Lv73uhV4G+CveeEQz+s48HTgOP+77QLvA/w0LxjieX028Fn83/E+wHfz/CGe118BL83/Ld8NvA/PC/G8zPP3MsBf8z/TZwOfxQv328DbALs8G+J5medP/M/23sB38cL9NfA2wK1cgXhe5vkT//O9NPDbwDFesF3gdYC/BhDPyzx/4n+HBwM/DbwUL9gu8DrAX4vnZZ4/8b/HceCngdfiBftr4GXE8zLPn/jf57uB9+IFexvxvMzzJ/53em/gu3j+vkc8L/P8if+9Phr4Kp7X74jnZZ4/8b/XRwNfxfP6HfG8zPMn/nd6b+C7eP6+Rzwv8/yJ/32+C3hvXrC3Ec/LPH/if4/jwE8Br80L9jfAS4vnZZ4/8b/Dg4GfAl6aF+wS8NrAX4vnZZ4/8T/fSwO/BRznBbsEvDbw1wDieZnnT/zP9t7Ad/HC/Q3w1sCtXIF4Xub5exngr/mf6bOAz+aF+x3grYFdng3xvP4aeCn+b/ke4L15Xojn9dnAZ/F/x/sA383zh3hex4FbgWP873YJeG/gp3nBEM/fewPfxf9ezwDeGvhrXjjEC/bewFcDx/jf5XeAtwZ2+ZchXrjjwEcDbw28FP9zXQJ+G/hu4Kd50SH+65nnT/zXQ/zXM8+f+K+H+K9nnj/xXw/xX888f+K/HuK/nnn+xH89xH898/yJ/3qI/3rm+RP/9RD/9czzJ/7rIf7rmedP/NdD/Nczz5/4r4f4r2eeP/FfD/Ffzzx/4r8e4r+eef7Efz3Efz3z/In/eoj/eub5E//1EP/1zPMn/ush/uuZ50/810P81zPPn/ivh/ivZ54/8V8P8V/PPH/ivx7iv555/sR/PcR/vV3gGM/pGcCD+a+H+K/32cBn8Zw+B/hs/ush/nt8NfDWXPHdwGfz34N/BN5zg6HTG2rLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMilestone;
impl IconShape for GoMilestone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.75 0a.75.75 0 01.75.75V3h3.634c.414 0 .814.147 1.13.414l2.07 1.75a1.75 1.75 0 010 2.672l-2.07 1.75a1.75 1.75 0 01-1.13.414H8.5v5.25a.75.75 0 11-1.5 0V10H2.75A1.75 1.75 0 011 8.25v-3.5C1 3.784 1.784 3 2.75 3H7V.75A.75.75 0 017.75 0zm0 8.5h4.384a.25.25 0 00.161-.06l2.07-1.75a.25.25 0 000-.38l-2.07-1.75a.25.25 0 00-.161-.06H2.75a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMirror;
impl IconShape for GoMirror {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.75 1.75a.75.75 0 00-1.5 0v.5a.75.75 0 001.5 0v-.5zM8 4a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 4zm.75 3.75a.75.75 0 00-1.5 0v.5a.75.75 0 001.5 0v-.5zM8 10a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 10zm0 3a.75.75 0 01.75.75v.5a.75.75 0 01-1.5 0v-.5A.75.75 0 018 13zm7.547-9.939A.75.75 0 0116 3.75v8.5a.75.75 0 01-1.265.545l-4.5-4.25a.75.75 0 010-1.09l4.5-4.25a.75.75 0 01.812-.144zM11.842 8l2.658 2.51V5.49L11.842 8zM0 12.25a.75.75 0 001.265.545l4.5-4.25a.75.75 0 000-1.09l-4.5-4.25A.75.75 0 000 3.75v8.5zm1.5-6.76L4.158 8 1.5 10.51V5.49z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMoon;
impl IconShape for GoMoon {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.598 1.591a.75.75 0 01.785-.175 7 7 0 11-8.967 8.967.75.75 0 01.961-.96 5.5 5.5 0 007.046-7.046.75.75 0 01.175-.786zm1.616 1.945a7 7 0 01-7.678 7.678 5.5 5.5 0 107.678-7.678z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMortarBoard;
impl IconShape for GoMortarBoard {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.693 1.066a.75.75 0 01.614 0l7.25 3.25a.75.75 0 010 1.368L13 6.831v2.794c0 1.024-.81 1.749-1.66 2.173-.893.447-2.075.702-3.34.702-.278 0-.55-.012-.816-.036a.75.75 0 01.133-1.494c.22.02.45.03.683.03 1.082 0 2.025-.221 2.67-.543.69-.345.83-.682.83-.832V7.503L8.307 8.934a.75.75 0 01-.614 0L4 7.28v1.663c.296.105.575.275.812.512.438.438.688 1.059.688 1.796v3a.75.75 0 01-.75.75h-3a.75.75 0 01-.75-.75v-3c0-.737.25-1.358.688-1.796.237-.237.516-.407.812-.512V6.606L.443 5.684a.75.75 0 010-1.368l7.25-3.25zM2.583 5L8 7.428 13.416 5 8 2.572 2.583 5zM2.5 11.25c0-.388.125-.611.25-.735a.704.704 0 01.5-.203c.19 0 .37.071.5.203.125.124.25.347.25.735v2.25H2.5v-2.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b43+evgJfmP8Zfi/99/hp4Kf5j/I14XseBzwLeGngw/zVuBb4b+Bz+ZceBvwYexL/PM4CXFs/rq4GP4r/H1wAfzb/spYHfBo7xb3MJeG3gr8Xzuggc57/HrcBDeNG8NPDbwDH+dS4Brw38NYB4XrvAMf57PAN4MC+6twZ+in+dtwF+misQz+uzgc/iv8fnAJ/Nv857A9/Fi+Z9gO/m2RDP31cDbw08iP8azwC+G/hs/m0+G/gsXriPAb6a54T4v+O7gffi+fse4L15Xoj/W74beC+e0/cA783zh/i/5Tjw28BLccXfAK8N7PL8If7vOQ78Nle8NrDLC4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+GeP4eDHwV8NJc8dfAxwC38n8L4nk9GPgr4DjPaRd4GeBW/u9APK+fBt6K5+9ngLfmhTsOfBbw1sCD+e9xK/DdwOfwwiGe10XgOM/frcBDeOG+Gvgo/mf4GuCjecEQz2sXOMbz9wzgwbxwF4Hj/M9wK/AQXjDE8/pp4K14/n4GeGteuF3gGP8zPAN4MC8Y4nk9GPhr4BjP6RLw0sCtvHCfDXwW/zN8DvDZvGCI5+/BwFcDL80Vfw18NHArL5qvBt4aeBD/PZ4BfDfw2bxwiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4/h4MfBXw0lzx18DHALfyfwvieT0Y+CvgOM9pF3gZ4Fb+70A8r58G3orn72eAt+aFOw58FvDWwIP573Er8N3A5/DCIZ7XReA4z9+twEN44b4a+Cj+Z/ga4KN5wRDPaxc4xvP3DODBvHAXgeP8z3Ar8BBeMMTz+mngrXj+fgZ4a164XeAY/zM8A3gwLxjieT0Y+GvgGM/pEvDSwK28cJ8NfBb/M3wO8Nm8YIjn78HAVwMvzRV/DXw0cCsvmq8G3hp4EP89ngF8N/DZvHCI/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/CN+GWzwkT48cgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMultiSelect;
impl IconShape for GoMultiSelect {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5zm4 5a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5zm0 5a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
            path {
                d: "M13.314 4.918L11.07 2.417A.25.25 0 0111.256 2h4.488a.25.25 0 01.186.417l-2.244 2.5a.25.25 0 01-.372 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3Mc+Cjgo7niu4GP4b8X4r/GceC3gJfmOX0N8NH890H853tp4KeAB/O8doET/PdB/Od6beCngOO8YOK/D+I/z3sD38W/TPz3Qfzn+Crgo3nRiP8+iP9Yx4GvAt6bF53474P4j3Mc+C3gpfnXEf99EP8xXhr4KeDB/OuJ/z6If7/XBn4KOM4L9gzgQTx/4r8P4t/nvYHv4oX7G+C1gYs8f+K/D+Lf7quAj+aF+x7go4FdwDx/4r8P4jkdBz4KeGvgpfn3+Rrgo3k28/yJF+448F3AW3PFVwMfw7/NZwFvzRXfDXyNeLb3Br4KOM6/3/sA381zMs+feOF+GngrntN3A+/Dv853Ae/Nc/oYccV7A9/Fv98l4K2B3+Z5medPvHDm+ftu4H140XwX8N48r78WcBx4OnCcf59nAG8N/DXPn3n+xAtnXrDvBt6HF+67gPfm+fsbAZ8NfBb/Pn8DvDawywtmnj/xwn018FG8YN8NvA/P33cB780L9jkC/gp4af5tLgFfDXw1sMsLZ54/8S/7buC9eMG+G3gfntN3Ae/NC/Y9wHsLMM/fywB/zX8c8/yJF813A+/FC/bdwPtwxXcB780L9j3AewMIMM+f+I9lnj/xovtu4L14wb6bK96bF+x7gPfmCgSY50/8xzLPn/jX+W7gvfi3+R7gvXk2BJjnT/zHMs+f+Nf7buC9+Nf5HuC9eU4IMM+f+I9lnj/xb/PdwHvxovke4L15Xggwz5/4j2WeP/Fv993Ae/HCfQ/w3jx/CDDPn/iPZZ4/8W/3XcB788J9N/A+PH8IMM+f+I9lnj/xb/NdwHvzovlu4H14Xggwz5/4j2WeP/Gv913Ae/Ov893A+/CcEGCeP/Efyzx/4l/nu4D35t/mu4H34dkQYJ4/8R/LPH/iRfddwHvzgn0PV7wXL9h3A+/DFQgwz9/LAH/Nfxzz/IkXzXcB780L9j3Ae3PFdwPvxQv23cD7AAj4a+Cl+LfZBb4a+BpglxfOPH/iX/ZdwHvzgn0P8N48p+8G3osX7LuB9xHw2cBn8e/z18DrALu8YOb5Ey/cVwMfxQv2PcB78/x9N/BevGBfI+A4cCtwjH+fW4G3Af6a5888f+KFMy/Y9wDvzQv33cB78fwhrnhv4Lv499sF3gb4bZ6Xef7EC2eev+8B3psXzXcD78XzQjzbewNfDRzj3+99gO/mOZnnT7xwPw28Fc/pe4D35l/nu4H34jn9jHhOx4GPBt4aeCn+fb4a+BiezTx/4oU7Dnw38FbAJeC7gY/m3+argY/iiu8BPlr823018FG8cN8NfAywC5jnT/z3Qfz7vDfwXbxwfw28DnCR50/890H8+7028NPAMV6wW4EH8/yJ/z6I/xgvDfw08CD+9cR/H8R/nOPAbwMvxb+O+O+D+I91HPhq4L140Yn/Poj/HF8NfBQvGvHfB/Gf572B7+JfJv77IP5zvTbw08AxXjDx3wfxn++lgZ8GHsTzugQc578P4r/GceC3gZfiOX0N8NH890H81zkOfDTw0Vzx3cBH898L8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/jX8ENv+9HpsbWrwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoMute;
impl IconShape for GoMute {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 2.75a.75.75 0 00-1.238-.57L3.472 5H1.75A1.75 1.75 0 000 6.75v2.5C0 10.216.784 11 1.75 11h1.723l3.289 2.82A.75.75 0 008 13.25V2.75zM4.238 6.32L6.5 4.38v7.24L4.238 9.68a.75.75 0 00-.488-.18h-2a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h2a.75.75 0 00.488-.18zm7.042-1.1a.75.75 0 10-1.06 1.06L11.94 8l-1.72 1.72a.75.75 0 101.06 1.06L13 9.06l1.72 1.72a.75.75 0 101.06-1.06L14.06 8l1.72-1.72a.75.75 0 00-1.06-1.06L13 6.94l-1.72-1.72z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNoEntry;
impl IconShape for GoNoEntry {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.25 7.25a.75.75 0 000 1.5h7.5a.75.75 0 000-1.5h-7.5z",
            }
            path {
                d: "M16 8A8 8 0 110 8a8 8 0 0116 0zm-1.5 0a6.5 6.5 0 11-13 0 6.5 6.5 0 0113 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovivdxz4LOCtueK7gc/hvwfiv95XAx/Fc/oa4KP5r4f4r3cROM5zuhV4CP/1EP/1zPMn/ush/uuZ50/810P81zPPn/ivh/ivZ54/8V8P8V/PPH/ivx7iv555/sR/PcR/PfP8if96iP965vkT//UQ//XM8yf+6yH+bY4D7wUcB24FvocXnXn+xIvurYCXBv4a+Bn+7RD/eseB3wJemmf7buB9eNGY50+8aL4LeG+e7aeBt+HfBvGv99HAV/G8vht4H/5l5vkT/7LvAt6b5/U2wE/zr4f41/ts4LN4/r4beB9eOPP8iRfuu4D35vn7HOCz+ddD/Ou9N/BdvGDfDbwPL5h5/sQL9l3Ae/OCvQ7w2/zrIf5tvht4L16w7wbeh+fPPH/i+fsu4L15wb4HeG/+bRD/dt8NvBcv2HcD78PzMs+feF7fBbw3L9j3AO/Nvx3i3+e7gffiBftu4H14Tub5E8/pu4D35gX7HuC9+fdB/Pt9N/BevGDfDbwPz2aeP/Fs3wW8Ny/Y9wDvzb8f4j/GdwPvxQv23cD7cIV5/sQV3wW8Ny/Y9wDvzX8MxH+c7wbeixfsu4H3AczzJ+C7gPfmBfse4L35j4P4j/XdwHvxgn038N48f98NvDcv2PcA781/LMR/vO8G3ov/WN8DvDf/8RD/Ob4beC/+Y3wP8N7850D85/lu4L349/ke4L35z4P4z/XTwFvxb/M9wHvznwsBx4HPAt4aeDD/t90KfDfwOVyBgK8GPor/X74G+GgAAReB4/z/civwEAABu8Ax/n95BvBgAAGfDXwW/798DvDZAOKKrwbeGngQ/7c9A/hu4LO5AvGf67uA9+bf5ruB9+E/F+I/z3cB782/z3cD78N/HsR/ju8C3pv/GN8NvA//ORD/8b4LeG/+Y3038D78x0P8x/ou4L15wb4HeC+ev+8B3osX7LuB9+E/FuI/xnHgp4DX5gX7HuC9AfP8Cfhu4L14wb4beB/+4yD+/Y4DvwW8NC/Y9wDvzRXm+RNXfDfwXrxg3w28D/8xEP8+x4HfAl6aF+x7gPfm2czzJ57tu4H34gX7buB9+PdD/NsdB34LeGlesO8B3pvnZJ4/8Zy+G3gvXrDvBt6Hfx/Ev81x4LeAl+YF+x7gvXle5vkTz+u7gffiBftu4H34t0P82/wW8Nq8YN8DvDfPn3n+xPP33cB78YJ9N/A+/Nsg/vXeG/guXrDvAd6bF8w8f+IF+27gvXjB3gf4bv71EP96nw18Fs/f9wDvzQtnnj/xwn038F48f58DfDb/eoh/vY8Gvorn9T3Ae/MvM8+f+Jd9N/BePK+PAb6afz3Ev95x4LeBl+LZvgd4b1405vkTL5qfBt6KZ/sb4LWBXf71EP82x4H3Bo4DtwLfzYvOPH/iRffewIOBXeC7gV3+bRD/9czzJ/7rIf7rmedP/NdD/Nczz5/4r4f4r2eeP/FfD/Ffzzx/4r8e4r+eef7Efz3Efz3z/In/eoj/eub5E//1EP/1zPMn/ush/uvtAsd4Ts8AHsx/PcR/vc8GPovn9DnAZ/NfD/Hf46uBt+aK7wY+m/8eiP/fEP+/8Y+8j7RBcVaMsQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNorthStar;
impl IconShape for GoNorthStar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.5.75a.75.75 0 00-1.5 0v5.19L4.391 3.33a.75.75 0 10-1.06 1.061L5.939 7H.75a.75.75 0 000 1.5h5.19l-2.61 2.609a.75.75 0 101.061 1.06L7 9.561v5.189a.75.75 0 001.5 0V9.56l2.609 2.61a.75.75 0 101.06-1.061L9.561 8.5h5.189a.75.75 0 000-1.5H9.56l2.61-2.609a.75.75 0 00-1.061-1.06L8.5 5.939V.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADXElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4TseBjwLeGnhp/m/5a+Cnga8BdrkC8WzvDXwVcJz/23aBjwG+G0Bc8d7Ad/H/y/sA3y3gOPB04Dj/v+wCDxHw2cBn8f/T5wj4K+Cl+f/prwWY5+9lgL/m/4aXBv6K54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RMv3HHgs4C3Bh7Mf61bge8GPocXnXleCDDPn3jhvhr4KP57fQ3w0bxozPNCgHn+xAt3ETjOf69bgYfwojHPCwHm+RMv3C5wjP9ezwAezIvGPC8EmOdPvHCfDXwW/70+B/hsXjTmeSHAPH/iX/bVwFsDD+K/1jOA7wY+mxedeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/ihTsOfBbw1sCD+fe5Ffhu4HP4z2OeFwLM8ydeuK8GPor/WF8DfDT/OczzQoB5/sQLdxE4zn+sW4GH8J/DPC8EmOdPvHC7wDH+Yz0DeDD/OczzQoB5/sQL99nAZ/Ef63OAz+Y/h3leCDDPn/iXfTXw1sCD+Pd5BvDdwGfzn8c8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszz9zLAX/N/w0sDf8XzQsBfAy/F/09/I+Czgc/i/6fPEXAcuBU4xv8vl4AHiyveG/gu/n95G+CnxbO9N/DVwDH+b7sEvDfw0wDiOR0HPhp4a+Cl+L/lb4CfBr4a2OUKxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4Ro+2C3BGcfEcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNote;
impl IconShape for GoNote {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5zm1.75-.25a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H1.75zM3.5 6.25a.75.75 0 01.75-.75h7a.75.75 0 010 1.5h-7a.75.75 0 01-.75-.75zm.75 2.25a.75.75 0 000 1.5h4a.75.75 0 000-1.5h-4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoNumber;
impl IconShape for GoNumber {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.604.089A.75.75 0 016 .75v4.77h.711a.75.75 0 110 1.5H3.759a.75.75 0 110-1.5H4.5V2.15l-.334.223a.75.75 0 01-.832-1.248l1.5-1a.75.75 0 01.77-.037zM9 4.75A.75.75 0 019.75 4h4a.75.75 0 01.53 1.28l-1.89 1.892c.312.076.604.18.867.319.742.391 1.244 1.063 1.244 2.005 0 .653-.231 1.208-.629 1.627-.386.408-.894.653-1.408.777-1.01.243-2.225.063-3.124-.527a.75.75 0 01.822-1.254c.534.35 1.32.474 1.951.322.306-.073.53-.201.67-.349.129-.136.218-.32.218-.596 0-.308-.123-.509-.444-.678-.373-.197-.98-.318-1.806-.318a.75.75 0 01-.53-1.28l1.72-1.72H9.75A.75.75 0 019 4.75zm-3.587 5.763c-.35-.05-.77.113-.983.572a.75.75 0 11-1.36-.632c.508-1.094 1.589-1.565 2.558-1.425 1 .145 1.872.945 1.872 2.222 0 1.433-1.088 2.192-1.79 2.681-.308.216-.571.397-.772.573H7a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75c0-.69.3-1.211.67-1.61.348-.372.8-.676 1.15-.92.8-.56 1.18-.904 1.18-1.474 0-.473-.267-.69-.587-.737z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoOrganization;
impl IconShape for GoOrganization {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 14.25c0 .138.112.25.25.25H4v-1.25a.75.75 0 01.75-.75h2.5a.75.75 0 01.75.75v1.25h2.25a.25.25 0 00.25-.25V1.75a.25.25 0 00-.25-.25h-8.5a.25.25 0 00-.25.25v12.5zM1.75 16A1.75 1.75 0 010 14.25V1.75C0 .784.784 0 1.75 0h8.5C11.216 0 12 .784 12 1.75v12.5c0 .085-.006.168-.018.25h2.268a.25.25 0 00.25-.25V8.285a.25.25 0 00-.111-.208l-1.055-.703a.75.75 0 11.832-1.248l1.055.703c.487.325.779.871.779 1.456v5.965A1.75 1.75 0 0114.25 16h-3.5a.75.75 0 01-.197-.026c-.099.017-.2.026-.303.026h-3a.75.75 0 01-.75-.75V14h-1v1.25a.75.75 0 01-.75.75h-3zM3 3.75A.75.75 0 013.75 3h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 3.75zM3.75 6a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM3 9.75A.75.75 0 013.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 9.75zM7.75 9a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM7 6.75A.75.75 0 017.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 017 6.75zM7.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackage;
impl IconShape for GoPackage {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.878.392a1.75 1.75 0 00-1.756 0l-5.25 3.045A1.75 1.75 0 001 4.951v6.098c0 .624.332 1.2.872 1.514l5.25 3.045a1.75 1.75 0 001.756 0l5.25-3.045c.54-.313.872-.89.872-1.514V4.951c0-.624-.332-1.2-.872-1.514L8.878.392zM7.875 1.69a.25.25 0 01.25 0l4.63 2.685L8 7.133 3.245 4.375l4.63-2.685zM2.5 5.677v5.372c0 .09.047.171.125.216l4.625 2.683V8.432L2.5 5.677zm6.25 8.271l4.625-2.683a.25.25 0 00.125-.216V5.677L8.75 8.432v5.516z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackageDependencies;
impl IconShape for GoPackageDependencies {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.122.392a1.75 1.75 0 011.756 0l5.25 3.045c.54.313.872.89.872 1.514V7.25a.75.75 0 01-1.5 0V5.677L7.75 8.432v6.384a1 1 0 01-1.502.865L.872 12.563A1.75 1.75 0 010 11.049V4.951c0-.624.332-1.2.872-1.514L6.122.392zM7.125 1.69l4.63 2.685L7 7.133 2.245 4.375l4.63-2.685a.25.25 0 01.25 0zM1.5 11.049V5.677l4.75 2.755v5.516l-4.625-2.683a.25.25 0 01-.125-.216zm11.672-.282a.75.75 0 10-1.087-1.034l-2.378 2.5a.75.75 0 000 1.034l2.378 2.5a.75.75 0 101.087-1.034L11.999 13.5h3.251a.75.75 0 000-1.5h-3.251l1.173-1.233z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPackageDependents;
impl IconShape for GoPackageDependents {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.122.392a1.75 1.75 0 011.756 0l5.25 3.045c.54.313.872.89.872 1.514V7.25a.75.75 0 01-1.5 0V5.677L7.75 8.432v6.384a1 1 0 01-1.502.865L.872 12.563A1.75 1.75 0 010 11.049V4.951c0-.624.332-1.2.872-1.514L6.122.392zM7.125 1.69l4.63 2.685L7 7.133 2.245 4.375l4.63-2.685a.25.25 0 01.25 0zM1.5 11.049V5.677l4.75 2.755v5.516l-4.625-2.683a.25.25 0 01-.125-.216zm10.828 3.684a.75.75 0 101.087 1.034l2.378-2.5a.75.75 0 000-1.034l-2.378-2.5a.75.75 0 00-1.087 1.034L13.501 12H10.25a.75.75 0 000 1.5h3.251l-1.173 1.233z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaintbrush;
impl IconShape for GoPaintbrush {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.134 1.535C9.722 2.562 8.16 4.057 6.889 5.312 5.8 6.387 5.041 7.401 4.575 8.294a3.745 3.745 0 00-3.227 1.054c-.43.431-.69 1.066-.86 1.657a11.982 11.982 0 00-.358 1.914A21.263 21.263 0 000 15.203v.054l.75-.007-.007.75h.054a14.404 14.404 0 00.654-.012 21.243 21.243 0 001.63-.118c.62-.07 1.3-.18 1.914-.357.592-.17 1.226-.43 1.657-.861a3.745 3.745 0 001.055-3.217c.908-.461 1.942-1.216 3.04-2.3 1.279-1.262 2.764-2.825 3.775-4.249.501-.706.923-1.428 1.125-2.096.2-.659.235-1.469-.368-2.07-.606-.607-1.42-.55-2.069-.34-.66.213-1.376.646-2.076 1.155zm-3.95 8.48a3.76 3.76 0 00-1.19-1.192 9.758 9.758 0 011.161-1.607l1.658 1.658a9.853 9.853 0 01-1.63 1.142zM.742 16l.007-.75-.75.008A.75.75 0 00.743 16zM12.016 2.749c-1.224.89-2.605 2.189-3.822 3.384l1.718 1.718c1.21-1.205 2.51-2.597 3.387-3.833.47-.662.78-1.227.912-1.662.134-.444.032-.551.009-.575h-.001V1.78c-.014-.014-.112-.113-.548.027-.432.14-.995.462-1.655.942zM1.62 13.089a19.56 19.56 0 00-.104 1.395 19.55 19.55 0 001.396-.104 10.528 10.528 0 001.668-.309c.526-.151.856-.325 1.011-.48a2.25 2.25 0 00-3.182-3.182c-.155.155-.329.485-.48 1.01a10.515 10.515 0 00-.309 1.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaperAirplane;
impl IconShape for GoPaperAirplane {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.592 2.712L2.38 7.25h4.87a.75.75 0 110 1.5H2.38l-.788 4.538L13.929 8 1.592 2.712zM.989 8L.064 2.68a1.341 1.341 0 011.85-1.462l13.402 5.744a1.13 1.13 0 010 2.076L1.913 14.782a1.341 1.341 0 01-1.85-1.463L.99 8z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEeklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/eMeBzwLeGngw/zFuBb4b+Bz+YyH+43018FH85/ga4KP5j4P4j3cROM5/jluBh/AfB/Efz/znEv9xEP86x4H34orvAXZ5XuY/l3hex4H34orvAXZ50SBedG8NfBdwnCt2gdcB/prnZP5zief00sBvAce5Yhd4H+Cn+ZchXjRvDfwUz+uvgZfhOZnnT/zrmOdPPKe/Al6a5/U2wE/zwiH+ZceBpwPHef7EczLPn/jXMc+feE7m+dsFHgLs8oIh/mWfDXwWz9/vAK/NczLPn/jXMc+feE5/DbwUz9/nAJ/NC4b4l/028Fo8r0vAawN/zXMyz5/41zHPn3hOLw38NnCM5/U7wGvzgiH+Zeb5exngr3le5vkT/zrm+RPP66WBv+L5Ey8Y4l9mnj/x/JnnT/zrmOdPPH/m+RMvGOJfZp4/8fyZ/1zi+TPPn3jBEP8y8/yJ58/85xLPn3n+xAuG+JeZ5088f+Y/l3j+zPMnXjDEv8w8f+L52wWO8Z/jGcCDef7M8ydeMMS/zDx/4vn7bOCz+M/xOcBn8/yZ50+8YIh/mXn+xAv22cB7Aw/iP8YzgO8GPpsXzDx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPPn/ifxTx/4gVD/MvM8yf+ZzHPn3jBEP8y8/yJ/1nM8ydeMMS/zDx/4n8W8/yJFwzxLzPP38sAf83/DC8N/BXPn3jBEP+yvwZeiv+d/gZ4aV4wxL/ss4HP4n+nzwE+mxcM8S87DtwKHON/l0vAg4FdXjDEi+a9ge/if5e3AX6aFw7xontv4KuBY/zPdgl4b+Cn+Zch/nWOAx8NvDXwUvzP8jfATwNfDezyokH8/4b4/w3x/xvi/zf+EfqKnUEXAAeZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPaste;
impl IconShape for GoPaste {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPencil;
impl IconShape for GoPencil {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.013 1.427a1.75 1.75 0 012.474 0l1.086 1.086a1.75 1.75 0 010 2.474l-8.61 8.61c-.21.21-.47.364-.756.445l-3.251.93a.75.75 0 01-.927-.928l.929-3.25a1.75 1.75 0 01.445-.758l8.61-8.61zm1.414 1.06a.25.25 0 00-.354 0L10.811 3.75l1.439 1.44 1.263-1.263a.25.25 0 000-.354l-1.086-1.086zM11.189 6.25L9.75 4.81l-6.286 6.287a.25.25 0 00-.064.108l-.558 1.953 1.953-.558a.249.249 0 00.108-.064l6.286-6.286z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPeople;
impl IconShape for GoPeople {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.5 3.5a2 2 0 100 4 2 2 0 000-4zM2 5.5a3.5 3.5 0 115.898 2.549 5.507 5.507 0 013.034 4.084.75.75 0 11-1.482.235 4.001 4.001 0 00-7.9 0 .75.75 0 01-1.482-.236A5.507 5.507 0 013.102 8.05 3.49 3.49 0 012 5.5zM11 4a.75.75 0 100 1.5 1.5 1.5 0 01.666 2.844.75.75 0 00-.416.672v.352a.75.75 0 00.574.73c1.2.289 2.162 1.2 2.522 2.372a.75.75 0 101.434-.44 5.01 5.01 0 00-2.56-3.012A3 3 0 0011 4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPerson;
impl IconShape for GoPerson {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.5 5a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0zm.061 3.073a4 4 0 10-5.123 0 6.004 6.004 0 00-3.431 5.142.75.75 0 001.498.07 4.5 4.5 0 018.99 0 .75.75 0 101.498-.07 6.005 6.005 0 00-3.432-5.142z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPersonAdd;
impl IconShape for GoPersonAdd {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13.25 0a.75.75 0 01.75.75V2h1.25a.75.75 0 010 1.5H14v1.25a.75.75 0 01-1.5 0V3.5h-1.25a.75.75 0 010-1.5h1.25V.75a.75.75 0 01.75-.75zM5.5 4a2 2 0 100 4 2 2 0 000-4zm2.4 4.548a3.5 3.5 0 10-4.799 0 5.527 5.527 0 00-3.1 4.66.75.75 0 101.498.085A4.01 4.01 0 015.5 9.5a4.01 4.01 0 014.001 3.793.75.75 0 101.498-.086 5.527 5.527 0 00-3.1-4.659z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/Wq8FvDXw0sBLA8e5Yhf4a+Cvgd8Gfob/Goj/fMeBjwI+GjjOi+ZW4LuBrwF2+c+D+M/11sB3Acf5t9kF3gf4af5zIP7zfBXw0fzH+GrgY/iPh/jP8V3Ae/Mf67uB9+E/FuI/3ncB781/ju8G3of/OIj/WG8N/BT/ud4G+Gn+YyD+4xwHng4c5z/XLvAQYJd/P8R/nM8GPov/Gp8DfDb/foj/GMeBpwPH+a+xC5zg3w/xH+OtgZ/iv9bbAD/Nvw/iP8ZXAx/Ff62vAT6afx/Ef4zfBl6L/1q/A7w2/z6I/xgXgeP819oFTvDvg/iPYf57iH8fxH8M899D/Psg/mOY/x7i3wfxH+O3gdfiv9bvAK/Nvw/iP8Z3A+/Ff62vAT6afx/Ef4y3Bn6K/1rvA3w3/z6I/xjHgVuBY/zXuAQc598P8R/ns4HP4r/G5wCfzb8f4j/OceBW4Bj/uS4BDwZ2+fdD/Mf6aOCr+M/1PsB38x8D8R/vu4H34j/H9wDvzX8cxH+O7wbei/9YPwO8Nf+xEP95vht4L/5jfA/w3vzHQ/znOQ78FPDa/Pv8NvA2wC7/8RD/8R4MvBfw0cBx/mPsAl8NfA2wy38cxH+sjwI+GzjOf45d4LOBr+E/BuI/xksDXwW8Nv81fht4H+BW/n0Q/36vDfwUcJz/WrvA2wC/zb8d4t/nvYHv4r/X+wDfzb8N4t/uvYHv4n+G9wG+m389xL/NawO/xf8srwP8Nv86iH+9lwZ+CzjO/yy7wOsAf82LDvGv91fAS/M/028Dr8OLDvGv89HAV/E/28cAX82LBvGiezDwV8Bx/mfbBR4C7PIvQ7zoPhv4LP53+Bzgs/mXIV40x4GnA8f532EXeAiwywuHeNG8N/Bd/O/yPsB388IhXjS/DbwW/7v8DvDavHCIf9lx4CL/O50AdnnBEP+ytwZ+iv+d3gb4aV4wxL/ss4HP4n+nzwE+mxcM8S/7beC1+N/pd4DX5gVD/Mt+G3gt/nf6HeC1ecEQ/zLzv5t4wRD/sluBB/G/0zOAB/OCIf5lnw18Fv87fQ7w2bxgiBfNVwNvDTyI/x2eAXw38Nm8cIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I4WndkFYFrBbAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPersonFill;
impl IconShape for GoPersonFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.243 4.757a3.757 3.757 0 115.851 3.119 6.006 6.006 0 013.9 5.339.75.75 0 01-.715.784H2.721a.75.75 0 01-.714-.784 6.006 6.006 0 013.9-5.34 3.753 3.753 0 01-1.664-3.118z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPin;
impl IconShape for GoPin {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.456.734a1.75 1.75 0 012.826.504l.613 1.327a3.081 3.081 0 002.084 1.707l2.454.584c1.332.317 1.8 1.972.832 2.94L11.06 10l3.72 3.72a.75.75 0 11-1.061 1.06L10 11.06l-2.204 2.205c-.968.968-2.623.5-2.94-.832l-.584-2.454a3.081 3.081 0 00-1.707-2.084l-1.327-.613a1.75 1.75 0 01-.504-2.826L4.456.734zM5.92 1.866a.25.25 0 00-.404-.072L1.794 5.516a.25.25 0 00.072.404l1.328.613A4.582 4.582 0 015.73 9.63l.584 2.454a.25.25 0 00.42.12l5.47-5.47a.25.25 0 00-.12-.42L9.63 5.73a4.581 4.581 0 01-3.098-2.537L5.92 1.866z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlay;
impl IconShape for GoPlay {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM6.379 5.227A.25.25 0 006 5.442v5.117a.25.25 0 00.379.214l4.264-2.559a.25.25 0 000-.428L6.379 5.227z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+53ow8FHAawMvzfP6a+Cnga8Bdvm3QfzP9N7AVwHH+ZftAh8DfDf/eoj/ed4b+C7+9d4H+G7+dRD/s7w38F382+wCDwF2edEh/ud4aeC3gOP8230O8Nm86BD/MxwHng4c59/nr4GX4UWH+K91HHgp4Djw0sBx4KWBBwMP5vm7BLw0cCvP9tLAX/H8iRcd4j/eceClgNcGjgMvDRwHXpp/vUvAawN/zfMyz5940SH+Y30W8NHAcf5jvA/w3Tx/5vkTLzrEf4zjwG8BL81/nM8BPpsXzDx/4kWH+I/xU8Bb8x/ne4D35oUz/3a3At8NfI7493tt4Lf4j3EJ+G7go/mXmX+/rxH/fj8NvBUvur8BdoG/BnaBvwZ2gb8GdnnRmX+/W8W/30XgOM/rd4DfBv4a2AX+GtjlP47593uG+Pczz98JYJf/PObf73PEv595/sR/LvNv9wzgu4HPFv9+5vkT/7nM8ydedIh/P/P8if9c5vkTLzrEv595/sR/LvP8iRcd4t/PPH/iP5d5/sSLDvGCvRfw2sBLAy/Nv574z2WeP/GiQzyvtwa+Cngw/z7iP5d5/sSLDvGcvgr4aP5jiP9c5vkTLzrEs3018FH8xxH/uczzJ150iCveGvgp/mOJ/1zm+RMvOsQVTwcezH8s8Z/LPH/iRYeA9wa+i+fvGcBHA38N3MrzZ54/8Z/LPH/iRYeA7wbei+f1DOClgV1eOPP8if9c5vkTLzoE/BXw0jyvtwF+mn+Zef7Efy7z/IkXHQLM83cC2OVfZp4/8Z/LPH/iRYcA8/yJF415/sR/LvP8iRcdAszzJ1405vkT/7nM8ydedAgwz5940ZjnT/znMv92twLfDXyOAPP8iReNef5OALv85zH/fl8jwDx/4kWzCxzjef028NvAXwO7wN8Au/zHMf9+twowz5940fw08Fa86P4a2AX+GtgF/hrYBf4G2OVFZ/79niHAPH/iRfPawG/xH2MX+G7gY/iXmX+/zxFgnj/xovtp4K34j/PdwPvwwpl/u2cA3w18tgDz/IkX3XHgt4GX4j/O5wCfzQtmnj/xokOAef7Ev95nAx8NHOM/xvsA383zZ54/8aJDgHn+xL/NceClgdcGjgMvDRwHXop/vV3gdYC/5nmZ50+86BBgnj/xH+848NLAceClgePASwMPBh7E87cLvAxwK8/20sBf8fyJFx0CzPMn/uscB24FjvHv8zfAS/OiQ4B5/sR/rZcGfhs4xr/d5wCfzYsOAeb5E//13hv4Lv5tLgEPBnZ50SHAPH/iv8d7A9/Fv97bAD/Nvw4CzPMn/vu8N/DVwDH+ZZeA9wZ+mn89BJjnT/z3ejDw0cBrAy/F8/ob4KeBrwZ2+bdBgHn+xP99CDDPn/i/DwHm+RP/9yFgFzjGc3oG8GD+70PAZwOfxXP6HOCz+b8PccVXA2/NFd8NfDb/PyD+f0P8/4b4/w3x/xv/CKxQx982Af7CAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlug;
impl IconShape for GoPlug {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.276 3.09a.25.25 0 01.192-.09h.782a.25.25 0 01.25.25v8.5a.25.25 0 01-.25.25h-.782a.25.25 0 01-.192-.09l-.95-1.14a.75.75 0 00-.483-.264l-3.124-.39a.25.25 0 01-.219-.249V5.133a.25.25 0 01.219-.248l3.124-.39a.75.75 0 00.483-.265l.95-1.14zM4 8v1.867a1.75 1.75 0 001.533 1.737l2.83.354.761.912c.332.4.825.63 1.344.63h.782A1.75 1.75 0 0013 11.75V11h2.25a.75.75 0 000-1.5H13v-4h2.25a.75.75 0 000-1.5H13v-.75a1.75 1.75 0 00-1.75-1.75h-.782c-.519 0-1.012.23-1.344.63l-.76.913-2.831.353A1.75 1.75 0 004 5.133V6.5H2.5A2.5 2.5 0 000 9v5.25a.75.75 0 001.5 0V9a1 1 0 011-1H4z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACpklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3cc+Czgrbniu4HP4b8H4r/eVwMfxXP6GuCj+a+H+K93ETjOc7oVeAj/9RD/9czzJ/7rIf7rmedP/NdD/Nczz5/4r4f4r2eeP/FfD/Ffzzx/4r8e4r+eef7Efz3Efz3z/In/eoj/eub5E//1EP/1zPMn/ush/uuZ50/810P81zPPn/ivh/ivZ54/8V8P8V/PPH/ivx7iv555/sR/PcR/PfP8if96iP965vkT//UQ//XM8yf+6yH+ZceBzwLeGngw/7PdCnw38Dm8aBD/sq8GPor/Xb4G+Gj+ZYh/2UXgOP+73Ao8hH8Z4l+2Cxzjf5dnAA/mX4b4l3028Fn87/I5wGfzL0O8aL4aeGvgQfzP9gzgu4HP5kWD+K9nnj/xXw/xX888f+K/HuK/nnn+xH89xH898/yJ/3qI/3rm+RP/9RD/9czzJ/7rIf7rmedP/NdD/Nczz5/4r4f4r2eeP/FfD/Ffzzx/4r8e4r+eef7Efz3Efz3z/In/eoj/eub5E//1EP/1zPMn/ush/uuZ50/810P81zPPn/ivh/ivZ54/8V8P8V9vFzjGc3oG8GD+6yH+63028Fk8p88BPpv/eoj/Hl8NvDVXfDfw2fz3QPz/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BLNDPEGYVbJDAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlus;
impl IconShape for GoPlus {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.75 2a.75.75 0 01.75.75V7h4.25a.75.75 0 110 1.5H8.5v4.25a.75.75 0 11-1.5 0V8.5H2.75a.75.75 0 010-1.5H7V2.75A.75.75 0 017.75 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPlusCircle;
impl IconShape for GoPlusCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm.75 4.75a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+b/hpcG/ornhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT7xwx4HPAt6aK74b+BxeNMeBzwLemiu+G/gcXjTHgc8C3porvhv4HP5l5nkhwDx/4oX7auCjeE5fA3w0/7KvBj6K5/Q1wEfzL/tq4KN4Tl8DfDQvnHleCDDPn3jhLgLHeU63Ag/hX3YROM5zuhV4CP+yi8BxntOtwEN44czzQoB5/sQLZ54/8S8zz5/4l5nnT7xw5nkhwDx/4oUzz5/4l5nnT/zLzPMnXjjzvBBgnj/xwpnnT/zLzPMn/mXm+RMvnHleCDDPn3jhzPMn/mXm+RP/MvP8iRfOPC8EmOdPvHDm+RP/MvP8iX+Zef7EC2eeFwLM8ydeOPP8iX+Zef7Ev8w8f+KFM88LAeb5Ey+cef7Ev8w8f+JfZp4/8cKZ54UA8/yJF848f+JfZp4/8S8zz5944czzQoB5/sQLZ54/8S8zz5/4l5nnT7xw5nkhwDx/4oUzz5/4l5nnT/zLzPMnXjjzvBBgnj/xwpnnT/zLzPMn/mXm+RMvnHleCDDPn3jhzPMn/mXm+RP/MvP8iRfOPC8EmOdPvHDm+RP/MvP8iX+Zef7EC2eeFwLM8ydeOPP8iX+Zef7Ev8w8f+KFM88LAeb5Ey+cef7Ev2wXOMZzegbwYP5l5vkTL5x5Xggwz5944czzJ/5lnw18Fs/pc4DP5l9mnj/xwpnnhQDz/IkXzjx/4kXz1cBbc8V3A5/Ni8Y8f+KFM88LAeb5Ey+cef7Efy7z/IkXzjwvBJjnT7xw5vkT/7nM8ydeOPO8EGCeP/HCmedP/Ocyz5944czzQoB5/sQLZ54/8Z/LPH/ihTPPCwHm+RMvnHn+xH8u8/yJF848LwSY50+8cLvAMZ7TM4AH85/LPH/ihTPPCwHm+RMv3GcDn8Vz+hzgs/nPZZ4/8cKZ54UA8/yJf9lXA2/NFd8NfDb/+czzJ14487wQYJ4/8T+Tef7EC2eeFwLM8yf+ZzLPn3jhzPNCgHn+xP9M5vkTL5x5Xggwz5/4n8k8f+KFM88LAeb5E/8zmedPvHDmeSHAPH/if6Zd4BjP6RnAg3nhzPNCgHn+xP9Mnw18Fs/pc4DP5oUzzwsB5vkT/3N9NfDWXPHdwGfzLzPPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8vQzw1/zf8NLAX/G8EPDXwEvx/9PfCPhs4LP4/+lzBBwHbgWO8f/LJeDB4or3Br6L/1/eBvhp8WzvDXw1cIz/2y4B7w38NIB4TseBjwbeGngp/m/5G+Cnga8GdrmCfwQFjuzMCjoeoQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoProject;
impl IconShape for GoProject {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4z/NVwHtzxXcDH8P/PIj/HF8NfBTP6WuAj+Z/FsR/vOPARZ6/E8Au/3Mg/uN9NPBVPH8fA3w1/3Mg/uM9HXgwz9+twEP4nwPxH+u1gd/ihXsd4Lf5nwHxH+u7gffihfse4L35nwHxH+c4cJEXzQlgl/9+iP84Hw18FS+ajwG+mv9+iP84TwcezIvmVuAh/PdD/Md4beC3+Nd5HeC3+e+F+I/x3cB78byewRUP4nl9D/De/PdC/PsdBy7y/H0MV3wVz98JYJf/Poh/v48Gvorn7wRXXOT5+xjgq/nvg/j3ezrwYJ7X9wDvzRXfDbwXz+tW4CH890H8+7w28Fs8f68D/DZXvDbwWzx/rwP8Nv89EP8+3w28F8/rGcCDeU63Ag/ieX0P8N7890D82x0HLvL8fQzw1Tynjwa+iufvBLDLfz3Ev91HA1/F83cC2OU5HQcu8vx9DPDV/NdD/Ns9HXgwz+t7gPfm+ftu4L14XrcCD+G/HgKOA58FvDdwnH+/1wF+m+fvtYHf4vl7HeC3edF9FfDewHH+9W4Fvhv4HAG/Bbw2/zGeATyYF+5W4EE8r+8B3psXzVcDH8W/39cIMP9xPgb4al64jwa+iufvBLDLC3ccuMh/jFsF7ALH+I9xAtjlhTsOXOT5+xjgq3nhPhr4Kv5jPEPAZwOfxb/f1wAfzYvmu4H34nndCjyEF+7pwIP5j/E54oqvBt4aeBD/epeA7wY+mhfdawO/xfP3OsBv8/y9NvBb/Ps9A/hu4LPFf59bgQfxvL4HeG+ev+8G3ovn9QzgwfzrIf77fDTwVTx/J4BdntNx4CLP38cAX82/HuK/z3HgIs/fxwBfzXP6aOCreP5OALv86yH+e3038F48r1uBh/Ccng48mOf1PcB782+D+O/12sBv8fy9DvDbXPHawG/x/L0O8Nv82yD++90KPIjn9T3Ae3PFdwPvxfN6BvBg/u0Q//0+Gvgqnr8TXHGR5+9jgK/m3w7x3+84cJHn72O44qt4/k4Au/zbIf5n+G7gvXhet3LFg3le3wO8N/8+iP8ZXhv4Lf51Xgf4bf59EP9z3Ao8iBfNM4AH8++H+J/jo4Gv4kXzMcBX8++H+J/jOHCRF80JYJd/P8T/LN8NvBcv3PcA781/DMT/LK8N/BYv3OsAv81/DMT/PLcCD+L5ewbwYP7jIP7n+Wjgq3j+3gf4bv7jIP7nOQ5c5HldAh4M7PIfB/E/01cDH8Vz+hzgs/mPhfif67OBj+aKrwY+m/94iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPCsiVENWW4EkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoPulse;
impl IconShape for GoPulse {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6 2a.75.75 0 01.696.471L10 10.731l1.304-3.26A.75.75 0 0112 7h3.25a.75.75 0 010 1.5h-2.742l-1.812 4.528a.75.75 0 01-1.392 0L6 4.77 4.696 8.03A.75.75 0 014 8.5H.75a.75.75 0 010-1.5h2.742l1.812-4.529A.75.75 0 016 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoQuestion;
impl IconShape for GoQuestion {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADAUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvieR0HPgt4a+DB/O9wK/DdwOfwr4N4Xl8NfBT/O30N8NG86BDP6yJwnP+dbgUewosO8bx2gWP87/QM4MG86BDP67OBz+J/p88BPpsXHeL5+2rgrYEH8b/DM4DvBj6bfx3E/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Qzyv48BnAW/NFd8NfA7/NyGe11cDH8Vz+hrgo/m/B/G8LgLHeU63Ag/hRXMc+CzgrYEH8z/DrcB3A5/Dc0I8L/P8iRfNVwMfxf9MXwN8NM+GeF7m+RMvmovAcf5nuhV4CM+GeF7m+RMvml3gGP8zPQN4MM+GeF7m+RMvms8GPov/mT4H+GyeDfG8zPMnXnRfDbw18CD+Z3gG8N3AZ/OcEM/LPH/i/x7E8zLPn/i/B/G8zPMn/u9BPC/z/In/exDPyzx/4v8exPMyz5/4vwfxvMzzJ/7vQTwv8/yJ/3sQz8s8f+L/HsTzMs+f+L8H8bzM8yf+70E8L/P8if97EM/LPH/i/x7E8zLPn/i/B/G8zPMnXjTHgc8C3hp4MP8z3Ap8N/A5PCfE8zLPn3jRfDXwUfzP9DXAR/NsiOdlnj/xorkIHOd/pluBh/BsiOdlnj/xotkFjvE/0zOAB/NsiOdlnj/xovls4LP4n+lzgM/m2RDPaxc4xnN6BvBgXnRfDbw18CD+Z3gG8N3AZ/OcEM/rs4HP4jl9DvDZ/N+DeP6+Gnhrrvhu4LP5vwnx/xvi/zfE/2+I/98Q/78h/n/jHwG8jE5BT4KiwAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoQuote;
impl IconShape for GoQuote {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 2.5a.75.75 0 000 1.5h10.5a.75.75 0 000-1.5H1.75zm4 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM2.5 7.75a.75.75 0 00-1.5 0v6a.75.75 0 001.5 0v-6z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEKElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P85/os4KO54quBz+F/FsR/nu8C3pvn9DnAZ/M/B+I/x3cB783zuhV4CP9zIP7jfRfw3jx/zwAezP8ciP9Y3wW8Ny/Y5wCfzf8ciP843wW8Ny/Y9wDvzf8siP8Y3wW8Ny/Y9wDvzf88iH+/7wLemxfse4D35n8mxL/PdwHvzQv2PcB78z8X4t/uu4D35gX7HuC9+Z8N8W/zXcB784J9D/De/M+H+Nf7LuC9ecG+B3hv/ndA/Ot8F/DevGDfA7w3/3sgXnTfBbw3L9j3AO/N/y6IF813Ae/NC/Y9wHvzvw/iX/ZdwHvzgn0P8N7874R44b4LeG9esO8B3pv/vRAv2HcB783/HH8N7AK7wE8DvwPcyr8P4vn7bOCz+J/vr4HvBr6GfxvE83cROM7/HrcCnw18D/86iOdvFzjG/z5/DbwNcCsvGsTz99nAZ/G/0y7wOsBf8y9DvGDfDbwX/3u9D/DdvHCIF+67gffiBftu4H34r/HSwHHgrYG3Bh7Ev+xlgL/mBUP8y74beC9esO8G3of/eq8NfDbwWrxgu8DLALfy/CFeNN8NvBcv2HcD78N/j48GvooX7K+Bl+H5Q7zovht4L16w7wbeh/8erw38NHCM5+99gO/meSH+db4beC9esO8G3of/Hq8N/BbP363AQ3heiH+97wbeixfsu4H34b/HRwNfxfP3McBX85wQ/zbfDbwXL9h3A+/Df4/fBl6L5/XXwMvwnBD/dt8NvBcv2HcD78N/vdcGfovn7yHArTwb4t/nu4H34gX7buB9+K93K/Agntf7AN/NsyH+/b4beC9esO8G3of/Wl8NfBTP62eAt+bZEP8xvht4L16w7wbeh/86rw38Fs/rd4DX5tkQ/3G+G3gvXrDvBt6H/xovDfwVz+uvgZfh2RD/sb4beC9esK8BPpr/Gub5E8+G+I/33cB78YKJ/xrm+RPPhvjP8d3Ae/H8if8a5vkTz4b4z/PdwHvxnH4GeGv+a5jnTzwb4j/XVwMfxRXfA3w0sMt/DfP8iWdD/N9lnj/xbIj/u8zzJ54N8X+Xef7EsyH+7zLPn3g2xP9d5vkTz4b4v8s8f+LZEP937QLHeE7PAB7MsyH+7/ps4LN4Tp8DfDbPhvi/7auBt+aK7wY+m+eE+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/xj8C+y2BQc56xDkAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoReply;
impl IconShape for GoReply {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.78 1.97a.75.75 0 010 1.06L3.81 6h6.44A4.75 4.75 0 0115 10.75v2.5a.75.75 0 01-1.5 0v-2.5a3.25 3.25 0 00-3.25-3.25H3.81l2.97 2.97a.75.75 0 11-1.06 1.06L1.47 7.28a.75.75 0 010-1.06l4.25-4.25a.75.75 0 011.06 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEhklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3ey/gtYGXBl6a/1luBb4b+BxeOMS/3lsDXwU8mP/5vgb4aF4wxL/OVwEfzf8etwIP4QVDvOi+Gvgo/vcRLxjiRfPWwE/xv5N4wRAvmqcDD+Z/J/GCIf5l7w18F8/fM4CPBv4auJX/Xub5Ey8Y4l/23cB78byeAbw0sMv/DOb5Ey8Y4l/2V8BL87zeBvhp/ucwz594wRD/MvP8nQB2+Z/DPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH/ifxbz/IkXDPEvM8+f+J/FPH/iBUP8y8zzJ/5nMc+feMEQ/zLz/In/WczzJ14wxL/MPH9vDbw18NLAS/M/l3jBEP8y87+beMEQ/zLzv5t4wRD/MvO/m3jBEP8y87+beMEQ/zLzv5t4wRD/MvOC/Q3w1cBPA7v89zLPn3jBEP8y8/w9A3hpYJf/GczzJ14wxL/MPH9vA/w0/3OY50+8YIh/mXn+TgC7/M9hnj/xgiH+Zeb5E/+zmOdPvGCIf9lfAy/F83ob4Kf5n8M8f+IFQ/zLvht4L57XrcDLALs8p78CXpp/m78GXoZ/G/P8iRcM8S97b+C7eP6+BvhonpP59xH/Nub5Ey8Y4kVzK/AgntcucILnZP59xHM6DnwW8NbAg/nXEy8Y4kXz1sBP8byeATyY52T+fcRz+mrgo/i3Ey8Y4kX31cBH8Zw+B/hsnpP59xHP6SJwnH878YIh/nU+Gnhrrvhp4Kt5XubfRzynXeAY/zbPAB7MC4b4j2f+fcRz+mzgs/i3+Rzgs3nBEP/xzL+PeF5fDbw18CBeNM8Avhv4bF44xH888+8j/usg/uOZfx/xXwfxH888f5eAz+aKzwaO8fyJ/zqI/3jmef0N8N7AX3PFSwPfDbwUz0v810H8xzPP6XuAjwZ2eU7Hga8G3ovnJP7rIP7jmSsuAR8NfDcv3HsDXw0c4wrxXwfxH++vueK9gb/mRfPSwHdzxUvzXwfx/xv/CIclnkFPebXZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepo;
impl IconShape for GoRepo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjiwcBXAS8NPJj/WuI5mf9ctwJ/DXwMcKuABwN/BRznv4d4Tua/xi7wMgJ+Gngr/vuI52T+6/yMgIvAcf77iOdk/uvcKsC8cOK/lnnhxL+PeTYEmBdO/NcyL5z49zHPhgDzwon/WuaFE/8+5tkQYF448V/LvHDi38c8GwLMCyeek3lO4j+WeeHEv495NgSYF048J/OcxH8s88KJfx/zbAgwL5x4TuY5if9Y5oUT/z7m2RBgXjjxnMxzEv+xzAsn/n3MsyHAvHDiv5Z54cS/j3k2BJgXTvzXMi+c+Pcxz4YA88KJ/1rmhRP/PubZEGBeOPFfy7xw4t/HPBsCzAsnnpN5TuI/lnnhxL+PeTYEmBdOPCfznMR/LPPCiX8f82wIMC+ceE7mOYn/WOaFE/8+5tkQYF448ZzMcxL/scwLJ/59zLMhwLxw4r+W+a+DAPPCif9a5r8OAswLJ/5rmf86CDAvnPivZf7rIMC8cOI5meck/mOZ5ySek3lO4l/HPBsCzAsnnpN5TuI/lnlO4jmZ5yT+dcyzIcC8cOI5meck/mOZ5ySek3lO4l/HPBsCzAsnnpN5TuI/lnlO4jmZ5yT+dcyzIcC8cOK/1i5wjP8azxBgXjjxX+ungbfiv8bPCDAvnPiv9WDgr4Fj/Oe6BLy0APPCif96Dwa+Gnhp4EH8x3oG8NfARwO3CjAvnHhO5jmJ/1rmOYl/OwSYF048J/OcxH8t85zEvx0CzAsnnpN5TuK/lnlO4t8OAeaFE8/JPCfxX8s8J/FvhwDzwon/uxBgXjjxfxcCzAsn/u9CgHnhxP9dCDD/u4l/OwSY/93Evx0CzP9u4t8OAeZ/N/FvhwDznMT/bOY5iX87BJjnJP5nM89J/NshwDwn8T+beU7i3w4B5jmJ/9nMcxL/dggwz0m8cOZfR/zHMs9J/NshwDwn8cKZfx3xH8s8J/FvhwDznMQLZ/51xH8s85zEvx0CzHMSL5z51xH/scxzEv92CNgFjvG/0zOAB/Nvh4CfBt6K/51+Bnhr/u0Q8GDgr4Fj/O9yCXhp4Fb+7RBXPBj4auClgQfxP9szgL8GPhq4lX8fxL+e+dcR/3Mh/vXMv474nwvxr2ee1/cAvw18F89L/M+F+Nczz+l7gPfmivcGvovnJP7nQvzrmWf7HuC9eU7vDXwXzyb+50L865krvgd4b56/9wa+iyvE/1yIfz0D3wO8Ny/cewPfBYj/uRD/et8NvDcvmvcGvpv/ufhHmw61iGbKqg0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoClone;
impl IconShape for GoRepoClone {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M15 0H9v7c0 .55.45 1 1 1h1v1h1V8h3c.55 0 1-.45 1-1V1c0-.55-.45-1-1-1zm-4 7h-1V6h1v1zm4 0h-3V6h3v1zm0-2h-4V1h4v4zM4 5H3V4h1v1zm0-2H3V2h1v1zM2 1h6V0H1C.45 0 0 .45 0 1v12c0 .55.45 1 1 1h2v2l1.5-1.5L6 16v-2h5c.55 0 1-.45 1-1v-3H2V1zm9 10v2H6v-1H3v1H1v-2h10zM3 8h1v1H3V8zm1-1H3V6h1v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFWElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjRvRfw2sBLAy/N/yy3At8NfA7/Ooh/2VsDXwU8mP/5vgb4aF50iBfuq4CP5n+PW4GH8KJDvGBfDXwU//uIFx3i+Xtr4Kf430m86BDP39OBB/O/k3jRIZ7XewPfxfP3DOCjgb8GbuW/l3n+xIsO8by+G3gvntczgJcGdvmfwTx/4kWHeF5/Bbw0z+ttgJ/mfw7z/IkXHeJ5mefvBLDL/xzm+RMvOsTzMs+f+J/FPH/iRYd4Xub5E/+zmOdPvOgQz8s8f+J/FvP8iRcd4nmZ50/8z2KeP/GiQzwv8/yJ/1nM8ydedIjnZZ4/8T+Lef7Eiw7xvMzzJ/5nMc+feNEhnpd5/sT/LOb5Ey86xPMyz5/4n8U8f+JFh3he5vkT/7OY50+86BDPyzx/4n8W8/yJFx3ieZnnT/zPYp4/8aJDPC/z/In/WczzJ150iOdlnj/xP4t5/sSLDvG8zPMn/mcxz5940SGel3n+xP8s5vkTLzrE8zLPn/ifxTx/4kWHeF7m+RP/s5jnT7zoEM/LPH/ifxbz/IkXHeJ5medP/M9inj/xokM8L/P8if9ZzPMnXnSI52WeP/E/i3n+xIsO8bzM8yf+ZzHPn3jRIZ6Xef7E/yy7wDGe0zOAB/OiQzwv8/yJ/1k+G/gsntPnAJ/Niw7xvMzzJ/7n+Wrgrbniu4HP5l8H8bzM8/fWwFsDLw28NP+xbgW+G/gc/mshnpf57/M1wEfzXwfxvMx/n1uBh/BfB/G8zH+fZwAP5r8O4nmZ/z6fA3w2z+k48F3AW3PFVwMfw7/NVwEfzRU/DbyPeF7mv94zgO8GPpvn9dPAW/Gcvht4H/51vgt4b57Tz4jnZV6wZwAfDfw1cCv/Nczz993A+/Ci+S7gvXleiOdlnr9nAC8N7PJfy7xg3w28Dy/cdwHvzfOHeF7m+Xsb4Kf5r/fVwEfxgn038D48f98FvDcv2NeI52WevxPALv89vht4L16w7wbeh+f0XcB784J9D/De4nmZ5+8EsMt/n+8G3osX7LuB9+GK7wLemxfse4D3BhDP66+Bl+J5vQ3w0/z3+m7gvXjBvpsr3psX7HuA9+YKxPP6buC9eF63Ai8D7PLf67uB9+Lf5nuA9+bZEM/rvYHv4vm7Ffho4G+AW/nv893Ae/Gv8z3Ae/OcEM/frcCD+K9zK/DdwOfwovtu4L140XwP8N48L8Tz99bAT/Ff72uAj+ZF993Ae/HCfQ/w3jx/iBfsq4GP4r/WrcBDeNF9F/DevHDfDbwPzx/ihftq4KP4r/MM4MG8aL4LeG9eNN8NvA/PC/Eve2vgq4EH8Z/vc4DP5l/2XcB786/z3cD78JwQL7q3Bt4aeGngpfiP9Qzgu4HP5l/2XcB782/z3cD78GyI/12+C3hvXrDv4Yr34gX7buB9uALxv8d3Ae/NC/Y9wHtzxXcD78UL9t3A+wCI/x2+C3hvXrDvAd6b5/TdwHvxgn038D7if76vBj6KF+x7gPfm+ftu4L14wb5G/M9nXrDvAd6bF+67gffi+UP8z2eev+8B3psXzXcD78XzQvzP99PAW/Gcvgd4b/51vht4L57Tz4j/+Y4D3w28FVd8DfDR/Nt8NfBRXPE9wEf/IwX2y91/eQbNAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoDeleted;
impl IconShape for GoRepoDeleted {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v7.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h4.75a.75.75 0 010 1.5H3.5a1 1 0 100 2h4.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9z",
            }
            path {
                d: "M11.28 10.22a.75.75 0 10-1.06 1.06L11.94 13l-1.72 1.72a.75.75 0 101.06 1.06L13 14.06l1.72 1.72a.75.75 0 101.06-1.06L14.06 13l1.72-1.72a.75.75 0 10-1.06-1.06L13 11.94l-1.72-1.72z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoForked;
impl IconShape for GoRepoForked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjRvRfw2sBLAy/N/yy3At8NfA7/Ooh/2VsDXwU8mP/5vgb4aF50iBfuq4CP5n+PW4GH8KJDvGBfDXwU//uIFx3i+Xtr4Kf430m86BDP39OBB/O/k3jRIZ7XewPfxfP3DOCjgb8GbuW/l3n+xIsO8by+G3gvntczgJcGdvmfwTx/4kWHeF5/Bbw0z+ttgJ/mfw7z/IkXHeJ5mefvBLDL/xzm+RMvOsTzMs+f+J/FPH/iRYd4Xub5E/+zmOdPvOgQz8s8f+J/FvP8iRcd4nmZ50/8z2KeP/GiQzwv8/yJ/1nM8ydedIjnZZ4/8T+Lef7Eiw7xvMzzJ/5nMc+feNEhnpd5/sT/LLvAMZ7TM4AH86JDPC/z/In/WT4b+Cye0+cAn82LDvG8zPMn/uf5auCtueK7gc/mXwfxvMzzJ/7vQTwv8/yJ/3sQz8s8f+L/HsTzMs+f+L8H8bzM8yf++z0YeCvgtYEHAy/NFX8N3Ar8NvAzwK28aBDPyzx/4r/Pg4HPAt6bF813A58D3MoLh3he5vkT/z3eGvgu4Dj/OrvA+wA/zQuGeF7m+RP/9d4b+C7+fd4H+G6eP8TzMs+f+K/11sBP8R/jbYCf5nkhnpd5/sR/nQcDfwUc5z/GLvAywK08J8TzMs+f+K/z3cB78YJ9DfDdwF9zxUsDHw28Fy/Y9wDvzXNCPC/z/In/Gg8Gns7zdwl4beCvef5eGvht4BjP30OAW3k2xPMyz5/4r/HRwFfx/L0M8Ne8cC8N/BXP38cAX82zIZ6Xef7Ef42fBt6K5/U1wEfzovlu4L14Xj8DvDXPhnhe5vkT/zX+CnhpntfLAH/Ni+algb/ief018DI8G+J5medP/Ncwz5/41zHPn3g2xPMyz99bA28NvDTw0vzr3Ap8N/A5/MvM8yf+dczzJ54N8bzMf56vAT6aF848f+Jfxzx/4tkQz8v857kVeAgvnHn+xL+Oef7EsyGel/nP8wzgwbxw5vkT/zrm+RPPhnhe5j/P5wCfzQt2HLjIf64TwC5XIJ6X+Y/3DOC7gc/mBXtr4LuA4/zn2gXeB/hpAPG8zAv2DOCjgb8GbuU/zlsDP8V/rbcBflo8L/P8PQN4aWCX/1jHgacDx/mvtQs8RDwv8/y9DfDT/Mf7bOCz+O/xOeJ5mefvBLDLf7zfBl6L/x6/I56Xef5OALv8xzP/fXbF8/pr4KV4Xm8D/DT/8cx/H8Tz+m7gvXhetwIvA+zyH8v890E8r/cGvovn71bgo4G/AW7lP4b574N4/m4FHsR/nFuB7wY+h+dl/vsgnr+3Bn6K/3hfA3w0z8n890G8YF8NfBT/sW4FHsJzMv99EC/cVwMfxX+cZwAP5jmZ/z6If9lbA18NPIh/v88BPpvnZP77IF50bw28NfDSwEvxr/MM4LuBz+Z5mf8+iP9+5r8P4r+f+e+D+O/318BL8d/jb8R/v88GPov/Hp8j/vsdB24FjvFf6xLwYPE/w3sD38V/rbcBflr8z/HewFcDx/jPdQl4b+CnAcT/LMeBjwbeGngp/mP9DfDTwFcDu1zBPwJL9czVeN0w4QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoLocked;
impl IconShape for GoRepoLocked {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h2.75a.75.75 0 010 1.5H3.5a1 1 0 100 2h2.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9z",
            }
            path {
                d: "M9 10.168V9a3 3 0 116 0v1.168c.591.281 1 .884 1 1.582v2.5A1.75 1.75 0 0114.25 16h-4.5A1.75 1.75 0 018 14.25v-2.5c0-.698.409-1.3 1-1.582zM13.5 10h-3V9a1.5 1.5 0 013 0v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjiwcBXAS8NPJj/2W4F/hr4GOBW/n0Q8GDgr4Dj/O+yC7wMcCv/dgj4aeCt+N/pZ4C35t8OAReB4/zvdCvwEP7tEGBeOPE/i3lO4t8OAeaFE/+zmOck/u0QYF448T+LeU7i3w4B5oUT/7OY5yT+7RBgXjjxnMxzEv+1zHMSz+m7gPfhRYMA88KJ52Sek/ivZZ6TeE4Gvht4H/5lCDAvnHhO5jmJ/1rmOYnnZK74buB9eOEQYF448ZzMcxL/tcxzEs/JPNt3A+/DC4YA88KJ/13Mc/pu4H14/hBgXjjxv4t5Xt8NvA/PCwHmhRP/u5jn77uB9+E5IcC8cOJ/F/OCfTfwPjwbAswLJ56TeU7iX8f89/pu4H24AgHmhRPPyTwn8a9j/vt9N/A+AALMCyeek3lO4l/H/M/w3cD7CDAvnHhO5jmJfx3zP8d3CzAvnPiPZf7n+B4B5oUT/7HM/wzfA7y3APPCif9Y5r/f9wDvDSDAvHDiP5b57/U9wHtzBQLMCyeek3lO4n8W84J9D/DePBsCzAsnnpN5TuJ/FvP8fQ/w3jwnBJgXTjwn85zE/yzmeX0P8N48LwSYF048J/OcxP8s5jl9D/DePH8IMC+c+J/FPCfxnMyzfQ/w3rxgCDAvnPifxTwn8ZzMFd8DvDcvHALMCyf+ZzHPSTwnA98DvDf/MgSYF078z2Kek3hO3w28Ny8aBJgXTjwn85zEfy3znMS/HQLMCyeek3lO4r+WeU7i3w4B5oUTz8k8J/Ffyzwn8W+HAPPCiedknpP4r2Wek/i3Q4B54cT/LOY5iX87BJgXTvzPYp6T+LdDgHnhxP8s5jmJfzsEmBdO/M9inpP4t0OA+d9N/NshwPzvJv7tEGD+dxP/dggw/7uJfzsEmOck/mczz0n82yHAPCfxP5t5TuLfDgHmOYn/2cxzEv92CDDPSfzPZp6T+LdDgHlO4oUz/zriP5Z5TuLfDgHmOYkXzvzriP9Y5jmJfzsEmOckXjjzryP+Y5nnJP7tEGCek3jhzL+O+I9lnpP4t0PALnCM/52eATyYfzsE/DTwVvzv9DPAW/Nvh4AHA38NHON/l0vASwO38m+HuOLBwFcDLw08iP/ZngH8NfDRwK38+yD+9cy/jvifC/GvZ/51xP9ciH8987y+B/ht4Lt4XuJ/LsS/nnlO3wO8N1e8N/BdPCfxPxfiX8882/cA781zem/gu3g28T8X4l/PXPE9wHvz/L038F1cIf7nQvzrGfge4L154d4b+C5A/M+F+Nf7buC9edG8N/Dd/M/FPwJTvLaAXAVSgAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoPull;
impl IconShape for GoRepoPull {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13 8V6H7V4h6V2l3 3-3 3zM4 2H3v1h1V2zm7 5h1v6c0 .55-.45 1-1 1H6v2l-1.5-1.5L3 16v-2H1c-.55 0-1-.45-1-1V1c0-.55.45-1 1-1h10c.55 0 1 .45 1 1v2h-1V1H2v9h9V7zm0 4H1v2h2v-1h3v1h5v-2zM4 6H3v1h1V6zm0-2H3v1h1V4zM3 9h1V8H3v1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonjRvRfw2sBLAy/N/yy3At8NfA7/Ooh/2VsDXwU8mP/5vgb4aF50iBfuq4CP5n+PW4GH8KJDvGBfDXwU//uIFx3i+Xtr4Kf430m86BDP39OBB/O/k3jRIZ7XewPfxfP3DOCjgb8GbuW/l3n+xIsO8by+G3gvntczgJcGdvmfwTx/4kWHeF5/Bbw0z+ttgJ/mfw7z/IkXHeJ5mefvBLDL/xzm+RMvOsTzMs+f+J/FPH/iRYd4Xub5E/+zmOdPvOgQz8s8f+J/FvP8iRcd4nmZ50/8z2KeP/GiQzwv8/yJ/1nM8ydedIjnZZ4/8T+Lef7Eiw7xvMzzJ/5nMc+feNEhnpd5/sT/LLvAMZ7TM4AH86JDPC/z/In/WT4b+Cye0+cAn82LDvG8zPMn/uf5auCtueK7gc/mXwfxvMzzJ/7vQTwv8/yJ/3sQz8s8f+L/HsTzMs+f+L8H8bzM8yf+70E8L/P8if97EM/LPH/if57PAt6bK74b+Bz+dRDPyzx/4n+W7wLem+f03cD78KJDPC/z/In/Ob4LeG+ev+8G3ocXDeJ5medP/M/wXcB788J9N/A+/MsQz8s8f+K/33cB782L5ruB9+GFQzwv8/yJ/17fBbw3/zrfDbwPLxjieZnnT/z3+S7gvfm3+W7gfXj+EM/LPH/iv8d3Ae/Nv893A+/D80I8L/P8if963wW8N/8xvht4H54T4nmZ50/81/ou4L35j/XdwPvwbIjnZZ6/twbeGnhp4KX5t7kV+G7gc3jhvgt4b16wvwFeiufve4D34gX7buB9uALxvMx/vq8BPprn77OBz+IF+xvgtYGLPH8Cvht4L16wzwE+G0A8L/Of71bgITx/TwcezPP3N8BrA7uAef7EFd8NvBfP363AQwDE8zL/+Z4BPJjn71bgQTyvvwFeG9jlCvP8iWf7buC9eF7PAB4MIJ6X+c/3OcBn8/x9NvBZPKe/AV4b2OXZzPMnntN3A+/Fc/oc4LMBxPMy/3meAXw38Nm8cJ8NvDdwHPhu4LOBXZ6Tef7E8/ps4L254ruBz+YKxPMyL9gzgI8G/hq4lf9e5vkTLzrE8zLP3zOAlwZ2+Z/BPH/iRYd4Xub5exvgp/mfwzx/4kWHeF7m+TsB7PI/h3n+xIsO8bzM83cC2OV/DvP8iRcd4nn9NfBSPK+3AX6a/znM8ydedIjn9d3Ae/G8bgVeBtjlfwbz/IkXHeJ5vTfwXTx/twIfDfwNcCv/vczzJ150iOfvVuBB/Pe5Ffhu4HN4wczzJ150iOfvrYGf4r/f1wAfzfNnnj/xokO8YF8NfBT/vW4FHsLzZ54/8aJDvHBfDXwU/32eATyY5888f+JFh/iXvTXw1cCD+K/3OcBn8/yZ50+86BAvurcG3hp4aeCl+M/1DOC7gc/mBTPPn3jRIf73Ms+feNEh/vcyz5940SH+9zLPn3jRIf73Ms+feNEh/vcyz5940SH+97oVeBDP6RJwnBcd4n+vzwY+i+f0NcBH86JD/O/22cB7A8eB7wY+G9jlRcc/AoZTwkGkI5TSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoPush;
impl IconShape for GoRepoPush {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1 2.5A2.5 2.5 0 013.5 0h8.75a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0V1.5h-8a1 1 0 00-1 1v6.708A2.492 2.492 0 013.5 9h3.25a.75.75 0 010 1.5H3.5a1 1 0 100 2h5.75a.75.75 0 010 1.5H3.5A2.5 2.5 0 011 11.5v-9zm13.23 7.79a.75.75 0 001.06-1.06l-2.505-2.505a.75.75 0 00-1.06 0L9.22 9.229a.75.75 0 001.06 1.061l1.225-1.224v6.184a.75.75 0 001.5 0V9.066l1.224 1.224z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRepoTemplate;
impl IconShape for GoRepoTemplate {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6 .75A.75.75 0 016.75 0h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 016 .75zm5 0a.75.75 0 01.75-.75h1.5a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0V1.5h-.75A.75.75 0 0111 .75zM4.992.662a.75.75 0 01-.636.848c-.436.063-.783.41-.846.846a.75.75 0 01-1.485-.212A2.501 2.501 0 014.144.025a.75.75 0 01.848.637zM2.75 4a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 012.75 4zm10.5 0a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5a.75.75 0 01.75-.75zM2.75 8a.75.75 0 01.75.75v.268A1.72 1.72 0 013.75 9h.5a.75.75 0 010 1.5h-.5a.25.25 0 00-.25.25v.75c0 .28.114.532.3.714a.75.75 0 01-1.05 1.072A2.495 2.495 0 012 11.5V8.75A.75.75 0 012.75 8zm10.5 0a.75.75 0 01.75.75v4.5a.75.75 0 01-.75.75h-2.5a.75.75 0 010-1.5h1.75v-2h-.75a.75.75 0 010-1.5h.75v-.25a.75.75 0 01.75-.75zM6 9.75A.75.75 0 016.75 9h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 016 9.75zm-1 2.5v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+b/hpcG/ornhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/znOQ58FvDWXPHdwOfwn8s8LwSY50/85/lq4KN4Tl8DfDT/eczzQoB5/sR/novAcZ7TrcBD+M9jnhcCzPMn/vOY50/85zHPCwHm+RP/eczzJ/7zmOeFAPP8if885vkT/3nM80KAef7Efx7z/In/POZ5IcA8f+I/j3n+xH8e87wQYJ4/8Z/HPH/iP495Xggwz5/4z2OeP/GfxzwvBJjnT/znMc+f+M9jnhcCzPMn/vOY50/85zHPCwHm+RP/eczzJ/7zmOeFAPP8if88u8AxntMzgAfzn8c8LwSY50/85/ls4LN4Tp8DfDb/eczzQoB5/sR/rq8G3porvhv4bP5zmeeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+M/zYOCrgJfmir8GPga4lf885nkhwDx/4j/Hg4G/Ao7znHaBlwFu5T+HeV4IMM+f+M/x08Bb8fz9DPDW/OcwzwsB5vkT/zkuAsd5/m4FHsJ/DvO8EGCeP/GfYxc4xvP3DODB/OcwzwsB5vkT/zl+Gngrnr+fAd6a/xzmeSHAPH/iP8eDgb8GjvGcLgEvDdzKfw7zvBBgnj/xn+fBwFcDL80Vfw18NHAr/3nM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8/cywF/zf8NLA3/F80LAXwMvxb/NrcB3A5/Dv85nAR8NHOe/198I+Gzgs/j3+Rrgo3nRfBfw3vzP8DkCjgO3Asf4t7sVeAj/su8C3pv/GS4BDxZXvDfwXfz7iBfuu4D35n+OtwF+WjzbewNfDRzj30a8YN8FvDf/M1wC3hv4aQDxnI4DHw28NfBS/OuI5++7gPfmv9/fAD8NfDWwyxWIfz3z/Inn9V3Ae/OCfQ/w3vz3QfzrmedPPKfvAt6bF+x7gPfmvxfiX888f+LZvgt4b16w7wHem/9+iH898/yJK74LeG9esO8B3pv/GRD/eub5E/BdwHvzgn0P8N78z4H41zPP33cD780L9j3Ae/M/C+Jfz/zrfQ/w3vzPg/jX+23gtXjRfQ/w3vzPhPjX+2zgs3jRfA/w3vzPhfjXOw78NvBSvHDfA7w3/7Mh/m0eDPw28CCev+8B3pv/+RD/Pp8NvDXwUlzxM8B3Az/N/w78Iy1SwdgemiCnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoReport;
impl IconShape for GoReport {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 1.5a.25.25 0 00-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 01.75.75v2.19l2.72-2.72a.75.75 0 01.53-.22h6.5a.25.25 0 00.25-.25v-9.5a.25.25 0 00-.25-.25H1.75zM0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0114.25 13H8.06l-2.573 2.573A1.457 1.457 0 013 14.543V13H1.75A1.75 1.75 0 010 11.25v-9.5zM9 9a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRocket;
impl IconShape for GoRocket {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M14.064 0a8.75 8.75 0 00-6.187 2.563l-.459.458c-.314.314-.616.641-.904.979H3.31a1.75 1.75 0 00-1.49.833L.11 7.607a.75.75 0 00.418 1.11l3.102.954c.037.051.079.1.124.145l2.429 2.428c.046.046.094.088.145.125l.954 3.102a.75.75 0 001.11.418l2.774-1.707a1.75 1.75 0 00.833-1.49V9.485c.338-.288.665-.59.979-.904l.458-.459A8.75 8.75 0 0016 1.936V1.75A1.75 1.75 0 0014.25 0h-.186zM10.5 10.625c-.088.06-.177.118-.266.175l-2.35 1.521.548 1.783 1.949-1.2a.25.25 0 00.119-.213v-2.066zM3.678 8.116L5.2 5.766c.058-.09.117-.178.176-.266H3.309a.25.25 0 00-.213.119l-1.2 1.95 1.782.547zm5.26-4.493A7.25 7.25 0 0114.063 1.5h.186a.25.25 0 01.25.25v.186a7.25 7.25 0 01-2.123 5.127l-.459.458a15.21 15.21 0 01-2.499 2.02l-2.317 1.5-2.143-2.143 1.5-2.317a15.25 15.25 0 012.02-2.5l.458-.458h.002zM12 5a1 1 0 11-2 0 1 1 0 012 0zm-8.44 9.56a1.5 1.5 0 10-2.12-2.12c-.734.73-1.047 2.332-1.15 3.003a.23.23 0 00.265.265c.671-.103 2.273-.416 3.005-1.148z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADBElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszz9zLAX/N/w0sDf8XzQsBfAy/F/09/I+Czgc/i/6fPEXAcuBU4xv8vl4AHiyveG/gu/n95G+CnxbO9N/DVwDH+b7sEvDfw0wDiOR0HPhp4a+Cl+L/lb4CfBr4a2OUKxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8RzOg58FPDWwEvzf8tfAz8NfA2wyxWIZ3tv4KuA4/zftgt8DPDdAOKK9wa+i/9f3gf4bgHHgacDx/n/ZRd4iIDPBj6L/58+R8BfAS/N/09/LcA8fy8D/DX/N7w08Fc8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5exngr/m/4aWBv+J5IeCvgZfi/6e/EfDZwGfx/9PnCDgO3Aoc4/+XS8CDxRXvDXwX/7+8DfDT4tneG/hq4Bj/t10C3hv4aQDxnI4DHw28NfBS/N/yN8BPA18N7HIF4v83xP9viP/fEP+/8Y/6rW2nbiYvygAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRows;
impl IconShape for GoRows {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M16 2.75A1.75 1.75 0 0014.25 1H1.75A1.75 1.75 0 000 2.75v2.5A1.75 1.75 0 001.75 7h12.5A1.75 1.75 0 0016 5.25v-2.5zm-1.75-.25a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h12.5zM16 10.75A1.75 1.75 0 0014.25 9H1.75A1.75 1.75 0 000 10.75v2.5A1.75 1.75 0 001.75 15h12.5A1.75 1.75 0 0016 13.25v-2.5zm-1.75-.25a.25.25 0 01.25.25v2.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-2.5a.25.25 0 01.25-.25h12.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRss;
impl IconShape for GoRss {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.002 2.725a.75.75 0 01.797-.699C8.79 2.42 13.58 7.21 13.974 13.201a.75.75 0 11-1.497.098 10.502 10.502 0 00-9.776-9.776.75.75 0 01-.7-.798zM2 13a1 1 0 112 0 1 1 0 01-2 0zm.84-5.95a.75.75 0 00-.179 1.489c2.509.3 4.5 2.291 4.8 4.8a.75.75 0 101.49-.178A7.003 7.003 0 002.838 7.05z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoRuby;
impl IconShape for GoRuby {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.637 2.291A.75.75 0 014.23 2h7.54a.75.75 0 01.593.291l3.48 4.5a.75.75 0 01-.072.999l-7.25 7a.75.75 0 01-1.042 0l-7.25-7a.75.75 0 01-.072-.999l3.48-4.5zM4.598 3.5L1.754 7.177 8 13.207l6.246-6.03L11.402 3.5H4.598z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEBElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/Ni+5W4LuBz+Hf7zjwWcBbAw/mRffXwE8DXwPs8vwhXrD3Br4KOM6/3dcAH82/z1cDH8W/3S7wMcB387wQz997A9/Fv9+twEP497kIHOff732A7+Y5IZ7XceDpwHH+/Z4BPJh/n13gGP9+u8BDgF2eDfG8Phv4LP5jfA7w2fz7fDbwWfzH+Bzgs3k2xPP6K+Cl+fd5BvDdwGfzH+OrgbcGHsS/z18DL8OzIZ6Xef5eBvhr/nd4aeCveP7EsyGel3n+xP8u5vkTz4Z4Xub5E/+7mOdPPBvieZnnT/zvYp4/8WyI52WeP/G/i3n+xLMhnpd5/sT/Lub5E8+GeF7m+RP/u5jnTzwb4nmZ50/872KeP/FsiOdlnj/xv4t5/sSzIZ6Xef7E/y7m+RPPhnhe5vkT/7uY5088G+J57QLHeE7PAB7M/y67wDGe0zOAB/NsiOf12cBn8Zw+B/hs/nf5bOCzeE6fA3w2z4Z4/r4aeGuu+G7gs/nf6auBt+aK7wY+m+eE+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4XseBzwLemiu+G/gc/vc5DnwW8NZc8d3A5/CcEM/rq4GP4jl9DfDR/O/y1cBH8Zy+Bvhong3xvC4Cx3lOtwIP4X+Xi8BxntOtwEN4NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/y9DPDX/O/w0sBf8fyJZ0M8r78GXop/n1uB7wY+h3+/48BnAW8NPJh/n78BXppnQzyvzwY+i/8YXwN8NP8+Xw18FP8xPgf4bJ4N8byOA7cCx/j3uxV4CP8+F4Hj/PtdAh4M7PJsiOfvvYHv4t/vGcCD+ffZBY7x7/c2wE/znBAv2HsDXw0c49/uc4DP5t/ns4HP4t/uEvDewE/zvBAv3HHgo4G3Bl6KF90zgO8GPpv/GF8NvDXwIF50fwP8NPDVwC7PH+L/N8T/b4j/3xD/v/GPOwCCQZsN9p4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoScreenFull;
impl IconShape for GoScreenFull {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.75 2.5a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5C1 1.784 1.784 1 2.75 1h2.5a.75.75 0 010 1.5h-2.5zM10 1.75a.75.75 0 01.75-.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 01-1.5 0v-2.5a.25.25 0 00-.25-.25h-2.5a.75.75 0 01-.75-.75zM1.75 10a.75.75 0 01.75.75v2.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 011 13.25v-2.5a.75.75 0 01.75-.75zm12.5 0a.75.75 0 01.75.75v2.5A1.75 1.75 0 0113.25 15h-2.5a.75.75 0 010-1.5h2.5a.25.25 0 00.25-.25v-2.5a.75.75 0 01.75-.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/OMeBzwLemiu+G/gc/v2OA58FvDVXfDfwOfzHQPzH+Wrgo3hOXwN8NP8+Xw18FM/pa4CP5t8P8R/nInCc53Qr8BD+fS4Cx3lOtwIP4d8P8R/HPH/i38c8f+LfD/Efxzx/4t/HPH/i3w/xH8c8f+Lfxzx/4t8P8R/HPH/i38c8f+LfD/Efxzx/4t/HPH/i3w/xH8c8f+Lfxzx/4t8P8R/HPH/i38c8f+LfD/Efxzx/4t/HPH/i3w/xH8c8f+Lfxzx/4t8P8R/HPH/i38c8f+LfD/Ef46WBv+L5E/8+5vl7GeCv+fdBPK/jwGcBbw08mH+fvwFemn+fvwZein+fW4HvBj6H54R4Xl8NfBT/MT4H+Gz+fT4b+Cz+Y3wN8NE8G+J5XQSO8+93CXgwsMu/z3HgVuAY/363Ag/h2RDPaxc4xr/f2wA/zX+M9wa+i3+/ZwAP5tkQz+uzgc/i3+4S8N7AT/Mf672BrwaO8W/3OcBn82yI5++rgbcGHsSL7m+Anwa+GtjlP8dx4KOBtwZeihfdM4DvBj6b54T4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hnhex4HPAt4aeDAvur8Gfhr4GmCX/xzHgY8C3hp4aV50twLfDXwOzwnxvL4a+Cj+7XaBjwG+m/9Y7w18FXCcf7uvAT6aZ0M8r4vAcf793gf4bv5jvDfwXfz73Qo8hGdDPK9d4Bj/frvAQ4Bd/n2OA08HjvPv9wzgwTwb4nl9NvBZ/Mf4HOCz+ff5bOCz+I/xOcBn82yI5++rgbcGHsS/z18DL8O/z18BL82/zzOA7wY+m+eE+I/x0sBf8fyJfx/z/L0M8Nf8+yD+45jnT/z7mOdP/Psh/uOY50/8+5jnT/z7If7jmOdP/PuY50/8+yH+45jnT/z7mOdP/Psh/uOY50/8+5jnT/z7If7jmOdP/PuY50/8+yH+45jnT/z7mOdP/Psh/uOY50/8+5jnT/z7If7jmOdP/PuY50/8+yH+45jnT/z7mOdP/Psh/uPsAsd4Ts8AHsy/zy5wjOf0DODB/Psh/uN8NvBZPKfPAT6bf5/PBj6L5/Q5wGfz74f4j/XVwFtzxXcDn81/jK8G3porvhv4bP5jIP5/Q/z/hvj/DfH/G/8IWw+CQWG5hOIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoScreenNormal;
impl IconShape for GoScreenNormal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.25 1a.75.75 0 01.75.75v2.5A1.75 1.75 0 014.25 6h-2.5a.75.75 0 010-1.5h2.5a.25.25 0 00.25-.25v-2.5A.75.75 0 015.25 1zm5.5 0a.75.75 0 01.75.75v2.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 0110 4.25v-2.5a.75.75 0 01.75-.75zM1 10.75a.75.75 0 01.75-.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 01-1.5 0v-2.5a.25.25 0 00-.25-.25h-2.5a.75.75 0 01-.75-.75zm9 1c0-.966.784-1.75 1.75-1.75h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v2.5a.75.75 0 01-1.5 0v-2.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSearch;
impl IconShape for GoSearch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sTzdxz4LOCtgQdzxa3AdwOfw7/sOPBZwFsDD+Y/xq3AdwOfwwtmnhcCzPMnnr+vBj6K5+9rgI/mhftq4KP4z/E1wEfz/JnnhQDz/Inn7yJwnOfvVuAhvHAXgeP857gVeAjPn3leCDDPn3j+doFjPH/PAB7MC7cLHOM/xzOAB/P8meeFAPP8iefvs4HP4vn7HOCzeeE+G/gs/nN8DvDZPH/meSHAPH/iBftq4K2BB3HFM4DvBj6bF81XA28NPIj/GM8Avhv4bF4w87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/y9DPDX/N/w0sBf8bwQ8NfAS/H/098I+Gzgs/j/6XMEHAduBY7x/8sl4MECjgNPB47z/8su8BABnw18Fv8/fY6ApwMP5v+nvxZgnr+XAf6a/xteGvgrnhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/x/B0HPgt4a+DBXHEr8N3A5/AvOw58FvDWwIP5j3Er8N3A5/CCmeeFAPP8iefvq4GP4vn7GuCjeeG+Gvgo/nN8DfDRPH/meSHAPH/i+bsIHOf5uxV4CC/cReA4/zluBR7C82eeFwLM8yeev13gGM/fM4AH88LtAsf4z/EM4ME8f+Z5IcA8f+L5+2zgs3j+Pgf4bF64zwY+i/8cnwN8Ns+feV4IMM+feMG+Gnhr4EFc8Qzgu4HP5kXz1cBbAw/iP8YzgO8GPpsXzDwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/LwP8Nf83vDTwVzwvBPw18FL8//Q3Aj4b+Cz+f/ocAceBW4Fj/P9yCXiwuOK9ge/i/5e3AX5aPNt7A18NHOP/tkvAewM/DSCe03Hgo4G3Bl6K/1v+Bvhp4KuBXa5A/P+G+P8N8f8b4v83xP9viP/f+EeZequb71tQUgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoServer;
impl IconShape for GoServer {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 1A1.75 1.75 0 000 2.75v4c0 .372.116.717.314 1a1.742 1.742 0 00-.314 1v4c0 .966.784 1.75 1.75 1.75h12.5A1.75 1.75 0 0016 12.75v-4c0-.372-.116-.717-.314-1 .198-.283.314-.628.314-1v-4A1.75 1.75 0 0014.25 1H1.75zm0 7.5a.25.25 0 00-.25.25v4c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-4a.25.25 0 00-.25-.25H1.75zM1.5 2.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v4a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25v-4zm5.5 2A.75.75 0 017.75 4h4.5a.75.75 0 010 1.5h-4.5A.75.75 0 017 4.75zM7.75 10a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zM3 4.75A.75.75 0 013.75 4h.5a.75.75 0 010 1.5h-.5A.75.75 0 013 4.75zM3.75 10a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEqUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjv9V5c8T3890D89/ku4L254ruB9+G/HuK/x3cB781z+m7gffivhfiv913Ae/P8fTfwPvzXQfzX+i7gvXnhvht4H/5rIP7rfBfw3rxovht4H/7zIf5rfBfw3vzrfDfwPvznQvzn+y7gvfm3+W7gffjPg/jP9V3Ae/Pv893A+/CfA/Gf57uA9+Y/xncD78N/PMR/ju8C3pv/WN8NvA//sRD/8b4LeG/+c3w38D78x0H8x3pv4Lv4z/U+wHfzHwPxH+utgZ/iP9fbAD/NfwzEfw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfjvYZ4/8V8L8a9zHPgo4K2Bl+Z53Qp8N/A5vHDm+RMv2HHgs4C3Bh7M8/pr4KeBrwF2edEgXnTvDXwVcJx/2dcAH80LZp4/8YJ9NfBR/Mt2gY8Bvpt/GeJF897Ad/GiuxV4CC+Yef7EC3YROM6L7n2A7+aFQ/zLjgNPB47zonsG8GBeMPP8iRdsFzjGi24XeAiwywuG+Jd9NvBZ/Ot8DvDZvGDm+RMv2GcDn8W/zucAn80LhviX/RXw0rxongF8N/DZvHDm+RMv3FcDbw08iBfNXwMvwwuG+JeZ5+9lgL/m38Y8f+Lf5qWBv+L5Ey8Y4l9mnj/xb2eeP/FvZ54/8YIh/mXm+RP/dub5E/925vkTLxjiX2aeP/FvZ54/8W9nnj/xgiH+Zeb5E/925vkT/3bm+RMvGOJfZp4/8W9nnj/xb2eeP/GCIf5l5vkT/3bm+RP/dub5Ey8Y4l9mnj/xb2eeP/FvZ54/8YIh/mXm+RP/dub5E/925vkTLxjiX2aeP/FvZ54/8W9nnj/xgiH+Zeb5E/925vkT/3bm+RMvGOJfZp4/8W+3CxzjOT0DeDD/dub5Ey8Y4l9mnj/xb/fZwGfxnD4H+Gz+7czzJ14wxL/MPH/i3+ergbfmiu8GPpt/H/P8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPP3MsBf8z/DSwN/xfMnXjDEv+yvgZfif6e/AV6aFwzxL/ts4LP43+lzgM/mBUP8y44DtwLH+N/lEvBgYJcXDPGieW/gu/jf5W2An+aFQ7zo3hv4auAY/7NdAt4b+Gn+ZYh/nePARwNvDbwU/7P8DfDTwFcDu7xoEP+/If5/Q/z/hvj/jX8EPXGgQXWV9/gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShare;
impl IconShape for GoShare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.823.177L4.927 3.073a.25.25 0 00.177.427H7.25v5.75a.75.75 0 001.5 0V3.5h2.146a.25.25 0 00.177-.427L8.177.177a.25.25 0 00-.354 0zM3.75 6.5a.25.25 0 00-.25.25v6.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25v-6.5a.25.25 0 00-.25-.25h-1a.75.75 0 010-1.5h1c.966 0 1.75.784 1.75 1.75v6.5A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25v-6.5C2 5.784 2.784 5 3.75 5h1a.75.75 0 110 1.5h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShareAndroid;
impl IconShape for GoShareAndroid {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M13.5 3a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM15 3a3 3 0 01-5.175 2.066l-3.92 2.179a3.005 3.005 0 010 1.51l3.92 2.179a3 3 0 11-.73 1.31l-3.92-2.178a3 3 0 110-4.133l3.92-2.178A3 3 0 1115 3zm-1.5 10a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-9-5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShield;
impl IconShape for GoShield {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.467.133a1.75 1.75 0 011.066 0l5.25 1.68A1.75 1.75 0 0115 3.48V7c0 1.566-.32 3.182-1.303 4.682-.983 1.498-2.585 2.813-5.032 3.855a1.7 1.7 0 01-1.33 0c-2.447-1.042-4.049-2.357-5.032-3.855C1.32 10.182 1 8.566 1 7V3.48a1.75 1.75 0 011.217-1.667l5.25-1.68zm.61 1.429a.25.25 0 00-.153 0l-5.25 1.68a.25.25 0 00-.174.238V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297a.2.2 0 00.154 0c2.245-.956 3.582-2.104 4.366-3.298C13.225 9.666 13.5 8.36 13.5 7V3.48a.25.25 0 00-.174-.237l-5.25-1.68zM9 10.5a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.75a.75.75 0 10-1.5 0v3a.75.75 0 001.5 0v-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldCheck;
impl IconShape for GoShieldCheck {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM11.28 6.28a.75.75 0 00-1.06-1.06L7.25 8.19l-.97-.97a.75.75 0 10-1.06 1.06l1.5 1.5a.75.75 0 001.06 0l3.5-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldLock;
impl IconShape for GoShieldLock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM9.5 6.5a1.5 1.5 0 01-.75 1.3v2.45a.75.75 0 01-1.5 0V7.8A1.5 1.5 0 119.5 6.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoShieldX;
impl IconShape for GoShieldX {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.533.133a1.75 1.75 0 00-1.066 0l-5.25 1.68A1.75 1.75 0 001 3.48V7c0 1.566.32 3.182 1.303 4.682.983 1.498 2.585 2.813 5.032 3.855a1.7 1.7 0 001.33 0c2.447-1.042 4.049-2.357 5.032-3.855C14.68 10.182 15 8.566 15 7V3.48a1.75 1.75 0 00-1.217-1.667L8.533.133zm-.61 1.429a.25.25 0 01.153 0l5.25 1.68a.25.25 0 01.174.238V7c0 1.358-.275 2.666-1.057 3.86-.784 1.194-2.121 2.34-4.366 3.297a.2.2 0 01-.154 0c-2.245-.956-3.582-2.104-4.366-3.298C2.775 9.666 2.5 8.36 2.5 7V3.48a.25.25 0 01.174-.237l5.25-1.68zM6.78 5.22a.75.75 0 10-1.06 1.06L6.94 7.5 5.72 8.72a.75.75 0 001.06 1.06L8 8.56l1.22 1.22a.75.75 0 101.06-1.06L9.06 7.5l1.22-1.22a.75.75 0 10-1.06-1.06L8 6.44 6.78 5.22z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+a/j3n+xL/eSwN/xfNCgHn+xH8v8/yJfxvzvBBgnj/x38s8f+LfxjwvBJjnT/z3Ms+f+LcxzwsB5vkT/73M8yf+bczzQoB5/sR/L/P8iX8b87wQYJ4/8d/LPH/i38Y8LwSY50/89zLPn/i3Mc8LAeb5E/+9zPMn/m3M80KAef7Efy/z/Il/G/O8EGCeP/Hfyzx/4t/GPC8EmOdP/Pcyz5/4tzHPCwHm+RP/vczzJ/5tzPNCgHn+xH8v8/yJfxvzvBBgnj/x38s8f+LfxjwvBJjnT7xo3ho4BnwP/7HM8yf+bczzQoB5/sSLxlzx3cD78B/HPH/i38Y8LwSY50+8aMyzfTfwPvzHMM+f+LcxzwsB5vkTLxrznL4beB/+/czzJ/5tzPNCgHn+xIvGPK/vBt6Hfx/z/Il/G/O8EGCeP/GiMc/fdwPvw7+def7Ev415Xggwz5940ZgX7LuB9+Hfxjx/4t/GPC8EmOdPvGjMC/fdwPvwr2eeP/FvY54XAszzJ1405l/23cD78K9jnj/xb2OeFwLM8ydeNOZF893A+/CiM8+f+LcxzwsB5vkTLxrzonsf4Lt50ZjnT/zbmOeFAPP8iReNedG9DfDTvGjM8yf+bczzQoB5/sSLxrxo3gf4bl505vkT/zbmeSHAPH/iRWP+Ze8DfDf/Oub5E/825nkhwDx/4kVjXrj3Ab6bfz3z/Il/G/O8EGCeP/GiMS/Y+wDfzb+Nef7Ev415Xggwz5940Zjn732A7+bfzjx/4t/GPC8EmOdPvGjM83of4Lv59zHPn/i3Mc8LAeb5Ey8a85zeB/hu/v3M8yf+bczzQoB5/sSLxjzb+wDfzX8M8/yJfxvzvBBgnj/xojFXvA/w3fzHMc+f+LcxzwsB5vkTL5q35oqf5j+Wef7Ev415Xggwz5/472WeP/FvY54XAszzJ/57medP/NuY54UA8/yJ/17m+RP/NuZ5IcA8f+K/l3n+xL+NeV4IMM+f+O9lnj/xb2OeFwLM8yf+e5nnT/zbmOeFAPP8if9e5vkT/zbmeSHAPH/iv5d5/sS/jXleCDDPn/jvZZ4/8W9jnhcCzPMn/nuZ50/825jnhQDz/In/Xub5E/825nkhwDx/4r+Xef7Ev415Xggwz5/472WeP/FvY54XAszzJ/57medP/NuY54UA8/y9DPDX/Pcxz5/413tp4K94Xgj4a+Cl+P/pbwR8NvBZ/P/0OQKOA7cCx/j/5RLwYHHFewPfxf8vbwP8tHi29wa+GjjG/22XgPcGfhpAPKfjwEcDbw28FP+3/A3w08BXA7tcwT8C6TzkzCSLNsoAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSidebarCollapse;
impl IconShape for GoSidebarCollapse {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.823 7.823L4.427 5.427A.25.25 0 004 5.604v4.792c0 .223.27.335.427.177l2.396-2.396a.25.25 0 000-.354z",
                fill_rule: "evenodd",
            }
            path {
                d: "M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25H9.5v13H1.75a.25.25 0 01-.25-.25V1.75zM11 14.5v-13h3.25a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H11z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEyklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+a/j3n+xL/eSwN/xfNCgHn+xH8v8/yJfxvzvBBgnj/x38s8f+LfxjwvBJjnT/z3Ms+f+LcxzwsB5vkT/73M8yf+bczzQoB5/sR/L/P8iX8b87wQYJ4/8d/LPH/i38Y8LwSY50/89zLPn/i3Mc8LAeb5E/+9zPMn/m3M80KAef7Efy/z/Il/G/O8EGCeP/Hfyzx/4t/GPC8EmOdP/Pcyz5/4tzHPCwHm+RP/vczzJ/5tzPNCgHn+xH8v8/yJfxvzvBBgnj/x38s8f+LfxjwvBJjnT/zHei/gEvDTvGjM8yf+bczzQoB5/sR/nO8C3psrxIvGPH/i38Y8LwSY50/8x/gu4L15NvGiMc+f+LcxzwsB5vkT/37fBbw3z0m8aMzzJ/5tzPNCgHn+xL/PdwHvzfMSLxrz/Il/G/O8EGCeP/Fv913Ae/P8iReNef7Ev415Xggwz5/4t/ku4L15wcSLxjx/4t/GPC8EmOdP/Ot9F/DevHDiRWOeP/FvY54XAszzJ/51vgt4b/5l4kVjnj/xb2OeFwLM8ydedN8FvDcvGvGiMc+f+LcxzwsB5vkTL5r3Br6LF5140ZjnT/zbmOeFAPP8iRfNWwM/xYtOvGjM8yf+bczzQoB5/sSL7r2B7+JFI1405vkT/zbmeSHAPH/iX+e9ge/iXyZeNOb5E/825nkhwDx/4l/vvYHv4oUTLxrz/Il/G/O8EGCeP/Fv897Ad/GCiReNef7Ev415Xggwz5/4t3tv4Lt4/sSLxjx/4t/GPC8EmOdP/Pu8N/BdPC/xojHPn/i3Mc8LAeb5E/9+7w18F89JvGjM8yf+bczzQoB5/sR/jPcGvotnEy8a8/yJfxvzvBBgnj/xH+e9ge/iCvGiMc+f+LcxzwsB5vkT/7Hemit+mheNef7Ev415Xggwz5/472WeP/FvY54XAszzJ/57medP/NuY54UA8/yJ/17m+RP/NuZ5IcA8f+K/l3n+xL+NeV4IMM+f+O9lnj/xb2OeFwLM8yf+e5nnT/zbmOeFAPP8if9e5vkT/zbmeSHAPH/iv5d5/sS/jXleCDDPn/jvZZ4/8W9jnhcCzPMn/nuZ50/825jnhQDz/In/Xub5E/825nkhwDx/4r+Xef7Ev415Xggwz5/472WeP/FvY54XAszzJ/57medP/NuY54UA8/y9DPDX/Pcxz5/413tp4K94Xgj4a+Cl+P/pbwR8NvBZ/P/0OQKOA7cCx/j/5RLwYHHFewPfxf8vbwP8tHi29wa+GjjG/22XgPcGfhpAPKfjwEcDbw28FP+3/A3w08BXA7tcwT8CltjRzDUoFvsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSidebarExpand;
impl IconShape for GoSidebarExpand {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.177 7.823l2.396-2.396A.25.25 0 017 5.604v4.792a.25.25 0 01-.427.177L4.177 8.177a.25.25 0 010-.354z",
                fill_rule: "evenodd",
            }
            path {
                d: "M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v12.5A1.75 1.75 0 0114.25 16H1.75A1.75 1.75 0 010 14.25V1.75zm1.75-.25a.25.25 0 00-.25.25v12.5c0 .138.112.25.25.25H9.5v-13H1.75zm12.5 13H11v-13h3.25a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/nePARwFvDbw0L7pbge8GPof/WRAvuvcGvgo4zr/d1wAfzf8ciBfNewPfxb/frcBD+J8D8S87DjwdOM6/3zOAB/M/B+Jf9tnAZ/Ef43OAz+Z/DsS/7K+Al+bf5xnAdwOfzf8siH+Zef5eBvhr/ndD/MvM8yf+90P8y8zzJ/73Q/zLzPMn/vdD/MvM8yf+90P8y8zzJ/73Q/zLzPMn/nN9FfDRXPHTwPsAu/zHQvzLzPMn/vN8F/DePKefAd6a/1iIf5l5/sR/ju8C3pvnT/zHQvzLzPMn/uN9F/DevGDiPxbiX2aeP/Ef67uA9+YF+xrgo/mPhfiXmedP/Mf5LuC9ecG+B3hv/uMh/mXm+RP/Mb4LeG9esO8B3pv/HIh/mXn+xL/fdwHvzQv2PcB7858H8S8zz5/49/ku4L15wb4HeG/+cyH+Zeb5E/923wW8Ny/Y9wDvzX8+xL/MPH/i3+a7gPfmBfse4L35r4H4l5nnT/zrfRfw3rxg3wO8N/91EP8y8/yJf53vAt6bF+x7gPfmvxbiX2aeP/Gi+y7gvfnvcyvw3cDn8JwQ/zLz/IkXzWcDn8X/DF8DfDTPhviXmedPvGguAsf5n+FW4CE8G+JfZp4/8aLZBY7xP8MzgAfzbIh/mXn+xIvms4HP4n+GzwE+m2dD/MvM8ydedN8NvBf/fZ4BfDfw2TwnxL/MPH/iX+e7gffiBftu4H34r4X4l5nnT/zrfTfwXrxg3w28D/91EP8y8/yJf5vvBt6LF+y7gffhvwbiX2aeP/Fv993Ae/GCfTfwPvznQ/zLzPMn/n2+G3gvXrDvBt6H/1yIf5l5/sS/33cD78UL9t3A+/CfB/EvM8+f+I/x3cB78YJ9N/A+/OdA/MvM8yf+43w38F68YN8NvA//8RD/MvP8if9Y3w28Fy/Y1wAfzX8sxL/MPH/iP953A+/FCyb+YyH+Zeb5E/85vht4L54/8R8L8S8zz5/4z/PdwHvxnH4GeGv+YyH+Zeb5E/+5vhr4KK74HuCjgV3+YyH+Zeb5E//7If5l5vkT//sh/mXm+RP/+yH+Zeb5E//7If5l5vkT//sh/mXm+XsZ4K/53w3xL/tr4KX497kV+G7gc/ifBfEv+2zgs/iP8TXAR/M/B+Jfdhy4FTjGv9+twEP4nwPxonlv4Lv493sG8GD+50C86N4b+GrgGP92nwN8Nv9zIP51jgMfDbw18FK86J4BfDfw2fzPgvj/DfH/G+L/N8T/b/wjxTeKQc6zoWIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSignIn;
impl IconShape for GoSignIn {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 012 13.25V2.75zm6.56 4.5l1.97-1.97a.75.75 0 10-1.06-1.06L6.22 7.47a.75.75 0 000 1.06l3.25 3.25a.75.75 0 101.06-1.06L8.56 8.75h5.69a.75.75 0 000-1.5H8.56z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4h/nePARwFvDbw0L7pbge8GPof/WRAvuvcGvgo4zr/d1wAfzf8ciBfNewPfxb/frcBD+J8D8S87DjwdOM6/3zOAB/M/B+Jf9tnAZ/Ef43OAz+Z/DsS/7K+Al+bf5xnAdwOfzf8siH+Zef5eBvhr/ndD/MvM8yf+90P8y8zzJ/73Q/zLzPMn/vdD/MvM8yf+90P8y8zzJ/73Q/zLzPMn/uscB74LeGuu+GrgY/j3Q/zLzPMn/uv8NPBWPKfvBt6Hfx/Ev8w8f+K/jnn+vht4H/7tEP8y8/yJ/zrmBftu4H34t0H8y8zzJ/7rfDXwUbxg3w28D/96iH+Zef7Ef63vBt6LF+y7gffhXwfxLzPPn/iv993Ae/GCfTfwPrzoEP8y8/yJ/x7fDbwXL9h3A+/DiwbxLzPPn/jv893Ae/GCfTfwPvzLEP8y8/yJ/17fDbwXL9h3A+/DC4f4l5nnT/z3+27gvXjBvht4H14wxL/MPH/if4bvBt6LF+y7gffh+UP8y8zzJ/7n+G7gvXjBvht4H54X4l9mnj/xojkOfBbw1sCD+e/z3cD78JwQ/zLz/IkXzVcDH8X/DJ8DfDbPhviXmedPvGguAsf5n+FW4CE8G+JfZp4/8aLZBY7xP8MzgAfzbIh/mXn+xIvms4HP4n+GzwE+m2dD/MvM8ydedF8NvDXwIP77fA/w3jwnxL/MPH/if47vAt6bF+x7gPfmeSH+Zeb5E/8zfBfw3rxg3wO8N88f4l9mnj/x3++7gPfmBfse4L15wRD/MvP8if9e3wW8Ny/Y9wDvzQuH+JeZ50/89/ku4L15wb4HeG/+ZYh/mXn+xH+P7wLemxfse4D35kWD+JeZ50/81/su4L15wb4HeG9edIh/mXn+xH+t7wLemxfse4D35l8H8S8zz5/4r/PVwEfxgn0P8N786yH+Zeb5E/91zAv2PcB782+D+JeZ50/81zHP3/cA782/HeJfZp4/8V/np4G34jl9D/De/Psg/mXm+RP/dY4D3w28FXAJ+G7go/n3Q/zLzPMn/vdD/MvM8yf+90P8y8zzJ/73Q/zLzPMn/vdD/MvM8yf+90P8y8zz9zLAX/O/G+Jf9tfAS/Hvcyvw3cDn8D8L4l/22cBn8R/ja4CP5n8OxL/sOHArcIx/v1uBh/A/B+JF897Ad/Hv9wzgwfzPgXjRvTfw1cAx/u0+B/hs/udA/OscBz4aeGvgpXjRPQP4buCz+Z8F8f8b4v83xP9viP/f+EcDc4tB2CmeCAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSignOut;
impl IconShape for GoSignOut {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2 2.75C2 1.784 2.784 1 3.75 1h2.5a.75.75 0 010 1.5h-2.5a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h2.5a.75.75 0 010 1.5h-2.5A1.75 1.75 0 012 13.25V2.75zm10.44 4.5H6.75a.75.75 0 000 1.5h5.69l-1.97 1.97a.75.75 0 101.06 1.06l3.25-3.25a.75.75 0 000-1.06l-3.25-3.25a.75.75 0 10-1.06 1.06l1.97 1.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn3jR/BXw0vzn+GvgZXjRmOdPPBvieZnnT7xovht4L/5zfA/w3rxozPMnng3xvMzzJ1503w28F/+xvgd4b1505vkTz4Z4Xub5E/863w28F/8xvgd4b/51zPMnng3xvMzzJ/71vht4L/59vgd4b/71zPMnng3xvMzzJ/5tvht4L/5tvgd4b/5tzPMnng3xvMzzJ/7t/hp4Kf51vgd4b/7tzPMnng3xvMzzJ/7tjgO/DbwUL5q/AV4b2OXfzjx/4tkQz8s8f+Lf5zjw28BL8cL9DfDawC7/Pub5E8+GeF7m+RP/fseB3wZeiufvb4DXBnb59zPPn3g2xPMyz5/4j3Ec+G3gpXhOfwO8NrDLfwzz/IlnQzwv8/yJ/zjHgd8GXoor/gZ4bWCX/zjm+RPPhnhe5vkT/7GOA7/NFa8N7PIfyzx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8fy8D/DX/O7w08Fc8f+LZEM/rr4GX4v+mvwFemmdDPK/PBj6L/5s+B/hsng3xvI4DtwLH+L/lEvBgYJdnQzx/7w18F/+3vA3w0zwnxAv23sBXA8f43+0S8N7AT/O8EC/cceCjgbcGXor/Xf4G+Gngq4Fdnj/E/2+I/98Q/78h/n/jHwEC2ZlBIHrvtAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSingleSelect;
impl IconShape for GoSingleSelect {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.06 7.356l2.795 2.833c.08.081.21.081.29 0l2.794-2.833c.13-.131.038-.356-.145-.356H5.206c-.183 0-.275.225-.145.356z",
            }
            path {
                d: "M1 2.75C1 1.784 1.784 1 2.75 1h10.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0113.25 15H2.75A1.75 1.75 0 011 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h10.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H2.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSkip;
impl IconShape for GoSkip {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zm3.28 5.78a.75.75 0 00-1.06-1.06l-5.5 5.5a.75.75 0 101.06 1.06l5.5-5.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovivcxz4LOCtueK7gc/hvxfiv85XAx/Fc/oa4KP574P4r3MROM5zuhV4CP99EP91zPMn/vsg/uuY50/890H81zHPn/jvg/ivY54/8d8H8V/HPH/ivw/ieR0HPgt4a+DB/OcTz99x4LOAtwYezL/PrcB3A5/Dc0I8r68GPor/OuL5+2rgo/iP9TXAR/NsiOd1ETjOfx3x/F0EjvMf61bgITwb4nntAsf4ryOev13gGP+xngE8mGdDPK/PBj6L/zri+fts4LP4j/U5wGfzbIjn76uBtwYexH8+8YJ9NfDWwIP493kG8N3AZ/OcEP91zPMn/vsg/uuY50/890H81zHPn/jvg/ivY54/8d8H8V/HPH/ivw/iv84ucIzn9Azgwfz3QfzX+Wzgs3hOnwN8Nv99EP+1vhp4a674buCz+ZcdBz4LeGuu+G7gc/iPgfif76uBj+I5fQ3w0fz7If7nuwgc5zndCjyEfz/E/3zm+RP/foj/+czzJ/79EP/zmedP/Psh/uczz5/490P8z2eeP/Hvh3hex4HPAt4aeDD/uW4Fvhv4HF4w8+93K/DdwOfwnBDP66uBj+K/1tcAH83zZ/7jfA3w0Twb4nldBI7zX+tW4CE8f+Y/zq3AQ3g2xPPaBY7xX+sZwIN5/sx/nGcAD+bZEM/rs4HP4r/W5wCfzfNn/uN8DvDZPBvi+ftq4K2BB/Gf6xnAdwOfzQtm/v2eAXw38Nk8J8T/fOb5E/9+iP/5zPMn/v0Q//OZ50/8+yH+5zPPn/j3Q/zPZ54/8e+H+J9vFzjGc3oG8GD+/RD/83028Fk8p88BPpt/P8R/nOPAZwFvzRXfDXwO/zG+Gnhrrvhu4LP5j4H4j/PVwEfxnL4G+Gj+50L8x7kIHOc53Qo8hP+5EP9xzPMn/udC/Mcxz5/4nwvxH8c8f+J/LsR/HPP8if+5EP9xzPMn/udCPK/jwGcBbw08mH8/8fwdBz4LeGvgwfznuhX4buBzeE6I5/XVwEfxH0c8f18NfBT/tb4G+GieDfG8LgLH+Y8jnr+LwHH+a90KPIRnQzyvXeAY/3HE87cLHOO/1jOAB/NsiOf12cBn8R9HPH+fDXwW/7U+B/hsng3x/H018NbAg/j3Ey/YVwNvDTyI/1zPAL4b+GyeE+I/jnn+xP9ciP845vkT/3Mh/uOY50/8z4X4j2OeP/E/F+I/jnn+xP9ciP84u8AxntMzgAfzPxfiP85nA5/Fc/oc4LP5nwvxH+urgbfmiu8GPpv/2fhHJs2GQeB4WVwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSliders;
impl IconShape for GoSliders {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M15 2.75a.75.75 0 01-.75.75h-4a.75.75 0 010-1.5h4a.75.75 0 01.75.75zm-8.5.75v1.25a.75.75 0 001.5 0v-4a.75.75 0 00-1.5 0V2H1.75a.75.75 0 000 1.5H6.5zm1.25 5.25a.75.75 0 000-1.5h-6a.75.75 0 000 1.5h6zM15 8a.75.75 0 01-.75.75H11.5V10a.75.75 0 11-1.5 0V6a.75.75 0 011.5 0v1.25h2.75A.75.75 0 0115 8zm-9 5.25v-2a.75.75 0 00-1.5 0v1.25H1.75a.75.75 0 000 1.5H4.5v1.25a.75.75 0 001.5 0v-2zm9 0a.75.75 0 01-.75.75h-6a.75.75 0 010-1.5h6a.75.75 0 01.75.75z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSmiley;
impl IconShape for GoSmiley {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0zM8 0a8 8 0 100 16A8 8 0 008 0zM5 8a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zM5.32 9.636a.75.75 0 011.038.175l.007.009c.103.118.22.222.35.31.264.178.683.37 1.285.37.602 0 1.02-.192 1.285-.371.13-.088.247-.192.35-.31l.007-.008a.75.75 0 111.222.87l-.614-.431c.614.43.614.431.613.431v.001l-.001.002-.002.003-.005.007-.014.019a1.984 1.984 0 01-.184.213c-.16.166-.338.316-.53.445-.63.418-1.37.638-2.127.629-.946 0-1.652-.308-2.126-.63a3.32 3.32 0 01-.715-.657l-.014-.02-.005-.006-.002-.003v-.002h-.001l.613-.432-.614.43a.75.75 0 01.183-1.044h.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADk0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/4bjwG9xxesAu7xoEP/7HQd+C3hprvhr4HWAXf5liP/djgO/Bbw0z+mvgdcBdnnhEP97HQd+C3hpnr+/Bl4H2OUFQ8Bx4LOA9waO86K5Ffhu4HP473Ec+C3gpXnh/hp4HWCX5w8BvwW8Nv82XwN8NP+1jgO/Bbw0L5q/Bl4H2OV5IcD8290KPIT/OseB3wJemn+dvwZeB9jlOSFgFzjGv80zgAfzX+M48FvAS/Nv89fA6wC7PBsCPhv4LP5tPgf4bP7zHQd+C3hp/n3+GngdYJcrEFd8NfDWwIN40TwD+G7gs/nPdxz4LeCl+Y/x18DrALsA4n+248BvAS/Nf6y/Bl4H2BX/cx0Hfgt4af5z/DXwOuJ/rr8CXpr/XH8t/uf6a+Cl+M/1N+J/P/P8iX8Z4n8/8/yJfxnifz/z/Il/GeJ/P/P8iX8Z4n8/8/yJfxnifz/z/Il/GQKOA58FvDdwnP8YtwLfDXwO//nM8yf+ZQj4LeC1+c/xNcBH85/LPH/iX4YA85/nVuAh/Ocyz5/4lyFgFzjGf45nAA/mP5d5/sS/DAGfDXwW/zk+B/hs/nOZ50/8yxBXfDXw1sCD+I/xDOC7gc/mP595/sS/DPG/n3n+xL8M8b+fef7Evwzxv595/sS/DPG/n3n+xL8M8b+fef7Evwzxv595/sS/DPG/n3n+xL8M8b+fef7Evwzxv595/sS/DPG/n3n+xL8MAceBzwLeGzjOf49bge8GPod/PfP8iX8ZAn4LeG3+Z/ga4KP51zHPn/iXIcD8z3Er8BD+dczzJ/5lCNgFjvE/wzOAB/OvY54/8S9DwGcDn8X/DJ8DfDb/Oub5E/8yxBVfDbw18CD+ezwD+G7gs/nX2wWO8ZyeATyYfxnif7/PBj6L5/Q5wGfzL0P83/DVwFtzxXcDn82LBvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EZ47cPDWtOGMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSortAsc;
impl IconShape for GoSortAsc {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 4.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5H.75A.75.75 0 010 4.25zm0 4a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5H.75A.75.75 0 010 8.25zm0 4a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H.75a.75.75 0 01-.75-.75zm12.927-9.677a.25.25 0 00-.354 0l-3 3A.25.25 0 009.75 6H12v6.75a.75.75 0 001.5 0V6h2.25a.25.25 0 00.177-.427l-3-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I//2OA58FvDVXfDfwObxoEP/7fTXwUTynrwE+mn8ZAo4DnwW8N3Cc/x63At8NfA7/eheB4zynW4GH8C9DwG8Br83/DF8DfDT/Oub5E/8yBJj/OW4FHsK/jnn+xL8MAbvAMf5neAbwYP51zPMn/mUI+Gzgs/if4XOAz+Zfxzx/4l+GuOKrgbcGHsR/j2cA3w18Nv965vkT/zLE/37m+RP/MsT/fub5E/8yxP9+5vkT/zLE/37m+RP/MsT/fub5E/8yxP9+5vkT/zLE/37m+RP/MsT/fub5E/8yxP9+5vkT/zLE/37m+RP/MgQcBz4LeG/gOP8xbgW+G/gc/vOZ50/8yxDwW8Br85/ja4CP5j+Xef7EvwwB5j/PrcBD+M9lnj/xL0PALnCM/xzPAB7Mfy7z/Il/GQI+G/gs/nN8DvDZ/Ocyz5/4lyGu+GrgrYEH8R/jGcB3A5/Nfz7z/Il/GeJ/P/P8iX8Z4n8/8/yJfxnifz/z/Il/GeJ/P/P8iX8Z4n+uvwJemv9cfy3+5/pr4KX4z/U34n+u48BvAy/Ff46/AV5b/M92HPht4KX4j/U3wGsDu+J/vuPAbwMvxX+MvwFeG9gFEP87HAd+G3gp/n3+BnhtYJcrEHAc+CzgvYHjvGhuBb4b+Bz+6xwHfht4Kf5t/gZ4bWCXZ0PAbwGvzb/N1wAfzX+d48BvAy/Fv87fAK8N7PKcEGD+7W4FHsJ/rePAbwMvxYvmb4DXBnZ5XgjYBY7xb/MM4MH81zsO/DbwUrxwfwO8NrDL84eAzwY+i3+bzwE+m/8ex4HfBl6K5+9vgNcGdnnBEFd8NfDWwIN40TwD+G7gs/nvdRz4beCleE5/A7w2sMsLh/jf7zjw28BLccXfAK8N7PIvQ/zfcBz4ba54bWCXFw3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9v/COqa27w7y3BRQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSortDesc;
impl IconShape for GoSortDesc {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 4.25a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5H.75A.75.75 0 010 4.25zm0 4a.75.75 0 01.75-.75h4.5a.75.75 0 010 1.5H.75A.75.75 0 010 8.25zm0 4a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5H.75a.75.75 0 01-.75-.75z",
                fill_rule: "evenodd",
            }
            path {
                d: "M13.5 10h2.25a.25.25 0 01.177.427l-3 3a.25.25 0 01-.354 0l-3-3A.25.25 0 019.75 10H12V3.75a.75.75 0 011.5 0V10z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAC6ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+I91HPgo4K2Bl+Y/1l8DPw18DbDLfwzEf5z3Br4KOM5/rl3gY4Dv5t8P8R/jvYHv4r/W+wDfzb8P4t/vOPB04Dj/tXaBhwC7/Nsh/v0+G/gs/nt8DvDZ/Nsh/v3+Cnhp/nv8NfAy/Nsh/v3M8/cywF/zH+Olgb/i+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8if9Y5vkT/3aIfz/z/In/WOb5E/92iH8/8/yJ/1jm+RP/doh/P/P8vQzw1/zHeGngr3j+xL8d4t/vr4GX4r/H3wAvzb8d4t/vs4HP4r/H5wCfzb8d4t/vOHArcIz/WpeABwO7/Nsh/mO8N/Bd/Nd6G+Cn+fdB/Md5b+CrgWP857oEvDfw0/z7If5jHQc+Gnhr4KX4j/U3wE8DXw3s8h8D8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BMC3SkEFUMCeAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquare;
impl IconShape for GoSquare {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4 5.75C4 4.784 4.784 4 5.75 4h4.5c.966 0 1.75.784 1.75 1.75v4.5A1.75 1.75 0 0110.25 12h-4.5A1.75 1.75 0 014 10.25v-4.5zm1.75-.25a.25.25 0 00-.25.25v4.5c0 .138.112.25.25.25h4.5a.25.25 0 00.25-.25v-4.5a.25.25 0 00-.25-.25h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACe0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+I91HPgo4K2Bl+Y/1l8DPw18DbDLfwzEf5z3Br4KOM5/rl3gY4Dv5t8P8R/jvYHv4r/W+wDfzb8P4t/vOPB04Dj/tXaBhwC7/Nsh/v0+G/gs/nt8DvDZ/Nsh/v3+Cnhp/nv8NfAy/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3Mfy/xb4f49zP/vcS/HeLfz/z3Ev92iH8/899L/Nsh/v3+Gngp/nv8DfDS/Nsh/v0+G/gs/nt8DvDZ/Nsh/v2OA7cCx/ivdQl4MLDLvx3iP8Z7A9/Ff623AX6afx/Ef5z3Br4aOMZ/rkvAewM/zb8f4j/WceCjgbcGXor/WH8D/DTw1cAu/zEQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hH8zc0QTUPzKYAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquareFill;
impl IconShape for GoSquareFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.75 4A1.75 1.75 0 004 5.75v4.5c0 .966.784 1.75 1.75 1.75h4.5A1.75 1.75 0 0012 10.25v-4.5A1.75 1.75 0 0010.25 4h-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSquirrel;
impl IconShape for GoSquirrel {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.499.75a.75.75 0 011.5 0v.996C5.9 2.903 6.793 3.65 7.662 4.376l.24.202c-.036-.694.055-1.422.426-2.163C9.1.873 10.794-.045 12.622.26 14.408.558 16 1.94 16 4.25c0 1.278-.954 2.575-2.44 2.734l.146.508.065.22c.203.701.412 1.455.476 2.226.142 1.707-.4 3.03-1.487 3.898C11.714 14.671 10.27 15 8.75 15h-6a.75.75 0 010-1.5h1.376a4.489 4.489 0 01-.563-1.191 3.833 3.833 0 01-.05-2.063 4.636 4.636 0 01-2.025-.293.75.75 0 11.525-1.406c1.357.507 2.376-.006 2.698-.318l.009-.01a.748.748 0 011.06 0 .75.75 0 01-.012 1.074c-.912.92-.992 1.835-.768 2.586.221.74.745 1.337 1.196 1.621H8.75c1.343 0 2.398-.296 3.074-.836.635-.507 1.036-1.31.928-2.602-.05-.603-.216-1.224-.422-1.93l-.064-.221c-.12-.407-.246-.84-.353-1.29a2.404 2.404 0 01-.507-.441 3.063 3.063 0 01-.633-1.248.75.75 0 011.455-.364c.046.185.144.436.31.627.146.168.353.305.712.305.738 0 1.25-.615 1.25-1.25 0-1.47-.95-2.315-2.123-2.51-1.172-.196-2.227.387-2.706 1.345-.46.92-.27 1.774.019 3.062l.042.19a.753.753 0 01.01.05c.348.443.666.949.94 1.553a.75.75 0 11-1.365.62c-.553-1.217-1.32-1.94-2.3-2.768a85.08 85.08 0 00-.317-.265c-.814-.68-1.75-1.462-2.692-2.619a3.74 3.74 0 00-1.023.88c-.406.495-.663 1.036-.722 1.508.116.122.306.21.591.239.388.038.797-.06 1.032-.19a.75.75 0 01.728 1.31c-.515.287-1.23.439-1.906.373-.682-.067-1.473-.38-1.879-1.193L.75 5.677V5.5c0-.984.48-1.94 1.077-2.664.46-.559 1.05-1.055 1.673-1.353V.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStack;
impl IconShape for GoStack {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.122.392a1.75 1.75 0 011.756 0l5.003 2.902c.83.481.83 1.68 0 2.162L8.878 8.358a1.75 1.75 0 01-1.756 0L2.119 5.456a1.25 1.25 0 010-2.162L7.122.392zM8.125 1.69a.25.25 0 00-.25 0l-4.63 2.685 4.63 2.685a.25.25 0 00.25 0l4.63-2.685-4.63-2.685zM1.601 7.789a.75.75 0 011.025-.273l5.249 3.044a.25.25 0 00.25 0l5.249-3.044a.75.75 0 01.752 1.298l-5.248 3.044a1.75 1.75 0 01-1.756 0L1.874 8.814A.75.75 0 011.6 7.789zm0 3.5a.75.75 0 011.025-.273l5.249 3.044a.25.25 0 00.25 0l5.249-3.044a.75.75 0 01.752 1.298l-5.248 3.044a1.75 1.75 0 01-1.756 0l-5.248-3.044a.75.75 0 01-.273-1.025z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStar;
impl IconShape for GoStar {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfHf56uA9+aK7wY+hv96iP8eXw18FM/pa4CP5r8W4r/eg4Gn8/w9BLiV/zqI/3rfDbwXz9/3AO/Nfx3Ef60HA0/nhXsIcCv/NRD/tb4beC9euO8B3pv/Goj/Og8Gns6L5iHArfznQ/zX+W7gvXjRfA/w3vznQ/zXeDDwdF50u8BDgF3+cyH+a3w18FH863wO8Nn850L85zsOPB04zr/OLvAQYJf/PIj/fJ8NfBb/Np8DfDb/eRD/uY4DTweO82+zCzwE2OU/B+I/12cDn8W/z+cAn81/DsR/nuPA04Hj/PvsAg8BdvmPh/jP89nAZ/Ef43OAz+Y/HuI/x3Hg6cBx/mPsAg8BdvmPhfj3eWngGPDawHHgpYEHAw/mP8etwK3AXwO7wG8Dl4C/5t8G8S97aeAY8NrAceClgQcDD+Z/lluBW4G/Bm4F/hp4BnArLxjiOR0HPgp4beA48NL83/DXwC7w28DXALtcgXi248BfAQ/m/7ZbgZcBdgHEs3008FX8//AxwFcDiGf7auCj+P/hc4DPBhDP9t7Ad/H/w/sA3w0gntN3A+/F/23fA7w3VyCe13cD78X/Td8DvDfPhnj+vht4L/5v+R7gvXlOiBfsu4H34v+G7wHem+eFeOG+G3gv/nf7HuC9ef4Q/7LvBt6L/52+B3hvXjDEi+a7gffif5fvAd6bFw7xovtu4L343+F7gPfmX4b41/lu4L34n+17gPfmRYP413tv4Lv4n+l9gO/mRYf4t3lv4Lv4n+V9gO/mXwfxb/fewHfxP8P7AN/Nvx7i3+e7gffiv9fXAB/Nvw3i3+e3gdfiv9fvAK/Nvw3i3+cicJz/XrvACf5tEP92x4GL/M9wAtjlXw/xb/fawG/xP8PrAL/Nvx7i3+6jga/if4aPAb6afz3Ev91XAx/F/wxfA3w0/3qIf7vfBl6L/xl+B3ht/vUQ/3YXgeP8z7ALnOBfD/Fv82Dg6fzPcgLY5V8H8W/z2sBv8T/L6wC/zb8O4t/ms4HP4n+WzwE+m38dxL/NdwPvxb/f93DFe/Hv9z3Ae/Ovg/i3+W3gtfi3+x3gvYFbueLBwHcDr8W/3e8Ar82/DuLfxvzb/A7w2cBv8/y9NvDZwGvxbyP+dRD/eg8Gns6/zu8Anw38Ni+a1wY+G3gt/nUeAtzKiw7xr/fawG/xonkG8NnAd/Nv897AZwMP4kXzOsBv86JD/Ot9NvBZvHDPAD4b+G7+Y7w38NnAg3jhPgf4bF50iH+9twZ+iufvEvDZwFfzn+Ojgc8GjvH8vQ3w07zoEP82fw28FM92Cfhq4KuBXf5zHQc+Gvho4BjP9jfAS/Ovg/i3OQ68N/Bg4Fbgu4Fd/msdB94beDBwK/DdwC7/Ooj/3xD/vyH+f0P8/4b4/w3x/xv/CNGhn0GgKKx2AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStarFill;
impl IconShape for GoStarFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStop;
impl IconShape for GoStop {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.47.22A.75.75 0 015 0h6a.75.75 0 01.53.22l4.25 4.25c.141.14.22.331.22.53v6a.75.75 0 01-.22.53l-4.25 4.25A.75.75 0 0111 16H5a.75.75 0 01-.53-.22L.22 11.53A.75.75 0 010 11V5a.75.75 0 01.22-.53L4.47.22zm.84 1.28L1.5 5.31v5.38l3.81 3.81h5.38l3.81-3.81V5.31L10.69 1.5H5.31zM8 4a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 018 4zm0 8a1 1 0 100-2 1 1 0 000 2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStopwatch;
impl IconShape for GoStopwatch {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.75.75A.75.75 0 016.5 0h3a.75.75 0 010 1.5h-.75v1l-.001.041a6.718 6.718 0 013.464 1.435l.007-.006.75-.75a.75.75 0 111.06 1.06l-.75.75-.006.007a6.75 6.75 0 11-10.548 0L2.72 5.03l-.75-.75a.75.75 0 011.06-1.06l.75.75.007.006A6.718 6.718 0 017.25 2.541a.756.756 0 010-.041v-1H6.5a.75.75 0 01-.75-.75zM8 14.5A5.25 5.25 0 108 4a5.25 5.25 0 000 10.5zm.389-6.7l1.33-1.33a.75.75 0 111.061 1.06L9.45 8.861A1.502 1.502 0 018 10.75a1.5 1.5 0 11.389-2.95z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEeElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P853st4LWBlwaOAw8GHsyz3QrcCvw18NfA7wC38l8D8Z/jpYGPAt6bf5u/Bn4a+B7gVv7zIP5jvTTwVcBr8x/np4GvAX6b/3iI/zifBXw2/3l+G/gY4K/5j4P4j/FdwHvzX+OrgY/hPwbi3++vgJfmv9b7AN/Nvx/i3+ezgc/ihbsE/DTw18BfA7cCt3LFg4EHA8eBtwZeG3gQ/7KfAd6afz/Ev91rA7/FC3YJ+Gjgp4FdXnQPBj4beGvgGM/f9wDvzb8f4t/up4G34vn7GeC9gV3+7Y4DHw18NHCM5/Q6wG/z74f4t3kw8HSev58B3pr/OA8GPht4MFd8NfDT/MdA/Nt8NPBVPK9LwIOBXf53QPzbfDXwUTyvrwE+mv89EP82vw28Fs/rdYDf5n8PxL/NbwOvxfN6HeC3+d8D8W/z08Bb8bweAtzK/x6If5u3Bn6K5/Q7wGvzvwvi3+69gc8GjgO/Dbw3sMv/Loj/3xD/vyH+f0P8/4Z4XseBzwLeGngw/zfcCnw38Dk8J8Tz+mrgo/i/6WuAj+bZEM/rInCc/5tuBR7CsyGe1y5wjP+bngE8mGdDPK/PBj6L/5s+B/hsng3x/H018NbAg/i/4RnAdwOfzXNC/Pd6MPDawGsDbw0c41/23cD78B8D8T/HceCtgc8GHsQL9z7Ad/Pvh/if6b2BrwaO8fztAg8Bdvn3QfzPdRz4beCleP4+B/hs/n0Q/7MdB34beCme163AQ/j3QfzP99LAX/H8PQS4lX87xP8Ofw28FM/rdYDf5t8O8b/DVwMfxfP6GOCr+bdD/O/w1cBH8bw+Bvhq/u0Q/zv8NvBaPK/XAX6bfzvE/3zHgYs8f68D/Db/doh/m7cGXgvYBb4HuJX/PF8NfBTP6xnAg/n3QfzrfTTwVTzbLvAxwHfzH++9ge/i+fsa4KP590H8610EjvO8fhp4H2CX/xgfBXw1L9hDgFv590H86xwHLvKC7QLfDXwNcCv/eseBtwI+G3gwL9jXAB/Nvx/iX+9W4EH8y24Ffhr4a+BW4G+AXZ7Ta3HFawMvDbw2cJwX7hLwYGCXfz/Ev95LA78NHOO/3iXgtYG/5j8G4t/mpYGfBh7Ef51LwGsDf81/HMS/3XHgs4GP4j/fzwDvDezyHwvx7/dg4LOB9+I/3t8Anw38NP85EP9xHgy8NfDewEvxb3cJ+Gngq4G/5j8X4j/Hg4GXBl4aeG3gOPBSPKe/AXa54q+Bvwb+Gvhr/usg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y9/a5RBMwZgBwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoStrikethrough;
impl IconShape for GoStrikethrough {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.581 3.25c-2.036 0-2.778 1.082-2.778 1.786 0 .055.002.107.006.157a.75.75 0 01-1.496.114 3.56 3.56 0 01-.01-.271c0-1.832 1.75-3.286 4.278-3.286 1.418 0 2.721.58 3.514 1.093a.75.75 0 11-.814 1.26c-.64-.414-1.662-.853-2.7-.853zm3.474 5.25h3.195a.75.75 0 000-1.5H1.75a.75.75 0 000 1.5h6.018c.835.187 1.503.464 1.951.81.439.34.647.725.647 1.197 0 .428-.159.895-.594 1.267-.444.38-1.254.726-2.676.726-1.373 0-2.38-.493-2.86-.956a.75.75 0 00-1.042 1.079C3.992 13.393 5.39 14 7.096 14c1.652 0 2.852-.403 3.65-1.085a3.134 3.134 0 001.12-2.408 2.85 2.85 0 00-.811-2.007z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSun;
impl IconShape for GoSun {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 10.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5zM8 12a4 4 0 100-8 4 4 0 000 8zM8 0a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0V.75A.75.75 0 018 0zm0 13a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 018 13zM2.343 2.343a.75.75 0 011.061 0l1.06 1.061a.75.75 0 01-1.06 1.06l-1.06-1.06a.75.75 0 010-1.06zm9.193 9.193a.75.75 0 011.06 0l1.061 1.06a.75.75 0 01-1.06 1.061l-1.061-1.06a.75.75 0 010-1.061zM16 8a.75.75 0 01-.75.75h-1.5a.75.75 0 010-1.5h1.5A.75.75 0 0116 8zM3 8a.75.75 0 01-.75.75H.75a.75.75 0 010-1.5h1.5A.75.75 0 013 8zm10.657-5.657a.75.75 0 010 1.061l-1.061 1.06a.75.75 0 11-1.06-1.06l1.06-1.06a.75.75 0 011.06 0zm-9.193 9.193a.75.75 0 010 1.06l-1.06 1.061a.75.75 0 11-1.061-1.06l1.06-1.061a.75.75 0 011.061 0z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoSync;
impl IconShape for GoSync {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8 2.5a5.487 5.487 0 00-4.131 1.869l1.204 1.204A.25.25 0 014.896 6H1.25A.25.25 0 011 5.75V2.104a.25.25 0 01.427-.177l1.38 1.38A7.001 7.001 0 0114.95 7.16a.75.75 0 11-1.49.178A5.501 5.501 0 008 2.5zM1.705 8.005a.75.75 0 01.834.656 5.501 5.501 0 009.592 2.97l-1.204-1.204a.25.25 0 01.177-.427h3.646a.25.25 0 01.25.25v3.646a.25.25 0 01-.427.177l-1.38-1.38A7.001 7.001 0 011.05 8.84a.75.75 0 01.656-.834z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEf0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviRXMc+CjgrYGX5n+2vwZ+GvgaYJcXDvEve2/gq4Dj/O+yC3wM8N28YIgX7r2B7+J/t/cBvpvnD/GCHQeeDhznf7dd4CHALs8L8YJ9NvBZ/N/wOcBn87wQL9hfAS/N/w1/DbwMzwvxgpnn72WAv+Z/ppcG/ornTzwvxAtmnj/xP5t5/sTzQrxg5vkT/7OZ5088L8QLZp4/8T+bef7E80K8YOb5E/+zmedPPC/EC2aeP/E/m3n+xPNCvGDm+RP/s5nnTzwvxAtmnj/xP5t5/sTzQrxg5vkT/3pvDfwU/35/DbwML5x5/sTzQrxg5vkT/3pvDfwU/zHEC2eeP/G8EC+Yef7Ev817A9/Fv5944czzJ54X4gUzz5/4t3tv4Lv49xEvnHn+xPNCvGDm+RP/Pu8NfBf/duKFM8+feF6IF8w8f+Lf772B7+LfRrxw5vkTzwvxgpnnT/zHeG/gu/jXEy+cef7E80K8YOb5E/9x3hv4Lv51xAtnnj/xvBAvmHn+xH+c7wLem38d8cKZ5088L8QLZp4/8R/ju4D35l9PvHDm+RPPC/GCmedP/Pt9F/De/NuIF848f+J5IV4w8/yJf5/vAt6bfzvxwpnnTzwvxAtmnj/xb/ddwHvzgn0P8NvAd/GCiRfOPH/ieSFeMPP8iX+b7wLemxfse4D35or3Br6L50+8cOb5E88L8YKZ50/8630X8N68YN8DvDfP6b2B7+J5iRfOPH/ieSFeMPP8iX+d7wLemxfse4D35vl7b+C7eE7ihTPPn3heiBfMPH/iRfddwHvzgn0P8N68cO8NfBfPJl448/yJ54V4wczzJ1403wW8Ny/Y9wDvzYvmvYHv4grxwpnnTzwvxAtmnj/xL/su4L15wb4HeG/+dd6aK36aF848f+J5IV4w8/yJF+6rgY/iBfse4L35z2OeP/G8EC+Yef7EC2desO8B3pv/XOb5E88L8YKZ50+8cOb5+x7gvfnPZ54/8bwQL5h5/sQL99PAW/Gcvgd4b/5rmOdPPC/EC2aeP/HCHQe+G3gr4BLw3cBH81/HPH/ieSFeMPP8if/ZzPMnnhfiBTPPn/ifzTx/4nkh4DjwWcB7A8f5l4n/2cy/7Fbgu4HPEfBbwGvzohP/s5kX3dcIMP864n8286K7VcAucIwXzTOAB/M/2y5wjBfNMwR8NvBZvGg+B/hs/mf7bOCzeNF8jrjiq4G3Bh7E8/cM4LuBz+Z/h68G3hp4EM/fM4DvBj5b/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BdfyZEGgac20AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTabExternal;
impl IconShape for GoTabExternal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.25 4a.25.25 0 00-.25.25v9a.75.75 0 01-.75.75H.75a.75.75 0 010-1.5h.75V4.25c0-.966.784-1.75 1.75-1.75h9.5c.966 0 1.75.784 1.75 1.75v8.25h.75a.75.75 0 010 1.5h-1.5a.75.75 0 01-.75-.75v-9a.25.25 0 00-.25-.25h-9.5z",
            }
            path {
                d: "M7.97 7.97l-2.75 2.75a.75.75 0 101.06 1.06l2.75-2.75 1.543 1.543a.25.25 0 00.427-.177V6.25a.25.25 0 00-.25-.25H6.604a.25.25 0 00-.177.427L7.97 7.97z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD4UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+b/lr4KeBrwF2uQLxbO8NfBVwnP/bdoGPAb4bQFzx3sB38f/L+wDfLeA48HTgOP+/7AIPEfDZwGfx/9PnCPgr4KX5/+mvBZjn72WAv+Zfzzx/4r/PSwN/xfNCgHn+xL+Nef7Efy/zvBBgnj/xb2OeP/HfyzwvBJjnT/zbmOdP/PcyzwsB5vkT/zbm+RP/vczzQoB5/sS/jXn+xH8v87wQYJ4/8W9jnj/x38s8LwSY50/825jnT/z3Ms8LAeb5E/825vkT/73M80KAef7Ev415/sR/L/O8EGCeP/FvY54/8d/LPC8EmOdP/NuY50/89zLPCwHm+RP/Nub5E/+9zPNCgHn+xL+Nef7Efy/zvBBg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb5E/825vkT/73M80KAef7Ev415/sR/L/O8EGCeP/FvY54/8d/LPC8EmOdP/NuY50/89zLPCwHm+RP/Nub5E/+9zPNCgHn+xL+Nef7Efy/zvBBgnj/xb2OeP/HfyzwvBJjnT/zbmOdP/PcyzwsB5vkT/zbm+RP/vczzQoB5/sS/jXn+xH8v87wQYJ4/8W9jnj/x38s8LwSY50/825jnT/z3Ms8LAeb5E/825vkT/73M80KAef7Ev415/sR/L/O8EGCeP/FvY54/8d/LPC8EmOdP/NuY50/89zLPCwHm+RP/Nub5E/+9zPNCgHn+xL+Nef7Efy/zvBBgnj/xb2OeP/HfyzwvBJjnT/zbmOdP/PcyzwsB5vkT/zbm+RP/vczzQoB5/sS/jXn+xH8v87wQYJ4/8W9jnj/x38s8LwSY50/825jnT/z3Ms8LAeb5E/825vkT/73M80KAef7Ev415/sR/L/O8EGCeP/FvY54/8d/LPC8EmOdP/NuY50/89zLPCwHm+RP/Nub5E/+9zPNCgHn+xL+Nef7Efy/zvBBgnj/xb2OeP/HfyzwvBJjn72WAv+Zfzzx/4r/PSwN/xfNCwF8DL8X/T38j4LOBz+L/p88RcBy4FTjG/y+XgAeLK94b+C7+f3kb4KfFs7038NXAMf5vuwS8N/DTAOI5HQc+Gnhr4KX4v+VvgJ8GvhrY5Qr+EbiXrsy0yFkxAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTable;
impl IconShape for GoTable {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v3.585a.746.746 0 010 .83v8.085A1.75 1.75 0 0114.25 16H6.309a.748.748 0 01-1.118 0H1.75A1.75 1.75 0 010 14.25V6.165a.746.746 0 010-.83V1.75zM1.5 6.5v7.75c0 .138.112.25.25.25H5v-8H1.5zM5 5H1.5V1.75a.25.25 0 01.25-.25H5V5zm1.5 1.5v8h7.75a.25.25 0 00.25-.25V6.5h-8zm8-1.5h-8V1.5h7.75a.25.25 0 01.25.25V5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTag;
impl IconShape for GoTag {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFB0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4gX7jjwUcBbAy/N/y5/Dfw08DXALs8f4gV7b+CrgOP877YLfAzw3TwvxPP33sB38X/L+wDfzXNCPK/jwNOB4/zfsgs8BNjl2RDP67OBz+L/ps8BPptnQzyvvwJemv+b/hp4GZ4N8bzM8/cywF/zv8NLA3/F8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLPn/jfxTx/4tkQz8s8f+J/F/P8iWdDPC/z/In/XczzJ54N8bzM8yf+dzHPn3g2xPMyz5/47/dZwFtzxXcDX8MLZp4/8WyI52WeP/Hf67uA9+Y5fQzw1Tx/5vkTz4Z4Xub5E/99vgt4b57XXwMvw/Nnnj/xbIjnZZ4/8d/ju4D35vn7G+Clef7M8yeeDfG8zPMn/ut9F/DevGCfA3w2z595/sSzIZ6Xef7Ef63vAt6bF+x7gPfmBTPPn3g2xPMyz5/4r/NdwHvzgn0P8N68cOb5E8+GeF7m+RP/Nb4LeG9esO8B3pt/mXn+xLMhnpd5/sR/vu8C3psX7HuA9+ZFY54/8WyI52WeP/Gf67uA9+YF+x7gvXnRmedPPBvieZnnT7xgHwW8N1f8NPA5/Ot8F/DevGDfA7w3/zrm+RPPhnhe5vkTz99HA1/Fc/pu4H140XwX8N68YN8DvDf/eub5E8+GeF7m+RPP318BL83z+m7gfXjhvgt4b16w7wHem38b8/yJZ0M8L/P8iefvr4GX4vn7buB9eP6+C3hvXrDvAd6bfzvz/IlnQzwv8/yJ5++zgc/iBftu4H14Tt8FvDcv2PcA782/j3n+xLMhnpd5/sQL9t3Ae/GCfTfwPlzxXcB784J9D/De/PuZ5088G+J5medPvHDfDbwXL9h3c8V784J9D/De/Mcwz594NsTzMs+f+Jd9N/Be/Nt8D/De/Mcxz594NsTzMs+feNF8N/Be/Ot8D/De/Mcyz594NsTzMs+feNF9N/BevGi+B3hv/uOZ5088G+J5medP/Ot8N/BevHDfA7w3/znM8yeeDfG8zPMn/vW+G3gvnr/vAd6b/zzm+RPPhnhe5vkT/zbfDbwXz+l7gPfmP5d5/sSzIZ6Xef7Ev91nA+/NFd8NfDb/+czzJ54N8bzM8yf+dzHPn3g2xPMyz5/438U8f+LZEM/LPH/ifxfz/IlnQzwv8/yJ/13M8yeeDfG8zPMn/ncxz594NsTzMs+f+N/FPH/i2RDPyzx/4n8X8/yJZ0M8L/P8if9dzPMnng3xvMzzJ/53Mc+feDbE8zLP38sAf83/Di8N/BXPn3g2xPP6a+Cl+L/pb4CX5tkQz+uzgc/i/6bPAT6bZ0M8r+PArcAx/m+5BDwY2OXZEM/fewPfxf8tbwP8NM8J8YK9N/DVwDH+d7sEvDfw0zwvxAt3HPho4K2Bl+J/l78Bfhr4amCX5w/x/xvi/zfE/2+I/9/4Rw5pt0GXH5hPAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTasklist;
impl IconShape for GoTasklist {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.5 2.75a.25.25 0 01.25-.25h10.5a.25.25 0 01.25.25v10.5a.25.25 0 01-.25.25H2.75a.25.25 0 01-.25-.25V2.75zM2.75 1A1.75 1.75 0 001 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0015 13.25V2.75A1.75 1.75 0 0013.25 1H2.75zm9.03 5.28a.75.75 0 00-1.06-1.06L6.75 9.19 5.28 7.72a.75.75 0 00-1.06 1.06l2 2a.75.75 0 001.06 0l4.5-4.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTelescope;
impl IconShape for GoTelescope {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M14.184 1.143a1.75 1.75 0 00-2.502-.57L.912 7.916a1.75 1.75 0 00-.53 2.32l.447.775a1.75 1.75 0 002.275.702l11.745-5.656a1.75 1.75 0 00.757-2.451l-1.422-2.464zm-1.657.669a.25.25 0 01.358.081l1.422 2.464a.25.25 0 01-.108.35l-2.016.97-1.505-2.605 1.85-1.26zM9.436 3.92l1.391 2.41-5.42 2.61-.942-1.63 4.97-3.39zM3.222 8.157l-1.466 1a.25.25 0 00-.075.33l.447.775a.25.25 0 00.325.1l1.598-.769-.83-1.436zm6.253 2.306a.75.75 0 00-.944-.252l-1.809.87a.75.75 0 00-.293.253L4.38 14.326a.75.75 0 101.238.848l1.881-2.75v2.826a.75.75 0 001.5 0v-2.826l1.881 2.75a.75.75 0 001.238-.848l-2.644-3.863z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTelescopeFill;
impl IconShape for GoTelescopeFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.531 10.21a.75.75 0 01.944.253l2.644 3.864a.75.75 0 11-1.238.847L9 12.424v2.826a.75.75 0 01-1.5 0v-2.826l-1.881 2.75a.75.75 0 01-1.238-.848l2.048-2.992a.75.75 0 01.293-.252l1.81-.871zM11.905.42a1.5 1.5 0 012.144.49l1.692 2.93a1.5 1.5 0 01-.649 2.102L2.895 11.815a1.5 1.5 0 01-1.95-.602l-.68-1.176a1.5 1.5 0 01.455-1.99L11.905.422zM3.279 8.119l.835 1.445 1.355-.653-.947-1.64-1.243.848zm7.728-1.874L9.6 3.808l1.243-.848 1.52 2.631-1.356.653z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAERklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jndBz4KOCtgZfm/5a/Bn4a+BpglysQz/bewFcBx/m/bRf4GOC7AcQV7w18F/+/vA/w3QKOA08HjvP/yy7wEAGfDXwW/z99joC/Al6a/5/+WoB5/l4G+Gv+b3hp4K94Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT7xwx4HvAt6aK74a+Bj+5zLPCwHm+RMv3E8Db8Vz+m7gffifyTwvBJjnT7xw5vn7buB9+J/HPC8EmOdPvHDmBftu4H34n8U8LwSY50+8cF8NfBQv2HcD78P/HOZ5IcA8f+Jf9t3Ae/GCfTfwPvzPYJ4XAszzJ1403w28Fy/YdwPvw38/87wQYJ4/8aL7buC9eMG+G3gf/nuZ54UA8/yJf53vBt6LF+y7gffhv495Xggwz5/41/tu4L14wb4beB/+e5jnhQDz/Il/m+8G3osX7HOAz+a/nnleCDDPn/i3+27gvXj+bgUewn8987wQYJ4/8W/3XcB78/w9A3gw//XM80KAef7Ev813Ae/NC/Y5wGfzX888LwSY50/8630X8N68YN8DvDf/PczzQoB5/sS/zncB780L9j3Ae/PfxzwvBJjnT7zovgt4b16w7wHem/9e5nkhwDx/4kXzXcB784J9D/De/PczzwsB5vkT/7LvAt6bF+x7gPfmhTsOfBbw1sCD+de5Ffhu4HP4l5nnhQDz/IkX7quBj+IF+x7gvfmXfTXwUfz7fA3w0bxw5nkhwDx/4oUzL9j3AO/Ni+YicJx/n1uBh/DCmeeFAPP8iRfOPH/fA7w3L7pd4Bj/Ps8AHswLZ54XAszzJ164nwbeiuf0PcB786/z2cBn8e/zOcBn88KZ54UA8/yJF+448N3AWwGXgO8GPpp/m68G3hp4EP86zwC+G/hs/mXmeSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCev5cB/pr/G14a+CueFwL+Gngp/n/6GwGfDXwW/z99joDjwK3AMf5/uQQ8WFzx3sB38f/L2wA/LZ7tvYGvBo7xf9sl4L2BnwYQz+k48NHAWwMvxf8tfwP8NPDVwC5XIP5/Q/z/hvj/DfH/G/8IDl+11G/Ne5oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTerminal;
impl IconShape for GoTerminal {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H1.75zM7.25 8a.75.75 0 01-.22.53l-2.25 2.25a.75.75 0 11-1.06-1.06L5.44 8 3.72 6.28a.75.75 0 111.06-1.06l2.25 2.25c.141.14.22.331.22.53zm1.5 1.5a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACSklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4XseBzwLeGngw/zfcCnw38Dk8J8Tz+mrgo/i/6WuAj+bZEM/rInCc/5tuBR7CsyGe1y5wjP+bngE8mGdDPK/PBj6L/5s+B/hsng3x/H018NbAg/i/4RnAdwOfzXNC/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xDP6zjwWcBbAw/m/4Zbge8GPofnhHheXw18FP83fQ3w0Twb4nldBI7zf9OtwEN4NsTz2gWO8X/TM4AH82yI5/XZwGfxf9PnAJ/NsyGev68G3hp4EP83PAP4buCzeU6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+J5HQc+C3hr4MH833Ar8N3A5/CcEM/rq4GP4v+mrwE+mmdDPK+LwHH+b7oVeAjPhnheu8Ax/m96BvBgng3xvD4b+Cz+b/oc4LN5NsTz99XAWwMP4v+GZwDfDXw2zwnx/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83/hE7YypB8w62/AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThreeBars;
impl IconShape for GoThreeBars {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 100 1.5h12.5a.75.75 0 100-1.5H1.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThumbsdown;
impl IconShape for GoThumbsdown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.083 15.986c1.34.153 2.334-.982 2.334-2.183v-.5c0-1.329.646-2.123 1.317-2.614.329-.24.66-.403.919-.508a1.75 1.75 0 001.514.872h1a1.75 1.75 0 001.75-1.75v-7.5a1.75 1.75 0 00-1.75-1.75h-1a1.75 1.75 0 00-1.662 1.2c-.525-.074-1.068-.228-1.726-.415L9.305.705C8.151.385 6.765.053 4.917.053c-1.706 0-2.97.152-3.722 1.139-.353.463-.537 1.042-.669 1.672C.41 3.424.32 4.108.214 4.897l-.04.306c-.25 1.869-.266 3.318.188 4.316.244.537.622.943 1.136 1.2.495.248 1.066.334 1.669.334h1.422l-.015.112c-.07.518-.157 1.17-.157 1.638 0 .921.151 1.718.655 2.299.512.589 1.248.797 2.011.884zm4.334-13.232c-.706-.089-1.39-.284-2.072-.479a63.914 63.914 0 00-.441-.125c-1.096-.304-2.335-.597-3.987-.597-1.794 0-2.28.222-2.529.548-.147.193-.275.505-.393 1.07-.105.502-.188 1.124-.295 1.93l-.04.3c-.25 1.882-.19 2.933.067 3.497a.921.921 0 00.443.48c.208.104.52.175.997.175h1.75c.685 0 1.295.577 1.205 1.335-.022.192-.049.39-.075.586-.066.488-.13.97-.13 1.329 0 .808.144 1.15.288 1.316.137.157.401.303 1.048.377.307.035.664-.237.664-.693v-.5c0-1.922.978-3.127 1.932-3.825a5.862 5.862 0 011.568-.809V2.754zm1.75 6.798a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoThumbsup;
impl IconShape for GoThumbsup {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.834.066C7.494-.087 6.5 1.048 6.5 2.25v.5c0 1.329-.647 2.124-1.318 2.614-.328.24-.66.403-.918.508A1.75 1.75 0 002.75 5h-1A1.75 1.75 0 000 6.75v7.5C0 15.216.784 16 1.75 16h1a1.75 1.75 0 001.662-1.201c.525.075 1.067.229 1.725.415.152.043.31.088.475.133 1.154.32 2.54.653 4.388.653 1.706 0 2.97-.153 3.722-1.14.353-.463.537-1.042.668-1.672.118-.56.208-1.243.313-2.033l.04-.306c.25-1.869.265-3.318-.188-4.316a2.418 2.418 0 00-1.137-1.2C13.924 5.085 13.353 5 12.75 5h-1.422l.015-.113c.07-.518.157-1.17.157-1.637 0-.922-.151-1.719-.656-2.3-.51-.589-1.247-.797-2.01-.884zM4.5 13.3c.705.088 1.39.284 2.072.478l.441.125c1.096.305 2.334.598 3.987.598 1.794 0 2.28-.223 2.528-.549.147-.193.276-.505.394-1.07.105-.502.188-1.124.295-1.93l.04-.3c.25-1.882.189-2.933-.068-3.497a.922.922 0 00-.442-.48c-.208-.104-.52-.174-.997-.174H11c-.686 0-1.295-.577-1.206-1.336.023-.192.05-.39.076-.586.065-.488.13-.97.13-1.328 0-.809-.144-1.15-.288-1.316-.137-.158-.402-.304-1.048-.378C8.357 1.521 8 1.793 8 2.25v.5c0 1.922-.978 3.128-1.933 3.825a5.861 5.861 0 01-1.567.81V13.3zM2.75 6.5a.25.25 0 01.25.25v7.5a.25.25 0 01-.25.25h-1a.25.25 0 01-.25-.25v-7.5a.25.25 0 01.25-.25h1z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTools;
impl IconShape for GoTools {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.433 2.304A4.494 4.494 0 003.5 6c0 1.598.832 3.002 2.09 3.802.518.328.929.923.902 1.64v.008l-.164 3.337a.75.75 0 11-1.498-.073l.163-3.33c.002-.085-.05-.216-.207-.316A5.996 5.996 0 012 6a5.994 5.994 0 012.567-4.92 1.482 1.482 0 011.673-.04c.462.296.76.827.76 1.423v2.82c0 .082.041.16.11.206l.75.51a.25.25 0 00.28 0l.75-.51A.25.25 0 009 5.282V2.463c0-.596.298-1.127.76-1.423a1.482 1.482 0 011.673.04A5.994 5.994 0 0114 6a5.996 5.996 0 01-2.786 5.068c-.157.1-.209.23-.207.315l.163 3.33a.75.75 0 11-1.498.074l-.164-3.345c-.027-.717.384-1.312.902-1.64A4.496 4.496 0 0012.5 6a4.494 4.494 0 00-1.933-3.696c-.024.017-.067.067-.067.16v2.818a1.75 1.75 0 01-.767 1.448l-.75.51a1.75 1.75 0 01-1.966 0l-.75-.51A1.75 1.75 0 015.5 5.282V2.463c0-.092-.043-.142-.067-.159zm.01-.005z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdRz4KOCtgZfmX+evgZ8GvgbY5T8H4j/PewNfBRzn32cX+Bjgu/mPh/jP8d7Ad/Ef632A7+Y/FuI/3nHg6cBx/mPtAg8BdvmPg/iP99nAZ/Gf43OAz+Y/DuI/3l8BL81/jr8GXob/OIj/eOb5exngr3nRvDTwVzx/4j8O4j+eef7Ev455/sR/HMR/PPP8iX8d8/yJ/ziI/3jm+RP/Oub5E/9xEP/xzPMn/nXM8yf+4yD+45nnT/zrmOdP/MdB/MuOA58FvDXwYP53uBX4buBzeOEQ/7KvBj6K/52+BvhoXjDEv+wicJz/nW4FHsILhviX7QLH+N/pGcCDecEQ/7LPBj6L/50+B/hsXjDEi+argbcGHsT/Ds8Avhv4bF44xP9viP/fEP+/If5/Q/z/hvj/DfGvdxz4KuC9ueK7gY8Bdvmv91nAWwPHge8GPod/HcS/3ncD78Vz+h7gvfmv9dXAR/GcPgf4bF50iH8987x2gRP817oIHOc53Qo8hBcd4l/PPH/iv5Z5/sSLDvGvdyvwIJ7XywB/zX+N1wZ+i+f1N8BL86JD/Ov9NvBaPK/XAX6b/xqvDfwWz+t3gNfmRYf41/tt4LV4Xm8D/DT/Nd4b+C6e188Ab82LDvGv99nAZ/G8Pgf4bP5rfDbwWTyvzwE+mxcd4l/vs4HP4nl9DvDZ/Nf4bOCzeF6fA3w2LzrEv957A9/F8/oZ4K35r/HbwGvxvN4G+GledIh/vdcGfovn9TvAa/Nf47eB1+J5vQ7w27zoEP96rw38Fs/rr4GX4b/G04EH87xeBvhrXnSIfxvz/In/Gub5E/86iH8b8/yJ/xrm+RP/Ooh/m1uBB/G8Xgb4a/5zvTbwWzyvvwFemn8dxL/NbwOvxfN6HeC3+c/12sBv8bx+B3ht/nUQ/za/DbwWz+ttgJ/mP9d7A9/F8/oZ4K3510H823w28Fk8r88BPpv/XJ8NfBbP63OAz+ZfB/Fv89nAZ/G8Pgf4bP5zfTbwWTyvzwE+m38dxL/NewPfxfP6GeCt+c/128Br8bzeBvhp/nUQ/zavDfwWz+t3gNfmP9dvA6/F83od4Lf510H827w28Fs8r78GXob/XE8HHszzehngr/nXQfzbmedP/Ocyz5/410P825nnT/znMs+f+NdD/NvdCjyI5/UywF/zn+O1gd/ief0N8NL86yH+7X4beC2e1+sAv81/jtcGfovn9TvAa/Ovh/i3+23gtXhebwP8NP853hv4Lp7XzwBvzb8e4t/us4HP4nl9DvDZ/Of4bOCzeF6fA3w2/3qIf7vPBj6L5/U5wGfzn+Ozgc/ieX0O8Nn86yH+7d4b+C6e163AywC7/Mc6DvwV8GCe19sAP82/HuLf7qWBv+J/hpcB/pp/PcS/z63Ag/jv9QzgwfzbIP59Phr4Kv57fQzw1fzbIP79vht4L/57fA/w3vzbIf79jgNfDbwX/7W+B/hoYJd/O8R/nNcG3ht4a+AY/zkuAT8NfDfw2/z7If5/Q/z/hvj/DfH/G/8IpaCqQWKly3QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTrash;
impl IconShape for GoTrash {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.5 1.75a.25.25 0 01.25-.25h2.5a.25.25 0 01.25.25V3h-3V1.75zm4.5 0V3h2.25a.75.75 0 010 1.5H2.75a.75.75 0 010-1.5H5V1.75C5 .784 5.784 0 6.75 0h2.5C10.216 0 11 .784 11 1.75zM4.496 6.675a.75.75 0 10-1.492.15l.66 6.6A1.75 1.75 0 005.405 15h5.19c.9 0 1.652-.681 1.741-1.576l.66-6.6a.75.75 0 00-1.492-.149l-.66 6.6a.25.25 0 01-.249.225h-5.19a.25.25 0 01-.249-.225l-.66-6.6z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/GG8N/BT/td4G+Gn+fRD/Md4a+Cn+a70N8NP8+yD+47w38F3813gf4Lv590P8x3pv4Lv4z/U+wHfzHwPxH++9ge/iP8f7AN/NfxzEf473Br6L/1jvA3w3/7EQ/3neG/gu/mO8D/Dd/MdD/Od6b+C7+Pd5H+C7+c+B+M/33sB38W/zPsB3858H8V/jvYHv4l/nfYDv5j8X4r/OewPfxYvmfYDv5j8f4r/WewPfxQv3PsB3818D8V/vvYHv4vl7H+C7+a+D+O/x3sB38ZzeB/hu/msh/vu8N/BdXPE+wHfzXw/x3+utueKn+e+B+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8IwYhH0GZ7d+oAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleDown;
impl IconShape for GoTriangleDown {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.427 7.427l3.396 3.396a.25.25 0 00.354 0l3.396-3.396A.25.25 0 0011.396 7H4.604a.25.25 0 00-.177.427z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8T/XewGXgJ/mPw/if6bvAt6bK8R/HsT/PN8FvDfPJv7zIP5n+S7gvXlO4j8P4n+O7wLem+cl/vMg/mf4LuC9ef7Efx7Ef7/vAt6bF0z850H89/ou4L154cR/HsR/n+8C3pt/mfjPg/jv8V3Ae/OiEf95EP/1vgt4b1504j8P4r/WdwHvzb+O+M+D+K/zXcB7868n/vMg/mt8F/De/NuI/zyI/3zvDXwX/3biPw/iP99bAz/Fv534z4P4r/HewHfxbyP+8yD+67w38F3864n/PIj/Wu8NfBf/OuI/D+K/3nsD38WLTvznQfz3eG/gu3jRiP88iP8+7w18F/8y8Z8H8d/rvYHv4oUT/3kQ//3eG/guXjDxnwfxP8N7A9/F8yf+8yD+53hv4Lt4XuI/D+J/lvcGvovnJP7zIP7neW/gu3g28Z8H8T/TewPfxRXiPw/if6635oqf5j8P4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EboWH0GbpotSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleLeft;
impl IconShape for GoTriangleLeft {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.573 4.427L6.177 7.823a.25.25 0 000 .354l3.396 3.396a.25.25 0 00.427-.177V4.604a.25.25 0 00-.427-.177z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8Z/vrYFjwPfwPw/iP5+54ruB9+F/FsR/PvNs3w28D/9zIP7zmef03cD78D8D4j+feV7fDbwP//0Q//nM8/fdwPvw3wvxn8+8YN8NvA//fRD/+cwL993A+/DfA/Gfz/zLvht4H/7rIf7zmRfNdwPvw38txH8+86L7buB9+K+D+M9n/nW+G3gf/msg/vOZf73vBt6H/3yI/3zm3+a7gffhPxfiP5/5t3sf4Lv5z4P4z2f+7d4G+Gn+8yD+85l/m/cBvpv/XIj/fOZf732A7+Y/H+I/n/nXeR/gu/mvgfjPZ1507wN8N/91EP/5zIvmfYDv5r8W4j+f+Ze9D/Dd/NdD/OczL9z7AN/Nfw/Efz7zgr0P8N3890H85zPP3/sA381/L8R/PvO83gf4bv77If7zmef0PsB38z8D4j+febb3Ab6b/zkQ//nMFe8DfDf/syD+8701V/w0//Mg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAYs+OkFWEw05AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleRight;
impl IconShape for GoTriangleRight {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.427 4.427l3.396 3.396a.25.25 0 010 .354l-3.396 3.396A.25.25 0 016 11.396V4.604a.25.25 0 01.427-.177z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvjv9V5c8T3890D89/ku4L254ruB9+G/HuK/x3cB781z+m7gffivhfiv913Ae/P8fTfwPvzXQfzX+i7gvXnhvht4H/5rIP7rfBfw3rxovht4H/7zIf5rfBfw3vzrfDfwPvznQvzn+y7gvfm3+W7gffjPg/jP9V3Ae/Pv893A+/CfA/Gf57uA9+Y/xncD78N/PMR/ju8C3pv/WN8NvA//sRD/8b4LeG/+c3w38D78x0H8x/ou4L35z/XdwPvwHwPxH+e7gPfmv8Z3A+/Dvx/iP8Z7A9/Ff633Ab6bfx/Ef4y3Bn6K/1pvA/w0/z6I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I5QUH0FdfdCgAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTriangleUp;
impl IconShape for GoTriangleUp {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M4.427 9.573l3.396-3.396a.25.25 0 01.354 0l3.396 3.396a.25.25 0 01-.177.427H4.604a.25.25 0 01-.177-.427z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTrophy;
impl IconShape for GoTrophy {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.217 6.962A3.75 3.75 0 010 3.25v-.5C0 1.784.784 1 1.75 1h1.356c.228-.585.796-1 1.462-1h6.864a1.57 1.57 0 011.462 1h1.356c.966 0 1.75.784 1.75 1.75v.5a3.75 3.75 0 01-3.217 3.712 5.014 5.014 0 01-2.771 3.117l.144 1.446c.005.05.03.12.114.204.086.087.217.17.373.227.283.103.618.274.89.568.285.31.467.723.467 1.226v.75h1.25a.75.75 0 110 1.5H2.75a.75.75 0 010-1.5H4v-.75c0-.503.182-.916.468-1.226.27-.294.606-.465.889-.568a1.03 1.03 0 00.373-.227c.084-.085.109-.153.114-.204l.144-1.446a5.014 5.014 0 01-2.77-3.117zM3 2.5H1.75a.25.25 0 00-.25.25v.5c0 .98.626 1.813 1.5 2.122V2.5zm4.457 7.97l-.12 1.204c-.093.925-.858 1.47-1.467 1.691a.764.764 0 00-.3.176c-.037.04-.07.093-.07.21v.75h5v-.75c0-.117-.033-.17-.07-.21a.763.763 0 00-.3-.176c-.609-.221-1.374-.766-1.466-1.69l-.12-1.204a5.052 5.052 0 01-1.087 0zM13 5.373V2.5h1.25a.25.25 0 01.25.25v.5A2.25 2.25 0 0113 5.372zM4.5 1.568c0-.037.03-.068.068-.068h6.864c.037 0 .068.03.068.068V5.5a3.5 3.5 0 11-7 0V1.568z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoTypography;
impl IconShape for GoTypography {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.21 8.5L4.574 3.594 2.857 8.5H6.21zm.5 1.5l.829 2.487a.75.75 0 001.423-.474L5.735 2.332a1.216 1.216 0 00-2.302-.018l-3.39 9.688a.75.75 0 001.415.496L2.332 10H6.71zm3.13-4.358C10.53 4.374 11.87 4 13 4c1.5 0 3 .939 3 2.601v5.649a.75.75 0 01-1.448.275C13.995 12.82 13.3 13 12.5 13c-.77 0-1.514-.231-2.078-.709-.577-.488-.922-1.199-.922-2.041 0-.694.265-1.411.887-1.944C11 7.78 11.88 7.5 13 7.5h1.5v-.899c0-.54-.5-1.101-1.5-1.101-.869 0-1.528.282-1.84.858a.75.75 0 11-1.32-.716zM14.5 9H13c-.881 0-1.375.22-1.637.444-.253.217-.363.5-.363.806 0 .408.155.697.39.896.249.21.63.354 1.11.354.732 0 1.26-.209 1.588-.449.35-.257.412-.495.412-.551V9z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEl0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+K/13txxffw3wPx3+e7gPfmiu8G3of/eoj/Ht8FvDfP6buB9+G/FuK/3ncB783z993A+/BfB/Ff67uA9+aF+27gffivgfiv813Ae/Oi+W7gffjPh/iv8V3Ae/Ov893A+/CfC/Gf77uA9+bf5ruB9+E/D+I/13cB782/z3cD78N/DsR/nu8C3pv/GN8NvA//8RD/Ob4LeG/+Y3038D78x0L8x/su4L35z/HdwPvwHwfxH+u9ge/iP9f7AN/NfwzEf6y3Bn6K/1xvA/w0/zEQ/z3M8yf+ayH+e5jnT/zXQvz3MM+f+K+F+O9hnj/xXwvx38M8f+K/FuK/xy5wjOf0DODB/NdC/Pf4bOCzeE6fA3w2/7UQ/32+Gnhrrvhu4LP5r4f4/w3x/xvi/zfE/2+I/98Qz+s48FnAWwMP5opbge8GPofndRz4LOCtgQdzxa3AdwOfw/M6DnwW8NbAg7niVuC7gc/heR0HPgt4a+DBXHEr8N3A5/C8jgOfBbw18GCuuBX4buBzeE6I5/XVwEfx/H0N8NE8p68GPorn72uAj+Y5fTXwUTx/XwN8NM/pq4GP4vn7GuCjeU5fDXwUz9/XAB/NsyGe10XgOM/frcBDeE4XgeM8f7cCD+E5XQSO8/zdCjyE53QROM7zdyvwEJ7TReA4z9+twEN4NsTz2gWO8fw9A3gwz2kXOMbz9wzgwTynXeAYz98zgAfznHaBYzx/zwAezHPaBY7x/D0DeDDPhnhenw18Fs/f5wCfzXP6bOCzeP4+B/hsntNnA5/F8/c5wGfznD4b+Cyev88BPpvn9NnAZ/H8fQ7w2Twb4vn7auCtgQdxxTOA7wY+m+fvq4G3Bh7EFc8Avhv4bJ6/rwbeGngQVzwD+G7gs3n+vhp4a+BBXPEM4LuBz+b5+2rgrYEHccUzgO8GPpvnhPj/DfH/G+L/N8T/b4j/3xD/PY4DnwW8NVd8N/A5/NdD/Pf4auCjeE5fA3w0/7UQ/z0uAsd5TrcCD+G/FuK/h3n+xH8txH8P8/yJ/1qI/x7m+RP/tRD/PczzJ/5rIf57mOdP/NdC/Md6a+Cn+M/1NsBP8x8D8R/rrYGf4j/X2wA/zX8MxH+89wa+i/8c7wN8N/9xEP853hv4Lv5jvQ/w3fzHQvzneW/gu/iP8T7Ad/MfD/Gf672B7+Lf532A7+Y/B+I/33sD38W/zfsA381/HsR/jfcGvot/nfcBvpv/XIj/Ou8NfBcvmvcBvpv/fIj/Wu8NfBcv3PsA381/DcR/vfcGvovn732A7+a/DuK/x3sD38Vzeh/gu/mvhfjv897Ad3HF+wDfzX89xH+vt+aKn+a/B+L/N8T/b/wjHTaSQQY4a5wAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnfold;
impl IconShape for GoUnfold {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.177.677l2.896 2.896a.25.25 0 01-.177.427H8.75v1.25a.75.75 0 01-1.5 0V4H5.104a.25.25 0 01-.177-.427L7.823.677a.25.25 0 01.354 0zM7.25 10.75a.75.75 0 011.5 0V12h2.146a.25.25 0 01.177.427l-2.896 2.896a.25.25 0 01-.354 0l-2.896-2.896A.25.25 0 015.104 12H7.25v-1.25zm-5-2a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM6 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 016 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5zM12 8a.75.75 0 01-.75.75h-.5a.75.75 0 010-1.5h.5A.75.75 0 0112 8zm2.25.75a.75.75 0 000-1.5h-.5a.75.75 0 000 1.5h.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEK0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv9V7AawNvDRznOd0K/Dbw28DPALv850P853sw8FnAWwPHedF9N/A5wK3850H85zkOfBTw2fz7fDXwMfznQPzneDDwU8BL8x/jr4HXAXb5j4X4j/fSwG8Bx/mPtQu8DvDX/MdB/Md6MPBXwHH+c9wKvAywy38MxH+svwJemhfue4CfBv4auJUrXhs4Drw38Fa8cL8NvA7/MRD/cT4b+CxesJ8BPhq4lRfuwcB3A6/FC/Y5wGfz74f4j/Fg4Om8YO8DfDf/Oh8NfBXP363AQ/j3Q/zH+G7gvXj+3gf4bv5tPhv4LJ7XM4AH8++H+Pc7DjwdOM7z+hngrfn3+W7gvXhOnwN8Nv9+iH+/twZ+iufvIcCt/PscBz4beG+u+Grgs/mPgfj3+2rgo3he3wO8N/+zIf79fht4LZ7X+wDfzf9siH+/pwMP5nk9BLiV/9kQ/37m+RP/8yH+/czzJ/7nQ/z7medP/M+H+Pczz5/4nw/x72eeP/E/H+Lfzzx/4n8+xL+fef7E/3yIfz/z/In/+RD/fub5E//zIf51Pgp4b+Cl+Z/pr4GvBr6HFw3iRXMc+C3gpfnf4a+B1wF2eeEQL5q/Al6a/13+GngZXjjEv+yjga/if6f3Ab6bFwzxL/sr4KX53+mvgZfhBUP8y8z/buIFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzPMn/mcxz594wRD/MvP8if9ZzPMnXjDEv8w8f+J/FvP8iRcM8S8zz5/4n8U8f+IFQ/zLzP9u4gVD/Mv+Gngp/nf6G+ClecEQ/7KPBr6K/53eB/huXjDEi+avgZfif5e/AV6aFw7xojkO/DbwUvzv8DfAawO7vHCIf533Bj4aeCn+Z/ob4KuB7+ZFg/j/DfH/G+L/N8T/b/wjMJKHQeNNdggAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnlock;
impl IconShape for GoUnlock {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.5 4a2.5 2.5 0 014.607-1.346.75.75 0 101.264-.808A4 4 0 004 4v2h-.501A1.5 1.5 0 002 7.5v6A1.5 1.5 0 003.5 15h9a1.5 1.5 0 001.5-1.5v-6A1.5 1.5 0 0012.5 6h-7V4zm-.75 3.5H3.5v6h9v-6H4.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnmute;
impl IconShape for GoUnmute {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.563 2.069A.75.75 0 018 2.75v10.5a.75.75 0 01-1.238.57L3.472 11H1.75A1.75 1.75 0 010 9.25v-2.5C0 5.784.784 5 1.75 5h1.723l3.289-2.82a.75.75 0 01.801-.111zM6.5 4.38L4.238 6.319a.75.75 0 01-.488.181h-2a.25.25 0 00-.25.25v2.5c0 .138.112.25.25.25h2a.75.75 0 01.488.18L6.5 11.62V4.38zm6.096-2.038a.75.75 0 011.06 0 8 8 0 010 11.314.75.75 0 01-1.06-1.06 6.5 6.5 0 000-9.193.75.75 0 010-1.06v-.001zm-1.06 2.121a.75.75 0 10-1.061 1.061 3.5 3.5 0 010 4.95.75.75 0 101.06 1.06 5 5 0 000-7.07l.001-.001z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUnverified;
impl IconShape for GoUnverified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M6.415.52a2.678 2.678 0 013.17 0l.928.68c.153.113.33.186.518.215l1.138.175a2.678 2.678 0 012.241 2.24l.175 1.138c.029.187.102.365.215.518l.68.928a2.678 2.678 0 010 3.17l-.68.928a1.179 1.179 0 00-.215.518l-.175 1.138a2.678 2.678 0 01-2.241 2.241l-1.138.175a1.179 1.179 0 00-.518.215l-.928.68a2.678 2.678 0 01-3.17 0l-.928-.68a1.179 1.179 0 00-.518-.215L3.83 14.41a2.678 2.678 0 01-2.24-2.24l-.175-1.138a1.179 1.179 0 00-.215-.518l-.68-.928a2.678 2.678 0 010-3.17l.68-.928a1.17 1.17 0 00.215-.518l.175-1.14a2.678 2.678 0 012.24-2.24l1.138-.175c.187-.029.365-.102.518-.215l.928-.68zm2.282 1.209a1.178 1.178 0 00-1.394 0l-.928.68a2.678 2.678 0 01-1.18.489l-1.136.174a1.178 1.178 0 00-.987.987l-.174 1.137a2.678 2.678 0 01-.489 1.18l-.68.927c-.305.415-.305.98 0 1.394l.68.928c.256.348.423.752.489 1.18l.174 1.136c.078.51.478.909.987.987l1.137.174c.427.066.831.233 1.18.489l.927.68c.415.305.98.305 1.394 0l.928-.68a2.678 2.678 0 011.18-.489l1.136-.174c.51-.078.909-.478.987-.987l.174-1.137c.066-.427.233-.831.489-1.18l.68-.927c.305-.415.305-.98 0-1.394l-.68-.928a2.678 2.678 0 01-.489-1.18l-.174-1.136a1.178 1.178 0 00-.987-.987l-1.137-.174a2.678 2.678 0 01-1.18-.489l-.927-.68zM9 11a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004c-.093.056-.204.122-.313.195a2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.862-1.725A2.76 2.76 0 008 4c-.631 0-1.154.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 001.342.67z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADw0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/Pp8FfDRXfDXwOfzXQ/z3+C7gvXlO3w28D/+1EP/1vgt4b56/7wbeh/86iP9a3wW8Ny/cdwPvw38NxH+d7wLemxfNdwPvw38+xH+N7wLem3+d7wbeh/9ciP983wW8N/823w28D/95EP+5vgt4b/59vht4H/5zIP7zfBfw3vzH+G7gffiPh/jP8V3Ae/Mf67uB9+E/FuI/3ncB780L9j3Ae/H8fQ/wXrxg3w28D/9xEP+xvgt4b16w7wHeGzDPn4DvBt6LF+y7gffhPwbiP853Ae/NC/Y9wHtzhXn+xBXfDbwXL9h3A+/Dvx/iP8Z3Ae/NC/Y9wHvzbOb5E8/23cB78YJ9N/A+/Psg/v2+C3hvXrDvAd6b52SeP/Gcvht4L16w7wbeh387xL/PdwHvzQv2PcB787zM8yee13cD78UL9t3A+/Bvg/i3+2zgs3jBvgd4b54/8/yJ5++7gffiBfsa4KP510P82/0V8NI8f98DvDcvmHn+xAv23cB78YKJfz3Ev91fAy/F8/oe4L154czzJ1647wbei+dP/Osh/u0+GvgqntP3AO/Nv8w8f+Jf9t3Ae/GcfgZ4a/71EP8+Hw28N1f8NPDZvGjM8ydeNF8NfBRXfA/w0cAu/3qI/x7m+RP/tRD/PczzJ/5rIf57mOdP/NdC/Pcwz5/4r4X472GeP/FfC/Hfwzx/4r8W4r+Hef7Efy3Efw/z/In/Woj/Hub5E/+1EP89zPMn/msh/nuY50/810L89zDPn/ivhfjvYZ4/8V8L8d/DPH/ivxbiv4d5/sR/LcR/D/P8if9aiP8eu8AxntMzgAfzXwvx3+Ozgc/iOX0O8Nn810L89/lq4K254ruBz+a/HuL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xviX+848FnAWwMP5n+GW4HvBj6Hfx3Ev95XAx/F/0xfA3w0LzrEv95F4Dj/M90KPIQXHeJfbxc4xv9MzwAezIsO8a/32cBn8T/T5wCfzYsO8W/z1cBbAw/if4ZnAN8NfDb/Ooj/3xD/vyH+f0P8/4b4/w3x/xv/CNvvbUEjEdHIAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoUpload;
impl IconShape for GoUpload {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M8.53 1.22a.75.75 0 00-1.06 0L3.72 4.97a.75.75 0 001.06 1.06l2.47-2.47v6.69a.75.75 0 001.5 0V3.56l2.47 2.47a.75.75 0 101.06-1.06L8.53 1.22zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVerified;
impl IconShape for GoVerified {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M9.585.52a2.678 2.678 0 00-3.17 0l-.928.68a1.178 1.178 0 01-.518.215L3.83 1.59a2.678 2.678 0 00-2.24 2.24l-.175 1.14a1.178 1.178 0 01-.215.518l-.68.928a2.678 2.678 0 000 3.17l.68.928c.113.153.186.33.215.518l.175 1.138a2.678 2.678 0 002.24 2.24l1.138.175c.187.029.365.102.518.215l.928.68a2.678 2.678 0 003.17 0l.928-.68a1.17 1.17 0 01.518-.215l1.138-.175a2.678 2.678 0 002.241-2.241l.175-1.138c.029-.187.102-.365.215-.518l.68-.928a2.678 2.678 0 000-3.17l-.68-.928a1.179 1.179 0 01-.215-.518L14.41 3.83a2.678 2.678 0 00-2.24-2.24l-1.138-.175a1.179 1.179 0 01-.518-.215L9.585.52zM7.303 1.728c.415-.305.98-.305 1.394 0l.928.68c.348.256.752.423 1.18.489l1.136.174c.51.078.909.478.987.987l.174 1.137c.066.427.233.831.489 1.18l.68.927c.305.415.305.98 0 1.394l-.68.928a2.678 2.678 0 00-.489 1.18l-.174 1.136a1.178 1.178 0 01-.987.987l-1.137.174a2.678 2.678 0 00-1.18.489l-.927.68c-.415.305-.98.305-1.394 0l-.928-.68a2.678 2.678 0 00-1.18-.489l-1.136-.174a1.178 1.178 0 01-.987-.987l-.174-1.137a2.678 2.678 0 00-.489-1.18l-.68-.927a1.178 1.178 0 010-1.394l.68-.928c.256-.348.423-.752.489-1.18l.174-1.136c.078-.51.478-.909.987-.987l1.137-.174a2.678 2.678 0 001.18-.489l.927-.68zM11.28 6.78a.75.75 0 00-1.06-1.06L7 8.94 5.78 7.72a.75.75 0 00-1.06 1.06l1.75 1.75a.75.75 0 001.06 0l3.75-3.75z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE2UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4r3Uc+CjgrYGX5r/WXwM/DXwNsMsViP867w18FXCc/167wMcA3w0g/mu8N/Bd/M/yPsB3i/98x4GnA8f5n2UXeIj49zkOvBdXfA+wy/P6bOCz+J/pc8S/3VsD3wUc54pd4HWAv+Y5/RXw0vzP9Nfi3+atgZ/ief018DI8J/P8vQzw1/zXeGngr3heiH+948DTgeM8f+I5medP/NcyzwvxnI4D78UV3wPs8rw+G/gsnr/fAV6b52SeP/FfyzwvxLO9N/BVwHGu2AVeB/hrntNvA6/F87oEvDbw1zwn8/yJ/1rmeSGueG/gu3hefw28DM/JPH8vA/w1z8s8f+K/lnleCHgw8FfAcZ4/8ZzM8yeeP/P8if9a5nkh4LuB9+L5+xvgpXlO5vkTz595/sR/LfO8EGCev0vAWwO/zXMyz594/szzJ/5rmeeFAPP8vQzw1zwv8/yJ5888f+K/lnleCDDPn3j+zPMnnj/z/In/WuZ5IcA8f+L5M8+feP7M8yf+a5nnhQDz/Innzzx/4vkzz5/4r2WeFwLM8yeeP/P8iefPPH/iv5Z5Xggwz594/szzJ54/8/yJ/1rmeSHAPH/i+TPPn3j+zPMn/muZ54UA8/yJ5888f+L5M8+f+K9lnhcCzPMnnj/z/Innzzx/4r+WeV4IMM+feP7M8yeeP/P8if9a5nkhwDx/4vkzz594/szzJ/5rmeeFAPP8iefPPH/i+TPPn/ivZZ4XAszzJ54/8/yJ5888f+K/lnleCDDPn3j+zPMnnj/z/In/WuZ5IcA8f+L5M8+feP7M8yf+a5nnhQDz/Innzzx/4vkzz5/4r2WeFwLM8yeeP/P8iefPPH/iv5Z5Xggwz594/szzJ54/8/yJ/1rmeSHAPH/i+TPPn3j+zPMn/muZ54UA8/y9DPDXPC/z/Innzzx/4r+WeV4I+G3gtXheu8DbAL/NczLPn3j+zPMn/muZ54WAzwY+i+fvr4GX4TmZ5088f+b5E/+1zPNCwHHgVuAYz594Tub5E8+fef7Efy3zvBBXvDXwUzyvvwFemudknr+XAf6a52WeP/FfyzwvxLO9NfDdwDGuuAS8NvDXPKffBl6L57ULvA3w2zwn8/yJ/1rmeSGe03Hgvbniu4FdntdnA5/F8/fXwMvwnMzzJ/5rmeeF+Nc7DtwKHOP5E8/JPH/iv5Z5Xoh/m7cGforn9TfAS/OczPP3MsBf81/jpYG/4nkh/u3eGvhu4BhXXAJeG/hrntNfAy/F/0x/I/59jgPvzRXfDezyvD4b+Cz+Z/oc8Z/vOHArcIz/WS4BDxb/Nd4b+C7+Z3kb4KfFf533Br4aOMZ/r0vAewM/DSD+ax0HPhp4a+Cl+K/1N8BPA18N7HIF4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IBKrO3ESSuxIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVersions;
impl IconShape for GoVersions {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M7.75 14A1.75 1.75 0 016 12.25v-8.5C6 2.784 6.784 2 7.75 2h6.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14h-6.5zm-.25-1.75c0 .138.112.25.25.25h6.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25h-6.5a.25.25 0 00-.25.25v8.5zM4.9 3.508a.75.75 0 01-.274 1.025.25.25 0 00-.126.217v6.5a.25.25 0 00.126.217.75.75 0 01-.752 1.298A1.75 1.75 0 013 11.25v-6.5c0-.649.353-1.214.874-1.516a.75.75 0 011.025.274zM1.625 5.533a.75.75 0 10-.752-1.299A1.75 1.75 0 000 5.75v4.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217v-4.5a.25.25 0 01.126-.217z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADrUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4TseBjwLeGnhp/m/5a+Cnga8BdrkC8WzvDXwVcJz/23aBjwG+G0Bc8d7Ad/H/y/sA3y3gOPB04Dj/v+wCDxHw2cBn8f/T5wj4K+Cl+f/prwWY5+9lgL/m/4aXBv6K54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/8x3gwV9zKfy/zvBBgnj/xH+MicBz4bOBrgF3+e5jnhQDz/In/GObZbgU+Bvhp/uuZ54UA8/yJ/xjmef028D7ArfzXMc8LAeb5E/8xzAv22cDXALv85zPPCwHm+RP/McwLdyvwMcBP85/LPC8EmOdP/McwL5rfBt4HuJX/HOZ5IcA8f+I/hvnX+Wzga4Bd/mOZ54UA8/yJ/xjmX+9W4GOAn+Y/jnleCDDPn/iPYf7tfht4H+BW/v3M80KAef7Efwzz7/fZwNcAu/zbmeeFAPP8if8Y5j/GXwMvw7+deV4IMM+f+I9h/mP8DfDS/NuZ54UA8/yJ/xjm3+cS8NXAZ/PvY54XAszzJ/5jmH+7nwE+GriVfz/zvBBgnj/xH8P86z0DeG/gt/mPY54XAszzJ/5jmBfdJeCrgc/mP555Xggwz5/4j2FeND8DfDRwK/85zPNCgHn+xH8M88I9A3hv4Lf5z2WeFwLM8yf+Y5jn7xLw1cBn81/DPC8EmOdP/Mcwz+tngI8GbuW/jnleCDDPn/iPYZ7tGcB7A7/Nfz3zvBBgnj/xH2OXK74a+Gz++5jnhQDz/In/GA/milv572WeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCev5cB/pr/G14a+CueFwL+Gngp/n/6GwGfDXwW/z99joDjwK3AMf5/uQQ8WFzx3sB38f/L2wA/LZ7tvYGvBo7xf9sl4L2BnwYQz+k48NHAWwMvxf8tfwP8NPDVwC5XIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPn1Kn3IxGWhcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoVideo;
impl IconShape for GoVideo {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M1.75 3.5a.25.25 0 00-.25.25v8.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25v-8.5a.25.25 0 00-.25-.25H1.75zM0 3.75C0 2.784.784 2 1.75 2h12.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0114.25 14H1.75A1.75 1.75 0 010 12.25v-8.5z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 10.559V5.442a.25.25 0 01.379-.215l4.264 2.559a.25.25 0 010 .428l-4.264 2.559A.25.25 0 016 10.559z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoWebhook;
impl IconShape for GoWebhook {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M5.5 4.25a2.25 2.25 0 014.5 0 .75.75 0 001.5 0 3.75 3.75 0 10-6.14 2.889l-2.272 4.258a.75.75 0 001.324.706L7 7.25a.75.75 0 00-.309-1.015A2.25 2.25 0 015.5 4.25z",
            }
            path {
                d: "M7.364 3.607a.75.75 0 011.03.257l2.608 4.349a3.75 3.75 0 11-.628 6.785.75.75 0 01.752-1.299 2.25 2.25 0 10-.033-3.88.75.75 0 01-1.03-.256L7.107 4.636a.75.75 0 01.257-1.03z",
            }
            path {
                d: "M2.9 8.776A.75.75 0 012.625 9.8 2.25 2.25 0 106 11.75a.75.75 0 01.75-.751h5.5a.75.75 0 010 1.5H7.425a3.751 3.751 0 11-5.55-3.998.75.75 0 011.024.274z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEQUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOx4GPAt4aeGn+ff4a+Gnga4Bd/mdCPNt7A18FHOc/1i7wMcB38z8P4or3Br6L/1zvA3w3/7Mg4DjwdOA4/7l2gYcAu/zPgYDPBj6L/xqfA3w2/3Mg4K+Al+a/xl8DL8P/HAgwz9/LAH/Nv81LA3/F8yf+50CAef7Ev495/sT/HAgwz5/49zHPn/ifAwHm+RP/Pub5E/9zIMA8f+Lfxzx/4n8OBJjnT/z7mOdP/M+BAPP8iX8f8/yJ/zkQYJ4/8e9jnj/xPwcCzPMn/n3M8yf+50CAef7Ev495/sT/HAgwz5/49zHPn/ifAwHm+RP/Pub5E/9zIMA8f+Lfxzx/4n8OBJjnT/z7mOdP/M+BAPP8iX8f8/yJ/zkQYJ6/lwH+mn+blwb+iudP/M+BgL8GXor/Gn8DvDT/cyDgs4HP4r/G5wCfzf8cCDgO3Aoc4z/XJeDBwC7/cyCueG/gu/jP9TbAT/M/C+LZ3hv4auAY/7EuAe8N/DT/8yCe03Hgo4G3Bl6Kf5+/AX4a+Gpgl/+ZEC868/yJ/70QLzrz/In/vRAvOvP8if+9EC868/yJ/70QLzrz/In/vRAvOvP8if+9EC868/yJ/70QLzrz/In/vRAvOvP8if88x4GPAt4aeGn+ff4a+Gnga4BdrkC86MzzJ/5zvDfwVcBx/mPtAh8DfDeAeNGZ50/8x3tv4Lv4z/U+wHeLF515/sR/rOPA04Hj/OfaBR4iXnTm+RP/sT4b+Cz+a3yOeNGZ50/8x/or4KX5r/HX4kVnnj/xH8s8fy8D/DX/Ni8N/BXPC/GiM8+f+I9lnj/x72OeF+JFZ56/E8Au/3HM8yf+fczzQrzo/hp4KZ7X2wA/zX8c8/yJfx/zvBAvuu8G3ovndSvwMsAu/zHM8yf+fczzQrzo3hv4Lp6/W4GPBv4GuJV/H/P8iX8f87wQ/zq3Ag/iv4f49zHPC/Gv89bAT/HfQ/z7mOeF+Nf7auCj+K8n/n3M80L823w18FH81xL/PuZ5If7t3hr4auBB/NcQ/z7meSH+/d4aeGvgpYGX4j+P+PcxzwvxP495/sS/j3leiP95zPMn/n3M80L8z2OeP/HvY54X4n8e8/y9DPDX/Nu8NPBXPC/E/zx/DbwU/zX+RvzP89nAZ/Ff43PE/zzHgVuBY/znugQ8WPzP9N7Ad/Gf622Anxb/c7038NXAMf5jXQLeG/hpAPE/23Hgo4G3Bl6Kf5+/AX4a+Gpglyv4RzM+m/C7X9UYAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoWorkflow;
impl IconShape for GoWorkflow {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M0 1.75C0 .784.784 0 1.75 0h3.5C6.216 0 7 .784 7 1.75v3.5A1.75 1.75 0 015.25 7H4v4a1 1 0 001 1h4v-1.25C9 9.784 9.784 9 10.75 9h3.5c.966 0 1.75.784 1.75 1.75v3.5A1.75 1.75 0 0114.25 16h-3.5A1.75 1.75 0 019 14.25v-.75H5A2.5 2.5 0 012.5 11V7h-.75A1.75 1.75 0 010 5.25v-3.5zm1.75-.25a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h3.5a.25.25 0 00.25-.25v-3.5a.25.25 0 00-.25-.25h-3.5zm9 9a.25.25 0 00-.25.25v3.5c0 .138.112.25.25.25h3.5a.25.25 0 00.25-.25v-3.5a.25.25 0 00-.25-.25h-3.5z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEO0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n0+C/horvhq4HP4r/FVwEdzxU8D7wPs8q+H+Lf7bOCzeE7fDbwP/7m+C3hvntPPAG/Nvx7i3+4icJzn9d3A+/Cf47uA9+b5E/96iH+7XeAYz993A+/Df6zvAt6bF0z86yH+7T4b+CxesO8G3of/GN8FvDcv2NcAH82/HuLf57uB9+IF+27gffj3+S7gvXnBvgd4b/5tEP9+3w28Fy/YdwPvw7/NdwHvzQv2PcB782+H+I/x3cB78YJ9N/A+/Ot8F/DevGDfA7w3/z6I/zjfDbwXL9h3A+/Di+a7gPfmBfse4L3590P8x/pu4L14wb4beB9euO8C3psX7HuA9+Y/BuI/3ncD78UL9t3A+/D8fRfw3rxg3wO8N/9xEP85vht4L16w7wbeh+f0XcB784J9D/De/MdC/Of5buC9eMG+G3gfrvgu4L15wb4HeG/+4yH+c3038F68YN/NFe/NC/Y9wHvznwPxn++7gffi3+Z7gPfmPw/iv8Z3A+/Fv873AO/Nfy7Ef53vBt6LF833AO/Nfz7Ef63vBt6LF+57gPfmvwbiv9Z3Ae/NC/fdwPvwXwPxX+e7gPfmRfPdwPvwnw/xX+O7gPfmX+e7gffhPxfiP993Ae/Nv813A+/Dfx7Ef67vAt6bF+x7uOK9eMG+G3gf/nMg/vN8F/DevGDfA7w3V3w38F68YN8NvA//8RD/Ob4LeG9esO8B3pvn9N3Ae/GCfTfwPvzHQvzH+y7gvXnBvgd4b56/7wbeixfsu4H34T8O4j/WdwHvzQv2PcB788J9N/BevGDfDbwP/zEQ/3G+C3hvXrDvAd6bF813A+/FC/bdwPvw74f4j/FdwHvzgn0P8N7863w38F68YN8NvA//Poh/v+8C3psX7HuA9+bf5ruB9+IF+27gffi3Q/z7fBfw3rxg3wO8N/8+3w28Fy/YdwPvw78N4t/us4HP4gX7HuC9+Y/x3cB78YJ9DfDR/Osh/u3+Cnhpnr/vAd6b/1jfDbwXL5j410P82/018FI8r+8B3pv/HN8NvBfPn/jXQ/zbfTTwVTyn7wHem/9c3w28F8/pZ4C35l8P8e/z0cB7c8VPA5/Nf42vBj6KK74H+Ghgl389xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/41/BNw5fEHFwNyTAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoX;
impl IconShape for GoX {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjP99LASwGvDTwYeG1esF3gr4Fbgd8G/gb4a/7zIP5zvBXw1sBrAw/m3+dW4LeBnwZ+hv9YiP84x4H3Aj4aeDD/OW4Fvhr4HmCXfz/Ev99x4KOAjwaO819jF/hq4GuAXf7tEP8+HwV8NnCc/x67wGcDX8O/DeLf5qWBrwJem3+f3wGOAy/Fv89vA+8D3Mq/DuJf772BrwKO86K5BPw08NfAXwO/zQv2YODBwGsDrw28Fi+6XeBjgO/mRYf41/kq4KP5l10Cfhr4aeCn+bc7Drw18NbAW/Gi+WrgY3jRIF503wW8Ny/cJeCrga8GdvmP9WDgs4H34l/23cD78C9DvGh+CnhrXrjPAb4a2OU/14OB7wZeixfup4G34YVD/Mu+C3hvXrC/Ad4b+Gv+a3008NnAMV6w7wbehxcM8cJ9F/DevGDfA7w3/31eGvhu4KV4wb4beB+eP8QL9t7Ad/GCfQ7w2fz3Ow78NvBSvGBvA/w0zwvx/L008FvAcZ6/9wG+m/85jgM/DbwWz98u8DLArTwnxPP3V8BL8/x9DvDZ/M9zHPht4KV4/n4beB2eE+J5fTTwVTx/3wO8N/9zHQf+GngQz9/7AN/NsyGe03Hg6cBxntffAC/N/3wvDfwVz98u8BBglysQz+mrgY/i+XsZ4K/53+Gzgc/i+fsc4LO5AvFsDwaezvP3OcBn87/LrcCDeF67wEOAXQDxbJ8NfBbP6xLwYGCXF81HAe/NFT8NfA7/Pl8FfDRX/DTwPsAu/7LXBn6L5+9jgK8GEM92ETjO8/oc4LN50Xw08FU8p+8G3od/m+8C3pvn9DPAW/Oi+W3gtXhetwIPARBXvDXwUzyvS8CDgV1eNH8FvDTP67uB9+Ff57uA9+b5Ey+a9wa+i+fvZYC/Fld8N/BePK/vAd6bF91fAy/F8/fdwPvwovku4L15/i4Bx3nR3Qo8iOf1NcBHiysuAsd5Xm8D/DQvus8GPosX7LuB9+GF+y7gvXnBvgb4aF50Xw18FM/rr4GXEfDSwF/xvC4Bx/nX+27gvXjBvht4H56/7wLemxfse4D35l/ntYHf4vk7IeCjga/ieX0P8N7823w38F68YN8NvA/P6buA9+YF+x7gvfm3Mc/f2wj4buC9eF4fA3w1/3bfDbwXL9h3A+/DFd8FvDcv2PcA782/3W8Dr8Xz+hwBvw28Fs/rdYDf5t/nu4H34gX7bq54b16w7wHem3+fzwY+i+f1MwLM8yf+Y3w38F7823wP8N78+3008FU8r98RYJ4/8R/nu4H34l/ne4D35j/GawO/xfO6VYB5/sR/rO8G3osXzfcA781/nNcGfovnhQDzvH4HeG3+43038F68cN8DvDf/sV4b+C2eFwLM8/pr4GX4j/ddwHvzwn038D78x3pt4Ld4Xggwz5/4j/VdwHvzovlu4H34j/PawG/xvBBgnj/xH+e7gPfmX+e7gffhP8ZbAz/F83qGAPP8PQS4lX+/7wLem3+b7wbeh3+/zwY+i+f1OwJ2gWM8r9cBfpt/n+8C3psX7Hu44r14wb4beB/+fb4beC+e1/cI+G3gtXhenwN8Nv923wW8Ny/Y9wDvzRXfDbwXL9h3A+/Dv91fAS/N8/ocAZ8NfBbP63eA1+bf5ruA9+YF+x7gvXlO3w28Fy/YdwPvw7/eceAiz9/bCHht4Ld4/k4Au/zrfBfw3rxg3wO8N8/fdwPvxQv23cD78K/z3sB38fydEFeY5+99gO/mRffVwEfxgn0P8N68cN8NvBcv2NcAH82L7qeBt+J5/Q3w0uKKnwbeiuf1M8Bb86IzL9j3AO/Ni+a7gffiBRMvmuPARZ6/rwE+Wlzx3sB38fw9BLiVF415/r4HeG/+db4beC+eP/Gi+Wzgs3j+Xgb4a3HFceAiz9/3AO/Ni+angbfiOX0P8N7823w38F48p58B3pp/2XHg6cBxntczgAcDiGf7buC9eP4eAtzKv+w48N3AWwGXgO8GPpp/n68GPoorvgf4aGCXf9lnA5/F8/cxwFcDiGd7MPB0nr/fBl6H/z0eDPwVcJzndQl4MLALIJ7TdwPvxfP3PsB387/DbwGvzfP3OcBncwXiOT0YeDrP3y7wOsBf8z/bRwNfxfN3CXgwsMsViOf11cBH8fz9NfA6wC7/M7028Fu8YO8DfDfPhnhex4G/Bh7E8/fXwOsAu/zP8tLAbwHHef5+B3htnhPi+Xtt4Ld4wX4aeB9gl/8ZXhr4LeA4z98l4KWBW3lOiBfsq4GP4gX7a+B1gF3+e7028FPAcV6wtwF+mueFeOF+G3gtXrBbgbcB/pr/Hh8FfDUv3NcAH83zh3jhjgO/DbwUL9xnA5/Df50HA98FvDYv3PcA780LhviXHQduBY7xwt0KvA/w2/znOQ58FPDRwHFeuO8B3psXDvGiOQ78NvBS/Mt+G/hu4Hv4j3Mc+Cjgo4Hj/Mu+B3hv/mWIF91x4KeB1+JFcyvw08BPA7/Dv95x4K2Atwbemhfd1wAfzYsG8a/31cBH8a/328BvA7vAX/P8vTZwHHht4KX517kEfDTw3bzoEP82rw18N/Ag/mf4G+C9gb/mXwfxb3cc+Gzgo/jvcwn4bOCr+bdB/Ps9GPhs4L34r/U5wFcDu/zbIf7jPBj4bOCtgWP853gG8N3AdwO38u+H+I93HHhr4K2Bt+Lf7xLw08BPAz/NfyzEf77XBl4beG3gpYFjvGCXgL8GbgX+Gvht4K/5z8M/Aio7ZpsvbphcAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoXCircle;
impl IconShape for GoXCircle {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M3.404 12.596a6.5 6.5 0 119.192-9.192 6.5 6.5 0 01-9.192 9.192zM2.344 2.343a8 8 0 1011.313 11.314A8 8 0 002.343 2.343zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoXCircleFill;
impl IconShape for GoXCircleFill {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M2.343 13.657A8 8 0 1113.657 2.343 8 8 0 012.343 13.657zM6.03 4.97a.75.75 0 00-1.06 1.06L6.94 8 4.97 9.97a.75.75 0 101.06 1.06L8 9.06l1.97 1.97a.75.75 0 101.06-1.06L9.06 8l1.97-1.97a.75.75 0 10-1.06-1.06L8 6.94 6.03 4.97z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoZap;
impl IconShape for GoZap {
    fn view_box(&self) -> &str {
        "0 0 16 16"
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
                d: "M10.561 1.5a.016.016 0 00-.01.004L3.286 8.571A.25.25 0 003.462 9H6.75a.75.75 0 01.694 1.034l-1.713 4.188 6.982-6.793A.25.25 0 0012.538 7H9.25a.75.75 0 01-.683-1.06l2.008-4.418.003-.006a.02.02 0 00-.004-.009.02.02 0 00-.006-.006L10.56 1.5zM9.504.43a1.516 1.516 0 012.437 1.713L10.415 5.5h2.123c1.57 0 2.346 1.909 1.22 3.004l-7.34 7.142a1.25 1.25 0 01-.871.354h-.302a1.25 1.25 0 01-1.157-1.723L5.633 10.5H3.462c-1.57 0-2.346-1.909-1.22-3.004L9.503.429z",
                fill_rule: "evenodd",
            }
        }
    }
}
