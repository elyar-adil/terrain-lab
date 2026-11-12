//! The body: one skin, and everything that is cut into it.
//!
//! The shell is lofted whole, sill to roof, and then drawn on: the wheel
//! arches, the glass with its black border, the bright strip round the side
//! windows, the lamps and openings (`lamps::cut`), and last the shut lines of
//! the doors, the bonnet and the boot, each a groove pressed into the paint.
//! Every one of those is a region of the same surface, so each meets its
//! neighbour along one shared edge.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;
use crate::model::{skin_cut, skin_lifted};

use super::form::{ARCH_GAP, Form, ROOF, ROOF_RAIL, SIDE_GLASS, Shell, lines_of, stations};
use super::label;
use super::parts::{round_and, sd_box, smoothstep};
use super::surface::Soup;

/// Half the width of a shut line, and its depth.
const SEAM_HALF_M: f32 = 0.0030;
const SEAM_DEEP_M: f32 = 0.0045;

/// Is this quad of the far shell glazing? Side glass between the pillars, a
/// windscreen and a rear glass between the roof rails.
fn glazed(form: &Form, shell: &Shell, i: usize, k: usize) -> bool {
    let spec = form.spec;
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let lines = lines_of(spec);
    let x = shell.mid_x(i);
    let (stretch, place) = shell.quad(k);
    let side = stretch == SIDE_GLASS
        && place > 0
        && form.cabin(x) > 0.42
        && (x - lines.b_pillar).abs() > 0.05
        && x < front_head + (front_foot - front_head) * 0.62
        && x > rear_head - (rear_head - rear_foot) * 0.62;
    let over = stretch == ROOF || (stretch == ROOF_RAIL && place > 0);
    let screen = over && x > front_head + 0.035 && x < front_foot - 0.045;
    let rear = over && x < rear_head - 0.035 && x > rear_foot + 0.045;
    side || screen || rear
}

/// The far model: the same outline in a few hundred triangles.
pub(super) fn build_far(builder: &mut MeshBuilder, form: &Form, body: &str, glass: &str) {
    let xs: Vec<f32> = (0..=18)
        .map(|k| form.tail + (form.nose - form.tail) * k as f32 / 18.0)
        .collect();
    let shell = Shell::new(form, xs, 3);
    let closed = shell.closed(form);
    let ends = shell.face_rings();
    let is_glass = |i: usize, k: usize| {
        i >= ends && i - ends < shell.xs.len() - 1 && glazed(form, &shell, i - ends, k)
    };
    skin_cut(builder, body, &closed, |i, k| !is_glass(i, k));
    skin_lifted(builder, glass, &closed, -0.004, is_glass);
}

