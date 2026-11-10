// One screenshot of the browser build at an exact view, driven by the same automation
// spec as scripts/app-shot (docs/real-app-verification.md), with no clicking.
//   node scripts/web-auto-shot.mjs --url http://localhost:4173/terrain-lab/ --out DIR --name street \
//     --spec '{"tileBudgetMs":3000,"camera":{"city":0,"pos":[40,1.7,10],"target":[0,6,60],"mode":"walk"}}'
// Options: --width 1600 --height 900 --timeout 1200 (seconds) --wait 4 (seconds after ready) --query "ao=0"
import { chromium } from "playwright-core";
import { mkdirSync, writeFileSync } from "node:fs";

const arg = (name, fallback) => {
  const i = process.argv.indexOf(`--${name}`);
  return i > 0 ? process.argv[i + 1] : fallback;
};
const url = arg("url", "http://localhost:4173/terrain-lab/");
const out = arg("out", "web-shots");
const name = arg("name", "shot");
const spec = JSON.parse(arg("spec", "{}"));
const width = Number(arg("width", 1600));
const height = Number(arg("height", 900));
const timeoutMs = Number(arg("timeout", 1200)) * 1000;
const query = arg("query", "");
mkdirSync(out, { recursive: true });

const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH || undefined,
  args: ["--no-sandbox", "--use-angle=swiftshader", "--enable-webgl", "--ignore-gpu-blocklist", "--disable-gpu-watchdog", "--enable-unsafe-swiftshader"],
});
const page = await browser.newPage({ viewport: { width, height } });
page.setDefaultTimeout(timeoutMs);
const log = [];
const started = Date.now();
const note = (kind, detail) => log.push({ t: Number(((Date.now() - started) / 1000).toFixed(1)), kind, detail });
page.on("pageerror", (error) => note("pageerror", error.message));
page.on("console", (message) => { if (["error", "warning"].includes(message.type())) note(`console.${message.type()}`, message.text().slice(0, 500)); });

let ready;
const readyPromise = new Promise((resolve) => { ready = resolve; });
await page.exposeFunction("__terrainLabEvent", (event, detail) => {
  if (event !== "console") note(event, detail);
  if (event === "ready" || event === "error") ready({ event, detail });
});
await page.addInitScript((automation) => { window.__terrainLabAutomation = automation; }, spec);
await page.goto(query ? `${url}?${query}` : url, { waitUntil: "load" });
const result = await Promise.race([
  readyPromise,
  new Promise((resolve) => setTimeout(() => resolve({ event: "timeout", detail: {} }), timeoutMs)),
]);
await page.waitForTimeout(Number(arg("wait", 4)) * 1000);
// The canvas alone: the panels around it are not what is being judged.
const canvas = page.locator("canvas").first();
if (process.argv.includes("--page") || !(await canvas.count())) await page.screenshot({ path: `${out}/${name}.png`, timeout: 10 * 60_000 });
else await canvas.screenshot({ path: `${out}/${name}.png`, timeout: 10 * 60_000 });
writeFileSync(`${out}/${name}.json`, JSON.stringify({ result, log }, null, 2));
console.log(`${name}: ${result.event} after ${((Date.now() - started) / 1000).toFixed(0)} s`);
await browser.close();
