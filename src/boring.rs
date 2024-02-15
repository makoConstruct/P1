//performance refactor that could be done: take mut Writers instead of outputting Strings

use elementtree::WriteOptions;
use mako_infinite_shuffle::{Cross, Indexing, OpsRef};
use nalgebra::{Rotation2, Vector2};
use std::{
    f64::consts::TAU,
    fmt::Display,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
};

pub fn from_angle_mag(angle: f64, mag: f64) -> V2 {
    V2::new(angle.cos() * mag, angle.sin() * mag)
}
pub fn from_angle(angle: f64) -> V2 {
    from_angle_mag(angle, 1.0)
}
//rotation should be unit. If it isn't, the result will be scaled by the magnitude of rotation.
pub fn rotate(rotation: V2, v: V2) -> V2 {
    V2::new(
        rotation.x * v.x - rotation.y * v.y,
        rotation.y * v.x + rotation.x * v.y,
    )
}
pub fn both_dims(v: f64) -> V2 {
    V2::new(v, v)
}

pub fn render_png(name: &str) {
    let mut c = Command::new("inkscape");
    c.arg("--export-type=\"png\"");
    c.arg(&format!("{}.svg", name));
    c.output().unwrap();
}

pub struct Displaying<F: Fn(&mut dyn Write)>(pub F);
impl<F> Display for Displaying<F>
where
    F: Fn(&mut dyn Write),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = assume_writes_utf8(&self.0);
        f.write_str(&r).into()
    }
}

// field forest mountain volcano lake ice tomb void
pub type ElementTag = usize;
pub const FIELD_I: usize = 0;
pub const FOREST_I: usize = 1;
pub const MOUNTAIN_I: usize = 2;
pub const VOLCANO_I: usize = 3;
pub const LAKE_I: usize = 4;
pub const ICE_I: usize = 5;
pub const TOMB_I: usize = 6;
pub const VOID_I: usize = 7;
pub fn opposite_element(e: ElementTag) -> ElementTag {
    match e {
        FIELD_I => FOREST_I,
        FOREST_I => FIELD_I,
        MOUNTAIN_I => VOLCANO_I,
        VOLCANO_I => MOUNTAIN_I,
        LAKE_I => ICE_I,
        ICE_I => LAKE_I,
        TOMB_I => VOID_I,
        VOID_I => TOMB_I,
        _ => panic!("invalid ElementTag"),
    }
}
pub const ELEMENT_G: [ElementGenerator; 8] = [
    field_g, forest_g, mountain_g, volcano_g, lake_g, ice_g, tomb_g, void_g,
];
pub const ELEMENT_NAMES: [&'static str; 8] = [
    "field", "forest", "mountain", "volcano", "lake", "ice", "tomb", "void",
];
pub const ELEMENT_PAIR_NAMES: [&'static str; 4] =
    ["field/forest", "mountain/volcano", "lake/ice", "tomb/void"];
pub const ELEMENT_COLORS_BACK: [&'static str; 8] = [
    "b5efb9", "94cf9c", "eeeca7", "efcfcf", "c3edf1", "e1eff0", "ebebeb", "969696",
];
pub const BOLD_COLOR_FOR_GRAPHIC: &'static str = "4b4b4b";
pub const ELEMENT_COLORS_FRONT: [&'static str; 8] = [
    "a3e2a7", "7eb47f", "e5e383", "f2b7b7", "a5dae0", "f4fcfd", "dedede", "414141",
];
pub const fn element_colors_bold(i: ElementTag) -> &'static str {
    if i != ICE_I && i != TOMB_I {
        ELEMENT_COLORS_FRONT[i]
    } else {
        BOLD_COLOR_FOR_GRAPHIC
    }
}
// macro_rules ! for_each_element {
//     ($f:ident) => {
//         f!(mountain)
//         f!(volcano)
//         f!(lake)
//         f!(ice)
//         f!(field)
//         f!(forest)
//         f!(void)
//         f!(tomb)
//     }
// }

pub fn elements() -> impl Indexing<Item = ElementTag> + Clone {
    0..8
}
pub fn element_primaries() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    (0..4).into_map(|i| (i * 2, i * 2 + 1))
}

pub type ElementGenerator = fn(V2, f64, &mut dyn Write);
pub fn each_nonequal_element() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    Cross(elements(), 0..7).into_map(|(a, b)| (a, if a > b { b } else { b + 1 }))
}
pub fn each_unordered_nonequal_pairing() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    mako_infinite_shuffle::KSubsets::new(8,2).into_map(|v| (v[0], v[1]))
}
pub fn each_unordered_pairing() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    mako_infinite_shuffle::KSubmultisets::new(8,2).into_map(|v| (v[0], v[1]))
}
pub fn each_unordered_nonequal_triple() -> impl Indexing<Item = (ElementTag, ElementTag, ElementTag)>
{
    mako_infinite_shuffle::KSubsets::new(8,3).into_map(|v| (v[0], v[1], v[2]))
}
pub type CardGen = Box<dyn Indexing<Item = CardSpec> + 'static>;

pub type V2 = Vector2<f64>;
// pub type U2 = Unit<V2>;
pub type R2 = Rotation2<f64>;

// there doesn't seem to be a way to make transforms be measured in pixel units :( but it hardly affects us to be real
// pub const big_element_dimensions:V2 = V2::new(405.540, 405.540);
// pub const card_dimensions:V2 = V2::new(600.0, 825.0);
// pub const end_graphic_center:V2 = V2::new(300.0, 525.0);
// pub const end_graphic_allowable_rad:f64 = 262.5;

pub const BIG_ELEMENT_SPAN: f64 = 107.299;
pub const BIG_ELEMENT_DIMENSIONS: V2 = V2::new(BIG_ELEMENT_SPAN, BIG_ELEMENT_SPAN);
pub const BIG_ELEMENT_RAD: f64 = BIG_ELEMENT_SPAN / 2.0;
pub const END_GRAPHIC_CENTER: V2 = V2::new(79.375, 138.906);
pub const GRAPHIC_RAD: f64 = 69.4535;
pub const CARD_DIMENSIONS: V2 = V2::new(158.75, 218.28127);
pub const CUTLINE_INSET: V2 = V2::new(9.922, 9.922);
pub const STANDARD_PAIR_SCALE: f64 = 0.6;

type Gravity = V2;
pub const LEFT_TOP: Gravity = V2::new(-1.0, -1.0);
pub const MIDDLE_TOP: Gravity = V2::new(0.0, -1.0);
pub const RIGHT_TOP: Gravity = V2::new(1.0, -1.0);
pub const LEFT_MIDDLE: Gravity = V2::new(-1.0, 0.0);
pub const MIDDLE_MIDDLE: Gravity = V2::new(0.0, 0.0);
pub const RIGHT_MIDDLE: Gravity = V2::new(1.0, 0.0);
pub const LEFT_BOTTOM: Gravity = V2::new(-1.0, 1.0);
pub const MIDDLE_BOTTOM: Gravity = V2::new(0.0, 1.0);
pub const RIGHT_BOTTOM: Gravity = V2::new(1.0, 1.0);
fn offset_for_grav(anchor: V2, grav: Gravity, bounds: V2) -> V2 {
    offset_for_grav_scale(anchor, grav, bounds, 1.0)
}
//I think the minus here might be wrong o_o it was wrong when transplanted to the below
fn offset_for_grav_scale(anchor: V2, grav: Gravity, bounds: V2, scale: f64) -> V2 {
    anchor - (grav + V2::new(1.0, 1.0)).component_mul(&(scale * bounds / 2.0))
}
fn anchor_for_grav(grav: Gravity, bounds: V2) -> V2 {
    (grav + V2::new(1.0, 1.0)).component_mul(&(bounds / 2.0))
}

