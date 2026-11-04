// Remove audit fixtures from dist-app/ before the desktop app embeds it.
//
// `public/` holds harness fixtures (audit-*.json, render-fixture.json, city-tiles/,
// city-scenes.json ...), hundreds of megabytes each. Vite copies all of public/ into
// dist/, and the desktop binary embeds the build output, so a plain `npm run build` made a
// 1.6 GB dist and a 330 MB executable. The harness pages are served from dist/ by
// scripts/city-audit.mjs, which is why this is a separate step (`npm run build:app`)
// and not part of `npm run build`. The app gets its own output directory, dist-app/
// (tauri.conf.json `frontendDist`), so the harnesses' dist/ keeps its fixtures.
import { readdir, rm, stat } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dist = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "dist-app");
let freed = 0;
for (const name of await readdir(dist)) {
  const file = path.join(dist, name);
  const info = await stat(file);
  const fixture = info.isDirectory()
    ? name === "city-tiles"
    : name.endsWith(".json") && info.size > 1_000_000;
  if (!fixture) continue;
  freed += info.isDirectory() ? 1 : info.size;
  await rm(file, { recursive: true, force: true });
  console.log(`pruned dist-app/${name}`);
}
console.log(`dist pruned of audit fixtures${freed ? "" : " (nothing to prune)"}`);
