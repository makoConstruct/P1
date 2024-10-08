//performance refactor that could be done: take mut Writers instead of outputting Strings

use elementtree::WriteOptions;
use mako_infinite_shuffle::{Cross, Indexing};
use nalgebra::{Rotation2, Vector2};
use std::{cell::RefCell, f64::consts::TAU, fmt::Display, fs::File, io::Write, iter, path::Path, rc::Rc};

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
pub fn scale_sqrad(dims: V2, to_rad:f64)-> f64 {
    // s*dims.x*s*dims.y = to_rad*to_rad
    ((to_rad*to_rad)/(dims.x*dims.y)).sqrt()
}
/// scale_fit(a, b)*a fits within b
pub fn scale_fit(v: V2, bounds: V2) -> f64 {
    (bounds.x.abs() / v.x.abs()).min(bounds.y.abs() / v.y.abs())
}

pub fn normalize<const N: usize>(mut v: [f64; N]) -> [f64; N] {
    let sum: f64 = v.iter().sum();
    for vv in v.iter_mut() {
        *vv /= sum;
    }
    v
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

const CARD_BACKGROUND_COLOR: &'static str = "f1f2f2";

// field forest mountain volcano lake ice tomb void
pub type ElementTag = usize;
pub const FIELD: usize = 0;
pub const FOREST: usize = 1;
pub const MOUNTAIN: usize = 2;
pub const VOLCANO: usize = 3;
pub const LAKE: usize = 4;
pub const ICE: usize = 5;
pub const TOMB: usize = 6;
pub const VOID: usize = 7;
pub fn opposite_element(e: ElementTag) -> ElementTag {
    match e {
        FIELD => FOREST,
        FOREST => FIELD,
        MOUNTAIN => VOLCANO,
        VOLCANO => MOUNTAIN,
        LAKE => ICE,
        ICE => LAKE,
        TOMB => VOID,
        VOID => TOMB,
        _ => panic!("invalid ElementTag"),
    }
}
pub const ELEMENT_G: [ElementGenerator; 8] = [
    field_g, forest_g, mountain_g, volcano_g, lake_g, ice_g, tomb_g, void_g,
];
pub const ELEMENT_NAMES: [&'static str; 8] = [
    "field", "forest", "mountain", "volcano", "lake", "ice", "tomb", "void",
];
pub const ELEMENT_ARTICLE: [&'static str; 8] = ["a", "a", "a", "a", "a", "an", "a", "a"];
pub const ELEMENT_NAMES_PLURAL: [&'static str; 8] = [
    "fields",
    "forests",
    "mountains",
    "volcanoes",
    "lakes",
    "ice",
    "tombs",
    "voids",
];
pub const ELEMENT_NAMES_SINGULAR: [&'static str; 8] = [
    "field",
    "forest",
    "mountain",
    "volcano",
    "lake",
    "ice",
    "tomb",
    "void",
];
pub const ELEMENT_PAIR_NAMES: [&'static str; 4] =
    ["field/forest", "mountain/volcano", "lake/ice", "tomb/void"];
pub fn pair_name_for(e: ElementTag) -> &'static str {
    ELEMENT_PAIR_NAMES[e / 2]
}
#[derive(Clone)]
pub struct LandTheme {
    colors_back: [&'static str; 8],
    color_front: [&'static str; 8],
    asset_paths: [&'static str; 8],
}
// we need to be able to const initialize it twice for it to be the initial value of LAND_THEME, clone isn't const
macro_rules! mako_land_theme {
    ()=> {
        LandTheme {
            colors_back: [
                "b5efb9", "94cf9c", "eeeca7", "efcfcf", "c3edf1", "e1eff0", "ebebeb", "969696",
            ],
            color_front: [
                "a3e2a7", "7eb47f", "e5e383", "f2b7b7", "a5dae0", "f4fcfd", "dedede", "414141",
            ],
            asset_paths: [
                "assets/field.svg",
                "assets/forest.svg",
                "assets/mountain.svg",
                "assets/volcano.svg",
                "assets/lake.svg",
                "assets/ice.svg",
                "assets/tomb.svg",
                "assets/void.svg",
            ]
        }
    }
}
pub const MAKO_LAND_THEME:LandTheme = mako_land_theme!();
pub const LAND_THEME: RefCell<LandTheme> = RefCell::new(mako_land_theme!());
// pub const BOLD_COLOR_FOR_GRAPHIC: &'static str = "4b4b4b";
pub const BOLD_COLOR_FOR_GRAPHIC: &'static str = "c3c3c3";
pub const DARKER_BLANK_COLOR: &'static str = "c1c1c1";
pub fn element_color_bold(i: ElementTag) -> &'static str {
    if i != ICE && i != TOMB {
        LAND_THEME.borrow().color_front[i]
    } else {
        BOLD_COLOR_FOR_GRAPHIC
    }
}
pub fn element_color_front(i: ElementTag) -> &'static str {
    LAND_THEME.borrow().color_front[i]
}
pub fn element_color_back(i: ElementTag) -> &'static str {
    LAND_THEME.borrow().colors_back[i]
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
    Cross(elements(), 0..7).into_map(|(a, b)| (a, if b >= a { b + 1 } else { b }))
}
pub fn each_unordered_nonopposite_unequal_pair() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    Cross(0..8, 0..6).into_map(|(a, mut b)| {
        let ao = opposite_element(a);
        let mi = a.min(ao);
        let ma = a.max(ao);
        if b >= mi {
            b += 1
        }
        if b >= ma {
            b += 1
        }
        (a, b)
    })
}
pub fn each_unordered_nonequal_pairing() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    mako_infinite_shuffle::KSubsets::new(8, 2).into_map(|v| (v[0], v[1]))
}
pub fn each_unordered_pairing() -> impl Indexing<Item = (ElementTag, ElementTag)> {
    mako_infinite_shuffle::KSubmultisets::new(8, 2).into_map(|v| (v[0], v[1]))
}
pub fn each_unordered_nonequal_triple() -> impl Indexing<Item = (ElementTag, ElementTag, ElementTag)>
{
    mako_infinite_shuffle::KSubsets::new(8, 3).into_map(|v| (v[0], v[1], v[2]))
}
pub fn each_unordered_triple() -> impl Indexing<Item = (ElementTag, ElementTag, ElementTag)> {
    mako_infinite_shuffle::KSubmultisets::new(8, 3).into_map(|v| (v[0], v[1], v[2]))
}

pub struct CardGen {
    // this many are guaranteed to be present in the pool
    pub min_count: usize,
    // the total proportion of the pool that you want to consist of this one. Try to keep these summing to 1.0 so that the meaning of the numbers in code is clear, but it'll work okay regardless
    pub desired_proportion: f64,
    pub generator: Box<dyn Indexing<Item = CardSpec> + 'static>,
}

pub type V2 = Vector2<f64>;
// pub type U2 = Unit<V2>;
pub type R2 = Rotation2<f64>;

// there doesn't seem to be a way to make transforms be measured in pixel units :( but it hardly affects us to be real
// pub const big_element_dimensions:V2 = V2::new(405.540, 405.540);
// pub const card_dimensions:V2 = V2::new(600.0, 825.0);
// pub const end_graphic_center:V2 = V2::new(300.0, 525.0);
// pub const end_graphic_allowable_rad:f64 = 262.5;

