// Renders every component from several angles, in parallel, with the software
// rasteriser, and writes one PNG per (component, angle).
//
//   node scripts/render-matrix.mjs                    # everything
//   node scripts/render-matrix.mjs --only trees       # one group
//   node scripts/render-matrix.mjs --jobs 3 --out render-out
//
// The shot list is data (`docs/render-validation/matrix.json`): adding a component
// or an angle is one line there. Each shot drives `city-audit.mjs`, which builds
// nothing itself beyond `dist/` and reports missing materials and short UV buffers.
//
// Needs `public/city-lite.json` (cargo run --release -p city-scene --example
// dump_scene -- --lite --out public/city-lite.json), `public/worldgen_trees.wasm`
// (npm run build:wasm) and a browser (CHROMIUM_PATH or BROWSER_CHANNEL).

import { spawn } from "node:child_process";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const argv = process.argv.slice(2);
const arg = (name, fallback) => {
  const i = argv.indexOf(`--${name}`);
  return i >= 0 && argv[i + 1] ? argv[i + 1] : fallback;
};

const only = arg("only", "");
const jobs = Number(arg("jobs", "3"));
const outDir = path.resolve(root, arg("out", "render-out"));
const readyTimeout = arg("ready-timeout", "420");
// Software rendering is pixel-bound; a smaller frame is the cheapest speed-up.
const width = arg("width", "1440");
const height = arg("height", "900");
const matrix = JSON.parse(
  await readFile(path.join(root, "docs", "render-validation", "matrix.json"), "utf8"),
);

const shots = matrix.shots.filter((shot) => !only || shot.group === only);
await mkdir(outDir, { recursive: true });

function runShot(shot, port) {
  const dir = path.join(outDir, shot.group);
  const args = [
    path.join("scripts", "city-audit.mjs"),
    "--port", String(port),
    "--out-dir", dir,
    "--page", shot.page,
    "--preset", shot.name,
    "--tag", shot.tag ?? "",
    "--ready-timeout", readyTimeout,
    "--width", width,
    "--height", height,
  ];
  if (shot.cam) args.push("--cam", shot.cam);
  if (shot.query) args.push("--query", shot.query);
  if (shot.scene) args.push("--scene", shot.scene);
  if (shot.cheap !== false) args.push("--cheap");
  return new Promise((resolve) => {
    const child = spawn(process.execPath, args, { cwd: root, stdio: ["ignore", "pipe", "pipe"] });
    let log = "";
    child.stdout.on("data", (d) => (log += d));
    child.stderr.on("data", (d) => (log += d));
    child.on("exit", (code) => resolve({ shot, code, log }));
  });
}

const results = [];
let next = 0;
async function worker(id) {
  while (next < shots.length) {
    const shot = shots[next++];
    const started = Date.now();
    const result = await runShot(shot, 4300 + id * 7 + (next % 7));
    const seconds = Math.round((Date.now() - started) / 1000);
    const problems = result.log.split("\n").filter((line) => line.includes("!!") && !line.includes("render-root child"));
    console.log(
      `${result.code === 0 ? "ok " : "ERR"} ${shot.group}/${shot.name}${shot.tag ? `-${shot.tag}` : ""} ${seconds}s` +
        (problems.length ? `\n${problems.map((p) => `    ${p.trim()}`).join("\n")}` : ""),
    );
    results.push({ ...shot, code: result.code, seconds, problems });
  }
}
await Promise.all(Array.from({ length: jobs }, (_, id) => worker(id)));
await writeFile(path.join(outDir, "matrix-report.json"), `${JSON.stringify(results, null, 2)}\n`);
const failed = results.filter((r) => r.code !== 0 || r.problems.length);
console.log(`\n${results.length - failed.length}/${results.length} clean`);
process.exit(failed.length ? 1 : 0);