/// The shell with everything cut into it, drawn; returned for the fittings to
/// find their places on.
pub(super) fn build(builder: &mut MeshBuilder, form: &Form, key: &dyn Fn(&str) -> String) -> Soup {
    let spec = form.spec;
    let lines = lines_of(spec);
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + ARCH_GAP;
    let shell = Shell::new(form, stations(form, 0.052, &[]), 1);
    let mut soup = shell_soup(form, &shell);
    let body = |l: u8| l == label::BODY;
    sculpt(&mut soup, form);

    // --- wheel arches: true circles, through the flank and the floor beside it.
    let flank = |p: Vec3| (spec.width * 0.5 - 0.42) - p.z.abs();
    let from_axle = |p: Vec3, axle: f32| form.to_axle(p.x, p.y, axle);
    let nearest_axle = |p: Vec3| from_axle(p, -half_base).min(from_axle(p, half_base));
    soup.cut(
        &|p| (nearest_axle(p) - arch).max(flank(p)),
        &body,
        label::ARCH,
    );

    // --- the black of the sills and the underside, and cladding if it has it.
    let sill_high = if spec.cladding { 0.115 } else { 0.07 };
    let lower = |x: f32| {
        let between = smoothstep(half_base + arch + 0.10, half_base - arch, x.abs());
        form.floor(x)
            + 0.035
            + (sill_high - 0.035) * between
            + if spec.cladding { 0.035 } else { 0.0 }
    };
    soup.cut(&|p| p.y - lower(p.x), &body, label::BLACK);
    if spec.cladding {
        soup.cut(
            &|p| (nearest_axle(p) - arch - 0.05).max(flank(p)),
            &body,
            label::BLACK,
        );
    }

    // --- glass.
    let side_front = front_head + (front_foot - front_head) * 0.60;
    let side_rear = rear_head - (rear_head - rear_foot) * 0.60;
    let side_glass = |p: Vec3| {
        let (foot, (_, head)) = form.glass(p.x);
        round_and(
            &[
                foot + 0.008 - p.y,
                p.y - (head - 0.026),
                p.x - side_front,
                side_rear - p.x,
            ],
            0.035,
        )
    };
    soup.cut(&side_glass, &body, label::GLASS);
    soup.cut(&|p| side_glass(p) - 0.013, &body, label::SURROUND);
    // A roof in black over the body's colour: everything above the glass's head.
    if spec.black_roof {
        soup.cut(
            &|p| {
                let (_, (_, head)) = form.glass(p.x);
                (head - 0.012 - p.y).max(0.3 - form.cabin(p.x))
            },
            &body,
            label::BLACK,
        );
    }
    // Pillars: black over the glass, as they are now.
    let glass = |l: u8| l == label::GLASS;
    soup.cut(
        &|p| (p.x - lines.b_pillar).abs() - 0.045,
        &glass,
        label::BLACK,
    );
    if !spec.boot && lines.rear_door + 0.03 > side_rear + 0.12 {
        soup.cut(
            &|p| (p.x - lines.rear_door - 0.03).abs() - 0.04,
            &glass,
            label::BLACK,
        );
    }
    // The fixed quarter lights: a slim bar divides each from the glass that
    // winds down, ahead of the front door's and behind the rear door's.
    if !spec.mirror.on_sail {
        soup.cut(
            &|p| (p.x - (side_front - 0.19)).abs() - 0.011,
            &glass,
            label::BLACK,
        );
    }
    if spec.boot && lines.rear_door + 0.02 > side_rear + 0.10 {
        soup.cut(
            &|p| (p.x - lines.rear_door - 0.02).abs() - 0.013,
            &glass,
            label::BLACK,
        );
    }
    if spec.mirror.on_sail {
        soup.cut(&|p| side_front - 0.13 - p.x, &glass, label::BLACK);
    }
    // A screen lies between two heights of the cabin's end, so it is the same
    // drawing whether the glass is raked (a saloon's) or upright (a van's).
    let screen = |front: bool| {
        let (foot_x, head_x) = if front {
            (front_foot, front_head)
        } else {
            (rear_foot, rear_head)
        };
        let low = form.top(foot_x) + 0.040;
        let high = form.top(head_x) - 0.032;
        let corner = ((high - low) * 0.25).min(0.05);
        move |p: Vec3| {
            let (_, (head, _)) = form.glass(p.x);
            let zone = if front {
                head_x - 0.03 - p.x
            } else {
                p.x - (head_x + 0.03)
            };
            round_and(&[p.z.abs() - (head - 0.030), low - p.y, p.y - high], corner).max(zone)
        }
    };
    let roof_glass = |p: Vec3| {
        let (_, (head, _)) = form.glass(p.x);
        round_and(
            &[
                p.z.abs() - (head - 0.16),
                rear_head + 0.16 - p.x,
                p.x - (front_head - 0.12),
            ],
            0.05,
        )
        .max(form.belt(p.x) + 0.02 - p.y)
    };
    let screens: Vec<Box<dyn Fn(Vec3) -> f32>> = {
        let mut list: Vec<Box<dyn Fn(Vec3) -> f32>> =
            vec![Box::new(screen(true)), Box::new(screen(false))];
        if spec.glass_roof {
            list.push(Box::new(roof_glass));
        }
        list
    };
    for field in &screens {
        soup.cut(field, &body, label::SEAM_NEW);
        // The black border printed on the glass, and the seal round it.
        soup.cut(
            &|p| -field(p) - 0.032,
            &|l| l == label::SEAM_NEW,
            label::FRIT,
        );
        soup.rename(label::SEAM_NEW, label::SCREEN);
        soup.cut(&|p| field(p) - 0.006, &body, label::BLACK);
    }
    // The cowl the wipers park on.
    soup.cut(
        &|p| {
            let (_, (head, _)) = form.glass(p.x);
            round_and(
                &[
                    p.z.abs() - (head + 0.03),
                    front_foot - 0.06 - p.x,
                    p.x - (front_foot + 0.05),
                ],
                0.02,
            )
            .max(form.belt(p.x) - 0.01 - p.y)
        },
        &body,
        label::BLACK,
    );

    // The valance under the rear bumper, where the pipes come out.
    let valance = (0.0, form.floor(form.tail) + 0.10);
    let valance_half = (form.face_half(false) * 0.80, 0.105);
    soup.cut(
        &|p| sd_box(valance, valance_half, (p.z, p.y), 0.03).max(p.x - form.tail - 0.2),
        &body,
        label::BLACK,
    );

    // Air curtains: an upright black slot in each corner of the front bumper.
    if !spec.fog {
        let half = form.face_half(true);
        let at = (half * 0.90, form.floor(form.nose) + 0.19);
        soup.cut(
            &|p| sd_box(at, (0.028, 0.085), form.unwrap(true, p), 0.022).max(-p.x),
            &body,
            label::BLACK,
        );
    }

    super::lamps::cut(&mut soup, form);

    // --- shut lines. Each is the edge of a panel, pressed in as a groove.
    let seam = |soup: &mut Soup, panel: &dyn Fn(Vec3) -> f32, limit: &dyn Fn(Vec3) -> f32| {
        soup.cut(
            &|p| (panel(p) - SEAM_HALF_M).max(limit(p)),
            &body,
            label::SEAM_NEW,
        );
        soup.cut(
            &|p| panel(p) + SEAM_HALF_M,
            &|l| l == label::SEAM_NEW,
            label::BODY,
        );
        soup.score(panel, &|l| l == label::SEAM_NEW);
        soup.emboss(&|l| l == label::SEAM_NEW, &|p| {
            -SEAM_DEEP_M * (1.0 - panel(p).abs() / SEAM_HALF_M).max(0.0)
        });
        soup.rename(label::SEAM_NEW, label::SEAM);
    };
    let sill_line = |x: f32| lower(x) + 0.014;
    let below_glass = |p: Vec3| p.y - form.glass(p.x).0;
    let front_door = |p: Vec3| {
        round_and(
            &[
                p.x - lines.front_door,
                lines.b_pillar - p.x,
                sill_line(p.x) - p.y,
            ],
            0.03,
        )
    };
    let rear_door = |p: Vec3| {
        round_and(
            &[
                p.x - lines.b_pillar,
                lines.rear_door - p.x,
                sill_line(p.x) - p.y,
                arch + 0.045 - from_axle(p, -half_base),
            ],
            0.03,
        )
    };
    seam(&mut soup, &front_door, &below_glass);
    if spec.four_doors {
        seam(&mut soup, &rear_door, &below_glass);
    }
    // The bonnet: its sides along the wings and its leading edge.
    let above_floor = |p: Vec3| form.floor(p.x) + 0.22 - p.y;
    let bonnet = |p: Vec3| {
        round_and(
            &[
                front_foot - p.x,
                p.z.abs() - (form.half_width(p.x) - 0.085),
                p.x - (form.nose - 0.085),
            ],
            0.07,
        )
    };
    seam(&mut soup, &bonnet, &above_floor);
    // The boot lid, or the tailgate: down the back to the bumper.
    let sill_rear = form.floor(form.tail) + 0.31;
    let lid_from = if spec.boot {
        lines.boot
    } else {
        rear_head + 0.03
    };
    let lid_inset = if spec.boot { 0.10 } else { 0.07 };
    let lid = |p: Vec3| {
        round_and(
            &[
                p.x - lid_from,
                p.z.abs() - (form.half_width(p.x.max(form.tail + 0.25)) - lid_inset),
                sill_rear - p.y,
            ],
            0.05,
        )
    };
    seam(&mut soup, &lid, &|p| p.x + 0.2);
    // Where the bumpers meet the wings: from each lamp's outer end to the arch.
    for front in [true, false] {
        let toward = if front { 1.0_f32 } else { -1.0 };
        let (wing, end) = if front {
            (&spec.head, form.nose)
        } else {
            (&spec.tail, form.tail)
        };
        let _ = end;
        let from = (
            form.wing_x(front, wing.s.1 - 0.012),
            form.face_top(front) + wing.y.1 - wing.h.1 * 0.3,
        );
        let angle = 0.50_f32;
        let to = (
            toward * (half_base + arch * angle.cos()),
            spec.wheel_r + arch * angle.sin(),
        );
        let (dx, dy) = (to.0 - from.0, to.1 - from.1);
        let length = (dx * dx + dy * dy).sqrt().max(1.0e-3);
        let across = move |p: Vec3| ((p.x - from.0) * dy - (p.y - from.1) * dx) / length;
        let along = move |p: Vec3| {
            let t = ((p.x - from.0) * dx + (p.y - from.1) * dy) / length;
            (-t).max(t - length).max(flank(p))
        };
        seam(&mut soup, &across, &along);
    }
    // The filler flap (or the charging port), on the right, over the rear wheel.
    let flap = (
        -half_base - arch * 0.45,
        (spec.wheel_r + arch + 0.15).min(form.belt(-half_base) - 0.13),
    );
    let flap_half = if spec.electric {
        (0.062, 0.062)
    } else {
        (0.072, 0.066)
    };
    let flap_r = if spec.electric { 0.06 } else { 0.022 };
    seam(
        &mut soup,
        &|p| sd_box(flap, flap_half, (p.x, p.y), flap_r),
        &|p| -p.z,
    );
    // Door handles: a dished pull to get fingers behind, or a flush bar's outline.
    for (x, _) in handle_places(form) {
        let at = (x, form.belt(x) - 0.105);
        if spec.electric {
            seam(
                &mut soup,
                &|p| sd_box(at, (0.10, 0.016), (p.x, p.y), 0.016),
                &flank,
            );
        } else {
            let dish = |p: Vec3| sd_box(at, (0.095, 0.030), (p.x, p.y), 0.028).max(flank(p));
            soup.cut(&dish, &body, label::POCKET);
            let pocket = |l: u8| l == label::POCKET;
            soup.score(&|p| dish(p) + 0.009, &pocket);
            soup.score(&|p| dish(p) + 0.019, &pocket);
            soup.emboss(&pocket, &|p| -0.016 * smoothstep(0.0, 0.022, -dish(p)));
        }
    }

    // --- draw it.
    let paint = key("body");
    let dark = key("trim");
    soup.emit(builder, &paint, 0.0, &|t| {
        matches!(t.label, label::BODY | label::POCKET)
    });
    soup.emit(builder, &key("glass"), 0.0, &|t| {
        matches!(t.label, label::GLASS | label::SCREEN | label::FRIT)
    });
    soup.emit(builder, &dark, -0.002, &|t| t.label == label::FRIT);
    soup.emit(builder, &dark, 0.0, &|t| {
        matches!(t.label, label::BLACK | label::SEAM)
    });
    let surround = if spec.bright_trim {
        key("chrome")
    } else {
        dark.clone()
    };
    soup.emit(builder, &surround, 0.0, &|t| t.label == label::SURROUND);
    // The lining: headlining, pillars and door cards, seen through the glass.
    // A coarse skin just inside the shell, open where the far model is glazed.
    let from = rear_foot - 0.02;
    let count = ((front_foot - rear_foot) / 0.11).ceil() as usize;
    let xs: Vec<f32> = (0..=count)
        .map(|k| from + (front_foot + 0.04 - from) * k as f32 / count as f32)
        .collect();
    let inner = Shell::new(form, xs, 2);
    skin_lifted(builder, &key("interior"), &inner.rings, -0.026, |i, k| {
        let (stretch, _) = inner.quad(k);
        // Not across the wheel arches, where it would show from the road.
        let at = inner.rings[i][k];
        // Nor behind either screen, however upright it stands.
        let x = inner.mid_x(i);
        let screen = stretch >= ROOF_RAIL && (x < rear_head + 0.03 || x > front_head - 0.03);
        stretch >= 2
            && !screen
            && !glazed(form, &inner, i, k)
            && (stretch >= SIDE_GLASS - 1 || nearest_axle(at) > arch + 0.16)
    });
    soup
}

