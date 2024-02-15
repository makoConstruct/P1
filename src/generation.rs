use std::{cell::RefCell, f64::consts::TAU, rc::Rc};

use noisy_float::prelude::*;

use rand::{seq::SliceRandom, Rng, SeedableRng};
use mako_infinite_shuffle::{Cross, Indexing, Once};

use super::*;

pub fn end_specs(all_assets: &Rc<Assets>) -> Vec<CardGen> {
    let all_assets = all_assets.clone();
    let mut specs: Vec<CardGen> = Vec::new();
    let assets = all_assets.clone();

    specs.push(Box::new(elements().into_map({let all_assets = all_assets.clone(); move|e| {
        let scores = "1".to_string();
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            format!("{}_1", ELEMENT_NAMES[e]),
            Rc::new(Displaying(move |w| {
                (ELEMENT_G[e])(END_GRAPHIC_CENTER, 1.0, w)
            })),
            scores.clone(),
            format!("{} point for every {}", &scores, ELEMENT_NAMES[e]),
            1,
        )
    }})));

    specs.push(Box::new(each_unordered_nonequal_pairing().into_map(
        {let all_assets = all_assets.clone(); move|(e1, e2)| {
            CardSpec::end_card_with_back_blurred_message(
                &all_assets,
                format!("{}_{}", ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]),
                Rc::new(Displaying(move |w| paired(e1, e2, false, w))),
                "2".to_string(),
                format!(
                    "2 points for every adjacent pairing of {} and {}",
                    ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]
                ),
                1,
            )
        }},
    )));

    specs.push(Box::new(elements().into_map({let all_assets = all_assets.clone(); move|e| {
        let scores = "6".to_string();
        let ename = ELEMENT_NAMES[e];
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            format!("just_1_{}", ename),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}\n{}",
                    &Displaying(|w: &mut dyn Write| ELEMENT_G[e](
                        END_GRAPHIC_CENTER + V2::new(0.0, GRAPHIC_RAD * 0.23),
                        0.83,
                        w
                    )),
                    &Displaying(|w: &mut dyn Write| just_1(element_colors_bold(e), w))
                )
                .unwrap();
            })),
            scores.clone(),
            format!(
                "{} points as long as there is only one {ename} at the end",
                &scores
            ),
            1,
        )
    }})));
    specs.push(Box::new(each_unordered_nonequal_triple().into_map(
        {let all_assets = all_assets.clone(); move|(e1, e2, e3)| {
            let tilt = -TAU / 24.0;
            let arc = TAU / 3.0;
            let r = GRAPHIC_RAD * 0.48;
            let scale = 0.5;

            let scores = "4".to_string();

            CardSpec::end_card_with_back_blurred_message(
                &all_assets,
                format!(
                    "{}_{}_{}",
                    ELEMENT_NAMES[e1], ELEMENT_NAMES[e2], ELEMENT_NAMES[e3]
                ),
                Rc::new(Displaying(move |w| {
                    write!(
                        w,
                        "{}{}{}",
                        &Displaying(|w: &mut dyn Write| ELEMENT_G[e1](
                            END_GRAPHIC_CENTER + from_angle_mag(tilt, r),
                            scale,
                            w
                        )),
                        &Displaying(|w: &mut dyn Write| ELEMENT_G[e2](
                            END_GRAPHIC_CENTER + from_angle_mag(tilt + arc, r),
                            scale,
                            w
                        )),
                        &Displaying(|w: &mut dyn Write| ELEMENT_G[e3](
                            END_GRAPHIC_CENTER + from_angle_mag(tilt + arc * 2.0, r),
                            scale,
                            w
                        )),
                    )
                    .unwrap();
                })),
                scores.clone(),
                format!(
                    "{} points for every trio of adjacent {}, {} and {}",
                    &scores, ELEMENT_NAMES[e1], ELEMENT_NAMES[e2], ELEMENT_NAMES[e3]
                ),
                1,
            )
        }},
    )));

    specs.push(Box::new(elements().into_map({let all_assets = all_assets.clone(); move|e|{
        let element_name = ELEMENT_NAMES[e];
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            format!("max_{}_cluster", element_name),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w:&mut dyn Write| big_splat(ELEMENT_COLORS_BACK[e], w)),
                    &Displaying(|w:&mut dyn Write| ELEMENT_G[e](END_GRAPHIC_CENTER, 0.7, w)),
                ).unwrap();
            })),
            "1".to_string(),
            format!("1 point for every {element_name} in the largest connected cluster of {element_name}s"),
            1
        )
    }})));

    specs.push(Box::new(elements().into_map({let all_assets = all_assets.clone(); move|e| {
        let ename = ELEMENT_NAMES[e];
        let scores = "11".to_string();
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            format!("forbid_{ename}"),
            Rc::new(Displaying(move |w: &mut dyn Write| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w: &mut dyn Write| ELEMENT_G[e](END_GRAPHIC_CENTER, 1.0, w)),
                    &Displaying(negatory),
                )
                .unwrap();
            })),
            scores.clone(),
            format!("{} points, only if there are no {ename}s at all", &scores),
            1,
        )
    }})));

    specs.push(Box::new(each_unordered_nonequal_pairing().into_map(
        {let all_assets = all_assets.clone(); move|(e1, e2)| {
            let scores = "9".to_string();
            let en1 = ELEMENT_NAMES[e1];
            let en2 = ELEMENT_NAMES[e2];
            CardSpec::end_card_with_back_blurred_message(
                &all_assets,
                format!("forbid_{en1}_{en2}"),
                Rc::new(Displaying(move |w: &mut dyn Write| {
                    paired(e1, e2, true, w);
                    negatory(w);
                })),
                scores.clone(),
                format!(
                    "{} as long as there are no {en1}s adjacent to {en2} at the end",
                    &scores
                ),
                2,
            )
        }},
    )));

    specs.push(Box::new(each_unordered_pairing().into_map({
        let assets = all_assets.clone();
        move |(e1, e2)| {
            let scores = "3".to_string();
            let en1 = ELEMENT_NAMES[e1];
            let en2 = ELEMENT_NAMES[e2];
            CardSpec::end_card_with_back_blurred_message(
                &assets,
                format!("{en1} without {en2}"),
                Rc::new(Displaying({
                    let assets = assets.clone();
                    move |w: &mut dyn Write| {
                        let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                        let e1a = assets.element(e1);
                        let e2a = assets.element(e2);
                        let negatory = &assets.negatory;
                        let er = e1a.bounds.min() / 2.0;
                        let sep = er * 0.23;
                        let our = er * 0.57;
                        let negatory_scale = our / er;
                        let arc = er + sep + our;
                        let d = er - our;
                        let ad = (arc * arc - d * d).sqrt();
                        let total_height = er + ad + our;
                        let total_scale = bounds.span().y / total_height;
                        let cx = bounds.center().x;
                        let e1c = V2::new(er, er);
                        let e2c = V2::new(er * 2.0 - our, er + ad);
                        let offset =
                            bounds.center() + V2::new(-er, -total_height / 2.0) * total_scale;
                        e1a.centered_rad(offset + e1c * total_scale, er * total_scale, w);
                        e2a.centered_rad(offset + e2c * total_scale, our * total_scale, w);
                        negatory.centered(
                            offset + e2c * total_scale,
                            negatory_scale * total_scale,
                            w,
                        );
                    }
                })),
                scores.clone(),
                format!("{scores} points per {en1} that is not adjacent to a {en2}",),
                1,
            )
        }
    })));

    fn from_asset(
        assets: &Rc<Assets>,
        asset: &Asset,
        name: String,
        scores: String,
        description: String,
        level: usize,
    ) -> CardSpec {
        CardSpec::end_card_with_back_blurred_message(
            assets,
            name.clone(),
            Rc::new({
                let asset = asset.clone();
                Displaying(move |w| {
                    asset.center_in_bounds(end_graphic_usual_bounds_shrunk_appropriately(), w)
                })
            }),
            scores,
            description,
            level,
        )
    }

    specs.push(Box::new(Once(from_asset(
        &all_assets,
        &assets.kill,
        String::from("kill"),
        "3".to_string(),
        String::from("You gain 3 points for each piece who is killed"),
        1,
    ))));
    specs.push(Box::new(Once(from_asset(&all_assets, &assets.altruism, String::from("altruism"), "".to_string(), String::from("Your values encompass the values of others.\n\nYour score will be the sum of the scores of others"), 2))));

    specs.push(Box::new({
        let assets = all_assets.clone();
        each_unordered_nonequal_pairing().into_map(move |(e1, e2)| {
            let e1n = ELEMENT_NAMES[e1];
            let e2n = ELEMENT_NAMES[e2];
            CardSpec::end_card_with_back_blurred_message(
                &assets,
                format!("{e1n} and {e2n} patch"),
                Rc::new({
                    let assets = assets.clone();
                    Displaying(move |w| {
                        dual_color_patch(&assets, e1, e2, end_graphic_usual_bounds(), w);
                    })
                }),
                "1".to_string(),
                format!(
                    "1 point for every {e1n} or {e2n} in the largest contiguous patch of them."
                ),
                2,
            )
        })
    }));

    specs.push(Box::new({let all_assets = all_assets.clone(); Once(CardSpec::end_card_with_back_blurred_message(
        &all_assets,
        "dog altruism".to_string(),
        Rc::new(Displaying({
            let all_assets = all_assets.clone();
            move |w| {
                all_assets.dog_altruism.centered(
                    end_graphic_usual_bounds().center(),
                    1.0,
                    w,
                );
            }
        })),
        "+=".to_string(),
        "You share the desires of other players, but only when they're adjacent to you. For instance, if someone standing next to you wants to establish forests, so do you. But if they move away, you will stop caring about forests.".to_string(),
        2,
    ))}));

    specs
}

