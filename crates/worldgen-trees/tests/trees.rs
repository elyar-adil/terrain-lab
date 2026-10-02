//! What a tree generator must keep true, on real output.

use std::collections::HashSet;

use worldgen_core::hash::hash_words;
use worldgen_trees::{LEAF_BUDGET, SPECIES, Tree, TreeSpec, WOOD_LEVEL, grow, by_key};

fn fingerprint(t: &Tree) -> u64 {
    let mut words = vec![t.segments.len() as u64, t.leaves.len() as u64];
    for s in t.segments.iter().step_by(7) {
        words.push(u64::from(s.a.x.to_bits()) << 32 | u64::from(s.b.y.to_bits()));
    }
    for l in t.leaves.iter().step_by(11) {
        words.push(u64::from(l.pos.x.to_bits()) << 32 | u64::from(l.dir.z.to_bits()));
    }
    hash_words(&words)
}

fn leaf_area(t: &Tree) -> f32 {
    let aspect = SPECIES[t.spec.species].leaf_aspect.min(1.1);
    t.leaves.iter().map(|l| 0.7 * aspect * l.length * l.length).sum()
}

#[test]
fn a_tree_is_a_pure_function_of_its_spec() {
    for species in [0, 6, 7, 9] {
        let spec = TreeSpec::typical(species, 42);
        let (a, b) = (grow(&spec, 0), grow(&spec, 0));
        assert_eq!(a.segments, b.segments);
        assert_eq!(a.leaves, b.leaves);
    }
}

#[test]
fn no_two_trees_are_alike_and_no_two_leaves_in_a_tree_are() {
    let mut seen = HashSet::new();
    for seed in 0..120u64 {
        let t = grow(&TreeSpec::typical(0, seed), 1);
        assert!(seen.insert(fingerprint(&t)), "seed {seed} grew a tree identical to an earlier one");
    }
    // Within one tree every leaf has its own outline parameters, tint and direction.
    let t = grow(&TreeSpec::typical(by_key("wu-tong").unwrap(), 7), 0);
    assert!(t.leaves.len() > 5000);
    let mut distinct = HashSet::new();
    for l in &t.leaves {
        distinct.insert((l.shape, l.tint, l.dir.x.to_bits(), l.length.to_bits()));
    }
    assert!(distinct.len() * 100 >= t.leaves.len() * 99, "{} distinct of {} leaves", distinct.len(), t.leaves.len());
    // And the sizes spread: not one size with noise, but a distribution.
    let mean = t.leaves.iter().map(|l| l.length).sum::<f32>() / t.leaves.len() as f32;
    let sd = (t.leaves.iter().map(|l| (l.length - mean).powi(2)).sum::<f32>() / t.leaves.len() as f32).sqrt();
    assert!(sd / mean > 0.10, "leaf size varies by only {:.1}%", 100.0 * sd / mean);
    let shapes: HashSet<u8> = t.leaves.iter().map(|l| l.shape[0] / 16).collect();
    assert!(shapes.len() >= 14, "outline parameter takes only {} values", shapes.len());
}

#[test]
fn trees_have_the_height_crown_and_proportions_of_their_species() {
    for (i, s) in SPECIES.iter().enumerate() {
        let spec = TreeSpec::typical(i, 1234);
        let t = grow(&spec, 0);
        let top = t.segments.iter().map(|g| g.b.y.max(g.a.y)).fold(0.0, f32::max);
        assert!((top - spec.height_m).abs() < 0.04 * spec.height_m + 0.3, "{}: grew to {top} not {}", s.key, spec.height_m);
        // Leaves sit inside the crown envelope (a little over, for ragged edges).
        let inside = t
            .leaves
            .iter()
            .filter(|l| (l.pos.x * l.pos.x + l.pos.z * l.pos.z).sqrt() < t.crown_radius * 1.45 && l.pos.y < spec.height_m * 1.04)
            .count();
        assert!(inside * 100 >= t.leaves.len() * 95, "{}: only {inside} of {} leaves are inside the crown", s.key, t.leaves.len());
        // A slender trunk: height over trunk diameter in the range of real trees.
        let slender = spec.height_m / (2.0 * t.trunk_radius);
        assert!((12.0..130.0).contains(&slender), "{}: slenderness {slender}", s.key);
        // Foliage hangs on a crown, not on the bare stem: nothing in leaf below a quarter of the clear stem.
        let lowest = t.leaves.iter().map(|l| l.pos.y).fold(f32::MAX, f32::min);
        assert!(lowest > 0.25 * t.crown_base, "{}: a leaf at {lowest} m on a trunk bare to {}", s.key, t.crown_base);
    }
}

