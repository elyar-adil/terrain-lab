# Checking the real desktop app

The only interface that counts is what `cargo run` (the Tauri app, crate
`wind-water-terrain-lab` in `src-tauri`) shows. The harness pages
(`city-harness.html`, `render-harness.html`, `lod-harness.html`, `tree-lod.html`
driven by `scripts/city-audit.mjs`) and the single-file web build (`web.html`,
`scripts/build-single.mjs`) are different programs: they render a pre-dumped scene
with `CityViewer` on a flat plane, not `Terrain3D` with tiles streamed from Rust.
Anything judged only there has not been seen in the app. This page is how to see
the app itself, here, with no display and no mouse.

## Build

```
npm run build:app                              # frontend into dist-app/, audit fixtures pruned
cargo build --release -p wind-water-terrain-lab # embeds dist-app/ (see tauri.conf.json frontendDist)
```

`npm run build` still writes `dist/` for the harnesses (it carries the 1.6 GB of
`public/` fixtures, which is why the app has its own output directory: with them
embedded the executable was 336 MB and compiling it needed a 3.3 GB metadata file;
without, 12 MB). Rebuild `dist-app/` before `cargo build` whenever `src/` changes,
the executable embeds it at compile time.

Needs `webkit2gtk-4.1` and `gtk3` development packages, `Xvfb` and ImageMagick
`import`. A release build is about 8 GB of intermediates plus the rlib/staticlib/
cdylib outputs of the app crate (about 3 GB): check `df -h /` first.

## Run and screenshot

```
scripts/app-shot --name aerial --out /tmp/shots --spec '{
  "tileBudgetMs": 3000, "settleFrames": 3,
  "camera": {"city": 0, "pos": [430, 300, 430], "target": [0, 12, 0], "mode": "fly"}}'
```

`scripts/app-shot` starts its own Xvfb on a free display, runs the app on software GL
(`LIBGL_ALWAYS_SOFTWARE=1`, llvmpipe; WebKitGTK gets WebGL2 from it, no further
flags needed beyond `WEBKIT_DISABLE_COMPOSITING_MODE=1`), waits for the ready marker,
waits `--extra-wait` seconds for the compositor, runs `import -window root`, and stops
only the app and Xvfb pids it started. Outputs in `--out`: `NAME.png`, `NAME.log`
(every event, plus the webview's console), `NAME.ready.json` (what was drawn),
`NAME.app.log` (stdout/stderr). With no `--spec` the app starts normally and the shot
is the startup screen after `--extra-wait` seconds.

Timing under software GL: generating the default 80 km world takes about 20 s, the
terrain view about 15 s more; a city view is dominated by tile building, one frame is
seconds, so `tileBudgetMs` (milliseconds of tile building per frame, default 6) should
be raised to 1000-3000 or a street view needs ten minutes.

## The automation hook (dev only)

Off unless `TERRAIN_LAB_AUTOMATION` is set. Rust side: `src-tauri/src/dev_automation.rs`
(commands `automation_spec`, `automation_event`; watchdog thread). Frontend:
`src/devAutomation.ts`, `App.tsx` (applies it), `Terrain3D.tsx` (camera, ready marker).
Without the variable the spec command answers `null`, the event command writes nothing,
and no watchdog runs, so a normal or release run is unchanged.

Environment:

| variable | meaning |
|---|---|
| `TERRAIN_LAB_AUTOMATION` | the spec: inline JSON, or the path of a JSON file |
| `TERRAIN_LAB_LOG` | file that receives one JSON line per event (generating, generated, console output, ready, error) |
| `TERRAIN_LAB_READY_FILE` | file written when the requested view has settled |
| `TERRAIN_LAB_EXIT_AFTER_SECS` | hard watchdog; the process exits with status 2 after this long |

Spec (all fields optional; schema in `src/devAutomation.ts`):

| field | meaning |
|---|---|
| `config` | overrides of the simulation config: `seed`, `landform` (`mountainRange`, `hills`, `plains`, `plateau`, `coastal`, `archipelago`), `preset`, `gridSize`, `worldSizeKm`, `sunAzimuth`, `sunElevation` (time of day), `haze`, `cloudCoverage`, ... The app then generates exactly as the button does. |
| `view` | `3d` (default), `satellite`, or `analysis` with `layer` |
| `focus` | `{option, pitchDeg, azimuthDeg, spanKm}`: the "jump to city" menu (option 0 is the world view, 1 the first city). This is the user path, including its whole-city scene request. |
| `camera` | `{city, pos, target, mode}`. With `city` the points are city-local metres `(x, height above ground, z)`, the same frame as the harness presets; without it `(x km, height above ground m, y km)` in the world. `mode`: `orbit` (orbit controls, target on the ground), `fly` or `walk` (free-roam; walk stands at 1.7 m). |
| `settleFrames` | frames to wait after the view has settled (default 6) |
| `tileBudgetMs`, `tileLoadM` | the streamer's per-frame build budget and radius (defaults 6 ms, 600 m) |
| `tileTimeoutSec` | report ready anyway after this long (default 240); `timedOut` is then true in the ready file |
| `exitAfterReadyMs` | close the app this long after ready (for runs that do not screenshot) |

"Ready" means: the camera is placed, every city tile within reach of it is mounted
(or empty, or failed), and `settleFrames` frames have been drawn since. The ready file
holds the camera in metres, each city streamer's tile states, and renderer info.

## Not the same as the harness

See `docs/real-app-vs-harness.md` for what differs, with causes.