// this was for the rounded guy
// pub const GUY2_ANCHOR: V2 = V2::new(19.912, 45.846);
pub const GUY2_ANCHOR: V2 = V2::new(21.243, 61.013);
pub const GUY2_DEAD_ANCHOR: V2 = V2::new(28.7, 47.095);
pub const GUY2_MAGE_ANCHOR: V2 = V2::new(36.380, 66.810);
pub const GUY2_RISEN_ANCHOR: V2 = V2::new(20.2, 45.75);
pub const GUY2_RAD: f64 = 20.2;
pub const GUY2_ADJACENCY_SMALLERNESS: f64 = 0.7;
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
    let color_back = element_color_back(FIELD);
    let color_front = element_color_front(FIELD);
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
    let color_back = element_color_back(FOREST);
    let color_front = element_color_front(FOREST);
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
    let number_offset = if &scores == "1" { 74.8 } else { 79.000023 };
    write!(to,
r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(0,-2e-4)">
    <polygon
       fill="#929497"
       points="144,0 0,0 0,198 144,198 "
       id="polygon1000"
       transform="matrix(1.1024306,0,0,1.1024306,0,2e-4)"
       style="fill:#f1f2f2;fill-opacity:1;stroke-width:0.24" />
    <path
       fill="#ffffff"
       stroke="#ec1e28"
       stroke-width="0.374174"
       d="M 138.9062,208.3596 H 19.8437 c -5.4799,0 -9.9221,-4.4417 -9.9221,-9.9219 V 19.844 c 0,-5.4802 4.4422,-9.9219 9.9221,-9.9219 h 119.0625 c 5.48,0 9.9218,4.4417 9.9218,9.9219 v 178.5937 c 0,5.4802 -4.4418,9.9219 -9.9218,9.9219 z"
       id="path1000"
       style="fill:#f1f2f2;fill-opacity:1;stroke:none" />
    <path
       id="path1023"
       style="fill:#d6d6d6;fill-opacity:1;stroke:none;stroke-width:12.3172;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1"
       d="M 0,69.4531 V 0 H 71.4633 L 79.375,8.355 87.2867,0 H 158.75 v 69.4531 z" />
    <path
       id="path1001"
       style="fill:#3f3f3f;fill-opacity:1;stroke:none;stroke-width:12.3172;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1"
       d="m 0,0 v 69.4531 l 71.351056,0 8.023943,-8.473528 8.023943,8.473528 71.351058,0 V 0 Z"
       sodipodi:nodetypes="cccccccc" />
    <text
       xml:space="preserve"
       style="font-style:normal;font-variant:normal;font-weight:500;font-stretch:normal;font-size:49.3895px;line-height:1.25;font-family:Rubik;-inkscape-font-specification:'Rubik Medium';text-align:center;letter-spacing:0px;word-spacing:0px;text-anchor:middle;fill:#eeeeee;fill-opacity:1;stroke:none;stroke-width:1.23474"
       x="{number_offset}"
       y="57.742939"
       id="text1001"><tspan
         sodipodi:role="line"
         id="tspan1001"
         x="{number_offset}"
         y="57.742939"
         style="font-style:normal;font-variant:normal;font-weight:500;font-stretch:normal;font-family:Rubik;-inkscape-font-specification:'Rubik Medium';text-align:center;text-anchor:middle;fill:#eeeeee;fill-opacity:1;stroke-width:1.23474">{scores}</tspan></text>
    {inserting}
  </g>"##,
    ).unwrap();
}

#[derive(Clone, Eq, PartialEq)]
pub enum CardSpecKind {
    Kill,
    Move,
    Change,
    Preference,
}
pub use CardSpecKind::*;

use crate::clear_or_create;
#[derive(Clone)]
pub struct CardSpec {
    // likes: Vec<ElementTag>,
    pub name: String,
    pub repeat: usize,
    pub level: usize,
    pub properties: Vec<(CardSpecKind, Vec<ElementTag>)>,
    // the amount this type of card's frequency should be changed from its baseline frequency
    pub frequency_modifier: f64,
    // the ratio of cards that are from this generator
    pub generate_front: Rc<dyn Fn(&mut dyn Write)>,
    pub generate_back: Rc<dyn Fn(&mut dyn Write)>,
}
impl CardSpec {
    pub fn has_property(&self, p: CardSpecKind, e: ElementTag) -> bool {
        self.properties
            .iter()
            .any(|ps| ps.0 == p && ps.1.iter().any(|es| *es == e))
    }
    pub fn means_card(
        assets: &Rc<Assets>,
        name: String,
        filename: Option<String>,
        // the level of play on which this card should become available
        level: usize,
        clown: bool,
        repeat: usize,
        properties: Vec<(CardSpecKind, Vec<usize>)>,
        front_graphic: Rc<dyn Fn(&mut dyn Write)>,
        back_text: String,
    ) -> Self {
        Self::means_card_repeated(
            assets,
            name,
            filename,
            repeat,
            level,
            clown,
            properties,
            front_graphic,
            back_text,
        )
    }
    pub fn means_card_repeated(
        assets: &Rc<Assets>,
        name: String,
        filename: Option<String>,
        repeated: usize,
        // the level of play on which this card should become available
        level: usize,
        clown: bool,
        properties: Vec<(CardSpecKind, Vec<usize>)>,
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
            frequency_modifier: 1.0,
            level,
            generate_front: {
                let front_graphic = front_graphic.clone();
                let name = name.clone();
                Rc::new(move |w| means_front(&Displaying(|w| front_graphic(w)), &name, w))
            },
            generate_back: Rc::new({
                let assets = assets.clone();
                move |w| {
                    card_outer(
                        &Displaying(|w| {
                            backing(
                                &assets,
                                &Displaying(|w| front_graphic(w)),
                                w,
                                &back_text,
                                level,
                                clown,
                                false,
                            )
                        }),
                        "",
                        CARD_BACKGROUND_COLOR,
                        false,
                        w,
                    );
                }
            }),
            properties,
        }
    }
    pub fn end_card_with_back_blurred_message(
        assets: &Rc<Assets>,
        name: String,
        front_graphic: Rc<dyn Display>,
        score: String,
        repeat: usize,
        back_text: String,
        elements_positive: Vec<ElementTag>,
        level: usize,
        clown: bool,
    ) -> Self {
        let rcd = Rc::new(front_graphic);
        let sc = score.clone();
        Self {
            name,
            repeat,
            level,
            generate_front: {
                let front_inner = rcd.clone();
                Rc::new(move |w| {
                    let scc = sc.clone();
                    // card_front_outer(, name, background_color, rotate, to)
                    end_outer(
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
                    end_outer(
                        &Displaying(|w| {
                            end_backing(&assets, &front_inner, w, &back_text, level, clown)
                        }),
                        w,
                    );
                }
            }),
            frequency_modifier: 1.0,
            properties: vec![(Preference, elements_positive)],
        }
    }
}

