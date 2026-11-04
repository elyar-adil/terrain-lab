// The browser build of the real app (index.html, unchanged UI), served from any
// sub-path (GitHub Pages serves it under /terrain-lab/) with the WebAssembly
// backend in place of the Tauri commands. Built by scripts/build-web.mjs.
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

export default defineConfig({
  plugins: [react()],
  base: "./",
  // Only the two wasm modules (staged by scripts/build-web.mjs); the repository's
  // public/ holds gigabytes of render fixtures that must never ship.
  publicDir: path.resolve(__dirname, ".web-public"),
  clearScreen: false,
  build: {
    outDir: "dist-web",
    emptyOutDir: true,
    target: "es2022",
    minify: "esbuild",
    rollupOptions: {
      input: { main: path.resolve(__dirname, "index.html") },
      output: { manualChunks: { three: ["three", "three/examples/jsm/controls/OrbitControls.js"] } },
    },
  },
  worker: { format: "es" },
});
