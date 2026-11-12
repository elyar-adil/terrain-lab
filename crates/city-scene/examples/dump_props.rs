//! Writes the street-furniture and car prototypes as Wavefront OBJ, one object
//! per prototype group, for looking at the models outside a city.
//!   cargo run --release -p city-scene --example dump_props -- out.obj [key-prefix ...]
use std::fmt::Write as _;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "props.obj".to_owned());
    let prefixes: Vec<String> = args.collect();
    let mut builder = city_scene::MeshBuilder::new();
    city_scene::furniture::build_prototypes(&mut builder);
    let scene = builder.build();
    let mut out = String::new();
    let mut base = 1_usize;
    for mesh in &scene.meshes {
        if !prefixes.is_empty()
            && !prefixes
                .iter()
                .any(|p| mesh.material.starts_with(p.as_str()))
        {
            continue;
        }
        writeln!(out, "o {}", mesh.material).unwrap();
        for v in mesh.positions.chunks(3) {
            writeln!(out, "v {} {} {}", v[0], v[1], v[2]).unwrap();
        }
        for n in mesh.normals.chunks(3) {
            writeln!(out, "vn {} {} {}", n[0], n[1], n[2]).unwrap();
        }
        for t in mesh.indices.chunks(3) {
            let (a, b, c) = (
                t[0] as usize + base,
                t[1] as usize + base,
                t[2] as usize + base,
            );
            writeln!(out, "f {a}//{a} {b}//{b} {c}//{c}").unwrap();
        }
        eprintln!(
            "{}: {} vertices, {} triangles",
            mesh.material,
            mesh.positions.len() / 3,
            mesh.indices.len() / 3
        );
        base += mesh.positions.len() / 3;
    }
    std::fs::write(&path, out).expect("write obj");
}