#[test]
fn wood_is_continuous_and_tapers() {
    let t = grow(&TreeSpec::typical(0, 9), 0);
    // The trunk (level 0) thins as it rises.
    let trunk: Vec<_> = t.segments.iter().filter(|s| s.level == 0).collect();
    assert!(trunk.len() > 10);
    for w in trunk.windows(2) {
        assert!(w[1].ra <= w[0].ra + 1e-4, "trunk widens going up");
        assert!(w[0].b.dist(w[1].a) < 1e-4, "trunk is broken");
    }
    assert!(trunk[0].ra > 1.4 * trunk[trunk.len() / 3].ra, "no root flare");
    // Every piece of wood is attached to other wood: its start lies within its own radius
    // of another segment's axis (or is the foot of the trunk).
    let mut loose = 0;
    for (k, s) in t.segments.iter().enumerate() {
        if s.a.y < 0.01 {
            continue;
        }
        let attached = t.segments.iter().enumerate().any(|(j, o)| {
            if j == k {
                return false;
            }
            let d = o.b - o.a;
            let l2 = d.dot(d).max(1e-9);
            let u = ((s.a - o.a).dot(d) / l2).clamp(0.0, 1.0);
            (o.a + d * u).dist(s.a) <= s.ra.max(o.ra) * 1.2 + 0.01
        });
        loose += usize::from(!attached);
    }
    assert_eq!(loose, 0, "{loose} pieces of wood hang in the air");
}

#[test]
fn levels_of_detail_keep_the_tree_and_its_coverage() {
    let spec = TreeSpec::typical(by_key("yu-shu").unwrap(), 31);
    let full = grow(&spec, 0);
    let mut last_leaves = usize::MAX;
    for lod in 0..4u8 {
        let t = grow(&spec, lod);
        assert!(t.leaves.len() <= LEAF_BUDGET[lod as usize], "lod {lod}: {} leaves", t.leaves.len());
        assert!(t.leaves.len() < last_leaves, "a farther level draws fewer leaves");
        last_leaves = t.leaves.len();
        assert!(t.segments.iter().all(|s| s.level <= WOOD_LEVEL[lod as usize]));
        // Fewer, bigger leaves cover about as much crown as many small ones.
        let (a, b) = (leaf_area(&full), leaf_area(&t));
        assert!(b > 0.6 * a && b < 1.5 * a, "lod {lod}: leaf area {b} against {a}");
        // The same tree: same height, crown, trunk.
        assert_eq!((t.height, t.crown_radius, t.trunk_radius), (full.height, full.crown_radius, full.trunk_radius));
    }
    // The trunk and the limbs are the same wood at every level.
    let far = grow(&spec, 3);
    assert!(far.segments.iter().all(|s| full.segments.contains(s)));
}

#[test]
fn the_seasons_change_a_deciduous_crown_and_leave_an_evergreen_alone() {
    let ginkgo = by_key("yin-xing").unwrap();
    let camphor = by_key("xiang-zhang").unwrap();
    let at = |species, season| {
        let spec = TreeSpec { season, ..TreeSpec::typical(species, 5) };
        grow(&spec, 1)
    };
    let (winter, spring, summer, autumn) = (at(ginkgo, 0.02), at(ginkgo, 0.24), at(ginkgo, 0.55), at(ginkgo, 0.88));
    assert_eq!(winter.leaves.len(), 0, "a ginkgo in January has leaves");
    assert!(summer.leaves.len() > 2000);
    assert!(summer.autumn < 0.05 && autumn.autumn > 0.8, "{} -> {}", summer.autumn, autumn.autumn);
    assert!(spring.flush > 0.3 || spring.leaves.len() < summer.leaves.len());
    // The wood is the same in every season.
    assert_eq!(winter.segments, summer.segments);
    let (c_winter, c_summer) = (at(camphor, 0.02), at(camphor, 0.55));
    assert!(c_winter.leaves.len() * 10 > c_summer.leaves.len() * 8, "an evergreen holds its leaves");
    // A flowering tree blooms at its time.
    let peach = by_key("tao-shu").unwrap();
    assert!(at(peach, 0.16).bloom > 0.5 && at(peach, 0.6).bloom < 0.05);
}

#[test]
fn the_conditions_a_tree_grew_in_change_its_shape() {
    let plane = by_key("wu-tong").unwrap();
    let base = TreeSpec::typical(plane, 77);
    let open = grow(&TreeSpec { openness: 1.0, ..base }, 2);
    let stand = grow(&TreeSpec { openness: 0.0, ..base }, 2);
    assert!(open.crown_radius > 1.25 * stand.crown_radius, "{} vs {}", open.crown_radius, stand.crown_radius);
    assert!(stand.crown_base > open.crown_base, "a stand-grown tree is bare lower down");
    let street = grow(&TreeSpec { lift_m: 4.5, openness: 0.9, ..base }, 2);
    assert!(street.crown_base >= 4.5);
    let young = grow(&TreeSpec { age: 0.1, ..base }, 2);
    let old = grow(&TreeSpec { age: 1.0, ..base }, 2);
    assert!(old.trunk_radius > 1.3 * young.trunk_radius);
    let sick = grow(&TreeSpec { health: 0.1, ..base }, 1);
    let well = grow(&TreeSpec { health: 1.0, ..base }, 1);
    assert!(leaf_area(&sick) < 0.75 * leaf_area(&well));
}

