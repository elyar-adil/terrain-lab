fn main() {
    for species in procedural::STANDARD_SPECIES {
        let tree = procedural::build_prototype(species, procedural::Lod::Near, 42);
        println!("{species:?}: h={:.1} crown={:.1} segs={} foliage={}", tree.height_metres, tree.crown_radius_metres, tree.segments.len()/8, tree.foliage.len()/5);
    }
}
