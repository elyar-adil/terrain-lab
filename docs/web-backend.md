# The browser build: the real app with a WebAssembly backend

`index.html` / `src/main.tsx` is the one frontend. On the desktop it talks to Rust
through Tauri `invoke`. In a plain browser (GitHub Pages) the same frontend, with
the same controls, talks to the same Rust code compiled to WebAssembly and running
in a Web Worker. Nothing is pre-generated and nothing is fetched from a server: the
seed, preset and grid size the user picks are computed on their machine.

This is a browser build. It is not the `cargo run` window: no native threads, no
native file dialogs, no WebKitGTK/WebView2, and a different GPU path (the browser's
WebGL instead of the webview's).

## Layout

```
crates/app-backend    plain Rust: terrain -> infrastructure -> payload, the city
                      cache (scenes and tiles), traffic, export, project files.
                      Moved out of src-tauri/src/lib.rs unchanged except that the
                      async/spawn_blocking/Tauri parts stayed behind.
src-tauri             thin Tauri commands that call app-backend (desktop unchanged)
crates/web-backend    cdylib, wasm32 only: one C-style entry point over app-backend
src/backend/          wasmHost.ts (the ABI), backendWorker.ts (the Web Worker),
                      webBackend.ts (invoke-compatible client), invoke.ts (host switch)
src/platform.ts       progress events and file pickers: Tauri or browser
src/api.ts            unchanged API for the UI; picks the host per call
vite.web.config.ts    base './', only index.html, only the two wasm files as public
scripts/build-web.mjs `npm run build:web` -> dist-web/
scripts/serve-web.mjs serves dist-web under /terrain-lab/ like GitHub Pages
scripts/web-shot.mjs  headless Chromium run of the real UI (screenshots, timings)
scripts/web-auto-shot.mjs  one screenshot at an exact view, from an automation spec (below)
scripts/web-smoke.mjs the wasm module in Node (timings, payload sizes)
```

Host selection: `window.__TAURI_INTERNALS__` present means Tauri, otherwise the wasm
worker. The frontend changes are the import lines in `App.tsx`, the `invoke` import in
`api.ts` and `city/cityLoader.ts`, the `BASE_URL` prefix on the tree-grower wasm URL,
and the web branches for export/save/load in `api.ts`.

## Feasibility: what compiles to wasm32-unknown-unknown

All of it. Checked with `cargo check --target wasm32-unknown-unknown`:
`worldgen-core`, `worldgen-contracts`, `worldgen-roads`, `worldgen-trees`, `world-core`,
`terrain-core`, `procedural`, `urban`, `infrastructure`, `city-scene` (including
`i_overlay`), `image` (png only) and `app-backend`. Tauri and `tauri-plugin-dialog` do
not, which is why the shared logic had to leave `src-tauri`.

Things that needed care:

* `std::time::Instant` panics on this target. `build_payload` used it only for
  `elapsedMs`; `app-backend::Clock` is `Instant` natively and a zero clock on wasm, and
  the worker measures the call and overwrites `elapsedMs`.
* `rayon`: compiles; with no thread support the global pool falls back to running
  inline on the calling thread (verified: terrain, infrastructure and tiles all run).
  So there is no shim and no `wasm-bindgen-rayon`. That also means no
  `SharedArrayBuffer`, so GitHub Pages' lack of COOP/COEP headers does not matter.
* `HashMap` hashing uses fixed keys on this target (no OS randomness); the generators
  do not depend on iteration order, and native `RandomState` is already random per run.
* `std::env::var` (debug switches) simply reports "unset".
* Floating point: wasm uses the compiler's libm rather than the system's, so a world is
  the same world but not bit-identical to the desktop's (seed 284735, 512: min/max
  elevation, water, snow and forest coverage agree to the printed digits, mean elevation
  1077.0917 vs 1077.0952). Use the same host if two runs must match exactly.

## No wasm-bindgen

`wasm-bindgen` CLI and `wasm-pack` are not installed and a CLI/crate version mismatch is a
classic CI failure, so the module uses a plain C ABI, like the existing tree grower
(`worldgen-trees`):

```
alloc(len) -> ptr              dealloc(ptr, len)
call(command, ptr, len) -> 0 ok | 1 error      (request: UTF-8 JSON at ptr)
result_ptr() / result_len() / result_free()    (result: JSON, or raw bytes)
init()                                          (installs the panic hook)
imports (module "env"): host_progress(f32, ptr, len), host_log(ptr, len)
```

Commands: generate, city_scene (JSON), city_scene_bin, city_tile_bin, traffic_step,
traffic_capacities, export PNG, project to/from bytes. The shapes are the ones in
`src-tauri/src/lib.rs`, because both call the same `app-backend` functions and the same
`serde` types. Pointers can exceed 2 GiB, so JS reads them with `>>> 0`. The build adds a
16 MiB stack (`crates/web-backend/build.rs`).

