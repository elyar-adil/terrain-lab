// One-command visual verification of the city pipeline.
//
//   node scripts/verify-city.mjs            # full: fixture + all presets + gallery
//   node scripts/verify-city.mjs --quick    # cheap-mode presets, one gallery row
//
// Regenerates public/city-scenes.json from the current Rust code, then drives
// the audit script across every camera preset and every gallery row. This is
// the gate a change must pass before the city counts as improved: geometry
// compiles, payload loads, every component and every framing renders.

import { spawnSync } from "node:child_process";
import path from "node:path";
import process from "node:process";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname).slice(1), "..");
const quick = process.argv.includes("--quick");

console.log("[1/3] regenerating public/city-scenes.json ...");
const gen = spawnSync(
  "cargo",
  ["run", "--release", "-p", "wind-water-terrain-lab", "--example", "tiny_city", "--", "--out", "city-scenes.json"],
  { cwd: root, stdio: "inherit", shell: true },
);
if (gen.status !== 0) {
  console.error("fixture generation failed");
  process.exit(1);
}

const auditArgs = (extra) => [
  path.join("scripts", "city-audit.mjs"),
  "--port", "4182",
  "--out-dir", path.join("docs", "render-validation", "current"),
  ...extra,
];

console.log("[2/3] city presets ...");
for (const preset of ["street", "junction", "tower", "aerial", "skyline"]) {
  const args = auditArgs(["--preset", preset, "--page", "city-harness.html"]);
  if (quick) args.push("--cheap");
  const run = spawnSync("node", args, { cwd: root, stdio: "inherit", shell: true });
  if (run.status !== 0) console.error(`preset ${preset} exited ${run.status}`);
}

console.log("[3/3] component gallery ...");
for (const row of quick ? ["facade"] : ["facade", "ground", "prototypes"]) {
  const args = auditArgs(["--preset", row, "--page", "gallery.html"]);
  if (quick) args.push("--cheap");
  const run = spawnSync("node", args, { cwd: root, stdio: "inherit", shell: true });
  if (run.status !== 0) console.error(`gallery row ${row} exited ${run.status}`);
}

console.log(`done — screenshots in docs/render-validation/current/`);
