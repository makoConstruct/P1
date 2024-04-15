#![feature(let_chains, coroutines, iter_from_coroutine, extract_if)]

use std::{
    fs::{create_dir, read_dir, remove_file, File},
    io::Write,
    ops::Deref,
    os::unix::ffi::OsStrExt,
    path::Path,
    process::Command,
    rc::Rc,
};

//core logic is in main, extended logic in generation, piles of uninteresting stuff in boring
mod boring;
pub use boring::*;
mod generation;
mod weighted_sampling;

use mako_infinite_shuffle::{rng::LFSRFNTimes, Indexing, OpsRef, Shuffled};

pub struct DerefIndexing<V>(V);
impl<V> Indexing for DerefIndexing<V>
where
    V: Deref,
    V::Target: Indexing,
{
    type Item = <V::Target as Indexing>::Item;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, at: usize) -> Self::Item {
        self.0.get(at)
    }
}

use weighted_sampling::Weighted;
impl Weighted<CardSpec> for CardSpec {
    fn weight(&self) -> f64 {
        self.frequency_modifier
    }
    fn transmit(self) -> CardSpec {
        self
    }
}

struct Conf {
    seed: u64,
    gen_count: usize,
    gen_front: bool,
    gen_back: bool,
    cut_clip: bool,
    final_gen: Option<Box<FinalGenConf>>,
    output: String,
}
impl Default for Conf {
    fn default() -> Self {
        Self {
            seed: 80,
            gen_count: 1,
            gen_front: true,
            gen_back: true,
            cut_clip: false,
            final_gen: None,
            output: "generated_card_svgs".to_string(),
        }
    }
}

