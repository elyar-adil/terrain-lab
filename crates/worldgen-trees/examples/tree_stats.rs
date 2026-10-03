use worldgen_trees::{SPECIES, TreeSpec, grow};

fn profile_of(key: &str) {
    let i = worldgen_trees::by_key(key).unwrap();
    let t = grow(&TreeSpec::typical(i, 5), 0);
    println!(
        "{key}: H {:.1} R {:.1} base {:.1}",
        t.height, t.crown_radius, t.crown_base
    );
    for band in 0..10 {
        let (lo, hi) = (
            t.height * band as f32 / 10.0,
            t.height * (band + 1) as f32 / 10.0,
        );
        let r: Vec<f32> = t
            .leaves
            .iter()
            .filter(|l| l.pos.y >= lo && l.pos.y < hi)
            .map(|l| (l.pos.x * l.pos.x + l.pos.z * l.pos.z).sqrt())
            .collect();
        let n = r.len();
        let m = r.iter().copied().fold(0.0, f32::max);
        println!(
            "  {:>4.1}-{:>4.1} m: {:>5} leaves, max radius {:.1}",
            lo, hi, n, m
        );
    }
}

fn main() {
    if let Some(k) = std::env::args().nth(1) {
        profile_of(&k);
        return;
    }
    println!(
        "{:<12} {:>6} {:>6} {:>7} {:>7} {:>7} {:>8} {:>8} {:>6}",
        "species", "H", "R", "leafTop", "leafR95", "leafLow", "segments", "leaves", "ms"
    );
    for (i, s) in SPECIES.iter().enumerate() {
        let spec = TreeSpec::typical(i, 77);
        let t0 = std::time::Instant::now();
        let t = grow(&spec, 0);
        let ms = t0.elapsed().as_secs_f32() * 1e3;
        let mut radii: Vec<f32> = t
            .leaves
            .iter()
            .map(|l| (l.pos.x * l.pos.x + l.pos.z * l.pos.z).sqrt())
            .collect();
        radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let r95 = radii[(radii.len() as f32 * 0.95) as usize];
        let top = t.leaves.iter().map(|l| l.pos.y).fold(0.0, f32::max);
        let low = t.leaves.iter().map(|l| l.pos.y).fold(f32::MAX, f32::min);
        println!(
            "{:<12} {:>6.1} {:>6.1} {:>7.1} {:>7.1} {:>7.1} {:>8} {:>8} {:>6.1}",
            s.key,
            t.height,
            t.crown_radius,
            top,
            r95,
            low,
            t.segments.len(),
            t.leaves.len(),
            ms
        );
    }
}
