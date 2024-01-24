use std::{f64::consts::TAU, rc::Rc};

use super::*;

pub fn generate_specs(specs: &mut Vec<EndCardSpec>) {
    for e in elements() {
        let scores = 1;
        specs.push(EndCardSpec::with_back_blured_message(
            format!("{}_1", element_names[e]),
            Rc::new(Displaying(move |w| {
                (element_g[e])(END_GRAPHIC_CENTER, 1.0, w)
            })),
            scores,
            format!("{scores} point for every {}", element_names[e]),
        ))
    }

    for e1 in elements() {
        for e2 in elements() {
            if e1 >= e2 {
                continue;
            }
            specs.push(EndCardSpec::with_back_blured_message(
                format!("{}_{}", element_names[e1], element_names[e2]),
                Rc::new(Displaying(move |w| paired(e1, e2, false, w))),
                2,
                format!(
                    "2 points for every adjacent pairing of {} and {}",
                    element_names[e1], element_names[e2]
                ),
            ));
        }
    }

    for e in elements() {
        let scores = 6;
        let ename = element_names[e];
        specs.push(EndCardSpec::with_back_blured_message(
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
            scores,
            format!("{scores} points as long as there is only one {ename} at the end"),
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

                let scores = 4;

                specs.push(EndCardSpec::with_back_blured_message(
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
                        ).unwrap();
                    })),
                    scores,
                    format!(
                        "{} points for every trio of adjacent {}, {} and {}",
                        scores, element_names[e1], element_names[e2], element_names[e3]
                    ),
                ));
            }
        }
    }

    for e in elements() {
        let element_name = element_names[e];
        specs.push(EndCardSpec::with_back_blured_message(
            format!("max_{}_cluster", element_name),
            Rc::new(Displaying(move |w| {
                write!(
                    w,
                    "{}{}",
                    &Displaying(|w:&mut dyn Write| big_splat(element_colors_back[e], w)),
                    &Displaying(|w:&mut dyn Write| element_g[e](END_GRAPHIC_CENTER, 0.7, w)),
                ).unwrap();
            })),
            1,
            format!("1 point for every {element_name} in the largest connected cluster of {element_name}s")
        ));
    }

    for e in elements() {
        let ename = element_names[e];
        let scores = 7;
        specs.push(EndCardSpec::with_back_blured_message(
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
            scores,
            format!("{scores} points, only if there are no {ename}s at all"),
        ))
    }

    for e1 in elements() {
        for e2 in elements() {
            if e1 < e2 {
                continue;
            }
            let scores = 9;
            let en1 = element_names[e1];
            let en2 = element_names[e2];
            specs.push(EndCardSpec::with_back_blured_message(
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
                scores,
                format!("{scores} as long as there are no {en1}s adjacent to {en2} at the end"),
            ));
        }
    }
}