fn gen_cards(assets: &Rc<Assets>, conf: &Conf) {
    fn prep_clear_dir(p: &Path) {
        if let Ok(dens) = read_dir(p) {
            for item_m in dens {
                if let Ok(item) = item_m {
                    remove_file(item.path()).unwrap();
                }
            }
        } else {
            //create otherwise
            drop(create_dir(p));
        }
    }

    let ends_specs = generation::end_specs(assets);
    let means_specs = generation::means_specs(assets);

    fn write_spec(spec: &CardSpec, conf: &Conf, output_dir: &Path) {
        if conf.gen_front {
            let mut w =
                File::create(output_dir.join(&format!("{}[face,{}].svg", &spec.name, spec.repeat)))
                    .unwrap();
            svg_outer(
                conf.cut_clip,
                &Displaying(|w| {
                    (spec.generate_front)(w);
                }),
                &mut w,
            );
        }
        if conf.gen_back {
            let mut w =
                File::create(output_dir.join(&format!("{}[back].svg", &spec.name))).unwrap();
            svg_outer(
                conf.cut_clip,
                &Displaying(|w| {
                    (spec.generate_back)(w);
                }),
                &mut w,
            );
        }
    }

    if let Some(ref fconf) = conf.final_gen {
        //generates the entire set and winnows them according to the weights of different kinds of cards in the conf
        let mut rng = StdRng::seed_from_u64(conf.seed);
        let final_means_svgs_path = Path::new("final_means_svgs");
        let final_ends_svgs_path = Path::new("final_ends_svgs");
        let final_land_svgs_path = Path::new("final_land_svgs");
        let final_means_pngs_path = Path::new("final_means_pngs");
        let final_ends_pngs_path = Path::new("final_ends_pngs");
        let final_land_pngs_path = Path::new("final_land_pngs");
        let final_surplus_land_svgs_path = Path::new("final_surplus_land_svgs");
        let final_surplus_land_pngs_path = Path::new("final_surplus_land_pngs");

        fn do_cards(cardgens: &Vec<CardGen>, output_dir: &Path, conf: &Conf, rng: &mut StdRng) {
            //generate all possible cards
            prep_clear_dir(output_dir);
            let cards_by_kind: Vec<Vec<CardSpec>> = cardgens
                .iter()
                .map(|g| g.generator.iter().collect())
                .collect();
            let mut cardspecs = Vec::new();
            let fconf = &**conf.final_gen.as_ref().unwrap();
            //set the weights on each individual cardspec according to the rules
            for (i, mut eg) in cards_by_kind.into_iter().enumerate() {
                for e in eg.iter_mut() {
                    if e.has_property(Preference, TOMB_I) {
                        e.frequency_modifier *= fconf.tomb_prefering_cards;
                    }
                    if e.has_property(Preference, VOID_I) {
                        e.frequency_modifier *= fconf.void_prefering_cards;
                    }
                    if e.has_property(Move, LAKE_I) || e.has_property(Move, ICE_I) {
                        e.frequency_modifier *= fconf.water_movement_cards;
                    }
                    if e.has_property(Kill, VOID_I) || e.has_property(Kill, VOLCANO_I) {
                        e.frequency_modifier *= fconf.kill_cards_for_void_volcano;
                    }
                    if e.has_property(Kill, FIELD_I) {
                        e.frequency_modifier *= fconf.kill_cards_for_field;
                    }
                    if e.has_property(Kill, TOMB_I) {
                        e.frequency_modifier *= fconf.kill_cards_for_tombs;
                    }
                    if e.has_property(Kill, MOUNTAIN_I) {
                        e.frequency_modifier *= fconf.kill_cards_for_mountain;
                    }
                    if e.has_property(Change, LAKE_I) || e.has_property(Change, ICE_I) {
                        e.frequency_modifier *= fconf.water_ice_changing_cards;
                    }
                    if e.has_property(Change, VOID_I) {
                        e.frequency_modifier *= fconf.cards_that_make_voids;
                    }
                    if e.has_property(Change, TOMB_I) {
                        e.frequency_modifier *= fconf.cards_that_make_tombs;
                    }
                }
                //winnow down those cards by grabbing randomly by weight
                weighted_sampling::weighted_draws(
                    &mut eg,
                    cardgens[i].min_count,
                    &mut cardspecs,
                    rng,
                );
            }
            for spec in cardspecs.iter() {
                write_spec(spec, conf, output_dir);
            }
        }

        if fconf.gen_svgs {
            do_cards(&ends_specs, final_ends_svgs_path, conf, &mut rng);
            do_cards(&means_specs, final_means_svgs_path, conf, &mut rng);

            prep_clear_dir(final_land_svgs_path);
            for spec in generation::land_specs(&assets, &fconf.land_counts)[0]
                .generator
                .iter()
            {
                write_spec(&spec, conf, final_land_svgs_path);
            }

            prep_clear_dir(final_surplus_land_svgs_path);
            for spec in generation::land_specs(&assets, &fconf.land_surplus_counts)[0]
                .generator
                .iter()
            {
                write_spec(&spec, conf, final_surplus_land_svgs_path);
            }
        }
        if fconf.gen_pngs {
            render_pngs_with_from_to(
                final_ends_svgs_path,
                final_ends_pngs_path,
                default_svg_to_png,
            );
            render_pngs_with_from_to(
                final_means_svgs_path,
                final_means_pngs_path,
                default_svg_to_png,
            );
            render_pngs_with_from_to(
                final_land_svgs_path,
                final_land_pngs_path,
                default_svg_to_png,
            );
            render_pngs_with_from_to(
                final_surplus_land_svgs_path,
                final_surplus_land_pngs_path,
                default_svg_to_png,
            );
        }
    } else {
        let debug_output_dir = Path::new(&conf.output);
        prep_clear_dir(debug_output_dir);
        // generates just a small sample (gen_count) of the possible cards for checking
        let land_specs = generation::land_specs(&assets, &[1, 1, 1, 1]);
        for gen in ends_specs
            .iter()
            .chain(means_specs.iter())
            .chain(land_specs.iter())
        {
            for spec in Shuffled::<_, LFSRFNTimes>::new(DerefIndexing(&*gen.generator))
                .iter()
                .take(conf.gen_count)
            {
                write_spec(&spec, conf, debug_output_dir);
            }
        }
    }
}