pub fn end_backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
    clown: bool,
) {
    backing(assets, inserting, to, description, level, clown, true);
}
pub fn backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
    clown: bool,
    is_end: bool,
) {
    let span = CARD_DIMENSIONS.x;
    let sep = span * 0.05;
    let level_marker = Displaying(|w| {
        let origin = cutline_bounds().br - V2::new(0.0, span*0.13);
        let mut offset = 0.0;
        if level >= 2 {
            let r = assets.level2.bounds.x/2.0;
            assets.level2.centered(
                origin + V2::new(-(offset + sep + r), 0.0),
                1.0,
                w,
            );
            offset += sep + r*2.0;
        }
        if level == 1 {
            let r = assets.level1.bounds.x/2.0;
            assets.level1.centered(
                origin + V2::new(-(offset + sep + r), 0.0),
                1.0,
                w,
            );
            offset += sep + r*2.0;
        }
        if clown {
            let r = assets.clown.bounds.x/2.0;
            assets.clown.centered(
                origin + V2::new(-(offset + sep + r), 0.0),
                1.0,
                w,
            );
        }
    });
    let end_bar = Displaying({
        let assets = assets.clone();
        move |w| {
            if is_end {
                assets.end_top_bar.by_ul(V2::new(0.0, 0.0), 1.0, w);
            }
        }
    });
    write!(to,
r##"

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
         stdDeviation="3.4"
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
    <g transform="matrix(-1,0,0,1,{span},0)" style="opacity:0.55;filter:url(#flipfilter)">
    <g style="opacity:0.5">
        {end_bar}
    </g>
    {inserting}
    </g>
    {level_marker}
    <text
       xml:space="preserve"
       transform="matrix(0.26458333,0,0,0.26458333,-0.21640517,0)"
       id="text1"
       style="font-weight:900;font-size:43px;font-family:'Inter UI';-inkscape-font-specification:'Rubik';text-align:center;vertical-align:bottom;white-space:pre;shape-inside:url(#descriptionrect);opacity:1;fill:#3e3e3e;fill-opacity:1;stroke:none;stroke-width:7.55906;stroke-linecap:round;stroke-linejoin:round"><tspan
         x="93.067162"
         y="126.73272"
         id="tspan3"><tspan
           style="font-weight:normal;font-family:Rubik;-inkscape-font-specification:Rubik"
           id="tspan2">{description}</tspan></tspan></text>
  </g>
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

pub fn underline(color: &str, anchor: V2, grav: V2, hspan: f64, to: &mut dyn Write) {
    let uspan = V2::new(81.849, 23.243);
    let scale = hspan / uspan.x;
    let offset = offset_for_grav_scale(anchor, grav, uspan, scale);
    write!(to, r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     transform="translate({},{}) scale({scale})"
     id="layer1">
    <rect
       transform="scale(-1)"
       ry="1.9218473"
       rx="1.9218473"
       y="-15.105407"
       x="-81.849274"
       height="15.105408"
       width="81.849274"
       id="rect4"
       style="fill:#f1f2f2;fill-opacity:1;stroke:none;stroke-width:6.42758;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" />
    <rect
       transform="scale(-1)"
       ry="1.9218473"
       rx="1.9218473"
       y="-23.243406"
       x="-81.849274"
       height="15.105408"
       width="81.849274"
       id="rect3"
       style="fill:#{color};fill-opacity:1;stroke:none;stroke-width:6.42758;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-dashoffset:0;stroke-opacity:1" />
  </g>"##, offset.x, offset.y).unwrap();
}

pub fn big_splat(color: &str, to: &mut dyn Write) {
    big_splat_scaled(color, 0.54, to);
}

pub fn big_splat_scaled(color: &str, scale: f64, to: &mut dyn Write) {
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

pub fn road_blob_rad(
    assets: &Assets,
    e1: ElementTag,
    e2: ElementTag,
    road: ElementTag,
    bounds: Rect,
    to: &mut dyn Write,
) {
    let center = bounds.center();
    let unscaled_anchor = V2::new(53.649, 53.649);
    let unscaled_rad = unscaled_anchor.x;
    let unscaled_span = V2::new(167.936, 212.7457);
    // to understand this code, understand that inner_span is the sort fo skeleton of the road blob
    let inner_span = V2::new(
        unscaled_span.x - 2.0 * unscaled_rad,
        unscaled_span.y - 2.0 * unscaled_rad,
    );
    let to_corner_element_center = V2::new(inner_span.x * 1.5, -inner_span.y / 2.0);
    let unscaled_total_radii = to_corner_element_center + both_dims(unscaled_rad);
    let scale = scale_fit(unscaled_total_radii, bounds.span() / 2.0);
    let e1c = unscaled_span / 2.0 - to_corner_element_center;
    let e2c = unscaled_span / 2.0 + to_corner_element_center;
    let rc = unscaled_span / 2.0 - inner_span / 2.0;
    let offset = center - scale * unscaled_span / 2.0;
    let color = element_color_back(road);
    write!(to,
        r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     transform="translate({},{}) scale({scale})"
     id="layer1">
    <path
       id="rect2665"
       style="fill:#{color};fill-opacity:1;stroke-width:1.48762;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:0.20634"
       d="M 53.65,0 C 24.020028,-2.1815337e-4 5.7989317e-5,24.019528 0,53.6495 0.05652097,73.170544 10.711886,91.120299 29.332876,102.32605 47.953866,113.5318 60.310565,135.27891 60.433,158.7867 h 0.2253 c -0.0076,0.10318 -0.01496,0.20638 -0.022,0.3096 1.13e-4,29.62993 24.020067,53.64962 53.65,53.6494 29.6297,-1.1e-4 53.64929,-24.0197 53.6494,-53.6494 -9.2e-4,-18.88963 -9.93573,-36.38688 -28.56906,-47.67054 C 120.73331,100.14209 107.65591,77.905508 107.5011,53.7378 h -0.2077 l 0.01,-0.088 C 107.30334,24.020062 83.283738,4.1321785e-4 53.654,3e-4 Z"
       sodipodi:nodetypes="cccccccccccccc" />
    "##,
        offset.x, offset.y
    ).unwrap();
    assets.element(e1).centered_rad(e1c, unscaled_rad, to);
    assets.element(e2).centered_rad(e2c, unscaled_rad, to);
    assets.element(road).centered_rad(rc, unscaled_rad, to);
    write!(to, r##"</g>"##).unwrap();
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

//these two replicate each others' dimensions
pub fn paired(e1: ElementTag, e2: ElementTag, flipped: bool, to: &mut dyn Write) {
    let sized = 0.55;
    let spaced = 0.08;
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
    pub fn grav_point(&self, grav: Gravity) -> V2 {
        self.center() + grav.component_mul(&(self.span() / 2.0))
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
/// stops before the HexSpiral's layer exceeds the usize
pub struct HexLayerIter(pub HexSpiral, pub usize);
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
    pub anchor: V2,
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
    pub fn by_grav_rad(&self, anchor: V2, grav: Gravity, rad: f64, to: &mut dyn Write) {
        let scale = (rad + rad) / self.bounds.max();
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
        self.by_anchor_given_rotated(
            anchor,
            anchor_for_grav(grav, self.bounds),
            scale,
            rotation,
            to,
        );
    }
    pub fn by_anchor(&self, anchor_screenspace: V2, scale: f64, to: &mut dyn Write) {
        self.by_anchor_given(anchor_screenspace, self.anchor, scale, to);
    }
    pub fn by_anchor_rad(&self, anchor_screenspace: V2, rad: f64, to: &mut dyn Write) {
        let scale = rad / (self.bounds.x / 2.0);
        self.by_anchor_given(anchor_screenspace, self.anchor, scale, to);
    }
    pub fn by_anchor_given(
        &self,
        anchor_screenspace: V2,
        anchor_within: V2,
        scale: f64,
        to: &mut dyn Write,
    ) {
        let ul = anchor_screenspace - anchor_within * scale;
        self.by_ul(ul, scale, to);
    }
    pub fn by_anchor_given_rotated(
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

pub fn load_asset(at: &Path, anchor: Option<V2>) -> Asset {
    // pub fn for_asset(at: &std::path::Path) -> Rc<dyn Display> {
    let assetxml = elementtree::Element::from_reader(&std::fs::File::open(at).unwrap()).unwrap_or_else(|e| panic!("couldn't parse {:?}. {:?}", at, e));
    //lol, turns out the comment isn't an element so the entire document is just the root element (what if a document contains multiple root elements? Is that not allowed?)
    let svgel = &assetxml;
    fn ignore_unit(v: &str) -> &str {
        v.split_at(v.len() - 2).0
    }
    let bounds = V2::new(
        str::parse(ignore_unit(&svgel.get_attr("width").unwrap())).unwrap(),
        str::parse(ignore_unit(&svgel.get_attr("height").unwrap())).unwrap(),
    );
    let mut inner: Vec<u8> = Vec::new();
    for e in svgel.children() {
        e.to_writer_with_options(&mut inner, WriteOptions::new().set_xml_prolog(None))
            .unwrap();
    }
    Asset {
        render: Rc::new(
            move |ul: V2, scale: f64, rotation: f64, to: &mut dyn Write| {
                write!(
                    to,
                    // r##"<g transform="translate({},{}) scale({scale}) rotate({rotation})">{defs_str}{graphic_str}</g>"##,
                    r##"<g transform="translate({},{}) scale({scale}) rotate({rotation})">"##,
                    ul.x, ul.y
                )
                .unwrap();
                to.write(&inner).unwrap();
                write!(to, "</g>").unwrap();
            },
        ),
        anchor: anchor.unwrap_or_else(|| bounds / 2.0),
        bounds,
    }
}

//used to use macros here but macros in rust are just so shit
pub struct Assets {
    pub kill: Asset,
    pub negatory: Asset,
    pub level1: Asset,
    pub level2: Asset,
    pub clown: Asset,
    pub guy: Asset,
    pub guy2: Asset,
    pub guy2_mage: Asset,
    pub guy2_flipped: Asset,
    pub dead_guy2: Asset,
    pub cubed_guy2: Asset,
    pub guyeye: Asset,
    pub dead_guy: Asset,
    pub altruism: Asset,
    pub field: Asset,
    pub forest: Asset,
    pub mountain: Asset,
    pub interventionist_helix: Asset,
    pub volcano: Asset,
    pub lake: Asset,
    pub ice: Asset,
    pub tomb: Asset,
    pub grouping1: Asset,
    pub grouping2: Asset,
    pub grouping3: Asset,
    pub void: Asset,
    pub blank: Asset,
    pub darker_blank: Asset,
    pub come_on_down: Asset,
    pub back_colored_circle: Asset,
    pub triangle: Asset,
    pub step: Asset,
    pub dog_altruism: Asset,
    pub kill_diamond: Asset,
    pub kill_diamond_around: Asset,
    pub double_diamond: Asset,
    pub end_top_bar: Asset,
    pub pnpmask: Asset,

    pub field_forest: Asset,
    pub mountain_volcano: Asset,
    pub lake_ice: Asset,
    pub tomb_void: Asset,

    //means flip TO field
    pub flip_field: Asset,
    pub flip_forest: Asset,
    pub flip_mountain: Asset,
    pub flip_volcano: Asset,
    pub flip_lake: Asset,
    pub flip_ice: Asset,
    pub flip_tomb: Asset,
    pub flip_void: Asset,

    //means flipping either
    pub flip_either_field_forest: Asset,
    pub flip_either_mountain_volcano: Asset,
    pub flip_either_tomb_void: Asset,
    pub flip_either_lake_ice: Asset,
}

fn generate_either(e1: &Asset, e2: &Asset) -> Asset {
    Asset {
        bounds: e1.bounds,
        anchor: e1.bounds / 2.0,
        render: Rc::new({
            let e1 = e1.clone();
            let e2 = e2.clone();
            move |p, s, rotation, w| {
                write!(w, r##"
<g transform="translate({},{}) scale({s}) rotate({rotation})">
<defs
     id="eitherdefs">
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath1either">
      <path
         id="path3"
         style="fill:#e4afaf;fill-opacity:1;stroke-width:23.4006;stroke-linecap:round;stroke-linejoin:round"
         d="M 53.649665,115.74702 H -15.872206 V 53.649629 -8.447763 h 69.521871 z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath2either">
      <path
         id="path4"
         style="fill:#e4afaf;fill-opacity:1;stroke-width:23.4006;stroke-linecap:round;stroke-linejoin:round"
         d="M 53.649337,115.74699 H 123.17121 V 53.649602 -8.447791 H 53.649337 Z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
  </defs>
<g clip-path="url(#clipPath1either)">{}</g>
<g clip-path="url(#clipPath2either)">{}</g>
</g>"##,
                    p.x, p.y,
                    &Displaying(|w| e1.by_ul(both_dims(0.0), 1.0, w)),
                    &Displaying(|w| e2.by_ul(both_dims(0.0), 1.0, w)),
                ).unwrap();
            }
        }),
    }
}

pub fn ring_conversion(
    assets: &Assets,
    c: V2,
    support: ElementTag,
    ring: ElementTag,
    w: &mut dyn Write,
) {
    let supporto = V2::new(-24.688, -80.945);
    let supportr = 49.378 / 2.0;
    let ringo = V2::new(-supportr, 7.248);
    let ringr = supportr;

    let ring_color = element_color_back(ring);
    let flip_from_color = element_color_back(opposite_element(ring));
    write!(w, r##"
<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{})">
    <g
       id="g3"
       transform="translate(-56.768161,-87.341589)">
      <path
         id="path1326"
         style="fill:#{ring_color};fill-opacity:1;stroke-width:1.56626;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:0.20634"
         d="M 86.827258,39.182875 A 31.17181,31.171893 0 0 1 56.769145,62.256402 31.17181,31.171893 0 0 1 26.718783,39.215432 56.769489,56.769489 0 0 0 0,87.323089 56.769489,56.769489 0 0 0 56.769661,144.09275 56.769489,56.769489 0 0 0 113.53881,87.323089 56.769489,56.769489 0 0 0 86.827258,39.182875 Z" />
      <path
         id="rect1371"
         style="fill:#{flip_from_color};fill-opacity:1;stroke-width:1.5;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:0.20634"
         d="m 18.361698,128.99514 a 56.769489,56.769489 0 0 0 38.408,15.0973 56.769489,56.769489 0 0 0 38.4074,-15.0973 z" />
    </g>
    {}
  </g>
"##,
     c.x, c.y,
     &Displaying(|w| {
        assets.flip_to(support).by_grav_rad(supporto, LEFT_TOP, supportr, w);
        assets.flip_to(ring).by_grav_rad(ringo, LEFT_TOP, ringr, w);
    }),
   ).unwrap();
}

pub fn horizontal_flip(a: &Asset) -> Asset {
    let ac = a.clone();
    Asset {
        bounds: a.bounds.clone(),
        anchor: V2::new(a.bounds.x - a.anchor.x, a.anchor.y),
        render: Rc::new(move |p, scale, angle, w| {
            write!(
                w,
                r##"<g transform="translate({},{}) scale({}) rotate({}) matrix(-1,0,0,1,{},0)">"##,
                p.x, p.y, scale, angle, ac.bounds.x,
            )
            .unwrap();
            (ac.render)(V2::new(0.0, 0.0), 1.0, 0.0, w);
            write!(w, "</g>").unwrap();
        }),
    }
}

impl Assets {
    pub fn load(_assets_dir: &Path) -> Self {
        let kill = load_asset(&Path::new("assets/kill.svg"), None);
        let negatory = load_asset(&Path::new("assets/negatory_shadowed.svg"), None);
        let level2 = load_asset(&Path::new("assets/level_22.svg"), None);
        let level1 = load_asset(&Path::new("assets/level1.svg"), None);
        let clown = load_asset(&Path::new("assets/clown_in_diamond.svg"), None);
        let guy = load_asset(&Path::new("assets/guy.svg"), None);
        let guyeye: Asset = load_asset(&Path::new("assets/guyeye.svg"), None);
        let dead_guy = load_asset(&Path::new("assets/dead_guy.svg"), None);
        let altruism = load_asset(&Path::new("assets/altruism.svg"), None);
        
        let land_paths = LAND_THEME.borrow().asset_paths;
        let field = load_asset(&Path::new(land_paths[0]), None);
        let forest = load_asset(&Path::new(land_paths[1]), None);
        let mountain = load_asset(&Path::new(land_paths[2]), None);
        let volcano = load_asset(&Path::new(land_paths[3]), None);
        let lake = load_asset(&Path::new(land_paths[4]), None);
        let ice = load_asset(&Path::new(land_paths[5]), None);
        let tomb = load_asset(&Path::new(land_paths[6]), None);
        let void = load_asset(&Path::new(land_paths[7]), None);
        
        let blank = load_asset(&Path::new("assets/blank.svg"), None);
        let darker_blank = load_asset(&Path::new("assets/darker_blank.svg"), None);
        let come_on_down = load_asset(&Path::new("assets/come_on_down.svg"), None);
        let back_colored_circle = load_asset(&Path::new("assets/back_colored_circle.svg"), None);
        let triangle = load_asset(
            &Path::new("assets/triangle.svg"),
            Some((V2::zeros() + V2::new(0.0, 1.0) + V2::new((3.0 / 4.0 as f64).sqrt(), 0.5)) / 3.0),
        );
        let end_top_bar = load_asset(&Path::new("assets/end_top_bar.svg"), None);
        // let step = load_asset(&Path::new("assets/step.svg"), None);
        let step = load_asset(&Path::new("assets/step2.svg"), None);
        let dog_altruism = load_asset(&Path::new("assets/dog_altruism.svg"), None);
        let guy2 = load_asset(&Path::new("assets/guy2_flat.svg"), Some(GUY2_ANCHOR));
        // let guy2_mage = load_asset(
        //     &Path::new("assets/guy2_mage_flat.svg"),
        //     Some(V2::new(37.347, 82.709)),
        // );
        let guy2_mage = load_asset(
            &Path::new("assets/guy2_mage_unfilled.svg"),
            Some(V2::new(40.812, 86.348)),
        );
        let dead_guy2 = load_asset(&Path::new("assets/dead_guy2.svg"), Some(GUY2_DEAD_ANCHOR));
        let cubed_guy2 = load_asset(
            &Path::new("assets/cubed guy2.svg"),
            Some(V2::new(27.316, 80.938)),
        );
        let kill_diamond = load_asset(&Path::new("assets/kill_diamond.svg"), None);
        let kill_diamond_around = load_asset(&Path::new("assets/kill_diamond_around.svg"), None);
        let double_diamond = load_asset(&Path::new("assets/double_diamond.svg"), None);

        let guy2_flipped = horizontal_flip(&guy2);
        let flip_field = element_flip(&forest, &field);
        let flip_forest = element_flip(&field, &forest);
        let flip_mountain = element_flip(&volcano, &mountain);
        let flip_volcano = element_flip(&mountain, &volcano);
        let flip_lake = element_flip(&ice, &lake);
        let flip_ice = element_flip(&lake, &ice);
        let flip_tomb = element_flip(&void, &tomb);
        let flip_void = element_flip(&tomb, &void);

        // let strikethrough_road = load_asset(&Path::new("assets/strikethrough_road.svg"), None);

        let field_forest = generate_either(&field, &forest);
        let mountain_volcano = generate_either(&mountain, &volcano);
        let lake_ice = generate_either(&lake, &ice);
        let tomb_void = generate_either(&tomb, &void);

        let flip_either_field_forest =
            element_flip(&field_forest, &generate_either(&forest, &field));
        let flip_either_mountain_volcano =
            element_flip(&mountain_volcano, &generate_either(&volcano, &mountain));
        let flip_either_tomb_void = element_flip(&tomb_void, &generate_either(&void, &tomb));
        let flip_either_lake_ice = element_flip(&lake_ice, &generate_either(&ice, &lake));

        Self {
            kill,
            negatory,
            level1,
            level2,
            clown,
            guy,
            guy2,
            kill_diamond_around,
            guy2_mage,
            interventionist_helix: load_asset(Path::new("assets/interventionist helix.svg"), None),
            guy2_flipped,
            dead_guy2,
            guyeye,
            dead_guy,
            altruism,
            darker_blank,
            field,
            triangle,
            forest,
            mountain,
            grouping1: load_asset(&Path::new("assets/grouping1.svg"), Some(V2::new(18.065, 18.065))),
            grouping2: load_asset(&Path::new("assets/grouping2.svg"), Some(V2::new(18.065, 18.065))),
            grouping3: load_asset(&Path::new("assets/grouping3.svg"), Some(V2::new(35.085, 47.547))),
            volcano,
            lake,
            double_diamond,
            cubed_guy2,
            ice,
            tomb,
            void,
            blank,
            come_on_down,
            pnpmask: load_asset(Path::new("assets/pnpmask.svg"), None),
            back_colored_circle,
            end_top_bar,
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
            kill_diamond,
            flip_either_field_forest,
            flip_either_mountain_volcano,
            flip_either_tomb_void,
            flip_either_lake_ice,
        }
    }
    pub fn element(&self, e: ElementTag) -> &Asset {
        match e {
            FIELD => &self.field,
            FOREST => &self.forest,
            MOUNTAIN => &self.mountain,
            VOLCANO => &self.volcano,
            LAKE => &self.lake,
            ICE => &self.ice,
            TOMB => &self.tomb,
            VOID => &self.void,
            _ => panic!("no such element as {e}"),
        }
    }
    pub fn element_both(&self, e: ElementTag) -> &Asset {
        match e {
            FIELD => &self.field_forest,
            MOUNTAIN => &self.mountain_volcano,
            LAKE => &self.lake_ice,
            TOMB => &self.tomb_void,
            _ => panic!(
                "{} is an invalid tag for a pair of elements",
                ELEMENT_NAMES[e]
            ),
        }
    }
    pub fn flip_to(&self, e: ElementTag) -> &Asset {
        match e {
            //means flip TO field
            FIELD => &self.flip_field,
            FOREST => &self.flip_forest,
            LAKE => &self.flip_lake,
            ICE => &self.flip_ice,
            TOMB => &self.flip_tomb,
            VOID => &self.flip_void,
            MOUNTAIN => &self.flip_mountain,
            VOLCANO => &self.flip_volcano,
            _ => panic!("{e} is not an element tag"),
        }
    }
    pub fn flip_either(&self, e: ElementTag) -> &Asset {
        match e {
            //means flip TO field
            FIELD => &self.flip_either_field_forest,
            FOREST => &self.flip_either_field_forest,
            LAKE => &self.flip_either_lake_ice,
            ICE => &self.flip_either_lake_ice,
            TOMB => &self.flip_either_tomb_void,
            VOID => &self.flip_either_tomb_void,
            MOUNTAIN => &self.flip_either_mountain_volcano,
            VOLCANO => &self.flip_either_mountain_volcano,
            _ => panic!("{e} is not an element tag"),
        }
    }
}

//end ends stuff, begin means

pub fn means_backing(
    assets: &Rc<Assets>,
    inserting: &impl Display,
    to: &mut dyn Write,
    description: &str,
    level: usize,
    clown: bool,
) {
    backing(assets, inserting, to, description, level, clown, false);
}

pub fn guylike(asset: &Asset, base_centered: V2, scale: f64, to: &mut dyn Write) {
    asset.by_anchor(base_centered, scale, to);
}

pub fn blank_front(inserting: &impl Display, color: &str, rotate: bool, to: &mut dyn Write) {
    card_outer(inserting, "", color, rotate, to);
}
pub fn means_front(inserting: &impl Display, name: &str, to: &mut dyn Write) {
    card_outer(inserting, name, CARD_BACKGROUND_COLOR, false, to);
}
pub fn card_outer(
    inserting: &impl Display,
    name: &str,
    background_color: &str,
    rotate: bool,
    to: &mut dyn Write,
) {
    let rotation = if rotate { "90" } else { "0" };
    write!(to, r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created with Inkscape (http://www.inkscape.org/) and also with mako -->

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
     id="defsbasic">
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
</svg>
"##).unwrap();
}

pub fn end_outer(inserting: &impl Display, to: &mut dyn Write) {
    let background_color = "f1f2f2";
    write!(to, r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created with Inkscape (http://www.inkscape.org/) and also with mako -->

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
    <g>
        {inserting}
    </g>
  </g>
</svg>
"##).unwrap();
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
    write!(to, r##"<g
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

pub fn chain_graphic(
    assets: &Assets,
    a: ElementTag,
    b: ElementTag,
    c: ElementTag,
    center: V2,
    r: f64,
    w: &mut dyn Write,
) {
    let tr = V2::new(123.744, 115.026);
    let ac = V2::new(32.535, 56.352);
    let bc = V2::new(0.0, 0.0);
    let cc = V2::new(65.070, 0.0);
    let er = 58.674;
    let ae = element_color_back(a);
    let be = element_color_back(b);
    let ce = element_color_back(c);
    let scale = r / (tr.x / 2.0);
    let offset = center - scale * tr / 2.0;
    write!(
        w,
        r##"
<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{}), scale({scale})"
     >
    <path
       id="b1"
       style="fill:#{be};fill-opacity:1;stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 52.652735,40.245696 -12.76314,7.3688 -12.76269,7.36854 2.15852,3.73867 c 1.45813,-0.0238 2.89643,0.34015 4.1677,1.0547 1.36653,0.78897 2.4775,1.9541 3.20058,3.35663 1.28363,2.48997 2.57379,4.67606 1.173,7.10204 -0.0148,0.0202 -0.0296,0.0403 -0.0445,0.0603 l 2.13578,3.69929 11.40169,-6.58277 11.62897,-6.71399 -2.13243,-3.69347 c -2.96826,0.0289 -4.588,-2.19745 -6.07311,-4.76764 -1.48334,-2.57122 -1.46733,-5.74195 0.0418,-8.2981 z"
       sodipodi:nodetypes="cccccsccccccccc" />
    <path
       id="a1"
       style="fill:#{ae};stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 61.950355,56.349656 a 8.2314558,8.2314434 60 0 1 -7.20728,-4.11283 l -18.30133,10.56628 a 8.2314434,8.2314558 30 0 1 0.21196,0.32993 8.2314434,8.2314558 30 0 1 -0.188,7.88782 8.2314434,8.2314558 30 0 1 -0.0445,0.0603 l 2.13579,3.69928 12.76269,-7.36854 12.76313,-7.3688 z" />
    <path
       id="b2"
       style="fill:#{be};fill-opacity:1;stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 50.442035,14.599716 v 14.73761 l -10e-6,14.73708 h 4.31705 c 0.70845,-1.27468 1.74279,-2.33831 2.99726,-3.08198 1.36654,-0.78895 2.93106,-1.16853 4.50722,-1.09345 2.79818,0.13333 5.33648,0.10905 6.73705,2.53516 0.01,0.023 0.0197,0.046 0.0304,0.0686 h 4.27157 v -13.16554 l -2e-5,-13.42797 -4.26484,-10e-6 c -1.4591,2.58505 -4.19705,2.87461 -7.16545,2.87566 -2.96841,-9.9e-4 -5.70635,-1.60024 -7.16547,-4.18526 z"
       sodipodi:nodetypes="cccccsccccccccc" />
    <path
       id="c1"
       style="fill:#{ce};stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 69.037285,14.599716 a 8.2314558,8.2314434 0 0 1 -7.16548,4.18525 l 10e-6,21.13256 a 8.2314558,8.2314434 60 0 1 0.39171,-0.0186 8.2314558,8.2314434 60 0 1 6.73706,4.10671 8.2314558,8.2314434 60 0 1 0.0304,0.0686 l 4.27158,-10e-6 -2e-5,-14.73704 2e-5,-14.7376 z" />
  {}
  </g>"##,
  offset.x, offset.y,
        &Displaying(|w| {
            let es = er / BIG_ELEMENT_SPAN;
            assets.element(a).by_ul(ac, es, w);
            assets.element(b).by_ul(bc, es, w);
            assets.element(c).by_ul(cc, es, w);
        })
    ).unwrap();
}

// pub fn pair_graphic(
//     a: ElementTag,
//     b: ElementTag,
//     aa: &Asset,
//     ba: &Asset,
//     center: V2,
//     r: f64,
//     w: &mut dyn Write,
// ) {

pub fn joined_pair_graphic_horizontal(
    assets: &Assets,
    a: ElementTag,
    b: ElementTag,
    center: V2,
    r: f64,
    w: &mut dyn Write,
) {
    let tr = V2::new(115.026, 91.209);
    let bc = V2::new(56.352, 32.535);
    let ac = V2::new(0.0, 0.0);
    let er = 58.674;
    let ae = element_color_back(a);
    let be = element_color_back(b);
    let scale = r / (tr.x / 2.0);
    let offset = center - scale * tr / 2.0;
    write!(
        w,
        r##"
<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{}), scale({scale})"
     >
    <path
       id="b1"
       style="fill:#{ae};fill-opacity:1;stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 54.983391,27.126322 -7.368804,12.763138 -7.368543,12.762687 3.738668,2.158524 c 1.250877,-0.749677 2.678457,-1.153637 4.136683,-1.170455 1.577935,3e-6 3.122628,0.45355 4.450099,1.306636 2.356641,1.514563 4.566998,2.762693 4.566867,5.564047 -0.0027,0.0249 -0.0055,0.0497 -0.0084,0.07447 l 3.699285,2.135789 6.582768,-11.401692 6.713989,-11.62897 -3.693474,-2.132423 c -2.556139,1.509157 -5.07205,0.390952 -7.643288,-1.092344 -2.57022,-1.485071 -4.14172,-4.239008 -4.112849,-7.207264 z"
       sodipodi:nodetypes="cccccsccccccccc" />
    <path
       id="a1"
       style="fill:#{be};stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 71.087347,36.42395 a 8.2314558,8.2314434 30 0 1 -8.298104,0.04182 L 52.222967,54.767107 a 8.2314434,8.2314558 0 0 1 0.348527,0.179747 8.2314434,8.2314558 0 0 1 3.781098,6.925053 8.2314434,8.2314558 0 0 1 -0.0084,0.07447 l 3.699288,2.135776 7.368544,-12.762688 7.368794,-12.763133 z" />
  {}
  </g>"##,
  offset.x, offset.y,
        &Displaying(|w| {
            let es = er / BIG_ELEMENT_SPAN;
            assets.element(a).by_ul(ac, es, w);
            assets.element(b).by_ul(bc, es, w);
        })
    ).unwrap();
}

pub fn pair_flip_verticalish(
    c: V2,
    r: f64,
    assets: &Assets,
    e1: ElementTag,
    e2: ElementTag,
    w: &mut dyn Write,
) {
    joined_pair_verticalish(
        c, r,
        &|c, r, w|{
            assets.flip_to(e1).centered_rad(c, r, w);
        },
        &|c, r, w|{
            assets.flip_to(e2).centered_rad(c, r, w);
        },
        element_color_back(e1),
        element_color_back(opposite_element(e2)),
        w
    );
}

pub fn joined_pair_verticalish(
    c: V2,
    r: f64,
    bottom: &impl Fn(V2, f64, &mut dyn Write),
    top: &impl Fn(V2, f64, &mut dyn Write),
    first_color:&str,
    second_color:&str,
    w:&mut dyn Write,
) {
    let ptsp = V2::new(91.208817, 115.02631);
    let e1ul = V2::new(0.0, 56.352);
    let e2ul = V2::new(32.535, 0.0);
    let er = 58.674 / 2.0;
    let s: f64 = r / (ptsp.x / 2.0);
    let offset = c - s * ptsp / 2.0;

    write!(
        w,
        r##"<g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate({},{}) scale({s})"
     >
    <path
       id="b1"
       style="fill:#{first_color};fill-opacity:1;stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 27.126323,60.042925 12.763138,7.368804 12.762687,7.368543 2.158524,-3.738668 c -0.749677,-1.250877 -1.153637,-2.678457 -1.170455,-4.136683 3e-6,-1.577935 0.45355,-3.122628 1.306636,-4.450099 1.514563,-2.356641 2.762693,-4.566998 5.564047,-4.566867 0.0249,0.0027 0.0497,0.0055 0.07447,0.0084 l 2.135789,-3.699285 -11.401692,-6.582768 -11.62897,-6.713989 -2.132423,3.693474 c 1.509157,2.556139 0.390952,5.07205 -1.092344,7.643288 -1.485071,2.57022 -4.239008,4.14172 -7.207264,4.112849 z"
       sodipodi:nodetypes="cccccsccccccccc" />
    <path
       id="a1"
       style="fill:#{second_color};stroke-width:1.34089;stroke-linecap:round;stroke-linejoin:round"
       d="m 36.423951,43.938969 a 8.2314434,8.2314558 30 0 1 0.04182,8.298104 l 18.301337,10.566276 a 8.2314558,8.2314434 0 0 1 0.179747,-0.348527 8.2314558,8.2314434 0 0 1 6.925053,-3.781098 8.2314558,8.2314434 0 0 1 0.07447,0.0084 L 64.082154,54.982836 51.319466,47.614292 38.556333,40.245498 Z" />
   {}
  </g>"##,
    offset.x, offset.y,
        &Displaying({
            |w| {
                top(e2ul + both_dims(er), er, w);
                bottom(e1ul + both_dims(er), er, w);
            }
        })
    ).unwrap();
}

pub const FLIP_RINGS_SPAN: f64 = 115.65681;
pub const FLIP_RINGS_RAD: f64 = FLIP_RINGS_SPAN / 2.0;

pub fn flipping_to(assets: &Assets, e: ElementTag, center: V2, scale: f64, w: &mut dyn Write) {
    let eo = opposite_element(e);
    let to_color = element_color_back(e);
    let from_color = element_color_back(eo);
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
    let color_left = element_color_back(e1);
    let color_right = element_color_back(e2);
    let splat_span = V2::new(205.18423, 224.67136);
    let scale = bounds.span().component_div(&splat_span).min() * 0.82;
    let offset = bounds.center() - scale * splat_span / 2.0;
    let (c1, c2) = tilted_pair(splat_span / 2.0, splat_span.x * 0.21);
    let e1d = Displaying(|w| assets.element(e1).by_grav(c1, MIDDLE_MIDDLE, 0.8, w));
    let e2d = Displaying(|w| assets.element(e2).by_grav(c2, MIDDLE_MIDDLE, 0.8, w));

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
    come_on_down_specifically(ea, ea, element_color_back(e), bounds, None, None, to);
}
pub fn come_on_down_specifically(
    left_asset: &Asset,
    right_asset: &Asset,
    el_color: &str,
    bounds: Rect,
    adjacent_guy: Option<&Asset>,
    adjacent_out_el: Option<&Asset>, //this one doesn't really fit, so don't use it
    to: &mut dyn Write,
) {
    let sd = bounds.span().x;
    let er = sd * 0.19;
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
       style="color:#000000;fill:#{el_color};stroke-linecap:round;stroke-linejoin:round;-inkscape-stroke:none"
       d="M 38.159945 0.0098185221 C 37.498511 0.023905038 36.837434 0.052318903 36.177637 0.0956014 C 28.260067 0.61499139 20.508659 3.2482202 14.083895 8.3576294 C 5.517543 15.170175 -1.2533172e-15 26.47775 0 40.095744 A 9.4499998 9.4499998 0 0 0 9.4490356 49.546847 A 9.4499998 9.4499998 0 0 0 18.900138 40.095744 C 18.900138 31.181933 21.795547 26.370862 25.847518 23.148458 C 29.89949 19.926055 35.796642 18.384631 42.089937 19.060335 C 54.676529 20.411744 67.298735 29.295611 67.298735 48.312297 A 9.4499998 9.4499998 0 0 0 67.402087 49.098295 L 55.604875 49.098295 L 76.933289 65.405827 L 98.261702 49.098295 L 86.158565 49.098295 A 9.4499998 9.4499998 0 0 0 86.197323 48.312297 C 86.197323 20.283577 65.169656 2.5291176 44.107385 0.26768392 C 42.132797 0.055674506 40.144244 -0.032441026 38.159945 0.0098185221 z " />
  </g>"##, codoffset.x, codoffset.y).unwrap();
    left_asset.centered(c1, escale, to);
    right_asset.centered(c2, escale, to);
    if let Some(guy) = adjacent_guy {
        guy.by_anchor(c1 + V2::new(0.0, er + sd*0.265), 0.77, to);
    }
    if let Some(el) = adjacent_out_el {
        el.centered_rad(c2 + V2::new(0.0, er + sd*0.06 + er), er, to);
    }
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

pub fn svg_outer(span: V2, background_color: &str, inserting: &impl Display, to: &mut dyn Write) {
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
     <polygon fill="#{background_color}" points="0,0 0,{span_y} {span_x},{span_y} {span_x},0 0,0 "/>
  {inserting}
</svg>
"##,
    )
    .unwrap();
}

pub fn do_sheet(span: V2, inserting: &impl Display, to: &mut dyn Write) {
    svg_outer(span, CARD_BACKGROUND_COLOR, inserting, to);
}

fn element_flip(from: &Asset, to: &Asset) -> Asset {
    Asset {
        bounds: BIG_ELEMENT_DIMENSIONS,
        anchor: BIG_ELEMENT_DIMENSIONS / 2.0,
        render: Rc::new({
            let to = to.clone();
            let from = from.clone();
            move |p, s, _rotation, w| {
                // flip_rings(
                //     element_color_back(e),
                //     element_color_back(opposite_element(e)),
                //     &Displaying(|w| to.by_ul(both_dims(FLIP_RINGS_RAD - BIG_ELEMENT_RAD), 1.0, w)),
                //     // p + both_dims(FLIP_RINGS_RAD),
                //     p + both_dims(FLIP_RINGS_RAD) * s,
                //     s,
                //     rotation,
                //     w,
                // );

                write!(
                    w,
                    r##"
<g transform="translate({},{}) scale({s})">
<defs
     id="defs1">
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath3">
      <path
         id="path4"
         style="fill:#c3c3c3;fill-opacity:0.988235;stroke-width:2.20738;stroke-linecap:round;stroke-linejoin:round"
         d="M 122.01824,74.762596 V 162.01159 H 53.649337 -14.719569 V 74.762596 Z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
    <clipPath
       clipPathUnits="userSpaceOnUse"
       id="clipPath4">
      <path
         id="path5"
         style="fill:#c3c3c3;fill-opacity:0.988235;stroke-width:2.20738;stroke-linecap:round;stroke-linejoin:round"
         d="M -14.719569,-12.486369 H 53.649337 122.01824 V 74.762625 H -14.719569 Z"
         sodipodi:nodetypes="cccccc" />
    </clipPath>
  </defs>
  <g
     inkscape:label="Layer 1"
     inkscape:groupmode="layer"
     id="layer1"
     transform="translate(3.5032504e-4,-4.5292512e-5)">
    <g
       id="g4"
       clip-path="url(#clipPath4)">
      {}
    </g>
    <g
       id="g3"
       clip-path="url(#clipPath3)">
      {}
    </g>
  </g>
</g>
                    "##,
                    p.x, p.y,
                    &Displaying(|w| to.by_ul(V2::new(0.0, 0.0), 1.0, w)),
                    &Displaying(|w| from.by_ul(V2::new(0.0, 0.0), 1.0, w)),
                ).unwrap();
            }
        }),
    }
}

pub fn guy2_mage(assets: &Rc<Assets>, c: V2, scale: f64, w: &mut dyn Write) {
    assets.guy2_mage.by_anchor(c, scale, w);
}

pub fn guy2_dead(assets: &Rc<Assets>, c: V2, scale: f64, w: &mut dyn Write) {
    assets.dead_guy2.by_anchor(c, scale, w);
}

pub fn guy2(assets: &Rc<Assets>, c: V2, scale: f64, w: &mut dyn Write) {
    assets.guy2.by_anchor(c, scale, w);
}

pub fn guy2_flipped(assets: &Rc<Assets>, c: V2, scale: f64, w: &mut dyn Write) {
    assets.guy2_flipped.by_anchor(c, scale, w);
}

// I was going to use constraint satisfication, but I think these constraints are all just ratios
pub struct FinalGenConf {
    pub total_preferred_count: usize,
    pub tomb_prefering_cards: f64,
    pub void_prefering_cards: f64,
    pub water_movement_cards: f64,
    pub kill_cards_for_void_volcano: f64,
    pub kill_cards_for_field: f64,
    pub kill_cards_for_tombs: f64,
    pub kill_cards_for_mountain: f64,
    pub water_ice_changing_cards: f64,
    pub cards_that_make_voids: f64,
    pub cards_that_make_tombs: f64,
    pub land_tile_shape: TileShape,
    pub land_counts: Vec<u8>,
    pub land_surplus_counts: Vec<u8>,
    pub end_ends: usize,
    pub end_continues: usize,
    pub gen_svgs: bool,
    pub gen_pngs: bool,
}
pub enum TileShape { Hex, Circle }
impl Default for FinalGenConf {
    fn default() -> Self {
        Self {
            total_preferred_count: 280,
            tomb_prefering_cards: 1.5,
            void_prefering_cards: 0.07,
            water_movement_cards: 3.2,
            kill_cards_for_void_volcano: 3.8,
            kill_cards_for_field: 0.9,
            kill_cards_for_tombs: 0.1,
            land_tile_shape: TileShape::Circle,
            kill_cards_for_mountain: 0.2,
            water_ice_changing_cards: 3.0,
            cards_that_make_voids: 2.6,
            end_ends: 3,
            end_continues: 13,
            cards_that_make_tombs: 0.7,
            land_counts: vec![15, 8, 7, 7],
            land_surplus_counts: vec![6, 6, 6, 6],
            gen_svgs: true,
            gen_pngs: false,
        }
    }
}
impl FinalGenConf {
    pub fn frequency_for(&self, e: &CardSpec) -> f64 {
        let mut total = 1.0;
        if e.has_property(Preference, TOMB) {
            total *= self.tomb_prefering_cards;
        }
        if e.has_property(Preference, VOID) {
            total *= self.void_prefering_cards;
        }
        if e.has_property(Move, LAKE) || e.has_property(Move, ICE) {
            total *= self.water_movement_cards;
        }
        if e.has_property(Kill, VOID) || e.has_property(Kill, VOLCANO) {
            total *= self.kill_cards_for_void_volcano;
        }
        if e.has_property(Kill, FIELD) {
            total *= self.kill_cards_for_field;
        }
        if e.has_property(Kill, TOMB) {
            total *= self.kill_cards_for_tombs;
        }
        if e.has_property(Kill, MOUNTAIN) {
            total *= self.kill_cards_for_mountain;
        }
        if e.has_property(Change, LAKE) || e.has_property(Change, ICE) {
            total *= self.water_ice_changing_cards;
        }
        if e.has_property(Change, VOID) {
            total *= self.cards_that_make_voids;
        }
        if e.has_property(Change, TOMB) {
            total *= self.cards_that_make_tombs;
        }
        total
    }
}

pub struct PnpGen {
    pub gen_svgs: bool,
    pub gen_pngs: bool,
    pub cutlines_on: bool,
}

pub fn print_and_play_sheets<I>(assets: &Assets, cards: I, output_dir: &Path, cutlines_on: bool)
where
    I: Iterator<Item = (usize, Rc<Asset>, Rc<Asset>)> + Clone,
{
    clear_or_create(output_dir);
    let (cards_front, cards_back): (Vec<Rc<Asset>>, Vec<Rc<Asset>>) = cards
        .flat_map(|(r, f, b)| iter::repeat_with(move || (f.clone(), b.clone())).take(r))
        .unzip();
    let card_count = cards_front.len();
    let tx = 6;
    let ty = 6;
    let sheets_needed = card_count.div_ceil(tx * ty);
    let page_dims = V2::new(674.688, 873.125);
    let cs = assets.pnpmask.bounds;
    let card_scale = (page_dims.x / tx as f64 / cs.x).min(page_dims.y / ty as f64 / cs.y);
    let card_span = cs * card_scale;

    let do_side = |cards: &Vec<Rc<Asset>>, is_front: bool| {
        let mut cards = cards.iter();
        for sheeti in 0..sheets_needed {
            let side = if is_front { "[face]" } else { "[back]" };
            let file_name = if sheets_needed == 1 {
                format!("sheet{side}.svg")
            } else {
                format!("sheet{sheeti}{side}.svg")
            };
            let mut w = File::create(output_dir.join(file_name)).unwrap();

            //displayings take immutable fns so we can't do this inline
            let mut inner_first = Vec::new();
            let mut inner_second = Vec::new();

            'outer: for y in 0..ty {
                for x in 0..tx {
                    if let Some(cn) = cards.next() {
                        // let lx = if is_front { x } else { tx - 1 - x };
                        let mx = card_span.x * x as f64;
                        let ul = V2::new(
                            if is_front {
                                mx
                            } else {
                                page_dims.x - mx - card_span.x
                            },
                            card_span.y * y as f64,
                        );
                        //render to different buffers to make sure the blur of the cards doesn't overlap any of the masks
                        cn.by_ul(ul, card_scale, &mut inner_first);
                        assets.pnpmask.by_ul(ul, card_scale, &mut inner_second);
                    } else {
                        break 'outer;
                    }; //checked at function start
                }
            }
            
            svg_outer(
                page_dims,
                CARD_BACKGROUND_COLOR,
                &Displaying(|w| {
                    w.write_all(&inner_first).unwrap();
                    w.write_all(&inner_second).unwrap();
                    if cutlines_on {
                        //vertical
                        let vertical_line_length = page_dims.y + 2.0;
                        for ci in 0..(tx+1) {
                            let mut offset = ci as f64 * card_span.x;
                            if !is_front {
                                offset = page_dims.x - offset;
                            }
                            write!(w,
                            "<path
                                style=\"opacity:0.5;fill:none;stroke-width:1.9;stroke-linecap:round;stroke-linejoin:round;fill-opacity:1;stroke:#ebebeb;stroke-opacity:1;stroke-dasharray:none\"
                                d=\"M {offset},-1 V {vertical_line_length}\"
                                id=\"path1\" />").unwrap();
                        }
                        //horizontal
                        let horizontal_line_length = page_dims.x + 2.0;
                        for ci in 0..(ty+1) {
                            let offset = ci as f64 * card_span.y;
                            write!(w,
                            "<path
                                style=\"opacity:0.5;fill:none;stroke-width:1.9;stroke-linecap:round;stroke-linejoin:round;fill-opacity:1;stroke:#ebebeb;stroke-opacity:1;stroke-dasharray:none\"
                                d=\"M -1,{offset} H{horizontal_line_length}\"
                                id=\"path1\" />").unwrap();
                        }
                    }
                }),
                &mut w,
            );
        }
    };
    do_side(&cards_front, true);
    do_side(&cards_back, false);
}