pub fn field_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    let color_back = ELEMENT_COLORS_BACK[FIELD_I];
    let color_front = ELEMENT_COLORS_FRONT[FIELD_I];
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
    inkscape:label="Layer 1"
    inkscape:groupmode="layer"
    transform="translate(-1176.4656,-400.9298)"
    id="g3">
  <g
      id="g4">
    <circle
        style="fill:#{color_back};fill-opacity:1;stroke-width:14.1468;stroke-linecap:round;stroke-linejoin:round"
        id="circle472"
        cx="-454.57941"
        cy="1230.1152"
        r="53.649605"
        transform="rotate(-90)" />
    <g
        transform="matrix(0,-2.3578005,2.3578005,0,-15090.411,6086.077)"
        style="stroke-width:4;stroke-dasharray:none"
        id="g1">
      <g
          id="g473"
          style="stroke-width:4;stroke-dasharray:none">
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2381.4098,6916.3753 -4.6242,-0.064"
            id="path472"
            sodipodi:nodetypes="cc" />
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2381.4098,6927.5459 -4.6242,-0.064"
            id="path473"
            sodipodi:nodetypes="cc" />
      </g>
      <g
          id="g476"
          transform="translate(0.726353)"
          style="stroke-width:4;stroke-dasharray:none">
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2390.0395,6910.6567 -4.6241,-0.065"
            id="path474"
            sodipodi:nodetypes="cc" />
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2390.0395,6922.0949 -4.6241,-0.064"
            id="path475"
            sodipodi:nodetypes="cc" />
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2390.0395,6933.2655 -4.6241,-0.064"
            id="path476"
            sodipodi:nodetypes="cc" />
      </g>
      <g
          id="g478"
          transform="translate(18.712206)"
          style="stroke-width:4;stroke-dasharray:none">
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2381.4098,6916.3753 -4.6242,-0.064"
            id="path477"
            sodipodi:nodetypes="cc" />
        <path
            style="fill:none;fill-opacity:1;stroke:#{color_front};stroke-width:4;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
            d="m 2381.4098,6927.5459 -4.6242,-0.064"
            id="path478"
            sodipodi:nodetypes="cc" />
      </g>
    </g>
  </g>
</g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap();
}
pub fn forest_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    let color_back = ELEMENT_COLORS_BACK[FOREST_I];
    let color_front = ELEMENT_COLORS_FRONT[FOREST_I];
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1063.4417,-346.64583)"><g
       id="g495"
       transform="matrix(0,-2.3578005,2.3578005,0,-17505.41,5241.7665)"><circle
         style="fill:#{color_back};fill-opacity:0.984314;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle490"
         cx="2053.3845"
         cy="7898.2515"
         r="22.754089" /><g
         id="g494"
         transform="matrix(0,0.8416404,-0.8416404,0,8491.8462,6160.2547)"><path
           style="fill:#91ce99;fill-opacity:1;stroke:#{color_front};stroke-width:6.89517;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           d="m 2056.5728,7663.7229 v -27.6536"
           id="path490"
           sodipodi:nodetypes="cc" /><path
           style="fill:#91ce99;fill-opacity:1;stroke:#{color_front};stroke-width:3.87542;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           d="m 2048.1345,7638.0446 v 23.7031"
           id="path491"
           sodipodi:nodetypes="cc" /><path
           style="fill:#91ce99;fill-o@pacity:1;stroke:#{color_front};stroke-width:6.89517;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           d="m 2073.4496,7663.7229 v -27.6536"
           id="path492"
           sodipodi:nodetypes="cc" /><path
           style="fill:#91ce99;fill-opacity:1;stroke:#{color_front};stroke-width:3.87542;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           d="m 2081.8878,7638.0446 v 23.7031"
           id="path493"
           sodipodi:nodetypes="cc" /><path
           style="fill:#91ce99;fill-opacity:1;stroke:#{color_front};stroke-width:3.87542;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           d="m 2065.0112,7638.0446 v 23.7031"
           id="path494"
           sodipodi:nodetypes="cc" /></g></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn volcano_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1066.7783,-589.34825)"><g
       id="g481"
       transform="matrix(0,-2.3578005,2.3578005,0,-17789.482,5484.4689)"
       inkscape:export-filename="cardgen/lava.svg"
       inkscape:export-xdpi="96"
       inkscape:export-ydpi="96"><circle
         style="fill:#efcfcf;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle480"
         cx="2053.3845"
         cy="8020.1484"
         r="22.754089" /><rect
         style="fill:#f2b7b7;fill-opacity:1;stroke:none;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:1"
         id="rect480"
         width="23.57588"
         height="23.57588"
         x="-7134.8511"
         y="-4230.9272"
         transform="rotate(-135)"
         rx="3.2182515"
         ry="3.2182515" /><circle
         style="fill:#efcfcf;fill-opacity:1;stroke:none;stroke-width:5.77001;stroke-linecap:butt;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
         id="circle481"
         cx="8020.1484"
         cy="-2053.3845"
         r="6.4129257"
         transform="rotate(90)" /></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn mountain_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1176.4665,-652.1566)"><g
       id="g483"
       transform="matrix(0,-2.3578005,2.3578005,0,-17679.794,6022.1133)"
       inkscape:export-filename="cardgen/moutnain.svg"
       inkscape:export-xdpi="96"
       inkscape:export-ydpi="96"><circle
         style="fill:#eeeca7;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle483"
         cx="2254.7739"
         cy="8020.1484"
         r="22.754089" /><rect
         style="fill:#e5e383;fill-opacity:1;stroke:none;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:1"
         id="rect483"
         width="23.57588"
         height="23.57588"
         x="-7277.2554"
         y="-4088.5234"
         transform="rotate(-135)"
         rx="3.2182515"
         ry="3.2182515" /></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn lake_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1176.4647,-526.5457)"><g
       id="g482"
       transform="matrix(0,-2.3578005,2.3578005,0,-17526.374,5896.5024)"><circle
         style="fill:#c3edf1;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle482"
         cx="2254.7739"
         cy="7955.0786"
         r="22.754089" /><path
         id="path482"
         style="fill:none;fill-opacity:1;stroke:#218b95;stroke-width:5.64738;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:0.186128"
         d="m 2255.3587,7967.5627 c -2.5669,-1.1553 -4.2454,-3.4397 -4.4031,-5.9929 -0.1542,-2.5532 1.2289,-4.9873 3.8179,-6.4914 2.5891,-1.5041 3.9757,-3.9382 3.8184,-6.4912 -0.1543,-2.5533 -1.8365,-4.8376 -4.4035,-5.9927"
         inkscape:connector-curvature="0"
         sodipodi:nodetypes="ccscc"
         inkscape:export-filename="/home/mako/work/witching lands/land 2021/water.png"
         inkscape:export-xdpi="588.79651"
         inkscape:export-ydpi="588.79651" /></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn ice_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1060.7978,-472.41335)"><g
       id="g496"
       transform="matrix(0,-2.3578005,2.3578005,0,-17642.041,5367.534)"><circle
         style="fill:#e1eff0;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle495"
         cx="2053.3845"
         cy="7955.0786"
         r="22.754089" /><path
         sodipodi:nodetypes="cc"
         inkscape:connector-curvature="0"
         d="m 2053.3845,7966.9504 v -23.7436"
         style="fill:none;fill-opacity:1;stroke:#f4fcfd;stroke-width:6.32004;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
         id="path495"
         inkscape:export-filename="/home/mako/work/witching lands/land 2021/ice.png"
         inkscape:export-xdpi="588.79651"
         inkscape:export-ydpi="588.79651" /></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn void_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1066.778,-714.96415)"><g
       id="g471"
       transform="matrix(0,-2.3578005,2.3578005,0,-17943.862,5610.0848)"><circle
         style="fill:#969696;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle470"
         cx="2053.3845"
         cy="8085.6245"
         r="22.754089" /><circle
         r="13.942471"
         cy="-4265.438"
         cx="-7169.3618"
         id="circle471"
         style="color:#000000;overflow:visible;fill:#414141;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:7.69773;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1;stop-color:#000000"
         transform="rotate(-135)"
         inkscape:export-filename="/home/mako/work/witching lands/land 2021/hole.png"
         inkscape:export-xdpi="588.79651"
         inkscape:export-ydpi="588.79651" /></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap()
}
pub fn tomb_g(center: V2, scale: f64, to: &mut dyn Write) {
    let offset = center - scale * BIG_ELEMENT_DIMENSIONS / 2.0;
    write!(to,
        r#"<g transform="translate({},{}) scale({})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-1176.4649,-777.7725)"><g
       id="g470"
       transform="matrix(0,-2.3578005,2.3578005,0,-17834.175,6147.7292)"><circle
         style="fill:#ebebeb;fill-opacity:1;stroke-width:6.00001;stroke-linecap:round;stroke-linejoin:round"
         id="circle464"
         cx="2254.7739"
         cy="8085.6245"
         r="22.754089" /><g
         id="g469"
         transform="translate(70.633624,-0.54262281)"><rect
           style="fill:#ebebeb;fill-opacity:1;stroke:#dedede;stroke-width:5.77;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           id="rect464"
           width="29.135572"
           height="29.13588"
           x="4158.7954"
           y="-7276.772"
           rx="14.567786"
           ry="14.567782"
           transform="rotate(135)" /><rect
           style="fill:none;fill-opacity:1;stroke:#dedede;stroke-width:5.76998;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
           id="rect465"
           width="10.431372"
           height="10.431509"
           x="-22.078732"
           y="-8381.1504"
           rx="5.2156858"
           ry="5.2156982"
           transform="rotate(165)" /><g
           id="g466"
           transform="matrix(-0.90017108,0,0,-0.90017108,4291.5704,14929.953)"
           style="stroke-width:1.11089"><path
             style="fill:#969696;fill-opacity:1;stroke:#dedede;stroke-width:6.4099;stroke-linecap:butt;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
             d="m 2341.078,7586.8044 v 10.4299"
             id="path465" /><path
             style="fill:#969696;fill-opacity:1;stroke:#dedede;stroke-width:6.4099;stroke-linecap:butt;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
             d="m 2341.2104,7608.2873 v 10.4299"
             id="path466" /></g><g
           id="g468"
           transform="matrix(0,0.90017108,0.90017108,0,-4659.6456,5978.7373)"
           style="fill:#dedede;fill-opacity:1;stroke-width:1.11089"><path
             style="fill:#dedede;fill-opacity:1;stroke:#dedede;stroke-width:6.4099;stroke-linecap:butt;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
             d="m 2341.078,7586.8044 v 10.4299"
             id="path467" /><path
             style="fill:#dedede;fill-opacity:1;stroke:#dedede;stroke-width:6.4099;stroke-linecap:butt;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
             d="m 2341.2104,7608.2873 v 10.4299"
             id="path468" /></g></g></g></g></g>
"#,
        offset.x, offset.y, scale
    ).unwrap();
}

