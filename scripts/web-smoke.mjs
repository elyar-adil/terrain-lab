// Runs the browser backend (target/.../web_backend.wasm) in Node, with the same
// wire format the Web Worker uses, and prints timings and payload sizes.
//   node scripts/web-smoke.mjs [gridSize=512] [seed=284735] [--city]
import { readFileSync } from "node:fs";

const wasmPath = process.env.WASM ?? "target/wasm32-unknown-unknown/release/web_backend.wasm";
const grid = Number(process.argv[2] ?? 512);
const seed = Number(process.argv[3] ?? 284735);
let memory;
const text = (p, n) => new TextDecoder().decode(new Uint8Array(memory.buffer, p, n));
let lastStage = "";
const { instance } = await WebAssembly.instantiate(readFileSync(wasmPath), {
  env: {
    host_progress: (f, p, n) => { lastStage = `${(f * 100).toFixed(0)}% ${text(p, n)}`; },
    host_log: (p, n) => console.error("[panic]", text(p, n)),
  },
});
const x = instance.exports;
memory = x.memory;
x.init();
function call(cmd, req) {
  const input = req instanceof Uint8Array ? req : new TextEncoder().encode(JSON.stringify(req));
  const ptr = x.alloc(input.length) >>> 0;
  new Uint8Array(memory.buffer, ptr, input.length).set(input);
  const status = x.call(cmd, ptr, input.length);
  x.dealloc(ptr, input.length);
  const out = new Uint8Array(memory.buffer, x.result_ptr() >>> 0, x.result_len() >>> 0).slice();
  x.result_free();
  if (status) throw new Error(new TextDecoder().decode(out));
  return out;
}
const config = {
  seed, preset: "temperate", landform: "mountainRange", gridSize: grid, worldSizeKm: 80,
  rainfall: 1200, evaporation: 600, windSpeed: 8, windDirection: 220, sunAzimuth: 235,
  sunElevation: 42, haze: 2.5, cloudCoverage: 35, cloudSpeed: 24,
};
let t = performance.now();
const bytes = call(0, config);
const gen = performance.now() - t;
t = performance.now();
const result = JSON.parse(new TextDecoder().decode(bytes));
console.log(`generate ${grid}: ${(gen / 1000).toFixed(1)} s, json ${(bytes.length / 1e6).toFixed(1)} MB, parse ${((performance.now() - t) / 1000).toFixed(1)} s, wasm memory ${(memory.buffer.byteLength / 1e6).toFixed(0)} MB`);
console.log("stats", JSON.stringify(result.stats), "cities", result.cities?.length ?? Object.keys(result.cities).length, "summary", JSON.stringify(result.infrastructureSummary));
console.log("modernCities", result.modernCities.length ?? "?");
if (process.argv.includes("--city")) {
  t = performance.now();
  const tile = call(2, { index: 0, column: 0, row: 0 });
  console.log(`tile 0/0/0: ${(performance.now() - t).toFixed(0)} ms, ${(tile.length / 1e6).toFixed(2)} MB`);
  t = performance.now();
  for (const index of [0, 5, 12]) {
    t = performance.now();
    const city = call(1, { index });
    console.log(`city scene ${index}: ${((performance.now() - t) / 1000).toFixed(1)} s, ${(city.length / 1e6).toFixed(1)} MB, wasm memory ${(memory.buffer.byteLength / 1e6).toFixed(0)} MB`);
  }
  console.log("capacities", JSON.stringify(JSON.parse(new TextDecoder().decode(call(4, {})))));
  console.log("traffic step bytes", call(3, { city: 0, dt: 0.05 }).length);
}
