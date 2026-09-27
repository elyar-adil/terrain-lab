// Visual audit driver.
//
// Screenshots a named camera preset from a headless browser and reports what the
// renderer actually drew.
//
// **It builds the app and serves `dist/` statically rather than using the vite
// dev server.** That is the whole reason this file exists in this form. The dev
// server works, but a screenshot harness that depends on it also inherits its
// HMR websocket, its dependency optimiser's forced reload, and its dev-only
// failure modes — and every one of those looks exactly like "the render is
// broken". Two full runs were lost to a dev server that served the page and then
// died, which is reported by this script as nothing at all.
//
// Two more things it does that a plain screenshot script does not:
//
//   * It waits for the renderer's own `__RENDER_READY__` flag rather than a fixed
//     delay, and the viewer logs a marker per phase, so a hang is distinguishable
//     from a slow frame instead of both reading as "no screenshot".
//   * It reads back `__CITY_DIAGNOSTICS__` — draw calls, triangles, and any
//     material key the scene asked for that the renderer does not have. A missing
//     material is the failure mode that silently produced a white city, so it is
//     reported as a number rather than left for someone to notice.
//
//   node scripts/city-audit.mjs --preset street --out street.png
//   node scripts/city-audit.mjs --all
//
import { createRequire } from "node:module";
import { spawn } from "node:child_process";
import { createServer } from "node:http";
import { readFile, mkdir, writeFile, stat } from "node:fs/promises";
import { setTimeout as sleep } from "node:timers/promises";
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
const outDir = path.resolve(root, arg("out-dir", "docs/render-validation"));
const width = Number(arg("width", "1440"));
const height = Number(arg("height", "900"));
const settle = Number(arg("settle", "1200"));
const city = arg("city", "0");
const fps = arg("fps", "3");
const port = Number(arg("port", "0")) || 4000 + Math.floor(Math.random() * 900);

// Every preset the viewer defines, so `--all` cannot silently skip one.
const PRESETS = ["street", "junction", "tower", "aerial", "skyline"];
const wanted = flag("all") ? PRESETS : [arg("preset", "street")];
// `cheap=1` drops the environment bake and the soft shadow filter. Both are large
// and both are slow on a software rasteriser, and neither changes framing,
// geometry, materials or tone — so an iteration loop that keeps them is a loop
// nobody runs. Final shots go through without it.
const cheap = flag("cheap") ? "1" : "0";
const readyTimeout = Number(arg("ready-timeout", "240")) * 1000;

const dist = path.join(root, "dist");

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".png": "image/png",
  ".svg": "image/svg+xml",
};

function run(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: root, stdio: "inherit" });
    child.on("error", reject);
    child.on("exit", (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`)),
    );
  });
}

/**
 * Serve `dist/` on one port, and only resolve once it is actually accepting
 * connections. Returning a handle the caller can close keeps the teardown in one
 * place, which is the other half of not leaking a server into the next run.
 */
async function serveDist() {
  const server = createServer(async (request, response) => {
    const url = new URL(request.url ?? "/", `http://127.0.0.1:${port}`);
    const relative = decodeURIComponent(url.pathname).replace(/^\/+/, "") || "index.html";
    const resolved = path.join(dist, relative);
    // Refuse to serve anything outside `dist`, so a crafted path cannot read the
    // repository or a user file.
    if (!resolved.startsWith(dist)) {
      response.writeHead(403).end("forbidden");
      return;
    }
    try {
      const body = await readFile(resolved);
      response.writeHead(200, {
        "content-type": MIME[path.extname(resolved)] ?? "application/octet-stream",
        "cache-control": "no-store",
      });
      response.end(body);
    } catch {
      response.writeHead(404).end("not found");
    }
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", resolve);
  });
  return {
    close: () => new Promise((resolve) => server.close(resolve)),
  };
}

await mkdir(outDir, { recursive: true });

// Rebuild only when asked, or when there is no build to serve. A rebuild is ten
// seconds; re-deciding whether one is needed every run is worse.
if (flag("build") || !(await stat(path.join(dist, "city-harness.html")).catch(() => null))) {
  console.log("building...");
  await run(process.execPath, [path.join(root, "node_modules", "vite", "bin", "vite.js"), "build"]);
}

const server = await serveDist();
const report = [];

try {
  const browser = await chromium.launch({
    channel: "msedge",
    args: [
      "--use-angle=swiftshader",
      "--enable-webgl",
      "--ignore-gpu-blocklist",
      "--hide-scrollbars",
    ],
  });

  for (const preset of wanted) {
    const page = await browser.newPage({ viewport: { width, height } });
    const problems = [];
    const trace = [];
    page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
    page.on("console", (message) => {
      const text = message.text();
      // The viewer logs a stage marker per phase when `cheap` is set. Without
      // these, a hang looks identical to a slow frame.
      if (message.type() === "log" && text.startsWith("[city]")) trace.push(text.slice(7));
      if (message.type() === "error") problems.push(`console: ${text}`);
    });

    const url =
      `http://127.0.0.1:${port}/city-harness.html` +
      `?preset=${preset}&city=${city}&fps=${fps}&cheap=${cheap}`;
    await page.goto(url, { waitUntil: "domcontentloaded" });

    let ready = true;
    try {
      await page.waitForFunction(() => window.__RENDER_READY__ === true, null, {
        timeout: readyTimeout,
      });
    } catch {
      ready = false;
      // The viewer reports a failure by rendering it into the page, not to the
      // console, so the page text is the only place the reason appears. Without
      // this, a thrown error and a hang are indistinguishable from outside.
      const text = await page
        .evaluate(() => document.body.innerText.trim().slice(0, 600))
        .catch(() => "");
      problems.push(
        `renderer never reported ready; reached: ${
          trace.length ? trace.join(" -> ") : "(nothing - the module did not run)"
        }${text ? `\n  page said: ${text}` : ""}`,
      );
    }
    await sleep(settle);

    const diagnostics = await page.evaluate(() => window.__CITY_DIAGNOSTICS__ ?? null);
    const out = path.join(outDir, `${preset}${cheap ? "-cheap" : ""}.png`);
    if (ready) {
      // Software WebGL needs a long budget: one composed frame of a city costs
      // seconds, and the render loop keeps queueing work while the screenshot
      // waits for a stable frame.
      await page.screenshot({ path: out, timeout: readyTimeout, animations: "disabled" });
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
      trace,
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
    if (trace.length) console.log(`  stages: ${trace.join(" -> ")}`);
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
  await server.close();
}