A trap (panic) leaves the module in an unknown state (a lock may be held), so the client
drops the worker and starts a new instance on the next call; the city cache is lost with
it and the user generates again.

## Threads and the UI

Everything runs in one dedicated worker, off the main thread: the page stays responsive
and the progress bar moves while a generation takes tens of seconds. The worker posts
`{progress, stage}` messages that `platform.listen("terrain-progress")` delivers to the
existing progress UI. City tiles are built on demand in the same worker (`city_tile_bin`),
so tile streaming is the desktop's algorithm; a tile build blocks the next request, and
the worker answers them one at a time.

## Budget (measured; one CPU thread in the worker)

| step | native, 4 cores | wasm, Node | wasm, headless Chromium |
|---|---|---|---|
| generate 512 (default) | 19 s | 31-35 s | 33-38 s reported by the app, 47-54 s wall incl. first render |
| generate 1024 | | 58 s | |
| generate 2048 | | 107 s, 762 MB wasm memory | |
| one city tile (500 m, centre) | | 7-8 s, 91 MB | streamed while flying |
| whole scene, largest city | | 27 s, 453 MB, 2.2 GB wasm heap | "metre detail loaded" about 75 s after the jump |

(The machine was shared with other builds, so wall times are pessimistic.) The grid
choices are not capped: 2048 runs in under two minutes and stays far below the 4 GiB
wasm32 address space. The default stays 512. Page load (HTML, JS, wasm) is 0.4 s locally.
Size: `web_backend.wasm` 3.7 MB (1.2 MB gzipped), whole `dist-web/` 5 MB (1.6 MB gzipped);
GitHub Pages serves gzip.

Memory is the real limit. A wasm32 heap never shrinks and tops out at 4 GiB, and the
largest city's full-detail scene needed 3.8 GB, so in the browser module (a) built scenes
and tiles are not cached (they are rebuilt identically on request; the desktop keeps them)
and (b) the whole-city scene is built at a town's budget (1600 buildings, 1400 trees;
2.2 GB heap). Streamed tiles keep the full budget. The browser tab as a whole reached
5-8 GB in the software-GL test (the frontend's own use: a desktop run of the same city
was killed at 7 GB here too), so the browser build wants a machine with 16 GB and a GPU.

## Differences from the desktop app

* One thread (no rayon parallelism); generation takes roughly 1.5 to 3 times as long.
* "Save", "Open" and "Export PNG" use browser downloads and a file picker instead of
  native dialogs (the file name from the UI becomes the download name).
* Whole-city scenes use a smaller building/tree budget than the desktop (see Budget).
* Huge cities and a 2048 grid need several GB of RAM in the tab.
* Traffic (`traffic_step`) is implemented in the module like the desktop.
* The header badge reads "RUST · WASM" instead of "RUST NATIVE".

## Build and deploy

```
npm run build:web        # cargo (wasm32) + vite -> dist-web/ (+ .nojekyll)
node scripts/serve-web.mjs 4173 /terrain-lab/
node scripts/web-shot.mjs --url http://localhost:4173/terrain-lab/ --out /tmp/shots
```

One exact view, without clicking: the browser build reads the same automation spec as
the desktop app (`docs/real-app-verification.md`) from `window.__terrainLabAutomation`,
which `scripts/web-auto-shot.mjs` sets before the page loads, and waits for its "ready"
event. Nothing changes in a normal page load, where that global does not exist.

```
node scripts/web-auto-shot.mjs --out /tmp/shots --name valley \
  --spec '{"settleFrames":3,"camera":{"pos":[20,300,60],"target":[40,1500,40],"mode":"fly"}}'
```

Set `CHROMIUM_PATH` if Playwright's own browser is not installed. Under software GL a
terrain view takes about three minutes, a city view five to ten.

`dist-web/` is deployed to the `gh-pages` branch root. Build output is never committed
to `main`.

## Phones (iOS Safari)

A phone tab is killed at roughly 1-1.5 GB. Measured in headless Chromium at 256^2 with
`?mobile=1`, the 3D scene's mount alone took the JS heap to about 1.86 GB: `buildCityMass`
(src/city/cityMass.ts) builds each city's far form from JS number arrays, about 35 MB
each, 20 cities per payload. The constrained path (`isConstrainedDevice`, platform.ts)
therefore builds a city's mass only while the camera is within 9 km of it, one at a time,
skips the per-city merged layers, caps tree instances (12k regional, 6k streamed), limits
anisotropy to 4, and moves the analysis PNGs into blob URLs. After the change the same
run peaks at about 260 MB heap. Desktop and Tauri take the unchanged path.

Capability checks (WebGL2, max texture size vs. grid, half-float render targets) run
before allocation; a failure to mount shows a message in the view. A marker in
localStorage written before the mount and cleared after three frames makes the next load
start in safe mode (no masses, no trees, no shadow map) if the tab died in between;
`?safe=0` clears it.
