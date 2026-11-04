// Serves dist-web/ under a sub-path, the way GitHub Pages serves a project site.
//   node scripts/serve-web.mjs [port=4173] [prefix=/terrain-lab/]
import { createServer } from "node:http";
import { createReadStream, existsSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";

const port = Number(process.argv[2] ?? 4173);
const prefix = process.argv[3] ?? "/terrain-lab/";
const root = process.argv[4] ?? "dist-web";
const types = { ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".wasm": "application/wasm", ".json": "application/json", ".png": "image/png" };

createServer((request, response) => {
  const url = new URL(request.url, "http://localhost");
  if (!url.pathname.startsWith(prefix)) {
    response.writeHead(404).end("not under " + prefix);
    return;
  }
  let path = normalize(join(root, decodeURIComponent(url.pathname.slice(prefix.length))));
  if (path.endsWith("/") || (existsSync(path) && statSync(path).isDirectory())) path = join(path, "index.html");
  if (!path.startsWith(root) || !existsSync(path)) {
    response.writeHead(404).end("not found");
    return;
  }
  response.writeHead(200, { "content-type": types[extname(path)] ?? "application/octet-stream" });
  createReadStream(path).pipe(response);
}).listen(port, () => console.log(`http://localhost:${port}${prefix}`));
