use std::{fmt::Display, fs::File, io::Write, path::Path, rc::Rc};

mod boring;
use boring::*;

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

pub struct EndCardSpec {
    // likes: Vec<ElementTag>,
    name: String,
    generate_front: Rc<dyn Fn(&mut dyn Write)>,
    generate_back: Rc<dyn Fn(&mut dyn Write)>,
}
impl EndCardSpec {
    fn with_back_blured_message(
        name: String,
        front_graphic: Rc<dyn Display>,
        score: usize,
        back_text: String
    ) -> Self {
        let rcd = Rc::new(front_graphic);
        Self {
            name,
            generate_front: {let front_inner = rcd.clone(); Rc::new(move |w| {
                end_front_outer(&Displaying(|w|end_front_inner(&front_inner, score, w)), w);
            })},
            generate_back: Rc::new({let front_inner = rcd.clone(); move |w| {
                //you have to clone, because this lambda could be called multiple times, meaning it has to retain something to clone from to create the lambda ahead
                end_backing(&front_inner, w, &back_text);
            }}),
        }
    }
}

fn main() {
    {
        //clear dir if present
        if let Ok(dens) = std::fs::read_dir("generated_card_svgs") {
            for item_m in dens {
                if let Ok(item) = item_m {
                    std::fs::remove_file(item.path()).unwrap();
                }
            }
        } else {
            //create otherwise
            drop(std::fs::create_dir("generated_card_svgs"));
        }
        std::env::set_current_dir("generated_card_svgs/").unwrap();

        let mut specs = Vec::new();

        generation::generate_specs(&mut specs);

        for spec in specs.iter() {
            (spec.generate_front)(
                &mut File::create(Path::new(&format!("{}[front].svg", &spec.name))).unwrap(),
            );
            (spec.generate_back)(
                &mut File::create(Path::new(&format!("{}[back].svg", &spec.name))).unwrap(),
            );
        }

        std::env::set_current_dir("../").unwrap();
    }
}