pub fn end_front_inner(inserting: &impl Display, scores: String, to: &mut dyn Write) {
    write!(to,
r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(0,0)">
    <polygon
       fill="#929497"
       points="144,198 144,0 0,0 0,198 "
       id="assetback"
       transform="matrix(1.1024306,0,0,1.1024306,0,2e-4)"
       style="fill:#f1f2f2;fill-opacity:1;stroke-width:0.24" />
    <path
       id="square"
       style="fill:#f1f2f2;fill-opacity:1;stroke:none"
       d="m 9.92142,69.45312 v 9.92188 119.0625 9.92187 h 9.92239 119.0625 9.92187 V 198.4375 79.375 69.45312 h -9.92187 -119.0625 z" />
    <path
       fill="#ffffff"
       stroke="#ec1e28"
       stroke-width="0.374174"
       d="M 138.9062,208.35959 H 19.8437 c -5.4799,0 -9.9221,-4.4417 -9.9221,-9.9219 V 19.844 c 0,-5.4802 4.4422,-9.9219 9.9221,-9.9219 h 119.0625 c 5.48,0 9.9218,4.4417 9.9218,9.9219 v 178.59369 c 0,5.4802 -4.4418,9.9219 -9.9218,9.9219 z"
       id="cutline"
       style="fill:#f1f2f2;fill-opacity:1;stroke:none" />
    <path
       id="heading"
       style="fill:#4e4e4e;fill-opacity:1;stroke:none;stroke-width:12.3172;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1"
       d="M 0 0.0002 L 0 69.453325 L 67.879061 69.453325 C 69.026866 69.453069 70.1277 68.996781 70.939339 68.185185 L 74.891036 64.229354 L 77.758561 61.357695 C 78.212915 60.949318 78.702724 60.766517 79.375 60.766517 C 80.047275 60.766517 80.537086 60.949318 80.991439 61.357695 L 83.858964 64.229354 L 87.810661 68.185185 C 88.6223 68.996781 89.723134 69.453069 90.870939 69.453325 L 158.75 69.453325 L 158.75 0.0002 L 0 0.0002 z " />
    <text
       xml:space="preserve"
       style="font-style:normal;font-variant:normal;font-weight:500;font-stretch:normal;font-size:49.3895px;line-height:1.25;font-family:Rubik;-inkscape-font-specification:'Rubik Medium';letter-spacing:0px;word-spacing:0px;fill:#eeeeee;fill-opacity:1;stroke:none;stroke-width:1.23474;text-anchor:middle;text-align:center"
       x="79"
       y="53.854977"
       id="pointscore"><tspan
         sodipodi:role="line"
         id="tspan4"
         x="63.172646"
         y="53.854977"
         style="font-style:normal;font-variant:normal;font-weight:500;font-stretch:normal;font-family:Rubik;-inkscape-font-specification:'Rubik Medium';fill:#eeeeee;fill-opacity:1;stroke-width:1.23474;text-anchor:middle;text-align:center">{scores}</tspan></text>
    {inserting}
  </g>"##,
    ).unwrap();
}

#[derive(Clone)]
pub struct CardSpec {
    // likes: Vec<ElementTag>,
    pub name: String,
    pub repeat: usize,
    pub generate_front: Rc<dyn Fn(&mut dyn Write)>,
    pub generate_back: Rc<dyn Fn(&mut dyn Write)>,
}
impl CardSpec {
    pub fn means_card(
        assets: &Rc<Assets>,
        name: String,
        filename: Option<String>,
        // the level of play on which this card should become available
        level: usize,
        front_graphic: Rc<dyn Fn(&mut dyn Write)>,
        back_text: String,
    ) -> Self {
        Self::means_card_repeated(assets, name, filename, 1, level, front_graphic, back_text)
    }
    pub fn means_card_repeated(
        assets: &Rc<Assets>,
        name: String,
        filename: Option<String>,
        repeated: usize,
        // the level of play on which this card should become available
        level: usize,
        front_graphic: Rc<dyn Fn(&mut dyn Write)>,
        back_text: String,
    ) -> Self {
        let filename = if let Some(n) = filename {
            n
        } else {
            name.clone()
        };
        Self {
            name: filename,
            repeat: repeated,
            generate_front: {
                let front_graphic = front_graphic.clone();
                let name = name.clone();
                Rc::new(move |w| means_front(&Displaying(|w| front_graphic(w)), &name, w))
            },
            generate_back: Rc::new({
                let assets = assets.clone();
                move |w| {
                    backing(
                        &assets,
                        &Displaying(|w| front_graphic(w)),
                        w,
                        &back_text,
                        level,
                    );
                }
            }),
        }
    }
    pub fn end_card_with_back_blurred_message(
        assets: &Rc<Assets>,
        name: String,
        front_graphic: Rc<dyn Display>,
        score: String,
        back_text: String,
        level: usize,
    ) -> Self {
        let rcd = Rc::new(front_graphic);
        let sc = score.clone();
        Self {
            name,
            repeat: 1,
            generate_front: {
                let front_inner = rcd.clone();
                Rc::new(move |w| {
                    let scc = sc.clone();
                    end_front_outer(
                        &Displaying(|w| end_front_inner(&front_inner, scc.clone(), w)),
                        w,
                    );
                })
            },
            generate_back: Rc::new({
                let assets = assets.clone();
                let front_inner = rcd.clone();
                move |w| {
                    //you have to clone, because this lambda could be called multiple times, meaning it has to retain something to clone from to create the lambda ahead
                    end_backing(&assets, &front_inner, w, &back_text, level);
                }
            }),
        }
    }
}

pub fn end_front(inserting: &impl Display, scores: String, to: &mut dyn Write) {
    end_front_outer(
        &Displaying(move |w: &mut dyn Write| end_front_inner(inserting, scores.clone(), w)),
        to,
    );
}

