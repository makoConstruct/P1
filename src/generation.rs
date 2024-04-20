use std::{cell::RefCell, f64::consts::TAU, rc::Rc};

use noisy_float::prelude::*;

use mako_infinite_shuffle::{Cross, Indexing, Once};
use rand::{distributions::OpenClosed01, seq::SliceRandom, Rng, SeedableRng};

use super::*;

pub fn end_specs(all_assets: &Rc<Assets>) -> Vec<CardGen> {
    let all_assets = all_assets.clone();
    let mut specs: Vec<CardGen> = Vec::new();
    let assets = all_assets.clone();

    specs.push(CardGen {
        min_count: 7,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let scores = "1".to_string();
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("1_{}", ELEMENT_NAMES[e]),
                    Rc::new(Displaying(move |w| {
                        (ELEMENT_G[e])(END_GRAPHIC_CENTER, 1.0, w)
                    })),
                    scores.clone(),
                    2,
                    format!("{} point for every {}", &scores, ELEMENT_NAMES[e]),
                    vec![e],
                    1,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 19,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_nonequal_pairing().into_map({
            let all_assets = all_assets.clone();
            move |(e1, e2)| {
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("adjacent_{}_{}", ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]),
                    Rc::new(Displaying(move |w| paired(e1, e2, false, w))),
                    "2".to_string(),
                    1,
                    format!(
                        "2 points for every adjacent pairing of {} and {}",
                        ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]
                    ),
                    vec![e1, e2],
                    1,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let scores = "6".to_string();
                let ename = ELEMENT_NAMES[e];
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("just_1_{}", ename),
                    Rc::new(Displaying({let all_assets = all_assets.clone(); move |w| {
                        let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                        let sc = bounds.span().x;
                        let er = sc*0.35;
                        let hspan = sc*0.65;
                        let b = bounds.grav_point(MIDDLE_BOTTOM) + V2::new(0.0, -sc*0.06);
                        let linel = hspan*0.3;
                        all_assets.element(e).by_grav_rad(b + V2::new(0.0, -(linel + er)), MIDDLE_MIDDLE, er, w);
                        underline(element_colors_bold(e), b, MIDDLE_BOTTOM, hspan, w);
                    }})),
                    scores.clone(),
                    1,
                    format!(
                        "{} points as long as there is only one {ename} at the end",
                        &scores
                    ),
                    vec![e],
                    1,
                    false,
                )
            }
        })),
    });
    
    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let scores = "5".to_string();
                let ename_plural = ELEMENT_NAMES_PLURAL[e];
                let ename = ELEMENT_NAMES[e];
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("just_2_{}", ename),
                    Rc::new(Displaying({let all_assets = all_assets.clone(); move |w| {
                        let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                        let sc = bounds.span().x;
                        let er = sc*0.23;
                        let hspan = sc*0.65;
                        let linel = hspan*0.3;
                        let sep = linel*0.6;
                        let out = sep/2.0 + er;
                        let b = bounds.grav_point(MIDDLE_BOTTOM) + V2::new(0.0, -sc*0.155);
                        all_assets.element(e).by_grav_rad(b + V2::new( out, -(linel + er)), MIDDLE_MIDDLE, er, w);
                        all_assets.element(e).by_grav_rad(b + V2::new(-out, -(linel + er)), MIDDLE_MIDDLE, er, w);
                        underline(element_colors_bold(e), b, MIDDLE_BOTTOM, hspan, w);
                    }})),
                    scores.clone(),
                    1,
                    format!(
                        "{} points as long as there are exactly 2 {ename_plural} at the end",
                        &scores
                    ),
                    vec![e],
                    1,
                    false,
                )
            }
        })),
    });
    
    specs.push(CardGen {
        min_count: 6,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_nonequal_triple().into_map({
            let all_assets = all_assets.clone();
            move |(e1, e2, e3)| {
                let tilt = -TAU / 24.0;
                let arc = TAU / 3.0;
                let r = GRAPHIC_RAD * 0.48;
                let scale = 0.5;

                let scores = "4".to_string();

                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!(
                        "triple_{}_{}_{}",
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
                    1,
                    format!(
                        "{} points for every trio of adjacent {}, {} and {}",
                        &scores, ELEMENT_NAMES[e1], ELEMENT_NAMES[e2], ELEMENT_NAMES[e3]
                    ),
                    vec![e1, e2, e3],
                    1,
                    true,
                )
            }
        })),
    });

    specs.push(CardGen { min_count: 4, desired_proportion: 0.0, generator: Box::new(each_unordered_nonequal_triple().into_map({
        let all_assets = all_assets.clone();
        move |(e1, eroad, e3)| {
            let tilt = -TAU / 24.0;
            let arc = TAU / 3.0;
            let r = GRAPHIC_RAD * 0.48;
            let scale = 0.5;
            let e1np = ELEMENT_NAMES_PLURAL[e1];
            let e3np = ELEMENT_NAMES_PLURAL[e3];
            let eroadn = ELEMENT_NAMES[eroad];

            let scores = "1".to_string();

            CardSpec::end_card_with_back_blurred_message(
                &all_assets,
                format!(
                    "road_{}_{}_{}",
                    ELEMENT_NAMES[e1], ELEMENT_NAMES[eroad], ELEMENT_NAMES[e3]
                ),
                Rc::new(Displaying({let all_assets = all_assets.clone(); move |w| {
                    let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                    road_blob_rad(&all_assets, e1, e3, eroad, bounds, w);
                }})),
                format!("{}", &scores),1,
                format!(
                    "for all {e1np} and {e3np} on the banks of a clump of {eroadn}, score the number of {e1np} multiplied by the number of {e3np}"
                ),
                vec![e1,eroad,e3],
                2,
                true
            )
        }
    }))});

    specs.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(elements().into_map({let all_assets = all_assets.clone(); move|e|{
        let element_name = ELEMENT_NAMES[e];
        let element_name_plural = ELEMENT_NAMES_PLURAL[e];
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
            "1".to_string(),1,
            format!("1 point for every {element_name} in the largest connected cluster of {element_name_plural}"),
            vec![e],
            1, false
        )
    }}))});

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let ename = ELEMENT_NAMES[e];
                let ename_plural = ELEMENT_NAMES_PLURAL[e];
                let scores = "11".to_string();
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("forbid_{ename}"),
                    Rc::new(Displaying({
                        let assets = all_assets.clone();
                        move |w: &mut dyn Write| {
                            assets
                                .element(e)
                                .centered_rad(END_GRAPHIC_CENTER, BIG_ELEMENT_RAD, w);
                            assets.negatory.centered_rad(
                                END_GRAPHIC_CENTER,
                                BIG_ELEMENT_RAD * 0.74,
                                w,
                            );
                        }
                    })),
                    scores.clone(),
                    1,
                    format!(
                        "{} points, only if there are no {ename_plural} at all",
                        &scores
                    ),
                    vec![opposite_element(e)],
                    1,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 13,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_nonequal_pairing().into_map({
            let all_assets = all_assets.clone();
            move |(e1, e2)| {
                let scores = "9".to_string();
                let en1 = ELEMENT_NAMES[e1];
                let en1_plural = ELEMENT_NAMES_PLURAL[e1];
                let en2 = ELEMENT_NAMES[e2];
                let en2p = ELEMENT_NAMES_PLURAL[e2];
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("forbid_{en1}_{en2}"),
                    Rc::new(Displaying({
                        let assets = all_assets.clone();
                        move |w: &mut dyn Write| {
                            paired(e1, e2, true, w);
                            assets.negatory.centered_rad(
                                end_graphic_usual_bounds().center(),
                                BIG_ELEMENT_RAD * 0.74,
                                w,
                            );
                            // negatory(w);
                        }
                    })),
                    scores.clone(),
                    1,
                    format!(
                        "{} as long as there are no {en1_plural} adjacent to {en2p} at the end",
                        &scores
                    ),
                    vec![opposite_element(e1), opposite_element(e2)],
                    2,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_pairing().into_map({
            let assets = all_assets.clone();
            move |(e1, e2)| {
                let scores = "3".to_string();
                let en1 = ELEMENT_NAMES[e1];
                let en2 = ELEMENT_NAMES[e2];
                CardSpec::end_card_with_back_blurred_message(
                    &assets,
                    format!("without {en2} {en1}"),
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
                    1,
                    format!("{scores} points per {en1} that is not adjacent to {en2}",),
                    vec![e1, opposite_element(e2)],
                    1,
                    false,
                )
            }
        })),
    });

    fn from_asset(
        assets: &Rc<Assets>,
        asset: &Asset,
        name: String,
        repeat: usize,
        scores: String,
        description: String,
        elements_positive: Vec<ElementTag>,
        level: usize,
        clown: bool,
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
            repeat,
            description,
            elements_positive,
            level,
            clown,
        )
    }

    specs.push(CardGen {
        min_count: 1,
        desired_proportion: 0.0,
        generator: Box::new(Once(from_asset(
            &all_assets,
            &assets.dead_guy2,
            String::from("scavenger"),
            5,
            "4".to_string(),
            String::from("You gain 4 points for each agent who is killed"),
            vec![],
            1,
            false,
        ))),
    });
    specs.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Once(from_asset(&all_assets, &assets.altruism, String::from("altruism"), 2, "=".to_string(), String::from("Your values encompass the values of others.\n\nThis drive scores the sum of the scores of others"), vec![], 1, true)))});

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new({
            let assets = all_assets.clone();
            each_unordered_nonopposite_unequal_pair().into_map(move |(e1, e2)| {
                let e1n = ELEMENT_NAMES[e1];
                let e2n = ELEMENT_NAMES[e2];
                CardSpec::end_card_with_back_blurred_message(
                    &assets,
                    format!("patch either {e1n}, {e2n}"),
                    Rc::new({
                        let assets = assets.clone();
                        Displaying(move |w| {
                            dual_color_patch(&assets, e1, e2, end_graphic_usual_bounds(), w);
                        })
                    }),
                    "1".to_string(),
                    1,
                    format!(
                        "1 point for every {e1n} or {e2n} in the largest contiguous patch of them."
                    ),
                    vec![e1, e2],
                    2,
                    false,
                )
            })
        }),
    });

    specs.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new({let all_assets = all_assets.clone(); Once(CardSpec::end_card_with_back_blurred_message(
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
        "+=".to_string(), 2,
        "You share the desires of other players, but only when they're adjacent to you. eg, if someone standing next to you wants forests, so do you. But if they move away, you will stop caring about forests.".to_string(),
        vec![],
        2,
        true
    ))})});

    specs
}

