#![feature(let_chains, coroutines, iter_from_coroutine, extract_if)]

use std::{
    collections::HashMap,
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
    print_and_play_gen: Option<Box<PnpGen>>,
    output: String,
    check_frequencies: bool,
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
            print_and_play_gen: None,
            check_frequencies: false,
            output: "generated_card_svgs".to_string(),
        }
    }
}

fn make_land_counts(spurt: usize, modulo: usize, initial_ratios: &[u8]) -> Vec<u8> {
    let mut land_counts: Vec<u8> = Vec::from_iter(initial_ratios.iter().cloned());
    // let cards_per_sheet: u8 = 12; //all that matters for cost is how many sheets you spend
    let prevtotal = land_counts
        .iter()
        .cloned()
        .map(|i| i as usize)
        .sum::<usize>()
        + spurt;
    let mut free_cards = prevtotal.div_ceil(modulo) * modulo - prevtotal;
    let mut ci = 0;
    while free_cards > 0 {
        land_counts[ci] += 1;
        free_cards -= 1;
        ci = (ci + 1) % 4;
    }
    land_counts
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
            (spec.generate_front)(&mut w);
        }
        if conf.gen_back {
            let mut w =
                File::create(output_dir.join(&format!("{}[back].svg", &spec.name))).unwrap();
            (spec.generate_back)(&mut w);
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
        let final_ends_hand_made_svgs_path = Path::new("hand_made_cards/ends");
        let final_means_hand_made_svgs_path = Path::new("hand_made_cards/means");
        // don't bother generating two distinct land decks for this print run, too expensive
        // let final_surplus_land_svgs_path = Path::new("final_surplus_land_svgs");
        // let final_surplus_land_pngs_path = Path::new("final_surplus_land_pngs");

        fn do_cards(
            run_name: &str,
            cardgens: &Vec<CardGen>,
            output_dir: &Path,
            conf: &Conf,
            rng: &mut StdRng,
        ) {
            //generate all possible cards
            prep_clear_dir(output_dir);
            let fconf = &**conf.final_gen.as_ref().unwrap();

            // we used to winnow cards based on their weight according to various criteria. Since reducing the deck size (turns out cards cost money) we don't automate this. You can still use the check_frequencies stuff to make sure every land is represented in the appropriate categories though.
            // let cards_by_kind: Vec<Vec<CardSpec>> = cardgens
            //     .iter()
            //     .map(|g| g.generator.iter().collect())
            //     .collect();
            // let mut cardspecs = Vec::new();
            // //set the weights on each individual cardspec according to the rules
            // for (i, mut eg) in cards_by_kind.into_iter().enumerate() {
            //     for e in eg.iter_mut() {
            //         e.frequency_modifier *= fconf.frequency_for(&e);
            //     }
            //     //winnow down those cards by grabbing randomly by weight
            //     weighted_sampling::weighted_draws(
            //         &mut eg,
            //         cardgens[i].min_count,
            //         &mut cardspecs,
            //         rng,
            //     );
            // }

            let cardspecs: Vec<CardSpec> =
                cardgens.iter().flat_map(|g| g.generator.iter()).collect();

            if conf.check_frequencies {
                // field forest mountain volcano lake ice tomb void
                let change_dist_preferred = normalize([0.9, 1.0, 0.85, 1.0, 1.5, 1.5, 1.0, 1.18]);
                let mut change_dist_actual = [0.0; 8];
                let kill_dist_preferred = normalize([0.6, 1.2, 1.15, 1.5, 0.85, 1.5, 0.5, 1.6]);
                let mut kill_dist_actual = [0.0; 8];
                for spec in cardspecs.iter() {
                    if let Some((_, es)) = spec.properties.iter().find(|e| e.0 == Change) {
                        for e in elements().into_iter() {
                            if es.contains(&e) {
                                change_dist_actual[e] += 1.0;
                            }
                        }
                    }
                    if let Some((_, es)) = spec.properties.iter().find(|e| e.0 == Kill) {
                        for e in elements().into_iter() {
                            if es.contains(&e) {
                                kill_dist_actual[e] += 1.0;
                            }
                        }
                    }
                }
                for (e, (a, p)) in normalize(kill_dist_actual)
                    .iter()
                    .zip(kill_dist_preferred.iter())
                    .enumerate()
                {
                    if (a - p).abs() >= p * 0.28 {
                        let relation = if a - p > 0.0 { "higher" } else { "lower" };
                        let en = ELEMENT_NAMES[e];
                        println!("warning, ratio of kill cards that are {en}, {a}, is {relation} than expected ({p})");
                    }
                }
                for (e, (a, p)) in normalize(change_dist_actual)
                    .iter()
                    .zip(change_dist_preferred.iter())
                    .enumerate()
                {
                    if (a - p).abs() >= p * 0.28 {
                        let relation = if a - p > 0.0 { "higher" } else { "lower" };
                        let en = ELEMENT_NAMES[e];
                        println!("warning, ratio of change cards that are {en}, {a}, is {relation} than expected ({p})");
                    }
                }
            }

            println!(
                "{run_name} count: {}",
                cardspecs.iter().map(|s| s.repeat).sum::<usize>()
            );

            for spec in cardspecs.iter() {
                write_spec(spec, conf, output_dir);
            }
        }

        if fconf.gen_svgs {
            do_cards("ends", &ends_specs, final_ends_svgs_path, conf, &mut rng);
            do_cards("means", &means_specs, final_means_svgs_path, conf, &mut rng);

            // I changed my mind, not going to have a separate surplus land deck. There'll just be one deck and we'll instruct users to separate the surplus.
            // ack, as part of budgeting, we'll ignore the surplus_counts and just print the amount of excess that we can afford
            // let land_counts: Vec<u8> = fconf.land_counts.iter().zip(fconf.land_surplus_counts.iter()).map(|(a, b)| a + b).collect();
            let cards_per_sheet = 60;
            let land_counts = make_land_counts(0, cards_per_sheet, &fconf.land_counts);

            // let mut land_counts: Vec<u8> = fconf.land_counts.clone();
            // // let cards_per_sheet: u8 = 12; //all that matters for cost is how many sheets you spend
            // let cards_per_sheet: u8 = 60; //all that matters for cost is how many sheets you spend
            // let prevtotal = land_counts.iter().sum::<u8>();
            // let modu = prevtotal % cards_per_sheet;
            // let mut free_cards = if modu == 0 { 0 } else { cards_per_sheet - modu }; //so we get these for free
            // let mut ci = 0;
            // while free_cards > 0 {
            //     land_counts[ci] += 1;
            //     free_cards -= 1;
            //     ci = (ci + 1) % 4;
            // }
            prep_clear_dir(final_land_svgs_path);
            for spec in generation::land_specs_smaller(&assets, &land_counts)[0]
                .generator
                .iter()
            {
                write_spec(&spec, conf, final_land_svgs_path);
            }

            // prep_clear_dir(final_surplus_land_svgs_path);
            // for spec in generation::land_specs(&assets, &fconf.land_surplus_counts)[0]
            //     .generator
            //     .iter()
            // {
            //     write_spec(&spec, conf, final_surplus_land_svgs_path);
            // }
        }
        if fconf.gen_pngs {
            clear_or_create(final_ends_pngs_path);
            render_pngs_with_from_to(
                final_ends_svgs_path,
                final_ends_pngs_path,
                default_svg_to_png,
            );
            render_pngs_with_from_to(
                final_ends_hand_made_svgs_path,
                final_ends_pngs_path,
                default_svg_to_png,
            );

            clear_or_create(final_means_pngs_path);
            render_pngs_with_from_to(
                final_means_svgs_path,
                final_means_pngs_path,
                default_svg_to_png,
            );
            render_pngs_with_from_to(
                final_means_hand_made_svgs_path,
                final_means_pngs_path,
                default_svg_to_png,
            );

            clear_or_create(final_land_pngs_path);
            render_pngs_with_from_to(
                final_land_svgs_path,
                final_land_pngs_path,
                default_svg_to_png,
            );
        }
    } else {
        let debug_output_dir = Path::new(&conf.output);
        prep_clear_dir(debug_output_dir);
        // generates just a small sample (gen_count) of the possible cards for checking
        let land_specs = generation::land_specs_smaller(&assets, &[1, 1, 1, 1]);
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

    if let Some(ref pnpconf) = conf.print_and_play_gen {
        let print_and_play_svgs = Path::new("print_and_play_svgs");
        if pnpconf.gen_svgs {
            //because of hand_drawn_cards, we have to read from the generated svg files instead of just using cardspecs
            clear_or_create(Path::new("print_and_play"));
            let gather_from = |path, cards: &mut Vec<(usize, Rc<Asset>, Rc<Asset>)>| {
                let mut by_name = HashMap::new();
                let dens = read_dir(Path::new(path)).unwrap();
                for item_m in dens {
                    if let Ok(item) = item_m {
                        let fns =
                            String::from_utf8(Vec::from(item.file_name().as_bytes())).unwrap();
                        let mut namesplit = fns.split("[");
                        let name = namesplit.next().unwrap();
                        let within_brackets = namesplit.next().unwrap().split("]").next().unwrap();
                        let is_front = within_brackets.contains("face");
                        let repeat: Option<usize> = within_brackets
                            .split(",")
                            .nth(1)
                            .map(|ns| (ns.split("]").next().unwrap()).parse().unwrap());
                        let a = Rc::new(load_asset(item.path().as_path(), None));
                        let entry = by_name
                            .entry(name.to_string())
                            .or_insert((None, None, None));
                        if let Some(r) = repeat {
                            entry.0 = Some(r);
                        }
                        let inserting_to = if is_front { &mut entry.1 } else { &mut entry.2 };
                        *inserting_to = Some(a)
                    }
                }
                for (s, c) in by_name.into_iter() {
                    if !c.1.is_some() {
                        println!("front missing on {s}");
                    }
                    if !c.2.is_some() {
                        println!("back missing on {s}");
                    }

                    cards.push((c.0.unwrap_or(1), c.1.unwrap(), c.2.unwrap()));
                }
            };

            let mut cards: Vec<(usize, Rc<Asset>, Rc<Asset>)> = Vec::new();
            gather_from(Path::new("final_ends_svgs"), &mut cards);
            gather_from(Path::new("final_means_svgs"), &mut cards);

            //generate our card-shaped land svgs if needed
            let land_path = Path::new("land_as_cards");
            if create_maybe(land_path) {
                let modulo = 6 * 6;
                let land_counts = make_land_counts(
                    cards.iter().map(|c| c.0).sum::<usize>() % modulo,
                    modulo,
                    &conf.final_gen.as_ref().unwrap().land_counts,
                );
                for spec in generation::land_specs_card(&assets, &land_counts)[0]
                    .generator
                    .iter()
                {
                    write_spec(&spec, conf, land_path);
                }
            }
            gather_from(land_path, &mut cards);

            print_and_play_sheets(&assets, cards.into_iter(), print_and_play_svgs);
        }

        if pnpconf.gen_pngs {
            let png_path = Path::new("print_and_play_pngs");
            clear_or_create(png_path);
            render_pngs_with_from_to(print_and_play_svgs, png_path, svg_to_png_using_inkscape);
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

fn clear_or_create(path: &Path) {
    if let Ok(dens) = read_dir(&path) {
        for item_m in dens {
            if let Ok(item) = item_m {
                remove_file(item.path()).unwrap();
            }
        }
    } else {
        //create otherwise
        drop(create_dir(&path));
    }
}

fn create_maybe(path: &Path) -> bool {
    if let Ok(_) = read_dir(&path) {
        false
    } else {
        //create otherwise
        drop(create_dir(&path));
        true
    }
}

fn render_pngs_with_from_to(from: &Path, to: &Path, renderer: fn(&Path, &Path, &Database)) {
    //clear dir if present

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
            seed: 879,
            gen_front: true,
            gen_back: true,
            // final_gen: None,
            final_gen: Some(Box::new(FinalGenConf {
                gen_svgs: true,
                gen_pngs: true,
                ..FinalGenConf::default()
            })),
            print_and_play_gen: Some(Box::new(PnpGen {
                gen_svgs: true,
                gen_pngs: true,
            })),
            ..Conf::default()
        },
    );
    // svg_to_png_using_resvg(&Path::new("simple-case.svg"), &Path::new(""), &get_fonts())
    // render_pngs_with_inkscape();
    // render_pngs_with_resvg();
    // demo_boards(&assets);
}