//todo refactor to return an array of writer generators
pub fn means_specs(all_assets: &Rc<Assets>) -> Vec<CardGen> {
    //this made a borrow check error far less mysterious
    let all_assets = all_assets.clone();
    let mut r: Vec<CardGen> = Vec::new();
    let assets = all_assets.clone();

    r.push(Box::new(elements().into_map({let assets = all_assets.clone(); move|e|{
        let bounds = means_graphic_usual_bounds();
        let element_name = ELEMENT_NAMES[e];
        CardSpec::means_card(
            &assets,
            format!("ambush from {element_name}"),
            None,
            1,
            {
                let assets = assets.clone();
                let bounds = bounds.clone();
                Rc::new(move |w| {
                    let sd = bounds.span().min();
                    let sep = sd * 0.265;
                    let rad = sep * 0.9;
                    let (c1, c2) = tilted_pair(bounds.center(), sep);
                    let ea = assets.element(e);
                    let ba = &assets.blank;
                    ea.by_grav(c1, MIDDLE_MIDDLE, rad / (ea.bounds.x / 2.0), w);
                    ba.by_grav(c2, MIDDLE_MIDDLE, rad / (ba.bounds.x / 2.0), w);
                    let guyscale = 1.2;
                    guylike(&assets.guy, c1, guyscale, w);
                    guylike(&assets.dead_guy, c2, guyscale, w);
                })
            },
            format!("standing in a {element_name}, kill one piece in the same land, or an adjacent land"),
        )
    }})));

    r.push(Box::new(elements().into_map({
        let all_assets = all_assets.clone();
        move |e| {
            let element_name = ELEMENT_NAMES[e];
            let opposite = opposite_element(e);
            let opposite_name = ELEMENT_NAMES[opposite];
            let center = card_upper_center();
            CardSpec::means_card(
                &all_assets,
                format!("transit {element_name}"),
                None,
                1,
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let f = all_assets.flip_to(e);
                        let fr = f.bounds.min() / 2.0;
                        f.centered(center, 1.0, w);
                        let eyr = fr * 0.3;
                        let sep = fr * 0.1;
                        let eyc = center + from_angle_mag(TAU * 1.0 / 3.0, eyr + sep + fr);
                        all_assets.guyeye.centered_rad(eyc, eyr, w);
                    }
                }),
                format!("transition an adjacent {opposite_name} to a {element_name}"),
            )
        }
    })));

    r.push(Box::new(elements().into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let opposite = opposite_element(e);
        let opposite_name = ELEMENT_NAMES[opposite];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        let total_y_span = bounds.span().y;
        let eye_space = total_y_span*0.06;
        // let ecb = element_colors_bold(e); //it would be nice to color the arrows this way, alas
        let fr = total_y_span*0.23;
        let sep = total_y_span*0.05;
        let arrow_span = (total_y_span - eye_space - fr*2.0 - sep*2.0)/2.0;
        let arrow_rad = arrow_span/2.0;
        let gbasy = bounds.br.y;
        let cx = bounds.center().x;
        let fcy = gbasy - eye_space - fr;
        let a1cy = fcy - fr - sep - arrow_rad;
        let a2cy = a1cy - arrow_rad - sep - arrow_rad;
        let ascale = arrow_rad/(all_assets.step.bounds.x/2.0);
        CardSpec::means_card(
            &all_assets,
            format!("{opposite_name} wind"),
            None,
            2,
            Rc::new({let all_assets = all_assets.clone(); move |w| {
                let bc = V2::new(cx, fcy);
                let f = all_assets.flip_to(e);
                f.centered_rad(bc, fr, w);
                all_assets.step.centered_rotated(V2::new(cx, a1cy), ascale, -TAU/4.0, w);
                // all_assets.step.centered_rotated(V2::new(cx, a1cy), 1.0, TAU/4.0, w);
                all_assets.step.centered_rotated(V2::new(cx, a2cy), ascale, -TAU/4.0, w);
                let ger = bounds.span().min()*0.12;
                let sep = ger*0.37;
                all_assets.guyeye.centered_rad(
                    bc + from_angle_mag(TAU*5.0/12.0, ger + sep + fr),
                    ger,
                    w
                );
            }}),
            format!("transition an adjacent {opposite_name} to a {element_name}, giving any occupants of that land the option of moving up to two lands"),
        )
    }})));

    // this is a bit redundant with the below, and less interesting
    // for eh in elements() {
    //     for ek in elements() {
    //         if ek == eh {
    //             continue;
    //         }
    //         let ekn = element_names[ek];
    //         let ehn = element_names[eh];
    //         r.push(CardSpec::means_card_with_back_blurred_message(
    //             "domain strike".to_string(),
    //             Some(format!("domain_strike_from_{ehn}_to_{ekn}")),
    //             Rc::new({
    //                 let assets = assets.clone();
    //                 move |w| {
    //                     let c = means_graphic_usual_bounds().center();
    //                     let h = means_graphic_usual_bounds().span().y;
    //                     let kr = 0.23 * h;
    //                     let hr = 0.14 * h;
    //                     let sep = 0.04 * h;
    //                     let th = hr * 2.0 + sep + kr * 2.0;
    //                     let hc = V2::new(c.x, c.y - th / 2.0 + hr);
    //                     let kc = V2::new(c.x, c.y + th / 2.0 - kr);
    //                     assets
    //                         .element(eh)
    //                         .by_grav(hc, MIDDLE_MIDDLE, hr / BIG_ELEMENT_RAD, w);
    //                     assets
    //                         .element(ek)
    //                         .by_grav(kc, MIDDLE_MIDDLE, kr / BIG_ELEMENT_RAD, w);
    //                     guylike(&assets.dead_guy, kc, 1.0, w);
    //                 }
    //             }),
    //             format!("choose a {ekn} that's adjacent to a {ehn}. kill all those standing in the {ekn}"),
    //         ))
    //     }
    // }

    r.push(Box::new(each_nonequal_element().into_map({
        let assets = all_assets.clone();
        move |(eho, ek)| {
            let eh = opposite_element(eho);
            let ekn = ELEMENT_NAMES[ek];
            let ehn = ELEMENT_NAMES[eh];
            let ehon = ELEMENT_NAMES[eho];
            CardSpec::means_card(
                &assets,
                "domain blast".to_string(),
                Some(format!("domain_blast_from_{}_to_{}", ehn, ekn)),
                2,
                Rc::new({
                    let assets = assets.clone();
                    move |w| {
                        let c = means_graphic_usual_bounds().center();
                        let h = means_graphic_usual_bounds().span().y;
                        let kr = 0.23 * h;
                        let hr = 0.14 * h;
                        let sep = 0.04 * h;
                        let th = hr * 2.0 + sep + kr * 2.0;
                        let hc = V2::new(c.x, c.y - th / 2.0 + hr);
                        let kc = V2::new(c.x, c.y + th / 2.0 - kr);
                        flipping_to(&assets, eh, hc, hr / FLIP_RINGS_RAD, w);
                        // assets
                        //     .element(eh)
                        //     .by_grav(hc, MIDDLE_MIDDLE, hr / BIG_ELEMENT_RAD, w);
                        assets
                            .element(ek)
                            .by_grav(kc, MIDDLE_MIDDLE, kr / BIG_ELEMENT_RAD, w);
                        guylike(&assets.dead_guy, kc, 1.0, w);
                    }
                }),
                format!("flip any {ehon} to a {ehn}, killing anyone on an adjacent {ekn}"),
            )
        }
    })));

    r.push(Box::new(elements().into_map({
        let assets = all_assets.clone();
        move |e| {
            let en = ELEMENT_NAMES[e];
            CardSpec::means_card(
                &assets,
                format!("{en} travel"),
                None,
                1,
                Rc::new({
                    let assets = assets.clone();
                    move |w| {
                        come_on_down_specifically(
                            assets.element(e),
                            assets.element(e),
                            e,
                            means_graphic_usual_bounds(),
                            w,
                        );
                    }
                }),
                format!("send any one person standing on a {en} to any other {en}"),
            )
        }
    })));

    r.push(Box::new(each_unordered_nonequal_pairing().into_map({let all_assets = all_assets.clone(); move|(e,et)|{
        let en = ELEMENT_NAMES[e];
        let eo = opposite_element(e);
        let eon = ELEMENT_NAMES[eo];
        let etn = ELEMENT_NAMES[et];
        CardSpec::means_card(
            &all_assets,
            format!("{en} liftoff"),
            Some(format!("{en} {etn} liftoff")),
            2,
            Rc::new({
                let all_assets = all_assets.clone();
                move |w| {
                    come_on_down_specifically(
                        all_assets.flip_to(e),
                        all_assets.element(et),
                        e,
                        means_graphic_usual_bounds(),
                        w,
                    );
                }
            }),
            format!("pick any {eon}, and send anyone standing on it to any {etn}, flipping the {eon} to a {en}"),
        )
    }})));

    r.push(Box::new(Once(CardSpec::means_card_repeated(
        &all_assets,
        "step".to_string(),
        None,
        8,
        1,
        Rc::new({
            let all_assets = all_assets.clone();
            move |w| {
                all_assets.step.centered(
                    means_graphic_usual_bounds_shrunk_appropriately().center(),
                    1.0,
                    w,
                );
            }
        }),
        "move to an adjacent land".to_string(),
    ))));

    //This requires defs, or altering the dual element assets to not use defs. They weren't good abilities anyway.
    // for ep in elements() {
    //     for (ett, eto) in element_primaries() {
    //         if ett == ep || eto == ep { continue; }
    //         let pn = element_names[ep];
    //         let epn = element_pair_names[ett/2];
    //         r.push(CardSpec::means_card_with_back_blurred_message(
    //             format!("birth {pn}"), Some(format!("birth {pn} over {epn}")),
    //             Rc::new({let assets = assets.clone(); move|w| overplace(&assets.back_colored_circle, assets.element(ep), assets.element_both(ett), means_graphic_usual_bounds(), w)}),
    //             format!("")
    //         ));
    //     }
    // }
    fn double_constrained_flip(
        assets: &Rc<Assets>,
        e1p: ElementTag,
        e1: ElementTag,
        e2: ElementTag,
    ) -> CardSpec {
        let e1on = ELEMENT_NAMES[opposite_element(e1)];
        let e2on = ELEMENT_NAMES[opposite_element(e2)];
        let e1pn = ELEMENT_NAMES[e1p];
        let e1n = ELEMENT_NAMES[e1];
        let e2n = ELEMENT_NAMES[e2];
        let assets = assets.clone();
        CardSpec::means_card(&assets, format!("{} surge", e1n), Some(format!("surge {e1pn} {e1n} {e2n}")), 
        2,Rc::new({let assets = assets.clone(); move |w|{
            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
            let fr = bounds.span().min()*0.24;
            let sep = bounds.span().min()*0.057;
            let e2y = bounds.ul.y + fr;
            let e1y = e2y + 2.0*fr + sep;
            let e1c = V2::new(bounds.center().x, e1y);
            let guyeye = &assets.guyeye;
            let adylr = bounds.span().min()*0.17;
            assets.element(e1p).centered_rad(e1c + from_angle_mag(TAU/2.0 - TAU*0.13, fr + sep + adylr), adylr, w);
            assets.guyeye.centered(e1c + from_angle_mag(TAU/12.0, fr + sep + guyeye.bounds.min()/2.0), 1.0, w);
            assets.flip_to(e1).centered_rad(e1c, fr, w);
            assets.flip_to(e2).centered_rad(V2::new(bounds.center().x, e2y), fr, w);
        }}), format!("standing adjacent to a {e1on} that's also adjacent to da {e1pn} and a {e2on}, flip the {e1on} and the {e2on}"))
    }
    r.push(Box::new(Cross(each_nonequal_element(), elements()).into_map({
        let all_assets = all_assets.clone();
        move |((e1, e2), e1p)| double_constrained_flip(&all_assets, e1p, e1, e2)
    })));

    r
}

