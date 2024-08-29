use std::{cell::RefCell, f64::consts::TAU, rc::Rc};

use noisy_float::prelude::*;

use mako_infinite_shuffle::{IndexVec, Indexing, Once};
use rand::{seq::SliceRandom, Rng, SeedableRng};

use super::*;

pub fn end_specs(all_assets: &Rc<Assets>) -> Vec<CardGen> {
    let all_assets = all_assets.clone();
    let mut specs: Vec<CardGen> = Vec::new();
    let assets = all_assets.clone();

    specs.push(CardGen {
        min_count: 7,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![FIELD, FOREST, LAKE, ICE, MOUNTAIN, VOLCANO, TOMB]).into_map({
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
                        1,
                        format!("{} point for every {}", &scores, ELEMENT_NAMES_SINGULAR[e]),
                        vec![e],
                        1,
                        false,
                    )
                }
            }),
        ),
    });

    specs.push(CardGen {
        min_count: 19,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (FIELD, TOMB),
                (MOUNTAIN, LAKE),
                (LAKE, TOMB),
                (FOREST, ICE),
                (FIELD, VOLCANO),
                (FIELD, FOREST),
                (MOUNTAIN, FIELD),
            ])
            .into_map({
                let all_assets = all_assets.clone();
                move |(e1, e2)| {
                    CardSpec::end_card_with_back_blurred_message(
                        &all_assets,
                        format!("adjacent_{}_{}", ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]),
                        // Rc::new(Displaying(move |w| paired(e1, e2, false, w))),
                        Rc::new(Displaying({
                            let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                            let assets = all_assets.clone();
                            move |w| {
                                joined_pair_graphic_horizontal(
                                    &assets,
                                    e1,
                                    e2,
                                    bounds.center(),
                                    bounds.span().x / 2.0,
                                    w,
                                )
                            }
                        })),
                        "2".to_string(),
                        1,
                        format!(
                            "2 points for every adjacent pairing of {} and {}",
                            ELEMENT_NAMES[e1], ELEMENT_NAMES[e2]
                        ),
                        vec![e1, e2],
                        0,
                        false,
                    )
                }
            }),
        ),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![VOLCANO, VOID, LAKE]).into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let scores = "14".to_string();
                let ename = ELEMENT_NAMES[e];
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("just_1_{}", ename),
                    Rc::new(Displaying({
                        let all_assets = all_assets.clone();
                        move |w| {
                            let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                            let sc = bounds.span().x;
                            let er = sc * 0.35;
                            let hspan = sc * 0.65;
                            let b = bounds.grav_point(MIDDLE_BOTTOM) + V2::new(0.0, -sc * 0.06);
                            let linel = hspan * 0.3;
                            all_assets.element(e).by_grav_rad(
                                b + V2::new(0.0, -(linel + er)),
                                MIDDLE_MIDDLE,
                                er,
                                w,
                            );
                            underline(element_color_bold(e), b, MIDDLE_BOTTOM, hspan, w);
                        }
                    })),
                    scores.clone(),
                    1,
                    format!(
                        "{} points if there's exactly one {ename} at the end",
                        &scores
                    ),
                    vec![e],
                    0,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![(MOUNTAIN, FIELD, LAKE), (VOLCANO, VOLCANO, FOREST), (VOID, MOUNTAIN, TOMB)]).into_map({
            let all_assets = all_assets.clone();
            move |(a,b,c)| {
                let aname = ELEMENT_NAMES[a];
                let bname = ELEMENT_NAMES[b];
                let cname = ELEMENT_NAMES[c];
                let scores = "1×3".to_string();
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("chain {aname} {bname} {cname}"),
                    Rc::new(Displaying({
                        let all_assets = all_assets.clone();
                        move |w| {
                            let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                            chain_graphic(&all_assets, a, b, c, bounds.center(), bounds.span().x/2.0, w);
                        }
                    })),
                    scores.clone(),
                    1,
                    format!(
                        "1 point for each land included in a chain of {aname}, {bname}, {cname} (multiple chains may overlap, but don't count any land more than once)",
                    ),
                    vec![a,b,c],
                    2,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![FIELD, LAKE, FOREST]).into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let scores = "13".to_string();
                let ename_plural = ELEMENT_NAMES_PLURAL[e];
                let ename = ELEMENT_NAMES[e];
                CardSpec::end_card_with_back_blurred_message(
                    &all_assets,
                    format!("just_2_{}", ename),
                    Rc::new(Displaying({
                        let all_assets = all_assets.clone();
                        move |w| {
                            let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                            let sc = bounds.span().x;
                            let er = sc * 0.23;
                            let hspan = sc * 0.65;
                            let linel = hspan * 0.3;
                            let sep = linel * 0.6;
                            let out = sep / 2.0 + er;
                            let b = bounds.grav_point(MIDDLE_BOTTOM) + V2::new(0.0, -sc * 0.155);
                            all_assets.element(e).by_grav_rad(
                                b + V2::new(out, -(linel + er)),
                                MIDDLE_MIDDLE,
                                er,
                                w,
                            );
                            all_assets.element(e).by_grav_rad(
                                b + V2::new(-out, -(linel + er)),
                                MIDDLE_MIDDLE,
                                er,
                                w,
                            );
                            underline(element_color_bold(e), b, MIDDLE_BOTTOM, hspan, w);
                        }
                    })),
                    scores.clone(),
                    1,
                    format!(
                        "{} points as long as there are exactly 2 {ename_plural} at the end",
                        &scores
                    ),
                    vec![e],
                    0,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 3,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (TOMB, VOLCANO, ICE),
                (LAKE, LAKE, MOUNTAIN),
                (VOLCANO, LAKE, FIELD),
                (MOUNTAIN, FOREST, VOLCANO),
            ])
            .into_map({
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
                            "{} points for every triangle of adjacent {}, {} and {}",
                            &scores, ELEMENT_NAMES[e1], ELEMENT_NAMES[e2], ELEMENT_NAMES[e3]
                        ),
                        vec![e1, e2, e3],
                        2,
                        false,
                    )
                }
            }),
        ),
    });
    
    specs.push(CardGen { min_count: 5, desired_proportion: 0.0, generator: Box::new({let all_assets = all_assets.clone(); IndexVec(vec![()]).into_map(move |()| {
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            "interventionist".to_string(),
            Rc::new({
                let all_assets = all_assets.clone();
                Displaying(move |w| {
                    all_assets.interventionist_helix.centered(end_graphic_usual_bounds_shrunk_appropriately().center(), 1.0, w);
                })
            }),
            "8-1".into(),
            1,
            format!("You care about every little thing. Score 8 points for each lake, 7 for each tomb, and so on; mountain:6, volcano:5, ice:4, forest:3, void:2, field:1, and score 2 points for very surviving agent."),
            vec![FIELD, FOREST, MOUNTAIN, VOLCANO, LAKE, ICE, TOMB, VOID], 2, true,
        )
    })})});

    specs.push(CardGen { min_count: 2, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(ICE, FIELD, TOMB), (VOLCANO, MOUNTAIN, FOREST)]).into_map({
        let all_assets = all_assets.clone();
        move |(e1, eroad, e3)| {
            let e1np = ELEMENT_NAMES_PLURAL[e1];
            let e3np = ELEMENT_NAMES_PLURAL[e3];
            let eroadn = ELEMENT_NAMES[eroad];

            let scores = "×".to_string();

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

    specs.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![VOID, FOREST, VOLCANO]).into_map({let all_assets = all_assets.clone(); move|e|{
        let element_name = ELEMENT_NAMES[e];
        let element_name_plural = ELEMENT_NAMES_PLURAL[e];
        CardSpec::end_card_with_back_blurred_message(
            &all_assets,
            format!("max_{}_cluster", element_name),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w:&mut dyn Write| big_splat(element_color_back(e), w)),
                    &Displaying(|w:&mut dyn Write| ELEMENT_G[e](END_GRAPHIC_CENTER, 0.7, w)),
                ).unwrap();
            })),
            "1".to_string(),1,
            format!("1 point for every {element_name} in the single largest connected cluster of {element_name_plural} ({element_name_plural} outside of that cluster is valueless)"),
            vec![e],
            0, false
        )
    }}))});

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![VOID, LAKE]).into_map({
            let all_assets = all_assets.clone();
            move |e| {
                let ename = ELEMENT_NAMES[e];
                let ename_plural = ELEMENT_NAMES_PLURAL[e];
                let scores = "12".to_string();
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
                        "{} points if there are no {ename_plural} at all",
                        &scores
                    ),
                    vec![opposite_element(e)],
                    0,
                    false,
                )
            }
        })),
    });

    specs.push(CardGen {
        min_count: 13,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (LAKE, TOMB),
                (TOMB, VOID),
                (VOLCANO, ICE),
                (VOLCANO, VOID),
                (LAKE, FIELD),
                (LAKE, VOID),
                (MOUNTAIN, MOUNTAIN),
            ])
            .into_map({
                let all_assets = all_assets.clone();
                move |(e1, e2)| {
                    let scores = "10".to_string();
                    let en1 = ELEMENT_NAMES[e1];
                    let en2 = ELEMENT_NAMES[e2];
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
                            "{} points if there is no {en1} adjacent to any {en2} at the end",
                            &scores
                        ),
                        vec![opposite_element(e1), opposite_element(e2)],
                        0,
                        false,
                    )
                }
            }),
        ),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (MOUNTAIN, TOMB),
                (MOUNTAIN, FIELD),
                (TOMB, FOREST),
                (ICE, VOLCANO),
                (VOLCANO, LAKE),
            ])
            .into_map({
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
                                let offset = bounds.center()
                                    + V2::new(-er, -total_height / 2.0) * total_scale;
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
                        0,
                        false,
                    )
                }
            }),
        ),
    });

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator:
            Box::new(
                IndexVec(vec![
                    (VOLCANO, ICE, VOID),
                    (MOUNTAIN, FOREST, LAKE),
                    (TOMB, FOREST, ICE),
                    (ICE, LAKE, FOREST),
                    (FOREST, MOUNTAIN, FOREST),
                ])
                .into_map({
                    let assets = all_assets.clone();
                    move |(e1, e2, e3)| {
                        let scores = "4".to_string();
                        let en1 = ELEMENT_NAMES[e1];
                        let en2 = ELEMENT_NAMES[e2];
                        let en3 = ELEMENT_NAMES[e3];
                        CardSpec::end_card_with_back_blurred_message(
                            &assets,
                            format!("without {en1} {en2} {en3}"),
                            Rc::new(Displaying({
                                let assets = assets.clone();
                                move |w: &mut dyn Write| {
                                    let bounds = end_graphic_usual_bounds_shrunk_appropriately();
                                    let e1a = assets.element(e1);
                                    let e2a = assets.element(e2);
                                    let e3a = assets.element(e3);
                                    let negatory = &assets.negatory;

                                    let bs = bounds.span();
                                    // i initially tried to calculate these by just defining the ratio and asking claude to simplify the formula but it didn't work
                                    // //clopus generated this
                                    // let a = bs.x + (bs.y - bs.x)*ratio;
                                    // let cr = -a/2.0 + ((a*a - 20.0*bs.x*bs.x + 4.0*bs.y*bs.y) / 4.0).sqrt();
                                    // // nope, didn't work
                                    let sm = bs.x * 0.158;
                                    let sep = sm * 0.3;
                                    let len = (bs - both_dims(sm * 2.0)).magnitude();
                                    let cr = (len - 2.0 * (sm + sep)) / 2.0;
                                    let e2c = bounds.ul + both_dims(sm);
                                    let e3c = bounds.br - both_dims(sm);
                                    e2a.centered_rad(e2c, sm, w);
                                    e3a.centered_rad(e3c, sm, w);
                                    let neg_rad = 0.8 * sm;
                                    negatory.centered_rad(e2c, neg_rad, w);
                                    negatory.centered_rad(e3c, neg_rad, w);
                                    e1a.centered_rad(bounds.center(), cr, w);
                                }
                            })),
                            scores.clone(),
                            1,
                            format!(
                                "{scores} points per {en1} that is not adjacent to {en2} or {en3}",
                            ),
                            vec![e1, opposite_element(e2)],
                            2,
                            false,
                        )
                    }
                }),
            ),
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
            3,
            "4".to_string(),
            String::from("a terrible hunger.\n4 points for every corpse in your possession at the end (killing creates corposes, corpses can be stowed as items and carried around)"),
            vec![],
            0,
            false,
        ))),
    });
    specs.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(Once(from_asset(&all_assets, &assets.altruism, String::from("altruism"), 2, "=".to_string(), String::from("Your values encompass the values of others.\n\nScore the sum of the scores of all other agencies"), vec![], 1, true)))});

    specs.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new({
            let assets = all_assets.clone();
            IndexVec(vec![
                (FIELD, ICE),
                (MOUNTAIN, FOREST),
                (FIELD, VOID),
                (VOLCANO, LAKE),
            ])
            .into_map(move |(e1, e2)| {
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
                        "1 point for every {e1n} or {e2n} in the largest connected patch of those land types."
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
        "=".to_string(), 2,
        "You share the desires of the players adjacent to you, but only when they're adjacent to you. As soon as you're apart, you will stop caring about those things.".to_string(),
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

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![FOREST, MOUNTAIN]).into_map({let assets = all_assets.clone(); move|e|{
        let bounds = means_graphic_usual_bounds();
        let element_name = ELEMENT_NAMES[e];
        CardSpec::means_card(
            &assets,
            format!("{element_name} ambush"),
            Some(format!("ambush from {element_name}")),
            0, false, 1,
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
            format!("standing in {element_name}, kill one nearby agent"),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(TOMB, VOID), (FIELD, FOREST), (MOUNTAIN, VOLCANO)]).into_map({let assets = all_assets.clone(); move|e|{
        let bounds = means_graphic_usual_bounds();
        let pair_name = pair_name_for(e.0);
        let pair_name_escaped = pair_name.replace('/', " or ");
        CardSpec::means_card(
            &assets,
            format!("slaying"),
            Some(format!("slaying beside {pair_name_escaped}")),
            0, false, 1,
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
            format!("Kill an agent in the same land as you, when near to {pair_name}"),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![TOMB, MOUNTAIN]).into_map({
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
                0, false, 1,
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
                format!("standing nearby to {opposite_name}, flip it to {element_name}"),
            )
        }
    }))});
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![LAKE]).into_map({
        let all_assets = all_assets.clone();
        move |e| {
            let element_name = ELEMENT_NAMES[e];
            let opposite = opposite_element(e);
            let opposite_name = ELEMENT_NAMES[opposite];
            let center = card_upper_center();
            CardSpec::means_card(
                &all_assets,
                format!("transit {element_name}"),
                Some(format!("transit any {element_name}")),
                1, true, 1,
                vec![(Change, vec![e])],
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let f = all_assets.flip_to(e);
                        f.centered(center, 1.0, w); 
                    }
                }),
                format!("flip any {opposite_name} to {element_name}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(VOLCANO, FOREST), (MOUNTAIN, LAKE), (FIELD, VOLCANO), (MOUNTAIN, VOID)]).into_map({
        let all_assets = all_assets.clone();
        move |(se, re)| {
            let ring_name = ELEMENT_NAMES[re];
            let ring_opposite_name_plural = ELEMENT_NAMES_PLURAL[opposite_element(re)];
            let support_opposite_name = ELEMENT_NAMES[opposite_element(se)];
            CardSpec::means_card(
                &all_assets,
                format!("bloom {ring_name}"),
                None,
                0, false, 1,
                vec![(Change, vec![se, re])],
                Rc::new({
                    let assets = all_assets.clone();
                    move |w| {
                        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                        let sd = bounds.span().x;
                        let center = bounds.center() + V2::new(0.0, sd*0.13);
                        ring_conversion(&*assets, center,  se, re, w);
                        assets.guy2.by_anchor_rad(center, sd*0.13, w);
                    }
                }),
                format!("near to {support_opposite_name}, flip it, and flip all of the {ring_opposite_name_plural} near to you"),
            )
        }
    }))});

    // I'm not sure why I had a second copy of this. I think I wanted there to be a variant that only allowed flipping one in the adjacency ring? But that wouldn't use the same asset. But that's still a good idea (ability that lets you flip two kinds as long as you're adjacent to both of them). Consider it!
    // r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(FIELD, FIELD), (LAKE, TOMB)]).into_map({
    //     let all_assets = all_assets.clone();
    //     move |(se, re)| {
    //         let ring_name = ELEMENT_NAMES[re];
    //         let ring_opposite_name_plural = ELEMENT_NAMES_PLURAL[opposite_element(re)];
    //         let support_opposite_name = ELEMENT_NAMES[opposite_element(se)];
    //         CardSpec::means_card(
    //             &all_assets,
    //             format!("bloom {ring_name}"),
    //             None,
    //             0, false, 1,
    //             vec![(Change, vec![se, re])],
    //             Rc::new({
    //                 let assets = all_assets.clone();
    //                 move |w| {
    //                     let bounds = means_graphic_usual_bounds_shrunk_appropriately();
    //                     let sd = bounds.span().x;
    //                     let center = bounds.center();
    //                     ring_conversion(center, sd/2.0, assets.element(se), element_color_back(re), assets.flip_to(re), w);
    //                     assets.guy2.by_anchor_rad(center, sd*0.13, w);
    //                 }
    //             }),
    //             format!("on or adjacent to {support_opposite_name}, flip all of the {ring_opposite_name_plural} below your feet or adjacent to you"),
    //         )
    //     }
    // }))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(TOMB, FIELD), (MOUNTAIN, MOUNTAIN)]).into_map({
    let all_assets = all_assets.clone();
    move |(a,b)| {
        let an = ELEMENT_NAMES[a];
        let bn = ELEMENT_NAMES[b];
        CardSpec::means_card(
            &all_assets,
            format!("prism"),
            Some(format!("prism {an} {bn}")),
            0, false, 1,
            vec![(Change, vec![])],
            Rc::new({
                let assets = all_assets.clone();
                move |w| {
                    let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                    let c = bounds.center();
                    let sd = bounds.span().x;
                    let sqr3o4: f64 = (3.0 / 4.0 as f64).sqrt();
                    let sepp = 0.1;
                    let r = sd/(2.0 + (2.0 + sepp)*sqr3o4);
                    let side = sd*sepp + r*2.0;
                    let ab = V2::new(0.0, side/2.0);
                    let m = V2::new(bounds.ul.x + r, c.y);
                    let ac = m + ab;
                    let bc = m - ab;
                    let cc = V2::new(bounds.ul.x + r + side*sqr3o4, c.y);
                    assets.blank.centered_rad(cc, r, w);
                    assets.element(a).centered_rad(ac, r, w);
                    assets.element(b).centered_rad(bc, r, w);
                    assets.triangle.by_grav(m, LEFT_MIDDLE, side, w);
                }
            }),
            format!("standing near a pair of {an} and {bn}, one land near to both lands on either side, or one of those lands of the pair, can be flipped"),
        )
    }}))});

    r.push(CardGen { min_count: 19, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![((FIELD, FOREST), TOMB), ((FIELD, FOREST), FOREST), ((MOUNTAIN, VOLCANO), LAKE), ((LAKE, ICE), VOID), ((LAKE, ICE), MOUNTAIN)]).into_map({
        let all_assets = all_assets.clone();
        move |((e, o), supporting_element)| {
            let element_name = ELEMENT_NAMES[e];
            let supporting_element_name = ELEMENT_NAMES[supporting_element];
            let opposite_name = ELEMENT_NAMES[o];
            CardSpec::means_card(
                &all_assets,
                format!("transit {element_name}/{opposite_name}"),
                Some(format!("transit either {element_name} {supporting_element_name}")),
                1, false, 1,
                vec![(Change, vec![e, o])],
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                        joined_pair_verticalish(
                            bounds.center(), bounds.span().x/2.0,
                            &|c, r, w|{
                                all_assets.element(supporting_element).centered_rad(c,r,w);
                                all_assets.guy2.by_anchor(c, 0.75, w);
                            },
                            &|c, r, w|{
                                all_assets.flip_either(e).centered_rad(c, r, w);
                            },
                            element_color_back(supporting_element),
                            element_color_back(e),
                            w
                        );
                    }
                }),
                format!("standing in {supporting_element_name}, flip a nearby {opposite_name}/{element_name}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 19, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(VOLCANO, VOLCANO)]).into_map({
        let all_assets = all_assets.clone();
        move |(e, supporting_element)| {
            let element_name = ELEMENT_NAMES[e];
            let supporting_element_name = ELEMENT_NAMES[supporting_element];
            let opposite = opposite_element(e);
            let opposite_name = ELEMENT_NAMES[opposite];
            CardSpec::means_card(
                &all_assets,
                format!("transit {element_name}"),
                Some(format!("transit {element_name} {supporting_element_name}")),
                0, false, 1,
                vec![(Change, vec![e])],
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                        joined_pair_verticalish(
                            bounds.center(), bounds.span().x/2.0,
                            &|c, r, w|{
                                all_assets.element(supporting_element).centered_rad(c,r,w);
                                all_assets.guy2.by_anchor(c, 0.75, w);
                            },
                            &|c, r, w|{
                                all_assets.flip_to(e).centered_rad(c, r, w);
                            },
                            element_color_back(supporting_element),
                            element_color_back(opposite_element(e)),
                            w
                        );
                    }
                }),
                format!("standing in {supporting_element_name}, flip a nearby {opposite_name} to {element_name}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(LAKE, VOID), (MOUNTAIN, FOREST)]).into_map({
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
                format!("Standing in {efon}, flip it, and flip a {eton} up to three lands away, killing everything near to the distant {eton}"),
            )
        }
    }))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(FIELD, VOID), (LAKE, MOUNTAIN)]).into_map({let assets=all_assets.clone(); move |(e, es)|{
        let en = ELEMENT_NAMES[e];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("dominion around {en}"),
            Some(format!("dominion around {en} {esn}")),
            0, false, 1,
            vec![(Kill, vec![e, es])],
            Rc::new({let asset = assets.clone(); move |w| {
                let br = BIG_ELEMENT_RAD*0.6;
                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                let ar = br*0.55;
                let sep = br*0.14;
                let ac = bounds.center() + from_angle_mag(TAU*3.0/8.0*0.87, br + sep + ar);
                asset.element(e).centered_rad(bounds.center(), br, w);
                asset.element(es).centered_rad(ac, ar, w);
                asset.kill_diamond_around.centered(bounds.center(), 1.0, w);
            }}),
            format!("kill any number of agents standing near to {en} that's adjacent to {esn}."),
        )
    }}))});
    
     r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![ICE]).into_map({
            let assets = all_assets.clone();
            move |e| {
                let enp = ELEMENT_NAMES_PLURAL[e];
                CardSpec::means_card(
                    &assets,
                    format!("dominion over {enp}"),
                    None,
                    0,
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
                    format!("kill any number of agents standing on any {enp}."),
                )
            }
        })),
    });
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![((FIELD, FOREST), VOID), ((LAKE, ICE), MOUNTAIN)]).into_map({let assets=all_assets.clone(); move |((e,eo), a)|{
        let enp = pair_name_for(e);
        let an = ELEMENT_NAMES[a];
        let escaped_enp = enp.replace('/', " or ");
        CardSpec::means_card(
            &assets,
            format!("dominion over {enp}"),
            Some(format!("dominion over {escaped_enp} beside {an}")),
            0, false, 1,
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
            format!("kill any number of agents standing on any {enp} that's adjacent to {an}."),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(TOMB, ICE)]).into_map({let assets=all_assets.clone(); move |(ek, es)|{
        let ekn = ELEMENT_NAMES[ek];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("tyranny"),
            Some(format!("tyranny {ekn} {esn}")),
            0, true, 1,
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
            format!("standing in {ekn}, near to {esn}, kill an agent within two steps of where you stand."),
        )
    }}))});

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(ICE, ICE), (VOLCANO, VOID)]).into_map({let assets=all_assets.clone(); move |(ek, es)|{
        let ekn = ELEMENT_NAMES[ek];
        let est = opposite_element(es);
        let estn = ELEMENT_NAMES[est];
        let esn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &assets,
            format!("tyrant shot"),
            Some(format!("tyrant shot {ekn} {esn}")),
            2, false, 1,
            vec![(Kill, vec![ek]), (Change, vec![est])],
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
            format!("standing in {ekn} near to {estn}, kill an agent within two steps of where you stand, flipping the {estn}."),
        )
    }}))});

    r.push(CardGen {
        min_count: 5,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![(TOMB, LAKE)]).into_map({
            let assets = all_assets.clone();
            move |(ek, es)| {
                let ekn = ELEMENT_NAMES[ek];
                let esn = ELEMENT_NAMES[es];
                CardSpec::means_card(
                    &assets,
                    format!("domain"),
                    Some(format!("domain {ekn} {esn}")),
                    0,
                    true,
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
                            let mut hs = HexSpiral::new().layer_iter(2);
                            hs.next();
                            let mut cur_layer = hs.0.layer;
                            let mut i = 0;
                            let mut el_pos = None;
                            while let Some(c) = hs.next() {
                                let first_layer_distance =
                                    br + sep + hr + (cur_layer - 1) as f64 * (hr * 2.0 + sep);
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
                            asset.element(es).centered_rad(el_pos.unwrap(), hr * 2.3, w);
                            asset.element(ek).centered_rad(bc, br, w);
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekn} near to {esn}, flip a land within a 2 land radius."
                    ),
                )
            }
        })),
    });
    
    r.push(CardGen {
        min_count: 5,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![(TOMB, LAKE)]).into_map({
            let assets = all_assets.clone();
            move |(ek, es)| {
                let ekn = ELEMENT_NAMES[ek];
                let esn = ELEMENT_NAMES[es];
                let eson = ELEMENT_NAMES[opposite_element(es)];
                CardSpec::means_card(
                    &assets,
                    format!("domain burn"),
                    Some(format!("domain burn {ekn} {esn}")),
                    0,
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
                            let mut hs = HexSpiral::new().layer_iter(2);
                            hs.next();
                            let mut cur_layer = hs.0.layer;
                            let mut i = 0;
                            let mut el_pos = None;
                            while let Some(c) = hs.next() {
                                let first_layer_distance =
                                    br + sep + hr + (cur_layer - 1) as f64 * (hr * 2.0 + sep);
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
                            asset.flip_to(es).centered_rad(el_pos.unwrap(), hr * 2.3, w);
                            asset.element(ek).centered_rad(bc, br, w);
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekn} near to {eson}, flip the supporting {eson}, and flip any other land within a 2 land radius."
                    ),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 5,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![VOLCANO]).into_map({
            let assets = all_assets.clone();
            move |ek| {
                let ekn = ELEMENT_NAMES[ek];
                let ekon = ELEMENT_NAMES[opposite_element(ek)];
                CardSpec::means_card(
                    &assets,
                    format!("hot cast"),
                    Some(format!("domain flip {ekn}")),
                    0,
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
                            let mut hs = HexSpiral::new().layer_iter(2);
                            hs.next();
                            let mut cur_layer = hs.0.layer;
                            while let Some(c) = hs.next() {
                                let first_layer_distance =
                                    br + sep + hr + (cur_layer - 1) as f64 * (hr * 2.0 + sep);
                                let spacing = first_layer_distance / (cur_layer as f64);
                                let p = bc + hexify(c.to_v2()) * spacing;
                                asset.darker_blank.centered_rad(p, hrs, w);
                                cur_layer = hs.0.layer;
                            }
                            asset.flip_to(ek).centered_rad(bc, br, w);
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekon}, flip it, and flip one land within a 2 land radius."
                    ),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 9,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![(FOREST, MOUNTAIN)]).into_map({
            let assets = all_assets.clone();
            move |(ek, es)| {
                let ekn = ELEMENT_NAMES[ek];
                let esn = ELEMENT_NAMES[es];
                CardSpec::means_card(
                    &assets,
                    format!("domain"),
                    Some(format!("domain smaller {ekn} {esn}")),
                    0,
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
                    format!("standing in {ekn} near {esn}, flip a nearby land."),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 9,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![(MOUNTAIN, TOMB)]).into_map({
            let assets = all_assets.clone();
            move |(ek, es)| {
                let ekn = ELEMENT_NAMES[ek];
                let esn = ELEMENT_NAMES[es];
                CardSpec::means_card(
                    &assets,
                    format!("domain burn"),
                    Some(format!("domain smaller exchange {ekn} {esn}")),
                    0,
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
                            asset.flip_to(opposite_element(es)).centered_rad(el_pos.unwrap(), esr, w);
                            asset.element(ek).centered_rad(bc, br, w);
                            guy2(&asset, bc, 1.0, w);
                        }
                    }),
                    format!(
                        "standing in {ekn} near {esn}, flip the {esn}, and flip any one other nearby land."
                    ),
                )
            }
        })),
    });

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![VOLCANO]).into_map({
            let assets = all_assets.clone();
            move |ek| {
                let ekn = ELEMENT_NAMES[ek];
                let eka = ELEMENT_ARTICLE[ek];
                CardSpec::means_card(
                    &assets,
                    format!("domain"),
                    Some(format!("domain {ekn}")),
                    0,
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
                    format!("standing on {eka} {ekn}, flip a land within a 2 land radius."),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![MOUNTAIN, VOID]).into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let opposite = opposite_element(e);
        let opposite_name = ELEMENT_NAMES[opposite];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        
        let sd = bounds.span().x;
        let bc = bounds.center() + V2::new(0.0, sd*0.21);
        let fr = sd*0.29;
        let arrow_rad = sd*0.1;
        let arrow_scale = sd*0.1/(all_assets.step.bounds.x/2.0);
        let arsep = sd*0.12;
        let ard = arrow_rad*1.2;
        
        CardSpec::means_card(
            &all_assets,
            format!("{opposite_name} wind"),
            Some(format!("wind flip {opposite_name}")),
            2, false, 1,
            vec![(Change, vec![e]), (Move, vec![e])],
            Rc::new({let assets = all_assets.clone(); move |w| {
                assets.flip_to(e).centered_rad(bc, fr, w);
                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                let aby = bc.y - fr - sd*0.16;
                do_arr(V2::new(bc.x - ard, aby));
                do_arr(V2::new(bc.x + ard, aby - arsep));
                do_arr(V2::new(bc.x - ard, aby - arsep*2.0));
                
                let ger = bounds.span().min()*0.174;
                let sep = ger*0.37;
                let gc = bc + from_angle_mag(TAU*3.0/8.0, ger + sep + fr);
                // assets.blank.centered_rad(gc, ger, w);
                assets.guy2.by_anchor_rad(gc, ger*0.9, w);
            }}),
            format!("flip a nearby {opposite_name} to {element_name}, moving each occupant of that land up to three lands"),
        )
    }}))});
    
    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (ICE, FIELD),
                (FOREST, VOID),
            ])
            .into_map({
                let assets = all_assets.clone();
                move |(ae, be)| {
                    let aen = ELEMENT_NAMES[ae];
                    let ben = ELEMENT_NAMES[be];
                    CardSpec::means_card(
                        &assets,
                        "reaction wind".to_string(),
                        Some(format!("wind_both_{aen}_{ben}")),
                        1,
                        // the ones that just sort of move an element along through a limited substrate without changing the total number of either are extraordinarily weak
                        ae == opposite_element(be),
                        1,
                        vec![(Change, vec![opposite_element(ae), opposite_element(be)])],
                        Rc::new({
                            let assets = assets.clone();
                            move |w| {
                                let bounds = means_graphic_usual_bounds_shrunk_appropriately();
                                let sd = bounds.span().x;
                                joined_pair_graphic_horizontal(
                                    &*assets,
                                    ae,
                                    be,
                                    bounds.center() + V2::new(0.0, sd*0.33),
                                    sd / 2.0,
                                    w,
                                );
                                let arrow_scale = 0.5;
                                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                                let arsyh = sd*0.2;
                                let arsxh = sd*0.32;
                                // let arb = bounds.center() + V2::new(sd*0.28, 0.0);
                                let arb = bounds.center() + V2::new(arsxh/2.0, -sd*0.1);
                                do_arr(arb);
                                do_arr(arb + V2::new(-arsxh, -arsyh));
                                do_arr(arb + V2::new(0.0, -2.0*arsyh));
                            }
                        }),
                        format!("move any one agent standing near to a pair of {aen} and {ben} by three lands."),
                    )
                }
            }),
        ),
    });
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![FIELD]).into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let opposite = opposite_element(e);
        let opposite_name = ELEMENT_NAMES[opposite];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        
        let sd = bounds.span().x;
        let bc = bounds.center() + V2::new(0.0, sd*0.21);
        let fr = sd*0.29;
        let arrow_scale = sd*0.1/(all_assets.step.bounds.x/2.0);
        let arsep = sd*0.12;
        
        CardSpec::means_card(
            &all_assets,
            format!("{opposite_name} wind"),
            Some(format!("wind {opposite_name}")),
            2, false, 1,
            vec![(Change, vec![e]), (Move, vec![e])],
            Rc::new({let assets = all_assets.clone(); move |w| {
                assets.flip_to(e).centered_rad(bc, fr, w);
                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                let aby = bc.y - fr - sd*0.16;
                do_arr(V2::new(bc.x, aby));
                // do_arr(V2::new(bc.x + ard, aby - arsep));
                do_arr(V2::new(bc.x, aby - arsep*2.0));
                
                let ger = bounds.span().min()*0.174;
                let sep = ger*0.37;
                let gc = bc + from_angle_mag(TAU*3.0/8.0, ger + sep + fr);
                // assets.blank.centered_rad(gc, ger, w);
                assets.guy2.by_anchor_rad(gc, ger*0.9, w);
            }}),
            format!("flip a nearby {opposite_name} to {element_name}, moving each occupant of that land up to two lands"),
        )
    }}))});
    
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![TOMB]).into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        
        let sd = bounds.span().x;
        let bc = bounds.center() + V2::new(0.0, sd*0.21);
        let fr = sd*0.29;
        let arrow_rad = sd*0.1;
        let arrow_scale = sd*0.1/(all_assets.step.bounds.x/2.0);
        let arsep = sd*0.12;
        let ard = arrow_rad*1.2;
        
        CardSpec::means_card(
            &all_assets,
            format!("{element_name} wind"),
            Some(format!("wind {element_name}")),
            1, false, 1,
            vec![(Move, vec![e])],
            Rc::new({let assets = all_assets.clone(); move |w| {
                assets.element(e).centered_rad(bc, fr, w);
                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                let aby = bc.y - fr - sd*0.16;
                do_arr(V2::new(bc.x - ard, aby));
                do_arr(V2::new(bc.x + ard, aby - arsep));
                do_arr(V2::new(bc.x - ard, aby - arsep*2.0));
                
                let ger = bounds.span().min()*0.174;
                let sep = ger*0.37;
                let gc = bc + from_angle_mag(TAU*3.0/8.0, ger + sep + fr);
                // assets.blank.centered_rad(gc, ger, w);
                assets.guy2.by_anchor_rad(gc, ger*0.9, w);
            }}),
            format!("move each occupant of a nearby {element_name} up to three lands"),
        )
    }}))});
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![MOUNTAIN]).into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let element_article = ELEMENT_ARTICLE[e];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        
        let sd = bounds.span().x;
        let bc = bounds.center() + V2::new(0.0, sd*0.21);
        let fr = sd*0.29;
        let arrow_scale = sd*0.1/(all_assets.step.bounds.x/2.0);
        let arsep = sd*0.12;
        // let ard = sd*0.12;
        
        CardSpec::means_card(
            &all_assets,
            format!("{element_name} wind"),
            Some(format!("wind {element_name}")),
            1, false, 1,
            vec![(Move, vec![e])],
            Rc::new({let assets = all_assets.clone(); move |w| {
                assets.element(e).centered_rad(bc, fr, w);
                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                let aby = bc.y - fr - sd*0.16;
                do_arr(V2::new(bc.x, aby));
                // do_arr(V2::new(bc.x + ard, aby - arsep));
                do_arr(V2::new(bc.x, aby - arsep*2.0));
            }}),
            format!("move each occupant of {element_article} {element_name} up to two lands"),
        )
    }}))});
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![ICE]).into_map({let all_assets=all_assets.clone(); move |e|{
        let element_name = ELEMENT_NAMES[e];
        let bounds = means_graphic_usual_bounds_shrunk_appropriately();
        
        let sd = bounds.span().x;
        let bc = bounds.center() + V2::new(0.0, sd*0.21);
        let fr = sd*0.29;
        let arrow_scale = sd*0.1/(all_assets.step.bounds.x/2.0);
        let arsep = sd*0.12;
        let ard = sd*0.12;
        
        CardSpec::means_card(
            &all_assets,
            format!("freezing wind"),
            Some(format!("wind {element_name}")),
            1, false, 1,
            vec![(Change, vec![e]), (Move, vec![e])],
            Rc::new({let assets = all_assets.clone(); move |w| {
                assets.flip_to(e).centered_rad(bc, fr, w);
                let mut do_arr = |c| assets.step.centered_rotated(c, arrow_scale, -TAU/4.0, w);
                let aby = bc.y - fr - sd*0.16;
                do_arr(V2::new(bc.x - ard, aby));
                do_arr(V2::new(bc.x + ard, aby - arsep));
                do_arr(V2::new(bc.x - ard, aby - arsep*2.0));
            }}),
            format!("freeze any lake, moving its occupants up to three lands over"),
        )
    }}))});

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(IndexVec(vec![MOUNTAIN, LAKE]).into_map({
            let assets = all_assets.clone();
            move |e| {
                let en = ELEMENT_NAMES[e];
                let ea = ELEMENT_ARTICLE[e];
                CardSpec::means_card(
                    &assets,
                    format!("{en} travel"),
                    Some(format!("travel {en}")),
                    0,
                    false,
                    1,
                    vec![(Move, vec![e])],
                    Rc::new({
                        let assets = assets.clone();
                        move |w| {
                            come_on_down_specifically(
                                assets.element(e),
                                assets.element(e),
                                element_color_back(e),
                                means_graphic_usual_bounds(),
                                None, None,
                                w,
                            );
                        }
                    }),
                    format!("send an agent standing on {ea} {en} to any other {en}"),
                )
            }
        })),
    });

    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![(VOLCANO, TOMB), (ICE, LAKE), (VOID, MOUNTAIN)]).into_map({let all_assets = all_assets.clone(); move|(e,et)|{
        let en = ELEMENT_NAMES[e];
        let eo = opposite_element(e);
        let eon = ELEMENT_NAMES[eo];
        let etn = ELEMENT_NAMES[et];
        CardSpec::means_card(
            &all_assets,
            format!("{eon} liftoff"),
            Some(format!("liftoff {en} {etn}")),
            0, false, 1,
            vec![(Change, vec![e]), (Move, vec![e, et])],
            Rc::new({
                let all_assets = all_assets.clone();
                move |w| {
                    come_on_down_specifically(
                        all_assets.flip_to(e),
                        all_assets.element(et),
                        element_color_back(e),
                        means_graphic_usual_bounds(),
                        None, None,
                        w,
                    );
                }
            }),
            format!("pick any {eon}, and send anyone standing on it to any {etn}, flipping the originating {eon}"),
        )
    }}))});
    
    r.push(CardGen { min_count: 8, desired_proportion: 0.0, generator: Box::new(IndexVec(vec![TOMB]).into_map({let all_assets = all_assets.clone(); move|e|{
        let en = ELEMENT_NAMES[e];
        // let etn = ELEMENT_NAMES[es];
        CardSpec::means_card(
            &all_assets,
            format!("{en} banishment"),
            Some(format!("sending {en}")),
            0, false, 1,
            vec![(Move, vec![e])],
            Rc::new({
                let all_assets = all_assets.clone();
                move |w| {
                    come_on_down_specifically(
                        &all_assets.darker_blank,
                        all_assets.element(e),
                        DARKER_BLANK_COLOR,
                        means_graphic_usual_bounds(),
                        Some(&all_assets.guy2),
                        None,
                        w,
                    );
                }
            }),
            format!("send anyone on a nearby land to any {en}"),
        )
    }}))});
    
    // in case you ever want a card that's just a move arrow for some reason
    // r.push(CardGen {
    //     min_count: 8,
    //     desired_proportion: 0.0,
    //     generator: Box::new(Once(CardSpec::means_card(
    //         &all_assets,
    //         "step".to_string(),
    //         None,
    //         0,
    //         true,
    //         4,
    //         vec![],
    //         Rc::new({
    //             let all_assets = all_assets.clone();
    //             move |w| {
    //                 all_assets.step.centered(
    //                     means_graphic_usual_bounds_shrunk_appropriately().center(),
    //                     1.0,
    //                     w,
    //                 );
    //             }
    //         }),
    //         "move to an adjacent land".to_string(),
    //     ))),
    // });

    r.push(CardGen {
        min_count: 9,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (FOREST, VOID, VOLCANO),
                (TOMB, FOREST, LAKE),
                (FIELD, FIELD, TOMB),
                (VOLCANO, MOUNTAIN, LAKE),
                (FIELD, ICE, VOID),
                (MOUNTAIN, FIELD, ICE),
            ])
            .into_map({
                let assets = all_assets.clone();
                move |(ae, be, ce)| {
                    let aen = ELEMENT_NAMES[ae];
                    let ben = ELEMENT_NAMES[be];
                    let cen = ELEMENT_NAMES[ce];
                    let aeon = ELEMENT_NAMES[opposite_element(ae)];
                    let beon = ELEMENT_NAMES[opposite_element(be)];
                    let ceon = ELEMENT_NAMES[opposite_element(ce)];
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
                        format!("standing on any chain of {aeon}, {beon}, {ceon}, flip all of them."),
                    )
                }
            }),
        ),
    });

    r.push(CardGen {
        min_count: 8,
        desired_proportion: 0.0,
        generator: Box::new(
            IndexVec(vec![
                (LAKE, FIELD),
                (LAKE, MOUNTAIN),
                (ICE, VOID),
                (FOREST, VOID),
                (TOMB, MOUNTAIN),
            ])
            .into_map({
                let assets = all_assets.clone();
                move |(ae, be)| {
                    let aeon = ELEMENT_NAMES[opposite_element(ae)];
                    let beon = ELEMENT_NAMES[opposite_element(be)];
                    CardSpec::means_card(
                        &assets,
                        "flip both".to_string(),
                        Some(format!("flip_both_{aeon}_{beon}")),
                        1,
                        // the ones that just sort of move an element along through a limited substrate without changing the total number of either are extraordinarily weak
                        ae == opposite_element(be),
                        1,
                        vec![(Change, vec![opposite_element(ae), opposite_element(be)])],
                        Rc::new({
                            let assets = assets.clone();
                            move |w| {
                                let bounds = means_graphic_usual_bounds_shrunk_appropriately();

                                pair_flip_verticalish(
                                    bounds.center(),
                                    bounds.span().x / 2.0,
                                    &*assets,
                                    ae,
                                    be,
                                    w,
                                );
                            }
                        }),
                        format!("When standing on a pair of {aeon} and {beon}, flip them."),
                    )
                }
            }),
        ),
    });

    r.push(CardGen { min_count: 5, desired_proportion: 0.0, generator: Box::new({let all_assets = all_assets.clone(); IndexVec(vec![ICE, FIELD]).into_map(move |e| {
        let ename = ELEMENT_NAMES[e];
        CardSpec::means_card(
            &all_assets,
            "atoms for something else".to_string(), Some(format!("atoms {ename}")),
            2, true, 1,
            vec![(Kill, vec![e])],
            Rc::new({
                let all_assets = all_assets.clone();
                move |w| {
                    let bounds = means_graphic_usual_bounds();
                    let sd = bounds.span().min();
                    let c = bounds.center();
                    let lateral = sd*0.23;
                    let downward = sd*0.3;
                    let guy_scale = 0.85;
                    all_assets.element(e).centered_rad(c + V2::new(0.0, downward), downward*0.9, w);
                    all_assets.guy2_mage.by_anchor(c + V2::new(-lateral, 0.0), guy_scale, w);
                    horizontal_flip(&all_assets.cubed_guy2).by_anchor(c + V2::new(lateral, 0.0), guy_scale, w);
                }
            }),
            format!("Near to {ename} that is also near to an opponent's agent, capture that agent and replace it with a spare of your own. All of your agents can now use that player's abilities."),
        )
    })})});
    
    //ensure no dupes
    let mut s = std::collections::HashSet::new();
    for cg in r.iter() {
        for c in cg.generator.iter() {
            let cn = &c.name;
            if !s.insert(cn.clone()) {
                println!("warning, two files are called \"{cn}\"")
            }
        }
    }

    r
}