pub fn end_front_outer(inserting: &impl Display, to: &mut dyn Write) {
    front_outer(inserting, to);
}
pub fn means_front_outer(inserting: &impl Display, to: &mut dyn Write) {
    front_outer(inserting, to);
}
pub fn front_outer(inserting: &impl Display, to: &mut dyn Write) {
    write!(
        to,
        r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created partially with Inkscape (http://www.inkscape.org/) but primarily through codegen -->

<svg
   width="158.75mm"
   height="218.28127mm"
   viewBox="0 0 158.75 218.28127"
   version="1.1"
   id="svg1"
   inkscape:version="1.3.1 (91b66b0783, 2023-11-16)"
   sodipodi:docname="card front template.svg"
   xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
   xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg">
  <sodipodi:namedview
     id="namedview1"
     pagecolor="#ffffff"
     bordercolor="#000000"
     borderopacity="0.25"
     inkscape:showpageshadow="2"
     inkscape:pageopacity="0.0"
     inkscape:pagecheckerboard="0"
     inkscape:deskcolor="#d1d1d1"
     inkscape:document-units="mm"
     inkscape:zoom="0.64462111"
     inkscape:cx="197.79061"
     inkscape:cy="62.827604"
     inkscape:window-width="1876"
     inkscape:window-height="1032"
     inkscape:window-x="44"
     inkscape:window-y="0"
     inkscape:window-maximized="1"
     inkscape:current-layer="layer1" />
  <defs
     id="defs1" />
  {inserting}
</svg>
"##,
    )
    .unwrap();
}

pub fn end_backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
) {
    backing(assets, inserting, to, description, level);
}
pub fn backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
) {
    let span = CARD_DIMENSIONS.x;
    let level_marker = Displaying(|w| {
        if level >= 2 {
            assets.level_2.by_grav(
                cutline_bounds_shrunk_appropriately().br,
                RIGHT_BOTTOM,
                1.0,
                w,
            );
        }
    });
    write!(to,
r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created partially with Inkscape (http://www.inkscape.org/) but primarily through codegen -->

<svg
   width="158.75mm"
   height="218.28127mm"
   viewBox="0 0 158.75 218.28127"
   version="1.1"
   id="svg1"
   inkscape:version="1.3.1 (91b66b0783, 2023-11-16)"
   sodipodi:docname="card front template.svg"
   xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
   xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg">
  <sodipodi:namedview
     id="namedview1"
     pagecolor="#ffffff"
     bordercolor="#000000"
     borderopacity="0.25"
     inkscape:showpageshadow="2"
     inkscape:pageopacity="0.0"
     inkscape:pagecheckerboard="0"
     inkscape:deskcolor="#d1d1d1"
     inkscape:document-units="mm"
     inkscape:zoom="0.64462111"
     inkscape:cx="197.79061"
     inkscape:cy="62.827604"
     inkscape:window-width="1876"
     inkscape:window-height="1032"
     inkscape:window-x="44"
     inkscape:window-y="0"
     inkscape:window-maximized="1"
     inkscape:current-layer="layer1" />
  <defs
     id="defs1">
    <rect
       x="73.083376"
       y="74.501079"
       width="454.10823"
       height="671.09289"
       id="descriptionrect" />
    <filter
       inkscape:collect="always"
       style="color-interpolation-filters:sRGB"
       id="flipfilter"
       x="-0.056058263"
       y="-0.040769646"
       width="1.1121165"
       height="1.0815393">
      <feGaussianBlur
         inkscape:collect="always"
         stdDeviation="3.7080208"
         id="feGaussianBlur5" />
    </filter>
  </defs>
  <g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(0,0)">
    <polygon
       fill="#929497"
       points="144,198 144,0 0,0 0,198"
       id="assetback"
       transform="matrix(1.1024306,0,0,1.1024306,0,2e-4)"
       style="fill:#f1f2f2;fill-opacity:1;stroke-width:0.24" />
    <g transform="matrix(-1,0,0,1,{span},0)" style="opacity:0.27;filter:url(#flipfilter)">
    {inserting}
    </g>
    <text
       xml:space="preserve"
       transform="matrix(0.26458333,0,0,0.26458333,-0.21640517,0)"
       id="text1"
       style="font-weight:900;font-size:37px;font-family:'Inter UI';-inkscape-font-specification:'Rubik';text-align:center;vertical-align:bottom;white-space:pre;shape-inside:url(#descriptionrect);opacity:1;fill:#3e3e3e;fill-opacity:1;stroke:none;stroke-width:7.55906;stroke-linecap:round;stroke-linejoin:round"><tspan
         x="93.067162"
         y="126.73272"
         id="tspan3"><tspan
           style="font-weight:normal;font-family:Rubik;-inkscape-font-specification:Rubik"
           id="tspan2">{description}</tspan></tspan></text>
    {level_marker}
  </g>
</svg>
"##,
    ).unwrap();
}

pub fn just_1(color: &str, to: &mut dyn Write) {
    let scale = 1.5;
    let offset = offset_for_grav(
        END_GRAPHIC_CENTER - V2::new(0.0, 0.23 * GRAPHIC_RAD),
        MIDDLE_BOTTOM,
        V2::new(27.831, 27.318) * scale,
    );
    write!(to,
        r##"<g transform="translate({},{}) scale({scale})"><g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(-38.099981,-226.48331)">
    <text
       xml:space="preserve"
       style="font-style:normal;font-variant:normal;font-weight:500;font-stretch:normal;font-size:39.0255px;line-height:1.25;font-family:Rubik;-inkscape-font-specification:'Rubik Medium';letter-spacing:0px;word-spacing:0px;fill:#7eb47f;fill-opacity:1;stroke:none;stroke-width:0.975639"
       x="37.481148"
       y="253.80116"
       id="text20"><tspan
         sodipodi:role="line"
         id="tspan20"
         x="37.481148"
         y="253.80116"
         style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-family:Rubik;-inkscape-font-specification:'Rubik Bold';fill:#{color};fill-opacity:1;stroke-width:0.975639">1!</tspan></text>
  </g></g>"##,
        offset.x, offset.y
    ).unwrap()
}

pub fn big_splat(color: &str, to: &mut dyn Write) {
    let scale = 0.54;
    let offset = offset_for_grav_scale(
        END_GRAPHIC_CENTER,
        MIDDLE_MIDDLE,
        V2::new(205.184, 224.671),
        scale,
    );
    write!(to,
        r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     transform="translate({},{}) scale({scale})"
     id="layer1">
    <path
       id="path22"
       style="color:#{color};fill:#{color};stroke-linecap:round;stroke-linejoin:round;-inkscape-stroke:none"
       d="M 102.8227,8.5461488e-5 A 40.004,40.004 0 0 0 64.00646,30.002395 c -0.71838,2.78675 -2.50075,5.11504 -4.95887,6.59753 -2.51216,1.36097 -5.40826,1.71961 -8.16436,0.93947 A 40.004,40.004 0 0 0 11.99258,104.59578 c 2.09677,2.0548 3.25951,4.8237 3.27163,7.7582 -0.0101,2.9307 -1.16703,5.6959 -3.25768,7.7515 a 40.004,40.004 0 0 0 38.89272,67.0217 c 2.75377,-0.7759 5.64309,-0.413 8.15195,0.9473 2.45565,1.4827 4.2374,3.8089 4.95526,6.5939 a 40.004,40.004 0 0 0 77.50794,-0.1582 c 0.68988,-2.7224 2.4144,-4.9928 4.77542,-6.4905 2.47863,-1.2958 5.30812,-1.6541 8.0114,-0.8889 a 40.004,40.004 0 0 0 38.89065,-67.0563 c -2.09243,-2.0505 -3.25055,-4.8104 -3.2675,-7.7381 0.017,-2.9276 1.17509,-5.6901 3.2675,-7.7406 A 40.004,40.004 0 0 0 154.30122,37.541465 c -2.70333,0.76523 -5.53275,0.40501 -8.0114,-0.8909 -2.36079,-1.49771 -4.08557,-3.76625 -4.77542,-6.48849 A 40.004,40.004 0 0 0 102.8227,8.5461488e-5 Z" />
  </g>"##,
        offset.x, offset.y
    ).unwrap()
}

pub fn negatory(to: &mut dyn Write) {
    // let scale = 0.54;
    let offset = offset_for_grav(END_GRAPHIC_CENTER, MIDDLE_MIDDLE, V2::new(122.431, 78.813));
    write!(to,
        r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{})">
    <rect
       transform="matrix(0.86602609,-0.49999881,0.5000012,0.86602471,0,0)"
       ry="1.9218427"
       rx="1.9218458"
       y="57.18195"
       x="-33.824398"
       height="15.105367"
       width="134.27014"
       id="rect26"
       style="fill:#4b4b4b;fill-opacity:1;stroke:none;stroke-width:6.42757;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" />
  </g>"##,
        offset.x, offset.y
    ).unwrap();
}

pub fn tilted_pair(center: V2, distance: f64) -> (V2, V2) {
    //left-bottom, right-top
    let tilt = -TAU / 12.0;
    let outv = from_angle_mag(tilt, distance);
    let c1 = center - outv;
    let c2 = center + outv;
    (c1, c2)
}

pub fn paired(e1: ElementTag, e2: ElementTag, flipped: bool, to: &mut dyn Write) {
    let sized = 0.6;
    let spaced = 0.04;
    let (mut c1, mut c2) = tilted_pair(END_GRAPHIC_CENTER, (sized + spaced) * BIG_ELEMENT_RAD);
    if flipped {
        std::mem::swap(&mut c1.y, &mut c2.y);
    }
    ELEMENT_G[e1](c1, sized, to);
    ELEMENT_G[e2](c2, sized, to);
}

