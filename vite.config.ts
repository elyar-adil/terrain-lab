import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: process.env.TAURI_ENV_DEBUG ? false : "esbuild",
    sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
    rollupOptions: {
      // Both harnesses are real entry points, not dev-only pages. The visual
      // audit builds the app and serves `dist/` statically rather than using the
      // dev server, because a screenshot harness that depends on a dev server
      // also depends on its HMR websocket, its dep optimiser's reload, and its
      // failure modes — none of which have anything to do with the render.
      input: {
        main: path.resolve(__dirname, "index.html"),
        "render-harness": path.resolve(__dirname, "render-harness.html"),
        "city-harness": path.resolve(__dirname, "city-harness.html"),
      },
      output: {
        manualChunks: {
          three: ["three", "three/examples/jsm/controls/OrbitControls.js"],
        },
      },
    },
  },
});
