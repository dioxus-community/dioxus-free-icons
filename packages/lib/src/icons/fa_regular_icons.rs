use super::super::IconShape;
use dioxus::prelude::*;

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaAddressBook;
impl IconShape for FaAddressBook {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M272 288h-64C163.8 288 128 323.8 128 368C128 376.8 135.2 384 144 384h192c8.836 0 16-7.164 16-16C352 323.8 316.2 288 272 288zM240 256c35.35 0 64-28.65 64-64s-28.65-64-64-64c-35.34 0-64 28.65-64 64S204.7 256 240 256zM496 320H480v96h16c8.836 0 16-7.164 16-16v-64C512 327.2 504.8 320 496 320zM496 64H480v96h16C504.8 160 512 152.8 512 144v-64C512 71.16 504.8 64 496 64zM496 192H480v96h16C504.8 288 512 280.8 512 272v-64C512 199.2 504.8 192 496 192zM384 0H96C60.65 0 32 28.65 32 64v384c0 35.35 28.65 64 64 64h288c35.35 0 64-28.65 64-64V64C448 28.65 419.3 0 384 0zM400 448c0 8.836-7.164 16-16 16H96c-8.836 0-16-7.164-16-16V64c0-8.838 7.164-16 16-16h288c8.836 0 16 7.162 16 16V448z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAE/klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIjn9WDgo4DXBl6a/x/+Gvht4GuAW3k2xHP6KuCj+f/tq4GP4QrEs/0V8NJcBfDXwMsAiCu+GvgornqgrwE+WsCDgadz1fPzEAFfDXwUVz0/XyPgr4CX5nn9DvDewK383/Zg4LuB1+J5/bUA8/w9BLiV/x8eDDyd54UA8/yJ/1/M80KAef7E/y/meSHAPH/i/xfzvBBgnj/x/4t5Xggwz5/493kw8FHAawMvzRV/Dfw28DXArfzPYp4XAszzJ/7tvgr4aF64rwY+hv85zPNCgHn+xL/NXwEvzYvmr4GX4X8G87wQYJ4/8a/31cBH8a/zNcBH86L7KuCtgQfzr3Mr8NPAx/D8meeFAPP8iX+dBwNP59/mIcCt/Mu+Gvgo/n0+B/hsnpd5Xggwz5/41/lq4KP4t/ka4KP5l10EjvPvcyvwEJ6XeV4IMM+f+Nf5K+Cl+bf5a+Bl+JftAsf493kG8GCel3leCDDPn/jXMf8+4l/22cBn8e/zOcBn87zM80KAef7Ev4759xEvms8G3ht4EP86zwC+G/hsnj/zvBBgnj/xr/PXwEvxb/M3wEvz38s8LwSY50/863w18FH823wN8NH89zLPCwHm+RP/Og8Gns6/zUOAW/nvZZ4XAszzJ/71vhr4KP51vgb4aP77meeFAPP8iX+bvwZeihfN3wAvzb/OVwFvDTyYf51bgZ8GPobnzzwvBJjnT/zbfTXwUbxwXwN8NP86Xw18FP8+nwN8Ns/LPC8EmOdP/Ps8GPho4LWBl+KKvwF+G/hq4Fb+9S4Cx/n3uRV4CM/LPC8EmOdP/M+zCxzj3+cZwIN5XuZ5IcA8f+J/ns8GPot/n88BPpvnZZ4XAszzJ/5n+mzgvYEH8a/zDOC7gc/m+TPPCwHm+RP/v5jnhQDz/Il/nfcCXht4aeCledH8NfDXwG8D38N/L/O8EGCeP/GieWvgq4AH8+9zK/AxwE/z38M8LwSY50/8y74K+Gj+Y3018DH81zPPCwHm+RMv3FcDH8V/jq8BPpr/WuZ5IcA8f+IFe2vgp/jP9TbAT/NfxzwvBJjnT7xgTwcezH+uW4GH8F/HPC8EmOdPPH/vDXwX/zXeBvhp/muY54UA8/yJ5++7gffiv8b3AO/Nfw3zvBBgnj/x/P0V8NL81/hr4GX4r2GeFwLM8yeeP/NfS/zXMM8LAeb5E8/fLnCM/xrPAB7Mfw3zvBBgnj/x/L028N3Ag/jP9QzgvYHf5r+GeV4IMM+f+P/FPC8EmOdP/P9inhcCzPMn/n8xzwsB5vkT/7+Y54UA8/ydAHb5/+HBwNN5Xgj4a+CleF6/DbwPcCv/tz0Y+C7gtXlefyPgq4GP4qrn52sEPBh4Olc9Pw8RV3w18FFc9UBfA3y0eLa/Bl6KqwD+BnhpAPGcvhr4KP5/+xrgo7kC8bweDHw08NrAS/H/w98Avw18NXArz4a46oVBXPXCIK56YRBXvTD8I5F20hYPI4rnAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaAddressCard;
impl IconShape for FaAddressCard {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M208 256c35.35 0 64-28.65 64-64c0-35.35-28.65-64-64-64s-64 28.65-64 64C144 227.3 172.7 256 208 256zM464 232h-96c-13.25 0-24 10.75-24 24s10.75 24 24 24h96c13.25 0 24-10.75 24-24S477.3 232 464 232zM240 288h-64C131.8 288 96 323.8 96 368C96 376.8 103.2 384 112 384h192c8.836 0 16-7.164 16-16C320 323.8 284.2 288 240 288zM464 152h-96c-13.25 0-24 10.75-24 24s10.75 24 24 24h96c13.25 0 24-10.75 24-24S477.3 152 464 152zM512 32H64C28.65 32 0 60.65 0 96v320c0 35.35 28.65 64 64 64h448c35.35 0 64-28.65 64-64V96C576 60.65 547.3 32 512 32zM528 416c0 8.822-7.178 16-16 16H64c-8.822 0-16-7.178-16-16V96c0-8.822 7.178-16 16-16h448c8.822 0 16 7.178 16 16V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFAAAABACAYAAACNx/A2AAAIMUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjiq4C3Bo4DXw18DbDLf58HA18FvDRX/DXwMcCt/M+CgM8GPovn9NfA2wC38l/vwcBfAcd5TrvAywC38j8HAi4Cx3leu8D7AD/Nf62fBt6K5+9ngLfmfw4E3Ao8iBfss4HP4b/OReA4z9+twEP4nwMBXw18FC/cTwPvA+zyn8+8cOJ/DsQV3w28Fy/crcDbAH/Nfy7zwon/ORDP9tHAV/HC7QIfA3w3/7FeGngt4LWBt+aF+27gt4HfAW7lvxfiOb008NvAMV64rwY+hn+f48B7AR8NPJh/m1uBzwa+h/8eiOd1HPhp4LV44f4aeB1gl3+9jwI+GzjOf4xd4LOBr+G/FuIF+2rgo3jhdoG3AX6bF81LA98FvDT/OX4beB/gVv5rIF649wa+GjjGC/fRwNfwwr038FXAcf5z7QJvA/w2//kQ/7KXBr4beCleuO8GPgbY5Xm9N/Bd/Nd6H+C7+c+FeNEcB74beCteuL8G3gf4a57trYGf4l/2N8BPA78N/DWwy3N6aeA48NbAWwMP4l/2NsBP858H8a/z2cBn8cLtAu8D/DTw0sBvAcd5wX4H+Gzgt/nXeW3gs4HX4gXbBV4GuJX/HIh/vdcGfho4xgv32cBbAy/N83cJ+Gzgq/n3+Wjgq3jBfht4Hf5zIP5tHgz8NPBS/NtcAl4b+Gv+Y7w28NPAMZ6/9wG+m/94iH+f7wbei3+dS8BrA3/Nf6zXBn6L5+9W4CH8x0P8+7038F286D4G+Gr+c3w08FU8f+8DfDf/sRD/MV4a+GngQbxwtwIvA+zyn+e3gdfief018DL8x0L8xzkO/DTwWrxwfw28DXAr/zleG/gtnr+HALfyHwfxH++rgY/ihdsF3gb4bf5z3Ao8iOf1PsB38x8H8Z/jrYHvBo7xwn028Dn8x/tq4KN4Xj8DvDX/cRD/eV4a+G7gpXjhfhp4H2CX/zivDfwWz+t3gNfmPw7iP9dx4KuB9+KFuxV4G+Cv+Y/x0sBf8bz+GngZ/uMg/mt8NPBVvHC7wMcA381/DPP8if84iP86rw38NHCMF+67gffh3888f+I/DuK/1nHgt4GX4oX7a+BtgFv5tzPPn/iPg/iPdxzY5YX7buC9eOF2gbcBfpt/G/P8if84iP94fwW8DXArL9x7A18NHOOF+2jga/jXM8+f+I+D+I9nYBd4G+C3eeFeGvhp4EG8cD8NvA+wy4vOPH/iPw7iP9aDgafzbJ8NfA4v3HHgu4G34oX7a+B9gL/mRWOevxPALv8xEP+xXhv4LZ7TTwPvA+zywn028Fm8cLvAxwDfzb9sFzjG83od4Lf5j4H4j/XawG/xvH4GeGv+ZW8NfDdwjBfuq4GP4YX7beC1eF6vA/w2/zEQ/7HeG/guntfPAG/Ni+bBwE8DL8UL99fA6wC7PH8/DbwVz+t9gO/mPwbiP9Z3A+/F8/oc4LN50R0Hvhp4L164XeBtgN/meX028Fk8r+8B3pv/GIj/WBeB4zyvtwF+mn+9jwa+in/ZRwNfw3N6a+CneF63Ag/hPwbiP85LA3/F83cC2OXf5qWB3waO8cJ9N/AxwC5XHAcu8vy9DPDX/Psh/uN8NPBVPK+/AV6af5/jwE8Dr8UL99fA+wB/zRV/DbwUz+tjgK/m3w/xH+fpwIN5Xp8DfDb/Mb4a+CheuF3gfYCfBr4a+Cie163AQ/j3Q/zHeG/gu3j+Xgb4a/7jvDfw1cAxXrivBr4H+Cuev/cBvpt/H8R/jN8CXpvn9TfAS/Mf76WB7wZeihfut4GHAA/ief028Dr8+yD+/V4b+C2ev/cBvpv/HMeB7wbein+71wF+m387xL/PceCvgAfzvC4Bx/nP99nAZ/FvcyvwMsAu/zaIf5+vBj6K5+9zgM/mv8ZrAz8NHONf72uAj+bfBvFv99rAb/H8PQN4MP+1Hgz8NPBS/Ou9DvDb/Osh/m0eDPwVcJzn722An+a/x3cD78W/zi7wMsCt/Osg/vWOA78FvDT/9X4aeB9glxfuvYHv4l/nr4HXAXZ50SH+9T4b+Cz++9wKvA3w17xwLw38NPAgXnSfA3w2LzrEv95F4Dj/vXaBjwG+mxfuOPDTwGvxorkVeAgvOsS/3i5wjP8Zvhr4GP5lXw18FP+yS8BxXnSIf72vBj6K/zn+GngdYJcX7q2B7waO8cJ9NPA1vGgQ/zZfDbw3cIz/GXaB1wH+mhfupYHvBl6KF+6ngfcBdnnhEP//HAe+GngvXri/Bt4H+GteMMT/Xx8NfBUv3C7wMcB38/wh/n97beCngWO8cF8NfAzPC3HVceC3gZfihftt4G2AXZ4N8d/P/PuI/xjfDbwXL9wu8DrAX3MF4r/fXwMvxb/N3wAvzX+c9wa+GjjGC/c+wHcDiP9+Xw18FP82XwN8NP+xXhr4aeBBvHDfDbyP+O/3YODp/Ns8BLiV/3jHge8G3ooX7mvE/wxfDXwU/zpfA3w0/7k+G/gsXjDE/xx/DbwUL5q/AV6a/xpvDXw3cIzndUn8z/LVwEfxwn0N8NH813ow8NPAS/GcPkf8z/Ng4KOB1wZeiiv+Bvht4KuBW/nvcRz4auC9uOJrgI/+R7UNSE+ub1utAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaBellSlash;
impl IconShape for FaBellSlash {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M183.6 118.6C206.5 82.58 244.1 56.84 288 49.88V32C288 14.33 302.3 .0003 320 .0003C337.7 .0003 352 14.33 352 32V49.88C424.5 61.39 480 124.2 480 200V233.4C480 278.8 495.5 322.9 523.8 358.4L538.7 377C543.1 383.5 545.4 392.2 542.6 400L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L183.6 118.6zM221.7 148.4L450.7 327.1C438.4 298.2 432 266.1 432 233.4V200C432 142.6 385.4 96 328 96H312C273.3 96 239.6 117.1 221.7 148.4V148.4zM160 233.4V222.1L206.7 258.9C202.7 297.7 189.5 335.2 168.3 368H345.2L406.2 416H120C110.8 416 102.4 410.7 98.37 402.4C94.37 394.1 95.5 384.2 101.3 377L116.2 358.4C144.5 322.9 160 278.8 160 233.4V233.4zM384 448C384 464.1 377.3 481.3 365.3 493.3C353.3 505.3 336.1 512 320 512C303 512 286.7 505.3 274.7 493.3C262.7 481.3 256 464.1 256 448H384z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAFTklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv8WDgq4CX5oq/Bj4GuJX/XIj/fA8G/go4znPaBV4GuJX/PIj/fD8NvBXP388Ab81/HsR/vovAcZ6/W4GH8J8H8Z/PvHDiPw/iP5954cR/HsR/jgcDrwW8NvDevHDfDfw28DvArfzHQvzHei/gs4EH829zK/DZwPfwHwPxH+OjgM8GjvMfYxf4bOBr+PdB/Ps8GPgu4LX5z/HbwPsAt/Jvg/i3e2vgu4Dj/OfaBd4G+G3+9RD/Nu8NfBf/td4H+G7+dRD/em8N/BT/smcAPw38NLAL/DXP6aWB48BbA28NPIh/2dsAP82LDvGv82Dgr4DjvGC/A3w28Nv867w28NnAa/GC7QIvA9zKiwbxr/NbwGvzgn0M8NX8+3w08FW8YL8NvA4vGsSL7qOBr+L5uwS8NfDb/Md4beCngWM8f+8DfDf/MsSL7unAg3n+Xgf4bf5jvTbwWzx/twIP4V+GeNG8N/BdPH8fA3w1/zk+Gvgqnr/3Ab6bFw7xonk68GCe1+8Ar81/rt8GXovn9dfAy/DCIf5lDwaezvP3OsBv85/rtYHf4vl7CHArLxjiX/bewHfxvJ4BPJj/GrcCD+J5vQ/w3bxgiH/ZTwNvxfP6GuCj+a/x1cBH8bx+BnhrXjDEv+y3gdfieb0O8Nv813ht4Ld4Xr8DvDYvGOJf9nTgwTyvlwH+mv8aLw38Fc/rr4GX4QVD/MvM8yf+a5nnT7xgiH+Zef7Efy3z/IkXDPEvM8+f+K9lnj/xgiH+Zeb5E/+1zPMnXjDEv8w8f+K/lnn+xAuG+JeZ50/81zLPn3jBEC/cg4Gn8/yJ/1rm+TsB7PL8IV641wZ+i+f1O8Br81/rt4HX4nm9DvDbPH+IF+61gd/ief0O8Nr81/pt4LV4Xq8D/DbPH+KFe2/gu3hePwO8Nf+1fhp4K57X+wDfzfOHeOG+G3gvntfnAJ/Nf63PBj6L5/U9wHvz/CFeuIvAcZ7X6wC/zX+t1wZ+i+d1K/AQnj/EC/bSwF/x/In/eseBizx/LwP8Nc8L8YJ9NPBVPK+/AV6a/x5/DbwUz+tjgK/meSFesKcDD+Z5fQ3w0fz3+Grgo3hetwIP4Xkhnr/3Br6L5+8hwK3893gw8HSev/cBvpvnhHj+fgt4bZ7X3wAvzX+vvwZeiuf128Dr8JwQz+u1gd/i+Xsf4Lv57/XewHfx/L0O8Ns8G+I5HQf+Cngwz+sS8GBgl/9ex4FbgWM8r1uBlwF2uQLxnL4a+Ciev/cBvpv/Gd4b+C6ev68BPporEM/22sBv8fw9A3gw/7PcCjyI5+91gN8GEFc8GPgr4DjP3+sAv83/LG8N/BTP3y7wMsCtAo4DvwW8NP+3/DXwOgK+C3hv/m/6HAHm/65dAeb/rksCvhr4KP5v+hpxxVcD7w0c4/+GS8B3Ax8t/m9D/N+G+L8N8X8b4j+e+fcR/3EQ//H+Gngp/m3+Bnhp/uMg/uN9NfBR/Nt8DfDR/MdB/Md7MPB0/m0eAtzKfxzEf46vBj6Kf52vAT6a/1iI/zx/DbwUL5q/AV6a/3iI/1xfDXwUL9zXAB/Nfw7Ef74HAx8NvDbwUlzxN8BvA18N3Mp/Hv4RpSe+QA/2iiIAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaBell;
impl IconShape for FaBell {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 32V49.88C328.5 61.39 384 124.2 384 200V233.4C384 278.8 399.5 322.9 427.8 358.4L442.7 377C448.5 384.2 449.6 394.1 445.6 402.4C441.6 410.7 433.2 416 424 416H24C14.77 416 6.365 410.7 2.369 402.4C-1.628 394.1-.504 384.2 5.26 377L20.17 358.4C48.54 322.9 64 278.8 64 233.4V200C64 124.2 119.5 61.39 192 49.88V32C192 14.33 206.3 0 224 0C241.7 0 256 14.33 256 32V32zM216 96C158.6 96 112 142.6 112 200V233.4C112 281.3 98.12 328 72.31 368H375.7C349.9 328 336 281.3 336 233.4V200C336 142.6 289.4 96 232 96H216zM288 448C288 464.1 281.3 481.3 269.3 493.3C257.3 505.3 240.1 512 224 512C207 512 190.7 505.3 178.7 493.3C166.7 481.3 160 464.1 160 448H288z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonhOHwW8N/DS/M/018BXA9/DFYgrjgO/Bbw0/zv8NfA6wK644q+Al+Z/l78GXkbARwNfxf9O7yPgr4CX5n+nvxZg/vdCgHn+Xgb4a/5neGngr3heCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/x7/Ng4Lu44n2AW/n3Mc8LAeb5E/82x4GPAj6b5/TZwNcAu/zbmOeFAPP8iX+99wK+GjjO87cLfDTwPfzrmeeFAPP8iRfdawOfBbw2L5rfBj4H+G1edOZ5IcA8f+Jfdhz4KuC9+bf5buBjgF3+ZeZ5IcA8f+KF+yzgo4Hj/PvsAl8NfA4vnHleCDDPn3j+Xhv4LuDB/Me6FXgf4Ld5/szzQoB5/sRzejDwXcBr86L5G+CjueKrgZfiRfPbwPsAt/KczPNCgHn+xBXHgY8CPpsXzSXgo4Hv5jl9NPDZwDFeNJ8NfA2wyxXmeSHAPH8C3gv4auA4L5qvAT4b2OX5Ow58NvBRvGh2gY8GvgcwzwsB5vn7beC1edH8DvDewK28aF4a+GrgtXjR/Dbw2jwvBJh/u2cAHw38NP82bw18NfAg/m0QYP71LgFfDXw2/37HgY8GPho4xr8OAsy/zvcAnw3cyn+sBwOfDbwXLzoE3Ao8iH/Z3wAfDfw2/7leG/hq4KX4lz1DwE8Db8ULdgn4aOC7+a/13sBXA8d4wX5GwIOBvwaO8by+BvhsYJf/HseBzwY+iud1CXiwuOLBwGcDrw0cB34a+GzgVv5neDDw2cBrA7vAXwMfDez+I8KbvdmhAVAyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaBookmark;
impl IconShape for FaBookmark {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M336 0h-288C21.49 0 0 21.49 0 48v431.9c0 24.7 26.79 40.08 48.12 27.64L192 423.6l143.9 83.93C357.2 519.1 384 504.6 384 479.9V48C384 21.49 362.5 0 336 0zM336 452L192 368l-144 84V54C48 50.63 50.63 48 53.1 48h276C333.4 48 336 50.63 336 54V452z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAE3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5n+Gvgd8Gvga4lWdDPKevAj6a/9m+GvgYrkA8218BL83/Dn8NvAyAuOKrgY/if5evAT5awIOBp/O/00MEfDXwUfzv9DUC/gp4aZ7X7wDvDdzKf68HA98NvBbP668FmOfvIcCt/M/wYODpPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ5/XawHcBD+b5uxV4H+C3ef5eG/gu4ME8f7cC7wP8Ns/LPC8EmOdPPK+nAw/mhbsVeAjP39OBB/PC3Qo8hOdlnhcCzPMnnpd50Yjnz7xoxPMyzwsB5vkTz8u8aMTzZ1404nmZ54UA8/yJ52VeNOL5My8a8bzM80KAef7E8zIvGvH8mReNeF7meSHAPH/ieZkXjXj+zItGPC/zvBBgnj/xvMyLRjx/5kUjnpd5Xggwz594XrvAMV64ZwAP5vnbBY7xwj0DeDDPyzwvBJjnTzyv1wa+G3gQz98zgPcGfpvn77WB7wYexPP3DOC9gd/meZnnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ57XawPfBTyY5+9W4H2A3+b5e23gu4AH8/zdCrwP8Ns8L/O8EGCeP/G8ng48mBfuVuAhPH9PBx7MC3cr8BCel3leCDDPn3he5kUjnj/zohHPyzwvBJjnTzwv86IRz5950YjnZZ4XAszzJ56XedGI58+8aMTzMs8LAeb5E8/LvGjE82deNOJ5meeFAPP8iedlXjTi+TMvGvG8zPNCgHn+xPMyLxrx/JkXjXhe5nkhwDx/4nntAsd44Z4BPJjnbxc4xgv3DODBPC/zvBBgnj/xvF4b+G7gQTx/zwDeG/htnr/XBr4beBDP3zOA9wZ+m+dlnhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8270X8NHAS3PFXwNfDXwP/3bmeSHAPH/iX+848FvAS/P8/TXwOsAu/3rmeSHAPH/iX++vgJfmhftr4GX41zPPCwHm+RP/Oh8NfBUvmvcBvpt/HfO8EGCeP/Gv81fAS/Oi+WvgZfjXMc8LAeb5E/865l9H/OuY54UA8/yJfx3zryP+dczzQoB5/sS/jvnXEf865nkhwDx/4l/H/OuIfx3zvBBgnj/xr2P+dcS/jnleCDDPn/jXMf864l/HPC8EmOdP/OuYfx3xr2OeFwLM83cC2OVFZ/51xIvuwcDTeV4I+GvgpXhevw28D3ArLxrzryNeNA8Gvgt4bZ7X3wj4auCj+N/pawQ8GHg6/zs9RFzx1cBH8b/L1wAfLZ7tr4GX4n+HvwFeGkA8p68GPor/2b4G+GiuQDyvBwMfDbw28FL8z/A3wG8DXw3cyrPxj5hv3g62SKm6AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaBuilding;
impl IconShape for FaBuilding {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M88 104C88 95.16 95.16 88 104 88H152C160.8 88 168 95.16 168 104V152C168 160.8 160.8 168 152 168H104C95.16 168 88 160.8 88 152V104zM280 88C288.8 88 296 95.16 296 104V152C296 160.8 288.8 168 280 168H232C223.2 168 216 160.8 216 152V104C216 95.16 223.2 88 232 88H280zM88 232C88 223.2 95.16 216 104 216H152C160.8 216 168 223.2 168 232V280C168 288.8 160.8 296 152 296H104C95.16 296 88 288.8 88 280V232zM280 216C288.8 216 296 223.2 296 232V280C296 288.8 288.8 296 280 296H232C223.2 296 216 288.8 216 280V232C216 223.2 223.2 216 232 216H280zM0 64C0 28.65 28.65 0 64 0H320C355.3 0 384 28.65 384 64V448C384 483.3 355.3 512 320 512H64C28.65 512 0 483.3 0 448V64zM48 64V448C48 456.8 55.16 464 64 464H144V400C144 373.5 165.5 352 192 352C218.5 352 240 373.5 240 400V464H320C328.8 464 336 456.8 336 448V64C336 55.16 328.8 48 320 48H64C55.16 48 48 55.16 48 64z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEcElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/89jgMfBXw0sAt8N/A5/MvM80KAef7Ef4/vAt6b5/QxwFfzwpnnhQDz/In/et8FvDfP61bgIbxw5nkhwDx/4r/WdwHvzfP3DODBvHDmeSHAPH/iv853Ae/NC/Y+wHfzwpnnhQDz/In/Gt8FvDcv2PcA782/zDwvBJjnT7xwx4GPAj4a2AW+G/gc/nW+C3hvXrDvAd6bF415Xggwz5944T4b+Cye03cD78OL5ruA9+YF+x7gvXnRmeeFAPP8iRfuInCc5/XdwPvwwn0X8N68YN8DvDf/OuZ5IcA8f+KFuxV4EM/fdwPvw/P3XcB784J9D/De/OuZ54UA8/yJF+6zgc/iBftu4H14Tt8FvDcv2PcA782/jXleCDDPn/iXfTfwXrxg3w28D1d8F/DevGDfA7w3/3bmeSHAPH/iRfPdwHvxgn03V7w3L9j3AO/Nv495Xggwz5940X038F7823wP8N78+5nnhQDz/Il/ne8G3ot/ne8B3pv/GOZ5IcA8f+Jf77uB9+JF8z3Ae/MfxzwvBJjnT/zbfDfwXrxw3wO8N/+xzPNCgHn+xL/ddwPvxfP3PcB78x/PPC8EmOdP/Pt8N/BePKfvAd6b/xzmeSHAPH/i3++jgY/mis8Gvpv/POZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzdwLY5X+HBwNP53kh4K+Bl+J5/TbwPsCt/M/2YOC7gNfmef2NgK8GPor/m75GwIOBp/N/00PEFV8NfBT/t3wN8NHi2f4aeCn+b/gb4KUBxHP6auCj+N/ta4CP5grE83ow8NHAawMvxf8OfwP8NvDVwK08G/8I6A+5FoIaJN0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendarCheck;
impl IconShape for FaCalendarCheck {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M216.1 408.1C207.6 418.3 192.4 418.3 183 408.1L119 344.1C109.7 335.6 109.7 320.4 119 311C128.4 301.7 143.6 301.7 152.1 311L200 358.1L295 263C304.4 253.7 319.6 253.7 328.1 263C338.3 272.4 338.3 287.6 328.1 296.1L216.1 408.1zM128 0C141.3 0 152 10.75 152 24V64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0zM400 192H48V448C48 456.8 55.16 464 64 464H384C392.8 464 400 456.8 400 448V192z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAD10lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RMvGvP8iX8d8/yJF415Xggwz5940ZjnT/zrmOdPvGjM80KAef7Ei8Y8f+Jfxzx/4kVjnhcCzPMnXjTm+RP/Oub5Ey8a87wQYJ4/8aIxz5/41zHPn3jRmOeFAPP8iReNef7Ev455/sSLxjwvBJjnT7xozPMn/nXM8ydeNOZ5IcD834UA838XAsz/XQgw/3chwPzfhQDzfxcCzPMnXjTm+RP/Oub5Ey8a87wQYJ4/8aIxz5/41zHPn3jRmOeFAPP8iReNef7Ev455/sSLxjwvBJjnT7xozPMn/nXM8ydeNOZ5IcA8f+JFY54/8a9jnj/xojHPCwHm+RMvGvP8iX8d8/yJF415Xggwz5940ZjnT/zrmOdPvGjM80KAef7Ei8Y8f+Jfxzx/4kVjnhcCzP9dCDD/dyHA/N+FAPN/FwLM/10IMP93IcA8f+JFY54/8a9jnj/xojHPCwHm+RMvGvP8iX8d8/yJF415Xggwz5940ZjnT/zrmOdPvGjM80KAef7Ei8Y8f+Jfxzx/4kVjnhcCzPMnXjTm+RP/Oub5Ey8a87wQYJ6/E8Au/zLz/Il/HfP8iX/Zg4Gn87wQ8NfAS/G8fht4H+BWXjjz/Il/HfP8iRfuwcB3Aa/N8/obAV8NfBT/N32NgAcDT+f/poeIK74a+Cj+b/ka4KPFs/018FL83/A3wEsDiOf01cBH8b/b1wAfzRWI5/Vg4KOB1wZeiv8d/gb4beCrgVt5Nv4R8s2iFrloWocAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendarDays;
impl IconShape for FaCalendarDays {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M152 64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0C141.3 0 152 10.75 152 24V64zM48 248H128V192H48V248zM48 296V360H128V296H48zM176 296V360H272V296H176zM320 296V360H400V296H320zM400 192H320V248H400V192zM400 408H320V464H384C392.8 464 400 456.8 400 448V408zM272 408H176V464H272V408zM128 408H48V448C48 456.8 55.16 464 64 464H128V408zM272 192H176V248H272V192z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAADjElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdPvGiOA58FvDXwYP5j3Ar8NPAxvOjM80KAef7Ei+argY/iP8fXAB/Ni8Y8LwSY50+8aC4Cx/nPcSvwEF405nkhwDx/4kWzCxzjP8czgAfzojHPCwHm+RMvmq8GPor/HJ8DfDYvGvO8EGCeP/Gi+2rgrYEH8R/jGcB3A5/Ni848LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+TgC7/O/wYODpPC8E/DXwUjyv3wbeB7iV/9keDHwX8No8r78R8NXAR/F/09cIeDDwdP5veoi44quBj+L/lq8BPlo8218DL8X/DX8DvDSAeE5fDXwU/7t9DfDRXIF4Xg8GPhp4beCl+N/hb4DfBr4auJVn4x8B1reQFnN74bsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendarMinus;
impl IconShape for FaCalendarMinus {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M152 352C138.7 352 128 341.3 128 328C128 314.7 138.7 304 152 304H296C309.3 304 320 314.7 320 328C320 341.3 309.3 352 296 352H152zM128 0C141.3 0 152 10.75 152 24V64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0zM400 192H48V448C48 456.8 55.16 464 64 464H384C392.8 464 400 456.8 400 448V192z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEDElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/iPcxz4LOCtueKngY/hP5Z5Xggwz5/4j/PVwEfxnL4G+Gj+45jnhQDz/In/OBeB4zynW4GH8B/HPC8EmOdP/Mcxz5/4j2OeFwLM8yf+45jnT/zHMc8LAeb5E/9xzPMn/uOY54UA8/yJ/zjm+RP/cczzQoB5/sR/HPP8if845nkhwDx/4j+Oef7EfxzzvBBgnj/xojkOfBbw1sCD+Y9xK/DTwMfwojPPCwHm+RMvmq8GPor/HF8DfDQvGvO8EGCeP/GiuQgc5z/HrcBDeNGY54UA8/yJF80ucIz/HM8AHsyLxjwvBJjnT7xovhr4KP5zfA7w2bxozPNCgHn+xIvuq4G3Bh7Ef4xnAN8NfDYvOvO8EGCeP/Efxzx/4j+OeV4IMM+f+I9jnj/xH8c8LwSY50/8xzHPn/iPY54XAszzJ/7jmOdP/McxzwsB5vkT/3HM8yf+45jnhQDz/In/OOb5E/9xzPNCgHn+xH+cXeAYz+kZwIP5j2OeFwLM8yf+43w18FE8p88BPpv/OOZ5IcA8f+I/1lcDb80V3w18Nv+xzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM/fCWCX/x0eDDyd54WAvwZeiuf128D7ALfyP9uDge8CXpvn9TcCvhr4KP5v+hoBDwaezv9NDxFXfDXwUfzf8jXAR4tn+2vgpfi/4W+AlwYQz+mrgY/if7evAT6aKxDP68HARwOvDbwU/zv8DfDbwFcDt/Js/CNb+agWVIjyMwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendarPlus;
impl IconShape for FaCalendarPlus {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M224 232C237.3 232 248 242.7 248 256V304H296C309.3 304 320 314.7 320 328C320 341.3 309.3 352 296 352H248V400C248 413.3 237.3 424 224 424C210.7 424 200 413.3 200 400V352H152C138.7 352 128 341.3 128 328C128 314.7 138.7 304 152 304H200V256C200 242.7 210.7 232 224 232zM152 64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0C141.3 0 152 10.75 152 24V64zM48 448C48 456.8 55.16 464 64 464H384C392.8 464 400 456.8 400 448V192H48V448z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAE00lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT7zojgMfBXw0sAt8N/A5vOiOAx8FfDSwC3w38Dn865jnhQDz/IkX3WcDn8Vz+m7gfXjRfBfw3jynjwG+mhedeV4IMM+feNFdBI7zvL4beB9euO8C3pvndSvwEF505nkhwDx/4kV3K/Agnr/vBt6H5++7gPfm+XsG8GBedOZ5IcA8f+JF99nAZ/GCfTfwPjyn7wLemxfsfYDv5kVnnhcCzPMn/nW+G3gvXrDvBt6HK74LeG9esO8B3pt/HfO8EGCeP/Gv993Ae/GCfTdXvDcv2PcA782/nnleCDDPn/i3+W7gvfi3+R7gvfm3Mc8LAeb5E/923w28F/863wO8N/925nkhwDx/4t/nu4H34kXzPcB78+9jnhcCzPMn/v2+G3gvXrjvAd6bfz/zvBBgnj/x7/ddwHvzwn038D78+5nnhQDz/Il/n+8C3psXzXcD78O/j3leCDDPn/i3+y7gvfnX+W7gffi3M88LAeb5E/823wW8N/823w28D/825nkhwDx/4l/nOPBbwEvzgn0PV7wXL9h3A+/Dv555Xggwz5940R0Hfgt4aV6w7wHemyu+G3gvXrDvBt6Hfx3zvBBgnj/xovts4LN4wb4HeG+e03cD78UL9jHAV/OiM88LAeb5Ey+6i8Bxnr/vAd6b5++7gffi+bsVeAgvOvO8EGCeP/GiuxV4EM/re4D35oX7buC9eF7PAB7Mi848LwSY50+86D4b+Cye0/cA782L5ruB9+I5vQ/w3bzozPNCgHn+xL/ORwMfzRWfDXw3/zofDXw0V3w28N3865jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDP3wlgl/8dHgw8neeFgL8GXorn9dvA+wC38j/bg4HvAl6b5/U3Ar4a+Cj+b/oaAQ8Gns7/TQ8RV3w18FH83/I1wEeLZ/tr4KX4v+FvgJcGEM/pq4GP4n+3rwE+misQz+vBwEcDrw28FP87/A3w28BXA7fybPwjnJjKFq8UlaUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendarXmark;
impl IconShape for FaCalendarXmark {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M257.9 328L304.1 375C314.3 384.4 314.3 399.6 304.1 408.1C295.6 418.3 280.4 418.3 271 408.1L224 361.9L176.1 408.1C167.6 418.3 152.4 418.3 143 408.1C133.7 399.6 133.7 384.4 143 375L190.1 328L143 280.1C133.7 271.6 133.7 256.4 143 247C152.4 237.7 167.6 237.7 176.1 247L224 294.1L271 247C280.4 237.7 295.6 237.7 304.1 247C314.3 256.4 314.3 271.6 304.1 280.1L257.9 328zM128 0C141.3 0 152 10.75 152 24V64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0zM400 192H48V448C48 456.8 55.16 464 64 464H384C392.8 464 400 456.8 400 448V192z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAADPklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3+yrgrbnip4GP4T/WVwFvzRU/DXwM/3qIf5uvBj6K5/Q5wGfzH+OrgY/iOX0O8Nn86yD+bS4Cx3lOtwIP4T/GReA4z+lW4CH86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIP5tzPMn/mOY50/86yD+bczzJ/5jmOdP/Osg/m3M8yf+Y5jnT/zrIJ7Xg4GPAl4beGn+d/hr4LeBrwFu5dkQz+mrgI/mf7evBj6GKxDP9lfAS/N/w18DLwMgrvhq4KP4v+VrgI8W8GDg6fzf9BABXw18FP83fY2AvwJemv+b/lqA+b8LAeb/LgSY/7sQYP7vQoD5vwsB5v8uBJj/uxBg/u9CgPm/CwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5OwHs8r/Dg4Gn87wQ8NfAS/G8fht4H+BW/md7MPBdwGvzvP5GwFcDH8X/TV8j4MHA0/m/6SHiiq8GPor/W74G+GjxbH8NvBT/N/wN8NIA4jl9NfBR/O/2NcBHcwXieT0Y+GjgtYGX4n+HvwF+G/hq4FaejX8EOPmCFvLurk4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCalendar;
impl IconShape for FaCalendar {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M152 64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0C141.3 0 152 10.75 152 24V64zM48 448C48 456.8 55.16 464 64 464H384C392.8 464 400 456.8 400 448V192H48V448z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4grvgp4a674buBz+P8BAV8NfBTP6XOAz+b/PgRcBI7znG4FHsL/fQgwz5/4vw8B5vkT//chwDx/4v8+BJjnT/zfhwDz/In/+xBgnj/xfx8CzPMn/u9DgHn+xIvmOPBZwFsDD+a/xq3ATwMfw78PAszzJ140Xw18FP89vgb4aP7tEGCeP/GiuQgc57/HrcBD+LdDgHn+xItmFzjGf49nAA/m3w4B5vkTL5qvBj6K/x6fA3w2/3YIMM+feNF9NfDWwIP4r/EM4LuBz+bfBwHm+RP/9yHAPH/i/z4EmOdP/N+HAPP8if/7EGCeP/F/HwLM8yf+70OAef7Ei+Y48FnAWwMP5j/GrcBPAx/Dfy4EmOdPvGi+Gvgo/nN8DfDR/OdBgHn+xIvmInCc/xy3Ag/hPw8CzPMnXjS7wDH+czwDeDD/eRBgnj/xovlq4KP4z/E5wGfznwcB5vkTL7qvBt4aeBD/MZ4BfDfw2fznQoB5/sT/fQgwz5/4vw8B5vkT//chwDx/4v8+BJjnT/zfhwDz/In/+xBgnj/xojkOfBbw1sCD+Z/hVuCngY/hhUOAef7Ei+argY/if6avAT6aFwwB5vkTL5qLwHH+Z7oVeAgvGALM8ydeNLvAMf5negbwYF4wBJjnT7xovhr4KP5n+hzgs3nBEGCeP/Gi+2rgrYEH8T/DM4DvBj6bFw4B5vkT//chwDx/4v8+BJjnT/zfhwDz/In/+xBgnj/xfx8CzPMn/u9DgHn+xP99CDDP3wlgl//bEPDXwEvxvD4G+Gr+b0PAdwPvxfP32cD3ALfyfxMCXhv4Lf5/uRX4aeBjxBW/DbwW//98jbjipYHfBo7x/8ut4tleGvht4Bj/fzxDPKeXBr4aeC3+f/gc8fy9NvDewEsDL8X/Pc8Avhv4bPH/G+L/N8T/b4j/3/hHySNwEIM6DnUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChartBar;
impl IconShape for FaChartBar {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M24 32C37.25 32 48 42.75 48 56V408C48 421.3 58.75 432 72 432H488C501.3 432 512 442.7 512 456C512 469.3 501.3 480 488 480H72C32.24 480 0 447.8 0 408V56C0 42.75 10.75 32 24 32zM128 136C128 122.7 138.7 112 152 112H360C373.3 112 384 122.7 384 136C384 149.3 373.3 160 360 160H152C138.7 160 128 149.3 128 136zM296 208C309.3 208 320 218.7 320 232C320 245.3 309.3 256 296 256H152C138.7 256 128 245.3 128 232C128 218.7 138.7 208 152 208H296zM424 304C437.3 304 448 314.7 448 328C448 341.3 437.3 352 424 352H152C138.7 352 128 341.3 128 328C128 314.7 138.7 304 152 304H424z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACgAAABACAYAAABoWTVaAAAFm0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi3eTDwVcBLAw/mhbsV+GvgY4Bb+ddB/Os9GPgr4Dj/OrvAywC38qJD/Ov9NPBW/Nv8DPDWvOgQ/3oXgeP829wKPIQXHeJfbxc4xr/NJeA4LzrEv953A+/Fv833AO/Niw7xr/dg4K+BY/zrXAJeGriVFx3i3+Y48NnAR/Gi+Rrgs4Fd/nUQ/3bfBbw3L5rvBt6Hfz3Ev813Ae/Nv853A+/Dvw7iX++7gPfm3+a7gffhRYf413lv4Lv49/kY4Kt50SBedC8N/BUv2CXgo7nio4GX4gV7HeC3+ZchXjTHgacDx3n+LgGvDfw1VxwHfhp4LZ6/XeAhwC4vHOJF81PAW/P8XQJeG/hrntNx4LeBl+L5+2ngbXjhEP+ytwZ+ihfsZYC/5vk7DtwKHOP5+xjgq3nBEC/cceDpwHGev/cBvpsX7rWB3+L52wUeAuzy/CFeuJ8C3prn73uA9+ZF89nAZ/H8/TTwNjx/iBfstYHf4vl7BvDSwC4vur8GXorn73WA3+Z5IV6w3wJem+fvZYC/5l/npYG/4vn7a+BleF6I5++9ge/i+fsa4KP5t/lq4KN4/t4H+G6eE+L5ezrwYJ7XM4CXBnb5tzkO3Aoc43ndCjyE54R4Xu8NfBfP39sAP82/z3sD38Xz9z7Ad/NsiOf1dODBPK/fAV6bf7/vAt6b5+9W4CE8G+I5vTfwXTx/rwP8Nv8+3wW8Ny/c+wDfzRWI5/RbwGvzvL4HeG/+fb4LeG/+ZX8NvAxXIJ7tpYG/4vl7CHAr/3bfBbw3L7qXAf4aQDzbVwMfxfP6HeC1+bf7LuC9+df5HuC9AcSzXQSO87zeBvhp/m2+C3hvXrBnAA/iee0CJwDEFW8N/BTP6xnAg/m3+S7gvXnBvgf4aOAiz9/bAD8trvhu4L14Xl8DfDT/et8FvDcv2PcA780V3w28F8/re4D3FldcBI7zvB4C3Mq/zlcDH8UL9j3Ae/Nsbw38FM9rFzgh4KWBv+J5PQN4MP965gX7HuC9eV7m+XsZAR8NfBXP63uA9+Zf77eB1+J5fQ/w3jx/Pw28Fc/rYwT8NPBWPK/3Ab6bf73jwG8DL8WzfQ/w3rxgnw18Fs/rZwT8NvBaPK/XAX6bf5vjwE8DrwV8DfDRvHCvDfwWz+t3BPwV8NI8L/Ff58HA03leCDDPn/ivZZ4XAszzJ/5rmeeFgF3gGM/rdYDf5r/GawO/xfO6JOC3gdfieb0P8N3813hv4Lt4Xr8j4KuBj+J53Qo8hP8aTwcezPP6GgFvDfwUz9/7AN/Nf66PBr6K5+9txBW3Ag/i+fto4Gv4z/FRwFfz/D0DeLC44rOBz+IFuxX4buBngL/m3+elgbcC3ht4MC/YxwBfLZ7tr4GX4l92K/DTwMfwr/NVwFsDD+Zf9jfASwOIZ3tp4LeBY7xoxL+OedFcAl4b+GsA8ZxeGvht4Bj/MvGvY/5ll4DXBv6aKxDP66WBrwZeixdO/OuYF+53gI8G/ppnQ7xg7w18NPBSPH/iX8c8f38DfDXw3TwvxL/MPH/iX8c8f+IFQ/zLzPMn/nXM8ydeMMS/zDx/4l/HPH/iBUP8y8zzJ/51zPMnXjDE/2yI/9kQ/7Mh/mdDwHHgs4D3Bo7zP8OtwE8DHyPgq4GP4n+mrxFwETjO/0y3CrgVeBD/Mz1DwGcDn8X/TJ8jrvhs4KOBY/zP8Azgu4HP/kef6ditM/XhFgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessBishop;
impl IconShape for FaChessBishop {
    fn view_box(&self) -> &str {
        "0 0 320 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M296 464H23.1C10.75 464 0 474.7 0 487.1S10.75 512 23.1 512h272C309.3 512 320 501.3 320 488S309.3 464 296 464zM0 304c0 51.63 30.12 85.25 64 96v32h48v-67.13l-33.5-10.63C63.75 349.5 48 333.9 48 304c0-84.1 93.2-206.5 112.6-206.5c19.63 0 60.01 67.18 70.28 85.8l-66.13 66.13c-3.125 3.125-4.688 7.219-4.688 11.31S161.6 268.9 164.8 272L176 283.2c3.125 3.125 7.219 4.688 11.31 4.688s8.188-1.562 11.31-4.688L253 229C264.4 256.8 272 283.5 272 304c0 29.88-15.75 45.5-30.5 50.25L208 364.9V432H256v-32c33.88-10.75 64-44.38 64-96c0-73.38-67.75-197.2-120.6-241.5C213.4 59.12 224 47 224 32c0-17.62-14.38-32-32-32H128C110.4 0 96 14.38 96 32c0 15 10.62 27.12 24.62 30.5C67.75 106.8 0 230.6 0 304z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovivcRz4KuCtgV3gu4HP4T8f4r/GdwPvxXP6GuCj+c+F+K9hnj/xnwvxX8M8f+I/F+K/hnn+xH8uxH8N8/yJ/1yI/xrm+RP/uRD/cY4DnwW8N3Ccf5tbge8GPof/GIj/OF8NfBT/MT4G+Gr+/RD/cS4Cx/mP8TvAa/Pvh/iPswsc4z/G7wCvzb8f4j/OVwMfxX+MjwG+mn8/xH+srwbeGzjGv80zgK8Gvpr/GIj/Gub5E/+5EP81zPMn/nMh/muY50/850L81zDPn/jPhfivYZ4/8Z8L8V/DPH/iPxfiv4Z5/sR/LsRzOg58FvDawEvzv8tfAz8NfA2wyxWIZ3tv4KuA4/zvtgu8D/DTAOKK9wa+i/9b3gf4bgHHgacDx/m/ZRd4iICvBj6K/5s+R8BfAS/N/01/LcA8fy8D/DX/O7w08Fc8LwSY5+8hwK387/Bg4Ok8LwT8NfBSPK+PAb6a/x0+GvgqntffCPhq4KN4Xn8NvAz/O/wV8NI8r48R8NLAX/H8PQS4lf/ZHgw8nefvIeKKW4EH8bw+Bvhq/mf7aOCreF5/A7y0uOKrgY/ief018DL8z/ZXwEvzvD4G+GpxxUsDf8Xz9xDgVv5nejDwdJ6/hwC3ime7FXgQz+tjgK/mf6aPBr6K5/U3wEsDiGf7auCjeF4/A7w1/zP9FfDSPK+PAb4aQDzbSwN/xfN3Atjlf5YHA0/n+XsIcCuAeE63Ag/ieb0P8N38z/LRwFfxvP4GeGmuQDynrwY+iuf1M8Bb8z/LXwEvzfP6GOCruQLxnF4a+CuevxPALv8zPBh4Os/fQ4BbuQLxvG4FHsTzeh/gu/mf4aOBr+J5/Q3w0jwb4nl9NfBRPK+fAd6a/xn+CnhpntfXAB/NsyGe10sDf8XzdwLY5b/Xg4Gn8/y9DPDXPBvi+bsVeBDP632A7+a/10cDX8XzegbwYJ4T4vn7buC9eF4/A7w1/73+CnhpntfXAB/Nc0I8f28N/BTP3wlgl/8eDwaezvP3MsBf85wQL9gucIzn9T7Ad/Pf46OBr+J5PQN4MM8L8YJ9N/BePK+fAd6a/x4/DbwVz+trgI/meSFesLcGforn7wSwy3+t48BFnr+XAf6a54V44XaBYzyv9wG+m/9a7w18F8/rGcCDef4QL9x3A+/F8/oZ4K35r/XTwFvxvL4G+GieP8QL99bAT/H8nQB2+a9xHLjI8/cywF/z/CH+ZbvAMZ7X+wDfzX+N9wa+i+f1DODBvGCIf9l3A+/F8/oZ4K35r/HTwFvxvL4HeG9eMMT/bYj/2xD/tyH+b0M8f58FfDRwnP8dbgU+B/hunhPieX028Fn87/Q6wG/zbIjn9VfAS/O/088Ab82zIZ7XbwOvxf9OPwO8Nc+GeF7vDXwX/zu9DfDTPBvi+fto4KOBB/G/wzOArwa+mufEPwL156f+RJJGTgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessKing;
impl IconShape for FaChessKing {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M391.9 464H55.95c-13.25 0-23.1 10.75-23.1 23.1S42.7 512 55.95 512h335.1c13.25 0 23.1-10.75 23.1-23.1S405.2 464 391.9 464zM448 216c0-11.82-3.783-23.51-11.08-33.17c-10.3-14.39-27-22.88-44.73-22.88L247.9 160V104h31.1c13.2 0 24.06-10.8 24.06-24S293.1 56 279.9 56h-31.1V23.1C247.9 10.8 237.2 0 223.1 0S199.9 10.8 199.9 23.1V56H167.9c-13.2 0-23.97 10.8-23.97 24S154.7 104 167.9 104h31.1V160H55.95C24.72 160 0 185.3 0 215.9C0 221.6 .8893 227.4 2.704 233L68.45 432h50.5L48.33 218.4C48.09 217.6 47.98 216.9 47.98 216.1C47.98 212.3 50.93 208 55.95 208h335.9c6.076 0 8.115 5.494 8.115 8.113c0 .6341-.078 1.269-.2405 1.887L328.8 432h50.62l65.1-199.2C447.2 227.3 448 221.7 448 216z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAFjUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjfDfG/G+J/N8T/bojndBx4L+Ctgdfm3++3gVuBW4HfBn6H/1iIZ3tp4KeAB/OfZxf4aeCngZ/h3w9xxUsDf8V/rVuBzwa+h387BBwH/gp4MP89bgU+Bvhp/vUQ8NHAV/Hf77eBtwF2edEh4LeB1+J/hl3gbYDf5kWDAPP8vQ3w0/zbPRh4MPDSwGsDrw0c40XzPsB38y9DgHn+TgC7/Mc5Drw18N7Aa/Evex/gu3nhEHAr8CCe1+cAn81/jtcGvht4EC/c+wDfzQuGgO8G3ovntQs8BNjlP89nA5/FC/c6wG/z/CHgs4HP4vn7HOCz+c/12sBPA8d4/naBhwC7PC8E/DTwVjyvS8BrA3/Nf76XBn4bOMbz99PA2/C8EHAROM7zehvgp/mv89LAbwPHeP5eB/htnhMCzPMn/uu9NfBTPH+3Ag/hOSHAPH/iOR0Hvgt4a+C3gbcBdvmP99XAR/H8vQ/w3TwbAszzJ57TTwNvxbP9NvA6/Mc7Dvw18CCe163AQ3g2BJjnTzwn87zEf473Br6L5+9lgL/mCgSY5088p58G3opn+x3gtfnPcyvwIJ7X1wAfzRUIMM+feE7Hge8G3gr4GeC9gV3+83w08FU8r1uBh3AFAszzJ/57PRh4Os/fQ4BbAQSY50/89/tr4KV4Xu8DfDeAAPP8if9+Xw18FM/rc4DPBhBgnj/x3++jga/ief0O8NoAAszzJ/77vTbwWzyv3wFeG0CAef7Ev82Dga8C3oZ/v9cGfovn9TvAawMIMM+f+Nd7b+C7uEL8+7028Fs8r98BXhtAgHn+xL/Og4Gn82zi3++1gd/ief0O8NoAAszzJ/51Phr4Kp5N/Pu9NvBbPK/fAV4bQIB5/h4C3MqL7qeBt+LZxL/fawO/xfP6HeC1AQT8NvBaPK+fBt6GF81x4CLPSfz7vTbwWzyv3wFeG0DAVwMfxfP308DHALfywn008FU8J/Hv99rAb/G8fgd4bQABDwaezgv308DXAL/N83ov4KuB4zwn8e/32sBv8bx+B3htAHHFVwMfxb/sr4GvBm4FXhr4aODBPK+/AV6af7/XBn6L5/U7wGsDiCuOA78NvBT/MT4H+Gz+/V4b+C2e1+8Arw0gnu048NvAS/Hv8zfAS/Mf47WB3+J5/Q7w2gDieX028Fn823wN8NnALv8xXhv4LZ7X7wCvDSCevwcD7w18NHCMf9nfAB8N/Db/sV4b+C2e1+8Arw0gXrCXBn4LOM4Ldgn4auCz+c/x2sBv8bx+B3htAPGC/RXw0rxgPwN8NHAr/3leG/gtnj8BiOfvvYHv4vm7BHw08N3853tt4Ld4/gQgnr+nAw/m+XsZ4K/5r/HawG/x/AlAPK/XBn6L5+9jgK/mv85rA7/F8ycA8bw+G/gsntczgAfzX+u1gd/i+ROAeF6/DbwWz+tjgK/mv9ZrA7/F8ycA8byeDjyY5/U6wG/zX+u1gd/i+ROAeF7m+TsB7PJf67WB3+L5E4B4Xub5E//1Xhv4LZ4/AYjnZZ4/8V/vtYHf4vkTgHhe5vkT//VeG/gtnj8BiOdlnj/xX++1gd/i+ROA+N8N8b8b4n83xP9uCDgOfBbw3sBx/ne4Ffhp4GMEfDXwUfzv9DUCLgLH+d/pVgG3Ag/if6dnCPhs4LP43+lzxBWfDXw0cIz/HZ4BfDfw2f8INHfTENYpwMUAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessKnight;
impl IconShape for FaChessKnight {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M44 320.6l14.5 6.5c-17.01 20.24-26.44 45.91-26.44 72.35C32.06 399.7 32.12 432 32.12 432h48v-32c0-24.75 14-47.5 36.13-58.63l38.13-23.37c13.25-6.625 21.75-20.25 21.75-35.13v-58.75l-15.37 9C155.6 235.8 151.9 240.4 150.5 245.9L143 271c-2.25 7.625-8 13.88-15.38 16.75L117.1 292C114 293.3 110.7 293.9 107.4 293.9c-3.626 0-7.263-.7514-10.66-2.254L63.5 276.9C54.12 272.6 48 263.2 48 252.9V140.5c0-5.125 2.125-10.12 5.75-13.88l7.375-7.375L49.5 96C48.5 94.12 48 92 48 89.88C48 84.38 52.38 80 57.88 80h105c86.75 0 156.1 70.38 156.1 157.1V432h48.06l-.0625-194.9C367.9 124 276 32 162.9 32H57.88C25.88 32 0 57.88 0 89.88c0 8.5 1.75 16.88 5.125 24.62C1.75 122.8 0 131.6 0 140.5v112.4C0 282.2 17.25 308.8 44 320.6zM80.12 164c0 11 8.875 20 20 20c11 0 20-9 20-20s-9-20-20-20C89 144 80.12 153 80.12 164zM360 464H23.1C10.75 464 0 474.7 0 487.1S10.75 512 23.1 512H360C373.3 512 384 501.3 384 488S373.3 464 360 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACgAAABACAYAAABoWTVaAAAFGUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovifDfE/G+J/NsT/bIh/n7cCXht4aeA48NJc8dtc8dfAbwM/w78N4l/vOPBRwEcDx3nR7ALfDXwOsMuLDvGv89bAdwHH+bfZBd4H+GleNIgX3VcBH81/jO8G3od/GeJF813Ae/Mf67uB9+GFQ/zLvhr4KP5zfA7w2bxgiBfurYGf4oX7HuCngZ/mOb018NbAe/HCvQ7w2zx/iBfsOPB04DjP3+8A7w3cygv3YOCngZfi+bsVeAjPH+IF+2zgs3j+vgd4b/51vht4L56/9wG+m+eFeMEuAsd5Xn8DvDT/eseB3wZeiud1K/AQnhfi+Xtr4Kd4/h4C3Mq/zUsDf8Xz9zLAX/OcEM/fVwMfxfP6HuC9+ff5aeCteF6fA3w2zwnx/P028Fo8r/cBvpt/n/cGvovn9TPAW/OcEM/fXwEvzfN6CHAr/z4vDfwVz+t3gNfmOSGeP/P8if8Y5nn9NfAyPCfE82eeP/Efwzyv3wFem+eEeP5+G3gtntfLAH/Nv89rA7/F8/od4LV5Tojn77eB1+J5fQzw1fz7fDTwVTyv7wHem+eEeP6+GvgontdvA6/Dv89fAS/N8/oY4Kt5Tojn76WBv+L5ex3gt/m3eWvgp3j+HgLcynNCvGC3Ag/ied0KvAywy7/Og4G/Ao7zvP4GeGmeF+IFe2/gu3j+/hp4HWCXF82DgZ8CXprn722An+Z5IV64vwZeiudvF/ho4Ht44d4L+GrgOM/f7wCvzfOHeOFeG/gtXri/Bn4a+G2e01sDbw08mBfuZYC/5vlD/MveG/gu/nO8D/DdvGCIF81HA1/Ff6yPAb6aFw7xvI4D3wW8Nf+1fhp4H2CXZ0M8r68GPor/Hl8DfDTPhnheF4Hj/PfYBU7wbIjndSvwIP57PAN4MM+GeF6fDXwW/z0+B/hsng3x/H028NbAS/Ff43eAnwa+mueEeNGZ50+8cOb5E/8yxIvOPH/ihTPPn/iXIV505vkTL5x5/sS/DPGiM8+feOHM8yf+ZYgXnXn+xAtnnj/xL0O86MzzJ1448/yJfxniRWeeP/HCmedP/MsQL5oHA0/neT0DeDAv3C5wjOd1AtjlhUO8aF4b+C2e1+8Ar80L99vAa/G8Xgf4bV44xIvmrYGf4nn9DvDavHC/DbwWz+ttgJ/mhUO8aD4b+Cye19cAH80L993Ae/G8Pgf4bF44xIvmu4H34nl9DPDVvHCfDXwWz+t7gPfmhUO8aJ4OPJjn9TrAb/PCvTbwWzyvW4GH8MIh/mUPBp7O83cC2OWFOw5c5Pl7CHArLxjiX/bewHfxvJ4BPJgXza3Ag3he7wN8Ny8Y4l/2dODBPK+vAT6aF81XAx/F8/pt4HV4wRAv3GsDv8Xz9zLAX/OieW3gt3j+Xgf4bZ4/xAv3W8Br87yeATyYf51d4BjP62eAt+b5Q/zPhvifDfE/G+J/NgQcB34KeG3+Z/lq4GMEfDfwXvzP9D4CLgLH+Z/pZwTcCjyI/5l+RsBnA5/F/0yvI674aOC9gZfif4bfAb4a+Ol/BArdtEGOUs7lAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessPawn;
impl IconShape for FaChessPawn {
    fn view_box(&self) -> &str {
        "0 0 320 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M296 463.1H23.1c-13.25 0-23.1 10.75-23.1 24s10.75 24 23.1 24h272c13.25 0 23.1-10.75 23.1-23.1S309.3 463.1 296 463.1zM55.1 287.1L80 287.1v29.5c0 40.25-3.5 81.25-23.38 114.5h53.5C125.1 394.1 128 354.6 128 317.5v-29.5h64v29.5c0 37.13 2.875 77.5 17.88 114.5h53.5C243.5 398.7 240 357.7 240 317.5V287.1l24-.0001C277.3 287.1 288 277.3 288 263.1c0-13.25-10.75-24-23.1-24H241c23.75-21.88 38.1-53.12 38.1-87.1c0-9.393-1.106-19.05-3.451-28.86C272.3 105.4 244.9 32 159.1 32C93.75 32 40 85.75 40 151.1c0 34.88 15.12 66.12 39 88H55.1C42.75 239.1 32 250.7 32 263.1C32 277.3 42.75 287.1 55.1 287.1zM160 79.1c39.75 0 72 32.25 72 72S199.8 223.1 160 223.1S88 191.7 88 151.1S120.2 79.1 160 79.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessQueen;
impl IconShape for FaChessQueen {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 112c30.88 0 56-25.12 56-56S286.9 0 256 0S199.1 25.12 199.1 56S225.1 112 256 112zM511.1 197.4c0-5.178-2.509-10.2-7.096-13.26L476.4 168.2c-2.5-1.75-5.497-2.62-8.497-2.62c-5.501 .125-10.63 2.87-13.75 7.245c-9.001 12-23.16 19.13-38.16 19.13c-3.125 0-6.089-.2528-9.089-.8778c-23.13-4.25-38.88-26.25-38.88-49.75C367.1 134 361.1 128 354.6 128h-38.75c-6.001 0-11.63 4-12.88 9.875C298.2 160.1 278.7 176 255.1 176c-22.75 0-42.25-15.88-47-38.12C207.7 132 202.2 128 196.1 128h-38.75C149.1 128 143.1 134 143.1 141.4c0 18.49-13.66 50.62-47.95 50.62c-15.13 0-29.3-7.118-38.3-19.24C54.6 168.4 49.66 165.7 44.15 165.6c-3 0-5.931 .8951-8.432 2.645l-28.63 16C2.509 187.2 0 192.3 0 197.4c0 2.438 .5583 4.901 1.72 7.185L109.9 432h53.13L69.85 236.4C78.35 238.8 87.11 240 95.98 240c2.432 0 56.83 1.503 84.76-52.5C198.1 210.5 226.6 224 255.9 224c29.38 0 57.01-13.38 75.26-36.25C336.1 197.6 360.6 240 416 240c8.751 0 17.5-1.125 26-3.5L349 432h53.13l108.1-227.4C511.4 202.3 511.1 199.8 511.1 197.4zM424 464H87.98c-13.26 0-24 10.75-24 23.1S74.72 512 87.98 512h336c13.26 0 24-10.75 24-23.1S437.3 464 424 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEiUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjfDfG/G+J/N8T/bohn+yjgo4EH8z/bXwNfDXwPgLjiu4D35n+X7wbeR8B7A9/F/07vI+CvgJfmf6e/FmD+90KA+d8LAeb5Ey+cef7EC2eeP/HCmeeFAPP8iRfOPH/ihTPPn3jhzPNCgHn+xAtnnj/xwpnnT7xw5nkhwDx/4oUzz5944czzJ14487wQYJ4/8cKZ50+8cOb5Ey+ceV4IMM+feOHM8ydeOPP8iRfOPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY5+8EsMv/DMeBizwvBJjnbxd4HeCv+e/10sBvAcd5XggwL9gu8DbAb/Pf462B7wKO8/wh4LeB1+KFex/gu/mv9d7Ad/HC/Y6A1wZ+i3/ZVwMfw4vmwcBXAS/NFX8NfAxwKy+arwI+mn/Z64gr3hv4auAYL9x3A+/DC/dg4K+A4zynXeBlgFt54b4LeG9euEvARwPfLZ7tpYHfBo7xwokX7qeBt+L5+xngrXnhzAt3CXht4K8BxHN6aeC7gZfiBRMv3EXgOM/frcBDeOHMC/Y3wHsDf80ViOd1HPht4KV4/sQLZ1448cKZ5+9vgNcGdnk2xAtmnj/xwpkXTrxw5vkTzwvxgpnnT7xw5oUTL5x5/sTzQrxg5vkTL5x54cQLZ54/8bwQL5h5/sQLZ1448cKZ5088L8QLZp4/8cKZF068cOb5E88L8YKZ50+8cOaFEy+cef7E80K8YOb5Ey+ceeHEC2eeP/G8EC+Yef7Efy7z/InnhXjBzPMn/nOZ5088L8Tz99rAb/G8/gZ4af5z3Qo8iOf1MsBf85wQz99rA7/F8/od4LX5z/XbwGvxvF4H+G2eE+L5e23gt3hevwO8Nv+5fht4LZ7X6wC/zXNCPH9vDfwUz+t3gNfmP9dvA6/F83ob4Kd5Tojn77OBz+J5fQ7w2fzn+mrgo3henwN8Ns8J8fx9NvBZPK/PAT6b/1yfDXwWz+tzgM/mOSGev58G3orn9THAV/Of67OBz+J5/Qzw1jwnxPP328Br8bxeB/ht/nO9NvBbPK/fAV6b54R4/szz9xDgVv5zvTTwVzx/4jkhntdx4CLPn/ivYZ6/E8Auz4Z4Xq8N/BbP62+Al+a/xq3Ag3herwP8Ns+GeF4fDXwVz+t3gNfmv8ZvA6/F8/oY4Kt5NsT/boj/3RD/uyH+d0PAceCzgPcGjvO/w63ATwMfI+CrgY/if6evEXAROM7/TrcKuBV4EP87PUPAZwOfxf9OnyOu+Gzgo4Fj/O/wDOC7gc/+R5AJtCzrJjaqAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaChessRook;
impl IconShape for FaChessRook {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M360 464H23.1C10.75 464 0 474.7 0 487.1S10.75 512 23.1 512H360C373.3 512 384 501.3 384 488S373.3 464 360 464zM345.1 32h-308C17 32 0 49 0 70v139.4C0 218.8 4 227.5 11 233.6L48 265.8c0 8.885 .0504 17.64 .0504 26.46c0 39.32-1.001 79.96-11.93 139.8h49C94.95 374.3 96.11 333.3 96.11 285.5C96.11 270.7 96 255.1 96 238.2L48 196.5V80h64V128H160V80h64V128h48V80h64v116.5L288 238.2c0 16.77-.1124 32.25-.1124 47.1c0 47.79 1.164 89.15 10.99 146.7h49c-10.92-59.83-11.93-100.6-11.93-139.9C335.9 283.3 336 274.6 336 265.8l37-32.13C380 227.5 384 218.8 384 209.4V70C384 49 367 32 345.1 32zM192 224C174.4 224 160 238.4 160 256v64h64V256C224 238.4 209.6 224 192 224z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleCheck;
impl IconShape for FaCircleCheck {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M243.8 339.8C232.9 350.7 215.1 350.7 204.2 339.8L140.2 275.8C129.3 264.9 129.3 247.1 140.2 236.2C151.1 225.3 168.9 225.3 179.8 236.2L224 280.4L332.2 172.2C343.1 161.3 360.9 161.3 371.8 172.2C382.7 183.1 382.7 200.9 371.8 211.8L243.8 339.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleDot;
impl IconShape for FaCircleDot {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M160 256C160 202.1 202.1 160 256 160C309 160 352 202.1 352 256C352 309 309 352 256 352C202.1 352 160 309 160 256zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleDown;
impl IconShape for FaCircleDown {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M344 240h-56L287.1 152c0-13.25-10.75-24-24-24h-16C234.7 128 223.1 138.8 223.1 152L224 240h-56c-9.531 0-18.16 5.656-22 14.38C142.2 263.1 143.9 273.3 150.4 280.3l88.75 96C243.7 381.2 250.1 384 256.8 384c7.781-.3125 13.25-2.875 17.75-7.844l87.25-96c6.406-7.031 8.031-17.19 4.188-25.88S353.5 240 344 240zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleLeft;
impl IconShape for FaCircleLeft {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M360 224L272 224v-56c0-9.531-5.656-18.16-14.38-22C248.9 142.2 238.7 143.9 231.7 150.4l-96 88.75C130.8 243.7 128 250.1 128 256.8c.3125 7.781 2.875 13.25 7.844 17.75l96 87.25c7.031 6.406 17.19 8.031 25.88 4.188s14.28-12.44 14.28-21.94l-.002-56L360 288C373.3 288 384 277.3 384 264v-16C384 234.8 373.3 224 360 224zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCirclePause;
impl IconShape for FaCirclePause {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M200 160C186.8 160 176 170.8 176 184v144C176 341.3 186.8 352 200 352S224 341.3 224 328v-144C224 170.8 213.3 160 200 160zM312 160C298.8 160 288 170.8 288 184v144c0 13.25 10.75 24 24 24s24-10.75 24-24v-144C336 170.8 325.3 160 312 160zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCirclePlay;
impl IconShape for FaCirclePlay {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M188.3 147.1C195.8 142.8 205.1 142.1 212.5 147.5L356.5 235.5C363.6 239.9 368 247.6 368 256C368 264.4 363.6 272.1 356.5 276.5L212.5 364.5C205.1 369 195.8 369.2 188.3 364.9C180.7 360.7 176 352.7 176 344V167.1C176 159.3 180.7 151.3 188.3 147.1V147.1zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleQuestion;
impl IconShape for FaCircleQuestion {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464zM256 336c-18 0-32 14-32 32s13.1 32 32 32c17.1 0 32-14 32-32S273.1 336 256 336zM289.1 128h-51.1C199 128 168 159 168 198c0 13 11 24 24 24s24-11 24-24C216 186 225.1 176 237.1 176h51.1C301.1 176 312 186 312 198c0 8-4 14.1-11 18.1L244 251C236 256 232 264 232 272V288c0 13 11 24 24 24S280 301 280 288V286l45.1-28c21-13 34-36 34-60C360 159 329 128 289.1 128z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleRight;
impl IconShape for FaCircleRight {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M280.2 150.2C273.1 143.8 262.1 142.2 254.3 146.1S239.1 158.5 239.1 167.1l.002 56L152 224C138.8 224 128 234.8 128 248v16C128 277.3 138.8 288 152 288L240 287.1v56c0 9.531 5.656 18.16 14.38 22c8.75 3.812 18.91 2.094 25.91-4.375l96-88.75C381.2 268.3 384 261.9 384 255.2c-.3125-7.781-2.875-13.25-7.844-17.75L280.2 150.2zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleStop;
impl IconShape for FaCircleStop {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M328 160h-144C170.8 160 160 170.8 160 184v144C160 341.2 170.8 352 184 352h144c13.2 0 24-10.8 24-24v-144C352 170.8 341.2 160 328 160zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleUp;
impl IconShape for FaCircleUp {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M272.9 135.7C268.3 130.8 261.9 128 255.2 128C247.5 128.3 241.1 130.9 237.5 135.8l-87.25 96C143.8 238.9 142.2 249 146.1 257.7C149.9 266.4 158.5 272 167.1 272h56L224 360c0 13.25 10.75 24 24 24h16c13.25 0 23.1-10.75 23.1-24L287.1 272h56c9.531 0 18.16-5.656 22-14.38c3.811-8.75 2.092-18.91-4.377-25.91L272.9 135.7zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleUser;
impl IconShape for FaCircleUser {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 112c-48.6 0-88 39.4-88 88C168 248.6 207.4 288 256 288s88-39.4 88-88C344 151.4 304.6 112 256 112zM256 240c-22.06 0-40-17.95-40-40C216 177.9 233.9 160 256 160s40 17.94 40 40C296 222.1 278.1 240 256 240zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-46.73 0-89.76-15.68-124.5-41.79C148.8 389 182.4 368 220.2 368h71.69c37.75 0 71.31 21.01 88.68 54.21C345.8 448.3 302.7 464 256 464zM416.2 388.5C389.2 346.3 343.2 320 291.8 320H220.2c-51.36 0-97.35 26.25-124.4 68.48C65.96 352.5 48 306.3 48 256c0-114.7 93.31-208 208-208s208 93.31 208 208C464 306.3 446 352.5 416.2 388.5z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircleXmark;
impl IconShape for FaCircleXmark {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M175 175C184.4 165.7 199.6 165.7 208.1 175L255.1 222.1L303 175C312.4 165.7 327.6 165.7 336.1 175C346.3 184.4 346.3 199.6 336.1 208.1L289.9 255.1L336.1 303C346.3 312.4 346.3 327.6 336.1 336.1C327.6 346.3 312.4 346.3 303 336.1L255.1 289.9L208.1 336.1C199.6 346.3 184.4 346.3 175 336.1C165.7 327.6 165.7 312.4 175 303L222.1 255.1L175 208.1C165.7 199.6 165.7 184.4 175 175V175zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCircle;
impl IconShape for FaCircle {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAD5UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviP917AawOvDTyYK24Ffhv4beB7+I+D+I/z1sBXAQ/mhbsV+Bjgp/n3Q/zH+C7gvfnX+WrgY/j3Qfz7fTXwUfzbfA3w0fzbIf593hr4Kf59Xgf4bf5tEP8+TwcezL/PrcBD+LdB/Nu9N/BdvGAfA/w0V7w38Fm8YG8D/DT/eoh/u+8G3ovn72OAr+Y5fTbwWTx/3wO8N/96iOf1YOCjgNcGXpp/mxPALs/pwcDT+bf5a+C3ga8BbuXZEM/pq4CP5t/vBLDLc3ow8HT+/b4a+BiuQDzbXwEvzX+MzwE+m+f02cBn8R/jr4GXARBXfDXwUfzH+mzge7jivYDP5j/W1wAfLeDBwNP53+khAr4a+Cj+d/oaAX8FvDTP63eA9wZu5Yq/Al6a/xp/DbwMVzwY+G7gtXhefy3APH8PAW7l2cx/LfFsDwaezvNCgHn+xHMy/7XEczLPCwHm+RPPyfzXEs/JPC8EmOdPPKdd4Bj/NZ4BPJjnZJ4XAszzJ57TawPfDTyI/1zPAN4b+G2ek3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/ydAHb5n+HBwNN5Xgj4a+CleF6/DbwPcCv/vR4MfBfw2jyvvxHw1cBH8b/T1wh4MPB0/nd6iLjiq4GP4n+XrwE+WjzbXwMvxf8OfwO8NIB4Tl8NfBT/s30N8NFcgXheDwY+Gnht4KX4n+FvgN8Gvhq4lWfjHwGHWaIWqFAHOAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaClipboard;
impl IconShape for FaClipboard {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 64h-49.61C262.1 27.48 230.7 0 192 0S121 27.48 113.6 64H64C28.65 64 0 92.66 0 128v320c0 35.34 28.65 64 64 64h256c35.35 0 64-28.66 64-64V128C384 92.66 355.3 64 320 64zM192 48c13.23 0 24 10.77 24 24S205.2 96 192 96S168 85.23 168 72S178.8 48 192 48zM336 448c0 8.82-7.178 16-16 16H64c-8.822 0-16-7.18-16-16V128c0-8.82 7.178-16 16-16h18.26C80.93 117.1 80 122.4 80 128v16C80 152.8 87.16 160 96 160h192c8.836 0 16-7.164 16-16V128c0-5.559-.9316-10.86-2.264-16H320c8.822 0 16 7.18 16 16V448z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaClock;
impl IconShape for FaClock {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M232 120C232 106.7 242.7 96 256 96C269.3 96 280 106.7 280 120V243.2L365.3 300C376.3 307.4 379.3 322.3 371.1 333.3C364.6 344.3 349.7 347.3 338.7 339.1L242.7 275.1C236 271.5 232 264 232 255.1L232 120zM256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0zM48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEPUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjP92Dgo4DXBl6a/x5/Dfw28DXArTwb4j/XVwEfzf8sXw18DFcg/vP8FfDS/M/018DLAIj/HF8NfBT/s30N8NHiP96Dgafzv8NDxH+8rwY+iv8dvkb8x/sr4KV5Xr8DvDdwK/+1Hgx8N/BaPK+/Fv/xzPP3EOBW/ns8GHg6zwvxH888f+K/l3leiP945vkT/73M80L8xzPPn/jvZZ4X4j+eef7Efy/zvBD/8czzJ/57meeF+I9nnj/x38s8L8R/PPP8if9e5nkh/uOZ50/89zLPC/Efzzx/4r+XeV6I/3jm+RP/vczzQvzHM8+f+O9lnhfiP555/sR/L/O8EM/rwcBHAa8NvDT/MvGczPMn/nuZ54V4Tl8FfDT/OuI5medP/PcyzwvxbH8FvDT/euI5medP/PcyzwtxxVcDH8W/jXhO5vkT/73M80LAg4Gn828nnpN5/sR/L/O8EPDVwEfxbyeek3n+xH8v87wQ8FfAS/O8fgd4b+BW/nXM8yf+e5nnhQDz/D0EuJV/PfP8if9e5nkhwDx/4t/GPH/iv5d5Xggwz5/4tzHPn/jvZZ4XAszzJ/5tzPMn/nuZ54UA8/yJfxvz/In/XuZ5IcA8f+Lfxjx/4r+XeV4IMM+f+Lcxz5/472WeFwLM8yf+bczzJ/57meeFAPP8iX8b8/yJ/17meSHAPH/i38Y8fyeAXf57PBh4Os8LAeb5E/82fw28FM/rt4H3AW7lv9aDge8CXpvn9TcCzPMn/m2+Gvgo/nf4GgHm+RP/Ng8Gns7/Dg8RYJ4/8W/31cBH8T/b1wAfLcA8f+Lf56+Bl+J/pr8BXhpAgHn+xL/fVwMfxf8sXwN8NFcgwDx/4j/Gg4GPBl4beCn+e/wN8NvAVwO38mwIMM+f+L8PAeb5E//3IcA8f+L/PgSY50/834cA8/yJ/17m+RP/cRBgnj/x38s8f+I/DgLM8yf+e5nnT/zHQYB5/sR/L/P8if84CDDPn/jvZZ4/8R8HAeb5E/+9zPMn/uMgwDx/4r+Xef7EfxwEmOdP/Pcyz5/4j4MA8/ydAHb57/Fg4Ok8f+I/DgL+GngpntdvA+8D3Mp/rQcD3wW8Ns/rb4CX5j8OAr4a+Cj+d/ga4KP5j4OABwNP53+HhwC38h8HccVXAx/F/2xfA3w0/7EQz/bXwEvxP9PfAC/NfzzEc/pq4KP4n+VrgI/mPwfieT0Y+GjgtYGX4r/H3wC/DXw1cCv/efhHuuGmIrqQG8EAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaClone;
impl IconShape for FaClone {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M64 464H288C296.8 464 304 456.8 304 448V384H352V448C352 483.3 323.3 512 288 512H64C28.65 512 0 483.3 0 448V224C0 188.7 28.65 160 64 160H128V208H64C55.16 208 48 215.2 48 224V448C48 456.8 55.16 464 64 464zM160 64C160 28.65 188.7 0 224 0H448C483.3 0 512 28.65 512 64V288C512 323.3 483.3 352 448 352H224C188.7 352 160 323.3 160 288V64zM224 304H448C456.8 304 464 296.8 464 288V64C464 55.16 456.8 48 448 48H224C215.2 48 208 55.16 208 64V288C208 296.8 215.2 304 224 304z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaClosedCaptioning;
impl IconShape for FaClosedCaptioning {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 32H64C28.65 32 0 60.65 0 96v320c0 35.35 28.65 64 64 64h448c35.35 0 64-28.65 64-64V96C576 60.65 547.3 32 512 32zM528 416c0 8.822-7.178 16-16 16H64c-8.822 0-16-7.178-16-16V96c0-8.822 7.178-16 16-16h448c8.822 0 16 7.178 16 16V416zM236.5 222.1c9.375 9.375 24.56 9.375 33.94 0c9.375-9.375 9.375-24.56 0-33.94c-37.44-37.44-98.31-37.44-135.7 0C116.5 206.2 106.5 230.4 106.5 256s9.1 49.75 28.12 67.88c18.72 18.72 43.28 28.08 67.87 28.08s49.16-9.359 67.87-28.08c9.375-9.375 9.375-24.56 0-33.94c-9.375-9.375-24.56-9.375-33.94 0c-18.69 18.72-49.19 18.72-67.87 0C159.5 280.9 154.5 268.8 154.5 256s5-24.88 14.06-33.94C187.3 203.3 217.8 203.3 236.5 222.1zM428.5 222.1c9.375 9.375 24.56 9.375 33.94 0c9.375-9.375 9.375-24.56 0-33.94c-37.44-37.44-98.31-37.44-135.7 0C308.5 206.2 298.5 230.4 298.5 256s9.1 49.75 28.12 67.88c18.72 18.72 43.28 28.08 67.87 28.08s49.16-9.359 67.87-28.08c9.375-9.375 9.375-24.56 0-33.94c-9.375-9.375-24.56-9.375-33.94 0c-18.69 18.72-49.19 18.72-67.87 0C351.5 280.9 346.5 268.8 346.5 256s5-24.88 14.06-33.94C379.3 203.3 409.8 203.3 428.5 222.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCommentDots;
impl IconShape for FaCommentDots {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M144 208C126.3 208 112 222.2 112 239.1C112 257.7 126.3 272 144 272s31.1-14.25 31.1-32S161.8 208 144 208zM256 207.1c-17.75 0-31.1 14.25-31.1 32s14.25 31.1 31.1 31.1s31.1-14.25 31.1-31.1S273.8 207.1 256 207.1zM368 208c-17.75 0-31.1 14.25-31.1 32s14.25 32 31.1 32c17.75 0 31.99-14.25 31.99-32C400 222.2 385.8 208 368 208zM256 31.1c-141.4 0-255.1 93.12-255.1 208c0 47.62 19.91 91.25 52.91 126.3c-14.87 39.5-45.87 72.88-46.37 73.25c-6.624 7-8.373 17.25-4.624 26C5.818 474.2 14.38 480 24 480c61.49 0 109.1-25.75 139.1-46.25c28.87 9 60.16 14.25 92.9 14.25c141.4 0 255.1-93.13 255.1-207.1S397.4 31.1 256 31.1zM256 400c-26.75 0-53.12-4.125-78.36-12.12l-22.75-7.125L135.4 394.5c-14.25 10.12-33.87 21.38-57.49 29c7.374-12.12 14.37-25.75 19.87-40.25l10.62-28l-20.62-21.87C69.81 314.1 48.06 282.2 48.06 240c0-88.25 93.24-160 207.1-160s207.1 71.75 207.1 160S370.8 400 256 400z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaComment;
impl IconShape for FaComment {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 32C114.6 32 .0272 125.1 .0272 240c0 47.63 19.91 91.25 52.91 126.2c-14.88 39.5-45.87 72.88-46.37 73.25c-6.625 7-8.375 17.25-4.625 26C5.818 474.2 14.38 480 24 480c61.5 0 109.1-25.75 139.1-46.25C191.1 442.8 223.3 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32zM256.1 400c-26.75 0-53.12-4.125-78.38-12.12l-22.75-7.125l-19.5 13.75c-14.25 10.12-33.88 21.38-57.5 29c7.375-12.12 14.37-25.75 19.88-40.25l10.62-28l-20.62-21.87C69.82 314.1 48.07 282.2 48.07 240c0-88.25 93.25-160 208-160s208 71.75 208 160S370.8 400 256.1 400z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaComments;
impl IconShape for FaComments {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M208 0C322.9 0 416 78.8 416 176C416 273.2 322.9 352 208 352C189.3 352 171.2 349.7 153.9 345.8C123.3 364.8 79.13 384 24.95 384C14.97 384 5.93 378.1 2.018 368.9C-1.896 359.7-.0074 349.1 6.739 341.9C7.26 341.5 29.38 317.4 45.73 285.9C17.18 255.8 0 217.6 0 176C0 78.8 93.13 0 208 0zM164.6 298.1C179.2 302.3 193.8 304 208 304C296.2 304 368 246.6 368 176C368 105.4 296.2 48 208 48C119.8 48 48 105.4 48 176C48 211.2 65.71 237.2 80.57 252.9L104.1 277.8L88.31 308.1C84.74 314.1 80.73 321.9 76.55 328.5C94.26 323.4 111.7 315.5 128.7 304.1L145.4 294.6L164.6 298.1zM441.6 128.2C552 132.4 640 209.5 640 304C640 345.6 622.8 383.8 594.3 413.9C610.6 445.4 632.7 469.5 633.3 469.9C640 477.1 641.9 487.7 637.1 496.9C634.1 506.1 625 512 615 512C560.9 512 516.7 492.8 486.1 473.8C468.8 477.7 450.7 480 432 480C350 480 279.1 439.8 245.2 381.5C262.5 379.2 279.1 375.3 294.9 369.9C322.9 407.1 373.9 432 432 432C446.2 432 460.8 430.3 475.4 426.1L494.6 422.6L511.3 432.1C528.3 443.5 545.7 451.4 563.5 456.5C559.3 449.9 555.3 442.1 551.7 436.1L535.9 405.8L559.4 380.9C574.3 365.3 592 339.2 592 304C592 237.7 528.7 183.1 447.1 176.6L448 176C448 159.5 445.8 143.5 441.6 128.2H441.6z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCompass;
impl IconShape for FaCompass {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M306.7 325.1L162.4 380.6C142.1 388.1 123.9 369 131.4 349.6L186.9 205.3C190.1 196.8 196.8 190.1 205.3 186.9L349.6 131.4C369 123.9 388.1 142.1 380.6 162.4L325.1 306.7C321.9 315.2 315.2 321.9 306.7 325.1V325.1zM255.1 224C238.3 224 223.1 238.3 223.1 256C223.1 273.7 238.3 288 255.1 288C273.7 288 288 273.7 288 256C288 238.3 273.7 224 255.1 224V224zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEdElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMoviv99rAewEvDbw0/zl+G/gc4Ld54RD/dY4D3wW8Nf913gf4bl4wxH+N48BvAS/Nf733Ab6b5w/xX+O3gNfmv8/7AN/N80L853tt4Lf47/c+wHfznBD/+X4aeCv+Z3gf4Lt5NsR/PvP8fQ/w0cAu/z7mX+d9gO/mCsR/PvP8if8Y5l/vfYDvBhD/+czzJ/5jmH+b9wG+W/znM8+f+I9h/u1eR/znM8+f+I9h/u1+R/znM8+f+I9h/u0Q//nM8yf+Y+wCx/i3QfznM8+f+I/x08Bb8W+D+M9nnj/xH+PBwF8Dx/jXQ/znM8+f+I/zYOCrgZcGHsSLDvG8Hgx8FPDWwIP51xPPyTx/4r+WeV6IZzsOfBbw0fz7iOdknj/xX8s8L8QVx4HfAl6afz/xnMzzJ/5rmeeFuOK7gPfmP4Z4Tub5E/+1zPNCwGsDv8V/HPGczPMn/muZ54WAnwbeiv844jmZ50/81zLPCwHm+fsd4L2BW/n3Mc+f+K9lnhcCzPN3Atjl3888f+K/lnleCDDPn/iPYZ4/8V/LPC8EmOdP/Mcwz5/4r2WeFwLM8yf+Y5jnT/zXMs8LAeb5E/8xzPMn/muZ54UA8/yJ/xjm+RP/tczzQoB5/sR/DPP8if9a5nkhwDx/4j+Gef7Efy3zvBBgnj/xH8M8f+K/lnleCDDPn/iPYZ4/8V/LPC8EmOdP/Mcwz5/4r2WeFwLM8yf+Y5jnT/zXMs8LAeb5E/8xzPMn/muZ54UA8/yJ/xjm+RP/tczzQoB5/sR/DPP8if9a5nkhwDx/4j+Gef7Efy3zvBBgnj/xH8M8f+K/lnleCDDPn/iPYZ4/8V/nwcDTeV4IMM+f+I+xCxzjeX038DHALv+5Hgx8F/DaPK+/EWCeP/Ef47uB9+J/pq8RYJ4/8R/jtYHf4n+mhwgwz5/4j/PVwEfxP8vXAB8twDx/4j/WXwMvxf8MPwO8NYAA8/yJ/3hfDXwU/72+BvhorkCAef7Ef44HAx8NvDbwUvzX+Bvgt4GvBm7l2RBgnj/xfx8CzPMn/u9DgHn+xP99CDDPn/i/DwHm+RP/tczzJ/7zIMA8f+K/lnn+xH8eBJjnT/zXMs+f+M+DAPP8if9a5vkT/3kQYJ4/8V/nwcDTef7Efx4EmOfvu4GPAXb5z/Vg4LuA1+Z5/Q3w0vznQcBPA2/F/0xfA3w0/3kQ8NrAb/E/00OAW/nPg7jiu4H34n+WrwE+mv9ciCuOA78NvBT/M/wM8Nb850M823Hgs4GP4r/X1wAfzX8NxPN6MPDRwGsDL8V/jb8Bfhv4auBW/uvwj/TZrqMiz8KvAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCopy;
impl IconShape for FaCopy {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M502.6 70.63l-61.25-61.25C435.4 3.371 427.2 0 418.7 0H255.1c-35.35 0-64 28.66-64 64l.0195 256C192 355.4 220.7 384 256 384h192c35.2 0 64-28.8 64-64V93.25C512 84.77 508.6 76.63 502.6 70.63zM464 320c0 8.836-7.164 16-16 16H255.1c-8.838 0-16-7.164-16-16L239.1 64.13c0-8.836 7.164-16 16-16h128L384 96c0 17.67 14.33 32 32 32h47.1V320zM272 448c0 8.836-7.164 16-16 16H63.1c-8.838 0-16-7.164-16-16L47.98 192.1c0-8.836 7.164-16 16-16H160V128H63.99c-35.35 0-64 28.65-64 64l.0098 256C.002 483.3 28.66 512 64 512h192c35.2 0 64-28.8 64-64v-32h-47.1L272 448z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCopyright;
impl IconShape for FaCopyright {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464zM255.1 176C255.1 176 255.1 176 255.1 176c21.06 0 40.92 8.312 55.83 23.38c9.375 9.344 24.53 9.5 33.97 .1562c9.406-9.344 9.469-24.53 .1562-33.97c-24-24.22-55.95-37.56-89.95-37.56c0 0 .0313 0 0 0c-33.97 0-65.95 13.34-89.95 37.56c-49.44 49.88-49.44 131 0 180.9c24 24.22 55.98 37.56 89.95 37.56c.0313 0 0 0 0 0c34 0 65.95-13.34 89.95-37.56c9.312-9.438 9.25-24.62-.1562-33.97c-9.438-9.312-24.59-9.219-33.97 .1562c-14.91 15.06-34.77 23.38-55.83 23.38c0 0 .0313 0 0 0c-21.09 0-40.95-8.312-55.89-23.38c-30.94-31.22-30.94-82.03 0-113.3C214.2 184.3 234 176 255.1 176z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAADgElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIjn9WDgo4DXBl6a/x/+Gvht4GuAW3k2xHP6KuCj+f/tq4GP4QrEs/0V8NJcBfDXwMsAiCu+GvgornqgrwE+WsCDgadz1fPzEAFfDXwUVz0/XyPgr4CX5nn9DvDewK383/Zg4LuB1+J5/bUA8/w9BLiV/x8eDDyd54UA8/yJ/1/M80KAef7E/y/meSHAPH/i/xfzvBBgnj/x/4t5XggwV70gCDBXvSAIMFe9IAgwV70gCDBXvSAIMFe9IAgwV70gCDBXvSAIMFe9IAgwV70gCDBXvSAIMFe9IAgwz5/4/8U8LwSY50/8/2KeFwLM8yf+fzHPCwHm+RP/v5jnhQDz/In/X8zzQoB5/sT/L+Z5IcA8f+L/F/O8EGCeP/H/i3leCDDPn/j/xTwvBJjnT/z/Yp4XAszzJ/5/Mc8LAeb5E/+/mOeFAPP8if9fzPNCgHn+xP8v5nkhwDx/4vk7DnwW8NbAg3n+bgV+GvgYntdx4LOAtwYezL/NrcBPAx/DfxzzvBBgnj/x/H018FG8aL4G+Gie01cDH8V/jK8BPpr/GOZ5IcA8f+L5uwgc50VzK/AQntNF4Dj/MW4FHsJ/DPO8EGCeP/H87QLHeNE8A3gwz2kXOMZ/jGcAD+Y/hnleCDDPn3j+vhr4KF40nwN8Ns/pq4GP4j/G5wCfzX8M87wQYJ4/8YJ9NfDWwIN4/p4BfDfw2Tx/Xw28NfAg/m2eAXw38Nn8xzHPCwHm+RP/v5jnhQDz/In/X8zzQoB5/sT/L+Z5IcA8f+L/F/O8EGCevxPALv8/PBh4Os8LAX8NvBTP67eB9wFu5f+2BwPfBbw2z+tvBHw18FFc9fx8jYAHA0/nqufnIeKKrwY+iqse6GuAjxbP9tfAS3EVwN8ALw0gntNXAx/F/29fA3w0VyCe14OBjwZeG3gp/n/4G+C3ga8GbuXZEFe9MIirXhjEVS8M4qoXhn8Ei56JFspk1CgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaCreditCard;
impl IconShape for FaCreditCard {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M168 336C181.3 336 192 346.7 192 360C192 373.3 181.3 384 168 384H120C106.7 384 96 373.3 96 360C96 346.7 106.7 336 120 336H168zM360 336C373.3 336 384 346.7 384 360C384 373.3 373.3 384 360 384H248C234.7 384 224 373.3 224 360C224 346.7 234.7 336 248 336H360zM512 32C547.3 32 576 60.65 576 96V416C576 451.3 547.3 480 512 480H64C28.65 480 0 451.3 0 416V96C0 60.65 28.65 32 64 32H512zM512 80H64C55.16 80 48 87.16 48 96V128H528V96C528 87.16 520.8 80 512 80zM528 224H48V416C48 424.8 55.16 432 64 432H512C520.8 432 528 424.8 528 416V224z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaEnvelopeOpen;
impl IconShape for FaEnvelopeOpen {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M493.6 163c-24.88-19.62-45.5-35.37-164.3-121.6C312.7 29.21 279.7 0 256.4 0H255.6C232.3 0 199.3 29.21 182.6 41.38C63.88 127.6 43.25 143.4 18.38 163C6.75 172 0 186 0 200.8v247.2C0 483.3 28.65 512 64 512h384c35.35 0 64-28.67 64-64.01V200.8C512 186 505.3 172 493.6 163zM464 448c0 8.822-7.178 16-16 16H64c-8.822 0-16-7.178-16-16V276.7l136.1 113.4C204.3 406.8 229.8 416 256 416s51.75-9.211 71.97-26.01L464 276.7V448zM464 214.2l-166.8 138.1c-23.19 19.28-59.34 19.27-82.47 .0156L48 214.2l.1055-13.48c23.24-18.33 42.25-32.97 162.9-120.6c3.082-2.254 6.674-5.027 10.63-8.094C229.4 65.99 246.7 52.59 256 48.62c9.312 3.973 26.62 17.37 34.41 23.41c3.959 3.066 7.553 5.84 10.76 8.186C421.6 167.7 440.7 182.4 464 200.8V214.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAExklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4Xg8GPgp4beCl+b/hr4HfBr4GuJVnQzynrwI+mv/bvhr4GK5APNtfAS/N/w9/DbwMgLjiq4GP4v+XrwE+WsCDgafz/9NDBHw18FH8//Q1Av4KeGme1/cAHw3s8r/bceCrgffief21APP8nQB2+b/hOHCR54UA8/yJ/1vM80KAef7E/y3meSHAPH8ngF3+bzgOXOR5IcA8f38NvA6wy/9ux4HfAl6a54UA84LtAq8D/DX/O7008FvAcZ4/BJh/2fsA383/Lu8NfBcvHALMi+argY/hf4evAj6afxkCzIvup4H3AXb5n+k48F3AW/OiQYD51/lr4H2Av+Z/lpcGvgt4aV50CDD/ervA2wC/zf8Mrw38FHCcfx0EmOfvGcCDeOE+Gvga/nt9FPDVvHDPAB7E80KAef5OAD8NvBYv3HcD78N/j+8C3psX7neAtwYu8rwQYJ4/ccV3A+/FC/fXwOsAu/zXOA78FvDSvHDfA7w3V5jnhQDz/Ilne2/gu3jhbgXeBvhr/nO9NPBbwHFeuPcBvptnM88LAeb5E8/ptYGfBo7xgu0CHwN8N/853hv4KuA4L9gl4LWBv+Y5meeFAPP8ief10sB3Ay/FC/fZwOfwH+uzgM/mhfsb4K2BW3le5nkhwDx/4vk7Dnw38Fa8cD8NvA+wy7/PceC7gLfmhfse4KOBXZ4/87wQYJ4/8cJ9NfBRvHB/DbwNcCv/Ng8Gfgp4aV64zwE+mxfOPC8EmOdP/MveG/guXrhd4G2A3+Zf57WBnwKO84JdAj4a+G7+ZeZ5IcA8f+JF89LAbwPHeOHeB/huXjTvDXwXL9wzgLcG/poXjXleCDDPn3jRPRj4aeCleOG+G/gYYJfn7zjwVcB788L9DfDawC4vOvO8EGCeP/Gvcxz4auC9eOFuBT4b+B3gVq54MPBawGcDD+aF+x7gvfnXM88LAeb5E/82nw18Fv853gf4bv5tzPNCgHn+xL/dewNfDRzjP8Yl4K2B3+bfzjwvBJjnT/z7vDTw28Ax/n3+Bnhv4K/59zHPCwHm+RP/fseB7wbein+brwE+G9jl3888LwSY50/8x3lr4LOBl+JF8zvAZwO/zX8c87wQYJ4/8R/vtYG3Bl4aeC2e0+8Avw38NPDX/MczzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnr8TwC7/NzwYeDrPCwF/DbwUz+u3gfcBbuV/twcD3wW8Ns/rbwR8NfBR/P/0NQIeDDyd/58eIq74auCj+P/la4CPFs/218BL8f/D3wAvDSCe01cDH8X/bV8DfDRXIJ7Xg4GPBl4beCn+b/gb4LeBrwZu5dkQ/78h/n9D/P+G+P8N8f8b4v83xP9viP/f+Edv5cYehpNAHAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaEnvelope;
impl IconShape for FaEnvelope {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 128C0 92.65 28.65 64 64 64H448C483.3 64 512 92.65 512 128V384C512 419.3 483.3 448 448 448H64C28.65 448 0 419.3 0 384V128zM48 128V150.1L220.5 291.7C241.1 308.7 270.9 308.7 291.5 291.7L464 150.1V127.1C464 119.2 456.8 111.1 448 111.1H64C55.16 111.1 48 119.2 48 127.1L48 128zM48 212.2V384C48 392.8 55.16 400 64 400H448C456.8 400 464 392.8 464 384V212.2L322 328.8C283.6 360.3 228.4 360.3 189.1 328.8L48 212.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaEyeSlash;
impl IconShape for FaEyeSlash {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M150.7 92.77C195 58.27 251.8 32 320 32C400.8 32 465.5 68.84 512.6 112.6C559.4 156 590.7 207.1 605.5 243.7C608.8 251.6 608.8 260.4 605.5 268.3C592.1 300.6 565.2 346.1 525.6 386.7L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L150.7 92.77zM189.8 123.5L235.8 159.5C258.3 139.9 287.8 128 320 128C390.7 128 448 185.3 448 256C448 277.2 442.9 297.1 433.8 314.7L487.6 356.9C521.1 322.8 545.9 283.1 558.6 256C544.1 225.1 518.4 183.5 479.9 147.7C438.8 109.6 385.2 79.1 320 79.1C269.5 79.1 225.1 97.73 189.8 123.5L189.8 123.5zM394.9 284.2C398.2 275.4 400 265.9 400 255.1C400 211.8 364.2 175.1 320 175.1C319.3 175.1 318.7 176 317.1 176C319.3 181.1 320 186.5 320 191.1C320 202.2 317.6 211.8 313.4 220.3L394.9 284.2zM404.3 414.5L446.2 447.5C409.9 467.1 367.8 480 320 480C239.2 480 174.5 443.2 127.4 399.4C80.62 355.1 49.34 304 34.46 268.3C31.18 260.4 31.18 251.6 34.46 243.7C44 220.8 60.29 191.2 83.09 161.5L120.8 191.2C102.1 214.5 89.76 237.6 81.45 255.1C95.02 286 121.6 328.5 160.1 364.3C201.2 402.4 254.8 432 320 432C350.7 432 378.8 425.4 404.3 414.5H404.3zM192 255.1C192 253.1 192.1 250.3 192.3 247.5L248.4 291.7C258.9 312.8 278.5 328.6 302 333.1L358.2 378.2C346.1 381.1 333.3 384 319.1 384C249.3 384 191.1 326.7 191.1 255.1H192z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaEye;
impl IconShape for FaEye {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M160 256C160 185.3 217.3 128 288 128C358.7 128 416 185.3 416 256C416 326.7 358.7 384 288 384C217.3 384 160 326.7 160 256zM288 336C332.2 336 368 300.2 368 256C368 211.8 332.2 176 288 176C287.3 176 286.7 176 285.1 176C287.3 181.1 288 186.5 288 192C288 227.3 259.3 256 224 256C218.5 256 213.1 255.3 208 253.1C208 254.7 208 255.3 208 255.1C208 300.2 243.8 336 288 336L288 336zM95.42 112.6C142.5 68.84 207.2 32 288 32C368.8 32 433.5 68.84 480.6 112.6C527.4 156 558.7 207.1 573.5 243.7C576.8 251.6 576.8 260.4 573.5 268.3C558.7 304 527.4 355.1 480.6 399.4C433.5 443.2 368.8 480 288 480C207.2 480 142.5 443.2 95.42 399.4C48.62 355.1 17.34 304 2.461 268.3C-.8205 260.4-.8205 251.6 2.461 243.7C17.34 207.1 48.62 156 95.42 112.6V112.6zM288 80C222.8 80 169.2 109.6 128.1 147.7C89.6 183.5 63.02 225.1 49.44 256C63.02 286 89.6 328.5 128.1 364.3C169.2 402.4 222.8 432 288 432C353.2 432 406.8 402.4 447.9 364.3C486.4 328.5 512.1 286 526.6 256C512.1 225.1 486.4 183.5 447.9 147.7C406.8 109.6 353.2 80 288 80V80z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAIz0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdxx4LeClgZcGjgMPBh7Mc7oVuBXYBf4a+Gvgd4Bd/vMg/nO8NPBewGsDL82/z18Dvw18D/DX/MdC/Mc5DrwX8NHAg/nPcSvw1cD3ALv8+yH+/Y4DHwV8NHCc/xq7wFcDXwPs8m+H+Pd5L+CrgeP899gFPhr4Hv5tEP82Dwa+C3ht/mf4beB9gFv510H867018F3AcV50zwB+G/hr4K+BW4FbeU4PBh4MvDTw0sBrAw/iRbcLvA/w07zoEP86XwV8NC+avwG+G/hp4Fb+bR4MvDXw3sBL8aL5auBjeNEgXnTfBbw3/7LfAT4b+G3+Y7028NnAa/Ev+27gffiXIV40PwW8NS/c3wDvDfw1/7leG/hq4KV44X4aeBteOMS/7LuA9+YFuwR8NvDV/Nf6aOCreOG+G3gfXjDEC/fVwEfxgv0N8N7AX/Pf46WBnwYexAv2NcBH8/whXrC3Bn6KF+xvgNcGdvnvdRz4beCleMHeBvhpnhfi+Xsw8FfAcZ6/7wE+Gtjlf4bjwE8Dr8Xztwu8DHArzwnx/P0W8No8f38DvDawy/8sx4HfBl6K5++3gdfhOSGe13sD38Xz9zfAawO7/M90HPhr4EE8f+8DfDfPhnhOx4GnA8d5XpeA1wb+mv/ZXhr4K56/XeAhwC5XIJ7TZwOfxfP3McBX87/DRwNfxfP3OcBncwXi2Y4DTweO87z+Bnhp/nf5a+CleF67wEOAXQDxbB8NfBXP3+sAv83/Lq8N/BbP38cAXw0gnu3pwIN5Xr8DvDb/O/028Fo8r1uBhwCIK14a+Cuev9cBfpv/nV4b+C2ev5cB/lpc8dXAR/G8/gZ4aZ6/9wa+iit+Gvhp4Gf4r/FWwFsDb80VHwN8N8/fXwMvxfP6GuCjxRV/Bbw0z+tjgK/m+bsIHOc57QI/Dfw08DP8x3or4K2BtwaO85x2gRM8fx8NfBXP66+BlxFwHLjI8/cQ4Faev1uBB/GC7QI/Dfw08DP827wV8NbAWwPHecGeATyY5+/BwNN5/k4IeGvgp3hezwAezAv22sBPA8f4l+0CPw38NPAzvHBvBbw18NbAcf5ll4C3Bn6bF+xW4EE8r7cR8NnAZ/G8vgb4aP5lbw28NfDWwDH+ZbvA2wC/zXN6beCngOP8yy4BPw38NPDT/Mu+G3gvntfnCPhp4K14Xh8DfDUvuuPARV40twIP4Tk9HXgwL5qXAf6aF91HA1/F8/oZAb8NvBbP63WA3+ZF993Ae/GiuQQc5zntAsd40fwM8Na86F4b+C2e1+8IeDrwYJ7XQ4BbedE9HXgwL9j3AG8N7AIfDfw0z+mtga8GjgM/DbwXL5x40T0YeDrP668FmOdP/OvcCjyI5+8ScJx/nVuBB/H8XQKO869jnhcCzPMn/nW+G3gvnr+fAd6af53vBt6L5+9ngLfmX8c8LwSY50/86xwHbgWO8ZwuAS8N3Mq/znHgVuAYz+kS8NLArfzrmOeFAPP8iX+948BXA68NHAd+G/ho4Fb+bY4DXw28NnAc+G3go4Fb+dczzwsBtwIP4nk9BLiV/xseDDyd5/UMAb8NvBbP63WA3+b/htcGfovn9TsCfhp4K57XxwBfzf8NHw18Fc/rewR8NvBZPK+vAT6a/xu+G3gvntfnCHhr4Kd4XrcCD+H/hqcDD+Z5vY2A48BFnr+HALfyv9uDgafz/J0QV/w18FI8r48Bvpr/3T4a+Cqe198ALy2u+Grgo3hefw28DP9xHgy8FfDWXPHaXPHbXPHTwM8At/If56+Al+Z5fQ3w0eKKlwb+iufvdYDf5t/npYGvAl6bF81vAx8D/DX/Pq8N/BbP38sAfy2e7VbgQTyv3wZeh3+7rwI+mn+bzwY+h3+73wJem+f1DODBAOLZPhr4Kp6/1wF+m3+97wLem3+f7wbeh3+91wZ+i+fvY4CvBhDPdhy4FTjG8/pr4GX41/ls4LP4j/E5wGfzr/NXwEvzvC4BDwZ2AcRz+mzgs3j+Pgb4al50F4Hj/MfYBU7wovto4Kt4/j4H+GyuQDyn48CtwDGev5cB/poXzS5wjOd1Cfhq4LeB3+aK1wZeG/ho4BjP6xnAg3nRvDTwVzx/l4AHA7tcgXhe7w18F8/frcDLALv8y74a+Cie098Arw3s8vwdB34beCme09cAH82/7DjwV8CDef7eB/hung3x/P028Fo8f38NvA6wy7/ss4GPBnaB7wY+mxfNZwPvDRwHvhr4bP5lx4HfAl6a5+93gNfmOSGevwcDfw0c4/n7beBtgF3+ZzgO/BTw2jx/l4CXBm7lOSFesLcGfooX7K+B1wF2+e91HPgt4KV5wd4G+GmeF+KF+2rgo3jB/hp4H+Cv+e/x0sBPAQ/mBfsa4KN5/hD/su8G3osX7qOBr+G/1kcBX80L9z3Ae/OCIV40Pw28FS/cXwMfA/w2/7leG/gq4KV54b4HeG9eOMSL7ruB9+Jf9tvA5wC/zX+s1wY+C3ht/mXfA7w3/zLEv85XAx/Fi+avge8Gfga4lX+bBwNvBbw38NK8aL4G+GheNIh/vbcGvhs4xovuVuC3gb8G/hp4BnArz+nBwIOAlwZeGnht4MG86C4B7w38NC86xL/Ng4HvBl6L/xl+B3hv4Fb+dRD/Pu8NfDVwjP8el4CPBr6bfxvEv99x4KOBjwaO8V/jEvDVwFcDu/zbIf7jHAfeG/ho4EH853gG8NXAdwO7/Psh/nO8NPDewGsDL8W/z98Avw18N/DX/MdC/Oc7Drw28NLASwPHgQcDD+I5PQO4FdgF/hr4a+C3gV3+8/CPyUt4i1SyOBcAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceAngry;
impl IconShape for FaFaceAngry {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M328.4 393.5C318.7 402.6 303.5 402.1 294.5 392.4C287.1 384.5 274.4 376 256 376C237.6 376 224.9 384.5 217.5 392.4C208.5 402.1 193.3 402.6 183.6 393.5C173.9 384.5 173.4 369.3 182.5 359.6C196.7 344.3 221.4 328 256 328C290.6 328 315.3 344.3 329.5 359.6C338.6 369.3 338.1 384.5 328.4 393.5zM144.4 240C144.4 231.2 147.9 223.2 153.7 217.4L122.9 207.2C114.6 204.4 110 195.3 112.8 186.9C115.6 178.6 124.7 174 133.1 176.8L229.1 208.8C237.4 211.6 241.1 220.7 239.2 229.1C236.4 237.4 227.3 241.1 218.9 239.2L208.1 235.6C208.3 237 208.4 238.5 208.4 240C208.4 257.7 194 272 176.4 272C158.7 272 144.4 257.7 144.4 240V240zM368.4 240C368.4 257.7 354 272 336.4 272C318.7 272 304.4 257.7 304.4 240C304.4 238.4 304.5 236.8 304.7 235.3L293.1 239.2C284.7 241.1 275.6 237.4 272.8 229.1C270 220.7 274.6 211.6 282.9 208.8L378.9 176.8C387.3 174 396.4 178.6 399.2 186.9C401.1 195.3 397.4 204.4 389.1 207.2L358.9 217.2C364.7 223 368.4 231.1 368.4 240H368.4zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceDizzy;
impl IconShape for FaFaceDizzy {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M192 352C192 316.7 220.7 288 256 288C291.3 288 320 316.7 320 352C320 387.3 291.3 416 256 416C220.7 416 192 387.3 192 352zM103 135C112.4 125.7 127.6 125.7 136.1 135L160 158.1L183 135C192.4 125.7 207.6 125.7 216.1 135C226.3 144.4 226.3 159.6 216.1 168.1L193.9 192L216.1 215C226.3 224.4 226.3 239.6 216.1 248.1C207.6 258.3 192.4 258.3 183 248.1L160 225.9L136.1 248.1C127.6 258.3 112.4 258.3 103 248.1C93.66 239.6 93.66 224.4 103 215L126.1 192L103 168.1C93.66 159.6 93.66 144.4 103 135V135zM295 135C304.4 125.7 319.6 125.7 328.1 135L352 158.1L375 135C384.4 125.7 399.6 125.7 408.1 135C418.3 144.4 418.3 159.6 408.1 168.1L385.9 192L408.1 215C418.3 224.4 418.3 239.6 408.1 248.1C399.6 258.3 384.4 258.3 375 248.1L352 225.9L328.1 248.1C319.6 258.3 304.4 258.3 295 248.1C285.7 239.6 285.7 224.4 295 215L318.1 192L295 168.1C285.7 159.6 285.7 144.4 295 135V135zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceFlushed;
impl IconShape for FaFaceFlushed {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 336C333.3 336 344 346.7 344 360C344 373.3 333.3 384 320 384H192C178.7 384 168 373.3 168 360C168 346.7 178.7 336 192 336H320zM136.4 224C136.4 210.7 147.1 200 160.4 200C173.6 200 184.4 210.7 184.4 224C184.4 237.3 173.6 248 160.4 248C147.1 248 136.4 237.3 136.4 224zM80 224C80 179.8 115.8 144 160 144C204.2 144 240 179.8 240 224C240 268.2 204.2 304 160 304C115.8 304 80 268.2 80 224zM160 272C186.5 272 208 250.5 208 224C208 197.5 186.5 176 160 176C133.5 176 112 197.5 112 224C112 250.5 133.5 272 160 272zM376.4 224C376.4 237.3 365.6 248 352.4 248C339.1 248 328.4 237.3 328.4 224C328.4 210.7 339.1 200 352.4 200C365.6 200 376.4 210.7 376.4 224zM432 224C432 268.2 396.2 304 352 304C307.8 304 272 268.2 272 224C272 179.8 307.8 144 352 144C396.2 144 432 179.8 432 224zM352 176C325.5 176 304 197.5 304 224C304 250.5 325.5 272 352 272C378.5 272 400 250.5 400 224C400 197.5 378.5 176 352 176zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceFrownOpen;
impl IconShape for FaFaceFrownOpen {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M179.3 369.3C166.1 374.5 153.1 365.1 158.4 352.9C175.1 314.7 214.3 287.8 259.9 287.8C305.6 287.8 344.8 314.7 361.4 352.1C366.7 365.2 352.9 374.5 340.6 369.3C316.2 359 288.8 353.2 259.9 353.2C231 353.2 203.7 358.1 179.3 369.3L179.3 369.3zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceFrown;
impl IconShape for FaFaceFrown {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M143.9 398.6C131.4 394.1 124.9 380.3 129.4 367.9C146.9 319.4 198.9 288 256 288C313.1 288 365.1 319.4 382.6 367.9C387.1 380.3 380.6 394.1 368.1 398.6C355.7 403.1 341.9 396.6 337.4 384.1C328.2 358.5 297.2 336 256 336C214.8 336 183.8 358.5 174.6 384.1C170.1 396.6 156.3 403.1 143.9 398.6V398.6zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrimace;
impl IconShape for FaFaceGrimace {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M344 288C374.9 288 400 313.1 400 344C400 374.9 374.9 400 344 400H168C137.1 400 112 374.9 112 344C112 313.1 137.1 288 168 288H344zM168 320C154.7 320 144 330.7 144 344C144 357.3 154.7 368 168 368H176V320H168zM208 368H240V320H208V368zM304 320H272V368H304V320zM336 368H344C357.3 368 368 357.3 368 344C368 330.7 357.3 320 344 320H336V368zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAJDUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdxx4LeClgZcGjgMPBh7Ms+0Cf80Vu8BfA78N/A7/fseB3+KK1wF2eTbEf46XBt4LeG3gpfn3+Wvgt4GvAW7lX++7gPfmiq8BPppnQ/zHOQ68F/DRwIP5z/HXwFcDPwPs8i97a+CneE5vA/w0VyD+/Y4DHwV8NHCc/xq7wFcDn8MLdhx4OnCc57QLPATYBRD/Pu8FfDVwnP8etwKfDXwPz+u7gffi+fsa4KMBxL/Ng4HvAl6b/xleBvhrnu21gd/ihXsd4LfFv95bA98FHOdF9wzgt4G/Bv4auBW4lWd7MPBgrngw8NrAWwPH+Jf9DfDSPKffAl6bF+63gdcR/zpfBXw0L5q/Ab4b+GngVv5tXhp4beCjgQfx/H0M8NU822sDv8WL5nXEi+67gPfmX/Y7wGcDv81/rLcGPhp4LZ7TywB/zbN9N/BevGi+R7xofgp4a164vwHeG/hr/nO9NvDVwEtxhXhOF4HjvGh2xb/su4D35gW7BHw28NX81/ps4LMA8ZzMi+6SeOG+GvgoXrC/Ad4b+Gv+e7w08Nc8p78GXooXze+IF+ytgZ/iBfsb4LWBXf5n+Wjgq3jRfIx4/h4M/BVwnOfve4CPBnb5n+c48NvAS/HC/Q3w0uL5+y3gtXn+/gZ4bWCX/7leGvhu4KV4/v4GeG1gVzyv9wa+i+fvb4DXBnb53+GjgfcGXoor/gb4auC7uQLxnI4DTweO87wuAa8N/DX/dyCe02cDn8Xz9zHAV/N/C+LZjgNPB47zvP4GeGn+70E820cDX8Xz9zrAb/Mve23go4DX5orfBr4G+G3+bV4b+Cjgtbnit4GvAX6bf9lx4K2A3wFu5YoHA68FPAP4bQDxbE8HHszz+h3gtfmXfRXw0Tx/Xw18DP86XwV8NM/fVwMfw7/st4HX4nn9NfAyAOKKlwb+iufvdYDf5oX7LuC9eeG+G3gfXjTfBbw3L9x3A+/DC/fawG/x/L0M8Nfiiq8GPorn9TfAS/PCvTXwU7xoXgf4bV64twZ+ihfN2wA/zQv318BL8by+BvhoccVfAS/N8/oY4Kt54Z4OPJgXza3AQ3jhng48mBfNrcBDeOE+GvgqntdfAy8j4DhwkefvIcCtvGDvDXwX/zqvA/w2z997A9/Fv877AN/NC/Zg4Ok8fycEvDXwUzyvZwAP5oX7aeCteLZnAJ8N/DRXvDXw1cAxnu1rgI/m+ftu4L14tmcAnw38NFe8NfDVwDGe7XuA9+aF+2vgpXhebyPgs4HP4nl9DfDRvHDm2X4GeG9gl+d0HPhu4K244neA1+b5ezrwYK74GeC9gV2e03Hgu4G34oq/Bl6GF+67gffieX2OgJ8G3orn9THAV/PCmSt+B3htXrjfBl4L+B3gtXn+zBW/A7w2L9xvA6/FFeKF+2jgq3hePyPgt4HX4nm9DvDbvHB/zRWvDezywh0Hfhu4FXhrnr+/5orXBnZ54Y4Dvw3sAq/NC/fawG/xvH5HwNOBB/O8HgLcyv8Nrw38Fs/rrwWY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQsCtwIN4Xg8BbuX/hgcDT+d5PUPAbwOvxfN6HeC3+b/htYHf4nn9joCfBt6K5/UxwFfzf8NHA1/F8/oeAZ8NfBbP62uAj+b/hu8G3ovn9TkC3hr4KZ7XrcBDeNF9FHAc2AX+Gvgd/mO9FvDSwHHgVuB7eNE9HXgwz+ttBBwHLvL8PQS4lX/ZbwGvzfP6a2AX+G2u+GtglxfuOPDSXPHawHHgpXlevw28Dv+yBwNP5/k7Ia74a+CleF4fA3w1L9xrA7/Ff4+XAf6aF+6jga/ief0N8NLiiq8GPorn9dfAy/DCPRh4Ov/1LgEPBnZ54f4KeGme19cAHy2ueGngr3j+Xgf4bV64rwY+iv9aHwN8NS/cawO/xfP3MsBfi2e7FXgQz+u3gdfhX/bXwEvxX+N3gNfmX/ZbwGvzvJ4BPBhAPNtHA1/F8/c6wG/zwh0Hfht4Kf5z/Q3w2sAuL9xrA7/F8/cxwFcDiGc7DtwKHON5/TXwMvzLjgO/DbwU/zl+B3hrYJd/2V8BL83zugQ8GNgFEM/ps4HP4vn7GOCredF8NfBR/Mf6GuCjedF8NPBVPH+fA3w2VyCe03HgVuAYz9/LAH/Ni+a1ga8GXop/n98BPhv4bV40Lw38Fc/fJeDBwC5XIJ7XewPfxfN3K/AywC4vutcGPhp4K/51fgb4auC3edEdB/4KeDDP3/sA382zIZ6/3wZei+fvr4HXAXb51zkOvDbw0sBrc8Vxrtjlit8G/hr4bWCXf53jwG8BL83z9zvAa/OcEM/fg4G/Bo7x/P028DbALv8zHAd+Cnhtnr9LwEsDt/KcEC/YWwM/xQv218DrALv89zoO/Bbw0rxgbwP8NM8L8cJ9NfBRvGB/DbwP8Nf893hp4KeAB/OCfQ3w0Tx/iH/ZdwPvxQv30cDX8F/ro4Cv5oX7HuC9ecEQL5qfBt6KF+6vgY8Bfpv/XK8NfBXw0rxw3wO8Ny8c4kX33cB78S/7beBzgN/mP9ZrA58FvDb/su8B3pt/GeJf56uBj+JF89fAdwM/A9zKv82DgbcC3ht4aV40XwN8NC8axL/eWwPfDRzjRXcr8NvAXwN/DTwDuJXn9GDgQcBLAy8NvDbwYF50l4D3Bn6aFx3i3+bBwHcDr8X/DL8DvDdwK/86iH+f9wa+GjjGf49LwEcD382/DeLf7zjw0cBHA8f4r3EJ+Grgq4Fd/u0Q/3GOA+8NfDTwIP5zPAP4auC7gV3+/RD/OV4aeG/gtYGX4t/nb4DfBr4b+Gv+YyH+8x0HXht4aeClgePAg4EH8ZyeAdwK7AJ/Dfw18NvALv95+Ec+kIh64y6F9AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinBeamSweat;
impl IconShape for FaFaceGrinBeamSweat {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M464 128C437.5 128 416 107 416 81.01C416 76.01 417.8 69.74 420.6 62.87C420.9 62.17 421.2 61.46 421.6 60.74C430.5 40.51 448.1 15.86 457.6 3.281C460.8-1.094 467.2-1.094 470.4 3.281C483.4 20.65 512 61.02 512 81.01C512 102.7 497.1 120.8 476.8 126.3C472.7 127.4 468.4 128 464 128L464 128zM391.1 50.53C387.8 58.57 384 69.57 384 81.01C384 84.1 384.3 88.91 384.9 92.72C349.4 64.71 304.7 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 219.7 454.7 185.5 438.3 155.8C446.4 158.5 455.1 160 464 160C473.6 160 482.8 158.3 491.4 155.2C504.7 186.2 512 220.2 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 .0002 256 .0002C307.4 .0002 355.3 15.15 395.4 41.23C393.9 44.32 392.4 47.43 391.1 50.53V50.53zM255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.9 255.9 318.9C289 318.9 320.6 315.1 349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1zM217.6 228.8L217.4 228.5C217.2 228.3 217 228 216.7 227.6C216 226.8 215.1 225.7 213.9 224.3C211.4 221.4 207.9 217.7 203.7 213.1C194.9 206.2 184.8 200 176 200C167.2 200 157.1 206.2 148.3 213.1C144.1 217.7 140.6 221.4 138.1 224.3C136.9 225.7 135.1 226.8 135.3 227.6C134.1 228 134.8 228.3 134.6 228.5L134.4 228.8L134.4 228.8C132.3 231.6 128.7 232.7 125.5 231.6C122.2 230.5 119.1 227.4 119.1 224C119.1 206.1 126.7 188.4 136.6 175.2C146.4 162.2 160.5 152 175.1 152C191.5 152 205.6 162.2 215.4 175.2C225.3 188.4 231.1 206.1 231.1 224C231.1 227.4 229.8 230.5 226.5 231.6C223.3 232.7 219.7 231.6 217.6 228.8L217.6 228.8zM377.6 228.8L377.6 228.8L377.4 228.5C377.2 228.3 377 228 376.7 227.6C376 226.8 375.1 225.7 373.9 224.3C371.4 221.4 367.9 217.7 363.7 213.1C354.9 206.2 344.8 200 336 200C327.2 200 317.1 206.2 308.3 213.1C304.1 217.7 300.6 221.4 298.1 224.3C296.9 225.7 295.1 226.8 295.3 227.6C294.1 228 294.8 228.3 294.6 228.5L294.4 228.8L294.4 228.8C292.3 231.6 288.7 232.7 285.5 231.6C282.2 230.5 280 227.4 280 224C280 206.1 286.7 188.4 296.6 175.2C306.4 162.2 320.5 152 336 152C351.5 152 365.6 162.2 375.4 175.2C385.3 188.4 392 206.1 392 224C392 227.4 389.8 230.5 386.5 231.6C383.3 232.7 379.7 231.6 377.6 228.8V228.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinBeam;
impl IconShape for FaFaceGrinBeam {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM217.6 228.8L217.6 228.8L217.4 228.5C217.2 228.3 217 228 216.7 227.6C216 226.8 215.1 225.7 213.9 224.3C211.4 221.4 207.9 217.7 203.7 213.1C194.9 206.2 184.8 200 176 200C167.2 200 157.1 206.2 148.3 213.1C144.1 217.7 140.6 221.4 138.1 224.3C136.9 225.7 135.1 226.8 135.3 227.6C134.1 228 134.8 228.3 134.6 228.5L134.4 228.8L134.4 228.8C132.3 231.6 128.7 232.7 125.5 231.6C122.2 230.5 120 227.4 120 224C120 206.1 126.7 188.4 136.6 175.2C146.4 162.2 160.5 152 176 152C191.5 152 205.6 162.2 215.4 175.2C225.3 188.4 232 206.1 232 224C232 227.4 229.8 230.5 226.5 231.6C223.3 232.7 219.7 231.6 217.6 228.8V228.8zM377.6 228.8L377.4 228.5C377.2 228.3 377 228 376.7 227.6C376 226.8 375.1 225.7 373.9 224.3C371.4 221.4 367.9 217.7 363.7 213.1C354.9 206.2 344.8 200 336 200C327.2 200 317.1 206.2 308.3 213.1C304.1 217.7 300.6 221.4 298.1 224.3C296.9 225.7 295.1 226.8 295.3 227.6C294.1 228 294.8 228.3 294.6 228.5L294.4 228.8L294.4 228.8C292.3 231.6 288.7 232.7 285.5 231.6C282.2 230.5 280 227.4 280 224C280 206.1 286.7 188.4 296.6 175.2C306.4 162.2 320.5 152 336 152C351.5 152 365.6 162.2 375.4 175.2C385.3 188.4 392 206.1 392 224C392 227.4 389.8 230.5 386.5 231.6C383.3 232.7 379.7 231.6 377.6 228.8L377.6 228.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinHearts;
impl IconShape for FaFaceGrinHearts {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM238.9 177.1L221.4 243C219.1 251.6 210.4 256.6 201.8 254.3L136.7 236.9C118.9 232.1 108.4 213.8 113.1 196.1C117.9 178.3 136.2 167.7 153.1 172.5L170.1 176.8L174.4 160.7C179.2 142.9 197.5 132.4 215.3 137.1C233.1 141.9 243.6 160.2 238.9 177.1H238.9zM341.9 176.8L358 172.5C375.8 167.7 394.1 178.3 398.9 196.1C403.6 213.8 393.1 232.1 375.3 236.9L310.2 254.3C301.6 256.6 292.9 251.6 290.6 243L273.1 177.1C268.4 160.2 278.9 141.9 296.7 137.1C314.5 132.4 332.8 142.9 337.6 160.7L341.9 176.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinSquintTears;
impl IconShape for FaFaceGrinSquintTears {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M426.8 14.18C446-5.046 477.5-4.646 497.1 14.92C516.6 34.49 517 65.95 497.8 85.18C483 99.97 432.2 108.8 409.6 111.9C403.1 112.8 399.2 108 400.1 102.4C403.3 79.94 412 28.97 426.8 14.18H426.8zM74.98 74.98C158.2-8.253 284.5-22.19 382.2 33.17C380.6 37.96 379.3 42.81 378.1 47.52C375 59.67 372.6 72.08 370.8 82.52C290.1 28.93 180.1 37.74 108.9 108.9C37.75 180.1 28.94 290 82.49 370.8C72.01 372.6 59.6 374.1 47.46 378.1C42.76 379.3 37.93 380.6 33.15 382.1C-22.19 284.5-8.245 158.2 74.98 74.98V74.98zM478.8 129.9C534.2 227.5 520.2 353.8 437 437C353.8 520.3 227.5 534.2 129.8 478.8C131.3 474 132.7 469.2 133.9 464.5C136.1 452.3 139.4 439.9 141.2 429.5C221.9 483.1 331.9 474.3 403.1 403.1C474.3 331.9 483.1 221.1 429.5 141.2C439.1 139.4 452.4 137 464.5 133.9C469.2 132.7 474.1 131.4 478.8 129.9L478.8 129.9zM359.2 226.9C369.3 210.6 393 210 397 228.8C406.6 273.1 393.4 322.3 357.8 357.9C322.2 393.5 273 406.7 228.6 397.1C209.9 393.1 210.5 369.4 226.8 359.3C252 343.6 276.1 323.9 300.4 300.5C323.8 277.1 343.5 252.1 359.2 226.9L359.2 226.9zM189.5 235.7C201.1 232.1 211.1 242.1 208.5 254.6L178.8 352.1C176.2 360.7 165.4 363.4 159 357C157.1 355 155.8 352.5 155.6 349.7L150.5 293.6L94.43 288.5C91.66 288.3 89.07 287.1 87.1 285.1C80.76 278.7 83.46 267.9 92.05 265.3L189.5 235.7zM288.5 94.43L293.6 150.5L349.7 155.6C352.5 155.8 355 157.1 357 159C363.4 165.4 360.7 176.2 352.1 178.8L254.6 208.5C242.1 211.1 232.1 201.1 235.7 189.5L265.3 92.05C267.9 83.46 278.7 80.76 285.1 87.1C287.1 89.07 288.3 91.66 288.5 94.43V94.43zM14.18 426.8C28.97 412 79.85 403.2 102.4 400.1C108 399.2 112.8 403.1 111.9 409.6C108.7 432.1 99.97 483 85.18 497.8C65.95 517 34.5 516.6 14.93 497.1C-4.645 477.5-5.046 446 14.18 426.8H14.18z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinSquint;
impl IconShape for FaFaceGrinSquint {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM223.4 194.6C234.1 200.3 234.1 215.7 223.4 221.4L133.5 269.3C125.6 273.6 116 267.8 116 258.9C116 256.1 116.1 253.4 118.8 251.2L154.8 208L118.8 164.8C116.1 162.6 116 159.9 116 157.1C116 148.2 125.6 142.4 133.5 146.7L223.4 194.6zM393.2 164.8L357.2 208L393.2 251.2C395 253.4 396 256.1 396 258.9C396 267.8 386.4 273.6 378.5 269.3L288.6 221.4C277.9 215.7 277.9 200.3 288.6 194.6L378.5 146.7C386.4 142.4 396 148.2 396 157.1C396 159.9 395 162.6 393.2 164.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinStars;
impl IconShape for FaFaceGrinStars {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M199.8 167.3L237.9 172.3C240.1 172.7 243.5 174.8 244.5 177.8C245.4 180.7 244.6 183.9 242.4 186L214.5 212.5L221.5 250.3C222 253.4 220.8 256.4 218.3 258.2C215.8 260.1 212.5 260.3 209.8 258.8L175.1 240.5L142.2 258.8C139.5 260.3 136.2 260.1 133.7 258.2C131.2 256.4 129.1 253.4 130.5 250.3L137.5 212.5L109.6 186C107.4 183.9 106.6 180.7 107.5 177.8C108.5 174.8 111 172.7 114.1 172.3L152.2 167.3L168.8 132.6C170.1 129.8 172.9 128 175.1 128C179.1 128 181.9 129.8 183.2 132.6L199.8 167.3zM359.8 167.3L397.9 172.3C400.1 172.7 403.5 174.8 404.5 177.8C405.4 180.7 404.6 183.9 402.4 186L374.5 212.5L381.5 250.3C382 253.4 380.8 256.4 378.3 258.2C375.8 260.1 372.5 260.3 369.8 258.8L336 240.5L302.2 258.8C299.5 260.3 296.2 260.1 293.7 258.2C291.2 256.4 289.1 253.4 290.5 250.3L297.5 212.5L269.6 186C267.4 183.9 266.6 180.7 267.5 177.8C268.5 174.8 271 172.7 274.1 172.3L312.2 167.3L328.8 132.6C330.1 129.8 332.9 128 336 128C339.1 128 341.9 129.8 343.2 132.6L359.8 167.3zM349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinTears;
impl IconShape for FaFaceGrinTears {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M519.4 334.4C522.7 342.5 527.8 352.1 535.9 361.1C539.9 365 544.1 368.4 548.6 371.4C506.4 454.8 419.9 512 319.1 512C220.1 512 133.6 454.8 91.4 371.4C95.87 368.4 100.1 365 104.1 361.1C112.2 352.1 117.3 342.5 120.6 334.4C121.8 331.5 122.9 328.6 123.9 325.5C152.5 406.2 229.5 464 319.1 464C410.5 464 487.5 406.2 516.1 325.5C517.1 328.6 518.2 331.5 519.4 334.4V334.4zM319.1 47.1C218.6 47.1 134.2 120.5 115.7 216.5C109.1 213.4 101.4 212.2 93.4 213.3C86.59 214.3 77.18 215.7 66.84 217.7C85.31 94.5 191.6 0 319.1 0C448.4 0 554.7 94.5 573.2 217.7C562.8 215.7 553.4 214.3 546.6 213.3C538.6 212.2 530.9 213.4 524.2 216.5C505.8 120.5 421.4 48 319.1 48V47.1zM78.5 341.1C59.98 356.7 32.01 355.5 14.27 337.7C-4.442 319-4.825 288.9 13.55 270.6C22.19 261.9 43.69 255.4 64.05 250.1C77.02 248.2 89.53 246.2 97.94 245C103.3 244.2 107.8 248.7 106.1 254.1C103.9 275.6 95.58 324.3 81.43 338.4C80.49 339.4 79.51 340.3 78.5 341.1V341.1zM561.5 341.1C560.7 340.5 559.1 339.8 559.2 339.1C559 338.9 558.8 338.7 558.6 338.4C544.4 324.3 536.1 275.6 533 254.1C532.2 248.7 536.7 244.2 542.1 245C543.1 245.2 544.2 245.3 545.4 245.5C553.6 246.7 564.6 248.5 575.1 250.1C596.3 255.4 617.8 261.9 626.4 270.6C644.8 288.9 644.4 319 625.7 337.7C607.1 355.5 580 356.7 561.5 341.1L561.5 341.1zM319.9 399.1C269.6 399.1 225.5 374.6 200.9 336.5C190.5 320.4 207.7 303.1 226.3 308.4C255.3 315.1 286.8 318.8 319.9 318.8C353 318.8 384.6 315.1 413.5 308.4C432.2 303.1 449.4 320.4 438.1 336.5C414.4 374.6 370.3 399.1 319.9 399.1zM281.6 228.8L281.4 228.5C281.2 228.3 281 228 280.7 227.6C280 226.8 279.1 225.7 277.9 224.3C275.4 221.4 271.9 217.7 267.7 213.1C258.9 206.2 248.8 200 239.1 200C231.2 200 221.1 206.2 212.3 213.1C208.1 217.7 204.6 221.4 202.1 224.3C200.9 225.7 199.1 226.8 199.3 227.6C198.1 228 198.8 228.3 198.6 228.5L198.4 228.8L198.4 228.8C196.3 231.6 192.7 232.7 189.5 231.6C186.2 230.5 183.1 227.4 183.1 224C183.1 206.1 190.7 188.4 200.6 175.2C210.4 162.2 224.5 152 239.1 152C255.5 152 269.6 162.2 279.4 175.2C289.3 188.4 295.1 206.1 295.1 224C295.1 227.4 293.8 230.5 290.5 231.6C287.3 232.7 283.7 231.6 281.6 228.8L281.6 228.8zM441.6 228.8L441.6 228.8L441.4 228.5C441.2 228.3 441 228 440.7 227.6C440 226.8 439.1 225.7 437.9 224.3C435.4 221.4 431.9 217.7 427.7 213.1C418.9 206.2 408.8 200 400 200C391.2 200 381.1 206.2 372.3 213.1C368.1 217.7 364.6 221.4 362.1 224.3C360.9 225.7 359.1 226.8 359.3 227.6C358.1 228 358.8 228.3 358.6 228.5L358.4 228.8L358.4 228.8C356.3 231.6 352.7 232.7 349.5 231.6C346.2 230.5 344 227.4 344 223.1C344 206.1 350.7 188.4 360.6 175.2C370.4 162.2 384.5 151.1 400 151.1C415.5 151.1 429.6 162.2 439.4 175.2C449.3 188.4 456 206.1 456 223.1C456 227.4 453.8 230.5 450.5 231.6C447.3 232.7 443.7 231.6 441.6 228.8V228.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinTongueSquint;
impl IconShape for FaFaceGrinTongueSquint {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M116 157.1C116 148.2 125.6 142.4 133.5 146.7L223.4 194.6C234.1 200.3 234.1 215.7 223.4 221.4L133.5 269.3C125.6 273.6 116 267.8 116 258.9C116 256.1 116.1 253.4 118.8 251.2L154.8 208L118.8 164.8C116.1 162.6 116 159.9 116 157.1V157.1zM378.5 146.7C386.4 142.4 396 148.2 396 157.1C396 159.9 395 162.6 393.2 164.8L357.2 208L393.2 251.2C395 253.4 396 256.1 396 258.9C396 267.8 386.4 273.6 378.5 269.3L288.6 221.4C277.9 215.7 277.9 200.3 288.6 194.6L378.5 146.7zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 337.7 95.13 408.4 163.7 442.4C161.3 434 160 425.2 160 416V392.7C135.1 375.1 116.9 351.3 105.2 323.5C100.2 311.7 112.2 301 124.5 304.8C164.1 316.9 208.9 323.8 256.3 323.8C303.7 323.8 348.4 316.9 388.1 304.8C400.4 301 412.4 311.7 407.4 323.5C395.6 351.5 376.3 375.5 352 393.1V416C352 425.2 350.7 434 348.3 442.4C416.9 408.4 464 337.7 464 255.1C464 141.1 370.9 47.1 256 47.1L256 48zM320 416V378.6C320 363.9 308.1 352 293.4 352H291.4C280.1 352 270.3 359.9 267.8 370.9C264.1 383.5 247 383.5 244.2 370.9C241.7 359.9 231.9 352 220.6 352H218.6C203.9 352 192 363.9 192 378.6V416C192 451.3 220.7 480 256 480C291.3 480 320 451.3 320 416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAKZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjPdxx4LeClgZcGjgMPBh7Mc7oVuBXYBf4a+Gvgd4Bd/vMg/nO8NPBewGsDL82/z18Dvw18D/DX/MdC/Mc5DrwX8NHAg/nPcSvw1cD3ALv8+yH+/Y4DHwV8NHCc/xq7wFcDXwPs8m+H+Pd5L+CrgeP899gFPhr4Hv5tEP82Dwa+C3ht/mf4beB9gFv510H867018F3AcV50zwB+G/hr4K+BW4FbeU4PBh4MvDTw0sBrAw/iRbcLvA/w07zoEP86XwV8NC+avwG+G/hp4Fb+bR4MvDXw3sBL8aL5auBjeNEgXnTfBbw3/7LfAT4b+G3+Y7028NnAa/Ev+27gffiXIV40PwW8NS/c3wDvDfw1/7leG/hq4KV44X4aeBteOMS/7LuA9+YFuwR8NvDV/Nf6aOCreOG+G3gfXjDEC/fVwEfxgv0N8N7AX/Pf46WBnwYexAv2NcBH8/whXrC3Bn6KF+xvgNcGdvnvdRz4beCleMHeBvhpnhfi+Xsw8FfAcZ6/7wE+Gtjlf4bjwE8Dr8Xztwu8DHArzwnx/P0W8No8f38DvDawy/8sx4HfBl6K5++3gdfhOSGe13sD38Xz9zfAawO7/M90HPhr4EE8f+8DfDfPhnhOx4GnA8d5XpeA1wb+mv8a7wW8NvDSwEtzxV8Dfw38NvA9PH8vDfwVz98u8BBglysQz+mzgc/i+fsY4Kv5z/fWwFcBD+aFuxX4GOCneV4fDXwVz9/nAJ/NFYhnOw48HTjO8/ob4KX5z/dVwEfzr/PVwMfwvP4aeCme1y7wEGAXQDzbRwNfxfP3OsBv85/rq4GP4t/ma4CP5jm9NvBbPH8fA3w1gHi2pwMP5nn9DvDa/Od6a+CneF7fA3w18Ndc8dLARwPvxfN6G+CneU6/DbwWz+tW4CEA4oqXBv6K5+91gN/mP9fTgQfzbJeA1wb+mufvpYHfBo7xbLcCD+E5vTbwWzx/LwP8tbjiq4GP4nn9DfDS/MteGngp4MFc8dfA3wC38i97b+C7eE4vA/w1L9xLA3/Fc3ob4Kd5Tn8NvBTP62uAjxZX/BXw0jyvjwG+mhfsvYDPBh7M83cr8NnA9/CCfTfwXjzb1wAfzbN9FvDewHHgu4HPAXa54ruB9+LZvgd4b57TRwNfxfP6a+BlBBwHLvL8PQS4lef1YOCngJfmRfPXwOsAuzyvvwJemmd7GeCvueKzgc/iOX0N8NFc8dLAX/Fsfw28DM/pwcDTef5OCHhr4Kd4Xs8AHszzOg48HTjOv85fA68D7PKczHMSz/Z04ME8p13gBM9mnpN4XrcCD+J5vY2AzwY+i+f1NcBH87w+G/gs/m3eB/hunpN5TuLZbgUexHO6BBzn2cxzEs/ru4H34nl9joCfBt6K5/UxwFfzvD4a+Cqe188Af80Vbw28FM/rfYDv5jn9NfBSPNvLAH/NFZ8NfBbP6WuAj+aKlwb+imf7G+CleV4fDXwVz+tnBPw28Fo8r9cBfpvndRy4FTjGFX8DvDfw1zyn1wZ+GjjGFX8DvDawy3P6buC9eLavAT6aZ/ts4L2B48B3A58N7HLFdwPvxbN9D/DePK/XBn6L5/U7Ap4OPJjn9RDgVp6/48B7A7vATwO7PH/HgfcGdoGfBnZ5Xu8NfBfP6WWAv+aFe2ngr3hObwP8NM/rwcDTeV5/LcA8f+K/zq3Ag3i2XeB1gL/m+Xtp4LeA4zzbM4AH84KZ54UA8/yJ/zpvDfwUz+u7ga8B/porXhr4KOC9eV5vA/w0L5h5Xggwz5/4r/XVwEfxb/M1wEfzwpnnhQDz/J0Advmv9dXAR/Gv8zXAR/PCHQcu8rwQcCvwIJ7X6wC/zX+9twa+GngQL9wzgI8Gfpp/2WsDv8XzeoaA3wZei+f1OsBv89/nrYG3Bl4aeCmu+Bvgr4GfBn6aF91rA7/F8/odAT8NvBXP62OAr+b/ho8Gvorn9T0CPhv4LJ7X9wDvzf8NPw28Fc/rcwS8NfBTPK9bgYfwonsv4MHALvDXwO/wH+u1gJcGjgO3At/Di+7pwIN5Xm8j4DhwkefvIcCt/Mv+CnhpntdfA7vAb3PFXwO7vHDHgZfmitcGjgMvzfP6beB1+Jc9GHg6z98JccVfAy/F8/oY4Kt54V4b+C3+e7wM8Ne8cJ8NfBbP62+AlxZXfDXwUTyvW4GH8MI9GHg6//UuAQ8Gdnnhng48mOf1NcBHiyteGvgrnr/XAX6bF+6rgY/iv9bHAF/NC/fawG/x/L0M8Nfi2W4FHsTz+m3gdfiX/TXwUvzX+B3gtfmX/Rbw2jyvZwAPBhDP9tHAV/H8vQ7w27xwx4HfBl6K5/U1wHcDfw28NPDZwFvxnH4G+Gzgr4GXBt4b+Cie198Arw3s8sK9NvBbPH8fA3w1gHi248CtwDGe128Dr8O/7Dhwkef0NcBH85yOA38NPIgrngE8mOf11cBH8ZxOALv8y/4KeGme1yXgwcAugHhOnw18Fs/fxwBfzb/MPKeXAf6a5/XewHdxxfsA383zemngr3hO4l/22cBn8fx9DvDZXIF4TseBW4FjPH8vA/w1L5x5TuIF+2hgF/huXjDznMQL99LAX/H8XQIeDOxyBeJ5vTfwXTx/twIvA+zygpnnJP59zHMSL9hx4K+AB/P8vQ/w3Twb4vn7beC1eP7+GngdYJfnzzwn8e9jnpN4/o4DvwW8NM/f7wCvzXNCPH8PBv4aOMbz99fA6wC7PC/znMS/j3lO4nkdB34LeGmev0vASwO38pwQL9hbAz/FC/bXwOsAuzwn85zEv495TuI5HQd+C3hpXrC3AX6a54V44b4a+ChesF3gdYC/5tnMcxL/PuY5iWd7aeC3gOO8YF8DfDTPH+Jf9t3Ae/HCfTTwNVyxCxzj2V4G+Gv+bV4a+Cue7RJwnCs+CvhqXrjvAd6bFwzxovlp4K144W4F3gf4bOC1eLaPAb6af5uPBr6KZ/sZ4KuB7wIezAv3PcB788IhXnTfDbwX/7Jd4DjPdivwEP5tng48mGfbBY7zL/se4L35lyH+db4a+Cj+9T4G+Gr+dT4a+Cr+9b4G+GheNIh/vbcGvhs4xr/OywB/zYvmpYG/4l/nEvDewE/zokP82zwY+G7gtfjX+Wjga3jhPgr4av51fgd4b+BW/nUQ/z7vDXw1cIwX3a3AVwO/A/w1V7w08FrARwMP5kV3Cfho4Lv5t0H8+x0HPhr4aOAY/zUuAV8NfDWwy78d4j/OceC9gY8GHsR/jmcAXw18N7DLvx/iP8dLA+8NvDbwUvz7/A3w28B3A3/NfyzEf77jwGsDLw28NHAceDDwIJ7TM4BbgV3gr4G/Bn4b2OU/D/8I8/XKi7zMgEwAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinTongueWink;
impl IconShape for FaFaceGrinTongueWink {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M159.6 220C148.1 220 139.7 223.8 134.2 229.7C126.7 237.7 114 238.1 105.9 230.6C97.89 223 97.48 210.4 105 202.3C119.6 186.8 140.3 180 159.6 180C178.1 180 199.7 186.8 214.2 202.3C221.8 210.4 221.4 223 213.3 230.6C205.2 238.1 192.6 237.7 185 229.7C179.6 223.8 170.3 220 159.6 220zM312.4 208C312.4 194.7 323.1 184 336.4 184C349.6 184 360.4 194.7 360.4 208C360.4 221.3 349.6 232 336.4 232C323.1 232 312.4 221.3 312.4 208zM256 208C256 163.8 291.8 128 336 128C380.2 128 416 163.8 416 208C416 252.2 380.2 288 336 288C291.8 288 256 252.2 256 208zM336 256C362.5 256 384 234.5 384 208C384 181.5 362.5 160 336 160C309.5 160 288 181.5 288 208C288 234.5 309.5 256 336 256zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM348.3 442.4C416.9 408.4 464 337.7 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 337.7 95.13 408.4 163.7 442.4C161.3 434 160 425.2 160 416V363.6C151.1 355.6 143.3 346.5 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C368.6 346.4 360.8 355.5 352 363.5V416C352 425.2 350.7 434 348.3 442.4H348.3zM320 416V378.6C320 363.9 308.1 352 293.4 352H291.4C280.1 352 270.3 359.9 267.8 370.9C264.1 383.5 247 383.5 244.2 370.9C241.7 359.9 231.9 352 220.6 352H218.6C203.9 352 192 363.9 192 378.6V416C192 451.3 220.7 480 256 480C291.3 480 320 451.3 320 416z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinTongue;
impl IconShape for FaFaceGrinTongue {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208zM368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 337.7 95.13 408.4 163.7 442.4C161.3 434 160 425.2 160 416V363.6C151.1 355.6 143.3 346.5 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C368.6 346.4 360.8 355.5 352 363.5V416C352 425.2 350.7 434 348.3 442.4C416.9 408.4 464 337.7 464 256C464 141.1 370.9 48 255.1 48H256zM320 416V378.6C320 363.9 308.1 352 293.4 352H291.4C280.1 352 270.3 359.9 267.8 370.9C264.1 383.5 247 383.5 244.2 370.9C241.7 359.9 231.9 352 220.6 352H218.6C203.9 352 192 363.9 192 378.6V416C192 451.3 220.7 480 256 480C291.3 480 320 451.3 320 416z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinWide;
impl IconShape for FaFaceGrinWide {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM224 192C224 227.3 209.7 256 192 256C174.3 256 160 227.3 160 192C160 156.7 174.3 128 192 128C209.7 128 224 156.7 224 192zM288 192C288 156.7 302.3 128 320 128C337.7 128 352 156.7 352 192C352 227.3 337.7 256 320 256C302.3 256 288 227.3 288 192zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrinWink;
impl IconShape for FaFaceGrinWink {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM281.9 230.6C273.9 223 273.5 210.4 281 202.3C295.6 186.8 316.3 180 335.6 180C354.1 180 375.7 186.8 390.2 202.3C397.8 210.4 397.4 223 389.3 230.6C381.2 238.1 368.6 237.7 361 229.7C355.6 223.8 346.3 220 335.6 220C324.1 220 315.7 223.8 310.2 229.7C302.7 237.7 290 238.1 281.9 230.6zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceGrin;
impl IconShape for FaFaceGrin {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M349.5 308.4C368.2 303.1 385.4 320.4 374.1 336.5C350.4 374.6 306.3 399.1 255.9 399.1C205.6 399.1 161.5 374.6 136.9 336.5C126.5 320.4 143.7 303.1 162.3 308.4C191.3 315.1 222.8 318.8 255.9 318.8C289 318.8 320.6 315.1 349.5 308.4zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceKissBeam;
impl IconShape for FaFaceKissBeam {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M304.7 297.7C308.9 302.8 312 309.1 312 316C312 322.9 308.9 329.2 304.7 334.3C300.4 339.5 294.5 344 287.9 347.7C285.2 349.3 282.3 350.7 279.2 352C282.3 353.3 285.2 354.7 287.9 356.3C294.5 359.1 300.4 364.5 304.7 369.7C308.9 374.8 312 381.1 312 388C312 394.9 308.9 401.2 304.7 406.3C300.4 411.5 294.5 416 287.9 419.7C274.7 427.1 257.4 432 240 432C236.4 432 233.2 429.5 232.3 426C231.3 422.5 232.9 418.8 236.1 417L236.1 417L236.3 416.9C236.5 416.8 236.8 416.6 237.2 416.3C238 415.9 239.2 415.1 240.6 414.2C243.4 412.4 247.2 409.7 250.8 406.6C254.6 403.5 258 400 260.5 396.6C262.1 393 264 390.2 264 388C264 385.8 262.1 382.1 260.5 379.4C258 375.1 254.6 372.5 250.8 369.4C247.2 366.3 243.4 363.6 240.6 361.8C239.2 360.9 238 360.1 237.2 359.7C236.8 359.4 236.5 359.2 236.3 359.1L236.1 358.1L236.1 358.1C233.6 357.6 232 354.9 232 352C232 349.1 233.6 346.4 236.1 345L236.1 345L236.3 344.9C236.5 344.8 236.8 344.6 237.2 344.3C238 343.9 239.2 343.1 240.6 342.2C243.4 340.4 247.2 337.7 250.8 334.6C254.6 331.5 258 328.1 260.5 324.6C262.1 321 264 318.2 264 316C264 313.8 262.1 310.1 260.5 307.4C258 303.1 254.6 300.5 250.8 297.4C247.2 294.3 243.4 291.6 240.6 289.8C239.2 288.9 238 288.1 237.2 287.7C236.8 287.4 236.5 287.2 236.3 287.1L236.1 286.1L236.1 286.1C232.9 285.2 231.3 281.5 232.3 277.1C233.2 274.5 236.4 272 240 272C257.4 272 274.7 276.9 287.9 284.3C294.5 287.1 300.4 292.5 304.7 297.7L304.7 297.7zM217.6 228.8L217.6 228.8L217.4 228.5C217.2 228.3 217 228 216.7 227.6C216 226.8 215.1 225.7 213.9 224.3C211.4 221.4 207.9 217.7 203.7 213.1C194.9 206.2 184.8 200 176 200C167.2 200 157.1 206.2 148.3 213.1C144.1 217.7 140.6 221.4 138.1 224.3C136.9 225.7 135.1 226.8 135.3 227.6C134.1 228 134.8 228.3 134.6 228.5L134.4 228.8L134.4 228.8C132.3 231.6 128.7 232.7 125.5 231.6C122.2 230.5 120 227.4 120 224C120 206.1 126.7 188.4 136.6 175.2C146.4 162.2 160.5 152 176 152C191.5 152 205.6 162.2 215.4 175.2C225.3 188.4 232 206.1 232 224C232 227.4 229.8 230.5 226.5 231.6C223.3 232.7 219.7 231.6 217.6 228.8V228.8zM377.6 228.8L377.4 228.5C377.2 228.3 377 228 376.7 227.6C376 226.8 375.1 225.7 373.9 224.3C371.4 221.4 367.9 217.7 363.7 213.1C354.9 206.2 344.8 200 336 200C327.2 200 317.1 206.2 308.3 213.1C304.1 217.7 300.6 221.4 298.1 224.3C296.9 225.7 295.1 226.8 295.3 227.6C294.1 228 294.8 228.3 294.6 228.5L294.4 228.8L294.4 228.8C292.3 231.6 288.7 232.7 285.5 231.6C282.2 230.5 280 227.4 280 224C280 206.1 286.7 188.4 296.6 175.2C306.4 162.2 320.5 152 336 152C351.5 152 365.6 162.2 375.4 175.2C385.3 188.4 392 206.1 392 224C392 227.4 389.8 230.5 386.5 231.6C383.3 232.7 379.7 231.6 377.6 228.8L377.6 228.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceKissWinkHeart;
impl IconShape for FaFaceKissWinkHeart {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M345.3 472.1C347.3 479.7 350.9 486.4 355.7 491.8C325.1 504.8 291.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 285.3 507.1 313.4 498 339.7C486.9 334.1 474.5 333.1 461.8 334.6C459.7 329.4 457 324.6 453.9 320.1C460.5 299.9 464 278.4 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C285.4 464 313.5 457.9 338.9 446.8L345.3 472.1zM288.7 334.3C284.4 339.5 278.5 344 271.9 347.7C269.2 349.3 266.3 350.7 263.2 352C266.3 353.3 269.2 354.7 271.9 356.3C278.5 359.1 284.4 364.5 288.7 369.7C292.9 374.8 296 381.1 296 388C296 394.9 292.9 401.2 288.7 406.3C284.4 411.5 278.5 416 271.9 419.7C258.7 427.1 241.4 432 224 432C220.4 432 217.2 429.5 216.3 426C215.3 422.5 216.9 418.8 220.1 417L220.1 417L220.3 416.9C220.5 416.8 220.8 416.6 221.2 416.3C222 415.9 223.2 415.1 224.6 414.2C227.4 412.4 231.2 409.7 234.8 406.6C238.6 403.5 242 400 244.5 396.6C246.1 393 248 390.2 248 388C248 385.8 246.1 382.1 244.5 379.4C242 375.1 238.6 372.5 234.8 369.4C231.2 366.3 227.4 363.6 224.6 361.8C223.2 360.9 222 360.1 221.2 359.7C220.8 359.4 220.5 359.2 220.3 359.1L220.1 358.1L220.1 358.1C217.6 357.6 216 354.9 216 352C216 349.1 217.6 346.4 220.1 345L220.1 345L220.3 344.9C220.5 344.8 220.8 344.6 221.2 344.3C222 343.9 223.2 343.1 224.6 342.2C227.4 340.4 231.2 337.7 234.8 334.6C238.6 331.5 242 328.1 244.5 324.6C246.1 321 248 318.2 248 316C248 313.8 246.1 310.1 244.5 307.4C242 303.1 238.6 300.5 234.8 297.4C231.2 294.3 227.4 291.6 224.6 289.8C223.2 288.9 222 288.1 221.2 287.7C220.8 287.4 220.5 287.2 220.3 287.1L220.1 286.1L220.1 286.1C216.9 285.2 215.3 281.5 216.3 277.1C217.2 274.5 220.4 272 224 272C241.4 272 258.7 276.9 271.9 284.3C278.5 287.1 284.4 292.5 288.7 297.7C292.9 302.8 296 309.1 296 316C296 322.9 292.9 329.2 288.7 334.3V334.3zM144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208zM335.6 220C324.1 220 315.7 223.8 310.2 229.7C302.7 237.7 290 238.1 281.9 230.6C273.9 223 273.5 210.4 281 202.3C295.6 186.8 316.3 180 335.6 180C354.1 180 375.7 186.8 390.2 202.3C397.8 210.4 397.4 223 389.3 230.6C381.2 238.1 368.6 237.7 361 229.7C355.6 223.8 346.3 220 335.6 220zM439.4 373.3L459.5 367.6C481.7 361.4 504.6 375.2 510.6 398.4C516.5 421.7 503.3 445.6 481.1 451.8L396.1 475.6C387.5 478 378.6 472.9 376.3 464.2L353.4 374.9C347.5 351.6 360.7 327.7 382.9 321.5C405.2 315.3 428 329.1 433.1 352.3L439.4 373.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceKiss;
impl IconShape for FaFaceKiss {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M304.7 281.7C308.9 286.8 312 293.1 312 300C312 306.9 308.9 313.2 304.7 318.3C300.4 323.5 294.5 328 287.9 331.7C285.2 333.3 282.3 334.7 279.2 336C282.3 337.3 285.2 338.7 287.9 340.3C294.5 343.1 300.4 348.5 304.7 353.7C308.9 358.8 312 365.1 312 372C312 378.9 308.9 385.2 304.7 390.3C300.4 395.5 294.5 400 287.9 403.7C274.7 411.1 257.4 416 240 416C236.4 416 233.2 413.5 232.3 410C231.3 406.5 232.9 402.8 236.1 401L236.1 401L236.3 400.9C236.5 400.8 236.8 400.6 237.2 400.3C238 399.9 239.2 399.1 240.6 398.2C243.4 396.4 247.2 393.7 250.8 390.6C254.6 387.5 258 384 260.5 380.6C262.1 377 264 374.2 264 372C264 369.8 262.1 366.1 260.5 363.4C258 359.1 254.6 356.5 250.8 353.4C247.2 350.3 243.4 347.6 240.6 345.8C239.2 344.9 238 344.1 237.2 343.7L236.5 343.2L236.3 343.1L236.1 342.1L236.1 342.1C233.6 341.6 232 338.9 232 336C232 333.1 233.6 330.4 236.1 329L236.1 329L236.3 328.9C236.5 328.8 236.8 328.6 237.2 328.3C238 327.9 239.2 327.1 240.6 326.2C243.4 324.4 247.2 321.7 250.8 318.6C254.6 315.5 258 312.1 260.5 308.6C262.1 305 264 302.2 264 300C264 297.8 262.1 294.1 260.5 291.4C258 287.1 254.6 284.5 250.8 281.4C247.2 278.3 243.4 275.6 240.6 273.8C239.2 272.9 238 272.1 237.2 271.7C236.8 271.4 236.5 271.2 236.3 271.1L236.1 270.1L236.1 270.1C232.9 269.2 231.3 265.5 232.3 261.1C233.2 258.5 236.4 256 240 256C257.4 256 274.7 260.9 287.9 268.3C294.5 271.1 300.4 276.5 304.7 281.7V281.7zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceLaughBeam;
impl IconShape for FaFaceLaughBeam {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M130.7 313.9C126.5 300.4 137.8 288 151.1 288H364.5C378.7 288 389.9 300.4 385.8 313.9C368.1 368.4 318.2 408 258.2 408C198.2 408 147.5 368.4 130.7 313.9V313.9zM217.6 228.8L217.6 228.8L217.4 228.5C217.2 228.3 217 228 216.7 227.6C216 226.8 215.1 225.7 213.9 224.3C211.4 221.4 207.9 217.7 203.7 213.1C194.9 206.2 184.8 200 176 200C167.2 200 157.1 206.2 148.3 213.1C144.1 217.7 140.6 221.4 138.1 224.3C136.9 225.7 135.1 226.8 135.3 227.6C134.1 228 134.8 228.3 134.6 228.5L134.4 228.8L134.4 228.8C132.3 231.6 128.7 232.7 125.5 231.6C122.2 230.5 120 227.4 120 224C120 206.1 126.7 188.4 136.6 175.2C146.4 162.2 160.5 152 176 152C191.5 152 205.6 162.2 215.4 175.2C225.3 188.4 232 206.1 232 224C232 227.4 229.8 230.5 226.5 231.6C223.3 232.7 219.7 231.6 217.6 228.8V228.8zM377.6 228.8L377.4 228.5C377.2 228.3 377 228 376.7 227.6C376 226.8 375.1 225.7 373.9 224.3C371.4 221.4 367.9 217.7 363.7 213.1C354.9 206.2 344.8 200 336 200C327.2 200 317.1 206.2 308.3 213.1C304.1 217.7 300.6 221.4 298.1 224.3C296.9 225.7 295.1 226.8 295.3 227.6C294.1 228 294.8 228.3 294.6 228.5L294.4 228.8L294.4 228.8C292.3 231.6 288.7 232.7 285.5 231.6C282.2 230.5 280 227.4 280 224C280 206.1 286.7 188.4 296.6 175.2C306.4 162.2 320.5 152 336 152C351.5 152 365.6 162.2 375.4 175.2C385.3 188.4 392 206.1 392 224C392 227.4 389.8 230.5 386.5 231.6C383.3 232.7 379.7 231.6 377.6 228.8L377.6 228.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceLaughSquint;
impl IconShape for FaFaceLaughSquint {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M130.7 313.9C126.5 300.4 137.8 288 151.1 288H364.5C378.7 288 389.9 300.4 385.8 313.9C368.1 368.4 318.2 408 258.2 408C198.2 408 147.5 368.4 130.7 313.9V313.9zM223.4 178.6C234.1 184.3 234.1 199.7 223.4 205.4L133.5 253.3C125.6 257.6 116 251.8 116 242.9C116 240.1 116.1 237.4 118.8 235.2L154.8 192L118.8 148.8C116.1 146.6 116 143.9 116 141.1C116 132.2 125.6 126.4 133.5 130.7L223.4 178.6zM393.2 148.8L357.2 192L393.2 235.2C395 237.4 396 240.1 396 242.9C396 251.8 386.4 257.6 378.5 253.3L288.6 205.4C277.9 199.7 277.9 184.3 288.6 178.6L378.5 130.7C386.4 126.4 396 132.2 396 141.1C396 143.9 395 146.6 393.2 148.8V148.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceLaughWink;
impl IconShape for FaFaceLaughWink {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M130.7 313.9C126.5 300.4 137.8 288 151.1 288H364.5C378.7 288 389.9 300.4 385.8 313.9C368.1 368.4 318.2 408 258.2 408C198.2 408 147.5 368.4 130.7 313.9V313.9zM208.4 192C208.4 209.7 194 224 176.4 224C158.7 224 144.4 209.7 144.4 192C144.4 174.3 158.7 160 176.4 160C194 160 208.4 174.3 208.4 192zM281.9 214.6C273.9 207 273.5 194.4 281 186.3C295.6 170.8 316.3 164 335.6 164C354.1 164 375.7 170.8 390.2 186.3C397.8 194.4 397.4 207 389.3 214.6C381.2 222.1 368.6 221.7 361 213.7C355.6 207.8 346.3 204 335.6 204C324.1 204 315.7 207.8 310.2 213.7C302.7 221.7 290 222.1 281.9 214.6zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceLaugh;
impl IconShape for FaFaceLaugh {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M130.7 313.9C126.5 300.4 137.8 288 151.1 288H364.5C378.7 288 389.9 300.4 385.8 313.9C368.1 368.4 318.2 408 258.2 408C198.2 408 147.5 368.4 130.7 313.9V313.9zM208.4 192C208.4 209.7 194 224 176.4 224C158.7 224 144.4 209.7 144.4 192C144.4 174.3 158.7 160 176.4 160C194 160 208.4 174.3 208.4 192zM304.4 192C304.4 174.3 318.7 160 336.4 160C354 160 368.4 174.3 368.4 192C368.4 209.7 354 224 336.4 224C318.7 224 304.4 209.7 304.4 192zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceMehBlank;
impl IconShape for FaFaceMehBlank {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceMeh;
impl IconShape for FaFaceMeh {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208zM368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208zM328 328C341.3 328 352 338.7 352 352C352 365.3 341.3 376 328 376H184C170.7 376 160 365.3 160 352C160 338.7 170.7 328 184 328H328zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceRollingEyes;
impl IconShape for FaFaceRollingEyes {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M168 376C168 362.7 178.7 352 192 352H320C333.3 352 344 362.7 344 376C344 389.3 333.3 400 320 400H192C178.7 400 168 389.3 168 376zM80 224C80 179.8 115.8 144 160 144C204.2 144 240 179.8 240 224C240 268.2 204.2 304 160 304C115.8 304 80 268.2 80 224zM160 272C186.5 272 208 250.5 208 224C208 209.7 201.7 196.8 191.8 188C191.9 189.3 192 190.6 192 192C192 209.7 177.7 224 160 224C142.3 224 128 209.7 128 192C128 190.6 128.1 189.3 128.2 188C118.3 196.8 112 209.7 112 224C112 250.5 133.5 272 160 272V272zM272 224C272 179.8 307.8 144 352 144C396.2 144 432 179.8 432 224C432 268.2 396.2 304 352 304C307.8 304 272 268.2 272 224zM352 272C378.5 272 400 250.5 400 224C400 209.7 393.7 196.8 383.8 188C383.9 189.3 384 190.6 384 192C384 209.7 369.7 224 352 224C334.3 224 320 209.7 320 192C320 190.6 320.1 189.3 320.2 188C310.3 196.8 304 209.7 304 224C304 250.5 325.5 272 352 272zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSadCry;
impl IconShape for FaFaceSadCry {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M159.6 220C148.1 220 139.7 223.8 134.2 229.7C126.7 237.7 114 238.1 105.1 230.6C97.89 223 97.48 210.4 105 202.3C119.6 186.8 140.3 180 159.6 180C178.1 180 199.7 186.8 214.2 202.3C221.8 210.4 221.4 223 213.3 230.6C205.2 238.1 192.6 237.7 185 229.7C179.6 223.8 170.3 220 159.6 220zM297.9 230.6C289.9 223 289.5 210.4 297 202.3C311.6 186.8 332.3 180 351.6 180C370.1 180 391.7 186.8 406.2 202.3C413.8 210.4 413.4 223 405.3 230.6C397.2 238.1 384.6 237.7 377 229.7C371.6 223.8 362.3 220 351.6 220C340.1 220 331.7 223.8 326.2 229.7C318.7 237.7 306 238.1 297.9 230.6zM208 320C208 293.5 229.5 272 256 272C282.5 272 304 293.5 304 320V352C304 378.5 282.5 400 256 400C229.5 400 208 378.5 208 352V320zM0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM400 406.1C439.4 368.2 464 314.1 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 314.1 72.55 368.2 112 406.1V288C112 274.7 122.7 264 136 264C149.3 264 160 274.7 160 288V440.6C188.7 455.5 221.4 464 256 464C290.6 464 323.3 455.5 352 440.6V288C352 274.7 362.7 264 376 264C389.3 264 400 274.7 400 288V406.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSadTear;
impl IconShape for FaFaceSadTear {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M169.6 291.3C172.8 286.9 179.2 286.9 182.4 291.3C195.6 308.6 223.1 349 223.1 369C223.1 395 202.5 416 175.1 416C149.5 416 127.1 395 127.1 369C127.1 349 156.6 308.6 169.6 291.3H169.6zM368 346.8C377.9 355.6 378.7 370.8 369.9 380.7C361 390.6 345.9 391.4 335.1 382.6C314.7 363.5 286.7 352 256 352C242.7 352 232 341.3 232 328C232 314.7 242.7 304 256 304C299 304 338.3 320.2 368 346.8L368 346.8zM335.6 176C353.3 176 367.6 190.3 367.6 208C367.6 225.7 353.3 240 335.6 240C317.1 240 303.6 225.7 303.6 208C303.6 190.3 317.1 176 335.6 176zM175.6 240C157.1 240 143.6 225.7 143.6 208C143.6 190.3 157.1 176 175.6 176C193.3 176 207.6 190.3 207.6 208C207.6 225.7 193.3 240 175.6 240zM256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0zM175.9 448C200.5 458.3 227.6 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48C141.1 48 48 141.1 48 256C48 308.7 67.59 356.8 99.88 393.4C110.4 425.4 140.9 447.9 175.9 448V448z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSmileBeam;
impl IconShape for FaFaceSmileBeam {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 352C293.2 352 319.2 334.5 334.4 318.1C343.3 308.4 358.5 307.7 368.3 316.7C378 325.7 378.6 340.9 369.6 350.6C347.7 374.5 309.7 400 256 400C202.3 400 164.3 374.5 142.4 350.6C133.4 340.9 133.1 325.7 143.7 316.7C153.5 307.7 168.7 308.4 177.6 318.1C192.8 334.5 218.8 352 256 352zM217.6 228.8L217.6 228.8L217.4 228.5C217.2 228.3 217 228 216.7 227.6C216 226.8 215.1 225.7 213.9 224.3C211.4 221.4 207.9 217.7 203.7 213.1C194.9 206.2 184.8 200 176 200C167.2 200 157.1 206.2 148.3 213.1C144.1 217.7 140.6 221.4 138.1 224.3C136.9 225.7 135.1 226.8 135.3 227.6C134.1 228 134.8 228.3 134.6 228.5L134.4 228.8L134.4 228.8C132.3 231.6 128.7 232.7 125.5 231.6C122.2 230.5 120 227.4 120 224C120 206.1 126.7 188.4 136.6 175.2C146.4 162.2 160.5 152 176 152C191.5 152 205.6 162.2 215.4 175.2C225.3 188.4 232 206.1 232 224C232 227.4 229.8 230.5 226.5 231.6C223.3 232.7 219.7 231.6 217.6 228.8V228.8zM377.6 228.8L377.4 228.5C377.2 228.3 377 228 376.7 227.6C376 226.8 375.1 225.7 373.9 224.3C371.4 221.4 367.9 217.7 363.7 213.1C354.9 206.2 344.8 200 336 200C327.2 200 317.1 206.2 308.3 213.1C304.1 217.7 300.6 221.4 298.1 224.3C296.9 225.7 295.1 226.8 295.3 227.6C294.1 228 294.8 228.3 294.6 228.5L294.4 228.8L294.4 228.8C292.3 231.6 288.7 232.7 285.5 231.6C282.2 230.5 280 227.4 280 224C280 206.1 286.7 188.4 296.6 175.2C306.4 162.2 320.5 152 336 152C351.5 152 365.6 162.2 375.4 175.2C385.3 188.4 392 206.1 392 224C392 227.4 389.8 230.5 386.5 231.6C383.3 232.7 379.7 231.6 377.6 228.8L377.6 228.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSmileWink;
impl IconShape for FaFaceSmileWink {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 352C293.2 352 319.2 334.5 334.4 318.1C343.3 308.4 358.5 307.7 368.3 316.7C378 325.7 378.6 340.9 369.6 350.6C347.7 374.5 309.7 400 256 400C202.3 400 164.3 374.5 142.4 350.6C133.4 340.9 133.1 325.7 143.7 316.7C153.5 307.7 168.7 308.4 177.6 318.1C192.8 334.5 218.8 352 256 352zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM281.9 230.6C273.9 223 273.5 210.4 281 202.3C295.6 186.8 316.3 180 335.6 180C354.1 180 375.7 186.8 390.2 202.3C397.8 210.4 397.4 223 389.3 230.6C381.2 238.1 368.6 237.7 361 229.7C355.6 223.8 346.3 220 335.6 220C324.1 220 315.7 223.8 310.2 229.7C302.7 237.7 290 238.1 281.9 230.6zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSmile;
impl IconShape for FaFaceSmile {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 352C293.2 352 319.2 334.5 334.4 318.1C343.3 308.4 358.5 307.7 368.3 316.7C378 325.7 378.6 340.9 369.6 350.6C347.7 374.5 309.7 400 256 400C202.3 400 164.3 374.5 142.4 350.6C133.4 340.9 133.1 325.7 143.7 316.7C153.5 307.7 168.7 308.4 177.6 318.1C192.8 334.5 218.8 352 256 352zM208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208zM304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceSurprise;
impl IconShape for FaFaceSurprise {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M144.4 208C144.4 190.3 158.7 176 176.4 176C194 176 208.4 190.3 208.4 208C208.4 225.7 194 240 176.4 240C158.7 240 144.4 225.7 144.4 208zM368.4 208C368.4 225.7 354 240 336.4 240C318.7 240 304.4 225.7 304.4 208C304.4 190.3 318.7 176 336.4 176C354 176 368.4 190.3 368.4 208zM192 352C192 316.7 220.7 288 256 288C291.3 288 320 316.7 320 352C320 387.3 291.3 416 256 416C220.7 416 192 387.3 192 352zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFaceTired;
impl IconShape for FaFaceTired {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M176.5 320.3C196.1 302.1 223.8 288 256 288C288.2 288 315.9 302.1 335.5 320.3C354.5 338.1 368 362 368 384C368 389.4 365.3 394.4 360.8 397.4C356.2 400.3 350.5 400.8 345.6 398.7L328.4 391.1C305.6 381.2 280.9 376 256 376C231.1 376 206.4 381.2 183.6 391.1L166.4 398.7C161.5 400.8 155.8 400.3 151.2 397.4C146.7 394.4 144 389.4 144 384C144 362 157.5 338.1 176.5 320.3zM223.4 194.6C234.1 200.3 234.1 215.7 223.4 221.4L133.5 269.3C125.6 273.6 116 267.8 116 258.9C116 256.1 116.1 253.4 118.8 251.2L154.8 208L118.8 164.8C116.1 162.6 116 159.9 116 157.1C116 148.2 125.6 142.4 133.5 146.7L223.4 194.6zM393.2 164.8L357.2 208L393.2 251.2C395 253.4 396 256.1 396 258.9C396 267.8 386.4 273.6 378.5 269.3L288.6 221.4C277.9 215.7 277.9 200.3 288.6 194.6L378.5 146.7C386.4 142.4 396 148.2 396 157.1C396 159.9 395 162.6 393.2 164.8zM512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAE1klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4t/uq4CPBj4H+Gye10cDXwXcCrwN8Nf8y8zzQoB5/sS/zXcB782zief0YODpPNsu8BBglxfOPC8EmOdP/Ot9F/DePCfxnB4MPJ3n9NPA2/DCmeeFAPP8iX+d7wLem+clntdXAx/Fc3od4Ld5wczzQoB5/sSL7ruA9+b5E8/fbwOvxbP9NvA6vGDmeSHAPH/iOX028Fn864nn77WB3+I5PQS4lefPPC8EmOdPPCfzbyNesN8GXotn+xzgs3n+zPNCgHn+xHMy/zbiBXtv4Lt4tt8BXpvnzzwvBJjnTzwn828jXrAHA0/n2W4FHsLzZ54XAszzJ56T+bcRL5x5TuL5M88LAeb5E8/J/NuIF848J/H8meeFAPP8iedk/m3EC/Zg4Ok82zOAB/P8meeFAPP8iedk/m3EC/bewHfxbL8DvDbPn3leCDDPn3hO5t9GvGA/DbwVz/Y5wGfz/JnnhQDz/Inn9NnAZ/GvJ56/lwb+iuf0EOBWnj/zvBBgnj/xovtu4L14/sTz91vAa/NsvwO8Ni+YeV4IMM+f+Nf5buC9eF7ieX018FE8p9cBfpsXzDwvBJjnT/zrfTfwXjwn8ZweDDyd5/Q9wHvzwpnnhQDz/Il/m+8G3otnE8/pwcDTebZnAC8N7PLCmeeFAPP8iX+7rwY+Cvgc4LN5Xh8NfDawC7w18Nf8y8zzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjn7wSwy/8MDwaezvNCwF8DL8Xz+m3gfYBb+e/1YOC7gNfmef2NgK8GPor/nb5GwIOBp/O/00PEFV8NfBT/u3wN8NHi2f4aeCn+d/gZ4K0BxHP6auCj+J/ta4CP5grE83ow8NHAawMvxf8MfwP8NvDVwK08G/8ICXbT2GulWmsAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileAudio;
impl IconShape for FaFileAudio {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0L64-.0001c-35.35 0-64 28.65-64 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM171.5 259.5L136 296H92C85.38 296 80 301.4 80 308v56C80 370.7 85.38 376 92 376H136l35.5 36.5C179.1 420 192 414.8 192 404v-136C192 257.3 179.1 251.9 171.5 259.5zM235.1 260.7c-6.25 6.25-6.25 16.38 0 22.62C235.3 283.5 256 305.1 256 336c0 30.94-20.77 52.53-20.91 52.69c-6.25 6.25-6.25 16.38 0 22.62C238.2 414.4 242.3 416 246.4 416s8.188-1.562 11.31-4.688C258.1 410.1 288 380.5 288 336s-29.05-74.06-30.28-75.31C251.5 254.4 241.3 254.4 235.1 260.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAFBUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xIvutYHXAn4H+G1euNcGXgv4HeC3edGZ54UA8/yJF813Ae/Ns70P8N08f+8NfBfP9tXAx/CiMc8LAeb5E/+y7wLem+f018DL8PxdBI7znL4beB/+ZeZ5IcA8f+KF+y7gvXlefwO8NM/fLnCM5/XdwPvwwpnnhQDz/IkX7LuA9+b5+xjgq3n+Phr4Kp6/7wbehxfMPC8EmOdPPH/fBbw3z9/3AO/NC/fdwHvx/H038D48f+Z5IcA8f+J5fRfw3jx/3wO8Ny+a7wbei+fvu4H34XmZ54UA8/yJ5/TZwGfx/H0P8N7863w38F48fx8DfDXPyTwvBJjnTzyni8Bxntf3AO/Nv813A+/F87oVeAjPyTwvBJjnTzynXeAYz+u7gffh3+a7gPfmeT0DeDDPyTwvBJjnTzynjwa+iufvu4H34V/nu4D35vn7GOCreU7meSHAPH/ieX038F48f98NvA8vmu8C3pvn73uA9+Z5meeFAPP8iefvu4H34vn7buB9eOG+C3hvnr/vAd6b5888LwSY50+8YN8NvBfP38cAX83z99HAV/H8fQ/w3rxg5nkhwDx/4oX7buC9eF63Ag/h+bsIHOd5fQ/w3rxw5nkhwDx/4l/23cB78ZyeATyY528XOMZz+h7gvfmXmeeFAPP8iRfNVwMfxbO9D/DdPH/vDXwXz/Y1wEfzojHPCwHm+RMvutcGXhv4beC3eeFeG3ht4LeB3+ZFZ54XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOfvBLDL/wwPBp7O80LAXwMvxfP6beB9gFv57/Vg4LuA1+Z5/Y2ArwY+iv+dvkbAg4Gn87/TQ8QVXw18FP+7fA3w0eLZ/hp4Kf53+BngrQHEc/pq4KP4n+1rgI/mCsTzejDw0cBrAy/F/wx/A/w28NXArTwb/wjb29vYB0ZmVQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileCode;
impl IconShape for FaFileCode {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M162.1 257.8c-7.812-7.812-20.47-7.812-28.28 0l-48 48c-7.812 7.812-7.812 20.5 0 28.31l48 48C137.8 386.1 142.9 388 148 388s10.23-1.938 14.14-5.844c7.812-7.812 7.812-20.5 0-28.31L128.3 320l33.86-33.84C169.1 278.3 169.1 265.7 162.1 257.8zM365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM221.9 257.8c-7.812 7.812-7.812 20.5 0 28.31L255.7 320l-33.86 33.84c-7.812 7.812-7.812 20.5 0 28.31C225.8 386.1 230.9 388 236 388s10.23-1.938 14.14-5.844l48-48c7.812-7.812 7.812-20.5 0-28.31l-48-48C242.3 250 229.7 250 221.9 257.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAFN0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5lnwV8NLAL/DTwMbxovgp4a+A48NXA5/AvM88LAeb5Ey/cVwMfxXP6buB9eOG+C3hvntPXAB/NC2eeFwLM8ydeuIvAcZ7X+wDfzfP33sB38bx2gRO8cOZ5IcA8f+KF2wWO8fy9DPDXPKeXBv6K5+8ScJwXzjwvBJjnT7xwnw18Fs/frcDLALtccRz4K+DBPH+fA3w2L5x5Xggwz5944Y4Dvw28FM/fbwOvwxW/Bbw2z9/fAK8N7PLCmeeFAPP8iX/Zg4G/Bo7x/H0OV3wWz98l4KWBW/mXmeeFAPP8iRfNWwM/xb/N2wA/zYvGPC8EmOdPvOg+G/gs/nU+B/hsXnTmeSHAPH/iX+e3gdfiRfM7wGvzr2OeFwLM8yf+dY4Dfw08iBfuGcBLA7v865jnhQDz/Il/vZcG/ooX7mWAv+ZfzzwvBJjnT/zrvTTwV7xwLwP8Nf965nkhwDx/4l/nOPBXwIN54W4FXgbY5V/HPC8EmOdP/Ov8FvDavGh+G3gd/nXM80KAef7Ei+6zgc/iX+dzgM/mRWeeFwLM8ydeNG8N/BT/Nm8D/DQvGvO8EGCeP/EvezDwV8Bxnr/P4YrP4vnbBV4GuJV/mXleCDDPn3jhjgO/Bbw0z9/PAG/NFb8NvBbP318DrwPs8sKZ54UA8/yJF+6zgc/i+fsb4LWBXa44Dvw18CCev88BPpsXzjwvBJjnT7xwF4HjPK9LwGsDf81zemngr3j+doETvHDmeSHAPH/ihdsFjvG83gb4aZ6/jwa+iud1CTjOC2eeFwLM8ydeuK8GPorn9DXAR/PCfTfwXjynrwE+mhfOPC8EmOdP/Ms+G/hoYBf4auCredF8NfDWwHHgq4HP5l9mnhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH8ngF3+Z3gw8HSeFwL+GngpntdvA+8D3Mp/rwcD3wW8Ns/rbwR8NfBR/O/0NQIeDDyd/50eIq74auCj+N/la4CPFs/218BL8b/DzwBvDSCe01cDH8X/bF8DfDRXIJ7Xg4GPBl4beCn+Z/gb4LeBrwZu5dn4RyMy6NgNWruaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileExcel;
impl IconShape for FaFileExcel {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM229.1 233.3L192 280.9L154.9 233.3C146.8 222.8 131.8 220.9 121.3 229.1C110.8 237.2 108.9 252.3 117.1 262.8L161.6 320l-44.53 57.25c-8.156 10.47-6.25 25.56 4.188 33.69C125.7 414.3 130.8 416 135.1 416c7.156 0 14.25-3.188 18.97-9.25L192 359.1l37.06 47.65C233.8 412.8 240.9 416 248 416c5.125 0 10.31-1.656 14.72-5.062c10.44-8.125 12.34-23.22 4.188-33.69L222.4 320l44.53-57.25c8.156-10.47 6.25-25.56-4.188-33.69C252.2 220.9 237.2 222.8 229.1 233.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAE1klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ16wBwNfBbw0V/w18DHArfznMc8LAeb5E8/fg4G/Ao7znHaBlwFu5T+HeV4IMM+feP5+Gngrnr+fAd6a/xzmeSHAPH/i+bsIHOf5uxV4CP85zPNCgHn+xPO3Cxzj+XsG8GD+c5jnhQDz/Inn76eBt+L5+xngrfnPYZ4XAszzJ56/BwN/DRzjOV0CXhq4lf8c5nkhwDx/4gV7MPDVwEtzxV8DHw3cyn8e87wQYJ4/8R/rvbjie/i3Mc8LAeb5E/9xfgp4a654H+C7+dczzwsB5vkT/zHeGvgpnm0XeBngVv51zPNCgHn+xL/fceDpwHGe028Dr8O/jnleCDDPn/j3+yngrXn+Pgb4al505nkhwDx/4kX3WsDv8JzeGvgpXrBd4GWAW3nRmOeFAPP8iRfNXwEvDfw18DrALnAceDpwnBfut4HX4UVjnhcCzPMn/mWfDXwWz/bTwNsAPwW8NS+ajwG+mn+ZeV4IMM+feOFeGvgrntd3A+/Ni24XeBngVl4487wQYJ4/8cL9FfDS/Mf4beB1eOHM80KAef7EC/bZwGfxH+tjgK/mBTPPCwHm+RPP30sDf8V/vF3gZYBbef7M80KAef7E8/dXwEvzn+O3gdfh+TPPCwHm+RPP67OBz+I/18cAX83zMs8LAeb5E8/ppYG/4j/fLvAywK08J/O8EGCeP/Gcfgt4bf5rfA/w3jwn87wQYJ4/8Zx2gWP81/gd4LV5TuZ5IcA8f+I5vTfw1cAx/nM9A/hs4Lt5TuZ5IcA8f+J/FvO8EGCeP/E/i3leCDDP3wlgl/8ZHgw8neeFgL8GXorn9dvA+wC38t/rwcB3Aa/N8/obAV8NfBT/O32NgAcDT+d/p4eIK74a+Cj+d/ka4KPFs/018FL87/AzwFsDiOf01cBH8T/b1wAfzRWI5/Vg4KOB1wZeiv8Z/gb4beCrgVt5Nv4RmMbX2BMGr2QAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileImage;
impl IconShape for FaFileImage {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM215.3 292c-4.68 0-9.051 2.34-11.65 6.234L164 357.8l-11.68-17.53C149.7 336.3 145.3 334 140.7 334c-4.682 0-9.053 2.34-11.65 6.234l-46.67 70c-2.865 4.297-3.131 9.82-.6953 14.37C84.09 429.2 88.84 432 93.1 432h196c5.163 0 9.907-2.844 12.34-7.395c2.436-4.551 2.17-10.07-.6953-14.37l-74.67-112C224.4 294.3 220 292 215.3 292zM128 288c17.67 0 32-14.33 32-32S145.7 224 128 224S96 238.3 96 256S110.3 288 128 288z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEEUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4gX7KuCtgQfzH+NW4KeBj+EFM88LAeb5E8/fVwMfxX+OrwE+mufPPC8EmOdPPH8XgeP857gVeAjPn3leCDDPn3j+doFj/Od4BvBgnj/zvBBgnj/x/H018FH85/gc4LN5/szzQoB5/sQL9tnAewMP4j/GM4DvBj6bF8w8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/GCfRXw1sCD+Y9xK/DTwMfwgpnnhQDz/Inn76uBj+I/x+cAn83zZ54XAszzJ56/i8Bx/nPcCjyE5888LwSY5088f7vAMf5zPAN4MM+feV4IMM+feP4+G/gs/nN8DvDZPH/meSHAPH/iBfts4L2BB/Ef4xnAdwOfzQtmnhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz98JYJf/GR4MPJ3nhYC/Bl6K5/XbwPsAt/Lf68HAdwGvzfP6GwFfDXwU/zt9jYAHA0/nf6eHiCu+Gvgo/nf5GuCjxbP9NfBS/O/wM8BbA4jn9NXAR/E/29cAH80ViOf1YOCjgdcGXor/Gf4G+G3gq4FbeTb+EXFsr9gjpTisAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileLines;
impl IconShape for FaFileLines {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0L64-.0001c-35.35 0-64 28.65-64 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM96 280C96 293.3 106.8 304 120 304h144C277.3 304 288 293.3 288 280S277.3 256 264 256h-144C106.8 256 96 266.8 96 280zM264 352h-144C106.8 352 96 362.8 96 376s10.75 24 24 24h144c13.25 0 24-10.75 24-24S277.3 352 264 352z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAFHElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/PbwE8D3wPs8m+DeE5fBXw0/7V2gY8Bvpt/PcSz/RXw0vz3eR/gu/nXQVzx1cBH8d/vfYDv5kWHgAcDT+d/jvcBvpsXDQK+Gvgo/md5H+C7+Zch4K+Al+Z5/Q7w3sCt/McyL5r3Ab6bFw4B5vl7CHAr//HMi+59gO/mBUOAef7Efw7zr/M+wHfz/CHAPH/iP4f513sf4Lt5Xggwz5/4z2H+bd4H+G6eEwLM8yf+c5h/u/cBvptnQ4B5/sR/DvPv8z7Ad3MFAszzJ/5zmH+/9wG+G0CAef7Efw7zH+N9gO8WYJ4/8Z/D/MfYBR4iwDx/4j/HLnCM/xgfI8A8f+I/x08Db8V/jN8RYJ4/8Z/jwcBfA8f490OAef7Ef54HA18NvDTwIP7tEGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8L8b8b4n83xP9uiP/dEC/YawPfBTyY5/TXwE8DXwPs8rxeG/gu4ME8p78Gvhr4Hp6/1wa+C3gwz9+twPsAv82zIV6wpwMP5gXbBT4G+G6e09OBB/OC/TXwOsAuz+npwIN54W4FHsKzIV4w86J5G+CneTbzL/tr4GV4TuZf9gzgwTwb4gUzL5pd4CHALleYF837AN/Ns5kX7hnAewO/zbMhXjDznAQ8GHhr4Kt4Tp8DfDZXmOck4MHAdwOvxbP9NfAyPJt5TuJfhnjBzHMSz/bRwFfxbH8NvAxXmOckrngw8HSek3g285zEvwzxgpnnJJ7twcDTeU7iCvOcxLOZ5ySezTwn8S9DvGDmOYlnezDwdJ6TuMI8J/Fs5jmJZzPPSfzLEC+YeU7i2T4b+Cye7W+Al+YK85zEs5nnJJ7NvHC3Au8D/DbPhnjBzHMS8GDgvYDP5jl9DvDZXGGek3g285zEs5l/2a3AQ3g2xAtmXjSXgAcDu1xhnpN4NvOcxLOZf9kzgAfzbIgXzLxo3gb4aZ7NPCfxbOY5iWczL9wzgPcGfptnQ7xg5oW7BLw38NM8J/OcxLOZ5ySezTwn8S9DvGDm+fsb4KeBrwZ2eV7mOYlnM89JPJt5TuJfhnjBzHMSLxrznMQVDwaeznMSz2aek/iXIV4w85zEi8Y8JwEPBr4LeG2e7W+Al+bZzHMS/zLEC2aek3jRmBfN+wDfzbOZ5yT+ZYgXzDwn8aIx/7K/AV6a52Sek/iXIV6wXeAYVzwDeDAvml3gGC/Y3wCvDezynHaBY1zxDODB/MsQL9hrA9/NFe8N/DYvmtcGvht4EM/pb4CvBr6b5++1ge/mivcGfpt/GeJ/N8T/boj/3RD/uyHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM83cC2OV/hgcDT+d5IeCvgZfief028D7Arfz3ejDwXcBr87z+RsBXAx/F/05fI+DBwNP53+kh4oqvBj6K/12+Bvho8Wx/DbwU/zv8DfDSAOI5fTXwUfzP9jXAR3MF4nk9GPho4LWBl+J/hr8Bfhv4auBWno1/BPla3fa/AZUAAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFilePdf;
impl IconShape for FaFilePdf {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 464C328.8 464 336 456.8 336 448V416H384V448C384 483.3 355.3 512 320 512H64C28.65 512 0 483.3 0 448V416H48V448C48 456.8 55.16 464 64 464H320zM256 160C238.3 160 224 145.7 224 128V48H64C55.16 48 48 55.16 48 64V192H0V64C0 28.65 28.65 0 64 0H229.5C246.5 0 262.7 6.743 274.7 18.75L365.3 109.3C377.3 121.3 384 137.5 384 154.5V192H336V160H256zM88 224C118.9 224 144 249.1 144 280C144 310.9 118.9 336 88 336H80V368C80 376.8 72.84 384 64 384C55.16 384 48 376.8 48 368V240C48 231.2 55.16 224 64 224H88zM112 280C112 266.7 101.3 256 88 256H80V304H88C101.3 304 112 293.3 112 280zM160 240C160 231.2 167.2 224 176 224H200C226.5 224 248 245.5 248 272V336C248 362.5 226.5 384 200 384H176C167.2 384 160 376.8 160 368V240zM192 352H200C208.8 352 216 344.8 216 336V272C216 263.2 208.8 256 200 256H192V352zM336 224C344.8 224 352 231.2 352 240C352 248.8 344.8 256 336 256H304V288H336C344.8 288 352 295.2 352 304C352 312.8 344.8 320 336 320H304V368C304 376.8 296.8 384 288 384C279.2 384 272 376.8 272 368V240C272 231.2 279.2 224 288 224H336z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEjklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ1641wa+C3gwL5pbgd8Gfhv4Hv71zPNCgHn+xAv3dODB/NvcCnw18DW86MzzQoB5/sQLZ/79vht4H1405nkhwDx/4oUz/zG+G3gf/mXmeSHAPH/ihTP/cT4H+GxeOPO8EGCeP/HCmf9YJ4BdXjDzvBBgnj/xwpnnTzx/rw18NPBWPH8fA3w1L5h5Xggwz5944czzJ164nwbeiuf1O8Br84KZ54UA8/yJF848f+KFe23gt3heu8AJXjDzvBBgnj/xwpnnT/zLzPMnXjDzvBBgnj/xwpnnT7xwrw38Fs+feMHM80KAef7EC2eeP/HC/Rbw2jyv3wFemxfMPC8EmOdPvHDm+RPP32sBnw28Ns/fxwBfzQtmnhcCzPMnXjjzH+sEsMsLZp4XAszzJ1448x/na4CP5oUzzwsB5vkTL5z5j/E3wEvzLzPPCwHm+RMvnPn3+xvgtYFd/mXmeSHAPH/ihTP/Pp8DfDWwy4vGPC8EmOdPvHDmX+cS8NfATwM/DdzKv455Xggwz5944czzJ/5zmOeFAPP8iRfOPH/iP4d5Xggwz5944czzJ/5zmOeFAPP8iRfOPH/iP4d5Xggwz5944XaBYzynZwAP5j+HeV4IMM+feOE+G/gsntPnAJ/Nfw7zvBBgnj/xL/ts4L254ruBz+Y/j3leCDDPn/ifxTwvBJjnT/zPYp4XAszzdwLY5X+GBwNP53kh4K+Bl+J5/TbwPsCt/Pd6MPBdwGvzvP5GwFcDH8X/Tl8j4MHA0/nf6SHiiq8GPor/Xb4G+GjxbH8NvBT/O/wM8NYA4jl9NfBR/M/2NcBHcwXieT0Y+GjgtYGX4n+GvwF+G/hq4FaejX8E4zjQ2KXpaU4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFilePowerpoint;
impl IconShape for FaFilePowerpoint {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM200 224H128C119.2 224 112 231.2 112 240v168c0 13.25 10.75 24 24 24S160 421.3 160 408v-32h44c44.21 0 79.73-37.95 75.69-82.98C276.1 253.2 240 224 200 224zM204 328H160V272h44c15.44 0 28 12.56 28 28S219.4 328 204 328z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEbElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV/1jmX+d9gO/m+UOAef4eAtzKfzzzr/c+wHfzvBBgnj/xn8P827wP8N08JwSY50/85zD/du8DfDfPhgDz/In/HObf532A7+YKBJjnT/znMP9+7wN8N4AA8/yJ/xzmP8b7AN8twDx/4j+H+Y+xCzxEgHn+xH8O8x/nYwSY50/85zD/cX5HgHn+xH+OXeAY/zEQYJ4/8Z/jp4G34j8GAszzJ/5zPBj4a+AY/34IMM+f+M/zYOCrgZcGHsS/HQLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4nk9GPgq4KWBB/Ovcyvw18DHALfyr2eeFwLM8yee04OBvwKO8++zC7wMcCv/OuZ5IcA8f+I5/TTwVvzH+BngrXlODwa+C3ht4HOAz+Y5meeFAPP8ied0ETjOf4xbgYfwbG8NfBdwnGcTz8k8LwSY5088J/MfS8Bx4LuAt+Z5iedknhcCzPMnnpP5j/U6wE8Bx3n+xHMyzwsB5vkTz8n81xLPyTwvBJjnTzwn819LPCfzvBBgnj/xnMx/LfGczPNCgHn+xHMy/7XEczLPCwHm+RPPyfzXEs/JPC8EmOdPPCfzX0s8J/O8EGCeP/GczH8t8ZzM80KAef7EczL/tcRzMs8LAeb5E8/J/Md6HeCngWM8f+I5meeFAPP8iedk/mMJOA58N/BWPC/xnMzzQoB5/sRz2gWO8R/jGcCDeba3Br4bOMaziedknhcCzPMnntNPA2/Ff4yfAd6a53Qc+GngtYDPAT6b52SeFwLM8yee04OBvwaO8e9zCXhp4Fb+dczzQoB5/sTzejDw1cBLAw/iX+cZwF8DHw3cyr+eeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/ydAHb5n+HBwNN5Xgj4a+CleF6/DbwPcCv/vR4MfBfw2jyvvxHw1cBH8b/T1wh4MPB0/nd6iLjiq4GP4n+XrwE+WjzbXwMvxf8OPwO8NYB4Tl8NfBT/s30N8NFcgXheDwY+Gnht4KX4n+FvgN8Gvhq4lWfjHwHPPr/YtiHEOAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileVideo;
impl IconShape for FaFileVideo {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM240 288c0-17.67-14.33-32-32-32h-96c-17.67 0-32 14.33-32 32v96c0 17.67 14.33 32 32 32h96c17.67 0 32-14.33 32-32v-16.52l43.84 30.2C292.3 403.5 304 397.6 304 387.4V284.6c0-10.16-11.64-16.16-20.16-10.32L240 304.5V288z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileWord;
impl IconShape for FaFileWord {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0H64C28.65 0 0 28.65 0 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h160L224 128c0 17.67 14.33 32 32 32h79.1V448zM214.6 248C211.3 238.4 202.2 232 192 232s-19.25 6.406-22.62 16L144.7 318.1l-25.89-77.66C114.6 227.8 101 221.2 88.41 225.2C75.83 229.4 69.05 243 73.23 255.6l48 144C124.5 409.3 133.5 415.9 143.8 416c10.17 0 19.45-6.406 22.83-16L192 328.1L217.4 400C220.8 409.6 229.8 416 240 416c10.27-.0938 19.53-6.688 22.77-16.41l48-144c4.188-12.59-2.594-26.16-15.17-30.38c-12.61-4.125-26.2 2.594-30.36 15.19l-25.89 77.66L214.6 248z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAE4ElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/fbwE8D3wPs8q+DeE5fBXw0/z12gY8BvpsXHeLZ/gp4af77vQ/w3bxoEFd8NfBR/M/xPsB38y9DwIOBp/M/z/sA380Lh4CvBj6K/5neB/huXjAE/BXw0jyv3wHeG7iV58+8cOL5M/867wN8N88fAszz9xDgVl4w88KJ58/8670P8N08LwSY50/85zD/Nu8DfDfPCQHm+RP/Ocy/3fsA382zIcA8f+I/h/n3eR/gu7kCAeb5E/85zL/f+wDfDSDAPH/ihTMvnHj+zH+M9wG+W4B5/sQLZ1448fyZ/xi7wEMEmOdPvHDmhRPPn/mP8zECzPMnXjjzwonnz/zH+R0B5vkT/zl2gWP8x0CAef7Ef46fBt6K/xgIMM+f+M/xYOCvgWP8+yHAPH/iP8+Dga8GXhp4EP92CDDPn3jhzAsn/mOZ54UA8/yJF868cOI/lnleCDDPn3jhzAsn/mOZ54UA8/yJF868cOI/lnleCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8iRfswcDTeeEeAtzKfxzzvBBgnj/xgr038F28cO8DfDf/cczzQoB5/sQL9tPAW/HC/Qzw1vzHMc8LAeb5Ey/YReA4L9wucIL/OOZ5IcA8f+L5e2ngr3jRvAzw1/zHMM8LAeb5E8/fRwNfxYvmY4Cv5j+GeV4IMM+feP5+GngrXjQ/A7w1/zHM80KAef7E83cROM6LTvzHMM8LAeb5E8/rtYHf4l/ndYDf5t/PPC8EmOdPPK/PBj6Lf53PAT6bfz/zvBBgnj/xvH4beC3+df4aeBn+/czzQoB5/sTzMv82J4Bd/n3M80KAef7Ec3pr4Kf4t3kb4Kf59zHPCwHm+RPP6auBj+Lf5muAj+bfxzwvBJjnTzyn3wZei2f7HeC9gVt5Tg8Gvht4LZ7tr4GX4d/HPC8EmOdPPCfznB4C3Mrz92Dg6Twn8e9jnhcCzPMnnpN5TieAXZ6/BwNP5zmJfx/zvBBgnj/xnP4aeCme7beB9wFu5Tk9GPgu4LV5tmcAD+bfxzwvBJjnTzynrwY+in+b7wHem38f87wQYJ4/8ZxeGvgr/m1eB/ht/n3M80KAef7E8/pq4KP41/ka4KP59zPPCwHm+RPP318DL8WL5m+A1wZ2+fczzwsB5vkTL9hXAx/FC/c1wGcDu/zHMM8LAeb5Ey/cg4GPBl4beCmu+Bvgt4HvBv6a/1jmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM83cC2OV/hgcDT+d5IeCvgZfief028D7Arfz3ejDwXcBr87z+RsBXAx/F/05fI+DBwNP53+kh4oqvBj6K/12+Bvho8Wx/DbwU/zv8DPDWAOI5fTXwUfzP9jXAR3MF4nk9GPho4LWBl+J/hr8Bfhv4auBWno1/BEr32Ng4V3/TAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFileZipper;
impl IconShape for FaFileZipper {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M365.3 93.38l-74.63-74.64C278.6 6.742 262.3 0 245.4 0L64-.0001c-35.35 0-64 28.65-64 64l.0065 384c0 35.34 28.65 64 64 64H320c35.2 0 64-28.8 64-64V138.6C384 121.7 377.3 105.4 365.3 93.38zM336 448c0 8.836-7.164 16-16 16H64.02c-8.838 0-16-7.164-16-16L48 64.13c0-8.836 7.164-16 16-16h48V64h64V48.13h48.01L224 128c0 17.67 14.33 32 32 32h79.1V448zM176 96h-64v32h64V96zM176 160h-64v32h64V160zM176 224h-64l-30.56 116.5C73.51 379.5 103.7 416 144.3 416c40.26 0 70.45-36.3 62.68-75.15L176 224zM160 368H128c-8.836 0-16-7.164-16-16s7.164-16 16-16h32c8.836 0 16 7.164 16 16S168.8 368 160 368z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAADiElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5z/PbwE8D3wPs8m+DeE5fBXw0/7V2gY8Bvpt/PcSz/RXw0vz3eR/gu/nXQVzx1cBH8d/vfYDv5kWHgAcDT+d/jvcBvpsXDQK+Gvgo/md5H+C7+Zch4K+Al+Z5/Q7w3sCt/McyL5r3Ab6bFw4B5vl7CHAr//HMi+59gO/mBUOAef7Efw7zr/M+wHfz/CHAPH/iP4f513sf4Lt5Xggwz5/4z2H+bd4H+G6eEwLM8yf+c5h/u/cBvptnQ4B5/sR/DvPv8z7Ad3MFAszzJ/5zmH+/9wG+G0CAef7Efw7zH+N9gO8WYJ4/8Z/D/MfYBR4iwDx/4j/HLnCM/xgfI8A8f+I/x08Db8V/jN8RYJ4/8Z/jwcBfA8f490OAef7Ef54HA18NvDTwIP7tEGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vkT/7OY54UA8/yJ/1nM80KAef7E/yzmeSHAPH/ifxbzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/E/i3leCDDPn/ifxTwvBJjnT/zPYp4XAszzJ/5nMc8LAeb5E/+zmOeFAPP8if9ZzPNCgHn+xP8s5nkhwDx/4n8W87wQYJ4/8T+LeV4IMM+f+J/FPC8EmOdP/M9inhcCzPMn/mcxzwsB5vk7AezyP8ODgafzvBDw18BL8bx+G3gf4Fb+ez0Y+C7gtXlefyPgq4GP4n+nrxHwYODp/O/0EHHFVwMfxf8uXwN8tHi2vwZeiv8d/gZ4aQDxnL4a+Cj+Z/sa4KO5AvG8Hgx8NPDawEvxP8PfAL8NfDVwK8/GPwLO3JXafh8DZQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFile;
impl IconShape for FaFile {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 64C0 28.65 28.65 0 64 0H229.5C246.5 0 262.7 6.743 274.7 18.75L365.3 109.3C377.3 121.3 384 137.5 384 154.5V448C384 483.3 355.3 512 320 512H64C28.65 512 0 483.3 0 448V64zM336 448V160H256C238.3 160 224 145.7 224 128V48H64C55.16 48 48 55.16 48 64V448C48 456.8 55.16 464 64 464H320C328.8 464 336 456.8 336 448z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEv0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjiq4C35orvBj6HF91rAS8NPBh4aa54bZ5tF/hrrvhr4Fbgd4C/5r/GSwMfBbw1cBz4a+Cvgc8BbhXw1cBH8Zw+B/hsnr/XAt4aeG3gpfm32wV+G/hp4GeAXf7jHAfeCvho4KV5/naBlxFwETjOc9oFTvBsbwW8NfDWwHH+c/w18N3AzwC38q93HHgr4LWBtwaO8y/7GQHm+XsI8FHAWwMP5r/WrcBvA38N/DXwDOBWntNrAQ8GHgy8NvDa/OvtCjD/fyHA/P+FAPPv8wzgVuC3gb8GdoFd4K+54jjw0sBx4KWB1wZei/9azwAexPNCgPnX+x3gp4HfBv6af5u3Bt4aeG3gQfzn+B7gu4HfBszzQoB50fwN8NXATwO7/Md6aeC9gdcGXop/u0vATwO/Dfw0sMuzmeeFAPOCPQP4buC7gVv5r3EceG3gpYGXBo4DLw0c4zn9Dlf8NnAr8NfAX/OCmeeFAPP8vQ3w0/zfYZ4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xAv3YOBBwGsDx4GX5ooHAw/m2XaBv+aKXeCvgVuBvwH+mv865nkhwDx/4jk9GHgt4LWB1wYezH+M3wb+Gvht4HeAXf5zmOeFAPP8CTgOvBXw3sBr81/jr4HfBn4b+B1gl3+7lwbeC3hr4ME8LwSY5++ngbfmv9+twG8DtwJ/DewCzwBu5dkeDDwIeDDwYOC1gdfmX4YA8//X3wgw/3+9jQDzb/c3wK3AXwO/zRW7wF/zbA8GHswVLw0cB14beGngGP89LgEfDXy3APOiewbw28BvAz8N7PLv89LASwOvDbw28CD+c/0M8NPATwO7AALMC3cJ+Gngq4G/5j/Xg4HXBl4beG3gQfz7XAJ+G/ht4KeBW3lOCDDP398AXw38NLDLf4/jwEsDrw0cB16aK16L5/U7XPHbwC7w28Bf88IhwDx/4v8+BJjnT/zfhwDz/In/+xBgnj/xfx8CzPMn/u9DgHn+xP99CDDPn/i/DwHm+RP/9yHAPH/i/z4EmOdP/N+HAPP8if/7EGCeP/F/HwLM8yf+70PALnCM5/QM4MH834eArwY+iuf0OcBn838f4orPBt6bK74b+Gz+f+AfAQmq3/pkLHMuAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFlag;
impl IconShape for FaFlag {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M476.3 0c-6.365 0-13.01 1.35-19.34 4.233c-45.69 20.86-79.56 27.94-107.8 27.94c-59.96 0-94.81-31.86-163.9-31.87c-34.63 0-77.87 8.003-137.2 32.05V24C48 10.75 37.25 0 24 0S0 10.75 0 24v464C0 501.3 10.75 512 24 512s24-10.75 24-24v-104c53.59-23.86 96.02-31.81 132.8-31.81c73.63 0 124.9 31.78 198.6 31.78c31.91 0 68.02-5.971 111.1-23.09C504.1 355.9 512 344.4 512 332.1V30.73C512 11.1 495.3 0 476.3 0zM464 319.8c-30.31 10.82-58.08 16.1-84.6 16.1c-30.8 0-58.31-7-87.44-14.41c-32.01-8.141-68.29-17.37-111.1-17.37c-42.35 0-85.99 9.09-132.8 27.73V84.14l18.03-7.301c47.39-19.2 86.38-28.54 119.2-28.54c28.24 .0039 49.12 6.711 73.31 14.48c25.38 8.148 54.13 17.39 90.58 17.39c35.43 0 72.24-8.496 114.9-26.61V319.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEsElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/367wE8D3wP8Nv86iOf0VcBH8z/XRwNfw4sO8Wx/Bbw0//N9N/A+vGgQV3w18FH87/HdwPvwL0PAg4Gn87/PdwPvwwuHgK8GPor/nb4beB9eMAT8FfDSPK/fAd4buJUrzPMn/mOY5+9vgJfiBftu4H14/hBgnr+HALfybOb5E/8xzPN3Avht4KV4wb4beB+eFwLM8yeek3n+xH8M8/wJOA78NvBSvGDfDbwPzwkB5vkTz8k8f+I/hnn+xBXHgd8GXooX7LuB9+HZEGCeP/GczPMn/mOY508823Hgt4GX4gX7buB9uAIB5vkTz8k8f+I/hnn+xHM6Dvw28FK8YN8NvA+AAPP8iedknj/xH8M8f+J5HQd+G3gpXrCPAb5agHn+xHMyz5/4j2GeP/H8HQd+G3gpXrCHCDDPn3hO5vkT/zHM8ydesOPAbwMvxfP3PQLM8yeek3n+xH8M8/yJF+448NvAS/G8dgWY5088J/PfQ/zLHgw8neeFAPP8iedk/nuIF415Xggwz594Tua/h3jRmOeFAPP8iee0Cxzjv9YzgAfzojHPCwHm+RPP6bOBz+K/1ucAn82LxjwvBJjnTzyvzwbeG3gQ/7meAXw38Nm86MzzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/8xHgx8FPDawEtzxV8Dvw18DXAr/zHM80KAef7Ev99XAR/NC/fVwMfw72eeFwLM8yf+ff4KeGleNL8NvA7/PuZ5IcA8f+Lf7quBj+Jf52uAj+bfzjwvBJjnT/zbPBh4Ov82DwFu5d/GPC8EmOdP/Nt8NfBR/Nt8DfDR/NuY54UA8/yJf5u/Al6af5u/Bl6GfxvzvBBgnj/xb2P+fcS/jXleCDDPn/i3Mf8+4t/GPC8EmOdP/Nv8NfBS/Nv8DfDS/NuY54UA8/yJf5uvBj6Kf5uvAT6afxvzvBBgnj/xb/Ng4On82zwEuJV/G/O8EGCeP/Fv99XAR/Gv8zXAR/NvZ54XAszzJ/59/hp4KV40PwO8Nf8+5nkhwDx/4t/vq4GP4oX7GuCj+fczzwsB5vkT/zEeDHw08NrAS3HF3wC/DXw1cCv/MczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjn7wSwy/8ODwaezvNCwF8DL8Xz+m3gfYBb+Z/twcB3Aa/N8/obAV8NfBT/N32NgAcDT+f/poeIK74a+Cj+b/ka4KPFs/018FL83/A3wEsDiOf01cBH8b/b1wAfzRWI5/Vg4KOB1wZeiv8d/gb4beCrgVt5NsT/bYj/2xD/tyH+b+MfARAbwk+6mwOMAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFloppyDisk;
impl IconShape for FaFloppyDisk {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M224 256c-35.2 0-64 28.8-64 64c0 35.2 28.8 64 64 64c35.2 0 64-28.8 64-64C288 284.8 259.2 256 224 256zM433.1 129.1l-83.9-83.9C341.1 37.06 328.8 32 316.1 32H64C28.65 32 0 60.65 0 96v320c0 35.35 28.65 64 64 64h320c35.35 0 64-28.65 64-64V163.9C448 151.2 442.9 138.9 433.1 129.1zM128 80h144V160H128V80zM400 416c0 8.836-7.164 16-16 16H64c-8.836 0-16-7.164-16-16V96c0-8.838 7.164-16 16-16h16v104c0 13.25 10.75 24 24 24h192C309.3 208 320 197.3 320 184V83.88l78.25 78.25C399.4 163.2 400 164.8 400 166.3V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADUElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9WDgo4DXBl6af7/fBn4a+B5gl/9ZEM/pq4CP5j/HLvAxwHfzPwfi2f4KeGn+83028Dn8z4C44quBj+K/zncD78N/PwQ8GHg6//W+G3gf/nsh4KuBj+K/x28DbwPs8t8DAX8FvDTP63eA9wZu5d/utYGvBl6KF+yvgdcBdvmvhwDz/D0EuJV/v+PAbwMvxQv218DrALv810KAef7Ef5zjwG8DL8V/j78Gfhv4GuBWng0B5vkT/7GOA78NvBT/vb4a+BiuQIB5/sR/vOPAVwPvxX+vvwZeBkCAef7Ef57vBt6L/15fA3y0APP8if9c3w28F/+9HiLAPH/iP993A+/Ff5+vEWCeP/F/w4OB7wZei+f11wLM8yf+73gw8HSeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcD8/4UA8/8XAsz/Xwgw/38hwPz/hQDz/xcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM/fCWCX/xseDDyd54WAvwZeiuf128D7ALfyv9uDge8CXpvn9TcCvhr4KP5/+hoBDwaezv9PDxFXfDXwUfz/8jXAR4tn+2vgpfj/4W+AlwYQz+mrgY/i/7avAT6aKxDP68HARwOvDbwU/zf8DfDbwFcDt/JsiP/fEP+/If5/Q/z/xj8C42WBHnx7b5EAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFolderClosed;
impl IconShape for FaFolderClosed {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M448 96h-172.1L226.7 50.75C214.7 38.74 198.5 32 181.5 32H64C28.65 32 0 60.66 0 96v320c0 35.34 28.65 64 64 64h384c35.35 0 64-28.66 64-64V160C512 124.7 483.3 96 448 96zM64 80h117.5c4.273 0 8.293 1.664 11.31 4.688L256 144h192c8.822 0 16 7.176 16 16v32h-416V96C48 87.18 55.18 80 64 80zM448 432H64c-8.822 0-16-7.176-16-16V240h416V416C464 424.8 456.8 432 448 432z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAEGklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIjn9WDgo4DXBl6af7/fBn4a+B5gl/9dEM/pq4CP5j/HLvAxwHfzvwfi2f4KeGn+83018DH874C44quBj+K/zncD78P/fAh4MPB0/ut9N/A+/M+GgK8GPor/Hr8NvA2wy/9MCPgr4KV5Xr8DvDdwK/92rw18NfBSvGB/DbwOsMv/PAgwz99DgFv59zsO/DbwUrxgfw28DrDL/ywIMM+f+I9zHPht4KX4r/HXwG8DXwPcyr8dAszzJ/5jHQd+G3gp/mt9NfAx/NsgwDx/4j/eceCrgffiv9ZfAy/Dvx4CzPMn/vN8N/Be/Nf6GuCj+ddBgHn+xH+u7wbei/9aDwFu5UWHAPP8if983w28F/91vgb4aF50CDDPn/jf6cHAdwOvxfP6a+BleNEhwDx/4n+vBwNP5/kTLzoEmOdP/O9mnj/xokOAef7E/27m+RMvOgSY50/872aeP/GiQ4B5/sT/bub5Ey86BJjnT1yFAPP8iasQYJ4/cRUCzPMnrkKAef7Ei+azgPcGHsz/DbcCfw18DHCrAPP8iX/ZdwHvzf9Nu8DLCDDPn3jhXhv4Lf5v+x4B5vkTL9xvAa/N/223CjDPn3jBXhv4Lf7vQ4B5/sQL9lvAa/N/388IMM+feP5eG/gt/n94HQHm+RPP328Br83/fb8DvLYA8/yJ5/XawG/x/8PrAL8twDx/4nn9FvDa/N/3O8BrAwgwz594Tq8N/Bb/P7wO8NsAAszzJ57TbwGvzf99vwO8NlcgwDx/4tleG/gt/n94HeC3uQIB5vkTz/ZbwGvzf9/vAK/NsyHAPH/iitcGfov/H14H+G2eDQHm+RNX/Bbw2vzf9zvAa/OcEGCePwGvDfwW/z+8DvDbPCcEmOdPwG8Br83/fb8DvDbPCwHm+Xsd4Lf4/+F1gN/meSHAPH+/Dbw2//f9DvDaPH8IMP+/vQ7w2zx/CDD/f/0O8Nq8YAgw/3+9DvDbvGAIMP8//Q7w2rxwCDD/P70O8Nu8cAj4a+Cl+P/ld4DX5l+GgK8GPor/X14H+G3+ZQh4MPB0/v/4HeC1edEgrvhq4KP4/+F1gN/mRYN4tr8GXor/234HeG1edIjn9NXAR/F/0yXgpYFbedEhnteDgY8GXht4Kf73uwT8NPDZwK386yCuemEQV70wiKteGMRVLwz/CBuDls+Le4cdAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFolderOpen;
impl IconShape for FaFolderOpen {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M572.6 270.3l-96 192C471.2 473.2 460.1 480 447.1 480H64c-35.35 0-64-28.66-64-64V96c0-35.34 28.65-64 64-64h117.5c16.97 0 33.25 6.742 45.26 18.75L275.9 96H416c35.35 0 64 28.66 64 64v32h-48V160c0-8.824-7.178-16-16-16H256L192.8 84.69C189.8 81.66 185.8 80 181.5 80H64C55.18 80 48 87.18 48 96v288l71.16-142.3C124.6 230.8 135.7 224 147.8 224h396.2C567.7 224 583.2 249 572.6 270.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADbUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9drAewGvDTyYf7/fBn4a+B5gl/9ZEM92HPgu4K35z7ELfAzw3fzPgbjiOPBbwEvzn++zgc/hfwbEFb8FvDb/db4beB/++yHgtYHf4r/edwPvw38vBPw08Fb89/hu4H3474MA8/x9D/DRwC7/dq8N/DRwjBfst4G3AXb5r4cA8/yJ/xgvDfw2cIwX7K+B1wF2+a+FAPP8if84x4HfBl6K/x5/Dfw28DXArTwbAszzJ/5jHQd+G3gp/nt9NfAxXIEA8/yJ/3jHgd8GXor/Xt8NvA+AAPP8if8cx4HfBl6K/16fA3y2APP8if9c3w28F/99doGXEWCeP/Gf77uB9+K/z8cIMM+f+L/hwcBPAy/F8/odAeb5E/93vDbwWzyv3xFgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/x2sDv8Xz+h0B5vn7buBjgF3+d3sw8FPAS/O8fkfATwNvxf9PHyPgtYHf4v+fS8CDxRW/DbwW/798DvDZ4orjwG8DL8X/D98DvDeAeLbjwHcDb8X/bZ8DfDZXIJ7XawPvDbw28CD+b/gd4K+BrwZu5dkQ/78h/n9D/P+G+P+NfwTYnYcxCLsV2gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFolder;
impl IconShape for FaFolder {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M447.1 96h-172.1L226.7 50.75C214.7 38.74 198.5 32 181.5 32H63.1c-35.35 0-64 28.66-64 64v320c0 35.34 28.65 64 64 64h384c35.35 0 64-28.66 64-64V160C511.1 124.7 483.3 96 447.1 96zM463.1 416c0 8.824-7.178 16-16 16h-384c-8.822 0-16-7.176-16-16V96c0-8.824 7.178-16 16-16h117.5c4.273 0 8.293 1.664 11.31 4.688L255.1 144h192c8.822 0 16 7.176 16 16V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEU0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYgrvgp4a674buBz+L8BAV8NfBTP6WuAj+Z/PwRcBI7znG4FHsKL5sHAWwEvDTwYeGngOM92K3Ar8NfAXwM/A+zyn+vBwFsB7y3APH/iBXsw8F7AewMP5l/vr4HvBr4H2OU/xnHgrYC3Bt6aKxBgnj/xnB4MvBXw3sBL8x/np4GfBn4G2OVf762Atwbem+eFAPP8vTbwYOClgdcGXpr/fL8N/Dbw18Au8Ds8p5cGjgGvDbw28Nq8cAgw/3chwPzfhQDz7/MM4KeBvwZuBf4a2OXZHgw8GHhp4KWBtwaO8V8DAeZf7xnAVwM/DdzKv95LA+8NvDdwjP88CDAvmmcAPw18N/DX/Md5a+CtgbcGjvGvcwn4aeCngZ/ieSHAPH+/A/w2cCvw28Ct/Od7beC1gZcGjgOvxXP6HWAX+Gvgt4Hf5tnM80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xonkw8FbASwMPBl4aOM6z3QrcCvw18NfAzwC7/MczzwsB5vkTL9iDgfcC3ht4MP96fw18N/A9wC7/MczzQoB5/sTz99rATwHH+Y/x08BPAz8D7PKvcxx4K+CtgbfmeSHAPH/i+Xs68GD+c/w28NvAXwO7wO/wnF4aOAa8NvDawGvzwiHAPH/i+TP/eyDAPH/i+ftp4K343wEB5vkTz99LA98NvBRXPAP4aeCvgVuBvwZ2ebYHAw8GXhp4aeCtgWP853oG8NPAdwswz5944V4a2AVu5V/vpYH3Bt4bOMZ/jEvATwM/Dfw0VyDAPH/iv8ZbA28NvDVwjH+dS8BvAz8NfDfPCwHm+RP/9V4beG3gpYHjwGvxnP4G2AV+G/ht4Ld54RBgnj/xvx8CzPMn/vdDgHn+xP9+CDDPn/jfDwHm+RP/+yFgFzjGc3oG8GD+90PAVwMfxXP6HOCz+d8PccVnA+/NFd8NfDb/NyD+b0P834b4vw3xfxv/CEFAuxf0j/rEAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFontAwesome;
impl IconShape for FaFontAwesome {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M448 48V384c-63.09 22.54-82.34 32-119.5 32c-62.82 0-86.6-32-149.3-32c-21.69 0-38.48 3.791-53.74 8.766C110.1 397.5 96 386.1 96 371.7v-.7461c0-9.275 5.734-17.6 14.42-20.86C129.1 342.8 150.2 336 179.2 336c62.73 0 86.51 32 149.3 32c25.5 0 42.85-4.604 71.47-14.7v-240C379.2 120.6 357.7 128 328.5 128c-.0039 0 .0039 0 0 0c-62.81 0-86.61-32-149.3-32C122.1 96 98.8 122.1 48 126.1V456C48 469.3 37.25 480 24 480S0 469.3 0 456V56C0 42.74 10.75 32 24 32S48 42.74 48 56v22.99C98.8 74.14 122.1 48 179.2 48c62.77 0 86.45 32 149.3 32C366.1 80 386.8 69.85 448 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaFutbol;
impl IconShape for FaFutbol {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M177.1 228.6L207.9 320h96.5l29.62-91.38L256 172.1L177.1 228.6zM255.1 0C114.6 0 .0001 114.6 .0001 256S114.6 512 256 512s255.1-114.6 255.1-255.1S397.4 0 255.1 0zM435.2 361.1l-103.9-1.578l-30.67 99.52C286.2 462.2 271.3 464 256 464s-30.19-1.773-44.56-4.93L180.8 359.6L76.83 361.1c-14.93-25.35-24.79-54.01-27.8-84.72L134.3 216.4L100.7 118.1c19.85-22.34 44.32-40.45 72.04-52.62L256 128l83.29-62.47c27.72 12.17 52.19 30.27 72.04 52.62L377.7 216.4l85.23 59.97C459.1 307.1 450.1 335.8 435.2 361.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaGem;
impl IconShape for FaGem {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M507.9 196.4l-104-153.8C399.4 35.95 391.1 32 384 32H127.1C120 32 112.6 35.95 108.1 42.56l-103.1 153.8c-6.312 9.297-5.281 21.72 2.406 29.89l231.1 246.2C243.1 477.3 249.4 480 256 480s12.94-2.734 17.47-7.547l232-246.2C513.2 218.1 514.2 205.7 507.9 196.4zM382.5 96.59L446.1 192h-140.1L382.5 96.59zM256 178.9L177.6 80h156.7L256 178.9zM129.5 96.59L205.1 192H65.04L129.5 96.59zM256 421L85.42 240h341.2L256 421z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAFYklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj3ezDwUcBrAy/Nc/pt4K+BrwFu5b8e4t/nq4CP5kXz1cDH8F8L8W/3V8BL86/z08Db8F8H8W/z1cBH8W/z18Auz+mvgd8Gfob/WIh/vQcDT+c/x63AxwA/zX8MxL/eVwMfxX+tvwb+Gvhp4Gd40SH+9f4KeGme1+8A7w3cyhUPBr4beC3+Y/018D7AX/MvQ/zrmefvIcCtPKcHA0/nP94u8DrAX/PCIf71zPMnnj/zn2MX+Grgt4G/AXZ5Xoh/PfP8iefPPKe/Ab4auJUrHgx8NPBS/Pv8NPA5wF/zbIh/PfP8iefvVuBBXHEJeDCwy3M6Dvw18CD+/T4a+BquQPzrmedPPH+vDfw0V3w08N08fw8GPpsrvhr4a57tvYGvBo7xovka4KMBxL+eef7Ef673Br6LF93bAD8t/vXM8yf+8/028Fq8aG4FXkb867w38F08f+I/33HgvYHjXPFg4K2BYzx/7yNedO8NfBcvmPjv8WDgt4EH8bx+Rrxovgt4b1448d/nrYGf4nndKv5l3wW8N/8y8d/nOHCR54V44b4LeG9eNOK/l3leiBfsu4D35kUn/nuZ54V4/l4b+C1esL8BXornJP57meeFeP6eDjyY5+9vgNcGLvKcxH8v87wQz+utgZ/i+fsb4LWBXcA8J/HfyzwvxPP6buC9eF7PAF4a2OUK85zEfy/zvBDP67eB1+J5vQ/w3TybeU7iv5d5XojndRE4zvM6AezybOY5if9e5nkhnpd5/sRz2gWOccUzgAfz38s8L8TzMs+feE6vDXw3V7w38Nv89zLPC/G8zPMn/mczzwvxvMzzJ/5nM88L8bzM8yf+ZzPPC/G8zPMn/mczzwvxvMzzJ/5nM88L8bzM8yf+ZzPPC/G8zPMn/mczzwvxvMzzJ/7nOg5c5Hkhnpd5/sT/XK8N/BbP63fE89oFjvG8Xgb4a/5n+mrgo3he3yOe128Dr8Xz+hrgo/mf6enAg3le7yOe10cDX8Xz9zLAX/M/y3sD38Xz9xDxvB4MPJ3nbxf4bOBr+J/hOPBXwIN5Xr8DvLZ4/r4beC9esL8GPgb4bf57fRfw3jx/7wN8t3j+Hgz8NXCMF+6ngY8BbuW/3mcBn83z9wzgwQDiBXtr4Kd40Xw18DnALv/5jgNfBbw3L9jrAL8NIF649wa+GjjGv2wX+Grgc/jP9VfAS/OCfQ/w3lyB+Je9NPDdwEvxohH/ucwL9jfAawO7XIF40b038NXAMV448Z/LPH9/A7w2sMuzIf51jgMfDXw0cIznT/znMs/fCWCX54T4t3kw8HSeP/Gfyzx/4nkh/u3M8yf+c5nnTzwvxL+def7Efy7z/Innhfi3M8+f+M9lnj/xvBD/dub5E/+5zPMnnhfi3848f+I/l3n+xPNC/NuZ50/85zLPn3heiH878/yJ/1zm+RPPC/FvZ54/8Z/LPH/ieSH+7czzJ/5zmedPPC/Ev515/sR/LvP8ieeF+Lczz5/4z2WeP/G8EP925vkT/7nM8yeeF+Lfbhc4xnN6BvBg/nPtAsd4Ts8AHszzQvzbfTXwUTynzwE+m/9cXw18FM/pc4DP5nkh/u2OA58NvDewC3w38Nn85zsOfDbw3sAu8N3AZ/P88Y/4G9cN28qR3QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandBackFist;
impl IconShape for FaHandBackFist {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M377.1 68.05C364.4 50.65 343.7 40 321.2 40h-13.53c-3.518 0-7.039 .2754-10.53 .8184C284.8 31.33 269.6 26 253.5 26H240c-3.977 0-7.904 .3691-11.75 1.084C216.7 10.71 197.6 0 176 0H160C124.7 0 96 28.65 96 64v49.71L63.04 143.3C43.3 160 32 184.6 32 210.9v78.97c0 32.1 17.11 61.65 44.65 77.12L112 386.9v101.1C112 501.3 122.7 512 135.1 512S160 501.3 160 488v-129.9c-1.316-.6543-2.775-.9199-4.062-1.639l-55.78-31.34C87.72 318.2 80 304.6 80 289.9V210.9c0-12.31 5.281-23.77 14.5-31.39L112 163.8V208C112 216.8 119.2 224 128 224s16-7.156 16-16V64c0-8.828 7.188-16 16-16h16C184.8 48 192 55.17 192 64v16c0 9.578 7.942 16.04 16.15 16.04c6.432 0 12.31-4.018 14.73-10.17C223.3 84.84 228.3 74 240 74h13.53c20.97 0 17.92 19.58 34.27 19.58c8.177 0 9.9-5.584 19.88-5.584h13.53c25.54 0 18.27 28.23 38.66 28.23c.1562 0 .3125-.002 .4668-.0078L375.4 116C388.1 116 400 127.7 400 142V272c0 36.15-19.54 67.32-48 83.69v132.3C352 501.3 362.7 512 375.1 512S400 501.3 400 488v-108.1C430.1 352.8 448 313.6 448 272V142C448 102.1 416.8 69.44 377.1 68.05z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFFUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4Z4Tg8GPgt4a+A4/71+myt+GvgeYJf/eIhn+yzgs/mf67uBjwF2+Y+DuOKrgY/if75d4H2An+Y/BgJeG/gt/nd5H+C7+fdDwHcD78X/Pu8DfDf/Pgh4OvBg/nd6GeCv+bdDgHn+TgC7/Pd5beCtgfcGjvH8/TXwOsAu/zYIMM/fywB/zX+/BwM/DbwUz9/XAB/Nvw0CzPP3McBX8z/DSwO/DRzj+Xsd4Lf510PATwNvxfPaBV4H+Gv+Z3hr4Kd4/naBhwC7/Osg4K2Bn+L52wU+GvgZYJf/fj8NvBXP308Db8O/DuKKvwZeiv96twJ/DXwN8Nv8y44Dfw08iOfvbYCf5kWHuOKlgb/iv9dPA+8D7PLCvTbwWzx/u8DLALfyokE823sD38V/r58G3oZ/2VcDH8Xz99vA6/CiQTyn9wa+i/9e7wN8N/+yvwZeiufvY4Cv5l+GeF4vDXw18Fr897gVeAj/spcG/ooX7GWAv+aFQ7xgLw28N/DSwGvxX0u8aD4b+Cyev78GXoYXDvHf66+Bl+J5vQ7w27xofht4LZ6/zwE+mxcM8d/rInCc5/UywF/zonkw8NfAMZ6/1wF+m+cP8d/nrYGf4nldAo7zr/PewHfx/N0KvAywy/NC/Pf5buC9eF7fA7w3/3o/DbwVz993A+/D80L89zgOPB04zvN6G+Cn+dc7DtwKHOP5exvgp3lOiH+b1wbeCnhp4LX5j3MJOM6/3VsDP8Xztws8BNjl2RD/Oq8NfBbw2vzn+B7gvfn3+Wrgo3j+fhp4G54N8aJ7b+C7+M/1EOBW/n2OA38NPIjn72OAr+YKxIvmvYHv4j/XxwBfzX+Mlwb+iudvF3gZ4FYA8S97aeCv+M/1PsB38x/rs4HP4vn7a+BlAMS/7LeA1+Y/3t8Afw18NnAr/zn+Gngpnr/3Ab5bvHCvDfwWz98l4KOBnwZ2+Z/ppYHfBo7xvG4FHiJeuK8GPorndQl4beCv+Z/vo4Gv4vl7G/HC/TbwWjyvjwG+mv89bgUexPP6GvHCmefvZYC/5n+P9wa+i+f11+KFM8/fCWCX/z1eGvgrnhfihbsVeBD/Nb4b+BzgVv5zmOeFeOG+G3gv/uvsAu8D/DT/8czzQrxwLw38Ff/1vhr4GP5jmeeF+Jd9NfBR/Nf7a+BtgFv5j2GeF+JF89nAZ/Ffbxd4H+Cn+fczzwvxonsw8NnAawMP4r/WVwMfw7+PeV6I/34vDfw2cIwX7q+BtwFu5d/GPC/E/wzHgZ8GXosXbhd4H+Cn+dczzwvxP8tnA5/Fv+xtgJ/mX8c8L8T/PK8N/DRwjBfsVuAh/OuY54X4n+k48NPAa/GCiX8d87wQ/7N9NfBRPH/iX8c8L8T/fOZ5/Q7w2vzr7ALHeE7PEP/z/TTwVjynjwG+mn+dzwY+i+f0OeJ/vuPAZwPvDewC3w18Nv82nw28N1d8N/DZ4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G/8IYqW+Q6q93+gAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandLizard;
impl IconShape for FaHandLizard {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 331.8V424c0 13.25-10.75 24-24 24c-13.25 0-24-10.75-24-24v-92.17c0-10.09-3.031-19.8-8.766-28.08l-118.6-170.5C327.4 119.1 312.2 112 295.1 112H53.32c-2.5 0-5.25 2.453-5.313 4.172c-.2969 9.5 3.156 18.47 9.75 25.28C64.36 148.3 73.2 152 82.67 152h161.8c17.09 0 33.4 8.281 43.4 22.14c9.984 13.88 12.73 31.83 7.328 48.05l-9.781 29.34C278.2 273.3 257.8 288 234.9 288H138.7C129.2 288 120.4 291.8 113.8 298.5c-6.594 6.812-10.05 15.78-9.75 25.28C104.1 325.5 106.8 328 109.3 328h156.6c5.188 0 10.14 1.688 14.3 4.797l78.22 58.67c6.031 4.531 9.594 11.66 9.594 19.2L367.1 424c0 13.25-10.75 24-24 24s-24-10.75-24-24v-1.328L257.8 376H109.3c-28.48 0-52.39-22.72-53.28-50.64c-.7187-22.61 7.531-43.98 23.23-60.2C94.1 248.9 116.1 240 138.7 240h96.19c2.297 0 4.328-1.469 5.063-3.656l9.781-29.33c.7031-2.141-.0156-3.797-.7344-4.797C248.2 201.2 246.9 200 244.6 200H82.67c-22.58 0-43.67-8.938-59.39-25.16C7.575 158.6-.6755 137.3 .0433 114.6C.9339 86.72 24.84 64 53.32 64h242.7c31.94 0 61.86 15.67 80.05 41.92l118.6 170.5C506 292.8 512 311.9 512 331.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPeace;
impl IconShape for FaHandPeace {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M412 160c-8.326 0-16.3 1.51-23.68 4.27C375.1 151.8 358.9 144 340 144c-11.64 0-22.44 3.223-32.03 8.418l11.12-68.95c.6228-3.874 .9243-7.725 .9243-11.53c0-36.08-28.91-71.95-72.09-71.95c-34.68 0-65.31 25.16-71.03 60.54L173.4 82.22L168.9 72.77c-12.4-25.75-38.07-40.78-64.89-40.78c-40.8 0-72.01 33.28-72.01 72.07c0 10.48 2.296 21.11 7.144 31.18L89.05 238.9C64.64 250.4 48 275.7 48 303.1v80c0 22.06 10.4 43.32 27.83 56.86l45.95 35.74c29.35 22.83 65.98 35.41 103.2 35.41l78.81 .0352C400.9 512 480 432.1 480 335.8v-107.5C480 189.6 447.9 160 412 160zM320 212.3C320 201.1 328.1 192 340 192c11.02 0 20 9.078 20 20.25v55.5C360 278.9 351 288 340 288C328.1 288 320 278.9 320 267.8V212.3zM247.9 47.98c12.05 0 24.13 9.511 24.13 23.98c0 1.277-.1022 2.57-.3134 3.871L248.4 220.5C240.7 217.6 232.4 215.1 223.9 215.1c0 0 .002 0 0 0c-4.475 0-8.967 .4199-13.38 1.254l-10.55 1.627l24.32-150.7C226.2 56.42 236.4 47.98 247.9 47.98zM79.1 104c0-13.27 10.79-24.04 24.02-24.04c8.937 0 17.5 5.023 21.61 13.61l61.29 127.3L137.3 228.5L82.38 114.4C80.76 111.1 79.1 107.5 79.1 104zM303.8 464l-78.81-.0352c-26.56 0-52.72-8.984-73.69-25.3l-45.97-35.75C99.47 398.4 96 391.3 96 383.1v-80c0-11.23 7.969-21.11 17.59-23.22l105.3-16.23C220.6 264.2 222.3 263.1 223.9 263.1c11.91 0 24.09 9.521 24.09 24.06c0 11.04-7.513 20.95-17.17 23.09L172.8 319c-12.03 1.633-20.78 11.92-20.78 23.75c0 20.21 18.82 24.08 23.7 24.08c2.645 0 64.61-8.619 65.54-8.826c23.55-5.227 41.51-22.23 49.73-43.64C303.3 327.5 320.6 336 340 336c8.326 0 16.31-1.51 23.69-4.27C376 344.2 393.1 352 412 352c.1992 0 10.08-.4453 18.65-2.92C423.9 413.5 369.9 464 303.8 464zM432 283.8C432 294.9 423 304 412 304c-11.02 0-20-9.078-20-20.25v-55.5C392 217.1 400.1 208 412 208c11.02 0 20 9.078 20 20.25V283.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPointDown;
impl IconShape for FaHandPointDown {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M448 248V144C448 64.6 385.1 0 307.7 0H199.8C176.4 0 153.1 6.104 132.5 17.65L76.63 49C49.1 64.47 32 94.02 32 126.1V176c0 27.23 12.51 51.53 32 67.69V440C64 479.7 96.3 512 136 512s72-32.3 72-72v-56.44C210.6 383.9 213.3 384 216 384c25.95 0 48.73-13.79 61.4-34.43C283.3 351.2 289.6 352 296 352c25.95 0 48.73-13.79 61.4-34.43C363.3 319.2 369.6 320 376 320C415.7 320 448 287.7 448 248zM272 232c0-13.23 10.78-24 24-24S320 218.9 320 232.1V280c0 13.23-10.78 24-24 24S272 293.2 272 280V232zM192 264h12c12.39 0 23.93-3.264 34.27-8.545C239.3 258.1 240 260.1 240 264v48c0 13.23-10.78 24-24 24S192 325.2 192 312V264zM112 264c0-.2813 .1504-.5137 .1602-.793C114.8 263.4 117.3 264 120 264H160v176c0 13.23-10.78 24-24 24S112 453.2 112 440V264zM397.9 123.8C390.9 121.6 383.7 120 376 120c-29.04 0-53.96 17.37-65.34 42.18C305.8 161.2 301 160 296 160c-7.139 0-13.96 1.273-20.46 3.355C265.2 133.6 237.2 112 204 112H152C138.8 112 128 122.8 128 136S138.8 160 152 160h52c15.44 0 28 12.56 28 28S219.4 216 204 216H120C97.94 216 80 198.1 80 176V126.1c0-14.77 7.719-28.28 20.16-35.27l55.78-31.34C169.4 51.98 184.6 48 199.8 48h107.9C351.9 48 388.9 80.56 397.9 123.8zM400 248c0 13.23-10.78 24-24 24S352 261.2 352 248V192c0-13.23 10.78-24 24-24S400 178.8 400 192V248z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPointLeft;
impl IconShape for FaHandPointLeft {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M264 480h104c79.4 0 144-62.95 144-140.3V231.8c0-23.44-6.104-46.73-17.65-67.35L462.1 108.6C447.5 81.1 417.1 64 385.9 64H336c-27.23 0-51.53 12.51-67.69 32H72C32.3 96 0 128.3 0 168S32.3 240 72 240h56.44C128.1 242.6 128 245.3 128 248c0 25.95 13.79 48.73 34.43 61.4C160.8 315.3 160 321.6 160 328c0 25.95 13.79 48.73 34.43 61.4C192.8 395.3 192 401.6 192 408C192 447.7 224.3 480 264 480zM280 304c13.23 0 24 10.78 24 24S293.1 352 279.9 352H232c-13.23 0-24-10.78-24-24S218.8 304 232 304H280zM248 224v12c0 12.39 3.264 23.93 8.545 34.27C253.9 271.3 251 272 248 272h-48C186.8 272 176 261.2 176 248S186.8 224 200 224H248zM248 144c.2813 0 .5137 .1504 .793 .1602C248.6 146.8 248 149.3 248 152V192h-176C58.77 192 48 181.2 48 168S58.77 144 72 144H248zM388.2 429.9C390.4 422.9 392 415.7 392 408c0-29.04-17.37-53.96-42.18-65.34C350.8 337.8 352 333 352 328c0-7.139-1.273-13.96-3.355-20.46C378.4 297.2 400 269.2 400 236V184C400 170.8 389.3 160 376 160S352 170.8 352 184v52c0 15.44-12.56 28-28 28S296 251.4 296 236V152c0-22.06 17.94-40 40-40h49.88c14.77 0 28.28 7.719 35.27 20.16l31.34 55.78C460 201.4 464 216.6 464 231.8v107.9C464 383.9 431.4 420.9 388.2 429.9zM264 432c-13.23 0-24-10.78-24-24S250.8 384 264 384H320c13.23 0 24 10.78 24 24S333.2 432 320 432H264z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPointRight;
impl IconShape for FaHandPointRight {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 408c0-6.428-.8457-12.66-2.434-18.6C338.2 376.7 352 353.9 352 328c0-6.428-.8457-12.66-2.434-18.6C370.2 296.7 384 273.9 384 248c0-2.705-.1484-5.373-.4414-8H440C479.7 240 512 207.7 512 168S479.7 96 440 96H243.7C227.5 76.51 203.2 64 176 64H126.1C94.02 64 64.47 81.1 49 108.6L17.65 164.5C6.104 185.1 0 208.4 0 231.8v107.9C0 417.1 64.6 480 144 480h104C287.7 480 320 447.7 320 408zM280 304c13.23 0 24 10.78 24 24S293.2 352 280 352H232.1C218.9 352 208 341.2 208 328S218.8 304 232 304H280zM312 224c13.23 0 24 10.78 24 24S325.2 272 312 272h-48c-3.029 0-5.875-.7012-8.545-1.73C260.7 259.9 264 248.4 264 236V224H312zM440 144c13.23 0 24 10.78 24 24S453.2 192 440 192h-176V152c0-2.686-.5566-5.217-.793-7.84C263.5 144.2 263.7 144 264 144H440zM48 339.7V231.8c0-15.25 3.984-30.41 11.52-43.88l31.34-55.78C97.84 119.7 111.4 112 126.1 112H176c22.06 0 40 17.94 40 40v84c0 15.44-12.56 28-28 28S160 251.4 160 236V184C160 170.8 149.3 160 136 160S112 170.8 112 184v52c0 33.23 21.58 61.25 51.36 71.54C161.3 314 160 320.9 160 328c0 5.041 1.166 9.836 2.178 14.66C137.4 354 120 378.1 120 408c0 7.684 1.557 14.94 3.836 21.87C80.56 420.9 48 383.9 48 339.7zM192 432c-13.23 0-24-10.78-24-24S178.8 384 192 384h56c13.23 0 24 10.78 24 24s-10.77 24-24 24H192z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPointUp;
impl IconShape for FaHandPointUp {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M376 192c-6.428 0-12.66 .8457-18.6 2.434C344.7 173.8 321.9 160 296 160c-6.428 0-12.66 .8457-18.6 2.434C264.7 141.8 241.9 128 216 128C213.3 128 210.6 128.1 208 128.4V72C208 32.3 175.7 0 136 0S64 32.3 64 72v196.3C44.51 284.5 32 308.8 32 336v49.88c0 32.1 17.1 61.65 44.63 77.12l55.83 31.35C153.1 505.9 176.4 512 199.8 512h107.9C385.1 512 448 447.4 448 368V264C448 224.3 415.7 192 376 192zM272 232c0-13.23 10.78-24 24-24S320 218.8 320 232v47.91C320 293.1 309.2 304 296 304S272 293.2 272 280V232zM192 200C192 186.8 202.8 176 216 176s24 10.77 24 24v48c0 3.029-.7012 5.875-1.73 8.545C227.9 251.3 216.4 248 204 248H192V200zM112 72c0-13.23 10.78-24 24-24S160 58.77 160 72v176H120c-2.686 0-5.217 .5566-7.84 .793C112.2 248.5 112 248.3 112 248V72zM307.7 464H199.8c-15.25 0-30.41-3.984-43.88-11.52l-55.78-31.34C87.72 414.2 80 400.6 80 385.9V336c0-22.06 17.94-40 40-40h84c15.44 0 28 12.56 28 28S219.4 352 204 352H152C138.8 352 128 362.8 128 376s10.75 24 24 24h52c33.23 0 61.25-21.58 71.54-51.36C282 350.7 288.9 352 296 352c5.041 0 9.836-1.166 14.66-2.178C322 374.6 346.1 392 376 392c7.684 0 14.94-1.557 21.87-3.836C388.9 431.4 351.9 464 307.7 464zM400 320c0 13.23-10.78 24-24 24S352 333.2 352 320V264c0-13.23 10.78-24 24-24s24 10.77 24 24V320z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandPointer;
impl IconShape for FaHandPointer {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M208 288C199.2 288 192 295.2 192 304v96C192 408.8 199.2 416 208 416s16-7.164 16-16v-96C224 295.2 216.8 288 208 288zM272 288C263.2 288 256 295.2 256 304v96c0 8.836 7.162 16 15.1 16S288 408.8 288 400l-.0013-96C287.1 295.2 280.8 288 272 288zM376.9 201.2c-13.74-17.12-34.8-27.45-56.92-27.45h-13.72c-3.713 0-7.412 .291-11.07 .8652C282.7 165.1 267.4 160 251.4 160h-11.44V72c0-39.7-32.31-72-72.01-72c-39.7 0-71.98 32.3-71.98 72v168.5C84.85 235.1 75.19 235.4 69.83 235.4c-44.35 0-69.83 37.23-69.83 69.85c0 14.99 4.821 29.51 13.99 41.69l78.14 104.2C120.7 489.3 166.2 512 213.7 512h109.7c6.309 0 12.83-.957 18.14-2.645c28.59-5.447 53.87-19.41 73.17-40.44C436.1 446.3 448 416.2 448 384.2V274.3C448 234.6 416.3 202.3 376.9 201.2zM400 384.2c0 19.62-7.219 38.06-20.44 52.06c-12.53 13.66-29.03 22.67-49.69 26.56C327.4 463.6 325.3 464 323.4 464H213.7c-32.56 0-63.65-15.55-83.18-41.59L52.36 318.2C49.52 314.4 48.02 309.8 48.02 305.2c0-16.32 14.5-21.75 21.72-21.75c4.454 0 12.01 1.55 17.34 8.703l28.12 37.5c3.093 4.105 7.865 6.419 12.8 6.419c11.94 0 16.01-10.7 16.01-16.01V72c0-13.23 10.78-24 23.1-24c13.22 0 24 10.77 24 24v130.7c0 6.938 5.451 16.01 16.03 16.01C219.5 218.7 220.1 208 237.7 208h13.72c21.5 0 18.56 19.21 34.7 19.21c8.063 0 9.805-5.487 20.15-5.487h13.72c26.96 0 17.37 27.43 40.77 27.43l14.07-.0037c13.88 0 25.16 11.28 25.16 25.14V384.2zM336 288C327.2 288 320 295.2 320 304v96c0 8.836 7.164 16 16 16s16-7.164 16-16v-96C352 295.2 344.8 288 336 288z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandScissors;
impl IconShape for FaHandScissors {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M270.1 480h97.92C447.4 480 512 417.1 512 339.7V231.8c0-23.45-6.106-46.73-17.66-67.33l-31.35-55.85C447.5 81.1 417.1 64 385.9 64h-46.97c-26.63 0-51.56 11.63-68.4 31.93l-15.46 18.71L127.3 68.44C119 65.46 110.5 64.05 102.1 64.05c-30.02 0-58.37 18.06-69.41 47.09C15.06 156.8 46.19 194 76.75 204.9l2.146 .7637L68.79 206.4C30.21 209 0 241.2 0 279.3c0 39.7 33.27 72.09 73.92 72.09c1.745 0 3.501-.0605 5.268-.1833l88.79-6.135v8.141c0 22.11 10.55 43.11 28.05 56.74C197.4 448.8 230.2 480 270.1 480zM269.1 432c-14.34 0-26-11.03-26-24.62c0 0 .0403-14.31 .0403-14.71c0-6.894-4.102-14.2-10.67-16.39c-10.39-3.5-17.38-12.78-17.38-23.06v-13.53c0-16.98 13.7-16.4 13.7-29.89c0-9.083-7.392-15.96-15.96-15.96c-.3646 0-.7311 .0125-1.099 .0377c0 0-138.1 9.505-138.7 9.505c-14.32 0-25.93-11.04-25.93-24.49c0-13.28 10.7-23.74 24.1-24.64l163.2-11.28c2.674-.1882 14.92-2.907 14.92-16.18c0-6.675-4.284-12.58-10.65-14.85L92.84 159.7C85.39 156.1 75.97 149.4 75.97 136.7c0-11.14 9.249-24.66 25.97-24.66c3.043 0 6.141 .5115 9.166 1.59l234.1 85.03c1.801 .6581 3.644 .9701 5.456 .9701c8.96 0 16-7.376 16-15.1c0-6.514-4.068-12.69-10.59-15.04l-64.81-23.47l15.34-18.56C315.2 117.3 326.6 112 338.9 112h46.97c14.77 0 28.28 7.719 35.27 20.16L452.5 188c7.531 13.41 11.52 28.56 11.52 43.81v107.9c0 50.91-43.06 92.31-96 92.31H269.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandSpock;
impl IconShape for FaHandSpock {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M234.9 48.02c10.43 0 20.72 5.834 24.13 19.17l47.33 184.1c2.142 8.456 9.174 12.62 16.21 12.62c7.326 0 14.66-4.505 16.51-13.37l31.72-155.1c2.921-14.09 13.76-20.57 24.67-20.57c13.01 0 26.14 9.19 26.14 25.62c0 2.19-.2333 4.508-.7313 6.951l-28.48 139.2c-.2389 1.156-.3514 2.265-.3514 3.323c0 8.644 7.504 13.9 14.86 13.9c5.869 0 11.65-3.341 13.46-10.98l24.73-104.2c.2347-.9802 4.12-19.76 24.28-19.76c13.21 0 26.64 9.4 26.64 24.79c0 2.168-.2665 4.455-.8378 6.852l-48.06 204.7c-13.59 57.85-65.15 98.74-124.5 98.74l-48.79-.0234c-40.7-.0196-79.86-15.58-109.5-43.51l-75.93-71.55c-5.938-5.584-8.419-11.1-8.419-18.2c0-13.88 12.45-26.69 26.38-26.69c5.756 0 11.76 2.182 17.26 7.376l51.08 48.14c1.682 1.569 3.599 2.249 5.448 2.249c4.192 0 8.04-3.49 8.04-8.001c0-23.76-3.372-47.39-10.12-70.28L142 161.1C141.2 159.1 140.8 156.3 140.8 153.7c0-15.23 13.48-24.82 26.75-24.82c10.11 0 20.1 5.559 23.94 18.42l31.22 105.8c2.231 7.546 8.029 10.8 13.9 10.8c7.752 0 15.64-5.659 15.64-14.57c0-1.339-.1783-2.752-.562-4.23L209.3 80.06C208.7 77.45 208.3 74.97 208.3 72.62C208.3 57.33 221.7 48.02 234.9 48.02zM234.9 0C201.5 0 160.4 25.24 160.4 72.72c0 2.807 .1579 5.632 .4761 8.463C129.9 83.9 92.84 108.9 92.84 153.8c0 7.175 1.038 14.47 3.148 21.68l24.33 81.94C115.8 256.5 111.1 256 106.4 256C65.74 256 32 290.6 32 330.8c0 19.59 8.162 38.58 23.6 53.1l75.89 71.51c38.68 36.45 89.23 56.53 142.3 56.56L322.6 512c82.1 0 152.5-55.83 171.3-135.8l48.06-204.7C543.3 165.7 544 159.7 544 153.9c0-54.55-49.55-72.95-74.59-72.95c-.7689 0-1.534 .0117-2.297 .0352c-10.49-39.43-46.46-54.11-71.62-54.11c-34.1 0-64.45 24.19-71.63 58.83L319.2 108.5l-13.7-53.29C297.1 22.22 268.7 0 234.9 0z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHand;
impl IconShape for FaHand {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M408 80c-3.994 0-7.91 .3262-11.73 .9551c-9.586-28.51-36.57-49.11-68.27-49.11c-6.457 0-12.72 .8555-18.68 2.457C296.6 13.73 273.9 0 248 0C222.1 0 199.3 13.79 186.6 34.44C180.7 32.85 174.5 32 168.1 32C128.4 32 96.01 64.3 96.01 104v121.6C90.77 224.6 85.41 224 80.01 224c-.0026 0 .0026 0 0 0C36.43 224 0 259.2 0 304.1c0 20.29 7.558 39.52 21.46 54.45l81.25 87.24C141.9 487.9 197.4 512 254.9 512h33.08C393.9 512 480 425.9 480 320V152C480 112.3 447.7 80 408 80zM432 320c0 79.41-64.59 144-143.1 144H254.9c-44.41 0-86.83-18.46-117.1-50.96l-79.76-85.63c-6.202-6.659-9.406-15.4-9.406-23.1c0-22.16 18.53-31.4 31.35-31.4c8.56 0 17.1 3.416 23.42 10.18l26.72 28.69C131.8 312.7 133.9 313.4 135.9 313.4c4.106 0 8.064-3.172 8.064-8.016V104c0-13.25 10.75-24 23.1-24c13.25 0 23.1 10.75 23.1 24v152C192 264.8 199.2 272 208 272s15.1-7.163 15.1-15.1L224 72c0-13.25 10.75-24 23.1-24c13.25 0 23.1 10.75 23.1 24v184C272 264.8 279.2 272 288 272s15.99-7.164 15.99-15.1l.0077-152.2c0-13.25 10.75-24 23.1-24c13.25 0 23.1 10.75 23.1 24v152.2C352 264.8 359.2 272 368 272s15.1-7.163 15.1-15.1V152c0-13.25 10.75-24 23.1-24c13.25 0 23.1 10.75 23.1 24V320z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHandshake;
impl IconShape for FaHandshake {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M506.1 127.1c-17.97-20.17-61.46-61.65-122.7-71.1c-22.5-3.354-45.39 3.606-63.41 18.21C302 60.47 279.1 53.42 256.5 56.86C176.8 69.17 126.7 136.2 124.6 139.1c-7.844 10.69-5.531 25.72 5.125 33.57c4.281 3.157 9.281 4.657 14.19 4.657c7.406 0 14.69-3.375 19.38-9.782c.4062-.5626 40.19-53.91 100.5-63.23c7.457-.9611 14.98 .67 21.56 4.483L227.2 168.2C214.8 180.5 207.1 196.1 207.1 214.5c0 17.5 6.812 33.94 19.16 46.29C239.5 273.2 255.9 279.1 273.4 279.1s33.94-6.813 46.31-19.19l11.35-11.35l124.2 100.9c2.312 1.875 2.656 5.251 .5 7.97l-27.69 35.75c-1.844 2.25-5.25 2.594-7.156 1.063l-22.22-18.69l-26.19 27.75c-2.344 2.875-5.344 3.563-6.906 3.719c-1.656 .1562-4.562 .125-6.812-1.719l-32.41-27.66L310.7 392.3l-2.812 2.938c-5.844 7.157-14.09 11.66-23.28 12.6c-9.469 .8126-18.25-1.75-24.5-6.782L170.3 319.8H96V128.3L0 128.3v255.6l64 .0404c11.74 0 21.57-6.706 27.14-16.14h60.64l77.06 69.66C243.7 449.6 261.9 456 280.8 456c2.875 0 5.781-.125 8.656-.4376c13.62-1.406 26.41-6.063 37.47-13.5l.9062 .8126c12.03 9.876 27.28 14.41 42.69 12.78c13.19-1.375 25.28-7.032 33.91-15.35c21.09 8.188 46.09 2.344 61.25-16.47l27.69-35.75c18.47-22.82 14.97-56.48-7.844-75.01l-120.3-97.76l8.381-8.382c9.375-9.376 9.375-24.57 0-33.94c-9.375-9.376-24.56-9.376-33.94 0L285.8 226.8C279.2 233.5 267.7 233.5 261.1 226.8c-3.312-3.282-5.125-7.657-5.125-12.31c0-4.688 1.812-9.064 5.281-12.53l85.91-87.64c7.812-7.845 18.53-11.75 28.94-10.03c59.75 9.22 100.2 62.73 100.6 63.29c3.088 4.155 7.264 6.946 11.84 8.376H544v175.1c0 17.67 14.33 32.05 31.1 32.05L640 384V128.1L506.1 127.1zM48 352c-8.75 0-16-7.245-16-15.99c0-8.876 7.25-15.99 16-15.99S64 327.2 64 336.1C64 344.8 56.75 352 48 352zM592 352c-8.75 0-16-7.245-16-15.99c0-8.876 7.25-15.99 16-15.99s16 7.117 16 15.99C608 344.8 600.8 352 592 352z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADi0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9WDgo4DXBl6a/xv+Gvht4GuAW3k2xHP6KuCj+b/tq4GP4QrEs/0V8NL8//DXwMsAiCu+Gvgo/n/5GuCjBTwYeDr/Pz1EwFcDH8X/T18j4K+Al+Z5/Q7w3sCt/O/2YOC7gdfief21APP8PQS4lf8bHgw8neeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDP318DL83/DX8NvDTPCwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY5+93gPcGbuV/twcD3w28Fs8LAeb5ewhwK/83PBh4Os8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4r/HVwFvzRU/DXwML9xXAW/NFT8NfAzPn3leCDDPn/iv99XAR/Gcvgb4aJ6/rwY+iuf0NcBH87zM80KAef7Ef72LwHGe063AQ3j+LgLHeU63Ag/heZnnhQDz/In/ervAMZ7TM4AH8/ztAsd4Ts8AHszzMs8LAeb5E//1Phv4LJ7T5wCfzfP32cBn8Zw+B/hsnpd5Xggwz5/47/HZwHtzxXcDn80L99nAe3PFdwOfzfNnnhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef5OALv83/Bg4Ok8LwT8NfBSPK/fBt4HuJX/3R4MfBfw2jyvvxHw1cBH8f/T1wh4MPB0/n96iLjiq4GP4v+XrwE+WjzbXwMvxf8PfwO8NIB4Tl8NfBT/t30N8NFcgXheDwY+Gnht4KX4v+FvgN8Gvhq4lWdD/P+G+P8N8f8b4v83/hGm3pAW0Ftc2AAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHardDrive;
impl IconShape for FaHardDrive {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M304 344c-13.25 0-24 10.74-24 24c0 13.25 10.75 24 24 24c13.26 0 24-10.75 24-24C328 354.7 317.3 344 304 344zM448 32h-384c-35.35 0-64 28.65-64 64v320c0 35.35 28.65 64 64 64h384c35.35 0 64-28.65 64-64V96C512 60.65 483.3 32 448 32zM464 416c0 8.822-7.178 16-16 16H64c-8.822 0-16-7.178-16-16v-96c0-8.822 7.178-16 16-16h384C456.8 304 464 311.2 464 320V416zM464 258.3C458.9 256.9 453.6 256 448 256H64C58.44 256 53.14 256.9 48 258.3V96c0-8.822 7.178-16 16-16h384c8.822 0 16 7.178 16 16V258.3zM400 344c-13.25 0-24 10.74-24 24c0 13.25 10.75 24 24 24c13.26 0 24-10.75 24-24C424 354.7 413.3 344 400 344z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHeart;
impl IconShape for FaHeart {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M244 84L255.1 96L267.1 84.02C300.6 51.37 347 36.51 392.6 44.1C461.5 55.58 512 115.2 512 185.1V190.9C512 232.4 494.8 272.1 464.4 300.4L283.7 469.1C276.2 476.1 266.3 480 256 480C245.7 480 235.8 476.1 228.3 469.1L47.59 300.4C17.23 272.1 0 232.4 0 190.9V185.1C0 115.2 50.52 55.58 119.4 44.1C164.1 36.51 211.4 51.37 244 84C243.1 84 244 84.01 244 84L244 84zM255.1 163.9L210.1 117.1C188.4 96.28 157.6 86.4 127.3 91.44C81.55 99.07 48 138.7 48 185.1V190.9C48 219.1 59.71 246.1 80.34 265.3L256 429.3L431.7 265.3C452.3 246.1 464 219.1 464 190.9V185.1C464 138.7 430.4 99.07 384.7 91.44C354.4 86.4 323.6 96.28 301.9 117.1L255.1 163.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHospital;
impl IconShape for FaHospital {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M296 96C296 87.16 303.2 80 312 80H328C336.8 80 344 87.16 344 96V120H368C376.8 120 384 127.2 384 136V152C384 160.8 376.8 168 368 168H344V192C344 200.8 336.8 208 328 208H312C303.2 208 296 200.8 296 192V168H272C263.2 168 256 160.8 256 152V136C256 127.2 263.2 120 272 120H296V96zM408 0C447.8 0 480 32.24 480 72V80H568C607.8 80 640 112.2 640 152V440C640 479.8 607.8 512 568 512H71.98C32.19 512 0 479.8 0 440V152C0 112.2 32.24 80 72 80H160V72C160 32.24 192.2 0 232 0L408 0zM480 128V464H568C581.3 464 592 453.3 592 440V336H536C522.7 336 512 325.3 512 312C512 298.7 522.7 288 536 288H592V240H536C522.7 240 512 229.3 512 216C512 202.7 522.7 192 536 192H592V152C592 138.7 581.3 128 568 128H480zM48 152V192H104C117.3 192 128 202.7 128 216C128 229.3 117.3 240 104 240H48V288H104C117.3 288 128 298.7 128 312C128 325.3 117.3 336 104 336H48V440C48 453.3 58.74 464 71.98 464H160V128H72C58.75 128 48 138.7 48 152V152zM208 464H272V400C272 373.5 293.5 352 320 352C346.5 352 368 373.5 368 400V464H432V72C432 58.75 421.3 48 408 48H232C218.7 48 208 58.75 208 72V464z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAE3UlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjiq4C3Bh7M/w63Aj8NfIyArwY+iv+dPkfAReA4/zvdKmAXOMb/Ts8Q8NnAZ/G/0+eIKz4beG/gQfzv8Azgu4HPFs/LPH/iv5Z5/sSzIZ6Xef7Efy3z/IlnQzwv8/yJ/1rm+RPPhnheu8Axnpf4r3McuMjzegbwYJ4N8bx+G3gtntfrAL/Nf43XBn6L5/U7wGvzbIjn9dvAa/G8Pgb4av5rfDTwVTyv3wFem2dDPK/PBj6L5/UzwFvzX+O3gNfmeX0M8NU8G+J5vTTwVzx/DwFu5T/Xg4Gn8/y9DPDXPBvi+bsVeBDP62uAj+Y/13cD78XzegbwYJ4T4vn7buC9eP5eBvhr/nO8NPBXPH+fA3w2zwnx/D0YeDr/c1wCHgzs8pwQL9hnA5/F/wyfA3w2zwvxgh0HbgWO8d/rEvBgYJfnhXjh3hr4Kf57vQ7w2zx/iH/ZZwOfxX+PjwG+mhcM8aL5buC9+K/1PcB788IhXnTfDbwX/zW+B3hv/mWIf53vBt6L/1zfA7w3LxrEv953A+/Ff47vAd6bFx3i3+a7gffiP9b3AO/Nvw7i3+67gffiP8b3AO/Nvx7i3+e7gffi3+d7gPfm3wbx7/fdwHvxb/M9wHvzb4f4j/HdwHvxr/M9wHvz74P4j/PdwHvxovke4L3590P8x/pu4L144b4HeG/+YyD+Y30X8N68cN8NvA//MRD/cb4LeG9eNN8NvA//foj/GN8FvDf/Ot8NvA//Poh/v+8C3pt/m+8G3od/O8S/z3cB780L9j1c8V68YN8NvA//Noh/u+8C3psX7HuA9+aK7wbeixfsu4H34V8P8W/zXcB784J9D/DePKfvBt6LF+y7gffhXwfxr3Mc+C3gpXnBvgd4b56/7wbeixfsu4H34UWHeNEdB34LeGlesO8B3psX7ruB9+IF+27gfXjRIF40x4HfAl6aF+x7gPfmRfPdwHvxgn038D78yxAvmp8C3poX7HuA9+Zf57uB9+IF+27gfXjhEP+yzwY+ixfse4D35t/mu4H34gX7HOCzecEQL9x7A9/FC/Y9wHvzb3cc+G3gpXjB3gb4aZ4/xAt2HHg6cJzn73uA9+bf7zjw28BL8fztAg8BdnleiBfsq4GP4vn7HeC1+Y9zHPht4KV4/j4H+GyeF+L5ezDwdJ6/vwFeG9jlP9Zx4LeBl+J57QIPAXZ5Tojn77uB9+J/lq8BPprnhHj+ng48mP9ZbgUewnNCPK+XBv6K/5leBvhrng3xvL4a+Cj+Z/oY4Kt5NsTz+m3gtfif6XeA1+bZEM/rt4HX4n+m3wFem2dDPK+LwHH+Z7oVeAjPhnhe5n828WyI52X+ZxPPhnhe5n828WyIK74KeGvgwfzvcCvw08DHCPhq4KP43+lrBFwEjvO/060CdoFj/O/0DAFfDXwU/zt9jrjis4H3Bh7E/w7PAL4b+Ox/BMSip89kWmLZAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaHourglass;
impl IconShape for FaHourglass {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 24C0 10.75 10.75 0 24 0H360C373.3 0 384 10.75 384 24C384 37.25 373.3 48 360 48H352V66.98C352 107.3 335.1 145.1 307.5 174.5L225.9 256L307.5 337.5C335.1 366 352 404.7 352 445V464H360C373.3 464 384 474.7 384 488C384 501.3 373.3 512 360 512H24C10.75 512 0 501.3 0 488C0 474.7 10.75 464 24 464H32V445C32 404.7 48.01 366 76.52 337.5L158.1 256L76.52 174.5C48.01 145.1 32 107.3 32 66.98V48H24C10.75 48 0 37.25 0 24V24zM99.78 384H284.2C281 379.6 277.4 375.4 273.5 371.5L192 289.9L110.5 371.5C106.6 375.4 102.1 379.6 99.78 384H99.78zM284.2 128C296.1 110.4 304 89.03 304 66.98V48H80V66.98C80 89.03 87 110.4 99.78 128H284.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAABACAYAAABcIPRGAAAEwklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonheDwY+Cnht4KX5n+Gvgd8Gvga4lWdDPKevAj6a/9m+GvgYrkA8218BL83/Dn8NvAyAuOKrgY/if5evAT5awIOBp/O/00MEfDXwUfzv9DUC/gp4aZ7X7wDvDdzKC2f+fcQL92Dgu4HX4nn9tQDz/D0EuJV/mfn3Ef+yBwNP53khwDx/4kWzCxzj3+YZwIN50ZjnhQDz/IkXzU8Db8W/zc8Ab82LxjwvBJjnT7xoHgz8NXCMf51LwEsDt/KiMc8LAeb5Ey+6BwNfDbw08CBeuGcAfw18NHArLzrzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sT/LOZ5IcA8f+J/FvO8EGCeP/Fv82Dgo4DXBl6aK/4a+G3ga4Bb+bcxzwsB5vkT/3pfBXw0L9xXAx/Dv555Xggwz5/41/kr4KV50fw18DL865jnhQDz/IkX3VcDH8W/ztcAH82LzjwvBJjnT7xoHgw8nX+bhwC38qIxzwsB5vkTL5qvBj6Kf5uvAT6aF415Xggwz5940fwV8NL82/w18DK8aMzzQoB5/sSLxvz7iBeNeV4IMM+feNGYfx/xojHPCwHm+RMvmr8GXop/m78BXpoXjXleCDDPn3jRfDXwUfzbfA3w0bxozPNCgHn+xIvmwcDT+bd5CHArLxrzvBBgnj/xovtq4KP41/ka4KN50ZnnhQDz/Il/nb8GXooXzd8AL82/jnleCDDPn/jX+2rgo3jhvgb4aP71zPNCgHn+xL/Ng4GPBl4beCmu+Bvgt4GvBm7l38Y8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+RP/s5jnhQDz/In/WczzQoB5/sSL5r2A1wZeGnhpXjR/Dfw18NvA9/CiMc8LAeb5Ey/cWwNfBTyYf59bgY8BfpoXzjwvBJjnT7xgXwV8NP+xvhr4GF4w87wQYJ4/8fx9NfBR/Of4GuCjef7M80KAef7E83pr4Kf4z/U2wE/zvMzzQoB5/sTzejrwYP5z3Qo8hOdlnhcCzPMnntN7A9/Ff423AX6a52SeFwLM8yee03cD78V/je8B3pvnZJ4XAszzJ57TXwEvzX+NvwZehudknhcCzPMnnpP5ryWek3leCDDPn3hOu8Ax/ms8A3gwz8k8LwSY5088p9cGvht4EP+5ngG8N/DbPCfzvBBgnj/xP4t5Xggwz5/4n8U8LwSY50/8z2KeFwLM8yf+ZzHPCwHm+TsB7PI/w4OBp/O8EPDXwEvxvH4beB/gVv57PRj4LuC1eV5/I+CrgY/if6evEfBg4On87/QQccVXAx/F/y5fA3y0eLa/Bl6K/x3+BnhpAPGcvhr4KP5n+xrgo7kC8bweDHw08NrAS/E/w98Avw18NXArz8Y/AuRV0w5hJOVFAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaIdBadge;
impl IconShape for FaIdBadge {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 0H64C28.65 0 0 28.65 0 64v384c0 35.35 28.65 64 64 64h256c35.35 0 64-28.65 64-64V64C384 28.65 355.3 0 320 0zM336 448c0 8.836-7.164 16-16 16H64c-8.836 0-16-7.164-16-16V64c0-8.838 7.164-16 16-16h64V64c0 17.67 14.33 32 32 32h64c17.67 0 32-14.33 32-32V48h64c8.836 0 16 7.162 16 16V448zM192 288c35.35 0 64-28.65 64-64s-28.65-64-64-64C156.7 160 128 188.7 128 224S156.7 288 192 288zM224 320H160c-44.18 0-80 35.82-80 80C80 408.8 87.16 416 96 416h192c8.836 0 16-7.164 16-16C304 355.8 268.2 320 224 320z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAErElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIjn9WDgo4DXBl6a/x/+Gvht4GuAW3k2xHP6KuCj+f/tq4GP4QrEs/0V8NJcBfDXwMsAiCu+GvgornqgrwE+WsCDgadz1fPzEAFfDXwUVz0/XyPgr4CX5qrn568FmKteEASYq14QBJirXhAEmKteEASYq14QBJirXhAEmKteEASYq14QBJirXhAEmOdP/P9inhcCzPMn/n8xzwsB5vkT/7+Y54UA8/yJ/1/M80KAef7Ev8+DgY8CXht4aa74a+C3ga8BbuV/FvO8EGCeP/Fv91XAR/PCfTXwMfzPYZ4XAszzJ/5t/gp4aV40fw28DP8zmOeFAPP8iX+9rwY+in+drwE+mhfdVwFvDTyYf51bge8GPofnzzwvBJjnT/zrPBh4Ov82DwFu5V/21cBH8e/zOcBn87zM80KAef7Ev85XAx/Fv83XAB/Nv+wicJx/n1uBh/C8zPNCgHn+xL/OXwEvzb/NXwMvw79sFzjGv88zgAfzvMzzQoB5/sS/jvn3Ef+yzwY+i3+fzwE+m+dlnhcCzPMn/nXMv4940Xw28N7Ag/jXeQbw3cBn8/yZ54UA8/yJf52/Bl6Kf5u/AV6a/17meSHAPH/iX+ergY/i3+ZrgI/mv5d5Xggwz5/413kw8HT+bR4C3Mp/L/O8EGCeP/Gv99XAR/Gv8zXAR/PfzzwvBJjnT/zb/DXwUrxo/gZ4af51vgp4a+DB/OvcCnw38Dk8f+Z5IcA8f+Lf7quBj+KF+xrgo/nX+Wrgo/j3+Rzgs3le5nkhwDx/4t/nwcBHA68NvBRX/A3w28BXA7fyr3cROM6/z63AQ3he5nkhwDx/4n+eXeAY/z7PAB7M8zLPCwHm+RP/83w28Fn8+3wO8Nk8L/O8EGCeP/E/02cD7w08iH+dZwDfDXw2z595Xggwz5/4/8U8LwSY50/867wX8NrASwMvzYvmr4G/Bn4a+Bn+e5nnhQDz/IkXzVsDXwU8mH+fW4GPAX6a/x7meSHAPH/iX/ZVwEfzH+urgY/hv555Xggwz5944b4a+Cj+c3wN8NH81zLPCwHm+RMv2FsDP8V/rrcBfpr/OuZ5IcA8f+IFezrwYP5z3Qo8hP865nkhwDx/4vl7b+C7+K/xNsBP81/DPC8EmOdPPH/fDbwX/zW+B3hv/muY54UA8/ydAHZ5Xn8FvDT/Nf4aeBn+8z0YeDrPCwF/DbwUz+u3gfcBbuU5mf9a4j/Xg4HvAl6b5/U3Ar4a+Ciuen6+RsCDgadz1fPzEHHFVwMfxVUP9DXAR4tn+2vgpbgK4G+AlwYQz+mrgY/i/7evAT6aKxDP68HARwOvDbwU/z/8DfDbwFcDt/JsiKteGMRVLwziqhcGcdULwz8C4//AFjeHt6oAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaIdCard;
impl IconShape for FaIdCard {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M368 344h96c13.25 0 24-10.75 24-24s-10.75-24-24-24h-96c-13.25 0-24 10.75-24 24S354.8 344 368 344zM208 320c35.35 0 64-28.65 64-64c0-35.35-28.65-64-64-64s-64 28.65-64 64C144 291.3 172.7 320 208 320zM512 32H64C28.65 32 0 60.65 0 96v320c0 35.35 28.65 64 64 64h448c35.35 0 64-28.65 64-64V96C576 60.65 547.3 32 512 32zM528 416c0 8.822-7.178 16-16 16h-192c0-44.18-35.82-80-80-80h-64C131.8 352 96 387.8 96 432H64c-8.822 0-16-7.178-16-16V160h480V416zM368 264h96c13.25 0 24-10.75 24-24s-10.75-24-24-24h-96c-13.25 0-24 10.75-24 24S354.8 264 368 264z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAE90lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9dLARwGvDTyY/xv+Gvht4GuAW3k2xLMdBz4L+Gj+b/tq4GO4AnHFceC3gJfm/4fvBt4HQFzxU8Bb8//L5wCfLeC1gd/i/59d4GUE/DTwVvz/9DECzPP3O8BbA7v87/Zg4LuB1+J5/Y4A8/ydAHb5v+HBwNN5Xr8jwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdPvGg+Cnhv4KW54q+Brwa+h/9ZzPNCgHn+xAt3HPgt4KV5/v4aeB1gl/8ZzPNCgHn+xAv3V8BL88L9NfAy/M9gnhcCzPMnXrCPBr6KF837AN/Nfz/zvBBgnj/xgv0V8NK8aP4aeBn++5nnhQDz/IkXzPzriP9+5nkhwDx/4gUz/zriv595Xggwz594wf4aeCleNH8DvDT//czzQoB5/sQL9tHAV/GieR/gu/nvZ54XAszzJ164vwZeihfub4CX5n8G87wQYJ4/8cIdB34beCmev78BXhvY5X8G87wQYJ4/8aJ5b+CjgZfiir8Bvhr4bv5nMc8LAeb5E/993hv4LK74HOC7+fczzwsB5vkT/z3eGvgpntPLAH/Nv495Xggwz5/4r/dg4K+A4zynW4GXAXb5tzPPCwHm+RP/9f4KeGmev58G3oZ/O/O8EGCeP/Ff66uBj+KF+xjgq/m3Mc8LAeb5E/913hr4KV40LwP8Nf965nkhwDx/4r/GceDpwHFeNH8NvA6wy7+OeV4IMM+f+K/xW8Br86/z3cD78K9jnhcCzPMn/vN9NvBZ/Nu8D/DdvOjM80KAef7Ef67XBn6Lf7td4GWAW3nRmOeFAPP8if88x4GnA8f59/lr4GV40ZjnhQDz/In/PL8FvDb/Mb4G+Gj+ZeZ5IcA8f+Jfdhx4L+ClgZ8Bfpp/2WcDn8V/rLcBfpoXzjwvBJjnT7xwx4HfAl6aZ3sf4Lt5wV4a+Cv+4+0CLwPcygtmnhcCzPMnXrjvAt6b5/UywF/zvI4DfwU8mP8cvw28Di+YeV4IMM+feME+Gvgqnr9d4CHALs/pp4C35j/X5wCfzfNnnhcCzPMnnr/XBn6LF+6vgZfh2T4a+Cr+a7wO8Ns8L/O8EGCeP/G8Hgz8FXCcf9l3A+8DvDTwV/zX2QUeAuzynMzzQoB5/sRzOg78FvDSvOg+Bvgo4MH81/pt4HV4TuZ5IcA8f+I5fRfw3vzv8THAV/Ns5nkhwDx/4tk+Gvgq/vd5GeCvucI8LwSY509c8drAb/G/063AywC7gHleCDDPn4AHA38FHOd/r58G3gYwzwsB5vk7AfwW8NL87/c+wHfxvBBgnr+fBt6a/xt2geM8LwSY/79+R4D5/+t3BPw08Fb8//QxAl4b+C3+/7kEPFhc8dPAW/H/y+cAny2uOA78NvBS/P/wPcB7A4hnOw58NvBR/N/2OcBncwXieb008NHAawMP4v+G3wH+Gvhq4FaeDfH/G+L/N8T/b4j/3/hHPyDVKVKhNAgAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaImage;
impl IconShape for FaImage {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M152 120c-26.51 0-48 21.49-48 48s21.49 48 48 48s48-21.49 48-48S178.5 120 152 120zM447.1 32h-384C28.65 32-.0091 60.65-.0091 96v320c0 35.35 28.65 64 63.1 64h384c35.35 0 64-28.65 64-64V96C511.1 60.65 483.3 32 447.1 32zM463.1 409.3l-136.8-185.9C323.8 218.8 318.1 216 312 216c-6.113 0-11.82 2.768-15.21 7.379l-106.6 144.1l-37.09-46.1c-3.441-4.279-8.934-6.809-14.77-6.809c-5.842 0-11.33 2.529-14.78 6.809l-75.52 93.81c0-.0293 0 .0293 0 0L47.99 96c0-8.822 7.178-16 16-16h384c8.822 0 16 7.178 16 16V409.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaImages;
impl IconShape for FaImages {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 32H160c-35.35 0-64 28.65-64 64v224c0 35.35 28.65 64 64 64H512c35.35 0 64-28.65 64-64V96C576 60.65 547.3 32 512 32zM528 320c0 8.822-7.178 16-16 16h-16l-109.3-160.9C383.7 170.7 378.7 168 373.3 168c-5.352 0-10.35 2.672-13.31 7.125l-62.74 94.11L274.9 238.6C271.9 234.4 267.1 232 262 232c-5.109 0-9.914 2.441-12.93 6.574L176 336H160c-8.822 0-16-7.178-16-16V96c0-8.822 7.178-16 16-16H512c8.822 0 16 7.178 16 16V320zM224 112c-17.67 0-32 14.33-32 32s14.33 32 32 32c17.68 0 32-14.33 32-32S241.7 112 224 112zM456 480H120C53.83 480 0 426.2 0 360v-240C0 106.8 10.75 96 24 96S48 106.8 48 120v240c0 39.7 32.3 72 72 72h336c13.25 0 24 10.75 24 24S469.3 480 456 480z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAEwElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIirXhjEVS8M4qoXBnHVC4N4Xg8GPgp4beCl+f/hr4HfBr4GuJVnQzynrwI+mv/fvhr4GK5APNtfAS/NVQB/DbwMgLjiq4GP4qoH+hrgowU8GHg6Vz0/DxHw1cBHcdXz8zUC/gp4aZ7X7wDvDdzK/20PBr4beC2e118LMM/fQ4Bb+f/hwcDTeV4IMM+f+P/FPC8EmOdP/P9inhcCzPMn/n8xzwsB5vkTL9hnA5/Fc/oc4LN5Tp8NfBbP6XOAz+Y5fTbwWTynzwE+m+f02cBn8Zw+B/hsntNnA5/Fc/oc4LN5wczzQoB5/sQLZp4/8ZzM8yeek3n+xHMyz594Tub5Ey+YeV4IMM+feMHM8yeek3n+xHMyz594Tub5E8/JPH/iBTPPCwHm+RMvmHn+xHMyz594Tub5E8/JPH/iOZnnT7xg5nkhwDx/4gUzz594Tub5E8/JPH/iOZnnTzwn8/yJF8w8LwSY50+8YJ8NfBbP6XOAz+Y5fTbwWTynzwE+m+f02cBn8Zw+B/hsntNnA5/Fc/oc4LN5Tp8NfBbP6XOAz+YFM88LAeb5E/+/mOeFAPP8if9fzPNCgHn+xP8v5nkhwDx/4v8X87wQYJ4/8YJ9NvBZPKfPAT6b5/TZwGfxnD4H+Gye02cDn8Vz+hzgs3lOnw18Fs/pc4DP5jl9NvBZPKfPAT6bF8w8LwSY50+8YOb5E8/JPH/iOZnnTzwn8/yJ52SeP/GCmeeFAPP8iRfMPH/iOZnnTzwn8/yJ52SeP/GczPMnXjDzvBBgnj/xgpnnTzwn8/yJ52SeP/GczPMnnpN5/sQLZp4XAszzJ14w8/yJ52SeP/GczPMnnpN5/sRzMs+feMHM80KAef7EC/bZwGfxnD4H+Gye02cDn8Vz+hzgs3lOnw18Fs/pc4DP5jl9NvBZPKfPAT6b5/TZwGfxnD4H+GxeMPO8EGCeP/H/i3leCDDPn/j/xTwvBJjnT/z/Yp4XAszzJ/5/Mc8LAeb5E/+/mOeFAPP8iX+b1wZ+CjjOf61d4G2A3+bfxjwvBJjnT/zbXASO89/jVuAh/NuY54UA8/yJfxvz30v825jnhQDz/Il/G/PfS/zbmOeFAPP8iX+bXeAY/z2eATyYfxvzvBBgnj/xb/PawHcDD+K/1jOA9wZ+m38b87wQYJ4/8f+LeV4IMM+f+P/FPC8EmOfvBLDL/w8PBp7O80LAXwMvxfP6beB9gFv5v+3BwHcBr83z+hsBXw18FFc9P18j4MHA07nq+XmIuOKrgY/iqgf6GuCjxbP9NfBSXAXwN8BLA4jn9NXAR/H/29cAH80ViOf1YOCjgdcGXor/H/4G+G3gq4FbeTbEVS8M4qoXBnHVC4O46oVBXPXCIK56YRBXvTCIq14Y/hFW1MUeFL2ElQAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaKeyboard;
impl IconShape for FaKeyboard {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 64H64C28.65 64 0 92.65 0 128v256c0 35.35 28.65 64 64 64h448c35.35 0 64-28.65 64-64V128C576 92.65 547.3 64 512 64zM528 384c0 8.822-7.178 16-16 16H64c-8.822 0-16-7.178-16-16V128c0-8.822 7.178-16 16-16h448c8.822 0 16 7.178 16 16V384zM140 152h-24c-6.656 0-12 5.344-12 12v24c0 6.656 5.344 12 12 12h24c6.656 0 12-5.344 12-12v-24C152 157.3 146.7 152 140 152zM196 200h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C184 194.7 189.3 200 196 200zM276 200h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C264 194.7 269.3 200 276 200zM356 200h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C344 194.7 349.3 200 356 200zM460 152h-24c-6.656 0-12 5.344-12 12v24c0 6.656 5.344 12 12 12h24c6.656 0 12-5.344 12-12v-24C472 157.3 466.7 152 460 152zM140 232h-24c-6.656 0-12 5.344-12 12v24c0 6.656 5.344 12 12 12h24c6.656 0 12-5.344 12-12v-24C152 237.3 146.7 232 140 232zM196 280h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C184 274.7 189.3 280 196 280zM276 280h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C264 274.7 269.3 280 276 280zM356 280h24c6.656 0 12-5.344 12-12v-24c0-6.656-5.344-12-12-12h-24c-6.656 0-12 5.344-12 12v24C344 274.7 349.3 280 356 280zM460 232h-24c-6.656 0-12 5.344-12 12v24c0 6.656 5.344 12 12 12h24c6.656 0 12-5.344 12-12v-24C472 237.3 466.7 232 460 232zM400 320h-224C167.1 320 160 327.1 160 336V352c0 8.875 7.125 16 16 16h224c8.875 0 16-7.125 16-16v-16C416 327.1 408.9 320 400 320z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaLemon;
impl IconShape for FaLemon {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M439.9 144.6c15.34-26.38 8.372-62.41-16.96-87.62c-25.21-25.32-61.22-32.26-87.61-16.95c-9.044 5.218-27.15 3.702-48.08 1.968c-50.78-4.327-127.4-10.73-207.6 69.56C-.6501 191.9 5.801 268.5 10.07 319.3c1.749 20.96 3.28 39.07-1.984 48.08c-15.35 26.4-8.357 62.45 16.92 87.57c16.26 16.37 37.05 25.09 56.83 25.09c10.89 0 21.46-2.64 30.83-8.092c9.013-5.249 27.12-3.718 48.08-1.968c50.69 4.233 127.4 10.7 207.6-69.56c80.27-80.28 73.82-156.9 69.56-207.7C436.2 171.8 434.7 153.7 439.9 144.6zM398.4 120.5c-12.87 22.09-10.67 48.41-8.326 76.25c4.155 49.3 8.841 105.2-55.67 169.7c-64.53 64.49-120.5 59.78-169.7 55.68c-27.85-2.328-54.12-4.53-76.26 8.311c-6.139 3.64-19.17 1.031-29.58-9.451c-10.39-10.33-12.95-23.35-9.372-29.49c12.87-22.09 10.67-48.41 8.326-76.25C53.72 265.1 49.04 210.1 113.5 145.5c48.27-48.27 91.71-57.8 131.2-57.8c13.28 0 26.12 1.078 38.52 2.125c27.9 2.359 54.17 4.561 76.26-8.311c6.123-3.577 19.18-1.031 29.49 9.357C399.4 101.2 402 114.4 398.4 120.5zM239.5 124.1c2.156 8.561-3.062 17.25-11.62 19.43C183.6 154.7 122.7 215.6 111.6 259.9C109.7 267.1 103.2 271.1 96.05 271.1c-1.281 0-2.593-.1562-3.905-.4687C83.58 269.3 78.4 260.6 80.52 252.1C94.67 195.8 163.8 126.7 220.1 112.5C228.8 110.4 237.3 115.5 239.5 124.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaLifeRing;
impl IconShape for FaLifeRing {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M464.1 431C474.3 440.4 474.3 455.6 464.1 464.1C455.6 474.3 440.4 474.3 431 464.1L419.3 453.2C374.9 489.9 318.1 512 256 512C193.9 512 137.1 489.9 92.74 453.2L80.97 464.1C71.6 474.3 56.4 474.3 47.03 464.1C37.66 455.6 37.66 440.4 47.03 431L58.8 419.3C22.08 374.9 0 318.1 0 256C0 193.9 22.08 137.1 58.8 92.74L47.03 80.97C37.66 71.6 37.66 56.4 47.03 47.03C56.4 37.66 71.6 37.66 80.97 47.03L92.74 58.8C137.1 22.08 193.9 0 256 0C318.1 0 374.9 22.08 419.3 58.8L431 47.03C440.4 37.66 455.6 37.66 464.1 47.03C474.3 56.4 474.3 71.6 464.1 80.97L453.2 92.74C489.9 137.1 512 193.9 512 256C512 318.1 489.9 374.9 453.2 419.3L464.1 431zM304.8 338.7C290.5 347.2 273.8 352 256 352C238.2 352 221.5 347.2 207.2 338.7L126.9 419.1C162.3 447.2 207.2 464 256 464C304.8 464 349.7 447.2 385.1 419.1L304.8 338.7zM464 256C464 207.2 447.2 162.3 419.1 126.9L338.7 207.2C347.2 221.5 352 238.2 352 256C352 273.8 347.2 290.5 338.7 304.8L419.1 385.1C447.2 349.7 464 304.8 464 256V256zM256 48C207.2 48 162.3 64.8 126.9 92.93L207.2 173.3C221.5 164.8 238.2 160 256 160C273.8 160 290.5 164.8 304.8 173.3L385.1 92.93C349.7 64.8 304.8 48 256 48V48zM173.3 304.8C164.8 290.5 160 273.8 160 256C160 238.2 164.8 221.5 173.3 207.2L92.93 126.9C64.8 162.3 48 207.2 48 256C48 304.8 64.8 349.7 92.93 385.1L173.3 304.8zM256 208C229.5 208 208 229.5 208 256C208 282.5 229.5 304 256 304C282.5 304 304 282.5 304 256C304 229.5 282.5 208 256 208z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaLightbulb;
impl IconShape for FaLightbulb {
    fn view_box(&self) -> &str {
        "0 0 384 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM192 0C90.02 .3203 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.8 289.2 .0039 192 0zM288.4 260.1c-15.66 17.85-35.04 46.3-49.05 75.89h-94.61c-14.01-29.59-33.39-58.04-49.04-75.88C75.24 236.8 64 206.1 64 175.1C64 113.3 112.1 48.25 191.1 48C262.6 48 320 105.4 320 175.1C320 206.1 308.8 236.8 288.4 260.1zM176 80C131.9 80 96 115.9 96 160c0 8.844 7.156 16 16 16S128 168.8 128 160c0-26.47 21.53-48 48-48c8.844 0 16-7.148 16-15.99S184.8 80 176 80z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaMap;
impl IconShape for FaMap {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M565.6 36.24C572.1 40.72 576 48.11 576 56V392C576 401.1 569.8 410.9 560.5 414.4L392.5 478.4C387.4 480.4 381.7 480.5 376.4 478.8L192.5 417.5L32.54 478.4C25.17 481.2 16.88 480.2 10.38 475.8C3.882 471.3 0 463.9 0 456V120C0 110 6.15 101.1 15.46 97.57L183.5 33.57C188.6 31.6 194.3 31.48 199.6 33.23L383.5 94.52L543.5 33.57C550.8 30.76 559.1 31.76 565.6 36.24H565.6zM48 421.2L168 375.5V90.83L48 136.5V421.2zM360 137.3L216 89.3V374.7L360 422.7V137.3zM408 421.2L528 375.5V90.83L408 136.5V421.2z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAADxUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMonherw28F/DawIP5v+Gvgd8Gvga4lWdDPNtx4LuAt+b/tq8GPoYrEFccB34LeGn+f/hu4H0AxBW/Bbw2/798DvDZAt4b+C7+f3qIgN8GXov/nz5GgHn+fgd4b+BW/nd7MPDdwGvxvH5HgHn+HgLcyv8NDwaezvP6awHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ6/E8Au/zc8GHg6z+t3BOwCx3hevw28D3Ar/7s9GPgu4LV5Xj8j4LuB9+I/n3jhjgNfBbw3/3XeR8CDgafzn0+8YC8NfBfw0vzXeQbwYHHFZwOfxX8u8fy9N/BVwHH+a70M8Nfi2b4beC/+84jndBz4KuC9+a91Cfho4LsBxHN6a+CrgQfxH08820sD3wW8NP+1fgf4aOCvuQLx/L018NLAg4EH86/zWjx/4or3Br4KOM4L9gzgVv5j/DWwC/w08Nc8J8R/PPP8nQC+CnhvXrifAd4b2OU/H+I/nnn+/hp4aV64jwG+mv86iP945l/vGcBbA3/Nfy3Efzzzr/MzwHsDu/zXQ/zHMy+6jwG+mv8+iP945l/2DOCtgb/mvxfiP5554X4GeG9gl/9+iP945gX7GOCr+Z8D8R/PPK9nAG8N/DX/syD+45nn9DPAewO7/M+D+I9nnu1jgK/mfy7Ef7y3Bt4a+Grgr/mfjX8ECQKeDJZOjz4AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaMessage;
impl IconShape for FaMessage {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M447.1 0h-384c-35.25 0-64 28.75-64 63.1v287.1c0 35.25 28.75 63.1 64 63.1h96v83.98c0 9.836 11.02 15.55 19.12 9.7l124.9-93.68h144c35.25 0 64-28.75 64-63.1V63.1C511.1 28.75 483.2 0 447.1 0zM464 352c0 8.75-7.25 16-16 16h-160l-80 60v-60H64c-8.75 0-16-7.25-16-16V64c0-8.75 7.25-16 16-16h384c8.75 0 16 7.25 16 16V352z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAFN0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIirXhjEVS8M4qoXBnHVC4N4Xg8GPgp4beCl+f/hr4HfBr4GuJVnQzynrwI+mv/fvhr4GK5APNtfAS/NVQB/DbwMgLjiq4GP4qoH+hrgowU8GHg6Vz0/DxHw1cBHcdXz8zUC/gp4af71xP8u5l/vrwWYf72/AV6a/13+Gngp/nUQYP71vgb4aP53+Wrgo/jXQYD513sIcCv/uzwYeDr/Oggw/zpfA3w0/zFeGngv4KWB48BLc8VfA7vAbwM/A/w1/zG+GvgoXnQIMC+6vwFemn+/9wI+G3gwL5pbgc8Gvod/v78GXooXDQLMi+ZrgI/m3+fBwHcBr82/zW8D7wPcyr/PVwMfxb8MAeYF+xvgt4GvBm7l3+e1gZ8CjvPvswu8DfDb/Ps8GPho4LWBl+L5Q4B5/sR/nNcGfov/WC8D/DX/cczzQoB5/sR/jAcDfwUc5z/WLvAywK38xzDPCwHm+RP/MX4LeG1esN8B3hu4lSseDHw38Fr8y34beB3+Y5jnhQDz/Il/v/cGvosX7iHArTynBwNP50XzPsB38+9nnhcCzPMn/v2eDjyYF+4EsMvzMi+aW4GH8O9nnhcCzPMn/n1eGvgr/mW/DbwPcCvPybzoXgb4a/59zPNCgHn+xL/PVwMfxYtOPCfzovsc4LP59zHPCwHm+RP/Pr8NvBYvOvGczIvud4DX5t/HPC8EmOdP/Pv8FfDSvOjEczIvur8GXoZ/H/O8EGCeP/HvY/51xHMy/zri38c8LwSY50/8+5h/HfGczL+O+PcxzwsB5vkT/z5/DbwULzrxnMyL7m+Al+bfxzwvBJjnT/z7/DbwWrzoxHMyL7rfAV6bfx/zvBBgnj/x7/PVwEfxohPPybzoPgf4bP59zPNCgHn+xL/PSwN/xQv3O8B7A7fy/D0Y+G7gtXjhXgb4a/59zPNCgHn+xL/frcCDeMEeAtzKC/dg4Om8YM8AHsy/n3leCDDPn/j3e2/gu3jBTgC7vHAPBp7OC/Y2wE/z72eeFwLM8yf+Y/w28Fo8f78NvA9wK8/fg4HvAl6b5+93gNfmP4Z5Xggwz5/4j/Fg4K+BY/zHugS8NHAr/zHM80KAef7Ef5zXBn6L/1gvA/w1/3HM80KAecH+Gvht4GuAW/n3eW3gp4Fj/PtcAl4b+Gv+fR4MfBTw2sBL8/whwLxovhr4GP59Hgx8N/Ba/Nv8DvDewK38+3wV8NH8yxBgXnR/DbwM/37vDXw28CBeNM8APhr4af79/gp4aV40CDD/Ol8DfDT/MV4aeGvgtYHjwEtxxd8Au8BvAz8N/DX/Mb4a+ChedAgw/3oPAW7lf5cHA0/nXwcB5l/va4CP5n+XrwY+in8dBJh/vb8GXob/Xf4KeGn+dRDw18BL8a8n/ncx/3p/I+CrgY/iqufnawQ8GHg6Vz0/DxFXfDXwUVz1QF8DfLR4tr8GXoqrAP4GeGkA8Zy+Gvgo/n/7GuCjuQLxvB4MfDTw2sBL8f/D3wC/DXw1cCvPhrjqhUFc9cIgrnphEFe9MIirXhjEVS8M4qoXBnHVC8M/AquK1R4R67GcAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaMoneyBill1;
impl IconShape for FaMoneyBill1 {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M400 256C400 317.9 349.9 368 288 368C226.1 368 176 317.9 176 256C176 194.1 226.1 144 288 144C349.9 144 400 194.1 400 256zM272 224V288H264C255.2 288 248 295.2 248 304C248 312.8 255.2 320 264 320H312C320.8 320 328 312.8 328 304C328 295.2 320.8 288 312 288H304V208C304 199.2 296.8 192 288 192H272C263.2 192 256 199.2 256 208C256 216.8 263.2 224 272 224zM0 128C0 92.65 28.65 64 64 64H512C547.3 64 576 92.65 576 128V384C576 419.3 547.3 448 512 448H64C28.65 448 0 419.3 0 384V128zM48 176V336C83.35 336 112 364.7 112 400H464C464 364.7 492.7 336 528 336V176C492.7 176 464 147.3 464 112H112C112 147.3 83.35 176 48 176z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaMoon;
impl IconShape for FaMoon {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M421.6 379.9c-.6641 0-1.35 .0625-2.049 .1953c-11.24 2.143-22.37 3.17-33.32 3.17c-94.81 0-174.1-77.14-174.1-175.5c0-63.19 33.79-121.3 88.73-152.6c8.467-4.812 6.339-17.66-3.279-19.44c-11.2-2.078-29.53-3.746-40.9-3.746C132.3 31.1 32 132.2 32 256c0 123.6 100.1 224 223.8 224c69.04 0 132.1-31.45 173.8-82.93C435.3 389.1 429.1 379.9 421.6 379.9zM255.8 432C158.9 432 80 353 80 256c0-76.32 48.77-141.4 116.7-165.8C175.2 125 163.2 165.6 163.2 207.8c0 99.44 65.13 183.9 154.9 212.8C298.5 428.1 277.4 432 255.8 432z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaNewspaper;
impl IconShape for FaNewspaper {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M456 32h-304C121.1 32 96 57.13 96 88v320c0 13.22-10.77 24-24 24S48 421.2 48 408V112c0-13.25-10.75-24-24-24S0 98.75 0 112v296C0 447.7 32.3 480 72 480h352c48.53 0 88-39.47 88-88v-304C512 57.13 486.9 32 456 32zM464 392c0 22.06-17.94 40-40 40H139.9C142.5 424.5 144 416.4 144 408v-320c0-4.406 3.594-8 8-8h304c4.406 0 8 3.594 8 8V392zM264 272h-64C186.8 272 176 282.8 176 296S186.8 320 200 320h64C277.3 320 288 309.3 288 296S277.3 272 264 272zM408 272h-64C330.8 272 320 282.8 320 296S330.8 320 344 320h64c13.25 0 24-10.75 24-24S421.3 272 408 272zM264 352h-64c-13.25 0-24 10.75-24 24s10.75 24 24 24h64c13.25 0 24-10.75 24-24S277.3 352 264 352zM408 352h-64C330.8 352 320 362.8 320 376s10.75 24 24 24h64c13.25 0 24-10.75 24-24S421.3 352 408 352zM400 112h-192c-17.67 0-32 14.33-32 32v64c0 17.67 14.33 32 32 32h192c17.67 0 32-14.33 32-32v-64C432 126.3 417.7 112 400 112z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAD0klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DbwOsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if96x4GPAt4aeGn+/RBgnj/xX+u9ga8CjvMfBwHm+RP/dd4b+C7+4yHAPH/iv8Zx4OnAcf7jIcA8f+K/xmcDn8V/DgSY50/81/gr4KX5z4EA8/yJ/xrm+XsZ4K950bw18FM8r78RYJ4/8V/DPH/iRffdwHvxvH5GgHn+xH8N8/yJF81rA7/F8/c+AszzJ/5rmOdP/MteGvgt4DjP30MEmOdP/Ncwz5944V4a+C3gOM/f9wDvLcA8f+K/hnn+xAv20sBvAcd5/i4BDwZ2BZjnT/zXMM+feP5eGvgt4Dgv2NsAPw0gwDx/4r+Gef7E83pp4LeA47xg7wN8N1cgwDx/4r+Gef7Ec3pp4LeA47xg7wN8N8+GAPP8if8a5vkTz/bSwG8Bx3nB3gf4bp4TAszz9xDgVv7zmedPXPHSwG8Bx3nB3gf4bp4XAszz99vA+wC38p/LPH9vDbw18N68cO8DfDfPHwJ+Gngr/nd6H+C7ecEQ8NrAb/G/z/sA380Lh7jiu4H34n+P9wG+m38Z4orjwG8DL8X/bJeA9wZ+mhcN4tmOA58NfBT/M30P8NnArbzoEM/rwcBHA68NvBT/ff4GuBX4beCngVv510P834b4vw3xfxvi/zb+EdXvlKGIW65aAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaNoteSticky;
impl IconShape for FaNoteSticky {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M384 32H64.01C28.66 32 .0085 60.65 .0065 96L0 415.1C-.002 451.3 28.65 480 64 480h232.1c25.46 0 49.88-10.12 67.89-28.12l55.88-55.89C437.9 377.1 448 353.6 448 328.1V96C448 60.8 419.2 32 384 32zM52.69 427.3C50.94 425.6 48 421.8 48 416l.0195-319.1C48.02 87.18 55.2 80 64.02 80H384c8.674 0 16 7.328 16 16v192h-88C281.1 288 256 313.1 256 344v88H64C58.23 432 54.44 429.1 52.69 427.3zM330.1 417.9C322.9 425.1 313.8 429.6 304 431.2V344c0-4.406 3.594-8 8-8h87.23c-1.617 9.812-6.115 18.88-13.29 26.05L330.1 417.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAFiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIgX7qOA9wZemiv+Gvhq4Hv43+mjgPcGXpor/hr4auB7eP4Qz99x4LeAl+b5+2vgdYBd/nc4DvwW8NI8f38NvA6wy3NCPH9/Bbw0/7/8NfAyPCfE8/po4Kv4/+l9gO/m2RDP66+Al+b/p78GXoZnQzwv8/+beDbE8zL/v4lnQzyvvwZeiv+ffgd4bZ4N8bzeG/guXjTvA3w3/7O9N/BdvGjeB/hung3x/P018FK8cH8DvDT/O/w18FK8cH8DvDTPCfH8HQcu8oL9DfDawC7/OxwHfht4KV6wE8Auzwnxgpnn732A7+Z/p/cGvovnTzwvxAtmnj/xvB4MfBXw0sCD+fe5Ffhr4GOAW/mPZ54/8bwQL5h5/sRzejDwV8Bx/mPtAi8D3Mp/LPP8ieeFeMHM8yee008Db8V/jp8B3pr/WOb5E88L8YKZ5088p4vAcf5z3Ao8hP9Y5vkTzwvxgpnnTzwn859L/Mcyz594XogXzDx/4jmZ/1ziP5Z5/sTzQrxg5vkTz8n85xL/sczzJ54X4gUzz594TuY/l/iPZZ4/8bwQL5h5/sRzMv+5xH8s8/yJ54V4wczzJ56T+c8l/mOZ5088L8QLZp4/8ZzMfy7xH8s8f+J5IV4w8/yJ52T+c4n/WOb5E88L8YKZ5088J/OfSzwn88LdCvw18DHArTwv8/yJ54V4wczzJ56T+c8lnpN50ewCLwPcynMyz594XogXzDx/4jmZ/1ziOZkX3c8Ab81zMs+feF6IF8w8f+I5mf9c4jmZF92twEN4Tub5E88L8YKZ5088p13gGP85ngE8mOdk/nXEczLPn3heiBfMPH/iOf008Fb85/gZ4K15tpcG/op/HfGczPMnnhfiBTPPn3hODwb+GjjGf6xLwEsDt/JsHw18Ff864jmZ5088L8QLZp4/8bweDHw18NLAg/j3eQbw18BHA7fynJ4OPJh/HfGczPMnnhfiBTPPn/jv8dHAV/GvJ56Tef7E80K8YOb5E//1Xhr4K/5txHMyz594XogXzDx/4r/WRwFfzb+deE7m+RPPC/GCmedP/Od7aeC1gI8GHsy/j3hO5vkTzwvxgpnnT/zHMv+5xHMyz594XogXzDx/4j+W+c8lnpN5/sTzQrxg5vkT/7HMfy7xnMzzJ54X4gUzz5/4j2X+c4nnZJ4/8bwQL5h5/sR/rF3gGP85ngE8mOdknj/xvBAvmHn+xH+snwbeiv8cPwO8Nc/JPH/ieSFeMPP8if9YDwb+GjjGf6xLwEsDt/KczPMnnhfiBTPPn/iP92Dgq4GXBh7Ev88zgL8GPhq4ledlnj/xvBAvmHn+3hv4Hv53ei/gu3n+xPNCPH/HgYu8YH8NvA6wy/8Ox4HfAl6aF+wEsMtzQjx/fwW8NC/cXwMvw/8OfwW8NC/cXwMvw3NCPK+PBr6KF837AN/N/2zvDXwXL5qPAb6aZ0M8r78CXpr/n/4aeBmeDfG8zP9v4tkQz8v8/yaeDfG8/hp4Kf5/+hvgpXk2xPP6aOCr+P/pfYDv5tkQz99fAy/F/y9/A7w0zwnx/B0Hfht4KZ6/vwFeG9jlf4fjwG8DL8Xz9zfAawO7PCfEC/fewEcDL8UVfwN8NfDd/O/00cB7Ay/FFX8DfDXw3Tx/iKteGMRVLwziqhcGcdULwz8CQffQQRPgmVQAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaObjectGroup;
impl IconShape for FaObjectGroup {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M128 160C128 142.3 142.3 128 160 128H288C305.7 128 320 142.3 320 160V256C320 273.7 305.7 288 288 288H160C142.3 288 128 273.7 128 256V160zM288 320C323.3 320 352 291.3 352 256V224H416C433.7 224 448 238.3 448 256V352C448 369.7 433.7 384 416 384H288C270.3 384 256 369.7 256 352V320H288zM48 115.8C38.18 106.1 32 94.22 32 80C32 53.49 53.49 32 80 32C94.22 32 106.1 38.18 115.8 48H460.2C469 38.18 481.8 32 496 32C522.5 32 544 53.49 544 80C544 94.22 537.8 106.1 528 115.8V396.2C537.8 405 544 417.8 544 432C544 458.5 522.5 480 496 480C481.8 480 469 473.8 460.2 464H115.8C106.1 473.8 94.22 480 80 480C53.49 480 32 458.5 32 432C32 417.8 38.18 405 48 396.2V115.8zM96 125.3V386.7C109.6 391.6 120.4 402.4 125.3 416H450.7C455.6 402.4 466.4 391.6 480 386.7V125.3C466.4 120.4 455.6 109.6 450.7 96H125.3C120.4 109.6 109.6 120.4 96 125.3z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaObjectUngroup;
impl IconShape for FaObjectUngroup {
    fn view_box(&self) -> &str {
        "0 0 640 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M64 0C90.86 0 113.9 16.55 123.3 40H324.7C334.1 16.55 357.1 0 384 0C419.3 0 448 28.65 448 64C448 90.86 431.5 113.9 408 123.3V228.7C431.5 238.1 448 261.1 448 288C448 323.3 419.3 352 384 352C357.1 352 334.1 335.5 324.7 312H123.3C113.9 335.5 90.86 352 64 352C28.65 352 0 323.3 0 288C0 261.1 16.55 238.1 40 228.7V123.3C16.55 113.9 0 90.86 0 64C0 28.65 28.65 0 64 0V0zM64 80C72.84 80 80 72.84 80 64C80 56.1 74.28 49.54 66.75 48.24C65.86 48.08 64.94 48 64 48C55.16 48 48 55.16 48 64C48 64.07 48 64.14 48 64.21C48.01 65.07 48.09 65.92 48.24 66.75C49.54 74.28 56.1 80 64 80zM384 48C383.1 48 382.1 48.08 381.2 48.24C373.7 49.54 368 56.1 368 64C368 72.84 375.2 80 384 80C391.9 80 398.5 74.28 399.8 66.75C399.9 65.86 400 64.94 400 64C400 55.16 392.8 48 384 48V48zM324.7 88H123.3C116.9 104 104 116.9 88 123.3V228.7C104 235.1 116.9 247.1 123.3 264H324.7C331.1 247.1 343.1 235.1 360 228.7V123.3C343.1 116.9 331.1 104 324.7 88zM400 288C400 287.1 399.9 286.1 399.8 285.2C398.5 277.7 391.9 272 384 272C375.2 272 368 279.2 368 288C368 295.9 373.7 302.5 381.2 303.8C382.1 303.9 383.1 304 384 304C392.8 304 400 296.8 400 288zM64 272C56.1 272 49.54 277.7 48.24 285.2C48.08 286.1 48 287.1 48 288C48 296.8 55.16 304 64 304L64.22 303.1C65.08 303.1 65.93 303.9 66.75 303.8C74.28 302.5 80 295.9 80 288C80 279.2 72.84 272 64 272zM471.3 248C465.8 235.9 457.8 225.2 448 216.4V200H516.7C526.1 176.5 549.1 160 576 160C611.3 160 640 188.7 640 224C640 250.9 623.5 273.9 600 283.3V388.7C623.5 398.1 640 421.1 640 448C640 483.3 611.3 512 576 512C549.1 512 526.1 495.5 516.7 472H315.3C305.9 495.5 282.9 512 256 512C220.7 512 192 483.3 192 448C192 421.1 208.5 398.1 232 388.7V352H280V388.7C296 395.1 308.9 407.1 315.3 424H516.7C523.1 407.1 535.1 395.1 552 388.7V283.3C535.1 276.9 523.1 264 516.7 248H471.3zM592 224C592 215.2 584.8 208 576 208C575.1 208 574.1 208.1 573.2 208.2C565.7 209.5 560 216.1 560 224C560 232.8 567.2 240 576 240C583.9 240 590.5 234.3 591.8 226.8C591.9 225.9 592 224.9 592 224zM240 448C240 456.8 247.2 464 256 464C256.9 464 257.9 463.9 258.8 463.8C266.3 462.5 272 455.9 272 448C272 439.2 264.8 432 256 432C248.1 432 241.5 437.7 240.2 445.2C240.1 446.1 240 447.1 240 448zM573.2 463.8C574.1 463.9 575.1 464 576 464C584.8 464 592 456.8 592 448C592 447.1 591.9 446.2 591.8 445.3L591.8 445.2C590.5 437.7 583.9 432 576 432C567.2 432 560 439.2 560 448C560 455.9 565.7 462.5 573.2 463.8V463.8z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaPaperPlane;
impl IconShape for FaPaperPlane {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M501.6 4.186c-7.594-5.156-17.41-5.594-25.44-1.063L12.12 267.1C4.184 271.7-.5037 280.3 .0431 289.4c.5469 9.125 6.234 17.16 14.66 20.69l153.3 64.38v113.5c0 8.781 4.797 16.84 12.5 21.06C184.1 511 188 512 191.1 512c4.516 0 9.038-1.281 12.99-3.812l111.2-71.46l98.56 41.4c2.984 1.25 6.141 1.875 9.297 1.875c4.078 0 8.141-1.031 11.78-3.094c6.453-3.625 10.88-10.06 11.95-17.38l64-432C513.1 18.44 509.1 9.373 501.6 4.186zM369.3 119.2l-187.1 208.9L78.23 284.7L369.3 119.2zM215.1 444v-49.36l46.45 19.51L215.1 444zM404.8 421.9l-176.6-74.19l224.6-249.5L404.8 421.9z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFKUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj3Ow58FPDWwEtzxV8DPw18DbDL/1yIf5+3Br4LOM7ztwu8D/DT/M+E+Ld7a+CneNG8DfDT/M+D+Lc5DjwdOM6LZhd4CLDL/yyIf5vPBj6Lf53PAT6b/1kQ/zZ/Bbw0z+l3gPfmiu8GXovn9DvAa/M/C+J5PRj4KOC1gZfmRfcQ4FaueDDwdF50twI/DXwNcCv/dRDP6auAj+bf5gSwyxUPBp7Ov81XAx/Dfw3Es/0V8NL82/028D5c8V3Aa/Nv99vA6/CfD3HFVwMfxf8sXwN8NP+5EPBg4On8z/QQ4Fb+8yDgq4GP4n+mrwE+mv88CPgr4KV5Xr8DvDdwK1eYf79nAD8N/DTw18Au/70QYJ6/hwC38mzm3+4S8NHAd/M/CwLM8yee0y5wjH+9nwHeG9jlfx4EmOdPPKefBt6Kf53vAd6b/7kQYJ4/8ZweDPw1cIwXzc8Ab83zejDwUcBrAy/Nf47fBj4H+G1eOASY5088rwcDXw28NPAgXrBLwIOBXZ7tOPBZwEfzX+ergY/hBUOAef7Ef5zjwG8BL81/ve8G3ofnDwHm+RP/cb4a+Cj++3w38D48LwSY50/8x3ht4Lf47/fdwPvwnBBgnj/xH+O7gffif4bvBt6HZ0OAef7Ef4ynAw/mef0O8N7Arfz7mH+d7wbehysQYJ4/8R/DPH8PAW7l38/863038D4AAszzJ/5jmOdP/Mcw/zZfA3y0APP8if8Y5vkT/zHMv93bCDDPn/iPYZ4/8R/D/NvdKsA8f+I/hnn+xH8M82+HAPP8if8Y5vkT/zF2gWP82yDAPH/iP4Z5/sR/jJ8G3op/GwSY50/8xzDPn/iP8WDgr4Fj/OshwDx/4j+Gef7Ef5wHA18NvDTwIF50CDDPn/iPYZ4/8V/LPC8EmOdP/Mcwz5/4r2WeFwLM8yf+Y5jnT/zXMs8LAeb5E/8xzPMn/muZ54UA8/yJ/xjm+RP/tczzQoB5/sR/DPP8if9a5nkhwDx/4j+Gef7Efy3zvBBgnj/xH8M8f+K/lnleCDDPn/iPYZ4/8V/LPC8EmOdP/Mcwz5/4r2WeFwLM8yf+Y5jnT/zXMs8LAeb5E/8xzPMn/muZ54UA8/yJ/xjm+RP/tczzQoB5/sR/DPP8if9a5nkhwDx/4j+Gef7Efy3zvBBgnj/xH8M8f+K/lnleCDDP3wlgl38/8/yJ/1rmeSHgr4GX4nn9NvA+wK38+5jnT/zXMs8LAV8NfBT/ccRzMs+f+K9lnhcCHgw8nf844jmZ50/81zLPC3HFVwMfxX8M8ZzM8yf+a5nnhXi2vwZein8/8ZzM8yf+a5nnhXhOXw18FP8+4jmZ50/81zLPC/G8Hgx8NPDawEvxryeek3n+xH8t87wQ//nM83cC2OW/xoOBp/O8EP/5/hp4KZ7XbwPvA9zKf64HA98FvDbP62/Ef76vBj6K/5m+RvznezDwdP5neoj4r/HVwEfxP8vXAB8t/uv8NPBW/M/wM8BbA4j/Wl8NfBT/vb4G+GiuQPzXezDw0cBrAy/Ff42/AX4b+GrgVp6NfwT0C9bSdZrpMgAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaPaste;
impl IconShape for FaPaste {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M502.6 198.6l-61.25-61.25C435.4 131.4 427.3 128 418.8 128H256C220.7 128 191.1 156.7 192 192l.0065 255.1C192 483.3 220.7 512 256 512h192c35.2 0 64-28.8 64-64l.0098-226.7C512 212.8 508.6 204.6 502.6 198.6zM464 448c0 8.836-7.164 16-16 16h-192c-8.838 0-16-7.164-16-16L240 192.1c0-8.836 7.164-16 16-16h128L384 224c0 17.67 14.33 32 32 32h48.01V448zM317.7 96C310.6 68.45 285.8 48 256 48H215.2C211.3 20.93 188.1 0 160 0C131.9 0 108.7 20.93 104.8 48H64c-35.35 0-64 28.65-64 64V384c0 35.34 28.65 64 64 64h96v-48H64c-8.836 0-16-7.164-16-16V112C48 103.2 55.18 96 64 96h16v16c0 17.67 14.33 32 32 32h61.35C190 115.4 220.6 96 256 96H317.7zM160 72c-8.822 0-16-7.176-16-16s7.178-16 16-16s16 7.176 16 16S168.8 72 160 72z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAHiklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/48HAawEP5opbgd8BbuUFQ/zv99rAZwGvzfP328DnAL/N80L87/ZdwHvzovlu4H14Toj/vb4LeG/+db4beB+eDfG/03cB782/zXcD78MViP99vgt4b/59vht4HwDxv8t3Ae/NC/YM4Ke54r2BY7xgrwP8tvjf47uA9+YF+x7go4FdrjgO/DbwUjx/vw28jnjBjgNvBbw18GDgpfm3uxX4aeBj+Lf5LuC9ecG+B3hvntdx4KeB1+L5e4h4XseBjwI+GjjOf6zPAT6bf53vAt6bF+x7gPfmBTsO/DXwIJ7X24jndBz4LeCl+c9xK/AQXnTfBbw3L9j3AO/Nv+yrgY/ieX2OeLbjwNOB4/zneQbwYP5lx4HvAt6aF+67gffhX/bVwEfxvD5HXHEc+C3gpfnP9TnAZ/PCHQd+C3hpXjTfDbwPL9hx4OnAcZ7X24grPhv4LP7zPAP4buCzeeGOA78FvDT/Ot8NvA/P6zjwW8BL8/w9RMBx4OnAcZ6/nwG+Gvht/nMdB34LeGn+bb4beB+e7TjwW8BL8/z9DvDaAt4b+C6ev/cBvpv/fMeB3wJemhfse7jivXjBvht4H+A48FvAS/OCvQ7w2wJ+GngrntfPAG/Nf77jwG8BL80L9j3Ae3PFdwPvxQv23cCDgdfmBfse4L0BBPwV8NI8r9cBfpv/XMeB3wJemhfse4D35jl9N/Be/Nt8D/DeXIEA8/yJ/1zHgd8CXpoX7HuA9+b5+27gvfjX+R7gvXk2BJjnT/znOQ78FvDSvGDfA7w3L9x3A+/Fi+Z7gPfmOSHAPH/iP8dx4LeAl+YF+x7gvXnRfDfwXrxw3wO8N88LAeb5E//xjgO/Bbw0L9j3AO/Ni+67gPfmBfse4L15/hBgnj/xH+s48FvAS/OCfQ/w3rzovgt4b16w7wHemxcMAeb5E/9xjgO/Bbw0L9j3AO/Ni+67gPfmBfse4L154RBgnj/xH+M48FvAS/OCfQ/w3rzovgt4b16w7wHem38ZAszzJ/79jgO/Bbw0L9j3AO/Ni+67gPfmBfse4L150SDAPH/i3+c48FvAS/OCfQ/w3rzovgt4b16w7wHemxcdAszzJ/59/gp4aV6w7wHemxfddwHvzQv2PcB786+DAPP8iX+77wLemxfse4D35kX3XcB784J9D/De/OshwDx/4t/mrYGf4gX7HuC9edEcB74LeGtesO8B3pt/GwSY50/82/wV8NI8f98DvDcvmuPAbwEvzQv2PcB782+HAPP8iX+9lwb+iufve4D35kVzHPgt4KV5wb4HeG/+fRBgnj/xr/fVwEfxvJ4BvDSwy7/sOPBbwEvzgn0P8N78+yHAPH/iX++vgJfmeb0P8N38y44DvwW8NC/Y9wDvzX8MBJjnT/zrPBh4Os/fQ4BbeeGOA78FvDQv2PcA781/HASY50/867w38F08r78BXpoX7jjwW8BL84J9D/De/MdCgHn+xL/OTwNvxfP6GuCjecGOA78FvDQv2PcA780Ldxz4LOCtueKngY/hhUOAef7Ev85F4DjP63WA3+b5Ow78FvDSvGDfA7w3/7KvBj6K5/Q1wEfzgiHAPH/iRffawG/xvC4Bx3leDwbeCvho4MG8YN8DvDcvmovAcZ7TrcBDeMEQYJ4/8aL7auCjeF4/A7w1V7wW8NbAWwMP5l/2PcB786Izz594wRBgnj/xons68GCe109zxWsDx3nRfQ/w3vzrmOdPvGAIMM+feNE8GHg6/3G+B3hv/vXM8ydeMASY50+8aD4a+Cr+Y3wN8NH825jnT7xgCDDPn3jR/DTwVvz7/A3w2cBP829nnj/xgiHAPH/iRWP+bf4G+G3gu4G/5t/PPH/iBUOAef7Ei2YXOMa/7BLw28BPA78N3Mp/LPP8iRcMAeb5Ey+azwY+i+fvb4DfBn4a+G3+c5nnT7xgCDDPn3jRfTbw3sBx4LeBnwZ+G7iV/zrm+RMvGALM8yf+dzHPn3jBEGCeP/G/i3n+xAuGAPP8if9dzPMnXjAEmOdP/O9inj/xgiHAPH/ifxfz/IkXDAHm+RP/u5jnT7xgCPhr4KV4Xq8D/Db/O7w28Fs8r78BXpoXDAE/DbwVz+ungbfhf4ffAl6b5/UzwFvzgiHgvYHv4vl7H+C7+Z/ts4DP5vl7H+C7ecEQcBy4FTjG8/fbwGcDv8P/LK8FfDbw2jx/l4AHA7u8YIgrPhv4LP5v+Rzgs3nhEFccB34beCn+b/gb4LWBXV44xLMdB24FjvG/2yXgwcAu/zLEczoO/DbwUvzv9DfAawO7vGgQz+s48NHARwPH+N/hEvDVwFcDu7zoEC/YceCtgbcGHgy8FP+z/A1wK/DTwE8Du/zr8Y+7qicw35S04gAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaPenToSquare;
impl IconShape for FaPenToSquare {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M373.1 24.97C401.2-3.147 446.8-3.147 474.9 24.97L487 37.09C515.1 65.21 515.1 110.8 487 138.9L289.8 336.2C281.1 344.8 270.4 351.1 258.6 354.5L158.6 383.1C150.2 385.5 141.2 383.1 135 376.1C128.9 370.8 126.5 361.8 128.9 353.4L157.5 253.4C160.9 241.6 167.2 230.9 175.8 222.2L373.1 24.97zM440.1 58.91C431.6 49.54 416.4 49.54 407 58.91L377.9 88L424 134.1L453.1 104.1C462.5 95.6 462.5 80.4 453.1 71.03L440.1 58.91zM203.7 266.6L186.9 325.1L245.4 308.3C249.4 307.2 252.9 305.1 255.8 302.2L390.1 168L344 121.9L209.8 256.2C206.9 259.1 204.8 262.6 203.7 266.6zM200 64C213.3 64 224 74.75 224 88C224 101.3 213.3 112 200 112H88C65.91 112 48 129.9 48 152V424C48 446.1 65.91 464 88 464H360C382.1 464 400 446.1 400 424V312C400 298.7 410.7 288 424 288C437.3 288 448 298.7 448 312V424C448 472.6 408.6 512 360 512H88C39.4 512 0 472.6 0 424V152C0 103.4 39.4 64 88 64H200z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAFLElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMorjqhUFc9cIgrnphEFe9MIjn9WDgo4DXBl6a/x/+Gvht4GuAW3k2xHP6KuCj+f/tq4GP4QrEs/0V8NJcBfDXwMsAiCu+GvgornqgrwE+WsCDgadz1fPzEAFfDXwUVz0/XyPgr4CX5nn9DvDewK383/Zg4LuB1+J5/bUA8/w9BLiV/x8eDDyd54UA8/yJ/1/M80KAef7E/y/meSHAPH/i/xfzvBBgnj/x/4t5Xggwz594wR4MfBXw0lzx18DHALfyv5d5Xggwz594/h4M/BVwnOe0C7wMcCvP6zjwWcBbAw/mv9atwE8DH8MLZ54XAszzJ56/nwbeiufvZ4C35nl9NfBR/Pf6GuCjecHM80KAef7E83cROM7zdyvwEJ7XReA4/71uBR7CC2aeFwLM8yeev13gGM/fM4AH87x2gWP893oG8GBeMPO8EGCeP/H8/TTwVjx/PwO8Nc/rq4GP4r/X5wCfzQtmnhcCzPMnnr8HA38NHOM5XQJeGriV5++rgbcGHsR/rWcA3w18Ni+ceV4IMM+feMEeDHw18NJc8dfARwO38r+XeV4IMM+f+P/FPC8EmOdP/P9inhcCzPMn/n8xzwsB5vkT/7+Y54UA8/yJF+zBwFcBL80Vfw18DHAr/3uZ54UA8/yJ5+/BwF8Bx3lOu8DLALfyvI4DnwW8NfBg/mvdCvw08DG8cOZ5IcA8f+L5+2ngrXj+fgZ4a57XVwMfxX+vrwE+mhfMPC8EmOdPPH8XgeM8f7cCD+F5XQSO89/rVuAhvGDmeSHAPH/i+dsFjvH8PQN4MM9rFzjGf69nAA/mBTPPCwHm+RPP308Db8Xz9zPAW/O8vhr4KP57fQ7w2bxg5nkhwDx/4vl7MPDXwDGe0yXgpYFbef6+Gnhr4EH813oG8N3AZ/PCmeeFAPP8iRfswcBXAy/NFX8NfDRwK/97meeFAPP8if9fzPNCgHn+xP8v5nkhwDx/4v8X87wQYJ4/8f+LeV4IMM+feMEeDHwV8NJc8dfAxwC38r+XeV4IMM+feP4eDPwVcJzntAu8DHArz+s48FnAWwMP5r/WrcBPAx/DC2eeFwLM8yeev58G3orn72eAt+Z5fTXwUfz3+hrgo3nBzPNCgHn+xPN3ETjO83cr8BCe10XgOP+9bgUewgtmnhcCzPMnnr9d4BjP3zOAB/O8doFj/Pd6BvBgXjDzvBBgnj/x/P008FY8fz8DvDXP66uBj+K/1+cAn80LZp4XAszzJ56/BwN/DRzjOV0CXhq4lefvq4G3Bh7Ef61nAN8NfDYvnHleCDDPn3jBHgx8NfDSXPHXwEcDt/K/l3leCDDPn/j/xTwvBJjnT/z/Yp4XAszzJ/5/Mc8LAeb5E/+/mOeFAPP8nQB2+f/hwcDTeV4I+GvgpXhevw28D3Ar/7c9GPgu4LV5Xn8j4KuBj+Kq5+drBDwYeDpXPT8PEVd8NfBRXPVAXwN8tHi2vwZeiqsA/gZ4aQDxnL4a+Cj+f/sa4KO5AvG8Hgx8NPDawEvx/8PfAL8NfDVwK8+GuOqFQVz1wiCuemEQV70w/COqrd8WZhaz5QAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaRectangleList;
impl IconShape for FaRectangleList {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M128 192C110.3 192 96 177.7 96 160C96 142.3 110.3 128 128 128C145.7 128 160 142.3 160 160C160 177.7 145.7 192 128 192zM200 160C200 146.7 210.7 136 224 136H448C461.3 136 472 146.7 472 160C472 173.3 461.3 184 448 184H224C210.7 184 200 173.3 200 160zM200 256C200 242.7 210.7 232 224 232H448C461.3 232 472 242.7 472 256C472 269.3 461.3 280 448 280H224C210.7 280 200 269.3 200 256zM200 352C200 338.7 210.7 328 224 328H448C461.3 328 472 338.7 472 352C472 365.3 461.3 376 448 376H224C210.7 376 200 365.3 200 352zM128 224C145.7 224 160 238.3 160 256C160 273.7 145.7 288 128 288C110.3 288 96 273.7 96 256C96 238.3 110.3 224 128 224zM128 384C110.3 384 96 369.7 96 352C96 334.3 110.3 320 128 320C145.7 320 160 334.3 160 352C160 369.7 145.7 384 128 384zM0 96C0 60.65 28.65 32 64 32H512C547.3 32 576 60.65 576 96V416C576 451.3 547.3 480 512 480H64C28.65 480 0 451.3 0 416V96zM48 96V416C48 424.8 55.16 432 64 432H512C520.8 432 528 424.8 528 416V96C528 87.16 520.8 80 512 80H64C55.16 80 48 87.16 48 96z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEvElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9WDgo4DXBl6a/xv+Gvht4GuAW3k2xHP6KuCj+b/tq4GP4QrEs/0V8NL8//DXwMsAiCu+Gvgo/n/5GuCjBTwYeDr/Pz1EwFcDH8X/T18j4K+Al+Z5/Q7w3sCt/O/2YOC7gdfief21APP8PQS4lf8bHgw8neeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7Ev91x4KOAjwZ2ge8GPocX3XHgo4CPBnaB7wY+h38f87wQYJ4/8W/32cBn8Zy+G3gf/mXHgd8CXprn9DHAV/NvZ54XAszzJ/7tLgLHeV7fDbwPL9hx4LeAl+Z53Qo8hH8787wQYJ4/8W93K/Agnr/vBt6H53Uc+C3gpXn+ngE8mH8787wQYJ4/8W/32cBn8YJ9N/A+PNtx4LeAl+YFex/gu/m3M88LAeb5E/8+3w28Fy/YdwPvAxwHfgt4aV6w7wHem38f87wQYJ4/8e/33cB78YJ9N/DSwEvzgn0P8N78+5nnhQDz/In/GN8NvBf/Nt8DvDf/MczzQoB5/sR/nO8G3ot/ne8B3pv/OOZ5IcA8f+I/1ncD78WL5nuA9+Y/lnleCDDPn/iP993Ae/HCfQ/w3vzHM88LAeb5E/+xjgO/Bbw0L9xfA68D7PIfyzwvBJjnT/zHOQ78FvDSvGj+GngdYJf/OOZ5IcA8f+I/xnHgt4CX5l/nt4HX4T+OeV4IMM+f+Pc7DvwW8NL823w38D78xzDPCwHm+RP/PseB3wJemhfse4CXBl6KF+y7gffh3888LwSY50/82x0Hfgt4aV6w7wHeGzgO/DbwUrxg3w28D/8+5nkhwDx/4t/us4HP4gX7HuC9ebbjwG8DL8UL9jHAV/NvZ54XAszzJ/7tLgLHef6+B3hvnr/vBt6L5+9W4CH825nnhQDz/Il/u1uBB/G8vgd4b1647wbei+f1DODB/NuZ54UA8/yJf7vPBj6L5/Q9wHvzovlu4L14Tu8DfDf/duZ5IcA8f+Lf56OBj+aKzwa+m3+djwY+mis+G/hu/n3M80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8fyeAXf5veDDwdJ4XAv4aeCme128D7wPcyv9uDwa+C3htntffCPhq4KP4/+lrBDwYeDr/Pz1EXPHVwEfx/8vXAB8tnu2vgZfi/4e/AV4aQDynrwY+iv/bvgb4aK5APK8HAx8NvDbwUvzf8DfAbwNfDdzKsyH+f0P8/4b4/w3x/xv/CH9fwxZDp0X+AAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaRectangleXmark;
impl IconShape for FaRectangleXmark {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M175 175C184.4 165.7 199.6 165.7 208.1 175L255.1 222.1L303 175C312.4 165.7 327.6 165.7 336.1 175C346.3 184.4 346.3 199.6 336.1 208.1L289.9 255.1L336.1 303C346.3 312.4 346.3 327.6 336.1 336.1C327.6 346.3 312.4 346.3 303 336.1L255.1 289.9L208.1 336.1C199.6 346.3 184.4 346.3 175 336.1C165.7 327.6 165.7 312.4 175 303L222.1 255.1L175 208.1C165.7 199.6 165.7 184.4 175 175V175zM0 96C0 60.65 28.65 32 64 32H448C483.3 32 512 60.65 512 96V416C512 451.3 483.3 480 448 480H64C28.65 480 0 451.3 0 416V96zM48 96V416C48 424.8 55.16 432 64 432H448C456.8 432 464 424.8 464 416V96C464 87.16 456.8 80 448 80H64C55.16 80 48 87.16 48 96z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaRegistered;
impl IconShape for FaRegistered {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464zM352 208c0-44.13-35.88-80-80-80L184 128c-13.25 0-24 10.75-24 24v208c0 13.25 10.75 24 24 24s24-10.75 24-24v-72h59.79l38.46 82.19C310.3 378.9 319 384 328 384c3.438 0 6.875-.7187 10.19-2.25c12-5.625 17.16-19.91 11.56-31.94l-34.87-74.5C337.1 261.1 352 236.3 352 208zM272 240h-64v-64h64c17.66 0 32 14.34 32 32S289.7 240 272 240z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaShareFromSquare;
impl IconShape for FaShareFromSquare {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M568.5 142.6l-144-135.1c-9.625-9.156-24.81-8.656-33.91 .9687c-9.125 9.625-8.688 24.81 .9687 33.91l100.1 94.56h-163.4C287.5 134.2 249.7 151 221 179.4C192 208.2 176 246.7 176 288v87.1c0 13.25 10.75 23.1 24 23.1S224 389.3 224 376V288c0-28.37 10.94-54.84 30.78-74.5C274.3 194.2 298.9 183 328 184h163.6l-100.1 94.56c-9.656 9.094-10.09 24.28-.9687 33.91c4.719 4.1 11.06 7.531 17.44 7.531c5.906 0 11.84-2.156 16.47-6.562l144-135.1C573.3 172.9 576 166.6 576 160S573.3 147.1 568.5 142.6zM360 384c-13.25 0-24 10.75-24 23.1v47.1c0 4.406-3.594 7.1-8 7.1h-272c-4.406 0-8-3.594-8-7.1V184c0-4.406 3.594-7.1 8-7.1H112c13.25 0 24-10.75 24-23.1s-10.75-23.1-24-23.1H56c-30.88 0-56 25.12-56 55.1v271.1C0 486.9 25.13 512 56 512h272c30.88 0 56-25.12 56-55.1v-47.1C384 394.8 373.3 384 360 384z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSnowflake;
impl IconShape for FaSnowflake {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M484.4 294.4c1.715 6.402 .6758 12.89-2.395 18.21s-8.172 9.463-14.57 11.18l-31.46 8.43l32.96 19.03C480.4 357.8 484.4 372.5 477.8 384s-21.38 15.41-32.86 8.783l-32.96-19.03l8.43 31.46c3.432 12.81-4.162 25.96-16.97 29.39s-25.96-4.162-29.39-16.97l-20.85-77.82L280 297.6v84.49l56.97 56.97c9.375 9.375 9.375 24.56 0 33.94C332.3 477.7 326.1 480 320 480s-12.28-2.344-16.97-7.031L280 449.9V488c0 13.25-10.75 24-24 24s-24-10.75-24-24v-38.06l-23.03 23.03c-9.375 9.375-24.56 9.375-33.94 0s-9.375-24.56 0-33.94L232 382.1V297.6l-73.17 42.25l-20.85 77.82c-3.432 12.81-16.58 20.4-29.39 16.97s-20.4-16.58-16.97-29.39l8.43-31.46l-32.96 19.03C55.61 399.4 40.85 395.5 34.22 384s-2.615-26.16 8.859-32.79l32.96-19.03l-31.46-8.43c-12.81-3.432-20.4-16.58-16.97-29.39s16.58-20.4 29.39-16.97l77.82 20.85L208 255.1L134.8 213.8L57.01 234.6C44.2 238 31.05 230.4 27.62 217.6s4.162-25.96 16.97-29.39l31.46-8.432L43.08 160.8C31.61 154.2 27.6 139.5 34.22 128s21.38-15.41 32.86-8.785l32.96 19.03L91.62 106.8C88.18 93.98 95.78 80.83 108.6 77.39s25.96 4.162 29.39 16.97l20.85 77.82L232 214.4V129.9L175 72.97c-9.375-9.375-9.375-24.56 0-33.94s24.56-9.375 33.94 0L232 62.06V24C232 10.75 242.8 0 256 0s24 10.75 24 24v38.06l23.03-23.03c9.375-9.375 24.56-9.375 33.94 0s9.375 24.56 0 33.94L280 129.9v84.49l73.17-42.25l20.85-77.82c3.432-12.81 16.58-20.4 29.39-16.97c6.402 1.715 11.5 5.861 14.57 11.18s4.109 11.81 2.395 18.21l-8.43 31.46l32.96-19.03C456.4 112.6 471.2 116.5 477.8 128s2.615 26.16-8.859 32.78l-32.96 19.03l31.46 8.432c12.81 3.432 20.4 16.58 16.97 29.39s-16.58 20.4-29.39 16.97l-77.82-20.85L304 255.1l73.17 42.25l77.82-20.85C467.8 273.1 480.1 281.6 484.4 294.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAD80lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5944b4KeG/gOP+5doHvBj6GF848LwSY50+8YD8FvDX/tb4G+GheMPO8EGCeP/GCmf96u8AJXjDzvBBgnj/xgu0Cx/ivdQk4zgtmnhcCzPMnXrCvBj6K/1pfA3w0L5h5Xggwz5944f4aeCn+a/wN8NK8cOZ5IcA8f+KFOw78NvBS/Of6G+C1gV1eOPO8EGCeP/EvOw78NvBS/Of4G+C1gV3+ZeZ5IcA8f+JFcxy4FTjGf6xLwIOBXV405nkhwDx/4kX30sBvA8f4j3EJeG3gr3nRmeeFAPP8iX+dlwZ+GzjGv88l4LWBv+ZfxzwvBJjnT/zrvTTwV/z7vAzw1/zrmeeFAPP8iX+b9wa+i3+b9wG+m38b87wQYJ4/8W/33sB38a/zPsB3829nnhcCzPMn/n3eG/guXjTvA3w3/z7meSHAPH/i3++7gffihfse4L359zPPCwHm+RP/Mb4beC+ev+8B3pv/GOZ5IcA8f+I/zncD78Vz+h7gvfmPY54XAszzJ/5jfTXw1lzx3cBn8x/LPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM83cC2OV/hwcDT+d5IeCvgZfief028D7ArfzP9mDgu4DX5nn9jYCvBj6K/5u+RsCDgafzf9NDxBVfDXwU/7d8DfDR4tn+Gngp/m/4G+ClAcRz+mrgo/jf7WuAj+YKxPN6MPDRwGsDL8X/Dn8D/Dbw1cCtPBvi/zbE/22I/9sQ/7fxj289pRadzdxCAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareCaretDown;
impl IconShape for FaSquareCaretDown {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M320 192H128C118.5 192 109.8 197.7 105.1 206.4C102.2 215.1 103.9 225.3 110.4 232.3l96 104C210.9 341.2 217.3 344 224 344s13.09-2.812 17.62-7.719l96-104c6.469-7 8.188-17.19 4.375-25.91C338.2 197.7 329.5 192 320 192zM384 32H64C28.65 32 0 60.66 0 96v320c0 35.34 28.65 64 64 64h320c35.35 0 64-28.66 64-64V96C448 60.66 419.3 32 384 32zM400 416c0 8.82-7.178 16-16 16H64c-8.822 0-16-7.18-16-16V96c0-8.82 7.178-16 16-16h320c8.822 0 16 7.18 16 16V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEKklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/Of7LOCjueK7gY/h3848LwSY50/85zkO/Bbw0jynrwE+mn8b87wQYJ4/8Z/jOPBbwEvzvHaBE/zbmOeFAPP8if94x4HfAl6aF0z825jnhQDz/In/WC8N/BZwnBdO/NuY54UA8/yJ/zgvDfwWcJx/mfi3Mc8LAeb5E/8xXhr4LeA4Lxrxb2OeFwLM8yf+/d4b+C7+dcS/jXleCDDPn/j3eW/gu/jXE/825nkhwDx/4t/uvYHv4t9G/NuY54UA8/yJf5vvAt6bfzvxb2OeFwLM8yf+9b4LeG/+fcS/jXleCDDPn/jX+S7gvfn3E/825nkhwDx/4kX31cBH8R9D/NuY54UA8/yJF93TgQfzH0P825jnhQDz/IkX3a3Ag/iPIf5tzPNCgHn+xIvus4HP4j+G+LcxzwsB5vkT/zrfDbwX/37i38Y8LwSY50/863038F78+4h/G/O8EGCeP/Fv893Ae/FvJ/5tzPNCgHn+xL/dewPfxb+N+LcxzwsB5vkT/z7vDXwX/3ri38Y8LwSY50/8+7038F3864h/G/O8EGCeP/Ef46WB3waO8aIR/zbmeSHAPH/iP85LA78NHONfJv5tzPNCgHn+xH+slwZ+GzjGCyf+bczzQoB5/sR/vOPAbwMvxQsm/m3M80KAef7Ef47jwG8DL8XzugQc59/GPC8EmOdP/Oc5Dvw28FI8p68BPpp/G/O8EGCeP/Gf77OBj+aK7wY+mn8787wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/ydAHb53+HBwNN5Xgj4a+CleF6/DbwPcCv/sz0Y+C7gtXlefyPgq4GP4v+mrxHwYODp/N/0EHHFVwMfxf8tXwN8tHi2vwZeiv8b/gZ4aQDxnL4a+Cj+d/sa4KO5AvG8Hgx8NPDawEvxv8PfAL8NfDVwK8+G+L8N8X8b4v82xP9t/COHfp0WYdmNOAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareCaretLeft;
impl IconShape for FaSquareCaretLeft {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M384 32H64C28.66 32 0 60.66 0 96v320c0 35.34 28.66 64 64 64h320c35.34 0 64-28.66 64-64V96C448 60.66 419.3 32 384 32zM400 416c0 8.82-7.18 16-16 16H64c-8.82 0-16-7.18-16-16V96c0-8.82 7.18-16 16-16h320c8.82 0 16 7.18 16 16V416zM273.6 138c-8.719-3.812-18.91-2.094-25.91 4.375l-104 96C138.8 242.9 136 249.3 136 256s2.812 13.09 7.719 17.62l104 96c7 6.469 17.19 8.188 25.91 4.375C282.3 370.2 288 361.5 288 352V160C288 150.5 282.3 141.8 273.6 138z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEJ0lEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/Nt9FfDeXPHVwOfwn888LwSY50/823w18FE8p78GXgfY5T+PeV4IMM+f+Lcxz99fA68D7PKfwzwvBJjnT/zbmBfsr4HXAXb5j2eeFwLM8yf+bcwLtwu8DvDX/McyzwsB5vkT/zbmX7YLvA7w1/zHMc8LAeb5E/825kWzC7wO8Nf8xzDPCwHm+RP/NuZf532A7+bfzzwvBJjnT/zbmH+99wG+m38f87wQYJ4/8W9j/m3eB/hu/u3M80KAef7Ev435t/tu4H34tzHPCwHm+RP/Nubf57uB9+FfzzwvBJjnT/zbmH+/7wbeh38d87wQYJ4/8W9j/mN8DvDZvOjM80KAef7Ev435j3Er8BBedOZ5IcA8f+LfxvzHeAbwYF505nkhwDx/4t/G/Mf4HOCzedGZ54UA8/yJfxvz7/c9wHvzr2OeFwLM8yf+bcy/z/cA782/nnleCDDPn/i3Mf923wO8N/825nkhwDx/4t/G/Nu8D/Dd/NuZ54UA8/yJfxvzr/c+wHfz72OeFwLM8yf+bcy/zvsA382/n3leCDDPn/i3MS+aS8BrA3/NfwzzvBBgnj/xb2P+ZZeA1wb+mv845nkhwDx/4t/GvHCXgNcG/pr/WOZ5IcA8f+LfxrxgfwO8NrDLfzzzvBBgnj/xb7MLHON5/Q3w2sAu/znM80KAef7Ev81nA5/Fc/ob4LWBXf7zmOeFAPP8iX+7rwbemyu+Gvhs/vOZ54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY5+8EsMv/Dg8Gns7zQsBfAy/F8/pt4H2AW/mf7cHAdwGvzfP6GwFfDXwU/zd9jYAHA0/n/6aHiCu+Gvgo/m/5GuCjxbP9NfBS/N/wN8BLA4jn9NXAR/G/29cAH80ViOf1YOCjgdcGXor/Hf4G+G3gq4FbeTbE/22I/9sQ/7ch/m/jHwEcmcEW2WUCzAAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareCaretRight;
impl IconShape for FaSquareCaretRight {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M200.3 142.4C193.3 135.9 183.1 134.2 174.4 138C165.7 141.8 160 150.5 160 159.1v192C160 361.5 165.7 370.2 174.4 374c8.719 3.812 18.91 2.094 25.91-4.375l104-96C309.2 269.1 312 262.7 312 256s-2.812-13.09-7.719-17.62L200.3 142.4zM384 32H64C28.66 32 0 60.66 0 96v320c0 35.34 28.66 64 64 64h320c35.34 0 64-28.66 64-64V96C448 60.66 419.3 32 384 32zM400 416c0 8.82-7.18 16-16 16H64c-8.82 0-16-7.18-16-16V96c0-8.82 7.18-16 16-16h320c8.82 0 16 7.18 16 16V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAD9klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7Ef6yvAt6aK34a+Bj+Y5nnhQDz/In/ON8FvDfP6buB9+E/jnleCDDPn/iP8V3Ae/P8fTfwPvzHMM8LAeb5E/9+3wW8Ny/c1wAfzb+feV4IMM+f+Pd5b+C7eNG8D/Dd/PuY54UA8/yJf7v3Br6Lf533Ab6bfzvzvBBgnj/xb/PewHfxb/M2wE/zb2OeFwLM8yf+9V4a+Cv+7XaB1wH+mn8987wQYJ4/8a/z0sBvAcf599kFXgf4a/51zPNCgHn+xIvupYHfAo7zH2MXeAiwy4vOPC8EmOdPvGiOA08HjvMf66+B1wF2edGY54UA8/yJf9lx4LeAl+Y/x18DrwPs8i8zzwsB5vkTL9xx4LeAl+Y/128Dr8O/zDwvBJjnT7xwfwW8NP81vht4H14487wQYJ4/8YJ9NfBR/Nf6GOCrecHM80KAef7EC3YROM5/rd8BXpsXzDwvBJjnT7xgu8Ax/mv9DvDavGDmeSHAPH/iBftq4KP4r/UxwFfzgpnnhQDz/IkX7quBj+I/3yXgq4HP5oUzzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCevxPALv87PBh4Os8LAX8NvBTP67eB9wFu5X+2BwPfBbw2z+tvBHw18FH83/Q1Ah4MPJ3/mx4irvhq4KP4v+VrgI8Wz/bXwEvxf8PfAC8NIJ7TVwMfxf9uXwN8NFcgnteDgY8GXht4Kf53+Bvgt4GvBm7l2RD/tyH+b0P834b4v41/BFedpRaL/iPaAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareCaretUp;
impl IconShape for FaSquareCaretUp {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M241.6 175.7C237.1 170.8 230.7 168 224 168S210.9 170.8 206.4 175.7l-96 104c-6.469 7-8.188 17.19-4.375 25.91C109.8 314.3 118.5 320 127.1 320h192c9.531 0 18.16-5.656 22-14.38c3.813-8.719 2.094-18.91-4.375-25.91L241.6 175.7zM384 32H64C28.65 32 0 60.66 0 96v320c0 35.34 28.65 64 64 64h320c35.35 0 64-28.66 64-64V96C448 60.66 419.3 32 384 32zM400 416c0 8.82-7.178 16-16 16H64c-8.822 0-16-7.18-16-16V96c0-8.82 7.178-16 16-16h320c8.822 0 16 7.18 16 16V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAEbklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/14PBt6KK34GuJUXzjwvBJjnT/z3eW/gu3i2XeBjgO/mBTPPCwHm+RP/Pd4b+C6e1y7wMsCtPH/meSHAPH/iv957A9/FC/Y+wHfz/JnnhQDz/In/Wu8NfBcv3PsA383zZ54XAszzJ/7rvDfwXbxwl4CXBm7l+TPPCwHm+RP/Nd4b+C7+Ze8DfDcvmHleCDDPn/jP997Ad/Evex/gu3nhzPNCgHn+xAv3YOCtuOJngFv513lv4Lv4l70P8N38y8zzQoB5/sQL9t7Ad/Fsu8DHAN/Ni+a9ge/iX/Y+wHfzojHPCwHm+RPP34OBp/P8vQ/w3bxw7w18F/+y9wG+mxedeV4IMM+feP4+GvgqXrD3Ab6b5++9ge/iX/Y+wHfzr2OeFwLM8yeev48GvooX7n2A7+Y5vTfwXfzL3gf4bv71zPNCgHn+xPP3YODp/MveB/hurnhv4Lv4l70P8N3825jnhQDz/IkX7L2B7+Jf9j5c8V38y94H+G7+7czzQoB5/sQL997Ad/Ef432A7+bfxzwvBJjnT/zL3hv4Lv593gf4bv79zPNCgHn+xIvmvYHv4t/mfYDv5j+GeV4IMM+feNG9N/Bd/Ou8D/Dd/McxzwsB5vkT/zrvDXwXL5r3Ab6b/1jmeSHAPH/iX++9ge/ihXsf4Lv5j2eeFwLM8yf+bd4b+C6ev/cBvpv/HOZ5IcA8f+Lf7r2BrwaOccUl4KOB7+Y/j3leCDDPn/j3eTDw2lzx28Ct/OcyzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+TsB7PK/w4OBp/O8EPDXwEvxvH4beB/gVv5nezDwXcBr87z+RsBXAx/F/01fI+DBwNP5v+kh4oqvBj6K/1u+Bvho8Wx/DbwU/zf8DfDSAOI5fTXwUfzv9jXAR3MF4nk9GPho4LWBl+J/h78Bfhv4auBWng3xfxvi/zbE/22I/9v4R4wMuxbukFXjAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareCheck;
impl IconShape for FaSquareCheck {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M211.8 339.8C200.9 350.7 183.1 350.7 172.2 339.8L108.2 275.8C97.27 264.9 97.27 247.1 108.2 236.2C119.1 225.3 136.9 225.3 147.8 236.2L192 280.4L300.2 172.2C311.1 161.3 328.9 161.3 339.8 172.2C350.7 183.1 350.7 200.9 339.8 211.8L211.8 339.8zM0 96C0 60.65 28.65 32 64 32H384C419.3 32 448 60.65 448 96V416C448 451.3 419.3 480 384 480H64C28.65 480 0 451.3 0 416V96zM48 96V416C48 424.8 55.16 432 64 432H384C392.8 432 400 424.8 400 416V96C400 87.16 392.8 80 384 80H64C55.16 80 48 87.16 48 96z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACoUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMogDz/xcCzP9fCDD/fyHA/P+FAPP/FwLM/18IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/8XAsz/Xwgw/38hwPz/hQDz/xcCzP9f/CMQd3QBlS/dTwAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareFull;
impl IconShape for FaSquareFull {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M512 0V512H0V0H512zM464 48H48V464H464V48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAADVklEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sQLdxz4LOCtgQfzn+NW4KeBj+FfZp4XAszzJ164rwY+iv8aXwN8NC+ceV4IMM+feOEuAsf5r3Er8BBeOPO8EGCeP/HC7QLH+K/xDODBvHDmeSHAPH/ihftq4KP4r/E5wGfzwpnnhQDz/Il/2VcDbw08iP8czwC+G/hs/mXmeSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ6/E8Au/zs8GHg6zwsBfw28FM/rt4H3AW7lf7YHA98FvDbP628EfDXwUfzf9DUCHgw8nf+bHiKu+Grgo/i/5WuAjxbP9tfAS/F/w98ALw0gntNXAx/F/25fA3w0VyCe14OBjwZeG3gp/nf4G+C3ga8GbuXZEP+3If5vQ/zfhvi/jX8EUq6HFmn07+UAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquareMinus;
impl IconShape for FaSquareMinus {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M312 232C325.3 232 336 242.7 336 256C336 269.3 325.3 280 312 280H136C122.7 280 112 269.3 112 256C112 242.7 122.7 232 136 232H312zM0 96C0 60.65 28.65 32 64 32H384C419.3 32 448 60.65 448 96V416C448 451.3 419.3 480 384 480H64C28.65 480 0 451.3 0 416V96zM48 96V416C48 424.8 55.16 432 64 432H384C392.8 432 400 424.8 400 416V96C400 87.16 392.8 80 384 80H64C55.16 80 48 87.16 48 96z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAAD6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/uMcBz4LeGuu+GngY/iPZZ4XAszzJ/7jfDXwUTynrwE+mv845nkhwDx/4j/OReA4z+lW4CH8xzHPCwHm+RP/cczzJ/7jmOeFAPP8if845vkT/3HM80KAef7Efxzz/In/OOZ5IcA8f+I/jnn+xH8c87wQYJ4/8R/HPH/iP455Xggwz5/4j2OeP/EfxzwvBJjnT/zHMc+f+I9jnhcCzPMn/uOY50/8xzHPCwHm+RMv3HHgs4C3Bh7Mf45bgZ8GPoZ/mXleCDDPn3jhvhr4KP5rfA3w0bxw5nkhwDx/4oW7CBznv8atwEN44czzQoB5/sQLtwsc47/GM4AH88KZ54UA8/yJF+6rgY/iv8bnAJ/NC2eeFwLM8yf+ZV8NvDXwIP5zPAP4buCz+ZeZ54UA8/yJ/zjm+RP/cczzQoB5/sR/HPP8if845nkhwDx/4j+Oef7EfxzzvBBgnj/xH8c8f+I/jnleCDDPn/iPY54/8R/HPC8EmOdP/Mcxz5/4j2OeFwLM8yf+45jnT/zHMc8LAeb5E/9xzPMn/uOY54UA8/yJ/zi7wDGe0zOAB/MfxzwvBJjnT/zH+Wrgo3hOnwN8Nv9xzPNCgHn+xH+srwbemiu+G/hs/mOZ54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM83cC2OV/hwcDT+d5IeCvgZfief028D7ArfzP9mDgu4DX5nn9jYCvBj6K/5u+RsCDgafzf9NDxBVfDXwU/7d8DfDR4tn+Gngp/m/4G+ClAcRz+mrgo/jf7WuAj+YKxPN6MPDRwGsDL8X/Dn8D/Dbw1cCtPBvi/zbE/22I/9sQ/7fxj2NCoxZyWQbUAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquarePlus;
impl IconShape for FaSquarePlus {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M200 344V280H136C122.7 280 112 269.3 112 256C112 242.7 122.7 232 136 232H200V168C200 154.7 210.7 144 224 144C237.3 144 248 154.7 248 168V232H312C325.3 232 336 242.7 336 256C336 269.3 325.3 280 312 280H248V344C248 357.3 237.3 368 224 368C210.7 368 200 357.3 200 344zM0 96C0 60.65 28.65 32 64 32H384C419.3 32 448 60.65 448 96V416C448 451.3 419.3 480 384 480H64C28.65 480 0 451.3 0 416V96zM48 96V416C48 424.8 55.16 432 64 432H384C392.8 432 400 424.8 400 416V96C400 87.16 392.8 80 384 80H64C55.16 80 48 87.16 48 96z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAABACAYAAABP97SyAAADCUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovi/DfF/G+L/NsT/bYjn9WDgo4DXBl6a/x3+Gvht4GuAW3k2xHP6KuCj+d/tq4GP4QrEs/0V8NL83/DXwMsAiCu+Gvgo/m/5GuCjBTwYeDr/Nz1EwFcDH8X/TV8j4K+Al+Z5/Q7w3sCt/M/2YOC7gdfief21APP8PQS4lf8dHgw8neeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/In/XczzQoB5/sT/LuZ5IcA8f+J/F/O8EGCeP/G/i3leCDDPn/jfxTwvBJjnT/zvYp4XAszzJ/53Mc8LAeb5E/+7mOeFAPP8if9dzPNCgHn+xP8u5nkhwDx/4n8X87wQYJ4/8b+LeV4IMM+f+N/FPC8EmOdP/O9inhcCzPMn/ncxzwsB5vkT/7uY54UA8/yJ/13M80KAef7E/y7meSHAPH/ifxfzvBBgnj/xv4t5Xggwz5/438U8LwSY50/872KeFwLM8yf+dzHPCwHm+RP/u5jnhQDz/J0Advnf4cHA03leCPhr4KV4Xr8NvA9wK/+zPRj4LuC1eV5/I+CrgY/i/6avEfBg4On83/QQccVXAx/F/y1fA3y0eLa/Bl6K/xv+BnhpAPGcvhr4KP53+xrgo7kC8bweDHw08NrAS/G/w98Avw18NXArz4b4vw3xfxvi/zbE/238IwDPeRbzhAUzAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSquare;
impl IconShape for FaSquare {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M384 32C419.3 32 448 60.65 448 96V416C448 451.3 419.3 480 384 480H64C28.65 480 0 451.3 0 416V96C0 60.65 28.65 32 64 32H384zM384 80H64C55.16 80 48 87.16 48 96V416C48 424.8 55.16 432 64 432H384C392.8 432 400 424.8 400 416V96C400 87.16 392.8 80 384 80z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaStarHalfStroke;
impl IconShape for FaStarHalfStroke {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8zM287.1 384.7C291.9 384.7 295.7 385.6 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.1 79.09L287.1 384.7z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABACAYAAAC5vjEqAAAFZElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovif46uA9+aK7wY+hv9+iP8Zvhr4KJ7T5wCfzX8vxH+/BwNP53ndCjyE/16I/37fDbwXz5/474X47/Vg4Om8YOK/F+K/13cD78ULJv57If77HAeeDhznBRP/vRD/fT4b+CxeOPHfC/Hf4zjwdOA4L5z474X47/HZwGfxLxP/vRD/9Y4DTweO8y8T/70Q//U+G/gsXjTivxfiv9Zx4OnAcV404r8X4r/WZwOfxYtO/PdC/Ne6CBznRSf+eyH+67w38F3864j/Xoj/Ok8HHszz+h3gtXj+xH8vxH+N9wa+i+fvdYDf4vkT/70Q/zWeDjyY5/U7wGsD5vkT/70Q/zleGngQ8NLASwNvzfP3NsBPA+b5E/+9EP92DwYeBLw2cBx4aeDBwIN50TwDeDBXmOdP/PdCvHDHgZcCXho4Drw2cBx4af793gf4bq4wz5/474V4Xu8FvDfw2vzneQbwYJ7NPH/ivxfiOX028Fn853sb4Kd5NvP8if9eiOd0ETjOf55LwFcDn81zMs+f+O+FeE67wDH+/S4Bfw38NbAL/DawC/w1z595/sR/L8Rz+m7gvXjRXAL+GrgVuBX4a2AX+G3+9czzJ/57IZ7TceC3gZfiBftr4GX4j2WeP/HfC/G8jgO/DbwUL9h3A+/Dfxzz/In/Xojn7zjw28BL8YJ9N/A+/Mcwz5/474V4wY4Dvw28FC/YdwPvw7+fef7Efy/EC3cc+G3gpXjBvht4H/59zPMn/nsh/mXHgd8GXooX7LuB9+Hfzjx/4r8X4kVzHPht4KV4wb4beB/+bczzJ/57IV50x4HfBl6KF+y7gffhX888f+K/F+Jf5zjw28BL8YJ9N/A+/OuY50/890L86x0Hfht4KV6w7wbehxedef7Efy/Ev81x4LeBl+IF+27gfXjRmOdP/PdC/NsdB34beClesM8BPpt/mXn+xH8vxL/PceC3gZfi+bsVeAj/MvP8if9eiH+/48BvAy/F8yf+Zeb5E/+9EP8xXhv4LZ7X7wCvzb/MPH/ivxfiP8ZnA5/F8/oe4L35l5nnT/z3QvzH+GngrXhenwN8Nv8y8/yJ/16I/xh/Bbw0z+t1gN/mX2aeP/HfC/Efwzx/DwFu5V9mnj/x3wvx7/fSwF/xvC4Bx3nRmOdP/PdC/Pu9NfBTPK/fAV6bF415/sR/L8S/32cDn8Xz+hrgo3nRmOdP/PdC/Pv9NvBaPK+PAb6af9lnAZ/N8yf+eyH+/Z4OPJjn9TrAb/OCvRfw2cCDecHEfy/Ev595/k4Auzyvtwa+CngwL9wl4Dj/vRD/Pq8N/BbP6xJwnOf02sBnAa/Ni+ZrgI/mvxfi3+ejga/ief0O8Npc8dLAVwGvzYvue4CPBnb574X49/lq4KN4Xl8DfDXwWcB786L7HeCzgd/mfwbEv89vA6/F87oVeDAvut8BPhv4bf5nQfz7XASO82/3DOCzge/mfybEv91x4CL/Ns8APhv4bv5nQ/zbvTbwW/zrXAK+Gvhs/ndA/Nt9NvBZvGguAV8NfDWwy/8eiH+7zwY+i3/Z9wAfDezyvw/i3+6zgc/iBfse4LOBW/nfC/Fv92Dgr4FjPKffAT4a+Gv+90P8+7w08N3ASwG/A3w28Nv838E/AkDys0GXC0qQAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaStarHalf;
impl IconShape for FaStarHalf {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M293.3 .6123C304.2 3.118 311.9 12.82 311.9 24V408.7C311.9 417.5 307.1 425.7 299.2 429.8L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.1 115.1 483.9L142.2 328.4L31.11 218.3C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C271.2 3.46 282.4-1.893 293.3 .6127L293.3 .6123zM263.9 128.4L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L263.9 394.3L263.9 128.4z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaStar;
impl IconShape for FaStar {
    fn view_box(&self) -> &str {
        "0 0 576 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0L287.9 0zM287.9 78.95L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L276.6 387.5C283.7 383.7 292.2 383.7 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.9 78.95z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaSun;
impl IconShape for FaSun {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M505.2 324.8l-47.73-68.78l47.75-68.81c7.359-10.62 8.797-24.12 3.844-36.06c-4.969-11.94-15.52-20.44-28.22-22.72l-82.39-14.88l-14.89-82.41c-2.281-12.72-10.76-23.25-22.69-28.22c-11.97-4.936-25.42-3.498-36.12 3.844L256 54.49L187.2 6.709C176.5-.6016 163.1-2.039 151.1 2.896c-11.92 4.971-20.4 15.5-22.7 28.19l-14.89 82.44L31.15 128.4C18.42 130.7 7.854 139.2 2.9 151.2C-2.051 163.1-.5996 176.6 6.775 187.2l47.73 68.78l-47.75 68.81c-7.359 10.62-8.795 24.12-3.844 36.06c4.969 11.94 15.52 20.44 28.22 22.72l82.39 14.88l14.89 82.41c2.297 12.72 10.78 23.25 22.7 28.22c11.95 4.906 25.44 3.531 36.09-3.844L256 457.5l68.83 47.78C331.3 509.7 338.8 512 346.3 512c4.906 0 9.859-.9687 14.56-2.906c11.92-4.969 20.4-15.5 22.7-28.19l14.89-82.44l82.37-14.88c12.73-2.281 23.3-10.78 28.25-22.75C514.1 348.9 512.6 335.4 505.2 324.8zM456.8 339.2l-99.61 18l-18 99.63L256 399.1L172.8 456.8l-18-99.63l-99.61-18L112.9 255.1L55.23 172.8l99.61-18l18-99.63L256 112.9l83.15-57.75l18.02 99.66l99.61 18L399.1 255.1L456.8 339.2zM256 143.1c-61.85 0-111.1 50.14-111.1 111.1c0 61.85 50.15 111.1 111.1 111.1s111.1-50.14 111.1-111.1C367.1 194.1 317.8 143.1 256 143.1zM256 319.1c-35.28 0-63.99-28.71-63.99-63.99S220.7 192 256 192s63.99 28.71 63.99 63.1S291.3 319.1 256 319.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAF6klEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4grjgMfBbw28Nq8cL8N/DXwOcAu/34PBl4LeG3gwcBx4KX5l/018NvA1wC38m+DgOPAXwEP5l9nF3gIsMu/3nHgrYDPBh7Mv99XAx/Dvx4Cvht4L/5tvgd4b150x4GPAj4aOM5/rL8GXgfY5UWHgKcDD+bf5lbgIbxoXhv4LuDB/Of5buB9eNEhwPz7iBfuOPBVwHvzX+N1gN/mRYMA8+8jXrCXBr4LeGn+6/wM8Na8aBBg/n3E8/fewFcBx/mX/Q3w28BvA7cCf80L92Dgq4G34vkTLxoEmH8f8by+C3hv/mXfA3w38Nv86x0HbgWO8bxeBvhr/mUIMP8+4jl9NfBRvHDPAN4b+G3+fX4beC2e1y7wOsBf88IhwPz7iOf0dODBvGDfA3w0sMu/328Dr8Xztws8BNjlBUOA+fcRz2kXOMbzugR8NPDd/Mf5aeCteMF+GngbXjAEmH8f8Zw+G/gsntPfAO8N/DX/sV4a+G3gGC/YQ4Bbef4QYP59xPP6auC9ueK7gc8GdvnPcRz4bOCjeP4+B/hsnj8EmH8f8T/DZwOfxfP6HeC1ef4QYP59xP8Mrw38Fs/rd4DX5vlDgPn3Ef8zvDfwXTyvvwZehucPAebfR/zP8FfAS/P8fTfwPjwvBJh/H/E/wy5wjBfsa4CP5jkhwPz7iP8Zfhp4K1641wF+m2dDgPn3Ef8zPBj4beBBvGA/A7w1z4YA8+8j/uc4Drw18NHAS/H8PQS4lSsQYP59xP88x4G/Bh7E83od4Le5AgHm30f8z/TTwFvxvD4G+GquQID59xH/M/0V8NI8r7cBfporEGD+fcT/PO8NfBfP39sAP80VCDD/PuJ/nqcDD+b52wXeBvhtAAHm30f8z2P+ZS8D/LUA8+8j/uf5aeCteOH+GngZAebfR/zP82Dgp4GX4oV7HwHm30f8z/XSwGsDnw0c43n9jADz7yP+53tv4Lt4XrcKMP8+4n8H87wQcCvwIP5tLgHH+Z/vrYGf4nldEvDdwHvxb/M9wHvzP9dx4LWA7waO87x+RsBx4LeBl+Jf5xLw0sCtPK+vAt6aK74b+BpglxfNewNfBRznP9/HiGf7aOC1geP8y/4a+Gxgl+f11cBH8Zz+Gngf4K954V4b+C3+a1wCHiz+410EjvO8doHXAf6aF+yzgc/iv8bbAD8t/uPdCjyI5+9W4GWAXZ6/jwa+iv987wN8N4D4j/fVwEfxgv028Do8f8eBi/znuQS8N/DTXIH4z/HdwHvxgn0O8Nk8f98NvBf/cS4Bvw38NvDdwC7PhvjP893Ae/H87QIvA9zK83ow8HSev7cBfpr/OIj/XN8NvBfP328Dr8Pz99PAW/G8fgZ4a/7jIP7z/TXwUjx/rwP8Ns/rrYGf4vk7AezyHwPxn+/BwF8Dx3hevw28Ds/fLnCM5/U2wE/zHwPxX+Ozgc/i+Xsd4Ld5Xt8NvBfP62uAj+Y/BuK/zq3Ag3hePwO8Nc/rvYHv4nn9DvDa/MdA/Nd5b+C7eP4eAtzKc3ow8HSeP/EfA/Ff5zhwK3CM5/U1wEfzvMzzJ/5jIP5rfTbwWTyvXeAhwC7P9t7Ad/G8LgHH+Y+B+K/1YODpPH/fDXwMV7wW8N3AcZ7XzwBvzX8MxH+97wbei3+79wG+m/8YiP96Dwaezr/NM4AH8x8H8d/jq4GP4l/vdYDf5j8O4r/HceC3gZfiRfc+wHfzHwvx3+c48NPAa/HCPQP4aOCn+Y+H+O/31sBbA68NPIgrngH8NfDTwE8Du/znQPz/hvj/DfH/G+L/N/4RmyjmT/BMdp0AAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaThumbsDown;
impl IconShape for FaThumbsDown {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M128 288V64.03c0-17.67-14.33-31.1-32-31.1H32c-17.67 0-32 14.33-32 31.1v223.1c0 17.67 14.33 31.1 32 31.1h64C113.7 320 128 305.7 128 288zM481.5 229.1c1.234-5.092 1.875-10.32 1.875-15.64c0-22.7-11.44-43.13-29.28-55.28c.4219-3.015 .6406-6.076 .6406-9.122c0-22.32-11.06-42.6-28.83-54.83c-2.438-34.71-31.47-62.2-66.8-62.2h-52.53c-35.94 0-71.55 11.87-100.3 33.41L169.6 92.93c-6.285 4.71-9.596 11.85-9.596 19.13c0 12.76 10.29 24.04 24.03 24.04c5.013 0 10.07-1.565 14.38-4.811l36.66-27.51c20.48-15.34 45.88-23.81 71.5-23.81h52.53c10.45 0 18.97 8.497 18.97 18.95c0 3.5-1.11 4.94-1.11 9.456c0 26.97 29.77 17.91 29.77 40.64c0 9.254-6.392 10.96-6.392 22.25c0 13.97 10.85 21.95 19.58 23.59c8.953 1.671 15.45 9.481 15.45 18.56c0 13.04-11.39 13.37-11.39 28.91c0 12.54 9.702 23.08 22.36 23.94C456.2 266.1 464 275.2 464 284.1c0 10.43-8.516 18.93-18.97 18.93H307.4c-12.44 0-24 10.02-24 23.1c0 4.038 1.02 8.078 3.066 11.72C304.4 371.7 312 403.8 312 411.2c0 8.044-5.984 20.79-22.06 20.79c-12.53 0-14.27-.9059-24.94-28.07c-24.75-62.91-61.74-99.9-80.98-99.9c-13.8 0-24.02 11.27-24.02 23.99c0 7.041 3.083 14.02 9.016 18.76C238.1 402 211.4 480 289.9 480C333.8 480 360 445 360 411.2c0-12.7-5.328-35.21-14.83-59.33h99.86C481.1 351.9 512 321.9 512 284.1C512 261.8 499.9 241 481.5 229.1z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaThumbsUp;
impl IconShape for FaThumbsUp {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M96 191.1H32c-17.67 0-32 14.33-32 31.1v223.1c0 17.67 14.33 31.1 32 31.1h64c17.67 0 32-14.33 32-31.1V223.1C128 206.3 113.7 191.1 96 191.1zM512 227c0-36.89-30.05-66.92-66.97-66.92h-99.86C354.7 135.1 360 113.5 360 100.8c0-33.8-26.2-68.78-70.06-68.78c-46.61 0-59.36 32.44-69.61 58.5c-31.66 80.5-60.33 66.39-60.33 93.47c0 12.84 10.36 23.99 24.02 23.99c5.256 0 10.55-1.721 14.97-5.26c76.76-61.37 57.97-122.7 90.95-122.7c16.08 0 22.06 12.75 22.06 20.79c0 7.404-7.594 39.55-25.55 71.59c-2.046 3.646-3.066 7.686-3.066 11.72c0 13.92 11.43 23.1 24 23.1h137.6C455.5 208.1 464 216.6 464 227c0 9.809-7.766 18.03-17.67 18.71c-12.66 .8593-22.36 11.4-22.36 23.94c0 15.47 11.39 15.95 11.39 28.91c0 25.37-35.03 12.34-35.03 42.15c0 11.22 6.392 13.03 6.392 22.25c0 22.66-29.77 13.76-29.77 40.64c0 4.515 1.11 5.961 1.11 9.456c0 10.45-8.516 18.95-18.97 18.95h-52.53c-25.62 0-51.02-8.466-71.5-23.81l-36.66-27.51c-4.315-3.245-9.37-4.811-14.38-4.811c-13.85 0-24.03 11.38-24.03 24.04c0 7.287 3.312 14.42 9.596 19.13l36.67 27.52C235 468.1 270.6 480 306.6 480h52.53c35.33 0 64.36-27.49 66.8-62.2c17.77-12.23 28.83-32.51 28.83-54.83c0-3.046-.2187-6.107-.6406-9.122c17.84-12.15 29.28-32.58 29.28-55.28c0-5.311-.6406-10.54-1.875-15.64C499.9 270.1 512 250.2 512 227z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaTrashCan;
impl IconShape for FaTrashCan {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M160 400C160 408.8 152.8 416 144 416C135.2 416 128 408.8 128 400V192C128 183.2 135.2 176 144 176C152.8 176 160 183.2 160 192V400zM240 400C240 408.8 232.8 416 224 416C215.2 416 208 408.8 208 400V192C208 183.2 215.2 176 224 176C232.8 176 240 183.2 240 192V400zM320 400C320 408.8 312.8 416 304 416C295.2 416 288 408.8 288 400V192C288 183.2 295.2 176 304 176C312.8 176 320 183.2 320 192V400zM317.5 24.94L354.2 80H424C437.3 80 448 90.75 448 104C448 117.3 437.3 128 424 128H416V432C416 476.2 380.2 512 336 512H112C67.82 512 32 476.2 32 432V128H24C10.75 128 0 117.3 0 104C0 90.75 10.75 80 24 80H93.82L130.5 24.94C140.9 9.357 158.4 0 177.1 0H270.9C289.6 0 307.1 9.358 317.5 24.94H317.5zM151.5 80H296.5L277.5 51.56C276 49.34 273.5 48 270.9 48H177.1C174.5 48 171.1 49.34 170.5 51.56L151.5 80zM80 432C80 449.7 94.33 464 112 464H336C353.7 464 368 449.7 368 432V128H80V432z",
            }
        }
    }
}

/// ![](data:image/png;base64,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)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaUser;
impl IconShape for FaUser {
    fn view_box(&self) -> &str {
        "0 0 448 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M272 304h-96C78.8 304 0 382.8 0 480c0 17.67 14.33 32 32 32h384c17.67 0 32-14.33 32-32C448 382.8 369.2 304 272 304zM48.99 464C56.89 400.9 110.8 352 176 352h96c65.16 0 119.1 48.95 127 112H48.99zM224 256c70.69 0 128-57.31 128-128c0-70.69-57.31-128-128-128S96 57.31 96 128C96 198.7 153.3 256 224 256zM224 48c44.11 0 80 35.89 80 80c0 44.11-35.89 80-80 80S144 172.1 144 128C144 83.89 179.9 48 224 48z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAACpUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4jn9WDgs4CXBl6a/xv+Gvht4GuAW3k2xHP6KuCj+b/tq4GP4QrEs/0W8Nr8//DXwMsAiCu+Gvgo/n/5GuCjBTwYeDr/Pz1EwHcD78X/T18j4K+Al+b/p78WYP7/QoD5/wsB5v8vBJj/vxBg/v9CgPn/CwHm/y8EmP+/EGD+/0KA+f8LAeb/LwSY/78QYP7/QoD5/wsB5v8vBJj/vxBg/v9CgHn+xP8t5nkhwDx/4v8W87wQYJ4/8X+LeV4IMM+f+L/FPC8EmOdP/N9inhcCzPMn/m8xzwsB5vkT/7eY54UA8/yJ/1vM80KAef7E/y3meSHAPH/i/xbzvBBgnj/xf4t5Xggwz5/4v8U8LwSY50/832KeFwLM8yf+bzHPCwHm+RP/t5jnhQDz/In/W8zzQoB5/sT/LeZ5IcA8f+L/FvO8EGCeP/F/i3leCDDPn/i/xTwvBJjnT/zfYp4XAszzJ/5vMc8LAeb5E/+3mOeFAPP8if9bzPNCgHn+TgC7/N/wYODpPC8E/DXwUjyv3wbeB7iV/90eDHwX8No8r78R8NXAR/H/09cIeDDwdP5/eoi44quBj+L/l68BPlo8218DL8X/D38DvDSAeE5fDXwU/7d9DfDRXIF4Xg8GPhp4beCl+L/hb4DfBr4auJVnQ/z/hvj/DfH/G+L/N/4RhKJkFm1ww9YAAAAASUVORK5CYII=)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaWindowMaximize;
impl IconShape for FaWindowMaximize {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.724 65.49C13.36 55.11 21.79 46.47 32 40.56C39.63 36.15 48.25 33.26 57.46 32.33C59.61 32.11 61.79 32 64 32H448C483.3 32 512 60.65 512 96V416C512 451.3 483.3 480 448 480H64C28.65 480 0 451.3 0 416V96C0 93.79 .112 91.61 .3306 89.46C1.204 80.85 3.784 72.75 7.724 65.49V65.49zM48 416C48 424.8 55.16 432 64 432H448C456.8 432 464 424.8 464 416V224H48V416z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAABmElEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyH+f0P8/4b4/w3x/xvi/zfE/2+I/98Q/78h/n9D/P+G+P8N8f8b4v83xP9viP/fEP+/If5/Q/z/hvj/DfH/G+L/N8T/b4j/3xD/vyGu+CrgrYEH8//DrcBPAx8j4KuBj+L/p68RcBE4zv9PtwrYBY7x/9MzBHw18FH8//Q54orPBt4beBD/PzwD+G7gs8X/b4j/3xD/vyH+f+MfAQwCDRBeufcyAAAAAElFTkSuQmCC)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaWindowMinimize;
impl IconShape for FaWindowMinimize {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M0 456C0 442.7 10.75 432 24 432H488C501.3 432 512 442.7 512 456C512 469.3 501.3 480 488 480H24C10.75 480 0 469.3 0 456z",
            }
        }
    }
}

/// ![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAEUUlEQVR4Ae3gAZAkSZIkSRKLqpm7R0REZmZmVlVVVVV3d3d3d/fMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMdHd3d3dXV1VVVVVmZkZGRIS7m5kKz0xmV3d1d3dPz8zMzMxMovjP817AawMvDbw0/z3+Gvhr4KeBn+F5If7jvTXwVcCD+Z/lVuBjgJ/m2RD/sb4K+Gj+Z/tq4GO4AvEf56uBj+J/h68BPhpA/Md4a+Cn+N/lbYCfFv8xng48mP9dbgUeIv793hv4Lp6/ZwAfDfw1cCv/tR4MvDTw1cCDeP7eRvz7fTfwXjyvZwAvDezy3+s48NfAg3he3yP+/f4KeGme19sAP83/DG8N/BTP66/Fv595/k4Au/zP8GDg6TwvxL+fef7E/yzmeSH+/czzJ/5nMc8L8f+HeV6I/z/M80L8/2GeF+L/D/O8EM/rwcBHAa8NvDT/M/w18NvA1wC38m9jnhfiOX0V8NH8z/bVwMfwr2eeF+LZ/gp4af53+GvgZfjXMc8LccVXAx/F/y5fA3w0LzrzvBDwYODp/O/0EOBWXjTmeSHgq4GP4n+nrwE+mheNeV4I+Cvgpfnf6a+Bl+FFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAsz/buJFY54XAszzJ/5nMc+feNGY54UA8/yJ/1nM8ydeNOZ5IcA8f+J/FvP8iReNeV4IMM+f+J/FPH/iRWOeFwLM8yf+ZzHPn3jRmOeFAPP8if9ZzPMnXjTmeSHAPH/ifxbz/Il/2YOBp/O8EGCeP/E/i3n+xL/srYGf4nn9jQDz/In/WczzJ16448BfAQ/meX2PAPP8if9ZzPMnnr8HAy8FfDXwYJ6/txFgnj/xP4t5/sRzMi+aZwAPFmCeP/E/i3n+xHMyL5q3AX5agHn+xP8s5vkTz8n8y74G+GgAAeb5E/+zmOdPPCfzwn0N8NFcgQDz/In/WczzJ56Tef6eAXw08NM8GwLM8yf+ZzHPn3hO5tn+Bvhr4KeBn+Z5IcA8f+J/FvP8iX87BJjnT/zPYp4/8W+HAPP8if9ZzPMn/u0QYJ4/8T+Lef7Evx0CzPMn/mcxz5/4t0OAef7E/yzm+RP/dggwz5/4n8U8f+LfDgHm+RP/s5jnT/zbIcA8fyeAXf5neDDwdJ4/8W+HgL8GXorn9dvA+wC38t/rwcB3Aa/N8/ob4KX5t0PAVwMfxf9OXwN8NP92CHgw8HT+d3oIcCv/dogrvhr4KP53+Rrgo/n3QTzbXwMvxf8OfwO8NP9+iOf01cBH8T/b1wAfzX8MxPN6MPDRwGsDL8X/DH8D/Dbw1cCt/MfhHwH+a7Gixmvp7wAAAABJRU5ErkJggg==)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FaWindowRestore;
impl IconShape for FaWindowRestore {
    fn view_box(&self) -> &str {
        "0 0 512 512"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        (user_color, "none", "0")
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M432 48H208C190.3 48 176 62.33 176 80V96H128V80C128 35.82 163.8 0 208 0H432C476.2 0 512 35.82 512 80V304C512 348.2 476.2 384 432 384H416V336H432C449.7 336 464 321.7 464 304V80C464 62.33 449.7 48 432 48zM320 128C355.3 128 384 156.7 384 192V448C384 483.3 355.3 512 320 512H64C28.65 512 0 483.3 0 448V192C0 156.7 28.65 128 64 128H320zM64 464H320C328.8 464 336 456.8 336 448V256H48V448C48 456.8 55.16 464 64 464z",
            }
        }
    }
}
