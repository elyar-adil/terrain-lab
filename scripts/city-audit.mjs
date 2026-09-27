// Visual audit driver.
//
// Screenshots a named camera preset from a headless browser and reports what the
// renderer actually drew. Two things it does that a plain screenshot script does
// not, and both because the naive version produced blank frames:
//
//   * It waits for the renderer's own `__RENDER_READY__` flag rather than a fixed
//     delay. Under SwiftShader the software rasteriser a headless browser falls
//     back to, a scene is *populated* seconds before a frame is *composed*.
//   * It reads back `__CITY_DIAGNOSTICS__` — draw calls, triangles, and any
//     material key the scene asked for that the renderer does not have. A missing
//     material is the failure mode that silently produces a white city, so it is
//     reported as a number rather than left for someone to notice.
//
//   node scripts/city-audit.mjs --preset street --out street.png
//   node scripts/city-audit.mjs --all --out-dir docs/render-validation
//
import { createRequire } from "node:module";
import { spawn } from "node:child_process";
import { setTimeout as sleep } from "node:timers/promises";
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";

// The reference project has Playwright installed but no browsers downloaded, so
// the system Edge is used via its channel.
const require = createRequire(
  "C:/Users/Elyar/Desktop/intersection-generator/package.json",
);
const { chromium } = require("playwright");

const argv = process.argv.slice(2);
const arg = (name, fallback) => {
  const index = argv.indexOf(`--${name}`);
  return index >= 0 && argv[index + 1] ? argv[index + 1] : fallback;
};
const flag = (name) => argv.includes(`--${name}`);

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname).slice(1), "..");
const port = Number(arg("port", "4182"));
const outDir = path.resolve(root, arg("out-dir", "docs/render-validation"));
const width = Number(arg("width", "1440"));
const height = Number(arg("height", "900"));
const settle = Number(arg("settle", "1500"));
const city = arg("city", "0");
const fps = arg("fps", "3");
const page_ = arg("page", "city-harness.html");

// Every preset the viewer defines, so `--all` cannot silently skip one.
const PRESETS = ["street", "junction", "tower", "aerial", "skyline"];
const wanted = flag("all") ? PRESETS : [arg("preset", "street")];
// `cheap=1` drops the environment bake and the soft shadow filter. Both are
// large, both are slow on a software rasteriser, and neither changes framing,
// geometry, materials or tone — so an iteration loop that keeps them is a loop
// nobody runs. Final shots go through without it.
const cheap = flag("cheap") ? "1" : "0";
const readyTimeout = Number(arg("ready-timeout", "180")) * 1000;

const server = spawn(
  process.execPath,
  [
    path.join(root, "node_modules", "vite", "bin", "vite.js"),
    "--host", "127.0.0.1",
    "--port", String(port),
    "--strictPort",
  ],
  { cwd: root, stdio: "ignore", windowsHide: true },
);

const shutdown = () => {
  if (!server.killed) server.kill();
};
process.on("exit", shutdown);
process.on("SIGINT", () => {
  shutdown();
  process.exit(1);
});

await mkdir(outDir, { recursive: true });

try {
  let started = false;
  for (let attempt = 0; attempt < 120; attempt += 1) {
    try {
      const response = await fetch(`http://127.0.0.1:${port}/${page_}`);
      if (response.ok) {
        started = true;
        break;
      }
    } catch {
      /* vite is still binding the port */
    }
    await sleep(250);
  }
  if (!started) throw new Error("vite did not start");

  const browser = await chromium.launch({
    channel: "msedge",
    args: [
      "--use-angle=swiftshader",
      "--enable-webgl",
      "--ignore-gpu-blocklist",
      "--hide-scrollbars",
    ],
  });

  const report = [];
  for (const preset of wanted) {
    const page = await browser.newPage({ viewport: { width, height } });
    const problems = [];
    page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
    page.on("console", (message) => {
      if (message.type() === "error") problems.push(`console: ${message.text()}`);
    });
    // A 404 is reported without its URL by the console message, which makes it
    // useless: the whole failure of a harness run is usually one missing asset.
    page.on("response", (response) => {
      if (response.status() >= 400) {
        problems.push(`http ${response.status()}: ${response.url()}`);
      }
    });

    const url =
      `http://127.0.0.1:${port}/${page_}` +
      `?preset=${preset}&city=${city}&fps=${fps}`;
    await page.goto(url, { waitUntil: "domcontentloaded" });

    let ready = true;
    try {
      await page.waitForFunction(() => window.__RENDER_READY__ === true, null, {
        timeout: 300_000,
      });
    } catch {
      ready = false;
      problems.push("renderer never reported ready");
    }
    await sleep(settle);

    const diagnostics = await page.evaluate(() => window.__CITY_DIAGNOSTICS__ ?? null);
    const out = path.join(outDir, `${preset}.png`);
    if (ready) {
      // Software WebGL needs a long budget: one composed frame of a city costs
      // seconds, and the render loop keeps queueing work while the screenshot
      // waits for a stable frame.
      await page.screenshot({ path: out, timeout: 300_000, animations: "disabled" });
    }
    await page.close();

    const line = {
      preset,
      out: ready ? path.relative(root, out) : null,
      draws: diagnostics?.draws ?? null,
      triangles: diagnostics?.triangles ?? null,
      geometries: diagnostics?.geometries ?? null,
      textures: diagnostics?.textures ?? null,
      vehicles: diagnostics?.vehicles ?? null,
      signals: diagnostics?.signals ?? null,
      lamps: diagnostics?.lamps ?? null,
      missingMaterials: diagnostics?.missingMaterials ?? null,
      problems,
    };
    report.push(line);

    const missing = line.missingMaterials ?? [];
    console.log(
      `${preset.padEnd(9)} ${String(line.draws ?? "?").padStart(4)} draws ` +
        `${String(line.triangles ?? "?").padStart(9)} tris  ` +
        `${String(line.vehicles ?? "?").padStart(3)} vehicles  ` +
        `${ready ? path.relative(root, out) : "NO SCREENSHOT"}`,
    );
    if (missing.length) {
      console.log(`  !! renderer has no material for: ${missing.join(", ")}`);
    }
    for (const problem of problems) console.log(`  !! ${problem}`);
  }

  await browser.close();
  await writeFile(
    path.join(outDir, "audit.json"),
    `${JSON.stringify(report, null, 2)}\n`,
    "utf8",
  );
  console.log(`\nreport: ${path.relative(root, path.join(outDir, "audit.json"))}`);
} finally {
  shutdown();
}
