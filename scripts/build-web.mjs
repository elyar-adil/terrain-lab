// Builds the browser version of the real app into dist-web/ (npm run build:web).
//
//   1. compile the Rust backend (crates/web-backend) and the tree grower to wasm32
//   2. stage both modules in .web-public/ (vite copies it verbatim into the build)
//   3. type-check and bundle the unchanged frontend with vite.web.config.ts
//   4. write .nojekyll so GitHub Pages serves the hashed asset names untouched
//
// Flags: --skip-wasm reuses the modules already built in target/.
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync, statSync } from "node:fs";
import { gzipSync } from "node:zlib";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const run = (command, args, env = {}) =>
  execFileSync(command, args, { stdio: "inherit", env: { ...process.env, ...env } });
const npx = process.platform === "win32" ? "npx.cmd" : "npx";
const target = "target/wasm32-unknown-unknown/release";

if (!process.argv.includes("--skip-wasm")) {
  run("cargo", [
    "build", "--release", "--target", "wasm32-unknown-unknown",
    "-p", "web-backend", "-p", "worldgen-trees",
  ]);
}

rmSync(".web-public", { recursive: true, force: true });
mkdirSync(".web-public", { recursive: true });
for (const name of ["web_backend.wasm", "worldgen_trees.wasm"]) {
  if (!existsSync(`${target}/${name}`)) throw new Error(`${target}/${name} is missing; run without --skip-wasm`);
  copyFileSync(`${target}/${name}`, `.web-public/${name}`);
}

run(npx, ["tsc", "--noEmit"]);
run(npx, ["vite", "build", "--config", "vite.web.config.ts"]);
writeFileSync("dist-web/.nojekyll", "");

let raw = 0;
let gz = 0;
const walk = (dir) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) walk(path);
    else {
      raw += statSync(path).size;
      gz += gzipSync(readFileSync(path)).length;
    }
  }
};
walk("dist-web");
console.log(`dist-web: ${(raw / 1e6).toFixed(1)} MB raw, ${(gz / 1e6).toFixed(1)} MB gzipped`);