//todo refactor to return an array of writer generators
pub fn means_specs(all_assets: &Rc<Assets>) -> Vec<CardGen> {
    //this made a borrow check error far less mysterious
    let all_assets = all_assets.clone();
    let mut r: Vec<CardGen> = Vec::new();

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(elements().into_map({let assets = all_assets.clone(); move|e|{
        let bounds = means_graphic_usual_bounds();
        let element_name = ELEMENT_NAMES[e];
        CardSpec::means_card(
            &assets,
            format!("ambush from {element_name}"),
            None,
            1, false, 1,
            vec![(Kill, vec![e])],
            {
                let assets = assets.clone();
                let bounds = bounds.clone();
                Rc::new(move |w| {
                    let sd = bounds.span().min();
                    let rad = sd * 0.22;
                    let sep = rad * 0.26;
                    let (mut c1, mut c2) = tilted_pair(bounds.center() + V2::new(0.0, bounds.span().y*0.13), (rad*2.0 + sep)/2.0);
                    std::mem::swap(&mut c1.y, &mut c2.y);
                    let ea = assets.element(e);
                    let ba = &assets.blank;
                    ea.by_grav(c1, MIDDLE_MIDDLE, rad / (ea.bounds.x / 2.0), w);
                    ba.by_grav(c2, MIDDLE_MIDDLE, rad / (ba.bounds.x / 2.0), w);
                    let guyscale = 0.9;
                    guy2_mage(&assets, c1, guyscale, w);
                    guy2_dead(&assets, c2, guyscale, w);
                })
            },
            format!("standing in a {element_name}, kill one agent in the same land, or an adjacent land"),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(element_primaries().into_map({let assets = all_assets.clone(); move|e|{
        let bounds = means_graphic_usual_bounds();
        let pair_name = pair_name_for(e.0);
        let pair_name_escaped = pair_name.replace('/', " or ");
        CardSpec::means_card(
            &assets,
            format!("ambush beside {pair_name}"),
            Some(format!("ambush beside {pair_name_escaped}")),
            1, false, 1,
            vec![(Kill,vec![e.0, e.1])],
            {
                let assets = assets.clone();
                let bounds = bounds.clone();
                Rc::new(move |w| {
                    let sd = bounds.span().min();
                    let ea = assets.element_both(e.0);
                    let ba = &assets.blank;
                    let bdc = bounds.center() + V2::new(0.0, sd*0.19);
                    let bdr = sd*0.368;
                    let adjr = sd*0.19;
                    let adjc = bounds.ul + both_dims(0.05*sd) + both_dims(adjr);
                    ea.centered_rad(adjc, adjr, w);
                    ba.centered_rad(bdc, bdr, w);
                    guy2_mage(&assets, bdc + V2::new(0.0, -bdr*0.11), 1.0, w);
                    guy2_dead(&assets, bdc + V2::new(0.0, bdr*0.57), 1.0, w);
                })
            },
            format!("standing on or adjacent to {pair_name}, standing with an agent, kill that agent"),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(elements().into_map({
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
                1, false, 1,
                vec![(Change, vec![e])],
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let f = all_assets.flip_to(e);
                        let fr = f.bounds.min() / 2.0;
                        f.centered(center, 1.0, w);
                        let eyr = fr * 0.3;
                        let sep = fr * 0.1;
                        let eyc = center + from_angle_mag(TAU * 1.0 / 3.0, eyr + sep + fr);
                        guy2(&all_assets, eyc, 1.0, w);
                        // all_assets.guy2.centered_rad(eyc, eyr, w);
                    }
                }),
                format!("standing on or adjacent to {opposite_name}, transition it to {element_name}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 19, desired_proportion: 0.0, generator: Box::new(Cross(elements(), elements()).into_map({
        let all_assets = all_assets.clone();
        move |(e, supporting_element)| {
            let element_name = ELEMENT_NAMES[e];
            let supporting_element_name = ELEMENT_NAMES[supporting_element];
            let opposite = opposite_element(e);
            let opposite_name = ELEMENT_NAMES[opposite];
            let center = card_upper_center();
            CardSpec::means_card(
                &all_assets,
                format!("transit {element_name}"),
                Some(format!("transit {element_name} {supporting_element_name}")),
                1, false, 1,
                vec![(Change, vec![e])],
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let f = all_assets.flip_to(e);
                        let fr = f.bounds.min() / 2.0 * 0.86;
                        f.centered_rad(center, fr, w);
                        let eyr = fr * 0.5;
                        let sep = fr * 0.17;
                        let eyc = center + from_angle_mag(TAU * 1.0 / 3.0, eyr + sep + fr);
                        all_assets.element(supporting_element).centered_rad(eyc, eyr, w);
                        all_assets.guy2.by_anchor_rad(eyc, eyr*0.77, w);
                        // all_assets.guy2.centered_rad(eyc, eyr, w);
                    }
                }),
                format!("standing in {supporting_element_name}, transit an adjacent {opposite_name} to {element_name}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Cross(elements(), elements()).into_map({
        let assets = all_assets.clone();
        move |(ef, et)| {
            let efn = ELEMENT_NAMES[ef];
            let etn = ELEMENT_NAMES[et];
            let eto = opposite_element(et);
            let eton = ELEMENT_NAMES[eto];
            let efo = opposite_element(ef);
            let efon = ELEMENT_NAMES[efo];
            CardSpec::means_card(
                &assets,
                format!("{etn} catastrophe"),
                Some(format!("distance flip {efn} {etn}")),
                2, false, 1,
                vec![(Kill, vec![et]), (Change, vec![ef])],
                Rc::new({
                    let assets = assets.clone();
                    move |w| {
                        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                        let sd = bounds.span().min();
                        // len = 4*er + 3*sep + 4*sr
                        // er = sr*ratio
                        // therefore
                        let kill_diamond_radius = sd*0.38;
                        let kill_zone_radius = kill_diamond_radius*0.68;
                        let kill_zone_y = bounds.ul.y + kill_diamond_radius*0.9;
                        let cast_zone_radius = sd*0.245;
                        let cast_zone_y = kill_zone_y + kill_diamond_radius - kill_diamond_radius*0.1 + cast_zone_radius*2.3;
                        let mage_radius = cast_zone_radius*1.14;
                        let cx = bounds.center().x;
                        let kill_zone_center = V2::new(cx, kill_zone_y);
                        assets.flip_to(et).centered_rad(kill_zone_center, kill_zone_radius, w);
                        let cast_zone_center = V2::new(cx, cast_zone_y);
                        assets.flip_to(ef).centered_rad(cast_zone_center, cast_zone_radius, w);
                        assets.guy2_mage.by_anchor_rad(cast_zone_center, mage_radius, w);
                        assets.kill_diamond.centered_rad(kill_zone_center, kill_diamond_radius, w);
                    }
                }),
                format!("Standing in {efon}, flip it, and flip a {eton} up to three lands away, killing everything on or adjacent to the distant {eton}"),
            )
        }
    }))});

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let assets = all_assets.clone();
            move |e| {
                let enp = ELEMENT_NAMES_PLURAL[e];
                CardSpec::means_card(
                    &assets,
                    format!("dominion over {enp}"),
                    None,
                    1,
                    false,
                    1,
                    vec![(Kill, vec![e])],
                    Rc::new({
                        let asset = assets.clone();
                        move |w| {
                            let br = BIG_ELEMENT_RAD * 0.8;
                            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                            asset.element(e).centered_rad(bounds.center(), br, w);
                            asset
                                .kill_diamond
                                .centered_rad(bounds.center(), br * 1.2, w);
                        }
                    }),
                    format!("kill any number of agents standing on any of the {enp}."),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Cross(element_primaries(), elements()).into_map({let assets=all_assets.clone(); move |((e,eo), a)|{
        let enp = pair_name_for(e);
        let an = ELEMENT_NAMES[a];
        let escaped_an = an.replace('/', " or ");
        CardSpec::means_card(
            &assets,
            format!("dominion over {enp}"),
            Some(format!("dominion over {escaped_an}")),
            1, false, 1,
            vec![(Kill, vec![e, eo, a])],
            Rc::new({let asset = assets.clone(); move |w| {
                let br = BIG_ELEMENT_RAD*0.8;
                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                let ar = br*0.4;
                let sep = br*0.14;
                let ac = bounds.center() + from_angle_mag(TAU*3.0/8.0*0.87, br + sep + ar);
                asset.element_both(e).centered_rad(bounds.center(), br, w);
                asset.kill_diamond.centered_rad(bounds.center(), br*1.2, w);
                asset.element(a).centered_rad(ac, ar, w);
            }}),
            format!("kill any number of agents standing on any {enp} land that's adjacent to {an}."),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Cross(elements(), elements()).into_map({let assets=all_assets.clone(); move |(ek, es)|{
        let ekn = ELEMENT_NAMES[ek];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("tyranny"),
            Some(format!("tyranny {ekn} {esn}")),
            1, true, 1,
            vec![(Kill, vec![es, ek])],
            Rc::new({let asset = assets.clone(); move |w| {
                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                let sd = bounds.span().min();
                let br = sd*0.32;
                let dr = br*1.82;
                let ar = br*0.52;
                let sep = br*0.14;
                let bc = bounds.center();
                let ac = bc + from_angle_mag(TAU*3.0/8.0, br + sep + ar);
                asset.element(ek).centered_rad(bc, br, w);
                asset.element(es).centered_rad(ac, ar, w);
                // asset.kill_diamond.centered_rad(bc, dr, w);
                // asset.kill_diamond.centered_rad(bc, dr*1.4, w);
                asset.double_diamond.centered_rad(bc, dr, w);
                guy2(&asset, bc, 1.0, w);
            }}),
            format!("standing in {ekn}, adjacent to {esn}, kill an agent within two land tiles of where you stand."),
        )
    }}))});
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Cross(elements(), elements()).into_map({let assets=all_assets.clone(); move |(ek, es)|{
        let ekn = ELEMENT_NAMES[ek];
        let est = opposite_element(es);
        let estn = ELEMENT_NAMES[est];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("tyrant shot"),
            Some(format!("tyrant shot {ekn} {esn}")),
            2, false, 1,
            vec![(Kill, vec![est, ek]), (Change, vec![est])],
            Rc::new({let asset = assets.clone(); move |w| {
                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                let sd = bounds.span().min();
                let br = sd*0.32;
                let dr = br*1.82;
                let ar = br*0.52;
                let sep = br*0.14;
                let bc = bounds.center();
                let ac = bc + from_angle_mag(TAU*3.0/8.0, br + sep + ar);
                asset.element(ek).centered_rad(bc, br, w);
                asset.flip_to(es).centered_rad(ac, ar, w);
                // asset.kill_diamond.centered_rad(bc, dr, w);
                // asset.kill_diamond.centered_rad(bc, dr*1.4, w);
                asset.double_diamond.centered_rad(bc, dr, w);
                guy2(&asset, bc, 1.0, w);
            }}),
            format!("standing in {ekn} adjacent to {estn}, kill an agent within two land tiles of where you stand, flipping the {estn}."),
        )
    }}))});

    r.push(CardGen { min_count: 5, desired_proportion: 0.0, generator: Box::new(Cross(elements(), elements()).into_map({let assets=all_assets.clone(); move |(ek, es)|{
        let ekn = ELEMENT_NAMES[ek];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("domain"),
            Some(format!("domain {ekn} {esn}")),
            1, true, 1,
            vec![],
            Rc::new({let asset = assets.clone(); move |w| {
                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                let sd = bounds.span().min();
                let br = sd*0.3;
                let sep = br*0.15;
                let bc = bounds.center();
                let hr = br * 0.24;
                let hrs = hr*0.3;
                let mut hs = HexSpiral::new().layer_iter(2);
                hs.next();
                let mut cur_layer = hs.0.layer;
                let mut i = 0;
                let mut el_pos = None;
                while let Some(c) = hs.next() {
                    let first_layer_distance = br + sep + hr + (cur_layer - 1) as f64*(hr*2.0 + sep);
                    let spacing = first_layer_distance / (cur_layer as f64);
                    let p = bc + hexify(c.to_v2())*spacing;
                    if i == 1 {
                        el_pos = Some(p);
                    } else{
                        asset.darker_blank.centered_rad(p, hrs, w);
                    }
                    cur_layer = hs.0.layer;
                    i += 1;
                }
                asset.element(es).centered_rad(el_pos.unwrap(), hr*2.3, w);
                asset.element(ek).centered_rad(bc, br, w);
                guy2(&asset, bc, 1.0, w);
            }}),
            format!("standing in {ekn} adjacent to {esn}, flip a land within a 2 land radius."),
        )
    }}))});

    r.push(CardGen {
        min_count: 9,
        desired_proportion: 0.0,
        generator: Box::new(Cross(elements(), elements()).into_map({
            let assets = all_assets.clone();
            move |(ek, es)| {
                let ekn = ELEMENT_NAMES[ek];
                let esn = ELEMENT_NAMES[es];
                CardSpec::means_card(
                    &assets,
                    format!("domain"),
                    Some(format!("domain smaller {ekn} {esn}")),
                    1,
                    false,
                    1,
                    vec![],
                    Rc::new({
                        let asset = assets.clone();
                        move |w| {
                            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                            let sd = bounds.span().min();
                            let br = sd * 0.3;
                            let sep = br * 0.15;
                            let bc = bounds.center();
                            let hr = br * 0.24;
                            let hrs = hr * 0.3;
                            let mut hs = HexSpiral::new().layer_iter(1);
                            hs.next();
                            let mut cur_layer = hs.0.layer;
                            let mut i = 0;
                            let mut el_pos = None;
                            let esr = hr * 2.3;
                            while let Some(c) = hs.next() {
                                let first_layer_distance =
                                    br + sep + esr + (cur_layer - 1) as f64 * (hr * 2.0 + sep);
                                let spacing = first_layer_distance / (cur_layer as f64);
                                let p = bc + hexify(c.to_v2()) * spacing;
                                if i == 1 {
                                    el_pos = Some(p);
                                } else {
                                    asset.darker_blank.centered_rad(p, hrs, w);
                                }
                                cur_layer = hs.0.layer;
                                i += 1;
                            }
                            asset.element(es).centered_rad(el_pos.unwrap(), esr, w);
                            asset.element(ek).centered_rad(bc, br, w);
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekn} adjacent to {esn}, flip an adjacent land."
                    ),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let assets = all_assets.clone();
            move |ek| {
                let ekn = ELEMENT_NAMES[ek];
                CardSpec::means_card(
                    &assets,
                    format!("domain"),
                    Some(format!("domain {ekn}")),
                    1,
                    true,
                    1,
                    vec![],
                    Rc::new({
                        let asset = assets.clone();
                        move |w| {
                            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                            let sd = bounds.span().min();
                            let br = sd * 0.3;
                            let sep = br * 0.14;
                            let bc = bounds.center();
                            asset.element(ek).centered_rad(bc, br, w);
                            // asset.element(es).centered_rad(ac, ar, w);
                            let hr = br * 0.24;
                            let hrs = hr * 0.337;

                            let mut hs = HexSpiral::new().layer_iter(2);
                            hs.next();
                            let mut cur_layer = hs.0.layer;
                            while let Some(c) = hs.next() {
                                let first_layer_distance =
                                    br + sep + hr + (cur_layer - 1) as f64 * (hr * 2.0 + sep);
                                let spacing = first_layer_distance / (cur_layer as f64);
                                asset.darker_blank.centered_rad(
                                    bc + hexify(c.to_v2()) * spacing,
                                    hrs,
                                    w,
                                );
                                cur_layer = hs.0.layer;
                            }
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekn}, flip a land within a 2 land radius."
                    ),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(elements().into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let opposite = opposite_element(e);
        let opposite_name = ELEMENT_NAMES[opposite];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        let total_y_span = bounds.span().y;
        let eye_space = total_y_span*0.138;
        // let ecb = element_colors_bold(e); //it would be nice to color the arrows this way, alas
        let fr = total_y_span*0.2;
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
            Some(format!("wind {opposite_name}")),
            2, false, 1,
            vec![(Change, vec![e]), (Move, vec![e])],
            Rc::new({let all_assets = all_assets.clone(); move |w| {
                let bc = V2::new(cx, fcy);
                let f = all_assets.flip_to(e);
                f.centered_rad(bc, fr, w);
                all_assets.step.centered_rotated(V2::new(cx, a1cy), ascale, -TAU/4.0, w);
                // all_assets.step.centered_rotated(V2::new(cx, a1cy), 1.0, TAU/4.0, w);
                all_assets.step.centered_rotated(V2::new(cx, a2cy), ascale, -TAU/4.0, w);
                let ger = bounds.span().min()*0.174;
                let sep = ger*0.37;
                let gc = bc + from_angle_mag(TAU*3.0/8.0, ger + sep + fr);
                all_assets.blank.centered_rad(gc, ger, w);
                all_assets.guy2.by_anchor_rad(gc, ger*0.77, w);
            }}),
            format!("transition an adjacent {opposite_name} to {element_name}, moving each occupant of that land up to two lands"),
        )
    }}))});

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(each_nonequal_element().into_map({
            let assets = all_assets.clone();
            move |(eho, ek)| {
                let eh = opposite_element(eho);
                let ekn = ELEMENT_NAMES[ek];
                let eka = ELEMENT_ARTICLE[ek];
                let ehn = ELEMENT_NAMES[eh];
                let ehon = ELEMENT_NAMES[eho];
                CardSpec::means_card(
                    &assets,
                    "domain blast".to_string(),
                    Some(format!("domain_blast_from_{}_to_{}", ehn, ekn)),
                    2,
                    false,
                    1,
                    vec![(Kill, vec![eh, ek]), (Change, vec![eh])],
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
                            assets.flip_to(eh).centered(hc, hr / FLIP_RINGS_RAD, w);
                            assets
                                .element(ek)
                                .by_grav(kc, MIDDLE_MIDDLE, kr / BIG_ELEMENT_RAD, w);
                            guy2_dead(&assets, kc, 1.0, w);
                            // guylike(&assets.dead_guy, kc, 1.0, w);
                        }
                    }),
                    format!("flip any {ehon} that's adjacent to {eka} {ekn}, killing anyone on an adjacent {ekn}"),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(elements().into_map({
            let assets = all_assets.clone();
            move |e| {
                let en = ELEMENT_NAMES[e];
                let ea = ELEMENT_ARTICLE[e];
                CardSpec::means_card(
                    &assets,
                    format!("{en} travel"),
                    Some(format!("travel {en}")),
                    1,
                    false,
                    1,
                    vec![(Move, vec![e])],
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
                    format!("send an agent standing on {ea} {en} to any other {en}"),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(each_unordered_nonequal_pairing().into_map({let all_assets = all_assets.clone(); move|(e,et)|{
        let en = ELEMENT_NAMES[e];
        let eo = opposite_element(e);
        let eon = ELEMENT_NAMES[eo];
        let etn = ELEMENT_NAMES[et];
        CardSpec::means_card(
            &all_assets,
            format!("{eon} liftoff"),
            Some(format!("liftoff {en} {etn}")),
            2, false, 1,
            vec![(Change, vec![e]), (Move, vec![e, et])],
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
            format!("pick any {eon}, and send anyone standing on it to any {etn}, flipping the {eon} to {en}"),
        )
    }}))});

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(Once(CardSpec::means_card(
            &all_assets,
            "step".to_string(),
            None,
            1,
            true,
            4,
            vec![],
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
        ))),
    });

    fn double_constrained_flip(
        assets: &Rc<Assets>,
        e1p: ElementTag,
        e1: ElementTag,
        e2: ElementTag,
    ) -> CardSpec {
        let e1oa = ELEMENT_ARTICLE[opposite_element(e1)];
        let e1on = ELEMENT_NAMES[opposite_element(e1)];
        let e2on = ELEMENT_NAMES[opposite_element(e2)];
        let e2oa = ELEMENT_ARTICLE[opposite_element(e2)];
        let e1pn = ELEMENT_NAMES[e1p];
        let e1n = ELEMENT_NAMES[e1];
        let e2n = ELEMENT_NAMES[e2];
        let assets = assets.clone();
        CardSpec::means_card(&assets, format!("{} surge", e1on), Some(format!("surge {e1pn} {e1n} {e2n}")), 
        2, true, 1, vec![(Change, vec![e1, e2])], Rc::new({let assets = assets.clone(); move |w|{
            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
            let fr = bounds.span().min()*0.24;
            let sep = bounds.span().min()*0.057;
            let e2y = bounds.ul.y + fr;
            let e1y = e2y + 2.0*fr + sep;
            let e1c = V2::new(bounds.center().x, e1y);
            let adylr = bounds.span().min()*0.17;
            assets.element(e1p).centered_rad(e1c + from_angle_mag(TAU/2.0 - TAU*0.13, fr + sep + adylr), adylr, w);
            assets.flip_to(e1).centered_rad(e1c, fr, w);
            assets.flip_to(e2).centered_rad(V2::new(bounds.center().x, e2y), fr, w);
            let gc = e1c + from_angle_mag(TAU/12.0, fr + sep + GUY2_RAD);
            assets.blank.centered_rad(gc, adylr, w);
            guy2_flipped(&assets, gc, GUY2_ADJACENCY_SMALLERNESS, w);
        }}), format!("standing adjacent to {e1oa} {e1on} that's also adjacent to {e1pn} and {e2oa} {e2on}, flip the {e1on} and the {e2on}"))
    }
    r.push(CardGen {
        min_count: 4,
        desired_proportion: 0.0,
        generator: Box::new(Cross(each_nonequal_element(), elements()).into_map({
            let all_assets = all_assets.clone();
            move |((e1, e2), e1p)| double_constrained_flip(&all_assets, e1p, e1, e2)
        })),
    });

    r.push(CardGen {
        min_count: 9,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_triple().into_map({
            let assets = all_assets.clone();
            move |(ae, be, ce)| {
                let aen = ELEMENT_NAMES[ae];
                let ben = ELEMENT_NAMES[be];
                let cen = ELEMENT_NAMES[ce];
                CardSpec::means_card(
                    &assets,
                    "flip all".to_string(),
                    Some(format!("flip_all_{aen}_{ben}_{cen}")),
                    2,
                    true,
                    1,
                    vec![(
                        Change,
                        vec![
                            opposite_element(ae),
                            opposite_element(be),
                            opposite_element(ce),
                        ],
                    )],
                    Rc::new({
                        let assets = assets.clone();
                        move |w| {
                            let fae = assets.flip_to(ae);
                            let fbe = assets.flip_to(be);
                            let fce = assets.flip_to(ce);

                            let tilt = -TAU / 24.0 + TAU / 2.0;
                            let arc = TAU / 3.0;
                            let r = GRAPHIC_RAD * 0.5;
                            let scale = 0.5;
                            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                            let c = bounds.center();

                            fae.centered(c + from_angle_mag(tilt, r), scale, w);
                            fbe.centered(c + from_angle_mag(tilt + arc, r), scale, w);
                            fce.centered(c + from_angle_mag(tilt + arc * 2.0, r), scale, w);
                        }
                    }),
                    format!("standing on any chain of {aen}, {ben}, {cen}, flip all at once."),
                )
            }
        })),
    });
    
    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(each_unordered_pairing().into_map({
            let assets = all_assets.clone();
            move |(ae, be)| {
                let aen = ELEMENT_NAMES[ae];
                let ben = ELEMENT_NAMES[be];
                CardSpec::means_card(
                    &assets,
                    "flip both".to_string(),
                    Some(format!("flip_both_{aen}_{ben}")),
                    2,
                    true,
                    1,
                    vec![(
                        Change,
                        vec![
                            opposite_element(ae),
                            opposite_element(be),
                        ],
                    )],
                    Rc::new({
                        let assets = assets.clone();
                        move |w| {
                            let fae = assets.flip_to(ae);
                            let fbe = assets.flip_to(be);
                            let r = 0.55*BIG_ELEMENT_RAD;
                            let sep = 0.08*BIG_ELEMENT_RAD;
                            let arc = from_angle_mag(-TAU/12.0, r + sep);
                            let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                            fae.centered_rad(bounds.center() + arc, r, w);
                            fbe.centered_rad(bounds.center() - arc, r, w);
                        }
                    }),
                    format!("standing on any pair of {aen} and {ben}, flip both."),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 1, desired_proportion: 0.0, generator: Box::new({let all_assets = all_assets.clone(); Once(CardSpec::means_card(
        &all_assets,
        "atoms for something else".to_string(), None,
        2, true, 2,
        vec![(Kill, vec![ICE_I])],
        Rc::new({
            let all_assets = all_assets.clone();
            move |w| {
                let bounds = means_graphic_usual_bounds();
                let sd = bounds.span().min();
                let c = bounds.center();
                let lateral = sd*0.23;
                let downward = sd*0.3;
                let guy_scale = 0.85;
                all_assets.element(ICE_I).centered_rad(c + V2::new(0.0, downward), downward*0.9, w);
                all_assets.guy2_mage.by_anchor(c + V2::new(-lateral, 0.0), guy_scale, w);
                horizontal_flip(&all_assets.cubed_guy2).by_anchor(c + V2::new(lateral, 0.0), guy_scale, w);
            }
        }),
        "Adjacent to ice that is also adjacent to an opponent's agent, capture that agent and replace their agent with a spare of your own. All of your agents can now use that player's abilities.".to_string(),
    ))})});

    r
}

pub fn land_specs(assets: &Rc<Assets>, repeating: &[u8]) -> Vec<CardGen> {
    assert_eq!(repeating.len(), 4);
    let mut r: Vec<CardGen> = Vec::new();
    fn side(assets: Rc<Assets>, e: ElementTag) -> Rc<impl Fn(&mut dyn Write)> {
        let rad = land_hex_usual_bounds().span().min()/2.0*0.98;
        Rc::new(move |w: &mut dyn Write| {
            land_hex_svg_outer(
                &Displaying({
                    let assets = assets.clone();
                    move |w| assets.element(e).centered_rad(land_hex_usual_bounds().center(), rad, w)
                }),
                ELEMENT_COLORS_BACK[e],
                w,
            )
        })
    }
    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(element_primaries().into_map({
            let assets = assets.clone();
            let repeatings = Vec::from(repeating);
            move |(e, eo)| CardSpec {
                name: format!("land_{}_{}", ELEMENT_NAMES[e], ELEMENT_NAMES[eo]),
                repeat: repeatings[e/2] as usize,
                level: 1,
                frequency_modifier: 1.0,
                properties: vec![],
                generate_front: side(assets.clone(), e),
                generate_back: side(assets.clone(), eo),
            }
        })),
    });
    r
}
// pub fn gen_board(radius:usize, to:&dyn Write)

// allocates each i a portion of the seats corresponding to its weight. A random process could be used for tiebreaking the remainders, but instead we allocate from largest remaining weight to smallest.
pub fn weights_to_cuts(weights: &[f64], seats: usize) -> Vec<usize> {
    // basically, it assigns remainders from smallest to largest
    let total: f64 = weights.iter().sum();
    assert!(weights.len() > 0);
    if total == 0.0 {
        let divo = seats / weights.len();
        let mut cuts: Vec<usize> = (0..weights.len()).map(|_| divo).collect();
        let mut i = 0;
        let mut seats_allocated = divo * weights.len();
        while seats_allocated < seats {
            cuts[i] += 1;
            seats_allocated += 1;
            i = (i + 1) % cuts.len();
        }
        cuts
    } else {
        let mut seats_allocated = 0;
        let (mut cuts, mut remainders): (Vec<usize>, Vec<(N64, usize)>) = weights
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let dec = if total != 0.0 {
                    (seats as f64) * c / total
                } else {
                    0.0
                };
                let rounded = dec.floor() as usize;
                seats_allocated += rounded;
                (rounded, (-n64(dec - rounded as f64), i))
            })
            .unzip();
        remainders.sort();
        while seats_allocated < seats {
            let (_, i) = remainders.pop().unwrap();
            seats_allocated += 1;
            cuts[i] += 1;
        }
        cuts
    }
}

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
    let cuts = weights_to_cuts(&weights, count);
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