//generalizable util stuff
#[derive(Clone)]
pub struct Rect {
    pub ul: V2,
    pub br: V2,
}
impl Rect {
    pub fn from_center_radii(center: V2, radii: V2) -> Self {
        Self {
            ul: center - radii,
            br: center + radii,
        }
    }
    pub fn width(&self) -> f64 {
        self.br.x - self.ul.x
    }
    pub fn height(&self) -> f64 {
        self.br.y - self.ul.y
    }
    pub fn center(&self) -> V2 {
        (self.ul + self.br) / 2.0
    }
    pub fn span(&self) -> V2 {
        self.br - self.ul
    }
    pub fn reduced_by(&self, margin: f64) -> Rect {
        let mars = V2::new(margin, margin);
        Rect {
            ul: self.ul + mars,
            br: self.br - mars,
        }
    }
    ///specifically it's reduced by a proportion of the smallest dimension
    pub fn shrunk(&self, to_proportion: f64) -> Rect {
        self.reduced_by(self.span().min() * (1.0 - to_proportion) / 2.0)
    }
}

pub fn cross<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |ai| {
        b.clone().map({
            let ai = ai.clone();
            move |bs| (ai.clone(), bs)
        })
    })
}

#[derive(Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl Coord {
    pub const DWARD: Coord = Coord { x: 1, y: 0 };
    pub const EWARD: Coord = Coord { x: 0, y: 1 };
    pub const WWARD: Coord = Coord { x: -1, y: 1 };
    pub const AWARD: Coord = Coord { x: -1, y: 0 };
    pub const ZWARD: Coord = Coord { x: 0, y: -1 };
    pub const XWARD: Coord = Coord { x: 1, y: -1 };
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
    pub fn to_v2(self) -> V2 {
        V2::new(self.x as f64, self.y as f64)
    }
}

pub struct HexSpiral {
    pub layer: u32,
    pub leg: u32,
    pub progress: u32,
    pub x: i32,
    pub y: i32,
}

impl HexSpiral {
    pub fn new() -> HexSpiral {
        HexSpiral {
            layer: 0,
            leg: 1,
            progress: 0,
            x: 0,
            y: 0,
        }
    }
    pub fn step(&mut self) {
        if self.layer == 0 {
            self.y += 1;
            self.layer = 1;
        } else {
            match self.leg {
                0 => {
                    self.y += 1;
                    self.x -= 1;
                }
                1 => {
                    self.x -= 1;
                }
                2 => {
                    self.y -= 1;
                }
                3 => {
                    self.x += 1;
                    self.y -= 1;
                }
                4 => {
                    self.x += 1;
                }
                _ => {
                    self.y += 1;
                }
            }
            self.progress += 1;
            if self.leg >= 5 {
                if self.progress > self.layer {
                    self.layer += 1;
                    self.leg = 0;
                    self.progress = 1;
                }
            } else {
                if self.progress == self.layer {
                    self.leg += 1;
                    self.progress = 0;
                }
            }
        }
    }

    pub fn current_hex_coord(&self) -> Coord {
        Coord::new(self.x, self.y)
    }
    pub fn layer_iter(self, layers: usize) -> HexLayerIter {
        HexLayerIter(self, layers)
    }
}
pub struct HexLayerIter(HexSpiral, usize);
impl Iterator for HexLayerIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.layer as usize > self.1 {
            None
        } else {
            self.0.next()
        }
    }
}

impl Iterator for HexSpiral {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let r = Some(self.current_hex_coord());
        self.step();
        r
    }
}

pub fn hexify(v: V2) -> V2 {
    V2::new(v.x + v.y / 2.0, v.y * (3.0 / 4.0 as f64).sqrt())
} //from square to hex
pub fn unhexify(v: V2) -> V2 {
    V2::new(
        v.x - v.y / (3.0 as f64).sqrt(),
        v.y / (3.0 / 4.0 as f64).sqrt(),
    )
} //from hex to square

//end generalizable util stuff

pub fn end_graphic_usual_bounds() -> Rect {
    Rect::from_center_radii(END_GRAPHIC_CENTER, V2::from_element(GRAPHIC_RAD))
}
pub fn end_graphic_usual_bounds_shrunk_appropriately() -> Rect {
    end_graphic_usual_bounds().shrunk(0.8)
}
pub fn cutline_bounds_shrunk_appropriately() -> Rect {
    cutline_bounds().shrunk(0.83)
}
pub fn cutline_bounds() -> Rect {
    Rect {
        ul: CUTLINE_INSET,
        br: CARD_DIMENSIONS - CUTLINE_INSET,
    }
}
pub fn means_graphic_usual_bounds() -> Rect {
    let ul = V2::new(9.922, 9.922);
    Rect {
        ul,
        br: ul + V2::new(138.906, 162.991),
    }
}
pub fn means_graphic_usual_bounds_shrunk_appropriately() -> Rect {
    means_graphic_usual_bounds().shrunk(0.8)
}
pub fn card_upper_center() -> V2 {
    V2::new(CARD_DIMENSIONS.x / 2.0, CARD_DIMENSIONS.x / 2.0)
}
pub fn card_lower_center() -> V2 {
    V2::new(
        CARD_DIMENSIONS.x / 2.0,
        CARD_DIMENSIONS.y - CARD_DIMENSIONS.x / 2.0,
    )
}

// //wait, this would break if you had nested svg elements. Fuck streaming parsers.
// fn parse_extract(at:&std::path::Path)-> Result<(String, V2), Box<dyn Error>> {
//     use quick_xml::events::{Event, BytesEnd, BytesStart};
//     use quick_xml::reader::Reader;
//     use quick_xml::writer::Writer;
//     use std::io::Cursor;
//     let xml = r#"<this_tag k1="v1" k2="v2"><child>text</child></this_tag>"#;
//     let mut reader = Reader::from_str(xml);
//     reader.trim_text(true);
//     let mut writer = Writer::new(Cursor::new(Vec::new()));
//     let mut bounds:Option<V2> = None;

//     'outer: loop {
//         if let Event::Start(e) = reader.read_event()? {
//             if &e.name().as_ref() == "svg" {
//                 writer.write_event(e);

//                 loop {
//                     let e = reader.read_event()?;
//                     writer.write_event(e);
//                     match e {
//                         Event::End(ee) if ee.name().as_ref() == "svg" => { break; }
//                         Event::Eof=> {break 'outer;}
//                         _=> {}
//                     }
//                 }
//             }
//         }
//     }

//     Ok((String::from_utf8(writer.into_inner().into_inner())?, bounds.ok_or("couldn't find width and height".to_string())?))
// }

pub fn assume_writes_utf8(f: impl Fn(&mut dyn Write)) -> String {
    let mut w = Vec::<u8>::new();
    f(&mut w);
    String::from_utf8(w).unwrap()
}

#[derive(Clone)]
pub struct Asset {
    pub render: Rc<dyn Fn(V2, f64, f64, &mut dyn Write)>,
    pub bounds: V2,
}
impl Asset {
    pub fn center_in_bounds(&self, bounds: Rect, to: &mut dyn Write) {
        let scale = (bounds.width() / self.bounds.x).min(bounds.height() / self.bounds.y);
        self.by_grav(bounds.center(), MIDDLE_MIDDLE, scale, to);
    }
    pub fn centered(&self, at: V2, scale: f64, to: &mut dyn Write) {
        self.by_grav(at, MIDDLE_MIDDLE, scale, to);
    }
    pub fn centered_rad(&self, at: V2, rad: f64, to: &mut dyn Write) {
        let scale = rad / (self.bounds.min() / 2.0);
        self.by_grav(at, MIDDLE_MIDDLE, scale, to);
    }
    pub fn centered_rotated(&self, at: V2, scale: f64, rotation: f64, to: &mut dyn Write) {
        self.by_grav_rotated(at, MIDDLE_MIDDLE, scale, rotation, to);
    }
    pub fn centered_rotr(&self, at: V2, rad: f64, rotation: f64, to: &mut dyn Write) {
        let scale = rad / (self.bounds.min() / 2.0);
        self.by_grav_rotated(at, MIDDLE_MIDDLE, scale, rotation, to);
    }
    pub fn by_grav(&self, anchor: V2, grav: Gravity, scale: f64, to: &mut dyn Write) {
        self.by_ul(
            offset_for_grav(anchor, grav, self.bounds * scale),
            scale,
            to,
        );
    }
    pub fn by_grav_rotated(
        &self,
        anchor: V2,
        grav: Gravity,
        scale: f64,
        rotation: f64,
        to: &mut dyn Write,
    ) {
        self.by_anchor_rotated(
            anchor,
            anchor_for_grav(grav, self.bounds),
            scale,
            rotation,
            to,
        );
    }
    pub fn by_anchor(
        &self,
        anchor_screenspace: V2,
        anchor_within: V2,
        scale: f64,
        to: &mut dyn Write,
    ) {
        let ul = anchor_screenspace - anchor_within * scale;
        self.by_ul(ul, scale, to);
    }
    pub fn by_anchor_rotated(
        &self,
        anchor_screenspace: V2,
        anchor_within: V2,
        scale: f64,
        rotation: f64,
        to: &mut dyn Write,
    ) {
        let rotatedawi = rotate(from_angle(rotation), anchor_within);
        let ul = anchor_screenspace - rotatedawi * scale;
        (self.render)(ul, scale, rotation / TAU * 360.0, to);
    }
    pub fn by_ul(&self, ul: V2, scale: f64, to: &mut dyn Write) {
        (self.render)(ul, scale, 0.0, to);
    }
}

