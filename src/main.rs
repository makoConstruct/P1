use std::{
    collections::HashMap,
    f64::consts::TAU,
    fs::{read_to_string, write},
    path::Path,
    process::Command,
};

mod boring;
use boring::*;

// #[derive(Serialize)]
// struct CardFrontParams<'a> {
//     inserting:&'a str,
// }

// fn do_end(tt:&TinyTemplate, name:&str, description:&str, inserting:Option<&str>)-> String {
//     let mut out = String::new();
//     tt.
//     out
// }

fn elements() -> impl Iterator<Item = ElementTag> {
    0..8
}

struct CardSpec {
    // likes: Vec<ElementTag>,
    name: String,
    generate: Box<dyn Fn() -> String>,
}

fn main() {
    // let mut tt = TinyTemplate<'static>::new();
    // tt.add_template("end_front", &read_to_string(Path::new("card front template.svg")).unwrap());
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

        for e in elements() {
            specs.push(CardSpec {
                name: format!("{}_1", element_names[e]),
                generate: Box::new(move || end_front(&(element_g[e])(end_graphic_center, 1.0), 1)),
            })
        }

        for e1 in elements() {
            for e2 in elements() {
                if e1 >= e2 {
                    continue;
                }
                specs.push(CardSpec {
                    name: format!("{}_{}", element_names[e1], element_names[e2]),
                    generate: Box::new(move || end_front(&paired(e1, e2, false), 2)),
                })
            }
        }

        for e in elements() {
            specs.push(CardSpec {
                name: format!("just_1_{}", element_names[e]),
                generate: Box::new(move || {
                    end_front(
                        &format!(
                            "{}\n{}",
                            &element_g[e](
                                end_graphic_center + V2::new(0.0, graphic_rad * 0.23),
                                0.83
                            ),
                            &just_1(element_colors_bold(e))
                        ),
                        3,
                    )
                }),
            })
        }
        for e1 in elements() {
            for e2 in elements() {
                for e3 in elements() {
                    if e1 > e2 || e2 > e3 {
                        continue;
                    }
                    let tilt = -TAU / 24.0;
                    let arc = TAU / 3.0;
                    let r = graphic_rad * 0.48;
                    let scale = 0.5;

                    specs.push(CardSpec {
                        name: format!(
                            "{}_{}_{}",
                            element_names[e1], element_names[e2], element_names[e3]
                        ),
                        generate: Box::new(move || {
                            end_front(
                                &format!(
                                    "{}{}{}",
                                    element_g[e1](
                                        end_graphic_center + from_angle_mag(tilt, r),
                                        scale
                                    ),
                                    element_g[e2](
                                        end_graphic_center + from_angle_mag(tilt + arc, r),
                                        scale
                                    ),
                                    element_g[e3](
                                        end_graphic_center + from_angle_mag(tilt + arc * 2.0, r),
                                        scale
                                    ),
                                ),
                                2,
                            )
                        }),
                    });
                }
            }
        }

        for e in elements() {
            specs.push(CardSpec {
                name: format!("max_{}_cluster", element_names[e]),
                generate: Box::new(move || {
                    end_front(
                        &format!(
                            "{}{}",
                            big_splat(element_colors_back[e]),
                            element_g[e](end_graphic_center, 0.7),
                        ),
                        1,
                    )
                }),
            });
        }

        for e in elements() {
            specs.push(CardSpec {
                name: format!("forbid_{}", element_names[e]),
                generate: Box::new(move || {
                    end_front(
                        &format!("{}{}", element_g[e](end_graphic_center, 1.0), negatory(),),
                        7,
                    )
                }),
            })
        }

        for e in elements() {
            specs.push(CardSpec {
                name: format!("forbid_{}", element_names[e]),
                generate: Box::new(move || {
                    end_front(
                        &format!("{}{}", element_g[e](end_graphic_center, 1.0), negatory(),),
                        13,
                    )
                }),
            });
        }

        for e1 in elements() {
            for e2 in elements() {
                if e1 < e2 {
                    continue;
                }
                specs.push(CardSpec {
                    name: format!("forbid_{}_{}", element_names[e1], element_names[e2]),
                    generate: Box::new(move || {
                        end_front(&format!("{}{}", paired(e1, e2, true), negatory()), 9)
                    }),
                });
            }
        }

        for spec in specs.iter() {
            write(
                Path::new(&format!("{}.svg", &spec.name)),
                &(spec.generate)(),
            )
            .unwrap();
        }

        std::env::set_current_dir("../").unwrap();
    }
}
