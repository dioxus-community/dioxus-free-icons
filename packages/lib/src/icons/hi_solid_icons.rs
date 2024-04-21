use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAJVElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+m7zy67/9a5P5WgBE/M4f//qP/zb/9RD/xV759d7uvZz50cBL85z+WhFf/ce/8RPfw38dxH+BV37jd34ww/Betj8aOM4Ltyvpq+n77/njX/7hW/nPhfhP9Mqv//avTeZ72X5v/g0kfTcR3/PHv/7jv81/DsR/gld+vbd7L2d+NPDS/Mf4a0V89R//xk98D/+xEP9BXvmN3/nBDMN72f5o4Dj/OXYlfTV9/z1//Ms/fCv/foh/p1d+/bd/bTLfy/Z7819I0ncT8T1//Os//tv82yH+jV759d7uvZz50cBL89/rrxXx1X/8Gz/xPfzrIf4VXvmN3/nBDMN72f5o4Dj/s+xK+mr6/nv++Jd/+FZeNIgXwSu//tu/NpnvZfu9+V9A0ncT8T1//Os//tu8cIgX4lXf4B1eOqfpqwyvzf9Cgt+OWj/mD3/tx/6a5w/xArzyG7/zg71e/xVwnP/ddjWbvcwf//IP38rzQrwAr/y6b/vZtj+L/wMkfc4f/+ZPfjbPC/ECvPLrvu1n2/4s/nNcQny3ovw0AK29t+G9+E8i6XP++Dd/8rN5XogX4JVf/+1f2639Fv+BBM9A+uz5zs5P//ZPf/cuD/DKr/u2n237s/hPoFJe549//cd/m+eFeCFe6XXf5qsxH8W/38+olK/+41//8d/mhXil13kb8x9NfM2f/OZPfTTPH+Jf8Kpv8A4vndP00Yb34l/nEuK71c+++o9/+Ydv5UXwSq/zNn8FvDT/AQTfE7V+9R/+2o/9NS8Y4kX02m/93sdXe3sfjf3ehgfxAgiegfTZ852dn/7tn/7uXV5Er/3W7318eenSRf4dBM9A+u75zs5X//ZPf/cu/zLEv8Erv+7bvjf2extei2f7GZXy1X/86z/+2/wbvNLrvu1PYb81/waC30H67j/+zZ/8bv51EP8Or/oG7/DSOU0fHbV+9R/+2o/9Nf8Gr/x6b/dezvxo4KX5VxJ8T9T61X/4az/21/zbIP4bvPZbv/fx1d7eR9l+b+DB/CsInoH03fOdna/+7Z/+7l3+fRD/hV71Dd7hpbO1j7L93vwrCX4H6bv/+Dd/8rv5j4P4L/DKr/d270Xmextem38lwfdErV/9h7/2Y3/NfzzEf7LXfuv3Pr7a2/to7Pc2PIgXgeAZSN8939n56t/+6e/e5T8P4kX0Kq/7th+d8CD1/df88S//8K38G7zy677te2O/t+G1eD4Ev4P03X/8mz/53fwbvPIbv/ODPQwfFfCMP/rNn/xq/mWIF9Ervc7b/DTwVgBIP62Ir/njX//x3+bf4FXf4B1eOqfpow1vDSD46aj1q//w137sr/k3eOXXf/vXduZHYb81V/zMn/zWT701/zLEi+iVXudtng48mOd065/81k89hP8Gr/3W7318tb//Vs78bODBPKe//pPf+qmX4V+GeBG90uu8jXk+/uS3fkr8F3rlN37nB3sYPgr7vYHjvAB/8ls/Jf5liBfBK7/+27+2W/stno8/+a2fEv8FXvn13/61nflR2G/Ni0ClvM4f//qP/zYvHOJF8Mqv//av7dZ+i+fjT37rp8Tz8cqv+7bvjeT59vbP/PZPf/cu/wav/dbvfXy1v/9Wzvxs4MH8K6iU1/njX//x3+aFQ7wIXvn13/613dpv8Xz8yW/9lHg+Xvl13/azbX8WsIv03er7r/njX/7hW3kRvPIbv/ODPQwfhf3ewHH+DVTK6/zxr//4b/PCIV4Er/z6b//abu23eD7+5Ld+Sjwfr/y6b/vZtj+LB5J+mvB3/8mv/9TP8Hy80uu/zVuRem/st+bfSaW8zh//+o//Ni8c4kXwyq/7tp9t+7N4Pv7kt35KPB+v/Lpv+9m2P4vnb1fw15Z2AWQfN7w0cJz/IJI+549/8yc/mxcO8SJ45dd92/e2/V08H3/yWz8lno9Xft23/Wzbn8V/k4h4mz/6jZ/4aV44xIvglV//7V/brf0Wz8ef/NZPiefjlV/3bT/b9mfx30SlvM4f//qP/zYvHOJF8Mqv//av7dZ+i+fjT37rp8Tz8cqv+7afbfuz+G+iUl7nj3/9x3+bFw7xInjlN37nB3u9fjrPx5/81k+J5+OVX/dtP9v2Z/HfZHHs2Inf/unv3uWFQ7yIXul13uangbfiufzJb/2UeD5e+XXf9rNtfxb/PX7mT37rp96afxniRfTab/3ex5d7lz4b897AMZ7pT37rp8Tz8cqv+7afbfuz+K91CfHdi51jn/3bP/3du/zLEP9Gr/z6b//aynxpSxfn29s/89s//d27PMArv+7bfrbtz+I/ieAZwG9L+mtH/PV8a+uvf/unv3uXfx3Ev9Mrvc7bGEDw24746cX29vf89k9/9+4rv+7bfrbtz+I/1iXEd5dSv/sPf+3H/pp/P8S/0yu9ztuY57Qr6aeFL6X5KP4DCJ6B9Nl//Js/+d38x0L8O73S67yNeT4kfQ5wK/ZnGx7Ev80lSR/9x7/5k9/Nfw7Ev9Mrvc7bmOdD0uf88W/+5Ge/9lu/9/Hl3qXPxnwU/xriaxY7xz77t3/6u3f5z4P4d3ql13kb83xI+pw//s2f/Gye6VVe7+3eOjO/GzjGC3cpIt77j37jJ36a/3yIf6dXep23Mc+HpM/549/8yc/mAV75jd/5wV6vfxp4KZ6/v9Fs9tZ//Ms/fCv/NRD/Tq/0Om9jng9Jn/PHv/mTn81zee23fu/jy0uXfht4KZ7T3yyOHXvt3/7p797lvw7i3+mVXudtzPMh6XP++Dd/8rN5AV75dd/2s22/NYCkn/7j3/zJz+a/HuLf6ZVe523M8yHpff74N3/yu/mfDfHv9Eqv8zY/DbwVz6XU+jJ/+Gs/9tf8z4b4d3rVN3iHl27T9NvAMe4nvuZPfvOnPpr/+RD/AV75jd/5wQzDewMQ8dt//Os//tv874D4/w3x/xvi/zfE/2+I/98Q/7/xj+aE4F8zYh4aAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAcademicCap;
impl IconShape for HiAcademicCap {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.3939 2.08085C10.1424 1.97305 9.85763 1.97305 9.60608 2.08085L2.60608 5.08085C2.2384 5.23843 2 5.59997 2 6C2 6.40003 2.2384 6.76157 2.60608 6.91915L5.25037 8.05241C5.34653 7.94347 5.46706 7.85473 5.60608 7.79515L9.60608 6.08087C10.1137 5.86331 10.7016 6.09846 10.9191 6.60609C11.1367 7.11372 10.9015 7.7016 10.3939 7.91916L7.66668 9.08797L9.60608 9.91915C9.85763 10.027 10.1424 10.027 10.3939 9.91915L17.3939 6.91915C17.7616 6.76157 18 6.40003 18 6C18 5.59997 17.7616 5.23843 17.3939 5.08085L10.3939 2.08085Z",
            }
            path {
                d: "M3.31004 9.39673L5 10.121V14.2226C4.65723 14.1449 4.30705 14.0867 3.95071 14.0494C3.48094 14.0001 3.1097 13.6289 3.06044 13.1591C3.02046 12.7778 3 12.391 3 11.9998C3 11.1033 3.10741 10.2315 3.31004 9.39673Z",
            }
            path {
                d: "M9.29996 16.5725C8.62708 15.9129 7.85167 15.3584 7 14.9351V10.9781L8.81824 11.7574C9.57289 12.0808 10.4271 12.0808 11.1818 11.7574L16.69 9.39673C16.8926 10.2315 17 11.1033 17 11.9998C17 12.391 16.9795 12.7778 16.9396 13.1591C16.8903 13.6289 16.5191 14.0001 16.0493 14.0494C13.9765 14.2667 12.1124 15.188 10.7 16.5725C10.3112 16.9537 9.68881 16.9537 9.29996 16.5725Z",
            }
            path {
                d: "M6 18C6.55228 18 7 17.5523 7 17V14.9351C6.37136 14.6227 5.70117 14.3817 5 14.2226V17C5 17.5523 5.44772 18 6 18Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAdjustments;
impl IconShape for HiAdjustments {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 4C5 3.44772 4.55228 3 4 3C3.44772 3 3 3.44772 3 4V11.2676C2.4022 11.6134 2 12.2597 2 13C2 13.7403 2.4022 14.3866 3 14.7324V16C3 16.5523 3.44772 17 4 17C4.55228 17 5 16.5523 5 16V14.7324C5.5978 14.3866 6 13.7403 6 13C6 12.2597 5.5978 11.6134 5 11.2676V4Z",
            }
            path {
                d: "M11 4C11 3.44772 10.5523 3 10 3C9.44772 3 9 3.44772 9 4V5.26756C8.4022 5.61337 8 6.25972 8 7C8 7.74028 8.4022 8.38663 9 8.73244V16C9 16.5523 9.44772 17 10 17C10.5523 17 11 16.5523 11 16V8.73244C11.5978 8.38663 12 7.74028 12 7C12 6.25972 11.5978 5.61337 11 5.26756V4Z",
            }
            path {
                d: "M16 3C16.5523 3 17 3.44772 17 4V11.2676C17.5978 11.6134 18 12.2597 18 13C18 13.7403 17.5978 14.3866 17 14.7324V16C17 16.5523 16.5523 17 16 17C15.4477 17 15 16.5523 15 16V14.7324C14.4022 14.3866 14 13.7403 14 13C14 12.2597 14.4022 11.6134 15 11.2676V4C15 3.44772 15.4477 3 16 3Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGoklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xr/AW7/Ahr5XiwcYP5n8goVvD3PpzP/ZNv8OLBvEieNN3/ODPkvXRxsf5X0Bo1/JX/+KPfvPn8MIh/gVv+g4f/F3Ae/O/03f/4o998/vwgiFeiDd9xw/+bMxn8b+Z+Jxf/NFv/myeP8QL8Nbv/MEPHhpP5/+AvvCQn/7hb76V54V4Ad70HT74vYHv4v+G9/nFH/vm7+Z5IV6AN33HD/5szGfxf4H4nF/80W/+bJ4X4gV403f84M/GfBb/F4jP+cUf/ebP5nkhXoA3fccP/mzMZ/F/gficX/zRb/5snhfiBXjTd/zgz8Z8Fv8XiM/5xR/95s/meSFegDd9xw/+bMxn8aL5nSJ9tcO7/BdQ6nizPxp4LV4U4nN+8Ue/+bN5XogX4E3f8YM/G/NZ/Mt+5hd/7Jvfmv8Gb/oOH/zTwFvxLxGf84s/+s2fzfNCvABv+o4f/NmYz+JfFC/ziz/2jX/Nf4M3fYcPfWnIv+JfIj7nF3/0mz+b54V4Ad70HT/4szGfxb/gF3/sm8V/ozd9hw82/xLxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+BUV6m5/70W/6af4bvMU7fshbN/un+JeIz/nFH/3mz+Z5IV6AN33HD/5szGfxL9Jf95uz1/np7/7qXf4LvfV7f/Tx4XD9W+CX5l8iPucXf/SbP5vnhXgB3vQdP/izMZ/Fi0LcCnw3/4VkfbTxcV4U4nN+8Ue/+bN5XogX4E3f8YM/G/NZ/F8gPucXf/SbP5vnhXgB3vQdP/izMZ/F/wXic37xR7/5s3leiBfgTd/xgz8b81n8XyA+5xd/9Js/m+eFeAHe9B0/+LMxn8X/BeJzfvFHv/mzeV6IF+BN3/GDPxvzWfxfID7nF3/0mz+b54V4Ad70HT/4szGfxf8F4nN+8Ue/+bN5XogX4E3f8YM/G/NZvGi+pi989U//8Dffyovgrd/5gx88ND4a+Cj+K4jP+cUf/ebP5nkhXoA3fccP/mzMZ/Ev+5pf/LFv/mj+Dd70HT74q4GP4j+b+Jxf/NFv/myeF+IFeNN3/ODPxnwW/4K+8JCf/uFvvpV/g7d+5w9+8NB4Ov/ZxOf84o9+82fzvBAvwJu+4wd/Nuaz+Bf0m/MTP/3dX73Lv8Fbv/dHHx8OVxf5zyY+5xd/9Js/m+eFeAHe9B0/+LMxn8W/oEhv83M/+k0/zb/Bm77DB7838F38ZxOf84s/+s2fzfNCvABv+o4f/NmYz+JfIm7F8Ta/+GPf+Nf8K7zpO3zoSwv/lvFx/rOJz/nFH/3mz+Z5IV6AN33HD/5szGfxIhL8tsVv8yKQeW3Da/NfRXzOL/7oN382zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK8xTt+yFs3+6f4P6BIb/NzP/pNP83zQrwQb/oOH3wr8CD+d3vGL/7YNz+Y5w/xQrz5O3/wa2fjt/hfLAqv8/M//M2/zfOH+Be8xTt+yFs3+7uBY/zvcqlI7/1zP/pNP80LhngRvPU7f/CDh8ZHA28NPIj/2Z4B/HS/Of/sn/7ur97lhUP8J3nTd/jQl4b8beAY/z6XIF77F3/sG/+a/3iI/0Rv+g4f+tKQvw0c49/mEsRr/+KPfeNf858D8Z/sTd/hQ18a8reBY/zrXIJ47V/8sW/8a/7zIP4LvOk7fOhLQ/42cIwXzSWI1/7FH/vGv+Y/F+K/yJu+w4e+NORvA8d44S5BvPYv/tg3/jX/+RD/hd70HT70pSF/GzjG83cJ4rV/8ce+8a/5r4H4L/am7/ChLw3528AxntMliNf+xR/7xr/mvw7iv8GbvsOHvjTkbwPHuOISxGv/4o9941/zXwvx3+RN3+FDXxrytwEgXvsXf+wb/5r/eoj/Rm/6Dh/60gC/+GPf+Nf890D8/4b4/w3x/xvi/zfE/2+I/9/4R0X1m1AaXoqLAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAnnotation;
impl IconShape for HiAnnotation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 13V5C18 3.89543 17.1046 3 16 3H4C2.89543 3 2 3.89543 2 5V13C2 14.1046 2.89543 15 4 15H7L10 18L13 15H16C17.1046 15 18 14.1046 18 13ZM5 7C5 6.44772 5.44772 6 6 6H14C14.5523 6 15 6.44772 15 7C15 7.55228 14.5523 8 14 8H6C5.44772 8 5 7.55228 5 7ZM6 10C5.44772 10 5 10.4477 5 11C5 11.5523 5.44772 12 6 12H9C9.55229 12 10 11.5523 10 11C10 10.4477 9.55229 10 9 10H6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArchive;
impl IconShape for HiArchive {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 3C2.89543 3 2 3.89543 2 5C2 6.10457 2.89543 7 4 7H16C17.1046 7 18 6.10457 18 5C18 3.89543 17.1046 3 16 3H4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 8H17V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V8ZM8 11C8 10.4477 8.44772 10 9 10H11C11.5523 10 12 10.4477 12 11C12 11.5523 11.5523 12 11 12H9C8.44772 12 8 11.5523 8 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleDown;
impl IconShape for HiArrowCircleDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 7C11 6.44772 10.5523 6 10 6C9.44771 6 9 6.44772 9 7L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071L13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289C13.3166 8.90237 12.6834 8.90237 12.2929 9.29289L11 10.5858V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleLeft;
impl IconShape for HiArrowCircleLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM10.7071 7.70711C11.0976 7.31658 11.0976 6.68342 10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071C11.0976 13.3166 11.0976 12.6834 10.7071 12.2929L9.41421 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9L9.41421 9L10.7071 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleRight;
impl IconShape for HiArrowCircleRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289C8.90237 6.68342 8.90237 7.31658 9.29289 7.70711L10.5858 9L7 9C6.44772 9 6 9.44771 6 10C6 10.5523 6.44772 11 7 11H10.5858L9.29289 12.2929C8.90237 12.6834 8.90237 13.3166 9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071L13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleUp;
impl IconShape for HiArrowCircleUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071C6.68342 11.0976 7.31658 11.0976 7.70711 10.7071L9 9.41421L9 13C9 13.5523 9.44771 14 10 14C10.5523 14 11 13.5523 11 13V9.41421L12.2929 10.7071C12.6834 11.0976 13.3166 11.0976 13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowDown;
impl IconShape for HiArrowDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M16.7071 10.2929C17.0976 10.6834 17.0976 11.3166 16.7071 11.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L3.29289 11.7071C2.90237 11.3166 2.90237 10.6834 3.29289 10.2929C3.68342 9.90237 4.31658 9.90237 4.70711 10.2929L9 14.5858L9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3L11 14.5858L15.2929 10.2929C15.6834 9.90237 16.3166 9.90237 16.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFNElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xX+it3/mDHzy4vFTJ3P25H/um3+G/H+K/yJu+wwe/t9BXGR/nMv11vzl7nZ/+7q/e5b8P4r/Am77DB7838F08F4nv+YUf/eb35r8P4j/Zm77DB7838F08H0K7v/Bj33SC/z6I/0Rv+g4f/N7Ad/GCPeMXf+ybH8x/H8R/kjd9hw9+b+C7eCEU+phf+JFv+mr++yD+E7zpO3zwewPfxQsh8T2/8KPf/N7890L8B3vTd/jg9wa+ixdC4nt+4Ue/+b3574f4D/Sm7/DB7w18Fy+ExPf8wo9+83vzPwPiP8ibvsMHvzfwXbwQEt/zCz/6ze/N/xyI/wBv+g4f/N7Ad/FCSHzPL/zoN783/7Mg/p3e9B0++L2B7+KFkPieX/jRb35v/udB/Du86Tt88HsD38ULIfE9v/Cj3/ze/M+E+Dd603f44PcGvosXQuJ7fuFHv/m9+Z8L8W/wpu/wwe8NfBcvhMT3/MKPfvN78z8b4l/pTd/hg98b+C5eCInv+YUf/eb35n8+xL/Cm77DB7838F28EBLf8ws/+s3vzf8OiBfRm77DB7838F28EBLf8ws/+s3vzf8eiBfBm77DB7838F28EBLf8ws/+s3vzf8uiH/Bm77DB7838F28UPrrAh/N/yANXfrFH/vGv+aFQ7wQb/oOH/zewHfxv5TQboj3+bkf/aaf5vlDvABv+g4f+tKQf8X/AX3hIT/9w998K88L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArzpO3zoS0P+Ff8H9IWH/PQPf/OtPC/EC/Gm7/DB7w18F/97XSrSe//cj37TT/P8If4Fb/oOH/zewHfxQumvo/hj+B8kW+z+4o9941/zwiFeBG/6Dh/83sB38cJ99y/+2De/D/+7IF5Eb/oOH/zewHfxwn33L/7YN78P/3sg/hXe9B0++L2B7+KF++5f/LFvfh/+d0D8K73pO3zwewPfxQv33b/4Y9/8PvzPh/g3eNN3+OD3Br6LF+67f/HHvvl9+J8N8W/0pu/wwe8NfBcv3Hf/4o998/vwPxfi3+FN3+GD3xv4Ll647/7FH/vm9+F/JsS/05u+wwe/N/BdvHDf/Ys/9s3vw/88iP8Ab/oOH/zewHfxwn33L/7YN78P/7Mg/oO86Tt88HsD38UL992/+GPf/D78z4H4D/Sm7/DB7w18Fy/cd//ij33z+/A/A+I/2Ju+wwe/N/BdvHDf/Ys/9s3vw38/xH+CN32HD35v4Lt44b77F3/sm9+H/16I/yRv+g4f/N7Ad/FCKPQxv/Aj3/TV/PdB/Cd603f44PcGvosXRNz6iz/6zQ/hvw/iP9mbvsMHvzfwXTx/l37xx775OP99EP8F3vQdPvi9ge/iuUh8zy/86De/N/99EP9F3vQdPvi9ga8GjgFI/E23MX/tn/7ur97lvw/iv9Bbv/MHP7ilXtrh3Z//4W/+bf77If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAV5xAFDeCDNEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowLeft;
impl IconShape for HiArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.70711 16.7071C9.31658 17.0976 8.68342 17.0976 8.29289 16.7071L2.29289 10.7071C1.90237 10.3166 1.90237 9.68342 2.29289 9.29289L8.29289 3.29289C8.68342 2.90237 9.31658 2.90237 9.70711 3.29289C10.0976 3.68342 10.0976 4.31658 9.70711 4.70711L5.41421 9H17C17.5523 9 18 9.44772 18 10C18 10.5523 17.5523 11 17 11L5.41421 11L9.70711 15.2929C10.0976 15.6834 10.0976 16.3166 9.70711 16.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowDown;
impl IconShape for HiArrowNarrowDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 12.2929C15.0976 12.6834 15.0976 13.3166 14.7071 13.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L5.29289 13.7071C4.90237 13.3166 4.90237 12.6834 5.29289 12.2929C5.68342 11.9024 6.31658 11.9024 6.70711 12.2929L9 14.5858L9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3L11 14.5858L13.2929 12.2929C13.6834 11.9024 14.3166 11.9024 14.7071 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+A/2Fu/4IW/dzGeBXxrpp7E+5xd/7Bv/mv+ZEP+B3vQdPvi9ge/iAYR2u83ZQ376u796l/95EP9B3vQdPvi9ge/i+RGf84s/+s2fzf88iP8Ab/oOH/zewHfxgojP+cUf/ebP5n8exL/Tm77DB7838F28EFF4nZ//4W/+bf7nQfw7vOk7fPB7A9/FCyHxPb/wo9/83vzPhPg3etN3+OD3Br6LF0Lie37hR7/5vfmfC/Fv8Kbv8MHvDXwXL4TE9/zCj37ze/M/G+Jf6U3f4YPfG/guXgiJ7/mFH/3m9+Z/PsS/wpu+wwe/N/BdvBAS3/MLP/rN783/DogX0Zu+wwe/N/BdvBAS3/MLP/rN783/HogXwZu+wwe/N/BdvBAS3/MLP/rN783/Loh/wZu+wwe/N/BdvFD66wIfzf8gDV36xR/7xr/mhUO8EG/6Dh/83sB38b+U0G6I9/m5H/2mn+b5Q7wAb/oOH/rSkH/F/wF94SE//cPffCvPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt86EtD/hX/B/SFh/z0D3/zrTwvxAvxpu/wwe8NfBf/e10q0nv/3I9+00/z/CH+BW/6Dh/83sB38ULpr6P4Y/gfJFvs/uKPfeNf88IhXgRv+g4f/N7Ad/HCffcv/tg3vw//uyBeRG/6Dh/83sB38cJ99y/+2De/D/97IP4V3vQdPvi9ge/ihfvuX/yxb34f/ndA/Cu96Tt88HsD38UL992/+GPf/D78z4f4N3jTd/jg9wa+ixfuu3/xx775ffifDfFv9Kbv8MHvDXwXL9x3/+KPffP78D8X4t/hTd/hg98b+C5euO/+xR/75vfhfybEv9ObvsMHvzfwXbwQUXidn//hb/5t/udB/Ad403f44PcGvosXRHzOL/7oN382//Mg/oO86Tt88HsD38XzIz7nF3/0mz+b/3kQ/4He9B0++L2B7+I5Xeo35w/+6e/+6l3+50H8B3uLd/yQt0782TYvBfwMxGf/4o9941/zPxPi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8I6TO6Qacc21UAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowLeft;
impl IconShape for HiArrowNarrowLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.70711 14.7071C7.31658 15.0976 6.68342 15.0976 6.29289 14.7071L2.29289 10.7071C1.90237 10.3166 1.90237 9.68342 2.29289 9.29289L6.29289 5.29289C6.68342 4.90237 7.31658 4.90237 7.70711 5.29289C8.09763 5.68342 8.09763 6.31658 7.70711 6.70711L5.41421 9L17 9C17.5523 9 18 9.44771 18 10C18 10.5523 17.5523 11 17 11L5.41421 11L7.70711 13.2929C8.09763 13.6834 8.09763 14.3166 7.70711 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEDklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+B/sTd/hQ18a+bOw3xr010V8zs/96Df9NP9xEP9DvfU7f/CDx6a/Mj7Oc3qfX/yxb/5u/mMg/od603f84M/GfBbP3/v84o9983fz74f4H+pN3/GDPxvzWbxg7/OLP/bN382/D+J/qLd4xw9562b/FC/c+/zij33zd/Nvh/gf7M3e8YO/2+a9eOHe5xd/7Ju/m38bxP9wb/aOH/zdNu/FC/c+v/hj3/zd/Osh/hd4s3f84O+2eS9euPf5xR/75u/mXwfxv8SbveMHf7fNe/HCvc8v/tg3fzcvOsT/Im/2jh/83TbvxQv3Pr/4Y9/83bxoEP/LvNk7fvB327wXL9z7/OKPffN38y9D/C/0Zu/4wd9t8168cO/ziz/2zd/NC4d4EbzpO3zoSxd8jP9BGnw1+KV54d7nF3/sm7+bFwzxQrzpO3zwewt9lfFx/vd6n1/8sW/+bp4/xAvwpu/woS8N+Vf8nxAv84s/9o1/zfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LP4vEJ/ziz/6zZ/N80K8AG/6Dh/60pB/xf8J8TK/+GPf+Nc8L8QL8Rbv+CFv3ezvBo7xv9f7/OKPffN38/whXgRv+g4f+tJR8jj/g2TTV4FfmhfufX7xx775u3nBEP8Lvek7fPB3Ae/NC/c+v/hj3/zdvHCI/2Xe9B0++LuA9+aFe59f/LFv/m7+ZYj/Rd70HT74u4D35oV7n1/8sW/+bl40iP8l3vQdPvi7gPfmhXufX/yxb/5uXnSI/wXe9B0++LuA9+aFe59f/LFv/m7+dRD/w73pO3zwdwHvzQv3Pr/4Y9/83fzrIf4He9N3+ODvAt6bF+59fvHHvvm7+bdB/A/15u/8wa+djd/ihXufX/yxb/5u/u0Q/0O96Tt+8GdjPosX7H1+8ce++bv590H8D/Wm7/jBn435LJ6/9/nFH/vm7+bfD/E/1Fu/90cfHw5XtwLHeE7v84s/9s3fzX8MxP9gb/oOH/rSkJ8NvJXE3wT67J/70W/6af7jIP5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/7/xj19V1EH2y2PXAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowRight;
impl IconShape for HiArrowNarrowRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.2929 5.29289C12.6834 4.90237 13.3166 4.90237 13.7071 5.29289L17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L13.7071 14.7071C13.3166 15.0976 12.6834 15.0976 12.2929 14.7071C11.9024 14.3166 11.9024 13.6834 12.2929 13.2929L14.5858 11H3C2.44772 11 2 10.5523 2 10C2 9.44772 2.44772 9 3 9H14.5858L12.2929 6.70711C11.9024 6.31658 11.9024 5.68342 12.2929 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowUp;
impl IconShape for HiArrowNarrowUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 7.70711C4.90237 7.31658 4.90237 6.68342 5.29289 6.29289L9.29289 2.29289C9.68342 1.90237 10.3166 1.90237 10.7071 2.29289L14.7071 6.29289C15.0976 6.68342 15.0976 7.31658 14.7071 7.70711C14.3166 8.09763 13.6834 8.09763 13.2929 7.70711L11 5.41421L11 17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17L9 5.41421L6.70711 7.70711C6.31658 8.09763 5.68342 8.09763 5.29289 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xP8RbvMOHvFaLON6r/c1P//A338p/DcR/s7d+5w9+8ND0U+CXBhDaNf6YX/yxb/5u/vMh/pu96Tt88E8Db8Xzep9f/LFv/m7+cyH+m73ZO3zIRePjPH/v84s/9s3fzX8exH+zN32HD74VeBAv2Pv84o9983fznwPx3+zN3ulDPtrpr+KFe59f/LFv/m7+4yH+B3izd/zg77Z5L1649/nFH/vm7+Y/FuJ/iDd7xw/+bpv34oV7n1/8sW/+bv7jIP4HebN3/ODvtnkvXrj3+cUf++bv5j8G4n+YN3vHD/5um/fihXufX/yxb/5u/v0Q/wO92Tt+8HfbvBcv3Pv84o9983fz74P4H+rN3vGDv9vmvXjh3ucXf+ybv5t/O8T/YG/2jh/83TbvxQv3Pr/4Y9/83fzbIP6He7N3/ODvtnkvXrj3+cUf++bv5l8P8b/Am73jB3+3zXvxwr3PL/7YN383/zqI/yXe7B0/+Ltt3osX7n1+8ce++bt50SH+F3mzd/zg77Z5L1649/nFH/vm7+ZFg/hf5s3e8YO/2+a9eOHe5xd/7Ju/m38Z4n+hN3vHD/5um/fihXufX/yxb/5uXjjEi+BN3+FDX7rgY/wP0uCrwS/NC/c+v/hj3/zdvGCIF+JN3+GD31voq4yP87/X+/zij33zd/P8IV6AN32HD31pyL/i/4R4mV/8sW/8a54X4gV403f84M/GfBb/F4jP+cUf/ebP5nkhXoA3fccP/mzMZ/F/gficX/zRb/5snhfiBXjTd/jQl4b8K/5PiJf5xR/7xr/meSFeiLd4xw9562Z/N3CM/73e5xd/7Ju/m+cP8SJ403f40JeOksf5HySbvgr80rxw7/OLP/bN380Lhvhf6E3f4YO/C3hvXrj3+cUf++bv5oVD/C/zpu/wwd8FvDcv3Pv84o9983fzL0P8L/Km7/DB3wW8Ny/c+/zij33zd/OiQfwv8abv8MHfBbw3L9z7/OKPffN386JD/C/wpu/wwd8FvDcv3Pv84o9983fzr4P4H+5N3+GDvwt4b1649/nFH/vm7+ZfD/E/2Ju+wwd/F/DevHDv84s/9s3fzb8N4n+oN32HD/4u4L154d7nF3/sm7+bfzvE/0Bv+g4f/F3Ae/PCvc8v/tg3fzf/Poj/Yd70HT74u4D35oV7n1/8sW/+bv79EP+DvOk7fPB3Ae/NC/c+v/hj3/zd/MdA/A/xpu/wwd8FvDcv3Pv84o9983fzHwfxP8CbvsMHfxfw3rxw7/OLP/bN381/LMR/szd7pw/5aKe/ihfufX7xx775u/mPh/hv9qbv+MFPxzyYF+x9fvHHvvm7+c+B+G/2pu/wweYFe59f/LFv/m7+8yD+m73ZO37wd9u8F8/rfX7xx775u/nPhfhv9tbv/dHHx6PVb9u8FFdcAj76F3/sm7+b/3yI/yHe/J0/+LWVOl7Cf/3TP/zNt/JfA/H/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CFE7K1CbARk0AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowRight;
impl IconShape for HiArrowRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 3.29289C10.6834 2.90237 11.3166 2.90237 11.7071 3.29289L17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L11.7071 16.7071C11.3166 17.0976 10.6834 17.0976 10.2929 16.7071C9.90237 16.3166 9.90237 15.6834 10.2929 15.2929L14.5858 11L3 11C2.44772 11 2 10.5523 2 10C2 9.44772 2.44772 9 3 9H14.5858L10.2929 4.70711C9.90237 4.31658 9.90237 3.68342 10.2929 3.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmDown;
impl IconShape for HiArrowSmDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 10.2929C15.0976 10.6834 15.0976 11.3166 14.7071 11.7071L10.7071 15.7071C10.3166 16.0976 9.68342 16.0976 9.29289 15.7071L5.29289 11.7071C4.90237 11.3166 4.90237 10.6834 5.29289 10.2929C5.68342 9.90237 6.31658 9.90237 6.70711 10.2929L9 12.5858V5C9 4.44772 9.44772 4 10 4C10.5523 4 11 4.44772 11 5L11 12.5858L13.2929 10.2929C13.6834 9.90237 14.3166 9.90237 14.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEb0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+C9y3XWPfvAAn2X5rTG7El994e4nfg3/vRD/Bc7c+GIvPeX0W5jjPIAUH3Ph7sd/Nf99EP/Jztz4Yi895fRbmOM8r1sv3vPEh/DfB/Gf6MyNL/bSU06/hTnO83fp4j1PPM5/H8R/kjM3vthLTzn9FuY4L4DQ91y45wnvzX8fxH+CMze+2EtPOf0W5jgv2KVj88WDb731r3f574P4D3bmxhd76Smn38Ic5wW7VEt97bN3/sNf898L8R/ozI0v9tJTTr+FOc4LdqmW+tpn7/yHv+a/H+I/yJkbX+ylp5x+C3OcF+xSLfW1z975D3/N/wyI/wBnbnyxl55y+i3McV6wS7XU1z575z/8Nf9zIP6dztz4Yi895fRbmOO8YJdqqa999s5/+Gv+Z0H8O5y58cVeesrptzDHecEu1VJf++yd//DX/M+D+Dc6c+OLvfSU029hjvNCSHx0WH/Nf5IKz7jnnifcyr8N4t/guuse/eC1/FeY4/yPoN8+Np+/za23/vUu/zqIf4MT1z36t8Cvzf8o+u2L9zzhdfjXQfwrXXfdox+8xk/nf6Bj88WJW2/9611edIh/peuue/SD1/jp/A90bL44ceutf73Liw7xb3Diukf/Nvi1+B9Fv3Pxnie8Nv86iH+D66579IPX+K+BY/yPoN85Np+/9a23/vUu/zqIf6MzN77YS09t+m3gGC+EFB8T9l/zn2RrPv/rW2/9613+bRD/DmdufLGXntr028AxXhCxW6O+ztk7/+Gv+Z8H8e905sYXe+mpTb8NHOMFEbs16uucvfMf/pr/WRD/Ac7c+GIvPbXpt4FjvCBit0Z9nbN3/sNf8z8H4j/ImRtf7KWnNv02cIwXROzWqK9z9s5/+Gv+Z0D8Bzpz44u99NSm3waO8YKI3Rr1dc7e+Q9/zX8/xH+wMze+2EtPbfpt4BgviNitUV/n7J3/8Nf890L8Jzhz44u99NSm3waO8YKI3WOzxUNuvfWvd/nvg/hPcubGF3vpqU2/DRzjBRD6ngv3POG9+e+D+E905sYXe+mpTb8NHOP5EbsX737iCf77IP6TnbnxxV56atNvA8d4HnrGxXue8GD++yD+C5y58cVeemrTbwPHeAApPubC3Y//av77IP6LPPjBL318b7X6auO3Bu1K+uoLdz/+q/nvhfj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P/GPwIijiFQq2ymcAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmLeft;
impl IconShape for HiArrowSmLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.70711 14.7071C9.31658 15.0976 8.68342 15.0976 8.29289 14.7071L4.29289 10.7071C3.90237 10.3166 3.90237 9.68342 4.29289 9.29289L8.29289 5.29289C8.68342 4.90237 9.31658 4.90237 9.70711 5.29289C10.0976 5.68342 10.0976 6.31658 9.70711 6.70711L7.41421 9L15 9C15.5523 9 16 9.44772 16 10C16 10.5523 15.5523 11 15 11H7.41421L9.70711 13.2929C10.0976 13.6834 10.0976 14.3166 9.70711 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEZ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+B/g5PWP+iibj0Ycl/XTO/P5x9x661/v8p8P8d/s5PWP+Wg7v4oHErs16uucvfMf/pr/XIj/Zieue9TTgQfz3MRujfo6Z+/8h7/mPw/iv9mJ6x5lXhCxW6O+ztk7/+Gv+c+B+G928rpHf7fxe/GCiN0a9XXO3vkPf81/PMR/swc/+KWPX1otbwWO8YKI3Rr1dc7e+Q9/zX8sxP8AZ258sZee2vTbwDFeELFbo77O2Tv/4a/5j4P4H+LMjS/20lObfhs4xgsidmvU1zl75z/8Nf8xEP+DnLnxxV56atNvA8d4QcRujfo6Z+/8h7/m3w/xP8yZG1/spac2/TZwjBdE7Naor3P2zn/4a/59EP8DnbnxxV56atNvA8d4QcRujfo6Z+/8h7/m3w7xP9SZG1/spac2/TZwjBdE7Naor3P2zn/4a/5tEP8O11336AdP8CD+k6T80jZfzQsjdmvU1zl75z/8Nf96iH+DBz/4pY9fWq1+Cvza/E8gdmfWy9xzzxNu5V8H8W9w4rpH/xb4tfkfRb998Z4nvA7/Ooh/peuue/SD1/jp/A80Qw+5554n3MqLDvGvdN11j37wGj+d/4GOzRcnbr31r3d50SH+DU5c9+jfBr8W/6Pody7e84TX5l8H8W/w4Ae/9PFLq9VPg1+L/xkuzdBL33PPE27lXwfx7/DgB7/08YPV6qX5T5LSS9v5Vbxwl2qpr332zn/4a/71EP9DnbnxxV56yum3MMd5wS7VUl/77J3/8Nf82yD+Bzpz44u99JTTb2GO84JdqqW+9tk7/+Gv+bdD/A9z5sYXe+kpp9/CHOcFu1RLfe2zd/7DX/Pvg/gf5MyNL/bSU06/hTnOC3aplvraZ+/8h7/m3w/xP8SZG1/spaecfgtznBfsUi31tc/e+Q9/zX8MxP8AZ258sZeecvotzHFesEu11Nc+e+c//DX/cRD/za677tEPXst/hTnOC3aplvraZ+/8h7/mPxbiv9mJ6x/105i34gW7VEt97bN3/sNf8x8P8d/sxPWPuog5zvN3qZb62mfv/Ie/5j8H4r/ZiesefSv4QTyvS7XU1z575z/8Nf95EP/NTl7/mI+286t4Tpdqqa999s5/+Gv+cyH+Bzh5/WM+2vZHg48L/fTOfP7Rt97617v850P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BpwIZUCCiNg8AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmRight;
impl IconShape for HiArrowSmRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 5.29289C10.6834 4.90237 11.3166 4.90237 11.7071 5.29289L15.7071 9.29289C16.0976 9.68342 16.0976 10.3166 15.7071 10.7071L11.7071 14.7071C11.3166 15.0976 10.6834 15.0976 10.2929 14.7071C9.90237 14.3166 9.90237 13.6834 10.2929 13.2929L12.5858 11L5 11C4.44772 11 4 10.5523 4 10C4 9.44772 4.44772 9 5 9H12.5858L10.2929 6.70711C9.90237 6.31658 9.90237 5.68342 10.2929 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmUp;
impl IconShape for HiArrowSmUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 9.70711C4.90237 9.31658 4.90237 8.68342 5.29289 8.29289L9.29289 4.29289C9.68342 3.90237 10.3166 3.90237 10.7071 4.29289L14.7071 8.29289C15.0976 8.68342 15.0976 9.31658 14.7071 9.70711C14.3166 10.0976 13.6834 10.0976 13.2929 9.70711L11 7.41421L11 15C11 15.5523 10.5523 16 10 16C9.44772 16 9 15.5523 9 15L9 7.41421L6.70711 9.70711C6.31658 10.0976 5.68342 10.0976 5.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowUp;
impl IconShape for HiArrowUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.29289 9.70711C2.90237 9.31658 2.90237 8.68342 3.29289 8.29289L9.29289 2.29289C9.68342 1.90237 10.3166 1.90237 10.7071 2.29289L16.7071 8.29289C17.0976 8.68342 17.0976 9.31658 16.7071 9.70711C16.3166 10.0976 15.6834 10.0976 15.2929 9.70711L11 5.41421L11 17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17L9 5.41421L4.70711 9.70711C4.31658 10.0976 3.68342 10.0976 3.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowsExpand;
impl IconShape for HiArrowsExpand {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H8C8.55228 3 9 3.44772 9 4C9 4.55228 8.55228 5 8 5H6.41421L8.70711 7.29289C9.09763 7.68342 9.09763 8.31658 8.70711 8.70711C8.31658 9.09763 7.68342 9.09763 7.29289 8.70711L5 6.41421V8C5 8.55228 4.55228 9 4 9C3.44772 9 3 8.55228 3 8V4ZM12 5C11.4477 5 11 4.55228 11 4C11 3.44772 11.4477 3 12 3H16C16.5523 3 17 3.44772 17 4V8C17 8.55228 16.5523 9 16 9C15.4477 9 15 8.55228 15 8V6.41421L12.7071 8.70711C12.3166 9.09763 11.6834 9.09763 11.2929 8.70711C10.9024 8.31658 10.9024 7.68342 11.2929 7.29289L13.5858 5H12ZM3 12C3 11.4477 3.44772 11 4 11C4.55228 11 5 11.4477 5 12V13.5858L7.29289 11.2929C7.68342 10.9024 8.31658 10.9024 8.70711 11.2929C9.09763 11.6834 9.09763 12.3166 8.70711 12.7071L6.41421 15H8C8.55228 15 9 15.4477 9 16C9 16.5523 8.55228 17 8 17H4C3.44772 17 3 16.5523 3 16V12ZM16 11C16.5523 11 17 11.4477 17 12V16C17 16.5523 16.5523 17 16 17H12C11.4477 17 11 16.5523 11 16C11 15.4477 11.4477 15 12 15H13.5858L11.2929 12.7071C10.9024 12.3166 10.9024 11.6834 11.2929 11.2929C11.6834 10.9024 12.3166 10.9024 12.7071 11.2929L15 13.5858V12C15 11.4477 15.4477 11 16 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAtSymbol;
impl IconShape for HiAtSymbol {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.2426 5.75736C11.8995 3.41421 8.10051 3.41421 5.75736 5.75736C3.41421 8.10051 3.41421 11.8995 5.75736 14.2426C7.79395 16.2792 10.9325 16.5464 13.257 15.0408C13.7205 14.7405 14.3397 14.8729 14.6399 15.3364C14.9402 15.8 14.8078 16.4191 14.3443 16.7194C11.2445 18.7273 7.0606 18.3743 4.34315 15.6569C1.21895 12.5327 1.21895 7.46734 4.34315 4.34315C7.46734 1.21895 12.5327 1.21895 15.6569 4.34315C17.2187 5.90503 18 7.9542 18 10C18 11.6569 16.6569 13 15 13C14.3247 13 13.7015 12.7769 13.2001 12.4003C12.4703 13.3717 11.3085 14 10 14C7.79086 14 6 12.2091 6 10C6 7.79086 7.79086 6 10 6C12.2091 6 14 7.79086 14 10C14 10.5523 14.4477 11 15 11C15.5523 11 16 10.5523 16 10C16 8.46294 15.4144 6.9291 14.2426 5.75736ZM12 10C12 8.89543 11.1046 8 10 8C8.89543 8 8 8.89543 8 10C8 11.1046 8.89543 12 10 12C11.1046 12 12 11.1046 12 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBackspace;
impl IconShape for HiBackspace {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.70711 4.87868C7.26972 4.31607 8.03278 4 8.82843 4H15C16.6569 4 18 5.34315 18 7V13C18 14.6569 16.6569 16 15 16H8.82843C8.03278 16 7.26972 15.6839 6.70711 15.1213L2.29289 10.7071C2.10536 10.5196 2 10.2652 2 10C2 9.73478 2.10536 9.48043 2.29289 9.29289L6.70711 4.87868ZM10.7071 7.29289C10.3166 6.90237 9.68342 6.90237 9.29289 7.29289C8.90237 7.68342 8.90237 8.31658 9.29289 8.70711L10.5858 10L9.29289 11.2929C8.90237 11.6834 8.90237 12.3166 9.29289 12.7071C9.68342 13.0976 10.3166 13.0976 10.7071 12.7071L12 11.4142L13.2929 12.7071C13.6834 13.0976 14.3166 13.0976 14.7071 12.7071C15.0976 12.3166 15.0976 11.6834 14.7071 11.2929L13.4142 10L14.7071 8.70711C15.0976 8.31658 15.0976 7.68342 14.7071 7.29289C14.3166 6.90237 13.6834 6.90237 13.2929 7.29289L12 8.58579L10.7071 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBadgeCheck;
impl IconShape for HiBadgeCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.26701 3.45496C6.91008 3.40364 7.52057 3.15077 8.01158 2.73234C9.15738 1.75589 10.8426 1.75589 11.9884 2.73234C12.4794 3.15077 13.0899 3.40364 13.733 3.45496C15.2336 3.57471 16.4253 4.76636 16.545 6.26701C16.5964 6.91008 16.8492 7.52057 17.2677 8.01158C18.2441 9.15738 18.2441 10.8426 17.2677 11.9884C16.8492 12.4794 16.5964 13.0899 16.545 13.733C16.4253 15.2336 15.2336 16.4253 13.733 16.545C13.0899 16.5964 12.4794 16.8492 11.9884 17.2677C10.8426 18.2441 9.15738 18.2441 8.01158 17.2677C7.52057 16.8492 6.91008 16.5964 6.26701 16.545C4.76636 16.4253 3.57471 15.2336 3.45496 13.733C3.40364 13.0899 3.15077 12.4794 2.73234 11.9884C1.75589 10.8426 1.75589 9.15738 2.73234 8.01158C3.15077 7.52057 3.40364 6.91008 3.45496 6.26701C3.57471 4.76636 4.76636 3.57471 6.26701 3.45496ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBan;
impl IconShape for HiBan {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.4766 14.8907C12.4958 15.5892 11.2959 16 10 16C6.68629 16 4 13.3137 4 10C4 8.70414 4.41081 7.50423 5.1093 6.52339L13.4766 14.8907ZM14.8908 13.4765L6.52354 5.1092C7.50434 4.41077 8.7042 4 10 4C13.3137 4 16 6.68629 16 10C16 11.2958 15.5892 12.4957 14.8908 13.4765ZM18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBeaker;
impl IconShape for HiBeaker {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.99985 2C6.59539 2 6.23075 2.24364 6.07597 2.61732C5.92119 2.99099 6.00675 3.42111 6.29275 3.70711L6.99985 4.41421V8.17157C6.99985 8.43679 6.8945 8.69114 6.70696 8.87868L2.70696 12.8787C0.817066 14.7686 2.15556 18 4.82828 18H15.1714C17.8441 18 19.1826 14.7686 17.2927 12.8787L13.2927 8.87868C13.1052 8.69114 12.9999 8.43679 12.9999 8.17157V4.41421L13.707 3.70711C13.993 3.42111 14.0785 2.99099 13.9237 2.61732C13.769 2.24364 13.4043 2 12.9999 2H6.99985ZM8.99985 8.17157V4H10.9999V8.17157C10.9999 8.96722 11.3159 9.73028 11.8785 10.2929L12.9061 11.3204C12.1892 11.1537 11.4377 11.1874 10.7349 11.4217L10.2647 11.5784C9.44364 11.8521 8.55596 11.8521 7.73489 11.5784L7.17244 11.3909C7.13436 11.3782 7.09607 11.3667 7.05762 11.3564L8.12117 10.2929C8.68378 9.73028 8.99985 8.96722 8.99985 8.17157Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBell;
impl IconShape for HiBell {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 2C6.68632 2 4.00003 4.68629 4.00003 8V11.5858L3.29292 12.2929C3.00692 12.5789 2.92137 13.009 3.07615 13.3827C3.23093 13.7564 3.59557 14 4.00003 14H16C16.4045 14 16.7691 13.7564 16.9239 13.3827C17.0787 13.009 16.9931 12.5789 16.7071 12.2929L16 11.5858V8C16 4.68629 13.3137 2 10 2Z",
            }
            path {
                d: "M10 18C8.34315 18 7 16.6569 7 15H13C13 16.6569 11.6569 18 10 18Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookOpen;
impl IconShape for HiBookOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 4.80423C7.9428 4.28906 6.75516 4 5.5 4C4.24484 4 3.0572 4.28906 2 4.80423V14.8042C3.0572 14.2891 4.24484 14 5.5 14C7.1686 14 8.71789 14.5108 10 15.3847C11.2821 14.5108 12.8314 14 14.5 14C15.7552 14 16.9428 14.2891 18 14.8042V4.80423C16.9428 4.28906 15.7552 4 14.5 4C13.2448 4 12.0572 4.28906 11 4.80423V12C11 12.5523 10.5523 13 10 13C9.44772 13 9 12.5523 9 12V4.80423Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookmarkAlt;
impl IconShape for HiBookmarkAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 3.89543 3.89543 3 5 3H15C16.1046 3 17 3.89543 17 5V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V5ZM14 6H6V14L10 12L14 14V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFM0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+g73pO33YW+H20vxnUPnrX/yRb/gZ/uMg/oO8+Tt/8Gtn8l2YB/OfSdwawfv8/A9/82/z74f4D/Cm7/DB7w18F/+FivQ2P/ej3/TT/Psg/p3e+p0/+MFj018ZH+e/kNButzl7yE9/91fv8m+H+Hd603f44K8GPor/Hl/ziz/2zR/Nvx3i3+lN3/GDn455MP8dxK2/+KPf/BD+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5b/SLP/bN4t8O8e/0pu/wwea/0S/+2DeLfzvEv9ObvsMHm/9Gv/hj3yz+7RD/Tm/6Dh9s/hv94o99s/i3Q/w7vek7fLD5t3sGVzyIf6Nf/LFvFv92iH+nN32HDzb/es8APvsXf+ybvxvgTd/hg98b+GzgQfwr/eKPfbP4t0P8O73pO3ywedE9A/jsX/yxb/5uno83fYcPfm/gs4EH8SL6xR/7ZvFvh/h3etN3+GDzL3sG8Nm/+GPf/N28CN70HT74vYHPBh7Ev+AXf+ybxb8d4t/pTd/hg80L9gzgs3/xx775u/k3eNN3+OD3Bj4beBAvwC/+2DeLfzvEv9ObvsMHm+f1DOCzf/HHvvm7+Q/wpu/wwe8NfDbwIJ7LL/7YN4t/O8S/05u+wwebZ3sG8Nm/+GPf/N38J3jTd/jg9wY+G3gQz/SLP/bN4t8O8e/0pu/wwQaeAXz2L/7YN383/wXe9B0++L2BzwYe9Is/9s3i3w7x7/Sm7/DB7/2LP/bN381/gzd9hw9+71/8sW/+bv7tEP+/If5/Q/z/hvj/DfH/G+L/N/4RpRURUArAkSAAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookmark;
impl IconShape for HiBookmark {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 4C5 2.89543 5.89543 2 7 2H13C14.1046 2 15 2.89543 15 4V18L10 15.5L5 18V4Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBriefcase;
impl IconShape for HiBriefcase {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 6V5C6 3.34315 7.34315 2 9 2H11C12.6569 2 14 3.34315 14 5V6H16C17.1046 6 18 6.89543 18 8V11.5708C15.5096 12.4947 12.8149 12.9999 10 12.9999C7.18514 12.9999 4.49037 12.4947 2 11.5707V8C2 6.89543 2.89543 6 4 6H6ZM8 5C8 4.44772 8.44772 4 9 4H11C11.5523 4 12 4.44772 12 5V6H8V5ZM9 10C9 9.44772 9.44772 9 10 9H10.01C10.5623 9 11.01 9.44772 11.01 10C11.01 10.5523 10.5623 11 10.01 11H10C9.44772 11 9 10.5523 9 10Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M2 13.6923V16C2 17.1046 2.89543 18 4 18H16C17.1046 18 18 17.1046 18 16V13.6923C15.4872 14.5404 12.7964 14.9999 10 14.9999C7.20363 14.9999 4.51279 14.5404 2 13.6923Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCake;
impl IconShape for HiCake {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 3C6 2.44772 6.44772 2 7 2H7.01C7.56228 2 8.01 2.44772 8.01 3C8.01 3.55228 7.56228 4 7.01 4H7C6.44772 4 6 3.55228 6 3ZM8 6C8 5.44772 7.55228 5 7 5C6.44772 5 6 5.44772 6 6V7C4.89543 7 4 7.89543 4 9V10C2.89543 10 2 10.8954 2 12V12.6833C2.36868 12.7866 2.72499 12.9482 3.0547 13.168C3.62713 13.5496 4.37287 13.5496 4.9453 13.168C6.18953 12.3385 7.81047 12.3385 9.0547 13.168C9.62713 13.5496 10.3729 13.5496 10.9453 13.168C12.1895 12.3385 13.8105 12.3385 15.0547 13.168C15.6271 13.5496 16.3729 13.5496 16.9453 13.168C17.275 12.9482 17.6313 12.7866 18 12.6833V12C18 10.8954 17.1046 10 16 10V9C16 7.89543 15.1046 7 14 7V6C14 5.44772 13.5523 5 13 5C12.4477 5 12 5.44772 12 6V7H11V6C11 5.44772 10.5523 5 10 5C9.44772 5 9 5.44772 9 6V7H8V6ZM18 14.8679C16.7633 15.6614 15.1714 15.6495 13.9453 14.8321C13.3729 14.4505 12.6271 14.4505 12.0547 14.8321C10.8105 15.6616 9.18953 15.6616 7.9453 14.8321C7.37287 14.4505 6.62713 14.4505 6.0547 14.8321C4.82863 15.6495 3.23675 15.6614 2 14.8679V17C2 17.5523 2.44772 18 3 18H17C17.5523 18 18 17.5523 18 17V14.8679ZM9 3C9 2.44772 9.44772 2 10 2H10.01C10.5623 2 11.01 2.44772 11.01 3C11.01 3.55228 10.5623 4 10.01 4H10C9.44772 4 9 3.55228 9 3ZM12 3C12 2.44772 12.4477 2 13 2H13.01C13.5623 2 14.01 2.44772 14.01 3C14.01 3.55228 13.5623 4 13.01 4H13C12.4477 4 12 3.55228 12 3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCalculator;
impl IconShape for HiCalculator {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V4C16 2.89543 15.1046 2 14 2H6ZM7 4C6.44772 4 6 4.44772 6 5C6 5.55228 6.44772 6 7 6H13C13.5523 6 14 5.55228 14 5C14 4.44772 13.5523 4 13 4H7ZM13 11C13.5523 11 14 11.4477 14 12V15C14 15.5523 13.5523 16 13 16C12.4477 16 12 15.5523 12 15V12C12 11.4477 12.4477 11 13 11ZM10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16H10.01C10.5623 16 11.01 15.5523 11.01 15C11.01 14.4477 10.5623 14 10.01 14H10ZM6 15C6 14.4477 6.44772 14 7 14H7.01C7.56228 14 8.01 14.4477 8.01 15C8.01 15.5523 7.56228 16 7.01 16H7C6.44772 16 6 15.5523 6 15ZM7 11C6.44772 11 6 11.4477 6 12C6 12.5523 6.44772 13 7 13H7.01C7.56228 13 8.01 12.5523 8.01 12C8.01 11.4477 7.56228 11 7.01 11H7ZM9 12C9 11.4477 9.44772 11 10 11H10.01C10.5623 11 11.01 11.4477 11.01 12C11.01 12.5523 10.5623 13 10.01 13H10C9.44772 13 9 12.5523 9 12ZM13 8C12.4477 8 12 8.44772 12 9C12 9.55228 12.4477 10 13 10H13.01C13.5623 10 14.01 9.55228 14.01 9C14.01 8.44772 13.5623 8 13.01 8H13ZM9 9C9 8.44772 9.44772 8 10 8H10.01C10.5623 8 11.01 8.44772 11.01 9C11.01 9.55228 10.5623 10 10.01 10H10C9.44772 10 9 9.55228 9 9ZM7 8C6.44772 8 6 8.44772 6 9C6 9.55228 6.44772 10 7 10H7.01C7.56228 10 8.01 9.55228 8.01 9C8.01 8.44772 7.56228 8 7.01 8H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCalendar;
impl IconShape for HiCalendar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C5.44772 2 5 2.44772 5 3V4H4C2.89543 4 2 4.89543 2 6V16C2 17.1046 2.89543 18 4 18H16C17.1046 18 18 17.1046 18 16V6C18 4.89543 17.1046 4 16 4H15V3C15 2.44772 14.5523 2 14 2C13.4477 2 13 2.44772 13 3V4H7V3C7 2.44772 6.55228 2 6 2ZM6 7C5.44772 7 5 7.44772 5 8C5 8.55228 5.44772 9 6 9H14C14.5523 9 15 8.55228 15 8C15 7.44772 14.5523 7 14 7H6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCamera;
impl IconShape for HiCamera {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 5C2.89543 5 2 5.89543 2 7V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V7C18 5.89543 17.1046 5 16 5H14.4142C14.149 5 13.8946 4.89464 13.7071 4.70711L12.5858 3.58579C12.2107 3.21071 11.702 3 11.1716 3H8.82843C8.29799 3 7.78929 3.21071 7.41421 3.58579L6.29289 4.70711C6.10536 4.89464 5.851 5 5.58579 5H4ZM10 14C11.6569 14 13 12.6569 13 11C13 9.34315 11.6569 8 10 8C8.34315 8 7 9.34315 7 11C7 12.6569 8.34315 14 10 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCash;
impl IconShape for HiCash {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C2.89543 4 2 4.89543 2 6V10C2 11.1046 2.89543 12 4 12V6H14C14 4.89543 13.1046 4 12 4H4ZM6 10C6 8.89543 6.89543 8 8 8H16C17.1046 8 18 8.89543 18 10V14C18 15.1046 17.1046 16 16 16H8C6.89543 16 6 15.1046 6 14V10ZM12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartBar;
impl IconShape for HiChartBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 11C2 10.4477 2.44772 10 3 10H5C5.55228 10 6 10.4477 6 11V16C6 16.5523 5.55228 17 5 17H3C2.44772 17 2 16.5523 2 16V11Z",
            }
            path {
                d: "M8 7C8 6.44772 8.44772 6 9 6H11C11.5523 6 12 6.44772 12 7V16C12 16.5523 11.5523 17 11 17H9C8.44772 17 8 16.5523 8 16V7Z",
            }
            path {
                d: "M14 4C14 3.44772 14.4477 3 15 3H17C17.5523 3 18 3.44772 18 4V16C18 16.5523 17.5523 17 17 17H15C14.4477 17 14 16.5523 14 16V4Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartPie;
impl IconShape for HiChartPie {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 10C2 5.58172 5.58172 2 10 2V10H18C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10Z",
            }
            path {
                d: "M12 2.25195C14.8113 2.97552 17.0245 5.18877 17.748 8.00004H12V2.25195Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartSquareBar;
impl IconShape for HiChartSquareBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 3C3.89543 3 3 3.89543 3 5V15C3 16.1046 3.89543 17 5 17H15C16.1046 17 17 16.1046 17 15V5C17 3.89543 16.1046 3 15 3H5ZM14 7C14 6.44772 13.5523 6 13 6C12.4477 6 12 6.44772 12 7V13C12 13.5523 12.4477 14 13 14C13.5523 14 14 13.5523 14 13V7ZM11 9C11 8.44772 10.5523 8 10 8C9.44772 8 9 8.44772 9 9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V9ZM8 12C8 11.4477 7.55228 11 7 11C6.44772 11 6 11.4477 6 12V13C6 13.5523 6.44772 14 7 14C7.55228 14 8 13.5523 8 13V12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChatAlt2;
impl IconShape for HiChatAlt2 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 5C2 3.89543 2.89543 3 4 3H11C12.1046 3 13 3.89543 13 5V9C13 10.1046 12.1046 11 11 11H9L6 14V11H4C2.89543 11 2 10.1046 2 9V5Z",
            }
            path {
                d: "M15 7V9C15 11.2091 13.2091 13 11 13H9.82843L8.06173 14.7667C8.34154 14.9156 8.66091 15 9 15H11L14 18V15H16C17.1046 15 18 14.1046 18 13V9C18 7.89543 17.1046 7 16 7H15Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChatAlt;
impl IconShape for HiChatAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 5V13C18 14.1046 17.1046 15 16 15H11L6 19V15H4C2.89543 15 2 14.1046 2 13V5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5ZM7 8H5V10H7V8ZM9 8H11V10H9V8ZM15 8H13V10H15V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChat;
impl IconShape for HiChat {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 13.866 14.4183 17 10 17C8.50836 17 7.11208 16.6428 5.91677 16.0208L2 17L3.3383 13.8773C2.4928 12.7673 2 11.434 2 10C2 6.13401 5.58172 3 10 3C14.4183 3 18 6.13401 18 10ZM7 9H5V11H7V9ZM15 9H13V11H15V9ZM9 9H11V11H9V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIIklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+C73FO3zIa7WI47i9NM+Pyl+TfsYv/tg3/jX/NRD/id76nT/4wVOLtzL51obX5l9B8NsifrqW/Jmf/uFvvpX/HIj/BG/2jh/yXjYfDX5p/gMIfhvpu3/hR7/pe/iPhfgP9Bbv+CFv3fBXYR7MfwZxa0Ef83M/+k0/zX8MxH+At37nD37w2Pguw2vzX0Dw213hfX76h7/5Vv59EP9Ob/oOH/zeQl9lfJz/QkK7Id7n5370m36afzvEv8ObvsMHfxfw3vw3kvTVv/Cj3/Qx/Nsg/o3e9B0++LuA9+Z/hu/+xR/75vfhXw/xb/Cm7/DB3wW8N/+zfPcv/tg3vw//Ooh/pTd9hw/+LuC9+Z/pa37xx775o3nRIf4V3vQdPvi9ge/if7Aivc3P/eg3/TQvGsSL6K3f+YMfPDb9lfFx/gcT2u2KX+anf/ibb+VfhngRvdk7fPBvGV6b/wUEv/0LP/bNr8O/DPEieIt3/JC3bvZP8b9Ikd7m5370m36aFw7xInjTd/zgp2MezP8m4tZf/NFvfggvHOJf8Kbv8MHvDXwX/zu9zy/+2Dd/Ny8Y4l/wZu/wwb9leG3+m0h8j+FWzHHgvYFjvIgEv/0LP/bNr8MLhngh3vqdP/jBQ+Pp/Pe4FIW3/vkf/ubf5pne+p0/+MFD46+BY7yI+sJDfvqHv/lWnj/EC/Fm7/QhH+30V/Ff7xLEa//ij33jX/Nc3uydPuSjnf4qXkQKfcwv/Mg3fTXPH+KFeNN3+ODfBl6L/1qXIF77F3/sG/+a5+PN3/mDXzsbv8WL7nd+8ce++bV5/hAvxJu+wweb/1qXIF77F3/sG/+aF+BN3+GD3xv4Lv4VfvHHvlk8f4gX4M3f+YNfOxu/xX+dSxCv/Ys/9o1/zQvw1u/90ceHo9VfYR7Mv0q8zC/+2Df+Nc8L8QK8xTt+yFs3+6f4N5L4G5uX4kVzCeK1f/HHvvGveQHe+r0/+vhwuP4t8Evzr1Skt/m5H/2mn+Z5IV6AN33HD/5szGfxryU+5xd/9Js/G+Ct3/ujjw+Hq+8G3ooX7BLEa//ij33jX/MCvPV7f/Tx4XD9W+CX5t9CfM4v/ug3fzbPC/ECvOk7fvBnYz6Lf52f+cUf++a35rm82Tt+8HfbvBfP6xLEa//ij33jX/MCvPV7f/Tx4XD9W+CX5t9KfM4v/ug3fzbPC/ECvOk7fvBnYz6Lf4UovM7P//A3/zbPx5u94wd/t8178WyXIF77F3/sG/+aF+Ct3/ujjw+H698CvzT/HuJzfvFHv/mzeV6IF+BN3/GDPxvzWfwrROF1fv6Hv/m3eQHe7B0/+Ltt3gu4BPHav/hj3/jXvABv/d4ffXw4XP8W+KX59xKf84s/+s2fzfNCvABv+o4f/NmYz+JfQ/rpX/zRb3obXog3fYcP/mqI7/7FH/vGv+YFeOv3/ujjw+H6t8AvzX8E8Tm/+KPf/Nk8L8QL8Kbv+MGfjfks/vXe5xd/7Ju/m3+jt37vjz4+HK5/C/zS/EcRn/OLP/rNn83zQrwAb/GOH/LWzf4p/m3e5xd/7Ju/m3+lt37vjz4+HK5/C/zS/Acq0tv83I9+00/zvBAvwJu/8we/djZ+i3+79/nFH/vm7+ZF9Nbv/dHHh8P1b4Ffmv9w8TK/+GPf+Nc8L8QL8abv8MHm3+d9fvHHvvm7+Re89Xt/9PHhcP1b4JfmP8Ev/tg3i+cP8UK86Tt88G8Dr8W/z/v84o9983fzArz1e3/08eFw/Vvgl+Y/x+/84o9982vz/CFeiDd7pw/5aKe/in+/9/nFH/vm7+a5vPV7f/Tx4XD9W+CX5j+JQh/zCz/yTV/N84d4Id76nT/4wUPj6fzHeJ9f/LFv/m6e6a3f+6OPD4fr3wK/NP+J+sJDfvqHv/lWnj/Ev+BN3+GDfxt4Lf4DCH7b4rcBZH208XH+c/3OL/7YN782LxjiX/Cm7/DB7w18F/87vc8v/tg3fzcvGOJF8Kbv8MG3Ag/if5dn/OKPffODeeEQL4K3eMcPeetm/xT/ixTpbX7uR7/pp3nhEC+iN32HD/5t4LX43+F3fvHHvvm1+ZchXkRv/c4f/OCh8dfAMf5nu9QXXvqnf/ibb+VfhvhXeIt3/JC3bvZP8T9Ykd7m5370m36aFw3iX+nN3vGDv9vmvfif6Wt+8ce++aN50SH+Dd7sHT/4u23ei/9BJL7nF370m9+bfx3Ev9GbveMHf7fNe/E/gMT3/MKPfvN786+H+Hd403f44K8GPor/Xl/ziz/2zR/Nvw3i3+kt3vFD3rrZ3w0c47/WpSK998/96Df9NP92iP8Ab/3OH/zgofHdwGvxX+N3+sJ7//QPf/Ot/Psg/gO9xTt+yFs3+6uBB/Gf4xlF+uif+9Fv+mn+YyD+E7zpO3zwewPvDbwW/zF+B/juX/yxb/5u/mMh/hO99Tt/8INH662dfmvgtfjX+R2FfrqTf/qnf/ibb+U/B+K/0Ju+w4e+dJEf3PBL83wU9NfNuvUXf+wb/5r/Goj/3xD/vyH+f0P8/4b4/w3x/xv/CFzwQF+mBcE3AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCheckCircle;
impl IconShape for HiCheckCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFLUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+D/gzd/hQz8qlR8t67jxT/eb84/56e/+6l3+ZYj/5d70HT74u4D35jnor3/xx77pZfiXIf4Xe9N3+ODvAt6b5yMKr/PzP/zNv80Lh/hf6k3f4YO/C3hvXoAivc3P/eg3/TQvHOJ/oTd9hw/+LuC9ecEu9YWX/ukf/uZbeeEQ/8u86Tt88HcB780L9z6/+GPf/N38yxD/i7zpO3zwdwHvzQv3Pr/4Y9/83bxoEP9LvOk7fPB3Ae/NC/c+v/hj3/zdvOgQ/wu86Tt88HcB780L9z6/+GPf/N386yD+h3vTd/jg7wLemxfufX7xx775u/nXQ/wP9qbv8MHfBbw3L9z7/OKPffN382+D+B/qTd/hg78LeG9euPf5xR/75u/m3w7xr/TW7/zBD2Y+3/3p7/7qXf6TvOk7fPB3Ae/NC/c+v/hj3/zd/PsgXkRv+g4f+tIofwrzYACkn+43Zu/z09/91bv8B3rTd/jg7wLemxfufX7xx775u/n3Q7wI3vq9P/r4eLh+uvFxnoP+ut+cvc5Pf/dX7/If4E3f4YO/C3hvXrj3+cUf++bv5j8G4kXwFu/4IW/d7J/i+dJf95uz1/np7/7qXf4d3vQdPvi7gPfmhXufX/yxb/5u/uMgXgRv8Y4f8tbN/ileIP11vzl7nZ/+7q/e5d/gTd/hg78LeG9euPf5xR/75u/mPxbiRfDW7/3Rx4fD1a3AMV4g/XW/OXudn/7ur97lX+FN3+GDvwt4b1649/nFH/vm7+Y/HuJF9Kbv8KEvDfnbwDFeIP11vzl7nZ/+7q/e5UXwpu/wwd8FvDcv3Pv84o9983fznwPxr/Cm7/ChLw3528AxXiD9db85e52f/u6v3uWFeNN3+ODvAt6bF+59fvHHvvm7+c+D+Fd603f40JeG/G3gGC+Q/rrfnL3OT3/3V+/yfLzpO3zwdwHvzQv3Pr/4Y9/83fznQvwbvOk7fOhLQ/42cIwXSH/db85e56e/+6t3eYA3fYcP/i7gvXnh3ucXf+ybv5v/fIh/ozd9hw99acjfBo7xAumv+83Z6/z0d3/1LsCbvsMHfxfw3rxw7/OLP/bN381/DcS/w5u+w4e+NORvA8d4gfTX/ebsdYbD1VcB780L9z6/+GPf/N3810H8O73pO3zoS0P+NnCMF0Bo1/g4L9z7/OKPffN3818L8R/gTd/hQ18a8reBY/zbvM8v/tg3fzf/9RD/Qd70HT70pSF/GzjGv877/OKPffN3898D8R/oTd/hQ18a8reBY7xo3ucXf+ybv5v/Poj/YG/6Dh/60pC/DRzjhXufX/yxb/5u/nsh/hO86Tt86EtD/jZwjOfvfX7xx775u/nvh/hP8qbv8KEvDfnbwDGe0/v84o9983fzPwPiP9GbvsOHvjTkZwNvJfE3gT775370m36a/zkQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3/hHHyAPUIp+OD4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCheck;
impl IconShape for HiCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M16.7071 5.29289C17.0976 5.68342 17.0976 6.31658 16.7071 6.70711L8.70711 14.7071C8.31658 15.0976 7.68342 15.0976 7.29289 14.7071L3.29289 10.7071C2.90237 10.3166 2.90237 9.68342 3.29289 9.29289C3.68342 8.90237 4.31658 8.90237 4.70711 9.29289L8 12.5858L15.2929 5.29289C15.6834 4.90237 16.3166 4.90237 16.7071 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleDown;
impl IconShape for HiChevronDoubleDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L10.7071 10.7071C10.3166 11.0976 9.68342 11.0976 9.29289 10.7071L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10 8.58579L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289ZM15.7071 10.2929C16.0976 10.6834 16.0976 11.3166 15.7071 11.7071L10.7071 16.7071C10.3166 17.0976 9.68342 17.0976 9.29289 16.7071L4.29289 11.7071C3.90237 11.3166 3.90237 10.6834 4.29289 10.2929C4.68342 9.90237 5.31658 9.90237 5.70711 10.2929L10 14.5858L14.2929 10.2929C14.6834 9.90237 15.3166 9.90237 15.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleLeft;
impl IconShape for HiChevronDoubleLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L9.29289 10.7071C8.90237 10.3166 8.90237 9.68342 9.29289 9.29289L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L11.4142 10L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071ZM9.70711 15.7071C9.31658 16.0976 8.68342 16.0976 8.29289 15.7071L3.29289 10.7071C2.90237 10.3166 2.90237 9.68342 3.29289 9.29289L8.29289 4.29289C8.68342 3.90237 9.31658 3.90237 9.70711 4.29289C10.0976 4.68342 10.0976 5.31658 9.70711 5.70711L5.41421 10L9.70711 14.2929C10.0976 14.6834 10.0976 15.3166 9.70711 15.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleRight;
impl IconShape for HiChevronDoubleRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 15.7071C9.90237 15.3166 9.90237 14.6834 10.2929 14.2929L14.5858 10L10.2929 5.70711C9.90237 5.31658 9.90237 4.68342 10.2929 4.29289C10.6834 3.90237 11.3166 3.90237 11.7071 4.29289L16.7071 9.29289C17.0976 9.68342 17.0976 10.3166 16.7071 10.7071L11.7071 15.7071C11.3166 16.0976 10.6834 16.0976 10.2929 15.7071Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L8.58579 10L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10.7071 9.29289C11.0976 9.68342 11.0976 10.3166 10.7071 10.7071L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleUp;
impl IconShape for HiChevronDoubleUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L9.29289 9.29289C9.68342 8.90237 10.3166 8.90237 10.7071 9.29289L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L10 11.4142L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071ZM4.29289 9.70711C3.90237 9.31658 3.90237 8.68342 4.29289 8.29289L9.29289 3.29289C9.68342 2.90237 10.3166 2.90237 10.7071 3.29289L15.7071 8.29289C16.0976 8.68342 16.0976 9.31658 15.7071 9.70711C15.3166 10.0976 14.6834 10.0976 14.2929 9.70711L10 5.41421L5.70711 9.70711C5.31658 10.0976 4.68342 10.0976 4.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD3klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If4DvPV7f/Tx4Wj1UQAR/M7P//A3/zb/Cd78nT/4tTN5LYB+Y/41P/3dX73Lvw/i3+lN3+FDX1r4t4yP82zf/Ys/9s3vw3+gN32HD/4u4L15JqFdo9f5xR/7xr/m3w7x7/Sm7/DBPw28Fc/ru3/xx775ffgP8Kbv8MHfBbw3z+tnfvHHvvmt+bdD/Du96Tt8sHnBvvsXf+yb34d/hzd9hw/+LuC9eQF+8ce+WfzbIf6d3uwdP/ivbV6KF+y7f/HHvvl9+Dd403f44O8C3psXQOJvfuFHv/ml+bdD/Du9xTt+yFs3+6d44b77F3/sm9+Hf4U3fYcP/i7gvXkhivQ2P/ej3/TT/Nsh/gO86Tt88HsD38UL992/+GPf/D68CN70HT74u4D35oV7n1/8sW/+bv59EP9B3vQdPvi9ge/ihfvuX/yxb34fXog3fYcP/i7gvXnh3ucXf+ybv5t/P8R/oDd9hw9+b+C7eOG++xd/7Jvfh+fjTd/hg78LeG9euPf5xR/75u/mPwbiP9ibvsMHvzfwXbxw3/2LP/bN78MDvOk7fPB3Ae/NC/c+v/hj3/zd/MdB/Cd403f44PcGvosX7rt/8ce++X0A3vQdPvi7gPfmhXufX/yxb/5u/mMh/pO86Tt88HsD38UL991c8d68cO/ziz/2zd/NfzzEf6I3fYcPfm/gu/j3eZ9f/LFv/m7+cyD+k73pO3zwewPfxb/N+/zij33zd/OfB/Ff4E3f4YPfG/gu/nXe5xd/7Ju/m/9ciP8ib/oOH/zewHfxonmfX/yxb/5u/vMh/gu96Tt88HsD38UL9z6/+GPf/N3810D8F3vTd/jg9wa+i+fvfX7xx775u/mvg/hv8Kbv8MHvDXwXz+l9fvHHvvm7+a+F+G/ypu/woS+N8q0BcPz0L/7YN/41//UQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EbObrEFphSqSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDown;
impl IconShape for HiChevronDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 7.29289C5.68342 6.90237 6.31658 6.90237 6.70711 7.29289L10 10.5858L13.2929 7.29289C13.6834 6.90237 14.3166 6.90237 14.7071 7.29289C15.0976 7.68342 15.0976 8.31658 14.7071 8.70711L10.7071 12.7071C10.3166 13.0976 9.68342 13.0976 9.29289 12.7071L5.29289 8.70711C4.90237 8.31658 4.90237 7.68342 5.29289 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAECklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+B/mLd7xQ966mc8CvzTST2N9zi/+2Df+Nf85EP+DvOk7fPB7A9/FAwjtdpuzh/z0d3/1Lv/xEP9DvOk7fPB7A9/F8yM+5xd/9Js/m/94iP8B3vQdPvi9ge/iBRGf84s/+s2fzX88xH+zN32HD35v4Lt4IaLwOj//w9/82/zHQ/w3etN3+OD3Br6LF0Lie37hR7/5vfnPgfhv8qbv8MHvDXwXL4TE9/zCj37ze/OfB/Hf4E3f4YPfG/guXgiJ7/mFH/3m9+Y/F+K/2Ju+wwe/N/BdvBAS3/MLP/rN781/PsR/oTd9hw9+b+C7eCEkvucXfvSb35v/Goj/Im/6Dh/83sB38UJIfM8v/Og3vzf/dRD/Bd70HT74vYHv4oWQ+J5f+NFvfm/+ayH+k73pO3zwewPfxQsh8T2/8KPf/N7810P8J3rTd/jg9wa+ixdC4nt+4Ue/+b3574H4T/Km7/ChLw35V7wQEt/zCz/6ze/Nfx/Ef5I3fccP/mzMZ/ECSHzPL/zoN783/70Q/0ne9B0/+LMxn8UL9t2/+GPf/D7890L8J3nTd/jQl4b8K1647/7FH/vm9+G/D+I/0Zu+wwe/N/BdvHDf/Ys/9s3vw38PxH+yN32HD35v4Lt44b77F3/sm9+H/3qI/wJv+g4f/N7Ad/HCffcv/tg3vw//tRD/Rd70HT74vYHv4oX77l/8sW9+H/7rIP4Lvek7fPB7A9/FC/fdv/hj3/w+/NdA/Bd703f44PcGvosX7rt/8ce++X34z4f4b/Cm7/DB7w18Fy/cd//ij33z+/CfC/Hf5E3f4YPfG/guXrjv/sUf++b34T8P4r/Rm77DB7838F28cN/9iz/2ze/Dfw7Ef7M3fYcPfm/gu3ghovA6P//D3/zb/MdD/A/wpu/wwe8NfBcviPicX/zRb/5s/uMh/od403f44PcGvovnR3zOL/7oN382//EQ/4O86Tt88HsD38VzutRvzh/809/91bv8x0P8D/MW7/ghb534s21eCvgZiM/+xR/7xr/mPwfi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IRqywQRS/CiQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronLeft;
impl IconShape for HiChevronLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.7071 5.29289C13.0976 5.68342 13.0976 6.31658 12.7071 6.70711L9.41421 10L12.7071 13.2929C13.0976 13.6834 13.0976 14.3166 12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929 14.7071L7.29289 10.7071C6.90237 10.3166 6.90237 9.68342 7.29289 9.29289L11.2929 5.29289C11.6834 4.90237 12.3166 4.90237 12.7071 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD/0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+E/0pu/woS+N/FnYbw366yI+5+d+9Jt+mv85EP9J3vqdP/jBY9NfGR/nOb3PL/7YN383/zMg/pO86Tt+8GdjPovn731+8ce++bv574f4T/Km7/jBn435LF6w9/nFH/vm7+a/F+I/yVu844e8dbN/ihfufX7xx775u/nvg/hP9Gbv+MHfbfNevHDv84s/9s3fzX8PxH+yN3vHD/5um/fihXufX/yxb/5u/ush/gu82Tt+8HfbvBcv3Pv84o9983fzXwvxX+TN3vGDv9vmvXjh3ucXf+ybv5v/Ooj/Qm/2jh/83TbvxQv3Pr/4Y9/83fzXQPwXe7N3/ODvtnkvXrj3+cUf++bv5j8f4r/Bm73jB3+3zXvxwr3PL/7YN383/7kQ/03e7B0/+Ltt3osX7n1+8ce++bv5z4P4b/Rm7/jB323zXrxw7/OLP/bN381/DsR/szd7xw/+bpv34oWKl/nFH/vGv+Y/HuJ/gDd7xw/+bpv34gURn/OLP/rNn81/PMT/AG/6Dh/8XcB784KIz/nFH/3mz+Y/HuK/2Zu+wwd/F/DevFDxMr/4Y9/41/zHQ/w3etN3+ODvAt6bF+59fvHHvvm7+c+B+G/ypu/wwd8FvDcv3Pv84o9983fznwfx3+BN3+GDvwt4b1649/nFH/vm7+Y/F+K/2Ju+wwd/F/DevHDv84s/9s3fzX8+xH+hN32HD/4u4L154d7nF3/sm7+b/xqI/yJv+g4f/F3Ae/PCvc8v/tg3fzf/dRD/Bd70HT74u4D35oV7n1/8sW/+bv5rIf6Tvek7fPB3Ae/NC/c+v/hj3/zd/NdD/Cd603f44O8C3psX7n1+8ce++bv574H4T/Lm7/zBr52N3+KFe59f/LFv/m7++yD+k7zpO37wZ2M+ixfsfX7xx775u/nvhfhP8qbv+MGfjfksnr/3+cUf++bv5r8f4j/JW7/3Rx8fDle3Asd4Tu/ziz/2zd/N/wyI/0Rv+g4f+tKQnw28lcTfBPrsn/vRb/pp/udA/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfARW7zEHgXPWdAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronRight;
impl IconShape for HiChevronRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.29289 14.7071C6.90237 14.3166 6.90237 13.6834 7.29289 13.2929L10.5858 10L7.29289 6.70711C6.90237 6.31658 6.90237 5.68342 7.29289 5.29289C7.68342 4.90237 8.31658 4.90237 8.70711 5.29289L12.7071 9.29289C13.0976 9.68342 13.0976 10.3166 12.7071 10.7071L8.70711 14.7071C8.31658 15.0976 7.68342 15.0976 7.29289 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If6bvOk7fOhLo3wrABw/84s/9o1/zX89xH+DN32HD35v4Lt4Tu/ziz/2zd/Nfy3Ef7E3fYcPfm/gu3j+3ucXf+ybv5v/Ooj/Qm/6Dh/83sB38cK9zy/+2Dd/N/81EP9F3vQdPvi9ge/iRfM+v/hj3/zd/OdD/Bd403f44PcGvot/nff5xR/75u/mPxfiP9mbvsMHvzfwXfzbvM8v/tg3fzf/eRD/id70HT74vYHv4t/nfX7xx775u/nPgfhP8qbv8MHvDXwXL4TE9wDYvBcv3Pv84o9983fzHw/xn+BN3+GD3xv4Ll4Iie/5hR/95vcGeLN3/ODvtnkvXrj3+cUf++bv5j8W4j/Ym77DB7838F28EBLf8ws/+s3vzQO82Tt+8HfbvBcv3Pv84o9983fzHwfxH+hN3+GD3xv4Ll4Iie/5hR/95vfm+Xizd/zg77Z5L1649/nFH/vm7+Y/BuI/yJu+wwe/N/BdvBAS3/MLP/rN780L8Wbv+MHfbfNevHDv84s/9s3fzb8f4j/Am77DB7838F28EBLf8ws/+s3vzYvgzd7xg7/b5r144d7nF3/sm7+bfx/Ev9NbvOOHvHWzf4oXQuJ7fuFHv/m9+Vd4s3f84O+2eS9eiCK9zc/96Df9NP92iH+nN32HD/kr8EvzAkh8zy/86De/N/8Gb/aOH/zdNu/FC6S//sUf+6aX4d8O8e/0pu/wweYFkPieX/jRb35v/h3e7B0/+Ltt3osX4Bd/7JvFvx3i3+lN3+GDfxp4K56LxPf8wo9+83vzH+DN3vGDv9vmvXheP/OLP/bNb82/HeLf6U3f4UNfGvK3gWM8k8T3/MKPfvN78x/ozd7xg7/b5r14tksQr/2LP/aNf82/HeI/wFu/90cfH45WHw0QwW///A9/82/zn+DN3/mDXzuT1wboN+Zf/dPf/dW7/Psg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I40yvkEUGhJGAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronUp;
impl IconShape for HiChevronUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 12.7071C14.3166 13.0976 13.6834 13.0976 13.2929 12.7071L10 9.41421L6.70711 12.7071C6.31658 13.0976 5.68342 13.0976 5.29289 12.7071C4.90237 12.3166 4.90237 11.6834 5.29289 11.2929L9.29289 7.29289C9.68342 6.90237 10.3166 6.90237 10.7071 7.29289L14.7071 11.2929C15.0976 11.6834 15.0976 12.3166 14.7071 12.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChip;
impl IconShape for HiChip {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 7H7V13H13V7Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M7 2C7 1.44772 7.44772 1 8 1C8.55228 1 9 1.44772 9 2V3H11V2C11 1.44772 11.4477 1 12 1C12.5523 1 13 1.44772 13 2V3H15C16.1046 3 17 3.89543 17 5V7H18C18.5523 7 19 7.44772 19 8C19 8.55228 18.5523 9 18 9H17V11H18C18.5523 11 19 11.4477 19 12C19 12.5523 18.5523 13 18 13H17V15C17 16.1046 16.1046 17 15 17H13V18C13 18.5523 12.5523 19 12 19C11.4477 19 11 18.5523 11 18V17H9V18C9 18.5523 8.55228 19 8 19C7.44772 19 7 18.5523 7 18V17H5C3.89543 17 3 16.1046 3 15V13H2C1.44772 13 1 12.5523 1 12C1 11.4477 1.44772 11 2 11H3V9H2C1.44772 9 1 8.55228 1 8C1 7.44772 1.44772 7 2 7H3V5C3 3.89543 3.89543 3 5 3H7V2ZM5 5H15V15H5V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardCheck;
impl IconShape for HiClipboardCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C8.44772 2 8 2.44772 8 3C8 3.55228 8.44772 4 9 4H11C11.5523 4 12 3.55228 12 3C12 2.44772 11.5523 2 11 2H9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M4 5C4 3.89543 4.89543 3 6 3C6 4.65685 7.34315 6 9 6H11C12.6569 6 14 4.65685 14 3C15.1046 3 16 3.89543 16 5V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V5ZM13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289C13.3166 8.90237 12.6834 8.90237 12.2929 9.29289L9 12.5858L7.70711 11.2929C7.31658 10.9024 6.68342 10.9024 6.29289 11.2929C5.90237 11.6834 5.90237 12.3166 6.29289 12.7071L8.29289 14.7071C8.68342 15.0976 9.31658 15.0976 9.70711 14.7071L13.7071 10.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardCopy;
impl IconShape for HiClipboardCopy {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 2C7.44772 2 7 2.44772 7 3C7 3.55228 7.44772 4 8 4H10C10.5523 4 11 3.55228 11 3C11 2.44772 10.5523 2 10 2H8Z",
            }
            path {
                d: "M3 5C3 3.89543 3.89543 3 5 3C5 4.65685 6.34315 6 8 6H10C11.6569 6 13 4.65685 13 3C14.1046 3 15 3.89543 15 5V11H10.4142L11.7071 9.70711C12.0976 9.31658 12.0976 8.68342 11.7071 8.29289C11.3166 7.90237 10.6834 7.90237 10.2929 8.29289L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071L10.2929 15.7071C10.6834 16.0976 11.3166 16.0976 11.7071 15.7071C12.0976 15.3166 12.0976 14.6834 11.7071 14.2929L10.4142 13H15V16C15 17.1046 14.1046 18 13 18H5C3.89543 18 3 17.1046 3 16V5Z",
            }
            path {
                d: "M15 11H17C17.5523 11 18 11.4477 18 12C18 12.5523 17.5523 13 17 13H15V11Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardList;
impl IconShape for HiClipboardList {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C8.44772 2 8 2.44772 8 3C8 3.55228 8.44772 4 9 4H11C11.5523 4 12 3.55228 12 3C12 2.44772 11.5523 2 11 2H9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M4 5C4 3.89543 4.89543 3 6 3C6 4.65685 7.34315 6 9 6H11C12.6569 6 14 4.65685 14 3C15.1046 3 16 3.89543 16 5V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V5ZM7 9C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H7.01C7.56228 11 8.01 10.5523 8.01 10C8.01 9.44772 7.56228 9 7.01 9H7ZM10 9C9.44772 9 9 9.44772 9 10C9 10.5523 9.44772 11 10 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H10ZM7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H7.01C7.56228 15 8.01 14.5523 8.01 14C8.01 13.4477 7.56228 13 7.01 13H7ZM10 13C9.44772 13 9 13.4477 9 14C9 14.5523 9.44772 15 10 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+C735O3/wa2fyWrwQEfzOz//wN/82/zUQ/0Xe9B0++KuBj+JF8zW/+GPf/NH850P8F3jTd/jQl4b8K/5V4mV+8ce+8a/5z4X4D/DW7/zBD25NDyrFz/jpH/7mW3kub/GOH/LWzf4p/hWK9DY/96Pf9NM8l7d+5w9+cGt6UCl+xk//8Dffyr8P4t/hTd/hQ19a5FcZXhsgCq/z8z/8zb/Nc3nzd/7g187Gb/GvEi/ziz/2jX/Nc3nzd/7g187GbwEIftvEx/zij33jX/Nvg/g3etN3+OD3Br6L5/Q+v/hj3/zdPB9v+g4f/NvAa/Gi+Zlf/LFvfmuejzd7pw/5aKe/iuf0Pr/4Y9/83fzrIf4N3vqdP/jBY9NfGR/nOf3ML/7YN781L8CbvdOHfLTTb80LodBP/8KPfNNX8wK86Tt88E8Db8UDCO12xS/z0z/8zbfyr4P4N3jTd/jgnwbeiuej35yf+Onv/upd/hO89Xt/9PHhcHWR50Pie37hR7/5vfnXQfwbvOk7fLB5ASS+5xd+9Jvfm/8Eb/aOH/zdNu/FC/CLP/bN4l8H8a/0pu/woS8N+Ve8EEV6m5/70W/6af4DvcU7fshbN/uneKHiZX7xx77xr3nRIf6V3vydP/i1s/FbvBBCu0av84s/9o1/zX+AN32HD31p4d8yPs4LEYXX+fkf/ubf5kWH+Fd683f+4NfOxm/xIgjio3/+x77xa/h3ePN3+NCPSvKreRFE4XV+/oe/+bd50SH+ld78nT/4tbPxW7yoxK3h+Opa8md++oe/+VZeBG/9zh/84KnFW6XyozEP5kUUhdf5+R/+5t/mRYf4V3rzd/7g187Gb/FvJPhtXgjDa/NvFIXX+fkf/ubf5kWH+Fd683f+4NfOxm/xP1AUXufnf/ibf5sXHeJf6c3f+YNfOxu/xf9AUXidn//hb/5tXnSIf6U3f+cPfu1s/Bb/A0XhdX7+h7/5t3nRIf6V3vydP/i1s/Fb/A8Uhdf5+R/+5t/mRYf4V3rzd/7g187Gb/E/UBRe5+d/+Jt/mxcd4l/pzd/5g187G7/F/0BReJ2f/+Fv/m1edIh/pTd/5w9+7Wz8Fv8DReF1fv6Hv/m3edEh/pXe/J0/+LWz8Vv8DxSF1/n5H/7m3+ZFh/hXevN3/uDXzsZv8T9QFF7n53/4m3+bFx3iX+nN3/mDXzsbv8X/QFF4nZ//4W/+bV50iH+lN3/nD37tbPwW/wNF4XV+/oe/+bd50SH+ld78nT/4tbPxW/wPFIXX+fkf/ubf5kWH+Fd683f+4NfOxm/xP1AUXufnf/ibf5sXHeJf6c3f+YNfOxu/xf9AUXidn//hb/5tXnSIf6U3f+cPfu1s/Bb/A0XhdX7+h7/5t3nRIf6V3vydP/i1s/Fb/A8Uhdf5+R/+5t/mRYf4V3rzd/7g187Gb/E/UBRe5+d/+Jt/mxcd4l/pzd/5g187G7/F/0BReJ2f/+Fv/m1edIh/pTd/5w9+7Wz8Fv8DReF1fv6Hv/m3edEh/pXe/J0/+LWz8Vv8DxSF1/n5H/7m3+ZFh/hXevN3/uDXzsZv8T9QFF7n53/4m3+bFx3iX+nN3/mDXzsbv8X/QFF4nZ//4W/+bV50iH+lN3/nD37tbPwW/wNF4XV+/oe/+bd50SH+ld78nT/4tbPxW/wPFIXX+fkf/ubf5kWH+Fd683f+4NfOxm/xP1AUXufnf/ibf5sXHeJf6c3f+YNfOxu/xf9AUXidn//hb/5tXnSIf6U3f+cPfu1s/Bb/A0XhdX7+h7/5t3nRIf6V3vydP/i1s/Fb/A8Uhdf5+R/+5t/mRYf4V3rzd/7g187Gb/E/UBRe5+d/+Jt/mxcd4l/pzd/5g187G7/F/0BReJ2f/+Fv/m1edIh/pTd/5w9+7Wz8Fv8DReF1fv6Hv/m3edEh/pXe/J0/+LWz8Vv8DxSF1/n5H/7m3+ZFh/g3eNN3+GDzP9Av/tg3i38dxL/Bm77DB/828Fr8z/Izv/hj3/zW/Osg/g3e/J0/+LWz8Vv8DxKF1/n5H/7m3+ZfB/Fv9Kbv+MGfjfks/icQn/OLP/rNn82/HuLf4U3f4YPfG/hq4Bj/PS4BH/2LP/bN382/DeLf6a3f+6OPD4ert0Y8mP9K5tZ+c/7TP/3dX73Lvx3i/zfE/2+I/98Q/78h/n9D/P/GPwKZZNRQGr94mwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboard;
impl IconShape for HiClipboard {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3C8 2.44772 8.44772 2 9 2H11C11.5523 2 12 2.44772 12 3C12 3.55228 11.5523 4 11 4H9C8.44772 4 8 3.55228 8 3Z",
            }
            path {
                d: "M6 3C4.89543 3 4 3.89543 4 5V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V5C16 3.89543 15.1046 3 14 3C14 4.65685 12.6569 6 11 6H9C7.34315 6 6 4.65685 6 3Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClock;
impl IconShape for HiClock {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 6C11 5.44772 10.5523 5 10 5C9.44771 5 9 5.44772 9 6V10C9 10.2652 9.10536 10.5196 9.29289 10.7071L12.1213 13.5355C12.5118 13.9261 13.145 13.9261 13.5355 13.5355C13.9261 13.145 13.9261 12.5118 13.5355 12.1213L11 9.58579V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloudDownload;
impl IconShape for HiCloudDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 9.5C2 11.433 3.567 13 5.5 13H9V15.5858L7.70711 14.2929C7.31658 13.9024 6.68342 13.9024 6.29289 14.2929C5.90237 14.6834 5.90237 15.3166 6.29289 15.7071L9.29289 18.7071C9.68342 19.0976 10.3166 19.0976 10.7071 18.7071L13.7071 15.7071C14.0976 15.3166 14.0976 14.6834 13.7071 14.2929C13.3166 13.9024 12.6834 13.9024 12.2929 14.2929L11 15.5858V13H13.5C15.9853 13 18 10.9853 18 8.5C18 6.01472 15.9853 4 13.5 4C13.2912 4 13.0857 4.01422 12.8845 4.04175C12.4551 2.29538 10.8788 1 9 1C6.79086 1 5 2.79086 5 5C5 5.35223 5.04553 5.69382 5.13102 6.01922C3.37146 6.20358 2 7.69163 2 9.5ZM11 13H9V8C9 7.44772 9.44772 7 10 7C10.5523 7 11 7.44772 11 8V13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloudUpload;
impl IconShape for HiCloudUpload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 13C3.567 13 2 11.433 2 9.5C2 7.69163 3.37146 6.20358 5.13102 6.01922C5.04553 5.69382 5 5.35223 5 5C5 2.79086 6.79086 1 9 1C10.8788 1 12.4551 2.29538 12.8845 4.04175C13.0857 4.01422 13.2912 4 13.5 4C15.9853 4 18 6.01472 18 8.5C18 10.9853 15.9853 13 13.5 13H11V9.41421L12.2929 10.7071C12.6834 11.0976 13.3166 11.0976 13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071C6.68342 11.0976 7.31658 11.0976 7.70711 10.7071L9 9.41421L9 13H5.5Z",
            }
            path {
                d: "M9 13H11L11 18C11 18.5523 10.5523 19 10 19C9.44772 19 9 18.5523 9 18L9 13Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/2Kv8jpv81qq9dIf/tqP/TX//RD/iV77rd/7+Gp//61svzX2W/O8bpX020R8zx//+o//Nv/1EP8JXvut3/v4am/vo2x/NHCcF82tKuV9/vjXf/y3+a+D+A/2qm/wDi/dpum7gJfm30L66j/5zZ/8GP5rIP4DveobvMNLt2n6LeA4/x7STy92dt7nt3/6u3f5z4X4D/LKb/zOD/Z6/VfAcf4jSD/9J7/5k2/Dfy7Ef5BXep23+SvgpfkPFNLH/NFv/uRX858H8R/glV/3bd/b9nfxH293cezYQ377p797l/8ciP8Ar/Q6b/N04MH8J5D0OX/8mz/52fznQPw7vcrrvd1bZ+ZP8Z/n1j/5rZ96CP85EP9Or/S6b/PVmI/iP5P01YJd7ifdGhF/84e/9mN/zb8P4t/plV/nbX7b8Fr897gV6afV91/zx7/8w7fyr4f4d3ql13mbi8Bx/ptJ+uz5zs7X/PZPf/cuLzrEv9Mrvc7bmP85bi21vs0f/tqP/TUvGsS/0yu9ztuY/1l2Vcrb/PGv//hv8y9D/Du90uu8zV8DL8X/LLul1tf5w1/7sb/mhUP8O73y67zNbxtei/95/npx7Njr/PZPf/cuLxjiX+m13/q9j6/299+KzPc2vDb/g0n6nD/+zZ/8bF4wxL/Cq7ze2711Zn4XcJz/JTSbPeSPf/mHb+X5Q7yIXvl13/a7bL83/9uIr/mT3/ypj+b5Q7wIXvl13/a7bL83/zvd+ie/9VMP4flD/Ate+XXf9rNtfxb/i5VaX+YPf+3H/prnhXghXvmN3/nBXq+fzv9yEfE2f/QbP/HTPC/EC/HKr/M23214L/6Xk/Q5f/ybP/nZPC/EC/FKr/M25v8ASZ/zx7/5k5/N80K8AK/yem/31pn5U/wfIOlz/vg3f/KzeV6IF+CVX/dtP9v2Z/F/gKTP+ePf/MnP5nkhXoBXft23/Wzbn8X/ASrldf7413/8t3leiBfglV/3bT/b9mfxf8Di2LETv/3T373L80K8AK/8um/72bY/i//lBL/zx7/1U6/N84d4AV7l9d7urTPzp/hfTtL7/PFv/uR38/whXohXep232QWO8b+U4Bl//Fs/9WBeMMQL8cqv8zbfbXgv/peKiLf5o9/4iZ/mBUO8EK/8xu/8YK/XT+d/IcH3/PFv/dR788Ih/gWv/Lpv+9m2P4v/Xf7mT37rp16afxniRfDKr/M23214L/53+JnFsWPv/ds//d27/MsQL6JXfp23+W7De/E/mfiaP/nNn/poXnSIf4VXeb23e2tnfrXhQfzP8jOazT76j3/5h2/lXwfxb/DKr/u27237rYG34r/P34T03e77n/7jX/7hW/m3Qfw7vfZbv/fx1cHBS/Nf5I9//cd/m/84iP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AjTeNlCPCfYFAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloud;
impl IconShape for HiCloud {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 16C3.567 16 2 14.433 2 12.5C2 10.6916 3.37146 9.20358 5.13102 9.01922C5.04553 8.69382 5 8.35223 5 8C5 5.79086 6.79086 4 9 4C10.8788 4 12.4551 5.29538 12.8845 7.04175C13.0857 7.01422 13.2912 7 13.5 7C15.9853 7 18 9.01472 18 11.5C18 13.9853 15.9853 16 13.5 16H5.5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCode;
impl IconShape for HiCode {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.3162 3.05134C12.8402 3.22599 13.1233 3.79231 12.9487 4.31625L8.94868 16.3163C8.77404 16.8402 8.20772 17.1234 7.68377 16.9487C7.15983 16.7741 6.87667 16.2077 7.05132 15.6838L11.0513 3.6838C11.226 3.15986 11.7923 2.8767 12.3162 3.05134ZM5.70711 6.29292C6.09763 6.68344 6.09763 7.31661 5.70711 7.70713L3.41421 10L5.70711 12.2929C6.09763 12.6834 6.09763 13.3166 5.70711 13.7071C5.31658 14.0977 4.68342 14.0977 4.29289 13.7071L1.29289 10.7071C0.902369 10.3166 0.902369 9.68344 1.29289 9.29292L4.29289 6.29292C4.68342 5.9024 5.31658 5.9024 5.70711 6.29292ZM14.2929 6.29292C14.6834 5.9024 15.3166 5.9024 15.7071 6.29292L18.7071 9.29292C19.0976 9.68344 19.0976 10.3166 18.7071 10.7071L15.7071 13.7071C15.3166 14.0977 14.6834 14.0977 14.2929 13.7071C13.9024 13.3166 13.9024 12.6834 14.2929 12.2929L16.5858 10L14.2929 7.70713C13.9024 7.31661 13.9024 6.68344 14.2929 6.29292Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCog;
impl IconShape for HiCog {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.4892 3.17094C11.1102 1.60969 8.8898 1.60969 8.51078 3.17094C8.26594 4.17949 7.11045 4.65811 6.22416 4.11809C4.85218 3.28212 3.28212 4.85218 4.11809 6.22416C4.65811 7.11045 4.17949 8.26593 3.17094 8.51078C1.60969 8.8898 1.60969 11.1102 3.17094 11.4892C4.17949 11.7341 4.65811 12.8896 4.11809 13.7758C3.28212 15.1478 4.85218 16.7179 6.22417 15.8819C7.11045 15.3419 8.26594 15.8205 8.51078 16.8291C8.8898 18.3903 11.1102 18.3903 11.4892 16.8291C11.7341 15.8205 12.8896 15.3419 13.7758 15.8819C15.1478 16.7179 16.7179 15.1478 15.8819 13.7758C15.3419 12.8896 15.8205 11.7341 16.8291 11.4892C18.3903 11.1102 18.3903 8.8898 16.8291 8.51078C15.8205 8.26593 15.3419 7.11045 15.8819 6.22416C16.7179 4.85218 15.1478 3.28212 13.7758 4.11809C12.8896 4.65811 11.7341 4.17949 11.4892 3.17094ZM10 13C11.6569 13 13 11.6569 13 10C13 8.34315 11.6569 7 10 7C8.34315 7 7 8.34315 7 10C7 11.6569 8.34315 13 10 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xn+At3uFDXsvopVN5nP8A4dgV/uuf+7Fv+h3+YyH+g73ZO3zwbxlem/8M0k//4o9+09vwHwfxH+jN3ulDPtrpr+I/kUIf8ws/8k1fzX8MxH+gN32HD/5t4LX4z/U7v/hj3/za/MdA/Ad603f44J8G3or/RBLf8ws/+s3vzX8MxH+gN3/nD37tbPwW/4mi8Do//8Pf/Nv8x0D8B3uLd/yQt0782TYvxX8gib8J9Nk/96Pf9NP8x0H8/4b4/w3x/xvi3+jN3vFD3gv7vflvZmlX8NO/8KPf9D386yH+Dd70HT/4szGfxf8k4nN+8Ue/+bP510H8G7zpO3yw+R/oF3/sm8W/DuJf6U3f4UNfGvKv+B8pXuYXf+wb/5oXHeLf4E3f4YN3gWP8z3LpF3/sm4/zr4P4N3jTd/jg9wa+i/9BivQ2P/ej3/TT/Osg/o3e+p0/+MFD8t78D9AH3/3TP/zNt/Kvh/j/DfH/G+Jf6S3e4UNei//Bfu7Hvul3eNEhXgRv/s4f/NpufJbhtflfQPDbKnzOz//wN/82LxziX/Cm7/jBn4X5bP43Ep/9iz/6zZ/DC4Z4Id70HT74vYHv4n+39/nFH/vm7+b5Q7wAb/3eH318PFw/3fg4/4sJ7Xabs4f89Hd/9S7PC/ECvOk7fPB7A9/F/w3v84s/9s3fzfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LP4vEJ/ziz/6zZ/N80K8AG/6jh/82ZjP4v8C8Tm/+KPf/Nk8L8QL8Kbv+MGfjfks/i8Qn/OLP/rNn83zQrwAb/qOH/zZmM/i/wLxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+LxCf84s/+s2fzfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LP4vEJ/ziz/6zZ/N80K8AG/6jh/82ZjP4v8C8Tm/+KPf/Nk8L8QL8Kbv+MGfjfks/i8Qn/OLP/rNn83zQrwAb/qOH/zZmM/i/wLxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+LxCf84s/+s2fzfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LP4vEJ/ziz/6zZ/N80K8AG/6jh/82ZjP4v8C8Tm/+KPf/Nk8L8QL8Kbv8MHvDXwX/ze8zy/+2Dd/N88L8QK89Tt/8IOHxtP5P6AvPOSnf/ibb+V5IV6IN33HD/5szGfxv5n4nF/80W/+bJ4/xL/gzd7xg7/b5r34X0jie37hR7/5vXnBEC+CN33HD/5szEcDx/jf4RLiq3/xR7/5s3nhEP8Kb/7OH/za2Xgw4sH8T2RujcKtP//D3/zbvGgQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/Q2AlQX/QCQwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCollection;
impl IconShape for HiCollection {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 3C6.44772 3 6 3.44772 6 4C6 4.55228 6.44772 5 7 5H13C13.5523 5 14 4.55228 14 4C14 3.44772 13.5523 3 13 3H7Z",
            }
            path {
                d: "M4 7C4 6.44772 4.44772 6 5 6H15C15.5523 6 16 6.44772 16 7C16 7.55228 15.5523 8 15 8H5C4.44772 8 4 7.55228 4 7Z",
            }
            path {
                d: "M2 11C2 9.89543 2.89543 9 4 9H16C17.1046 9 18 9.89543 18 11V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V11Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiColorSwatch;
impl IconShape for HiColorSwatch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2C2.89543 2 2 2.89543 2 4V15C2 16.6569 3.34315 18 5 18C6.65685 18 8 16.6569 8 15V4C8 2.89543 7.10457 2 6 2H4ZM5 16C5.55228 16 6 15.5523 6 15C6 14.4477 5.55228 14 5 14C4.44772 14 4 14.4477 4 15C4 15.5523 4.44772 16 5 16ZM10 14.2426L14.8995 9.34308C15.6805 8.56203 15.6805 7.2957 14.8995 6.51465L13.4853 5.10044C12.7042 4.31939 11.4379 4.31939 10.6568 5.10044L10 5.75728V14.2426ZM16 18H9.07104L15.071 12H16C17.1046 12 18 12.8954 18 14V16C18 17.1046 17.1046 18 16 18Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3eIcPeS3+B/u5H/um3+FFh3gRvPk7f/Bru/FZhtfmfwHBb6vwOT//w9/827xwiH/Bm77jB38W5rP530h89i/+6Dd/Di8Y4oV403f44PcGvov/3d7nF3/sm7+b5w/xArz1e3/08fFw/XTj4/wvJrTbbc4e8tPf/dW7PC/EC/Cm7/DB7w18F/83vM8v/tg3fzfPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArz5O3/wa2fy2vwfEMFv//wPf/Nv87wQ/78h/n9D/P+G+P8N8f8b4gV463f+4Ae3pgfxf0ApfsZP//A338rzQrwAb/qOH/zZmM/i/wLxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+LxCf84s/+s2fzfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LP4vEJ/ziz/6zZ/N80K8AG/6jh/82ZjP4v8C8Tm/+KPf/Nk8L8QL8Kbv+MGfjfks/i8Qn/OLP/rNn83zQrwAb/qOH/zZmM/i/wLxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+LxCf84s/+s2fzfNCvABv+o4f/NmYz+L/AvE5v/ij3/zZPC/EC/Cm7/jBn435LF4Aib8x/DTPzbw28Fo8f7+D+G2ei+CtbV6K/yzic37xR7/5s3leiBfgTd/xgz8b81k8HxLf8ws/+s3vzQvwZu/0IR/t9FfxAAp9zC/8yDd9NS/Am77DB/808Fb8ZxCf84s/+s2fzfNCvABv+o4f/NmYz+L56DfnJ376u796lxfizd7xg//a5qUAJP7mF370m1+aF+Kt3/ujjw+Hq4v8ZxCf84s/+s2fzfNCvABv+o4f/NmYz+K5SPzNL/zoN780/4I3fYcP/mngrbjiZ37xx775rfkXvNk7fvBf27wU/9HE5/zij37zZ/O8EC/Am77jB3825rN4Pn7xx75Z/Ave9B0+5K/AL80Vv/OLP/bNr82/4E3f4YPNfwbxOb/4o9/82TwvxAvwpu/4wZ+N+SyeH/E5v/ij3/zZvABv8Y4f8tbN/ikeIAqv8/M//M2/zQvwpu/4wZ+N+Sz+M4jP+cUf/ebP5nkhXoA3fccP/mzMZ/GCSD+N4rtL5i4PkOKtbX80z4ekrw7z0zxAiziO872x35r/LOJzfvFHv/mzeV6IF+BN3/GDPxvzWfxfID7nF3/0mz+b54V4Ad70HT74vYHv4v+AIr3Nz/3oN/00zwvxArz1e3/08eFwdStwjP/dLvWb8wf/9Hd/9S7PC/FCvOk7fPB7A9/F/27v84s/9s3fzfOH+Be86Tt+8GdjPov/jcTn/OKPfvNn84IhXgRv/s4f/NrZ+Gjgrfjf4Wei8NU//8Pf/Nu8cIh/pTd/5w9+bf4H+/kf/ubf5kWH+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjZZ8zUJPOhVIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCreditCard;
impl IconShape for HiCreditCard {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V7H18V6C18 4.89543 17.1046 4 16 4H4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M18 9H2V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V9ZM4 13C4 12.4477 4.44772 12 5 12H6C6.55228 12 7 12.4477 7 13C7 13.5523 6.55228 14 6 14H5C4.44772 14 4 13.5523 4 13ZM9 12C8.44772 12 8 12.4477 8 13C8 13.5523 8.44772 14 9 14H10C10.5523 14 11 13.5523 11 13C11 12.4477 10.5523 12 10 12H9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCubeTransparent;
impl IconShape for HiCubeTransparent {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.50386 1.13176C9.81129 0.956081 10.1887 0.956081 10.4961 1.13176L12.2461 2.13176C12.7257 2.40577 12.8923 3.01662 12.6182 3.49614C12.3442 3.97566 11.7334 4.14225 11.2539 3.86824L10 3.15175L8.74614 3.86824C8.26662 4.14225 7.65577 3.97566 7.38176 3.49614C7.10775 3.01662 7.27434 2.40577 7.75386 2.13176L9.50386 1.13176ZM5.61824 4.50386C5.89225 4.98338 5.72566 5.59423 5.24614 5.86824L5.01556 6L5.24614 6.13176C5.72566 6.40577 5.89225 7.01662 5.61824 7.49614C5.34423 7.97566 4.73338 8.14225 4.25386 7.86824L4 7.72318V8C4 8.55228 3.55228 9 3 9C2.44772 9 2 8.55228 2 8V6C2 5.75001 2.09173 5.52145 2.24336 5.34614C2.27802 5.30603 2.31598 5.26854 2.35699 5.23411C2.40754 5.19163 2.46236 5.15405 2.52071 5.12213L4.25386 4.13176C4.73338 3.85775 5.34423 4.02434 5.61824 4.50386ZM14.3818 4.50386C14.6558 4.02434 15.2666 3.85775 15.7461 4.13176L17.4793 5.12212C17.5376 5.15405 17.5925 5.19162 17.643 5.23411C17.8613 5.41755 18 5.69258 18 6V8C18 8.55228 17.5523 9 17 9C16.4477 9 16 8.55228 16 8V7.72318L15.7461 7.86824C15.2666 8.14225 14.6558 7.97566 14.3818 7.49614C14.1077 7.01662 14.2743 6.40577 14.7539 6.13176L14.9844 6L14.7539 5.86824C14.2743 5.59423 14.1077 4.98338 14.3818 4.50386ZM7.38176 8.50386C7.65577 8.02434 8.26662 7.85775 8.74614 8.13176L10 8.84825L11.2539 8.13176C11.7334 7.85775 12.3442 8.02434 12.6182 8.50386C12.8923 8.98338 12.7257 9.59423 12.2461 9.86824L11 10.5803V12C11 12.5523 10.5523 13 10 13C9.44772 13 9 12.5523 9 12V10.5803L7.75386 9.86824C7.27434 9.59423 7.10775 8.98338 7.38176 8.50386ZM3 11C3.55228 11 4 11.4477 4 12V13.4197L5.24614 14.1318C5.72566 14.4058 5.89225 15.0166 5.61824 15.4961C5.34423 15.9757 4.73338 16.1423 4.25386 15.8682L2.50386 14.8682C2.19229 14.6902 2 14.3589 2 14V12C2 11.4477 2.44772 11 3 11ZM17 11C17.5523 11 18 11.4477 18 12V14C18 14.3589 17.8077 14.6902 17.4961 14.8682L15.7461 15.8682C15.2666 16.1423 14.6558 15.9757 14.3818 15.4961C14.1077 15.0166 14.2743 14.4058 14.7539 14.1318L16 13.4197V12C16 11.4477 16.4477 11 17 11ZM7.38176 16.5039C7.65577 16.0243 8.26662 15.8577 8.74614 16.1318L9 16.2768V16C9 15.4477 9.44772 15 10 15C10.5523 15 11 15.4477 11 16V16.2768L11.2539 16.1318C11.7334 15.8577 12.3442 16.0243 12.6182 16.5039C12.8923 16.9834 12.7257 17.5942 12.2461 17.8682L10.5113 18.8596C10.3617 18.9488 10.1868 19 10 19C9.81316 19 9.63828 18.9488 9.48866 18.8596L7.75386 17.8682C7.27434 17.5942 7.10775 16.9834 7.38176 16.5039Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCube;
impl IconShape for HiCube {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 17C11 17.3466 11.1795 17.6684 11.4743 17.8507C11.7691 18.0329 12.1372 18.0494 12.4472 17.8944L16.4472 15.8944C16.786 15.725 17 15.3788 17 15V9.23607C17 8.88949 16.8205 8.56762 16.5257 8.38542C16.2309 8.20321 15.8628 8.18665 15.5528 8.34164L11.5528 10.3416C11.214 10.511 11 10.8573 11 11.2361V17Z",
            }
            path {
                d: "M15.2111 6.27639C15.5499 6.107 15.7639 5.76074 15.7639 5.38197C15.7639 5.00319 15.5499 4.65693 15.2111 4.48754L10.4472 2.10557C10.1657 1.96481 9.83431 1.96481 9.55279 2.10557L4.78885 4.48754C4.45007 4.65693 4.23607 5.00319 4.23607 5.38197C4.23607 5.76074 4.45007 6.107 4.78885 6.27639L9.55279 8.65836C9.83431 8.79912 10.1657 8.79912 10.4472 8.65836L15.2111 6.27639Z",
            }
            path {
                d: "M4.44721 8.34164C4.13723 8.18665 3.76909 8.20321 3.47427 8.38542C3.17945 8.56762 3 8.88949 3 9.23607V15C3 15.3788 3.214 15.725 3.55279 15.8944L7.55279 17.8944C7.86277 18.0494 8.23091 18.0329 8.52573 17.8507C8.82055 17.6684 9 17.3466 9 17V11.2361C9 10.8573 8.786 10.511 8.44721 10.3416L4.44721 8.34164Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyBangladeshi;
impl IconShape for HiCurrencyBangladeshi {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 4C6.44772 4 6 4.44772 6 5C6 5.55228 6.44772 6 7 6C7.55228 6 8 6.44772 8 7V8H7C6.44772 8 6 8.44772 6 9C6 9.55228 6.44772 10 7 10H8V13C8 14.6569 9.34315 16 11 16C12.6569 16 14 14.6569 14 13V12C14 11.4477 13.5523 11 13 11C12.4477 11 12 11.4477 12 12V13C12 13.5523 11.5523 14 11 14C10.4477 14 10 13.5523 10 13V10H13C13.5523 10 14 9.55228 14 9C14 8.44772 13.5523 8 13 8H10V7C10 5.34315 8.65685 4 7 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyDollar;
impl IconShape for HiCurrencyDollar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.43338 7.41784C8.58818 7.31464 8.77939 7.2224 9 7.15101L9.00001 8.84899C8.77939 8.7776 8.58818 8.68536 8.43338 8.58216C8.06927 8.33942 8 8.1139 8 8C8 7.8861 8.06927 7.66058 8.43338 7.41784Z",
            }
            path {
                d: "M11 12.849L11 11.151C11.2206 11.2224 11.4118 11.3146 11.5666 11.4178C11.9308 11.6606 12 11.8861 12 12C12 12.1139 11.9308 12.3394 11.5666 12.5822C11.4118 12.6854 11.2206 12.7776 11 12.849Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 5C11 4.44772 10.5523 4 10 4C9.44772 4 9 4.44772 9 5V5.09199C8.3784 5.20873 7.80348 5.43407 7.32398 5.75374C6.6023 6.23485 6 7.00933 6 8C6 8.99067 6.6023 9.76515 7.32398 10.2463C7.80348 10.5659 8.37841 10.7913 9.00001 10.908L9.00002 12.8492C8.60902 12.7223 8.31917 12.5319 8.15667 12.3446C7.79471 11.9275 7.16313 11.8827 6.74599 12.2447C6.32885 12.6067 6.28411 13.2382 6.64607 13.6554C7.20855 14.3036 8.05956 14.7308 9 14.9076L9 15C8.99999 15.5523 9.44769 16 9.99998 16C10.5523 16 11 15.5523 11 15L11 14.908C11.6216 14.7913 12.1965 14.5659 12.676 14.2463C13.3977 13.7651 14 12.9907 14 12C14 11.0093 13.3977 10.2348 12.676 9.75373C12.1965 9.43407 11.6216 9.20873 11 9.09199L11 7.15075C11.391 7.27771 11.6808 7.4681 11.8434 7.65538C12.2053 8.07252 12.8369 8.11726 13.254 7.7553C13.6712 7.39335 13.7159 6.76176 13.354 6.34462C12.7915 5.69637 11.9405 5.26915 11 5.09236V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyEuro;
impl IconShape for HiCurrencyEuro {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8.73617 6.97896C9.20793 6.1927 9.69618 6 10 6C10.3038 6 10.7921 6.1927 11.2638 6.97896C11.548 7.45254 12.1622 7.60611 12.6358 7.32196C13.1094 7.03781 13.263 6.42355 12.9788 5.94997C12.279 4.78361 11.2317 4 10 4C8.76829 4 7.721 4.78361 7.02119 5.94997C6.73632 6.42475 6.51422 6.94939 6.35097 7.5H6C5.44772 7.5 5 7.94772 5 8.5C5 9.05228 5.44772 9.5 6 9.5H6.01337C6.00443 9.66702 6 9.83388 6 10C6 10.1661 6.00443 10.333 6.01337 10.5H6C5.44772 10.5 5 10.9477 5 11.5C5 12.0523 5.44772 12.5 6 12.5H6.35097C6.51422 13.0506 6.73632 13.5753 7.02119 14.05C7.721 15.2164 8.76829 16 10 16C11.2317 16 12.279 15.2164 12.9788 14.05C13.263 13.5764 13.1094 12.9622 12.6358 12.678C12.1622 12.3939 11.548 12.5475 11.2638 13.021C10.7921 13.8073 10.3038 14 10 14C9.69618 14 9.20793 13.8073 8.73617 13.021C8.63927 12.8595 8.5511 12.6851 8.47216 12.5H10C10.5523 12.5 11 12.0523 11 11.5C11 10.9477 10.5523 10.5 10 10.5H8.01695C8.00571 10.335 8 10.1681 8 10C8 9.83191 8.00571 9.66498 8.01695 9.5H10C10.5523 9.5 11 9.05228 11 8.5C11 7.94772 10.5523 7.5 10 7.5H8.47216C8.5511 7.31488 8.63927 7.14047 8.73617 6.97896Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyPound;
impl IconShape for HiCurrencyPound {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 4C9.34315 4 8 5.34315 8 7V9H7C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H8V12C8 12.5523 7.55228 13 7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H9.82929C9.93985 12.6872 10 12.3506 10 12V11H11C11.5523 11 12 10.5523 12 10C12 9.44772 11.5523 9 11 9H10V7C10 6.44772 10.4477 6 11 6C11.5523 6 12 6.44772 12 7C12 7.55228 12.4477 8 13 8C13.5523 8 14 7.55228 14 7C14 5.34315 12.6569 4 11 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyRupee;
impl IconShape for HiCurrencyRupee {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7.00003 5C6.44774 5 6.00003 5.44772 6.00003 6C6.00003 6.55228 6.44774 7 7.00003 7H8.00003C8.74031 7 9.38666 7.4022 9.73246 8H7.00003C6.44774 8 6.00003 8.44772 6.00003 9C6.00003 9.55228 6.44774 10 7.00003 10H9.73246C9.38665 10.5978 8.74031 11 8.00003 11H7.00003C6.59557 11 6.23093 11.2436 6.07615 11.6173C5.92137 11.991 6.00692 12.4211 6.29292 12.7071L9.29292 15.7071C9.68345 16.0976 10.3166 16.0976 10.7071 15.7071C11.0977 15.3166 11.0977 14.6834 10.7071 14.2929L9.22363 12.8094C10.5222 12.3926 11.5316 11.3302 11.874 10H13C13.5523 10 14 9.55228 14 9C14 8.44772 13.5523 8 13 8H11.874C11.7827 7.64523 11.6439 7.30951 11.4649 7H13C13.5523 7 14 6.55228 14 6C14 5.44772 13.5523 5 13 5H7.00003Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyYen;
impl IconShape for HiCurrencyYen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7.85752 5.48541C7.57337 5.01183 6.95911 4.85827 6.48553 5.14241C6.01195 5.42656 5.85839 6.04082 6.14254 6.5144L7.63384 8.99991H7.00003C6.44774 8.99991 6.00003 9.44762 6.00003 9.99991C6.00003 10.5522 6.44774 10.9999 7.00003 10.9999H8.83384L9.00003 11.2769V11.9999H7.00003C6.44774 11.9999 6.00003 12.4476 6.00003 12.9999C6.00003 13.5522 6.44774 13.9999 7.00003 13.9999H9.00003V14.9999C9.00003 15.5522 9.44774 15.9999 10 15.9999C10.5523 15.9999 11 15.5522 11 14.9999V13.9999H13C13.5523 13.9999 14 13.5522 14 12.9999C14 12.4476 13.5523 11.9999 13 11.9999H11V11.2769L11.1662 10.9999H13C13.5523 10.9999 14 10.5522 14 9.99991C14 9.44762 13.5523 8.99991 13 8.99991H12.3662L13.8575 6.5144C14.1417 6.04082 13.9881 5.42656 13.5145 5.14241C13.0409 4.85827 12.4267 5.01183 12.1425 5.48541L10.0338 8.99991H9.96622L7.85752 5.48541Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCursorClick;
impl IconShape for HiCursorClick {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.67187 1.91147C6.52893 1.37801 5.9806 1.06142 5.44713 1.20437C4.91366 1.34731 4.59708 1.89565 4.74002 2.42911L4.99884 3.39504C5.14178 3.9285 5.69012 4.24509 6.22359 4.10214C6.75705 3.9592 7.07363 3.41086 6.93069 2.8774L6.67187 1.91147ZM2.42923 4.7399C1.89577 4.59696 1.34743 4.91354 1.20449 5.44701C1.06155 5.98047 1.37813 6.52881 1.9116 6.67175L2.87752 6.93057C3.41099 7.07351 3.95932 6.75693 4.10227 6.22346C4.24521 5.69 3.92863 5.14166 3.39516 4.99872L2.42923 4.7399ZM11.2427 4.17149C11.6332 3.78097 11.6332 3.1478 11.2427 2.75728C10.8522 2.36676 10.219 2.36676 9.82847 2.75728L9.12136 3.46439C8.73084 3.85491 8.73084 4.48808 9.12136 4.8786C9.51189 5.26912 10.1451 5.26912 10.5356 4.8786L11.2427 4.17149ZM4.17162 11.2426L4.87872 10.5355C5.26925 10.1449 5.26925 9.51177 4.87872 9.12124C4.4882 8.73072 3.85503 8.73072 3.46451 9.12124L2.7574 9.82835C2.36688 10.2189 2.36688 10.852 2.7574 11.2426C3.14793 11.6331 3.78109 11.6331 4.17162 11.2426ZM7.37154 6.07152C7.00012 5.92295 6.5759 6.01002 6.29304 6.29289C6.01018 6.57575 5.92311 6.99997 6.07167 7.37138L10.0717 17.3714C10.2179 17.737 10.5651 17.9828 10.9586 17.9991C11.352 18.0155 11.7185 17.7994 11.8946 17.4472L13.2741 14.6882L16.293 17.7071C16.6836 18.0976 17.3167 18.0976 17.7073 17.7071C18.0978 17.3166 18.0978 16.6834 17.7073 16.2929L14.6883 13.2739L17.4474 11.8944C17.7996 11.7183 18.0157 11.3519 17.9993 10.9584C17.9829 10.565 17.7372 10.2178 17.3715 10.0715L7.37154 6.07152Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDatabase;
impl IconShape for HiDatabase {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 12V15C3 16.6569 6.13401 18 10 18C13.866 18 17 16.6569 17 15V12C17 13.6569 13.866 15 10 15C6.13401 15 3 13.6569 3 12Z",
            }
            path {
                d: "M3 7V10C3 11.6569 6.13401 13 10 13C13.866 13 17 11.6569 17 10V7C17 8.65685 13.866 10 10 10C6.13401 10 3 8.65685 3 7Z",
            }
            path {
                d: "M17 5C17 6.65685 13.866 8 10 8C6.13401 8 3 6.65685 3 5C3 3.34315 6.13401 2 10 2C13.866 2 17 3.34315 17 5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDesktopComputer;
impl IconShape for HiDesktopComputer {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 3.89543 3.89543 3 5 3H15C16.1046 3 17 3.89543 17 5V13C17 14.1046 16.1046 15 15 15H12.7808L12.903 15.4887L13.7071 16.2929C13.9931 16.5789 14.0787 17.009 13.9239 17.3827C13.7691 17.7563 13.4045 18 13 18H7.00003C6.59557 18 6.23093 17.7563 6.07615 17.3827C5.92137 17.009 6.00692 16.5789 6.29292 16.2929L7.09706 15.4887L7.21925 15H5C3.89543 15 3 14.1046 3 13V5ZM8.7713 12C8.75657 11.9997 8.74189 11.9997 8.72725 12H5V5H15V12H11.2728C11.2582 11.9997 11.2435 11.9997 11.2288 12H8.7713Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFQ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+g73S67/NWyn10vwncPiv/+TXf+pn+I+D+A/yyq//9q/t1r4LeDD/uW5VKe/zx7/+47/Nvx/iP8Arv+7bvrft7+K/UES8zR/9xk/8NP8+iH+nV37jd36w1+u/Ao7zX2t3cezYQ377p797l387xL/TK73u23w15qP47yC+5k9+86c+mn87xL/TK73O2zwdeDD/PW79k9/6qYfwb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb89/oT37rp8S/HeLf6ZVe523Mf6M/+a2fEv92iH+nV3qdtzH/jf7kt35K/Nsh/p1e6XXexvw3+pPf+inxb4f4d3ql13kb828keAaA4UH8G/3Jb/2U+LdD/Du90uu8jfnXu6RS3vqPf/3HfxvglV//7V/brf00cIx/pT/5rZ8S/3aIf6dXep23Mf9KKuV1/vjXf/y3eYBXfv23f2239lv8K/3Jb/2U+LdD/Du90uu8jflXEDzjj3/rpx7M8/FKr/M2u8Ax/hX+5Ld+SvzbIf6dXul13sb869z6J7/1Uw/h+Xil13mbi8Bx/hX+5Ld+SvzbIf6dXul13sb8K6mU1/njX//x3+YBXvn13/613dpv8a/0J7/1U+LfDvHv9Eqv8zbmX29XpbzNH//6j/82wCu//tu/tlv7KeA4/0p/8ls/Jf7tEP9Or/Q6b2P+7Xa54jj/Rn/yWz8l/u0Q/06v/Dpvc6vhQfw3EDzjj3/rpx7Mvx3i3+mVXvdtvhrzUfx3EF/zJ7/5Ux/Nvx3i3+mV3/idH+z1+q+BY/zXurQ4duzBv/3T373Lvx3iP8CrvN7bvXVm/hT/hSLibf7oN37ip/n3QfwHeeXXf/vXprXvNjyI/0SCZ1DKe//xr//4b/Pvh/gP9iqv93Zvbful+U8g6a//6Dd+4qf5j4P4/w3x/xvi/zfE/2+I/98Q/7/xj5ZEKlBt9Z2yAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDeviceMobile;
impl IconShape for HiDeviceMobile {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 2C5.89543 2 5 2.89543 5 4V16C5 17.1046 5.89543 18 7 18H13C14.1046 18 15 17.1046 15 16V4C15 2.89543 14.1046 2 13 2H7ZM10 16C10.5523 16 11 15.5523 11 15C11 14.4477 10.5523 14 10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDeviceTablet;
impl IconShape for HiDeviceTablet {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V4C16 2.89543 15.1046 2 14 2H6ZM10 16C10.5523 16 11 15.5523 11 15C11 14.4477 10.5523 14 10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentAdd;
impl IconShape for HiDocumentAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V10H7C6.44772 10 6 10.4477 6 11C6 11.5523 6.44772 12 7 12H9V14C9 14.5523 9.44771 15 10 15C10.5523 15 11 14.5523 11 14L11 12H13C13.5523 12 14 11.5523 14 11C14 10.4477 13.5523 10 13 10H11V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentDownload;
impl IconShape for HiDocumentDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V11.5858L7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929C5.90237 10.6834 5.90237 11.3166 6.29289 11.7071L9.29289 14.7071C9.68342 15.0976 10.3166 15.0976 10.7071 14.7071L13.7071 11.7071C14.0976 11.3166 14.0976 10.6834 13.7071 10.2929C13.3166 9.90237 12.6834 9.90237 12.2929 10.2929L11 11.5858V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentDuplicate;
impl IconShape for HiDocumentDuplicate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C7.89543 2 7 2.89543 7 4V12C7 13.1046 7.89543 14 9 14H15C16.1046 14 17 13.1046 17 12V6.41421C17 5.88378 16.7893 5.37507 16.4142 5L14 2.58579C13.6249 2.21071 13.1162 2 12.5858 2H9Z",
            }
            path {
                d: "M3 8C3 6.89543 3.89543 6 5 6V16H13C13 17.1046 12.1046 18 11 18H5C3.89543 18 3 17.1046 3 16V8Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentRemove;
impl IconShape for HiDocumentRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM7 10C6.44772 10 6 10.4477 6 11C6 11.5523 6.44772 12 7 12H13C13.5523 12 14 11.5523 14 11C14 10.4477 13.5523 10 13 10H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentReport;
impl IconShape for HiDocumentReport {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM8 12C8 11.4477 7.55228 11 7 11C6.44772 11 6 11.4477 6 12V15C6 15.5523 6.44772 16 7 16C7.55228 16 8 15.5523 8 15V12ZM10 9C10.5523 9 11 9.44772 11 10V15C11 15.5523 10.5523 16 10 16C9.44772 16 9 15.5523 9 15V10C9 9.44772 9.44772 9 10 9ZM14 8C14 7.44772 13.5523 7 13 7C12.4477 7 12 7.44772 12 8V15C12 15.5523 12.4477 16 13 16C13.5523 16 14 15.5523 14 15V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentSearch;
impl IconShape for HiDocumentSearch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H12.4722C13.4223 16.9385 14 15.5367 14 14C14 10.6863 11.3137 8 8 8C6.46329 8 5.06151 8.57771 4 9.52779V4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 10C5.79086 10 4 11.7909 4 14C4 14.7414 4.20229 15.4364 4.55397 16.0318L3.29289 17.2929C2.90237 17.6834 2.90237 18.3166 3.29289 18.7071C3.68342 19.0976 4.31658 19.0976 4.70711 18.7071L5.96818 17.446C6.56362 17.7977 7.25862 18 8 18C10.2091 18 12 16.2091 12 14C12 11.7909 10.2091 10 8 10ZM6 14C6 12.8954 6.89543 12 8 12C9.10457 12 10 12.8954 10 14C10 15.1046 9.10457 16 8 16C7.44744 16 6.94881 15.7772 6.58579 15.4142C6.22276 15.0512 6 14.5526 6 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentText;
impl IconShape for HiDocumentText {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V4ZM6 10C6 9.44772 6.44772 9 7 9H13C13.5523 9 14 9.44772 14 10C14 10.5523 13.5523 11 13 11H7C6.44772 11 6 10.5523 6 10ZM7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocument;
impl IconShape for HiDocument {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsCircleHorizontal;
impl IconShape for HiDotsCircleHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9H5V11H7V9ZM15 9H13V11H15V9ZM9 9H11V11H9V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE40lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfGv8Bbv8CGvleLBYW4txc/46R/+5lv5N3rTd/jQlyb0IADSz/jFH/vGv+bf6K3f+YMf3JoelOLBYW79uR/7pt/hRYN4EbzpO37wZ8n6aOPjPIDgt018zC/+2Df+NS+it3jHD3nrhr8K82AeSNxa0Mf83I9+00/zInrTd/jQlxb5VYbX5gGEdi1/9S/+6Dd/Di8c4l/wpu/wwd8FvDcv3Pv84o9983fzL3jTd/zgz8Z8Fi+M+Jxf/NFv/mz+BW/6Dh/83sB38cJ99y/+2De/Dy8Y4oV403f84M/GfBb/AqHdrvhlfvqHv/lWXoC3eMcPeetm/xQvgiK9zc/96Df9NC/AW7/zBz94bPor4+P8S8Tn/OKPfvNn8/whXoC3fucPfvDQeDovIonv+YUf/eb35gV403f84KdjHsyLQtz6iz/6zQ/hBXjTd/jgnwbeihdRX3jIT//wN9/K80K8AG/6Dh/83sB38a/wiz/2zeL5eNN3+NCXhvwr/lXiZX7xx77xr3k+3vQdPtj867zPL/7YN383zwvxArzpO37wZ2M+i3+VeJlf/LFv/Guey1u844e8dbN/in+FIr3Nz/3oN/00z+VN3+FDXxryr/jXEJ/ziz/6zZ/N80K8AG/6jh/82ZjP4l8lXuYXf+wb/5rn8hbv+CFv3eyf4l+hSG/zcz/6TT/Nc3nTd/jQl4b8K/41xOf84o9+82fzvBAvwJu+wwe/N/Bd/Cv84o99s3g+3vQdPvSlIf+Kf5V4mV/8sW/8a56PN32HDzb/Ou/ziz/2zd/N80K8AG/9zh/84KHxdF5EEt/zCz/6ze/NC/Cm7/DBtwIP4kXzjF/8sW9+MC/Am77DB/808Fa8iPrCQ376h7/5Vp4X4oV403f84M/GfBb/skt94aV/+oe/+VZegLd4xw9562b/FC+CIr3Nz/3oN/00L8Bbv/MHP3ho/DVwjH+J+Jxf/NFv/myeP8S/4M3e8YO/2+a9eOHe5xd/7Ju/m3/Bm77jB3825rN4YcTn/OKPfvNn8y9403f44PcGvosXQuJ7fuFHv/m9ecEQL4I3fccP/mzMRwPHeE6/A/HRv/hj3/jXvIje4h0/5K2b/dXAg3hOzyjSR//cj37TT/MietN3+NCXhvxq4LV4TpcQX/2LP/rNn80Lh/hXePN3/uDXzsaDo3BrhVt/+oe/+Vb+jd70HT70pYv8YIBm3fqLP/aNf82/0Vu/8wc/eIIHZ+PBUbj153/4m3+bFw3i/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+ERLQH1BXFWMKAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsHorizontal;
impl IconShape for HiDotsHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 10C6 11.1046 5.10457 12 4 12C2.89543 12 2 11.1046 2 10C2 8.89543 2.89543 8 4 8C5.10457 8 6 8.89543 6 10Z",
            }
            path {
                d: "M12 10C12 11.1046 11.1046 12 10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10Z",
            }
            path {
                d: "M16 12C17.1046 12 18 11.1046 18 10C18 8.89543 17.1046 8 16 8C14.8954 8 14 8.89543 14 10C14 11.1046 14.8954 12 16 12Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE9ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+G7z1O3/wg8fUaxk/GEDo1i78Oz/9w998K/+1EP/F3uwdP+SrbH80z4/47F/80W/+HP7rIP4Lvek7fPB3Ae/NC/fdv/hj3/w+/NdA/Bd503f84M/GfBYvCvE5v/ij3/zZ/OdD/Bd5s3f4kIvGx3kRCO3+wo990wn+8yH+C7z5O3/wa2fjt/hXiMLr/PwPf/Nv858L8V/gTd/hg98b+C7+dd7nF3/sm7+b/1yI/wJv/s4f/NrZ+C3+FaLwOj//w9/82/znQvwXeOt3/uAHD42n86/QFx7y0z/8zbfynwvxX+RN3+GDfxt4LV40v/OLP/bNr81/PsR/kTd9hw99aci/4kUSL/OLP/aNf81/PsR/oTd9hw9+b+CrgWM8f5eAj/7FH/vm7+a/BuK/2Fu/8wc/eEw+2+a9eACJ7+mCz/7pH/7mW/mvg/hv9Kbv8KEvDfCLP/aNf81/D8T/b4j/3xD/vyH+f0P8/4b4b/Km7/ChL03oQQCkn/GLP/aNf81/PcR/sbd4xw9564a/CvNgHkjcWtDH/NyPftNP818H8V/oTd/xgz8b81m8MOJzfvFHv/mz+a+B+C/yFu/4IW/d7J/iRVCkt/m5H/2mn+Y/H+K/yJu+4wc/HfNgXhTi1l/80W9+CP/5EP8F3vQdPvSlIf+Kf5V4mV/8sW/8a/5zIf4LvMU7fshbN/un+Fco0tv83I9+00/znwvxX+At3vFD3rrZP8W/QpHe5ud+9Jt+mv9ciP8Cb/oOH/rSkH/Fv0q8zC/+2Df+Nf+5EP9F3vQdPvhW4EG8aJ7xiz/2zQ/mPx/iv8hbvOOHvHWzf4oXQZHe5ud+9Jt+mv98iP9Cb/qOH/zZmM/ihRGf84s/+s2fzX8NxH+xt3jHD3nrZn818CCe0zOK9NE/96Pf9NP810H8N3nTd/jQly7ygwGadesv/tg3/jX/9RD/vyH+f0P8/4b4/w3x/xviv9GbvsOHvjTAL/7YN/41/z0Q/8Xe+p0/+MFD6quw35oHkn66D3/MT//wN9/Kfx3Ef6E3fYcPfm+hrzI+zvMhtGv8Mb/4Y9/83fzXQPwXedN3+NCXhvwrXiTxMr/4Y9/41/znQ/wXebN3+ODfMrw2LwLBb//Cj33z6/CfD/Ff4K3f+YMfPDSezr9CX3jIT//wN9/Kfy7Ef4E3f+cPfu1s/Bb/ClF4nZ//4W/+bf5zIf4LvOk7fPB7A9/Fv877/OKPffN3858L8V/gzd/5g187G7/Fv0IUXufnf/ibf5v/XIj/Im/6Dh+8CxzjRXPpF3/sm4/znw/xX+RN3/GDPxvzWbwoxOf84o9+82fznw/xX+jN3vGDv9vmvXghJL7nF370m9+b/xqI/2Jv+o4f/NmYz+L5EZ/ziz/6zZ/Nfx3Ef4O3fucPfvDQeG3EgwEwt/aF3/7pH/7mW/mvhfj/DfH/G+L/N8T/b4j/3xD/v/GPj4AiUDuHRy0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsVertical;
impl IconShape for HiDotsVertical {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 6C8.89543 6 8 5.10457 8 4C8 2.89543 8.89543 2 10 2C11.1046 2 12 2.89543 12 4C12 5.10457 11.1046 6 10 6Z",
            }
            path {
                d: "M10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10C12 11.1046 11.1046 12 10 12Z",
            }
            path {
                d: "M10 18C8.89543 18 8 17.1046 8 16C8 14.8954 8.89543 14 10 14C11.1046 14 12 14.8954 12 16C12 17.1046 11.1046 18 10 18Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDownload;
impl IconShape for HiDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 17C3 16.4477 3.44772 16 4 16H16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H4C3.44772 18 3 17.5523 3 17ZM6.29289 9.29289C6.68342 8.90237 7.31658 8.90237 7.70711 9.29289L9 10.5858L9 3C9 2.44772 9.44771 2 10 2C10.5523 2 11 2.44771 11 3L11 10.5858L12.2929 9.29289C12.6834 8.90237 13.3166 8.90237 13.7071 9.29289C14.0976 9.68342 14.0976 10.3166 13.7071 10.7071L10.7071 13.7071C10.5196 13.8946 10.2652 14 10 14C9.73478 14 9.48043 13.8946 9.29289 13.7071L6.29289 10.7071C5.90237 10.3166 5.90237 9.68342 6.29289 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDuplicate;
impl IconShape for HiDuplicate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 9C7 7.89543 7.89543 7 9 7H15C16.1046 7 17 7.89543 17 9V15C17 16.1046 16.1046 17 15 17H9C7.89543 17 7 16.1046 7 15V9Z",
            }
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V11C3 12.1046 3.89543 13 5 13L5 5H13C13 3.89543 12.1046 3 11 3H5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEmojiHappy;
impl IconShape for HiEmojiHappy {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C7.55228 9 8 8.55228 8 8C8 7.44772 7.55228 7 7 7C6.44772 7 6 7.44772 6 8C6 8.55228 6.44772 9 7 9ZM14 8C14 8.55228 13.5523 9 13 9C12.4477 9 12 8.55228 12 8C12 7.44772 12.4477 7 13 7C13.5523 7 14 7.44772 14 8ZM13.5355 13.5354C13.9261 13.1449 13.9261 12.5118 13.5355 12.1212C13.145 11.7307 12.5118 11.7307 12.1213 12.1212C10.9497 13.2928 9.05025 13.2928 7.87868 12.1212C7.48816 11.7307 6.85499 11.7307 6.46447 12.1212C6.07394 12.5118 6.07394 13.1449 6.46447 13.5354C8.41709 15.4881 11.5829 15.4881 13.5355 13.5354Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEmojiSad;
impl IconShape for HiEmojiSad {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C7.55228 9 8 8.55228 8 8C8 7.44772 7.55228 7 7 7C6.44772 7 6 7.44772 6 8C6 8.55228 6.44772 9 7 9ZM14 8C14 8.55228 13.5523 9 13 9C12.4477 9 12 8.55228 12 8C12 7.44772 12.4477 7 13 7C13.5523 7 14 7.44772 14 8ZM6.46447 13.8785C6.85499 14.269 7.48816 14.269 7.87868 13.8785C9.05025 12.7069 10.9497 12.7069 12.1213 13.8785C12.5118 14.269 13.145 14.269 13.5355 13.8785C13.9261 13.4879 13.9261 12.8548 13.5355 12.4642C11.5829 10.5116 8.41709 10.5116 6.46447 12.4642C6.07394 12.8548 6.07394 13.4879 6.46447 13.8785Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExclamationCircle;
impl IconShape for HiExclamationCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM11 14C11 14.5523 10.5523 15 10 15C9.44772 15 9 14.5523 9 14C9 13.4477 9.44772 13 10 13C10.5523 13 11 13.4477 11 14ZM10 5C9.44772 5 9 5.44772 9 6V10C9 10.5523 9.44772 11 10 11C10.5523 11 11 10.5523 11 10V6C11 5.44772 10.5523 5 10 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExclamation;
impl IconShape for HiExclamation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.25706 3.09882C9.02167 1.73952 10.9788 1.73952 11.7434 3.09882L17.3237 13.0194C18.0736 14.3526 17.1102 15.9999 15.5805 15.9999H4.4199C2.89025 15.9999 1.92682 14.3526 2.67675 13.0194L8.25706 3.09882ZM11.0001 13C11.0001 13.5523 10.5524 14 10.0001 14C9.44784 14 9.00012 13.5523 9.00012 13C9.00012 12.4477 9.44784 12 10.0001 12C10.5524 12 11.0001 12.4477 11.0001 13ZM10.0001 5C9.44784 5 9.00012 5.44772 9.00012 6V9C9.00012 9.55228 9.44784 10 10.0001 10C10.5524 10 11.0001 9.55228 11.0001 9V6C11.0001 5.44772 10.5524 5 10.0001 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExternalLink;
impl IconShape for HiExternalLink {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 3C10.4477 3 10 3.44772 10 4C10 4.55228 10.4477 5 11 5H13.5858L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071C7.68342 13.0976 8.31658 13.0976 8.70711 12.7071L15 6.41421V9C15 9.55228 15.4477 10 16 10C16.5523 10 17 9.55228 17 9V4C17 3.44772 16.5523 3 16 3H11Z",
            }
            path {
                d: "M5 5C3.89543 5 3 5.89543 3 7V15C3 16.1046 3.89543 17 5 17H13C14.1046 17 15 16.1046 15 15V12C15 11.4477 14.5523 11 14 11C13.4477 11 13 11.4477 13 12V15H5V7L8 7C8.55228 7 9 6.55228 9 6C9 5.44772 8.55228 5 8 5H5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEyeOff;
impl IconShape for HiEyeOff {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.70711 2.29289C3.31658 1.90237 2.68342 1.90237 2.29289 2.29289C1.90237 2.68342 1.90237 3.31658 2.29289 3.70711L16.2929 17.7071C16.6834 18.0976 17.3166 18.0976 17.7071 17.7071C18.0976 17.3166 18.0976 16.6834 17.7071 16.2929L16.2339 14.8197C17.7715 13.5924 18.939 11.9211 19.5424 9.99996C18.2681 5.94288 14.4778 3 10.0002 3C8.37665 3 6.84344 3.38692 5.48779 4.07358L3.70711 2.29289ZM7.96813 6.55391L9.48201 8.0678C9.6473 8.02358 9.82102 8 10.0003 8C11.1048 8 12.0003 8.89543 12.0003 10C12.0003 10.1792 11.9767 10.353 11.9325 10.5182L13.4463 12.0321C13.7983 11.4366 14.0003 10.7419 14.0003 10C14.0003 7.79086 12.2094 6 10.0003 6C9.25838 6 8.56367 6.20197 7.96813 6.55391Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12.4541 16.6967L9.74965 13.9923C7.74013 13.8681 6.1322 12.2601 6.00798 10.2506L2.33492 6.57754C1.50063 7.57223 0.856368 8.73169 0.458008 10C1.73228 14.0571 5.52257 17 10.0002 17C10.8469 17 11.6689 16.8948 12.4541 16.6967Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEye;
impl IconShape for HiEye {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 12C11.1046 12 12 11.1046 12 10C12 8.89543 11.1046 8 10 8C8.89544 8 8.00001 8.89543 8.00001 10C8.00001 11.1046 8.89544 12 10 12Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M0.457764 10C1.73202 5.94291 5.52232 3 9.99997 3C14.4776 3 18.2679 5.94288 19.5422 9.99996C18.2679 14.0571 14.4776 17 9.99995 17C5.52232 17 1.73204 14.0571 0.457764 10ZM14 10C14 12.2091 12.2091 14 10 14C7.79087 14 6.00001 12.2091 6.00001 10C6.00001 7.79086 7.79087 6 10 6C12.2091 6 14 7.79086 14 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFastForward;
impl IconShape for HiFastForward {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5547 5.16795C4.24784 4.96338 3.8533 4.94431 3.52814 5.11833C3.20298 5.29235 3 5.63121 3 6V14C3 14.3688 3.20298 14.7077 3.52814 14.8817C3.8533 15.0557 4.24784 15.0366 4.5547 14.8321L10 11.2019V14C10 14.3688 10.203 14.7077 10.5281 14.8817C10.8533 15.0557 11.2478 15.0366 11.5547 14.8321L17.5547 10.8321C17.8329 10.6466 18 10.3344 18 10C18 9.66565 17.8329 9.35342 17.5547 9.16795L11.5547 5.16795C11.2478 4.96338 10.8533 4.94431 10.5281 5.11833C10.203 5.29235 10 5.63121 10 6V8.79815L4.5547 5.16795Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFilm;
impl IconShape for HiFilm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3C2.89543 3 2 3.89543 2 5V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V5C18 3.89543 17.1046 3 16 3H4ZM7 5L13 5V9H7V5ZM15 13V15H16V13H15ZM13 11H7V15H13V11ZM15 11H16V9H15V11ZM16 7V5H15V7H16ZM5 5V7H4V5H5ZM5 9H4V11H5V9ZM4 13H5V15H4V13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFilter;
impl IconShape for HiFilter {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C3 2.44772 3.44772 2 4 2H16C16.5523 2 17 2.44772 17 3V6C17 6.26522 16.8946 6.51957 16.7071 6.70711L12 11.4142V15C12 15.2652 11.8946 15.5196 11.7071 15.7071L9.70711 17.7071C9.42111 17.9931 8.99099 18.0787 8.61732 17.9239C8.24364 17.7691 8 17.4045 8 17V11.4142L3.29289 6.70711C3.10536 6.51957 3 6.26522 3 6V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFingerPrint;
impl IconShape for HiFingerPrint {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.62478 2.65458C7.6684 2.23213 8.80833 2 10 2C14.9706 2 19 6.02944 19 11C19 11.5523 18.5523 12 18 12C17.4477 12 17 11.5523 17 11C17 7.13401 13.866 4 10 4C9.06987 4 8.18446 4.18088 7.37522 4.50845C6.86328 4.71568 6.28029 4.46867 6.07306 3.95673C5.86584 3.4448 6.11285 2.8618 6.62478 2.65458ZM4.66173 4.95861C5.0758 5.32408 5.1152 5.95602 4.74974 6.37008C3.66007 7.60467 3 9.22404 3 11C3 11.5523 2.55228 12 2 12C1.44772 12 1 11.5523 1 11C1 8.71818 1.85048 6.63256 3.25026 5.04662C3.61573 4.63255 4.24766 4.59315 4.66173 4.95861Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 11C5 8.23858 7.23857 6 10 6C12.7614 6 15 8.23858 15 11C15 11.5523 14.5523 12 14 12C13.4477 12 13 11.5523 13 11C13 9.34315 11.6569 8 10 8C8.34315 8 7 9.34315 7 11C7 12.6772 6.65535 14.2764 6.03206 15.7288C5.81426 16.2363 5.22626 16.4712 4.71874 16.2533C4.21122 16.0355 3.97636 15.4475 4.19416 14.94C4.71247 13.7323 5 12.401 5 11ZM13.9212 13.0123C14.4666 13.0989 14.8387 13.6112 14.7521 14.1567C14.6205 14.9867 14.4378 15.7998 14.2072 16.5928C14.0531 17.1231 13.4982 17.428 12.9679 17.2739C12.4375 17.1197 12.1326 16.5648 12.2868 16.0345C12.494 15.3215 12.6584 14.5901 12.7768 13.8433C12.8634 13.2979 13.3757 12.9258 13.9212 13.0123Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 10C10.5523 10 11 10.4477 11 11C11 13.2363 10.5406 15.3679 9.71014 17.3036C9.49239 17.8111 8.90441 18.046 8.39687 17.8283C7.88933 17.6105 7.65441 17.0225 7.87217 16.515C8.59772 14.8239 9 12.9602 9 11C9 10.4477 9.44771 10 10 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFire;
impl IconShape for HiFire {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.3945 2.55279C12.2662 2.29624 12.034 2.10713 11.7568 2.03351C11.4795 1.95988 11.184 2.00885 10.9454 2.16795C10.5995 2.39858 10.3314 2.72608 10.1229 3.04791C9.90855 3.37854 9.71986 3.76148 9.553 4.16366C9.21939 4.96773 8.93911 5.93195 8.71375 6.89778C8.42752 8.12448 8.21568 9.41687 8.10004 10.4776C7.61585 10.1512 7.33491 9.78527 7.15481 9.41104C6.82729 8.73046 6.75736 7.8772 6.75736 6.75739C6.75736 6.35292 6.51372 5.98829 6.14004 5.83351C5.76637 5.67872 5.33625 5.76428 5.05025 6.05028C3.68361 7.41692 3 9.21013 3 11C3 12.7899 3.68361 14.5831 5.05025 15.9498C7.78392 18.6834 12.2161 18.6834 14.9497 15.9498C16.3164 14.5831 17 12.7899 17 11C17 9.21013 16.3164 7.41692 14.9497 6.05028C14.3584 5.45889 13.9696 5.06453 13.6021 4.5828C13.239 4.10688 12.8781 3.51991 12.3945 2.55279ZM12.1213 15.1213C10.9497 16.2929 9.05025 16.2929 7.87868 15.1213C7.29289 14.5355 7 13.7678 7 13C7 13 7.87868 13.5 9.50005 13.5C9.50005 12.5 10 9.5 10.75 9C11.25 10 11.5355 10.2929 12.1213 10.8787C12.7071 11.4645 13 12.2322 13 13C13 13.7678 12.7071 14.5355 12.1213 15.1213Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xH+At3uFDXgugya/NfyOhW7vw7/z0D3/zrbxoEP9Gb/3OH/zgofFZQm9tfJz/Wd7nF3/sm7+bfxni3+BN3/GDPwvz2fwP1hce8tM//M238sIh/pXe9B0++LuA9+Z/uCi8zs//8Df/Ni8c4l/hTd/xgz8b81n8LxCF1/n5H/7m3+aFQ7yI3vqdP/jBQ+Pp/O/wjF/8sW9+MP8yxIvozd7xg7/b5r34XyFe5hd/7Bv/mn8Z4kX0Zu/wIReNj/M/nEIf8ws/8k1fzYsG8SJ483f+4NfOxm/xP9/P/OKPffNb86JDvAje/J0/+LWz8Vv8z/aMfnP+0j/93V+9y4sO8SJ403f84M/GfBb/o8XL/OKPfeNf86+DeBG86Tt+8GdjPov/oRT6mF/4kW/6av71EC+CN33HD/5szGfxP9PP/OKPffNb82+DeBG86Tt+8GdjPov/eZ7Rb85f+qe/+6t3+bdBvAje9B0/+LMxn8X/OPEyv/hj3/jX/NshXgRv+o4f/NmYz+J/EIU+5hd+5Ju+mn8fxIvgTd/xgz8b81n8z/E7v/hj3/za/PshXgRv+o4f/NmYz+J/kCi8zs//8Df/Nv8+iBfBm77jB3825rP4H0Rot9ucPeSnv/urd/m3Q7wI3vQdP/izMZ/F/zCC3/6FH/vm1+HfDvEieNN3/ODPxnwW/xOJz/nFH/3mz+bfBvEieNN3/ODPxnwW/0NF4XV+/oe/+bf510O8CN70HT/4szGfxf9QQrvd5uwhP/3dX73Lvw7iRfCm7/jBn435LP4HE/z2L/zYN78O/zqIF8GbvuMHfzbms/ifTnzOL/7oN382LzrEi+BN3/GDPxvzWfwvEIXX+fkf/ubf5kWDeBG86Tt+8GdjPov/BYR2u83ZQ376u796l38Z4kXwpu/4wZ+N+Sz+lxD89i/82De/Dv8yxIvgTd/xgz8b81n8rxIv84s/9o1/zQuHeBG86Tt+8GdjPov/RYr0Nj/3o9/007xwiBfBm77jB3825rP4XyVe5hd/7Bv/mhcO8SJ403f84M/GfBb/e3zNL/7YN380/zLEi+BN3/GDPxvzWbxwv4P4bf6bRfDbP//D3/zbvGgQL4I3fccP/mzMZ/H8PQPirX/xx77xr/nfB/EieNN3/ODPxnwWz0eR3ubnfvSbfpr/nRAvgjd9xw/+bMxn8Xz0m/MTP/3dX73L/06IF8GbvuMHfzbms3g+fvHHvln874V4EbzpO37wZ2M+i+fjF3/sm8X/XogXwZu+4wd/NuazeD5+8ce+WfzvhXgRvOk7fvBnYz6L5+MXf+ybxf9eiBfBm77jB3825rN4Pn7xx75Z/O+FeBG86Tt+8GdjPovn4xd/7JvF/16IF8GbvuMHfzbms3g+fvHHvln874V4EbzpO37wZ2M+i+fjF3/sm8X/XogXwZu+4wd/NuazeD5+8ce+WfzvhXgRvOk7fPB7A9/F8/GLP/bN4n8vxIvgTd/hQ18a8q94Xj/ziz/2zW/N/16IF9GbvsMHfzXwUTzbJYjX/sUf+8a/5n8vxL/Cm7/zB792Jq8N0G/Mv/qnv/urd/nfDfH/G+L/N8T/b4j/3xD/vyH+f+MfAffwLlDOsbWPAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFlag;
impl IconShape for HiFlag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 6C3 4.34315 4.34315 3 6 3H16C16.3788 3 16.725 3.214 16.8944 3.55279C17.0638 3.89157 17.0273 4.29698 16.8 4.6L14.25 8L16.8 11.4C17.0273 11.703 17.0638 12.1084 16.8944 12.4472C16.725 12.786 16.3788 13 16 13H6C5.44772 13 5 13.4477 5 14V17C5 17.5523 4.55228 18 4 18C3.44772 18 3 17.5523 3 17V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderAdd;
impl IconShape for HiFolderAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM11 9C11 8.44771 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10H8C7.44772 10 7 10.4477 7 11C7 11.5523 7.44772 12 8 12H9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V12H12C12.5523 12 13 11.5523 13 11C13 10.4477 12.5523 10 12 10H11V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderDownload;
impl IconShape for HiFolderDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM11 9C11 8.44771 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10.5858L8.70711 10.2929C8.31658 9.90237 7.68342 9.90237 7.29289 10.2929C6.90237 10.6834 6.90237 11.3166 7.29289 11.7071L9.2926 13.7068L9.29289 13.7071L9.29502 13.7092C9.3904 13.804 9.50014 13.8757 9.61722 13.9241C9.73512 13.973 9.86441 14 10 14C10.1356 14 10.2649 13.973 10.3828 13.9241C10.4999 13.8757 10.6096 13.804 10.705 13.7092L10.7071 13.7071L10.7074 13.7068L12.7071 11.7071C13.0976 11.3166 13.0976 10.6834 12.7071 10.2929C12.3166 9.90237 11.6834 9.90237 11.2929 10.2929L11 10.5858V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderOpen;
impl IconShape for HiFolderOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 6C2 4.89543 2.89543 4 4 4H8L10 6H14C15.1046 6 16 6.89543 16 8V9H8C6.34315 9 5 10.3431 5 12V13.5C5 14.3284 4.32843 15 3.5 15C2.67157 15 2 14.3284 2 13.5V6Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 12C6 10.8954 6.89543 10 8 10H16C17.1046 10 18 10.8954 18 12V14C18 15.1046 17.1046 16 16 16H2H4C5.10457 16 6 15.1046 6 14V12Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6VXeZ23eS3+nf7ot37qd/ifAfEieOXXf/vXprXPMrw2/wEkffcf/+ZPvg///RD/gld+3bf9LNufzX8wSd/9x7/5k+/Dfy/EC/HKr/u27237u/hPIum7//g3f/J9+O+DeAFe+63f+/jy0qWnA8f5TyTpu//4N3/yffjvgXgBXvl13/a9bX8X/wUkffcf/+ZPvg//9RAvwCu/7tt+tu3P4r+IpO/+49/8yffhvxbiBXjl133bz7b9WfzX+mvBLv8OlnYl/fQf/8ZPfA//MsQL8Mqv+7afbfuz+N/r1lLr2/zhr/3YX/OCIV6AV37dt/1s25/F/267pdbX+cNf+7G/5vlDvACv/Lpv+9m2P4v/5QS//ce/9VOvw/OHeAFe+XXf9rNtfxb/B5RaX+YPf+3H/prnhXgBXvl13/azbX8W/wdI+pw//s2f/GyeF+IFeOXXfdvPtv1Z/B8g6XP++Dd/8rN5XogX4JVf920/2/Zn8X+ApM/549/8yc/meSFegFd+3bf9bNufxf8Bkj7nj3/zJz+b54V4AV75dd/2s21/Fv8HSPqcP/7Nn/xsnhfiBXjl133bz7b9WfwfIOlz/vg3f/KzeV6IF+CVX/dtP9v2Z/F/gKTP+ePf/MnP5nkhXoBXft23/Wzbn8X/AZI+549/8yc/m+eFeAFe+XXf9rNtfxb/Pn8TEZ/9R7/xEz/Nv8GrvN7bvXVmfjbwUvw7SPqcP/7Nn/xsnhfiBXjl133bz7b9WfwbCZ4xP3bspX/7p797l3+H137r9z6+vHTpVuAY/0aSPuePf/MnP5vnhXgBXvl13/azbX8W/0aSPuePf/MnP5v/AK/8um/72bY/i38jSZ/zx7/5k5/N80K8AK/8um/72bY/i38jSZ/zx7/5k5/Nf4BXft23/Wzbn8W/kaTP+ePf/MnP5nkhXoBXft23/Wzbn8W/keC3//i3fup1+A/wyq/zNr9leG3+jSR9zh//5k9+Ns8L8QK88uu+7Wfb/iz+PSI++k9+4ye+hn+HV3q9t/soMr+afwdJn/PHv/mTn83zQrwAr/y6b/vZtj+Lf7+/lvTXwK386zzY9ksDL82/k6TP+ePf/MnP5nkhXoBXft23/Wzbn8X/AZI+549/8yc/m+eFeAFe+XXf9rNtfxb/B0j6nD/+zZ/8bJ4X4gV45dd928+2/Vn8HyDpc/74N3/ys3leiBfglV/3bT/b9mfxf4Ckz/nj3/zJz+Z5IV6AV37dt/1s25/F/wGSPuePf/MnP5vnhXgBXvl13/azbX8W/wdI+pw//s2f/GyeF+IFeOXXfdvPtv1Z/B8g6XP++Dd/8rN5XogX4JVf923f2/Z38X9ARLzNH/3GT/w0zwvxArz2W7/38eWlS7cCx/jf7dLi2LEH//ZPf/cuzwvxQrzy677te9v+Lv4Xk/Q+f/ybP/ndPH+If8Erv+7bfrbtz+J/IUmf88e/+ZOfzQuGeBG88uu//Wu7tY8G3or/HX5GpXz1H//6j/82LxziX+mVX//tX5v/wf7413/8t3nRIf5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xv/CFZcdVAbG5W6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderRemove;
impl IconShape for HiFolderRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM8 10C7.44772 10 7 10.4477 7 11C7 11.5523 7.44772 12 8 12H12C12.5523 12 13 11.5523 13 11C13 10.4477 12.5523 10 12 10H8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAErUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3eIcPeS3+nX7ux77pd/ifAfEiePN3/uDXduOzDK/Nf4zv/sUf++b34b8f4l/wpu/4wZ+F+Wz+4333L/7YN78P/70QL8SbvsMHvzfwXfzn+e5f/LFvfh/++yBegLd+748+Ph6un258nP9c3/2LP/bN78N/D8QL8Kbv8MHvDXwX/zW++xd/7Jvfh/96iBfgTd/xgz8b81n81/nuX/yxb34f/mshXoA3fccP/mzMZ/FfSn8tvMu/g6VdwU//wo9+0/fwL0O8AG/6jh/82ZjP4n8rcSuOt/nFH/vGv+YFQ7wAb/qOH/zZmM/ifzGhXaPX+cUf+8a/5vlDvABv+o4f/NmYz+J/OcFv/8KPffPr8PwhXoA3fccP/mzMZ/F/QrzML/7YN/41zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt88HsD38X/AUV6m5/70W/6aZ4X4gV46/f+6OPD4epW4Bj/u13qN+cP/unv/updnhfihXjTd/jg9wa+i//d3ucXf+ybv5vnD/EveNN3/ODPxnwW/xuJz/nFH/3mz+YFQ7wI3vydP/i1s/HRwFvxv8PPROGrf/6Hv/m3eeEQ/0pv/s4f/Nr8D/bzP/zNv82LDvH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R02Q/EEsE4+qAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolder;
impl IconShape for HiFolder {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H9L11 6H16C17.1046 6 18 6.89543 18 8V14C18 15.1046 17.1046 16 16 16H4C2.89543 16 2 15.1046 2 14V6Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGift;
impl IconShape for HiGift {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 5C5 3.34315 6.34315 2 8 2C8.76836 2 9.46924 2.28885 10 2.7639C10.5308 2.28885 11.2316 2 12 2C13.6569 2 15 3.34315 15 5C15 5.35064 14.9398 5.68722 14.8293 6H16C17.1046 6 18 6.89543 18 8C18 9.10457 17.1046 10 16 10H11V9C11 8.44772 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10H4C2.89543 10 2 9.10457 2 8C2 6.89543 2.89543 6 4 6H5.17071C5.06015 5.68722 5 5.35064 5 5ZM9 6V5C9 4.44772 8.55228 4 8 4C7.44772 4 7 4.44772 7 5C7 5.55228 7.44772 6 8 6H9ZM12 6C12.5523 6 13 5.55228 13 5C13 4.44772 12.5523 4 12 4C11.4477 4 11 4.44772 11 5V6H12Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M9 11H3V16C3 17.1046 3.89543 18 5 18H9V11Z",
            }
            path {
                d: "M11 18H15C16.1046 18 17 17.1046 17 16V11H11V18Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGlobeAlt;
impl IconShape for HiGlobeAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.08296 9H6.02863C6.11783 7.45361 6.41228 6.02907 6.86644 4.88228C5.41752 5.77135 4.37513 7.25848 4.08296 9ZM10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2ZM10 4C9.92395 4 9.76787 4.03173 9.5347 4.26184C9.29723 4.4962 9.03751 4.8849 8.79782 5.44417C8.40914 6.3511 8.12491 7.58559 8.03237 9H11.9676C11.8751 7.58559 11.5909 6.3511 11.2022 5.44417C10.9625 4.8849 10.7028 4.4962 10.4653 4.26184C10.2321 4.03173 10.076 4 10 4ZM13.9714 9C13.8822 7.45361 13.5877 6.02907 13.1336 4.88228C14.5825 5.77135 15.6249 7.25848 15.917 9H13.9714ZM11.9676 11H8.03237C8.12491 12.4144 8.40914 13.6489 8.79782 14.5558C9.03751 15.1151 9.29723 15.5038 9.5347 15.7382C9.76787 15.9683 9.92395 16 10 16C10.076 16 10.2321 15.9683 10.4653 15.7382C10.7028 15.5038 10.9625 15.1151 11.2022 14.5558C11.5909 13.6489 11.8751 12.4144 11.9676 11ZM13.1336 15.1177C13.5877 13.9709 13.8822 12.5464 13.9714 11H15.917C15.6249 12.7415 14.5825 14.2287 13.1336 15.1177ZM6.86644 15.1177C6.41228 13.9709 6.11783 12.5464 6.02863 11H4.08296C4.37513 12.7415 5.41752 14.2287 6.86644 15.1177Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGlobe;
impl IconShape for HiGlobe {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM4.33179 8.02741C4.70542 6.95361 5.37558 6.01864 6.24421 5.32056C6.51209 5.72966 6.97449 5.99991 7.50001 5.99991C8.32844 5.99991 9.00001 6.67148 9.00001 7.49991V7.99991C9.00001 9.10448 9.89545 9.99991 11 9.99991C12.1046 9.99991 13 9.10448 13 7.99991C13 7.05979 13.6487 6.27118 14.5228 6.05719C15.4428 7.11161 16 8.49069 16 9.99992C16 10.3407 15.9716 10.6748 15.917 11.0001H15C13.8954 11.0001 13 11.8955 13 13.0001V15.1973C12.1175 15.7078 11.0928 15.9999 9.99992 15.9999V14C9.99992 12.8954 9.10448 12 7.99992 12C6.89535 12 5.99992 11.1046 5.99992 10C5.99992 9.00849 5.27841 8.1855 4.33179 8.02741Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHand;
impl IconShape for HiHand {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3V8V8.5C11 8.77614 11.2239 9 11.5 9C11.7761 9 12 8.77614 12 8.5V8V4C12 3.44772 12.4477 3 13 3C13.5523 3 14 3.44772 14 4V8V8.5C14 8.77614 14.2239 9 14.5 9C14.7761 9 15 8.77614 15 8.5V8V6C15 5.44772 15.4477 5 16 5C16.5523 5 17 5.44772 17 6V11C17 14.866 13.866 18 10 18C6.13401 18 3 14.866 3 11V9C3 8.44772 3.44772 8 4 8C4.55228 8 5 8.44772 5 9V11V11.5C5 11.7761 5.22386 12 5.5 12C5.77614 12 6 11.7761 6 11.5V11V10V8V4C6 3.44772 6.44772 3 7 3C7.55228 3 8 3.44772 8 4V8V8.5C8 8.77614 8.22386 9 8.5 9C8.77614 9 9 8.77614 9 8.5V8V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHashtag;
impl IconShape for HiHashtag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.24254 3.02985C9.77833 3.1638 10.1041 3.70673 9.97014 4.24253L9.53078 5.99999H12.4692L13.0299 3.75746C13.1638 3.22166 13.7067 2.8959 14.2425 3.02985C14.7783 3.1638 15.1041 3.70673 14.9701 4.24253L14.5308 5.99999H17C17.5523 5.99999 18 6.44771 18 6.99999C18 7.55228 17.5523 7.99999 17 7.99999H14.0308L13.0308 12H15C15.5523 12 16 12.4477 16 13C16 13.5523 15.5523 14 15 14H12.5308L11.9701 16.2425C11.8362 16.7783 11.2933 17.1041 10.7575 16.9701C10.2217 16.8362 9.89591 16.2933 10.0299 15.7575L10.4692 14H7.53078L6.97014 16.2425C6.83619 16.7783 6.29326 17.1041 5.75746 16.9701C5.22167 16.8362 4.89591 16.2933 5.02986 15.7575L5.46922 14H3C2.44772 14 2 13.5523 2 13C2 12.4477 2.44772 12 3 12H5.96922L6.96922 7.99999H5C4.44772 7.99999 4 7.55228 4 6.99999C4 6.44771 4.44772 5.99999 5 5.99999H7.46922L8.02986 3.75746C8.16381 3.22166 8.70674 2.8959 9.24254 3.02985ZM9.03078 7.99999L8.03078 12H10.9692L11.9692 7.99999H9.03078Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHeart;
impl IconShape for HiHeart {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.17157 5.17157C4.73367 3.60948 7.26633 3.60948 8.82843 5.17157L10 6.34315L11.1716 5.17157C12.7337 3.60948 15.2663 3.60948 16.8284 5.17157C18.3905 6.73367 18.3905 9.26633 16.8284 10.8284L10 17.6569L3.17157 10.8284C1.60948 9.26633 1.60948 6.73367 3.17157 5.17157Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+m7zpO3zoS6N8KwAcP/OLP/aNf81/PcR/gzd9hw9+b+C7eE7v84s/9s3fzX8txH+xN32HD35v4Lt4/t7nF3/sm7+b/zqI/0Jv+g4f/N7Ad/HCvc8v/tg3fzf/NRD/Rd70HT74vYHv4kXzPr/4Y9/83fznQ/wXeNN3+OD3Br6Lf533+cUf++bv5j8X4j/Zm77DB7838F3827zPL/7YN383/3kQ/4ne9B0++L2B7+Lf531+8ce++bv5z4H4T/Km7/DB7w18F/8x3ucXf+ybv5v/eIj/BG/6Dh/83sB38R/rfX7xx775u/mPhfgP9qbv8MHvDXwX/zne5xd/7Ju/m/84iP9Ab/oOH/zewHfxn+t9fvHHvvm7+Y+B+A/ypu/wwe8NfBf/Nd7nF3/sm7+bfz/Ef4A3fYcPfm/gu/iv9T6/+GPf/N38+yD+nd70HT74vYHv4r/H+/zij33zd/Nvh/h3eNN3+OD3Br6L/17v84s/9s3fzb8N4t/oTd/hg98b+C7+Z3ifX/yxb/5u/vUQ/wZv+g4f/N7Ad/E/y/v84o9983fzr4P4V3rTd/jg9wa+i/+Z3ucXf+ybv5sXHeJf4U3f4YPfG/gu/md7n1/8sW/+bl40iBfRm77DB7838F387/A+v/hj3/zd/MsQL4I3fYcPfm/gu/jf5X1+8ce++bt54RD/gjd9hw9+b+C7+N/pfX7xx775u3nBEC/Em77DB7838F387/Y+v/hj3/zdPH+IF+BN3+FDXxryr/g/IV7mF3/sG/+a54V4Ad70HT/4szGfxf8F4nN+8Ue/+bN5XogX4E3f8YM/G/NZ/F8gPucXf/SbP5vnhXgB3vQdPvSlIf+K5yLxNzYvxf9AEn9j81I8j3iZX/yxb/xrnhfihXiLd/yQt272dwPHABCfE8FvZ+O3+B8oCq+TyWtjPosrLhXpvX/uR7/pp3n+EC+CN32HD33pX/yxb/xrgDd/5w9+7Wz8Fv8DReF1fv6Hv/m3Ad70HT70pX/xx77xr3nhEP9Kb/7OH/za2fgt/geKwuv8/A9/82/zokP8K735O3/wa2fjt/gfKAqv8/M//M2/zYsO8a/05u/8wa+djd/if6AovM7P//A3/zYvOsS/0pu/8we/djZ+i/+BovA6P//D3/zbvOgQ/0pv/s4f/NrZ+C3+B4rC6/z8D3/zb/OiQ/wrvfk7f/BrZ+O3+B8oCq/z8z/8zb/Niw7xr/Tm7/zBr52N3+J/oCi8zs//8Df/Ni86xL/Sm7/zB792Nn6L/4Gi8Do//8Pf/Nu86BD/Sm/+zh/82tn4Lf7tngHcyvP3YOBB/BtF4XV+/oe/+bd50SH+ld78nT/4tbPxW/wrSfxNF7z1T//wN9/KC/HW7/zBDx6Tn7Z5Kf6VovA6P//D3/zbvOgQ/0pv/s4f/NrZ+C3+dS71m/MH//R3f/UuL4K3fu+PPj4crm4FjvGvEIXX+fkf/ubf5kWH+Fd683f+4NfOxm/xr/Mzv/hj3/zW/Cu86Tt88G8Dr8W/QhRe5+d/+Jt/mxcd4l/pzd/5g187G7/Fv4b4nF/80W/+bP4V3vQdP/izMZ/Fv0IUXufnf/ibf5sXHeJf6c3f+YNfOxu/xb+G+Jxf/NFv/mz+Fd70HT/4szGfxb9CFF7n53/4m3+bFx3iX+nN3/mDXzsbv8W/hvicX/zRb/5s/hXe9B0/+LMxn8W/QhRe5+d/+Jt/mxcd4l/pzd/5g187G7/Fv4b4nF/80W/+bP4V3vQdP/izMZ/Fv0IUXufnf/ibf5sXHeJf6c3f+YNfOxu/xb+G+Jxf/NFv/mz+Fd70HT/4szGfxb9CFF7n53/4m3+bFx3iX+nN3/mDXzsbv8W/hvicX/zRb/5s/hXe9B0/+LMxn8W/QhRe5+d/+Jt/mxcd4l/pTd/hQ18a8q/41xCf84s/+s2fzb/Cm77jB3825rP4V4mX+cUf+8a/5kWH+Dd403f44F3gGC+69/nFH/vm7+Zf4U3f4YPfG/guXnSXfvHHvvk4/zqIf4M3fYcPfm/gu3jR/Mwv/tg3vzX/Bm/6Dh/808Bb8SIo0tv83I9+00/zr4P4N3rrd/7gBw/Je/PCmFt/8ce++bv5d3jTd/jg90Y8mBeiD777p3/4m2/lXw/x/xvi/zfE/2+I/98Q/78h/n/jHwFV+aFQt8zo7QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHome;
impl IconShape for HiHome {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.7071 2.29289C10.3166 1.90237 9.68342 1.90237 9.29289 2.29289L2.29289 9.29289C1.90237 9.68342 1.90237 10.3166 2.29289 10.7071C2.68342 11.0976 3.31658 11.0976 3.70711 10.7071L4 10.4142V17C4 17.5523 4.44772 18 5 18H7C7.55228 18 8 17.5523 8 17V15C8 14.4477 8.44772 14 9 14H11C11.5523 14 12 14.4477 12 15V17C12 17.5523 12.4477 18 13 18H15C15.5523 18 16 17.5523 16 17V10.4142L16.2929 10.7071C16.6834 11.0976 17.3166 11.0976 17.7071 10.7071C18.0976 10.3166 18.0976 9.68342 17.7071 9.29289L10.7071 2.29289Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiIdentification;
impl IconShape for HiIdentification {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C9.44772 2 9 2.44772 9 3V4C9 4.55228 9.44772 5 10 5C10.5523 5 11 4.55228 11 4V3C11 2.44772 10.5523 2 10 2ZM4 4H7C7 5.65685 8.34315 7 10 7C11.6569 7 13 5.65685 13 4H16C17.1046 4 18 4.89543 18 6V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V6C2 4.89543 2.89543 4 4 4ZM6.5 11C7.32843 11 8 10.3284 8 9.5C8 8.67157 7.32843 8 6.5 8C5.67157 8 5 8.67157 5 9.5C5 10.3284 5.67157 11 6.5 11ZM8.95048 15C8.98327 14.8384 9.00049 14.6712 9.00049 14.5C9.00049 13.1193 7.8812 12 6.50049 12C5.11978 12 4.00049 13.1193 4.00049 14.5C4.00049 14.6712 4.0177 14.8384 4.0505 15H8.95048ZM12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11H15C15.5523 11 16 10.5523 16 10C16 9.44772 15.5523 9 15 9H12ZM11 13C11 12.4477 11.4477 12 12 12H14C14.5523 12 15 12.4477 15 13C15 13.5523 14.5523 14 14 14H12C11.4477 14 11 13.5523 11 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAKc0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+m7zpO3zoS6N8KwAcP/OLP/aNf81/PcR/gzd9hw9+b+C7eIAivc3P/eg3/TT/tRD/Dd7sHT7kovFxHkBo9xd+7JtO8F8L8W/0pu/woS9N6EG4vTT3U/nrXu1vfvqHv/lWXoA3fYcPfWnIv+L5ipf5xR/7xr/mBXjrd/7gBw8uL4XbS3M/lb8m/Yxf/LFv/Gv+9RD/Sm/xjh/y1g1/FebBPB9Cu8Yf84s/9s3fzfPx5u/8wa+djd/i+YjC6/z8D3/zb/N8vOk7fPB7C32V8XGeH3FrQR/zcz/6TT/Niw7xr/Cm7/jBn435LP4FQrtd8cv89A9/8608lzd/5w9+7Wz8Fs9HFF7n53/4m3+b5/LW7/zBDx6b/sr4OP8S8Tm/+KPf/Nm8aBAvord4xw9562b/FC+iIr3Nz/3oN/00z+XN3/mDXzsbv8XzEYXX+fkf/ubf5rm8xTt+yFs3+6d4ERXpbX7uR7/pp/mXIV5Eb/qOH/x0zIN5EUXhdX7+h7/5t3kub/7OH/za2fgtno8ovM7P//A3/zbP5c3f+YNfOxu/xYtK3PqLP/rND+FfhngRvOk7fOhLQ/4VLyKJv/mFH/3ml+b5ePN3/uDXzsZv8XxE4XV+/oe/+bd5Pt7sHT/4r21eihdZvMwv/tg3/jUvHOJF8Bbv+CFv3eyf4l92SeKnu435R//0d3/1Ls/Hm7/zB792Nn6L5yMKr/PzP/zNv83z8dbv/dHHh8PVdwOvDRzjX1Ckt/m5H/2mn+aFQ7wI3vQdP/izMZ/F8/GLP/bN4l/hzd/5g187G7/F8xGF1/n5H/7m3+Zf4U3f4YPN8yM+5xd/9Js/mxcO8SJ403f84M/GfBbPxy/+2DeLf4U3f+cPfu1s/BbPRxRe5+d/+Jt/m3+FN32HDzbPj/icX/zRb/5sXjjEi+BN3/GDPxvzWTwfv/hj3yz+Fd78nT/4tbPxWzwfUXidn//hb/5t/hXe9B0+2Dw/4nN+8Ue/+bN54RAvgjd9xw/+bMxn8Xz84o99s/hXePN3/uDXzsZv8XxE4XV+/oe/+bf5V3jTd/hg8/yIz/nFH/3mz+aFQ7wI3vQdP/izMZ/F8/GLP/bN4l/hzd/5g187G7/F8xGF1/n5H/7m3+Zf4U3f4YPN8yM+5xd/9Js/mxcO8SJ403f84M/GfBbPxy/+2DeLf4U3f+cPfu1s/BbPRxRe5+d/+Jt/m3+FN32HDzbPj/icX/zRb/5sXjjEi+BN3/GDPxvzWTwfv/hj3yxeiDd7xw95L+MHh2O3bvbfM61WL52N3+L5iMLr1Pn8r6fD4b1SeVzo1l/40W/6Hl6IN32HDzbPj/icX/zRb/5sXjjEi+BN3/GDPxvzWTwfv/hj3yxegDd9xw/5Key35n7iVklf4/RX8Xwo9DG2PwrzYO4n/fQv/ug3vQ0vwJu+wweb50d8zi/+6Dd/Ni8c4kXwpu/4wZ+N+Syej1/8sW8Wz8ebvsMHvzfwXfzHeJ9f/LFv/m6ejzd9hw82z4/4nF/80W/+bF44xIvgTd/xgz8b81k8H7/4Y98sno83fccP/mzMZ/EfQXzOL/7oN382z8ebvsMHm+dHfM4v/ug3fzYvHOJF8Kbv+MGfjfksno9f/LFvFs/Hm73Th3y001/FfwCFPuYXfuSbvprn403f4YPN8yM+5xd/9Js/mxcO8SJ403f84M/GfBbPxy/+2DeL5+Ot3/ujjw+Hq78GHsS/zzP6zflL//R3f/Uuz8ebvsMHm+dHfM4v/ug3fzYvHOJF8Kbv+MGfjfksno9f/LFvFi/Am77Dh7405G8Dx/i3uQTx2r/4Y9/417wAb/oOH2yeH/E5v/ij3/zZvHCIF8GbvuMHfzbms3g+fvHHvlm8EG/6Dh/60pC/DRzjX+cSxGv/4o9941/zQrzpO3yweX7E5/zij37zZ/PCIV4Eb/qOH/zZmM/i+fjFH/tm8S9403f40JeG/G3gGC+aSxCv/Ys/9o1/zb/gTd/hg83zIz7nF3/0mz+bFw7xInjTd/zgz8Z8Fs/HL/7YN4sXwZu+w4e+NORvA8d44S5BvPYv/tg3/jUvgjd9hw82z4/4nF/80W/+bF44xIvgTd/xgz8b81k8H7/4Y98sXkRv+g4f+tKQvw0c4/m7BPHav/hj3/jXvIje9B0+2Dw/4nN+8Ue/+bN54RAvgjd9xw/+bMxn8Xz84o99s/hXeNN3+NCXhvxt4BjP6RLEa//ij33jX/Ov8Kbv8MHm+RGf84s/+s2fzQuHeBG86Tt+8GdjPovn4xd/7JvFv9Jbv/MHP3hofDfwWlzxO33hvX/6h7/5Vv6V3vQdPtg8P+JzfvFHv/mzeeEQL4I3fccP/mzMZ/F8/OKPfbP4N3rr9/7o4wA//d1fvcu/0Zu+wweb50d8zi/+6Dd/Ni8c4kXwpu/4wZ+N+Syej1/8sW8W/43e9B0+2Dw/4nN+8Ue/+bN54RAvgjd9xw/+bMxn8Xz84o99s/hv9Kbv8MHm+RGf84s/+s2fzQuHeBG86Tt+8GdjPovnR3w2z0dBf/NzP/pNP81/gLd4xw9564ZfiufHfDbPj/icX/zRb/5sXjjEi+BN3/GDPxvzWfwrCO12m7OH/PR3f/Uu/w5v/d4ffXw8XD/d+Dj/GuJzfvFHv/mzeeEQL4I3fccP/mzMZ/GvJT7nF3/0mz+bf4c3fccP/mzMZ/GvJT7nF3/0mz+bFw7xInjTd/zgz8Z8Fv9KQrvd5uwhP/3dX73Lv8Fbv/dHHx8P1083Ps6/lvicX/zRb/5sXjjEi+BN3/GDPxvzWfxbiM/5xR/95s/m3+BN3/GDPxvzWfxbiM/5xR/95s/mhUO8CN70HT/4szGfxb+B0G63OXvIT3/3V+/yr/DW7/3Rx8fD9dONj/NvIT7nF3/0mz+bFw7xInjTd/zgz8Z8Fv9Gkr46zE/zr5DirW1/NP9W4nN+8Ue/+bN54RAvgjd9xw/+bMxn8b+J+Jxf/NFv/mxeOMSL4E3f8YM/G/NZ/G8iPucXf/SbP5sXDvEieNN3/ODPxnwW/5uIz/nFH/3mz+aFQ7wI3uIdP+Stm/1T/C9SpLf5uR/9pp/mhUO8CN70HT70pSH/iv9V4mV+8ce+8a954RAvojd9hw++FXgQ/zs84xd/7JsfzL8M8SJ6i3f8kLdu9k/xv0CR3ubnfvSbfpp/GeJf4U3f8YM/G/NZ/E8mPucXf/SbP5sXDeJf6S3e8UPeutlfDTyI/1meUaSP/rkf/aaf5kWH+Dd603f40Jcu8oMbfmn+GxX018269Rd/7Bv/mn89xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b/wjf5TlX8MGlYIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInboxIn;
impl IconShape for HiInboxIn {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.70711 7.29289C8.31658 6.90237 7.68342 6.90237 7.29289 7.29289C6.90237 7.68342 6.90237 8.31658 7.29289 8.70711L9.29289 10.7071C9.48043 10.8946 9.73478 11 10 11C10.2652 11 10.5196 10.8946 10.7071 10.7071L12.7071 8.70711C13.0976 8.31658 13.0976 7.68342 12.7071 7.29289C12.3166 6.90237 11.6834 6.90237 11.2929 7.29289L11 7.58579L11 3C11 2.44771 10.5523 2 10 2C9.44771 2 9 2.44772 9 3V7.58579L8.70711 7.29289Z",
            }
            path {
                d: "M3 5C3 3.89543 3.89543 3 5 3H6C6.55228 3 7 3.44772 7 4C7 4.55228 6.55228 5 6 5L5 5V12H7L8 14H12L13 12H15V5H14C13.4477 5 13 4.55228 13 4C13 3.44772 13.4477 3 14 3H15C16.1046 3 17 3.89543 17 5V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInbox;
impl IconShape for HiInbox {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 3C3.89543 3 3 3.89543 3 5V15C3 16.1046 3.89543 17 5 17H15C16.1046 17 17 16.1046 17 15V5C17 3.89543 16.1046 3 15 3H5ZM5 5L15 5V12H13L12 14H8L7 12H5V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIH0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+C73FO3zIa7WI47i9NM+Pyl+TfsYv/tg3/jX/NRD/id76nT/4wVOLtzL51obX5l9B8NsifrqW/Jmf/uFvvpX/HIj/BG/2jh/yXjYfDX5p/gMIfhvpu3/hR7/pe/iPhfgP9Bbv+CFv3fBXYR7MfwZxa0Ef83M/+k0/zX8MxH+At37nD37w2Pguw2vzX0Dw213hfX76h7/5Vv59EP9Ob/oOH/zeQl9lfJz/QkK7Id7n5370m36afzvEv8ObvsMHfxfw3vw3kvTVv/Cj3/Qx/Nsg/o3e9B0++LuA9+Z/hu/+xR/75vfhXw/xb/Cm7/DB3wW8N/+zfPcv/tg3vw//Ooh/pTd9hw/+LuC9+Z/pa37xx775o3nRIf4V3vQdPvi9ge/i3++SQp/dyT8NMFpv7fRnA8f4dyrS2/zcj37TT/OiQbyI3vqdP/jBY9NfGR/n36lIb/NzP/pNP80DvMU7fshbN/un+HcS2u2KX+anf/ibb+VfhngRvdk7fPBvGV6bf79Lv/hj33yc5+NN3+GDd4Fj/DsJfvsXfuybX4d/GeJF8Bbv+CFv3eyf4j+CuPUXf/SbH8Lz8abv+MFPxzyY/wBFepuf+9Fv+mleOMSL4E3f8YOfjnkw/0H6wkN++oe/+VYe4K3f+YMfPDSezn8Ucesv/ug3P4QXDvEveNN3+OD3Br6L/1D6635z9jo//d1fvQvw1u/90ceHw/VvgV+a/1jv84s/9s3fzQuG+Be86Tt8yF+BX5r/YEK74L8GAL208XH+gwl++xd+7JtfhxcM8UK89Tt/8IOHxtP5X6wvPOSnf/ibb+X5Q7wQb/ZOH/LRTn8V/4sp9DG/8CPf9NU8f4gX4k3f4YN/G3gt/nf7nV/8sW9+bZ4/xAvxpu/wweb/gF/8sW8Wzx/iBXjzd/7g187Gb/F/QrzML/7YN/41zwvxArzFO37IWzf7p/iP9wzgVp7twcCD+E9UpLf5uR/9pp/meSFegDd9xw/+bMxn8R9E4m+64K1/+oe/+Vaey5u/8we/djZ+GjjGfwbxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+Y1zqN+cP/unv/updXoA3f+cPfu1s/Bb/GcTn/OKPfvNn87wQL8CbvuMHfzbms/iP8TO/+GPf/Nb8C970HT54FzjGfzTxOb/4o9/82TwvxAvwpu/4wZ+N+Sz+Y/zML/7YN781/4I3fYcP/m3gtfiPJj7nF3/0mz+b54V4Ad70HT/4szGfxX8Aod1uc/aQn/7ur97lhXjTd/jg3wZei/9o4nN+8Ue/+bN5XogX4E3f8YM/G/NZ/IfRX/fFb/PTP/zNt/ICvOk7fPBvA6/FfzTxOb/4o9/82TwvxAvwFu/4IW/d7J/iP97P/OKPffNb83y86Tt88G8Dr8V/sCK9zc/96Df9NM8L8QK8+Tt/8Gtn47f4j/c7v/hj3/zaPB9v+g4f/NvAa/EfLl7mF3/sG/+a54V4Id70HT7Y/Mf7nV/8sW9+bZ6PN32HD/5t4LX4D/aLP/bN4vlDvBBv+g4f/NvAa/Ef63d+8ce++bV5Pt70HT74t4HX4j/W7/zij33za/P8IV6IN3unD/lop7+K/1i/84s/9s2vzfPxpu/wwb8NvBb/gRT6mF/4kW/6ap4/xAvx1u/8wQ8eGk/nP9bv/OKPffNr83y86Tt88G8Dr8V/oL7wkJ/+4W++lecP8S9403f44N8GXov/OL/ziz/2za/N8/Gm7/DBXw28NfAg/mP8zi/+2De/Ni8Y4l/wpu/wwe8NfBf/cX7nF3/sm1+bF+Ct3/ujj49Hq6+2eS/+/d7nF3/sm7+bFwzxInjTd/jgW4EH8R/jd37xx775tXkh3vq9P/r4cLj6a+BB/Ns94xd/7JsfzAuHeBG8xTt+yFs3+6f4j/E7v/hj3/za/Ave9B0++KeBt+LfqEhv83M/+k0/zQuHeBG96Tt88G8Dr8W/k9DuL/zYN53gX/Cm7/DBvw28Fv82v/OLP/bNr82/DPEieut3/uAHD42/Bo7x71Skt/m5H/2mn+YFeNN3+NCXhvwr/m0u9YWX/ukf/uZb+Zch/hXe4h0/5K2b/VP8OwntCn12LfkzP/3D33wrz/TW7/3Rx8ej9Vthvtr4OP8GRXqbn/vRb/ppXjSIf6U3e8cP/m6b9+J/pq/5xR/75o/mRYf4N3izd/zg77Z5L/4HkfieX/jRb35v/nUQ/0Zv9o4f/N0278X/ABLf8ws/+s3vzb8e4t/hTd/hg78a+Cj+e33NL/7YN380/zaIf6e3eMcPeetmfzdwjP9al4r03j/3o9/00/zbIf4DvPU7f/CDh8Z3A6/Ff43f6Qvv/dM//M238u+D+A/0Fu/4IW/d7K8GHsR/jmcU6aN/7ke/6af5j4H4T/Cm7/DB7w28N/Ba/Mf4HeC7f/HHvvm7+Y+F+E/01u/8wQ8erbd2+q2B1+Jf53cU+ulO/umf/uFvvpX/HIj/Qm/6Dh/60kV+cMMvzfNR0F8369Zf/LFv/Gv+ayD+f0P8/4b4/w3x/xvi/zfE/2/8I3uzHF9M4XobAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInformationCircle;
impl IconShape for HiInformationCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM11 6C11 6.55228 10.5523 7 10 7C9.44772 7 9 6.55228 9 6C9 5.44772 9.44772 5 10 5C10.5523 5 11 5.44772 11 6ZM9 9C8.44772 9 8 9.44772 8 10C8 10.5523 8.44772 11 9 11V14C9 14.5523 9.44772 15 10 15H11C11.5523 15 12 14.5523 12 14C12 13.4477 11.5523 13 11 13V10C11 9.44772 10.5523 9 10 9H9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiKey;
impl IconShape for HiKey {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 8C18 11.3137 15.3137 14 12 14C11.3938 14 10.8087 13.9101 10.2571 13.7429L10 14L9 15L8 16H6V18H2V14L6.25707 9.74293C6.08989 9.19135 6 8.60617 6 8C6 4.68629 8.68629 2 12 2C15.3137 2 18 4.68629 18 8ZM12 4C11.4477 4 11 4.44772 11 5C11 5.55228 11.4477 6 12 6C13.1046 6 14 6.89543 14 8C14 8.55228 14.4477 9 15 9C15.5523 9 16 8.55228 16 8C16 5.79086 14.2091 4 12 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLibrary;
impl IconShape for HiLibrary {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.4963 2.13176C10.1889 1.95608 9.81146 1.95608 9.50403 2.13176L2.50403 6.13176C2.02451 6.40577 1.85792 7.01662 2.13193 7.49614C2.31631 7.81881 2.65322 7.99979 3 8.00017V15C2.44772 15 2 15.4477 2 16C2 16.5523 2.44772 17 3 17H17C17.5523 17 18 16.5523 18 16C18 15.4477 17.5523 15 17 15V8.00017C17.3469 7.99991 17.684 7.81892 17.8684 7.49614C18.1424 7.01662 17.9758 6.40577 17.4963 6.13176L10.4963 2.13176ZM6 9C5.44772 9 5 9.44772 5 10V13C5 13.5523 5.44772 14 6 14C6.55228 14 7 13.5523 7 13V10C7 9.44772 6.55228 9 6 9ZM9 10C9 9.44772 9.44772 9 10 9C10.5523 9 11 9.44772 11 10V13C11 13.5523 10.5523 14 10 14C9.44772 14 9 13.5523 9 13V10ZM14 9C13.4477 9 13 9.44772 13 10V13C13 13.5523 13.4477 14 14 14C14.5523 14 15 13.5523 15 13V10C15 9.44772 14.5523 9 14 9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLightBulb;
impl IconShape for HiLightBulb {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 3C11 2.44772 10.5523 2 10 2C9.44771 2 9 2.44772 9 3V4C9 4.55228 9.44771 5 10 5C10.5523 5 11 4.55228 11 4V3Z",
            }
            path {
                d: "M15.6568 5.75731C16.0473 5.36678 16.0473 4.73362 15.6568 4.34309C15.2663 3.95257 14.6331 3.95257 14.2426 4.34309L13.5355 5.0502C13.145 5.44072 13.145 6.07389 13.5355 6.46441C13.926 6.85494 14.5592 6.85494 14.9497 6.46441L15.6568 5.75731Z",
            }
            path {
                d: "M18 10C18 10.5523 17.5523 11 17 11H16C15.4477 11 15 10.5523 15 10C15 9.44771 15.4477 9 16 9H17C17.5523 9 18 9.44771 18 10Z",
            }
            path {
                d: "M5.05019 6.46443C5.44071 6.85496 6.07388 6.85496 6.4644 6.46443C6.85493 6.07391 6.85493 5.44074 6.4644 5.05022L5.7573 4.34311C5.36677 3.95259 4.73361 3.95259 4.34308 4.34311C3.95256 4.73363 3.95256 5.3668 4.34308 5.75732L5.05019 6.46443Z",
            }
            path {
                d: "M5 10C5 10.5523 4.55228 11 4 11H3C2.44772 11 2 10.5523 2 10C2 9.44771 2.44772 9 3 9H4C4.55228 9 5 9.44771 5 10Z",
            }
            path {
                d: "M8 16V15H12V16C12 17.1046 11.1046 18 10 18C8.89543 18 8 17.1046 8 16Z",
            }
            path {
                d: "M12.0009 14C12.0155 13.6597 12.2076 13.3537 12.4768 13.1411C13.4046 12.4086 14 11.2738 14 10C14 7.79086 12.2091 6 10 6C7.79086 6 6 7.79086 6 10C6 11.2738 6.59545 12.4086 7.52319 13.1411C7.79241 13.3537 7.98451 13.6597 7.99911 14H12.0009Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLightningBolt;
impl IconShape for HiLightningBolt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.3006 1.04621C11.7169 1.17743 12 1.56348 12 1.99995V6.99995L16 6.99995C16.3729 6.99995 16.7148 7.20741 16.887 7.53814C17.0592 7.86887 17.0331 8.26794 16.8192 8.57341L9.81924 18.5734C9.56894 18.931 9.11564 19.0849 8.69936 18.9537C8.28309 18.8225 8 18.4364 8 18L8 13H4C3.62713 13 3.28522 12.7925 3.11302 12.4618C2.94083 12.131 2.96694 11.732 3.18077 11.4265L10.1808 1.42649C10.4311 1.06892 10.8844 0.914992 11.3006 1.04621Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLink;
impl IconShape for HiLink {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.5858 4.58579C13.3668 3.80474 14.6331 3.80474 15.4142 4.58579C16.1952 5.36683 16.1952 6.63316 15.4142 7.41421L12.4142 10.4142C11.6331 11.1953 10.3668 11.1953 9.58577 10.4142C9.19524 10.0237 8.56208 10.0237 8.17156 10.4142C7.78103 10.8047 7.78103 11.4379 8.17156 11.8284C9.73365 13.3905 12.2663 13.3905 13.8284 11.8284L16.8284 8.82843C18.3905 7.26633 18.3905 4.73367 16.8284 3.17157C15.2663 1.60948 12.7337 1.60948 11.1716 3.17157L9.67156 4.67157C9.28103 5.0621 9.28103 5.69526 9.67156 6.08579C10.0621 6.47631 10.6952 6.47631 11.0858 6.08579L12.5858 4.58579ZM7.58579 9.58579C8.36683 8.80474 9.63316 8.80474 10.4142 9.58579C10.8047 9.97631 11.4379 9.97631 11.8284 9.58579C12.219 9.19526 12.219 8.5621 11.8284 8.17157C10.2663 6.60948 7.73367 6.60948 6.17157 8.17157L3.17157 11.1716C1.60948 12.7337 1.60948 15.2663 3.17157 16.8284C4.73367 18.3905 7.26633 18.3905 8.82843 16.8284L10.3284 15.3284C10.719 14.9379 10.719 14.3047 10.3284 13.9142C9.9379 13.5237 9.30474 13.5237 8.91421 13.9142L7.41421 15.4142C6.63316 16.1953 5.36684 16.1953 4.58579 15.4142C3.80474 14.6332 3.80474 13.3668 4.58579 12.5858L7.58579 9.58579Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLocationMarker;
impl IconShape for HiLocationMarker {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.05025 4.05025C7.78392 1.31658 12.2161 1.31658 14.9497 4.05025C17.6834 6.78392 17.6834 11.2161 14.9497 13.9497L10 18.8995L5.05025 13.9497C2.31658 11.2161 2.31658 6.78392 5.05025 4.05025ZM10 11C11.1046 11 12 10.1046 12 9C12 7.89543 11.1046 7 10 7C8.89543 7 8 7.89543 8 9C8 10.1046 8.89543 11 10 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLockClosed;
impl IconShape for HiLockClosed {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 9V7C5 4.23858 7.23858 2 10 2C12.7614 2 15 4.23858 15 7V9C16.1046 9 17 9.89543 17 11V16C17 17.1046 16.1046 18 15 18H5C3.89543 18 3 17.1046 3 16V11C3 9.89543 3.89543 9 5 9ZM13 7V9H7V7C7 5.34315 8.34315 4 10 4C11.6569 4 13 5.34315 13 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLockOpen;
impl IconShape for HiLockOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 2C7.23858 2 5 4.23858 5 7V9C3.89543 9 3 9.89543 3 11V16C3 17.1046 3.89543 18 5 18H15C16.1046 18 17 17.1046 17 16V11C17 9.89543 16.1046 9 15 9H7V7C7 5.34315 8.34315 4 10 4C11.3965 4 12.5725 4.95512 12.9055 6.24926C13.0432 6.78411 13.5884 7.1061 14.1232 6.96844C14.6581 6.83078 14.9801 6.28559 14.8424 5.75074C14.2874 3.59442 12.3312 2 10 2Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLogin;
impl IconShape for HiLogin {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C3.55229 3 4 3.44771 4 4L4 16C4 16.5523 3.55228 17 3 17C2.44771 17 2 16.5523 2 16L2 4C2 3.44771 2.44772 3 3 3ZM10.7071 6.29289C11.0976 6.68342 11.0976 7.31658 10.7071 7.70711L9.41421 9L17 9C17.5523 9 18 9.44771 18 10C18 10.5523 17.5523 11 17 11L9.41421 11L10.7071 12.2929C11.0976 12.6834 11.0976 13.3166 10.7071 13.7071C10.3166 14.0976 9.68342 14.0976 9.29289 13.7071L6.29289 10.7071C6.10536 10.5196 6 10.2652 6 10C6 9.73478 6.10536 9.48043 6.29289 9.29289L9.29289 6.29289C9.68342 5.90237 10.3166 5.90237 10.7071 6.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLogout;
impl IconShape for HiLogout {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4V16C2 16.5523 2.44772 17 3 17C3.55228 17 4 16.5523 4 16V4C4 3.44772 3.55228 3 3 3ZM13.2929 12.2929C12.9024 12.6834 12.9024 13.3166 13.2929 13.7071C13.6834 14.0976 14.3166 14.0976 14.7071 13.7071L17.7071 10.7071C17.8946 10.5196 18 10.2652 18 10C18 9.73478 17.8946 9.48043 17.7071 9.29289L14.7071 6.29289C14.3166 5.90237 13.6834 5.90237 13.2929 6.29289C12.9024 6.68342 12.9024 7.31658 13.2929 7.70711L14.5858 9L7 9C6.44771 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H14.5858L13.2929 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMailOpen;
impl IconShape for HiMailOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.94 6.4124C2.35524 6.77788 2 7.41882 2 8.1084V15.9999C2 17.1045 2.89543 17.9999 4 17.9999H16C17.1046 17.9999 18 17.1045 18 15.9999V8.1084C18 7.41882 17.6448 6.77788 17.06 6.4124L11.06 2.6624C10.4115 2.25706 9.58854 2.25706 8.94 2.6624L2.94 6.4124ZM5.5547 8.83462C5.09517 8.52826 4.4743 8.65244 4.16795 9.11197C3.8616 9.5715 3.98577 10.1924 4.4453 10.4987L9.4453 13.8321C9.7812 14.056 10.2188 14.056 10.5547 13.8321L15.5547 10.4987C16.0142 10.1924 16.1384 9.5715 15.8321 9.11197C15.5257 8.65244 14.9048 8.52826 14.4453 8.83462L10 11.7981L5.5547 8.83462Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMail;
impl IconShape for HiMail {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.00333 5.88355L9.99995 9.88186L17.9967 5.8835C17.9363 4.83315 17.0655 4 16 4H4C2.93452 4 2.06363 4.83318 2.00333 5.88355Z",
            }
            path {
                d: "M18 8.1179L9.99995 12.1179L2 8.11796V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8.1179Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMap;
impl IconShape for HiMap {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 1.58582L8 5.58582V18.4142L12 14.4142V1.58582Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3.70711 3.29292C3.42111 3.00692 2.99099 2.92137 2.61732 3.07615C2.24364 3.23093 2 3.59557 2 4.00003V14C2 14.2652 2.10536 14.5196 2.29289 14.7071L6 18.4142V5.58582L3.70711 3.29292Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M17.7071 5.29292L14 1.58582V14.4142L16.2929 16.7071C16.5789 16.9931 17.009 17.0787 17.3827 16.9239C17.7564 16.7691 18 16.4045 18 16V6.00003C18 5.73481 17.8946 5.48046 17.7071 5.29292Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3fucPfnBrehD/A5XiZ/z0D3/zrbzoEC+iN32HD31plD+FeTD/k4lbcbzNL/7YN/41/zLEi+Ct3/ujj4+H66cbH+d/AaHdbnP2kJ/+7q/e5YVDvAje4h0/5K2b/VP8LxKF1/n5H/7m3+aFQ7wI3uIdP+Stm/1T/C8Shdf5+R/+5t/mhUO8CN76vT/6+HC4uhU4xv8Ol/rN+YN/+ru/epcXDvEietN3+NCXhvxp4EH8z/YMiLf+xR/7xr/mX4b4V3rrd/7gB0/wYP4HqnDrT//wN9/Kiw7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8a/01u/8wQ9uTQ/iP0gpfsZP//A338p/D8SL6E3f4UNfGuVPYR7MfzDBb3eb87f56e/+6l3+ayFeBG/93h99fDxcP934OP9JBL/9Cz/2za/Dfy3Ei+At3vFD3rrZP8V/sn5zfuKnv/urd/mvg3gRvMU7fshbN/un+E/Wb85P/PR3f/Uu/3UQL4K3fu+PPj4crm4FjvGf53d+8ce++bX5r4V4Eb3pO3zoS0P+NPAg/uP9Tr85f+uf/u6v3uW/FuJf6a3f+YMfPMGD+Q9S5/O//unv/upd/nsg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If6V3vqdP/jBrelB/A9Uip/x0z/8zbfyokO8iN70HT70pVH+FObB/E8mbsXxNr/4Y9/41/zLEC+Ct37vjz4+Hq6fbnyc/wWEdrvN2UN++ru/epcXDvEieIt3/JC3bvZP8b9IFF7n53/4m3+bFw7xIniLd/yQt272T/G/SBRe5+d/+Jt/mxcO8SJ46/f+6OPD4epW4Bj/O1zqN+cP/unv/updXjjEi+hN3+FDXxryp4EH8T/bMyDe+hd/7Bv/mn8Z4l/prd/5gx88wYP5H6jCrT/9w998Ky86xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAUH8w0E+gR/oAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt1;
impl IconShape for HiMenuAlt1 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H10C10.5523 9 11 9.44772 11 10C11 10.5523 10.5523 11 10 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3fucPfnBrehD/A5XiZ/z0D3/zrbzoEC+iN32HD31plD+FeTD/k4lbcbzNL/7YN/41/zLEi+Ct3/ujj4+H66cbH+d/AaHdbnP2kJ/+7q/e5YVDvAje4h0/5K2b/VP8LxKF1/n5H/7m3+aFQ7wI3uIdP+Stm/1T/C8Shdf5+R/+5t/mhUO8CN76vT/6+HC4uhU4xv8Ol/rN+YN/+ru/epcXDvEietN3+NCXhvxp4EH8z/YMiLf+xR/7xr/mX4b4V3rrd/7gB0/wYP4HqnDrT//wN9/Kiw7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8a/01u/8wQ9uTQ/if6BS/Iyf/uFvvpUXHeJF9Kbv8KEvjfKnMA/mfzJxK463+cUf+8a/5l+GeBG89Xt/9PHxcP104+P8LyC0223OHvLT3/3Vu7xwiBfBW7zjh7x1s3+K/0Wi8Do//8Pf/Nu8cIgXwVu844e8dbN/iv9FovA6P//D3/zbvHCIF8Fbv/dHHx8OV7cCx/jf4VK/OX/wT3/3V+/ywiFeRG/6Dh/60pA/DTyI/9meAfHWv/hj3/jX/MsQ/0pv/c4f/OAJHsz/QBVu/ekf/uZbedEh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If6V3vqdP/jBrelB/Acpxc/46R/+5lv574F4Eb3pO3zoS6P8KcyD+Q8m+O1uc/42P/3dX73Lfy3Ei+Ct3/ujj4+H66cbH+c/ieC3f+HHvvl1+K+FeBG8xTt+yFs3+6f4T9Zvzk/89Hd/9S7/dRAvgrd4xw9562b/FP/J+s35iZ/+7q/e5b8O4kXw1u/90ceHw9WtwDH+8/zOL/7YN782/7UQL6I3fYcPfWnInwYexH+83+k352/909/91bv810L8K731O3/wgyd4MP9B6nz+1z/93V+9y38PxP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAXTMw0H1y/1PAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt2;
impl IconShape for HiMenuAlt2 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H10C10.5523 14 11 14.4477 11 15C11 15.5523 10.5523 16 10 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADo0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3fucPfnBrehD/A5XiZ/z0D3/zrbzoEC+iN32HD31plD+FeTD/k4lbcbzNL/7YN/41/zLEi+Ct3/ujj4+H66cbH+d/AaHdbnP2kJ/+7q/e5YVDvAje4h0/5K2b/VP8LxKF1/n5H/7m3+aFQ7wI3uIdP+Stm/1T/C8Shdf5+R/+5t/mhUO8CN76vT/6+HC4uhU4xv8Ol/rN+YN/+ru/epcXDvEietN3+NCXhvxp4EH8z/YMiLf+xR/7xr/mX4b4V3rrd/7gB0/wYP4HqnDrT//wN9/Kiw7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8a/01u/8wQ9uTQ/if6BS/Iyf/uFvvpUXHeJF9Kbv8KEvjfKnMA/mfzJxK463+cUf+8a/5l+GeBG89Xt/9PHxcP104+P8LyC0223OHvLT3/3Vu7xwiBfBW7zjh7x1s3+K/0Wi8Do//8Pf/Nu8cIgXwVu844e8dbN/iv9FovA6P//D3/zbvHCIF8Fbv/dHHx8OV7cCx/jf4VK/OX/wT3/3V+/ywiFeRG/6Dh/60pA/DTyI/9meAfHWv/hj3/jX/MsQ/0pv/c4f/OAJHsz/QBVu/ekf/uZbedEh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If4bvfU7f/CDW9OD+A9Sip/x0z/8zbfyokP8N3jr9/7o4+Ph6qcMr81/NHErjrf5xR/7xr/mX4b4b/Bm7/DBv2V4bf6TCO12m7OH/PR3f/UuLxziv9hbv/MHP3hoPJ3/ZFF4nZ//4W/+bV44xH+xt37nD37w0Hg6/8mi8Do//8Pf/Nu8cIj/Bm/6Dh/828Br8Z/nUr85f/BPf/dX7/LCIf4bvPV7f/Tx4XD108Br8R/vGRBv/Ys/9o1/zb8M8d/ord/7o49Pq9VL8x+kwq0//cPffCsvOsT/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n/jHwFNxsNBxKStOwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt3;
impl IconShape for HiMenuAlt3 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M9 15C9 14.4477 9.44772 14 10 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H10C9.44772 16 9 15.5523 9 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACkUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xL/Cm7/Dh36UybfmfzARP/3zP/aNX8OLBvEierN3+ODfMrw2/xtIP/2LP/pNb8O/DPEiePN3/uDXzsZv8b9IFF7n53/4m3+bFw7xIniLd/yQt272T/G/SJHe5ud+9Jt+mhcO8SJ403f40JeG/Cv+V4mX+cUf+8a/5oVDvIje9B0++KuBj+J/h6/5xR/75o/mX4b4V3jzd/7g187ktfkfLILf/vkf/ubf5kWD+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4h/hTd/5w9+7Uxei//BIvidn//hb/5tXjSIF9GbvsMHfzXwUfzv8DW/+GPf/NH8yxAvgjd9hw99aci/4n+VeJlf/LFv/GteOMSL4C3e8UPeutk/xf8iRXqbn/vRb/ppXjjEi+DN3/mDXzsbv8X/KvEyv/hj3/jXvHCIF9GbvsMH/zbwWvzv8DO/+GPf/Nb8yxD/Cm/2Th/y0U6/Nf+DKfTTv/Aj3/TVvGgQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/8Y/AoQ8XEG8LVyYAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt4;
impl IconShape for HiMenuAlt4 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 7C3 6.44772 3.44772 6 4 6H16C16.5523 6 17 6.44772 17 7C17 7.55228 16.5523 8 16 8H4C3.44772 8 3 7.55228 3 7Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 13C3 12.4477 3.44772 12 4 12H16C16.5523 12 17 12.4477 17 13C17 13.5523 16.5523 14 16 14H4C3.44772 14 3 13.5523 3 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADpUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3fucPfnBrehD/A5XiZ/z0D3/zrbzoEC+iN32HD31plD+FeTD/k4lbcbzNL/7YN/41/zLEi+Ct3/ujj4+H66cbH+d/AaHdbnP2kJ/+7q/e5YVDvAje4h0/5K2b/VP8LxKF1/n5H/7m3+aFQ7wI3uIdP+Stm/1T/C8Shdf5+R/+5t/mhUO8CN76vT/6+HC4uhU4xv8Ol/rN+YN/+ru/epcXDvEietN3+NCXhvxp4EH8z/YMiLf+xR/7xr/mX4b4V3rrd/7gB0/wYP4HqnDrT//wN9/Kiw7x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8a/01u/8wQ9uTQ/if6BS/Iyf/uFvvpUXHeJF9Kbv8KEvjfKnMA/mfzJxK463+cUf+8a/5l+GeBG89Xt/9PHxcP104+P8LyC0223OHvLT3/3Vu7xwiBfBW7zjh7x1s3+K/0Wi8Do//8Pf/Nu8cIgXwVu844e8dbN/iv9FovA6P//D3/zbvHCIF8Fbv/dHHx8OV7cCx/jf4VK/OX/wT3/3V+/ywiFeRG/6Dh/60pA/DTyI/9meAfHWv/hj3/jX/MsQ/0pv/c4f/OAJHsz/QBVu/ekf/uZbedEh/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If6V3vqdP/jBrelB/A9Uip/x0z/8zbfyokO8iN70HT70pVH+FObB/E8mbsXxNr/4Y9/41/zLEC+Ct37vjz4+Hq6fbnyc/wWEdrvN2UN++ru/epcXDvEieIt3/JC3bvZP8b9IFF7n53/4m3+bFw7xIniLd/yQt272T/G/SBRe5+d/+Jt/mxcO8SJ46/f+6OPD4epW4Bj/O1zqN+cP/unv/updXjjEi+hN3+FDXxryp4EH8T/bMyDe+hd/7Bv/mn8Z4l/prd/5gx88wYP5H6jCrT/9w998Ky86xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f+MfAaWXxkGSWNYnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenu;
impl IconShape for HiMenu {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMicrophone;
impl IconShape for HiMicrophone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 4C7 2.34315 8.34315 1 10 1C11.6569 1 13 2.34315 13 4V8C13 9.65685 11.6569 11 10 11C8.34315 11 7 9.65685 7 8V4ZM11 14.9291C14.3923 14.4439 17 11.5265 17 8C17 7.44772 16.5523 7 16 7C15.4477 7 15 7.44772 15 8C15 10.7614 12.7614 13 10 13C7.23858 13 5 10.7614 5 8C5 7.44772 4.55228 7 4 7C3.44772 7 3 7.44772 3 8C3 11.5265 5.60771 14.4439 9 14.9291V17H6C5.44772 17 5 17.4477 5 18C5 18.5523 5.44772 19 6 19H14C14.5523 19 15 18.5523 15 18C15 17.4477 14.5523 17 14 17H11V14.9291Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinusCircle;
impl IconShape for HiMinusCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACJElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/IK/91u99fH3p0kvxX2B27Njf/PZPf/cu/36If6fXfuv3Pr7c2/su7Lfmv5L004udnff57Z/+7l3+7RD/Tq/0um/z1ZiP4r+D+Jo/+c2f+mj+7RD/Tq/0Om9zETjOf49b/+S3fuoh/Nsh/p1e6XXeZhc4xn8DwTP++Ld+6sH82yH+nV7pdd/mqzEfxX8H8TV/8ps/9dH82yH+nV77rd/7+PLSpe8G3or/Wj+zOHbsvX/7p797l387xH+Q137r9z6+Ojh4af4LzLe2/vq3f/q7d/n3Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N/4RUCA6QUtK6jsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinusSm;
impl IconShape for HiMinusSm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 10C5 9.44772 5.44772 9 6 9L14 9C14.5523 9 15 9.44772 15 10C15 10.5523 14.5523 11 14 11L6 11C5.44772 11 5 10.5523 5 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACLklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4h/pVd+43d+sNbrB/E/kGezZ/zxL//wrbzoEC+iV32Dd3jpNk0/BTyY/9luLbW+zR/+2o/9Nf8yxIvgtd/6vY8vL116OnCc/x12F8eOPeS3f/q7d3nhEC+CV3m9t3vrzPwp/hdRKa/zx7/+47/NC4d4EbzK673dW2fmT/G/iEp5nT/+9R//bV44xIvgtd/6vY8vL126FTjG/w6XFseOPfi3f/q7d3nhEC+iV32Dd3jpnKafNjyI/8EEz4ha3/oPf+3H/pp/GeJf6ZXf+J0fzDQ9mP+Jar31j3/5h2/lRYf4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I/ouREHoFrozAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinus;
impl IconShape for HiMinus {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9L16 9C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11L4 11C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMoon;
impl IconShape for HiMoon {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.2929 13.2929C16.2886 13.7471 15.1738 13.9999 14 13.9999C9.58172 13.9999 6 10.4182 6 5.9999C6 4.82593 6.25287 3.71102 6.70712 2.70667C3.93137 3.96191 2 6.75526 2 9.9997C2 14.418 5.58172 17.9997 10 17.9997C13.2443 17.9997 16.0376 16.0685 17.2929 13.2929Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMusicNote;
impl IconShape for HiMusicNote {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 3.00001C18 2.70042 17.8657 2.41661 17.634 2.22667C17.4023 2.03673 17.0977 1.96067 16.8039 2.01943L6.80388 4.01943C6.33646 4.11291 6 4.52333 6 5.00001V14.1138C5.68722 14.0401 5.35064 14 5 14C3.34315 14 2 14.8954 2 16C2 17.1046 3.34315 18 5 18C6.65685 18 7.99999 17.1046 8 16V7.81981L16 6.21981V12.1138C15.6872 12.0401 15.3506 12 15 12C13.3431 12 12 12.8954 12 14C12 15.1046 13.3431 16 15 16C16.6569 16 18 15.1046 18 14V3.00001Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiNewspaper;
impl IconShape for HiNewspaper {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H12C13.1046 3 14 3.89543 14 5V15C14 16.1046 14.8954 17 16 17H4C2.89543 17 2 16.1046 2 15V5ZM5 6H11V10H5V6ZM11 12H5V14H11V12Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M15 7H16C17.1046 7 18 7.89543 18 9V14.5C18 15.3284 17.3284 16 16.5 16C15.6716 16 15 15.3284 15 14.5V7Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiOfficeBuilding;
impl IconShape for HiOfficeBuilding {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H14C15.1046 2 16 2.89543 16 4V16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H13C12.4477 18 12 17.5523 12 17V15C12 14.4477 11.5523 14 11 14H9C8.44772 14 8 14.4477 8 15V17C8 17.5523 7.55228 18 7 18H4C3.44772 18 3 17.5523 3 17C3 16.4477 3.44772 16 4 16V4ZM7 5H9V7H7V5ZM9 9H7V11H9V9ZM11 5H13V7H11V5ZM13 9H11V11H13V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPaperAirplane;
impl IconShape for HiPaperAirplane {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.8944 2.55279C10.725 2.214 10.3788 2 10 2C9.62124 2 9.27498 2.214 9.10558 2.55279L2.10558 16.5528C1.92823 16.9075 1.97724 17.3335 2.2305 17.6386C2.48376 17.9438 2.89342 18.0705 3.27473 17.9615L8.27472 16.533C8.70402 16.4103 9 16.0179 9 15.5714V11C9 10.4477 9.44772 10 10 10C10.5523 10 11 10.4477 11 11V15.5714C11 16.0179 11.296 16.4103 11.7253 16.533L16.7253 17.9615C17.1066 18.0705 17.5163 17.9438 17.7695 17.6386C18.0228 17.3335 18.0718 16.9075 17.8944 16.5528L10.8944 2.55279Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPaperClip;
impl IconShape for HiPaperClip {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C6.34315 4 5 5.34315 5 7V11C5 13.7614 7.23858 16 10 16C12.7614 16 15 13.7614 15 11V7C15 6.44772 15.4477 6 16 6C16.5523 6 17 6.44772 17 7V11C17 14.866 13.866 18 10 18C6.13401 18 3 14.866 3 11V7C3 4.23858 5.23858 2 8 2C10.7614 2 13 4.23858 13 7V11C13 12.6569 11.6569 14 10 14C8.34315 14 7 12.6569 7 11V7C7 6.44772 7.44772 6 8 6C8.55228 6 9 6.44772 9 7V11C9 11.5523 9.44772 12 10 12C10.5523 12 11 11.5523 11 11V7C11 5.34315 9.65685 4 8 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPause;
impl IconShape for HiPause {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM7 8C7 7.44772 7.44772 7 8 7C8.55228 7 9 7.44772 9 8V12C9 12.5523 8.55228 13 8 13C7.44772 13 7 12.5523 7 12V8ZM12 7C11.4477 7 11 7.44772 11 8V12C11 12.5523 11.4477 13 12 13C12.5523 13 13 12.5523 13 12V8C13 7.44772 12.5523 7 12 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAJd0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+l3vr9/7o4+PR+q2MHwwQjt1a8md++oe/+Vb+ZYj/xd70HT74vYW+yvg4z+u7+835x/z0d3/1Li8Y4n+pN32HD/4u4L15ofTX/ebsdX76u796l+cP8b/Qm77DB38X8N68CCS+5xd+9Jvfm+cP8b/Mm77DB38X8N78K/SFh/z0D3/zrTwvxP8ib/oOH/xdwHvzr6TQx/zCj3zTV/O8EP9Kb/EOH/Ja/Bv83I990+/w7/Cm7/DB3wW8N/8W4nN+8Ue/+bN5XogXwZu/8we/thufZXht/j3EZ//ij37z5/Cv9Kbv8MHfBbw3/1bic37xR7/5s3leiH/Bm77jB38W5rP5D1Kkt/m5H/2mn+ZF9Kbv8MHfBbw3/w5Fepuf+9Fv+mmeF+KFeNN3+OD3Br6L/1g/84s/9s1vzYvgTd/hg78LeG/+fS71m/MH//R3f/UuzwvxArz1e3/08fFw/XTj4/zH+plf/LFvfmv+BW/6Dh/8XcB78+/3Pr/4Y9/83Tx/iBfgTd/hg98b+C7+gxXpbX7uR7/pp3kh3vQdPvi7gPfm30nie37hR7/5vXnBEC/Am77jB3825rP4jyQ+5xd/9Js/mxfiTd/hg78LeG/+nSS+5xd+9JvfmxcO8QK86Tt+8GdjPovnIvE3Cj6af6Wf/+Fv/m3+BW/6Dh/8XcB78+8k8T2/8KPf/N78yxAvwJu+4wd/NuazeF6/84s/9s2vzX+wN32HD/4u4L35d5L4nl/40W9+b140iBfgTd/xgz8b81k8r9/5xR/75tfmP9CbvsMHfxfw3vw7SXzPL/zoN783LzrEC/Cm7/jBn435LJ7X7/zij33za/Mf5E3f4YO/C3hv/p0kvucXfvSb35t/HcQL8Kbv+MGfjfksntfv/OKPffNr8x/gTd/hg78LeG/+nSS+5xd+9Jvfm389xAvwpu/4wZ+N+Sye1+/84o9982vz7/Sm7/DB3wW8N/9OEt/zCz/6ze/Nvw3iBXjTd/zgz8Z8Fs/rd37xx775tfl3eNN3+ODvAt6bfyeJ7/mFH/3m9+bfDvECvOk7fvBnYz6L5/U7v/hj3/za/Bu96Tt88HcB782/k8T3/MKPfvN78++DeAHe9B0/+LMxn8Xz+p1f/LFvfm3+Dd70HT74u4D35t9J4nt+4Ue/+b3590O8AG/6jh/82ZjP4nn9zi/+2De/Nv9Kb/oOH/xdwHvz7yTxPb/wo9/83vzHQLwAb/qOH/zZmM/ief3OL/7YN782/wpv+g4f/F3Ae/PvJPE9v/Cj3/ze/MdBvABv+o4f/NmYz+J5/c4v/tg3vzYvojd9hw/+LuC9+XeS+J5f+NFvfm/+YyFegDd9xw/+bMxn8bx+5xd/7JtfmxfBm77DB38X8N78O0l8zy/86De/N//xEC/Am77jB3825rN4Xr/ziz/2za/Nv+DN3ulDPtrpr+LfSeJ7fuFHv/m9+c+BeAHe9B0/+LMxn8Xz+p1f/LFvfm3+BW/6jh/82ZjP4t9B4nt+4Ue/+b35z4N4Ad70HT/4szGfxfP6nV/8sW9+bf4Fb/qOH/zZmM/i30jie37hR7/5vfnPhXgB3vQdP/izMZ/F8/qdX/yxb35t/gVv+o4f/NmYz+LfQOJ7fuFHv/m9eSHe7B0/5L2w39vSruCnf+FHv+l7+NdDvABv+o4f/NmYz+J5/c4v/tg3vzb/gjd9xw/+bMxn8a8k8T2/8KPf/N68EG/6jh/82ZjP4oHE5/zij37zZ/Ovg3gB3vQdP/izMZ/F8/qdX/yxb35t/gVv+o4f/NmYz+JfQeJ7fuFHv/m9+Re86Tt8sHk+fvHHvln86yBegDd9xw/+bMxn8bx+5xd/7Jtfm3/Bm77jB3825rN4EUl8zy/86De/N/+CN3/nD37tbPwWz0cUXufnf/ibf5sXHeIFeNN3/ODPxnwWz+t3fvHHvvm1+Re86Tt+8GdjPosXgcT3/MKPfvN78yJ483f+4NfOxm/xfEThdX7+h7/5t3nRIV6AN33HD/5szGfxvH7nF3/sm1+bf8GbvuMHfzbms/gXSHzPL/zoN783L6I3f+cPfu1s/BbPRxRe5+d/+Jt/mxcd4gV403f84M/GfBbP63d+8ce++bX5F7z5O3/wa2fy2rwQknZ/4Ue+6av5V3jzd/7g187Gb/F8ROF1fv6Hv/m3edEhXoA3fccP/mzMZ/G8fucXf+ybX5v/Jm/+zh/82tn4LZ6PKLzOz//wN/82LzrEC/Cm7/jBn435LJ7X7/zij33za/Pf5M3f+YNfOxu/xfMRhdf5+R/+5t/mRYd4Ad70HT/4szGfxfP6nV/8sW9+bf6bvPk7f/BrZ+O3eD6i8Do//8Pf/Nu86BAvwJu+4wd/NuazeF6/84s/9s2vzX+TN3/nD37tbPwWz0cUXufnf/ibf5sXHeIFeNN3/ODPxnwWz+t3fvHHvvm1+W/y5u/8wa+djd/i+YjC6/z8D3/zb/OiQ7wAb/qOH/zZmM/ief3OL/7YN782/03e/J0/+LWz8Vs8H1F4nZ//4W/+bV50iBfgTd/xgz8b81k8r9/5xR/75tfmv8mbv/MHv3Y2fovnIwqv8/M//M2/zYsO8QK86Tt+8GdjPovn9Tu/+GPf/Nr8N3nzd/7g187Gb/F8ROF1fv6Hv/m3edEhXoA3fccP/mzMZ/G8fucXf+ybX5v/Jm/+zh/82tn4LZ6PKLzOz//wN/82LzrEC/Cm7/jBn435LJ6L0C74r/lvYnQc/NI8H1F4nZ//4W/+bV50iBfgzd7pQz7a6a/if5Ff/LFvFv86iBfgrd/5gx88NJ7O/x4/84s/9s1vzb8O4oV4s3f84O+2eS/+F4jC6/z8D3/zb/Ovg3gh3vq9P/r4eLT6bZuX4n8y8Tm/+KPf/Nn86yH+BW/93h99fDxafbXNe/E/zyXgo3/xx775u/m3QbyI3vqdP/jBo/XWto/zP4G5td+c//RPf/dX7/Jvh/j/DfH/G+L/N8T/b4j/3xD/v/GPPllqX+AmY1MAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPencilAlt;
impl IconShape for HiPencilAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.4142 2.58579C16.6332 1.80474 15.3668 1.80474 14.5858 2.58579L7 10.1716V13H9.82842L17.4142 5.41421C18.1953 4.63316 18.1953 3.36683 17.4142 2.58579Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 6C2 4.89543 2.89543 4 4 4H8C8.55228 4 9 4.44772 9 5C9 5.55228 8.55228 6 8 6H4V16H14V12C14 11.4477 14.4477 11 15 11C15.5523 11 16 11.4477 16 12V16C16 17.1046 15.1046 18 14 18H4C2.89543 18 2 17.1046 2 16V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xv9SbvsOHvjShBwGQfsYv/tg3/jX/eoj/Zd7iHT/krRv+KsyDeSBxa0Ef83M/+k0/zYsO8b/Im73jh3yV7Y/mhRGf84s/+s2fzYsG8b/Em77DB38X8N68CIr0Nj/3o9/00/zLEP8LvOk7fPB3Ae/Ni0rc+os/+s0P4V+G+F/gzd7xg7/b5r34V4mX+cUf+8a/5oVD/C/xZu/4wd9t8168iIr0Nj/3o9/007xwiP9F3uwdP/i7bd6LF0GR3ubnfvSbfpoXDvG/zJu94wd/t8178S+Kl/nFH/vGv+aFQ/wP8+bv/MGv/fM//M2/zQvxZu/4wd9t8168YM/4xR/75gfzL0P8D/Km7/DB3wW8N/A+v/hj3/zdvBBv9o4f/N0278XzUaS3+bkf/aaf5l+G+B/iTd/hg78LeG+e7X1+8ce++bt5Id7sHT/4u23ei+f0Nb/4Y9/80bxoEP8DvOk7fPB3Ae/N83qfX/yxb/5uXog3e8cP/m6b9wKQ+J5f+NFvfm9edIj/Zm/6Dh/8XcB784K9zy/+2Dd/Ny/Em73jB383wC/86De/N/86iP9Gb/oOH/xdwHvzL3ufX/yxb/5u/uMh/pu86Tt88HcB782L7n1+8ce++bv5j4X4b/Cm7/DB3wW8N/967/OLP/bN381/HMR/sTd9hw/+LuC9+bd7n1/8sW/+bv5jIP4Lvek7fPB3Ae/Nv1MUXufnf/ibf5t/P8R/kTd9hw/+LuC9+XeS+J5f+NFvfm/+YyD+C7zpO3zwdwHvzb+TxPf8wo9+83vzHwfxn+xN3+GDvwt4b/6dJL7nF370m9+b/1iI/0Rv+g4f/F3Ae/PvJPE9v/Cj3/ze/MdD/Cd503f44O8C3pt/J4nv+YUf/eb35j8H4j/Bm77DB38X8N78O0l8zy/86De/N/95EP/B3vQdPvi7gPfm30nie37hR7/5vfnPhfgP9Kbv8MHfBbw3/04S3/MLP/rN781/PsR/kDd9hw/+LuC9+XeS+J5f+NFvfm/+ayD+A7zpO3zwdwHvzb+TxPf8wo9+83vzXwfx7/Sm7/DB3wW8N/9OEt/zCz/6ze/Nfy3Ev8ObvsMHfxfw3vw7SXzPL/zoN783//UQ/0Zv+g4f/F3Ae/PvJPE9v/Cj3/ze/PdA/Bu86Tt88HcB782/k8T3/MKPfvN7898H8a/0pu/wwd8FvDf/ThLf8ws/+s3vzX8vxL/Cm77DB38X8N78O0l8zy/86De/N//9EC+iN32HD/4u4L35d5L4nl/40W9+b/5nQLwI3uydPuSjnf4q/p0kvucXfvSb35v/ORAvgjd9xw/+bMxn8e8g8T2/8KPf/N78z4J4EbzpO37wZ2M+i38jie/5hR/95vfmfx7Ei+BN3/GDPxvzWfwbSHzPL/zoN783/zMhXgRv+o4f/NmYz+JfSeJ7fuFHv/m9+Z8L8SJ403f84M/GfBb/ChLf8ws/+s3vzf9siBfBm77jB3825rN4EUl8zy/86De/N//zIV4Eb/qOH/zZmM/iRSDxPb/wo9/83vzvgHgRvOk7fvBnYz6Lf4HE9/zCj37ze/O/B+JF8Obv/MGvnclr80JI2v2FH/mmr+Z/F8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I28EI1Cg9WfSAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPencil;
impl IconShape for HiPencil {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5858 3.58579C14.3668 2.80474 15.6332 2.80474 16.4142 3.58579C17.1953 4.36683 17.1953 5.63316 16.4142 6.41421L15.6213 7.20711L12.7929 4.37868L13.5858 3.58579Z",
            }
            path {
                d: "M11.3787 5.79289L3 14.1716V17H5.82842L14.2071 8.62132L11.3787 5.79289Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneIncoming;
impl IconShape for HiPhoneIncoming {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.4142 7L17.7071 3.70711C18.0976 3.31658 18.0976 2.68342 17.7071 2.29289C17.3166 1.90237 16.6834 1.90237 16.2929 2.29289L13 5.58579V4C13 3.44772 12.5523 3 12 3C11.4477 3 11 3.44772 11 4V7.99931C11 8.00031 11 8.002 11 8.003C11.0004 8.1375 11.0273 8.26575 11.0759 8.38278C11.1236 8.49805 11.1937 8.6062 11.2864 8.70055C11.2907 8.70494 11.2951 8.70929 11.2995 8.7136C11.3938 8.80626 11.502 8.87643 11.6172 8.92412C11.7351 8.97301 11.8644 9 12 9H16C16.5523 9 17 8.55228 17 8C17 7.44772 16.5523 7 16 7H14.4142Z",
            }
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneMissedCall;
impl IconShape for HiPhoneMissedCall {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
            path {
                d: "M16.7071 3.29289C17.0976 3.68342 17.0976 4.31658 16.7071 4.70711L15.4142 6L16.7071 7.29289C17.0976 7.68342 17.0976 8.31658 16.7071 8.70711C16.3166 9.09763 15.6834 9.09763 15.2929 8.70711L14 7.41421L12.7071 8.70711C12.3166 9.09763 11.6834 9.09763 11.2929 8.70711C10.9024 8.31658 10.9024 7.68342 11.2929 7.29289L12.5858 6L11.2929 4.70711C10.9024 4.31658 10.9024 3.68342 11.2929 3.29289C11.6834 2.90237 12.3166 2.90237 12.7071 3.29289L14 4.58579L15.2929 3.29289C15.6834 2.90237 16.3166 2.90237 16.7071 3.29289Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneOutgoing;
impl IconShape for HiPhoneOutgoing {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.9241 2.61722C17.8757 2.50014 17.804 2.3904 17.7092 2.29502C17.7078 2.2936 17.7064 2.29219 17.705 2.29078C17.5242 2.11106 17.2751 2 17 2H13C12.4477 2 12 2.44772 12 3C12 3.55228 12.4477 4 13 4H14.5858L11.2929 7.29289C10.9024 7.68342 10.9024 8.31658 11.2929 8.70711C11.6834 9.09763 12.3166 9.09763 12.7071 8.70711L16 5.41421V7C16 7.55228 16.4477 8 17 8C17.5523 8 18 7.55228 18 7V3C18 2.86441 17.973 2.73512 17.9241 2.61722Z",
            }
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhone;
impl IconShape for HiPhone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhotograph;
impl IconShape for HiPhotograph {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3C2.89543 3 2 3.89543 2 5V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V5C18 3.89543 17.1046 3 16 3H4ZM16 15H4L8 7L11 13L13 9L16 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlay;
impl IconShape for HiPlay {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM9.5547 7.16795C9.24784 6.96338 8.8533 6.94431 8.52814 7.11833C8.20298 7.29235 8 7.63121 8 8V12C8 12.3688 8.20298 12.7077 8.52814 12.8817C8.8533 13.0557 9.24784 13.0366 9.5547 12.8321L12.5547 10.8321C12.8329 10.6466 13 10.3344 13 10C13 9.66565 12.8329 9.35342 12.5547 9.16795L9.5547 7.16795Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlusCircle;
impl IconShape for HiPlusCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 7C11 6.44772 10.5523 6 10 6C9.44772 6 9 6.44772 9 7V9H7C6.44772 9 6 9.44771 6 10C6 10.5523 6.44772 11 7 11H9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H11V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+G/w2m/93seXe3ufhf3WAEg/vdjZ+Zzf/unv3uW/FuK/wSu97tv+FPZb80DST//Jb/7k2/BfC/Ff7LXf+r2PLy9dusjzsTh27MRv//R37/JfB/Ff7JVf/+1f2639Fs+HSnmdP/71H/9t/usg/ou98uu//Wu7td/i+VApr/PHv/7jv81/HcR/sVd+/bd/bbf2WzwfKuV1/vjXf/y3+a+D+C/2yq//9q/t1n6L50OlvM4f//qP/zb/dRD/xV759d/+td3ab/F8qJTX+eNf//Hf5r8O4r/YK7/+27+2W/stng+V8jp//Os//tv810H8F3vl13/713Zrv8XzoVJe549//cd/m/86iP9ir/z6b//abu23eD5Uyuv88a//+G/zXwfxX+yVX//tX9ut/RbPh0p5nT/+9R//bf7rIP6DvPZbv/fx9aVLL8W/ICNemsyv5vmJ+OjI/Gv+BbNjx/7mt3/6u3f590P8O732W7/38eXe3ndhvzX/laSfXuzsvM9v//R37/Jvh/h3eqXXfZuvxnwU/x3E1/zJb/7UR/Nvh/h3eqXXeZuLwHH+e9z6J7/1Uw/h3w7x7/RKr/M2u8Ax/hsInvHHv/VTD+bfDvHv9Eqv+zZfjfko/juIr/mT3/ypj+bfDvHv9Npv/d7Hl5cufTfwVvzX+pnFsWPv/ds//d27/Nsh/oO89lu/9/HVwcFL8y9Q5kun/VU8HyF9jCP+mn/BfGvrr3/7p797l38/xH+xV379t39tt/ZbPB8q5XX++Nd//Lf5r4P4L/bKr//2r+3WfovnQ6W8zh//+o//Nv91EP/FXvn13/613dpv8XyolNf541//8d/mvw7iv9grv/7bv7Zb+y2eD5XyOn/86z/+2/zXQfwXe+XXf/vXdmu/xfOhUl7nj3/9x3+b/zqI/2Kv/Ppv/9pu7bd4PlTK6/zxr//4b/NfB/Ff7JVf/+1f2639Fs+HSnmdP/71H/9t/usg/ou98uu//Wu7td/i+VApr/PHv/7jv81/HcR/sVd+/bd/bbf2WzwfKuV1/vjXf/y3+a+D+C/22m/93seXly5d5PlYHDt24rd/+rt3+a+D+G/wSq/zNj8NvBXP6Wf+5Ld+6q35r4X4b/Dab/3ex5d7lz5b5q0BLH56sXPss3/7p797l/9aiP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2/8I+fcC1BDdOLhAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlusSm;
impl IconShape for HiPlusSm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 5C10.5523 5 11 5.44772 11 6V9L14 9C14.5523 9 15 9.44772 15 10C15 10.5523 14.5523 11 14 11H11V14C11 14.5523 10.5523 15 10 15C9.44771 15 9 14.5523 9 14V11H6C5.44772 11 5 10.5523 5 10C5 9.44771 5.44772 9 6 9L9 9V6C9 5.44772 9.44771 5 10 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFR0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+TBD37p4/vD8FoA233/O7fe+te7/NdD/Dc4c+OLvfSU029hjgMgdmvU1zl75z/8Nf+1EP8NTlz3qKcDD+Y53Xrxnic+hP9aiP9i11336Aev8dN5PmboIffc84Rb+a+D+C92+rpHv3bDv8XzUdDrnLvnCb/Nfx3Ef7HT1z36tRv+LZ6Pgl7n3D1P+G3+6yD+i52+7tGv3fBv8XwU9Drn7nnCb/NfB/Ff7PR1j37thn+L56Og1zl3zxN+m/86iP9ip6979Gs3/Fs8HwW9zrl7nvDb/NdB/Bc7fd2jX7vh3+L5KOh1zt3zhN/mvw7iv9jp6x792g3/Fs9HQa9z7p4n/Db/dRD/xU5f9+jXbvi3eD4Kep1z9zzht/mvg/gvdvq6R792w7/F81HQ65y75wm/zX8dxH+x09c9+rUb/i2ej4Je59w9T/ht/usg/oudvu7Rr93wb/F8FPQ65+55wm/zXwfxX+z0dY9+7YZ/i+ejoNc5d88Tfpv/Ooj/Yqeve/RrN/xbPB8Fvc65e57w2/zXQfwXO33do1+74d/i+Sjodc7d84Tf5r8O4r/Y6ese/doN/xbPR0Gvc+6eJ/w2/3UQ/0rXXffoB0/wIP6NUn5pm6/m+ZD46LD+mn+jCs+4554n3MqLDvEiOnPji7301KafAh7M/2y31lLf5uyd//DX/MsQL4IHP/ilj19aL5+OOc7/BmL32GzxkFtv/etdXjjEi+DUDY9568z8Kf4XKeh1zt3zhN/mhUO8CE7d8Ji3zsyf4n+Rgl7n3D1P+G1eOMSL4MEPfunjl1bLW4Fj/O9w6dh88eBbb/3rXV44xIvozI0v9tJTaz8NfhD/o+kZtZS3PnvnP/w1/zLEv9J11z36wRM8mH+jlF7azq/i+ZDiY8L+a/6NKtx6zz1PuJUXHeK/2OnrHv3aDf8Wz0dBr3Punif8Nv91EP/FTl/36Ndu+Ld4Pgp6nXP3POG3+a+D+C92+rpHv3bDv8XzUdDrnLvnCb/Nfx3Ef7HT1z36tRv+LZ6Pgl7n3D1P+G3+6yD+i52+7tGv3fBv8XwU9Drn7nnCb/NfB/Ff7PR1j37thn+L56Og1zl3zxN+m/86iP9ip6979Gs3/Fs8HwW9zrl7nvDb/NdB/Bc7fd2jX7vh3+L5KOh1zt3zhN/mvw7iv9jp6x792g3/Fs9HQa9z7p4n/Db/dRD/xU5f9+jXbvi3eD4Kep1z9zzht/mvg/gvdvq6R792w7/F81HQ65y75wm/zX8dxH+x09c9+rUb/i2ej4Je59w9T/ht/usg/oudvu7Rr93wb/F8FPQ65+55wm/zXwfxX+z0dY9+7YZ/i+ejoNc5d88Tfpv/Ooj/Yqeve/RrN/xbPB8Fvc65e57w2/zXQfwXu+66Rz94jZ/O8zFDD7nnnifcyn8dxH+DE9c9+lbwg3gOesbFe57wYP5rIf4bnLnxxV56atNvA8e44lIt9bXP3vkPf81/LcR/kwc/+KWPH6xWLw2wNZ//9a23/vUu//UQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/8Y/Y9F9QkTSqyAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlus;
impl IconShape for HiPlus {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 3C10.5523 3 11 3.44772 11 4V9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H11V16C11 16.5523 10.5523 17 10 17C9.44772 17 9 16.5523 9 16V11H4C3.44772 11 3 10.5523 3 10C3 9.44771 3.44772 9 4 9L9 9V4C9 3.44772 9.44772 3 10 3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPresentationChartBar;
impl IconShape for HiPresentationChartBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5V13C3 14.1046 3.89543 15 5 15H7.58579L6.29289 16.2929C5.90237 16.6834 5.90237 17.3166 6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071L10 15.4142L12.2929 17.7071C12.6834 18.0976 13.3166 18.0976 13.7071 17.7071C14.0976 17.3166 14.0976 16.6834 13.7071 16.2929L12.4142 15H15C16.1046 15 17 14.1046 17 13V5C17.5523 5 18 4.55228 18 4C18 3.44772 17.5523 3 17 3H3ZM14 7C14 6.44772 13.5523 6 13 6C12.4477 6 12 6.44772 12 7V11C12 11.5523 12.4477 12 13 12C13.5523 12 14 11.5523 14 11V7ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V11C9 11.5523 9.44772 12 10 12C10.5523 12 11 11.5523 11 11V8ZM8 9C8 8.44772 7.55228 8 7 8C6.44772 8 6 8.44772 6 9V11C6 11.5523 6.44772 12 7 12C7.55228 12 8 11.5523 8 11V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAJaElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xL3jVN3iHl87Ml8J+MP+bSLdGxN/84a/92F/zgiFeiFd63bf9KuyP5n8z6av/5Dd/8mN4/hAvwCu//tu/tlv7Lf4PUCmv88e//uO/zfNCvACv/Lpv+9m2P4v/AyR9zh//5k9+Ns8L8QK88uu+7Wfb/iz+DwjpY/7oN3/yq3leiBfgld/4nR/s9fqvgWP873ZpcezYg3/7p797l+eFeCFe9Q3e4aXbNH028Fb87/QzpdbP/sNf+7G/5vlDvAhe+XXf9rNtfxb/i0j6nD/+zZ/8bF44xIvglV/3bT/b9mfxv4ikz/nj3/zJz+aFQ7wIXvl13/azbX8W/4tI+pw//s2f/GxeOMSL4JVf920/2/Zn8d9NfI2i/LTs4878aMNr8QJI+pw//s2f/GxeOMSL4JVf920/2/Zn8d/nUqn1tf/w137sr3mAV3qdt/lp4K14PiR9zh//5k9+Ni8c4kXwyq/7tp9t+7P473Gp1Praf/hrP/bXPJdXfYN3eOk2TX/F8yHpc/74N3/ys3nhEC+CV37dt/1s25/Ff71LpdbX/sNf+7G/5gV4pdd5G/N8SPqcP/7Nn/xsXjjEi+CVX/dtP9v2Z/Ff61Kp9bX/8Nd+7K95AV7l9d7urTPzp3g+JH3OH//mT342LxziRfDKr/u2n237s/ivc6nU+tp/+Gs/9te8AK/91u99fHnp0m8BL83zIelz/vg3f/KzeeEQL4JXft23/Wzbn8W/zd8AtwIPBl6Kf9mlUutr/+Gv/dhf8wK89lu/9/HlpUu/Bbw0L4Ckz/nj3/zJz+aFQ7wIXvl13/azbX8W/0ohfcwf/eZPfjXP9Cqv93ZvnZnfDRzj+btUan3tP/y1H/trXoDXfuv3Pr68dOm3gJfmhZD0OX/8mz/52bxwiBfBK7/u23627c/iX0HS+/zxb/7kd/NcXvUN3uGl2zT9NnCM53Sp1Praf/hrP/bXvACv/dbvfXx56dJvAS/Nv0DS5/zxb/7kZ/PCIV4Er/y6b/vZtj+LF5Gk9/nj3/zJ7+YFeNU3eIeXbtP028AxrrhUan3tP/y1H/trXoDXfuv3Pr68dOm3gJfmRSDpc/74N3/ys3nhEC+CV37dt/1s25/Fi0DS+/zxb/7kd/MveNU3eIeXbtP02wCl1tf+w1/7sb/mBXjtt37v48tLl34LeGleRJI+549/8yc/mxcO8SJ45dd928+2/Vn8CyS9zx//5k9+Ny+iV32Dd3hpgD/8tR/7a16A137r9z6+vHTpt4CX5l9B0uf88W/+5GfzwiFeBK/8um/72bY/ixdC0vv88W/+5HfzH+i13/q9jy8vXfot4KX5V5L0OX/8mz/52bxwiBfBK7/u23627c/iBZD0Pn/8mz/53fwHeu23fu/jy0uXfgt4af4NJH3OH//mT342LxziRfDKr/u2n237s3h+xNf8yW/+1EfzQrzy677tZ813dr7mt3/6u3d5Ebz2W7/38eWlS78FvDT/RpI+549/8yc/mxcO8SJ45dd928+2/Vk8H5rNHvLHv/zDt/ICvPLrvu132X5v4K8Xx469zm//9Hfv8kK89lu/9/HlpUu/Bbw0/w6SPuePf/MnP5sXDvEieOXXfdvPtv1ZPB9/8ls/JV6AV37dt/0u2+/Ns/314tix1/ntn/7uXZ6P137r9z6+vHTpt4CX5t9J0uf88W/+5GfzwiFeBK/8um/72bY/i+dD0vv88W/+5HfzXF75dd/2u2y/N8/rrxfHjr3Ob//0d+/yAK/91u99fHnp0m8BL81/AEmf88e/+ZOfzQuHeBG88uu+7Wfb/iyev91S6+v84a/92F8DvPZbv/fx1d7eV9l+b16wv14cO/Y6v/3T370L8Npv/d7Hl5cu/Rbw0vwHkfQ5f/ybP/nZvHCIF8Erv+7bfrbtz+KFEPy2pV3slwYezL9sV/DXAIaXBo7zH0jS5/zxb/7kZ/PCIV4Er/y6b/vZtj+L/0Ukfc4f/+ZPfjYvHOJF8Mqv+7afbfuz+F9E0uf88W/+5GfzwiFeBK/8um/72bY/i/9FQvqYP/rNn/xqXjjEi+CVX//tX9ut/Rb/i6iU1/njX//x3+aFQ7wIXvmN3/nBXq+fzv8ii2PHTvz2T3/3Li8c4kX0Sq/zNj8NvBX/Cwi+549/66fem38Z4kX0ym/8zg/2ev3XwDH+Z7uk2eyl//iXf/hW/mWIf4VXfYN3eOmcpp82PIj/gQTPiFrf+g9/7cf+mhcN4l/ptd/6vY+v9/beO+33Bl6K/xn+JqTvnu3sfPdv//R37/KiQ/wnetU3eIeXbtP0W8Bx/m12S62v84e/9mN/zX8OxH+SV32Dd3jpNk2/BRznBfsbrngpXrDdUuvr/OGv/dhf8x8P8Z/gVd/gHV66TdNvAcd5wf5mcezYawMsL136beCleMF2S62v84e/9mN/zX8sxH+wV32Dd3jpNk2/BRznBfubxbFjr/3bP/3duwCv/dbvfXx56dJvAy/FC7Zban2dP/y1H/tr/uMg/gO98hu/84O9Xv8VcJwX7G8Wx4699m//9Hfv8gCv/dbvfXx56dJvAy/FC7ar2exl/viXf/hW/mMg/gO98uu8zXcb3osX7G8Wx4699m//9Hfv8ny89lu/9/HlpUu/DbwUL4Dge/74t37qvfmPgfgP9Mqv8za/bXgtnr+/WRw79tq//dPfvcsL8dpv/d7Hl5cu/TbwUjwfgt/549/6qdfmPwbiP9Arve7bfDXmo3hef7M4duy1f/unv3uXF8Frv/V7H19euvTbwEvx3MTX/Mlv/tRH8x8D8R/otd/6vY+vLl36a8ODeCbB78yPHXvr3/7p797lX+G13/q9j68uXfppw2vxTIJnzI8de+nf/unv3uU/BuI/2Gu/9XsfX+/tvbfhOHDrH//mT343/w6v/Lpv+97AgwW7s52d7/7tn/7uXf7jIP5/Q/z/hvj/DfH/G+L/N8T/b/wjxtjLX1rzs9kAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPresentationChartLine;
impl IconShape for HiPresentationChartLine {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5V13C3 14.1046 3.89543 15 5 15H7.58579L6.29289 16.2929C5.90237 16.6834 5.90237 17.3166 6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071L10 15.4142L12.2929 17.7071C12.6834 18.0976 13.3166 18.0976 13.7071 17.7071C14.0976 17.3166 14.0976 16.6834 13.7071 16.2929L12.4142 15H15C16.1046 15 17 14.1046 17 13V5C17.5523 5 18 4.55228 18 4C18 3.44772 17.5523 3 17 3H3ZM14.7071 7.70711C15.0976 7.31658 15.0976 6.68342 14.7071 6.29289C14.3166 5.90237 13.6834 5.90237 13.2929 6.29289L10 9.58579L8.70711 8.29289C8.31658 7.90237 7.68342 7.90237 7.29289 8.29289L5.29289 10.2929C4.90237 10.6834 4.90237 11.3166 5.29289 11.7071C5.68342 12.0976 6.31658 12.0976 6.70711 11.7071L8 10.4142L9.29289 11.7071C9.68342 12.0976 10.3166 12.0976 10.7071 11.7071L14.7071 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPrinter;
impl IconShape for HiPrinter {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 4V7H4C2.89543 7 2 7.89543 2 9V12C2 13.1046 2.89543 14 4 14H5V16C5 17.1046 5.89543 18 7 18H13C14.1046 18 15 17.1046 15 16V14H16C17.1046 14 18 13.1046 18 12V9C18 7.89543 17.1046 7 16 7H15V4C15 2.89543 14.1046 2 13 2H7C5.89543 2 5 2.89543 5 4ZM13 4H7V7H13V4ZM13 12H7V16H13V12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGnElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+h3jtt37v46v9/bfCfjAAEb/zx7/+47/Nfy7E/wCv8npv99aZ+V3AcR5A8NvzY8fe5rd/+rt3+c+B+G/2qm/wDi/dpum3gOM8H4Lf/uPf+qnX4T8H4r/ZK73O2/w08Fa8ECrldf7413/8t/mPh/hv9kqv8zYXgeO8EJI+549/8yc/m/94iP9mr/Q6b2P+BZI+549/8yc/m/94iP9mr/Q6b2P+BZI+549/8yc/m/94iP9Gr/zG7/xgr9dP518g+J0//q2fem3+4yH+G73S677NV2M+ihdBqfVl/vDXfuyv+Y+F+G/wym/8zg/2MHwU9kfzotul8N6LrWO/89s//d27/MdA/Ad4pdd7u48i86OBB/M/ifTTpZTP+cNf+7G/5vlD/Du9yuu+7Uen/VX8z7W7OHbsIb/909+9y/NC/Du90uu8zdOBB/M/mKTP+ePf/MnP5nkh/p1e6XXexvwPJ+lz/vg3f/KzeV6If6dXep23Mf/DSfqcP/7Nn/xsnhfi3+mVXudtzP9wkj7nj3/zJz+b54X4d3ql13kb8z+cpM/549/8yc/meSH+nV7pdd7G/A8n6XP++Dd/8rN5Xoh/p1d6nbcx/8NJ+pw//s2f/GyeF+Lf6ZVe523M/3CSPuePf/MnP5vnhfh3eqXXeRvzP5ykz/nj3/zJz+Z5If6dXul13sb8Dyfpc/74N3/ys3leiH+nV3qdtzH/w0n6nD/+zZ/8bJ4X4t/hld/4nR/s9frp/A8X0sf80W/+5FfzvBD/Dq/0um/z1ZiP4n+2S4tjxx782z/93bs8L8S/wSu/8Ts/2MPwUdgfzf9sP1Nq/ew//LUf+2ueP8S/4FXf4B1eurX2WdivDRznfyjB90StX/2Hv/Zjf82LDvFCvMrrvd1bZ+Z3Acf5H0zS+/zxb/7kd/Ovh3gBXvut3/v48tKlpwPH+R9M8D1//Fs/9d782yBegFd+3bd9b9vfxf9wpdaX+cNf+7G/5t8G8QK88uu+7Wfb/iz+h/uT3/op8W+HeAFe+XXf9rNtfxb/wy2OHTvx2z/93bv82yBegFd+/bd/bbf2W/wPFxFv80e/8RM/zb8N4oV45dd5m982vBb/s/314tix1/ntn/7uXf71EC/Ea7/1ex9fXbr004bX4n8wwW/Pjx17m9/+6e/e5V8H8SJ45dd/+9cm87W5n/3ahtfif5ZdST8N3AqAdGtE/M0f/tqP/TUvGOLf6FXf4B1euk3TbwPH+J9M+uo/+c2f/BieP8S/wyu/7tu+t+3v4n84lfI6f/zrP/7bPC/Ev8Nrv/V7H19eunSR/+Ekfc4f/+ZPfjbPC/Hv9Eqv8zbmfzhJn/PHv/mTn83zQvw7vdLrvI35H07S5/zxb/7kZ/O8EP9Or/Q6b2P+h5P0OX/8mz/52TwvxL/TK73O25j/AILviVq/+g9/7cf++rXf+r2Pr/f3XzszPxt4Kf6dJH3OH//mT342zwvx7/RKr/M25t9J0vv88W/+5HfzXF77rd/7+OrSpZ82vBb/DpI+549/8yc/m+eF+Hd6pdd5G/PvIPieP/6tn3pvXoDXfuv3Pr68dOlW4Bj/RpI+549/8yc/m+eF+Hd6pdd5G/PvUGp9mT/8tR/7a16IV36dt/luw3vxbyTpc/74N3/ys3leiH+nV3qdtzH/Dn/yWz8l/gWv/Lpv+9m2P4t/I0mf88e/+ZOfzfNC/Du90uu8jfl3+JPf+inxL3jl133bz7b9WfwbSfqcP/7Nn/xsnhfi3+mVXudtzL/Dn/zWT4l/wSu/7tu+t+3v4t9I0uf88W/+5GfzvBD/Tq/0Om/z08Bb8W/zM3/yWz/11vwLXvUN3uGl2zT9Ff9Gkt7nj3/zJ7+b54X4d3rVN3iHl27T9NvAMf51LpVaX/sPf+3H/poXwSu97tt8Neaj+Nf7mT/5rZ96a54/xH+AV37jd34ww/De/CvMd3a++rd/+rt3+Vd45dd/+9cm87V50d36x7/5k9/NC4b4/w3x/xvi/zfE/2+I/98Q/7/xj7SssFDxsQ9EAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPuzzle;
impl IconShape for HiPuzzle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 3.5C10 2.67157 10.6716 2 11.5 2C12.3284 2 13 2.67157 13 3.5V4C13 4.55228 13.4477 5 14 5H17C17.5523 5 18 5.44772 18 6V9C18 9.55228 17.5523 10 17 10H16.5C15.6716 10 15 10.6716 15 11.5C15 12.3284 15.6716 13 16.5 13H17C17.5523 13 18 13.4477 18 14V17C18 17.5523 17.5523 18 17 18H14C13.4477 18 13 17.5523 13 17V16.5C13 15.6716 12.3284 15 11.5 15C10.6716 15 10 15.6716 10 16.5V17C10 17.5523 9.55228 18 9 18H6C5.44772 18 5 17.5523 5 17V14C5 13.4477 4.55228 13 4 13H3.5C2.67157 13 2 12.3284 2 11.5C2 10.6716 2.67157 10 3.5 10H4C4.55228 10 5 9.55228 5 9V6C5 5.44772 5.44772 5 6 5H9C9.55228 5 10 4.55228 10 4V3.5Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAMSklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xr/DW7/zBDx5cXgq3l+ZfIHSrrb/5xR/7xr/m+Xjr9/7o48NyfC2AftH9zk9/91fv8ny86Tt86EtLfinjB/MvUfnrXu1vfvqHv/lWXjSIF9GbvsMHv7fQVxkf519B0lf/wo9+08fwAG/6Dh/60sK/ZXwcQGjX6HV+8ce+8a95gDd7xw/5Ktsfzb+C0K7xx/zij33zd/MvQ7wI3vqdP/jBY9NfGR/n3yAKr/PzP/zNv80zvek7fvDTMQ/mgcStv/ij3/wQnunN3/mDXzsbv8W/gdBuV/wyP/3D33wrLxziRfAW7/ghb93sn+LfSnzOL/7oN382wFu/8wc/eGg8neejLzzkp3/4m28FeNN3/ODPxnwW/0ZFepuf+9Fv+mleOMSL4E3f8YM/G/NZ/FuJz/nFH/3mzwZ483f+4NfOxm/xfEThdX7+h7/5twHe9B0/+LMxn8W/lficX/zRb/5sXjjEi+BN3/GDPxvzWfxbic/5xR/95s8GePN3/uDXzsZv8XxE4XV+/oe/+bcB3vQdP/izMZ/Fv5X4nF/80W/+bF44xIvgTd/xgz8b81n8W4nP+cUf/ebPBnjzd/7g187Gb/F8ROF1fv6Hv/m3Ad70HT/4szGfxb+V+Jxf/NFv/mxeOMSL4E3f8YM/G/NZPB+/+GPfLJ7pTd/hg38beC2em/icX/zRb/5sgLd+5w9+8NB4Os9Hvzk/8dPf/dW7AG/6jh/82ZjP4nn9zi/+2De/Ns/0pu/wweb5EZ/ziz/6zZ/NC4d4EbzpO37wZ2M+i+fjF3/sm8Uzvek7fPBvA6/FcxOf84s/+s2fzTO96Tt88G8Dr8Vz+p1f/LFvfm2e6U3f8YM/G/NZPK/f+cUf++bX5pne9B0+2Dw/4nN+8Ue/+bN54RAvgjd9xw/+bMxn8Xz84o99s3imN32HD/5t4LV4buJzfvFHv/mzeaa3fu+PPj4crn4aeC2u+J1+c/7WP/3dX73LM73pO37wZ2M+i+f1O7/4Y9/82jzTm77DB5vnR3zOL/7oN382LxziRfCm7/jBn435LJ4PwW/zLHpp4+M8N/E5v/ij3/zZPJe3fu+PPg7w09/91bs8lzd9xw/+bMxn8VyEdsF/zTMZXpvnR3zOL/7oN382LxziRfCm7/jBn435LP6txOf84o9+82fzr/Cm7/jBn435LP6txOf84o9+82fzwiFeBG/6jh/82ZjP4t9KfM4v/ug3fzb/Cm/6jh/82ZjP4t9KfM4v/ug3fzYvHOJF8Kbv8MHvDXwX/1bic37xR7/5s3mAN32HD31plG8FgONnfvHHvvGveYA3fccP/mzMZ/Fv9z6/+GPf/N28cIgXwZu+w4e+NORf8W/3Pr/4Y9/83TzTm77DB7838F08QJHe5ud+9Jt+mmd603f44PcGvot/s3iZX/yxb/xrXjjEi+hN3+GDvxr4KP71fuYXf+yb35oHeLN3+JCLxsd5AKHdX/ixbzrBA7zpO3zwTwNvxb/e1/zij33zR/MvQ/wrvPk7f/BrZ/LavKjMrb/4Y9/83TzAm77Dh7405F/xfMXL/OKPfeNf8wBv+g4f/N6IB/MiiuC3f/6Hv/m3edEg/ou9+Tt/8Gtn47d4PqLwOj//w9/82/zXQfwXe/N3/uDXzsZv8XxE4XV+/oe/+bf5r4P4V3rrd/7gB7emB/EvaOjSL/7YN/41z+XN3/mDXzsbv8XzEcRHC/81z8fP/dg3/Q4vxFu/90cfZ7U6/tM//M238qJDvIje9B0+9KVR/hTmwbyIhHZDvM/P/eg3/TTP9Obv/MGvnY3f4t9CfPYv/ug3fw4P8Nbv/dHHh6P1d2G/NQDiVhxv84s/9o1/zb8M8SJ46/f+6OPj4frpxsf5N+gLD/npH/7mWwHe/J0/+LWz8Vv8GxXpbX7uR7/pp3mmN32HD/5q4KN4AKHdbnP2kJ/+7q/e5YVDvAje4h0/5K2b/VP8W4nP+cUf/ebPBnjzd/7g187Gb/Fv9zO/+GPf/NY805u9w4dcND7Oc4nC6/z8D3/zb/PCIV4Eb/GOH/LWzf4p/q3E5/zij37zZwO89Tt/8IOHxtP5t/uZX/yxb35rnulN3+GDd4FjPJcovM7P//A3/zYvHOJF8Nbv/dHHh8PVrcAx/g36wkN++oe/+Vae6U3f4YN/G3gt/g2K9DY/96Pf9NM805u+wwd/NfBRPKdL/eb8wT/93V+9ywuHeBG96Tt86EtD/jTwIF50l4r03j/3o9/00zzAW7/3Rx8fDlc/DbwW/xric37xR7/5s3mAt37vjz4+HK6+G3grrngGxFv/4o9941/zL0P8K731O3/wgyd4MP+CbLH7iz/2jX/NC/HW7/3Rx6fV6qV5Efz8D3/zb/NCvPV7f/RxVqvjP/3D33wrLzrE/2+I/98Q/wpv/s4f/NqZvBYvIqFbf+FHv+l7eD7e9B0+9KVRvhXPJYLf+fkf/ubf5vl483f+4NfO5LV4ISL4nZ//4W/+bV40iBfRm77DB3818FH8a0k//Ys/+k1vwwO86Tt88HsD38UL9jW/+GPf/NE8wJu+wwd/NfBRvGi+5hd/7Js/mn8Z4kXwpu/woS8N+Vf8273PL/7YN383z/Rm7/AhF42P80LFy/zij33jXwO86Tt86EtD/hX/KvEyv/hj3/jXvHCIF8GbvsMHvzfwXfxbic/5xR/95s8GeNN3+NCXhvwr/gVFepuf+9Fv+mmAt3jHD3nrZv8U/wpFepuf+9Fv+mleOMSL4E3f8YM/G/NZ/FuJz/nFH/3mzwZ483f+4NfOxm/xL4qX+cUf+8a/Bnjzd/7g187Gb/GvEi/ziz/2jX/NC4d4EbzpO37wZ2M+i38r8Tm/+KPf/NkAb/7OH/za2fgtXrif+cUf++a35gHe9B0++LeB1+JF8zO/+GPf/Nb8yxAvgjd9xw/+bMxn8fz9Ds/20sAxnpv4nF/80W/+bIA3f+cPfu1s/BbPh8TfIH33L/zIN301z8ebvdOHfLTTb80LodBP/8KPfNNX86JBvAje9B0/+LMxn8Xz8Ys/9s3imd70HT74t4HX4rmJz/nFH/3mzwZ483f+4NfOxm/xfEThdX7+h7/5t/mvg3gRvOk7fvBnYz6L5+MXf+ybxTO96Tt88G8Dr8VzE5/ziz/6zZ8N8Obv/MGvnY3f4vmIwuv8/A9/82/zXwfxInjTd/zgz8Z8Fs/HL/7YN4tnetN3+ODfBl6L5yY+5xd/9Js/G+DN3/mDXzsbv8XzEYXX+fkf/ubf5r8O4kXwpu/4wZ+N+Sz+rcTn/OKPfvNnA7z5O3/wa2fjt3g+ovA6P//D3/zbPNObvsOHvjTyZ2G/NS8CoV2L3+43Zu/z09/91bv8yxAvgjd9xw/+bMxn8W8lPucXf/SbPxvgzd/5g187G7/F8xGF1/n5H/7m3wZ463f+4AePTX9lfJx/Nf31L/7YN70M/zLEi+BN3/GDPxvzWfxbic/5xR/95s8GePN3/uDXzsZv8XxE4XV+/oe/+bcB3vQdP/izMZ/Fv1EUXufnf/ibf5sXDvEieIt3/JC3bvZP8W8lPucXf/SbPxvgrd/5gx88NJ7O89EXHvLTP/zNtwK86Tt+8GdjPot/oyK9zc/96Df9NC8c4kXw1u/8wQ8eGn8NHOPfIAqv8/M//M2/zTO96Tt88K3Ag3hOz/jFH/vmB/NMb/7OH/za2fgt/m0u9YWX/ukf/uZbeeEQL6I3fYcPfm/gq4Fj/Ot8zS/+2Dd/NA/wpu/woS8N+dvAMa64BPHav/hj3/jXPMCbvsMHfzXwUfzrXAI++hd/7Ju/m38Z4l/hrd/5gx/cUi/d8EvzLzG3Qvz1L/7YN/41z8dbv/dHH59Wq5cGqPP5X//0d3/1Ls/Hm77Dh7405EsjHsy/oKC/LuG//ukf/uZbedEg/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/4x8BxsV6brmgilUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiQrcode;
impl IconShape for HiQrcode {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H7C7.55228 3 8 3.44772 8 4V7C8 7.55228 7.55228 8 7 8H4C3.44772 8 3 7.55228 3 7V4ZM5 6V5H6V6H5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 13C3 12.4477 3.44772 12 4 12H7C7.55228 12 8 12.4477 8 13V16C8 16.5523 7.55228 17 7 17H4C3.44772 17 3 16.5523 3 16V13ZM5 15V14H6V15H5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M13 3C12.4477 3 12 3.44772 12 4V7C12 7.55228 12.4477 8 13 8H16C16.5523 8 17 7.55228 17 7V4C17 3.44772 16.5523 3 16 3H13ZM14 5V6H15V5H14Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M11 4C11 3.44772 10.5523 3 10 3C9.44772 3 9 3.44772 9 4V5C9 5.55228 9.44772 6 10 6C10.5523 6 11 5.55228 11 5V4Z",
            }
            path {
                d: "M10 7C10.5523 7 11 7.44772 11 8V9H13C13.5523 9 14 9.44772 14 10C14 10.5523 13.5523 11 13 11H10C9.44772 11 9 10.5523 9 10V8C9 7.44772 9.44772 7 10 7Z",
            }
            path {
                d: "M16 9C15.4477 9 15 9.44772 15 10C15 10.5523 15.4477 11 16 11C16.5523 11 17 10.5523 17 10C17 9.44772 16.5523 9 16 9Z",
            }
            path {
                d: "M9 13C9 12.4477 9.44772 12 10 12H11C11.5523 12 12 12.4477 12 13C12 13.5523 11.5523 14 11 14V16C11 16.5523 10.5523 17 10 17C9.44772 17 9 16.5523 9 16V13Z",
            }
            path {
                d: "M7 11C7.55228 11 8 10.5523 8 10C8 9.44772 7.55228 9 7 9H4C3.44772 9 3 9.44771 3 10C3 10.5523 3.44772 11 4 11H7Z",
            }
            path {
                d: "M17 13C17 13.5523 16.5523 14 16 14H14C13.4477 14 13 13.5523 13 13C13 12.4477 13.4477 12 14 12H16C16.5523 12 17 12.4477 17 13Z",
            }
            path {
                d: "M16 17C16.5523 17 17 16.5523 17 16C17 15.4477 16.5523 15 16 15H13C12.4477 15 12 15.4477 12 16C12 16.5523 12.4477 17 13 17H16Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiQuestionMarkCircle;
impl IconShape for HiQuestionMarkCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM10 7C9.63113 7 9.3076 7.19922 9.13318 7.50073C8.85664 7.97879 8.24491 8.14215 7.76685 7.86561C7.28879 7.58906 7.12543 6.97733 7.40197 6.49927C7.91918 5.60518 8.88833 5 10 5C11.6569 5 13 6.34315 13 8C13 9.30622 12.1652 10.4175 11 10.8293V11C11 11.5523 10.5523 12 10 12C9.44773 12 9.00001 11.5523 9.00001 11V10C9.00001 9.44772 9.44773 9 10 9C10.5523 9 11 8.55228 11 8C11 7.44772 10.5523 7 10 7ZM10 15C10.5523 15 11 14.5523 11 14C11 13.4477 10.5523 13 10 13C9.44772 13 9 13.4477 9 14C9 14.5523 9.44772 15 10 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReceiptRefund;
impl IconShape for HiReceiptRefund {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C3.89543 2 3 2.89543 3 4V18L6.5 16L10 18L13.5 16L17 18V4C17 2.89543 16.1046 2 15 2H5ZM9.70711 5.70711C10.0976 5.31658 10.0976 4.68342 9.70711 4.29289C9.31658 3.90237 8.68342 3.90237 8.29289 4.29289L5.29289 7.29289C4.90237 7.68342 4.90237 8.31658 5.29289 8.70711L8.29289 11.7071C8.68342 12.0976 9.31658 12.0976 9.70711 11.7071C10.0976 11.3166 10.0976 10.6834 9.70711 10.2929L8.41421 9H10C11.6569 9 13 10.3431 13 12V13C13 13.5523 13.4477 14 14 14C14.5523 14 15 13.5523 15 13V12C15 9.23858 12.7614 7 10 7H8.41421L9.70711 5.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReceiptTax;
impl IconShape for HiReceiptTax {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C3.89543 2 3 2.89543 3 4V18L6.5 16L10 18L13.5 16L17 18V4C17 2.89543 16.1046 2 15 2H5ZM7.5 5C6.67157 5 6 5.67157 6 6.5C6 7.32843 6.67157 8 7.5 8C8.32843 8 9 7.32843 9 6.5C9 5.67157 8.32843 5 7.5 5ZM13.7071 5.29289C13.3166 4.90237 12.6834 4.90237 12.2929 5.29289L6.29289 11.2929C5.90237 11.6834 5.90237 12.3166 6.29289 12.7071C6.68342 13.0976 7.31658 13.0976 7.70711 12.7071L13.7071 6.70711C14.0976 6.31658 14.0976 5.68342 13.7071 5.29289ZM12.5 10C11.6716 10 11 10.6716 11 11.5C11 12.3284 11.6716 13 12.5 13C13.3284 13 14 12.3284 14 11.5C14 10.6716 13.3284 10 12.5 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRefresh;
impl IconShape for HiRefresh {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2C4.55228 2 5 2.44772 5 3V5.10125C6.27009 3.80489 8.04052 3 10 3C13.0494 3 15.641 4.94932 16.6014 7.66675C16.7855 8.18747 16.5126 8.75879 15.9918 8.94284C15.4711 9.12689 14.8998 8.85396 14.7157 8.33325C14.0289 6.38991 12.1755 5 10 5C8.36507 5 6.91204 5.78502 5.99935 7H9C9.55228 7 10 7.44772 10 8C10 8.55228 9.55228 9 9 9H4C3.44772 9 3 8.55228 3 8V3C3 2.44772 3.44772 2 4 2ZM4.00817 11.0572C4.52888 10.8731 5.1002 11.146 5.28425 11.6668C5.97112 13.6101 7.82453 15 10 15C11.6349 15 13.088 14.215 14.0006 13L11 13C10.4477 13 10 12.5523 10 12C10 11.4477 10.4477 11 11 11H16C16.2652 11 16.5196 11.1054 16.7071 11.2929C16.8946 11.4804 17 11.7348 17 12V17C17 17.5523 16.5523 18 16 18C15.4477 18 15 17.5523 15 17V14.8987C13.7299 16.1951 11.9595 17 10 17C6.95059 17 4.35905 15.0507 3.39857 12.3332C3.21452 11.8125 3.48745 11.2412 4.00817 11.0572Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReply;
impl IconShape for HiReply {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.70711 3.29289C8.09763 3.68342 8.09763 4.31658 7.70711 4.70711L5.41421 7H11C14.866 7 18 10.134 18 14V16C18 16.5523 17.5523 17 17 17C16.4477 17 16 16.5523 16 16V14C16 11.2386 13.7614 9 11 9H5.41421L7.70711 11.2929C8.09763 11.6834 8.09763 12.3166 7.70711 12.7071C7.31658 13.0976 6.68342 13.0976 6.29289 12.7071L2.29289 8.70711C1.90237 8.31658 1.90237 7.68342 2.29289 7.29289L6.29289 3.29289C6.68342 2.90237 7.31658 2.90237 7.70711 3.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRewind;
impl IconShape for HiRewind {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.4453 14.8321C8.75216 15.0366 9.1467 15.0557 9.47186 14.8817C9.79701 14.7077 10 14.3688 10 14L10 11.2019L15.4453 14.8321C15.7522 15.0366 16.1467 15.0557 16.4719 14.8817C16.797 14.7077 17 14.3688 17 14V6C17 5.63121 16.797 5.29235 16.4719 5.11833C16.1467 4.94431 15.7522 4.96338 15.4453 5.16795L10 8.79815V6C10 5.63121 9.79702 5.29235 9.47186 5.11833C9.1467 4.94431 8.75216 4.96338 8.4453 5.16795L2.4453 9.16795C2.1671 9.35342 2 9.66565 2 10C2 10.3344 2.1671 10.6466 2.4453 10.8321L8.4453 14.8321Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRss;
impl IconShape for HiRss {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C4.44772 3 4 3.44772 4 4C4 4.55228 4.44772 5 5 5C10.5228 5 15 9.47715 15 15C15 15.5523 15.4477 16 16 16C16.5523 16 17 15.5523 17 15C17 8.37258 11.6274 3 5 3Z",
            }
            path {
                d: "M4 9C4 8.44772 4.44772 8 5 8C8.86599 8 12 11.134 12 15C12 15.5523 11.5523 16 11 16C10.4477 16 10 15.5523 10 15C10 12.2386 7.76142 10 5 10C4.44772 10 4 9.55228 4 9Z",
            }
            path {
                d: "M3 15C3 13.8954 3.89543 13 5 13C6.10457 13 7 13.8954 7 15C7 16.1046 6.10457 17 5 17C3.89543 17 3 16.1046 3 15Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSaveAs;
impl IconShape for HiSaveAs {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.70711 7.29289C9.31658 6.90237 8.68342 6.90237 8.29289 7.29289C7.90237 7.68342 7.90237 8.31658 8.29289 8.70711L11.2929 11.7071C11.6834 12.0976 12.3166 12.0976 12.7071 11.7071L15.7071 8.70711C16.0976 8.31658 16.0976 7.68342 15.7071 7.29289C15.3166 6.90237 14.6834 6.90237 14.2929 7.29289L13 8.58579L13 5H16C17.1046 5 18 5.89543 18 7V12C18 13.1046 17.1046 14 16 14H8C6.89543 14 6 13.1046 6 12V7C6 5.89543 6.89543 5 8 5H11L11 8.58579L9.70711 7.29289Z",
            }
            path {
                d: "M11 3C11 2.44772 11.4477 2 12 2C12.5523 2 13 2.44772 13 3L13 5H11L11 3Z",
            }
            path {
                d: "M4 9C2.89543 9 2 9.89543 2 11V16C2 17.1046 2.89543 18 4 18H12C13.1046 18 14 17.1046 14 16H4V9Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIvUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+S13/q9jy8PLr0WwGLr2O/89k9/9y7/9RD/DV71Dd7hpds0/RZwnCt2S62v84e/9mN/zX8txH+DV3qdt3k68GCe061/8ls/9RD+ayH+i73yG7/zg71eP53nQ7PZQ/74l3/4Vv7rIP6LvfLrv/1ru7Xf4vlQKa/zx7/+47/Nfx3Ef7FXfv23f2239ls8Hyrldf7413/8t/mvg/gv9sqv//av7dZ+i+dDpbzOH//6j/82/3UQ/8Ve+fXf/rXd2m/xfKiU1/njX//x3+a/DuK/2Cu//tu/tlv7LZ4PlfI6f/zrP/7b/NdB/Bd75dd/+9d2a7/F86FSXuePf/3Hf5v/OogX0Su/3tu9l+23ln2cfw/FEwj9CM9P+p1wPpp/B0u7kn76j3/jJ76HfxniX/Cqb/AOL92m6aeAB/MfQPA7f/xbP/XaPB+v/Dpv89uG1+I/xq2l1rf5w1/7sb/mBUO8EK/6Bu/w0m2afgs4zn8Qwe/88W/91GvzfLzy67zNbxtei/84u6XW1/nDX/uxv+b5Q7wQr/w6b/NbhtfmP5Dgd/74t37qtXk+Xvl13ua3Da/FfyDBb//xb/3U6/D8IV6AV32Dd3jpNk1/xX8wwe/88W/91GvzfLzy67zNbxtei/9gpdaX+cNf+7G/5nkhXoBXft23/Wzbn8V/MMHv/PFv/dRr83y88uu8zW8bXov/YJI+549/8yc/m+eFeAFe+XXf9rNtfxb/wQS/88e/9VOvzfPxyq/zNr9teC3+g0n6nD/+zZ/8bJ4X4gV45dd928+2/Vn8BxP8zh//1k+9Ns/HK7/O2/y24bX4Dybpc/74N3/ys3leiBfglV/3bT/b9mfxH0zwO3/8Wz/12jwfr/w6b/PbhtfiP5ikz/nj3/zJz+Z5IV6AV37dt/1s25/FfzDB7/zxb/3Ua/N8vPLrvM1vG16L/2CSPuePf/MnP5vnhXgBXvl13/azbX8W/8EEv/PHv/VTr83z8cqv8za/bXgt/oNJ+pw//s2f/GyeF+IFeOXXfdvPtv1Z/AcT/M4f/9ZPvTbPxyu/ztv8tuG1+A8m6XP++Dd/8rN5XogX4JVf920/2/Zn8R9M8Dt//Fs/9do8H6/8Om/z24bX4j+YpM/549/8yc/meSFegFd+3bf9bNufxb/ez5RaP/sPf+3H/vpVXu/t3jozPxt4KZ5J8Dt//Fs/9do8H6/8Om/z24bX4tn+JiI++49+4yd++lXf4B1euk3TZwNvxb+SpM/549/8yc/meSFegFd+3bf9bNufxb+C4Hf++Ld+6rV5gNd+6/c+vrx06beBlwIQ/M4f/9ZPvTbPxyu/ztv8tuG1uOJvFseOvfZv//R37/IAr/w6b/PbhtfiX0HS5/zxb/7kZ/O8EC/AK7/u23627c/iXyEi3uaPfuMnfprn8tpv/d7Hl5cu/TbwUoLf+ePf+qnX5vl45dd5m982vBbwN4tjx177t3/6u3d5Lq/yem/31pn5U/wrSPqcP/7Nn/xsnhfiBXjl133bz7b9WfwrqJTX+eNf//Hf5vl47bd+7+PLS5d+W7D7x7/1U6/N8/HKr/M2v204vjh27LV/+6e/e5fn45Vf/+1f2639Fv8Kkj7nj3/zJz+b54V4AV75dd/2s21/Fv8a4mv+5Dd/6qN5AV77rd/7+HLv0mf/yW/+1EfzfLzS677NVy92jn32b//0d+/yArzS677NV2M+in8FSZ/zx7/5k5/N80K8AK/8um/72bY/i3+lUuvL/OGv/dhf85/gVd/gHV66TdNf8a8k6XP++Dd/8rN5XogX4JVf920/2/Zn8a+3W2p9nT/8tR/7a/4DveobvMNLt2n6LeA4/0qSPuePf/MnP5vnhXgBXvl13/azbX8W/za7pdbX+cNf+7G/5j/Aq77BO7x0m6bfAo7zbyDpc/74N3/ys3leiBfglV/3bT/b9mfxb7dban2dP/y1H/tr/h1e9Q3e4aXbNP0WcJx/I0mf88e/+ZOfzfNCvACv/Lpv+9m2P4t/n91S6+v84a/92F/zb/Cqb/AOL92m6beA4/w7SPqcP/7Nn/xsnhfiBXjl133bz7b9Wfz77ZZaX+cPf+3H/pp/hVd9g3d46TZNvwUc599J0uf88W/+5GfzvBAvwCu/7tt+tu3P4j/Gbqn1df7w137sr3kRvOobvMNLt2n6LeA4/wEkfc4f/+ZPfjbPC/ECvPLrvu1n2/4s/uPsllpf5w9/7cf+mhfiVd/gHV66TdNvAcf5DyLpc/74N3/ys3leiBfglV/3bT/b9mfxH2u31Po6f/hrP/bXPB+v+gbv8NJtmn4LOM5/IEmf88e/+ZOfzfNCvACv/Lpv+9m2P4v/eLul1tf5w1/7sb/mAV71Dd7hpds0/RZwnP9gkj7nj3/zJz+b54V4AV75dd/2s21/Fv85don4bHXdzwB4HN+KzM8GjvOfQNLn/PFv/uRn87wQL8Arv+7bfrbtz+L/AEmf88e/+ZOfzfNCvACv/Lpv+962v4v/AyS9zx//5k9+N88L8QK88hu/84O9Xj+d/wM0mz3kj3/5h2/leSFeiFd+3bf9bNufxf9ikj7nj3/zJz+b5w/xL3jl13mb7za8F/8LCb7nj3/rp96bFwzxInjl133bz7b90cAx/ne4JOmr//g3f/KzeeEQ/wqv/Ppv/9pkPhh4MP8z3UrErX/86z/+27xoEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/v/GPTHSMX8nSPYwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSave;
impl IconShape for HiSave {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929C5.90237 10.6834 5.90237 11.3166 6.29289 11.7071L9.29289 14.7071C9.68342 15.0976 10.3166 15.0976 10.7071 14.7071L13.7071 11.7071C14.0976 11.3166 14.0976 10.6834 13.7071 10.2929C13.3166 9.90237 12.6834 9.90237 12.2929 10.2929L11 11.5858L11 6H16C17.1046 6 18 6.89543 18 8V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V8C2 6.89543 2.89543 6 4 6H9L9 11.5858L7.70711 10.2929Z",
            }
            path {
                d: "M9 4C9 3.44772 9.44772 3 10 3C10.5523 3 11 3.44772 11 4L11 6H9L9 4Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiScale;
impl IconShape for HiScale {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.99998 2C10.5523 2 11 2.44772 11 3V4.32297L14.9544 5.90474L16.5528 5.10557C17.0467 4.85858 17.6474 5.05881 17.8944 5.55279C18.1414 6.04676 17.9412 6.64744 17.4472 6.89443L16.214 7.51101L17.9522 12.9307C18.0727 13.3065 17.961 13.718 17.6669 13.9812C16.9599 14.614 16.0238 15 15 15C13.9761 15 13.0401 14.614 12.3331 13.9812C12.039 13.718 11.9272 13.3065 12.0477 12.9307L13.7631 7.58227L11 6.47703V16H13C13.5523 16 14 16.4477 14 17C14 17.5523 13.5523 18 13 18H6.99997C6.44769 18 5.99997 17.5523 5.99997 17C5.99997 16.4477 6.44769 16 6.99997 16H8.99997V6.47703L6.23689 7.58227L7.9522 12.9307C8.07272 13.3065 7.96096 13.718 7.66689 13.9812C6.95988 14.614 6.02381 15 4.99997 15C3.97614 15 3.04007 14.614 2.33306 13.9812C2.03899 13.718 1.92723 13.3065 2.04775 12.9307L3.78592 7.51101L2.55276 6.89443C2.05878 6.64744 1.85856 6.04676 2.10555 5.55279C2.35254 5.05881 2.95321 4.85858 3.44719 5.10557L5.04553 5.90474L8.99997 4.32297V3C8.99997 2.44772 9.44769 2 9.99998 2ZM4.99997 10.2745L4.18174 12.8258C4.43132 12.9378 4.708 13 4.99997 13C5.29194 13 5.56863 12.9378 5.81821 12.8258L4.99997 10.2745ZM15 10.2745L14.1817 12.8258C14.4313 12.9378 14.708 13 15 13C15.2919 13 15.5686 12.9378 15.8182 12.8258L15 10.2745Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiScissors;
impl IconShape for HiScissors {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.5 2C3.567 2 2 3.567 2 5.5C2 7.433 3.567 9 5.5 9C6.10276 9 6.66993 8.84763 7.1651 8.57931L8.58582 10L7.16515 11.4207C6.66997 11.1524 6.10278 11 5.5 11C3.567 11 2 12.567 2 14.5C2 16.433 3.567 18 5.5 18C7.433 18 9 16.433 9 14.5C9 13.8973 8.84764 13.3301 8.57934 12.835L16.7072 4.70711C17.0977 4.31658 17.0977 3.68342 16.7072 3.29289C16.3167 2.90237 15.6835 2.90237 15.293 3.29289L10 8.58582L8.57931 7.1651C8.84763 6.66993 9 6.10276 9 5.5C9 3.567 7.433 2 5.5 2ZM4 5.5C4 4.67157 4.67157 4 5.5 4C6.32843 4 7 4.67157 7 5.5C7 6.32843 6.32843 7 5.5 7C4.67157 7 4 6.32843 4 5.5ZM4 14.5C4 13.6716 4.67157 13 5.5 13C6.32843 13 7 13.6716 7 14.5C7 15.3284 6.32843 16 5.5 16C4.67157 16 4 15.3284 4 14.5Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12.8284 11.4142C12.4379 11.0237 11.8047 11.0237 11.4142 11.4142C11.0237 11.8047 11.0237 12.4379 11.4142 12.8284L15.2929 16.7071C15.6834 17.0976 16.3166 17.0976 16.7071 16.7071C17.0976 16.3166 17.0976 15.6834 16.7071 15.2929L12.8284 11.4142Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSearchCircle;
impl IconShape for HiSearchCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 9C9 7.89543 9.89543 7 11 7C12.1046 7 13 7.89543 13 9C13 10.1046 12.1046 11 11 11C10.4474 11 9.94881 10.7772 9.58579 10.4142C9.22276 10.0512 9 9.55256 9 9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 5C8.79086 5 7 6.79086 7 9C7 9.74138 7.20229 10.4364 7.55397 11.0318L5.29289 13.2929C4.90237 13.6834 4.90237 14.3166 5.29289 14.7071C5.68342 15.0976 6.31658 15.0976 6.70711 14.7071L8.96818 12.446C9.56362 12.7977 10.2586 13 11 13C13.2091 13 15 11.2091 15 9C15 6.79086 13.2091 5 11 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIqElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+E7z1O3/wgweXl8LtpXkuQrfa+ptf/LFv/Gv++yH+g7zpO3zoS0N+FOK1MQ/mXyC0a/zTRPnpX/yRb/gZ/nsg/p3e/J0/+LXd+CzDa/NvJW4V+uxf+NFv+h7+ayH+jd76nT/4wWPjuwyvzX8Y/TXofX7xx77xr/mvgfg3eIt3/JC3TvNdxsf5TxDER//8j33j1/CfD/Gv9Kbv8MHvDXwX//m++xd/7Jvfh/9ciH+FN32HD35v4Lv417kE/DXwYOBB/Ot89y/+2De/D/95EC+it3jHD3nrZv8U/7JnAD8N8d2/+GPf+Nc8l7d+5w9+8Gi9NfZ727wU/xLxOb/4o9/82fznQLwI3vqdP/jBY9NfGR/nBbsEfPQv/tg3fzcvojd/5w9+7Wx8N/AgXogovM7P//A3/zb/8RAvgjd7hw/+LcNr84L9Tr85f+uf/u6v3uXf4M3e8YO/2+a9eEHErf3G/GV++ru/epf/WIh/wZu/8we/djZ+ixdA4nt+4Ue/+b35d3rTd/jg9wa+ixdEfM4v/ug3fzb/sRD/gjd7hw/+LcNr8/z9zi/+2De/Nv9B3vQdPvirgY/i+RDa7TZnD/np7/7qXf7jIF6IN32HD31pyL/i+bvUb84f/NPf/dW7/Ad603f44N8GXovnQ6GP+YUf+aav5j8O4oV4s3f84O+2eS+ev/f5xR/75u/mP9ibv/MHv3Y2fovnS3/9iz/2TS/DfxzEC/Gm7/jBT8c8mOf1jF/8sW9+MP9J3uwdP/i7bd6L56PfnJ/46e/+6l3+YyBegLd+5w9+8NB4Os/f1/zij33zR/Of5E3f4YPfG/gunr/3+cUf++bv5j8G4gV4i3f8kLdu9k/xfMXL/OKPfeNf85/krd/7o48Ph6uLPD/ic37xR7/5s/mPgXgB3vQdP/izMZ/F8/GLP/bN4j/Zm77DB/828Fo8r5/5xR/75rfmPwbiBXjTd/zgz8Z8Fs/r0i/+2Dcf5z/Zm77DB/828Fo8r9/5xR/75tfmPwbiBXjTd/zgz8Z8Fs/rd37xx775tflP9qbv8ME/DbwVz+t3fvHHvvm1+Y+BeAHe9B0/+LMxn8VzE7f+4o9+80P4T/am7/DBvw28Fs/rd37xx775tfmPgXgB3vQdP/izMZ/F8/GLP/bN4j/Zm77DB/828Fo8r5/5xR/75rfmPwbiBXjTd/jg9wa+i+ejLzzkp3/4m2/lP9GbvcOHXDQ+znMTn/OLP/rNn81/DMQL8Kbv8KEvDflXPB8Kfcwv/Mg3fTX/Sd7iHT/krZv9Uzx/7/OLP/bN381/DMQL8abv8MG7wDGeh/76F3/sm16G/yRv9o4f/N0278Xz0Rce8tM//M238h8D8UK82Tt+8HfbvBfPRxRe5+d/+Jt/m/9gb/3OH/zgofF0nr9n/OKPffOD+Y+DeCHe4h0/5K2b/VM8P+LWX/zRb34I/8He9B0/5Kew35rnQ6GP+YUf+aav5j8O4l/wpu/wwbcCD+L5++5f/LFvfh/+g7zpO3zwewPfxQvQb85P/PR3f/Uu/3EQ/4I3fYcPfm/gu3jB3ucXf+ybv5t/p7d4xw9562b/FC/Y1/zij33zR/MfC/EieLN3/OC/tnkpXgBJX/0LP/pNH8O/0Zu+wwe/N/BdvGCX+s35g3/6u796l/9YiBfBm77Dh7405F/xQgh+W4XP+fkf/ubf5kX01u/8wQ8eUl+F/da8EAp9zC/8yDd9Nf/xEC+iN3unD/lop7+Kf9l3E+Wn+0X3Oz/93V+9y/Pxpu/0YW9FtrcG3psXgdCu0ev84o9941/zHwvxr/Bm7/jB323zXryIBL8NYLhV8GAu00sbH+dfSWjX6HV+8ce+8a/5j4P4V3qzd/zg77Z5L/4bCO0avc4v/tg3/jX/MRD/Bm/6jh/82ZjP4r+B0K7R6/zij33jX/Pvh/g3evN3/uDXzsZ3Aw/iP84lhT7b6c8GjvECCO0avc4v/tg3/jX/Poh/h7d+748+PhytPhrz0cAx/n2+pt+cf/ZPf/dX777pO3zoS0P+NnCMF0Bo1+h1fvHHvvGv+bdD/Ad46/f+6OPjcv3e2O9t81K86J6h0Fd3i9l3//R3f/UuD/Cm7/ChLw3528AxXgChXaPX+cUf+8a/5t8G8R/srd/7o48Ph6u3RjwY89LAcZ5tF/HXmFv7wm//9A9/8628EG/6Dh/60pC/DRzjBRDaNXqdX/yxb/xr/vUQ/8O96Tt86EtD/jZwjBdAaNfodX7xx77xr/nXQfwv8Kbv8KEvDfnbwDFeAKFdo9f5xR/7xr/mRYf4X+JN3+FDXxryt4FjvABCu0av84s/9o1/zYsG8b/Im77Dh7405G8Dx3gBhHaNXucXf+wb/5p/GeJ/mTd9hw99acjfBo7xAgjtGr3OL/7YN/41Lxzif6E3fYcPfWnI3waO8QII7Rq9zi/+2Df+NS8Y4n+pN32HD31pyN8GjvECCO0avc4v/tg3/jXPH+J/sTd9hw99acjfBo7xgohb+435y/z0d3/1Ls8L8b/cm77Dh7405G8Dx3gBFPqYX/iRb/pqnhfi/4A3fYcPfWnI3waO8fyIz/nFH/3mz+Z5If6PeNN3+NCXhvxt4BjP631+8ce++bt5Xoj/Q970HT70pSF/GngQz/Yzv/hj3/zWPH+I/2Pe+r0/+vi4XL+37eOYW3/xx775u3nBEP+/If5/Q/z/hvj/DfH/G+L/N/4RzuVaX2oC58IAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSearch;
impl IconShape for HiSearch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4ZM2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSelector;
impl IconShape for HiSelector {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 3C10.2652 3 10.5196 3.10536 10.7071 3.29289L13.7071 6.29289C14.0976 6.68342 14.0976 7.31658 13.7071 7.70711C13.3166 8.09763 12.6834 8.09763 12.2929 7.70711L10 5.41421L7.70711 7.70711C7.31658 8.09763 6.68342 8.09763 6.29289 7.70711C5.90237 7.31658 5.90237 6.68342 6.29289 6.29289L9.29289 3.29289C9.48043 3.10536 9.73478 3 10 3ZM6.29289 12.2929C6.68342 11.9024 7.31658 11.9024 7.70711 12.2929L10 14.5858L12.2929 12.2929C12.6834 11.9024 13.3166 11.9024 13.7071 12.2929C14.0976 12.6834 14.0976 13.3166 13.7071 13.7071L10.7071 16.7071C10.3166 17.0976 9.68342 17.0976 9.29289 16.7071L6.29289 13.7071C5.90237 13.3166 5.90237 12.6834 6.29289 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiServer;
impl IconShape for HiServer {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5V7C18 8.10457 17.1046 9 16 9H4C2.89543 9 2 8.10457 2 7V5ZM16 6C16 6.55228 15.5523 7 15 7C14.4477 7 14 6.55228 14 6C14 5.44772 14.4477 5 15 5C15.5523 5 16 5.44772 16 6Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 13C2 11.8954 2.89543 11 4 11H16C17.1046 11 18 11.8954 18 13V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V13ZM16 14C16 14.5523 15.5523 15 15 15C14.4477 15 14 14.5523 14 14C14 13.4477 14.4477 13 15 13C15.5523 13 16 13.4477 16 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShare;
impl IconShape for HiShare {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 8C16.6569 8 18 6.65685 18 5C18 3.34315 16.6569 2 15 2C13.3431 2 12 3.34315 12 5C12 5.12548 12.0077 5.24917 12.0227 5.37061L7.08259 7.84064C6.54303 7.32015 5.8089 7 5 7C3.34315 7 2 8.34315 2 10C2 11.6569 3.34315 13 5 13C5.80892 13 6.54306 12.6798 7.08263 12.1593L12.0227 14.6293C12.0077 14.7508 12 14.8745 12 15C12 16.6569 13.3431 18 15 18C16.6569 18 18 16.6569 18 15C18 13.3431 16.6569 12 15 12C14.1911 12 13.457 12.3201 12.9174 12.8406L7.97733 10.3706C7.9923 10.2492 8 10.1255 8 10C8 9.8745 7.99229 9.7508 7.97733 9.62934L12.9174 7.15932C13.4569 7.67984 14.1911 8 15 8Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIfklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+G73pO3zwewP84o9983fz3wPx3+TN3vFDvsr2RwNI+upf+NFv+hj+6yH+i731O3/wg8fGdxlemwcQ/Ha3OX+bn/7ur97lvw7iv9BbvOOHvHWa7zI+zvMhtBvifX7uR7/pp/mvgfgv8Nbv/MEPHlJfhf3WvCikn+7DH/PTP/zNt/KfC/Gf6K3f+6OPj0frz7L90fwbSPrqbmP2OT/93V+9y38OxH+Ct37nD37w0Pgsobc2Ps6/g9Au4ru78Nf89A9/8638x0L8B3qzd/yQ9zK8NfZb859B+mkU3/2LP/INP8N/DMS/w1u/8wc/eEy9lu3XFnpr4+P8FxDaNf5pSb/dhX/np3/4m2/l3wbxIniLd/iQ1wJI8WDjB4NeGvzSmAfzP4G4FfTX4L8WujXMrQA/92Pf9Du8cIgX4K3f+YMfPDSezv8BfeEhP/3D33wrzwvxArz5O3/wa2fjt/g/IAqv8/M//M2/zfNCvABv+g4f/N7Ad/F/w/v84o9983fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/ZhLfY7gVcxx4b+AY/1ric37xR7/5s3leiBfgTd/xgz8b81n897kUhbf++R/+5t/mmd76nT/4wUPjr4Fj/GuIz/nFH/3mz+Z5IV6AN33HD/5szGfx3+MSxGv/4o9941/zXN7snT7ko53+Kv41xOf84o9+82fzvBAvwJu+4wd/Nuaz+K93CeK1f/HHvvGveT7e/J0/+LWz8Vv8a4jP+cUf/ebP5nkhXoA3fYcPfm/gu/ivdQnitX/xx77xr3kB3vQdPvi9ge/iX6FIb/NzP/pNP83zQrwAb/7OH/za2fgt/utcgnjtX/yxb/xrXoC3fu+PPj4crf4K82D+FaLwOj//w9/82zwvxAvw1u/8wQ8eGk/n30jib2xeihfNJYjX/sUf+8a/5gV46/f+6OPD4fq3wC/Nv1JfeMhP//A338rzQrwQb/oOH7wLHONfQ3zOL/7oN382wFu/90cfHw5X3w28FS/YJYjX/sUf+8a/5gV46/f+6OPD4fq3wC/Nv96lX/yxbz7O84d4Id70HT74t4HX4kX3M7/4Y9/81jyXN3vHD/5um/fieV2CeO1f/LFv/GtegLd+748+Phyufwv80vzb/M4v/tg3vzbPH+KFeNN3/ODPxnwWL6IovM7P//A3/zbPx5u94wd/t8178WyXIF77F3/sG/+aF+Ct3/ujjw+H698CvzT/VuJzfvFHv/mzef4QL8Sbv/MHv3Y2fosXURRe5+d/+Jt/mxfgzd7xg7/b5r2ASxCv/Ys/9o1/zQvw1u/90ceHw/VvgV+af5d4mV/8sW/8a54/xL/gTd/hg3eBY7wopJ/+xR/9prfhhXjTd/jgr4b47l/8sW/8a16At37vjz4+HK5/C/zS/Ptc+sUf++bjvGCIf8GbveMHf7fNe/Gie59f/LFv/m7+jd76vT/6+HC4/i3wS/PvJPE9v/Cj3/zevGCIf8FbvOOHvHWzf4p/nff5xR/75u/mX+mt3/ujjw+H698CvzT/AYr0Nj/3o9/007xgiBfBm77DB98KPIh/nff5xR/75u/mRfTW7/3Rx4fD9W+BX5r/GM/4xR/75gfzwiFeBG/6jh/82ZjP4l/vfX7xx775u/kXvPV7f/Tx4XD9W+CX5j+K+Jxf/NFv/mxeOMSL4K3f+6OPD4erW4Fj/Ou9zy/+2Dd/Ny/AW7/3Rx8fDte/BX5p/uNc6jfnD/7p7/7qXV44xIvozd7xg7/b5r34t3mfX/yxb/5unstbv/dHHx8O178Ffmn+A0l8zy/86De/N/8yxIvord/5gx88NP4aOMa/zfv84o9983fzTG/93h99fDhc/xb4pfmPdanfnD/4p7/7q3f5lyH+Fd70HT/4szGfxb+R4LctfhtA1kcbH+c/mvicX/zRb/5sXjSIf4W3fu+PPj4crv4aeBD/Mz2j35y/9E9/91fv8qJB/Cu9+Tt/8Gtn47f4HygKr/PzP/zNv82LDvFv8Kbv8MFfDXwU/7N8zS/+2Dd/NP86iH+Dt37vjz4+Hq1+2+al+B9A4m+6jflr//R3f/Uu/zqIf6O3fucPfvDQ+GvgGP+9LvWFl/7pH/7mW/nXQ/w7vPk7f/BrZ+O3+G8VL/OLP/aNf82/DeLf6U3f4YPfG/gu/nu8zy/+2Dd/N/92iP8Ab/oOH/zewHfxX+t9fvHHvvm7+fdB/Ad503f44PcGvov/Gu/ziz/2zd/Nvx/iP9BbvOOHvHWzvxs4xn+OS0V675/70W/6af5jIP6Dvek7fOhLS/ndNi/FfyCJv7HjvX/xx77xr/mPg/hP8Nbv/dHHx6PVV9u8F/8BJL6n25h/9E9/91fv8h8L8Z/oLd7xQ9662d8NHOPf5lKR3vvnfvSbfpr/HIj/ZG/93h99fDhcfTbwUfzrfE2/Of/sn/7ur97lPw/iv8hbv/MHP3hMPtvmvXghJL6nCz77p3/4m2/lPx/iv9hbv/MHP3hMPtvmrYFjXHFJ4qe74LN/+oe/+Vb+6yD+m7z1e3/08eFw9dYA/eb8p3/6u796l/96iP/fEP+/If5/Q/z/hvj/jX8EcuRvX5IEElUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShieldCheck;
impl IconShape for HiShieldCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.16611 4.99891C5.17437 4.95809 7.91528 3.81033 10 1.94446C12.0847 3.81033 14.8256 4.95809 17.8339 4.99891C17.9431 5.64968 18 6.31821 18 7.00003C18 12.2249 14.6608 16.6698 10 18.3172C5.33923 16.6698 2 12.2249 2 7.00003C2 6.31821 2.05686 5.64968 2.16611 4.99891ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShieldExclamation;
impl IconShape for HiShieldExclamation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 1.94446C7.91528 3.81033 5.17437 4.95809 2.16611 4.99891C2.05686 5.64968 2 6.31821 2 7.00003C2 12.2249 5.33923 16.6698 10 18.3172C14.6608 16.6698 18 12.2249 18 7.00003C18 6.31821 17.9431 5.64968 17.8339 4.99891C14.8256 4.95809 12.0847 3.81033 10 1.94446ZM11 14C11 14.5523 10.5523 15 10 15C9.44771 15 9 14.5523 9 14C9 13.4477 9.44771 13 10 13C10.5523 13 11 13.4477 11 14ZM11 7C11 6.44772 10.5523 6 10 6C9.44771 6 9 6.44772 9 7V10C9 10.5523 9.44771 11 10 11C10.5523 11 11 10.5523 11 10V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShoppingBag;
impl IconShape for HiShoppingBag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C7.79086 2 6 3.79086 6 6V7H5C4.49046 7 4.06239 7.38314 4.00612 7.88957L3.00612 16.8896C2.97471 17.1723 3.06518 17.455 3.25488 17.6669C3.44458 17.8789 3.71556 18 4 18H16C16.2844 18 16.5554 17.8789 16.7451 17.6669C16.9348 17.455 17.0253 17.1723 16.9939 16.8896L15.9939 7.88957C15.9376 7.38314 15.5096 7 15 7H14V6C14 3.79086 12.2091 2 10 2ZM12 7V6C12 4.89543 11.1046 4 10 4C8.89543 4 8 4.89543 8 6V7H12ZM6 10C6 9.44772 6.44772 9 7 9C7.55228 9 8 9.44772 8 10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10ZM13 9C12.4477 9 12 9.44772 12 10C12 10.5523 12.4477 11 13 11C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShoppingCart;
impl IconShape for HiShoppingCart {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 1C2.44772 1 2 1.44772 2 2C2 2.55228 2.44772 3 3 3H4.21922L4.52478 4.22224C4.52799 4.23637 4.5315 4.25039 4.5353 4.26429L5.89253 9.69321L4.99995 10.5858C3.74002 11.8457 4.63235 14 6.41416 14H15C15.5522 14 16 13.5523 16 13C16 12.4477 15.5522 12 15 12L6.41417 12L7.41416 11H14C14.3788 11 14.725 10.786 14.8944 10.4472L17.8944 4.44721C18.0494 4.13723 18.0329 3.76909 17.8507 3.47427C17.6684 3.17945 17.3466 3 17 3H6.28078L5.97014 1.75746C5.85885 1.3123 5.45887 1 5 1H3Z",
            }
            path {
                d: "M16 16.5C16 17.3284 15.3284 18 14.5 18C13.6716 18 13 17.3284 13 16.5C13 15.6716 13.6716 15 14.5 15C15.3284 15 16 15.6716 16 16.5Z",
            }
            path {
                d: "M6.5 18C7.32843 18 8 17.3284 8 16.5C8 15.6716 7.32843 15 6.5 15C5.67157 15 5 15.6716 5 16.5C5 17.3284 5.67157 18 6.5 18Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSortAscending;
impl IconShape for HiSortAscending {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5H14C14.5523 5 15 4.55228 15 4C15 3.44772 14.5523 3 14 3H3Z",
            }
            path {
                d: "M3 7C2.44772 7 2 7.44772 2 8C2 8.55228 2.44772 9 3 9H8C8.55228 9 9 8.55228 9 8C9 7.44772 8.55228 7 8 7H3Z",
            }
            path {
                d: "M3 11C2.44772 11 2 11.4477 2 12C2 12.5523 2.44772 13 3 13H7C7.55228 13 8 12.5523 8 12C8 11.4477 7.55228 11 7 11H3Z",
            }
            path {
                d: "M13 16C13 16.5523 13.4477 17 14 17C14.5523 17 15 16.5523 15 16L15 10.4142L16.2929 11.7071C16.6834 12.0976 17.3166 12.0976 17.7071 11.7071C18.0976 11.3166 18.0976 10.6834 17.7071 10.2929L14.7071 7.29289C14.5196 7.10536 14.2652 7 14 7C13.7348 7 13.4804 7.10536 13.2929 7.29289L10.2929 10.2929C9.90237 10.6834 9.90237 11.3166 10.2929 11.7071C10.6834 12.0976 11.3166 12.0976 11.7071 11.7071L13 10.4142L13 16Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSortDescending;
impl IconShape for HiSortDescending {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5H14C14.5523 5 15 4.55228 15 4C15 3.44772 14.5523 3 14 3H3Z",
            }
            path {
                d: "M3 7C2.44772 7 2 7.44772 2 8C2 8.55228 2.44772 9 3 9H10C10.5523 9 11 8.55228 11 8C11 7.44772 10.5523 7 10 7H3Z",
            }
            path {
                d: "M3 11C2.44772 11 2 11.4477 2 12C2 12.5523 2.44772 13 3 13H7C7.55228 13 8 12.5523 8 12C8 11.4477 7.55228 11 7 11H3Z",
            }
            path {
                d: "M15 8C15 7.44772 14.5523 7 14 7C13.4477 7 13 7.44771 13 8L13 13.5858L11.7071 12.2929C11.3166 11.9024 10.6834 11.9024 10.2929 12.2929C9.90237 12.6834 9.90237 13.3166 10.2929 13.7071L13.2929 16.7071C13.4804 16.8946 13.7348 17 14 17C14.2652 17 14.5196 16.8946 14.7071 16.7071L17.7071 13.7071C18.0976 13.3166 18.0976 12.6834 17.7071 12.2929C17.3166 11.9024 16.6834 11.9024 16.2929 12.2929L15 13.5858L15 8Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSparkles;
impl IconShape for HiSparkles {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C5.55228 2 6 2.44772 6 3V4H7C7.55228 4 8 4.44772 8 5C8 5.55228 7.55228 6 7 6H6V7C6 7.55228 5.55228 8 5 8C4.44772 8 4 7.55228 4 7V6H3C2.44772 6 2 5.55228 2 5C2 4.44772 2.44772 4 3 4H4V3C4 2.44772 4.44772 2 5 2ZM5 12C5.55228 12 6 12.4477 6 13V14H7C7.55228 14 8 14.4477 8 15C8 15.5523 7.55228 16 7 16H6V17C6 17.5523 5.55228 18 5 18C4.44772 18 4 17.5523 4 17V16H3C2.44772 16 2 15.5523 2 15C2 14.4477 2.44772 14 3 14H4V13C4 12.4477 4.44772 12 5 12Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.9999 2C12.4537 2 12.8505 2.30548 12.9667 2.74411L14.1459 7.19893L17.4997 9.13381C17.8092 9.31241 17.9999 9.64262 17.9999 10C17.9999 10.3574 17.8092 10.6876 17.4997 10.8662L14.1459 12.8011L12.9667 17.2559C12.8505 17.6945 12.4537 18 11.9999 18C11.5462 18 11.1493 17.6945 11.0332 17.2559L9.85402 12.8011L6.50027 10.8662C6.19072 10.6876 6 10.3574 6 10C6 9.64262 6.19072 9.31241 6.50027 9.13382L9.85402 7.19893L11.0332 2.74411C11.1493 2.30548 11.5462 2 11.9999 2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSpeakerphone;
impl IconShape for HiSpeakerphone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 3C18 2.65342 17.8205 2.33156 17.5257 2.14935C17.2309 1.96714 16.8628 1.95058 16.5528 2.10557L8.76393 6H5C3.34315 6 2 7.34315 2 9C2 10.6569 3.34315 12 5 12H5.27925L7.05132 17.3162C7.18744 17.7246 7.56958 18 8.00001 18H9.00001C9.55229 18 10 17.5523 10 17V12.618L16.5528 15.8944C16.8628 16.0494 17.2309 16.0329 17.5257 15.8507C17.8205 15.6684 18 15.3466 18 15V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStar;
impl IconShape for HiStar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.04893 2.92707C9.34828 2.00576 10.6517 2.00576 10.951 2.92707L12.0206 6.21886C12.1545 6.63089 12.5384 6.90985 12.9717 6.90985H16.4329C17.4016 6.90985 17.8044 8.14946 17.0207 8.71886L14.2205 10.7533C13.87 11.0079 13.7233 11.4593 13.8572 11.8713L14.9268 15.1631C15.2261 16.0844 14.1717 16.8506 13.3879 16.2812L10.5878 14.2467C10.2373 13.9921 9.76269 13.9921 9.4122 14.2467L6.61203 16.2812C5.82832 16.8506 4.77384 16.0844 5.07319 15.1631L6.14276 11.8713C6.27663 11.4593 6.12997 11.0079 5.77949 10.7533L2.97932 8.71886C2.1956 8.14946 2.59838 6.90985 3.5671 6.90985H7.0283C7.46153 6.90985 7.84548 6.63089 7.97936 6.21886L9.04893 2.92707Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStatusOffline;
impl IconShape for HiStatusOffline {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.70711 2.29289C3.31658 1.90237 2.68342 1.90237 2.29289 2.29289C1.90237 2.68342 1.90237 3.31658 2.29289 3.70711L9.21426 10.6285C9.26325 10.6906 9.31947 10.7469 9.38164 10.7958L16.2929 17.7071C16.6834 18.0976 17.3166 18.0976 17.7071 17.7071C18.0976 17.3166 18.0976 16.6834 17.7071 16.2929L17.0323 15.6181C19.8626 12.0844 19.6398 6.91177 16.3641 3.63603C15.9736 3.24551 15.3404 3.24551 14.9499 3.63603C14.5593 4.02656 14.5593 4.65972 14.9499 5.05025C17.4435 7.54386 17.6625 11.4508 15.6068 14.1926L14.172 12.7578C15.4582 10.8164 15.2461 8.17494 13.5357 6.46446C13.1451 6.07394 12.512 6.07394 12.1214 6.46446C11.7309 6.85498 11.7309 7.48815 12.1214 7.87867C13.0451 8.80233 13.2406 10.1784 12.7078 11.2936L10.7164 9.30219C10.7103 9.29595 10.7042 9.28979 10.6979 9.2837L3.70711 2.29289Z",
            }
            path {
                d: "M3.23766 8.1865C3.38012 7.65291 3.06305 7.10485 2.52946 6.96239C1.99586 6.81992 1.44781 7.137 1.30535 7.67059C0.5045 10.6701 1.27982 14.0074 3.63615 16.3637C4.02667 16.7542 4.65984 16.7542 5.05036 16.3637C5.44089 15.9732 5.44089 15.34 5.05036 14.9495C3.21924 13.1184 2.6134 10.5246 3.23766 8.1865Z",
            }
            path {
                d: "M7.40075 11.4995C7.12434 11.0214 6.51266 10.8578 6.03452 11.1343C5.55639 11.4107 5.39285 12.0223 5.66926 12.5005C5.88367 12.8714 6.14907 13.2198 6.46458 13.5353C6.85511 13.9258 7.48827 13.9258 7.8788 13.5353C8.26932 13.1448 8.26932 12.5116 7.8788 12.1211C7.68771 11.93 7.52865 11.7208 7.40075 11.4995Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStatusOnline;
impl IconShape for HiStatusOnline {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.05025 3.63579C5.44078 4.02631 5.44078 4.65948 5.05025 5.05C2.31658 7.78367 2.31658 12.2158 5.05025 14.9495C5.44078 15.34 5.44078 15.9732 5.05025 16.3637C4.65973 16.7542 4.02656 16.7542 3.63604 16.3637C0.12132 12.849 0.12132 7.15051 3.63604 3.63579C4.02656 3.24526 4.65973 3.24526 5.05025 3.63579ZM14.9498 3.63602C15.3403 3.2455 15.9735 3.2455 16.364 3.63602C19.8787 7.15074 19.8787 12.8492 16.364 16.3639C15.9735 16.7545 15.3403 16.7545 14.9498 16.3639C14.5592 15.9734 14.5592 15.3403 14.9498 14.9497C17.6834 12.2161 17.6834 7.78391 14.9498 5.05023C14.5592 4.65971 14.5592 4.02655 14.9498 3.63602ZM7.87869 6.46422C8.26921 6.85474 8.26921 7.48791 7.87869 7.87843C6.70711 9.05 6.70711 10.9495 7.87869 12.1211C8.26921 12.5116 8.26921 13.1448 7.87868 13.5353C7.48816 13.9258 6.855 13.9258 6.46447 13.5353C4.51185 11.5827 4.51185 8.41684 6.46447 6.46422C6.855 6.07369 7.48816 6.07369 7.87869 6.46422ZM12.1213 6.46445C12.5119 6.07392 13.145 6.07392 13.5355 6.46445C15.4882 8.41707 15.4882 11.5829 13.5355 13.5355C13.145 13.926 12.5119 13.926 12.1213 13.5355C11.7308 13.145 11.7308 12.5118 12.1213 12.1213C13.2929 10.9497 13.2929 9.05023 12.1213 7.87866C11.7308 7.48814 11.7308 6.85497 12.1213 6.46445ZM10 8.99998C10.5523 8.99998 11 9.4477 11 9.99998V10.01C11 10.5623 10.5523 11.01 10 11.01C9.44772 11.01 9 10.5623 9 10.01V9.99998C9 9.4477 9.44772 8.99998 10 8.99998Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStop;
impl IconShape for HiStop {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8 7C7.44772 7 7 7.44772 7 8V12C7 12.5523 7.44772 13 8 13H12C12.5523 13 13 12.5523 13 12V8C13 7.44772 12.5523 7 12 7H8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSun;
impl IconShape for HiSun {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C10.5523 2 11 2.44772 11 3V4C11 4.55228 10.5523 5 10 5C9.44772 5 9 4.55228 9 4V3C9 2.44772 9.44772 2 10 2ZM14 10C14 12.2091 12.2091 14 10 14C7.79086 14 6 12.2091 6 10C6 7.79086 7.79086 6 10 6C12.2091 6 14 7.79086 14 10ZM13.5356 14.9497L14.2427 15.6568C14.6332 16.0473 15.2664 16.0473 15.6569 15.6568C16.0474 15.2663 16.0474 14.6331 15.6569 14.2426L14.9498 13.5355C14.5593 13.145 13.9261 13.145 13.5356 13.5355C13.1451 13.926 13.1451 14.5592 13.5356 14.9497ZM15.6568 4.34309C16.0473 4.73362 16.0473 5.36678 15.6568 5.75731L14.9497 6.46441C14.5592 6.85494 13.926 6.85494 13.5355 6.46441C13.145 6.07389 13.145 5.44072 13.5355 5.0502L14.2426 4.34309C14.6331 3.95257 15.2663 3.95257 15.6568 4.34309ZM17 11C17.5523 11 18 10.5523 18 10C18 9.44772 17.5523 9 17 9H16C15.4477 9 15 9.44772 15 10C15 10.5523 15.4477 11 16 11H17ZM10 15C10.5523 15 11 15.4477 11 16V17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17V16C9 15.4477 9.44772 15 10 15ZM5.05031 6.46443C5.44083 6.85496 6.074 6.85496 6.46452 6.46443C6.85505 6.07391 6.85505 5.44074 6.46452 5.05022L5.75742 4.34311C5.36689 3.95259 4.73373 3.95259 4.3432 4.34311C3.95268 4.73363 3.95268 5.3668 4.3432 5.75732L5.05031 6.46443ZM6.46443 14.9497L5.75732 15.6568C5.3668 16.0473 4.73363 16.0473 4.34311 15.6568C3.95259 15.2663 3.95259 14.6331 4.34311 14.2426L5.05022 13.5355C5.44074 13.145 6.07391 13.145 6.46443 13.5355C6.85496 13.926 6.85496 14.5592 6.46443 14.9497ZM4 11C4.55228 11 5 10.5523 5 10C5 9.44772 4.55228 9 4 9H3C2.44772 9 2 9.44772 2 10C2 10.5523 2.44772 11 3 11H4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSupport;
impl IconShape for HiSupport {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM16 10C16 10.9926 15.7589 11.929 15.3322 12.7537L13.8076 11.2291C13.9325 10.8419 14 10.4288 14 10C14 9.6714 13.9604 9.35205 13.8856 9.04648L15.4484 7.48368C15.8025 8.24895 16 9.1014 16 10ZM10.8345 13.9128L12.4156 15.4939C11.6765 15.8193 10.8594 16 10 16C9.1014 16 8.24895 15.8025 7.48368 15.4484L9.04648 13.8856C9.35205 13.9604 9.6714 14 10 14C10.2862 14 10.5653 13.9699 10.8345 13.9128ZM6.1581 11.1172C6.05517 10.7626 6 10.3878 6 10C6 9.66814 6.04041 9.34571 6.11659 9.03738L6.0378 9.11617L4.50608 7.58444C4.18066 8.32349 4 9.14065 4 10C4 10.9539 4.2226 11.8558 4.61868 12.6566L6.1581 11.1172ZM7.24631 4.66782C8.07101 4.24105 9.00735 4 10 4C10.9539 4 11.8558 4.2226 12.6566 4.61868L11.1172 6.1581C10.7626 6.05517 10.3878 6 10 6C9.57119 6 9.15814 6.06748 8.77088 6.19239L7.24631 4.66782ZM12 10C12 11.1046 11.1046 12 10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSwitchHorizontal;
impl IconShape for HiSwitchHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5C7.44772 5 7 5.44771 7 6C7 6.55228 7.44772 7 8 7L13.5858 7L12.2929 8.29289C11.9024 8.68342 11.9024 9.31658 12.2929 9.70711C12.6834 10.0976 13.3166 10.0976 13.7071 9.70711L16.7071 6.70711C16.8946 6.51957 17 6.26522 17 6C17 5.73478 16.8946 5.48043 16.7071 5.29289L13.7071 2.29289C13.3166 1.90237 12.6834 1.90237 12.2929 2.29289C11.9024 2.68342 11.9024 3.31658 12.2929 3.70711L13.5858 5L8 5Z",
            }
            path {
                d: "M12 15C12.5523 15 13 14.5523 13 14C13 13.4477 12.5523 13 12 13L6.41421 13L7.70711 11.7071C8.09763 11.3166 8.09763 10.6834 7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929L3.29289 13.2929C3.10536 13.4804 3 13.7348 3 14C3 14.2652 3.10536 14.5196 3.29289 14.7071L6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071C8.09763 17.3166 8.09763 16.6834 7.70711 16.2929L6.41421 15L12 15Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSwitchVertical;
impl IconShape for HiSwitchVertical {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 12C5 12.5523 5.44771 13 6 13C6.55228 13 7 12.5523 7 12L7 6.41421L8.29289 7.70711C8.68342 8.09763 9.31658 8.09763 9.70711 7.70711C10.0976 7.31658 10.0976 6.68342 9.70711 6.29289L6.70711 3.29289C6.51957 3.10536 6.26522 3 6 3C5.73478 3 5.48043 3.10536 5.29289 3.29289L2.29289 6.29289C1.90237 6.68342 1.90237 7.31658 2.29289 7.70711C2.68342 8.09763 3.31658 8.09763 3.70711 7.70711L5 6.41421L5 12Z",
            }
            path {
                d: "M15 8C15 7.44772 14.5523 7 14 7C13.4477 7 13 7.44772 13 8L13 13.5858L11.7071 12.2929C11.3166 11.9024 10.6834 11.9024 10.2929 12.2929C9.90237 12.6834 9.90237 13.3166 10.2929 13.7071L13.2929 16.7071C13.4804 16.8946 13.7348 17 14 17C14.2652 17 14.5196 16.8946 14.7071 16.7071L17.7071 13.7071C18.0976 13.3166 18.0976 12.6834 17.7071 12.2929C17.3166 11.9024 16.6834 11.9024 16.2929 12.2929L15 13.5858L15 8Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTable;
impl IconShape for HiTable {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 4C3.34315 4 2 5.34315 2 7V13C2 14.6569 3.34315 16 5 16H15C16.6569 16 18 14.6569 18 13V7C18 5.34315 16.6569 4 15 4H5ZM4 13V12H9V14H5C4.44772 14 4 13.5523 4 13ZM11 14H15C15.5523 14 16 13.5523 16 13V12H11V14ZM11 10H16V8H11V10ZM9 8H4V10H9V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTag;
impl IconShape for HiTag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L2.29289 10.7071C2.0976 10.5118 1.99997 10.2558 2 9.99988V5C2 3.34315 3.34315 2 5 2H10.0003C10.2561 2.00007 10.5119 2.0977 10.7071 2.29289L17.7071 9.29289ZM5 6C5.55228 6 6 5.55228 6 5C6 4.44772 5.55228 4 5 4C4.44772 4 4 4.44772 4 5C4 5.55228 4.44772 6 5 6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTemplate;
impl IconShape for HiTemplate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 4C3 3.44772 3.44772 3 4 3H16C16.5523 3 17 3.44772 17 4V6C17 6.55228 16.5523 7 16 7H4C3.44772 7 3 6.55228 3 6V4Z",
            }
            path {
                d: "M3 10C3 9.44771 3.44772 9 4 9H10C10.5523 9 11 9.44771 11 10V16C11 16.5523 10.5523 17 10 17H4C3.44772 17 3 16.5523 3 16V10Z",
            }
            path {
                d: "M14 9C13.4477 9 13 9.44771 13 10V16C13 16.5523 13.4477 17 14 17H16C16.5523 17 17 16.5523 17 16V10C17 9.44771 16.5523 9 16 9H14Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTerminal;
impl IconShape for HiTerminal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V5ZM5.29289 6.29289C5.68342 5.90237 6.31658 5.90237 6.70711 6.29289L9.70711 9.29289C10.0976 9.68342 10.0976 10.3166 9.70711 10.7071L6.70711 13.7071C6.31658 14.0976 5.68342 14.0976 5.29289 13.7071C4.90237 13.3166 4.90237 12.6834 5.29289 12.2929L7.58579 10L5.29289 7.70711C4.90237 7.31658 4.90237 6.68342 5.29289 6.29289ZM11 12C10.4477 12 10 12.4477 10 13C10 13.5523 10.4477 14 11 14H14C14.5523 14 15 13.5523 15 13C15 12.4477 14.5523 12 14 12H11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiThumbDown;
impl IconShape for HiThumbDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 9.5C18 10.3284 17.3285 11 16.5 11C15.6716 11 15 10.3284 15 9.5V3.5C15 2.67157 15.6716 2 16.5 2C17.3285 2 18 2.67157 18 3.5V9.5Z",
            }
            path {
                d: "M14 9.66667V4.23607C14 3.47852 13.572 2.786 12.8945 2.44721L12.8446 2.42229C12.2892 2.14458 11.6767 2 11.0558 2L5.63964 2C4.68628 2 3.86545 2.67292 3.67848 3.60777L2.47848 9.60777C2.23097 10.8453 3.17755 12 4.43964 12H8.00004V16C8.00004 17.1046 8.89547 18 10 18C10.5523 18 11 17.5523 11 17V16.3333C11 15.4679 11.2807 14.6257 11.8 13.9333L13.2 12.0667C13.7193 11.3743 14 10.5321 14 9.66667Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiThumbUp;
impl IconShape for HiThumbUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 10.5C2 9.67157 2.67157 9 3.5 9C4.32843 9 5 9.67157 5 10.5V16.5C5 17.3284 4.32843 18 3.5 18C2.67157 18 2 17.3284 2 16.5V10.5Z",
            }
            path {
                d: "M6 10.3333V15.7639C6 16.5215 6.428 17.214 7.10557 17.5528L7.15542 17.5777C7.71084 17.8554 8.32329 18 8.94427 18H14.3604C15.3138 18 16.1346 17.3271 16.3216 16.3922L17.5216 10.3922C17.7691 9.15465 16.8225 8 15.5604 8H12V4C12 2.89543 11.1046 2 10 2C9.44772 2 9 2.44772 9 3V3.66667C9 4.53215 8.71929 5.37428 8.2 6.06667L6.8 7.93333C6.28071 8.62572 6 9.46785 6 10.3333Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+If6W3eIcPeS3+B/u5H/um3+FFh3gRvPk7f/Bru/FZhtfmfwHBb6vwOT//w9/827xwiH/Bm77jB38W5rP530h89i/+6Dd/Di8Y4oV403f44PcGvov/3d7nF3/sm7+b5w/xArz1e3/08fFw/XTj4/wvJrTbbc4e8tPf/dW7PC/EC/Cm7/DB7w18F/83vM8v/tg3fzfPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/Am77jB3825rP4v0B8zi/+6Dd/Ns8L8QK86Tt+8GdjPov/C8Tn/OKPfvNn87wQL8CbvuMHfzbms/i/QHzOL/7oN382zwvxArzpO37wZ2M+i/8LxOf84o9+82fzvBAvwJu+4wd/Nuaz+L9AfM4v/ug3fzbPC/ECvOk7fvBnYz6L/wvE5/zij37zZ/O8EC/AW7zjh7x1sz8aeGngGP87XQL+ukhf/XM/+k0/zfNCvAje7J0+5KOd/mzgGP87XFLos3/hR77pq3nhEC+iN32HD31pyN8GjvE/2yWI1/7FH/vGv+ZfhvhXePN3/uDXzsZv8T9YFF7n53/4m3+bFw3iX+lN3+GDfxt4Lf5n+p1f/LFvfm1edIh/pTd9hw9+b+C7+J/pfX7xx775u3nRIf6V3vydP/i1s/Fb/A8Uhdf5+R/+5t/mRYf4V3rzd/7g187Gb/E/UBRe5+d/+Jt/mxcd4l/pTd/hg98b+C7+Z3qfX/yxb/5uXnSIf6U3e4cP/i3Da/M/kOC3f+HHvvl1eNEh/hXe/J0/+LWz8Vv8DxaF1/n5H/7m3+ZFg3gRvek7fOhLC/+W8XH+BxPaNXqdX/yxb/xr/mWIF8Gbv8OHfpTxZxsf538BoV2hz/75H/vGr+GFQ7wAb/GOH/LWaX8U6KWNj/O/kNAu+K9D+pqf+9Fv+mmeF+IFeNN3/ODPxnwW/xeIz/nFH/3mz+Z5IV6AN33HD/5szGfxf4H4nF/80W/+bJ4X4gV403f84M/GfBb/F4jP+cUf/ebP5nkhXoA3fccP/mzMZ/F/gficX/zRb/5snhfiBXjTd/zgz8Z8Fv8XiM/5xR/95s/meSFegDd9xw/+bMxn8X+B+Jxf/NFv/myeF+IFeNN3/ODPxnwW/xeIz/nFH/3mz+Z5IV6AN32HD35v4Lv4P6BIb/NzP/pNP83zQrwAb/3eH318OFzdChzjf7dL/eb8wT/93V+9y/NCvBBv+g4f/N7Ad/G/2/v84o9983fz/CH+BW/6jh/82ZjP4n8j8Tm/+KPf/Nm8YIgXwZu/8we/djY+Gngr/nf4mSh89c//8Df/Ni8c4l/pzd/5g1+b/8F+/oe/+bd50SH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b/wgy4Q1QClRjEgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTicket;
impl IconShape for HiTicket {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H16C17.1046 4 18 4.89543 18 6V8C16.8954 8 16 8.89543 16 10C16 11.1046 16.8954 12 18 12V14C18 15.1046 17.1046 16 16 16H4C2.89543 16 2 15.1046 2 14V12C3.10457 12 4 11.1046 4 10C4 8.89543 3.10457 8 2 8V6Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTranslate;
impl IconShape for HiTranslate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.00001 2C7.55229 2 8.00001 2.44772 8.00001 3V4H8.73223C8.744 3.99979 8.75581 3.99979 8.76765 4H11C11.5523 4 12 4.44772 12 5C12 5.55228 11.5523 6 11 6H9.57801C9.21635 7.68748 8.63076 9.29154 7.85405 10.7796C8.14482 11.1338 8.44964 11.476 8.76767 11.8055C9.15124 12.2028 9.14007 12.8359 8.74272 13.2195C8.34537 13.603 7.7123 13.5919 7.32873 13.1945C7.13962 12.9986 6.95468 12.7987 6.77405 12.5948C5.88895 13.9101 4.84387 15.1084 3.66692 16.1618C3.2554 16.5301 2.6232 16.4951 2.25487 16.0836C1.88655 15.672 1.92157 15.0398 2.3331 14.6715C3.54619 13.5858 4.60214 12.3288 5.4631 10.9389C4.90663 10.1499 4.40868 9.31652 3.97558 8.44503C3.7298 7.95045 3.93148 7.35027 4.42606 7.10449C4.92064 6.8587 5.52083 7.06039 5.76661 7.55497C6.00021 8.02502 6.25495 8.48278 6.52961 8.92699C6.947 7.99272 7.28247 7.01402 7.52698 6H3.00001C2.44772 6 2.00001 5.55228 2.00001 5C2.00001 4.44772 2.44772 4 3.00001 4H6.00001V3C6.00001 2.44772 6.44772 2 7.00001 2ZM13 8C13.3788 8 13.725 8.214 13.8944 8.55279L16.8854 14.5348C16.8919 14.5471 16.8982 14.5596 16.9041 14.5722L17.8944 16.5528C18.1414 17.0468 17.9412 17.6474 17.4472 17.8944C16.9532 18.1414 16.3526 17.9412 16.1056 17.4472L15.382 16H10.618L9.89444 17.4472C9.64745 17.9412 9.04677 18.1414 8.5528 17.8944C8.05882 17.6474 7.85859 17.0468 8.10558 16.5528L9.09589 14.5722C9.10187 14.5596 9.1081 14.5471 9.11458 14.5348L12.1056 8.55279C12.275 8.214 12.6212 8 13 8ZM11.618 14H14.382L13 11.2361L11.618 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrash;
impl IconShape for HiTrash {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9 2C8.62123 2 8.27497 2.214 8.10557 2.55279L7.38197 4H4C3.44772 4 3 4.44772 3 5C3 5.55228 3.44772 6 4 6L4 16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V6C16.5523 6 17 5.55228 17 5C17 4.44772 16.5523 4 16 4H12.618L11.8944 2.55279C11.725 2.214 11.3788 2 11 2H9ZM7 8C7 7.44772 7.44772 7 8 7C8.55228 7 9 7.44772 9 8V14C9 14.5523 8.55228 15 8 15C7.44772 15 7 14.5523 7 14V8ZM12 7C11.4477 7 11 7.44772 11 8V14C11 14.5523 11.4477 15 12 15C12.5523 15 13 14.5523 13 14V8C13 7.44772 12.5523 7 12 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrendingDown;
impl IconShape for HiTrendingDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 13C11.4477 13 11 13.4477 11 14C11 14.5523 11.4477 15 12 15H17C17.5523 15 18 14.5523 18 14V9C18 8.44772 17.5523 8 17 8C16.4477 8 16 8.44772 16 9V11.5858L11.7071 7.29289C11.3166 6.90237 10.6834 6.90237 10.2929 7.29289L8 9.58579L3.70711 5.29289C3.31658 4.90237 2.68342 4.90237 2.29289 5.29289C1.90237 5.68342 1.90237 6.31658 2.29289 6.70711L7.29289 11.7071C7.68342 12.0976 8.31658 12.0976 8.70711 11.7071L11 9.41421L14.5858 13H12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAGaklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+B/mLd7xQ966mc8CvzT/XtJPY33OL/7YN/41zx/if5A3f+cPfu1s/Bb/gYR2u83ZQ376u796l+eF+B/kTd/hg38aeCv+o4nP+cUf/ebP5nkh/gd503f44N8GXov/aOJzfvFHv/mzeV6I/0He7J0+5KOd/ir+o4nP+cUf/ebP5nkh/od503f44J8G3or/SOJzfvFHv/mzeV6I/4He/J0/+LVtvbTt47wAgre2eSleFOJzfvFHv/mzeV6I/4Xe9B0++L2B7+JFJT7nF3/0mz+b54X4X+ZN3+GD3xv4Lv41xOf84o9+82fzvBD/yd70HT70pQk9iPQzfvHHvvGv+Xd403f44PcGvot/LfE5v/ij3/zZPC/Ef6I3e8cP+SrbH839pJ/uN2bv89Pf/dW7/Cu96Tt88HsD38W/hficX/zRb/5snhfiP8mbvsMHfxfw3jwP/XW/OXudn/7ur97lRfSm7/DB7w18Fy+ExPfYPBh4LZ6b+Jxf/NFv/myeF+I/wZu+wwd/F/DevED6635z9jo//d1fvcu/4E3f4YPfG/guXgiJ7/mFH/3m937Td/jg3wZei+cmPucXf/SbP5vnhfgP9qbv8MHfBbw3/yL9db85e52f/u6v3uUFeNN3+OD3Br6LF0Lie37hR7/5vQHe9B0++LeB1+K5ic/5xR/95s/meSH+A73pO3zwdwHvzYtMf91vzl7np7/7q3d5Lm/6Dh/83sB38UJIfM8v/Og3vzfP9Kbv8MG/DbwWz018zi/+6Dd/Ns8L8R/kTd/hg78LeG/+1fTX/ebsdX76u796l2d603f44PcGvosXQuJ7fuFHv/m9eYA3fYcP/m3gtXhu4nN+8Ue/+bN5Xoj/AG/6Dh/8XcB782+mv+43Z6/z09/91btv+g4f/N7Ad/FCSHzPL/zoN783z+VN3+GDfxt4LZ6b+Jxf/NFv/myeF+Lf6U3f4YO/C3hvXhjxOZiPBo7xAumvA313kl/NCyHxPb/wo9/83jwfb/oOH/zbwGvx3MTn/OKPfvNn87wQ/w5v+g4f/F3Ae/PCvc8v/tg3f/ebvsOHvjTkbwPH+DeS+J5f+NFvfm9egDd9hw/+beC1eG7ic37xR7/5s3leiH+jN32HD/4u4L154d7nF3/sm7+bZ3rTd/jQl4b8beAY/0oS3/MLP/rN780L8abv8MG/DbwWz018zi/+6Dd/Ns8L8W/wpu/wwd8FvDcv3Pv84o9983fzXN70HT70pSF/GzjGi0jie37hR7/5vfkXvNk7fchHO/1VPJcovM7P//A3/zbPC/Gv9Kbv8MHfBbw3L9z7/OKPffN38wK86Tt86EtD/jZwjH+BxPf8wo9+83vzInrTd/jgnwbeimf7ml/8sW/+aJ4/xL/Cm77DB38X8N68cO/ziz/2zd/Nv+BN3+FDXxryt4FjvAAS3/MLP/rN782/0pu/8we/tlLHm3XrL/7YN/41LxjiRfSm7/DB3wW8Ny/c+/zij33zd/MietN3+NCXhvxt4BjPReJ7fuFHv/m9+c+FeBG86Tt88HcB780L9z6/+GPf/N38K73pO3zoS0P+NnCMZ5L4nl/40W9+b/7zIf4Fb/oOH/xdwHvzwr3PL/7YN383/0Zv/d4ffXw4Wn00QAS//fM//M2/zX8NxAvxpu/wwd8FvDcv3Pv84o9983fzvxPiBXjzd/7g187Gb/HCvc8v/tg3fzf/eyFegDd9xw/+bMxn8YK9zy/+2Dd/N/+7IV6AN33HD/5szGfx/L3PL/7YN383//shXoC3fu+PPj4crm4FjvGc3ucXf+ybv5v/GxAvxJu+w4e+NORnA28l8TeBPvvnfvSbfpr/OxD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+EcM/GdQioB1UAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrendingUp;
impl IconShape for HiTrendingUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 7C11.4477 7 11 6.55228 11 6C11 5.44772 11.4477 5 12 5H17C17.5523 5 18 5.44772 18 6V11C18 11.5523 17.5523 12 17 12C16.4477 12 16 11.5523 16 11V8.41421L11.7071 12.7071C11.3166 13.0976 10.6834 13.0976 10.2929 12.7071L8 10.4142L3.70711 14.7071C3.31658 15.0976 2.68342 15.0976 2.29289 14.7071C1.90237 14.3166 1.90237 13.6834 2.29289 13.2929L7.29289 8.29289C7.68342 7.90237 8.31658 7.90237 8.70711 8.29289L11 10.5858L14.5858 7H12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTruck;
impl IconShape for HiTruck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16.5C8 17.3284 7.32843 18 6.5 18C5.67157 18 5 17.3284 5 16.5C5 15.6716 5.67157 15 6.5 15C7.32843 15 8 15.6716 8 16.5Z",
            }
            path {
                d: "M15 16.5C15 17.3284 14.3284 18 13.5 18C12.6716 18 12 17.3284 12 16.5C12 15.6716 12.6716 15 13.5 15C14.3284 15 15 15.6716 15 16.5Z",
            }
            path {
                d: "M3 4C2.44772 4 2 4.44772 2 5V15C2 15.5523 2.44772 16 3 16H4.05001C4.28164 14.8589 5.29052 14 6.5 14C7.70948 14 8.71836 14.8589 8.94999 16H10C10.5523 16 11 15.5523 11 15V5C11 4.44772 10.5523 4 10 4H3Z",
            }
            path {
                d: "M14 7C13.4477 7 13 7.44772 13 8V14.05C13.1616 14.0172 13.3288 14 13.5 14C14.7095 14 15.7184 14.8589 15.95 16H17C17.5523 16 18 15.5523 18 15V10C18 9.73478 17.8946 9.48043 17.7071 9.29289L15.7071 7.29289C15.5196 7.10536 15.2652 7 15 7H14Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUpload;
impl IconShape for HiUpload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 17C3 16.4477 3.44772 16 4 16H16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H4C3.44772 18 3 17.5523 3 17ZM6.29289 6.70711C5.90237 6.31658 5.90237 5.68342 6.29289 5.29289L9.29289 2.29289C9.48043 2.10536 9.73478 2 10 2C10.2652 2 10.5196 2.10536 10.7071 2.29289L13.7071 5.29289C14.0976 5.68342 14.0976 6.31658 13.7071 6.70711C13.3166 7.09763 12.6834 7.09763 12.2929 6.70711L11 5.41421L11 13C11 13.5523 10.5523 14 10 14C9.44771 14 9 13.5523 9 13L9 5.41421L7.70711 6.70711C7.31658 7.09763 6.68342 7.09763 6.29289 6.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserAdd;
impl IconShape for HiUserAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 9C9.65685 9 11 7.65685 11 6C11 4.34315 9.65685 3 8 3C6.34315 3 5 4.34315 5 6C5 7.65685 6.34315 9 8 9Z",
            }
            path {
                d: "M8 11C11.3137 11 14 13.6863 14 17H2C2 13.6863 4.68629 11 8 11Z",
            }
            path {
                d: "M16 7C16 6.44772 15.5523 6 15 6C14.4477 6 14 6.44772 14 7V8H13C12.4477 8 12 8.44771 12 9C12 9.55228 12.4477 10 13 10H14V11C14 11.5523 14.4477 12 15 12C15.5523 12 16 11.5523 16 11V10H17C17.5523 10 18 9.55228 18 9C18 8.44772 17.5523 8 17 8H16V7Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserCircle;
impl IconShape for HiUserCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM12 7C12 8.10457 11.1046 9 10 9C8.89543 9 8 8.10457 8 7C8 5.89543 8.89543 5 10 5C11.1046 5 12 5.89543 12 7ZM9.99993 11C7.98239 11 6.24394 12.195 5.45374 13.9157C6.55403 15.192 8.18265 16 9.99998 16C11.8173 16 13.4459 15.1921 14.5462 13.9158C13.756 12.195 12.0175 11 9.99993 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserGroup;
impl IconShape for HiUserGroup {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 6C13 7.65685 11.6569 9 10 9C8.34315 9 7 7.65685 7 6C7 4.34315 8.34315 3 10 3C11.6569 3 13 4.34315 13 6Z",
            }
            path {
                d: "M18 8C18 9.10457 17.1046 10 16 10C14.8954 10 14 9.10457 14 8C14 6.89543 14.8954 6 16 6C17.1046 6 18 6.89543 18 8Z",
            }
            path {
                d: "M14 15C14 12.7909 12.2091 11 10 11C7.79086 11 6 12.7909 6 15V18H14V15Z",
            }
            path {
                d: "M6 8C6 9.10457 5.10457 10 4 10C2.89543 10 2 9.10457 2 8C2 6.89543 2.89543 6 4 6C5.10457 6 6 6.89543 6 8Z",
            }
            path {
                d: "M16 18V15C16 13.9459 15.7282 12.9552 15.2507 12.0943C15.4902 12.0327 15.7413 12 16 12C17.6569 12 19 13.3431 19 15V18H16Z",
            }
            path {
                d: "M4.74926 12.0943C4.27185 12.9552 4 13.9459 4 15V18H1V15C1 13.3431 2.34315 12 4 12C4.25871 12 4.50977 12.0327 4.74926 12.0943Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFlElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xn+it3/mDH9yaHpTiwWFubejSL/7YN/41/3Mg/hO86Tt+8GcB7415MM9N3Cr0093G7HN++ru/epf/Xoj/QG/6Dh/60ih/CvNg/gVCu8Yf84s/9s3fzX8fxH+QN3/nD35tN/2U8XH+dd7nF3/sm7+b/x6I/wBv+g4f+tLCv2V8nH+DIr3Nz/3oN/00//UQ/wHe7B0++LcMr82/lbj1F3/0mx/Cfz3Ev9Obv/MHv3Y2fot/J4U+5hd+5Ju+mv9aiH+nN32HD/5q4KP4d9Nf/+KPfdPL8F8L8e/0pu/wwb8NvBb/AX7xx75Z/NdC/Du96Tt8sPkP0hce8tM//M238l8H8e/0pu/wweY/SBRe5+d/+Jt/m/86iH+nN32HD74VeBD/AX7xx75Z/NdC/Du96Tt88E8Db8W/3zN+8ce++cH810L8O73pO3zwewPfxb/f1/zij33zR/NfC/Hv9Nbv/dHHh8PVXwMP4t+hLzzkp3/4m2/lvxbiP8BbvOOHvHWzf4p/u6/5xR/75o/mvx7iP8ibveMHf7fNe/GvJPE3v/Cj3/zSvABv+g4f+tKSX8r4wfwrCN1q629+8ce+8a95wRD/gd70HT74q4GP4kX3M/3m/L1/+ru/epfn483e8UO+yvZH8+8g6at/4Ue/6WN4/hD/wd78nT/4tbPx3cCDeMEuAR/9iz/2zd/NC/Dm7/zBr52N3+I/QBRe5+d/+Jt/m+eF+E/ypu/woS+N8q0xr82z/XWRfvvnfvSbfpp/wZu+4wd/Nuaz+I8gPucXf/SbP5vnhfgf6k3f8YM/G/NZ/AdQ6GN+4Ue+6at5Xoj/od76nT/4wUPjr4Fj/Ptc6jfnD/7p7/7qXZ4X4n+wN32HD31pyM8G3op/m5+B+Oxf/LFv/GueP8T/b4j/3xD/vyH+g731O3/wg8fUaxk/WOa1eS4Wvy10axf+nZ/+4W++lf9eiP8Ab/3eH318OhzeK/F7g1+aF5W4FfjuPvien/7hb76V/3qIf4e3fu+PPj4crT5K1kcbH+ff57v7zfnH/PR3f/Uu/3UQ/0Zv/s4f/NqZfBfmwfwHEdo1/phf/LFv/m7+ayD+Dd70HT/4szCfzX+e7+435x/z09/91bv850L8K73pO3zwdwHvzX86/XW/OXudn/7ur97lPw/iX+FN3+GDvwt4b/7L6K/7zdnr/PR3f/Uu/zkQL6I3e6cP+Winv4r/et/9iz/2ze/Dfw7Ei+BN3+FDXxryr/jv8z6/+GPf/N38x0O8CN70HT7kr8AvzX8Tod1uc/aQn/7ur97lPxbiX/Cm7/DB7w18F//NJL7nF370m9+b/1iIf8GbvuMHPx3zYP4H6AsP+ekf/uZb+Y+DeCHe4h0/5K2b/VP8TyE+5xd/9Js/m/84iBfizd7xg7/b5r34n0Lc+os/+s0P4T8O4oV4s3f4kIvGx/kfpC885Kd/+Jtv5T8G4gV403f40JeG/Cv+53mfX/yxb/5u/mMgXoA3fYcPfm/gu/ifRnzOL/7oN382/zEQL8CbvuMHfzbms/if53d+8ce++bX5j4F4Ad70HT/4szGfxf88v/OLP/bNr81/DMQL8Obv/MGvnclr8z+NufUXf+ybv5v/GIj/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/9/4R6kvWFBNOoTzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserRemove;
impl IconShape for HiUserRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 6C11 7.65685 9.65685 9 8 9C6.34315 9 5 7.65685 5 6C5 4.34315 6.34315 3 8 3C9.65685 3 11 4.34315 11 6Z",
            }
            path {
                d: "M14 17C14 13.6863 11.3137 11 8 11C4.68629 11 2 13.6863 2 17H14Z",
            }
            path {
                d: "M13 8C12.4477 8 12 8.44771 12 9C12 9.55229 12.4477 10 13 10H17C17.5523 10 18 9.55229 18 9C18 8.44771 17.5523 8 17 8H13Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFU0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x3+Ct3/ujj7fD9Uu1iOMAJXO3bM7+5qe/+6t3+a+F+C/05u/8wa+dqY/CfmueH+mnI/w1P//D3/zb/NdA/Bd46/f+6OPD4eqrgPfmRfPd/eb8Y376u796l/9ciP9kb/3eH318OFz/Fvil+VfRX/ebs9f56e/+6l3+8yD+k73pO37IT2G/Nf8W0k//4o9+09vwnwfxn+hN3+GD3xv4Lv4divQ2P/ej3/TT/OdA/Cd603f84KdjHsy/h7j1F3/0mx/Cfw7Ef5I3fYcPfWnIv+I/QF94yE//8Dffyn88xH+SN3unD/lop7+K/wAKfcwv/Mg3fTX/8RD/Sd70HT/4szGfxX8E8Tm/+KPf/Nn8x0P8J3nTd/zgz8Z8Fv8RxOf84o9+82fzHw/xn+RN3/GDPxvzWfxHEJ/ziz/6zZ/NfzzEf5K3eMcPeetm/xT/AYr0Nj/3o9/00/zHQ/wnetN3+OBd4Bj/Ppd+8ce++Tj/ORD/id70HT/4szGfxb+H+Jxf/NFv/mz+cyD+E731e3/08eFwdStwjH+bS/3m/ME//d1fvct/DsR/sjd9hw99aci/4l/vEsRr/+KPfeNf858H8V/gTd/hQ18a8reBY7xoLkG89i/+2Df+Nf+5EP9F3vqdP/jBY/LZNu/FCyHxPV3w2T/9w998K//5EP/F3vqdP/jBo/XWTr82cJwrdhX67U7+6Z/+4W++lf86iP/fEP+/If5/Q/z/hvj/DfH/G+K/yFu/8wc/eHB5KdxeGvTSso/zAJZ2wX+Nyl/3an/z0z/8zbfynw/xn+it3/mDHzwk74X11uCX5l9Ff438033wPT/9w998K/85EP8J3vQdPvSlIT8KeG/+Y3x3X/icn/7hb76V/1iI/0Bv/d4ffXw8Wn+W7Y/mP4Gkr+42Zp/z09/91bv8x0D8B3nzd/7g13bTTxkf5z+R0K6K3+bnf/ibf5t/P8R/gDd9hw9+b+C7+K/1Pr/4Y9/83fz7IP6d3vQdPvi7gPfmv8d3/+KPffP78G+H+Hd403f44K8GPor/TuJzfvFHv/mz+bdB/Bu96Tt88HsD38X/DO/ziz/2zd/Nvx7i3+BN3+FDX1r4t4yP8z+A0K7R6/zij33jX/Ovg/g3eNN3+JC/Ar80/6Por3/xx77pZfjXQfwrvdk7fchHO/1V/A+k0Mf8wo9801fzokP8K7z1e3/08fFw/XTj4/wPJLTbbc4e8tPf/dW7vGgQ/wpv+o4f/NmYz+J/MvE5v/ij3/zZvGgQ/wpv+o4f/HTMg/mfTNz6iz/6zQ/hRYN4Eb35O3/wa2fjt/hfIV7mF3/sG/+afxniRfSm7/DBXw18FP87fM0v/tg3fzT/MsSL6E3f4UP+CvzS/K+gv/7FH/uml+FfhngRvek7fLD5X+QXf+ybxb8M8SJ403f40JeG/Cv+V4mX+cUf+8a/5oVDvAje/J0/+LWz8Vv8LxKF1/n5H/7m3+aFQ7wI3vydP/i1s/Fb/C8Shdf5+R/+5t/mhUO8CN78nT/4tTN5bf43cfz0L/7YN/41Lxzi/zfE/2+I/98Q/78h/n9D/P/GPwIbNU1QBBECTwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUser;
impl IconShape for HiUser {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 9C11.6569 9 13 7.65685 13 6C13 4.34315 11.6569 3 10 3C8.34315 3 7 4.34315 7 6C7 7.65685 8.34315 9 10 9ZM3 18C3 14.134 6.13401 11 10 11C13.866 11 17 14.134 17 18H3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUsers;
impl IconShape for HiUsers {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 6C9 7.65685 7.65685 9 6 9C4.34315 9 3 7.65685 3 6C3 4.34315 4.34315 3 6 3C7.65685 3 9 4.34315 9 6Z",
            }
            path {
                d: "M17 6C17 7.65685 15.6569 9 14 9C12.3431 9 11 7.65685 11 6C11 4.34315 12.3431 3 14 3C15.6569 3 17 4.34315 17 6Z",
            }
            path {
                d: "M12.9291 17C12.9758 16.6734 13 16.3395 13 16C13 14.3648 12.4393 12.8606 11.4998 11.6691C12.2352 11.2435 13.0892 11 14 11C16.7614 11 19 13.2386 19 16V17H12.9291Z",
            }
            path {
                d: "M6 11C8.76142 11 11 13.2386 11 16V17H1V16C1 13.2386 3.23858 11 6 11Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVariable;
impl IconShape for HiVariable {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.6485 3.08366C5.15459 3.30478 5.3856 3.89429 5.16448 4.40038C4.41582 6.11389 4 8.00707 4 10C4 11.9929 4.41582 13.8861 5.16448 15.5996C5.3856 16.1057 5.15459 16.6952 4.6485 16.9164C4.14242 17.1375 3.5529 16.9065 3.33178 16.4004C2.47486 14.4391 2 12.2737 2 10C2 7.72632 2.47486 5.56091 3.33178 3.59964C3.5529 3.09355 4.14242 2.86254 4.6485 3.08366ZM12.9613 7C12.0499 7 11.188 7.41427 10.6186 8.12592L10.2911 8.53528L10.1799 8.25722C9.87619 7.4979 9.14078 7 8.32297 7H8C7.44772 7 7 7.44772 7 8C7 8.55228 7.44772 9 8 9H8.32297L8.8551 10.3303L7.81962 11.6247C7.62985 11.8619 7.34253 12 7.03875 12H7C6.44772 12 6 12.4477 6 13C6 13.5523 6.44772 14 7 14H7.03875C7.9501 14 8.81204 13.5857 9.38136 12.8741L9.70885 12.4647L9.82008 12.7428C10.1238 13.5021 10.8592 14 11.677 14H12C12.5523 14 13 13.5523 13 13C13 12.4477 12.5523 12 12 12H11.677L11.1449 10.6697L12.1804 9.37531C12.3702 9.13809 12.6575 9 12.9613 9H13C13.5523 9 14 8.55228 14 8C14 7.44772 13.5523 7 13 7H12.9613ZM14.8355 4.40038C14.6144 3.89429 14.8454 3.30478 15.3515 3.08366C15.8576 2.86254 16.4471 3.09355 16.6682 3.59964C17.5251 5.56091 18 7.72632 18 10C18 12.2737 17.5251 14.4391 16.6682 16.4004C16.4471 16.9065 15.8576 17.1375 15.3515 16.9164C14.8454 16.6952 14.6144 16.1057 14.8355 15.5996C15.5842 13.8861 16 11.9929 16 10C16 8.00707 15.5842 6.11389 14.8355 4.40038Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVideoCamera;
impl IconShape for HiVideoCamera {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H10C11.1046 4 12 4.89543 12 6V14C12 15.1046 11.1046 16 10 16H4C2.89543 16 2 15.1046 2 14V6Z",
            }
            path {
                d: "M14.5528 7.10557C14.214 7.27497 14 7.62123 14 8V12C14 12.3788 14.214 12.725 14.5528 12.8944L16.5528 13.8944C16.8628 14.0494 17.2309 14.0329 17.5257 13.8507C17.8205 13.6684 18 13.3466 18 13V7C18 6.65342 17.8205 6.33156 17.5257 6.14935C17.2309 5.96714 16.8628 5.95058 16.5528 6.10557L14.5528 7.10557Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewBoards;
impl IconShape for HiViewBoards {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 4C2 3.44772 2.44772 3 3 3H5C5.55228 3 6 3.44772 6 4V16C6 16.5523 5.55228 17 5 17H3C2.44772 17 2 16.5523 2 16V4Z",
            }
            path {
                d: "M8 4C8 3.44772 8.44772 3 9 3H11C11.5523 3 12 3.44772 12 4V16C12 16.5523 11.5523 17 11 17H9C8.44772 17 8 16.5523 8 16V4Z",
            }
            path {
                d: "M15 3C14.4477 3 14 3.44772 14 4V16C14 16.5523 14.4477 17 15 17H17C17.5523 17 18 16.5523 18 16V4C18 3.44772 17.5523 3 17 3H15Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewGridAdd;
impl IconShape for HiViewGridAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z",
            }
            path {
                d: "M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z",
            }
            path {
                d: "M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z",
            }
            path {
                d: "M14 11C14.5523 11 15 11.4477 15 12V13H16C16.5523 13 17 13.4477 17 14C17 14.5523 16.5523 15 16 15H15V16C15 16.5523 14.5523 17 14 17C13.4477 17 13 16.5523 13 16V15H12C11.4477 15 11 14.5523 11 14C11 13.4477 11.4477 13 12 13H13V12C13 11.4477 13.4477 11 14 11Z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewGrid;
impl IconShape for HiViewGrid {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z",
            }
            path {
                d: "M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z",
            }
            path {
                d: "M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z",
            }
            path {
                d: "M11 13C11 11.8954 11.8954 11 13 11H15C16.1046 11 17 11.8954 17 13V15C17 16.1046 16.1046 17 15 17H13C11.8954 17 11 16.1046 11 15V13Z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAD7UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3xr/DW7/zBDx5cXgq3l+Z/IpW/7tX+5qd/+Jtv5UWDeBG96Tt88HsLfZXxcf4HE9o1/phf/LFv/m7+ZYgXwVu/8wc/eGz6K+Pj/C8gtNsVv8xP//A338oLh3gRvMU7fshbN/un+F+kSG/zcz/6TT/NC4d4Ebz5O3/wa2fjt/hfJAqv8/M//M2/zQuHeBG92Tt+8F/bvBT/C0j8zS/86De/NP8yxIvord/7o48Ph6vvBl4bOMb/TJckfrrbmH/0T3/3V+/yL0P8/4b4/w3x/xvi/zfE/2+I/98Q/wpv/s4f/NqZvBb/g0XwOz//w9/827xoEC+iN32HD/5q4KP43+FrfvHHvvmj+ZchXgRv+g4f+tKQf8X/KvEyv/hj3/jXvHCIF8FbvOOHvHWzf4r/RYr0Nj/3o9/007xwiBfBm7/zB792Nn6L/1XiZX7xx77xr3nhEC+iN32HD/5t4LX43+FnfvHHvvmt+Zch/hXe7J0+5KOdfmv+B1Pop3/hR77pq3nRIP5/Q/z/hvj/DfH/G+L/N8T/b4h/hTd/hw/9KJNvzf9gIn7653/sG7+GFw3iRfRm7/DBv2V4bf43kH76F3/0m96GfxniRfDm7/zBr52N3+J/kSi8zs//8Df/Ni8c4kXwFu/4IW/d7J/if5Eivc3P/eg3/TQvHOJF8Kbv8KEvDflX/K8SL/OLP/aNf80Lh3gRvek7fPBXAx/F/w5f84s/9s0fzb8M8a/w5u/8wa+dyWvzP1gEv/3zP/zNv82LBvH/G+L/N8T/b4j/3xD/vyH+f0O8iN76nT/4wUPqq2Re2/g4/wMJ7Vr8dr8xe5+f/u6v3uVfhngRvek7fMhfgV+a/xX017/4Y9/0MvzLEC+CN3/nD37tbPwW/4tE4XV+/oe/+bd54RAvgrd4xw9562b/FP+LFOltfu5Hv+mneeEQL4K3fucPfvDQ+GvgGP87XOoLL/3TP/zNt/LCIV5Eb/oOH/zewFcDx/if7RLw0b/4Y9/83fzLEP8Kb/3OH/zglnrphl+a/4EK+usS/uuf/uFvvpUXDeL/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zf+EWw/10E1thHZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewList;
impl IconShape for HiViewList {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H16C16.5523 3 17 3.44772 17 4C17 4.55228 16.5523 5 16 5H4C3.44772 5 3 4.55228 3 4ZM3 8C3 7.44772 3.44772 7 4 7H16C16.5523 7 17 7.44772 17 8C17 8.55228 16.5523 9 16 9H4C3.44772 9 3 8.55228 3 8ZM3 12C3 11.4477 3.44772 11 4 11H16C16.5523 11 17 11.4477 17 12C17 12.5523 16.5523 13 16 13H4C3.44772 13 3 12.5523 3 12ZM3 16C3 15.4477 3.44772 15 4 15H16C16.5523 15 17 15.4477 17 16C17 16.5523 16.5523 17 16 17H4C3.44772 17 3 16.5523 3 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVolumeOff;
impl IconShape for HiVolumeOff {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.38268 3.07615C9.75636 3.23093 10 3.59557 10 4.00003V16C10 16.4045 9.75636 16.7691 9.38268 16.9239C9.00901 17.0787 8.57889 16.9931 8.29289 16.7071L4.58579 13H2C1.44772 13 1 12.5523 1 12V8.00003C1 7.44774 1.44772 7.00003 2 7.00003H4.58579L8.29289 3.29292C8.57889 3.00692 9.00901 2.92137 9.38268 3.07615Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M12.2929 7.29289C12.6834 6.90237 13.3166 6.90237 13.7071 7.29289L15 8.58579L16.2929 7.29289C16.6834 6.90237 17.3166 6.90237 17.7071 7.29289C18.0976 7.68342 18.0976 8.31658 17.7071 8.70711L16.4142 10L17.7071 11.2929C18.0976 11.6834 18.0976 12.3166 17.7071 12.7071C17.3166 13.0976 16.6834 13.0976 16.2929 12.7071L15 11.4142L13.7071 12.7071C13.3166 13.0976 12.6834 13.0976 12.2929 12.7071C11.9024 12.3166 11.9024 11.6834 12.2929 11.2929L13.5858 10L12.2929 8.70711C11.9024 8.31658 11.9024 7.68342 12.2929 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVolumeUp;
impl IconShape for HiVolumeUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.38268 3.07615C9.75636 3.23093 10 3.59557 10 4.00003V16C10 16.4045 9.75636 16.7691 9.38268 16.9239C9.00901 17.0787 8.57889 16.9931 8.29289 16.7071L4.58579 13H2C1.44772 13 1 12.5523 1 12V8.00003C1 7.44774 1.44772 7.00003 2 7.00003H4.58579L8.29289 3.29292C8.57889 3.00692 9.00901 2.92137 9.38268 3.07615Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M14.6568 2.92888C15.0474 2.53836 15.6805 2.53836 16.0711 2.92888C17.8796 4.73743 19 7.2388 19 9.99995C19 12.7611 17.8796 15.2625 16.0711 17.071C15.6805 17.4615 15.0474 17.4615 14.6568 17.071C14.2663 16.6805 14.2663 16.0473 14.6568 15.6568C16.1057 14.208 17 12.2094 17 9.99995C17 7.79053 16.1057 5.7919 14.6568 4.34309C14.2663 3.95257 14.2663 3.3194 14.6568 2.92888ZM11.8284 5.75731C12.2189 5.36678 12.8521 5.36678 13.2426 5.75731C13.7685 6.28319 14.1976 6.90687 14.5003 7.59958C14.822 8.33592 15 9.14847 15 9.99995C15 11.6565 14.3273 13.1579 13.2426 14.2426C12.8521 14.6331 12.2189 14.6331 11.8284 14.2426C11.4379 13.8521 11.4379 13.2189 11.8284 12.8284C12.5534 12.1034 13 11.1048 13 9.99995C13 9.42922 12.8811 8.8889 12.6676 8.40032C12.4663 7.93958 12.1802 7.52327 11.8284 7.17152C11.4379 6.781 11.4379 6.14783 11.8284 5.75731Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiWifi;
impl IconShape for HiWifi {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M17.7781 8.22183C13.4823 3.92606 6.51752 3.92606 2.22176 8.22183C1.83123 8.61235 1.19807 8.61235 0.807542 8.22183C0.417017 7.8313 0.417017 7.19814 0.807542 6.80761C5.88436 1.7308 14.1155 1.7308 19.1923 6.80761C19.5828 7.19814 19.5828 7.8313 19.1923 8.22183C18.8018 8.61235 18.1686 8.61235 17.7781 8.22183ZM14.9497 11.0503C12.216 8.31659 7.78385 8.31659 5.05018 11.0503C4.65966 11.4408 4.02649 11.4408 3.63597 11.0503C3.24544 10.6597 3.24544 10.0266 3.63597 9.63605C7.15069 6.12133 12.8492 6.12133 16.3639 9.63605C16.7544 10.0266 16.7544 10.6597 16.3639 11.0503C15.9734 11.4408 15.3402 11.4408 14.9497 11.0503ZM12.1213 13.8787C10.9497 12.7071 9.05018 12.7071 7.87861 13.8787C7.48809 14.2692 6.85492 14.2692 6.4644 13.8787C6.07387 13.4882 6.07387 12.855 6.4644 12.4645C8.41702 10.5119 11.5828 10.5119 13.5355 12.4645C13.926 12.855 13.926 13.4882 13.5355 13.8787C13.1449 14.2692 12.5118 14.2692 12.1213 13.8787ZM8.99993 16C8.99993 15.4477 9.44765 15 9.99993 15H10.0099C10.5622 15 11.0099 15.4477 11.0099 16C11.0099 16.5523 10.5622 17 10.0099 17H9.99993C9.44765 17 8.99993 16.5523 8.99993 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiXCircle;
impl IconShape for HiXCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8.70711 7.29289C8.31658 6.90237 7.68342 6.90237 7.29289 7.29289C6.90237 7.68342 6.90237 8.31658 7.29289 8.70711L8.58579 10L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071C7.68342 13.0976 8.31658 13.0976 8.70711 12.7071L10 11.4142L11.2929 12.7071C11.6834 13.0976 12.3166 13.0976 12.7071 12.7071C13.0976 12.3166 13.0976 11.6834 12.7071 11.2929L11.4142 10L12.7071 8.70711C13.0976 8.31658 13.0976 7.68342 12.7071 7.29289C12.3166 6.90237 11.6834 6.90237 11.2929 7.29289L10 8.58579L8.70711 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiX;
impl IconShape for HiX {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10 8.58579L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L11.4142 10L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L10 11.4142L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L8.58579 10L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiZoomIn;
impl IconShape for HiZoomIn {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 8C5 7.44772 5.44772 7 6 7H7V6C7 5.44772 7.44772 5 8 5C8.55228 5 9 5.44772 9 6V7H10C10.5523 7 11 7.44772 11 8C11 8.55228 10.5523 9 10 9H9V10C9 10.5523 8.55228 11 8 11C7.44771 11 7 10.5523 7 10V9H6C5.44772 9 5 8.55228 5 8Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8ZM8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiZoomOut;
impl IconShape for HiZoomOut {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4ZM2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 8C5 7.44772 5.44772 7 6 7H10C10.5523 7 11 7.44772 11 8C11 8.55228 10.5523 9 10 9H6C5.44772 9 5 8.55228 5 8Z",
                fill_rule: "evenodd",
            }
        }
    }
}
