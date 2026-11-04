// LOD continuity check for the far form of buildings.
//
// Serves a build of the app (`--dist`), opens `lod-harness.html`, which
// photographs a few buildings from 30 m to 4 km in steps of x1.2, and judges
// the numbers it measures on each picture:
//
//   pop        largest step in the building's mean linear colour between two
//              neighbouring distances (relative, worst channel), among pictures
//              where it is at least MIN_HEIGHT_PX tall. A switch of material,
//              palette or LOD shows as a step; a smooth fade does not.
//   collapse   smallest ratio of local contrast between two neighbouring
//              distances. Local contrast is how far a pixel sits from the mean
//              of its four neighbours, over the building's interior pixels, per
//              unit of mean luminance: whole faces in light or shade cancel,
//              floors, bays and windows do not. A mip that averages the windows
//              away shows as this falling off a cliff while the building still
//              has pixels to show floors in.
//   roof dark  share of roof pixels, seen from straight above, under 0.55 of the roof's
//              own median luminance (limit 0.12), and the mean roof luminance (at least
//              0.08): the check that no roof carries a black slab.
//   lc@N       the local contrast where the building is N pixels tall: how much
//              of the facade structure is still on the picture.
//
//   node scripts/lod-continuity.mjs --dist D --out-dir OUT [--tag after]
//        [--max-pop 0.12] [--min-collapse 0.6] [--ids 1,2,3]
//
// Exits 1 when a building fails a limit. Writes OUT/lod-<tag>.json and the
// contact sheet OUT/lod-<tag>-sheet.png (rows: buildings; columns: 50, 150,
// 300, 800 and 2000 m).

import { createServer } from "node:http";
import { readFile, mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

import { chromium } from "playwright-core";

const argv = process.argv.slice(2);
const arg = (name, fallback) => {
  const index = argv.indexOf(`--${name}`);
  return index >= 0 && argv[index + 1] ? argv[index + 1] : fallback;
};
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const dist = path.resolve(root, arg("dist", "dist"));
const outDir = path.resolve(root, arg("out-dir", "docs/render-validation"));
const tag = arg("tag", "run");
const maxPop = Number(arg("max-pop", "0.12"));
const minCollapse = Number(arg("min-collapse", "0.6"));
const ids = arg("ids", "");
const MIN_HEIGHT_PX = 14;

const MIME = { ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".wasm": "application/wasm" };
const server = createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", "http://127.0.0.1");
  const file = path.join(dist, decodeURIComponent(url.pathname).replace(/^\/+/, "") || "index.html");
  if (!file.startsWith(dist)) return response.writeHead(403).end();
  let body;
  try {
    body = await readFile(file);
  } catch {
    return response.writeHead(404).end();
  }
  response.writeHead(200, { "content-type": MIME[path.extname(file)] ?? "application/octet-stream" });
  response.end(body);
});
await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const port = server.address().port;

await mkdir(outDir, { recursive: true });
const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH || undefined,
  args: ["--no-sandbox", "--use-angle=swiftshader", "--enable-webgl", "--ignore-gpu-blocklist", "--disable-gpu-watchdog"],
});
const page = await browser.newPage({ viewport: { width: 400, height: 400 } });
const problems = [];
page.on("pageerror", (e) => problems.push(`pageerror: ${e.message}`));
page.on("console", (m) => { if (m.type() === "error") problems.push(`console: ${m.text()}`); });
await page.goto(`http://127.0.0.1:${port}/lod-harness.html${ids ? `?ids=${ids}` : ""}`);
await page.waitForFunction(() => window.__LOD_DONE__ === true, null, { timeout: 500000 });
const stats = await page.evaluate(() => window.__LOD_STATS__);
const sheet = await page.evaluate(() => window.__LOD_SHEET__);
await browser.close();
server.close();
if (stats.error) {
  console.error("harness failed:", stats.error, problems.slice(0, 5));
  process.exit(2);
}
await writeFile(path.join(outDir, `lod-${tag}.json`), JSON.stringify(stats, null, 1));
if (sheet) await writeFile(path.join(outDir, `lod-${tag}-sheet.png`), Buffer.from(sheet.split(",")[1], "base64"));

const worst = (v) => Math.max(...v);
let failed = false;
const rows = [];
for (const row of stats.rows) {
  // Pictures where the building is cropped by the frame show a different part of it,
  // which is not an LOD effect, so they are left out.
  const steps = row.steps.filter((s) => !s.cropped && s.heightPx >= MIN_HEIGHT_PX && s.px > 60 && s.lcPx > 20);
  let pop = 0, popAt = 0, collapse = 1, collapseAt = 0;
  for (let i = 1; i < steps.length; i += 1) {
    const a = steps[i - 1], b = steps[i];
    const rel = worst(a.mean.map((v, c) => Math.abs(b.mean[c] - v) / Math.max(v, 1e-3)));
    if (rel > pop) { pop = rel; popAt = b.d; }
    const ratio = b.lc / Math.max(a.lc, 1e-6);
    // Only a loss of spread counts: the spread may rise as a building grows.
    if (ratio < collapse) { collapse = ratio; collapseAt = b.d; }
  }
  const at = (px) => steps.reduce((best, s) => (Math.abs(s.heightPx - px) < Math.abs(best.heightPx - px) ? s : best), steps[0]);
  const near = steps[0], far = steps[steps.length - 1];
  const drift = near && far ? worst(near.mean.map((v, c) => Math.abs(far.mean[c] - v) / Math.max(v, 1e-3))) : 0;
  // A roof seen from straight above must read as a roof: not mostly black.
  const roofOk = row.roof.darkShare <= 0.12 && row.roof.mean >= 0.08;
  const pass = pop <= maxPop && collapse >= minCollapse && roofOk;
  if (!pass) failed = true;
  rows.push({
    building: `${row.label} #${row.id} (${row.floors} fl, ${row.heightM.toFixed(0)} m${row.farLook ? "" : ", plan look"})`,
    pop: pop.toFixed(3), popAt, collapse: collapse.toFixed(2), collapseAt,
    "lc@200px": at(200).lc.toFixed(3), "lc@100px": at(100).lc.toFixed(3), "lc@50px": at(50).lc.toFixed(3), "lc@25px": at(25).lc.toFixed(3),
    drift: drift.toFixed(3), "roof dark": row.roof.darkShare.toFixed(3), "roof mean": row.roof.mean.toFixed(3), pass: pass ? "ok" : "FAIL",
  });
}
console.table(rows);
console.log(`limits: pop <= ${maxPop}, collapse >= ${minCollapse}; whole-building pictures ${MIN_HEIGHT_PX}+ px tall; ladder ${stats.ladder[0]}..${stats.ladder.at(-1)} m`);
if (problems.length) console.log("page problems:", problems.slice(0, 5));
process.exit(failed ? 1 : 0);