fn demo_boards(assets: &Rc<Assets>) {
    let output_dir = Path::new("boards");
    //clear dir if present
    if let Ok(dens) = read_dir(&output_dir) {
        for item_m in dens {
            if let Ok(item) = item_m {
                remove_file(item.path()).unwrap();
            }
        }
    } else {
        //create otherwise
        drop(create_dir(&output_dir));
    }

    fn do_boards(
        assets: &Rc<Assets>,
        weights: &Vec<f64>,
        rad: usize,
        count: usize,
        output_dir: &Path,
    ) {
        let mut weights_str = Vec::new();
        for w in weights.iter() {
            write!(&mut weights_str, "{w}_").unwrap();
        }
        let ws = String::from_utf8(weights_str).unwrap();
        for i in 0..count {
            generation::generate_board(
                &assets,
                &weights,
                rad,
                true,
                i as u64,
                &mut File::create(&output_dir.join(format!("{}board{i}.svg", &ws))).unwrap(),
            );
        }
    }
    do_boards(
        &assets,
        &vec![12.7, 7.0, 6.0, 5.0],
        3,
        6,
        &Path::new("boards"),
    );
}

use rand::{rngs::StdRng, SeedableRng};
use resvg::usvg::fontdb::Database;

use crate::generation::weights_to_cuts;
fn get_fonts() -> Database {
    let mut fonts = Database::new();
    // fonts
    //     .load_font_file(&Path::new("Rubik-VariableFont_wght.ttf"))
    //     .unwrap();
    fonts.load_system_fonts();
    fonts
}
fn svg_to_png_using_resvg(p: &Path, output: &Path, fonts: &Database) {
    use resvg::{
        tiny_skia,
        usvg::{Options, Tree},
    };
    let svgdata = Tree::from_data(&std::fs::read(p).unwrap(), &Options::default(), fonts).unwrap();
    let pixmap_size = svgdata.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &svgdata,
        tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(output).unwrap();
}
pub fn svg_to_png_using_inkscape(input: &Path, output: &Path, _fonts: &Database) {
    let mut c = Command::new("inkscape");
    c.arg("--export-type=png");
    c.arg(input);
    c.arg("-o");
    c.arg(output);
    c.output().unwrap();
}
fn default_svg_to_png(input: &Path, output: &Path, _fonts: &Database) {
    svg_to_png_using_inkscape(input, output, _fonts);
}

fn render_pngs_with_resvg() {
    render_pngs_with(svg_to_png_using_resvg);
}
fn render_pngs_with_inkscape() {
    render_pngs_with(svg_to_png_using_inkscape);
}
fn render_pngs_with(renderer: fn(&Path, &Path, &Database)) {
    let to = Path::new("generated_card_pngs");
    let from = Path::new("generated_card_svgs");
    render_pngs_with_from_to(from, to, renderer);
}
fn render_pngs_with_from_to(from: &Path, to: &Path, renderer: fn(&Path, &Path, &Database)) {
    //clear dir if present
    if let Ok(dens) = read_dir(&to) {
        for item_m in dens {
            if let Ok(item) = item_m {
                remove_file(item.path()).unwrap();
            }
        }
    } else {
        //create otherwise
        drop(create_dir(&to));
    }

    let fonts = get_fonts();

    for item_m in read_dir(&from).unwrap() {
        if let Ok(item) = item_m {
            let output = to.join(format!(
                "{}.png",
                String::from_utf8(Vec::from(item.path().file_stem().unwrap().as_bytes())).unwrap()
            ));
            renderer(&item.path(), Path::new(&output), &fonts);
        }
    }
}

fn main() {
    let assets = Rc::new(Assets::load(Path::new("assets")));

    gen_cards(
        &assets,
        &Conf {
            output: "generated_card_svgs".to_string(),
            seed: 90,
            gen_count: 4,
            gen_front: true,
            gen_back: false,
            cut_clip: true,
            // final_gen: Some(Box::new(FinalGenConf {
            //     gen_svgs: true,
            //     gen_pngs: false,
            //     ..FinalGenConf::default()
            // })),
            final_gen: None,
        },
    );
    // svg_to_png_using_resvg(&Path::new("simple-case.svg"), &Path::new(""), &get_fonts())
    // render_pngs_with_inkscape();
    // render_pngs_with_resvg();
    // demo_boards(&assets);
}