#[test]
fn leaves_face_the_sky_more_than_the_ground() {
    let t = grow(&TreeSpec::typical(by_key("xiang-zhang").unwrap(), 3), 1);
    let mean_up = t.leaves.iter().map(|l| l.normal.y).sum::<f32>() / t.leaves.len() as f32;
    assert!(mean_up > 0.2, "mean leaf normal y {mean_up}");
    for l in t.leaves.iter().take(500) {
        assert!((l.dir.len() - 1.0).abs() < 1e-3 && (l.normal.len() - 1.0).abs() < 1e-3);
        assert!(l.dir.dot(l.normal).abs() < 1e-2, "normal is not perpendicular to the blade");
        assert!(l.length > 0.01 && l.length < 3.0);
    }
}

#[test]
fn a_whole_species_of_trees_is_a_population_not_a_stamp() {
    // Heights and crowns vary tree to tree around the species' mean, and silhouettes differ.
    let s = by_key("yin-xing").unwrap();
    let mut widths = Vec::new();
    for seed in 0..40u64 {
        let t = grow(&TreeSpec::typical(s, seed * 31 + 1), 2);
        let (mut lo, mut hi) = (f32::MAX, f32::MIN);
        for l in &t.leaves {
            lo = lo.min(l.pos.x);
            hi = hi.max(l.pos.x);
        }
        widths.push(hi - lo);
    }
    let mean = widths.iter().sum::<f32>() / widths.len() as f32;
    let sd = (widths.iter().map(|w| (w - mean).powi(2)).sum::<f32>() / widths.len() as f32).sqrt();
    assert!(sd / mean > 0.06, "crown widths vary by only {:.1}%", 100.0 * sd / mean);
}

/// Light decides where a broadleaf tree keeps leaves: most of the leaf area is on the lit
/// outer shell of the crown, and the shaded interior framework is nearly bare.
#[test]
fn broadleaf_leaf_area_sits_on_the_outer_shell() {
    for key in ["xiang-zhang", "yu-shu", "huai-shu", "yin-xing", "liu-shu"] {
        let species = SPECIES.iter().position(|s| s.key == key).unwrap();
        let mut outer = 0.0_f32;
        let mut inner = 0.0_f32;
        for seed in 1..=6u64 {
            let t = grow(&TreeSpec::typical(species, seed), 0);
            for l in &t.leaves {
                // Position inside the crown taken as an ellipsoid: 1 at its surface, 0 on its axis.
                let half = 0.5 * (t.height - t.crown_base);
                let (x, y, z) = (l.pos.x / t.crown_radius, (l.pos.y - t.crown_base - half) / half, l.pos.z / t.crown_radius);
                let norm = (x * x + y * y + z * z).sqrt();
                let a = 0.7 * l.length * l.length;
                if norm > 0.72 {
                    outer += a;
                } else {
                    inner += a;
                }
            }
        }
        assert!(outer > 1.1 * inner, "{key}: outer shell {outer:.1} vs interior {inner:.1}");
    }
}

/// Leaf arrangement follows the species: alternate leaves are the golden angle apart;
/// opposite leaves come in pairs 180 degrees apart, each pair a quarter turn on.
#[test]
fn leaf_arrangement_follows_the_species() {
    use std::f32::consts::{FRAC_PI_2, PI, TAU};
    let wrap = |a: f32| a.rem_euclid(TAU);
    for j in 0..8 {
        let (a, b) = (worldgen_trees::leaf_azimuth(false, j, 3, 0.4), worldgen_trees::leaf_azimuth(false, j + 1, 3, 0.4));
        let step = wrap(b - a);
        assert!((step - 137.5_f32.to_radians()).abs() < 0.01, "alternate step {step}");
        let (c, d) = (worldgen_trees::leaf_azimuth(true, 2 * j, 3, 0.4), worldgen_trees::leaf_azimuth(true, 2 * j + 1, 3, 0.4));
        assert!((wrap(d - c) - PI).abs() < 0.01, "pair not opposite");
        let e = worldgen_trees::leaf_azimuth(true, 2 * j + 2, 3, 0.4);
        assert!((wrap(e - c) - FRAC_PI_2).abs() < 0.01, "pairs not decussate");
    }
    // And the table says which species is which.
    for key in ["feng-shu", "gui-hua"] {
        assert!(by_key(key).is_some(), "{key}");
    }
}
