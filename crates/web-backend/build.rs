fn main() {
    // The generators recurse and keep sizeable locals; the default 1 MiB wasm stack
    // is too small for the larger worlds.
    println!("cargo:rustc-link-arg-cdylib=-zstack-size=16777216");
}
