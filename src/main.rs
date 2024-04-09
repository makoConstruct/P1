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

use mako_infinite_shuffle::{ rng::LFSRFNTimes, Indexing, OpsRef, Shuffled, };

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

struct Conf {
    seed: u64,
    gen_count: usize,
    gen_front: bool,
    gen_back: bool,
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
            final_gen: None,
            output: "generated_card_svgs".to_string(),
        }
    }
}

fn gen_cards(assets: &Rc<Assets>, conf: &Conf) {
    let output_dir = Path::new(&conf.output);
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

    let ends_specs = generation::end_specs(assets);
    let means_specs = generation::means_specs(assets);
    let ends_means = ends_specs.into_iter().chain(means_specs.into_iter());

    let land_specs = generation::land_specs(assets);

    let cardspecs: Vec<CardSpec> = if let Some(ref fconf) = conf.final_gen {
        //generates the entire set and winnows them according to the weights of different kinds of cards in the conf
        let mut rng = rand::prelude::StdRng::seed_from_u64(conf.seed);
        let ems = ends_means.collect::<Vec<CardGen>>();
        let min_count: usize = ems.iter().map(|cg| cg.min_count).sum();
        
        //report info about the desired_proportions if necessary
        {
            let total_weight: f64 = ems.iter().map(|cg| cg.desired_proportion).sum();
            if total_weight != 0.0 && (total_weight > 1.3 || total_weight < 0.75) {
                print!("the total desired_proportion across all cardgens doesn't sum to 1 at all, it sums to {total_weight}, so here are some more reasonable weights if you want them:[");
                for cg in ems.iter() {
                    print!("{}, ", cg.desired_proportion/total_weight);
                }
                println!("]");
            }
        }
        
        assert!(fconf.total_preferred_count > min_count);
        let remainder = fconf.total_preferred_count - min_count;
        let weight_yet_ungranted: Vec<f64> = ems
            .iter()
            .map(|cg| {
                (cg.desired_proportion - cg.min_count as f64 / fconf.total_preferred_count as f64)
                    .max(0.0)
            })
            .collect();
        
        let mut cards_per_generator = weights_to_cuts(&weight_yet_ungranted, remainder);
        for (i, a) in cards_per_generator.iter_mut().enumerate() {
            *a += ems[i].min_count;
        }

        //now generate all possible cards
        let cards_by_kind: Vec<Vec<CardSpec>> =
            ems.iter().map(|g| g.generator.iter().collect()).collect();
        let mut cardspecs = Vec::new();
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
            let quota = cards_per_generator[i];
            if eg.len() <= quota {
                for s in eg.into_iter() {
                    cardspecs.push(s);
                }
            } else {
                use weighted_sampling::Weighted;
                impl Weighted<CardSpec> for CardSpec {
                    fn weight(&self) -> f64 {
                        self.frequency_modifier
                    }
                    fn transmit(self) -> CardSpec {
                        self
                    }
                }
                weighted_sampling::weighted_draws(&mut eg, quota, &mut cardspecs, &mut rng);
            }
        }
        cardspecs
    } else {
        // generates just a small sample (gen_count) of the possible cards for checking
        ends_means
            .chain(land_specs.into_iter())
            .flat_map(|specs| {
                Shuffled::<_, LFSRFNTimes>::new(DerefIndexing(specs.generator))
                    .into_iter()
                    .take(conf.gen_count)
            })
            .collect()
    };
    
    for spec in cardspecs {
        if conf.gen_front {
            (spec.generate_front)(
                &mut File::create(
                    output_dir.join(&format!("{}[face,{}].svg", &spec.name, spec.repeat)),
                )
                .unwrap(),
            );
        }
        if conf.gen_back {
            (spec.generate_back)(
                &mut File::create(output_dir.join(&format!("{}[back].svg", &spec.name))).unwrap(),
            );
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

use rand::SeedableRng;
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

// dedepricated for now, resvg indeed worked on the next version
// #[deprecated(
//     note = "everything just renders as transparent blank, even though their test cases run correctly on my machine. Version 0.39.0. This might start working on the next version. I notice the APIs for this are different, and some of this stuff required me to guess at APIs."
// )]
fn render_pngs_with_resvg() {
    render_pngs_with(svg_to_png_using_resvg);
}
fn render_pngs_with_inkscape() {
    render_pngs_with(svg_to_png_using_inkscape);
}
fn render_pngs_with(renderer: fn(&Path, &Path, &Database)) {
    let output_dir = Path::new("generated_card_pngs");
    let input_dir = Path::new("generated_card_svgs");
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

    let fonts = get_fonts();

    for item_m in read_dir(&input_dir).unwrap() {
        if let Ok(item) = item_m {
            let output = output_dir.join(format!(
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
            gen_back: true,
            final_gen: Some(Box::new(FinalGenConf::default())),
            // final_gen: None,
        },
    );
    // svg_to_png_using_resvg(&Path::new("simple-case.svg"), &Path::new(""), &get_fonts())
    // render_pngs_with_inkscape();
    // render_pngs_with_resvg();
    // demo_boards(&assets);
}
