// Puts the freshly built tree grower where the page loads it from.
import { copyFileSync, mkdirSync } from "node:fs";
mkdirSync("public", { recursive: true });
copyFileSync("target/wasm32-unknown-unknown/release/worldgen_trees.wasm", "public/worldgen_trees.wasm");
console.log("public/worldgen_trees.wasm");