pub fn land_specs(assets: &Rc<Assets>) -> Vec<CardGen> {
    let mut r: Vec<CardGen> = Vec::new();
    fn side(assets: Rc<Assets>, e: ElementTag) -> Rc<impl Fn(&mut dyn Write)> {
        let scale = CARD_DIMENSIONS.min() / 2.0 / BIG_ELEMENT_RAD * 0.86;
        Rc::new(move |w: &mut dyn Write| {
            blank_front(
                &Displaying({
                    let assets = assets.clone();
                    move |w| {
                        assets
                            .element(e)
                            .centered(CARD_DIMENSIONS / 2.0, scale, w)
                    }
                }),
                ELEMENT_COLORS_BACK[e],
                true,
                w,
            )
        })
    }
    r.push(Box::new(element_primaries().into_map({
        let assets = assets.clone();
        move |(e, eo)| CardSpec {
            name: format!("land_{}_{}", ELEMENT_NAMES[e], ELEMENT_NAMES[eo]),
            repeat: 1,
            generate_front: side(assets.clone(), e),
            generate_back: side(assets.clone(), eo),
        }
    })));
    r
}
// pub fn gen_board(radius:usize, to:&dyn Write)

pub fn generate_board(
    assets: &Rc<Assets>,
    weights: &Vec<f64>,
    radius: usize,
    suppress_voids: bool,
    seed: u64,
    w: &mut dyn Write,
) {
    // let lc = land_specs(assets);
    // let cards: Vec<CardSpec> = lc.into_iter().next().unwrap().collect();
    // forest/field, mountain/volcano, lake/ice, tomb/void
    // figure out the card counts based on the weights
    let count = {
        let mut t = 1;
        for i in 0..radius {
            t += (i + 1) * 6;
        }
        t
    };
    let total = weights.iter().fold(0.0, |a, b| a + b);
    let (mut cuts, remainders): (Vec<usize>, Vec<N64>) = weights
        .iter()
        .map(|c| {
            let dec = (count as f64) * c / total;
            let rounded = dec.floor();
            (rounded as usize, N64::from_f64(dec - rounded))
        })
        .unzip();
    let mut ct = cuts.iter().fold(0, |a, b| a + b);
    let mut remi: Vec<(N64, usize)> = remainders
        .iter()
        .enumerate()
        .map(|(a, b)| (*b, a))
        .collect();
    // remi.sort_by(|a,b| b.partial_cmp(&a).unwrap());
    remi.sort();
    while ct < count {
        let (_, i) = remi.pop().unwrap();
        ct += 1;
        cuts[i] += 1;
    }
    fn shuffled_land_tiles(
        land_card_counts: &Vec<usize>,
        suppress_voids: bool,
        rng: &mut impl Rng,
    ) -> Vec<ElementTag> {
        let mut rng = RefCell::new(rng);
        let mut r: Vec<usize> = land_card_counts
            .iter()
            .enumerate()
            .flat_map(|(i, c)| {
                (0..*c).map({
                    let mut rng = rng.borrow_mut();
                    move |_| {
                        if (suppress_voids && i == 3) || rng.gen_bool(0.5) {
                            i * 2
                        } else {
                            i * 2 + 1
                        }
                    }
                })
            })
            .collect();
        (&mut r).shuffle::<_>(rng.get_mut());
        r
    }
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let cards = shuffled_land_tiles(&cuts, suppress_voids, &mut rng);

    let sep = BIG_ELEMENT_RAD * 2.25;
    let span = both_dims(sep * (1 + 1 + 2 * radius) as f64);
    let center = span / 2.0;
    do_sheet(
        span,
        &Displaying(|w| {
            for (p, e) in HexSpiral::new()
                .layer_iter(radius)
                .map(|c| hexify(c.to_v2()))
                .zip(cards.iter())
            {
                assets
                    .element(*e)
                    .centered_rad(p.yx() * sep + center, BIG_ELEMENT_RAD, w);
            }
        }),
        w,
    );
}