pub fn load_asset(at: &Path) -> Asset {
    // pub fn for_asset(at: &std::path::Path) -> Rc<dyn Display> {
    let assetxml = elementtree::Element::from_reader(&std::fs::File::open(at).unwrap()).unwrap();
    //lol, turns out the comment isn't an element so the entire document is just the root element (what if a document contains multiple root elements? Is that not allowed?)
    let svgel = &assetxml;
    fn ignore_unit(v: &str) -> &str {
        v.split_at(v.len() - 2).0
    }
    let bounds = V2::new(
        str::parse(ignore_unit(&svgel.get_attr("width").unwrap())).unwrap(),
        str::parse(ignore_unit(&svgel.get_attr("height").unwrap())).unwrap(),
    );
    // for se in assetxml.children() { println!("{}", se.tag() ) }
    if let Some(defel) = svgel.find("{http://www.w3.org/2000/svg}defs") {
        if defel.child_count() != 0 {
            println!(
                "warning: there were defs in {:?}, we don't handle those",
                at
            );
        }
    }
    let graphicel = svgel.find("{http://www.w3.org/2000/svg}g").unwrap();
    //scale proportionally to fit
    // let placement_bounds = end_graphic_usual_bounds().reduced_by(0.3 * GRAPHIC_RAD);
    // let scale = (placement_bounds.width() / bounds.x).min(placement_bounds.height() / bounds.y);
    // let scale = (placement_bounds.width() / CUTLINE_INSET.x).min(placement_bounds.height() / bounds.y);
    // let offset = placement_bounds.ul + (1.0 - scale) * bounds / 2.0;

    // let offset = placement_bounds.ul + placement_bounds.span() / 2.0 - (bounds * scale) / 2.0;

    let graphic_str = assume_writes_utf8(|w| {
        graphicel
            .to_writer_with_options(w, WriteOptions::new().set_xml_prolog(None))
            .unwrap()
    });
    Asset {
        render: Rc::new(
            move |ul: V2, scale: f64, rotation: f64, to: &mut dyn Write| {
                write!(
                to,
                r##"<g transform="translate({},{}) scale({scale}) rotate({rotation})">{graphic_str}</g>"##,
                ul.x, ul.y
            )
            .unwrap();
            },
        ),
        bounds,
    }
}

//used to use macros here but macros in rust are just so shit
pub struct Assets {
    pub kill: Asset,
    pub negatory: Asset,
    pub level_2: Asset,
    pub guy: Asset,
    pub guyeye: Asset,
    pub dead_guy: Asset,
    pub altruism: Asset,
    pub field: Asset,
    pub forest: Asset,
    pub mountain: Asset,
    pub volcano: Asset,
    pub lake: Asset,
    pub ice: Asset,
    pub tomb: Asset,
    pub void: Asset,
    pub blank: Asset,
    pub come_on_down: Asset,
    pub back_colored_circle: Asset,
    pub step: Asset,
    pub dog_altruism: Asset,

    pub field_forest: Asset,
    pub mountain_volcano: Asset,
    pub lake_ice: Asset,
    pub tomb_void: Asset,

    pub flip_field: Asset,
    pub flip_forest: Asset,
    pub flip_mountain: Asset,
    pub flip_volcano: Asset,
    pub flip_lake: Asset,
    pub flip_ice: Asset,
    pub flip_tomb: Asset,
    pub flip_void: Asset,
}

fn flip_asset_for(to: &Asset, e: ElementTag) -> Asset {
    let b = both_dims(FLIP_RINGS_SPAN);
    Asset {
        bounds: b,
        render: Rc::new({
            let to = to.clone();
            move |p, s, rotation, w| {
                flip_rings(
                    ELEMENT_COLORS_BACK[e],
                    ELEMENT_COLORS_BACK[opposite_element(e)],
                    &Displaying(|w| to.by_ul(both_dims(FLIP_RINGS_RAD - BIG_ELEMENT_RAD), 1.0, w)),
                    // p + both_dims(FLIP_RINGS_RAD),
                    p + both_dims(FLIP_RINGS_RAD) * s,
                    s,
                    rotation,
                    w,
                )
            }
        }),
    }
}

fn generate_either(e1: &Asset, e2: &Asset) -> Asset {
    Asset {
        bounds: e1.bounds,
        render: Rc::new({
            let e1 = e1.clone();
            let e2 = e2.clone();
            move |p, s, rotation, w| {
                write!(w, r##"
<g transform="translate({},{}) scale({s}) rotate({rotation})">
<defs id="eitherdefs">
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath68">
      <path
         id="path69"
         style="fill:#e4afaf;fill-opacity:1;stroke-width:9.92476;stroke-linecap:round;stroke-linejoin:round"
         d="m 2281.1109,7898.2515 v 29.4859 h -26.337 -26.337 v -29.4859 z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath67">
      <path
         id="path68"
         style="fill:#e4afaf;fill-opacity:1;stroke-width:9.92476;stroke-linecap:round;stroke-linejoin:round"
         d="m 2079.7215,7898.2515 v 29.4859 h -26.337 -26.337 v -29.4859 z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
<g clip-path="url(#clipPath68)">{}</g>
<g clip-path="url(#clipPath67)">{}</g>
</g>"##,
                    p.x, p.y,
                    &Displaying(|w| e1.centered(both_dims(0.0), 1.0, w)),
                    &Displaying(|w| e2.centered(both_dims(0.0), 1.0, w)),
                ).unwrap();
            }
        }),
    }
}

impl Assets {
    pub fn load(assets_dir: &Path) -> Self {
        let kill = load_asset(&Path::new("assets/kill.svg"));
        let negatory = load_asset(&Path::new("assets/negatory.svg"));
        let level_2 = load_asset(&Path::new("assets/level_2.svg"));
        let guy = load_asset(&Path::new("assets/guy.svg"));
        let guyeye = load_asset(&Path::new("assets/guyeye.svg"));
        let dead_guy = load_asset(&Path::new("assets/dead_guy.svg"));
        let altruism = load_asset(&Path::new("assets/altruism.svg"));
        let field = load_asset(&Path::new("assets/field.svg"));
        let forest = load_asset(&Path::new("assets/forest.svg"));
        let mountain = load_asset(&Path::new("assets/mountain.svg"));
        let volcano = load_asset(&Path::new("assets/volcano.svg"));
        let lake = load_asset(&Path::new("assets/lake.svg"));
        let ice = load_asset(&Path::new("assets/ice.svg"));
        let tomb = load_asset(&Path::new("assets/tomb.svg"));
        let void = load_asset(&Path::new("assets/void.svg"));
        let blank = load_asset(&Path::new("assets/blank.svg"));
        let come_on_down = load_asset(&Path::new("assets/come_on_down.svg"));
        let back_colored_circle = load_asset(&Path::new("assets/back_colored_circle.svg"));
        let step = load_asset(&Path::new("assets/step.svg"));
        let dog_altruism = load_asset(&Path::new("assets/dog_altruism.svg"));

        let flip_field = flip_asset_for(&field, FIELD_I);
        let flip_forest = flip_asset_for(&forest, FOREST_I);
        let flip_mountain = flip_asset_for(&mountain, MOUNTAIN_I);
        let flip_volcano = flip_asset_for(&volcano, VOLCANO_I);
        let flip_lake = flip_asset_for(&lake, LAKE_I);
        let flip_ice = flip_asset_for(&ice, ICE_I);
        let flip_tomb = flip_asset_for(&tomb, TOMB_I);
        let flip_void = flip_asset_for(&void, VOID_I);

        let field_forest = generate_either(&field, &forest);
        let mountain_volcano = generate_either(&mountain, &volcano);
        let lake_ice = generate_either(&lake, &ice);
        let tomb_void = generate_either(&tomb, &void);

        Self {
            kill,
            negatory,
            level_2,
            guy,
            guyeye,
            dead_guy,
            altruism,
            field,
            forest,
            mountain,
            volcano,
            lake,
            ice,
            tomb,
            void,
            blank,
            come_on_down,
            back_colored_circle,
            step,
            dog_altruism,
            flip_field,
            flip_forest,
            flip_mountain,
            flip_volcano,
            flip_lake,
            flip_ice,
            flip_tomb,
            flip_void,
            field_forest,
            mountain_volcano,
            lake_ice,
            tomb_void,
        }
    }
    pub fn element(&self, e: ElementTag) -> &Asset {
        match e {
            FIELD_I => &self.field,
            FOREST_I => &self.forest,
            MOUNTAIN_I => &self.mountain,
            VOLCANO_I => &self.volcano,
            LAKE_I => &self.lake,
            ICE_I => &self.ice,
            TOMB_I => &self.tomb,
            VOID_I => &self.void,
            _ => panic!("no such element as {e}"),
        }
    }
    pub fn element_both(&self, e: ElementTag) -> &Asset {
        match e {
            FIELD_I => &self.field_forest,
            MOUNTAIN_I => &self.mountain_volcano,
            LAKE_I => &self.lake_ice,
            TOMB_I => &self.tomb_void,
            _ => panic!(
                "{} is an invalid tag for a pair of elements",
                ELEMENT_NAMES[e]
            ),
        }
    }
    pub fn flip_to(&self, e: ElementTag) -> &Asset {
        match e {
            FIELD_I => &self.flip_field,
            FOREST_I => &self.flip_forest,
            LAKE_I => &self.flip_lake,
            ICE_I => &self.flip_ice,
            TOMB_I => &self.flip_tomb,
            VOID_I => &self.flip_void,
            MOUNTAIN_I => &self.flip_mountain,
            VOLCANO_I => &self.flip_volcano,
            _ => panic!("{e} is not an element tag"),
        }
    }
}

