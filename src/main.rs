use std::{
    collections::HashSet,
    fs::{create_dir, read_dir, remove_file, File},
    io::Write,
    os::unix::ffi::OsStrExt,
    path::Path,
    rc::Rc,
};

//core logic is in main, extended logic in generation, piles of uninteresting stuff in boring
mod boring;
pub use boring::*;
mod generation;

fn cards_to_remove() -> HashSet<&'static str> {
    HashSet::from([])
}

fn gen_cards(assets: &Rc<Assets>) {
    let output_dir = Path::new("generated_card_svgs");
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
    let land_specs = generation::land_specs(assets);
    let all_cards = ends_specs
        .into_iter()
        .chain(means_specs.into_iter())
        .chain(land_specs.into_iter());
    
    // otherwise, just gen one per category
    let gen_all = false;

    for specs in all_cards {
        let specsi: Box<dyn Iterator<Item=CardSpec>> = if gen_all {
            Box::new(specs.iter())
        } else {
            Box::new(specs.iter().take(1))
        };
        for spec in specsi {
            (spec.generate_front)(
                &mut File::create(
                    output_dir.join(&format!("{}[face,{}].svg", &spec.name, spec.repeat)),
                )
                .unwrap(),
            );
            (spec.generate_back)(
                &mut File::create(output_dir.join(&format!("{}[back].svg", &spec.name))).unwrap(),
            );
        }
    }

    // let to_remove = cards_to_remove();

    // drop(create_dir("final cards"));

    // if let Ok(ecsf) = read_dir("handmade cards") {
    //     for c in ecsf {
    //         if let Ok(cc) = c {
    //             if to_remove.contains(cc.file_name().to_str().unwrap()) { continue; }
    //             fs::copy(cc.path(), Path::new("final cards").join(cc.file_name())).unwrap();
    //         }
    //     }
    // }
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

use mako_infinite_shuffle::OpsRef;
use resvg::usvg::fontdb::Database;
fn get_fonts()-> Database {
    let mut fonts = Database::new();
    // fonts
    //     .load_font_file(&Path::new("Rubik-VariableFont_wght.ttf"))
    //     .unwrap();
    fonts.load_system_fonts();
    fonts
}
fn svg_to_png_using_resvg(p: &Path, output_dir: &Path, fonts: &Database) {
    use resvg::{tiny_skia, usvg::{ Options, Tree}};
    let svgdata = Tree::from_data(&std::fs::read(p).unwrap(), &Options::default()).unwrap();
    let pixmap_size = svgdata.view_box.rect.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &svgdata,
        tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );
    pixmap
        .save_png(&output_dir.join(format!(
            "{}.png",
            String::from_utf8(Vec::from(p.file_stem().unwrap().as_bytes())).unwrap()
        )))
        .unwrap();
}


#[deprecated(note = "everything just renders as transparent blank, even though their test cases run correctly on my machine. Version 0.39.0. This might start working on the next version. I notice the APIs for this are different, and some of this stuff required me to guess at APIs.")]
fn render_svgs_with_resvg() {
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
            svg_to_png_using_resvg(&item.path(), output_dir, &fonts);
        }
    }
}

fn main() {
    let assets = Rc::new(Assets::load(Path::new("assets")));

    gen_cards(&assets);
    // svg_to_png_using_resvg(&Path::new("simple-case.svg"), &Path::new(""), &get_fonts())
    // render_svgs_with_resvg();
    // demo_boards(&assets);
}
