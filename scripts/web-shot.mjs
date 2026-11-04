// Drives the browser build of the real app in headless Chromium: loads the page from
// a sub-path, clicks the real generate button, looks at 3D, satellite, analysis and a
// city, and saves a screenshot of each plus the timings and console problems.
//   node scripts/web-shot.mjs --url http://localhost:4173/terrain-lab/ --out DIR [--grid 512] [--seed N] [--city 1]
import { chromium } from "playwright-core";
import { mkdirSync, writeFileSync } from "node:fs";

const arg = (name, fallback) => {
  const i = process.argv.indexOf(`--${name}`);
  return i > 0 ? process.argv[i + 1] : fallback;
};
const url = arg("url", "http://localhost:4173/terrain-lab/");
const out = arg("out", "web-shots");
const grid = arg("grid", null);
const seed = arg("seed", null);
const cityOption = Number(arg("city", 1));
mkdirSync(out, { recursive: true });

const problems = [];
const timings = {};
const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH || undefined,
  args: ["--no-sandbox", "--use-angle=swiftshader", "--enable-webgl", "--ignore-gpu-blocklist", "--disable-gpu-watchdog", "--enable-unsafe-swiftshader"],
});
// Software GL renders a frame of the terrain in seconds, and a screenshot waits for one.
const shot = (page, path) => page.screenshot({ path, timeout: 5 * 60_000 });
const page = await browser.newPage({ viewport: { width: 1600, height: 900 } });
page.setDefaultTimeout(5 * 60_000);
page.on("console", (m) => { if (["error", "warning"].includes(m.type())) problems.push(`console.${m.type()}: ${m.text()}`); });
page.on("pageerror", (e) => problems.push(`pageerror: ${e.message}`));
page.on("requestfailed", (r) => problems.push(`requestfailed: ${r.url()} ${r.failure()?.errorText}`));
page.on("response", (r) => { if (r.status() >= 400) problems.push(`http ${r.status()}: ${r.url()}`); });

// Frames take seconds under software GL, so Playwright'"'"'s "stable" checks never settle: click in the page.
const clickText = (text) =>
  page.evaluate((label) => [...document.querySelectorAll("button")].find((b) => b.textContent.trim() === label).click(), text);
const t0 = Date.now();
await page.goto(url, { waitUntil: "load" });
await page.waitForSelector("button.primary");
timings.pageLoadMs = Date.now() - t0;
await shot(page, `${out}/0-start.png`);

if (grid) await page.selectOption(".select-control select", grid);
if (seed) {
  const input = page.locator('input[type="number"]').first();
  if (await input.count()) await input.fill(seed);
}
const g0 = Date.now();
await page.click("button.primary");
await page.waitForTimeout(3000);
await shot(page, `${out}/1-generating.png`);
await page.waitForSelector(".view-switcher", { timeout: 15 * 60_000 });
timings.generateMs = Date.now() - g0;
console.error("generated in", timings.generateMs, "ms");
await page.waitForTimeout(15_000);
timings.statusAfter = await page.locator(".progress-overlay, .status, .topbar").first().innerText().catch(() => "");
await shot(page, `${out}/2-terrain3d.png`);

await clickText("动态卫星");
await page.waitForTimeout(4000);
await shot(page, `${out}/3-satellite.png`);
await clickText("3D 地形");
await page.waitForTimeout(3000);

const c0 = Date.now();
await page.evaluate((value) => {
  const select = document.querySelector("select.city-jump");
  select.value = value;
  select.dispatchEvent(new Event("change", { bubbles: true }));
}, String(cityOption));
await page.waitForTimeout(5000);
await shot(page, `${out}/4-city-jump.png`);
try {
  await page.waitForFunction(() => /米制细节已载入/.test(document.querySelector(".viewport-meta")?.textContent ?? ""), null, { timeout: 8 * 60_000 });
  timings.cityDetailMs = Date.now() - c0;
} catch { timings.cityDetailMs = "timeout"; }
await page.waitForTimeout(20_000);
await shot(page, `${out}/5-city.png`);

if (process.argv.includes("--zoom")) {
  // Wheel in on the city so its tiles are requested from the backend, then look.
  await page.mouse.move(850, 440);
  for (let i = 0; i < 14; i += 1) {
    await page.mouse.wheel(0, -400);
    await page.waitForTimeout(400);
  }
  const z0 = Date.now();
  await page.waitForTimeout(90_000);
  timings.zoomWaitMs = Date.now() - z0;
  await shot(page, `${out}/6-city-zoom.png`);
}

console.log(JSON.stringify({ timings, problems: [...new Set(problems)].slice(0, 30) }, null, 2));
writeFileSync(`${out}/report.json`, JSON.stringify({ timings, problems }, null, 2));
await browser.close();
