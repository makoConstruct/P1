use std::{f64::consts::TAU, rc::Rc};

use super::*;

pub fn end_specs(all_assets: &AllAssets) -> Vec<CardSpec> {
    let mut specs = Vec::new();
    let assets = all_assets.svg.clone();

    for e in elements() {
        let scores = "1".to_string();
        specs.push(CardSpec::end_card_with_back_blurred_message(
            format!("{}_1", element_names[e]),
            Rc::new(Displaying(move |w| {
                (element_g[e])(END_GRAPHIC_CENTER, 1.0, w)
            })),
            scores.clone(),
            format!("{} point for every {}", &scores, element_names[e]),
        ))
    }

    for e1 in elements() {
        for e2 in elements() {
            if e1 >= e2 {
                continue;
            }
            specs.push(CardSpec::end_card_with_back_blurred_message(
                format!("{}_{}", element_names[e1], element_names[e2]),
                Rc::new(Displaying(move |w| paired(e1, e2, false, w))),
                "2".to_string(),
                format!(
                    "2 points for every adjacent pairing of {} and {}",
                    element_names[e1], element_names[e2]
                ),
            ));
        }
    }

    for e in elements() {
        let scores = "6".to_string();
        let ename = element_names[e];
        specs.push(CardSpec::end_card_with_back_blurred_message(
            format!("just_1_{}", ename),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}\n{}",
                    &Displaying(|w: &mut dyn Write| element_g[e](
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
        ));
    }

    for e1 in elements() {
        for e2 in elements() {
            for e3 in elements() {
                if e1 > e2 || e2 > e3 {
                    continue;
                }
                let tilt = -TAU / 24.0;
                let arc = TAU / 3.0;
                let r = GRAPHIC_RAD * 0.48;
                let scale = 0.5;

                let scores = "4".to_string();

                specs.push(CardSpec::end_card_with_back_blurred_message(
                    format!(
                        "{}_{}_{}",
                        element_names[e1], element_names[e2], element_names[e3]
                    ),
                    Rc::new(Displaying(move |w| {
                        write!(
                            w,
                            "{}{}{}",
                            &Displaying(|w: &mut dyn Write| element_g[e1](
                                END_GRAPHIC_CENTER + from_angle_mag(tilt, r),
                                scale,
                                w
                            )),
                            &Displaying(|w: &mut dyn Write| element_g[e2](
                                END_GRAPHIC_CENTER + from_angle_mag(tilt + arc, r),
                                scale,
                                w
                            )),
                            &Displaying(|w: &mut dyn Write| element_g[e3](
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
                        &scores, element_names[e1], element_names[e2], element_names[e3]
                    ),
                ));
            }
        }
    }

    for e in elements() {
        let element_name = element_names[e];
        specs.push(CardSpec::end_card_with_back_blurred_message(
            format!("max_{}_cluster", element_name),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w:&mut dyn Write| big_splat(element_colors_back[e], w)),
                    &Displaying(|w:&mut dyn Write| element_g[e](END_GRAPHIC_CENTER, 0.7, w)),
                ).unwrap();
            })),
            "1".to_string(),
            format!("1 point for every {element_name} in the largest connected cluster of {element_name}s")
        ));
    }

    for e in elements() {
        let ename = element_names[e];
        let scores = "7".to_string();
        specs.push(CardSpec::end_card_with_back_blurred_message(
            format!("forbid_{ename}"),
            Rc::new(Displaying(move |w: &mut dyn Write| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w: &mut dyn Write| element_g[e](END_GRAPHIC_CENTER, 1.0, w)),
                    &Displaying(negatory),
                )
                .unwrap();
            })),
            scores.clone(),
            format!("{} points, only if there are no {ename}s at all", &scores),
        ))
    }

    for e1 in elements() {
        for e2 in elements() {
            if e1 < e2 {
                continue;
            }
            let scores = "9".to_string();
            let en1 = element_names[e1];
            let en2 = element_names[e2];
            specs.push(CardSpec::end_card_with_back_blurred_message(
                format!("forbid_{en1}_{en2}"),
                Rc::new(Displaying(move |w: &mut dyn Write| {
                    write!(
                        w,
                        "{}{}",
                        &Displaying(|w: &mut dyn Write| paired(e1, e2, true, w)),
                        &Displaying(negatory)
                    )
                    .unwrap();
                })),
                scores.clone(),
                format!(
                    "{} as long as there are no {en1}s adjacent to {en2} at the end",
                    &scores
                ),
            ));
        }
    }

    fn from_asset(asset: &Asset, name: String, scores: String, description: String) -> CardSpec {
        CardSpec::end_card_with_back_blurred_message(
            name.clone(),
            Rc::new({
                let asset = asset.clone();
                Displaying(move |w| asset.center_in_bounds(end_graphic_usual_bounds().shrunk(0.8), w))
            }),
            scores,
            description,
        )
    }
    specs.push(from_asset(
        &assets.kill,
        String::from("kill"),
        "3".to_string(),
        String::from("You gain 3 points for each piece who is killed"),
    ));
    specs.push(from_asset(&assets.altruism, String::from("altruism"), "".to_string(), String::from("Your values encompass the values of others.\n\nYour score will be the sum of the scores of others")));
    
    {
        for e1 in elements() {
            for e2 in elements() {
                if e1 >= e2 {
                    continue;
                }
                let e1n = element_names[e1];
                let e2n = element_names[e2];
                specs.push(CardSpec::end_card_with_back_blurred_message(
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
                ))
            }
        }
    }

    specs
}

