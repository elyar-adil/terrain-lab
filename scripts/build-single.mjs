#!/usr/bin/env node
// Builds the whole viewer into ONE self-contained html file (dist-single/index.html).
//
//   node scripts/build-single.mjs [--fixture public/render-fixture.json]
//                                 [--city public/tiny-city.json]
//                                 [--wasm public/worldgen_trees.wasm]
//                                 [--out dist-single/index.html]
//
// The fixture and city are the generated outputs of the Rust examples
// (`render_fixture`/`city_fixture` and `tiny_city`). They are stripped of data the
// viewer never reads, gzipped and embedded as base64 together with the tree grower
// wasm, so the page makes no requests at run time and works from file:// too.
// No third-party plugin: vite emits one JS and one CSS file and this script
// inlines them.
import fs from "node:fs";
import path from "node:path";
import zlib from "node:zlib";
import { build } from "vite";
import react from "@vitejs/plugin-react";

const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..");
const arg = (name, fallback) => {
  const i = process.argv.indexOf(name);
  return i >= 0 ? process.argv[i + 1] : fallback;
};
const fixturePath = path.resolve(root, arg("--fixture", "public/render-fixture.json"));
const cityPath = path.resolve(root, arg("--city", "public/tiny-city.json"));
const wasmPath = path.resolve(root, arg("--wasm", "public/worldgen_trees.wasm"));
const outFile = path.resolve(root, arg("--out", "dist-single/index.html"));

const gz = (buffer) => zlib.gzipSync(buffer, { level: 9 }).toString("base64");
const mb = (n) => (n / 1048576).toFixed(2) + " MB";

const fixture = JSON.parse(fs.readFileSync(fixturePath, "utf8"));
// Debug/analysis rasters the renderer never reads (about a quarter of the file).
delete fixture.result.analysisPreviews;
const fixtureText = JSON.stringify(fixture);
const cityText = fs.readFileSync(cityPath, "utf8");
const payload = { fixture: gz(Buffer.from(fixtureText)), city: gz(Buffer.from(cityText)) };
const wasmUrl = "data:application/wasm;base64," + fs.readFileSync(wasmPath).toString("base64");
console.log(`fixture ${mb(fixtureText.length)} -> ${mb(payload.fixture.length)}, city ${mb(cityText.length)} -> ${mb(payload.city.length)}`);

const virtualId = "virtual:single-payload";
function single() {
  return {
    name: "single-file",
    enforce: "pre",
    resolveId: (id) => (id === virtualId ? "\0" + virtualId : null),
    load: (id) => (id === "\0" + virtualId ? `export default ${JSON.stringify(payload)};` : null),
    transform(code, id) {
      const file = id.split("?")[0].replaceAll("\\", "/");
      const patch = (from, to) => {
        if (!code.includes(from)) throw new Error(`single-file patch target missing in ${file}: ${from}`);
        return code.replace(from, () => to);
      };
      // The wasm comes from the page itself instead of a fetch of /worldgen_trees.wasm.
      if (file.endsWith("src/trees/wasm.ts")) {
        code = patch('url = "/worldgen_trees.wasm"', `url = ${JSON.stringify(wasmUrl)}`);
        code = patch('new Worker(new URL("./growWorker.ts", import.meta.url), { type: "module" })', 'null as never');
        return { code, map: null };
      }
      // Trees are grown on the main thread: a module worker cannot be started from
      // a file:// page or from inlined code.
      if (file.endsWith("src/trees/index.ts")) {
        return { code: patch('new URLSearchParams(location.search).get("workers") === "0" ? null : TreePool.create()', "null"), map: null };
      }
      // There is no tile server here: the city is the one scene shipped in the page.
      if (file.endsWith("src/api.ts")) {
        return { code: patch("const response = await fetch(`/city-tiles/${index}-${column}-${row}.json`);", "throw new Error(\"city tiles are not available in the single-file build\");\n    const response = new Response();"), map: null };
      }
      return null;
    },
  };
}

const outDir = path.join(root, "dist-single", ".stage");
fs.rmSync(outDir, { recursive: true, force: true });
await build({
  root,
  configFile: false,
  logLevel: "warn",
  plugins: [single(), react()],
  publicDir: false,
  build: {
    outDir,
    emptyOutDir: true,
    target: "es2022",
    assetsInlineLimit: 100_000_000,
    cssCodeSplit: false,
    modulePreload: false,
    chunkSizeWarningLimit: 100_000,
    rollupOptions: {
      input: path.join(root, "web.html"),
      output: { inlineDynamicImports: true },
    },
  },
});

let html = fs.readFileSync(path.join(outDir, "web.html"), "utf8");
const assets = path.join(outDir, "assets");
const read = (href) => fs.readFileSync(path.join(outDir, href.replace(/^\.?\//, "")), "utf8");
// `</script` inside the bundle would end the element early.
const safeJs = (js) => js.replace(/<\/script/gi, "<\\/script");
html = html.replace(/<link rel="stylesheet"[^>]*href="([^"]+)"[^>]*>/g, (_, href) => `<style>${read(href)}</style>`);
html = html.replace(/<script type="module"[^>]*src="([^"]+)"[^>]*><\/script>/g, () => "");
const scripts = fs.readdirSync(assets).filter((f) => f.endsWith(".js"));
if (scripts.length !== 1) throw new Error(`expected one js chunk, got ${scripts.join(", ")}`);
const js = safeJs(fs.readFileSync(path.join(assets, scripts[0]), "utf8"));
html = html.replace("</body>", () => `<script type="module">${js}</script>\n</body>`);
if (/(src|href)="(?!data:|#)[^"]*\.(js|css|wasm|json)"/.test(html)) throw new Error("external reference left in html");
fs.mkdirSync(path.dirname(outFile), { recursive: true });
fs.writeFileSync(outFile, html);
fs.rmSync(outDir, { recursive: true, force: true });
console.log(`wrote ${outFile} (${mb(html.length)})`);
