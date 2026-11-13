//! Lamps, built the way lamps are: a clear lens that is a piece of the body's
//! own skin, a housing behind it, and in the housing what makes the light.
//!
//! A lamp is drawn once, in elevation, on its end of the car (`Form::unwrap`
//! carries the drawing round the corner onto the wing). Its outline is cut out
//! of the shell, so the lens shares the body's surface and the body's edge.
//! Behind it the same patch, sunk, is the back of the housing, and the units
//! stand in that: projector lenses in their shrouds, reflector bowls with a
//! bulb, square LED modules, light guides that follow the lamp's outline,
//! rows of single LEDs. Tail lamps the same in red and amber. The grille is a
//! lamp without a lens: an opening with bars strung across it.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;

use super::form::Form;
use super::label;
use super::parts::{ball, dome, lathe, lathe_arc, sd_box, sd_convex, slab, sweep, wall};
use super::surface::{Soup, Tri, Vert, runs, spaced};

/// A lamp's outline on its end of the car: a quadrilateral from an inner end
/// (toward the centre line) to an outer end round the corner.
#[derive(Clone, Copy)]
pub(super) struct Wing {
    /// The inner end as a share of the face's half-width, and how far the
    /// outer end wraps past the corner along the wing, in metres.
    pub s: (f32, f32),
    /// The height of the lamp's middle at the inner and outer end, from the
    /// top of the face (down is negative).
    pub y: (f32, f32),
    /// The lamp's depth, top to bottom, at each end.
    pub h: (f32, f32),
    /// Corner radius.
    pub r: f32,
}