pub fn land_circle_bounds() -> Rect {
    let a = V2::new(9.922, 9.922);
    Rect {
        ul: a,
        br: a + V2::new(99.219, 99.219),
    }
}
pub fn land_hex_smaller_bounds() -> Rect {
    let a = V2::new(10.075, 16.016);
    Rect {
        ul: a,
        br: a + V2::new(98.913, 87.029),
    }
}

const MINI_HEX_DIMS: V2 = V2::new(119.063, 119.063);
const MINI_CIRCLE_DIMS: V2 = V2::new(119.063, 119.063);
pub fn land_specs_smaller(assets: &Rc<Assets>, repeating: &[u8]) -> Vec<CardGen> {
    land_specs_dims(assets, repeating, MINI_HEX_DIMS, land_hex_smaller_bounds(), false)
}
pub fn land_specs_card(assets: &Rc<Assets>, repeating: &[u8]) -> Vec<CardGen> {
    land_specs_dims(assets, repeating, CARD_DIMENSIONS, cutline_bounds(), true)
}
pub fn land_specs_mini_circles(assets: &Rc<Assets>, repeating: &[u8]) -> Vec<CardGen> {
    land_specs_dims(assets, repeating, MINI_CIRCLE_DIMS, land_circle_bounds(), false)
}
pub fn land_specs_dims(assets: &Rc<Assets>, repeating: &[u8], dims: V2, bounds:Rect, rotated:bool) -> Vec<CardGen> {
    assert_eq!(repeating.len(), 4);
    let mut r: Vec<CardGen> = Vec::new();
    fn side(assets: Rc<Assets>, e: ElementTag, dims:V2, bounds:Rect, rotated:bool) -> Rc<impl Fn(&mut dyn Write)> {
        let rad = bounds.span().min() / 2.0 * 0.98;
        Rc::new(move |w: &mut dyn Write| {
            svg_outer(
                dims,
                element_color_back(e),
                &Displaying(
                    {let assets = assets.clone(); let bounds=bounds.clone(); move |w| {
                        assets
                            .element(e)
                            .centered_rotr(bounds.center(), rad, if rotated {TAU/4.0} else {0.0}, w)
                    }}
                ),
                w,
            );
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
                repeat: repeatings[e / 2] as usize,
                level: 0,
                frequency_modifier: 1.0,
                properties: vec![],
                generate_front: side(assets.clone(), e, dims, bounds.clone(), rotated),
                generate_back: side(assets.clone(), eo, dims, bounds.clone(), rotated),
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