pub fn for_asset(at: PathBuf) -> Displaying<impl Fn(&mut dyn Write)> {
    Displaying(move |w: &mut dyn Write| {
        load_asset(&at).center_in_bounds(end_graphic_usual_bounds(), w)
    })
}

//end ends stuff, begin means

pub fn means_backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
) {
    backing(assets, inserting, to, description, level);
}

pub fn guylike(asset: &Asset, base_centered: V2, scale: f64, to: &mut dyn Write) {
    let bx = asset.bounds.x / 2.0;
    asset.by_anchor(base_centered, V2::new(bx, asset.bounds.y - bx), scale, to);
}

pub fn blank_front(inserting: &impl Display, color: &str, rotate: bool, to: &mut dyn Write) {
    card_front_outer(inserting, "", color, rotate, to);
}
pub fn means_front(inserting: &impl Display, name: &str, to: &mut dyn Write) {
    let background_color = "f1f2f2";
    card_front_outer(inserting, name, background_color, false, to);
}
pub fn card_front_outer(
    inserting: &impl Display,
    name: &str,
    background_color: &str,
    rotate: bool,
    to: &mut dyn Write,
) {
    let rotation = if rotate { "90" } else { "0" };
    write!(to, r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created partially with Inkscape (http://www.inkscape.org/) but primarily through codegen -->

<svg
   width="158.75mm"
   height="218.28127mm"
   viewBox="0 0 158.75 218.28127"
   version="1.1"
   id="svg1"
   inkscape:version="1.3.1 (91b66b0783, 2023-11-16)"
   sodipodi:docname="means front template.svg"
   xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
   xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg">
  <sodipodi:namedview
     id="namedview1"
     pagecolor="#ffffff"
     bordercolor="#000000"
     borderopacity="0.25"
     inkscape:showpageshadow="2"
     inkscape:pageopacity="0.0"
     inkscape:pagecheckerboard="0"
     inkscape:deskcolor="#d1d1d1"
     inkscape:document-units="mm"
     inkscape:zoom="0.49829275"
     inkscape:cx="60.205571"
     inkscape:cy="601.05229"
     inkscape:window-width="1876"
     inkscape:window-height="1032"
     inkscape:window-x="44"
     inkscape:window-y="0"
     inkscape:window-maximized="1"
     inkscape:current-layer="g8" />
  <defs
     id="defs1">
    <rect
       x="40.66116"
       y="678.98822"
       width="512.60158"
       height="111.65349"
       id="rect10" />
    <rect
       x="442.54103"
       y="764.68524"
       width="32.549689"
       height="18.533424"
       id="rect9" />
  </defs>
  <g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="g8"
     transform="translate(0,0)">
    <polygon
       fill="#929497"
       points="144,198 144,0 0,0 0,198 "
       id="assetback"
       transform="matrix(1.1024306,0,0,1.1024306,0,2e-4)"
       style="fill:#{background_color};fill-opacity:1;stroke-width:0.24" />
    <path
       fill="#ffffff"
       stroke="#ec1e28"
       stroke-width="0.374174"
       d="M 138.9062,208.35959 H 19.8437 c -5.4799,0 -9.9221,-4.4417 -9.9221,-9.9219 V 19.844 c 0,-5.4802 4.4422,-9.9219 9.9221,-9.9219 h 119.0625 c 5.48,0 9.9218,4.4417 9.9218,9.9219 v 178.59369 c 0,5.4802 -4.4418,9.9219 -9.9218,9.9219 z"
       id="cutline"
       style="fill:#{background_color};fill-opacity:1;stroke:none" />
    <g
       transform="translate(84.75197610056054,103.4528234964666) scale(0.5)"
       id="g1">
      <g
         inkscape:label="Layer 1"
         inkscape:groupmode="layer"
         id="layer1"
         transform="translate(-1066.7783,-589.34825)" />
    </g>
    <g transform="rotate({rotation},79.375003,109.14083)">
    {inserting}
    </g>
    <text
       xml:space="preserve"
       transform="matrix(0.26458333,0,0,0.26458333,0.55598493,-3.6076306)"
       id="text10"
       style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:53.3333px;line-height:1.05;font-family:Rubik;-inkscape-font-specification:Rubik;text-align:center;white-space:pre;shape-inside:url(#rect10);display:inline;fill:#757575;fill-opacity:1;stroke:none;stroke-width:7.55906;stroke-linecap:round;stroke-linejoin:round"><tspan
         x="84.767989"
         y="722.40316"
         id="tspan3">{name}</tspan></text>
  </g>
</svg>"##).unwrap();
}

pub fn flip_rings(
    to_color: &str,
    from_color: &str,
    element_graphic: &Displaying<impl Fn(&mut dyn Write)>,
    center: V2,
    scale: f64,
    rotation: f64,
    to: &mut dyn Write,
) {
    let offset = center - rotate(from_angle(rotation), both_dims(FLIP_RINGS_RAD)) * scale;
    write!(to, r##"
        <g
        inkscape:label="Layer 1"
        inkscape:groupmode="layer"
        id="layer1"
        transform="translate({},{}) scale({}) rotate({})">
        <circle
        style="fill:#{to_color};stroke-width:2;stroke-linecap:round;stroke-linejoin:round"
        id="path1"
        cx="57.828403"
        cy="57.828403"
        r="57.828403" />
        {element_graphic}
        <path
        id="circle1"
        style="fill:#{from_color};stroke-width:2;stroke-linecap:round;stroke-linejoin:round;fill-opacity:1"
        d="m 0,57.828512 c -5.7990107e-5,31.937829 25.890683,57.828568 57.828512,57.828508 31.937827,6e-5 57.828568,-25.890681 57.828508,-57.828508 H 102.70071 C 102.70061,82.610703 82.610703,102.70061 57.828512,102.70071 33.04612,102.7009 12.955894,82.610904 12.955798,57.828512 Z"
        sodipodi:nodetypes="ccccccc" />
    </g>"##, offset.x, offset.y, scale, rotation/TAU*360.0).unwrap();
}

pub const FLIP_RINGS_SPAN: f64 = 115.65681;
pub const FLIP_RINGS_RAD: f64 = FLIP_RINGS_SPAN / 2.0;

pub fn flipping_to(assets: &Assets, e: ElementTag, center: V2, scale: f64, w: &mut dyn Write) {
    let eo = opposite_element(e);
    let to_color = ELEMENT_COLORS_BACK[e];
    let from_color = ELEMENT_COLORS_BACK[eo];
    let element_graphic = {
        Displaying(|w| {
            assets
                .element(e)
                .by_grav(both_dims(FLIP_RINGS_RAD), MIDDLE_MIDDLE, 1.0, w)
        })
    };
    flip_rings(
        to_color,
        from_color,
        &element_graphic,
        center,
        scale,
        0.0,
        w,
    );
}

pub fn dual_color_patch(
    assets: &Assets,
    e1: ElementTag,
    e2: ElementTag,
    bounds: Rect,
    w: &mut dyn Write,
) {
    let color_left = ELEMENT_COLORS_BACK[e1];
    let color_right = ELEMENT_COLORS_BACK[e2];
    let splat_span = V2::new(205.18423, 224.67136);
    let scale = bounds.span().component_div(&splat_span).min() * 0.78;
    let offset = bounds.center() - scale * splat_span / 2.0;
    let (c1, c2) = tilted_pair(splat_span / 2.0, splat_span.x * 0.17);
    let e1d = Displaying(|w| assets.element(e1).by_grav(c1, MIDDLE_MIDDLE, 0.6, w));
    let e2d = Displaying(|w| assets.element(e2).by_grav(c2, MIDDLE_MIDDLE, 0.6, w));

    write!(
        w,
        r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{}) scale({})">
    <path
       id="path27"
       style="color:#000000;fill:#{color_right};fill-opacity:1;stroke-linecap:round;stroke-linejoin:round;-inkscape-stroke:none"
       d="m 163.76833,188.61458 a 40.004,40.004 0 0 0 29.42353,-68.539 c -2.09677,-2.0548 -3.25951,-4.8237 -3.27163,-7.7582 0.0101,-2.9307 1.16703,-5.6958 3.25768,-7.7514 a 40.004,40.004 0 0 0 -38.89272,-67.0218 c -2.75377,0.7759 -5.6431,0.4131 -8.15196,-0.9472 -2.45564,-1.4827 -4.23739,-3.8089 -4.95525,-6.5939 a 40.004,40.004 0 0 0 -77.50793,0.1581 c -0.68976,2.7219 -2.414,4.9918 -4.77439,6.4895 l 87.38692,151.3588 c 2.4791,-1.2988 5.31406,-1.643 8.01863,-0.8775 a 40.004,40.004 0 0 0 9.46712,1.4826 z" />
    <path
       id="path28"
       style="color:#000000;fill:#{color_left};fill-opacity:1;stroke-linecap:round;stroke-linejoin:round;-inkscape-stroke:none"
       d="m 102.36175,224.67128 a 40.004,40.004 0 0 0 38.81623,-30.0023 c 0.71838,-2.7867 2.50075,-5.115 4.95887,-6.5975 0.047,-0.025 0.0985,-0.037 0.14573,-0.062 L 58.89566,36.65068 c -4.1e-4,3e-4 -6.2e-4,8e-4 -10e-4,0 -2.47863,1.2958 -5.30812,1.654 -8.0114,0.8888 a 40.004,40.004 0 0 0 -38.89065,67.0564 c 2.09243,2.0505 3.25056,4.8103 3.26751,7.738 -0.017,2.9276 -1.1751,5.6901 -3.26751,7.7406 a 40.004,40.004 0 0 0 38.89065,67.0543 c 2.70333,-0.7652 5.53275,-0.405 8.0114,0.8909 2.36079,1.4977 4.08557,3.7663 4.77542,6.4885 a 40.004,40.004 0 0 0 38.6917,30.162 z" />
    {e1d}
    {e2d}
  </g>"##, offset.x, offset.y, scale,
    ).unwrap();
}

pub fn come_on_down(assets: &Assets, e: ElementTag, bounds: Rect, to: &mut dyn Write) {
    let ea = assets.element(e);
    come_on_down_specifically(ea, ea, e, bounds, to);
}
pub fn come_on_down_specifically(
    left_asset: &Asset,
    right_asset: &Asset,
    ef: ElementTag,
    bounds: Rect,
    to: &mut dyn Write,
) {
    let ec = ELEMENT_COLORS_BACK[ef];
    let er = bounds.span().x * 0.19;
    let escale = er / BIG_ELEMENT_RAD;
    let sepx = er * 1.26;
    let sepy = er * 0.8;
    let hsep = sepx * 2.0;
    let cc = bounds.center() + V2::new(0.0, er * 0.3);
    let c1off = V2::new(-sepx, -sepy);
    let c1 = cc + c1off;
    let c2 = cc - c1off;
    let codspan = 67.3;
    let codscale = hsep / codspan;
    let codanchor = V2::new(9.29, 41.64) * codscale;
    let codoffset = V2::new(c1.x - codanchor.x, c1.y - er * 0.6 - codanchor.y);
    write!(to, r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{}) scale({codscale})">
    <path
       id="path50"
       style="color:#000000;fill:#{ec};stroke-linecap:round;stroke-linejoin:round;-inkscape-stroke:none"
       d="M 38.159945 0.0098185221 C 37.498511 0.023905038 36.837434 0.052318903 36.177637 0.0956014 C 28.260067 0.61499139 20.508659 3.2482202 14.083895 8.3576294 C 5.517543 15.170175 -1.2533172e-15 26.47775 0 40.095744 A 9.4499998 9.4499998 0 0 0 9.4490356 49.546847 A 9.4499998 9.4499998 0 0 0 18.900138 40.095744 C 18.900138 31.181933 21.795547 26.370862 25.847518 23.148458 C 29.89949 19.926055 35.796642 18.384631 42.089937 19.060335 C 54.676529 20.411744 67.298735 29.295611 67.298735 48.312297 A 9.4499998 9.4499998 0 0 0 67.402087 49.098295 L 55.604875 49.098295 L 76.933289 65.405827 L 98.261702 49.098295 L 86.158565 49.098295 A 9.4499998 9.4499998 0 0 0 86.197323 48.312297 C 86.197323 20.283577 65.169656 2.5291176 44.107385 0.26768392 C 42.132797 0.055674506 40.144244 -0.032441026 38.159945 0.0098185221 z " />
  </g>"##, codoffset.x, codoffset.y).unwrap();
    left_asset.centered(c1, escale, to);
    right_asset.centered(c2, escale, to);
}

pub fn overplace(
    blank_circle: &Asset,
    placing: &Asset,
    over: &Asset,
    bounds: Rect,
    to: &mut dyn Write,
) {
    let pr = placing.bounds.x / 2.0;
    let tr = over.bounds.x / 2.0;
    let sep = pr * 1.5;
    let tsp = pr + sep + tr;
    let tsph = tsp / 2.0;
    let scale = bounds.span().y * 0.8 / tsp;
    let center = bounds.center();
    let pc = V2::new(0.0, -tsph + pr);
    let bc = pc;
    let tc = V2::new(0.0, -tsph + pr + sep);
    let placingd = &Displaying(|w| placing.centered(pc, 1.0, w));
    let overd = &Displaying(|w| over.centered(tc, 1.0, w));
    let blankd = &Displaying(|w| blank_circle.centered(bc, 1.2, w));
    write!(
        to,
        r##"<g transform="translate({},{}) scale({scale})">{overd}{blankd}{placingd}</g>
    "##,
        center.x, center.y
    )
    .unwrap();
}

pub fn do_sheet(span: V2, inserting: &impl Display, to: &mut dyn Write) {
    let span_x = span.x;
    let span_y = span.y;
    write!(
        to,
        r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created partially with Inkscape (http://www.inkscape.org/) but primarily through codegen -->

<svg
   width="{span_x}mm"
   height="{span_y}mm"
   viewBox="0 0 {span_x} {span_y}"
   version="1.1"
   id="svg1"
   inkscape:version="1.3.1 (91b66b0783, 2023-11-16)"
   sodipodi:docname="card front template.svg"
   xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
   xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg">
  <sodipodi:namedview
     id="namedview1"
     pagecolor="#ffffff"
     bordercolor="#000000"
     borderopacity="0.25"
     inkscape:showpageshadow="2"
     inkscape:pageopacity="1.0"
     inkscape:pagecheckerboard="0"
     inkscape:deskcolor="#d1d1d1"
     inkscape:document-units="mm"
     inkscape:zoom="0.64462111"
     inkscape:cx="197.79061"
     inkscape:cy="62.827604"
     inkscape:window-width="{span_x}"
     inkscape:window-height="{span_y}"
     inkscape:window-x="44"
     inkscape:window-y="0"
     inkscape:window-maximized="1"
     inkscape:current-layer="layer1" />
  <defs
     id="defs1" />
  {inserting}
</svg>
"##,
    )
    .unwrap();
}