/// The lines pressed into the panels before anything is cut from them: a
/// crease along the shoulder that catches the light from lamp to lamp, a
/// hollow along the doors above the sill, a lip round each wheel arch and two
/// creases down the bonnet. Each is a few millimetres of relief with a sharp
/// line at its middle, which is all a pressed panel has.
fn sculpt(soup: &mut Soup, form: &Form) {
    let spec = form.spec;
    let all = |_: u8| true;
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + ARCH_GAP;
    let length = form.nose - form.tail;
    let front_foot = spec.cabin[3];
    // A ridge of height `a` and half-width `w` about the zero of `f`.
    let ridge = |f: f32, a: f32, w: f32| a * (1.0 - f.abs() / w).max(0.0).powi(2);
    let flank = |p: Vec3| {
        smoothstep(
            form.half_width(p.x) - 0.20,
            form.half_width(p.x) - 0.06,
            p.z.abs(),
        )
    };
    let ends = |p: Vec3, from: f32, to: f32| {
        smoothstep(form.tail + from, form.tail + to, p.x)
            * smoothstep(form.nose - from, form.nose - to, p.x)
    };
    // The shoulder: rising a little toward the tail, as a wedge does.
    let style = spec.style;
    let shoulder = |p: Vec3| {
        p.y - (form.belt(p.x) - style.crease_drop + style.wedge * (form.nose - p.x) / length)
    };
    // The hollow above the sill.
    let hollow = |p: Vec3| p.y - (form.floor(p.x) + 0.26);
    let from_axle = |p: Vec3| {
        form.to_axle(p.x, p.y, -half_base)
            .min(form.to_axle(p.x, p.y, half_base))
    };
    let between = |p: Vec3| smoothstep(arch + 0.02, arch + 0.16, from_axle(p));
    // Bonnet creases, from the screen pillars in toward the grille.
    let bonnet_line = |p: Vec3| {
        let t = ((p.x - front_foot) / (form.nose - front_foot)).clamp(0.0, 1.0);
        p.z.abs() - form.half_width(p.x) * (0.66 - 0.20 * t)
    };
    let on_bonnet = |p: Vec3| {
        smoothstep(form.belt(p.x) - 0.03, form.belt(p.x) + 0.0, p.y)
            * smoothstep(front_foot + 0.06, front_foot + 0.25, p.x)
            * smoothstep(form.nose - 0.10, form.nose - 0.32, p.x)
    };
    // The wing stands out round the arch and falls back into the door: a
    // flare a hand wide, with the lip of the arch itself at its edge.
    let lip_wide = 0.18;
    let relief = |p: Vec3| -> f32 {
        let side = flank(p);
        let mut h = 0.0;
        if side > 0.0 {
            h += ridge(shoulder(p), style.crease, 0.06) * side * ends(p, 0.30, 0.75) * between(p);
            h -= ridge(hollow(p), style.hollow, 0.10) * side * ends(p, 0.5, 0.9) * between(p);
            let t = ((from_axle(p) - arch) / lip_wide).clamp(0.0, 1.0);
            let lip = (1.0 - (from_axle(p) - arch) / 0.035).clamp(0.0, 1.0);
            h += (style.flare * (1.0 - t).powi(2) * (1.0 + 2.0 * t) + 0.005 * lip * lip) * side;
        }
        let top = on_bonnet(p);
        if top > 0.0 {
            h += ridge(bonnet_line(p), style.bonnet, 0.06) * top;
        }
        h
    };
    // Vertices along every line and across every slope of the relief.
    let lines: [(&dyn Fn(Vec3) -> f32, f32); 5] = [
        (&shoulder, 0.06),
        (&hollow, 0.10),
        (&bonnet_line, 0.06),
        (&|p| from_axle(p) - arch - lip_wide * 0.5, lip_wide * 0.5),
        (&|p| from_axle(p) - arch - 0.035, 0.035),
    ];
    for (line, wide) in lines {
        for level in [-1.0, 0.0, 1.0] {
            // Only where there is relief to carry.
            soup.score_in(&|p| line(p) - level * wide, &|t| {
                let c = t.centre();
                line(c).abs() < wide + 0.12 && (flank(c) > 0.0 || on_bonnet(c) > 0.0)
            });
        }
    }
    soup.emboss(&all, &relief);
}

