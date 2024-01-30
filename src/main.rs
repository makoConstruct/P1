use std::{
    collections::HashSet,
    fs::{self, create_dir, read_dir, remove_file, File},
    io::Write,
    path::Path,
    rc::Rc,
};

//core logic is in main, extended logic in generation, piles of uninteresting stuff in boring
mod boring;
pub use boring::*;
mod generation;

// #[derive(Serialize)]
// struct CardFrontParams<'a> {
//     inserting:&'a str,
// }

// fn do_end(tt:&TinyTemplate, name:&str, description:&str, inserting:Option<&str>)-> String {
//     let mut out = String::new();
//     tt.
//     out
// }

pub fn elements() -> impl Iterator<Item = ElementTag> {
    0..8
}
pub fn element_primaries() -> impl Iterator<Item = (ElementTag, ElementTag)> {
    (0..4).map(|i| (i * 2, i * 2 + 1))
}

fn cards_to_remove() -> HashSet<&'static str> {
    HashSet::from([])
}

fn main() {
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

    let assets = Rc::new(SvgAssets::load(Path::new("assets")));
    let all_assets = Rc::new(AllAssets {
        generated: generate_assets(&assets),
        svg: assets,
    });

    let ends_specs = generation::end_specs(&all_assets);
    let means_specs = generation::means_specs(&all_assets);

    for spec in ends_specs.iter().chain(means_specs.iter()).chain(generation::land_specs(&all_assets).iter()) {
        (spec.generate_front)(
            &mut File::create(output_dir.join(&format!("{}[front].svg", &spec.name))).unwrap(),
        );
        (spec.generate_back)(
            &mut File::create(output_dir.join(&format!("{}[back].svg", &spec.name))).unwrap(),
        );
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
