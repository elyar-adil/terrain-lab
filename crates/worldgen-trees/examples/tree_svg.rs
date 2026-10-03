//! A row of trees of one species (or of every species), seen from the side, for looking at.
//!
//!   cargo run -p worldgen-trees --example tree_svg -- OUT.svg [species-key|all] [count] [lod] [season]

use std::fmt::Write as _;

use worldgen_trees::{SPECIES, TreeSpec, by_key, grow};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "trees.svg".into());
    let which = args.get(2).cloned().unwrap_or_else(|| "xiang-zhang".into());
    let count: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
    let lod: u8 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let season: f32 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0.5);
    let species: Vec<usize> = if which == "all" {
        (0..SPECIES.len()).collect()
    } else {
        which
            .split(',')
            .map(|k| by_key(k).expect("species key"))
            .collect()
    };
    let k = 13.0_f32; // pixels per metre
    let (cell_w, row_h) = (11.0 * k * 1.6, 28.0 * k);
    let width = cell_w * count as f32;
    let height = row_h * species.len() as f32;
    let mut svg = String::new();
    writeln!(svg, r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}"><rect width="{width}" height="{height}" fill="#cfe0ee"/>"##).unwrap();
    for (r, &sp) in species.iter().enumerate() {
        let ground = row_h * (r as f32 + 1.0) - 14.0;
        writeln!(
            svg,
            r##"<rect x="0" y="{ground}" width="{width}" height="8" fill="#8a7a5a"/>"##
        )
        .unwrap();
        for c in 0..count {
            let spec = TreeSpec {
                season,
                ..TreeSpec::typical(sp, 1000 + c as u64 * 7919)
            };
            let tree = grow(&spec, lod);
            let cx = cell_w * (c as f32 + 0.5);
            let s = &SPECIES[sp];
            let tx = |x: f32| cx + x * k;
            let ty = |y: f32| ground - y * k;
            for seg in &tree.segments {
                let w = ((seg.ra + seg.rb) * k).max(0.6);
                writeln!(
                    svg,
                    r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#4a3a2a" stroke-width="{:.2}" stroke-linecap="round"/>"##,
                    tx(seg.a.x), ty(seg.a.y), tx(seg.b.x), ty(seg.b.y), w
                )
                .unwrap();
            }
            let fol = [
                s.foliage[0].powf(0.45),
                s.foliage[1].powf(0.45),
                s.foliage[2].powf(0.45),
            ];
            let aut = s
                .autumn
                .map(|a| [a[0].powf(0.45), a[1].powf(0.45), a[2].powf(0.45)])
                .unwrap_or(fol);
            for leaf in &tree.leaves {
                let a = tree.autumn;
                let col: Vec<u8> = (0..3)
                    .map(|i| {
                        ((fol[i] * (1.0 - a) + aut[i] * a)
                            * (0.7 + 0.5 * leaf.tint[1] as f32 / 255.0)
                            * 255.0)
                            .clamp(0.0, 255.0) as u8
                    })
                    .collect();
                let ang = (-leaf.dir.y).atan2(leaf.dir.x).to_degrees();
                let len = leaf.length * k;
                let wid = (len * s.leaf_aspect * 0.6).max(0.5);
                let c = (
                    leaf.pos.x + leaf.dir.x * leaf.length * 0.5,
                    leaf.pos.y + leaf.dir.y * leaf.length * 0.5,
                );
                writeln!(
                    svg,
                    r##"<ellipse cx="{:.1}" cy="{:.1}" rx="{:.2}" ry="{:.2}" transform="rotate({:.0} {:.1} {:.1})" fill="rgb({},{},{})" fill-opacity="0.85"/>"##,
                    tx(c.0), ty(c.1), (len * 0.5).max(0.8), wid.max(0.6) * 0.5 + 0.3, ang, tx(c.0), ty(c.1), col[0], col[1], col[2]
                )
                .unwrap();
            }
            if c == 0 {
                writeln!(
                    svg,
                    r##"<text x="8" y="{}" font-size="20" fill="#222">{} {}</text>"##,
                    ground - 8.0,
                    s.name_zh,
                    s.key
                )
                .unwrap();
            }
        }
    }
    svg.push_str("</svg>\n");
    std::fs::write(&out, svg).unwrap();
}