/// The closed shell as a surface. A face is nearly flat and square to the
/// car's length, so which way it looks is settled here outright (away from the
/// car) and not left to the lie of its rings.
fn shell_soup(form: &Form, shell: &Shell) -> Soup {
    let rings = shell.closed(form);
    let normals = crate::model::skin_normals(&rings);
    let ends = shell.face_rings();
    let rows: Vec<Vec<super::surface::Vert>> = rings
        .iter()
        .zip(&normals)
        .enumerate()
        .map(|(i, (ring, normals))| {
            let toward = if i < ends {
                -1.0
            } else if i >= rings.len() - ends {
                1.0
            } else {
                0.0
            };
            let around = ring.len();
            ring.iter()
                .zip(normals)
                .enumerate()
                .map(|(k, (p, n))| {
                    if toward == 0.0 {
                        return super::surface::Vert::new(*p, *n);
                    }
                    // On a face: the surface's own normal from its
                    // neighbours along and around (the middle point
                    // included), looking away from the inside of the car.
                    let before = &rings[i.saturating_sub(1)];
                    let after = &rings[(i + 1).min(rings.len() - 1)];
                    let du = ring[(k + 1) % around] - ring[(k + around - 1) % around];
                    let dv = after[k] - before[k];
                    let inside = Vec3::new(p.x - toward * 0.6, (p.y + 0.6) * 0.5, 0.0);
                    let mut normal = du.cross(dv);
                    if normal.length() < 1.0e-9 {
                        normal = Vec3::new(toward, 0.0, 0.0);
                    }
                    let mut normal = normal.normalized_or_up();
                    if normal.dot(*p - inside) < 0.0 {
                        normal = -normal;
                    }
                    super::surface::Vert::new(*p, normal)
                })
                .collect()
        })
        .collect();
    Soup::from_grid(&rows, true, label::BODY)
}

/// Where the door handles are: the station of each, front door then rear.
pub(super) fn handle_places(form: &Form) -> Vec<(f32, bool)> {
    let lines = lines_of(form.spec);
    let mut places = vec![(lines.b_pillar + 0.17, true)];
    if form.spec.four_doors {
        places.push((lines.rear_door + 0.17, false));
    }
    places
}