/// What a headlamp is besides its wing.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Extra {
    None,
    /// A hook: a blade of light dropping from the outer end, so long and so wide.
    Fang(f32, f32),
    /// A second tier: the wing is only the running light, and the lamps
    /// proper are in a pod of their own this far below it, so wide and so high.
    Pod(f32, f32, f32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum HeadStyle {
    /// Reflector bowls with bulbs, all chrome.
    Halogen,
    /// Projector lenses, a light guide under them.
    Projector,
    /// A row of square LED modules under a brow of single LEDs.
    Matrix,
    /// A slit: a blade of light and pin-point modules.
    Slim,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum TailStyle {
    /// Red lens over a reflector, with a clear and an amber window.
    Cluster,
    /// Glowing guides that follow the outline, under a dark lens.
    Guide,
    /// Guides, and rows of single LEDs in a bar across the tail.
    Bar,
    /// Upright: brake, turn and reverse stacked.
    Stack,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum GrilleKind {
    /// So many horizontal slats.
    Slats(usize),
    /// So many upright bars each side.
    Bars(usize),
    /// A diagonal mesh.
    Mesh,
}

/// An opening in a face: half-widths at its top and bottom as shares of the
/// face's half-width, and its bottom and top heights (a grille's from the top
/// of the face, an intake's from the floor).
#[derive(Clone, Copy)]
pub(super) struct Grille {
    pub half: (f32, f32),
    pub y: (f32, f32),
    pub r: f32,
    pub kind: GrilleKind,
    pub bright: bool,
    /// Parted down the middle into two openings.
    pub twin: bool,
}

/// A lamp's outline placed on a particular car.
#[derive(Clone, Copy)]
struct Shape {
    s: (f32, f32),
    y: (f32, f32),
    h: (f32, f32),
    r: f32,
    bar: Option<(f32, f32)>,
    extra: Extra,
    /// Drawn as a C, open toward the middle of the car.
    notch: bool,
}

impl Shape {
    fn new(wing: &Wing, bar: Option<(f32, f32)>, half: f32, top: f32) -> Self {
        Self {
            s: (wing.s.0 * half, half + wing.s.1),
            y: (top + wing.y.0, top + wing.y.1),
            h: wing.h,
            r: wing.r,
            bar: bar.map(|(y, h)| (top + y, h)),
            extra: Extra::None,
            notch: false,
        }
    }
    /// The middle of the pod of a two-tier lamp, on the drawing.
    fn pod(&self) -> Option<((f32, f32), (f32, f32))> {
        match self.extra {
            Extra::Pod(drop, wide, high) => {
                let (s, y) = self.at(0.45, 0.0);
                Some(((s, y - drop), (wide * 0.5, high * 0.5)))
            }
            _ => None,
        }
    }
    /// From the lamp's own coordinates (`u` from the inner end to the outer,
    /// `v` from the bottom edge, -1, to the top, 1) to the drawing.
    fn at(&self, u: f32, v: f32) -> (f32, f32) {
        let lerp = |a: (f32, f32)| a.0 + (a.1 - a.0) * u;
        (lerp(self.s), lerp(self.y) + v * lerp(self.h) * 0.5)
    }
    fn uv(&self, p: (f32, f32)) -> (f32, f32) {
        let u = (p.0 - self.s.0) / (self.s.1 - self.s.0);
        let lerp = |a: (f32, f32)| a.0 + (a.1 - a.0) * u.clamp(-0.5, 1.5);
        (u, (p.1 - lerp(self.y)) / (lerp(self.h) * 0.5))
    }
    fn field(&self, p: (f32, f32)) -> f32 {
        let quad = [
            (self.s.0, self.y.0 - self.h.0 * 0.5),
            (self.s.1, self.y.1 - self.h.1 * 0.5),
            (self.s.1, self.y.1 + self.h.1 * 0.5),
            (self.s.0, self.y.0 + self.h.0 * 0.5),
        ];
        let mut d = sd_convex(&quad, p, self.r);
        if self.notch {
            let (s, y) = self.at(0.16, 0.0);
            let bite = sd_box(
                (s, y),
                ((self.s.1 - self.s.0) * 0.30, self.h.0 * 0.19),
                p,
                0.008,
            );
            d = d.max(-bite);
        }
        match self.extra {
            Extra::Fang(long, wide) => {
                let (s, y) = self.at(0.80, -1.0);
                let blade = sd_box(
                    (s, y - long * 0.5 + 0.012),
                    (wide * 0.5, long * 0.5 + 0.012),
                    p,
                    wide * 0.4,
                );
                d = d.min(blade);
            }
            Extra::Pod(..) => {
                if let Some((at, half)) = self.pod() {
                    d = d.min(sd_box(at, half, p, half.1 * 0.6));
                }
            }
            Extra::None => {}
        }
        if let Some((y, h)) = self.bar {
            let half = self.s.0 * 0.5 + 0.03;
            d = d.min(sd_box((half - 0.03, y), (half, h * 0.5), p, h * 0.3));
        }
        d
    }
}

fn head_shape(form: &Form) -> Shape {
    let spec = form.spec;
    let mut shape = Shape::new(
        &spec.head,
        spec.head_bar,
        form.face_half(true),
        form.face_top(true),
    );
    shape.extra = spec.head_extra;
    shape
}

fn tail_shape(form: &Form) -> Shape {
    let spec = form.spec;
    let mut shape = Shape::new(
        &spec.tail,
        spec.tail_bar,
        form.face_half(false),
        form.face_top(false),
    );
    shape.notch = spec.tail_notch;
    shape
}

/// An opening's outline on the front face.
fn opening(grille: &Grille, half: f32, base: f32, p: (f32, f32)) -> f32 {
    let (low, high) = (base + grille.y.0, base + grille.y.1);
    let (top, bottom) = (grille.half.0 * half, grille.half.1 * half);
    let whole = sd_convex(
        &[(-bottom, low), (bottom, low), (top, high), (-top, high)],
        p,
        grille.r,
    );
    if grille.twin {
        // A bar of bodywork down the middle leaves two openings.
        super::parts::round_and(&[whole, 0.032 - p.0.abs()], grille.r.min(0.03))
    } else {
        whole
    }
}

/// The grille as it fits this car: never lower than the bar over the intake.
fn fitted_grille(form: &Form) -> Option<Grille> {
    let mut grille = form.spec.grille?;
    let floor = form.floor(form.nose) + form.spec.intake.y.1 + 0.035 - form.face_top(true);
    grille.y.0 = grille.y.0.max(floor);
    (grille.y.1 - grille.y.0 > 0.04).then_some(grille)
}

fn grille_field(form: &Form, p: Vec3) -> f32 {
    match &fitted_grille(form) {
        Some(grille) => opening(
            grille,
            form.face_half(true),
            form.face_top(true),
            (p.z, p.y),
        )
        .max(form.nose - 0.5 - p.x),
        None => 1.0,
    }
}

fn intake_field(form: &Form, p: Vec3) -> f32 {
    let middle = opening(
        &form.spec.intake,
        form.face_half(true),
        form.floor(form.nose),
        (p.z, p.y),
    )
    .max(form.nose - 0.3 - p.x);
    if form.spec.fog {
        return middle;
    }
    // A pocket in each corner of the bumper where there is no fog lamp: the
    // intake that feeds the brakes, its inner edge leaning with the grille's.
    let half = form.face_half(true);
    let floor = form.floor(form.nose);
    let high = form.face_top(true) - floor;
    let pocket = sd_convex(
        &[
            (half * 0.64, floor + high * 0.12 + 0.03),
            (half * 0.94, floor + high * 0.16 + 0.03),
            (half * 0.94, floor + high * 0.46),
            (half * 0.74, floor + high * 0.38),
        ],
        form.unwrap(true, p),
        0.018,
    )
    .max(-p.x);
    middle.min(pocket)
}

const FOG_R: f32 = 0.040;

/// Where a fog lamp sits on the front's drawing: out beyond the intake.
fn fog_place(form: &Form) -> (f32, f32) {
    let intake = &form.spec.intake;
    let half = form.face_half(true);
    let s = (intake.half.0.max(intake.half.1) * half + 0.085).min(half * 0.90);
    (
        s,
        form.floor(form.nose) + (intake.y.0 + intake.y.1) * 0.5 + 0.01,
    )
}

/// Cut the lamps and openings out of the shell.
pub(super) fn cut(soup: &mut Soup, form: &Form) {
    let spec = form.spec;
    let body = |l: u8| l == label::BODY;
    let (head, tail) = (head_shape(form), tail_shape(form));
    let head_field = |p: Vec3| head.field(form.unwrap(true, p)).max(0.2 - p.x);
    let tail_field = |p: Vec3| tail.field(form.unwrap(false, p)).max(p.x + 0.2);
    soup.cut(&head_field, &body, label::HEAD);
    soup.cut(&tail_field, &body, label::TAIL);
    // The windows of a tail lamp that are not red.
    let in_tail = |l: u8| l == label::TAIL;
    let window = |u: (f32, f32), v: (f32, f32)| {
        move |p: Vec3| {
            let (pu, pv) = tail.uv(form.unwrap(false, p));
            // In the lamp's own coordinates, scaled back to metres near enough.
            let du = ((pu - (u.0 + u.1) * 0.5).abs() - (u.1 - u.0) * 0.5) * (tail.s.1 - tail.s.0);
            let dv = ((pv - (v.0 + v.1) * 0.5).abs() - (v.1 - v.0) * 0.5) * tail.h.0 * 0.5;
            let r = 0.008;
            (du + r).max(dv + r).min(0.0)
                + ((du + r).max(0.0).powi(2) + (dv + r).max(0.0).powi(2)).sqrt()
                - r
        }
    };
    match spec.tail_style {
        TailStyle::Cluster => {
            soup.cut(
                &window((0.06, 0.34), (-0.80, -0.05)),
                &in_tail,
                label::CLEAR,
            );
            soup.cut(
                &window((0.40, 0.72), (-0.80, -0.05)),
                &in_tail,
                label::AMBER,
            );
        }
        TailStyle::Stack => {
            soup.cut(&window((0.12, 0.88), (-0.30, 0.22)), &in_tail, label::AMBER);
            soup.cut(
                &window((0.12, 0.88), (-0.90, -0.40)),
                &in_tail,
                label::CLEAR,
            );
        }
        _ => {}
    }
    // Openings: the grille and the intake under the bumper.
    soup.cut(&|p| grille_field(form, p), &body, label::GRILLE);
    soup.cut(&|p| intake_field(form, p), &body, label::INTAKE);
    if spec.fog {
        let (s, y) = fog_place(form);
        soup.cut(
            &|p: Vec3| {
                let (ps, py) = form.unwrap(true, p);
                ((ps - s).powi(2) + (py - y).powi(2)).sqrt() - FOG_R
            },
            &body,
            label::FOG,
        );
        soup.cut(
            &|p: Vec3| {
                let (ps, py) = form.unwrap(true, p);
                (((ps - s).powi(2) + (py - y).powi(2)).sqrt() - FOG_R - 0.004).max(-p.x)
            },
            &body,
            label::SEAM,
        );
    }
    // Reflectors low in the rear bumper, and a repeater on the wing if the
    // mirror does not carry it.
    let half = form.face_half(false);
    let low = form.floor(form.tail) + 0.215;
    soup.cut(
        &|p: Vec3| {
            let (s, y) = form.unwrap(false, p);
            sd_box((half * 0.80, low), (0.075, 0.013), (s, y), 0.008).max(p.x + 0.2)
        },
        &body,
        label::REFLECT,
    );
    if !spec.mirror.repeater {
        let x = spec.wheelbase * 0.5 + spec.wheel_r + 0.20;
        let x = x.min(form.nose - 0.38);
        let y = form.belt(x) - 0.17;
        soup.cut(
            &|p: Vec3| sd_box((x, y), (0.032, 0.010), (p.x, p.y), 0.008),
            &body,
            label::MARKER,
        );
    }
    // The fine dark line round every lens and opening: the gap it is set in with.
    let gaps: [&dyn Fn(Vec3) -> f32; 4] =
        [&head_field, &tail_field, &|p| grille_field(form, p), &|p| {
            intake_field(form, p)
        }];
    for gap in gaps {
        soup.cut(&|p| gap(p) - 0.0035, &body, label::SEAM);
    }
}

/// One lamp on one side of the car: its lens, and where things stand behind it.
struct Bay<'a> {
    form: &'a Form<'a>,
    lens: Soup,
    shape: Shape,
    front: bool,
    depth: f32,
}

impl Bay<'_> {
    fn map(&self, p: Vec3) -> (f32, f32) {
        self.form.unwrap(self.front, p)
    }
    /// The lens at a place of the lamp.
    fn on(&self, u: f32, v: f32) -> Option<Vert> {
        self.lens
            .locate(&|_| true, &|p| self.map(p), self.shape.at(u, v))
    }
    /// A seat `sink` under the lens, and the way a unit there faces: out along
    /// the car more than square to the lens, as a beam does.
    fn seat(&self, u: f32, v: f32, sink: f32) -> Option<(Vec3, Vec3)> {
        let at = self.on(u, v)?;
        let along = Vec3::new(if self.front { 1.0 } else { -1.0 }, 0.0, 0.0);
        Some((at.p - at.n * sink, (along * 0.8 + at.n).normalized_or_up()))
    }
    /// A line across the lamp at a height `v`, `sink` under the lens.
    fn line(&self, u: (f32, f32), v: f32, count: usize, sink: f32) -> Vec<Vert> {
        (0..=count)
            .filter_map(|k| self.on(u.0 + (u.1 - u.0) * k as f32 / count as f32, v))
            .map(|at| at.sunk(sink))
            .collect()
    }
    /// The lamp's own outline drawn `inset` inside it and `sink` under the
    /// lens, where `keep` (in the lamp's `u, v`) allows.
    fn outline(&self, inset: f32, sink: f32, keep: &dyn Fn(f32, f32) -> bool) -> Vec<Vec<Vert>> {
        let field = |p: Vec3| self.shape.field(self.map(p)) + inset;
        let mut out = Vec::new();
        for curve in self.lens.contour(&field, &|_| true) {
            let kept = runs(&curve, &|v| {
                let (u, w) = self.shape.uv(self.map(v.p));
                keep(u, w)
            });
            for run in kept {
                out.push(run.into_iter().map(|v| v.sunk(sink)).collect());
            }
        }
        out
    }
    /// The lamp's depth at `u`.
    fn high(&self, u: f32) -> f32 {
        self.shape.h.0 + (self.shape.h.1 - self.shape.h.0) * u
    }
}

/// The back and the wall of a pocket behind a patch of the surface.
fn pocket(builder: &mut MeshBuilder, patch: &Soup, depth: f32, back: &str, side: &str) {
    patch.sunk(depth).emit(builder, back, 0.0, &|_| true);
    for edge in patch.boundary(&|_| true) {
        wall(builder, side, &edge, &|v| v.n * -depth, true);
    }
}

/// A projector: a glass lens in a bright shroud.
fn projector(
    builder: &mut MeshBuilder,
    key: &dyn Fn(&str) -> String,
    at: Vec3,
    axis: Vec3,
    r: f32,
) {
    lathe(
        builder,
        &key("chrome"),
        at,
        axis,
        &[
            (r * 1.02, -r * 0.5),
            (r * 1.06, r * 0.10),
            (r * 1.18, r * 0.22),
            (r * 1.36, r * 0.18),
            (r * 1.42, -r * 0.5),
        ],
        28,
        true,
    );
    // Behind the lens, the dark of the unit.
    lathe(
        builder,
        &key("well"),
        at,
        axis,
        &[(r * 1.02, -r * 0.25), (0.0, -r * 0.25)],
        20,
        false,
    );
    dome(
        builder,
        &key("glass"),
        at - axis * (r * 0.2),
        axis,
        r,
        r * 0.62,
        24,
    );
}

/// A reflector bowl with its bulb.
fn reflector(
    builder: &mut MeshBuilder,
    key: &dyn Fn(&str) -> String,
    at: Vec3,
    axis: Vec3,
    r: f32,
    bulb: &str,
) {
    let deep = r * 0.85;
    let profile: Vec<(f32, f32)> = (0..=8)
        .map(|k| {
            let t = 1.0 - k as f32 / 8.0;
            (r * t, -deep * (1.0 - t * t))
        })
        .collect();
    lathe(builder, &key("chrome"), at, axis, &profile, 28, false);
    lathe(
        builder,
        &key("chrome"),
        at,
        axis,
        &[(r, 0.0), (r * 1.08, 0.004), (r * 1.10, -0.004)],
        28,
        true,
    );
    ball(
        builder,
        &key(bulb),
        at - axis * (deep * 0.55),
        axis,
        r * 0.2,
        1.5,
        12,
    );
}

/// A square LED module: a little bright cup with a lens at the bottom of it.
fn module(
    builder: &mut MeshBuilder,
    key: &dyn Fn(&str) -> String,
    at: Vec3,
    axis: Vec3,
    size: f32,
) {
    lathe_arc(
        builder,
        &key("chrome"),
        at,
        axis,
        &[
            (size * 1.42, 0.0),
            (size * 0.80, -size * 0.9),
            (size * 0.80, -size * 0.9),
            (0.0, -size * 0.9),
        ],
        4,
        false,
        (0.0, 1.0),
        0.125,
    );
    dome(
        builder,
        &key("glass"),
        at - axis * (size * 0.9),
        axis,
        size * 0.52,
        size * 0.34,
        14,
    );
}

/// A row of single LEDs along curves.
fn beads(builder: &mut MeshBuilder, key: &str, curves: &[Vec<Vert>], pitch: f32, r: f32) {
    for curve in curves {
        for at in spaced(curve, pitch) {
            dome(builder, key, at.p, at.n, r, r * 0.8, 6);
        }
    }
}

/// Light guides along curves.
fn guides(builder: &mut MeshBuilder, key: &str, curves: &[Vec<Vert>], half_w: f32) {
    for curve in curves {
        sweep(builder, key, curve, half_w, half_w * 0.7, 0.0, 8);
    }
}

fn bay<'a>(form: &'a Form<'a>, soup: &Soup, front: bool, side: f32, depth: f32) -> Bay<'a> {
    let here = move |t: &Tri| {
        let c = t.centre();
        c.z * side >= 0.0 && (c.x > 0.0) == front
    };
    let main = if front { label::HEAD } else { label::TAIL };
    Bay {
        form,
        lens: soup.take(&|t| {
            here(t)
                && (t.label == main || (!front && matches!(t.label, label::CLEAR | label::AMBER)))
        }),
        shape: if front {
            head_shape(form)
        } else {
            tail_shape(form)
        },
        front,
        depth,
    }
}

/// The lenses, the housings and everything in them; the grille and intake.
pub(super) fn build(
    builder: &mut MeshBuilder,
    soup: &Soup,
    form: &Form,
    key: &dyn Fn(&str) -> String,
) {
    let spec = form.spec;
    // Lenses: the shell's own triangles, drawn as glass.
    for (labels, part) in [
        (&[label::HEAD, label::CLEAR, label::FOG][..], "lens"),
        (&[label::TAIL, label::REFLECT][..], "lens.red"),
        (&[label::AMBER, label::MARKER][..], "lamp.amber"),
    ] {
        soup.emit(builder, &key(part), 0.0, &|t| labels.contains(&t.label));
    }
    // Small lamps: a shallow bright pocket is all there is behind them.
    let small = soup.take(&|t| matches!(t.label, label::REFLECT | label::MARKER));
    pocket(builder, &small, 0.006, &key("chrome"), &key("chrome"));

    for side in [-1.0_f32, 1.0] {
        // --- fog lamp: a small projector in a round housing ---
        let fog = soup.take(&|t| t.label == label::FOG && t.centre().z * side > 0.0);
        if !fog.tris.is_empty() {
            pocket(builder, &fog, 0.05, &key("well"), &key("chrome"));
            if let Some(at) = fog.locate(&|_| true, &|p| form.unwrap(true, p), fog_place(form)) {
                let axis = (Vec3::new(1.0, 0.0, 0.0) + at.n * 0.5).normalized_or_up();
                projector(builder, key, at.p - at.n * 0.034, axis, FOG_R * 0.55);
            }
        }
        // --- headlamp ---
        let slim = spec.head_style == HeadStyle::Slim;
        let lamp = bay(form, soup, true, side, if slim { 0.04 } else { 0.065 });
        let bright = spec.head_style == HeadStyle::Halogen;
        let lining = if bright { key("chrome") } else { key("well") };
        pocket(builder, &lamp.lens, lamp.depth, &lining, &lining);
        let h = lamp.shape.h.0;
        match spec.head_style {
            HeadStyle::Halogen => {
                for (u, share, bulb) in [
                    (0.30, 0.40, "lamp.front"),
                    (0.62, 0.34, "lamp.front"),
                    (0.88, 0.22, "lamp.amber"),
                ] {
                    let r = (lamp.high(u) * share).min(0.075);
                    if let Some((at, axis)) = lamp.seat(u, 0.0, r * 0.25 + 0.006) {
                        reflector(builder, key, at, axis, r, bulb);
                    }
                }
            }
            HeadStyle::Projector => {
                for (u, share) in [(0.26, 0.30), (0.52, 0.25)] {
                    let r = lamp.high(u) * share;
                    if let Some((at, axis)) = lamp.seat(u, 0.12, r * 0.75 + 0.012) {
                        projector(builder, key, at, axis, r);
                    }
                }
                // The signature: a guide along the lower edge and up the inner end.
                let guide = lamp.outline(0.013, lamp.depth - 0.022, &|u, v| v < -0.1 || u < 0.12);
                guides(builder, &key("lamp.front"), &guide, 0.0055);
                let turn = lamp.line((0.70, 0.94), 0.15, 10, lamp.depth - 0.016);
                beads(builder, &key("lamp.amber"), &[turn], 0.017, 0.0058);
            }
            HeadStyle::Matrix => {
                let size = (h * 0.22).clamp(0.012, 0.024);
                for k in 0..4 {
                    let u = 0.16 + 0.15 * k as f32;
                    if let Some((at, axis)) = lamp.seat(u, -0.22, size + 0.010) {
                        module(builder, key, at, axis, size);
                    }
                }
                // A brow of single LEDs along the top edge, a guide under them.
                let brow = lamp.outline(0.012, lamp.depth - 0.018, &|_, v| v > 0.25);
                beads(builder, &key("lamp.front"), &brow, 0.0135, 0.0048);
                let under = lamp.outline(0.024, lamp.depth - 0.012, &|_, v| v > 0.25);
                guides(builder, &key("trim"), &under, 0.004);
                let turn = lamp.outline(0.012, lamp.depth - 0.018, &|u, v| v < -0.25 && u > 0.55);
                beads(builder, &key("lamp.amber"), &turn, 0.0135, 0.0048);
            }
            HeadStyle::Slim => {
                let u0 = if lamp.shape.bar.is_some() {
                    -lamp.shape.s.0 / (lamp.shape.s.1 - lamp.shape.s.0) + 0.02
                } else {
                    0.05
                };
                let blade = lamp.line((u0, 0.95), 0.36, 40, lamp.depth - 0.016);
                guides(builder, &key("lamp.front"), &[blade], 0.0050);
                let size = (h * 0.20).clamp(0.008, 0.014);
                for k in 0..5 {
                    let u = 0.10 + 0.12 * k as f32;
                    if let Some((at, axis)) = lamp.seat(u, -0.34, size + 0.008) {
                        module(builder, key, at, axis, size);
                    }
                }
                let turn = lamp.line((0.70, 0.95), -0.30, 8, lamp.depth - 0.016);
                beads(builder, &key("lamp.amber"), &[turn], 0.012, 0.0042);
            }
        }

        // The lower tier of a two-tier lamp: the lamps proper, in their pod.
        if let Some((at, half)) = lamp.shape.pod() {
            let size = (half.1 * 0.62).min(0.03);
            for k in [-1.0_f32, 0.0, 1.0] {
                let place = (at.0 + k * half.0 * 0.56, at.1);
                if let Some(seat) = lamp.lens.locate(&|_| true, &|p| lamp.map(p), place) {
                    let axis = (Vec3::new(1.0, 0.0, 0.0) * 0.8 + seat.n).normalized_or_up();
                    module(builder, key, seat.p - seat.n * (size + 0.012), axis, size);
                }
            }
        }

        // --- tail lamp ---
        let lamp = bay(form, soup, false, side, 0.045);
        let bright = matches!(spec.tail_style, TailStyle::Cluster | TailStyle::Stack);
        let lining = if bright { key("chrome") } else { key("well") };
        pocket(builder, &lamp.lens, lamp.depth, &lining, &lining);
        match spec.tail_style {
            TailStyle::Cluster => {
                for (u, v, share, bulb) in [
                    (0.20, -0.42, 0.20, "lamp.front"),
                    (0.56, -0.42, 0.20, "lamp.amber"),
                    (0.80, 0.30, 0.24, "lamp.rear"),
                ] {
                    let r = lamp.high(u) * share;
                    if let Some((at, axis)) = lamp.seat(u, v, r * 0.3 + 0.006) {
                        reflector(builder, key, at, axis, r, bulb);
                    }
                }
                // The tail light proper: a row of LEDs along the top.
                let row = lamp.line((0.06, 0.62), 0.48, 30, lamp.depth - 0.014);
                beads(builder, &key("lamp.rear"), &[row], 0.014, 0.005);
            }
            TailStyle::Guide | TailStyle::Bar => {
                for inset in [0.014, 0.014 + lamp.shape.h.0 * 0.22] {
                    let ring = lamp.outline(inset, lamp.depth - 0.018, &|u, _| u > 0.02);
                    guides(builder, &key("lamp.rear"), &ring, 0.0052);
                }
                let turn = lamp.line((0.34, 0.86), 0.0, 20, lamp.depth - 0.014);
                beads(builder, &key("lamp.amber"), &[turn], 0.013, 0.0042);
                if let Some((_, bar_h)) = lamp.shape.bar {
                    // Rows of single LEDs the width of the car.
                    let u0 = -lamp.shape.s.0 / (lamp.shape.s.1 - lamp.shape.s.0) + 0.005;
                    let rows: &[f32] = if spec.tail_style == TailStyle::Bar {
                        &[-0.5, 0.5]
                    } else {
                        &[0.0]
                    };
                    for row in rows {
                        let v = row * bar_h * 0.5 / (lamp.shape.h.0 * 0.5);
                        let line = lamp.line((u0, 0.0), v, 36, lamp.depth - 0.014);
                        if spec.tail_style == TailStyle::Bar {
                            beads(builder, &key("lamp.rear"), &[line], 0.011, 0.0040);
                        } else {
                            guides(builder, &key("lamp.rear"), &[line], 0.005);
                        }
                    }
                }
            }
            TailStyle::Stack => {
                for (v, bulb) in [
                    (0.62, "lamp.rear"),
                    (-0.04, "lamp.amber"),
                    (-0.65, "lamp.front"),
                ] {
                    let r = (lamp.shape.s.1 - lamp.shape.s.0).min(lamp.shape.h.0 * 0.3) * 0.36;
                    if let Some((at, axis)) = lamp.seat(0.5, v, r * 0.3 + 0.006) {
                        reflector(builder, key, at, axis, r, bulb);
                    }
                }
            }
        }
    }

    // --- grille and intake ---
    let dark = key("trim");
    let openings = [
        (label::GRILLE, fitted_grille(form), form.face_top(true)),
        (label::INTAKE, Some(spec.intake), form.floor(form.nose)),
    ];
    for (which, grille, base) in openings {
        let Some(grille) = grille else { continue };
        let patch = soup.take(&|t| t.label == which);
        if patch.tris.is_empty() {
            continue;
        }
        let depth = 0.04;
        pocket(builder, &patch, depth, &key("well"), &dark);
        let back = patch.sunk(depth);
        let bars = if grille.bright {
            key("chrome")
        } else {
            dark.clone()
        };
        let all = |_: u8| true;
        match grille.kind {
            GrilleKind::Slats(count) => {
                for k in 0..count {
                    let y = base
                        + grille.y.0
                        + (grille.y.1 - grille.y.0) * (k as f32 + 0.5) / count as f32;
                    for curve in back.contour(&|p| p.y - y, &all) {
                        sweep(builder, &bars, &curve, 0.0075, 0.014, depth * 0.55, 8);
                    }
                }
            }
            GrilleKind::Bars(count) => {
                let half = grille.half.0.max(grille.half.1) * form.face_half(true);
                for k in 0..count {
                    let z = half * (k as f32 + 0.5) / count as f32;
                    for curve in back.contour(&|p| p.z.abs() - z, &all) {
                        sweep(builder, &bars, &curve, 0.0065, 0.014, depth * 0.55, 8);
                    }
                }
            }
            GrilleKind::Mesh => {
                let pitch = 0.040;
                let half = grille.half.0.max(grille.half.1) * form.face_half(true) + 0.3;
                let lines = (half * 2.0 / pitch) as i32;
                for lean in [-1.0_f32, 1.0] {
                    for k in -lines..=lines {
                        let c = k as f32 * pitch;
                        for curve in back.contour(&|p| p.z + lean * p.y * 1.1 - c, &all) {
                            sweep(builder, &dark, &curve, 0.0032, 0.0045, depth * 0.45, 4);
                        }
                    }
                }
            }
        }
        if grille.bright {
            for edge in patch.boundary(&all) {
                sweep(builder, &key("chrome"), &edge, 0.007, 0.005, 0.001, 8);
            }
        }
    }
    // Parking sensors: four small discs let into each bumper.
    for front in [true, false] {
        let (end, top) = (
            if front { form.nose } else { form.tail },
            form.face_top(front),
        );
        let floor = form.floor(end);
        let half = form.face_half(front);
        let paint = soup.take(&|t| t.label == label::BODY && (t.centre().x > 0.0) == front);
        for side in [-1.0_f32, 1.0] {
            let here = paint.take(&|t| t.centre().z * side >= 0.0);
            for share in [0.24_f32, 0.60] {
                let place = (half * share, floor + (top - floor) * 0.30);
                if let Some(at) = here.locate(&|_| true, &|p| form.unwrap(front, p), place) {
                    lathe(
                        builder,
                        &key("trim"),
                        at.p,
                        at.n,
                        &[(0.0, 0.0018), (0.0095, 0.0016), (0.0115, 0.0)],
                        12,
                        true,
                    );
                }
            }
        }
    }
    // The radar's panel, low in the middle of the grille.
    if let Some(grille) = fitted_grille(form)
        && !grille.twin
        && let Some((at, normal)) = on_centre(
            soup,
            form,
            true,
            form.face_top(true) + grille.y.0 + 0.045,
            true,
        )
    {
        let up = Vec3::new(-normal.y, normal.x, 0.0) * if normal.x >= 0.0 { 1.0 } else { -1.0 };
        slab(
            builder,
            &key("trim"),
            at + normal * 0.012,
            normal,
            up * 0.028,
            Vec3::new(0.0, 0.0, 0.05),
            0.01,
        );
    }
    // The maker's badge on the nose, and on the tail.
    for front in [true, false] {
        let grille = fitted_grille(form);
        let y = match (&grille, front) {
            (Some(grille), true) => form.face_top(true) + (grille.y.0 + grille.y.1) * 0.5,
            (None, true) => form.face_top(true) - 0.02,
            (_, false) => form.face_top(false) - 0.035,
        };
        if let Some((at, normal)) = on_centre(
            soup,
            form,
            front,
            y,
            front && grille.is_some_and(|g| !g.twin),
        ) {
            lathe(
                builder,
                &key("chrome"),
                at,
                normal,
                &[(0.0, 0.010), (0.030, 0.009), (0.040, 0.004), (0.042, -0.02)],
                24,
                true,
            );
        }
    }
    plates(builder, soup, form, key);
}

/// The point of the centre line of an end at a height, and the face's normal
/// there: on the body's skin, or on the back of the grille's opening.
fn on_centre(
    soup: &Soup,
    form: &Form,
    front: bool,
    y: f32,
    in_grille: bool,
) -> Option<(Vec3, Vec3)> {
    let here = soup.take(&|t| {
        (t.centre().x > 0.0) == front
            && t.centre().z >= 0.0
            && (!in_grille || t.label == label::GRILLE)
    });
    let at = here.locate(&|_| true, &|p| form.unwrap(front, p), (0.03, y))?;
    // Things on the centre line stand nearly upright whatever the face does.
    let normal = Vec3::new(at.n.x, at.n.y * 0.35, 0.0).normalized_or_up();
    let sink = if in_grille { 0.022 } else { 0.0 };
    Some((Vec3::new(at.p.x, at.p.y, 0.0) - normal * sink, normal))
}

/// Number plates, front and back, each in its frame.
fn plates(builder: &mut MeshBuilder, soup: &Soup, form: &Form, key: &dyn Fn(&str) -> String) {
    let spec = form.spec;
    for front in [true, false] {
        let y = if front {
            // On the bumper, over the intake.
            let floor = form.floor(form.nose);
            (floor + spec.intake.y.1 + 0.085).max(floor + (form.face_top(true) - floor) * 0.34)
        } else {
            let lamp_low = form.face_top(false) + spec.tail.y.0 - spec.tail.h.0 * 0.5;
            if spec.tail_style == TailStyle::Stack {
                form.face_top(false) - 0.30
            } else {
                (lamp_low - 0.115).max(form.floor(form.tail) + 0.30)
            }
        };
        let Some((at, normal)) = on_centre(soup, form, front, y, false) else {
            continue;
        };
        let up = Vec3::new(-normal.y, normal.x, 0.0) * if normal.x >= 0.0 { 1.0 } else { -1.0 };
        let across = Vec3::new(0.0, 0.0, 1.0);
        // A plate is flat and the face is not: it stands off far enough to
        // clear the face under its edges.
        let here = soup.take(&|t| (t.centre().x > 0.0) == front && t.centre().z >= 0.0);
        let mut proud = 0.0_f32;
        for (s, dy) in [
            (0.03, -0.07),
            (0.03, 0.07),
            (0.22, -0.07),
            (0.22, 0.0),
            (0.22, 0.07),
        ] {
            if let Some(v) = here.locate(&|_| true, &|p| form.unwrap(front, p), (s, y + dy)) {
                proud = proud.max((v.p - at).dot(normal));
            }
        }
        let face = at + normal * (proud + 0.010);
        // A blue plate is 440 mm wide; a new-energy one, with a character
        // more, 480. Its face is one quad that the plate shader writes on.
        let wide = if spec.electric { 0.24 } else { 0.22 };
        let right = up.cross(normal).normalized_or_up();
        slab(
            builder,
            &key("trim"),
            face - normal * 0.002,
            normal,
            up * 0.074,
            across * (wide + 0.006),
            0.012,
        );
        let corner = |u: f32, v: f32| face + right * (wide * u) + up * (0.070 * v);
        builder.quad_uv(
            &key("plate"),
            corner(-1.0, -1.0),
            corner(1.0, -1.0),
            corner(1.0, 1.0),
            corner(-1.0, 1.0),
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            None,
        );
    }
}
