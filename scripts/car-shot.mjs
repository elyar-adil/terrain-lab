// Screenshots of the generated cars, alone, from the standard views.
//
//   cargo run --release -p city-scene --example dump_cars -- public/audit-cars.json
//   npx vite --port 1420 &
//   node scripts/car-shot.mjs --out render-out/cars --variants 0,2 --views front34,headlamp
//
// With no --variants / --views, every design from every view.
import { chromium } from "playwright-core";
import { mkdirSync } from "node:fs";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const at = args.indexOf(`--${name}`);
  return at >= 0 ? args[at + 1] : fallback;
};
const out = option("out", "render-out/cars");
const base = option("url", "http://localhost:1420/car-harness.html");
const variants = option("variants", "0,1,2,3,4,5").split(",");
const views = option(
  "views",
  "front34,rear34,side,front,rear,headlamp,taillamp,wheel,mirror",
).split(",");
const [width, height] = option("size", "1280x720").split("x").map(Number);
const extra = option("query", "");
mkdirSync(out, { recursive: true });

const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH || undefined,
  args: ["--no-sandbox", "--use-angle=swiftshader", "--enable-webgl", "--ignore-gpu-blocklist", "--disable-gpu-watchdog", "--enable-unsafe-swiftshader"],
});
const page = await browser.newPage({ viewport: { width, height } });
page.on("console", (message) => {
  if (message.type() === "error") console.error(message.text());
});
for (const variant of variants) {
  for (const view of views) {
    await page.goto(`${base}?variant=${variant}&view=${view}${extra ? `&${extra}` : ""}`);
    await page.waitForFunction(() => window.__carReady, null, { timeout: 120000 });
    const path = `${out}/car${variant}-${view}.png`;
    await page.screenshot({ path });
    console.log(path);
  }
}
await browser.close();