pub fn means_specs(all_assets: &Rc<AllAssets>) -> Vec<CardSpec> {
    let mut r = Vec::new();
    let assets = all_assets.svg.clone();

    {
        for e in elements() {
            let angle = TAU / 12.0;
            let separation = 1.1;
            let radius = 2.1;
            let off = from_angle_mag(angle, radius + separation);
            let bounds = means_graphic_usual_bounds();
            let ab = bounds.center() + off;
            let bb = bounds.center() - off;
            let element_name = element_names[e];
            r.push(CardSpec::means_card_with_back_blurred_message(
                format!("ambush from {element_name}"),
                None,
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
                format!("standing in a {element_name}, kill one piece in an adjacent land"),
            ))
        }
    }

    for e in elements() {
        let element_name = element_names[e];
        let opposite = opposite_element(e);
        let opposite_name = element_names[opposite];
        let center = card_upper_center();
        let rings_span: f64 = 115.65681;
        let offset = V2::new(center.x - rings_span / 2.0, center.y - rings_span / 2.0);
        let to_color = element_colors_back[e];
        let from_color = element_colors_back[opposite];
        let ea = assets.element(e);
        let element_graphic = {
            let assets = assets.clone();
            Displaying(move |w| {
                assets.element(e).by_grav(
                    V2::new(rings_span / 2.0, rings_span / 2.0),
                    MIDDLE_MIDDLE,
                    1.0,
                    w,
                )
            })
        };
        r.push(CardSpec::means_card_with_back_blurred_message(
            format!("flip {element_name}"),
            None,
            Rc::new(move |w| {
                flip_rings(to_color, from_color, &element_graphic, offset, 1.0, w);
            }),
            format!("transition the {opposite_name} you stand in to into a {element_name}"),
        ))
    }

    for eh in elements() {
        for ek in elements() {
            if ek == eh {
                continue;
            }
            let ekn = element_names[ek];
            let ehn = element_names[eh];
            r.push(CardSpec::means_card_with_back_blurred_message(
                "domain strike".to_string(),
                Some(format!("domain_strike_from_{ehn}_to_{ekn}")),
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
                        assets
                            .element(eh)
                            .by_grav(hc, MIDDLE_MIDDLE, hr / BIG_ELEMENT_RAD, w);
                        assets
                            .element(ek)
                            .by_grav(kc, MIDDLE_MIDDLE, kr / BIG_ELEMENT_RAD, w);
                        guylike(&assets.dead_guy, kc, 1.0, w);
                    }
                }),
                format!("choose a {ekn} that's adjacent to a {ehn}. kill all those standing in the {ekn}"),
            ))
        }
    }

    for eho in elements() {
        for ek in elements() {
            if ek == eho {
                continue;
            }
            let eh = opposite_element(eho);
            let ekn = element_names[ek];
            let ehn = element_names[eh];
            let ehon = element_names[eho];
            r.push(CardSpec::means_card_with_back_blurred_message(
                "domain blast".to_string(),
                Some(format!("domain_blast_from_{}_to_{}", ehn, ekn)),
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
                        flipping_to(&assets, eh, hc, hr/FLIP_RINGS_RAD, w);
                        // assets
                        //     .element(eh)
                        //     .by_grav(hc, MIDDLE_MIDDLE, hr / BIG_ELEMENT_RAD, w);
                        assets
                            .element(ek)
                            .by_grav(kc, MIDDLE_MIDDLE, kr / BIG_ELEMENT_RAD, w);
                        guylike(&assets.dead_guy, kc, 1.0, w);
                    }
                }),
                format!("choose a {ekn} that's adjacent to a {ehon}. kill all those standing in the {ekn}, and flip the {ehon} to a {ehn}."),
            ))
        }
    }

    for e in elements() {
        let en = element_names[e];
        r.push(CardSpec::means_card_with_back_blurred_message(
            format!("{en} travel"),
            None,
            Rc::new({
                let assets = assets.clone();
                move |w| {
                    come_on_down_specifically(
                        &assets,
                        assets.element(e),
                        assets.element(e),
                        e,
                        means_graphic_usual_bounds(),
                        w,
                    );
                }
            }),
            format!("send any one person standing on a {en} to any other {en}"),
        ))
    }

    for e in elements() {
        for et in elements() {
            if e == et {
                continue;
            }
            let en = element_names[e];
            let eo = opposite_element(e);
            let etn = element_names[et];
            r.push(CardSpec::means_card_with_back_blurred_message(
                format!("{en} liftoff"),
                Some(format!("{en} {etn} liftoff")),
                Rc::new({
                    let all_assets = all_assets.clone();
                    move |w| {
                        come_on_down_specifically(
                            &all_assets.svg,
                            all_assets.flip_to(e),
                            all_assets.svg.element(et),
                            e,
                            means_graphic_usual_bounds(),
                            w,
                        );
                    }
                }),
                format!("pick a {eo}, and send anyone standing on it to any {etn}, flipping the {eo} to a {e}"),
            ))
        }
    }

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

    r
}

pub fn land_specs(assets: &Rc<AllAssets>) -> Vec<CardSpec> {
    let mut r = Vec::new();
    fn side(assets: Rc<AllAssets>, e: ElementTag) -> Rc<impl Fn(&mut dyn Write)> {
        let scale = CARD_DIMENSIONS.min()/2.0/BIG_ELEMENT_RAD*0.86;
        Rc::new(
            move |w:&mut dyn Write| {
                blank_front(
                    &Displaying({
                        let assets = assets.clone();
                        move |w| {
                            assets
                                .svg
                                .element(e)
                                .centered(CARD_DIMENSIONS / 2.0, scale, w)
                        }
                    }),
                    element_colors_back[e],
                    true,
                    w,
                )
            }
        )
    };
    for (e, eo) in element_primaries() {
        r.push(CardSpec {
            name: format!("land_{}_{}", element_names[e], element_names[eo]),
            generate_front: side(assets.clone(), e),
            generate_back: side(assets.clone(), eo),
        });
    }
    r
}
// pub fn gen_board(radius:usize, to:&dyn Write)
